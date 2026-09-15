 12554| define hidden zeroext i1 @ai::small_action4castNtB5_14SmallActionUlt6is_end(ptr %0, ptr readnone %1, i64 %2, ptr %3, ptr %4) unnamed_addr #0 {
 12555|     ;; self = ptr %0
 12556|     ;; _rnd = ptr %1
 12557|     ;; _version = i64 %2
 12558|     ;; player = ptr %3
 12559|     ;; data = ptr %4
 12560|  %6 = load i64, ptr %0, , !!8                                                                                          ;L291
 12561|  %7 = add i64 %6, 5                                                                                                    ;L291
 12562|  %8 = load ptr, ptr %4, , !!8, !!8                                                                                     ;L291
 12563|  %9 = load ptr, ptr %8, , !!8, !!8                                                                                     ;L291
 12564|  %10 = gep %8, i64 8                                                                                                   ;L291
 12565|  %11 = load ptr, ptr %10, , !!8, !!8                                                                                   ;L291
 12566|  %12 = gep %11, i64 40                                                                                                 ;L291
 12567|  %13 = load ptr, ptr %12, , !!8                                                                                        ;L291
 12568|  %14 = tail call i64 %13(ptr %9)                                                                                       ;L291
 12569|  %15 = icmp ule i64 %7, %14                                                                                            ;L291
 12570|  %16 = gep %0, i64 16                                                                                                  ;L291
 12571|  %17 = load i8, ptr %16,                                                                                               ;L291
 12572|  %18 = trunc nuw i8 %17 to i1                                                                                          ;L291
 12573|  %19 = select i1 %15, i1 true, i1 %18                                                                                  ;L291
 12574|  br i1 %19, label %55, label %20                                                                                       ;L291
 12575| 
 12576| 20: ; preds = %5
 12577|  %21 = gep %0, i64 8                                                                                                   ;L292
 12578|  %22 = load i64, ptr %21, , !!8                                                                                        ;L292
 12579|  %23 = gep %11, i64 496                                                                                                ;L292
 12580|  %24 = load ptr, ptr %23, , !!8                                                                                        ;L292
 12581|  %25 = tail call ptr %24(ptr %9, i64 %22)                                                                              ;L292
 12582|     ;; self = ptr %25
 12583|     ;; f[0..+8] = ptr %8
 12584|     ;; f[8..+8] = ptr %3
 12585|  %26 = icmp eq ptr %25, null                                                                                           ;L708<292
 12586|  br i1 %26, label %55, label %27                                                                                       ;L708<292
 12587| 
 12588| 27: ; preds = %20
 12589|     ;; x = ptr %25
 12590|  %28 = gep %3, i64 2352                                                                                                ;L710<292
 12591|  %29 = load i64, ptr %28, , !!8                                                                                        ;L710<292
 12592|  %30 = gep %3, i64 2496                                                                                                ;L710<292
 12593|  %31 = load i32, ptr %30,                                                                                              ;L710<292
 12599|     ;; t = ptr %25
 12600|     ;; self = ptr %25
 12601|  %32 = icmp ult i64 %29, 2                                                                                             ;L293<710<292
 12602|  br i1 %32, label %34, label %33                                                                                       ;L293<710<292
 12603| 
 12604| 33: ; preds = %27
 12605|  tail call void @core::panicking18panic_bounds_check(i64 %29, i64 2, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.9) #25, !!27504 ;L293<710<292
 12606|  unreachable                                                                                                           ;L293<710<292
 12607| 
 12608| 34: ; preds = %27
 12610|  %35 = zext nneg i32 %31 to i64                                                                                        ;L581<293<710<292
 12611|  %36 = gep %8, i64 480                                                                                                 ;L293<710<292
 12612|  %37 = getelementptr [5 x ptr], ptr %36, i64 %29                                                                       ;L293<710<292
 12613|  %38 = getelementptr ptr, ptr %37, i64 %35                                                                             ;L293<710<292
 12614|  %39 = load ptr, ptr %38, , !!27482, !!8                                                                               ;L293<710<292
 12615|     ;; self = ptr %39
 12616|  %40 = icmp eq ptr %39, null                                                                                           ;L1011<293<710<292
 12617|  br i1 %40, label %44, label %41                                                                                       ;L1011<293<710<292
 12618| 
 12619| 41: ; preds = %34
 12620|     ;; champ = ptr %39
 12621|     ;; entity = ptr %39
 12622|     ;; self = ptr %39
 12623|  %42 = load i64, ptr %39, , !!27504, !!8                                                                               ;L1136<1482<294<710<292
 12624|  %43 = trunc nuw i64 %42 to i1                                                                                         ;L1136<1482<294<710<292
 12625|  br i1 %43, label %55, label %45                                                                                       ;L1136<1482<294<710<292
 12626| 
 12627| 44: ; preds = %34
 12628|  tail call void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.10) #25, !!27504               ;L1013<293<710<292
 12629|  unreachable                                                                                                           ;L1013<293<710<292
 12630| 
 12631| 45: ; preds = %41
 12632|  %46 = gep %39, i64 8                                                                                                  ;L1136<1482<294<710<292
 12633|     ;; team = ptr %39
 12634|  %47 = load i64, ptr %46, , !!27504, !!8                                                                               ;L1137<1482<294<710<292
 12635|     ;; team = i64 %47
 12636|  %48 = icmp ult i64 %47, 2                                                                                             ;L1483<294<710<292
 12637|  br i1 %48, label %49, label %54                                                                                       ;L1483<294<710<292
 12638| 
 12639| 49: ; preds = %45
 12641|  %50 = gep %25, i64 56                                                                                                 ;L122<1483<294<710<292
 12642|  %51 = gepS %50, i64 %47                                                                                               ;L122<1483<294<710<292
 12643|  %52 = load i64, ptr %51, , !!27479, !!8                                                                               ;L122<1483<294<710<292
 12644|  %53 = icmp ne i64 %52, 0                                                                                              ;L122<1483<294<710<292
 12645|  br label %55                                                                                                          ;L1482<294<710<292
 12646| 
 12647| 54: ; preds = %45
 12648|  tail call void @core::panicking18panic_bounds_check(i64 %47, i64 2, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.8) #25, !!27504 ;L1483<294<710<292
 12649|  unreachable                                                                                                           ;L1483<294<710<292
 12650| 
 12651| 55: ; preds = %49, %41, %20, %5
 12652|  %56 = phi i1 [ true, %5 ], [ true, %20 ], [ %53, %49 ], [ false, %41 ]                                                ;L0
 12653|  ret i1 %56                                                                                                            ;L296
 12654| }
