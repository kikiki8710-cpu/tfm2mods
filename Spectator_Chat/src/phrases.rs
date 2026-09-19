//! phrases.rs — 원작 Spectator_Chat 문구·닉·메시지 생성부 그대로(순수 Rust). 0.6.0 stable 판에서 재사용.
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use super::mod_dir;
static RNG: AtomicUsize = AtomicUsize::new(0x12345);
pub fn rng() -> usize {
    let mut x = RNG.load(Ordering::Relaxed);
    x ^= x << 13; x ^= x >> 17; x ^= x << 5;
    RNG.store(x, Ordering::Relaxed); x
}

// ───────── 외부 문구 파일 시스템 ─────────
// chat_lines.txt (dll 옆)에서 섹션별 문구 로드. 없으면 내장 기본값.
pub static PHRASES: Mutex<Option<HashMap<String, Vec<String>>>> = Mutex::new(None);
// 섹션별 최근 뽑은 문구 인덱스들 (연속 중복 방지, 최근 2개)
static LAST_PICK: Mutex<Option<HashMap<String, Vec<usize>>>> = Mutex::new(None);

pub fn load_phrases() {
    let mut text = String::new();
    if let Some(d) = mod_dir() {
        if let Ok(s) = std::fs::read_to_string(format!(r"{}\chat_lines.txt", d)) { text = s; }
    }
    if text.trim().is_empty() { text = default_phrases(); }   // 폴백
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut cur = String::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') { continue; }
        if t.starts_with('[') && t.ends_with(']') {
            cur = t[1..t.len()-1].trim().to_string();
            map.entry(cur.clone()).or_default();
        } else if !cur.is_empty() {
            map.entry(cur.clone()).or_default().push(t.to_string());
        }
    }
    *PHRASES.lock().unwrap_or_else(|e| e.into_inner()) = Some(map);
}

// 섹션에서 랜덤 문구 뽑기 (없으면 빈 문자열)
pub fn line_of(section: &str) -> String {
    let g = PHRASES.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(m) = g.as_ref() {
        if let Some(v) = m.get(section) {
            let n = v.len();
            if n == 0 { return String::new(); }
            if n == 1 { return v[0].clone(); }
            // 최근 뽑은 인덱스들을 피해서 뽑기 (연속 중복 방지)
            let mut lp = LAST_PICK.lock().unwrap_or_else(|e| e.into_inner());
            let map = lp.get_or_insert_with(HashMap::new);
            let recent = map.entry(section.to_string()).or_insert_with(Vec::new);
            // 피할 개수: 문구가 적으면 줄임 (최대 2개, 단 n-1 이하)
            let avoid = recent.len().min(2).min(n - 1);
            let mut idx = rng() % n;
            // 최근 avoid개에 들면 다시 뽑기 (최대 8회 시도)
            let mut tries = 0;
            while tries < 8 && recent.iter().rev().take(avoid).any(|&r| r == idx) {
                idx = rng() % n;
                tries += 1;
            }
            recent.push(idx);
            if recent.len() > 2 { recent.remove(0); }  // 최근 2개만 유지
            return v[idx].clone();
        }
    }
    String::new()
}

// 파일 없을 때 폴백(이자 chat_lines.txt 기본 템플릿)
pub fn default_phrases() -> String {
r#"# Spectator_Chat 문구 파일. # 는 주석. [섹션] 아래 한 줄에 하나씩.
# 치환자: {k}킬러 {d}죽은선수 {p}선수명 {b}블루점수 {r}레드점수 {n}관여인원 {t}팀(블루/레드)
[kill]
{k} 굿킬
{d} 잘렸다
{k} 따냄
ㅋㅋ {d} 사망
나이스 갱
{k} +1
깔끔한 킬
{d} 짐 쌌네
[kill_solo]
ㅋㅋㅋㅋ {d} 솔킬당함
{d} 뭐함ㅋㅋ
{k} 1대1 압살
{d} 라인에서 녹음ㅋㅋ
{k} 손 좋네
{d} 이걸 짤리네ㄷㄷ
ㄷㄷ {k} 솔로킬
{d} 또 죽네
[kill_multi]
{k} 미친 캐치ㄷㄷ
{d} 물려서 끔살
{n}명이 달려드네ㅋㅋㅋ
한타 터졌다
{d} 포커싱 광속삭제
이걸 다 붙네ㅋㅋ
{k} 막타 강탈
집중공격 ㄷㄷ
[surv]
{p} 살았다ㄷㄷ
{p} 도망 지렸다
{p} 한 칸 남았는데 생존ㄷㄷ
{p} 왜 안죽음ㅋㅋ
{p} 기적의 생존
{p} 피 1 남기고 빠짐
[serpen]
{t} 세르펜 꿀꺽
세르펜 가져간다
{t} 세르펜 스틸ㄷㄷ
세르펜 먹고 스노우볼
{t} 오브젝트 챙김
세르펜 났다
[score]
{b}:{r} 박빙
치열하네
스코어 따라붙음
{b}:{r} ㄷㄷ
엎치락뒤치락
스코어 벌어진다
이거 터졌네
{b}:{r} 차이 크다
[idle_early]
오 시작이네
긴장된다
누가 이길까
초반 중요하지
오늘 누가 잘함?
라인전 보자
ㄷㄷ 기대된다
어디가 셀까
잘 좀 해봐라
집중집중
[idle_mid_close]
치열하네
해볼만한데
오 비등비등
아직 모름
둘 다 잘함
팽팽하다
[idle_mid_lead]
분위기 탔네
이거 벌어지나
한쪽이 잡았다
흐름 좋다
우세 굳히나
오 차이난다
[idle_late_close]
끝까지 모름ㄷㄷ
심장 떨린다
이거 진짜 박빙
한 끗 차이
누가 이겨도 인정
미쳤다 이거
[idle_late_lead]
거의 굳었나
역전 가능?
아직 희망 있나
이거 넘어가나
마무리각
따라잡을수 있을까
[idle_late_crush]
이거 GG각
클린업이네
사실상 끝
압도적이다
경기 터졌다
수고하셨습니다ㅋㅋ
[idle_player]
{p} 잘하네
{p} 기대된다
{p} 폼 좋아보임
{p} 집중하자
{p} 살아있네
[cheer]
블루팀 화이팅~
레드팀 가자!
오늘 경기 기대된다
꿀잼각ㅋㅋ
드디어 시작이네
가즈아~~
오늘 누가 이기냐
명경기 가자
두근두근
화이팅!
재밌겠다
다들 잘하자
[cheer_player]
{p} 화이팅
{p} 캐리 가자
{p} 믿는다
{p} 오늘 폼 보여줘
{p} 가즈아
하나 둘 셋 {p} 화이팅!
{p} 슈퍼플레이 가자
"#.to_string()
}

const NICK_A: &[&str] = &[
    "롤","새벽","야식","퇴근","본방","익명","침착","광동","한타","정글차이",
    "페이커","망겜","갱맘","다이브","노데스","캐리","트수","백수","직장인","학생",
    "모쏠","치킨","라면","콜라","닥터","골드","다이아","챌린저","브론즈","운지",
    "고독한","지나가던","화난","신난","졸린","배고픈","현질","무과금","랜덤","익명의",
];
const NICK_B: &[&str] = &[
    "장인","충","러","갓생","빠","안티","관전러","골수팬","분석가","평론가",
    "코치","해설","빌런","고수","뉴비","린저씨","아재","큰손","워리어","마스터",
    "헌터","폐인","거북이","워치맨","구독자","스트리머","택배","감자","고양이","너구리",
    "12년차","3년차","복귀","휴면","주작러","팩트","오타쿠","프로","아마추어","구경꾼",
];
pub fn nick() -> String {
    let a = NICK_A[rng() % NICK_A.len()];
    let b = NICK_B[rng() % NICK_B.len()];
    format!("{}{}", a, b)
}

pub fn field_i64(s:&str,key:&str)->Option<i64>{
    let p=s.find(key)?+key.len(); let r=&s[p..];
    let r=r.trim_start_matches(|c:char|c==' '||c==':');
    let e=r.find(|c:char|!(c.is_ascii_digit()||c=='-')).unwrap_or(r.len());
    r[..e].parse().ok()
}
pub fn field_quoted(s:&str,key:&str)->Option<String>{
    let p=s.find(key)?+key.len(); let r=&s[p..];
    let q1=r.find('"')?+1; let q2=r[q1..].find('"')?+q1; Some(r[q1..q2].to_string())
}
pub fn count_assist(s:&str)->usize{
    if let Some(p)=s.find("assist:"){ let r=&s[p..]; let e=r.find(']').unwrap_or(r.len());
        return r[..e].matches('"').count()/2; } 0
}

// 8% 확률로 뜬금없는 random 채팅 (모든 상황 공통). 해당되면 Some.
pub fn maybe_random()->Option<String>{
    if rng()%100 < 8 {
        let l=line_of("random");
        if !l.is_empty(){ return Some(format!("{}: {}", nick(), l)); }
    }
    None
}
pub fn sub(s:&str, k:&str,d:&str,p:&str,b:i64,r:i64,n:usize)->String{
    // {w}=앞선 팀, {l}=밀리는 팀 (점수 비교). 동점이면 둘다 "" (문구가 어색하면 안 쓰면 됨)
    let (win, lose) = if b > r { ("블루","레드") } else if r > b { ("레드","블루") } else { ("","") };
    s.replace("{k}",k).replace("{d}",d).replace("{p}",p)
     .replace("{b}",&b.to_string()).replace("{r}",&r.to_string())
     .replace("{w}",win).replace("{l}",lose)
     .replace("{n}",&n.to_string())
}
pub fn msg_kill(killer:&str, killed:&str, an:usize)->String{
    if let Some(x)=maybe_random(){return x;}
    let sec = if an==0 {"kill_solo"} else if an>=3 {"kill_multi"} else {"kill"};
    let mut l = line_of(sec);
    if l.is_empty() { l = line_of("kill"); }
    if l.is_empty() { l = format!("{} 처치", killer); }
    format!("{}: {}", nick(), sub(&l, killer, killed, "", 0, 0, an+1))
}
pub fn msg_serpen(team:i64)->String{
    if let Some(x)=maybe_random(){return x;}
    let t=if team==0 {"블루"} else {"레드"};
    let mut l=line_of("serpen"); if l.is_empty(){ l="세르펜 획득".into(); }
    format!("{}: {}", nick(), l.replace("{t}",t))
}
pub fn team_str(team:i64)->&'static str{ if team==0 {"블루"} else {"레드"} }
// 타워 파괴 (team = 파괴한 쪽)
pub fn msg_tower(team:i64)->String{
    if let Some(x)=maybe_random(){return x;}
    let mut l=line_of("tower"); if l.is_empty(){ l="{t} 타워 파괴".into(); }
    format!("{}: {}", nick(), l.replace("{t}",team_str(team)))
}
// N인 CC. hit_team=true → CC 건 쪽 시점(긍정), false → 당한 쪽 시점(부정).
pub fn msg_cc(n:usize, cc:&str, taken:bool, who:&str)->String{
    if let Some(x)=maybe_random(){return x;}
    let sec = if taken {"cc_taken"} else {"cc_hit"};
    let mut l=line_of(sec); if l.is_empty(){ l="{n}인 {cc}ㄷㄷ".into(); }
    let l=l.replace("{n}",&n.to_string()).replace("{cc}",cc).replace("{p}",who);
    format!("{}: {}", nick(), l)
}
// 궁 사용 (who = 시전 선수명)
pub fn msg_ult(who:&str)->String{
    if let Some(x)=maybe_random(){return x;}
    let mut l=line_of("ult"); if l.is_empty(){ l="{p} 궁".into(); }
    format!("{}: {}", nick(), l.replace("{p}",who))
}
// 골드/딜 우위 (team = 앞선 쪽, deal=true 면 딜량)
pub fn msg_lead(team:i64, deal:bool)->String{
    if let Some(x)=maybe_random(){return x;}
    let sec = if deal {"lead_deal"} else {"lead_gold"};
    let mut l=line_of(sec); if l.is_empty(){ l="{t} 우세".into(); }
    format!("{}: {}", nick(), l.replace("{t}",team_str(team)))
}
pub fn msg_score(b:i64,r:i64)->String{
    if let Some(x)=maybe_random(){return x;}
    let mut l=line_of("score"); if l.is_empty(){ l=format!("{}:{}",b,r); }
    format!("{}: {}", nick(), sub(&l,"","","",b,r,0))
}
pub fn ambient_section(b:i64,r:i64)->&'static str{
    let sum=b+r; let diff=(b-r).abs();
    if sum<=3 { "idle_early" }
    else if sum<=8 { if diff<=1 {"idle_mid_close"} else {"idle_mid_lead"} }
    else { if diff<=1 {"idle_late_close"} else if diff<=4 {"idle_late_lead"} else {"idle_late_crush"} }
}
pub fn msg_ambient(names:&[String], b:i64, r:i64)->String{
    if let Some(x)=maybe_random(){return x;}
    // 25% 선수 언급(idle_player 섹션)
    if !names.is_empty() && rng()%4==0 {
        let who=&names[rng()%names.len()];
        let l=line_of("idle_player");
        if !l.is_empty() { return format!("{}: {}", nick(), sub(&l,"","",who,b,r,0)); }
    }
    let mut l=line_of(ambient_section(b,r));
    if l.is_empty(){ l=line_of("idle_early"); }
    if l.is_empty(){ l="음...".into(); }
    // ambient 섹션도 {p} 쓰면 랜덤 선수명으로 치환 (idle_mid_close 등)
    let who = if names.is_empty() { String::new() } else { names[rng()%names.len()].clone() };
    format!("{}: {}", nick(), sub(&l,"","",&who,b,r,0))
}
pub fn msg_survive(who:&str)->String{
    if let Some(x)=maybe_random(){return x;}
    let mut l=line_of("surv"); if l.is_empty(){ l="{p} 생존".into(); }
    format!("{}: {}", nick(), l.replace("{p}",who))
}
pub fn msg_cheer(names:&[String])->String{
    if !names.is_empty() && rng()%10<4 {
        let who=&names[rng()%names.len()];
        let l=line_of("cheer_player");
        if !l.is_empty() { return format!("{}: {}", nick(), l.replace("{p}",who)); }
    }
    let mut l=line_of("cheer"); if l.is_empty(){ l="화이팅!".into(); }
    format!("{}: {}", nick(), l)
}
