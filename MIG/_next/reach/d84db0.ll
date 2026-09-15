 24507| define void @ai::position_eval16position_eval_at(ptr sret([56 x i8]) %0, i64 %1, ptr %2, ptr %3, i64 %4, i64 %5, i8 %6) unnamed_addr #0 {
 24508|  %8 = alloca [40 x i8],
 24509|  %9 = alloca [56 x i8],
 24510|  %10 = alloca [32 x i8],
 24511|  %11 = alloca [56 x i8],
 24512|  %12 = alloca [8 x i8],
 24513|  %13 = alloca [40 x i8],
 24514|  %14 = alloca [8 x i8],
 24515|  %15 = alloca [8 x i8],
 24516|     ;; v = ptr %0
 24517|     ;; version = i64 %1
 24518|     ;; player = ptr %2
 24519|     ;; data = ptr %3
 24520|     ;; x = i64 %4
 24521|     ;; y = i64 %5
 24522|     ;; purpose = i8 %6
 24523|     ;; seed = ptr %15
 24524|     ;; tick = ptr %14
 24525|     ;; key = ptr %13
 24526|     ;; idx = ptr %12
 24527|     ;; cached = ptr %11
 24528|     ;; v = ptr %9
 24530|  %16 = load ptr, ptr %3, , !!8, !!8                                                                                    ;L293
 24531|  %17 = load ptr, ptr %16, , !!8, !!8                                                                                   ;L293
 24532|  %18 = gep %16, i64 8                                                                                                  ;L293
 24533|  %19 = load ptr, ptr %18, , !!8, !!8                                                                                   ;L293
 24534|  %20 = gep %19, i64 32                                                                                                 ;L293
 24535|  %21 = load ptr, ptr %20, , !!8                                                                                        ;L293
 24536|  %22 = tail call i64 %21(ptr %17)                                                                                      ;L293
 24537|  store i64 %22, ptr %15,                                                                                               ;L293
 24539|  %23 = gep %19, i64 40                                                                                                 ;L294
 24540|  %24 = load ptr, ptr %23, , !!8                                                                                        ;L294
 24541|  %25 = tail call i64 %24(ptr %17)                                                                                      ;L294
 24542|  store i64 %25, ptr %14,                                                                                               ;L294
 24544|  %26 = gep %2, i64 2344                                                                                                ;L295
 24545|  %27 = load i64, ptr %26, , !!8                                                                                        ;L295
 24546|  store i64 %27, ptr %13,                                                                                               ;L295
 24547|  %28 = gep %13, i64 8                                                                                                  ;L295
 24548|  store i64 %4, ptr %28,                                                                                                ;L295
 24549|  %29 = gep %13, i64 16                                                                                                 ;L295
 24550|  store i64 %5, ptr %29,                                                                                                ;L295
 24551|  %30 = gep %13, i64 24                                                                                                 ;L295
 24552|  store i8 %6, ptr %30,                                                                                                 ;L295
 24553|  %31 = gep %13, i64 32                                                                                                 ;L295
 24554|  store i64 %1, ptr %31,                                                                                                ;L295
 24556|     ;; purpose = i8 %6
 24557|     ;; p = i8 %6
 24558|     ;; id = i64 %27
 24559|     ;; x = i64 %4
 24560|     ;; y = i64 %5
 24561|     ;; rhs = i64 -7046029254386353131
 24562|  %32 = icmp ne i8 %6, 9                                                                                                ;L83<101<296
 24563|  tail call void @llvm.assume(i1 %32)                                                                                   ;L83<101<296
 24564|  %33 = add nsw i8 %6, -2                                                                                               ;L83<101<296
 24565|  %34 = icmp samesign ugt i8 %6, 1                                                                                      ;L83<101<296
 24566|  %35 = select i1 %34, i8 %33, i8 7                                                                                     ;L83<101<296
 24567|  switch i8 %35, label %36 [
 24568|  i8 0, label %49
 24569|  i8 1, label %37
 24570|  i8 2, label %38
 24571|  i8 3, label %39
 24572|  i8 4, label %40
 24573|  i8 5, label %41
 24574|  i8 6, label %42
 24575|  i8 7, label %43
 24576|  i8 8, label %46
 24577|  i8 9, label %47
 24578|  i8 10, label %48
 24579|  ]                                                                                                                     ;L83<101<296
 24580| 
 24581| 36: ; preds = %7
 24582|  unreachable                                                                                                           ;L83<101<296
 24583| 
 24584| 37: ; preds = %7
 24585|  br label %49                                                                                                          ;L85<101<296
 24586| 
 24587| 38: ; preds = %7
 24588|  br label %49                                                                                                          ;L86<101<296
 24589| 
 24590| 39: ; preds = %7
 24591|  br label %49                                                                                                          ;L87<101<296
 24592| 
 24593| 40: ; preds = %7
 24594|  br label %49                                                                                                          ;L88<101<296
 24595| 
 24596| 41: ; preds = %7
 24597|  br label %49                                                                                                          ;L89<101<296
 24598| 
 24599| 42: ; preds = %7
 24600|  br label %49                                                                                                          ;L90<101<296
 24601| 
 24602| 43: ; preds = %7
 24603|  %44 = trunc nuw i8 %6 to i1                                                                                           ;L83<101<296
 24604|  %45 = select i1 %44, i64 4608, i64 4096                                                                               ;L0<101<296
 24605|  br label %49                                                                                                          ;L0<101<296
 24606| 
 24607| 46: ; preds = %7
 24608|  br label %49                                                                                                          ;L93<101<296
 24609| 
 24610| 47: ; preds = %7
 24611|  br label %49                                                                                                          ;L94<101<296
 24612| 
 24613| 48: ; preds = %7
 24614|  br label %49                                                                                                          ;L95<101<296
 24615| 
 24616| 49: ; preds = %48, %47, %46, %43, %42, %41, %40, %39, %38, %37, %7
 24617|  %50 = phi i64 [ 6144, %48 ], [ 512, %37 ], [ 1024, %38 ], [ 1536, %39 ], [ 2048, %40 ], [ 2560, %41 ], [ 3072, %42 ], [ %45, %43 ], [ 0, %7 ], [ 5120, %46 ], [ 5632, %47 ] ;L0<101<296
 24618|  %51 = shl i64 %5, 21                                                                                                  ;L101<296
 24619|  %52 = shl i64 %27, 42                                                                                                 ;L101<296
 24620|  %53 = xor i64 %52, %51                                                                                                ;L101<296
 24621|  %54 = or disjoint i64 %50, %53                                                                                        ;L101<296
 24622|  %55 = xor i64 %54, %4                                                                                                 ;L101<296
 24623|     ;; h = i64 %55
 24624|     ;; self = i64 %55
 24625|  %56 = mul i64 %55, -7046029254386353131                                                                               ;L2660<102<296
 24626|  %57 = lshr i64 %56, 55                                                                                                ;L102<296
 24627|  store i64 %57, ptr %12,                                                                                               ;L296
 24630|  store ptr %15, ptr %10,                                                                                               ;L298
 24631|  %58 = gep %10, i64 8                                                                                                  ;L298
 24632|  store ptr %12, ptr %58,                                                                                               ;L298
 24633|  %59 = gep %10, i64 16                                                                                                 ;L298
 24634|  store ptr %14, ptr %59,                                                                                               ;L298
 24635|  %60 = gep %10, i64 24                                                                                                 ;L298
 24636|  store ptr %13, ptr %60,                                                                                               ;L298
 24637|  call void @core::cell7RefCellNtNtCshdEBA0ozCnw_7game_ai13position_eval12PosEvalCacheEE4withNCNvB1x_16position_eval_at0INtNtBZ_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation12ai_interface16PositioningScoreEEB1z_(ptr sret([56 x i8]) %11, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.97, ptr %10) ;L298
 24639|  %61 = gep %11, i64 49                                                                                                 ;L308
 24640|  %62 = load i8, ptr %61, , !!8                                                                                         ;L308
 24641|  %63 = icmp eq i8 %62, 2                                                                                               ;L308
 24642|  br i1 %63, label %69, label %64                                                                                       ;L308
 24643| 
 24644| 64: ; preds = %49
 24645|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %11, i64 56, i1 false)                                                   ;L308
 24646|     ;; phase = i64 90
 24647|     ;; order = i8 0
 24648|     ;; val = i64 1
 24649|     ;; order = i8 0
 24650|     ;; val = i64 1
 24651|     ;; order = i8 0
 24652|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 24653|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 24654|     ;; order = i8 0
 24655|  %65 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                     ;L3904<741<160<309
 24656|  %66 = icmp eq i8 %65, 0                                                                                               ;L160<309
 24657|  br i1 %66, label %79, label %67                                                                                       ;L160<309
 24658| 
 24659| 67: ; preds = %64
 24660|     ;; self = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 720)
 24661|     ;; dst = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 720)
 24662|  %68 = atomicrmw add ptr getelementptr (i8, ptr @gc::simulation4prof11PHASE_CALLS, i64 720), i64 1 monotonic,          ;L3937<3162<161<309
 24663|  br label %79                                                                                                          ;L160<309
 24664| 
 24665| 69: ; preds = %49
 24666|     ;; phase = i64 91
 24667|     ;; order = i8 0
 24668|     ;; val = i64 1
 24669|     ;; order = i8 0
 24670|     ;; val = i64 1
 24671|     ;; order = i8 0
 24672|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 24673|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 24674|     ;; order = i8 0
 24675|  %70 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                     ;L3904<741<160<312
 24676|  %71 = icmp eq i8 %70, 0                                                                                               ;L160<312
 24677|  br i1 %71, label %74, label %72                                                                                       ;L160<312
 24678| 
 24679| 72: ; preds = %69
 24680|     ;; self = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 728)
 24681|     ;; dst = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 728)
 24682|  %73 = atomicrmw add ptr getelementptr (i8, ptr @gc::simulation4prof11PHASE_CALLS, i64 728), i64 1 monotonic,          ;L3937<3162<161<312
 24683|  br label %74                                                                                                          ;L160<312
 24684| 
 24685| 74: ; preds = %72, %69
 24687|  call fastcc void @ai::position_eval25position_eval_at_uncached(ptr %9, i64 %1, ptr %2, ptr %3, i64 %4, i64 %5, i8 %6) ;L314
 24689|  store ptr %15, ptr %8,                                                                                                ;L316
 24690|  %75 = gep %8, i64 8                                                                                                   ;L316
 24691|  store ptr %12, ptr %75,                                                                                               ;L316
 24692|  %76 = gep %8, i64 16                                                                                                  ;L316
 24693|  store ptr %13, ptr %76,                                                                                               ;L316
 24694|  %77 = gep %8, i64 24                                                                                                  ;L316
 24695|  store ptr %14, ptr %77,                                                                                               ;L316
 24696|  %78 = gep %8, i64 32                                                                                                  ;L316
 24697|  store ptr %9, ptr %78,                                                                                                ;L316
 24698|  call void @core::cell7RefCellNtNtCshdEBA0ozCnw_7game_ai13position_eval12PosEvalCacheEE4withNCNvB1x_16position_eval_ats_0uEB1z_(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.97, ptr %8) ;L316
 24700|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %9, i64 56, i1 false)                                                    ;L323
 24702|  br label %79                                                                                                          ;L324
 24703| 
 24704| 79: ; preds = %74, %67, %64
 24710|  ret void                                                                                                              ;L324
 24711| }
