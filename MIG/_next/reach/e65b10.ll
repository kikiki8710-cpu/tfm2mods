 45629| define void @ai::plan_legacy7handler7auctionNtB4_17LegacyPlanHandler16get_small_action(ptr sret([5576 x i8]) %0, ptr %1, i64 %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %7, ptr %8) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 45630|  %10 = alloca [32 x i8],
 45631|  %11 = alloca [8 x i8],
 45632|  %12 = alloca [24 x i8],
 45633|  %13 = alloca [8 x i8],
 45634|  %14 = alloca [128 x i8],
 45635|  %15 = alloca [128 x i8],
 45636|  %16 = alloca [24 x i8],
 45637|  %17 = alloca [24 x i8],
 45638|  %18 = alloca [184 x i8],
 45641|  %19 = alloca [24 x i8],
 45642|  %20 = alloca [32 x i8],
 45643|  %21 = alloca [8 x i8],
 45644|  %22 = alloca [32 x i8],
 45645|  %23 = alloca [184 x i8],
 45648|  %24 = alloca [24 x i8],
 45649|  %25 = alloca [32 x i8],
 45650|  %26 = alloca [8 x i8],
 45651|  %27 = alloca [24 x i8],
 45652|  %28 = alloca [32 x i8],
 45653|  %29 = alloca [8 x i8],
 45654|  %30 = alloca [96 x i8],
 45655|  %31 = alloca [24 x i8],
 45656|  %32 = alloca [24 x i8],
 45659|  %33 = alloca [16 x i8],
 45660|  %34 = alloca [24 x i8],
 45661|  %35 = alloca [24 x i8],
 45662|  %36 = alloca [24 x i8],
 45663|  %37 = alloca [1 x i8],
 45664|  %38 = alloca [96 x i8],
 45665|  %39 = alloca [8 x i8],
 45666|  %40 = alloca [24 x i8],
 45667|  %41 = alloca [24 x i8],
 45668|  %42 = alloca [80 x i8],
 45669|  %43 = alloca [24 x i8],
 45670|  %44 = alloca [24 x i8],
 45671|  %45 = alloca [184 x i8],
 45672|  %46 = alloca [24 x i8],
 45673|  %47 = alloca [32 x i8],
 45674|  %48 = alloca [184 x i8],
 45675|  %49 = alloca [192 x i8],
 45676|  %50 = alloca [24 x i8],
 45677|  %51 = alloca [32 x i8],
 45678|  %52 = alloca [136 x i8],
 45679|  %53 = alloca [184 x i8],
 45680|  %54 = alloca [8 x i8],
 45681|  %55 = alloca [24 x i8],
 45682|  %56 = alloca [32 x i8],
 45685|  %57 = alloca [24 x i8],
 45686|  %58 = alloca [32 x i8],
 45687|  %59 = alloca [48 x i8],
 45688|  %60 = alloca [32 x i8],
 45689|  %61 = alloca [24 x i8],
 45690|  %62 = alloca [24 x i8],
 45691|  %63 = alloca [24 x i8],
 45692|  %64 = alloca [8 x i8],
 45694|  %65 = alloca [184 x i8],
 45702|  %66 = alloca [120 x i8],
 45703|  %67 = alloca [184 x i8],
 45706|  %68 = alloca [24 x i8],
 45707|  %69 = alloca [80 x i8],
 45708|  %70 = alloca [32 x i8],
 45709|  %71 = alloca [88 x i8],
 45710|  %72 = alloca [24 x i8],
 45711|  %73 = alloca [24 x i8],
 45713|  %74 = alloca [32 x i8],
 45714|  %75 = alloca [88 x i8],
 45715|  %76 = alloca [32 x i8],
 45716|  %77 = alloca [8 x i8],
 45717|  %78 = alloca [32 x i8],
 45718|  %79 = alloca [24 x i8],
 45719|  %80 = alloca [32 x i8],
 45720|  %81 = alloca [24 x i8],
 45721|  %82 = alloca [24 x i8],
 45722|  %83 = alloca [5384 x i8],
 45723|  %84 = alloca [136 x i8],
 45724|  %85 = alloca [5384 x i8],
 45725|  %86 = alloca [160 x i8],
 45726|  %87 = alloca [24 x i8],
 45727|  %88 = alloca [1 x i8],
 45728|  %89 = alloca [16 x i8],
 45729|  %90 = alloca [1 x i8],
 45730|  %91 = alloca [24 x i8],
 45731|  %92 = alloca [2320 x i8],
 45732|  %93 = alloca [24 x i8],
 45733|  %94 = alloca [24 x i8],
 45734|  %95 = alloca [5384 x i8],
 45735|  %96 = alloca [24 x i8],
 45736|  %97 = alloca [8 x i8],
 45737|  store i64 %2, ptr %97,
 45738|     ;; self = ptr %1
 45739|     ;; version = ptr %97
 45740|     ;; rnd = ptr %3
 45741|     ;; player = ptr %4
 45742|     ;; data = ptr %5
 45743|     ;; pre_action = ptr %6
 45744|     ;; ignore_action = ptr %7
 45745|     ;; self = ptr %7
 45746|     ;; self = ptr %7
 45747|     ;; debug = ptr %8
 45748|     ;; _t_csp = ptr %96
 45749|     ;; parameter = ptr %95
 45750|     ;; _x = ptr %94
 45751|     ;; _t_ws = ptr %93
 45752|     ;; _x = ptr %91
 45753|     ;; base_exempt = ptr %90
 45754|     ;; sh = ptr %89
 45755|     ;; scene_now = ptr %88
 45756|     ;; value = ptr %87
 45757|     ;; _t_ac = ptr %81
 45758|     ;; candidates = ptr %80
 45759|     ;; _x = ptr %79
 45760|     ;; candidates = ptr %78
 45761|     ;; move_speed = ptr %77
 45762|     ;; filtered = ptr %76
 45763|     ;; combat = ptr %74
 45764|     ;; _t_sl = ptr %72
 45765|     ;; judge_noise_ratio = ptr %71
 45766|     ;; with_score = ptr %70
 45767|     ;; _x = ptr %68
 45768|     ;; value = ptr %67
 45769|     ;; self = ptr %66
 45770|     ;; value = ptr %65
 45771|     ;; score = ptr %64
 45772|     ;; value = ptr %62
 45773|     ;; value = ptr %62
 45774|     ;; with_score = ptr %58
 45775|     ;; keep = ptr %57
 45776|     ;; filtered = ptr %56
 45777|     ;; max_score = ptr %54
 45778|     ;; run = ptr %53
 45779|     ;; max_score_actions = ptr %51
 45780|     ;; result = ptr %49
 45781|     ;; without_ignore_action = ptr %47
 45782|     ;; value = ptr %44
 45783|     ;; value = ptr %41
 45784|     ;; dive = ptr %37
 45785|     ;; subtag = ptr %36
 45786|     ;; s = ptr %35
 45787|     ;; value = ptr %34
 45788|     ;; value = ptr %32
 45789|     ;; target_id = ptr %29
 45790|     ;; actions = ptr %28
 45791|     ;; max_score = ptr %26
 45792|     ;; max_score_actions = ptr %25
 45793|     ;; result = ptr %23
 45794|     ;; actions = ptr %22
 45795|     ;; max_score = ptr %21
 45796|     ;; max_score_actions = ptr %20
 45797|     ;; result = ptr %18
 45798|     ;; raw = ptr %17
 45799|     ;; phase = i64 33
 45800|     ;; order = i8 0
 45801|     ;; phase = i64 35
 45802|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 45803|     ;; order = i8 0
 45804|     ;; phase = i64 36
 45805|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 45806|     ;; order = i8 0
 45807|     ;; index = i64 0
 45808|     ;; count = i64 1
 45809|     ;; phase = i64 37
 45810|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 45811|     ;; order = i8 0
 45813|     ;; default = i64 0
 45815|     ;; count = i64 1
 45825|     ;; len = i64 0
 45829|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 45830|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 45831|     ;; order = i8 0
 45832|  %98 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                     ;L3904<741<176<9
 45833|  %99 = icmp eq i8 %98, 0                                                                                               ;L176<9
 45834|  br i1 %99, label %105, label %100                                                                                     ;L176<9
 45835| 
 45836| 100: ; preds = %9
 45837|  %101 = tail call { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()                                      ;L179<9
 45838|  %102 = extractvalue { i64, i32 } %101, 0                                                                              ;L179<9
 45839|  %103 = extractvalue { i64, i32 } %101, 1                                                                              ;L179<9
 45840|  store i64 33, ptr %96,                                                                                                ;L179<9
 45841|  %104 = gep %96, i64 8                                                                                                 ;L179<9
 45842|  store i64 %102, ptr %104,                                                                                             ;L179<9
 45843|  br label %105                                                                                                         ;L180<9
 45844| 
 45845| 105: ; preds = %100, %9
 45846|  %106 = phi i32 [ %103, %100 ], [ -1, %9 ]                                                                             ;L0<9
 45847|  %107 = gep %96, i64 16                                                                                                ;L0<9
 45848|  store i32 %106, ptr %107,                                                                                             ;L0<9
 45850|  invoke void @ai::score_parameter25calculate_score_parameter(ptr sret([5384 x i8]) %95, i64 %2, ptr %3, ptr %4, ptr %5, ptr %8)
 45851|  to label %111 unwind label %108                                                                                       ;L10
 45852| 
 45853| 108: ; preds = %2514, %2513, %105
 45854|  %109 = phi i1 [ %116, %2514 ], [ %116, %2513 ], [ true, %105 ]                                                        ;L0
 45855|  %110 = cleanuppad within none []
 45856|  br i1 %109, label %2516, label %2515                                                                                  ;L416
 45857| 
 45858| 111: ; preds = %105
 45859|  %112 = gep %1, i64 1896                                                                                               ;L11
 45860|  %113 = load i64, ptr %97, , !!8                                                                                       ;L11
 45861|  invoke void @ai::plan_legacy8sub_planNtB4_7SubPlan31calculate_score_parameter_value(ptr %112, i64 %113, ptr %3, ptr %4, ptr %5, ptr %95)
 45862|  to label %118 unwind label %114                                                                                       ;L11
 45863| 
 45864| 114: ; preds = %2512, %2511, %146, %129, %127, %111
 45865|  %115 = phi i1 [ %159, %2512 ], [ %159, %2511 ], [ true, %146 ], [ true, %127 ], [ true, %129 ], [ true, %111 ]        ;L10
 45866|  %116 = phi i1 [ false, %2512 ], [ false, %2511 ], [ false, %146 ], [ false, %127 ], [ false, %129 ], [ true, %111 ]   ;L0
 45867|  %117 = cleanuppad within none []
 45868|  br i1 %115, label %2514, label %2513                                                                                  ;L416
 45869| 
 45870| 118: ; preds = %111
 45871|  %119 = gep %95, i64 2544                                                                                              ;L12
 45872|  %120 = gep %1, i64 2448                                                                                               ;L12
 45873|  call void @llvm.memcpy.p0.p0.i64(ptr %120, ptr %119, i64 2760, i1 false)                                              ;L12
 45875|  call void @llvm.memcpy.p0.p0.i64(ptr %94, ptr %96, i64 24, i1 false)                                                  ;L13
 45878|  %121 = gep %94, i64 16                                                                                                ;L825<1004<13
 45879|  %122 = load i32, ptr %121, , !!8                                                                                      ;L825<1004<13
 45880|  %123 = icmp eq i32 %122, -1                                                                                           ;L825<1004<13
 45881|  br i1 %123, label %143, label %124                                                                                    ;L825<1004<13
 45882| 
 45883| 124: ; preds = %118
 45887|     ;; self = ptr %94
 45888|     ;; order = i8 0
 45889|     ;; order = i8 0
 45890|     ;; val = i64 1
 45891|     ;; order = i8 0
 45892|     ;; val = i64 1
 45893|     ;; order = i8 0
 45894|  %125 = load i64, ptr %94, , !!8                                                                                       ;L185<825<825<1004<13
 45895|  %126 = icmp ult i64 %125, 132                                                                                         ;L185<825<825<1004<13
 45896|  br i1 %126, label %129, label %127                                                                                    ;L185<825<825<1004<13
 45897| 
 45898| 127: ; preds = %124
 45899|  invoke void @core::panicking18panic_bounds_check(i64 %125, i64 132, ptr @anon.282069a2ed2ad3a275929b639963fb55.182) #32
 45900|  to label %128 unwind label %114                                                                                       ;L185<825<825<1004<13
 45901| 
 45902| 128: ; preds = %127
 45903|  unreachable                                                                                                           ;L185<825<825<1004<13
 45904| 
 45905| 129: ; preds = %124
 45906|     ;; self = !DIArgList(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 %125)
 45907|  %130 = gep %94, i64 8                                                                                                 ;L185<825<825<1004<13
 45908|  %131 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %130)
 45909|  to label %132 unwind label %114                                                                                       ;L185<825<825<1004<13
 45910| 
 45911| 132: ; preds = %129
 45912|  %133 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %125                                 ;L185<825<825<1004<13
 45913|     ;; self = ptr %133
 45914|  %134 = extractvalue { i64, i32 } %131, 0                                                                              ;L185<825<825<1004<13
 45915|  %135 = extractvalue { i64, i32 } %131, 1                                                                              ;L185<825<825<1004<13
 45917|  %136 = mul i64 %134, 1000000000                                                                                       ;L632<185<825<825<1004<13
 45918|  %137 = icmp ult i32 %135, 1000000000                                                                                  ;L49<632<185<825<825<1004<13
 45919|  call void @llvm.assume(i1 %137)                                                                                       ;L49<632<185<825<825<1004<13
 45920|  %138 = zext nneg i32 %135 to i64                                                                                      ;L632<185<825<825<1004<13
 45921|  %139 = add i64 %136, %138                                                                                             ;L632<185<825<825<1004<13
 45922|     ;; val = i64 %139
 45923|     ;; val = i64 %139
 45924|     ;; dst = ptr %133
 45925|  %140 = atomicrmw add ptr %133, i64 %139 monotonic, , !!50166                                                          ;L3937<3162<185<825<825<1004<13
 45926|  %141 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %125                                 ;L186<825<825<1004<13
 45927|     ;; self = ptr %141
 45928|     ;; dst = ptr %141
 45929|  %142 = atomicrmw add ptr %141, i64 1 monotonic, , !!50166                                                             ;L3937<3162<186<825<825<1004<13
 45930|  br label %143                                                                                                         ;L825<1004<13
 45931| 
 45932| 143: ; preds = %132, %118
 45935|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 45936|     ;; order = i8 0
 45937|  %144 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                    ;L3904<741<176<16
 45938|  %145 = icmp eq i8 %144, 0                                                                                             ;L176<16
 45939|  br i1 %145, label %148, label %146                                                                                    ;L176<16
 45940| 
 45941| 146: ; preds = %143
 45942|  %147 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()
 45943|  to label %153 unwind label %114                                                                                       ;L179<16
 45944| 
 45945| 148: ; preds = %153, %143
 45946|  %149 = phi i32 [ %155, %153 ], [ -1, %143 ]
 45947|  %150 = gep %93, i64 16                                                                                                ;L0<16
 45948|  store i32 %149, ptr %150,                                                                                             ;L0<16
 45949|  %151 = gep %4, i64 384                                                                                                ;L17
 45950|  %152 = invoke i64 @gc::simulation5state6playerNtB4_16AthleteParameter25last_hit_prediction_depth(ptr %151)
 45951|  to label %161 unwind label %157                                                                                       ;L17
 45952| 
 45953| 153: ; preds = %146
 45954|  %154 = extractvalue { i64, i32 } %147, 0                                                                              ;L179<16
 45955|  %155 = extractvalue { i64, i32 } %147, 1                                                                              ;L179<16
 45956|  store i64 35, ptr %93,                                                                                                ;L179<16
 45957|  %156 = gep %93, i64 8                                                                                                 ;L179<16
 45958|  store i64 %154, ptr %156,                                                                                             ;L179<16
 45959|  br label %148                                                                                                         ;L180<16
 45960| 
 45961| 157: ; preds = %2510, %2509, %410, %400, %395, %363, %334, %331, %318, %312, %308, %302, %297, %273, %235, %230, %222, %208, %207, %202, %174, %172, %163, %161, %148
 45962|  %158 = phi i1 [ false, %2510 ], [ false, %2509 ], [ false, %410 ], [ false, %174 ], [ false, %400 ], [ false, %395 ], [ false, %363 ], [ true, %163 ], [ false, %334 ], [ false, %331 ], [ false, %318 ], [ false, %312 ], [ false, %308 ], [ false, %302 ], [ false, %297 ], [ false, %273 ], [ false, %172 ], [ false, %235 ], [ false, %230 ], [ true, %161 ], [ false, %222 ], [ false, %208 ], [ false, %207 ], [ true, %148 ], [ false, %202 ] ;L0
 45963|  %159 = phi i1 [ %423, %2510 ], [ %423, %2509 ], [ true, %410 ], [ true, %174 ], [ false, %400 ], [ true, %395 ], [ true, %363 ], [ true, %163 ], [ true, %334 ], [ true, %331 ], [ true, %318 ], [ false, %312 ], [ true, %308 ], [ true, %302 ], [ true, %297 ], [ true, %273 ], [ true, %172 ], [ true, %235 ], [ true, %230 ], [ true, %161 ], [ true, %222 ], [ true, %208 ], [ true, %207 ], [ true, %148 ], [ true, %202 ] ;L0
 45964|  %160 = cleanuppad within none []
 45965|  br i1 %158, label %2512, label %2511                                                                                  ;L416
 45966| 
 45967| 161: ; preds = %148
 45968|     ;; prediction_depth = i64 %152
 45969|  %162 = invoke i64 @gc::simulation5state6playerNtB4_16AthleteParameter23last_hit_source_quality(ptr %151)
 45970|  to label %163 unwind label %157                                                                                       ;L18
 45971| 
 45972| 163: ; preds = %161
 45973|     ;; source_quality = i64 %162
 45975|  invoke void @ai::utils26build_minion_wave_snapshot(ptr sret([2320 x i8]) %92, ptr %4, ptr %5, i64 %152, i64 %162)
 45976|  to label %164 unwind label %157                                                                                       ;L19
 45977| 
 45978| 164: ; preds = %163
 45979|  %165 = gep %95, i64 8                                                                                                 ;L19
 45980|  call void @llvm.memcpy.p0.p0.i64(ptr %165, ptr %92, i64 2320, i1 false)                                               ;L19
 45982|  store i64 1, ptr %95,                                                                                                 ;L19
 45984|  call void @llvm.memcpy.p0.p0.i64(ptr %91, ptr %93, i64 24, i1 false)                                                  ;L20
 45987|  %166 = gep %91, i64 16                                                                                                ;L825<1004<20
 45988|  %167 = load i32, ptr %166, , !!8                                                                                      ;L825<1004<20
 45989|  %168 = icmp eq i32 %167, -1                                                                                           ;L825<1004<20
 45990|  br i1 %168, label %188, label %169                                                                                    ;L825<1004<20
 45991| 
 45992| 169: ; preds = %164
 45996|     ;; self = ptr %91
 45997|     ;; order = i8 0
 45998|     ;; order = i8 0
 45999|     ;; val = i64 1
 46000|     ;; order = i8 0
 46001|     ;; val = i64 1
 46002|     ;; order = i8 0
 46003|  %170 = load i64, ptr %91, , !!8                                                                                       ;L185<825<825<1004<20
 46004|  %171 = icmp ult i64 %170, 132                                                                                         ;L185<825<825<1004<20
 46005|  br i1 %171, label %174, label %172                                                                                    ;L185<825<825<1004<20
 46006| 
 46007| 172: ; preds = %169
 46008|  invoke void @core::panicking18panic_bounds_check(i64 %170, i64 132, ptr @anon.282069a2ed2ad3a275929b639963fb55.182) #32
 46009|  to label %173 unwind label %157                                                                                       ;L185<825<825<1004<20
 46010| 
 46011| 173: ; preds = %172
 46012|  unreachable                                                                                                           ;L185<825<825<1004<20
 46013| 
 46014| 174: ; preds = %169
 46015|     ;; self = !DIArgList(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 %170)
 46016|  %175 = gep %91, i64 8                                                                                                 ;L185<825<825<1004<20
 46017|  %176 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %175)
 46018|  to label %177 unwind label %157                                                                                       ;L185<825<825<1004<20
 46019| 
 46020| 177: ; preds = %174
 46021|  %178 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %170                                 ;L185<825<825<1004<20
 46022|     ;; self = ptr %178
 46023|  %179 = extractvalue { i64, i32 } %176, 0                                                                              ;L185<825<825<1004<20
 46024|  %180 = extractvalue { i64, i32 } %176, 1                                                                              ;L185<825<825<1004<20
 46026|  %181 = mul i64 %179, 1000000000                                                                                       ;L632<185<825<825<1004<20
 46027|  %182 = icmp ult i32 %180, 1000000000                                                                                  ;L49<632<185<825<825<1004<20
 46028|  call void @llvm.assume(i1 %182)                                                                                       ;L49<632<185<825<825<1004<20
 46029|  %183 = zext nneg i32 %180 to i64                                                                                      ;L632<185<825<825<1004<20
 46030|  %184 = add i64 %181, %183                                                                                             ;L632<185<825<825<1004<20
 46031|     ;; val = i64 %184
 46032|     ;; val = i64 %184
 46033|     ;; dst = ptr %178
 46034|  %185 = atomicrmw add ptr %178, i64 %184 monotonic, , !!50215                                                          ;L3937<3162<185<825<825<1004<20
 46035|  %186 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %170                                 ;L186<825<825<1004<20
 46036|     ;; self = ptr %186
 46037|     ;; dst = ptr %186
 46038|  %187 = atomicrmw add ptr %186, i64 1 monotonic, , !!50215                                                             ;L3937<3162<186<825<825<1004<20
 46039|  br label %188                                                                                                         ;L825<1004<20
 46040| 
 46041| 188: ; preds = %177, %164
 46043|  %189 = gep %4, i64 2352                                                                                               ;L22
 46044|  %190 = load i64, ptr %189, , !!8                                                                                      ;L22
 46045|     ;; team = i64 %190
 46046|  %191 = icmp ult i64 %190, 2                                                                                           ;L22
 46047|  br i1 %191, label %192, label %202                                                                                    ;L22
 46048| 
 46049| 192: ; preds = %188
 46050|     ;; self = ptr %4
 46051|  %193 = gep %4, i64 2496                                                                                               ;L581<22
 46052|  %194 = load i32, ptr %193, , !!8                                                                                      ;L581<22
 46053|  %195 = zext nneg i32 %194 to i64                                                                                      ;L581<22
 46054|  %196 = load ptr, ptr %5, , !!8, !!8                                                                                   ;L22
 46055|     ;; self = ptr %196
 46056|     ;; self = ptr %196
 46057|     ;; self = ptr %196
 46058|     ;; self = ptr %196
 46059|     ;; self = ptr %196
 46060|  %197 = gep %196, i64 480                                                                                              ;L22
 46061|  %198 = getelementptr [5 x ptr], ptr %197, i64 %190                                                                    ;L22
 46062|  %199 = getelementptr ptr, ptr %198, i64 %195                                                                          ;L22
 46063|  %200 = load ptr, ptr %199, , !!8                                                                                      ;L22
 46064|     ;; self = ptr %200
 46065|  %201 = icmp eq ptr %200, null                                                                                         ;L1011<22
 46066|  br i1 %201, label %207, label %204                                                                                    ;L1011<22
 46067| 
 46068| 202: ; preds = %188
 46069|  invoke void @core::panicking18panic_bounds_check(i64 %190, i64 2, ptr @anon.282069a2ed2ad3a275929b639963fb55.144) #32
 46070|  to label %203 unwind label %157                                                                                       ;L22
 46071| 
 46072| 203: ; preds = %2443, %2255, %2195, %2020, %1806, %1472, %1461, %1440, %1391, %1149, %770, %207, %202
 46073|  unreachable
 46074| 
 46075| 204: ; preds = %192
 46076|     ;; champ = ptr %200
 46077|     ;; self = ptr %200
 46078|     ;; other = ptr %200
 46079|     ;; other = ptr %200
 46080|  %205 = load i64, ptr %97, , !!8                                                                                       ;L29
 46081|  %206 = icmp ugt i64 %205, 1                                                                                           ;L29
 46082|  br i1 %206, label %208, label %298                                                                                    ;L29
 46083| 
 46084| 207: ; preds = %192
 46085|  invoke void @core::option13unwrap_failed(ptr @anon.282069a2ed2ad3a275929b639963fb55.145) #32
 46086|  to label %203 unwind label %157                                                                                       ;L1013<22
 46087| 
 46088| 208: ; preds = %204
 46089|  %209 = load ptr, ptr %196, , !!8, !!8                                                                                 ;L29
 46090|  %210 = gep %196, i64 8                                                                                                ;L29
 46091|  %211 = load ptr, ptr %210, , !!8, !!8                                                                                 ;L29
 46092|  %212 = gep %211, i64 64                                                                                               ;L29
 46093|  %213 = load ptr, ptr %212, , !!8                                                                                      ;L29
 46094|  %214 = invoke { i64, ptr } %213(ptr %209)
 46095|  to label %215 unwind label %157                                                                                       ;L29
 46096| 
 46097| 215: ; preds = %208
 46098|  %216 = extractvalue { i64, ptr } %214, 0                                                                              ;L29
 46099|  %217 = icmp eq i64 %216, 2                                                                                            ;L29
 46100|  br i1 %217, label %302, label %218                                                                                    ;L29
 46101| 
 46102| 218: ; preds = %215
 46104|  %219 = gep %200, i64 1160                                                                                             ;L30
 46105|  %220 = load i8, ptr %219, , !!8                                                                                       ;L30
 46106|  %221 = trunc nuw i8 %220 to i1                                                                                        ;L30
 46107|  br i1 %221, label %235, label %222                                                                                    ;L30
 46108| 
 46109| 222: ; preds = %218
 46110|  %223 = invoke zeroext i1 @ai::plan_legacy3old13defense_nexus17nexus_final_stand(ptr %4, ptr %5)
 46111|  to label %224 unwind label %157                                                                                       ;L31
 46112| 
 46113| 224: ; preds = %222
 46114|  br i1 %223, label %235, label %225                                                                                    ;L31
 46115| 
 46116| 225: ; preds = %224
 46117|  %226 = gep %196, i64 368                                                                                              ;L32
 46118|  %227 = getelementptr ptr, ptr %226, i64 %190                                                                          ;L32
 46119|  %228 = load ptr, ptr %227, , !!8                                                                                      ;L32
 46120|     ;; self = ptr %228
 46121|     ;; f = ptr %200
 46122|  %229 = icmp eq ptr %228, null                                                                                         ;L659<32
 46123|  br i1 %229, label %235, label %230                                                                                    ;L659<32
 46124| 
 46125| 230: ; preds = %225
 46126|     ;; x = ptr %228
 46128|     ;; n = ptr %228
 46129|  %231 = invoke i64 @gc::simulation6entityNtB5_6Entity8distance(ptr %200, ptr %228)
 46130|  to label %232 unwind label %157                                                                                       ;L32<661<32
 46131| 
 46132| 232: ; preds = %230
 46133|  %233 = icmp ult i64 %231, 180001                                                                                      ;L32<661<32
 46134|  %234 = zext i1 %233 to i8                                                                                             ;L661<32
 46135|  br label %235                                                                                                         ;L661<32
 46136| 
 46137| 235: ; preds = %232, %225, %224, %218
 46138|  %236 = phi i8 [ 1, %218 ], [ 1, %224 ], [ %234, %232 ], [ 0, %225 ]                                                   ;L0
 46139|  store i8 %236, ptr %90,                                                                                               ;L0
 46141|  invoke void @ai::turnback13solo_hunt_obs(ptr sret([16 x i8]) %89, ptr %209, ptr %211, i64 %190, ptr %200)
 46142|  to label %237 unwind label %157                                                                                       ;L33
 46143| 
 46144| 237: ; preds = %235
 46146|  %238 = load i8, ptr %90, , !!8                                                                                        ;L34
 46147|  %239 = trunc nuw i8 %238 to i1                                                                                        ;L34
 46148|     ;; self = ptr %89
 46149|  %240 = gep %89, i64 9
 46150|  %241 = load i8, ptr %240,
 46151|  %242 = icmp ugt i8 %241, 1
 46152|  %243 = select i1 %239, i1 true, i1 %242                                                                               ;L34
 46153|  br i1 %243, label %260, label %244                                                                                    ;L34
 46154| 
 46155| 244: ; preds = %237
 46156|  %245 = gep %89, i64 10                                                                                                ;L322<34
 46157|  %246 = load i8, ptr %245, , !!8                                                                                       ;L322<34
 46158|  %247 = zext i8 %246 to i64                                                                                            ;L322<34
 46159|  %248 = gep %89, i64 8                                                                                                 ;L322<34
 46160|  %249 = load i8, ptr %248, , !!8                                                                                       ;L322<34
 46161|  %250 = zext i8 %249 to i64                                                                                            ;L322<34
 46162|  %251 = add nuw nsw i64 %250, 1                                                                                        ;L322<34
 46163|  %252 = icmp samesign ult i64 %251, %247                                                                               ;L322<34
 46164|  %253 = gep %89, i64 11
 46165|  %254 = load i8, ptr %253,
 46166|  %255 = icmp ne i8 %254, 0
 46167|  %256 = select i1 %252, i1 %255, i1 false                                                                              ;L322<34
 46168|  br i1 %256, label %257, label %260                                                                                    ;L322<34
 46169| 
 46170| 257: ; preds = %244
 46171|  %258 = gep %89, i64 12                                                                                                ;L323<34
 46172|  %259 = load i8, ptr %258, , !!8                                                                                       ;L323<34
 46173|  br label %260                                                                                                         ;L322<34
 46174| 
 46175| 260: ; preds = %257, %244, %237
 46176|  %261 = phi i8 [ 0, %237 ], [ %259, %257 ], [ 0, %244 ]                                                                ;L34
 46177|  store i8 %261, ptr %88,                                                                                               ;L34
 46178|  %262 = trunc nuw i8 %261 to i1                                                                                        ;L35
 46179|  %263 = gep %1, i64 6153                                                                                               ;L35
 46180|  %264 = load i8, ptr %263,                                                                                             ;L35
 46181|  %265 = select i1 %262, i8 1, i8 %264                                                                                  ;L35
 46182|  %266 = gep %95, i64 5376                                                                                              ;L35
 46183|  store i8 %265, ptr %266,                                                                                              ;L35
 46184|  store i8 %261, ptr %263,                                                                                              ;L36
 46185|  %267 = gep %5, i64 8                                                                                                  ;L37
 46186|  %268 = load ptr, ptr %267, , !!8, !!8                                                                                 ;L37
 46187|  %269 = gep %268, i64 59                                                                                               ;L37
 46188|  %270 = load i8, ptr %269, , !!8                                                                                       ;L37
 46189|  %271 = trunc nuw i8 %270 to i1                                                                                        ;L37
 46190|  br i1 %271, label %273, label %272                                                                                    ;L37
 46191| 
 46192| 272: ; preds = %297, %260
 46196|  br label %302                                                                                                         ;L29
 46197| 
 46198| 273: ; preds = %260
 46199|     ;; args[0..+8] = ptr %189
 46200|     ;; args[8..+8] = ptr %193
 46201|     ;; args[16..+8] = ptr %266
 46202|     ;; args[24..+8] = ptr %88
 46203|     ;; args[32..+8] = ptr %90
 46204|  %274 = gep %89, i64 8                                                                                                 ;L38
 46205|     ;; args[40..+8] = ptr %274
 46206|     ;; args[48..+8] = ptr %240
 46207|  %275 = gep %89, i64 10                                                                                                ;L38
 46208|     ;; args[56..+8] = ptr %275
 46209|  %276 = gep %89, i64 11                                                                                                ;L38
 46210|     ;; args[64..+8] = ptr %276
 46211|  %277 = gep %89, i64 12                                                                                                ;L38
 46212|     ;; args[72..+8] = ptr %277
 46214|  store ptr %189, ptr %86,                                                                                              ;L38
 46215|  %278 = gep %86, i64 8                                                                                                 ;L38
 46216|  store ptr @core::fmt3num3impjNtB9_7Display3fmt, ptr %278,                                                             ;L38
 46217|  %279 = gep %86, i64 16                                                                                                ;L38
 46218|  store ptr %193, ptr %279,                                                                                             ;L38
 46219|  %280 = gep %86, i64 24                                                                                                ;L38
 46220|  store ptr @gc::simulation6entityNtB5_8PositionNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %280,                        ;L38
 46221|  %281 = gep %86, i64 32                                                                                                ;L38
 46222|  store ptr %266, ptr %281,                                                                                             ;L38
 46223|  %282 = gep %86, i64 40                                                                                                ;L38
 46224|  store ptr @core::fmtbNtB5_7Display3fmt, ptr %282,                                                                     ;L38
 46225|  %283 = gep %86, i64 48                                                                                                ;L38
 46226|  store ptr %88, ptr %283,                                                                                              ;L38
 46227|  %284 = gep %86, i64 56                                                                                                ;L38
 46228|  store ptr @core::fmtbNtB5_7Display3fmt, ptr %284,                                                                     ;L38
 46229|  %285 = gep %86, i64 64                                                                                                ;L38
 46230|  store ptr %90, ptr %285,                                                                                              ;L38
 46231|  %286 = gep %86, i64 72                                                                                                ;L38
 46232|  store ptr @core::fmtbNtB5_7Display3fmt, ptr %286,                                                                     ;L38
 46233|  %287 = gep %86, i64 80                                                                                                ;L38
 46234|  store ptr %274, ptr %287,                                                                                             ;L38
 46235|  %288 = gep %86, i64 88                                                                                                ;L38
 46236|  store ptr @core::fmt3num3imphNtB6_7Display3fmt, ptr %288,                                                             ;L38
 46237|  %289 = gep %86, i64 96                                                                                                ;L38
 46238|  store ptr %240, ptr %289,                                                                                             ;L38
 46239|  %290 = gep %86, i64 104                                                                                               ;L38
 46240|  store ptr @core::fmt3num3imphNtB6_7Display3fmt, ptr %290,                                                             ;L38
 46241|  %291 = gep %86, i64 112                                                                                               ;L38
 46242|  store ptr %275, ptr %291,                                                                                             ;L38
 46243|  %292 = gep %86, i64 120                                                                                               ;L38
 46244|  store ptr @core::fmt3num3imphNtB6_7Display3fmt, ptr %292,                                                             ;L38
 46245|  %293 = gep %86, i64 128                                                                                               ;L38
 46246|  store ptr %276, ptr %293,                                                                                             ;L38
 46247|  %294 = gep %86, i64 136                                                                                               ;L38
 46248|  store ptr @core::fmt3num3imphNtB6_7Display3fmt, ptr %294,                                                             ;L38
 46249|  %295 = gep %86, i64 144                                                                                               ;L38
 46250|  store ptr %277, ptr %295,                                                                                             ;L38
 46251|  %296 = gep %86, i64 152                                                                                               ;L38
 46252|  store ptr @core::fmtbNtB5_7Display3fmt, ptr %296,                                                                     ;L38
 46253|     ;; args[0..+8] = ptr @anon.282069a2ed2ad3a275929b639963fb55.138
 46254|     ;; args[8..+8] = ptr %86
 46255|     ;; self[0..+8] = ptr null
 46256|     ;; self[8..+8] = i64 undef
 46260|  invoke void @_RNvNvNtCs9LexZzt9XJB_5alloc3fmt6format12format_inner(ptr sret([24 x i8]) %87, ptr @anon.282069a2ed2ad3a275929b639963fb55.138, ptr %86)
 46261|  to label %297 unwind label %157                                                                                       ;L659<1275<659<38
 46262| 
 46263| 297: ; preds = %273
 46265|  invoke void @gc::simulation4game5frameNtB4_14DebugFrameData7add_log(ptr %8, ptr %5, ptr %4, ptr %87)
 46266|  to label %272 unwind label %157                                                                                       ;L38
 46267| 
 46268| 298: ; preds = %310, %304, %204
 46269|  %299 = gep %1, i64 1328                                                                                               ;L56
 46270|  %300 = load i64, ptr %299, , !!8                                                                                      ;L56
 46271|  %301 = trunc nuw i64 %300 to i1                                                                                       ;L56
 46272|  br i1 %301, label %318, label %407                                                                                    ;L56
 46273| 
 46274| 302: ; preds = %272, %215
 46275|  %303 = invoke i64 @ai::tower_discipline20v3_survival_incoming(ptr %4, ptr %5, ptr %200)
 46276|  to label %304 unwind label %157                                                                                       ;L50
 46277| 
 46278| 304: ; preds = %302
 46279|  %305 = gep %200, i64 1648                                                                                             ;L50
 46280|  %306 = load i64, ptr %305, , !!8                                                                                      ;L50
 46281|  %307 = icmp ult i64 %303, %306                                                                                        ;L50
 46282|  br i1 %307, label %298, label %308                                                                                    ;L50
 46283| 
 46284| 308: ; preds = %304
 46285|  %309 = invoke zeroext i1 @ai::plan_legacy3old13defense_nexus17nexus_final_stand(ptr %4, ptr %5)
 46286|  to label %310 unwind label %157                                                                                       ;L51
 46287| 
 46288| 310: ; preds = %308
 46289|  br i1 %309, label %298, label %311                                                                                    ;L51
 46290| 
 46291| 311: ; preds = %310
 46293|  call void @llvm.memcpy.p0.p0.i64(ptr %85, ptr %95, i64 5384, i1 false)                                                ;L52
 46295|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway3new(ptr sret([136 x i8]) %84, ptr %5, ptr %4, i64 5)
 46296|  to label %314 unwind label %312                                                                                       ;L52
 46297| 
 46298| 312: ; preds = %311
 46299|  %313 = cleanuppad within none []
 46300|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai15score_parameter14ScoreParameterEBF_(ptr %85) #31 [ "funclet"(token %313) ] ;L52
 46301|  cleanupret from %313 unwind label %157                                                                                ;L52
 46302| 
 46303| 314: ; preds = %311
 46304|  %315 = gep %0, i64 5392                                                                                               ;L52
 46305|  call void @llvm.memcpy.p0.p0.i64(ptr %315, ptr %84, i64 136, i1 false)                                                ;L52
 46307|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %85, i64 5384, i1 false)                                                 ;L52
 46308|  %316 = gep %0, i64 5384                                                                                               ;L52
 46309|  store i64 99999, ptr %316,                                                                                            ;L52
 46310|  %317 = gep %0, i64 5569                                                                                               ;L52
 46311|  store i8 3, ptr %317,                                                                                                 ;L52
 46313|  br label %2439                                                                                                        ;L1
 46314| 
 46315| 318: ; preds = %298
 46316|  %319 = gep %1, i64 1336                                                                                               ;L56
 46317|  %320 = load i64, ptr %319, , !!8                                                                                      ;L56
 46318|     ;; ally_id = i64 %320
 46319|  %321 = gep %1, i64 1344                                                                                               ;L56
 46320|  %322 = load i64, ptr %321, , !!8                                                                                      ;L56
 46321|     ;; expire_tick = i64 %322
 46322|  %323 = load ptr, ptr %196, , !!8, !!8                                                                                 ;L58
 46323|  %324 = gep %196, i64 8                                                                                                ;L58
 46324|  %325 = load ptr, ptr %324, , !!8, !!8                                                                                 ;L58
 46325|  %326 = gep %325, i64 40                                                                                               ;L58
 46326|  %327 = load ptr, ptr %326, , !!8                                                                                      ;L58
 46327|  %328 = invoke i64 %327(ptr %323)
 46328|  to label %329 unwind label %157                                                                                       ;L58
 46329| 
 46330| 329: ; preds = %318
 46331|  %330 = icmp ugt i64 %328, %322                                                                                        ;L58
 46332|  br i1 %330, label %406, label %331                                                                                    ;L58
 46333| 
 46334| 331: ; preds = %329
 46335|  %332 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity7can_ult(ptr %200)
 46336|  to label %333 unwind label %157                                                                                       ;L61
 46337| 
 46338| 333: ; preds = %331
 46339|  br i1 %332, label %334, label %406                                                                                    ;L61
 46340| 
 46341| 334: ; preds = %333
 46342|  %335 = gep %325, i64 496                                                                                              ;L64
 46343|  %336 = load ptr, ptr %335, , !!8                                                                                      ;L64
 46344|  %337 = invoke ptr %336(ptr %323, i64 %320)
 46345|  to label %338 unwind label %157                                                                                       ;L64
 46346| 
 46347| 338: ; preds = %334
 46348|  %339 = icmp eq ptr %337, null                                                                                         ;L64
 46349|  br i1 %339, label %406, label %340                                                                                    ;L64
 46350| 
 46351| 340: ; preds = %338
 46352|     ;; ally = ptr %337
 46353|  %341 = gep %200, i64 1480                                                                                             ;L1701<66
 46354|  %342 = load i64, ptr %341, , !!8                                                                                      ;L1701<66
 46355|  %343 = icmp ugt i64 %342, 4                                                                                           ;L1701<66
 46356|  %344 = gep %200, i64 1336                                                                                             ;L1701<66
 46357|  %345 = select i1 %343, ptr %344, ptr @anon.282069a2ed2ad3a275929b639963fb55.76                                        ;L1701<66
 46358|     ;; ult_effect = ptr %345
 46359|  %346 = gep %345, i64 48                                                                                               ;L67
 46360|  %347 = load i32, ptr %346, , !!8                                                                                      ;L67
 46361|  %348 = icmp eq i32 %347, -1                                                                                           ;L67
 46362|  br i1 %348, label %406, label %349                                                                                    ;L67
 46363| 
 46364| 349: ; preds = %340
 46365|     ;; ult_effect = ptr %345
 46366|     ;; team = !DIArgList(i64 1, i64 %190)
 46367|  %350 = sub nuw nsw i64 1, %190                                                                                        ;L69
 46368|     ;; team = i64 %350
 46369|  %351 = getelementptr [5 x ptr], ptr %197, i64 %350                                                                    ;L1905<69
 46370|  %352 = gep %5, i64 16                                                                                                 ;L70
 46371|  %353 = load ptr, ptr %352, , !!8, !!8                                                                                 ;L70
 46372|     ;; self[0..+8] = ptr %351
 46373|     ;; self[8..+8] = ptr %351
 46374|     ;; self[16..+8] = ptr %323
 46375|     ;; self[24..+8] = ptr %325
 46376|     ;; self[32..+8] = ptr %353
 46377|     ;; self[40..+8] = ptr %4
 46378|     ;; self[48..+8] = ptr %200
 46379|     ;; init = i64 0
 46382|     ;; self[0..+8] = ptr %351
 46383|     ;; iter[0..+8] = ptr %351
 46384|     ;; self[0..+8] = ptr %351
 46385|     ;; self[8..+8] = ptr %351
 46386|     ;; iter[8..+8] = ptr %351
 46387|     ;; self[8..+8] = ptr %351
 46388|     ;; self[16..+8] = ptr %323
 46389|     ;; iter[16..+8] = ptr %323
 46390|     ;; self[16..+8] = ptr %323
 46391|     ;; self[24..+8] = ptr %325
 46392|     ;; iter[24..+8] = ptr %325
 46393|     ;; self[24..+8] = ptr %325
 46394|     ;; self[32..+8] = ptr %353
 46395|     ;; iter[32..+8] = ptr %353
 46396|     ;; self[32..+8] = ptr %353
 46397|     ;; self[40..+8] = ptr %4
 46398|     ;; iter[40..+8] = ptr %4
 46399|     ;; self[40..+8] = ptr %4
 46400|     ;; fold[0..+8] = ptr %323
 46401|     ;; fold[8..+8] = ptr %325
 46402|     ;; fold[16..+8] = ptr %353
 46403|     ;; fold[24..+8] = ptr %4
 46404|     ;; self[0..+8] = ptr %351
 46405|     ;; self[8..+8] = ptr %351
 46406|     ;; init = i64 0
 46407|     ;; f[0..+8] = ptr %323
 46408|     ;; f[8..+8] = ptr %325
 46409|     ;; f[16..+8] = ptr %353
 46410|     ;; f[24..+8] = ptr %4
 46411|     ;; self[0..+8] = ptr %351
 46412|     ;; self[8..+8] = ptr %351
 46413|     ;; init = i64 0
 46414|     ;; rhs = i64 1
 46415|     ;; self[48..+8] = ptr %200
 46416|     ;; iter[48..+8] = ptr %200
 46417|     ;; self[48..+8] = ptr %200
 46418|     ;; f[32..+8] = ptr %200
 46419|     ;; fold[32..+8] = ptr %200
 46420|     ;; acc = i64 0
 46421|     ;; i = i64 0
 46422|     ;; self = i64 0
 46423|     ;; len = i64 5
 46424|  %354 = gep %200, i64 1632
 46425|  %355 = gep %200, i64 1640
 46426|  %356 = getelementptr { { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, [5 x { i64, [2 x i64] }], [5 x { i64, { i8, [23 x i8] } }], [5 x { i8, [7 x i8], i64 }], [5 x i64], [5 x i64], [5 x i64], [5 x { i64, i64 }], i64, i64, i64, i64, i64, [5 x i32], i8, [3 x i8] }, ptr %353, i64 %350
 46427|  br label %357                                                                                                         ;L28<146<128<52<3674<142<72
 46428| 
 46429| 357: ; preds = %389, %349
 46430|  %358 = phi i64 [ 0, %349 ], [ %391, %389 ]                                                                            ;L0<146<128<52<3674<142<72
 46431|  %359 = phi i64 [ 0, %349 ], [ %390, %389 ]                                                                            ;L0<146<128<52<3674<142<72
 46432|     ;; acc = i64 %359
 46433|     ;; self = i64 %358
 46434|     ;; i = i64 %358
 46435|     ;; self = ptr %351
 46436|     ;; count = i64 %358
 46437|  %360 = getelementptr ptr, ptr %351, i64 %358                                                                          ;L656<279<146<128<52<3674<142<72
 46438|  %361 = load ptr, ptr %360, , !!50476, !!8                                                                             ;L279<146<128<52<3674<142<72
 46440|     ;; acc = i64 %359
 46442|  %362 = icmp eq ptr %361, null                                                                                         ;L39<279<146<128<52<3674<142<72
 46443|  br i1 %362, label %389, label %363                                                                                    ;L39<279<146<128<52<3674<142<72
 46444| 
 46445| 363: ; preds = %357
 46446|     ;; x = ptr %361
 46448|     ;; acc = i64 %359
 46449|     ;; elt = ptr %361
 46450|     ;; x = ptr %361
 46456|  %364 = invoke zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %356, ptr %323, ptr %325, ptr %4, ptr %361)
 46457|  to label %365 unwind label %157                                                                                       ;L70<138<88<40<279<146<128<52<3674<142<72
 46458| 
 46459| 365: ; preds = %363
 46460|  br i1 %364, label %366, label %386                                                                                    ;L70<138<88<40<279<146<128<52<3674<142<72
 46461| 
 46462| 366: ; preds = %365
 46463|     ;; self = ptr %361
 46465|  %367 = gep %361, i64 1632                                                                                             ;L2158<71<138<88<40<279<146<128<52<3674<142<72
 46466|  %368 = load i64, ptr %367, , !!50550, !!8                                                                             ;L2158<71<138<88<40<279<146<128<52<3674<142<72
 46467|     ;; x1 = i64 %368
 46468|     ;; self = i64 %368
 46469|  %369 = gep %361, i64 1640                                                                                             ;L2158<71<138<88<40<279<146<128<52<3674<142<72
 46470|  %370 = load i64, ptr %369, , !!50550, !!8                                                                             ;L2158<71<138<88<40<279<146<128<52<3674<142<72
 46471|     ;; y1 = i64 %370
 46472|     ;; self = i64 %370
 46473|  %371 = load i64, ptr %354, , !!50550, !!8                                                                             ;L2158<71<138<88<40<279<146<128<52<3674<142<72
 46474|     ;; x2 = i64 %371
 46475|     ;; other = i64 %371
 46476|  %372 = load i64, ptr %355, , !!50550, !!8                                                                             ;L2158<71<138<88<40<279<146<128<52<3674<142<72
 46477|     ;; y2 = i64 %372
 46478|     ;; other = i64 %372
 46479|  %373 = icmp ult i64 %368, %371                                                                                        ;L3147<7<2158<71<138<88<40<279<146<128<52<3674<142<72
 46480|  %374 = sub nuw i64 %371, %368                                                                                         ;L3147<7<2158<71<138<88<40<279<146<128<52<3674<142<72
 46481|  %375 = sub nuw i64 %368, %371                                                                                         ;L3147<7<2158<71<138<88<40<279<146<128<52<3674<142<72
 46482|  %376 = select i1 %373, i64 %374, i64 %375                                                                             ;L3147<7<2158<71<138<88<40<279<146<128<52<3674<142<72
 46483|     ;; dx = i64 %376
 46484|  %377 = icmp ult i64 %370, %372                                                                                        ;L3147<8<2158<71<138<88<40<279<146<128<52<3674<142<72
 46485|  %378 = sub nuw i64 %372, %370                                                                                         ;L3147<8<2158<71<138<88<40<279<146<128<52<3674<142<72
 46486|  %379 = sub nuw i64 %370, %372                                                                                         ;L3147<8<2158<71<138<88<40<279<146<128<52<3674<142<72
 46487|  %380 = select i1 %377, i64 %378, i64 %379                                                                             ;L3147<8<2158<71<138<88<40<279<146<128<52<3674<142<72
 46488|     ;; dy = i64 %380
 46489|  %381 = mul i64 %376, %376                                                                                             ;L9<2158<71<138<88<40<279<146<128<52<3674<142<72
 46490|  %382 = mul i64 %380, %380                                                                                             ;L9<2158<71<138<88<40<279<146<128<52<3674<142<72
 46491|  %383 = add i64 %382, %381                                                                                             ;L9<2158<71<138<88<40<279<146<128<52<3674<142<72
 46492|  %384 = icmp ult i64 %383, 22500000001                                                                                 ;L71<138<88<40<279<146<128<52<3674<142<72
 46493|  %385 = zext i1 %384 to i64                                                                                            ;L138<88<40<279<146<128<52<3674<142<72
 46494|  br label %386                                                                                                         ;L70<138<88<40<279<146<128<52<3674<142<72
 46495| 
 46496| 386: ; preds = %366, %365
 46497|  %387 = phi i64 [ %385, %366 ], [ 0, %365 ]                                                                            ;L0<138<88<40<279<146<128<52<3674<142<72
 46499|     ;; a = i64 %359
 46500|     ;; b = i64 %387
 46501|  %388 = add i64 %387, %359                                                                                             ;L55<88<40<279<146<128<52<3674<142<72
 46502|  br label %389                                                                                                         ;L42<279<146<128<52<3674<142<72
 46503| 
 46504| 389: ; preds = %386, %357
 46505|  %390 = phi i64 [ %388, %386 ], [ %359, %357 ]                                                                         ;L0<279<146<128<52<3674<142<72
 46506|     ;; acc = i64 %390
 46507|  %391 = add nuw i64 %358, 1                                                                                            ;L971<283<146<128<52<3674<142<72
 46508|     ;; i = i64 %391
 46509|     ;; self = i64 %391
 46510|  %392 = icmp eq i64 %391, 5                                                                                            ;L284<146<128<52<3674<142<72
 46511|  br i1 %392, label %393, label %357                                                                                    ;L284<146<128<52<3674<142<72
 46512| 
 46513| 393: ; preds = %389
 46514|     ;; nearby_enemies = i64 %390
 46515|  %394 = icmp eq i64 %390, 0                                                                                            ;L73
 46516|  br i1 %394, label %395, label %406                                                                                    ;L73
 46517| 
 46518| 395: ; preds = %393
 46519|  %396 = gep %345, i64 40                                                                                               ;L73
 46520|  %397 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %396, ptr %200, ptr %337)
 46521|  to label %398 unwind label %157                                                                                       ;L73
 46522| 
 46523| 398: ; preds = %395
 46524|  br i1 %397, label %399, label %406                                                                                    ;L73
 46525| 
 46526| 399: ; preds = %398
 46528|  call void @llvm.memcpy.p0.p0.i64(ptr %83, ptr %95, i64 5384, i1 false)                                                ;L75
 46530|  invoke void @ai::small_action4castNtB5_14SmallActionUlt3new(ptr sret([24 x i8]) %82, ptr %5, i64 %320)
 46531|  to label %402 unwind label %400                                                                                       ;L75
 46532| 
 46533| 400: ; preds = %399
 46534|  %401 = cleanuppad within none []
 46535|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai15score_parameter14ScoreParameterEBF_(ptr %83) #31 [ "funclet"(token %401) ] ;L75
 46536|  cleanupret from %401 unwind label %157                                                                                ;L75
 46537| 
 46538| 402: ; preds = %399
 46539|  %403 = gep %0, i64 5392                                                                                               ;L75
 46540|  call void @llvm.memcpy.p0.p0.i64(ptr %403, ptr %82, i64 24, i1 false)                                                 ;L75
 46542|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %83, i64 5384, i1 false)                                                 ;L75
 46543|  %404 = gep %0, i64 5384                                                                                               ;L75
 46544|  store i64 99999, ptr %404,                                                                                            ;L75
 46545|  %405 = gep %0, i64 5569                                                                                               ;L75
 46546|  store i8 18, ptr %405,                                                                                                ;L75
 46548|  br label %2439                                                                                                        ;L1
 46549| 
 46550| 406: ; preds = %398, %393, %340, %338, %333, %329
 46551|  store i64 0, ptr %299,                                                                                                ;L88
 46552|  br label %407                                                                                                         ;L56
 46553| 
 46554| 407: ; preds = %406, %298
 46556|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 46557|     ;; order = i8 0
 46558|  %408 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                    ;L3904<741<176<92
 46559|  %409 = icmp eq i8 %408, 0                                                                                             ;L176<92
 46560|  br i1 %409, label %412, label %410                                                                                    ;L176<92
 46561| 
 46562| 410: ; preds = %407
 46563|  %411 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()
 46564|  to label %417 unwind label %157                                                                                       ;L179<92
 46565| 
 46566| 412: ; preds = %417, %407
 46567|  %413 = phi i32 [ %419, %417 ], [ -1, %407 ]
 46568|  %414 = gep %81, i64 16                                                                                                ;L0<92
 46569|  store i32 %413, ptr %414,                                                                                             ;L0<92
 46571|  %415 = gep %1, i64 248                                                                                                ;L93
 46572|  %416 = load i64, ptr %97, , !!8                                                                                       ;L93
 46573|  invoke void @ai::plan_legacy8sub_planNtB4_7SubPlan17action_candidates(ptr sret([32 x i8]) %80, ptr %112, i64 %416, ptr %3, ptr %4, ptr %5, ptr %95, ptr %415, ptr %8)
 46574|  to label %425 unwind label %421                                                                                       ;L93
 46575| 
 46576| 417: ; preds = %410
 46577|  %418 = extractvalue { i64, i32 } %411, 0                                                                              ;L179<92
 46578|  %419 = extractvalue { i64, i32 } %411, 1                                                                              ;L179<92
 46579|  store i64 36, ptr %81,                                                                                                ;L179<92
 46580|  %420 = gep %81, i64 8                                                                                                 ;L179<92
 46581|  store i64 %418, ptr %420,                                                                                             ;L179<92
 46582|  br label %412                                                                                                         ;L180<92
 46583| 
 46584| 421: ; preds = %2508, %2507, %2500, %2499, %2497, %2482, %2481, %2479, %2438, %2437, %2435, %412
 46585|  %422 = phi i1 [ false, %2508 ], [ false, %2507 ], [ false, %2500 ], [ false, %2438 ], [ false, %2482 ], [ true, %412 ], [ false, %2437 ], [ false, %2435 ], [ false, %2481 ], [ false, %2479 ], [ false, %2499 ], [ false, %2497 ] ;L0
 46586|  %423 = phi i1 [ %450, %2508 ], [ %450, %2507 ], [ false, %2500 ], [ false, %2438 ], [ false, %2482 ], [ true, %412 ], [ false, %2437 ], [ false, %2435 ], [ false, %2481 ], [ false, %2479 ], [ false, %2499 ], [ false, %2497 ] ;L0
 46587|  %424 = cleanuppad within none []
 46588|  br i1 %422, label %2510, label %2509                                                                                  ;L416
 46589| 
 46590| 425: ; preds = %412
 46592|  call void @llvm.memcpy.p0.p0.i64(ptr %79, ptr %81, i64 24, i1 false)                                                  ;L94
 46595|  %426 = gep %79, i64 16                                                                                                ;L825<1004<94
 46596|  %427 = load i32, ptr %426, , !!8                                                                                      ;L825<1004<94
 46597|  %428 = icmp eq i32 %427, -1                                                                                           ;L825<1004<94
 46598|  br i1 %428, label %453, label %429                                                                                    ;L825<1004<94
 46599| 
 46600| 429: ; preds = %425
 46604|     ;; self = ptr %79
 46605|     ;; order = i8 0
 46606|     ;; order = i8 0
 46607|     ;; val = i64 1
 46608|     ;; order = i8 0
 46609|     ;; val = i64 1
 46610|     ;; order = i8 0
 46611|  %430 = load i64, ptr %79, , !!8                                                                                       ;L185<825<825<1004<94
 46612|  %431 = icmp ult i64 %430, 132                                                                                         ;L185<825<825<1004<94
 46613|  br i1 %431, label %434, label %432                                                                                    ;L185<825<825<1004<94
 46614| 
 46615| 432: ; preds = %429
 46616|  invoke void @core::panicking18panic_bounds_check(i64 %430, i64 132, ptr @anon.282069a2ed2ad3a275929b639963fb55.182) #32
 46617|  to label %433 unwind label %448                                                                                       ;L185<825<825<1004<94
 46618| 
 46619| 433: ; preds = %432
 46620|  unreachable                                                                                                           ;L185<825<825<1004<94
 46621| 
 46622| 434: ; preds = %429
 46623|     ;; self = !DIArgList(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 %430)
 46624|  %435 = gep %79, i64 8                                                                                                 ;L185<825<825<1004<94
 46625|  %436 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %435)
 46626|  to label %437 unwind label %448                                                                                       ;L185<825<825<1004<94
 46627| 
 46628| 437: ; preds = %434
 46629|  %438 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %430                                 ;L185<825<825<1004<94
 46630|     ;; self = ptr %438
 46631|  %439 = extractvalue { i64, i32 } %436, 0                                                                              ;L185<825<825<1004<94
 46632|  %440 = extractvalue { i64, i32 } %436, 1                                                                              ;L185<825<825<1004<94
 46634|  %441 = mul i64 %439, 1000000000                                                                                       ;L632<185<825<825<1004<94
 46635|  %442 = icmp ult i32 %440, 1000000000                                                                                  ;L49<632<185<825<825<1004<94
 46636|  call void @llvm.assume(i1 %442)                                                                                       ;L49<632<185<825<825<1004<94
 46637|  %443 = zext nneg i32 %440 to i64                                                                                      ;L632<185<825<825<1004<94
 46638|  %444 = add i64 %441, %443                                                                                             ;L632<185<825<825<1004<94
 46639|     ;; val = i64 %444
 46640|     ;; val = i64 %444
 46641|     ;; dst = ptr %438
 46642|  %445 = atomicrmw add ptr %438, i64 %444 monotonic, , !!50639                                                          ;L3937<3162<185<825<825<1004<94
 46643|  %446 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %430                                 ;L186<825<825<1004<94
 46644|     ;; self = ptr %446
 46645|     ;; dst = ptr %446
 46646|  %447 = atomicrmw add ptr %446, i64 1 monotonic, , !!50639                                                             ;L3937<3162<186<825<825<1004<94
 46647|  br label %453                                                                                                         ;L825<1004<94
 46648| 
 46649| 448: ; preds = %2506, %2505, %509, %508, %506, %491, %463, %453, %434, %432
 46650|  %449 = phi i8 [ %513, %2506 ], [ %513, %2505 ], [ %505, %509 ], [ %492, %491 ], [ 1, %463 ], [ 1, %453 ], [ 1, %434 ], [ 1, %432 ], [ %505, %508 ], [ %505, %506 ] ;L0
 46651|  %450 = phi i1 [ %517, %2506 ], [ %517, %2505 ], [ true, %509 ], [ true, %491 ], [ true, %463 ], [ true, %453 ], [ true, %434 ], [ true, %432 ], [ true, %508 ], [ true, %506 ] ;L10
 46652|  %451 = cleanuppad within none []
 46653|  %452 = trunc nuw i8 %449 to i1                                                                                        ;L416
 46654|  br i1 %452, label %2508, label %2507                                                                                  ;L416
 46655| 
 46656| 453: ; preds = %437, %425
 46659|  %454 = load ptr, ptr %196, , !!8, !!8                                                                                 ;L99
 46660|  %455 = gep %196, i64 8                                                                                                ;L99
 46661|  %456 = load ptr, ptr %455, , !!8, !!8                                                                                 ;L99
 46662|  %457 = gep %456, i64 64                                                                                               ;L99
 46663|  %458 = load ptr, ptr %457, , !!8                                                                                      ;L99
 46664|  %459 = invoke { i64, ptr } %458(ptr %454)
 46665|  to label %460 unwind label %448                                                                                       ;L99
 46666| 
 46667| 460: ; preds = %453
 46668|  %461 = extractvalue { i64, ptr } %459, 0                                                                              ;L99
 46669|  %462 = icmp eq i64 %461, 2                                                                                            ;L99
 46670|  br i1 %462, label %463, label %483                                                                                    ;L99
 46671| 
 46672| 463: ; preds = %460
 46674|  %464 = gep %200, i64 1600                                                                                             ;L100
 46675|  %465 = load i64, ptr %464, , !!8                                                                                      ;L100
 46676|  store i64 %465, ptr %77,                                                                                              ;L100
 46679|     ;; self = ptr %80
 46680|     ;; self = ptr %80
 46681|  %466 = load ptr, ptr %80, , !!8, !!8                                                                                  ;L138<2073<102
 46682|     ;; p = ptr %466
 46683|  %467 = gep %80, i64 24                                                                                                ;L2075<102
 46684|  %468 = load i64, ptr %467, , !!8                                                                                      ;L2075<102
 46685|     ;; len = i64 %468
 46686|     ;; count = i64 %468
 46687|     ;; self[0..+8] = ptr %466
 46688|     ;; slice[0..+8] = ptr %466
 46689|     ;; self[8..+8] = i64 %468
 46690|     ;; slice[8..+8] = i64 %468
 46691|     ;; ptr = ptr %466
 46692|     ;; self = ptr %466
 46693|  %469 = gepS %466, i64 %468                                                                                            ;L961<100<1042<102
 46694|     ;; self[0..+8] = ptr %466
 46695|     ;; self[8..+8] = ptr %469
 46696|     ;; self[16..+8] = ptr %6
 46697|     ;; self[24..+8] = ptr %97
 46698|     ;; self[32..+8] = ptr %200
 46699|     ;; self[40..+8] = ptr %3
 46700|     ;; self[48..+8] = ptr %4
 46701|     ;; self[56..+8] = ptr %5
 46702|     ;; self[64..+8] = ptr %119
 46703|     ;; self[72..+8] = ptr %8
 46704|     ;; self[80..+8] = ptr %77
 46705|  store ptr %466, ptr %75,                                                                                              ;L24<3564<115
 46706|  %470 = gep %75, i64 8                                                                                                 ;L24<3564<115
 46707|  store ptr %469, ptr %470,                                                                                             ;L24<3564<115
 46708|  %471 = gep %75, i64 16                                                                                                ;L24<3564<115
 46709|  store ptr %6, ptr %471,                                                                                               ;L24<3564<115
 46710|  %472 = gep %75, i64 24                                                                                                ;L24<3564<115
 46711|  store ptr %97, ptr %472,                                                                                              ;L24<3564<115
 46712|  %473 = gep %75, i64 32                                                                                                ;L24<3564<115
 46713|  store ptr %200, ptr %473,                                                                                             ;L24<3564<115
 46714|  %474 = gep %75, i64 40                                                                                                ;L24<3564<115
 46715|  store ptr %3, ptr %474,                                                                                               ;L24<3564<115
 46716|  %475 = gep %75, i64 48                                                                                                ;L24<3564<115
 46717|  store ptr %4, ptr %475,                                                                                               ;L24<3564<115
 46718|  %476 = gep %75, i64 56                                                                                                ;L24<3564<115
 46719|  store ptr %5, ptr %476,                                                                                               ;L24<3564<115
 46720|  %477 = gep %75, i64 64                                                                                                ;L24<3564<115
 46721|  store ptr %119, ptr %477,                                                                                             ;L24<3564<115
 46722|  %478 = gep %75, i64 72                                                                                                ;L24<3564<115
 46723|  store ptr %8, ptr %478,                                                                                               ;L24<3564<115
 46724|  %479 = gep %75, i64 80                                                                                                ;L24<3564<115
 46725|  store ptr %77, ptr %479,                                                                                              ;L24<3564<115
 46726|  %480 = gep %5, i64 8                                                                                                  ;L115
 46727|  %481 = load ptr, ptr %480, , !!8, !!8                                                                                 ;L115
 46728|  %482 = load ptr, ptr %481, , !!8, !!8                                                                                 ;L115
 46729|  invoke void @core::iter8adapters6cloned6ClonedINtNtB29_6filter6FilterINtNtNtB2d_5slice4iter4IterBU_ENCNvMNtNtNtBY_11plan_legacy7handler7auctionNtB3X_17LegacyPlanHandler16get_small_actions0_0EEEBY_(ptr sret([32 x i8]) %76, ptr %75, ptr %482)
 46730|  to label %484 unwind label %448                                                                                       ;L101
 46731| 
 46732| 483: ; preds = %460
 46733|  call void @llvm.memcpy.p0.p0.i64(ptr %78, ptr %80, i64 32, i1 false)                                                  ;L122
 46734|  br label %512                                                                                                         ;L99
 46735| 
 46736| 484: ; preds = %463
 46738|     ;; self = ptr %76
 46739|     ;; self = ptr %76
 46740|  %485 = gep %76, i64 24                                                                                                ;L1617<1636<116
 46741|  %486 = load i64, ptr %485, , !!8                                                                                      ;L1617<1636<116
 46742|  %487 = icmp eq i64 %486, 0                                                                                            ;L116
 46743|  br i1 %487, label %488, label %490                                                                                    ;L116
 46744| 
 46745| 488: ; preds = %484
 46747|  %489 = load i64, ptr %97, , !!8                                                                                       ;L119
 46748|  invoke void @ai::plan_legacy8sub_planNtB4_7SubPlan15combat_fallback(ptr sret([32 x i8]) %74, ptr %112, i64 %489, ptr %3, ptr %4, ptr %5)
 46749|  to label %494 unwind label %491                                                                                       ;L119
 46750| 
 46751| 490: ; preds = %484
 46752|  call void @llvm.memcpy.p0.p0.i64(ptr %78, ptr %76, i64 32, i1 false)                                                  ;L121
 46753|  br label %510                                                                                                         ;L122
 46754| 
 46755| 491: ; preds = %502, %501, %499, %488
 46756|  %492 = phi i8 [ 0, %502 ], [ 1, %488 ], [ 0, %501 ], [ 0, %499 ]                                                      ;L0
 46757|  %493 = cleanuppad within none []
 46758|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %76) #31 [ "funclet"(token %493) ] ;L122
 46759|  cleanupret from %493 unwind label %448                                                                                ;L122
 46760| 
 46761| 494: ; preds = %488
 46762|     ;; self = ptr %74
 46763|     ;; self = ptr %74
 46764|  %495 = gep %74, i64 24                                                                                                ;L1617<1636<120
 46765|  %496 = load i64, ptr %495, , !!8                                                                                      ;L1617<1636<120
 46766|  %497 = icmp eq i64 %496, 0                                                                                            ;L120
 46767|  br i1 %497, label %498, label %503                                                                                    ;L120
 46768| 
 46769| 498: ; preds = %494
 46770|  call void @llvm.memcpy.p0.p0.i64(ptr %78, ptr %80, i64 32, i1 false)                                                  ;L120
 46772|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %74)
 46773|  to label %502 unwind label %499                                                                                       ;L825<121
 46774| 
 46775| 499: ; preds = %498
 46776|  %500 = cleanuppad within none []
 46778|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %74) [ "funclet"(token %500) ]
 46779|  to label %501 unwind label %491                                                                                       ;L825<825<121
 46780| 
 46781| 501: ; preds = %499
 46782|  cleanupret from %500 unwind label %491
 46783| 
 46784| 502: ; preds = %498
 46786|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %74)
 46787|  to label %504 unwind label %491                                                                                       ;L825<825<121
 46788| 
 46789| 503: ; preds = %494
 46790|  call void @llvm.memcpy.p0.p0.i64(ptr %78, ptr %74, i64 32, i1 false)                                                  ;L120
 46791|  br label %504                                                                                                         ;L121
 46792| 
 46793| 504: ; preds = %503, %502
 46794|  %505 = phi i8 [ 1, %503 ], [ 0, %502 ]                                                                                ;L0
 46797|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %76)
 46798|  to label %509 unwind label %506                                                                                       ;L825<122
 46799| 
 46800| 506: ; preds = %504
 46801|  %507 = cleanuppad within none []
 46803|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %76) [ "funclet"(token %507) ]
 46804|  to label %508 unwind label %448                                                                                       ;L825<825<122
 46805| 
 46806| 508: ; preds = %506
 46807|  cleanupret from %507 unwind label %448
 46808| 
 46809| 509: ; preds = %504
 46811|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %76)
 46812|  to label %510 unwind label %448                                                                                       ;L825<825<122
 46813| 
 46814| 510: ; preds = %509, %490
 46815|  %511 = phi i8 [ 1, %490 ], [ %505, %509 ]                                                                             ;L0
 46818|  br label %512                                                                                                         ;L99
 46819| 
 46820| 512: ; preds = %510, %483
 46821|  %513 = phi i8 [ %511, %510 ], [ 0, %483 ]                                                                             ;L0
 46822|  %514 = invoke zeroext i1 @ai::plan_legacy3old13defense_nexus16nexus_last_stand(ptr %4, ptr %5)
 46823|  to label %519 unwind label %515                                                                                       ;L125
 46824| 
 46825| 515: ; preds = %2504, %2503, %628, %621, %592, %580, %519, %512
 46826|  %516 = phi i1 [ false, %2504 ], [ false, %2503 ], [ true, %628 ], [ true, %519 ], [ true, %621 ], [ true, %592 ], [ true, %580 ], [ true, %512 ] ;L0
 46827|  %517 = phi i1 [ %636, %2504 ], [ %636, %2503 ], [ true, %628 ], [ true, %519 ], [ true, %621 ], [ true, %592 ], [ true, %580 ], [ true, %512 ] ;L10
 46828|  %518 = cleanuppad within none []
 46829|  br i1 %516, label %2506, label %2505                                                                                  ;L416
 46830| 
 46831| 519: ; preds = %512
 46832|  %520 = gep %1, i64 6148                                                                                               ;L125
 46833|  %521 = zext i1 %514 to i8                                                                                             ;L125
 46834|  store i8 %521, ptr %520,                                                                                              ;L125
 46835|  %522 = invoke zeroext i1 @ai::plan_legacy3old13defense_nexus17nexus_final_stand(ptr %4, ptr %5)
 46836|  to label %523 unwind label %515                                                                                       ;L126
 46837| 
 46838| 523: ; preds = %519
 46839|  %524 = gep %1, i64 6149                                                                                               ;L126
 46840|  %525 = zext i1 %522 to i8                                                                                             ;L126
 46841|  store i8 %525, ptr %524,                                                                                              ;L126
 46842|  %526 = gep %1, i64 1512                                                                                               ;L127
 46843|  %527 = load i64, ptr %526, , !!8                                                                                      ;L127
 46844|  %528 = icmp ne i64 %527, 6                                                                                            ;L127
 46845|  call void @llvm.assume(i1 %528)                                                                                       ;L127
 46846|  %529 = icmp eq i64 %527, 9                                                                                            ;L127
 46847|  br i1 %529, label %530, label %540                                                                                    ;L127
 46848| 
 46849| 530: ; preds = %523
 46850|     ;; b = ptr %1
 46851|  %531 = gep %1, i64 1608                                                                                               ;L128
 46852|  %532 = load i64, ptr %531, , !!8                                                                                      ;L128
 46853|  switch i64 %532, label %533 [
 46854|  i64 0, label %540
 46855|  i64 1, label %534
 46856|  i64 2, label %535
 46857|  i64 3, label %536
 46858|  i64 4, label %537
 46859|  i64 5, label %538
 46860|  i64 6, label %538
 46861|  i64 7, label %539
 46862|  ]                                                                                                                     ;L128
 46863| 
 46864| 533: ; preds = %530
 46865|  unreachable
 46866| 
 46867| 534: ; preds = %530
 46868|  br label %540                                                                                                         ;L134
 46869| 
 46870| 535: ; preds = %530
 46871|  br label %540                                                                                                         ;L130
 46872| 
 46873| 536: ; preds = %530
 46874|  br label %540                                                                                                         ;L131
 46875| 
 46876| 537: ; preds = %530
 46877|  br label %540                                                                                                         ;L132
 46878| 
 46879| 538: ; preds = %530, %530
 46880|  br label %540                                                                                                         ;L133
 46881| 
 46882| 539: ; preds = %530
 46883|  br label %540                                                                                                         ;L135
 46884| 
 46885| 540: ; preds = %539, %538, %537, %536, %535, %534, %530, %523
 46886|  %541 = phi i8 [ 7, %523 ], [ 5, %534 ], [ 1, %535 ], [ 2, %536 ], [ 3, %537 ], [ 4, %538 ], [ 6, %539 ], [ 0, %530 ]  ;L0
 46887|  %542 = gep %1, i64 6163                                                                                               ;L127
 46888|  store i8 %541, ptr %542,                                                                                              ;L127
 46889|  %543 = load i64, ptr %112, , !!8                                                                                      ;L139
 46890|  %544 = icmp ne i64 %543, 8                                                                                            ;L139
 46891|  call void @llvm.assume(i1 %544)                                                                                       ;L139
 46892|  %545 = add nsw i64 %543, -2                                                                                           ;L139
 46893|  %546 = icmp samesign ugt i64 %543, 1                                                                                  ;L139
 46894|  %547 = select i1 %546, i64 %545, i64 6                                                                                ;L139
 46895|  switch i64 %547, label %567 [
 46896|  i64 5, label %548
 46897|  i64 15, label %553
 46898|  ]                                                                                                                     ;L139
 46899| 
 46900| 548: ; preds = %540
 46901|     ;; p = ptr %1
 46902|  %549 = gep %1, i64 1949                                                                                               ;L141
 46903|  %550 = load i8, ptr %549, , !!8                                                                                       ;L141
 46904|  %551 = add i8 %550, -1                                                                                                ;L141
 46905|  %552 = icmp ult i8 %551, 3                                                                                            ;L141
 46906|  br i1 %552, label %565, label %560                                                                                    ;L141
 46907| 
 46908| 553: ; preds = %540
 46909|     ;; p = ptr %1
 46910|  %554 = gep %1, i64 1920                                                                                               ;L140
 46911|  %555 = load i8, ptr %554, , !!8                                                                                       ;L140
 46912|  %556 = icmp eq i8 %555, 2                                                                                             ;L140
 46913|  %557 = select i1 %556, i8 2, i8 3                                                                                     ;L140
 46914|  %558 = icmp eq i8 %555, 1                                                                                             ;L140
 46915|  %559 = select i1 %558, i8 1, i8 %557                                                                                  ;L140
 46916|  br label %567                                                                                                         ;L140
 46917| 
 46918| 560: ; preds = %548
 46919|     ;; self = ptr %78
 46920|  %561 = gep %78, i64 24                                                                                                ;L1617<141
 46921|  %562 = load i64, ptr %561, , !!8                                                                                      ;L1617<141
 46922|  %563 = icmp eq i64 %562, 1                                                                                            ;L141
 46923|  %564 = select i1 %563, i8 7, i8 8                                                                                     ;L141
 46924|  br label %567                                                                                                         ;L141
 46925| 
 46926| 565: ; preds = %548
 46927|  %566 = add nuw nsw i8 %550, 3                                                                                         ;L141
 46928|  br label %567                                                                                                         ;L141
 46929| 
 46930| 567: ; preds = %565, %560, %553, %540
 46931|  %568 = phi i8 [ 0, %540 ], [ %566, %565 ], [ %559, %553 ], [ %564, %560 ]                                             ;L0
 46932|  %569 = gep %1, i64 6162                                                                                               ;L139
 46933|  store i8 %568, ptr %569,                                                                                              ;L139
 46934|  %570 = icmp eq i64 %543, 7                                                                                            ;L146
 46935|     ;; self = ptr %78
 46936|  %571 = gep %78, i64 24
 46937|  %572 = load i64, ptr %571,
 46938|  %573 = icmp eq i64 %572, 1                                                                                            ;L146
 46939|  %574 = select i1 %570, i1 %573, i1 false                                                                              ;L146
 46940|  br i1 %574, label %582, label %580                                                                                    ;L146
 46941| 
 46942| 575: ; preds = %591, %582, %582, %582
 46943|  %576 = phi i64 [ 5440, %591 ], [ 5448, %582 ], [ 5448, %582 ], [ 5448, %582 ]
 46944|  %577 = gep %1, i64 %576                                                                                               ;L0
 46945|  %578 = load i64, ptr %577, , !!8                                                                                      ;L0
 46946|  %579 = add i64 %578, 1                                                                                                ;L0
 46947|  store i64 %579, ptr %577,                                                                                             ;L0
 46948|  br label %580                                                                                                         ;L154
 46949| 
 46950| 580: ; preds = %582, %582, %582, %582, %582, %582, %582, %582, %582, %582, %575, %567
 46951|  %581 = invoke i64 @gc::simulation5state6playerNtB4_16AthleteParameter14judge_accuracy(ptr %151)
 46952|  to label %592 unwind label %515                                                                                       ;L154
 46953| 
 46954| 582: ; preds = %567
 46955|     ;; self = ptr %78
 46956|     ;; self = ptr %78
 46957|     ;; self = ptr %78
 46958|  %583 = load ptr, ptr %78, , !!8, !!8                                                                                  ;L138<2073<2054<147
 46959|     ;; self = ptr %583
 46960|  %584 = gep %583, i64 177                                                                                              ;L309<147
 46961|  %585 = load i8, ptr %584, , !!50902, !!8                                                                              ;L309<147
 46962|  %586 = icmp ne i8 %585, 10                                                                                            ;L309<147
 46963|  call void @llvm.assume(i1 %586)                                                                                       ;L309<147
 46964|  %587 = add nsw i8 %585, -3                                                                                            ;L309<147
 46965|  %588 = icmp samesign ugt i8 %585, 2                                                                                   ;L309<147
 46966|  %589 = select i1 %588, i8 %587, i8 7                                                                                  ;L309<147
 46967|  switch i8 %589, label %590 [
 46968|  i8 0, label %575
 46969|  i8 1, label %575
 46970|  i8 2, label %580
 46971|  i8 3, label %580
 46972|  i8 4, label %591
 46973|  i8 5, label %575
 46974|  i8 6, label %580
 46975|  i8 7, label %591
 46976|  i8 8, label %591
 46977|  i8 9, label %591
 46978|  i8 10, label %580
 46979|  i8 11, label %580
 46980|  i8 12, label %580
 46981|  i8 13, label %580
 46982|  i8 14, label %580
 46983|  i8 15, label %580
 46984|  i8 16, label %580
 46985|  ]                                                                                                                     ;L309<147
 46986| 
 46987| 590: ; preds = %582
 46988|  unreachable                                                                                                           ;L309<147
 46989| 
 46990| 591: ; preds = %582, %582, %582, %582
 46991|  br label %575                                                                                                         ;L148
 46992| 
 46993| 592: ; preds = %580
 46994|     ;; judge_accuracy = i64 %581
 46995|  %593 = invoke { i64, ptr } %458(ptr %454)
 46996|  to label %594 unwind label %515                                                                                       ;L157
 46997| 
 46998| 594: ; preds = %592
 46999|  %595 = extractvalue { i64, ptr } %593, 0                                                                              ;L157
 47000|  %596 = icmp eq i64 %595, 2                                                                                            ;L157
 47001|  %597 = sub i64 1000, %581                                                                                             ;L157
 47002|  %598 = lshr i64 %597, 1                                                                                               ;L157
 47003|  %599 = select i1 %596, i64 0, i64 %598                                                                                ;L157
 47004|     ;; spread = i64 %599
 47005|  %600 = sub nsw i64 1000, %599                                                                                         ;L158
 47006|     ;; range_min = i64 %600
 47007|     ;; start = i64 %600
 47008|  %601 = add nuw i64 %599, 1000                                                                                         ;L159
 47009|     ;; range_max = i64 %601
 47010|     ;; end = i64 %601
 47011|     ;; v = ptr %1
 47012|  %602 = load i64, ptr %526, , !!8                                                                                      ;L1329<165
 47013|  %603 = icmp ne i64 %602, 6                                                                                            ;L1329<165
 47014|  call void @llvm.assume(i1 %603)                                                                                       ;L1329<165
 47015|  %604 = add nsw i64 %602, -2                                                                                           ;L1329<165
 47016|  %605 = icmp samesign ugt i64 %602, 1                                                                                  ;L1329<165
 47017|  %606 = select i1 %605, i64 %604, i64 4                                                                                ;L1329<165
 47018|     ;; plan_disc = i64 %606
 47019|     ;; self = ptr %1
 47020|     ;; self = ptr %1
 47023|  %607 = gep %1, i64 1376                                                                                               ;L2439<264<166
 47024|  %608 = load i64, ptr %607, , !!8                                                                                      ;L2439<264<166
 47025|  %609 = gep %1, i64 1384                                                                                               ;L2439<264<166
 47026|  %610 = trunc nuw i64 %608 to i1                                                                                       ;L2439<264<166
 47027|  %611 = load i64, ptr %609,
 47028|  %612 = icmp eq i64 %611, %606
 47029|  %613 = select i1 %610, i1 %612, i1 false                                                                              ;L2439<264<166
 47030|  br i1 %613, label %625, label %614                                                                                    ;L2439<264<166
 47031| 
 47032| 614: ; preds = %594
 47033|  store i64 1, ptr %607,                                                                                                ;L167
 47034|  store i64 %606, ptr %609,                                                                                             ;L167
 47035|     ;; iter[0..+8] = ptr %1
 47036|     ;; iter[8..+8] = ptr %1
 47037|  %615 = gep %73, i64 8
 47038|  %616 = gep %73, i64 16
 47039|  br label %617                                                                                                         ;L168
 47040| 
 47041| 617: ; preds = %623, %614
 47042|  %618 = phi i64 [ 6048, %614 ], [ %624, %623 ]
 47043|  %619 = gep %1, i64 %618                                                                                               ;L1714<180<168
 47044|     ;; iter[0..+8] = ptr %619
 47045|     ;; self = ptr undef
 47046|     ;; ptr = ptr %619
 47047|     ;; self = ptr %619
 47048|     ;; end_or_len = ptr %1
 47051|  %620 = icmp eq i64 %618, 6136                                                                                         ;L1714<180<168
 47052|  br i1 %620, label %625, label %621                                                                                    ;L180<168
 47053| 
 47054| 621: ; preds = %617
 47055|     ;; iter[0..+8] = !DIArgList(ptr %1, i64 %618)
 47056|     ;; r = ptr %619
 47058|  store i64 %600, ptr %73,                                                                                              ;L391<169
 47059|  store i64 %601, ptr %615,                                                                                             ;L391<169
 47060|  store i8 0, ptr %616,                                                                                                 ;L391<169
 47061|  %622 = invoke i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %3, ptr %73)
 47062|  to label %623 unwind label %515                                                                                       ;L169
 47063| 
 47064| 623: ; preds = %621
 47065|  %624 = add nuw nsw i64 %618, 8                                                                                        ;L656<185<168
 47066|     ;; iter[0..+8] = !DIArgList(ptr %1, i64 %624)
 47068|  store i64 %622, ptr %619,                                                                                             ;L169
 47069|  br label %617                                                                                                         ;L168
 47070| 
 47071| 625: ; preds = %617, %594
 47073|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 47074|     ;; order = i8 0
 47075|  %626 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                    ;L3904<741<176<174
 47076|  %627 = icmp eq i8 %626, 0                                                                                             ;L176<174
 47077|  br i1 %627, label %638, label %628                                                                                    ;L176<174
 47078| 
 47079| 628: ; preds = %625
 47080|  %629 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()
 47081|  to label %630 unwind label %515                                                                                       ;L179<174
 47082| 
 47083| 630: ; preds = %628
 47084|  %631 = extractvalue { i64, i32 } %629, 0                                                                              ;L179<174
 47085|  %632 = extractvalue { i64, i32 } %629, 1                                                                              ;L179<174
 47086|  store i64 37, ptr %72,                                                                                                ;L179<174
 47087|  %633 = gep %72, i64 8                                                                                                 ;L179<174
 47088|  store i64 %631, ptr %633,                                                                                             ;L179<174
 47089|  br label %638                                                                                                         ;L180<174
 47090| 
 47091| 634: ; preds = %2502, %2501, %2494, %2493, %2491, %2476, %2475, %2473, %2432, %2431, %2429, %638
 47092|  %635 = phi i1 [ false, %2502 ], [ false, %2501 ], [ false, %2494 ], [ false, %2432 ], [ false, %2476 ], [ true, %638 ], [ false, %2431 ], [ false, %2429 ], [ false, %2475 ], [ false, %2473 ], [ false, %2493 ], [ false, %2491 ] ;L0
 47093|  %636 = phi i1 [ %681, %2502 ], [ %681, %2501 ], [ false, %2494 ], [ false, %2432 ], [ false, %2476 ], [ true, %638 ], [ false, %2431 ], [ false, %2429 ], [ false, %2475 ], [ false, %2473 ], [ false, %2493 ], [ false, %2491 ] ;L0
 47094|  %637 = cleanuppad within none []
 47095|  br i1 %635, label %2504, label %2503                                                                                  ;L416
 47096| 
 47097| 638: ; preds = %630, %625
 47098|  %639 = phi i32 [ %632, %630 ], [ -1, %625 ]
 47099|  %640 = gep %72, i64 16                                                                                                ;L0<174
 47100|  store i32 %639, ptr %640,                                                                                             ;L0<174
 47102|  %641 = gep %1, i64 6048                                                                                               ;L175
 47103|  call void @llvm.memcpy.p0.p0.i64(ptr %71, ptr %641, i64 88, i1 false)                                                 ;L175
 47106|  %642 = load ptr, ptr %78, , !!8, !!8                                                                                  ;L177
 47108|     ;; begin = ptr %642
 47109|     ;; self = ptr %642
 47110|     ;; count = i64 %572
 47111|  %643 = gepS %642, i64 %572                                                                                            ;L961<2119<177
 47112|     ;; self[0..+8] = ptr %642
 47113|     ;; self[8..+8] = ptr %643
 47114|     ;; f[0..+8] = ptr %112
 47115|     ;; f[8..+8] = ptr %97
 47116|     ;; f[16..+8] = ptr %95
 47117|     ;; f[24..+8] = ptr %3
 47118|     ;; f[32..+8] = ptr %4
 47119|     ;; f[40..+8] = ptr %5
 47120|     ;; f[48..+8] = ptr %8
 47121|     ;; f[56..+8] = ptr %71
 47122|  %644 = gep %69, i64 64                                                                                                ;L69<836<177
 47123|  store ptr %642, ptr %644,                                                                                             ;L69<836<177
 47124|  %645 = gep %69, i64 72                                                                                                ;L69<836<177
 47125|  store ptr %643, ptr %645,                                                                                             ;L69<836<177
 47126|  store ptr %112, ptr %69,                                                                                              ;L69<836<177
 47127|  %646 = gep %69, i64 8                                                                                                 ;L69<836<177
 47128|  store ptr %97, ptr %646,                                                                                              ;L69<836<177
 47129|  %647 = gep %69, i64 16                                                                                                ;L69<836<177
 47130|  store ptr %95, ptr %647,                                                                                              ;L69<836<177
 47131|  %648 = gep %69, i64 24                                                                                                ;L69<836<177
 47132|  store ptr %3, ptr %648,                                                                                               ;L69<836<177
 47133|  %649 = gep %69, i64 32                                                                                                ;L69<836<177
 47134|  store ptr %4, ptr %649,                                                                                               ;L69<836<177
 47135|  %650 = gep %69, i64 40                                                                                                ;L69<836<177
 47136|  store ptr %5, ptr %650,                                                                                               ;L69<836<177
 47137|  %651 = gep %69, i64 48                                                                                                ;L69<836<177
 47138|  store ptr %8, ptr %651,                                                                                               ;L69<836<177
 47139|  %652 = gep %69, i64 56                                                                                                ;L69<836<177
 47140|  store ptr %71, ptr %652,                                                                                              ;L69<836<177
 47141|  %653 = gep %5, i64 8                                                                                                  ;L185
 47142|  %654 = load ptr, ptr %653, , !!8, !!8                                                                                 ;L185
 47143|  %655 = load ptr, ptr %654, , !!8, !!8                                                                                 ;L185
 47144|  invoke void @core::iter8adapters3map3MapINtB3_8IntoIterBW_ENCNvMNtNtNtB10_11plan_legacy7handler7auctionNtB3l_17LegacyPlanHandler16get_small_actions1_0EEB10_(ptr sret([32 x i8]) %70, ptr %69, ptr %655)
 47145|  to label %656 unwind label %634                                                                                       ;L176
 47146| 
 47147| 656: ; preds = %638
 47150|  call void @llvm.memcpy.p0.p0.i64(ptr %68, ptr %72, i64 24, i1 false)                                                  ;L186
 47153|  %657 = gep %68, i64 16                                                                                                ;L825<1004<186
 47154|  %658 = load i32, ptr %657, , !!8                                                                                      ;L825<1004<186
 47155|  %659 = icmp eq i32 %658, -1                                                                                           ;L825<1004<186
 47156|  br i1 %659, label %684, label %660                                                                                    ;L825<1004<186
 47157| 
 47158| 660: ; preds = %656
 47162|     ;; self = ptr %68
 47163|     ;; order = i8 0
 47164|     ;; order = i8 0
 47165|     ;; val = i64 1
 47166|     ;; order = i8 0
 47167|     ;; val = i64 1
 47168|     ;; order = i8 0
 47169|  %661 = load i64, ptr %68, , !!8                                                                                       ;L185<825<825<1004<186
 47170|  %662 = icmp ult i64 %661, 132                                                                                         ;L185<825<825<1004<186
 47171|  br i1 %662, label %665, label %663                                                                                    ;L185<825<825<1004<186
 47172| 
 47173| 663: ; preds = %660
 47174|  invoke void @core::panicking18panic_bounds_check(i64 %661, i64 132, ptr @anon.282069a2ed2ad3a275929b639963fb55.182) #32
 47175|  to label %664 unwind label %679                                                                                       ;L185<825<825<1004<186
 47176| 
 47177| 664: ; preds = %663
 47178|  unreachable                                                                                                           ;L185<825<825<1004<186
 47179| 
 47180| 665: ; preds = %660
 47181|     ;; self = !DIArgList(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 %661)
 47182|  %666 = gep %68, i64 8                                                                                                 ;L185<825<825<1004<186
 47183|  %667 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %666)
 47184|  to label %668 unwind label %679                                                                                       ;L185<825<825<1004<186
 47185| 
 47186| 668: ; preds = %665
 47187|  %669 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %661                                 ;L185<825<825<1004<186
 47188|     ;; self = ptr %669
 47189|  %670 = extractvalue { i64, i32 } %667, 0                                                                              ;L185<825<825<1004<186
 47190|  %671 = extractvalue { i64, i32 } %667, 1                                                                              ;L185<825<825<1004<186
 47192|  %672 = mul i64 %670, 1000000000                                                                                       ;L632<185<825<825<1004<186
 47193|  %673 = icmp ult i32 %671, 1000000000                                                                                  ;L49<632<185<825<825<1004<186
 47194|  call void @llvm.assume(i1 %673)                                                                                       ;L49<632<185<825<825<1004<186
 47195|  %674 = zext nneg i32 %671 to i64                                                                                      ;L632<185<825<825<1004<186
 47196|  %675 = add i64 %672, %674                                                                                             ;L632<185<825<825<1004<186
 47197|     ;; val = i64 %675
 47198|     ;; val = i64 %675
 47199|     ;; dst = ptr %669
 47200|  %676 = atomicrmw add ptr %669, i64 %675 monotonic, , !!51069                                                          ;L3937<3162<185<825<825<1004<186
 47201|  %677 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %661                                 ;L186<825<825<1004<186
 47202|     ;; self = ptr %677
 47203|     ;; dst = ptr %677
 47204|  %678 = atomicrmw add ptr %677, i64 1 monotonic, , !!51069                                                             ;L3937<3162<186<825<825<1004<186
 47205|  br label %684                                                                                                         ;L825<1004<186
 47206| 
 47207| 679: ; preds = %2467, %2466, %2464, %2423, %2422, %2420, %1935, %1934, %1932, %1381, %1365, %1364, %1362, %1350, %1330, %1300, %1294, %1292, %1284, %1223, %1222, %1220, %1213, %1207, %1171, %1169, %1150, %1149, %1140, %1103, %989, %984, %971, %945, %939, %937, %920, %770, %740, %665, %663
 47208|  %680 = phi i8 [ 1, %770 ], [ 1, %1149 ], [ %1370, %1935 ], [ %1370, %1381 ], [ %1370, %2423 ], [ %1370, %2467 ], [ 0, %1365 ], [ 1, %1350 ], [ 1, %1330 ], [ 1, %1300 ], [ 1, %1292 ], [ 1, %1284 ], [ 1, %1220 ], [ 1, %1223 ], [ 1, %1207 ], [ 1, %1169 ], [ 1, %1150 ], [ 1, %1140 ], [ %1370, %2464 ], [ %1370, %2466 ], [ 1, %1103 ], [ 1, %665 ], [ 1, %984 ], [ 1, %989 ], [ 1, %945 ], [ 1, %939 ], [ 1, %937 ], [ 1, %920 ], [ 1, %740 ], [ 1, %663 ], [ 1, %971 ], [ %1370, %2420 ], [ 1, %1171 ], [ 1, %1222 ], [ 1, %1213 ], [ 1, %1294 ], [ 0, %1364 ], [ 0, %1362 ], [ %1370, %1934 ], [ %1370, %1932 ], [ %1370, %2422 ] ;L0
 47209|  %681 = phi i1 [ true, %770 ], [ true, %1149 ], [ false, %1935 ], [ %1382, %1381 ], [ false, %2423 ], [ false, %2467 ], [ true, %1365 ], [ true, %1350 ], [ true, %1330 ], [ true, %1300 ], [ true, %1292 ], [ true, %1284 ], [ true, %1220 ], [ true, %1223 ], [ true, %1207 ], [ true, %1169 ], [ true, %1150 ], [ true, %1140 ], [ false, %2464 ], [ false, %2466 ], [ true, %1103 ], [ true, %665 ], [ true, %984 ], [ true, %989 ], [ true, %945 ], [ true, %939 ], [ true, %937 ], [ true, %920 ], [ true, %740 ], [ true, %663 ], [ true, %971 ], [ false, %2420 ], [ true, %1171 ], [ true, %1222 ], [ true, %1213 ], [ true, %1294 ], [ true, %1364 ], [ true, %1362 ], [ false, %1934 ], [ false, %1932 ], [ false, %2422 ] ;L0
 47210|  %682 = cleanuppad within none []
 47211|  %683 = trunc nuw i8 %680 to i1                                                                                        ;L416
 47212|  br i1 %683, label %2502, label %2501                                                                                  ;L416
 47213| 
 47214| 684: ; preds = %668, %656
 47216|     ;; self = ptr %654
 47217|  %685 = gep %654, i64 57                                                                                               ;L15<189
 47218|  %686 = load i8, ptr %685, , !!8                                                                                       ;L15<189
 47219|  %687 = icmp eq i8 %686, 0                                                                                             ;L15<189
 47220|  br i1 %687, label %688, label %692                                                                                    ;L189
 47221| 
 47222| 688: ; preds = %1172, %1016, %1013, %1012, %710, %684
 47223|  %689 = gep %654, i64 59                                                                                               ;L254
 47224|  %690 = load i8, ptr %689, , !!8                                                                                       ;L254
 47225|  %691 = trunc nuw i8 %690 to i1                                                                                        ;L254
 47226|  br i1 %691, label %1187, label %1177                                                                                  ;L254
 47227| 
 47228| 692: ; preds = %684
 47229|     ;; self = ptr %70
 47230|     ;; self = ptr %70
 47231|  %693 = load ptr, ptr %70, , !!8, !!8                                                                                  ;L138<2073<190
 47232|     ;; p = ptr %693
 47233|  %694 = gep %70, i64 24                                                                                                ;L2075<190
 47234|  %695 = load i64, ptr %694, , !!8                                                                                      ;L2075<190
 47235|     ;; len = i64 %695
 47236|     ;; count = i64 %695
 47237|     ;; self[0..+8] = ptr %693
 47238|     ;; slice[0..+8] = ptr %693
 47239|     ;; self[8..+8] = i64 %695
 47240|     ;; slice[8..+8] = i64 %695
 47241|     ;; ptr = ptr %693
 47242|     ;; self = ptr %693
 47243|  %696 = getelementptr { i64, { [177 x i8], i8, [6 x i8] } }, ptr %693, i64 %695                                        ;L961<100<1042<190
 47244|     ;; self = ptr undef
 47245|     ;; self = ptr undef
 47247|     ;; count = i64 1
 47248|  br label %697                                                                                                         ;L331<190
 47249| 
 47250| 697: ; preds = %700, %692
 47251|  %698 = phi ptr [ %701, %700 ], [ %693, %692 ]
 47252|     ;; ptr = ptr %698
 47253|     ;; self = ptr %698
 47254|     ;; end_or_len = ptr %696
 47257|  %699 = icmp eq ptr %698, %696                                                                                         ;L1714<180<331<190
 47258|  br i1 %699, label %710, label %700                                                                                    ;L180<331<190
 47259| 
 47260| 700: ; preds = %697
 47261|  %701 = gep %698, i64 192                                                                                              ;L656<185<331<190
 47262|     ;; x = ptr %698
 47263|  %702 = gep %698, i64 137                                                                                              ;L332<190
 47264|  %703 = load i8, ptr %702, , !!51230                                                                                   ;L332<190
 47265|  %704 = gep %698, i64 185                                                                                              ;L332<190
 47266|  %705 = load i8, ptr %704, , !!51230, !!8                                                                              ;L332<190
 47271|  %706 = icmp ne i8 %705, 10                                                                                            ;L438<190<332<190
 47272|  call void @llvm.assume(i1 %706)                                                                                       ;L438<190<332<190
 47273|  %707 = icmp eq i8 %705, 3                                                                                             ;L438<190<332<190
 47274|  %708 = trunc nuw i8 %703 to i1                                                                                        ;L438<190<332<190
 47275|  %709 = select i1 %707, i1 %708, i1 false                                                                              ;L438<190<332<190
 47276|  br i1 %709, label %718, label %697                                                                                    ;L332<190
 47277| 
 47278| 710: ; preds = %940, %697
 47279|     ;; self = ptr %654
 47280|  %711 = load i64, ptr %526, , !!8                                                                                      ;L215
 47281|  %712 = icmp ne i64 %711, 6                                                                                            ;L215
 47282|  call void @llvm.assume(i1 %712)                                                                                       ;L215
 47283|  %713 = icmp eq i64 %711, 9                                                                                            ;L215
 47284|     ;; battle_plan = ptr %1
 47285|  %714 = gep %1, i64 1608
 47286|  %715 = load i64, ptr %714,
 47287|  %716 = icmp eq i64 %715, 4
 47288|  %717 = select i1 %713, i1 %716, i1 false                                                                              ;L215
 47289|  br i1 %717, label %945, label %688                                                                                    ;L215
 47290| 
 47291| 718: ; preds = %700
 47292|     ;; has_ult_escape = i1 %699
 47293|  %719 = gep %200, i64 1576                                                                                             ;L192
 47294|  %720 = load i64, ptr %719, , !!8                                                                                      ;L192
 47295|  %721 = icmp eq i64 %720, 0                                                                                            ;L192
 47296|  br i1 %721, label %770, label %722                                                                                    ;L192
 47297| 
 47298| 722: ; preds = %718
 47299|  %723 = gep %200, i64 1648                                                                                             ;L192
 47300|  %724 = load i64, ptr %723, , !!8                                                                                      ;L192
 47301|  %725 = mul i64 %724, 100                                                                                              ;L192
 47302|  %726 = udiv i64 %725, %720                                                                                            ;L192
 47303|     ;; hp_ratio = i64 %726
 47304|     ;; team = !DIArgList(i64 1, i64 %190)
 47305|  %727 = sub nuw nsw i64 1, %190                                                                                        ;L193
 47306|     ;; team = i64 %727
 47307|  %728 = getelementptr [5 x ptr], ptr %197, i64 %727                                                                    ;L1905<193
 47308|  %729 = gep %5, i64 16                                                                                                 ;L194
 47309|  %730 = load ptr, ptr %729, , !!8, !!8                                                                                 ;L194
 47310|     ;; self[0..+8] = ptr %728
 47311|     ;; self[8..+8] = ptr %728
 47312|     ;; self[16..+8] = ptr %454
 47313|     ;; self[24..+8] = ptr %456
 47314|     ;; self[32..+8] = ptr %730
 47315|     ;; self[40..+8] = ptr %4
 47316|     ;; self[48..+8] = ptr %200
 47317|     ;; init = i64 0
 47320|     ;; self[0..+8] = ptr %728
 47321|     ;; iter[0..+8] = ptr %728
 47322|     ;; self[0..+8] = ptr %728
 47323|     ;; self[8..+8] = ptr %728
 47324|     ;; iter[8..+8] = ptr %728
 47325|     ;; self[8..+8] = ptr %728
 47326|     ;; self[16..+8] = ptr %454
 47327|     ;; iter[16..+8] = ptr %454
 47328|     ;; self[16..+8] = ptr %454
 47329|     ;; self[24..+8] = ptr %456
 47330|     ;; iter[24..+8] = ptr %456
 47331|     ;; self[24..+8] = ptr %456
 47332|     ;; self[32..+8] = ptr %730
 47333|     ;; iter[32..+8] = ptr %730
 47334|     ;; self[32..+8] = ptr %730
 47335|     ;; self[40..+8] = ptr %4
 47336|     ;; iter[40..+8] = ptr %4
 47337|     ;; self[40..+8] = ptr %4
 47338|     ;; fold[0..+8] = ptr %454
 47339|     ;; fold[8..+8] = ptr %456
 47340|     ;; fold[16..+8] = ptr %730
 47341|     ;; fold[24..+8] = ptr %4
 47342|     ;; self[0..+8] = ptr %728
 47343|     ;; self[8..+8] = ptr %728
 47344|     ;; init = i64 0
 47345|     ;; f[0..+8] = ptr %454
 47346|     ;; f[8..+8] = ptr %456
 47347|     ;; f[16..+8] = ptr %730
 47348|     ;; f[24..+8] = ptr %4
 47349|     ;; self[0..+8] = ptr %728
 47350|     ;; self[8..+8] = ptr %728
 47351|     ;; init = i64 0
 47352|     ;; rhs = i64 1
 47353|     ;; self[48..+8] = ptr %200
 47354|     ;; iter[48..+8] = ptr %200
 47355|     ;; self[48..+8] = ptr %200
 47356|     ;; f[32..+8] = ptr %200
 47357|     ;; fold[32..+8] = ptr %200
 47358|     ;; acc = i64 0
 47359|     ;; i = i64 0
 47360|     ;; self = i64 0
 47361|     ;; len = i64 5
 47362|  %731 = gep %200, i64 1632
 47363|  %732 = gep %200, i64 1640
 47364|  %733 = getelementptr { { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, [5 x { i64, [2 x i64] }], [5 x { i64, { i8, [23 x i8] } }], [5 x { i8, [7 x i8], i64 }], [5 x i64], [5 x i64], [5 x i64], [5 x { i64, i64 }], i64, i64, i64, i64, i64, [5 x i32], i8, [3 x i8] }, ptr %730, i64 %727
 47365|  br label %734                                                                                                         ;L28<146<128<52<3674<142<196
 47366| 
 47367| 734: ; preds = %766, %722
 47368|  %735 = phi i64 [ 0, %722 ], [ %768, %766 ]                                                                            ;L0<146<128<52<3674<142<196
 47369|  %736 = phi i64 [ 0, %722 ], [ %767, %766 ]                                                                            ;L0<146<128<52<3674<142<196
 47370|     ;; acc = i64 %736
 47371|     ;; self = i64 %735
 47372|     ;; i = i64 %735
 47373|     ;; self = ptr %728
 47374|     ;; count = i64 %735
 47375|  %737 = getelementptr ptr, ptr %728, i64 %735                                                                          ;L656<279<146<128<52<3674<142<196
 47376|  %738 = load ptr, ptr %737, , !!51388, !!8                                                                             ;L279<146<128<52<3674<142<196
 47378|     ;; acc = i64 %736
 47380|  %739 = icmp eq ptr %738, null                                                                                         ;L39<279<146<128<52<3674<142<196
 47381|  br i1 %739, label %766, label %740                                                                                    ;L39<279<146<128<52<3674<142<196
 47382| 
 47383| 740: ; preds = %734
 47384|     ;; x = ptr %738
 47386|     ;; acc = i64 %736
 47387|     ;; elt = ptr %738
 47388|     ;; x = ptr %738
 47394|  %741 = invoke zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %733, ptr %454, ptr %456, ptr %4, ptr %738)
 47395|  to label %742 unwind label %679                                                                                       ;L194<138<88<40<279<146<128<52<3674<142<196
 47396| 
 47397| 742: ; preds = %740
 47398|  br i1 %741, label %743, label %763                                                                                    ;L194<138<88<40<279<146<128<52<3674<142<196
 47399| 
 47400| 743: ; preds = %742
 47401|     ;; self = ptr %738
 47403|  %744 = gep %738, i64 1632                                                                                             ;L2158<195<138<88<40<279<146<128<52<3674<142<196
 47404|  %745 = load i64, ptr %744, , !!51462, !!8                                                                             ;L2158<195<138<88<40<279<146<128<52<3674<142<196
 47405|     ;; x1 = i64 %745
 47406|     ;; self = i64 %745
 47407|  %746 = gep %738, i64 1640                                                                                             ;L2158<195<138<88<40<279<146<128<52<3674<142<196
 47408|  %747 = load i64, ptr %746, , !!51462, !!8                                                                             ;L2158<195<138<88<40<279<146<128<52<3674<142<196
 47409|     ;; y1 = i64 %747
 47410|     ;; self = i64 %747
 47411|  %748 = load i64, ptr %731, , !!51462, !!8                                                                             ;L2158<195<138<88<40<279<146<128<52<3674<142<196
 47412|     ;; x2 = i64 %748
 47413|     ;; other = i64 %748
 47414|  %749 = load i64, ptr %732, , !!51462, !!8                                                                             ;L2158<195<138<88<40<279<146<128<52<3674<142<196
 47415|     ;; y2 = i64 %749
 47416|     ;; other = i64 %749
 47417|  %750 = icmp ult i64 %745, %748                                                                                        ;L3147<7<2158<195<138<88<40<279<146<128<52<3674<142<196
 47418|  %751 = sub nuw i64 %748, %745                                                                                         ;L3147<7<2158<195<138<88<40<279<146<128<52<3674<142<196
 47419|  %752 = sub nuw i64 %745, %748                                                                                         ;L3147<7<2158<195<138<88<40<279<146<128<52<3674<142<196
 47420|  %753 = select i1 %750, i64 %751, i64 %752                                                                             ;L3147<7<2158<195<138<88<40<279<146<128<52<3674<142<196
 47421|     ;; dx = i64 %753
 47422|  %754 = icmp ult i64 %747, %749                                                                                        ;L3147<8<2158<195<138<88<40<279<146<128<52<3674<142<196
 47423|  %755 = sub nuw i64 %749, %747                                                                                         ;L3147<8<2158<195<138<88<40<279<146<128<52<3674<142<196
 47424|  %756 = sub nuw i64 %747, %749                                                                                         ;L3147<8<2158<195<138<88<40<279<146<128<52<3674<142<196
 47425|  %757 = select i1 %754, i64 %755, i64 %756                                                                             ;L3147<8<2158<195<138<88<40<279<146<128<52<3674<142<196
 47426|     ;; dy = i64 %757
 47427|  %758 = mul i64 %753, %753                                                                                             ;L9<2158<195<138<88<40<279<146<128<52<3674<142<196
 47428|  %759 = mul i64 %757, %757                                                                                             ;L9<2158<195<138<88<40<279<146<128<52<3674<142<196
 47429|  %760 = add i64 %759, %758                                                                                             ;L9<2158<195<138<88<40<279<146<128<52<3674<142<196
 47430|  %761 = icmp ult i64 %760, 22500000001                                                                                 ;L195<138<88<40<279<146<128<52<3674<142<196
 47431|  %762 = zext i1 %761 to i64                                                                                            ;L138<88<40<279<146<128<52<3674<142<196
 47432|  br label %763                                                                                                         ;L194<138<88<40<279<146<128<52<3674<142<196
 47433| 
 47434| 763: ; preds = %743, %742
 47435|  %764 = phi i64 [ %762, %743 ], [ 0, %742 ]                                                                            ;L0<138<88<40<279<146<128<52<3674<142<196
 47437|     ;; a = i64 %736
 47438|     ;; b = i64 %764
 47439|  %765 = add i64 %764, %736                                                                                             ;L55<88<40<279<146<128<52<3674<142<196
 47440|  br label %766                                                                                                         ;L42<279<146<128<52<3674<142<196
 47441| 
 47442| 766: ; preds = %763, %734
 47443|  %767 = phi i64 [ %765, %763 ], [ %736, %734 ]                                                                         ;L0<279<146<128<52<3674<142<196
 47444|     ;; acc = i64 %767
 47445|  %768 = add nuw i64 %735, 1                                                                                            ;L971<283<146<128<52<3674<142<196
 47446|     ;; i = i64 %768
 47447|     ;; self = i64 %768
 47448|  %769 = icmp eq i64 %768, 5                                                                                            ;L284<146<128<52<3674<142<196
 47449|  br i1 %769, label %771, label %734                                                                                    ;L284<146<128<52<3674<142<196
 47450| 
 47451| 770: ; preds = %718
 47452|  invoke void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.282069a2ed2ad3a275929b639963fb55.146) #32
 47453|  to label %203 unwind label %679                                                                                       ;L192
 47454| 
 47455| 771: ; preds = %766
 47456|     ;; visible_enemies = i64 %767
 47457|     ;; self[0..+8] = ptr %198
 47458|     ;; self[8..+8] = ptr %198
 47459|     ;; self[16..+8] = ptr %200
 47460|     ;; init = i64 0
 47463|     ;; self[0..+8] = ptr %198
 47464|     ;; iter[0..+8] = ptr %198
 47465|     ;; self[0..+8] = ptr %198
 47466|     ;; self[8..+8] = ptr %198
 47467|     ;; iter[8..+8] = ptr %198
 47468|     ;; self[8..+8] = ptr %198
 47469|     ;; self[16..+8] = ptr %200
 47470|     ;; iter[16..+8] = ptr %200
 47471|     ;; self[16..+8] = ptr %200
 47473|     ;; self[0..+8] = ptr %198
 47474|     ;; self[8..+8] = ptr %198
 47475|     ;; init = i64 0
 47476|     ;; fold = ptr %200
 47478|     ;; f = ptr %200
 47479|     ;; self[0..+8] = ptr %198
 47480|     ;; self[8..+8] = ptr %198
 47481|     ;; init = i64 0
 47482|     ;; acc = i64 0
 47483|     ;; i = i64 0
 47484|     ;; len = i64 5
 47485|  %772 = gep %200, i64 1472
 47486|  %773 = load i64, ptr %772, , !!51626
 47487|  %774 = load i64, ptr %731, , !!51626
 47488|  %775 = load i64, ptr %732, , !!51626
 47489|     ;; self = ptr %198
 47490|     ;; count = i64 0
 47491|  %776 = load ptr, ptr %198, , !!51636, !!8                                                                             ;L279<146<128<52<3674<142<199
 47493|     ;; acc = i64 0
 47495|  %777 = icmp eq ptr %776, null                                                                                         ;L39<279<146<128<52<3674<142<199
 47496|  br i1 %777, label %800, label %778                                                                                    ;L39<279<146<128<52<3674<142<199
 47497| 
 47498| 778: ; preds = %771
 47499|     ;; x = ptr %776
 47501|     ;; acc = i64 0
 47502|     ;; elt = ptr %776
 47503|     ;; x = ptr %776
 47507|  %779 = gep %776, i64 1472                                                                                             ;L198<138<88<40<279<146<128<52<3674<142<199
 47508|  %780 = load i64, ptr %779, , !!51636, !!8                                                                             ;L198<138<88<40<279<146<128<52<3674<142<199
 47509|  %781 = icmp eq i64 %780, %773                                                                                         ;L198<138<88<40<279<146<128<52<3674<142<199
 47510|  br i1 %781, label %800, label %782                                                                                    ;L198<138<88<40<279<146<128<52<3674<142<199
 47511| 
 47512| 782: ; preds = %778
 47513|     ;; self = ptr %776
 47514|     ;; other = ptr %200
 47515|  %783 = gep %776, i64 1632                                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47516|  %784 = load i64, ptr %783, , !!51636, !!8                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47517|     ;; x1 = i64 %784
 47518|     ;; self = i64 %784
 47519|  %785 = gep %776, i64 1640                                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47520|  %786 = load i64, ptr %785, , !!51636, !!8                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47521|     ;; y1 = i64 %786
 47522|     ;; self = i64 %786
 47523|     ;; x2 = i64 %774
 47524|     ;; other = i64 %774
 47525|     ;; y2 = i64 %775
 47526|     ;; other = i64 %775
 47527|  %787 = icmp ult i64 %784, %774                                                                                        ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47528|  %788 = sub nuw i64 %774, %784                                                                                         ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47529|  %789 = sub nuw i64 %784, %774                                                                                         ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47530|  %790 = select i1 %787, i64 %788, i64 %789                                                                             ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47531|     ;; dx = i64 %790
 47532|  %791 = icmp ult i64 %786, %775                                                                                        ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47533|  %792 = sub nuw i64 %775, %786                                                                                         ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47534|  %793 = sub nuw i64 %786, %775                                                                                         ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47535|  %794 = select i1 %791, i64 %792, i64 %793                                                                             ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47536|     ;; dy = i64 %794
 47537|  %795 = mul i64 %790, %790                                                                                             ;L9<2158<198<138<88<40<279<146<128<52<3674<142<199
 47538|  %796 = mul i64 %794, %794                                                                                             ;L9<2158<198<138<88<40<279<146<128<52<3674<142<199
 47539|  %797 = add i64 %796, %795                                                                                             ;L9<2158<198<138<88<40<279<146<128<52<3674<142<199
 47540|  %798 = icmp ult i64 %797, 22500000001                                                                                 ;L198<138<88<40<279<146<128<52<3674<142<199
 47541|  %799 = zext i1 %798 to i64                                                                                            ;L138<88<40<279<146<128<52<3674<142<199
 47542|  br label %800                                                                                                         ;L198<138<88<40<279<146<128<52<3674<142<199
 47543| 
 47544| 800: ; preds = %782, %778, %771
 47545|  %801 = phi i64 [ 0, %771 ], [ %799, %782 ], [ 0, %778 ]                                                               ;L0<279<146<128<52<3674<142<199
 47546|     ;; acc = i64 %801
 47547|     ;; i = i64 1
 47548|     ;; self = ptr %198
 47549|     ;; count = i64 1
 47550|  %802 = gep %198, i64 8                                                                                                ;L656<279<146<128<52<3674<142<199
 47551|  %803 = load ptr, ptr %802, , !!51636, !!8                                                                             ;L279<146<128<52<3674<142<199
 47553|     ;; acc = i64 %801
 47555|  %804 = icmp eq ptr %803, null                                                                                         ;L39<279<146<128<52<3674<142<199
 47556|  br i1 %804, label %830, label %805                                                                                    ;L39<279<146<128<52<3674<142<199
 47557| 
 47558| 805: ; preds = %800
 47559|     ;; x = ptr %803
 47561|     ;; acc = i64 %801
 47562|     ;; elt = ptr %803
 47563|     ;; x = ptr %803
 47567|  %806 = gep %803, i64 1472                                                                                             ;L198<138<88<40<279<146<128<52<3674<142<199
 47568|  %807 = load i64, ptr %806, , !!51636, !!8                                                                             ;L198<138<88<40<279<146<128<52<3674<142<199
 47569|  %808 = icmp eq i64 %807, %773                                                                                         ;L198<138<88<40<279<146<128<52<3674<142<199
 47570|  br i1 %808, label %827, label %809                                                                                    ;L198<138<88<40<279<146<128<52<3674<142<199
 47571| 
 47572| 809: ; preds = %805
 47573|     ;; self = ptr %803
 47574|     ;; other = ptr %200
 47575|  %810 = gep %803, i64 1632                                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47576|  %811 = load i64, ptr %810, , !!51636, !!8                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47577|     ;; x1 = i64 %811
 47578|     ;; self = i64 %811
 47579|  %812 = gep %803, i64 1640                                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47580|  %813 = load i64, ptr %812, , !!51636, !!8                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47581|     ;; y1 = i64 %813
 47582|     ;; self = i64 %813
 47583|     ;; x2 = i64 %774
 47584|     ;; other = i64 %774
 47585|     ;; y2 = i64 %775
 47586|     ;; other = i64 %775
 47587|  %814 = icmp ult i64 %811, %774                                                                                        ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47588|  %815 = sub nuw i64 %774, %811                                                                                         ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47589|  %816 = sub nuw i64 %811, %774                                                                                         ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47590|  %817 = select i1 %814, i64 %815, i64 %816                                                                             ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47591|     ;; dx = i64 %817
 47592|  %818 = icmp ult i64 %813, %775                                                                                        ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47593|  %819 = sub nuw i64 %775, %813                                                                                         ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47594|  %820 = sub nuw i64 %813, %775                                                                                         ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47595|  %821 = select i1 %818, i64 %819, i64 %820                                                                             ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47596|     ;; dy = i64 %821
 47597|  %822 = mul i64 %817, %817                                                                                             ;L9<2158<198<138<88<40<279<146<128<52<3674<142<199
 47598|  %823 = mul i64 %821, %821                                                                                             ;L9<2158<198<138<88<40<279<146<128<52<3674<142<199
 47599|  %824 = add i64 %823, %822                                                                                             ;L9<2158<198<138<88<40<279<146<128<52<3674<142<199
 47600|  %825 = icmp ult i64 %824, 22500000001                                                                                 ;L198<138<88<40<279<146<128<52<3674<142<199
 47601|  %826 = zext i1 %825 to i64                                                                                            ;L138<88<40<279<146<128<52<3674<142<199
 47602|  br label %827                                                                                                         ;L198<138<88<40<279<146<128<52<3674<142<199
 47603| 
 47604| 827: ; preds = %809, %805
 47605|  %828 = phi i64 [ %826, %809 ], [ 0, %805 ]                                                                            ;L198<138<88<40<279<146<128<52<3674<142<199
 47607|     ;; a = i64 %801
 47608|     ;; b = i64 %828
 47609|  %829 = add nuw nsw i64 %828, %801                                                                                     ;L55<88<40<279<146<128<52<3674<142<199
 47610|  br label %830                                                                                                         ;L42<279<146<128<52<3674<142<199
 47611| 
 47612| 830: ; preds = %827, %800
 47613|  %831 = phi i64 [ %829, %827 ], [ %801, %800 ]                                                                         ;L0<279<146<128<52<3674<142<199
 47614|     ;; acc = i64 %831
 47615|     ;; i = i64 2
 47616|     ;; self = ptr %198
 47617|     ;; count = i64 2
 47618|  %832 = gep %198, i64 16                                                                                               ;L656<279<146<128<52<3674<142<199
 47619|  %833 = load ptr, ptr %832, , !!51636, !!8                                                                             ;L279<146<128<52<3674<142<199
 47621|     ;; acc = i64 %831
 47623|  %834 = icmp eq ptr %833, null                                                                                         ;L39<279<146<128<52<3674<142<199
 47624|  br i1 %834, label %860, label %835                                                                                    ;L39<279<146<128<52<3674<142<199
 47625| 
 47626| 835: ; preds = %830
 47627|     ;; x = ptr %833
 47629|     ;; acc = i64 %831
 47630|     ;; elt = ptr %833
 47631|     ;; x = ptr %833
 47635|  %836 = gep %833, i64 1472                                                                                             ;L198<138<88<40<279<146<128<52<3674<142<199
 47636|  %837 = load i64, ptr %836, , !!51636, !!8                                                                             ;L198<138<88<40<279<146<128<52<3674<142<199
 47637|  %838 = icmp eq i64 %837, %773                                                                                         ;L198<138<88<40<279<146<128<52<3674<142<199
 47638|  br i1 %838, label %857, label %839                                                                                    ;L198<138<88<40<279<146<128<52<3674<142<199
 47639| 
 47640| 839: ; preds = %835
 47641|     ;; self = ptr %833
 47642|     ;; other = ptr %200
 47643|  %840 = gep %833, i64 1632                                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47644|  %841 = load i64, ptr %840, , !!51636, !!8                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47645|     ;; x1 = i64 %841
 47646|     ;; self = i64 %841
 47647|  %842 = gep %833, i64 1640                                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47648|  %843 = load i64, ptr %842, , !!51636, !!8                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47649|     ;; y1 = i64 %843
 47650|     ;; self = i64 %843
 47651|     ;; x2 = i64 %774
 47652|     ;; other = i64 %774
 47653|     ;; y2 = i64 %775
 47654|     ;; other = i64 %775
 47655|  %844 = icmp ult i64 %841, %774                                                                                        ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47656|  %845 = sub nuw i64 %774, %841                                                                                         ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47657|  %846 = sub nuw i64 %841, %774                                                                                         ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47658|  %847 = select i1 %844, i64 %845, i64 %846                                                                             ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47659|     ;; dx = i64 %847
 47660|  %848 = icmp ult i64 %843, %775                                                                                        ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47661|  %849 = sub nuw i64 %775, %843                                                                                         ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47662|  %850 = sub nuw i64 %843, %775                                                                                         ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47663|  %851 = select i1 %848, i64 %849, i64 %850                                                                             ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47664|     ;; dy = i64 %851
 47665|  %852 = mul i64 %847, %847                                                                                             ;L9<2158<198<138<88<40<279<146<128<52<3674<142<199
 47666|  %853 = mul i64 %851, %851                                                                                             ;L9<2158<198<138<88<40<279<146<128<52<3674<142<199
 47667|  %854 = add i64 %853, %852                                                                                             ;L9<2158<198<138<88<40<279<146<128<52<3674<142<199
 47668|  %855 = icmp ult i64 %854, 22500000001                                                                                 ;L198<138<88<40<279<146<128<52<3674<142<199
 47669|  %856 = zext i1 %855 to i64                                                                                            ;L138<88<40<279<146<128<52<3674<142<199
 47670|  br label %857                                                                                                         ;L198<138<88<40<279<146<128<52<3674<142<199
 47671| 
 47672| 857: ; preds = %839, %835
 47673|  %858 = phi i64 [ %856, %839 ], [ 0, %835 ]                                                                            ;L198<138<88<40<279<146<128<52<3674<142<199
 47675|     ;; a = i64 %831
 47676|     ;; b = i64 %858
 47677|  %859 = add nuw nsw i64 %858, %831                                                                                     ;L55<88<40<279<146<128<52<3674<142<199
 47678|  br label %860                                                                                                         ;L42<279<146<128<52<3674<142<199
 47679| 
 47680| 860: ; preds = %857, %830
 47681|  %861 = phi i64 [ %859, %857 ], [ %831, %830 ]                                                                         ;L0<279<146<128<52<3674<142<199
 47682|     ;; acc = i64 %861
 47683|     ;; i = i64 3
 47684|     ;; self = ptr %198
 47685|     ;; count = i64 3
 47686|  %862 = gep %198, i64 24                                                                                               ;L656<279<146<128<52<3674<142<199
 47687|  %863 = load ptr, ptr %862, , !!51636, !!8                                                                             ;L279<146<128<52<3674<142<199
 47689|     ;; acc = i64 %861
 47691|  %864 = icmp eq ptr %863, null                                                                                         ;L39<279<146<128<52<3674<142<199
 47692|  br i1 %864, label %890, label %865                                                                                    ;L39<279<146<128<52<3674<142<199
 47693| 
 47694| 865: ; preds = %860
 47695|     ;; x = ptr %863
 47697|     ;; acc = i64 %861
 47698|     ;; elt = ptr %863
 47699|     ;; x = ptr %863
 47703|  %866 = gep %863, i64 1472                                                                                             ;L198<138<88<40<279<146<128<52<3674<142<199
 47704|  %867 = load i64, ptr %866, , !!51636, !!8                                                                             ;L198<138<88<40<279<146<128<52<3674<142<199
 47705|  %868 = icmp eq i64 %867, %773                                                                                         ;L198<138<88<40<279<146<128<52<3674<142<199
 47706|  br i1 %868, label %887, label %869                                                                                    ;L198<138<88<40<279<146<128<52<3674<142<199
 47707| 
 47708| 869: ; preds = %865
 47709|     ;; self = ptr %863
 47710|     ;; other = ptr %200
 47711|  %870 = gep %863, i64 1632                                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47712|  %871 = load i64, ptr %870, , !!51636, !!8                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47713|     ;; x1 = i64 %871
 47714|     ;; self = i64 %871
 47715|  %872 = gep %863, i64 1640                                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47716|  %873 = load i64, ptr %872, , !!51636, !!8                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47717|     ;; y1 = i64 %873
 47718|     ;; self = i64 %873
 47719|     ;; x2 = i64 %774
 47720|     ;; other = i64 %774
 47721|     ;; y2 = i64 %775
 47722|     ;; other = i64 %775
 47723|  %874 = icmp ult i64 %871, %774                                                                                        ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47724|  %875 = sub nuw i64 %774, %871                                                                                         ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47725|  %876 = sub nuw i64 %871, %774                                                                                         ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47726|  %877 = select i1 %874, i64 %875, i64 %876                                                                             ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47727|     ;; dx = i64 %877
 47728|  %878 = icmp ult i64 %873, %775                                                                                        ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47729|  %879 = sub nuw i64 %775, %873                                                                                         ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47730|  %880 = sub nuw i64 %873, %775                                                                                         ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47731|  %881 = select i1 %878, i64 %879, i64 %880                                                                             ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47732|     ;; dy = i64 %881
 47733|  %882 = mul i64 %877, %877                                                                                             ;L9<2158<198<138<88<40<279<146<128<52<3674<142<199
 47734|  %883 = mul i64 %881, %881                                                                                             ;L9<2158<198<138<88<40<279<146<128<52<3674<142<199
 47735|  %884 = add i64 %883, %882                                                                                             ;L9<2158<198<138<88<40<279<146<128<52<3674<142<199
 47736|  %885 = icmp ult i64 %884, 22500000001                                                                                 ;L198<138<88<40<279<146<128<52<3674<142<199
 47737|  %886 = zext i1 %885 to i64                                                                                            ;L138<88<40<279<146<128<52<3674<142<199
 47738|  br label %887                                                                                                         ;L198<138<88<40<279<146<128<52<3674<142<199
 47739| 
 47740| 887: ; preds = %869, %865
 47741|  %888 = phi i64 [ %886, %869 ], [ 0, %865 ]                                                                            ;L198<138<88<40<279<146<128<52<3674<142<199
 47743|     ;; a = i64 %861
 47744|     ;; b = i64 %888
 47745|  %889 = add nuw nsw i64 %888, %861                                                                                     ;L55<88<40<279<146<128<52<3674<142<199
 47746|  br label %890                                                                                                         ;L42<279<146<128<52<3674<142<199
 47747| 
 47748| 890: ; preds = %887, %860
 47749|  %891 = phi i64 [ %889, %887 ], [ %861, %860 ]                                                                         ;L0<279<146<128<52<3674<142<199
 47750|     ;; acc = i64 %891
 47751|     ;; i = i64 4
 47752|     ;; self = ptr %198
 47753|     ;; count = i64 4
 47754|  %892 = gep %198, i64 32                                                                                               ;L656<279<146<128<52<3674<142<199
 47755|  %893 = load ptr, ptr %892, , !!51636, !!8                                                                             ;L279<146<128<52<3674<142<199
 47757|     ;; acc = i64 %891
 47759|  %894 = icmp eq ptr %893, null                                                                                         ;L39<279<146<128<52<3674<142<199
 47760|  br i1 %894, label %920, label %895                                                                                    ;L39<279<146<128<52<3674<142<199
 47761| 
 47762| 895: ; preds = %890
 47763|     ;; x = ptr %893
 47765|     ;; acc = i64 %891
 47766|     ;; elt = ptr %893
 47767|     ;; x = ptr %893
 47771|  %896 = gep %893, i64 1472                                                                                             ;L198<138<88<40<279<146<128<52<3674<142<199
 47772|  %897 = load i64, ptr %896, , !!51636, !!8                                                                             ;L198<138<88<40<279<146<128<52<3674<142<199
 47773|  %898 = icmp eq i64 %897, %773                                                                                         ;L198<138<88<40<279<146<128<52<3674<142<199
 47774|  br i1 %898, label %917, label %899                                                                                    ;L198<138<88<40<279<146<128<52<3674<142<199
 47775| 
 47776| 899: ; preds = %895
 47777|     ;; self = ptr %893
 47778|     ;; other = ptr %200
 47779|  %900 = gep %893, i64 1632                                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47780|  %901 = load i64, ptr %900, , !!51636, !!8                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47781|     ;; x1 = i64 %901
 47782|     ;; self = i64 %901
 47783|  %902 = gep %893, i64 1640                                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47784|  %903 = load i64, ptr %902, , !!51636, !!8                                                                             ;L2158<198<138<88<40<279<146<128<52<3674<142<199
 47785|     ;; y1 = i64 %903
 47786|     ;; self = i64 %903
 47787|     ;; x2 = i64 %774
 47788|     ;; other = i64 %774
 47789|     ;; y2 = i64 %775
 47790|     ;; other = i64 %775
 47791|  %904 = icmp ult i64 %901, %774                                                                                        ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47792|  %905 = sub nuw i64 %774, %901                                                                                         ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47793|  %906 = sub nuw i64 %901, %774                                                                                         ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47794|  %907 = select i1 %904, i64 %905, i64 %906                                                                             ;L3147<7<2158<198<138<88<40<279<146<128<52<3674<142<199
 47795|     ;; dx = i64 %907
 47796|  %908 = icmp ult i64 %903, %775                                                                                        ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47797|  %909 = sub nuw i64 %775, %903                                                                                         ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47798|  %910 = sub nuw i64 %903, %775                                                                                         ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47799|  %911 = select i1 %908, i64 %909, i64 %910                                                                             ;L3147<8<2158<198<138<88<40<279<146<128<52<3674<142<199
 47800|     ;; dy = i64 %911
 47801|  %912 = mul i64 %907, %907                                                                                             ;L9<2158<198<138<88<40<279<146<128<52<3674<142<199
 47802|  %913 = mul i64 %911, %911                                                                                             ;L9<2158<198<138<88<40<279<146<128<52<3674<142<199
 47803|  %914 = add i64 %913, %912                                                                                             ;L9<2158<198<138<88<40<279<146<128<52<3674<142<199
 47804|  %915 = icmp ult i64 %914, 22500000001                                                                                 ;L198<138<88<40<279<146<128<52<3674<142<199
 47805|  %916 = zext i1 %915 to i64                                                                                            ;L138<88<40<279<146<128<52<3674<142<199
 47806|  br label %917                                                                                                         ;L198<138<88<40<279<146<128<52<3674<142<199
 47807| 
 47808| 917: ; preds = %899, %895
 47809|  %918 = phi i64 [ %916, %899 ], [ 0, %895 ]                                                                            ;L198<138<88<40<279<146<128<52<3674<142<199
 47811|     ;; a = i64 %891
 47812|     ;; b = i64 %918
 47813|  %919 = add nuw nsw i64 %918, %891                                                                                     ;L55<88<40<279<146<128<52<3674<142<199
 47814|  br label %920                                                                                                         ;L42<279<146<128<52<3674<142<199
 47815| 
 47816| 920: ; preds = %917, %890
 47817|  %921 = phi i64 [ %919, %917 ], [ %891, %890 ]                                                                         ;L0<279<146<128<52<3674<142<199
 47818|     ;; acc = i64 %921
 47819|     ;; i = i64 5
 47820|     ;; nearby_allies = i64 %921
 47821|     ;; self = ptr %1
 47823|  %922 = gep %456, i64 40                                                                                               ;L202
 47824|  %923 = load ptr, ptr %922, , !!8                                                                                      ;L202
 47825|  %924 = invoke i64 %923(ptr %454)
 47826|  to label %925 unwind label %679                                                                                       ;L202
 47827| 
 47828| 925: ; preds = %920
 47829|  %926 = gep %1, i64 2136                                                                                               ;L201
 47830|     ;; self = ptr %926
 47831|  %927 = gep %67, i64 176                                                                                               ;L201
 47832|  store i64 %924, ptr %927,                                                                                             ;L201
 47833|  store i64 -9223372036854775797, ptr %67,                                                                              ;L201
 47834|  %928 = gep %67, i64 8                                                                                                 ;L201
 47835|  store i64 %726, ptr %928,                                                                                             ;L201
 47836|  %929 = gep %67, i64 16                                                                                                ;L201
 47837|  store i64 %767, ptr %929,                                                                                             ;L201
 47838|  %930 = gep %67, i64 24                                                                                                ;L201
 47839|  store i64 %921, ptr %930,                                                                                             ;L201
 47840|  %931 = gep %67, i64 32                                                                                                ;L201
 47841|  store i8 0, ptr %931,                                                                                                 ;L201
 47842|     ;; self = ptr %926
 47843|     ;; self = ptr %926
 47844|     ;; value = ptr %67
 47846|     ;; elem_size = i64 184
 47847|  %932 = gep %1, i64 2152                                                                                               ;L1037<1004<201
 47848|  %933 = load i64, ptr %932, , !!51757, !!8                                                                             ;L1037<1004<201
 47849|     ;; len = i64 %933
 47850|     ;; count = i64 %933
 47851|     ;; self = ptr %926
 47852|  %934 = load i64, ptr %926, , !!51757, !!8                                                                             ;L619<309<1040<1004<201
 47853|  %935 = icmp eq i64 %933, %934                                                                                         ;L1040<1004<201
 47854|  br i1 %935, label %936, label %940                                                                                    ;L1040<1004<201
 47855| 
 47856| 936: ; preds = %925
 47857|  invoke void @gc::simulation12ai_interface17PendingTraceEventE8grow_oneCshdEBA0ozCnw_7game_ai(ptr %926)
 47858|  to label %940 unwind label %937, !!51757                                                                              ;L1041<1004<201
 47859| 
 47860| 937: ; preds = %936
 47861|  %938 = cleanuppad within none []
 47862|  invoke fastcc void @core::ptr9drop_glueNtNtNtCs97f5S1uJLkH_9game_core10simulation12ai_interface17PendingTraceEventECshdEBA0ozCnw_7game_ai(ptr %67) #31 [ "funclet"(token %938) ]
 47863|  to label %939 unwind label %679                                                                                       ;L1050<1004<201
 47864| 
 47865| 939: ; preds = %937
 47866|  cleanupret from %938 unwind label %679
 47867| 
 47868| 940: ; preds = %936, %925
 47869|  %941 = gep %1, i64 2144                                                                                               ;L614<609<296<2052<1044<1004<201
 47870|  %942 = load ptr, ptr %941, , !!51757, !!8, !!8                                                                        ;L614<609<296<2052<1044<1004<201
 47871|     ;; self = ptr %942
 47872|  %943 = getelementptr { { i64, [21 x i64] }, i64 }, ptr %942, i64 %933                                                 ;L961<1044<1004<201
 47873|     ;; end = ptr %943
 47874|     ;; dst = ptr %943
 47875|  call void @llvm.memcpy.p0.p0.i64(ptr %943, ptr %67, i64 184, i1 false)                                                ;L1933<1045<1004<201
 47876|  %944 = add i64 %933, 1                                                                                                ;L1046<1004<201
 47877|  store i64 %944, ptr %932, , !!51757                                                                                   ;L1046<1004<201
 47879|  br label %710                                                                                                         ;L191
 47880| 
 47881| 945: ; preds = %710
 47883|  invoke void @gc::simulationNtB5_21AbstractGameWithCache25iter_towers_without_nexus(ptr sret([120 x i8]) %66, ptr %196, i64 %190)
 47884|  to label %946 unwind label %679                                                                                       ;L217
 47885| 
 47886| 946: ; preds = %945
 47888|  call void @llvm.memcpy.p0.p0.i64(ptr %15, ptr %66, i64 120, i1 false)                                                 ;L28<957<218
 47891|     ;; f = ptr %200
 47892|     ;; self = ptr %15
 47895|     ;; f = ptr %200
 47896|  %947 = gep %15, i64 120                                                                                               ;L69<836<3387<219
 47897|  store ptr %200, ptr %947, , !!51846                                                                                   ;L69<836<3387<219
 47899|     ;; self = ptr %15
 47902|     ;; self = ptr %15
 47904|     ;; self = ptr %15
 47907|  store ptr %947, ptr %13, , !!51898
 47908|     ;; self = ptr %15
 47909|     ;; predicate = ptr %13
 47911|     ;; opt = ptr %15
 47912|     ;; self = ptr %15
 47913|     ;; f = ptr %13
 47914|  %948 = load i64, ptr %15, , !!51945, !!8                                                                              ;L764<332<169<98<107<2706<3416<3387<219
 47915|  %949 = icmp eq i64 %948, -1                                                                                           ;L764<332<169<98<107<2706<3416<3387<219
 47916|  br i1 %949, label %979, label %950                                                                                    ;L764<332<169<98<107<2706<3416<3387<219
 47917| 
 47918| 950: ; preds = %946
 47920|     ;; predicate = ptr %13
 47921|     ;; a = ptr %15
 47923|     ;; self = ptr %15
 47924|     ;; predicate = ptr %13
 47926|     ;; self = ptr %15
 47928|     ;; fold = ptr %13
 47930|     ;; self = ptr %15
 47933|     ;; fold = ptr %13
 47934|     ;; self = ptr %15
 47935|     ;; fold = ptr %13
 47937|     ;; self = ptr %15
 47939|     ;; fold = ptr %13
 47940|  %951 = trunc nuw i64 %948 to i1                                                                                       ;L396<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47941|  br i1 %951, label %952, label %977                                                                                    ;L396<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47942| 
 47943| 952: ; preds = %950
 47944|  %953 = gep %15, i64 8                                                                                                 ;L396<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47945|     ;; iter = ptr %953
 47947|     ;; self = ptr %953
 47950|     ;; f = ptr %13
 47951|     ;; f = ptr %13
 47952|     ;; self[0..+8] = ptr %953
 47953|     ;; self[8..+8] = i64 6
 47954|  %954 = gep %15, i64 24                                                                                                ;L214<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47955|     ;; data[0..+8] = ptr %954
 47956|     ;; data[8..+8] = i64 6
 47958|  store ptr %954, ptr %12, , !!52073                                                                                    ;L215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47959|  %955 = gep %12, i64 8                                                                                                 ;L215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47960|  store i64 6, ptr %955, , !!52073                                                                                      ;L215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47961|  %956 = gep %12, i64 16                                                                                                ;L215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47962|  store ptr %13, ptr %956, , !!52073                                                                                    ;L215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47965|     ;; self = ptr %953
 47966|     ;; self = ptr %953
 47967|     ;; self = ptr %953
 47969|     ;; f = ptr %12
 47970|     ;; rhs = i64 1
 47971|  %957 = load i64, ptr %953, , !!52127, !!8                                                                             ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47972|  %958 = gep %15, i64 16                                                                                                ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47973|  %959 = load i64, ptr %958, , !!52127, !!8                                                                             ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47974|  %960 = icmp ule i64 %957, %959                                                                                        ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47975|     ;; cond = i1 true
 47976|  call void @llvm.assume(i1 %960)                                                                                       ;L210<122<166<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47977|  %961 = icmp eq i64 %957, %959                                                                                         ;L167<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47978|  br i1 %961, label %976, label %962                                                                                    ;L167<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47979| 
 47980| 962: ; preds = %974, %952
 47981|  %963 = phi i64 [ %964, %974 ], [ %957, %952 ]
 47982|     ;; i = i64 %963
 47983|     ;; value = i64 %963
 47984|     ;; self = i64 %963
 47985|  %964 = add nuw i64 %963, 1                                                                                            ;L971<63<169<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47986|  store i64 %964, ptr %953, , !!52127                                                                                   ;L63<169<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47989|     ;; f = ptr %12
 47991|     ;; idx = i64 %963
 47992|     ;; index = i64 %963
 47993|     ;; self = i64 %963
 47994|  %965 = load ptr, ptr %12, , !!52171, !!8, !!8                                                                         ;L219<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47995|  %966 = load i64, ptr %955, , !!52171, !!8                                                                             ;L219<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 47996|     ;; self[0..+8] = ptr %965
 47997|     ;; slice[0..+8] = ptr %965
 47998|     ;; self[8..+8] = i64 %966
 47999|     ;; slice[8..+8] = i64 %966
 48000|  %967 = icmp ult i64 %963, %966                                                                                        ;L252<646<219<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 48001|  call void @llvm.assume(i1 %967)                                                                                       ;L252<646<219<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 48002|  %968 = getelementptr ptr, ptr %965, i64 %963                                                                          ;L253<646<219<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 48003|     ;; self = ptr %968
 48004|     ;; self = ptr %968
 48005|     ;; src = ptr %968
 48006|  %969 = load ptr, ptr %968, , !!52189, !!8                                                                             ;L1733<1171<798<219<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 48007|     ;; elem = ptr %969
 48008|     ;; fold = ptr %12
 48010|     ;; inner = ptr %969
 48011|  %970 = icmp eq ptr %969, null                                                                                         ;L819<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 48012|  br i1 %970, label %974, label %971                                                                                    ;L819<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 48013| 
 48014| 971: ; preds = %962
 48015|     ;; fold = ptr %956
 48016|     ;; item = ptr %969
 48018|  store ptr %969, ptr %11, , !!52211
 48019|     ;; predicate = ptr %956
 48021|     ;; x = ptr %11
 48022|  %972 = invoke zeroext i1 @ai::plan_legacy7handler7auctionNtBW_17LegacyPlanHandler16get_small_actions5_0INtB7_5FnMutTRRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEE8call_mutB10_(ptr %956, ptr %11)
 48023|  to label %973 unwind label %679                                                                                       ;L2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 48024| 
 48025| 973: ; preds = %971
 48027|  br i1 %972, label %978, label %974                                                                                    ;L820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 48028| 
 48029| 974: ; preds = %973, %962
 48030|  %975 = icmp eq i64 %964, %959                                                                                         ;L167<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 48031|  br i1 %975, label %976, label %962                                                                                    ;L167<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<219
 48032| 
 48033| 976: ; preds = %974, %952
 48035|     ;; x = ptr null
 48036|  br label %977                                                                                                         ;L333<169<98<107<2706<3416<3387<219
 48037| 
 48038| 977: ; preds = %976, %950
 48039|  store i64 -1, ptr %15, , !!51945                                                                                      ;L334<169<98<107<2706<3416<3387<219
 48040|  br label %979                                                                                                         ;L333<169<98<107<2706<3416<3387<219
 48041| 
 48042| 978: ; preds = %973
 48044|     ;; x = ptr %969
 48045|     ;; self = ptr %969
 48046|     ;; f[0..+8] = ptr %15
 48049|     ;; self = ptr %969
 48050|     ;; f = ptr %15
 48051|     ;; self = ptr %15
 48052|  br label %989                                                                                                         ;L1161<107<2706<3416<3387<219
 48053| 
 48054| 979: ; preds = %977, %946
 48055|  %980 = gep %15, i64 104                                                                                               ;L170<98<107<2706<3416<3387<219
 48056|     ;; self = ptr null
 48057|     ;; f[0..+8] = ptr %980
 48062|     ;; self = ptr %980
 48063|  %981 = load ptr, ptr %980, , !!52303, !!8                                                                             ;L764<170<1653<170<98<107<2706<3416<3387<219
 48064|  %982 = icmp eq ptr %981, null                                                                                         ;L764<170<1653<170<98<107<2706<3416<3387<219
 48065|  br i1 %982, label %983, label %984                                                                                    ;L764<170<1653<170<98<107<2706<3416<3387<219
 48066| 
 48067| 983: ; preds = %979
 48069|     ;; self = ptr null
 48070|     ;; f = ptr %15
 48071|     ;; self = ptr %15
 48072|  br label %1012                                                                                                        ;L1161<107<2706<3416<3387<219
 48073| 
 48074| 984: ; preds = %979
 48075|  %985 = load ptr, ptr %13, , !!51898, !!8, !!8                                                                         ;L170<98<107<2706<3416<3387<219
 48076|     ;; f[8..+8] = ptr %985
 48077|     ;; self = ptr %980
 48078|     ;; predicate = ptr %985
 48079|  %986 = invoke ptr @core::iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2p_4find5checkB1s_QNCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler7auctionNtB3F_17LegacyPlanHandler16get_small_actions5_0E0INtNtNtBb_3ops12control_flow11ControlFlowB1s_EEB3J_(ptr %980, ptr %985)
 48080|  to label %987 unwind label %679                                                                                       ;L2971<170<1653<170<98<107<2706<3416<3387<219
 48081| 
 48082| 987: ; preds = %984
 48084|     ;; self = ptr %986
 48085|     ;; f = ptr %15
 48086|     ;; self = ptr %15
 48087|  %988 = icmp eq ptr %986, null                                                                                         ;L1161<107<2706<3416<3387<219
 48088|  br i1 %988, label %1012, label %989                                                                                   ;L1161<107<2706<3416<3387<219
 48089| 
 48090| 989: ; preds = %987, %978
 48091|  %990 = phi ptr [ %969, %978 ], [ %986, %987 ]
 48092|     ;; f = ptr %947
 48093|     ;; self = ptr %947
 48094|     ;; x = ptr %990
 48095|     ;; args = ptr %990
 48096|  %991 = load ptr, ptr %947, , !!51798, !!8, !!8                                                                        ;L310<1162<107<2706<3416<3387<219
 48098|     ;; x = ptr %990
 48102|     ;; self = ptr %990
 48103|     ;; other = ptr %991
 48104|  %992 = gep %990, i64 1632                                                                                             ;L2158<219<3379<310<1162<107<2706<3416<3387<219
 48105|  %993 = load i64, ptr %992, , !!52347, !!8                                                                             ;L2158<219<3379<310<1162<107<2706<3416<3387<219
 48106|     ;; x1 = i64 %993
 48107|     ;; self = i64 %993
 48108|  %994 = gep %990, i64 1640                                                                                             ;L2158<219<3379<310<1162<107<2706<3416<3387<219
 48109|  %995 = load i64, ptr %994, , !!52347, !!8                                                                             ;L2158<219<3379<310<1162<107<2706<3416<3387<219
 48110|     ;; y1 = i64 %995
 48111|     ;; self = i64 %995
 48112|  %996 = gep %991, i64 1632                                                                                             ;L2158<219<3379<310<1162<107<2706<3416<3387<219
 48113|  %997 = load i64, ptr %996, , !!52368, !!8                                                                             ;L2158<219<3379<310<1162<107<2706<3416<3387<219
 48114|     ;; x2 = i64 %997
 48115|     ;; other = i64 %997
 48116|  %998 = gep %991, i64 1640                                                                                             ;L2158<219<3379<310<1162<107<2706<3416<3387<219
 48117|  %999 = load i64, ptr %998, , !!52368, !!8                                                                             ;L2158<219<3379<310<1162<107<2706<3416<3387<219
 48118|     ;; y2 = i64 %999
 48119|     ;; other = i64 %999
 48120|  %1000 = icmp ult i64 %993, %997                                                                                       ;L3147<7<2158<219<3379<310<1162<107<2706<3416<3387<219
 48121|  %1001 = sub nuw i64 %997, %993                                                                                        ;L3147<7<2158<219<3379<310<1162<107<2706<3416<3387<219
 48122|  %1002 = sub nuw i64 %993, %997                                                                                        ;L3147<7<2158<219<3379<310<1162<107<2706<3416<3387<219
 48123|  %1003 = select i1 %1000, i64 %1001, i64 %1002                                                                         ;L3147<7<2158<219<3379<310<1162<107<2706<3416<3387<219
 48124|     ;; dx = i64 %1003
 48125|  %1004 = icmp ult i64 %995, %999                                                                                       ;L3147<8<2158<219<3379<310<1162<107<2706<3416<3387<219
 48126|  %1005 = sub nuw i64 %999, %995                                                                                        ;L3147<8<2158<219<3379<310<1162<107<2706<3416<3387<219
 48127|  %1006 = sub nuw i64 %995, %999                                                                                        ;L3147<8<2158<219<3379<310<1162<107<2706<3416<3387<219
 48128|  %1007 = select i1 %1004, i64 %1005, i64 %1006                                                                         ;L3147<8<2158<219<3379<310<1162<107<2706<3416<3387<219
 48129|     ;; dy = i64 %1007
 48130|  %1008 = mul i64 %1003, %1003                                                                                          ;L9<2158<219<3379<310<1162<107<2706<3416<3387<219
 48131|  %1009 = mul i64 %1007, %1007                                                                                          ;L9<2158<219<3379<310<1162<107<2706<3416<3387<219
 48132|  %1010 = add i64 %1009, %1008                                                                                          ;L9<2158<219<3379<310<1162<107<2706<3416<3387<219
 48133|     ;; first[0..+8] = i64 %1010
 48134|     ;; first[8..+8] = ptr %990
 48136|  call void @llvm.memcpy.p0.p0.i64(ptr %14, ptr %15, i64 128, i1 false), !!51798                                        ;L2707<3416<3387<219
 48137|  %1011 = invoke { i64, ptr } @core::iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtB8_5chain5ChainINtNtB8_7flatten7FlattenINtNtNtBc_5array4iter8IntoIterINtNtBc_6option6OptionRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEKj6_EEINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterB2R_EEENCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler7auctionNtB4T_17LegacyPlanHandler16get_small_actions5_0ENCINvNvNtNtNtBa_6traits8iterator8Iterator10min_by_key3keyB2R_yNCB4O_s6_0E0EB6F_4foldTyB2R_ENCINvNvB6F_6min_by4foldB7U_INvB6D_7compareB2R_yEE0EB4X_(ptr %14, i64 %1010, ptr %990)
 48138|  to label %1013 unwind label %679                                                                                      ;L2707<3416<3387<219
 48139| 
 48140| 1012: ; preds = %987, %983
 48142|     ;; nearest_tower = ptr null
 48143|  br label %688                                                                                                         ;L221
 48144| 
 48145| 1013: ; preds = %989
 48146|  %1014 = extractvalue { i64, ptr } %1011, 1                                                                            ;L2707<3416<3387<219
 48149|     ;; nearest_tower = ptr %1014
 48150|  %1015 = icmp eq ptr %1014, null                                                                                       ;L221
 48151|  br i1 %1015, label %688, label %1016                                                                                  ;L221
 48152| 
 48153| 1016: ; preds = %1013
 48154|     ;; tower = ptr %1014
 48155|     ;; self = ptr %1014
 48156|  %1017 = gep %1014, i64 1632                                                                                           ;L2158<222
 48157|  %1018 = load i64, ptr %1017, , !!8                                                                                    ;L2158<222
 48158|     ;; x1 = i64 %1018
 48159|     ;; self = i64 %1018
 48160|  %1019 = gep %1014, i64 1640                                                                                           ;L2158<222
 48161|  %1020 = load i64, ptr %1019, , !!8                                                                                    ;L2158<222
 48162|     ;; y1 = i64 %1020
 48163|     ;; self = i64 %1020
 48164|  %1021 = gep %200, i64 1632                                                                                            ;L2158<222
 48165|  %1022 = load i64, ptr %1021, , !!8                                                                                    ;L2158<222
 48166|     ;; x2 = i64 %1022
 48167|     ;; other = i64 %1022
 48168|  %1023 = gep %200, i64 1640                                                                                            ;L2158<222
 48169|  %1024 = load i64, ptr %1023, , !!8                                                                                    ;L2158<222
 48170|     ;; y2 = i64 %1024
 48171|     ;; other = i64 %1024
 48172|  %1025 = icmp ult i64 %1018, %1022                                                                                     ;L3147<7<2158<222
 48173|  %1026 = sub nuw i64 %1022, %1018                                                                                      ;L3147<7<2158<222
 48174|  %1027 = sub nuw i64 %1018, %1022                                                                                      ;L3147<7<2158<222
 48175|  %1028 = select i1 %1025, i64 %1026, i64 %1027                                                                         ;L3147<7<2158<222
 48176|     ;; dx = i64 %1028
 48177|  %1029 = icmp ult i64 %1020, %1024                                                                                     ;L3147<8<2158<222
 48178|  %1030 = sub nuw i64 %1024, %1020                                                                                      ;L3147<8<2158<222
 48179|  %1031 = sub nuw i64 %1020, %1024                                                                                      ;L3147<8<2158<222
 48180|  %1032 = select i1 %1029, i64 %1030, i64 %1031                                                                         ;L3147<8<2158<222
 48181|     ;; dy = i64 %1032
 48182|  %1033 = mul i64 %1028, %1028                                                                                          ;L9<2158<222
 48183|  %1034 = mul i64 %1032, %1032                                                                                          ;L9<2158<222
 48184|  %1035 = add i64 %1034, %1033                                                                                          ;L9<2158<222
 48185|     ;; tower_dist = i64 %1035
 48186|  %1036 = icmp ult i64 %1035, 2500000001                                                                                ;L223
 48187|  br i1 %1036, label %1037, label %688                                                                                  ;L223
 48188| 
 48189| 1037: ; preds = %1016
 48190|     ;; self = ptr %1014
 48191|  %1038 = gep %1014, i64 1216                                                                                           ;L742<224
 48192|  %1039 = load i32, ptr %1038, , !!8                                                                                    ;L742<224
 48193|  %1040 = icmp eq i32 %1039, -1                                                                                         ;L742<224
 48194|  br i1 %1040, label %1070, label %1041                                                                                 ;L742<224
 48195| 
 48196| 1041: ; preds = %1037
 48197|     ;; self = ptr %1014
 48198|     ;; f = ptr %1014
 48199|     ;; x = ptr %1014
 48200|  %1042 = gep %1014, i64 1184                                                                                           ;L1162<225
 48201|  %1043 = load i64, ptr %1042, , !!8                                                                                    ;L1162<225
 48202|  %1044 = gep %1014, i64 1192                                                                                           ;L1162<225
 48203|  %1045 = load i64, ptr %1044, , !!8                                                                                    ;L1162<225
 48207|     ;; caster = ptr %1014
 48208|     ;; self = ptr %1014
 48209|  %1046 = gep %1014, i64 1480                                                                                           ;L26<225<1162<225
 48210|  %1047 = load i64, ptr %1046, , !!8                                                                                    ;L26<225<1162<225
 48211|  %1048 = gep %1014, i64 1080                                                                                           ;L26<225<1162<225
 48212|  %1049 = load i64, ptr %1048, , !!8                                                                                    ;L26<225<1162<225
 48213|  %1050 = gep %1014, i64 1136                                                                                           ;L1511<225<1162<225
 48214|  %1051 = load i32, ptr %1050, , !!8                                                                                    ;L1511<225<1162<225
 48215|     ;; mult = i32 %1051
 48216|  %1052 = icmp eq i32 %1051, 0                                                                                          ;L1512<225<1162<225
 48217|  br i1 %1052, label %1053, label %1056                                                                                 ;L1512<225<1162<225
 48218| 
 48219| 1053: ; preds = %1041
 48220|  %1054 = gep %1014, i64 1664                                                                                           ;L1513<225<1162<225
 48221|  %1055 = load i64, ptr %1054, , !!8                                                                                    ;L1513<225<1162<225
 48222|  br label %1063                                                                                                        ;L1512<225<1162<225
 48223| 
 48224| 1056: ; preds = %1041
 48225|  %1057 = sext i32 %1051 to i64                                                                                         ;L1511<225<1162<225
 48226|     ;; mult = i64 %1057
 48227|  %1058 = gep %1014, i64 1664                                                                                           ;L1515<225<1162<225
 48228|  %1059 = load i64, ptr %1058, , !!8                                                                                    ;L1515<225<1162<225
 48229|  %1060 = add nsw i64 %1057, 100                                                                                        ;L1515<225<1162<225
 48230|  %1061 = mul i64 %1059, %1060                                                                                          ;L1515<225<1162<225
 48231|  %1062 = udiv i64 %1061, 100                                                                                           ;L1515<225<1162<225
 48232|  br label %1063                                                                                                        ;L1512<225<1162<225
 48233| 
 48234| 1063: ; preds = %1056, %1053
 48235|  %1064 = phi i64 [ %1055, %1053 ], [ %1062, %1056 ]                                                                    ;L0<225<1162<225
 48236|  %1065 = add i64 %1047, -1                                                                                             ;L26<225<1162<225
 48237|  %1066 = mul i64 %1065, %1045                                                                                          ;L26<225<1162<225
 48238|  %1067 = add i64 %1049, %1043                                                                                          ;L26<225<1162<225
 48239|  %1068 = add i64 %1067, %1066                                                                                          ;L26<225<1162<225
 48240|  %1069 = add i64 %1068, %1064                                                                                          ;L225<1162<225
 48241|  br label %1070                                                                                                        ;L225<1162<225
 48242| 
 48243| 1070: ; preds = %1063, %1037
 48244|  %1071 = phi i64 [ 0, %1037 ], [ %1069, %1063 ]                                                                        ;L0<226
 48245|     ;; tower_attack_range = i64 %1071
 48246|     ;; team = !DIArgList(i64 1, i64 %190)
 48247|     ;; team = !DIArgList(i64 1, i64 %190)
 48248|  %1072 = sub nuw nsw i64 1, %190                                                                                       ;L227
 48249|     ;; team = i64 %1072
 48250|     ;; team = i64 %1072
 48251|  %1073 = getelementptr [5 x ptr], ptr %197, i64 %1072                                                                  ;L1905<227
 48252|     ;; self = ptr undef
 48253|     ;; self = ptr undef
 48254|  %1074 = gep %5, i64 16                                                                                                ;L228
 48255|  %1075 = load ptr, ptr %1074, , !!8, !!8                                                                               ;L228
 48257|     ;; f[8..+8] = ptr undef
 48258|     ;; f[16..+8] = ptr %454
 48259|     ;; f[24..+8] = ptr %456
 48260|     ;; f[32..+8] = ptr %1075
 48261|     ;; f[40..+8] = ptr %4
 48263|     ;; fold[8..+8] = ptr undef
 48264|     ;; fold[16..+8] = ptr %454
 48265|     ;; fold[24..+8] = ptr %456
 48266|     ;; fold[32..+8] = ptr %1075
 48267|     ;; fold[40..+8] = ptr %4
 48271|     ;; f[16..+8] = ptr undef
 48272|     ;; f[24..+8] = ptr %454
 48273|     ;; f[32..+8] = ptr %456
 48274|     ;; f[40..+8] = ptr %1075
 48275|     ;; f[48..+8] = ptr %4
 48276|     ;; self = ptr undef
 48279|     ;; self = ptr undef
 48280|     ;; count = i64 1
 48281|     ;; ptr = ptr %1073
 48282|     ;; self = ptr %1073
 48283|     ;; end_or_len = ptr %1073
 48286|  %1076 = getelementptr { { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, [5 x { i64, [2 x i64] }], [5 x { i64, { i8, [23 x i8] } }], [5 x { i8, [7 x i8], i64 }], [5 x i64], [5 x i64], [5 x i64], [5 x { i64, i64 }], i64, i64, i64, i64, i64, [5 x i32], i8, [3 x i8] }, ptr %1075, i64 %1072
 48287|  br label %1077                                                                                                        ;L180<2493<138<2897<228
 48288| 
 48289| 1077: ; preds = %1122, %1070
 48290|  %1078 = phi i64 [ 0, %1070 ], [ %1080, %1122 ]
 48291|  %1079 = gep %1073, i64 %1078                                                                                          ;L656<185<2493<138<2897<228
 48292|     ;; ptr = ptr %1079
 48293|  %1080 = add nuw nsw i64 %1078, 8                                                                                      ;L656<185<2493<138<2897<228
 48294|     ;; x = ptr %1079
 48295|  %1081 = load ptr, ptr %1079, , !!52496, !!8                                                                           ;L2494<138<2897<228
 48296|     ;; f = ptr undef
 48300|  %1082 = icmp eq ptr %1081, null                                                                                       ;L49<2494<138<2897<228
 48301|  br i1 %1082, label %1122, label %1083                                                                                 ;L49<2494<138<2897<228
 48302| 
 48303| 1083: ; preds = %1077
 48304|     ;; x = ptr %1081
 48308|     ;; x = ptr %1081
 48314|     ;; c = ptr %1081
 48315|     ;; other = ptr %1081
 48316|     ;; self = ptr %1081
 48318|  %1084 = load i64, ptr %1017, , !!52554, !!8                                                                           ;L2158<229<2893<50<2494<138<2897<228
 48319|     ;; x1 = i64 %1084
 48320|     ;; self = i64 %1084
 48321|  %1085 = load i64, ptr %1019, , !!52554, !!8                                                                           ;L2158<229<2893<50<2494<138<2897<228
 48322|     ;; y1 = i64 %1085
 48323|     ;; self = i64 %1085
 48324|  %1086 = gep %1081, i64 1632                                                                                           ;L2158<229<2893<50<2494<138<2897<228
 48325|  %1087 = load i64, ptr %1086, , !!52580, !!8                                                                           ;L2158<229<2893<50<2494<138<2897<228
 48326|     ;; x2 = i64 %1087
 48327|     ;; other = i64 %1087
 48328|  %1088 = gep %1081, i64 1640                                                                                           ;L2158<229<2893<50<2494<138<2897<228
 48329|  %1089 = load i64, ptr %1088, , !!52580, !!8                                                                           ;L2158<229<2893<50<2494<138<2897<228
 48330|     ;; y2 = i64 %1089
 48331|     ;; other = i64 %1089
 48335|  %1090 = gep %1081, i64 1136                                                                                           ;L1511<230<2893<50<2494<138<2897<228
 48336|  %1091 = load i32, ptr %1090, , !!52580, !!8                                                                           ;L1511<230<2893<50<2494<138<2897<228
 48337|     ;; mult = i32 %1091
 48338|  %1092 = icmp eq i32 %1091, 0                                                                                          ;L1512<230<2893<50<2494<138<2897<228
 48339|  br i1 %1092, label %1093, label %1096                                                                                 ;L1512<230<2893<50<2494<138<2897<228
 48340| 
 48341| 1093: ; preds = %1083
 48342|  %1094 = gep %1081, i64 1664                                                                                           ;L1513<230<2893<50<2494<138<2897<228
 48343|  %1095 = load i64, ptr %1094, , !!52580, !!8                                                                           ;L1513<230<2893<50<2494<138<2897<228
 48344|  br label %1103                                                                                                        ;L1512<230<2893<50<2494<138<2897<228
 48345| 
 48346| 1096: ; preds = %1083
 48347|  %1097 = sext i32 %1091 to i64                                                                                         ;L1511<230<2893<50<2494<138<2897<228
 48348|     ;; mult = i64 %1097
 48349|  %1098 = gep %1081, i64 1664                                                                                           ;L1515<230<2893<50<2494<138<2897<228
 48350|  %1099 = load i64, ptr %1098, , !!52580, !!8                                                                           ;L1515<230<2893<50<2494<138<2897<228
 48351|  %1100 = add nsw i64 %1097, 100                                                                                        ;L1515<230<2893<50<2494<138<2897<228
 48352|  %1101 = mul i64 %1099, %1100                                                                                          ;L1515<230<2893<50<2494<138<2897<228
 48353|  %1102 = udiv i64 %1101, 100                                                                                           ;L1515<230<2893<50<2494<138<2897<228
 48354|  br label %1103                                                                                                        ;L1512<230<2893<50<2494<138<2897<228
 48355| 
 48356| 1103: ; preds = %1096, %1093
 48357|  %1104 = phi i64 [ %1095, %1093 ], [ %1102, %1096 ]                                                                    ;L0<230<2893<50<2494<138<2897<228
 48358|     ;; range_with_enemy = !DIArgList(i64 %1104, i64 %1071)
 48362|  %1105 = invoke zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %1076, ptr %454, ptr %456, ptr %4, ptr %1081)
 48363|  to label %1106 unwind label %679                                                                                      ;L231<2893<50<2494<138<2897<228
 48364| 
 48365| 1106: ; preds = %1103
 48366|  %1107 = icmp ult i64 %1085, %1089                                                                                     ;L3147<8<2158<229<2893<50<2494<138<2897<228
 48367|  %1108 = sub nuw i64 %1089, %1085                                                                                      ;L3147<8<2158<229<2893<50<2494<138<2897<228
 48368|  %1109 = sub nuw i64 %1085, %1089                                                                                      ;L3147<8<2158<229<2893<50<2494<138<2897<228
 48369|  %1110 = select i1 %1107, i64 %1108, i64 %1109                                                                         ;L3147<8<2158<229<2893<50<2494<138<2897<228
 48370|     ;; dy = i64 %1110
 48371|  %1111 = mul i64 %1110, %1110                                                                                          ;L9<2158<229<2893<50<2494<138<2897<228
 48372|  %1112 = icmp ult i64 %1084, %1087                                                                                     ;L3147<7<2158<229<2893<50<2494<138<2897<228
 48373|  %1113 = sub nuw i64 %1087, %1084                                                                                      ;L3147<7<2158<229<2893<50<2494<138<2897<228
 48374|  %1114 = sub nuw i64 %1084, %1087                                                                                      ;L3147<7<2158<229<2893<50<2494<138<2897<228
 48375|  %1115 = select i1 %1112, i64 %1113, i64 %1114                                                                         ;L3147<7<2158<229<2893<50<2494<138<2897<228
 48376|     ;; dx = i64 %1115
 48377|     ;; enemy_dist = !DIArgList(i64 %1111, i64 %1115, i64 %1115)
 48378|  %1116 = mul i64 %1115, %1115                                                                                          ;L9<2158<229<2893<50<2494<138<2897<228
 48379|     ;; enemy_dist = !DIArgList(i64 %1111, i64 %1116)
 48380|  %1117 = add i64 %1111, %1116                                                                                          ;L9<2158<229<2893<50<2494<138<2897<228
 48381|     ;; enemy_dist = i64 %1117
 48382|  %1118 = add i64 %1104, %1071                                                                                          ;L230<2893<50<2494<138<2897<228
 48383|     ;; range_with_enemy = i64 %1118
 48384|  %1119 = mul i64 %1118, %1118                                                                                          ;L231<2893<50<2494<138<2897<228
 48385|  %1120 = icmp ule i64 %1117, %1119                                                                                     ;L231<2893<50<2494<138<2897<228
 48386|  %1121 = select i1 %1105, i1 %1120, i1 false                                                                           ;L231<2893<50<2494<138<2897<228
 48387|  br i1 %1121, label %1124, label %1122                                                                                 ;L2494<138<2897<228
 48388| 
 48389| 1122: ; preds = %1106, %1077
 48390|     ;; self = ptr undef
 48391|     ;; count = i64 1
 48392|     ;; ptr = !DIArgList(ptr %1073, i64 %1080)
 48393|     ;; self = !DIArgList(ptr %1073, i64 %1080)
 48394|     ;; end_or_len = ptr %1073
 48397|  %1123 = icmp eq i64 %1080, 40                                                                                         ;L1714<180<2493<138<2897<228
 48398|  br i1 %1123, label %1124, label %1077                                                                                 ;L180<2493<138<2897<228
 48399| 
 48400| 1124: ; preds = %1122, %1106
 48401|  %1125 = phi i8 [ 0, %1122 ], [ 1, %1106 ]                                                                             ;L1714<180<2493<138<2897<228
 48402|     ;; enemy_in_tower_range = i8 %1125
 48403|  %1126 = gep %200, i64 1576                                                                                            ;L234
 48404|  %1127 = load i64, ptr %1126, , !!8                                                                                    ;L234
 48405|  %1128 = icmp eq i64 %1127, 0                                                                                          ;L234
 48406|  br i1 %1128, label %1149, label %1129                                                                                 ;L234
 48407| 
 48408| 1129: ; preds = %1124
 48409|  %1130 = gep %200, i64 1648                                                                                            ;L234
 48410|  %1131 = load i64, ptr %1130, , !!8                                                                                    ;L234
 48411|  %1132 = mul i64 %1131, 100                                                                                            ;L234
 48412|  %1133 = udiv i64 %1132, %1127                                                                                         ;L234
 48413|     ;; hp_ratio = i64 %1133
 48414|     ;; self[0..+8] = ptr %1073
 48415|     ;; self[8..+8] = ptr %1073
 48416|     ;; self[16..+8] = ptr %454
 48417|     ;; self[24..+8] = ptr %456
 48418|     ;; self[32..+8] = ptr %1075
 48419|     ;; self[40..+8] = ptr %4
 48420|     ;; init = i64 0
 48423|     ;; self[0..+8] = ptr %1073
 48424|     ;; iter[0..+8] = ptr %1073
 48425|     ;; self[0..+8] = ptr %1073
 48426|     ;; self[8..+8] = ptr %1073
 48427|     ;; iter[8..+8] = ptr %1073
 48428|     ;; self[8..+8] = ptr %1073
 48429|     ;; self[16..+8] = ptr %454
 48430|     ;; iter[16..+8] = ptr %454
 48431|     ;; self[16..+8] = ptr %454
 48432|     ;; self[24..+8] = ptr %456
 48433|     ;; iter[24..+8] = ptr %456
 48434|     ;; self[24..+8] = ptr %456
 48435|     ;; self[32..+8] = ptr %1075
 48436|     ;; iter[32..+8] = ptr %1075
 48437|     ;; self[32..+8] = ptr %1075
 48438|     ;; self[40..+8] = ptr %4
 48439|     ;; iter[40..+8] = ptr %4
 48440|     ;; self[40..+8] = ptr %4
 48441|     ;; fold[0..+8] = ptr %454
 48442|     ;; fold[8..+8] = ptr %456
 48443|     ;; fold[16..+8] = ptr %1075
 48444|     ;; fold[24..+8] = ptr %4
 48445|     ;; self[0..+8] = ptr %1073
 48446|     ;; self[8..+8] = ptr %1073
 48447|     ;; init = i64 0
 48448|     ;; f[0..+8] = ptr %454
 48449|     ;; f[8..+8] = ptr %456
 48450|     ;; f[16..+8] = ptr %1075
 48451|     ;; f[24..+8] = ptr %4
 48452|     ;; self[0..+8] = ptr %1073
 48453|     ;; self[8..+8] = ptr %1073
 48454|     ;; init = i64 0
 48455|     ;; rhs = i64 1
 48456|     ;; acc = i64 0
 48457|     ;; i = i64 0
 48458|     ;; self = i64 0
 48459|     ;; len = i64 5
 48460|  br label %1134                                                                                                        ;L28<146<128<52<3674<142<237
 48461| 
 48462| 1134: ; preds = %1145, %1129
 48463|  %1135 = phi i64 [ 0, %1129 ], [ %1147, %1145 ]                                                                        ;L0<146<128<52<3674<142<237
 48464|  %1136 = phi i64 [ 0, %1129 ], [ %1146, %1145 ]                                                                        ;L0<146<128<52<3674<142<237
 48465|     ;; acc = i64 %1136
 48466|     ;; self = i64 %1135
 48467|     ;; i = i64 %1135
 48468|     ;; self = ptr %1073
 48469|     ;; count = i64 %1135
 48470|  %1137 = getelementptr ptr, ptr %1073, i64 %1135                                                                       ;L656<279<146<128<52<3674<142<237
 48471|  %1138 = load ptr, ptr %1137, , !!52722, !!8                                                                           ;L279<146<128<52<3674<142<237
 48473|     ;; acc = i64 %1136
 48475|  %1139 = icmp eq ptr %1138, null                                                                                       ;L39<279<146<128<52<3674<142<237
 48476|  br i1 %1139, label %1145, label %1140                                                                                 ;L39<279<146<128<52<3674<142<237
 48477| 
 48478| 1140: ; preds = %1134
 48479|     ;; x = ptr %1138
 48481|     ;; acc = i64 %1136
 48482|     ;; elt = ptr %1138
 48483|     ;; x = ptr %1138
 48488|  %1141 = invoke zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %1076, ptr %454, ptr %456, ptr %4, ptr %1138)
 48489|  to label %1142 unwind label %679                                                                                      ;L236<138<88<40<279<146<128<52<3674<142<237
 48490| 
 48491| 1142: ; preds = %1140
 48492|  %1143 = zext i1 %1141 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<237
 48494|     ;; a = i64 %1136
 48495|     ;; b = i64 %1143
 48496|  %1144 = add i64 %1136, %1143                                                                                          ;L55<88<40<279<146<128<52<3674<142<237
 48497|  br label %1145                                                                                                        ;L42<279<146<128<52<3674<142<237
 48498| 
 48499| 1145: ; preds = %1142, %1134
 48500|  %1146 = phi i64 [ %1144, %1142 ], [ %1136, %1134 ]                                                                    ;L0<279<146<128<52<3674<142<237
 48501|     ;; acc = i64 %1146
 48502|  %1147 = add nuw i64 %1135, 1                                                                                          ;L971<283<146<128<52<3674<142<237
 48503|     ;; i = i64 %1147
 48504|     ;; self = i64 %1147
 48505|  %1148 = icmp eq i64 %1147, 5                                                                                          ;L284<146<128<52<3674<142<237
 48506|  br i1 %1148, label %1150, label %1134                                                                                 ;L284<146<128<52<3674<142<237
 48507| 
 48508| 1149: ; preds = %1124
 48509|  invoke void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.282069a2ed2ad3a275929b639963fb55.147) #32
 48510|  to label %203 unwind label %679                                                                                       ;L234
 48511| 
 48512| 1150: ; preds = %1145
 48513|     ;; enemy_count = i64 %1146
 48514|     ;; self = ptr %1
 48516|  %1151 = gep %456, i64 40                                                                                              ;L240
 48517|  %1152 = load ptr, ptr %1151, , !!8                                                                                    ;L240
 48518|  %1153 = invoke i64 %1152(ptr %454)
 48519|  to label %1154 unwind label %679                                                                                      ;L240
 48520| 
 48521| 1154: ; preds = %1150
 48522|  %1155 = gep %1, i64 2136                                                                                              ;L239
 48523|     ;; self = ptr %1155
 48524|  %1156 = uitofp nneg i64 %1035 to double                                                                               ;L242
 48525|     ;; self = double %1156
 48526|     ;; x = double %1156
 48527|  %1157 = call double @llvm.sqrt.f64(double %1156)                                                                      ;L2082<387<242
 48528|  %1158 = call i64 @llvm.fptoui.sat.i64.f64(double %1157)                                                               ;L242
 48529|  %1159 = gep %65, i64 176                                                                                              ;L239
 48530|  store i64 %1153, ptr %1159,                                                                                           ;L239
 48531|  store i64 -9223372036854775796, ptr %65,                                                                              ;L239
 48532|  %1160 = gep %65, i64 8                                                                                                ;L239
 48533|  store i64 %1158, ptr %1160,                                                                                           ;L239
 48534|  %1161 = gep %65, i64 16                                                                                               ;L239
 48535|  store i64 %1133, ptr %1161,                                                                                           ;L239
 48536|  %1162 = gep %65, i64 24                                                                                               ;L239
 48537|  store i64 %1146, ptr %1162,                                                                                           ;L239
 48538|  %1163 = gep %65, i64 32                                                                                               ;L239
 48539|  store i8 %1125, ptr %1163,                                                                                            ;L239
 48540|     ;; self = ptr %1155
 48541|     ;; self = ptr %1155
 48542|     ;; value = ptr %65
 48544|     ;; elem_size = i64 184
 48545|  %1164 = gep %1, i64 2152                                                                                              ;L1037<1004<239
 48546|  %1165 = load i64, ptr %1164, , !!52834, !!8                                                                           ;L1037<1004<239
 48547|     ;; len = i64 %1165
 48548|     ;; count = i64 %1165
 48549|     ;; self = ptr %1155
 48550|  %1166 = load i64, ptr %1155, , !!52834, !!8                                                                           ;L619<309<1040<1004<239
 48551|  %1167 = icmp eq i64 %1165, %1166                                                                                      ;L1040<1004<239
 48552|  br i1 %1167, label %1168, label %1172                                                                                 ;L1040<1004<239
 48553| 
 48554| 1168: ; preds = %1154
 48555|  invoke void @gc::simulation12ai_interface17PendingTraceEventE8grow_oneCshdEBA0ozCnw_7game_ai(ptr %1155)
 48556|  to label %1172 unwind label %1169, !!52834                                                                            ;L1041<1004<239
 48557| 
 48558| 1169: ; preds = %1168
 48559|  %1170 = cleanuppad within none []
 48560|  invoke fastcc void @core::ptr9drop_glueNtNtNtCs97f5S1uJLkH_9game_core10simulation12ai_interface17PendingTraceEventECshdEBA0ozCnw_7game_ai(ptr %65) #31 [ "funclet"(token %1170) ]
 48561|  to label %1171 unwind label %679                                                                                      ;L1050<1004<239
 48562| 
 48563| 1171: ; preds = %1169
 48564|  cleanupret from %1170 unwind label %679
 48565| 
 48566| 1172: ; preds = %1168, %1154
 48567|  %1173 = gep %1, i64 2144                                                                                              ;L614<609<296<2052<1044<1004<239
 48568|  %1174 = load ptr, ptr %1173, , !!52834, !!8, !!8                                                                      ;L614<609<296<2052<1044<1004<239
 48569|     ;; self = ptr %1174
 48570|  %1175 = getelementptr { { i64, [21 x i64] }, i64 }, ptr %1174, i64 %1165                                              ;L961<1044<1004<239
 48571|     ;; end = ptr %1175
 48572|     ;; dst = ptr %1175
 48573|  call void @llvm.memcpy.p0.p0.i64(ptr %1175, ptr %65, i64 184, i1 false)                                               ;L1933<1045<1004<239
 48574|  %1176 = add i64 %1165, 1                                                                                              ;L1046<1004<239
 48575|  store i64 %1176, ptr %1164, , !!52834                                                                                 ;L1046<1004<239
 48577|  br label %688                                                                                                         ;L223
 48578| 
 48579| 1177: ; preds = %1204, %688
 48580|     ;; self = ptr %70
 48581|     ;; self = ptr %70
 48582|  %1178 = gep %70, i64 24                                                                                               ;L1617<1636<260
 48583|  %1179 = load i64, ptr %1178, , !!8                                                                                    ;L1617<1636<260
 48584|  %1180 = icmp eq i64 %1179, 0                                                                                          ;L260
 48585|  br i1 %1180, label %1300, label %1181                                                                                 ;L260
 48586| 
 48587| 1181: ; preds = %1177
 48590|  store ptr %454, ptr %57,                                                                                              ;L269
 48591|  %1182 = gep %57, i64 8                                                                                                ;L269
 48592|  store ptr %456, ptr %1182,                                                                                            ;L269
 48593|  %1183 = gep %57, i64 16                                                                                               ;L269
 48594|  store ptr %200, ptr %1183,                                                                                            ;L269
 48595|     ;; self = ptr %70
 48596|     ;; self = ptr %70
 48597|  %1184 = load ptr, ptr %70, , !!8, !!8                                                                                 ;L138<2073<277
 48598|     ;; p = ptr %1184
 48599|     ;; len = i64 %1179
 48600|     ;; count = i64 %1179
 48601|     ;; self[0..+8] = ptr %1184
 48602|     ;; slice[0..+8] = ptr %1184
 48603|     ;; self[8..+8] = i64 %1179
 48604|     ;; slice[8..+8] = i64 %1179
 48605|     ;; ptr = ptr %1184
 48606|     ;; self = ptr %1184
 48607|  %1185 = mul nuw nsw i64 %1179, 192                                                                                    ;L961<100<1042<277
 48608|  %1186 = gep %1184, i64 %1185                                                                                          ;L961<100<1042<277
 48609|     ;; f = ptr %57
 48610|     ;; self = ptr undef
 48611|     ;; self = ptr undef
 48612|     ;; count = i64 1
 48613|     ;; ptr = ptr %1184
 48614|     ;; self = ptr %1184
 48615|     ;; end_or_len = ptr %1186
 48618|  br label %1314                                                                                                        ;L180<314<277
 48619| 
 48620| 1187: ; preds = %688
 48621|     ;; self = ptr %70
 48622|     ;; self = ptr %70
 48623|  %1188 = load ptr, ptr %70, , !!8, !!8                                                                                 ;L138<2073<255
 48624|     ;; p = ptr %1188
 48625|  %1189 = gep %70, i64 24                                                                                               ;L2075<255
 48626|  %1190 = load i64, ptr %1189, , !!8                                                                                    ;L2075<255
 48627|     ;; len = i64 %1190
 48628|     ;; count = i64 %1190
 48629|     ;; self[0..+8] = ptr %1188
 48630|     ;; slice[0..+8] = ptr %1188
 48631|     ;; self[8..+8] = i64 %1190
 48632|     ;; slice[8..+8] = i64 %1190
 48633|     ;; ptr = ptr %1188
 48634|     ;; self = ptr %1188
 48635|  %1191 = getelementptr { i64, { [177 x i8], i8, [6 x i8] } }, ptr %1188, i64 %1190                                     ;L961<100<1042<255
 48636|     ;; iter[0..+8] = ptr %1188
 48637|     ;; iter[8..+8] = ptr %1191
 48638|  %1192 = gep %200, i64 1472
 48639|  %1193 = gep %8, i64 160
 48640|  %1194 = gep %17, i64 8
 48641|  %1195 = gep %17, i64 16
 48642|  %1196 = gep %63, i64 8
 48643|  %1197 = gep %63, i64 16
 48644|  %1198 = gep %10, i64 8
 48645|  %1199 = gep %61, i64 8
 48646|  %1200 = gep %61, i64 16
 48647|  %1201 = gep %60, i64 8
 48648|  %1202 = gep %60, i64 16
 48649|  %1203 = gep %60, i64 24
 48650|  br label %1204                                                                                                        ;L255
 48651| 
 48652| 1204: ; preds = %1295, %1187
 48653|  %1205 = phi ptr [ %1188, %1187 ], [ %1208, %1295 ]                                                                    ;L255
 48654|     ;; iter[0..+8] = ptr %1205
 48655|     ;; self = ptr undef
 48656|     ;; ptr = ptr %1205
 48657|     ;; self = ptr %1205
 48658|     ;; end_or_len = ptr %1191
 48661|  %1206 = icmp eq ptr %1205, %1191                                                                                      ;L1714<180<255
 48662|  br i1 %1206, label %1177, label %1207                                                                                 ;L180<255
 48663| 
 48664| 1207: ; preds = %1204
 48665|  %1208 = gep %1205, i64 192                                                                                            ;L656<185<255
 48666|     ;; iter[0..+8] = ptr %1208
 48668|  store ptr %1205, ptr %64,                                                                                             ;L255
 48669|     ;; action = ptr %1205
 48670|     ;; self = ptr %8
 48671|  %1209 = load i64, ptr %1192, , !!8                                                                                    ;L256
 48672|     ;; key = i64 %1209
 48674|  invoke void @_RNvMNtCs5gUUnHMsxBL_9hashbrown11rustc_entryINtNtB4_3map7HashMapjINtNtCs9LexZzt9XJB_5alloc3vec3VecNtNtB15_6string6StringENtNtCs9EYcZKFYzm_5ahash12random_state11RandomStateE11rustc_entryCshdEBA0ozCnw_7game_ai(ptr sret([24 x i8]) %17, ptr %1193, i64 %1209)
 48675|  to label %1210 unwind label %679                                                                                      ;L1014<256
 48676| 
 48677| 1210: ; preds = %1207
 48678|  %1211 = load ptr, ptr %17, , !!8                                                                                      ;L3008<1014<256
 48679|  %1212 = icmp eq ptr %1211, null                                                                                       ;L3008<1014<256
 48680|  br i1 %1212, label %1218, label %1213                                                                                 ;L3008<1014<256
 48681| 
 48682| 1213: ; preds = %1210
 48683|  %1214 = load i64, ptr %1194,                                                                                          ;L3010<1014<256
 48684|  %1215 = load i64, ptr %1195,                                                                                          ;L3010<1014<256
 48685|     ;; self[16..+8] = i64 %1215
 48686|     ;; self[8..+8] = i64 %1214
 48687|     ;; self[0..+8] = ptr %1211
 48690|  store i64 0, ptr %63,                                                                                                 ;L464<256
 48691|  store ptr inttoptr (i64 8 to ptr), ptr %1196,                                                                         ;L464<256
 48692|  store i64 0, ptr %1197,                                                                                               ;L464<256
 48693|     ;; default = ptr %63
 48696|     ;; entry[8..+8] = i64 %1214
 48697|     ;; self[8..+8] = i64 %1214
 48698|     ;; self[8..+8] = i64 %1214
 48699|     ;; entry[16..+8] = i64 %1215
 48700|     ;; self[16..+8] = i64 %1215
 48701|     ;; self[16..+8] = i64 %1215
 48702|     ;; entry[0..+8] = ptr %1211
 48703|     ;; self[0..+8] = ptr %1211
 48704|     ;; self[0..+8] = ptr %1211
 48706|  call void @llvm.memcpy.p0.p0.i64(ptr %1198, ptr %63, i64 24, i1 false), !!52959                                       ;L2519<256
 48707|  store i64 %1215, ptr %10, , !!52954                                                                                   ;L576<2911<2519<256
 48708|  %1216 = invoke ptr @_RNvMs6_NtCs5gUUnHMsxBL_9hashbrown3rawINtB5_8RawTableTjINtNtCs9LexZzt9XJB_5alloc3vec3VecNtNtBV_6string6StringEEE14insert_no_growCshdEBA0ozCnw_7game_ai(ptr %1211, i64 %1214, ptr %10)
 48709|  to label %1217 unwind label %679                                                                                      ;L576<2911<2519<256
 48710| 
 48711| 1217: ; preds = %1213
 48713|  br label %1224                                                                                                        ;L2521<256
 48714| 
 48715| 1218: ; preds = %1210
 48716|  %1219 = load ptr, ptr %1194, , !!8, !!8                                                                               ;L3009<1014<256
 48718|     ;; self[8..+8] = ptr %1219
 48719|     ;; self[0..+8] = ptr null
 48722|  store i64 0, ptr %63,                                                                                                 ;L464<256
 48723|  store ptr inttoptr (i64 8 to ptr), ptr %1196,                                                                         ;L464<256
 48724|  store i64 0, ptr %1197,                                                                                               ;L464<256
 48725|     ;; default = ptr %63
 48729|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %63)
 48730|  to label %1223 unwind label %1220, !!52959                                                                            ;L825<2521<256
 48731| 
 48732| 1220: ; preds = %1218
 48733|  %1221 = cleanuppad within none []
 48735|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %63) [ "funclet"(token %1221) ]
 48736|  to label %1222 unwind label %679                                                                                      ;L825<825<2521<256
 48737| 
 48738| 1222: ; preds = %1220
 48739|  cleanupret from %1221 unwind label %679
 48740| 
 48741| 1223: ; preds = %1218
 48743|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %63)
 48744|  to label %1224 unwind label %679                                                                                      ;L825<825<2521<256
 48745| 
 48746| 1224: ; preds = %1223, %1217
 48747|  %1225 = phi ptr [ %1216, %1217 ], [ %1219, %1223 ]
 48748|  %1226 = gep %1225, i64 -24                                                                                            ;L0<256
 48749|     ;; self = ptr %1226
 48754|     ;; self = ptr %1205
 48755|  %1227 = gep %1205, i64 185                                                                                            ;L309<256
 48756|  %1228 = load i8, ptr %1227, , !!52972, !!8                                                                            ;L309<256
 48757|  %1229 = icmp ne i8 %1228, 10                                                                                          ;L309<256
 48758|  call void @llvm.assume(i1 %1229)                                                                                      ;L309<256
 48759|  %1230 = add nsw i8 %1228, -3                                                                                          ;L309<256
 48760|  %1231 = icmp samesign ugt i8 %1228, 2                                                                                 ;L309<256
 48761|  %1232 = select i1 %1231, i8 %1230, i8 7                                                                               ;L309<256
 48762|  switch i8 %1232, label %1233 [
 48763|  i8 0, label %1284
 48764|  i8 1, label %1284
 48765|  i8 2, label %1234
 48766|  i8 3, label %1237
 48767|  i8 4, label %1240
 48768|  i8 5, label %1284
 48769|  i8 6, label %1245
 48770|  i8 7, label %1250
 48771|  i8 8, label %1255
 48772|  i8 9, label %1260
 48773|  i8 10, label %1265
 48774|  i8 11, label %1268
 48775|  i8 12, label %1271
 48776|  i8 13, label %1274
 48777|  i8 14, label %1277
 48778|  i8 15, label %1280
 48779|  i8 16, label %1283
 48780|  ]                                                                                                                     ;L309<256
 48781| 
 48782| 1233: ; preds = %1224
 48783|  unreachable                                                                                                           ;L309<256
 48784| 
 48785| 1234: ; preds = %1224
 48786|     ;; action = ptr %1205
 48787|     ;; self = ptr %1205
 48788|  %1235 = gep %1205, i64 16                                                                                             ;L285<316<256
 48789|  %1236 = load i64, ptr %1235, , !!52972, !!8                                                                           ;L285<316<256
 48790|  store i64 %1236, ptr %1199, , !!52975                                                                                 ;L285<316<256
 48791|  br label %1284                                                                                                        ;L316<256
 48792| 
 48793| 1237: ; preds = %1224
 48794|     ;; action = ptr %1205
 48795|     ;; self = ptr %1205
 48796|  %1238 = gep %1205, i64 16                                                                                             ;L491<313<256
 48797|  %1239 = load i64, ptr %1238, , !!52972, !!8                                                                           ;L491<313<256
 48798|  store i64 %1239, ptr %1199, , !!52975                                                                                 ;L491<313<256
 48799|  br label %1284                                                                                                        ;L313<256
 48800| 
 48801| 1240: ; preds = %1224
 48802|     ;; action = ptr %1205
 48803|     ;; self = ptr %1205
 48804|  %1241 = gep %1205, i64 24                                                                                             ;L665<314<256
 48805|  %1242 = load i64, ptr %1241, , !!52972, !!8                                                                           ;L665<314<256
 48806|  %1243 = gep %1205, i64 32                                                                                             ;L665<314<256
 48807|  %1244 = load i64, ptr %1243, , !!52972, !!8                                                                           ;L665<314<256
 48808|  store i64 %1242, ptr %1199, , !!52975                                                                                 ;L665<314<256
 48809|  store i64 %1244, ptr %1200, , !!52975                                                                                 ;L665<314<256
 48810|  br label %1284                                                                                                        ;L314<256
 48811| 
 48812| 1245: ; preds = %1224
 48813|     ;; action = ptr %1205
 48814|     ;; self = ptr %1205
 48815|  %1246 = gep %1205, i64 16                                                                                             ;L800<312<256
 48816|  %1247 = load i64, ptr %1246, , !!52972, !!8                                                                           ;L800<312<256
 48817|  %1248 = gep %1205, i64 24                                                                                             ;L800<312<256
 48818|  %1249 = load i64, ptr %1248, , !!52972, !!8                                                                           ;L800<312<256
 48819|  store i64 %1247, ptr %1199, , !!52975                                                                                 ;L800<312<256
 48820|  store i64 %1249, ptr %1200, , !!52975                                                                                 ;L800<312<256
 48821|  br label %1284                                                                                                        ;L312<256
 48822| 
 48823| 1250: ; preds = %1224
 48824|     ;; action = ptr %1205
 48825|     ;; self = ptr %1205
 48826|  %1251 = gep %1205, i64 56                                                                                             ;L1035<317<256
 48827|  %1252 = load i64, ptr %1251, , !!52972, !!8                                                                           ;L1035<317<256
 48828|  %1253 = gep %1205, i64 64                                                                                             ;L1035<317<256
 48829|  %1254 = load i64, ptr %1253, , !!52972, !!8                                                                           ;L1035<317<256
 48830|  store i64 %1252, ptr %1199, , !!52975                                                                                 ;L1035<317<256
 48831|  store i64 %1254, ptr %1200, , !!52975                                                                                 ;L1035<317<256
 48832|  br label %1284                                                                                                        ;L317<256
 48833| 
 48834| 1255: ; preds = %1224
 48835|     ;; action = ptr %1205
 48836|     ;; self = ptr %1205
 48837|  %1256 = gep %1205, i64 16                                                                                             ;L1123<318<256
 48838|  %1257 = load i64, ptr %1256, , !!52972, !!8                                                                           ;L1123<318<256
 48839|  %1258 = gep %1205, i64 24                                                                                             ;L1123<318<256
 48840|  %1259 = load i64, ptr %1258, , !!52972, !!8                                                                           ;L1123<318<256
 48841|  store i64 %1257, ptr %1199, , !!52975                                                                                 ;L1123<318<256
 48842|  store i64 %1259, ptr %1200, , !!52975                                                                                 ;L1123<318<256
 48843|  br label %1284                                                                                                        ;L318<256
 48844| 
 48845| 1260: ; preds = %1224
 48846|     ;; action = ptr %1205
 48847|     ;; self = ptr %1205
 48848|  %1261 = gep %1205, i64 32                                                                                             ;L1241<319<256
 48849|  %1262 = load i64, ptr %1261, , !!52972, !!8                                                                           ;L1241<319<256
 48850|  %1263 = gep %1205, i64 40                                                                                             ;L1241<319<256
 48851|  %1264 = load i64, ptr %1263, , !!52972, !!8                                                                           ;L1241<319<256
 48852|  store i64 %1262, ptr %1199, , !!52975                                                                                 ;L1241<319<256
 48853|  store i64 %1264, ptr %1200, , !!52975                                                                                 ;L1241<319<256
 48854|  br label %1284                                                                                                        ;L319<256
 48855| 
 48856| 1265: ; preds = %1224
 48857|     ;; action = ptr %1205
 48858|     ;; self = ptr %1205
 48859|  %1266 = gep %1205, i64 16                                                                                             ;L653<320<256
 48860|  %1267 = load i64, ptr %1266, , !!52972, !!8                                                                           ;L653<320<256
 48861|  store i64 %1267, ptr %1199, , !!52975                                                                                 ;L653<320<256
 48862|  br label %1284                                                                                                        ;L320<256
 48863| 
 48864| 1268: ; preds = %1224
 48865|     ;; action = ptr %1205
 48866|     ;; self = ptr %1205
 48867|  %1269 = gep %1205, i64 104                                                                                            ;L404<321<256
 48868|  %1270 = load i64, ptr %1269, , !!52972, !!8                                                                           ;L404<321<256
 48869|  store i64 %1270, ptr %1199, , !!52975                                                                                 ;L404<321<256
 48870|  br label %1284                                                                                                        ;L321<256
 48871| 
 48872| 1271: ; preds = %1224
 48873|     ;; action = ptr %1205
 48874|     ;; self = ptr %1205
 48875|  %1272 = gep %1205, i64 16                                                                                             ;L94<322<256
 48876|  %1273 = load i64, ptr %1272, , !!52972, !!8                                                                           ;L94<322<256
 48877|  store i64 %1273, ptr %1199, , !!52975                                                                                 ;L94<322<256
 48878|  br label %1284                                                                                                        ;L322<256
 48879| 
 48880| 1274: ; preds = %1224
 48881|     ;; action = ptr %1205
 48882|     ;; self = ptr %1205
 48883|  %1275 = gep %1205, i64 16                                                                                             ;L160<323<256
 48884|  %1276 = load i64, ptr %1275, , !!52972, !!8                                                                           ;L160<323<256
 48885|  store i64 %1276, ptr %1199, , !!52975                                                                                 ;L160<323<256
 48886|  br label %1284                                                                                                        ;L323<256
 48887| 
 48888| 1277: ; preds = %1224
 48889|     ;; action = ptr %1205
 48890|     ;; self = ptr %1205
 48891|  %1278 = gep %1205, i64 16                                                                                             ;L222<324<256
 48892|  %1279 = load i64, ptr %1278, , !!52972, !!8                                                                           ;L222<324<256
 48893|  store i64 %1279, ptr %1199, , !!52975                                                                                 ;L222<324<256
 48894|  br label %1284                                                                                                        ;L324<256
 48895| 
 48896| 1280: ; preds = %1224
 48897|     ;; action = ptr %1205
 48898|     ;; self = ptr %1205
 48899|  %1281 = gep %1205, i64 16                                                                                             ;L287<325<256
 48900|  %1282 = load i64, ptr %1281, , !!52972, !!8                                                                           ;L287<325<256
 48901|  store i64 %1282, ptr %1199, , !!52975                                                                                 ;L287<325<256
 48902|  br label %1284                                                                                                        ;L325<256
 48903| 
 48904| 1283: ; preds = %1224
 48905|  br label %1284                                                                                                        ;L326<256
 48906| 
 48907| 1284: ; preds = %1283, %1280, %1277, %1274, %1271, %1268, %1265, %1260, %1255, %1250, %1245, %1240, %1237, %1234, %1224, %1224, %1224
 48908|  %1285 = phi i64 [ 10, %1283 ], [ 9, %1280 ], [ 8, %1277 ], [ 7, %1274 ], [ 6, %1271 ], [ 4, %1268 ], [ 2, %1265 ], [ 3, %1260 ], [ 3, %1255 ], [ 3, %1250 ], [ 1, %1245 ], [ 0, %1224 ], [ 3, %1240 ], [ 2, %1237 ], [ 2, %1234 ], [ 0, %1224 ], [ 0, %1224 ]
 48909|  store i64 %1285, ptr %61, , !!52975                                                                                   ;L0<256
 48910|     ;; args[0..+8] = ptr %61
 48911|     ;; args[8..+8] = ptr %64
 48913|  store ptr %61, ptr %60,                                                                                               ;L256
 48914|  store ptr @gc::simulation4game10blackboardNtB5_11SmallActionNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %1201,         ;L256
 48915|  store ptr %64, ptr %1202,                                                                                             ;L256
 48916|  store ptr @core::fmtRxNtB6_7Display3fmtCshdEBA0ozCnw_7game_ai, ptr %1203,                                             ;L256
 48917|     ;; args[0..+8] = ptr @anon.282069a2ed2ad3a275929b639963fb55.139
 48918|     ;; args[8..+8] = ptr %60
 48919|     ;; self[0..+8] = ptr null
 48920|     ;; self[8..+8] = i64 undef
 48924|  invoke void @_RNvNvNtCs9LexZzt9XJB_5alloc3fmt6format12format_inner(ptr sret([24 x i8]) %62, ptr @anon.282069a2ed2ad3a275929b639963fb55.139, ptr %60)
 48925|  to label %1286 unwind label %679                                                                                      ;L659<1275<659<256
 48926| 
 48927| 1286: ; preds = %1284
 48930|     ;; self = ptr %1226
 48931|     ;; self = ptr %1226
 48932|     ;; value = ptr %62
 48934|     ;; elem_size = i64 24
 48935|  %1287 = gep %1225, i64 -8                                                                                             ;L1037<1004<256
 48936|  %1288 = load i64, ptr %1287, , !!53153, !!8                                                                           ;L1037<1004<256
 48937|     ;; len = i64 %1288
 48938|     ;; count = i64 %1288
 48939|     ;; self = ptr %1226
 48940|  %1289 = load i64, ptr %1226, , !!53153, !!8                                                                           ;L619<309<1040<1004<256
 48941|  %1290 = icmp eq i64 %1288, %1289                                                                                      ;L1040<1004<256
 48942|  br i1 %1290, label %1291, label %1295                                                                                 ;L1040<1004<256
 48943| 
 48944| 1291: ; preds = %1286
 48945|  invoke void @_RNvMs3_NtCs9LexZzt9XJB_5alloc7raw_vecINtB5_6RawVecNtNtB7_6string6StringE8grow_oneCszutcqs0z2F_10sys_locale(ptr %1226)
 48946|  to label %1295 unwind label %1292, !!53153                                                                            ;L1041<1004<256
 48947| 
 48948| 1292: ; preds = %1291
 48949|  %1293 = cleanuppad within none []
 48950|  invoke fastcc void @core::ptr9drop_glueNtNtCs9LexZzt9XJB_5alloc6string6StringECshdEBA0ozCnw_7game_ai(ptr %62) #31 [ "funclet"(token %1293) ]
 48951|  to label %1294 unwind label %679                                                                                      ;L1050<1004<256
 48952| 
 48953| 1294: ; preds = %1292
 48954|  cleanupret from %1293 unwind label %679
 48955| 
 48956| 1295: ; preds = %1291, %1286
 48957|  %1296 = gep %1225, i64 -16                                                                                            ;L614<609<296<2052<1044<1004<256
 48958|  %1297 = load ptr, ptr %1296, , !!53153, !!8, !!8                                                                      ;L614<609<296<2052<1044<1004<256
 48959|     ;; self = ptr %1297
 48960|  %1298 = getelementptr { { { { i64, ptr, {} }, {} }, i64 } }, ptr %1297, i64 %1288                                     ;L961<1044<1004<256
 48961|     ;; end = ptr %1298
 48962|     ;; dst = ptr %1298
 48963|  call void @llvm.memcpy.p0.p0.i64(ptr %1298, ptr %62, i64 24, i1 false)                                                ;L1933<1045<1004<256
 48964|  %1299 = add i64 %1288, 1                                                                                              ;L1046<1004<256
 48965|  store i64 %1299, ptr %1287, , !!53153                                                                                 ;L1046<1004<256
 48967|  br label %1204                                                                                                        ;L255
 48968| 
 48969| 1300: ; preds = %1177
 48970|     ;; args[0..+8] = ptr %415
 48971|     ;; args[8..+8] = ptr %526
 48972|     ;; args[16..+8] = ptr %112
 48974|  store ptr %415, ptr %59,                                                                                              ;L261
 48975|  %1301 = gep %59, i64 8                                                                                                ;L261
 48976|  store ptr @ai::plan_legacy9team_planNtB5_8TeamPlanNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %1301,                   ;L261
 48977|  %1302 = gep %59, i64 16                                                                                               ;L261
 48978|  store ptr %526, ptr %1302,                                                                                            ;L261
 48979|  %1303 = gep %59, i64 24                                                                                               ;L261
 48980|  store ptr @ai::plan_legacy5typesNtB5_7BigPlanNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %1303,                        ;L261
 48981|  %1304 = gep %59, i64 32                                                                                               ;L261
 48982|  store ptr %112, ptr %1304,                                                                                            ;L261
 48983|  %1305 = gep %59, i64 40                                                                                               ;L261
 48984|  store ptr @ai::plan_legacy8sub_planNtB5_7SubPlanNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %1305,                     ;L261
 48985|  invoke void @_RNvNtNtCs9ec1k27omRZ_3std2io5stdio6__print(ptr @anon.282069a2ed2ad3a275929b639963fb55.140, ptr %59)
 48986|  to label %1306 unwind label %679                                                                                      ;L261
 48987| 
 48988| 1306: ; preds = %1300
 48990|  %1307 = load i64, ptr %1178,                                                                                          ;L2075<277
 48993|  store ptr %454, ptr %57,                                                                                              ;L269
 48994|  %1308 = gep %57, i64 8                                                                                                ;L269
 48995|  store ptr %456, ptr %1308,                                                                                            ;L269
 48996|  %1309 = gep %57, i64 16                                                                                               ;L269
 48997|  store ptr %200, ptr %1309,                                                                                            ;L269
 48998|     ;; self = ptr %70
 48999|     ;; self = ptr %70
 49000|  %1310 = load ptr, ptr %70, , !!8, !!8                                                                                 ;L138<2073<277
 49001|     ;; p = ptr %1310
 49002|     ;; len = i64 %1307
 49003|     ;; count = i64 %1307
 49004|     ;; self[0..+8] = ptr %1310
 49005|     ;; slice[0..+8] = ptr %1310
 49006|     ;; self[8..+8] = i64 %1307
 49007|     ;; slice[8..+8] = i64 %1307
 49008|     ;; ptr = ptr %1310
 49009|     ;; self = ptr %1310
 49010|  %1311 = mul nuw nsw i64 %1307, 192                                                                                    ;L961<100<1042<277
 49011|  %1312 = gep %1310, i64 %1311                                                                                          ;L961<100<1042<277
 49012|     ;; f = ptr %57
 49013|     ;; self = ptr undef
 49014|     ;; self = ptr undef
 49015|     ;; count = i64 1
 49016|     ;; ptr = ptr %1310
 49017|     ;; self = ptr %1310
 49018|     ;; end_or_len = ptr %1312
 49021|  %1313 = icmp eq i64 %1307, 0                                                                                          ;L1714<180<314<277
 49022|  br i1 %1313, label %1356, label %1314                                                                                 ;L180<314<277
 49023| 
 49024| 1314: ; preds = %1306, %1181
 49025|  %1315 = phi ptr [ %1186, %1181 ], [ %1312, %1306 ]
 49026|  %1316 = phi ptr [ %1184, %1181 ], [ %1310, %1306 ]
 49027|  %1317 = gep %456, i64 496
 49028|  %1318 = gep %200, i64 8
 49029|  br label %1319                                                                                                        ;L180<314<277
 49030| 
 49031| 1319: ; preds = %1348, %1314
 49032|  %1320 = phi ptr [ %1316, %1314 ], [ %1321, %1348 ]
 49033|     ;; ptr = ptr %1320
 49034|  %1321 = gep %1320, i64 192                                                                                            ;L656<185<314<277
 49035|     ;; x = ptr %1320
 49039|     ;; sv = ptr %1320
 49040|     ;; c = ptr %1320
 49041|  %1322 = load i64, ptr %1320, , !!53197, !!8                                                                           ;L277<315<277
 49044|     ;; sv = i64 %1322
 49045|     ;; c = ptr %1320
 49046|  %1323 = icmp sgt i64 %1322, -1                                                                                        ;L270<277<315<277
 49047|  br i1 %1323, label %1348, label %1324                                                                                 ;L270<277<315<277
 49048| 
 49049| 1324: ; preds = %1319
 49050|     ;; self = ptr %1320
 49051|  %1325 = gep %1320, i64 185                                                                                            ;L309<271<277<315<277
 49052|  %1326 = load i8, ptr %1325, , !!53223, !!8                                                                            ;L309<271<277<315<277
 49053|  %1327 = icmp ne i8 %1326, 10                                                                                          ;L309<271<277<315<277
 49054|  call void @llvm.assume(i1 %1327)                                                                                      ;L309<271<277<315<277
 49055|  %1328 = add nsw i8 %1326, -16                                                                                         ;L309<271<277<315<277
 49056|  %1329 = icmp ult i8 %1328, 3                                                                                          ;L309<271<277<315<277
 49057|  br i1 %1329, label %1330, label %1348                                                                                 ;L309<271<277<315<277
 49058| 
 49059| 1330: ; preds = %1324
 49060|  %1331 = gep %1320, i64 16                                                                                             ;L0<271<277<315<277
 49061|  %1332 = load i64, ptr %1331, , !!53223, !!8                                                                           ;L0<271<277<315<277
 49062|     ;; target_id = i64 %1332
 49063|  %1333 = load ptr, ptr %1317, , !!53228, !!8                                                                           ;L273<277<315<277
 49064|  %1334 = invoke ptr %1333(ptr %454, i64 %1332)
 49065|  to label %1335 unwind label %679                                                                                      ;L273<277<315<277
 49066| 
 49067| 1335: ; preds = %1330
 49068|     ;; self = ptr %1334
 49071|  %1336 = icmp eq ptr %1334, null                                                                                       ;L708<273<277<315<277
 49072|  br i1 %1336, label %1348, label %1337                                                                                 ;L708<273<277<315<277
 49073| 
 49074| 1337: ; preds = %1335
 49075|     ;; f = ptr %200
 49076|     ;; x = ptr %1334
 49077|     ;; t = ptr %1334
 49078|     ;; self = ptr %1334
 49079|     ;; self = ptr %1334
 49080|     ;; other = ptr %200
 49081|     ;; other = ptr %200
 49082|  %1338 = load i64, ptr %1334, , !!53228, !!8                                                                           ;L1127<264<273<710<273<277<315<277
 49083|  %1339 = gep %1334, i64 8                                                                                              ;L1127<264<273<710<273<277<315<277
 49084|     ;; __self_discr = i64 %1338
 49085|  %1340 = load i64, ptr %200, , !!53228, !!8                                                                            ;L1127<264<273<710<273<277<315<277
 49086|     ;; __arg1_discr = i64 %1340
 49087|  %1341 = icmp eq i64 %1338, %1340                                                                                      ;L1127<264<273<710<273<277<315<277
 49088|  br i1 %1341, label %1342, label %1348                                                                                 ;L1127<264<273<710<273<277<315<277
 49089| 
 49090| 1342: ; preds = %1337
 49091|  %1343 = icmp eq i64 %1338, 0                                                                                          ;L1127<264<273<710<273<277<315<277
 49092|  br i1 %1343, label %1344, label %1350                                                                                 ;L1127<264<273<710<273<277<315<277
 49093| 
 49094| 1344: ; preds = %1342
 49095|     ;; __self_0 = ptr %1334
 49096|     ;; self = ptr %1334
 49101|  %1345 = load i64, ptr %1339, , !!53228, !!8                                                                           ;L1878<2123<1127<264<273<710<273<277<315<277
 49102|  %1346 = load i64, ptr %1318, , !!53228, !!8                                                                           ;L1878<2123<1127<264<273<710<273<277<315<277
 49103|  %1347 = icmp eq i64 %1345, %1346                                                                                      ;L1878<2123<1127<264<273<710<273<277<315<277
 49104|  br i1 %1347, label %1350, label %1348                                                                                 ;L315<277
 49105| 
 49106| 1348: ; preds = %1344, %1337, %1335, %1324, %1319
 49107|     ;; ptr = ptr %1321
 49108|     ;; self = ptr %1321
 49109|     ;; end_or_len = ptr %1315
 49112|  %1349 = icmp eq ptr %1321, %1315                                                                                      ;L1714<180<314<277
 49113|  br i1 %1349, label %1356, label %1319                                                                                 ;L180<314<277
 49114| 
 49115| 1350: ; preds = %1344, %1342
 49118|     ;; self = ptr %70
 49119|     ;; self = ptr %70
 49120|  %1351 = load ptr, ptr %70, , !!8, !!8                                                                                 ;L138<2073<281
 49121|     ;; p = ptr %1351
 49122|  %1352 = load i64, ptr %1178, , !!8                                                                                    ;L2075<281
 49123|     ;; len = i64 %1352
 49124|     ;; count = i64 %1352
 49125|     ;; self[0..+8] = ptr %1351
 49126|     ;; slice[0..+8] = ptr %1351
 49127|     ;; self[8..+8] = i64 %1352
 49128|     ;; slice[8..+8] = i64 %1352
 49129|     ;; ptr = ptr %1351
 49130|     ;; self = ptr %1351
 49131|  %1353 = getelementptr { i64, { [177 x i8], i8, [6 x i8] } }, ptr %1351, i64 %1352                                     ;L961<100<1042<281
 49132|     ;; self[0..+8] = ptr %1351
 49133|     ;; self[8..+8] = ptr %1353
 49134|     ;; self[16..+8] = ptr %57
 49135|  store ptr %1351, ptr %55,                                                                                             ;L24<3564<281
 49136|  %1354 = gep %55, i64 8                                                                                                ;L24<3564<281
 49137|  store ptr %1353, ptr %1354,                                                                                           ;L24<3564<281
 49138|  %1355 = gep %55, i64 16                                                                                               ;L24<3564<281
 49139|  store ptr %57, ptr %1355,                                                                                             ;L24<3564<281
 49140|  invoke void @core::iter8adapters6cloned6ClonedINtNtB2c_6filter6FilterINtNtNtB2g_5slice4iter4IterBU_ENCNvMNtNtNtB10_11plan_legacy7handler7auctionNtB40_17LegacyPlanHandler16get_small_actionsc_0EEEB10_(ptr sret([32 x i8]) %56, ptr %55, ptr %655)
 49141|  to label %1357 unwind label %679                                                                                      ;L280
 49142| 
 49143| 1356: ; preds = %1348, %1306
 49144|  call void @llvm.memcpy.p0.p0.i64(ptr %58, ptr %70, i64 32, i1 false)                                                  ;L278
 49145|  br label %1369                                                                                                        ;L277
 49146| 
 49147| 1357: ; preds = %1350
 49149|     ;; self = ptr %56
 49150|     ;; self = ptr %56
 49151|  %1358 = gep %56, i64 24                                                                                               ;L1617<1636<282
 49152|  %1359 = load i64, ptr %1358, , !!8                                                                                    ;L1617<1636<282
 49153|  %1360 = icmp eq i64 %1359, 0                                                                                          ;L282
 49154|  br i1 %1360, label %1361, label %1366                                                                                 ;L282
 49155| 
 49156| 1361: ; preds = %1357
 49157|  call void @llvm.memcpy.p0.p0.i64(ptr %58, ptr %70, i64 32, i1 false)                                                  ;L287
 49159|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB12_(ptr %56)
 49160|  to label %1365 unwind label %1362                                                                                     ;L825<289
 49161| 
 49162| 1362: ; preds = %1361
 49163|  %1363 = cleanuppad within none []
 49165|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB19_(ptr %56) [ "funclet"(token %1363) ]
 49166|  to label %1364 unwind label %679                                                                                      ;L825<825<289
 49167| 
 49168| 1364: ; preds = %1362
 49169|  cleanupret from %1363 unwind label %679
 49170| 
 49171| 1365: ; preds = %1361
 49173|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB19_(ptr %56)
 49174|  to label %1367 unwind label %679                                                                                      ;L825<825<289
 49175| 
 49176| 1366: ; preds = %1357
 49177|  call void @llvm.memcpy.p0.p0.i64(ptr %58, ptr %56, i64 32, i1 false)                                                  ;L288
 49178|  br label %1367                                                                                                        ;L289
 49179| 
 49180| 1367: ; preds = %1366, %1365
 49181|  %1368 = phi i8 [ 1, %1366 ], [ 0, %1365 ]                                                                             ;L0
 49183|  br label %1369                                                                                                        ;L277
 49184| 
 49185| 1369: ; preds = %1367, %1356
 49186|  %1370 = phi i8 [ 0, %1356 ], [ %1368, %1367 ]                                                                         ;L0
 49189|     ;; self = ptr %58
 49190|     ;; self = ptr %58
 49192|  %1371 = gep %58, i64 24                                                                                               ;L2075<291
 49193|  %1372 = load i64, ptr %1371, , !!8                                                                                    ;L2075<291
 49194|     ;; len = i64 %1372
 49195|     ;; count = i64 %1372
 49198|     ;; self[8..+8] = i64 %1372
 49199|     ;; slice[8..+8] = i64 %1372
 49211|     ;; self = ptr undef
 49212|     ;; self = ptr undef
 49213|     ;; count = i64 1
 49219|  %1373 = icmp eq i64 %1372, 0                                                                                          ;L1714<180<107<2706<3354<3325<291
 49220|  br i1 %1373, label %1391, label %1374                                                                                 ;L180<107<2706<3354<3325<291
 49221| 
 49222| 1374: ; preds = %1369
 49223|  %1375 = mul nuw nsw i64 %1372, 192                                                                                    ;L961<100<1042<291
 49224|  %1376 = load ptr, ptr %58, , !!8, !!8                                                                                 ;L138<2073<291
 49225|     ;; p = ptr %1376
 49226|     ;; self = ptr %1376
 49227|     ;; ptr = ptr %1376
 49228|     ;; self[0..+8] = ptr %1376
 49229|     ;; slice[0..+8] = ptr %1376
 49230|     ;; end_or_len = !DIArgList(ptr %1376, i64 %1375)
 49231|     ;; self[8..+8] = !DIArgList(ptr %1376, i64 %1375)
 49232|     ;; self[8..+8] = !DIArgList(ptr %1376, i64 %1375)
 49233|     ;; self[8..+8] = !DIArgList(ptr %1376, i64 %1375)
 49234|     ;; self[0..+8] = ptr %1376
 49235|     ;; self[0..+8] = ptr %1376
 49236|     ;; self[0..+8] = ptr %1376
 49237|     ;; ptr = ptr %1376
 49238|     ;; self = ptr %1376
 49239|  %1377 = gep %1376, i64 %1375                                                                                          ;L961<100<1042<291
 49240|     ;; self[8..+8] = ptr %1377
 49241|     ;; self[8..+8] = ptr %1377
 49242|     ;; self[8..+8] = ptr %1377
 49243|     ;; end_or_len = ptr %1377
 49244|  %1378 = gep %1376, i64 192                                                                                            ;L656<185<107<2706<3354<3325<291
 49245|     ;; self[0..+8] = ptr %1378
 49246|     ;; self = ptr %1376
 49247|     ;; f = ptr undef
 49248|     ;; self = ptr undef
 49249|     ;; x = ptr %1376
 49250|     ;; args = ptr %1376
 49251|     ;; x = ptr %1376
 49254|  %1379 = load i64, ptr %1376, , !!53534, !!8                                                                           ;L291<3317<310<1162<107<2706<3354<3325<291
 49255|     ;; first[0..+8] = i64 %1379
 49256|     ;; first[8..+8] = ptr %1376
 49257|  %1380 = invoke { i64, ptr } @core::iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterTxNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEENCINvNvNtNtNtBa_6traits8iterator8Iterator10max_by_key3keyRB1n_xNCNvMNtNtNtB1t_11plan_legacy7handler7auctionNtB3u_17LegacyPlanHandler16get_small_actionsd_0E0EB2t_4foldTxB3h_ENCINvNvB2t_6max_by4foldB52_INvB2r_7compareB3h_xEE0EB1t_(ptr %1378, ptr %1377, i64 %1379, ptr %1376)
 49258|  to label %1384 unwind label %1381                                                                                     ;L2707<3354<3325<291
 49259| 
 49260| 1381: ; preds = %2462, %2461, %2459, %2191, %2190, %2188, %1924, %1923, %1921, %1434, %1408, %1403, %1398, %1392, %1391, %1374
 49261|  %1382 = phi i1 [ false, %1924 ], [ false, %2191 ], [ %1435, %1434 ], [ false, %2462 ], [ true, %1392 ], [ false, %2459 ], [ true, %1408 ], [ true, %1403 ], [ true, %1398 ], [ true, %1391 ], [ true, %1374 ], [ false, %1923 ], [ false, %1921 ], [ false, %2190 ], [ false, %2188 ], [ false, %2461 ] ;L0
 49262|  %1383 = cleanuppad within none []
 49263|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecTxNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEEB1v_(ptr %58) #31 [ "funclet"(token %1383) ] ;L416
 49264|  cleanupret from %1383 unwind label %679                                                                               ;L416
 49265| 
 49266| 1384: ; preds = %1374
 49267|  %1385 = extractvalue { i64, ptr } %1380, 1                                                                            ;L2707<3354<3325<291
 49268|     ;; self = ptr %1385
 49269|  %1386 = icmp eq ptr %1385, null                                                                                       ;L1011<291
 49270|  br i1 %1386, label %1391, label %1387                                                                                 ;L1011<291
 49271| 
 49272| 1387: ; preds = %1384
 49273|  %1388 = load i64, ptr %1385, , !!8                                                                                    ;L291
 49274|  store i64 %1388, ptr %54,                                                                                             ;L291
 49275|  %1389 = icmp slt i64 %1388, -8999999                                                                                  ;L300
 49276|  %1390 = and i1 %206, %1389                                                                                            ;L300
 49277|  br i1 %1390, label %1398, label %1392                                                                                 ;L300
 49278| 
 49279| 1391: ; preds = %1384, %1369
 49280|  invoke void @core::option13unwrap_failed(ptr @anon.282069a2ed2ad3a275929b639963fb55.148) #32
 49281|  to label %203 unwind label %1381                                                                                      ;L1013<291
 49282| 
 49283| 1392: ; preds = %1413, %1400, %1387
 49286|     ;; self = ptr %58
 49287|     ;; self = ptr %58
 49288|  %1393 = load ptr, ptr %58, , !!8, !!8                                                                                 ;L138<2073<310
 49289|     ;; p = ptr %1393
 49290|  %1394 = load i64, ptr %1371, , !!8                                                                                    ;L2075<310
 49291|     ;; len = i64 %1394
 49292|     ;; count = i64 %1394
 49293|     ;; self[0..+8] = ptr %1393
 49294|     ;; slice[0..+8] = ptr %1393
 49295|     ;; self[8..+8] = i64 %1394
 49296|     ;; slice[8..+8] = i64 %1394
 49297|     ;; ptr = ptr %1393
 49298|     ;; self = ptr %1393
 49299|  %1395 = getelementptr { i64, { [177 x i8], i8, [6 x i8] } }, ptr %1393, i64 %1394                                     ;L961<100<1042<310
 49300|     ;; self[0..+8] = ptr %1393
 49301|     ;; self[8..+8] = ptr %1395
 49302|     ;; self[16..+8] = ptr %54
 49303|  store ptr %1393, ptr %50,                                                                                             ;L69<836<310
 49304|  %1396 = gep %50, i64 8                                                                                                ;L69<836<310
 49305|  store ptr %1395, ptr %1396,                                                                                           ;L69<836<310
 49306|  %1397 = gep %50, i64 16                                                                                               ;L69<836<310
 49307|  store ptr %54, ptr %1397,                                                                                             ;L69<836<310
 49308|  invoke void @core::iter8adapters3map3MapINtNtB29_6filter6FilterINtNtNtB2d_5slice4iter4IterTxBU_EENCNvMNtNtNtBY_11plan_legacy7handler7auctionNtB3U_17LegacyPlanHandler16get_small_actionse_0ENCB3P_sf_0EEBY_(ptr sret([32 x i8]) %51, ptr %50, ptr %655)
 49309|  to label %1417 unwind label %1381                                                                                     ;L309
 49310| 
 49311| 1398: ; preds = %1387
 49312|  %1399 = invoke { i64, ptr } %458(ptr %454)
 49313|  to label %1400 unwind label %1381                                                                                     ;L301
 49314| 
 49315| 1400: ; preds = %1398
 49316|  %1401 = extractvalue { i64, ptr } %1399, 0                                                                            ;L301
 49317|  %1402 = icmp eq i64 %1401, 2                                                                                          ;L301
 49318|  br i1 %1402, label %1392, label %1403                                                                                 ;L301
 49319| 
 49320| 1403: ; preds = %1400
 49323|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway3new(ptr sret([136 x i8]) %52, ptr %5, ptr %4, i64 5)
 49324|  to label %1404 unwind label %1381                                                                                     ;L302
 49325| 
 49326| 1404: ; preds = %1403
 49327|  call void @llvm.memcpy.p0.p0.i64(ptr %53, ptr %52, i64 136, i1 false)                                                 ;L302
 49328|  %1405 = gep %53, i64 177                                                                                              ;L302
 49329|  store i8 3, ptr %1405,                                                                                                ;L302
 49331|  %1406 = load i64, ptr %97, , !!8                                                                                      ;L303
 49332|  %1407 = invoke i64 @ai::plan_legacy8sub_planNtB4_7SubPlan5score(ptr %112, i64 %1406, ptr %95, ptr %3, ptr %4, ptr %5, ptr %53, ptr %8)
 49333|  to label %1410 unwind label %1408                                                                                     ;L303
 49334| 
 49335| 1408: ; preds = %1404
 49336|  %1409 = cleanuppad within none []
 49337|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %53) #31 [ "funclet"(token %1409) ] ;L307
 49338|  cleanupret from %1409 unwind label %1381                                                                              ;L307
 49339| 
 49340| 1410: ; preds = %1404
 49341|     ;; s = i64 %1407
 49342|  %1411 = load i64, ptr %54, , !!8                                                                                      ;L304
 49343|  %1412 = icmp sgt i64 %1407, %1411                                                                                     ;L304
 49344|  br i1 %1412, label %1414, label %1413                                                                                 ;L304
 49345| 
 49346| 1413: ; preds = %1410
 49347|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %53)           ;L307
 49349|  br label %1392                                                                                                        ;L300
 49350| 
 49351| 1414: ; preds = %1410
 49352|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %95, i64 5384, i1 false)                                                 ;L305
 49353|  %1415 = gep %0, i64 5392                                                                                              ;L305
 49354|  call void @llvm.memcpy.p0.p0.i64(ptr %1415, ptr %53, i64 184, i1 false)                                               ;L305
 49355|  %1416 = gep %0, i64 5384                                                                                              ;L305
 49356|  store i64 %1407, ptr %1416,                                                                                           ;L305
 49358|  br label %1931                                                                                                        ;L1
 49359| 
 49360| 1417: ; preds = %1392
 49363|  %1418 = gep %7, i64 16                                                                                                ;L3054<3079<313
 49364|  %1419 = load i64, ptr %1418, , !!8                                                                                    ;L3054<3079<313
 49365|  %1420 = icmp ult i64 %1419, 288230376151711744                                                                        ;L3059<3079<313
 49366|  call void @llvm.assume(i1 %1420)                                                                                      ;L3059<3079<313
 49367|  %1421 = icmp eq i64 %1419, 0                                                                                          ;L313
 49368|  br i1 %1421, label %1422, label %1427                                                                                 ;L313
 49369| 
 49370| 1422: ; preds = %1417
 49372|     ;; self = ptr %51
 49373|     ;; self = ptr %51
 49374|  %1423 = load ptr, ptr %51, , !!8, !!8                                                                                 ;L138<2073<314
 49375|     ;; p = ptr %1423
 49376|  %1424 = gep %51, i64 24                                                                                               ;L2075<314
 49377|  %1425 = load i64, ptr %1424, , !!8                                                                                    ;L2075<314
 49378|  %1426 = invoke ptr @_RINvXNtCsMBkRBYhlca_4rand3seqSNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayNtB3_11SliceRandom6chooseNtNtNtB5_4rngs3std6StdRngEBw_(ptr %1423, i64 %1425, ptr %3)
 49379|  to label %1437 unwind label %1434                                                                                     ;L314
 49380| 
 49381| 1427: ; preds = %1417
 49384|     ;; self = ptr %51
 49385|     ;; self = ptr %51
 49386|  %1428 = load ptr, ptr %51, , !!8, !!8                                                                                 ;L138<2073<317
 49387|     ;; p = ptr %1428
 49388|  %1429 = gep %51, i64 24                                                                                               ;L2075<317
 49389|  %1430 = load i64, ptr %1429, , !!8                                                                                    ;L2075<317
 49390|     ;; len = i64 %1430
 49391|     ;; count = i64 %1430
 49392|     ;; self[0..+8] = ptr %1428
 49393|     ;; slice[0..+8] = ptr %1428
 49394|     ;; self[8..+8] = i64 %1430
 49395|     ;; slice[8..+8] = i64 %1430
 49396|     ;; ptr = ptr %1428
 49397|     ;; self = ptr %1428
 49398|  %1431 = gepS %1428, i64 %1430                                                                                         ;L961<100<1042<317
 49399|     ;; self[0..+8] = ptr %1428
 49400|     ;; self[8..+8] = ptr %1431
 49401|     ;; self[16..+8] = ptr %7
 49402|  store ptr %1428, ptr %46,                                                                                             ;L69<836<319
 49403|  %1432 = gep %46, i64 8                                                                                                ;L69<836<319
 49404|  store ptr %1431, ptr %1432,                                                                                           ;L69<836<319
 49405|  %1433 = gep %46, i64 16                                                                                               ;L69<836<319
 49406|  store ptr %7, ptr %1433,                                                                                              ;L69<836<319
 49407|  invoke void @core::iter8adapters3map3MapINtNtB29_6filter6FilterINtNtNtB2d_5slice4iter4IterBU_ENCNvMNtNtNtBY_11plan_legacy7handler7auctionNtB3R_17LegacyPlanHandler16get_small_actionsg_0ENCB3M_sh_0EEBY_(ptr sret([32 x i8]) %47, ptr %46, ptr %655)
 49408|  to label %1445 unwind label %1434                                                                                     ;L316
 49409| 
 49410| 1434: ; preds = %2484, %2483, %1468, %1467, %1465, %1456, %1440, %1439, %1427, %1422
 49411|  %1435 = phi i1 [ %1488, %2484 ], [ %1488, %2483 ], [ true, %1465 ], [ true, %1439 ], [ true, %1440 ], [ true, %1422 ], [ true, %1468 ], [ true, %1456 ], [ true, %1427 ], [ true, %1467 ] ;L0
 49412|  %1436 = cleanuppad within none []
 49413|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %51) #31 [ "funclet"(token %1436) ] ;L416
 49414|  cleanupret from %1436 unwind label %1381                                                                              ;L416
 49415| 
 49416| 1437: ; preds = %1422
 49417|     ;; self = ptr %1426
 49418|  %1438 = icmp eq ptr %1426, null                                                                                       ;L1011<314
 49419|  br i1 %1438, label %1440, label %1439                                                                                 ;L1011<314
 49420| 
 49421| 1439: ; preds = %1437
 49422|  invoke fastcc void @ai::small_actionNtB4_15SmallActionPlayNtNtCsjihNppCmMEE_4core5clone5Clone5clone(ptr %48, ptr %1426)
 49423|  to label %1441 unwind label %1434                                                                                     ;L314
 49424| 
 49425| 1440: ; preds = %1437
 49426|  invoke void @core::option13unwrap_failed(ptr @anon.282069a2ed2ad3a275929b639963fb55.149) #32
 49427|  to label %203 unwind label %1434                                                                                      ;L1013<314
 49428| 
 49429| 1441: ; preds = %1439
 49430|  %1442 = load i64, ptr %54, , !!8                                                                                      ;L314
 49431|  store i64 %1442, ptr %49,                                                                                             ;L314
 49432|  %1443 = gep %49, i64 8                                                                                                ;L314
 49433|  call void @llvm.memcpy.p0.p0.i64(ptr %1443, ptr %48, i64 184, i1 false)                                               ;L314
 49435|  br label %1444                                                                                                        ;L313
 49436| 
 49437| 1444: ; preds = %1473, %1441
 49438|  br i1 %691, label %1480, label %1474                                                                                  ;L329
 49439| 
 49440| 1445: ; preds = %1427
 49443|     ;; self = ptr %47
 49444|     ;; self = ptr %47
 49445|  %1446 = gep %47, i64 24                                                                                               ;L1617<1636<321
 49446|  %1447 = load i64, ptr %1446, , !!8                                                                                    ;L1617<1636<321
 49447|  %1448 = icmp eq i64 %1447, 0                                                                                          ;L321
 49448|  br i1 %1448, label %1449, label %1453                                                                                 ;L321
 49449| 
 49450| 1449: ; preds = %1445
 49451|     ;; self = ptr %51
 49452|     ;; self = ptr %51
 49453|  %1450 = load ptr, ptr %51, , !!8, !!8                                                                                 ;L138<2073<322
 49454|     ;; p = ptr %1450
 49455|  %1451 = load i64, ptr %1429, , !!8                                                                                    ;L2075<322
 49456|  %1452 = invoke ptr @_RINvXNtCsMBkRBYhlca_4rand3seqSNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayNtB3_11SliceRandom6chooseNtNtNtB5_4rngs3std6StdRngEBw_(ptr %1450, i64 %1451, ptr %3)
 49457|  to label %1458 unwind label %1456                                                                                     ;L322
 49458| 
 49459| 1453: ; preds = %1445
 49460|     ;; self = ptr %47
 49461|     ;; self = ptr %47
 49462|  %1454 = load ptr, ptr %47, , !!8, !!8                                                                                 ;L138<2073<324
 49463|  %1455 = invoke ptr @_RINvXNtCsMBkRBYhlca_4rand3seqSNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayNtB3_11SliceRandom6chooseNtNtNtB5_4rngs3std6StdRngEBw_(ptr %1454, i64 %1447, ptr %3)
 49464|  to label %1469 unwind label %1456                                                                                     ;L324
 49465| 
 49466| 1456: ; preds = %1472, %1471, %1461, %1460, %1453, %1449
 49467|  %1457 = cleanuppad within none []
 49468|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %47) #31 [ "funclet"(token %1457) ] ;L326
 49469|  cleanupret from %1457 unwind label %1434                                                                              ;L326
 49470| 
 49471| 1458: ; preds = %1449
 49472|     ;; self = ptr %1452
 49473|  %1459 = icmp eq ptr %1452, null                                                                                       ;L1011<322
 49474|  br i1 %1459, label %1461, label %1460                                                                                 ;L1011<322
 49475| 
 49476| 1460: ; preds = %1458
 49477|  invoke fastcc void @ai::small_actionNtB4_15SmallActionPlayNtNtCsjihNppCmMEE_4core5clone5Clone5clone(ptr %45, ptr %1452)
 49478|  to label %1462 unwind label %1456                                                                                     ;L322
 49479| 
 49480| 1461: ; preds = %1458
 49481|  invoke void @core::option13unwrap_failed(ptr @anon.282069a2ed2ad3a275929b639963fb55.150) #32
 49482|  to label %203 unwind label %1456                                                                                      ;L1013<322
 49483| 
 49484| 1462: ; preds = %1471, %1460
 49485|  %1463 = load i64, ptr %54, , !!8                                                                                      ;L321
 49486|  store i64 %1463, ptr %49,                                                                                             ;L321
 49487|  %1464 = gep %49, i64 8                                                                                                ;L321
 49488|  call void @llvm.memcpy.p0.p0.i64(ptr %1464, ptr %45, i64 184, i1 false)                                               ;L321
 49491|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %47)
 49492|  to label %1468 unwind label %1465                                                                                     ;L825<326
 49493| 
 49494| 1465: ; preds = %1462
 49495|  %1466 = cleanuppad within none []
 49497|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %47) [ "funclet"(token %1466) ]
 49498|  to label %1467 unwind label %1434                                                                                     ;L825<825<326
 49499| 
 49500| 1467: ; preds = %1465
 49501|  cleanupret from %1466 unwind label %1434
 49502| 
 49503| 1468: ; preds = %1462
 49505|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %47)
 49506|  to label %1473 unwind label %1434                                                                                     ;L825<825<326
 49507| 
 49508| 1469: ; preds = %1453
 49509|     ;; self = ptr %1455
 49510|  %1470 = icmp eq ptr %1455, null                                                                                       ;L1011<324
 49511|  br i1 %1470, label %1472, label %1471                                                                                 ;L1011<324
 49512| 
 49513| 1471: ; preds = %1469
 49514|  invoke fastcc void @ai::small_actionNtB4_15SmallActionPlayNtNtCsjihNppCmMEE_4core5clone5Clone5clone(ptr %45, ptr %1455)
 49515|  to label %1462 unwind label %1456                                                                                     ;L324
 49516| 
 49517| 1472: ; preds = %1469
 49518|  invoke void @core::option13unwrap_failed(ptr @anon.282069a2ed2ad3a275929b639963fb55.151) #32
 49519|  to label %203 unwind label %1456                                                                                      ;L1013<324
 49520| 
 49521| 1473: ; preds = %1468
 49523|  br label %1444                                                                                                        ;L313
 49524| 
 49525| 1474: ; preds = %1916, %1725, %1717, %1711, %1679, %1679, %1679, %1679, %1679, %1679, %1679, %1679, %1679, %1679, %1679, %1679, %1444
 49526|  %1475 = gep %49, i64 8                                                                                                ;L356
 49527|  %1476 = gep %49, i64 185                                                                                              ;L356
 49528|  %1477 = load i8, ptr %1476, , !!8                                                                                     ;L356
 49529|  %1478 = icmp ne i8 %1477, 10                                                                                          ;L356
 49530|  call void @llvm.assume(i1 %1478)                                                                                      ;L356
 49531|  %1479 = icmp eq i8 %1477, 13                                                                                          ;L356
 49532|  br i1 %1479, label %1917, label %1925                                                                                 ;L356
 49533| 
 49534| 1480: ; preds = %1444
 49535|     ;; self = ptr %49
 49536|  %1481 = gep %49, i64 185                                                                                              ;L309<330
 49537|  %1482 = load i8, ptr %1481, , !!53686, !!8                                                                            ;L309<330
 49538|  %1483 = icmp ne i8 %1482, 10                                                                                          ;L309<330
 49539|  call void @llvm.assume(i1 %1483)                                                                                      ;L309<330
 49540|  %1484 = add nsw i8 %1482, -16                                                                                         ;L309<330
 49541|  %1485 = icmp ult i8 %1484, 3                                                                                          ;L309<330
 49542|  br i1 %1485, label %1494, label %1490                                                                                 ;L309<330
 49543| 
 49544| 1486: ; preds = %2456, %2455, %2453, %2240, %2239, %2238, %2236, %2215, %2208, %2207, %2205, %2005, %2004, %2003, %2001, %1954, %1946, %1942, %1915, %1914, %1912, %1895, %1818, %1817, %1815, %1737, %1729, %1704, %1700, %1686, %1604, %1593, %1494
 49545|  %1487 = phi i1 [ false, %2004 ], [ true, %2208 ], [ %2006, %2005 ], [ true, %1946 ], [ true, %1954 ], [ true, %1942 ], [ false, %2239 ], [ true, %2456 ], [ %2241, %2240 ], [ true, %2215 ], [ true, %1914 ], [ true, %1912 ], [ true, %1737 ], [ true, %1915 ], [ true, %1895 ], [ true, %1818 ], [ true, %1729 ], [ true, %1604 ], [ true, %1704 ], [ true, %1593 ], [ true, %1700 ], [ true, %1686 ], [ true, %1494 ], [ true, %1817 ], [ true, %1815 ], [ false, %2003 ], [ false, %2001 ], [ true, %2207 ], [ true, %2205 ], [ false, %2238 ], [ false, %2236 ], [ true, %2455 ], [ true, %2453 ] ;L0
 49546|  %1488 = phi i1 [ false, %2004 ], [ false, %2208 ], [ %2007, %2005 ], [ true, %1946 ], [ true, %1954 ], [ true, %1942 ], [ false, %2239 ], [ false, %2456 ], [ %2242, %2240 ], [ true, %2215 ], [ true, %1914 ], [ true, %1912 ], [ true, %1737 ], [ true, %1915 ], [ true, %1895 ], [ true, %1818 ], [ true, %1729 ], [ true, %1604 ], [ true, %1704 ], [ true, %1593 ], [ true, %1700 ], [ true, %1686 ], [ true, %1494 ], [ true, %1817 ], [ true, %1815 ], [ false, %2003 ], [ false, %2001 ], [ false, %2207 ], [ false, %2205 ], [ false, %2238 ], [ false, %2236 ], [ false, %2455 ], [ false, %2453 ] ;L0
 49547|  %1489 = cleanuppad within none []
 49548|  br i1 %1487, label %2484, label %2483                                                                                 ;L416
 49549| 
 49550| 1490: ; preds = %1604, %1514, %1506, %1502, %1500, %1480
 49551|  %1491 = load i64, ptr %49, , !!8                                                                                      ;L337
 49552|  %1492 = icmp slt i64 %1491, -8999999                                                                                  ;L337
 49553|  %1493 = load i8, ptr %1481, , !!53696                                                                                 ;L309<343
 49554|  br i1 %1492, label %1605, label %1679                                                                                 ;L337
 49555| 
 49556| 1494: ; preds = %1480
 49557|  %1495 = gep %49, i64 16                                                                                               ;L0<330
 49558|  %1496 = load i64, ptr %1495, , !!53686, !!8                                                                           ;L0<330
 49559|     ;; target_id = i64 %1496
 49560|  %1497 = gep %456, i64 496                                                                                             ;L331
 49561|  %1498 = load ptr, ptr %1497, , !!8                                                                                    ;L331
 49562|  %1499 = invoke ptr %1498(ptr %454, i64 %1496)
 49563|  to label %1500 unwind label %1486                                                                                     ;L331
 49564| 
 49565| 1500: ; preds = %1494
 49566|     ;; self = ptr %1499
 49567|     ;; f = ptr %200
 49568|  %1501 = icmp eq ptr %1499, null                                                                                       ;L659<331
 49569|  br i1 %1501, label %1490, label %1502                                                                                 ;L659<331
 49570| 
 49571| 1502: ; preds = %1500
 49572|     ;; x = ptr %1499
 49573|  %1503 = load i64, ptr %200, , !!8                                                                                     ;L661<331
 49575|     ;; t = ptr %1499
 49576|     ;; self = ptr %1499
 49578|  %1504 = load i64, ptr %1499, , !!8                                                                                    ;L1127<331<661<331
 49579|     ;; __self_discr = i64 %1504
 49580|     ;; __arg1_discr = i64 %1503
 49581|  %1505 = icmp eq i64 %1504, %1503                                                                                      ;L1127<331<661<331
 49582|  br i1 %1505, label %1506, label %1490                                                                                 ;L1127<331<661<331
 49583| 
 49584| 1506: ; preds = %1502
 49585|  %1507 = gep %200, i64 8                                                                                               ;L661<331
 49586|  %1508 = load i64, ptr %1507,                                                                                          ;L661<331
 49587|  %1509 = gep %1499, i64 8                                                                                              ;L1127<331<661<331
 49588|  %1510 = icmp ne i64 %1503, 0                                                                                          ;L1127<331<661<331
 49589|  %1511 = load i64, ptr %1509,
 49590|  %1512 = icmp eq i64 %1511, %1508
 49591|  %1513 = select i1 %1510, i1 true, i1 %1512                                                                            ;L1127<331<661<331
 49592|  br i1 %1513, label %1514, label %1490                                                                                 ;L1127<331<661<331
 49593| 
 49594| 1514: ; preds = %1506
 49595|     ;; self = ptr %1499
 49596|  %1515 = gep %1499, i64 104                                                                                            ;L1404<331<661<331
 49597|  %1516 = load i64, ptr %1515, , !!8                                                                                    ;L1404<331<661<331
 49598|  %1517 = icmp eq i64 %1516, 13                                                                                         ;L1404<331<661<331
 49599|  br i1 %1517, label %1518, label %1490                                                                                 ;L331
 49600| 
 49601| 1518: ; preds = %1514
 49602|     ;; args[0..+8] = ptr %189
 49603|     ;; args[8..+8] = ptr %193
 49607|     ;; self = ptr %49
 49608|  %1519 = load i8, ptr %1481, , !!53752, !!8                                                                            ;L309<333
 49609|  %1520 = icmp ne i8 %1519, 10                                                                                          ;L309<333
 49610|  call void @llvm.assume(i1 %1520)                                                                                      ;L309<333
 49611|  %1521 = add nsw i8 %1519, -3                                                                                          ;L309<333
 49612|  %1522 = icmp samesign ugt i8 %1519, 2                                                                                 ;L309<333
 49613|  %1523 = select i1 %1522, i8 %1521, i8 7                                                                               ;L309<333
 49614|  switch i8 %1523, label %1524 [
 49615|  i8 0, label %1593
 49616|  i8 1, label %1593
 49617|  i8 2, label %1525
 49618|  i8 3, label %1529
 49619|  i8 4, label %1533
 49620|  i8 5, label %1593
 49621|  i8 6, label %1540
 49622|  i8 7, label %1547
 49623|  i8 8, label %1554
 49624|  i8 9, label %1561
 49625|  i8 10, label %1568
 49626|  i8 11, label %1572
 49627|  i8 12, label %1576
 49628|  i8 13, label %1580
 49629|  i8 14, label %1584
 49630|  i8 15, label %1588
 49631|  i8 16, label %1592
 49632|  ]                                                                                                                     ;L309<333
 49633| 
 49634| 1524: ; preds = %1518
 49635|  unreachable                                                                                                           ;L309<333
 49636| 
 49637| 1525: ; preds = %1518
 49638|     ;; action = ptr %49
 49639|     ;; self = ptr %49
 49640|  %1526 = gep %49, i64 16                                                                                               ;L285<316<333
 49641|  %1527 = load i64, ptr %1526, , !!53752, !!8                                                                           ;L285<316<333
 49642|  %1528 = gep %43, i64 8                                                                                                ;L285<316<333
 49643|  store i64 %1527, ptr %1528, , !!53755                                                                                 ;L285<316<333
 49644|  br label %1593                                                                                                        ;L316<333
 49645| 
 49646| 1529: ; preds = %1518
 49647|     ;; action = ptr %49
 49648|     ;; self = ptr %49
 49649|  %1530 = gep %49, i64 16                                                                                               ;L491<313<333
 49650|  %1531 = load i64, ptr %1530, , !!53752, !!8                                                                           ;L491<313<333
 49651|  %1532 = gep %43, i64 8                                                                                                ;L491<313<333
 49652|  store i64 %1531, ptr %1532, , !!53755                                                                                 ;L491<313<333
 49653|  br label %1593                                                                                                        ;L313<333
 49654| 
 49655| 1533: ; preds = %1518
 49656|     ;; action = ptr %49
 49657|     ;; self = ptr %49
 49658|  %1534 = gep %49, i64 24                                                                                               ;L665<314<333
 49659|  %1535 = load i64, ptr %1534, , !!53752, !!8                                                                           ;L665<314<333
 49660|  %1536 = gep %49, i64 32                                                                                               ;L665<314<333
 49661|  %1537 = load i64, ptr %1536, , !!53752, !!8                                                                           ;L665<314<333
 49662|  %1538 = gep %43, i64 8                                                                                                ;L665<314<333
 49663|  store i64 %1535, ptr %1538, , !!53755                                                                                 ;L665<314<333
 49664|  %1539 = gep %43, i64 16                                                                                               ;L665<314<333
 49665|  store i64 %1537, ptr %1539, , !!53755                                                                                 ;L665<314<333
 49666|  br label %1593                                                                                                        ;L314<333
 49667| 
 49668| 1540: ; preds = %1518
 49669|     ;; action = ptr %49
 49670|     ;; self = ptr %49
 49671|  %1541 = gep %49, i64 16                                                                                               ;L800<312<333
 49672|  %1542 = load i64, ptr %1541, , !!53752, !!8                                                                           ;L800<312<333
 49673|  %1543 = gep %49, i64 24                                                                                               ;L800<312<333
 49674|  %1544 = load i64, ptr %1543, , !!53752, !!8                                                                           ;L800<312<333
 49675|  %1545 = gep %43, i64 8                                                                                                ;L800<312<333
 49676|  store i64 %1542, ptr %1545, , !!53755                                                                                 ;L800<312<333
 49677|  %1546 = gep %43, i64 16                                                                                               ;L800<312<333
 49678|  store i64 %1544, ptr %1546, , !!53755                                                                                 ;L800<312<333
 49679|  br label %1593                                                                                                        ;L312<333
 49680| 
 49681| 1547: ; preds = %1518
 49682|     ;; action = ptr %49
 49683|     ;; self = ptr %49
 49684|  %1548 = gep %49, i64 56                                                                                               ;L1035<317<333
 49685|  %1549 = load i64, ptr %1548, , !!53752, !!8                                                                           ;L1035<317<333
 49686|  %1550 = gep %49, i64 64                                                                                               ;L1035<317<333
 49687|  %1551 = load i64, ptr %1550, , !!53752, !!8                                                                           ;L1035<317<333
 49688|  %1552 = gep %43, i64 8                                                                                                ;L1035<317<333
 49689|  store i64 %1549, ptr %1552, , !!53755                                                                                 ;L1035<317<333
 49690|  %1553 = gep %43, i64 16                                                                                               ;L1035<317<333
 49691|  store i64 %1551, ptr %1553, , !!53755                                                                                 ;L1035<317<333
 49692|  br label %1593                                                                                                        ;L317<333
 49693| 
 49694| 1554: ; preds = %1518
 49695|     ;; action = ptr %49
 49696|     ;; self = ptr %49
 49697|  %1555 = gep %49, i64 16                                                                                               ;L1123<318<333
 49698|  %1556 = load i64, ptr %1555, , !!53752, !!8                                                                           ;L1123<318<333
 49699|  %1557 = gep %49, i64 24                                                                                               ;L1123<318<333
 49700|  %1558 = load i64, ptr %1557, , !!53752, !!8                                                                           ;L1123<318<333
 49701|  %1559 = gep %43, i64 8                                                                                                ;L1123<318<333
 49702|  store i64 %1556, ptr %1559, , !!53755                                                                                 ;L1123<318<333
 49703|  %1560 = gep %43, i64 16                                                                                               ;L1123<318<333
 49704|  store i64 %1558, ptr %1560, , !!53755                                                                                 ;L1123<318<333
 49705|  br label %1593                                                                                                        ;L318<333
 49706| 
 49707| 1561: ; preds = %1518
 49708|     ;; action = ptr %49
 49709|     ;; self = ptr %49
 49710|  %1562 = gep %49, i64 32                                                                                               ;L1241<319<333
 49711|  %1563 = load i64, ptr %1562, , !!53752, !!8                                                                           ;L1241<319<333
 49712|  %1564 = gep %49, i64 40                                                                                               ;L1241<319<333
 49713|  %1565 = load i64, ptr %1564, , !!53752, !!8                                                                           ;L1241<319<333
 49714|  %1566 = gep %43, i64 8                                                                                                ;L1241<319<333
 49715|  store i64 %1563, ptr %1566, , !!53755                                                                                 ;L1241<319<333
 49716|  %1567 = gep %43, i64 16                                                                                               ;L1241<319<333
 49717|  store i64 %1565, ptr %1567, , !!53755                                                                                 ;L1241<319<333
 49718|  br label %1593                                                                                                        ;L319<333
 49719| 
 49720| 1568: ; preds = %1518
 49721|     ;; action = ptr %49
 49722|     ;; self = ptr %49
 49723|  %1569 = gep %49, i64 16                                                                                               ;L653<320<333
 49724|  %1570 = load i64, ptr %1569, , !!53752, !!8                                                                           ;L653<320<333
 49725|  %1571 = gep %43, i64 8                                                                                                ;L653<320<333
 49726|  store i64 %1570, ptr %1571, , !!53755                                                                                 ;L653<320<333
 49727|  br label %1593                                                                                                        ;L320<333
 49728| 
 49729| 1572: ; preds = %1518
 49730|     ;; action = ptr %49
 49731|     ;; self = ptr %49
 49732|  %1573 = gep %49, i64 104                                                                                              ;L404<321<333
 49733|  %1574 = load i64, ptr %1573, , !!53752, !!8                                                                           ;L404<321<333
 49734|  %1575 = gep %43, i64 8                                                                                                ;L404<321<333
 49735|  store i64 %1574, ptr %1575, , !!53755                                                                                 ;L404<321<333
 49736|  br label %1593                                                                                                        ;L321<333
 49737| 
 49738| 1576: ; preds = %1518
 49739|     ;; action = ptr %49
 49740|     ;; self = ptr %49
 49741|  %1577 = gep %49, i64 16                                                                                               ;L94<322<333
 49742|  %1578 = load i64, ptr %1577, , !!53752, !!8                                                                           ;L94<322<333
 49743|  %1579 = gep %43, i64 8                                                                                                ;L94<322<333
 49744|  store i64 %1578, ptr %1579, , !!53755                                                                                 ;L94<322<333
 49745|  br label %1593                                                                                                        ;L322<333
 49746| 
 49747| 1580: ; preds = %1518
 49748|     ;; action = ptr %49
 49749|     ;; self = ptr %49
 49750|  %1581 = gep %49, i64 16                                                                                               ;L160<323<333
 49751|  %1582 = load i64, ptr %1581, , !!53752, !!8                                                                           ;L160<323<333
 49752|  %1583 = gep %43, i64 8                                                                                                ;L160<323<333
 49753|  store i64 %1582, ptr %1583, , !!53755                                                                                 ;L160<323<333
 49754|  br label %1593                                                                                                        ;L323<333
 49755| 
 49756| 1584: ; preds = %1518
 49757|     ;; action = ptr %49
 49758|     ;; self = ptr %49
 49759|  %1585 = gep %49, i64 16                                                                                               ;L222<324<333
 49760|  %1586 = load i64, ptr %1585, , !!53752, !!8                                                                           ;L222<324<333
 49761|  %1587 = gep %43, i64 8                                                                                                ;L222<324<333
 49762|  store i64 %1586, ptr %1587, , !!53755                                                                                 ;L222<324<333
 49763|  br label %1593                                                                                                        ;L324<333
 49764| 
 49765| 1588: ; preds = %1518
 49766|     ;; action = ptr %49
 49767|     ;; self = ptr %49
 49768|  %1589 = gep %49, i64 16                                                                                               ;L287<325<333
 49769|  %1590 = load i64, ptr %1589, , !!53752, !!8                                                                           ;L287<325<333
 49770|  %1591 = gep %43, i64 8                                                                                                ;L287<325<333
 49771|  store i64 %1590, ptr %1591, , !!53755                                                                                 ;L287<325<333
 49772|  br label %1593                                                                                                        ;L325<333
 49773| 
 49774| 1592: ; preds = %1518
 49775|  br label %1593                                                                                                        ;L326<333
 49776| 
 49777| 1593: ; preds = %1592, %1588, %1584, %1580, %1576, %1572, %1568, %1561, %1554, %1547, %1540, %1533, %1529, %1525, %1518, %1518, %1518
 49778|  %1594 = phi i64 [ 10, %1592 ], [ 9, %1588 ], [ 8, %1584 ], [ 7, %1580 ], [ 6, %1576 ], [ 4, %1572 ], [ 2, %1568 ], [ 3, %1561 ], [ 3, %1554 ], [ 3, %1547 ], [ 1, %1540 ], [ 0, %1518 ], [ 3, %1533 ], [ 2, %1529 ], [ 2, %1525 ], [ 0, %1518 ], [ 0, %1518 ]
 49779|  store i64 %1594, ptr %43, , !!53755                                                                                   ;L0<333
 49780|     ;; args[16..+8] = ptr %43
 49781|     ;; args[24..+8] = ptr %49
 49782|     ;; args[32..+8] = ptr %112
 49784|  store ptr %189, ptr %42,                                                                                              ;L332
 49785|  %1595 = gep %42, i64 8                                                                                                ;L332
 49786|  store ptr @core::fmt3num3impjNtB9_7Display3fmt, ptr %1595,                                                            ;L332
 49787|  %1596 = gep %42, i64 16                                                                                               ;L332
 49788|  store ptr %193, ptr %1596,                                                                                            ;L332
 49789|  %1597 = gep %42, i64 24                                                                                               ;L332
 49790|  store ptr @gc::simulation6entityNtB5_8PositionNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %1597,                       ;L332
 49791|  %1598 = gep %42, i64 32                                                                                               ;L332
 49792|  store ptr %43, ptr %1598,                                                                                             ;L332
 49793|  %1599 = gep %42, i64 40                                                                                               ;L332
 49794|  store ptr @gc::simulation4game10blackboardNtB5_11SmallActionNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %1599,         ;L332
 49795|  %1600 = gep %42, i64 48                                                                                               ;L332
 49796|  store ptr %49, ptr %1600,                                                                                             ;L332
 49797|  %1601 = gep %42, i64 56                                                                                               ;L332
 49798|  store ptr @core::fmt3num3impxNtB9_7Display3fmt, ptr %1601,                                                            ;L332
 49799|  %1602 = gep %42, i64 64                                                                                               ;L332
 49800|  store ptr %112, ptr %1602,                                                                                            ;L332
 49801|  %1603 = gep %42, i64 72                                                                                               ;L332
 49802|  store ptr @ai::plan_legacy8sub_planNtB5_7SubPlanNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %1603,                     ;L332
 49803|     ;; args[0..+8] = ptr @anon.282069a2ed2ad3a275929b639963fb55.141
 49804|     ;; args[8..+8] = ptr %42
 49805|     ;; self[0..+8] = ptr null
 49806|     ;; self[8..+8] = i64 undef
 49810|  invoke void @_RNvNvNtCs9LexZzt9XJB_5alloc3fmt6format12format_inner(ptr sret([24 x i8]) %44, ptr @anon.282069a2ed2ad3a275929b639963fb55.141, ptr %42)
 49811|  to label %1604 unwind label %1486                                                                                     ;L659<1275<659<332
 49812| 
 49813| 1604: ; preds = %1593
 49816|  invoke void @gc::simulation4game5frameNtB4_14DebugFrameData7add_log(ptr %8, ptr %5, ptr %4, ptr %44)
 49817|  to label %1490 unwind label %1486                                                                                     ;L332
 49818| 
 49819| 1605: ; preds = %1490
 49820|     ;; args[0..+8] = ptr %189
 49821|     ;; args[8..+8] = ptr %193
 49825|     ;; self = ptr %49
 49826|  %1606 = icmp ne i8 %1493, 10                                                                                          ;L309<339
 49827|  call void @llvm.assume(i1 %1606)                                                                                      ;L309<339
 49828|  %1607 = add nsw i8 %1493, -3                                                                                          ;L309<339
 49829|  %1608 = icmp samesign ugt i8 %1493, 2                                                                                 ;L309<339
 49830|  %1609 = select i1 %1608, i8 %1607, i8 7                                                                               ;L309<339
 49831|  switch i8 %1609, label %1610 [
 49832|  i8 0, label %1686
 49833|  i8 1, label %1686
 49834|  i8 2, label %1611
 49835|  i8 3, label %1615
 49836|  i8 4, label %1619
 49837|  i8 5, label %1686
 49838|  i8 6, label %1626
 49839|  i8 7, label %1633
 49840|  i8 8, label %1640
 49841|  i8 9, label %1647
 49842|  i8 10, label %1654
 49843|  i8 11, label %1658
 49844|  i8 12, label %1662
 49845|  i8 13, label %1666
 49846|  i8 14, label %1670
 49847|  i8 15, label %1674
 49848|  i8 16, label %1678
 49849|  ]                                                                                                                     ;L309<339
 49850| 
 49851| 1610: ; preds = %1605
 49852|  unreachable                                                                                                           ;L309<339
 49853| 
 49854| 1611: ; preds = %1605
 49855|     ;; action = ptr %49
 49856|     ;; self = ptr %49
 49857|  %1612 = gep %49, i64 16                                                                                               ;L285<316<339
 49858|  %1613 = load i64, ptr %1612, , !!53839, !!8                                                                           ;L285<316<339
 49859|  %1614 = gep %40, i64 8                                                                                                ;L285<316<339
 49860|  store i64 %1613, ptr %1614, , !!53842                                                                                 ;L285<316<339
 49861|  br label %1686                                                                                                        ;L316<339
 49862| 
 49863| 1615: ; preds = %1605
 49864|     ;; action = ptr %49
 49865|     ;; self = ptr %49
 49866|  %1616 = gep %49, i64 16                                                                                               ;L491<313<339
 49867|  %1617 = load i64, ptr %1616, , !!53839, !!8                                                                           ;L491<313<339
 49868|  %1618 = gep %40, i64 8                                                                                                ;L491<313<339
 49869|  store i64 %1617, ptr %1618, , !!53842                                                                                 ;L491<313<339
 49870|  br label %1686                                                                                                        ;L313<339
 49871| 
 49872| 1619: ; preds = %1605
 49873|     ;; action = ptr %49
 49874|     ;; self = ptr %49
 49875|  %1620 = gep %49, i64 24                                                                                               ;L665<314<339
 49876|  %1621 = load i64, ptr %1620, , !!53839, !!8                                                                           ;L665<314<339
 49877|  %1622 = gep %49, i64 32                                                                                               ;L665<314<339
 49878|  %1623 = load i64, ptr %1622, , !!53839, !!8                                                                           ;L665<314<339
 49879|  %1624 = gep %40, i64 8                                                                                                ;L665<314<339
 49880|  store i64 %1621, ptr %1624, , !!53842                                                                                 ;L665<314<339
 49881|  %1625 = gep %40, i64 16                                                                                               ;L665<314<339
 49882|  store i64 %1623, ptr %1625, , !!53842                                                                                 ;L665<314<339
 49883|  br label %1686                                                                                                        ;L314<339
 49884| 
 49885| 1626: ; preds = %1605
 49886|     ;; action = ptr %49
 49887|     ;; self = ptr %49
 49888|  %1627 = gep %49, i64 16                                                                                               ;L800<312<339
 49889|  %1628 = load i64, ptr %1627, , !!53839, !!8                                                                           ;L800<312<339
 49890|  %1629 = gep %49, i64 24                                                                                               ;L800<312<339
 49891|  %1630 = load i64, ptr %1629, , !!53839, !!8                                                                           ;L800<312<339
 49892|  %1631 = gep %40, i64 8                                                                                                ;L800<312<339
 49893|  store i64 %1628, ptr %1631, , !!53842                                                                                 ;L800<312<339
 49894|  %1632 = gep %40, i64 16                                                                                               ;L800<312<339
 49895|  store i64 %1630, ptr %1632, , !!53842                                                                                 ;L800<312<339
 49896|  br label %1686                                                                                                        ;L312<339
 49897| 
 49898| 1633: ; preds = %1605
 49899|     ;; action = ptr %49
 49900|     ;; self = ptr %49
 49901|  %1634 = gep %49, i64 56                                                                                               ;L1035<317<339
 49902|  %1635 = load i64, ptr %1634, , !!53839, !!8                                                                           ;L1035<317<339
 49903|  %1636 = gep %49, i64 64                                                                                               ;L1035<317<339
 49904|  %1637 = load i64, ptr %1636, , !!53839, !!8                                                                           ;L1035<317<339
 49905|  %1638 = gep %40, i64 8                                                                                                ;L1035<317<339
 49906|  store i64 %1635, ptr %1638, , !!53842                                                                                 ;L1035<317<339
 49907|  %1639 = gep %40, i64 16                                                                                               ;L1035<317<339
 49908|  store i64 %1637, ptr %1639, , !!53842                                                                                 ;L1035<317<339
 49909|  br label %1686                                                                                                        ;L317<339
 49910| 
 49911| 1640: ; preds = %1605
 49912|     ;; action = ptr %49
 49913|     ;; self = ptr %49
 49914|  %1641 = gep %49, i64 16                                                                                               ;L1123<318<339
 49915|  %1642 = load i64, ptr %1641, , !!53839, !!8                                                                           ;L1123<318<339
 49916|  %1643 = gep %49, i64 24                                                                                               ;L1123<318<339
 49917|  %1644 = load i64, ptr %1643, , !!53839, !!8                                                                           ;L1123<318<339
 49918|  %1645 = gep %40, i64 8                                                                                                ;L1123<318<339
 49919|  store i64 %1642, ptr %1645, , !!53842                                                                                 ;L1123<318<339
 49920|  %1646 = gep %40, i64 16                                                                                               ;L1123<318<339
 49921|  store i64 %1644, ptr %1646, , !!53842                                                                                 ;L1123<318<339
 49922|  br label %1686                                                                                                        ;L318<339
 49923| 
 49924| 1647: ; preds = %1605
 49925|     ;; action = ptr %49
 49926|     ;; self = ptr %49
 49927|  %1648 = gep %49, i64 32                                                                                               ;L1241<319<339
 49928|  %1649 = load i64, ptr %1648, , !!53839, !!8                                                                           ;L1241<319<339
 49929|  %1650 = gep %49, i64 40                                                                                               ;L1241<319<339
 49930|  %1651 = load i64, ptr %1650, , !!53839, !!8                                                                           ;L1241<319<339
 49931|  %1652 = gep %40, i64 8                                                                                                ;L1241<319<339
 49932|  store i64 %1649, ptr %1652, , !!53842                                                                                 ;L1241<319<339
 49933|  %1653 = gep %40, i64 16                                                                                               ;L1241<319<339
 49934|  store i64 %1651, ptr %1653, , !!53842                                                                                 ;L1241<319<339
 49935|  br label %1686                                                                                                        ;L319<339
 49936| 
 49937| 1654: ; preds = %1605
 49938|     ;; action = ptr %49
 49939|     ;; self = ptr %49
 49940|  %1655 = gep %49, i64 16                                                                                               ;L653<320<339
 49941|  %1656 = load i64, ptr %1655, , !!53839, !!8                                                                           ;L653<320<339
 49942|  %1657 = gep %40, i64 8                                                                                                ;L653<320<339
 49943|  store i64 %1656, ptr %1657, , !!53842                                                                                 ;L653<320<339
 49944|  br label %1686                                                                                                        ;L320<339
 49945| 
 49946| 1658: ; preds = %1605
 49947|     ;; action = ptr %49
 49948|     ;; self = ptr %49
 49949|  %1659 = gep %49, i64 104                                                                                              ;L404<321<339
 49950|  %1660 = load i64, ptr %1659, , !!53839, !!8                                                                           ;L404<321<339
 49951|  %1661 = gep %40, i64 8                                                                                                ;L404<321<339
 49952|  store i64 %1660, ptr %1661, , !!53842                                                                                 ;L404<321<339
 49953|  br label %1686                                                                                                        ;L321<339
 49954| 
 49955| 1662: ; preds = %1605
 49956|     ;; action = ptr %49
 49957|     ;; self = ptr %49
 49958|  %1663 = gep %49, i64 16                                                                                               ;L94<322<339
 49959|  %1664 = load i64, ptr %1663, , !!53839, !!8                                                                           ;L94<322<339
 49960|  %1665 = gep %40, i64 8                                                                                                ;L94<322<339
 49961|  store i64 %1664, ptr %1665, , !!53842                                                                                 ;L94<322<339
 49962|  br label %1686                                                                                                        ;L322<339
 49963| 
 49964| 1666: ; preds = %1605
 49965|     ;; action = ptr %49
 49966|     ;; self = ptr %49
 49967|  %1667 = gep %49, i64 16                                                                                               ;L160<323<339
 49968|  %1668 = load i64, ptr %1667, , !!53839, !!8                                                                           ;L160<323<339
 49969|  %1669 = gep %40, i64 8                                                                                                ;L160<323<339
 49970|  store i64 %1668, ptr %1669, , !!53842                                                                                 ;L160<323<339
 49971|  br label %1686                                                                                                        ;L323<339
 49972| 
 49973| 1670: ; preds = %1605
 49974|     ;; action = ptr %49
 49975|     ;; self = ptr %49
 49976|  %1671 = gep %49, i64 16                                                                                               ;L222<324<339
 49977|  %1672 = load i64, ptr %1671, , !!53839, !!8                                                                           ;L222<324<339
 49978|  %1673 = gep %40, i64 8                                                                                                ;L222<324<339
 49979|  store i64 %1672, ptr %1673, , !!53842                                                                                 ;L222<324<339
 49980|  br label %1686                                                                                                        ;L324<339
 49981| 
 49982| 1674: ; preds = %1605
 49983|     ;; action = ptr %49
 49984|     ;; self = ptr %49
 49985|  %1675 = gep %49, i64 16                                                                                               ;L287<325<339
 49986|  %1676 = load i64, ptr %1675, , !!53839, !!8                                                                           ;L287<325<339
 49987|  %1677 = gep %40, i64 8                                                                                                ;L287<325<339
 49988|  store i64 %1676, ptr %1677, , !!53842                                                                                 ;L287<325<339
 49989|  br label %1686                                                                                                        ;L325<339
 49990| 
 49991| 1678: ; preds = %1605
 49992|  br label %1686                                                                                                        ;L326<339
 49993| 
 49994| 1679: ; preds = %1701, %1490
 49995|  %1680 = phi i8 [ %1702, %1701 ], [ %1493, %1490 ]                                                                     ;L309<343
 49996|     ;; self = ptr %49
 49997|  %1681 = icmp ne i8 %1680, 10                                                                                          ;L309<343
 49998|  call void @llvm.assume(i1 %1681)                                                                                      ;L309<343
 49999|  %1682 = add nsw i8 %1680, -3                                                                                          ;L309<343
 50000|  %1683 = icmp samesign ugt i8 %1680, 2                                                                                 ;L309<343
 50001|  %1684 = select i1 %1683, i8 %1682, i8 7                                                                               ;L309<343
 50002|  switch i8 %1684, label %1685 [
 50003|  i8 0, label %1474
 50004|  i8 1, label %1474
 50005|  i8 2, label %1474
 50006|  i8 3, label %1474
 50007|  i8 4, label %1474
 50008|  i8 5, label %1474
 50009|  i8 6, label %1474
 50010|  i8 7, label %1474
 50011|  i8 8, label %1474
 50012|  i8 9, label %1474
 50013|  i8 10, label %1474
 50014|  i8 11, label %1704
 50015|  i8 12, label %1703
 50016|  i8 13, label %1703
 50017|  i8 14, label %1703
 50018|  i8 15, label %1703
 50019|  i8 16, label %1474
 50020|  ]                                                                                                                     ;L309<343
 50021| 
 50022| 1685: ; preds = %1679
 50023|  unreachable                                                                                                           ;L309<343
 50024| 
 50025| 1686: ; preds = %1678, %1674, %1670, %1666, %1662, %1658, %1654, %1647, %1640, %1633, %1626, %1619, %1615, %1611, %1605, %1605, %1605
 50026|  %1687 = phi i64 [ 10, %1678 ], [ 9, %1674 ], [ 8, %1670 ], [ 7, %1666 ], [ 6, %1662 ], [ 4, %1658 ], [ 2, %1654 ], [ 3, %1647 ], [ 3, %1640 ], [ 3, %1633 ], [ 1, %1626 ], [ 0, %1605 ], [ 3, %1619 ], [ 2, %1615 ], [ 2, %1611 ], [ 0, %1605 ], [ 0, %1605 ]
 50027|  store i64 %1687, ptr %40, , !!53842                                                                                   ;L0<339
 50028|     ;; args[16..+8] = ptr %40
 50029|     ;; args[24..+8] = ptr %49
 50030|     ;; args[32..+8] = ptr %112
 50032|     ;; self = ptr %58
 50033|  %1688 = load i64, ptr %1371, , !!8                                                                                    ;L1617<339
 50034|  store i64 %1688, ptr %39,                                                                                             ;L1617<339
 50035|     ;; args[40..+8] = ptr %39
 50037|  store ptr %189, ptr %38,                                                                                              ;L338
 50038|  %1689 = gep %38, i64 8                                                                                                ;L338
 50039|  store ptr @core::fmt3num3impjNtB9_7Display3fmt, ptr %1689,                                                            ;L338
 50040|  %1690 = gep %38, i64 16                                                                                               ;L338
 50041|  store ptr %193, ptr %1690,                                                                                            ;L338
 50042|  %1691 = gep %38, i64 24                                                                                               ;L338
 50043|  store ptr @gc::simulation6entityNtB5_8PositionNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %1691,                       ;L338
 50044|  %1692 = gep %38, i64 32                                                                                               ;L338
 50045|  store ptr %40, ptr %1692,                                                                                             ;L338
 50046|  %1693 = gep %38, i64 40                                                                                               ;L338
 50047|  store ptr @gc::simulation4game10blackboardNtB5_11SmallActionNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %1693,         ;L338
 50048|  %1694 = gep %38, i64 48                                                                                               ;L338
 50049|  store ptr %49, ptr %1694,                                                                                             ;L338
 50050|  %1695 = gep %38, i64 56                                                                                               ;L338
 50051|  store ptr @core::fmt3num3impxNtB9_7Display3fmt, ptr %1695,                                                            ;L338
 50052|  %1696 = gep %38, i64 64                                                                                               ;L338
 50053|  store ptr %112, ptr %1696,                                                                                            ;L338
 50054|  %1697 = gep %38, i64 72                                                                                               ;L338
 50055|  store ptr @ai::plan_legacy8sub_planNtB5_7SubPlanNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %1697,                     ;L338
 50056|  %1698 = gep %38, i64 80                                                                                               ;L338
 50057|  store ptr %39, ptr %1698,                                                                                             ;L338
 50058|  %1699 = gep %38, i64 88                                                                                               ;L338
 50059|  store ptr @core::fmt3num3impjNtB9_7Display3fmt, ptr %1699,                                                            ;L338
 50060|     ;; args[0..+8] = ptr @anon.282069a2ed2ad3a275929b639963fb55.142
 50061|     ;; args[8..+8] = ptr %38
 50062|     ;; self[0..+8] = ptr null
 50063|     ;; self[8..+8] = i64 undef
 50067|  invoke void @_RNvNvNtCs9LexZzt9XJB_5alloc3fmt6format12format_inner(ptr sret([24 x i8]) %41, ptr @anon.282069a2ed2ad3a275929b639963fb55.142, ptr %38)
 50068|  to label %1700 unwind label %1486                                                                                     ;L659<1275<659<338
 50069| 
 50070| 1700: ; preds = %1686
 50074|  invoke void @gc::simulation4game5frameNtB4_14DebugFrameData7add_log(ptr %8, ptr %5, ptr %4, ptr %41)
 50075|  to label %1701 unwind label %1486                                                                                     ;L338
 50076| 
 50077| 1701: ; preds = %1700
 50078|  %1702 = load i8, ptr %1481, , !!53696                                                                                 ;L309<343
 50079|  br label %1679                                                                                                        ;L338
 50080| 
 50081| 1703: ; preds = %1679, %1679, %1679, %1679
 50082|     ;; action = ptr %49
 50083|     ;; self = ptr %49
 50084|     ;; target_id = i64 %1707
 50085|  br label %1704                                                                                                        ;L342
 50086| 
 50087| 1704: ; preds = %1703, %1679
 50088|  %1705 = phi i64 [ 104, %1679 ], [ 16, %1703 ]
 50089|  %1706 = gep %49, i64 %1705                                                                                            ;L0<343
 50090|  %1707 = load i64, ptr %1706, , !!53696, !!8                                                                           ;L0<343
 50091|     ;; target_id = i64 %1707
 50092|  %1708 = gep %456, i64 496                                                                                             ;L344
 50093|  %1709 = load ptr, ptr %1708, , !!8                                                                                    ;L344
 50094|  %1710 = invoke ptr %1709(ptr %454, i64 %1707)
 50095|  to label %1711 unwind label %1486                                                                                     ;L344
 50096| 
 50097| 1711: ; preds = %1704
 50098|     ;; self = ptr %1710
 50099|     ;; f = ptr %200
 50100|  %1712 = icmp eq ptr %1710, null                                                                                       ;L659<344
 50101|  br i1 %1712, label %1474, label %1713                                                                                 ;L659<344
 50102| 
 50103| 1713: ; preds = %1711
 50104|     ;; x = ptr %1710
 50105|  %1714 = load i64, ptr %200, , !!8                                                                                     ;L661<344
 50107|     ;; t = ptr %1710
 50108|     ;; self = ptr %1710
 50109|     ;; self = ptr %1710
 50112|  %1715 = load i64, ptr %1710, , !!8                                                                                    ;L1127<264<344<661<344
 50113|     ;; __self_discr = i64 %1715
 50114|     ;; __arg1_discr = i64 %1714
 50115|  %1716 = icmp eq i64 %1715, %1714                                                                                      ;L1127<264<344<661<344
 50116|  br i1 %1716, label %1717, label %1725                                                                                 ;L1127<264<344<661<344
 50117| 
 50118| 1717: ; preds = %1713
 50119|  %1718 = gep %200, i64 8                                                                                               ;L661<344
 50120|  %1719 = load i64, ptr %1718,                                                                                          ;L661<344
 50121|  %1720 = gep %1710, i64 8                                                                                              ;L1127<264<344<661<344
 50122|  %1721 = icmp ne i64 %1714, 0                                                                                          ;L1127<264<344<661<344
 50123|  %1722 = load i64, ptr %1720,
 50124|  %1723 = icmp eq i64 %1722, %1719
 50125|  %1724 = select i1 %1721, i1 true, i1 %1723                                                                            ;L1127<264<344<661<344
 50126|  br i1 %1724, label %1474, label %1725                                                                                 ;L1127<264<344<661<344
 50127| 
 50128| 1725: ; preds = %1717, %1713
 50129|     ;; self = ptr %1710
 50130|  %1726 = gep %1710, i64 104                                                                                            ;L1404<344<661<344
 50131|  %1727 = load i64, ptr %1726, , !!8                                                                                    ;L1404<344<661<344
 50132|  %1728 = icmp eq i64 %1727, 13                                                                                         ;L1404<344<661<344
 50133|  br i1 %1728, label %1729, label %1474                                                                                 ;L344
 50134| 
 50135| 1729: ; preds = %1725
 50137|  %1730 = load i8, ptr %1481, , !!8                                                                                     ;L345
 50138|  %1731 = icmp ne i8 %1730, 10                                                                                          ;L345
 50139|  call void @llvm.assume(i1 %1731)                                                                                      ;L345
 50140|  %1732 = icmp eq i8 %1730, 14                                                                                          ;L345
 50141|  %1733 = gep %49, i64 156                                                                                              ;L345
 50142|  %1734 = load i8, ptr %1733,                                                                                           ;L345
 50143|  %1735 = select i1 %1732, i8 %1734, i8 0                                                                               ;L345
 50144|  store i8 %1735, ptr %37,                                                                                              ;L345
 50148|     ;; args = ptr %112
 50150|  store ptr %112, ptr %33,                                                                                              ;L347
 50151|  %1736 = gep %33, i64 8                                                                                                ;L347
 50152|  store ptr @ai::plan_legacy8sub_planNtB5_7SubPlanNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %1736,                     ;L347
 50153|     ;; args[0..+8] = ptr @anon.282069a2ed2ad3a275929b639963fb55.62
 50154|     ;; args[8..+8] = ptr %33
 50155|     ;; self[0..+8] = ptr null
 50156|     ;; self[8..+8] = i64 undef
 50160|  invoke void @_RNvNvNtCs9LexZzt9XJB_5alloc3fmt6format12format_inner(ptr sret([24 x i8]) %34, ptr @anon.282069a2ed2ad3a275929b639963fb55.62, ptr %33)
 50161|  to label %1739 unwind label %1486                                                                                     ;L659<1275<659<347
 50162| 
 50163| 1737: ; preds = %1806, %1798
 50164|  %1738 = cleanuppad within none []
 50165|  call fastcc void @core::ptr9drop_glueNtNtCs9LexZzt9XJB_5alloc6string6StringECshdEBA0ozCnw_7game_ai(ptr %35) #31 [ "funclet"(token %1738) ] ;L349
 50166|  cleanupret from %1738 unwind label %1486                                                                              ;L349
 50167| 
 50168| 1739: ; preds = %1729
 50170|  call void @llvm.memcpy.p0.p0.i64(ptr %35, ptr %34, i64 24, i1 false)                                                  ;L614<347
 50172|     ;; self = ptr %35
 50173|     ;; self = ptr %35
 50174|  %1740 = gep %35, i64 8                                                                                                ;L614<609<296<1968<1864<1064<2836<348
 50175|  %1741 = load ptr, ptr %1740, , !!8, !!8                                                                               ;L614<609<296<1968<1864<1064<2836<348
 50176|  %1742 = gep %35, i64 16                                                                                               ;L1864<1064<2836<348
 50177|  %1743 = load i64, ptr %1742, , !!8                                                                                    ;L1864<1064<2836<348
 50178|     ;; self[0..+8] = ptr %1741
 50179|     ;; haystack[0..+8] = ptr %1741
 50180|     ;; self[8..+8] = i64 %1743
 50181|     ;; haystack[8..+8] = i64 %1743
 50183|     ;; haystack[0..+8] = ptr %1741
 50184|     ;; self[0..+8] = ptr %1741
 50185|     ;; haystack[8..+8] = i64 %1743
 50186|     ;; self[8..+8] = i64 %1743
 50187|     ;; self[0..+8] = ptr %1741
 50188|     ;; self[8..+8] = i64 %1743
 50189|  %1744 = gep %1741, i64 %1743                                                                                          ;L961<100<1042<1064<1121<680<739<1659<348
 50190|     ;; self = ptr undef
 50191|     ;; self = ptr undef
 50194|     ;; self = ptr undef
 50195|     ;; self = ptr undef
 50196|  br label %1745                                                                                                        ;L250<787<663<512<348
 50197| 
 50198| 1745: ; preds = %1789, %1739
 50199|  %1746 = phi i64 [ %1795, %1789 ], [ 0, %1739 ]
 50200|  %1747 = phi ptr [ %1790, %1789 ], [ %1741, %1739 ]
 50201|     ;; self = ptr undef
 50202|     ;; s = ptr undef
 50203|     ;; self = ptr undef
 50204|     ;; pointee_size = i64 1
 50205|     ;; end = ptr %1744
 50206|     ;; self = ptr %1744
 50207|     ;; subtracted = ptr %1747
 50208|     ;; self = ptr %1744
 50209|     ;; origin = ptr %1747
 50210|     ;; origin = ptr %1747
 50211|     ;; self = ptr %1744
 50212|  %1748 = ptrtoint ptr %1747 to i64                                                                                     ;L729<887<950<57<695<251<787<663<512<348
 50213|     ;; pre_len = !DIArgList(ptr %1744, i64 %1748)
 50214|     ;; self = ptr undef
 50216|     ;; self = ptr undef
 50217|     ;; pointee_size = i64 1
 50218|     ;; end = ptr %1744
 50219|     ;; self = ptr %1744
 50220|     ;; subtracted = ptr %1747
 50221|     ;; self = ptr %1744
 50222|     ;; origin = ptr %1747
 50223|     ;; origin = ptr %1747
 50224|     ;; self = ptr %1744
 50225|     ;; pre_len = !DIArgList(ptr %1744, i64 %1748)
 50226|     ;; self = ptr undef
 50227|     ;; bytes = ptr undef
 50228|     ;; width = i32 2
 50229|     ;; self = ptr undef
 50230|     ;; count = i64 1
 50231|     ;; ptr = ptr %1747
 50232|     ;; self = ptr %1747
 50233|     ;; end_or_len = ptr %1744
 50236|  %1749 = icmp eq ptr %1747, %1744                                                                                      ;L1714<180<37<42<184<696<251<787<663<512<348
 50237|  br i1 %1749, label %1798, label %1750                                                                                 ;L180<37<42<184<696<251<787<663<512<348
 50238| 
 50239| 1750: ; preds = %1745
 50240|  %1751 = gep %1747, i64 1                                                                                              ;L656<185<37<42<184<696<251<787<663<512<348
 50241|  %1752 = load i8, ptr %1747, , !!54280, !!8                                                                            ;L37<42<184<696<251<787<663<512<348
 50242|     ;; x = i8 %1752
 50243|     ;; byte = i8 %1752
 50244|  %1753 = icmp sgt i8 %1752, -1                                                                                         ;L38<42<184<696<251<787<663<512<348
 50245|  br i1 %1753, label %1765, label %1754                                                                                 ;L38<42<184<696<251<787<663<512<348
 50246| 
 50247| 1754: ; preds = %1750
 50248|  %1755 = and i8 %1752, 31                                                                                              ;L11<45<42<184<696<251<787<663<512<348
 50249|  %1756 = zext nneg i8 %1755 to i32                                                                                     ;L11<45<42<184<696<251<787<663<512<348
 50250|     ;; init = i32 %1756
 50251|     ;; ch = i32 %1756
 50252|     ;; self = ptr undef
 50253|     ;; count = i64 1
 50254|     ;; ptr = ptr %1751
 50255|     ;; self = ptr %1751
 50256|     ;; end_or_len = ptr %1744
 50259|  %1757 = icmp ne ptr %1751, %1744                                                                                      ;L1714<180<48<42<184<696<251<787<663<512<348
 50260|  call void @llvm.assume(i1 %1757)                                                                                      ;L180<48<42<184<696<251<787<663<512<348
 50261|  %1758 = gep %1747, i64 2                                                                                              ;L656<185<48<42<184<696<251<787<663<512<348
 50262|  %1759 = load i8, ptr %1751, , !!54280, !!8                                                                            ;L48<42<184<696<251<787<663<512<348
 50263|     ;; y = i8 %1759
 50264|     ;; byte = i8 %1759
 50265|  %1760 = shl nuw nsw i32 %1756, 6                                                                                      ;L17<49<42<184<696<251<787<663<512<348
 50266|  %1761 = and i8 %1759, 63                                                                                              ;L17<49<42<184<696<251<787<663<512<348
 50267|  %1762 = zext nneg i8 %1761 to i32                                                                                     ;L17<49<42<184<696<251<787<663<512<348
 50268|  %1763 = or disjoint i32 %1760, %1762                                                                                  ;L17<49<42<184<696<251<787<663<512<348
 50269|     ;; ch = i32 %1763
 50270|  %1764 = icmp samesign ugt i8 %1752, -33                                                                               ;L50<42<184<696<251<787<663<512<348
 50271|  br i1 %1764, label %1767, label %1789                                                                                 ;L50<42<184<696<251<787<663<512<348
 50272| 
 50273| 1765: ; preds = %1750
 50274|  %1766 = zext nneg i8 %1752 to i32                                                                                     ;L39<42<184<696<251<787<663<512<348
 50275|  br label %1789                                                                                                        ;L1<42<184<696<251<787<663<512<348
 50276| 
 50277| 1767: ; preds = %1754
 50278|     ;; self = ptr undef
 50279|     ;; count = i64 1
 50280|     ;; ptr = ptr %1758
 50281|     ;; self = ptr %1758
 50282|     ;; end_or_len = ptr %1744
 50285|  %1768 = icmp ne ptr %1758, %1744                                                                                      ;L1714<180<55<42<184<696<251<787<663<512<348
 50286|  call void @llvm.assume(i1 %1768)                                                                                      ;L180<55<42<184<696<251<787<663<512<348
 50287|  %1769 = gep %1747, i64 3                                                                                              ;L656<185<55<42<184<696<251<787<663<512<348
 50288|  %1770 = load i8, ptr %1758, , !!54280, !!8                                                                            ;L55<42<184<696<251<787<663<512<348
 50289|     ;; z = i8 %1770
 50290|     ;; byte = i8 %1770
 50291|     ;; ch = i32 %1762
 50292|  %1771 = shl nuw nsw i32 %1762, 6                                                                                      ;L17<56<42<184<696<251<787<663<512<348
 50293|  %1772 = and i8 %1770, 63                                                                                              ;L17<56<42<184<696<251<787<663<512<348
 50294|  %1773 = zext nneg i8 %1772 to i32                                                                                     ;L17<56<42<184<696<251<787<663<512<348
 50295|  %1774 = or disjoint i32 %1771, %1773                                                                                  ;L17<56<42<184<696<251<787<663<512<348
 50296|     ;; y_z = i32 %1774
 50297|     ;; ch = i32 %1774
 50298|  %1775 = shl nuw nsw i32 %1756, 12                                                                                     ;L57<42<184<696<251<787<663<512<348
 50299|  %1776 = or disjoint i32 %1774, %1775                                                                                  ;L57<42<184<696<251<787<663<512<348
 50300|     ;; ch = i32 %1776
 50301|  %1777 = icmp samesign ugt i8 %1752, -17                                                                               ;L58<42<184<696<251<787<663<512<348
 50302|  br i1 %1777, label %1778, label %1789                                                                                 ;L58<42<184<696<251<787<663<512<348
 50303| 
 50304| 1778: ; preds = %1767
 50305|     ;; self = ptr undef
 50306|     ;; count = i64 1
 50307|     ;; ptr = ptr %1769
 50308|     ;; self = ptr %1769
 50309|     ;; end_or_len = ptr %1744
 50312|  %1779 = icmp ne ptr %1769, %1744                                                                                      ;L1714<180<63<42<184<696<251<787<663<512<348
 50313|  call void @llvm.assume(i1 %1779)                                                                                      ;L180<63<42<184<696<251<787<663<512<348
 50314|  %1780 = gep %1747, i64 4                                                                                              ;L656<185<63<42<184<696<251<787<663<512<348
 50315|  %1781 = load i8, ptr %1769, , !!54280, !!8                                                                            ;L63<42<184<696<251<787<663<512<348
 50316|     ;; w = i8 %1781
 50317|     ;; byte = i8 %1781
 50318|  %1782 = shl nuw nsw i32 %1756, 18                                                                                     ;L64<42<184<696<251<787<663<512<348
 50319|  %1783 = and i32 %1782, 1835008                                                                                        ;L64<42<184<696<251<787<663<512<348
 50320|  %1784 = shl nuw nsw i32 %1774, 6                                                                                      ;L17<64<42<184<696<251<787<663<512<348
 50321|  %1785 = and i8 %1781, 63                                                                                              ;L17<64<42<184<696<251<787<663<512<348
 50322|  %1786 = zext nneg i8 %1785 to i32                                                                                     ;L17<64<42<184<696<251<787<663<512<348
 50323|  %1787 = or disjoint i32 %1784, %1786                                                                                  ;L17<64<42<184<696<251<787<663<512<348
 50324|  %1788 = or disjoint i32 %1787, %1783                                                                                  ;L64<42<184<696<251<787<663<512<348
 50325|     ;; ch = i32 %1788
 50326|  br label %1789                                                                                                        ;L58<42<184<696<251<787<663<512<348
 50327| 
 50328| 1789: ; preds = %1778, %1767, %1765, %1754
 50329|  %1790 = phi ptr [ %1769, %1767 ], [ %1780, %1778 ], [ %1758, %1754 ], [ %1751, %1765 ]                                ;L57<697<251<787<663<512<348
 50330|  %1791 = phi i32 [ %1776, %1767 ], [ %1788, %1778 ], [ %1763, %1754 ], [ %1766, %1765 ]
 50331|     ;; self[0..+4] = i32 1
 50332|     ;; self[4..+4] = i32 %1791
 50333|     ;; x = i32 %1791
 50334|     ;; ch = i32 %1791
 50335|     ;; i = i32 %1791
 50336|     ;; i = i32 %1791
 50337|  %1792 = icmp samesign ult i32 %1791, 1114112                                                                          ;L34<239<42<1162<42<184<696<251<787<663<512<348
 50338|  call void @llvm.assume(i1 %1792)                                                                                      ;L34<239<42<1162<42<184<696<251<787<663<512<348
 50339|     ;; ch = i32 %1791
 50340|     ;; index = i64 %1746
 50341|     ;; self = ptr undef
 50342|     ;; pointee_size = i64 1
 50343|     ;; end = ptr %1744
 50344|     ;; self = ptr %1744
 50345|     ;; subtracted = ptr %1790
 50346|     ;; self = ptr %1744
 50347|     ;; origin = ptr %1790
 50348|     ;; origin = ptr %1790
 50349|     ;; self = ptr %1744
 50350|  %1793 = ptrtoint ptr %1790 to i64                                                                                     ;L729<887<950<57<188<696<251<787<663<512<348
 50351|     ;; len = !DIArgList(ptr %1744, i64 %1793)
 50352|  %1794 = sub i64 %1746, %1748                                                                                          ;L189<696<251<787<663<512<348
 50353|  %1795 = add i64 %1794, %1793                                                                                          ;L189<696<251<787<663<512<348
 50354|     ;; i = i64 %1746
 50355|     ;; c = i32 %1791
 50356|     ;; self = ptr undef
 50357|     ;; pointee_size = i64 1
 50358|     ;; end = ptr %1744
 50359|     ;; self = ptr %1744
 50360|     ;; subtracted = ptr %1790
 50361|     ;; self = ptr %1744
 50362|     ;; origin = ptr %1790
 50363|     ;; origin = ptr %1790
 50364|     ;; self = ptr %1744
 50365|     ;; len = !DIArgList(ptr %1744, i64 %1793)
 50368|     ;; c = i32 %1791
 50370|     ;; c = i32 %1791
 50371|  %1796 = and i32 %1791, 2097143                                                                                        ;L348<641<699<251<787<663<512<348
 50372|  %1797 = icmp eq i32 %1796, 32                                                                                         ;L348<641<699<251<787<663<512<348
 50373|  br i1 %1797, label %1798, label %1745                                                                                 ;L251<787<663<512<348
 50374| 
 50375| 1798: ; preds = %1789, %1745
 50376|  %1799 = phi i64 [ %1746, %1789 ], [ %1743, %1745 ]                                                                    ;L0<512<348
 50377|     ;; self[8..+8] = i64 %1799
 50378|     ;; s[8..+8] = i64 %1799
 50379|     ;; s[8..+8] = i64 %1799
 50380|     ;; self[8..+8] = i64 %1799
 50381|     ;; self[8..+8] = i64 %1799
 50382|     ;; self[0..+8] = ptr %1741
 50383|     ;; s[0..+8] = ptr %1741
 50384|     ;; s[0..+8] = ptr %1741
 50385|     ;; self[0..+8] = ptr %1741
 50386|     ;; self[0..+8] = ptr %1741
 50387|     ;; self[0..+8] = ptr %1741
 50388|     ;; self[0..+8] = ptr %1741
 50389|     ;; self[0..+8] = ptr %1741
 50390|     ;; s[0..+8] = ptr %1741
 50391|     ;; self[8..+8] = i64 %1799
 50392|     ;; self[8..+8] = i64 %1799
 50393|     ;; self[8..+8] = i64 %1799
 50394|     ;; s[8..+8] = i64 %1799
 50395|     ;; len = i64 %1799
 50396|     ;; capacity = i64 %1799
 50397|     ;; capacity = i64 %1799
 50398|     ;; count = i64 %1799
 50399|     ;; count = i64 %1799
 50400|     ;; capacity = i64 %1799
 50401|     ;; additional = i64 %1799
 50402|     ;; elem_layout[0..+8] = i64 1
 50403|     ;; elem_layout[0..+8] = i64 1
 50404|     ;; elem_layout[8..+8] = i64 1
 50405|     ;; elem_layout[8..+8] = i64 1
 50407|  invoke void @_RNvMs4_NtCs9LexZzt9XJB_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCshdEBA0ozCnw_7game_ai(ptr sret([24 x i8]) %16, i64 %1799, i1 zeroext false, i64 1, i64 1)
 50408|  to label %1800 unwind label %1737                                                                                     ;L434<177<977<448<400<376<842<251<3127<3058<2907<348
 50409| 
 50410| 1800: ; preds = %1798
 50411|  %1801 = load i64, ptr %16, , !!8                                                                                      ;L434<177<977<448<400<376<842<251<3127<3058<2907<348
 50412|  %1802 = trunc nuw i64 %1801 to i1                                                                                     ;L434<177<977<448<400<376<842<251<3127<3058<2907<348
 50413|  %1803 = gep %16, i64 8                                                                                                ;L0<177<977<448<400<376<842<251<3127<3058<2907<348
 50414|  %1804 = load i64, ptr %1803, , !!8                                                                                    ;L0<177<977<448<400<376<842<251<3127<3058<2907<348
 50415|  %1805 = gep %16, i64 16                                                                                               ;L0<177<977<448<400<376<842<251<3127<3058<2907<348
 50416|  br i1 %1802, label %1806, label %1808                                                                                 ;L434<177<977<448<400<376<842<251<3127<3058<2907<348
 50417| 
 50418| 1806: ; preds = %1800
 50419|  %1807 = load i64, ptr %1805,                                                                                          ;L442<177<977<448<400<376<842<251<3127<3058<2907<348
 50420|     ;; err[0..+8] = i64 %1804
 50421|     ;; err[8..+8] = i64 %1807
 50422|  invoke void @_RNvNtCs9LexZzt9XJB_5alloc7raw_vec12handle_error(i64 %1804, i64 %1807) #32
 50423|  to label %203 unwind label %1737                                                                                      ;L442<177<977<448<400<376<842<251<3127<3058<2907<348
 50424| 
 50425| 1808: ; preds = %1800
 50426|  %1809 = load ptr, ptr %1805, , !!8, !!8                                                                               ;L435<177<977<448<400<376<842<251<3127<3058<2907<348
 50427|     ;; this[0..+8] = i64 %1804
 50428|     ;; this[8..+8] = ptr %1809
 50430|  %1810 = icmp ule i64 %1799, %1804                                                                                     ;L767<438<177<977<448<400<376<842<251<3127<3058<2907<348
 50431|     ;; cond = i1 true
 50432|  call void @llvm.assume(i1 %1810)                                                                                      ;L210<438<177<977<448<400<376<842<251<3127<3058<2907<348
 50434|     ;; bytes[0..+8] = i64 %1804
 50435|     ;; v[0..+8] = i64 %1804
 50436|     ;; bytes[8..+8] = ptr %1809
 50437|     ;; v[8..+8] = ptr %1809
 50438|     ;; bytes[16..+8] = i64 0
 50439|     ;; v[16..+8] = i64 0
 50440|  %1811 = icmp eq i64 %1799, 0                                                                                          ;L452<400<376<842<251<3127<3058<2907<348
 50441|  br i1 %1811, label %1812, label %1819                                                                                 ;L452<400<376<842<251<3127<3058<2907<348
 50442| 
 50443| 1812: ; preds = %1819, %1808
 50444|     ;; v[16..+8] = i64 %1799
 50445|     ;; bytes[16..+8] = i64 %1799
 50446|  store i64 %1804, ptr %36,                                                                                             ;L1023<251<3127<3058<2907<348
 50447|  %1813 = gep %36, i64 8                                                                                                ;L1023<251<3127<3058<2907<348
 50448|  store ptr %1809, ptr %1813,                                                                                           ;L1023<251<3127<3058<2907<348
 50449|  %1814 = gep %36, i64 16                                                                                               ;L1023<251<3127<3058<2907<348
 50450|  store i64 %1799, ptr %1814,                                                                                           ;L1023<251<3127<3058<2907<348
 50453|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %35)
 50454|  to label %1818 unwind label %1815                                                                                     ;L825<825<349
 50455| 
 50456| 1815: ; preds = %1812
 50457|  %1816 = cleanuppad within none []
 50459|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %35) [ "funclet"(token %1816) ]
 50460|  to label %1817 unwind label %1486                                                                                     ;L825<825<825<349
 50461| 
 50462| 1817: ; preds = %1815
 50463|  cleanupret from %1816 unwind label %1486
 50464| 
 50465| 1818: ; preds = %1812
 50467|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %35)
 50468|  to label %1820 unwind label %1486                                                                                     ;L825<825<825<349
 50469| 
 50470| 1819: ; preds = %1808
 50471|     ;; self = ptr %1741
 50472|     ;; src = ptr %1741
 50473|     ;; dest = ptr %1809
 50474|     ;; dst = ptr %1809
 50475|  call void @llvm.memcpy.p0.p0.i64(ptr %1809, ptr %1741, i64 %1799, i1 false)                                           ;L551<1252<454<400<376<842<251<3127<3058<2907<348
 50476|     ;; bytes[16..+8] = i64 %1799
 50477|     ;; v[16..+8] = i64 %1799
 50478|  br label %1812                                                                                                        ;L452<400<376<842<251<3127<3058<2907<348
 50479| 
 50480| 1820: ; preds = %1818
 50482|     ;; args[0..+8] = ptr %189
 50483|     ;; args[8..+8] = ptr %193
 50487|     ;; self = ptr %49
 50488|  %1821 = load i8, ptr %1481, , !!54505, !!8                                                                            ;L309<351
 50489|  %1822 = icmp ne i8 %1821, 10                                                                                          ;L309<351
 50490|  call void @llvm.assume(i1 %1822)                                                                                      ;L309<351
 50491|  %1823 = add nsw i8 %1821, -3                                                                                          ;L309<351
 50492|  %1824 = icmp samesign ugt i8 %1821, 2                                                                                 ;L309<351
 50493|  %1825 = select i1 %1824, i8 %1823, i8 7                                                                               ;L309<351
 50494|  switch i8 %1825, label %1826 [
 50495|  i8 0, label %1897
 50496|  i8 1, label %1897
 50497|  i8 2, label %1827
 50498|  i8 3, label %1831
 50499|  i8 4, label %1835
 50500|  i8 5, label %1897
 50501|  i8 6, label %1842
 50502|  i8 7, label %1849
 50503|  i8 8, label %1856
 50504|  i8 9, label %1863
 50505|  i8 10, label %1870
 50506|  i8 11, label %1874
 50507|  i8 12, label %1878
 50508|  i8 13, label %1882
 50509|  i8 14, label %1886
 50510|  i8 15, label %1890
 50511|  i8 16, label %1894
 50512|  ]                                                                                                                     ;L309<351
 50513| 
 50514| 1826: ; preds = %1820
 50515|  unreachable                                                                                                           ;L309<351
 50516| 
 50517| 1827: ; preds = %1820
 50518|     ;; action = ptr %49
 50519|     ;; self = ptr %49
 50520|  %1828 = gep %49, i64 16                                                                                               ;L285<316<351
 50521|  %1829 = load i64, ptr %1828, , !!54505, !!8                                                                           ;L285<316<351
 50522|  %1830 = gep %31, i64 8                                                                                                ;L285<316<351
 50523|  store i64 %1829, ptr %1830, , !!54508                                                                                 ;L285<316<351
 50524|  br label %1897                                                                                                        ;L316<351
 50525| 
 50526| 1831: ; preds = %1820
 50527|     ;; action = ptr %49
 50528|     ;; self = ptr %49
 50529|  %1832 = gep %49, i64 16                                                                                               ;L491<313<351
 50530|  %1833 = load i64, ptr %1832, , !!54505, !!8                                                                           ;L491<313<351
 50531|  %1834 = gep %31, i64 8                                                                                                ;L491<313<351
 50532|  store i64 %1833, ptr %1834, , !!54508                                                                                 ;L491<313<351
 50533|  br label %1897                                                                                                        ;L313<351
 50534| 
 50535| 1835: ; preds = %1820
 50536|     ;; action = ptr %49
 50537|     ;; self = ptr %49
 50538|  %1836 = gep %49, i64 24                                                                                               ;L665<314<351
 50539|  %1837 = load i64, ptr %1836, , !!54505, !!8                                                                           ;L665<314<351
 50540|  %1838 = gep %49, i64 32                                                                                               ;L665<314<351
 50541|  %1839 = load i64, ptr %1838, , !!54505, !!8                                                                           ;L665<314<351
 50542|  %1840 = gep %31, i64 8                                                                                                ;L665<314<351
 50543|  store i64 %1837, ptr %1840, , !!54508                                                                                 ;L665<314<351
 50544|  %1841 = gep %31, i64 16                                                                                               ;L665<314<351
 50545|  store i64 %1839, ptr %1841, , !!54508                                                                                 ;L665<314<351
 50546|  br label %1897                                                                                                        ;L314<351
 50547| 
 50548| 1842: ; preds = %1820
 50549|     ;; action = ptr %49
 50550|     ;; self = ptr %49
 50551|  %1843 = gep %49, i64 16                                                                                               ;L800<312<351
 50552|  %1844 = load i64, ptr %1843, , !!54505, !!8                                                                           ;L800<312<351
 50553|  %1845 = gep %49, i64 24                                                                                               ;L800<312<351
 50554|  %1846 = load i64, ptr %1845, , !!54505, !!8                                                                           ;L800<312<351
 50555|  %1847 = gep %31, i64 8                                                                                                ;L800<312<351
 50556|  store i64 %1844, ptr %1847, , !!54508                                                                                 ;L800<312<351
 50557|  %1848 = gep %31, i64 16                                                                                               ;L800<312<351
 50558|  store i64 %1846, ptr %1848, , !!54508                                                                                 ;L800<312<351
 50559|  br label %1897                                                                                                        ;L312<351
 50560| 
 50561| 1849: ; preds = %1820
 50562|     ;; action = ptr %49
 50563|     ;; self = ptr %49
 50564|  %1850 = gep %49, i64 56                                                                                               ;L1035<317<351
 50565|  %1851 = load i64, ptr %1850, , !!54505, !!8                                                                           ;L1035<317<351
 50566|  %1852 = gep %49, i64 64                                                                                               ;L1035<317<351
 50567|  %1853 = load i64, ptr %1852, , !!54505, !!8                                                                           ;L1035<317<351
 50568|  %1854 = gep %31, i64 8                                                                                                ;L1035<317<351
 50569|  store i64 %1851, ptr %1854, , !!54508                                                                                 ;L1035<317<351
 50570|  %1855 = gep %31, i64 16                                                                                               ;L1035<317<351
 50571|  store i64 %1853, ptr %1855, , !!54508                                                                                 ;L1035<317<351
 50572|  br label %1897                                                                                                        ;L317<351
 50573| 
 50574| 1856: ; preds = %1820
 50575|     ;; action = ptr %49
 50576|     ;; self = ptr %49
 50577|  %1857 = gep %49, i64 16                                                                                               ;L1123<318<351
 50578|  %1858 = load i64, ptr %1857, , !!54505, !!8                                                                           ;L1123<318<351
 50579|  %1859 = gep %49, i64 24                                                                                               ;L1123<318<351
 50580|  %1860 = load i64, ptr %1859, , !!54505, !!8                                                                           ;L1123<318<351
 50581|  %1861 = gep %31, i64 8                                                                                                ;L1123<318<351
 50582|  store i64 %1858, ptr %1861, , !!54508                                                                                 ;L1123<318<351
 50583|  %1862 = gep %31, i64 16                                                                                               ;L1123<318<351
 50584|  store i64 %1860, ptr %1862, , !!54508                                                                                 ;L1123<318<351
 50585|  br label %1897                                                                                                        ;L318<351
 50586| 
 50587| 1863: ; preds = %1820
 50588|     ;; action = ptr %49
 50589|     ;; self = ptr %49
 50590|  %1864 = gep %49, i64 32                                                                                               ;L1241<319<351
 50591|  %1865 = load i64, ptr %1864, , !!54505, !!8                                                                           ;L1241<319<351
 50592|  %1866 = gep %49, i64 40                                                                                               ;L1241<319<351
 50593|  %1867 = load i64, ptr %1866, , !!54505, !!8                                                                           ;L1241<319<351
 50594|  %1868 = gep %31, i64 8                                                                                                ;L1241<319<351
 50595|  store i64 %1865, ptr %1868, , !!54508                                                                                 ;L1241<319<351
 50596|  %1869 = gep %31, i64 16                                                                                               ;L1241<319<351
 50597|  store i64 %1867, ptr %1869, , !!54508                                                                                 ;L1241<319<351
 50598|  br label %1897                                                                                                        ;L319<351
 50599| 
 50600| 1870: ; preds = %1820
 50601|     ;; action = ptr %49
 50602|     ;; self = ptr %49
 50603|  %1871 = gep %49, i64 16                                                                                               ;L653<320<351
 50604|  %1872 = load i64, ptr %1871, , !!54505, !!8                                                                           ;L653<320<351
 50605|  %1873 = gep %31, i64 8                                                                                                ;L653<320<351
 50606|  store i64 %1872, ptr %1873, , !!54508                                                                                 ;L653<320<351
 50607|  br label %1897                                                                                                        ;L320<351
 50608| 
 50609| 1874: ; preds = %1820
 50610|     ;; action = ptr %49
 50611|     ;; self = ptr %49
 50612|  %1875 = gep %49, i64 104                                                                                              ;L404<321<351
 50613|  %1876 = load i64, ptr %1875, , !!54505, !!8                                                                           ;L404<321<351
 50614|  %1877 = gep %31, i64 8                                                                                                ;L404<321<351
 50615|  store i64 %1876, ptr %1877, , !!54508                                                                                 ;L404<321<351
 50616|  br label %1897                                                                                                        ;L321<351
 50617| 
 50618| 1878: ; preds = %1820
 50619|     ;; action = ptr %49
 50620|     ;; self = ptr %49
 50621|  %1879 = gep %49, i64 16                                                                                               ;L94<322<351
 50622|  %1880 = load i64, ptr %1879, , !!54505, !!8                                                                           ;L94<322<351
 50623|  %1881 = gep %31, i64 8                                                                                                ;L94<322<351
 50624|  store i64 %1880, ptr %1881, , !!54508                                                                                 ;L94<322<351
 50625|  br label %1897                                                                                                        ;L322<351
 50626| 
 50627| 1882: ; preds = %1820
 50628|     ;; action = ptr %49
 50629|     ;; self = ptr %49
 50630|  %1883 = gep %49, i64 16                                                                                               ;L160<323<351
 50631|  %1884 = load i64, ptr %1883, , !!54505, !!8                                                                           ;L160<323<351
 50632|  %1885 = gep %31, i64 8                                                                                                ;L160<323<351
 50633|  store i64 %1884, ptr %1885, , !!54508                                                                                 ;L160<323<351
 50634|  br label %1897                                                                                                        ;L323<351
 50635| 
 50636| 1886: ; preds = %1820
 50637|     ;; action = ptr %49
 50638|     ;; self = ptr %49
 50639|  %1887 = gep %49, i64 16                                                                                               ;L222<324<351
 50640|  %1888 = load i64, ptr %1887, , !!54505, !!8                                                                           ;L222<324<351
 50641|  %1889 = gep %31, i64 8                                                                                                ;L222<324<351
 50642|  store i64 %1888, ptr %1889, , !!54508                                                                                 ;L222<324<351
 50643|  br label %1897                                                                                                        ;L324<351
 50644| 
 50645| 1890: ; preds = %1820
 50646|     ;; action = ptr %49
 50647|     ;; self = ptr %49
 50648|  %1891 = gep %49, i64 16                                                                                               ;L287<325<351
 50649|  %1892 = load i64, ptr %1891, , !!54505, !!8                                                                           ;L287<325<351
 50650|  %1893 = gep %31, i64 8                                                                                                ;L287<325<351
 50651|  store i64 %1892, ptr %1893, , !!54508                                                                                 ;L287<325<351
 50652|  br label %1897                                                                                                        ;L325<351
 50653| 
 50654| 1894: ; preds = %1820
 50655|  br label %1897                                                                                                        ;L326<351
 50656| 
 50657| 1895: ; preds = %1910, %1897
 50658|  %1896 = cleanuppad within none []
 50659|  call fastcc void @core::ptr9drop_glueNtNtCs9LexZzt9XJB_5alloc6string6StringECshdEBA0ozCnw_7game_ai(ptr %36) #31 [ "funclet"(token %1896) ] ;L352
 50660|  cleanupret from %1896 unwind label %1486                                                                              ;L352
 50661| 
 50662| 1897: ; preds = %1894, %1890, %1886, %1882, %1878, %1874, %1870, %1863, %1856, %1849, %1842, %1835, %1831, %1827, %1820, %1820, %1820
 50663|  %1898 = phi i64 [ 10, %1894 ], [ 9, %1890 ], [ 8, %1886 ], [ 7, %1882 ], [ 6, %1878 ], [ 4, %1874 ], [ 2, %1870 ], [ 3, %1863 ], [ 3, %1856 ], [ 3, %1849 ], [ 1, %1842 ], [ 0, %1820 ], [ 3, %1835 ], [ 2, %1831 ], [ 2, %1827 ], [ 0, %1820 ], [ 0, %1820 ]
 50664|  store i64 %1898, ptr %31, , !!54508                                                                                   ;L0<351
 50665|     ;; args[16..+8] = ptr %31
 50666|     ;; args[24..+8] = ptr %49
 50667|     ;; args[32..+8] = ptr %37
 50668|     ;; args[40..+8] = ptr %36
 50670|  store ptr %189, ptr %30,                                                                                              ;L350
 50671|  %1899 = gep %30, i64 8                                                                                                ;L350
 50672|  store ptr @core::fmt3num3impjNtB9_7Display3fmt, ptr %1899,                                                            ;L350
 50673|  %1900 = gep %30, i64 16                                                                                               ;L350
 50674|  store ptr %193, ptr %1900,                                                                                            ;L350
 50675|  %1901 = gep %30, i64 24                                                                                               ;L350
 50676|  store ptr @gc::simulation6entityNtB5_8PositionNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %1901,                       ;L350
 50677|  %1902 = gep %30, i64 32                                                                                               ;L350
 50678|  store ptr %31, ptr %1902,                                                                                             ;L350
 50679|  %1903 = gep %30, i64 40                                                                                               ;L350
 50680|  store ptr @gc::simulation4game10blackboardNtB5_11SmallActionNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %1903,         ;L350
 50681|  %1904 = gep %30, i64 48                                                                                               ;L350
 50682|  store ptr %49, ptr %1904,                                                                                             ;L350
 50683|  %1905 = gep %30, i64 56                                                                                               ;L350
 50684|  store ptr @core::fmt3num3impxNtB9_7Display3fmt, ptr %1905,                                                            ;L350
 50685|  %1906 = gep %30, i64 64                                                                                               ;L350
 50686|  store ptr %37, ptr %1906,                                                                                             ;L350
 50687|  %1907 = gep %30, i64 72                                                                                               ;L350
 50688|  store ptr @core::fmtbNtB5_7Display3fmt, ptr %1907,                                                                    ;L350
 50689|  %1908 = gep %30, i64 80                                                                                               ;L350
 50690|  store ptr %36, ptr %1908,                                                                                             ;L350
 50691|  %1909 = gep %30, i64 88                                                                                               ;L350
 50692|  store ptr @core::fmt7Display3fmt, ptr %1909,                                                                          ;L350
 50693|     ;; args[0..+8] = ptr @anon.282069a2ed2ad3a275929b639963fb55.143
 50694|     ;; args[8..+8] = ptr %30
 50695|     ;; self[0..+8] = ptr null
 50696|     ;; self[8..+8] = i64 undef
 50700|  invoke void @_RNvNvNtCs9LexZzt9XJB_5alloc3fmt6format12format_inner(ptr sret([24 x i8]) %32, ptr @anon.282069a2ed2ad3a275929b639963fb55.143, ptr %30)
 50701|  to label %1910 unwind label %1895                                                                                     ;L659<1275<659<350
 50702| 
 50703| 1910: ; preds = %1897
 50706|  invoke void @gc::simulation4game5frameNtB4_14DebugFrameData7add_log(ptr %8, ptr %5, ptr %4, ptr %32)
 50707|  to label %1911 unwind label %1895                                                                                     ;L350
 50708| 
 50709| 1911: ; preds = %1910
 50712|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %36)
 50713|  to label %1915 unwind label %1912                                                                                     ;L825<825<352
 50714| 
 50715| 1912: ; preds = %1911
 50716|  %1913 = cleanuppad within none []
 50718|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %36) [ "funclet"(token %1913) ]
 50719|  to label %1914 unwind label %1486                                                                                     ;L825<825<825<352
 50720| 
 50721| 1914: ; preds = %1912
 50722|  cleanupret from %1913 unwind label %1486
 50723| 
 50724| 1915: ; preds = %1911
 50726|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %36)
 50727|  to label %1916 unwind label %1486                                                                                     ;L825<825<825<352
 50728| 
 50729| 1916: ; preds = %1915
 50732|  br label %1474                                                                                                        ;L344
 50733| 
 50734| 1917: ; preds = %1474
 50735|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %95, i64 5384, i1 false)                                                 ;L357
 50736|  %1918 = load i64, ptr %49, , !!8                                                                                      ;L357
 50737|  %1919 = gep %0, i64 5392                                                                                              ;L357
 50738|  call void @llvm.memcpy.p0.p0.i64(ptr %1919, ptr %1475, i64 184, i1 false)                                             ;L357
 50739|  %1920 = gep %0, i64 5384                                                                                              ;L357
 50740|  store i64 %1918, ptr %1920,                                                                                           ;L357
 50743|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %51)
 50744|  to label %1924 unwind label %1921                                                                                     ;L825<416
 50745| 
 50746| 1921: ; preds = %1917
 50747|  %1922 = cleanuppad within none []
 50749|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %51) [ "funclet"(token %1922) ]
 50750|  to label %1923 unwind label %1381                                                                                     ;L825<825<416
 50751| 
 50752| 1923: ; preds = %1921
 50753|  cleanupret from %1922 unwind label %1381
 50754| 
 50755| 1924: ; preds = %1917
 50757|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %51)
 50758|  to label %1930 unwind label %1381                                                                                     ;L825<825<416
 50759| 
 50760| 1925: ; preds = %1474
 50761|     ;; self = ptr %1475
 50762|  %1926 = add nsw i8 %1477, -3                                                                                          ;L309<360
 50763|  %1927 = icmp samesign ugt i8 %1477, 2                                                                                 ;L309<360
 50764|  %1928 = select i1 %1927, i8 %1926, i8 7                                                                               ;L309<360
 50765|  switch i8 %1928, label %1929 [
 50766|  i8 16, label %2211
 50767|  i8 15, label %2211
 50768|  i8 2, label %2215
 50769|  i8 3, label %2215
 50770|  i8 4, label %2215
 50771|  i8 14, label %2211
 50772|  i8 6, label %2211
 50773|  i8 7, label %2215
 50774|  i8 8, label %2215
 50775|  i8 9, label %2215
 50776|  i8 10, label %2215
 50777|  i8 11, label %1936
 50778|  i8 12, label %2211
 50779|  i8 13, label %2211
 50780|  i8 0, label %2211
 50781|  i8 1, label %2211
 50782|  i8 5, label %2211
 50783|  ]                                                                                                                     ;L309<360
 50784| 
 50785| 1929: ; preds = %1925
 50786|  unreachable                                                                                                           ;L309<360
 50787| 
 50788| 1930: ; preds = %1924
 50790|  br label %1931                                                                                                        ;L1
 50791| 
 50792| 1931: ; preds = %1930, %1414
 50795|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB12_(ptr %58)
 50796|  to label %1935 unwind label %1932                                                                                     ;L825<416
 50797| 
 50798| 1932: ; preds = %1931
 50799|  %1933 = cleanuppad within none []
 50801|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB19_(ptr %58) [ "funclet"(token %1933) ]
 50802|  to label %1934 unwind label %679                                                                                      ;L825<825<416
 50803| 
 50804| 1934: ; preds = %1932
 50805|  cleanupret from %1933 unwind label %679
 50806| 
 50807| 1935: ; preds = %1931
 50809|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB19_(ptr %58)
 50810|  to label %2486 unwind label %679                                                                                      ;L825<825<416
 50811| 
 50812| 1936: ; preds = %1925
 50813|     ;; action = ptr %1475
 50814|     ;; self = ptr %1475
 50815|  %1937 = gep %49, i64 104                                                                                              ;L404<321<360
 50816|  %1938 = load i64, ptr %1937, , !!54633, !!8                                                                           ;L404<321<360
 50818|  store i64 %1938, ptr %29,                                                                                             ;L360
 50819|  %1939 = load i64, ptr %112, , !!8                                                                                     ;L361
 50820|  %1940 = icmp ne i64 %1939, 8                                                                                          ;L361
 50821|  call void @llvm.assume(i1 %1940)                                                                                      ;L361
 50822|  %1941 = icmp eq i64 %1939, 7                                                                                          ;L361
 50823|  br i1 %1941, label %1942, label %1946                                                                                 ;L361
 50824| 
 50825| 1942: ; preds = %1936
 50826|  %1943 = gep %456, i64 496                                                                                             ;L362
 50827|  %1944 = load ptr, ptr %1943, , !!8                                                                                    ;L362
 50828|  %1945 = invoke ptr %1944(ptr %454, i64 %1938)
 50829|  to label %1952 unwind label %1486                                                                                     ;L362
 50830| 
 50831| 1946: ; preds = %1956, %1952, %1936
 50834|     ;; self = ptr %58
 50835|     ;; self = ptr %58
 50836|  %1947 = load ptr, ptr %58, , !!8, !!8                                                                                 ;L138<2073<371
 50837|     ;; p = ptr %1947
 50838|  %1948 = load i64, ptr %1371, , !!8                                                                                    ;L2075<371
 50839|     ;; len = i64 %1948
 50840|     ;; count = i64 %1948
 50841|     ;; self[0..+8] = ptr %1947
 50842|     ;; slice[0..+8] = ptr %1947
 50843|     ;; self[8..+8] = i64 %1948
 50844|     ;; slice[8..+8] = i64 %1948
 50845|     ;; ptr = ptr %1947
 50846|     ;; self = ptr %1947
 50847|  %1949 = getelementptr { i64, { [177 x i8], i8, [6 x i8] } }, ptr %1947, i64 %1948                                     ;L961<100<1042<371
 50848|     ;; self[0..+8] = ptr %1947
 50849|     ;; self[8..+8] = ptr %1949
 50850|     ;; predicate = ptr %29
 50851|  store ptr %1947, ptr %27,                                                                                             ;L28<957<371
 50852|  %1950 = gep %27, i64 8                                                                                                ;L28<957<371
 50853|  store ptr %1949, ptr %1950,                                                                                           ;L28<957<371
 50854|  %1951 = gep %27, i64 16                                                                                               ;L28<957<371
 50855|  store ptr %29, ptr %1951,                                                                                             ;L28<957<371
 50856|  invoke void @core::iter8adapters6filter6FilterINtNtNtB2h_5slice4iter4IterBV_ENCNvMNtNtNtB11_11plan_legacy7handler7auctionNtB3E_17LegacyPlanHandler16get_small_actionsl_0EEB11_(ptr sret([32 x i8]) %28, ptr %27, ptr %655)
 50857|  to label %1984 unwind label %1486                                                                                     ;L370
 50858| 
 50859| 1952: ; preds = %1942
 50860|  %1953 = icmp eq ptr %1945, null                                                                                       ;L362
 50861|  br i1 %1953, label %1946, label %1954                                                                                 ;L362
 50862| 
 50863| 1954: ; preds = %1952
 50864|     ;; target = ptr %1945
 50865|     ;; self = ptr %1945
 50866|  %1955 = invoke i64 @ai::plan_legacy8sub_plan6battle24support_min_action_range(ptr %200, ptr %1945)
 50867|  to label %1956 unwind label %1486                                                                                     ;L363
 50868| 
 50869| 1956: ; preds = %1954
 50870|  %1957 = add i64 %1955, 25000                                                                                          ;L363
 50871|     ;; mr = i64 %1957
 50872|  %1958 = gep %1945, i64 1632                                                                                           ;L2158<364
 50873|  %1959 = load i64, ptr %1958, , !!8                                                                                    ;L2158<364
 50874|     ;; x1 = i64 %1959
 50875|     ;; self = i64 %1959
 50876|  %1960 = gep %1945, i64 1640                                                                                           ;L2158<364
 50877|  %1961 = load i64, ptr %1960, , !!8                                                                                    ;L2158<364
 50878|     ;; y1 = i64 %1961
 50879|     ;; self = i64 %1961
 50880|  %1962 = gep %200, i64 1632                                                                                            ;L2158<364
 50881|  %1963 = load i64, ptr %1962, , !!8                                                                                    ;L2158<364
 50882|     ;; x2 = i64 %1963
 50883|     ;; other = i64 %1963
 50884|  %1964 = gep %200, i64 1640                                                                                            ;L2158<364
 50885|  %1965 = load i64, ptr %1964, , !!8                                                                                    ;L2158<364
 50886|     ;; y2 = i64 %1965
 50887|     ;; other = i64 %1965
 50888|  %1966 = icmp ult i64 %1959, %1963                                                                                     ;L3147<7<2158<364
 50889|  %1967 = sub nuw i64 %1963, %1959                                                                                      ;L3147<7<2158<364
 50890|  %1968 = sub nuw i64 %1959, %1963                                                                                      ;L3147<7<2158<364
 50891|  %1969 = select i1 %1966, i64 %1967, i64 %1968                                                                         ;L3147<7<2158<364
 50892|     ;; dx = i64 %1969
 50893|  %1970 = icmp ult i64 %1961, %1965                                                                                     ;L3147<8<2158<364
 50894|  %1971 = sub nuw i64 %1965, %1961                                                                                      ;L3147<8<2158<364
 50895|  %1972 = sub nuw i64 %1961, %1965                                                                                      ;L3147<8<2158<364
 50896|  %1973 = select i1 %1970, i64 %1971, i64 %1972                                                                         ;L3147<8<2158<364
 50897|     ;; dy = i64 %1973
 50898|  %1974 = mul i64 %1969, %1969                                                                                          ;L9<2158<364
 50899|  %1975 = mul i64 %1973, %1973                                                                                          ;L9<2158<364
 50900|  %1976 = add i64 %1975, %1974                                                                                          ;L9<2158<364
 50901|  %1977 = mul i64 %1957, %1957                                                                                          ;L364
 50902|  %1978 = icmp ugt i64 %1976, %1977                                                                                     ;L364
 50903|  br i1 %1978, label %1979, label %1946                                                                                 ;L364
 50904| 
 50905| 1979: ; preds = %1956
 50906|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %95, i64 5384, i1 false)                                                 ;L365
 50907|  %1980 = load i64, ptr %49, , !!8                                                                                      ;L365
 50908|  %1981 = gep %0, i64 5392                                                                                              ;L365
 50909|  call void @llvm.memcpy.p0.p0.i64(ptr %1981, ptr %1475, i64 184, i1 false)                                             ;L365
 50910|  %1982 = gep %0, i64 5384                                                                                              ;L365
 50911|  store i64 %1980, ptr %1982,                                                                                           ;L365
 50912|  br label %1983                                                                                                        ;L1
 50913| 
 50914| 1983: ; preds = %2186, %1979
 50916|  br label %2187                                                                                                        ;L1
 50917| 
 50918| 1984: ; preds = %1946
 50920|     ;; self = ptr %28
 50921|     ;; self = ptr %28
 50922|  %1985 = gep %28, i64 24                                                                                               ;L1617<1636<378
 50923|  %1986 = load i64, ptr %1985, , !!8                                                                                    ;L1617<1636<378
 50924|  %1987 = icmp eq i64 %1986, 0                                                                                          ;L378
 50925|  br i1 %1987, label %1988, label %1992                                                                                 ;L378
 50926| 
 50927| 1988: ; preds = %1984
 50928|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %95, i64 5384, i1 false)                                                 ;L379
 50929|  %1989 = load i64, ptr %49, , !!8                                                                                      ;L379
 50930|  %1990 = gep %0, i64 5392                                                                                              ;L379
 50931|  call void @llvm.memcpy.p0.p0.i64(ptr %1990, ptr %1475, i64 184, i1 false)                                             ;L379
 50932|  %1991 = gep %0, i64 5384                                                                                              ;L379
 50933|  store i64 %1989, ptr %1991,                                                                                           ;L379
 50934|  br label %2000                                                                                                        ;L1
 50935| 
 50936| 1992: ; preds = %1984
 50938|     ;; self = ptr %28
 50939|     ;; self = ptr %28
 50940|     ;; len = i64 %1986
 50941|     ;; count = i64 %1986
 50944|     ;; self[8..+8] = i64 %1986
 50945|     ;; slice[8..+8] = i64 %1986
 50957|     ;; self = ptr undef
 50958|     ;; self = ptr undef
 50959|     ;; count = i64 1
 50963|  %1993 = shl nuw nsw i64 %1986, 3                                                                                      ;L961<100<1042<382
 50964|  %1994 = load ptr, ptr %28, , !!8, !!8                                                                                 ;L138<2073<382
 50965|     ;; self = ptr %1994
 50966|     ;; ptr = ptr %1994
 50967|     ;; self[0..+8] = ptr %1994
 50968|     ;; slice[0..+8] = ptr %1994
 50969|     ;; end_or_len = !DIArgList(ptr %1994, i64 %1993)
 50970|     ;; self[8..+8] = !DIArgList(ptr %1994, i64 %1993)
 50971|     ;; self[8..+8] = !DIArgList(ptr %1994, i64 %1993)
 50972|     ;; self[8..+8] = !DIArgList(ptr %1994, i64 %1993)
 50973|     ;; self[0..+8] = ptr %1994
 50974|     ;; self[0..+8] = ptr %1994
 50975|     ;; self[0..+8] = ptr %1994
 50976|     ;; ptr = ptr %1994
 50977|     ;; self = ptr %1994
 50978|  %1995 = gep %1994, i64 %1993                                                                                          ;L961<100<1042<382
 50979|     ;; self[8..+8] = ptr %1995
 50980|     ;; self[8..+8] = ptr %1995
 50981|     ;; self[8..+8] = ptr %1995
 50982|     ;; end_or_len = ptr %1995
 50983|  %1996 = gep %1994, i64 8                                                                                              ;L656<185<107<2706<3354<3325<382
 50984|     ;; self[0..+8] = ptr %1996
 50985|     ;; self = ptr %1994
 50986|     ;; f = ptr undef
 50987|     ;; self = ptr undef
 50988|     ;; x = ptr %1994
 50989|     ;; args = ptr %1994
 50991|     ;; x = ptr %1994
 50994|  %1997 = load ptr, ptr %1994, , !!54962, !!8, !!8                                                                      ;L382<3317<310<1162<107<2706<3354<3325<382
 50995|  %1998 = load i64, ptr %1997, , !!54965, !!8                                                                           ;L382<3317<310<1162<107<2706<3354<3325<382
 50996|     ;; first[0..+8] = i64 %1998
 50997|     ;; first[8..+8] = ptr %1994
 50998|  %1999 = invoke { i64, ptr } @core::iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterRTxNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEENCINvNvNtNtNtBa_6traits8iterator8Iterator10max_by_key3keyRB1n_xNCNvMNtNtNtB1u_11plan_legacy7handler7auctionNtB3v_17LegacyPlanHandler16get_small_actionsm_0E0EB2u_4foldTxB3i_ENCINvNvB2u_6max_by4foldB53_INvB2s_7compareB3i_xEE0EB1u_(ptr %1996, ptr %1995, i64 %1998, ptr %1994)
 50999|  to label %2009 unwind label %2005                                                                                     ;L2707<3354<3325<382
 51000| 
 51001| 2000: ; preds = %2185, %1988
 51003|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB13_(ptr %28)
 51004|  to label %2004 unwind label %2001                                                                                     ;L825<391
 51005| 
 51006| 2001: ; preds = %2000
 51007|  %2002 = cleanuppad within none []
 51009|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB1a_(ptr %28) [ "funclet"(token %2002) ]
 51010|  to label %2003 unwind label %1486                                                                                     ;L825<825<391
 51011| 
 51012| 2003: ; preds = %2001
 51013|  cleanupret from %2002 unwind label %1486
 51014| 
 51015| 2004: ; preds = %2000
 51017|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB1a_(ptr %28)
 51018|  to label %2186 unwind label %1486                                                                                     ;L825<825<391
 51019| 
 51020| 2005: ; preds = %2203, %2202, %2200, %2183, %2182, %2180, %2085, %2020, %2012, %1992
 51021|  %2006 = phi i1 [ true, %2203 ], [ true, %2085 ], [ false, %2183 ], [ true, %2012 ], [ true, %2020 ], [ true, %1992 ], [ false, %2182 ], [ false, %2180 ], [ true, %2202 ], [ true, %2200 ] ;L0
 51022|  %2007 = phi i1 [ false, %2203 ], [ true, %2085 ], [ false, %2183 ], [ true, %2012 ], [ true, %2020 ], [ true, %1992 ], [ false, %2182 ], [ false, %2180 ], [ false, %2202 ], [ false, %2200 ] ;L0
 51023|  %2008 = cleanuppad within none []
 51024|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecRTxNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEEB1w_(ptr %28) #31 [ "funclet"(token %2008) ] ;L391
 51025|  cleanupret from %2008 unwind label %1486                                                                              ;L391
 51026| 
 51027| 2009: ; preds = %1992
 51028|  %2010 = extractvalue { i64, ptr } %1999, 1                                                                            ;L2707<3354<3325<382
 51029|     ;; self = ptr %2010
 51030|  %2011 = icmp eq ptr %2010, null                                                                                       ;L1011<382
 51031|  br i1 %2011, label %2020, label %2012                                                                                 ;L1011<382
 51032| 
 51033| 2012: ; preds = %2009
 51034|  %2013 = load ptr, ptr %2010, , !!8, !!8                                                                               ;L382
 51035|  %2014 = load i64, ptr %2013, , !!8                                                                                    ;L382
 51036|  store i64 %2014, ptr %26,                                                                                             ;L382
 51039|     ;; self = ptr %28
 51040|     ;; self = ptr %28
 51041|  %2015 = load ptr, ptr %28, , !!8, !!8                                                                                 ;L138<2073<384
 51042|     ;; p = ptr %2015
 51043|  %2016 = load i64, ptr %1985, , !!8                                                                                    ;L2075<384
 51044|     ;; len = i64 %2016
 51045|     ;; count = i64 %2016
 51046|     ;; self[0..+8] = ptr %2015
 51047|     ;; slice[0..+8] = ptr %2015
 51048|     ;; self[8..+8] = i64 %2016
 51049|     ;; slice[8..+8] = i64 %2016
 51050|     ;; ptr = ptr %2015
 51051|     ;; self = ptr %2015
 51052|  %2017 = getelementptr ptr, ptr %2015, i64 %2016                                                                       ;L961<100<1042<384
 51053|     ;; self[0..+8] = ptr %2015
 51054|     ;; self[8..+8] = ptr %2017
 51055|     ;; self[16..+8] = ptr %26
 51056|  store ptr %2015, ptr %24,                                                                                             ;L69<836<384
 51057|  %2018 = gep %24, i64 8                                                                                                ;L69<836<384
 51058|  store ptr %2017, ptr %2018,                                                                                           ;L69<836<384
 51059|  %2019 = gep %24, i64 16                                                                                               ;L69<836<384
 51060|  store ptr %26, ptr %2019,                                                                                             ;L69<836<384
 51061|  invoke void @core::iter8adapters3map3MapINtNtB29_6filter6FilterINtNtNtB2d_5slice4iter4IterRTxBU_EENCNvMNtNtNtBY_11plan_legacy7handler7auctionNtB3V_17LegacyPlanHandler16get_small_actionsn_0ENCB3Q_so_0EEBY_(ptr sret([32 x i8]) %25, ptr %24, ptr %655)
 51062|  to label %2021 unwind label %2005                                                                                     ;L383
 51063| 
 51064| 2020: ; preds = %2009
 51065|  invoke void @core::option13unwrap_failed(ptr @anon.282069a2ed2ad3a275929b639963fb55.152) #32
 51066|  to label %203 unwind label %2005                                                                                      ;L1013<382
 51067| 
 51068| 2021: ; preds = %2012
 51070|  %2022 = load i64, ptr %26, , !!8                                                                                      ;L385
 51071|  %2023 = icmp sgt i64 %2022, -1                                                                                        ;L385
 51072|  br i1 %2023, label %2080, label %2024                                                                                 ;L385
 51073| 
 51074| 2024: ; preds = %2021
 51075|     ;; self = ptr %1475
 51076|  %2025 = load i8, ptr %1476, , !!55022, !!8                                                                            ;L309<385
 51077|  %2026 = icmp ne i8 %2025, 10                                                                                          ;L309<385
 51078|  call void @llvm.assume(i1 %2026)                                                                                      ;L309<385
 51079|  %2027 = add nsw i8 %2025, -3                                                                                          ;L309<385
 51080|  %2028 = icmp samesign ugt i8 %2025, 2                                                                                 ;L309<385
 51081|  %2029 = select i1 %2028, i8 %2027, i8 7                                                                               ;L309<385
 51082|  switch i8 %2029, label %2030 [
 51083|  i8 0, label %2087
 51084|  i8 1, label %2087
 51085|  i8 2, label %2031
 51086|  i8 3, label %2034
 51087|  i8 4, label %2037
 51088|  i8 5, label %2087
 51089|  i8 6, label %2042
 51090|  i8 7, label %2047
 51091|  i8 8, label %2052
 51092|  i8 9, label %2057
 51093|  i8 10, label %2062
 51094|  i8 11, label %2065
 51095|  i8 12, label %2067
 51096|  i8 13, label %2070
 51097|  i8 14, label %2073
 51098|  i8 15, label %2076
 51099|  i8 16, label %2079
 51100|  ]                                                                                                                     ;L309<385
 51101| 
 51102| 2030: ; preds = %2024
 51103|  unreachable                                                                                                           ;L309<385
 51104| 
 51105| 2031: ; preds = %2024
 51106|     ;; action = ptr %1475
 51107|     ;; self = ptr %1475
 51108|  %2032 = gep %49, i64 16                                                                                               ;L285<316<385
 51109|  %2033 = load i64, ptr %2032, , !!55022, !!8                                                                           ;L285<316<385
 51110|  br label %2087                                                                                                        ;L316<385
 51111| 
 51112| 2034: ; preds = %2024
 51113|     ;; action = ptr %1475
 51114|     ;; self = ptr %1475
 51115|  %2035 = gep %49, i64 16                                                                                               ;L491<313<385
 51116|  %2036 = load i64, ptr %2035, , !!55022, !!8                                                                           ;L491<313<385
 51117|  br label %2087                                                                                                        ;L313<385
 51118| 
 51119| 2037: ; preds = %2024
 51120|     ;; action = ptr %1475
 51121|     ;; self = ptr %1475
 51122|  %2038 = gep %49, i64 24                                                                                               ;L665<314<385
 51123|  %2039 = load i64, ptr %2038, , !!55022, !!8                                                                           ;L665<314<385
 51124|  %2040 = gep %49, i64 32                                                                                               ;L665<314<385
 51125|  %2041 = load i64, ptr %2040, , !!55022, !!8                                                                           ;L665<314<385
 51126|  br label %2087                                                                                                        ;L314<385
 51127| 
 51128| 2042: ; preds = %2024
 51129|     ;; action = ptr %1475
 51130|     ;; self = ptr %1475
 51131|  %2043 = gep %49, i64 16                                                                                               ;L800<312<385
 51132|  %2044 = load i64, ptr %2043, , !!55022, !!8                                                                           ;L800<312<385
 51133|  %2045 = gep %49, i64 24                                                                                               ;L800<312<385
 51134|  %2046 = load i64, ptr %2045, , !!55022, !!8                                                                           ;L800<312<385
 51135|  br label %2087                                                                                                        ;L312<385
 51136| 
 51137| 2047: ; preds = %2024
 51138|     ;; action = ptr %1475
 51139|     ;; self = ptr %1475
 51140|  %2048 = gep %49, i64 56                                                                                               ;L1035<317<385
 51141|  %2049 = load i64, ptr %2048, , !!55022, !!8                                                                           ;L1035<317<385
 51142|  %2050 = gep %49, i64 64                                                                                               ;L1035<317<385
 51143|  %2051 = load i64, ptr %2050, , !!55022, !!8                                                                           ;L1035<317<385
 51144|  br label %2087                                                                                                        ;L317<385
 51145| 
 51146| 2052: ; preds = %2024
 51147|     ;; action = ptr %1475
 51148|     ;; self = ptr %1475
 51149|  %2053 = gep %49, i64 16                                                                                               ;L1123<318<385
 51150|  %2054 = load i64, ptr %2053, , !!55022, !!8                                                                           ;L1123<318<385
 51151|  %2055 = gep %49, i64 24                                                                                               ;L1123<318<385
 51152|  %2056 = load i64, ptr %2055, , !!55022, !!8                                                                           ;L1123<318<385
 51153|  br label %2087                                                                                                        ;L318<385
 51154| 
 51155| 2057: ; preds = %2024
 51156|     ;; action = ptr %1475
 51157|     ;; self = ptr %1475
 51158|  %2058 = gep %49, i64 32                                                                                               ;L1241<319<385
 51159|  %2059 = load i64, ptr %2058, , !!55022, !!8                                                                           ;L1241<319<385
 51160|  %2060 = gep %49, i64 40                                                                                               ;L1241<319<385
 51161|  %2061 = load i64, ptr %2060, , !!55022, !!8                                                                           ;L1241<319<385
 51162|  br label %2087                                                                                                        ;L319<385
 51163| 
 51164| 2062: ; preds = %2024
 51165|     ;; action = ptr %1475
 51166|     ;; self = ptr %1475
 51167|  %2063 = gep %49, i64 16                                                                                               ;L653<320<385
 51168|  %2064 = load i64, ptr %2063, , !!55022, !!8                                                                           ;L653<320<385
 51169|  br label %2087                                                                                                        ;L320<385
 51170| 
 51171| 2065: ; preds = %2024
 51172|     ;; action = ptr %1475
 51173|     ;; self = ptr %1475
 51174|  %2066 = load i64, ptr %1937, , !!55022, !!8                                                                           ;L404<321<385
 51175|  br label %2087                                                                                                        ;L321<385
 51176| 
 51177| 2067: ; preds = %2024
 51178|     ;; action = ptr %1475
 51179|     ;; self = ptr %1475
 51180|  %2068 = gep %49, i64 16                                                                                               ;L94<322<385
 51181|  %2069 = load i64, ptr %2068, , !!55022, !!8                                                                           ;L94<322<385
 51182|  br label %2087                                                                                                        ;L322<385
 51183| 
 51184| 2070: ; preds = %2024
 51185|     ;; action = ptr %1475
 51186|     ;; self = ptr %1475
 51187|  %2071 = gep %49, i64 16                                                                                               ;L160<323<385
 51188|  %2072 = load i64, ptr %2071, , !!55022, !!8                                                                           ;L160<323<385
 51189|  br label %2087                                                                                                        ;L323<385
 51190| 
 51191| 2073: ; preds = %2024
 51192|     ;; action = ptr %1475
 51193|     ;; self = ptr %1475
 51194|  %2074 = gep %49, i64 16                                                                                               ;L222<324<385
 51195|  %2075 = load i64, ptr %2074, , !!55022, !!8                                                                           ;L222<324<385
 51196|  br label %2087                                                                                                        ;L324<385
 51197| 
 51198| 2076: ; preds = %2024
 51199|     ;; action = ptr %1475
 51200|     ;; self = ptr %1475
 51201|  %2077 = gep %49, i64 16                                                                                               ;L287<325<385
 51202|  %2078 = load i64, ptr %2077, , !!55022, !!8                                                                           ;L287<325<385
 51203|  br label %2087                                                                                                        ;L325<385
 51204| 
 51205| 2079: ; preds = %2024
 51206|  br label %2087                                                                                                        ;L326<385
 51207| 
 51208| 2080: ; preds = %2184, %2021
 51210|     ;; self = ptr %25
 51211|     ;; self = ptr %25
 51212|  %2081 = load ptr, ptr %25, , !!8, !!8                                                                                 ;L138<2073<386
 51213|     ;; p = ptr %2081
 51214|  %2082 = gep %25, i64 24                                                                                               ;L2075<386
 51215|  %2083 = load i64, ptr %2082, , !!8                                                                                    ;L2075<386
 51216|  %2084 = invoke ptr @_RINvXNtCsMBkRBYhlca_4rand3seqSNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayNtB3_11SliceRandom6chooseNtNtNtB5_4rngs3std6StdRngEBw_(ptr %2081, i64 %2083, ptr %3)
 51217|  to label %2192 unwind label %2085                                                                                     ;L386
 51218| 
 51219| 2085: ; preds = %2195, %2194, %2174, %2080
 51220|  %2086 = cleanuppad within none []
 51221|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %25) #31 [ "funclet"(token %2086) ] ;L391
 51222|  cleanupret from %2086 unwind label %2005                                                                              ;L391
 51223| 
 51224| 2087: ; preds = %2079, %2076, %2073, %2070, %2067, %2065, %2062, %2057, %2052, %2047, %2042, %2037, %2034, %2031, %2024, %2024, %2024
 51225|  %2088 = phi i64 [ %2033, %2031 ], [ %2036, %2034 ], [ %2039, %2037 ], [ undef, %2024 ], [ undef, %2024 ], [ undef, %2024 ], [ %2044, %2042 ], [ %2049, %2047 ], [ %2054, %2052 ], [ %2059, %2057 ], [ %2064, %2062 ], [ %2066, %2065 ], [ %2069, %2067 ], [ %2072, %2070 ], [ %2075, %2073 ], [ %2078, %2076 ], [ undef, %2079 ]
 51226|  %2089 = phi i64 [ undef, %2031 ], [ undef, %2034 ], [ %2041, %2037 ], [ undef, %2024 ], [ undef, %2024 ], [ undef, %2024 ], [ %2046, %2042 ], [ %2051, %2047 ], [ %2056, %2052 ], [ %2061, %2057 ], [ undef, %2062 ], [ undef, %2065 ], [ undef, %2067 ], [ undef, %2070 ], [ undef, %2073 ], [ undef, %2076 ], [ undef, %2079 ]
 51227|  %2090 = phi i64 [ 2, %2031 ], [ 2, %2034 ], [ 3, %2037 ], [ 0, %2024 ], [ 0, %2024 ], [ 0, %2024 ], [ 1, %2042 ], [ 3, %2047 ], [ 3, %2052 ], [ 3, %2057 ], [ 2, %2062 ], [ 4, %2065 ], [ 6, %2067 ], [ 7, %2070 ], [ 8, %2073 ], [ 9, %2076 ], [ 10, %2079 ]
 51228|     ;; self = ptr %6
 51229|  %2091 = gep %6, i64 177                                                                                               ;L309<385
 51230|  %2092 = load i8, ptr %2091, , !!55105, !!8                                                                            ;L309<385
 51231|  %2093 = icmp ne i8 %2092, 10                                                                                          ;L309<385
 51232|  call void @llvm.assume(i1 %2093)                                                                                      ;L309<385
 51233|  %2094 = add nsw i8 %2092, -3                                                                                          ;L309<385
 51234|  %2095 = icmp samesign ugt i8 %2092, 2                                                                                 ;L309<385
 51235|  %2096 = select i1 %2095, i8 %2094, i8 7                                                                               ;L309<385
 51236|  switch i8 %2096, label %2097 [
 51237|  i8 0, label %2148
 51238|  i8 1, label %2148
 51239|  i8 2, label %2098
 51240|  i8 3, label %2101
 51241|  i8 4, label %2104
 51242|  i8 5, label %2148
 51243|  i8 6, label %2109
 51244|  i8 7, label %2114
 51245|  i8 8, label %2119
 51246|  i8 9, label %2124
 51247|  i8 10, label %2129
 51248|  i8 11, label %2132
 51249|  i8 12, label %2135
 51250|  i8 13, label %2138
 51251|  i8 14, label %2141
 51252|  i8 15, label %2144
 51253|  i8 16, label %2147
 51254|  ]                                                                                                                     ;L309<385
 51255| 
 51256| 2097: ; preds = %2087
 51257|  unreachable                                                                                                           ;L309<385
 51258| 
 51259| 2098: ; preds = %2087
 51260|     ;; action = ptr %6
 51261|     ;; self = ptr %6
 51262|  %2099 = gep %6, i64 8                                                                                                 ;L285<316<385
 51263|  %2100 = load i64, ptr %2099, , !!55105, !!8                                                                           ;L285<316<385
 51264|  br label %2148                                                                                                        ;L316<385
 51265| 
 51266| 2101: ; preds = %2087
 51267|     ;; action = ptr %6
 51268|     ;; self = ptr %6
 51269|  %2102 = gep %6, i64 8                                                                                                 ;L491<313<385
 51270|  %2103 = load i64, ptr %2102, , !!55105, !!8                                                                           ;L491<313<385
 51271|  br label %2148                                                                                                        ;L313<385
 51272| 
 51273| 2104: ; preds = %2087
 51274|     ;; action = ptr %6
 51275|     ;; self = ptr %6
 51276|  %2105 = gep %6, i64 16                                                                                                ;L665<314<385
 51277|  %2106 = load i64, ptr %2105, , !!55105, !!8                                                                           ;L665<314<385
 51278|  %2107 = gep %6, i64 24                                                                                                ;L665<314<385
 51279|  %2108 = load i64, ptr %2107, , !!55105, !!8                                                                           ;L665<314<385
 51280|  br label %2148                                                                                                        ;L314<385
 51281| 
 51282| 2109: ; preds = %2087
 51283|     ;; action = ptr %6
 51284|     ;; self = ptr %6
 51285|  %2110 = gep %6, i64 8                                                                                                 ;L800<312<385
 51286|  %2111 = load i64, ptr %2110, , !!55105, !!8                                                                           ;L800<312<385
 51287|  %2112 = gep %6, i64 16                                                                                                ;L800<312<385
 51288|  %2113 = load i64, ptr %2112, , !!55105, !!8                                                                           ;L800<312<385
 51289|  br label %2148                                                                                                        ;L312<385
 51290| 
 51291| 2114: ; preds = %2087
 51292|     ;; action = ptr %6
 51293|     ;; self = ptr %6
 51294|  %2115 = gep %6, i64 48                                                                                                ;L1035<317<385
 51295|  %2116 = load i64, ptr %2115, , !!55105, !!8                                                                           ;L1035<317<385
 51296|  %2117 = gep %6, i64 56                                                                                                ;L1035<317<385
 51297|  %2118 = load i64, ptr %2117, , !!55105, !!8                                                                           ;L1035<317<385
 51298|  br label %2148                                                                                                        ;L317<385
 51299| 
 51300| 2119: ; preds = %2087
 51301|     ;; action = ptr %6
 51302|     ;; self = ptr %6
 51303|  %2120 = gep %6, i64 8                                                                                                 ;L1123<318<385
 51304|  %2121 = load i64, ptr %2120, , !!55105, !!8                                                                           ;L1123<318<385
 51305|  %2122 = gep %6, i64 16                                                                                                ;L1123<318<385
 51306|  %2123 = load i64, ptr %2122, , !!55105, !!8                                                                           ;L1123<318<385
 51307|  br label %2148                                                                                                        ;L318<385
 51308| 
 51309| 2124: ; preds = %2087
 51310|     ;; action = ptr %6
 51311|     ;; self = ptr %6
 51312|  %2125 = gep %6, i64 24                                                                                                ;L1241<319<385
 51313|  %2126 = load i64, ptr %2125, , !!55105, !!8                                                                           ;L1241<319<385
 51314|  %2127 = gep %6, i64 32                                                                                                ;L1241<319<385
 51315|  %2128 = load i64, ptr %2127, , !!55105, !!8                                                                           ;L1241<319<385
 51316|  br label %2148                                                                                                        ;L319<385
 51317| 
 51318| 2129: ; preds = %2087
 51319|     ;; action = ptr %6
 51320|     ;; self = ptr %6
 51321|  %2130 = gep %6, i64 8                                                                                                 ;L653<320<385
 51322|  %2131 = load i64, ptr %2130, , !!55105, !!8                                                                           ;L653<320<385
 51323|  br label %2148                                                                                                        ;L320<385
 51324| 
 51325| 2132: ; preds = %2087
 51326|     ;; action = ptr %6
 51327|     ;; self = ptr %6
 51328|  %2133 = gep %6, i64 96                                                                                                ;L404<321<385
 51329|  %2134 = load i64, ptr %2133, , !!55105, !!8                                                                           ;L404<321<385
 51330|  br label %2148                                                                                                        ;L321<385
 51331| 
 51332| 2135: ; preds = %2087
 51333|     ;; action = ptr %6
 51334|     ;; self = ptr %6
 51335|  %2136 = gep %6, i64 8                                                                                                 ;L94<322<385
 51336|  %2137 = load i64, ptr %2136, , !!55105, !!8                                                                           ;L94<322<385
 51337|  br label %2148                                                                                                        ;L322<385
 51338| 
 51339| 2138: ; preds = %2087
 51340|     ;; action = ptr %6
 51341|     ;; self = ptr %6
 51342|  %2139 = gep %6, i64 8                                                                                                 ;L160<323<385
 51343|  %2140 = load i64, ptr %2139, , !!55105, !!8                                                                           ;L160<323<385
 51344|  br label %2148                                                                                                        ;L323<385
 51345| 
 51346| 2141: ; preds = %2087
 51347|     ;; action = ptr %6
 51348|     ;; self = ptr %6
 51349|  %2142 = gep %6, i64 8                                                                                                 ;L222<324<385
 51350|  %2143 = load i64, ptr %2142, , !!55105, !!8                                                                           ;L222<324<385
 51351|  br label %2148                                                                                                        ;L324<385
 51352| 
 51353| 2144: ; preds = %2087
 51354|     ;; action = ptr %6
 51355|     ;; self = ptr %6
 51356|  %2145 = gep %6, i64 8                                                                                                 ;L287<325<385
 51357|  %2146 = load i64, ptr %2145, , !!55105, !!8                                                                           ;L287<325<385
 51358|  br label %2148                                                                                                        ;L325<385
 51359| 
 51360| 2147: ; preds = %2087
 51361|  br label %2148                                                                                                        ;L326<385
 51362| 
 51363| 2148: ; preds = %2147, %2144, %2141, %2138, %2135, %2132, %2129, %2124, %2119, %2114, %2109, %2104, %2101, %2098, %2087, %2087, %2087
 51364|  %2149 = phi i64 [ %2100, %2098 ], [ %2103, %2101 ], [ %2106, %2104 ], [ undef, %2087 ], [ undef, %2087 ], [ undef, %2087 ], [ %2111, %2109 ], [ %2116, %2114 ], [ %2121, %2119 ], [ %2126, %2124 ], [ %2131, %2129 ], [ %2134, %2132 ], [ %2137, %2135 ], [ %2140, %2138 ], [ %2143, %2141 ], [ %2146, %2144 ], [ undef, %2147 ]
 51365|  %2150 = phi i64 [ undef, %2098 ], [ undef, %2101 ], [ %2108, %2104 ], [ undef, %2087 ], [ undef, %2087 ], [ undef, %2087 ], [ %2113, %2109 ], [ %2118, %2114 ], [ %2123, %2119 ], [ %2128, %2124 ], [ undef, %2129 ], [ undef, %2132 ], [ undef, %2135 ], [ undef, %2138 ], [ undef, %2141 ], [ undef, %2144 ], [ undef, %2147 ]
 51366|  %2151 = phi i64 [ 2, %2098 ], [ 2, %2101 ], [ 3, %2104 ], [ 0, %2087 ], [ 0, %2087 ], [ 0, %2087 ], [ 1, %2109 ], [ 3, %2114 ], [ 3, %2119 ], [ 3, %2124 ], [ 2, %2129 ], [ 4, %2132 ], [ 6, %2135 ], [ 7, %2138 ], [ 8, %2141 ], [ 9, %2144 ], [ 10, %2147 ]
 51367|     ;; self = ptr undef
 51368|     ;; other = ptr undef
 51369|     ;; __self_discr = i64 %2090
 51370|     ;; __arg1_discr = i64 %2151
 51371|  %2152 = icmp eq i64 %2090, %2151                                                                                      ;L81<385
 51372|  br i1 %2152, label %2153, label %2176                                                                                 ;L81<385
 51373| 
 51374| 2153: ; preds = %2148
 51375|  switch i64 %2090, label %2174 [
 51376|  i64 1, label %2154
 51377|  i64 2, label %2158
 51378|  i64 3, label %2160
 51379|  i64 4, label %2164
 51380|  i64 6, label %2166
 51381|  i64 7, label %2168
 51382|  i64 8, label %2170
 51383|  i64 9, label %2172
 51384|  ]                                                                                                                     ;L81<385
 51385| 
 51386| 2154: ; preds = %2153
 51387|     ;; __self_0 = ptr undef
 51388|     ;; self = ptr undef
 51389|     ;; __self_1 = ptr undef
 51390|     ;; self = ptr undef
 51391|     ;; __arg1_0 = ptr undef
 51392|     ;; other = ptr undef
 51393|     ;; __arg1_1 = ptr undef
 51394|     ;; other = ptr undef
 51397|  %2155 = icmp eq i64 %2088, %2149                                                                                      ;L1878<2123<84<385
 51398|  %2156 = icmp eq i64 %2089, %2150
 51399|  %2157 = and i1 %2155, %2156                                                                                           ;L84<385
 51400|  br i1 %2157, label %2174, label %2176                                                                                 ;L84<385
 51401| 
 51402| 2158: ; preds = %2153
 51403|     ;; __self_0 = ptr undef
 51404|     ;; self = ptr undef
 51405|     ;; __arg1_0 = ptr undef
 51406|     ;; other = ptr undef
 51409|  %2159 = icmp eq i64 %2088, %2149                                                                                      ;L1878<2123<85<385
 51410|  br i1 %2159, label %2174, label %2176                                                                                 ;L385
 51411| 
 51412| 2160: ; preds = %2153
 51413|     ;; __self_0 = ptr undef
 51414|     ;; self = ptr undef
 51415|     ;; __self_1 = ptr undef
 51416|     ;; self = ptr undef
 51417|     ;; __arg1_0 = ptr undef
 51418|     ;; other = ptr undef
 51419|     ;; __arg1_1 = ptr undef
 51420|     ;; other = ptr undef
 51423|  %2161 = icmp eq i64 %2088, %2149                                                                                      ;L1878<2123<86<385
 51424|  %2162 = icmp eq i64 %2089, %2150
 51425|  %2163 = and i1 %2161, %2162                                                                                           ;L86<385
 51426|  br i1 %2163, label %2174, label %2176                                                                                 ;L86<385
 51427| 
 51428| 2164: ; preds = %2153
 51429|     ;; __self_0 = ptr undef
 51430|     ;; self = ptr undef
 51431|     ;; __arg1_0 = ptr undef
 51432|     ;; other = ptr undef
 51435|  %2165 = icmp eq i64 %2088, %2149                                                                                      ;L1878<2123<87<385
 51436|  br i1 %2165, label %2174, label %2176                                                                                 ;L385
 51437| 
 51438| 2166: ; preds = %2153
 51439|     ;; __self_0 = ptr undef
 51440|     ;; self = ptr undef
 51441|     ;; __arg1_0 = ptr undef
 51442|     ;; other = ptr undef
 51445|  %2167 = icmp eq i64 %2088, %2149                                                                                      ;L1878<2123<89<385
 51446|  br i1 %2167, label %2174, label %2176                                                                                 ;L385
 51447| 
 51448| 2168: ; preds = %2153
 51449|     ;; __self_0 = ptr undef
 51450|     ;; self = ptr undef
 51451|     ;; __arg1_0 = ptr undef
 51452|     ;; other = ptr undef
 51455|  %2169 = icmp eq i64 %2088, %2149                                                                                      ;L1878<2123<90<385
 51456|  br i1 %2169, label %2174, label %2176                                                                                 ;L385
 51457| 
 51458| 2170: ; preds = %2153
 51459|     ;; __self_0 = ptr undef
 51460|     ;; self = ptr undef
 51461|     ;; __arg1_0 = ptr undef
 51462|     ;; other = ptr undef
 51465|  %2171 = icmp eq i64 %2088, %2149                                                                                      ;L1878<2123<91<385
 51466|  br i1 %2171, label %2174, label %2176                                                                                 ;L385
 51467| 
 51468| 2172: ; preds = %2153
 51469|     ;; __self_0 = ptr undef
 51470|     ;; self = ptr undef
 51471|     ;; __arg1_0 = ptr undef
 51472|     ;; other = ptr undef
 51475|  %2173 = icmp eq i64 %2088, %2149                                                                                      ;L1878<2123<92<385
 51476|  br i1 %2173, label %2174, label %2176                                                                                 ;L385
 51477| 
 51478| 2174: ; preds = %2172, %2170, %2168, %2166, %2164, %2160, %2158, %2154, %2153
 51479|  %2175 = invoke zeroext i1 @ai::small_actionNtB2_15SmallActionPlay18move_near_complete(ptr %6, ptr %200)
 51480|  to label %2184 unwind label %2085                                                                                     ;L385
 51481| 
 51482| 2176: ; preds = %2184, %2172, %2170, %2168, %2166, %2164, %2160, %2158, %2154, %2148
 51483|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %95, i64 5384, i1 false)                                                 ;L389
 51484|  %2177 = load i64, ptr %49, , !!8                                                                                      ;L389
 51485|  %2178 = gep %0, i64 5392                                                                                              ;L389
 51486|  call void @llvm.memcpy.p0.p0.i64(ptr %2178, ptr %1475, i64 184, i1 false)                                             ;L389
 51487|  %2179 = gep %0, i64 5384                                                                                              ;L389
 51488|  store i64 %2177, ptr %2179,                                                                                           ;L389
 51490|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %25)
 51491|  to label %2183 unwind label %2180                                                                                     ;L825<391
 51492| 
 51493| 2180: ; preds = %2176
 51494|  %2181 = cleanuppad within none []
 51496|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %25) [ "funclet"(token %2181) ]
 51497|  to label %2182 unwind label %2005                                                                                     ;L825<825<391
 51498| 
 51499| 2182: ; preds = %2180
 51500|  cleanupret from %2181 unwind label %2005
 51501| 
 51502| 2183: ; preds = %2176
 51504|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %25)
 51505|  to label %2185 unwind label %2005                                                                                     ;L825<825<391
 51506| 
 51507| 2184: ; preds = %2174
 51508|  br i1 %2175, label %2080, label %2176                                                                                 ;L385
 51509| 
 51510| 2185: ; preds = %2183
 51513|  br label %2000                                                                                                        ;L1
 51514| 
 51515| 2186: ; preds = %2004
 51517|  br label %1983                                                                                                        ;L1
 51518| 
 51519| 2187: ; preds = %2418, %1983
 51522|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %51)
 51523|  to label %2191 unwind label %2188                                                                                     ;L825<416
 51524| 
 51525| 2188: ; preds = %2187
 51526|  %2189 = cleanuppad within none []
 51528|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %51) [ "funclet"(token %2189) ]
 51529|  to label %2190 unwind label %1381                                                                                     ;L825<825<416
 51530| 
 51531| 2190: ; preds = %2188
 51532|  cleanupret from %2189 unwind label %1381
 51533| 
 51534| 2191: ; preds = %2187
 51536|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %51)
 51537|  to label %2419 unwind label %1381                                                                                     ;L825<825<416
 51538| 
 51539| 2192: ; preds = %2080
 51540|     ;; self = ptr %2084
 51541|  %2193 = icmp eq ptr %2084, null                                                                                       ;L1011<386
 51542|  br i1 %2193, label %2195, label %2194                                                                                 ;L1011<386
 51543| 
 51544| 2194: ; preds = %2192
 51545|  invoke fastcc void @ai::small_actionNtB4_15SmallActionPlayNtNtCsjihNppCmMEE_4core5clone5Clone5clone(ptr %23, ptr %2084)
 51546|  to label %2196 unwind label %2085                                                                                     ;L386
 51547| 
 51548| 2195: ; preds = %2192
 51549|  invoke void @core::option13unwrap_failed(ptr @anon.282069a2ed2ad3a275929b639963fb55.153) #32
 51550|  to label %203 unwind label %2085                                                                                      ;L1013<386
 51551| 
 51552| 2196: ; preds = %2194
 51553|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %95, i64 5384, i1 false)                                                 ;L387
 51554|  %2197 = load i64, ptr %26, , !!8                                                                                      ;L387
 51555|  %2198 = gep %0, i64 5384                                                                                              ;L387
 51556|  store i64 %2197, ptr %2198,                                                                                           ;L387
 51557|  %2199 = gep %0, i64 5392                                                                                              ;L387
 51558|  call void @llvm.memcpy.p0.p0.i64(ptr %2199, ptr %23, i64 184, i1 false)                                               ;L387
 51561|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %25)
 51562|  to label %2203 unwind label %2200                                                                                     ;L825<391
 51563| 
 51564| 2200: ; preds = %2196
 51565|  %2201 = cleanuppad within none []
 51567|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %25) [ "funclet"(token %2201) ]
 51568|  to label %2202 unwind label %2005                                                                                     ;L825<825<391
 51569| 
 51570| 2202: ; preds = %2200
 51571|  cleanupret from %2201 unwind label %2005
 51572| 
 51573| 2203: ; preds = %2196
 51575|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %25)
 51576|  to label %2204 unwind label %2005                                                                                     ;L825<825<391
 51577| 
 51578| 2204: ; preds = %2203
 51582|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB13_(ptr %28)
 51583|  to label %2208 unwind label %2205                                                                                     ;L825<391
 51584| 
 51585| 2205: ; preds = %2204
 51586|  %2206 = cleanuppad within none []
 51588|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB1a_(ptr %28) [ "funclet"(token %2206) ]
 51589|  to label %2207 unwind label %1486                                                                                     ;L825<825<391
 51590| 
 51591| 2207: ; preds = %2205
 51592|  cleanupret from %2206 unwind label %1486
 51593| 
 51594| 2208: ; preds = %2204
 51596|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB1a_(ptr %28)
 51597|  to label %2209 unwind label %1486                                                                                     ;L825<825<391
 51598| 
 51599| 2209: ; preds = %2208
 51602|  br label %2210                                                                                                        ;L360
 51603| 
 51604| 2210: ; preds = %2457, %2209
 51605|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %1475)         ;L416
 51606|  br label %2458                                                                                                        ;L416
 51607| 
 51608| 2211: ; preds = %1925, %1925, %1925, %1925, %1925, %1925, %1925, %1925, %1925
 51609|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %95, i64 5384, i1 false)                                                 ;L414
 51610|  %2212 = load i64, ptr %49, , !!8                                                                                      ;L414
 51611|  %2213 = gep %0, i64 5392                                                                                              ;L414
 51612|  call void @llvm.memcpy.p0.p0.i64(ptr %2213, ptr %1475, i64 184, i1 false)                                             ;L414
 51613|  %2214 = gep %0, i64 5384                                                                                              ;L414
 51614|  store i64 %2212, ptr %2214,                                                                                           ;L414
 51615|  br label %2458                                                                                                        ;L416
 51616| 
 51617| 2215: ; preds = %1925, %1925, %1925, %1925, %1925, %1925, %1925
 51619|     ;; self = ptr %58
 51620|     ;; self = ptr %58
 51621|  %2216 = load ptr, ptr %58, , !!8, !!8                                                                                 ;L138<2073<394
 51622|     ;; p = ptr %2216
 51623|  %2217 = load i64, ptr %1371, , !!8                                                                                    ;L2075<394
 51624|     ;; len = i64 %2217
 51625|     ;; count = i64 %2217
 51626|     ;; self[0..+8] = ptr %2216
 51627|     ;; slice[0..+8] = ptr %2216
 51628|     ;; self[8..+8] = i64 %2217
 51629|     ;; slice[8..+8] = i64 %2217
 51630|     ;; ptr = ptr %2216
 51631|     ;; self = ptr %2216
 51632|  %2218 = getelementptr { i64, { [177 x i8], i8, [6 x i8] } }, ptr %2216, i64 %2217                                     ;L961<100<1042<394
 51633|  invoke void @core::iter8adapters6filter6FilterINtNtNtB2h_5slice4iter4IterBV_ENCNvMNtNtNtB11_11plan_legacy7handler7auctionNtB3E_17LegacyPlanHandler16get_small_actionsp_0EEB11_(ptr sret([32 x i8]) %22, ptr %2216, ptr %2218, ptr %655)
 51634|  to label %2219 unwind label %1486                                                                                     ;L393
 51635| 
 51636| 2219: ; preds = %2215
 51637|     ;; self = ptr %22
 51638|     ;; self = ptr %22
 51639|  %2220 = gep %22, i64 24                                                                                               ;L1617<1636<399
 51640|  %2221 = load i64, ptr %2220, , !!8                                                                                    ;L1617<1636<399
 51641|  %2222 = icmp eq i64 %2221, 0                                                                                          ;L399
 51642|  br i1 %2222, label %2223, label %2227                                                                                 ;L399
 51643| 
 51644| 2223: ; preds = %2219
 51645|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %95, i64 5384, i1 false)                                                 ;L400
 51646|  %2224 = load i64, ptr %49, , !!8                                                                                      ;L400
 51647|  %2225 = gep %0, i64 5392                                                                                              ;L400
 51648|  call void @llvm.memcpy.p0.p0.i64(ptr %2225, ptr %1475, i64 184, i1 false)                                             ;L400
 51649|  %2226 = gep %0, i64 5384                                                                                              ;L400
 51650|  store i64 %2224, ptr %2226,                                                                                           ;L400
 51651|  br label %2235                                                                                                        ;L1
 51652| 
 51653| 2227: ; preds = %2219
 51655|     ;; self = ptr %22
 51656|     ;; self = ptr %22
 51657|     ;; len = i64 %2221
 51658|     ;; count = i64 %2221
 51661|     ;; self[8..+8] = i64 %2221
 51662|     ;; slice[8..+8] = i64 %2221
 51674|     ;; self = ptr undef
 51675|     ;; self = ptr undef
 51676|     ;; count = i64 1
 51680|  %2228 = shl nuw nsw i64 %2221, 3                                                                                      ;L961<100<1042<403
 51681|  %2229 = load ptr, ptr %22, , !!8, !!8                                                                                 ;L138<2073<403
 51682|     ;; self = ptr %2229
 51683|     ;; ptr = ptr %2229
 51684|     ;; self[0..+8] = ptr %2229
 51685|     ;; slice[0..+8] = ptr %2229
 51686|     ;; end_or_len = !DIArgList(ptr %2229, i64 %2228)
 51687|     ;; self[8..+8] = !DIArgList(ptr %2229, i64 %2228)
 51688|     ;; self[8..+8] = !DIArgList(ptr %2229, i64 %2228)
 51689|     ;; self[8..+8] = !DIArgList(ptr %2229, i64 %2228)
 51690|     ;; self[0..+8] = ptr %2229
 51691|     ;; self[0..+8] = ptr %2229
 51692|     ;; self[0..+8] = ptr %2229
 51693|     ;; ptr = ptr %2229
 51694|     ;; self = ptr %2229
 51695|  %2230 = gep %2229, i64 %2228                                                                                          ;L961<100<1042<403
 51696|     ;; self[8..+8] = ptr %2230
 51697|     ;; self[8..+8] = ptr %2230
 51698|     ;; self[8..+8] = ptr %2230
 51699|     ;; end_or_len = ptr %2230
 51700|  %2231 = gep %2229, i64 8                                                                                              ;L656<185<107<2706<3354<3325<403
 51701|     ;; self[0..+8] = ptr %2231
 51702|     ;; self = ptr %2229
 51703|     ;; f = ptr undef
 51704|     ;; self = ptr undef
 51705|     ;; x = ptr %2229
 51706|     ;; args = ptr %2229
 51708|     ;; x = ptr %2229
 51711|  %2232 = load ptr, ptr %2229, , !!55458, !!8, !!8                                                                      ;L403<3317<310<1162<107<2706<3354<3325<403
 51712|  %2233 = load i64, ptr %2232, , !!55461, !!8                                                                           ;L403<3317<310<1162<107<2706<3354<3325<403
 51713|     ;; first[0..+8] = i64 %2233
 51714|     ;; first[8..+8] = ptr %2229
 51715|  %2234 = invoke { i64, ptr } @core::iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterRTxNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEENCINvNvNtNtNtBa_6traits8iterator8Iterator10max_by_key3keyRB1n_xNCNvMNtNtNtB1u_11plan_legacy7handler7auctionNtB3v_17LegacyPlanHandler16get_small_actionsq_0E0EB2u_4foldTxB3i_ENCINvNvB2u_6max_by4foldB53_INvB2s_7compareB3i_xEE0EB1u_(ptr %2231, ptr %2230, i64 %2233, ptr %2229)
 51716|  to label %2244 unwind label %2240                                                                                     ;L2707<3354<3325<403
 51717| 
 51718| 2235: ; preds = %2417, %2223
 51720|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB13_(ptr %22)
 51721|  to label %2239 unwind label %2236                                                                                     ;L825<413
 51722| 
 51723| 2236: ; preds = %2235
 51724|  %2237 = cleanuppad within none []
 51726|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB1a_(ptr %22) [ "funclet"(token %2237) ]
 51727|  to label %2238 unwind label %1486                                                                                     ;L825<825<413
 51728| 
 51729| 2238: ; preds = %2236
 51730|  cleanupret from %2237 unwind label %1486
 51731| 
 51732| 2239: ; preds = %2235
 51734|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB1a_(ptr %22)
 51735|  to label %2418 unwind label %1486                                                                                     ;L825<825<413
 51736| 
 51737| 2240: ; preds = %2451, %2450, %2448, %2412, %2321, %2255, %2247, %2227
 51738|  %2241 = phi i1 [ true, %2451 ], [ true, %2321 ], [ false, %2412 ], [ true, %2247 ], [ true, %2255 ], [ true, %2227 ], [ true, %2450 ], [ true, %2448 ] ;L0
 51739|  %2242 = phi i1 [ false, %2451 ], [ true, %2321 ], [ false, %2412 ], [ true, %2247 ], [ true, %2255 ], [ true, %2227 ], [ false, %2450 ], [ false, %2448 ] ;L0
 51740|  %2243 = cleanuppad within none []
 51741|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecRTxNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEEB1w_(ptr %22) #31 [ "funclet"(token %2243) ] ;L413
 51742|  cleanupret from %2243 unwind label %1486                                                                              ;L413
 51743| 
 51744| 2244: ; preds = %2227
 51745|  %2245 = extractvalue { i64, ptr } %2234, 1                                                                            ;L2707<3354<3325<403
 51746|     ;; self = ptr %2245
 51747|  %2246 = icmp eq ptr %2245, null                                                                                       ;L1011<403
 51748|  br i1 %2246, label %2255, label %2247                                                                                 ;L1011<403
 51749| 
 51750| 2247: ; preds = %2244
 51751|  %2248 = load ptr, ptr %2245, , !!8, !!8                                                                               ;L403
 51752|  %2249 = load i64, ptr %2248, , !!8                                                                                    ;L403
 51753|  store i64 %2249, ptr %21,                                                                                             ;L403
 51756|     ;; self = ptr %22
 51757|     ;; self = ptr %22
 51758|  %2250 = load ptr, ptr %22, , !!8, !!8                                                                                 ;L138<2073<405
 51759|     ;; p = ptr %2250
 51760|  %2251 = load i64, ptr %2220, , !!8                                                                                    ;L2075<405
 51761|     ;; len = i64 %2251
 51762|     ;; count = i64 %2251
 51763|     ;; self[0..+8] = ptr %2250
 51764|     ;; slice[0..+8] = ptr %2250
 51765|     ;; self[8..+8] = i64 %2251
 51766|     ;; slice[8..+8] = i64 %2251
 51767|     ;; ptr = ptr %2250
 51768|     ;; self = ptr %2250
 51769|  %2252 = getelementptr ptr, ptr %2250, i64 %2251                                                                       ;L961<100<1042<405
 51770|     ;; self[0..+8] = ptr %2250
 51771|     ;; self[8..+8] = ptr %2252
 51772|     ;; self[16..+8] = ptr %21
 51773|  store ptr %2250, ptr %19,                                                                                             ;L69<836<405
 51774|  %2253 = gep %19, i64 8                                                                                                ;L69<836<405
 51775|  store ptr %2252, ptr %2253,                                                                                           ;L69<836<405
 51776|  %2254 = gep %19, i64 16                                                                                               ;L69<836<405
 51777|  store ptr %21, ptr %2254,                                                                                             ;L69<836<405
 51778|  invoke void @core::iter8adapters3map3MapINtNtB29_6filter6FilterINtNtNtB2d_5slice4iter4IterRTxBU_EENCNvMNtNtNtBY_11plan_legacy7handler7auctionNtB3V_17LegacyPlanHandler16get_small_actionsr_0ENCB3Q_ss_0EEBY_(ptr sret([32 x i8]) %20, ptr %19, ptr %655)
 51779|  to label %2256 unwind label %2240                                                                                     ;L404
 51780| 
 51781| 2255: ; preds = %2244
 51782|  invoke void @core::option13unwrap_failed(ptr @anon.282069a2ed2ad3a275929b639963fb55.154) #32
 51783|  to label %203 unwind label %2240                                                                                      ;L1013<403
 51784| 
 51785| 2256: ; preds = %2247
 51787|  %2257 = load i64, ptr %21, , !!8                                                                                      ;L407
 51788|  %2258 = icmp sgt i64 %2257, -1                                                                                        ;L407
 51789|  br i1 %2258, label %2316, label %2259                                                                                 ;L407
 51790| 
 51791| 2259: ; preds = %2256
 51792|     ;; self = ptr %1475
 51793|  %2260 = load i8, ptr %1476, , !!55508, !!8                                                                            ;L309<407
 51794|  %2261 = icmp ne i8 %2260, 10                                                                                          ;L309<407
 51795|  call void @llvm.assume(i1 %2261)                                                                                      ;L309<407
 51796|  %2262 = add nsw i8 %2260, -3                                                                                          ;L309<407
 51797|  %2263 = icmp samesign ugt i8 %2260, 2                                                                                 ;L309<407
 51798|  %2264 = select i1 %2263, i8 %2262, i8 7                                                                               ;L309<407
 51799|  switch i8 %2264, label %2265 [
 51800|  i8 0, label %2323
 51801|  i8 1, label %2323
 51802|  i8 2, label %2266
 51803|  i8 3, label %2269
 51804|  i8 4, label %2272
 51805|  i8 5, label %2323
 51806|  i8 6, label %2277
 51807|  i8 7, label %2282
 51808|  i8 8, label %2287
 51809|  i8 9, label %2292
 51810|  i8 10, label %2297
 51811|  i8 11, label %2300
 51812|  i8 12, label %2303
 51813|  i8 13, label %2306
 51814|  i8 14, label %2309
 51815|  i8 15, label %2312
 51816|  i8 16, label %2315
 51817|  ]                                                                                                                     ;L309<407
 51818| 
 51819| 2265: ; preds = %2259
 51820|  unreachable                                                                                                           ;L309<407
 51821| 
 51822| 2266: ; preds = %2259
 51823|     ;; action = ptr %1475
 51824|     ;; self = ptr %1475
 51825|  %2267 = gep %49, i64 16                                                                                               ;L285<316<407
 51826|  %2268 = load i64, ptr %2267, , !!55508, !!8                                                                           ;L285<316<407
 51827|  br label %2323                                                                                                        ;L316<407
 51828| 
 51829| 2269: ; preds = %2259
 51830|     ;; action = ptr %1475
 51831|     ;; self = ptr %1475
 51832|  %2270 = gep %49, i64 16                                                                                               ;L491<313<407
 51833|  %2271 = load i64, ptr %2270, , !!55508, !!8                                                                           ;L491<313<407
 51834|  br label %2323                                                                                                        ;L313<407
 51835| 
 51836| 2272: ; preds = %2259
 51837|     ;; action = ptr %1475
 51838|     ;; self = ptr %1475
 51839|  %2273 = gep %49, i64 24                                                                                               ;L665<314<407
 51840|  %2274 = load i64, ptr %2273, , !!55508, !!8                                                                           ;L665<314<407
 51841|  %2275 = gep %49, i64 32                                                                                               ;L665<314<407
 51842|  %2276 = load i64, ptr %2275, , !!55508, !!8                                                                           ;L665<314<407
 51843|  br label %2323                                                                                                        ;L314<407
 51844| 
 51845| 2277: ; preds = %2259
 51846|     ;; action = ptr %1475
 51847|     ;; self = ptr %1475
 51848|  %2278 = gep %49, i64 16                                                                                               ;L800<312<407
 51849|  %2279 = load i64, ptr %2278, , !!55508, !!8                                                                           ;L800<312<407
 51850|  %2280 = gep %49, i64 24                                                                                               ;L800<312<407
 51851|  %2281 = load i64, ptr %2280, , !!55508, !!8                                                                           ;L800<312<407
 51852|  br label %2323                                                                                                        ;L312<407
 51853| 
 51854| 2282: ; preds = %2259
 51855|     ;; action = ptr %1475
 51856|     ;; self = ptr %1475
 51857|  %2283 = gep %49, i64 56                                                                                               ;L1035<317<407
 51858|  %2284 = load i64, ptr %2283, , !!55508, !!8                                                                           ;L1035<317<407
 51859|  %2285 = gep %49, i64 64                                                                                               ;L1035<317<407
 51860|  %2286 = load i64, ptr %2285, , !!55508, !!8                                                                           ;L1035<317<407
 51861|  br label %2323                                                                                                        ;L317<407
 51862| 
 51863| 2287: ; preds = %2259
 51864|     ;; action = ptr %1475
 51865|     ;; self = ptr %1475
 51866|  %2288 = gep %49, i64 16                                                                                               ;L1123<318<407
 51867|  %2289 = load i64, ptr %2288, , !!55508, !!8                                                                           ;L1123<318<407
 51868|  %2290 = gep %49, i64 24                                                                                               ;L1123<318<407
 51869|  %2291 = load i64, ptr %2290, , !!55508, !!8                                                                           ;L1123<318<407
 51870|  br label %2323                                                                                                        ;L318<407
 51871| 
 51872| 2292: ; preds = %2259
 51873|     ;; action = ptr %1475
 51874|     ;; self = ptr %1475
 51875|  %2293 = gep %49, i64 32                                                                                               ;L1241<319<407
 51876|  %2294 = load i64, ptr %2293, , !!55508, !!8                                                                           ;L1241<319<407
 51877|  %2295 = gep %49, i64 40                                                                                               ;L1241<319<407
 51878|  %2296 = load i64, ptr %2295, , !!55508, !!8                                                                           ;L1241<319<407
 51879|  br label %2323                                                                                                        ;L319<407
 51880| 
 51881| 2297: ; preds = %2259
 51882|     ;; action = ptr %1475
 51883|     ;; self = ptr %1475
 51884|  %2298 = gep %49, i64 16                                                                                               ;L653<320<407
 51885|  %2299 = load i64, ptr %2298, , !!55508, !!8                                                                           ;L653<320<407
 51886|  br label %2323                                                                                                        ;L320<407
 51887| 
 51888| 2300: ; preds = %2259
 51889|     ;; action = ptr %1475
 51890|     ;; self = ptr %1475
 51891|  %2301 = gep %49, i64 104                                                                                              ;L404<321<407
 51892|  %2302 = load i64, ptr %2301, , !!55508, !!8                                                                           ;L404<321<407
 51893|  br label %2323                                                                                                        ;L321<407
 51894| 
 51895| 2303: ; preds = %2259
 51896|     ;; action = ptr %1475
 51897|     ;; self = ptr %1475
 51898|  %2304 = gep %49, i64 16                                                                                               ;L94<322<407
 51899|  %2305 = load i64, ptr %2304, , !!55508, !!8                                                                           ;L94<322<407
 51900|  br label %2323                                                                                                        ;L322<407
 51901| 
 51902| 2306: ; preds = %2259
 51903|     ;; action = ptr %1475
 51904|     ;; self = ptr %1475
 51905|  %2307 = gep %49, i64 16                                                                                               ;L160<323<407
 51906|  %2308 = load i64, ptr %2307, , !!55508, !!8                                                                           ;L160<323<407
 51907|  br label %2323                                                                                                        ;L323<407
 51908| 
 51909| 2309: ; preds = %2259
 51910|     ;; action = ptr %1475
 51911|     ;; self = ptr %1475
 51912|  %2310 = gep %49, i64 16                                                                                               ;L222<324<407
 51913|  %2311 = load i64, ptr %2310, , !!55508, !!8                                                                           ;L222<324<407
 51914|  br label %2323                                                                                                        ;L324<407
 51915| 
 51916| 2312: ; preds = %2259
 51917|     ;; action = ptr %1475
 51918|     ;; self = ptr %1475
 51919|  %2313 = gep %49, i64 16                                                                                               ;L287<325<407
 51920|  %2314 = load i64, ptr %2313, , !!55508, !!8                                                                           ;L287<325<407
 51921|  br label %2323                                                                                                        ;L325<407
 51922| 
 51923| 2315: ; preds = %2259
 51924|  br label %2323                                                                                                        ;L326<407
 51925| 
 51926| 2316: ; preds = %2416, %2256
 51928|     ;; self = ptr %20
 51929|     ;; self = ptr %20
 51930|  %2317 = load ptr, ptr %20, , !!8, !!8                                                                                 ;L138<2073<408
 51931|     ;; p = ptr %2317
 51932|  %2318 = gep %20, i64 24                                                                                               ;L2075<408
 51933|  %2319 = load i64, ptr %2318, , !!8                                                                                    ;L2075<408
 51934|  %2320 = invoke ptr @_RINvXNtCsMBkRBYhlca_4rand3seqSNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayNtB3_11SliceRandom6chooseNtNtNtB5_4rngs3std6StdRngEBw_(ptr %2317, i64 %2319, ptr %3)
 51935|  to label %2440 unwind label %2321                                                                                     ;L408
 51936| 
 51937| 2321: ; preds = %2443, %2442, %2410, %2316
 51938|  %2322 = cleanuppad within none []
 51939|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %20) #31 [ "funclet"(token %2322) ] ;L413
 51940|  cleanupret from %2322 unwind label %2240                                                                              ;L413
 51941| 
 51942| 2323: ; preds = %2315, %2312, %2309, %2306, %2303, %2300, %2297, %2292, %2287, %2282, %2277, %2272, %2269, %2266, %2259, %2259, %2259
 51943|  %2324 = phi i64 [ %2268, %2266 ], [ %2271, %2269 ], [ %2274, %2272 ], [ undef, %2259 ], [ undef, %2259 ], [ undef, %2259 ], [ %2279, %2277 ], [ %2284, %2282 ], [ %2289, %2287 ], [ %2294, %2292 ], [ %2299, %2297 ], [ %2302, %2300 ], [ %2305, %2303 ], [ %2308, %2306 ], [ %2311, %2309 ], [ %2314, %2312 ], [ undef, %2315 ]
 51944|  %2325 = phi i64 [ undef, %2266 ], [ undef, %2269 ], [ %2276, %2272 ], [ undef, %2259 ], [ undef, %2259 ], [ undef, %2259 ], [ %2281, %2277 ], [ %2286, %2282 ], [ %2291, %2287 ], [ %2296, %2292 ], [ undef, %2297 ], [ undef, %2300 ], [ undef, %2303 ], [ undef, %2306 ], [ undef, %2309 ], [ undef, %2312 ], [ undef, %2315 ]
 51945|  %2326 = phi i64 [ 2, %2266 ], [ 2, %2269 ], [ 3, %2272 ], [ 0, %2259 ], [ 0, %2259 ], [ 0, %2259 ], [ 1, %2277 ], [ 3, %2282 ], [ 3, %2287 ], [ 3, %2292 ], [ 2, %2297 ], [ 4, %2300 ], [ 6, %2303 ], [ 7, %2306 ], [ 8, %2309 ], [ 9, %2312 ], [ 10, %2315 ]
 51946|     ;; self = ptr %6
 51947|  %2327 = gep %6, i64 177                                                                                               ;L309<407
 51948|  %2328 = load i8, ptr %2327, , !!55591, !!8                                                                            ;L309<407
 51949|  %2329 = icmp ne i8 %2328, 10                                                                                          ;L309<407
 51950|  call void @llvm.assume(i1 %2329)                                                                                      ;L309<407
 51951|  %2330 = add nsw i8 %2328, -3                                                                                          ;L309<407
 51952|  %2331 = icmp samesign ugt i8 %2328, 2                                                                                 ;L309<407
 51953|  %2332 = select i1 %2331, i8 %2330, i8 7                                                                               ;L309<407
 51954|  switch i8 %2332, label %2333 [
 51955|  i8 0, label %2384
 51956|  i8 1, label %2384
 51957|  i8 2, label %2334
 51958|  i8 3, label %2337
 51959|  i8 4, label %2340
 51960|  i8 5, label %2384
 51961|  i8 6, label %2345
 51962|  i8 7, label %2350
 51963|  i8 8, label %2355
 51964|  i8 9, label %2360
 51965|  i8 10, label %2365
 51966|  i8 11, label %2368
 51967|  i8 12, label %2371
 51968|  i8 13, label %2374
 51969|  i8 14, label %2377
 51970|  i8 15, label %2380
 51971|  i8 16, label %2383
 51972|  ]                                                                                                                     ;L309<407
 51973| 
 51974| 2333: ; preds = %2323
 51975|  unreachable                                                                                                           ;L309<407
 51976| 
 51977| 2334: ; preds = %2323
 51978|     ;; action = ptr %6
 51979|     ;; self = ptr %6
 51980|  %2335 = gep %6, i64 8                                                                                                 ;L285<316<407
 51981|  %2336 = load i64, ptr %2335, , !!55591, !!8                                                                           ;L285<316<407
 51982|  br label %2384                                                                                                        ;L316<407
 51983| 
 51984| 2337: ; preds = %2323
 51985|     ;; action = ptr %6
 51986|     ;; self = ptr %6
 51987|  %2338 = gep %6, i64 8                                                                                                 ;L491<313<407
 51988|  %2339 = load i64, ptr %2338, , !!55591, !!8                                                                           ;L491<313<407
 51989|  br label %2384                                                                                                        ;L313<407
 51990| 
 51991| 2340: ; preds = %2323
 51992|     ;; action = ptr %6
 51993|     ;; self = ptr %6
 51994|  %2341 = gep %6, i64 16                                                                                                ;L665<314<407
 51995|  %2342 = load i64, ptr %2341, , !!55591, !!8                                                                           ;L665<314<407
 51996|  %2343 = gep %6, i64 24                                                                                                ;L665<314<407
 51997|  %2344 = load i64, ptr %2343, , !!55591, !!8                                                                           ;L665<314<407
 51998|  br label %2384                                                                                                        ;L314<407
 51999| 
 52000| 2345: ; preds = %2323
 52001|     ;; action = ptr %6
 52002|     ;; self = ptr %6
 52003|  %2346 = gep %6, i64 8                                                                                                 ;L800<312<407
 52004|  %2347 = load i64, ptr %2346, , !!55591, !!8                                                                           ;L800<312<407
 52005|  %2348 = gep %6, i64 16                                                                                                ;L800<312<407
 52006|  %2349 = load i64, ptr %2348, , !!55591, !!8                                                                           ;L800<312<407
 52007|  br label %2384                                                                                                        ;L312<407
 52008| 
 52009| 2350: ; preds = %2323
 52010|     ;; action = ptr %6
 52011|     ;; self = ptr %6
 52012|  %2351 = gep %6, i64 48                                                                                                ;L1035<317<407
 52013|  %2352 = load i64, ptr %2351, , !!55591, !!8                                                                           ;L1035<317<407
 52014|  %2353 = gep %6, i64 56                                                                                                ;L1035<317<407
 52015|  %2354 = load i64, ptr %2353, , !!55591, !!8                                                                           ;L1035<317<407
 52016|  br label %2384                                                                                                        ;L317<407
 52017| 
 52018| 2355: ; preds = %2323
 52019|     ;; action = ptr %6
 52020|     ;; self = ptr %6
 52021|  %2356 = gep %6, i64 8                                                                                                 ;L1123<318<407
 52022|  %2357 = load i64, ptr %2356, , !!55591, !!8                                                                           ;L1123<318<407
 52023|  %2358 = gep %6, i64 16                                                                                                ;L1123<318<407
 52024|  %2359 = load i64, ptr %2358, , !!55591, !!8                                                                           ;L1123<318<407
 52025|  br label %2384                                                                                                        ;L318<407
 52026| 
 52027| 2360: ; preds = %2323
 52028|     ;; action = ptr %6
 52029|     ;; self = ptr %6
 52030|  %2361 = gep %6, i64 24                                                                                                ;L1241<319<407
 52031|  %2362 = load i64, ptr %2361, , !!55591, !!8                                                                           ;L1241<319<407
 52032|  %2363 = gep %6, i64 32                                                                                                ;L1241<319<407
 52033|  %2364 = load i64, ptr %2363, , !!55591, !!8                                                                           ;L1241<319<407
 52034|  br label %2384                                                                                                        ;L319<407
 52035| 
 52036| 2365: ; preds = %2323
 52037|     ;; action = ptr %6
 52038|     ;; self = ptr %6
 52039|  %2366 = gep %6, i64 8                                                                                                 ;L653<320<407
 52040|  %2367 = load i64, ptr %2366, , !!55591, !!8                                                                           ;L653<320<407
 52041|  br label %2384                                                                                                        ;L320<407
 52042| 
 52043| 2368: ; preds = %2323
 52044|     ;; action = ptr %6
 52045|     ;; self = ptr %6
 52046|  %2369 = gep %6, i64 96                                                                                                ;L404<321<407
 52047|  %2370 = load i64, ptr %2369, , !!55591, !!8                                                                           ;L404<321<407
 52048|  br label %2384                                                                                                        ;L321<407
 52049| 
 52050| 2371: ; preds = %2323
 52051|     ;; action = ptr %6
 52052|     ;; self = ptr %6
 52053|  %2372 = gep %6, i64 8                                                                                                 ;L94<322<407
 52054|  %2373 = load i64, ptr %2372, , !!55591, !!8                                                                           ;L94<322<407
 52055|  br label %2384                                                                                                        ;L322<407
 52056| 
 52057| 2374: ; preds = %2323
 52058|     ;; action = ptr %6
 52059|     ;; self = ptr %6
 52060|  %2375 = gep %6, i64 8                                                                                                 ;L160<323<407
 52061|  %2376 = load i64, ptr %2375, , !!55591, !!8                                                                           ;L160<323<407
 52062|  br label %2384                                                                                                        ;L323<407
 52063| 
 52064| 2377: ; preds = %2323
 52065|     ;; action = ptr %6
 52066|     ;; self = ptr %6
 52067|  %2378 = gep %6, i64 8                                                                                                 ;L222<324<407
 52068|  %2379 = load i64, ptr %2378, , !!55591, !!8                                                                           ;L222<324<407
 52069|  br label %2384                                                                                                        ;L324<407
 52070| 
 52071| 2380: ; preds = %2323
 52072|     ;; action = ptr %6
 52073|     ;; self = ptr %6
 52074|  %2381 = gep %6, i64 8                                                                                                 ;L287<325<407
 52075|  %2382 = load i64, ptr %2381, , !!55591, !!8                                                                           ;L287<325<407
 52076|  br label %2384                                                                                                        ;L325<407
 52077| 
 52078| 2383: ; preds = %2323
 52079|  br label %2384                                                                                                        ;L326<407
 52080| 
 52081| 2384: ; preds = %2383, %2380, %2377, %2374, %2371, %2368, %2365, %2360, %2355, %2350, %2345, %2340, %2337, %2334, %2323, %2323, %2323
 52082|  %2385 = phi i64 [ %2336, %2334 ], [ %2339, %2337 ], [ %2342, %2340 ], [ undef, %2323 ], [ undef, %2323 ], [ undef, %2323 ], [ %2347, %2345 ], [ %2352, %2350 ], [ %2357, %2355 ], [ %2362, %2360 ], [ %2367, %2365 ], [ %2370, %2368 ], [ %2373, %2371 ], [ %2376, %2374 ], [ %2379, %2377 ], [ %2382, %2380 ], [ undef, %2383 ]
 52083|  %2386 = phi i64 [ undef, %2334 ], [ undef, %2337 ], [ %2344, %2340 ], [ undef, %2323 ], [ undef, %2323 ], [ undef, %2323 ], [ %2349, %2345 ], [ %2354, %2350 ], [ %2359, %2355 ], [ %2364, %2360 ], [ undef, %2365 ], [ undef, %2368 ], [ undef, %2371 ], [ undef, %2374 ], [ undef, %2377 ], [ undef, %2380 ], [ undef, %2383 ]
 52084|  %2387 = phi i64 [ 2, %2334 ], [ 2, %2337 ], [ 3, %2340 ], [ 0, %2323 ], [ 0, %2323 ], [ 0, %2323 ], [ 1, %2345 ], [ 3, %2350 ], [ 3, %2355 ], [ 3, %2360 ], [ 2, %2365 ], [ 4, %2368 ], [ 6, %2371 ], [ 7, %2374 ], [ 8, %2377 ], [ 9, %2380 ], [ 10, %2383 ]
 52085|     ;; self = ptr undef
 52086|     ;; other = ptr undef
 52087|     ;; __self_discr = i64 %2326
 52088|     ;; __arg1_discr = i64 %2387
 52089|  %2388 = icmp eq i64 %2326, %2387                                                                                      ;L81<407
 52090|  br i1 %2388, label %2389, label %2412                                                                                 ;L81<407
 52091| 
 52092| 2389: ; preds = %2384
 52093|  switch i64 %2326, label %2410 [
 52094|  i64 1, label %2390
 52095|  i64 2, label %2394
 52096|  i64 3, label %2396
 52097|  i64 4, label %2400
 52098|  i64 6, label %2402
 52099|  i64 7, label %2404
 52100|  i64 8, label %2406
 52101|  i64 9, label %2408
 52102|  ]                                                                                                                     ;L81<407
 52103| 
 52104| 2390: ; preds = %2389
 52105|     ;; __self_0 = ptr undef
 52106|     ;; self = ptr undef
 52107|     ;; __self_1 = ptr undef
 52108|     ;; self = ptr undef
 52109|     ;; __arg1_0 = ptr undef
 52110|     ;; other = ptr undef
 52111|     ;; __arg1_1 = ptr undef
 52112|     ;; other = ptr undef
 52115|  %2391 = icmp eq i64 %2324, %2385                                                                                      ;L1878<2123<84<407
 52116|  %2392 = icmp eq i64 %2325, %2386
 52117|  %2393 = and i1 %2391, %2392                                                                                           ;L84<407
 52118|  br i1 %2393, label %2410, label %2412                                                                                 ;L84<407
 52119| 
 52120| 2394: ; preds = %2389
 52121|     ;; __self_0 = ptr undef
 52122|     ;; self = ptr undef
 52123|     ;; __arg1_0 = ptr undef
 52124|     ;; other = ptr undef
 52127|  %2395 = icmp eq i64 %2324, %2385                                                                                      ;L1878<2123<85<407
 52128|  br i1 %2395, label %2410, label %2412                                                                                 ;L407
 52129| 
 52130| 2396: ; preds = %2389
 52131|     ;; __self_0 = ptr undef
 52132|     ;; self = ptr undef
 52133|     ;; __self_1 = ptr undef
 52134|     ;; self = ptr undef
 52135|     ;; __arg1_0 = ptr undef
 52136|     ;; other = ptr undef
 52137|     ;; __arg1_1 = ptr undef
 52138|     ;; other = ptr undef
 52141|  %2397 = icmp eq i64 %2324, %2385                                                                                      ;L1878<2123<86<407
 52142|  %2398 = icmp eq i64 %2325, %2386
 52143|  %2399 = and i1 %2397, %2398                                                                                           ;L86<407
 52144|  br i1 %2399, label %2410, label %2412                                                                                 ;L86<407
 52145| 
 52146| 2400: ; preds = %2389
 52147|     ;; __self_0 = ptr undef
 52148|     ;; self = ptr undef
 52149|     ;; __arg1_0 = ptr undef
 52150|     ;; other = ptr undef
 52153|  %2401 = icmp eq i64 %2324, %2385                                                                                      ;L1878<2123<87<407
 52154|  br i1 %2401, label %2410, label %2412                                                                                 ;L407
 52155| 
 52156| 2402: ; preds = %2389
 52157|     ;; __self_0 = ptr undef
 52158|     ;; self = ptr undef
 52159|     ;; __arg1_0 = ptr undef
 52160|     ;; other = ptr undef
 52163|  %2403 = icmp eq i64 %2324, %2385                                                                                      ;L1878<2123<89<407
 52164|  br i1 %2403, label %2410, label %2412                                                                                 ;L407
 52165| 
 52166| 2404: ; preds = %2389
 52167|     ;; __self_0 = ptr undef
 52168|     ;; self = ptr undef
 52169|     ;; __arg1_0 = ptr undef
 52170|     ;; other = ptr undef
 52173|  %2405 = icmp eq i64 %2324, %2385                                                                                      ;L1878<2123<90<407
 52174|  br i1 %2405, label %2410, label %2412                                                                                 ;L407
 52175| 
 52176| 2406: ; preds = %2389
 52177|     ;; __self_0 = ptr undef
 52178|     ;; self = ptr undef
 52179|     ;; __arg1_0 = ptr undef
 52180|     ;; other = ptr undef
 52183|  %2407 = icmp eq i64 %2324, %2385                                                                                      ;L1878<2123<91<407
 52184|  br i1 %2407, label %2410, label %2412                                                                                 ;L407
 52185| 
 52186| 2408: ; preds = %2389
 52187|     ;; __self_0 = ptr undef
 52188|     ;; self = ptr undef
 52189|     ;; __arg1_0 = ptr undef
 52190|     ;; other = ptr undef
 52193|  %2409 = icmp eq i64 %2324, %2385                                                                                      ;L1878<2123<92<407
 52194|  br i1 %2409, label %2410, label %2412                                                                                 ;L407
 52195| 
 52196| 2410: ; preds = %2408, %2406, %2404, %2402, %2400, %2396, %2394, %2390, %2389
 52197|  %2411 = invoke zeroext i1 @ai::small_actionNtB2_15SmallActionPlay18move_near_complete(ptr %6, ptr %200)
 52198|  to label %2416 unwind label %2321                                                                                     ;L407
 52199| 
 52200| 2412: ; preds = %2416, %2408, %2406, %2404, %2402, %2400, %2396, %2394, %2390, %2384
 52201|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %95, i64 5384, i1 false)                                                 ;L411
 52202|  %2413 = load i64, ptr %49, , !!8                                                                                      ;L411
 52203|  %2414 = gep %0, i64 5392                                                                                              ;L411
 52204|  call void @llvm.memcpy.p0.p0.i64(ptr %2414, ptr %1475, i64 184, i1 false)                                             ;L411
 52205|  %2415 = gep %0, i64 5384                                                                                              ;L411
 52206|  store i64 %2413, ptr %2415,                                                                                           ;L411
 52207|  invoke fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %20)
 52208|  to label %2417 unwind label %2240                                                                                     ;L413
 52209| 
 52210| 2416: ; preds = %2410
 52211|  br i1 %2411, label %2316, label %2412                                                                                 ;L407
 52212| 
 52213| 2417: ; preds = %2412
 52216|  br label %2235                                                                                                        ;L1
 52217| 
 52218| 2418: ; preds = %2239
 52220|  br label %2187                                                                                                        ;L1
 52221| 
 52222| 2419: ; preds = %2191
 52226|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB12_(ptr %58)
 52227|  to label %2423 unwind label %2420                                                                                     ;L825<416
 52228| 
 52229| 2420: ; preds = %2419
 52230|  %2421 = cleanuppad within none []
 52232|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB19_(ptr %58) [ "funclet"(token %2421) ]
 52233|  to label %2422 unwind label %679                                                                                      ;L825<825<416
 52234| 
 52235| 2422: ; preds = %2420
 52236|  cleanupret from %2421 unwind label %679
 52237| 
 52238| 2423: ; preds = %2419
 52240|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB19_(ptr %58)
 52241|  to label %2424 unwind label %679                                                                                      ;L825<825<416
 52242| 
 52243| 2424: ; preds = %2423
 52245|  %2425 = trunc nuw i8 %1370 to i1                                                                                      ;L416
 52246|  br i1 %2425, label %2428, label %2426                                                                                 ;L416
 52247| 
 52248| 2426: ; preds = %2432, %2424
 52253|  %2427 = trunc nuw i8 %513 to i1                                                                                       ;L416
 52254|  br i1 %2427, label %2434, label %2433                                                                                 ;L416
 52255| 
 52256| 2428: ; preds = %2424
 52258|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB12_(ptr %70)
 52259|  to label %2432 unwind label %2429                                                                                     ;L825<416
 52260| 
 52261| 2429: ; preds = %2428
 52262|  %2430 = cleanuppad within none []
 52264|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB19_(ptr %70) [ "funclet"(token %2430) ]
 52265|  to label %2431 unwind label %634                                                                                      ;L825<825<416
 52266| 
 52267| 2431: ; preds = %2429
 52268|  cleanupret from %2430 unwind label %634
 52269| 
 52270| 2432: ; preds = %2428
 52272|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB19_(ptr %70)
 52273|  to label %2426 unwind label %634                                                                                      ;L825<825<416
 52274| 
 52275| 2433: ; preds = %2438, %2426
 52278|  br label %2439                                                                                                        ;L1
 52279| 
 52280| 2434: ; preds = %2426
 52282|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %80)
 52283|  to label %2438 unwind label %2435                                                                                     ;L825<416
 52284| 
 52285| 2435: ; preds = %2434
 52286|  %2436 = cleanuppad within none []
 52288|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %80) [ "funclet"(token %2436) ]
 52289|  to label %2437 unwind label %421                                                                                      ;L825<825<416
 52290| 
 52291| 2437: ; preds = %2435
 52292|  cleanupret from %2436 unwind label %421
 52293| 
 52294| 2438: ; preds = %2434
 52296|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %80)
 52297|  to label %2433 unwind label %421                                                                                      ;L825<825<416
 52298| 
 52299| 2439: ; preds = %2495, %2477, %2433, %402, %314
 52303|  ret void                                                                                                              ;L416
 52304| 
 52305| 2440: ; preds = %2316
 52306|     ;; self = ptr %2320
 52307|  %2441 = icmp eq ptr %2320, null                                                                                       ;L1011<408
 52308|  br i1 %2441, label %2443, label %2442                                                                                 ;L1011<408
 52309| 
 52310| 2442: ; preds = %2440
 52311|  invoke fastcc void @ai::small_actionNtB4_15SmallActionPlayNtNtCsjihNppCmMEE_4core5clone5Clone5clone(ptr %18, ptr %2320)
 52312|  to label %2444 unwind label %2321                                                                                     ;L408
 52313| 
 52314| 2443: ; preds = %2440
 52315|  invoke void @core::option13unwrap_failed(ptr @anon.282069a2ed2ad3a275929b639963fb55.155) #32
 52316|  to label %203 unwind label %2321                                                                                      ;L1013<408
 52317| 
 52318| 2444: ; preds = %2442
 52319|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %95, i64 5384, i1 false)                                                 ;L409
 52320|  %2445 = load i64, ptr %21, , !!8                                                                                      ;L409
 52321|  %2446 = gep %0, i64 5384                                                                                              ;L409
 52322|  store i64 %2445, ptr %2446,                                                                                           ;L409
 52323|  %2447 = gep %0, i64 5392                                                                                              ;L409
 52324|  call void @llvm.memcpy.p0.p0.i64(ptr %2447, ptr %18, i64 184, i1 false)                                               ;L409
 52327|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %20)
 52328|  to label %2451 unwind label %2448                                                                                     ;L825<413
 52329| 
 52330| 2448: ; preds = %2444
 52331|  %2449 = cleanuppad within none []
 52333|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %20) [ "funclet"(token %2449) ]
 52334|  to label %2450 unwind label %2240                                                                                     ;L825<825<413
 52335| 
 52336| 2450: ; preds = %2448
 52337|  cleanupret from %2449 unwind label %2240
 52338| 
 52339| 2451: ; preds = %2444
 52341|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %20)
 52342|  to label %2452 unwind label %2240                                                                                     ;L825<825<413
 52343| 
 52344| 2452: ; preds = %2451
 52348|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB13_(ptr %22)
 52349|  to label %2456 unwind label %2453                                                                                     ;L825<413
 52350| 
 52351| 2453: ; preds = %2452
 52352|  %2454 = cleanuppad within none []
 52354|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB1a_(ptr %22) [ "funclet"(token %2454) ]
 52355|  to label %2455 unwind label %1486                                                                                     ;L825<825<413
 52356| 
 52357| 2455: ; preds = %2453
 52358|  cleanupret from %2454 unwind label %1486
 52359| 
 52360| 2456: ; preds = %2452
 52362|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB1a_(ptr %22)
 52363|  to label %2457 unwind label %1486                                                                                     ;L825<825<413
 52364| 
 52365| 2457: ; preds = %2456
 52367|  br label %2210                                                                                                        ;L416
 52368| 
 52369| 2458: ; preds = %2211, %2210
 52372|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %51)
 52373|  to label %2462 unwind label %2459                                                                                     ;L825<416
 52374| 
 52375| 2459: ; preds = %2458
 52376|  %2460 = cleanuppad within none []
 52378|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %51) [ "funclet"(token %2460) ]
 52379|  to label %2461 unwind label %1381                                                                                     ;L825<825<416
 52380| 
 52381| 2461: ; preds = %2459
 52382|  cleanupret from %2460 unwind label %1381
 52383| 
 52384| 2462: ; preds = %2458
 52386|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %51)
 52387|  to label %2463 unwind label %1381                                                                                     ;L825<825<416
 52388| 
 52389| 2463: ; preds = %2462
 52393|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB12_(ptr %58)
 52394|  to label %2467 unwind label %2464                                                                                     ;L825<416
 52395| 
 52396| 2464: ; preds = %2463
 52397|  %2465 = cleanuppad within none []
 52399|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB19_(ptr %58) [ "funclet"(token %2465) ]
 52400|  to label %2466 unwind label %679                                                                                      ;L825<825<416
 52401| 
 52402| 2466: ; preds = %2464
 52403|  cleanupret from %2465 unwind label %679
 52404| 
 52405| 2467: ; preds = %2463
 52407|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB19_(ptr %58)
 52408|  to label %2468 unwind label %679                                                                                      ;L825<825<416
 52409| 
 52410| 2468: ; preds = %2467
 52412|  %2469 = trunc nuw i8 %1370 to i1                                                                                      ;L416
 52413|  br i1 %2469, label %2472, label %2470                                                                                 ;L416
 52414| 
 52415| 2470: ; preds = %2476, %2468
 52420|  %2471 = trunc nuw i8 %513 to i1                                                                                       ;L416
 52421|  br i1 %2471, label %2478, label %2477                                                                                 ;L416
 52422| 
 52423| 2472: ; preds = %2468
 52425|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB12_(ptr %70)
 52426|  to label %2476 unwind label %2473                                                                                     ;L825<416
 52427| 
 52428| 2473: ; preds = %2472
 52429|  %2474 = cleanuppad within none []
 52431|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB19_(ptr %70) [ "funclet"(token %2474) ]
 52432|  to label %2475 unwind label %634                                                                                      ;L825<825<416
 52433| 
 52434| 2475: ; preds = %2473
 52435|  cleanupret from %2474 unwind label %634
 52436| 
 52437| 2476: ; preds = %2472
 52439|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB19_(ptr %70)
 52440|  to label %2470 unwind label %634                                                                                      ;L825<825<416
 52441| 
 52442| 2477: ; preds = %2482, %2470
 52445|  br label %2439                                                                                                        ;L416
 52446| 
 52447| 2478: ; preds = %2470
 52449|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %80)
 52450|  to label %2482 unwind label %2479                                                                                     ;L825<416
 52451| 
 52452| 2479: ; preds = %2478
 52453|  %2480 = cleanuppad within none []
 52455|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %80) [ "funclet"(token %2480) ]
 52456|  to label %2481 unwind label %421                                                                                      ;L825<825<416
 52457| 
 52458| 2481: ; preds = %2479
 52459|  cleanupret from %2480 unwind label %421
 52460| 
 52461| 2482: ; preds = %2478
 52463|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %80)
 52464|  to label %2477 unwind label %421                                                                                      ;L825<825<416
 52465| 
 52466| 2483: ; preds = %1486
 52467|  cleanupret from %1489 unwind label %1434
 52468| 
 52469| 2484: ; preds = %1486
 52470|  %2485 = gep %49, i64 8                                                                                                ;L416
 52471|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %2485) #31 [ "funclet"(token %1489) ] ;L416
 52472|  cleanupret from %1489 unwind label %1434                                                                              ;L416
 52473| 
 52474| 2486: ; preds = %1935
 52476|  %2487 = trunc nuw i8 %1370 to i1                                                                                      ;L416
 52477|  br i1 %2487, label %2490, label %2488                                                                                 ;L416
 52478| 
 52479| 2488: ; preds = %2494, %2486
 52484|  %2489 = trunc nuw i8 %513 to i1                                                                                       ;L416
 52485|  br i1 %2489, label %2496, label %2495                                                                                 ;L416
 52486| 
 52487| 2490: ; preds = %2486
 52489|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB12_(ptr %70)
 52490|  to label %2494 unwind label %2491                                                                                     ;L825<416
 52491| 
 52492| 2491: ; preds = %2490
 52493|  %2492 = cleanuppad within none []
 52495|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB19_(ptr %70) [ "funclet"(token %2492) ]
 52496|  to label %2493 unwind label %634                                                                                      ;L825<825<416
 52497| 
 52498| 2493: ; preds = %2491
 52499|  cleanupret from %2492 unwind label %634
 52500| 
 52501| 2494: ; preds = %2490
 52503|  invoke void @ai::small_action15SmallActionPlayEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB19_(ptr %70)
 52504|  to label %2488 unwind label %634                                                                                      ;L825<825<416
 52505| 
 52506| 2495: ; preds = %2500, %2488
 52509|  br label %2439                                                                                                        ;L1
 52510| 
 52511| 2496: ; preds = %2488
 52513|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %80)
 52514|  to label %2500 unwind label %2497                                                                                     ;L825<416
 52515| 
 52516| 2497: ; preds = %2496
 52517|  %2498 = cleanuppad within none []
 52519|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %80) [ "funclet"(token %2498) ]
 52520|  to label %2499 unwind label %421                                                                                      ;L825<825<416
 52521| 
 52522| 2499: ; preds = %2497
 52523|  cleanupret from %2498 unwind label %421
 52524| 
 52525| 2500: ; preds = %2496
 52527|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %80)
 52528|  to label %2495 unwind label %421                                                                                      ;L825<825<416
 52529| 
 52530| 2501: ; preds = %679
 52531|  cleanupret from %682 unwind label %634
 52532| 
 52533| 2502: ; preds = %679
 52534|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecTxNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEEB1v_(ptr %70) #31 [ "funclet"(token %682) ] ;L416
 52535|  cleanupret from %682 unwind label %634                                                                                ;L416
 52536| 
 52537| 2503: ; preds = %634
 52538|  cleanupret from %637 unwind label %515
 52539| 
 52540| 2504: ; preds = %634
 52541|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %72) #31 [ "funclet"(token %637) ] ;L416
 52542|  cleanupret from %637 unwind label %515                                                                                ;L416
 52543| 
 52544| 2505: ; preds = %515
 52545|  cleanupret from %518 unwind label %448
 52546| 
 52547| 2506: ; preds = %515
 52548|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %78) #31 [ "funclet"(token %518) ] ;L416
 52549|  cleanupret from %518 unwind label %448                                                                                ;L416
 52550| 
 52551| 2507: ; preds = %448
 52552|  cleanupret from %451 unwind label %421
 52553| 
 52554| 2508: ; preds = %448
 52555|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %80) #31 [ "funclet"(token %451) ] ;L416
 52556|  cleanupret from %451 unwind label %421                                                                                ;L416
 52557| 
 52558| 2509: ; preds = %421
 52559|  cleanupret from %424 unwind label %157
 52560| 
 52561| 2510: ; preds = %421
 52562|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %81) #31 [ "funclet"(token %424) ] ;L416
 52563|  cleanupret from %424 unwind label %157                                                                                ;L416
 52564| 
 52565| 2511: ; preds = %157
 52566|  cleanupret from %160 unwind label %114
 52567| 
 52568| 2512: ; preds = %157
 52569|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %93) #31 [ "funclet"(token %160) ] ;L416
 52570|  cleanupret from %160 unwind label %114                                                                                ;L416
 52571| 
 52572| 2513: ; preds = %114
 52573|  cleanupret from %117 unwind label %108
 52574| 
 52575| 2514: ; preds = %114
 52576|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai15score_parameter14ScoreParameterEBF_(ptr %95) #31 [ "funclet"(token %117) ] ;L416
 52577|  cleanupret from %117 unwind label %108                                                                                ;L416
 52578| 
 52579| 2515: ; preds = %2516, %108
 52580|  cleanupret from %110 unwind to caller                                                                                 ;L8
 52581| 
 52582| 2516: ; preds = %108
 52583|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %96) #31 [ "funclet"(token %110) ] ;L416
 52584|  br label %2515                                                                                                        ;L416
 52585| }
