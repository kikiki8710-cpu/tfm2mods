//! 게임 팀전술 창에 버튼 + 종류별 탭과 포지션별 상세전술 모달.
use crate::{tactics::{self,Kind,Scope},save,presentation::{self,CATEGORIES}};
use mod_api_stable::StableClient;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool,Ordering::Relaxed};
#[path="../../ui_kit/dropdown_stable.rs"] mod dd;
const POP:&str="main.dt_modal";
const PARENT:&str="main.contents.strategy";
struct Editor { open:bool, tab:usize, draft:[[u8;6];tactics::N], dirty:bool, dropdown:Option<(usize,usize)> }
impl Default for Editor { fn default()->Self{Self{open:false,tab:0,draft:[[0;6];tactics::N],dirty:true,dropdown:None}} }
static EDIT:Mutex<Editor>=Mutex::new(Editor{open:false,tab:0,draft:[[0;6];tactics::N],dirty:true,dropdown:None});
static DROPDOWN_OPEN:AtomicBool=AtomicBool::new(false);
const ITEM_IDS:[&str;3]=["lv0","lv1","lv2"];
// UI 전용 칸. 저장·적용은 기존 원딜(3), 서폿(4) 슬롯을 그대로 쓴다.
const COL_DUO:usize=6;
fn cell_id(row:usize,col:usize)->String{match col{5=>format!("team{row}"),COL_DUO=>format!("duo{row}"),_=>format!("c{row}_{col}")}}
fn row_path(tab:usize,row:usize,col:usize)->String{format!("{POP}.page{tab}.contents.{}",cell_id(row,col))}
fn choice_id(col:usize)->&'static str{match col{5=>"choices_team",COL_DUO=>"choices_duo",_=>"choices_player"}}
fn cell_visible(t:usize,col:usize)->bool{
    let d=&tactics::TACTICS[t];
    if d.id=="bot_duo_partner"{col==COL_DUO}else if d.scope==Scope::Team{col==5}else{presentation::player_visible(d.id,col)}
}
impl Editor {
    fn tactic(&self,row:usize)->Option<usize>{presentation::tactic_at(self.tab,row)}
    fn switch_tab(&mut self,tab:usize){
        if tab>=CATEGORIES.len(){return;}
        close_dropdown(self);self.tab=tab;self.dirty=true;
    }
    fn select(&mut self,row:usize,col:usize,lv:u8)->bool{
        let Some(t)=self.tactic(row)else{return false;};
        if !cell_visible(t,col)||lv>=if tactics::TACTICS[t].kind==Kind::Toggle{2}else{3}{return false;}
        if col==COL_DUO{self.draft[t][3]=lv;self.draft[t][4]=lv;}else{self.draft[t][col]=lv;}
        close_dropdown(self);self.dirty=true;true
    }
}
fn cell_level(e:&Editor,t:usize,col:usize)->Option<u8>{
    if col==COL_DUO{(e.draft[t][3]==e.draft[t][4]).then_some(e.draft[t][3])}else{Some(e.draft[t][col])}
}
fn cell_choice(t:usize,lv:Option<u8>)->String{lv.map(|lv|choice(t,lv)).unwrap_or_else(||"서로 다름 · 다시 선택".into())}
fn close_dropdown(e:&mut Editor){e.dropdown=None;DROPDOWN_OPEN.store(false,Relaxed);}
static FIRED:Mutex<Option<std::collections::HashMap<String,u64>>>=Mutex::new(None);
fn text(ctx:&mut StableClient<'_>,p:&str,s:&str){if ctx.ui_text(p).as_deref()!=Some(s){ctx.ui_set_text(p,s);}}
fn button(id:&str,x:usize,y:usize,w:usize,label:&str)->String{
    format!("#{id}:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x:{x}px; y:{y}px; width:{w}px; height:38px; #caption:label {{ ignore_event:true; width:100%; height:100%; text:\"{label}\"; font:\"asset/base/font/set/bold\"; size:16; align_x:Center; align_y:Center; }} }}\n")
}
fn label(id:&str,x:usize,y:usize,w:usize,h:usize,size:usize,s:&str)->String{
    format!("#{id}:label {{ @\"asset/base/style/main#label\"; x:{x}px; y:{y}px; width:{w}px; height:{h}px; size:{size}; align_y:Center; text:\"{s}\"; }}\n")
}
// 공용 UI kit 출력의 글자 영역만 대칭 여백으로 바꿔 칸 전체의 중앙에 맞춘다.
// 오른쪽 화살표·선택 체크 공간은 남기며 다른 모드의 풀다운은 건드리지 않는다.
fn centered_dropdown(src:String,w:usize,kit_text_inset:usize)->String{
    src.replace("x: 14px;","x: 40px;")
        .replace(&format!("width: {}px;",w-kit_text_inset),&format!("width: {}px;",w-80))
        .replace("align_y: Center;","align_x: Center; align_y: Center;")
}
fn source()->String{
    let mut s=String::from("dt_modal:color { width:1740px; height:970px; anchor_x:0.5; pivot_x:0.5; anchor_y:0.5; pivot_y:0.5; visible:false; color:#161721ff; rounding:Uniform{rounding:12;}\n");
    s+=&label("title",30,18,1450,42,26,"상세전술");
    s+=&label("hint",30,63,1680,30,16,"종류별 탭에서 전술을 고르고 풀다운으로 설정합니다. 확인하면 내 팀에 적용하고 이 세이브에 저장합니다.");
    s+=&label("status",30,95,1680,28,15,"");
    for(tab,category)in CATEGORIES.iter().enumerate(){
        s+=&button(&format!("tab{tab}"),30+tab*(1680/CATEGORIES.len()),132,1680/CATEGORIES.len()-10,category.name);
        s+=&format!("#tab_mark{tab}:color {{ ignore_event:true; x:{}px; y:173px; width:{}px; height:3px; color:#69bfcfff; }}\n",30+tab*(1680/CATEGORIES.len()),1680/CATEGORIES.len()-10);
    }
    s+=&label("name_header",30,184,640,35,17,"전술 / 적용 상태");
    for(c,n)in tactics::POS_NAMES.iter().enumerate(){s+=&label(&format!("col{c}"),690+c*200,184,190,35,18,n).replace("align_y:Center;","align_x:Center; align_y:Center;");}
    for(tab,category)in CATEGORIES.iter().enumerate(){
      s+=&format!("#page{tab}:scroll_view {{ x:0px; y:229px; width:1720px; height:616px; speed:100; visible:false; bar_width:5; bar:{{source:\"asset/base/sprite/white\";color:#69bfcfff;hover:{{color:#ecfbf8ff;}}}} back:{{source:\"asset/base/sprite/white\";color:#00000000;}} #contents:empty {{width:1700px;height:{}px;\n",category.ids.len()*76);
      for row in 0..category.ids.len() {
        let y=row*76;
        s+=&label(&format!("name{row}"),30,y,640,30,18,"");
        s+=&label(&format!("desc{row}"),30,y+30,640,44,14,"");
        for col in 0..5{s+=&centered_dropdown(dd::button_source(&format!("c{row}_{col}"),(690+col*200)as i32,(y+12)as i32,190,38,"",16),190,44);}
        s+=&centered_dropdown(dd::button_source(&format!("team{row}"),690,(y+12)as i32,990,38,"",16),990,44);
        s+=&centered_dropdown(dd::button_source(&format!("duo{row}"),1290,(y+12)as i32,390,38,"",16),390,44);
    }
      s+="}}\n";
    }
    s+=&label("category_info",30,862,1370,38,16,"");
    s+=&button("reset",1450,862,230,"전부 기본으로");
    s+=&label("config",30,916,1120,30,14,"단계별 숫자는 모드 폴더의 detail_tactics_values.cfg에서 바꾸면 자동 반영됩니다.");
    s+=&button("cancel",1220,912,210,"취소");s+=&button("ok",1450,912,230,"확인");
    for(id,w)in [("choices_player",190),("choices_team",990),("choices_duo",390)]{
      s+=&centered_dropdown(dd::list_source(id,690,234,w,38,16,&[("lv0",""),("lv1",""),("lv2","")]),w as usize,54);
    }
    s+="}\n";s
}
fn register(ctx:&mut StableClient<'_>,id:&str,f:impl Fn()+Send+Sync+'static){
    let key=format!("{POP}.{id}");
    // 재생성 때 호스트 등록이 남아도 같은 프레임에서 한 번만 처리한다.
    let click_key=key.clone();
    ctx.ui_register_click(&key,"",move |_|{
        let frame=crate::FRAME.load(std::sync::atomic::Ordering::Relaxed);
        {let mut g=FIRED.lock().unwrap_or_else(|e|e.into_inner());let m=g.get_or_insert_with(Default::default);
         if m.get(&click_key)==Some(&frame){return;}m.insert(click_key.clone(),frame);}
        f();
    });
}
fn register_popup(ctx:&mut StableClient<'_>){
    register(ctx,"cancel",||{let mut e=EDIT.lock().unwrap_or_else(|e|e.into_inner());close_dropdown(&mut e);e.open=false;});
    register(ctx,"ok",||{
        let mut e=EDIT.lock().unwrap_or_else(|e|e.into_inner());
        if !e.open||!save::loaded(){return;}
        let _update=tactics::config_update();
        for t in 0..tactics::N{for col in 0..6{tactics::set_level(t,col,e.draft[t][col]);}}
        let body=tactics::state_text();crate::log(&format!("전술 확인: {body:?}"));save::request(body);close_dropdown(&mut e);e.open=false;
    });
    for tab in 0..CATEGORIES.len(){register(ctx,&format!("tab{tab}"),move||{
        let mut e=EDIT.lock().unwrap_or_else(|e|e.into_inner());if e.open{e.switch_tab(tab);}
    });}
    register(ctx,"reset",||{let mut e=EDIT.lock().unwrap_or_else(|e|e.into_inner());close_dropdown(&mut e);e.draft=[[0;6];tactics::N];e.dirty=true;});
    for(tab,category)in CATEGORIES.iter().enumerate(){for row in 0..category.ids.len(){for col in 0..=COL_DUO{
        let id=format!("page{tab}.contents.{}",cell_id(row,col));
        register(ctx,&id,move||{
            let mut e=EDIT.lock().unwrap_or_else(|e|e.into_inner());let Some(t)=e.tactic(row)else{return;};
            if !e.open||e.tab!=tab||tactics::status(t)!=tactics::ST_ACTIVE{return;}
            if !cell_visible(t,col){return;}
            if e.dropdown==Some((row,col))&&DROPDOWN_OPEN.load(Relaxed){close_dropdown(&mut e);}else{e.dropdown=Some((row,col));DROPDOWN_OPEN.store(true,Relaxed);}
            e.dirty=true;
        });
    }}}
    for list in ["choices_player","choices_team","choices_duo"]{for(lv,id)in ITEM_IDS.iter().enumerate(){register(ctx,&format!("{list}.{id}"),move||{
        let mut e=EDIT.lock().unwrap_or_else(|e|e.into_inner());
        let Some((row,col))=e.dropdown else{return;};let Some(t)=e.tactic(row)else{return;};
        if !e.open||choice_id(col)!=list||!DROPDOWN_OPEN.load(Relaxed)||tactics::status(t)!=tactics::ST_ACTIVE{return;}
        e.select(row,col,lv as u8);
    });}}
}
fn choice(t:usize,lv:u8)->String {
    if tactics::TACTICS[t].kind==Kind::Toggle {return if lv==0{"기본 · 켬"}else{"끔"}.into();}
    format!("{} · {}",["기본","낮게","높게"][lv.min(2) as usize],tactics::value_of(t,lv))
}
pub fn tick(ctx:&mut StableClient<'_>,frame:u64){
    if !ctx.ui_exists(PARENT){leave();if ctx.ui_exists(POP){ctx.ui_set_visible(POP,false);}return;}
    let entry=format!("{PARENT}.dt_open");
    if !ctx.ui_exists(&entry){
        let src=button("dt_open",710,15,216,"상세전술").replacen('#',"",1);
        if ctx.ui_spawn_source(PARENT,&src){
            ctx.ui_register_click(&entry,"",move |_|{
                if !save::loaded(){return;}
                let mut e=EDIT.lock().unwrap_or_else(|e|e.into_inner());
                if e.open{return;}
                close_dropdown(&mut e);dd::reset_cache();
                for t in 0..tactics::N{for c in 0..6{e.draft[t][c]=tactics::level(t,c);}}
                e.open=true;e.switch_tab(0);
            });
        }
    }
    let is_open=EDIT.lock().unwrap_or_else(|e|e.into_inner()).open;
    if is_open&&!ctx.ui_exists(POP){if !ctx.ui_spawn_source("main",&source()){crate::log("상세전술 모달 생성 실패");return;}register_popup(ctx);}
    if ctx.ui_exists(POP)&&ctx.ui_visible(POP)!=Some(is_open){ctx.ui_set_visible(POP,is_open);}
    if !is_open{for id in ["choices_player","choices_team","choices_duo"]{if ctx.ui_exists(POP){ctx.ui_set_visible(&format!("{POP}.{id}"),false);}}return;}
    let mut e=EDIT.lock().unwrap_or_else(|e|e.into_inner());
    let active=e.dropdown.map(|(_,col)|choice_id(col));
    for id in ["choices_player","choices_team","choices_duo"]{if active!=Some(id){ctx.ui_set_visible(&format!("{POP}.{id}"),false);}}
    if let Some((row,col))=e.dropdown{
        let Some(t)=e.tactic(row)else{close_dropdown(&mut e);return;};let count=if tactics::TACTICS[t].kind==Kind::Toggle{2}else{3};
        let btn=row_path(e.tab,row,col);
        let list=format!("{POP}.{}",choice_id(col));
        let Some((bx,by,_,bh))=ctx.ui_node_rect(&btn)else{close_dropdown(&mut e);return;};
        let Some((mx,my,_,_))=ctx.ui_node_rect(POP)else{return;};
        let local_y=by-my;
        if local_y<229.0||local_y+bh>845.0{close_dropdown(&mut e);e.dirty=true;return;}
        let h=count*38;let y=if local_y+bh+2.0+h as f32>845.0{local_y-h as f32-2.0}else{local_y+bh+2.0};
        ctx.ui_set_properties(&list,&format!("x:{}px; y:{y}px; height:{h}px;",bx-mx));
        for(lv,id)in ITEM_IDS.iter().enumerate(){let p=format!("{list}.{id}");ctx.ui_set_visible(&p,lv<count);
            text(ctx,&format!("{p}.label"),&choice(t,lv as u8));}
        let lv=cell_level(&e,t,col);
        let edge=dd::click_edge();dd::tick(ctx,&btn,&list,&ITEM_IDS[..count],&DROPDOWN_OPEN,lv.map(usize::from).unwrap_or(usize::MAX),&cell_choice(t,lv),edge);
        if !DROPDOWN_OPEN.load(Relaxed){e.dropdown=None;}
    }else{let _=dd::click_edge();}
    if !e.dirty&&frame%120!=0{return;}e.dirty=false;
    text(ctx,&format!("{POP}.status"),if tactics::armed(){"내 팀 적용 준비됨"}else{"내 팀 선수 / 세이브 설정을 확인 중입니다. 확인이 끝나면 자동 적용합니다."});
    let category=&CATEGORIES[e.tab];
    text(ctx,&format!("{POP}.category_info"),&format!("{} · {}개 전술  |  적용되지 않는 포지션은 빈칸으로 표시합니다.",category.name,category.ids.len()));
    for tab in 0..CATEGORIES.len(){
        let selected=tab==e.tab;let bg=if selected{"#274b5fff"}else{"#30343fff"};
        ctx.ui_set_properties(&format!("{POP}.tab{tab}"),&format!("btn: {{color:{bg}; back_color:{bg};}}"));
        ctx.ui_set_visible(&format!("{POP}.tab_mark{tab}"),selected);
        ctx.ui_set_visible(&format!("{POP}.page{tab}"),selected);
    }
    for row in 0..category.ids.len() {
        let t=e.tactic(row).expect("category catalog");
        let base=format!("{POP}.page{}.contents",e.tab);
        let d=&tactics::TACTICS[t];
        let st=tactics::status(t);let suffix=match st{tactics::ST_ACTIVE=>"",tactics::ST_UNSUPPORTED=>" · 이번 버전 미지원",tactics::ST_FAILED=>" · 설치 실패(원본 유지)",_=>" · 준비 중"};
        text(ctx,&format!("{base}.name{row}"),&format!("{}{suffix}",d.name));
        text(ctx,&format!("{base}.desc{row}"),d.desc);
        for col in 0..=COL_DUO{
            let path=row_path(e.tab,row,col);
            let show=cell_visible(t,col);
            if ctx.ui_visible(&path)!=Some(show){ctx.ui_set_visible(&path,show);}
            if !show{continue;}
            let lv=cell_level(&e,t,col);let title=if st!=tactics::ST_ACTIVE{"원본 유지".into()}else{cell_choice(t,lv)};
            text(ctx,&format!("{path}.label"),&title);
            let bg=if st!=tactics::ST_ACTIVE{"#292b33ff"}else{match lv{Some(1)=>"#274555ff",Some(2)=>"#60452bff",_=>"#30343fff"}};
            ctx.ui_set_properties(&path,&format!("btn: {{color:{bg}; back_color:{bg};}}"));
        }
    }
}
pub fn leave(){let mut e=EDIT.lock().unwrap_or_else(|e|e.into_inner());close_dropdown(&mut e);dd::reset_cache();e.open=false;e.dirty=true;}
#[cfg(test)] mod tests {
    use super::*;
    #[test]fn scroll_tabs_expose_last_row_and_preserve_draft(){
        let mut e=Editor::default();let mut total=0;
        for(tab,category)in CATEGORIES.iter().enumerate(){
            total+=category.ids.len();e.switch_tab(tab);
            let row=category.ids.len()-1;let t=e.tactic(row).unwrap();
            let col=(0..=COL_DUO).find(|c|cell_visible(t,*c)).unwrap();
            assert!(e.select(row,col,1));
            assert_eq!(cell_level(&e,t,col),Some(1));
        }
        assert_eq!(total,88);assert_eq!(CATEGORIES.len(),8);
        let src=source();assert_eq!(src.matches(":scroll_view").count(),8);
        for(tab,category)in CATEGORIES.iter().enumerate(){assert!(src.contains(&format!("height:{}px",category.ids.len()*76)));assert!(src.contains(&format!("#page{tab}:scroll_view")));}
    }
    #[test]fn audited_positions_and_shared_cells(){
        let cases=[
            ("jg_gank_hp_start",0b0000010),("jg_gank_hp_floor",0b0000010),
            ("jungle_safe_radius",0b0000010),("jungle_stop_hp",0b0000010),
            ("liner_roam_self_hp",0b0011101),("liner_roam_lane_healthy_hp",0b0011101),
            ("liner_roam_call_chance",0b0011101),("support_minion_block",0b0010000),
            ("lane_assign_hp",0b0001101),("bot_duo_partner",0b1000000),
            ("gank_abort_hp",0b0011111),("gank_abort_tower",0b0011111),
            ("gank_call_accept_hp",0b0011111),("lane_opportunity_own_min",0b0011111),
            ("lane_retreat_final",0b0011111),("push_sustain_margin",0b0100000),
        ];
        for(id,mask)in cases{let t=tactics::index_of(id).unwrap();for col in 0..=COL_DUO{
            assert_eq!(cell_visible(t,col),mask&(1<<col)!=0,"{id} col={col}");
        }}
    }
    #[test]fn tab_edits_keep_saved_identity_and_hidden_values(){
        let mut e=Editor::default();
        let combat=tactics::index_of("combat_retreat_hp").unwrap();
        let jungle=tactics::index_of("jg_gank_hp_start").unwrap();
        let duo=tactics::index_of("bot_duo_partner").unwrap();
        e.draft[jungle][0]=2; // 예전 세이브의 숨길 칸은 보존한다.
        assert!(e.select(0,0,1));
        e.dropdown=Some((0,0));DROPDOWN_OPEN.store(true,Relaxed);
        e.switch_tab(2);
        assert!(e.dropdown.is_none()&&!DROPDOWN_OPEN.load(Relaxed));
        assert!(!e.select(0,0,1));assert!(e.select(0,1,2));
        e.switch_tab(1);
        assert!(!e.select(CATEGORIES[1].ids.len(),0,1)); // 범위 밖 행 클릭 방지.
        assert!(e.select(5,COL_DUO,1));assert!(!e.select(5,COL_DUO,2));
        e.switch_tab(0);
        assert_eq!(e.draft[combat],[1,0,0,0,0,0]);
        assert_eq!(e.draft[jungle],[2,2,0,0,0,0]);
        assert_eq!(e.draft[duo],[0,0,0,1,1,0]);
    }
    #[test]fn generated_source_has_all_cells(){let s=source();for r in 0..CATEGORIES[0].ids.len(){assert!(s.contains(&format!("#team{r}:")));for c in 0..5{assert!(s.contains(&format!("#c{r}_{c}:")));}}assert_eq!(s.matches('{').count(),s.matches('}').count());
        if let Some(dir)=std::env::var_os("DT_EXPORT_DIR"){std::fs::create_dir_all(&dir).unwrap();std::fs::write(std::path::Path::new(&dir).join("detail_tactics_popup.ui"),s).unwrap();}
    }
}
