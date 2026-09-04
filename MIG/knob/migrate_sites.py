# -*- coding: utf-8 -*-
"""재핀 4차 — **증거 = 정렬 유사도의 여유(margin)**.

1~3차가 실패한 이유는 전부 같다: 0.5.7 owner 의 0.5.8 짝을 *측정하지 않고 추측*했다
(모듈+크기 / 즉치창 / 전단사 가정). 진짜 짝은 명령스트림 유사도가 형제들보다
확연히 높아야 하고, 그 **여유가 유일성의 근거**다. 여기서 그걸 실제로 잰다.

절차: 모듈별로 0.5.7 dead-owner × 0.5.8 후보 유사도 행렬 → 1위와 2위의 여유 확인
      → 여유가 기준 미만이면 그 owner 는 폐기 → 통과분만 사이트 사상 + prefix 검증.
"""
exec(open('_prelude.py', encoding='utf-8').read())
import difflib

TOK = {}
def tk(img, fn, tag):
    k = (tag, fn)
    if k not in TOK:
        ins = stream(img, fn)
        TOK[k] = (ins, toks(ins)) if ins else (None, None)
    return TOK[k]

# dead owner 수집
owners = {}
for r in dead:
    fr = img7.frange(r['rva'])
    if fr:
        owners.setdefault(fr[0], []).append(r)
bymod = defaultdict(list)
nomod = []
for o, rs in owners.items():
    m = om7.get(o)
    (bymod[m.most_common(1)[0][0]] if m else nomod).append(o) if m else nomod.append(o)
print("dead owner %d개 / 모듈 %d개 / 모듈미상 %d개\n" % (len(owners), len(bymod), len(nomod)))

assign, amb = {}, []
for mod, os7 in sorted(bymod.items(), key=lambda x: -len(x[1])):
    C = mod2fn8.get(mod, [])
    short = mod.split(chr(92))[-1]
    print("[%s] 0.5.7 dead owner %d / 0.5.8 후보 %d" % (short, len(os7), len(C)))
    if not C:
        amb += [(o, short, '후보 0') for o in os7]; continue
    for o7 in os7:
        i7, t7 = tk(img7, o7, '7')
        if not t7:
            amb.append((o7, short, '디스어셈 실패')); continue
        sc = []
        for c8 in C:
            i8, t8 = tk(img8, c8, '8')
            if not t8:
                continue
            sm = difflib.SequenceMatcher(None, t7, t8, autojunk=False)
            if sm.real_quick_ratio() < 0.55 or sm.quick_ratio() < 0.55:
                sc.append((sm.quick_ratio() * 0.99, c8, False)); continue
            sc.append((sm.ratio(), c8, True))
        sc.sort(reverse=True)
        if not sc:
            amb.append((o7, short, '점수 없음')); continue
        best, second = sc[0], (sc[1] if len(sc) > 1 else (0.0, None, True))
        marg = best[0] - second[0]
        okk = best[2] and best[0] >= 0.70 and (len(sc) == 1 or marg >= 0.03)
        print("    0x%-8x(%2d사이트) → 0x%-8x r=%.3f  2위 %.3f  여유 %+.3f  %s"
              % (o7, len(owners[o7]), best[1], best[0], second[0], marg,
                 'OK' if okk else '폐기'))
        if okk:
            assign[o7] = (best[1], best[0], marg, short)
        else:
            amb.append((o7, short, 'r=%.3f 여유=%.3f' % (best[0], marg)))

print("\nowner 확정 %d / 폐기 %d" % (len(assign), len(amb) + len(nomod)))

ok, rej = [], []
for o7, (o8, r0, mg, short) in assign.items():
    i7, _ = tk(img7, o7, '7'); i8, _ = tk(img8, o8, '8')
    sm = align(i7, i8)[0]
    for r in owners[o7]:
        ia = idx_of(i7, r['rva'])
        if ia is None:
            rej.append((r, '인덱스 없음')); continue
        tag, jb = map_idx(sm, ia)
        if tag != 'equal' or jb is None or jb >= len(i8):
            rej.append((r, '사상 tag=%s' % tag)); continue
        na = i8[jb].address - BASE
        b = img8.code(na, r['off'] + r['w'] + 4)
        if not b or list(b[:len(r['pre'])]) != r['pre']:
            rej.append((r, 'prefix 불일치 0x%x' % na)); continue
        r['new'] = na; r['mod'] = short; r['r'] = r0; r['margin'] = mg
        ok.append(r)

print("\n*최종 채택 : %d 사이트" % len(ok))
c = Counter((r['fn'], r['line']) for r in ok)
for (f, l), n in c.most_common(90):
    ex = next(r for r in ok if r['fn'] == f and r['line'] == l)
    print("   %-11s:%-5d %2d개  r=%.3f 여유%+.3f  %-24s %s"
          % (f, l, n, ex['r'], ex['margin'], ex['val'], ex['mod']))
print("   ... %d 소스행" % len(c))

ORIGRE = re.compile(r"b[14]\(\s*\w+\s*,\s*(0x[0-9a-fA-F]+|\d+)\s*\)")
drift = []
for r in ok:
    m = ORIGRE.fullmatch(r['val'].strip())
    r['orig'] = None
    if not m:
        continue
    want = int(m.group(1), 0); r['orig'] = want
    b = img8.code(r['new'], r['off'] + r['w'] + 4)
    cur = b[r['off']] if r['w'] == 1 else struct.unpack('<I', b[r['off']:r['off'] + 4])[0]
    r['cur'] = cur
    if cur != want:
        drift.append(r)
print("\n**ORIG 드리프트 : %d건" % len(drift))
for r in drift:
    print("   %-11s:%-5d 0x%-8x 소스=%s 게임=%s  %s"
          % (r['fn'], r['line'], r['new'], r['orig'], r['cur'], r['val']))

print("\n사이트 폐기 : %d" % len(rej))
for k, n in Counter(w.split('0x')[0].strip() for _, w in rej).most_common(8):
    print("   %-30s %d" % (k, n))
pickle.dump(ok, open('repin_final.pkl', 'wb'))
print("\n-> repin_final.pkl 저장 (%d)" % len(ok))
