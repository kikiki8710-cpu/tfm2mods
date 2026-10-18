# typed 47모델·124명명지표 독립후처리 준비

기존44식은새3route·arity분기·상한guard를제외하면AST동일하고helper11파일은byte동일하다. 새helper는A동결models_stacked09.py만호출한다. old0..43은정확4args,44/45는정확5args,46은정확6args이며생략/추가/zero-padding을허용하지않는다. 잘못된arity는SKIP에도형식오류다.

기존89+44확장43+새typed44시험과matrix40시험PASS다. 신규모델정상418/guard19는기존고정raw/Pythonfixture의후처리이며새rawVM/native/실게임/함수완료로중복계상하지않는다. 준비47/이번root선택13·미선택34를구별하며거절/未선택/설치성공·표본관측은각각다르다.

44FE0160은Data5를소비하고AL만비교한다.45F98E60은Context5와CL/R8d의정확절삭·fullRAXselector,46E3FA20은radius5/HPthreshold6와fullRAXcount를비교한다. 물리argv5/6저장은원문이E3HP6→radius5를읽는순서의동적증명이아니다. 새3은readonlyscalar이므로outputfield와입력안정성예외0이다. 원본panic·입력alias·coherent동시성/수명·EH·privateLTO·foreignprovider는미확정이다.

```powershell
python -B -X utf8 analyze_logs_wave09.py --summary <STOP.json> --observations <fixed.jsonl> --journal <summaries_RUN.jsonl> --expected-run-id <RUN> --expected-stop-record <ACK_RECORD> --input-origin runtime --output-dir <새분석폴더>
```

journal의전체완전줄/선택STOP정본/연속seq/run/drain·후속OFF표본불변을확인하고줄을자르지않는다. 이름이summaries_run.jsonl로복사됐으면동일bytes를summary_journal선언원래basename의독립input사본으로보존할수있다. journal미입력은runtime freshness미확정이며matrix승격이거절된다. 함수call수와stride/cap/gate제외를큐/IO유실로합산하지않는다.

```powershell
python -B -X utf8 runtime_metric_matrix_wave09/build_matrix.py --summary <STOP.json> --observations <fixed.jsonl> --analysis-json <analysis.json> --expected-run-id <RUN> --input-origin runtime --output-dir <R9/analyzer_wave09_runtime_matrix/새결과>
```

새matrix CLI는기존도구하위뿐아니라명시REPORT sibling analyzer_wave09_runtime_matrix 새폴더도허용하며기존결과덮어쓰기는거절한다. 새MD제목을47/124로정밀화했고이전111행·집계식은동일하다. 124중13개새명명지표는새8수치군과다른집합이며원본내부분기trace는0/계수완료추가0이다.

source_pin_manifest.json은이REPORTbundle의절대경로키다. M/tools에그대로복사하면matrix는M경로키를못찾아거절한다. REPORTbundle에서실행하거나root가원본manifest를보존하고SHA동일성확인후별도M용경로재결박manifest/복사영수증을만들어야한다. 독립analyzer의helper검사는__file__옆실제파일을확인하므로M에서실행하면M사본을검증한다. 동일bundle에서analyzer와matrix를실행해야source_files절대경로가일치한다.

이번M/GUI/빌드/배포/게임쓰기·실행0이다. 실제root47소스/native/설치/실게임결과와이후처리준비를분리한다.
