 39903| define i64 @ai::plan_legacy8sub_plan6jungleNtB2_13JungleSubPlan5score(ptr %0, i64 %1, ptr %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %7) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
 39904|     ;; self = ptr %0
 39905|     ;; version = i64 %1
 39906|     ;; parameter = ptr %2
 39907|     ;; rnd = ptr %3
 39908|     ;; player = ptr %4
 39909|     ;; data = ptr %5
 39910|     ;; action = ptr %6
 39911|     ;; debug = ptr %7
 39912|     ;; team = i64 1
 39913|     ;; team = i64 1
 39914|     ;; team = i64 1
 39915|  %9 = gep %4, i64 2352                                                                                                 ;L153
 39916|  %10 = load i64, ptr %9, , !!8                                                                                         ;L153
 39917|  %11 = icmp ult i64 %10, 2                                                                                             ;L153
 39918|  br i1 %11, label %13, label %12                                                                                       ;L153
 39919| 
 39920| 12: ; preds = %8
 39921|  tail call void @core::panicking18panic_bounds_check(i64 %10, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.190) #31 ;L153
 39922|  unreachable                                                                                                           ;L153
 39923| 
 39924| 13: ; preds = %8
 39925|     ;; self = ptr %4
 39926|  %14 = gep %4, i64 2496                                                                                                ;L581<153
 39927|  %15 = load i32, ptr %14, , !!8                                                                                        ;L581<153
 39928|  %16 = zext nneg i32 %15 to i64                                                                                        ;L581<153
 39929|  %17 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L153
 39930|  %18 = gep %17, i64 480                                                                                                ;L153
 39931|  %19 = getelementptr [5 x ptr], ptr %18, i64 %10                                                                       ;L153
 39932|  %20 = getelementptr ptr, ptr %19, i64 %16                                                                             ;L153
 39933|  %21 = load ptr, ptr %20, , !!8                                                                                        ;L153
 39934|     ;; self = ptr %21
 39935|  %22 = icmp eq ptr %21, null                                                                                           ;L1011<153
 39936|  br i1 %22, label %32, label %23                                                                                       ;L1011<153
 39937| 
 39938| 23: ; preds = %13
 39939|     ;; champ = ptr %21
 39940|     ;; self = ptr %21
 39941|     ;; self = ptr %21
 39942|     ;; self = ptr %21
 39943|  %24 = tail call i64 @ai::action_score17interaction_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %6, ptr %7)      ;L154
 39944|     ;; base = i64 %24
 39945|     ;; self = ptr %6
 39946|  %25 = gep %6, i64 177                                                                                                 ;L309<156
 39947|  %26 = load i8, ptr %25, , !!46624, !!8                                                                                ;L309<156
 39948|  %27 = icmp ne i8 %26, 10                                                                                              ;L309<156
 39949|  tail call void @llvm.assume(i1 %27)                                                                                   ;L309<156
 39950|  %28 = add nsw i8 %26, -3                                                                                              ;L309<156
 39951|  %29 = icmp samesign ugt i8 %26, 2                                                                                     ;L309<156
 39952|  %30 = select i1 %29, i8 %28, i8 7                                                                                     ;L309<156
 39953|  switch i8 %30, label %31 [
 39954|  i8 0, label %82
 39955|  i8 1, label %82
 39956|  i8 2, label %82
 39957|  i8 3, label %82
 39958|  i8 4, label %82
 39959|  i8 5, label %82
 39960|  i8 6, label %82
 39961|  i8 7, label %82
 39962|  i8 8, label %82
 39963|  i8 9, label %82
 39964|  i8 10, label %82
 39965|  i8 11, label %82
 39966|  i8 12, label %33
 39967|  i8 13, label %43
 39968|  i8 14, label %53
 39969|  i8 15, label %82
 39970|  i8 16, label %82
 39971|  ]                                                                                                                     ;L309<156
 39972| 
 39973| 31: ; preds = %23
 39974|  unreachable                                                                                                           ;L309<156
 39975| 
 39976| 32: ; preds = %13
 39977|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.191) #31                       ;L1013<153
 39978|  unreachable                                                                                                           ;L1013<153
 39979| 
 39980| 33: ; preds = %23
 39981|     ;; action = ptr %6
 39982|     ;; self = ptr %6
 39983|  %34 = gep %6, i64 8                                                                                                   ;L94<322<156
 39984|  %35 = load i64, ptr %34, , !!46624, !!8                                                                               ;L94<322<156
 39985|     ;; target_id = i64 %35
 39986|  %36 = load ptr, ptr %17, , !!8, !!8                                                                                   ;L158
 39987|  %37 = gep %17, i64 8                                                                                                  ;L158
 39988|  %38 = load ptr, ptr %37, , !!8, !!8                                                                                   ;L158
 39989|  %39 = gep %38, i64 496                                                                                                ;L158
 39990|  %40 = load ptr, ptr %39, , !!8                                                                                        ;L158
 39991|  %41 = tail call ptr %40(ptr %36, i64 %35)                                                                             ;L158
 39992|  %42 = icmp eq ptr %41, null                                                                                           ;L158
 39993|  br i1 %42, label %82, label %63                                                                                       ;L158
 39994| 
 39995| 43: ; preds = %23
 39996|     ;; action = ptr %6
 39997|     ;; self = ptr %6
 39998|  %44 = gep %6, i64 8                                                                                                   ;L160<323<156
 39999|  %45 = load i64, ptr %44, , !!46624, !!8                                                                               ;L160<323<156
 40000|     ;; target_id = i64 %45
 40001|  %46 = load ptr, ptr %17, , !!8, !!8                                                                                   ;L170
 40002|  %47 = gep %17, i64 8                                                                                                  ;L170
 40003|  %48 = load ptr, ptr %47, , !!8, !!8                                                                                   ;L170
 40004|  %49 = gep %48, i64 496                                                                                                ;L170
 40005|  %50 = load ptr, ptr %49, , !!8                                                                                        ;L170
 40006|  %51 = tail call ptr %50(ptr %46, i64 %45)                                                                             ;L170
 40007|  %52 = icmp eq ptr %51, null                                                                                           ;L170
 40008|  br i1 %52, label %82, label %84                                                                                       ;L170
 40009| 
 40010| 53: ; preds = %23
 40011|     ;; action = ptr %6
 40012|     ;; self = ptr %6
 40013|  %54 = gep %6, i64 8                                                                                                   ;L222<324<156
 40014|  %55 = load i64, ptr %54, , !!46624, !!8                                                                               ;L222<324<156
 40015|     ;; target_id = i64 %55
 40016|  %56 = load ptr, ptr %17, , !!8, !!8                                                                                   ;L182
 40017|  %57 = gep %17, i64 8                                                                                                  ;L182
 40018|  %58 = load ptr, ptr %57, , !!8, !!8                                                                                   ;L182
 40019|  %59 = gep %58, i64 496                                                                                                ;L182
 40020|  %60 = load ptr, ptr %59, , !!8                                                                                        ;L182
 40021|  %61 = tail call ptr %60(ptr %56, i64 %55)                                                                             ;L182
 40022|  %62 = icmp eq ptr %61, null                                                                                           ;L182
 40023|  br i1 %62, label %82, label %102                                                                                      ;L182
 40024| 
 40025| 63: ; preds = %33
 40026|     ;; t = ptr %41
 40027|     ;; self = ptr %21
 40028|  %64 = gep %21, i64 1216                                                                                               ;L742<159
 40029|  %65 = load i32, ptr %64, , !!8                                                                                        ;L742<159
 40030|  %66 = icmp eq i32 %65, -1                                                                                             ;L742<159
 40031|  br i1 %66, label %75, label %67                                                                                       ;L742<159
 40032| 
 40033| 67: ; preds = %63
 40034|  %68 = gep %21, i64 1168                                                                                               ;L742<159
 40035|  %69 = gep %21, i64 1392                                                                                               ;L1494<159
 40036|     ;; self = ptr %68
 40037|  %70 = tail call i64 @ai::action_score29calculate_jungle_action_score(ptr %3, ptr %4, ptr %5, ptr %2, ptr %69, ptr %68, ptr %41) ;L159
 40038|  %71 = add i64 %70, %24                                                                                                ;L159
 40039|     ;; score = i64 %71
 40040|     ;; self = ptr %41
 40041|  %72 = gep %41, i64 104                                                                                                ;L1378<160
 40042|  %73 = load i64, ptr %72, , !!8                                                                                        ;L1378<160
 40043|  %74 = icmp eq i64 %73, 4                                                                                              ;L1378<160
 40044|  br i1 %74, label %76, label %82                                                                                       ;L1378<160
 40045| 
 40046| 75: ; preds = %63
 40047|     ;; self = ptr null
 40048|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.192) #31                       ;L1013<159
 40049|  unreachable                                                                                                           ;L1013<159
 40050| 
 40051| 76: ; preds = %67
 40052|  %77 = gep %41, i64 152                                                                                                ;L1378<160
 40053|  %78 = load i64, ptr %77, , !!8                                                                                        ;L1378<160
 40054|  %79 = icmp ult i64 %78, 2                                                                                             ;L1378<160
 40055|  br i1 %79, label %80, label %82                                                                                       ;L1378<160
 40056| 
 40057| 80: ; preds = %76
 40058|     ;; self = i64 %71
 40059|     ;; other = i64 1
 40060|  %81 = tail call i64 @llvm.smax.i64(i64 %71, i64 1)                                                                    ;L1039<161
 40061|     ;; base = i64 %81
 40062|  br label %82                                                                                                          ;L161
 40063| 
 40064| 82: ; preds = %123, %119, %111, %100, %96, %88, %80, %76, %67, %53, %43, %33, %23, %23, %23, %23, %23, %23, %23, %23, %23, %23, %23, %23, %23, %23
 40065|  %83 = phi i64 [ %114, %111 ], [ %91, %88 ], [ %71, %67 ], [ %81, %80 ], [ -99999, %33 ], [ %71, %76 ], [ %101, %100 ], [ -99999, %43 ], [ %91, %96 ], [ %124, %123 ], [ -99999, %53 ], [ %114, %119 ], [ %24, %23 ], [ %24, %23 ], [ %24, %23 ], [ %24, %23 ], [ %24, %23 ], [ %24, %23 ], [ %24, %23 ], [ %24, %23 ], [ %24, %23 ], [ %24, %23 ], [ %24, %23 ], [ %24, %23 ], [ %24, %23 ], [ %24, %23 ] ;L0
 40066|     ;; base = i64 %83
 40067|  ret i64 %83                                                                                                           ;L195
 40068| 
 40069| 84: ; preds = %43
 40070|     ;; t = ptr %51
 40071|     ;; self = ptr %21
 40072|  %85 = gep %21, i64 1272                                                                                               ;L742<171
 40073|  %86 = load i32, ptr %85, , !!8                                                                                        ;L742<171
 40074|  %87 = icmp eq i32 %86, -1                                                                                             ;L742<171
 40075|  br i1 %87, label %95, label %88                                                                                       ;L742<171
 40076| 
 40077| 88: ; preds = %84
 40078|  %89 = gep %21, i64 1224                                                                                               ;L742<171
 40079|  %90 = gep %21, i64 1408                                                                                               ;L1665<171
 40080|     ;; self = ptr %89
 40081|  %91 = tail call i64 @ai::action_score29calculate_jungle_action_score(ptr %3, ptr %4, ptr %5, ptr %2, ptr %90, ptr %89, ptr %51) ;L171
 40082|     ;; score = i64 %91
 40083|     ;; self = ptr %51
 40084|  %92 = gep %51, i64 104                                                                                                ;L1378<172
 40085|  %93 = load i64, ptr %92, , !!8                                                                                        ;L1378<172
 40086|  %94 = icmp eq i64 %93, 4                                                                                              ;L1378<172
 40087|  br i1 %94, label %96, label %82                                                                                       ;L1378<172
 40088| 
 40089| 95: ; preds = %84
 40090|     ;; self = ptr null
 40091|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.193) #31                       ;L1013<171
 40092|  unreachable                                                                                                           ;L1013<171
 40093| 
 40094| 96: ; preds = %88
 40095|  %97 = gep %51, i64 152                                                                                                ;L1378<172
 40096|  %98 = load i64, ptr %97, , !!8                                                                                        ;L1378<172
 40097|  %99 = icmp ult i64 %98, 2                                                                                             ;L1378<172
 40098|  br i1 %99, label %100, label %82                                                                                      ;L1378<172
 40099| 
 40100| 100: ; preds = %96
 40101|     ;; self = i64 %91
 40102|     ;; other = i64 1
 40103|  %101 = tail call i64 @llvm.smax.i64(i64 %91, i64 1)                                                                   ;L1039<173
 40104|     ;; base = i64 %101
 40105|  br label %82                                                                                                          ;L173
 40106| 
 40107| 102: ; preds = %53
 40108|     ;; t = ptr %61
 40109|  %103 = gep %21, i64 1480                                                                                              ;L1669<183
 40110|  %104 = load i64, ptr %103, , !!8                                                                                      ;L1669<183
 40111|  %105 = icmp ugt i64 %104, 2                                                                                           ;L1669<183
 40112|  %106 = gep %21, i64 1280                                                                                              ;L1669<183
 40113|  %107 = select i1 %105, ptr %106, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.19                                        ;L1669<183
 40114|     ;; self = ptr %107
 40115|  %108 = gep %107, i64 48                                                                                               ;L742<183
 40116|  %109 = load i32, ptr %108, , !!8                                                                                      ;L742<183
 40117|  %110 = icmp eq i32 %109, -1                                                                                           ;L742<183
 40118|  br i1 %110, label %118, label %111                                                                                    ;L742<183
 40119| 
 40120| 111: ; preds = %102
 40121|  %112 = select i1 %105, i64 1424, i64 1456                                                                             ;L1669<183
 40122|  %113 = gep %21, i64 %112                                                                                              ;L1669<183
 40123|     ;; self = ptr %107
 40124|  %114 = tail call i64 @ai::action_score29calculate_jungle_action_score(ptr %3, ptr %4, ptr %5, ptr %2, ptr %113, ptr %107, ptr %61) ;L183
 40125|     ;; score = i64 %114
 40126|     ;; self = ptr %61
 40127|  %115 = gep %61, i64 104                                                                                               ;L1378<184
 40128|  %116 = load i64, ptr %115, , !!8                                                                                      ;L1378<184
 40129|  %117 = icmp eq i64 %116, 4                                                                                            ;L1378<184
 40130|  br i1 %117, label %119, label %82                                                                                     ;L1378<184
 40131| 
 40132| 118: ; preds = %102
 40133|     ;; self = ptr null
 40134|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.194) #31                       ;L1013<183
 40135|  unreachable                                                                                                           ;L1013<183
 40136| 
 40137| 119: ; preds = %111
 40138|  %120 = gep %61, i64 152                                                                                               ;L1378<184
 40139|  %121 = load i64, ptr %120, , !!8                                                                                      ;L1378<184
 40140|  %122 = icmp ult i64 %121, 2                                                                                           ;L1378<184
 40141|  br i1 %122, label %123, label %82                                                                                     ;L1378<184
 40142| 
 40143| 123: ; preds = %119
 40144|     ;; self = i64 %114
 40145|     ;; other = i64 1
 40146|  %124 = tail call i64 @llvm.smax.i64(i64 %114, i64 1)                                                                  ;L1039<185
 40147|     ;; base = i64 %124
 40148|  br label %82                                                                                                          ;L185
 40149| }
