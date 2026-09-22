# -*- coding: utf-8 -*-
"""_gaibc + _gcbc IR 콜그래프(define → 직접 call/invoke 심볼 집합) + TLS 전역 직접 참조 표를 만들어 pickle."""
import io, re, glob, os, pickle, sys, time
sys.stdout.reconfigure(encoding='utf-8')
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'cg.pkl')
files = sorted(glob.glob(r'C:\tfm2mods\_gaibc\m*.ll')) + sorted(glob.glob(r'C:\tfm2mods\_gcbc\*.ll'))
cg = {}      # sym -> set(callee syms)
tlsref = {}  # sym -> set(tls globals)
tlsg = {}    # tls global -> (file, decl line)
loc = {}     # sym -> (file, line)
callre = re.compile(r'(?:call|invoke)\s[^@]*@([A-Za-z0-9_.$]+)\(')
t0 = time.time()
for f in files:
    base = os.path.basename(f)
    cur = None
    with io.open(f, encoding='utf-8', errors='ignore') as fh:
        for ln, l in enumerate(fh, 1):
            if l.startswith('@') and 'thread_local' in l:
                tlsg[l.split(' =', 1)[0][1:]] = (base, ln)
                continue
            if l.startswith('define '):
                m = re.search(r'@([A-Za-z0-9_.$]+)\(', l)
                cur = m.group(1) if m else None
                if cur: loc[cur] = (base, ln); cg.setdefault(cur, set()); tlsref.setdefault(cur, set())
                continue
            if cur is None: continue
            if l.startswith('}'):
                cur = None; continue
            if 'call' in l or 'invoke' in l:
                m = callre.search(l)
                if m: cg[cur].add(m.group(1))
            if '@' in l and 'thread_local' not in l:
                for g in re.findall(r'@([A-Za-z0-9_.$]+)', l):
                    if g in tlsg or ('thread' in g.lower() and 'local' in g.lower()):
                        tlsref[cur].add(g)
    print(base, 'done', int(time.time()-t0), 's', flush=True)
# tlsref 재검사: tlsg 는 파일 순서상 뒤에 나올 수 있으니 2차 패스는 생략하고 이름 집합만 저장
pickle.dump({'cg': cg, 'tlsref': tlsref, 'tlsg': tlsg, 'loc': loc}, open(OUT, 'wb'))
print('defines', len(cg), 'tls globals', len(tlsg))
