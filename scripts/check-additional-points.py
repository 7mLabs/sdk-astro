"""Check embedded point integration against standard Swiss file I/O, not JPL accuracy.
Build the local release first. Uses only the pinned vendored source/data; no network.
"""
import ctypes
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
PROVIDER = ROOT / 'neutral-engine/crates/astro-provider-swiss'
DATA = PROVIDER / 'data/seas_18.se1'
assert hashlib.sha256(DATA.read_bytes()).hexdigest() == 'a2cd8fc33807c78ca9a700c91c2e042258b12fc4796519e00781440b5ad8b2e2'
native = ROOT / 'neutral-engine/target/release' / ('libastro_engine.dylib' if sys.platform == 'darwin' else 'libastro_engine.so')
lib = ctypes.CDLL(str(native))
lib.astro_calculate_json.argtypes = [ctypes.c_char_p]
lib.astro_calculate_json.restype = ctypes.c_void_p
lib.astro_free_string.argtypes = [ctypes.c_void_p]
with tempfile.TemporaryDirectory(prefix='astro-point-reference-') as temp:
    work = Path(temp)
    # Restore normal upstream file I/O in a separate reference executable.
    source = (PROVIDER / 'vendor/sweph.c').read_text()
    start = source.index('#ifdef ASTRO_MOSHIER_ONLY', source.index('FILE *swi_fopen('))
    end = source.index('#endif', start) + len('#endif')
    (work / 'sweph.c').write_text(source[:start] + source[end:])
    (work / 'probe.c').write_text('''#include <stdio.h>
#include <stdlib.h>
#include "swephexp.h"
int main(int argc,char **argv){
 if(argc!=5)return 2; swe_set_ephe_path(argv[1]); double jd[2],pos[6];char err[256]={0};
 swe_deltat_ex(2451545.,4,NULL);swe_set_tid_acc(-25.580);
 if(swe_utc_to_jd(atoi(argv[2]),atoi(argv[3]),atoi(argv[4]),12,34,56.125,1,jd,err)<0)return 3;
 int ids[]={15,11,10,12,13};
 for(int i=0;i<5;i++){int flags=swe_calc_ut(jd[1],ids[i],4|256,pos,err);if(flags<0){fprintf(stderr,"%s",err);return 4;}
 printf("%d",ids[i]);for(int j=0;j<6;j++)printf(" %.17g",pos[j]);puts("");}
 swe_close();return 0;}
''')
    files = ['swedate','swehouse','swejpl','swemmoon','swemplan','swephlib','swecl','swehel']
    cmd = ['cc','-O3','-std=c99','-I',str(PROVIDER/'vendor'),str(work/'probe.c'),str(work/'sweph.c')]
    if sys.platform != 'darwin': cmd += ['-D_GNU_SOURCE']
    cmd += [str(PROVIDER/'vendor'/f'{name}.c') for name in files]
    cmd += ['-lm','-o',str(work/'probe')]
    if sys.platform != 'darwin': cmd += ['-ldl']
    subprocess.run(cmd,check=True,capture_output=True)
    maximum = 0.
    dates = [(1800,1,1),(1800,12,31),(1900,2,28),(1997,2,8),(2000,2,29),(2026,10,18),(2100,3,1),(2300,12,31),(2399,12,31)]
    for year,month,day in dates:
        reference = subprocess.check_output([str(work/'probe'),str(DATA.parent),str(year),str(month),str(day)],text=True)
        rows = {int(parts[0]):list(map(float,parts[1:])) for parts in (line.split() for line in reference.splitlines())}
        request = {'operation':'natal','utc':{'year':year,'month':month,'day':day,'hour':12,'minute':34,'second':56.125},'location':{'latitude':10.8,'longitude':106.6},'additionalPoints':'all'}
        pointer = lib.astro_calculate_json(json.dumps(request).encode())
        try: result = json.loads(ctypes.string_at(pointer))
        finally: lib.astro_free_string(pointer)
        assert result['errors'] == [], result
        points = {p['id']:p for p in result['data']['additionalPoints']}
        for name,body in [('chiron',15),('trueNorthNode',11),('meanNorthNode',10),('meanLilith',12),('osculatingLilith',13)]:
            for key,index in [('longitude',0),('latitude',1),('distanceAu',2),('speed',3)]:
                diff = abs(points[name][key]-rows[body][index]);maximum=max(maximum,diff)
                assert diff < 1e-10, (year,name,key,diff)
        for method in ['true','mean']:
            north,south=points[method+'NorthNode'],points[method+'SouthNode']
            assert abs((south['longitude']-north['longitude'])%360-180)<1e-10
    report={'passed':True,'reference':'same pinned Swiss source with ordinary fopen versus embedded adapter; integration parity, not independent ephemeris accuracy','dates':len(dates),'definitions':7,'maximumAbsoluteDifference':maximum,'dataSha256':hashlib.sha256(DATA.read_bytes()).hexdigest()}
    (ROOT/'artifacts/additional-points-tests.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(report,indent=2))
