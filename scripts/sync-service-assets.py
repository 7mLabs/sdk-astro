"""Regenerate committed render geometry from the brand kit. Build/runtime needs no Python/fontTools.

Usage: python scripts/sync-service-assets.py --brand-root ../brand/7mlabs-brand-kit-v1
Requires fontTools only when regenerating the outlined font asset.
"""
import argparse
import hashlib
import json
import shutil
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument("--brand-root", required=True, type=Path)
args = parser.parse_args()
from fontTools.ttLib import TTFont
from fontTools.pens.svgPathPen import SVGPathPen

root = Path(__file__).resolve().parents[1]
destination = root / "neutral-engine/crates/astro-render-svg/assets"
destination.mkdir(parents=True, exist_ok=True)
font = args.brand_root / "fonts/plusjakartasans/static/PlusJakartaSans-Medium.ttf"
license_file = args.brand_root / "fonts/plusjakartasans/OFL.txt"
zodiac = args.brand_root / "icons/zodiac/registry.json"
for source, filename in [(font, font.name), (license_file, "PlusJakartaSans-OFL.txt"), (zodiac, "zodiac.json")]:
    shutil.copyfile(source, destination / filename)
f = TTFont(font)
glyph_set = f.getGlyphSet()
outlines = {}
for code, name in f.getBestCmap().items():
    pen = SVGPathPen(glyph_set)
    glyph_set[name].draw(pen)
    outlines[chr(code)] = {"advance": f["hmtx"][name][0], "path": pen.getCommands()}
asset = {"font": "Plus Jakarta Sans Medium", "sha256": hashlib.sha256(font.read_bytes()).hexdigest(),
         "unitsPerEm": f["head"].unitsPerEm, "glyphs": outlines}
(destination / "font-outlines.json").write_text(json.dumps(asset, ensure_ascii=False, separators=(",", ":")) + "\n")
provenance = {"generator": "scripts/sync-service-assets.py", "fontToolsVersion": __import__("fontTools").__version__,
              "sourceFiles": {str(p.relative_to(args.brand_root)): hashlib.sha256(p.read_bytes()).hexdigest() for p in (font, license_file, zodiac)},
              "zodiacPack": "7mlabs-zodiac/0.5.0-draft", "fontLicense": "OFL-1.1",
              "glyphs": len(outlines), "planetGeometry": "Original conventional vector glyph geometry in src/symbols.rs"}
(destination / "provenance.json").write_text(json.dumps(provenance, indent=2) + "\n")
print(f"Generated {len(outlines)} outlined glyphs and the 7mlabs zodiac pack")
