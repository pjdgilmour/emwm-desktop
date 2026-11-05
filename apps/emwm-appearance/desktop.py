# SPDX-License-Identifier: GPL-3.0-only
"""Desktop placement preferences, consumed live by the private PCManFM."""
import configparser
import subprocess
from core import edit_ini, read

DEFAULTS = {'monitor': 'primary', 'anchor': 'left'}


def validate(values):
    result = {**DEFAULTS, **values}
    monitor = result['monitor']
    if (not isinstance(monitor, str) or not monitor or len(monitor) > 256
            or any(ord(c) < 32 or c in '\\[]' for c in monitor)):
        raise ValueError('Monitor inválido.')
    if result['anchor'] not in ('left', 'right'):
        raise ValueError('Escolha o alinhamento à esquerda ou à direita.')
    return {key: result[key] for key in DEFAULTS}


def current(config):
    parser = configparser.ConfigParser(interpolation=None, strict=False)
    values = dict(DEFAULTS)
    try:
        parser.read_string(read(config / 'emwm-desktop/desktop.conf'))
        for key, default in DEFAULTS.items():
            value = parser.get('desktop', key, fallback=default)
            try:
                validate({key: value})
                values[key] = value
            except ValueError:
                pass
    except configparser.Error:
        pass
    return values


def configuration(config, values):
    path = config / 'emwm-desktop/desktop.conf'
    return path, edit_ini(read(path), 'desktop', validate(values))


class Controls:
    def __init__(self, Gtk, Gdk, config):
        self.config = config
        self.screen = Gdk.Screen.get_default()
        self.frame = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=14)
        self.frame.set_border_width(14)
        intro = Gtk.Label(label='Ícones, arquivos e lixeira em um único monitor', xalign=0)
        self.frame.pack_start(intro, False, False, 0)
        grid = Gtk.Grid(column_spacing=16, row_spacing=12)
        self.frame.pack_start(grid, False, False, 0)
        self.monitor = Gtk.ComboBoxText()
        self.monitor.set_hexpand(True)
        self.anchor = Gtk.ComboBoxText()
        self.anchor.append('left', 'Esquerda')
        self.anchor.append('right', 'Direita')
        for row, (label, widget) in enumerate((('Monitor dos ícones', self.monitor),
                                               ('Ancorar ícones à', self.anchor))):
            grid.attach(Gtk.Label(label=label, xalign=0), 0, row, 1, 1)
            grid.attach(widget, 1, row, 1, 1)
        self.detected = Gtk.Label(xalign=0)
        self.detected.set_line_wrap(True)
        self.detected.set_max_width_chars(64)
        self.frame.pack_start(self.detected, False, False, 0)
        note = Gtk.Label(label='Seguir o sistema respeita o monitor principal informado pelo X11/driver. '
                         'Se ele ou a saída escolhida estiver desconectado, será usado o principal disponível '
                         'ou o primeiro monitor ativo. A escolha é lembrada quando o monitor voltar.\n\n'
                         'Os outros monitores continuam com o papel de parede. Ao trocar o lado, '
                         'as posições manuais são espelhadas horizontalmente, mantendo a altura. '
                         'Esta escolha afeta apenas os ícones; não altera a disposição dos monitores no driver.', xalign=0)
        note.set_line_wrap(True)
        note.set_max_width_chars(64)
        self.frame.pack_start(note, False, False, 0)
        wallpaper = Gtk.Button(label='Configurar papel de parede…')
        wallpaper.connect('clicked', lambda *_: subprocess.Popen(['emwm-wallpaper-tool']))
        self.frame.pack_start(wallpaper, False, False, 0)
        self.reload()
        if self.screen:
            handler = self.screen.connect('monitors-changed', self.monitors_changed)
            self.frame.connect('destroy', lambda *_: self.screen.disconnect(handler))

    def monitors_changed(self, *_):
        self.populate(self.monitor.get_active_id() or 'primary')

    def populate(self, selected):
        self.monitor.remove_all()
        self.monitor.append('primary', 'Seguir o principal do sistema')
        primary_name = None
        if self.screen:
            primary = self.screen.get_primary_monitor()
            for i in range(self.screen.get_n_monitors()):
                name = self.screen.get_monitor_plug_name(i)
                if not name:
                    continue
                geometry = self.screen.get_monitor_geometry(i)
                label = f'{name} — {geometry.width}×{geometry.height} ({geometry.x}, {geometry.y})'
                if i == primary:
                    label += ' · principal do sistema'
                    primary_name = name
                self.monitor.append(name, label)
        if not self.monitor.set_active_id(selected):
            self.monitor.append(selected, selected + ' · desconectado')
            self.monitor.set_active_id(selected)
        self.detected.set_text('Principal disponível informado pelo X11: ' +
                               (primary_name or 'não informado; será usado o primeiro ativo.') )

    def reload(self):
        values = current(self.config)
        self.populate(values['monitor'])
        self.anchor.set_active_id(values['anchor'])

    def values(self):
        return validate({'monitor': self.monitor.get_active_id(), 'anchor': self.anchor.get_active_id()})
