# SPDX-License-Identifier: GPL-3.0-only
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from core import Settings,edit_ini,atomic_write,detect_login
from login_helper import apply_login

class AppearanceTests(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root=Path(self.temp.name)
        self.home=self.root/'home'; self.home.mkdir()
        self.system=self.root/'usr/share'; self.system.mkdir(parents=True)
        self.settings=Settings(self.home,{'XDG_DATA_DIRS':str(self.system)})
        roots=patch.object(self.settings,'system_roots',side_effect=lambda kind:[self.system/('themes' if kind=='gtk' else 'icons')])
        roots.start(); self.addCleanup(roots.stop)
        for name in ('Irixium','Tema com espaços'):
            path=self.system/'themes'/name/'gtk-3.0/gtk.css'; path.parent.mkdir(parents=True); path.write_text('')
        self.icon=self.system/'icons/Test Icons'; self.icon.mkdir(parents=True)
        (self.icon/'index.theme').write_text('[Icon Theme]\nName=Test\nDirectories=16x16/apps\n')
        (self.system/'icons/Test Cursor/cursors').mkdir(parents=True)
        self.values=dict(theme='Irixium',icons='Test Icons',cursor='Test Cursor',size=32,palette='irix')

    def login(self):
        path=self.root/'etc/X11/default-display-manager'; path.parent.mkdir(parents=True); path.write_text('/usr/sbin/lightdm\n')
        path=self.system/'xgreeters/lightdm-greeter.desktop'; path.parent.mkdir(parents=True)
        path.write_text('[Desktop Entry]\nExec=lightdm-gtk-greeter\n')
        path=self.root/'etc/lightdm/lightdm-gtk-greeter.conf'; path.parent.mkdir(parents=True)
        path.write_text('# Keep this comment\n[greeter]\nbackground=/wallpaper.jpg\ntheme-name=Old\nindicators=~clock;~session\n')
        return path

    def test_discovery_precedence_and_separate_cursors(self):
        local=self.home/'.themes/Irixium/gtk-3.0'; local.mkdir(parents=True); (local/'gtk.css').write_text('')
        self.assertEqual(self.settings.discover('gtk')['Irixium'].path,local.parent)
        self.assertNotIn('Test Cursor',self.settings.discover('icons'))
        self.assertNotIn('Test Icons',self.settings.discover('cursor'))
        self.assertIn('Tema com espaços',self.settings.discover('gtk'))
        self.assertIn('Adwaita',self.settings.discover('gtk'))

    def test_duplicate_icons_keep_personal_precedence_and_allow_login(self):
        local=self.home/'.icons/Test Icons'; local.mkdir(parents=True)
        (local/'index.theme').write_text((self.icon/'index.theme').read_text())
        themes=self.settings.discover('icons')
        self.assertEqual(list(themes).count('Test Icons'),1)
        self.assertEqual(themes['Test Icons'].path,local)
        self.assertFalse(themes['Test Icons'].system)
        self.assertEqual(themes['Test Icons'].system_capabilities,('Ícones',))
        self.settings.validate_login_themes(self.values)
        (self.icon/'index.theme').unlink()
        with self.assertRaises(ValueError): self.settings.validate_login_themes(self.values)

    def test_login_checks_system_gtk_version_independently(self):
        local=self.home/'.themes/Irixium/gtk-2.0'; local.mkdir(parents=True)
        (local/'gtkrc').write_text('')
        self.assertEqual(self.settings.discover('gtk')['Irixium'].capabilities,('GTK 2',))
        self.settings.validate_login_themes(self.values)
        # Reverse the versions: personal GTK 3 cannot make system GTK 2 usable.
        (local.parent/'gtk-3.0').mkdir(); (local.parent/'gtk-3.0/gtk.css').write_text('')
        system=self.system/'themes/Irixium'
        (system/'gtk-3.0/gtk.css').unlink()
        (system/'gtk-2.0').mkdir(); (system/'gtk-2.0/gtkrc').write_text('')
        with self.assertRaisesRegex(ValueError,'GTK 3'): self.settings.validate_login_themes(self.values)

    def test_system_copy_available_even_with_custom_xdg_dirs(self):
        self.settings.env['XDG_DATA_DIRS']=str(self.root/'other')
        self.settings.validate_login_themes(self.values)

    def test_builtin_gtk_available_with_personal_override(self):
        local=self.home/'.themes/Adwaita/gtk-2.0'; local.mkdir(parents=True)
        (local/'gtkrc').write_text('')
        theme=self.settings.discover('gtk')['Adwaita']
        self.assertEqual(theme.path,local.parent)
        self.assertIn('GTK 3',theme.system_capabilities)
        self.settings.validate_login_themes(dict(self.values,theme='Adwaita'))

    def test_preserve_configs_backups_idempotence_and_shell_quoting(self):
        (self.home/'.Xprofile').write_text('export GTK_THEME=Old\nxrandr --output test --rotate left\n')
        (self.home/'.Xresources').write_text('XTerm*faceSize: 14\n#include "/unchanged/backdrops.resources"\n')
        p=self.settings.config/'gtk-3.0/settings.ini'; p.parent.mkdir(parents=True)
        p.write_text('# comment\n[Settings]\ngtk-font-name=Serif 12\ngtk-theme-name=Old\n')
        backup=self.settings.apply(self.values)
        self.assertTrue((backup/'manifest.txt').is_file())
        self.assertIn('gtk-font-name=Serif 12',p.read_text())
        self.assertIn('backdrops.resources',(self.home/'.Xresources').read_text())
        self.assertIn('xrandr --output test --rotate left',(self.home/'.Xprofile').read_text())
        self.assertIn('unset GTK_THEME',(self.settings.directory/'session.sh').read_text())
        before={p:p.read_bytes() for p in self.home.rglob('*') if p.is_file() and 'appearance-backups' not in str(p)}
        self.settings.apply(self.values)
        for path,contents in before.items(): self.assertEqual(path.read_bytes(),contents)
        subprocess.run(['sh','-n',str(self.home/'.Xprofile')],check=True)
        subprocess.run(['sh','-n',str(self.settings.directory/'session.sh')],check=True)
        self.assertEqual(self.settings.current(),self.values)

    def test_keep_palette_retains_previous_motif_colors(self):
        self.settings.apply(self.values)
        before=(self.settings.directory/'appearance.resources').read_text()
        self.settings.apply(dict(self.values,palette='keep',size=40))
        after=(self.settings.directory/'appearance.resources').read_text()
        self.assertIn('Xcursor.size: 40',after)
        for line in before.splitlines():
            if not line.startswith('Xcursor.'): self.assertIn(line,after)

    def test_atomic_write_preserves_symlink_and_mode(self):
        target=self.root/'actual'; target.write_text('old'); target.chmod(0o640)
        link=self.root/'link'; link.symlink_to(target)
        atomic_write(link,'new',self.root/'backups')
        self.assertTrue(link.is_symlink()); self.assertEqual(target.read_text(),'new')
        self.assertEqual(target.stat().st_mode & 0o777,0o640)

    def test_invalid_selection_does_not_write(self):
        for bad in ('../Irixium','Irixium\n[greeter]','/etc/shadow'):
            with self.assertRaises(ValueError): self.settings.apply(dict(self.values,theme=bad))
        self.assertFalse(self.settings.directory.exists())

    def test_write_failure_rolls_back(self):
        import core
        original=core.atomic_write
        count=0
        def fail(path,text,*args,**kwargs):
            nonlocal count
            count+=1
            if count==3: raise OSError('simulated disk error')
            return original(path,text,*args,**kwargs)
        with patch('core.atomic_write',side_effect=fail):
            with self.assertRaises(OSError): self.settings.apply(self.values)
        self.assertFalse((self.settings.config/'gtk-3.0/settings.ini').exists())
        self.assertFalse((self.settings.config/'gtk-4.0/settings.ini').exists())

    def test_login_detection_and_key_only_update(self):
        target=self.login()
        self.assertTrue(detect_login(self.root)['supported'])
        before=target.read_text()
        backup=apply_login('Irixium','Test Icons','Test Cursor',32,self.root)
        text=target.read_text()
        self.assertIn('theme-name=Irixium',text)
        self.assertIn('background=/wallpaper.jpg',text)
        self.assertIn('indicators=~clock;~session',text)
        self.assertIn('# Keep this comment',text)
        self.assertEqual((backup/'lightdm-gtk-greeter.conf-0').read_text(),before)
        with self.assertRaises(ValueError): apply_login('Missing','Test Icons','Test Cursor',32,self.root)
        self.assertEqual(target.read_text(),text)

    def test_unknown_greeter_and_seat_overrides(self):
        target=self.login()
        conf=self.root/'etc/lightdm/lightdm.conf'
        conf.write_text('[Seat:*]\ngreeter-session=lightdm-greeter\n[Seat:seat0]\ngreeter-session=slick-greeter\n')
        self.assertFalse(detect_login(self.root,'seat0')['supported'])
        self.assertTrue(detect_login(self.root,'seat1')['supported'])
        before=target.read_text()
        with patch.dict(os.environ,{'XDG_SEAT':'seat0'}):
            with self.assertRaises(ValueError): apply_login('Irixium','Test Icons','Test Cursor',24,self.root)
        self.assertEqual(target.read_text(),before)

    def test_edit_ini_preserves_other_sections(self):
        text='[Other]\ntheme-name=Keep\n[greeter]\n# hi\ntheme-name=One\ntheme-name=Two\nfont-name=Serif\n[Last]\na=b\n'
        result=edit_ini(text,'greeter',{'theme-name':'New','cursor-theme-size':'24'})
        self.assertEqual(result.count('theme-name=New'),1)
        self.assertIn('theme-name=Keep',result)
        self.assertIn('font-name=Serif',result)
        self.assertEqual(edit_ini(result,'greeter',{'theme-name':'New','cursor-theme-size':'24'}),result)

if __name__=='__main__': unittest.main()
