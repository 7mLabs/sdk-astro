from setuptools import Distribution, setup
from setuptools.command.bdist_wheel import bdist_wheel
import platform
import re

# Mark the wheel platform-specific even though the wrapper itself is pure Python.
class BinaryDistribution(Distribution):
    def has_ext_modules(self):
        return True

class PlatformWheel(bdist_wheel):
    def finalize_options(self):
        super().finalize_options()
        self.root_is_pure = False

    def get_tag(self):
        _, _, platform_tag = super().get_tag()
        if platform.system() == "Darwin":
            # setup-python can run a universal2 interpreter on an ARM64 runner.
            # The wheel embeds one host-architecture Rust library, not a fat one.
            architecture = platform.machine()
            match = re.fullmatch(r"(macosx_\d+_\d+)_(?:universal2|arm64|x86_64)", platform_tag)
            if match is None or architecture not in ("arm64", "x86_64"):
                raise RuntimeError("Cannot identify the macOS native wheel target")
            platform_tag = f"{match.group(1)}_{architecture}"
        return "py3", "none", platform_tag

setup(distclass=BinaryDistribution, cmdclass={"bdist_wheel": PlatformWheel})
