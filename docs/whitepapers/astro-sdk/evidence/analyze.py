import json, subprocess, pathlib, hashlib, platform, re
from decimal import Decimal
ROOT=pathlib.Path(__file__).resolve().parents[4]
OUT=pathlib.Path(__file__).resolve().parent
BIN=ROOT/'examples/rust/target/debug/astrology-rust-example'
def run(args, value=None):
 p=subprocess.run([str(BIN)]+args,input=None if value is None else json.dumps(value,ensure_ascii=False),text=True,capture_output=True,check=True)
 return p.stdout, json.loads(p.stdout)
def strict(s): return json.loads(s,parse_float=Decimal,parse_int=Decimal)
rows=[]
for f in sorted((ROOT/'examples').glob('*-result.json')):
 src=f.read_text(); payload=json.loads(src)
 raw,c=run(['--compress-json'],{'payload':payload,'options':{'mode':'compact'}})
 if c['errors']: raise RuntimeError(c['errors'])
 restored,r=run(['--expand-context-json'],c)
 m=c['data']['metrics']
 rows.append({'fixture':f.name,'sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'sourceVersion':payload['engineVersion'],'inputBytes':m['inputBytes'],'outputBytes':m['outputBytes'],'reductionPercent':100*(1-m['outputBytes']/m['inputBytes']),'roundtrip':strict(src)==strict(restored),'nativeOutputBytesMatch':len(raw.rstrip('\n').encode())==m['outputBytes']})
refs=json.loads((ROOT/'tests/conformance/natal-references.json').read_text()); deviations=[]
for c in refs['cases']:
 raw,r=run([json.dumps(c['request'])])
 (OUT/('natal-'+str(len(deviations))+'.json')).write_text(raw)
 vals={p['id']:p['longitude'] for p in r['data']['placements']}
 item={'case':c['name']}
 for key in ['swiss','skyfield']:
  positions=c[key]['positions'] if isinstance(c[key],dict) else c[key]
  ds=[]
  for idx,p in enumerate(positions):
   body=p.get('id',c['swiss']['positions'][idx]['id'])
   d=abs(vals[body]-p['longitude'])%360
   ds.append(min(d,360-d))
  item[key]={'n':len(ds),'maxLongitudeErrorDegrees':max(ds)}
 deviations.append(item)
log=(OUT/'cargo-test.log').read_text()
summary={'sourceCommit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'platform':platform.platform(),'machine':platform.machine(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'cargo':subprocess.check_output(['cargo','--version'],text=True).strip(),'fixtureCount':len(rows),'roundtripsPassed':sum(r['roundtrip'] for r in rows),'totalInputBytes':sum(r['inputBytes'] for r in rows),'totalOutputBytes':sum(r['outputBytes'] for r in rows),'compression':rows,'references':deviations,'tests':[list(map(int,m)) for m in re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored',log)]}
(OUT/'results.json').write_text(json.dumps(summary,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(summary,ensure_ascii=False,indent=2))
