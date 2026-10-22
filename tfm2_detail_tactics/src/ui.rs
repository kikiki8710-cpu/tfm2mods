//! 게임 팀전술 창에 버튼 + 8행씩 보는 포지션별 상세전술 모달.
use crate::{tactics::{self,Kind,Scope},save};
use mod_api_stable::StableClient;
use std::sync::Mutex;
const ROWS:usize=8;
const POP:&str="main.dt_modal";
const PARENT:&str="main.contents.strategy";
#[derive(Default)] struct Editor { open:bool, page:usize, draft:[[u8;6];tactics::N], dirty:bool }
static EDIT:Mutex<Editor>=Mutex::new(Editor{open:false,page:0,draft:[[0;6];tactics::N],dirty:true});
static FIRED:Mutex<Option<std::collections::HashMap<String,u64>>>=Mutex::new(None);
fn text(ctx:&mut StableClient<'_>,p:&str,s:&str){if ctx.ui_text(p).as_deref()!=Some(s){ctx.ui_set_text(p,s);}}
fn button(id:&str,x:usize,y:usize,w:usize,label:&str)->String{
    format!("#{id}:color_icon_button {{ @\"asset/base/style/main#tertiary_button\"; x:{x}px; y:{y}px; width:{w}px; height:38px; #caption:label {{ ignore_event:true; width:100%; height:100%; text:\"{label}\"; font:\"asset/base/font/set/bold\"; size:16; align_x:Center; align_y:Center; }} }}\n")
}
fn label(id:&str,x:usize,y:usize,w:usize,h:usize,size:usize,s:&str)->String{
    format!("#{id}:label {{ @\"asset/base/style/main#label\"; x:{x}px; y:{y}px; width:{w}px; height:{h}px; size:{size}; align_y:Center; text:\"{s}\"; }}\n")
}
fn source()->String{
    let mut s=String::from("dt_modal:color { width:1740px; height:970px; anchor_x:0.5; pivot_x:0.5; anchor_y:0.5; pivot_y:0.5; visible:false; color:#161721ff; rounding:Uniform{rounding:12;}\n");
    s+=&label("title",30,18,1450,42,26,"상세전술");
    s+=&label("hint",30,63,1680,30,16,"칸을 눌러 기본 → 낮게 → 높게 순서로 선택합니다. 확인하면 내 팀에 적용하고 이 세이브에 저장합니다.");
    s+=&label("status",30,95,1680,28,15,"");
    s+=&label("name_header",30,137,640,35,17,"전술 / 적용 상태");
    for(c,n)in tactics::POS_NAMES.iter().enumerate(){s+=&label(&format!("col{c}"),690+c*200,137,190,35,18,n);}
    for row in 0..ROWS {
        let y=182+row*82;
        s+=&label(&format!("name{row}"),30,y,640,30,18,"");
        s+=&label(&format!("desc{row}"),30,y+30,640,44,14,"");
        for col in 0..5{s+=&button(&format!("c{row}_{col}"),690+col*200,y+12,190,"");}
        s+=&button(&format!("team{row}"),690,y+12,990,"");
    }
    s+=&button("previous",30,862,120,"이전");s+=&button("next",310,862,120,"다음");
    s+=&label("page",165,862,135,38,18,"");
    s+=&button("reset",465,862,210,"전부 기본으로");
    s+=&label("config",30,916,1120,30,14,"단계별 숫자는 모드 폴더의 detail_tactics_values.cfg에서 바꾸면 자동 반영됩니다.");
    s+=&button("cancel",1220,912,210,"취소");s+=&button("ok",1450,912,230,"확인");
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
    register(ctx,"cancel",||{EDIT.lock().unwrap_or_else(|e|e.into_inner()).open=false;});
    register(ctx,"ok",||{
        let mut e=EDIT.lock().unwrap_or_else(|e|e.into_inner());
        if !e.open||!save::loaded(){return;}
        for t in 0..tactics::N{for col in 0..6{tactics::set_level(t,col,e.draft[t][col]);}}
        let body=tactics::state_text();crate::log(&format!("전술 확인: {body:?}"));save::request(body);e.open=false;
    });
    register(ctx,"previous",||{let mut e=EDIT.lock().unwrap_or_else(|e|e.into_inner());e.page=e.page.saturating_sub(1);e.dirty=true;});
    register(ctx,"next",||{let mut e=EDIT.lock().unwrap_or_else(|e|e.into_inner());e.page=(e.page+1).min((tactics::N-1)/ROWS);e.dirty=true;});
    register(ctx,"reset",||{let mut e=EDIT.lock().unwrap_or_else(|e|e.into_inner());e.draft=[[0;6];tactics::N];e.dirty=true;});
    for row in 0..ROWS{for col in 0..6{
        let id=if col==5{format!("team{row}")}else{format!("c{row}_{col}")};
        register(ctx,&id,move||{
            let mut e=EDIT.lock().unwrap_or_else(|e|e.into_inner());let t=e.page*ROWS+row;
            if !e.open||t>=tactics::N||tactics::status(t)!=tactics::ST_ACTIVE{return;}
            let d=&tactics::TACTICS[t];
            if (d.scope==Scope::Team)!=(col==5){return;}
            let n=if d.kind==Kind::Toggle{2}else{3};e.draft[t][col]=(e.draft[t][col]+1)%n;e.dirty=true;
        });
    }}
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
                for t in 0..tactics::N{for c in 0..6{e.draft[t][c]=tactics::level(t,c);}}
                e.open=true;e.page=0;e.dirty=true;
            });
        }
    }
    let is_open=EDIT.lock().unwrap_or_else(|e|e.into_inner()).open;
    if is_open&&!ctx.ui_exists(POP){if !ctx.ui_spawn_source("main",&source()){crate::log("상세전술 모달 생성 실패");return;}register_popup(ctx);}
    if ctx.ui_exists(POP)&&ctx.ui_visible(POP)!=Some(is_open){ctx.ui_set_visible(POP,is_open);}
    if !is_open{return;}
    let mut e=EDIT.lock().unwrap_or_else(|e|e.into_inner());
    if !e.dirty&&frame%120!=0{return;}e.dirty=false;
    text(ctx,&format!("{POP}.status"),if tactics::armed(){"내 팀 적용 준비됨 · 미지원 전술은 기본 동작을 유지합니다."}else{"내 팀 선수 / 세이브 설정을 확인 중입니다. 확인이 끝나면 자동 적용합니다."});
    text(ctx,&format!("{POP}.page"),&format!("{} / {}",e.page+1,tactics::N.div_ceil(ROWS)));
    for row in 0..ROWS {
        let t=e.page*ROWS+row;let Some(d)=tactics::TACTICS.get(t)else{continue;};
        let st=tactics::status(t);let suffix=match st{tactics::ST_ACTIVE=>"",tactics::ST_UNSUPPORTED=>" · 이번 버전 미지원",tactics::ST_FAILED=>" · 설치 실패(원본 유지)",_=>" · 준비 중"};
        text(ctx,&format!("{POP}.name{row}"),&format!("{}{suffix}",d.name));
        text(ctx,&format!("{POP}.desc{row}"),d.desc);
        for col in 0..6{
            let id=if col==5{format!("team{row}")}else{format!("c{row}_{col}")};let path=format!("{POP}.{id}");
            let show=(d.scope==Scope::Team)==(col==5);
            if ctx.ui_visible(&path)!=Some(show){ctx.ui_set_visible(&path,show);}
            if !show{continue;}
            let lv=e.draft[t][col];let title=if st!=tactics::ST_ACTIVE{"원본 유지".into()}else{choice(t,lv)};
            text(ctx,&format!("{path}.caption"),&title);
            let bg=if st!=tactics::ST_ACTIVE{"#292b33ff"}else{match lv{1=>"#274555ff",2=>"#60452bff",_=>"#30343fff"}};
            ctx.ui_set_properties(&path,&format!("btn: {{color:{bg}; back_color:{bg};}}"));
        }
    }
}
pub fn leave(){let mut e=EDIT.lock().unwrap_or_else(|e|e.into_inner());e.open=false;e.dirty=true;}
#[cfg(test)] mod tests {
    use super::*;
    #[test]fn generated_source_has_all_cells(){let s=source();for r in 0..ROWS{assert!(s.contains(&format!("#team{r}:")));for c in 0..5{assert!(s.contains(&format!("#c{r}_{c}:")));}}assert_eq!(s.matches('{').count(),s.matches('}').count());
        if let Some(dir)=std::env::var_os("DT_EXPORT_DIR"){std::fs::create_dir_all(&dir).unwrap();std::fs::write(std::path::Path::new(&dir).join("detail_tactics_popup.ui"),s).unwrap();}
    }
}
