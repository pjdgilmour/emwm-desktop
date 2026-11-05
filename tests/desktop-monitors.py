#!/usr/bin/python3
# SPDX-License-Identifier: GPL-3.0-only
"""Two simulated GDK monitors on a real private X server, desktop and file manager."""
import argparse
import configparser
import ctypes
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
from PIL import Image
ROOT = Path(__file__).resolve().parents[1]


def run(args, work):
    processes, logs = [], []
    def launch(command, env=None, **kwargs):
        log = (work / (Path(command[0]).name + '.log')).open('a'); logs.append(log)
        proc = subprocess.Popen(command, env=env, stdout=log, stderr=log, **kwargs)
        processes.append(proc)
        return proc
    try:
        shim = work / 'monitors.so'
        flags = subprocess.check_output(['pkg-config', '--cflags', '--libs', 'gtk+-3.0'], text=True).split()
        subprocess.run(['cc', '-shared', '-fPIC', str(ROOT/'tests/desktop-monitors-shim.c'), '-o', str(shim), *flags], check=True)
        with (work/'display').open('w') as stream:
            launch(['Xvfb', '-displayfd', str(stream.fileno()), '-screen', '0', '1600x600x24', '-nolisten', 'tcp', '-noreset'], pass_fds=(stream.fileno(),))
        for _ in range(100):
            if (work/'display').read_text().strip(): break
            time.sleep(.05)
        home, runtime = work/'home', work/'runtime'
        (home/'.config').mkdir(parents=True); runtime.mkdir(mode=0o700)
        env = dict(os.environ, HOME=str(home), DISPLAY=':'+(work/'display').read_text().strip(),
                   XDG_CONFIG_HOME=str(home/'.config'), XDG_DATA_HOME=str(home/'.local/share'),
                   XDG_STATE_HOME=str(home/'.local/state'), XDG_RUNTIME_DIR=str(runtime),
                   GVFS_DISABLE_FUSE='1', NO_AT_BRIDGE='1', GTK_THEME='Adwaita', GSETTINGS_BACKEND='memory')
        for key in ('SESSION_MANAGER','XAPPLRESDIR','XENVIRONMENT'): env.pop(key, None)
        desktop = home/'Desktop'; desktop.mkdir(); (desktop/'Test.txt').write_text('fixture\n')
        (home/'.config/user-dirs.dirs').write_text('XDG_DESKTOP_DIR="$HOME/Desktop"\n')
        profile = home/'.config/pcmanfm/emwm-desktop'; profile.mkdir(parents=True)
        for i in range(2):
            (profile/f'desktop-items-{i}.conf').write_text('[*]\nwallpaper_mode=color\ndesktop_bg=#516286\nshow_trash=1\nshow_documents=0\nshow_mounts=0\n')
        (home/'.local/share/Trash/files').mkdir(parents=True); (home/'.local/share/Trash/info').mkdir()
        prefs = home/'.config/emwm-desktop/desktop.conf'; prefs.parent.mkdir()
        topology = work/'topology'; topology.write_text('2 1')
        subprocess.run(['dbus-update-activation-environment','HOME','DISPLAY','XDG_CONFIG_HOME','XDG_DATA_HOME','XDG_STATE_HOME','XDG_RUNTIME_DIR','GVFS_DISABLE_FUSE'],env=env,check=True)
        launch(['/usr/libexec/gvfsd','--no-fuse'],env)
        wm_command=[str(args.wm.resolve()),'-xrm','Emwm*interactivePlacement: False',
                    '-xrm','Emwm*clientAutoPlace: False','-xrm','Emwm*showFeedback: none']
        wm=launch(wm_command,env)
        time.sleep(.4)
        if args.compositor:
            launch([str(args.compositor.resolve()),'--backend',args.backend],env)
            time.sleep(.4)
        fm = launch([str(ROOT/'apps/emwm-desktop-icons/emwm-desktop-icons')],
                    dict(env,LD_PRELOAD=str(shim),EMWM_TEST_TOPOLOGY=str(topology),
                         PATH=str(args.file_manager.resolve().parent)+os.pathsep+env['PATH']))
        def shot():
            path=work/'screen.png'
            subprocess.run(['import','-window','root',str(path)],env=env,check=True,timeout=5)
            with Image.open(path) as img: return img.convert('RGB')
        def whites(img, box): return sum(min(c)>190 for c in img.crop(box).getdata())
        def check_icon(x,y, empty, label):
            deadline=time.monotonic()+4
            while time.monotonic()<deadline:
                img=shot()
                if whites(img,(x-18,y-18,x+18,y+26))>200 and whites(img,empty)<50:
                    assert fm.poll() is None
                    print('PASS:',label,flush=True); return
                time.sleep(.08)
            img.save(work/(label.replace(' ','-')+'.png'))
            raise AssertionError(label+': missing icon or duplicated icons')
        def set_pref(monitor,anchor):
            temporary=prefs.with_suffix('.tmp')
            temporary.write_text(f'[desktop]\nmonitor={monitor}\nanchor={anchor}\n')
            temporary.replace(prefs)
        # No settings: use the second monitor, the simulated system primary.
        check_icon(860,130,(0,0,800,600),'default primary')
        set_pref('primary','right')
        check_icon(1540,130,(0,0,800,600),'right anchor')
        # XTest DnD, so fixed positions use the same path as real interaction.
        x=ctypes.CDLL('libX11.so.6'); xt=ctypes.CDLL('libXtst.so.6')
        x.XOpenDisplay.argtypes=[ctypes.c_char_p]; x.XOpenDisplay.restype=ctypes.c_void_p
        x.XFlush.argtypes=[ctypes.c_void_p]; x.XCloseDisplay.argtypes=[ctypes.c_void_p]
        xt.XTestFakeMotionEvent.argtypes=[ctypes.c_void_p,ctypes.c_int,ctypes.c_int,ctypes.c_int,ctypes.c_ulong]
        xt.XTestFakeButtonEvent.argtypes=[ctypes.c_void_p,ctypes.c_uint,ctypes.c_int,ctypes.c_ulong]
        display=x.XOpenDisplay(env['DISPLAY'].encode()); assert display
        def move(a,b): xt.XTestFakeMotionEvent(display,-1,a,b,0); x.XFlush(display)
        def button(value): xt.XTestFakeButtonEvent(display,1,value,0); x.XFlush(display)
        try:
            move(1540,130); time.sleep(.1); button(1); time.sleep(.1)
            for i in range(1,26):
                move(round(1540+(1370-1540)*i/25),round(130+(300-130)*i/25)); time.sleep(.015)
            button(0)
            check_icon(1370,300,(0,0,800,600),'right drag')
        finally: x.XCloseDisplay(display)
        time.sleep(.4)
        old=(profile/'desktop-items-1.conf').read_text()
        assert '[Test.txt]' in old, old
        set_pref('primary','left')
        check_icon(1030,300,(0,0,800,600),'mirror manual position')
        set_pref('TEST-A','left')
        check_icon(60,130,(800,0,1600,600),'explicit monitor A')
        # Updating wallpaper must not discard the inactive monitor's positions.
        subprocess.run([str(args.file_manager.resolve()),'--profile=emwm-desktop','--wallpaper-mode=color'],env=env,check=True)
        time.sleep(.4)
        saved=(profile/'desktop-items-1.conf').read_text()
        assert '[Test.txt]' in saved, saved
        set_pref('TEST-B','right')
        check_icon(1370,300,(0,0,800,600),'restore manual position')
        topology.write_text('1 0')
        check_icon(740,130,(800,0,1600,600),'disconnect fallback')
        topology.write_text('2 1')
        check_icon(1370,300,(0,0,800,600),'reconnect chosen monitor')
        set_pref('primary','right')
        topology.write_text('2 0')
        check_icon(740,130,(800,0,1600,600),'system primary change')
        assert (desktop/'Test.txt').read_text()=='fixture\n'
        assert fm.poll() is None, 'file manager restarted or crashed'
        print('PASS: one icon set, live settings, mirrored DnD positions, wallpaper preservation, primary change and hotplug',flush=True)
        # Different images per monitor, composed as the wallpaper tool does.
        colors=[((170,40,60),(20,150,90)),((30,60,160),(190,140,20))]
        for i,(left,right) in enumerate(colors):
            img=Image.new('RGBA',(1600,600),left)
            img.paste(right,(800,0,1600,600));img.save(home/f'wallpaper-{i}.png')
        subprocess.run(['xrdb','-merge'],input=''.join(
            f'Emwm*ws{i}*backdrop*image: "{home}/wallpaper-{i}.png"\n' for i in range(2)),
            text=True,env=env,check=True)
        def check_background(index,label):
            deadline=time.monotonic()+5
            while time.monotonic()<deadline:
                img=shot()
                if (img.getpixel((500,500)),img.getpixel((1300,500)))==colors[index]:
                    print('PASS:',label,flush=True);return
                time.sleep(.1)
            raise AssertionError(f'{label}: {img.getpixel((500,500))}, {img.getpixel((1300,500))}')
        for i in (0,1,0):
            subprocess.run([str(args.tellmwm.resolve()),'setws',f'ws{i}'],env=env,check=True)
            check_background(i,f'per-monitor wallpaper workspace {i}')
        # Rebuild the desktop surfaces after changing their icon monitor.
        def fixed_positions():
            result={}
            for path in profile.glob('desktop-items-*.conf'):
                parser=configparser.RawConfigParser();parser.read(path)
                result[path.name]={key:dict(parser[key]) for key in parser.sections() if key!='*'}
            return result
        positions=fixed_positions()
        set_pref('TEST-B','right')
        time.sleep(.7)
        check_background(0,'wallpaper after desktop rebuild')
        assert fixed_positions()==positions, 'wallpaper refresh discarded manual icon positions'
        topology.write_text('1 0')
        time.sleep(.6)
        (profile/'desktop-items-1.conf').unlink()
        topology.write_text('2 0')
        time.sleep(.6)
        check_background(0,'wallpaper on a monitor without saved profile')
        wm.terminate();wm.wait(timeout=5)
        wm=launch(wm_command,env)
        time.sleep(.8)
        check_background(0,'wallpaper after WM restart')
        # Removing PCManFM restores the WM's configured native backdrop.
        fm.terminate();fm.wait(timeout=5)
        time.sleep(.5)
        check_background(0,'native wallpaper fallback without PCManFM')
    finally:
        for proc in reversed(processes):
            if proc.poll() is None:
                proc.terminate()
                try: proc.wait(timeout=4)
                except subprocess.TimeoutExpired: proc.kill(); proc.wait()
        for log in logs: log.close()
        if sys.exc_info()[0]:
            for log in work.glob('*.log'): print(log.name+':\n'+log.read_text(errors='replace')[-3000:])


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--wm',required=True,type=Path)
    parser.add_argument('--file-manager',required=True,type=Path)
    parser.add_argument('--tellmwm',type=Path,default=Path('/usr/lib/emwm-desktop/bin/tellmwm'))
    parser.add_argument('--compositor',type=Path)
    parser.add_argument('--backend',choices=('glx','xrender'),default='glx')
    parser.add_argument('--private-bus',action='store_true')
    args=parser.parse_args()
    if not args.private_bus:
        subprocess.run(['dbus-run-session','--','/usr/bin/python3',__file__,'--private-bus',*sys.argv[1:]],check=True)
    else:
        with tempfile.TemporaryDirectory(prefix='emwm-monitors-') as temp: run(args,Path(temp))

if __name__=='__main__': main()
