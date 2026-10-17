# SPDX-License-Identifier: GPL-3.0-only
"""Theme discovery and non-destructive configuration, independent of GTK."""
import configparser
from dataclasses import dataclass
from datetime import datetime
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import stat
import tempfile

PALETTES = {
    'keep': ('Manter as cores atuais', None),
    'irix': ('IRIX clássico', ('#d2d2d2', '#000000', '#8c8c8c', '#aba68c')),
    'classic': ('Motif cinza', ('#bdbdbd', '#000000', '#a0a0a0', '#7399ba')),
}


def valid_name(value):
    if not isinstance(value, str) or not value or len(value) > 200 or value in ('.', '..') or any(c in value for c in '/\\\r\n\x00') or any(ord(c) < 32 for c in value):
        raise ValueError('Nome de tema inválido.')
    return value


def read(path):
    try:
        return Path(path).read_text()
    except FileNotFoundError:
        return ''


def ini_value(text, section, key, default=''):
    parser = configparser.ConfigParser(interpolation=None, strict=False)
    try:
        parser.read_string(text)
        return parser.get(section, key, fallback=default)
    except configparser.Error:
        return default


def edit_ini(text, section, values):
    """Replace only selected keys in a section, retaining comments/other keys."""
    output, seen = [], set()
    current, found = '', False
    def missing():
        return [f'{k}={v}\n' for k, v in values.items() if k.lower() not in seen]
    for line in text.splitlines(keepends=True):
        match = re.match(r'^\s*\[([^]]+)\]', line)
        if match:
            if current == section:
                output.extend(missing())
                seen.update(k.lower() for k in values)
            current = match[1]
            if current == section:
                found = True
        assignment = re.match(r'^\s*([^#;\s=]+)\s*=', line)
        keys = {k.lower(): k for k in values}
        if current == section and assignment and assignment[1].lower() in keys:
            key = keys[assignment[1].lower()]
            if key.lower() not in seen:
                output.append(f'{key}={values[key]}\n')
                seen.add(key.lower())
        else:
            output.append(line if line.endswith('\n') else line+'\n')
    if not found:
        output.append(f'\n[{section}]\n')
    output.extend(missing())
    return ''.join(output)


def managed_block(text, name, content, comment='#'):
    start, end = f'{comment} BEGIN {name}', f'{comment} END {name}'
    pattern = re.compile(r'^'+re.escape(start)+r'\n.*?^'+re.escape(end)+r'\n?', re.M|re.S)
    clean = pattern.sub('', text).rstrip('\n')
    return clean+'\n\n'+start+'\n'+content.rstrip('\n')+'\n'+end+'\n'


def atomic_write(path, text, backups=None, mode=0o600):
    """Preserve symlink targets and permissions; never truncate an existing file."""
    path = Path(path).resolve()
    path.parent.mkdir(parents=True, exist_ok=True)
    old = path.read_bytes() if path.exists() else None
    data = text.encode()
    if old == data:
        return
    if old is not None:
        mode = stat.S_IMODE(path.stat().st_mode)
        if backups is not None:
            backups = Path(backups)
            backups.mkdir(parents=True, exist_ok=True)
            name = path.name+'-'+str(len(list(backups.iterdir())))
            shutil.copy2(path, backups/name)
            with (backups/'manifest.txt').open('a') as stream:
                stream.write(f'{name}\t{path}\n')
    fd, temp = tempfile.mkstemp(prefix='.'+path.name+'.', dir=path.parent)
    try:
        with os.fdopen(fd, 'wb') as stream:
            stream.write(data)
            stream.flush()
            os.fsync(stream.fileno())
        os.chmod(temp, mode)
        os.replace(temp, path)
    finally:
        if os.path.exists(temp): os.unlink(temp)


@dataclass
class Theme:
    name: str
    path: Path | None
    capabilities: tuple
    system: bool
    system_capabilities: tuple = ()


class Settings:
    def __init__(self, home=None, env=None):
        self.env = dict(os.environ if env is None else env)
        self.home = Path(home or self.env.get('HOME', str(Path.home())))
        self.config = Path(self.env.get('XDG_CONFIG_HOME') or self.home/'.config')
        self.data = Path(self.env.get('XDG_DATA_HOME') or self.home/'.local/share')
        self.state = Path(self.env.get('XDG_STATE_HOME') or self.home/'.local/state')
        self.directory = self.config/'emwm-desktop/appearance'

    def roots(self, kind):
        leaf = 'themes' if kind == 'gtk' else 'icons'
        roots = [self.data/leaf, self.home/('.themes' if kind == 'gtk' else '.icons')]
        for root in self.env.get('XDG_DATA_DIRS','/usr/local/share:/usr/share').split(':'):
            if root and Path(root).is_absolute(): roots.append(Path(root)/leaf)
        # libXcursor traditionally also searches this directory.
        if kind == 'cursor': roots.append(Path('/usr/share/pixmaps'))
        return list(dict.fromkeys(roots))

    def system_roots(self, kind):
        leaf = 'themes' if kind == 'gtk' else 'icons'
        return [Path(base)/leaf for base in ('/usr/local/share','/usr/share')]

    def discover(self, kind):
        found = {}
        system_roots = self.system_roots(kind)
        for root in dict.fromkeys(self.roots(kind) + system_roots):
            try: children = sorted(root.iterdir())
            except OSError: continue
            for path in children:
                if not path.is_dir(): continue
                try: valid_name(path.name)
                except ValueError: continue
                try: info = read(path/'index.theme')
                except (OSError, UnicodeError): continue
                if kind == 'gtk':
                    caps = tuple(v for v, file in [('GTK 2','gtk-2.0/gtkrc'),('GTK 3','gtk-3.0/gtk.css'),('GTK 4','gtk-4.0/gtk.css')] if (path/file).is_file())
                elif kind == 'icons':
                    caps = ('Ícones',) if ini_value(info,'Icon Theme','Directories') or ini_value(info,'Icon Theme','ScaledDirectories') else ()
                else:
                    caps = ('Cursor',) if (path/'cursors').is_dir() else ()
                if caps:
                    system = root in system_roots
                    theme = found.setdefault(path.name,Theme(path.name,path,caps,system))
                    # Keep session precedence, but retain the independently
                    # available system assets for the login greeter.
                    if system:
                        theme.system_capabilities = tuple(dict.fromkeys(theme.system_capabilities + caps))
        if kind == 'gtk':
            for name in ('Adwaita','Adwaita-dark'):
                theme = found.setdefault(name,Theme(name,None,('GTK 3',),True))
                theme.system_capabilities = tuple(dict.fromkeys(theme.system_capabilities + ('GTK 3',)))
        return dict(sorted(found.items(), key=lambda pair:pair[0].casefold()))

    def validate_login_themes(self, values):
        for key,kind in [('theme','gtk'),('icons','icons'),('cursor','cursor')]:
            theme = self.discover(kind).get(values[key])
            if theme is None or not theme.system_capabilities:
                raise ValueError(f'{values[key]} não está disponível para o login. Instale o tema também em /usr/share/themes ou /usr/share/icons; ou desmarque a opção de login.')
            if key == 'theme' and 'GTK 3' not in theme.system_capabilities:
                raise ValueError('O LightDM GTK Greeter precisa de um tema GTK 3 instalado para todo o sistema.')

    def current(self):
        saved = read(self.directory/'selection.json')
        if saved:
            try: return self.validate(json.loads(saved))
            except (ValueError,TypeError): pass
        gtk = read(self.config/'gtk-3.0/settings.ini')
        resource = read(self.home/'.Xresources')
        def xr(key,default):
            match = re.search(r'^'+re.escape(key)+r'\s*:\s*(.+)$',resource,re.M)
            return match[1].strip() if match else default
        return dict(theme=ini_value(gtk,'Settings','gtk-theme-name',self.env.get('GTK_THEME','Adwaita').split(':')[0]),
                    icons=ini_value(gtk,'Settings','gtk-icon-theme-name','Adwaita'),
                    cursor=ini_value(gtk,'Settings','gtk-cursor-theme-name',xr('Xcursor.theme','Adwaita')),
                    size=int(ini_value(gtk,'Settings','gtk-cursor-theme-size',xr('Xcursor.size','24'))), palette='keep')

    def validate(self, values):
        result = {key:valid_name(values[key]) for key in ('theme','icons','cursor')}
        result['size'] = int(values['size'])
        result['palette'] = values.get('palette','keep')
        if not 8 <= result['size'] <= 128: raise ValueError('Tamanho de cursor deve estar entre 8 e 128.')
        if result['palette'] not in PALETTES: raise ValueError('Paleta inválida.')
        return result

    def apply(self, values):
        values = self.validate(values)
        for key,kind in [('theme','gtk'),('icons','icons'),('cursor','cursor')]:
            if values[key] not in self.discover(kind): raise ValueError(f'Tema não encontrado: {values[key]}')
        backup = self.state/'emwm-desktop/appearance-backups'/datetime.now().strftime('%Y%m%d-%H%M%S-%f')
        changes = {}
        settings = {'gtk-theme-name':values['theme'], 'gtk-icon-theme-name':values['icons'],
                    'gtk-cursor-theme-name':values['cursor'], 'gtk-cursor-theme-size':str(values['size'])}
        for version in ('3.0','4.0'):
            path = self.config/f'gtk-{version}/settings.ini'
            changes[path] = edit_ini(read(path),'Settings',settings)
        # GTK2 rc syntax uses quoted strings. Preserve includes and custom styles.
        path = self.home/'.gtkrc-2.0'
        content = '\n'.join(f'{key} = {value if key.endswith("size") else json.dumps(value,ensure_ascii=False)}' for key,value in settings.items())
        changes[path] = managed_block(read(path),'EMWM APPEARANCE',content)
        resources = f'Xcursor.theme: {values["cursor"]}\nXcursor.size: {values["size"]}\n'
        palette = PALETTES[values['palette']][1]
        if palette:
            bg,fg,frame,active = palette
            resources += f'*background: {bg}\n*foreground: {fg}\nEmwm*client*background: {frame}\nEmwm*client*activeBackground: {active}\nEmwm*icon*activeBackground: {active}\n'
        else:
            # 'Keep' means keep the current Motif palette, including a palette
            # previously selected here, rather than silently reverting at login.
            resources += ''.join(line+'\n' for line in read(self.directory/'appearance.resources').splitlines()
                                 if line and not line.startswith(('Xcursor.theme:', 'Xcursor.size:')))
        changes[self.directory/'appearance.resources'] = resources
        path = self.home/'.Xresources'
        include_path = str(self.directory/'appearance.resources')
        if any(c in include_path for c in '\n\r"\\'): raise ValueError('Caminho de configuração incompatível com Xresources.')
        changes[path] = managed_block(read(path),'EMWM APPEARANCE',f'#include "{include_path}"',comment='!')
        changes[self.directory/'session.sh'] = ('# Generated by EMWM Appearance\nunset GTK_THEME\n'
                    +f'export XCURSOR_THEME={shlex.quote(values["cursor"])}\nexport XCURSOR_SIZE={values["size"]}\n')
        path = self.home/'.Xprofile'
        hook = ('# The selected appearance is activated only in an EMWM session.\n'
                'case "${XDG_CURRENT_DESKTOP:-}:${DESKTOP_SESSION:-}:${XDG_SESSION_DESKTOP:-}" in\n'
                '  *EMWM*|*emwm*)\n'
                f'    . {shlex.quote(str(self.directory/"session.sh"))}\n'
                '    if command -v emwm-appearance >/dev/null 2>&1; then emwm-appearance --session; fi\n'
                '    ;;\nesac')
        changes[path] = managed_block(read(path),'EMWM APPEARANCE',hook)
        changes[self.directory/'xsettingsd.conf'] = (
            f'Net/ThemeName {json.dumps(values["theme"],ensure_ascii=False)}\n'
            f'Net/IconThemeName {json.dumps(values["icons"],ensure_ascii=False)}\n'
            f'Gtk/CursorThemeName {json.dumps(values["cursor"],ensure_ascii=False)}\n'
            f'Gtk/CursorThemeSize {values["size"]}\n')
        # qt5ct/qt6ct can share icon names; GTK CSS is not a Qt style.
        for version in ('5','6'):
            path = self.config/f'qt{version}ct/qt{version}ct.conf'
            if path.is_file(): changes[path] = edit_ini(read(path),'Appearance',{'icon_theme':values['icons']})
        changes[self.directory/'selection.json'] = json.dumps(values,ensure_ascii=False,indent=2)+'\n'
        # Read/validate all planned contents before touching any file. Restore
        # prior contents if a write fails (e.g. a read-only configuration).
        originals = {path: (path.read_bytes() if path.exists() else None) for path in changes}
        done = []
        try:
            for path,text in changes.items():
                atomic_write(path,text,backup)
                done.append(path)
        except Exception:
            for path in reversed(done):
                old = originals[path]
                if old is None: path.unlink(missing_ok=True)
                else: atomic_write(path,old.decode())
            raise
        return backup


def detect_login(root=Path('/'), seat=None):
    root = Path(root)
    manager = Path(read(root/'etc/X11/default-display-manager').strip()).name
    service = root/'etc/systemd/system/display-manager.service'
    if service.is_symlink():
        service_name = Path(os.readlink(service)).name.removesuffix('.service')
        if service_name: manager = service_name
    result = {'manager':manager or 'desconhecido','greeter':'desconhecido','supported':False}
    if manager != 'lightdm': return result
    seat = seat or os.environ.get('XDG_SEAT','seat0')
    import fnmatch
    files = []
    for directory in ('usr/share/lightdm/lightdm.conf.d','usr/local/share/lightdm/lightdm.conf.d','etc/lightdm/lightdm.conf.d'):
        files.extend(sorted((root/directory).glob('*.conf')))
    files.append(root/'etc/lightdm/lightdm.conf')
    greeter = 'lightdm-greeter'
    for path in files:
        parser = configparser.ConfigParser(interpolation=None,strict=False)
        try: parser.read_string(read(path))
        except configparser.Error: continue
        for section in parser.sections():
            if section.startswith('Seat:') and fnmatch.fnmatchcase(seat,section[5:]):
                greeter = parser.get(section,'greeter-session',fallback=greeter)
    # Do not treat arbitrary config text as a command or path.
    if not re.fullmatch(r'[A-Za-z0-9_.-]+',greeter): return result
    desktop = root/'usr/share/xgreeters'/f'{greeter}.desktop'
    executable = ini_value(read(desktop),'Desktop Entry','Exec')
    try: command = Path(shlex.split(executable)[0]).name if executable else ''
    except ValueError: command = ''
    result['greeter'] = command or greeter
    result['supported'] = command == 'lightdm-gtk-greeter'
    return result
