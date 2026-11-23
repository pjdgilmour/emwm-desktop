#!/usr/bin/python3
# SPDX-License-Identifier: GPL-2.0-or-later
"""Adapt the private X11 regression tests to rust-anything app record/replay."""
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

root = Path(__file__).resolve().parents[3]
if len(sys.argv) != 3 or sys.argv[2] not in ('monitors', 'drag'):
    sys.exit('Usage: replay-desktop.py /path/to/pcmanfm monitors|drag')
file_manager = Path(sys.argv[1]).resolve(strict=True)
bin_dir = Path(os.environ.get('EMWM_TEST_BIN_DIR', '/usr/lib/emwm-desktop/bin'))

with tempfile.TemporaryDirectory(prefix='pcmanfm-replay-') as tmp:
    # emwm-desktop-icons prefers its own binary directory over PATH.
    # Give it an isolated copy so the test always exercises the chosen file manager.
    icons = Path(tmp) / 'emwm-desktop-icons'
    shutil.copy2(bin_dir / icons.name, icons)
    (icons.parent / 'pcmanfm').symlink_to(file_manager)
    command = [sys.executable, str(root / 'tests' / ('desktop-' + sys.argv[2] + '.py')),
               '--wm', str(bin_dir / 'emwm'), '--file-manager', str(file_manager)]
    if sys.argv[2] == 'drag':
        command += ['--compositor', str(bin_dir / 'emwm-compositor'), '--backend', 'none']
    else:
        command += ['--tellmwm', str(bin_dir / 'tellmwm')]
    result = subprocess.run(command, capture_output=True, text=True,
                            env=dict(os.environ, EMWM_TEST_ICONS_BINARY=str(icons)))
    # D-Bus activation diagnostics include transient PIDs and bus addresses.
    if result.returncode:
        sys.stderr.write(result.stderr)
    sys.stdout.write(result.stdout)
    sys.exit(result.returncode)
