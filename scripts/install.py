#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Compatibility entry point for existing scripts invoking the installer with Python."""
import os
from pathlib import Path
import sys

if __name__ == '__main__':
    launcher = Path(__file__).resolve().parents[1] / 'install.sh'
    os.execv(launcher, [str(launcher), *sys.argv[1:]])
