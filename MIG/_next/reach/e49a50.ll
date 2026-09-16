 10120| define void @ai::plan_legacy7handlerNtB2_17LegacyPlanHandler14update_on_dead(ptr %0, i64 %1, ptr %2, ptr %3, ptr %4, ptr %5) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 10121|  %7 = alloca [24 x i8],
 10122|  %8 = alloca [24 x i8],
 10126|  %9 = alloca [24 x i8],
 10128|  %10 = alloca [40 x i8],
 10129|  %11 = alloca [24 x i8],
 10130|  %12 = alloca [28 x i8],
 10131|  %13 = alloca [32 x i8],
 10132|  %14 = alloca [28 x i8],
 10133|     ;; src[12..+28] = ptr %14
 10134|     ;; value[12..+28] = ptr %14
 10135|     ;; value[12..+28] = ptr %14
 10137|  %15 = alloca [24 x i8],
 10138|  %16 = alloca [392 x i8],
 10139|  %17 = alloca [384 x i8],
 10140|     ;; self = ptr %0
 10141|     ;; version = i64 %1
 10142|     ;; rnd = ptr %2
 10143|     ;; player = ptr %3
 10144|     ;; data = ptr %4
 10145|     ;; debug = ptr %5
 10146|     ;; mf_p = ptr %17
 10147|     ;; handled_chats = ptr %15
 10148|     ;; iter = ptr %13
 10149|     ;; c = ptr %11
 10150|     ;; iter = ptr %10
 10151|     ;; iterator = ptr %10
 10152|     ;; value = ptr %9
 10153|     ;; count = i64 1
 10155|     ;; elem_size = i64 40
 10156|     ;; count = i64 1
 10157|  %18 = load ptr, ptr %4,                                                                                               ;L636
 10158|  %19 = gep %4, i64 8                                                                                                   ;L636
 10159|  %20 = load ptr, ptr %19, , !!8, !!8                                                                                   ;L636
 10160|  tail call fastcc void @ai::plan_legacy7handlerNtB2_17LegacyPlanHandler19sanitize_rule_scope(ptr %0, ptr %18, ptr %20) ;L636
 10161|  %21 = gep %0, i64 6165                                                                                                ;L639
 10162|  store i8 0, ptr %21,                                                                                                  ;L639
 10163|  %22 = gep %0, i64 6144                                                                                                ;L640
 10164|  store i8 0, ptr %22,                                                                                                  ;L640
 10165|  %23 = gep %0, i64 1512                                                                                                ;L642
 10166|     ;; self = ptr %23
 10167|  %24 = load i64, ptr %23, , !!8                                                                                        ;L254<642
 10168|  %25 = icmp ne i64 %24, 6                                                                                              ;L254<642
 10169|  tail call void @llvm.assume(i1 %25)                                                                                   ;L254<642
 10170|  %26 = add nsw i64 %24, -2                                                                                             ;L254<642
 10171|  %27 = icmp samesign ugt i64 %24, 1                                                                                    ;L254<642
 10172|  %28 = select i1 %27, i64 %26, i64 4                                                                                   ;L254<642
 10173|  switch i64 %28, label %35 [
 10174|  i64 1, label %92
 10175|  i64 2, label %92
 10176|  i64 5, label %29
 10177|  ]                                                                                                                     ;L254<642
 10178| 
 10179| 29: ; preds = %6
 10180|     ;; plan = ptr %23
 10181|     ;; self = ptr %23
 10182|  %30 = gep %0, i64 1592                                                                                                ;L104<257<642
 10183|  %31 = load i64, ptr %30, , !!8                                                                                        ;L104<257<642
 10184|  %32 = gep %0, i64 1600                                                                                                ;L104<257<642
 10185|  %33 = load i64, ptr %32, , !!8                                                                                        ;L104<257<642
 10186|  %34 = icmp eq i64 %31, %33                                                                                            ;L104<257<642
 10187|  br i1 %34, label %92, label %35                                                                                       ;L642
 10188| 
 10189| 35: ; preds = %29, %6
 10190|  %36 = load ptr, ptr %18, , !!8, !!8                                                                                   ;L643
 10191|  %37 = gep %18, i64 8                                                                                                  ;L643
 10192|  %38 = load ptr, ptr %37, , !!8, !!8                                                                                   ;L643
 10193|  %39 = gep %38, i64 40                                                                                                 ;L643
 10194|  %40 = load ptr, ptr %39, , !!8                                                                                        ;L643
 10195|  %41 = tail call i64 %40(ptr %36)                                                                                      ;L643
 10196|     ;; self = ptr %0
 10197|     ;; src = i8 16
 10198|     ;; tick = i64 %41
 10199|  %42 = load i64, ptr %23, , !!8                                                                                        ;L401<643
 10200|  %43 = icmp ne i64 %42, 6                                                                                              ;L401<643
 10201|  tail call void @llvm.assume(i1 %43)                                                                                   ;L401<643
 10202|  %44 = icmp eq i64 %42, 9                                                                                              ;L401<643
 10203|  br i1 %44, label %45, label %81                                                                                       ;L401<643
 10204| 
 10205| 45: ; preds = %35
 10206|     ;; b = ptr %0
 10207|  %46 = gep %0, i64 1783                                                                                                ;L402<643
 10208|  %47 = load i8, ptr %46, , !!8                                                                                         ;L402<643
 10209|  %48 = gep %0, i64 1793                                                                                                ;L402<643
 10210|  %49 = load i8, ptr %48, , !!8                                                                                         ;L402<643
 10211|  %50 = gep %0, i64 1785                                                                                                ;L402<643
 10212|  %51 = load i8, ptr %50, , !!8                                                                                         ;L402<643
 10213|  %52 = gep %0, i64 5624                                                                                                ;L402<643
 10214|  %53 = gep %0, i64 5632                                                                                                ;L402<643
 10215|  store i8 16, ptr %53,                                                                                                 ;L402<643
 10216|  %54 = gep %0, i64 5633                                                                                                ;L402<643
 10217|  store i8 %47, ptr %54,                                                                                                ;L402<643
 10218|  store i64 %41, ptr %52,                                                                                               ;L402<643
 10219|  %55 = gep %0, i64 5634                                                                                                ;L402<643
 10220|  store i8 %49, ptr %55,                                                                                                ;L402<643
 10221|  %56 = gep %0, i64 5635                                                                                                ;L402<643
 10222|  store i8 %51, ptr %56,                                                                                                ;L402<643
 10223|  %57 = gep %0, i64 1792                                                                                                ;L403<643
 10224|  %58 = load i8, ptr %57, , !!8                                                                                         ;L403<643
 10225|  %59 = gep %0, i64 1786                                                                                                ;L403<643
 10226|  %60 = load i8, ptr %59, , !!8                                                                                         ;L403<643
 10227|  %61 = gep %0, i64 1787                                                                                                ;L403<643
 10228|  %62 = load i8, ptr %61, , !!8                                                                                         ;L403<643
 10229|  %63 = gep %0, i64 1788                                                                                                ;L404<643
 10230|  %64 = load i8, ptr %63, , !!8                                                                                         ;L404<643
 10231|  %65 = gep %0, i64 1789                                                                                                ;L404<643
 10232|  %66 = load i8, ptr %65, , !!8                                                                                         ;L404<643
 10233|  %67 = gep %0, i64 1790                                                                                                ;L404<643
 10234|  %68 = load i8, ptr %67, , !!8                                                                                         ;L404<643
 10235|  %69 = gep %0, i64 1791                                                                                                ;L404<643
 10236|  %70 = load i8, ptr %69, , !!8                                                                                         ;L404<643
 10237|  %71 = gep %0, i64 1779                                                                                                ;L404<643
 10238|  %72 = load i8, ptr %71, , !!8                                                                                         ;L404<643
 10239|  %73 = gep %0, i64 5640                                                                                                ;L403<643
 10240|  store i8 %58, ptr %73,                                                                                                ;L403<643
 10241|  %74 = gep %0, i64 5641                                                                                                ;L403<643
 10242|  store i8 %60, ptr %74,                                                                                                ;L403<643
 10243|  %75 = gep %0, i64 5642                                                                                                ;L403<643
 10244|  store i8 %62, ptr %75,                                                                                                ;L403<643
 10245|  %76 = gep %0, i64 5643                                                                                                ;L403<643
 10246|  store i8 %64, ptr %76,                                                                                                ;L403<643
 10247|  %77 = gep %0, i64 5644                                                                                                ;L403<643
 10248|  store i8 %66, ptr %77,                                                                                                ;L403<643
 10249|  %78 = gep %0, i64 5645                                                                                                ;L403<643
 10250|  store i8 %68, ptr %78,                                                                                                ;L403<643
 10251|  %79 = gep %0, i64 5646                                                                                                ;L403<643
 10252|  store i8 %70, ptr %79,                                                                                                ;L403<643
 10253|  %80 = gep %0, i64 5647                                                                                                ;L403<643
 10254|  store i8 %72, ptr %80,                                                                                                ;L403<643
 10255|  br label %81                                                                                                          ;L401<643
 10256| 
 10257| 81: ; preds = %45, %35
 10259|  call fastcc void @ai::plan_legacy7handlerNtB2_17LegacyPlanHandler12passive_plan(ptr %16, ptr %0, i64 %1, ptr %2, ptr %3, ptr %4, ptr %5) ;L644
 10261|  call void @llvm.memcpy.p0.p0.i64(ptr %17, ptr %16, i64 384, i1 false)                                                 ;L644
 10264|  %82 = invoke i64 %40(ptr %36)
 10265|  to label %86 unwind label %83                                                                                         ;L645
 10266| 
 10267| 83: ; preds = %89, %81
 10268|  %84 = phi i1 [ false, %89 ], [ true, %81 ]                                                                            ;L0
 10269|  %85 = cleanuppad within none []
 10270|  br i1 %84, label %104, label %103                                                                                     ;L647
 10271| 
 10272| 86: ; preds = %81
 10273|     ;; self = ptr %0
 10274|     ;; src = i8 24
 10275|     ;; tick = i64 %82
 10276|  %87 = gep %0, i64 5648                                                                                                ;L410<645
 10277|  store i8 24, ptr %87,                                                                                                 ;L410<645
 10278|  %88 = gep %0, i64 5656                                                                                                ;L410<645
 10279|  store i64 %82, ptr %88,                                                                                               ;L410<645
 10280|  invoke fastcc void @core::ptr9drop_glueNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy5types7BigPlanEBH_(ptr %23)
 10281|  to label %91 unwind label %89                                                                                         ;L646
 10282| 
 10283| 89: ; preds = %86
 10284|  %90 = cleanuppad within none []
 10285|  call void @llvm.memcpy.p0.p0.i64(ptr %23, ptr %17, i64 384, i1 false)                                                 ;L646
 10286|  cleanupret from %90 unwind label %83                                                                                  ;L646
 10287| 
 10288| 91: ; preds = %86
 10289|  call void @llvm.memcpy.p0.p0.i64(ptr %23, ptr %17, i64 384, i1 false)                                                 ;L646
 10291|  br label %92                                                                                                          ;L642
 10292| 
 10293| 92: ; preds = %91, %29, %6, %6
 10295|  store i64 0, ptr %15,                                                                                                 ;L464<649
 10296|  %93 = gep %15, i64 8                                                                                                  ;L464<649
 10297|  store ptr inttoptr (i64 8 to ptr), ptr %93,                                                                           ;L464<649
 10298|  %94 = gep %15, i64 16                                                                                                 ;L464<649
 10299|  store i64 0, ptr %94,                                                                                                 ;L464<649
 10300|     ;; self = ptr %0
 10301|     ;; self = ptr %0
 10302|     ;; self = ptr %0
 10303|  %95 = gep %0, i64 2040                                                                                                ;L614<609<296<1968<1864<3787<650
 10304|  %96 = gep %0, i64 2048                                                                                                ;L614<609<296<1968<1864<3787<650
 10305|  %97 = load ptr, ptr %96, , !!8, !!8                                                                                   ;L614<609<296<1968<1864<3787<650
 10306|  %98 = gep %0, i64 2056                                                                                                ;L1864<3787<650
 10307|  %99 = load i64, ptr %98, , !!8                                                                                        ;L1864<3787<650
 10308|     ;; len = i64 %99
 10309|     ;; count = i64 %99
 10310|     ;; self[0..+8] = ptr %97
 10311|     ;; slice[0..+8] = ptr %97
 10312|     ;; self[8..+8] = i64 %99
 10313|     ;; slice[8..+8] = i64 %99
 10314|     ;; ptr = ptr %97
 10315|     ;; self = ptr %97
 10316|  %100 = getelementptr { i64, i32, [1 x i32], { i8, [23 x i8] } }, ptr %97, i64 %99                                     ;L961<100<1042<650
 10317|     ;; iter[0..+8] = ptr %97
 10318|     ;; iter[8..+8] = ptr %100
 10319|  %101 = gep %18, i64 8
 10320|  %102 = gep %14, i64 4
 10321|  br label %105                                                                                                         ;L650
 10322| 
 10323| 103: ; preds = %83
 10324|  cleanupret from %85 unwind to caller
 10325| 
 10326| 104: ; preds = %83
 10327|  call fastcc void @core::ptr9drop_glueNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy5types7BigPlanEBH_(ptr %17) #31 [ "funclet"(token %85) ] ;L647
 10328|  cleanupret from %85 unwind to caller                                                                                  ;L647
 10329| 
 10330| 105: ; preds = %132, %92
 10331|  %106 = phi ptr [ %97, %92 ], [ %109, %132 ]                                                                           ;L650
 10332|     ;; iter[0..+8] = ptr %106
 10333|     ;; self = ptr undef
 10334|     ;; ptr = ptr %106
 10335|     ;; self = ptr %106
 10336|     ;; end_or_len = ptr %100
 10339|  %107 = icmp eq ptr %106, %100                                                                                         ;L1714<180<650
 10340|  br i1 %107, label %116, label %108                                                                                    ;L180<650
 10341| 
 10342| 108: ; preds = %105
 10343|  %109 = gep %106, i64 40                                                                                               ;L656<185<650
 10344|     ;; iter[0..+8] = ptr %109
 10345|     ;; tick = ptr %106
 10346|     ;; from = ptr %106
 10347|     ;; chat = ptr %106
 10348|  %110 = load i64, ptr %106, , !!8                                                                                      ;L651
 10349|  %111 = load ptr, ptr %18, , !!8, !!8                                                                                  ;L651
 10350|  %112 = load ptr, ptr %101, , !!8, !!8                                                                                 ;L651
 10351|  %113 = gep %112, i64 40                                                                                               ;L651
 10352|  %114 = load ptr, ptr %113, , !!8                                                                                      ;L651
 10353|  %115 = invoke i64 %114(ptr %111)
 10354|  to label %122 unwind label %119                                                                                       ;L651
 10355| 
 10356| 116: ; preds = %105
 10357|  %117 = load ptr, ptr %18, , !!8, !!8                                                                                  ;L655
 10358|  %118 = load ptr, ptr %101, , !!8, !!8                                                                                 ;L655
 10359|  invoke void @_RINvMs_NtCs9LexZzt9XJB_5alloc3vecINtB5_3VecTjNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity8PositionNtNtNtBL_5state6player4ChatEE6retainNCNvMNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handlerNtB2i_17LegacyPlanHandler14update_on_dead0EB2m_(ptr %95, ptr %117, ptr %118)
 10360|  to label %139 unwind label %119                                                                                       ;L655
 10361| 
 10362| 119: ; preds = %211, %206, %196, %190, %176, %175, %174, %171, %169, %167, %166, %153, %131, %116, %108
 10363|  %120 = phi i1 [ false, %153 ], [ false, %211 ], [ false, %196 ], [ false, %206 ], [ true, %108 ], [ false, %190 ], [ false, %176 ], [ false, %175 ], [ false, %174 ], [ false, %171 ], [ false, %169 ], [ false, %167 ], [ false, %166 ], [ true, %116 ], [ true, %131 ] ;L0
 10364|  %121 = cleanuppad within none []
 10365|  br i1 %120, label %223, label %222                                                                                    ;L679
 10366| 
 10367| 122: ; preds = %108
 10368|  %123 = icmp ugt i64 %110, %115                                                                                        ;L651
 10369|  br i1 %123, label %132, label %124                                                                                    ;L651
 10370| 
 10371| 124: ; preds = %122
 10372|     ;; self = ptr %15
 10374|  %125 = gep %106, i64 8                                                                                                ;L652
 10375|  %126 = load i32, ptr %125, , !!8                                                                                      ;L652
 10376|  %127 = gep %106, i64 16                                                                                               ;L652
 10377|     ;; value[0..+8] = i64 %110
 10378|     ;; value[0..+8] = i64 %110
 10379|     ;; src[0..+8] = i64 %110
 10380|     ;; value[8..+4] = i32 %126
 10381|     ;; value[8..+4] = i32 %126
 10382|     ;; src[8..+4] = i32 %126
 10383|  call void @llvm.memcpy.p0.p0.i64(ptr %102, ptr %127, i64 24, i1 false)                                                ;L652
 10384|     ;; self = ptr %15
 10385|     ;; self = ptr %15
 10386|     ;; elem_size = i64 40
 10387|  %128 = load i64, ptr %94, , !!20604, !!8                                                                              ;L1037<1004<652
 10388|     ;; len = i64 %128
 10389|     ;; count = i64 %128
 10390|     ;; self = ptr %15
 10391|  %129 = load i64, ptr %15, , !!20604, !!8                                                                              ;L619<309<1040<1004<652
 10392|  %130 = icmp eq i64 %128, %129                                                                                         ;L1040<1004<652
 10393|  br i1 %130, label %131, label %133                                                                                    ;L1040<1004<652
 10394| 
 10395| 131: ; preds = %124
 10396|  invoke void @gc::simulation6entity8PositionNtNtNtBS_5state6player4ChatEE8grow_oneCshdEBA0ozCnw_7game_ai(ptr %15)
 10397|  to label %133 unwind label %119                                                                                       ;L1041<1004<652
 10398| 
 10399| 132: ; preds = %133, %122
 10400|  br label %105                                                                                                         ;L650
 10401| 
 10402| 133: ; preds = %131, %124
 10403|  %134 = load ptr, ptr %93, , !!20604, !!8, !!8                                                                         ;L614<609<296<2052<1044<1004<652
 10404|     ;; self = ptr %134
 10405|  %135 = getelementptr { i64, i32, [1 x i32], { i8, [23 x i8] } }, ptr %134, i64 %128                                   ;L961<1044<1004<652
 10406|     ;; end = ptr %135
 10407|     ;; dst = ptr %135
 10408|  store i64 %110, ptr %135,                                                                                             ;L1933<1045<1004<652
 10409|  %136 = gep %135, i64 8                                                                                                ;L1933<1045<1004<652
 10410|  store i32 %126, ptr %136,                                                                                             ;L1933<1045<1004<652
 10411|  %137 = gep %135, i64 12                                                                                               ;L1933<1045<1004<652
 10412|  call void @llvm.memcpy.p0.p0.i64(ptr %137, ptr %14, i64 28, i1 false)                                                 ;L1933<1045<1004<652
 10413|  %138 = add i64 %128, 1                                                                                                ;L1046<1004<652
 10414|  store i64 %138, ptr %94, , !!20604                                                                                    ;L1046<1004<652
 10416|  br label %132                                                                                                         ;L651
 10417| 
 10418| 139: ; preds = %116
 10419|  %140 = load i64, ptr %15,                                                                                             ;L656
 10420|     ;; self[0..+8] = i64 %140
 10421|  %141 = load ptr, ptr %93, , !!8, !!8                                                                                  ;L656
 10422|     ;; self[8..+8] = ptr %141
 10423|  %142 = load i64, ptr %94,                                                                                             ;L656
 10424|     ;; self[16..+8] = i64 %142
 10425|     ;; me[0..+8] = i64 %140
 10426|     ;; me[8..+8] = ptr %141
 10427|     ;; me[16..+8] = i64 %142
 10428|     ;; buf = ptr %141
 10429|     ;; begin = ptr %141
 10430|     ;; self = ptr %141
 10431|     ;; self = ptr undef
 10432|     ;; count = i64 %142
 10433|  %143 = icmp ult i64 %142, 230584300921369396                                                                          ;L3059<3961<656
 10434|  call void @llvm.assume(i1 %143)                                                                                       ;L3059<3961<656
 10435|  %144 = mul nuw nsw i64 %142, 40                                                                                       ;L961<3961<656
 10436|  %145 = gep %141, i64 %144                                                                                             ;L961<3961<656
 10437|     ;; end = ptr %145
 10438|     ;; self = ptr undef
 10439|     ;; self = ptr undef
 10440|     ;; self = i64 %140
 10441|  %146 = icmp sgt i64 %140, -1                                                                                          ;L49<619<309<3963<656
 10442|  call void @llvm.assume(i1 %146)                                                                                       ;L49<619<309<3963<656
 10444|  store ptr %141, ptr %13,                                                                                              ;L656
 10445|  %147 = gep %13, i64 8                                                                                                 ;L656
 10446|  store ptr %141, ptr %147,                                                                                             ;L656
 10447|  %148 = gep %13, i64 16                                                                                                ;L656
 10448|  store i64 %140, ptr %148,                                                                                             ;L656
 10449|  %149 = gep %13, i64 24                                                                                                ;L656
 10450|  store ptr %145, ptr %149,                                                                                             ;L656
 10452|  %150 = icmp eq i64 %142, 0                                                                                            ;L1714<262<656
 10453|  br i1 %150, label %166, label %151                                                                                    ;L262<656
 10454| 
 10455| 151: ; preds = %139
 10456|  %152 = gep %12, i64 4
 10457|  br label %155                                                                                                         ;L262<656
 10458| 
 10459| 153: ; preds = %213
 10460|  %154 = cleanuppad within none []
 10462|  call void @gc::simulation6entity8PositionNtNtNtB12_5state6player4ChatEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %13) [ "funclet"(token %154) ] ;L825<661
 10463|  cleanupret from %154 unwind label %119                                                                                ;L661
 10464| 
 10465| 155: ; preds = %215, %151
 10466|  %156 = phi ptr [ %145, %151 ], [ %217, %215 ]
 10467|  %157 = phi ptr [ %141, %151 ], [ %216, %215 ]
 10469|     ;; ptr = ptr %157
 10470|     ;; old = ptr %157
 10471|     ;; self = ptr %157
 10472|     ;; self = ptr %157
 10473|  %158 = gep %157, i64 40                                                                                               ;L656<266<656
 10474|  store ptr %158, ptr %147, , !!20696                                                                                   ;L266<656
 10475|  %159 = load i64, ptr %157, , !!20675                                                                                  ;L1733<987<269<656
 10476|  %160 = gep %157, i64 8                                                                                                ;L1733<987<269<656
 10477|  %161 = load i32, ptr %160, , !!20675                                                                                  ;L1733<987<269<656
 10478|  %162 = gep %157, i64 12                                                                                               ;L1733<987<269<656
 10479|  call void @llvm.memcpy.p0.p0.i64(ptr %12, ptr %162, i64 28, i1 false), !!20675                                        ;L1733<987<269<656
 10480|  %163 = icmp eq i32 %161, -1                                                                                           ;L656
 10481|  br i1 %163, label %166, label %164                                                                                    ;L656
 10482| 
 10483| 164: ; preds = %155
 10484|     ;; tick = i64 %159
 10485|     ;; from = i32 %161
 10487|  call void @llvm.memcpy.p0.p0.i64(ptr %11, ptr %152, i64 24, i1 false)                                                 ;L656
 10488|  %165 = call zeroext i1 @ai::plan_legacy10rule_scope12chat_allowed(ptr %20, ptr %11)                                   ;L657
 10489|  br i1 %165, label %213, label %215                                                                                    ;L657
 10490| 
 10491| 166: ; preds = %215, %155, %139
 10494|  invoke void @gc::simulation6entity8PositionNtNtNtB12_5state6player4ChatEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %13)
 10495|  to label %167 unwind label %119                                                                                       ;L825<661
 10496| 
 10497| 167: ; preds = %166
 10499|  %168 = gep %0, i64 1968                                                                                               ;L662
 10500|  invoke void @_RINvMs_NtCs9LexZzt9XJB_5alloc3vecINtB5_3VecTjNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity8PositionNtNtNtBL_5state6player4ChatEE6retainNCNvMNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handlerNtB2i_17LegacyPlanHandler14update_on_deads_0EB2m_(ptr %168, ptr %117, ptr %118)
 10501|  to label %169 unwind label %119                                                                                       ;L662
 10502| 
 10503| 169: ; preds = %167
 10504|  %170 = gep %0, i64 248                                                                                                ;L665
 10505|  invoke void @ai::plan_legacy9team_planNtB5_8TeamPlan6update(ptr %170, i64 %1, ptr %2, ptr %3, ptr %4, ptr %5)
 10506|  to label %171 unwind label %119                                                                                       ;L665
 10507| 
 10508| 171: ; preds = %169
 10509|  %172 = gep %0, i64 1992                                                                                               ;L666
 10510|     ;; self = ptr %172
 10511|     ;; self = ptr %172
 10513|  %173 = gep %0, i64 440                                                                                                ;L666
 10514|  invoke void @core::ops5range9RangeFullECshdEBA0ozCnw_7game_ai(ptr sret([40 x i8]) %10, ptr %173)
 10515|  to label %174 unwind label %119                                                                                       ;L666
 10516| 
 10517| 174: ; preds = %171
 10518|  invoke void @_RINvMsj_NtCs9LexZzt9XJB_5alloc3vecINtB6_3VecNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player4ChatE14extend_trustedINtNtB6_5drain5DrainBG_EECshdEBA0ozCnw_7game_ai(ptr %172, ptr %10)
 10519|  to label %175 unwind label %119                                                                                       ;L27<3994<666
 10520| 
 10521| 175: ; preds = %174
 10523|  invoke void @ai::goal_dataNtB2_8GoalData6update(ptr %0, i64 %1, ptr %2, ptr %3, ptr %4, ptr %5)
 10524|  to label %176 unwind label %119                                                                                       ;L667
 10525| 
 10526| 176: ; preds = %175
 10527|  invoke fastcc void @ai::plan_legacy7handlerNtB2_17LegacyPlanHandler19sanitize_rule_scope(ptr %0, ptr %18, ptr %20)
 10528|  to label %177 unwind label %119                                                                                       ;L668
 10529| 
 10530| 177: ; preds = %176
 10531|     ;; self = ptr %0
 10532|     ;; self = ptr %0
 10533|     ;; self = ptr %0
 10534|  %178 = gep %0, i64 2016                                                                                               ;L614<609<296<1968<1864<3787<670
 10535|  %179 = gep %0, i64 2024                                                                                               ;L614<609<296<1968<1864<3787<670
 10536|  %180 = load ptr, ptr %179, , !!8, !!8                                                                                 ;L614<609<296<1968<1864<3787<670
 10537|  %181 = gep %0, i64 2032                                                                                               ;L1864<3787<670
 10538|  %182 = load i64, ptr %181, , !!8                                                                                      ;L1864<3787<670
 10539|     ;; len = i64 %182
 10540|     ;; count = i64 %182
 10541|     ;; self[0..+8] = ptr %180
 10542|     ;; slice[0..+8] = ptr %180
 10543|     ;; self[8..+8] = i64 %182
 10544|     ;; slice[8..+8] = i64 %182
 10545|     ;; ptr = ptr %180
 10546|     ;; self = ptr %180
 10547|  %183 = getelementptr { i64, { i8, [23 x i8] } }, ptr %180, i64 %182                                                   ;L961<100<1042<670
 10548|     ;; iter[0..+8] = ptr %180
 10549|     ;; iter[8..+8] = ptr %183
 10550|  %184 = gep %118, i64 40
 10551|  %185 = gep %0, i64 2008
 10552|  %186 = gep %0, i64 2000
 10553|  br label %187                                                                                                         ;L670
 10554| 
 10555| 187: ; preds = %201, %177
 10556|  %188 = phi ptr [ %180, %177 ], [ %191, %201 ]                                                                         ;L670
 10557|     ;; iter[0..+8] = ptr %188
 10558|     ;; self = ptr undef
 10559|     ;; ptr = ptr %188
 10560|     ;; self = ptr %188
 10561|     ;; end_or_len = ptr %183
 10564|  %189 = icmp eq ptr %188, %183                                                                                         ;L1714<180<670
 10565|  br i1 %189, label %196, label %190                                                                                    ;L180<670
 10566| 
 10567| 190: ; preds = %187
 10568|  %191 = gep %188, i64 32                                                                                               ;L656<185<670
 10569|     ;; iter[0..+8] = ptr %191
 10570|     ;; tick = ptr %188
 10571|  %192 = gep %188, i64 8                                                                                                ;L670
 10572|     ;; chat = ptr %192
 10573|     ;; self = ptr %192
 10574|  %193 = load i64, ptr %188, , !!8                                                                                      ;L671
 10575|  %194 = load ptr, ptr %184, , !!8                                                                                      ;L671
 10576|  %195 = invoke i64 %194(ptr %117)
 10577|  to label %197 unwind label %119                                                                                       ;L671
 10578| 
 10579| 196: ; preds = %187
 10580|  invoke void @_RINvMs_NtCs9LexZzt9XJB_5alloc3vecINtB5_3VecTjNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player4ChatEE6retainNCNvMNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handlerNtB1V_17LegacyPlanHandler14update_on_deads0_0EB1Z_(ptr %178, ptr %117, ptr %118)
 10581|  to label %211 unwind label %119                                                                                       ;L677
 10582| 
 10583| 197: ; preds = %190
 10584|  %198 = icmp ugt i64 %193, %195                                                                                        ;L671
 10585|  br i1 %198, label %201, label %199                                                                                    ;L671
 10586| 
 10587| 199: ; preds = %197
 10588|  %200 = call zeroext i1 @ai::plan_legacy10rule_scope12chat_allowed(ptr %20, ptr %192)                                  ;L672
 10589|  br i1 %200, label %202, label %201                                                                                    ;L672
 10590| 
 10591| 201: ; preds = %207, %199, %197
 10592|  br label %187                                                                                                         ;L670
 10593| 
 10594| 202: ; preds = %199
 10595|     ;; self = ptr %172
 10597|  call void @llvm.memcpy.p0.p0.i64(ptr %9, ptr %192, i64 24, i1 false)                                                  ;L737<673
 10598|     ;; self = ptr %172
 10599|     ;; self = ptr %172
 10600|     ;; value = ptr %9
 10601|     ;; src = ptr %9
 10602|     ;; elem_size = i64 24
 10603|  %203 = load i64, ptr %185, , !!20884, !!8                                                                             ;L1037<1004<673
 10604|     ;; len = i64 %203
 10605|     ;; count = i64 %203
 10606|     ;; self = ptr %172
 10607|  %204 = load i64, ptr %172, , !!20884, !!8                                                                             ;L619<309<1040<1004<673
 10608|  %205 = icmp eq i64 %203, %204                                                                                         ;L1040<1004<673
 10609|  br i1 %205, label %206, label %207                                                                                    ;L1040<1004<673
 10610| 
 10611| 206: ; preds = %202
 10612|  invoke void @gc::simulation5state6player4ChatE8grow_oneCshdEBA0ozCnw_7game_ai(ptr %172)
 10613|  to label %207 unwind label %119                                                                                       ;L1041<1004<673
 10614| 
 10615| 207: ; preds = %206, %202
 10616|  %208 = load ptr, ptr %186, , !!20884, !!8, !!8                                                                        ;L614<609<296<2052<1044<1004<673
 10617|     ;; self = ptr %208
 10618|  %209 = gepS %208, i64 %203                                                                                            ;L961<1044<1004<673
 10619|     ;; end = ptr %209
 10620|     ;; dst = ptr %209
 10621|  call void @llvm.memcpy.p0.p0.i64(ptr %209, ptr %9, i64 24, i1 false)                                                  ;L1933<1045<1004<673
 10622|  %210 = add i64 %203, 1                                                                                                ;L1046<1004<673
 10623|  store i64 %210, ptr %185, , !!20884                                                                                   ;L1046<1004<673
 10625|  br label %201                                                                                                         ;L672
 10626| 
 10627| 211: ; preds = %196
 10628|  invoke fastcc void @ai::plan_legacy7handlerNtB2_17LegacyPlanHandler19sanitize_rule_scope(ptr %0, ptr %18, ptr %20)
 10629|  to label %212 unwind label %119                                                                                       ;L678
 10630| 
 10631| 212: ; preds = %211
 10633|  ret void                                                                                                              ;L679
 10634| 
 10635| 213: ; preds = %164
 10637|  call void @llvm.memcpy.p0.p0.i64(ptr %8, ptr %152, i64 24, i1 false)                                                  ;L658
 10638|  %214 = call fastcc zeroext i1 @ai::plan_legacy7handlerNtB2_17LegacyPlanHandler32take_misunderstood_received_chat(ptr %0, i64 %159, i32 %161, ptr %8) ;L658
 10640|     ;; misunderstood = i1 %214
 10642|  call void @llvm.memcpy.p0.p0.i64(ptr %7, ptr %152, i64 24, i1 false)                                                  ;L659
 10643|  invoke void @ai::plan_legacy7handler4chatNtB4_17LegacyPlanHandler11handle_chat(ptr %0, i64 %1, ptr %2, ptr %3, ptr %4, i32 %161, ptr %7, i1 zeroext %214, ptr %5)
 10644|  to label %219 unwind label %153                                                                                       ;L659
 10645| 
 10646| 215: ; preds = %219, %164
 10647|  %216 = phi ptr [ %158, %164 ], [ %221, %219 ]                                                                         ;L1714<262<656
 10648|  %217 = phi ptr [ %156, %164 ], [ %220, %219 ]                                                                         ;L28<656
 10652|     ;; self = ptr %13
 10653|     ;; count = i64 1
 10654|     ;; self = ptr %13
 10656|  %218 = icmp eq ptr %216, %217                                                                                         ;L1714<262<656
 10657|  br i1 %218, label %166, label %155                                                                                    ;L262<656
 10658| 
 10659| 219: ; preds = %213
 10661|  %220 = load ptr, ptr %149, , !!20696                                                                                  ;L28<656
 10662|  %221 = load ptr, ptr %147, , !!20696                                                                                  ;L1714<262<656
 10663|  br label %215                                                                                                         ;L657
 10664| 
 10665| 222: ; preds = %119
 10666|  cleanupret from %121 unwind to caller
 10667| 
 10668| 223: ; preds = %119
 10669|  call fastcc void @core::ptr9drop_glueINtNtCs9LexZzt9XJB_5alloc3vec3VecTjNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity8PositionNtNtNtB1e_5state6player4ChatEEECshdEBA0ozCnw_7game_ai(ptr %15) #31 [ "funclet"(token %121) ] ;L679
 10670|  cleanupret from %121 unwind to caller                                                                                 ;L679
 10671| }
