# -*- coding: utf-8 -*-
u"""크레이트의 pub(선언 가시성) API 표면 추출. 사용: python -X utf8 tcxpub.py <crate> > out.txt"""
import json,io,sys
from collections import Counter,defaultdict
sys.stdout.reconfigure(encoding='utf-8',errors='replace')
cr=sys.argv[1]
d=json.load(io.open(r'C:\tfm2mods\MIG\_tcx\%s.json'%cr,encoding='utf-8'))
c=Counter(); mods=defaultdict(Counter); rows=[]; pubmods=[]
for it in d['items']:
    if it['k']=='Mod' and it['v']=='pub': pubmods.append(it['p'])
    if it['v']!='pub': continue
    if it['k'] in ('Use','ExternCrate','LifetimeParam','TyParam','AnonConst','Field'): continue
    if '::_::' in it['p'] or '__Visitor' in it['p'] or '__Field' in it['p']: continue
    c[it['k']]+=1; mods['::'.join(it['p'].split('::')[:3])][it['k']]+=1; rows.append(it)
print("== %s pub 아이템 종류별 =="%cr)
for k,v in c.most_common(): print("  %-30s %d"%(k,v))
print("\n== pub 모듈 %d개 =="%len(pubmods))
for m in pubmods: print("   "+m)
print("\n== pub Struct/Enum/Trait/TyAlias ==")
for it in rows:
    if it['k'] in ('Struct','Enum','Union','Trait','TyAlias'):
        print("  %-8s %-72s %s:%s"%(it['k'],it['p'],it['sp']['f'],it['sp']['l']))
print("\n== pub 자유함수(Fn) ==")
for it in rows:
    if it['k']=='Fn': print("  %-74s %s"%(it['p'],(it.get('sig') or '')[:160]))
print("\n== pub const/static ==")
for it in rows:
    if it['k'].startswith('Const') or it['k'].startswith('Static'): print("  %s"%it['p'])
