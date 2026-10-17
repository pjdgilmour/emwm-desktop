#!/usr/bin/python3
# SPDX-License-Identifier: GPL-3.0-only
import argparse
import json
import os
from pathlib import Path
import subprocess
import sys
import threading
from core import Settings, PALETTES, detect_login


def trusted_helper(background=False):
    path=Path('/usr/lib/emwm-desktop/appearance/login_helper.py')
    modules=[path,path.with_name('core.py')]
    if background: modules.append(path.with_name('wallpaper.py'))
    for parent in (*modules,*path.parents):
        try:
            info=parent.stat()
            if info.st_uid!=0 or info.st_mode & 0o022: return None
        except OSError: return None
    return path


def main():
    parser=argparse.ArgumentParser(description='Aparência do EMWM e da tela de login')
    parser.add_argument('--session',action='store_true',help='Ativar as preferências salvas, sem janela.')
    parser.add_argument('--list',action='store_true',help='Listar temas e detectar o gerenciador de login.')
    args=parser.parse_args()
    settings=Settings()
    if args.list:
        print(json.dumps({'login':detect_login(),**{kind:list(settings.discover(kind)) for kind in ('gtk','icons','cursor')}},ensure_ascii=False,indent=2)); return
    if args.session:
        if not (settings.directory/'selection.json').is_file(): return
        from runtime import apply_live
        for note in apply_live(settings,settings.current()): print(note,file=sys.stderr)
        return
    original=settings.current()
    # GTK_THEME overrides XSETTINGS. Do not impose it on this preview process.
    os.environ.pop('GTK_THEME',None)
    import gi
    gi.require_version('Gtk','3.0')
    gi.require_version('Gdk','3.0')
    from gi.repository import Gtk,Gdk,GLib
    from runtime import apply_live
    if not Gtk.init_check()[0]: raise RuntimeError('Abra a ferramenta dentro de uma sessão gráfica.')

    class Window(Gtk.Window):
        def __init__(self):
            super().__init__(title='Aparência do EMWM')
            self.set_default_size(680,700)
            self.set_border_width(14)
            self.connect('destroy',Gtk.main_quit)
            self.login=detect_login()
            self.busy=False
            self.connect('delete-event',lambda *_:self.busy)
            outer=Gtk.Box(orientation=Gtk.Orientation.VERTICAL,spacing=12)
            layout=Gtk.Box(orientation=Gtk.Orientation.VERTICAL,spacing=12)
            self.add(layout)
            scroll=Gtk.ScrolledWindow()
            scroll.set_policy(Gtk.PolicyType.NEVER,Gtk.PolicyType.AUTOMATIC)
            scroll.add(outer); layout.pack_start(scroll,True,True,0)
            intro=Gtk.Label(label='Temas dos aplicativos, ícones e ponteiro do mouse',xalign=0)
            outer.pack_start(intro,False,False,0)
            grid=Gtk.Grid(column_spacing=16,row_spacing=10)
            outer.pack_start(grid,False,False,0)
            self.combos={}
            self.catalogs={}
            for row,(key,kind,label) in enumerate([('theme','gtk','Tema dos aplicativos'),('icons','icons','Tema de ícones'),('cursor','cursor','Tema do cursor')]):
                combo=Gtk.ComboBoxText()
                self.populate(key,kind,combo,original[key])
                combo.set_hexpand(True)
                grid.attach(Gtk.Label(label=label,xalign=0),0,row,1,1)
                grid.attach(combo,1,row,1,1)
                self.combos[key]=combo
                combo.connect('changed',self.preview)
            self.size=Gtk.SpinButton.new_with_range(8,128,1)
            self.size.set_value(original['size'])
            self.size.connect('value-changed',self.preview)
            grid.attach(Gtk.Label(label='Tamanho do cursor',xalign=0),0,3,1,1)
            grid.attach(self.size,1,3,1,1)
            self.palette=Gtk.ComboBoxText()
            for key,(label,_) in PALETTES.items(): self.palette.append(key,label)
            self.palette.set_active_id(original.get('palette','keep'))
            grid.attach(Gtk.Label(label='Cores das janelas Motif',xalign=0),0,4,1,1)
            grid.attach(self.palette,1,4,1,1)
            frame=Gtk.Frame(label='Prévia local — nenhuma preferência é salva até Aplicar')
            demo=Gtk.Box(orientation=Gtk.Orientation.VERTICAL,spacing=8)
            demo.set_border_width(12)
            entry=Gtk.Entry(); entry.set_text('Texto de exemplo')
            demo.pack_start(entry,False,False,0)
            row=Gtk.Box(spacing=10)
            self.images=[]
            for icon in ('folder','text-x-generic','preferences-desktop-theme'):
                image=Gtk.Image.new_from_icon_name(icon,Gtk.IconSize.DIALOG)
                row.pack_start(image,False,False,0); self.images.append((image,icon))
            row.pack_end(Gtk.Button(label='Botão de exemplo'),False,False,0)
            demo.pack_start(row,False,False,0)
            check=Gtk.CheckButton(label='Opção de exemplo'); check.set_active(True)
            demo.pack_start(check,False,False,0)
            frame.add(demo); outer.pack_start(frame,False,False,0)
            logintext=f'Tela de login detectada: {self.login["manager"]} / {self.login["greeter"]}'
            outer.pack_start(Gtk.Label(label=logintext,xalign=0),False,False,0)
            self.apply_login=Gtk.CheckButton(label='Aplicar também à tela de login (solicita senha administrativa)')
            self.apply_login.set_sensitive(self.login['supported'])
            outer.pack_start(self.apply_login,False,False,0)
            wallpaper=Gtk.Button(label='Fundo da tela de login…')
            wallpaper.set_sensitive(self.login['supported'])
            wallpaper.connect('clicked',self.show_wallpaper)
            outer.pack_start(wallpaper,False,False,0)
            details=('A tela de login usa os temas instalados para todo o sistema. Quando há duas cópias, a sessão prioriza a pessoal e o login usa a do sistema. As mudanças aparecem no próximo login. '
                     'Aplicativos Qt/libadwaita podem manter aparência própria; GTK 2/4 precisam de arquivos compatíveis no tema. '
                     'As cores Motif entram em vigor ao reiniciar o WM; o formato das decorações é preservado.')
            if not self.login['supported']: details+=' Este gerenciador/greeter foi detectado, mas não possui integração de tema nesta versão.'
            info=Gtk.Label(label=details,xalign=0); info.set_line_wrap(True); info.set_max_width_chars(76)
            outer.pack_start(info,False,False,0)
            self.status=Gtk.Label(xalign=0); self.status.set_line_wrap(True); self.status.set_max_width_chars(76)
            outer.pack_start(self.status,True,True,0)
            buttons=Gtk.ButtonBox(orientation=Gtk.Orientation.HORIZONTAL)
            buttons.set_layout(Gtk.ButtonBoxStyle.END)
            self.reset=Gtk.Button(label='Recarregar'); self.reset.connect('clicked',self.reload)
            self.apply=Gtk.Button(label='Aplicar'); self.apply.connect('clicked',self.on_apply)
            close=Gtk.Button(label='Fechar'); close.connect('clicked',lambda *_:self.close())
            for button in (self.reset,close,self.apply): buttons.add(button)
            layout.pack_end(buttons,False,False,0)
            self.preview()

        def values(self):
            return {**{key:combo.get_active_id() for key,combo in self.combos.items()},
                    'size':self.size.get_value_as_int(),'palette':self.palette.get_active_id()}

        def show_wallpaper(self,*_):
            from wallpaper_ui import show
            show(self,settings,trusted_helper(background=True))

        def populate(self,key,kind,combo,selected):
            self.catalogs[key]=settings.discover(kind)
            combo.remove_all()
            for name,theme in self.catalogs[key].items():
                suffix=' · '+', '.join(theme.capabilities) if key=='theme' else ''
                if not theme.system:
                    suffix+=' · pessoal + sistema' if theme.system_capabilities else ' · pessoal'
                else:
                    suffix+=' · sistema'
                combo.append(name,name+suffix)
            if not combo.set_active_id(selected): combo.set_active(0)

        def preview(self,*_):
            if not hasattr(self,'images'): return
            values=self.values(); prefs=Gtk.Settings.get_default()
            for key,prop in [('theme','gtk-theme-name'),('icons','gtk-icon-theme-name'),('cursor','gtk-cursor-theme-name')]:
                if values[key]: prefs.set_property(prop,values[key])
            prefs.set_property('gtk-cursor-theme-size',values['size'])
            for image,icon in self.images: image.set_from_icon_name(icon,Gtk.IconSize.DIALOG)
            if self.get_window(): self.get_window().set_cursor(Gdk.Cursor.new_from_name(self.get_display(),'default'))

        def reload(self,*_):
            values=settings.current()
            for key,kind in [('theme','gtk'),('icons','icons'),('cursor','cursor')]:
                self.populate(key,kind,self.combos[key],values[key])
            self.size.set_value(values['size']); self.palette.set_active_id(values['palette'])
            self.status.set_text('Temas e preferências recarregados.'); self.preview()

        def on_apply(self,*_):
            try:
                values=settings.validate(self.values())
                login=self.apply_login.get_active()
                helper=trusted_helper() if login else None
                if login and not helper:
                    raise ValueError('Instale o componente administrativo primeiro: sudo ./apps/emwm-appearance/install-system.sh (no repositório emwm-desktop).')
                if login:
                    settings.validate_login_themes(values)
                backup=settings.apply(values)
                notes=apply_live(settings,values)
                self.status.set_text('Aplicado à sessão. Backup: '+str(backup)+'\n'+'\n'.join(notes)+
                    '\nSe a sessão foi iniciada com GTK_THEME fixado, saia e entre novamente uma vez para liberar a troca de tema em todos os aplicativos.')
                if login:
                    self.busy=True; self.apply.set_sensitive(False); self.reset.set_sensitive(False)
                    def job():
                        try:
                            result=subprocess.run(['pkexec','/usr/bin/python3','-I',str(helper),values['theme'],values['icons'],values['cursor'],str(values['size'])],capture_output=True,text=True)
                            message=('Tela de login configurada. Será usada no próximo login.' if result.returncode==0 else
                                     'A sessão foi configurada, mas a tela de login não foi alterada: '+(result.stderr.strip() or 'autenticação cancelada.'))
                        except OSError as error: message='Sessão configurada; falha na integração do login: '+str(error)
                        GLib.idle_add(self.login_done,message)
                    threading.Thread(target=job,daemon=True).start()
            except (OSError,ValueError,TypeError) as error:
                self.status.set_text(str(error))

        def login_done(self,message):
            self.busy=False; self.apply.set_sensitive(True); self.reset.set_sensitive(True)
            self.status.set_text(self.status.get_text()+'\n'+message)
            return False

    window=Window(); window.show_all(); Gtk.main()

if __name__=='__main__': main()
