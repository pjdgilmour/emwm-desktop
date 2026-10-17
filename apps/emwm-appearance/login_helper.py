#!/usr/bin/python3 -I
# SPDX-License-Identifier: GPL-3.0-only
"""Privileged helper for LightDM GTK appearance and --background PNG input.
No command execution, service restart or user-supplied filesystem paths.
"""
import os
from pathlib import Path
import sys
sys.path.insert(0,str(Path(__file__).resolve().parent))
from core import atomic_write, detect_login, edit_ini, read, valid_name


def apply_background(data, mode, root=Path('/')):
    import hashlib
    import re
    import tempfile
    from datetime import datetime
    from wallpaper import MODES, normalize_png
    root=Path(root)
    if not detect_login(root)['supported']:
        raise ValueError('O fundo de login requer LightDM GTK Greeter.')
    if mode not in MODES: raise ValueError('Modo de fundo inválido.')
    data=normalize_png(data)
    directory=root/'usr/share/backgrounds/emwm-desktop'
    # The administrator-controlled destination must never redirect writes into
    # an unprivileged directory, even if a previous installation was damaged.
    for path in (directory,*directory.parents):
        if path.is_symlink(): raise ValueError('Destino de fundo não pode conter links simbólicos.')
        if root == Path('/') and path.exists():
            info=path.stat()
            if info.st_uid != 0 or info.st_mode & 0o022:
                raise ValueError('O diretório de fundos deve pertencer ao administrador e não permitir escrita por outros usuários.')
        if path == root: break
    directory.mkdir(parents=True,exist_ok=True,mode=0o755)
    name='lightdm-'+hashlib.sha256(data).hexdigest()+'.png'
    target=directory/name
    if target.is_symlink(): raise ValueError('Destino de imagem inválido.')
    existed=target.exists()
    prefix='#zoomed:' if mode=='zoom' else '#stretched:'
    value=prefix+'/usr/share/backgrounds/emwm-desktop/'+name
    config=root/'etc/lightdm/lightdm-gtk-greeter.conf'
    old=read(config)
    changes={'background':value,'user-background':'false'}
    new=edit_ini(old,'greeter',changes)
    # Existing per-monitor choices override [greeter]; update backgrounds there
    # too, but preserve explicitly disabled monitors and unrelated options.
    from core import ini_value
    for section in re.findall(r'^\s*\[(monitor:[^]]+)\]',old,re.M):
        if ini_value(old,section,'background') != '#skip':
            new=edit_ini(new,section,changes)
    backup=root/'var/backups/emwm-appearance'/datetime.now().strftime('%Y%m%d-%H%M%S-%f')
    fd,temp=tempfile.mkstemp(prefix='.lightdm-',dir=directory)
    try:
        with os.fdopen(fd,'wb') as stream:
            stream.write(data); stream.flush(); os.fsync(stream.fileno())
        os.chmod(temp,0o644)
        os.replace(temp,target)
        try: atomic_write(config,new,backup,mode=0o644)
        except Exception:
            if not existed: target.unlink()
            raise
    finally:
        if os.path.exists(temp): os.unlink(temp)
    return backup


def apply_login(theme,icons,cursor,size,root=Path('/')):
    root=Path(root)
    if not detect_login(root)['supported']:
        raise ValueError('Gerenciador/greeter detectado não suporta temas GTK pelo método desta ferramenta.')
    for name in (theme,icons,cursor): valid_name(name)
    size=int(size)
    if not 8<=size<=128: raise ValueError('Tamanho de cursor inválido.')
    def exists(leaf,name,file):
        return any((root/base/leaf/name/file).is_file() for base in ('usr/share','usr/local/share'))
    if theme not in ('Adwaita','Adwaita-dark') and not exists('themes',theme,'gtk-3.0/gtk.css'):
        raise ValueError(f'O tema GTK {theme} precisa estar instalado em /usr/share/themes ou /usr/local/share/themes.')
    if not exists('icons',icons,'index.theme'):
        raise ValueError(f'Os ícones {icons} precisam estar instalados para todo o sistema.')
    if not any((root/base/'icons'/cursor/'cursors').is_dir() for base in ('usr/share','usr/local/share')):
        raise ValueError(f'O cursor {cursor} precisa estar instalado para todo o sistema.')
    target=root/'etc/lightdm/lightdm-gtk-greeter.conf'
    # This administrator-owned file is loaded after the greeter's conf.d files.
    # Only appearance keys change; authentication/session configuration survives.
    old=read(target)
    new=edit_ini(old,'greeter',{'theme-name':theme,'icon-theme-name':icons,
                              'cursor-theme-name':cursor,'cursor-theme-size':str(size)})
    from datetime import datetime
    backup=root/'var/backups/emwm-appearance'/datetime.now().strftime('%Y%m%d-%H%M%S-%f')
    atomic_write(target,new,backup,mode=0o644)
    return backup


def main():
    if os.geteuid()!=0: raise PermissionError('Esta operação requer autenticação administrativa.')
    if len(sys.argv)==3 and sys.argv[1]=='--background':
        from wallpaper import MAX_BYTES
        print(apply_background(sys.stdin.buffer.read(MAX_BYTES+1),sys.argv[2]))
        return
    if len(sys.argv)!=5: raise ValueError('Uso: login_helper.py TEMA ICONES CURSOR TAMANHO')
    print(apply_login(*sys.argv[1:]))

if __name__=='__main__':
    try: main()
    except (OSError,ValueError) as error:
        print(str(error),file=sys.stderr); sys.exit(1)
