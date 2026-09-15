 20984| define i64 @ai::plan_legacy8sub_plan12attack_nexusNtB2_18AttackNexusSubPlan5score(ptr %0, i64 %1, ptr %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %7) unnamed_addr #0 {
 20985|     ;; self = ptr %0
 20986|     ;; version = i64 %1
 20987|     ;; parameter = ptr %2
 20988|     ;; rnd = ptr %3
 20989|     ;; player = ptr %4
 20990|     ;; data = ptr %5
 20991|     ;; action = ptr %6
 20992|     ;; debug = ptr %7
 20993|     ;; action_type = i8 2
 20994|  %9 = gep %4, i64 2352                                                                                                 ;L159
 20995|  %10 = load i64, ptr %9, , !!8                                                                                         ;L159
 20996|  %11 = icmp ult i64 %10, 2                                                                                             ;L159
 20997|  br i1 %11, label %13, label %12                                                                                       ;L159
 20998| 
 20999| 12: ; preds = %8
 21000|  tail call void @core::panicking18panic_bounds_check(i64 %10, i64 2, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.109) #35 ;L159
 21001|  unreachable                                                                                                           ;L159
 21002| 
 21003| 13: ; preds = %8
 21004|     ;; self = ptr %4
 21005|  %14 = gep %4, i64 2496                                                                                                ;L581<159
 21006|  %15 = load i32, ptr %14, , !!8                                                                                        ;L581<159
 21007|  %16 = zext nneg i32 %15 to i64                                                                                        ;L581<159
 21008|  %17 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L159
 21009|  %18 = gep %17, i64 480                                                                                                ;L159
 21010|  %19 = getelementptr [5 x ptr], ptr %18, i64 %10                                                                       ;L159
 21011|  %20 = getelementptr ptr, ptr %19, i64 %16                                                                             ;L159
 21012|  %21 = load ptr, ptr %20, , !!8                                                                                        ;L159
 21013|     ;; self = ptr %21
 21014|  %22 = icmp eq ptr %21, null                                                                                           ;L1011<159
 21015|  br i1 %22, label %32, label %23                                                                                       ;L1011<159
 21016| 
 21017| 23: ; preds = %13
 21018|     ;; champ = ptr %21
 21019|     ;; self = ptr %21
 21020|     ;; self = ptr %21
 21021|     ;; self = ptr %21
 21022|     ;; self = ptr %21
 21023|  %24 = tail call i64 @ai::action_score17interaction_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %6, ptr %7)      ;L160
 21024|     ;; base = i64 %24
 21025|     ;; self = ptr %6
 21026|  %25 = gep %6, i64 177                                                                                                 ;L309<164
 21027|  %26 = load i8, ptr %25, , !!35249, !!8                                                                                ;L309<164
 21028|  %27 = icmp ne i8 %26, 10                                                                                              ;L309<164
 21029|  tail call void @llvm.assume(i1 %27)                                                                                   ;L309<164
 21030|  %28 = add nsw i8 %26, -3                                                                                              ;L309<164
 21031|  %29 = icmp samesign ugt i8 %26, 2                                                                                     ;L309<164
 21032|  %30 = select i1 %29, i8 %28, i8 7                                                                                     ;L309<164
 21033|  switch i8 %30, label %31 [
 21034|  i8 0, label %72
 21035|  i8 1, label %72
 21036|  i8 2, label %33
 21037|  i8 3, label %33
 21038|  i8 4, label %72
 21039|  i8 5, label %72
 21040|  i8 6, label %72
 21041|  i8 7, label %72
 21042|  i8 8, label %72
 21043|  i8 9, label %72
 21044|  i8 10, label %33
 21045|  i8 11, label %72
 21046|  i8 12, label %42
 21047|  i8 13, label %52
 21048|  i8 14, label %62
 21049|  i8 15, label %72
 21050|  i8 16, label %72
 21051|  ]                                                                                                                     ;L309<164
 21052| 
 21053| 31: ; preds = %23
 21054|  unreachable                                                                                                           ;L309<164
 21055| 
 21056| 32: ; preds = %13
 21057|  tail call void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.110) #35                       ;L1013<159
 21058|  unreachable                                                                                                           ;L1013<159
 21059| 
 21060| 33: ; preds = %23, %23, %23
 21061|  %34 = gep %6, i64 8                                                                                                   ;L0<164
 21062|  %35 = load i64, ptr %34, , !!35249, !!8                                                                               ;L0<164
 21063|     ;; target_id = i64 %35
 21064|  %36 = load ptr, ptr %17, , !!8, !!8                                                                                   ;L201
 21065|  %37 = gep %17, i64 8                                                                                                  ;L201
 21066|  %38 = load ptr, ptr %37, , !!8, !!8                                                                                   ;L201
 21067|  %39 = gep %38, i64 496                                                                                                ;L201
 21068|  %40 = load ptr, ptr %39, , !!8                                                                                        ;L201
 21069|  %41 = tail call ptr %40(ptr %36, i64 %35)                                                                             ;L201
 21070|  br label %72                                                                                                          ;L210
 21071| 
 21072| 42: ; preds = %23
 21073|     ;; action = ptr %6
 21074|     ;; self = ptr %6
 21075|  %43 = gep %6, i64 8                                                                                                   ;L94<322<164
 21076|  %44 = load i64, ptr %43, , !!35249, !!8                                                                               ;L94<322<164
 21077|     ;; target_id = i64 %44
 21078|  %45 = load ptr, ptr %17, , !!8, !!8                                                                                   ;L169
 21079|  %46 = gep %17, i64 8                                                                                                  ;L169
 21080|  %47 = load ptr, ptr %46, , !!8, !!8                                                                                   ;L169
 21081|  %48 = gep %47, i64 496                                                                                                ;L169
 21082|  %49 = load ptr, ptr %48, , !!8                                                                                        ;L169
 21083|  %50 = tail call ptr %49(ptr %45, i64 %44)                                                                             ;L169
 21084|  %51 = icmp eq ptr %50, null                                                                                           ;L169
 21085|  br i1 %51, label %72, label %75                                                                                       ;L169
 21086| 
 21087| 52: ; preds = %23
 21088|     ;; action = ptr %6
 21089|     ;; self = ptr %6
 21090|  %53 = gep %6, i64 8                                                                                                   ;L160<323<164
 21091|  %54 = load i64, ptr %53, , !!35249, !!8                                                                               ;L160<323<164
 21092|     ;; target_id = i64 %54
 21093|  %55 = load ptr, ptr %17, , !!8, !!8                                                                                   ;L177
 21094|  %56 = gep %17, i64 8                                                                                                  ;L177
 21095|  %57 = load ptr, ptr %56, , !!8, !!8                                                                                   ;L177
 21096|  %58 = gep %57, i64 496                                                                                                ;L177
 21097|  %59 = load ptr, ptr %58, , !!8                                                                                        ;L177
 21098|  %60 = tail call ptr %59(ptr %55, i64 %54)                                                                             ;L177
 21099|  %61 = icmp eq ptr %60, null                                                                                           ;L177
 21100|  br i1 %61, label %72, label %85                                                                                       ;L177
 21101| 
 21102| 62: ; preds = %23
 21103|     ;; action = ptr %6
 21104|     ;; self = ptr %6
 21105|  %63 = gep %6, i64 8                                                                                                   ;L222<324<164
 21106|  %64 = load i64, ptr %63, , !!35249, !!8                                                                               ;L222<324<164
 21107|     ;; target_id = i64 %64
 21108|  %65 = load ptr, ptr %17, , !!8, !!8                                                                                   ;L191
 21109|  %66 = gep %17, i64 8                                                                                                  ;L191
 21110|  %67 = load ptr, ptr %66, , !!8, !!8                                                                                   ;L191
 21111|  %68 = gep %67, i64 496                                                                                                ;L191
 21112|  %69 = load ptr, ptr %68, , !!8                                                                                        ;L191
 21113|  %70 = tail call ptr %69(ptr %65, i64 %64)                                                                             ;L191
 21114|  %71 = icmp eq ptr %70, null                                                                                           ;L191
 21115|  br i1 %71, label %72, label %100                                                                                      ;L191
 21116| 
 21117| 72: ; preds = %113, %89, %79, %62, %52, %42, %33, %23, %23, %23, %23, %23, %23, %23, %23, %23, %23, %23
 21118|  %73 = phi i64 [ -99999, %52 ], [ -99999, %62 ], [ 0, %33 ], [ -99999, %42 ], [ %83, %79 ], [ %98, %89 ], [ %118, %113 ], [ 0, %23 ], [ 0, %23 ], [ 0, %23 ], [ 0, %23 ], [ 0, %23 ], [ 0, %23 ], [ 0, %23 ], [ 0, %23 ], [ 0, %23 ], [ 0, %23 ], [ 0, %23 ] ;L0
 21119|  %74 = add i64 %73, %24                                                                                                ;L164
 21120|  ret i64 %74                                                                                                           ;L213
 21121| 
 21122| 75: ; preds = %42
 21123|     ;; t = ptr %50
 21124|     ;; self = ptr %21
 21125|  %76 = gep %21, i64 1216                                                                                               ;L742<170
 21126|  %77 = load i32, ptr %76, , !!8                                                                                        ;L742<170
 21127|  %78 = icmp eq i32 %77, -1                                                                                             ;L742<170
 21128|  br i1 %78, label %84, label %79                                                                                       ;L742<170
 21129| 
 21130| 79: ; preds = %75
 21131|  %80 = gep %21, i64 1168                                                                                               ;L742<170
 21132|     ;; self = ptr %80
 21133|     ;; effect = ptr %80
 21134|  %81 = gep %21, i64 1392                                                                                               ;L1494<171
 21135|  %82 = tail call i64 @gc::simulation6entityNtB5_6Entity17attack_speed_mult(ptr %21)                                    ;L171
 21136|  %83 = tail call i64 @ai::action_score22calculate_action_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %81, ptr %80, i64 %82, ptr %50, i8 2, ptr %7) ;L171
 21137|  br label %72                                                                                                          ;L169
 21138| 
 21139| 84: ; preds = %75
 21140|     ;; self = ptr null
 21141|  tail call void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.111) #35                       ;L1013<170
 21142|  unreachable                                                                                                           ;L1013<170
 21143| 
 21144| 85: ; preds = %52
 21145|     ;; t = ptr %60
 21146|     ;; self = ptr %60
 21148|     ;; self = ptr %21
 21149|  %86 = gep %21, i64 1272                                                                                               ;L742<184
 21150|  %87 = load i32, ptr %86, , !!8                                                                                        ;L742<184
 21151|  %88 = icmp eq i32 %87, -1                                                                                             ;L742<184
 21152|  br i1 %88, label %99, label %89                                                                                       ;L742<184
 21153| 
 21154| 89: ; preds = %85
 21155|  %90 = gep %60, i64 104                                                                                                ;L1386<178
 21156|  %91 = load i64, ptr %90, , !!8                                                                                        ;L1386<178
 21157|  %92 = icmp eq i64 %91, 2                                                                                              ;L178
 21158|  %93 = select i1 %92, i64 3, i64 0                                                                                     ;L178
 21159|     ;; tower_bonus = i64 %93
 21160|  %94 = gep %21, i64 1224                                                                                               ;L742<184
 21161|     ;; self = ptr %94
 21162|     ;; effect = ptr %94
 21163|  %95 = gep %21, i64 1408                                                                                               ;L1665<185
 21164|  %96 = tail call i64 @gc::simulation6entityNtB5_6Entity15cooldown_reduce(ptr %21, i1 zeroext false)                    ;L185
 21165|  %97 = tail call i64 @ai::action_score22calculate_action_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %95, ptr %94, i64 %96, ptr %60, i8 2, ptr %7) ;L185
 21166|  %98 = add i64 %97, %93                                                                                                ;L185
 21167|  br label %72                                                                                                          ;L177
 21168| 
 21169| 99: ; preds = %85
 21170|     ;; self = ptr null
 21171|  tail call void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.112) #35                       ;L1013<184
 21172|  unreachable                                                                                                           ;L1013<184
 21173| 
 21174| 100: ; preds = %62
 21175|     ;; t = ptr %70
 21176|     ;; self = ptr %70
 21177|  %101 = gep %70, i64 104                                                                                               ;L1386<193
 21178|  %102 = load i64, ptr %101, , !!8                                                                                      ;L1386<193
 21179|  %103 = icmp eq i64 %102, 2                                                                                            ;L193
 21180|  %104 = select i1 %103, i64 3, i64 0                                                                                   ;L193
 21181|     ;; tower_bonus = i64 %104
 21182|  %105 = gep %21, i64 1480                                                                                              ;L1693<194
 21183|  %106 = load i64, ptr %105, , !!8                                                                                      ;L1693<194
 21184|  %107 = icmp ugt i64 %106, 2                                                                                           ;L1693<194
 21185|  br i1 %107, label %108, label %112                                                                                    ;L1693<194
 21186| 
 21187| 108: ; preds = %100
 21188|     ;; self = ptr %21
 21189|  %109 = gep %21, i64 1328                                                                                              ;L742<194
 21190|  %110 = load i32, ptr %109, , !!8                                                                                      ;L742<194
 21191|  %111 = icmp eq i32 %110, -1                                                                                           ;L742<194
 21192|  br i1 %111, label %112, label %113                                                                                    ;L742<194
 21193| 
 21194| 112: ; preds = %108, %100
 21195|  tail call void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.113) #35                       ;L1013<194
 21196|  unreachable                                                                                                           ;L1013<194
 21197| 
 21198| 113: ; preds = %108
 21199|  %114 = gep %21, i64 1280                                                                                              ;L1694<194
 21200|     ;; self = ptr %114
 21201|     ;; self = ptr %114
 21202|     ;; effect = ptr %114
 21203|  %115 = gep %21, i64 1424                                                                                              ;L1670<195
 21204|  %116 = tail call i64 @gc::simulation6entityNtB5_6Entity15cooldown_reduce(ptr %21, i1 zeroext false)                   ;L195
 21205|  %117 = tail call i64 @ai::action_score22calculate_action_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %115, ptr %114, i64 %116, ptr %70, i8 2, ptr %7) ;L195
 21206|  %118 = add i64 %117, %104                                                                                             ;L195
 21207|  br label %72                                                                                                          ;L191
 21208| }
