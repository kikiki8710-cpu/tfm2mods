 35109| define void @_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster12upgrade_item(ptr sret([24 x i8]) %0, ptr %1, ptr %2, ptr %3, ptr readnone %4, ptr %5, ptr %6) unnamed_addr #0 {
 35110|  %8 = alloca [24 x i8],
 35111|     ;; self = ptr %1
 35112|     ;; rnd = ptr %2
 35113|     ;; player = ptr %3
 35116|     ;; context = ptr %6
 35117|     ;; self = ptr %8
 35121|     ;; self = ptr %3
 35122|     ;; self = ptr %3
 35123|  %9 = gep %3, i64 1264                                                                                                 ;L3054<3079<1194
 35124|  %10 = load i64, ptr %9, , !!8                                                                                         ;L3054<3079<1194
 35125|  %11 = icmp ult i64 %10, 1152921504606846976                                                                           ;L3059<3079<1194
 35126|  tail call void @llvm.assume(i1 %11)                                                                                   ;L3059<3079<1194
 35127|  %12 = icmp eq i64 %10, 0                                                                                              ;L1194
 35128|  br i1 %12, label %13, label %16                                                                                       ;L1194
 35129| 
 35130| 13: ; preds = %7
 35131|  %14 = gep %1, i64 10512                                                                                               ;L1205
 35132|  %15 = load i64, ptr %14, , !!8                                                                                        ;L1205
 35133|  tail call void @ai::upgrade_item(ptr sret([24 x i8]) %0, i64 poison, ptr %2, ptr %3, ptr poison, ptr poison, ptr %6)  ;L1205
 35134|  br label %55                                                                                                          ;L1194
 35135| 
 35136| 16: ; preds = %7
 35138|  %17 = gep %6, i64 48                                                                                                  ;L1195
 35139|  %18 = load ptr, ptr %17, , !!8, !!8                                                                                   ;L1195
 35140|     ;; self = ptr %18
 35141|     ;; self = ptr %18
 35142|     ;; self = ptr %18
 35143|     ;; self = ptr %18
 35144|     ;; self = ptr %18
 35145|     ;; self = ptr %18
 35146|     ;; self = ptr %18
 35147|  %19 = gep %18, i64 8                                                                                                  ;L614<609<296<1968<1864<3787<1195
 35148|  %20 = load ptr, ptr %19, , !!8, !!8                                                                                   ;L614<609<296<1968<1864<3787<1195
 35149|  %21 = gep %18, i64 16                                                                                                 ;L1864<3787<1195
 35150|  %22 = load i64, ptr %21, , !!8                                                                                        ;L1864<3787<1195
 35151|     ;; self[0..+8] = ptr %20
 35152|     ;; slice[0..+8] = ptr %20
 35153|     ;; self[8..+8] = i64 %22
 35154|     ;; slice[8..+8] = i64 %22
 35155|  call fastcc void @_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster13item_v26_slot(ptr %8, ptr %3, ptr %20, i64 %22) ;L1195
 35156|  %23 = gep %8, i64 8                                                                                                   ;L2775<1195
 35157|  %24 = load i64, ptr %23, , !!8                                                                                        ;L2775<1195
 35158|  %25 = icmp eq i64 %24, -1                                                                                             ;L2775<1195
 35159|  br i1 %25, label %31, label %26                                                                                       ;L2775<1195
 35160| 
 35161| 26: ; preds = %16
 35162|  %27 = load i64, ptr %8,                                                                                               ;L2776<1195
 35163|  %28 = gep %8, i64 16                                                                                                  ;L2776<1195
 35164|  %29 = load i64, ptr %28,                                                                                              ;L2776<1195
 35166|     ;; build_slot = i64 %27
 35167|     ;; inventory_index[0..+8] = i64 %24
 35168|     ;; self[0..+8] = i64 %24
 35169|     ;; inventory_index[8..+8] = i64 %29
 35170|     ;; self[8..+8] = i64 %29
 35171|  %30 = trunc nuw i64 %24 to i1                                                                                         ;L2775<1196
 35172|  br i1 %30, label %32, label %37                                                                                       ;L2775<1196
 35173| 
 35174| 31: ; preds = %16
 35176|  store i64 0, ptr %0,                                                                                                  ;L2790<1195
 35177|  br label %55                                                                                                          ;L1
 35178| 
 35179| 32: ; preds = %26
 35180|     ;; inventory_index = i64 %29
 35181|  %33 = tail call fastcc { i64, i64 } @_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster8item_v26(ptr %2, ptr %3, ptr %18, i64 %27, i64 1, i64 %29) ;L1198
 35182|  %34 = extractvalue { i64, i64 } %33, 0                                                                                ;L1198
 35183|  %35 = extractvalue { i64, i64 } %33, 1                                                                                ;L1198
 35184|     ;; self[0..+8] = i64 %34
 35185|     ;; self[8..+8] = i64 %35
 35186|  %36 = trunc nuw i64 %34 to i1                                                                                         ;L2775<1198
 35187|  br i1 %36, label %38, label %40                                                                                       ;L2775<1198
 35188| 
 35189| 37: ; preds = %26
 35190|  store i64 0, ptr %0,                                                                                                  ;L2790<1196
 35191|  br label %55                                                                                                          ;L1
 35192| 
 35193| 38: ; preds = %32
 35194|     ;; item = i64 %35
 35195|     ;; index = i64 %35
 35196|     ;; index = i64 %35
 35197|     ;; self = i64 %35
 35198|  %39 = icmp ult i64 %35, %22                                                                                           ;L272<19<3864<1199
 35199|  br i1 %39, label %41, label %50                                                                                       ;L272<19<3864<1199
 35200| 
 35201| 40: ; preds = %32
 35202|  store i64 0, ptr %0,                                                                                                  ;L2790<1198
 35203|  br label %55                                                                                                          ;L1
 35204| 
 35205| 41: ; preds = %38
 35206|  %42 = getelementptr { { { { { ptr, ptr } } }, {} }, {} }, ptr %20, i64 %35                                            ;L272<19<3864<1199
 35207|  %43 = load ptr, ptr %42, , !!8, !!8                                                                                   ;L1199
 35208|  %44 = gep %42, i64 8                                                                                                  ;L1199
 35209|  %45 = load ptr, ptr %44, , !!8, !!8                                                                                   ;L1199
 35210|  %46 = gep %45, i64 112                                                                                                ;L1199
 35211|  %47 = load ptr, ptr %46, , !!8                                                                                        ;L1199
 35212|  %48 = tail call i64 %47(ptr %43)                                                                                      ;L1199
 35213|  %49 = icmp eq i64 %48, 0                                                                                              ;L1199
 35214|  br i1 %49, label %51, label %52                                                                                       ;L1199
 35215| 
 35216| 50: ; preds = %38
 35217|  tail call void @core::panicking18panic_bounds_check(i64 %35, i64 %22, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.145) #35 ;L272<19<3864<1199
 35218|  unreachable                                                                                                           ;L272<19<3864<1199
 35219| 
 35220| 51: ; preds = %41
 35221|  store i64 0, ptr %0,                                                                                                  ;L1200
 35222|  br label %55                                                                                                          ;L1
 35223| 
 35224| 52: ; preds = %41
 35225|  %53 = gep %0, i64 8                                                                                                   ;L1203
 35226|  store i64 %29, ptr %53,                                                                                               ;L1203
 35227|  %54 = gep %0, i64 16                                                                                                  ;L1203
 35228|  store i64 %35, ptr %54,                                                                                               ;L1203
 35229|  store i64 1, ptr %0,                                                                                                  ;L1203
 35230|  br label %55                                                                                                          ;L1194
 35231| 
 35232| 55: ; preds = %52, %51, %40, %37, %31, %13
 35233|  ret void                                                                                                              ;L1207
 35234| }
