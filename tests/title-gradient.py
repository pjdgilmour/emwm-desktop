#!/usr/bin/python3
# SPDX-License-Identifier: GPL-3.0-only
"""Pixel regression for native title gradients on private X11 displays."""
import argparse,os,subprocess,tempfile,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]

def run(args,work,backend,decoupled):
    processes=[];logs=[]
    home=work/'home';home.mkdir()
    def launch(command,env=None,**kwargs):
        stream=(work/(Path(command[0]).name+'.log')).open('a');logs.append(stream)
        p=subprocess.Popen(command,env=env,stdout=stream,stderr=stream,**kwargs);processes.append(p);return p
    try:
        with (work/'display').open('w') as stream:
            launch(['Xvfb','-displayfd',str(stream.fileno()),'-screen','0','1024x768x24','-nolisten','tcp','-noreset'],pass_fds=(stream.fileno(),))
        for _ in range(100):
            if (work/'display').read_text().strip():break
            time.sleep(.05)
        env=dict(os.environ,HOME=str(home),DISPLAY=':'+(work/'display').read_text().strip(),XDG_CONFIG_HOME=str(home/'.config'),XDG_STATE_HOME=str(home/'.local/state'))
        for key in ('SESSION_MANAGER','XENVIRONMENT','XAPPLRESDIR'):env.pop(key,None)
        resources='Emwm*interactivePlacement: False\nEmwm*clientAutoPlace: False\nEmwm*focusAutoRaise: False\nEmwm*cleanText: True\n'
        if decoupled: resources+='Emwm*client*title*background: #113355\n'
        subprocess.run(['xrdb','-merge'],input=resources,text=True,env=env,check=True)
        wm=launch([str(args.wm.resolve())],env);time.sleep(.5)
        if backend!='none':launch([str(args.compositor.resolve()),'--backend',backend],env);time.sleep(.5)
        for visual in ('rgb','argb'):
            if visual=='argb' and backend=='none':continue
            subprocess.run([str(args.probe),visual],env=env,check=True,timeout=20)
        assert wm.poll() is None
        print(f'PASS: {backend}, separate title appearance={decoupled}',flush=True)
    except Exception:
        for p in work.glob('*.log'):print(p.name,p.read_text()[-2500:])
        raise
    finally:
        for p in reversed(processes):
            if p.poll() is None:
                p.terminate()
                try:p.wait(timeout=3)
                except subprocess.TimeoutExpired:p.kill();p.wait()
        for s in logs:s.close()

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--wm',type=Path,required=True);parser.add_argument('--compositor',type=Path,required=True)
    args=parser.parse_args()
    with tempfile.TemporaryDirectory(prefix='emwm-gradient-') as temp:
        work=Path(temp);args.probe=work/'probe'
        subprocess.run(['cc','-O2','-Wall',str(ROOT/'tests/title-gradient.c'),'-o',str(args.probe),'-lX11','-lXrender'],check=True)
        for backend in ('none','xrender','glx'):
            for decoupled in (False,True):
                directory=work/f'{backend}-{decoupled}';directory.mkdir();run(args,directory,backend,decoupled)
if __name__=='__main__':main()
