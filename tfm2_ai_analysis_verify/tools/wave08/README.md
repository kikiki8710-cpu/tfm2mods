지시: 기존41 동결식/byte helper를 보존하고 D8C680·FCBBB0·E40560의 독립 후처리와 명명지표 도구를 준비한다.
요약: 준비44/명명111. 기존89회귀+새43시험+matrix36 PASS. native/실측/새raw/함수·계수추가0.

runtime41→models_candidates.evaluate(0xd8c680,args,snapshot),42/43→models_wave08_direct.evaluate(pid,args,snapshot). E1B5B0와 1016B00/E53290은 미배선이다. 기존0..26 handfixture와27..40 저장fixture를 새 metadata에서 재검사하며 기존41 evaluate 식은 새 route/상한guard 이외 AST동일하다. helper8파일은 byte동일 사본이다.

준비44와 root의 이번 선택10을 구별한다. `probe_selection_scope`는 NOT_SELECTED34/선택성공/안전거절을 따로 남기며 미선택 비0표본은 거절한다. 선택10·설치9·거절1의 합성축도 시험했고 선택/설치는 runtime비교완료가 아니다. 실제 선택/거절/호출은 후속 고정 summary/설정영수증으로 확인한다.

merge 정상 fullRAX0/모든 QWORD output front·back·oddcenter 순서와 실제값을 비교한다. source/header/실제 score read-out alias는 모델이 거절하며 원래 읽기 첫 unique순서·전량소비가 달라지면 거절한다. cap16은 게임상한이 아니다. 직접2는 AL만 비교하므로 상위RAX는 원래잔류 범위로 남긴다. invalidteam/panic/진단·alias/객체동시성 미지원은 SKIP이며 무쓰기나 정상반환으로 발명하지 않는다.

```powershell
python -B -X utf8 analyze_logs_wave08.py --summary <STOP.json> --observations <fixed.jsonl> --journal <summaries_RUN.jsonl> --expected-run-id <RUN> --expected-stop-record <ACK_RECORD> --input-origin runtime --output-dir <새폴더>
```

정본 전체journal은 완전줄/중복키/연속record/run/STOP정본 동일성/drain/후속OFF 표본카운터 불변을 검수한다. STOP뒤 calls 증가는 표본추가가 아니므로 허용하고 실제 줄을 자르지 않는다. journal 부재면 runtime freshness미확정으로 표시하며 matrix는 승격을 거절한다. 실제 벽시계 freshness·세션/DLL 신원은 자동입증하지 않는다.

`runtime_metric_matrix_wave08/build_matrix.py`는 분석기/helper/명세 source핀과 입력·독립analysis SHA를 재검수한다. `metric_plan_wave08/matrix_plan.json`은 기존97+merge2+직접12를 담고 반복evaluation과 함수표본, exact(rhs±1), 동적비교·코드/sentinel·고정계수장부를 분리한다. `numeric_consumption_ledger.json`은 기존원값/소비점/폭을 source snapshot에 연결하며 실제계수완료를 늘리지 않는다.

```powershell
python -B -X utf8 runtime_metric_matrix_wave08/build_matrix.py --summary <STOP.json> --observations <fixed.jsonl> --analysis-json <analysis.json> --expected-run-id <RUN> --input-origin runtime --output-dir <새matrix폴더>
```

canonical0.6.4/외부chain허용false/bodyguard의 root 선택은 loaded/foreign provider나 전역LTO 증명과 별개다. root 실제배선·native/게임은 이 scratch후처리 동결과 다른 증거다. 기존M/GUI/원장·동결원본을 수정하지 않았다.
