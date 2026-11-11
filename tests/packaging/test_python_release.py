"""Release staging must reject mislabeled native wheels and retain rebuildable source."""
import base64
import csv
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


class PythonReleaseTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.folder = Path(self.temp.name)

    def candidate(self, target, native=None, tag=None, extra=False):
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
        with zipfile.ZipFile(path, 'w') as wheel:
            wheel.writestr(f'sevenmlabs_astrology/native/{target}/{filename}', native)
            wheel.writestr('sevenmlabs_astrology/__init__.py', (ROOT/'bindings/python/sevenmlabs_astrology/__init__.py').read_bytes())
            wheel.writestr(info+'METADATA', b'Metadata-Version: 2.4\n\n'+(ROOT/'bindings/python/README.md').read_bytes())
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
                self.assertFalse(any('/native/' in name or '/target/' in name for name in manifest))
                for name, digest in manifest.items():
                    self.assertEqual(hashlib.sha256(archive.extractfile(prefix+name).read()).hexdigest(), digest)
        self.assertEqual(sources[0], sources[1])

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
