from pathlib import Path
from decimal import Decimal as D
import json
P=Path(__file__).parent
r=json.loads((P/'review-v1.1/results.json').read_text())
B=sum(x['inputBytes'] for x in r['compression']);C=sum(x['outputBytes'] for x in r['compression'])
r_total=D(100)*(B-C)/B
weighted=sum(D(x['inputBytes'])/B*(D(100)*(x['inputBytes']-x['outputBytes'])/x['inputBytes']) for x in r['compression'])
assert abs(weighted-r_total)<D('1e-24')
assert sum(x['inputBytes'] for x in r['compression'])==r['totalInputBytes']
assert sum(x['outputBytes'] for x in r['compression'])==r['totalOutputBytes']
def angle(a,b):
 delta=abs(D(a)-D(b))%D(360)
 return min(delta,D(360)-delta)
cases=[('359','1','2'),('0','360','0'),('0','180','180'),('-1','1','2')]
for a,b,d in cases:assert angle(a,b)==D(d)
assert D(900)*D('.5')/(900+100)*100==45
out={'scope':'Kiểm tra phép tính minh họa; không thay chứng minh hoặc kiểm thử sản phẩm','totalSourceBytes':B,'totalContextBytes':C,'reductionPercent':str(r_total),'weightedReductionPercent':str(weighted),'remainingPercent':str(D(100)*C/B),'arcExamples':cases,'arcsecondsFromRoundedReportedDegrees':str(D('0.0001213184')*3600),'counterexampleUnweightedMean':25,'counterexampleWeightedTotal':45}
(P/'math-check-v1.3.json').write_text(json.dumps(out,ensure_ascii=False,indent=2));print(json.dumps(out,ensure_ascii=False,indent=2))
