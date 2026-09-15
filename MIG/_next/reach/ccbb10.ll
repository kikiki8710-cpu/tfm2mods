 48427| define i64 @ai::plan_legacy8sub_plan9line_safeNtB2_15LineSafeSubPlan5score(ptr %0, i64 %1, ptr %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %7) unnamed_addr #1 {
 48428|  %9 = alloca [40 x i8],
 48429|     ;; self = ptr %0
 48430|     ;; version = i64 %1
 48431|     ;; parameter = ptr %2
 48432|     ;; rnd = ptr %3
 48433|     ;; player = ptr %4
 48434|     ;; data = ptr %5
 48435|     ;; action = ptr %6
 48436|     ;; debug = ptr %7
 48437|     ;; action_type = i8 0
 48439|  %10 = load i8, ptr %0, , !!8                                                                                          ;L105
 48440|  %11 = gep %9, i64 16                                                                                                  ;L105
 48441|  store i8 1, ptr %11,                                                                                                  ;L105
 48442|  %12 = gep %9, i64 17                                                                                                  ;L105
 48443|  store i8 %10, ptr %12,                                                                                                ;L105
 48444|  store i64 2, ptr %9,                                                                                                  ;L105
 48445|  %13 = call { i64, i64 } @ai::plan_legacy11action_eval15evaluate_action(i64 %1, ptr %9, ptr %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %7) ;L105
 48446|  %14 = extractvalue { i64, i64 } %13, 0                                                                                ;L105
 48447|  %15 = trunc nuw i64 %14 to i1                                                                                         ;L105
 48448|  br i1 %15, label %16, label %18                                                                                       ;L105
 48449| 
 48450| 16: ; preds = %8
 48451|  %17 = extractvalue { i64, i64 } %13, 1                                                                                ;L105
 48452|     ;; v = i64 %17
 48454|  br label %22                                                                                                          ;L167
 48455| 
 48456| 18: ; preds = %8
 48458|  %19 = gep %4, i64 2352                                                                                                ;L108
 48459|  %20 = load i64, ptr %19, , !!8                                                                                        ;L108
 48460|  %21 = icmp ult i64 %20, 2                                                                                             ;L108
 48461|  br i1 %21, label %25, label %24                                                                                       ;L108
 48462| 
 48463| 22: ; preds = %86, %16
 48464|  %23 = phi i64 [ %17, %16 ], [ %89, %86 ]                                                                              ;L0
 48465|     ;; v = i64 %23
 48466|  ret i64 %23                                                                                                           ;L167
 48467| 
 48468| 24: ; preds = %18
 48469|  call void @core::panicking18panic_bounds_check(i64 %20, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.220) #31    ;L108
 48470|  unreachable                                                                                                           ;L108
 48471| 
 48472| 25: ; preds = %18
 48473|     ;; self = ptr %4
 48474|  %26 = gep %4, i64 2496                                                                                                ;L581<108
 48475|  %27 = load i32, ptr %26, , !!8                                                                                        ;L581<108
 48476|  %28 = zext nneg i32 %27 to i64                                                                                        ;L581<108
 48477|  %29 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L108
 48478|  %30 = gep %29, i64 480                                                                                                ;L108
 48479|  %31 = getelementptr [5 x ptr], ptr %30, i64 %20                                                                       ;L108
 48480|  %32 = getelementptr ptr, ptr %31, i64 %28                                                                             ;L108
 48481|  %33 = load ptr, ptr %32, , !!8                                                                                        ;L108
 48482|     ;; self = ptr %33
 48483|  %34 = icmp eq ptr %33, null                                                                                           ;L1011<108
 48484|  br i1 %34, label %45, label %35                                                                                       ;L1011<108
 48485| 
 48486| 35: ; preds = %25
 48487|     ;; champ = ptr %33
 48488|     ;; self = ptr %33
 48489|     ;; self = ptr %33
 48490|     ;; self = ptr %33
 48491|     ;; self = ptr %33
 48492|     ;; other = ptr %33
 48493|  %36 = call i64 @ai::action_score17interaction_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %6, ptr %7)           ;L109
 48494|     ;; base = i64 %36
 48495|  %37 = call i64 @ai::lane_economy30line_action_economy_adjustment(i64 %1, ptr %4, ptr %5, ptr %2, ptr %6, i8 0)        ;L113
 48496|     ;; economy_adjustment = i64 %37
 48497|     ;; self = ptr %6
 48498|  %38 = gep %6, i64 177                                                                                                 ;L309<114
 48499|  %39 = load i8, ptr %38, , !!54137, !!8                                                                                ;L309<114
 48500|  %40 = icmp ne i8 %39, 10                                                                                              ;L309<114
 48501|  call void @llvm.assume(i1 %40)                                                                                        ;L309<114
 48502|  %41 = add nsw i8 %39, -3                                                                                              ;L309<114
 48503|  %42 = icmp samesign ugt i8 %39, 2                                                                                     ;L309<114
 48504|  %43 = select i1 %42, i8 %41, i8 7                                                                                     ;L309<114
 48505|  switch i8 %43, label %44 [
 48506|  i8 0, label %86
 48507|  i8 1, label %86
 48508|  i8 2, label %46
 48509|  i8 3, label %46
 48510|  i8 4, label %86
 48511|  i8 5, label %86
 48512|  i8 6, label %86
 48513|  i8 7, label %86
 48514|  i8 8, label %86
 48515|  i8 9, label %86
 48516|  i8 10, label %46
 48517|  i8 11, label %86
 48518|  i8 12, label %56
 48519|  i8 13, label %66
 48520|  i8 14, label %76
 48521|  i8 15, label %86
 48522|  i8 16, label %86
 48523|  ]                                                                                                                     ;L309<114
 48524| 
 48525| 44: ; preds = %35
 48526|  unreachable                                                                                                           ;L309<114
 48527| 
 48528| 45: ; preds = %25
 48529|  call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.221) #31                            ;L1013<108
 48530|  unreachable                                                                                                           ;L1013<108
 48531| 
 48532| 46: ; preds = %35, %35, %35
 48533|  %47 = gep %6, i64 8                                                                                                   ;L0<114
 48534|  %48 = load i64, ptr %47, , !!54137, !!8                                                                               ;L0<114
 48535|     ;; target_id = i64 %48
 48536|  %49 = load ptr, ptr %29, , !!8, !!8                                                                                   ;L142
 48537|  %50 = gep %29, i64 8                                                                                                  ;L142
 48538|  %51 = load ptr, ptr %50, , !!8, !!8                                                                                   ;L142
 48539|  %52 = gep %51, i64 496                                                                                                ;L142
 48540|  %53 = load ptr, ptr %52, , !!8                                                                                        ;L142
 48541|  %54 = call ptr %53(ptr %49, i64 %48)                                                                                  ;L142
 48542|  %55 = icmp eq ptr %54, null                                                                                           ;L142
 48543|  br i1 %55, label %86, label %90                                                                                       ;L142
 48544| 
 48545| 56: ; preds = %35
 48546|     ;; action = ptr %6
 48547|     ;; self = ptr %6
 48548|  %57 = gep %6, i64 8                                                                                                   ;L94<322<114
 48549|  %58 = load i64, ptr %57, , !!54137, !!8                                                                               ;L94<322<114
 48550|     ;; target_id = i64 %58
 48551|  %59 = load ptr, ptr %29, , !!8, !!8                                                                                   ;L118
 48552|  %60 = gep %29, i64 8                                                                                                  ;L118
 48553|  %61 = load ptr, ptr %60, , !!8, !!8                                                                                   ;L118
 48554|  %62 = gep %61, i64 496                                                                                                ;L118
 48555|  %63 = load ptr, ptr %62, , !!8                                                                                        ;L118
 48556|  %64 = call ptr %63(ptr %59, i64 %58)                                                                                  ;L118
 48557|  %65 = icmp eq ptr %64, null                                                                                           ;L118
 48558|  br i1 %65, label %86, label %128                                                                                      ;L118
 48559| 
 48560| 66: ; preds = %35
 48561|     ;; action = ptr %6
 48562|     ;; self = ptr %6
 48563|  %67 = gep %6, i64 8                                                                                                   ;L160<323<114
 48564|  %68 = load i64, ptr %67, , !!54137, !!8                                                                               ;L160<323<114
 48565|     ;; target_id = i64 %68
 48566|  %69 = load ptr, ptr %29, , !!8, !!8                                                                                   ;L126
 48567|  %70 = gep %29, i64 8                                                                                                  ;L126
 48568|  %71 = load ptr, ptr %70, , !!8, !!8                                                                                   ;L126
 48569|  %72 = gep %71, i64 496                                                                                                ;L126
 48570|  %73 = load ptr, ptr %72, , !!8                                                                                        ;L126
 48571|  %74 = call ptr %73(ptr %69, i64 %68)                                                                                  ;L126
 48572|  %75 = icmp eq ptr %74, null                                                                                           ;L126
 48573|  br i1 %75, label %86, label %138                                                                                      ;L126
 48574| 
 48575| 76: ; preds = %35
 48576|     ;; action = ptr %6
 48577|     ;; self = ptr %6
 48578|  %77 = gep %6, i64 8                                                                                                   ;L222<324<114
 48579|  %78 = load i64, ptr %77, , !!54137, !!8                                                                               ;L222<324<114
 48580|     ;; target_id = i64 %78
 48581|  %79 = load ptr, ptr %29, , !!8, !!8                                                                                   ;L134
 48582|  %80 = gep %29, i64 8                                                                                                  ;L134
 48583|  %81 = load ptr, ptr %80, , !!8, !!8                                                                                   ;L134
 48584|  %82 = gep %81, i64 496                                                                                                ;L134
 48585|  %83 = load ptr, ptr %82, , !!8                                                                                        ;L134
 48586|  %84 = call ptr %83(ptr %79, i64 %78)                                                                                  ;L134
 48587|  %85 = icmp eq ptr %84, null                                                                                           ;L134
 48588|  br i1 %85, label %86, label %148                                                                                      ;L134
 48589| 
 48590| 86: ; preds = %157, %142, %132, %127, %126, %123, %123, %102, %98, %90, %76, %66, %56, %46, %35, %35, %35, %35, %35, %35, %35, %35, %35, %35, %35
 48591|  %87 = phi i64 [ -99999, %66 ], [ -99999, %76 ], [ -99999, %56 ], [ 50, %123 ], [ 0, %127 ], [ 0, %102 ], [ 100, %126 ], [ 0, %98 ], [ 0, %46 ], [ 0, %90 ], [ 50, %123 ], [ %136, %132 ], [ %146, %142 ], [ %161, %157 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ], [ 0, %35 ] ;L0
 48592|  %88 = add i64 %37, %36                                                                                                ;L114
 48593|  %89 = add i64 %88, %87                                                                                                ;L114
 48594|     ;; v = i64 %89
 48595|  br label %22                                                                                                          ;L167
 48596| 
 48597| 90: ; preds = %46
 48598|     ;; t = ptr %54
 48599|     ;; self = ptr %54
 48600|     ;; self = ptr %54
 48601|     ;; other = ptr %33
 48602|  %91 = load i64, ptr %54, , !!8                                                                                        ;L1127<143
 48603|  %92 = gep %54, i64 8                                                                                                  ;L1127<143
 48604|     ;; __self_discr = i64 %91
 48605|  %93 = load i64, ptr %33, , !!8                                                                                        ;L1127<143
 48606|  %94 = gep %33, i64 8                                                                                                  ;L1127<143
 48607|     ;; __arg1_discr = i64 %93
 48608|  %95 = icmp eq i64 %91, %93                                                                                            ;L1127<143
 48609|  br i1 %95, label %96, label %86                                                                                       ;L1127<143
 48610| 
 48611| 96: ; preds = %90
 48612|  %97 = icmp eq i64 %91, 0                                                                                              ;L1127<143
 48613|  br i1 %97, label %98, label %102                                                                                      ;L1127<143
 48614| 
 48615| 98: ; preds = %96
 48616|     ;; __self_0 = ptr %54
 48617|     ;; self = ptr %54
 48618|     ;; __arg1_0 = ptr %33
 48619|     ;; other = ptr %33
 48622|  %99 = load i64, ptr %92, , !!8                                                                                        ;L1878<2123<1127<143
 48623|  %100 = load i64, ptr %94, , !!8                                                                                       ;L1878<2123<1127<143
 48624|  %101 = icmp eq i64 %99, %100                                                                                          ;L1878<2123<1127<143
 48625|  br i1 %101, label %102, label %86                                                                                     ;L143
 48626| 
 48627| 102: ; preds = %98, %96
 48628|  %103 = gep %54, i64 1632                                                                                              ;L2158<144
 48629|  %104 = load i64, ptr %103, , !!8                                                                                      ;L2158<144
 48630|     ;; x1 = i64 %104
 48631|     ;; self = i64 %104
 48632|  %105 = gep %54, i64 1640                                                                                              ;L2158<144
 48633|  %106 = load i64, ptr %105, , !!8                                                                                      ;L2158<144
 48634|     ;; y1 = i64 %106
 48635|     ;; self = i64 %106
 48636|  %107 = gep %33, i64 1632                                                                                              ;L2158<144
 48637|  %108 = load i64, ptr %107, , !!8                                                                                      ;L2158<144
 48638|     ;; x2 = i64 %108
 48639|     ;; other = i64 %108
 48640|  %109 = gep %33, i64 1640                                                                                              ;L2158<144
 48641|  %110 = load i64, ptr %109, , !!8                                                                                      ;L2158<144
 48642|     ;; y2 = i64 %110
 48643|     ;; other = i64 %110
 48644|  %111 = icmp ult i64 %104, %108                                                                                        ;L3147<7<2158<144
 48645|  %112 = sub nuw i64 %108, %104                                                                                         ;L3147<7<2158<144
 48646|  %113 = sub nuw i64 %104, %108                                                                                         ;L3147<7<2158<144
 48647|  %114 = select i1 %111, i64 %112, i64 %113                                                                             ;L3147<7<2158<144
 48648|     ;; dx = i64 %114
 48649|  %115 = icmp ult i64 %106, %110                                                                                        ;L3147<8<2158<144
 48650|  %116 = sub nuw i64 %110, %106                                                                                         ;L3147<8<2158<144
 48651|  %117 = sub nuw i64 %106, %110                                                                                         ;L3147<8<2158<144
 48652|  %118 = select i1 %115, i64 %116, i64 %117                                                                             ;L3147<8<2158<144
 48653|     ;; dy = i64 %118
 48654|  %119 = mul i64 %114, %114                                                                                             ;L9<2158<144
 48655|  %120 = mul i64 %118, %118                                                                                             ;L9<2158<144
 48656|  %121 = add i64 %120, %119                                                                                             ;L9<2158<144
 48657|     ;; dist = i64 %121
 48658|  %122 = icmp ugt i64 %121, 39999999999                                                                                 ;L147
 48659|  br i1 %122, label %123, label %86                                                                                     ;L147
 48660| 
 48661| 123: ; preds = %102
 48662|     ;; self = ptr %54
 48663|  %124 = gep %54, i64 104                                                                                               ;L1261<148
 48664|  %125 = load i64, ptr %124, , !!8                                                                                      ;L1261<148
 48665|  switch i64 %125, label %127 [
 48666|  i64 1, label %86
 48667|  i64 2, label %86
 48668|  i64 3, label %126
 48669|  ]                                                                                                                     ;L148
 48670| 
 48671| 126: ; preds = %123
 48672|  br label %86                                                                                                          ;L150
 48673| 
 48674| 127: ; preds = %123
 48675|  br label %86                                                                                                          ;L150
 48676| 
 48677| 128: ; preds = %56
 48678|     ;; t = ptr %64
 48679|     ;; self = ptr %33
 48680|  %129 = gep %33, i64 1216                                                                                              ;L742<119
 48681|  %130 = load i32, ptr %129, , !!8                                                                                      ;L742<119
 48682|  %131 = icmp eq i32 %130, -1                                                                                           ;L742<119
 48683|  br i1 %131, label %137, label %132                                                                                    ;L742<119
 48684| 
 48685| 132: ; preds = %128
 48686|  %133 = gep %33, i64 1168                                                                                              ;L742<119
 48687|     ;; self = ptr %133
 48688|     ;; effect = ptr %133
 48689|  %134 = gep %33, i64 1392                                                                                              ;L1494<120
 48690|  %135 = call i64 @gc::simulation6entityNtB5_6Entity17attack_speed_mult(ptr %33)                                        ;L120
 48691|  %136 = call i64 @ai::action_score22calculate_action_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %134, ptr %133, i64 %135, ptr %64, i8 0, ptr %7) ;L120
 48692|  br label %86                                                                                                          ;L118
 48693| 
 48694| 137: ; preds = %128
 48695|     ;; self = ptr null
 48696|  call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.222) #31                            ;L1013<119
 48697|  unreachable                                                                                                           ;L1013<119
 48698| 
 48699| 138: ; preds = %66
 48700|     ;; t = ptr %74
 48701|     ;; self = ptr %33
 48702|  %139 = gep %33, i64 1272                                                                                              ;L742<127
 48703|  %140 = load i32, ptr %139, , !!8                                                                                      ;L742<127
 48704|  %141 = icmp eq i32 %140, -1                                                                                           ;L742<127
 48705|  br i1 %141, label %147, label %142                                                                                    ;L742<127
 48706| 
 48707| 142: ; preds = %138
 48708|  %143 = gep %33, i64 1224                                                                                              ;L742<127
 48709|     ;; self = ptr %143
 48710|     ;; effect = ptr %143
 48711|  %144 = gep %33, i64 1408                                                                                              ;L1665<128
 48712|  %145 = call i64 @gc::simulation6entityNtB5_6Entity15cooldown_reduce(ptr %33, i1 zeroext false)                        ;L128
 48713|  %146 = call i64 @ai::action_score22calculate_action_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %144, ptr %143, i64 %145, ptr %74, i8 0, ptr %7) ;L128
 48714|  br label %86                                                                                                          ;L126
 48715| 
 48716| 147: ; preds = %138
 48717|     ;; self = ptr null
 48718|  call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.223) #31                            ;L1013<127
 48719|  unreachable                                                                                                           ;L1013<127
 48720| 
 48721| 148: ; preds = %76
 48722|     ;; t = ptr %84
 48723|  %149 = gep %33, i64 1480                                                                                              ;L1693<135
 48724|  %150 = load i64, ptr %149, , !!8                                                                                      ;L1693<135
 48725|  %151 = icmp ugt i64 %150, 2                                                                                           ;L1693<135
 48726|  br i1 %151, label %152, label %156                                                                                    ;L1693<135
 48727| 
 48728| 152: ; preds = %148
 48729|     ;; self = ptr %33
 48730|  %153 = gep %33, i64 1328                                                                                              ;L742<135
 48731|  %154 = load i32, ptr %153, , !!8                                                                                      ;L742<135
 48732|  %155 = icmp eq i32 %154, -1                                                                                           ;L742<135
 48733|  br i1 %155, label %156, label %157                                                                                    ;L742<135
 48734| 
 48735| 156: ; preds = %152, %148
 48736|  call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.224) #31                            ;L1013<135
 48737|  unreachable                                                                                                           ;L1013<135
 48738| 
 48739| 157: ; preds = %152
 48740|  %158 = gep %33, i64 1280                                                                                              ;L1694<135
 48741|     ;; self = ptr %158
 48742|     ;; self = ptr %158
 48743|     ;; effect = ptr %158
 48744|  %159 = gep %33, i64 1424                                                                                              ;L1670<136
 48745|  %160 = call i64 @gc::simulation6entityNtB5_6Entity15cooldown_reduce(ptr %33, i1 zeroext false)                        ;L136
 48746|  %161 = call i64 @ai::action_score22calculate_action_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %159, ptr %158, i64 %160, ptr %84, i8 0, ptr %7) ;L136
 48747|  br label %86                                                                                                          ;L134
 48748| }
