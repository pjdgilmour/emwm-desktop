#!/usr/bin/python3
# SPDX-License-Identifier: GPL-3.0-only
"""Real PCManFM drag/drop/cancel regression, with private HOME, X server and bus."""
import argparse
import ctypes
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]


def run(args, work):
    processes, logs = [], []
    def launch(command, env=None, **kwargs):
        log = (work / (Path(command[0]).name + '.log')).open('w')
        logs.append(log)
        process = subprocess.Popen(command, env=env, stdout=log, stderr=log, **kwargs)
        processes.append(process)
        return process
    try:
        with (work / 'display').open('w') as stream:
            launch(['Xvfb', '-displayfd', str(stream.fileno()), '-screen', '0', '1024x768x24',
                    '-nolisten', 'tcp', '-noreset'], pass_fds=(stream.fileno(),))
        for _ in range(100):
            if (work / 'display').read_text().strip():
                break
            time.sleep(.05)
        else:
            raise RuntimeError('Private X server did not start')
        home, runtime = work / 'home', work / 'runtime'
        home.mkdir(); runtime.mkdir(mode=0o700)
        env = dict(os.environ, HOME=str(home), DISPLAY=':' + (work / 'display').read_text().strip(),
                   XDG_CONFIG_HOME=str(home / '.config'), XDG_DATA_HOME=str(home / '.local/share'),
                   XDG_STATE_HOME=str(home / '.local/state'), XDG_RUNTIME_DIR=str(runtime),
                   GVFS_DISABLE_FUSE='1', NO_AT_BRIDGE='1', GTK_THEME='Adwaita', GSETTINGS_BACKEND='memory',
                   PATH=str(args.file_manager.resolve().parent) + os.pathsep + os.environ['PATH'])
        for key in ('SESSION_MANAGER', 'XAPPLRESDIR', 'XENVIRONMENT'):
            env.pop(key, None)
        (home / '.config').mkdir()
        desktop = home / 'Desktop'; desktop.mkdir()
        (desktop / 'Test.txt').write_text('drag fixture\n')
        (home / '.config/user-dirs.dirs').write_text('XDG_DESKTOP_DIR="$HOME/Desktop"\n')
        subprocess.run(['dbus-update-activation-environment', 'HOME', 'DISPLAY', 'XDG_CONFIG_HOME',
                        'XDG_DATA_HOME', 'XDG_STATE_HOME', 'XDG_RUNTIME_DIR', 'GVFS_DISABLE_FUSE'], env=env, check=True)
        launch(['/usr/libexec/gvfsd', '--no-fuse'], env)
        subprocess.run(['xrdb', '-merge'], input='Emwm*interactivePlacement: False\nEmwm*clientAutoPlace: False\n',
                       text=True, env=env, check=True)
        launch([str(args.wm.resolve())], env)
        time.sleep(.3)
        if args.backend != 'none':
            launch([str(args.compositor.resolve()), '--backend', args.backend], env)
        icons = launch([str(ROOT / 'apps/emwm-desktop-icons/emwm-desktop-icons')], env)
        time.sleep(2)
        assert icons.poll() is None, 'desktop did not start'
        x = ctypes.CDLL('libX11.so.6'); xt = ctypes.CDLL('libXtst.so.6')
        x.XOpenDisplay.argtypes = [ctypes.c_char_p]; x.XOpenDisplay.restype = ctypes.c_void_p
        x.XFlush.argtypes = [ctypes.c_void_p]; x.XCloseDisplay.argtypes = [ctypes.c_void_p]
        x.XKeysymToKeycode.argtypes = [ctypes.c_void_p, ctypes.c_ulong]; x.XKeysymToKeycode.restype = ctypes.c_uint
        xt.XTestFakeMotionEvent.argtypes = [ctypes.c_void_p, ctypes.c_int, ctypes.c_int, ctypes.c_int, ctypes.c_ulong]
        xt.XTestFakeButtonEvent.argtypes = [ctypes.c_void_p, ctypes.c_uint, ctypes.c_int, ctypes.c_ulong]
        xt.XTestFakeKeyEvent.argtypes = xt.XTestFakeButtonEvent.argtypes
        display = x.XOpenDisplay(env['DISPLAY'].encode()); assert display
        def move(a, b):
            xt.XTestFakeMotionEvent(display, -1, a, b, 0); x.XFlush(display)
        def button(down):
            xt.XTestFakeButtonEvent(display, 1, down, 0); x.XFlush(display)
        def visible(a, b):
            shot = work / 'screen.png'
            subprocess.run(['import', '-window', 'root', str(shot)], env=env, check=True, timeout=5)
            with Image.open(shot) as image:
                patch = image.convert('RGB').crop((a-18, b-18, a+18, b+26))
                return sum(min(color) > 190 for color in patch.getdata()) > 200
        def assert_visible(a, b, label):
            deadline = time.monotonic() + .7
            while time.monotonic() < deadline:
                if visible(a, b):
                    return
                time.sleep(.04)
            raise AssertionError(f'{args.backend}: icon missing after {label}, without any extra click')
        def drag(start, end):
            move(*start); time.sleep(.08); button(1); time.sleep(.08)
            for step in range(1, 26):
                move(round(start[0] + (end[0]-start[0])*step/25),
                     round(start[1] + (end[1]-start[1])*step/25))
                time.sleep(.012)
        try:
            assert_visible(60, 130, 'initial load')
            drag((60, 130), (410, 305)); button(0)
            assert_visible(410, 305, 'drop')
            drag((410, 305), (650, 450)); button(0)
            assert_visible(650, 450, 'second drop')
            drag((650, 450), (800, 550))
            escape = x.XKeysymToKeycode(display, 0xff1b)
            xt.XTestFakeKeyEvent(display, escape, 1, 0); xt.XTestFakeKeyEvent(display, escape, 0, 0); x.XFlush(display)
            # Inspect before mouse release: drag-end, not a later click, must repair it.
            assert_visible(650, 450, 'Escape cancellation')
            assert (desktop / 'Test.txt').read_text() == 'drag fixture\n'
        finally:
            button(0); x.XCloseDisplay(display)
        print(f'PASS: {args.backend} desktop drag/drop/repeated move/cancel; icon visible within 700 ms without clicking', flush=True)
    finally:
        for process in reversed(processes):
            if process.poll() is None:
                process.terminate()
                try: process.wait(timeout=4)
                except subprocess.TimeoutExpired: process.kill(); process.wait()
        for log in logs: log.close()
        if sys.exc_info()[0]:
            for log in work.glob('*.log'):
                print(log.name + ':\n' + log.read_text(errors='replace')[-1800:])


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--wm', required=True, type=Path)
    parser.add_argument('--compositor', required=True, type=Path)
    parser.add_argument('--file-manager', required=True, type=Path)
    parser.add_argument('--backend', choices=('none', 'xrender', 'glx'))
    parser.add_argument('--private-bus', action='store_true')
    args = parser.parse_args()
    if not args.private_bus:
        for backend in ([args.backend] if args.backend else ['none', 'xrender', 'glx']):
            subprocess.run(['dbus-run-session', '--', '/usr/bin/python3', __file__, '--private-bus',
                            *sys.argv[1:], '--backend', backend], check=True)
    else:
        with tempfile.TemporaryDirectory(prefix='emwm-drag-') as directory:
            run(args, Path(directory))


if __name__ == '__main__':
    main()
