 40317| define i64 @ai::plan_legacy8sub_plan6recallNtB2_13RecallSubPlan5score(ptr %0, i64 %1, ptr %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %7) unnamed_addr #1 {
 40318|     ;; self = ptr %0
 40319|     ;; version = i64 %1
 40320|     ;; parameter = ptr %2
 40321|     ;; rnd = ptr %3
 40322|     ;; player = ptr %4
 40323|     ;; data = ptr %5
 40324|     ;; action = ptr %6
 40325|     ;; debug = ptr %7
 40326|  %9 = tail call i64 @ai::action_score17interaction_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %6, ptr %7)       ;L43
 40327|     ;; base = i64 %9
 40328|     ;; self = ptr %6
 40329|  %10 = gep %6, i64 177                                                                                                 ;L309<46
 40330|  %11 = load i8, ptr %10, , !!46812, !!8                                                                                ;L309<46
 40331|  %12 = icmp ne i8 %11, 10                                                                                              ;L309<46
 40332|  tail call void @llvm.assume(i1 %12)                                                                                   ;L309<46
 40333|  %13 = add nsw i8 %11, -3                                                                                              ;L309<46
 40334|  %14 = icmp samesign ugt i8 %11, 2                                                                                     ;L309<46
 40335|  %15 = select i1 %14, i8 %13, i8 7                                                                                     ;L309<46
 40336|  switch i8 %15, label %16 [
 40337|  i8 0, label %19
 40338|  i8 1, label %19
 40339|  i8 2, label %17
 40340|  i8 3, label %17
 40341|  i8 4, label %17
 40342|  i8 5, label %19
 40343|  i8 6, label %17
 40344|  i8 7, label %17
 40345|  i8 8, label %17
 40346|  i8 9, label %17
 40347|  i8 10, label %17
 40348|  i8 11, label %17
 40349|  i8 12, label %17
 40350|  i8 13, label %17
 40351|  i8 14, label %17
 40352|  i8 15, label %17
 40353|  i8 16, label %17
 40354|  ]                                                                                                                     ;L309<46
 40355| 
 40356| 16: ; preds = %8
 40357|  unreachable                                                                                                           ;L309<46
 40358| 
 40359| 17: ; preds = %8, %8, %8, %8, %8, %8, %8, %8, %8, %8, %8, %8, %8, %8
 40360|  %18 = icmp sgt i64 %9, 0                                                                                              ;L48
 40361|  br i1 %18, label %23, label %21                                                                                       ;L48
 40362| 
 40363| 19: ; preds = %8, %8, %8
 40364|  %20 = add i64 %9, 50                                                                                                  ;L47
 40365|  br label %91                                                                                                          ;L46
 40366| 
 40367| 21: ; preds = %17
 40368|  %22 = mul i64 %9, 3                                                                                                   ;L58
 40369|  br label %91                                                                                                          ;L48
 40370| 
 40371| 23: ; preds = %17
 40372|  %24 = gep %4, i64 2352                                                                                                ;L49
 40373|  %25 = load i64, ptr %24, , !!8                                                                                        ;L49
 40374|  %26 = icmp ult i64 %25, 2                                                                                             ;L49
 40375|  br i1 %26, label %28, label %27                                                                                       ;L49
 40376| 
 40377| 27: ; preds = %23
 40378|  tail call void @core::panicking18panic_bounds_check(i64 %25, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.198) #31 ;L49
 40379|  unreachable                                                                                                           ;L49
 40380| 
 40381| 28: ; preds = %23
 40382|     ;; self = ptr %4
 40383|  %29 = gep %4, i64 2496                                                                                                ;L581<49
 40384|  %30 = load i32, ptr %29, , !!8                                                                                        ;L581<49
 40385|  %31 = zext nneg i32 %30 to i64                                                                                        ;L581<49
 40386|  %32 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L49
 40387|  %33 = gep %32, i64 480                                                                                                ;L49
 40388|  %34 = getelementptr [5 x ptr], ptr %33, i64 %25                                                                       ;L49
 40389|  %35 = getelementptr ptr, ptr %34, i64 %31                                                                             ;L49
 40390|  %36 = load ptr, ptr %35, , !!8                                                                                        ;L49
 40391|     ;; self = ptr %36
 40392|  %37 = icmp eq ptr %36, null                                                                                           ;L1011<49
 40393|  br i1 %37, label %49, label %38                                                                                       ;L1011<49
 40394| 
 40395| 38: ; preds = %28
 40396|     ;; champ = ptr %36
 40397|     ;; self = ptr %36
 40398|     ;; self = ptr %6
 40399|  switch i8 %15, label %39 [
 40400|  i8 16, label %89
 40401|  i8 15, label %89
 40402|  i8 2, label %89
 40403|  i8 3, label %89
 40404|  i8 4, label %89
 40405|  i8 5, label %89
 40406|  i8 6, label %89
 40407|  i8 7, label %89
 40408|  i8 8, label %89
 40409|  i8 9, label %89
 40410|  i8 10, label %89
 40411|  i8 11, label %89
 40412|  i8 12, label %89
 40413|  i8 13, label %50
 40414|  i8 14, label %40
 40415|  ]                                                                                                                     ;L309<51
 40416| 
 40417| 39: ; preds = %38
 40418|  unreachable                                                                                                           ;L309<51
 40419| 
 40420| 40: ; preds = %38
 40421|     ;; self = ptr %6
 40422|  %41 = gep %36, i64 1480                                                                                               ;L1693<52
 40423|  %42 = load i64, ptr %41, , !!8                                                                                        ;L1693<52
 40424|  %43 = icmp ugt i64 %42, 2                                                                                             ;L1693<52
 40425|  %44 = gep %36, i64 1280                                                                                               ;L1693<52
 40426|  %45 = select i1 %43, ptr %44, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.19                                           ;L1693<52
 40427|     ;; self = ptr %45
 40428|  %46 = gep %45, i64 48                                                                                                 ;L742<52
 40429|  %47 = load i32, ptr %46, , !!8                                                                                        ;L742<52
 40430|  %48 = icmp eq i32 %47, -1                                                                                             ;L742<52
 40431|  br i1 %48, label %88, label %73                                                                                       ;L742<52
 40432| 
 40433| 49: ; preds = %28
 40434|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.199) #31                       ;L1013<49
 40435|  unreachable                                                                                                           ;L1013<49
 40436| 
 40437| 50: ; preds = %38
 40438|     ;; self = ptr %36
 40439|  %51 = gep %36, i64 1272                                                                                               ;L742<51
 40440|  %52 = load i32, ptr %51, , !!8                                                                                        ;L742<51
 40441|  %53 = icmp eq i32 %52, -1                                                                                             ;L742<51
 40442|  br i1 %53, label %70, label %54                                                                                       ;L742<51
 40443| 
 40444| 54: ; preds = %50
 40445|  %55 = gep %36, i64 1224                                                                                               ;L742<51
 40446|     ;; self = ptr %55
 40447|     ;; self = ptr %55
 40448|     ;; self = ptr %55
 40449|  %56 = load ptr, ptr %55, , !!8, !!8                                                                                   ;L441<2127<2445<51
 40450|  %57 = gep %36, i64 1232                                                                                               ;L441<2127<2445<51
 40451|  %58 = load ptr, ptr %57, , !!8, !!8                                                                                   ;L441<2127<2445<51
 40452|  %59 = gep %58, i64 16                                                                                                 ;L2445<51
 40453|  %60 = load i64, ptr %59,                                                                                              ;L2445<51
 40454|  %61 = add nsw i64 %60, -1                                                                                             ;L2445<51
 40455|  %62 = and i64 %61, -16                                                                                                ;L2445<51
 40456|  %63 = gep %56, i64 %62                                                                                                ;L2445<51
 40457|  %64 = gep %63, i64 16                                                                                                 ;L2445<51
 40458|  %65 = gep %58, i64 128                                                                                                ;L51
 40459|  %66 = load ptr, ptr %65, , !!8                                                                                        ;L51
 40460|  %67 = tail call { i64, i64 } %66(ptr %64)                                                                             ;L51
 40461|  %68 = extractvalue { i64, i64 } %67, 0                                                                                ;L51
 40462|  %69 = icmp eq i64 %68, 1                                                                                              ;L51
 40463|  br i1 %69, label %71, label %89                                                                                       ;L51
 40464| 
 40465| 70: ; preds = %50
 40466|     ;; self = ptr null
 40467|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.200) #31                       ;L1013<51
 40468|  unreachable                                                                                                           ;L1013<51
 40469| 
 40470| 71: ; preds = %73, %54
 40471|  %72 = add nuw i64 %9, 50                                                                                              ;L53
 40472|  br label %91                                                                                                          ;L51
 40473| 
 40474| 73: ; preds = %40
 40475|     ;; self = ptr %45
 40476|     ;; self = ptr %45
 40477|     ;; self = ptr %45
 40478|  %74 = load ptr, ptr %45, , !!8, !!8                                                                                   ;L441<2127<2445<52
 40479|  %75 = gep %45, i64 8                                                                                                  ;L441<2127<2445<52
 40480|  %76 = load ptr, ptr %75, , !!8, !!8                                                                                   ;L441<2127<2445<52
 40481|  %77 = gep %76, i64 16                                                                                                 ;L2445<52
 40482|  %78 = load i64, ptr %77,                                                                                              ;L2445<52
 40483|  %79 = add nsw i64 %78, -1                                                                                             ;L2445<52
 40484|  %80 = and i64 %79, -16                                                                                                ;L2445<52
 40485|  %81 = gep %74, i64 %80                                                                                                ;L2445<52
 40486|  %82 = gep %81, i64 16                                                                                                 ;L2445<52
 40487|  %83 = gep %76, i64 128                                                                                                ;L52
 40488|  %84 = load ptr, ptr %83, , !!8                                                                                        ;L52
 40489|  %85 = tail call { i64, i64 } %84(ptr %82)                                                                             ;L52
 40490|  %86 = extractvalue { i64, i64 } %85, 0                                                                                ;L52
 40491|  %87 = icmp eq i64 %86, 1                                                                                              ;L52
 40492|  br i1 %87, label %71, label %89                                                                                       ;L52
 40493| 
 40494| 88: ; preds = %40
 40495|     ;; self = ptr null
 40496|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.201) #31                       ;L1013<52
 40497|  unreachable                                                                                                           ;L1013<52
 40498| 
 40499| 89: ; preds = %73, %54, %38, %38, %38, %38, %38, %38, %38, %38, %38, %38, %38, %38, %38
 40500|  %90 = udiv i64 %9, 3                                                                                                  ;L55
 40501|  br label %91                                                                                                          ;L51
 40502| 
 40503| 91: ; preds = %89, %71, %21, %19
 40504|  %92 = phi i64 [ %20, %19 ], [ %72, %71 ], [ %90, %89 ], [ %22, %21 ]                                                  ;L0
 40505|  ret i64 %92                                                                                                           ;L60
 40506| }
