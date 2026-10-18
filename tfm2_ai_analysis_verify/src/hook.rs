//! 검증 목록의 Win64 정수 ABI만 지원하는 실험용 inline detour.
//! armed 설정/ABI와 제한 Jcc 외 relocation 없는 명령 경계 검증은 상위 manifest가 책임진다.
//! INT3는 정상 entry 진입만 중재한다. private LTO/외부 VEH/loader-lock/unwind 안전 증명은 아니다.
//! 정상 entry 동시진입은 INT3 원본우회로 중재하며 게임장면 판별에 의존하지 않는다.
//! 실게임 미검증.
use crate::platform;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

pub struct HookSpec {
    pub rva: usize,
    pub prologue: &'static [u8],
    pub body: &'static [u8],
    pub wrapper: usize,
    pub allow_foreign_chain: bool,
    pub branches:&'static [BranchFixup],
}
#[derive(Clone,Copy)]
pub struct BranchFixup{pub offset:usize,pub len:usize,pub target_rva:usize}

// 빌드 시 전체 명령 경계를 감사한 Jcc만 허용한다. 일반 x86 디코더가 아니다.
// 조건 반전 → 14바이트 절대 점프를 건너뛰게 하여 flags/RAX를 바꾸지 않는다.
fn relocated_prefix(spec:&HookSpec,base:usize)->Result<Vec<u8>,&'static str>{
    let mut out=Vec::new();let mut previous=0;
    for b in spec.branches{
        if b.offset<previous||b.offset.checked_add(b.len).ok_or("branch offset overflow")?>spec.prologue.len(){return Err("branch outside audited prefix");}
        let bytes=&spec.prologue[b.offset..b.offset+b.len];
        let (condition,delta)=match bytes{
            [op,d] if (0x70..=0x7f).contains(op)=>(*op&15,*d as i8 as i64),
            [0x0f,op,a,b,c,d] if (0x80..=0x8f).contains(op)=>(*op&15,i32::from_le_bytes([*a,*b,*c,*d])as i64),
            _=>return Err("only audited rel8/rel32 Jcc relocation is supported")};
        let next=spec.rva.checked_add(b.offset+b.len).ok_or("branch next overflow")?;
        let target=next.checked_add_signed(delta as isize).ok_or("branch target overflow")?;
        if target!=b.target_rva{return Err("branch target does not match pinned bytes");}
        if target>=spec.rva&&target<spec.rva+spec.prologue.len(){return Err("internal prefix branch target refused");}
        out.extend_from_slice(&spec.prologue[previous..b.offset]);
        out.extend_from_slice(&[0x70|(condition^1),14]);
        rip_jump(base.checked_add(target).ok_or("absolute branch overflow")?,&mut out);
        previous=b.offset+b.len;
    }
    out.extend_from_slice(&spec.prologue[previous..]);Ok(out)
}
#[derive(Clone, Copy, Debug)]
pub struct InstalledHook {
    pub original: usize,
    pub foreign_chain: bool,
    /// vanilla는 원 함수 진입, FOREIGN_CHAIN은 기존 48B8 detour의 다음 target.
    pub target: usize,
}
// 실패 후에도 같은 RVA를 재시도하지 않는다. 프레임별 재후킹/체인 변경 추적 없음.
static ATTEMPTED: Mutex<Vec<usize>> = Mutex::new(Vec::new());

fn rip_jump(target: usize, bytes: &mut Vec<u8>) {
    bytes.extend_from_slice(&[0xff, 0x25, 0, 0, 0, 0]);
    bytes.extend_from_slice(&(target as u64).to_le_bytes());
}
fn mov_rax_target(bytes: &[u8]) -> Option<usize> {
    if bytes.len() >= 12 && bytes[..2] == [0x48, 0xb8] && bytes[10..12] == [0xff, 0xe0] {
        Some(u64::from_le_bytes(bytes[2..10].try_into().ok()?) as usize)
    } else { None }
}
fn inspect_chain(first: usize, entry: usize, wrapper: usize) -> Result<(), &'static str> {
    let mut seen = [0usize; 16];
    let mut target = first;
    for i in 0..seen.len() {
        if target == entry || target == wrapper || seen[..i].contains(&target) {
            return Err("foreign detour cycle/own-wrapper target refused");
        }
        platform::executable_range(target, 12)?;
        seen[i] = target;
        let mut head = [0u8; 12];
        platform::safe_read_bytes(target, &mut head)?;
        if let Some(next) = mov_rax_target(&head) { target = next; }
        else { return Ok(()); }
    }
    Err("foreign detour chain depth exceeds bound")
}

/// original은 wrapper가 Acquire로 읽는다. wrapper는 원본을 딱 한 번 실행해야 한다.
/// 호출자는 모든 설치 결과/domain 게시를 끝내기 전까지 비교 기능을 꺼 두어야 한다.
/// 오류가 나도 original!=0이면 코드 쓰기가 시도된 상태: 해제/0 복원/재시도 금지.
/// 동적 unwind 등록/일반 relocation/원래 return-address 동일성 보존은 제공하지 않는다.
pub fn install_once(spec: &HookSpec, original: &AtomicUsize) -> Result<InstalledHook, &'static str> {
    let mut attempted = ATTEMPTED.lock().map_err(|_| "hook installer lock poisoned")?;
    if attempted.contains(&spec.rva) { return Err("hook RVA already attempted; restart required"); }
    attempted.push(spec.rva);
    if original.load(Ordering::Acquire) != 0 { return Err("original pointer already published"); }
    if spec.prologue.len() < 12 || spec.prologue.len() > 64 || spec.body.len() < spec.prologue.len()
        || spec.body.len() > 16 * 1024 * 1024 || !spec.body.starts_with(spec.prologue) {
        return Err("invalid pinned body/prologue specification");
    }
    platform::init_safe_reader()?;
    let base = platform::module_base()?;
    install_at(spec, base, original, InstallBehavior::default())
}

#[derive(Default)]
struct InstallBehavior {
    #[cfg(test)] fail_after_partial_tail: bool,
    #[cfg(test)] wait_for_gate_hits: u64,
    #[cfg(test)] outside_prefix: Option<&'static PrefixSafepoint>,
}
// 자기소유 native 시험만의 장벽이다. 실제 설치 경로나 가드를 대체하지 않는다.
#[cfg(test)]
struct PrefixSafepoint {
    request: std::sync::atomic::AtomicBool,
    release: std::sync::atomic::AtomicBool,
    acknowledged: AtomicUsize,
    completed: AtomicUsize,
}
fn install_at(spec: &HookSpec, base: usize, original: &AtomicUsize, behavior: InstallBehavior) -> Result<InstalledHook, &'static str> {
    let _ = &behavior;
    if platform::debugger_present() { return Err("debugger attached; INT3 install refused"); }
    let entry = base.checked_add(spec.rva).ok_or("hook RVA overflow")?;
    platform::executable_range(entry, spec.body.len())?;
    platform::executable_range(spec.wrapper, 1)?;
    if spec.wrapper == entry { return Err("wrapper equals original entry"); }
    let mut before = vec![0u8; spec.body.len()];
    platform::safe_read_bytes(entry, &mut before)?;
    let foreign_target = mov_rax_target(&before);
    let foreign = before != spec.body;
    let target = if foreign {
        if !spec.allow_foreign_chain { return Err("body differs; foreign chain disabled"); }
        let next = foreign_target.ok_or("unknown entry patch/body change refused")?;
        if before[12..] != spec.body[12..] { return Err("foreign body differs outside first 12 bytes"); }
        inspect_chain(next, entry, spec.wrapper)?;
        next
    } else { entry };
    let patch_len = if foreign { 12 } else { spec.prologue.len() };
    let mut trampoline = Vec::with_capacity(spec.prologue.len() + 14);
    if foreign {
        // 기존 entry의 MOV RAX,next / JMP RAX를 그대로 재현한다.
        // RIP 간접 jump만 쓰면 외부 wrapper 진입 RAX가 CALL 시점 잔류값이 된다.
        trampoline.extend_from_slice(&before[..12]);
    }
    else {
        // copied prologue에 RIP/분기/CALL/분할 명령이 없다는 근거는 고정 manifest에 둔다.
        trampoline.extend_from_slice(&relocated_prefix(spec,base)?);
        let resume = entry.checked_add(spec.prologue.len()).ok_or("trampoline resume overflow")?;
        rip_jump(resume, &mut trampoline);
    }
    let stub = platform::allocate_code(&trampoline)?;
    let mut patch = vec![0x90u8; patch_len];
    patch[..2].copy_from_slice(&[0x48, 0xb8]);
    patch[2..10].copy_from_slice(&(spec.wrapper as u64).to_le_bytes());
    patch[10..12].copy_from_slice(&[0xff, 0xe0]);
    let mut recheck = vec![0u8; spec.body.len()];
    // 원본과 영구 descriptor를 CC 전에 게시한다. 이 시점 이후 stub은 회수하지 않는다.
    // 지연 INT3도 terminal 상태와 무관하게 정확한 원본 trampoline으로 한 번 우회한다.
    if let Err(reason) = platform::register_breakpoint_gate(entry, stub) {
        unsafe { platform::free_unpublished_code(stub); }
        return Err(reason);
    }
    original.store(stub, Ordering::Release);
    let gate = unsafe { platform::publish_code_byte(entry, before[0], 0xcc) };
    gate.result?;
    #[cfg(test)]
    if behavior.wait_for_gate_hits != 0 {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while platform::breakpoint_gate_hits(entry) < behavior.wait_for_gate_hits {
            if std::time::Instant::now() >= deadline {
                unsafe { platform::publish_code_byte(entry, 0xcc, before[0]) }.result?;
                return Err("fixture breakpoint worker progress timeout; tail not written");
            }
            std::thread::yield_now();
        }
    }
    #[cfg(test)]
    if let Some(point) = behavior.outside_prefix {
        point.request.store(true, Ordering::Release);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while point.acknowledged.load(Ordering::Acquire) != 4 {
            if std::time::Instant::now() >= deadline {
                unsafe { platform::publish_code_byte(entry, 0xcc, before[0]) }.result?;
                return Err("fixture prefix-outside acknowledgement timeout; tail not written");
            }
            std::thread::yield_now();
        }
    }
    // CC 이후 Suspend/GetContext 실패에도 tail은 아직 미수정이다. 원복 성공만 clean refusal.
    let mut stopped = match platform::SuspendedThreads::acquire(entry, patch_len) {
        Ok(g) => g,
        Err(reason) => {
            unsafe { platform::publish_code_byte(entry, 0xcc, before[0]) }.result?;
            return Err(reason);
        }
    };
    // 이 구간은 Rust 할당/로그/일반 mutex/게임 CALL을 하지 않는다.
    let checked = platform::safe_read_bytes(entry, &mut recheck);
    if checked.is_err() || recheck[0] != 0xcc || recheck[1..] != before[1..] {
        let restored = unsafe { platform::publish_code_byte(entry, 0xcc, before[0]) };
        let resumed = stopped.resume_all();
        restored.result?;
        resumed?;
        return Err("body changed/unreadable after INT3 suspension; tail not written");
    }
    #[cfg(test)]
    if behavior.fail_after_partial_tail {
        // 자기소유 native fixture 전용: 일부 tail을 실제 쓴 뒤 영구 CC fallback을 시험한다.
        let partial = unsafe { platform::write_code(entry + 1, &patch[1..3]) };
        let resumed = stopped.resume_all();
        partial.result?;
        resumed?;
        return Err("fixture injected partial tail failure; permanent INT3 fallback retained");
    }
    // tail 쓰기가 시도된 뒤에는 첫 byte를 원문으로 되돌리면 안 된다.
    let tail = unsafe { platform::write_code(entry + 1, &patch[1..]) };
    if let Err(reason) = tail.result {
        stopped.resume_all()?;
        return Err(reason);
    }
    let checked = platform::safe_read_bytes(entry, &mut recheck[..patch_len]);
    if checked.is_err() || recheck[0] != 0xcc || recheck[1..patch_len] != patch[1..] {
        stopped.resume_all()?;
        return Err("tail readback failed; permanent INT3 fallback retained");
    }
    // write_code가 tail cache flush까지 성공한 뒤에만 1byte를 원자 게시한다.
    let committed = unsafe { platform::publish_code_byte(entry, 0xcc, patch[0]) };
    let resumed = stopped.resume_all();
    committed.result?;
    resumed?;
    Ok(InstalledHook { original: stub, foreign_chain: foreign, target })
}

#[cfg(test)]mod tests{
    use super::*;
    static BRANCH:&[BranchFixup]=&[BranchFixup{offset:7,len:2,target_rva:0x1014}];
    static PREFIX:&[u8]=&[0x48,0x89,0xc8,0x48,0x85,0xc9,0x90,0x74,0x0b,0x48,0x83,0xc0,7];
    fn spec(branches:&'static [BranchFixup])->HookSpec{HookSpec{rva:0x1000,prologue:PREFIX,body:PREFIX,wrapper:0,allow_foreign_chain:false,branches}}
    #[test]fn relocated_conditional_preserves_taken_and_fallthrough_outputs(){
        let mut body=PREFIX.to_vec();body.push(0xc3);body.resize(20,0x90);body.extend_from_slice(&[0xb8,9,0,0,0,0xc3]);
        let origin=platform::allocate_code(&body).unwrap();let base=origin-0x1000;
        let mut prefix=relocated_prefix(&spec(BRANCH),base).unwrap();rip_jump(origin+PREFIX.len(),&mut prefix);
        let copy=platform::allocate_code(&prefix).unwrap();
        let original:unsafe extern "system" fn(u64)->u64=unsafe{std::mem::transmute(origin)};
        let relocated:unsafe extern "system" fn(u64)->u64=unsafe{std::mem::transmute(copy)};
        for input in [0,1,2,u64::MAX]{assert_eq!(unsafe{original(input)},unsafe{relocated(input)});}
        unsafe{platform::free_unpublished_code(copy);platform::free_unpublished_code(origin);}
    }
    #[test]fn unsupported_or_wrong_target_relocation_is_refused(){
        static WRONG:&[BranchFixup]=&[BranchFixup{offset:7,len:2,target_rva:0x1015}];
        static BAD:&[BranchFixup]=&[BranchFixup{offset:0,len:2,target_rva:0x1014}];
        assert!(relocated_prefix(&spec(WRONG),0x10000).is_err());assert!(relocated_prefix(&spec(BAD),0x10000).is_err());
    }
    // 이 native 시험은 자기소유 RX 버퍼만 수정한다. 게임/실 provider 실행이 아니다.
    // 전체 테스트는 RUST_TEST_THREADS=1로 실행해 다른 시험의 OS/allocator 락과 분리한다.
    static NATIVE_PATCH_TEST: Mutex<()> = Mutex::new(());
    static TEST_ORIGINAL: AtomicUsize = AtomicUsize::new(0);
    static WRAPPER_CALLS: AtomicUsize = AtomicUsize::new(0);
    static COMMIT_ORIGINAL: AtomicUsize = AtomicUsize::new(0);
    static COMMIT_WRAPPER_CALLS: AtomicUsize = AtomicUsize::new(0);
    static COMMIT_SAFEPOINT: PrefixSafepoint = PrefixSafepoint {
        request: std::sync::atomic::AtomicBool::new(false),
        release: std::sync::atomic::AtomicBool::new(false),
        acknowledged: AtomicUsize::new(0), completed: AtomicUsize::new(0),
    };
    static FAILURE_ORIGINAL: AtomicUsize = AtomicUsize::new(0);
    static FIXTURE_BODY: &[u8] = &[0x48,0x89,0xc8,0x48,0x83,0xc0,7,0x90,0x90,0x90,0x90,0x90,0xc3];
    unsafe extern "system" fn native_wrapper(a:u64,b:u64,c:u64,d:u64)->u64 {
        let f:unsafe extern "system" fn(u64,u64,u64,u64)->u64=std::mem::transmute(TEST_ORIGINAL.load(Ordering::Acquire));
        WRAPPER_CALLS.fetch_add(1,Ordering::Relaxed);
        f(a,b,c,d)
    }
    unsafe extern "system" fn failure_wrapper(a:u64,b:u64,c:u64,d:u64)->u64 {
        let f:unsafe extern "system" fn(u64,u64,u64,u64)->u64=std::mem::transmute(FAILURE_ORIGINAL.load(Ordering::Acquire));
        f(a,b,c,d)
    }
    unsafe extern "system" fn commit_wrapper(a:u64,b:u64,c:u64,d:u64)->u64 {
        let f:unsafe extern "system" fn(u64,u64,u64,u64)->u64=std::mem::transmute(COMMIT_ORIGINAL.load(Ordering::Acquire));
        COMMIT_WRAPPER_CALLS.fetch_add(1,Ordering::Relaxed); f(a,b,c,d)
    }
    fn native_spec(wrapper:usize)->HookSpec { HookSpec { rva:0,prologue:&FIXTURE_BODY[..12],body:FIXTURE_BODY,wrapper,allow_foreign_chain:false,branches:&[] } }
    // assertion/설치 panic에서도 worker를 먼저 풀고 모두 join한다. 실제 게임 스레드에는 사용하지 않는다.
    struct FixtureWorkers {
        stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
        calls: std::sync::Arc<AtomicUsize>,
        point: Option<&'static PrefixSafepoint>,
        handles: Vec<std::thread::JoinHandle<()>>,
    }
    impl FixtureWorkers {
        fn start(entry:usize, point:Option<&'static PrefixSafepoint>)->Self {
            let stop=std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let ready=std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let calls=std::sync::Arc::new(AtomicUsize::new(0));
            let mut workers=Self{stop:stop.clone(),calls:calls.clone(),point,handles:Vec::new()};
            for worker in 0..4 {
                let stop=stop.clone();let ready=ready.clone();let calls=calls.clone();
                workers.handles.push(std::thread::spawn(move|| {
                    let f:unsafe extern "system" fn(u64,u64,u64,u64)->u64=unsafe{std::mem::transmute(entry)};
                    // 부분 spawn 실패에서도 stop으로 빠져나오게 고정 Barrier를 쓰지 않는다.
                    let deadline=std::time::Instant::now()+std::time::Duration::from_secs(5);
                    while !ready.load(Ordering::Acquire)&&!stop.load(Ordering::Acquire) {
                        assert!(std::time::Instant::now()<deadline,"fixture worker start timeout");
                        std::thread::yield_now();
                    }
                    if stop.load(Ordering::Acquire){return;}
                    let mut i=worker as u64;
                    while !stop.load(Ordering::Acquire) {
                        if let Some(point)=point {
                            if point.request.load(Ordering::Acquire) {
                                // 앞 f가 반환하고 출력검사까지 끝난 위치: stolen prefix 밖이다.
                                point.acknowledged.fetch_add(1,Ordering::AcqRel);
                                let deadline=std::time::Instant::now()+std::time::Duration::from_secs(5);
                                while !point.release.load(Ordering::Acquire)&&!stop.load(Ordering::Acquire) {
                                    assert!(std::time::Instant::now()<deadline,"fixture worker release timeout");
                                    std::thread::yield_now();
                                }
                                if stop.load(Ordering::Acquire){break;}
                                assert_eq!(unsafe{f(i,0,0,0)},i.wrapping_add(7));
                                calls.fetch_add(1,Ordering::Relaxed);
                                point.completed.fetch_add(1,Ordering::Release);
                                break;
                            }
                        }
                        assert_eq!(unsafe{f(i,0,0,0)},i.wrapping_add(7));
                        calls.fetch_add(1,Ordering::Relaxed);i=i.wrapping_add(4);
                        std::thread::yield_now();
                    }
                }));
            }
            ready.store(true,Ordering::Release);workers
        }
        fn finish(&mut self)->bool {
            self.stop.store(true,Ordering::Release);
            if let Some(point)=self.point {point.release.store(true,Ordering::Release);}
            let mut success=true;
            for worker in self.handles.drain(..){success &= worker.join().is_ok();}
            success
        }
    }
    impl Drop for FixtureWorkers {fn drop(&mut self){let _=self.finish();}}
    #[test]
    fn native_int3_gate_four_workers_preserve_outputs_during_commit() {
        let _serial=NATIVE_PATCH_TEST.lock().unwrap_or_else(|e|e.into_inner());
        let entry=platform::allocate_code(FIXTURE_BODY).unwrap();
        let mut workers=FixtureWorkers::start(entry,None);
        let result=install_at(&native_spec(native_wrapper as *const () as usize),entry,&TEST_ORIGINAL,
            InstallBehavior{wait_for_gate_hits:128,..InstallBehavior::default()});
        assert!(workers.finish(),"모든 worker 출력과 정상 join 필요");
        assert!(platform::breakpoint_gate_hits(entry)>=128);
        assert!(workers.calls.load(Ordering::Relaxed)>=128);
        assert_ne!(TEST_ORIGINAL.load(Ordering::Acquire),0);
        let committed=match result {
            Ok(installed)=>{assert_eq!(installed.original,TEST_ORIGINAL.load(Ordering::Acquire));true},
            Err("thread RIP inside patch range; refusing patch")=>{
                // 무제어 진입에서 보수 가드 거절은 허용하되 skip하지 않고 원문/꼬리를 검사한다.
                let mut body=vec![0;FIXTURE_BODY.len()];platform::safe_read_bytes(entry,&mut body).unwrap();
                assert_eq!(body,FIXTURE_BODY);assert_eq!(&body[1..12],&FIXTURE_BODY[1..12]);
                false
            },
            Err(reason)=>panic!("예상하지 않은 동시 설치 실패: {reason}"),
        };
        let f:unsafe extern "system" fn(u64,u64,u64,u64)->u64=unsafe{std::mem::transmute(entry)};
        for i in [0,1,42,u64::MAX] {assert_eq!(unsafe{f(i,0,0,0)},i.wrapping_add(7));}
        if committed {
            assert!(WRAPPER_CALLS.load(Ordering::Relaxed)>=4);
            let mut byte=[0];platform::safe_read_bytes(entry,&mut byte).unwrap();assert_eq!(byte[0],0x48);
        }else{assert_eq!(WRAPPER_CALLS.load(Ordering::Relaxed),0);}
        println!("무제어4worker 결과: {} / gatehits={} / outputs={}",
            if committed{"COMMITTED"}else{"REFUSED_RIP_INSIDE_ORIGINAL_INTACT"},
            platform::breakpoint_gate_hits(entry),workers.calls.load(Ordering::Relaxed));
        // entry/stub/descriptor는 실제 설치와 같이 회수하지 않는다.
    }
    #[test]
    fn native_four_workers_outside_prefix_must_commit_then_enter_wrapper() {
        let _serial=NATIVE_PATCH_TEST.lock().unwrap_or_else(|e|e.into_inner());
        let entry=platform::allocate_code(FIXTURE_BODY).unwrap();
        let mut workers=FixtureWorkers::start(entry,Some(&COMMIT_SAFEPOINT));
        let result=install_at(&native_spec(commit_wrapper as *const () as usize),entry,&COMMIT_ORIGINAL,
            InstallBehavior{wait_for_gate_hits:128,outside_prefix:Some(&COMMIT_SAFEPOINT),..InstallBehavior::default()});
        // 설치 실패도 먼저 worker를 해제한다. 실제 commit 성공 여부는 join 이후 판정한다.
        COMMIT_SAFEPOINT.release.store(true,Ordering::Release);
        let deadline=std::time::Instant::now()+std::time::Duration::from_secs(5);
        while COMMIT_SAFEPOINT.completed.load(Ordering::Acquire)!=4&&std::time::Instant::now()<deadline {
            std::thread::yield_now();
        }
        let completed=COMMIT_SAFEPOINT.completed.load(Ordering::Acquire);
        assert!(workers.finish(),"모든 worker 출력과 정상 join 필요");
        let installed=result.expect("prefix밖4ack 뒤 실제 commit 필수");
        assert_eq!(COMMIT_SAFEPOINT.acknowledged.load(Ordering::Acquire),4);assert_eq!(completed,4);
        assert_eq!(installed.original,COMMIT_ORIGINAL.load(Ordering::Acquire));
        assert!(platform::breakpoint_gate_hits(entry)>=128);
        assert!(workers.calls.load(Ordering::Relaxed)>=132);
        assert_eq!(COMMIT_WRAPPER_CALLS.load(Ordering::Relaxed),4);
        let mut head=[0;12];platform::safe_read_bytes(entry,&mut head).unwrap();
        assert_eq!(&head[..2],&[0x48,0xb8]);assert_eq!(&head[10..],&[0xff,0xe0]);
        assert_eq!(u64::from_le_bytes(head[2..10].try_into().unwrap()),commit_wrapper as *const () as u64);
    }
    #[test]
    fn native_partial_tail_failure_keeps_int3_original_fallback() {
        let _serial=NATIVE_PATCH_TEST.lock().unwrap_or_else(|e|e.into_inner());
        let entry=platform::allocate_code(FIXTURE_BODY).unwrap();
        let result=install_at(&native_spec(failure_wrapper as *const () as usize),entry,&FAILURE_ORIGINAL,
            InstallBehavior{fail_after_partial_tail:true,..InstallBehavior::default()});
        assert_eq!(result.unwrap_err(),"fixture injected partial tail failure; permanent INT3 fallback retained");
        assert_ne!(FAILURE_ORIGINAL.load(Ordering::Acquire),0);
        let mut head=[0;3];platform::safe_read_bytes(entry,&mut head).unwrap();
        assert_eq!(head[0],0xcc);assert_eq!(head[1],0xb8);assert_ne!(head[1],FIXTURE_BODY[1]);
        let hits=platform::breakpoint_gate_hits(entry);
        let f:unsafe extern "system" fn(u64,u64,u64,u64)->u64=unsafe{std::mem::transmute(entry)};
        for i in 0u64..32 {assert_eq!(unsafe{f(i,0,0,0)},i+7);}
        assert_eq!(platform::breakpoint_gate_hits(entry),hits+32);
    }

}
