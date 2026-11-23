# SPDX-License-Identifier: GPL-3.0-only
import io
import json
import os
import shutil
from pathlib import Path
import subprocess
import tarfile
import tempfile
import time
import unittest

ROOT = Path(__file__).resolve().parent.parent


class SessionTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        if 'EMWM_TEST_SESSION_BINARY' in os.environ:
            cls.session = Path(os.environ['EMWM_TEST_SESSION_BINARY'])
        else:
            cls.build = tempfile.TemporaryDirectory()
            cls.addClassCleanup(cls.build.cleanup)
            subprocess.run(['cargo', 'build', '--locked', '--manifest-path',
                            ROOT / 'rust/desktop/Cargo.toml', '--target-dir', cls.build.name,
                            '--bin', 'emwm-session', '--bin', 'xmsession'], check=True)
            cls.session = Path(cls.build.name) / 'debug/emwm-session'

    def test_existing_profile_resource_order_and_component_switches(self):
        self.check_session('1')

    def test_disabled_tray_is_exported_to_toolchest(self):
        self.check_session('0')

    def check_session(self, tray):
        with tempfile.TemporaryDirectory() as temp:
            base = Path(temp)
            home = base / 'home'
            home.mkdir()
            prefix = base / 'private'
            bins = prefix / 'bin'
            bins.mkdir(parents=True)
            (bins / 'emwm-session').symlink_to(self.session)
            data = base / 'data'
            data.mkdir()
            config = home / '.config/emwm-desktop'
            config.mkdir(parents=True)
            record = base / 'record'
            (home / '.Xprofile').write_text('export USER_PROFILE_READ=yes\n')
            (home / '.Xdefaults').write_text('XTerm*faceSize: 12\n')
            (home / '.Xresources').write_text('Emwm*workspaceCount: 4\n')
            (config / 'session.conf').write_text(
                'EMWM_DESKTOP_POLKIT=0\nEMWM_DESKTOP_NETWORK=0\n'
                f'EMWM_DESKTOP_TRAY={tray}\n')
            before = {p: p.read_bytes() for p in home.rglob('*') if p.is_file()}
            emitter = bins / 'emitter'
            emitter.write_text('''#!/usr/bin/python3
import json,os,sys,time,shutil
name=os.path.basename(sys.argv[0])
if name=='pgrep': sys.exit(1)
record={'name':name,'args':sys.argv[1:],'profile':os.environ.get('USER_PROFILE_READ'),
        'desktop':os.environ.get('XDG_CURRENT_DESKTOP'),'path':os.environ['PATH'],
        'tray':os.environ.get('EMWM_DESKTOP_TRAY'),'appearance':shutil.which('emwm-appearance')}
fd=os.open(os.environ['TEST_RECORD'],os.O_WRONLY|os.O_CREAT|os.O_APPEND,0o600)
os.write(fd,(json.dumps(record)+'\\n').encode());os.close(fd)
if name=='xmsm': time.sleep(.4)
''')
            emitter.chmod(0o755)
            for name in ['xrdb', 'dbus-update-activation-environment', 'pgrep', 'xmsm',
                         'xmtray', 'emwm-compositor', 'emwm-desktop-icons', 'emwm-appearance', 'gnome-keyring-daemon', 'nm-applet']:
                (bins / name).symlink_to('emitter')
            script = base / 'session'
            script.write_text((ROOT / 'session/emwm-desktop-session').read_text()
                              .replace('/usr/lib/emwm-desktop', str(prefix))
                              .replace('/usr/share/emwm-desktop', str(data)))
            env = dict(os.environ, HOME=str(home), PATH='/usr/bin:/bin',
                       DBUS_SESSION_BUS_ADDRESS='unix:path=/nonexistent-test-bus',
                       DISPLAY=':test', TEST_RECORD=str(record))
            for name in ['XDG_CONFIG_HOME','XDG_STATE_HOME','XFILESEARCHPATH']:
                env.pop(name, None)
            subprocess.run(['sh', str(script)], env=env, check=True, timeout=10)
            events = []
            deadline = time.monotonic()+3
            while time.monotonic() < deadline:
                events = [json.loads(line) for line in record.read_text().splitlines()]
                if len(events) >= 7:
                    break
                time.sleep(.02)
            names = [e['name'] for e in events]
            self.assertIn('xmsm', names)
            self.assertNotIn('xmtray', names)
            self.assertEqual(next(e for e in events if e['name']=='xmsm')['tray'], tray)
            self.assertIn('gnome-keyring-daemon', names)
            self.assertNotIn('nm-applet', names)
            self.assertEqual([e['args'][1] for e in events if e['name']=='xrdb'],
                             [str(data/'defaults.Xresources'), str(home/'.Xdefaults'), str(home/'.Xresources')])
            self.assertIn('emwm-compositor', names)
            self.assertIn('emwm-desktop-icons', names)
            self.assertEqual(next(e for e in events if e['name']=='xmsm')['appearance'],str(bins/'emwm-appearance'))
            self.assertNotIn('picom', names)
            self.assertTrue(all(e['profile']=='yes' and e['desktop']=='EMWM' for e in events))
            self.assertTrue(all(e['path'].startswith(str(bins)+':') for e in events))
            for path, contents in before.items():
                self.assertEqual(path.read_bytes(), contents)

    def test_xmsession_rotates_log_and_respects_profile(self):
        with tempfile.TemporaryDirectory() as temp:
            base = Path(temp)
            launcher = base / 'xmsession'
            shutil.copy2(self.session.with_name('xmsession'), launcher)
            for name, command in [('xmsm', 'echo "manager ${FROM_PROFILE:-}"; exit 23'),
                                  ('xrdb', 'echo "xrdb $*"')]:
                path = base / name
                path.write_text('#!/bin/sh\n' + command + '\n')
                path.chmod(0o755)
            for name in ['.Xdefaults', '.Xresources']:
                (base / name).touch()
            log = base / '.xmsession.log'
            log.write_text('original log')
            env = dict(os.environ, HOME=temp, PATH=str(base) + ':/usr/bin')
            plain = subprocess.run([launcher], env=env, timeout=5)
            self.assertEqual(plain.returncode, 23)
            first = log.read_text()
            self.assertEqual((base / '.xmsession.log.old').read_text(), 'original log')
            self.assertLess(first.index('xrdb -merge ' + str(base / '.Xdefaults')),
                            first.index('xrdb -merge ' + str(base / '.Xresources')))
            (base / '.Xprofile').write_text('export FROM_PROFILE="$1"\nXDEFAULTS=/nonexistent-defaults\n')
            profiled = subprocess.run([launcher, 'example'], env=env, timeout=5)
            self.assertEqual(profiled.returncode, 23)
            self.assertEqual((base / '.xmsession.log.old').read_text(), first)
            self.assertIn('manager example', log.read_text())
            self.assertNotIn('.Xdefaults', log.read_text())

    def test_requires_x11_without_modifying_home(self):
        with tempfile.TemporaryDirectory() as temp:
            env=dict(os.environ,HOME=temp)
            env.pop('DISPLAY',None)
            proc=subprocess.run([self.session],
                                env=env,capture_output=True,text=True)
            self.assertNotEqual(proc.returncode,0)
            self.assertIn('X11 display',proc.stderr)
            self.assertEqual(list(Path(temp).iterdir()),[])

    def test_termination_reaps_owned_children(self):
        with tempfile.TemporaryDirectory() as temp:
            base = Path(temp)
            bins = base / 'bin'
            bins.mkdir()
            config = base / 'config/emwm-desktop'
            config.mkdir(parents=True)
            (config / 'session.conf').write_text(
                'EMWM_DESKTOP_NETWORK=0\nEMWM_DESKTOP_KEYRING=0\nEMWM_DESKTOP_POLKIT=0\n')
            for name in ['xmsm', 'emwm-compositor', 'emwm-desktop-icons']:
                script = bins / name
                script.write_text(f'#!/bin/sh\necho "$$" > "$HOME/{name}.pid"\nexec sleep 10\n')
                script.chmod(0o755)
            for name in ['xrdb', 'dbus-update-activation-environment']:
                (bins / name).symlink_to('/bin/true')
            env = dict(os.environ, HOME=temp, DISPLAY=':fixture', XDG_STATE_HOME=str(base/'state'),
                       XDG_CONFIG_HOME=str(base/'config'), EMWM_DESKTOP_PREFIX=temp,
                       DBUS_SESSION_BUS_ADDRESS='unix:path=/nonexistent-test-bus')
            with subprocess.Popen([self.session], env=env) as session:
                try:
                    deadline = time.monotonic() + 5
                    while len(list(base.glob('*.pid'))) < 3 and time.monotonic() < deadline:
                        time.sleep(.03)
                    session.terminate()
                    self.assertEqual(session.wait(timeout=6), 0)
                    pids = [int(path.read_text()) for path in base.glob('*.pid')]
                    self.assertEqual(len(pids), 3)
                    self.assertTrue(all(not Path(f'/proc/{pid}').exists() for pid in pids))
                finally:
                    if session.poll() is None:
                        session.kill()


class PackageTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        packages=([Path(os.environ['EMWM_TEST_PACKAGE'])] if 'EMWM_TEST_PACKAGE' in os.environ
                  else list((ROOT/'dist').glob('*.deb')))
        if not packages:
            raise unittest.SkipTest('Gere um pacote com --build-only para verificar seu conteúdo.')
        cls.package=max(packages,key=lambda p:p.stat().st_mtime)
        data=subprocess.check_output(['dpkg-deb','--fsys-tarfile',str(cls.package)])
        cls.archive=tarfile.open(fileobj=io.BytesIO(data))
        cls.files={p.name.lstrip('./'):p for p in cls.archive.getmembers() if p.isfile()}

    def read(self,path):
        return self.archive.extractfile(self.files[path]).read()

    def test_private_layout_and_permissions(self):
        required=['usr/bin/emwm-appearance', 'usr/lib/emwm-desktop/bin/emwm-session',
                  'usr/lib/emwm-desktop/appearance/emwm-appearance',
                  'usr/lib/emwm-desktop/appearance/emwm-appearance-login',
                  'usr/bin/emwm-desktop-session','usr/share/xsessions/emwm-desktop.desktop',
                  'usr/share/emwm-desktop/app-defaults/Emwm','usr/share/emwm-desktop/app-defaults/XmSm',
                  'usr/share/emwm-desktop/emwm-applications.menu']
        for name in ['emwm','xmsm','xmtoolbox','xmtray','tellmwm','emwm-wallpaper-tool',
                     'emwm-compositor','emwm-desktop-icons','emwm-appearance','pcmanfm']:
            required.append('usr/lib/emwm-desktop/bin/'+name)
        for path in required:
            self.assertIn(path,self.files)
        for native in ('bin/emwm-session', 'bin/emwm-desktop-icons', 'bin/emwm-compositor',
                       'bin/emwm-wallpaper-tool', 'bin/tellmwm', 'bin/xmtray', 'bin/xmsession',
                       'appearance/emwm-appearance',
                       'appearance/emwm-appearance-login'):
            self.assertEqual(self.read('usr/lib/emwm-desktop/' + native)[:4], b'\x7fELF', native)
        self.assertEqual(self.files['usr/lib/emwm-desktop/bin/xmsm'].mode & 0o7777,0o4755)
        privileged=[path for path,member in self.files.items() if member.mode & 0o6000]
        self.assertEqual(privileged,['usr/lib/emwm-desktop/bin/xmsm'])
        for path,member in self.files.items():
            self.assertEqual((member.uid,member.gid),(0,0))
            self.assertTrue(path.startswith(('usr/lib/emwm-desktop/','usr/share/emwm-desktop/',
                                            'usr/share/doc/emwm-desktop/','usr/share/applications/',
                                            'usr/share/xsessions/')) or path in ('usr/bin/emwm-desktop-session','usr/bin/emwm-appearance'), path)
        self.assertNotIn('usr/bin/emwm',self.files)
        for member in self.archive.getmembers():
            if member.isdir():
                self.assertEqual(member.mode & 0o7777, 0o755, member.name)

    def test_generic_configuration_and_bundled_source(self):
        for path in ['usr/bin/emwm-desktop-session','usr/share/emwm-desktop/defaults.Xresources',
                     'usr/share/emwm-desktop/session.conf','usr/share/emwm-desktop/toolboxrc']:
            content=self.read(path)
            for private in [b'/home/paulo',b'/mnt/2tera',b'DP-5',b'Irixium',b'anydesk']:
                self.assertNotIn(private,content)
        doc='usr/share/doc/emwm-desktop/'
        specs=json.loads(self.read(doc+'sources.lock.json'))
        self.assertEqual(specs,json.loads((ROOT/'sources.lock.json').read_text()))
        with tarfile.open(fileobj=io.BytesIO(self.read(doc+'sources.tar.xz'))) as sources:
            names=set(sources.getnames())
            for path in ['emwm/COPYING','emwm-utils/COPYING','emwm/src/WmArgb.c',
                         'pcmanfm/COPYING','pcmanfm/src/desktop.c',
                         'emwm-utils/src/tray.c','emwm-utils/src/tray.h',
                         'emwm-desktop/patches/emwm-utils/0001-toolchest-tray.patch',
                         'emwm-desktop/scripts/install.py',
                         'emwm-desktop/rust/gradient-rs/src/port.rs',
                         'emwm-desktop/rust/gradient-rs/Cargo.lock',
                         'emwm-desktop/rust/dependency-sources/sources.json',
                         'emwm-desktop/third_party/tellmwm/COPYING',
                         'emwm-desktop/apps/emwm-wallpaper-tool/src/backdrops.cpp']:
                self.assertIn(path,names)
            self.assertFalse(any('/.git/' in p or p.endswith('.o') for p in names))
            self.assertIn(b'TrayStart();', sources.extractfile('emwm-utils/src/tbmain.c').read())
            self.assertIn(b'widget_class->drag_end = on_drag_end;', sources.extractfile('pcmanfm/src/desktop.c').read())
        bindings = self.read('usr/share/emwm-desktop/system.emwmrc').decode()
        for direction in ['Left', 'Right', 'Up', 'Down']:
            self.assertRegex(bindings, rf'Super<Key>{direction}\s+window\s+f\.tile {direction.lower()}')
        deps=subprocess.check_output(['dpkg-deb','-f',str(self.package),'Depends'],text=True)
        self.assertNotIn('picom', deps)
        for package in ['libxm4','libgtk-3','libgl1','libxdamage1','gvfs','gnome-keyring']:
            self.assertIn(package,deps)


if __name__=='__main__':
    unittest.main()
