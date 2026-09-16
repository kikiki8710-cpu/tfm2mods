 23520| define i64 @ai::fight_check12expected_dps(ptr %0, ptr %1, ptr %2) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 23521|  %4 = alloca [24 x i8],
 23522|     ;; context = ptr %0
 23523|     ;; target = ptr %1
 23524|     ;; self = ptr %1
 23525|     ;; enemy = ptr %2
 23526|     ;; _t = ptr %4
 23527|     ;; phase = i64 52
 23528|     ;; order = i8 0
 23530|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 23531|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 23532|     ;; order = i8 0
 23533|  %5 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                      ;L3904<741<176<10
 23534|  %6 = icmp eq i8 %5, 0                                                                                                 ;L176<10
 23535|  br i1 %6, label %12, label %7                                                                                         ;L176<10
 23536| 
 23537| 7: ; preds = %3
 23538|  %8 = tail call { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()                                        ;L179<10
 23539|  %9 = extractvalue { i64, i32 } %8, 0                                                                                  ;L179<10
 23540|  %10 = extractvalue { i64, i32 } %8, 1                                                                                 ;L179<10
 23541|  store i64 52, ptr %4,                                                                                                 ;L179<10
 23542|  %11 = gep %4, i64 8                                                                                                   ;L179<10
 23543|  store i64 %9, ptr %11,                                                                                                ;L179<10
 23544|  br label %12                                                                                                          ;L180<10
 23545| 
 23546| 12: ; preds = %7, %3
 23547|  %13 = phi i32 [ %10, %7 ], [ -1, %3 ]                                                                                 ;L0<10
 23548|  %14 = gep %4, i64 16                                                                                                  ;L0<10
 23549|  store i32 %13, ptr %14,                                                                                               ;L0<10
 23550|     ;; res = i64 0
 23551|  %15 = gep %1, i64 1216                                                                                                ;L13
 23552|  %16 = load i32, ptr %15, , !!8                                                                                        ;L13
 23553|  %17 = icmp eq i32 %16, -1                                                                                             ;L13
 23554|  br i1 %17, label %21, label %18                                                                                       ;L13
 23555| 
 23556| 18: ; preds = %12
 23557|  %19 = gep %1, i64 1168                                                                                                ;L13
 23558|     ;; atk = ptr %19
 23559|  %20 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %19, ptr %0, ptr %1, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.56, ptr %2)
 23560|  to label %28 unwind label %26                                                                                         ;L14
 23561| 
 23562| 21: ; preds = %33, %12
 23563|  %22 = phi i64 [ %34, %33 ], [ 0, %12 ]                                                                                ;L0
 23564|     ;; res = i64 %22
 23565|  %23 = gep %1, i64 1272                                                                                                ;L17
 23566|  %24 = load i32, ptr %23, , !!8                                                                                        ;L17
 23567|  %25 = icmp eq i32 %24, -1                                                                                             ;L17
 23568|  br i1 %25, label %40, label %37                                                                                       ;L17
 23569| 
 23570| 26: ; preds = %84, %76, %59, %58, %50, %37, %35, %28, %18
 23571|  %27 = cleanuppad within none []
 23572|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %4) #32 [ "funclet"(token %27) ] ;L26
 23573|  cleanupret from %27 unwind to caller                                                                                  ;L9
 23574| 
 23575| 28: ; preds = %18
 23576|  %29 = mul i64 %20, 1000                                                                                               ;L14
 23577|  %30 = invoke i64 @gc::simulation6entityNtB5_6Entity15attack_cooltime(ptr %1)
 23578|  to label %31 unwind label %26                                                                                         ;L14
 23579| 
 23580| 31: ; preds = %28
 23581|  %32 = icmp eq i64 %30, 0                                                                                              ;L14
 23582|  br i1 %32, label %35, label %33                                                                                       ;L14
 23583| 
 23584| 33: ; preds = %31
 23585|  %34 = udiv i64 %29, %30                                                                                               ;L14
 23586|     ;; res = i64 %34
 23587|  br label %21                                                                                                          ;L13
 23588| 
 23589| 35: ; preds = %31
 23590|  invoke void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.e5f9a016f1220e5cee86475419aee1e5.165) #30
 23591|  to label %36 unwind label %26                                                                                         ;L14
 23592| 
 23593| 36: ; preds = %84, %58, %35
 23594|  unreachable
 23595| 
 23596| 37: ; preds = %21
 23597|  %38 = gep %1, i64 1224                                                                                                ;L17
 23598|     ;; skill = ptr %38
 23599|  %39 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %38, ptr %0, ptr %1, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.56, ptr %2)
 23600|  to label %50 unwind label %26                                                                                         ;L18
 23601| 
 23602| 40: ; preds = %55, %21
 23603|  %41 = phi i64 [ %57, %55 ], [ %22, %21 ]                                                                              ;L0
 23604|     ;; res = i64 %41
 23605|  %42 = gep %1, i64 1480                                                                                                ;L1693<21
 23606|  %43 = load i64, ptr %42, , !!8                                                                                        ;L1693<21
 23607|  %44 = icmp ugt i64 %43, 2                                                                                             ;L1693<21
 23608|  %45 = gep %1, i64 1280                                                                                                ;L1693<21
 23609|  %46 = select i1 %44, ptr %45, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.58                                           ;L1693<21
 23610|  %47 = gep %46, i64 48                                                                                                 ;L21
 23611|  %48 = load i32, ptr %47, , !!8                                                                                        ;L21
 23612|  %49 = icmp eq i32 %48, -1                                                                                             ;L21
 23613|  br i1 %49, label %61, label %59                                                                                       ;L21
 23614| 
 23615| 50: ; preds = %37
 23616|  %51 = mul i64 %39, 1000                                                                                               ;L18
 23617|  %52 = invoke i64 @gc::simulation6entityNtB5_6Entity14skill_cooltime(ptr %1)
 23618|  to label %53 unwind label %26                                                                                         ;L18
 23619| 
 23620| 53: ; preds = %50
 23621|  %54 = icmp eq i64 %52, 0                                                                                              ;L18
 23622|  br i1 %54, label %58, label %55                                                                                       ;L18
 23623| 
 23624| 55: ; preds = %53
 23625|  %56 = udiv i64 %51, %52                                                                                               ;L18
 23626|  %57 = add i64 %56, %22                                                                                                ;L18
 23627|     ;; res = i64 %57
 23628|  br label %40                                                                                                          ;L17
 23629| 
 23630| 58: ; preds = %53
 23631|  invoke void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.e5f9a016f1220e5cee86475419aee1e5.166) #30
 23632|  to label %36 unwind label %26                                                                                         ;L18
 23633| 
 23634| 59: ; preds = %40
 23635|     ;; ult = ptr %46
 23636|  %60 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %46, ptr %0, ptr %1, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.56, ptr %2)
 23637|  to label %76 unwind label %26                                                                                         ;L22
 23638| 
 23639| 61: ; preds = %81, %40
 23640|  %62 = phi i64 [ %83, %81 ], [ %41, %40 ]                                                                              ;L0
 23641|     ;; res = i64 %62
 23643|  %63 = icmp eq i32 %13, -1                                                                                             ;L825<26
 23644|  br i1 %63, label %75, label %64                                                                                       ;L825<26
 23645| 
 23646| 64: ; preds = %61
 23648|     ;; self = ptr %4
 23649|     ;; order = i8 0
 23650|     ;; order = i8 0
 23651|     ;; val = i64 1
 23652|     ;; order = i8 0
 23653|     ;; val = i64 1
 23654|     ;; order = i8 0
 23655|     ;; self = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 416)
 23656|  %65 = gep %4, i64 8                                                                                                   ;L185<825<825<26
 23657|  %66 = call { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %65)                                 ;L185<825<825<26
 23658|  %67 = extractvalue { i64, i32 } %66, 0                                                                                ;L185<825<825<26
 23659|  %68 = extractvalue { i64, i32 } %66, 1                                                                                ;L185<825<825<26
 23661|  %69 = mul i64 %67, 1000000000                                                                                         ;L632<185<825<825<26
 23662|  %70 = icmp ult i32 %68, 1000000000                                                                                    ;L49<632<185<825<825<26
 23663|  call void @llvm.assume(i1 %70)                                                                                        ;L49<632<185<825<825<26
 23664|  %71 = zext nneg i32 %68 to i64                                                                                        ;L632<185<825<825<26
 23665|  %72 = add i64 %69, %71                                                                                                ;L632<185<825<825<26
 23666|     ;; val = i64 %72
 23667|     ;; val = i64 %72
 23668|     ;; dst = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 416)
 23669|  %73 = atomicrmw add ptr getelementptr (i8, ptr @gc::simulation4prof11PHASE_NANOS, i64 416), i64 %72 monotonic, , !!37130 ;L3937<3162<185<825<825<26
 23670|     ;; self = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 416)
 23671|     ;; dst = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 416)
 23672|  %74 = atomicrmw add ptr getelementptr (i8, ptr @gc::simulation4prof11PHASE_CALLS, i64 416), i64 1 monotonic, , !!37130 ;L3937<3162<186<825<825<26
 23673|  br label %75                                                                                                          ;L825<26
 23674| 
 23675| 75: ; preds = %64, %61
 23677|  ret i64 %62                                                                                                           ;L26
 23678| 
 23679| 76: ; preds = %59
 23680|  %77 = mul i64 %60, 1000                                                                                               ;L22
 23681|  %78 = invoke i64 @gc::simulation6entityNtB5_6Entity15skill2_cooltime(ptr %1)
 23682|  to label %79 unwind label %26                                                                                         ;L22
 23683| 
 23684| 79: ; preds = %76
 23685|  %80 = icmp eq i64 %78, 0                                                                                              ;L22
 23686|  br i1 %80, label %84, label %81                                                                                       ;L22
 23687| 
 23688| 81: ; preds = %79
 23689|  %82 = udiv i64 %77, %78                                                                                               ;L22
 23690|  %83 = add i64 %82, %41                                                                                                ;L22
 23691|     ;; res = i64 %83
 23692|  br label %61                                                                                                          ;L21
 23693| 
 23694| 84: ; preds = %79
 23695|  invoke void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.e5f9a016f1220e5cee86475419aee1e5.167) #30
 23696|  to label %36 unwind label %26                                                                                         ;L22
 23697| }
