"""Review protocol v1.1; no product source edits, raw JSON through native CLI."""
import json, subprocess, pathlib, hashlib, platform, re, gzip, statistics
from decimal import Decimal
from datetime import datetime, timezone
ROOT=pathlib.Path(__file__).resolve().parents[5]
OUT=pathlib.Path(__file__).resolve().parent
BIN=ROOT/'examples/rust/target/debug/astrology-rust-example'
def sha(b): return hashlib.sha256(b).hexdigest()
def pairs(items):
 d={}
 for k,v in items:
  if k in d: raise ValueError('duplicate key: '+k)
  d[k]=v
 return d
def reject(x): raise ValueError('nonfinite JSON constant: '+x)
def load(s):return json.loads(s,parse_int=Decimal,parse_float=Decimal,parse_constant=reject,object_pairs_hook=pairs)
def same(a,b):
 if type(a) is not type(b):return False
 if isinstance(a,dict):return a.keys()==b.keys() and all(same(a[k],b[k]) for k in a)
 if isinstance(a,list):return len(a)==len(b) and all(same(x,y) for x,y in zip(a,b))
 return a==b
controls=[('bool-number','true','1',False),('null-bool','null','false',False),('numeric-change','1.01','1.02',False),('array-order','[1,2]','[2,1]',False),('missing-key','{"a":1}','{}',False),('object-order','{"a":1,"b":2}','{"b":2,"a":1}',True),('number-spelling','1','1.0',True)]
for _,a,b,expected in controls:assert same(load(a),load(b))==expected
for invalid in ['{"x":1,"x":2}','NaN','Infinity']:
 try:load(invalid)
 except ValueError:pass
 else:raise AssertionError(invalid)
def strip_ws(s):
 out=[];inside=False;escape=False
 for c in s:
  if inside:
   out.append(c)
   if escape:escape=False
   elif c=='\\':escape=True
   elif c=='"':inside=False
  elif c=='"':inside=True;out.append(c)
  elif c not in ' \r\n\t':out.append(c)
 return ''.join(out)
def run(args,raw=None):
 p=subprocess.run([str(BIN)]+args,input=raw,text=True,encoding='utf-8',capture_output=True,check=True)
 s=p.stdout.removesuffix('\n');v=load(s)
 assert v['errors']==[],v['errors']
 return s,v
rows=[]
for f in sorted((ROOT/'examples').glob('*-result.json')):
 src=f.read_text();v=load(src)
 raw,c=run(['--compress-json'],'{"payload":'+src+',"options":{"mode":"compact"}}')
 expanded,e=run(['--expand-context-json'],raw)
 m=c['data']['metrics'];ib=int(m['inputBytes']);ob=int(m['outputBytes'])
 row={'fixture':f.name,'sha256':sha(f.read_bytes()),'sourceVersion':v['engineVersion'],'inputBytes':ib,'outputBytes':ob,'reductionPercent':100*(1-ob/ib),'roundtrip':same(v,e),'nativeOutputBytesMatch':len(raw.encode())==ob,'sourceLexicalMinifiedBytes':len(strip_ws(src).encode()),'expandedNativeBytes':len(expanded.encode()),'expandedBytesMatchInputMetric':len(expanded.encode())==ib,'coverageComplete':c['data']['coverage']['complete'],'omittedEmpty':c['data']['omitted']==[],'contextSha256':sha(raw.encode()),'expandedSha256':sha(expanded.encode())}
 assert row['roundtrip'] and row['nativeOutputBytesMatch'] and row['coverageComplete'] and row['omittedEmpty']
 for label,s in [('context',raw),('expanded',expanded)]:
  (OUT/(f.stem+'.'+label+'.json.gz')).write_bytes(gzip.compress(s.encode(),mtime=0))
 rows.append(row);print(f.name,ib,ob,flush=True)
refpath=ROOT/'tests/conformance/natal-references.json';refs=json.loads(refpath.read_text());deviations=[]
for i,c in enumerate(refs['cases']):
 raw,r=run([json.dumps(c['request'])]);(OUT/f'natal-{i}.json').write_text(raw+'\n')
 vals={p['id']:p['longitude'] for p in r['data']['placements']};item={'case':c['name'],'request':c['request']}
 assert len(vals)==10
 for key in ['swiss','skyfield']:
  positions=c[key]['positions'] if isinstance(c[key],dict) else c[key];ds=[]
  assert len(positions)==10
  for idx,p in enumerate(positions):
   body=p.get('id',c['swiss']['positions'][idx]['id']);baseline=Decimal(str(p['longitude']))
   diff=abs(vals[body]-baseline)%Decimal(360);diff=min(diff,Decimal(360)-diff)
   ds.append({'body':body,'actual':str(vals[body]),'reference':str(baseline),'longitudeErrorDegrees':float(diff)})
  item[key]={'n':len(ds),'maxLongitudeErrorDegrees':max(x['longitudeErrorDegrees'] for x in ds),'perBody':ds}
 deviations.append(item)
log=(OUT/'cargo-test.log').read_text();suites=[];name=''
for ln in log.splitlines():
 if 'Running ' in ln or 'Doc-tests ' in ln:name=ln.strip()
 m=re.search(r'test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored',ln)
 if m:suites.append({'suite':name,'status':m[1],'passed':int(m[2]),'failed':int(m[3]),'ignored':int(m[4])})
assert suites and all(x['status']=='ok' and not x['failed'] for x in suites)
ib=sum(x['inputBytes'] for x in rows);ob=sum(x['outputBytes'] for x in rows)
files=[BIN,ROOT/'neutral-engine/Cargo.lock',ROOT/'examples/rust/Cargo.lock',refpath,pathlib.Path(__file__),OUT/'cargo-test.log']
s={'runUtc':datetime.now(timezone.utc).isoformat(),'sourceCommit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'fixtureCount':len(rows),'roundtripsPassed':sum(x['roundtrip'] for x in rows),'totalInputBytes':ib,'totalOutputBytes':ob,'aggregateReductionPercent':100*(1-ob/ib),'medianPerFixtureReductionPercent':statistics.median(x['reductionPercent'] for x in rows),'smaller':sum(x['outputBytes']<x['inputBytes'] for x in rows),'larger':sum(x['outputBytes']>x['inputBytes'] for x in rows),'compression':rows,'references':deviations,'suites':suites,'passedTests':sum(x['passed'] for x in suites),'comparatorControls':[x[0] for x in controls]+['duplicate-key-rejected','NaN-rejected','Infinity-rejected'],'artifactHashes':{str(f.relative_to(ROOT)):sha(f.read_bytes()) for f in files}}
(OUT/'results.json').write_text(json.dumps(s,ensure_ascii=False,indent=2)+'\n')
print('SUMMARY',ib,ob,s['aggregateReductionPercent'],s['medianPerFixtureReductionPercent'],s['passedTests'])
