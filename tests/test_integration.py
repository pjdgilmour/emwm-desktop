# SPDX-License-Identifier: GPL-3.0-only
import io
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile
import time
import unittest

ROOT = Path(__file__).resolve().parent.parent


class SessionTests(unittest.TestCase):
    def test_existing_profile_resource_order_and_component_switches(self):
        with tempfile.TemporaryDirectory() as temp:
            base = Path(temp)
            home = base / 'home'
            home.mkdir()
            prefix = base / 'private'
            bins = prefix / 'bin'
            bins.mkdir(parents=True)
            data = base / 'data'
            data.mkdir()
            config = home / '.config/emwm-desktop'
            config.mkdir(parents=True)
            record = base / 'record'
            (home / '.Xprofile').write_text('export USER_PROFILE_READ=yes\n')
            (home / '.Xdefaults').write_text('XTerm*faceSize: 12\n')
            (home / '.Xresources').write_text('Emwm*workspaceCount: 4\n')
            (config / 'session.conf').write_text(
                'EMWM_DESKTOP_POLKIT=0\nEMWM_DESKTOP_NETWORK=0\nEMWM_DESKTOP_PICOM_BACKEND=glx\n')
            before = {p: p.read_bytes() for p in home.rglob('*') if p.is_file()}
            emitter = bins / 'emitter'
            emitter.write_text('''#!/usr/bin/python3
import json,os,sys
name=os.path.basename(sys.argv[0])
if name=='pgrep': sys.exit(1)
record={'name':name,'args':sys.argv[1:],'profile':os.environ.get('USER_PROFILE_READ'),
        'desktop':os.environ.get('XDG_CURRENT_DESKTOP'),'path':os.environ['PATH']}
fd=os.open(os.environ['TEST_RECORD'],os.O_WRONLY|os.O_CREAT|os.O_APPEND,0o600)
os.write(fd,(json.dumps(record)+'\\n').encode());os.close(fd)
''')
            emitter.chmod(0o755)
            for name in ['xrdb', 'dbus-update-activation-environment', 'pgrep', 'xmsm',
                         'xmtray', 'picom', 'gnome-keyring-daemon', 'nm-applet']:
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
                if len(events) >= 8:
                    break
                time.sleep(.02)
            names = [e['name'] for e in events]
            self.assertIn('xmsm', names)
            self.assertIn('xmtray', names)
            self.assertIn('gnome-keyring-daemon', names)
            self.assertNotIn('nm-applet', names)
            self.assertEqual([e['args'][1] for e in events if e['name']=='xrdb'],
                             [str(data/'defaults.Xresources'), str(home/'.Xdefaults'), str(home/'.Xresources')])
            picom = next(e for e in events if e['name']=='picom')
            self.assertEqual(picom['args'][:2], ['--backend', 'glx'])
            self.assertTrue(all(e['profile']=='yes' and e['desktop']=='EMWM' for e in events))
            self.assertTrue(all(e['path'].startswith(str(bins)+':') for e in events))
            for path, contents in before.items():
                self.assertEqual(path.read_bytes(), contents)

    def test_requires_x11_without_modifying_home(self):
        with tempfile.TemporaryDirectory() as temp:
            env=dict(os.environ,HOME=temp)
            env.pop('DISPLAY',None)
            proc=subprocess.run(['sh', str(ROOT/'session/emwm-desktop-session')],
                                env=env,capture_output=True,text=True)
            self.assertNotEqual(proc.returncode,0)
            self.assertIn('X11 display',proc.stderr)
            self.assertEqual(list(Path(temp).iterdir()),[])


class PackageTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        packages=list((ROOT/'dist').glob('*.deb'))
        if not packages:
            raise unittest.SkipTest('Gere um pacote com --build-only para verificar seu conteúdo.')
        cls.package=max(packages,key=lambda p:p.stat().st_mtime)
        data=subprocess.check_output(['dpkg-deb','--fsys-tarfile',str(cls.package)])
        cls.archive=tarfile.open(fileobj=io.BytesIO(data))
        cls.files={p.name.lstrip('./'):p for p in cls.archive.getmembers() if p.isfile()}

    def read(self,path):
        return self.archive.extractfile(self.files[path]).read()

    def test_private_layout_and_permissions(self):
        required=['usr/bin/emwm-appearance','usr/lib/emwm-desktop/appearance/login_helper.py',
                  'usr/lib/emwm-desktop/appearance/wallpaper.py',
                  'usr/lib/emwm-desktop/appearance/wallpaper_ui.py',
                  'usr/bin/emwm-desktop-session','usr/share/xsessions/emwm-desktop.desktop',
                  'usr/share/emwm-desktop/app-defaults/Emwm','usr/share/emwm-desktop/app-defaults/XmSm',
                  'usr/share/emwm-desktop/emwm-applications.menu']
        for name in ['emwm','xmsm','xmtoolbox','xmtray','tellmwm','emwm-wallpaper-tool']:
            required.append('usr/lib/emwm-desktop/bin/'+name)
        for path in required:
            self.assertIn(path,self.files)
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
                         'emwm-utils/src/tray.c','emwm-desktop/scripts/install.py',
                         'emwm-desktop/third_party/tellmwm/COPYING',
                         'emwm-desktop/apps/emwm-wallpaper-tool/src/backdrops.cpp']:
                self.assertIn(path,names)
            self.assertFalse(any('/.git/' in p or p.endswith('.o') for p in names))
        deps=subprocess.check_output(['dpkg-deb','-f',str(self.package),'Depends'],text=True)
        for package in ['libxm4','libgtkmm-3.0','picom','gnome-keyring']:
            self.assertIn(package,deps)


if __name__=='__main__':
    unittest.main()
