"""Recheck saved comparison counts, thresholds, maxima and file hashes (stdlib only)."""
import gzip,hashlib,json
from pathlib import Path
root=Path(__file__).resolve().parent
result=root/'results'
summary=json.loads((result/'summary.json').read_text())
protocol=json.loads((root/'protocol.json').read_text())
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def rows(name):
 p=result/f'{name}.ndjson';z=result/f'{name}.ndjson.gz'
 with (p.open() if p.exists() else gzip.open(z,'rt')) as f:return [json.loads(line) for line in f]
cases=json.loads((result/'cases.json').read_text());native=rows('native');reference=rows('reference');checks=rows('comparisons')
assert len(cases)==len(native)==len(reference)==summary['charts']==64
assert len({v['id'] for v in cases})==64
assert summary['protocolSha256']==digest(root/'protocol.json')
assert summary['scriptSha256']==digest(root/'run.py')
assert all(not v['response']['errors'] for v in native)
assert len(checks)==summary['checks']
for key,limit in protocol['thresholds'].items():
 selected=[r for r in checks if r['metric']==key]
 assert selected and all(r['limit']==limit and r['passed']==(r['error']<=limit) for r in selected)
 assert summary['metrics'][key]==dict(count=len(selected),maxError=max(r['error'] for r in selected),limit=limit)
assert summary['failures']==[r for r in checks if not r['passed']]
assert summary['result']==('failed' if summary['failures'] else 'passed')
manifest={str(p.relative_to(root)):digest(p) for p in sorted(root.rglob('*')) if p.is_file() and p.name not in ['manifest.json','verification.json'] and '__pycache__' not in p.parts}
(root/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
report=dict(result=summary['result'],charts=len(cases),checks=len(checks),failedChecks=len(summary['failures']),savedEvidence='hashes and summary statistics verified; does not rerun reference calculations')
(root/'verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
