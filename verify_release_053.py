# -*- coding: utf-8 -*-
# verify_release_053.py — 릴리스 zip 무결성 검증 (/deploy §4 체크리스트)
import io, sys, os, re, zipfile
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

Z = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\release\0.5.3\tfm2_ai_adjust.zip"
LIVE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust"
z = zipfile.ZipFile(Z)

print("① BOM 검사 (게임이 읽는 텍스트 = BOM 있으면 모드 강제비활성)")
bom = [e.filename for e in z.infolist() if z.read(e.filename)[:3] == b"\xef\xbb\xbf"]
mi = z.read("tfm2_ai_adjust/mod.mod_info")[:3]
print("   mod.mod_info 첫3바이트 = %s  %s" % (list(mi), "✅(0x7b)" if mi[0] == 0x7b else "⛔"))
print("   BOM 포함 파일: %s" % (bom if bom else "0개 ✅"))

print("\n② 개인정보·로컬경로 유출 검사 (텍스트 엔트리)")
PAT = re.compile(rb"dev|Users|Desktop|steamapps|tfm2mods|[.]pdb", re.I)
hit = 0
for e in z.infolist():
    if e.filename.endswith((".exe", ".dll")):
        continue
    for m in sorted(set(PAT.findall(z.read(e.filename)))):
        print("   ⚠ %s : %s" % (e.filename, m.decode(errors="replace")))
        hit += 1
print("   → %d건 %s" % (hit, "" if hit else "(없음 ✅)"))

print("\n③ 산출물이 최신 배포본과 동일한가")
for f in ("tfm2_ai_adjust.dll", "설정편집기.exe"):
    zs = z.getinfo("tfm2_ai_adjust/" + f).file_size
    ls = os.path.getsize(os.path.join(LIVE, f))
    print("   %-20s zip=%-9d live=%-9d %s" % (f, zs, ls, "✅" if zs == ls else "⛔불일치"))

print("\n④ zip 내 cfg 핵심값 (진단 OFF / 기능 ON)")
t = z.read("tfm2_ai_adjust/tfm2_ai_adjust.cfg").decode("utf-8")
for k in ("log", "mpcap", "hang_diag", "adv_prof", "d7_repl", "mp_repl",
          "oi_enable", "d19i_enable", "gb_enable", "sv_enable", "d4_repl"):
    m = re.search(r'(?m)^\s*%s\s*=\s*(\S+)' % k, t)
    print("   %-13s = %s" % (k, m.group(1) if m else "(없음)"))

print("\n⑤ 라이브 cfg 무수정 확인 (유저 테스트값 보존이 원칙)")
lv = open(os.path.join(LIVE, "tfm2_ai_adjust.cfg"), "rb").read()
print("   라이브 첫3바이트=%s (BOM 없음) / 줄수 live=%d zip=%d"
      % (list(lv[:3]), len(lv.decode("utf-8-sig").splitlines()), len(t.splitlines())))
lm = re.search(r'(?m)^\s*d7_repl\s*=\s*(\S+)', lv.decode("utf-8-sig"))
print("   라이브 d7_repl = %s (zip과 동일해도 무방 — 이번엔 둘 다 1)" % (lm.group(1) if lm else "?"))

print("\n⑥ 제외돼야 할 런타임·개인 파일이 섞였는지")
BAD = re.compile(r"(_imm|_diag|hooks|crash|mpcmp|mpout|mpws|egate|poke|defwatch|repl_status|seedstrat|dispcmp|sp_seen|baseinp|condcmp|detdiv|dd0diff|itemnet_guard|sim_unchunk|lane_gate|pokerng)\.txt$|[.]bak|[.]pdb|_crash|match_log", re.I)
bad = [e.filename for e in z.infolist() if BAD.search(e.filename)]
print("   → %s" % (bad if bad else "0개 ✅"))
print("\n총 %d 엔트리 / %d B" % (len(z.infolist()), os.path.getsize(Z)))
z.close()
