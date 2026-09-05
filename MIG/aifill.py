# -*- coding: utf-8 -*-
r"""aifill — aidump 스캐폴딩의 `<본문>` 자리를 **디컴 결과로 자동 채운다**

    python MIG\aifill.py <decomp디렉터리> [옵션]

`aidump.py` 가 깔아 둔 원본-소스-트리 스캐폴딩을 순회하며 함수마다
Ghidra 서버에서 디컴을 받아 본문에 넣는다. **매 버전 반복하는 작업이라 도구화했다** —
예전엔 사람이/에이전트가 함수 수백 개를 손으로 돌렸고, 그때마다 같은 함정을 다시 밟았다.

세 패스로 나뉜다(기본은 셋 다, `--pass` 로 선택):

  fill  각 함수의 디컴 본문을 받아 채운다. 알려진 필드 오프셋·상수는 줄 끝에 주석.
  cap   ★Ghidra 에 **함수가 정의돼 있지 않아** 디컴이 실패한 자리를 exe 바이트의
        capstone 선형 디스어셈으로 대체한다. (xref 가 `.rdata` DATA 뿐인 함수는
        오토애널라이저가 함수를 만들지 않는다. MCP 에 create-function API 는 없다.)
  jt    ★Ghidra 가 `Could not recover jumptable … Too many branches` 로 **switch 아암
        전체를 조용히 누락**한 함수에 보강 블록을 붙인다. 디컴 C 만 믿으면 로직이
        통째로 빠진 채로 남는다 — 실측 0.5.8 에서 21개 함수가 이랬다.

전부 **멱등**이다. 다시 돌리면 이전 산출물을 걷어내고 다시 만든다.

⚠ Ghidra 서버 포트는 버전마다 다르다(0.5.8 = 8081). 엉뚱한 포트에 붙으면
   **구버전 exe 를 디컴해 놓고 성공했다고 보고**하므로, 시작 시 프롤로그 바이트를
   exe 와 대조해 확인한다(`--no-verify` 로 끌 수 있으나 권장하지 않는다).
"""
import argparse
import io
import os
import re
import struct
import sys
import time
import urllib.parse
import urllib.request

try:
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64
except ImportError:
    Cs = None

if hasattr(sys.stdout, 'reconfigure'):          # 콘솔 기본 cp949 라 한국어·em-dash 가 깨진다
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')
    sys.stderr.reconfigure(encoding='utf-8', errors='replace')

BASE_IMG = 0x140000000
PH = '// <\ubcf8\ubb38> \u2014 \ub514\ucef4 \uacb0\uacfc\ub97c \uc5ec\uae30\uc5d0 \ucc44\uc6b4\ub2e4'
MANUAL = '// [MANUAL]'   # 이 마커가 본문에 있으면 --force 여도 보존한다

# 확립된 오프셋 관례 — 줄 끝 주석으로 붙여 디컴 C 를 읽을 수 있게 만든다.
OFFS = {
    '0x930': 'side', '0x9c0': 'role', '0x928': '\ucea1\uc2dc\ud0a4',
    '0x660': 'x', '0x668': 'y', '0x670': '\ud604\uc7acHP', '0x628': '\ucd5c\ub300HP',
    '0x5c0': '\ud578\ub4e4', '0x680': '\uae30\ubcf8\uc0ac\uac70\ub9ac',
    '0x470': '\uc0ac\uac70\ub9ac%\ubcf4\uc815', '0x438': '\ud788\ud2b8\ubc15\uc2a4',
    '0x5c8': '\ub808\ubca8', '0x12f8': 'tick/sec',
    '0x490': '\uc5b4\ube4c\uc2ac\ub86f base(+k*0x38)',
    '0x1e0': '\ub85c\uc2a4\ud130 base(+side*0x28+role*8)',
    '0xb1': 'SmallAction \ud0dc\uadf8', '0xb8': 'SmallAction stride',
    '0x2940': '\uc7ac\ud50c\ub79c \ud0c0\uc774\uba38',
}
CONSTS = {
    '0x9502f9001': '200000^2+1', '0x9502f900': '200000^2',
    '0x2faf080': '50000000', '0x3b9aca00': '1e9', '0x5f5e100': '1e8', '0x989680': '1e7',
}


# ────────────────────────────────────────────────────────────── PE
class Exe(object):
    def __init__(self, path):
        d = io.open(path, 'rb').read()
        pe = struct.unpack_from('<I', d, 0x3c)[0]
        nsec = struct.unpack_from('<H', d, pe + 6)[0]
        opt = pe + 24
        self.ib = struct.unpack_from('<Q', d, opt + 24)[0]
        sectab = opt + struct.unpack_from('<H', d, pe + 20)[0]
        self.secs = []
        for i in range(nsec):
            o = sectab + i * 40
            nm = d[o:o + 8].rstrip(b'\0').decode('ascii', 'replace')
            vsz, va, rsz, rraw = struct.unpack_from('<IIII', d, o + 8)
            self.secs.append((nm, va, vsz, rraw, rsz))
        self.d = d
        t = [x for x in self.secs if x[0] == '.text'][0]
        self.ts, self.tsz = t[1], t[2]

    def off(self, rva):
        for nm, va, vsz, rraw, rsz in self.secs:
            if va <= rva < va + max(vsz, rsz):
                return rraw + (rva - va)
        return None

    def code(self, s, e):
        o = self.off(s)
        return self.d[o:o + (e - s + 1)] if o is not None else b''

    def rd32(self, rva):
        o = self.off(rva)
        if o is None or o + 4 > len(self.d):
            return None
        return struct.unpack_from('<i', self.d, o)[0]


def mkmd():
    if Cs is None:
        sys.exit('capstone 이 없다 — pip install capstone')
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = False
    return md


# ────────────────────────────────────────────────────────────── HTTP
class Srv(object):
    def __init__(self, port, timeout):
        self.url = 'http://127.0.0.1:%d/' % port
        self.timeout = timeout
        self.dec_opt = {}          # decompile_function 에 함께 보낼 인자(timeout/payload)

    def get(self, path, **params):
        u = self.url + path + '?' + urllib.parse.urlencode(params)
        with urllib.request.urlopen(u, timeout=self.timeout) as r:
            return r.read().decode('utf-8', 'replace')

    def get_retry(self, path, **params):
        """★1회 재시도는 필수 — RemoteDisconnected 로 죽은 요청이 재시도로 살아난다."""
        try:
            return self.get(path, **params), None
        except Exception as e1:
            time.sleep(1.5)
            try:
                return self.get(path, **params), None
            except Exception as e2:
                return '', '%s / 재시도: %s' % (e1, e2)


def bad_body(b):
    t = b.strip()
    return (not t) or t.startswith('Decompilation failed') or t[:6] == 'Error:' \
        or 'No function' in t[:60]


# ────────────────────────────────────────────────────────────── 스캐폴딩 파싱
H_RVA = re.compile(r'^## `0x([0-9a-f]+)`')
H_RNG = re.compile(r'^\| RVA \| `0x([0-9a-f]+)` ~ `0x([0-9a-f]+)`')
H_LINE = re.compile(r'^\| \uc6d0\ubcf8 \ud589 \| (.+?) \|')


def scan(path):
    """{rva문자열: (시작, 끝)} 범위표."""
    rng, cur = {}, None
    for ln in io.open(path, encoding='utf-8'):
        h = H_RVA.match(ln)
        if h:
            cur = h.group(1)
        r = H_RNG.match(ln)
        if r and cur:
            rng[cur] = (int(r.group(1), 16), int(r.group(2), 16))
    return rng


def walk(root):
    out = []
    for dp, _, fns in os.walk(root):
        for f in sorted(fns):
            if f.endswith('.md') and f != 'INDEX.md':
                out.append(os.path.join(dp, f))
    return sorted(out)


def annotate(body):
    out = []
    for ln in body.split('\n'):
        if ln.strip().startswith('//'):
            out.append(ln)
            continue
        notes = []
        for k, v in OFFS.items():
            if re.search(r'(?<![0-9a-zA-Z_])' + k + r'(?![0-9a-fA-F])', ln):
                notes.append(k + '=' + v)
        for k, v in CONSTS.items():
            if k in ln:
                notes.append(k + '=' + v)
        out.append(ln.rstrip() + '  // ' + ' \u00b7 '.join(notes[:4]) if notes else ln)
    return '\n'.join(out)


# ────────────────────────────────────────────────────────────── pass 1: fill
# 함수 블록 = `## \`0x…\`` 헤더 + 표 + ```c 코드펜스. 본문은 group(3).
BLOCK_RE = re.compile(r'(^## `0x([0-9a-f]+)`.*?\n```c\n)(.*?)(\n```)', re.S | re.M)


def pass_fill(path, srv, force):
    """★블록 **전체**를 교체한다.

    ⚠예전 구현은 `// fn @` **한 줄만** 갈아끼웠다. 그러면 그 아래 옛 본문이 그대로 남아
    `--force` 를 돌릴 때마다 본문이 **한 벌씩 누적**된다(실측: 640블록 중 267개가 2~4벌).
    헤더는 1개뿐이라 헤더 수로는 이 중복이 안 잡힌다 — 산출물이 커진 것을 "복구 성과"로
    오인하기 딱 좋다. 그래서 코드펜스 안을 통째로 갈아끼운다.
    """
    txt = io.open(path, encoding='utf-8').read()
    m0 = re.search(r'`game-ai.src.(.+?)`', txt.split('\n', 1)[0])
    module = m0.group(1).replace(chr(92), '/').split('/')[-1] if m0 else os.path.basename(path)

    pieces, last, filled, fails = [], 0, 0, []
    for m in BLOCK_RE.finditer(txt):
        body = m.group(3)
        # ★사람이 손으로 복원한 블록은 --force 여도 건드리지 않는다.
        #   Ghidra 가 끝내 못 푸는 함수(예: `Backward normalization not implemented`)는
        #   디스어셈을 읽고 C 로 옮겨 적는 수밖에 없는데, 그게 재수신 때 날아가면
        #   그 수고를 매번 다시 해야 한다.
        if MANUAL in body:
            continue
        if not (PH in body or force):
            continue
        rva = m.group(2)
        lm = H_LINE.search(m.group(1))
        cur_line = lm.group(1).split(',')[0].strip() if lm else '?'
        addr = '0x%x' % (int(rva, 16) + BASE_IMG)
        got, err = srv.get_retry('decompile_function', address=addr, **srv.dec_opt)
        # ★1차가 타임아웃이면 한도를 올려 한 번 더. 대형 함수는 이것만으로 살아난다
        #   (실측 0.5.8: `0xe4c5c0` 8,407명령 → 444초에 32,528행, `0xe46bc0` → 44,977행).
        if (err or bad_body(got)) and 'timeout' in (got or '').lower():
            got, err = srv.get_retry('decompile_function', address=addr,
                                     timeout='2400', payload='800')
        head = '// fn @ %s:%s   [RVA 0x%s]' % (module, cur_line, rva)
        if err or bad_body(got):
            fails.append(rva)
            dis, err2 = srv.get_retry('disassemble_function', address=addr)
            # 사유는 한 줄로 눌러 담는다(여러 줄이면 뒤가 잘려 원인이 안 보인다).
            why = ' '.join((err or got or 'empty').split())[:220]
            new = '%s\n// DECOMP FAILED: %s\n/* --- disassembly ---\n%s\n*/' % (
                head, why, dis or ('(disasm 실패: %s)' % err2))
        else:
            new = '%s\n%s' % (head, annotate(got.strip('\n')))
        pieces.append(txt[last:m.start(3)])
        pieces.append(new)
        last = m.end(3)
        filled += 1
    pieces.append(txt[last:])
    io.open(path, 'w', encoding='utf-8', newline='\n').write(''.join(pieces))
    return filled, fails


# ────────────────────────────────────────────────────────────── pass 2: capstone 대체
def pass_cap(path, exe, md):
    txt = io.open(path, encoding='utf-8').read()
    if 'DECOMP FAILED' not in txt:
        return 0
    rng = scan(path)
    lines, out, i, fixed = txt.split('\n'), [], 0, 0
    while i < len(lines):
        if lines[i].startswith('// DECOMP FAILED:'):
            rv = None
            for j in range(i - 1, max(-1, i - 4), -1):
                mm = re.search(r'\[RVA 0x([0-9a-f]+)\]', lines[j])
                if mm:
                    rv = mm.group(1)
                    break
            k = i + 1
            while k < len(lines) and not lines[k].startswith('*/'):
                k += 1
            if rv and rv in rng:
                s, e = rng[rv]
                try:
                    body = '\n'.join('%08x  %-8s %s' % (x.address, x.mnemonic, x.op_str)
                                     for x in md.disasm(exe.code(s, e), exe.ib + s))
                except Exception as ex:
                    body = '(capstone 실패: %s)' % ex
                # ⚠사유를 고정 문구로 덮어쓰면 안 된다. 실제 사유(타임아웃/payload 초과/
                #   함수 미정의)가 가려져 다음 사람이 엉뚱한 대책을 세운다 — 실제로 그랬다.
                out.append(lines[i] + '   // (capstone 으로 대체함)')
                out.append('// 대체: capstone 선형 디스어셈 (RVA 0x%x~0x%x)' % (s, e))
                out.append('/* --- capstone linear disassembly ---')
                out.append(body)
                out.append('*/')
                fixed += 1
                i = k + 1
                continue
        out.append(lines[i])
        i += 1
    io.open(path, 'w', encoding='utf-8', newline='\n').write('\n'.join(out))
    return fixed


# ────────────────────────────────────────────────────────────── pass 3: jumptable 보강
RIP = re.compile(r'\[rip ([+-]) (0x[0-9a-f]+)\]')
IDXMEM = re.compile(r'dword ptr \[(r[a-z0-9]+) \+ r[a-z0-9]+\*4\]')
MARK = '/* === \ubcf4\uac15:'


class JT(object):
    def __init__(self, exe, md):
        self.exe, self.md = exe, md

    def dis(self, s, e):
        return list(self.md.disasm(self.exe.code(s, e), self.exe.ib + s))

    def _lea(self, t):
        mm = RIP.search(t.op_str)
        if not mm:
            return None
        d = int(mm.group(2), 16) * (1 if mm.group(1) == '+' else -1)
        return (t.address + t.size + d) - self.exe.ib

    def bound(self, seq, n):
        """`cmp reg, N; ja` 가드에서 아암 상한을 뽑는다 — 인접 테이블 과읽 방지."""
        for k in range(n, max(0, n - 8), -1):
            if seq[k].mnemonic in ('ja', 'jae') and k > 0 and seq[k - 1].mnemonic == 'cmp':
                mm = re.search(r',\s*(0x[0-9a-f]+|\d+)$', seq[k - 1].op_str)
                if mm:
                    v = int(mm.group(1), 16) if mm.group(1).startswith('0x') else int(mm.group(1))
                    if 0 < v < 512:
                        return v + 1 if seq[k].mnemonic == 'ja' else v
        return None

    def cands(self, seq, n, full):
        a, c = seq[n].address, []
        for k in range(n, max(0, n - 16), -1):
            t = seq[k]
            if t.mnemonic == 'lea' and 'rip' in t.op_str:
                v = self._lea(t)
                if v is not None:
                    c.append(v)
        for k in range(n, max(0, n - 10), -1):
            mm = IDXMEM.search(seq[k].op_str)
            if mm:
                reg = mm.group(1)
                for pool in (seq, full):
                    hits = [self._lea(t) for t in pool
                            if t.address < a and t.mnemonic == 'lea'
                            and t.op_str.startswith(reg + ',') and 'rip' in t.op_str]
                    c += [h for h in reversed(hits) if h is not None]
                break
        seen, out = set(), []
        for x in c:
            if x not in seen:
                seen.add(x)
                out.append(x)
        return out

    def arms(self, base, a_abs, limit=512):
        e, out = self.exe, []
        if base is None:
            return out
        for i in range(limit):
            v = e.rd32(base + 4 * i)
            if v is None:
                break
            t = e.ib + base + v
            if not (e.ib + e.ts <= t < e.ib + e.ts + e.tsz):
                break
            if abs(t - a_abs) > 0x40000:
                break
            out.append(t)
        return out

    def resolve(self, a_abs, ins):
        idx = {i.address: n for n, i in enumerate(ins)}
        pools = []
        if a_abs in idx:
            pools.append((ins, idx[a_abs]))
        else:
            arva = a_abs - self.exe.ib
            for back in range(0x60, 0, -1):
                try:
                    seq = self.dis(arva - back, arva + 8)
                except Exception:
                    continue
                hit = [n for n, t in enumerate(seq) if t.address == a_abs]
                if hit:
                    pools.append((seq, hit[0]))
                    if len(pools) >= 8:
                        break
        for wide in (0x200, 0x400, 0x800):      # 테이블 베이스가 호이스팅된 경우
            try:
                seq = self.dis(a_abs - self.exe.ib - wide, a_abs - self.exe.ib + 8)
            except Exception:
                continue
            hit = [n for n, t in enumerate(seq) if t.address == a_abs]
            if hit:
                pools.append((seq, hit[0]))
        best = (None, [], None)
        for seq, n in pools:
            bd = self.bound(seq, n)
            for c in self.cands(seq, n, ins):
                arms = self.arms(c, a_abs)
                if len(arms) > len(best[1]):
                    best = (c, arms, bd)
                if len(best[1]) >= 4:
                    return best
        return best

    def build(self, s, e, body):
        ib = self.exe.ib
        jts = sorted(set(int(x, 16) for x in re.findall(
            r'Could not recover jumptable at 0x0*([0-9a-f]+)', body)))
        ins = self.dis(s, e)
        ap = ['', MARK + ' jumptable 미복구 구간 (Ghidra "Too many branches") ===',
              '   ※ case 목록 = rip-rel lea 테이블 휴리스틱 디코드(서명 4B 오프셋, .text 이탈 시 종료).',
              '   ※ arms=0 은 스위치 테이블이 아니라 간접 tail-call(fmt/vtable) 일 수 있다.']
        extra = []
        for a in jts:
            base, arms, bd = self.resolve(a, ins)
            if base is None:
                ap.append('   JT @ 0x%x : 테이블 베이스 미판정' % a)
            else:
                note = ''
                if bd and len(arms) > bd:
                    note, arms = '  (cmp/ja bound=%d 적용; 초과분은 인접 테이블 과읽)' % bd, arms[:bd]
                elif bd:
                    note = '  (cmp/ja bound=%d)' % bd
                ap.append('   JT @ 0x%x  table=0x%x  arms=%d%s' % (a, ib + base, len(arms), note))
                for i, t in enumerate(arms):
                    ap.append('     case %3d -> 0x%x' % (i, t))
                for t in arms:
                    r = t - ib
                    if not (s <= r <= e):
                        extra.append([r, min(r + 0x200, self.exe.ts + self.exe.tsz - 1)])
            if not (s <= a - ib <= e):
                extra.append([a - ib - 0x120, a - ib + 0x40])
        ap.append('')
        ap.append('   --- capstone 선형 디스어셈 (본체 0x%x~0x%x) ---' % (s, e))
        for i in ins:
            ap.append('   %08x  %-8s %s' % (i.address, i.mnemonic, i.op_str))
        merged = []
        for a, b in sorted(extra):
            if merged and a <= merged[-1][1] + 0x80:
                merged[-1][1] = max(merged[-1][1], b)
            else:
                merged.append([a, b])
        for cs, ce in merged:
            ap.append('')
            ap.append('   --- 추가 청크(Ghidra 비연속 함수바디) 0x%x~0x%x ---' % (cs, ce))
            try:
                for i in self.dis(cs, ce):
                    ap.append('   %08x  %-8s %s' % (i.address, i.mnemonic, i.op_str))
            except Exception as ex:
                ap.append('   (disasm 실패: %s)' % ex)
        ap.append('*/')
        return '\n'.join(ap)


def strip_old(path):
    L, out, i, n = io.open(path, encoding='utf-8').read().split('\n'), [], 0, 0
    while i < len(L):
        if L[i].startswith(MARK):
            if out and out[-1] == '':
                out.pop()
            while i < len(L) and L[i].strip() != '*/':
                i += 1
            i += 1
            n += 1
            continue
        out.append(L[i])
        i += 1
    io.open(path, 'w', encoding='utf-8', newline='\n').write('\n'.join(out))
    return n


def pass_jt(path, jt):
    strip_old(path)                                   # 멱등
    txt = io.open(path, encoding='utf-8').read()
    if 'Too many branches' not in txt:
        return 0
    rng = scan(path)
    pieces, last, n = [], 0, 0
    for m in re.finditer(r'// fn @ (\S+)\s+\[RVA 0x([0-9a-f]+)\]\n(.*?)\n```', txt, re.S):
        if 'Too many branches' not in m.group(3) or m.group(2) not in rng:
            continue
        s, e = rng[m.group(2)]
        pieces.append(txt[last:m.end(3)])
        pieces.append(jt.build(s, e, m.group(3)))
        last, n = m.end(3), n + 1
    pieces.append(txt[last:])
    io.open(path, 'w', encoding='utf-8', newline='\n').write(''.join(pieces))
    return n


# ────────────────────────────────────────────────────────────── 검증·main
def verify_server(srv, exe, files):
    """★서버가 **이 exe** 를 열고 있는지 프롤로그 16B 대조로 확인.

    포트를 잘못 잡으면 구버전을 디컴해 놓고 성공으로 보고한다 — 조용한 오염이라
    나중에 원인 추적이 거의 불가능하다. 그래서 시작 시 반드시 본다.
    """
    md = mkmd()
    for p in files:
        for rv, (s, e) in sorted(scan(p).items()):
            dis, err = srv.get_retry('disassemble_function', address='0x%x' % (s + BASE_IMG))
            if err or not dis.strip():
                continue
            # Ghidra 포맷: "140db6850: PUSH RBP" — 니모닉이 **대문자**다.
            # 명령 1개만 보면 약하다(대부분 push 로 시작) → 앞 3개 니모닉을 본다.
            g = []
            for ln in dis.split('\n'):
                mm = re.match(r'([0-9a-f]{6,}):\s+([A-Za-z][A-Za-z0-9.]*)\s*(.*)', ln.strip())
                if mm and int(mm.group(1), 16) >= s + BASE_IMG:
                    g.append((int(mm.group(1), 16), mm.group(2).lower()))
                    if len(g) == 3:
                        break
            if len(g) < 3 or g[0][0] != s + BASE_IMG:
                continue
            mine = [(x.address, x.mnemonic)
                    for x in md.disasm(exe.code(s, min(e, s + 48)), exe.ib + s)][:3]
            if len(mine) < 3:
                continue
            ok = all(a[0] == b[0] and a[1] == b[1] for a, b in zip(g, mine))
            return ok, '0x%x  ghidra=[%s]  /  exe=[%s]' % (
                s, ' '.join(x[1] for x in g), ' '.join(x[1] for x in mine))
    return None, '대조할 함수를 못 찾음'


def main():
    ap = argparse.ArgumentParser(description='aidump 스캐폴딩 자동 채움')
    ap.add_argument('root', help='decomp\\<버전> 디렉터리')
    ap.add_argument('--exe', default=r'C:\Program Files (x86)\Steam\steamapps'
                                     r'\common\Teamfight Manager2\TeamfightManager2.exe')
    ap.add_argument('--port', type=int, default=8081, help='Ghidra MCP 서버 포트 (0.5.8=8081)')
    ap.add_argument('--timeout', type=int, default=600)
    ap.add_argument('--only', default='', help='이 문자열을 포함한 경로만')
    ap.add_argument('--passes', default='fill,cap,jt')
    ap.add_argument('--force', action='store_true', help='이미 채운 본문도 다시 받는다')
    ap.add_argument('--no-verify', action='store_true')
    ap.add_argument('--dec-timeout', type=int, default=0, help='디컴 타임아웃(초)')
    ap.add_argument('--dec-payload', type=int, default=0, help='디컴 payload 한도(MB)')
    a = ap.parse_args()

    files = [p for p in walk(a.root) if a.only in p]
    if not files:
        sys.exit('대상 .md 가 없다: %s' % a.root)
    passes = set(x.strip() for x in a.passes.split(','))
    exe, srv = Exe(a.exe), Srv(a.port, a.timeout)
    if a.dec_timeout:
        srv.dec_opt['timeout'] = str(a.dec_timeout)
    if a.dec_payload:
        srv.dec_opt['payload'] = str(a.dec_payload)
    print('대상 %d개 파일 · exe=%s · 포트 %d' % (len(files), os.path.basename(a.exe), a.port))

    if 'fill' in passes and not a.no_verify:
        ok, msg = verify_server(srv, exe, files[:3])
        if ok is None:
            print('  ⚠ 서버 대조 불가 — %s' % msg)
        elif not ok:
            sys.exit('★서버가 다른 exe 를 열고 있다 (포트 확인!):\n   ' + msg)
        else:
            print('  서버↔exe 프롤로그 일치 확인: %s' % msg)

    md = mkmd()
    jt = JT(exe, md)
    t0 = time.time()
    tf = tc = tj = 0
    allfail = []
    for n, p in enumerate(files, 1):
        rel = os.path.relpath(p, a.root)
        f = c = j = 0
        if 'fill' in passes:
            f, fails = pass_fill(p, srv, a.force)
            allfail += [(rel, x) for x in fails]
        if 'cap' in passes:
            c = pass_cap(p, exe, md)
        if 'jt' in passes:
            j = pass_jt(p, jt)
        tf, tc, tj = tf + f, tc + c, tj + j
        print('  [%3d/%d] %-52s 채움 %3d · capstone %2d · JT %2d  (%.0fs)'
              % (n, len(files), rel, f, c, j, time.time() - t0))
        sys.stdout.flush()

    left = 0
    for p in files:
        left += io.open(p, encoding='utf-8').read().count(PH)
    print('\n합계 — 채움 %d · capstone 대체 %d · JT 보강 %d · 잔여 플레이스홀더 %d  (%.0f초)'
          % (tf, tc, tj, left, time.time() - t0))
    if allfail:
        print('디컴 실패 %d건 (capstone 으로 대체됨):' % len(allfail))
        for rel, rv in allfail[:40]:
            print('   0x%s  %s' % (rv, rel))


if __name__ == '__main__':
    main()
