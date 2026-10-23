"""구현 보고서 생성. 개인 런타임 파일은 별도 경로에만 보관한다."""
import json
from pathlib import Path
MOD=Path(__file__).resolve().parent.parent
REPORT=Path(r'C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_detail_tactics')
OUT=REPORT/'implement_20261024'
for part,filename in [('ops','ops_implementation_conclusion.md'),('combat','combat_impl_part01_conclusion.md'),('lane','lane_implementation_conclusion.md')]:
    p=OUT/filename
    if p.exists():
        raw=p.read_text(encoding='utf-8')
        target=REPORT/'RE'/f'2026-10-24_56종구현_{part}.md'
        target.write_text('지시: 정적 연결 확보56개를 모두 실제 상세전술에 구현하고 검증한다.\n요약:\n- 담당 제품 콜백과 동적 명령을 구현했다.\n- 실제 Rust 콜백과 독립 정책식, 원본 기계어를 대조했다.\n- 아래 원문은 담당별 전체 결과이며 실게임 검수와 구분한다.\n\n'+raw,encoding='utf-8',newline='\n')
rows=sorted(sum([json.loads((MOD/'tools'/'targets'/f'extra_{p}_0.6.5.json').read_text(encoding='utf-8'))['tactics'] for p in ('lane','combat','ops')],[]),key=lambda t:t['audit_id'])
summary='''# 56개 추가 상세전술 구현·검증

2026-10-24 · 게임0.6.5 · 상세전술0.2.0. 기존32개에 확정 후보56개를 더한 **88개 전술·197개 실제 적용창**이다. 설계10개는 이번56개에 포함되지 않는다. 정적 원장149창 중 원본 보존 참조5창은 패치하지 않아 추가창은144개다.

## 제품 구조

기존32개 저장ID/번호와 v1 문법을 유지하고, 새56개는 감사ID 순서로 뒤에 추가한다. 신규 세이브 선택 기본값은 전부 기본(원본 우회)다. 한 프로필에 묶인 복수 문턱은 같은 선택값에서 파생한다. Tf::Compute는 현재 읽기 전용 Frame으로 원래 TPS·거리제곱·정수폭·부호·carry 규칙을 재현하며 게임 헬퍼를 호출하지 않는다.

현재 Player의 계약 선수ID·팀·포지션과 Caller whitelist를 검증하고, 미등록/읽기 실패/상대 선수는 원본을 유지한다. 실제 스킬 사용 승인과 선행 역할·상대 보정 예외는 유지한다. 설정 전체 쓰기 동안 revision을 변경해 각 dispatch가 섞인 단계/값을 읽으면 원본을 선택한다. 여러 창을 가로지르는 게임 함수 전체 snapshot을 제공한다는 의미는 아니다.

기존6탭에 오브젝트/위치·기술을 추가했다. 각 탭은 별도의 native scroll_view이며 헤더·하단 버튼을 고정한다. 탭 전환 시 초안/스크롤 위치를 유지하며 스크롤된 버튼의 실제 rect로 풀다운을 배치한다. 선수190/팀990/듀오390 폭을 선언 시 고정하고 글자 가운데 정렬과 원딜·서폿 공유칸을 유지한다.

## 자동 검증

- 공용 생성기: EXE SHA/197창 바이트·명령경계·함수범위·분기/JT/전역rel32 내부진입·전체창 중복 검사 통과. 88개 모두 연결, 미지원0.
- Rust 단위검사32개: 저장88ID 왕복, 기본 우회, 전체창 재배치, 역할표, 마지막행 편집, native 설치 fixture 및 기존 검사를 통과.
- 기존53창의 전체 스텁 Unicorn 비교9540건 불일치0. 콜백 ABI/모든 GPR/RSP/정의된flags/XMM/분기/원본fallback 포함.
- 추가56개는 담당별 `lane_verify.json`, `combat_native_check.json`, `ops_verify.json` 참조. 실제 제품 Rust callback 실행 및 독립식/동적fragment 차등 통과. 이 검사는 실제 경기 도달/행동효과나 모든 새창의 dispatch FFI 검사를 대신하지 않는다.
- 운영51행은 독립 통합 메타데이터 대조도 통과(`ops_integration_review.json`).

실게임 UI/저장/실제 적용 결과는 후속 `runtime_result.json`에 기록한다. 해당 파일이 없는 시점에는 실게임 통과로 해석하지 않는다.

## 추가 항목

| 감사ID | 전술 | 탭 | 범위 | 기본/낮게/높게 | 창수 |
|---|---|---|---|---|---|
'''
for t in rows:summary+=f"| {t['audit_id']} | {t['name']} | {t['tab']} | {t['scope']} | {t['default']}/{t['low']}/{t['high']} | {len(t['sites'])} |\n"
(REPORT/'18_56종_구현검증.md').write_text(summary,encoding='utf-8',newline='\n')
for name in ('02_구현정보.md','03_시행착오.md'):
    p=REPORT/name;archive=OUT/('previous_'+name)
    if not archive.exists():archive.write_bytes(p.read_bytes())
(REPORT/'02_구현정보.md').write_text('''# 상세전술 구현·검증

2026-10-24 · 현행0.2.0 · 게임0.6.5 · 88전술197창. 소스는 `C:/tfm2mods/tfm2_detail_tactics/`.

현행56종의 목록·변환·설정 주체·UI·검증 구분은 `18_56종_구현검증.md`와 `implement_20261024/`가 정본이다. 기존 구조체/스텁/장벽/세이브 계약과0.1.8까지의 검증은 `implement_20261024/previous_02_구현정보.md`에 원문 보존했다.

| 항목 | 현행 계약 |
|---|---|
| 설정 선수 | Player+0x810 u64 ID / +0x820 u64 team0..1 / +0x8b0 u32 pos0..4 |
| 주체 | Src scalar/Caller최대2단/현재cache lookup. 저장·전역side 추정 불가 |
| 변환 | 기존 Add/Sq1/Map + Compute(Frame,i64)->Option<u64>. typed bitpattern/overflow는 개별계약 |
| 원본 우회 | 기본·비무장·타팀·unknowncaller·읽기실패·설정revision변화 |
| 설치 | CAP256,197창.1회설치/전술단위모두검증후enable |
| 저장 | 기존32ID 보존+신규56ID 후방추가. v1/6칸 형식 |
| UI | 8탭별 native scroll_view,실제rect 풀다운,고정헤더/푸터 |
| 빌드 | nightly-2026-05-24/stable SDK/opt-level1/overflow checks off |
| 자동검증 | Rust32 PASS,기존전체stub9540비교,신규담당별Rust callback/기계어차등PASS |
| 실게임 | runtime_result.json의 개별검수범위 참조. 승률효과는 미측정 |

기존 cfg를 설치 시 덮어쓰지 않는다. 새 전술의 누락 cfg섹션은 제품 기본값으로 동작한다. 패키지 cfg는88개 정의 기본값으로 생성하며 개인 세이브·로그는 제외한다.
''',encoding='utf-8',newline='\n')
(REPORT/'03_시행착오.md').write_text('''# 상세전술 시행착오

## 2026-10-24 · 56종 통합

정적 연결 확정은 기존 즉치 생성기에 그대로 넣을 수 있다는 뜻이 아니었다. TPS곱·signed threshold·32bitADD·ADD/ADCcarry·LEA/SHL/SUB fragment가 있어 단일값 치환으로는 원본동작을 보존할 수 없다. 현재Frame 기반 Compute와 원래 폭의 동적 조각을 쓰고 원본값 중립 비교를 별도로 수행했다. 참고창5개는 설치에서 제외했다.

88행으로 확장하자 Rust 배열 Default derive가32 초과 배열에 대해 실패했다. Editor에 명시적Default를 구현해 해결했으며 최종 단위32개 PASS다. 부분 배열초기화로 우회하지 않았다.

스크롤 도입 시 고정 row좌표로 풀다운을 열면 잘못된 행 위치에 나타난다. 탭별scroll_view와실제버튼rect를 쓰고, clip밖 버튼은 풀다운을 닫는다. interactive목록항목 폭을 런타임에 바꾸면hover가스타일을복구하므로 선수/팀/듀오폭별목록을 소스에 따로선언했다.

단계값파일읽기·확인·세이브로드에서 여러 원자값을 순차갱신하는 동안 한dispatch가중간값을읽을수있어 revisionguard를 추가했다. 함수전체를가로지르는동시스냅샷이라고과장하지않는다.

이전 원인·실패·교훈 원문은 `implement_20261024/previous_03_시행착오.md`에 보존한다. 실게임에서 추가 발견한 결함은 후속 기록한다.
''',encoding='utf-8',newline='\n')
p=REPORT/'01_구조.md';s=p.read_text(encoding='utf-8')
if '## 현행0.2.0' not in s:s=s.replace('\n\n','\n\n## 현행0.2.0 · 2026-10-24\n\n88전술197창. 기존6개+오브젝트/위치·기술의8탭이며 각 탭에 native scroll_view를 둔다. 신규56개 Compute/typed fragment 및 검증은 `18_56종_구현검증.md` 참조. 아래32/53·6탭은0.1.8까지의 이력이다.\n\n',1)
p.write_text(s,encoding='utf-8',newline='\n')
assert all(p.stat().st_size<=15360 for p in [REPORT/'18_56종_구현검증.md',REPORT/'01_구조.md',REPORT/'02_구현정보.md',REPORT/'03_시행착오.md'])
print('보고서/원문 보존 완료')
