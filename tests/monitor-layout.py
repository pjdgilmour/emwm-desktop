#!/usr/bin/python3
# SPDX-License-Identifier: GPL-3.0-only
"""Regression: recomposition and Toolchest recovery with a missing left output."""
import argparse,ctypes,os,re,subprocess,tempfile,time
from pathlib import Path
from PIL import Image
ROOT=Path(__file__).resolve().parents[1]
def main():
    ap=argparse.ArgumentParser();ap.add_argument('--wm',required=True,type=Path);ap.add_argument('--wallpaper',required=True,type=Path);ap.add_argument('--toolbox',required=True,type=Path)
    args=ap.parse_args();processes=[];logs=[]
    with tempfile.TemporaryDirectory(prefix='emwm-layout-') as temp:
        work=Path(temp);home=work/'home';home.mkdir();topology=work/'topology';topology.write_text('2 1')
        def launch(cmd,env=None,**kw):
            log=(work/(Path(cmd[0]).name+'.log')).open('a');logs.append(log)
            p=subprocess.Popen(cmd,env=env,stdout=log,stderr=log,**kw);processes.append(p);return p
        try:
            shim=work/'randr.so'
            subprocess.run(['cc','-shared','-fPIC',str(ROOT/'tests/randr-topology-shim.c'),'-o',str(shim)],check=True)
            with (work/'display').open('w') as f:launch(['Xvfb','-displayfd',str(f.fileno()),'-screen','0','1600x600x24','-nolisten','tcp','-noreset'],pass_fds=(f.fileno(),))
            for _ in range(100):
                if (work/'display').read_text().strip():break
                time.sleep(.05)
            env=dict(os.environ,HOME=str(home),DISPLAY=':'+(work/'display').read_text().strip(),
                     XDG_CONFIG_HOME=str(home/'.config'),XDG_STATE_HOME=str(home/'.local/state'),
                     EMWM_TEST_TOPOLOGY=str(topology),LD_PRELOAD=str(shim))
            for k in ('SESSION_MANAGER','XAPPLRESDIR','XENVIRONMENT'):env.pop(k,None)
            launch([str(args.wm.resolve()),'-xrm','Emwm*interactivePlacement: False','-xrm','Emwm*clientAutoPlace: False'],env);time.sleep(.4)
            folder=home/'.config/emwm';folder.mkdir(parents=True)
            for name,color in [('A','red'),('B','green')]:Image.new('RGB',(800,600),color).save(home/f'{name}.png')
            resources=folder/'backdrops.resources'
            resources.write_text(f'''Emwm*ws0*backdrop*image: "{home}/A.png"
EmwmWallpaperTool.ws0.source: "{home}/A.png"
EmwmWallpaperTool.ws0.mode: center
EmwmWallpaperTool.ws0.color: #000000
EmwmWallpaperTool.ws0.monitorCount: 2
EmwmWallpaperTool.ws0.monitor0.name: "TEST-A"
EmwmWallpaperTool.ws0.monitor0.source: "{home}/A.png"
EmwmWallpaperTool.ws0.monitor0.mode: center
EmwmWallpaperTool.ws0.monitor0.color: #000000
EmwmWallpaperTool.ws0.monitor0.rotation: auto
EmwmWallpaperTool.ws0.monitor1.name: "TEST-B"
EmwmWallpaperTool.ws0.monitor1.source: "{home}/B.png"
EmwmWallpaperTool.ws0.monitor1.mode: center
EmwmWallpaperTool.ws0.monitor1.color: #000000
EmwmWallpaperTool.ws0.monitor1.rotation: auto
''')
            for topology_value,expected in [('2 1',('red','green')),('1 0 1 800',('black','green')),('1 0 1 0',('green','black')),('2 1',('red','green'))]:
                topology.write_text(topology_value)
                subprocess.run([str(args.wallpaper.resolve()),'--refresh-layout'],env=env,check=True,timeout=10)
                content=resources.read_text();path=re.search(r'^Emwm\*ws0\*backdrop\*image: "(.*)"',content,re.M)[1]
                with Image.open(path) as img:
                    from PIL import ImageColor
                    assert tuple(img.convert('RGB').getpixel((x,300)) for x in (400,1200))==tuple(ImageColor.getrgb(c) for c in expected)
                assert 'TEST-A' in content and 'TEST-B' in content
            before=resources.stat().st_mtime_ns
            subprocess.run([str(args.wallpaper.resolve()),'--refresh-layout'],env=env,check=True,timeout=10)
            assert resources.stat().st_mtime_ns==before,'unchanged layout rewrites preferences'
            print('PASS: wallpaper recomposition, offset origin, reconnect and saved output choices',flush=True)
            topology.write_text('1 0 1 800')
            for name,fmt in [('_XM_SESSION_MANAGER','32c'),('_XM_SESSION_MANAGER_CFG','32i')]:
                subprocess.run(['xprop','-root','-f',name,fmt,'-set',name,'0'],env=env,check=True)
            toolbox=launch([str(args.toolbox.resolve()),'-geometry','+30+40'],env)
            def bounds():
                tree=subprocess.check_output(['xwininfo','-root','-tree'],env=env,text=True)
                ids=re.findall(r'(0x[0-9a-f]+) "[^"\n]+": \("xmtoolbox" "XmToolbox"\)\s+(\d+)x(\d+)',tree)
                for wid,w,h in ids:
                    if int(w)<10:continue
                    info=subprocess.check_output(['xwininfo','-id',wid],env=env,text=True)
                    x=int(re.search(r'Absolute upper-left X:\s+(-?\d+)',info)[1]);y=int(re.search(r'Absolute upper-left Y:\s+(-?\d+)',info)[1])
                    return int(wid,16),x,y,int(w),int(h)
                raise AssertionError(tree)
            time.sleep(1.5)
            wid,x,y,w,h=bounds();assert 800<=x and x+w<=1600 and 0<=y and y+h<=600,(x,y,w,h)
            # Simulate the RandR event delivered when the surviving output moves.
            topology.write_text('1 0 1 0')
            lib=ctypes.CDLL('libX11.so.6');rr=ctypes.CDLL('libXrandr.so.2')
            lib.XOpenDisplay.restype=ctypes.c_void_p;lib.XOpenDisplay.argtypes=[ctypes.c_char_p]
            d=lib.XOpenDisplay(env['DISPLAY'].encode());event=ctypes.c_int();error=ctypes.c_int()
            rr.XRRQueryExtension.argtypes=[ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p];rr.XRRQueryExtension(d,ctypes.byref(event),ctypes.byref(error))
            data=ctypes.create_string_buffer(192);ctypes.c_int.from_buffer(data).value=event.value+1
            lib.XSendEvent.argtypes=[ctypes.c_void_p,ctypes.c_ulong,ctypes.c_int,ctypes.c_long,ctypes.c_void_p]
            lib.XFlush.argtypes=[ctypes.c_void_p];lib.XCloseDisplay.argtypes=[ctypes.c_void_p]
            lib.XSendEvent(d,wid,False,0,data);lib.XFlush(d);lib.XCloseDisplay(d)
            time.sleep(.8)
            _,x,y,w,h=bounds();assert 0<=x and x+w<=800 and 0<=y and y+h<=600,(x,y,w,h)
            assert toolbox.poll() is None
            print('PASS: Toolchest visible at login and after RandR change',flush=True)
        except Exception:
            for p in work.glob('*.log'):print(p.name,p.read_text()[-2000:])
            raise
        finally:
            for p in reversed(processes):
                if p.poll() is None:
                    p.terminate()
                    try:p.wait(timeout=3)
                    except subprocess.TimeoutExpired:p.kill();p.wait()
            for log in logs:log.close()
if __name__=='__main__':main()
