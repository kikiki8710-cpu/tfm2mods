 61661| define hidden zeroext i1 @ai::plan_legacy3old13defense_nexus26nexus_final_stand_uncached(ptr %0, ptr %1) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
 61666|  %3 = alloca [56 x i8],
 61667|     ;; player = ptr %0
 61668|     ;; data = ptr %1
 61670|  %4 = gep %0, i64 2352                                                                                                 ;L240
 61671|  %5 = load i64, ptr %4, , !!8                                                                                          ;L240
 61672|     ;; team = i64 %5
 61673|  %6 = icmp ult i64 %5, 2                                                                                               ;L241
 61674|  br i1 %6, label %7, label %13                                                                                         ;L241
 61675| 
 61676| 7: ; preds = %2
 61677|  %8 = load ptr, ptr %1, , !!8, !!8                                                                                     ;L241
 61678|     ;; self = ptr %8
 61679|  %9 = gep %8, i64 368                                                                                                  ;L241
 61680|  %10 = getelementptr ptr, ptr %9, i64 %5                                                                               ;L241
 61681|  %11 = load ptr, ptr %10, , !!8                                                                                        ;L241
 61682|  %12 = icmp eq ptr %11, null                                                                                           ;L241
 61683|  br i1 %12, label %79, label %14                                                                                       ;L241
 61684| 
 61685| 13: ; preds = %2
 61686|  tail call void @core::panicking18panic_bounds_check(i64 %5, i64 2, ptr @anon.168add0ea037d45d276f5936ae758fe5.262) #30 ;L241
 61687|  unreachable                                                                                                           ;L241
 61688| 
 61689| 14: ; preds = %7
 61690|     ;; nexus = ptr %11
 61691|  %15 = getelementptr { { ptr, ptr, i64 }, i64 }, ptr %8, i64 %5                                                        ;L244
 61692|     ;; self = ptr %15
 61693|     ;; self = ptr %15
 61694|  %16 = gep %15, i64 328                                                                                                ;L1617<1636<244
 61695|  %17 = load i64, ptr %16, , !!8                                                                                        ;L1617<1636<244
 61696|  %18 = icmp eq i64 %17, 0                                                                                              ;L244
 61697|  br i1 %18, label %19, label %79                                                                                       ;L244
 61698| 
 61699| 19: ; preds = %14
 61701|  %20 = sub nuw nsw i64 1, %5                                                                                           ;L247
 61702|     ;; team = i64 %20
 61703|  call void @gc::simulationNtB5_21AbstractGameWithCache12iter_minions(ptr sret([56 x i8]) %3, ptr %8, i64 %20)          ;L247
 61704|     ;; self = ptr %3
 61705|     ;; f = ptr %11
 61706|  %21 = call zeroext i1 @core::iter8adapters5chainINtB5_5ChainIBP_INtNtB7_6copied6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEEB14_EB14_ENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2X_3any5checkB1Q_NCNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus26nexus_final_stand_uncached0E0INtNtNtBb_3ops12control_flow11ControlFlowuEEB4e_(ptr %3, ptr %11) ;L2897<247
 61708|  br i1 %21, label %79, label %22                                                                                       ;L247
 61709| 
 61710| 22: ; preds = %19
 61711|  %23 = gep %8, i64 480                                                                                                 ;L1905<252
 61712|  %24 = getelementptr [5 x ptr], ptr %23, i64 %20                                                                       ;L1905<252
 61713|     ;; self = ptr undef
 61714|     ;; self = ptr undef
 61715|     ;; f = ptr %11
 61716|     ;; fold = ptr %11
 61718|     ;; f[8..+8] = ptr %11
 61719|     ;; self = ptr undef
 61722|     ;; self = ptr undef
 61723|     ;; count = i64 1
 61724|     ;; self = ptr %24
 61725|     ;; end_or_len = ptr %24
 61726|     ;; ptr = ptr %24
 61727|     ;; x = ptr %24
 61728|  %25 = load ptr, ptr %24, , !!70564, !!8                                                                               ;L2494<138<2897<252
 61733|  %26 = icmp eq ptr %25, null                                                                                           ;L49<2494<138<2897<252
 61734|  br i1 %26, label %34, label %27                                                                                       ;L49<2494<138<2897<252
 61735| 
 61736| 27: ; preds = %22
 61737|     ;; x = ptr %25
 61740|     ;; x = ptr %25
 61742|     ;; c = ptr %25
 61744|     ;; self = ptr %25
 61745|  %28 = gep %25, i64 1216                                                                                               ;L742<253<2893<50<2494<138<2897<252
 61746|  %29 = load i32, ptr %28, , !!70564, !!8                                                                               ;L742<253<2893<50<2494<138<2897<252
 61747|  %30 = icmp eq i32 %29, -1                                                                                             ;L742<253<2893<50<2494<138<2897<252
 61748|  br i1 %30, label %34, label %31                                                                                       ;L742<253<2893<50<2494<138<2897<252
 61749| 
 61750| 31: ; preds = %27
 61751|  %32 = gep %25, i64 1168                                                                                               ;L742<253<2893<50<2494<138<2897<252
 61752|     ;; self = ptr %32
 61753|     ;; f[0..+8] = ptr %25
 61754|     ;; f[8..+8] = ptr %11
 61755|     ;; x = ptr %32
 61756|     ;; atk = ptr %32
 61758|  %33 = call zeroext i1 @gc::simulation6effectNtB2_6Effect11is_in_range(ptr %32, ptr %25, ptr %11), !!70564             ;L253<661<253<2893<50<2494<138<2897<252
 61759|  br i1 %33, label %79, label %34                                                                                       ;L2494<138<2897<252
 61760| 
 61761| 34: ; preds = %31, %27, %22
 61762|     ;; self = ptr undef
 61763|     ;; count = i64 1
 61764|     ;; ptr = !DIArgList(ptr %24, i64 8)
 61765|     ;; self = !DIArgList(ptr %24, i64 8)
 61766|     ;; end_or_len = ptr %24
 61767|  %35 = gep %24, i64 8                                                                                                  ;L656<185<2493<138<2897<252
 61768|     ;; ptr = ptr %35
 61769|     ;; x = ptr %35
 61770|  %36 = load ptr, ptr %35, , !!70564, !!8                                                                               ;L2494<138<2897<252
 61775|  %37 = icmp eq ptr %36, null                                                                                           ;L49<2494<138<2897<252
 61776|  br i1 %37, label %45, label %38                                                                                       ;L49<2494<138<2897<252
 61777| 
 61778| 38: ; preds = %34
 61779|     ;; x = ptr %36
 61782|     ;; x = ptr %36
 61784|     ;; c = ptr %36
 61786|     ;; self = ptr %36
 61787|  %39 = gep %36, i64 1216                                                                                               ;L742<253<2893<50<2494<138<2897<252
 61788|  %40 = load i32, ptr %39, , !!70564, !!8                                                                               ;L742<253<2893<50<2494<138<2897<252
 61789|  %41 = icmp eq i32 %40, -1                                                                                             ;L742<253<2893<50<2494<138<2897<252
 61790|  br i1 %41, label %45, label %42                                                                                       ;L742<253<2893<50<2494<138<2897<252
 61791| 
 61792| 42: ; preds = %38
 61793|  %43 = gep %36, i64 1168                                                                                               ;L742<253<2893<50<2494<138<2897<252
 61794|     ;; self = ptr %43
 61795|     ;; f[0..+8] = ptr %36
 61796|     ;; f[8..+8] = ptr %11
 61797|     ;; x = ptr %43
 61798|     ;; atk = ptr %43
 61800|  %44 = call zeroext i1 @gc::simulation6effectNtB2_6Effect11is_in_range(ptr %43, ptr %36, ptr %11), !!70564             ;L253<661<253<2893<50<2494<138<2897<252
 61801|  br i1 %44, label %79, label %45                                                                                       ;L2494<138<2897<252
 61802| 
 61803| 45: ; preds = %42, %38, %34
 61804|     ;; self = ptr undef
 61805|     ;; count = i64 1
 61806|     ;; ptr = !DIArgList(ptr %24, i64 16)
 61807|     ;; self = !DIArgList(ptr %24, i64 16)
 61808|     ;; end_or_len = ptr %24
 61809|  %46 = gep %24, i64 16                                                                                                 ;L656<185<2493<138<2897<252
 61810|     ;; ptr = ptr %46
 61811|     ;; x = ptr %46
 61812|  %47 = load ptr, ptr %46, , !!70564, !!8                                                                               ;L2494<138<2897<252
 61817|  %48 = icmp eq ptr %47, null                                                                                           ;L49<2494<138<2897<252
 61818|  br i1 %48, label %56, label %49                                                                                       ;L49<2494<138<2897<252
 61819| 
 61820| 49: ; preds = %45
 61821|     ;; x = ptr %47
 61824|     ;; x = ptr %47
 61826|     ;; c = ptr %47
 61828|     ;; self = ptr %47
 61829|  %50 = gep %47, i64 1216                                                                                               ;L742<253<2893<50<2494<138<2897<252
 61830|  %51 = load i32, ptr %50, , !!70564, !!8                                                                               ;L742<253<2893<50<2494<138<2897<252
 61831|  %52 = icmp eq i32 %51, -1                                                                                             ;L742<253<2893<50<2494<138<2897<252
 61832|  br i1 %52, label %56, label %53                                                                                       ;L742<253<2893<50<2494<138<2897<252
 61833| 
 61834| 53: ; preds = %49
 61835|  %54 = gep %47, i64 1168                                                                                               ;L742<253<2893<50<2494<138<2897<252
 61836|     ;; self = ptr %54
 61837|     ;; f[0..+8] = ptr %47
 61838|     ;; f[8..+8] = ptr %11
 61839|     ;; x = ptr %54
 61840|     ;; atk = ptr %54
 61842|  %55 = call zeroext i1 @gc::simulation6effectNtB2_6Effect11is_in_range(ptr %54, ptr %47, ptr %11), !!70564             ;L253<661<253<2893<50<2494<138<2897<252
 61843|  br i1 %55, label %79, label %56                                                                                       ;L2494<138<2897<252
 61844| 
 61845| 56: ; preds = %53, %49, %45
 61846|     ;; self = ptr undef
 61847|     ;; count = i64 1
 61848|     ;; ptr = !DIArgList(ptr %24, i64 24)
 61849|     ;; self = !DIArgList(ptr %24, i64 24)
 61850|     ;; end_or_len = ptr %24
 61851|  %57 = gep %24, i64 24                                                                                                 ;L656<185<2493<138<2897<252
 61852|     ;; ptr = ptr %57
 61853|     ;; x = ptr %57
 61854|  %58 = load ptr, ptr %57, , !!70564, !!8                                                                               ;L2494<138<2897<252
 61859|  %59 = icmp eq ptr %58, null                                                                                           ;L49<2494<138<2897<252
 61860|  br i1 %59, label %67, label %60                                                                                       ;L49<2494<138<2897<252
 61861| 
 61862| 60: ; preds = %56
 61863|     ;; x = ptr %58
 61866|     ;; x = ptr %58
 61868|     ;; c = ptr %58
 61870|     ;; self = ptr %58
 61871|  %61 = gep %58, i64 1216                                                                                               ;L742<253<2893<50<2494<138<2897<252
 61872|  %62 = load i32, ptr %61, , !!70564, !!8                                                                               ;L742<253<2893<50<2494<138<2897<252
 61873|  %63 = icmp eq i32 %62, -1                                                                                             ;L742<253<2893<50<2494<138<2897<252
 61874|  br i1 %63, label %67, label %64                                                                                       ;L742<253<2893<50<2494<138<2897<252
 61875| 
 61876| 64: ; preds = %60
 61877|  %65 = gep %58, i64 1168                                                                                               ;L742<253<2893<50<2494<138<2897<252
 61878|     ;; self = ptr %65
 61879|     ;; f[0..+8] = ptr %58
 61880|     ;; f[8..+8] = ptr %11
 61881|     ;; x = ptr %65
 61882|     ;; atk = ptr %65
 61884|  %66 = call zeroext i1 @gc::simulation6effectNtB2_6Effect11is_in_range(ptr %65, ptr %58, ptr %11), !!70564             ;L253<661<253<2893<50<2494<138<2897<252
 61885|  br i1 %66, label %79, label %67                                                                                       ;L2494<138<2897<252
 61886| 
 61887| 67: ; preds = %64, %60, %56
 61888|     ;; self = ptr undef
 61889|     ;; count = i64 1
 61890|     ;; ptr = !DIArgList(ptr %24, i64 32)
 61891|     ;; self = !DIArgList(ptr %24, i64 32)
 61892|     ;; end_or_len = ptr %24
 61893|  %68 = gep %24, i64 32                                                                                                 ;L656<185<2493<138<2897<252
 61894|     ;; ptr = ptr %68
 61895|     ;; x = ptr %68
 61896|  %69 = load ptr, ptr %68, , !!70564, !!8                                                                               ;L2494<138<2897<252
 61901|  %70 = icmp eq ptr %69, null                                                                                           ;L49<2494<138<2897<252
 61902|  br i1 %70, label %78, label %71                                                                                       ;L49<2494<138<2897<252
 61903| 
 61904| 71: ; preds = %67
 61905|     ;; x = ptr %69
 61908|     ;; x = ptr %69
 61910|     ;; c = ptr %69
 61912|     ;; self = ptr %69
 61913|  %72 = gep %69, i64 1216                                                                                               ;L742<253<2893<50<2494<138<2897<252
 61914|  %73 = load i32, ptr %72, , !!70564, !!8                                                                               ;L742<253<2893<50<2494<138<2897<252
 61915|  %74 = icmp eq i32 %73, -1                                                                                             ;L742<253<2893<50<2494<138<2897<252
 61916|  br i1 %74, label %78, label %75                                                                                       ;L742<253<2893<50<2494<138<2897<252
 61917| 
 61918| 75: ; preds = %71
 61919|  %76 = gep %69, i64 1168                                                                                               ;L742<253<2893<50<2494<138<2897<252
 61920|     ;; self = ptr %76
 61921|     ;; f[0..+8] = ptr %69
 61922|     ;; f[8..+8] = ptr %11
 61923|     ;; x = ptr %76
 61924|     ;; atk = ptr %76
 61926|  %77 = call zeroext i1 @gc::simulation6effectNtB2_6Effect11is_in_range(ptr %76, ptr %69, ptr %11), !!70564             ;L253<661<253<2893<50<2494<138<2897<252
 61927|  br i1 %77, label %79, label %78                                                                                       ;L2494<138<2897<252
 61928| 
 61929| 78: ; preds = %75, %71, %67
 61930|     ;; self = ptr undef
 61931|     ;; count = i64 1
 61932|     ;; ptr = !DIArgList(ptr %24, i64 40)
 61933|     ;; self = !DIArgList(ptr %24, i64 40)
 61934|     ;; end_or_len = ptr %24
 61935|  br label %79                                                                                                          ;L180<2493<138<2897<252
 61936| 
 61937| 79: ; preds = %78, %75, %64, %53, %42, %31, %19, %14, %7
 61938|  %80 = phi i1 [ true, %64 ], [ false, %14 ], [ false, %7 ], [ false, %78 ], [ true, %31 ], [ true, %42 ], [ true, %75 ], [ true, %53 ], [ true, %19 ] ;L0
 61939|  ret i1 %80                                                                                                            ;L255
 61940| }
