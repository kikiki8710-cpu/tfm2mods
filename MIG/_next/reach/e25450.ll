 44238| define internal fastcc void @ai::small_action11lane_minionNtB2_29SmallActionLaneMinionPosition14push_candidate(ptr %0, i64 %1, ptr %2, ptr %3, ptr %4, i64 %5, i64 %6, ptr %7, i64 %8, i64 %9, i8 %10, i64 %11, i64 %12, i64 %13, i8 %14, i64 %15) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
 44239|  %17 = alloca [56 x i8],
 44240|  %18 = alloca [24 x i8],
 44241|     ;; candidates = ptr %0
 44242|     ;; version = i64 %1
 44243|     ;; player = ptr %2
 44244|     ;; data = ptr %3
 44245|     ;; champ = ptr %4
 44247|     ;; positioning_score = ptr %7
 44248|     ;; x = i64 %8
 44249|     ;; y = i64 %9
 44250|     ;; position_eval_purpose = i8 %10
 44251|     ;; attack_range = i64 %11
 44252|     ;; preferred_range = i64 %12
 44253|     ;; target_score = i64 %13
 44254|     ;; line = i8 %14
 44255|     ;; source_bonus = i64 %15
 44256|  %19 = udiv i64 %8, 32000                                                                                              ;L456
 44257|     ;; self = i64 %19
 44258|     ;; min = i64 0
 44259|     ;; max = i64 29
 44260|  %20 = tail call i64 @llvm.umin.i64(i64 %19, i64 29)                                                                   ;L2027<456
 44261|     ;; xi = i64 %20
 44262|  %21 = udiv i64 %9, 32000                                                                                              ;L457
 44263|     ;; self = i64 %21
 44264|     ;; min = i64 0
 44265|     ;; max = i64 29
 44266|  %22 = tail call i64 @llvm.umin.i64(i64 %21, i64 29)                                                                   ;L2027<457
 44267|     ;; yi = i64 %22
 44268|  %23 = gep %3, i64 8                                                                                                   ;L458
 44269|  %24 = load ptr, ptr %23, , !!8, !!8                                                                                   ;L458
 44270|  %25 = gep %24, i64 32                                                                                                 ;L458
 44271|  %26 = load ptr, ptr %25, , !!8, !!8                                                                                   ;L458
 44272|  %27 = gep %26, i64 120                                                                                                ;L458
 44273|  %28 = getelementptr [30 x i64], ptr %27, i64 %22                                                                      ;L458
 44274|  %29 = getelementptr i64, ptr %28, i64 %20                                                                             ;L458
 44275|  %30 = load i64, ptr %29, , !!8                                                                                        ;L458
 44276|  %31 = icmp eq i64 %30, 0                                                                                              ;L458
 44277|  br i1 %31, label %32, label %123                                                                                      ;L458
 44278| 
 44279| 32: ; preds = %16
 44280|  %33 = tail call zeroext i1 @ai::path_finder20is_enemy_well_danger(i64 %1, ptr %2, i64 %8, i64 %9)                     ;L458
 44281|  br i1 %33, label %123, label %34                                                                                      ;L458
 44282| 
 44283| 34: ; preds = %32
 44284|  %35 = tail call zeroext i1 @gc::simulation11map_regions12is_near_line(ptr %24, i64 %8, i64 %9, i8 %14)                ;L461
 44285|  br i1 %35, label %36, label %123                                                                                      ;L461
 44286| 
 44287| 36: ; preds = %34
 44288|  %37 = tail call zeroext i1 @ai::path_finder35is_unnecessary_enemy_tower_position(i64 %1, ptr %2, ptr %3, i64 %8, i64 %9) ;L464
 44289|  br i1 %37, label %123, label %38                                                                                      ;L464
 44290| 
 44291| 38: ; preds = %36
 44292|  %39 = tail call i64 @gc::utils8distance(i64 %8, i64 %9, i64 %5, i64 %6)                                               ;L468
 44293|     ;; dist_to_target = i64 %39
 44294|  %40 = icmp ugt i64 %39, %11                                                                                           ;L469
 44295|  br i1 %40, label %123, label %41                                                                                      ;L469
 44296| 
 44297| 41: ; preds = %38
 44298|  %42 = icmp ugt i64 %11, 69999                                                                                         ;L473
 44299|  br i1 %42, label %43, label %50                                                                                       ;L473
 44300| 
 44301| 43: ; preds = %41
 44302|  %44 = mul i64 %11, 55                                                                                                 ;L473
 44303|  %45 = udiv i64 %44, 100                                                                                               ;L473
 44304|     ;; min_spacing = i64 %45
 44305|  %46 = icmp ult i64 %39, %45                                                                                           ;L474
 44306|  br i1 %46, label %47, label %50                                                                                       ;L474
 44307| 
 44308| 47: ; preds = %43
 44309|  %48 = sub nuw nsw i64 %45, %39                                                                                        ;L475
 44310|  %49 = udiv i64 %48, 1000                                                                                              ;L475
 44311|     ;; close_penalty = i64 %49
 44312|  br label %50                                                                                                          ;L474
 44313| 
 44314| 50: ; preds = %47, %43, %41
 44315|  %51 = phi i64 [ %49, %47 ], [ 0, %43 ], [ 0, %41 ]                                                                    ;L0
 44316|     ;; close_penalty = i64 %51
 44317|  %52 = sub i64 %39, %12                                                                                                ;L480
 44318|     ;; self = i64 %52
 44319|  %53 = tail call i64 @llvm.abs.i64(i64 %52, i1 false)                                                                  ;L3648<480
 44320|     ;; band_penalty = i64 %53
 44322|     ;; version = i64 %1
 44323|     ;; player = ptr %2
 44324|     ;; data = ptr %3
 44325|     ;; positioning_score = ptr %7
 44326|     ;; x = i64 %8
 44327|     ;; y = i64 %9
 44328|     ;; position_eval_purpose = i8 %10
 44329|     ;; score = ptr %17
 44330|     ;; self = i64 %19
 44331|     ;; min = i64 0
 44332|     ;; max = i64 29
 44333|  %54 = trunc nuw nsw i64 %20 to i32                                                                                    ;L361<481
 44334|     ;; xi = i32 %54
 44335|     ;; self = i64 %21
 44336|     ;; min = i64 0
 44337|     ;; max = i64 29
 44338|  %55 = trunc nuw nsw i64 %22 to i32                                                                                    ;L362<481
 44339|     ;; yi = i32 %55
 44340|  %56 = gep %7, i64 2744                                                                                                ;L363<481
 44341|  %57 = load i64, ptr %56, , !!66638, !!8                                                                               ;L363<481
 44342|  %58 = trunc i64 %57 to i32                                                                                            ;L363<481
 44344|  %59 = gep %7, i64 2752                                                                                                ;L364<481
 44345|  %60 = load i64, ptr %59, , !!66638, !!8                                                                               ;L364<481
 44346|  %61 = trunc i64 %60 to i32                                                                                            ;L364<481
 44348|  %62 = add nsw i32 %54, -4                                                                                             ;L363<481
 44349|  %63 = sub i32 %62, %58                                                                                                ;L365<481
 44350|  %64 = icmp ult i32 %63, -7                                                                                            ;L365<481
 44351|  %65 = add nsw i32 %55, -4                                                                                             ;L364<481
 44352|  %66 = sub i32 %65, %61                                                                                                ;L365<481
 44353|  %67 = icmp ult i32 %66, -7                                                                                            ;L365<481
 44354|  %68 = or i1 %64, %67                                                                                                  ;L365<481
 44355|  br i1 %68, label %100, label %69                                                                                      ;L365<481
 44356| 
 44357| 69: ; preds = %50
 44359|     ;; version = i64 %1
 44360|     ;; player = ptr %2
 44361|     ;; data = ptr %3
 44362|     ;; positioning_score = ptr %7
 44363|     ;; x = i64 %8
 44364|     ;; y = i64 %9
 44365|     ;; purpose = i8 %10
 44366|  call void @ai::position_eval26position_score_at_position(ptr sret([56 x i8]) %17, i64 %1, ptr %2, ptr %3, ptr %7, i64 %8, i64 %9, i8 %10) ;L101<369<481
 44367|  %70 = gep %17, i64 48                                                                                                 ;L370<481
 44368|  %71 = load i8, ptr %70, , !!66646, !!8                                                                                ;L370<481
 44369|  %72 = trunc nuw i8 %71 to i1                                                                                          ;L370<481
 44370|  br i1 %72, label %79, label %73                                                                                       ;L370<481
 44371| 
 44372| 73: ; preds = %69
 44373|  %74 = gep %17, i64 49                                                                                                 ;L370<481
 44374|  %75 = load i8, ptr %74, , !!66646, !!8                                                                                ;L370<481
 44375|  %76 = trunc nuw i8 %75 to i1                                                                                          ;L370<481
 44376|     ;; on_trajectory = i1 %76
 44377|  %77 = gep %2, i64 384                                                                                                 ;L371<481
 44378|  %78 = call i64 @gc::simulation5state6playerNtB4_16AthleteParameter21positioning_effective(ptr %77)                    ;L371<481
 44379|     ;; positioning = i64 %78
 44380|  br i1 %76, label %82, label %90                                                                                       ;L372<481
 44381| 
 44382| 79: ; preds = %69
 44383|     ;; on_trajectory = i8 1
 44384|  %80 = gep %2, i64 384                                                                                                 ;L371<481
 44385|  %81 = call i64 @gc::simulation5state6playerNtB4_16AthleteParameter21positioning_effective(ptr %80)                    ;L371<481
 44386|     ;; positioning = i64 %81
 44387|  br label %82                                                                                                          ;L372<481
 44388| 
 44389| 82: ; preds = %79, %73
 44390|  %83 = phi i64 [ %81, %79 ], [ %78, %73 ]                                                                              ;L371<481
 44391|     ;; positioning = i64 %83
 44392|  %84 = sub i64 50, %83                                                                                                 ;L373<481
 44393|     ;; self = i64 %84
 44394|     ;; other = i64 0
 44395|  %85 = call i64 @llvm.smax.i64(i64 %84, i64 0)                                                                         ;L1039<373<481
 44396|  %86 = mul i64 %85, -3                                                                                                 ;L373<481
 44397|  %87 = add i64 %86, 220                                                                                                ;L373<481
 44398|     ;; self = i64 %87
 44399|     ;; min = i64 70
 44400|     ;; max = i64 220
 44401|  %88 = call i64 @llvm.smax.i64(i64 %87, i64 70)                                                                        ;L2025<373<481
 44402|  %89 = call i64 @llvm.umin.i64(i64 %88, i64 220)                                                                       ;L2025<373<481
 44403|     ;; trajectory_penalty = i64 %89
 44404|  br label %90                                                                                                          ;L372<481
 44405| 
 44406| 90: ; preds = %82, %73
 44407|  %91 = phi i64 [ %89, %82 ], [ 0, %73 ]                                                                                ;L0<481
 44408|     ;; trajectory_penalty = i64 %91
 44409|  %92 = gep %17, i64 16                                                                                                 ;L377<481
 44410|  %93 = load i64, ptr %92, , !!66646, !!8                                                                               ;L377<481
 44411|  %94 = load i64, ptr %17, , !!66646, !!8                                                                               ;L377<481
 44412|     ;; _version = i64 %1
 44413|     ;; player = ptr %2
 44414|     ;; risk = i64 %94
 44415|     ;; on_trajectory = i1 false
 44416|  %95 = gep %2, i64 384                                                                                                 ;L81<377<481
 44417|  %96 = call i64 @gc::simulation5state6playerNtB4_16AthleteParameter21skill_avoid_effective(ptr %95)                    ;L81<377<481
 44418|  %97 = shl i64 %94, 1                                                                                                  ;L377<481
 44419|  %98 = add i64 %91, %97                                                                                                ;L377<481
 44420|  %99 = sub i64 %93, %98                                                                                                ;L377<481
 44422|  br label %100                                                                                                         ;L378<481
 44423| 
 44424| 100: ; preds = %90, %50
 44425|  %101 = phi i64 [ %99, %90 ], [ 0, %50 ]                                                                               ;L0<481
 44426|     ;; local_score = i64 %101
 44427|  %102 = gep %4, i64 1632                                                                                               ;L482
 44428|  %103 = load i64, ptr %102, , !!8                                                                                      ;L482
 44429|  %104 = gep %4, i64 1640                                                                                               ;L482
 44430|  %105 = load i64, ptr %104, , !!8                                                                                      ;L482
 44431|  %106 = call i64 @gc::utils8distance(i64 %103, i64 %105, i64 %8, i64 %9)                                               ;L482
 44432|     ;; move_penalty = i64 %106
 44433|  %107 = call { i64, i64 } @ai::small_action11lane_minionNtB2_29SmallActionLaneMinionPosition16lane_stance_risk(i64 poison, ptr %2, ptr %3, ptr %4, i64 %8, i64 %9) ;L483
 44434|  %108 = extractvalue { i64, i64 } %107, 0                                                                              ;L483
 44435|  %109 = trunc nuw i64 %108 to i1                                                                                       ;L483
 44436|  br i1 %109, label %110, label %123                                                                                    ;L483
 44437| 
 44438| 110: ; preds = %100
 44439|  %111 = extractvalue { i64, i64 } %107, 1                                                                              ;L483
 44440|  %112 = sdiv i64 %106, -5000                                                                                           ;L482
 44441|     ;; move_penalty = i64 %106
 44442|  %113 = sdiv i64 %53, -3000                                                                                            ;L480
 44443|     ;; band_penalty = i64 %53
 44444|     ;; risk = i64 %111
 44445|  %114 = add i64 %13, 80                                                                                                ;L489
 44446|  %115 = add i64 %114, %15                                                                                              ;L489
 44447|  %116 = add i64 %115, %113                                                                                             ;L489
 44448|  %117 = sub i64 %116, %51                                                                                              ;L489
 44449|  %118 = add i64 %117, %101                                                                                             ;L489
 44450|  %119 = add i64 %118, %112                                                                                             ;L489
 44451|  %120 = sub i64 %119, %111                                                                                             ;L489
 44452|     ;; score = i64 %120
 44454|  store i64 %8, ptr %18,                                                                                                ;L490
 44455|  %121 = gep %18, i64 8                                                                                                 ;L490
 44456|  store i64 %9, ptr %121,                                                                                               ;L490
 44457|  %122 = gep %18, i64 16                                                                                                ;L490
 44458|  store i64 %120, ptr %122,                                                                                             ;L490
 44459|  call fastcc void @_RNvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB2_3VecTyyxEE4pushCshdEBA0ozCnw_7game_ai(ptr %0, ptr %18) ;L490
 44461|  br label %123                                                                                                         ;L491
 44462| 
 44463| 123: ; preds = %110, %100, %38, %36, %34, %32, %16
 44464|  ret void                                                                                                              ;L491
 44465| }
