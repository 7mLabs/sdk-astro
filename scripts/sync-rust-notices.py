"""Copy exact licenses for the Cargo.lock inventory and font to source/package notice folders.

Run after cargo fetch. Includes optional/build dependencies for source provenance;
does not assert every listed crate is linked into every target binary.
"""
import hashlib
import json
import shutil
import tomllib
from pathlib import Path

root = Path(__file__).resolve().parents[1]
registry = Path.home() / ".cargo/registry"
lock = tomllib.loads((root / "neutral-engine/Cargo.lock").read_text())
version = tomllib.loads((root / "neutral-engine/Cargo.toml").read_text())["workspace"]["package"]["version"]
destination = root / "third-party/rust"
entries = []
notice = ["Rust dependency notices", "=======================", "", "Exact upstream license texts for Cargo.lock, including optional/build dependencies.",
          "This inventory is source provenance, not a claim that every crate is linked into every binary.", ""]
for item in lock["package"]:
    if not item.get("source", "").startswith("registry+"):
        continue
    name, ver = item["name"], item["version"]
    source = next((registry / "src").glob(f"*/{name}-{ver}"))
    archive = next((registry / "cache").glob(f"*/{name}-{ver}.crate"))
    if hashlib.sha256(archive.read_bytes()).hexdigest() != item["checksum"]:
        raise ValueError(f"Archive checksum mismatch: {name} {ver}")
    cargo = tomllib.loads((source / "Cargo.toml").read_text())["package"]
    licenses = sorted(p for p in source.iterdir() if p.is_file() and ("LICENSE" in p.name.upper() or p.name.upper().startswith("COPYING")))
    if not licenses:
        raise ValueError(f"No license text found: {name} {ver}")
    records = []
    for p in licenses:
        filename = f"{name}-{ver}-{p.name}"
        shutil.copyfile(p, destination / filename)
        records.append({"path": filename, "sdkPackagePath": f"third-party/rust-{filename}", "bytes": p.stat().st_size,
                        "sha256": hashlib.sha256(p.read_bytes()).hexdigest()})
    entries.append({"name": name, "version": ver, "classification": "locked dependency; may be optional/build",
                    "license": cargo.get("license"), "repository": cargo.get("repository"), "source": item["source"],
                    "registryUrl": f"https://crates.io/crates/{name}/{ver}", "cargoPackageChecksum": item["checksum"],
                    "cachedCrateArchiveVerified": True, "licenses": records})
    notice += [f"- {name} {ver}: {cargo.get('license')}", f"  Source: https://crates.io/crates/{name}/{ver}"]
    notice += [f"  License text: {p['path']}" for p in records]
manifest = {"schemaVersion": "1", "engineVersion": version, "lockfile": "neutral-engine/Cargo.lock",
            "description": "Exact upstream license texts for all locked registry dependencies; optional/build entries do not imply runtime linkage.", "dependencies": entries}
(destination / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
(destination / "NOTICE").write_text("\n".join(notice) + "\n")
for folder in [root / "bindings/node/third-party", root / "bindings/python/sevenmlabs_astrology/third-party", root / "bindings/dotnet/third-party"]:
    for item in entries:
        for record in item["licenses"]:
            shutil.copyfile(destination / record["path"], folder / ("rust-" + record["path"]))
    shutil.copyfile(destination / "manifest.json", folder / "rust-manifest.json")
    shutil.copyfile(destination / "NOTICE", folder / "rust-NOTICE")
    shutil.copyfile(root / "neutral-engine/crates/astro-render-svg/assets/PlusJakartaSans-OFL.txt", folder / "PlusJakartaSans-OFL.txt")
print(f"Verified archives and copied license texts for {len(entries)} locked dependencies")
