import sys
from pathlib import Path
import tempfile
import unittest
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from compositor import DEFAULTS, configuration, current, validate


class CompositorPreferences(unittest.TestCase):
    def test_invalid_settings_are_rejected(self):
        for values in ({'backend': 'vulkan'}, {'duration_ms': -1}, {'effect': 'bad'},
                       {'shadow_opacity': float('nan')}, {'shadows': 'false'}):
            with self.assertRaises(ValueError):
                validate(values)

    def test_round_trip_and_unknown_keys_preserved(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            path = root / 'emwm-desktop/compositor.conf'
            path.parent.mkdir()
            path.write_text('# local preferences\n[compositor]\ncustom_future_option=42\n')
            values = {**DEFAULTS, 'backend': 'glx', 'effect': 'flip', 'shadow_opacity': .7}
            dest, text = configuration(root, values)
            self.assertEqual(dest, path)
            self.assertIn('custom_future_option=42', text)
            self.assertIn('# local preferences', text)
            path.write_text(text)
            self.assertEqual(current(root), values)

    def test_bad_file_recovers_per_setting(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            path = root / 'emwm-desktop/compositor.conf'
            path.parent.mkdir()
            path.write_text('[compositor]\neffect=twist\nduration_ms=oops\nshadow_opacity=nan\n')
            values = current(root)
            self.assertEqual(values['effect'], 'twist')
            self.assertEqual(values['duration_ms'], DEFAULTS['duration_ms'])
            self.assertEqual(values['shadow_opacity'], DEFAULTS['shadow_opacity'])
