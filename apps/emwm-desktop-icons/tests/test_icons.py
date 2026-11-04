import importlib.machinery
import importlib.util
from pathlib import Path
import tempfile
import socket
import unittest

loader = importlib.machinery.SourceFileLoader('icons', str(Path(__file__).resolve().parents[1] / 'emwm-desktop-icons'))
spec = importlib.util.spec_from_loader(loader.name, loader)
icons = importlib.util.module_from_spec(spec)
loader.exec_module(icons)


class DesktopTests(unittest.TestCase):
    def test_existing_file_manager_is_detected_without_ipc(self):
        with tempfile.TemporaryDirectory() as temp:
            runtime = Path(temp)
            path = runtime / 'pcmanfm-socket--17'
            with socket.socket(socket.AF_UNIX) as server:
                server.bind(str(path))
                server.listen()
                self.assertTrue(icons.active_pcmanfm_socket(runtime, ':17.0'))
                self.assertFalse(icons.active_pcmanfm_socket(runtime, ':18'))
            self.assertFalse(icons.active_pcmanfm_socket(runtime, ':17'))
    def test_localized_and_quoted_wallpapers(self):
        resources = 'Emwm*ws0*backdrop*image: "/tmp/Área de trabalho/a \\"b\\".png"\nEmwm.ws1.backdrop.image: "/tmp/bg.png"'
        self.assertEqual(icons.wallpapers(resources), {0: '/tmp/Área de trabalho/a "b".png', 1: '/tmp/bg.png'})
        self.assertEqual(icons.wallpapers('Emwm*ws2*backdrop*image: "bad'), {})

    def test_profile_does_not_replace_positions_or_other_profiles(self):
        with tempfile.TemporaryDirectory() as temp:
            config = Path(temp)
            icons.prepare_profile(config, 2)
            path = config / 'pcmanfm/emwm-desktop/desktop-items-0.conf'
            self.assertIn('show_trash=1', path.read_text())
            path.write_text('[file.txt]\nx=42\ny=100\n')
            icons.prepare_profile(config, 3)
            self.assertEqual(path.read_text(), '[file.txt]\nx=42\ny=100\n')
            self.assertTrue((path.parent / 'desktop-items-2.conf').is_file())


if __name__ == '__main__':
    unittest.main()
