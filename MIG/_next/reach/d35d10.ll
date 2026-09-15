 44657| define void @ai::abstract_input6attack(ptr sret([32 x i8]) %0, i64 %1, ptr %2, ptr %3, ptr %4, ptr %5, ptr %6) unnamed_addr #1 {
 44658|  %8 = alloca [24 x i8],
 44659|     ;; version = i64 %1
 44660|     ;; rnd = ptr %2
 44661|     ;; player = ptr %3
 44662|     ;; data = ptr %4
 44663|     ;; positioning_score = ptr %5
 44664|     ;; target = ptr %6
 44665|     ;; self = ptr %6
 44666|     ;; self = ptr %6
 44667|     ;; input_target = ptr %8
 44671|  %9 = gep %3, i64 2352                                                                                                 ;L149
 44672|  %10 = load i64, ptr %9, , !!8                                                                                         ;L149
 44673|  %11 = icmp ult i64 %10, 2                                                                                             ;L149
 44674|  br i1 %11, label %13, label %12                                                                                       ;L149
 44675| 
 44676| 12: ; preds = %7
 44677|  tail call void @core::panicking18panic_bounds_check(i64 %10, i64 2, ptr @anon.168add0ea037d45d276f5936ae758fe5.189) #30 ;L149
 44678|  unreachable                                                                                                           ;L149
 44679| 
 44680| 13: ; preds = %7
 44681|     ;; self = ptr %3
 44682|  %14 = gep %3, i64 2496                                                                                                ;L581<149
 44683|  %15 = load i32, ptr %14, , !!8                                                                                        ;L581<149
 44684|  %16 = zext nneg i32 %15 to i64                                                                                        ;L581<149
 44685|  %17 = load ptr, ptr %4, , !!8, !!8                                                                                    ;L149
 44686|  %18 = gep %17, i64 480                                                                                                ;L149
 44687|  %19 = getelementptr [5 x ptr], ptr %18, i64 %10                                                                       ;L149
 44688|  %20 = getelementptr ptr, ptr %19, i64 %16                                                                             ;L149
 44689|  %21 = load ptr, ptr %20, , !!8                                                                                        ;L149
 44690|     ;; self = ptr %21
 44691|  %22 = icmp eq ptr %21, null                                                                                           ;L2775<149
 44692|  br i1 %22, label %25, label %23                                                                                       ;L2775<149
 44693| 
 44694| 23: ; preds = %13
 44695|     ;; champ = ptr %21
 44696|     ;; entity = ptr %21
 44697|     ;; caster = ptr %21
 44698|     ;; self = ptr %21
 44699|  %24 = tail call zeroext i1 @gc::simulation6entityNtB5_6Entity10can_attack(ptr %21)                                    ;L150
 44700|  br i1 %24, label %28, label %27                                                                                       ;L150
 44701| 
 44702| 25: ; preds = %13
 44703|  store i64 -1, ptr %0,                                                                                                 ;L2790<149
 44704|  br label %26                                                                                                          ;L1
 44705| 
 44706| 26: ; preds = %47, %37, %36, %27, %25
 44707|  ret void                                                                                                              ;L188
 44708| 
 44709| 27: ; preds = %23
 44710|  store i64 -1, ptr %0,                                                                                                 ;L151
 44711|  br label %26                                                                                                          ;L1
 44712| 
 44713| 28: ; preds = %23
 44714|     ;; self = ptr %21
 44715|  %29 = gep %21, i64 1168                                                                                               ;L742<154
 44716|  %30 = gep %21, i64 1216                                                                                               ;L742<154
 44717|  %31 = load i32, ptr %30, , !!8                                                                                        ;L742<154
 44718|  %32 = icmp eq i32 %31, -1                                                                                             ;L742<154
 44719|  br i1 %32, label %36, label %33                                                                                       ;L742<154
 44720| 
 44721| 33: ; preds = %28
 44722|     ;; effect = ptr %29
 44723|     ;; self = ptr %29
 44724|     ;; self = ptr %21
 44725|  %34 = gep %21, i64 1208                                                                                               ;L156
 44726|  %35 = tail call zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %34, ptr %21, ptr %6)            ;L156
 44727|  br i1 %35, label %38, label %37                                                                                       ;L156
 44728| 
 44729| 36: ; preds = %28
 44730|  store i64 -1, ptr %0,                                                                                                 ;L2790<154
 44731|  br label %26                                                                                                          ;L1
 44732| 
 44733| 37: ; preds = %33
 44734|  store i64 -1, ptr %0,                                                                                                 ;L157
 44735|  br label %26                                                                                                          ;L1
 44736| 
 44737| 38: ; preds = %33
 44739|  %39 = tail call i64 @gc::simulation6entityNtB5_6Entity17attack_speed_mult(ptr %21)                                    ;L160
 44740|  call fastcc void @ai::abstract_input16get_input_target(ptr %8, i64 %1, ptr %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %29, i64 %39) ;L160
 44741|  %40 = load i32, ptr %8, , !!8                                                                                         ;L162
 44742|  %41 = icmp eq i32 %40, -1                                                                                             ;L162
 44743|  br i1 %41, label %44, label %42                                                                                       ;L162
 44744| 
 44745| 42: ; preds = %38
 44746|  %43 = gep %0, i64 8                                                                                                   ;L163
 44747|  call void @llvm.memcpy.p0.p0.i64(ptr %43, ptr %8, i64 24, i1 false)                                                   ;L162
 44748|  store i64 2, ptr %0,                                                                                                  ;L163
 44749|  br label %47                                                                                                          ;L162
 44750| 
 44751| 44: ; preds = %38
 44752|     ;; self = ptr %21
 44753|  %45 = load i64, ptr %21, , !!8                                                                                        ;L1136<1482<165
 44754|  %46 = trunc nuw i64 %45 to i1                                                                                         ;L1136<1482<165
 44755|  br i1 %46, label %63, label %48                                                                                       ;L1136<1482<165
 44756| 
 44757| 47: ; preds = %142, %58, %42
 44759|  br label %26                                                                                                          ;L188
 44760| 
 44761| 48: ; preds = %44
 44762|  %49 = gep %21, i64 8                                                                                                  ;L1136<1482<165
 44763|     ;; team = ptr %21
 44764|  %50 = load i64, ptr %49, , !!8                                                                                        ;L1137<1482<165
 44765|     ;; team = i64 %50
 44766|  %51 = icmp ult i64 %50, 2                                                                                             ;L1483<165
 44767|  br i1 %51, label %52, label %57                                                                                       ;L1483<165
 44768| 
 44769| 52: ; preds = %48
 44771|  %53 = gep %6, i64 56                                                                                                  ;L122<1483<165
 44772|  %54 = gepS %53, i64 %50                                                                                               ;L122<1483<165
 44773|  %55 = load i64, ptr %54, , !!8                                                                                        ;L122<1483<165
 44774|  %56 = icmp eq i64 %55, 0                                                                                              ;L122<1483<165
 44775|  br i1 %56, label %63, label %58                                                                                       ;L165
 44776| 
 44777| 57: ; preds = %48
 44778|  tail call void @core::panicking18panic_bounds_check(i64 %50, i64 2, ptr @anon.168add0ea037d45d276f5936ae758fe5.39) #30 ;L1483<165
 44779|  unreachable                                                                                                           ;L1483<165
 44780| 
 44781| 58: ; preds = %52
 44782|  %59 = gep %6, i64 1632                                                                                                ;L185
 44783|  %60 = load i64, ptr %59, , !!8                                                                                        ;L185
 44784|  %61 = gep %6, i64 1640                                                                                                ;L185
 44785|  %62 = load i64, ptr %61, , !!8                                                                                        ;L185
 44786|  tail call void @ai::abstract_input29safe_move_avoiding_enemy_well(ptr sret([32 x i8]) %0, i64 %1, ptr %3, ptr %4, ptr %21, i64 %60, i64 %62) ;L185
 44787|  br label %47                                                                                                          ;L165
 44788| 
 44789| 63: ; preds = %52, %44
 44790|  %64 = gep %21, i64 1632                                                                                               ;L166
 44791|  %65 = load i64, ptr %64, , !!8                                                                                        ;L166
 44792|  %66 = gep %6, i64 1632                                                                                                ;L166
 44793|  %67 = load i64, ptr %66, , !!8                                                                                        ;L166
 44794|  %68 = sub i64 %65, %67                                                                                                ;L166
 44795|     ;; dx = i64 %68
 44796|  %69 = gep %21, i64 1640                                                                                               ;L167
 44797|  %70 = load i64, ptr %69, , !!8                                                                                        ;L167
 44798|  %71 = gep %6, i64 1640                                                                                                ;L167
 44799|  %72 = load i64, ptr %71, , !!8                                                                                        ;L167
 44800|  %73 = sub i64 %70, %72                                                                                                ;L167
 44801|     ;; dy = i64 %73
 44802|  %74 = mul i64 %68, %68                                                                                                ;L168
 44803|  %75 = mul i64 %73, %73                                                                                                ;L168
 44804|  %76 = add i64 %75, %74                                                                                                ;L168
 44805|  %77 = tail call i64 @gc::utils5isqrt(i64 %76)                                                                         ;L168
 44806|     ;; sz = i64 %77
 44807|  %78 = gep %21, i64 1184                                                                                               ;L26<170
 44808|  %79 = load i64, ptr %78, , !!8                                                                                        ;L26<170
 44809|  %80 = gep %21, i64 1192                                                                                               ;L26<170
 44810|  %81 = load i64, ptr %80, , !!8                                                                                        ;L26<170
 44811|  %82 = gep %21, i64 1480                                                                                               ;L26<170
 44812|  %83 = load i64, ptr %82, , !!8                                                                                        ;L26<170
 44813|  %84 = add i64 %83, -1                                                                                                 ;L26<170
 44814|  %85 = mul i64 %84, %81                                                                                                ;L26<170
 44815|  %86 = gep %21, i64 1080                                                                                               ;L26<170
 44816|  %87 = load i64, ptr %86, , !!8                                                                                        ;L26<170
 44817|  %88 = tail call i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %29, ptr %21, ptr %6)                        ;L170
 44818|  %89 = gep %21, i64 1136                                                                                               ;L1511<170
 44819|  %90 = load i32, ptr %89, , !!8                                                                                        ;L1511<170
 44820|     ;; mult = i32 %90
 44821|  %91 = icmp eq i32 %90, 0                                                                                              ;L1512<170
 44822|  br i1 %91, label %92, label %95                                                                                       ;L1512<170
 44823| 
 44824| 92: ; preds = %63
 44825|  %93 = gep %21, i64 1664                                                                                               ;L1513<170
 44826|  %94 = load i64, ptr %93, , !!8                                                                                        ;L1513<170
 44827|  br label %102                                                                                                         ;L1512<170
 44828| 
 44829| 95: ; preds = %63
 44830|  %96 = sext i32 %90 to i64                                                                                             ;L1511<170
 44831|     ;; mult = i64 %96
 44832|  %97 = gep %21, i64 1664                                                                                               ;L1515<170
 44833|  %98 = load i64, ptr %97, , !!8                                                                                        ;L1515<170
 44834|  %99 = add nsw i64 %96, 100                                                                                            ;L1515<170
 44835|  %100 = mul i64 %98, %99                                                                                               ;L1515<170
 44836|  %101 = udiv i64 %100, 100                                                                                             ;L1515<170
 44837|  br label %102                                                                                                         ;L1512<170
 44838| 
 44839| 102: ; preds = %95, %92
 44840|  %103 = phi i64 [ %94, %92 ], [ %101, %95 ]                                                                            ;L0<170
 44841|  %104 = gep %6, i64 1136                                                                                               ;L1511<170
 44842|  %105 = load i32, ptr %104, , !!8                                                                                      ;L1511<170
 44843|     ;; mult = i32 %105
 44844|  %106 = icmp eq i32 %105, 0                                                                                            ;L1512<170
 44845|  br i1 %106, label %107, label %110                                                                                    ;L1512<170
 44846| 
 44847| 107: ; preds = %102
 44848|  %108 = gep %6, i64 1664                                                                                               ;L1513<170
 44849|  %109 = load i64, ptr %108, , !!8                                                                                      ;L1513<170
 44850|  br label %117                                                                                                         ;L1512<170
 44851| 
 44852| 110: ; preds = %102
 44853|  %111 = sext i32 %105 to i64                                                                                           ;L1511<170
 44854|     ;; mult = i64 %111
 44855|  %112 = gep %6, i64 1664                                                                                               ;L1515<170
 44856|  %113 = load i64, ptr %112, , !!8                                                                                      ;L1515<170
 44857|  %114 = add nsw i64 %111, 100                                                                                          ;L1515<170
 44858|  %115 = mul i64 %113, %114                                                                                             ;L1515<170
 44859|  %116 = udiv i64 %115, 100                                                                                             ;L1515<170
 44860|  br label %117                                                                                                         ;L1512<170
 44861| 
 44862| 117: ; preds = %110, %107
 44863|  %118 = phi i64 [ %109, %107 ], [ %116, %110 ]                                                                         ;L0<170
 44864|  %119 = add i64 %87, %79                                                                                               ;L26<170
 44865|  %120 = add i64 %119, %85                                                                                              ;L26<170
 44866|  %121 = add i64 %120, %88                                                                                              ;L170
 44867|  %122 = add i64 %121, %103                                                                                             ;L170
 44868|  %123 = add i64 %122, %118                                                                                             ;L170
 44869|     ;; full_range = i64 %123
 44870|     ;; self = i64 %123
 44871|     ;; self = ptr %6
 44872|  %124 = gep %6, i64 104                                                                                                ;L1386<173
 44873|  %125 = load i64, ptr %124, , !!8                                                                                      ;L1386<173
 44874|  %126 = and i64 %125, 14                                                                                               ;L173
 44875|  %127 = icmp eq i64 %126, 2                                                                                            ;L173
 44876|  %128 = select i1 %127, i64 2000, i64 15000                                                                            ;L173
 44877|     ;; rhs = i64 %128
 44878|     ;; attack_margin = i64 %128
 44879|  %129 = tail call i64 @llvm.usub.sat.i64(i64 %123, i64 %128)                                                           ;L2472<178
 44880|     ;; from_distance = i64 %129
 44881|  %130 = mul i64 %129, %68                                                                                              ;L179
 44882|  %131 = icmp eq i64 %77, 0                                                                                             ;L179
 44883|  br i1 %131, label %136, label %132                                                                                    ;L179
 44884| 
 44885| 132: ; preds = %117
 44886|  %133 = icmp eq i64 %77, -1                                                                                            ;L179
 44887|  %134 = icmp eq i64 %130, -9223372036854775808                                                                         ;L179
 44888|  %135 = and i1 %133, %134                                                                                              ;L179
 44889|  br i1 %135, label %141, label %137                                                                                    ;L179
 44890| 
 44891| 136: ; preds = %117
 44892|  tail call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.168add0ea037d45d276f5936ae758fe5.190) #30 ;L179
 44893|  unreachable                                                                                                           ;L179
 44894| 
 44895| 137: ; preds = %132
 44896|     ;; x = !DIArgList(i64 %67, i64 %130, i64 %77)
 44897|  %138 = mul i64 %129, %73                                                                                              ;L180
 44898|  %139 = icmp eq i64 %138, -9223372036854775808                                                                         ;L180
 44899|  %140 = and i1 %133, %139                                                                                              ;L180
 44900|  br i1 %140, label %156, label %142                                                                                    ;L180
 44901| 
 44902| 141: ; preds = %132
 44903|  tail call void @core::panicking11panic_const24panic_const_div_overflow(ptr @anon.168add0ea037d45d276f5936ae758fe5.190) #30 ;L179
 44904|  unreachable                                                                                                           ;L179
 44905| 
 44906| 142: ; preds = %137
 44907|  %143 = sdiv i64 %130, %77                                                                                             ;L179
 44908|     ;; x = !DIArgList(i64 %67, i64 %143)
 44909|  %144 = add i64 %143, %67                                                                                              ;L179
 44910|     ;; x = i64 %144
 44911|  %145 = sdiv i64 %138, %77                                                                                             ;L180
 44912|  %146 = add i64 %145, %72                                                                                              ;L180
 44913|     ;; y = i64 %146
 44914|  %147 = gep %4, i64 8                                                                                                  ;L181
 44915|  %148 = load ptr, ptr %147, , !!8, !!8                                                                                 ;L181
 44916|  %149 = gep %148, i64 32                                                                                               ;L181
 44917|  %150 = load ptr, ptr %149, , !!8, !!8                                                                                 ;L181
 44918|  %151 = gep %148, i64 8                                                                                                ;L181
 44919|  %152 = load ptr, ptr %151, , !!8, !!8                                                                                 ;L181
 44920|  %153 = tail call { i64, i64 } @gc::simulation4gameNtB5_4Game15adjust_position(ptr %150, ptr %152, i64 %144, i64 %146) ;L181
 44921|  %154 = extractvalue { i64, i64 } %153, 0                                                                              ;L181
 44922|  %155 = extractvalue { i64, i64 } %153, 1                                                                              ;L181
 44923|     ;; x = i64 %154
 44924|     ;; y = i64 %155
 44925|  tail call void @ai::abstract_input29safe_move_avoiding_enemy_well(ptr sret([32 x i8]) %0, i64 %1, ptr %3, ptr %4, ptr %21, i64 %154, i64 %155) ;L183
 44926|  br label %47                                                                                                          ;L165
 44927| 
 44928| 156: ; preds = %137
 44929|  tail call void @core::panicking11panic_const24panic_const_div_overflow(ptr @anon.168add0ea037d45d276f5936ae758fe5.191) #30 ;L180
 44930|  unreachable                                                                                                           ;L180
 44931| }
