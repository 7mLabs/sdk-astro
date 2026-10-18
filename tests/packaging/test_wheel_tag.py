"""A universal Python interpreter must not advertise a universal native SDK."""
from pathlib import Path
import runpy
import unittest
from unittest.mock import patch

from setuptools import Distribution
from setuptools.command.bdist_wheel import bdist_wheel


class WheelTargetTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        with patch("setuptools.setup"):
            cls.wheel = runpy.run_path(str(Path(__file__).resolve().parents[2] / "bindings/python/setup.py"))["PlatformWheel"]

    def tag(self, system, architecture, inherited):
        with patch.object(bdist_wheel, "get_tag", return_value=("cp313", "cp313", inherited)), patch("platform.system", return_value=system), patch("platform.machine", return_value=architecture):
            return self.wheel(Distribution()).get_tag()

    def test_universal_interpreter_arm_native(self):
        self.assertEqual(self.tag("Darwin", "arm64", "macosx_11_0_universal2"), ("py3", "none", "macosx_11_0_arm64"))

    def test_universal_interpreter_intel_native(self):
        self.assertEqual(self.tag("Darwin", "x86_64", "macosx_11_0_universal2"), ("py3", "none", "macosx_11_0_x86_64"))

    def test_single_arch_interpreter(self):
        self.assertEqual(self.tag("Darwin", "arm64", "macosx_11_0_arm64"), ("py3", "none", "macosx_11_0_arm64"))

    def test_linux_target_unchanged(self):
        self.assertEqual(self.tag("Linux", "x86_64", "linux_x86_64"), ("py3", "none", "linux_x86_64"))

    def test_unknown_macos_target_fails(self):
        with self.assertRaises(RuntimeError):
            self.tag("Darwin", "unknown", "macosx_11_0_universal2")


if __name__ == "__main__":
    unittest.main()
