"""Reanalyse stored evidence; no SDK execution or reference regeneration."""
from pathlib import Path
from decimal import Decimal as D
import json,gzip,hashlib,re,statistics
P=Path(__file__).resolve().parent;ROOT=P.parents[3];E=P/'review-v1.1'
r=json.loads((E/'results.json').read_text())
def reject(x):raise ValueError(x)
def pairs(items):
 out={}
 for k,v in items:
  assert k not in out
  out[k]=v
 return out
def load(raw):return json.loads(raw,parse_int=D,parse_float=D,parse_constant=reject,object_pairs_hook=pairs)
def same(a,b):
 if type(a)!=type(b):return False
 if isinstance(a,dict):return a.keys()==b.keys() and all(same(a[k],b[k]) for k in a)
 if isinstance(a,list):return len(a)==len(b) and all(same(x,y) for x,y in zip(a,b))
 return a==b
def sha(b):return hashlib.sha256(b).hexdigest()
rows=[]
for x in r['compression']:
 src=(ROOT/'examples'/x['fixture']).read_bytes();stem=Path(x['fixture']).stem
 c=gzip.decompress((E/(stem+'.context.json.gz')).read_bytes());e=gzip.decompress((E/(stem+'.expanded.json.gz')).read_bytes())
 assert sha(src)==x['sha256'] and sha(c)==x['contextSha256'] and sha(e)==x['expandedSha256']
 cv=load(c);assert same(load(src),load(e))
 assert len(c)==x['outputBytes']==cv['data']['metrics']['outputBytes']
 assert len(e)==x['inputBytes']==cv['data']['metrics']['inputBytes']
 assert cv['data']['coverage']['complete'] is True and cv['data']['omitted']==[]
 rate=100*(D(len(e))-len(c))/len(e)
 assert abs(rate-D(str(x['reductionPercent'])))<D('1e-12')
 rows.append({'fixture':x['fixture'],'B':len(e),'C':len(c),'R':str(rate)})
B=sum(x['B'] for x in rows);C=sum(x['C'] for x in rows);rates=[D(x['R']) for x in rows]
assert B==r['totalInputBytes'] and C==r['totalOutputBytes']
assert len(rows)==17 and sum(x['C']<x['B'] for x in rows)==15
assert abs(statistics.median(rates)-D(str(r['medianPerFixtureReductionPercent'])))<D('1e-12')
refs=load((ROOT/'tests/conformance/natal-references.json').read_bytes());angles=[]
for i,case in enumerate(refs['cases']):
 natal=load((E/f'natal-{i}.json').read_bytes());actual={x['id']:x['longitude'] for x in natal['data']['placements']}
 for baseline in ['swiss','skyfield']:
  positions=case[baseline]['positions'] if isinstance(case[baseline],dict) else case[baseline]
  ds=[]
  for j,point in enumerate(positions):
   body=point.get('id',case['swiss']['positions'][j]['id']);diff=abs(actual[body]-point['longitude'])%360;ds.append(min(diff,360-diff))
  peak=max(ds);saved=r['references'][i][baseline]['maxLongitudeErrorDegrees']
  assert abs(peak-D(str(saved)))<D('1e-15')
  angles.append({'case':case['name'],'source':baseline,'pairs':len(ds),'maxDegrees':str(peak)})
log=(E/'cargo-test.log').read_text();counts=[tuple(map(int,m)) for m in re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored',log)]
assert sum(x[0] for x in counts)==132 and all(x[1:]==(0,0) for x in counts)
checks={}
for name,h in r['artifactHashes'].items():
 f=ROOT/name
 checks[name]=f.exists() and sha(f.read_bytes())==h
assert all(checks.values())
text=(P.parent/'paper-source.md').read_text();body,bib=text.split('## Tài liệu tham khảo');bib=bib.split('## Thông tin công bố')[0]
used={int(x) for m in re.findall(r'\[(\d+(?:, \d+)*)\]',body) for x in m.split(', ')}
listed={int(x) for x in re.findall(r'^\[(\d+)\]',bib,re.M)}
assert used==listed==set(range(1,12))
report={'scope':'Reanalysis of archived raw outputs and source files; no new product run or new astronomical oracle','sourceHashesMatch':checks,'compression':rows,'totalB':B,'totalC':C,'reduction':str(100*(D(B)-C)/B),'median':str(statistics.median(rates)),'min':str(min(rates)),'max':str(max(rates)),'referenceChecks':angles,'testCountFromLog':132,'citationIds':sorted(used)}
(P/'final-review-v1.4.json').write_text(json.dumps(report,ensure_ascii=False,indent=2))
print('PASS: 17 raw roundtrips and hashes; byte totals and all rates; 80 longitude pairs; 132 test log entries; references 1–11.')
