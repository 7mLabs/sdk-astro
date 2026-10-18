"""Assemble final PyPI/NuGet artifacts from validated same-commit CI candidates."""
import argparse
import hashlib
import importlib.util
import json
import shutil
import subprocess
import sys
import tomllib
import xml.etree.ElementTree as ET
import zipfile
from pathlib import Path, PurePosixPath

ROOT=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('npm_release',ROOT/'scripts/prepare-npm-release.py')
base=importlib.util.module_from_spec(spec);spec.loader.exec_module(base)
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--darwin',type=Path,required=True);parser.add_argument('--linux',type=Path,required=True)
parser.add_argument('--commit',required=True)
args=parser.parse_args()
version=tomllib.loads((ROOT/'neutral-engine/Cargo.toml').read_text())['workspace']['package']['version']
OUT=ROOT/'artifacts/managed-release'; OUT.mkdir(parents=True,exist_ok=True)
candidates=[]
for directory,platform,arch in [(args.darwin,'darwin','arm64'),(args.linux,'linux','x64')]:
    _,_,info,_=base.load_candidate(directory,platform,arch,args.commit,version)
    manifests=list(directory.rglob('manifest.json'))
    manifests=[p for p in manifests if json.loads(p.read_text()).get('rid')==('osx-arm64' if platform=='darwin' else 'linux-x64')]
    base.require(len(manifests)==1,'Ambiguous candidate manifest')
    folder=manifests[0].parent
    time_report=base.find_file(directory,'time-svg-tests.json',folder)
    time_data=json.loads(time_report.read_text())
    fingerprint=hashlib.sha256(json.dumps(json.loads(manifests[0].read_text())['files'],sort_keys=True).encode()).hexdigest()[:12]
    base.require(time_data.get('passed') is True and time_data.get('engineVersion')==version and time_data.get('artifactFingerprint')==fingerprint,'Time/SVG candidate proof is missing or stale')
    info['timeSvgEvidenceSha256']=hashlib.sha256(time_report.read_bytes()).hexdigest()
    candidates.append((folder,info))

python_wheel=candidates[0][0]/f'sevenmlabs_astrology-{version}-py3-none-macosx_11_0_arm64.whl'
subprocess.run([sys.executable,str(ROOT/'scripts/prepare-python-release.py'),'--baseline-wheel',str(python_wheel),'--output',str(OUT/'python')],check=True)
python_manifest=json.loads((OUT/'python/manifest.json').read_text())
wheel=OUT/python_wheel.name;shutil.copyfile(OUT/'python'/wheel.name,wheel)

def entries(path):
    with zipfile.ZipFile(path) as archive:
        result={}
        for item in archive.infolist():
            p=PurePosixPath(item.filename)
            base.require(not p.is_absolute() and '..' not in p.parts and item.filename not in result,'Unsafe package path')
            base.require(item.filename!='.signature.p7s','Candidates must be unsigned before assembly')
            result[item.filename]=archive.read(item)
        return result

name=f'SevenMLabs.Astrology.{version}.nupkg'
mac=entries(candidates[0][0]/name);linux=entries(candidates[1][0]/name)
mac_path='runtimes/osx-arm64/native/libastro_engine.dylib';linux_path='runtimes/linux-x64/native/libastro_engine.so'
base.require([p for p in mac if p.startswith('runtimes/')]==[mac_path],'Invalid macOS RID assets')
base.require([p for p in linux if p.startswith('runtimes/')]==[linux_path],'Invalid Linux RID assets')
for entry in ['README.md','LICENSE','NOTICE']+[p for p in mac if p.startswith('third-party/')]:
    base.require(mac[entry]==linux.get(entry),f'Portable NuGet content differs between targets: {entry}')
# Universal managed IL can have OS-specific deterministic debug identities.
# Require every other PE byte to agree; retain the original Mac assembly and
# exercise that exact assembly on both operating systems in the final gate.
dll_name='lib/net10.0/SevenMLabs.Astrology.dll'
proof_dir=OUT/'assembly-proof';proof_dir.mkdir(exist_ok=True)
dll_paths=[]
for target,payload in [('darwin',mac),('linux',linux)]:
    path=proof_dir/f'{target}.dll';path.write_bytes(payload[dll_name]);dll_paths.append(path)
inspection=subprocess.run(['dotnet','run','--project',str(ROOT/'tests/packaging/managed-identity/Inspect.csproj'),
                          '--configuration','Release','--',*map(str,dll_paths)],check=True,text=True,capture_output=True)
proof=[json.loads(line) for line in inspection.stdout.splitlines() if line.startswith('{')]
base.require(len(proof)==2,'Missing managed assembly identity proof')
for item in proof:
    base.require(item['identity']=='SevenMLabs.Astrology' and item['version']==version+'.0','Managed assembly identity mismatch')
    base.require(item['debug']==[dict(Path='/_/bindings/dotnet/obj/Release/net10.0/SevenMLabs.Astrology.pdb',Guid=item['debug'][0]['Guid'],Age=1)],'Unexpected debug identity')
for key in ('normalizedSha','api','il'):
    base.require(proof[0][key]==proof[1][key],f'Managed runtime content differs: {key}')
assembly_proof={'sha256':hashlib.sha256(mac[dll_name]).hexdigest(),
                'normalizedSha256':proof[0]['normalizedSha'].lower(),
                'excludedFields':['MVID','COFF timestamp','PE checksum','debug timestamps','CodeView','PDB checksum'],
                'candidates':[{key:value for key,value in item.items() if key!='file'} for item in proof]}
for payload in (mac,linux):
    nuspec=ET.fromstring(payload['SevenMLabs.Astrology.nuspec'])
    fields={n.tag.rsplit('}',1)[-1]:n.text for n in nuspec.iter()}
    base.require(fields.get('id')=='SevenMLabs.Astrology' and fields.get('version')==version,'NuGet identity mismatch')
mac[linux_path]=linux[linux_path]
content=ET.fromstring(mac['[Content_Types].xml'])
if not any(n.get('Extension')=='so' for n in content):
    ET.SubElement(content,'{http://schemas.openxmlformats.org/package/2006/content-types}Default',Extension='so',ContentType='application/octet')
mac['[Content_Types].xml']=ET.tostring(content,encoding='utf-8',xml_declaration=True)
nuget=OUT/name
with zipfile.ZipFile(nuget,'w',compression=zipfile.ZIP_DEFLATED,compresslevel=9) as archive:
    for member,data in sorted(mac.items()):
        info=zipfile.ZipInfo(member,date_time=(2020,1,1,0,0,0));info.compress_type=zipfile.ZIP_DEFLATED;archive.writestr(info,data)
npm=ROOT/'artifacts/npm-release'/f'7mlabs-astrology-{version}.tgz'
shutil.copyfile(npm,OUT/npm.name)
files={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in (wheel,nuget,OUT/npm.name)}
manifest={'schemaVersion':'1.0','version':version,'commitSha':args.commit,'pythonTargets':['darwin-arm64'],
          'nugetTargets':['osx-arm64','linux-x64'],'files':files,'pythonSourceSha256':python_manifest['files'][f'astro-sdk-{version}.tar.gz'],
          'nativeSha256':{'osx-arm64':hashlib.sha256(mac[mac_path]).hexdigest(),'linux-x64':hashlib.sha256(mac[linux_path]).hexdigest()},
          'managedAssembly':assembly_proof,'candidates':[c[1] for c in candidates]}
(OUT/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(json.dumps(manifest,indent=2))
