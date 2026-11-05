import sys
from pathlib import Path
import tempfile
import unittest
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from desktop import DEFAULTS, configuration, current, validate


class DesktopPreferences(unittest.TestCase):
    def test_round_trip_preserves_unknown_options(self):
        with tempfile.TemporaryDirectory() as temp:
            config = Path(temp)
            path = config / 'emwm-desktop/desktop.conf'
            path.parent.mkdir()
            path.write_text('# personal settings\n[desktop]\nfuture=42\n')
            values = {'monitor': 'DP-4', 'anchor': 'right'}
            dest, text = configuration(config, values)
            self.assertEqual(dest, path)
            self.assertIn('future=42', text)
            self.assertIn('# personal settings', text)
            path.write_text(text)
            self.assertEqual(current(config), values)
            path.write_text('[desktop]\nmonitor=DP-6\nanchor=oops\n')
            self.assertEqual(current(config), {'monitor': 'DP-6', 'anchor': 'left'})

    def test_invalid_selections(self):
        for values in ({'monitor': ''}, {'monitor': 'DP-4\nanchor=right'},
                       {'monitor': None}, {'anchor': 'top'}):
            with self.assertRaises(ValueError):
                validate(values)
        self.assertEqual(validate({}), DEFAULTS)
