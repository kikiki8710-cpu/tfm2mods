#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//! TFM2 모의 밴픽 도우미 — 독립 egui 앱.
//! draft_data.json(추출기 산출)을 읽어, 드래프트 진행에 맞춰 밴/픽을 추천한다.
//!  - 엔진 A: 게임 내장 밴픽 AI 재현(승률예측)  - 엔진 B: 베이지안 메타 통계
//! exe 옆 또는 작업폴더의 draft_data.json 을 자동 탐색.

mod model;
mod engine;
mod draft;

use eframe::egui;
use model::DraftData;
use draft::{Draft, Kind, Side, Step};

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    for path in [
        "C:/Windows/Fonts/malgun.ttf",
        "C:/Windows/Fonts/malgunsl.ttf",
        "C:/Windows/Fonts/gulim.ttc",
    ] {
        if let Ok(bytes) = std::fs::read(path) {
            fonts
                .font_data
                .insert("kr".to_owned(), egui::FontData::from_owned(bytes).into());
            fonts
                .families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "kr".to_owned());
            fonts
                .families
                .get_mut(&egui::FontFamily::Monospace)
                .unwrap()
                .insert(0, "kr".to_owned());
            break;
        }
    }
    ctx.set_fonts(fonts);
}

fn resolve_data_path() -> Option<String> {
    let mut cands: Vec<std::path::PathBuf> = vec![];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            cands.push(dir.join("draft_data.json"));
        }
    }
    cands.push(std::path::PathBuf::from("draft_data.json"));
    cands.push(std::path::PathBuf::from(
        r"C:\tfm2mods\draft_helper\draft_data.json",
    ));
    for c in cands {
        if c.exists() {
            return Some(c.to_string_lossy().to_string());
        }
    }
    None
}

struct App {
    data: Option<DraftData>,
    err: String,
    draft: Draft,
    current: Step,
    blend_a: f32, // 게임AI 비중 0..1
    filter: String,
}

impl App {
    fn new() -> Self {
        let (data, err) = match resolve_data_path() {
            Some(p) => match DraftData::load(&p) {
                Ok(d) => (Some(d), String::new()),
                Err(e) => (None, e),
            },
            None => (
                None,
                "draft_data.json 을 찾을 수 없습니다. export/export_draft_data.py 를 먼저 실행하세요.".into(),
            ),
        };
        Self {
            data,
            err,
            draft: Draft::default(),
            current: Step { side: Side::My, kind: Kind::Ban },
            blend_a: 0.6,
            filter: String::new(),
        }
    }

    fn assign(&mut self, champ: &str) {
        let step = self.current;
        if self.draft.assign(step, champ) {
            self.current = self.draft.suggest_next(step);
        }
    }
}

fn side_kind_label(s: Step) -> &'static str {
    match (s.side, s.kind) {
        (Side::My, Kind::Ban) => "내 밴",
        (Side::My, Kind::Pick) => "내 픽",
        (Side::Enemy, Kind::Ban) => "상대 밴",
        (Side::Enemy, Kind::Pick) => "상대 픽",
    }
}

fn tier_color(t: &str) -> egui::Color32 {
    match t {
        "OP" => egui::Color32::from_rgb(255, 90, 90),
        "1" => egui::Color32::from_rgb(255, 170, 60),
        "2" => egui::Color32::from_rgb(240, 220, 80),
        "3" => egui::Color32::from_rgb(140, 200, 240),
        _ => egui::Color32::from_gray(150),
    }
}

impl eframe::App for App {
    fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let Some(data) = self.data.clone() else {
            egui::CentralPanel::default().show_inside(root, |ui| {
                ui.heading("TFM2 밴픽 도우미");
                ui.add_space(8.0);
                ui.colored_label(egui::Color32::from_rgb(240, 120, 120), &self.err);
            });
            return;
        };

        // ── 상단 바 ──
        egui::TopBottomPanel::top("top").show_inside(root, |ui| {
            ui.horizontal(|ui| {
                ui.heading("TFM2 밴픽 도우미");
                ui.separator();
                ui.label(format!(
                    "경기 {} · 챔프 {}",
                    data.meta.total_matches, data.meta.champion_count
                ));
                ui.separator();
                if ui.button("⟲ 초기화").clicked() {
                    self.draft.reset();
                    self.current = Step { side: Side::My, kind: Kind::Ban };
                }
                if ui.button("↶ 되돌리기").clicked() {
                    self.draft.undo();
                }
            });
            ui.horizontal(|ui| {
                ui.label("추천 가중치:");
                ui.label("메타");
                ui.add(egui::Slider::new(&mut self.blend_a, 0.0..=1.0).show_value(false));
                ui.label("게임AI");
                ui.label(format!("(게임AI {:.0}%)", self.blend_a * 100.0));
                ui.separator();
                ui.label("현재 차례:");
                for st in [
                    Step { side: Side::My, kind: Kind::Ban },
                    Step { side: Side::My, kind: Kind::Pick },
                    Step { side: Side::Enemy, kind: Kind::Ban },
                    Step { side: Side::Enemy, kind: Kind::Pick },
                ] {
                    let on = self.current.side == st.side && self.current.kind == st.kind;
                    if ui.selectable_label(on, side_kind_label(st)).clicked() {
                        self.current = st;
                    }
                }
            });
        });

        // ── 좌: 내 팀 ──
        egui::SidePanel::left("my").resizable(false).exact_width(170.0).show_inside(root, |ui| {
            team_panel(ui, &data, "내 팀", &self.draft.my_bans, &self.draft.my_picks, true);
        });
        // ── 우: 상대 팀 ──
        egui::SidePanel::right("en").resizable(false).exact_width(170.0).show_inside(root, |ui| {
            team_panel(ui, &data, "상대 팀", &self.draft.en_bans, &self.draft.en_picks, false);
        });

        // ── 중앙: 추천 + 챔프 그리드 ──
        egui::CentralPanel::default().show_inside(root, |ui| {
            let cur = self.current;
            let for_my = cur.side == Side::My;
            let is_ban = cur.kind == Kind::Ban;

            ui.horizontal(|ui| {
                ui.heading(format!("▶ {} 추천", side_kind_label(cur)));
                if self.draft.complete() {
                    ui.colored_label(egui::Color32::LIGHT_GREEN, "드래프트 완료");
                }
            });
            ui.label(
                egui::RichText::new(if is_ban {
                    "상대에게 위협적인(강한) 챔프를 밴하세요."
                } else {
                    "지금 팀에 가장 가치 높은 챔프를 픽하세요."
                })
                .small()
                .color(egui::Color32::from_gray(160)),
            );
            ui.add_space(4.0);

            let used = self.draft.used();
            let available: Vec<String> = data
                .champions
                .iter()
                .filter(|c| !used.contains(&c.id))
                .map(|c| c.id.clone())
                .collect();

            let recos = engine::recommend(
                &data,
                &available,
                &self.draft.my_picks,
                &self.draft.en_picks,
                for_my,
                is_ban,
                self.blend_a,
            );

            // 추천 상위 목록
            let mut click: Option<String> = None;
            egui::ScrollArea::vertical()
                .id_salt("reco")
                .max_height(260.0)
                .show(ui, |ui| {
                    for (i, r) in recos.iter().take(12).enumerate() {
                        let nm = data.name(&r.id);
                        let frame = egui::Frame::group(ui.style()).inner_margin(6.0);
                        frame.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                let rank = if i == 0 {
                                    egui::RichText::new("⭐1").strong().color(egui::Color32::GOLD)
                                } else {
                                    egui::RichText::new(format!("{}", i + 1)).weak()
                                };
                                ui.label(rank);
                                if ui.button(egui::RichText::new(&nm).strong()).clicked() {
                                    click = Some(r.id.clone());
                                }
                                ui.colored_label(tier_color(r.tier), format!("[{}]", r.tier));
                                ui.label(
                                    egui::RichText::new(format!("종합 {:.0}", r.blended))
                                        .strong()
                                        .color(egui::Color32::from_rgb(120, 200, 255)),
                                );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "게임AI {:.0} · 메타 {:.0}",
                                        r.score_a, r.score_b
                                    ))
                                    .small()
                                    .weak(),
                                );
                            });
                            // 분해 (게임AI 기여)
                            ui.label(
                                egui::RichText::new(format!(
                                    "   솔로 {:+.2} · 포지션 {:+.2} · 시너지 {:+.2} · 카운터 {:+.2}",
                                    r.bd.solo, r.bd.pos, r.bd.syn, r.bd.ctr
                                ))
                                .small()
                                .color(egui::Color32::from_gray(150)),
                            );
                        });
                    }
                });
            if let Some(c) = click {
                self.assign(&c);
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("챔프 검색:");
                ui.text_edit_singleline(&mut self.filter);
                ui.label(
                    egui::RichText::new("(클릭 = 현재 차례 슬롯에 배정)")
                        .small()
                        .weak(),
                );
            });

            // 챔프 그리드 (이름순/검색필터, 사용된 챔프 흐리게)
            let filt = self.filter.trim().to_lowercase();
            let mut grid_click: Option<String> = None;
            egui::ScrollArea::vertical().id_salt("grid").show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for c in &data.champions {
                        if !filt.is_empty()
                            && !c.name.to_lowercase().contains(&filt)
                            && !c.id.to_lowercase().contains(&filt)
                        {
                            continue;
                        }
                        let is_used = used.contains(&c.id);
                        let btn = egui::Button::new(&c.name).min_size(egui::vec2(78.0, 24.0));
                        let resp = ui.add_enabled(!is_used, btn);
                        if resp.clicked() {
                            grid_click = Some(c.id.clone());
                        }
                    }
                });
            });
            if let Some(c) = grid_click {
                self.assign(&c);
            }
        });
    }
}

fn team_panel(
    ui: &mut egui::Ui,
    data: &DraftData,
    title: &str,
    bans: &[String],
    picks: &[String],
    mine: bool,
) {
    let col = if mine {
        egui::Color32::from_rgb(120, 180, 255)
    } else {
        egui::Color32::from_rgb(255, 140, 140)
    };
    ui.heading(egui::RichText::new(title).color(col));
    ui.add_space(4.0);
    ui.label(egui::RichText::new("밴").strong().color(egui::Color32::from_gray(170)));
    for i in 0..draft::BANS_PER_TEAM {
        let s = bans.get(i).map(|c| data.name(c)).unwrap_or_else(|| "—".into());
        ui.label(egui::RichText::new(format!("  ✕ {}", s)).color(egui::Color32::from_gray(140)));
    }
    ui.add_space(6.0);
    ui.label(egui::RichText::new("픽").strong().color(col));
    for i in 0..draft::PICKS_PER_TEAM {
        match picks.get(i) {
            Some(c) => {
                let st = data.stat(c);
                ui.label(format!("  {}. {}  ({:.0}%)", i + 1, data.name(c), st.winrate * 100.0));
            }
            None => {
                ui.label(egui::RichText::new(format!("  {}. —", i + 1)).color(egui::Color32::from_gray(110)));
            }
        }
    }
}

/// 헤드리스 검증: 실데이터로 추천 산출을 콘솔에 덤프 (GUI 없이 엔진 점검).
fn run_dump() {
    let path = resolve_data_path().expect("draft_data.json 없음");
    let d = DraftData::load(&path).expect("load 실패");
    println!("=== 데이터: 경기 {} · 챔프 {} ===", d.meta.total_matches, d.champions.len());
    let all: Vec<String> = d.champions.iter().map(|c| c.id.clone()).collect();

    let show = |title: &str, recos: &[engine::Reco]| {
        println!("\n[{}] 상위 6:", title);
        for (i, r) in recos.iter().take(6).enumerate() {
            println!(
                "  {}. {:<10} 종합 {:5.1}  (게임AI {:5.1} / 메타 {:5.1} [{}])  솔로{:+.2} 포지션{:+.2} 시너지{:+.2} 카운터{:+.2}",
                i + 1, d.name(&r.id), r.blended, r.score_a, r.score_b, r.tier,
                r.bd.solo, r.bd.pos, r.bd.syn, r.bd.ctr
            );
        }
    };

    // 1) 빈 드래프트에서 내 밴 추천
    let r = engine::recommend(&d, &all, &[], &[], true, true, 0.6);
    show("빈 상태 · 내 밴 추천(상대에게 강한 챔프)", &r);

    // 2) 빈 드래프트에서 내 픽 추천
    let r = engine::recommend(&d, &all, &[], &[], true, false, 0.6);
    show("빈 상태 · 내 픽 추천", &r);

    // 3) 상대가 'gambler','priest' 픽 / 내가 'soldier' 픽한 상태에서 내 다음 픽
    let mypicks = vec!["soldier".to_string()];
    let enemypicks = vec!["gambler".to_string(), "priest".to_string()];
    let used: std::collections::HashSet<String> =
        mypicks.iter().chain(enemypicks.iter()).cloned().collect();
    let avail: Vec<String> = all.iter().filter(|c| !used.contains(*c)).cloned().collect();
    let r = engine::recommend(&d, &avail, &mypicks, &enemypicks, true, false, 0.6);
    show("내=[soldier] 상대=[gambler,priest] · 내 다음 픽(시너지+카운터 반영)", &r);
}

fn main() -> eframe::Result<()> {
    if std::env::args().any(|a| a == "--dump") {
        run_dump();
        return Ok(());
    }
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1220.0, 820.0]),
        ..Default::default()
    };
    eframe::run_native(
        "TFM2 밴픽 도우미",
        native_options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(App::new()))
        }),
    )
}
