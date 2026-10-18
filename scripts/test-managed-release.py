"""Fresh-install the final three archives; compare calculation/time/SVG outputs."""
import argparse
import hashlib
import json
import os
import platform
import subprocess
import sys
import zipfile
from pathlib import Path
from xml.etree import ElementTree as ET

ROOT=Path(__file__).resolve().parents[1]
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--manifest',type=Path,required=True);parser.add_argument('--report',type=Path,required=True)
args=parser.parse_args();manifest_path=args.manifest.resolve();manifest=json.loads(manifest_path.read_text());folder=manifest_path.parent
version=manifest['version'];manifest_hash=hashlib.sha256(manifest_path.read_bytes()).hexdigest()
for name,digest in manifest['files'].items():
    assert Path(name).name==name and hashlib.sha256((folder/name).read_bytes()).hexdigest()==digest,name
system={'Darwin':'darwin','Linux':'linux'}[platform.system()]
arch={'arm64':'arm64','aarch64':'arm64','x86_64':'x64'}[platform.machine()]
rid=('osx' if system=='darwin' else 'linux')+'-'+arch
work=ROOT/'artifacts/managed-consumers'/manifest_hash[:12]/f'{rid}-py{sys.version_info.major}{sys.version_info.minor}'
work.mkdir(parents=True,exist_ok=True)
node=os.environ.get('NODE_BIN','node');npm=[node,os.environ['NPM_CLI']] if os.environ.get('NPM_CLI') else ['npm']

def run(command,cwd=ROOT):
    return subprocess.run(list(map(str,command)),cwd=cwd,check=True,text=True,capture_output=True).stdout

node_dir=work/'node';node_dir.mkdir(exist_ok=True)
(node_dir/'package.json').write_text('{"private":true}')
run(npm+['install','--offline','--ignore-scripts','--no-audit','--no-fund','--cache',str(work/'npm-cache'),str(folder/f'7mlabs-astrology-{version}.tgz')],node_dir)
node_probe=node_dir/'probe.cjs'
node_probe.write_text('''const s=require('@7mlabs/astrology');const rl=require('node:readline').createInterface({input:process.stdin});
rl.on('line',line=>{const q=JSON.parse(line);const mode=q.mode;const raw=q.raw;
let result=mode==='calculate'?s.calculateJson(raw):mode==='resolve'?s.resolveBirthTimeJson(raw):mode==='local'?s.calculateFromLocalJson(raw):mode==='capabilities'?JSON.stringify({data:s.getCapabilities()}):s.renderChartSvgRequestJson(raw);
console.log(result);});''')
project=work/'dotnet';project.mkdir(exist_ok=True)
(project/'Probe.csproj').write_text(f'<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net10.0</TargetFramework><ImplicitUsings>enable</ImplicitUsings></PropertyGroup><ItemGroup><PackageReference Include="SevenMLabs.Astrology" Version="{version}" /></ItemGroup></Project>')
(project/'Program.cs').write_text('''using SevenMLabs.Astrology;using System.Text.Json;
while(Console.ReadLine() is string line){using var d=JsonDocument.Parse(line);var q=d.RootElement;var mode=q.GetProperty("mode").GetString();var raw=q.GetProperty("raw").GetString()!;
string result;
if(mode=="calculate") result=Engine.CalculateJson(raw);
else if(mode=="resolve") result=Engine.ResolveBirthTimeJson(raw);
else if(mode=="local") result=Engine.CalculateFromLocalJson(raw);
else if(mode=="capabilities") result=JsonSerializer.Serialize(new{data=Engine.GetCapabilities()});
else {using var r=JsonDocument.Parse(raw);result=Engine.RenderChartSvgJson(r.RootElement.GetProperty("payload").GetRawText(),r.RootElement.GetProperty("options").GetRawText());}
Console.WriteLine(result);}
''')
run(['dotnet','restore',project/'Probe.csproj','--source',folder,'--packages',work/'nuget-cache','-p:NuGetAudit=false'])
run(['dotnet','build',project/'Probe.csproj','--no-restore','-c','Release','--nologo'])
commands={'node':[node,node_probe],'dotnet':['dotnet',project/'bin/Release/net10.0/Probe.dll']}
python_enabled=system=='darwin' and arch=='arm64'
if python_enabled:
    venv=work/'venv';run([sys.executable,'-m','venv',venv]);python=venv/'bin/python'
    run([python,'-m','pip','install','--no-index','--no-deps',folder/f'sevenmlabs_astrology-{version}-py3-none-macosx_11_0_arm64.whl'])
    probe=work/'python-probe.py'
    probe.write_text('''import json,sys
import sevenmlabs_astrology as s
for line in sys.stdin:
 q=json.loads(line);mode=q['mode'];raw=q['raw']
 if mode=='calculate':result=s.calculate_json(raw)
 elif mode=='resolve':result=s.resolve_birth_time_json(raw)
 elif mode=='local':result=s.calculate_from_local_json(raw)
 elif mode=='capabilities':result=json.dumps({'data':s.get_capabilities()})
 else:
  r=json.loads(raw);result=s.render_chart_svg_json(json.dumps(r['payload']),json.dumps(r['options']))
 print(result,flush=True)
''')
    commands['python']=[python,probe]
    run([python,ROOT/'examples/python/test_sample.py'])
    readme_probe=work/'readme-check.py'
    readme_probe.write_text("from pathlib import Path\ntext=Path("+repr(str(ROOT/'bindings/python/README.md'))+").read_text()\nns={}\nfor block in text.split('```python\\n')[1:]: exec(block.split('```')[0],ns)\n")
    run([python,readme_probe],work)
# Native assets in the final package and the actual installed RID output must match.
native_file='libastro_engine.dylib' if system=='darwin' else 'libastro_engine.so'
installed_native=project/'bin/Release/net10.0/runtimes'/rid/'native'/native_file
assert hashlib.sha256(installed_native.read_bytes()).hexdigest()==manifest['nativeSha256'][rid]
processes={name:subprocess.Popen(list(map(str,cmd)),stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,bufsize=1) for name,cmd in commands.items()}

def invoke(mode,request):
    raw=request if isinstance(request,str) else json.dumps(request,separators=(',',':'))
    line=json.dumps({'mode':mode,'raw':raw},separators=(',',':'))+'\n'
    values={}
    for name,process in processes.items():
        process.stdin.write(line);process.stdin.flush();output=process.stdout.readline()
        if not output:
            raise AssertionError(f'{name} probe failed: '+process.stderr.read()[:2000])
        values[name]=json.loads(output)
    reference=values['node']
    if mode!='capabilities': assert reference['engineVersion']==version
    for name,value in values.items():
        assert value==reference,f'{mode} differs in {name}'
    return reference

counts={'conformance':0,'time':0,'extended':0,'svg':0,'model':0}
try:
    caps=invoke('capabilities',{})['data'];assert caps['abiVersion']==1 and caps['svg']['rendererVersion']=='1.2.0'
    for case in json.loads((ROOT/'tests/conformance/cases.json').read_text()):
        result=invoke('calculate',case.get('rawRequest',case.get('request')))
        assert bool(result['errors'])==bool(case.get('error',False)),case['name']
        counts['conformance']+=1
    time_cases=[
      {'local':{'year':1995,'month':6,'day':15,'hour':14,'minute':30},'timeZone':'Asia/Ho_Chi_Minh'},
      {'local':{'year':2024,'month':3,'day':10,'hour':2,'minute':30},'timeZone':'America/New_York'},
      {'local':{'year':2024,'month':11,'day':3,'hour':1,'minute':30},'timeZone':'America/New_York'},
      *[{'local':{'year':2024,'month':11,'day':3,'hour':1,'minute':30},'timeZone':'America/New_York','disambiguation':choice} for choice in ['earlier','later']],
      {'local':{'year':2000,'month':1,'day':1,'hour':0,'minute':0,'second':0.125},'utcOffsetSeconds':20700},
      {'local':{'year':1800,'month':1,'day':1,'hour':12,'minute':0},'timeZone':'Etc/UTC'},
      {'local':{'year':2000,'month':2,'day':30,'hour':12,'minute':0},'timeZone':'Etc/UTC'},
      {'local':{'year':2000,'month':1,'day':1,'hour':0,'minute':0},'timeZone':'Invalid/Zone'},
    ]
    for index,request in enumerate(time_cases):
        result=invoke('resolve',request);counts['time']+=1
        assert bool(result['errors'])==(index in [1,2,7,8])
    base={'operation':'natalDomains','local':{'year':1995,'month':6,'day':15,'hour':14,'minute':30},'timeZone':'Asia/Ho_Chi_Minh','location':{'latitude':10.8231,'longitude':106.6297},'domains':['identity'],'aspectPreset':'extended'}
    for points in ['standard','all']:
        payload=invoke('local',{**base,'additionalPoints':points});assert not payload['errors'];counts['extended']+=1
        assert len(payload['data']['natal']['additionalPoints'])==(3 if points=='standard' else 7)
    for theme in ['dark','light']:
        for part in ['wheel','positions','houses','aspectMatrix','aspects','houseFocus']:
            options={'sections':[part],'theme':theme,'focus':{'house':7,'mode':'highlight'},'aspectEndpoints':'all','aspectColors':{'harmonious':'#123456','challenging':'#F57891'}}
            rendered=invoke('render',{'payload':payload,'options':options})
            assert not rendered['errors'];ET.fromstring(rendered['data']['svg']);counts['svg']+=1
        model=invoke('render',{'payload':payload,'options':{'sections':['wheel','houseFocus'],'focus':{'house':7,'mode':'isolate'},'output':'model'}})
        assert not model['errors'] and 'svg' not in model['data'];assert model['data']['model']['focus']['house']['number']==7;counts['model']+=1
finally:
    for process in processes.values():
        process.stdin.close();process.wait(timeout=15)
        assert process.returncode==0,process.stderr.read()[:2000]
report={'result':'passed','version':version,'commitSha':manifest['commitSha'],'manifestSha256':manifest_hash,
        'platform':system,'arch':arch,'rid':rid,'pythonVersion':platform.python_version(),'pythonInstalled':python_enabled,
        'bindings':list(commands),'checks':counts,'nativeSha256':manifest['nativeSha256'][rid],
        'nodeVersion':run([node,'--version']).strip(),'dotnetVersion':run(['dotnet','--version']).strip()}
args.report.parent.mkdir(parents=True,exist_ok=True);args.report.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
