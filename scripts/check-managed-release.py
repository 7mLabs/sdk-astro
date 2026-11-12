"""Validate immutable artifacts and require final Mac/Python and Linux/.NET proof."""
import argparse
from email.parser import BytesParser
import hashlib
import io
import json
from pathlib import Path
import tomllib
import tarfile
import zipfile

ROOT=Path(__file__).resolve().parents[1]
LICENSE_COMPANIONS=('LICENSE-7MLABS-MIT','LICENSE-SCOPE.json','LICENSE-SCOPE.md')
p=argparse.ArgumentParser(description=__doc__)
p.add_argument('--manifest',type=Path,required=True);p.add_argument('--commit',required=True);p.add_argument('--reports',type=Path)
a=p.parse_args();m=json.loads(a.manifest.read_text());v=tomllib.loads((ROOT/'neutral-engine/Cargo.toml').read_text())['workspace']['package']['version']
assert m['version']==v and m['commitSha']==a.commit
assert m['pythonTargets']==['darwin-arm64','linux-x64'] and m['nugetTargets']==['osx-arm64','linux-x64']
expected={f'7mlabs-astrology-{v}.tgz',f'SevenMLabs.Astrology.{v}.nupkg',f'sevenmlabs_astrology-{v}-py3-none-macosx_11_0_arm64.whl',f'sevenmlabs_astrology-{v}-py3-none-manylinux_2_38_x86_64.whl'}
assert set(m['files'])==expected
for name,digest in m['files'].items():
 assert Path(name).name==name and hashlib.sha256((a.manifest.parent/name).read_bytes()).hexdigest()==digest
with zipfile.ZipFile(a.manifest.parent/f'SevenMLabs.Astrology.{v}.nupkg') as archive:
 for name in LICENSE_COMPANIONS:
  assert archive.read(name)==(ROOT/name).read_bytes(),f'NuGet scoped license companion differs: {name}'
 assert hashlib.sha256(archive.read('lib/net10.0/SevenMLabs.Astrology.dll')).hexdigest()==m['managedAssembly']['sha256']
 proof=m['managedAssembly']['candidates'];assert len(proof)==2
 assert all(x['normalizedSha'].lower()==m['managedAssembly']['normalizedSha256'] and x['identity']=='SevenMLabs.Astrology' and x['version']==v+'.0' for x in proof)
 assert proof[0]['il']==proof[1]['il'] and proof[0]['api']==proof[1]['api']
 for rid,filename in [('osx-arm64','libastro_engine.dylib'),('linux-x64','libastro_engine.so')]:
  assert hashlib.sha256(archive.read(f'runtimes/{rid}/native/{filename}')).hexdigest()==m['nativeSha256'][rid]
for target,tag,rid,native in [('darwin-arm64','macosx_11_0_arm64','osx-arm64','libastro_engine.dylib'),('linux-x64','manylinux_2_38_x86_64','linux-x64','libastro_engine.so')]:
 with zipfile.ZipFile(a.manifest.parent/f'sevenmlabs_astrology-{v}-py3-none-{tag}.whl') as archive:
  source_bytes=archive.read(f'sevenmlabs_astrology/source/astro-sdk-{v}.tar.gz')
  assert hashlib.sha256(source_bytes).hexdigest()==m['pythonSourceSha256']
  dist_info=f'sevenmlabs_astrology-{v}.dist-info'
  metadata=BytesParser().parsebytes(archive.read(f'{dist_info}/METADATA'))
  assert metadata.get('License-Expression')=='AGPL-3.0-only'
  assert {'LICENSE','NOTICE',*LICENSE_COMPANIONS}.issubset(metadata.get_all('License-File',[]))
  for name in LICENSE_COMPANIONS:
   matches=[path for path in (f'{dist_info}/licenses/{name}',f'{dist_info}/{name}') if path in archive.namelist()]
   assert len(matches)==1 and archive.read(matches[0])==(ROOT/name).read_bytes(),f'Wheel scoped license companion differs: {name}'
  with tarfile.open(fileobj=io.BytesIO(source_bytes),mode='r:gz') as source:
   for name in LICENSE_COMPANIONS:
    assert source.extractfile(f'astro-sdk-{v}/{name}').read()==(ROOT/name).read_bytes(),f'Matching-source scoped license companion differs: {name}'
  assert hashlib.sha256(archive.read(f'sevenmlabs_astrology/native/{target}/{native}')).hexdigest()==m['nativeSha256'][rid]
  assert f'Tag: py3-none-{tag}' in archive.read(f'sevenmlabs_astrology-{v}.dist-info/WHEEL').decode()
seen=set()
if a.reports:
 for path in a.reports.rglob('*.json'):
  report=json.loads(path.read_text());assert report['result']=='passed' and report['version']==v and report['commitSha']==a.commit
  assert report['manifestSha256']==hashlib.sha256(a.manifest.read_bytes()).hexdigest()
  assert report['managedAssemblySha256']==m['managedAssembly']['sha256']
  assert report['nativeSha256']==m['nativeSha256'][report['rid']]
  assert report['checks']=={'conformance':339,'time':9,'extended':2,'svg':12,'model':2,'davison':4}
  key=(report['rid'],'.'.join(report['pythonVersion'].split('.')[:2]))
  assert key not in seen;seen.add(key)
  assert report['pythonInstalled'] is True
  assert set(report['bindings'])==({'node','dotnet','python'} if report['pythonInstalled'] else {'node','dotnet'})
 assert {('osx-arm64','3.10'),('osx-arm64','3.13'),('linux-x64','3.10'),('linux-x64','3.13')}<=seen
print(json.dumps({'result':'passed','version':v,'commitSha':a.commit,'finalConsumerReports':sorted(seen)},indent=2))
