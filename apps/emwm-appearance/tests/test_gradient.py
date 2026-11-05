import sys
from pathlib import Path
import unittest
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from gradient import validate, resources
import test_appearance as fixture


class GradientValidation(unittest.TestCase):
    def test_validation_and_encoding(self):
        for values in ({'mode':'invalid'},{'active_start':'red\nBad: value'},[],
                       {'inactive_text':None}):
            with self.assertRaises(ValueError):validate(values)
        self.assertIn('Emwm*titleGradient: vertical\n',resources({'mode':'vertical'}))
        self.assertEqual(validate({'active_start':'#ABCDEF'})['active_start'],'#abcdef')


class GradientPersistence(unittest.TestCase):
    setUp = fixture.AppearanceTests.setUp
    def test_gradient_roundtrip_keep_disable_and_backup(self):
        values={**self.values,'gradient':{'mode':'diagonal','active_start':'#123456'}}
        self.settings.apply(values)
        self.assertEqual(self.settings.current()['gradient']['active_start'],'#123456')
        path=self.settings.directory/'appearance.resources'
        before=path.read_text()
        values=self.settings.current();values['palette']='keep'
        self.settings.apply(values)
        self.assertEqual(path.read_text(),before)
        values['gradient']['mode']='none'
        backup=self.settings.apply(values)
        self.assertIn('Emwm*titleGradient: none',path.read_text())
        self.assertEqual(path.read_text().count('Emwm*titleGradient:'),1)
        self.assertTrue(any(p.read_text()==before for p in backup.rglob('*') if p.is_file()))
