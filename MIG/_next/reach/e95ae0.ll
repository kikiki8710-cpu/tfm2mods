 48981| define i64 @ai::plan_legacy8sub_plan13defense_nexusNtB4_19DefenseNexusSubPlan5score(ptr %0, i64 %1, ptr %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %7) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 48982|  %9 = alloca [24 x i8],
 48983|  %10 = alloca [32 x i8],
 48984|  %11 = alloca [32 x i8],
 48985|     ;; self = ptr %0
 48986|     ;; version = i64 %1
 48987|     ;; parameter = ptr %2
 48988|     ;; rnd = ptr %3
 48989|     ;; player = ptr %4
 48990|     ;; data = ptr %5
 48991|     ;; action = ptr %6
 48992|     ;; debug = ptr %7
 48993|     ;; front_minions = ptr %11
 48994|     ;; action_type = i8 2
 48995|     ;; line = i8 0
 48996|     ;; line = i8 1
 48997|     ;; line = i8 2
 49000|  %12 = gep %4, i64 2352                                                                                                ;L312
 49001|  %13 = load i64, ptr %12, , !!8                                                                                        ;L312
 49002|  %14 = icmp ult i64 %13, 2                                                                                             ;L312
 49003|  br i1 %14, label %16, label %15                                                                                       ;L312
 49004| 
 49005| 15: ; preds = %8
 49006|  tail call void @core::panicking18panic_bounds_check(i64 %13, i64 2, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.196) #35 ;L312
 49007|  unreachable                                                                                                           ;L312
 49008| 
 49009| 16: ; preds = %8
 49010|     ;; self = ptr %4
 49011|  %17 = gep %4, i64 2496                                                                                                ;L581<312
 49012|  %18 = load i32, ptr %17, , !!8                                                                                        ;L581<312
 49013|  %19 = zext nneg i32 %18 to i64                                                                                        ;L581<312
 49014|  %20 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L312
 49015|  %21 = gep %20, i64 480                                                                                                ;L312
 49016|  %22 = getelementptr [5 x ptr], ptr %21, i64 %13                                                                       ;L312
 49017|  %23 = getelementptr ptr, ptr %22, i64 %19                                                                             ;L312
 49018|  %24 = load ptr, ptr %23, , !!8                                                                                        ;L312
 49019|     ;; self = ptr %24
 49020|  %25 = icmp eq ptr %24, null                                                                                           ;L1011<312
 49021|  br i1 %25, label %31, label %26                                                                                       ;L1011<312
 49022| 
 49023| 26: ; preds = %16
 49024|     ;; champ = ptr %24
 49025|     ;; self = ptr %24
 49026|     ;; self = ptr %24
 49027|     ;; self = ptr %24
 49028|     ;; self = ptr %24
 49029|  %27 = tail call i64 @ai::action_score17interaction_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %6, ptr %7)      ;L313
 49030|     ;; base = i64 %27
 49031|  %28 = icmp ugt i64 %1, 1                                                                                              ;L318
 49032|  %29 = gep %6, i64 177
 49033|  %30 = load i8, ptr %29, , !!8                                                                                         ;L309<0
 49034|  br i1 %28, label %32, label %36                                                                                       ;L318
 49035| 
 49036| 31: ; preds = %16
 49037|  tail call void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.197) #35                       ;L1013<312
 49038|  unreachable                                                                                                           ;L1013<312
 49039| 
 49040| 32: ; preds = %26
 49041|     ;; self = ptr %6
 49042|  %33 = icmp ne i8 %30, 10                                                                                              ;L309<319
 49043|  tail call void @llvm.assume(i1 %33)                                                                                   ;L309<319
 49044|  %34 = add nsw i8 %30, -15                                                                                             ;L309<319
 49045|  %35 = icmp ult i8 %34, 4                                                                                              ;L309<319
 49046|  br i1 %35, label %43, label %36                                                                                       ;L309<319
 49047| 
 49048| 36: ; preds = %53, %43, %32, %26
 49049|  %37 = phi i64 [ %27, %32 ], [ %56, %53 ], [ %27, %43 ], [ %27, %26 ]                                                  ;L0
 49050|     ;; base = i64 %37
 49051|     ;; self = ptr %6
 49052|  %38 = icmp ne i8 %30, 10                                                                                              ;L309<329
 49053|  tail call void @llvm.assume(i1 %38)                                                                                   ;L309<329
 49054|  %39 = add nsw i8 %30, -3                                                                                              ;L309<329
 49055|  %40 = icmp samesign ugt i8 %30, 2                                                                                     ;L309<329
 49056|  %41 = select i1 %40, i8 %39, i8 7                                                                                     ;L309<329
 49057|  switch i8 %41, label %42 [
 49058|  i8 0, label %97
 49059|  i8 1, label %97
 49060|  i8 2, label %57
 49061|  i8 3, label %57
 49062|  i8 4, label %97
 49063|  i8 5, label %97
 49064|  i8 6, label %97
 49065|  i8 7, label %97
 49066|  i8 8, label %97
 49067|  i8 9, label %97
 49068|  i8 10, label %57
 49069|  i8 11, label %97
 49070|  i8 12, label %67
 49071|  i8 13, label %77
 49072|  i8 14, label %87
 49073|  i8 15, label %97
 49074|  i8 16, label %97
 49075|  ]                                                                                                                     ;L309<329
 49076| 
 49077| 42: ; preds = %36
 49078|  unreachable                                                                                                           ;L309<329
 49079| 
 49080| 43: ; preds = %32
 49081|  %44 = gep %6, i64 8                                                                                                   ;L0<319
 49082|  %45 = load i64, ptr %44, , !!62282, !!8                                                                               ;L0<319
 49083|     ;; self[8..+8] = i64 %45
 49084|  %46 = load ptr, ptr %20, , !!8, !!8                                                                                   ;L320
 49085|  %47 = gep %20, i64 8                                                                                                  ;L320
 49086|  %48 = load ptr, ptr %47, , !!8, !!8                                                                                   ;L320
 49087|     ;; f[0..+8] = ptr %46
 49088|     ;; f[8..+8] = ptr %48
 49089|     ;; x = i64 %45
 49090|  %49 = gep %48, i64 496                                                                                                ;L1543<320
 49091|  %50 = load ptr, ptr %49,                                                                                              ;L1543<320
 49092|     ;; id = i64 %45
 49093|  %51 = tail call ptr %50(ptr %46, i64 %45)                                                                             ;L320<1543<320
 49094|     ;; self = ptr %51
 49095|     ;; f[0..+8] = ptr %4
 49096|     ;; f[8..+8] = ptr %5
 49097|  %52 = icmp eq ptr %51, null                                                                                           ;L659<321
 49098|  br i1 %52, label %36, label %53                                                                                       ;L659<321
 49099| 
 49100| 53: ; preds = %43
 49101|     ;; x = ptr %51
 49105|     ;; e = ptr %51
 49106|  %54 = tail call zeroext i1 @ai::plan_legacy3old13defense_nexus24is_base_attacking_minion(ptr %4, ptr %5, ptr %51)     ;L321<661<321
 49107|     ;; target_is_base_attacker = i1 %54
 49108|  %55 = add i64 %27, 100
 49109|  %56 = select i1 %54, i64 %55, i64 %27                                                                                 ;L322
 49110|  br label %36                                                                                                          ;L322
 49111| 
 49112| 57: ; preds = %36, %36, %36
 49113|  %58 = gep %6, i64 8                                                                                                   ;L0<329
 49114|  %59 = load i64, ptr %58, , !!62345, !!8                                                                               ;L0<329
 49115|     ;; target_id = i64 %59
 49116|  %60 = load ptr, ptr %20, , !!8, !!8                                                                                   ;L358
 49117|  %61 = gep %20, i64 8                                                                                                  ;L358
 49118|  %62 = load ptr, ptr %61, , !!8, !!8                                                                                   ;L358
 49119|  %63 = gep %62, i64 496                                                                                                ;L358
 49120|  %64 = load ptr, ptr %63,                                                                                              ;L358
 49121|  %65 = tail call ptr %64(ptr %60, i64 %59)                                                                             ;L358
 49122|  %66 = icmp eq ptr %65, null                                                                                           ;L358
 49123|  br i1 %66, label %97, label %100                                                                                      ;L358
 49124| 
 49125| 67: ; preds = %36
 49126|     ;; action = ptr %6
 49127|     ;; self = ptr %6
 49128|  %68 = gep %6, i64 8                                                                                                   ;L94<322<329
 49129|  %69 = load i64, ptr %68, , !!62345, !!8                                                                               ;L94<322<329
 49130|     ;; target_id = i64 %69
 49131|  %70 = load ptr, ptr %20, , !!8, !!8                                                                                   ;L334
 49132|  %71 = gep %20, i64 8                                                                                                  ;L334
 49133|  %72 = load ptr, ptr %71, , !!8, !!8                                                                                   ;L334
 49134|  %73 = gep %72, i64 496                                                                                                ;L334
 49135|  %74 = load ptr, ptr %73, , !!8                                                                                        ;L334
 49136|  %75 = tail call ptr %74(ptr %70, i64 %69)                                                                             ;L334
 49137|  %76 = icmp eq ptr %75, null                                                                                           ;L334
 49138|  br i1 %76, label %97, label %216                                                                                      ;L334
 49139| 
 49140| 77: ; preds = %36
 49141|     ;; action = ptr %6
 49142|     ;; self = ptr %6
 49143|  %78 = gep %6, i64 8                                                                                                   ;L160<323<329
 49144|  %79 = load i64, ptr %78, , !!62345, !!8                                                                               ;L160<323<329
 49145|     ;; target_id = i64 %79
 49146|  %80 = load ptr, ptr %20, , !!8, !!8                                                                                   ;L342
 49147|  %81 = gep %20, i64 8                                                                                                  ;L342
 49148|  %82 = load ptr, ptr %81, , !!8, !!8                                                                                   ;L342
 49149|  %83 = gep %82, i64 496                                                                                                ;L342
 49150|  %84 = load ptr, ptr %83, , !!8                                                                                        ;L342
 49151|  %85 = tail call ptr %84(ptr %80, i64 %79)                                                                             ;L342
 49152|  %86 = icmp eq ptr %85, null                                                                                           ;L342
 49153|  br i1 %86, label %97, label %226                                                                                      ;L342
 49154| 
 49155| 87: ; preds = %36
 49156|     ;; action = ptr %6
 49157|     ;; self = ptr %6
 49158|  %88 = gep %6, i64 8                                                                                                   ;L222<324<329
 49159|  %89 = load i64, ptr %88, , !!62345, !!8                                                                               ;L222<324<329
 49160|     ;; target_id = i64 %89
 49161|  %90 = load ptr, ptr %20, , !!8, !!8                                                                                   ;L350
 49162|  %91 = gep %20, i64 8                                                                                                  ;L350
 49163|  %92 = load ptr, ptr %91, , !!8, !!8                                                                                   ;L350
 49164|  %93 = gep %92, i64 496                                                                                                ;L350
 49165|  %94 = load ptr, ptr %93, , !!8                                                                                        ;L350
 49166|  %95 = tail call ptr %94(ptr %90, i64 %89)                                                                             ;L350
 49167|  %96 = icmp eq ptr %95, null                                                                                           ;L350
 49168|  br i1 %96, label %97, label %236                                                                                      ;L350
 49169| 
 49170| 97: ; preds = %245, %230, %220, %215, %112, %108, %106, %87, %77, %67, %57, %36, %36, %36, %36, %36, %36, %36, %36, %36, %36, %36
 49171|  %98 = phi i64 [ -99999, %77 ], [ -99999, %87 ], [ -99999, %67 ], [ 0, %108 ], [ 0, %57 ], [ %212, %215 ], [ 0, %112 ], [ 0, %106 ], [ %224, %220 ], [ %234, %230 ], [ %249, %245 ], [ 0, %36 ], [ 0, %36 ], [ 0, %36 ], [ 0, %36 ], [ 0, %36 ], [ 0, %36 ], [ 0, %36 ], [ 0, %36 ], [ 0, %36 ], [ 0, %36 ], [ 0, %36 ] ;L0
 49172|  %99 = add i64 %98, %37                                                                                                ;L329
 49173|  ret i64 %99                                                                                                           ;L394
 49174| 
 49175| 100: ; preds = %57
 49176|     ;; t = ptr %65
 49177|     ;; self = ptr %65
 49178|     ;; self = ptr %65
 49179|     ;; other = ptr %24
 49180|     ;; other = ptr %24
 49181|  %101 = load i64, ptr %65, , !!8                                                                                       ;L1127<264<359
 49182|  %102 = gep %65, i64 8                                                                                                 ;L1127<264<359
 49183|     ;; __self_discr = i64 %101
 49184|  %103 = load i64, ptr %24, , !!8                                                                                       ;L1127<264<359
 49185|  %104 = gep %24, i64 8                                                                                                 ;L1127<264<359
 49186|     ;; __arg1_discr = i64 %103
 49187|  %105 = icmp eq i64 %101, %103                                                                                         ;L1127<264<359
 49188|  br i1 %105, label %106, label %108                                                                                    ;L1127<264<359
 49189| 
 49190| 106: ; preds = %100
 49191|  %107 = icmp eq i64 %101, 0                                                                                            ;L1127<264<359
 49192|  br i1 %107, label %112, label %97                                                                                     ;L1127<264<359
 49193| 
 49194| 108: ; preds = %112, %100
 49195|     ;; self = ptr %65
 49196|  %109 = gep %65, i64 104                                                                                               ;L1261<360
 49197|  %110 = load i64, ptr %109, , !!8                                                                                      ;L1261<360
 49198|  %111 = icmp eq i64 %110, 1                                                                                            ;L360
 49199|  br i1 %111, label %116, label %97                                                                                     ;L360
 49200| 
 49201| 112: ; preds = %106
 49202|     ;; __self_0 = ptr %65
 49203|     ;; self = ptr %65
 49204|     ;; __arg1_0 = ptr %24
 49205|     ;; other = ptr %24
 49208|  %113 = load i64, ptr %102, , !!8                                                                                      ;L1878<2123<1127<264<359
 49209|  %114 = load i64, ptr %104, , !!8                                                                                      ;L1878<2123<1127<264<359
 49210|  %115 = icmp eq i64 %113, %114                                                                                         ;L1878<2123<1127<264<359
 49211|  br i1 %115, label %97, label %108                                                                                     ;L359
 49212| 
 49213| 116: ; preds = %108
 49214|  %117 = sub nuw nsw i64 1, %13                                                                                         ;L361
 49215|  %118 = gep %5, i64 16                                                                                                 ;L361
 49216|  %119 = load ptr, ptr %118, , !!8, !!8                                                                                 ;L361
 49217|  %120 = getelementptr { { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, [5 x { i64, [2 x i64] }], [5 x { i64, { i8, [23 x i8] } }], [5 x { i8, [7 x i8], i64 }], [5 x i64], [5 x i64], [5 x i64], [5 x { i64, i64 }], i64, i64, i64, i64, i64, [5 x i32], i8, [3 x i8] }, ptr %119, i64 %117 ;L361
 49218|     ;; self = ptr %120
 49219|  %121 = load i64, ptr %120, , !!8                                                                                      ;L361
 49220|     ;; self[0..+8] = i64 %121
 49222|     ;; f[0..+8] = ptr %60
 49223|     ;; f[8..+8] = ptr %62
 49224|  %122 = trunc nuw i64 %121 to i1                                                                                       ;L1542<362
 49225|  br i1 %122, label %123, label %127                                                                                    ;L1542<362
 49226| 
 49227| 123: ; preds = %116
 49228|  %124 = gep %120, i64 8                                                                                                ;L361
 49229|  %125 = load i64, ptr %124,                                                                                            ;L361
 49230|     ;; self[8..+8] = i64 %125
 49231|     ;; x = i64 %125
 49232|     ;; m = i64 %125
 49233|  %126 = tail call ptr %64(ptr %60, i64 %125)                                                                           ;L362<1543<362
 49234|     ;; top_front_minion = ptr %126
 49235|  br label %127                                                                                                         ;L1543<362
 49236| 
 49237| 127: ; preds = %123, %116
 49238|  %128 = phi ptr [ %126, %123 ], [ null, %116 ]                                                                         ;L0<362
 49239|     ;; top_front_minion = ptr %128
 49240|     ;; self = ptr %120
 49241|  %129 = gep %120, i64 40                                                                                               ;L381<363
 49242|  %130 = load i64, ptr %129, , !!8                                                                                      ;L363
 49243|     ;; self[0..+8] = i64 %130
 49245|     ;; f[0..+8] = ptr %60
 49246|     ;; f[8..+8] = ptr %62
 49247|  %131 = trunc nuw i64 %130 to i1                                                                                       ;L1542<364
 49248|  br i1 %131, label %132, label %136                                                                                    ;L1542<364
 49249| 
 49250| 132: ; preds = %127
 49251|  %133 = gep %120, i64 48                                                                                               ;L363
 49252|  %134 = load i64, ptr %133,                                                                                            ;L363
 49253|     ;; self[8..+8] = i64 %134
 49254|     ;; x = i64 %134
 49255|     ;; m = i64 %134
 49256|  %135 = tail call ptr %64(ptr %60, i64 %134)                                                                           ;L364<1543<364
 49257|     ;; mid_front_minion = ptr %135
 49258|  br label %136                                                                                                         ;L1543<364
 49259| 
 49260| 136: ; preds = %132, %127
 49261|  %137 = phi ptr [ %135, %132 ], [ null, %127 ]                                                                         ;L0<364
 49262|     ;; mid_front_minion = ptr %137
 49263|     ;; self = ptr %120
 49264|  %138 = gep %120, i64 80                                                                                               ;L382<365
 49265|  %139 = load i64, ptr %138, , !!8                                                                                      ;L365
 49266|     ;; self[0..+8] = i64 %139
 49268|     ;; f[0..+8] = ptr %60
 49269|     ;; f[8..+8] = ptr %62
 49270|  %140 = trunc nuw i64 %139 to i1                                                                                       ;L1542<366
 49271|  br i1 %140, label %141, label %145                                                                                    ;L1542<366
 49272| 
 49273| 141: ; preds = %136
 49274|  %142 = gep %120, i64 88                                                                                               ;L365
 49275|  %143 = load i64, ptr %142,                                                                                            ;L365
 49276|     ;; self[8..+8] = i64 %143
 49277|     ;; x = i64 %143
 49278|     ;; m = i64 %143
 49279|  %144 = tail call ptr %64(ptr %60, i64 %143)                                                                           ;L366<1543<366
 49280|     ;; bottom_front_minion = ptr %144
 49281|  br label %145                                                                                                         ;L1543<366
 49282| 
 49283| 145: ; preds = %141, %136
 49284|  %146 = phi ptr [ %144, %141 ], [ null, %136 ]                                                                         ;L0<366
 49285|     ;; bottom_front_minion = ptr %146
 49286|  %147 = gep %20, i64 368                                                                                               ;L367
 49287|  %148 = getelementptr ptr, ptr %147, i64 %13                                                                           ;L367
 49288|  %149 = load ptr, ptr %148, , !!8                                                                                      ;L367
 49289|     ;; self = ptr %149
 49290|  %150 = icmp eq ptr %149, null                                                                                         ;L1011<367
 49291|  br i1 %150, label %197, label %151                                                                                    ;L1011<367
 49292| 
 49293| 151: ; preds = %145
 49294|     ;; nexus = ptr %149
 49297|     ;; layout[0..+8] = i64 8
 49298|     ;; layout[0..+8] = i64 8
 49299|     ;; layout[0..+8] = i64 8
 49300|     ;; layout[8..+8] = i64 24
 49301|     ;; layout[8..+8] = i64 24
 49302|     ;; layout[8..+8] = i64 24
 49303|     ;; self = ptr inttoptr (i64 1 to ptr)
 49304|     ;; self = ptr inttoptr (i64 1 to ptr)
 49305|     ;; zeroed = i8 0
 49306|     ;; layout[0..+8] = i64 8
 49307|     ;; layout[0..+8] = i64 8
 49308|     ;; layout[8..+8] = i64 24
 49309|     ;; layout[8..+8] = i64 24
 49310|     ;; zeroed = i1 false
 49311|     ;; size = i64 24
 49312|  tail call void @_RNvCseLSQwpavqd5_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #36                               ;L99<210<332<449<248<317<370
 49313|  %152 = tail call ptr @_RNvCseLSQwpavqd5_7___rustc12___rust_alloc(i64 24, i64 8) #36                                   ;L101<210<332<449<248<317<370
 49314|  %153 = icmp eq ptr %152, null                                                                                         ;L248<317<370
 49315|  br i1 %153, label %154, label %155                                                                                    ;L248<317<370
 49316| 
 49317| 154: ; preds = %151
 49318|  tail call void @_RNvNtCs9LexZzt9XJB_5alloc5alloc18handle_alloc_error(i64 8, i64 24) #35                               ;L250<317<370
 49319|  unreachable                                                                                                           ;L250<317<370
 49320| 
 49321| 155: ; preds = %151
 49322|  store ptr %128, ptr %152,                                                                                             ;L370
 49323|  %156 = gep %152, i64 8                                                                                                ;L370
 49324|  store ptr %137, ptr %156,                                                                                             ;L370
 49325|  %157 = gep %152, i64 16                                                                                               ;L370
 49326|  store ptr %146, ptr %157,                                                                                             ;L370
 49327|     ;; self[0..+8] = i64 3
 49328|     ;; self[8..+8] = ptr %152
 49329|     ;; self[16..+8] = i64 3
 49330|     ;; me[0..+8] = i64 3
 49331|     ;; me[8..+8] = ptr %152
 49332|     ;; me[16..+8] = i64 3
 49333|     ;; buf = ptr %152
 49334|     ;; begin = ptr %152
 49335|     ;; self = ptr %152
 49336|     ;; count = i64 3
 49337|  %158 = gep %152, i64 24                                                                                               ;L961<3961<370
 49338|     ;; self[0..+8] = ptr %152
 49339|     ;; self[16..+8] = i64 3
 49340|     ;; self[8..+8] = ptr %152
 49341|     ;; self[24..+8] = ptr %158
 49342|  store ptr %152, ptr %10,                                                                                              ;L24<1002<370
 49343|  %159 = gep %10, i64 8                                                                                                 ;L24<1002<370
 49344|  store ptr %152, ptr %159,                                                                                             ;L24<1002<370
 49345|  %160 = gep %10, i64 16                                                                                                ;L24<1002<370
 49346|  store i64 3, ptr %160,                                                                                                ;L24<1002<370
 49347|  %161 = gep %10, i64 24                                                                                                ;L24<1002<370
 49348|  store ptr %158, ptr %161,                                                                                             ;L24<1002<370
 49349|  %162 = gep %5, i64 8                                                                                                  ;L370
 49350|  %163 = load ptr, ptr %162, , !!8, !!8                                                                                 ;L370
 49351|  %164 = load ptr, ptr %163, , !!8, !!8                                                                                 ;L370
 49352|  call void @core::iter8adapters10filter_map9FilterMapINtNtNtCs9LexZzt9XJB_5alloc3vec9into_iter8IntoIterINtNtB2d_6option6OptionBU_EENCNvMs_NtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan13defense_nexusNtB4t_19DefenseNexusSubPlan5scores3_0EEB4z_(ptr sret([32 x i8]) %11, ptr %10, ptr %164) ;L369
 49354|     ;; self = ptr %11
 49355|     ;; self = ptr %11
 49356|  %165 = load ptr, ptr %11, , !!8, !!8                                                                                  ;L138<2073<372
 49357|     ;; p = ptr %165
 49358|  %166 = gep %11, i64 24                                                                                                ;L2075<372
 49359|  %167 = load i64, ptr %166, , !!8                                                                                      ;L2075<372
 49360|     ;; len = i64 %167
 49361|     ;; count = i64 %167
 49362|     ;; self[0..+8] = ptr %165
 49363|     ;; slice[0..+8] = ptr %165
 49364|     ;; self[8..+8] = i64 %167
 49365|     ;; slice[8..+8] = i64 %167
 49366|     ;; ptr = ptr %165
 49367|     ;; self = ptr %165
 49368|  %168 = shl nuw nsw i64 %167, 3                                                                                        ;L961<100<1042<372
 49369|  %169 = gep %165, i64 %168                                                                                             ;L961<100<1042<372
 49371|     ;; self[0..+8] = ptr %165
 49372|     ;; self[8..+8] = ptr %169
 49373|     ;; f = ptr %149
 49374|     ;; self = ptr %9
 49377|     ;; self[0..+8] = ptr %165
 49378|     ;; self[8..+8] = ptr %169
 49379|     ;; f = ptr %149
 49380|  %170 = gep %9, i64 8                                                                                                  ;L69<836<3387<372
 49381|  store ptr %169, ptr %170, , !!62634                                                                                   ;L69<836<3387<372
 49382|  %171 = gep %9, i64 16                                                                                                 ;L69<836<3387<372
 49383|  store ptr %149, ptr %171, , !!62634                                                                                   ;L69<836<3387<372
 49385|     ;; self = ptr %9
 49388|     ;; self = ptr %9
 49389|     ;; self = ptr %9
 49390|     ;; count = i64 1
 49391|     ;; ptr = ptr %165
 49392|     ;; self = ptr %165
 49393|     ;; end_or_len = ptr %169
 49396|  %172 = icmp eq i64 %167, 0                                                                                            ;L1714<180<107<2706<3416<3387<372
 49397|  br i1 %172, label %173, label %174                                                                                    ;L180<107<2706<3416<3387<372
 49398| 
 49399| 173: ; preds = %155
 49401|     ;; nearest_front_minion = ptr null
 49402|  br label %211                                                                                                         ;L373
 49403| 
 49404| 174: ; preds = %155
 49405|  %175 = gep %165, i64 8                                                                                                ;L656<185<107<2706<3416<3387<372
 49406|  store ptr %175, ptr %9, , !!62576                                                                                     ;L185<107<2706<3416<3387<372
 49407|     ;; self = ptr %165
 49408|     ;; f = ptr %9
 49409|     ;; self = ptr %9
 49410|     ;; x = ptr %165
 49411|     ;; args = ptr %165
 49413|     ;; x = ptr %165
 49417|  %176 = load ptr, ptr %165, , !!62731, !!8, !!8                                                                        ;L372<3379<310<1162<107<2706<3416<3387<372
 49418|     ;; self = ptr %176
 49419|     ;; other = ptr %149
 49420|  %177 = gep %176, i64 1632                                                                                             ;L2158<372<3379<310<1162<107<2706<3416<3387<372
 49421|  %178 = load i64, ptr %177, , !!62739, !!8                                                                             ;L2158<372<3379<310<1162<107<2706<3416<3387<372
 49422|     ;; x1 = i64 %178
 49423|     ;; self = i64 %178
 49424|  %179 = gep %176, i64 1640                                                                                             ;L2158<372<3379<310<1162<107<2706<3416<3387<372
 49425|  %180 = load i64, ptr %179, , !!62739, !!8                                                                             ;L2158<372<3379<310<1162<107<2706<3416<3387<372
 49426|     ;; y1 = i64 %180
 49427|     ;; self = i64 %180
 49428|  %181 = gep %149, i64 1632                                                                                             ;L2158<372<3379<310<1162<107<2706<3416<3387<372
 49429|  %182 = load i64, ptr %181, , !!62739, !!8                                                                             ;L2158<372<3379<310<1162<107<2706<3416<3387<372
 49430|     ;; x2 = i64 %182
 49431|     ;; other = i64 %182
 49432|  %183 = gep %149, i64 1640                                                                                             ;L2158<372<3379<310<1162<107<2706<3416<3387<372
 49433|  %184 = load i64, ptr %183, , !!62739, !!8                                                                             ;L2158<372<3379<310<1162<107<2706<3416<3387<372
 49434|     ;; y2 = i64 %184
 49435|     ;; other = i64 %184
 49436|  %185 = icmp ult i64 %178, %182                                                                                        ;L3147<7<2158<372<3379<310<1162<107<2706<3416<3387<372
 49437|  %186 = sub nuw i64 %182, %178                                                                                         ;L3147<7<2158<372<3379<310<1162<107<2706<3416<3387<372
 49438|  %187 = sub nuw i64 %178, %182                                                                                         ;L3147<7<2158<372<3379<310<1162<107<2706<3416<3387<372
 49439|  %188 = select i1 %185, i64 %186, i64 %187                                                                             ;L3147<7<2158<372<3379<310<1162<107<2706<3416<3387<372
 49440|     ;; dx = i64 %188
 49441|  %189 = icmp ult i64 %180, %184                                                                                        ;L3147<8<2158<372<3379<310<1162<107<2706<3416<3387<372
 49442|  %190 = sub nuw i64 %184, %180                                                                                         ;L3147<8<2158<372<3379<310<1162<107<2706<3416<3387<372
 49443|  %191 = sub nuw i64 %180, %184                                                                                         ;L3147<8<2158<372<3379<310<1162<107<2706<3416<3387<372
 49444|  %192 = select i1 %189, i64 %190, i64 %191                                                                             ;L3147<8<2158<372<3379<310<1162<107<2706<3416<3387<372
 49445|     ;; dy = i64 %192
 49446|  %193 = mul i64 %188, %188                                                                                             ;L9<2158<372<3379<310<1162<107<2706<3416<3387<372
 49447|  %194 = mul i64 %192, %192                                                                                             ;L9<2158<372<3379<310<1162<107<2706<3416<3387<372
 49448|  %195 = add i64 %194, %193                                                                                             ;L9<2158<372<3379<310<1162<107<2706<3416<3387<372
 49449|     ;; first[0..+8] = i64 %195
 49450|     ;; first[8..+8] = ptr %165
 49451|  %196 = invoke { i64, ptr } @core::iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityENCINvNvNtNtNtBa_6traits8iterator8Iterator10min_by_key3keyRB1n_yNCNvMs_NtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan13defense_nexusNtB3r_19DefenseNexusSubPlan5scores4_0E0EB2q_4foldTyB3e_ENCINvNvB2q_6min_by4foldB5h_INvB2o_7compareB3e_yEE0EB3x_(ptr %9, i64 %195, ptr %165)
 49452|  to label %200 unwind label %198                                                                                       ;L2707<3416<3387<372
 49453| 
 49454| 197: ; preds = %145
 49455|  tail call void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.198) #35                       ;L1013<367
 49456|  unreachable                                                                                                           ;L1013<367
 49457| 
 49458| 198: ; preds = %174
 49459|  %199 = cleanuppad within none []
 49460|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEECshdEBA0ozCnw_7game_ai(ptr %11) #34 [ "funclet"(token %199) ] ;L382
 49461|  cleanupret from %199 unwind to caller                                                                                 ;L311
 49462| 
 49463| 200: ; preds = %174
 49464|  %201 = extractvalue { i64, ptr } %196, 1                                                                              ;L2707<3416<3387<372
 49466|     ;; nearest_front_minion = ptr %201
 49467|  %202 = icmp eq ptr %201, null                                                                                         ;L373
 49468|  br i1 %202, label %211, label %203                                                                                    ;L373
 49469| 
 49470| 203: ; preds = %200
 49471|     ;; nearest_front_minion = ptr %201
 49472|  %204 = load ptr, ptr %201, , !!8, !!8                                                                                 ;L374
 49473|  %205 = gep %204, i64 1472                                                                                             ;L374
 49474|  %206 = load i64, ptr %205, , !!8                                                                                      ;L374
 49475|  %207 = gep %65, i64 1472                                                                                              ;L374
 49476|  %208 = load i64, ptr %207, , !!8                                                                                      ;L374
 49477|  %209 = icmp eq i64 %206, %208                                                                                         ;L374
 49478|  %210 = select i1 %209, i64 5, i64 0                                                                                   ;L374
 49479|  br label %211                                                                                                         ;L374
 49480| 
 49481| 211: ; preds = %203, %200, %173
 49482|  %212 = phi i64 [ %210, %203 ], [ 0, %200 ], [ 0, %173 ]                                                               ;L0
 49484|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %11)
 49485|  to label %215 unwind label %213                                                                                       ;L825<382
 49486| 
 49487| 213: ; preds = %211
 49488|  %214 = cleanuppad within none []
 49490|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %11) [ "funclet"(token %214) ] ;L825<825<382
 49491|  cleanupret from %214 unwind to caller                                                                                 ;L825<382
 49492| 
 49493| 215: ; preds = %211
 49495|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %11)   ;L825<825<382
 49497|  br label %97                                                                                                          ;L360
 49498| 
 49499| 216: ; preds = %67
 49500|     ;; t = ptr %75
 49501|     ;; self = ptr %24
 49502|  %217 = gep %24, i64 1216                                                                                              ;L742<335
 49503|  %218 = load i32, ptr %217, , !!8                                                                                      ;L742<335
 49504|  %219 = icmp eq i32 %218, -1                                                                                           ;L742<335
 49505|  br i1 %219, label %225, label %220                                                                                    ;L742<335
 49506| 
 49507| 220: ; preds = %216
 49508|  %221 = gep %24, i64 1168                                                                                              ;L742<335
 49509|     ;; self = ptr %221
 49510|     ;; effect = ptr %221
 49511|  %222 = gep %24, i64 1392                                                                                              ;L1494<336
 49512|  %223 = tail call i64 @gc::simulation6entityNtB5_6Entity17attack_speed_mult(ptr %24)                                   ;L336
 49513|  %224 = tail call i64 @ai::action_score22calculate_action_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %222, ptr %221, i64 %223, ptr %75, i8 2, ptr %7) ;L336
 49514|  br label %97                                                                                                          ;L334
 49515| 
 49516| 225: ; preds = %216
 49517|     ;; self = ptr null
 49518|  tail call void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.199) #35                       ;L1013<335
 49519|  unreachable                                                                                                           ;L1013<335
 49520| 
 49521| 226: ; preds = %77
 49522|     ;; t = ptr %85
 49523|     ;; self = ptr %24
 49524|  %227 = gep %24, i64 1272                                                                                              ;L742<343
 49525|  %228 = load i32, ptr %227, , !!8                                                                                      ;L742<343
 49526|  %229 = icmp eq i32 %228, -1                                                                                           ;L742<343
 49527|  br i1 %229, label %235, label %230                                                                                    ;L742<343
 49528| 
 49529| 230: ; preds = %226
 49530|  %231 = gep %24, i64 1224                                                                                              ;L742<343
 49531|     ;; self = ptr %231
 49532|     ;; effect = ptr %231
 49533|  %232 = gep %24, i64 1408                                                                                              ;L1665<344
 49534|  %233 = tail call i64 @gc::simulation6entityNtB5_6Entity15cooldown_reduce(ptr %24, i1 zeroext false)                   ;L344
 49535|  %234 = tail call i64 @ai::action_score22calculate_action_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %232, ptr %231, i64 %233, ptr %85, i8 2, ptr %7) ;L344
 49536|  br label %97                                                                                                          ;L342
 49537| 
 49538| 235: ; preds = %226
 49539|     ;; self = ptr null
 49540|  tail call void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.200) #35                       ;L1013<343
 49541|  unreachable                                                                                                           ;L1013<343
 49542| 
 49543| 236: ; preds = %87
 49544|     ;; t = ptr %95
 49545|  %237 = gep %24, i64 1480                                                                                              ;L1693<351
 49546|  %238 = load i64, ptr %237, , !!8                                                                                      ;L1693<351
 49547|  %239 = icmp ugt i64 %238, 2                                                                                           ;L1693<351
 49548|  br i1 %239, label %240, label %244                                                                                    ;L1693<351
 49549| 
 49550| 240: ; preds = %236
 49551|     ;; self = ptr %24
 49552|  %241 = gep %24, i64 1328                                                                                              ;L742<351
 49553|  %242 = load i32, ptr %241, , !!8                                                                                      ;L742<351
 49554|  %243 = icmp eq i32 %242, -1                                                                                           ;L742<351
 49555|  br i1 %243, label %244, label %245                                                                                    ;L742<351
 49556| 
 49557| 244: ; preds = %240, %236
 49558|  tail call void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.201) #35                       ;L1013<351
 49559|  unreachable                                                                                                           ;L1013<351
 49560| 
 49561| 245: ; preds = %240
 49562|  %246 = gep %24, i64 1280                                                                                              ;L1694<351
 49563|     ;; self = ptr %246
 49564|     ;; self = ptr %246
 49565|     ;; effect = ptr %246
 49566|  %247 = gep %24, i64 1424                                                                                              ;L1670<352
 49567|  %248 = tail call i64 @gc::simulation6entityNtB5_6Entity15cooldown_reduce(ptr %24, i1 zeroext false)                   ;L352
 49568|  %249 = tail call i64 @ai::action_score22calculate_action_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %247, ptr %246, i64 %248, ptr %95, i8 2, ptr %7) ;L352
 49569|  br label %97                                                                                                          ;L350
 49570| }
