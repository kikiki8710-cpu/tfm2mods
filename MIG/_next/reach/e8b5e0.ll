 30237| define void @ai::plan_legacy8sub_plan12serpen_checkNtB2_18SerpenCheckSubPlan17action_candidates(ptr sret([32 x i8]) %0, ptr %1, i64 %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %7, ptr %8) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 30238|  %10 = alloca [32 x i8],
 30239|  %11 = alloca [24 x i8],
 30240|  %12 = alloca [32 x i8],
 30241|  %13 = alloca [32 x i8],
 30242|  %14 = alloca [136 x i8],
 30243|  %15 = alloca [184 x i8],
 30250|  %16 = alloca [184 x i8],
 30251|  %17 = alloca [184 x i8],
 30252|  %18 = alloca [184 x i8],
 30253|  %19 = alloca [184 x i8],
 30254|  %20 = alloca [184 x i8],
 30255|  %21 = alloca [184 x i8],
 30256|  %22 = alloca [152 x i8],
 30257|  %23 = alloca [184 x i8],
 30258|  %24 = alloca [16 x i8],
 30259|  %25 = alloca [24 x i8],
 30260|  %26 = alloca [24 x i8],
 30261|  %27 = alloca [184 x i8],
 30262|  %28 = alloca [184 x i8],
 30263|  %29 = alloca [136 x i8],
 30264|  %30 = alloca [184 x i8],
 30265|  %31 = alloca [88 x i8],
 30266|  %32 = alloca [88 x i8],
 30267|  %33 = alloca [136 x i8],
 30268|  %34 = alloca [184 x i8],
 30269|  %35 = alloca [56 x i8],
 30274|  %36 = alloca [32 x i8],
 30278|     ;; version = i64 %2
 30279|     ;; self = ptr %1
 30280|     ;; rnd = ptr %3
 30281|     ;; player = ptr %4
 30282|     ;; data = ptr %5
 30283|     ;; parameter = ptr %6
 30284|     ;; team_plan = ptr %7
 30285|     ;; debug = ptr %8
 30286|     ;; res = ptr %36
 30287|     ;; position_score = ptr %35
 30288|     ;; posture = ptr %32
 30289|     ;; posture = ptr %31
 30290|     ;; value = ptr %25
 30291|     ;; raw = ptr %11
 30295|  %37 = gep %5, i64 8                                                                                                   ;L15
 30296|  %38 = load ptr, ptr %37, , !!8, !!8                                                                                   ;L15
 30297|     ;; context = ptr %38
 30298|     ;; context = ptr %38
 30299|     ;; context = ptr %38
 30300|  %39 = load ptr, ptr %38, , !!8, !!8                                                                                   ;L15
 30301|     ;; bump = ptr %39
 30302|  store ptr inttoptr (i64 8 to ptr), ptr %36,                                                                           ;L547<15
 30303|  %40 = gep %36, i64 8                                                                                                  ;L547<15
 30304|  store ptr %39, ptr %40,                                                                                               ;L547<15
 30305|  %41 = gep %36, i64 16                                                                                                 ;L547<15
 30306|  %42 = gep %36, i64 24                                                                                                 ;L547<15
 30307|  %43 = gep %4, i64 2352                                                                                                ;L17
 30308|  call void @llvm.memset.p0.i64(ptr %41, i8 0, i64 16, i1 false)                                                        ;L547<15
 30309|  %44 = load i64, ptr %43, , !!8                                                                                        ;L17
 30310|     ;; team = i64 %44
 30311|  %45 = icmp ult i64 %44, 2                                                                                             ;L17
 30312|  br i1 %45, label %50, label %46                                                                                       ;L17
 30313| 
 30314| 46: ; preds = %9
 30315|  invoke void @core::panicking18panic_bounds_check(i64 %44, i64 2, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.141) #35
 30316|  to label %49 unwind label %47                                                                                         ;L17
 30317| 
 30318| 47: ; preds = %472, %458, %453, %452, %451, %443, %434, %428, %390, %376, %366, %365, %363, %339, %319, %310, %289, %287, %277, %276, %275, %273, %260, %252, %241, %227, %218, %213, %206, %195, %174, %170, %169, %153, %151, %141, %129, %117, %110, %80, %63, %46
 30319|  %48 = cleanuppad within none []
 30320|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %36) #34 [ "funclet"(token %48) ] ;L104
 30321|  cleanupret from %48 unwind to caller                                                                                  ;L14
 30322| 
 30323| 49: ; preds = %63, %46
 30324|  unreachable
 30325| 
 30326| 50: ; preds = %9
 30327|     ;; self = ptr %4
 30328|  %51 = gep %4, i64 2496                                                                                                ;L581<17
 30329|  %52 = load i32, ptr %51, , !!8                                                                                        ;L581<17
 30330|  %53 = zext nneg i32 %52 to i64                                                                                        ;L581<17
 30331|  %54 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L17
 30332|     ;; self = ptr %54
 30333|     ;; self = ptr %54
 30334|  %55 = gep %54, i64 480                                                                                                ;L17
 30335|  %56 = getelementptr [5 x ptr], ptr %55, i64 %44                                                                       ;L17
 30336|  %57 = getelementptr ptr, ptr %56, i64 %53                                                                             ;L17
 30337|  %58 = load ptr, ptr %57, , !!8                                                                                        ;L17
 30338|     ;; self = ptr %58
 30339|     ;; self = ptr %58
 30340|  %59 = icmp eq ptr %58, null                                                                                           ;L1011<17
 30341|  br i1 %59, label %63, label %60                                                                                       ;L1011<17
 30342| 
 30343| 60: ; preds = %50
 30344|     ;; champ = ptr %58
 30345|     ;; champ = ptr %58
 30346|  %61 = load i8, ptr %1, , !!8                                                                                          ;L19
 30347|  %62 = trunc nuw i8 %61 to i1                                                                                          ;L19
 30348|  br i1 %62, label %100, label %64                                                                                      ;L19
 30349| 
 30350| 63: ; preds = %50
 30351|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.142) #35
 30352|  to label %49 unwind label %47                                                                                         ;L1013<17
 30353| 
 30354| 64: ; preds = %60
 30355|  %65 = gep %58, i64 1632                                                                                               ;L20
 30356|  %66 = load i64, ptr %65, , !!8                                                                                        ;L20
 30357|     ;; x = i64 %66
 30358|     ;; x = i64 %66
 30359|     ;; x = i64 %66
 30360|     ;; x1 = i64 %66
 30361|     ;; self = i64 %66
 30362|  %67 = gep %58, i64 1640                                                                                               ;L20
 30363|  %68 = load i64, ptr %67, , !!8                                                                                        ;L20
 30364|     ;; y = i64 %68
 30365|     ;; y = i64 %68
 30366|     ;; y = i64 %68
 30367|     ;; y1 = i64 %68
 30368|     ;; self = i64 %68
 30369|  %69 = icmp eq i64 %44, 0                                                                                              ;L58<20
 30370|  %70 = gep %38, i64 8                                                                                                  ;L7<0<20
 30371|  %71 = load ptr, ptr %70, , !!8, !!8                                                                                   ;L7<0<20
 30372|  %72 = gep %71, i64 4800                                                                                               ;L7<0<20
 30373|  %73 = load i64, ptr %72, , !!8                                                                                        ;L7<0<20
 30374|  %74 = sub i64 %66, %68                                                                                                ;L7<0<20
 30375|  %75 = add i64 %74, %73                                                                                                ;L8<0<20
 30376|  %76 = gep %71, i64 4792                                                                                               ;L8<0<20
 30377|  %77 = load i64, ptr %76, , !!8                                                                                        ;L8<0<20
 30378|  %78 = icmp ugt i64 %75, %77                                                                                           ;L8<0<20
 30379|  %79 = xor i1 %69, %78                                                                                                 ;L58<20
 30380|  br i1 %79, label %80, label %99                                                                                       ;L58<20
 30381| 
 30382| 80: ; preds = %64
 30383|  %81 = gep %38, i64 32                                                                                                 ;L23
 30384|  %82 = load ptr, ptr %81, , !!8, !!8                                                                                   ;L23
 30385|  %83 = invoke { i64, i64 } @gc::simulation7map_defNtB2_6MapDef8camp_pos(ptr %82, i8 0, i1 zeroext %69)
 30386|  to label %84 unwind label %47                                                                                         ;L23
 30387| 
 30388| 84: ; preds = %80
 30389|  %85 = extractvalue { i64, i64 } %83, 0                                                                                ;L23
 30390|  %86 = extractvalue { i64, i64 } %83, 1                                                                                ;L23
 30391|     ;; camp[0..+8] = i64 %85
 30392|     ;; camp[8..+8] = i64 %86
 30393|     ;; x2 = i64 %85
 30394|     ;; other = i64 %85
 30395|     ;; y2 = i64 %86
 30396|     ;; other = i64 %86
 30397|  %87 = icmp ult i64 %66, %85                                                                                           ;L3147<7<25
 30398|  %88 = sub nuw i64 %85, %66                                                                                            ;L3147<7<25
 30399|  %89 = sub nuw i64 %66, %85                                                                                            ;L3147<7<25
 30400|  %90 = select i1 %87, i64 %88, i64 %89                                                                                 ;L3147<7<25
 30401|     ;; dx = i64 %90
 30402|  %91 = icmp ult i64 %68, %86                                                                                           ;L3147<8<25
 30403|  %92 = sub nuw i64 %86, %68                                                                                            ;L3147<8<25
 30404|  %93 = sub nuw i64 %68, %86                                                                                            ;L3147<8<25
 30405|  %94 = select i1 %91, i64 %92, i64 %93                                                                                 ;L3147<8<25
 30406|     ;; dy = i64 %94
 30407|  %95 = mul i64 %90, %90                                                                                                ;L9<25
 30408|  %96 = mul i64 %94, %94                                                                                                ;L9<25
 30409|  %97 = add i64 %95, %96                                                                                                ;L9<25
 30410|  %98 = icmp ult i64 %97, 4900000001                                                                                    ;L25
 30411|  br i1 %98, label %99, label %100                                                                                      ;L25
 30412| 
 30413| 99: ; preds = %84, %64
 30414|  store i8 1, ptr %1,                                                                                                   ;L0
 30415|  br label %100                                                                                                         ;L35
 30416| 
 30417| 100: ; preds = %99, %84, %60
 30418|  %101 = phi i1 [ true, %60 ], [ false, %84 ], [ true, %99 ]
 30419|     ;; self = ptr %4
 30420|  %102 = sub nuw nsw i64 1, %44                                                                                         ;L35
 30421|     ;; team = i64 %102
 30422|     ;; team = i64 %102
 30423|  %103 = getelementptr [5 x ptr], ptr %55, i64 %102                                                                     ;L1905<35
 30424|     ;; self = ptr undef
 30425|     ;; self = ptr undef
 30426|     ;; f[0..+8] = ptr undef
 30427|     ;; f[8..+8] = ptr %4
 30428|     ;; f[16..+8] = ptr %5
 30429|     ;; f[24..+8] = ptr %58
 30430|     ;; fold[0..+8] = ptr undef
 30431|     ;; fold[8..+8] = ptr %4
 30432|     ;; fold[16..+8] = ptr %5
 30433|     ;; fold[24..+8] = ptr %58
 30436|     ;; f[8..+8] = ptr undef
 30437|     ;; f[16..+8] = ptr %4
 30438|     ;; f[24..+8] = ptr %5
 30439|     ;; f[32..+8] = ptr %58
 30440|     ;; self = ptr undef
 30443|     ;; self = ptr undef
 30444|     ;; count = i64 1
 30445|     ;; ptr = ptr %103
 30446|     ;; self = ptr %103
 30447|     ;; end_or_len = ptr %103
 30450|  br label %104                                                                                                         ;L180<2493<138<2897<35
 30451| 
 30452| 104: ; preds = %121, %100
 30453|  %105 = phi i64 [ 0, %100 ], [ %107, %121 ]
 30454|  %106 = gep %103, i64 %105                                                                                             ;L656<185<2493<138<2897<35
 30455|     ;; ptr = ptr %106
 30456|  %107 = add nuw nsw i64 %105, 8                                                                                        ;L656<185<2493<138<2897<35
 30457|     ;; x = ptr %106
 30458|  %108 = load ptr, ptr %106, , !!44330, !!8                                                                             ;L2494<138<2897<35
 30459|     ;; f = ptr undef
 30463|  %109 = icmp eq ptr %108, null                                                                                         ;L49<2494<138<2897<35
 30464|  br i1 %109, label %121, label %110                                                                                    ;L49<2494<138<2897<35
 30465| 
 30466| 110: ; preds = %104
 30467|     ;; x = ptr %108
 30470|     ;; x = ptr %108
 30475|     ;; c = ptr %108
 30476|     ;; self = ptr %108
 30477|     ;; self = ptr %108
 30478|     ;; self = ptr %108
 30479|     ;; self = ptr %108
 30480|     ;; self = ptr %108
 30481|  %111 = invoke zeroext i1 @ai::utils26nontarget_windup_perceived(i64 %2, ptr %4, ptr %5, ptr %108)
 30482|  to label %112 unwind label %47                                                                                        ;L36<2893<50<2494<138<2897<35
 30483| 
 30484| 112: ; preds = %110
 30485|  %113 = gep %108, i64 104
 30486|  %114 = load i64, ptr %113, , !!44388
 30487|  %115 = icmp eq i64 %114, 13
 30488|  %116 = select i1 %111, i1 %115, i1 false                                                                              ;L36<2893<50<2494<138<2897<35
 30489|  br i1 %116, label %123, label %121                                                                                    ;L36<2893<50<2494<138<2897<35
 30490| 
 30491| 117: ; preds = %143, %143, %133, %133, %131
 30492|  %118 = phi ptr [ %138, %133 ], [ %132, %131 ], [ %138, %133 ], [ %148, %143 ], [ %148, %143 ]
 30493|  %119 = invoke zeroext i1 @gc::simulation6effectNtB2_6Effect11is_in_range(ptr %118, ptr %108, ptr %58)
 30494|  to label %120 unwind label %47                                                                                        ;L0<2893<50<2494<138<2897<35
 30495| 
 30496| 120: ; preds = %117
 30497|  br i1 %119, label %153, label %121                                                                                    ;L2494<138<2897<35
 30498| 
 30499| 121: ; preds = %143, %133, %126, %123, %120, %112, %104
 30500|     ;; self = ptr undef
 30501|     ;; count = i64 1
 30502|     ;; ptr = !DIArgList(ptr %103, i64 %107)
 30503|     ;; self = !DIArgList(ptr %103, i64 %107)
 30504|     ;; end_or_len = ptr %103
 30507|  %122 = icmp eq i64 %107, 40                                                                                           ;L1714<180<2493<138<2897<35
 30508|  br i1 %122, label %153, label %104                                                                                    ;L180<2493<138<2897<35
 30509| 
 30510| 123: ; preds = %112
 30511|     ;; champ = ptr %108
 30512|  %124 = gep %108, i64 112                                                                                              ;L1572<37<2893<50<2494<138<2897<35
 30513|  %125 = load i64, ptr %124, , !!44388, !!8                                                                             ;L1572<37<2893<50<2494<138<2897<35
 30514|  switch i64 %125, label %121 [
 30515|  i64 4, label %126
 30516|  i64 5, label %133
 30517|  i64 6, label %143
 30518|  ]                                                                                                                     ;L37<2893<50<2494<138<2897<35
 30519| 
 30520| 126: ; preds = %123
 30521|     ;; self = ptr %108
 30522|  %127 = gep %108, i64 1272                                                                                             ;L742<37<2893<50<2494<138<2897<35
 30523|  %128 = load i32, ptr %127, , !!44388, !!8                                                                             ;L742<37<2893<50<2494<138<2897<35
 30524|  switch i32 %128, label %121 [
 30525|  i32 -1, label %129
 30526|  i32 1, label %131
 30527|  i32 2, label %131
 30528|  ]                                                                                                                     ;L742<37<2893<50<2494<138<2897<35
 30529| 
 30530| 129: ; preds = %126
 30531|     ;; self = ptr null
 30532|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.52) #35
 30533|  to label %130 unwind label %47                                                                                        ;L1013<37<2893<50<2494<138<2897<35
 30534| 
 30535| 130: ; preds = %129
 30536|  unreachable                                                                                                           ;L1013<37<2893<50<2494<138<2897<35
 30537| 
 30538| 131: ; preds = %126, %126
 30539|  %132 = gep %108, i64 1224                                                                                             ;L742<37<2893<50<2494<138<2897<35
 30540|     ;; self = ptr %108
 30541|     ;; self = ptr %132
 30542|  br label %117                                                                                                         ;L37<2893<50<2494<138<2897<35
 30543| 
 30544| 133: ; preds = %123
 30545|  %134 = gep %108, i64 1480                                                                                             ;L1693<39<2893<50<2494<138<2897<35
 30546|  %135 = load i64, ptr %134, , !!44388, !!8                                                                             ;L1693<39<2893<50<2494<138<2897<35
 30547|  %136 = icmp ugt i64 %135, 2                                                                                           ;L1693<39<2893<50<2494<138<2897<35
 30548|  %137 = gep %108, i64 1280                                                                                             ;L1693<39<2893<50<2494<138<2897<35
 30549|  %138 = select i1 %136, ptr %137, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.22                                        ;L1693<39<2893<50<2494<138<2897<35
 30550|     ;; self = ptr %138
 30551|  %139 = gep %138, i64 48                                                                                               ;L742<39<2893<50<2494<138<2897<35
 30552|  %140 = load i32, ptr %139, , !!44388, !!8                                                                             ;L742<39<2893<50<2494<138<2897<35
 30553|  switch i32 %140, label %121 [
 30554|  i32 -1, label %141
 30555|  i32 1, label %117
 30556|  i32 2, label %117
 30557|  ]                                                                                                                     ;L742<39<2893<50<2494<138<2897<35
 30558| 
 30559| 141: ; preds = %133
 30560|     ;; self = ptr null
 30561|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.53) #35
 30562|  to label %142 unwind label %47                                                                                        ;L1013<39<2893<50<2494<138<2897<35
 30563| 
 30564| 142: ; preds = %141
 30565|  unreachable                                                                                                           ;L1013<39<2893<50<2494<138<2897<35
 30566| 
 30567| 143: ; preds = %123
 30568|  %144 = gep %108, i64 1480                                                                                             ;L1701<41<2893<50<2494<138<2897<35
 30569|  %145 = load i64, ptr %144, , !!44388, !!8                                                                             ;L1701<41<2893<50<2494<138<2897<35
 30570|  %146 = icmp ugt i64 %145, 4                                                                                           ;L1701<41<2893<50<2494<138<2897<35
 30571|  %147 = gep %108, i64 1336                                                                                             ;L1701<41<2893<50<2494<138<2897<35
 30572|  %148 = select i1 %146, ptr %147, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.22                                        ;L1701<41<2893<50<2494<138<2897<35
 30573|     ;; self = ptr %148
 30574|  %149 = gep %148, i64 48                                                                                               ;L742<41<2893<50<2494<138<2897<35
 30575|  %150 = load i32, ptr %149, , !!44388, !!8                                                                             ;L742<41<2893<50<2494<138<2897<35
 30576|  switch i32 %150, label %121 [
 30577|  i32 -1, label %151
 30578|  i32 1, label %117
 30579|  i32 2, label %117
 30580|  ]                                                                                                                     ;L742<41<2893<50<2494<138<2897<35
 30581| 
 30582| 151: ; preds = %143
 30583|     ;; self = ptr null
 30584|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.54) #35
 30585|  to label %152 unwind label %47                                                                                        ;L1013<41<2893<50<2494<138<2897<35
 30586| 
 30587| 152: ; preds = %151
 30588|  unreachable                                                                                                           ;L1013<41<2893<50<2494<138<2897<35
 30589| 
 30590| 153: ; preds = %121, %120
 30591|  %154 = phi i1 [ false, %121 ], [ true, %120 ]                                                                         ;L1714<180<2493<138<2897<35
 30592|     ;; has_non_target_action_range = i1 %154
 30594|  %155 = gep %6, i64 2544                                                                                               ;L48
 30595|  %156 = gep %58, i64 1632                                                                                              ;L49
 30596|  %157 = load i64, ptr %156, , !!8                                                                                      ;L49
 30597|     ;; x1 = i64 %157
 30598|     ;; self = i64 %157
 30599|  %158 = gep %58, i64 1640                                                                                              ;L49
 30600|  %159 = load i64, ptr %158, , !!8                                                                                      ;L49
 30601|     ;; y1 = i64 %159
 30602|     ;; self = i64 %159
 30603|  invoke void @ai::position_eval26position_score_at_position(ptr sret([56 x i8]) %35, i64 %2, ptr %4, ptr %5, ptr %155, i64 %157, i64 %159, i8 11)
 30604|  to label %160 unwind label %47                                                                                        ;L48
 30605| 
 30606| 160: ; preds = %153
 30607|  %161 = gep %35, i64 48                                                                                                ;L50
 30608|  %162 = load i8, ptr %161, , !!8                                                                                       ;L50
 30609|  %163 = trunc nuw i8 %162 to i1                                                                                        ;L50
 30610|  %164 = gep %35, i64 49                                                                                                ;L50
 30611|  %165 = load i8, ptr %164,                                                                                             ;L50
 30612|  %166 = trunc nuw i8 %165 to i1                                                                                        ;L50
 30613|     ;; on_trajectory = i1 %166
 30614|  %167 = or i1 %154, %166
 30615|  %168 = select i1 %163, i1 true, i1 %167                                                                               ;L50
 30616|  br i1 %168, label %169, label %170                                                                                    ;L50
 30617| 
 30618| 169: ; preds = %160
 30621|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %33, ptr %5, ptr %4, i64 5, i1 zeroext true)
 30622|  to label %464 unwind label %47                                                                                        ;L52
 30623| 
 30624| 170: ; preds = %160
 30627|  invoke void @ai::plan_legacy9team_plan20objective_disciplineNtB4_8TeamPlan21v25_objective_posture(ptr sret([88 x i8]) %32, ptr %7, i64 %2, ptr %4, ptr %5, i8 5)
 30628|  to label %171 unwind label %47                                                                                        ;L57
 30629| 
 30630| 171: ; preds = %170
 30631|  %172 = load i64, ptr %32, , !!8                                                                                       ;L58
 30632|  %173 = icmp eq i64 %172, -1                                                                                           ;L58
 30633|  br i1 %173, label %174, label %179                                                                                    ;L58
 30634| 
 30635| 174: ; preds = %194, %171
 30636|  %175 = gep %38, i64 32                                                                                                ;L77
 30637|  %176 = load ptr, ptr %175, , !!8, !!8                                                                                 ;L77
 30638|  %177 = icmp eq i64 %44, 0                                                                                             ;L77
 30639|  %178 = invoke { i64, i64 } @gc::simulation7map_defNtB2_6MapDef8camp_pos(ptr %176, i8 5, i1 zeroext %177)
 30640|  to label %295 unwind label %47                                                                                        ;L77
 30641| 
 30642| 179: ; preds = %171
 30644|  call void @llvm.memcpy.p0.p0.i64(ptr %31, ptr %32, i64 88, i1 false)                                                  ;L58
 30645|  %180 = gep %31, i64 80                                                                                                ;L59
 30646|  %181 = load i8, ptr %180, , !!8                                                                                       ;L59
 30648|  %182 = icmp samesign ugt i8 %181, 2                                                                                   ;L186<59
 30649|  br i1 %182, label %183, label %189                                                                                    ;L59
 30650| 
 30651| 183: ; preds = %179
 30652|  %184 = icmp eq i8 %181, 4                                                                                             ;L60
 30653|  %185 = gep %31, i64 56                                                                                                ;L60
 30654|  %186 = load i64, ptr %185,                                                                                            ;L60
 30655|  %187 = icmp ne i64 %186, 0                                                                                            ;L60
 30656|  %188 = select i1 %184, i1 %187, i1 false                                                                              ;L60
 30657|  br i1 %188, label %218, label %213                                                                                    ;L60
 30658| 
 30659| 189: ; preds = %179
 30661|  %190 = icmp eq i8 %181, 2                                                                                             ;L190<70
 30662|  %191 = load i64, ptr %31,
 30663|  %192 = trunc nuw i64 %191 to i1
 30664|  %193 = select i1 %190, i1 %192, i1 false                                                                              ;L70
 30665|  br i1 %193, label %195, label %194                                                                                    ;L70
 30666| 
 30667| 194: ; preds = %208, %189
 30669|  br label %174                                                                                                         ;L58
 30670| 
 30671| 195: ; preds = %189
 30672|  %196 = gep %31, i64 8                                                                                                 ;L71
 30673|  %197 = load i64, ptr %196, , !!8                                                                                      ;L71
 30674|     ;; focus_enemy = i64 %197
 30677|  invoke void @ai::small_action5traceNtB2_16SmallActionTrace16new_attack_range(ptr sret([152 x i8]) %22, ptr %5, i64 %197, i64 5)
 30678|  to label %198 unwind label %47                                                                                        ;L72
 30679| 
 30680| 198: ; preds = %195
 30681|  call void @llvm.memcpy.p0.p0.i64(ptr %23, ptr %22, i64 152, i1 false)                                                 ;L72
 30682|  %199 = gep %23, i64 177                                                                                               ;L72
 30683|  store i8 14, ptr %199,                                                                                                ;L72
 30685|     ;; self = ptr %36
 30686|     ;; self = ptr %36
 30687|     ;; value = ptr %23
 30688|     ;; src = ptr %23
 30689|     ;; additional = i64 1
 30690|     ;; needed_extra_cap = i64 1
 30691|     ;; needed_extra_cap = i64 1
 30692|     ;; strategy = i8 1
 30693|  %200 = load i64, ptr %42, , !!44502, !!8                                                                              ;L1428<72
 30694|     ;; self = ptr %36
 30695|  %201 = load i64, ptr %41, , !!44502, !!8                                                                              ;L149<1428<72
 30696|  %202 = icmp eq i64 %200, %201                                                                                         ;L1428<72
 30697|  br i1 %202, label %203, label %208                                                                                    ;L1428<72
 30698| 
 30699| 203: ; preds = %198
 30700|     ;; self = ptr %36
 30701|     ;; self = ptr %36
 30702|     ;; self = ptr %36
 30703|     ;; used_cap = i64 %200
 30704|     ;; used_cap = i64 %200
 30705|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %36, i64 %200, i64 1, i1 zeroext true)
 30706|  to label %204 unwind label %206, !!44502                                                                              ;L619<430<738<1429<72
 30707| 
 30708| 204: ; preds = %203
 30709|  %205 = load i64, ptr %42, , !!44502                                                                                   ;L1432<72
 30710|  br label %208                                                                                                         ;L619<430<738<1429<72
 30711| 
 30712| 206: ; preds = %203
 30713|  %207 = cleanuppad within none []
 30714|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %23) #34 [ "funclet"(token %207) ] ;L1436<72
 30715|  cleanupret from %207 unwind label %47
 30716| 
 30717| 208: ; preds = %204, %198
 30718|  %209 = phi i64 [ %205, %204 ], [ %200, %198 ]                                                                         ;L1434<72
 30719|     ;; self = ptr %36
 30720|  %210 = load ptr, ptr %36, , !!44502, !!8, !!8                                                                         ;L138<1432<72
 30721|     ;; self = ptr %210
 30722|     ;; count = i64 %209
 30723|  %211 = gepS %210, i64 %209                                                                                            ;L961<1432<72
 30724|     ;; end = ptr %211
 30725|     ;; dst = ptr %211
 30726|  call void @llvm.memcpy.p0.p0.i64(ptr %211, ptr %23, i64 184, i1 false)                                                ;L1933<1433<72
 30727|  %212 = add i64 %209, 1                                                                                                ;L1434<72
 30728|  store i64 %212, ptr %42, , !!44502                                                                                    ;L1434<72
 30730|  br label %194                                                                                                         ;L71
 30731| 
 30732| 213: ; preds = %229, %183
 30735|  %214 = gep %31, i64 32                                                                                                ;L63
 30736|  %215 = load i64, ptr %214, , !!8                                                                                      ;L63
 30737|  %216 = gep %31, i64 40                                                                                                ;L63
 30738|  %217 = load i64, ptr %216, , !!8                                                                                      ;L63
 30739|  invoke void @ai::small_action6aroundNtB5_25SmallActionAroundPosition3new(ptr sret([184 x i8]) %27, ptr %3, ptr %5, i64 %215, i64 %217, i64 5)
 30740|  to label %234 unwind label %47                                                                                        ;L63
 30741| 
 30742| 218: ; preds = %183
 30745|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %29, ptr %5, ptr %4, i64 5, i1 zeroext false)
 30746|  to label %219 unwind label %47                                                                                        ;L61
 30747| 
 30748| 219: ; preds = %218
 30749|  call void @llvm.memcpy.p0.p0.i64(ptr %30, ptr %29, i64 136, i1 false)                                                 ;L61
 30750|  %220 = gep %30, i64 177                                                                                               ;L61
 30751|  store i8 3, ptr %220,                                                                                                 ;L61
 30753|     ;; self = ptr %36
 30754|     ;; self = ptr %36
 30755|     ;; value = ptr %30
 30756|     ;; src = ptr %30
 30757|     ;; additional = i64 1
 30758|     ;; needed_extra_cap = i64 1
 30759|     ;; needed_extra_cap = i64 1
 30760|     ;; strategy = i8 1
 30761|  %221 = load i64, ptr %42, , !!44539, !!8                                                                              ;L1428<61
 30762|     ;; self = ptr %36
 30763|  %222 = load i64, ptr %41, , !!44539, !!8                                                                              ;L149<1428<61
 30764|  %223 = icmp eq i64 %221, %222                                                                                         ;L1428<61
 30765|  br i1 %223, label %224, label %229                                                                                    ;L1428<61
 30766| 
 30767| 224: ; preds = %219
 30768|     ;; self = ptr %36
 30769|     ;; self = ptr %36
 30770|     ;; self = ptr %36
 30771|     ;; used_cap = i64 %221
 30772|     ;; used_cap = i64 %221
 30773|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %36, i64 %221, i64 1, i1 zeroext true)
 30774|  to label %225 unwind label %227, !!44539                                                                              ;L619<430<738<1429<61
 30775| 
 30776| 225: ; preds = %224
 30777|  %226 = load i64, ptr %42, , !!44539                                                                                   ;L1432<61
 30778|  br label %229                                                                                                         ;L619<430<738<1429<61
 30779| 
 30780| 227: ; preds = %224
 30781|  %228 = cleanuppad within none []
 30782|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %30) #34 [ "funclet"(token %228) ] ;L1436<61
 30783|  cleanupret from %228 unwind label %47
 30784| 
 30785| 229: ; preds = %225, %219
 30786|  %230 = phi i64 [ %226, %225 ], [ %221, %219 ]                                                                         ;L1434<61
 30787|     ;; self = ptr %36
 30788|  %231 = load ptr, ptr %36, , !!44539, !!8, !!8                                                                         ;L138<1432<61
 30789|     ;; self = ptr %231
 30790|     ;; count = i64 %230
 30791|  %232 = gepS %231, i64 %230                                                                                            ;L961<1432<61
 30792|     ;; end = ptr %232
 30793|     ;; dst = ptr %232
 30794|  call void @llvm.memcpy.p0.p0.i64(ptr %232, ptr %30, i64 184, i1 false)                                                ;L1933<1433<61
 30795|  %233 = add i64 %230, 1                                                                                                ;L1434<61
 30796|  store i64 %233, ptr %42, , !!44539                                                                                    ;L1434<61
 30798|  br label %213                                                                                                         ;L60
 30799| 
 30800| 234: ; preds = %213
 30801|  call void @llvm.memcpy.p0.p0.i64(ptr %28, ptr %27, i64 184, i1 false)                                                 ;L63
 30803|     ;; self = ptr %36
 30804|     ;; self = ptr %36
 30805|     ;; value = ptr %28
 30806|     ;; src = ptr %28
 30807|     ;; additional = i64 1
 30808|     ;; needed_extra_cap = i64 1
 30809|     ;; needed_extra_cap = i64 1
 30810|     ;; strategy = i8 1
 30811|  %235 = load i64, ptr %42, , !!44573, !!8                                                                              ;L1428<63
 30812|     ;; self = ptr %36
 30813|  %236 = load i64, ptr %41, , !!44573, !!8                                                                              ;L149<1428<63
 30814|  %237 = icmp eq i64 %235, %236                                                                                         ;L1428<63
 30815|  br i1 %237, label %238, label %243                                                                                    ;L1428<63
 30816| 
 30817| 238: ; preds = %234
 30818|     ;; self = ptr %36
 30819|     ;; self = ptr %36
 30820|     ;; self = ptr %36
 30821|     ;; used_cap = i64 %235
 30822|     ;; used_cap = i64 %235
 30823|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %36, i64 %235, i64 1, i1 zeroext true)
 30824|  to label %239 unwind label %241, !!44573                                                                              ;L619<430<738<1429<63
 30825| 
 30826| 239: ; preds = %238
 30827|  %240 = load i64, ptr %42, , !!44573                                                                                   ;L1432<63
 30828|  br label %243                                                                                                         ;L619<430<738<1429<63
 30829| 
 30830| 241: ; preds = %238
 30831|  %242 = cleanuppad within none []
 30832|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %28) #34 [ "funclet"(token %242) ] ;L1436<63
 30833|  cleanupret from %242 unwind label %47
 30834| 
 30835| 243: ; preds = %239, %234
 30836|  %244 = phi i64 [ %240, %239 ], [ %235, %234 ]                                                                         ;L1434<63
 30837|     ;; self = ptr %36
 30838|  %245 = load ptr, ptr %36, , !!44573, !!8, !!8                                                                         ;L138<1432<63
 30839|     ;; self = ptr %245
 30840|     ;; count = i64 %244
 30841|  %246 = gepS %245, i64 %244                                                                                            ;L961<1432<63
 30842|     ;; end = ptr %246
 30843|     ;; dst = ptr %246
 30844|  call void @llvm.memcpy.p0.p0.i64(ptr %246, ptr %28, i64 184, i1 false)                                                ;L1933<1433<63
 30845|  %247 = add i64 %244, 1                                                                                                ;L1434<63
 30846|  store i64 %247, ptr %42, , !!44573                                                                                    ;L1434<63
 30848|  %248 = gep %38, i64 59                                                                                                ;L64
 30849|  %249 = load i8, ptr %248, , !!8                                                                                       ;L64
 30850|  %250 = trunc nuw i8 %249 to i1                                                                                        ;L64
 30851|  br i1 %250, label %252, label %251                                                                                    ;L64
 30852| 
 30853| 251: ; preds = %290, %243
 30854|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %36, i64 32, i1 false)                                                   ;L67
 30857|  br label %463                                                                                                         ;L1
 30858| 
 30859| 252: ; preds = %243
 30860|     ;; self = ptr %8
 30861|  %253 = gep %58, i64 1472                                                                                              ;L65
 30862|  %254 = load i64, ptr %253, , !!8                                                                                      ;L65
 30863|     ;; key = i64 %254
 30865|  %255 = gep %8, i64 160                                                                                                ;L1014<65
 30866|  invoke void @_RNvMNtCs5gUUnHMsxBL_9hashbrown11rustc_entryINtNtB4_3map7HashMapjINtNtCs9LexZzt9XJB_5alloc3vec3VecNtNtB15_6string6StringENtNtCs9EYcZKFYzm_5ahash12random_state11RandomStateE11rustc_entryCshdEBA0ozCnw_7game_ai(ptr sret([24 x i8]) %11, ptr %255, i64 %254)
 30867|  to label %256 unwind label %47                                                                                        ;L1014<65
 30868| 
 30869| 256: ; preds = %252
 30870|  %257 = load ptr, ptr %11, , !!8                                                                                       ;L3008<1014<65
 30871|  %258 = icmp eq ptr %257, null                                                                                         ;L3008<1014<65
 30872|  %259 = gep %11, i64 8                                                                                                 ;L0<1014<65
 30873|  br i1 %258, label %269, label %260                                                                                    ;L3008<1014<65
 30874| 
 30875| 260: ; preds = %256
 30876|  %261 = load i64, ptr %259,                                                                                            ;L3010<1014<65
 30877|  %262 = gep %11, i64 16                                                                                                ;L3010<1014<65
 30878|  %263 = load i64, ptr %262,                                                                                            ;L3010<1014<65
 30879|     ;; self[0..+8] = ptr %257
 30880|     ;; self[8..+8] = i64 %261
 30881|     ;; self[16..+8] = i64 %263
 30884|  store i64 0, ptr %26,                                                                                                 ;L464<65
 30885|  %264 = gep %26, i64 8                                                                                                 ;L464<65
 30886|  store ptr inttoptr (i64 8 to ptr), ptr %264,                                                                          ;L464<65
 30887|  %265 = gep %26, i64 16                                                                                                ;L464<65
 30888|  store i64 0, ptr %265,                                                                                                ;L464<65
 30889|     ;; default = ptr %26
 30892|     ;; entry[8..+8] = i64 %261
 30893|     ;; self[8..+8] = i64 %261
 30894|     ;; self[8..+8] = i64 %261
 30895|     ;; entry[16..+8] = i64 %263
 30896|     ;; self[16..+8] = i64 %263
 30897|     ;; self[16..+8] = i64 %263
 30898|     ;; entry[0..+8] = ptr %257
 30899|     ;; self[0..+8] = ptr %257
 30900|     ;; self[0..+8] = ptr %257
 30901|  %266 = gep %10, i64 8                                                                                                 ;L576<2911<2519<65
 30903|  call void @llvm.memcpy.p0.p0.i64(ptr %266, ptr %26, i64 24, i1 false), !!44620                                        ;L2519<65
 30904|  store i64 %263, ptr %10, , !!44615                                                                                    ;L576<2911<2519<65
 30905|  %267 = invoke ptr @_RNvMs6_NtCs5gUUnHMsxBL_9hashbrown3rawINtB5_8RawTableTjINtNtCs9LexZzt9XJB_5alloc3vec3VecNtNtBV_6string6StringEEE14insert_no_growCshdEBA0ozCnw_7game_ai(ptr %257, i64 %261, ptr %10)
 30906|  to label %268 unwind label %47                                                                                        ;L576<2911<2519<65
 30907| 
 30908| 268: ; preds = %260
 30910|  br label %277                                                                                                         ;L2521<65
 30911| 
 30912| 269: ; preds = %256
 30913|  %270 = load ptr, ptr %259, , !!8, !!8                                                                                 ;L3009<1014<65
 30914|     ;; self[0..+8] = ptr null
 30915|     ;; self[8..+8] = ptr %270
 30919|  store i64 0, ptr %26,                                                                                                 ;L464<65
 30920|  %271 = gep %26, i64 8                                                                                                 ;L464<65
 30921|  store ptr inttoptr (i64 8 to ptr), ptr %271,                                                                          ;L464<65
 30922|  %272 = gep %26, i64 16                                                                                                ;L464<65
 30923|  store i64 0, ptr %272,                                                                                                ;L464<65
 30924|     ;; default = ptr %26
 30928|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %26)
 30929|  to label %276 unwind label %273, !!44620                                                                              ;L825<2521<65
 30930| 
 30931| 273: ; preds = %269
 30932|  %274 = cleanuppad within none []
 30934|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %26) [ "funclet"(token %274) ]
 30935|  to label %275 unwind label %47                                                                                        ;L825<825<2521<65
 30936| 
 30937| 275: ; preds = %273
 30938|  cleanupret from %274 unwind label %47
 30939| 
 30940| 276: ; preds = %269
 30942|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %26)
 30943|  to label %277 unwind label %47                                                                                        ;L825<825<2521<65
 30944| 
 30945| 277: ; preds = %276, %268
 30946|  %278 = phi ptr [ %267, %268 ], [ %270, %276 ]
 30947|  %279 = gep %278, i64 -24                                                                                              ;L0<65
 30948|     ;; self = ptr %279
 30950|     ;; args = ptr %180
 30952|  store ptr %180, ptr %24,                                                                                              ;L65
 30953|  %280 = gep %24, i64 8                                                                                                 ;L65
 30954|  store ptr @ai::plan_legacy9team_planNtB5_20ObjectivePostureKindNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %280,       ;L65
 30955|     ;; args[0..+8] = ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.140
 30956|     ;; args[8..+8] = ptr %24
 30957|     ;; self[0..+8] = ptr null
 30958|     ;; self[8..+8] = i64 undef
 30962|  invoke void @_RNvNvNtCs9LexZzt9XJB_5alloc3fmt6format12format_inner(ptr sret([24 x i8]) %25, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.140, ptr %24)
 30963|  to label %281 unwind label %47                                                                                        ;L659<1275<659<65
 30964| 
 30965| 281: ; preds = %277
 30967|     ;; self = ptr %279
 30968|     ;; self = ptr %279
 30969|     ;; value = ptr %25
 30971|     ;; elem_size = i64 24
 30972|  %282 = gep %278, i64 -8                                                                                               ;L1037<1004<65
 30973|  %283 = load i64, ptr %282, , !!44661, !!8                                                                             ;L1037<1004<65
 30974|     ;; len = i64 %283
 30975|     ;; count = i64 %283
 30976|     ;; self = ptr %279
 30977|  %284 = load i64, ptr %279, , !!44661, !!8                                                                             ;L619<309<1040<1004<65
 30978|  %285 = icmp eq i64 %283, %284                                                                                         ;L1040<1004<65
 30979|  br i1 %285, label %286, label %290                                                                                    ;L1040<1004<65
 30980| 
 30981| 286: ; preds = %281
 30982|  invoke void @_RNvMs3_NtCs9LexZzt9XJB_5alloc7raw_vecINtB5_6RawVecNtNtB7_6string6StringE8grow_oneCszutcqs0z2F_10sys_locale(ptr %279)
 30983|  to label %290 unwind label %287, !!44661                                                                              ;L1041<1004<65
 30984| 
 30985| 287: ; preds = %286
 30986|  %288 = cleanuppad within none []
 30987|  invoke void @core::ptr9drop_glueNtNtCs9LexZzt9XJB_5alloc6string6StringECshdEBA0ozCnw_7game_ai(ptr %25) #34 [ "funclet"(token %288) ]
 30988|  to label %289 unwind label %47                                                                                        ;L1050<1004<65
 30989| 
 30990| 289: ; preds = %287
 30991|  cleanupret from %288 unwind label %47
 30992| 
 30993| 290: ; preds = %286, %281
 30994|  %291 = gep %278, i64 -16                                                                                              ;L614<609<296<2052<1044<1004<65
 30995|  %292 = load ptr, ptr %291, , !!44661, !!8, !!8                                                                        ;L614<609<296<2052<1044<1004<65
 30996|     ;; self = ptr %292
 30997|  %293 = getelementptr { { { { i64, ptr, {} }, {} }, i64 } }, ptr %292, i64 %283                                        ;L961<1044<1004<65
 30998|     ;; end = ptr %293
 30999|     ;; dst = ptr %293
 31000|  call void @llvm.memcpy.p0.p0.i64(ptr %293, ptr %25, i64 24, i1 false)                                                 ;L1933<1045<1004<65
 31001|  %294 = add i64 %283, 1                                                                                                ;L1046<1004<65
 31002|  store i64 %294, ptr %282, , !!44661                                                                                   ;L1046<1004<65
 31003|  br label %251                                                                                                         ;L1050<1004<65
 31004| 
 31005| 295: ; preds = %174
 31006|  %296 = extractvalue { i64, i64 } %178, 0                                                                              ;L77
 31007|  %297 = extractvalue { i64, i64 } %178, 1                                                                              ;L77
 31008|     ;; camp[0..+8] = i64 %296
 31009|     ;; camp[8..+8] = i64 %297
 31010|     ;; x2 = i64 %296
 31011|     ;; other = i64 %296
 31012|     ;; y2 = i64 %297
 31013|     ;; other = i64 %297
 31014|  %298 = icmp ult i64 %157, %296                                                                                        ;L3147<7<78
 31015|  %299 = sub nuw i64 %296, %157                                                                                         ;L3147<7<78
 31016|  %300 = sub nuw i64 %157, %296                                                                                         ;L3147<7<78
 31017|  %301 = select i1 %298, i64 %299, i64 %300                                                                             ;L3147<7<78
 31018|     ;; dx = i64 %301
 31019|  %302 = icmp ult i64 %159, %297                                                                                        ;L3147<8<78
 31020|  %303 = sub nuw i64 %297, %159                                                                                         ;L3147<8<78
 31021|  %304 = sub nuw i64 %159, %297                                                                                         ;L3147<8<78
 31022|  %305 = select i1 %302, i64 %303, i64 %304                                                                             ;L3147<8<78
 31023|     ;; dy = i64 %305
 31024|  %306 = mul i64 %301, %301                                                                                             ;L9<78
 31025|  %307 = mul i64 %305, %305                                                                                             ;L9<78
 31026|  %308 = add i64 %306, %307                                                                                             ;L9<78
 31027|  %309 = icmp ugt i64 %308, 22500000000                                                                                 ;L78
 31028|  br i1 %309, label %311, label %310                                                                                    ;L78
 31029| 
 31030| 310: ; preds = %295
 31033|  invoke void @ai::small_action6aroundNtB5_25SmallActionAroundPosition3new(ptr sret([184 x i8]) %16, ptr %3, ptr %5, i64 %296, i64 %297, i64 5)
 31034|  to label %312 unwind label %47                                                                                        ;L87
 31035| 
 31036| 311: ; preds = %295
 31037|  br i1 %101, label %365, label %363                                                                                    ;L79
 31038| 
 31039| 312: ; preds = %310
 31040|  call void @llvm.memcpy.p0.p0.i64(ptr %17, ptr %16, i64 184, i1 false)                                                 ;L87
 31042|     ;; self = ptr %36
 31043|     ;; self = ptr %36
 31044|     ;; value = ptr %17
 31045|     ;; src = ptr %17
 31046|     ;; additional = i64 1
 31047|     ;; needed_extra_cap = i64 1
 31048|     ;; needed_extra_cap = i64 1
 31049|     ;; strategy = i8 1
 31050|  %313 = load i64, ptr %42, , !!44702, !!8                                                                              ;L1428<87
 31051|     ;; self = ptr %36
 31052|  %314 = load i64, ptr %41, , !!44702, !!8                                                                              ;L149<1428<87
 31053|  %315 = icmp eq i64 %313, %314                                                                                         ;L1428<87
 31054|  br i1 %315, label %316, label %321                                                                                    ;L1428<87
 31055| 
 31056| 316: ; preds = %312
 31057|     ;; self = ptr %36
 31058|     ;; self = ptr %36
 31059|     ;; self = ptr %36
 31060|     ;; used_cap = i64 %313
 31061|     ;; used_cap = i64 %313
 31062|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %36, i64 %313, i64 1, i1 zeroext true)
 31063|  to label %317 unwind label %319, !!44702                                                                              ;L619<430<738<1429<87
 31064| 
 31065| 317: ; preds = %316
 31066|  %318 = load i64, ptr %42, , !!44702                                                                                   ;L1432<87
 31067|  br label %321                                                                                                         ;L619<430<738<1429<87
 31068| 
 31069| 319: ; preds = %316
 31070|  %320 = cleanuppad within none []
 31071|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %17) #34 [ "funclet"(token %320) ] ;L1436<87
 31072|  cleanupret from %320 unwind label %47
 31073| 
 31074| 321: ; preds = %317, %312
 31075|  %322 = phi i64 [ %318, %317 ], [ %313, %312 ]                                                                         ;L1434<87
 31076|     ;; self = ptr %36
 31077|  %323 = load ptr, ptr %36, , !!44702, !!8, !!8                                                                         ;L138<1432<87
 31078|     ;; self = ptr %323
 31079|     ;; count = i64 %322
 31080|  %324 = gepS %323, i64 %322                                                                                            ;L961<1432<87
 31081|     ;; end = ptr %324
 31082|     ;; dst = ptr %324
 31083|  call void @llvm.memcpy.p0.p0.i64(ptr %324, ptr %17, i64 184, i1 false)                                                ;L1933<1433<87
 31084|  %325 = add i64 %322, 1                                                                                                ;L1434<87
 31085|  store i64 %325, ptr %42, , !!44702                                                                                    ;L1434<87
 31087|  br label %326                                                                                                         ;L78
 31088| 
 31089| 326: ; preds = %392, %378, %321
 31090|     ;; self = ptr undef
 31091|     ;; self = ptr undef
 31092|  %327 = load ptr, ptr %54, , !!8, !!8                                                                                  ;L90
 31093|  %328 = gep %54, i64 8                                                                                                 ;L90
 31094|  %329 = load ptr, ptr %328, , !!8, !!8                                                                                 ;L90
 31095|  %330 = gep %5, i64 16                                                                                                 ;L90
 31096|  %331 = load ptr, ptr %330, , !!8, !!8                                                                                 ;L90
 31097|     ;; f[0..+8] = ptr %327
 31098|     ;; f[8..+8] = ptr %329
 31099|     ;; f[16..+8] = ptr %331
 31100|     ;; f[24..+8] = ptr %4
 31101|     ;; f[32..+8] = ptr %58
 31102|     ;; fold[0..+8] = ptr %327
 31103|     ;; fold[8..+8] = ptr %329
 31104|     ;; fold[16..+8] = ptr %331
 31105|     ;; fold[24..+8] = ptr %4
 31106|     ;; fold[32..+8] = ptr %58
 31109|     ;; f[8..+8] = ptr %327
 31110|     ;; f[16..+8] = ptr %329
 31111|     ;; f[24..+8] = ptr %331
 31112|     ;; f[32..+8] = ptr %4
 31113|     ;; f[40..+8] = ptr %58
 31114|     ;; self = ptr undef
 31117|     ;; self = ptr undef
 31118|     ;; count = i64 1
 31119|     ;; ptr = ptr %103
 31120|     ;; self = ptr %103
 31121|     ;; end_or_len = ptr %103
 31124|  %332 = getelementptr { { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, [5 x { i64, [2 x i64] }], [5 x { i64, { i8, [23 x i8] } }], [5 x { i8, [7 x i8], i64 }], [5 x i64], [5 x i64], [5 x i64], [5 x { i64, i64 }], i64, i64, i64, i64, i64, [5 x i32], i8, [3 x i8] }, ptr %331, i64 %102
 31125|  br label %333                                                                                                         ;L180<2493<138<2897<90
 31126| 
 31127| 333: ; preds = %361, %326
 31128|  %334 = phi i64 [ 0, %326 ], [ %336, %361 ]
 31129|  %335 = gep %103, i64 %334                                                                                             ;L656<185<2493<138<2897<90
 31130|     ;; ptr = ptr %335
 31131|  %336 = add nuw nsw i64 %334, 8                                                                                        ;L656<185<2493<138<2897<90
 31132|     ;; x = ptr %335
 31133|  %337 = load ptr, ptr %335, , !!44750, !!8                                                                             ;L2494<138<2897<90
 31134|     ;; f = ptr undef
 31138|  %338 = icmp eq ptr %337, null                                                                                         ;L49<2494<138<2897<90
 31139|  br i1 %338, label %361, label %339                                                                                    ;L49<2494<138<2897<90
 31140| 
 31141| 339: ; preds = %333
 31142|     ;; x = ptr %337
 31145|     ;; x = ptr %337
 31149|     ;; c = ptr %337
 31150|     ;; self = ptr %337
 31151|  %340 = invoke zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %332, ptr %327, ptr %329, ptr %4, ptr %337)
 31152|  to label %341 unwind label %47                                                                                        ;L90<2893<50<2494<138<2897<90
 31153| 
 31154| 341: ; preds = %339
 31155|  br i1 %340, label %342, label %361                                                                                    ;L90<2893<50<2494<138<2897<90
 31156| 
 31157| 342: ; preds = %341
 31158|     ;; other = ptr %58
 31159|  %343 = gep %337, i64 1632                                                                                             ;L2158<91<2893<50<2494<138<2897<90
 31160|  %344 = load i64, ptr %343, , !!44794, !!8                                                                             ;L2158<91<2893<50<2494<138<2897<90
 31161|     ;; x1 = i64 %344
 31162|     ;; self = i64 %344
 31163|  %345 = gep %337, i64 1640                                                                                             ;L2158<91<2893<50<2494<138<2897<90
 31164|  %346 = load i64, ptr %345, , !!44794, !!8                                                                             ;L2158<91<2893<50<2494<138<2897<90
 31165|     ;; y1 = i64 %346
 31166|     ;; self = i64 %346
 31167|  %347 = load i64, ptr %156, , !!44794, !!8                                                                             ;L2158<91<2893<50<2494<138<2897<90
 31168|     ;; x2 = i64 %347
 31169|     ;; other = i64 %347
 31170|  %348 = load i64, ptr %158, , !!44794, !!8                                                                             ;L2158<91<2893<50<2494<138<2897<90
 31171|     ;; y2 = i64 %348
 31172|     ;; other = i64 %348
 31173|  %349 = icmp ult i64 %344, %347                                                                                        ;L3147<7<2158<91<2893<50<2494<138<2897<90
 31174|  %350 = sub nuw i64 %347, %344                                                                                         ;L3147<7<2158<91<2893<50<2494<138<2897<90
 31175|  %351 = sub nuw i64 %344, %347                                                                                         ;L3147<7<2158<91<2893<50<2494<138<2897<90
 31176|  %352 = select i1 %349, i64 %350, i64 %351                                                                             ;L3147<7<2158<91<2893<50<2494<138<2897<90
 31177|     ;; dx = i64 %352
 31178|  %353 = icmp ult i64 %346, %348                                                                                        ;L3147<8<2158<91<2893<50<2494<138<2897<90
 31179|  %354 = sub nuw i64 %348, %346                                                                                         ;L3147<8<2158<91<2893<50<2494<138<2897<90
 31180|  %355 = sub nuw i64 %346, %348                                                                                         ;L3147<8<2158<91<2893<50<2494<138<2897<90
 31181|  %356 = select i1 %353, i64 %354, i64 %355                                                                             ;L3147<8<2158<91<2893<50<2494<138<2897<90
 31182|     ;; dy = i64 %356
 31183|  %357 = mul i64 %352, %352                                                                                             ;L9<2158<91<2893<50<2494<138<2897<90
 31184|  %358 = mul i64 %356, %356                                                                                             ;L9<2158<91<2893<50<2494<138<2897<90
 31185|  %359 = add i64 %358, %357                                                                                             ;L9<2158<91<2893<50<2494<138<2897<90
 31186|  %360 = icmp ult i64 %359, 22500000001                                                                                 ;L91<2893<50<2494<138<2897<90
 31187|  br i1 %360, label %434, label %361                                                                                    ;L2494<138<2897<90
 31188| 
 31189| 361: ; preds = %342, %341, %333
 31190|     ;; self = ptr undef
 31191|     ;; count = i64 1
 31192|     ;; ptr = !DIArgList(ptr %103, i64 %336)
 31193|     ;; self = !DIArgList(ptr %103, i64 %336)
 31194|     ;; end_or_len = ptr %103
 31197|  %362 = icmp eq i64 %336, 40                                                                                           ;L1714<180<2493<138<2897<90
 31198|  br i1 %362, label %397, label %333                                                                                    ;L180<2493<138<2897<90
 31199| 
 31200| 363: ; preds = %311
 31201|  %364 = invoke { i64, i64 } @gc::simulation7map_defNtB2_6MapDef8camp_pos(ptr %176, i8 0, i1 zeroext %177)
 31202|  to label %366 unwind label %47                                                                                        ;L83
 31203| 
 31204| 365: ; preds = %311
 31207|  invoke void @ai::small_action6aroundNtB5_25SmallActionAroundPosition3new(ptr sret([184 x i8]) %20, ptr %3, ptr %5, i64 %296, i64 %297, i64 5)
 31208|  to label %383 unwind label %47                                                                                        ;L80
 31209| 
 31210| 366: ; preds = %363
 31211|  %367 = extractvalue { i64, i64 } %364, 0                                                                              ;L83
 31212|  %368 = extractvalue { i64, i64 } %364, 1                                                                              ;L83
 31213|     ;; camp[0..+8] = i64 %367
 31214|     ;; camp[8..+8] = i64 %368
 31217|  invoke void @ai::small_action6aroundNtB5_25SmallActionAroundPosition3new(ptr sret([184 x i8]) %18, ptr %3, ptr %5, i64 %367, i64 %368, i64 5)
 31218|  to label %369 unwind label %47                                                                                        ;L84
 31219| 
 31220| 369: ; preds = %366
 31221|  call void @llvm.memcpy.p0.p0.i64(ptr %19, ptr %18, i64 184, i1 false)                                                 ;L84
 31223|     ;; self = ptr %36
 31224|     ;; self = ptr %36
 31225|     ;; value = ptr %19
 31226|     ;; src = ptr %19
 31227|     ;; additional = i64 1
 31228|     ;; needed_extra_cap = i64 1
 31229|     ;; needed_extra_cap = i64 1
 31230|     ;; strategy = i8 1
 31231|  %370 = load i64, ptr %42, , !!44845, !!8                                                                              ;L1428<84
 31232|     ;; self = ptr %36
 31233|  %371 = load i64, ptr %41, , !!44845, !!8                                                                              ;L149<1428<84
 31234|  %372 = icmp eq i64 %370, %371                                                                                         ;L1428<84
 31235|  br i1 %372, label %373, label %378                                                                                    ;L1428<84
 31236| 
 31237| 373: ; preds = %369
 31238|     ;; self = ptr %36
 31239|     ;; self = ptr %36
 31240|     ;; self = ptr %36
 31241|     ;; used_cap = i64 %370
 31242|     ;; used_cap = i64 %370
 31243|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %36, i64 %370, i64 1, i1 zeroext true)
 31244|  to label %374 unwind label %376, !!44845                                                                              ;L619<430<738<1429<84
 31245| 
 31246| 374: ; preds = %373
 31247|  %375 = load i64, ptr %42, , !!44845                                                                                   ;L1432<84
 31248|  br label %378                                                                                                         ;L619<430<738<1429<84
 31249| 
 31250| 376: ; preds = %373
 31251|  %377 = cleanuppad within none []
 31252|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %19) #34 [ "funclet"(token %377) ] ;L1436<84
 31253|  cleanupret from %377 unwind label %47
 31254| 
 31255| 378: ; preds = %374, %369
 31256|  %379 = phi i64 [ %375, %374 ], [ %370, %369 ]                                                                         ;L1434<84
 31257|     ;; self = ptr %36
 31258|  %380 = load ptr, ptr %36, , !!44845, !!8, !!8                                                                         ;L138<1432<84
 31259|     ;; self = ptr %380
 31260|     ;; count = i64 %379
 31261|  %381 = gepS %380, i64 %379                                                                                            ;L961<1432<84
 31262|     ;; end = ptr %381
 31263|     ;; dst = ptr %381
 31264|  call void @llvm.memcpy.p0.p0.i64(ptr %381, ptr %19, i64 184, i1 false)                                                ;L1933<1433<84
 31265|  %382 = add i64 %379, 1                                                                                                ;L1434<84
 31266|  store i64 %382, ptr %42, , !!44845                                                                                    ;L1434<84
 31268|  br label %326                                                                                                         ;L79
 31269| 
 31270| 383: ; preds = %365
 31271|  call void @llvm.memcpy.p0.p0.i64(ptr %21, ptr %20, i64 184, i1 false)                                                 ;L80
 31273|     ;; self = ptr %36
 31274|     ;; self = ptr %36
 31275|     ;; value = ptr %21
 31276|     ;; src = ptr %21
 31277|     ;; additional = i64 1
 31278|     ;; needed_extra_cap = i64 1
 31279|     ;; needed_extra_cap = i64 1
 31280|     ;; strategy = i8 1
 31281|  %384 = load i64, ptr %42, , !!44879, !!8                                                                              ;L1428<80
 31282|     ;; self = ptr %36
 31283|  %385 = load i64, ptr %41, , !!44879, !!8                                                                              ;L149<1428<80
 31284|  %386 = icmp eq i64 %384, %385                                                                                         ;L1428<80
 31285|  br i1 %386, label %387, label %392                                                                                    ;L1428<80
 31286| 
 31287| 387: ; preds = %383
 31288|     ;; self = ptr %36
 31289|     ;; self = ptr %36
 31290|     ;; self = ptr %36
 31291|     ;; used_cap = i64 %384
 31292|     ;; used_cap = i64 %384
 31293|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %36, i64 %384, i64 1, i1 zeroext true)
 31294|  to label %388 unwind label %390, !!44879                                                                              ;L619<430<738<1429<80
 31295| 
 31296| 388: ; preds = %387
 31297|  %389 = load i64, ptr %42, , !!44879                                                                                   ;L1432<80
 31298|  br label %392                                                                                                         ;L619<430<738<1429<80
 31299| 
 31300| 390: ; preds = %387
 31301|  %391 = cleanuppad within none []
 31302|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %21) #34 [ "funclet"(token %391) ] ;L1436<80
 31303|  cleanupret from %391 unwind label %47
 31304| 
 31305| 392: ; preds = %388, %383
 31306|  %393 = phi i64 [ %389, %388 ], [ %384, %383 ]                                                                         ;L1434<80
 31307|     ;; self = ptr %36
 31308|  %394 = load ptr, ptr %36, , !!44879, !!8, !!8                                                                         ;L138<1432<80
 31309|     ;; self = ptr %394
 31310|     ;; count = i64 %393
 31311|  %395 = gepS %394, i64 %393                                                                                            ;L961<1432<80
 31312|     ;; end = ptr %395
 31313|     ;; dst = ptr %395
 31314|  call void @llvm.memcpy.p0.p0.i64(ptr %395, ptr %21, i64 184, i1 false)                                                ;L1933<1433<80
 31315|  %396 = add i64 %393, 1                                                                                                ;L1434<80
 31316|  store i64 %396, ptr %42, , !!44879                                                                                    ;L1434<80
 31318|  br label %326                                                                                                         ;L79
 31319| 
 31320| 397: ; preds = %361
 31321|  %398 = gep %54, i64 240                                                                                               ;L92
 31322|  %399 = getelementptr { { ptr, ptr, i64 }, i64 }, ptr %398, i64 %102                                                   ;L92
 31323|     ;; self = ptr %399
 31324|     ;; self = ptr %399
 31325|  %400 = load ptr, ptr %399, , !!8, !!8                                                                                 ;L138<2073<92
 31326|     ;; p = ptr %400
 31327|  %401 = gep %399, i64 24                                                                                               ;L2075<92
 31328|  %402 = load i64, ptr %401, , !!8                                                                                      ;L2075<92
 31329|     ;; len = i64 %402
 31330|     ;; count = i64 %402
 31331|     ;; self[0..+8] = ptr %400
 31332|     ;; slice[0..+8] = ptr %400
 31333|     ;; self[8..+8] = i64 %402
 31334|     ;; slice[8..+8] = i64 %402
 31335|     ;; ptr = ptr %400
 31336|     ;; self = ptr %400
 31337|  %403 = getelementptr ptr, ptr %400, i64 %402                                                                          ;L961<100<1042<92
 31339|     ;; f = ptr %58
 31340|     ;; self = ptr undef
 31341|     ;; self = ptr undef
 31342|     ;; count = i64 1
 31343|  %404 = load i64, ptr %156, , !!44948
 31344|  %405 = load i64, ptr %158, , !!44948
 31345|  br label %406                                                                                                         ;L331<92
 31346| 
 31347| 406: ; preds = %409, %397
 31348|  %407 = phi ptr [ %410, %409 ], [ %400, %397 ]
 31349|     ;; ptr = ptr %407
 31350|     ;; self = ptr %407
 31351|     ;; end_or_len = ptr %403
 31354|  %408 = icmp eq ptr %407, %403                                                                                         ;L1714<180<331<92
 31355|  br i1 %408, label %428, label %409                                                                                    ;L180<331<92
 31356| 
 31357| 409: ; preds = %406
 31358|  %410 = gep %407, i64 8                                                                                                ;L656<185<331<92
 31359|     ;; x = ptr %407
 31360|  %411 = load ptr, ptr %407, , !!44964, !!8, !!8                                                                        ;L332<92
 31363|     ;; self = ptr %411
 31364|     ;; other = ptr %58
 31365|  %412 = gep %411, i64 1632                                                                                             ;L2158<92<332<92
 31366|  %413 = load i64, ptr %412, , !!44964, !!8                                                                             ;L2158<92<332<92
 31367|     ;; x1 = i64 %413
 31368|     ;; self = i64 %413
 31369|  %414 = gep %411, i64 1640                                                                                             ;L2158<92<332<92
 31370|  %415 = load i64, ptr %414, , !!44964, !!8                                                                             ;L2158<92<332<92
 31371|     ;; y1 = i64 %415
 31372|     ;; self = i64 %415
 31373|     ;; x2 = i64 %404
 31374|     ;; other = i64 %404
 31375|     ;; y2 = i64 %405
 31376|     ;; other = i64 %405
 31377|  %416 = icmp ult i64 %413, %404                                                                                        ;L3147<7<2158<92<332<92
 31378|  %417 = sub nuw i64 %404, %413                                                                                         ;L3147<7<2158<92<332<92
 31379|  %418 = sub nuw i64 %413, %404                                                                                         ;L3147<7<2158<92<332<92
 31380|  %419 = select i1 %416, i64 %417, i64 %418                                                                             ;L3147<7<2158<92<332<92
 31381|     ;; dx = i64 %419
 31382|  %420 = icmp ult i64 %415, %405                                                                                        ;L3147<8<2158<92<332<92
 31383|  %421 = sub nuw i64 %405, %415                                                                                         ;L3147<8<2158<92<332<92
 31384|  %422 = sub nuw i64 %415, %405                                                                                         ;L3147<8<2158<92<332<92
 31385|  %423 = select i1 %420, i64 %421, i64 %422                                                                             ;L3147<8<2158<92<332<92
 31386|     ;; dy = i64 %423
 31387|  %424 = mul i64 %419, %419                                                                                             ;L9<2158<92<332<92
 31388|  %425 = mul i64 %423, %423                                                                                             ;L9<2158<92<332<92
 31389|  %426 = add i64 %425, %424                                                                                             ;L9<2158<92<332<92
 31390|  %427 = icmp ult i64 %426, 22500000001                                                                                 ;L92<332<92
 31391|  br i1 %427, label %434, label %406                                                                                    ;L332<92
 31392| 
 31393| 428: ; preds = %445, %406
 31394|  %429 = gep %58, i64 1472                                                                                              ;L98
 31395|  %430 = load i64, ptr %429, , !!8                                                                                      ;L98
 31396|  %431 = gep %329, i64 248                                                                                              ;L98
 31397|  %432 = load ptr, ptr %431, , !!8                                                                                      ;L98
 31398|  %433 = invoke zeroext i1 %432(ptr %327, i64 %102, i64 %430)
 31399|  to label %450 unwind label %47                                                                                        ;L98
 31400| 
 31401| 434: ; preds = %409, %342
 31404|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway3new(ptr sret([136 x i8]) %14, ptr %5, ptr %4, i64 5)
 31405|  to label %435 unwind label %47                                                                                        ;L95
 31406| 
 31407| 435: ; preds = %434
 31408|  call void @llvm.memcpy.p0.p0.i64(ptr %15, ptr %14, i64 136, i1 false)                                                 ;L95
 31409|  %436 = gep %15, i64 177                                                                                               ;L95
 31410|  store i8 3, ptr %436,                                                                                                 ;L95
 31412|     ;; self = ptr %36
 31413|     ;; self = ptr %36
 31414|     ;; value = ptr %15
 31415|     ;; src = ptr %15
 31416|     ;; additional = i64 1
 31417|     ;; needed_extra_cap = i64 1
 31418|     ;; needed_extra_cap = i64 1
 31419|     ;; strategy = i8 1
 31420|  %437 = load i64, ptr %42, , !!45024, !!8                                                                              ;L1428<95
 31421|     ;; self = ptr %36
 31422|  %438 = load i64, ptr %41, , !!45024, !!8                                                                              ;L149<1428<95
 31423|  %439 = icmp eq i64 %437, %438                                                                                         ;L1428<95
 31424|  br i1 %439, label %440, label %445                                                                                    ;L1428<95
 31425| 
 31426| 440: ; preds = %435
 31427|     ;; self = ptr %36
 31428|     ;; self = ptr %36
 31429|     ;; self = ptr %36
 31430|     ;; used_cap = i64 %437
 31431|     ;; used_cap = i64 %437
 31432|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %36, i64 %437, i64 1, i1 zeroext true)
 31433|  to label %441 unwind label %443, !!45024                                                                              ;L619<430<738<1429<95
 31434| 
 31435| 441: ; preds = %440
 31436|  %442 = load i64, ptr %42, , !!45024                                                                                   ;L1432<95
 31437|  br label %445                                                                                                         ;L619<430<738<1429<95
 31438| 
 31439| 443: ; preds = %440
 31440|  %444 = cleanuppad within none []
 31441|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %15) #34 [ "funclet"(token %444) ] ;L1436<95
 31442|  cleanupret from %444 unwind label %47
 31443| 
 31444| 445: ; preds = %441, %435
 31445|  %446 = phi i64 [ %442, %441 ], [ %437, %435 ]                                                                         ;L1434<95
 31446|     ;; self = ptr %36
 31447|  %447 = load ptr, ptr %36, , !!45024, !!8, !!8                                                                         ;L138<1432<95
 31448|     ;; self = ptr %447
 31449|     ;; count = i64 %446
 31450|  %448 = gepS %447, i64 %446                                                                                            ;L961<1432<95
 31451|     ;; end = ptr %448
 31452|     ;; dst = ptr %448
 31453|  call void @llvm.memcpy.p0.p0.i64(ptr %448, ptr %15, i64 184, i1 false)                                                ;L1933<1433<95
 31454|  %449 = add i64 %446, 1                                                                                                ;L1434<95
 31455|  store i64 %449, ptr %42, , !!45024                                                                                    ;L1434<95
 31457|  br label %428                                                                                                         ;L94
 31458| 
 31459| 450: ; preds = %428
 31460|  br i1 %433, label %451, label %452                                                                                    ;L98
 31461| 
 31462| 451: ; preds = %450
 31464|  invoke void @ai::fight_check13battle_action(ptr sret([32 x i8]) %13, i64 %2, ptr %3, ptr %4, ptr %5, i64 5)
 31465|  to label %453 unwind label %47                                                                                        ;L99
 31466| 
 31467| 452: ; preds = %457, %450
 31469|  invoke void @ai::fight_check20attack_summon_action(ptr sret([32 x i8]) %12, ptr %4, ptr %5)
 31470|  to label %458 unwind label %47                                                                                        ;L101
 31471| 
 31472| 453: ; preds = %451
 31473|  %454 = load ptr, ptr %13, , !!8, !!8                                                                                  ;L99
 31474|  %455 = gep %13, i64 24                                                                                                ;L99
 31475|  %456 = load i64, ptr %455, , !!8                                                                                      ;L99
 31476|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %36, ptr %454, i64 %456)
 31477|  to label %457 unwind label %47                                                                                        ;L99
 31478| 
 31479| 457: ; preds = %453
 31481|  br label %452                                                                                                         ;L98
 31482| 
 31483| 458: ; preds = %452
 31484|  %459 = load ptr, ptr %12, , !!8, !!8                                                                                  ;L101
 31485|  %460 = gep %12, i64 24                                                                                                ;L101
 31486|  %461 = load i64, ptr %460, , !!8                                                                                      ;L101
 31487|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %36, ptr %459, i64 %461)
 31488|  to label %462 unwind label %47                                                                                        ;L101
 31489| 
 31490| 462: ; preds = %458
 31492|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %36, i64 32, i1 false)                                                   ;L103
 31494|  br label %463                                                                                                         ;L104
 31495| 
 31496| 463: ; preds = %474, %462, %251
 31498|  ret void                                                                                                              ;L104
 31499| 
 31500| 464: ; preds = %169
 31501|  call void @llvm.memcpy.p0.p0.i64(ptr %34, ptr %33, i64 136, i1 false)                                                 ;L52
 31502|  %465 = gep %34, i64 177                                                                                               ;L52
 31503|  store i8 3, ptr %465,                                                                                                 ;L52
 31505|     ;; self = ptr %36
 31506|     ;; self = ptr %36
 31507|     ;; value = ptr %34
 31508|     ;; src = ptr %34
 31509|     ;; additional = i64 1
 31510|     ;; needed_extra_cap = i64 1
 31511|     ;; needed_extra_cap = i64 1
 31512|     ;; strategy = i8 1
 31513|  %466 = load i64, ptr %42, , !!45062, !!8                                                                              ;L1428<52
 31514|     ;; self = ptr %36
 31515|  %467 = load i64, ptr %41, , !!45062, !!8                                                                              ;L149<1428<52
 31516|  %468 = icmp eq i64 %466, %467                                                                                         ;L1428<52
 31517|  br i1 %468, label %469, label %474                                                                                    ;L1428<52
 31518| 
 31519| 469: ; preds = %464
 31520|     ;; self = ptr %36
 31521|     ;; self = ptr %36
 31522|     ;; self = ptr %36
 31523|     ;; used_cap = i64 %466
 31524|     ;; used_cap = i64 %466
 31525|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %36, i64 %466, i64 1, i1 zeroext true)
 31526|  to label %470 unwind label %472, !!45062                                                                              ;L619<430<738<1429<52
 31527| 
 31528| 470: ; preds = %469
 31529|  %471 = load i64, ptr %42, , !!45062                                                                                   ;L1432<52
 31530|  br label %474                                                                                                         ;L619<430<738<1429<52
 31531| 
 31532| 472: ; preds = %469
 31533|  %473 = cleanuppad within none []
 31534|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %34) #34 [ "funclet"(token %473) ] ;L1436<52
 31535|  cleanupret from %473 unwind label %47
 31536| 
 31537| 474: ; preds = %470, %464
 31538|  %475 = phi i64 [ %471, %470 ], [ %466, %464 ]                                                                         ;L1434<52
 31539|     ;; self = ptr %36
 31540|  %476 = load ptr, ptr %36, , !!45062, !!8, !!8                                                                         ;L138<1432<52
 31541|     ;; self = ptr %476
 31542|     ;; count = i64 %475
 31543|  %477 = gepS %476, i64 %475                                                                                            ;L961<1432<52
 31544|     ;; end = ptr %477
 31545|     ;; dst = ptr %477
 31546|  call void @llvm.memcpy.p0.p0.i64(ptr %477, ptr %34, i64 184, i1 false)                                                ;L1933<1433<52
 31547|  %478 = add i64 %475, 1                                                                                                ;L1434<52
 31548|  store i64 %478, ptr %42, , !!45062                                                                                    ;L1434<52
 31550|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %36, i64 32, i1 false)                                                   ;L53
 31552|  br label %463                                                                                                         ;L1
 31553| }
