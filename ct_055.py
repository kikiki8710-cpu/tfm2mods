# -*- coding: utf-8 -*-
# ct_055.py — tfm2_comptest_unlock 전 RVA/바이트패치 사이트를 0.5.4 -> 0.5.5 로 재핀.
#   _mig055.py의 match_fn(skeleton)·match_mid(container-delta + orig대조)를 재사용.
import sys, io, importlib.util
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
spec = importlib.util.spec_from_file_location("mig055", r"C:\tfm2mods\_mig055.py")
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)

# ── (A) 바이트패치 + imm 사이트: match_mid(site, orig_hex) ──
BYTES = [
    ("no_stamina_cost",     0x20ecf0c, "05"),
    ("dr_inline_a",         0x2306164, "4c0f44e2"),
    ("dr_inline_b",         0x2310c86, "4120c5"),
    ("dr_inline_d",         0x23ce6bc, "4c0f44f8"),
    ("panel_btn_daily_gate",0x23ceae2, "20c1"),
    ("daily_inc_gate",      0x20e8246, "04"),
    ("server_pregate",      0x20e5471, "04"),
    ("server_dedup_real",   0x2126f73, "0f85d3000000"),
    ("allow_dup_players",   0x2311131, "7547"),
    ("server_dedup",        0x20e42d1, "7510"),
    ("btn5v5_roster_min_a", 0x23ceae4, "4883fb0a0fb6f9b8"),
    ("btn5v5_warn_text",    0x23ce6fc, "4883ff0ab8380000"),
    ("server_roster_min",   0x2126ed0, "4801db"),
    ("roster_count_gate",   0x231e155, "0f834d010000"),
    ("collected_gate",      0x231e142, "7517"),
    ("collect_err_gate",    0x231e127, "7457"),
    ("run_push_gate",       0x231e838, "0f8412faffff"),
    # imm poke 사이트 (disk 상 기본 imm 값)
    ("TAKE_RVA(imm32)",     0x2311d43, "14000000"),
    ("PAGE_IMM_RVA(imm8)",  0x230d0ec, "05"),
]

# ── (B) 함수시작 훅: match_fn ──
FNS = [
    ("RUN_RVA",       0x231de30, "LIVE"),
    ("ORACLE_RVA",    0x13b3150, "LIVE(TICK/CONC_ON)"),
    ("HPUSH_RVA",     0x13006b0, "LIVE(CONC_ON, install_hook_n)"),
    ("CGATE_RVA",     0x2310a90, "LIVE"),
    ("CSEND_RVA",     0x230c910, "LIVE"),
    ("ITEMCONV_RVA",  0x18429d0, "LIVE"),
    ("COLLECT_RVA",   0x18f2b50, "LIVE"),
    ("SREG_RVA",      0x2126b00, "reg_loop cont(§7.5=0x21bee60)"),
    ("MFORGE_RVA",    0x2123590, "CONC_PROBE inert"),
    ("RESULT_RVA",    0x235b270, "CONC_PROBE inert"),
    ("SIMBODY_RVA",   0x237c030, "CONC_PROBE inert"),
    ("POLLER_RVA",    0x148a7c0, "CONC_PROBE inert"),
    ("WARN_RVA",      0x22f6d00, "CONC_PROBE inert(install_hook_n)"),
    ("RPLY3_RVA",     0x2323aa0, "LIVE(replay)"),
    ("LIVEB_RVA",     0x235bf20, "LIVE(replay)"),
    ("RPLY2_RVA",     0x2326820, "RPLY2_ON=false inert"),
    ("REFRESH_RVA",   0x2306000, "LIVE(resultview)"),
    ("ARRIVE_RVA",    0x2327080, "QUEUE_ON=false; ARRIVE_FN_RVA=동일 LIVE"),
    ("SLOT_RVA",      0x1904640, "SIM_PROBE inert"),
    ("FN_DD_SETOPT_RVA",0x1bfc80, "ui_inject drop"),
    ("CTX_CLONE_RVA", 0x23e11e0, "LIVE(replay ctx)"),
    ("CTX_DROP_RVA",  0x22df620, "LIVE(replay ctx)"),
    ("CLONE_CHAMP_RVA",0x193d560,"replay swap(RPLY_ON=false)"),
    ("DROP_CHAMP_RVA",0x182bf30, "replay swap(RPLY_ON=false)"),
    ("EF1EA0_RVA",    0,         "0 dead"),
    ("SUBBODY?RUST_ALLOC_RVA",0x28f7df0,"game_dealloc"),
]

# ── (C) mid-func 훅 + resume 주소: owner+off 델타(orig 있으면 대조) ──
MIDS = [
    ("RPLY_RVA",        0x2323bb2, None,   "RPLY_ON=false inert"),
    ("RPLY_RESUME_RVA", 0x2323bc4, None,   "resume"),
    ("DELAY_RVA",       0x2327094, None,   "LIVE(install_mid; DELAY_ORIG 별도)"),
    ("DELAY_RESUME_RVA",0x23270a3, None,   "resume LIVE"),
    ("EPILOG_RVA",      0x232790a, None,   "epilog LIVE"),
    ("DRIVE_RVA",       0xa289c0,  None,   "LIVE(install_mid)"),
    ("DRIVE_RESUME_RVA",0xa289d6,  None,   "resume LIVE"),
    ("A15E20_RVA",      0xa15e20,  None,   "drive helper"),
    ("RUNNER_VT_RVA",   0x33b91f8, None,   ".rdata vtable(match_fn 불가)"),
]

def owner_delta(site):
    own = m.O.owner(site)
    if own is None: return None
    res, nown, note = m.match_fn(own)
    if isinstance(nown, int) and res.startswith("UNIQUE"):
        return (own, nown, site-own, nown+(site-own), res)
    return (own, None, site-own, None, res+" "+str(note))

print("="*70); print("[A] 바이트패치 + imm 사이트  (match_mid + orig 대조)"); print("="*70)
for name, site, oh in BYTES:
    st, ns, info = m.match_mid(site, oh)
    if isinstance(info, dict):
        ok = info.get("orig_match")
        print(f"{name:24s} 0x{site:x} -> 0x{info['new_site']:x}  [{st}] "
              f"off=0x{info['off']:x} old_orig={info.get('old_orig')} new={info.get('new_bytes')} MATCH={ok}")
    else:
        print(f"{name:24s} 0x{site:x} -> ???  [{st}] {info}")

print(); print("="*70); print("[B] 함수시작 훅  (match_fn skeleton)"); print("="*70)
for name, rva, tag in FNS:
    if rva == 0:
        print(f"{name:24s} 0x0 (dead)  [{tag}]"); continue
    st, nr, note = m.match_fn(rva)
    nrs = f"0x{nr:x}" if isinstance(nr,int) else str(nr)
    print(f"{name:24s} 0x{rva:x} -> {nrs}  [{st}] {note}   <{tag}>")

print(); print("="*70); print("[C] mid-func 훅 + resume  (owner-delta)"); print("="*70)
for name, site, oh, tag in MIDS:
    r = owner_delta(site)
    if r is None:
        print(f"{name:24s} 0x{site:x} -> NO_OWNER  <{tag}>"); continue
    own, nown, off, nsite, res = r
    nss = f"0x{nsite:x}" if nsite else "???"
    nos = f"0x{nown:x}" if nown else "None"
    print(f"{name:24s} 0x{site:x} -> {nss}  owner 0x{own:x}->{nos} off=0x{off:x} [{res}]  <{tag}>")
