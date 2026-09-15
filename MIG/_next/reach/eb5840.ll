 22517| define i64 @ai::plan_legacy8sub_plan9line_waitNtB2_15LineWaitSubPlan5score(ptr %0, i64 %1, ptr %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %7) unnamed_addr #0 {
 22518|  %9 = alloca [40 x i8],
 22519|     ;; self = ptr %0
 22520|     ;; version = i64 %1
 22521|     ;; parameter = ptr %2
 22522|     ;; rnd = ptr %3
 22523|     ;; player = ptr %4
 22524|     ;; data = ptr %5
 22525|     ;; action = ptr %6
 22526|     ;; debug = ptr %7
 22527|     ;; action_type = i8 0
 22529|  %10 = load i8, ptr %0, , !!8                                                                                          ;L160
 22530|  %11 = gep %9, i64 16                                                                                                  ;L160
 22531|  store i8 1, ptr %11,                                                                                                  ;L160
 22532|  %12 = gep %9, i64 17                                                                                                  ;L160
 22533|  store i8 %10, ptr %12,                                                                                                ;L160
 22534|  store i64 2, ptr %9,                                                                                                  ;L160
 22535|  %13 = call { i64, i64 } @ai::plan_legacy11action_eval15evaluate_action(i64 %1, ptr %9, ptr %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %7) ;L160
 22536|  %14 = extractvalue { i64, i64 } %13, 0                                                                                ;L160
 22537|  %15 = trunc nuw i64 %14 to i1                                                                                         ;L160
 22538|  br i1 %15, label %16, label %18                                                                                       ;L160
 22539| 
 22540| 16: ; preds = %8
 22541|  %17 = extractvalue { i64, i64 } %13, 1                                                                                ;L160
 22542|     ;; v = i64 %17
 22544|  br label %22                                                                                                          ;L222
 22545| 
 22546| 18: ; preds = %8
 22548|  %19 = gep %4, i64 2352                                                                                                ;L163
 22549|  %20 = load i64, ptr %19, , !!8                                                                                        ;L163
 22550|  %21 = icmp ult i64 %20, 2                                                                                             ;L163
 22551|  br i1 %21, label %25, label %24                                                                                       ;L163
 22552| 
 22553| 22: ; preds = %86, %16
 22554|  %23 = phi i64 [ %17, %16 ], [ %89, %86 ]                                                                              ;L0
 22555|     ;; v = i64 %23
 22556|  ret i64 %23                                                                                                           ;L222
 22557| 
 22558| 24: ; preds = %18
 22559|  call void @core::panicking18panic_bounds_check(i64 %20, i64 2, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.144) #30    ;L163
 22560|  unreachable                                                                                                           ;L163
 22561| 
 22562| 25: ; preds = %18
 22563|     ;; self = ptr %4
 22564|  %26 = gep %4, i64 2496                                                                                                ;L581<163
 22565|  %27 = load i32, ptr %26, , !!8                                                                                        ;L581<163
 22566|  %28 = zext nneg i32 %27 to i64                                                                                        ;L581<163
 22567|  %29 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L163
 22568|  %30 = gep %29, i64 480                                                                                                ;L163
 22569|  %31 = getelementptr [5 x ptr], ptr %30, i64 %20                                                                       ;L163
 22570|  %32 = getelementptr ptr, ptr %31, i64 %28                                                                             ;L163
 22571|  %33 = load ptr, ptr %32, , !!8                                                                                        ;L163
 22572|     ;; self = ptr %33
 22573|  %34 = icmp eq ptr %33, null                                                                                           ;L1011<163
 22574|  br i1 %34, label %45, label %35                                                                                       ;L1011<163
 22575| 
 22576| 35: ; preds = %25
 22577|     ;; champ = ptr %33
 22578|     ;; self = ptr %33
 22579|     ;; self = ptr %33
 22580|     ;; self = ptr %33
 22581|     ;; self = ptr %33
 22582|     ;; other = ptr %33
 22583|  %36 = call i64 @ai::action_score17interaction_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %6, ptr %7)           ;L164
 22584|     ;; base = i64 %36
 22585|  %37 = call i64 @ai::lane_economy30line_action_economy_adjustment(i64 %1, ptr %4, ptr %5, ptr %2, ptr %6, i8 0)        ;L168
 22586|     ;; economy_adjustment = i64 %37
 22587|     ;; self = ptr %6
 22588|  %38 = gep %6, i64 177                                                                                                 ;L309<169
 22589|  %39 = load i8, ptr %38, , !!36431, !!8                                                                                ;L309<169
 22590|  %40 = icmp ne i8 %39, 10                                                                                              ;L309<169
 22591|  call void @llvm.assume(i1 %40)                                                                                        ;L309<169
 22592|  %41 = add nsw i8 %39, -3                                                                                              ;L309<169
 22593|  %42 = icmp samesign ugt i8 %39, 2                                                                                     ;L309<169
 22594|  %43 = select i1 %42, i8 %41, i8 7                                                                                     ;L309<169
 22595|  switch i8 %43, label %44 [
 22596|  i8 0, label %86
 22597|  i8 1, label %86
 22598|  i8 2, label %46
 22599|  i8 3, label %46
 22600|  i8 4, label %86
 22601|  i8 5, label %86
 22602|  i8 6, label %86
 22603|  i8 7, label %86
 22604|  i8 8, label %86
 22605|  i8 9, label %86
 22606|  i8 10, label %46
 22607|  i8 11, label %86
 22608|  i8 12, label %56
 22609|  i8 13, label %66
 22610|  i8 14, label %76
 22611|  i8 15, label %86
 22612|  i8 16, label %86
 22613|  ]                                                                                                                     ;L309<169
 22614| 
 22615| 44: ; preds = %35
 22616|  unreachable                                                                                                           ;L309<169
 22617| 
 22618| 45: ; preds = %25
 22619|  call void @core::option13unwrap_failed(ptr @anon.e5f9a016f1220e5cee86475419aee1e5.145) #30                            ;L1013<163
 22620|  unreachable                                                                                                           ;L1013<163
 22621| 
 22622| 46: ; preds = %35, %35, %35
 22623|  %47 = gep %6, i64 8                                                                                                   ;L0<169
 22624|  %48 = load i64, ptr %47, , !!36431, !!8                                                                               ;L0<169
 22625|     ;; target_id = i64 %48
 22626|  %49 = load ptr, ptr %29, , !!8, !!8                                                                                   ;L197
 22627|  %50 = gep %29, i64 8                                                                                                  ;L197
 22628|  %51 = load ptr, ptr %50, , !!8, !!8                                                                                   ;L197
 22629|  %52 = gep %51, i64 496                                                                                                ;L197
 22630|  %53 = load ptr, ptr %52, , !!8                                                                                        ;L197
 22631|  %54 = call ptr %53(ptr %49, i64 %48)                                                                                  ;L197
 22632|  %55 = icmp eq ptr %54, null                                                                                           ;L197
 22633|  br i1 %55, label %86, label %90                                                                                       ;L197
 22634| 
 22635| 56: ; preds = %35
 22636|     ;; action = ptr %6
 22637|     ;; self = ptr %6
 22638|  %57 = gep %6, i64 8                                                                                                   ;L94<322<169
 22639|  %58 = load i64, ptr %57, , !!36431, !!8                                                                               ;L94<322<169
 22640|     ;; target_id = i64 %58
 22641|  %59 = load ptr, ptr %29, , !!8, !!8                                                                                   ;L173
 22642|  %60 = gep %29, i64 8                                                                                                  ;L173
 22643|  %61 = load ptr, ptr %60, , !!8, !!8                                                                                   ;L173
 22644|  %62 = gep %61, i64 496                                                                                                ;L173
 22645|  %63 = load ptr, ptr %62, , !!8                                                                                        ;L173
 22646|  %64 = call ptr %63(ptr %59, i64 %58)                                                                                  ;L173
 22647|  %65 = icmp eq ptr %64, null                                                                                           ;L173
 22648|  br i1 %65, label %86, label %128                                                                                      ;L173
 22649| 
 22650| 66: ; preds = %35
 22651|     ;; action = ptr %6
 22652|     ;; self = ptr %6
 22653|  %67 = gep %6, i64 8                                                                                                   ;L160<323<169
 22654|  %68 = load i64, ptr %67, , !!36431, !!8                                                                               ;L160<323<169
 22655|     ;; target_id = i64 %68
 22656|  %69 = load ptr, ptr %29, , !!8, !!8                                                                                   ;L181
 22657|  %70 = gep %29, i64 8                                                                                                  ;L181
 22658|  %71 = load ptr, ptr %70, , !!8, !!8                                                                                   ;L181
 22659|  %72 = gep %71, i64 496                                                                                                ;L181
 22660|  %73 = load ptr, ptr %72, , !!8                                                                                        ;L181
 22661|  %74 = call ptr %73(ptr %69, i64 %68)                                                                                  ;L181
 22662|  %75 = icmp eq ptr %74, null                                                                                           ;L181
 22663|  br i1 %75, label %86, label %138                                                                                      ;L181
 22664| 
 22665| 76: ; preds = %35
 22666|     ;; action = ptr %6
 22667|     ;; self = ptr %6
 22668|  %77 = gep %6, i64 8                                                                                                   ;L222<324<169
 22669|  %78 = load i64, ptr %77, , !!36431, !!8                                                                               ;L222<324<169
 22670|     ;; target_id = i64 %78
 22671|  %79 = load ptr, ptr %29, , !!8, !!8                                                                                   ;L189
 22672|  %80 = gep %29, i64 8                                                                                                  ;L189
 22673|  %81 = load ptr, ptr %80, , !!8, !!8                                                                                   ;L189
 22674|  %82 = gep %81, i64 496                                                                                                ;L189
 22675|  %83 = load ptr, ptr %82, , !!8                                                                                        ;L189
 22676|  %84 = call ptr %83(ptr %79, i64 %78)                                                                                  ;L189
 22677|  %85 = icmp eq ptr %84, null                                                                                           ;L189
 22678|  br i1 %85, label %86, label %148                                                                                      ;L189
 22679| 
 22680| 86: ; preds = %157, %142, %132, %127, %126, %123, %123, %102, %98, %90, %76, %66, %56, %46, %35, %35, %35, %35, %35, %35, %35, %35, %35, %35, %35
 22681|  %87 = phi i64 [ -99999, %66 ], [ -99999, %76 ], [ -99999, %56 ], [ 50, %123 ], [ 0, %127 ], [ 0, %102 ], [ 100, %126 ], [ 0, %98 ], [ 0, %46 ], [ 0, %90 ], [ 50, %123 ], [ %136, %132 ], [ %146, %142 ], [ %161, %157 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ] ;L0
 22682|  %88 = add i64 %37, %36                                                                                                ;L169
 22683|  %89 = add i64 %88, %87                                                                                                ;L169
 22684|     ;; v = i64 %89
 22685|  br label %22                                                                                                          ;L222
 22686| 
 22687| 90: ; preds = %46
 22688|     ;; t = ptr %54
 22689|     ;; self = ptr %54
 22690|     ;; self = ptr %54
 22691|     ;; other = ptr %33
 22692|  %91 = load i64, ptr %54, , !!8                                                                                        ;L1127<198
 22693|  %92 = gep %54, i64 8                                                                                                  ;L1127<198
 22694|     ;; __self_discr = i64 %91
 22695|  %93 = load i64, ptr %33, , !!8                                                                                        ;L1127<198
 22696|  %94 = gep %33, i64 8                                                                                                  ;L1127<198
 22697|     ;; __arg1_discr = i64 %93
 22698|  %95 = icmp eq i64 %91, %93                                                                                            ;L1127<198
 22699|  br i1 %95, label %96, label %86                                                                                       ;L1127<198
 22700| 
 22701| 96: ; preds = %90
 22702|  %97 = icmp eq i64 %91, 0                                                                                              ;L1127<198
 22703|  br i1 %97, label %98, label %102                                                                                      ;L1127<198
 22704| 
 22705| 98: ; preds = %96
 22706|     ;; __self_0 = ptr %54
 22707|     ;; self = ptr %54
 22708|     ;; __arg1_0 = ptr %33
 22709|     ;; other = ptr %33
 22712|  %99 = load i64, ptr %92, , !!8                                                                                        ;L1878<2123<1127<198
 22713|  %100 = load i64, ptr %94, , !!8                                                                                       ;L1878<2123<1127<198
 22714|  %101 = icmp eq i64 %99, %100                                                                                          ;L1878<2123<1127<198
 22715|  br i1 %101, label %102, label %86                                                                                     ;L198
 22716| 
 22717| 102: ; preds = %98, %96
 22718|  %103 = gep %54, i64 1632                                                                                              ;L2158<199
 22719|  %104 = load i64, ptr %103, , !!8                                                                                      ;L2158<199
 22720|     ;; x1 = i64 %104
 22721|     ;; self = i64 %104
 22722|  %105 = gep %54, i64 1640                                                                                              ;L2158<199
 22723|  %106 = load i64, ptr %105, , !!8                                                                                      ;L2158<199
 22724|     ;; y1 = i64 %106
 22725|     ;; self = i64 %106
 22726|  %107 = gep %33, i64 1632                                                                                              ;L2158<199
 22727|  %108 = load i64, ptr %107, , !!8                                                                                      ;L2158<199
 22728|     ;; x2 = i64 %108
 22729|     ;; other = i64 %108
 22730|  %109 = gep %33, i64 1640                                                                                              ;L2158<199
 22731|  %110 = load i64, ptr %109, , !!8                                                                                      ;L2158<199
 22732|     ;; y2 = i64 %110
 22733|     ;; other = i64 %110
 22734|  %111 = icmp ult i64 %104, %108                                                                                        ;L3147<7<2158<199
 22735|  %112 = sub nuw i64 %108, %104                                                                                         ;L3147<7<2158<199
 22736|  %113 = sub nuw i64 %104, %108                                                                                         ;L3147<7<2158<199
 22737|  %114 = select i1 %111, i64 %112, i64 %113                                                                             ;L3147<7<2158<199
 22738|     ;; dx = i64 %114
 22739|  %115 = icmp ult i64 %106, %110                                                                                        ;L3147<8<2158<199
 22740|  %116 = sub nuw i64 %110, %106                                                                                         ;L3147<8<2158<199
 22741|  %117 = sub nuw i64 %106, %110                                                                                         ;L3147<8<2158<199
 22742|  %118 = select i1 %115, i64 %116, i64 %117                                                                             ;L3147<8<2158<199
 22743|     ;; dy = i64 %118
 22744|  %119 = mul i64 %114, %114                                                                                             ;L9<2158<199
 22745|  %120 = mul i64 %118, %118                                                                                             ;L9<2158<199
 22746|  %121 = add i64 %120, %119                                                                                             ;L9<2158<199
 22747|     ;; dist = i64 %121
 22748|  %122 = icmp ugt i64 %121, 39999999999                                                                                 ;L202
 22749|  br i1 %122, label %123, label %86                                                                                     ;L202
 22750| 
 22751| 123: ; preds = %102
 22752|     ;; self = ptr %54
 22753|  %124 = gep %54, i64 104                                                                                               ;L1261<203
 22754|  %125 = load i64, ptr %124, , !!8                                                                                      ;L1261<203
 22755|  switch i64 %125, label %127 [
 22756|  i64 1, label %86
 22757|  i64 2, label %86
 22758|  i64 3, label %126
 22759|  ]                                                                                                                     ;L203
 22760| 
 22761| 126: ; preds = %123
 22762|  br label %86                                                                                                          ;L205
 22763| 
 22764| 127: ; preds = %123
 22765|  br label %86                                                                                                          ;L205
 22766| 
 22767| 128: ; preds = %56
 22768|     ;; t = ptr %64
 22769|     ;; self = ptr %33
 22770|  %129 = gep %33, i64 1216                                                                                              ;L742<174
 22771|  %130 = load i32, ptr %129, , !!8                                                                                      ;L742<174
 22772|  %131 = icmp eq i32 %130, -1                                                                                           ;L742<174
 22773|  br i1 %131, label %137, label %132                                                                                    ;L742<174
 22774| 
 22775| 132: ; preds = %128
 22776|  %133 = gep %33, i64 1168                                                                                              ;L742<174
 22777|     ;; self = ptr %133
 22778|     ;; effect = ptr %133
 22779|  %134 = gep %33, i64 1392                                                                                              ;L1494<175
 22780|  %135 = call i64 @gc::simulation6entityNtB5_6Entity17attack_speed_mult(ptr %33)                                        ;L175
 22781|  %136 = call i64 @ai::action_score22calculate_action_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %134, ptr %133, i64 %135, ptr %64, i8 0, ptr %7) ;L175
 22782|  br label %86                                                                                                          ;L173
 22783| 
 22784| 137: ; preds = %128
 22785|     ;; self = ptr null
 22786|  call void @core::option13unwrap_failed(ptr @anon.e5f9a016f1220e5cee86475419aee1e5.146) #30                            ;L1013<174
 22787|  unreachable                                                                                                           ;L1013<174
 22788| 
 22789| 138: ; preds = %66
 22790|     ;; t = ptr %74
 22791|     ;; self = ptr %33
 22792|  %139 = gep %33, i64 1272                                                                                              ;L742<182
 22793|  %140 = load i32, ptr %139, , !!8                                                                                      ;L742<182
 22794|  %141 = icmp eq i32 %140, -1                                                                                           ;L742<182
 22795|  br i1 %141, label %147, label %142                                                                                    ;L742<182
 22796| 
 22797| 142: ; preds = %138
 22798|  %143 = gep %33, i64 1224                                                                                              ;L742<182
 22799|     ;; self = ptr %143
 22800|     ;; effect = ptr %143
 22801|  %144 = gep %33, i64 1408                                                                                              ;L1665<183
 22802|  %145 = call i64 @gc::simulation6entityNtB5_6Entity15cooldown_reduce(ptr %33, i1 zeroext false)                        ;L183
 22803|  %146 = call i64 @ai::action_score22calculate_action_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %144, ptr %143, i64 %145, ptr %74, i8 0, ptr %7) ;L183
 22804|  br label %86                                                                                                          ;L181
 22805| 
 22806| 147: ; preds = %138
 22807|     ;; self = ptr null
 22808|  call void @core::option13unwrap_failed(ptr @anon.e5f9a016f1220e5cee86475419aee1e5.147) #30                            ;L1013<182
 22809|  unreachable                                                                                                           ;L1013<182
 22810| 
 22811| 148: ; preds = %76
 22812|     ;; t = ptr %84
 22813|  %149 = gep %33, i64 1480                                                                                              ;L1693<190
 22814|  %150 = load i64, ptr %149, , !!8                                                                                      ;L1693<190
 22815|  %151 = icmp ugt i64 %150, 2                                                                                           ;L1693<190
 22816|  br i1 %151, label %152, label %156                                                                                    ;L1693<190
 22817| 
 22818| 152: ; preds = %148
 22819|     ;; self = ptr %33
 22820|  %153 = gep %33, i64 1328                                                                                              ;L742<190
 22821|  %154 = load i32, ptr %153, , !!8                                                                                      ;L742<190
 22822|  %155 = icmp eq i32 %154, -1                                                                                           ;L742<190
 22823|  br i1 %155, label %156, label %157                                                                                    ;L742<190
 22824| 
 22825| 156: ; preds = %152, %148
 22826|  call void @core::option13unwrap_failed(ptr @anon.e5f9a016f1220e5cee86475419aee1e5.148) #30                            ;L1013<190
 22827|  unreachable                                                                                                           ;L1013<190
 22828| 
 22829| 157: ; preds = %152
 22830|  %158 = gep %33, i64 1280                                                                                              ;L1694<190
 22831|     ;; self = ptr %158
 22832|     ;; self = ptr %158
 22833|     ;; effect = ptr %158
 22834|  %159 = gep %33, i64 1424                                                                                              ;L1670<191
 22835|  %160 = call i64 @gc::simulation6entityNtB5_6Entity15cooldown_reduce(ptr %33, i1 zeroext false)                        ;L191
 22836|  %161 = call i64 @ai::action_score22calculate_action_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %159, ptr %158, i64 %160, ptr %84, i8 0, ptr %7) ;L191
 22837|  br label %86                                                                                                          ;L189
 22838| }
