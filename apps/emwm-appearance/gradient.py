# SPDX-License-Identifier: GPL-3.0-only
"""Title-bar gradient preferences and their Xresources representation."""
import re

DEFAULTS = {'mode': 'none', 'active_start': '#24466b', 'active_end': '#7399ba',
            'active_text': '#ffffff', 'inactive_start': '#b0b0b0',
            'inactive_end': '#707070', 'inactive_text': '#101010'}
RESOURCES = {'mode': 'titleGradient', **{
    key: 'titleGradient' + ''.join(word.title() for word in key.split('_'))
    for key in DEFAULTS if key != 'mode'}}


def validate(values):
    if not isinstance(values, dict):
        raise ValueError('Configuração de gradiente inválida.')
    result = {**DEFAULTS, **values}
    if result['mode'] not in ('none', 'horizontal', 'vertical', 'diagonal'):
        raise ValueError('Direção de gradiente inválida.')
    for key in DEFAULTS:
        if key == 'mode': continue
        value = result[key]
        if not isinstance(value, str) or not re.fullmatch(r'#[0-9a-fA-F]{6}', value):
            raise ValueError('Cor do gradiente inválida: use #RRGGBB.')
        result[key] = value.lower()
    return {key: result[key] for key in DEFAULTS}


def resources(values):
    return ''.join(f'Emwm*{RESOURCES[key]}: {value}\n' for key, value in validate(values).items())


class Controls:
    def __init__(self, Gtk, Gdk, values, changed):
        self.Gdk = Gdk
        self.frame = Gtk.Frame(label='Gradiente da barra de título')
        grid = Gtk.Grid(column_spacing=12, row_spacing=10)
        grid.set_border_width(12); self.frame.add(grid)
        self.mode = Gtk.ComboBoxText()
        for key, label in [('none','Sem gradiente (cores do tema)'),
                           ('horizontal','Horizontal — esquerda para direita'),
                           ('vertical','Vertical — cima para baixo'),
                           ('diagonal','Diagonal — canto superior esquerdo ao inferior direito')]:
            self.mode.append(key,label)
        grid.attach(Gtk.Label(label='Preenchimento', xalign=0),0,0,1,1)
        grid.attach(self.mode,1,0,3,1)
        for col, label in enumerate(('Cor inicial','Cor final','Texto'),1):
            grid.attach(Gtk.Label(label=label),col,1,1,1)
        self.colors = {}
        for row, (state,label) in enumerate((('active','Janela ativa'),('inactive','Janela inativa')),2):
            grid.attach(Gtk.Label(label=label,xalign=0),0,row,1,1)
            for col, part in enumerate(('start','end','text'),1):
                button = Gtk.ColorButton()
                button.set_title(label+' — '+{'start':'cor inicial','end':'cor final','text':'texto'}[part])
                self.colors[state+'_'+part] = button
                grid.attach(button,col,row,1,1)
                button.connect('color-set',lambda *_:changed())
        note = Gtk.Label(label='A prévia não altera as janelas. Aplicar atualiza o gradiente e o texto ao vivo. '
                         'O relevo e os botões Motif são preservados.',xalign=0)
        note.set_line_wrap(True); note.set_max_width_chars(65)
        grid.attach(note,0,4,4,1)
        self.reload(values)
        self.mode.connect('changed',lambda *_:changed())

    def reload(self, values):
        values = validate(values)
        self.mode.set_active_id(values['mode'])
        for key, button in self.colors.items():
            color = self.Gdk.RGBA(); color.parse(values[key]); button.set_rgba(color)

    def values(self):
        values = {'mode': self.mode.get_active_id()}
        for key, button in self.colors.items():
            c = button.get_rgba()
            values[key] = '#'+''.join(f'{round(v*255):02x}' for v in (c.red,c.green,c.blue))
        return validate(values)
