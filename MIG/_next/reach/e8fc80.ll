 38174| define { i64, i64 } @_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster8buy_item(ptr %0, ptr %1, ptr %2, ptr readnone %3, ptr %4, ptr %5) unnamed_addr #0 {
 38175|  %7 = alloca [24 x i8],
 38176|     ;; self = ptr %0
 38177|     ;; rnd = ptr %1
 38178|     ;; player = ptr %2
 38181|     ;; context = ptr %5
 38182|     ;; self = ptr %7
 38183|     ;; self = ptr %2
 38184|     ;; self = ptr %2
 38185|  %8 = gep %2, i64 1264                                                                                                 ;L3054<3079<1174
 38186|  %9 = load i64, ptr %8, , !!8                                                                                          ;L3054<3079<1174
 38187|  %10 = icmp ult i64 %9, 1152921504606846976                                                                            ;L3059<3079<1174
 38188|  tail call void @llvm.assume(i1 %10)                                                                                   ;L3059<3079<1174
 38189|  %11 = icmp eq i64 %9, 0                                                                                               ;L1174
 38190|  br i1 %11, label %12, label %18                                                                                       ;L1174
 38191| 
 38192| 12: ; preds = %6
 38193|  %13 = gep %0, i64 10512                                                                                               ;L1188
 38194|  %14 = load i64, ptr %13, , !!8                                                                                        ;L1188
 38195|  %15 = tail call { i64, i64 } @ai::buy_item(i64 poison, ptr %1, ptr %2, ptr poison, ptr poison, ptr %5)                ;L1188
 38196|  %16 = extractvalue { i64, i64 } %15, 0                                                                                ;L1188
 38197|  %17 = extractvalue { i64, i64 } %15, 1                                                                                ;L1188
 38198|  br label %53                                                                                                          ;L1174
 38199| 
 38200| 18: ; preds = %6
 38202|  %19 = gep %5, i64 48                                                                                                  ;L1175
 38203|  %20 = load ptr, ptr %19, , !!8, !!8                                                                                   ;L1175
 38204|     ;; self = ptr %20
 38205|     ;; self = ptr %20
 38206|     ;; self = ptr %20
 38207|     ;; self = ptr %20
 38208|     ;; self = ptr %20
 38209|     ;; self = ptr %20
 38210|     ;; self = ptr %20
 38211|  %21 = gep %20, i64 8                                                                                                  ;L614<609<296<1968<1864<3787<1175
 38212|  %22 = load ptr, ptr %21, , !!8, !!8                                                                                   ;L614<609<296<1968<1864<3787<1175
 38213|  %23 = gep %20, i64 16                                                                                                 ;L1864<3787<1175
 38214|  %24 = load i64, ptr %23, , !!8                                                                                        ;L1864<3787<1175
 38215|     ;; self[0..+8] = ptr %22
 38216|     ;; slice[0..+8] = ptr %22
 38217|     ;; self[8..+8] = i64 %24
 38218|     ;; slice[8..+8] = i64 %24
 38219|  call fastcc void @_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster13item_v26_slot(ptr %7, ptr %2, ptr %22, i64 %24) ;L1175
 38220|  %25 = gep %7, i64 8                                                                                                   ;L2775<1175
 38221|  %26 = load i64, ptr %25, , !!8                                                                                        ;L2775<1175
 38222|  %27 = icmp eq i64 %26, -1                                                                                             ;L2775<1175
 38223|  br i1 %27, label %33, label %28                                                                                       ;L2775<1175
 38224| 
 38225| 28: ; preds = %18
 38226|  %29 = load i64, ptr %7,                                                                                               ;L2776<1175
 38227|  %30 = gep %7, i64 16                                                                                                  ;L2776<1175
 38228|  %31 = load i64, ptr %30,                                                                                              ;L2776<1175
 38230|     ;; build_slot = i64 %29
 38231|     ;; inventory_index[0..+8] = i64 %26
 38232|     ;; inventory_index[8..+8] = i64 %31
 38233|  %32 = icmp eq i64 %26, 1                                                                                              ;L1176
 38234|  br i1 %32, label %53, label %34                                                                                       ;L1176
 38235| 
 38236| 33: ; preds = %18
 38238|  br label %53                                                                                                          ;L1
 38239| 
 38240| 34: ; preds = %28
 38241|  %35 = tail call fastcc { i64, i64 } @_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster8item_v26(ptr %1, ptr %2, ptr %20, i64 %29, i64 0, i64 %31) ;L1180
 38242|  %36 = extractvalue { i64, i64 } %35, 0                                                                                ;L1180
 38243|  %37 = extractvalue { i64, i64 } %35, 1                                                                                ;L1180
 38244|     ;; self[0..+8] = i64 %36
 38245|     ;; self[8..+8] = i64 %37
 38246|  %38 = trunc nuw i64 %36 to i1                                                                                         ;L2775<1180
 38247|  br i1 %38, label %39, label %53                                                                                       ;L2775<1180
 38248| 
 38249| 39: ; preds = %34
 38250|     ;; item = i64 %37
 38251|     ;; index = i64 %37
 38252|     ;; index = i64 %37
 38253|     ;; self = i64 %37
 38254|  %40 = icmp ult i64 %37, %24                                                                                           ;L272<19<3864<1182
 38255|  br i1 %40, label %41, label %52                                                                                       ;L272<19<3864<1182
 38256| 
 38257| 41: ; preds = %39
 38258|  %42 = getelementptr { { { { { ptr, ptr } } }, {} }, {} }, ptr %22, i64 %37                                            ;L272<19<3864<1182
 38259|  %43 = load ptr, ptr %42, , !!8, !!8                                                                                   ;L1182
 38260|  %44 = gep %42, i64 8                                                                                                  ;L1182
 38261|  %45 = load ptr, ptr %44, , !!8, !!8                                                                                   ;L1182
 38262|  %46 = gep %45, i64 112                                                                                                ;L1182
 38263|  %47 = load ptr, ptr %46, , !!8                                                                                        ;L1182
 38264|  %48 = tail call i64 %47(ptr %43)                                                                                      ;L1182
 38265|  %49 = icmp eq i64 %48, 0                                                                                              ;L1182
 38266|  %50 = select i1 %49, i64 %37, i64 undef                                                                               ;L0
 38267|  %51 = zext i1 %49 to i64                                                                                              ;L0
 38268|  br label %53                                                                                                          ;L0
 38269| 
 38270| 52: ; preds = %39
 38271|  tail call void @core::panicking18panic_bounds_check(i64 %37, i64 %24, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.152) #35 ;L272<19<3864<1182
 38272|  unreachable                                                                                                           ;L272<19<3864<1182
 38273| 
 38274| 53: ; preds = %41, %34, %33, %28, %12
 38275|  %54 = phi i64 [ %17, %12 ], [ undef, %33 ], [ undef, %34 ], [ %50, %41 ], [ undef, %28 ]                              ;L0
 38276|  %55 = phi i64 [ %16, %12 ], [ 0, %33 ], [ 0, %34 ], [ %51, %41 ], [ 0, %28 ]                                          ;L0
 38277|  %56 = insertvalue { i64, i64 } poison, i64 %55, 0                                                                     ;L1190
 38278|  %57 = insertvalue { i64, i64 } %56, i64 %54, 1                                                                        ;L1190
 38279|  ret { i64, i64 } %57                                                                                                  ;L1190
 38280| }
