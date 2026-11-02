"""Compare development Davison against separately assembled Swiss and Skyfield/JPL.
Run with the pinned reference environment; one process, no concurrent workers.
"""
import argparse,ctypes,hashlib,json,math,random,sys
from pathlib import Path
from datetime import datetime,timedelta,timezone
import importlib.metadata as metadata
import swisseph as swe
import numpy as np
from skyfield.api import Loader,load_file
from skyfield.framelib import ecliptic_frame
from skyfield.nutationlib import iau2000a_radians,mean_obliquity
HERE=Path(__file__).resolve().parent
BODY_IDS=['sun','moon','mercury','venus','mars','jupiter','saturn','uranus','neptune','pluto']
TARGETS=[10,301,199,299,4,5,6,7,8,9]
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def diff(a,b):return (a-b+180.)%360.-180.
def birth(dt,lat,lon):return {'utc':dict(year=dt.year,month=dt.month,day=dt.day,hour=dt.hour,minute=dt.minute,second=dt.second+dt.microsecond/1e6),'location':dict(latitude=lat,longitude=lon),'houseSystem':'wholeSign'}
def cases():
 t=lambda y,m,d,h=12:datetime(y,m,d,h,tzinfo=timezone.utc)
 pairs=[('identity',birth(t(2000,1,1),10.8,106.6),birth(t(2000,1,1),10.8,106.6)),
 ('dateline',birth(t(1980,1,1),35,179),birth(t(1999,6,1),-20,-179)),
 ('leap-and-century',birth(t(1900,3,1),51.5,-.12),birth(t(2000,2,29),-33.86,151.21)),
 ('utc-midnight',birth(t(2016,12,31,23),0,0),birth(t(2017,1,1,0),0,20)),
 ('future',birth(t(2030,4,1),60,-100),birth(t(2050,12,1),-50,140)),
 ('wide-time',birth(t(1900,1,1),-40,-150),birth(t(2050,1,1),45,130))]
 rng=random.Random(20261102)
 for i in range(10):
  make=lambda:birth(t(1900,1,1)+timedelta(seconds=rng.randrange(150*365*86400),microseconds=rng.choice([0,125000,750000])),rng.uniform(-60,60),rng.uniform(-180,180))
  pairs.append((f'random-{i:02}',make(),make()))
 return [dict(id=f'{name}-{variant}-{system}',request=dict(operation='davison',variant=variant,houseSystem=system,personA=a,personB=b,domains=['love'])) for name,a,b in pairs for variant in ['uncorrected','corrected'] for system in ['placidus','wholeSign']]
def jd_of(b):
 u=b['utc'];return swe.utc_to_jd(u['year'],u['month'],u['day'],u['hour'],u['minute'],u['second'],swe.GREG_CAL)[1]
def houses(jd,loc,system):return swe.houses_ex(jd,loc['latitude'],loc['longitude'],b'P' if system=='placidus' else b'W',0)
def oracle(r):
 # Mirror only provider settings, not SDK results. The independently packaged
 # Swiss library is a shared astronomical model, explicitly disclosed in protocol.
 swe.close();swe.deltat_ex(2451545.,swe.FLG_MOSEPH);swe.set_tid_acc(-25.580)
 a,b=r['personA'],r['personB'];ja,jb=jd_of(a),jd_of(b)
 loc={k:(a['location'][k]+b['location'][k])/2 for k in ['latitude','longitude']}
 mean=(ja+jb)/2; jd=mean
 if r['variant']=='corrected':
  ma=houses(ja,a['location'],a['houseSystem'])[1][1];mb=houses(jb,b['location'],b['houseSystem'])[1][1]
  target=(ma+diff(mb,ma)/2)%360
  # Newton on MC, unlike the SDK's bisection; numerical derivative from houses.
  for _ in range(12):
   mc=houses(jd,loc,r['houseSystem'])[1][1];res=diff(target,mc)
   if abs(res)<1e-9:break
   step=.00001;deriv=diff(houses(jd+step,loc,r['houseSystem'])[1][1],houses(jd-step,loc,r['houseSystem'])[1][1])/(2*step)
   jd+=res/deriv
  if abs(diff(target,houses(jd,loc,r['houseSystem'])[1][1]))>1e-6:raise RuntimeError('Reference Newton solver failed')
 coords=[swe.calc_ut(jd,i,swe.FLG_MOSEPH|swe.FLG_SPEED)[0] for i in range(10)]
 cusps,angles=houses(jd,loc,r['houseSystem'])
 return dict(julianDayUt1=jd,location=loc,bodies=coords,houses=cusps,angles=angles)
def main():
 parser=argparse.ArgumentParser();parser.add_argument('--library',type=Path,required=True);parser.add_argument('--kernel',type=Path,required=True);parser.add_argument('--output',type=Path,required=True);args=parser.parse_args()
 protocol=json.loads((HERE/'protocol.json').read_text());assert sha(args.kernel)==protocol['kernel']['sha256']
 args.output.mkdir(exist_ok=False,parents=True);cs=cases();(args.output/'cases.json').write_text(json.dumps(cs,indent=2)+'\n')
 lib=ctypes.CDLL(str(args.library.resolve()));calc=lib.astro_calculate_json;calc.argtypes=[ctypes.c_char_p];calc.restype=ctypes.c_void_p;free=lib.astro_free_string;free.argtypes=[ctypes.c_void_p]
 ephem=load_file(str(args.kernel));ts=Loader(str(args.output/'skyfield-cache')).timescale(builtin=True)
 failures=[];rows=[];payloads=[];refs=[];limits=protocol['thresholds']
 def check(case,metric,error):
  error=float(error)
  rows.append(dict(caseId=case,metric=metric,error=error,limit=limits[metric],passed=error<=limits[metric]))
  if not error<=limits[metric]:failures.append(rows[-1])
 for ix,case in enumerate(cs):
  req=case['request'];p=calc(json.dumps(req).encode())
  try:v=json.loads(ctypes.string_at(p))
  finally:free(p)
  payloads.append(dict(caseId=case['id'],response=v))
  if v['errors']:failures.append(dict(caseId=case['id'],errors=v['errors']));continue
  o=oracle(req);refs.append(dict(caseId=case['id'],reference=o));d=v['data']['natal'];meta=v['calculation']
  check(case['id'],'derivedTimeSeconds',abs(meta['julianDayUt1']-o['julianDayUt1'])*86400)
  for key in ['latitude','longitude']:check(case['id'],'geographicDegrees',abs(d['location'][key]-o['location'][key]))
  for body,coords in zip(d['placements'],o['bodies']):
   check(case['id'],'swissLongitudeDegrees',abs(diff(body['longitude'],coords[0])))
   check(case['id'],'swissSpeedDegreesPerDay',abs(body['speed']-coords[3]))
  for a,b in zip(d['houseCusps'],o['houses']):check(case['id'],'swissHouseDegrees',abs(diff(a,b)))
  # Independent angle geometry from Skyfield's sidereal time and nutation.
  t=ts.ut1_jd(meta['julianDayUt1']);eps=math.radians(mean_obliquity(t.tdb)/3600)+iau2000a_radians(t)[1]
  theta=math.radians(t.gast*15+d['location']['longitude']);lat=math.radians(d['location']['latitude'])
  mc=math.degrees(math.atan2(math.sin(theta),math.cos(theta)*math.cos(eps)))%360
  asc=(math.degrees(math.atan2(-math.cos(theta),math.sin(theta)*math.cos(eps)+math.tan(lat)*math.sin(eps)))+180)%360
  byid={x['id']:x['longitude'] for x in d['angles']}
  check(case['id'],'independentAngleDegrees',abs(diff(byid['midheaven'],mc)));check(case['id'],'independentAngleDegrees',abs(diff(byid['ascendant'],asc)))
  t=ts.tt_jd(meta['julianDayTt']);observer=ephem[399].at(t)
  for body,target in zip(d['placements'],TARGETS):
   ref_lat,ref_lon,_=observer.observe(ephem[target]).apparent().frame_latlon(ecliptic_frame)
   check(case['id'],'jplLongitudeDegrees',abs(diff(body['longitude'],ref_lon.degrees)))
   check(case['id'],'jplLatitudeDegrees',abs(body['latitude']-ref_lat.degrees))
  if ix%16==0:print(f'Compared {ix+1}/{len(cs)} charts',flush=True)
 for name,values in [('native',payloads),('reference',refs),('comparisons',rows)]:
  with (args.output/f'{name}.ndjson').open('w') as f:
   for row in values:f.write(json.dumps(row,separators=(',',':'))+'\n')
 metrics={k:dict(count=len([r for r in rows if r['metric']==k]),maxError=max([r['error'] for r in rows if r['metric']==k],default=None),limit=limit) for k,limit in limits.items()}
 result=dict(result='passed' if not failures else 'failed',charts=len(cs),checks=len(rows),failures=failures,metrics=metrics,librarySha256=sha(args.library),sourceSha256={str(p.relative_to(HERE.parents[2])):sha(p) for p in sorted((HERE.parents[2]/'neutral-engine/crates').glob('*/src/*.rs'))},kernelSha256=sha(args.kernel),protocolSha256=sha(HERE/'protocol.json'),scriptSha256=sha(__file__),dependencies={p:metadata.version(p) for p in ['skyfield','pyswisseph','numpy','jplephem']},limits=protocol['limits'])
 (args.output/'summary.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2));return bool(failures)
if __name__=='__main__':sys.exit(main())
