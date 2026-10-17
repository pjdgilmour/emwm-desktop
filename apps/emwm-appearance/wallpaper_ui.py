# SPDX-License-Identifier: GPL-3.0-only
"""Independent LightDM wallpaper dialog; does not change session themes."""
import json
import subprocess
import threading
from pathlib import Path
import gi
gi.require_version('Gtk', '3.0')
gi.require_version('Gdk', '3.0')
gi.require_version('GdkPixbuf', '2.0')
from gi.repository import Gtk, Gdk, GdkPixbuf, GLib
from core import atomic_write, read
from wallpaper import MODES, compose, load_image, prepare, encode


def show(parent, settings, helper):
    dialog = Gtk.Dialog(title='Fundo da tela de login', transient_for=parent, modal=True)
    dialog.set_default_size(660, 560)
    dialog.add_button('Fechar', Gtk.ResponseType.CLOSE)
    apply = dialog.add_button('Aplicar ao login', Gtk.ResponseType.APPLY)
    box = dialog.get_content_area(); box.set_spacing(10); box.set_border_width(12)
    grid = Gtk.Grid(column_spacing=12, row_spacing=10)
    box.pack_start(grid, False, False, 0)
    chooser = Gtk.FileChooserButton(title='Escolher imagem', action=Gtk.FileChooserAction.OPEN)
    chooser.set_hexpand(True)
    filefilter = Gtk.FileFilter(); filefilter.set_name('Imagens (PNG, JPEG, WebP, BMP, TIFF, GIF)')
    for pattern in ('*.png','*.jpg','*.jpeg','*.webp','*.bmp','*.tif','*.tiff','*.gif'):
        filefilter.add_pattern(pattern); filefilter.add_pattern(pattern.upper())
    chooser.add_filter(filefilter)
    mode = Gtk.ComboBoxText()
    for key, label in MODES.items(): mode.append(key, label)
    mode.set_active_id('zoom')
    color = Gtk.ColorButton(); rgba = Gdk.RGBA(); rgba.parse('#000000'); color.set_rgba(rgba)
    width = Gtk.SpinButton.new_with_range(1, 16384, 1)
    height = Gtk.SpinButton.new_with_range(1, 16384, 1)
    screen = parent.get_screen()
    monitor = screen.get_primary_monitor()
    geometry = screen.get_monitor_geometry(max(0, monitor))
    scale = screen.get_monitor_scale_factor(max(0, monitor))
    width.set_value(geometry.width*scale); height.set_value(geometry.height*scale)
    resolution = Gtk.Box(spacing=6)
    resolution.pack_start(width, True, True, 0)
    resolution.pack_start(Gtk.Label(label='×'), False, False, 0)
    resolution.pack_start(height, True, True, 0)
    prefpath = settings.directory/'login-background.json'
    try:
        saved = json.loads(read(prefpath) or '{}')
        if saved.get('path') and Path(saved['path']).is_file(): chooser.set_filename(saved['path'])
        if saved.get('mode') in MODES: mode.set_active_id(saved['mode'])
        if saved.get('color') and rgba.parse(saved['color']): color.set_rgba(rgba)
        if saved.get('width'): width.set_value(saved['width'])
        if saved.get('height'): height.set_value(saved['height'])
    except (ValueError, TypeError, KeyError): pass
    for row, (label, widget) in enumerate([('Imagem',chooser),('Posicionamento',mode),('Cor das margens',color),('Resolução de referência',resolution)]):
        grid.attach(Gtk.Label(label=label,xalign=0),0,row,1,1); grid.attach(widget,1,row,1,1)
    preview = Gtk.Image(); preview.set_size_request(320,180)
    box.pack_start(preview, True, True, 0)
    preview_button = Gtk.Button(label='Atualizar prévia')
    box.pack_start(preview_button, False, False, 0)
    note = Gtk.Label(label='Centralizar, ajustar e mosaico usam a resolução de referência. Em monitores com outra resolução, o resultado será redimensionado. Preencher e esticar se adaptam a cada monitor.\nA cópia para o sistema é feita ao aplicar, com senha administrativa.',xalign=0)
    note.set_line_wrap(True); note.set_max_width_chars(76); box.pack_start(note,False,False,0)
    status = Gtk.Label(xalign=0); status.set_line_wrap(True); status.set_max_width_chars(76)
    box.pack_start(status,False,False,0)
    busy = False

    def values():
        rgba = color.get_rgba()
        return dict(path=chooser.get_filename(), mode=mode.get_active_id(),
                    width=width.get_value_as_int(), height=height.get_value_as_int(),
                    color='#'+''.join(f'{round(c*255):02x}' for c in (rgba.red,rgba.green,rgba.blue)))

    def update_preview(*_):
        try:
            v = values()
            if not v['path']: return
            with open(v['path'],'rb') as stream: image=load_image(stream)
            image=compose(image,v['mode'],v['width'],v['height'],v['color'])
            image.thumbnail((480,200))
            loader=GdkPixbuf.PixbufLoader.new_with_type('png')
            loader.write(encode(image)); loader.close()
            preview.set_from_pixbuf(loader.get_pixbuf()); status.set_text('Prévia; nenhuma alteração aplicada.')
        except (OSError,ValueError) as error: status.set_text(str(error))

    def done(result, selected, error=None):
        nonlocal busy
        busy=False; parent.busy=False; apply.set_sensitive(True); grid.set_sensitive(True)
        if error: status.set_text(error)
        elif result.returncode:
            status.set_text('Fundo não alterado: '+(result.stderr.decode(errors='replace').strip() or 'autenticação cancelada.'))
        else:
            message='Fundo do login instalado. Será usado no próximo login; a imagem original pode ser movida ou removida.'
            try: atomic_write(prefpath,json.dumps(selected,ensure_ascii=False,indent=2)+'\n')
            except OSError as error: message+=' Não foi possível salvar a seleção local: '+str(error)
            status.set_text(message)
        return False

    def response(_dialog, response_id):
        nonlocal busy
        if busy: return
        if response_id != Gtk.ResponseType.APPLY:
            dialog.destroy(); return
        try:
            if helper is None or '--background' not in helper.read_text():
                raise ValueError('Atualize o componente administrativo: sudo ./apps/emwm-appearance/install-system.sh (no repositório emwm-desktop).')
            selected=values()
            if not selected['path']: raise ValueError('Selecione uma imagem primeiro.')
            data=prepare(**selected)
            busy=True; parent.busy=True; apply.set_sensitive(False); grid.set_sensitive(False)
            status.set_text('Aguardando autenticação e instalação do fundo…')
            def job():
                try:
                    result=subprocess.run(['pkexec','/usr/bin/python3','-I',str(helper),'--background',selected['mode']],input=data,capture_output=True)
                    GLib.idle_add(done,result,selected)
                except OSError as error: GLib.idle_add(done,None,selected,str(error))
            threading.Thread(target=job,daemon=True).start()
        except (OSError,ValueError) as error: status.set_text(str(error))

    chooser.connect('file-set',update_preview); mode.connect('changed',update_preview)
    preview_button.connect('clicked',update_preview)
    dialog.connect('response',response)
    dialog.connect('delete-event',lambda *_:busy)
    dialog.show_all(); update_preview()
    return dialog
