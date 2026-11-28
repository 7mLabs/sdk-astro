"""Current export validation; requires the Word render documented in check_latex_exports.py."""
from pathlib import Path
import runpy
runpy.run_path(str(Path(__file__).with_name('check_latex_exports.py')), run_name='__main__')
