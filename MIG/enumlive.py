#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""enumlive.py — tcx 열거형 레이아웃에서 **variant 별 살아있는 바이트 범위**(ELEM_LIVE 명세)를 자동 생성한다.

왜 = 열거형 요소를 원시 바이트로 비교하면 **그 variant 가 안 쓰는 칸**(패딩·다른 variant 의 페이로드 자리)에
     스택/힙 잔재가 남아 거짓 DIFF 가 난다. `Chat`(57 variant) 은 variant 마다 페이로드 위치가 달라 손으로 못 한다
     (2026-09-13 실사고: `+8..15` 를 항상 live 로 뒀더니 tag 7 BattleStop 이 `+8` 1B 만 써서 `+9` 에서 거짓 DIFF).

원리 = `tcxdict.py --enum <타입>` 의 「판별자 위치·인코딩」과 「페이로드 X — 절대 오프셋」 표를 파싱해
       live = [(off, len, [tag…])] 를 만든다. 판별자는 항상 live. Direct 인코딩이면 태그 = 선언 discr,
       Niche 이면 메모리태그 열을 쓴다(untagged variant 는 태그가 없어 페이로드를 「항상 live」로 둘 수 없다 ⟹ 제외·보고).

사용: python MIG\\enumlive.py game_core::Chat [--name chat]   → 파이썬 dict 리터럴 출력(gensweep20.ELEM_LIVE 에 붙인다)
"""
import io, re, subprocess, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

def main():
    ty = sys.argv[1]
    name = next((sys.argv[i + 1] for i, a in enumerate(sys.argv) if a == "--name"), ty.split("::")[-1].lower())
    out = subprocess.run([sys.executable, r"C:\tfm2mods\MIG\tcxdict.py", "--enum", ty],
                         capture_output=True).stdout.decode("utf-8", "replace").split("\n")
    disc = next((l for l in out if "판별자:" in l), "")
    m = re.search(r"enum\+0x([0-9a-f]+) \((\d+)B\) · 인코딩=(\w+)", disc)
    if not m: print("판별자 줄을 못 읽음:", disc); sys.exit(1)
    d_off, d_len, enc = int(m.group(1), 16), int(m.group(2)), m.group(3)
    # variant 표: idx 선언discr 메모리태그 이름
    vt = {}
    for l in out:
        mm = re.match(r"\s+(\d+)\s+(\d+)\s+(\S+)\s+(\w+)", l)
        if mm:
            idx, decl, memtag, vname = int(mm.group(1)), int(mm.group(2)), mm.group(3), mm.group(4)
            tag = None if not memtag.isdigit() else int(memtag)
            if enc == "Direct": tag = decl
            vt[vname] = tag
    # 페이로드 표
    live = {}   # (off,len) -> set(tags)
    cur = None; untagged = []
    for l in out:
        mp = re.match(r"\s+페이로드 (\w+) —", l)
        if mp:
            cur = vt.get(mp.group(1));
            if cur is None: untagged.append(mp.group(1))
            continue
        mf = re.match(r"\s+enum\+0x([0-9a-f]+)\s+\S+\s+.*\((\d+)B\)", l)
        if mf and cur is not None:
            off, ln = int(mf.group(1), 16), int(mf.group(2))
            if ln == 0: continue
            live.setdefault((off, ln), set()).add(cur)
    items = [(d_off, d_len, [])] + sorted((o, n, sorted(t)) for (o, n), t in live.items())
    print("# %s — tcx 자동 생성(enumlive.py) · 판별자 +%#x %dB %s · variant %d · untagged(제외)=%s"
          % (ty, d_off, d_len, enc, len(vt), untagged or "없음"))
    print('    "%s": {"live": [' % name)
    for o, n, t in items:
        print("        (%#x, %d, [%s])," % (o, n, ", ".join("%#x" % x for x in t)))
    print('    ], "str": []},')

if __name__ == "__main__":
    main()
