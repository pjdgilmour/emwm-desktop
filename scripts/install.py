#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build a pinned EMWM Desktop .deb as the user; elevate only apt operations."""
import argparse
import hashlib
import io
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile

ROOT = Path(__file__).resolve().parent.parent
PREFIX = '/usr/lib/emwm-desktop'
DATA = '/usr/share/emwm-desktop'
BUILD_DEPS = [
    'build-essential', 'git', 'ca-certificates', 'python3', 'python3-pil', 'cmake', 'pkg-config',
    'dpkg-dev', 'libmotif-dev', 'libx11-dev', 'libxt-dev', 'libxext-dev',
    'libxrender-dev', 'libxrandr-dev', 'libxinerama-dev', 'libxss-dev',
    'libcrypt-dev', 'libgarcon-1-dev', 'libglib2.0-dev', 'libgtkmm-3.0-dev',
]
TEST_DEPS = ['xvfb', 'x11-xserver-utils', 'x11-utils', 'libxtst-dev',
             'libxcomposite-dev', 'picom', 'dbus-daemon', 'procps',
             'fonts-liberation', 'xfonts-base', 'python3-gi', 'python3-pil', 'gir1.2-gtk-3.0', 'xsettingsd']
RUNTIME = [
    'xserver-xorg', 'xinit', 'x11-xserver-utils', 'x11-utils', 'xterm', 'pcmanfm',
    'picom', 'mate-polkit', 'gnome-keyring', 'libpam-gnome-keyring',
    'dbus-user-session', 'dbus-x11', 'xdg-utils', 'fonts-liberation', 'procps',
    'python3-gi', 'python3-pil', 'gir1.2-gtk-3.0', 'xsettingsd', 'pkexec',
]


def run(args, cwd=None, capture=False, env=None):
    print('+ ' + ' '.join(str(a) for a in args), flush=True)
    return subprocess.run([str(a) for a in args], cwd=cwd, check=True, env=env,
                          stdout=subprocess.PIPE if capture else None,
                          text=True).stdout


def debian_version():
    values = {}
    for line in Path('/etc/os-release').read_text().splitlines():
        if '=' in line:
            key, value = line.split('=', 1)
            values[key] = value.strip('"')
    if values.get('ID') != 'debian' or values.get('VERSION_ID') != '13':
        raise RuntimeError('Esta versão do instalador foi preparada para Debian 13 (trixie).')


def source_snapshot(name, spec, work, cache, local):
    revision = spec['revision']
    if not re.fullmatch(r'[0-9a-f]{40}', revision):
        raise RuntimeError(f'Revisão inválida em sources.lock.json: {name}')
    if local:
        repo = local / name
        run(['git', '-C', repo, 'cat-file', '-e', revision + '^{commit}'])
    else:
        repo = cache / (name + '.git')
        if not repo.exists():
            run(['git', 'init', '--bare', repo])
        try:
            run(['git', '-C', repo, 'fetch', '--depth=1', spec['url'], revision])
        except subprocess.CalledProcessError as error:
            raise RuntimeError(f'Não foi possível obter {name}@{revision}. '
                               'Confirme que os commits dos forks já foram publicados; '
                               'para testar antes do push, use --sources /diretorio/dos/repositorios.') from error
    archive = subprocess.check_output(['git', '-C', str(repo), 'archive', revision])
    dest = work / name
    dest.mkdir()
    with tarfile.open(fileobj=io.BytesIO(archive)) as stream:
        stream.extractall(dest, filter='data')
    return dest


def copy(src, dest, mode=0o644):
    dest.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(src, dest)
    dest.chmod(mode)


def build(args):
    version = (ROOT / 'VERSION').read_text().strip()
    if not re.fullmatch(r'[0-9][0-9A-Za-z.+~:-]*', version):
        raise RuntimeError('VERSION inválida.')
    specs = json.loads((ROOT / 'sources.lock.json').read_text())
    base = ROOT / 'build'
    cache = base / 'cache'
    cache.mkdir(parents=True, exist_ok=True)
    dist = ROOT / 'dist'
    dist.mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='package-', dir=base) as temp:
        work = Path(temp)
        wm = source_snapshot('emwm', specs['emwm'], work, cache, args.sources)
        utils = source_snapshot('emwm-utils', specs['emwm-utils'], work, cache, args.sources)
        # Always build clean source snapshots, never the user's working trees.
        variables = [f'PREFIX={PREFIX}', f'RCDIR={DATA}', f'APPLRESDIR={DATA}/app-defaults']
        for project in (wm, utils):
            (project / 'src/Makefile').symlink_to('../mf/Makefile.Linux')
        run(['make', '-C', 'src', '-j' + str(args.jobs), *variables], cwd=wm)
        # Use logind-compatible systemctl instead of the obsolete pm-suspend.
        # CPPFLAGS applies to the existing xmsm compilation rules.
        run(['make', '-C', 'src', '-j' + str(args.jobs), *variables,
             'CPPFLAGS=-DSUSPEND_CMD=\'"/usr/bin/systemctl suspend"\''], cwd=utils)
        tell = work / 'tellmwm'
        shutil.copytree(ROOT / 'third_party/tellmwm', tell)
        run(['make', '-j' + str(args.jobs)], cwd=tell)
        wallpaper = work / 'wallpaper-build'
        run(['cmake', '-S', ROOT / 'apps/emwm-wallpaper-tool', '-B', wallpaper,
             '-DCMAKE_BUILD_TYPE=Release'])
        run(['cmake', '--build', wallpaper, '-j', args.jobs])
        run(['ctest', '--test-dir', wallpaper, '--output-on-failure'])
        if args.test:
            run(['make', 'test'], cwd=wm)
            run(['make', 'test-argb'], cwd=wm)
            run(['make', 'test-x11'], cwd=utils)
            env = dict(os.environ, EMWM_BINARY=str(wm / 'src/emwm'))
            run(['cmake', '--build', wallpaper, '--target', 'test-x11'], env=env)
        run([sys.executable, '-m', 'unittest', 'discover', '-s',
             ROOT / 'apps/emwm-appearance/tests', '-v'])
        if args.test:
            run(['sh', ROOT / 'apps/emwm-appearance/tests/run-x11.sh'])
        stage = work / 'debian/emwm-desktop'
        bin_dir = stage / PREFIX.lstrip('/') / 'bin'
        data_dir = stage / DATA.lstrip('/')
        bins = {'emwm': wm / 'src/emwm', 'xmtoolbox': utils / 'src/xmtoolbox',
                'xmtray': utils / 'src/xmtray', 'xmsm': utils / 'src/xmsm',
                'xmsession': utils / 'src/xmsession', 'tellmwm': tell / 'tellmwm',
                'emwm-wallpaper-tool': wallpaper / 'emwm-wallpaper-tool'}
        for name, src in bins.items():
            copy(src, bin_dir / name, 0o4755 if name == 'xmsm' else 0o755)
        for name, src in {'Emwm': wm / 'src/Emwm.ad', 'XmSm': utils / 'src/XmSm.ad',
                          'XmToolbox': utils / 'src/XmToolbox.ad'}.items():
            copy(src, data_dir / 'app-defaults' / name)
        copy(wm / 'src/system.emwmrc', data_dir / 'system.emwmrc')
        copy(utils / 'src/emwm-applications.menu', data_dir / 'emwm-applications.menu')
        for src in (utils / 'src/desktop-directories').glob('*.directory'):
            copy(src, data_dir / 'desktop-directories' / src.name)
        for src in (ROOT / 'config').iterdir():
            copy(src, data_dir / src.name)
        appearance = ROOT / 'apps/emwm-appearance'
        for name in ['core.py', 'gui.py', 'runtime.py', 'login_helper.py', 'wallpaper.py', 'wallpaper_ui.py']:
            copy(appearance / name, stage / PREFIX.lstrip('/') / 'appearance' / name)
        copy(appearance / 'emwm-appearance', stage / 'usr/bin/emwm-appearance', 0o755)
        copy(appearance / 'emwm-appearance.desktop', stage / 'usr/share/applications/emwm-appearance.desktop')
        copy(ROOT / 'session/emwm-desktop-session', stage / 'usr/bin/emwm-desktop-session', 0o755)
        copy(ROOT / 'packaging/emwm-desktop.desktop', stage / 'usr/share/xsessions/emwm-desktop.desktop')
        copy(ROOT / 'packaging/emwm-wallpaper-tool.desktop', stage / 'usr/share/applications/emwm-desktop-wallpaper.desktop')
        manuals = [wm / 'src/emwm.1', wm / 'src/emwmrc.4', utils / 'src/xmtoolbox.1',
                   utils / 'src/xmsm.1', utils / 'src/xmtray.1', tell / 'tellmwm.1']
        for src in manuals:
            copy(src, data_dir / 'man' / ('man' + src.suffix[1:]) / src.name)
        doc = stage / 'usr/share/doc/emwm-desktop'
        copy(ROOT / 'README.md', doc / 'README.md')
        copy(ROOT / 'THIRD_PARTY.md', doc / 'copyright')
        copy(ROOT / 'LICENSE', doc / 'licenses/emwm-desktop-GPL-3')
        copy(wm / 'COPYING', doc / 'licenses/emwm-COPYING')
        copy(utils / 'COPYING', doc / 'licenses/emwm-utils-COPYING')
        copy(tell / 'COPYING', doc / 'licenses/tellmwm-COPYING')
        copy(ROOT / 'sources.lock.json', doc / 'sources.lock.json')
        # Ship the exact source snapshots for the binaries, including build scripts
        # and license notices, even when someone redistributes just the .deb.
        source_tar = doc / 'sources.tar.xz'
        with tarfile.open(source_tar, 'w:xz') as archive:
            for name, spec in specs.items():
                repo = args.sources / name if args.sources else cache / (name + '.git')
                content = subprocess.check_output(['git', '-C', str(repo), 'archive', spec['revision']])
                with tarfile.open(fileobj=io.BytesIO(content)) as source:
                    for member in source.getmembers():
                        data = source.extractfile(member) if member.isfile() else None
                        member.name = name + '/' + member.name
                        archive.addfile(member, data)
            for item in ['apps', 'third_party', 'scripts', 'session', 'config', 'packaging',
                         'tests', 'install.sh', 'VERSION', 'sources.lock.json', 'README.md', 'THIRD_PARTY.md', 'LICENSE']:
                archive.add(ROOT / item, arcname='emwm-desktop/' + item,
                            filter=lambda member: None if '__pycache__' in member.name.split('/') else member)
        # Determine ABI dependencies from the actual build, including Debian t64 names.
        debian = work / 'debian'
        debian.mkdir(exist_ok=True)
        (debian / 'control').write_text('Source: emwm-desktop\nSection: x11\nPriority: optional\n'
            'Maintainer: pjdgilmour <paulogilmour@hotmail.com>\nStandards-Version: 4.7.0\n\n'
            'Package: emwm-desktop\nArchitecture: any\nDescription: Classic Motif desktop\n')
        elf = [bin_dir / name for name in bins if name != 'xmsession']
        deps = run(['dpkg-shlibdeps', '-O', *['-e' + str(path) for path in elf]], cwd=work, capture=True)
        abi = next(line.split('=', 1)[1] for line in deps.splitlines() if line.startswith('shlibs:Depends='))
        arch = run(['dpkg', '--print-architecture'], capture=True).strip()
        control = stage / 'DEBIAN'
        control.mkdir()
        size = sum(p.stat().st_size for p in stage.rglob('*') if p.is_file()) // 1024
        (control / 'control').write_text(
            f'Package: emwm-desktop\nVersion: {version}\nArchitecture: {arch}\nSection: x11\nPriority: optional\n'
            f'Maintainer: pjdgilmour <paulogilmour@hotmail.com>\nInstalled-Size: {size}\n'
            f'Depends: {abi}, ' + ', '.join(RUNTIME) + '\n'
            'Recommends: lightdm | x-display-manager, network-manager-gnome, pipewire-audio, pavucontrol\n'
            'Homepage: https://github.com/pjdgilmour/emwm-desktop\n'
            'Description: Classic Motif desktop with XDG menus and system tray\n'
            ' Enhanced Motif Window Manager, Toolchest, xmtray and persistent wallpapers.\n'
            ' Installed as a separate EMWM Desktop session with private executables.\n')
        package = dist / f'emwm-desktop_{version}_{arch}.deb'
        # Package permissions must not depend on the developer's umask.
        stage.chmod(0o755)
        for directory in stage.rglob('*'):
            if directory.is_dir():
                directory.chmod(0o755)
        run(['dpkg-deb', '--root-owner-group', '--build', stage, package])
        checksum = hashlib.sha256(package.read_bytes()).hexdigest()
        package.with_suffix('.deb.sha256').write_text(f'{checksum}  {package.name}\n')
        run(['dpkg-deb', '--info', package])
        return package


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--build-only', action='store_true', help='Compilar o .deb sem apt, sudo ou instalação.')
    parser.add_argument('--dependencies-only', action='store_true', help='Instalar apenas dependências de compilação.')
    parser.add_argument('--sources', type=Path, help='Usar commits dos repositórios locais DIR/emwm e DIR/emwm-utils.')
    parser.add_argument('--test', action='store_true', help='Executar também testes X11/ARGB em Xvfb.')
    parser.add_argument('--yes', action='store_true', help='Passar -y ao apt-get.')
    parser.add_argument('--jobs', type=int, default=min(os.cpu_count() or 1, 4))
    args = parser.parse_args()
    if args.jobs < 1 or args.jobs > 128:
        parser.error('--jobs deve estar entre 1 e 128')
    if args.build_only and args.dependencies_only:
        parser.error('--build-only e --dependencies-only são mutuamente exclusivos')
    if args.sources:
        args.sources = args.sources.resolve()
    debian_version()
    if os.geteuid() == 0:
        raise RuntimeError('Execute como usuário normal. O instalador usa sudo somente para apt.')
    if not args.build_only:
        yes = ['-y'] if args.yes else []
        run(['sudo', 'apt-get', 'update'])
        run(['sudo', 'apt-get', 'install', *yes, *BUILD_DEPS, *(TEST_DEPS if args.test else [])])
    if args.dependencies_only:
        return
    package = build(args)
    if args.test:
        run([sys.executable, '-m', 'unittest', 'discover', '-s', ROOT / 'tests', '-v'])
        run(['sh', ROOT / 'tests/smoke-session.sh', package])
    if not args.build_only:
        run(['sudo', 'apt-get', 'install', *(['-y'] if args.yes else []), package])
        print('\nInstalado. Selecione EMWM Desktop na tela de login. A sessão atual foi preservada.')
    else:
        print(f'\nPacote gerado, sem instalação: {package}')


if __name__ == '__main__':
    try:
        main()
    except (RuntimeError, OSError, subprocess.CalledProcessError) as error:
        print(f'Erro: {error}', file=sys.stderr)
        sys.exit(1)
