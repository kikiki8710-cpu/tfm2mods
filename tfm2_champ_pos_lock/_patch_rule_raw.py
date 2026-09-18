# -*- coding: utf-8 -*-
# 09-17: observe_rule_from_option(UI 선택 상태) → observe_rule_raw(ClientDatabase GamePlayOption raw) 교체 패치
import os
os.chdir(os.path.dirname(os.path.abspath(__file__)))
p = 'src/lib.rs'
s = open(p, encoding='utf-8').read()
uk = '#[path = r"C:\\tfm2mods\\ui_kit\\ui_kit_stable.rs"]\npub mod uk;'
assert uk in s
if 'pub mod cdb;' not in s:
    s = s.replace(uk, uk + '\n#[path = r"C:\\tfm2mods\\ui_kit\\client_db_stable.rs"]\npub mod cdb;', 1)
start = s.index('/// 옵션 화면의 밴픽 스타일/밴 수 선택값 → config::set_rule.')
end = s.index('struct Ext;')
new = r'''/// ★0.6.0(09-17, RE `RE\2026-09-17_0.6.0-GamePlayOption-…`): 옵션 값을 UI 로 못 읽음(pause 팝업 안 selectable = None) →
/// ClientDatabase 의 GamePlayOption 을 raw 로 읽는다. banpick_style u8 cdb+0x738(0/1/2) · room_practice_ban_count u64 cdb+0x720
/// (0=기본) · available len cdb+0xe750. 실효 밴 수 = 게임 공식(5v5): (1≤n≤5 && avail ≥ 2n+20·style+15) ? n : default,
/// default = (avail≥40 ? 3 : 2), avail < 2·default+20·style+15 면 default=2. 옵션 화면이 아니어도 120프레임마다 관측(밴픽 전 확정).
const OFF_BAN_COUNT: usize = 0x720;
const OFF_TWO_PHASE: usize = 0x733;
const OFF_BANPICK_STYLE: usize = 0x738;
const OFF_AVAIL_LEN: usize = 0xe750;
static RULE_LOGGED: AtomicU64 = AtomicU64::new(u64::MAX);
fn observe_rule_raw(ctx: &StableClient<'_>) {
    let Some(db) = cdb::client_db(ctx) else { return };
    let (style, n, two, avail) = unsafe {
        (cdb::rd_u32(db + OFF_BANPICK_STYLE).map(|v| (v & 0xff) as u8), cdb::rd_u64(db + OFF_BAN_COUNT),
         cdb::rd_u32(db + OFF_TWO_PHASE).map(|v| (v & 0xff) as u8), cdb::rd_u64(db + OFF_AVAIL_LEN))
    };
    let (Some(style), Some(n), Some(two), Some(avail)) = (style, n, two, avail) else { return };
    if style > 2 || n > 64 || two > 1 || avail > 4096 { return; } // 레이아웃 stale 가드
    let n = n as usize; let avail = avail as usize; let st = style as usize;
    let mut default = if avail >= 40 { 3 } else { 2 };
    if avail < 2 * default + 20 * st + 15 { default = 2; }
    let eff = if (1..=5).contains(&n) && avail >= 2 * n + 20 * st + 15 { n } else { default };
    let sig = ((style as u64) << 48) | ((n as u64) << 32) | ((two as u64) << 24) | ((eff as u64) << 16) | (avail as u64 & 0xffff);
    if RULE_LOGGED.swap(sig, Ordering::Relaxed) != sig { dlog(&format!("룰 raw: style={} ban_opt={}(0=기본) two_phase={} avail={} → 실효 밴 {}", style, n, two, avail, eff)); }
    let (cs, cb) = config::cur_rule();
    if cs != style || cb != Some(eff) { config::set_rule(style, Some(eff)); }
}

'''
s = s[:start] + new + s[end:]
old = '''            if let Some(contents) = option_contents(ctx) {
                observe_rule_from_option(ctx, &contents);
                ui_popup::tick(ctx, &contents);
            } else { ui_popup::hidden(); }'''
new2 = '''            if FRAME.load(Ordering::Relaxed) % 120 == 0 { observe_rule_raw(ctx); }
            if let Some(contents) = option_contents(ctx) {
                ui_popup::tick(ctx, &contents);
            } else { ui_popup::hidden(); }'''
assert old in s
s = s.replace(old, new2)
open(p, 'w', encoding='utf-8').write(s)
print('ok')
