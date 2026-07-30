# -*- coding: utf-8 -*-
# pii_check.py — 릴리스 산출물의 유저 로컬 경로(PII) 검사.
#   ⚠bash heredoc 으로 이 패턴을 쓰면 백슬래시가 소실돼 **가짜 음성**이 난다 ⟹ 반드시 파일로 실행할 것.
import zipfile, os, re, sys, io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
G = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods"
PAT = re.compile(rb"[A-Za-z]:\\Users\\dev", re.I)


def probe(data, tag):
    hits = list(PAT.finditer(data))
    print(f"{tag}: {len(hits)}건")
    for m in hits[:8]:
        s = max(0, m.start() - 30)
        e = min(len(data), m.end() + 110)
        txt = data[s:e].decode("utf-8", "replace").replace("\x00", "")
        print(f"     …{txt[:150]}")
    return len(hits)


if __name__ == "__main__":
    z2 = zipfile.ZipFile(os.path.join(G, "release", "0.5.2", "tfm2_ai_adjust.zip"))
    probe(z2.read("tfm2_ai_adjust/설정편집기.exe"), "0.5.2 릴리스 설정편집기.exe")
    probe(open(os.path.join(G, "tfm2_ai_adjust", "설정편집기.exe"), "rb").read(), "라이브 설정편집기.exe")
    print()
    # 0.5.3 릴리스 zip 전수
    OUT = os.path.join(G, "release", "0.5.3")
    for f in sorted(os.listdir(OUT)):
        if not f.endswith(".zip"):
            continue
        z = zipfile.ZipFile(os.path.join(OUT, f))
        bad = []
        for i in z.infolist():
            if i.filename.endswith("/") or i.file_size > 60 * 1024 * 1024:
                continue
            if PAT.search(z.read(i.filename)):
                bad.append(i.filename)
        print(f"{f}: {'★' + str(bad) if bad else 'PII 0건'}")
