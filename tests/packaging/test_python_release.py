"""Release staging must reject mislabeled native wheels and retain rebuildable source."""
import base64
import csv
from email.parser import BytesParser
import hashlib
import io
import json
from pathlib import Path
import subprocess
import sys
import tarfile
import tempfile
import tomllib
import unittest
import zipfile

ROOT = Path(__file__).resolve().parents[2]
VERSION = tomllib.loads((ROOT/'bindings/python/pyproject.toml').read_text())['project']['version']
LICENSE_COMPANIONS = ('LICENSE-7MLABS-MIT', 'LICENSE-SCOPE.json', 'LICENSE-SCOPE.md')


class PythonReleaseTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.folder = Path(self.temp.name)

    def candidate(self, target, native=None, tag=None, extra=False, *,
                  declared_companions=None, companion_files=None,
                  license_expression='AGPL-3.0-only'):
        platform_tag, filename = {
            'darwin-arm64': ('macosx_11_0_arm64', 'libastro_engine.dylib'),
            'linux-x64': ('manylinux_2_38_x86_64', 'libastro_engine.so'),
        }[target]
        if native is None:
            if target == 'linux-x64':
                native = bytearray(64)
                native[:6] = b'\x7fELF\x02\x01'
                native[18:20] = (62).to_bytes(2, 'little')
                native = bytes(native)
            else:
                native = b'\xcf\xfa\xed\xfe' + (0x0100000C).to_bytes(4, 'little') + bytes(56)
        path = self.folder/f'sevenmlabs_astrology-{VERSION}-py3-none-{platform_tag}.whl'
        info = f'sevenmlabs_astrology-{VERSION}.dist-info/'
        if declared_companions is None:
            declared_companions = LICENSE_COMPANIONS
        license_files = {name: (ROOT/name).read_bytes()
                         for name in ('LICENSE', 'NOTICE', *LICENSE_COMPANIONS)}
        license_files.update(companion_files or {})
        metadata = ['Metadata-Version: 2.4', 'Name: sevenmlabs-astrology', f'Version: {VERSION}',
                    f'License-Expression: {license_expression}',
                    *[f'License-File: {name}' for name in ('LICENSE', 'NOTICE', *declared_companions)]]
        with zipfile.ZipFile(path, 'w') as wheel:
            wheel.writestr(f'sevenmlabs_astrology/native/{target}/{filename}', native)
            wheel.writestr('sevenmlabs_astrology/__init__.py', (ROOT/'bindings/python/sevenmlabs_astrology/__init__.py').read_bytes())
            wheel.writestr(info+'METADATA', ('\n'.join(metadata)+'\n\n').encode()
                           +(ROOT/'bindings/python/README.md').read_bytes())
            for name, payload in license_files.items():
                if payload is not None:
                    wheel.writestr(info+'licenses/'+name, payload)
            wheel.writestr(info+'WHEEL', 'Wheel-Version: 1.0\nRoot-Is-Purelib: false\nTag: py3-none-'+(tag or platform_tag)+'\n')
            wheel.writestr(info+'RECORD', '')
            if extra:
                wheel.writestr('sevenmlabs_astrology/native/win32-x64/astro_engine.dll', b'other-platform')
        return path, native

    def stage(self, target, path):
        output = self.folder/target
        result = subprocess.run([sys.executable, ROOT/'scripts/prepare-python-release.py',
                                 '--platform', target, '--baseline-wheel', path, '--output', output],
                                capture_output=True, text=True)
        return result, output

    def test_both_targets_keep_native_and_identical_source_with_valid_record(self):
        sources = []
        for target in ['darwin-arm64', 'linux-x64']:
            path, native = self.candidate(target)
            result, output = self.stage(target, path)
            self.assertEqual(result.returncode, 0, result.stderr)
            with zipfile.ZipFile(output/path.name) as wheel:
                self.assertIn(native, [wheel.read(n) for n in wheel.namelist() if '/native/' in n])
                info = f'sevenmlabs_astrology-{VERSION}.dist-info/'
                metadata = BytesParser().parsebytes(wheel.read(info+'METADATA'))
                self.assertEqual(metadata['License-Expression'], 'AGPL-3.0-only')
                self.assertTrue(set(LICENSE_COMPANIONS).issubset(metadata.get_all('License-File', [])))
                for name in LICENSE_COMPANIONS:
                    self.assertEqual(wheel.read(info+'licenses/'+name), (ROOT/name).read_bytes())
                record = f'sevenmlabs_astrology-{VERSION}.dist-info/RECORD'
                rows = list(csv.reader(io.StringIO(wheel.read(record).decode())))
                self.assertEqual({row[0] for row in rows}, set(wheel.namelist()))
                for name, digest, size in rows:
                    if name == record:
                        self.assertEqual((digest, size), ('', ''))
                        continue
                    payload = wheel.read(name)
                    expected = base64.urlsafe_b64encode(hashlib.sha256(payload).digest()).rstrip(b'=').decode()
                    self.assertEqual((digest, size), ('sha256='+expected, str(len(payload))))
                source = wheel.read(f'sevenmlabs_astrology/source/astro-sdk-{VERSION}.tar.gz')
                sources.append(source)
            with tarfile.open(fileobj=io.BytesIO(source), mode='r:gz') as archive:
                prefix = f'astro-sdk-{VERSION}/'
                manifest = json.load(archive.extractfile(prefix+'SOURCE-MANIFEST.json'))
                self.assertIn('bindings/python/setup.py', manifest)
                self.assertIn('docs/python-linux.md', manifest)
                for name in (*LICENSE_COMPANIONS, 'licensing/swisseph-sweph.c.patch',
                             'licensing/swisseph-modifications.json'):
                    self.assertIn(name, manifest)
                    self.assertEqual(archive.extractfile(prefix+name).read(), (ROOT/name).read_bytes())
                self.assertFalse(any('/native/' in name or '/target/' in name for name in manifest))
                for name, digest in manifest.items():
                    self.assertEqual(hashlib.sha256(archive.extractfile(prefix+name).read()).hexdigest(), digest)
        self.assertEqual(sources[0], sources[1])

    def test_scoped_companions_must_be_declared(self):
        for target in ['darwin-arm64', 'linux-x64']:
            with self.subTest(target=target):
                path, _ = self.candidate(target, declared_companions=LICENSE_COMPANIONS[:-1])
                result, _ = self.stage(target, path)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn('license companion declarations are missing', result.stderr)

    def test_missing_or_stale_scoped_companion_is_rejected(self):
        for target in ['darwin-arm64', 'linux-x64']:
            for name in LICENSE_COMPANIONS:
                for payload in [None, b'stale license companion']:
                    with self.subTest(target=target, name=name, missing=payload is None):
                        path, _ = self.candidate(target, companion_files={name: payload})
                        result, _ = self.stage(target, path)
                        self.assertNotEqual(result.returncode, 0)
                        self.assertIn('scoped license companion missing or different: '+name, result.stderr)

    def test_aggregate_license_remains_agpl(self):
        path, _ = self.candidate('linux-x64', license_expression='MIT')
        result, _ = self.stage('linux-x64', path)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('aggregate license differs', result.stderr)

    def test_linux_tag_cannot_wrap_macos_binary(self):
        path, _ = self.candidate('linux-x64', native=b'\xcf\xfa\xed\xfe'+bytes(60))
        result, _ = self.stage('linux-x64', path)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Linux x86-64 ELF', result.stderr)

    def test_wheel_metadata_must_match_native_target(self):
        path, _ = self.candidate('linux-x64', tag='macosx_11_0_arm64')
        result, _ = self.stage('linux-x64', path)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('platform metadata', result.stderr)

    def test_platform_wheel_cannot_accidentally_bundle_another_target(self):
        path, _ = self.candidate('linux-x64', extra=True)
        result, _ = self.stage('linux-x64', path)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('native targets', result.stderr)


if __name__ == '__main__':
    unittest.main()
