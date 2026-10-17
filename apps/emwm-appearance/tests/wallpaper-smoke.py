# Private Xvfb/HOME only: GUI -> PNG stdin -> helper against a fake system tree.
import os
from pathlib import Path
import subprocess
import sys
import traceback
from unittest.mock import patch
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
import gi
gi.require_version('Gtk','3.0')
from gi.repository import Gtk, GLib
from PIL import Image
from core import Settings, ini_value
from login_helper import apply_background
from test_wallpaper import login_tree
import wallpaper_ui
settings=Settings()
selection=settings.directory/'selection.json'
selection_before=selection.read_bytes() if selection.exists() else None
root=settings.home/'fake-system'; config=login_tree(root)
before=config.read_bytes()
helper=settings.home/'helper.py'; helper.write_text('# --background')
source=settings.home/'test.png'; Image.new('RGB',(64,32),'#7399ba').save(source)
parent=Gtk.Window(); parent.busy=False; parent.show_all()
errors=[]; calls=[]; cancel=True

def walk(widget):
    yield widget
    if isinstance(widget,Gtk.Container):
        for child in widget.get_children(): yield from walk(child)

def command(args,**kwargs):
    assert args[:3]==['pkexec','/usr/bin/python3','-I'],args
    assert args[4:]==['--background','tile'],args
    calls.append(args)
    if cancel: return subprocess.CompletedProcess(args,126,b'',b'cancelled')
    apply_background(kwargs['input'],'tile',root)
    return subprocess.CompletedProcess(args,0,b'',b'')

def finish():
    if parent.busy: return True
    try:
        assert len(calls)==2,calls
        assert (settings.directory/'login-background.json').is_file()
        assert (selection.read_bytes() if selection.exists() else None)==selection_before, 'wallpaper must not apply session themes'
        assert config.read_bytes()!=before
        value=ini_value(config.read_text(),'greeter','background')
        assert (root/value.split(':',1)[1].lstrip('/')).is_file()
        print('PASS: wallpaper dialog, preview, cancelled authentication, PNG transfer and independent login apply',flush=True)
        capture=os.environ.get('APPEARANCE_WALLPAPER_CAPTURE')
        if capture:
            # Use the saved real subprocess implementation, not the pkexec stub.
            real_run(['import','-window','root',capture],check=True)
    except Exception: errors.append(traceback.format_exc())
    finally: Gtk.main_quit()
    return False

def after_cancel():
    global cancel
    if parent.busy: return True
    try:
        assert config.read_bytes()==before
        assert not (settings.directory/'login-background.json').exists()
        cancel=False
        dialog.response(Gtk.ResponseType.APPLY)
        GLib.timeout_add(100,finish)
    except Exception:
        errors.append(traceback.format_exc()); Gtk.main_quit()
    return False

def start():
    try:
        chooser=next(w for w in walk(dialog) if isinstance(w,Gtk.FileChooserButton))
        chooser.set_filename(str(source))
        combo=next(w for w in walk(dialog) if isinstance(w,Gtk.ComboBoxText))
        combo.set_active_id('tile')
        dialog.response(Gtk.ResponseType.APPLY)
        GLib.timeout_add(100,after_cancel)
    except Exception:
        errors.append(traceback.format_exc()); Gtk.main_quit()
    return False

real_run=subprocess.run
dialog=wallpaper_ui.show(parent,settings,helper)
GLib.timeout_add(250,start)
with patch('wallpaper_ui.subprocess.run',side_effect=command): Gtk.main()
if errors: raise AssertionError('\n'.join(errors))
