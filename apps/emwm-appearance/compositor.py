# SPDX-License-Identifier: GPL-3.0-only
"""Compositor preferences shared by the appearance UI and its tests."""
import configparser
import math
from core import edit_ini, read

DEFAULTS = {'backend': 'auto', 'effect': 'zoom', 'duration_ms': 220,
            'fade_ms': 120, 'shadows': True, 'shadow_radius': 16,
            'shadow_offset': 6, 'shadow_opacity': .45, 'vsync': True, 'max_fps': 60}
RANGES = {'duration_ms': (50, 2000), 'fade_ms': (0, 1000),
          'shadow_radius': (1, 64), 'shadow_offset': (-32, 64), 'max_fps': (15, 240)}


def validate(values):
    result = dict(DEFAULTS)
    result.update(values)
    if result['backend'] not in ('auto', 'glx', 'xrender'):
        raise ValueError('Backend de composição inválido.')
    if result['effect'] not in ('none', 'zoom', 'twist', 'flip'):
        raise ValueError('Efeito de animação inválido.')
    for key, (low, high) in RANGES.items():
        value = result[key]
        if isinstance(value, bool) or not isinstance(value, int) or not low <= value <= high:
            raise ValueError(f'{key}: use um valor entre {low} e {high}.')
    for key in ('shadows', 'vsync'):
        if not isinstance(result[key], bool):
            raise ValueError(f'{key}: valor inválido.')
    opacity = result['shadow_opacity']
    if isinstance(opacity, bool) or not isinstance(opacity, (int, float)) or not math.isfinite(opacity) or not 0 <= opacity <= 1:
        raise ValueError('A intensidade da sombra deve estar entre 0 e 100%.')
    return {key: result[key] for key in DEFAULTS}


def current(config):
    parser = configparser.ConfigParser(interpolation=None, strict=False)
    values = dict(DEFAULTS)
    try:
        parser.read_string(read(config / 'emwm-desktop/compositor.conf'))
        for key, default in DEFAULTS.items():
            try:
                if isinstance(default, bool):
                    value = parser.getboolean('compositor', key, fallback=default)
                elif isinstance(default, int):
                    value = parser.getint('compositor', key, fallback=default)
                elif isinstance(default, float):
                    value = parser.getfloat('compositor', key, fallback=default)
                else:
                    value = parser.get('compositor', key, fallback=default)
                validate({key: value})
                values[key] = value
            except (ValueError, TypeError):
                pass
    except configparser.Error:
        pass
    return values


def configuration(config, values):
    values = validate(values)
    path = config / 'emwm-desktop/compositor.conf'
    encoded = {key: ('true' if value else 'false') if isinstance(value, bool) else str(value)
               for key, value in values.items()}
    return path, edit_ini(read(path), 'compositor', encoded)


class Controls:
    def __init__(self, Gtk, config):
        self.config = config
        self.widgets = {}
        self.frame = Gtk.Frame(label='Composição, sombras e animações')
        grid = Gtk.Grid(column_spacing=16, row_spacing=8)
        grid.set_border_width(12)
        self.frame.add(grid)
        rows = [
            ('backend', 'Renderização', [('auto', 'Automática (OpenGL quando disponível)'),
                                       ('glx', 'OpenGL / GPU'), ('xrender', 'XRender (compatibilidade)')]),
            ('effect', 'Minimizar, restaurar e maximizar', [('none', 'Sem animação'),
                       ('zoom', 'Zoom — contornos clássicos'), ('twist', 'Twist — giro'), ('flip', 'Flip — dobra')]),
            ('duration_ms', 'Duração da animação (ms)', RANGES['duration_ms']),
            ('fade_ms', 'Aparecimento das janelas (ms)', RANGES['fade_ms']),
            ('shadows', 'Sombras das janelas', None),
            ('shadow_radius', 'Suavidade da sombra (px)', RANGES['shadow_radius']),
            ('shadow_offset', 'Deslocamento da sombra (px)', RANGES['shadow_offset']),
            ('shadow_opacity', 'Intensidade da sombra (%)', (0, 100)),
            ('vsync', 'Sincronização vertical (OpenGL)', None),
            ('max_fps', 'Limite de quadros por segundo', RANGES['max_fps']),
        ]
        for row, (key, label, choices) in enumerate(rows):
            grid.attach(Gtk.Label(label=label, xalign=0), 0, row, 1, 1)
            if choices is None:
                widget = Gtk.CheckButton()
            elif isinstance(choices, list):
                widget = Gtk.ComboBoxText()
                for ident, text in choices:
                    widget.append(ident, text)
            else:
                widget = Gtk.SpinButton.new_with_range(*choices, 1)
            grid.attach(widget, 1, row, 1, 1)
            self.widgets[key] = widget
        note = Gtk.Label(label='Os efeitos lembram o Window Maker. Aplicar atualiza o compositor em execução.\n'
                         'OpenGL depende do driver; a sessão usa XRender se não puder iniciá-lo.', xalign=0)
        note.set_line_wrap(True)
        grid.attach(note, 0, len(rows), 2, 1)
        self.reload()

    def reload(self):
        values = current(self.config)
        for key, widget in self.widgets.items():
            value = values[key]
            if isinstance(value, bool):
                widget.set_active(value)
            elif isinstance(value, str):
                widget.set_active_id(value)
            else:
                widget.set_value(value * 100 if key == 'shadow_opacity' else value)

    def values(self):
        values = {}
        for key, widget in self.widgets.items():
            default = DEFAULTS[key]
            if isinstance(default, bool):
                values[key] = widget.get_active()
            elif isinstance(default, str):
                values[key] = widget.get_active_id()
            else:
                values[key] = widget.get_value_as_int() / 100 if key == 'shadow_opacity' else widget.get_value_as_int()
        return validate(values)
