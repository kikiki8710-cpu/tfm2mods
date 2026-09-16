 25960| define void @ai::plan_legacy9team_planNtB5_8TeamPlan22can_near_enemies_range(ptr sret([32 x i8]) %0, ptr %1, i64 %2, ptr %3, ptr %4, ptr %5, i64 %6, i64 %7, i64 %8, ptr readnone %9) unnamed_addr #0 {
 25961|  %11 = alloca [152 x i8],
 25962|  %12 = alloca [8 x i8],
 25963|  %13 = alloca [8 x i8],
 25964|  %14 = alloca [8 x i8],
 25965|  %15 = alloca [8 x i8],
 25966|  %16 = alloca [1 x i8],
 25967|  %17 = alloca [8 x i8],
 25968|  %18 = alloca [8 x i8],
 25969|  %19 = alloca [8 x i8],
 25970|  %20 = alloca [8 x i8],
 25971|  %21 = alloca [8 x i8],
 25972|  store i64 %6, ptr %21,
 25973|  store i64 %7, ptr %20,
 25974|  store i64 %8, ptr %19,
 25975|     ;; self = ptr %1
 25977|     ;; rnd = ptr %3
 25978|     ;; player = ptr %4
 25979|     ;; data = ptr %5
 25980|     ;; x = ptr %21
 25981|     ;; y = ptr %20
 25982|     ;; d = ptr %19
 25984|     ;; range_min = ptr %18
 25985|     ;; range_max = ptr %17
 25986|     ;; is_dm = ptr %16
 25987|     ;; judgement_base = ptr %15
 25988|     ;; game_seed = ptr %14
 25989|     ;; tick = ptr %13
 25990|     ;; tps = ptr %12
 25991|  %22 = gep %4, i64 384                                                                                                 ;L487
 25992|  %23 = tail call i64 @gc::simulation5state6playerNtB4_16AthleteParameter14judge_accuracy(ptr %22)                      ;L487
 25993|     ;; judge_accuracy = i64 %23
 25995|  %24 = sub i64 1000, %23                                                                                               ;L488
 25996|  %25 = lshr i64 %24, 1                                                                                                 ;L488
 25997|  %26 = sub nsw i64 1000, %25                                                                                           ;L488
 25998|  store i64 %26, ptr %18,                                                                                               ;L488
 26000|  %27 = add nuw i64 %25, 1000                                                                                           ;L489
 26001|  store i64 %27, ptr %17,                                                                                               ;L489
 26003|  %28 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L494
 26004|  %29 = load ptr, ptr %28, , !!8, !!8                                                                                   ;L494
 26005|  %30 = gep %28, i64 8                                                                                                  ;L494
 26006|  %31 = load ptr, ptr %30, , !!8, !!8                                                                                   ;L494
 26007|  %32 = gep %31, i64 64                                                                                                 ;L494
 26008|  %33 = load ptr, ptr %32, , !!8                                                                                        ;L494
 26009|  %34 = tail call { i64, ptr } %33(ptr %29)                                                                             ;L494
 26010|  %35 = extractvalue { i64, ptr } %34, 0                                                                                ;L494
 26011|  %36 = icmp eq i64 %35, 2                                                                                              ;L494
 26012|  %37 = zext i1 %36 to i8                                                                                               ;L494
 26013|  store i8 %37, ptr %16,                                                                                                ;L494
 26014|  br i1 %36, label %40, label %38                                                                                       ;L496
 26015| 
 26016| 38: ; preds = %10
 26017|  %39 = tail call zeroext i1 @ai::utils22player_awareness_lapse(ptr %4, ptr %5)                                         ;L496
 26018|  br i1 %39, label %75, label %40                                                                                       ;L496
 26019| 
 26020| 40: ; preds = %38, %10
 26022|  %41 = tail call i64 @gc::simulation5state6playerNtB4_16AthleteParameter14judgement_base(ptr %22)                      ;L499
 26023|  store i64 %41, ptr %15,                                                                                               ;L499
 26025|  %42 = gep %31, i64 32                                                                                                 ;L500
 26026|  %43 = load ptr, ptr %42, , !!8                                                                                        ;L500
 26027|  %44 = tail call i64 %43(ptr %29)                                                                                      ;L500
 26028|  store i64 %44, ptr %14,                                                                                               ;L500
 26030|  %45 = gep %31, i64 40                                                                                                 ;L501
 26031|  %46 = load ptr, ptr %45, , !!8                                                                                        ;L501
 26032|  %47 = tail call i64 %46(ptr %29)                                                                                      ;L501
 26033|  store i64 %47, ptr %13,                                                                                               ;L501
 26035|  %48 = gep %5, i64 8                                                                                                   ;L502
 26036|  %49 = load ptr, ptr %48, , !!8, !!8                                                                                   ;L502
 26037|  %50 = gep %49, i64 8                                                                                                  ;L502
 26038|  %51 = load ptr, ptr %50, , !!8, !!8                                                                                   ;L502
 26039|  %52 = gep %51, i64 4856                                                                                               ;L502
 26040|  %53 = load i64, ptr %52, , !!8                                                                                        ;L502
 26041|  store i64 %53, ptr %12,                                                                                               ;L502
 26043|  %54 = gep %5, i64 16                                                                                                  ;L505
 26044|  %55 = load ptr, ptr %54, , !!8, !!8                                                                                   ;L505
 26045|     ;; self[120..+8] = i64 0
 26046|     ;; self[128..+8] = i64 5
 26047|     ;; self[0..+8] = ptr %28
 26048|     ;; self[8..+8] = ptr %55
 26049|     ;; self[16..+8] = ptr %4
 26050|     ;; self[24..+8] = ptr %16
 26051|     ;; self[32..+8] = ptr %13
 26052|     ;; self[40..+8] = ptr %1
 26053|     ;; self[48..+8] = ptr %12
 26054|     ;; self[56..+8] = ptr %21
 26055|     ;; self[64..+8] = ptr %20
 26056|     ;; self[72..+8] = ptr %19
 26057|     ;; self[80..+8] = ptr %15
 26058|     ;; self[88..+8] = ptr %14
 26059|     ;; self[96..+8] = ptr %3
 26060|     ;; self[104..+8] = ptr %18
 26061|     ;; self[112..+8] = ptr %17
 26062|     ;; f[0..+8] = ptr %28
 26063|     ;; f[8..+8] = ptr %4
 26064|  store ptr %28, ptr %11,                                                                                               ;L24<1002<537
 26065|  %56 = gep %11, i64 8                                                                                                  ;L24<1002<537
 26066|  store ptr %55, ptr %56,                                                                                               ;L24<1002<537
 26067|  %57 = gep %11, i64 16                                                                                                 ;L24<1002<537
 26068|  store ptr %4, ptr %57,                                                                                                ;L24<1002<537
 26069|  %58 = gep %11, i64 24                                                                                                 ;L24<1002<537
 26070|  store ptr %16, ptr %58,                                                                                               ;L24<1002<537
 26071|  %59 = gep %11, i64 32                                                                                                 ;L24<1002<537
 26072|  store ptr %13, ptr %59,                                                                                               ;L24<1002<537
 26073|  %60 = gep %11, i64 40                                                                                                 ;L24<1002<537
 26074|  store ptr %1, ptr %60,                                                                                                ;L24<1002<537
 26075|  %61 = gep %11, i64 48                                                                                                 ;L24<1002<537
 26076|  store ptr %12, ptr %61,                                                                                               ;L24<1002<537
 26077|  %62 = gep %11, i64 56                                                                                                 ;L24<1002<537
 26078|  store ptr %21, ptr %62,                                                                                               ;L24<1002<537
 26079|  %63 = gep %11, i64 64                                                                                                 ;L24<1002<537
 26080|  store ptr %20, ptr %63,                                                                                               ;L24<1002<537
 26081|  %64 = gep %11, i64 72                                                                                                 ;L24<1002<537
 26082|  store ptr %19, ptr %64,                                                                                               ;L24<1002<537
 26083|  %65 = gep %11, i64 80                                                                                                 ;L24<1002<537
 26084|  store ptr %15, ptr %65,                                                                                               ;L24<1002<537
 26085|  %66 = gep %11, i64 88                                                                                                 ;L24<1002<537
 26086|  store ptr %14, ptr %66,                                                                                               ;L24<1002<537
 26087|  %67 = gep %11, i64 96                                                                                                 ;L24<1002<537
 26088|  store ptr %3, ptr %67,                                                                                                ;L24<1002<537
 26089|  %68 = gep %11, i64 104                                                                                                ;L24<1002<537
 26090|  store ptr %18, ptr %68,                                                                                               ;L24<1002<537
 26091|  %69 = gep %11, i64 112                                                                                                ;L24<1002<537
 26092|  store ptr %17, ptr %69,                                                                                               ;L24<1002<537
 26093|  %70 = gep %11, i64 120                                                                                                ;L24<1002<537
 26094|  store i64 0, ptr %70,                                                                                                 ;L24<1002<537
 26095|  %71 = gep %11, i64 128                                                                                                ;L24<1002<537
 26096|  store i64 5, ptr %71,                                                                                                 ;L24<1002<537
 26097|  %72 = gep %11, i64 136                                                                                                ;L24<1002<537
 26098|  store ptr %28, ptr %72,                                                                                               ;L24<1002<537
 26099|  %73 = gep %11, i64 144                                                                                                ;L24<1002<537
 26100|  store ptr %4, ptr %73,                                                                                                ;L24<1002<537
 26101|  %74 = load ptr, ptr %49, , !!8, !!8                                                                                   ;L537
 26102|  call void @core::iter8adapters10filter_map9FilterMapINtNtB29_6filter6FilterINtNtNtB2d_3ops5range5RangejENCNvMs1_NtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_planNtB44_8TeamPlan22can_near_enemies_range0ENCB3Y_s_0EEB48_(ptr sret([32 x i8]) %0, ptr %11, ptr %74) ;L505
 26108|  br label %81                                                                                                          ;L538
 26109| 
 26110| 75: ; preds = %38
 26111|  %76 = gep %5, i64 8                                                                                                   ;L497
 26112|  %77 = load ptr, ptr %76, , !!8, !!8                                                                                   ;L497
 26113|  %78 = load ptr, ptr %77, , !!8, !!8                                                                                   ;L497
 26114|     ;; bump = ptr %78
 26115|  store ptr inttoptr (i64 8 to ptr), ptr %0,                                                                            ;L547<497
 26116|  %79 = gep %0, i64 8                                                                                                   ;L547<497
 26117|  store ptr %78, ptr %79,                                                                                               ;L547<497
 26118|  %80 = gep %0, i64 16                                                                                                  ;L547<497
 26119|  call void @llvm.memset.p0.i64(ptr %80, i8 0, i64 16, i1 false)                                                        ;L547<497
 26120|  br label %81                                                                                                          ;L538
 26121| 
 26122| 81: ; preds = %75, %40
 26126|  ret void                                                                                                              ;L538
 26127| }
