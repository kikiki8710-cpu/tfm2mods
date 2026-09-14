# -*- coding: utf-8 -*-
"""pfscan.py — SmallActionPlay::get_input 의 16 콜리(변종별 get_input) 안에서 Option<PathFinder>(72B, Box 1120B/70B)
drop(__rust_dealloc 1120/70) · 재생성(PathFinder::new_target_with_policy / update_path / sret 72B) 지점 전수.
출력: 변종 · 파일:줄 · 명령 · !dbg 루트 줄(inlinedAt 사슬)."""
import io, os, re, sys, glob
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
IR = r'C:\tfm2mods\_gaibc'
CALLEES = [
 ('RunAway', '18SmallActionRunAway9get_input('),
 ('Recall', '17SmallActionRecall9get_input('),
 ('Around', '17SmallActionAround9get_input('),
 ('AroundHide', '21SmallActionAroundHide9get_input('),
 ('AroundRegion', '23SmallActionAroundRegion9get_input('),
 ('Positioning', '22SmallActionPositioning9get_input('),
 ('AroundPosition', '25SmallActionAroundPosition9get_input('),
 ('AroundPositionBush', '29SmallActionAroundPositionBush9get_input('),
 ('AroundBush', '21SmallActionAroundBush9get_input('),
 ('LaneMinionPosition', '29SmallActionLaneMinionPosition9get_input('),
 ('Trace', '16SmallActionTrace9get_input('),
 ('Attack', '17SmallActionAttack9get_input('),
 ('Skill', '16SmallActionSkill9get_input('),
 ('Skill2', '17SmallActionSkill29get_input('),
 ('Ult', '14SmallActionUlt9get_input('),
]
files = {}
for p in sorted(glob.glob(IR + r'\*.ll')):
    files[os.path.basename(p)] = io.open(p, encoding='utf-8', errors='replace').read().split('\n')
mdcache = {}
def md(fn):
    if fn not in mdcache:
        d = {}
        for l in files[fn]:
            if l.startswith('!') and ' = ' in l:
                d[l.split(' = ', 1)[0]] = l
        mdcache[fn] = d
    return mdcache[fn]
def loc(fn, id_):
    m = md(fn); k = '!' + id_; out = []
    for _ in range(30):
        s = m.get(k)
        if not s: break
        ml = re.search(r'line: (\d+)', s); ia = re.search(r'inlinedAt: (![0-9]+)', s)
        fl = ''
        sc = re.search(r'scope: (![0-9]+)', s)
        out.append(ml.group(1) if ml else '?')
        if not ia: break
        k = ia.group(1)
    return '<'.join(out)
PAT = re.compile(r'__rust_dealloc\(ptr noundef nonnull (%\d+), i64 noundef (1120|70),|__rust_alloc\(i64 noundef (1120|70),|exchange_malloc|10PathFinder(\d+\w+?)(?:NC|B|\()|sret\(\[72 x i8\]\)|memcpy\.p0\.p0\.i64\(ptr[^,]*, ptr[^,]*, i64 72,')
for name, frag in CALLEES:
    hit = None
    for fn, L in files.items():
        for i, l in enumerate(L):
            if l.startswith('define') and frag in l:
                hit = (fn, i); break
        if hit: break
    if not hit:
        print('##', name, 'define 없음(declare only?)'); continue
    fn, s = hit
    e = s
    while not files[fn][e].startswith('}'): e += 1
    print('##', name, '%s:%d~%d' % (fn, s + 1, e + 1))
    for i in range(s, e + 1):
        l = files[fn][i]
        if PAT.search(l):
            d = re.search(r'!dbg !(\d+)', l)
            short = re.sub(r'@_R\w*?(10PathFinder\w{0,40})\w*', r'@..\1..', l.strip())
            short = re.sub(r'noalias |noundef |nonnull |align \d+ |captures\([^)]*\) |dereferenceable\(\d+\) ', '', short)
            print('  %d: %s   ;L%s' % (i + 1, short[:170], loc(fn, d.group(1)) if d else '-'))
