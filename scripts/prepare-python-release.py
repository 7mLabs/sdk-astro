"""Stage a tested platform-specific Python wheel with matching source; never publish."""
import argparse
import base64
import csv
from email.parser import BytesParser
import gzip
import hashlib
import io
import json
import tarfile
import tomllib
import zipfile
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[1]
LICENSE_COMPANIONS = ('LICENSE-7MLABS-MIT','LICENSE-SCOPE.json','LICENSE-SCOPE.md')
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--baseline-wheel', type=Path)
parser.add_argument('--platform', choices=['darwin-arm64','linux-x64'], default='darwin-arm64')
parser.add_argument('--output', type=Path, default=ROOT / 'artifacts/pypi-release')
args = parser.parse_args()
OUT = args.output.resolve()
OUT.mkdir(parents=True, exist_ok=True)
version = tomllib.loads((ROOT / 'bindings/python/pyproject.toml').read_text())['project']['version']
core_version = tomllib.loads((ROOT / 'neutral-engine/Cargo.toml').read_text())['workspace']['package']['version']
target = {'darwin-arm64':('macosx_11_0_arm64','libastro_engine.dylib'), 'linux-x64':('manylinux_2_38_x86_64','libastro_engine.so')}[args.platform]
filename = f'sevenmlabs_astrology-{version}-py3-none-{target[0]}.whl'
baseline = args.baseline_wheel or ROOT / 'artifacts/packages' / filename
if baseline.name != filename:
    raise SystemExit('Unexpected Python wheel identity or platform')
native_name = f'sevenmlabs_astrology/native/{args.platform}/{target[1]}'
with zipfile.ZipFile(baseline) as previous:
    entries = {}
    for item in previous.infolist():
        path = PurePosixPath(item.filename)
        if path.is_absolute() or '..' in path.parts or item.filename in entries or item.file_size > 128*1024*1024:
            raise SystemExit('Unsafe or duplicate wheel member')
        entries[item.filename] = previous.read(item)
    if [n for n in entries if n.endswith(('.dylib','.so','.dll'))] != [native_name]:
        raise SystemExit('Unexpected Python native targets')
    wheel_metadata=entries[f'sevenmlabs_astrology-{version}.dist-info/WHEEL'].decode()
    if 'Root-Is-Purelib: false' not in wheel_metadata or f'Tag: py3-none-{target[0]}' not in wheel_metadata:
        raise SystemExit('Wheel platform metadata does not match its native target')
    native=entries[native_name]
    if args.platform=='linux-x64' and not (native[:4]==b'\x7fELF' and native[4:6]==b'\x02\x01' and int.from_bytes(native[18:20],'little')==62):
        raise SystemExit('Expected a Linux x86-64 ELF shared library')
    if args.platform=='darwin-arm64' and not (native[:4]==b'\xcf\xfa\xed\xfe' and int.from_bytes(native[4:8],'little')==0x0100000C):
        raise SystemExit('Expected a macOS ARM64 native library')
    dist_info=f'sevenmlabs_astrology-{version}.dist-info'
    metadata=BytesParser().parsebytes(entries[f'{dist_info}/METADATA'])
    if metadata.get('License-Expression')!='AGPL-3.0-only':
        raise SystemExit('Candidate Python aggregate license differs')
    if not {'LICENSE','NOTICE',*LICENSE_COMPANIONS}.issubset(metadata.get_all('License-File',[])):
        raise SystemExit('Candidate Python license companion declarations are missing')
    for name in LICENSE_COMPANIONS:
        matches=[path for path in (f'{dist_info}/licenses/{name}',f'{dist_info}/{name}') if path in entries]
        if len(matches)!=1 or entries[matches[0]]!=(ROOT/name).read_bytes():
            raise SystemExit(f'Candidate Python scoped license companion missing or different: {name}')
    for relative in ['sevenmlabs_astrology/__init__.py', 'README.md']:
        archived = relative if relative != 'README.md' else f'sevenmlabs_astrology-{version}.dist-info/METADATA'
        source = (ROOT / 'bindings/python' / relative).read_bytes()
        if relative == 'README.md':
            if source not in entries[archived]:
                raise SystemExit('Candidate Python README differs from source')
        elif source != entries[archived]:
            raise SystemExit('Candidate Python wrapper differs from source')

# Source roots are explicit; build outputs, credentials and native blobs are excluded.
excluded = {'target','build','dist','native','source','__pycache__','node_modules','.git','.venv'}
files = set()
for folder in ('neutral-engine','bindings/python','schemas','third-party/rust','licensing'):
    for path in (ROOT / folder).rglob('*'):
        relative = path.relative_to(ROOT)
        if path.is_symlink():
            raise SystemExit(f'Symlink in source snapshot: {relative}')
        if not path.is_file() or excluded.intersection(relative.parts) or any(p.endswith('.egg-info') for p in relative.parts):
            continue
        if path.name.startswith('.') or path.suffix in {'.pyc','.log','.key','.pem'}:
            continue
        files.add(path)
for name in ('LICENSE','NOTICE',*LICENSE_COMPANIONS,'global.json','README.md','CHANGELOG.md','docs/license.md','docs/license.en.md','docs/time-and-svg.md',
             'docs/payload-metadata.md','docs/release-1.1.2.md','docs/release-1.1.3.md','docs/python-linux.md','docs/domains.md','docs/query.md','docs/profiles.md','docs/couple.md','docs/composite.md',
             'docs/forecast.md','docs/payload-compression.md','scripts/prepare-python-release.py'):
    files.add(ROOT / name)
files.update((ROOT / 'examples').glob('*.json'))
prefix = f'astro-sdk-{core_version}'
members = {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(files)}
notes = f'''# Matching source for sevenmlabs-astrology {version}

SOURCE-MANIFEST.json records original file checksums. Swiss source/data, renderer
assets, locked dependency declarations and notices are included. Cargo downloads
its pinned registry dependencies on the first build.

This is the Python/core rebuild subset of the repository. LICENSE-SCOPE.json
uses full-repository paths and also identifies files from other bindings/tests
that are not needed to rebuild this wheel. Consult the full repository for those
files or to run its complete license-scope validation. Both license guides and
the scoped permission/provenance files are included here.

Rebuild using Rust stable and Python 3.10+. macOS ARM64 needs Xcode command
line tools; Linux x64 needs a C compiler and a glibc-compatible build host.
The supported Linux release is built on Ubuntu 24.04 and requires glibc 2.38+.
On macOS ARM64:

    cargo build --manifest-path neutral-engine/Cargo.toml -p astro-ffi --lib --release --locked
    mkdir -p bindings/python/sevenmlabs_astrology/native/darwin-arm64
    cp neutral-engine/target/release/libastro_engine.dylib bindings/python/sevenmlabs_astrology/native/darwin-arm64/
    python3 -m pip install 'setuptools>=77' wheel
    python3 -m pip wheel --no-deps --no-build-isolation --wheel-dir dist bindings/python

Copy this archive into bindings/python/sevenmlabs_astrology/source/ before the
wheel step to retain corresponding source in the rebuilt wheel.
Run engine tests with cargo test --manifest-path neutral-engine/Cargo.toml --locked.
For Linux x64, build the same astro-ffi library and copy libastro_engine.so into
bindings/python/sevenmlabs_astrology/native/linux-x64/. Build the wheel using the
same pip command, then run auditwheel repair --plat manylinux_2_38_x86_64 --only-plat.
Use a clean checkout for each platform; never mix native targets in one wheel.
See docs/python-linux.md for the complete Linux build and validation procedure.
The public wheels cover macOS ARM64 and Linux x64/glibc 2.38+ only.
Other targets require separate validation.
'''
source = OUT / f'{prefix}.tar.gz'
with source.open('wb') as raw, gzip.GzipFile(filename='',mode='wb',fileobj=raw,mtime=0) as gz, tarfile.open(fileobj=gz,mode='w') as archive:
    def add(name, data):
        info=tarfile.TarInfo(f'{prefix}/{name}'); info.size=len(data); info.mode=0o644
        archive.addfile(info,io.BytesIO(data))
    for path in sorted(files):
        add(str(path.relative_to(ROOT)),path.read_bytes())
    add('SOURCE-MANIFEST.json',(json.dumps(members,indent=2,sort_keys=True)+'\n').encode())
    add('BUILD-SOURCE.md',notes.encode())
source_name=f'sevenmlabs_astrology/source/{source.name}'
entries[source_name]=source.read_bytes()
record_name=f'sevenmlabs_astrology-{version}.dist-info/RECORD'
entries.pop(record_name)
rows=[]
for name,data in sorted(entries.items()):
    digest=base64.urlsafe_b64encode(hashlib.sha256(data).digest()).rstrip(b'=').decode()
    rows.append([name,'sha256='+digest,str(len(data))])
rows.append([record_name,'',''])
stream=io.StringIO(newline=''); csv.writer(stream,lineterminator='\n').writerows(rows)
entries[record_name]=stream.getvalue().encode()
wheel=OUT/filename
with zipfile.ZipFile(wheel,'w',compression=zipfile.ZIP_DEFLATED,compresslevel=9) as result:
    for name,data in sorted(entries.items()):
        info=zipfile.ZipInfo(name,date_time=(2020,1,1,0,0,0));info.compress_type=zipfile.ZIP_DEFLATED
        result.writestr(info,data)
manifest={'version':version,'engineVersion':core_version,'platform':args.platform,
          'baselineWheelSha256':hashlib.sha256(baseline.read_bytes()).hexdigest(),
          'nativeSha256':hashlib.sha256(entries[native_name]).hexdigest(),'sourceFileCount':len(files),
          'files':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in (wheel,source)}}
(OUT/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(json.dumps(manifest,indent=2))
