"""Check the scoped MIT permission without granting rights to third-party files.

This is a provenance/packaging gate, not a legal ownership determination.
Run from any directory. It does not use the network or modify the inventory.
"""
import argparse
import difflib
import hashlib
import json
from pathlib import Path, PurePosixPath
import re


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read_relative(root, name):
    path = PurePosixPath(name)
    if path.is_absolute() or ".." in path.parts or not path.parts:
        raise ValueError(f"Unsafe scope path: {name}")
    source = root / name
    if source.is_symlink() or not source.resolve().is_relative_to(root.resolve()):
        raise ValueError(f"Scope path escapes source: {name}")
    return source.read_bytes()


def original_path(name):
    path = PurePosixPath(name)
    if set(path.parts) & {"vendor", "third-party", "native", "source", "fixtures", "target", "bin", "obj"}:
        return False
    if name.startswith("neutral-engine/"):
        return path.suffix in {".rs", ".c", ".h", ".toml"} or name == "neutral-engine/crates/astro-render-svg/assets/zodiac.json"
    if name.startswith("bindings/"):
        return path.suffix in {".cjs", ".ts", ".py", ".cs"}
    if name.startswith("schemas/"):
        return path.suffix == ".json"
    if name.startswith("examples/"):
        return path.suffix in {".rs", ".c", ".h", ".cjs", ".mjs", ".ts", ".py", ".cs", ".css", ".html", ".toml", ".csproj"}
    if name.startswith("tests/"):
        return path.suffix in {".rs", ".c", ".h", ".cjs", ".mjs", ".ts", ".py", ".cs", ".toml", ".csproj"}
    return False


def verify_delta(root, record):
    current = read_relative(root, record["modifiedFile"])
    if sha(current) != record["modifiedSha256"]:
        raise ValueError("Modified Swiss source no longer matches reviewed contribution")
    patch = record["unifiedDiff"]
    if sha(patch.encode()) != record["diffSha256"]:
        raise ValueError("Swiss contribution diff hash differs")
    # Reverse the reviewed unified diff in memory, then check pinned upstream
    # bytes. Neither the full mixed file nor unchanged diff context is MIT.
    lines = current.decode().splitlines(keepends=True)
    original, cursor, in_hunk = [], 0, False
    for line in patch.splitlines(keepends=True)[2:]:
        match = re.fullmatch(r"@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@[^\n]*\n", line)
        if match:
            start = int(match[3]) - 1
            if start < cursor:
                raise ValueError("Overlapping Swiss diff hunks")
            original.extend(lines[cursor:start])
            cursor, in_hunk = start, True
        elif in_hunk and line.startswith((" ", "+")):
            if cursor >= len(lines) or lines[cursor] != line[1:]:
                raise ValueError("Swiss diff does not match the reviewed mixed source")
            if line.startswith(" "):
                original.append(line[1:])
            cursor += 1
        elif in_hunk and line.startswith("-"):
            original.append(line[1:])
        else:
            raise ValueError("Unexpected Swiss diff format")
    original.extend(lines[cursor:])
    upstream = "".join(original)
    if sha(upstream.encode()) != record["upstreamSha256"]:
        raise ValueError("Swiss diff does not reconstruct the pinned upstream file")
    expected = "".join(difflib.unified_diff(upstream.splitlines(True), lines,
                                         fromfile="upstream/sweph.c", tofile="7mlabs/sweph.c"))
    if patch != expected:
        raise ValueError("Swiss contribution diff is not canonical")


def check(root):
    scope = json.loads(read_relative(root, "LICENSE-SCOPE.json"))
    if scope["schemaVersion"] != 1 or scope["permission"] != "MIT" or scope["copyrightHolder"] != "7mlabs":
        raise ValueError("Unexpected 7mlabs permission identity")
    if sha(read_relative(root, "LICENSE-7MLABS-MIT")) != scope["mitTextSha256"]:
        raise ValueError("Scoped MIT text differs from the reviewed grant")
    seen = set()
    for record in scope["originalFiles"]:
        name = record["path"]
        if name in seen or not original_path(name):
            raise ValueError(f"Duplicate or non-original path in MIT file scope: {name}")
        seen.add(name)
        if sha(read_relative(root, name)) != record["sha256"]:
            raise ValueError(f"Original source changed; review its license scope: {name}")
    if not seen:
        raise ValueError("Empty MIT scope")
    protected = set()
    for record in scope["thirdPartyFiles"]:
        name = record["path"]
        if name in seen or name in protected:
            raise ValueError(f"Overlapping or duplicate third-party scope: {name}")
        protected.add(name)
        if sha(read_relative(root, name)) != record["sha256"]:
            raise ValueError(f"Third-party material changed; retain/review its own terms: {name}")
    swiss_file = "neutral-engine/crates/astro-provider-swiss/vendor/sweph.c"
    if swiss_file not in protected or len(scope["swissChanges"]) != 1:
        raise ValueError("The mixed Swiss file must remain protected")
    delta = scope["swissChanges"][0]
    if delta["modifiedFile"] != swiss_file or delta["upstreamCommit"] != "aacf962d19d79f8bc921dbcdaacf306d85be1917":
        raise ValueError("Unexpected Swiss contribution identity")
    verify_delta(root, delta)
    companion_files = ("LICENSE-7MLABS-MIT", "LICENSE-SCOPE.md", "LICENSE-SCOPE.json")
    for directory in ("neutral-engine", "bindings/node", "bindings/python", "bindings/dotnet"):
        for name in companion_files:
            if read_relative(root, directory + "/" + name) != read_relative(root, name):
                raise ValueError(f"Stale license companion: {directory}/{name}")
    return {"status": "passed", "permission": "MIT for identified 7mlabs material only",
            "aggregatePackageLicense": "AGPL-3.0-only", "originalFileCount": len(seen),
            "thirdPartyFileCount": len(protected), "swissContributionCount": len(scope["swissChanges"])}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    try:
        print(json.dumps(check(args.root), indent=2))
    except (ValueError, KeyError, OSError, TypeError, IndexError) as error:
        raise SystemExit(f"License scope check failed: {error}")
