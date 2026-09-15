 49213| define void @ai::plan_legacy3old6serpen15hunt_and_battleNtB2_23SerpenHuntAndBattlePlan6update(ptr %0, i64 %1, ptr readnone %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr readnone %7) unnamed_addr #1 {
 49214|  %9 = alloca [24 x i8],
 49215|     ;; self = ptr %0
 49218|     ;; player = ptr %3
 49219|     ;; data = ptr %4
 49223|     ;; self = ptr %9
 49224|  %10 = gep %3, i64 2352                                                                                                ;L21
 49225|  %11 = load i64, ptr %10, , !!8                                                                                        ;L21
 49226|  %12 = icmp ult i64 %11, 2                                                                                             ;L21
 49227|  br i1 %12, label %14, label %13                                                                                       ;L21
 49228| 
 49229| 13: ; preds = %8
 49230|  tail call void @core::panicking18panic_bounds_check(i64 %11, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.236) #31 ;L21
 49231|  unreachable                                                                                                           ;L21
 49232| 
 49233| 14: ; preds = %8
 49234|     ;; self = ptr %3
 49235|  %15 = gep %3, i64 2496                                                                                                ;L581<21
 49236|  %16 = load i32, ptr %15, , !!8                                                                                        ;L581<21
 49237|  %17 = zext nneg i32 %16 to i64                                                                                        ;L581<21
 49238|  %18 = load ptr, ptr %4, , !!8, !!8                                                                                    ;L21
 49239|  %19 = gep %18, i64 480                                                                                                ;L21
 49240|  %20 = getelementptr [5 x ptr], ptr %19, i64 %11                                                                       ;L21
 49241|  %21 = getelementptr ptr, ptr %20, i64 %17                                                                             ;L21
 49242|  %22 = load ptr, ptr %21, , !!8                                                                                        ;L21
 49243|     ;; self = ptr %22
 49244|  %23 = icmp eq ptr %22, null                                                                                           ;L1011<21
 49245|  br i1 %23, label %39, label %24                                                                                       ;L1011<21
 49246| 
 49247| 24: ; preds = %14
 49248|     ;; champ = ptr %22
 49249|  %25 = gep %4, i64 8                                                                                                   ;L22
 49250|  %26 = load ptr, ptr %25, , !!8, !!8                                                                                   ;L22
 49251|  %27 = gep %26, i64 32                                                                                                 ;L22
 49252|  %28 = load ptr, ptr %27, , !!8, !!8                                                                                   ;L22
 49253|  %29 = icmp eq i64 %11, 0                                                                                              ;L22
 49254|  %30 = tail call { i64, i64 } @gc::simulation7map_defNtB2_6MapDef8camp_pos(ptr %28, i8 5, i1 zeroext %29)              ;L22
 49255|  %31 = extractvalue { i64, i64 } %30, 0                                                                                ;L22
 49256|  %32 = extractvalue { i64, i64 } %30, 1                                                                                ;L22
 49257|     ;; camp_pos[0..+8] = i64 %31
 49258|     ;; camp_pos[8..+8] = i64 %32
 49260|  %33 = gep %22, i64 1632                                                                                               ;L23
 49261|  %34 = load i64, ptr %33, , !!8                                                                                        ;L23
 49262|  %35 = gep %22, i64 1640                                                                                               ;L23
 49263|  %36 = load i64, ptr %35, , !!8                                                                                        ;L23
 49264|  call void @gc::simulation11map_regions16near_jungle_bush(ptr sret([24 x i8]) %9, ptr %26, i64 %34, i64 %36, i64 %31, i64 %32) ;L23
 49265|  %37 = load i64, ptr %9, , !!8                                                                                         ;L1011<23
 49266|  %38 = trunc nuw i64 %37 to i1                                                                                         ;L1011<23
 49267|  br i1 %38, label %40, label %54                                                                                       ;L1011<23
 49268| 
 49269| 39: ; preds = %14
 49270|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.237) #31                       ;L1013<21
 49271|  unreachable                                                                                                           ;L1013<21
 49272| 
 49273| 40: ; preds = %24
 49274|  %41 = gep %9, i64 8                                                                                                   ;L1012<23
 49275|  %42 = load i64, ptr %41, , !!8                                                                                        ;L1012<23
 49276|     ;; bx = i64 %42
 49277|  %43 = gep %9, i64 16                                                                                                  ;L1012<23
 49278|  %44 = load i64, ptr %43, , !!8                                                                                        ;L1012<23
 49279|     ;; by = i64 %44
 49281|  %45 = udiv i64 %44, 32000                                                                                             ;L24
 49282|     ;; self = i64 %45
 49283|     ;; min = i64 0
 49284|     ;; max = i64 29
 49285|  %46 = call i64 @llvm.umin.i64(i64 %45, i64 29)                                                                        ;L2027<24
 49286|  %47 = udiv i64 %42, 32000                                                                                             ;L24
 49287|     ;; self = i64 %47
 49288|     ;; min = i64 0
 49289|     ;; max = i64 29
 49290|  %48 = call i64 @llvm.umin.i64(i64 %47, i64 29)                                                                        ;L2027<24
 49291|  %49 = gep %28, i64 7320                                                                                               ;L24
 49292|  %50 = getelementptr [30 x i64], ptr %49, i64 %46                                                                      ;L24
 49293|  %51 = getelementptr i64, ptr %50, i64 %48                                                                             ;L24
 49294|  %52 = load i64, ptr %51, , !!8                                                                                        ;L24
 49295|     ;; target_bush = i64 %52
 49296|  store i64 1, ptr %0,                                                                                                  ;L25
 49297|  %53 = gep %0, i64 8                                                                                                   ;L25
 49298|  store i64 %52, ptr %53,                                                                                               ;L25
 49299|  ret void                                                                                                              ;L26
 49300| 
 49301| 54: ; preds = %24
 49302|  call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.238) #31                            ;L1013<23
 49303|  unreachable                                                                                                           ;L1013<23
 49304| }
