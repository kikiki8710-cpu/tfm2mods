# 0.6.2 릴리스 zip 생성: 0.6.1 zip 의 엔트리 구성을 그대로 따르고, 지정 모드 폴더의 파일은 게임 배포본으로 교체.
#   사용: _rel062.py <zip이름> <교체할 모드ID,...> [--dry]
import zipfile,sys,os,hashlib
G=r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods'
SRC=r'C:\tfm2mods\release\0.6.1'; DST=r'C:\tfm2mods\release\0.6.2'
name=sys.argv[1]; repl=set(sys.argv[2].split(',')); dry='--dry' in sys.argv
zin=zipfile.ZipFile(os.path.join(SRC,name+'.zip'))
out=None if dry else zipfile.ZipFile(os.path.join(DST,name+'.zip'),'w',zipfile.ZIP_DEFLATED)
for zi in zin.infolist():
    top=zi.filename.split('/')[0]; data=zin.read(zi)
    tag='keep'
    if top in repl and not zi.is_dir():
        gp=os.path.join(G,*zi.filename.split('/'))
        if os.path.isfile(gp):
            g=open(gp,'rb').read()
            if g!=data: tag='REPLACED' ; data=g
        else: tag='MISSING-in-game(keep)'
    if tag!='keep' or dry: print('%-10s %s'%(tag,zi.filename))
    if out:
        ni=zipfile.ZipInfo(zi.filename,date_time=zi.date_time if tag=='keep' else __import__('time').localtime()[:6])
        ni.compress_type=zipfile.ZIP_DEFLATED; ni.external_attr=zi.external_attr
        out.writestr(ni,data)
if out: out.close(); print('->',os.path.join(DST,name+'.zip'),os.path.getsize(os.path.join(DST,name+'.zip')),'B',len(zin.infolist()),'entries')
