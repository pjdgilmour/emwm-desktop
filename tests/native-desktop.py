#!/usr/bin/python3
# SPDX-License-Identifier: GPL-3.0-only
"""Exercise the real WM, compositor and icon desktop on private X servers."""
import argparse
import os
from pathlib import Path
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]


def stop(process):
    if process.poll() is None:
        process.terminate()
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            process.kill()
            process.wait()


def test(args, backend, work, probe):
    processes, streams = [], []

    def launch(command, env=None, **kwargs):
        log = (work / (Path(command[0]).name + '.log')).open('a')
        streams.append(log)
        process = subprocess.Popen(command, stdout=log, stderr=subprocess.STDOUT, env=env, **kwargs)
        processes.append(process)
        return process

    try:
        display_file = (work / 'display').open('w')
        streams.append(display_file)
        server = launch(['Xvfb', '-displayfd', str(display_file.fileno()), '-screen', '0', '1024x768x24',
                         '-nolisten', 'tcp', '-noreset'], pass_fds=(display_file.fileno(),))
        for _ in range(100):
            if (work / 'display').read_text().strip():
                break
            if server.poll() is not None:
                raise RuntimeError('Xvfb failed; check socket permissions')
            time.sleep(.05)
        else:
            raise RuntimeError('Xvfb startup timed out')
        home = work / 'home'
        (home / '.config/emwm-desktop').mkdir(parents=True)
        runtime = work / 'runtime'
        runtime.mkdir(mode=0o700)
        env = dict(os.environ, HOME=str(home), DISPLAY=':' + (work / 'display').read_text().strip(),
                   XDG_CONFIG_HOME=str(home / '.config'), XDG_STATE_HOME=str(home / '.local/state'),
                   XDG_DATA_HOME=str(home / '.local/share'), XENVIRONMENT=str(work / 'resources'),
                   XDG_RUNTIME_DIR=str(runtime),
                   GSETTINGS_BACKEND='memory', NO_AT_BRIDGE='1', GVFS_DISABLE_FUSE='1')
        env.pop('SESSION_MANAGER', None)
        subprocess.run(['dbus-update-activation-environment', 'HOME', 'DISPLAY', 'XDG_CONFIG_HOME',
                        'XDG_STATE_HOME', 'XDG_DATA_HOME', 'XDG_RUNTIME_DIR', 'GVFS_DISABLE_FUSE'], env=env, check=True)
        launch(['/usr/libexec/gvfsd', '--no-fuse'], env)
        resources = ('Emwm*workspaceCount: 4\nEmwm*interactivePlacement: False\nEmwm*clientAutoPlace: False\n'
                     'Emwm*focusAutoRaise: False\nEmwm*useIconBox: False\n')
        (work / 'resources').write_text(resources)
        subprocess.run(['xrdb', '-merge'], input=resources, text=True, env=env, check=True)
        wm = launch([str(args.wm), '-xrm', f'Emwm*configFile: {args.wm.parent}/system.emwmrc'], env)
        time.sleep(.4)
        config = home / '.config/emwm-desktop/compositor.conf'
        config.write_text(f'[compositor]\nbackend={backend}\neffect=zoom\nduration_ms=220\n')
        compositor = launch([str(args.compositor)], env)
        time.sleep(.4)
        assert wm.poll() is None and compositor.poll() is None
        second = subprocess.run([str(args.compositor)], env=env, capture_output=True, text=True, timeout=5)
        assert second.returncode != 0 and 'Another compositor' in second.stderr
        subprocess.run([str(probe)], env=env, check=True, timeout=20)
        # Test live configuration, backend teardown and reconstruction, and effects.
        for effect in ('twist', 'flip'):
            config.write_text(f'[compositor]\nbackend={backend}\neffect={effect}\nduration_ms=220\n')
            compositor.send_signal(__import__('signal').SIGHUP)
            time.sleep(.1)
            subprocess.run([str(probe)], env=env, check=True, timeout=20)
        config.write_text(f'[compositor]\nbackend={"xrender" if backend == "glx" else "glx"}\neffect=zoom\n')
        compositor.send_signal(__import__('signal').SIGHUP)
        time.sleep(.3)
        assert compositor.poll() is None
        subprocess.run([str(probe)], env=env, check=True, timeout=20)
        # Real desktop, with a private XDG folder and trash backend / private bus.
        folder = home / 'Área de trabalho'
        folder.mkdir()
        (folder / 'Arquivo de teste.txt').write_text('desktop integration\n')
        (home / '.config/user-dirs.dirs').write_text(f'XDG_DESKTOP_DIR="{folder}"\n')
        icons = launch([str(ROOT / 'apps/emwm-desktop-icons/emwm-desktop-icons')], env)
        time.sleep(2)
        tree = subprocess.check_output(['xwininfo', '-root', '-tree'], env=env, text=True)
        assert 'pcmanfm' in tree.lower(), tree
        assert icons.poll() is None, 'desktop controller exited'
        from PIL import Image
        for name, color in [('zero', '#aa8855'), ('one', '#335577')]:
            Image.new('RGB', (1024, 768), color).save(home / f'{name}.png')
        subprocess.run(['xrdb', '-merge'], env=env, input=(
            f'Emwm*ws0*backdrop*image: "{home}/zero.png"\n'
            f'Emwm*ws1*backdrop*image: "{home}/one.png"\n'), text=True, check=True)
        for workspace, expected in [(0, 'aa8855'), (1, '335577'), (0, 'aa8855')]:
            subprocess.run([str(probe), '--workspace', str(workspace)], env=env, check=True)
            for _ in range(40):
                color = subprocess.check_output([str(probe), '--pixel', '900', '650'], env=env, text=True).strip()
                if color == expected:
                    break
                time.sleep(.1)
            else:
                raise AssertionError(f'workspace {workspace} wallpaper: {color}, expected {expected}')
        if args.capture:
            args.capture.mkdir(parents=True, exist_ok=True)
            subprocess.run(['import', '-window', 'root', str(args.capture / f'desktop-{backend}.png')], env=env, check=True, timeout=10)
        subprocess.run(['gio', 'trash', str(folder / 'Arquivo de teste.txt')], env=env, check=True, timeout=10)
        for _ in range(30):
            trash = subprocess.check_output(['gio', 'list', '-d', 'trash:///'], env=env, text=True, timeout=10)
            if 'Arquivo de teste' in trash:
                break
            time.sleep(.1)
        else:
            local_trash = list((home / '.local/share/Trash').rglob('*'))
            raise AssertionError(f'trash contents: {trash!r}, local trash: {local_trash!r}')
        assert not (folder / 'Arquivo de teste.txt').exists()
        uri = subprocess.check_output(['gio', 'list', '-u', 'trash:///'], env=env, text=True, timeout=10).strip()
        assert uri.startswith('trash:///') and '\n' not in uri
        subprocess.run(['gio', 'move', uri, str(folder / 'Arquivo restaurado.txt')], env=env, check=True, timeout=10)
        assert (folder / 'Arquivo restaurado.txt').read_text() == 'desktop integration\n'
        stop(icons)
        assert compositor.poll() is None and wm.poll() is None
        stop(compositor)
        # Clean compositor shutdown must release its selection and redirection.
        compositor2 = launch([str(args.compositor)], env)
        time.sleep(.3)
        assert compositor2.poll() is None
        stop(compositor2)
        if backend == 'glx':
            shim = work / 'glx-no-textures.so'
            subprocess.run(['cc', '-shared', '-fPIC', str(ROOT / 'tests/glx-no-textures.c'),
                            '-o', str(shim), '-ldl'], check=True)
            fallback = launch([str(args.compositor), '--backend', 'glx'],
                              dict(env, LD_PRELOAD=str(shim)))
            time.sleep(.4)
            assert fallback.poll() is None, 'initial shadow failure must fall back, not exit'
            subprocess.run([str(probe)], env=env, check=True, timeout=20)
            assert 'requested backend failed; restoring XRender' in (work / 'emwm-compositor.log').read_text()
            print('PASS: initial GLX texture failure recovers with working XRender shadows/effects', flush=True)
        print(f'PASS: {backend}, live effects/backend changes, singleton, real desktop/trash and restart', flush=True)
    finally:
        for process in reversed(processes):
            stop(process)
        for stream in streams:
            stream.close()
        for path in work.glob('*.log'):
            text = path.read_text(errors='replace')
            if text:
                print(f'{path.name}:\n{text[-4000:]}', flush=True)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--wm', required=True, type=Path)
    parser.add_argument('--compositor', required=True, type=Path)
    parser.add_argument('--capture', type=Path)
    parser.add_argument('--private-bus', action='store_true')
    parser.add_argument('--backend', choices=('xrender', 'glx'))
    args = parser.parse_args()
    if not args.private_bus:
        for backend in ([args.backend] if args.backend else ['xrender', 'glx']):
            subprocess.run(['dbus-run-session', '--', '/usr/bin/python3', __file__,
                            '--private-bus', *__import__('sys').argv[1:], '--backend', backend], check=True)
        return
    args.wm = args.wm.resolve()
    args.compositor = args.compositor.resolve()
    with tempfile.TemporaryDirectory(prefix='emwm-native-') as directory:
        base = Path(directory)
        probe = base / 'probe'
        subprocess.run(['cc', '-Wall', '-Wextra', str(ROOT / 'tests/native-desktop.c'), '-lX11', '-lXext', '-o', str(probe)], check=True)
        for backend in (args.backend,):
            work = base / backend
            work.mkdir()
            test(args, backend, work, probe)


if __name__ == '__main__':
    main()
