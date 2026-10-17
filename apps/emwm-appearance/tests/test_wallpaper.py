# SPDX-License-Identifier: GPL-3.0-only
import io
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from PIL import Image
from wallpaper import compose, prepare, normalize_png, encode
from login_helper import apply_background
from core import ini_value


def login_tree(root):
    path=root/'etc/X11/default-display-manager'; path.parent.mkdir(parents=True)
    path.write_text('/usr/sbin/lightdm\n')
    path=root/'usr/share/xgreeters/lightdm-greeter.desktop'; path.parent.mkdir(parents=True)
    path.write_text('[Desktop Entry]\nExec=lightdm-gtk-greeter\n')
    path=root/'etc/lightdm/lightdm-gtk-greeter.conf'; path.parent.mkdir(parents=True)
    path.write_text('# keep\n[greeter]\nbackground=/old.png\nuser-background=true\ntheme-name=Irixium\nindicators=~clock\n[monitor: HDMI-1]\nbackground=/other.png\nlaptop=false\n[monitor: disabled]\nbackground=#skip\n')
    return path


class WallpaperTests(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory(); self.addCleanup(self.temp.cleanup)
        self.root=Path(self.temp.name)
        self.image=Image.new('RGBA',(2,1)); self.image.putdata([(255,0,0,255),(0,255,0,255)])
        self.source=self.root/'photo.png'; self.image.save(self.source)

    def test_center_and_tile_pixels(self):
        centered=compose(self.image,'center',4,3,'#0000ff')
        self.assertEqual(centered.getpixel((0,0)),(0,0,255))
        self.assertEqual(centered.getpixel((1,1)),(255,0,0))
        self.assertEqual(centered.getpixel((2,1)),(0,255,0))
        tiled=compose(self.image,'tile',5,3)
        for y in range(3):
            for x in range(5): self.assertEqual(tiled.getpixel((x,y)),(255,0,0) if x%2==0 else (0,255,0))

    def test_fit_zoom_and_stretch(self):
        image=Image.new('RGBA',(8,4),'red')
        fit=compose(image,'fit',8,8,'#0000ff')
        self.assertEqual(fit.getpixel((3,0)),(0,0,255))
        self.assertEqual(fit.getpixel((3,3)),(255,0,0))
        for mode in ('zoom','stretch'):
            result=compose(image,mode,8,8)
            self.assertEqual(result.getpixel((0,0)),(255,0,0))
            self.assertEqual(result.getpixel((7,7)),(255,0,0))

    def test_preparation_dimensions_and_transparency(self):
        for mode in ('zoom','stretch','center','fit','tile'):
            with Image.open(io.BytesIO(prepare(self.source,mode,8,6))) as image:
                self.assertEqual(image.size,(2,1) if mode in ('zoom','stretch') else (8,6))
                self.assertEqual(image.mode,'RGB')
        transparent=Image.new('RGBA',(1,1),(255,0,0,0))
        self.assertEqual(compose(transparent,'tile',3,3,'#0000ff').getpixel((2,2)),(0,0,255))

    def test_invalid_data_and_limits(self):
        for data in (b'',b'not PNG'):
            with self.assertRaises(ValueError): normalize_png(data)
        jpeg=io.BytesIO(); Image.new('RGB',(1,1)).save(jpeg,format='JPEG')
        with self.assertRaises(ValueError): normalize_png(jpeg.getvalue())
        for width,height in ((0,1),(16385,1),(10000,10000)):
            with self.assertRaises(ValueError): compose(self.image,'center',width,height)
        with self.assertRaises(ValueError): compose(self.image,'invalid',1,1)
        with self.assertRaises(ValueError): compose(self.image,'fit',1,1,'red\nkey=bad')

    def test_install_backup_modes_permissions_and_source_independence(self):
        config=login_tree(self.root); before=config.read_bytes()
        data=prepare(self.source,'tile',8,6)
        backup=apply_background(data,'tile',self.root)
        self.source.unlink()
        result=config.read_text()
        value=ini_value(result,'greeter','background')
        self.assertTrue(value.startswith('#stretched:/usr/share/backgrounds/emwm-desktop/lightdm-'))
        image=self.root/value.split(':',1)[1].lstrip('/')
        self.assertEqual(image.stat().st_mode & 0o777,0o644)
        with Image.open(image) as installed: self.assertEqual(installed.size,(8,6))
        self.assertEqual(ini_value(result,'greeter','user-background'),'false')
        self.assertEqual(ini_value(result,'monitor: HDMI-1','background'),value)
        self.assertEqual(ini_value(result,'monitor: disabled','background'),'#skip')
        self.assertIn('theme-name=Irixium',result); self.assertIn('indicators=~clock',result)
        self.assertEqual((backup/'lightdm-gtk-greeter.conf-0').read_bytes(),before)
        apply_background(data,'zoom',self.root)
        self.assertTrue(ini_value(config.read_text(),'greeter','background').startswith('#zoomed:'))
        self.assertEqual(len(list(image.parent.glob('*.png'))),1)

    def test_failure_preserves_old_config_and_does_not_leave_new_image(self):
        config=login_tree(self.root); before=config.read_bytes()
        with patch('login_helper.atomic_write',side_effect=OSError('disk full')):
            with self.assertRaises(OSError): apply_background(encode(self.image),'center',self.root)
        self.assertEqual(config.read_bytes(),before)
        self.assertFalse(list((self.root/'usr/share/backgrounds/emwm-desktop').glob('*.png')))
        with self.assertRaises(ValueError): apply_background(b'bad','tile',self.root)
        self.assertEqual(config.read_bytes(),before)

    def test_refuses_redirected_destination(self):
        config=login_tree(self.root); before=config.read_bytes()
        folder=self.root/'usr/share/backgrounds'; folder.mkdir()
        outside=self.root/'outside'; outside.mkdir()
        (folder/'emwm-desktop').symlink_to(outside,target_is_directory=True)
        with self.assertRaises(ValueError): apply_background(encode(self.image),'zoom',self.root)
        self.assertEqual(list(outside.iterdir()),[])
        self.assertEqual(config.read_bytes(),before)

if __name__=='__main__': unittest.main()
