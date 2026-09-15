 31556| define i64 @ai::plan_legacy8sub_plan12serpen_checkNtB2_18SerpenCheckSubPlan5score(ptr %0, i64 %1, ptr %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %7) unnamed_addr #0 {
 31557|     ;; self = ptr %0
 31558|     ;; version = i64 %1
 31559|     ;; parameter = ptr %2
 31560|     ;; rnd = ptr %3
 31561|     ;; player = ptr %4
 31562|     ;; data = ptr %5
 31563|     ;; action = ptr %6
 31564|     ;; debug = ptr %7
 31565|     ;; index = i64 0
 31566|     ;; self = i64 0
 31567|  %9 = tail call i64 @ai::action_score17interaction_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %6, ptr %7)       ;L124
 31568|     ;; self = ptr %6
 31569|  %10 = gep %6, i64 177                                                                                                 ;L309<125
 31570|  %11 = load i8, ptr %10, , !!45119, !!8                                                                                ;L309<125
 31571|  %12 = icmp ne i8 %11, 10                                                                                              ;L309<125
 31572|  tail call void @llvm.assume(i1 %12)                                                                                   ;L309<125
 31573|  %13 = add nsw i8 %11, -3                                                                                              ;L309<125
 31574|  %14 = icmp samesign ugt i8 %11, 2                                                                                     ;L309<125
 31575|  %15 = select i1 %14, i8 %13, i8 7                                                                                     ;L309<125
 31576|  switch i8 %15, label %16 [
 31577|  i8 0, label %54
 31578|  i8 1, label %54
 31579|  i8 2, label %17
 31580|  i8 3, label %17
 31581|  i8 4, label %54
 31582|  i8 5, label %54
 31583|  i8 6, label %54
 31584|  i8 7, label %54
 31585|  i8 8, label %54
 31586|  i8 9, label %54
 31587|  i8 10, label %17
 31588|  i8 11, label %54
 31589|  i8 12, label %54
 31590|  i8 13, label %54
 31591|  i8 14, label %54
 31592|  i8 15, label %54
 31593|  i8 16, label %54
 31594|  ]                                                                                                                     ;L309<125
 31595| 
 31596| 16: ; preds = %8
 31597|  unreachable                                                                                                           ;L309<125
 31598| 
 31599| 17: ; preds = %8, %8, %8
 31600|  %18 = gep %6, i64 8                                                                                                   ;L0<125
 31601|  %19 = load i64, ptr %18, , !!45119, !!8                                                                               ;L0<125
 31602|     ;; target_id = i64 %19
 31603|  %20 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L127
 31604|  %21 = load ptr, ptr %20, , !!8, !!8                                                                                   ;L127
 31605|  %22 = gep %20, i64 8                                                                                                  ;L127
 31606|  %23 = load ptr, ptr %22, , !!8, !!8                                                                                   ;L127
 31607|  %24 = gep %23, i64 64                                                                                                 ;L127
 31608|  %25 = load ptr, ptr %24, , !!8                                                                                        ;L127
 31609|  %26 = tail call { i64, ptr } %25(ptr %21)                                                                             ;L127
 31610|  %27 = extractvalue { i64, ptr } %26, 0                                                                                ;L127
 31611|     ;; self[0..+8] = i64 %27
 31613|  %28 = icmp eq i64 %27, 0                                                                                              ;L231<127
 31614|  br i1 %28, label %29, label %57                                                                                       ;L231<127
 31615| 
 31616| 29: ; preds = %17
 31617|  %30 = extractvalue { i64, ptr } %26, 1                                                                                ;L127
 31618|     ;; self[8..+8] = ptr %30
 31619|  %31 = icmp ne ptr %30, null
 31620|  tail call void @llvm.assume(i1 %31)
 31621|     ;; self = ptr %30
 31622|     ;; self = ptr %30
 31623|     ;; self = ptr %30
 31624|     ;; self = ptr %30
 31625|  %32 = gep %30, i64 472                                                                                                ;L1864<3787<127
 31626|  %33 = load i64, ptr %32, , !!8                                                                                        ;L1864<3787<127
 31629|     ;; self[8..+8] = i64 %33
 31630|     ;; slice[8..+8] = i64 %33
 31631|  %34 = icmp eq i64 %33, 0                                                                                              ;L219<576<127
 31632|  br i1 %34, label %54, label %35                                                                                       ;L219<576<127
 31633| 
 31634| 35: ; preds = %29
 31635|  %36 = gep %30, i64 464                                                                                                ;L614<609<296<1968<1864<3787<127
 31636|  %37 = load ptr, ptr %36, , !!8, !!8                                                                                   ;L614<609<296<1968<1864<3787<127
 31637|     ;; self[0..+8] = ptr %37
 31638|     ;; slice[0..+8] = ptr %37
 31639|     ;; self = ptr %37
 31640|     ;; f[0..+8] = ptr %21
 31641|     ;; f[8..+8] = ptr %23
 31642|     ;; x = ptr %37
 31643|  %38 = gep %23, i64 496                                                                                                ;L1543<127
 31644|  %39 = load ptr, ptr %38,                                                                                              ;L1543<127
 31645|  %40 = load i64, ptr %37, , !!8                                                                                        ;L1543<127
 31647|  %41 = tail call ptr %39(ptr %21, i64 %40)                                                                             ;L127<1543<127
 31648|  %42 = icmp eq ptr %41, null                                                                                           ;L127
 31649|  br i1 %42, label %54, label %43                                                                                       ;L127
 31650| 
 31651| 43: ; preds = %35
 31652|     ;; serpen = ptr %41
 31653|  %44 = gep %41, i64 1472                                                                                               ;L128
 31654|  %45 = load i64, ptr %44, , !!8                                                                                        ;L128
 31655|  %46 = icmp eq i64 %45, %19                                                                                            ;L128
 31656|  br i1 %46, label %47, label %54                                                                                       ;L128
 31657| 
 31658| 47: ; preds = %43
 31659|  %48 = gep %4, i64 2352                                                                                                ;L128
 31660|  %49 = load i64, ptr %48, , !!8                                                                                        ;L128
 31661|  %50 = gep %23, i64 248                                                                                                ;L128
 31662|  %51 = load ptr, ptr %50, , !!8                                                                                        ;L128
 31663|  %52 = tail call zeroext i1 %51(ptr %21, i64 %49, i64 %19)                                                             ;L128
 31664|  %53 = select i1 %52, i64 0, i64 10                                                                                    ;L128
 31665|  br label %54                                                                                                          ;L128
 31666| 
 31667| 54: ; preds = %47, %43, %35, %29, %8, %8, %8, %8, %8, %8, %8, %8, %8, %8, %8, %8, %8, %8
 31668|  %55 = phi i64 [ %53, %47 ], [ 0, %35 ], [ 0, %43 ], [ 0, %29 ], [ 0, %8 ], [ 0, %8 ], [ 0, %8 ], [ 0, %8 ], [ 0, %8 ], [ 0, %8 ], [ 0, %8 ], [ 0, %8 ], [ 0, %8 ], [ 0, %8 ], [ 0, %8 ], [ 0, %8 ], [ 0, %8 ], [ 0, %8 ] ;L0
 31669|  %56 = add i64 %55, %9                                                                                                 ;L124
 31670|  ret i64 %56                                                                                                           ;L141
 31671| 
 31672| 57: ; preds = %17
 31673|  tail call void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.143) #35                       ;L1013<127
 31674|  unreachable                                                                                                           ;L1013<127
 31675| }
