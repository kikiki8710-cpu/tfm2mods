 33111| define internal fastcc void @ai::abstract_input16get_input_target(ptr %0, i64 %1, ptr %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %7, i64 %8) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
 33113|  %10 = alloca [88 x i8],
 33114|  %11 = alloca [24 x i8],
 33115|  %12 = alloca [8 x i8],
 33116|  %13 = alloca [8 x i8],
 33117|  %14 = alloca [8 x i8],
 33118|  %15 = alloca [24 x i8],
 33120|  %16 = alloca [56 x i8],
 33134|  %17 = alloca [88 x i8],
 33135|  %18 = alloca [88 x i8],
 33136|  %19 = alloca [24 x i8],
 33137|  %20 = alloca [88 x i8],
 33138|  %21 = alloca [8 x i8],
 33139|  %22 = alloca [24 x i8],
 33140|  %23 = alloca [8 x i8],
 33141|  %24 = alloca [8 x i8],
 33143|  %25 = alloca [56 x i8],
 33156|  %26 = alloca [24 x i8],
 33157|     ;; version = i64 %1
 33158|     ;; rnd = ptr %2
 33159|     ;; player = ptr %3
 33160|     ;; data = ptr %4
 33161|     ;; positioning_score = ptr %5
 33162|     ;; target = ptr %6
 33163|     ;; self = ptr %6
 33164|     ;; entity = ptr %6
 33165|     ;; entity = ptr %6
 33166|     ;; entity = ptr %6
 33167|     ;; effect = ptr %7
 33168|     ;; speed = i64 %8
 33169|     ;; score = ptr %25
 33170|     ;; target_x = ptr %24
 33171|     ;; target_y = ptr %23
 33172|     ;; score = ptr %16
 33173|     ;; target_x = ptr %14
 33174|     ;; target_y = ptr %13
 33176|     ;; default = i64 0
 33177|     ;; n = i64 1
 33178|     ;; rhs = i32 1
 33179|     ;; rhs = i32 1
 33180|     ;; n = i64 1
 33181|     ;; rhs = i32 1
 33182|     ;; rhs = i32 1
 33183|     ;; start = i64 0
 33184|     ;; end = i64 1000
 33185|     ;; start = i64 0
 33186|     ;; end = i64 1000
 33187|     ;; n = i64 1
 33188|     ;; rhs = i32 1
 33189|     ;; rhs = i32 1
 33190|     ;; n = i64 1
 33191|     ;; rhs = i32 1
 33192|     ;; rhs = i32 1
 33193|     ;; start = i64 0
 33194|     ;; end = i64 1000
 33195|     ;; start = i64 0
 33196|     ;; end = i64 1000
 33197|  %27 = gep %3, i64 2352                                                                                                ;L346
 33198|  %28 = load i64, ptr %27, , !!8                                                                                        ;L346
 33199|  %29 = icmp ult i64 %28, 2                                                                                             ;L346
 33200|  br i1 %29, label %31, label %30                                                                                       ;L346
 33201| 
 33202| 30: ; preds = %9
 33203|  tail call void @core::panicking18panic_bounds_check(i64 %28, i64 2, ptr @anon.168add0ea037d45d276f5936ae758fe5.137) #30 ;L346
 33204|  unreachable                                                                                                           ;L346
 33205| 
 33206| 31: ; preds = %9
 33207|     ;; self = ptr %3
 33208|  %32 = gep %3, i64 2496                                                                                                ;L581<346
 33209|  %33 = load i32, ptr %32, , !!8                                                                                        ;L581<346
 33210|  %34 = zext nneg i32 %33 to i64                                                                                        ;L581<346
 33211|  %35 = load ptr, ptr %4, , !!8, !!8                                                                                    ;L346
 33212|  %36 = gep %35, i64 480                                                                                                ;L346
 33213|  %37 = getelementptr [5 x ptr], ptr %36, i64 %28                                                                       ;L346
 33214|  %38 = getelementptr ptr, ptr %37, i64 %34                                                                             ;L346
 33215|  %39 = load ptr, ptr %38, , !!8                                                                                        ;L346
 33216|     ;; self = ptr %39
 33217|  %40 = icmp eq ptr %39, null                                                                                           ;L2775<346
 33218|  br i1 %40, label %43, label %41                                                                                       ;L2775<346
 33219| 
 33220| 41: ; preds = %31
 33221|     ;; champ = ptr %39
 33222|     ;; entity = ptr %39
 33223|     ;; caster = ptr %39
 33224|     ;; caster = ptr %39
 33225|     ;; caster = ptr %39
 33226|  %42 = tail call zeroext i1 @gc::simulation6effectNtB2_6Effect11is_in_range(ptr %7, ptr %39, ptr %6)                   ;L348
 33227|  br i1 %42, label %46, label %45                                                                                       ;L348
 33228| 
 33229| 43: ; preds = %31
 33230|  store i32 -1, ptr %0,                                                                                                 ;L2790<346
 33231|  br label %44                                                                                                          ;L1
 33232| 
 33233| 44: ; preds = %833, %751, %742, %599, %514, %450, %328, %279, %249, %129, %70, %45, %43
 33234|  ret void                                                                                                              ;L655
 33235| 
 33236| 45: ; preds = %41
 33237|  store i32 -1, ptr %0,                                                                                                 ;L349
 33238|  br label %44                                                                                                          ;L1
 33239| 
 33240| 46: ; preds = %41
 33241|     ;; self = ptr %39
 33242|  %47 = load i64, ptr %39, , !!8                                                                                        ;L1136<1482<353
 33243|  %48 = trunc nuw i64 %47 to i1                                                                                         ;L1136<1482<353
 33244|  br i1 %48, label %64, label %49                                                                                       ;L1136<1482<353
 33245| 
 33246| 49: ; preds = %46
 33247|  %50 = gep %39, i64 8                                                                                                  ;L1136<1482<353
 33248|     ;; team = ptr %39
 33249|  %51 = load i64, ptr %50, , !!8                                                                                        ;L1137<1482<353
 33250|     ;; team = i64 %51
 33251|  %52 = icmp ult i64 %51, 2                                                                                             ;L1483<353
 33252|  br i1 %52, label %53, label %58                                                                                       ;L1483<353
 33253| 
 33254| 53: ; preds = %49
 33256|  %54 = gep %6, i64 56                                                                                                  ;L122<1483<353
 33257|  %55 = gepS %54, i64 %51                                                                                               ;L122<1483<353
 33258|  %56 = load i64, ptr %55, , !!8                                                                                        ;L122<1483<353
 33259|  %57 = icmp eq i64 %56, 0                                                                                              ;L122<1483<353
 33260|  br i1 %57, label %64, label %59                                                                                       ;L353
 33261| 
 33262| 58: ; preds = %49
 33263|  tail call void @core::panicking18panic_bounds_check(i64 %51, i64 2, ptr @anon.168add0ea037d45d276f5936ae758fe5.39) #30 ;L1483<353
 33264|  unreachable                                                                                                           ;L1483<353
 33265| 
 33266| 59: ; preds = %53
 33267|     ;; self = ptr %7
 33268|  %60 = gep %7, i64 48                                                                                                  ;L149<353
 33269|  %61 = load i32, ptr %60, , !!8                                                                                        ;L149<353
 33270|  %62 = add nsw i32 %61, -1                                                                                             ;L149<353
 33271|  %63 = icmp ult i32 %62, 2                                                                                             ;L149<353
 33272|  br i1 %63, label %64, label %70                                                                                       ;L149<353
 33273| 
 33274| 64: ; preds = %59, %53, %46
 33275|  %65 = gep %3, i64 384                                                                                                 ;L357
 33276|  %66 = tail call i64 @gc::simulation5state6playerNtB4_16AthleteParameter18skill_hit_accuracy(ptr %65)                  ;L357
 33277|     ;; skill_hit_accuracy = i64 %66
 33278|  %67 = tail call i64 @gc::simulation5state6playerNtB4_16AthleteParameter19skill_hit_effective(ptr %65)                 ;L358
 33279|     ;; caster_skill_hit = i64 %67
 33280|  %68 = sub i64 1000, %66                                                                                               ;L360
 33281|     ;; self = i64 %68
 33282|     ;; other = i64 1
 33288|  %69 = icmp eq i64 %8, 0                                                                                               ;L364
 33289|  br i1 %69, label %92, label %71                                                                                       ;L364
 33290| 
 33291| 70: ; preds = %59
 33292|  store i32 -1, ptr %0,                                                                                                 ;L354
 33293|  br label %44                                                                                                          ;L1
 33294| 
 33295| 71: ; preds = %64
 33296|  %72 = tail call i64 @llvm.umax.i64(i64 %68, i64 1)                                                                    ;L1039<360
 33297|     ;; start = !DIArgList(i64 1000, i64 %72)
 33298|     ;; min_range = !DIArgList(i64 1000, i64 %72)
 33299|     ;; end = !DIArgList(i64 1000, i64 %72)
 33300|     ;; max_range = !DIArgList(i64 1000, i64 %72)
 33301|     ;; timing_error = i64 %72
 33302|  %73 = gep %7, i64 32                                                                                                  ;L364
 33303|  %74 = load i64, ptr %73, , !!8                                                                                        ;L364
 33304|  %75 = mul i64 %74, 100                                                                                                ;L364
 33305|  %76 = add i64 %72, 1000                                                                                               ;L362
 33306|     ;; max_range = i64 %76
 33307|     ;; end = i64 %76
 33308|  %77 = sub i64 1000, %72                                                                                               ;L361
 33309|     ;; min_range = i64 %77
 33310|     ;; start = i64 %77
 33311|  %78 = udiv i64 %75, %8                                                                                                ;L364
 33313|  store i64 %77, ptr %26,                                                                                               ;L391<364
 33314|  %79 = gep %26, i64 8                                                                                                  ;L391<364
 33315|  store i64 %76, ptr %79,                                                                                               ;L391<364
 33316|  %80 = gep %26, i64 16                                                                                                 ;L391<364
 33317|  store i8 0, ptr %80,                                                                                                  ;L391<364
 33318|  %81 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %2, ptr %26)                             ;L364
 33320|  %82 = mul i64 %81, %78                                                                                                ;L364
 33321|  %83 = udiv i64 %82, 1000                                                                                              ;L364
 33322|     ;; start_timing = i64 %83
 33323|  %84 = load ptr, ptr %35, , !!8, !!8                                                                                   ;L368
 33324|  %85 = gep %35, i64 8                                                                                                  ;L368
 33325|  %86 = load ptr, ptr %85, , !!8, !!8                                                                                   ;L368
 33326|  %87 = gep %86, i64 64                                                                                                 ;L368
 33327|  %88 = load ptr, ptr %87, , !!8                                                                                        ;L368
 33328|  %89 = tail call { i64, ptr } %88(ptr %84)                                                                             ;L368
 33329|  %90 = extractvalue { i64, ptr } %89, 0                                                                                ;L368
 33330|  %91 = icmp eq i64 %90, 2                                                                                              ;L368
 33331|  br i1 %91, label %93, label %124                                                                                      ;L368
 33332| 
 33333| 92: ; preds = %64
 33334|  tail call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.168add0ea037d45d276f5936ae758fe5.138) #30 ;L364
 33335|  unreachable                                                                                                           ;L364
 33336| 
 33337| 93: ; preds = %71
 33338|     ;; self = ptr %7
 33339|     ;; self = ptr %7
 33340|  %94 = load ptr, ptr %7, , !!8, !!8                                                                                    ;L441<2127<2445<369
 33341|  %95 = gep %7, i64 8                                                                                                   ;L441<2127<2445<369
 33342|  %96 = load ptr, ptr %95, , !!8, !!8                                                                                   ;L441<2127<2445<369
 33343|  %97 = gep %96, i64 16                                                                                                 ;L2445<369
 33344|  %98 = load i64, ptr %97,                                                                                              ;L2445<369
 33345|  %99 = add nsw i64 %98, -1                                                                                             ;L2445<369
 33346|  %100 = and i64 %99, -16                                                                                               ;L2445<369
 33347|  %101 = gep %94, i64 %100                                                                                              ;L2445<369
 33348|  %102 = gep %101, i64 16                                                                                               ;L2445<369
 33349|  %103 = gep %96, i64 248                                                                                               ;L369
 33350|  %104 = load ptr, ptr %103, , !!8                                                                                      ;L369
 33351|  %105 = tail call { i64, i64 } %104(ptr %102)                                                                          ;L369
 33352|  %106 = extractvalue { i64, i64 } %105, 0                                                                              ;L369
 33353|     ;; self[0..+8] = i64 %106
 33355|     ;; f[0..+8] = ptr %39
 33356|     ;; f[8..+8] = ptr %6
 33357|  %107 = trunc nuw i64 %106 to i1                                                                                       ;L1161<370
 33358|  br i1 %107, label %108, label %121                                                                                    ;L1161<370
 33359| 
 33360| 108: ; preds = %93
 33361|  %109 = extractvalue { i64, i64 } %105, 1                                                                              ;L369
 33362|     ;; self[8..+8] = i64 %109
 33363|     ;; x = i64 %109
 33364|  %110 = gep %39, i64 1632                                                                                              ;L1162<370
 33365|  %111 = load i64, ptr %110, , !!8                                                                                      ;L1162<370
 33366|  %112 = gep %39, i64 1640                                                                                              ;L1162<370
 33367|  %113 = load i64, ptr %112, , !!8                                                                                      ;L1162<370
 33368|  %114 = gep %6, i64 1632                                                                                               ;L1162<370
 33369|  %115 = load i64, ptr %114, , !!8                                                                                      ;L1162<370
 33370|  %116 = gep %6, i64 1640                                                                                               ;L1162<370
 33371|  %117 = load i64, ptr %116, , !!8                                                                                      ;L1162<370
 33375|     ;; psp = i64 %109
 33376|  %118 = tail call i64 @gc::utils8distance(i64 %111, i64 %113, i64 %115, i64 %117)                                      ;L370<1162<370
 33377|     ;; self = i64 %109
 33378|     ;; other = i64 1
 33379|  %119 = tail call i64 @llvm.umax.i64(i64 %109, i64 1)                                                                  ;L1039<370<1162<370
 33380|  %120 = udiv i64 %118, %119                                                                                            ;L370<1162<370
 33381|     ;; self[8..+8] = i64 %120
 33382|     ;; self[0..+8] = i64 1
 33383|     ;; flight = i64 %120
 33384|  br label %121                                                                                                         ;L1043<371
 33385| 
 33386| 121: ; preds = %108, %93
 33387|  %122 = phi i64 [ %120, %108 ], [ 0, %93 ]                                                                             ;L0<371
 33388|     ;; flight = i64 %122
 33389|  %123 = add i64 %122, %83                                                                                              ;L372
 33390|     ;; lead_timing = i64 %123
 33391|  br label %124                                                                                                         ;L368
 33392| 
 33393| 124: ; preds = %121, %71
 33394|  %125 = phi i64 [ %123, %121 ], [ %83, %71 ]                                                                           ;L0
 33395|     ;; lead_timing = i64 %125
 33396|  %126 = gep %7, i64 48                                                                                                 ;L377
 33397|  %127 = load i32, ptr %126, , !!8                                                                                      ;L377
 33398|  switch i32 %127, label %128 [
 33399|  i32 0, label %129
 33400|  i32 1, label %133
 33401|  i32 2, label %191
 33402|  i32 3, label %249
 33403|  ]                                                                                                                     ;L377
 33404| 
 33405| 128: ; preds = %124
 33406|  unreachable
 33407| 
 33408| 129: ; preds = %124
 33409|     ;; self = ptr %7
 33410|  %130 = gep %6, i64 1472                                                                                               ;L163<652
 33411|  %131 = load i64, ptr %130, , !!8                                                                                      ;L163<652
 33412|  store i32 0, ptr %0,                                                                                                  ;L652
 33413|  %132 = gep %0, i64 8                                                                                                  ;L652
 33414|  store i64 %131, ptr %132,                                                                                             ;L652
 33415|  br label %44                                                                                                          ;L652
 33416| 
 33417| 133: ; preds = %124
 33418|     ;; self = ptr %7
 33419|     ;; self = ptr %7
 33420|  %134 = load ptr, ptr %7, , !!8, !!8                                                                                   ;L441<2127<2445<534
 33421|  %135 = gep %7, i64 8                                                                                                  ;L441<2127<2445<534
 33422|  %136 = load ptr, ptr %135, , !!8, !!8                                                                                 ;L441<2127<2445<534
 33423|  %137 = gep %136, i64 16                                                                                               ;L2445<534
 33424|  %138 = load i64, ptr %137,                                                                                            ;L2445<534
 33425|  %139 = add nsw i64 %138, -1                                                                                           ;L2445<534
 33426|  %140 = and i64 %139, -16                                                                                              ;L2445<534
 33427|  %141 = gep %134, i64 %140                                                                                             ;L2445<534
 33428|  %142 = gep %141, i64 16                                                                                               ;L2445<534
 33429|  %143 = gep %136, i64 216                                                                                              ;L534
 33430|  %144 = load ptr, ptr %143, , !!8                                                                                      ;L534
 33431|  %145 = tail call zeroext i1 %144(ptr %142)                                                                            ;L534
 33432|  br i1 %145, label %146, label %250                                                                                    ;L534
 33433| 
 33434| 146: ; preds = %133
 33435|  %147 = gep %5, i64 2744
 33436|  %148 = load i64, ptr %147, , !!8
 33437|  %149 = add i64 %148, -3                                                                                               ;L900<985<537
 33438|  %150 = gep %5, i64 2752
 33439|  %151 = load i64, ptr %150, , !!8
 33440|  %152 = add i64 %151, -3
 33441|  %153 = gep %16, i64 16
 33442|     ;; self = ptr undef
 33443|     ;; self = ptr undef
 33444|     ;; self = ptr undef
 33445|     ;; other = ptr undef
 33446|     ;; old = i64 0
 33447|     ;; start = i64 0
 33448|     ;; self = i64 0
 33449|     ;; self = i64 0
 33450|     ;; iter[0..+4] = i64 0
 33451|     ;; dx = i64 0
 33452|     ;; iter[4..+4] = i32 6
 33453|     ;; best_pos[20..+4] = i32 undef
 33454|     ;; best_pos[16..+4] = i32 undef
 33455|     ;; best_pos[8..+8] = i64 undef
 33457|     ;; self = ptr undef
 33458|     ;; self = ptr undef
 33459|     ;; self = ptr undef
 33460|     ;; other = ptr undef
 33461|     ;; start = i64 0
 33462|     ;; self = i64 0
 33463|     ;; self = i64 0
 33464|     ;; iter[0..+4] = i64 1
 33465|     ;; dy = i64 0
 33467|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %16, i64 %1, ptr %3, ptr %4, ptr %5, i64 %149, i64 %152, i8 12) ;L539
 33468|  %154 = load i64, ptr %153, , !!8                                                                                      ;L542
 33469|  %155 = load i64, ptr %16, , !!8                                                                                       ;L542
 33470|     ;; v = !DIArgList(i64 %154, i64 %155)
 33472|     ;; self[8..+8] = i64 undef
 33473|     ;; self[16..+4] = i32 undef
 33474|     ;; self[20..+4] = i32 undef
 33475|     ;; f = ptr undef
 33476|     ;; best_pos[20..+4] = i32 0
 33477|     ;; best_pos[16..+4] = i32 0
 33478|     ;; best_pos[8..+8] = !DIArgList(i64 %154, i64 %155)
 33479|     ;; best_pos[0..+8] = i64 1
 33481|     ;; best_pos[20..+4] = i32 0
 33482|     ;; best_pos[16..+4] = i32 0
 33483|     ;; best_pos[8..+8] = !DIArgList(i64 %154, i64 %155)
 33484|     ;; best_pos[0..+8] = i64 1
 33485|     ;; start = i64 1
 33486|     ;; self = i64 1
 33487|     ;; self = i64 1
 33488|     ;; iter[0..+4] = i64 2
 33489|     ;; dy = i64 1
 33491|  %156 = add i64 %151, -2
 33492|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %16, i64 %1, ptr %3, ptr %4, ptr %5, i64 %149, i64 %156, i8 12) ;L539
 33493|  %157 = load i64, ptr %153, , !!8                                                                                      ;L542
 33494|  %158 = load i64, ptr %16, , !!8                                                                                       ;L542
 33495|     ;; v = !DIArgList(i64 %157, i64 %158)
 33497|     ;; self[8..+8] = !DIArgList(i64 %154, i64 %155)
 33498|     ;; self[16..+4] = i32 0
 33499|     ;; self[20..+4] = i32 0
 33500|     ;; best_pos[20..+4] = !DIArgList(i64 %154, i64 %157, i64 %158, i64 %155)
 33501|     ;; best_pos[16..+4] = i32 0
 33503|     ;; best_pos[0..+8] = i64 1
 33505|     ;; best_pos[20..+4] = !DIArgList(i64 %154, i64 %157, i64 %158, i64 %155)
 33506|     ;; best_pos[16..+4] = i32 0
 33508|     ;; best_pos[0..+8] = i64 1
 33509|     ;; start = i64 2
 33510|     ;; self = i64 2
 33511|     ;; self = i64 2
 33512|     ;; iter[0..+4] = i64 3
 33513|     ;; dy = i64 2
 33515|  %159 = add i64 %151, -1
 33516|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %16, i64 %1, ptr %3, ptr %4, ptr %5, i64 %149, i64 %159, i8 12) ;L539
 33517|  %160 = load i64, ptr %153, , !!8                                                                                      ;L542
 33518|  %161 = load i64, ptr %16, , !!8                                                                                       ;L542
 33519|     ;; v = !DIArgList(i64 %160, i64 %161)
 33522|     ;; self[16..+4] = i32 0
 33523|     ;; self[20..+4] = !DIArgList(i64 %154, i64 %157, i64 %158, i64 %155)
 33525|     ;; best_pos[16..+4] = i32 0
 33527|     ;; best_pos[0..+8] = i64 1
 33530|     ;; best_pos[16..+4] = i32 0
 33532|     ;; best_pos[0..+8] = i64 1
 33533|     ;; start = i64 3
 33534|     ;; self = i64 3
 33535|     ;; self = i64 3
 33536|     ;; iter[0..+4] = i64 4
 33537|     ;; dy = i64 3
 33539|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %16, i64 %1, ptr %3, ptr %4, ptr %5, i64 %149, i64 %151, i8 12) ;L539
 33540|  %162 = load i64, ptr %153, , !!8                                                                                      ;L542
 33541|  %163 = load i64, ptr %16, , !!8                                                                                       ;L542
 33542|     ;; v = !DIArgList(i64 %162, i64 %163)
 33545|     ;; self[16..+4] = i32 0
 33548|     ;; best_pos[16..+4] = i32 0
 33550|     ;; best_pos[0..+8] = i64 1
 33553|     ;; best_pos[16..+4] = i32 0
 33555|     ;; best_pos[0..+8] = i64 1
 33556|     ;; start = i64 4
 33557|     ;; self = i64 4
 33558|     ;; self = i64 4
 33559|     ;; iter[0..+4] = i64 5
 33560|     ;; dy = i64 4
 33562|  %164 = add i64 %151, 1
 33563|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %16, i64 %1, ptr %3, ptr %4, ptr %5, i64 %149, i64 %164, i8 12) ;L539
 33564|  %165 = load i64, ptr %153, , !!8                                                                                      ;L542
 33565|  %166 = load i64, ptr %16, , !!8                                                                                       ;L542
 33566|     ;; v = !DIArgList(i64 %165, i64 %166)
 33569|     ;; self[16..+4] = i32 0
 33572|     ;; best_pos[16..+4] = i32 0
 33574|     ;; best_pos[0..+8] = i64 1
 33577|     ;; best_pos[16..+4] = i32 0
 33579|     ;; best_pos[0..+8] = i64 1
 33580|     ;; start = i64 5
 33581|     ;; self = i64 5
 33582|     ;; self = i64 5
 33583|     ;; iter[0..+4] = i64 6
 33584|     ;; dy = i64 5
 33586|  %167 = add i64 %151, 2
 33587|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %16, i64 %1, ptr %3, ptr %4, ptr %5, i64 %149, i64 %167, i8 12) ;L539
 33588|  %168 = load i64, ptr %153, , !!8                                                                                      ;L542
 33589|  %169 = load i64, ptr %16, , !!8                                                                                       ;L542
 33590|     ;; v = !DIArgList(i64 %168, i64 %169)
 33593|     ;; self[16..+4] = i32 0
 33596|     ;; best_pos[16..+4] = i32 0
 33598|     ;; best_pos[0..+8] = i64 1
 33601|     ;; best_pos[16..+4] = i32 0
 33603|     ;; best_pos[0..+8] = i64 1
 33604|     ;; iter[0..+4] = i64 1
 33605|  %170 = sub i64 %154, %155                                                                                             ;L542
 33606|     ;; v = i64 %170
 33607|     ;; best_pos[8..+8] = i64 %170
 33608|     ;; self[8..+8] = i64 %170
 33609|     ;; best_pos[20..+4] = !DIArgList(i64 %170, i64 %157, i64 %158)
 33610|     ;; self[20..+4] = !DIArgList(i64 %170, i64 %157, i64 %158)
 33611|  %171 = sub i64 %157, %158                                                                                             ;L542
 33612|     ;; v = i64 %171
 33613|     ;; best_pos[20..+4] = !DIArgList(i64 %170, i64 %171)
 33614|     ;; self[20..+4] = !DIArgList(i64 %170, i64 %171)
 33615|  %172 = call i64 @llvm.smax.i64(i64 %170, i64 %171)                                                                    ;L708<544
 33616|     ;; best_pos[8..+8] = i64 %172
 33617|     ;; self[8..+8] = i64 %172
 33618|  %173 = sub i64 %160, %161                                                                                             ;L542
 33619|     ;; v = i64 %173
 33620|  %174 = call i64 @llvm.smax.i64(i64 %172, i64 %173)                                                                    ;L708<544
 33621|     ;; best_pos[8..+8] = i64 %174
 33622|     ;; self[8..+8] = i64 %174
 33623|  %175 = sub i64 %162, %163                                                                                             ;L542
 33624|     ;; v = i64 %175
 33625|  %176 = call i64 @llvm.smax.i64(i64 %174, i64 %175)                                                                    ;L708<544
 33626|     ;; best_pos[8..+8] = i64 %176
 33627|     ;; self[8..+8] = i64 %176
 33628|  %177 = sub i64 %165, %166                                                                                             ;L542
 33629|     ;; v = i64 %177
 33630|  %178 = call i64 @llvm.smax.i64(i64 %176, i64 %177)                                                                    ;L708<544
 33631|     ;; best_pos[8..+8] = i64 %178
 33632|     ;; self[8..+8] = i64 %178
 33633|  %179 = sub i64 %168, %169                                                                                             ;L542
 33634|     ;; v = i64 %179
 33635|  %180 = icmp slt i64 %178, %179
 33636|  %181 = icmp slt i64 %176, %177
 33637|  %182 = icmp slt i64 %174, %175
 33638|  %183 = icmp slt i64 %172, %173
 33639|  %184 = icmp slt i64 %170, %171
 33640|     ;; best_pos[20..+4] = i1 %184
 33641|     ;; self[20..+4] = i1 %184
 33642|  %185 = zext i1 %184 to i32                                                                                            ;L708<544
 33643|     ;; best_pos[20..+4] = i32 %185
 33644|     ;; self[20..+4] = i32 %185
 33645|  %186 = select i1 %183, i32 2, i32 %185                                                                                ;L708<544
 33646|     ;; best_pos[20..+4] = i32 %186
 33647|     ;; self[20..+4] = i32 %186
 33648|  %187 = select i1 %182, i32 3, i32 %186                                                                                ;L708<544
 33649|     ;; best_pos[20..+4] = i32 %187
 33650|     ;; self[20..+4] = i32 %187
 33651|  %188 = select i1 %181, i32 4, i32 %187                                                                                ;L708<544
 33652|     ;; best_pos[20..+4] = i32 %188
 33653|     ;; self[20..+4] = i32 %188
 33654|  %189 = select i1 %180, i32 5, i32 %188                                                                                ;L708<544
 33655|     ;; best_pos[20..+4] = i32 %189
 33656|  %190 = call i64 @llvm.smax.i64(i64 %178, i64 %179)                                                                    ;L708<544
 33657|     ;; best_pos[8..+8] = i64 %190
 33658|  br label %463                                                                                                         ;L900<985<537
 33659| 
 33660| 191: ; preds = %124
 33661|     ;; self = ptr %7
 33662|     ;; self = ptr %7
 33663|  %192 = load ptr, ptr %7, , !!8, !!8                                                                                   ;L441<2127<2445<379
 33664|  %193 = gep %7, i64 8                                                                                                  ;L441<2127<2445<379
 33665|  %194 = load ptr, ptr %193, , !!8, !!8                                                                                 ;L441<2127<2445<379
 33666|  %195 = gep %194, i64 16                                                                                               ;L2445<379
 33667|  %196 = load i64, ptr %195,                                                                                            ;L2445<379
 33668|  %197 = add nsw i64 %196, -1                                                                                           ;L2445<379
 33669|  %198 = and i64 %197, -16                                                                                              ;L2445<379
 33670|  %199 = gep %192, i64 %198                                                                                             ;L2445<379
 33671|  %200 = gep %199, i64 16                                                                                               ;L2445<379
 33672|  %201 = gep %194, i64 216                                                                                              ;L379
 33673|  %202 = load ptr, ptr %201, , !!8                                                                                      ;L379
 33674|  %203 = tail call zeroext i1 %202(ptr %200)                                                                            ;L379
 33675|  br i1 %203, label %204, label %531                                                                                    ;L379
 33676| 
 33677| 204: ; preds = %191
 33678|  %205 = gep %5, i64 2744
 33679|  %206 = load i64, ptr %205, , !!8
 33680|  %207 = add i64 %206, -3                                                                                               ;L900<985<382
 33681|  %208 = gep %5, i64 2752
 33682|  %209 = load i64, ptr %208, , !!8
 33683|  %210 = add i64 %209, -3
 33684|  %211 = gep %25, i64 16
 33685|     ;; self = ptr undef
 33686|     ;; self = ptr undef
 33687|     ;; self = ptr undef
 33688|     ;; other = ptr undef
 33689|     ;; old = i64 0
 33690|     ;; start = i64 0
 33691|     ;; self = i64 0
 33692|     ;; self = i64 0
 33693|     ;; iter[0..+4] = i64 0
 33694|     ;; dx = i64 0
 33695|     ;; iter[4..+4] = i32 6
 33697|     ;; best_pos[8..+8] = i64 undef
 33698|     ;; best_pos[16..+4] = i32 undef
 33699|     ;; best_pos[20..+4] = i32 undef
 33700|     ;; self = ptr undef
 33701|     ;; self = ptr undef
 33702|     ;; self = ptr undef
 33703|     ;; other = ptr undef
 33704|     ;; start = i64 0
 33705|     ;; self = i64 0
 33706|     ;; self = i64 0
 33707|     ;; iter[0..+4] = i64 1
 33708|     ;; dy = i64 0
 33710|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %25, i64 %1, ptr %3, ptr %4, ptr %5, i64 %207, i64 %210, i8 12) ;L384
 33711|  %212 = load i64, ptr %211, , !!8                                                                                      ;L387
 33712|  %213 = load i64, ptr %25, , !!8                                                                                       ;L387
 33713|     ;; v = !DIArgList(i64 %212, i64 %213)
 33715|     ;; self[8..+8] = i64 undef
 33716|     ;; self[16..+4] = i32 undef
 33717|     ;; self[20..+4] = i32 undef
 33718|     ;; f = ptr undef
 33719|     ;; best_pos[0..+8] = i64 1
 33720|     ;; best_pos[8..+8] = !DIArgList(i64 %212, i64 %213)
 33721|     ;; best_pos[16..+4] = i32 0
 33722|     ;; best_pos[20..+4] = i32 0
 33724|     ;; best_pos[0..+8] = i64 1
 33725|     ;; best_pos[8..+8] = !DIArgList(i64 %212, i64 %213)
 33726|     ;; best_pos[16..+4] = i32 0
 33727|     ;; best_pos[20..+4] = i32 0
 33728|     ;; start = i64 1
 33729|     ;; self = i64 1
 33730|     ;; self = i64 1
 33731|     ;; iter[0..+4] = i64 2
 33732|     ;; dy = i64 1
 33734|  %214 = add i64 %209, -2
 33735|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %25, i64 %1, ptr %3, ptr %4, ptr %5, i64 %207, i64 %214, i8 12) ;L384
 33736|  %215 = load i64, ptr %211, , !!8                                                                                      ;L387
 33737|  %216 = load i64, ptr %25, , !!8                                                                                       ;L387
 33738|     ;; v = !DIArgList(i64 %215, i64 %216)
 33740|     ;; self[8..+8] = !DIArgList(i64 %212, i64 %213)
 33741|     ;; self[16..+4] = i32 0
 33742|     ;; self[20..+4] = i32 0
 33743|     ;; best_pos[0..+8] = i64 1
 33745|     ;; best_pos[16..+4] = i32 0
 33746|     ;; best_pos[20..+4] = !DIArgList(i64 %212, i64 %215, i64 %216, i64 %213)
 33748|     ;; best_pos[0..+8] = i64 1
 33750|     ;; best_pos[16..+4] = i32 0
 33751|     ;; best_pos[20..+4] = !DIArgList(i64 %212, i64 %215, i64 %216, i64 %213)
 33752|     ;; start = i64 2
 33753|     ;; self = i64 2
 33754|     ;; self = i64 2
 33755|     ;; iter[0..+4] = i64 3
 33756|     ;; dy = i64 2
 33758|  %217 = add i64 %209, -1
 33759|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %25, i64 %1, ptr %3, ptr %4, ptr %5, i64 %207, i64 %217, i8 12) ;L384
 33760|  %218 = load i64, ptr %211, , !!8                                                                                      ;L387
 33761|  %219 = load i64, ptr %25, , !!8                                                                                       ;L387
 33762|     ;; v = !DIArgList(i64 %218, i64 %219)
 33765|     ;; self[16..+4] = i32 0
 33766|     ;; self[20..+4] = !DIArgList(i64 %212, i64 %215, i64 %216, i64 %213)
 33767|     ;; best_pos[0..+8] = i64 1
 33769|     ;; best_pos[16..+4] = i32 0
 33772|     ;; best_pos[0..+8] = i64 1
 33774|     ;; best_pos[16..+4] = i32 0
 33776|     ;; start = i64 3
 33777|     ;; self = i64 3
 33778|     ;; self = i64 3
 33779|     ;; iter[0..+4] = i64 4
 33780|     ;; dy = i64 3
 33782|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %25, i64 %1, ptr %3, ptr %4, ptr %5, i64 %207, i64 %209, i8 12) ;L384
 33783|  %220 = load i64, ptr %211, , !!8                                                                                      ;L387
 33784|  %221 = load i64, ptr %25, , !!8                                                                                       ;L387
 33785|     ;; v = !DIArgList(i64 %220, i64 %221)
 33788|     ;; self[16..+4] = i32 0
 33790|     ;; best_pos[0..+8] = i64 1
 33792|     ;; best_pos[16..+4] = i32 0
 33795|     ;; best_pos[0..+8] = i64 1
 33797|     ;; best_pos[16..+4] = i32 0
 33799|     ;; start = i64 4
 33800|     ;; self = i64 4
 33801|     ;; self = i64 4
 33802|     ;; iter[0..+4] = i64 5
 33803|     ;; dy = i64 4
 33805|  %222 = add i64 %209, 1
 33806|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %25, i64 %1, ptr %3, ptr %4, ptr %5, i64 %207, i64 %222, i8 12) ;L384
 33807|  %223 = load i64, ptr %211, , !!8                                                                                      ;L387
 33808|  %224 = load i64, ptr %25, , !!8                                                                                       ;L387
 33809|     ;; v = !DIArgList(i64 %223, i64 %224)
 33812|     ;; self[16..+4] = i32 0
 33814|     ;; best_pos[0..+8] = i64 1
 33816|     ;; best_pos[16..+4] = i32 0
 33819|     ;; best_pos[0..+8] = i64 1
 33821|     ;; best_pos[16..+4] = i32 0
 33823|     ;; start = i64 5
 33824|     ;; self = i64 5
 33825|     ;; self = i64 5
 33826|     ;; iter[0..+4] = i64 6
 33827|     ;; dy = i64 5
 33829|  %225 = add i64 %209, 2
 33830|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %25, i64 %1, ptr %3, ptr %4, ptr %5, i64 %207, i64 %225, i8 12) ;L384
 33831|  %226 = load i64, ptr %211, , !!8                                                                                      ;L387
 33832|  %227 = load i64, ptr %25, , !!8                                                                                       ;L387
 33833|     ;; v = !DIArgList(i64 %226, i64 %227)
 33836|     ;; self[16..+4] = i32 0
 33838|     ;; best_pos[0..+8] = i64 1
 33840|     ;; best_pos[16..+4] = i32 0
 33843|     ;; iter[0..+4] = i64 1
 33844|     ;; best_pos[0..+8] = i64 1
 33846|     ;; best_pos[16..+4] = i32 0
 33848|  %228 = sub i64 %212, %213                                                                                             ;L387
 33849|     ;; v = i64 %228
 33850|     ;; best_pos[8..+8] = i64 %228
 33851|     ;; self[8..+8] = i64 %228
 33852|     ;; best_pos[20..+4] = !DIArgList(i64 %228, i64 %215, i64 %216)
 33853|     ;; self[20..+4] = !DIArgList(i64 %228, i64 %215, i64 %216)
 33854|  %229 = sub i64 %215, %216                                                                                             ;L387
 33855|     ;; v = i64 %229
 33856|     ;; best_pos[20..+4] = !DIArgList(i64 %228, i64 %229)
 33857|     ;; self[20..+4] = !DIArgList(i64 %228, i64 %229)
 33858|  %230 = call i64 @llvm.smax.i64(i64 %228, i64 %229)                                                                    ;L708<389
 33859|     ;; best_pos[8..+8] = i64 %230
 33860|     ;; self[8..+8] = i64 %230
 33861|  %231 = sub i64 %218, %219                                                                                             ;L387
 33862|     ;; v = i64 %231
 33863|  %232 = call i64 @llvm.smax.i64(i64 %230, i64 %231)                                                                    ;L708<389
 33864|     ;; best_pos[8..+8] = i64 %232
 33865|     ;; self[8..+8] = i64 %232
 33866|  %233 = sub i64 %220, %221                                                                                             ;L387
 33867|     ;; v = i64 %233
 33868|  %234 = call i64 @llvm.smax.i64(i64 %232, i64 %233)                                                                    ;L708<389
 33869|     ;; best_pos[8..+8] = i64 %234
 33870|     ;; self[8..+8] = i64 %234
 33871|  %235 = sub i64 %223, %224                                                                                             ;L387
 33872|     ;; v = i64 %235
 33873|  %236 = call i64 @llvm.smax.i64(i64 %234, i64 %235)                                                                    ;L708<389
 33874|     ;; best_pos[8..+8] = i64 %236
 33875|     ;; self[8..+8] = i64 %236
 33876|  %237 = sub i64 %226, %227                                                                                             ;L387
 33877|     ;; v = i64 %237
 33878|  %238 = icmp slt i64 %236, %237
 33879|  %239 = call i64 @llvm.smax.i64(i64 %236, i64 %237)                                                                    ;L708<389
 33880|     ;; best_pos[8..+8] = i64 %239
 33881|  %240 = icmp slt i64 %234, %235
 33882|  %241 = icmp slt i64 %232, %233
 33883|  %242 = icmp slt i64 %230, %231
 33884|  %243 = icmp slt i64 %228, %229
 33885|     ;; best_pos[20..+4] = i1 %243
 33886|     ;; self[20..+4] = i1 %243
 33887|  %244 = zext i1 %243 to i32                                                                                            ;L708<389
 33888|     ;; best_pos[20..+4] = i32 %244
 33889|     ;; self[20..+4] = i32 %244
 33890|  %245 = select i1 %242, i32 2, i32 %244                                                                                ;L708<389
 33891|     ;; best_pos[20..+4] = i32 %245
 33892|     ;; self[20..+4] = i32 %245
 33893|  %246 = select i1 %241, i32 3, i32 %245                                                                                ;L708<389
 33894|     ;; best_pos[20..+4] = i32 %246
 33895|     ;; self[20..+4] = i32 %246
 33896|  %247 = select i1 %240, i32 4, i32 %246                                                                                ;L708<389
 33897|     ;; best_pos[20..+4] = i32 %247
 33898|     ;; self[20..+4] = i32 %247
 33899|  %248 = select i1 %238, i32 5, i32 %247                                                                                ;L708<389
 33900|     ;; best_pos[20..+4] = i32 %248
 33901|  br label %782                                                                                                         ;L900<985<382
 33902| 
 33903| 249: ; preds = %124
 33904|  store i32 3, ptr %0,                                                                                                  ;L653
 33905|  br label %44                                                                                                          ;L653
 33906| 
 33907| 250: ; preds = %133
 33908|  %251 = gep %6, i64 104                                                                                                ;L556
 33909|  %252 = load i64, ptr %251, , !!8                                                                                      ;L556
 33910|  %253 = icmp eq i64 %252, 13                                                                                           ;L556
 33911|     ;; c = ptr %6
 33912|  %254 = gep %6, i64 112
 33913|  %255 = load i64, ptr %254,
 33914|  %256 = icmp eq i64 %255, 2
 33915|  %257 = select i1 %253, i1 %256, i1 false                                                                              ;L556
 33916|  br i1 %257, label %264, label %258                                                                                    ;L556
 33917| 
 33918| 258: ; preds = %250
 33919|  %259 = gep %6, i64 776                                                                                                ;L592
 33920|  %260 = load i64, ptr %259, , !!8                                                                                      ;L592
 33921|  %261 = xor i64 %260, -9223372036854775808                                                                             ;L592
 33922|  %262 = icmp slt i64 %260, 0                                                                                           ;L592
 33923|  %263 = select i1 %262, i64 %261, i64 4                                                                                ;L592
 33924|  switch i64 %263, label %328 [
 33925|  i64 1, label %345
 33926|  i64 3, label %343
 33927|  i64 4, label %344
 33928|  ]                                                                                                                     ;L592
 33929| 
 33930| 264: ; preds = %250
 33931|  %265 = gep %6, i64 120                                                                                                ;L559
 33932|  %266 = load i64, ptr %265, , !!8                                                                                      ;L559
 33933|     ;; x = i64 %266
 33934|  %267 = gep %6, i64 128                                                                                                ;L559
 33935|  %268 = load i64, ptr %267, , !!8                                                                                      ;L559
 33936|     ;; y = i64 %268
 33938|  store i64 0, ptr %15,                                                                                                 ;L391<562
 33939|  %269 = gep %15, i64 8                                                                                                 ;L391<562
 33940|  store i64 1000, ptr %269,                                                                                             ;L391<562
 33941|  %270 = gep %15, i64 16                                                                                                ;L391<562
 33942|  store i8 0, ptr %270,                                                                                                 ;L391<562
 33943|  %271 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %2, ptr %15)                            ;L562
 33945|  %272 = icmp ult i64 %271, %68                                                                                         ;L562
 33946|  br i1 %272, label %273, label %279                                                                                    ;L562
 33947| 
 33948| 273: ; preds = %264
 33949|  %274 = tail call { i64, ptr } %88(ptr %84)                                                                            ;L565
 33950|  %275 = extractvalue { i64, ptr } %274, 0                                                                              ;L565
 33951|  %276 = icmp eq i64 %275, 2                                                                                            ;L565
 33952|  %277 = icmp ugt i64 %67, 39                                                                                           ;L565
 33953|  %278 = and i1 %277, %276                                                                                              ;L565
 33954|  br i1 %278, label %323, label %305                                                                                    ;L565
 33955| 
 33956| 279: ; preds = %323, %305, %264
 33957|  %280 = phi i64 [ %325, %323 ], [ %321, %305 ], [ %266, %264 ]                                                         ;L0
 33958|  %281 = phi i64 [ %327, %323 ], [ %322, %305 ], [ %268, %264 ]                                                         ;L0
 33959|     ;; x = i64 %280
 33960|     ;; y = i64 %281
 33962|  %282 = gep %6, i64 1632                                                                                               ;L577
 33963|  %283 = load i64, ptr %282, , !!8                                                                                      ;L577
 33964|  store i64 %283, ptr %14,                                                                                              ;L577
 33966|  %284 = gep %6, i64 1640                                                                                               ;L578
 33967|  %285 = load i64, ptr %284, , !!8                                                                                      ;L578
 33968|  store i64 %285, ptr %13,                                                                                              ;L578
 33969|  %286 = gep %4, i64 8                                                                                                  ;L579
 33970|  %287 = load ptr, ptr %286, , !!8, !!8                                                                                 ;L579
 33971|  %288 = gep %287, i64 8                                                                                                ;L579
 33972|  %289 = load ptr, ptr %288, , !!8, !!8                                                                                 ;L579
 33973|  %290 = gep %287, i64 24                                                                                               ;L579
 33974|  %291 = load ptr, ptr %290, , !!8, !!8                                                                                 ;L579
 33975|  %292 = gep %287, i64 32                                                                                               ;L579
 33976|  %293 = load ptr, ptr %292, , !!8, !!8                                                                                 ;L579
 33977|  %294 = gep %6, i64 1472                                                                                               ;L579
 33978|  %295 = load i64, ptr %294, , !!8                                                                                      ;L579
 33979|  %296 = gep %6, i64 1600                                                                                               ;L580
 33980|  %297 = load i64, ptr %296, , !!8                                                                                      ;L580
 33981|  %298 = mul i64 %297, %125                                                                                             ;L580
 33983|  store ptr null, ptr %12,                                                                                              ;L580
 33984|  call void @gc::simulation6entityNtB5_6Entity7move_to(ptr %289, ptr %291, ptr %293, i64 %295, ptr %14, ptr %13, i64 %298, i64 %280, i64 %281, ptr %12) ;L579
 33986|  %299 = load i64, ptr %14, , !!8                                                                                       ;L582
 33987|  %300 = load i64, ptr %13, , !!8                                                                                       ;L582
 33988|  %301 = gep %39, i64 1632                                                                                              ;L582
 33989|  %302 = load i64, ptr %301,                                                                                            ;L582
 33990|  %303 = gep %39, i64 1640                                                                                              ;L582
 33991|  %304 = load i64, ptr %303,                                                                                            ;L582
 33992|  call fastcc void @ai::abstract_input20apply_aim_offset_pos(ptr %0, ptr %2, ptr %289, ptr %293, i64 %66, i64 %302, i64 %304, i64 %299, i64 %300) ;L582
 33995|  br label %44                                                                                                          ;L1
 33996| 
 33997| 305: ; preds = %273
 33998|  %306 = gep %39, i64 1632                                                                                              ;L568
 33999|  %307 = load i64, ptr %306, , !!8                                                                                      ;L568
 34000|     ;; dir_x = !DIArgList(i64 %266, i64 %307)
 34001|  %308 = gep %39, i64 1640                                                                                              ;L569
 34002|  %309 = load i64, ptr %308, , !!8                                                                                      ;L569
 34003|     ;; dir_y = !DIArgList(i64 %268, i64 %309)
 34004|  %310 = gep %4, i64 8                                                                                                  ;L570
 34005|  %311 = load ptr, ptr %310, , !!8, !!8                                                                                 ;L570
 34006|  %312 = gep %311, i64 32                                                                                               ;L570
 34007|  %313 = load ptr, ptr %312, , !!8, !!8                                                                                 ;L570
 34008|  %314 = gep %311, i64 8                                                                                                ;L570
 34009|  %315 = load ptr, ptr %314, , !!8, !!8                                                                                 ;L570
 34010|  %316 = shl i64 %307, 1                                                                                                ;L571
 34011|  %317 = sub i64 %316, %266                                                                                             ;L571
 34012|  %318 = shl i64 %309, 1                                                                                                ;L571
 34013|  %319 = sub i64 %318, %268                                                                                             ;L571
 34014|  %320 = tail call { i64, i64 } @gc::simulation4gameNtB5_4Game15adjust_position(ptr %313, ptr %315, i64 %317, i64 %319) ;L570
 34015|  %321 = extractvalue { i64, i64 } %320, 0                                                                              ;L570
 34016|  %322 = extractvalue { i64, i64 } %320, 1                                                                              ;L570
 34017|  br label %279                                                                                                         ;L565
 34018| 
 34019| 323: ; preds = %273
 34020|  %324 = gep %6, i64 1632                                                                                               ;L566
 34021|  %325 = load i64, ptr %324, , !!8                                                                                      ;L566
 34022|  %326 = gep %6, i64 1640                                                                                               ;L566
 34023|  %327 = load i64, ptr %326, , !!8                                                                                      ;L566
 34024|  br label %279                                                                                                         ;L565
 34025| 
 34026| 328: ; preds = %258
 34027|     ;; self = ptr %7
 34028|  %329 = gep %6, i64 1632                                                                                               ;L158<645
 34029|  %330 = load i64, ptr %329, , !!8                                                                                      ;L158<645
 34030|  %331 = gep %6, i64 1640                                                                                               ;L158<645
 34031|  %332 = load i64, ptr %331, , !!8                                                                                      ;L158<645
 34032|     ;; input[8..+8] = i64 %330
 34033|     ;; input[16..+8] = i64 %332
 34034|     ;; input[0..+4] = i32 2
 34035|     ;; x = i64 %330
 34036|     ;; y = i64 %332
 34037|  %333 = gep %4, i64 8                                                                                                  ;L647
 34038|  %334 = load ptr, ptr %333, , !!8, !!8                                                                                 ;L647
 34039|  %335 = gep %334, i64 8                                                                                                ;L647
 34040|  %336 = load ptr, ptr %335, , !!8, !!8                                                                                 ;L647
 34041|  %337 = gep %334, i64 32                                                                                               ;L647
 34042|  %338 = load ptr, ptr %337, , !!8, !!8                                                                                 ;L647
 34043|  %339 = gep %39, i64 1632                                                                                              ;L647
 34044|  %340 = load i64, ptr %339,                                                                                            ;L647
 34045|  %341 = gep %39, i64 1640                                                                                              ;L647
 34046|  %342 = load i64, ptr %341,                                                                                            ;L647
 34047|  call fastcc void @ai::abstract_input20apply_aim_offset_pos(ptr %0, ptr %2, ptr %336, ptr %338, i64 %66, i64 %340, i64 %342, i64 %330, i64 %332) ;L647
 34048|  br label %44                                                                                                          ;L650
 34049| 
 34050| 343: ; preds = %258
 34054|  br label %345                                                                                                         ;L592
 34055| 
 34056| 344: ; preds = %258
 34060|  br label %345                                                                                                         ;L592
 34061| 
 34062| 345: ; preds = %344, %343, %258
 34063|  %346 = phi i64 [ 824, %344 ], [ 808, %343 ], [ 832, %258 ]
 34064|  %347 = phi i64 [ 832, %344 ], [ 816, %343 ], [ 840, %258 ]
 34065|  %348 = phi i64 [ 840, %344 ], [ 824, %343 ], [ 848, %258 ]
 34066|  %349 = gep %6, i64 %346                                                                                               ;L593
 34067|  %350 = gep %6, i64 %347                                                                                               ;L593
 34068|  %351 = gep %6, i64 %348                                                                                               ;L593
 34069|  %352 = load i64, ptr %351, , !!8                                                                                      ;L593
 34070|  %353 = load i64, ptr %350, , !!8                                                                                      ;L593
 34071|  %354 = load i64, ptr %349, , !!8                                                                                      ;L593
 34072|     ;; y = i64 %352
 34073|     ;; x = i64 %353
 34074|     ;; speed = i64 %354
 34075|     ;; rush_real_dest[0..+8] = i64 %353
 34076|     ;; rush_real_dest[8..+8] = i64 %352
 34078|  store i64 0, ptr %11,                                                                                                 ;L391<596
 34079|  %355 = gep %11, i64 8                                                                                                 ;L391<596
 34080|  store i64 1000, ptr %355,                                                                                             ;L391<596
 34081|  %356 = gep %11, i64 16                                                                                                ;L391<596
 34082|  store i8 0, ptr %356,                                                                                                 ;L391<596
 34083|  %357 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %2, ptr %11)                            ;L596
 34085|  %358 = icmp ult i64 %357, %68                                                                                         ;L596
 34086|     ;; rush_misjudged = i1 %358
 34087|  br i1 %358, label %359, label %365                                                                                    ;L597
 34088| 
 34089| 359: ; preds = %345
 34090|  %360 = tail call { i64, ptr } %88(ptr %84)                                                                            ;L599
 34091|  %361 = extractvalue { i64, ptr } %360, 0                                                                              ;L599
 34092|  %362 = icmp eq i64 %361, 2                                                                                            ;L599
 34093|  %363 = icmp ugt i64 %67, 39                                                                                           ;L599
 34094|  %364 = and i1 %363, %362                                                                                              ;L599
 34095|  br i1 %364, label %393, label %375                                                                                    ;L599
 34096| 
 34097| 365: ; preds = %393, %375, %345
 34098|  %366 = phi i64 [ %395, %393 ], [ %391, %375 ], [ %353, %345 ]                                                         ;L0
 34099|  %367 = phi i64 [ %397, %393 ], [ %392, %375 ], [ %352, %345 ]                                                         ;L0
 34100|     ;; x = i64 %366
 34101|     ;; target_x = i64 %366
 34102|     ;; y = i64 %367
 34103|     ;; target_y = i64 %367
 34104|  %368 = tail call zeroext i1 @gc::simulation6entity11nt_trace_on()                                                     ;L610
 34105|  %369 = and i1 %253, %368                                                                                              ;L610
 34106|  br i1 %369, label %403, label %370                                                                                    ;L610
 34107| 
 34108| 370: ; preds = %365
 34109|  %371 = gep %6, i64 1632
 34110|  %372 = load i64, ptr %371,                                                                                            ;L624
 34111|  %373 = gep %6, i64 1640
 34112|  %374 = load i64, ptr %373,                                                                                            ;L624
 34113|  br label %398                                                                                                         ;L610
 34114| 
 34115| 375: ; preds = %359
 34116|  %376 = gep %39, i64 1632                                                                                              ;L602
 34117|  %377 = load i64, ptr %376, , !!8                                                                                      ;L602
 34118|     ;; dir_x = !DIArgList(i64 %353, i64 %377)
 34119|  %378 = gep %39, i64 1640                                                                                              ;L603
 34120|  %379 = load i64, ptr %378, , !!8                                                                                      ;L603
 34121|     ;; dir_y = !DIArgList(i64 %352, i64 %379)
 34122|  %380 = gep %4, i64 8                                                                                                  ;L604
 34123|  %381 = load ptr, ptr %380, , !!8, !!8                                                                                 ;L604
 34124|  %382 = gep %381, i64 32                                                                                               ;L604
 34125|  %383 = load ptr, ptr %382, , !!8, !!8                                                                                 ;L604
 34126|  %384 = gep %381, i64 8                                                                                                ;L604
 34127|  %385 = load ptr, ptr %384, , !!8, !!8                                                                                 ;L604
 34128|  %386 = shl i64 %377, 1                                                                                                ;L604
 34129|  %387 = sub i64 %386, %353                                                                                             ;L604
 34130|  %388 = shl i64 %379, 1                                                                                                ;L604
 34131|  %389 = sub i64 %388, %352                                                                                             ;L604
 34132|  %390 = tail call { i64, i64 } @gc::simulation4gameNtB5_4Game15adjust_position(ptr %383, ptr %385, i64 %387, i64 %389) ;L604
 34133|  %391 = extractvalue { i64, i64 } %390, 0                                                                              ;L604
 34134|  %392 = extractvalue { i64, i64 } %390, 1                                                                              ;L604
 34135|  br label %365                                                                                                         ;L599
 34136| 
 34137| 393: ; preds = %359
 34138|  %394 = gep %6, i64 1632                                                                                               ;L600
 34139|  %395 = load i64, ptr %394, , !!8                                                                                      ;L600
 34140|  %396 = gep %6, i64 1640                                                                                               ;L600
 34141|  %397 = load i64, ptr %396, , !!8                                                                                      ;L600
 34142|  br label %365                                                                                                         ;L599
 34143| 
 34144| 398: ; preds = %403, %370
 34145|  %399 = phi i64 [ %374, %370 ], [ %414, %403 ]                                                                         ;L624
 34146|  %400 = phi i64 [ %372, %370 ], [ %412, %403 ]                                                                         ;L624
 34147|  %401 = call i64 @gc::utils8distance(i64 %366, i64 %367, i64 %400, i64 %399)                                           ;L624
 34148|     ;; dist = i64 %401
 34149|  %402 = icmp eq i64 %354, 0                                                                                            ;L625
 34150|  br i1 %402, label %431, label %428                                                                                    ;L625
 34151| 
 34152| 403: ; preds = %365
 34153|  %404 = gep %39, i64 1472                                                                                              ;L611
 34154|  %405 = load i64, ptr %404, , !!8                                                                                      ;L611
 34156|  %406 = gep %86, i64 40                                                                                                ;L612
 34157|  %407 = load ptr, ptr %406, , !!8                                                                                      ;L612
 34158|  %408 = tail call i64 %407(ptr %84)                                                                                    ;L612
 34159|  %409 = gep %6, i64 1472                                                                                               ;L613
 34160|  %410 = load i64, ptr %409, , !!8                                                                                      ;L613
 34161|  %411 = gep %6, i64 1632                                                                                               ;L614
 34162|  %412 = load i64, ptr %411, , !!8                                                                                      ;L614
 34163|  %413 = gep %6, i64 1640                                                                                               ;L615
 34164|  %414 = load i64, ptr %413, , !!8                                                                                      ;L615
 34165|  %415 = gep %3, i64 264                                                                                                ;L619
 34166|  %416 = load i64, ptr %415, , !!8                                                                                      ;L619
 34167|  %417 = gep %10, i64 24                                                                                                ;L611
 34168|  store i64 %408, ptr %417,                                                                                             ;L611
 34169|  %418 = gep %10, i64 32                                                                                                ;L611
 34170|  store i64 %410, ptr %418,                                                                                             ;L611
 34171|  %419 = gep %10, i64 40                                                                                                ;L611
 34172|  store i64 %412, ptr %419,                                                                                             ;L611
 34173|  %420 = gep %10, i64 48                                                                                                ;L611
 34174|  store i64 %414, ptr %420,                                                                                             ;L611
 34175|  store i64 1, ptr %10,                                                                                                 ;L611
 34176|  %421 = gep %10, i64 8                                                                                                 ;L611
 34177|  store i64 %353, ptr %421,                                                                                             ;L611
 34178|  %422 = gep %10, i64 16                                                                                                ;L611
 34179|  store i64 %352, ptr %422,                                                                                             ;L611
 34180|  %423 = gep %10, i64 80                                                                                                ;L611
 34181|  %424 = zext i1 %358 to i8                                                                                             ;L611
 34182|  store i8 %424, ptr %423,                                                                                              ;L611
 34183|  %425 = gep %10, i64 56                                                                                                ;L611
 34184|  store i64 %125, ptr %425,                                                                                             ;L611
 34185|  %426 = gep %10, i64 64                                                                                                ;L611
 34186|  store i64 %416, ptr %426,                                                                                             ;L611
 34187|  %427 = gep %10, i64 72                                                                                                ;L611
 34188|  store i64 %66, ptr %427,                                                                                              ;L611
 34189|  call void @gc::simulation6entity19nt_trace_record_aim(i64 %405, ptr %10)                                              ;L611
 34191|  br label %398                                                                                                         ;L610
 34192| 
 34193| 428: ; preds = %398
 34194|  %429 = udiv i64 %401, %354                                                                                            ;L625
 34195|     ;; tick = i64 %429
 34196|  %430 = icmp ult i64 %429, %125                                                                                        ;L626
 34197|  br i1 %430, label %432, label %450                                                                                    ;L626
 34198| 
 34199| 431: ; preds = %398
 34200|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.168add0ea037d45d276f5936ae758fe5.139) #30  ;L625
 34201|  unreachable                                                                                                           ;L625
 34202| 
 34203| 432: ; preds = %428
 34204|  %433 = gep %39, i64 1632                                                                                              ;L629
 34205|  %434 = load i64, ptr %433, , !!8                                                                                      ;L629
 34206|  %435 = sub i64 %366, %434                                                                                             ;L629
 34207|     ;; dir_x = i64 %435
 34208|  %436 = gep %39, i64 1640                                                                                              ;L630
 34209|  %437 = load i64, ptr %436, , !!8                                                                                      ;L630
 34210|  %438 = sub i64 %367, %437                                                                                             ;L630
 34211|     ;; dir_y = i64 %438
 34212|  %439 = mul i64 %435, %435                                                                                             ;L631
 34213|  %440 = mul i64 %438, %438                                                                                             ;L631
 34214|  %441 = add i64 %440, %439                                                                                             ;L631
 34215|  %442 = call i64 @gc::utils5isqrt(i64 %441)                                                                            ;L631
 34216|     ;; self = i64 %442
 34217|     ;; other = i64 1
 34218|  %443 = call i64 @llvm.smax.i64(i64 %442, i64 1)                                                                       ;L1039<631
 34219|     ;; sz = i64 %443
 34220|  %444 = mul i64 %435, %125                                                                                             ;L633
 34221|  %445 = sdiv i64 %444, %443                                                                                            ;L633
 34222|  %446 = add i64 %445, %434                                                                                             ;L633
 34223|     ;; nx = i64 %446
 34224|  %447 = mul i64 %438, %125                                                                                             ;L634
 34225|  %448 = sdiv i64 %447, %443                                                                                            ;L634
 34226|  %449 = add i64 %448, %437                                                                                             ;L634
 34227|     ;; x = i64 %446
 34228|     ;; target_x = i64 %446
 34229|     ;; y = i64 %449
 34230|     ;; target_y = i64 %449
 34231|  br label %450                                                                                                         ;L626
 34232| 
 34233| 450: ; preds = %432, %428
 34234|  %451 = phi i64 [ %366, %428 ], [ %446, %432 ]                                                                         ;L0
 34235|  %452 = phi i64 [ %367, %428 ], [ %449, %432 ]                                                                         ;L0
 34236|     ;; target_y = i64 %452
 34237|     ;; y = i64 %452
 34238|     ;; target_x = i64 %451
 34239|     ;; x = i64 %451
 34240|  %453 = gep %4, i64 8                                                                                                  ;L638
 34241|  %454 = load ptr, ptr %453, , !!8, !!8                                                                                 ;L638
 34242|  %455 = gep %454, i64 8                                                                                                ;L638
 34243|  %456 = load ptr, ptr %455, , !!8, !!8                                                                                 ;L638
 34244|  %457 = gep %454, i64 32                                                                                               ;L638
 34245|  %458 = load ptr, ptr %457, , !!8, !!8                                                                                 ;L638
 34246|  %459 = gep %39, i64 1632                                                                                              ;L638
 34247|  %460 = load i64, ptr %459,                                                                                            ;L638
 34248|  %461 = gep %39, i64 1640                                                                                              ;L638
 34249|  %462 = load i64, ptr %461,                                                                                            ;L638
 34250|  call fastcc void @ai::abstract_input20apply_aim_offset_pos(ptr %0, ptr %2, ptr %456, ptr %458, i64 %66, i64 %460, i64 %462, i64 %451, i64 %452) ;L638
 34251|  br label %44                                                                                                          ;L1
 34252| 
 34253| 463: ; preds = %463, %146
 34254|  %464 = phi i64 [ 1, %146 ], [ %512, %463 ]
 34255|  %465 = phi i32 [ %189, %146 ], [ %511, %463 ]
 34256|  %466 = phi i32 [ 0, %146 ], [ %510, %463 ]
 34257|  %467 = phi i64 [ %190, %146 ], [ %506, %463 ]
 34258|     ;; old = i64 %464
 34259|     ;; start = i64 %464
 34260|     ;; self = i64 %464
 34261|     ;; self = i64 %464
 34262|     ;; iter[0..+4] = i64 %464
 34263|     ;; dx = i64 %464
 34264|     ;; iter[0..+4] = i32 0
 34265|     ;; iter[4..+4] = i32 6
 34266|  %468 = add i64 %149, %464
 34267|     ;; best_pos[20..+4] = i32 %465
 34268|     ;; best_pos[16..+4] = i32 %466
 34269|     ;; best_pos[8..+8] = i64 %467
 34271|     ;; self = ptr undef
 34272|     ;; self = ptr undef
 34273|     ;; self = ptr undef
 34274|     ;; other = ptr undef
 34275|     ;; start = i64 0
 34276|     ;; self = i64 0
 34277|     ;; self = i64 0
 34278|     ;; iter[0..+4] = i64 1
 34279|     ;; dy = i64 0
 34281|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %16, i64 %1, ptr %3, ptr %4, ptr %5, i64 %468, i64 %152, i8 12) ;L539
 34282|  %469 = load i64, ptr %153, , !!8                                                                                      ;L542
 34283|  %470 = load i64, ptr %16, , !!8                                                                                       ;L542
 34284|  %471 = sub i64 %469, %470                                                                                             ;L542
 34285|     ;; v = i64 %471
 34287|     ;; self[8..+8] = i64 %467
 34288|     ;; self[16..+4] = i32 %466
 34289|     ;; self[20..+4] = i32 %465
 34290|     ;; f = ptr undef
 34291|  %472 = icmp slt i64 %467, %471
 34292|  %473 = call i64 @llvm.smax.i64(i64 %467, i64 %471)                                                                    ;L708<544
 34293|  %474 = trunc nuw nsw i64 %464 to i32                                                                                  ;L708<544
 34294|  %475 = select i1 %472, i32 0, i32 %465                                                                                ;L708<544
 34295|     ;; best_pos[20..+4] = i32 %475
 34297|     ;; best_pos[8..+8] = i64 %473
 34298|     ;; best_pos[0..+8] = i64 1
 34300|     ;; best_pos[20..+4] = i32 %475
 34302|     ;; best_pos[8..+8] = i64 %473
 34303|     ;; best_pos[0..+8] = i64 1
 34304|     ;; start = i64 1
 34305|     ;; self = i64 1
 34306|     ;; self = i64 1
 34307|     ;; iter[0..+4] = i64 2
 34308|     ;; dy = i64 1
 34310|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %16, i64 %1, ptr %3, ptr %4, ptr %5, i64 %468, i64 %156, i8 12) ;L539
 34311|  %476 = load i64, ptr %153, , !!8                                                                                      ;L542
 34312|  %477 = load i64, ptr %16, , !!8                                                                                       ;L542
 34313|  %478 = sub i64 %476, %477                                                                                             ;L542
 34314|     ;; v = i64 %478
 34316|     ;; self[8..+8] = i64 %473
 34318|     ;; self[20..+4] = i32 %475
 34319|  %479 = icmp slt i64 %473, %478
 34320|  %480 = call i64 @llvm.smax.i64(i64 %473, i64 %478)                                                                    ;L708<544
 34321|  %481 = or i1 %479, %472                                                                                               ;L708<544
 34322|  %482 = select i1 %479, i32 1, i32 %475                                                                                ;L708<544
 34323|     ;; best_pos[20..+4] = i32 %482
 34325|     ;; best_pos[8..+8] = i64 %480
 34326|     ;; best_pos[0..+8] = i64 1
 34328|     ;; best_pos[20..+4] = i32 %482
 34330|     ;; best_pos[8..+8] = i64 %480
 34331|     ;; best_pos[0..+8] = i64 1
 34332|     ;; start = i64 2
 34333|     ;; self = i64 2
 34334|     ;; self = i64 2
 34335|     ;; iter[0..+4] = i64 3
 34336|     ;; dy = i64 2
 34338|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %16, i64 %1, ptr %3, ptr %4, ptr %5, i64 %468, i64 %159, i8 12) ;L539
 34339|  %483 = load i64, ptr %153, , !!8                                                                                      ;L542
 34340|  %484 = load i64, ptr %16, , !!8                                                                                       ;L542
 34341|  %485 = sub i64 %483, %484                                                                                             ;L542
 34342|     ;; v = i64 %485
 34344|     ;; self[8..+8] = i64 %480
 34346|     ;; self[20..+4] = i32 %482
 34347|  %486 = icmp slt i64 %480, %485
 34348|  %487 = call i64 @llvm.smax.i64(i64 %480, i64 %485)                                                                    ;L708<544
 34349|  %488 = select i1 %486, i32 2, i32 %482                                                                                ;L708<544
 34350|     ;; best_pos[20..+4] = i32 %488
 34352|     ;; best_pos[8..+8] = i64 %487
 34353|     ;; best_pos[0..+8] = i64 1
 34355|     ;; best_pos[20..+4] = i32 %488
 34357|     ;; best_pos[8..+8] = i64 %487
 34358|     ;; best_pos[0..+8] = i64 1
 34359|     ;; start = i64 3
 34360|     ;; self = i64 3
 34361|     ;; self = i64 3
 34362|     ;; iter[0..+4] = i64 4
 34363|     ;; dy = i64 3
 34365|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %16, i64 %1, ptr %3, ptr %4, ptr %5, i64 %468, i64 %151, i8 12) ;L539
 34366|  %489 = load i64, ptr %153, , !!8                                                                                      ;L542
 34367|  %490 = load i64, ptr %16, , !!8                                                                                       ;L542
 34368|  %491 = sub i64 %489, %490                                                                                             ;L542
 34369|     ;; v = i64 %491
 34371|     ;; self[8..+8] = i64 %487
 34373|     ;; self[20..+4] = i32 %488
 34374|  %492 = icmp slt i64 %487, %491
 34375|  %493 = call i64 @llvm.smax.i64(i64 %487, i64 %491)                                                                    ;L708<544
 34376|  %494 = or i1 %492, %486                                                                                               ;L708<544
 34377|  %495 = select i1 %492, i32 3, i32 %488                                                                                ;L708<544
 34378|     ;; best_pos[20..+4] = i32 %495
 34380|     ;; best_pos[8..+8] = i64 %493
 34381|     ;; best_pos[0..+8] = i64 1
 34383|     ;; best_pos[20..+4] = i32 %495
 34385|     ;; best_pos[8..+8] = i64 %493
 34386|     ;; best_pos[0..+8] = i64 1
 34387|     ;; start = i64 4
 34388|     ;; self = i64 4
 34389|     ;; self = i64 4
 34390|     ;; iter[0..+4] = i64 5
 34391|     ;; dy = i64 4
 34393|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %16, i64 %1, ptr %3, ptr %4, ptr %5, i64 %468, i64 %164, i8 12) ;L539
 34394|  %496 = load i64, ptr %153, , !!8                                                                                      ;L542
 34395|  %497 = load i64, ptr %16, , !!8                                                                                       ;L542
 34396|  %498 = sub i64 %496, %497                                                                                             ;L542
 34397|     ;; v = i64 %498
 34399|     ;; self[8..+8] = i64 %493
 34401|     ;; self[20..+4] = i32 %495
 34402|  %499 = icmp slt i64 %493, %498
 34403|  %500 = call i64 @llvm.smax.i64(i64 %493, i64 %498)                                                                    ;L708<544
 34404|  %501 = select i1 %499, i32 4, i32 %495                                                                                ;L708<544
 34405|     ;; best_pos[20..+4] = i32 %501
 34407|     ;; best_pos[8..+8] = i64 %500
 34408|     ;; best_pos[0..+8] = i64 1
 34410|     ;; best_pos[20..+4] = i32 %501
 34412|     ;; best_pos[8..+8] = i64 %500
 34413|     ;; best_pos[0..+8] = i64 1
 34414|     ;; start = i64 5
 34415|     ;; self = i64 5
 34416|     ;; self = i64 5
 34417|     ;; iter[0..+4] = i64 6
 34418|     ;; dy = i64 5
 34420|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %16, i64 %1, ptr %3, ptr %4, ptr %5, i64 %468, i64 %167, i8 12) ;L539
 34421|  %502 = load i64, ptr %153, , !!8                                                                                      ;L542
 34422|  %503 = load i64, ptr %16, , !!8                                                                                       ;L542
 34423|  %504 = sub i64 %502, %503                                                                                             ;L542
 34424|     ;; v = i64 %504
 34426|     ;; self[8..+8] = i64 %500
 34428|     ;; self[20..+4] = i32 %501
 34429|  %505 = icmp slt i64 %500, %504
 34430|  %506 = call i64 @llvm.smax.i64(i64 %500, i64 %504)                                                                    ;L708<544
 34431|  %507 = or i1 %505, %499                                                                                               ;L708<544
 34432|  %508 = select i1 %507, i1 true, i1 %494                                                                               ;L708<544
 34433|  %509 = select i1 %508, i1 true, i1 %481                                                                               ;L708<544
 34434|  %510 = select i1 %509, i32 %474, i32 %466                                                                             ;L708<544
 34435|  %511 = select i1 %505, i32 5, i32 %501                                                                                ;L708<544
 34436|     ;; best_pos[20..+4] = i32 %511
 34437|     ;; best_pos[16..+4] = i32 %510
 34438|     ;; best_pos[8..+8] = i64 %506
 34439|     ;; best_pos[0..+8] = i64 1
 34441|     ;; best_pos[20..+4] = i32 %511
 34442|     ;; best_pos[16..+4] = i32 %510
 34443|     ;; best_pos[8..+8] = i64 %506
 34444|     ;; best_pos[0..+8] = i64 1
 34445|  %512 = add nuw nsw i64 %464, 1                                                                                        ;L2564<2648<682<199<903<985<537
 34446|     ;; best_pos[20..+4] = i32 %511
 34447|     ;; best_pos[16..+4] = i32 %510
 34448|     ;; best_pos[8..+8] = i64 %506
 34449|     ;; best_pos[0..+8] = i64 1
 34450|     ;; iter[0..+4] = i64 %512
 34451|     ;; self = ptr undef
 34452|     ;; self = ptr undef
 34453|     ;; self = ptr undef
 34454|     ;; other = ptr undef
 34455|  %513 = icmp eq i64 %512, 6                                                                                            ;L1916<900<985<537
 34456|  br i1 %513, label %514, label %463, !llvm.loop !46625                                                                 ;L900<985<537
 34457| 
 34458| 514: ; preds = %463
 34459|     ;; dx = i32 %510
 34460|     ;; dy = i32 %511
 34461|  %515 = sext i32 %510 to i64                                                                                           ;L551
 34462|  %516 = add nsw i64 %515, -3                                                                                           ;L551
 34463|  %517 = add i64 %516, %148                                                                                             ;L551
 34464|     ;; self = i64 %517
 34465|     ;; min = i64 0
 34466|     ;; max = i64 29
 34467|  %518 = call i64 @llvm.smax.i64(i64 %517, i64 0)                                                                       ;L2025<551
 34468|  %519 = call i64 @llvm.umin.i64(i64 %518, i64 29)                                                                      ;L2025<551
 34469|  %520 = mul nuw nsw i64 %519, 32000                                                                                    ;L551
 34470|  %521 = add nuw nsw i64 %520, 16000                                                                                    ;L551
 34471|     ;; x = i64 %521
 34472|  %522 = zext nneg i32 %511 to i64                                                                                      ;L552
 34473|  %523 = add nsw i64 %522, -3                                                                                           ;L552
 34474|  %524 = add i64 %523, %151                                                                                             ;L552
 34475|     ;; self = i64 %524
 34476|     ;; min = i64 0
 34477|     ;; max = i64 29
 34478|  %525 = call i64 @llvm.smax.i64(i64 %524, i64 0)                                                                       ;L2025<552
 34479|  %526 = call i64 @llvm.umin.i64(i64 %525, i64 29)                                                                      ;L2025<552
 34480|  %527 = mul nuw nsw i64 %526, 32000                                                                                    ;L552
 34481|  %528 = add nuw nsw i64 %527, 16000                                                                                    ;L552
 34482|     ;; y = i64 %528
 34483|  store i32 2, ptr %0,                                                                                                  ;L554
 34484|  %529 = gep %0, i64 8                                                                                                  ;L554
 34485|  store i64 %521, ptr %529,                                                                                             ;L554
 34486|  %530 = gep %0, i64 16                                                                                                 ;L554
 34487|  store i64 %528, ptr %530,                                                                                             ;L554
 34488|  br label %44                                                                                                          ;L1
 34489| 
 34490| 531: ; preds = %191
 34491|  %532 = gep %6, i64 104                                                                                                ;L403
 34492|  %533 = load i64, ptr %532, , !!8                                                                                      ;L403
 34493|  %534 = icmp eq i64 %533, 13                                                                                           ;L403
 34494|     ;; c = ptr %6
 34495|  %535 = gep %6, i64 112
 34496|  %536 = load i64, ptr %535,
 34497|  %537 = icmp eq i64 %536, 2
 34498|  %538 = select i1 %534, i1 %537, i1 false                                                                              ;L403
 34499|  br i1 %538, label %545, label %539                                                                                    ;L403
 34500| 
 34501| 539: ; preds = %531
 34502|  %540 = gep %6, i64 776                                                                                                ;L458
 34503|  %541 = load i64, ptr %540, , !!8                                                                                      ;L458
 34504|  %542 = xor i64 %541, -9223372036854775808                                                                             ;L458
 34505|  %543 = icmp slt i64 %541, 0                                                                                           ;L458
 34506|  %544 = select i1 %543, i64 %542, i64 4                                                                                ;L458
 34507|  switch i64 %544, label %627 [
 34508|  i64 1, label %637
 34509|  i64 3, label %635
 34510|  i64 4, label %636
 34511|  ]                                                                                                                     ;L458
 34512| 
 34513| 545: ; preds = %531
 34514|  %546 = gep %6, i64 120                                                                                                ;L406
 34515|  %547 = load i64, ptr %546, , !!8                                                                                      ;L406
 34516|     ;; x = i64 %547
 34517|  %548 = gep %6, i64 128                                                                                                ;L406
 34518|  %549 = load i64, ptr %548, , !!8                                                                                      ;L406
 34519|     ;; y = i64 %549
 34521|  %550 = gep %6, i64 1632                                                                                               ;L408
 34522|  %551 = load i64, ptr %550, , !!8                                                                                      ;L408
 34523|  store i64 %551, ptr %24,                                                                                              ;L408
 34525|  %552 = gep %6, i64 1640                                                                                               ;L409
 34526|  %553 = load i64, ptr %552, , !!8                                                                                      ;L409
 34527|  store i64 %553, ptr %23,                                                                                              ;L409
 34528|     ;; real_dest[0..+8] = i64 %547
 34529|     ;; real_dest[8..+8] = i64 %549
 34531|  store i64 0, ptr %22,                                                                                                 ;L391<413
 34532|  %554 = gep %22, i64 8                                                                                                 ;L391<413
 34533|  store i64 1000, ptr %554,                                                                                             ;L391<413
 34534|  %555 = gep %22, i64 16                                                                                                ;L391<413
 34535|  store i8 0, ptr %555,                                                                                                 ;L391<413
 34536|  %556 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %2, ptr %22)                            ;L413
 34538|  %557 = icmp ult i64 %556, %68                                                                                         ;L413
 34539|     ;; misjudged = i1 %557
 34540|  br i1 %557, label %558, label %564                                                                                    ;L414
 34541| 
 34542| 558: ; preds = %545
 34543|  %559 = tail call { i64, ptr } %88(ptr %84)                                                                            ;L417
 34544|  %560 = extractvalue { i64, ptr } %559, 0                                                                              ;L417
 34545|  %561 = icmp eq i64 %560, 2                                                                                            ;L417
 34546|  %562 = icmp ugt i64 %67, 39                                                                                           ;L417
 34547|  %563 = and i1 %562, %561                                                                                              ;L417
 34548|  br i1 %563, label %564, label %581                                                                                    ;L417
 34549| 
 34550| 564: ; preds = %581, %558, %545
 34551|  %565 = phi i64 [ %549, %545 ], [ %598, %581 ], [ %553, %558 ]                                                         ;L0
 34552|  %566 = phi i64 [ %547, %545 ], [ %597, %581 ], [ %551, %558 ]                                                         ;L0
 34553|     ;; x = i64 %566
 34554|     ;; y = i64 %565
 34555|  %567 = gep %4, i64 8                                                                                                  ;L429
 34556|  %568 = load ptr, ptr %567, , !!8, !!8                                                                                 ;L429
 34557|  %569 = gep %568, i64 8                                                                                                ;L429
 34558|  %570 = load ptr, ptr %569, , !!8, !!8                                                                                 ;L429
 34559|  %571 = gep %568, i64 24                                                                                               ;L429
 34560|  %572 = load ptr, ptr %571, , !!8, !!8                                                                                 ;L429
 34561|  %573 = gep %568, i64 32                                                                                               ;L429
 34562|  %574 = load ptr, ptr %573, , !!8, !!8                                                                                 ;L429
 34563|  %575 = gep %6, i64 1472                                                                                               ;L429
 34564|  %576 = load i64, ptr %575, , !!8                                                                                      ;L429
 34565|  %577 = gep %6, i64 1600                                                                                               ;L430
 34566|  %578 = load i64, ptr %577, , !!8                                                                                      ;L430
 34567|  %579 = mul i64 %578, %125                                                                                             ;L430
 34569|  store ptr null, ptr %21,                                                                                              ;L430
 34570|  call void @gc::simulation6entityNtB5_6Entity7move_to(ptr %570, ptr %572, ptr %574, i64 %576, ptr %24, ptr %23, i64 %579, i64 %566, i64 %565, ptr %21) ;L429
 34572|  %580 = call zeroext i1 @gc::simulation6entity11nt_trace_on()                                                          ;L432
 34573|  br i1 %580, label %608, label %599                                                                                    ;L432
 34574| 
 34575| 581: ; preds = %558
 34576|  %582 = gep %39, i64 1632                                                                                              ;L420
 34577|  %583 = load i64, ptr %582, , !!8                                                                                      ;L420
 34578|     ;; dir_x = !DIArgList(i64 %547, i64 %583)
 34579|  %584 = gep %39, i64 1640                                                                                              ;L421
 34580|  %585 = load i64, ptr %584, , !!8                                                                                      ;L421
 34581|     ;; dir_y = !DIArgList(i64 %549, i64 %585)
 34582|  %586 = gep %4, i64 8                                                                                                  ;L422
 34583|  %587 = load ptr, ptr %586, , !!8, !!8                                                                                 ;L422
 34584|  %588 = gep %587, i64 32                                                                                               ;L422
 34585|  %589 = load ptr, ptr %588, , !!8, !!8                                                                                 ;L422
 34586|  %590 = gep %587, i64 8                                                                                                ;L422
 34587|  %591 = load ptr, ptr %590, , !!8, !!8                                                                                 ;L422
 34588|  %592 = shl i64 %583, 1                                                                                                ;L423
 34589|  %593 = sub i64 %592, %547                                                                                             ;L423
 34590|  %594 = shl i64 %585, 1                                                                                                ;L423
 34591|  %595 = sub i64 %594, %549                                                                                             ;L423
 34592|  %596 = tail call { i64, i64 } @gc::simulation4gameNtB5_4Game15adjust_position(ptr %589, ptr %591, i64 %593, i64 %595) ;L422
 34593|  %597 = extractvalue { i64, i64 } %596, 0                                                                              ;L422
 34594|  %598 = extractvalue { i64, i64 } %596, 1                                                                              ;L422
 34595|  br label %564                                                                                                         ;L417
 34596| 
 34597| 599: ; preds = %608, %564
 34598|  %600 = load i64, ptr %24, , !!8                                                                                       ;L446
 34599|  %601 = gep %39, i64 1632                                                                                              ;L446
 34600|  %602 = load i64, ptr %601, , !!8                                                                                      ;L446
 34601|  %603 = sub i64 %600, %602                                                                                             ;L446
 34602|     ;; dir_x = i64 %603
 34603|  %604 = load i64, ptr %23, , !!8                                                                                       ;L447
 34604|  %605 = gep %39, i64 1640                                                                                              ;L447
 34605|  %606 = load i64, ptr %605, , !!8                                                                                      ;L447
 34606|  %607 = sub i64 %604, %606                                                                                             ;L447
 34607|     ;; dir_y = i64 %607
 34608|  call fastcc void @ai::abstract_input20apply_aim_offset_dir(ptr %0, ptr %2, i64 %66, i64 %603, i64 %607)               ;L448
 34611|  br label %44                                                                                                          ;L1
 34612| 
 34613| 608: ; preds = %564
 34614|  %609 = gep %39, i64 1472                                                                                              ;L433
 34615|  %610 = load i64, ptr %609, , !!8                                                                                      ;L433
 34617|  %611 = gep %86, i64 40                                                                                                ;L434
 34618|  %612 = load ptr, ptr %611, , !!8                                                                                      ;L434
 34619|  %613 = call i64 %612(ptr %84)                                                                                         ;L434
 34620|  %614 = gep %3, i64 264                                                                                                ;L441
 34621|  %615 = load i64, ptr %614, , !!8                                                                                      ;L441
 34622|  %616 = gep %20, i64 24                                                                                                ;L433
 34623|  store i64 %613, ptr %616,                                                                                             ;L433
 34624|  %617 = gep %20, i64 32                                                                                                ;L433
 34625|  store i64 %576, ptr %617,                                                                                             ;L433
 34626|  %618 = gep %20, i64 40                                                                                                ;L433
 34627|  store i64 %551, ptr %618,                                                                                             ;L433
 34628|  %619 = gep %20, i64 48                                                                                                ;L433
 34629|  store i64 %553, ptr %619,                                                                                             ;L433
 34630|  store i64 1, ptr %20,                                                                                                 ;L433
 34631|  %620 = gep %20, i64 8                                                                                                 ;L433
 34632|  store i64 %547, ptr %620,                                                                                             ;L433
 34633|  %621 = gep %20, i64 16                                                                                                ;L433
 34634|  store i64 %549, ptr %621,                                                                                             ;L433
 34635|  %622 = gep %20, i64 80                                                                                                ;L433
 34636|  %623 = zext i1 %557 to i8                                                                                             ;L433
 34637|  store i8 %623, ptr %622,                                                                                              ;L433
 34638|  %624 = gep %20, i64 56                                                                                                ;L433
 34639|  store i64 %125, ptr %624,                                                                                             ;L433
 34640|  %625 = gep %20, i64 64                                                                                                ;L433
 34641|  store i64 %615, ptr %625,                                                                                             ;L433
 34642|  %626 = gep %20, i64 72                                                                                                ;L433
 34643|  store i64 %66, ptr %626,                                                                                              ;L433
 34644|  call void @gc::simulation6entity19nt_trace_record_aim(i64 %610, ptr %20)                                              ;L433
 34646|  br label %599                                                                                                         ;L432
 34647| 
 34648| 627: ; preds = %539
 34649|  %628 = tail call zeroext i1 @gc::simulation6entity11nt_trace_on()                                                     ;L513
 34650|  %629 = and i1 %534, %628                                                                                              ;L513
 34651|  br i1 %629, label %760, label %630                                                                                    ;L513
 34652| 
 34653| 630: ; preds = %627
 34654|  %631 = gep %6, i64 1632
 34655|  %632 = load i64, ptr %631,                                                                                            ;L160<526
 34656|  %633 = gep %6, i64 1640
 34657|  %634 = load i64, ptr %633,                                                                                            ;L161<526
 34658|  br label %751                                                                                                         ;L513
 34659| 
 34660| 635: ; preds = %539
 34664|  br label %637                                                                                                         ;L458
 34665| 
 34666| 636: ; preds = %539
 34670|  br label %637                                                                                                         ;L458
 34671| 
 34672| 637: ; preds = %636, %635, %539
 34673|  %638 = phi i64 [ 824, %636 ], [ 808, %635 ], [ 832, %539 ]
 34674|  %639 = phi i64 [ 832, %636 ], [ 816, %635 ], [ 840, %539 ]
 34675|  %640 = phi i64 [ 840, %636 ], [ 824, %635 ], [ 848, %539 ]
 34676|  %641 = gep %6, i64 %638                                                                                               ;L459
 34677|  %642 = gep %6, i64 %639                                                                                               ;L459
 34678|  %643 = gep %6, i64 %640                                                                                               ;L459
 34679|  %644 = load i64, ptr %643, , !!8                                                                                      ;L459
 34680|  %645 = load i64, ptr %642, , !!8                                                                                      ;L459
 34681|  %646 = load i64, ptr %641, , !!8                                                                                      ;L459
 34682|     ;; y = i64 %644
 34683|     ;; x = i64 %645
 34684|     ;; speed = i64 %646
 34685|     ;; rush_real_dest[0..+8] = i64 %645
 34686|     ;; rush_real_dest[8..+8] = i64 %644
 34688|  store i64 0, ptr %19,                                                                                                 ;L391<462
 34689|  %647 = gep %19, i64 8                                                                                                 ;L391<462
 34690|  store i64 1000, ptr %647,                                                                                             ;L391<462
 34691|  %648 = gep %19, i64 16                                                                                                ;L391<462
 34692|  store i8 0, ptr %648,                                                                                                 ;L391<462
 34693|  %649 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %2, ptr %19)                            ;L462
 34695|  %650 = icmp ult i64 %649, %68                                                                                         ;L462
 34696|     ;; rush_misjudged = i1 %650
 34697|  br i1 %650, label %651, label %657                                                                                    ;L463
 34698| 
 34699| 651: ; preds = %637
 34700|  %652 = tail call { i64, ptr } %88(ptr %84)                                                                            ;L465
 34701|  %653 = extractvalue { i64, ptr } %652, 0                                                                              ;L465
 34702|  %654 = icmp eq i64 %653, 2                                                                                            ;L465
 34703|  %655 = icmp ugt i64 %67, 39                                                                                           ;L465
 34704|  %656 = and i1 %655, %654                                                                                              ;L465
 34705|  br i1 %656, label %685, label %667                                                                                    ;L465
 34706| 
 34707| 657: ; preds = %685, %667, %637
 34708|  %658 = phi i64 [ %687, %685 ], [ %683, %667 ], [ %645, %637 ]                                                         ;L0
 34709|  %659 = phi i64 [ %689, %685 ], [ %684, %667 ], [ %644, %637 ]                                                         ;L0
 34710|     ;; x = i64 %658
 34711|     ;; target_x = i64 %658
 34712|     ;; y = i64 %659
 34713|     ;; target_y = i64 %659
 34714|  %660 = tail call zeroext i1 @gc::simulation6entity11nt_trace_on()                                                     ;L476
 34715|  %661 = and i1 %534, %660                                                                                              ;L476
 34716|  br i1 %661, label %695, label %662                                                                                    ;L476
 34717| 
 34718| 662: ; preds = %657
 34719|  %663 = gep %6, i64 1632
 34720|  %664 = load i64, ptr %663,                                                                                            ;L490
 34721|  %665 = gep %6, i64 1640
 34722|  %666 = load i64, ptr %665,                                                                                            ;L490
 34723|  br label %690                                                                                                         ;L476
 34724| 
 34725| 667: ; preds = %651
 34726|  %668 = gep %39, i64 1632                                                                                              ;L468
 34727|  %669 = load i64, ptr %668, , !!8                                                                                      ;L468
 34728|     ;; dir_x = !DIArgList(i64 %645, i64 %669)
 34729|  %670 = gep %39, i64 1640                                                                                              ;L469
 34730|  %671 = load i64, ptr %670, , !!8                                                                                      ;L469
 34731|     ;; dir_y = !DIArgList(i64 %644, i64 %671)
 34732|  %672 = gep %4, i64 8                                                                                                  ;L470
 34733|  %673 = load ptr, ptr %672, , !!8, !!8                                                                                 ;L470
 34734|  %674 = gep %673, i64 32                                                                                               ;L470
 34735|  %675 = load ptr, ptr %674, , !!8, !!8                                                                                 ;L470
 34736|  %676 = gep %673, i64 8                                                                                                ;L470
 34737|  %677 = load ptr, ptr %676, , !!8, !!8                                                                                 ;L470
 34738|  %678 = shl i64 %669, 1                                                                                                ;L470
 34739|  %679 = sub i64 %678, %645                                                                                             ;L470
 34740|  %680 = shl i64 %671, 1                                                                                                ;L470
 34741|  %681 = sub i64 %680, %644                                                                                             ;L470
 34742|  %682 = tail call { i64, i64 } @gc::simulation4gameNtB5_4Game15adjust_position(ptr %675, ptr %677, i64 %679, i64 %681) ;L470
 34743|  %683 = extractvalue { i64, i64 } %682, 0                                                                              ;L470
 34744|  %684 = extractvalue { i64, i64 } %682, 1                                                                              ;L470
 34745|  br label %657                                                                                                         ;L465
 34746| 
 34747| 685: ; preds = %651
 34748|  %686 = gep %6, i64 1632                                                                                               ;L466
 34749|  %687 = load i64, ptr %686, , !!8                                                                                      ;L466
 34750|  %688 = gep %6, i64 1640                                                                                               ;L466
 34751|  %689 = load i64, ptr %688, , !!8                                                                                      ;L466
 34752|  br label %657                                                                                                         ;L465
 34753| 
 34754| 690: ; preds = %695, %662
 34755|  %691 = phi i64 [ %666, %662 ], [ %706, %695 ]                                                                         ;L490
 34756|  %692 = phi i64 [ %664, %662 ], [ %704, %695 ]                                                                         ;L490
 34757|  %693 = call i64 @gc::utils8distance(i64 %658, i64 %659, i64 %692, i64 %691)                                           ;L490
 34758|     ;; dist = i64 %693
 34759|  %694 = icmp eq i64 %646, 0                                                                                            ;L491
 34760|  br i1 %694, label %723, label %720                                                                                    ;L491
 34761| 
 34762| 695: ; preds = %657
 34763|  %696 = gep %39, i64 1472                                                                                              ;L477
 34764|  %697 = load i64, ptr %696, , !!8                                                                                      ;L477
 34766|  %698 = gep %86, i64 40                                                                                                ;L478
 34767|  %699 = load ptr, ptr %698, , !!8                                                                                      ;L478
 34768|  %700 = tail call i64 %699(ptr %84)                                                                                    ;L478
 34769|  %701 = gep %6, i64 1472                                                                                               ;L479
 34770|  %702 = load i64, ptr %701, , !!8                                                                                      ;L479
 34771|  %703 = gep %6, i64 1632                                                                                               ;L480
 34772|  %704 = load i64, ptr %703, , !!8                                                                                      ;L480
 34773|  %705 = gep %6, i64 1640                                                                                               ;L481
 34774|  %706 = load i64, ptr %705, , !!8                                                                                      ;L481
 34775|  %707 = gep %3, i64 264                                                                                                ;L485
 34776|  %708 = load i64, ptr %707, , !!8                                                                                      ;L485
 34777|  %709 = gep %18, i64 24                                                                                                ;L477
 34778|  store i64 %700, ptr %709,                                                                                             ;L477
 34779|  %710 = gep %18, i64 32                                                                                                ;L477
 34780|  store i64 %702, ptr %710,                                                                                             ;L477
 34781|  %711 = gep %18, i64 40                                                                                                ;L477
 34782|  store i64 %704, ptr %711,                                                                                             ;L477
 34783|  %712 = gep %18, i64 48                                                                                                ;L477
 34784|  store i64 %706, ptr %712,                                                                                             ;L477
 34785|  store i64 1, ptr %18,                                                                                                 ;L477
 34786|  %713 = gep %18, i64 8                                                                                                 ;L477
 34787|  store i64 %645, ptr %713,                                                                                             ;L477
 34788|  %714 = gep %18, i64 16                                                                                                ;L477
 34789|  store i64 %644, ptr %714,                                                                                             ;L477
 34790|  %715 = gep %18, i64 80                                                                                                ;L477
 34791|  %716 = zext i1 %650 to i8                                                                                             ;L477
 34792|  store i8 %716, ptr %715,                                                                                              ;L477
 34793|  %717 = gep %18, i64 56                                                                                                ;L477
 34794|  store i64 %125, ptr %717,                                                                                             ;L477
 34795|  %718 = gep %18, i64 64                                                                                                ;L477
 34796|  store i64 %708, ptr %718,                                                                                             ;L477
 34797|  %719 = gep %18, i64 72                                                                                                ;L477
 34798|  store i64 %66, ptr %719,                                                                                              ;L477
 34799|  call void @gc::simulation6entity19nt_trace_record_aim(i64 %697, ptr %18)                                              ;L477
 34801|  br label %690                                                                                                         ;L476
 34802| 
 34803| 720: ; preds = %690
 34804|  %721 = udiv i64 %693, %646                                                                                            ;L491
 34805|     ;; tick = i64 %721
 34806|  %722 = icmp ult i64 %721, %125                                                                                        ;L492
 34807|  br i1 %722, label %724, label %742                                                                                    ;L492
 34808| 
 34809| 723: ; preds = %690
 34810|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.168add0ea037d45d276f5936ae758fe5.140) #30  ;L491
 34811|  unreachable                                                                                                           ;L491
 34812| 
 34813| 724: ; preds = %720
 34814|  %725 = gep %39, i64 1632                                                                                              ;L495
 34815|  %726 = load i64, ptr %725, , !!8                                                                                      ;L495
 34816|  %727 = sub i64 %658, %726                                                                                             ;L495
 34817|     ;; dir_x = i64 %727
 34818|  %728 = gep %39, i64 1640                                                                                              ;L496
 34819|  %729 = load i64, ptr %728, , !!8                                                                                      ;L496
 34820|  %730 = sub i64 %659, %729                                                                                             ;L496
 34821|     ;; dir_y = i64 %730
 34822|  %731 = mul i64 %727, %727                                                                                             ;L497
 34823|  %732 = mul i64 %730, %730                                                                                             ;L497
 34824|  %733 = add i64 %732, %731                                                                                             ;L497
 34825|  %734 = call i64 @gc::utils5isqrt(i64 %733)                                                                            ;L497
 34826|     ;; self = i64 %734
 34827|     ;; other = i64 1
 34828|  %735 = call i64 @llvm.smax.i64(i64 %734, i64 1)                                                                       ;L1039<497
 34829|     ;; sz = i64 %735
 34830|  %736 = mul i64 %727, %125                                                                                             ;L499
 34831|  %737 = sdiv i64 %736, %735                                                                                            ;L499
 34832|  %738 = add i64 %737, %726                                                                                             ;L499
 34833|     ;; nx = i64 %738
 34834|  %739 = mul i64 %730, %125                                                                                             ;L500
 34835|  %740 = sdiv i64 %739, %735                                                                                            ;L500
 34836|  %741 = add i64 %740, %729                                                                                             ;L500
 34837|     ;; x = i64 %738
 34838|     ;; target_x = i64 %738
 34839|     ;; y = i64 %741
 34840|     ;; target_y = i64 %741
 34841|  br label %742                                                                                                         ;L492
 34842| 
 34843| 742: ; preds = %724, %720
 34844|  %743 = phi i64 [ %658, %720 ], [ %738, %724 ]                                                                         ;L0
 34845|  %744 = phi i64 [ %659, %720 ], [ %741, %724 ]                                                                         ;L0
 34846|     ;; target_y = i64 %744
 34847|     ;; y = i64 %744
 34848|     ;; target_x = i64 %743
 34849|     ;; x = i64 %743
 34850|  %745 = gep %39, i64 1632                                                                                              ;L504
 34851|  %746 = load i64, ptr %745, , !!8                                                                                      ;L504
 34852|  %747 = sub i64 %743, %746                                                                                             ;L504
 34853|     ;; dir_x = i64 %747
 34854|  %748 = gep %39, i64 1640                                                                                              ;L505
 34855|  %749 = load i64, ptr %748, , !!8                                                                                      ;L505
 34856|  %750 = sub i64 %744, %749                                                                                             ;L505
 34857|     ;; dir_y = i64 %750
 34858|  call fastcc void @ai::abstract_input20apply_aim_offset_dir(ptr %0, ptr %2, i64 %66, i64 %747, i64 %750)               ;L506
 34859|  br label %44                                                                                                          ;L1
 34860| 
 34861| 751: ; preds = %760, %630
 34862|  %752 = phi i64 [ %634, %630 ], [ %771, %760 ]                                                                         ;L161<526
 34863|  %753 = phi i64 [ %632, %630 ], [ %769, %760 ]                                                                         ;L160<526
 34864|     ;; self = ptr %7
 34865|  %754 = gep %39, i64 1632                                                                                              ;L160<526
 34866|  %755 = load i64, ptr %754, , !!8                                                                                      ;L160<526
 34867|  %756 = sub i64 %753, %755                                                                                             ;L160<526
 34868|  %757 = gep %39, i64 1640                                                                                              ;L161<526
 34869|  %758 = load i64, ptr %757, , !!8                                                                                      ;L161<526
 34870|  %759 = sub i64 %752, %758                                                                                             ;L161<526
 34871|     ;; input[8..+8] = i64 %756
 34872|     ;; input[16..+8] = i64 %759
 34873|     ;; input[0..+4] = i32 1
 34874|     ;; dir_x = i64 %756
 34875|     ;; dir_y = i64 %759
 34876|  call fastcc void @ai::abstract_input20apply_aim_offset_dir(ptr %0, ptr %2, i64 %66, i64 %756, i64 %759)               ;L528
 34877|  br label %44                                                                                                          ;L531
 34878| 
 34879| 760: ; preds = %627
 34880|  %761 = gep %39, i64 1472                                                                                              ;L514
 34881|  %762 = load i64, ptr %761, , !!8                                                                                      ;L514
 34883|  %763 = gep %86, i64 40                                                                                                ;L515
 34884|  %764 = load ptr, ptr %763, , !!8                                                                                      ;L515
 34885|  %765 = tail call i64 %764(ptr %84)                                                                                    ;L515
 34886|  %766 = gep %6, i64 1472                                                                                               ;L516
 34887|  %767 = load i64, ptr %766, , !!8                                                                                      ;L516
 34888|  %768 = gep %6, i64 1632                                                                                               ;L517
 34889|  %769 = load i64, ptr %768, , !!8                                                                                      ;L517
 34890|  %770 = gep %6, i64 1640                                                                                               ;L518
 34891|  %771 = load i64, ptr %770, , !!8                                                                                      ;L518
 34892|  %772 = gep %3, i64 264                                                                                                ;L522
 34893|  %773 = load i64, ptr %772, , !!8                                                                                      ;L522
 34894|  %774 = gep %17, i64 24                                                                                                ;L514
 34895|  store i64 %765, ptr %774,                                                                                             ;L514
 34896|  %775 = gep %17, i64 32                                                                                                ;L514
 34897|  store i64 %767, ptr %775,                                                                                             ;L514
 34898|  %776 = gep %17, i64 40                                                                                                ;L514
 34899|  store i64 %769, ptr %776,                                                                                             ;L514
 34900|  %777 = gep %17, i64 48                                                                                                ;L514
 34901|  store i64 %771, ptr %777,                                                                                             ;L514
 34902|  store i64 0, ptr %17,                                                                                                 ;L514
 34903|  %778 = gep %17, i64 80                                                                                                ;L514
 34904|  store i8 0, ptr %778,                                                                                                 ;L514
 34905|  %779 = gep %17, i64 56                                                                                                ;L514
 34906|  store i64 %125, ptr %779,                                                                                             ;L514
 34907|  %780 = gep %17, i64 64                                                                                                ;L514
 34908|  store i64 %773, ptr %780,                                                                                             ;L514
 34909|  %781 = gep %17, i64 72                                                                                                ;L514
 34910|  store i64 %66, ptr %781,                                                                                              ;L514
 34911|  call void @gc::simulation6entity19nt_trace_record_aim(i64 %762, ptr %17)                                              ;L514
 34913|  br label %751                                                                                                         ;L513
 34914| 
 34915| 782: ; preds = %782, %204
 34916|  %783 = phi i64 [ 1, %204 ], [ %831, %782 ]
 34917|  %784 = phi i64 [ %239, %204 ], [ %830, %782 ]
 34918|  %785 = phi i32 [ 0, %204 ], [ %829, %782 ]
 34919|  %786 = phi i32 [ %248, %204 ], [ %825, %782 ]
 34920|     ;; old = i64 %783
 34921|     ;; start = i64 %783
 34922|     ;; self = i64 %783
 34923|     ;; self = i64 %783
 34924|     ;; iter[0..+4] = i64 %783
 34925|     ;; dx = i64 %783
 34926|     ;; iter[0..+4] = i32 0
 34927|     ;; iter[4..+4] = i32 6
 34928|  %787 = add i64 %207, %783
 34930|     ;; best_pos[8..+8] = i64 %784
 34931|     ;; best_pos[16..+4] = i32 %785
 34932|     ;; best_pos[20..+4] = i32 %786
 34933|     ;; self = ptr undef
 34934|     ;; self = ptr undef
 34935|     ;; self = ptr undef
 34936|     ;; other = ptr undef
 34937|     ;; start = i64 0
 34938|     ;; self = i64 0
 34939|     ;; self = i64 0
 34940|     ;; iter[0..+4] = i64 1
 34941|     ;; dy = i64 0
 34943|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %25, i64 %1, ptr %3, ptr %4, ptr %5, i64 %787, i64 %210, i8 12) ;L384
 34944|  %788 = load i64, ptr %211, , !!8                                                                                      ;L387
 34945|  %789 = load i64, ptr %25, , !!8                                                                                       ;L387
 34946|  %790 = sub i64 %788, %789                                                                                             ;L387
 34947|     ;; v = i64 %790
 34949|     ;; self[8..+8] = i64 %784
 34950|     ;; self[16..+4] = i32 %785
 34951|     ;; self[20..+4] = i32 %786
 34952|     ;; f = ptr undef
 34953|  %791 = icmp slt i64 %784, %790
 34954|  %792 = select i1 %791, i32 0, i32 %786                                                                                ;L708<389
 34955|  %793 = trunc nuw nsw i64 %783 to i32                                                                                  ;L708<389
 34956|  %794 = call i64 @llvm.smax.i64(i64 %784, i64 %790)                                                                    ;L708<389
 34957|     ;; best_pos[0..+8] = i64 1
 34958|     ;; best_pos[8..+8] = i64 %794
 34960|     ;; best_pos[20..+4] = i32 %792
 34962|     ;; best_pos[0..+8] = i64 1
 34963|     ;; best_pos[8..+8] = i64 %794
 34965|     ;; best_pos[20..+4] = i32 %792
 34966|     ;; start = i64 1
 34967|     ;; self = i64 1
 34968|     ;; self = i64 1
 34969|     ;; iter[0..+4] = i64 2
 34970|     ;; dy = i64 1
 34972|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %25, i64 %1, ptr %3, ptr %4, ptr %5, i64 %787, i64 %214, i8 12) ;L384
 34973|  %795 = load i64, ptr %211, , !!8                                                                                      ;L387
 34974|  %796 = load i64, ptr %25, , !!8                                                                                       ;L387
 34975|  %797 = sub i64 %795, %796                                                                                             ;L387
 34976|     ;; v = i64 %797
 34978|     ;; self[8..+8] = i64 %794
 34980|     ;; self[20..+4] = i32 %792
 34981|  %798 = icmp slt i64 %794, %797
 34982|  %799 = select i1 %798, i32 1, i32 %792                                                                                ;L708<389
 34983|  %800 = or i1 %798, %791                                                                                               ;L708<389
 34984|  %801 = call i64 @llvm.smax.i64(i64 %794, i64 %797)                                                                    ;L708<389
 34985|     ;; best_pos[0..+8] = i64 1
 34986|     ;; best_pos[8..+8] = i64 %801
 34988|     ;; best_pos[20..+4] = i32 %799
 34990|     ;; best_pos[0..+8] = i64 1
 34991|     ;; best_pos[8..+8] = i64 %801
 34993|     ;; best_pos[20..+4] = i32 %799
 34994|     ;; start = i64 2
 34995|     ;; self = i64 2
 34996|     ;; self = i64 2
 34997|     ;; iter[0..+4] = i64 3
 34998|     ;; dy = i64 2
 35000|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %25, i64 %1, ptr %3, ptr %4, ptr %5, i64 %787, i64 %217, i8 12) ;L384
 35001|  %802 = load i64, ptr %211, , !!8                                                                                      ;L387
 35002|  %803 = load i64, ptr %25, , !!8                                                                                       ;L387
 35003|  %804 = sub i64 %802, %803                                                                                             ;L387
 35004|     ;; v = i64 %804
 35006|     ;; self[8..+8] = i64 %801
 35008|     ;; self[20..+4] = i32 %799
 35009|  %805 = icmp slt i64 %801, %804
 35010|  %806 = select i1 %805, i32 2, i32 %799                                                                                ;L708<389
 35011|  %807 = call i64 @llvm.smax.i64(i64 %801, i64 %804)                                                                    ;L708<389
 35012|     ;; best_pos[0..+8] = i64 1
 35013|     ;; best_pos[8..+8] = i64 %807
 35015|     ;; best_pos[20..+4] = i32 %806
 35017|     ;; best_pos[0..+8] = i64 1
 35018|     ;; best_pos[8..+8] = i64 %807
 35020|     ;; best_pos[20..+4] = i32 %806
 35021|     ;; start = i64 3
 35022|     ;; self = i64 3
 35023|     ;; self = i64 3
 35024|     ;; iter[0..+4] = i64 4
 35025|     ;; dy = i64 3
 35027|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %25, i64 %1, ptr %3, ptr %4, ptr %5, i64 %787, i64 %209, i8 12) ;L384
 35028|  %808 = load i64, ptr %211, , !!8                                                                                      ;L387
 35029|  %809 = load i64, ptr %25, , !!8                                                                                       ;L387
 35030|  %810 = sub i64 %808, %809                                                                                             ;L387
 35031|     ;; v = i64 %810
 35033|     ;; self[8..+8] = i64 %807
 35035|     ;; self[20..+4] = i32 %806
 35036|  %811 = icmp slt i64 %807, %810
 35037|  %812 = select i1 %811, i32 3, i32 %806                                                                                ;L708<389
 35038|  %813 = or i1 %811, %805                                                                                               ;L708<389
 35039|  %814 = call i64 @llvm.smax.i64(i64 %807, i64 %810)                                                                    ;L708<389
 35040|     ;; best_pos[0..+8] = i64 1
 35041|     ;; best_pos[8..+8] = i64 %814
 35043|     ;; best_pos[20..+4] = i32 %812
 35045|     ;; best_pos[0..+8] = i64 1
 35046|     ;; best_pos[8..+8] = i64 %814
 35048|     ;; best_pos[20..+4] = i32 %812
 35049|     ;; start = i64 4
 35050|     ;; self = i64 4
 35051|     ;; self = i64 4
 35052|     ;; iter[0..+4] = i64 5
 35053|     ;; dy = i64 4
 35055|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %25, i64 %1, ptr %3, ptr %4, ptr %5, i64 %787, i64 %222, i8 12) ;L384
 35056|  %815 = load i64, ptr %211, , !!8                                                                                      ;L387
 35057|  %816 = load i64, ptr %25, , !!8                                                                                       ;L387
 35058|  %817 = sub i64 %815, %816                                                                                             ;L387
 35059|     ;; v = i64 %817
 35061|     ;; self[8..+8] = i64 %814
 35063|     ;; self[20..+4] = i32 %812
 35064|  %818 = icmp slt i64 %814, %817
 35065|  %819 = select i1 %818, i32 4, i32 %812                                                                                ;L708<389
 35066|  %820 = call i64 @llvm.smax.i64(i64 %814, i64 %817)                                                                    ;L708<389
 35067|     ;; best_pos[0..+8] = i64 1
 35068|     ;; best_pos[8..+8] = i64 %820
 35070|     ;; best_pos[20..+4] = i32 %819
 35072|     ;; best_pos[0..+8] = i64 1
 35073|     ;; best_pos[8..+8] = i64 %820
 35075|     ;; best_pos[20..+4] = i32 %819
 35076|     ;; start = i64 5
 35077|     ;; self = i64 5
 35078|     ;; self = i64 5
 35079|     ;; iter[0..+4] = i64 6
 35080|     ;; dy = i64 5
 35082|  call void @ai::position_eval22position_score_at_cell(ptr sret([56 x i8]) %25, i64 %1, ptr %3, ptr %4, ptr %5, i64 %787, i64 %225, i8 12) ;L384
 35083|  %821 = load i64, ptr %211, , !!8                                                                                      ;L387
 35084|  %822 = load i64, ptr %25, , !!8                                                                                       ;L387
 35085|  %823 = sub i64 %821, %822                                                                                             ;L387
 35086|     ;; v = i64 %823
 35088|     ;; self[8..+8] = i64 %820
 35090|     ;; self[20..+4] = i32 %819
 35091|  %824 = icmp slt i64 %820, %823
 35092|  %825 = select i1 %824, i32 5, i32 %819                                                                                ;L708<389
 35093|  %826 = or i1 %824, %818                                                                                               ;L708<389
 35094|  %827 = select i1 %826, i1 true, i1 %813                                                                               ;L708<389
 35095|  %828 = select i1 %827, i1 true, i1 %800                                                                               ;L708<389
 35096|  %829 = select i1 %828, i32 %793, i32 %785                                                                             ;L708<389
 35097|  %830 = call i64 @llvm.smax.i64(i64 %820, i64 %823)                                                                    ;L708<389
 35098|     ;; best_pos[0..+8] = i64 1
 35099|     ;; best_pos[8..+8] = i64 %830
 35100|     ;; best_pos[16..+4] = i32 %829
 35101|     ;; best_pos[20..+4] = i32 %825
 35103|     ;; best_pos[0..+8] = i64 1
 35104|     ;; best_pos[8..+8] = i64 %830
 35105|     ;; best_pos[16..+4] = i32 %829
 35106|     ;; best_pos[20..+4] = i32 %825
 35107|  %831 = add nuw nsw i64 %783, 1                                                                                        ;L2564<2648<682<199<903<985<382
 35108|     ;; iter[0..+4] = i64 %831
 35109|     ;; best_pos[0..+8] = i64 1
 35110|     ;; best_pos[8..+8] = i64 %830
 35111|     ;; best_pos[16..+4] = i32 %829
 35112|     ;; best_pos[20..+4] = i32 %825
 35113|     ;; self = ptr undef
 35114|     ;; self = ptr undef
 35115|     ;; self = ptr undef
 35116|     ;; other = ptr undef
 35117|  %832 = icmp eq i64 %831, 6                                                                                            ;L1916<900<985<382
 35118|  br i1 %832, label %833, label %782, !llvm.loop !46742                                                                 ;L900<985<382
 35119| 
 35120| 833: ; preds = %782
 35121|     ;; dx = i32 %829
 35122|     ;; dy = i32 %825
 35123|  %834 = sext i32 %829 to i64                                                                                           ;L396
 35124|  %835 = add nsw i64 %834, -3                                                                                           ;L396
 35125|  %836 = add i64 %835, %206                                                                                             ;L396
 35126|     ;; self = i64 %836
 35127|     ;; min = i64 0
 35128|     ;; max = i64 29
 35129|  %837 = call i64 @llvm.smax.i64(i64 %836, i64 0)                                                                       ;L2025<396
 35130|  %838 = call i64 @llvm.umin.i64(i64 %837, i64 29)                                                                      ;L2025<396
 35131|  %839 = mul nuw nsw i64 %838, 32000                                                                                    ;L396
 35133|  %840 = zext nneg i32 %825 to i64                                                                                      ;L397
 35134|  %841 = add nsw i64 %840, -3                                                                                           ;L397
 35135|  %842 = add i64 %841, %209                                                                                             ;L397
 35136|     ;; self = i64 %842
 35137|     ;; min = i64 0
 35138|     ;; max = i64 29
 35139|  %843 = call i64 @llvm.smax.i64(i64 %842, i64 0)                                                                       ;L2025<397
 35140|  %844 = call i64 @llvm.umin.i64(i64 %843, i64 29)                                                                      ;L2025<397
 35141|  %845 = mul nuw nsw i64 %844, 32000                                                                                    ;L397
 35143|  %846 = gep %39, i64 1632                                                                                              ;L399
 35144|  %847 = load i64, ptr %846, , !!8                                                                                      ;L399
 35145|  %848 = sub i64 %839, %847                                                                                             ;L399
 35146|  %849 = add i64 %848, 16000                                                                                            ;L399
 35147|     ;; dx = i64 %849
 35148|  %850 = gep %39, i64 1640                                                                                              ;L400
 35149|  %851 = load i64, ptr %850, , !!8                                                                                      ;L400
 35150|  %852 = sub i64 %845, %851                                                                                             ;L400
 35151|  %853 = add i64 %852, 16000                                                                                            ;L400
 35152|     ;; dy = i64 %853
 35153|  store i32 1, ptr %0,                                                                                                  ;L401
 35154|  %854 = gep %0, i64 8                                                                                                  ;L401
 35155|  store i64 %849, ptr %854,                                                                                             ;L401
 35156|  %855 = gep %0, i64 16                                                                                                 ;L401
 35157|  store i64 %853, ptr %855,                                                                                             ;L401
 35158|  br label %44                                                                                                          ;L1
 35159| }
