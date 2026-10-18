"""Validate immutable artifacts and require final Mac/Python and Linux/.NET proof."""
import argparse
import hashlib
import json
from pathlib import Path
import tomllib
import zipfile

ROOT=Path(__file__).resolve().parents[1]
p=argparse.ArgumentParser(description=__doc__)
p.add_argument('--manifest',type=Path,required=True);p.add_argument('--commit',required=True);p.add_argument('--reports',type=Path)
a=p.parse_args();m=json.loads(a.manifest.read_text());v=tomllib.loads((ROOT/'neutral-engine/Cargo.toml').read_text())['workspace']['package']['version']
assert m['version']==v and m['commitSha']==a.commit
assert m['pythonTargets']==['darwin-arm64'] and m['nugetTargets']==['osx-arm64','linux-x64']
expected={f'7mlabs-astrology-{v}.tgz',f'SevenMLabs.Astrology.{v}.nupkg',f'sevenmlabs_astrology-{v}-py3-none-macosx_11_0_arm64.whl'}
assert set(m['files'])==expected
for name,digest in m['files'].items():
 assert Path(name).name==name and hashlib.sha256((a.manifest.parent/name).read_bytes()).hexdigest()==digest
with zipfile.ZipFile(a.manifest.parent/f'SevenMLabs.Astrology.{v}.nupkg') as archive:
 for rid,filename in [('osx-arm64','libastro_engine.dylib'),('linux-x64','libastro_engine.so')]:
  assert hashlib.sha256(archive.read(f'runtimes/{rid}/native/{filename}')).hexdigest()==m['nativeSha256'][rid]
with zipfile.ZipFile(a.manifest.parent/f'sevenmlabs_astrology-{v}-py3-none-macosx_11_0_arm64.whl') as archive:
 assert hashlib.sha256(archive.read(f'sevenmlabs_astrology/source/astro-sdk-{v}.tar.gz')).hexdigest()==m['pythonSourceSha256']
seen=set()
if a.reports:
 for path in a.reports.rglob('*.json'):
  report=json.loads(path.read_text());assert report['result']=='passed' and report['version']==v and report['commitSha']==a.commit
  assert report['manifestSha256']==hashlib.sha256(a.manifest.read_bytes()).hexdigest()
  assert report['nativeSha256']==m['nativeSha256'][report['rid']]
  assert report['checks']=={'conformance':339,'time':9,'extended':2,'svg':12,'model':2}
  key=(report['rid'],'.'.join(report['pythonVersion'].split('.')[:2]))
  assert key not in seen;seen.add(key)
  assert report['pythonInstalled']==(report['rid']=='osx-arm64')
  assert set(report['bindings'])==({'node','dotnet','python'} if report['pythonInstalled'] else {'node','dotnet'})
 assert {('osx-arm64','3.10'),('osx-arm64','3.13'),('linux-x64','3.13')}<=seen
print(json.dumps({'result':'passed','version':v,'commitSha':a.commit,'finalConsumerReports':sorted(seen)},indent=2))
