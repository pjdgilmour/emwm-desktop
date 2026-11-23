#!/usr/bin/python3
"""Exercise file creation, tabs and cross-version IPC on a private X server."""
import ctypes as ct
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import tempfile
import time


class Keyboard:
    def __init__(self):
        self.x = ct.CDLL("libX11.so.6")
        self.xtest = ct.CDLL("libXtst.so.6")
        self.x.XOpenDisplay.argtypes = [ct.c_char_p]
        self.x.XOpenDisplay.restype = ct.c_void_p
        self.x.XSetInputFocus.argtypes = [ct.c_void_p, ct.c_ulong, ct.c_int, ct.c_ulong]
        self.x.XStringToKeysym.argtypes = [ct.c_char_p]
        self.x.XStringToKeysym.restype = ct.c_ulong
        self.x.XKeysymToKeycode.argtypes = [ct.c_void_p, ct.c_ulong]
        self.x.XKeysymToKeycode.restype = ct.c_uint
        self.xtest.XTestFakeKeyEvent.argtypes = [ct.c_void_p, ct.c_uint, ct.c_int, ct.c_ulong]
        self.x.XSync.argtypes = [ct.c_void_p, ct.c_int]
        self.display = self.x.XOpenDisplay(None)
        assert self.display

    def type_keys(self, window, *names):
        codes = [self.x.XKeysymToKeycode(self.display, self.x.XStringToKeysym(n.encode()))
                 for n in names]
        assert all(codes), names
        self.x.XSetInputFocus(self.display, window, 2, 0)
        for pressed, keys in [(True, codes), (False, reversed(codes))]:
            for key in keys:
                self.xtest.XTestFakeKeyEvent(self.display, key, pressed, 0)
        self.x.XSync(self.display, False)
        time.sleep(.035)


def windows():
    tree = subprocess.check_output(["xwininfo", "-root", "-tree"], text=True)
    return {int(w, 16): name for w, name in re.findall(
        r'(0x[\da-f]+) "([^"]*)": \("pcmanfm" "Pcmanfm"\)', tree, re.IGNORECASE)}


def until(probe, description):
    deadline = time.monotonic() + 6
    while time.monotonic() < deadline:
        value = probe()
        if value:
            return value
        time.sleep(.05)
    raise AssertionError(f"Timeout waiting for {description}, windows: {windows()}")


def exercise(binary, client):
    base = Path(os.environ["HOME"])
    folder = base / "documents"
    folder.mkdir()
    other = base / "Outra pasta"
    other.mkdir()
    (folder / "example.txt").write_text("contents")
    keyboard = Keyboard()
    with (base / "application.log").open("wb") as log:
        process = subprocess.Popen([binary, str(folder)], stdout=log, stderr=log)
        try:
            window = until(lambda: next((w for w,n in windows().items() if n == folder.name), None),
                           "file browser")
            # The browser survives closing its second tab.
            keyboard.type_keys(window, "Control_L", "t")
            keyboard.type_keys(window, "Control_L", "w")
            assert process.poll() is None and window in windows()
            keyboard.type_keys(window, "Control_L", "Shift_L", "n")
            dialog = until(lambda: next((w for w,n in windows().items() if w != window and
                                         ("Folder" in n or "Create" in n)), None),
                           "new folder dialog")
            keyboard.type_keys(dialog, "Control_L", "a")
            for key in "rustfolder":
                keyboard.type_keys(dialog, key)
            keyboard.type_keys(dialog, "Return")
            until(lambda: (folder / "rustfolder").is_dir(), "created directory")
            # Relative names and spaces cross the existing PCManFM socket protocol.
            subprocess.run([client, "--new-win", other.name], cwd=base, check=True, timeout=6)
            until(lambda: other.name in windows().values(), "second window via IPC")
            subprocess.run([client, "--show-pref", "1"], check=True, timeout=6)
            until(lambda: "Preferences" in windows().values(), "preferences")
            print("PASS: files, GTK folder creation, tabs, preferences and IPC")
        finally:
            process.terminate()
            process.wait(timeout=5)


def main():
    binary, client = [str(Path(p).resolve()) for p in sys.argv[1:3]]
    if len(sys.argv) == 4 and sys.argv[3] == "--private":
        exercise(binary, client)
    else:
        with tempfile.TemporaryDirectory(prefix="emwm-file-test-") as temp:
            base = Path(temp)
            runtime = base / "runtime"
            runtime.mkdir(mode=0o700)
            env = dict(os.environ, HOME=temp, LC_ALL="C.UTF-8",
                       XDG_CONFIG_HOME=str(base / "config"), XDG_RUNTIME_DIR=str(runtime),
                       XDG_DATA_HOME=str(base / "data"), XDG_CACHE_HOME=str(base / "cache"),
                       XDG_STATE_HOME=str(base / "state"), GSETTINGS_BACKEND="memory",
                       GVFS_DISABLE_FUSE="1", GIO_USE_VFS="local", NO_AT_BRIDGE="1")
            env.pop("SESSION_MANAGER", None)
            with subprocess.Popen(["xvfb-run", "-a", "dbus-run-session", "--", sys.executable,
                                   __file__, binary, client, "--private"], env=env,
                                  start_new_session=True) as child:
                try:
                    if child.wait(timeout=40):
                        raise SystemExit(1)
                finally:
                    try:
                        os.killpg(child.pid, signal.SIGTERM)
                    except ProcessLookupError:
                        pass


if __name__ == "__main__":
    main()
