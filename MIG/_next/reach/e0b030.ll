 47870| define hidden void @ai::plan_legacy3old11fight_model26resolve_fight_stake_roster(ptr sret([64 x i8]) %0, i64 %1, ptr %2, ptr %3, ptr %4, i64 %5, ptr %6, i64 %7, i8 %8, ptr %9, i64 %10) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
 47871|  %12 = alloca [24 x i8],
 47872|  %13 = alloca [24 x i8],
 47873|  %14 = alloca [24 x i8],
 47874|  %15 = alloca [64 x i8],
 47875|  %16 = alloca [64 x i8],
 47876|  %17 = alloca [24 x i8],
 47877|  %18 = alloca [32 x i8],
 47879|  %19 = alloca [32 x i8],
 47880|  %20 = alloca [32 x i8],
 47881|     ;; absolute = ptr %0
 47882|     ;; version = i64 %1
 47883|     ;; data = ptr %2
 47884|     ;; champ = ptr %3
 47885|     ;; roster[0..+8] = ptr %4
 47886|     ;; self[0..+8] = ptr %4
 47887|     ;; slice[0..+8] = ptr %4
 47888|     ;; self[0..+8] = ptr %4
 47889|     ;; slice[0..+8] = ptr %4
 47890|     ;; roster[8..+8] = i64 %5
 47891|     ;; self[8..+8] = i64 %5
 47892|     ;; slice[8..+8] = i64 %5
 47893|     ;; self[8..+8] = i64 %5
 47894|     ;; slice[8..+8] = i64 %5
 47895|     ;; near_enemies[0..+8] = ptr %6
 47896|     ;; near_enemies[8..+8] = i64 %7
 47897|     ;; committed_dir = i8 %8
 47898|     ;; tower = ptr %9
 47899|     ;; judge_accuracy = i64 %10
 47900|     ;; allies = ptr %20
 47901|     ;; arrivals = ptr %19
 47902|     ;; remaining = ptr %18
 47903|     ;; abandon = ptr %16
 47904|     ;; diff = ptr %15
 47905|     ;; count = i64 1
 47909|  %21 = gep %2, i64 8                                                                                                   ;L648
 47910|  %22 = load ptr, ptr %21, , !!8, !!8                                                                                   ;L648
 47911|  %23 = load ptr, ptr %22, , !!8, !!8                                                                                   ;L648
 47912|     ;; bump = ptr %23
 47913|     ;; bump = ptr %23
 47914|  store ptr inttoptr (i64 8 to ptr), ptr %20,                                                                           ;L547<648
 47915|  %24 = gep %20, i64 8                                                                                                  ;L547<648
 47916|  store ptr %23, ptr %24,                                                                                               ;L547<648
 47917|  %25 = gep %20, i64 16                                                                                                 ;L547<648
 47918|  %26 = gep %20, i64 24                                                                                                 ;L547<648
 47919|  call void @llvm.memset.p0.i64(ptr %25, i8 0, i64 16, i1 false)                                                        ;L547<648
 47921|  store ptr inttoptr (i64 8 to ptr), ptr %19,                                                                           ;L547<649
 47922|  %27 = gep %19, i64 8                                                                                                  ;L547<649
 47923|  store ptr %23, ptr %27,                                                                                               ;L547<649
 47924|  %28 = gep %19, i64 16                                                                                                 ;L547<649
 47925|  %29 = gep %19, i64 24                                                                                                 ;L547<649
 47926|     ;; len = i64 %5
 47927|     ;; count = i64 %5
 47928|     ;; count = i64 %5
 47929|     ;; ptr = ptr %4
 47930|     ;; self = ptr %4
 47931|  %30 = gepS %4, i64 %5                                                                                                 ;L961<100<1042<650
 47932|     ;; iter[0..+8] = ptr %4
 47933|     ;; iter[8..+8] = ptr %30
 47934|  call void @llvm.memset.p0.i64(ptr %28, i8 0, i64 16, i1 false)                                                        ;L547<649
 47935|  br label %31                                                                                                          ;L650
 47936| 
 47937| 31: ; preds = %64, %11
 47938|  %32 = phi i64 [ 0, %11 ], [ %69, %64 ]
 47939|  %33 = phi ptr [ %4, %11 ], [ %36, %64 ]                                                                               ;L650
 47940|     ;; iter[0..+8] = ptr %33
 47941|     ;; self = ptr undef
 47942|     ;; ptr = ptr %33
 47943|     ;; self = ptr %33
 47944|     ;; end_or_len = ptr %30
 47947|  %34 = icmp eq ptr %33, %30                                                                                            ;L1714<180<650
 47948|  br i1 %34, label %44, label %35                                                                                       ;L180<650
 47949| 
 47950| 35: ; preds = %31
 47951|  %36 = gep %33, i64 24                                                                                                 ;L656<185<650
 47952|     ;; iter[0..+8] = ptr %36
 47953|     ;; a = ptr %33
 47954|     ;; t = ptr %33
 47955|  %37 = load ptr, ptr %33, , !!8, !!8                                                                                   ;L651
 47956|     ;; self = ptr %20
 47957|     ;; self = ptr %20
 47958|     ;; value = ptr %37
 47959|     ;; additional = i64 1
 47960|     ;; needed_extra_cap = i64 1
 47961|     ;; needed_extra_cap = i64 1
 47962|     ;; strategy = i8 1
 47963|  %38 = load i64, ptr %26, , !!53669, !!8                                                                               ;L1428<651
 47964|     ;; self = ptr %20
 47965|  %39 = load i64, ptr %25, , !!53669, !!8                                                                               ;L149<1428<651
 47966|  %40 = icmp eq i64 %38, %39                                                                                            ;L1428<651
 47967|  br i1 %40, label %41, label %50                                                                                       ;L1428<651
 47968| 
 47969| 41: ; preds = %35
 47970|     ;; self = ptr %20
 47971|     ;; self = ptr %20
 47972|     ;; self = ptr %20
 47973|     ;; used_cap = i64 %38
 47974|     ;; used_cap = i64 %38
 47975|  invoke void @gc::simulation6entity6EntityE25reserve_internal_or_panicB1a_(ptr %20, i64 %38, i64 1, i1 zeroext true)
 47976|  to label %42 unwind label %48                                                                                         ;L619<430<738<1429<651
 47977| 
 47978| 42: ; preds = %41
 47979|  %43 = load i64, ptr %26, , !!53669                                                                                    ;L1432<651
 47980|  br label %50                                                                                                          ;L1428<651
 47981| 
 47982| 44: ; preds = %31
 47983|     ;; self = ptr %20
 47984|     ;; self = ptr %20
 47985|  %45 = load ptr, ptr %20, , !!8, !!8                                                                                   ;L138<2073<654
 47986|     ;; p = ptr %45
 47987|  %46 = load i64, ptr %26, , !!8                                                                                        ;L2075<654
 47988|     ;; self = ptr %19
 47989|     ;; self = ptr %19
 47990|  %47 = load ptr, ptr %19, , !!8, !!8                                                                                   ;L138<2073<654
 47991|     ;; version = i64 %1
 47992|     ;; data = ptr %2
 47993|     ;; champ = ptr %3
 47994|     ;; near_allies[0..+8] = ptr %45
 47995|     ;; near_allies[8..+8] = i64 %46
 47996|     ;; near_enemies[0..+8] = ptr %6
 47997|     ;; near_enemies[8..+8] = i64 %7
 47998|     ;; committed_dir = i8 %8
 47999|     ;; tower = ptr %9
 48000|     ;; judge_accuracy = i64 %10
 48001|     ;; arrivals[0..+8] = ptr %47
 48002|     ;; arrivals[8..+8] = i64 %32
 48003|  invoke fastcc void @ai::plan_legacy3old11fight_model18resolve_fight_full(ptr %0, i64 %1, ptr %2, ptr %3, ptr %45, i64 %46, ptr %6, i64 %7, i8 %8, ptr %9, i64 %10, ptr %47, i64 %32, i64 0)
 48004|  to label %70 unwind label %48                                                                                         ;L319<654
 48005| 
 48006| 48: ; preds = %115, %114, %112, %95, %81, %80, %78, %70, %61, %44, %41
 48007|  %49 = cleanuppad within none []
 48008|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecxEECshdEBA0ozCnw_7game_ai(ptr %19) #28 [ "funclet"(token %49) ] ;L669
 48009|  cleanupret from %49 unwind label %89                                                                                  ;L669
 48010| 
 48011| 50: ; preds = %42, %35
 48012|  %51 = phi i64 [ %38, %35 ], [ %43, %42 ]                                                                              ;L1432<651
 48013|     ;; self = ptr %20
 48014|  %52 = load ptr, ptr %20, , !!53669, !!8, !!8                                                                          ;L138<1432<651
 48015|     ;; self = ptr %52
 48016|     ;; count = i64 %51
 48017|  %53 = getelementptr ptr, ptr %52, i64 %51                                                                             ;L961<1432<651
 48018|     ;; end = ptr %53
 48019|     ;; dst = ptr %53
 48020|     ;; src = ptr %37
 48021|  store ptr %37, ptr %53, , !!53669                                                                                     ;L1933<1433<651
 48022|  %54 = load i64, ptr %26, , !!53669, !!8                                                                               ;L1434<651
 48023|  %55 = add i64 %54, 1                                                                                                  ;L1434<651
 48024|  store i64 %55, ptr %26, , !!53669                                                                                     ;L1434<651
 48025|  %56 = gep %33, i64 8                                                                                                  ;L652
 48026|  %57 = load i64, ptr %56, , !!8                                                                                        ;L652
 48027|     ;; self = ptr %19
 48028|     ;; self = ptr %19
 48029|     ;; value = i64 %57
 48030|     ;; additional = i64 1
 48031|     ;; needed_extra_cap = i64 1
 48032|     ;; needed_extra_cap = i64 1
 48033|     ;; strategy = i8 1
 48034|  %58 = load i64, ptr %29, , !!8                                                                                        ;L1428<652
 48035|     ;; self = ptr %19
 48036|  %59 = load i64, ptr %28, , !!8                                                                                        ;L149<1428<652
 48037|  %60 = icmp eq i64 %58, %59                                                                                            ;L1428<652
 48038|  br i1 %60, label %61, label %64                                                                                       ;L1428<652
 48039| 
 48040| 61: ; preds = %50
 48041|     ;; self = ptr %19
 48042|     ;; self = ptr %19
 48043|     ;; self = ptr %19
 48044|     ;; used_cap = i64 %58
 48045|     ;; used_cap = i64 %58
 48046|  invoke void @_RNvMs2_NtNtCshWfHDMLkPaX_7bumpalo11collections7raw_vecINtB5_6RawVecxE25reserve_internal_or_panicCshdEBA0ozCnw_7game_ai(ptr %19, i64 %58, i64 1, i1 zeroext true)
 48047|  to label %62 unwind label %48                                                                                         ;L619<430<738<1429<652
 48048| 
 48049| 62: ; preds = %61
 48050|  %63 = load i64, ptr %29,                                                                                              ;L1432<652
 48051|  br label %64                                                                                                          ;L1428<652
 48052| 
 48053| 64: ; preds = %62, %50
 48054|  %65 = phi i64 [ %58, %50 ], [ %63, %62 ]                                                                              ;L1432<652
 48055|     ;; self = ptr %19
 48056|  %66 = load ptr, ptr %19, , !!8, !!8                                                                                   ;L138<1432<652
 48057|     ;; self = ptr %66
 48058|     ;; count = i64 %65
 48059|  %67 = getelementptr i64, ptr %66, i64 %65                                                                             ;L961<1432<652
 48060|     ;; end = ptr %67
 48061|     ;; dst = ptr %67
 48062|     ;; src = i64 %57
 48063|  store i64 %57, ptr %67,                                                                                               ;L1933<1433<652
 48064|  %68 = load i64, ptr %29, , !!8                                                                                        ;L1434<652
 48065|  %69 = add i64 %68, 1                                                                                                  ;L1434<652
 48066|  store i64 %69, ptr %29,                                                                                               ;L1434<652
 48067|  br label %31                                                                                                          ;L1436<652
 48068| 
 48069| 70: ; preds = %44
 48072|     ;; self = ptr %4
 48073|     ;; self[0..+8] = ptr %4
 48074|     ;; self[8..+8] = ptr %30
 48075|     ;; self[16..+8] = ptr %3
 48076|  store ptr %4, ptr %17,                                                                                                ;L69<836<656
 48077|  %71 = gep %17, i64 8                                                                                                  ;L69<836<656
 48078|  store ptr %30, ptr %71,                                                                                               ;L69<836<656
 48079|  %72 = gep %17, i64 16                                                                                                 ;L69<836<656
 48080|  store ptr %3, ptr %72,                                                                                                ;L69<836<656
 48081|  invoke void @core::iter8adapters3map3MapINtNtB29_6filter6FilterINtNtNtB2d_5slice4iter4IterTBU_xbEENCNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model26resolve_fight_stake_roster0ENCB3Q_s_0EEB3Y_(ptr sret([32 x i8]) %18, ptr %17, ptr %23)
 48082|  to label %73 unwind label %48                                                                                         ;L655
 48083| 
 48084| 73: ; preds = %70
 48086|     ;; self = ptr %18
 48087|     ;; self = ptr %18
 48088|  %74 = gep %18, i64 24                                                                                                 ;L1617<1636<658
 48089|  %75 = load i64, ptr %74, , !!8                                                                                        ;L1617<1636<658
 48090|  %76 = icmp eq i64 %75, 0                                                                                              ;L658
 48091|  br i1 %76, label %77, label %82                                                                                       ;L658
 48092| 
 48093| 77: ; preds = %73
 48095|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %18)
 48096|  to label %81 unwind label %78                                                                                         ;L825<669
 48097| 
 48098| 78: ; preds = %77
 48099|  %79 = cleanuppad within none []
 48101|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %18) [ "funclet"(token %79) ]
 48102|  to label %80 unwind label %48                                                                                         ;L825<825<669
 48103| 
 48104| 80: ; preds = %78
 48105|  cleanupret from %79 unwind label %48
 48106| 
 48107| 81: ; preds = %77
 48109|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %18)
 48110|  to label %84 unwind label %48                                                                                         ;L825<825<669
 48111| 
 48112| 82: ; preds = %73
 48114|     ;; self = ptr %18
 48115|     ;; self = ptr %18
 48116|  %83 = load ptr, ptr %18, , !!8, !!8                                                                                   ;L138<2073<661
 48117|     ;; version = i64 %1
 48118|     ;; data = ptr %2
 48119|     ;; champ = ptr %3
 48120|     ;; near_allies[0..+8] = ptr %83
 48121|     ;; near_allies[8..+8] = i64 %75
 48122|     ;; near_enemies[0..+8] = ptr %6
 48123|     ;; near_enemies[8..+8] = i64 %7
 48124|     ;; committed_dir = i8 0
 48125|     ;; tower = ptr %9
 48126|     ;; judge_accuracy = i64 %10
 48127|  invoke fastcc void @ai::plan_legacy3old11fight_model18resolve_fight_full(ptr %16, i64 %1, ptr %2, ptr %3, ptr %83, i64 %75, ptr %6, i64 %7, i8 0, ptr %9, i64 %10, ptr inttoptr (i64 8 to ptr), i64 0, i64 0)
 48128|  to label %97 unwind label %95                                                                                         ;L311<661
 48129| 
 48130| 84: ; preds = %81
 48133|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %19)
 48134|  to label %88 unwind label %85                                                                                         ;L825<669
 48135| 
 48136| 85: ; preds = %84
 48137|  %86 = cleanuppad within none []
 48139|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %19) [ "funclet"(token %86) ]
 48140|  to label %87 unwind label %89                                                                                         ;L825<825<669
 48141| 
 48142| 87: ; preds = %85
 48143|  cleanupret from %86 unwind label %89
 48144| 
 48145| 88: ; preds = %84
 48147|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %19)
 48148|  to label %91 unwind label %89                                                                                         ;L825<825<669
 48149| 
 48150| 89: ; preds = %168, %167, %165, %88, %87, %85, %48
 48151|  %90 = cleanuppad within none []
 48152|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEECshdEBA0ozCnw_7game_ai(ptr %20) #28 [ "funclet"(token %90) ] ;L669
 48153|  cleanupret from %90 unwind to caller                                                                                  ;L645
 48154| 
 48155| 91: ; preds = %88
 48158|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %20)
 48159|  to label %94 unwind label %92                                                                                         ;L825<669
 48160| 
 48161| 92: ; preds = %91
 48162|  %93 = cleanuppad within none []
 48164|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %20) [ "funclet"(token %93) ] ;L825<825<669
 48165|  cleanupret from %93 unwind to caller                                                                                  ;L825<669
 48166| 
 48167| 94: ; preds = %169, %91
 48168|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %20)   ;L825<825<669
 48170|  ret void                                                                                                              ;L669
 48171| 
 48172| 95: ; preds = %125, %97, %82
 48173|  %96 = cleanuppad within none []
 48174|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEECshdEBA0ozCnw_7game_ai(ptr %18) #28 [ "funclet"(token %96) ] ;L669
 48175|  cleanupret from %96 unwind label %48                                                                                  ;L669
 48176| 
 48177| 97: ; preds = %82
 48179|     ;; self = ptr %20
 48180|     ;; self = ptr %20
 48181|  %98 = load ptr, ptr %20, , !!8, !!8                                                                                   ;L138<2073<662
 48182|     ;; p = ptr %98
 48183|  %99 = load i64, ptr %26, , !!8                                                                                        ;L2075<662
 48184|     ;; self = ptr %19
 48185|     ;; self = ptr %19
 48186|  %100 = load ptr, ptr %19, , !!8, !!8                                                                                  ;L138<2073<663
 48187|     ;; p = ptr %100
 48188|  %101 = load i64, ptr %29, , !!8                                                                                       ;L2075<663
 48189|  %102 = gep %16, i64 48                                                                                                ;L663
 48190|  %103 = load i64, ptr %102, , !!8                                                                                      ;L663
 48191|  invoke fastcc void @ai::plan_legacy3old11fight_model18resolve_fight_full(ptr %15, i64 %1, ptr %2, ptr %3, ptr %98, i64 %99, ptr %6, i64 %7, i8 %8, ptr %9, i64 %10, ptr %100, i64 %101, i64 %103)
 48192|  to label %104 unwind label %95                                                                                        ;L662
 48193| 
 48194| 104: ; preds = %97
 48195|  %105 = gep %0, i64 56                                                                                                 ;L664
 48196|  %106 = load i8, ptr %105, , !!8                                                                                       ;L664
 48197|  %107 = gep %15, i64 57                                                                                                ;L664
 48198|  store i8 %106, ptr %107,                                                                                              ;L664
 48199|     ;; self = ptr %15
 48200|     ;; self = ptr %15
 48201|     ;; other = ptr %0
 48202|     ;; other = ptr %0
 48203|  %108 = gep %15, i64 56                                                                                                ;L243<264<665
 48204|  %109 = load i8, ptr %108, , !!8                                                                                       ;L243<264<665
 48205|     ;; __self_discr = i8 %109
 48206|     ;; __arg1_discr = i8 %106
 48207|  %110 = icmp eq i8 %109, %106                                                                                          ;L243<264<665
 48208|  br i1 %110, label %111, label %116                                                                                    ;L665
 48209| 
 48210| 111: ; preds = %159, %104
 48211|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %15, i64 64, i1 false)                                                   ;L668
 48215|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %18)
 48216|  to label %115 unwind label %112                                                                                       ;L825<669
 48217| 
 48218| 112: ; preds = %111
 48219|  %113 = cleanuppad within none []
 48221|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %18) [ "funclet"(token %113) ]
 48222|  to label %114 unwind label %48                                                                                        ;L825<825<669
 48223| 
 48224| 114: ; preds = %112
 48225|  cleanupret from %113 unwind label %48
 48226| 
 48227| 115: ; preds = %111
 48229|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %18)
 48230|  to label %164 unwind label %48                                                                                        ;L825<825<669
 48231| 
 48232| 116: ; preds = %104
 48233|     ;; self = ptr %18
 48234|     ;; self = ptr %18
 48235|  %117 = load ptr, ptr %18, , !!8, !!8                                                                                  ;L138<2073<666
 48236|     ;; p = ptr %117
 48237|  %118 = load i64, ptr %74, , !!8                                                                                       ;L2075<666
 48238|     ;; len = i64 %118
 48239|     ;; count = i64 %118
 48240|     ;; self[0..+8] = ptr %117
 48241|     ;; slice[0..+8] = ptr %117
 48242|     ;; self[8..+8] = i64 %118
 48243|     ;; slice[8..+8] = i64 %118
 48244|     ;; ptr = ptr %117
 48245|     ;; self = ptr %117
 48246|  %119 = shl nuw nsw i64 %118, 3                                                                                        ;L961<100<1042<666
 48247|  %120 = gep %117, i64 %119                                                                                             ;L961<100<1042<666
 48249|     ;; self[0..+8] = ptr %117
 48250|     ;; self[8..+8] = ptr %120
 48251|     ;; f = ptr %3
 48252|     ;; self = ptr %14
 48253|     ;; self = ptr %13
 48257|     ;; self[0..+8] = ptr %117
 48258|     ;; self[8..+8] = ptr %120
 48259|     ;; f = ptr %3
 48260|  %121 = gep %13, i64 8                                                                                                 ;L69<836<3387<666
 48261|  store ptr %120, ptr %121, , !!53960                                                                                   ;L69<836<3387<666
 48262|  %122 = gep %13, i64 16                                                                                                ;L69<836<3387<666
 48263|  store ptr %3, ptr %122, , !!53960                                                                                     ;L69<836<3387<666
 48266|     ;; self = ptr %13
 48268|     ;; first = ptr %12
 48270|     ;; self = ptr %13
 48271|     ;; self = ptr %13
 48272|     ;; count = i64 1
 48273|     ;; ptr = ptr %117
 48274|     ;; self = ptr %117
 48275|     ;; end_or_len = ptr %120
 48278|  %123 = icmp eq i64 %118, 0                                                                                            ;L1714<180<107<2706<3416<3387<666
 48279|  br i1 %123, label %124, label %125                                                                                    ;L180<107<2706<3416<3387<666
 48280| 
 48281| 124: ; preds = %116
 48285|     ;; self = ptr null
 48286|  br label %159                                                                                                         ;L1161<666
 48287| 
 48288| 125: ; preds = %116
 48289|  %126 = gep %117, i64 8                                                                                                ;L656<185<107<2706<3416<3387<666
 48290|  store ptr %126, ptr %13, , !!54013                                                                                    ;L185<107<2706<3416<3387<666
 48291|     ;; self = ptr %117
 48292|     ;; f = ptr %13
 48293|     ;; self = ptr %13
 48294|     ;; x = ptr %117
 48295|     ;; args = ptr %117
 48297|     ;; x = ptr %117
 48301|  %127 = load ptr, ptr %117, , !!54062, !!8, !!8                                                                        ;L666<3379<310<1162<107<2706<3416<3387<666
 48302|     ;; self = ptr %127
 48303|     ;; other = ptr %3
 48304|  %128 = gep %127, i64 1632                                                                                             ;L2158<666<3379<310<1162<107<2706<3416<3387<666
 48305|  %129 = load i64, ptr %128, , !!54071, !!8                                                                             ;L2158<666<3379<310<1162<107<2706<3416<3387<666
 48306|     ;; x1 = i64 %129
 48307|     ;; self = i64 %129
 48308|  %130 = gep %127, i64 1640                                                                                             ;L2158<666<3379<310<1162<107<2706<3416<3387<666
 48309|  %131 = load i64, ptr %130, , !!54071, !!8                                                                             ;L2158<666<3379<310<1162<107<2706<3416<3387<666
 48310|     ;; y1 = i64 %131
 48311|     ;; self = i64 %131
 48312|  %132 = gep %3, i64 1632                                                                                               ;L2158<666<3379<310<1162<107<2706<3416<3387<666
 48313|  %133 = load i64, ptr %132, , !!54071, !!8                                                                             ;L2158<666<3379<310<1162<107<2706<3416<3387<666
 48314|     ;; x2 = i64 %133
 48315|     ;; other = i64 %133
 48316|  %134 = gep %3, i64 1640                                                                                               ;L2158<666<3379<310<1162<107<2706<3416<3387<666
 48317|  %135 = load i64, ptr %134, , !!54071, !!8                                                                             ;L2158<666<3379<310<1162<107<2706<3416<3387<666
 48318|     ;; y2 = i64 %135
 48319|     ;; other = i64 %135
 48320|  %136 = icmp ult i64 %129, %133                                                                                        ;L3147<7<2158<666<3379<310<1162<107<2706<3416<3387<666
 48321|  %137 = sub nuw i64 %133, %129                                                                                         ;L3147<7<2158<666<3379<310<1162<107<2706<3416<3387<666
 48322|  %138 = sub nuw i64 %129, %133                                                                                         ;L3147<7<2158<666<3379<310<1162<107<2706<3416<3387<666
 48323|  %139 = select i1 %136, i64 %137, i64 %138                                                                             ;L3147<7<2158<666<3379<310<1162<107<2706<3416<3387<666
 48324|     ;; dx = i64 %139
 48325|  %140 = icmp ult i64 %131, %135                                                                                        ;L3147<8<2158<666<3379<310<1162<107<2706<3416<3387<666
 48326|  %141 = sub nuw i64 %135, %131                                                                                         ;L3147<8<2158<666<3379<310<1162<107<2706<3416<3387<666
 48327|  %142 = sub nuw i64 %131, %135                                                                                         ;L3147<8<2158<666<3379<310<1162<107<2706<3416<3387<666
 48328|  %143 = select i1 %140, i64 %141, i64 %142                                                                             ;L3147<8<2158<666<3379<310<1162<107<2706<3416<3387<666
 48329|     ;; dy = i64 %143
 48330|  %144 = mul i64 %139, %139                                                                                             ;L9<2158<666<3379<310<1162<107<2706<3416<3387<666
 48331|  %145 = mul i64 %143, %143                                                                                             ;L9<2158<666<3379<310<1162<107<2706<3416<3387<666
 48332|  %146 = add i64 %145, %144                                                                                             ;L9<2158<666<3379<310<1162<107<2706<3416<3387<666
 48333|  %147 = gep %127, i64 1472                                                                                             ;L666<3379<310<1162<107<2706<3416<3387<666
 48334|  %148 = load i64, ptr %147, , !!54071, !!8                                                                             ;L666<3379<310<1162<107<2706<3416<3387<666
 48335|  store i64 %146, ptr %12, , !!54097                                                                                    ;L2706<3416<3387<666
 48336|  %149 = gep %12, i64 8                                                                                                 ;L2706<3416<3387<666
 48337|  store i64 %148, ptr %149, , !!54097                                                                                   ;L2706<3416<3387<666
 48338|  %150 = gep %12, i64 16                                                                                                ;L2706<3416<3387<666
 48339|  store ptr %117, ptr %150, , !!54097                                                                                   ;L2706<3416<3387<666
 48340|  invoke void @core::iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityENCINvNvNtNtNtBa_6traits8iterator8Iterator10min_by_key3keyRB1n_TyjENCNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model26resolve_fight_stake_rosters0_0E0EB2q_4foldTB3j_B3e_ENCINvNvB2q_6min_by4foldB55_INvB2o_7compareB3e_B3j_EE0EB3x_(ptr sret([24 x i8]) %14, ptr %13, ptr %12)
 48341|  to label %151 unwind label %95                                                                                        ;L2707<3416<3387<666
 48342| 
 48343| 151: ; preds = %125
 48344|  %152 = gep %14, i64 16
 48345|  %153 = load ptr, ptr %152, , !!53896                                                                                  ;L2775<3387<666
 48349|     ;; self = ptr %153
 48350|  %154 = icmp eq ptr %153, null                                                                                         ;L1161<666
 48351|  br i1 %154, label %159, label %155                                                                                    ;L1161<666
 48352| 
 48353| 155: ; preds = %151
 48354|     ;; x = ptr %153
 48355|     ;; a = ptr %153
 48356|  %156 = load ptr, ptr %153, , !!8, !!8                                                                                 ;L666<1162<666
 48357|  %157 = gep %156, i64 1472                                                                                             ;L666<1162<666
 48358|  %158 = load i64, ptr %157, , !!8                                                                                      ;L666<1162<666
 48359|  br label %159                                                                                                         ;L1165<666
 48360| 
 48361| 159: ; preds = %155, %151, %124
 48362|  %160 = phi i64 [ 1, %155 ], [ 0, %151 ], [ 0, %124 ]                                                                  ;L0<666
 48363|  %161 = phi i64 [ %158, %155 ], [ undef, %151 ], [ undef, %124 ]                                                       ;L0<666
 48364|  %162 = gep %15, i64 32                                                                                                ;L666
 48365|  store i64 %160, ptr %162,                                                                                             ;L666
 48366|  %163 = gep %15, i64 40                                                                                                ;L666
 48367|  store i64 %161, ptr %163,                                                                                             ;L666
 48368|  br label %111                                                                                                         ;L665
 48369| 
 48370| 164: ; preds = %115
 48373|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %19)
 48374|  to label %168 unwind label %165                                                                                       ;L825<669
 48375| 
 48376| 165: ; preds = %164
 48377|  %166 = cleanuppad within none []
 48379|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %19) [ "funclet"(token %166) ]
 48380|  to label %167 unwind label %89                                                                                        ;L825<825<669
 48381| 
 48382| 167: ; preds = %165
 48383|  cleanupret from %166 unwind label %89
 48384| 
 48385| 168: ; preds = %164
 48387|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %19)
 48388|  to label %169 unwind label %89                                                                                        ;L825<825<669
 48389| 
 48390| 169: ; preds = %168
 48393|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %20)
 48394|  to label %94 unwind label %170                                                                                        ;L825<669
 48395| 
 48396| 170: ; preds = %169
 48397|  %171 = cleanuppad within none []
 48399|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %20) [ "funclet"(token %171) ] ;L825<825<669
 48400|  cleanupret from %171 unwind to caller                                                                                 ;L825<669
 48401| }
