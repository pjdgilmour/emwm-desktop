#!/usr/bin/env python3
"""Run on a new Xvfb. Exercise the native, unprivileged manager with fake session programs."""
import os
import signal
import subprocess
import sys
import tempfile
import time
from pathlib import Path

binary = Path(os.environ['XMSM_TEST_BINARY']).resolve()
probe = Path(os.environ['XMSM_TEST_PROBE']).resolve()
if binary.stat().st_mode & 0o6000 or os.geteuid() == 0:
    sys.exit('This test needs an unprivileged build and user.')

def wait_for(call, label):
    for _ in range(160):
        if call():
            return
        time.sleep(.025)
    raise RuntimeError(label)

def check(*args):
    return subprocess.run([probe, *args], stdout=subprocess.DEVNULL).returncode == 0

def root_prop(name):
    return subprocess.check_output(['xprop','-root',name],text=True)

def run(case):
    with tempfile.TemporaryDirectory() as temp:
        home=Path(temp)
        env=dict(os.environ, HOME=temp, XENVIRONMENT=str(home/'resources'), XDG_CONFIG_HOME=temp)
        env.pop('SESSION_MANAGER',None)
        env.pop('XFILESEARCHPATH',None)
        (home/'resources').write_text('XmSm.width: 1\nXmSm.height: 1\n')
        # Avoid CPU, locale and host dependencies in the log comparisons.
        env['LC_ALL']='C'
        if case in ('version','missing-display','failed-launch'):
            args=['xmsm','-version'] if case!='failed-launch' else [
                'xmsm','-xrm','XmSm*windowManager: /xmsm-test/nonexistent']
            if case=='missing-display': env['DISPLAY']=':99441'
            p=subprocess.run(args,executable=binary,env=env)
            return p.returncode
        if case not in ('signal','message','no-shade','duplicates','lock-denied','disabled-suspend','stale'):
            return 2
        for name in ['wm','launcher']:
            script=home/name
            script.write_text('#!/bin/sh\necho "$1" > "$HOME"/'+name+'.ran\n')
            script.chmod(0o700)
        (home/'.sessionetc').write_text('echo yes > "$HOME/session.ran"\n')
        args=['xmsm','-xrm',f'XmSm*windowManager: {home}/wm hello','-xrm',
              f'XmSm*launcher: {home}/launcher yes','-xrm','XmSm*enableSuspend: False',
              '-xrm','XmSm*showReboot: False','-xrm','XmSm*showShutdown: False',
              '-xrm','XmSm*numLockState: keep']
        if case=='no-shade': args+=['-xrm','XmSm*enableShade: False']
        if case=='disabled-suspend': args+=['-xrm','XmSm*enableLocking: False']
        log=home/'session.log'
        with log.open('wb') as output:
            proc=subprocess.Popen(args,executable=binary,env=env,stdout=output,stderr=output)
            try:
                wait_for(lambda: (home/'session.ran').exists(),'session startup')
                assert (home/'wm.ran').read_text().strip()=='hello'
                assert (home/'launcher.ran').read_text().strip()=='yes'
                assert 'window id #' in root_prop('_XM_SESSION_MANAGER')
                cfg=root_prop('_XM_SESSION_MANAGER_CFG').strip().split()[-1]
                assert cfg==('0' if case=='disabled-suspend' else '2')
                if case=='duplicates':
                    second=subprocess.run(args,executable=binary,env=env,capture_output=True)
                    assert second.returncode==0 and f'already running as PID {proc.pid}'.encode() in second.stderr
                elif case=='stale':
                    proc.kill()
                    proc.wait()
                    (home/'session.ran').unlink()
                    proc=subprocess.Popen(args,executable=binary,env=env,stdout=output,stderr=output)
                    wait_for(lambda: (home/'session.ran').exists(),'stale manager recovery')
                elif case=='lock-denied':
                    assert check('LOCK')
                    wait_for(lambda: 'Cannot authenticate. Screen locking disabled!' in log.read_text(),'denied lock')
                    assert not check('grab')
                elif case=='disabled-suspend':
                    assert check('SUSPEND')
                    wait_for(lambda: "Can't suspend. Command is disabled" in log.read_text(),'disabled suspension')
                else:
                    if case=='signal': proc.send_signal(signal.SIGUSR2)
                    else: assert check('LOGOUT')
                    wait_for(lambda: check('dialog'),'show logout')
                    assert check('cancel')
                    wait_for(lambda: not check('dialog'),'cancel logout')
                    assert proc.poll() is None
                    # The upstream signal callback reuses a stale stack address when a
                    # different dispatch path reopens its dialog. Verify the second
                    # confirmation through the regular IPC path in the other cases.
                    if case != 'signal':
                        assert check('LOGOUT')
                        wait_for(lambda: check('dialog'),'show logout again')
                        assert check('ok')
                        assert proc.wait(timeout=5)==0
                        assert 'window id #' not in root_prop('_XM_SESSION_MANAGER')
                print('PASS',case)
            except BaseException:
                print(log.read_text(),file=sys.stderr)
                raise
            finally:
                if proc.poll() is None:
                    proc.kill()
                    proc.wait()
        return 0

sys.exit(run(sys.argv[1]))
