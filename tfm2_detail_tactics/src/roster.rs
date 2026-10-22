//! 공식 read-only JSON API의 계약 소속으로 내 팀 선수를 찾는다(raw 오프셋 없음).
use mod_api_stable::{RecordKindV1 as R,StableClient};
use std::collections::VecDeque;
use serde_json::Value;
#[derive(Default)]
pub struct Roster { team:Option<usize>, queue:VecDeque<usize>, ids:Vec<u64>, refresh:u64 }
pub fn contract_team(text:&str)->Option<u64>{
    let v:Value=serde_json::from_str(text).ok()?;
    v.get("InContract")?.get("team_id")?.as_u64()
}
impl Roster {
    pub fn tick(&mut self,ctx:&StableClient<'_>,frame:u64)->bool {
        let team=ctx.player_team_id();
        if team!=self.team { *self=Self::default();self.team=team;crate::tactics::set_armed(false);crate::tactics::set_my_athletes(&[]); }
        let Some(team)=team else{return false;};
        if frame>=self.refresh && self.queue.is_empty(){
            self.queue=ctx.athlete_ids().into();self.refresh=frame+300;
            let mut candidates=self.ids.clone();
            if let Some(s)=ctx.record_get_json(R::Team,team,"last_starting") {
                if let Ok(Value::Array(a))=serde_json::from_str::<Value>(&s){candidates.extend(a.iter().filter_map(Value::as_u64));}
            }
            candidates.sort_unstable();candidates.dedup();
            // 기존 명단도 계약을 다시 확인하므로 이적·방출 선수가 남지 않는다.
            self.ids=candidates.into_iter().filter(|id|ctx.record_get_json(R::Athlete,*id as usize,"contract").and_then(|s|contract_team(&s))==Some(team as u64)).collect();
        }
        for _ in 0..32 {
            let Some(id)=self.queue.pop_front() else{break;};
            let own=ctx.record_get_json(R::Athlete,id,"contract").and_then(|s|contract_team(&s))==Some(team as u64);
            if own&&!self.ids.contains(&(id as u64)) {self.ids.push(id as u64);}
        }
        {
            // 용량 초과/불완전 명단은 무장하지 않는다.
            if self.ids.len()>32 {crate::tactics::set_my_athletes(&[]);return false;}
            crate::tactics::set_my_athletes(&self.ids);
        }
        !self.ids.is_empty()
    }
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn only_current_contract(){
        assert_eq!(contract_team(r#"{"InContract":{"team_id":7}}"#),Some(7));
        assert_eq!(contract_team(r#"{"FreeAgent":{"requests":[{"team_id":7}]}}"#),None);
        assert_eq!(contract_team(r#"{"InContract":{"team_id":"7"}}"#),None);
    }
}
