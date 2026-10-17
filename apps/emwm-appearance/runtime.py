# SPDX-License-Identifier: GPL-3.0-only
"""Optional live updates through xsettingsd, GSettings and Xresources."""
import ctypes
import fcntl
import os
from pathlib import Path
import shutil
import signal
import subprocess
import time
from core import read


def selection_owned():
    """Never replace an XSETTINGS owner belonging to XFCE/GNOME/etc."""
    lib=ctypes.CDLL('libX11.so.6')
    lib.XOpenDisplay.argtypes=[ctypes.c_char_p]; lib.XOpenDisplay.restype=ctypes.c_void_p
    lib.XDefaultScreen.argtypes=[ctypes.c_void_p]; lib.XDefaultScreen.restype=ctypes.c_int
    lib.XInternAtom.argtypes=[ctypes.c_void_p,ctypes.c_char_p,ctypes.c_int]; lib.XInternAtom.restype=ctypes.c_ulong
    lib.XGetSelectionOwner.argtypes=[ctypes.c_void_p,ctypes.c_ulong]; lib.XGetSelectionOwner.restype=ctypes.c_ulong
    lib.XCloseDisplay.argtypes=[ctypes.c_void_p]
    display=lib.XOpenDisplay(None)
    if not display: return True
    try:
        atom=lib.XInternAtom(display,f'_XSETTINGS_S{lib.XDefaultScreen(display)}'.encode(),False)
        return bool(lib.XGetSelectionOwner(display,atom))
    finally: lib.XCloseDisplay(display)


def xsettings(settings):
    if not os.environ.get('DISPLAY'): return ['Atualização ao vivo requer uma sessão X11.']
    executable=shutil.which('xsettingsd')
    if not executable: return ['Instale xsettingsd para atualizar aplicativos GTK abertos. As preferências foram salvas.']
    config=settings.directory/'xsettingsd.conf'
    if not config.is_file(): return []
    directory=settings.state/'emwm-desktop'
    directory.mkdir(parents=True,exist_ok=True)
    import hashlib
    display=hashlib.sha256(os.environ['DISPLAY'].encode()).hexdigest()[:16]
    pidfile=directory/f'xsettingsd-{display}.pid'
    lock=(directory/f'xsettingsd-{display}.lock').open('a')
    with lock:
        fcntl.flock(lock,fcntl.LOCK_EX)
        try:
            pid=int(read(pidfile))
            proc=Path('/proc')/str(pid)
            args=(proc/'cmdline').read_bytes().split(b'\0')
            if proc.stat().st_uid==os.getuid() and os.fsencode(str(config)) in args and Path(os.fsdecode(args[0])).name=='xsettingsd':
                os.kill(pid,signal.SIGHUP)
                return []
        except (OSError,ValueError,IndexError): pass
        if selection_owned():
            return ['Outro serviço já controla a aparência nesta sessão. Ele foi preservado; reentre no EMWM para usar estas preferências.']
        with (directory/'appearance-xsettingsd.log').open('ab') as log:
            proc=subprocess.Popen([executable,'--config',str(config)],stdin=subprocess.DEVNULL,stdout=log,stderr=log,start_new_session=True)
        time.sleep(.12)
        if proc.poll() is not None: return ['xsettingsd não iniciou. Consulte appearance-xsettingsd.log.']
        pidfile.write_text(str(proc.pid))
    return []


def apply_live(settings,values):
    notes=xsettings(settings)
    resource=settings.directory/'appearance.resources'
    if resource.is_file() and shutil.which('xrdb'):
        result=subprocess.run(['xrdb','-merge',str(resource)],capture_output=True,text=True)
        if result.returncode: notes.append('Não foi possível atualizar os recursos X: '+result.stderr.strip())
    try:
        import gi
        gi.require_version('Gtk','3.0')
        from gi.repository import Gio,Gdk
        source=Gio.SettingsSchemaSource.get_default()
        schema=source.lookup('org.gnome.desktop.interface',True) if source else None
        if schema:
            preferences=Gio.Settings.new_full(schema,None,None)
            for key,value in [('gtk-theme',values['theme']),('icon-theme',values['icons']),('cursor-theme',values['cursor'])]:
                if schema.has_key(key) and preferences.is_writable(key): preferences.set_string(key,value)
            if schema.has_key('cursor-size') and preferences.is_writable('cursor-size'): preferences.set_int('cursor-size',values['size'])
            Gio.Settings.sync()
        display=Gdk.Display.get_default()
        if display:
            root=Gdk.get_default_root_window()
            if root: root.set_cursor(Gdk.Cursor.new_from_name(display,'default'))
            display.flush()
    except (ImportError,ValueError,RuntimeError) as error:
        notes.append('Atualização complementar indisponível: '+str(error))
    return notes
