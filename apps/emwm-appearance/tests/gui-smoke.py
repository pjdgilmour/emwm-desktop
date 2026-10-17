# Real GTK UI/XSETTINGS test. Invoked only with a private DISPLAY and HOME.
import os
from pathlib import Path
import signal
import subprocess
import sys
import traceback
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
import gi
gi.require_version('Gtk','3.0')
from gi.repository import Gtk,GLib
from core import Settings
import gui
settings=Settings()
errors=[]
theme=os.environ.get("APPEARANCE_TEST_THEME","Adwaita")

def walk(widget):
    yield widget
    if isinstance(widget,Gtk.Container):
        for child in widget.get_children(): yield from walk(child)

def check():
    try:
        saved=settings.current()
        assert saved['theme']==theme,saved
        assert saved['size']==32,saved
        assert 'backdrops.resources' in (settings.home/'.Xresources').read_text()
        probe='''import gi; gi.require_version("Gtk","3.0"); from gi.repository import Gtk; Gtk.init([]); s=Gtk.Settings.get_default(); print(s.get_property("gtk-theme-name"),s.get_property("gtk-icon-theme-name"),s.get_property("gtk-cursor-theme-name"),s.get_property("gtk-cursor-theme-size"))'''
        env=dict(os.environ); env.pop('GTK_THEME',None)
        output=subprocess.check_output(['/usr/bin/python3','-c',probe],env=env,text=True).strip()
        assert output==f'{theme} Adwaita Adwaita 32',output
        # A second application and a daemon reload see the new setting too.
        from runtime import apply_live
        saved['size']=40; settings.apply(saved); assert not apply_live(settings,saved)
        output=subprocess.check_output(['/usr/bin/python3','-c',probe],env=env,text=True).strip()
        assert output==f'{theme} Adwaita Adwaita 40',output
        print('PASS: GUI preview, Apply, configuration preservation, real GTK XSETTINGS and daemon reload',flush=True)
        capture=os.environ.get('APPEARANCE_CAPTURE')
        if capture: subprocess.run(['import','-window','root',capture],check=True)
    except Exception:
        errors.append(traceback.format_exc())
    finally:
        Gtk.main_quit()
    return False

def apply():
    try:
        win=next(w for w in Gtk.Window.list_toplevels() if w.get_title()=='Aparência do EMWM')
        assert not (settings.directory/'selection.json').exists()
        # Discover a theme installed after the window was opened, then remove it.
        added=settings.home/'.icons/New Icons'
        added.mkdir(parents=True)
        (added/'index.theme').write_text('[Icon Theme]\nName=New Icons\nDirectories=16x16/apps\n')
        assert 'New Icons' not in win.catalogs['icons']
        win.reset.clicked()
        assert 'New Icons' in win.catalogs['icons']
        (added/'index.theme').unlink()
        win.reset.clicked()
        assert 'New Icons' not in win.catalogs['icons']
        win.combos['theme'].set_active_id(theme)
        win.combos['icons'].set_active_id('Adwaita')
        win.combos['cursor'].set_active_id('Adwaita')
        win.size.set_value(32)
        assert not (settings.directory/'selection.json').exists(), 'preview must not save'
        win.apply.clicked()
        GLib.timeout_add(600,check)
    except Exception:
        errors.append(traceback.format_exc()); Gtk.main_quit()
    return False

GLib.timeout_add(700,apply)
sys.argv=['gui.py']
try: gui.main()
finally:
    for path in (settings.state/'emwm-desktop').glob('xsettingsd-*.pid'):
        try: os.kill(int(path.read_text()),signal.SIGTERM)
        except (ValueError,ProcessLookupError): pass
if errors: raise AssertionError('\n'.join(errors))
