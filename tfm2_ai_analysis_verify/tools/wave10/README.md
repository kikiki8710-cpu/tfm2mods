# typed50 모델·142명명지표 독립 후처리 준비

기존47 모델식은 route·arity·범위 guard를 제외하면 AST 동일하다. 기존 formula helper와 audit는 byte 동일하며 구4인수 base도 `legacy_base_analyze_logs.py`로 그대로 보존했다. 이 legacy 진단을 typed 전량에 실행하지 않는다. 새 `base_analyze_logs.py`는 정확4/5/6인수 형식·masked 로그·카운터 진단이고 결과에 범위를 표시한다. 과거3833줄의5/6인수를 실제손실로 오진단하지 않는다. 실제 malformed typed는 계속 거절한다.

시험은 기존89+43+44, 신규typed41, 새형식진단음성3 = 분석기220이다. matrix43+실제함수의분모 protocol2 =45로 합계265 Python 시험 PASS다. 정상360/panic6/관측cap3은 A의 고정 fixture 재사용이며 새rawVM·native·실게임 표본이 아니다. 별도 모델거절14요약에는 args/read가 없으므로 가짜 raw event를 만들지 않았다.

id0..43=정확4args,44/45=정확5args,46..49=정확6args다. 새47/48은fullRAX선택주소 bits,49는AL bool만 비교하며상위actual은보존한다. 반환주소bits를출력ptr로다루어checked_span을추가하지않고실제inputread의주소/폭만검증한다. 세함수 output_fields/안정성예외0이다. DC6번째mode는lowBYTE, EED6번째pad는QWORD이며EED원문이pad6→Y5로읽는명령순서와argv저장은구별한다.

matrix의 `actual_runtime_samples`와 `runtime_samples_received`는 runtime origin에서 같은 고정 raw줄수로 갱신된다. 독립 비교 수는 `actual_runtime_comparisons_rechecked`, 함수 표본 분모가 있는 지표행 수는 `metric_rows_with_function_sample_denominator`, 실제평가행수는 `metric_entries_evaluated`다. fixture origin의실측필드는0이다. loaded상수/DLL독립신원 false는 이 도구 자체가 입증하지 않은 범위 제한으로서 실제다른초기캡처보고서를 부정하지 않는다. 전체계수/branch검증으로승격하지 않는다.

```powershell
python -B -X utf8 analyze_logs_wave10.py --summary <STOP.json> --observations <raw.jsonl> --journal <summaries_RUN.jsonl> --expected-run-id <RUN> --expected-stop-record <ACK> --input-origin runtime --output-dir <새분석폴더>
python -B -X utf8 runtime_metric_matrix_wave10/build_matrix.py --summary <STOP.json> --observations <raw.jsonl> --analysis-json <분석/analysis.json> --expected-run-id <RUN> --input-origin runtime --source-commit <commit> --dll-sha256 <sha> --output-dir <R10/analyzer_wave10_runtime_matrix/새결과>
```

선택subset/거절/미선택/설치/관측을구별한다. STOP정본·전체완전journal·후속OFF sample불변을검수하고원문을자르거나seq순으로정렬하지않는다. basename을 바꿔 복사한 입력은 summary 선언 이름의 byte동일 소유 사본으로 보존할수 있다. summary alias soft오류와hardIO/lost/drop는별개다. stride·상한으로의도제외된호출을유실로합산하지않는다.

정확필수사본19개는 `copy_manifest.json`에있다. source_pin_manifest의 REPORT 절대키를M으로옮길때는원본을보존하고root가SHA동일사본을검증한뒤새M용키만재결박한다. analyzer는 __file__옆helper를검증하며matrix도같은bundle의분석기·명세를사용해야한다. 준비/시험스크립트와fixture는실행의존이아니므로M에서실행하지않는다.

이번 M/GUI/공유원장쓰기·컴파일·native·게임실행·실표본·함수추가0이다. 총괄의native/실설치/실게임결과는별도다.
