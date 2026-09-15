 29736| define void @ai::plan_legacy8sub_plan12line_defenseNtB2_18LineDefenseSubPlan31calculate_score_parameter_value(ptr %0, ptr readnone %1, ptr %2, ptr %3, ptr %4) unnamed_addr #0 {
 29739|     ;; self = ptr %0
 29740|     ;; _rnd = ptr %1
 29741|     ;; player = ptr %2
 29742|     ;; data = ptr %3
 29743|     ;; parameter = ptr %4
 29744|     ;; count = i64 1
 29745|     ;; count = i64 1
 29746|  %6 = load i8, ptr %0, , !!8                                                                                           ;L397
 29747|  %7 = trunc nuw i8 %6 to i1                                                                                            ;L397
 29748|     ;; line_style = i1 %7
 29749|  %8 = gep %2, i64 2352                                                                                                 ;L399
 29750|  %9 = load i64, ptr %8, , !!8                                                                                          ;L399
 29751|  %10 = icmp ult i64 %9, 2                                                                                              ;L399
 29752|  br i1 %10, label %12, label %11                                                                                       ;L399
 29753| 
 29754| 11: ; preds = %5
 29755|  tail call void @core::panicking18panic_bounds_check(i64 %9, i64 2, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.132) #35 ;L399
 29756|  unreachable                                                                                                           ;L399
 29757| 
 29758| 12: ; preds = %5
 29759|     ;; self = ptr %2
 29760|  %13 = gep %2, i64 2496                                                                                                ;L581<399
 29761|  %14 = load i32, ptr %13, , !!8                                                                                        ;L581<399
 29762|  %15 = zext nneg i32 %14 to i64                                                                                        ;L581<399
 29763|  %16 = load ptr, ptr %3, , !!8, !!8                                                                                    ;L399
 29764|  %17 = gep %16, i64 480                                                                                                ;L399
 29765|  %18 = getelementptr [5 x ptr], ptr %17, i64 %9                                                                        ;L399
 29766|  %19 = getelementptr ptr, ptr %18, i64 %15                                                                             ;L399
 29767|  %20 = load ptr, ptr %19, , !!8                                                                                        ;L399
 29768|     ;; self = ptr %20
 29769|  %21 = icmp eq ptr %20, null                                                                                           ;L1011<399
 29770|  br i1 %21, label %26, label %22                                                                                       ;L1011<399
 29771| 
 29772| 22: ; preds = %12
 29773|     ;; champ = ptr %20
 29774|  %23 = gep %20, i64 1576                                                                                               ;L400
 29775|  %24 = load i64, ptr %23, , !!8                                                                                        ;L400
 29776|  %25 = icmp eq i64 %24, 0                                                                                              ;L400
 29777|  br i1 %25, label %46, label %27                                                                                       ;L400
 29778| 
 29779| 26: ; preds = %12
 29780|  tail call void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.133) #35                       ;L1013<399
 29781|  unreachable                                                                                                           ;L1013<399
 29782| 
 29783| 27: ; preds = %22
 29784|  %28 = gep %20, i64 1648                                                                                               ;L400
 29785|  %29 = load i64, ptr %28, , !!8                                                                                        ;L400
 29786|  %30 = mul i64 %29, 100                                                                                                ;L400
 29787|  %31 = udiv i64 %30, %24                                                                                               ;L400
 29788|     ;; hp_ratio = i64 %31
 29789|  %32 = icmp ugt i64 %31, 50                                                                                            ;L401
 29790|  %33 = select i1 %32, i64 50, i64 70                                                                                   ;L401
 29791|  %34 = icmp ult i64 %31, 50                                                                                            ;L401
 29792|  %35 = select i1 %34, i64 50, i64 30                                                                                   ;L401
 29793|  %36 = select i1 %7, i64 %33, i64 %35                                                                                  ;L401
 29794|     ;; value = i64 %36
 29795|  %37 = gep %4, i64 2496                                                                                                ;L418
 29796|  store i64 %36, ptr %37,                                                                                               ;L418
 29797|  %38 = gep %4, i64 2504                                                                                                ;L419
 29798|  store i64 %36, ptr %38,                                                                                               ;L419
 29799|     ;; self = ptr %4
 29800|     ;; self = ptr %4
 29801|  %39 = gep %4, i64 5304                                                                                                ;L138<2083<421
 29802|  %40 = load ptr, ptr %39, , !!8, !!8                                                                                   ;L138<2083<421
 29803|     ;; ptr = ptr %40
 29804|  %41 = gep %4, i64 5328                                                                                                ;L2085<421
 29805|  %42 = load i64, ptr %41, , !!8                                                                                        ;L2085<421
 29806|     ;; len = i64 %42
 29807|     ;; count = i64 %42
 29808|     ;; self[0..+8] = ptr %40
 29809|     ;; slice[0..+8] = ptr %40
 29810|     ;; self[8..+8] = i64 %42
 29811|     ;; slice[8..+8] = i64 %42
 29812|     ;; ptr = ptr %40
 29813|     ;; self = ptr %40
 29814|  %43 = mul nuw nsw i64 %42, 216                                                                                        ;L961<240<1062<421
 29815|  %44 = gep %40, i64 %43                                                                                                ;L961<240<1062<421
 29816|     ;; iter[0..+8] = ptr %40
 29817|     ;; iter[8..+8] = ptr %44
 29818|     ;; self = ptr undef
 29819|     ;; ptr = ptr %40
 29820|     ;; self = ptr %40
 29821|     ;; end_or_len = ptr %44
 29824|  %45 = icmp eq i64 %42, 0                                                                                              ;L1714<180<421
 29825|  br i1 %45, label %53, label %47                                                                                       ;L180<421
 29826| 
 29827| 46: ; preds = %22
 29828|  tail call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.134) #35 ;L400
 29829|  unreachable                                                                                                           ;L400
 29830| 
 29831| 47: ; preds = %47, %27
 29832|  %48 = phi ptr [ %49, %47 ], [ %40, %27 ]
 29833|  %49 = gep %48, i64 216                                                                                                ;L656<185<421
 29834|     ;; iter[0..+8] = ptr %49
 29835|     ;; p = ptr %48
 29836|  %50 = gep %48, i64 168                                                                                                ;L422
 29837|  store i64 50, ptr %50,                                                                                                ;L422
 29838|  %51 = gep %48, i64 176                                                                                                ;L423
 29839|  store i64 50, ptr %51,                                                                                                ;L423
 29840|     ;; self = ptr undef
 29841|     ;; ptr = ptr %49
 29842|     ;; self = ptr %49
 29843|     ;; end_or_len = ptr %44
 29846|  %52 = icmp eq ptr %49, %44                                                                                            ;L1714<180<421
 29847|  br i1 %52, label %53, label %47                                                                                       ;L180<421
 29848| 
 29849| 53: ; preds = %47, %27
 29850|     ;; self = ptr %4
 29851|     ;; self = ptr %4
 29852|  %54 = gep %4, i64 5336                                                                                                ;L138<2083<426
 29853|  %55 = load ptr, ptr %54, , !!8, !!8                                                                                   ;L138<2083<426
 29854|     ;; ptr = ptr %55
 29855|  %56 = gep %4, i64 5360                                                                                                ;L2085<426
 29856|  %57 = load i64, ptr %56, , !!8                                                                                        ;L2085<426
 29857|     ;; len = i64 %57
 29858|     ;; count = i64 %57
 29859|     ;; self[0..+8] = ptr %55
 29860|     ;; slice[0..+8] = ptr %55
 29861|     ;; self[8..+8] = i64 %57
 29862|     ;; slice[8..+8] = i64 %57
 29863|     ;; ptr = ptr %55
 29864|     ;; self = ptr %55
 29865|  %58 = mul nuw nsw i64 %57, 216                                                                                        ;L961<240<1062<426
 29866|  %59 = gep %55, i64 %58                                                                                                ;L961<240<1062<426
 29867|     ;; iter[0..+8] = ptr %55
 29868|     ;; iter[8..+8] = ptr %59
 29869|     ;; self = ptr undef
 29870|     ;; ptr = ptr %55
 29871|     ;; self = ptr %55
 29872|     ;; end_or_len = ptr %59
 29875|  %60 = icmp eq i64 %57, 0                                                                                              ;L1714<180<426
 29876|  br i1 %60, label %69, label %61                                                                                       ;L180<426
 29877| 
 29878| 61: ; preds = %53
 29879|  %62 = select i1 %7, i64 50, i64 100
 29880|  br label %63                                                                                                          ;L180<426
 29881| 
 29882| 63: ; preds = %63, %61
 29883|  %64 = phi ptr [ %55, %61 ], [ %65, %63 ]
 29884|  %65 = gep %64, i64 216                                                                                                ;L656<185<426
 29885|     ;; iter[0..+8] = ptr %65
 29886|     ;; p = ptr %64
 29887|     ;; value = i64 %62
 29888|  %66 = gep %64, i64 168                                                                                                ;L435
 29889|  store i64 %62, ptr %66,                                                                                               ;L435
 29890|  %67 = gep %64, i64 176                                                                                                ;L436
 29891|  store i64 %62, ptr %67,                                                                                               ;L436
 29892|     ;; self = ptr undef
 29893|     ;; ptr = ptr %65
 29894|     ;; self = ptr %65
 29895|     ;; end_or_len = ptr %59
 29898|  %68 = icmp eq ptr %65, %59                                                                                            ;L1714<180<426
 29899|  br i1 %68, label %69, label %63                                                                                       ;L180<426
 29900| 
 29901| 69: ; preds = %63, %53
 29902|  ret void                                                                                                              ;L438
 29903| }
