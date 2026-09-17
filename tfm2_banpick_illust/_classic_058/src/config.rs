//! tfm2_banpick_illust 설정 — `mods\tfm2_banpick_illust\tfm2_banpick_illust.cfg`.
//! 없으면 기본값으로 자동 생성. 값 수정 후 게임 재시작.

use std::sync::OnceLock;

#[derive(Clone, Copy)]
pub struct Cfg {
    /// 기능 마스터 스위치.
    pub enabled: bool,
    /// 레드 진영에 좌우반전 구도를 쓸지. 1 = `illust/red/`, 0 = `illust/red_noflip/`.
    /// (반전은 아트에 이미 반영돼 있어 폴더 선택으로 처리한다 — 엔진 flip 미사용.)
    pub red_flip: bool,
    /// 픽 확정 후 확대 구도(`<champ>-1.png`)를 쓸지.
    pub zoom: bool,
    /// 하단 정보 영역(이름·숙련도)에 회색 반투명 띠를 깔지. 밝은 일러스트에서
    /// 글자가 묻히는 걸 막는다(2026-07-20 요청). 전체를 덮던 예전 방식은 그림이
    /// 탁해져서 하단 띠로 바꿨다. 크기·색은 `.ui` 의 `#bpi_dim`.
    pub dim: bool,
    /// 일러스트가 슬롯을 채우므로 바닐라 소형 초상화·구분선 숨김.
    pub hide_portrait: bool,
    /// 픽 완료 슬롯 하단에 이름/숙련도 보너스/%/숙련도 챔프 초상화를 한 줄로 재배치.
    pub move_info_row: bool,
    /// 밴/픽 셀렉트 연출 카드(중앙에 떴다가 슬롯으로 가는 카드)에도 일러스트 적용
    /// (가로형 카드 + 밴 분할 연출 확대 — showcase.rs, 훅·바이트패치 사용).
    pub showcase: bool,
    /// 버프/너프 이름 색칠 (밸런스 패치로 오른 챔프 초록 / 내린 챔프 빨강).
    pub name_color: bool,
    /// (제거된 기능 — 하단 스탯 패널 수치 색칠. 유저 판단으로 비활성, 키만 잔존.)
    pub stat_color: bool,
    /// 호버 챔프 8각 스탯 레이더 (원래 능력치 대비 증감).
    pub radar: bool,
    /// 레이더 중심 좌표 (-1 = 자동: 화면 우하단). 화면 논리 px.
    pub radar_x: f32,
    pub radar_y: f32,
    /// 레이더 크기 배율.
    pub radar_scale: f32,
    /// 챔프 아이콘 source 실관측값을 illust_debug.txt 로 덤프(1회성 진단용).
    pub debug: bool,
    /// 쓰기 경로 시험 모드 — 챔프 감지와 무관하게 강제로 이미지를 넣는다(진단 전용).
    pub test_mode: bool,
}

impl Default for Cfg {
    fn default() -> Self {
        Cfg {
            enabled: true,
            red_flip: true,
            zoom: false,
            dim: true,
            hide_portrait: true,
            move_info_row: true,
            showcase: true,
            name_color: true,
            stat_color: false,
            radar: true,
            radar_x: -1.0,
            radar_y: -1.0,
            radar_scale: 1.0,
            debug: false,
            test_mode: false,
        }
    }
}

static CFG: OnceLock<Cfg> = OnceLock::new();

pub fn get() -> Cfg {
    *CFG.get_or_init(Cfg::default)
}

const TEMPLATE: &str = "\
# tfm2_banpick_illust 설정 — 값 수정 후 게임 재시작.
# 밴픽에서 픽이 확정된 슬롯 배경에 챔피언 스플래시 일러스트를 깔아줍니다.

# 기능 켬/끔 (1=켬, 0=끔)
enabled        = 1

# 레드 진영 구도: 1 = illust/red/ (좌우반전 구도), 0 = illust/red_noflip/
red_flip       = 1

# 확대 구도 사용: 1 = <챔프>-1.png (피사체를 크게 잡은 버전), 0 = 기본 구도
zoom           = 0

# 하단 정보 영역에 회색 반투명 띠 깔기 (글자가 일러스트에 묻히는 걸 방지)
# 크기·색 조절은 ui/layout/banpick/*.ui 의 #bpi_dim
dim            = 1

# 일러스트가 슬롯을 채우므로 바닐라 소형 초상화/구분선 숨김 (1=숨김)
hide_portrait  = 1

# 픽 완료 슬롯 하단에 이름/숙련도보너스/%/숙련도챔프 초상화를 한 줄로 재배치 (1=켬)
move_info_row  = 1

# 밴/픽 셀렉트 연출 카드(중앙에 떴다가 슬롯으로 날아가는 카드)에도 일러스트 적용 (1=켬)
# 가로형 카드로 재구성 + 밴 분할 연출 확대. 일러 없는 챔프는 도트가 진영 방향에 맞게 나옴.
showcase       = 1

# 버프/너프 가시성 (밸런스 패치로 스탯이 오른 챔프 = 초록, 내린 챔프 = 빨강)
# name_color: 밴픽 화면의 챔피언 이름 색칠 (1=켬)
name_color     = 1

# radar: 하단 스탯 패널에 챔프가 표시될 때 8각 스탯 레이더 표시 (1=켬)
# 축 = 공격/주문력/공속/체력/방어/마저/사거리/이속, 기준 8각형(안쪽 링) = 변화 없음
radar          = 1

# 레이더 중심 위치·크기 (radar_x/y = -1 이면 자동: 화면 우하단)
radar_x        = -1
radar_y        = -1
radar_scale    = 1.0

# 진단용: 챔피언 아이콘의 실제 에셋 경로를 illust_debug.txt 로 덤프 (평소 0)
debug          = 0

# 진단 전용: 챔프 감지 없이 강제로 이미지를 넣어 쓰기 경로를 시험 (평소 0)
test_mode      = 0
";

/// cfg 를 읽어 CFG 초기화. 파일이 없으면 기본 템플릿 생성.
pub fn load() {
    let mut c = Cfg::default();
    if let Some(dir) = crate::mod_dir() {
        let path = format!("{dir}\\{}.cfg", crate::MOD_ID);
        match std::fs::read_to_string(&path) {
            Ok(s) => parse(&s, &mut c),
            Err(_) => {
                let _ = std::fs::create_dir_all(&dir);
                let _ = std::fs::write(&path, TEMPLATE);
            }
        }
    }
    let _ = CFG.set(c);
}

fn parse(s: &str, c: &mut Cfg) {
    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (k, v) = match line.split_once('=') {
            Some(kv) => kv,
            None => continue,
        };
        let (k, v) = (k.trim(), v.trim());
        let on = v == "1" || v.eq_ignore_ascii_case("true");
        match k {
            "enabled" => c.enabled = on,
            "red_flip" => c.red_flip = on,
            "zoom" => c.zoom = on,
            "hide_portrait" => c.hide_portrait = on,
            "move_info_row" => c.move_info_row = on,
            "showcase" => c.showcase = on,
            "name_color" => c.name_color = on,
            "stat_color" => c.stat_color = on,
            "radar" => c.radar = on,
            "radar_x" => c.radar_x = v.parse().unwrap_or(-1.0),
            "radar_y" => c.radar_y = v.parse().unwrap_or(-1.0),
            "radar_scale" => c.radar_scale = v.parse().unwrap_or(1.0),
            "debug" => c.debug = on,
            "test_mode" => c.test_mode = on,
            "dim" => c.dim = on,
            _ => {}
        }
    }
}

/// 진단 파일에 append (300KB 넘으면 리셋).
pub fn append_diag(name: &str, body: &str) {
    if let Some(dir) = crate::mod_dir() {
        let _ = std::fs::create_dir_all(&dir);
        let p = format!("{dir}\\{name}");
        if std::fs::metadata(&p).map(|m| m.len() > 300_000).unwrap_or(false) {
            let _ = std::fs::remove_file(&p);
        }
        use std::io::Write;
        let _ = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&p)
            .and_then(|mut f| f.write_all(body.as_bytes()));
    }
}

/// 진단 파일을 모드 폴더에 쓴다(덮어쓰기).
pub fn write_diag(name: &str, body: &str) {
    if let Some(dir) = crate::mod_dir() {
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::write(format!("{dir}\\{name}"), body);
    }
}

/// 픽 연출 후보 노드 덤프 — 관측된 경로를 **누적**해서 남긴다(연출은 한순간이라
/// 덮어쓰면 놓친다).
pub fn stray_dump(lines: &[String]) {
    static SEEN: OnceLock<std::sync::Mutex<Vec<String>>> = OnceLock::new();
    let m = SEEN.get_or_init(|| std::sync::Mutex::new(Vec::new()));
    let mut g = m.lock().unwrap_or_else(|e| e.into_inner());
    let before = g.len();
    for l in lines {
        if !g.contains(l) {
            g.push(l.clone());
        }
    }
    if g.len() == before {
        return;
    }
    let body = format!(
        "# 픽 슬롯/챔피언목록 바깥에서 관측된 챔피언 스프라이트 (픽 연출 노드 후보)
{}
",
        g.join("
")
    );
    write_diag("anim_probe.txt", &body);
}

/// 진단 덤프 — **내용이 바뀔 때만** 다시 기록한다.
///
/// 예전엔 프로세스당 1회만 썼는데, 그러면 첫 프레임 스냅샷 하나만 남아서
/// "픽할 때마다 무엇이 바뀌는가" 라는 정작 필요한 정보가 안 잡혔다.
pub fn debug_dump(lines: &[String]) {
    static LAST: OnceLock<std::sync::Mutex<String>> = OnceLock::new();
    let joined = lines.join("\n");
    {
        let m = LAST.get_or_init(|| std::sync::Mutex::new(String::new()));
        let mut g = m.lock().unwrap_or_else(|e| e.into_inner());
        if *g == joined {
            return;
        }
        *g = joined;
    }
    if let Some(dir) = crate::mod_dir() {
        let mut out = String::from("# 챔피언 아이콘 image source 실관측값\n");
        let mut seen: Vec<&String> = Vec::new();
        for l in lines {
            if !seen.contains(&l) {
                seen.push(l);
                out.push_str(l);
                out.push('\n');
            }
        }
        let _ = std::fs::write(format!("{dir}\\illust_debug.txt"), out);
    }
}
