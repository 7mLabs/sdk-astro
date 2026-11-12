"""Regression checks for permission boundaries, not legal ownership assertions."""
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location("license_scope", ROOT / "scripts/check-license-scope.py")
scope_check = importlib.util.module_from_spec(spec)
spec.loader.exec_module(scope_check)


class LicenseScopeTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.scope = json.loads((ROOT / "LICENSE-SCOPE.json").read_text())
        names = {r["path"] for r in self.scope["originalFiles"] + self.scope["thirdPartyFiles"]}
        for directory in ("", "neutral-engine/", "bindings/node/", "bindings/python/", "bindings/dotnet/"):
            names.update(directory + name for name in ("LICENSE-7MLABS-MIT", "LICENSE-SCOPE.md", "LICENSE-SCOPE.json"))
        for name in names:
            target = self.root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes((ROOT / name).read_bytes())

    def save_scope(self):
        data = (json.dumps(self.scope, indent=2) + "\n").encode()
        for directory in ("", "neutral-engine/", "bindings/node/", "bindings/python/", "bindings/dotnet/"):
            (self.root / directory / "LICENSE-SCOPE.json").write_bytes(data)

    def test_reviewed_source_passes(self):
        self.assertEqual(scope_check.check(self.root)["status"], "passed")

    def test_swiss_whole_file_cannot_be_added_to_mit_scope(self):
        self.scope["originalFiles"].append(copy.deepcopy(self.scope["thirdPartyFiles"][0]))
        self.save_scope()
        with self.assertRaisesRegex(ValueError, "non-original path"):
            scope_check.check(self.root)

    def test_font_outlines_cannot_be_added_to_mit_scope(self):
        record = next(r for r in self.scope["thirdPartyFiles"] if r["path"].endswith("font-outlines.json"))
        self.scope["originalFiles"].append(copy.deepcopy(record))
        self.save_scope()
        with self.assertRaisesRegex(ValueError, "non-original path"):
            scope_check.check(self.root)

    def test_changed_source_requires_review(self):
        source = self.root / self.scope["originalFiles"][0]["path"]
        source.write_bytes(source.read_bytes() + b"\n")
        with self.assertRaisesRegex(ValueError, "review its license scope"):
            scope_check.check(self.root)

    def test_changed_third_party_material_is_detected(self):
        source = self.root / self.scope["thirdPartyFiles"][0]["path"]
        source.write_bytes(source.read_bytes() + b"\n")
        with self.assertRaisesRegex(ValueError, "Third-party material changed"):
            scope_check.check(self.root)

    def test_rehashed_but_false_swiss_diff_is_rejected(self):
        record = self.scope["swissChanges"][0]
        record["unifiedDiff"] = record["unifiedDiff"].replace("return NULL;", "return 0;")
        record["diffSha256"] = hashlib.sha256(record["unifiedDiff"].encode()).hexdigest()
        self.save_scope()
        with self.assertRaisesRegex(ValueError, "diff does not match"):
            scope_check.check(self.root)

    def test_stale_installed_companion_is_detected(self):
        (self.root / "bindings/python/LICENSE-7MLABS-MIT").write_text("stale")
        with self.assertRaisesRegex(ValueError, "Stale license companion"):
            scope_check.check(self.root)


if __name__ == "__main__":
    unittest.main()
