# typed53·162명명지표 독립 후처리 준비

기존50식은 route·arity·ID상한을 제외하면 동일하고 formula helper들은 byte동일하다. 기존142지표행과 matrix aggregate·출력검사 함수도 동일하다. 구4인수 진단은 보존하며 전량에 실행하지 않는다. 새 진단은 정확4/5/6/3인수이며 id52만 정확3이다.

Python 시험은 기존220+신규84 = 분석기304, matrix46+분모 protocol2=48, 합계352 PASS다. 정상658/guard7은 고정 모델자료의 후처리 재사용으로 새rawVM/native/게임 증거가 아니다. protocol의 runtime origin은 실제게임 대신 명시된 합성journal로 함수의 분모 필드를 시험한다.

50E60는5인수·RAXout,51F2는6인수·RAXout,52D8은3인수·mask0다. 52의 expected0은 비교하지 않는 placeholder이며 실제fullRAX를변경하지 않는다. 주소·폭·순서·기대값은입력snapshot모델에서독립계산하고 actual output을오라클로사용하지않는다. WORD는2byte이고 None payload/F2out+16Q/padding을기대목록에추가하지않는다. 해당 nominalout24/40/32와실inputread겹침은지원Err,안정성예외0이다.

준비53·선택계획19·설치/거절·미선택·관측은별도로기록한다. 선택subset를고정된ID목록으로강제하지않으며summary실선택을검사한다. STOP ACK정본과전체완전journal·후속OFF sample불변을검수하고seq순으로raw를재정렬하지않는다. alias soft오류와hardIO/drop/lost·의도stride/budget제외는구별한다.

matrix actual_runtime_samples/runtime_samples_received는 runtime origin의 고정raw줄수다. 실제독립비교수는actual_runtime_comparisons_rechecked,표본분모가있는지표행은metric_rows_with_function_sample_denominator,실제평가행은metric_entries_evaluated다. fixture origin의실측은0이고 loaded상수/DLL false는이도구가입증하지않은별도제약이다. 계수/직접분기전량완료로승격하지않는다.

필수사본20개는copy_manifest.json에있다. REPORT절대source_pin은그대로보존하고root가M의동일bytes를검증한뒤별도M용경로핀만발급한다. analyzer는자기옆helper를검증하며matrix도동일bundle의명세/분석기를쓴다. 준비시험/fixture는런타임실행의존이아니다.

```powershell
python -B -X utf8 analyze_logs_wave11.py --summary <STOP.json> --observations <raw.jsonl> --journal <summaries_RUN.jsonl> --expected-run-id <RUN> --expected-stop-record <ACK> --input-origin runtime --output-dir <새분석폴더>
python -B -X utf8 runtime_metric_matrix_wave11/build_matrix.py --summary <STOP.json> --observations <raw.jsonl> --analysis-json <분석/analysis.json> --expected-run-id <RUN> --input-origin runtime --source-commit <commit> --dll-sha256 <sha> --output-dir <새matrix폴더>
```

M/GUI/공유원장쓰기·컴파일·native·게임실행·실표본·함수추가0이다. 설치선택과실게임결과는root의별도근거다.
