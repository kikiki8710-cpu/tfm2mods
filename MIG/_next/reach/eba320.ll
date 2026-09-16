 30617| define void @ai::fight_check22v48_projectile_profile(ptr sret([32 x i8]) %0, ptr %1, ptr %2, i8 %3) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 30618|  %5 = alloca [40 x i8],
 30619|  %6 = alloca [8 x i8],
 30620|  %7 = alloca [40 x i8],
 30621|  %8 = alloca [168 x i8],
 30622|  %9 = alloca [320 x i8],
 30623|  %10 = alloca [24 x i8],
 30624|  %11 = alloca [32 x i8],
 30625|  %12 = alloca [32 x i8],
 30626|  %13 = alloca [24 x i8],
 30627|  %14 = alloca [32 x i8],
 30628|  %15 = alloca [8 x i8],
 30629|  %16 = alloca [1 x i8],
 30630|  store i8 %3, ptr %16,
 30631|     ;; v = ptr %0
 30632|     ;; data = ptr %1
 30633|     ;; caster = ptr %2
 30634|     ;; slot = ptr %16
 30635|     ;; seed = ptr %15
 30636|     ;; cached = ptr %14
 30637|     ;; profile = ptr %12
 30639|  %17 = load ptr, ptr %1, , !!8, !!8                                                                                    ;L185
 30640|  %18 = load ptr, ptr %17, , !!8, !!8                                                                                   ;L185
 30641|  %19 = gep %17, i64 8                                                                                                  ;L185
 30642|  %20 = load ptr, ptr %19, , !!8, !!8                                                                                   ;L185
 30643|  %21 = gep %20, i64 32                                                                                                 ;L185
 30644|  %22 = load ptr, ptr %21, , !!8                                                                                        ;L185
 30645|  %23 = tail call i64 %22(ptr %18)                                                                                      ;L185
 30646|  store i64 %23, ptr %15,                                                                                               ;L185
 30649|  store ptr %15, ptr %13,                                                                                               ;L186
 30650|  %24 = gep %13, i64 8                                                                                                  ;L186
 30651|  store ptr %2, ptr %24,                                                                                                ;L186
 30652|  %25 = gep %13, i64 16                                                                                                 ;L186
 30653|  store ptr %16, ptr %25,                                                                                               ;L186
 30654|  call void @core::cell7RefCellNtNtCshdEBA0ozCnw_7game_ai11fight_check15V48ProfileCacheEE4withNCNvB1x_22v48_projectile_profile0INtNtBZ_6option6OptionIB34_TyyyEEEEB1z_(ptr sret([32 x i8]) %14, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.194, ptr %13) ;L186
 30656|  %26 = load i64, ptr %14, , !!8                                                                                        ;L196
 30657|  %27 = icmp eq i64 %26, -1                                                                                             ;L196
 30658|  br i1 %27, label %29, label %28                                                                                       ;L196
 30659| 
 30660| 28: ; preds = %4
 30661|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %14, i64 32, i1 false)                                                   ;L196
 30662|  br label %195                                                                                                         ;L239
 30663| 
 30664| 29: ; preds = %4
 30666|  %30 = gep %1, i64 8                                                                                                   ;L199
 30667|  %31 = load ptr, ptr %30, , !!8, !!8                                                                                   ;L199
 30674|     ;; target = ptr %10
 30675|     ;; probe_rnd = ptr %9
 30676|     ;; expected = ptr %8
 30677|     ;; self[8..+40] = ptr %5
 30678|     ;; self[8..+40] = ptr %5
 30681|     ;; init = ptr null
 30683|     ;; caster = ptr %2
 30684|  %32 = load i8, ptr %16, , !!41786, !!8                                                                                ;L200<199
 30685|     ;; e = ptr %2
 30686|     ;; self = ptr %2
 30687|     ;; self = ptr %2
 30688|     ;; slot = i8 %32
 30689|  switch i8 %32, label %33 [
 30690|  i8 0, label %42
 30691|  i8 1, label %48
 30692|  ]                                                                                                                     ;L174<200<199
 30693| 
 30694| 33: ; preds = %29
 30695|  %34 = gep %2, i64 1480                                                                                                ;L1701<177<200<199
 30696|  %35 = load i64, ptr %34, , !!41786, !!8                                                                               ;L1701<177<200<199
 30697|  %36 = icmp ugt i64 %35, 4                                                                                             ;L1701<177<200<199
 30698|  %37 = gep %2, i64 1336                                                                                                ;L1701<177<200<199
 30699|  %38 = select i1 %36, ptr %37, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.58                                           ;L1701<177<200<199
 30700|     ;; self = ptr %38
 30701|  %39 = gep %38, i64 48                                                                                                 ;L742<177<200<199
 30702|  %40 = load i32, ptr %39, , !!41786, !!8                                                                               ;L742<177<200<199
 30703|  %41 = icmp eq i32 %40, -1                                                                                             ;L742<177<200<199
 30704|  br i1 %41, label %62, label %57                                                                                       ;L742<177<200<199
 30705| 
 30706| 42: ; preds = %29
 30707|     ;; self = ptr %2
 30708|  %43 = gep %2, i64 1272                                                                                                ;L742<175<200<199
 30709|  %44 = load i32, ptr %43, , !!41786, !!8                                                                               ;L742<175<200<199
 30710|  %45 = icmp eq i32 %44, -1                                                                                             ;L742<175<200<199
 30711|  br i1 %45, label %62, label %46                                                                                       ;L742<175<200<199
 30712| 
 30713| 46: ; preds = %42
 30714|  %47 = gep %2, i64 1224
 30715|  br label %57                                                                                                          ;L742<175<200<199
 30716| 
 30717| 48: ; preds = %29
 30718|  %49 = gep %2, i64 1480                                                                                                ;L1693<176<200<199
 30719|  %50 = load i64, ptr %49, , !!41786, !!8                                                                               ;L1693<176<200<199
 30720|  %51 = icmp ugt i64 %50, 2                                                                                             ;L1693<176<200<199
 30721|  %52 = gep %2, i64 1280                                                                                                ;L1693<176<200<199
 30722|  %53 = select i1 %51, ptr %52, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.58                                           ;L1693<176<200<199
 30723|     ;; self = ptr %53
 30724|  %54 = gep %53, i64 48                                                                                                 ;L742<176<200<199
 30725|  %55 = load i32, ptr %54, , !!41786, !!8                                                                               ;L742<176<200<199
 30726|  %56 = icmp eq i32 %55, -1                                                                                             ;L742<176<200<199
 30727|  br i1 %56, label %62, label %57                                                                                       ;L742<176<200<199
 30728| 
 30729| 57: ; preds = %48, %46, %33
 30730|  %58 = phi i32 [ %44, %46 ], [ %55, %48 ], [ %40, %33 ]                                                                ;L149<201<199
 30731|  %59 = phi ptr [ %47, %46 ], [ %53, %48 ], [ %38, %33 ]                                                                ;L0<0<200<199
 30732|     ;; effect = ptr %59
 30733|     ;; self = ptr %59
 30734|     ;; self = ptr %59
 30735|  %60 = add nsw i32 %58, -1                                                                                             ;L149<201<199
 30736|  %61 = icmp ult i32 %60, 2                                                                                             ;L149<201<199
 30737|  br i1 %61, label %64, label %63                                                                                       ;L149<201<199
 30738| 
 30739| 62: ; preds = %48, %42, %33
 30740|  store i64 0, ptr %12, , !!41848                                                                                       ;L2790<200<199
 30741|  br label %191                                                                                                         ;L1<199
 30742| 
 30743| 63: ; preds = %57
 30744|  store i64 0, ptr %12, , !!41848                                                                                       ;L202<199
 30745|  br label %191                                                                                                         ;L1<199
 30746| 
 30747| 64: ; preds = %57
 30748|  %65 = gep %59, i64 16                                                                                                 ;L26<204<199
 30749|  %66 = load i64, ptr %65, , !!41786, !!8                                                                               ;L26<204<199
 30750|  %67 = gep %59, i64 24                                                                                                 ;L26<204<199
 30751|  %68 = load i64, ptr %67, , !!41786, !!8                                                                               ;L26<204<199
 30752|  %69 = gep %2, i64 1480                                                                                                ;L26<204<199
 30753|  %70 = load i64, ptr %69, , !!41786, !!8                                                                               ;L26<204<199
 30754|  %71 = add i64 %70, -1                                                                                                 ;L26<204<199
 30755|  %72 = mul i64 %71, %68                                                                                                ;L26<204<199
 30756|  %73 = gep %2, i64 1080                                                                                                ;L26<204<199
 30757|  %74 = load i64, ptr %73, , !!41786, !!8                                                                               ;L26<204<199
 30758|  %75 = add i64 %74, %66                                                                                                ;L26<204<199
 30759|  %76 = add i64 %75, %72                                                                                                ;L26<204<199
 30760|     ;; self = i64 %76
 30761|     ;; other = i64 1
 30762|  %77 = call i64 @llvm.umax.i64(i64 %76, i64 1)                                                                         ;L1039<204<199
 30763|     ;; range = i64 %77
 30764|  %78 = gep %31, i64 32                                                                                                 ;L205<199
 30765|  %79 = load ptr, ptr %78, , !!41786, !!8, !!8                                                                          ;L205<199
 30766|  %80 = gep %31, i64 8                                                                                                  ;L205<199
 30767|  %81 = load ptr, ptr %80, , !!41786, !!8, !!8                                                                          ;L205<199
 30768|  %82 = gep %2, i64 1632                                                                                                ;L205<199
 30769|  %83 = load i64, ptr %82, , !!41786, !!8                                                                               ;L205<199
 30770|  %84 = add i64 %83, %77                                                                                                ;L205<199
 30771|  %85 = gep %2, i64 1640                                                                                                ;L205<199
 30772|  %86 = load i64, ptr %85, , !!41786, !!8                                                                               ;L205<199
 30773|  %87 = call { i64, i64 } @gc::simulation4gameNtB5_4Game15adjust_position(ptr %79, ptr %81, i64 %84, i64 %86), !!41786  ;L205<199
 30776|  %88 = icmp eq i32 %58, 1                                                                                              ;L206<199
 30777|  %89 = extractvalue { i64, i64 } %87, 1                                                                                ;L206<199
 30778|  %90 = extractvalue { i64, i64 } %87, 0                                                                                ;L206<199
 30779|  %91 = select i1 %88, i64 %90, i64 1000                                                                                ;L206<199
 30780|  %92 = select i1 %88, i64 %89, i64 0                                                                                   ;L206<199
 30781|  %93 = select i1 %88, i32 2, i32 1                                                                                     ;L206<199
 30782|  %94 = gep %10, i64 8                                                                                                  ;L0<199
 30783|  store i64 %91, ptr %94, , !!41786                                                                                     ;L0<199
 30784|  %95 = gep %10, i64 16                                                                                                 ;L0<199
 30785|  store i64 %92, ptr %95, , !!41786                                                                                     ;L0<199
 30786|  store i32 %93, ptr %10, , !!41786                                                                                     ;L0<199
 30788|     ;; self = ptr %2
 30789|  %96 = gep %2, i64 600                                                                                                 ;L3054<1870<211<199
 30790|  %97 = load i64, ptr %96, , !!41786, !!8                                                                               ;L3054<1870<211<199
 30791|  %98 = icmp sgt i64 %97, -1                                                                                            ;L3059<1870<211<199
 30792|  call void @llvm.assume(i1 %98)                                                                                        ;L3059<1870<211<199
 30793|  %99 = shl i64 %97, 16                                                                                                 ;L211<199
 30794|  %100 = or disjoint i64 %99, 30280                                                                                     ;L211<199
 30795|  %101 = zext i8 %32 to i64                                                                                             ;L211<199
 30796|  %102 = shl nuw nsw i64 %101, 8                                                                                        ;L211<199
 30797|  %103 = xor i64 %100, %102                                                                                             ;L211<199
 30798|  call void @_RNvYNtNtNtCsMBkRBYhlca_4rand4rngs3std6StdRngNtCsi8xCTVceeg2_9rand_core11SeedableRng13seed_from_u64CshdEBA0ozCnw_7game_ai(ptr sret([320 x i8]) %9, i64 %103), !!41786 ;L211<199
 30800|  call void @gc::simulation13expected_gameNtB2_12ExpectedGame9from_game(ptr sret([168 x i8]) %8, ptr %9, ptr %18, ptr %20), !!41786 ;L212<199
 30801|     ;; self = ptr %59
 30802|     ;; self = ptr %59
 30803|  %104 = load ptr, ptr %59, , !!41786, !!8, !!8                                                                         ;L441<2127<2445<213<199
 30804|  %105 = gep %59, i64 8                                                                                                 ;L441<2127<2445<213<199
 30805|  %106 = load ptr, ptr %105, , !!41786, !!8, !!8                                                                        ;L441<2127<2445<213<199
 30806|  %107 = gep %106, i64 16                                                                                               ;L2445<213<199
 30807|  %108 = load i64, ptr %107, , !!41786                                                                                  ;L2445<213<199
 30808|  %109 = add nsw i64 %108, -1                                                                                           ;L2445<213<199
 30809|  %110 = and i64 %109, -16                                                                                              ;L2445<213<199
 30810|  %111 = gep %104, i64 %110                                                                                             ;L2445<213<199
 30811|  %112 = gep %111, i64 16                                                                                               ;L2445<213<199
 30812|  %113 = gep %2, i64 1472                                                                                               ;L213<199
 30813|  %114 = load i64, ptr %113, , !!41786, !!8                                                                             ;L213<199
 30814|  %115 = gep %59, i64 44                                                                                                ;L213<199
 30815|  %116 = load i32, ptr %115, , !!41786, !!8                                                                             ;L213<199
 30817|  store i64 -2, ptr %7, , !!41786                                                                                       ;L213<199
 30819|  store ptr null, ptr %6, , !!41786                                                                                     ;L213<199
 30820|  %117 = gep %106, i64 32                                                                                               ;L213<199
 30821|  %118 = load ptr, ptr %117, , !!41786, !!8                                                                             ;L213<199
 30822|  invoke void %118(ptr %112, ptr %9, ptr %8, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.80, ptr %31, i64 %114, ptr %10, i32 %116, ptr %7, ptr %6)
 30823|  to label %121 unwind label %119, !!41786                                                                              ;L213<199
 30824| 
 30825| 119: ; preds = %165, %122, %121, %64
 30826|  %120 = cleanuppad within none []
 30827|  call void @core::ptr9drop_glueNtNtNtCs97f5S1uJLkH_9game_core10simulation13expected_game12ExpectedGameECshdEBA0ozCnw_7game_ai(ptr %8) #32 [ "funclet"(token %120) ], !!41786 ;L231<199
 30828|  cleanupret from %120 unwind to caller                                                                                 ;L199<199
 30829| 
 30830| 121: ; preds = %64
 30833|  invoke void @gc::simulation13expected_gameNtB4_12ExpectedGameNtB6_12AbstractGame15iter_projectile(ptr sret([40 x i8]) %5, ptr %8)
 30834|  to label %122 unwind label %119, !!41786                                                                              ;L218<199
 30835| 
 30836| 122: ; preds = %139, %121
 30837|  %123 = phi ptr [ %140, %139 ], [ null, %121 ]                                                                         ;L0<165<271<218<199
 30838|     ;; accum = ptr %123
 30839|  %124 = invoke ptr @gc::simulationNtB5_14ProjectileIterNtNtNtNtCsjihNppCmMEE_4core4iter6traits8iterator8Iterator4next(ptr %5)
 30840|  to label %125 unwind label %119, !!41786                                                                              ;L2670<165<271<218<199
 30841| 
 30842| 125: ; preds = %122
 30843|  %126 = icmp eq ptr %124, null                                                                                         ;L2670<165<271<218<199
 30844|  br i1 %126, label %141, label %127                                                                                    ;L2670<165<271<218<199
 30845| 
 30846| 127: ; preds = %125
 30847|     ;; x = ptr %124
 30848|     ;; item = ptr %124
 30850|     ;; acc = ptr %123
 30853|  %128 = gep %124, i64 248                                                                                              ;L218<79<2671<165<271<218<199
 30854|  %129 = load i64, ptr %128, , !!41950, !!8                                                                             ;L218<79<2671<165<271<218<199
 30855|  %130 = icmp eq i64 %129, %114                                                                                         ;L218<79<2671<165<271<218<199
 30856|  br i1 %130, label %131, label %139                                                                                    ;L218<79<2671<165<271<218<199
 30857| 
 30858| 131: ; preds = %127
 30859|     ;; self = ptr %124
 30860|  %132 = gep %124, i64 64                                                                                               ;L208<218<79<2671<165<271<218<199
 30861|  %133 = load i64, ptr %132, , !!41950, !!8                                                                             ;L208<218<79<2671<165<271<218<199
 30862|  %134 = icmp ne i64 %133, 9                                                                                            ;L208<218<79<2671<165<271<218<199
 30863|  call void @llvm.assume(i1 %134)                                                                                       ;L208<218<79<2671<165<271<218<199
 30864|  %135 = add nsw i64 %133, -2                                                                                           ;L208<218<79<2671<165<271<218<199
 30865|  %136 = icmp samesign ugt i64 %133, 1                                                                                  ;L208<218<79<2671<165<271<218<199
 30866|  %137 = select i1 %136, i64 %135, i64 7                                                                                ;L208<218<79<2671<165<271<218<199
 30867|  switch i64 %137, label %139 [
 30868|  i64 0, label %138
 30869|  i64 1, label %138
 30870|  i64 2, label %138
 30871|  i64 8, label %138
 30872|  ]                                                                                                                     ;L208<218<79<2671<165<271<218<199
 30873| 
 30874| 138: ; preds = %131, %131, %131, %131
 30875|  br label %139                                                                                                         ;L79<2671<165<271<218<199
 30876| 
 30877| 139: ; preds = %138, %131, %127
 30878|  %140 = phi ptr [ %124, %138 ], [ %123, %127 ], [ %123, %131 ]                                                         ;L79<2671<165<271<218<199
 30879|     ;; accum = ptr %140
 30880|  br label %122                                                                                                         ;L2670<165<271<218<199
 30881| 
 30882| 141: ; preds = %125
 30883|     ;; self = ptr %123
 30884|  %142 = icmp eq ptr %123, null                                                                                         ;L2775<218<199
 30885|  br i1 %142, label %147, label %143                                                                                    ;L2775<218<199
 30886| 
 30887| 143: ; preds = %141
 30888|     ;; p = ptr %123
 30889|  %144 = gep %123, i64 16                                                                                               ;L219<199
 30890|  %145 = load i64, ptr %144, , !!41786, !!8                                                                             ;L219<199
 30891|  %146 = icmp eq i64 %145, 0                                                                                            ;L219<199
 30892|  br i1 %146, label %154, label %157                                                                                    ;L219<199
 30893| 
 30894| 147: ; preds = %141
 30895|  store i64 0, ptr %12, , !!41848                                                                                       ;L2790<218<199
 30897|  %148 = gep %8, i64 16                                                                                                 ;L825<231<199
 30901|  invoke void @gc::simulation6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %148)
 30902|  to label %152 unwind label %149, !!41786                                                                              ;L825<825<825<825<231<199
 30903| 
 30904| 149: ; preds = %147
 30905|  %150 = cleanuppad within none []
 30906|  %151 = gep %8, i64 80                                                                                                 ;L825<231<199
 30910|  call void @gc::simulation10projectile10ProjectileEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %151) [ "funclet"(token %150) ], !!41786 ;L825<825<825<825<231<199
 30911|  cleanupret from %150 unwind to caller                                                                                 ;L825<231<199
 30912| 
 30913| 152: ; preds = %147
 30914|  %153 = gep %8, i64 80                                                                                                 ;L825<231<199
 30918|  call void @gc::simulation10projectile10ProjectileEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %153), !!41786 ;L825<825<825<825<231<199
 30921|  br label %191                                                                                                         ;L1<199
 30922| 
 30923| 154: ; preds = %143
 30924|  %155 = gep %123, i64 24                                                                                               ;L220<199
 30925|  %156 = load i64, ptr %155, , !!41786, !!8                                                                             ;L220<199
 30926|     ;; halfwidth = i64 %156
 30927|  br label %157                                                                                                         ;L220<199
 30928| 
 30929| 157: ; preds = %154, %143
 30930|  %158 = phi i64 [ %156, %154 ], [ 20000, %143 ]                                                                        ;L0<199
 30931|     ;; halfwidth = i64 %158
 30932|  %159 = gep %123, i64 64                                                                                               ;L223<199
 30933|  %160 = load i64, ptr %159, , !!41786, !!8                                                                             ;L223<199
 30934|  %161 = icmp ne i64 %160, 9                                                                                            ;L223<199
 30935|  call void @llvm.assume(i1 %161)                                                                                       ;L223<199
 30936|  %162 = add nsw i64 %160, -2                                                                                           ;L223<199
 30937|  %163 = icmp samesign ugt i64 %160, 1                                                                                  ;L223<199
 30938|  %164 = select i1 %163, i64 %162, i64 7                                                                                ;L223<199
 30939|  switch i64 %164, label %165 [
 30940|  i64 0, label %167
 30941|  i64 1, label %170
 30942|  i64 2, label %173
 30943|  i64 8, label %176
 30944|  ]                                                                                                                     ;L223<199
 30945| 
 30946| 165: ; preds = %157
 30947|  %166 = invoke i64 @gc::simulation10projectileNtB5_10Projectile5speed(ptr %123)
 30948|  to label %179 unwind label %119, !!41786                                                                              ;L228<199
 30949| 
 30950| 167: ; preds = %157
 30951|     ;; speed = ptr %123
 30952|  %168 = gep %123, i64 96                                                                                               ;L224<199
 30953|  %169 = load i64, ptr %168, , !!41786, !!8                                                                             ;L224<199
 30954|     ;; speed = i64 %169
 30955|     ;; delay = i64 0
 30956|  br label %179                                                                                                         ;L224<199
 30957| 
 30958| 170: ; preds = %157
 30959|     ;; applyed = ptr %123
 30960|  %171 = gep %123, i64 80                                                                                               ;L225<199
 30961|  %172 = load i64, ptr %171, , !!41786, !!8                                                                             ;L225<199
 30962|     ;; delay = i64 %172
 30963|     ;; speed = i64 0
 30964|  br label %179                                                                                                         ;L225<199
 30965| 
 30966| 173: ; preds = %157
 30967|     ;; first_delay = ptr %123
 30968|  %174 = gep %123, i64 120                                                                                              ;L227<199
 30969|  %175 = load i64, ptr %174, , !!41786, !!8                                                                             ;L227<199
 30970|     ;; delay = i64 %175
 30971|     ;; speed = i64 0
 30972|  br label %179                                                                                                         ;L227<199
 30973| 
 30974| 176: ; preds = %157
 30975|     ;; tick = ptr %123
 30976|  %177 = gep %123, i64 96                                                                                               ;L226<199
 30977|  %178 = load i64, ptr %177, , !!41786, !!8                                                                             ;L226<199
 30978|     ;; delay = i64 %178
 30979|     ;; speed = i64 0
 30980|  br label %179                                                                                                         ;L226<199
 30981| 
 30982| 179: ; preds = %176, %173, %170, %167, %165
 30983|  %180 = phi i64 [ %178, %176 ], [ 0, %167 ], [ %172, %170 ], [ %175, %173 ], [ 0, %165 ]                               ;L0<199
 30984|  %181 = phi i64 [ 0, %176 ], [ %169, %167 ], [ 0, %170 ], [ 0, %173 ], [ %166, %165 ]                                  ;L0<199
 30985|     ;; speed = i64 %181
 30986|     ;; delay = i64 %180
 30987|  %182 = gep %12, i64 8                                                                                                 ;L230<199
 30988|  store i64 %181, ptr %182, , !!41848                                                                                   ;L230<199
 30989|  %183 = gep %12, i64 16                                                                                                ;L230<199
 30990|  store i64 %158, ptr %183, , !!41848                                                                                   ;L230<199
 30991|  %184 = gep %12, i64 24                                                                                                ;L230<199
 30992|  store i64 %180, ptr %184, , !!41848                                                                                   ;L230<199
 30993|  store i64 1, ptr %12, , !!41848                                                                                       ;L230<199
 30995|  %185 = gep %8, i64 16                                                                                                 ;L825<231<199
 30999|  invoke void @gc::simulation6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %185)
 31000|  to label %189 unwind label %186, !!41786                                                                              ;L825<825<825<825<231<199
 31001| 
 31002| 186: ; preds = %179
 31003|  %187 = cleanuppad within none []
 31004|  %188 = gep %8, i64 80                                                                                                 ;L825<231<199
 31008|  call void @gc::simulation10projectile10ProjectileEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %188) [ "funclet"(token %187) ], !!41786 ;L825<825<825<825<231<199
 31009|  cleanupret from %187 unwind to caller                                                                                 ;L825<231<199
 31010| 
 31011| 189: ; preds = %179
 31012|  %190 = gep %8, i64 80                                                                                                 ;L825<231<199
 31016|  call void @gc::simulation10projectile10ProjectileEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %190), !!41786 ;L825<825<825<825<231<199
 31019|  br label %191                                                                                                         ;L231<199
 31020| 
 31021| 191: ; preds = %189, %152, %63, %62
 31025|  store ptr %15, ptr %11,                                                                                               ;L232
 31026|  %192 = gep %11, i64 8                                                                                                 ;L232
 31027|  store ptr %2, ptr %192,                                                                                               ;L232
 31028|  %193 = gep %11, i64 16                                                                                                ;L232
 31029|  store ptr %16, ptr %193,                                                                                              ;L232
 31030|  %194 = gep %11, i64 24                                                                                                ;L232
 31031|  store ptr %12, ptr %194,                                                                                              ;L232
 31032|  call void @core::cell7RefCellNtNtCshdEBA0ozCnw_7game_ai11fight_check15V48ProfileCacheEE4withNCNvB1x_22v48_projectile_profiles0_0uEB1z_(ptr @anon.e5f9a016f1220e5cee86475419aee1e5.194, ptr %11) ;L232
 31034|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %12, i64 32, i1 false)                                                   ;L238
 31036|  br label %195                                                                                                         ;L239
 31037| 
 31038| 195: ; preds = %191, %28
 31041|  ret void                                                                                                              ;L239
 31042| }
