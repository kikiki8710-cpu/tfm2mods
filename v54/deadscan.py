# -*- coding: utf-8 -*-
"""★deadscan.py — "지금 소스에 적힌 모든 패치 사이트가 0.5.4 exe에서 실제로 물리는가"를 전수 판정.

왜 필요했나(2026-08-05): sweep.py 의 후보 추출이 **줄 단위 정규식**이라
  `for a in [0xAAA, 0xBBB,      ← 이 줄만 본다
             0xCCC, 0xDDD] {`   ← 이 줄은 통째로 무시
처럼 여러 줄에 걸친 배열 리터럴의 2번째 줄 이후를 못 본다 = 그만큼 조용히 죽는다.
sites.py/sites2.py 파서는 re.S 로 여러 줄을 처리하므로 그쪽을 쓴다.

판정(054 실바이트 기준):
  OK      = .pdata 함수 안 **명령 시작** + 선언 prefix 중 하나와 일치 + capstone imm/disp 오프셋 == 선언 off
  DEAD-A  = 명령 시작이 아님(주소가 어긋남 = 100% 죽음, 잘못 쓰면 명령 파괴)
  DEAD-B  = 명령 시작이지만 prefix 불일치(= 조용히 skip. 안전하지만 노브는 죽음)
  DEAD-C  = prefix 는 맞는데 **off 가 실제 즉치 자리가 아님**(★가장 위험 — 오패치·크래시)
"""
import io, sys, collections
sys.path.insert(0, r'C:\tfm2mods\v54')
import sites as S1, sites2 as S2, reloc as R
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000
E4 = R.E4
_c = {}
# ⚠E4.func_of 는 호출마다 .pdata 를 다시 파싱한다(수만 엔트리) → 700회면 분 단위.
#   한 번만 만들어 bisect 로 찾는다.
import bisect
_FN = E4.funcs(); _FS = [s for s, e in _FN]
def func_of(rva):
    k = bisect.bisect_right(_FS, rva) - 1
    if k < 0: return None
    s, e = _FN[k]
    return (s, e) if s <= rva < e else None

def ins_at(rva):
    f = func_of(rva)
    if not f: return None
    if f[0] not in _c:
        _c[f[0]] = {i.address - B: i for i in R.insns(E4, f[0], f[1])}
    return _c[f[0]].get(rva)

def judge(x):
    i = ins_at(x['rva'])
    if i is None: return 'DEAD-A', None
    pres = x.get('pre') or []
    if pres and not any(bytes(p) == i.bytes[:len(p)] for p in pres if p):
        return 'DEAD-B', i
    e = getattr(i, 'encoding', None)
    io_, is_ = (getattr(e,'imm_offset',0), getattr(e,'imm_size',0)) if e else (0,0)
    do_, ds_ = (getattr(e,'disp_offset',0), getattr(e,'disp_size',0)) if e else (0,0)
    if not ((is_ and x['off']==io_) or (ds_ and x['off']==do_)):
        return 'DEAD-C', i
    return 'OK', i

if __name__ == '__main__':
    site = S1.parse() + S2.parse()
    cnt = collections.Counter(); rows=[]
    for x in site:
        v, i = judge(x)
        cnt[v]+=1
        if v!='OK': rows.append((v,x,i))
    print('사이트 %d개 — 0.5.4 실바이트 대조'%len(site))
    for k in ('OK','DEAD-A','DEAD-B','DEAD-C'): print('  %-7s %4d'%(k,cnt[k]))
    byfn=collections.Counter()
    for v,x,i in rows: byfn[(v,x['file'],x['line'])]+=1
    print('\n죽은 사이트 (파일:줄 기준):')
    for v,x,i in sorted(rows,key=lambda r:(r[1]['file'],r[1]['line'],r[1]['rva'])):
        print('  %-7s %s:%-5d %06x off%d w%d  %s  | 054실물: %s'
              %(v,x['file'],x['line'],x['rva'],x['off'],x['w'],x['pre_txt'][:34],
                (i.bytes.hex()+' '+i.mnemonic+' '+i.op_str) if i else '(명령경계아님)'))
