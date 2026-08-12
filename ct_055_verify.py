# -*- coding: utf-8 -*-
# ct_055_verify.py — 0.5.5 exe 온디스크 바이트가 소스 선언 orig 와 일치하는지 검증.
import struct, sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
P5 = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.5\TeamfightManager2.exe"
d = open(P5,"rb").read()
pe = struct.unpack_from("<I", d, 0x3c)[0]; nsec = struct.unpack_from("<H", d, pe+6)[0]
opt = pe+24; sectab = opt + struct.unpack_from("<H", d, pe+20)[0]; secs=[]
for i in range(nsec):
    o=sectab+i*40; nm=d[o:o+8].rstrip(b"\0").decode(errors="replace")
    vsz,va,rsz,rraw=struct.unpack_from("<IIII",d,o+8); secs.append((va,vsz,rraw,rsz))
def roff(rva):
    for va,vsz,rraw,rsz in secs:
        if va<=rva<va+max(vsz,rsz):
            off=rva-va; return rraw+off if off<rsz else None
# (name, rva, orig_hex)  — 소스 선언과 동일해야 함
T=[("no_stamina_cost",0x2185b96,"05"),("dr_inline_a",0x1a8ab76,"4c0f44e2"),
   ("dr_inline_b",0x1a95776,"4120c5"),("dr_inline_d",0x1afdf6c,"4c0f44f8"),
   ("panel_btn_daily_gate",0x1afe392,"20c1"),("daily_inc_gate",0x2180f84,"04"),
   ("server_pregate",0x217e1ac,"04"),("server_dedup_real",0x21bf2d3,"0f85d3000000"),
   ("allow_dup_players",0x1a95c21,"7547"),("server_dedup",0x217d01f,"7510"),
   ("btn5v5_roster_min_a",0x1afe394,"4883fb0a0fb6f9b8"),("btn5v5_warn_text",0x1afdfac,"4883ff0ab8380000"),
   ("server_roster_min",0x21bf230,"4801db"),("roster_count_gate",0x1aa2c28,"0f8361010000"),
   ("collected_gate",0x1aa2c1c,"7510"),("collect_err_gate",0x1aa2bff,"7452"),
   ("run_push_gate",0x1aa3325,"0f8405faffff"),("TAKE_RVA",0x1a9683a,"14000000"),
   ("PAGE_IMM_RVA",0x1a91bcc,"05")]
allok=True
for name,rva,oh in T:
    o=roff(rva); got=d[o:o+len(oh)//2].hex()
    ok = got==oh.lower(); allok &= ok
    print(f"{'OK ' if ok else 'FAIL'} {name:22s} 0x{rva:x} disk={got} decl={oh}")
# function-start 프롤로그 확인(주요 훅)
def prol(rva,n=12):
    o=roff(rva); return d[o:o+n].hex()
PUSH8="554157415641554154565753"
print("--- 함수시작 프롤로그 ---")
for name,rva,exp in [("RUN",0x1aa2930,PUSH8),("CGATE",0x1a95570,PUSH8),("CSEND",0x1a913a0,PUSH8),
                     ("REFRESH",0x1a8aa10,PUSH8),("RPLY3",0x1aa8370,PUSH8),("ORACLE",0x14aa160,PUSH8),
                     ("LIVEB",0x1aece30,PUSH8),("CTX_CLONE",0x1cb5390,PUSH8),
                     ("LOADER",0x2e42d0,"5541574156415541545657534881ec98"),
                     ("HPUSH",0x16e3890,"5556574883ec60488d6c2460"),
                     ("ARRIVE",0x1aab950,"55415741564154565753b8c0420000")]:
    p=prol(rva,len(exp)//2); print(f"{'OK ' if p==exp else 'CHK'} {name:10s} 0x{rva:x} {p}")
print("\n=== ALL BYTEPATCH OK ===" if allok else "\n=== 실패 있음 ===")
