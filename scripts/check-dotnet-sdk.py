"""Require the reviewed .NET build SDK instead of a newer preinstalled SDK."""
from pathlib import Path
import json
import subprocess

root = Path(__file__).resolve().parents[1]
sdk = json.loads((root / 'global.json').read_text())['sdk']
assert sdk['rollForward'] == 'disable' and sdk['allowPrerelease'] is False
actual = subprocess.check_output(['dotnet', '--version'], cwd=root, text=True).strip()
print(json.dumps({'expectedSdk': sdk['version'], 'actualSdk': actual}))
if actual != sdk['version']:
    raise SystemExit('The .NET build SDK must match global.json exactly')
