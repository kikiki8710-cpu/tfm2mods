define internal fastcc void @ai::plan_legacy3old11fight_model18resolve_fight_full(ptr %0, i64 %1, ptr %2, ptr %3, ptr %4, i64 %5, ptr %6, i64 %7, i8 %8, ptr %9, i64 %10, ptr %11, i64 %12, i64 %13) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
 %15 = alloca [32 x i8],
 %16 = alloca [64 x i8],
 %17 = alloca [24 x i8],
 %18 = alloca [64 x i8],
 %19 = alloca [8 x i8],
 %20 = alloca [8 x i8],
 %21 = alloca [184 x i8],
 %22 = alloca [24 x i8],
    ;; tower = ptr %9
    ;; self = ptr %9
    ;; v = ptr %0
    ;; version = i64 %1
    ;; data = ptr %2
    ;; champ = ptr %3
    ;; near_allies[0..+8] = ptr %4
    ;; near_allies[8..+8] = i64 %5
    ;; near_enemies[0..+8] = ptr %6
    ;; near_enemies[8..+8] = i64 %7
    ;; committed_dir = i8 %8
    ;; judge_accuracy = i64 %10
    ;; arrivals[0..+8] = ptr %11
    ;; arrivals[8..+8] = i64 %12
    ;; baseline = i64 %13
    ;; _t = ptr %22
    ;; key = ptr %21
    ;; seed = ptr %20
    ;; tick = ptr %19
    ;; cached = ptr %18
    ;; v = ptr %16
    ;; phase = i64 50
    ;; order = i8 0
    ;; default = i64 -1
    ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
    ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
    ;; order = i8 0
 %23 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                     ;L3904<741<176<325
 %24 = icmp eq i8 %23, 0                                                                                               ;L176<325
 br i1 %24, label %30, label %25                                                                                       ;L176<325

25: ; preds = %14
 %26 = tail call { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()                                       ;L179<325
 %27 = extractvalue { i64, i32 } %26, 0                                                                                ;L179<325
 %28 = extractvalue { i64, i32 } %26, 1                                                                                ;L179<325
 store i64 50, ptr %22,                                                                                                ;L179<325
 %29 = gep %22, i64 8                                                                                                  ;L179<325
 store i64 %27, ptr %29,                                                                                               ;L179<325
 br label %30                                                                                                          ;L180<325

30: ; preds = %25, %14
 %31 = phi i32 [ %28, %25 ], [ -1, %14 ]                                                                               ;L0<325
 %32 = gep %22, i64 16                                                                                                 ;L0<325
 store i32 %31, ptr %32,                                                                                               ;L0<325
 %33 = icmp samesign ugt i64 %5, 8                                                                                     ;L326
 %34 = icmp samesign ugt i64 %7, 8                                                                                     ;L326
 %35 = select i1 %33, i1 true, i1 %34                                                                                  ;L326
 br i1 %35, label %36, label %40                                                                                       ;L326

36: ; preds = %30
 %37 = load ptr, ptr %2,                                                                                               ;L327
 %38 = gep %2, i64 8                                                                                                   ;L327
 %39 = load ptr, ptr %38, , !!8, !!8                                                                                   ;L327
 invoke fastcc void @ai::plan_legacy3old11fight_model22resolve_fight_uncached(ptr %0, i64 %1, ptr %37, ptr %39, ptr %3, ptr %4, i64 %5, ptr %6, i64 %7, i8 %8, ptr %9, i64 %10, ptr %11, i64 %12, i64 %13)
 to label %135 unwind label %50                                                                                        ;L327

40: ; preds = %30
 %41 = gep %3, i64 1472                                                                                                ;L331
 %42 = load i64, ptr %41, , !!8                                                                                        ;L331
 %43 = icmp eq ptr %9, null                                                                                            ;L1161<333
 br i1 %43, label %47, label %44                                                                                       ;L1161<333

44: ; preds = %40
    ;; x = ptr %9
    ;; t = ptr %9
 %45 = gep %9, i64 1472                                                                                                ;L333<1162<333
 %46 = load i64, ptr %45, , !!8                                                                                        ;L333<1162<333
    ;; self[8..+8] = i64 %46
    ;; self[0..+8] = i64 1
 br label %47                                                                                                          ;L1043<333

47: ; preds = %44, %40
 %48 = phi i64 [ %46, %44 ], [ -1, %40 ]                                                                               ;L0<333
    ;; a[0..+1] = i8 0
    ;; a[1..+1] = i8 0
    ;; a[2..+1] = i8 0
    ;; a[3..+1] = i8 0
    ;; a[4..+1] = i8 0
    ;; a[5..+1] = i8 0
    ;; a[6..+1] = i8 0
    ;; a[7..+1] = i8 0
    ;; iter[8..+8] = !DIArgList(ptr %11, i64 %12)
    ;; self = ptr undef
    ;; self = ptr undef
    ;; iter[0..+8] = ptr %11
    ;; iter[24..+8] = i64 0
    ;; iter[16..+8] = i64 7
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %11
    ;; self = ptr %11
    ;; end_or_len = !DIArgList(ptr %11, i64 %12)
 %49 = icmp eq i64 %12, 0                                                                                              ;L1714<180<39<80<336
 br i1 %49, label %52, label %267                                                                                      ;L180<39<80<336

50: ; preds = %145, %142, %118, %114, %106, %36
 %51 = cleanuppad within none []
 call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %22) #28 [ "funclet"(token %51) ] ;L376
 cleanupret from %51 unwind to caller                                                                                  ;L324

52: ; preds = %329, %320, %311, %302, %293, %284, %275, %267, %47
 %53 = phi i64 [ 0, %47 ], [ 0, %267 ], [ 0, %275 ], [ 0, %284 ], [ 0, %293 ], [ 0, %302 ], [ 0, %311 ], [ 0, %320 ], [ %336, %329 ] ;L0
 %54 = phi i8 [ 0, %47 ], [ 0, %267 ], [ 0, %275 ], [ 0, %284 ], [ 0, %293 ], [ 0, %302 ], [ 0, %311 ], [ %327, %320 ], [ %327, %329 ] ;L0
 %55 = phi i8 [ 0, %47 ], [ 0, %267 ], [ 0, %275 ], [ 0, %284 ], [ 0, %293 ], [ 0, %302 ], [ %318, %311 ], [ %318, %320 ], [ %318, %329 ] ;L0
 %56 = phi i8 [ 0, %47 ], [ 0, %267 ], [ 0, %275 ], [ 0, %284 ], [ 0, %293 ], [ %309, %302 ], [ %309, %311 ], [ %309, %320 ], [ %309, %329 ] ;L0
 %57 = phi i8 [ 0, %47 ], [ 0, %267 ], [ 0, %275 ], [ 0, %284 ], [ %300, %293 ], [ %300, %302 ], [ %300, %311 ], [ %300, %320 ], [ %300, %329 ] ;L0
 %58 = phi i8 [ 0, %47 ], [ 0, %267 ], [ 0, %275 ], [ %291, %284 ], [ %291, %293 ], [ %291, %302 ], [ %291, %311 ], [ %291, %320 ], [ %291, %329 ] ;L0
 %59 = phi i8 [ 0, %47 ], [ 0, %267 ], [ %282, %275 ], [ %282, %284 ], [ %282, %293 ], [ %282, %302 ], [ %282, %311 ], [ %282, %320 ], [ %282, %329 ] ;L0
 %60 = phi i8 [ 0, %47 ], [ %273, %267 ], [ %273, %275 ], [ %273, %284 ], [ %273, %293 ], [ %273, %302 ], [ %273, %311 ], [ %273, %320 ], [ %273, %329 ] ;L0
    ;; a[0..+1] = i8 %60
    ;; a[1..+1] = i8 %59
    ;; a[2..+1] = i8 %58
    ;; a[3..+1] = i8 %57
    ;; a[4..+1] = i8 %56
    ;; a[5..+1] = i8 %55
    ;; a[6..+1] = i8 %54
    ;; a[7..+1] = i64 %53
 %61 = trunc nuw nsw i64 %5 to i8                                                                                      ;L338
 %62 = trunc nuw nsw i64 %7 to i8                                                                                      ;L339
 %63 = gep %21, i64 64                                                                                                 ;L329
 call void @llvm.memset.p0.i64(ptr %63, i8 0, i64 64, i1 false)                                                        ;L340
 %64 = gep %21, i64 128                                                                                                ;L329
 store i64 %1, ptr %64,                                                                                                ;L329
 %65 = gep %21, i64 136                                                                                                ;L329
 store i64 %42, ptr %65,                                                                                               ;L329
 %66 = gep %21, i64 176                                                                                                ;L329
 store i8 %8, ptr %66,                                                                                                 ;L329
 %67 = gep %21, i64 144                                                                                                ;L329
 store i64 %48, ptr %67,                                                                                               ;L329
 %68 = gep %21, i64 152                                                                                                ;L329
 store i64 %10, ptr %68,                                                                                               ;L329
 %69 = gep %21, i64 160                                                                                                ;L329
 %70 = zext i8 %54 to i64                                                                                              ;L329
 %71 = shl nuw nsw i64 %70, 48                                                                                         ;L329
 %72 = zext i8 %55 to i64                                                                                              ;L329
 %73 = shl nuw nsw i64 %72, 40                                                                                         ;L329
 %74 = zext i8 %56 to i64                                                                                              ;L329
 %75 = shl nuw nsw i64 %74, 32                                                                                         ;L329
 %76 = zext i8 %57 to i64                                                                                              ;L329
 %77 = shl nuw nsw i64 %76, 24                                                                                         ;L329
 %78 = zext i8 %58 to i64                                                                                              ;L329
 %79 = shl nuw nsw i64 %78, 16                                                                                         ;L329
 %80 = zext i8 %59 to i64                                                                                              ;L329
 %81 = shl nuw nsw i64 %80, 8                                                                                          ;L329
 %82 = zext i8 %60 to i64                                                                                              ;L329
 %83 = or i64 %53, %71                                                                                                 ;L329
 %84 = or i64 %83, %73                                                                                                 ;L329
 %85 = or i64 %84, %75                                                                                                 ;L329
 %86 = or i64 %85, %77                                                                                                 ;L329
 %87 = or i64 %86, %79                                                                                                 ;L329
 %88 = or i64 %87, %81                                                                                                 ;L329
 %89 = or i64 %88, %82                                                                                                 ;L329
 store i64 %89, ptr %69,                                                                                               ;L329
 %90 = gep %21, i64 168                                                                                                ;L329
 store i64 %13, ptr %90,                                                                                               ;L329
 %91 = gep %21, i64 177                                                                                                ;L329
 store i8 %61, ptr %91,                                                                                                ;L329
 %92 = gep %21, i64 178                                                                                                ;L329
 store i8 %62, ptr %92,                                                                                                ;L329
 call void @llvm.memset.p0.i64(ptr %21, i8 0, i64 64, i1 false)                                                        ;L329
    ;; iter[0..+8] = ptr %4
    ;; iter[8..+8] = !DIArgList(ptr %4, i64 %5)
    ;; iter[16..+8] = i64 0
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %4
    ;; self = ptr %4
    ;; end_or_len = !DIArgList(ptr %4, i64 %5)
 %93 = icmp eq i64 %5, 0                                                                                               ;L1714<180<80<343
 br i1 %93, label %99, label %94                                                                                       ;L180<80<343

94: ; preds = %52
    ;; i = i64 0
    ;; a = ptr %4
    ;; iter[16..+8] = i64 1
    ;; iter[0..+8] = ptr %4
 %95 = load ptr, ptr %4, , !!8, !!8                                                                                    ;L344
 %96 = gep %95, i64 1472                                                                                               ;L344
 %97 = load i64, ptr %96, , !!8                                                                                        ;L344
 store i64 %97, ptr %21,                                                                                               ;L344
    ;; iter[16..+8] = i64 1
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %4
    ;; self = ptr %4
    ;; end_or_len = !DIArgList(ptr %4, i64 %5)
 %98 = icmp eq i64 %5, 1                                                                                               ;L1714<180<80<343
 br i1 %98, label %99, label %219                                                                                      ;L180<80<343

99: ; preds = %261, %254, %247, %240, %233, %226, %219, %94, %52
    ;; iter[0..+8] = ptr %6
    ;; iter[8..+8] = !DIArgList(ptr %6, i64 %7)
    ;; iter[16..+8] = i64 0
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %6
    ;; self = ptr %6
    ;; end_or_len = !DIArgList(ptr %6, i64 %7)
 %100 = icmp eq i64 %7, 0                                                                                              ;L1714<180<80<346
 br i1 %100, label %106, label %101                                                                                    ;L180<80<346

101: ; preds = %99
    ;; i = i64 0
    ;; e = ptr %6
    ;; iter[16..+8] = i64 1
    ;; iter[0..+8] = ptr %6
 %102 = load ptr, ptr %6, , !!8, !!8                                                                                   ;L347
 %103 = gep %102, i64 1472                                                                                             ;L347
 %104 = load i64, ptr %103, , !!8                                                                                      ;L347
 store i64 %104, ptr %63,                                                                                              ;L347
    ;; iter[16..+8] = i64 1
    ;; iter[0..+8] = ptr %6
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %6
    ;; self = ptr %6
    ;; end_or_len = !DIArgList(ptr %6, i64 %7)
 %105 = icmp eq i64 %7, 1                                                                                              ;L1714<180<80<346
 br i1 %105, label %106, label %171                                                                                    ;L180<80<346

106: ; preds = %213, %206, %199, %192, %185, %178, %171, %101, %99
 %107 = load ptr, ptr %2, , !!8, !!8                                                                                   ;L350
 %108 = load ptr, ptr %107, , !!8, !!8                                                                                 ;L350
 %109 = gep %107, i64 8                                                                                                ;L350
 %110 = load ptr, ptr %109, , !!8, !!8                                                                                 ;L350
 %111 = gep %110, i64 32                                                                                               ;L350
 %112 = load ptr, ptr %111, , !!8                                                                                      ;L350
 %113 = invoke i64 %112(ptr %108)
 to label %114 unwind label %50                                                                                        ;L350

114: ; preds = %106
 store i64 %113, ptr %20,                                                                                              ;L350
 %115 = gep %110, i64 40                                                                                               ;L351
 %116 = load ptr, ptr %115, , !!8                                                                                      ;L351
 %117 = invoke i64 %116(ptr %108)
 to label %118 unwind label %50                                                                                        ;L351

118: ; preds = %114
 store i64 %117, ptr %19,                                                                                              ;L351
 store ptr %20, ptr %17,                                                                                               ;L352
 %119 = gep %17, i64 8                                                                                                 ;L352
 store ptr %19, ptr %119,                                                                                              ;L352
 %120 = gep %17, i64 16                                                                                                ;L352
 store ptr %21, ptr %120,                                                                                              ;L352
 invoke void @core::cell7RefCellNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model17ResolveFightCacheEE4withNCNvB1x_18resolve_fight_fulls_0INtNtBZ_6option6OptionNtB1x_15FightPredictionEEB1D_(ptr sret([64 x i8]) %18, ptr @anon.ff23c5838f81fa2a3acb125bb4b6e568.153, ptr %17)
 to label %121 unwind label %50                                                                                        ;L352

121: ; preds = %118
 %122 = load i64, ptr %18, , !!8                                                                                       ;L363
 %123 = icmp eq i64 %122, -1                                                                                           ;L363
 br i1 %123, label %129, label %124                                                                                    ;L363

124: ; preds = %121
 call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %18, i64 64, i1 false)                                                   ;L363
    ;; phase = i64 98
    ;; order = i8 0
    ;; val = i64 1
    ;; order = i8 0
    ;; val = i64 1
    ;; order = i8 0
    ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
    ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
    ;; order = i8 0
 %125 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                    ;L3904<741<160<364
 %126 = icmp eq i8 %125, 0                                                                                             ;L160<364
 br i1 %126, label %134, label %127                                                                                    ;L160<364

127: ; preds = %124
    ;; self = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 784)
    ;; dst = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 784)
 %128 = atomicrmw add ptr getelementptr (i8, ptr @gc::simulation4prof11PHASE_CALLS, i64 784), i64 1 monotonic,         ;L3937<3162<161<364
 br label %134                                                                                                         ;L160<364

129: ; preds = %121
    ;; phase = i64 99
    ;; order = i8 0
    ;; val = i64 1
    ;; order = i8 0
    ;; val = i64 1
    ;; order = i8 0
    ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
    ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
    ;; order = i8 0
 %130 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                    ;L3904<741<160<367
 %131 = icmp eq i8 %130, 0                                                                                             ;L160<367
 br i1 %131, label %142, label %132                                                                                    ;L160<367

132: ; preds = %129
    ;; self = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 792)
    ;; dst = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 792)
 %133 = atomicrmw add ptr getelementptr (i8, ptr @gc::simulation4prof11PHASE_CALLS, i64 792), i64 1 monotonic,         ;L3937<3162<161<367
 br label %142                                                                                                         ;L160<367

134: ; preds = %127, %124
 br label %135                                                                                                         ;L1

135: ; preds = %134, %36
 %136 = load i32, ptr %32, , !!8                                                                                       ;L825<376
 %137 = icmp eq i32 %136, -1                                                                                           ;L825<376
 br i1 %137, label %170, label %138                                                                                    ;L825<376

138: ; preds = %135
    ;; self = ptr %22
    ;; order = i8 0
    ;; order = i8 0
    ;; val = i64 1
    ;; order = i8 0
    ;; val = i64 1
    ;; order = i8 0
 %139 = load i64, ptr %22, , !!8                                                                                       ;L185<825<825<376
 %140 = icmp ult i64 %139, 132                                                                                         ;L185<825<825<376
 br i1 %140, label %156, label %141                                                                                    ;L185<825<825<376

141: ; preds = %138
 call void @core::panicking18panic_bounds_check(i64 %139, i64 132, ptr @anon.ff23c5838f81fa2a3acb125bb4b6e568.172) #26, !!46317 ;L185<825<825<376
 unreachable                                                                                                           ;L185<825<825<376

142: ; preds = %132, %129
 %143 = gep %2, i64 8                                                                                                  ;L368
 %144 = load ptr, ptr %143, , !!8, !!8                                                                                 ;L368
 invoke fastcc void @ai::plan_legacy3old11fight_model22resolve_fight_uncached(ptr %16, i64 %1, ptr %107, ptr %144, ptr %3, ptr %4, i64 %5, ptr %6, i64 %7, i8 %8, ptr %9, i64 %10, ptr %11, i64 %12, i64 %13)
 to label %145 unwind label %50                                                                                        ;L368

145: ; preds = %142
 store ptr %20, ptr %15,                                                                                               ;L369
 %146 = gep %15, i64 8                                                                                                 ;L369
 store ptr %19, ptr %146,                                                                                              ;L369
 %147 = gep %15, i64 16                                                                                                ;L369
 store ptr %21, ptr %147,                                                                                              ;L369
 %148 = gep %15, i64 24                                                                                                ;L369
 store ptr %16, ptr %148,                                                                                              ;L369
 invoke void @core::cell7RefCellNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model17ResolveFightCacheEE4withNCNvB1x_18resolve_fight_fulls0_0uEB1D_(ptr @anon.ff23c5838f81fa2a3acb125bb4b6e568.153, ptr %15)
 to label %149 unwind label %50                                                                                        ;L369

149: ; preds = %145
 call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %16, i64 64, i1 false)                                                   ;L375
 %150 = load i32, ptr %32, , !!8                                                                                       ;L825<376
 %151 = icmp eq i32 %150, -1                                                                                           ;L825<376
 br i1 %151, label %170, label %152                                                                                    ;L825<376

152: ; preds = %149
    ;; self = ptr %22
    ;; order = i8 0
    ;; order = i8 0
    ;; val = i64 1
    ;; order = i8 0
    ;; val = i64 1
    ;; order = i8 0
 %153 = load i64, ptr %22, , !!8                                                                                       ;L185<825<825<376
 %154 = icmp ult i64 %153, 132                                                                                         ;L185<825<825<376
 br i1 %154, label %156, label %155                                                                                    ;L185<825<825<376

155: ; preds = %152
 call void @core::panicking18panic_bounds_check(i64 %153, i64 132, ptr @anon.ff23c5838f81fa2a3acb125bb4b6e568.172) #26, !!46349 ;L185<825<825<376
 unreachable                                                                                                           ;L185<825<825<376

156: ; preds = %152, %138
 %157 = phi i64 [ %139, %138 ], [ %153, %152 ]
 %158 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %157                                 ;L185<825<825<376
 %159 = gep %22, i64 8                                                                                                 ;L185<825<825<376
 %160 = call { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %159)                               ;L185<825<825<376
 %161 = extractvalue { i64, i32 } %160, 0                                                                              ;L185<825<825<376
 %162 = extractvalue { i64, i32 } %160, 1                                                                              ;L185<825<825<376
 %163 = mul i64 %161, 1000000000                                                                                       ;L632<185<825<825<376
 %164 = icmp ult i32 %162, 1000000000                                                                                  ;L49<632<185<825<825<376
 call void @llvm.assume(i1 %164)                                                                                       ;L49<632<185<825<825<376
 %165 = zext nneg i32 %162 to i64                                                                                      ;L632<185<825<825<376
 %166 = add i64 %163, %165                                                                                             ;L632<185<825<825<376
 %167 = atomicrmw add ptr %158, i64 %166 monotonic, , !!8                                                              ;L3937<3162<185<825<825<376
 %168 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %157                                 ;L186<825<825<376
 %169 = atomicrmw add ptr %168, i64 1 monotonic, , !!8                                                                 ;L3937<3162<186<825<825<376
 br label %170                                                                                                         ;L376

170: ; preds = %156, %149, %135
 ret void                                                                                                              ;L376

171: ; preds = %101
 %172 = gep %6, i64 8                                                                                                  ;L656<185<80<346
    ;; ptr = ptr %172
    ;; self = ptr %172
    ;; i = i64 1
    ;; e = ptr %172
    ;; iter[16..+8] = i64 2
    ;; iter[0..+8] = ptr %6
 %173 = load ptr, ptr %172, , !!8, !!8                                                                                 ;L347
 %174 = gep %173, i64 1472                                                                                             ;L347
 %175 = load i64, ptr %174, , !!8                                                                                      ;L347
 %176 = gep %21, i64 72                                                                                                ;L347
 store i64 %175, ptr %176,                                                                                             ;L347
    ;; iter[16..+8] = i64 2
    ;; iter[0..+8] = ptr %6
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %6
    ;; self = ptr %6
    ;; end_or_len = !DIArgList(ptr %6, i64 %7)
 %177 = icmp eq i64 %7, 2                                                                                              ;L1714<180<80<346
 br i1 %177, label %106, label %178                                                                                    ;L180<80<346

178: ; preds = %171
 %179 = gep %6, i64 16                                                                                                 ;L656<185<80<346
    ;; ptr = ptr %179
    ;; self = ptr %179
    ;; i = i64 2
    ;; e = ptr %179
    ;; iter[16..+8] = i64 3
    ;; iter[0..+8] = ptr %6
 %180 = load ptr, ptr %179, , !!8, !!8                                                                                 ;L347
 %181 = gep %180, i64 1472                                                                                             ;L347
 %182 = load i64, ptr %181, , !!8                                                                                      ;L347
 %183 = gep %21, i64 80                                                                                                ;L347
 store i64 %182, ptr %183,                                                                                             ;L347
    ;; iter[16..+8] = i64 3
    ;; iter[0..+8] = ptr %6
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %6
    ;; self = ptr %6
    ;; end_or_len = !DIArgList(ptr %6, i64 %7)
 %184 = icmp eq i64 %7, 3                                                                                              ;L1714<180<80<346
 br i1 %184, label %106, label %185                                                                                    ;L180<80<346

185: ; preds = %178
 %186 = gep %6, i64 24                                                                                                 ;L656<185<80<346
    ;; ptr = ptr %186
    ;; self = ptr %186
    ;; i = i64 3
    ;; e = ptr %186
    ;; iter[16..+8] = i64 4
    ;; iter[0..+8] = ptr %6
 %187 = load ptr, ptr %186, , !!8, !!8                                                                                 ;L347
 %188 = gep %187, i64 1472                                                                                             ;L347
 %189 = load i64, ptr %188, , !!8                                                                                      ;L347
 %190 = gep %21, i64 88                                                                                                ;L347
 store i64 %189, ptr %190,                                                                                             ;L347
    ;; iter[16..+8] = i64 4
    ;; iter[0..+8] = ptr %6
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %6
    ;; self = ptr %6
    ;; end_or_len = !DIArgList(ptr %6, i64 %7)
 %191 = icmp eq i64 %7, 4                                                                                              ;L1714<180<80<346
 br i1 %191, label %106, label %192                                                                                    ;L180<80<346

192: ; preds = %185
 %193 = gep %6, i64 32                                                                                                 ;L656<185<80<346
    ;; ptr = ptr %193
    ;; self = ptr %193
    ;; i = i64 4
    ;; e = ptr %193
    ;; iter[16..+8] = i64 5
    ;; iter[0..+8] = ptr %6
 %194 = load ptr, ptr %193, , !!8, !!8                                                                                 ;L347
 %195 = gep %194, i64 1472                                                                                             ;L347
 %196 = load i64, ptr %195, , !!8                                                                                      ;L347
 %197 = gep %21, i64 96                                                                                                ;L347
 store i64 %196, ptr %197,                                                                                             ;L347
    ;; iter[16..+8] = i64 5
    ;; iter[0..+8] = ptr %6
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %6
    ;; self = ptr %6
    ;; end_or_len = !DIArgList(ptr %6, i64 %7)
 %198 = icmp eq i64 %7, 5                                                                                              ;L1714<180<80<346
 br i1 %198, label %106, label %199                                                                                    ;L180<80<346

199: ; preds = %192
 %200 = gep %6, i64 40                                                                                                 ;L656<185<80<346
    ;; ptr = ptr %200
    ;; self = ptr %200
    ;; i = i64 5
    ;; e = ptr %200
    ;; iter[16..+8] = i64 6
    ;; iter[0..+8] = ptr %6
 %201 = load ptr, ptr %200, , !!8, !!8                                                                                 ;L347
 %202 = gep %201, i64 1472                                                                                             ;L347
 %203 = load i64, ptr %202, , !!8                                                                                      ;L347
 %204 = gep %21, i64 104                                                                                               ;L347
 store i64 %203, ptr %204,                                                                                             ;L347
    ;; iter[16..+8] = i64 6
    ;; iter[0..+8] = ptr %6
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %6
    ;; self = ptr %6
    ;; end_or_len = !DIArgList(ptr %6, i64 %7)
 %205 = icmp eq i64 %7, 6                                                                                              ;L1714<180<80<346
 br i1 %205, label %106, label %206                                                                                    ;L180<80<346

206: ; preds = %199
 %207 = gep %6, i64 48                                                                                                 ;L656<185<80<346
    ;; ptr = ptr %207
    ;; self = ptr %207
    ;; i = i64 6
    ;; e = ptr %207
    ;; iter[16..+8] = i64 7
    ;; iter[0..+8] = ptr %6
 %208 = load ptr, ptr %207, , !!8, !!8                                                                                 ;L347
 %209 = gep %208, i64 1472                                                                                             ;L347
 %210 = load i64, ptr %209, , !!8                                                                                      ;L347
 %211 = gep %21, i64 112                                                                                               ;L347
 store i64 %210, ptr %211,                                                                                             ;L347
    ;; iter[16..+8] = i64 7
    ;; iter[0..+8] = ptr %6
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %6
    ;; self = ptr %6
    ;; end_or_len = !DIArgList(ptr %6, i64 %7)
 %212 = icmp eq i64 %7, 7                                                                                              ;L1714<180<80<346
 br i1 %212, label %106, label %213                                                                                    ;L180<80<346

213: ; preds = %206
 %214 = gep %6, i64 56                                                                                                 ;L656<185<80<346
    ;; ptr = ptr %214
    ;; self = ptr %214
    ;; i = i64 7
    ;; e = ptr %214
    ;; iter[16..+8] = i64 8
    ;; iter[0..+8] = ptr %6
 %215 = load ptr, ptr %214, , !!8, !!8                                                                                 ;L347
 %216 = gep %215, i64 1472                                                                                             ;L347
 %217 = load i64, ptr %216, , !!8                                                                                      ;L347
 %218 = gep %21, i64 120                                                                                               ;L347
 store i64 %217, ptr %218,                                                                                             ;L347
    ;; iter[16..+8] = i64 8
    ;; iter[0..+8] = ptr %6
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %6
    ;; self = ptr %6
    ;; end_or_len = !DIArgList(ptr %6, i64 %7)
 br label %106                                                                                                         ;L180<80<346

219: ; preds = %94
 %220 = gep %4, i64 8                                                                                                  ;L656<185<80<343
    ;; ptr = ptr %220
    ;; self = ptr %220
    ;; i = i64 1
    ;; a = ptr %220
    ;; iter[16..+8] = i64 2
    ;; iter[0..+8] = ptr %4
 %221 = load ptr, ptr %220, , !!8, !!8                                                                                 ;L344
 %222 = gep %221, i64 1472                                                                                             ;L344
 %223 = load i64, ptr %222, , !!8                                                                                      ;L344
 %224 = gep %21, i64 8                                                                                                 ;L344
 store i64 %223, ptr %224,                                                                                             ;L344
    ;; iter[16..+8] = i64 2
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %4
    ;; self = ptr %4
    ;; end_or_len = !DIArgList(ptr %4, i64 %5)
 %225 = icmp eq i64 %5, 2                                                                                              ;L1714<180<80<343
 br i1 %225, label %99, label %226                                                                                     ;L180<80<343

226: ; preds = %219
 %227 = gep %4, i64 16                                                                                                 ;L656<185<80<343
    ;; ptr = ptr %227
    ;; self = ptr %227
    ;; i = i64 2
    ;; a = ptr %227
    ;; iter[16..+8] = i64 3
    ;; iter[0..+8] = ptr %4
 %228 = load ptr, ptr %227, , !!8, !!8                                                                                 ;L344
 %229 = gep %228, i64 1472                                                                                             ;L344
 %230 = load i64, ptr %229, , !!8                                                                                      ;L344
 %231 = gep %21, i64 16                                                                                                ;L344
 store i64 %230, ptr %231,                                                                                             ;L344
    ;; iter[16..+8] = i64 3
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %4
    ;; self = ptr %4
    ;; end_or_len = !DIArgList(ptr %4, i64 %5)
 %232 = icmp eq i64 %5, 3                                                                                              ;L1714<180<80<343
 br i1 %232, label %99, label %233                                                                                     ;L180<80<343

233: ; preds = %226
 %234 = gep %4, i64 24                                                                                                 ;L656<185<80<343
    ;; ptr = ptr %234
    ;; self = ptr %234
    ;; i = i64 3
    ;; a = ptr %234
    ;; iter[16..+8] = i64 4
    ;; iter[0..+8] = ptr %4
 %235 = load ptr, ptr %234, , !!8, !!8                                                                                 ;L344
 %236 = gep %235, i64 1472                                                                                             ;L344
 %237 = load i64, ptr %236, , !!8                                                                                      ;L344
 %238 = gep %21, i64 24                                                                                                ;L344
 store i64 %237, ptr %238,                                                                                             ;L344
    ;; iter[16..+8] = i64 4
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %4
    ;; self = ptr %4
    ;; end_or_len = !DIArgList(ptr %4, i64 %5)
 %239 = icmp eq i64 %5, 4                                                                                              ;L1714<180<80<343
 br i1 %239, label %99, label %240                                                                                     ;L180<80<343

240: ; preds = %233
 %241 = gep %4, i64 32                                                                                                 ;L656<185<80<343
    ;; ptr = ptr %241
    ;; self = ptr %241
    ;; i = i64 4
    ;; a = ptr %241
    ;; iter[16..+8] = i64 5
    ;; iter[0..+8] = ptr %4
 %242 = load ptr, ptr %241, , !!8, !!8                                                                                 ;L344
 %243 = gep %242, i64 1472                                                                                             ;L344
 %244 = load i64, ptr %243, , !!8                                                                                      ;L344
 %245 = gep %21, i64 32                                                                                                ;L344
 store i64 %244, ptr %245,                                                                                             ;L344
    ;; iter[16..+8] = i64 5
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %4
    ;; self = ptr %4
    ;; end_or_len = !DIArgList(ptr %4, i64 %5)
 %246 = icmp eq i64 %5, 5                                                                                              ;L1714<180<80<343
 br i1 %246, label %99, label %247                                                                                     ;L180<80<343

247: ; preds = %240
 %248 = gep %4, i64 40                                                                                                 ;L656<185<80<343
    ;; ptr = ptr %248
    ;; self = ptr %248
    ;; i = i64 5
    ;; a = ptr %248
    ;; iter[16..+8] = i64 6
    ;; iter[0..+8] = ptr %4
 %249 = load ptr, ptr %248, , !!8, !!8                                                                                 ;L344
 %250 = gep %249, i64 1472                                                                                             ;L344
 %251 = load i64, ptr %250, , !!8                                                                                      ;L344
 %252 = gep %21, i64 40                                                                                                ;L344
 store i64 %251, ptr %252,                                                                                             ;L344
    ;; iter[16..+8] = i64 6
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %4
    ;; self = ptr %4
    ;; end_or_len = !DIArgList(ptr %4, i64 %5)
 %253 = icmp eq i64 %5, 6                                                                                              ;L1714<180<80<343
 br i1 %253, label %99, label %254                                                                                     ;L180<80<343

254: ; preds = %247
 %255 = gep %4, i64 48                                                                                                 ;L656<185<80<343
    ;; ptr = ptr %255
    ;; self = ptr %255
    ;; i = i64 6
    ;; a = ptr %255
    ;; iter[16..+8] = i64 7
    ;; iter[0..+8] = ptr %4
 %256 = load ptr, ptr %255, , !!8, !!8                                                                                 ;L344
 %257 = gep %256, i64 1472                                                                                             ;L344
 %258 = load i64, ptr %257, , !!8                                                                                      ;L344
 %259 = gep %21, i64 48                                                                                                ;L344
 store i64 %258, ptr %259,                                                                                             ;L344
    ;; iter[16..+8] = i64 7
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %4
    ;; self = ptr %4
    ;; end_or_len = !DIArgList(ptr %4, i64 %5)
 %260 = icmp eq i64 %5, 7                                                                                              ;L1714<180<80<343
 br i1 %260, label %99, label %261                                                                                     ;L180<80<343

261: ; preds = %254
 %262 = gep %4, i64 56                                                                                                 ;L656<185<80<343
    ;; ptr = ptr %262
    ;; self = ptr %262
    ;; i = i64 7
    ;; a = ptr %262
    ;; iter[16..+8] = i64 8
    ;; iter[0..+8] = ptr %4
 %263 = load ptr, ptr %262, , !!8, !!8                                                                                 ;L344
 %264 = gep %263, i64 1472                                                                                             ;L344
 %265 = load i64, ptr %264, , !!8                                                                                      ;L344
 %266 = gep %21, i64 56                                                                                                ;L344
 store i64 %265, ptr %266,                                                                                             ;L344
    ;; iter[16..+8] = i64 8
    ;; self = ptr undef
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %4
    ;; self = ptr %4
    ;; end_or_len = !DIArgList(ptr %4, i64 %5)
 br label %99                                                                                                          ;L180<80<343

267: ; preds = %47
    ;; iter[0..+8] = ptr %11
    ;; iter[16..+8] = i64 7
    ;; iter[24..+8] = i64 0
    ;; i = i64 0
    ;; v = ptr %11
    ;; min = i64 0
    ;; max = i64 255
 %268 = load i64, ptr %11, , !!8                                                                                       ;L336
    ;; self = i64 %268
 %269 = icmp slt i64 %268, -14                                                                                         ;L2025<336
 %270 = sdiv i64 %268, 15                                                                                              ;L336
    ;; self = i64 %270
 %271 = tail call i64 @llvm.umin.i64(i64 %270, i64 255)                                                                ;L2025<336
    ;; iter[24..+8] = i64 1
    ;; iter[0..+8] = ptr %11
 %272 = trunc nuw i64 %271 to i8                                                                                       ;L336
 %273 = select i1 %269, i8 0, i8 %272                                                                                  ;L2025<336
    ;; a[0..+1] = i8 %273
    ;; self = ptr undef
    ;; self = ptr undef
    ;; iter[24..+8] = i64 1
    ;; iter[16..+8] = i64 6
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %11
    ;; self = ptr %11
    ;; end_or_len = !DIArgList(ptr %11, i64 %12)
 %274 = icmp eq i64 %12, 1                                                                                             ;L1714<180<39<80<336
 br i1 %274, label %52, label %275                                                                                     ;L180<39<80<336

275: ; preds = %267
 %276 = gep %11, i64 8                                                                                                 ;L656<185<39<80<336
    ;; ptr = ptr %276
    ;; self = ptr %276
    ;; iter[0..+8] = ptr %276
    ;; iter[16..+8] = i64 6
    ;; iter[24..+8] = i64 1
    ;; i = i64 1
    ;; v = ptr %276
    ;; min = i64 0
    ;; max = i64 255
 %277 = load i64, ptr %276, , !!8                                                                                      ;L336
    ;; self = i64 %277
 %278 = icmp slt i64 %277, -14                                                                                         ;L2025<336
 %279 = sdiv i64 %277, 15                                                                                              ;L336
    ;; self = i64 %279
 %280 = tail call i64 @llvm.umin.i64(i64 %279, i64 255)                                                                ;L2025<336
    ;; iter[24..+8] = i64 2
    ;; iter[0..+8] = ptr %11
 %281 = trunc nuw i64 %280 to i8                                                                                       ;L336
 %282 = select i1 %278, i8 0, i8 %281                                                                                  ;L2025<336
    ;; a[1..+1] = i8 %282
    ;; self = ptr undef
    ;; self = ptr undef
    ;; iter[24..+8] = i64 2
    ;; iter[16..+8] = i64 5
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %11
    ;; self = ptr %11
    ;; end_or_len = !DIArgList(ptr %11, i64 %12)
 %283 = icmp eq i64 %12, 2                                                                                             ;L1714<180<39<80<336
 br i1 %283, label %52, label %284                                                                                     ;L180<39<80<336

284: ; preds = %275
 %285 = gep %11, i64 16                                                                                                ;L656<185<39<80<336
    ;; ptr = ptr %285
    ;; self = ptr %285
    ;; iter[0..+8] = ptr %285
    ;; iter[16..+8] = i64 5
    ;; iter[24..+8] = i64 2
    ;; i = i64 2
    ;; v = ptr %285
    ;; min = i64 0
    ;; max = i64 255
 %286 = load i64, ptr %285, , !!8                                                                                      ;L336
    ;; self = i64 %286
 %287 = icmp slt i64 %286, -14                                                                                         ;L2025<336
 %288 = sdiv i64 %286, 15                                                                                              ;L336
    ;; self = i64 %288
 %289 = tail call i64 @llvm.umin.i64(i64 %288, i64 255)                                                                ;L2025<336
    ;; iter[24..+8] = i64 3
    ;; iter[0..+8] = ptr %11
 %290 = trunc nuw i64 %289 to i8                                                                                       ;L336
 %291 = select i1 %287, i8 0, i8 %290                                                                                  ;L2025<336
    ;; a[2..+1] = i8 %291
    ;; self = ptr undef
    ;; self = ptr undef
    ;; iter[24..+8] = i64 3
    ;; iter[16..+8] = i64 4
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %11
    ;; self = ptr %11
    ;; end_or_len = !DIArgList(ptr %11, i64 %12)
 %292 = icmp eq i64 %12, 3                                                                                             ;L1714<180<39<80<336
 br i1 %292, label %52, label %293                                                                                     ;L180<39<80<336

293: ; preds = %284
 %294 = gep %11, i64 24                                                                                                ;L656<185<39<80<336
    ;; ptr = ptr %294
    ;; self = ptr %294
    ;; iter[0..+8] = ptr %294
    ;; iter[16..+8] = i64 4
    ;; iter[24..+8] = i64 3
    ;; i = i64 3
    ;; v = ptr %294
    ;; min = i64 0
    ;; max = i64 255
 %295 = load i64, ptr %294, , !!8                                                                                      ;L336
    ;; self = i64 %295
 %296 = icmp slt i64 %295, -14                                                                                         ;L2025<336
 %297 = sdiv i64 %295, 15                                                                                              ;L336
    ;; self = i64 %297
 %298 = tail call i64 @llvm.umin.i64(i64 %297, i64 255)                                                                ;L2025<336
    ;; iter[24..+8] = i64 4
    ;; iter[0..+8] = ptr %11
 %299 = trunc nuw i64 %298 to i8                                                                                       ;L336
 %300 = select i1 %296, i8 0, i8 %299                                                                                  ;L2025<336
    ;; a[3..+1] = i8 %300
    ;; self = ptr undef
    ;; self = ptr undef
    ;; iter[24..+8] = i64 4
    ;; iter[16..+8] = i64 3
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %11
    ;; self = ptr %11
    ;; end_or_len = !DIArgList(ptr %11, i64 %12)
 %301 = icmp eq i64 %12, 4                                                                                             ;L1714<180<39<80<336
 br i1 %301, label %52, label %302                                                                                     ;L180<39<80<336

302: ; preds = %293
 %303 = gep %11, i64 32                                                                                                ;L656<185<39<80<336
    ;; ptr = ptr %303
    ;; self = ptr %303
    ;; iter[0..+8] = ptr %303
    ;; iter[16..+8] = i64 3
    ;; iter[24..+8] = i64 4
    ;; i = i64 4
    ;; v = ptr %303
    ;; min = i64 0
    ;; max = i64 255
 %304 = load i64, ptr %303, , !!8                                                                                      ;L336
    ;; self = i64 %304
 %305 = icmp slt i64 %304, -14                                                                                         ;L2025<336
 %306 = sdiv i64 %304, 15                                                                                              ;L336
    ;; self = i64 %306
 %307 = tail call i64 @llvm.umin.i64(i64 %306, i64 255)                                                                ;L2025<336
    ;; iter[24..+8] = i64 5
    ;; iter[0..+8] = ptr %11
 %308 = trunc nuw i64 %307 to i8                                                                                       ;L336
 %309 = select i1 %305, i8 0, i8 %308                                                                                  ;L2025<336
    ;; a[4..+1] = i8 %309
    ;; self = ptr undef
    ;; self = ptr undef
    ;; iter[24..+8] = i64 5
    ;; iter[16..+8] = i64 2
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %11
    ;; self = ptr %11
    ;; end_or_len = !DIArgList(ptr %11, i64 %12)
 %310 = icmp eq i64 %12, 5                                                                                             ;L1714<180<39<80<336
 br i1 %310, label %52, label %311                                                                                     ;L180<39<80<336

311: ; preds = %302
 %312 = gep %11, i64 40                                                                                                ;L656<185<39<80<336
    ;; ptr = ptr %312
    ;; self = ptr %312
    ;; iter[0..+8] = ptr %312
    ;; iter[16..+8] = i64 2
    ;; iter[24..+8] = i64 5
    ;; i = i64 5
    ;; v = ptr %312
    ;; min = i64 0
    ;; max = i64 255
 %313 = load i64, ptr %312, , !!8                                                                                      ;L336
    ;; self = i64 %313
 %314 = icmp slt i64 %313, -14                                                                                         ;L2025<336
 %315 = sdiv i64 %313, 15                                                                                              ;L336
    ;; self = i64 %315
 %316 = tail call i64 @llvm.umin.i64(i64 %315, i64 255)                                                                ;L2025<336
    ;; iter[24..+8] = i64 6
    ;; iter[0..+8] = ptr %11
 %317 = trunc nuw i64 %316 to i8                                                                                       ;L336
 %318 = select i1 %314, i8 0, i8 %317                                                                                  ;L2025<336
    ;; a[5..+1] = i8 %318
    ;; self = ptr undef
    ;; self = ptr undef
    ;; iter[24..+8] = i64 6
    ;; iter[16..+8] = i64 1
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %11
    ;; self = ptr %11
    ;; end_or_len = !DIArgList(ptr %11, i64 %12)
 %319 = icmp eq i64 %12, 6                                                                                             ;L1714<180<39<80<336
 br i1 %319, label %52, label %320                                                                                     ;L180<39<80<336

320: ; preds = %311
 %321 = gep %11, i64 48                                                                                                ;L656<185<39<80<336
    ;; ptr = ptr %321
    ;; self = ptr %321
    ;; iter[0..+8] = ptr %321
    ;; iter[16..+8] = i64 1
    ;; iter[24..+8] = i64 6
    ;; i = i64 6
    ;; v = ptr %321
    ;; min = i64 0
    ;; max = i64 255
 %322 = load i64, ptr %321, , !!8                                                                                      ;L336
    ;; self = i64 %322
 %323 = icmp slt i64 %322, -14                                                                                         ;L2025<336
 %324 = sdiv i64 %322, 15                                                                                              ;L336
    ;; self = i64 %324
 %325 = tail call i64 @llvm.umin.i64(i64 %324, i64 255)                                                                ;L2025<336
    ;; iter[24..+8] = i64 7
    ;; iter[0..+8] = ptr %11
 %326 = trunc nuw i64 %325 to i8                                                                                       ;L336
 %327 = select i1 %323, i8 0, i8 %326                                                                                  ;L2025<336
    ;; a[6..+1] = i8 %327
    ;; self = ptr undef
    ;; self = ptr undef
    ;; iter[24..+8] = i64 7
    ;; iter[16..+8] = i64 0
    ;; self = ptr undef
    ;; count = i64 1
    ;; ptr = ptr %11
    ;; self = ptr %11
    ;; end_or_len = !DIArgList(ptr %11, i64 %12)
 %328 = icmp eq i64 %12, 7                                                                                             ;L1714<180<39<80<336
 br i1 %328, label %52, label %329                                                                                     ;L180<39<80<336

329: ; preds = %320
 %330 = gep %11, i64 56                                                                                                ;L656<185<39<80<336
    ;; iter[0..+8] = ptr %330
    ;; ptr = ptr %330
    ;; self = ptr %330
    ;; iter[0..+8] = ptr %330
    ;; iter[16..+8] = i64 0
    ;; iter[24..+8] = i64 7
    ;; i = i64 7
    ;; v = ptr %330
    ;; min = i64 0
    ;; max = i64 255
 %331 = load i64, ptr %330, , !!8                                                                                      ;L336
    ;; self = i64 %331
 %332 = icmp slt i64 %331, -14                                                                                         ;L2025<336
 %333 = sdiv i64 %331, 15                                                                                              ;L336
    ;; self = i64 %333
 %334 = tail call i64 @llvm.umin.i64(i64 %333, i64 255)                                                                ;L2025<336
    ;; iter[0..+8] = ptr %330
    ;; iter[16..+8] = i64 0
    ;; iter[24..+8] = i64 8
    ;; self = ptr undef
    ;; self = ptr undef
 %335 = shl nuw i64 %334, 56                                                                                           ;L329
 %336 = select i1 %332, i64 0, i64 %335                                                                                ;L2025<336
 br label %52                                                                                                          ;L37<80<336
}
