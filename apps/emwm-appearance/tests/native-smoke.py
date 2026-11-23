#!/usr/bin/python3
"""Drive the installed Rust executable through GTK accessibility on a private X display."""
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time

import gi
gi.require_version('Atspi', '2.0')
from gi.repository import Atspi, GLib


def widgets(node, depth=0):
    yield node
    if depth < 9:
        for i in range(node.get_child_count()):
            child = node.get_child_at_index(i)
            if child:
                yield from widgets(child, depth + 1)


def until(probe):
    limit = time.monotonic() + 8
    while time.monotonic() < limit:
        while GLib.MainContext.default().iteration(False):
            pass
        value = probe()
        if value:
            return value
        time.sleep(.05)
    raise AssertionError(f'Timeout waiting for {probe.__name__}')


def click(node):
    for i in range(node.get_n_actions()):
        if node.get_action_name(i) in ('click', 'activate', 'press'):
            assert node.do_action(i)
            return
    raise AssertionError(f'No click action on {node.get_name()}')


def main():
    binary = Path(sys.argv[1]).resolve()
    with tempfile.TemporaryDirectory() as temp:
        home = Path(temp)
        env = dict(os.environ, HOME=temp, GSETTINGS_BACKEND='memory', NO_AT_BRIDGE='0')
        for name in ('XDG_CONFIG_HOME', 'XDG_DATA_HOME', 'XDG_STATE_HOME', 'GTK_THEME'):
            env.pop(name, None)
        selection = home/'.config/emwm-desktop/appearance/selection.json'
        state = home/'.local/state/emwm-desktop'
        process = subprocess.Popen([binary, '--decoration'], env=env)
        try:
            Atspi.init()
            def find_window():
                assert process.poll() is None, process.returncode
                for app in widgets(Atspi.get_desktop(0)):
                    if app and app.get_name() == 'Aparência do EMWM':
                        return app
                return None
            window = until(find_window)
            names = {w.get_name() for w in widgets(window)}
            assert {'Temas e cores','Área de trabalho','Efeitos e renderização','Decoração'} <= names, names
            def gradient_combo():
                for widget in widgets(window):
                    if widget.get_role() != Atspi.Role.COMBO_BOX:
                        continue
                    names = [w.get_name() or '' for w in widgets(widget)]
                    if any('Sem gradiente' in name for name in names):
                        return widget
                return None
            control = until(gradient_combo)
            # GtkComboBox exposes GtkComboBox.set_active via ATK selection.
            assert control.select_child(3), control.get_name()
            assert not selection.exists(), 'a prévia gravou as preferências'
            button = next(w for w in widgets(window) if w.get_name() == 'Aplicar' and w.get_role() == Atspi.Role.PUSH_BUTTON)
            click(button)
            until(selection.is_file)
            saved = json.loads(selection.read_text())
            assert saved['gradient']['mode'] == 'diagonal', saved
            resources = (selection.parent/'appearance.resources').read_text()
            assert 'Emwm*titleGradient: diagonal\n' in resources, resources
            def live():
                return 'Emwm*titleGradient:\tdiagonal' in subprocess.check_output(['xrdb','-query'], env=env, text=True)
            until(live)
            reset = next(w for w in widgets(window) if w.get_name() == 'Recarregar')
            click(reset)
            close = next(w for w in widgets(window) if w.get_name() == 'Fechar')
            click(close)
            assert process.wait(5) == 0
            print('PASS: native GTK tabs, gradient preview, Apply, live X resources, reload and close')
        finally:
            if process.poll() is None:
                process.terminate()
                process.wait()
            for pid in state.glob('xsettingsd-*.pid'):
                try:
                    os.kill(int(pid.read_text()), signal.SIGTERM)
                except (ProcessLookupError, ValueError):
                    pass


if __name__ == '__main__':
    main()
