104740| define hidden void @ai::small_action6aroundNtB5_21SmallActionAroundBush12update_state(ptr %0, ptr %1, ptr %2, ptr %3, ptr %4) unnamed_addr #2 {
104741|  %6 = alloca [16 x i8],
104742|  %7 = alloca [24 x i8],
104743|  %8 = alloca [24 x i8],
104744|     ;; self[16..+1136] = ptr @anon.afb017e85830050c32e5158c55060899.94
104745|  %9 = alloca [1168 x i8],
104746|     ;; self = ptr %0
104747|     ;; rnd = ptr %1
104748|     ;; player = ptr %2
104749|     ;; data = ptr %3
104750|     ;; debug = ptr %4
104751|     ;; candidates = ptr %9
104752|     ;; self = ptr %8
104753|     ;; start = i64 60
104754|     ;; end = i64 120
104755|     ;; r = i64 4294902015
104756|  %10 = gep %2, i64 2352                                                                                                ;L1222
104757|  %11 = load i64, ptr %10, , !!8                                                                                        ;L1222
104758|  %12 = icmp ult i64 %11, 2                                                                                             ;L1222
104759|  br i1 %12, label %14, label %13                                                                                       ;L1222
104760| 
104761| 13: ; preds = %5
104762|  tail call void @core::panicking18panic_bounds_check(i64 %11, i64 2, ptr @anon.afb017e85830050c32e5158c55060899.95) #25 ;L1222
104763|  unreachable                                                                                                           ;L1222
104764| 
104765| 14: ; preds = %5
104766|     ;; self = ptr %2
104767|  %15 = gep %2, i64 2496                                                                                                ;L581<1222
104768|  %16 = load i32, ptr %15, , !!8                                                                                        ;L581<1222
104769|  %17 = zext nneg i32 %16 to i64                                                                                        ;L581<1222
104770|  %18 = load ptr, ptr %3, , !!8, !!8                                                                                    ;L1222
104771|  %19 = gep %18, i64 480                                                                                                ;L1222
104772|  %20 = getelementptr [5 x ptr], ptr %19, i64 %11                                                                       ;L1222
104773|  %21 = getelementptr ptr, ptr %20, i64 %17                                                                             ;L1222
104774|  %22 = load ptr, ptr %21, , !!8                                                                                        ;L1222
104775|     ;; self = ptr %22
104776|  %23 = icmp eq ptr %22, null                                                                                           ;L1011<1222
104777|  br i1 %23, label %45, label %24                                                                                       ;L1011<1222
104778| 
104779| 24: ; preds = %14
104780|     ;; champ = ptr %22
104781|  %25 = gep %22, i64 1632                                                                                               ;L1224
104782|  %26 = load i64, ptr %25, , !!8                                                                                        ;L1224
104783|     ;; x1 = i64 %26
104784|     ;; self = i64 %26
104785|  %27 = gep %22, i64 1640                                                                                               ;L1224
104786|  %28 = load i64, ptr %27, , !!8                                                                                        ;L1224
104787|     ;; y1 = i64 %28
104788|     ;; self = i64 %28
104789|  %29 = gep %0, i64 24                                                                                                  ;L1224
104790|  %30 = load i64, ptr %29, , !!8                                                                                        ;L1224
104791|     ;; x2 = i64 %30
104792|     ;; other = i64 %30
104793|  %31 = gep %0, i64 32                                                                                                  ;L1224
104794|  %32 = load i64, ptr %31, , !!8                                                                                        ;L1224
104795|     ;; y2 = i64 %32
104796|     ;; other = i64 %32
104797|  %33 = icmp ult i64 %26, %30                                                                                           ;L3147<7<1224
104798|  %34 = sub nuw i64 %30, %26                                                                                            ;L3147<7<1224
104799|  %35 = sub nuw i64 %26, %30                                                                                            ;L3147<7<1224
104800|  %36 = select i1 %33, i64 %34, i64 %35                                                                                 ;L3147<7<1224
104801|     ;; dx = i64 %36
104802|  %37 = icmp ult i64 %28, %32                                                                                           ;L3147<8<1224
104803|  %38 = sub nuw i64 %32, %28                                                                                            ;L3147<8<1224
104804|  %39 = sub nuw i64 %28, %32                                                                                            ;L3147<8<1224
104805|  %40 = select i1 %37, i64 %38, i64 %39                                                                                 ;L3147<8<1224
104806|     ;; dy = i64 %40
104807|  %41 = mul i64 %36, %36                                                                                                ;L9<1224
104808|  %42 = mul i64 %40, %40                                                                                                ;L9<1224
104809|  %43 = add i64 %42, %41                                                                                                ;L9<1224
104810|  %44 = icmp ult i64 %43, 256000001                                                                                     ;L1224
104811|  br i1 %44, label %46, label %56                                                                                       ;L1224
104812| 
104813| 45: ; preds = %14
104814|  tail call void @core::option13unwrap_failed(ptr @anon.afb017e85830050c32e5158c55060899.96) #25                        ;L1013<1222
104815|  unreachable                                                                                                           ;L1013<1222
104816| 
104817| 46: ; preds = %24
104818|  %47 = load ptr, ptr %18, , !!8, !!8                                                                                   ;L1224
104819|  %48 = gep %18, i64 8                                                                                                  ;L1224
104820|  %49 = load ptr, ptr %48, , !!8, !!8                                                                                   ;L1224
104821|  %50 = gep %49, i64 40                                                                                                 ;L1224
104822|  %51 = load ptr, ptr %50, , !!8                                                                                        ;L1224
104823|  %52 = tail call i64 %51(ptr %47)                                                                                      ;L1224
104824|  %53 = gep %0, i64 8                                                                                                   ;L1224
104825|  %54 = load i64, ptr %53, , !!8                                                                                        ;L1224
104826|  %55 = icmp ult i64 %52, %54                                                                                           ;L1224
104827|  br i1 %55, label %56, label %62                                                                                       ;L1224
104828| 
104829| 56: ; preds = %74, %46, %24
104830|  %57 = gep %3, i64 8                                                                                                   ;L1235
104831|  %58 = load ptr, ptr %57, , !!8, !!8                                                                                   ;L1235
104832|  %59 = gep %58, i64 59                                                                                                 ;L1235
104833|  %60 = load i8, ptr %59, , !!8                                                                                         ;L1235
104834|  %61 = trunc nuw i8 %60 to i1                                                                                          ;L1235
104835|  br i1 %61, label %90, label %89                                                                                       ;L1235
104836| 
104837| 62: ; preds = %46
104839|     ;; self[0..+8] = i64 0
104840|     ;; self[8..+8] = i64 71
104841|  %63 = gep %3, i64 8                                                                                                   ;L1225
104842|  %64 = load ptr, ptr %63, , !!8, !!8                                                                                   ;L1225
104843|  %65 = gep %64, i64 32                                                                                                 ;L1225
104844|  %66 = load ptr, ptr %65, , !!8, !!8                                                                                   ;L1225
104845|  %67 = gep %0, i64 16                                                                                                  ;L1225
104846|     ;; predicate[0..+8] = ptr %66
104847|     ;; predicate[8..+8] = ptr %67
104848|  %68 = gep %9, i64 16                                                                                                  ;L28<957<1225
104849|  store i64 0, ptr %68,                                                                                                 ;L28<957<1225
104850|  %69 = gep %9, i64 24                                                                                                  ;L28<957<1225
104851|  store i64 71, ptr %69,                                                                                                ;L28<957<1225
104852|  %70 = gep %9, i64 32                                                                                                  ;L28<957<1225
104853|  call void @llvm.memcpy.p0.p0.i64(ptr %70, ptr @anon.afb017e85830050c32e5158c55060899.94, i64 1136, i1 false)          ;L28<957<1225
104854|  store ptr %66, ptr %9,                                                                                                ;L28<957<1225
104855|  %71 = gep %9, i64 8                                                                                                   ;L28<957<1225
104856|  store ptr %67, ptr %71,                                                                                               ;L28<957<1225
104858|  call void @core::iter8adapters6filter6FilterINtNtNtBc_5array4iter8IntoIterTjjEKj47_ENCNvMs6_NtNtCshdEBA0ozCnw_7game_ai12small_action6aroundNtB1J_21SmallActionAroundBush12update_state0ENtNtCsMBkRBYhlca_4rand3seq14IteratorRandom6chooseNtNtNtB3h_4rngs3std6StdRngEB1N_(ptr sret([24 x i8]) %8, ptr %9, ptr %1) ;L1226
104859|  %72 = load i64, ptr %8, , !!8                                                                                         ;L1011<1226
104860|  %73 = trunc nuw i64 %72 to i1                                                                                         ;L1011<1226
104861|  br i1 %73, label %74, label %88                                                                                       ;L1011<1226
104862| 
104863| 74: ; preds = %62
104864|  %75 = gep %8, i64 8                                                                                                   ;L1012<1226
104865|  %76 = load i64, ptr %75, , !!8                                                                                        ;L1012<1226
104866|     ;; target_x = i64 %76
104867|  %77 = gep %8, i64 16                                                                                                  ;L1012<1226
104868|  %78 = load i64, ptr %77, , !!8                                                                                        ;L1012<1226
104869|     ;; target_y = i64 %78
104871|  %79 = mul i64 %76, 32000                                                                                              ;L1227
104872|  %80 = add i64 %79, 16000                                                                                              ;L1227
104873|     ;; target_x = i64 %80
104874|  %81 = mul i64 %78, 32000                                                                                              ;L1228
104875|  %82 = add i64 %81, 16000                                                                                              ;L1228
104876|     ;; target_y = i64 %82
104877|  store i64 %80, ptr %29,                                                                                               ;L1230
104878|  store i64 %82, ptr %31,                                                                                               ;L1231
104879|  %83 = call i64 %51(ptr %47)                                                                                           ;L1232
104881|  store i64 60, ptr %7,                                                                                                 ;L391<1232
104882|  %84 = gep %7, i64 8                                                                                                   ;L391<1232
104883|  store i64 120, ptr %84,                                                                                               ;L391<1232
104884|  %85 = gep %7, i64 16                                                                                                  ;L391<1232
104885|  store i8 0, ptr %85,                                                                                                  ;L391<1232
104886|  %86 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %1, ptr %7)                              ;L1232
104888|  %87 = add i64 %86, %83                                                                                                ;L1232
104889|  store i64 %87, ptr %53,                                                                                               ;L1232
104891|  br label %56                                                                                                          ;L1224
104892| 
104893| 88: ; preds = %62
104894|  call void @core::option13unwrap_failed(ptr @anon.afb017e85830050c32e5158c55060899.97) #25                             ;L1013<1226
104895|  unreachable                                                                                                           ;L1013<1226
104896| 
104897| 89: ; preds = %90, %56
104898|  ret void                                                                                                              ;L1238
104899| 
104900| 90: ; preds = %56
104901|  %91 = load i64, ptr %29, , !!8                                                                                        ;L1236
104902|  %92 = load i64, ptr %31, , !!8                                                                                        ;L1236
104904|  store float 1.000000e+00, ptr %6,                                                                                     ;L60<1236
104905|  %93 = gep %6, i64 4                                                                                                   ;L60<1236
104906|  store float 1.000000e+00, ptr %93,                                                                                    ;L60<1236
104907|  %94 = gep %6, i64 8                                                                                                   ;L60<1236
104908|  store float 0.000000e+00, ptr %94,                                                                                    ;L60<1236
104909|  %95 = gep %6, i64 12                                                                                                  ;L60<1236
104910|  store float 1.000000e+00, ptr %95,                                                                                    ;L60<1236
104911|  call void @gc::simulation4game5frameNtB4_14DebugFrameData8add_line(ptr %4, i64 %26, i64 %28, i64 %91, i64 %92, ptr %6) ;L1236
104913|  br label %89                                                                                                          ;L1235
104914| }
