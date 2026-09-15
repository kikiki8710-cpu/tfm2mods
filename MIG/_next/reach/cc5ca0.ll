 40152| define void @ai::plan_legacy8sub_plan6recallNtB2_13RecallSubPlan17action_candidates(ptr sret([32 x i8]) %0, ptr readnone %1, i64 %2, ptr %3, ptr %4, ptr %5, ptr %6) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
 40153|  %8 = alloca [32 x i8],
 40154|  %9 = alloca [24 x i8],
 40155|  %10 = alloca [32 x i8],
 40156|  %11 = alloca [136 x i8],
 40157|  %12 = alloca [184 x i8],
 40158|  %13 = alloca [32 x i8],
 40159|     ;; self = ptr %1
 40160|     ;; version = i64 %2
 40161|     ;; rnd = ptr %3
 40162|     ;; player = ptr %4
 40163|     ;; data = ptr %5
 40164|     ;; _parameter = ptr %6
 40165|     ;; res = ptr %13
 40166|     ;; battle = ptr %10
 40168|  %14 = gep %5, i64 8                                                                                                   ;L11
 40169|  %15 = load ptr, ptr %14, , !!8, !!8                                                                                   ;L11
 40170|  %16 = load ptr, ptr %15, , !!8, !!8                                                                                   ;L11
 40171|     ;; bump = ptr %16
 40172|  store ptr inttoptr (i64 8 to ptr), ptr %13,                                                                           ;L547<11
 40173|  %17 = gep %13, i64 8                                                                                                  ;L547<11
 40174|  store ptr %16, ptr %17,                                                                                               ;L547<11
 40175|  %18 = gep %13, i64 16                                                                                                 ;L547<11
 40176|  %19 = gep %13, i64 24                                                                                                 ;L547<11
 40177|  call void @llvm.memset.p0.i64(ptr %18, i8 0, i64 16, i1 false)                                                        ;L547<11
 40180|  invoke void @ai::small_action12move_actionsNtB5_17SmallActionRecall3new(ptr sret([136 x i8]) %11, ptr %5, ptr %4, i64 5)
 40181|  to label %22 unwind label %20                                                                                         ;L12
 40182| 
 40183| 20: ; preds = %68, %67, %26, %24, %7
 40184|  %21 = cleanuppad within none []
 40185|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %13) #30 [ "funclet"(token %21) ] ;L28
 40186|  cleanupret from %21 unwind to caller                                                                                  ;L10
 40187| 
 40188| 22: ; preds = %7
 40189|  call void @llvm.memcpy.p0.p0.i64(ptr %12, ptr %11, i64 136, i1 false)                                                 ;L12
 40190|  %23 = gep %12, i64 177                                                                                                ;L12
 40191|  store i8 4, ptr %23,                                                                                                  ;L12
 40194|     ;; self = ptr %13
 40195|     ;; self = ptr %13
 40196|     ;; value = ptr %12
 40197|     ;; src = ptr %12
 40198|     ;; additional = i64 1
 40199|     ;; needed_extra_cap = i64 1
 40200|     ;; needed_extra_cap = i64 1
 40201|     ;; strategy = i8 1
 40202|     ;; self = ptr %13
 40203|     ;; self = ptr %13
 40204|     ;; used_cap = i64 0
 40205|     ;; used_cap = i64 0
 40206|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %13, i64 0, i64 1, i1 zeroext true)
 40207|  to label %26 unwind label %24, !!46749                                                                                ;L619<430<738<1429<12
 40208| 
 40209| 24: ; preds = %22
 40210|  %25 = cleanuppad within none []
 40211|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %12) #30 [ "funclet"(token %25) ], !!46734 ;L1436<12
 40212|  cleanupret from %25 unwind label %20
 40213| 
 40214| 26: ; preds = %22
 40215|  %27 = load ptr, ptr %13, , !!46749                                                                                    ;L138<1432<12
 40216|  %28 = load i64, ptr %19, , !!46749                                                                                    ;L1432<12
 40217|     ;; self = ptr %13
 40218|     ;; self = ptr %27
 40219|     ;; count = i64 %28
 40220|  %29 = gepS %27, i64 %28                                                                                               ;L961<1432<12
 40221|     ;; end = ptr %29
 40222|     ;; dst = ptr %29
 40223|  call void @llvm.memcpy.p0.p0.i64(ptr %29, ptr %12, i64 184, i1 false), !!46734                                        ;L1933<1433<12
 40224|  %30 = add i64 %28, 1                                                                                                  ;L1434<12
 40225|  store i64 %30, ptr %19, , !!46749                                                                                     ;L1434<12
 40228|  invoke void @ai::fight_check13battle_action(ptr sret([32 x i8]) %10, i64 %2, ptr %3, ptr %4, ptr %5, i64 5)
 40229|  to label %31 unwind label %20                                                                                         ;L13
 40230| 
 40231| 31: ; preds = %26
 40232|  %32 = gep %4, i64 2352                                                                                                ;L18
 40233|  %33 = load i64, ptr %32, , !!8                                                                                        ;L18
 40234|  %34 = icmp ult i64 %33, 2                                                                                             ;L18
 40235|  br i1 %34, label %40, label %35                                                                                       ;L18
 40236| 
 40237| 35: ; preds = %31
 40238|  invoke void @core::panicking18panic_bounds_check(i64 %33, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.196) #31
 40239|  to label %39 unwind label %36                                                                                         ;L18
 40240| 
 40241| 36: ; preds = %62, %61, %57, %56, %50, %35
 40242|  %37 = phi i1 [ false, %62 ], [ false, %61 ], [ false, %57 ], [ true, %50 ], [ true, %56 ], [ true, %35 ]              ;L0
 40243|  %38 = cleanuppad within none []
 40244|  br i1 %37, label %68, label %67                                                                                       ;L28
 40245| 
 40246| 39: ; preds = %56, %35
 40247|  unreachable
 40248| 
 40249| 40: ; preds = %31
 40250|     ;; self = ptr %4
 40251|  %41 = gep %4, i64 2496                                                                                                ;L581<18
 40252|  %42 = load i32, ptr %41, , !!8                                                                                        ;L581<18
 40253|  %43 = zext nneg i32 %42 to i64                                                                                        ;L581<18
 40254|  %44 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L18
 40255|  %45 = gep %44, i64 480                                                                                                ;L18
 40256|  %46 = getelementptr [5 x ptr], ptr %45, i64 %33                                                                       ;L18
 40257|  %47 = getelementptr ptr, ptr %46, i64 %43                                                                             ;L18
 40258|  %48 = load ptr, ptr %47, , !!8                                                                                        ;L18
 40259|     ;; self = ptr %48
 40260|  %49 = icmp eq ptr %48, null                                                                                           ;L1011<18
 40261|  br i1 %49, label %56, label %50                                                                                       ;L1011<18
 40262| 
 40263| 50: ; preds = %40
 40264|     ;; champ = ptr %48
 40266|  %51 = load ptr, ptr %44, , !!8, !!8                                                                                   ;L19
 40267|  %52 = gep %44, i64 8                                                                                                  ;L19
 40268|  %53 = load ptr, ptr %52, , !!8, !!8                                                                                   ;L19
 40269|  store ptr %51, ptr %9,                                                                                                ;L19
 40270|  %54 = gep %9, i64 8                                                                                                   ;L19
 40271|  store ptr %53, ptr %54,                                                                                               ;L19
 40272|  %55 = gep %9, i64 16                                                                                                  ;L19
 40273|  store ptr %48, ptr %55,                                                                                               ;L19
 40274|  invoke void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE6retainNCNvMNtNtNtBY_11plan_legacy8sub_plan6recallNtB22_13RecallSubPlan17action_candidates0EBY_(ptr %10, ptr %9)
 40275|  to label %57 unwind label %36                                                                                         ;L19
 40276| 
 40277| 56: ; preds = %40
 40278|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.197) #31
 40279|  to label %39 unwind label %36                                                                                         ;L1013<18
 40280| 
 40281| 57: ; preds = %50
 40283|  %58 = load ptr, ptr %10, , !!8, !!8                                                                                   ;L25
 40284|  %59 = gep %10, i64 24                                                                                                 ;L25
 40285|  %60 = load i64, ptr %59,                                                                                              ;L25
 40286|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %13, ptr %58, i64 %60)
 40287|  to label %61 unwind label %36                                                                                         ;L25
 40288| 
 40289| 61: ; preds = %57
 40291|  invoke void @ai::fight_check20attack_summon_action(ptr sret([32 x i8]) %8, ptr %4, ptr %5)
 40292|  to label %62 unwind label %36                                                                                         ;L26
 40293| 
 40294| 62: ; preds = %61
 40295|  %63 = load ptr, ptr %8, , !!8, !!8                                                                                    ;L26
 40296|  %64 = gep %8, i64 24                                                                                                  ;L26
 40297|  %65 = load i64, ptr %64, , !!8                                                                                        ;L26
 40298|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %13, ptr %63, i64 %65)
 40299|  to label %66 unwind label %36                                                                                         ;L26
 40300| 
 40301| 66: ; preds = %62
 40303|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %13, i64 32, i1 false)                                                   ;L27
 40306|  ret void                                                                                                              ;L28
 40307| 
 40308| 67: ; preds = %36
 40309|  cleanupret from %38 unwind label %20
 40310| 
 40311| 68: ; preds = %36
 40312|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %10) #30 [ "funclet"(token %38) ] ;L28
 40313|  cleanupret from %38 unwind label %20                                                                                  ;L28
 40314| }
