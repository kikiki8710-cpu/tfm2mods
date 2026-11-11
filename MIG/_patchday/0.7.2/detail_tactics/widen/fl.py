# usage: python -X utf8 fl.py <ret_rva>...  -> owner 경계 찾아 call 명령(ret-5) 시점 flow 출력 (A? 라벨만)
import sys,subprocess
sys.path.insert(0, r'F:\claude\tfm2mods\MIG'); import aidiff as A
N=A.Img(r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe')
T=r'F:\claude\tfm2mods\MIG\_patchday\0.7.2\detail_tactics\re\tools\flow.py'
for a in sys.argv[1:]:
    site=int(a[1:],16) if a.startswith("w") else int(a,16)-5
    f=N.owner(site)
    print('#### ret %s  call@%x  owner %x-%x'%(a,site,f[0],f[1]),flush=True)
    out=subprocess.run([sys.executable,'-X','utf8',T,'new',hex(f[0]),hex(f[1]),hex(site)],capture_output=True,text=True,encoding='utf-8').stdout
    for l in out.splitlines():
        if '==' in l or 'A' in l.split('<-')[-1] or 'UNREACH' in l: print(l)
