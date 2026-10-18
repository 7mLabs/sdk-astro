"""Verify new services against fresh installed Node/Python/NuGet packages and C ABI.

Run after build-packages.py and test-packages.py. No registry access. Writes only
generated consumer probes/evidence under artifacts/. Optional --typescript checks
the public declarations with an already installed compiler.
"""
import argparse
import hashlib
import json
import os
import subprocess
import sys
from pathlib import Path
from xml.etree import ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]
PACKAGES = ROOT / "artifacts/packages"
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--typescript", type=Path)
args = parser.parse_args()
manifest = json.loads((PACKAGES / "manifest.json").read_text())
fingerprint = hashlib.sha256(json.dumps(manifest["files"], sort_keys=True).encode()).hexdigest()[:12]
consumer = ROOT / "artifacts/consumers" / fingerprint
for name, digest in manifest["files"].items():
    assert hashlib.sha256((PACKAGES / name).read_bytes()).hexdigest() == digest, name
assert (consumer / "node/node_modules/@7mlabs/astrology/index.cjs").exists(), "Run test-packages.py first"
work = ROOT / "artifacts/time-svg" / fingerprint
work.mkdir(parents=True, exist_ok=True)
node = os.environ.get("NODE_BIN", "node")

def run(cmd, *, input=None, cwd=ROOT):
    return subprocess.run(list(map(str, cmd)), input=input, text=True, capture_output=True, check=True, cwd=cwd).stdout

node_probe = consumer / "node/service-probe.cjs"
node_probe.write_text("""const fs=require('node:fs');const s=require('@7mlabs/astrology');
const input=fs.readFileSync(0,'utf8');const mode=process.argv[2];
const output=mode==='resolve'?s.resolveBirthTimeJson(input):mode==='local'?s.calculateFromLocalJson(input):mode==='render'?s.renderChartSvgRequestJson(input):JSON.stringify({data:s.getCapabilities()});
process.stdout.write(output);""")
python_probe = work / "probe.py"
python_probe.write_text("""import json,sys
import sevenmlabs_astrology as s
value=sys.stdin.read(); mode=sys.argv[1]
if mode=='resolve': output=s.resolve_birth_time_json(value)
elif mode=='local': output=s.calculate_from_local_json(value)
elif mode=='render':
 r=json.loads(value); output=s.render_chart_svg_json(json.dumps(r['payload']),json.dumps(r.get('options',{})))
 if r.get('options',{}).get('output')=='model':
  options={k:v for k,v in r.get('options',{}).items() if k!='output'}
  assert s.build_chart_model(r['payload'],options)==json.loads(output)['data']['model']
else: output=json.dumps({'data':s.get_capabilities()})
print(output)
""")
dotnet = work / "dotnet"
dotnet.mkdir(exist_ok=True)
(dotnet / "Probe.csproj").write_text(f'''<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net10.0</TargetFramework><ImplicitUsings>enable</ImplicitUsings><Nullable>enable</Nullable></PropertyGroup><ItemGroup><PackageReference Include="SevenMLabs.Astrology" Version="{manifest['engineVersion']}" /></ItemGroup></Project>''')
(dotnet / "Program.cs").write_text('''using SevenMLabs.Astrology;
using System.Text.Json;
var input=Console.In.ReadToEnd();
if(args[0]=="resolve") Console.Write(Engine.ResolveBirthTimeJson(input));
else if(args[0]=="local") Console.Write(Engine.CalculateFromLocalJson(input));
else if(args[0]=="render") {using var doc=JsonDocument.Parse(input);var r=doc.RootElement; var result=Engine.RenderChartSvgJson(r.GetProperty("payload").GetRawText(),r.TryGetProperty("options",out var o)?o.GetRawText():"{}");
if(r.TryGetProperty("options",out var opts)&&opts.TryGetProperty("output",out var output)&&output.GetString()=="model") {using var check=JsonDocument.Parse(result); if(Engine.BuildChartModel(r.GetProperty("payload"),opts).GetRawText()!=check.RootElement.GetProperty("data").GetProperty("model").GetRawText()) throw new Exception("Model helper differs");}
Console.Write(result);}
else Console.Write(JsonSerializer.Serialize(new {data=Engine.GetCapabilities()}));
''')
run(["dotnet", "restore", "Probe.csproj", "--source", PACKAGES, "--packages", consumer / "nuget-cache", "-p:NuGetAudit=false", "--verbosity", "quiet"], cwd=dotnet)
run(["dotnet", "build", "Probe.csproj", "--no-restore", "--verbosity", "quiet"], cwd=dotnet)
c = work / "probe.c"
c.write_text('''#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "astro_engine.h"
int main(int argc,char **argv){if(argc!=2)return 2;char *in=calloc(33554433,1);if(!in)return 3;size_t n=fread(in,1,33554432,stdin);in[n]=0;
char *out=!strcmp(argv[1],"resolve")?astro_resolve_birth_time_json(in):!strcmp(argv[1],"local")?astro_calculate_from_local_json(in):!strcmp(argv[1],"render")?astro_render_chart_svg_json(in):astro_capabilities_json(in);
if(!out){free(in);return 4;}fputs(out,stdout);astro_free_string(out);free(in);return 0;}
''')
native = ROOT / "neutral-engine/target/release"
compiler = "clang" if sys.platform == "darwin" else "cc"
run([compiler, c, "-I", ROOT / "neutral-engine/include", "-L", native, "-lastro_engine", "-Wl,-rpath," + str(native), "-o", work / "probe"])
commands = {"node": [node, node_probe], "python": [consumer / "python/venv/bin/python", python_probe],
            "dotnet": ["dotnet", dotnet / "bin/Debug/net10.0/Probe.dll"], "c": [work / "probe"]}

def probe(mode, value):
    raw = json.dumps(value, ensure_ascii=False, separators=(",", ":"))
    outputs = {name: json.loads(run(command + [mode], input=raw)) for name, command in commands.items()}
    ref = outputs["node"]
    if mode == "capabilities":
        outputs = {name: output["data"] for name, output in outputs.items()}
        ref = outputs["node"]
    for name, output in outputs.items():
        assert output == ref, f"{mode}: {name} differs from Node"
    return ref

birth = {"local": {"year": 1995, "month": 6, "day": 15, "hour": 14, "minute": 30, "second": 12.125}, "timeZone": "Asia/Ho_Chi_Minh"}
location = {"latitude": 10.8231, "longitude": 106.6297}
capabilities = probe("capabilities", {})
resolved = probe("resolve", birth)
assert resolved["errors"] == []
fold = {"local": {"year": 2024, "month": 11, "day": 3, "hour": 1, "minute": 30}, "timeZone": "America/New_York"}
time_cases = [birth, fold, {**fold, "disambiguation": "earlier"}, {**fold, "disambiguation": "later"},
              {"local": {"year": 2024, "month": 3, "day": 10, "hour": 2, "minute": 30}, "timeZone": "America/New_York"},
              {"local": {"year": 2399, "month": 7, "day": 15, "hour": 12, "minute": 0}, "timeZone": "America/New_York"},
              {"local": {"year": 1900, "month": 1, "day": 1, "hour": 0, "minute": 0}, "utcOffsetSeconds": 2079},
              {"local": {"year": 1800, "month": 1, "day": 1, "hour": 0, "minute": 0}, "timeZone": "Asia/Ho_Chi_Minh"},
              {"local": {"year": 2023, "month": 2, "day": 29, "hour": 0, "minute": 0}, "timeZone": "Etc/UTC"}]
for case in time_cases:
    probe("resolve", case)
natal = probe("local", {"operation": "natal", **birth, "location": location})
assert natal["errors"] == []
request = json.loads((ROOT / "examples/couple-request.json").read_text())
request["personA"].pop("utc")
request["personA"].update(birth)
couple = probe("local", request)
assert couple["errors"] == []
probe("local", {**birth, "utc": resolved["data"]["utc"], "location": location})
svg_cases = 0
for detail in ["basic", "advanced", "professional"]:
    for theme in ["dark", "light"]:
        out = probe("render", {"payload": natal, "options": {"detail": detail, "theme": theme}})
        assert out["errors"] == []
        svg = out["data"]["svg"]
        tree = ET.fromstring(svg)
        assert tree.tag == "{http://www.w3.org/2000/svg}svg"
        assert not any(bad in svg for bad in ["<text", "<script", "<image", "href=", "var(--"])
        metadata = json.loads(tree.find("{http://www.w3.org/2000/svg}metadata").text)
        source = {p["id"]: p["longitude"] for p in metadata["geometry"][0]["placements"]}
        markers = {n.attrib["data-point-id"]: float(n.attrib["data-longitude"]) for n in tree.iter() if "data-point-id" in n.attrib}
        assert source == markers
        (work / f"natal-{detail}-{theme}.svg").write_text(svg)
        svg_cases += 1
extended = probe("local", {"operation":"natal", **birth, "location":location,"additionalPoints":"all","aspectPreset":"extended"})
assert extended["errors"] == []
assert len(extended["data"]["additionalPoints"]) == 7
assert extended["data"]["placements"] == natal["data"]["placements"]
for theme in ["dark","light"]:
    out = probe("render", {"payload":extended,"options":{"detail":"professional","theme":theme,"aspectColors":{"harmonious":"#29b999","square":"#e25b79"},"maxAspectRows":4096}})
    assert out["errors"] == []
    assert out["data"]["coverage"]["renderedBodies"] == 17
    assert out["data"]["coverage"]["aspectTableOmitted"] == 0
    tree = ET.fromstring(out["data"]["svg"])
    assert len([n for n in tree.iter() if "data-point-id" in n.attrib]) == 17
    (work / f"extended-professional-{theme}.svg").write_text(out["data"]["svg"])
    svg_cases += 1
probe("render", {"payload":extended,"options":{"detail":"professional","aspectGroups":["harmonious"],"maxOrb":2.,"aspectPhase":"applying"}})
domain = probe("local", {"operation":"natalDomains", **birth, "location":location,"domains":["identity"],"additionalPoints":"standard"})
assert domain["errors"] == []
for part in ["wheel","positions","houses","aspectMatrix","aspects","houseFocus"]:
    options={"sections":[part],"focus":{"house":7}} if part=="houseFocus" else {"sections":[part]}
    out=probe("render", {"payload":domain,"options":options})
    assert out["errors"] == []
    tree=ET.fromstring(out["data"]["svg"])
    assert len([n for n in tree.iter() if "data-part" in n.attrib]) == 1
    assert out["data"]["model"]["sections"][0]["part"] == part
    svg_cases += 1
model=probe("render", {"payload":domain,"options":{"sections":["positions","wheel"],"output":"model","focus":{"house":7,"mode":"isolate"}}})
assert model["errors"] == [] and "svg" not in model["data"]
assert model["data"]["model"]["focus"]["house"]["number"] == 7
out = probe("render", {"payload": couple, "options": {"chart": "synastry", "detail": "advanced", "maxAspectLines": 0}})
assert out["data"]["coverage"]["renderedBodies"] == 20
svg_cases += 1
env = {**os.environ, "ASTRO_ENGINE_MODULE": str(consumer / "node/node_modules/@7mlabs/astrology/index.cjs")}
subprocess.run([node, str(ROOT / "tests/services/time-svg.cjs")], cwd=ROOT, env=env, check=True)
if args.typescript:
    types = consumer / "node/time-svg-types.ts"
    types.write_text('''import {calculateFromLocal,resolveBirthTime,renderChartSvg,renderChartPart,buildChartModel,getCapabilities} from '@7mlabs/astrology';
const t=resolveBirthTime({local:{year:1995,month:6,day:15,hour:14,minute:30},timeZone:'Asia/Ho_Chi_Minh'});
const p=calculateFromLocal({local:t.local,timeZone:'Asia/Ho_Chi_Minh',location:{latitude:10.8,longitude:106.6}});
const extended=calculateFromLocal({local:t.local,timeZone:'Etc/UTC',location:{latitude:10.8,longitude:106.6},additionalPoints:'standard',aspectPreset:'extended'});
if(extended.data.additionalPoints) { const definition:string=extended.data.additionalPoints[0].definition; }
const svg:string=renderChartSvg(p,{detail:'professional',aspectColors:{harmonious:'#28b',trine:'#2c9'},aspectGroups:['harmonious'],display:{aspectMatrix:true},bodyIds:['chiron','trueNorthNode','sun'],planetSymbols:{sun:{viewBox:[0,0,32,32],nodes:[['circle',{cx:16,cy:16,r:8}]]}}});
const sectionSvg:string=renderChartPart(extended,'houses');
const m=buildChartModel(extended,{sections:['wheel','houseFocus'],focus:{house:7}});
const x:number=m.hitTargets[0].center.x;
// @ts-expect-error focus cannot combine house and point selection
buildChartModel(extended,{focus:{house:7,pointIds:['sun']}});
const version:string=getCapabilities().civilTime.tzdbVersion;
const convertedYear:number=p.calculation.timeResolution.birth.utc.year;
const domain=calculateFromLocal({operation:'natalDomains',local:t.local,timeZone:'Etc/UTC',location:{latitude:0,longitude:0}});
if(domain.data) { const kind:'individualNatal'=domain.data.chartKind; }
// @ts-expect-error provide exactly one zone/offset
resolveBirthTime({local:t.local,timeZone:'Etc/UTC',utcOffsetSeconds:0});
// @ts-expect-error renderer detail enum
renderChartSvg(p,{detail:'incorrect'});
''')
    run([node, args.typescript.resolve(), "--strict", "--noEmit", "--target", "ES2022", "--module", "commonjs", types])
summary = {"passed": True, "engineVersion": manifest["engineVersion"], "platform": manifest["platform"], "arch": manifest["arch"],
           "bindings": list(commands), "timeCases": len(time_cases), "localCalculationCases": 5, "svgCases": svg_cases,
           "fullJsonAndSvgParity": True, "tzdbVersion": capabilities["civilTime"]["tzdbVersion"], "typescript": bool(args.typescript), "artifactFingerprint": fingerprint}
(ROOT / "artifacts/time-svg-tests.json").write_text(json.dumps(summary, indent=2) + "\n")
print(json.dumps(summary, indent=2))
