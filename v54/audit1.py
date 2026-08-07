# -*- coding: utf-8 -*-
"""① 배선된 노브(tune 키) <-> 편집기 등록(탭/설명/원본맵) 정합 감사.
Rust 문자열 리터럴을 직접 스캔한다 — 정규식 [^"]+ 는 \" 이스케이프와 줄이음(\ 개행)에서 깨진다."""
import sys, io, re, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
src = ''
import glob
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    src += io.open(f, encoding='utf-8').read()
# 배선 방식 2종: ①tune("key") ②cfg 로더의 match arm  "key" => { ... static.store }
wired_tune = set(re.findall(r'tune\(\s*"([a-zA-Z0-9_]+)"', src))
# ⚠꼬리를 (?=...) 룩어헤드로 볼 것. `(.{0,200})` 로 잡으면 그 200자를 **소비**해서,
#   가까이 붙어 있는 다음 match arm 들이 통째로 먹힌다(비중첩 매칭) — 2026-08-06에 이걸로
#   멀쩡한 노브 9개를 "死키"로 오판했다.
wired_arm = set()
for m in re.finditer(r'"([a-zA-Z0-9_]+)"\s*=>\s*\{(?=(.{0,300}))', src, re.S):
    if re.search(r'\.store\(|\.parse|fetch_add|unsafe\s*\{', m.group(2)):
        wired_arm.add(m.group(1))
# ★[08-06] 알리아스 표 반영 — 모드는 cfg 로드 때 옛 키를 새 키로 자동 개명한다
#   (`"oi_dn_nexus_hp" => "nx_dn_nexus_hp"` 등). 이 표를 모르면 **멀쩡히 작동하는 옛 키를
#   '배선 없음'으로 오판**한다 — 실제로 그렇게 판단해 테스트C 에서 oi_* 9개를 죽였다.
wired_alias = set()
for m in re.finditer(r'((?:"[a-z][a-zA-Z0-9_]*"\s*\|\s*)*"[a-z][a-zA-Z0-9_]*")\s*=>\s*"([a-z][a-zA-Z0-9_]*)"\s*,', src):
    tgt = m.group(2)
    if tgt in wired_tune or tgt in wired_arm:
        for k in re.findall(r'"([^"]+)"', m.group(1)):
            wired_alias.add(k)
wired = wired_tune | wired_arm | wired_alias
print('  (알리아스로 살아있는 옛 키 %d개)' % len(wired_alias - wired_tune - wired_arm))
print('  (배선 내역: tune %d + cfg match arm %d)' % (len(wired_tune), len(wired_arm)))

ed = io.open('C:/tfm2mods/ai_adjust_editor/src/main.rs', encoding='utf-8').read()


def scan_strings(s):
    """(시작idx, 문자열내용) 목록. \" 와 \\ 이스케이프 처리."""
    out, i, n = [], 0, len(s)
    while i < n:
        if s[i] == '"':
            j, buf = i + 1, []
            while j < n:
                if s[j] == '\\':
                    buf.append(s[j:j + 2]); j += 2; continue
                if s[j] == '"':
                    break
                buf.append(s[j]); j += 1
            out.append((i, ''.join(buf)))
            i = j + 1
        elif s[i] == '/' and i + 1 < n and s[i + 1] == '/':
            while i < n and s[i] != '\n':
                i += 1
        else:
            i += 1
    return out


STR = scan_strings(ed)
KEY = re.compile(r'^[a-z][a-zA-Z0-9_]{2,}$')

# 설명 맵: "key" => "설명"  — 설명은 한글을 포함하거나 20자 이상
desc, orig = {}, {}
for idx in range(len(STR) - 1):
    (p1, k), (p2, v) = STR[idx], STR[idx + 1]
    between = ed[p1 + len(k) + 2:p2]
    if '=>' not in between or len(between) > 12:
        continue
    if not KEY.match(k):
        continue
    if re.fullmatch(r'-?\d+', v):
        orig[k] = v
    elif re.search(r'[가-힣]', v) or len(v) >= 20:
        desc[k] = v

tabs = {}
for m in re.finditer(r'Tab\{\s*id:"(\w+)".*?keys:&\[(.*?)\], note:', ed, re.S):
    ks = [x for _, x in scan_strings(m.group(2)) if not x.startswith('§')]
    tabs[m.group(1)] = ks
in_tab = {k for ks in tabs.values() for k in ks}

print('배선된 노브(tune 키)  = %d' % len(wired))
print('편집기 탭 노출 키     = %d  (탭 %d개)' % (len(in_tab), len(tabs)))
print('설명이 있는 키        = %d' % len(desc))
print('원본값 맵 키          = %d' % len(orig))


def show(title, s, limit=60):
    s = sorted(s)
    print('\n= %s : %d건' % (title, len(s)))
    for i in range(0, min(len(s), limit), 6):
        print('   ' + '  '.join(s[i:i + 6]))
    if len(s) > limit:
        print('   ... 외 %d건' % (len(s) - limit))


# ⚠알리아스(옛 이름)는 탭에 없는 게 정상이다 — 은닉 노브로 세면 안 된다
show('*배선됐지만 편집기 탭에 없음(만질 수 없는 死노브)', wired - in_tab - wired_alias)
show('알리아스(옛 이름 → 현행 키). 탭에 없는 것이 정상', wired_alias - in_tab, limit=36)
show('*편집기 탭에 있지만 배선 안 됨(눌러도 무반응인 死키)', in_tab - wired)
show('탭엔 있는데 설명 없음', in_tab - set(desc))
show('설명은 있는데 탭에 없음(고아 설명)', set(desc) - in_tab)

import json
json.dump({'wired': sorted(wired), 'in_tab': sorted(in_tab),
           'desc': desc, 'orig': orig, 'tabs': tabs},
          io.open('audit_state.json', 'w', encoding='utf-8'), ensure_ascii=False)
print('\n(audit_state.json 저장)')
