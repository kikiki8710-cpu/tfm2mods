 47101| define void @ai::plan_legacy8sub_plan9line_safeNtB2_15LineSafeSubPlan17action_candidates(ptr sret([32 x i8]) %0, ptr %1, i64 %2, ptr %3, ptr %4, ptr %5, ptr %6) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
 47102|  %8 = alloca [184 x i8],
 47103|  %9 = alloca [8 x i8],
 47104|  %10 = alloca [144 x i8],
 47105|  %11 = alloca [144 x i8],
 47106|  %12 = alloca [136 x i8],
 47107|  %13 = alloca [40 x i8],
 47108|  %14 = alloca [136 x i8],
 47109|  %15 = alloca [184 x i8],
 47110|  %16 = alloca [136 x i8],
 47111|  %17 = alloca [184 x i8],
 47112|  %18 = alloca [32 x i8],
 47113|  %19 = alloca [24 x i8],
 47114|  %20 = alloca [32 x i8],
 47115|  %21 = alloca [184 x i8],
 47116|  %22 = alloca [32 x i8],
 47117|  %23 = alloca [32 x i8],
 47118|  %24 = alloca [32 x i8],
 47119|  %25 = alloca [136 x i8],
 47120|  %26 = alloca [184 x i8],
 47121|  %27 = alloca [32 x i8],
 47122|  %28 = alloca [8 x i8],
 47123|  store i64 %2, ptr %28,
 47124|     ;; self = ptr %1
 47125|     ;; version = ptr %28
 47126|     ;; rnd = ptr %3
 47127|     ;; player = ptr %4
 47128|     ;; data = ptr %5
 47129|     ;; _parameter = ptr %6
 47130|     ;; res = ptr %27
 47132|  %29 = gep %5, i64 8                                                                                                   ;L90
 47133|  %30 = load ptr, ptr %29, , !!8, !!8                                                                                   ;L90
 47134|  %31 = load ptr, ptr %30, , !!8, !!8                                                                                   ;L90
 47135|     ;; bump = ptr %31
 47136|  store ptr inttoptr (i64 8 to ptr), ptr %27,                                                                           ;L547<90
 47137|  %32 = gep %27, i64 8                                                                                                  ;L547<90
 47138|  store ptr %31, ptr %32,                                                                                               ;L547<90
 47139|  %33 = gep %27, i64 16                                                                                                 ;L547<90
 47140|  %34 = gep %27, i64 24                                                                                                 ;L547<90
 47144|  call void @llvm.memset.p0.i64(ptr %33, i8 0, i64 16, i1 false)                                                        ;L547<90
 47147|     ;; self = ptr %1
 47148|     ;; version = i64 %2
 47149|     ;; rnd = ptr %3
 47150|     ;; player = ptr %4
 47151|     ;; data = ptr %5
 47152|     ;; res = ptr %18
 47153|     ;; self = ptr %16
 47154|     ;; self = ptr %14
 47156|     ;; purpose = i8 10
 47157|     ;; purpose = i8 10
 47159|     ;; bump = ptr %31
 47160|  store ptr inttoptr (i64 8 to ptr), ptr %18, , !!52243                                                                 ;L547<16<92
 47161|  %35 = gep %18, i64 8                                                                                                  ;L547<16<92
 47162|  store ptr %31, ptr %35, , !!52243                                                                                     ;L547<16<92
 47163|  %36 = gep %18, i64 16                                                                                                 ;L547<16<92
 47164|  %37 = gep %18, i64 24                                                                                                 ;L547<16<92
 47165|  call void @llvm.memset.p0.i64(ptr %36, i8 0, i64 16, i1 false), !!52243                                               ;L547<16<92
 47166|  %38 = load ptr, ptr %5, , !!52253, !!8, !!8                                                                           ;L18<92
 47167|  %39 = load i8, ptr %1, , !!52254, !!8                                                                                 ;L18<92
 47168|     ;; line = i8 %39
 47169|  %40 = gep %4, i64 2352                                                                                                ;L18<92
 47170|  %41 = load i64, ptr %40, , !!52267, !!8                                                                               ;L18<92
 47171|     ;; self = ptr %38
 47172|     ;; line = i8 %39
 47173|     ;; team = i64 %41
 47174|  %42 = icmp ult i64 %41, 2                                                                                             ;L0<18<92
 47175|  switch i8 %39, label %43 [
 47176|  i8 0, label %44
 47177|  i8 1, label %45
 47178|  i8 2, label %46
 47179|  ]                                                                                                                     ;L1823<18<92
 47180| 
 47181| 43: ; preds = %125, %7
 47182|  unreachable
 47183| 
 47184| 44: ; preds = %7
 47185|  br i1 %42, label %56, label %47                                                                                       ;L1824<18<92
 47186| 
 47187| 45: ; preds = %7
 47188|  br i1 %42, label %56, label %49                                                                                       ;L1825<18<92
 47189| 
 47190| 46: ; preds = %7
 47191|  br i1 %42, label %56, label %51                                                                                       ;L1826<18<92
 47192| 
 47193| 47: ; preds = %44
 47194|  invoke void @core::panicking18panic_bounds_check(i64 %41, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.243) #31
 47195|  to label %48 unwind label %53, !!52243                                                                                ;L1824<18<92
 47196| 
 47197| 48: ; preds = %47
 47198|  unreachable                                                                                                           ;L1824<18<92
 47199| 
 47200| 49: ; preds = %45
 47201|  invoke void @core::panicking18panic_bounds_check(i64 %41, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.244) #31
 47202|  to label %50 unwind label %53, !!52243                                                                                ;L1825<18<92
 47203| 
 47204| 50: ; preds = %49
 47205|  unreachable                                                                                                           ;L1825<18<92
 47206| 
 47207| 51: ; preds = %46
 47208|  invoke void @core::panicking18panic_bounds_check(i64 %41, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.245) #31
 47209|  to label %52 unwind label %53, !!52243                                                                                ;L1826<18<92
 47210| 
 47211| 52: ; preds = %51
 47212|  unreachable                                                                                                           ;L1826<18<92
 47213| 
 47214| 53: ; preds = %202, %193, %187, %148, %137, %124, %121, %85, %82, %51, %49, %47
 47215|  %54 = cleanuppad within none []
 47216|  invoke fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %18) #30 [ "funclet"(token %54) ]
 47217|  to label %55 unwind label %208                                                                                        ;L42<92
 47218| 
 47219| 55: ; preds = %53
 47220|  cleanupret from %54 unwind label %208
 47221| 
 47222| 56: ; preds = %46, %45, %44
 47223|  %57 = phi i64 [ 416, %45 ], [ 448, %46 ], [ 384, %44 ]
 47224|  %58 = phi i64 [ 432, %45 ], [ 464, %46 ], [ 400, %44 ]
 47225|  %59 = gep %38, i64 %57                                                                                                ;L0<18<92
 47226|  %60 = getelementptr ptr, ptr %59, i64 %41                                                                             ;L0<18<92
 47227|  %61 = load ptr, ptr %60, , !!52243, !!8                                                                               ;L0<18<92
 47228|  %62 = gep %38, i64 %58                                                                                                ;L0<18<92
 47229|  %63 = getelementptr ptr, ptr %62, i64 %41                                                                             ;L0<18<92
 47230|  %64 = load ptr, ptr %63, , !!52243, !!8                                                                               ;L0<18<92
 47231|  %65 = icmp eq ptr %61, null                                                                                           ;L1622<0<18<92
 47232|  %66 = select i1 %65, ptr %64, ptr %61                                                                                 ;L1622<0<18<92
 47233|     ;; optb = ptr %66
 47234|     ;; self = ptr %66
 47235|  %67 = gep %38, i64 304                                                                                                ;L19<92
 47236|  %68 = getelementptr { { ptr, ptr, i64 }, i64 }, ptr %67, i64 %41                                                      ;L19<92
 47237|     ;; self = ptr %68
 47238|     ;; self = ptr %68
 47239|  %69 = load ptr, ptr %68, , !!52243, !!8, !!8                                                                          ;L138<2073<19<92
 47240|     ;; p = ptr %69
 47241|  %70 = gep %68, i64 24                                                                                                 ;L2075<19<92
 47242|  %71 = load i64, ptr %70, , !!52243, !!8                                                                               ;L2075<19<92
 47243|     ;; len = i64 %71
 47244|     ;; count = i64 %71
 47245|     ;; self[0..+8] = ptr %69
 47246|     ;; slice[0..+8] = ptr %69
 47247|     ;; self[8..+8] = i64 %71
 47248|     ;; slice[8..+8] = i64 %71
 47249|     ;; ptr = ptr %69
 47250|     ;; self = ptr %69
 47251|  %72 = shl nuw nsw i64 %71, 3                                                                                          ;L961<100<1042<19<92
 47252|  %73 = gep %69, i64 %72                                                                                                ;L961<100<1042<19<92
 47253|  %74 = gep %30, i64 8                                                                                                  ;L20<92
 47254|  %75 = load ptr, ptr %74, , !!52243, !!8, !!8                                                                          ;L20<92
 47255|     ;; f[0..+8] = ptr %1
 47256|     ;; f[0..+8] = ptr %1
 47257|     ;; f[8..+8] = ptr %75
 47258|     ;; f[8..+8] = ptr %75
 47259|     ;; f[16..+8] = ptr %4
 47260|     ;; f[16..+8] = ptr %4
 47261|     ;; self[0..+8] = ptr %69
 47262|     ;; self[8..+8] = ptr %73
 47263|     ;; self = ptr %13
 47267|     ;; self[0..+8] = ptr %69
 47268|     ;; self[8..+8] = ptr %73
 47269|  %76 = gep %13, i64 8                                                                                                  ;L69<836<3387<20<92
 47270|  store ptr %73, ptr %76, , !!52421                                                                                     ;L69<836<3387<20<92
 47271|  %77 = gep %13, i64 16                                                                                                 ;L69<836<3387<20<92
 47272|  store ptr %1, ptr %77, , !!52243                                                                                      ;L69<836<3387<20<92
 47273|  %78 = gep %13, i64 24                                                                                                 ;L69<836<3387<20<92
 47274|  store ptr %75, ptr %78, , !!52243                                                                                     ;L69<836<3387<20<92
 47275|  %79 = gep %13, i64 32                                                                                                 ;L69<836<3387<20<92
 47276|  store ptr %4, ptr %79, , !!52243                                                                                      ;L69<836<3387<20<92
 47278|     ;; self = ptr %13
 47281|     ;; self = ptr %13
 47282|     ;; self = ptr %13
 47283|     ;; count = i64 1
 47284|     ;; ptr = ptr %69
 47285|     ;; self = ptr %69
 47286|     ;; end_or_len = ptr %73
 47289|  %80 = icmp eq i64 %71, 0                                                                                              ;L1714<180<107<2706<3416<3387<20<92
 47290|  br i1 %80, label %81, label %82                                                                                       ;L180<107<2706<3416<3387<20<92
 47291| 
 47292| 81: ; preds = %56
 47294|     ;; self = ptr null
 47295|  br label %111                                                                                                         ;L1161<24<92
 47296| 
 47297| 82: ; preds = %56
 47298|  %83 = gep %69, i64 8                                                                                                  ;L656<185<107<2706<3416<3387<20<92
 47299|  store ptr %83, ptr %13, , !!52409                                                                                     ;L185<107<2706<3416<3387<20<92
 47300|     ;; self = ptr %69
 47301|     ;; f = ptr %77
 47302|     ;; self = ptr %77
 47303|     ;; x = ptr %69
 47304|     ;; args = ptr %69
 47306|     ;; x = ptr %69
 47312|  %84 = invoke { i64, i64 } @gc::simulation5state6playerNtB5_8LineType18get_start_position(ptr %1, ptr %75, i64 %41)
 47313|  to label %85 unwind label %53, !!52529                                                                                ;L21<3379<310<1162<107<2706<3416<3387<20<92
 47314| 
 47315| 85: ; preds = %82
 47316|  %86 = extractvalue { i64, i64 } %84, 0                                                                                ;L21<3379<310<1162<107<2706<3416<3387<20<92
 47317|  %87 = extractvalue { i64, i64 } %84, 1                                                                                ;L21<3379<310<1162<107<2706<3416<3387<20<92
 47318|     ;; x = i64 %86
 47319|     ;; x2 = i64 %86
 47320|     ;; other = i64 %86
 47321|     ;; y = i64 %87
 47322|     ;; y2 = i64 %87
 47323|     ;; other = i64 %87
 47324|  %88 = load ptr, ptr %69, , !!52552, !!8, !!8                                                                          ;L22<3379<310<1162<107<2706<3416<3387<20<92
 47325|  %89 = gep %88, i64 1632                                                                                               ;L22<3379<310<1162<107<2706<3416<3387<20<92
 47326|  %90 = load i64, ptr %89, , !!52556, !!8                                                                               ;L22<3379<310<1162<107<2706<3416<3387<20<92
 47327|     ;; x1 = i64 %90
 47328|     ;; self = i64 %90
 47329|  %91 = gep %88, i64 1640                                                                                               ;L22<3379<310<1162<107<2706<3416<3387<20<92
 47330|  %92 = load i64, ptr %91, , !!52556, !!8                                                                               ;L22<3379<310<1162<107<2706<3416<3387<20<92
 47331|     ;; y1 = i64 %92
 47332|     ;; self = i64 %92
 47333|  %93 = icmp ult i64 %90, %86                                                                                           ;L3147<7<22<3379<310<1162<107<2706<3416<3387<20<92
 47334|  %94 = sub nuw i64 %86, %90                                                                                            ;L3147<7<22<3379<310<1162<107<2706<3416<3387<20<92
 47335|  %95 = sub nuw i64 %90, %86                                                                                            ;L3147<7<22<3379<310<1162<107<2706<3416<3387<20<92
 47336|  %96 = select i1 %93, i64 %94, i64 %95                                                                                 ;L3147<7<22<3379<310<1162<107<2706<3416<3387<20<92
 47337|     ;; dx = i64 %96
 47338|  %97 = icmp ult i64 %92, %87                                                                                           ;L3147<8<22<3379<310<1162<107<2706<3416<3387<20<92
 47339|  %98 = sub nuw i64 %87, %92                                                                                            ;L3147<8<22<3379<310<1162<107<2706<3416<3387<20<92
 47340|  %99 = sub nuw i64 %92, %87                                                                                            ;L3147<8<22<3379<310<1162<107<2706<3416<3387<20<92
 47341|  %100 = select i1 %97, i64 %98, i64 %99                                                                                ;L3147<8<22<3379<310<1162<107<2706<3416<3387<20<92
 47342|     ;; dy = i64 %100
 47343|  %101 = mul i64 %96, %96                                                                                               ;L9<22<3379<310<1162<107<2706<3416<3387<20<92
 47344|  %102 = mul i64 %100, %100                                                                                             ;L9<22<3379<310<1162<107<2706<3416<3387<20<92
 47345|  %103 = add i64 %102, %101                                                                                             ;L9<22<3379<310<1162<107<2706<3416<3387<20<92
 47346|     ;; first[0..+8] = i64 %103
 47347|     ;; first[8..+8] = ptr %69
 47348|  %104 = invoke { i64, ptr } @core::iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityENCINvNvNtNtNtBa_6traits8iterator8Iterator10min_by_key3keyRB1n_yNCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan9line_safeNtB3p_15LineSafeSubPlan16base_positioning0E0EB2q_4foldTyB3e_ENCINvNvB2q_6min_by4foldB5f_INvB2o_7compareB3e_yEE0EB3v_(ptr %13, i64 %103, ptr %69)
 47349|  to label %106 unwind label %53, !!52529                                                                               ;L2707<3416<3387<20<92
 47350| 
 47351| 105: ; preds = %124, %121
 47352|  unreachable
 47353| 
 47354| 106: ; preds = %85
 47355|  %107 = extractvalue { i64, ptr } %104, 1                                                                              ;L2707<3416<3387<20<92
 47357|     ;; self = ptr %107
 47358|  %108 = icmp eq ptr %107, null                                                                                         ;L1161<24<92
 47359|  br i1 %108, label %111, label %109                                                                                    ;L1161<24<92
 47360| 
 47361| 109: ; preds = %106
 47362|     ;; x = ptr %107
 47363|     ;; t = ptr %107
 47364|  %110 = load ptr, ptr %107, , !!52529, !!8, !!8                                                                        ;L24<1162<24<92
 47365|     ;; optb = ptr %110
 47366|     ;; self = ptr %110
 47367|     ;; self = ptr %110
 47368|  br label %111                                                                                                         ;L1165<24<92
 47369| 
 47370| 111: ; preds = %109, %106, %81
 47371|  %112 = phi ptr [ %110, %109 ], [ null, %106 ], [ null, %81 ]                                                          ;L0<24<92
 47372|     ;; self = ptr %112
 47373|     ;; self = ptr %112
 47374|     ;; optb = ptr %112
 47375|  %113 = icmp eq ptr %66, null                                                                                          ;L1622<19<92
 47376|  %114 = select i1 %113, ptr %112, ptr %66                                                                              ;L1622<19<92
 47377|     ;; self = ptr %114
 47378|     ;; self = ptr %114
 47379|     ;; optb = ptr %114
 47380|  %115 = gep %38, i64 368                                                                                               ;L25<92
 47381|  %116 = getelementptr ptr, ptr %115, i64 %41                                                                           ;L25<92
 47382|  %117 = load ptr, ptr %116, , !!52529, !!8                                                                             ;L25<92
 47383|     ;; optb = ptr %117
 47384|     ;; self = ptr %117
 47385|  %118 = icmp eq ptr %114, null                                                                                         ;L1622<25<92
 47386|  %119 = select i1 %118, ptr %117, ptr %114                                                                             ;L1622<25<92
 47387|     ;; self = ptr %119
 47388|     ;; self = ptr %119
 47389|     ;; optb = ptr %119
 47390|  %120 = icmp eq ptr %119, null                                                                                         ;L1011<26<92
 47391|  br i1 %120, label %121, label %122                                                                                    ;L1011<26<92
 47392| 
 47393| 121: ; preds = %111
 47394|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.218) #31
 47395|  to label %105 unwind label %53, !!52529                                                                               ;L1013<26<92
 47396| 
 47397| 122: ; preds = %111
 47398|     ;; nearest_tower = ptr %119
 47399|  %123 = icmp eq ptr %117, null                                                                                         ;L1011<28<92
 47400|  br i1 %123, label %124, label %125                                                                                    ;L1011<28<92
 47401| 
 47402| 124: ; preds = %122
 47403|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.219) #31
 47404|  to label %105 unwind label %53, !!52529                                                                               ;L1013<28<92
 47405| 
 47406| 125: ; preds = %122
 47407|     ;; nexus = ptr %117
 47408|  %126 = gep %5, i64 16                                                                                                 ;L30<92
 47409|  %127 = load ptr, ptr %126, , !!52253, !!8, !!8                                                                        ;L30<92
 47410|  %128 = getelementptr { { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, [5 x { i64, [2 x i64] }], [5 x { i64, { i8, [23 x i8] } }], [5 x { i8, [7 x i8], i64 }], [5 x i64], [5 x i64], [5 x i64], [5 x { i64, i64 }], i64, i64, i64, i64, i64, [5 x i32], i8, [3 x i8] }, ptr %127, i64 %41 ;L30<92
 47411|     ;; self = ptr %128
 47412|  switch i8 %39, label %43 [
 47413|  i8 0, label %133
 47414|  i8 1, label %129
 47415|  i8 2, label %131
 47416|  ]                                                                                                                     ;L379<30<92
 47417| 
 47418| 129: ; preds = %125
 47419|  %130 = gep %128, i64 40                                                                                               ;L381<30<92
 47420|  br label %133                                                                                                         ;L381<30<92
 47421| 
 47422| 131: ; preds = %125
 47423|  %132 = gep %128, i64 80                                                                                               ;L382<30<92
 47424|  br label %133                                                                                                         ;L382<30<92
 47425| 
 47426| 133: ; preds = %131, %129, %125
 47427|  %134 = phi ptr [ %132, %131 ], [ %130, %129 ], [ %128, %125 ]                                                         ;L0<30<92
 47428|  %135 = load i64, ptr %134, , !!52529, !!8                                                                             ;L30<92
 47429|     ;; self[0..+8] = i64 %135
 47433|  %136 = trunc nuw i64 %135 to i1                                                                                       ;L1542<31<92
 47434|  br i1 %136, label %137, label %148                                                                                    ;L1542<31<92
 47435| 
 47436| 137: ; preds = %133
 47437|  %138 = gep %38, i64 8                                                                                                 ;L31<92
 47438|  %139 = load ptr, ptr %138, , !!52529, !!8, !!8                                                                        ;L31<92
 47439|     ;; f[8..+8] = ptr %139
 47440|  %140 = load ptr, ptr %38, , !!52529, !!8, !!8                                                                         ;L31<92
 47441|     ;; f[0..+8] = ptr %140
 47442|  %141 = gep %134, i64 8                                                                                                ;L30<92
 47443|  %142 = load i64, ptr %141, , !!52529                                                                                  ;L30<92
 47444|     ;; self[8..+8] = i64 %142
 47445|     ;; x = i64 %142
 47446|     ;; m = i64 %142
 47447|  %143 = gep %139, i64 496                                                                                              ;L31<1543<31<92
 47448|  %144 = load ptr, ptr %143, , !!52529, !!8                                                                             ;L31<1543<31<92
 47449|  %145 = invoke ptr %144(ptr %140, i64 %142)
 47450|  to label %146 unwind label %53, !!52529                                                                               ;L31<1543<31<92
 47451| 
 47452| 146: ; preds = %137
 47453|     ;; self = ptr %145
 47454|     ;; front_minion = ptr %145
 47455|     ;; self = ptr %145
 47456|     ;; f[0..+8] = ptr %117
 47457|     ;; f[8..+8] = ptr %119
 47458|  %147 = icmp eq ptr %145, null                                                                                         ;L708<33<92
 47459|  br i1 %147, label %148, label %151                                                                                    ;L708<33<92
 47460| 
 47461| 148: ; preds = %151, %146, %133
 47463|  %149 = gep %119, i64 1472                                                                                             ;L34<92
 47464|  %150 = load i64, ptr %149, , !!52529, !!8                                                                             ;L34<92
 47465|  invoke void @ai::small_action6aroundNtB2_17SmallActionAround3new(ptr sret([136 x i8]) %16, i64 %2, ptr %3, ptr %5, ptr %4, i64 %150, i64 5)
 47466|  to label %199 unwind label %53, !!52652                                                                               ;L34<92
 47467| 
 47468| 151: ; preds = %146
 47469|     ;; x = ptr %145
 47470|  %152 = gep %117, i64 1632                                                                                             ;L710<33<92
 47471|  %153 = load i64, ptr %152, , !!52529, !!8                                                                             ;L710<33<92
 47472|  %154 = gep %117, i64 1640                                                                                             ;L710<33<92
 47473|  %155 = load i64, ptr %154, , !!52529, !!8                                                                             ;L710<33<92
 47474|  %156 = gep %119, i64 1632                                                                                             ;L710<33<92
 47475|  %157 = load i64, ptr %156, , !!52529, !!8                                                                             ;L710<33<92
 47476|  %158 = gep %119, i64 1640                                                                                             ;L710<33<92
 47477|  %159 = load i64, ptr %158, , !!52529, !!8                                                                             ;L710<33<92
 47478|  %160 = gep %145, i64 1632                                                                                             ;L710<33<92
 47479|  %161 = load i64, ptr %160, , !!52529, !!8                                                                             ;L710<33<92
 47480|  %162 = gep %145, i64 1640                                                                                             ;L710<33<92
 47481|  %163 = load i64, ptr %162, , !!52529, !!8                                                                             ;L710<33<92
 47488|     ;; x1 = i64 %161
 47489|     ;; self = i64 %161
 47490|     ;; y1 = i64 %163
 47491|     ;; self = i64 %163
 47492|     ;; x2 = i64 %153
 47493|     ;; other = i64 %153
 47494|     ;; x2 = i64 %153
 47495|     ;; other = i64 %153
 47496|     ;; y2 = i64 %155
 47497|     ;; other = i64 %155
 47498|     ;; y2 = i64 %155
 47499|     ;; other = i64 %155
 47500|  %164 = icmp ult i64 %161, %153                                                                                        ;L3147<7<2158<33<710<33<92
 47501|  %165 = sub nuw i64 %153, %161                                                                                         ;L3147<7<2158<33<710<33<92
 47502|  %166 = sub nuw i64 %161, %153                                                                                         ;L3147<7<2158<33<710<33<92
 47503|  %167 = select i1 %164, i64 %165, i64 %166                                                                             ;L3147<7<2158<33<710<33<92
 47504|     ;; dx = i64 %167
 47505|  %168 = icmp ult i64 %163, %155                                                                                        ;L3147<8<2158<33<710<33<92
 47506|  %169 = sub nuw i64 %155, %163                                                                                         ;L3147<8<2158<33<710<33<92
 47507|  %170 = sub nuw i64 %163, %155                                                                                         ;L3147<8<2158<33<710<33<92
 47508|  %171 = select i1 %168, i64 %169, i64 %170                                                                             ;L3147<8<2158<33<710<33<92
 47509|     ;; dy = i64 %171
 47510|     ;; x1 = i64 %157
 47511|     ;; self = i64 %157
 47512|     ;; y1 = i64 %159
 47513|     ;; self = i64 %159
 47514|  %172 = icmp ult i64 %157, %153                                                                                        ;L3147<7<2158<33<710<33<92
 47515|  %173 = sub nuw i64 %153, %157                                                                                         ;L3147<7<2158<33<710<33<92
 47516|  %174 = sub nuw i64 %157, %153                                                                                         ;L3147<7<2158<33<710<33<92
 47517|  %175 = select i1 %172, i64 %173, i64 %174                                                                             ;L3147<7<2158<33<710<33<92
 47518|     ;; dx = i64 %175
 47519|  %176 = icmp ult i64 %159, %155                                                                                        ;L3147<8<2158<33<710<33<92
 47520|  %177 = sub nuw i64 %155, %159                                                                                         ;L3147<8<2158<33<710<33<92
 47521|  %178 = sub nuw i64 %159, %155                                                                                         ;L3147<8<2158<33<710<33<92
 47522|  %179 = select i1 %176, i64 %177, i64 %178                                                                             ;L3147<8<2158<33<710<33<92
 47523|  %180 = mul i64 %167, %167                                                                                             ;L9<2158<33<710<33<92
 47524|  %181 = mul i64 %171, %171                                                                                             ;L9<2158<33<710<33<92
 47525|  %182 = add i64 %181, %180                                                                                             ;L9<2158<33<710<33<92
 47526|     ;; dy = i64 %179
 47527|  %183 = mul i64 %175, %175                                                                                             ;L9<2158<33<710<33<92
 47528|  %184 = mul i64 %179, %179                                                                                             ;L9<2158<33<710<33<92
 47529|  %185 = add i64 %184, %183                                                                                             ;L9<2158<33<710<33<92
 47530|  %186 = icmp ult i64 %182, %185                                                                                        ;L33<710<33<92
 47531|  br i1 %186, label %148, label %187                                                                                    ;L33<92
 47532| 
 47533| 187: ; preds = %151
 47535|  %188 = gep %145, i64 1472                                                                                             ;L37<92
 47536|  %189 = load i64, ptr %188, , !!52529, !!8                                                                             ;L37<92
 47537|  invoke void @ai::small_action6aroundNtB2_17SmallActionAround3new(ptr sret([136 x i8]) %14, i64 %2, ptr %3, ptr %5, ptr %4, i64 %189, i64 5)
 47538|  to label %190 unwind label %53, !!52652                                                                               ;L37<92
 47539| 
 47540| 190: ; preds = %187
 47541|  %191 = gep %14, i64 128                                                                                               ;L57<38<92
 47542|  store i8 10, ptr %191, , !!52243                                                                                      ;L57<38<92
 47543|  call void @llvm.memcpy.p0.p0.i64(ptr %15, ptr %14, i64 136, i1 false), !!52243                                        ;L37<92
 47544|  %192 = gep %15, i64 177                                                                                               ;L37<92
 47545|  store i8 5, ptr %192, , !!52243                                                                                       ;L37<92
 47547|     ;; self = ptr %18
 47548|     ;; self = ptr %18
 47549|     ;; value = ptr %15
 47550|     ;; src = ptr %15
 47551|     ;; additional = i64 1
 47552|     ;; needed_extra_cap = i64 1
 47553|     ;; needed_extra_cap = i64 1
 47554|     ;; strategy = i8 1
 47555|     ;; self = ptr %18
 47556|     ;; self = ptr %18
 47557|     ;; used_cap = i64 0
 47558|     ;; used_cap = i64 0
 47559|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %18, i64 0, i64 1, i1 zeroext true)
 47560|  to label %195 unwind label %193, !!52732                                                                              ;L619<430<738<1429<37<92
 47561| 
 47562| 193: ; preds = %190
 47563|  %194 = cleanuppad within none []
 47564|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %15) #30 [ "funclet"(token %194) ], !!52735 ;L1436<37<92
 47565|  cleanupret from %194 unwind label %53
 47566| 
 47567| 195: ; preds = %190
 47568|  %196 = load ptr, ptr %18, , !!52738                                                                                   ;L138<1432<37<92
 47569|  %197 = load i64, ptr %37, , !!52738                                                                                   ;L1432<37<92
 47570|     ;; self = ptr %18
 47571|     ;; self = ptr %196
 47572|     ;; count = i64 %197
 47573|  %198 = gepS %196, i64 %197                                                                                            ;L961<1432<37<92
 47574|     ;; end = ptr %198
 47575|     ;; dst = ptr %198
 47576|  call void @llvm.memcpy.p0.p0.i64(ptr %198, ptr %15, i64 184, i1 false), !!52735                                       ;L1933<1433<37<92
 47578|  br label %210                                                                                                         ;L33<92
 47579| 
 47580| 199: ; preds = %148
 47581|  %200 = gep %16, i64 128                                                                                               ;L57<35<92
 47582|  store i8 10, ptr %200, , !!52243                                                                                      ;L57<35<92
 47583|  call void @llvm.memcpy.p0.p0.i64(ptr %17, ptr %16, i64 136, i1 false), !!52243                                        ;L34<92
 47584|  %201 = gep %17, i64 177                                                                                               ;L34<92
 47585|  store i8 5, ptr %201, , !!52243                                                                                       ;L34<92
 47587|     ;; self = ptr %18
 47588|     ;; self = ptr %18
 47589|     ;; value = ptr %17
 47590|     ;; src = ptr %17
 47591|     ;; additional = i64 1
 47592|     ;; needed_extra_cap = i64 1
 47593|     ;; needed_extra_cap = i64 1
 47594|     ;; strategy = i8 1
 47595|     ;; self = ptr %18
 47596|     ;; self = ptr %18
 47597|     ;; used_cap = i64 0
 47598|     ;; used_cap = i64 0
 47599|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %18, i64 0, i64 1, i1 zeroext true)
 47600|  to label %204 unwind label %202, !!52763                                                                              ;L619<430<738<1429<34<92
 47601| 
 47602| 202: ; preds = %199
 47603|  %203 = cleanuppad within none []
 47604|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %17) #30 [ "funclet"(token %203) ], !!52766 ;L1436<34<92
 47605|  cleanupret from %203 unwind label %53
 47606| 
 47607| 204: ; preds = %199
 47608|  %205 = load ptr, ptr %18, , !!52769                                                                                   ;L138<1432<34<92
 47609|  %206 = load i64, ptr %37, , !!52769                                                                                   ;L1432<34<92
 47610|     ;; self = ptr %18
 47611|     ;; self = ptr %205
 47612|     ;; count = i64 %206
 47613|  %207 = gepS %205, i64 %206                                                                                            ;L961<1432<34<92
 47614|     ;; end = ptr %207
 47615|     ;; dst = ptr %207
 47616|  call void @llvm.memcpy.p0.p0.i64(ptr %207, ptr %17, i64 184, i1 false), !!52766                                       ;L1933<1433<34<92
 47618|  br label %210                                                                                                         ;L33<92
 47619| 
 47620| 208: ; preds = %465, %461, %460, %458, %428, %425, %353, %321, %297, %254, %241, %240, %236, %234, %230, %225, %223, %214, %210, %55, %53
 47621|  %209 = cleanuppad within none []
 47622|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %27) #30 [ "funclet"(token %209) ] ;L102
 47623|  cleanupret from %209 unwind to caller                                                                                 ;L89
 47624| 
 47625| 210: ; preds = %204, %195
 47626|  %211 = phi i64 [ %206, %204 ], [ %197, %195 ]
 47627|  %212 = phi ptr [ %205, %204 ], [ %196, %195 ]                                                                         ;L41<92
 47628|  %213 = add i64 %211, 1                                                                                                ;L1434<0<92
 47632|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %27, ptr %212, i64 %213)
 47633|  to label %214 unwind label %208                                                                                       ;L92
 47634| 
 47635| 214: ; preds = %210
 47638|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway3new(ptr sret([136 x i8]) %25, ptr %5, ptr %4, i64 5)
 47639|  to label %215 unwind label %208                                                                                       ;L93
 47640| 
 47641| 215: ; preds = %214
 47642|  call void @llvm.memcpy.p0.p0.i64(ptr %26, ptr %25, i64 136, i1 false)                                                 ;L93
 47643|  %216 = gep %26, i64 177                                                                                               ;L93
 47644|  store i8 3, ptr %216,                                                                                                 ;L93
 47647|     ;; self = ptr %27
 47648|     ;; self = ptr %27
 47649|     ;; value = ptr %26
 47650|     ;; src = ptr %26
 47651|     ;; additional = i64 1
 47652|     ;; needed_extra_cap = i64 1
 47653|     ;; needed_extra_cap = i64 1
 47654|     ;; strategy = i8 1
 47655|  %217 = load i64, ptr %34, , !!52797, !!8                                                                              ;L1428<93
 47656|     ;; self = ptr %27
 47657|  %218 = load i64, ptr %33, , !!52797, !!8                                                                              ;L149<1428<93
 47658|  %219 = icmp eq i64 %217, %218                                                                                         ;L1428<93
 47659|  br i1 %219, label %220, label %225                                                                                    ;L1428<93
 47660| 
 47661| 220: ; preds = %215
 47662|     ;; self = ptr %27
 47663|     ;; self = ptr %27
 47664|     ;; self = ptr %27
 47665|     ;; used_cap = i64 %217
 47666|     ;; used_cap = i64 %217
 47667|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %27, i64 %217, i64 1, i1 zeroext true)
 47668|  to label %221 unwind label %223, !!52797                                                                              ;L619<430<738<1429<93
 47669| 
 47670| 221: ; preds = %220
 47671|  %222 = load i64, ptr %34, , !!52797                                                                                   ;L1432<93
 47672|  br label %225                                                                                                         ;L619<430<738<1429<93
 47673| 
 47674| 223: ; preds = %220
 47675|  %224 = cleanuppad within none []
 47676|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %26) #30 [ "funclet"(token %224) ], !!52782 ;L1436<93
 47677|  cleanupret from %224 unwind label %208
 47678| 
 47679| 225: ; preds = %221, %215
 47680|  %226 = phi i64 [ %222, %221 ], [ %217, %215 ]                                                                         ;L1432<93
 47681|     ;; self = ptr %27
 47682|  %227 = load ptr, ptr %27, , !!52797, !!8, !!8                                                                         ;L138<1432<93
 47683|     ;; self = ptr %227
 47684|     ;; count = i64 %226
 47685|  %228 = gepS %227, i64 %226                                                                                            ;L961<1432<93
 47686|     ;; end = ptr %228
 47687|     ;; dst = ptr %228
 47688|  call void @llvm.memcpy.p0.p0.i64(ptr %228, ptr %26, i64 184, i1 false), !!52782                                       ;L1933<1433<93
 47689|  %229 = add i64 %226, 1                                                                                                ;L1434<93
 47690|  store i64 %229, ptr %34, , !!52797                                                                                    ;L1434<93
 47693|  invoke void @ai::fight_check13battle_action(ptr sret([32 x i8]) %24, i64 %2, ptr %3, ptr %4, ptr %5, i64 5)
 47694|  to label %230 unwind label %208                                                                                       ;L94
 47695| 
 47696| 230: ; preds = %225
 47697|  %231 = load ptr, ptr %24, , !!8, !!8                                                                                  ;L94
 47698|  %232 = gep %24, i64 24                                                                                                ;L94
 47699|  %233 = load i64, ptr %232, , !!8                                                                                      ;L94
 47700|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %27, ptr %231, i64 %233)
 47701|  to label %234 unwind label %208                                                                                       ;L94
 47702| 
 47703| 234: ; preds = %230
 47706|  %235 = load i8, ptr %1, , !!8                                                                                         ;L95
 47708|     ;; version = i64 %2
 47710|     ;; player = ptr %4
 47711|     ;; data = ptr %5
 47712|  invoke void @ai::small_action11lane_minion29line_minion_action_candidates(ptr sret([32 x i8]) %23, i64 %2, ptr %5, ptr %4, i8 %235)
 47713|  to label %236 unwind label %208                                                                                       ;L45<95
 47714| 
 47715| 236: ; preds = %234
 47716|  %237 = load ptr, ptr %23, , !!8, !!8                                                                                  ;L95
 47717|  %238 = gep %23, i64 24                                                                                                ;L95
 47718|  %239 = load i64, ptr %238, , !!8                                                                                      ;L95
 47719|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %27, ptr %237, i64 %239)
 47720|  to label %240 unwind label %208                                                                                       ;L95
 47721| 
 47722| 240: ; preds = %236
 47725|  invoke void @ai::fight_check20attack_summon_action(ptr sret([32 x i8]) %22, ptr %4, ptr %5)
 47726|  to label %241 unwind label %208                                                                                       ;L96
 47727| 
 47728| 241: ; preds = %240
 47729|  %242 = load ptr, ptr %22, , !!8, !!8                                                                                  ;L96
 47730|  %243 = gep %22, i64 24                                                                                                ;L96
 47731|  %244 = load i64, ptr %243, , !!8                                                                                      ;L96
 47732|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %27, ptr %242, i64 %244)
 47733|  to label %245 unwind label %208                                                                                       ;L96
 47734| 
 47735| 245: ; preds = %241
 47741|     ;; version = i64 %2
 47743|     ;; player = ptr %4
 47744|     ;; data = ptr %5
 47748|     ;; self = ptr %4
 47749|  %246 = gep %4, i64 2496                                                                                               ;L581<49<97
 47750|  %247 = load i32, ptr %246, , !!52885, !!8                                                                             ;L581<49<97
 47751|  %248 = zext nneg i32 %247 to i64                                                                                      ;L581<49<97
 47752|  %249 = gep %38, i64 480                                                                                               ;L49<97
 47753|  %250 = getelementptr [5 x ptr], ptr %249, i64 %41                                                                     ;L49<97
 47754|  %251 = getelementptr ptr, ptr %250, i64 %248                                                                          ;L49<97
 47755|  %252 = load ptr, ptr %251, , !!52888, !!8                                                                             ;L49<97
 47756|     ;; self = ptr %252
 47757|  %253 = icmp eq ptr %252, null                                                                                         ;L2775<49<97
 47758|  br i1 %253, label %440, label %254                                                                                    ;L2775<49<97
 47759| 
 47760| 254: ; preds = %245
 47761|     ;; champ = ptr %252
 47762|     ;; other = ptr %252
 47763|     ;; caster = ptr %252
 47764|     ;; self = ptr %252
 47766|  %255 = sub nuw nsw i64 1, %41                                                                                         ;L50<97
 47767|  invoke void @gc::simulationNtB5_21AbstractGameWithCache11iter_towers(ptr sret([136 x i8]) %12, ptr %38, i64 %255)
 47768|  to label %256 unwind label %208                                                                                       ;L50<97
 47769| 
 47770| 256: ; preds = %254
 47771|     ;; self = ptr %12
 47772|     ;; f = ptr %252
 47773|     ;; self = ptr %11
 47777|     ;; self = ptr %12
 47778|     ;; f = ptr %252
 47779|  call void @llvm.memcpy.p0.p0.i64(ptr %11, ptr %12, i64 136, i1 false), !!53001                                        ;L69<836<3387<52<97
 47780|  %257 = gep %11, i64 136                                                                                               ;L69<836<3387<52<97
 47781|  store ptr %252, ptr %257, , !!53004                                                                                   ;L69<836<3387<52<97
 47783|     ;; self = ptr %11
 47786|     ;; self = ptr %11
 47788|     ;; self = ptr %11
 47791|  store ptr %257, ptr %9, , !!53054
 47792|     ;; self = ptr %11
 47793|     ;; predicate = ptr %9
 47794|  %258 = gep %11, i64 16                                                                                                ;L169<98<107<2706<3416<3387<52<97
 47796|     ;; opt = ptr %258
 47797|     ;; self = ptr %258
 47798|     ;; f = ptr %9
 47799|  %259 = load i64, ptr %258, , !!53107, !!8                                                                             ;L764<332<169<98<107<2706<3416<3387<52<97
 47800|  %260 = icmp eq i64 %259, -2                                                                                           ;L764<332<169<98<107<2706<3416<3387<52<97
 47801|  br i1 %260, label %304, label %261                                                                                    ;L764<332<169<98<107<2706<3416<3387<52<97
 47802| 
 47803| 261: ; preds = %256
 47805|     ;; predicate = ptr %9
 47806|     ;; a = ptr %258
 47808|     ;; predicate = ptr %9
 47809|     ;; self = ptr %258
 47811|     ;; opt = ptr %258
 47812|     ;; self = ptr %258
 47814|  %262 = icmp eq i64 %259, -1                                                                                           ;L764<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47815|  br i1 %262, label %293, label %263                                                                                    ;L764<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47816| 
 47817| 263: ; preds = %261
 47820|     ;; a = ptr %258
 47822|     ;; self = ptr %258
 47825|     ;; self = ptr %258
 47829|     ;; self = ptr %258
 47833|     ;; self = ptr %258
 47836|     ;; self = ptr %258
 47839|  %264 = trunc nuw i64 %259 to i1                                                                                       ;L396<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47840|  br i1 %264, label %265, label %291                                                                                    ;L396<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47841| 
 47842| 265: ; preds = %263
 47843|  %266 = gep %11, i64 24                                                                                                ;L396<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47844|     ;; iter = ptr %266
 47846|     ;; self = ptr %266
 47851|     ;; self[0..+8] = ptr %266
 47852|     ;; self[8..+8] = i64 6
 47853|  %267 = gep %11, i64 40                                                                                                ;L214<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47854|     ;; data[0..+8] = ptr %267
 47855|     ;; data[8..+8] = i64 6
 47856|     ;; f[0..+8] = ptr %267
 47857|     ;; f[8..+8] = i64 6
 47861|     ;; self = ptr %266
 47862|     ;; self = ptr %266
 47863|     ;; self = ptr %266
 47865|     ;; rhs = i64 1
 47866|  %268 = load i64, ptr %266, , !!53351, !!8                                                                             ;L122<166<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47867|  %269 = gep %11, i64 32                                                                                                ;L122<166<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47868|  %270 = load i64, ptr %269, , !!53351, !!8                                                                             ;L122<166<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47869|  %271 = icmp ule i64 %268, %270                                                                                        ;L122<166<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47870|     ;; cond = i1 true
 47871|  call void @llvm.assume(i1 %271)                                                                                       ;L210<122<166<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47872|  %272 = icmp eq i64 %268, %270                                                                                         ;L167<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47873|  br i1 %272, label %291, label %273                                                                                    ;L167<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47874| 
 47875| 273: ; preds = %288, %265
 47876|  %274 = phi i64 [ %275, %288 ], [ %268, %265 ]
 47877|     ;; i = i64 %274
 47878|     ;; value = i64 %274
 47879|     ;; self = i64 %274
 47880|  %275 = add nuw nsw i64 %274, 1                                                                                        ;L971<63<169<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47882|     ;; f = ptr undef
 47884|     ;; idx = i64 %274
 47885|     ;; index = i64 %274
 47886|     ;; self = i64 %274
 47887|     ;; self[0..+8] = ptr %267
 47888|     ;; slice[0..+8] = ptr %267
 47889|     ;; self[8..+8] = i64 6
 47890|     ;; slice[8..+8] = i64 6
 47891|  %276 = icmp ult i64 %274, 6                                                                                           ;L252<646<219<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47892|  call void @llvm.assume(i1 %276)                                                                                       ;L252<646<219<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47893|  %277 = getelementptr ptr, ptr %267, i64 %274                                                                          ;L253<646<219<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47894|     ;; self = ptr %277
 47895|     ;; self = ptr %277
 47896|     ;; src = ptr %277
 47897|  %278 = load ptr, ptr %277, , !!53396, !!8                                                                             ;L1733<1171<798<219<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47898|     ;; elem = ptr %278
 47901|     ;; inner = ptr %278
 47902|  %279 = icmp eq ptr %278, null                                                                                         ;L819<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47903|  br i1 %279, label %288, label %280                                                                                    ;L819<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47904| 
 47905| 280: ; preds = %273
 47906|     ;; item = ptr %278
 47907|     ;; x = ptr %278
 47918|     ;; self = ptr %278
 47919|  %281 = gep %278, i64 1721                                                                                             ;L1478<51<298<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47920|  %282 = load i8, ptr %281, , !!53479, !!8                                                                              ;L1478<51<298<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47921|  %283 = trunc nuw i8 %282 to i1                                                                                        ;L1478<51<298<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47922|  %284 = gep %278, i64 1696                                                                                             ;L1478<51<298<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47923|  %285 = load i64, ptr %284, , !!53482                                                                                  ;L1478<51<298<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47924|  %286 = icmp eq i64 %285, 0                                                                                            ;L1478<51<298<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47925|  %287 = select i1 %283, i1 %286, i1 false                                                                              ;L1478<51<298<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47926|  br i1 %287, label %292, label %288                                                                                    ;L820<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47927| 
 47928| 288: ; preds = %280, %273
 47929|  %289 = icmp eq i64 %275, %270                                                                                         ;L167<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47930|  br i1 %289, label %290, label %273                                                                                    ;L167<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47931| 
 47932| 290: ; preds = %288
 47933|  store i64 %270, ptr %266, , !!53351                                                                                   ;L63<169<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47934|     ;; x = ptr null
 47935|  br label %291                                                                                                         ;L333<169<169<332<169<98<107<2706<3416<3387<52<97
 47936| 
 47937| 291: ; preds = %290, %265, %263
 47938|  store i64 -1, ptr %258, , !!53489                                                                                     ;L334<169<169<332<169<98<107<2706<3416<3387<52<97
 47939|  br label %293                                                                                                         ;L333<169<169<332<169<98<107<2706<3416<3387<52<97
 47940| 
 47941| 292: ; preds = %280
 47942|  store i64 %275, ptr %266, , !!53351                                                                                   ;L63<169<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<52<97
 47943|     ;; x = ptr %278
 47944|     ;; self = ptr %278
 47945|     ;; f[0..+8] = ptr %258
 47946|     ;; f[8..+8] = ptr %9
 47947|     ;; x = ptr %278
 47950|  br label %302                                                                                                         ;L333<169<98<107<2706<3416<3387<52<97
 47951| 
 47952| 293: ; preds = %291, %261
 47953|  %294 = gep %11, i64 120                                                                                               ;L170<169<332<169<98<107<2706<3416<3387<52<97
 47954|     ;; self = ptr null
 47955|     ;; f[0..+8] = ptr %294
 47956|     ;; f[8..+8] = ptr %9
 47960|     ;; self = ptr %294
 47961|  %295 = load ptr, ptr %294, , !!53545, !!8                                                                             ;L764<170<1653<170<169<332<169<98<107<2706<3416<3387<52<97
 47962|  %296 = icmp eq ptr %295, null                                                                                         ;L764<170<1653<170<169<332<169<98<107<2706<3416<3387<52<97
 47963|  br i1 %296, label %301, label %297                                                                                    ;L764<170<1653<170<169<332<169<98<107<2706<3416<3387<52<97
 47964| 
 47965| 297: ; preds = %293
 47966|     ;; self = ptr %294
 47967|     ;; predicate = ptr %9
 47968|  %298 = invoke ptr @core::iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2p_4find5checkB1s_QQNCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan9line_safeNtB3E_15LineSafeSubPlan19attack_tower_action0E0INtNtNtBb_3ops12control_flow11ControlFlowB1s_EEB3K_(ptr %294, ptr %9)
 47969|  to label %299 unwind label %208                                                                                       ;L2971<170<1653<170<169<332<169<98<107<2706<3416<3387<52<97
 47970| 
 47971| 299: ; preds = %297
 47972|     ;; x = ptr %298
 47975|  %300 = icmp eq ptr %298, null                                                                                         ;L633<682<333<169<98<107<2706<3416<3387<52<97
 47976|  br i1 %300, label %301, label %302                                                                                    ;L333<169<98<107<2706<3416<3387<52<97
 47977| 
 47978| 301: ; preds = %299, %293
 47979|  store i64 -2, ptr %258, , !!53107                                                                                     ;L334<169<98<107<2706<3416<3387<52<97
 47980|  br label %304                                                                                                         ;L333<169<98<107<2706<3416<3387<52<97
 47981| 
 47982| 302: ; preds = %299, %292
 47983|  %303 = phi ptr [ %298, %299 ], [ %278, %292 ]                                                                         ;L0<169<98<107<2706<3416<3387<52<97
 47985|     ;; self = ptr %303
 47986|     ;; f[0..+8] = ptr %11
 47988|     ;; self = ptr %303
 47989|     ;; f = ptr %11
 47990|     ;; self = ptr %11
 47991|  br label %321                                                                                                         ;L1161<107<2706<3416<3387<52<97
 47992| 
 47993| 304: ; preds = %301, %256
 47994|     ;; self = ptr null
 47995|     ;; f[0..+8] = ptr %11
 48001|     ;; self = ptr %11
 48002|  %305 = load i64, ptr %11, , !!53635, !!8                                                                              ;L764<170<1653<170<98<107<2706<3416<3387<52<97
 48003|  %306 = trunc nuw i64 %305 to i1                                                                                       ;L764<170<1653<170<98<107<2706<3416<3387<52<97
 48004|  br i1 %306, label %307, label %320                                                                                    ;L764<170<1653<170<98<107<2706<3416<3387<52<97
 48005| 
 48006| 307: ; preds = %304
 48007|  %308 = gep %11, i64 8                                                                                                 ;L764<170<1653<170<98<107<2706<3416<3387<52<97
 48009|     ;; self = ptr %308
 48013|     ;; self = ptr %308
 48016|  %309 = load ptr, ptr %308, , !!53635
 48017|     ;; self = ptr %308
 48018|  %310 = icmp eq ptr %309, null                                                                                         ;L2493<2971<170<1653<170<98<107<2706<3416<3387<52<97
 48019|  br i1 %310, label %320, label %311                                                                                    ;L2493<2971<170<1653<170<98<107<2706<3416<3387<52<97
 48020| 
 48021| 311: ; preds = %307
 48022|     ;; x = ptr %309
 48023|     ;; x = ptr %309
 48030|     ;; self = ptr %309
 48031|  %312 = gep %309, i64 1721                                                                                             ;L1478<51<298<2967<2494<2971<170<1653<170<98<107<2706<3416<3387<52<97
 48032|  %313 = load i8, ptr %312, , !!53715, !!8                                                                              ;L1478<51<298<2967<2494<2971<170<1653<170<98<107<2706<3416<3387<52<97
 48033|  %314 = trunc nuw i8 %313 to i1                                                                                        ;L1478<51<298<2967<2494<2971<170<1653<170<98<107<2706<3416<3387<52<97
 48034|  %315 = gep %309, i64 1696                                                                                             ;L1478<51<298<2967<2494<2971<170<1653<170<98<107<2706<3416<3387<52<97
 48035|  %316 = load i64, ptr %315, , !!53715                                                                                  ;L1478<51<298<2967<2494<2971<170<1653<170<98<107<2706<3416<3387<52<97
 48036|  %317 = icmp eq i64 %316, 0                                                                                            ;L1478<51<298<2967<2494<2971<170<1653<170<98<107<2706<3416<3387<52<97
 48037|  %318 = select i1 %314, i1 %317, i1 false                                                                              ;L2494<2971<170<1653<170<98<107<2706<3416<3387<52<97
 48038|  br i1 %318, label %319, label %320                                                                                    ;L2494<2971<170<1653<170<98<107<2706<3416<3387<52<97
 48039| 
 48040| 319: ; preds = %311
 48041|  store ptr null, ptr %308, , !!53635                                                                                   ;L0<1898<2494<2629<2493<2971<170<1653<170<98<107<2706<3416<3387<52<97
 48042|     ;; self = ptr %309
 48043|     ;; f = ptr %11
 48044|     ;; self = ptr %11
 48045|  br label %321                                                                                                         ;L1161<107<2706<3416<3387<52<97
 48046| 
 48047| 320: ; preds = %311, %307, %304
 48050|     ;; self = ptr null
 48052|  br label %440                                                                                                         ;L2775<50<97
 48053| 
 48054| 321: ; preds = %319, %302
 48055|  %322 = phi ptr [ %303, %302 ], [ %309, %319 ]
 48057|     ;; f = ptr %257
 48058|     ;; self = ptr %257
 48059|     ;; x = ptr %322
 48060|     ;; args = ptr %322
 48061|  %323 = load ptr, ptr %257, , !!52978, !!8, !!8                                                                        ;L310<1162<107<2706<3416<3387<52<97
 48063|     ;; x = ptr %322
 48067|     ;; self = ptr %322
 48068|     ;; other = ptr %323
 48069|  %324 = gep %322, i64 1632                                                                                             ;L2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48070|  %325 = load i64, ptr %324, , !!53769, !!8                                                                             ;L2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48071|     ;; x1 = i64 %325
 48072|     ;; self = i64 %325
 48073|  %326 = gep %322, i64 1640                                                                                             ;L2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48074|  %327 = load i64, ptr %326, , !!53769, !!8                                                                             ;L2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48075|     ;; y1 = i64 %327
 48076|     ;; self = i64 %327
 48077|  %328 = gep %323, i64 1632                                                                                             ;L2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48078|  %329 = load i64, ptr %328, , !!53790, !!8                                                                             ;L2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48079|     ;; x2 = i64 %329
 48080|     ;; other = i64 %329
 48081|  %330 = gep %323, i64 1640                                                                                             ;L2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48082|  %331 = load i64, ptr %330, , !!53790, !!8                                                                             ;L2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48083|     ;; y2 = i64 %331
 48084|     ;; other = i64 %331
 48085|  %332 = icmp ult i64 %325, %329                                                                                        ;L3147<7<2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48086|  %333 = sub nuw i64 %329, %325                                                                                         ;L3147<7<2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48087|  %334 = sub nuw i64 %325, %329                                                                                         ;L3147<7<2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48088|  %335 = select i1 %332, i64 %333, i64 %334                                                                             ;L3147<7<2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48089|     ;; dx = i64 %335
 48090|  %336 = icmp ult i64 %327, %331                                                                                        ;L3147<8<2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48091|  %337 = sub nuw i64 %331, %327                                                                                         ;L3147<8<2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48092|  %338 = sub nuw i64 %327, %331                                                                                         ;L3147<8<2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48093|  %339 = select i1 %336, i64 %337, i64 %338                                                                             ;L3147<8<2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48094|     ;; dy = i64 %339
 48095|  %340 = mul i64 %335, %335                                                                                             ;L9<2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48096|  %341 = mul i64 %339, %339                                                                                             ;L9<2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48097|  %342 = add i64 %341, %340                                                                                             ;L9<2158<52<3379<310<1162<107<2706<3416<3387<52<97
 48098|     ;; first[0..+8] = i64 %342
 48099|     ;; first[8..+8] = ptr %322
 48101|  call void @llvm.memcpy.p0.p0.i64(ptr %10, ptr %11, i64 144, i1 false), !!52978                                        ;L2707<3416<3387<52<97
 48102|  %343 = invoke { i64, ptr } @core::iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtB8_5chain5ChainIB1k_INtNtB8_7flatten7FlattenINtNtNtBc_5array4iter8IntoIterINtNtBc_6option6OptionRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEKj6_EEINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterB2W_EEEINtB2D_8IntoIterB2W_EENCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan9line_safeNtB5i_15LineSafeSubPlan19attack_tower_action0ENCINvNvNtNtNtBa_6traits8iterator8Iterator10min_by_key3keyB2W_yNCB5f_s_0E0EB77_4foldTyB2W_ENCINvNvB77_6min_by4foldB8l_INvB75_7compareB2W_yEE0EB5o_(ptr %10, i64 %342, ptr %322)
 48103|  to label %344 unwind label %208                                                                                       ;L2707<3416<3387<52<97
 48104| 
 48105| 344: ; preds = %321
 48106|  %345 = extractvalue { i64, ptr } %343, 1                                                                              ;L2707<3416<3387<52<97
 48109|     ;; self = ptr %345
 48111|  %346 = icmp eq ptr %345, null                                                                                         ;L2775<50<97
 48112|  br i1 %346, label %440, label %347                                                                                    ;L2775<50<97
 48113| 
 48114| 347: ; preds = %344
 48115|     ;; nearest_enemy_tower = ptr %345
 48116|     ;; self = ptr %345
 48117|     ;; self = ptr %345
 48118|  %348 = gep %252, i64 1600                                                                                             ;L54<97
 48119|  %349 = load i64, ptr %348, , !!52888, !!8                                                                             ;L54<97
 48120|     ;; move_speed = i64 %349
 48121|     ;; self = ptr %252
 48122|  %350 = gep %252, i64 1216                                                                                             ;L742<55<97
 48123|  %351 = load i32, ptr %350, , !!52888, !!8                                                                             ;L742<55<97
 48124|  %352 = icmp eq i32 %351, -1                                                                                           ;L742<55<97
 48125|  br i1 %352, label %440, label %353                                                                                    ;L742<55<97
 48126| 
 48127| 353: ; preds = %347
 48128|  %354 = gep %252, i64 1168                                                                                             ;L742<55<97
 48129|     ;; atk = ptr %354
 48130|     ;; self = ptr %354
 48131|  %355 = gep %345, i64 1632                                                                                             ;L2158<57<97
 48132|  %356 = load i64, ptr %355, , !!52888, !!8                                                                             ;L2158<57<97
 48133|     ;; x1 = i64 %356
 48134|     ;; self = i64 %356
 48135|  %357 = gep %345, i64 1640                                                                                             ;L2158<57<97
 48136|  %358 = load i64, ptr %357, , !!52888, !!8                                                                             ;L2158<57<97
 48137|     ;; y1 = i64 %358
 48138|     ;; self = i64 %358
 48139|  %359 = gep %252, i64 1632                                                                                             ;L2158<57<97
 48140|  %360 = load i64, ptr %359, , !!52888, !!8                                                                             ;L2158<57<97
 48141|     ;; x2 = i64 %360
 48142|     ;; other = i64 %360
 48143|  %361 = gep %252, i64 1640                                                                                             ;L2158<57<97
 48144|  %362 = load i64, ptr %361, , !!52888, !!8                                                                             ;L2158<57<97
 48145|     ;; y2 = i64 %362
 48146|     ;; other = i64 %362
 48147|  %363 = icmp ult i64 %356, %360                                                                                        ;L3147<7<2158<57<97
 48148|  %364 = sub nuw i64 %360, %356                                                                                         ;L3147<7<2158<57<97
 48149|  %365 = sub nuw i64 %356, %360                                                                                         ;L3147<7<2158<57<97
 48150|  %366 = select i1 %363, i64 %364, i64 %365                                                                             ;L3147<7<2158<57<97
 48151|     ;; dx = i64 %366
 48152|  %367 = icmp ult i64 %358, %362                                                                                        ;L3147<8<2158<57<97
 48153|  %368 = sub nuw i64 %362, %358                                                                                         ;L3147<8<2158<57<97
 48154|  %369 = sub nuw i64 %358, %362                                                                                         ;L3147<8<2158<57<97
 48155|  %370 = select i1 %367, i64 %368, i64 %369                                                                             ;L3147<8<2158<57<97
 48156|     ;; dy = i64 %370
 48157|  %371 = mul i64 %366, %366                                                                                             ;L9<2158<57<97
 48158|  %372 = mul i64 %370, %370                                                                                             ;L9<2158<57<97
 48159|  %373 = add i64 %372, %371                                                                                             ;L9<2158<57<97
 48160|     ;; dist = i64 %373
 48161|  %374 = gep %252, i64 1184                                                                                             ;L26<58<97
 48162|  %375 = load i64, ptr %374, , !!52888, !!8                                                                             ;L26<58<97
 48163|  %376 = gep %252, i64 1192                                                                                             ;L26<58<97
 48164|  %377 = load i64, ptr %376, , !!52888, !!8                                                                             ;L26<58<97
 48165|  %378 = gep %252, i64 1480                                                                                             ;L26<58<97
 48166|  %379 = load i64, ptr %378, , !!52888, !!8                                                                             ;L26<58<97
 48167|  %380 = add i64 %379, -1                                                                                               ;L26<58<97
 48168|  %381 = mul i64 %380, %377                                                                                             ;L26<58<97
 48169|  %382 = gep %252, i64 1080                                                                                             ;L26<58<97
 48170|  %383 = load i64, ptr %382, , !!52888, !!8                                                                             ;L26<58<97
 48171|  %384 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %354, ptr %252, ptr %345)
 48172|  to label %385 unwind label %208                                                                                       ;L58<97
 48173| 
 48174| 385: ; preds = %353
 48175|  %386 = gep %252, i64 1136                                                                                             ;L1511<58<97
 48176|  %387 = load i32, ptr %386, , !!52888, !!8                                                                             ;L1511<58<97
 48177|     ;; mult = i32 %387
 48178|  %388 = icmp eq i32 %387, 0                                                                                            ;L1512<58<97
 48179|  br i1 %388, label %389, label %392                                                                                    ;L1512<58<97
 48180| 
 48181| 389: ; preds = %385
 48182|  %390 = gep %252, i64 1664                                                                                             ;L1513<58<97
 48183|  %391 = load i64, ptr %390, , !!52888, !!8                                                                             ;L1513<58<97
 48184|  br label %399                                                                                                         ;L1512<58<97
 48185| 
 48186| 392: ; preds = %385
 48187|  %393 = sext i32 %387 to i64                                                                                           ;L1511<58<97
 48188|     ;; mult = i64 %393
 48189|  %394 = gep %252, i64 1664                                                                                             ;L1515<58<97
 48190|  %395 = load i64, ptr %394, , !!52888, !!8                                                                             ;L1515<58<97
 48191|  %396 = add nsw i64 %393, 100                                                                                          ;L1515<58<97
 48192|  %397 = mul i64 %395, %396                                                                                             ;L1515<58<97
 48193|  %398 = udiv i64 %397, 100                                                                                             ;L1515<58<97
 48194|  br label %399                                                                                                         ;L1512<58<97
 48195| 
 48196| 399: ; preds = %392, %389
 48197|  %400 = phi i64 [ %391, %389 ], [ %398, %392 ]                                                                         ;L0<58<97
 48198|  %401 = gep %345, i64 1136                                                                                             ;L1511<58<97
 48199|  %402 = load i32, ptr %401, , !!52888, !!8                                                                             ;L1511<58<97
 48200|     ;; mult = i32 %402
 48201|  %403 = icmp eq i32 %402, 0                                                                                            ;L1512<58<97
 48202|  br i1 %403, label %404, label %407                                                                                    ;L1512<58<97
 48203| 
 48204| 404: ; preds = %399
 48205|  %405 = gep %345, i64 1664                                                                                             ;L1513<58<97
 48206|  %406 = load i64, ptr %405, , !!52888, !!8                                                                             ;L1513<58<97
 48207|  br label %414                                                                                                         ;L1512<58<97
 48208| 
 48209| 407: ; preds = %399
 48210|  %408 = sext i32 %402 to i64                                                                                           ;L1511<58<97
 48211|     ;; mult = i64 %408
 48212|  %409 = gep %345, i64 1664                                                                                             ;L1515<58<97
 48213|  %410 = load i64, ptr %409, , !!52888, !!8                                                                             ;L1515<58<97
 48214|  %411 = add nsw i64 %408, 100                                                                                          ;L1515<58<97
 48215|  %412 = mul i64 %410, %411                                                                                             ;L1515<58<97
 48216|  %413 = udiv i64 %412, 100                                                                                             ;L1515<58<97
 48217|  br label %414                                                                                                         ;L1512<58<97
 48218| 
 48219| 414: ; preds = %407, %404
 48220|  %415 = phi i64 [ %406, %404 ], [ %413, %407 ]                                                                         ;L0<58<97
 48222|  %416 = mul i64 %349, 30                                                                                               ;L59<97
 48223|  %417 = add i64 %375, %416                                                                                             ;L26<58<97
 48224|  %418 = add i64 %417, %383                                                                                             ;L26<58<97
 48225|  %419 = add i64 %418, %381                                                                                             ;L58<97
 48226|  %420 = add i64 %419, %384                                                                                             ;L58<97
 48227|  %421 = add i64 %420, %400                                                                                             ;L58<97
 48228|  %422 = add i64 %421, %415                                                                                             ;L59<97
 48229|     ;; max_dist = i64 %422
 48230|  %423 = mul i64 %422, %422                                                                                             ;L60<97
 48231|  %424 = icmp ugt i64 %373, %423                                                                                        ;L60<97
 48232|  br i1 %424, label %440, label %425                                                                                    ;L60<97
 48233| 
 48234| 425: ; preds = %414
 48235|  %426 = invoke zeroext i1 @ai::tower_discipline38v22_lane_tower_pressure_attack_allowed(i64 %2, ptr %5, ptr %4, ptr %345)
 48236|  to label %427 unwind label %208                                                                                       ;L64<97
 48237| 
 48238| 427: ; preds = %425
 48239|  br i1 %426, label %428, label %440                                                                                    ;L64<97
 48240| 
 48241| 428: ; preds = %427
 48242|  %429 = gep %345, i64 1472                                                                                             ;L65<97
 48243|  %430 = load i64, ptr %429, , !!52833, !!8                                                                             ;L65<97
 48244|  invoke void @ai::small_action4castNtB2_17SmallActionAttack3new(ptr sret([24 x i8]) %21, ptr %5, i64 %430)
 48245|  to label %431 unwind label %208                                                                                       ;L65<97
 48246| 
 48247| 431: ; preds = %428
 48248|  %432 = gep %21, i64 177                                                                                               ;L65<97
 48249|  store i8 15, ptr %432, , !!53855                                                                                      ;L65<97
 48251|     ;; iter[0..+177] = ptr %21
 48259|     ;; self = ptr %21
 48260|     ;; self = ptr %27
 48261|     ;; self = ptr %27
 48262|     ;; iter = ptr %21
 48263|     ;; iter = ptr %21
 48264|     ;; t = ptr %8
 48265|     ;; strategy = i8 1
 48266|     ;; additional = i64 1
 48267|     ;; needed_extra_cap = i64 1
 48268|     ;; needed_extra_cap = i64 1
 48269|     ;; self = ptr %27
 48270|     ;; self = ptr %27
 48271|     ;; self = ptr %27
 48272|  %433 = load i64, ptr %34, , !!53957, !!8                                                                              ;L738<2153<97
 48273|     ;; used_cap = i64 %433
 48274|     ;; used_cap = i64 %433
 48275|     ;; rhs = i64 %433
 48276|  %434 = load i64, ptr %33, , !!53957, !!8                                                                              ;L149<614<430<738<2153<97
 48277|     ;; self = i64 %434
 48278|  %435 = icmp eq i64 %434, %433                                                                                         ;L614<430<738<2153<97
 48279|  br i1 %435, label %439, label %441                                                                                    ;L614<430<738<2153<97
 48280| 
 48281| 436: ; preds = %451, %439
 48282|  %437 = phi i1 [ true, %451 ], [ false, %439 ]                                                                         ;L0<97
 48283|  %438 = cleanuppad within none []
 48287|  br i1 %437, label %458, label %459                                                                                    ;L2158<97
 48288| 
 48289| 439: ; preds = %431
 48290|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %27, i64 %433, i64 1, i1 zeroext true)
 48291|  to label %441 unwind label %436, !!53957                                                                              ;L619<430<738<2153<97
 48292| 
 48293| 440: ; preds = %427, %414, %347, %344, %320, %245
 48294|     ;; iter[0..+177] = ptr %21
 48302|     ;; self = ptr %21
 48303|     ;; self = ptr %27
 48304|     ;; self = ptr %27
 48305|     ;; iter = ptr %21
 48306|     ;; iter = ptr %21
 48307|     ;; t = ptr %8
 48308|     ;; strategy = i8 1
 48309|     ;; additional = i64 0
 48310|     ;; needed_extra_cap = i64 0
 48311|     ;; needed_extra_cap = i64 0
 48312|     ;; self = ptr %27
 48313|     ;; self = ptr %27
 48314|     ;; self = ptr %27
 48319|     ;; iter[177..+1] = i8 -1
 48320|  br label %460                                                                                                         ;L2155<97
 48321| 
 48322| 441: ; preds = %439, %431
 48323|     ;; iter[177..+1] = i8 15
 48324|  %442 = gep %21, i64 178                                                                                               ;L2155<97
 48325|  %443 = gep %8, i64 177
 48326|  %444 = gep %8, i64 178
 48328|  call void @llvm.memcpy.p0.p0.i64(ptr %8, ptr %21, i64 177, i1 false), !!53856                                         ;L2155<97
 48329|  store i8 15, ptr %443, , !!53976                                                                                      ;L2155<97
 48330|  call void @llvm.memcpy.p0.p0.i64(ptr %444, ptr %442, i64 6, i1 false), !!53856                                        ;L2155<97
 48332|     ;; self = ptr %27
 48333|     ;; self = ptr %27
 48334|     ;; value = ptr %8
 48335|     ;; src = ptr %8
 48336|     ;; additional = i64 1
 48337|     ;; needed_extra_cap = i64 1
 48338|     ;; needed_extra_cap = i64 1
 48339|     ;; strategy = i8 1
 48340|  %445 = load i64, ptr %34, , !!53994, !!8                                                                              ;L1428<2156<97
 48341|     ;; self = ptr %27
 48342|  %446 = load i64, ptr %33, , !!53994, !!8                                                                              ;L149<1428<2156<97
 48343|  %447 = icmp eq i64 %445, %446                                                                                         ;L1428<2156<97
 48344|  br i1 %447, label %448, label %453                                                                                    ;L1428<2156<97
 48345| 
 48346| 448: ; preds = %441
 48347|     ;; self = ptr %27
 48348|     ;; self = ptr %27
 48349|     ;; self = ptr %27
 48350|     ;; used_cap = i64 %445
 48351|     ;; used_cap = i64 %445
 48352|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %27, i64 %445, i64 1, i1 zeroext true)
 48353|  to label %449 unwind label %451, !!53994                                                                              ;L619<430<738<1429<2156<97
 48354| 
 48355| 449: ; preds = %448
 48356|  %450 = load i64, ptr %34, , !!53994                                                                                   ;L1432<2156<97
 48357|  br label %453                                                                                                         ;L619<430<738<1429<2156<97
 48358| 
 48359| 451: ; preds = %448
 48360|  %452 = cleanuppad within none []
 48361|     ;; iter[177..+1] = i8 -1
 48362|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %8) #30 [ "funclet"(token %452) ], !!54004 ;L1436<2156<97
 48366|  cleanupret from %452 unwind label %436                                                                                ;L2157<97
 48367| 
 48368| 453: ; preds = %449, %441
 48369|  %454 = phi i64 [ %450, %449 ], [ %445, %441 ]                                                                         ;L1432<2156<97
 48370|     ;; self = ptr %27
 48371|  %455 = load ptr, ptr %27, , !!53994, !!8, !!8                                                                         ;L138<1432<2156<97
 48372|     ;; self = ptr %455
 48373|     ;; count = i64 %454
 48374|  %456 = gepS %455, i64 %454                                                                                            ;L961<1432<2156<97
 48375|     ;; end = ptr %456
 48376|     ;; dst = ptr %456
 48377|  call void @llvm.memcpy.p0.p0.i64(ptr %456, ptr %8, i64 184, i1 false), !!54004                                        ;L1933<1433<2156<97
 48378|  %457 = add i64 %454, 1                                                                                                ;L1434<2156<97
 48379|  store i64 %457, ptr %34, , !!53994                                                                                    ;L1434<2156<97
 48381|     ;; self = ptr undef
 48382|  br label %460                                                                                                         ;L0<1898<2494<2629<2155<97
 48383| 
 48384| 458: ; preds = %459, %436
 48385|  cleanupret from %438 unwind label %208
 48386| 
 48387| 459: ; preds = %436
 48388|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %21) [ "funclet"(token %438) ], !!53856 ;L825<825<825<2158<97
 48389|  br label %458                                                                                                         ;L825<825<825<2158<97
 48390| 
 48391| 460: ; preds = %453, %440
 48392|     ;; iter[177..+1] = i8 -1
 48398|  invoke void @ai::fight_check29attack_structure_skill_action(ptr sret([32 x i8]) %20, ptr %4, ptr %5)
 48399|  to label %461 unwind label %208                                                                                       ;L98
 48400| 
 48401| 461: ; preds = %460
 48402|  %462 = load ptr, ptr %20, , !!8, !!8                                                                                  ;L98
 48403|  %463 = gep %20, i64 24                                                                                                ;L98
 48404|  %464 = load i64, ptr %463, , !!8                                                                                      ;L98
 48405|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %27, ptr %462, i64 %464)
 48406|  to label %465 unwind label %208                                                                                       ;L98
 48407| 
 48408| 465: ; preds = %461
 48411|  store ptr %28, ptr %19,                                                                                               ;L99
 48412|  %466 = gep %19, i64 8                                                                                                 ;L99
 48413|  store ptr %5, ptr %466,                                                                                               ;L99
 48414|  %467 = gep %19, i64 16                                                                                                ;L99
 48415|  store ptr %4, ptr %467,                                                                                               ;L99
 48416|  invoke void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE6retainNCNvMNtNtNtBY_11plan_legacy8sub_plan9line_safeNtB22_15LineSafeSubPlan17action_candidates0EBY_(ptr %27, ptr %19)
 48417|  to label %468 unwind label %208                                                                                       ;L99
 48418| 
 48419| 468: ; preds = %465
 48421|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %27, i64 32, i1 false)                                                   ;L101
 48423|  ret void                                                                                                              ;L102
 48424| }
