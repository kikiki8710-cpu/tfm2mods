//! 분석에서 FILE로 확인한 기존6개와 EED표9개 항목의 시작 시점 loaded 값 관측.
//! 두 번 같은 값이라는 사실은 매 호출 소비·계속 불변·내부 분기 직접 관측 증명이 아니다.
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[derive(Clone,Copy)]
struct Site { rva:u64,width:u8,expected:u64,consumers:&'static str }
const SITES:[Site;15]=[
    Site{rva:0x3b69980,width:4,expected:0,consumers:"FFA970/FD4260"},
    Site{rva:0x3b69984,width:4,expected:2,consumers:"FFA970/FD4260"},
    Site{rva:0x3b69988,width:4,expected:3,consumers:"FFA970/FD4260"},
    Site{rva:0x3b6d318,width:8,expected:0x10,consumers:"FE8F30(line0..2 모델 도메인)"},
    Site{rva:0x3b6d320,width:8,expected:0x90,consumers:"FE8F30(line0..2 모델 도메인)"},
    Site{rva:0x3b6d328,width:8,expected:0xd0,consumers:"FE8F30(line0..2 모델 도메인)"},
    Site{rva:0x3b50ce0,width:8,expected:0x13f8,consumers:"EED870(mode0..8 모델 지원도메인)"},
    Site{rva:0x3b50ce8,width:8,expected:0x1400,consumers:"EED870(mode0..8 모델 지원도메인)"},
    Site{rva:0x3b50cf0,width:8,expected:0x1400,consumers:"EED870(mode0..8 모델 지원도메인)"},
    Site{rva:0x3b50cf8,width:8,expected:0x1400,consumers:"EED870(mode0..8 모델 지원도메인)"},
    Site{rva:0x3b50d00,width:8,expected:0x1400,consumers:"EED870(mode0..8 모델 지원도메인)"},
    Site{rva:0x3b50d08,width:8,expected:0x1408,consumers:"EED870(mode0..8 모델 지원도메인)"},
    Site{rva:0x3b50d10,width:8,expected:0x1400,consumers:"EED870(mode0..8 모델 지원도메인)"},
    Site{rva:0x3b50d18,width:8,expected:0x13f8,consumers:"EED870(mode0..8 모델 지원도메인)"},
    Site{rva:0x3b50d20,width:8,expected:0x13f8,consumers:"EED870(mode0..8 모델 지원도메인)"},
];
struct Row { site:Site,address:u64,first:Result<u64,&'static str>,second:Result<u64,&'static str> }
impl Row {
    fn matches(&self)->bool { self.first==Ok(self.site.expected)&&self.second==Ok(self.site.expected) }
}
fn snapshot<F:FnMut(u64,u8)->Result<u64,&'static str>>(base:u64,mut read:F)->Result<Vec<Row>,String>{
    let mut rows=Vec::new();
    for site in SITES{
        let address=base.checked_add(site.rva).ok_or("loaded constant address overflow")?;
        address.checked_add(site.width as u64).ok_or("loaded constant end overflow")?;
        let first=read(address,site.width);let second=read(address,site.width);
        rows.push(Row{site,address,first,second});
    }
    Ok(rows)
}
fn encoded_read(result:&Result<u64,&'static str>)->String{
    match result {Ok(v)=>format!("{{\"value\":{v},\"error\":null}}"),Err(e)=>format!("{{\"value\":null,\"error\":{}}}",crate::engine::json_quote(e))}
}
fn save(dir:&Path,run:&str,base:u64,rows:&[Row])->Result<(),String>{
    if run.is_empty()||!run.bytes().all(|x|x.is_ascii_digit()||x==b'_'){return Err("loaded constant run_id 형식 오류".into());}
    let matched=rows.len()==SITES.len()&&rows.iter().all(Row::matches);
    let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e|e.to_string())?.as_millis();
    let records=rows.iter().map(|r|format!(
        "{{\"rva\":\"0x{:x}\",\"address\":\"0x{:x}\",\"width\":{},\"expected_file_value\":{},\"first\":{},\"second\":{},\"both_equal_expected\":{},\"consumers\":{}}}",
        r.site.rva,r.address,r.site.width,r.site.expected,encoded_read(&r.first),encoded_read(&r.second),r.matches(),crate::engine::json_quote(r.site.consumers))).collect::<Vec<_>>().join(",");
    let text=format!("{{\"schema_version\":1,\"run_id\":{},\"game_version\":\"0.6.4\",\"module_base\":\"0x{base:x}\",\"timestamp_unix_ms\":{stamp},\"matched\":{matched},\"records\":[{records}],\"scope\":{},\"actual_internal_branch_trace\":false,\"per_call_consumption_trace\":false}}\n",
        crate::engine::json_quote(run),crate::engine::json_quote("정체성이 검증된 armed EXE에서 시작 시점15항목을 각각2회 safe_read. 계속 불변이나 매 호출 소비의 직접 관측은 아님."));
    let path=dir.join(format!("loaded_constants_{run}.json"));
    let mut file=OpenOptions::new().create_new(true).write(true).open(&path).map_err(|e|format!("loaded constant report create: {e}"))?;
    file.write_all(text.as_bytes()).map_err(|e|format!("loaded constant report write: {e}"))?;
    file.sync_all().map_err(|e|format!("loaded constant report sync: {e}"))?;
    if matched{Ok(())}else{Err(format!("loaded constant 값/읽기 불일치. 설치 전 거절, 근거={}",path.display()))}
}
pub fn capture(dir:&Path,run:&str)->Result<(),String>{
    let base=crate::platform::module_base().map_err(str::to_string)? as u64;
    let rows=snapshot(base,|address,width|match width{
        4=>crate::platform::safe_read_u32(address as usize).map(u64::from),
        8=>crate::platform::safe_read_u64(address as usize),
        _=>Err("loaded constant width 미지원"),
    })?;
    save(dir,run,base,&rows)
}

#[cfg(test)] mod tests{
    use super::*;
    #[test]fn two_reads_detect_wrong_unstable_unreadable_and_overflow(){
        let base=0x1000;let mut calls=0;
        let rows=snapshot(base,|addr,w|{let i=calls/2;calls+=1;assert_eq!(addr,base+SITES[i].rva);assert_eq!(w,SITES[i].width);Ok(SITES[i].expected)}).unwrap();
        assert_eq!(calls,SITES.len()*2);assert!(rows.iter().all(Row::matches));
        let mut calls=0;let rows=snapshot(base,|_,_|{let i=calls/2;let read=calls%2;calls+=1;
            if i==0&&read==1{Ok(1)}else if i==1{Err("fixture unreadable")}else{Ok(SITES[i].expected)}}).unwrap();
        assert!(!rows[0].matches());assert!(!rows[1].matches());assert!(rows[2..].iter().all(Row::matches));
        let mut calls=0;assert!(snapshot(u64::MAX,|_,_|{calls+=1;Ok(0)}).is_err());assert_eq!(calls,0);
    }
    #[test]fn report_is_evidence_and_create_new_never_replaces(){
        let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let dir=std::env::temp_dir().join(format!("aiav_constants_{}_{stamp}",std::process::id()));std::fs::create_dir(&dir).unwrap();
        let mut n=0;let rows=snapshot(0x1000,|_,_|{let i=n/2;n+=1;Ok(SITES[i].expected)}).unwrap();
        save(&dir,"1_2",0x1000,&rows).unwrap();let path=dir.join("loaded_constants_1_2.json");let old=std::fs::read(&path).unwrap();
        assert!(save(&dir,"1_2",0x1000,&rows).is_err());assert_eq!(std::fs::read(&path).unwrap(),old);
        let bad=snapshot(0x1000,|_,_|Err("fixture unreadable")).unwrap();assert!(save(&dir,"1_3",0x1000,&bad).is_err());
        let text=std::fs::read_to_string(dir.join("loaded_constants_1_3.json")).unwrap();assert!(text.contains("\"matched\":false")&&text.contains("fixture unreadable"));
        assert!(save(&dir,"../escape",0x1000,&rows).is_err());
    }
    #[test]fn eed_file9q_table_is_additive_and_each_failure_rejects(){
        assert_eq!(SITES.len(),15);
        let values=[0x13f8u64,0x1400,0x1400,0x1400,0x1400,0x1408,0x1400,0x13f8,0x13f8];
        for (i,value) in values.iter().enumerate(){let site=SITES[6+i];assert_eq!(site.rva,0x3b50ce0+(i as u64)*8);assert_eq!(site.width,8);assert_eq!(site.expected,*value);}
        for bad in 6..15 { for failure in 0..3 { let mut n=0;
            let rows=snapshot(0x1000,|_,_|{let i=n/2;let second=n%2;n+=1;
                if i==bad {match failure {0=>Ok(SITES[i].expected^1),1 if second==1=>Ok(SITES[i].expected^1),2=>Err("fixture EED unreadable"),_=>Ok(SITES[i].expected)}}else{Ok(SITES[i].expected)}}).unwrap();
            assert_eq!(n,30);assert!(!rows.iter().all(Row::matches));assert!(rows[..6].iter().all(Row::matches));assert!(!rows[bad].matches());
        }}
    }

}
