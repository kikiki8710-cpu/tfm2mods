 16364| define internal fastcc void @ai::plan_legacy3old12passive_lineNtB2_15PassiveLinePlan10v46_stage1(ptr %0, i64 %1, i64 %2, i32 %3, ptr %4, ptr %5, ptr %6, ptr %7, ptr %8, i64 %9, i1 zeroext %10, ptr %11, i64 %12, ptr %13, i64 %14) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
 16365|  %16 = alloca [8 x i8],
 16366|  %17 = alloca [8 x i8],
 16368|  %18 = alloca [16 x i8],
 16369|  %19 = alloca [16 x i8],
 16371|  %20 = alloca [32 x i8],
 16372|  %21 = alloca [120 x i8],
 16373|  %22 = alloca [160 x i8],
 16374|  %23 = alloca [32 x i8],
 16375|  %24 = alloca [8 x i8],
 16376|  %25 = alloca [8 x i8],
 16377|     ;; only[0..+8] = ptr %11
 16378|     ;; self[0..+8] = ptr %11
 16379|     ;; only[8..+8] = i64 %12
 16380|     ;; self[8..+8] = i64 %12
 16382|     ;; version = i64 %1
 16385|     ;; champ = ptr %6
 16387|     ;; caster = ptr %6
 16388|     ;; self = ptr %6
 16389|     ;; front = ptr %7
 16390|     ;; self = ptr %7
 16391|     ;; my_tower = ptr %8
 16392|     ;; my_hp = i64 %9
 16393|     ;; self = i64 %9
 16394|     ;; range_gate = i1 %10
 16397|     ;; pull = ptr %25
 16398|     ;; rhs = ptr %25
 16399|     ;; tower_disable_tick = ptr %24
 16400|     ;; my_towers = ptr %23
 16401|     ;; self = ptr %21
 16402|     ;; committers = ptr %20
 16403|     ;; iter = ptr %19
 16404|     ;; iter = ptr %18
 16405|     ;; default = i64 0
 16406|     ;; count = i64 1
 16407|     ;; count = i64 1
 16408|     ;; default = i64 0
 16409|  %26 = icmp ne ptr %5, null
 16410|  tail call void @llvm.assume(i1 %26)
 16411|  %27 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L667
 16412|     ;; pool = ptr %27
 16413|     ;; bump = ptr %27
 16414|  %28 = gep %5, i64 8                                                                                                   ;L668
 16415|  %29 = load ptr, ptr %28, , !!8, !!8                                                                                   ;L668
 16416|  %30 = gep %29, i64 4856                                                                                               ;L668
 16417|  %31 = load i64, ptr %30, , !!8                                                                                        ;L668
 16418|     ;; tps = i64 %31
 16420|  %32 = zext nneg i32 %3 to i64                                                                                         ;L581<669
 16421|     ;; my_pos = i64 %32
 16422|  %33 = gep %6, i64 1600                                                                                                ;L670
 16423|  %34 = load i64, ptr %33, , !!8                                                                                        ;L670
 16424|     ;; my_ms = i64 %34
 16425|     ;; self = ptr %6
 16426|  %35 = gep %6, i64 1216                                                                                                ;L742<671
 16427|  %36 = load i32, ptr %35, , !!8                                                                                        ;L742<671
 16428|  %37 = icmp eq i32 %36, -1                                                                                             ;L742<671
 16429|  br i1 %37, label %51, label %38                                                                                       ;L742<671
 16430| 
 16431| 38: ; preds = %15
 16432|     ;; self = ptr %6
 16433|     ;; f = ptr %6
 16434|     ;; x = ptr %6
 16435|     ;; a = ptr %6
 16436|     ;; self = ptr %6
 16437|  %39 = gep %6, i64 1184                                                                                                ;L26<671<1162<671
 16438|  %40 = load i64, ptr %39, , !!8                                                                                        ;L26<671<1162<671
 16439|  %41 = gep %6, i64 1192                                                                                                ;L26<671<1162<671
 16440|  %42 = load i64, ptr %41, , !!8                                                                                        ;L26<671<1162<671
 16441|  %43 = gep %6, i64 1480                                                                                                ;L26<671<1162<671
 16442|  %44 = load i64, ptr %43, , !!8                                                                                        ;L26<671<1162<671
 16443|  %45 = add i64 %44, -1                                                                                                 ;L26<671<1162<671
 16444|  %46 = mul i64 %45, %42                                                                                                ;L26<671<1162<671
 16445|  %47 = gep %6, i64 1080                                                                                                ;L26<671<1162<671
 16446|  %48 = load i64, ptr %47, , !!8                                                                                        ;L26<671<1162<671
 16447|  %49 = add i64 %48, %40                                                                                                ;L26<671<1162<671
 16448|  %50 = add i64 %49, %46                                                                                                ;L26<671<1162<671
 16449|     ;; self[8..+8] = i64 %50
 16450|     ;; self[0..+8] = i64 1
 16451|     ;; my_range = i64 %50
 16452|  br label %51                                                                                                          ;L1043<671
 16453| 
 16454| 51: ; preds = %38, %15
 16455|  %52 = phi i64 [ %50, %38 ], [ 0, %15 ]                                                                                ;L0<671
 16456|     ;; my_range = i64 %52
 16458|  %53 = gep %6, i64 1136                                                                                                ;L1511<672
 16459|  %54 = load i32, ptr %53, , !!8                                                                                        ;L1511<672
 16460|  %55 = sext i32 %54 to i64                                                                                             ;L1511<672
 16461|     ;; mult = i64 %55
 16462|     ;; mult = i64 %55
 16463|  %56 = icmp eq i32 %54, 0                                                                                              ;L1512<672
 16464|  %57 = gep %6, i64 1664                                                                                                ;L0<672
 16465|  %58 = load i64, ptr %57, , !!8                                                                                        ;L0<672
 16466|  br i1 %56, label %63, label %59                                                                                       ;L1512<672
 16467| 
 16468| 59: ; preds = %51
 16469|  %60 = add nsw i64 %55, 100                                                                                            ;L1515<672
 16470|  %61 = mul i64 %58, %60                                                                                                ;L1515<672
 16471|  %62 = udiv i64 %61, 100                                                                                               ;L1515<672
 16472|  br label %63                                                                                                          ;L1512<672
 16473| 
 16474| 63: ; preds = %59, %51
 16475|  %64 = phi i64 [ %62, %59 ], [ %58, %51 ]                                                                              ;L0<672
 16476|  %65 = add i64 %64, %52                                                                                                ;L672
 16477|  %66 = gep %7, i64 1136                                                                                                ;L1511<672
 16478|  %67 = load i32, ptr %66, , !!8                                                                                        ;L1511<672
 16479|     ;; mult = i32 %67
 16480|  %68 = icmp eq i32 %67, 0                                                                                              ;L1512<672
 16481|  br i1 %68, label %69, label %72                                                                                       ;L1512<672
 16482| 
 16483| 69: ; preds = %63
 16484|  %70 = gep %7, i64 1664                                                                                                ;L1513<672
 16485|  %71 = load i64, ptr %70, , !!8                                                                                        ;L1513<672
 16486|  br label %79                                                                                                          ;L1512<672
 16487| 
 16488| 72: ; preds = %63
 16489|  %73 = sext i32 %67 to i64                                                                                             ;L1511<672
 16490|     ;; mult = i64 %73
 16491|  %74 = gep %7, i64 1664                                                                                                ;L1515<672
 16492|  %75 = load i64, ptr %74, , !!8                                                                                        ;L1515<672
 16493|  %76 = add nsw i64 %73, 100                                                                                            ;L1515<672
 16494|  %77 = mul i64 %75, %76                                                                                                ;L1515<672
 16495|  %78 = udiv i64 %77, 100                                                                                               ;L1515<672
 16496|  br label %79                                                                                                          ;L1512<672
 16497| 
 16498| 79: ; preds = %72, %69
 16499|  %80 = phi i64 [ %71, %69 ], [ %78, %72 ]                                                                              ;L0<672
 16500|  %81 = add i64 %65, %80                                                                                                ;L672
 16501|  store i64 %81, ptr %25,                                                                                               ;L672
 16502|  %82 = tail call i64 @gc::simulation6entityNtB5_6Entity15attack_duration(ptr %6)                                       ;L673
 16503|     ;; cs_lock = i64 %82
 16504|  %83 = tail call i64 @gc::simulation6entityNtB5_6Entity8distance(ptr %7, ptr %8)                                       ;L674
 16505|     ;; self = i64 %83
 16506|     ;; self = i64 %34
 16507|     ;; other = i64 1
 16508|  %84 = tail call i64 @llvm.umax.i64(i64 %34, i64 1)                                                                    ;L1039<674
 16509|  %85 = tail call i64 @llvm.usub.sat.i64(i64 %83, i64 %81)                                                              ;L2472<674
 16510|  %86 = udiv i64 %85, %84                                                                                               ;L674
 16511|  %87 = add i64 %86, %82                                                                                                ;L674
 16512|     ;; escape_ticks = i64 %87
 16513|     ;; self = i64 %87
 16515|     ;; self = ptr %5
 16516|  %88 = gep %5, i64 56                                                                                                  ;L295<677
 16517|  %89 = load i8, ptr %88, , !!8                                                                                         ;L295<677
 16518|  switch i8 %89, label %90 [
 16519|  i8 0, label %92
 16520|  i8 1, label %93
 16521|  i8 2, label %92
 16522|  i8 3, label %93
 16523|  i8 4, label %92
 16524|  i8 5, label %91
 16525|  i8 6, label %92
 16526|  i8 7, label %92
 16527|  i8 8, label %92
 16528|  ]                                                                                                                     ;L295<677
 16529| 
 16530| 90: ; preds = %691, %521, %79
 16531|  unreachable
 16532| 
 16533| 91: ; preds = %79
 16534|  br label %93                                                                                                          ;L679
 16535| 
 16536| 92: ; preds = %79, %79, %79, %79, %79, %79
 16537|  br label %93                                                                                                          ;L680
 16538| 
 16539| 93: ; preds = %92, %91, %79, %79
 16540|  %94 = phi i64 [ 5112, %92 ], [ 5128, %91 ], [ 5120, %79 ], [ 5120, %79 ]
 16541|  %95 = gep %29, i64 %94                                                                                                ;L0
 16542|  %96 = load i64, ptr %95, , !!8                                                                                        ;L0
 16543|  store i64 %96, ptr %24,                                                                                               ;L0
 16547|  %97 = icmp ne ptr %4, null
 16548|  tail call void @llvm.assume(i1 %97)
 16549|     ;; self = ptr %4
 16550|     ;; self = ptr %4
 16551|     ;; team = i64 %2
 16552|  call void @gc::simulationNtB5_21AbstractGameWithCache25iter_towers_without_nexus(ptr sret([120 x i8]) %21, ptr %4, i64 %2) ;L684
 16553|  %98 = load ptr, ptr %4, , !!8, !!8                                                                                    ;L685
 16554|  %99 = gep %4, i64 8                                                                                                   ;L685
 16555|  %100 = load ptr, ptr %99, , !!8, !!8                                                                                  ;L685
 16556|     ;; predicate[0..+8] = ptr %98
 16557|     ;; predicate[8..+8] = ptr %100
 16558|     ;; predicate[16..+8] = ptr %24
 16559|     ;; predicate[24..+8] = ptr %7
 16560|     ;; predicate[32..+8] = ptr %25
 16561|  call void @llvm.memcpy.p0.p0.i64(ptr %22, ptr %21, i64 120, i1 false)                                                 ;L28<957<685
 16562|  %101 = gep %22, i64 120                                                                                               ;L28<957<685
 16563|  store ptr %98, ptr %101,                                                                                              ;L28<957<685
 16564|  %102 = gep %22, i64 128                                                                                               ;L28<957<685
 16565|  store ptr %100, ptr %102,                                                                                             ;L28<957<685
 16566|  %103 = gep %22, i64 136                                                                                               ;L28<957<685
 16567|  store ptr %24, ptr %103,                                                                                              ;L28<957<685
 16568|  %104 = gep %22, i64 144                                                                                               ;L28<957<685
 16569|  store ptr %7, ptr %104,                                                                                               ;L28<957<685
 16570|  %105 = gep %22, i64 152                                                                                               ;L28<957<685
 16571|  store ptr %25, ptr %105,                                                                                              ;L28<957<685
 16573|  call void @core::iter8adapters6filter6FilterINtNtB29_5chain5ChainINtNtB29_7flatten7FlattenINtNtNtB2d_5array4iter8IntoIterINtNtB2d_6option6OptionBU_EKj6_EEINtNtB29_6copied6CopiedINtNtNtB2d_5slice4iter4IterBU_EEENCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old12passive_lineNtB5J_15PassiveLinePlan10v46_stage1s_0EEB5P_(ptr sret([32 x i8]) %23, ptr %22, ptr %27) ;L683
 16576|  store ptr inttoptr (i64 8 to ptr), ptr %20,                                                                           ;L547<689
 16577|  %106 = gep %20, i64 8                                                                                                 ;L547<689
 16578|  store ptr %27, ptr %106,                                                                                              ;L547<689
 16579|  %107 = gep %20, i64 16                                                                                                ;L547<689
 16580|  %108 = gep %20, i64 24                                                                                                ;L547<689
 16582|  %109 = icmp ne ptr %13, null
 16583|  call void @llvm.assume(i1 %109)
 16584|     ;; len = i64 %14
 16585|     ;; count = i64 %14
 16586|     ;; self[0..+8] = ptr %13
 16587|     ;; slice[0..+8] = ptr %13
 16588|     ;; self[8..+8] = i64 %14
 16589|     ;; slice[8..+8] = i64 %14
 16590|     ;; ptr = ptr %13
 16591|     ;; self = ptr %13
 16592|  %110 = getelementptr ptr, ptr %13, i64 %14                                                                            ;L961<100<1042<690
 16593|     ;; iter[0..+8] = ptr %13
 16594|     ;; iter[8..+8] = ptr %110
 16595|  %111 = icmp eq ptr %11, null
 16596|  %112 = and i64 %12, 1152921504606846968
 16597|  %113 = getelementptr i64, ptr %11, i64 %112
 16598|  %114 = and i64 %12, 7
 16599|  %115 = getelementptr i64, ptr %113, i64 %114
 16600|  %116 = sub i64 1, %2
 16601|  %117 = icmp ult i64 %116, 2
 16602|  %118 = gep %4, i64 480
 16603|  %119 = getelementptr [5 x ptr], ptr %118, i64 %116
 16604|  %120 = gep %119, i64 40
 16605|  %121 = gep %19, i64 8
 16606|  %122 = gep %19, i64 16
 16607|  %123 = gep %4, i64 488
 16608|  %124 = gep %4, i64 496
 16609|  %125 = gep %4, i64 504
 16610|  %126 = gep %4, i64 512
 16611|  %127 = gep %4, i64 520
 16612|  %128 = gep %4, i64 528
 16613|  %129 = gep %4, i64 536
 16614|  %130 = gep %4, i64 544
 16615|  %131 = gep %4, i64 552
 16616|  %132 = gep %4, i64 560
 16617|  %133 = gep %4, i64 640
 16618|  %134 = icmp ult i64 %2, 2
 16619|  %135 = getelementptr [5 x ptr], ptr %118, i64 %2
 16620|  %136 = gep %135, i64 40
 16621|  %137 = gep %18, i64 8
 16622|  %138 = gep %18, i64 16
 16623|  %139 = gep %23, i64 24
 16624|  %140 = add nsw i64 %55, 100
 16625|  %141 = mul i64 %58, %140
 16626|  %142 = udiv i64 %141, 100
 16627|  call void @llvm.memset.p0.i64(ptr %107, i8 0, i64 16, i1 false)                                                       ;L547<689
 16628|  %143 = select i1 %56, i64 %58, i64 %142
 16629|  br label %144                                                                                                         ;L690
 16630| 
 16631| 144: ; preds = %411, %93
 16632|  %145 = phi ptr [ %13, %93 ], [ %148, %411 ]                                                                           ;L690
 16633|     ;; iter[0..+8] = ptr %145
 16634|     ;; self = ptr undef
 16635|     ;; ptr = ptr %145
 16636|     ;; self = ptr %145
 16637|     ;; end_or_len = ptr %110
 16640|  %146 = icmp eq ptr %145, %110                                                                                         ;L1714<180<690
 16641|  br i1 %146, label %152, label %147                                                                                    ;L180<690
 16642| 
 16643| 147: ; preds = %144
 16644|  %148 = gep %145, i64 8                                                                                                ;L656<185<690
 16645|     ;; iter[0..+8] = ptr %148
 16646|  %149 = load ptr, ptr %145, , !!8, !!8                                                                                 ;L690
 16647|     ;; e = ptr %149
 16648|     ;; f = ptr %149
 16650|     ;; other = ptr %149
 16651|     ;; other = ptr %149
 16652|     ;; entity = ptr %149
 16654|     ;; caster = ptr %149
 16655|     ;; self = ptr %149
 16656|  %150 = gep %149, i64 1472
 16657|  %151 = load i64, ptr %150,                                                                                            ;L0
 16658|  br i1 %111, label %200, label %156                                                                                    ;L659<691
 16659| 
 16660| 152: ; preds = %144
 16661|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %20, i64 32, i1 false)                                                   ;L777
 16664|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %23)
 16665|  to label %155 unwind label %153                                                                                       ;L825<778
 16666| 
 16667| 153: ; preds = %152
 16668|  %154 = cleanuppad within none []
 16670|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %23) [ "funclet"(token %154) ] ;L825<825<778
 16671|  cleanupret from %154 unwind to caller                                                                                 ;L825<778
 16672| 
 16673| 155: ; preds = %152
 16675|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %23)   ;L825<825<778
 16679|  ret void                                                                                                              ;L778
 16680| 
 16681| 156: ; preds = %167, %147
 16682|  %157 = phi i64 [ %199, %167 ], [ %112, %147 ]                                                                         ;L0<2594<691<661<691
 16683|  %158 = phi ptr [ %198, %167 ], [ %11, %147 ]                                                                          ;L0<2594<691<661<691
 16684|     ;; chunks[0..+8] = ptr %158
 16685|     ;; chunks[8..+8] = i64 %157
 16688|     ;; self[8..+8] = i64 %157
 16689|     ;; mid = i64 8
 16690|  %159 = icmp eq i64 %157, 0                                                                                            ;L2155<1895<4241<425<2594<691<661<691
 16691|  br i1 %159, label %160, label %167                                                                                    ;L2155<1895<4241<425<2594<691<661<691
 16692| 
 16693| 160: ; preds = %163, %156
 16694|  %161 = phi ptr [ %164, %163 ], [ %113, %156 ]
 16695|     ;; ptr = ptr %161
 16696|     ;; self = ptr %161
 16697|     ;; end_or_len = ptr %115
 16700|  %162 = icmp eq ptr %161, %115                                                                                         ;L1714<180<331<431<2594<691<661<691
 16701|  br i1 %162, label %411, label %163                                                                                    ;L180<331<431<2594<691<661<691
 16702| 
 16703| 163: ; preds = %160
 16704|  %164 = gep %161, i64 8                                                                                                ;L656<185<331<431<2594<691<661<691
 16705|     ;; x = ptr %161
 16706|  %165 = load i64, ptr %161, , !!29306, !!8                                                                             ;L332<431<2594<691<661<691
 16709|  %166 = icmp eq i64 %165, %151                                                                                         ;L431<332<431<2594<691<661<691
 16710|  br i1 %166, label %200, label %160                                                                                    ;L332<431<2594<691<661<691
 16711| 
 16712| 167: ; preds = %156
 16713|     ;; self[0..+8] = ptr %158
 16714|     ;; self[0..+8] = ptr %158
 16715|     ;; self[8..+8] = i64 %157
 16716|     ;; mid = i64 8
 16717|     ;; count = i64 8
 16718|     ;; len = i64 %157
 16719|     ;; ptr = ptr %158
 16720|     ;; self = ptr %158
 16721|     ;; chunks[0..+8] = ptr %198
 16722|     ;; chunks[8..+8] = i64 %199
 16723|     ;; chunk[0..+8] = ptr %158
 16724|     ;; chunk[8..+8] = i64 8
 16726|     ;; self[0..+8] = ptr %158
 16727|     ;; self[8..+8] = ptr %198
 16728|     ;; init = i1 false
 16729|     ;; len = i64 8
 16731|     ;; i = i64 0
 16732|     ;; self = ptr %158
 16733|     ;; count = i64 0
 16734|  %168 = load i64, ptr %158, , !!29381, !!8                                                                             ;L279<426<2594<691<661<691
 16736|     ;; acc = i1 false
 16738|  %169 = icmp eq i64 %168, %151                                                                                         ;L426<279<426<2594<691<661<691
 16739|     ;; i = i64 1
 16740|     ;; count = i64 1
 16741|  %170 = gep %158, i64 8                                                                                                ;L656<279<426<2594<691<661<691
 16742|  %171 = load i64, ptr %170, , !!29381, !!8                                                                             ;L279<426<2594<691<661<691
 16743|     ;; acc = i1 %169
 16744|  %172 = icmp eq i64 %171, %151                                                                                         ;L426<279<426<2594<691<661<691
 16745|  %173 = or i1 %169, %172                                                                                               ;L426<279<426<2594<691<661<691
 16746|     ;; i = i64 2
 16747|     ;; count = i64 2
 16748|  %174 = gep %158, i64 16                                                                                               ;L656<279<426<2594<691<661<691
 16749|  %175 = load i64, ptr %174, , !!29381, !!8                                                                             ;L279<426<2594<691<661<691
 16750|     ;; acc = i1 %173
 16751|  %176 = icmp eq i64 %175, %151                                                                                         ;L426<279<426<2594<691<661<691
 16752|  %177 = or i1 %173, %176                                                                                               ;L426<279<426<2594<691<661<691
 16753|     ;; i = i64 3
 16754|     ;; count = i64 3
 16755|  %178 = gep %158, i64 24                                                                                               ;L656<279<426<2594<691<661<691
 16756|  %179 = load i64, ptr %178, , !!29381, !!8                                                                             ;L279<426<2594<691<661<691
 16757|     ;; acc = i1 %177
 16758|  %180 = icmp eq i64 %179, %151                                                                                         ;L426<279<426<2594<691<661<691
 16759|  %181 = or i1 %177, %180                                                                                               ;L426<279<426<2594<691<661<691
 16760|     ;; i = i64 4
 16761|     ;; count = i64 4
 16762|  %182 = gep %158, i64 32                                                                                               ;L656<279<426<2594<691<661<691
 16763|  %183 = load i64, ptr %182, , !!29381, !!8                                                                             ;L279<426<2594<691<661<691
 16764|     ;; acc = i1 %181
 16765|  %184 = icmp eq i64 %183, %151                                                                                         ;L426<279<426<2594<691<661<691
 16766|  %185 = or i1 %181, %184                                                                                               ;L426<279<426<2594<691<661<691
 16767|     ;; i = i64 5
 16768|     ;; count = i64 5
 16769|  %186 = gep %158, i64 40                                                                                               ;L656<279<426<2594<691<661<691
 16770|  %187 = load i64, ptr %186, , !!29381, !!8                                                                             ;L279<426<2594<691<661<691
 16771|     ;; acc = i1 %185
 16772|  %188 = icmp eq i64 %187, %151                                                                                         ;L426<279<426<2594<691<661<691
 16773|  %189 = or i1 %185, %188                                                                                               ;L426<279<426<2594<691<661<691
 16774|     ;; i = i64 6
 16775|     ;; count = i64 6
 16776|  %190 = gep %158, i64 48                                                                                               ;L656<279<426<2594<691<661<691
 16777|  %191 = load i64, ptr %190, , !!29381, !!8                                                                             ;L279<426<2594<691<661<691
 16778|     ;; acc = i1 %189
 16779|  %192 = icmp eq i64 %191, %151                                                                                         ;L426<279<426<2594<691<661<691
 16780|  %193 = or i1 %189, %192                                                                                               ;L426<279<426<2594<691<661<691
 16781|     ;; i = i64 7
 16782|     ;; count = i64 7
 16783|  %194 = gep %158, i64 56                                                                                               ;L656<279<426<2594<691<661<691
 16784|  %195 = load i64, ptr %194, , !!29381, !!8                                                                             ;L279<426<2594<691<661<691
 16785|     ;; acc = i1 %193
 16786|  %196 = icmp eq i64 %195, %151                                                                                         ;L426<279<426<2594<691<661<691
 16787|  %197 = or i1 %193, %196                                                                                               ;L426<279<426<2594<691<661<691
 16788|     ;; acc = i1 %197
 16789|     ;; i = i64 8
 16790|  %198 = gep %158, i64 64                                                                                               ;L863<2054<2158<1895<4241<425<2594<691<661<691
 16791|  %199 = add nsw i64 %157, -8                                                                                           ;L2054<2158<1895<4241<425<2594<691<661<691
 16792|  br i1 %197, label %200, label %156                                                                                    ;L426<2594<691<661<691
 16793| 
 16794| 200: ; preds = %167, %163, %147
 16795|  %201 = call fastcc ptr @gc::simulationNtB5_21AbstractGameWithCache21player_by_champion_id(ptr %4, i64 %151)           ;L694
 16796|     ;; self = ptr %201
 16797|  %202 = icmp eq ptr %201, null                                                                                         ;L1011<694
 16798|  br i1 %202, label %217, label %211                                                                                    ;L1011<694
 16799| 
 16800| 203: ; preds = %722, %707, %683, %682, %681, %552, %537, %513, %512, %511, %424, %423, %392, %354, %326, %320, %281, %274, %240, %233, %217, %211
 16801|  %204 = cleanuppad within none []
 16803|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %20) [ "funclet"(token %204) ]
 16804|  to label %207 unwind label %205                                                                                       ;L825<778
 16805| 
 16806| 205: ; preds = %203
 16807|  %206 = cleanuppad within %204 []
 16809|  call void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %20) [ "funclet"(token %206) ]                          ;L825<825<778
 16810|  cleanupret from %206 unwind to caller                                                                                 ;L825<778
 16811| 
 16812| 207: ; preds = %203
 16814|  call void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %20) [ "funclet"(token %204) ]                          ;L825<825<778
 16816|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %23) [ "funclet"(token %204) ]
 16817|  to label %210 unwind label %208                                                                                       ;L825<778
 16818| 
 16819| 208: ; preds = %207
 16820|  %209 = cleanuppad within %204 []
 16822|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %23) [ "funclet"(token %209) ] ;L825<825<778
 16823|  cleanupret from %209 unwind to caller                                                                                 ;L825<778
 16824| 
 16825| 210: ; preds = %207
 16827|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %23) [ "funclet"(token %204) ] ;L825<825<778
 16828|  cleanupret from %204 unwind to caller                                                                                 ;L663
 16829| 
 16830| 211: ; preds = %200
 16831|     ;; ep = ptr %201
 16832|     ;; self = ptr %201
 16833|  %212 = gep %201, i64 2496                                                                                             ;L581<695
 16834|  %213 = load i32, ptr %212, , !!8                                                                                      ;L581<695
 16835|  %214 = zext nneg i32 %213 to i64                                                                                      ;L581<695
 16836|     ;; e_pos = i64 %214
 16837|  %215 = gep %201, i64 384                                                                                              ;L696
 16838|  %216 = invoke i64 @gc::simulation5state6playerNtB4_16AthleteParameter16aggressive_ratio(ptr %215)
 16839|  to label %219 unwind label %203                                                                                       ;L696
 16840| 
 16841| 217: ; preds = %200
 16842|  invoke void @core::option13unwrap_failed(ptr @anon.168add0ea037d45d276f5936ae758fe5.48) #30
 16843|  to label %218 unwind label %203                                                                                       ;L1013<694
 16844| 
 16845| 218: ; preds = %682, %681, %512, %511, %423, %274, %233, %217
 16846|  unreachable
 16847| 
 16848| 219: ; preds = %211
 16849|     ;; aggr = i64 %216
 16850|  %220 = sub i64 1000, %216                                                                                             ;L697
 16851|  %221 = mul i64 %220, 80                                                                                               ;L697
 16852|  %222 = udiv i64 %221, 1000                                                                                            ;L697
 16853|  %223 = add nuw nsw i64 %222, 80                                                                                       ;L697
 16854|     ;; diff_bound = i64 %223
 16855|  %224 = mul i64 %216, 45                                                                                               ;L698
 16856|  %225 = udiv i64 %224, 1000                                                                                            ;L698
 16857|  %226 = add nuw nsw i64 %225, 45                                                                                       ;L698
 16858|     ;; die_tick_bound = i64 %226
 16859|  %227 = mul i64 %216, 35                                                                                               ;L699
 16860|  %228 = udiv i64 %227, 1000                                                                                            ;L699
 16861|  %229 = add nuw nsw i64 %228, 15                                                                                       ;L699
 16862|     ;; tower_tick_bound = i64 %229
 16863|     ;; rhs = i64 %229
 16864|     ;; kill_dps = i64 0
 16865|     ;; kill_nuke = i64 0
 16866|     ;; rhs = i64 0
 16867|     ;; team = i64 %116
 16868|  br i1 %117, label %230, label %233                                                                                    ;L1905<704
 16869| 
 16870| 230: ; preds = %219
 16872|  store ptr %119, ptr %19,                                                                                              ;L704
 16873|  store ptr %120, ptr %121,                                                                                             ;L704
 16874|  %231 = gep %149, i64 1632
 16875|  %232 = gep %149, i64 1640
 16876|  br label %759                                                                                                         ;L704
 16877| 
 16878| 233: ; preds = %219
 16879|  invoke void @core::panicking18panic_bounds_check(i64 %116, i64 2, ptr @anon.168add0ea037d45d276f5936ae758fe5.50) #30
 16880|  to label %218 unwind label %203                                                                                       ;L1905<704
 16881| 
 16882| 234: ; preds = %762, %759
 16883|     ;; kill_dps = i64 %761
 16884|     ;; rhs = i64 %760
 16885|     ;; kill_nuke = i64 %760
 16886|     ;; self = ptr %19
 16889|  store ptr %122, ptr %17, , !!29464
 16890|     ;; self = ptr %19
 16891|     ;; self = ptr %19
 16892|     ;; f = ptr %17
 16893|     ;; count = i64 1
 16894|  %235 = load ptr, ptr %121, , !!29505, !!8, !!8
 16895|  %236 = load ptr, ptr %19, , !!29505
 16896|  br label %237                                                                                                         ;L365<64<704
 16897| 
 16898| 237: ; preds = %243, %234
 16899|  %238 = phi ptr [ %241, %243 ], [ %236, %234 ]
 16900|     ;; ptr = ptr %238
 16901|     ;; self = ptr %238
 16902|     ;; end_or_len = ptr %235
 16905|  %239 = icmp eq ptr %238, %235                                                                                         ;L1714<180<365<64<704
 16906|  br i1 %239, label %264, label %240                                                                                    ;L180<365<64<704
 16907| 
 16908| 240: ; preds = %237
 16909|  %241 = gep %238, i64 8                                                                                                ;L656<185<365<64<704
 16910|  store ptr %241, ptr %19, , !!29505                                                                                    ;L185<365<64<704
 16911|     ;; x = ptr %238
 16912|  %242 = invoke ptr @gc::simulationNtBW_21AbstractGameWithCache14iter_champions0INtB7_5FnMutTRINtNtBb_6option6OptionRNtNtBW_6entity6EntityEEE8call_mutCshdEBA0ozCnw_7game_ai(ptr %17, ptr %238)
 16913|  to label %243 unwind label %203                                                                                       ;L366<64<704
 16914| 
 16915| 243: ; preds = %240
 16916|  %244 = icmp eq ptr %242, null                                                                                         ;L366<64<704
 16917|  br i1 %244, label %237, label %245                                                                                    ;L366<64<704
 16918| 
 16919| 245: ; preds = %243
 16921|     ;; a = ptr %242
 16922|     ;; self = ptr %242
 16923|     ;; self = ptr %242
 16924|     ;; self = ptr %242
 16925|     ;; self = ptr %242
 16926|  %246 = gep %242, i64 1632                                                                                             ;L2158<705
 16927|  %247 = load i64, ptr %246, , !!8                                                                                      ;L2158<705
 16928|     ;; x1 = i64 %247
 16929|     ;; self = i64 %247
 16930|  %248 = gep %242, i64 1640                                                                                             ;L2158<705
 16931|  %249 = load i64, ptr %248, , !!8                                                                                      ;L2158<705
 16932|     ;; y1 = i64 %249
 16933|     ;; self = i64 %249
 16934|  %250 = load i64, ptr %231, , !!8                                                                                      ;L2158<705
 16935|     ;; x2 = i64 %250
 16936|     ;; other = i64 %250
 16937|  %251 = load i64, ptr %232, , !!8                                                                                      ;L2158<705
 16938|     ;; y2 = i64 %251
 16939|     ;; other = i64 %251
 16940|  %252 = icmp ult i64 %247, %250                                                                                        ;L3147<7<2158<705
 16941|  %253 = sub nuw i64 %250, %247                                                                                         ;L3147<7<2158<705
 16942|  %254 = sub nuw i64 %247, %250                                                                                         ;L3147<7<2158<705
 16943|  %255 = select i1 %252, i64 %253, i64 %254                                                                             ;L3147<7<2158<705
 16944|     ;; dx = i64 %255
 16945|  %256 = icmp ult i64 %249, %251                                                                                        ;L3147<8<2158<705
 16946|  %257 = sub nuw i64 %251, %249                                                                                         ;L3147<8<2158<705
 16947|  %258 = sub nuw i64 %249, %251                                                                                         ;L3147<8<2158<705
 16948|  %259 = select i1 %256, i64 %257, i64 %258                                                                             ;L3147<8<2158<705
 16949|     ;; dy = i64 %259
 16950|  %260 = mul i64 %255, %255                                                                                             ;L9<2158<705
 16951|  %261 = mul i64 %259, %259                                                                                             ;L9<2158<705
 16952|  %262 = add i64 %261, %260                                                                                             ;L9<2158<705
 16953|  %263 = icmp ugt i64 %262, 22500000000                                                                                 ;L705
 16954|  br i1 %263, label %762, label %589                                                                                    ;L705
 16955| 
 16956| 264: ; preds = %237
 16959|  %265 = call i64 @llvm.usub.sat.i64(i64 %9, i64 %760)                                                                  ;L2472<720
 16960|  %266 = mul i64 %265, 60                                                                                               ;L720
 16961|     ;; self = i64 %761
 16962|     ;; other = i64 1
 16963|  %267 = call i64 @llvm.umax.i64(i64 %761, i64 1)                                                                       ;L1039<720
 16964|  %268 = udiv i64 %266, %267                                                                                            ;L720
 16965|     ;; my_die = i64 %268
 16966|     ;; det_dps = i64 0
 16967|     ;; det_nuke = i64 0
 16968|     ;; rhs = i64 0
 16969|  br i1 %134, label %269, label %274                                                                                    ;L1905<726
 16970| 
 16971| 269: ; preds = %264
 16973|  store ptr %135, ptr %18,                                                                                              ;L726
 16974|  store ptr %136, ptr %137,                                                                                             ;L726
 16975|  %270 = gep %149, i64 8
 16976|  br label %271                                                                                                         ;L726
 16977| 
 16978| 271: ; preds = %574, %269
 16979|  %272 = phi i64 [ %588, %574 ], [ 0, %269 ]
 16980|  %273 = phi i64 [ %587, %574 ], [ 0, %269 ]
 16981|  br label %275                                                                                                         ;L365<64<726
 16982| 
 16983| 274: ; preds = %264
 16984|  invoke void @core::panicking18panic_bounds_check(i64 %2, i64 2, ptr @anon.168add0ea037d45d276f5936ae758fe5.50) #30
 16985|  to label %218 unwind label %203                                                                                       ;L1905<726
 16986| 
 16987| 275: ; preds = %426, %271
 16988|     ;; det_dps = i64 %273
 16989|     ;; rhs = i64 %272
 16990|     ;; det_nuke = i64 %272
 16991|     ;; self = ptr %18
 16994|  store ptr %138, ptr %16, , !!29666
 16995|     ;; self = ptr %18
 16996|     ;; self = ptr %18
 16997|     ;; f = ptr %16
 16998|     ;; count = i64 1
 16999|  %276 = load ptr, ptr %137, , !!29674, !!8, !!8
 17000|  %277 = load ptr, ptr %18, , !!29674
 17001|  br label %278                                                                                                         ;L365<64<726
 17002| 
 17003| 278: ; preds = %284, %275
 17004|  %279 = phi ptr [ %282, %284 ], [ %277, %275 ]
 17005|     ;; ptr = ptr %279
 17006|     ;; self = ptr %279
 17007|     ;; end_or_len = ptr %276
 17010|  %280 = icmp eq ptr %279, %276                                                                                         ;L1714<180<365<64<726
 17011|  br i1 %280, label %305, label %281                                                                                    ;L180<365<64<726
 17012| 
 17013| 281: ; preds = %278
 17014|  %282 = gep %279, i64 8                                                                                                ;L656<185<365<64<726
 17015|  store ptr %282, ptr %18, , !!29674                                                                                    ;L185<365<64<726
 17016|     ;; x = ptr %279
 17017|  %283 = invoke ptr @gc::simulationNtBW_21AbstractGameWithCache14iter_champions0INtB7_5FnMutTRINtNtBb_6option6OptionRNtNtBW_6entity6EntityEEE8call_mutCshdEBA0ozCnw_7game_ai(ptr %16, ptr %279)
 17018|  to label %284 unwind label %203                                                                                       ;L366<64<726
 17019| 
 17020| 284: ; preds = %281
 17021|  %285 = icmp eq ptr %283, null                                                                                         ;L366<64<726
 17022|  br i1 %285, label %278, label %286                                                                                    ;L366<64<726
 17023| 
 17024| 286: ; preds = %284
 17026|     ;; a = ptr %283
 17027|     ;; self = ptr %283
 17028|     ;; self = ptr %283
 17029|     ;; self = ptr %283
 17030|     ;; self = ptr %283
 17031|     ;; self = ptr %283
 17032|  %287 = gep %283, i64 1632                                                                                             ;L2158<727
 17033|  %288 = load i64, ptr %287, , !!8                                                                                      ;L2158<727
 17034|     ;; x1 = i64 %288
 17035|     ;; self = i64 %288
 17036|  %289 = gep %283, i64 1640                                                                                             ;L2158<727
 17037|  %290 = load i64, ptr %289, , !!8                                                                                      ;L2158<727
 17038|     ;; y1 = i64 %290
 17039|     ;; self = i64 %290
 17040|  %291 = load i64, ptr %231, , !!8                                                                                      ;L2158<727
 17041|     ;; x2 = i64 %291
 17042|     ;; other = i64 %291
 17043|  %292 = load i64, ptr %232, , !!8                                                                                      ;L2158<727
 17044|     ;; y2 = i64 %292
 17045|     ;; other = i64 %292
 17046|  %293 = icmp ult i64 %288, %291                                                                                        ;L3147<7<2158<727
 17047|  %294 = sub nuw i64 %291, %288                                                                                         ;L3147<7<2158<727
 17048|  %295 = sub nuw i64 %288, %291                                                                                         ;L3147<7<2158<727
 17049|  %296 = select i1 %293, i64 %294, i64 %295                                                                             ;L3147<7<2158<727
 17050|     ;; dx = i64 %296
 17051|  %297 = icmp ult i64 %290, %292                                                                                        ;L3147<8<2158<727
 17052|  %298 = sub nuw i64 %292, %290                                                                                         ;L3147<8<2158<727
 17053|  %299 = sub nuw i64 %290, %292                                                                                         ;L3147<8<2158<727
 17054|  %300 = select i1 %297, i64 %298, i64 %299                                                                             ;L3147<8<2158<727
 17055|     ;; dy = i64 %300
 17056|  %301 = mul i64 %296, %296                                                                                             ;L9<2158<727
 17057|  %302 = mul i64 %300, %300                                                                                             ;L9<2158<727
 17058|  %303 = add i64 %302, %301                                                                                             ;L9<2158<727
 17059|  %304 = icmp ugt i64 %303, 22500000000                                                                                 ;L727
 17060|  br i1 %304, label %426, label %412                                                                                    ;L727
 17061| 
 17062| 305: ; preds = %278
 17065|     ;; self = ptr %23
 17066|     ;; self = ptr %23
 17067|  %306 = load ptr, ptr %23, , !!8, !!8                                                                                  ;L138<2073<745
 17068|     ;; p = ptr %306
 17069|  %307 = load i64, ptr %139, , !!8                                                                                      ;L2075<745
 17070|     ;; len = i64 %307
 17071|     ;; count = i64 %307
 17072|     ;; self[0..+8] = ptr %306
 17073|     ;; slice[0..+8] = ptr %306
 17074|     ;; self[8..+8] = i64 %307
 17075|     ;; slice[8..+8] = i64 %307
 17076|     ;; ptr = ptr %306
 17077|     ;; self = ptr %306
 17078|  %308 = getelementptr ptr, ptr %306, i64 %307                                                                          ;L961<100<1042<745
 17079|     ;; iter[0..+8] = ptr %306
 17080|     ;; iter[8..+8] = ptr %308
 17081|  br label %309                                                                                                         ;L745
 17082| 
 17083| 309: ; preds = %323, %305
 17084|  %310 = phi ptr [ %306, %305 ], [ %315, %323 ]                                                                         ;L745
 17085|  %311 = phi i64 [ %272, %305 ], [ %324, %323 ]                                                                         ;L0
 17086|  %312 = phi i64 [ %273, %305 ], [ %325, %323 ]                                                                         ;L0
 17087|     ;; det_dps = i64 %312
 17088|     ;; rhs = i64 %311
 17089|     ;; det_nuke = i64 %311
 17090|     ;; iter[0..+8] = ptr %310
 17091|     ;; self = ptr undef
 17092|     ;; ptr = ptr %310
 17093|     ;; self = ptr %310
 17094|     ;; end_or_len = ptr %308
 17097|  %313 = icmp eq ptr %310, %308                                                                                         ;L1714<180<745
 17098|  br i1 %313, label %334, label %314                                                                                    ;L180<745
 17099| 
 17100| 314: ; preds = %309
 17101|  %315 = gep %310, i64 8                                                                                                ;L656<185<745
 17102|     ;; iter[0..+8] = ptr %315
 17103|  %316 = load ptr, ptr %310, , !!8, !!8                                                                                 ;L745
 17104|     ;; t = ptr %316
 17105|     ;; self = ptr %316
 17106|  %317 = gep %316, i64 1216                                                                                             ;L742<746
 17107|  %318 = load i32, ptr %317, , !!8                                                                                      ;L742<746
 17108|  %319 = icmp eq i32 %318, -1                                                                                           ;L742<746
 17109|  br i1 %319, label %323, label %320                                                                                    ;L742<746
 17110| 
 17111| 320: ; preds = %314
 17112|  %321 = gep %316, i64 1168                                                                                             ;L742<746
 17113|     ;; atk = ptr %321
 17114|  %322 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %321, ptr %5, ptr %316, ptr @anon.168add0ea037d45d276f5936ae758fe5.6, ptr %149)
 17115|  to label %326 unwind label %203                                                                                       ;L747
 17116| 
 17117| 323: ; preds = %328, %314
 17118|  %324 = phi i64 [ %333, %328 ], [ %311, %314 ]                                                                         ;L0
 17119|  %325 = phi i64 [ %332, %328 ], [ %312, %314 ]                                                                         ;L0
 17120|     ;; det_dps = i64 %325
 17121|     ;; rhs = i64 %324
 17122|     ;; det_nuke = i64 %324
 17123|  br label %309                                                                                                         ;L745
 17124| 
 17125| 326: ; preds = %320
 17126|     ;; dmg = i64 %322
 17127|  %327 = invoke i64 @gc::simulation6entityNtB5_6Entity15attack_cooltime(ptr %316)
 17128|  to label %328 unwind label %203                                                                                       ;L748
 17129| 
 17130| 328: ; preds = %326
 17131|  %329 = mul i64 %322, %31                                                                                              ;L748
 17132|     ;; self = i64 %327
 17133|     ;; other = i64 1
 17134|  %330 = call i64 @llvm.umax.i64(i64 %327, i64 1)                                                                       ;L1039<748
 17135|  %331 = udiv i64 %329, %330                                                                                            ;L748
 17136|  %332 = add i64 %331, %312                                                                                             ;L748
 17137|     ;; det_dps = i64 %332
 17138|  %333 = add i64 %322, %311                                                                                             ;L749
 17139|     ;; det_nuke = i64 %333
 17140|     ;; rhs = i64 %333
 17141|  br label %323                                                                                                         ;L746
 17142| 
 17143| 334: ; preds = %309
 17144|  %335 = gep %149, i64 1648                                                                                             ;L752
 17145|  %336 = load i64, ptr %335, , !!8                                                                                      ;L752
 17146|     ;; self = i64 %336
 17147|  %337 = call i64 @llvm.usub.sat.i64(i64 %336, i64 %311)                                                                ;L2472<752
 17148|  %338 = mul i64 %337, 60                                                                                               ;L752
 17149|     ;; self = i64 %312
 17150|     ;; other = i64 1
 17151|  %339 = call i64 @llvm.umax.i64(i64 %312, i64 1)                                                                       ;L1039<752
 17152|  %340 = udiv i64 %338, %339                                                                                            ;L752
 17153|     ;; e_die = i64 %340
 17154|  %341 = add i64 %223, %268                                                                                             ;L755
 17155|  %342 = icmp ugt i64 %341, %340                                                                                        ;L755
 17156|  %343 = call i64 @llvm.usub.sat.i64(i64 %87, i64 %229)
 17157|  %344 = icmp uge i64 %268, %343
 17158|  %345 = or i1 %344, %342                                                                                               ;L755
 17159|  br i1 %345, label %411, label %346                                                                                    ;L755
 17160| 
 17161| 346: ; preds = %334
 17162|  %347 = gep %149, i64 1600                                                                                             ;L762
 17163|  %348 = load i64, ptr %347, , !!8                                                                                      ;L762
 17164|  %349 = icmp ugt i64 %348, %34                                                                                         ;L762
 17165|  br i1 %349, label %350, label %357                                                                                    ;L762
 17166| 
 17167| 350: ; preds = %395, %359, %346
 17168|     ;; value[0..+8] = i64 %151
 17169|     ;; src[0..+8] = i64 %151
 17170|     ;; value[8..+8] = i64 %268
 17171|     ;; src[8..+8] = i64 %268
 17172|     ;; value[16..+8] = i64 %761
 17173|     ;; src[16..+8] = i64 %761
 17174|     ;; self = ptr %20
 17175|     ;; self = ptr %20
 17176|     ;; additional = i64 1
 17177|     ;; needed_extra_cap = i64 1
 17178|     ;; needed_extra_cap = i64 1
 17179|     ;; strategy = i8 1
 17180|  %351 = load i64, ptr %108, , !!29810, !!8                                                                             ;L1428<775
 17181|     ;; self = ptr %20
 17182|  %352 = load i64, ptr %107, , !!29810, !!8                                                                             ;L149<1428<775
 17183|  %353 = icmp eq i64 %351, %352                                                                                         ;L1428<775
 17184|  br i1 %353, label %354, label %403                                                                                    ;L1428<775
 17185| 
 17186| 354: ; preds = %350
 17187|     ;; self = ptr %20
 17188|     ;; self = ptr %20
 17189|     ;; self = ptr %20
 17190|     ;; used_cap = i64 %351
 17191|     ;; used_cap = i64 %351
 17192|  invoke void @_RNvMs2_NtNtCshWfHDMLkPaX_7bumpalo11collections7raw_vecINtB5_6RawVecTjjjEE25reserve_internal_or_panicCshdEBA0ozCnw_7game_ai(ptr %20, i64 %351, i64 1, i1 zeroext true)
 17193|  to label %355 unwind label %203                                                                                       ;L619<430<738<1429<775
 17194| 
 17195| 355: ; preds = %354
 17196|  %356 = load i64, ptr %108, , !!29810                                                                                  ;L1432<775
 17197|  br label %403                                                                                                         ;L1428<775
 17198| 
 17199| 357: ; preds = %346
 17200|  %358 = icmp ult i64 %268, %226                                                                                        ;L764
 17201|  br i1 %358, label %359, label %411                                                                                    ;L764
 17202| 
 17203| 359: ; preds = %357
 17204|  br i1 %10, label %360, label %350                                                                                     ;L767
 17205| 
 17206| 360: ; preds = %359
 17207|     ;; self = ptr %149
 17208|  %361 = gep %149, i64 1216                                                                                             ;L742<768
 17209|  %362 = load i32, ptr %361, , !!8                                                                                      ;L742<768
 17210|  %363 = icmp eq i32 %362, -1                                                                                           ;L742<768
 17211|  br i1 %363, label %377, label %364                                                                                    ;L742<768
 17212| 
 17213| 364: ; preds = %360
 17214|     ;; self = ptr %149
 17215|     ;; f = ptr %149
 17216|     ;; x = ptr %149
 17217|     ;; a = ptr %149
 17218|     ;; self = ptr %149
 17219|  %365 = gep %149, i64 1184                                                                                             ;L26<768<1162<768
 17220|  %366 = load i64, ptr %365, , !!8                                                                                      ;L26<768<1162<768
 17221|  %367 = gep %149, i64 1192                                                                                             ;L26<768<1162<768
 17222|  %368 = load i64, ptr %367, , !!8                                                                                      ;L26<768<1162<768
 17223|  %369 = gep %149, i64 1480                                                                                             ;L26<768<1162<768
 17224|  %370 = load i64, ptr %369, , !!8                                                                                      ;L26<768<1162<768
 17225|  %371 = add i64 %370, -1                                                                                               ;L26<768<1162<768
 17226|  %372 = mul i64 %371, %368                                                                                             ;L26<768<1162<768
 17227|  %373 = gep %149, i64 1080                                                                                             ;L26<768<1162<768
 17228|  %374 = load i64, ptr %373, , !!8                                                                                      ;L26<768<1162<768
 17229|  %375 = add i64 %374, %366                                                                                             ;L26<768<1162<768
 17230|  %376 = add i64 %375, %372                                                                                             ;L26<768<1162<768
 17231|     ;; self[8..+8] = i64 %376
 17232|     ;; self[0..+8] = i64 1
 17233|  br label %377                                                                                                         ;L1043<768
 17234| 
 17235| 377: ; preds = %364, %360
 17236|  %378 = phi i64 [ %376, %364 ], [ 0, %360 ]                                                                            ;L0<768
 17237|  %379 = gep %149, i64 1136                                                                                             ;L1511<769
 17238|  %380 = load i32, ptr %379, , !!8                                                                                      ;L1511<769
 17239|     ;; mult = i32 %380
 17240|  %381 = icmp eq i32 %380, 0                                                                                            ;L1512<769
 17241|  br i1 %381, label %382, label %385                                                                                    ;L1512<769
 17242| 
 17243| 382: ; preds = %377
 17244|  %383 = gep %149, i64 1664                                                                                             ;L1513<769
 17245|  %384 = load i64, ptr %383, , !!8                                                                                      ;L1513<769
 17246|  br label %392                                                                                                         ;L1512<769
 17247| 
 17248| 385: ; preds = %377
 17249|  %386 = sext i32 %380 to i64                                                                                           ;L1511<769
 17250|     ;; mult = i64 %386
 17251|  %387 = gep %149, i64 1664                                                                                             ;L1515<769
 17252|  %388 = load i64, ptr %387, , !!8                                                                                      ;L1515<769
 17253|  %389 = add nsw i64 %386, 100                                                                                          ;L1515<769
 17254|  %390 = mul i64 %388, %389                                                                                             ;L1515<769
 17255|  %391 = udiv i64 %390, 100                                                                                             ;L1515<769
 17256|  br label %392                                                                                                         ;L1512<769
 17257| 
 17258| 392: ; preds = %385, %382
 17259|  %393 = phi i64 [ %384, %382 ], [ %391, %385 ]                                                                         ;L0<769
 17261|  %394 = invoke i64 @gc::simulation6entityNtB5_6Entity8distance(ptr %149, ptr %7)
 17262|  to label %395 unwind label %203                                                                                       ;L770
 17263| 
 17264| 395: ; preds = %392
 17266|  %396 = load i64, ptr %25, , !!8                                                                                       ;L770
 17267|  %397 = add i64 %396, %394                                                                                             ;L770
 17268|  %398 = mul i64 %348, %82                                                                                              ;L770
 17269|  %399 = add i64 %378, %398                                                                                             ;L768
 17270|  %400 = add i64 %399, %393                                                                                             ;L768
 17271|  %401 = add i64 %400, %143                                                                                             ;L770
 17272|  %402 = icmp ugt i64 %397, %401                                                                                        ;L770
 17273|  br i1 %402, label %411, label %350                                                                                    ;L770
 17274| 
 17275| 403: ; preds = %355, %350
 17276|  %404 = phi i64 [ %351, %350 ], [ %356, %355 ]                                                                         ;L1432<775
 17277|     ;; self = ptr %20
 17278|  %405 = load ptr, ptr %20, , !!29810, !!8, !!8                                                                         ;L138<1432<775
 17279|     ;; self = ptr %405
 17280|     ;; count = i64 %404
 17281|  %406 = gepS %405, i64 %404                                                                                            ;L961<1432<775
 17282|     ;; end = ptr %406
 17283|     ;; dst = ptr %406
 17284|  store i64 %151, ptr %406,                                                                                             ;L1933<1433<775
 17285|  %407 = gep %406, i64 8                                                                                                ;L1933<1433<775
 17286|  store i64 %268, ptr %407,                                                                                             ;L1933<1433<775
 17287|  %408 = gep %406, i64 16                                                                                               ;L1933<1433<775
 17288|  store i64 %761, ptr %408,                                                                                             ;L1933<1433<775
 17289|  %409 = load i64, ptr %108, , !!29810, !!8                                                                             ;L1434<775
 17290|  %410 = add i64 %409, 1                                                                                                ;L1434<775
 17291|  store i64 %410, ptr %108, , !!29810                                                                                   ;L1434<775
 17292|  br label %411                                                                                                         ;L690
 17293| 
 17294| 411: ; preds = %403, %395, %357, %334, %160
 17295|  br label %144                                                                                                         ;L1714<180<690
 17296| 
 17297| 412: ; preds = %286
 17298|     ;; self = ptr %149
 17299|  %413 = load i64, ptr %149, , !!8                                                                                      ;L1136<1482<730
 17300|  %414 = trunc nuw i64 %413 to i1                                                                                       ;L1136<1482<730
 17301|  br i1 %414, label %424, label %415                                                                                    ;L1136<1482<730
 17302| 
 17303| 415: ; preds = %412
 17304|     ;; team = ptr %149
 17305|  %416 = load i64, ptr %270, , !!8                                                                                      ;L1137<1482<730
 17306|     ;; team = i64 %416
 17307|  %417 = icmp ult i64 %416, 2                                                                                           ;L1483<730
 17308|  br i1 %417, label %418, label %423                                                                                    ;L1483<730
 17309| 
 17310| 418: ; preds = %415
 17312|  %419 = gep %283, i64 56                                                                                               ;L122<1483<730
 17313|  %420 = gepS %419, i64 %416                                                                                            ;L122<1483<730
 17314|  %421 = load i64, ptr %420, , !!8                                                                                      ;L122<1483<730
 17315|  %422 = icmp eq i64 %421, 0                                                                                            ;L122<1483<730
 17316|  br i1 %422, label %424, label %426                                                                                    ;L730
 17317| 
 17318| 423: ; preds = %415
 17319|  invoke void @core::panicking18panic_bounds_check(i64 %416, i64 2, ptr @anon.168add0ea037d45d276f5936ae758fe5.39) #30
 17320|  to label %218 unwind label %203                                                                                       ;L1483<730
 17321| 
 17322| 424: ; preds = %418, %412
 17323|  %425 = invoke zeroext i1 @ai::plan_legacy3old11fight_model21is_ignored_well_enemy(i64 %1, ptr %201, ptr %283)
 17324|  to label %427 unwind label %203                                                                                       ;L733
 17325| 
 17326| 426: ; preds = %427, %418, %286
 17327|  br label %275                                                                                                         ;L1
 17328| 
 17329| 427: ; preds = %424
 17330|  br i1 %425, label %426, label %428                                                                                    ;L733
 17331| 
 17332| 428: ; preds = %427
 17333|  %429 = gep %283, i64 1472                                                                                             ;L736
 17334|  %430 = load i64, ptr %429, , !!8                                                                                      ;L736
 17336|     ;; self = ptr %4
 17337|     ;; id = i64 %430
 17338|     ;; iter[8..+8] = i64 2
 17339|     ;; iter[0..+8] = i64 1
 17340|     ;; t = i64 0
 17341|     ;; iter[8..+8] = i64 5
 17342|     ;; iter[0..+8] = i64 1
 17343|     ;; p = i64 0
 17344|  %431 = load ptr, ptr %118, , !!8                                                                                      ;L1918<736
 17345|  %432 = icmp eq ptr %431, null                                                                                         ;L1918<736
 17346|  br i1 %432, label %437, label %433                                                                                    ;L1918<736
 17347| 
 17348| 433: ; preds = %428
 17349|     ;; c = ptr %431
 17350|  %434 = gep %431, i64 1472                                                                                             ;L1919<736
 17351|  %435 = load i64, ptr %434, , !!29890, !!8                                                                             ;L1919<736
 17352|  %436 = icmp eq i64 %435, %430                                                                                         ;L1919<736
 17353|  br i1 %436, label %500, label %437                                                                                    ;L1919<736
 17354| 
 17355| 437: ; preds = %433, %428
 17356|     ;; iter[0..+8] = i64 2
 17357|     ;; p = i64 1
 17358|  %438 = load ptr, ptr %123, , !!8                                                                                      ;L1918<736
 17359|  %439 = icmp eq ptr %438, null                                                                                         ;L1918<736
 17360|  br i1 %439, label %444, label %440                                                                                    ;L1918<736
 17361| 
 17362| 440: ; preds = %437
 17363|     ;; c = ptr %438
 17364|  %441 = gep %438, i64 1472                                                                                             ;L1919<736
 17365|  %442 = load i64, ptr %441, , !!29890, !!8                                                                             ;L1919<736
 17366|  %443 = icmp eq i64 %442, %430                                                                                         ;L1919<736
 17367|  br i1 %443, label %500, label %444                                                                                    ;L1919<736
 17368| 
 17369| 444: ; preds = %440, %437
 17370|     ;; iter[0..+8] = i64 3
 17371|     ;; p = i64 2
 17372|  %445 = load ptr, ptr %124, , !!8                                                                                      ;L1918<736
 17373|  %446 = icmp eq ptr %445, null                                                                                         ;L1918<736
 17374|  br i1 %446, label %451, label %447                                                                                    ;L1918<736
 17375| 
 17376| 447: ; preds = %444
 17377|     ;; c = ptr %445
 17378|  %448 = gep %445, i64 1472                                                                                             ;L1919<736
 17379|  %449 = load i64, ptr %448, , !!29890, !!8                                                                             ;L1919<736
 17380|  %450 = icmp eq i64 %449, %430                                                                                         ;L1919<736
 17381|  br i1 %450, label %500, label %451                                                                                    ;L1919<736
 17382| 
 17383| 451: ; preds = %447, %444
 17384|     ;; iter[0..+8] = i64 4
 17385|     ;; p = i64 3
 17386|  %452 = load ptr, ptr %125, , !!8                                                                                      ;L1918<736
 17387|  %453 = icmp eq ptr %452, null                                                                                         ;L1918<736
 17388|  br i1 %453, label %458, label %454                                                                                    ;L1918<736
 17389| 
 17390| 454: ; preds = %451
 17391|     ;; c = ptr %452
 17392|  %455 = gep %452, i64 1472                                                                                             ;L1919<736
 17393|  %456 = load i64, ptr %455, , !!29890, !!8                                                                             ;L1919<736
 17394|  %457 = icmp eq i64 %456, %430                                                                                         ;L1919<736
 17395|  br i1 %457, label %500, label %458                                                                                    ;L1919<736
 17396| 
 17397| 458: ; preds = %454, %451
 17398|     ;; iter[0..+8] = i64 5
 17399|     ;; p = i64 4
 17400|  %459 = load ptr, ptr %126, , !!8                                                                                      ;L1918<736
 17401|  %460 = icmp eq ptr %459, null                                                                                         ;L1918<736
 17402|  br i1 %460, label %465, label %461                                                                                    ;L1918<736
 17403| 
 17404| 461: ; preds = %458
 17405|     ;; c = ptr %459
 17406|  %462 = gep %459, i64 1472                                                                                             ;L1919<736
 17407|  %463 = load i64, ptr %462, , !!29890, !!8                                                                             ;L1919<736
 17408|  %464 = icmp eq i64 %463, %430                                                                                         ;L1919<736
 17409|  br i1 %464, label %500, label %465                                                                                    ;L1919<736
 17410| 
 17411| 465: ; preds = %461, %458
 17412|     ;; iter[0..+8] = i64 2
 17413|     ;; t = i64 1
 17414|     ;; iter[8..+8] = i64 5
 17415|     ;; iter[0..+8] = i64 1
 17416|     ;; p = i64 0
 17417|  %466 = load ptr, ptr %127, , !!8                                                                                      ;L1918<736
 17418|  %467 = icmp eq ptr %466, null                                                                                         ;L1918<736
 17419|  br i1 %467, label %472, label %468                                                                                    ;L1918<736
 17420| 
 17421| 468: ; preds = %465
 17422|     ;; c = ptr %466
 17423|  %469 = gep %466, i64 1472                                                                                             ;L1919<736
 17424|  %470 = load i64, ptr %469, , !!29890, !!8                                                                             ;L1919<736
 17425|  %471 = icmp eq i64 %470, %430                                                                                         ;L1919<736
 17426|  br i1 %471, label %500, label %472                                                                                    ;L1919<736
 17427| 
 17428| 472: ; preds = %468, %465
 17429|     ;; iter[0..+8] = i64 2
 17430|     ;; p = i64 1
 17431|  %473 = load ptr, ptr %128, , !!8                                                                                      ;L1918<736
 17432|  %474 = icmp eq ptr %473, null                                                                                         ;L1918<736
 17433|  br i1 %474, label %479, label %475                                                                                    ;L1918<736
 17434| 
 17435| 475: ; preds = %472
 17436|     ;; c = ptr %473
 17437|  %476 = gep %473, i64 1472                                                                                             ;L1919<736
 17438|  %477 = load i64, ptr %476, , !!29890, !!8                                                                             ;L1919<736
 17439|  %478 = icmp eq i64 %477, %430                                                                                         ;L1919<736
 17440|  br i1 %478, label %500, label %479                                                                                    ;L1919<736
 17441| 
 17442| 479: ; preds = %475, %472
 17443|     ;; iter[0..+8] = i64 3
 17444|     ;; p = i64 2
 17445|  %480 = load ptr, ptr %129, , !!8                                                                                      ;L1918<736
 17446|  %481 = icmp eq ptr %480, null                                                                                         ;L1918<736
 17447|  br i1 %481, label %486, label %482                                                                                    ;L1918<736
 17448| 
 17449| 482: ; preds = %479
 17450|     ;; c = ptr %480
 17451|  %483 = gep %480, i64 1472                                                                                             ;L1919<736
 17452|  %484 = load i64, ptr %483, , !!29890, !!8                                                                             ;L1919<736
 17453|  %485 = icmp eq i64 %484, %430                                                                                         ;L1919<736
 17454|  br i1 %485, label %500, label %486                                                                                    ;L1919<736
 17455| 
 17456| 486: ; preds = %482, %479
 17457|     ;; iter[0..+8] = i64 4
 17458|     ;; p = i64 3
 17459|  %487 = load ptr, ptr %130, , !!8                                                                                      ;L1918<736
 17460|  %488 = icmp eq ptr %487, null                                                                                         ;L1918<736
 17461|  br i1 %488, label %493, label %489                                                                                    ;L1918<736
 17462| 
 17463| 489: ; preds = %486
 17464|     ;; c = ptr %487
 17465|  %490 = gep %487, i64 1472                                                                                             ;L1919<736
 17466|  %491 = load i64, ptr %490, , !!29890, !!8                                                                             ;L1919<736
 17467|  %492 = icmp eq i64 %491, %430                                                                                         ;L1919<736
 17468|  br i1 %492, label %500, label %493                                                                                    ;L1919<736
 17469| 
 17470| 493: ; preds = %489, %486
 17471|     ;; iter[0..+8] = i64 5
 17472|     ;; p = i64 4
 17473|  %494 = load ptr, ptr %131, , !!8                                                                                      ;L1918<736
 17474|  %495 = icmp eq ptr %494, null                                                                                         ;L1918<736
 17475|  br i1 %495, label %511, label %496                                                                                    ;L1918<736
 17476| 
 17477| 496: ; preds = %493
 17478|     ;; c = ptr %494
 17479|  %497 = gep %494, i64 1472                                                                                             ;L1919<736
 17480|  %498 = load i64, ptr %497, , !!29890, !!8                                                                             ;L1919<736
 17481|  %499 = icmp eq i64 %498, %430                                                                                         ;L1919<736
 17482|  br i1 %499, label %500, label %511                                                                                    ;L1919<736
 17483| 
 17484| 500: ; preds = %496, %489, %482, %475, %468, %461, %454, %447, %440, %433
 17485|  %501 = phi i64 [ 0, %433 ], [ 0, %440 ], [ 0, %447 ], [ 0, %454 ], [ 0, %461 ], [ 1, %468 ], [ 1, %475 ], [ 1, %482 ], [ 1, %489 ], [ 1, %496 ]
 17486|  %502 = phi i64 [ 0, %433 ], [ 1, %440 ], [ 2, %447 ], [ 3, %454 ], [ 4, %461 ], [ 0, %468 ], [ 1, %475 ], [ 2, %482 ], [ 3, %489 ], [ 4, %496 ]
 17487|  %503 = getelementptr [5 x ptr], ptr %132, i64 %501                                                                    ;L1920<736
 17488|  %504 = getelementptr ptr, ptr %503, i64 %502                                                                          ;L1920<736
 17489|  %505 = load ptr, ptr %504, , !!8                                                                                      ;L1920<736
 17490|     ;; self = ptr %505
 17491|  %506 = icmp eq ptr %505, null                                                                                         ;L1011<736
 17492|  br i1 %506, label %511, label %507                                                                                    ;L1011<736
 17493| 
 17494| 507: ; preds = %500
 17495|     ;; ap = ptr %505
 17496|  %508 = gep %505, i64 2352                                                                                             ;L737
 17497|  %509 = load i64, ptr %508, , !!8                                                                                      ;L737
 17498|  %510 = icmp ult i64 %509, 2                                                                                           ;L737
 17499|  br i1 %510, label %513, label %512                                                                                    ;L737
 17500| 
 17501| 511: ; preds = %500, %496, %493
 17502|  invoke void @core::option13unwrap_failed(ptr @anon.168add0ea037d45d276f5936ae758fe5.51) #30
 17503|  to label %218 unwind label %203                                                                                       ;L1013<736
 17504| 
 17505| 512: ; preds = %507
 17506|  invoke void @core::panicking18panic_bounds_check(i64 %509, i64 2, ptr @anon.168add0ea037d45d276f5936ae758fe5.52) #30
 17507|  to label %218 unwind label %203                                                                                       ;L737
 17508| 
 17509| 513: ; preds = %507
 17510|     ;; self = ptr %505
 17511|  %514 = gep %505, i64 2496                                                                                             ;L581<737
 17512|  %515 = load i32, ptr %514, , !!8                                                                                      ;L581<737
 17513|  %516 = zext nneg i32 %515 to i64                                                                                      ;L581<737
 17514|  %517 = getelementptr [5 x { [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64] }], ptr %133, i64 %509 ;L737
 17515|  %518 = gepS %517, i64 %516                                                                                            ;L737
 17516|     ;; c = ptr %518
 17517|     ;; nuke = i64 0
 17518|  %519 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_attack(ptr %283)
 17519|  to label %520 unwind label %203                                                                                       ;L739
 17520| 
 17521| 520: ; preds = %513
 17522|  br i1 %519, label %540, label %521                                                                                    ;L739
 17523| 
 17524| 521: ; preds = %520
 17525|  %522 = gep %283, i64 104                                                                                              ;L1748<739
 17526|  %523 = load i64, ptr %522, , !!8                                                                                      ;L1748<739
 17527|  switch i64 %523, label %90 [
 17528|  i64 0, label %540
 17529|  i64 1, label %532
 17530|  i64 2, label %524
 17531|  i64 3, label %540
 17532|  i64 4, label %525
 17533|  i64 5, label %526
 17534|  i64 6, label %526
 17535|  i64 7, label %525
 17536|  i64 8, label %527
 17537|  i64 9, label %528
 17538|  i64 10, label %529
 17539|  i64 11, label %530
 17540|  i64 12, label %531
 17541|  i64 13, label %527
 17542|  ]                                                                                                                     ;L1748<739
 17543| 
 17544| 524: ; preds = %521
 17545|     ;; info = ptr %283
 17546|  br label %532                                                                                                         ;L1758<739
 17547| 
 17548| 525: ; preds = %521, %521
 17549|     ;; info = ptr %283
 17550|  br label %532                                                                                                         ;L1750<739
 17551| 
 17552| 526: ; preds = %521, %521
 17553|     ;; info = ptr %283
 17554|  br label %532                                                                                                         ;L1760<739
 17555| 
 17556| 527: ; preds = %521, %521
 17557|     ;; info = ptr %283
 17558|  br label %532                                                                                                         ;L1753<739
 17559| 
 17560| 528: ; preds = %521
 17561|     ;; info = ptr %283
 17562|  br label %532                                                                                                         ;L1754<739
 17563| 
 17564| 529: ; preds = %521
 17565|     ;; info = ptr %283
 17566|  br label %532                                                                                                         ;L1755<739
 17567| 
 17568| 530: ; preds = %521
 17569|     ;; info = ptr %283
 17570|  br label %532                                                                                                         ;L1756<739
 17571| 
 17572| 531: ; preds = %521
 17573|     ;; info = ptr %283
 17574|  br label %532                                                                                                         ;L1757<739
 17575| 
 17576| 532: ; preds = %531, %530, %529, %528, %527, %526, %525, %524, %521
 17577|  %533 = phi i64 [ 232, %525 ], [ 208, %531 ], [ 216, %530 ], [ 240, %529 ], [ 200, %528 ], [ 176, %527 ], [ 272, %524 ], [ 184, %521 ], [ 496, %526 ]
 17578|  %534 = gep %283, i64 %533                                                                                             ;L0<739
 17579|  %535 = load i64, ptr %534, , !!8                                                                                      ;L0<739
 17580|  %536 = icmp ugt i64 %535, %31                                                                                         ;L739
 17581|  br i1 %536, label %537, label %540                                                                                    ;L739
 17582| 
 17583| 537: ; preds = %540, %532
 17584|  %538 = phi i64 [ 0, %532 ], [ %542, %540 ]                                                                            ;L0
 17585|     ;; nuke = i64 %538
 17586|  %539 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity9can_skill(ptr %283)
 17587|  to label %543 unwind label %203                                                                                       ;L740
 17588| 
 17589| 540: ; preds = %532, %521, %521, %520
 17590|  %541 = getelementptr i64, ptr %518, i64 %214                                                                          ;L739
 17591|  %542 = load i64, ptr %541, , !!8                                                                                      ;L739
 17592|     ;; self = i64 0
 17593|     ;; other = i64 %542
 17594|  br label %537                                                                                                         ;L1039<739
 17595| 
 17596| 543: ; preds = %537
 17597|  br i1 %539, label %555, label %544                                                                                    ;L740
 17598| 
 17599| 544: ; preds = %543
 17600|  %545 = gep %283, i64 104                                                                                              ;L1775<740
 17601|  %546 = load i64, ptr %545, , !!8                                                                                      ;L1775<740
 17602|  %547 = icmp eq i64 %546, 13                                                                                           ;L1775<740
 17603|  br i1 %547, label %548, label %555                                                                                    ;L1775<740
 17604| 
 17605| 548: ; preds = %544
 17607|  %549 = gep %283, i64 184                                                                                              ;L1776<740
 17608|  %550 = load i64, ptr %549, , !!8                                                                                      ;L1776<740
 17609|  %551 = icmp ugt i64 %550, %31                                                                                         ;L740
 17610|  br i1 %551, label %552, label %555                                                                                    ;L740
 17611| 
 17612| 552: ; preds = %555, %548
 17613|  %553 = phi i64 [ %538, %548 ], [ %559, %555 ]                                                                         ;L0
 17614|     ;; nuke = i64 %553
 17615|  %554 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_skill2(ptr %283)
 17616|  to label %560 unwind label %203                                                                                       ;L741
 17617| 
 17618| 555: ; preds = %548, %544, %543
 17619|  %556 = gep %518, i64 40                                                                                               ;L740
 17620|  %557 = getelementptr i64, ptr %556, i64 %214                                                                          ;L740
 17621|  %558 = load i64, ptr %557, , !!8                                                                                      ;L740
 17622|     ;; self = i64 %538
 17623|     ;; other = i64 %558
 17624|  %559 = call i64 @llvm.umax.i64(i64 %558, i64 %538)                                                                    ;L1039<740
 17625|  br label %552                                                                                                         ;L1039<740
 17626| 
 17627| 560: ; preds = %552
 17628|  br i1 %554, label %569, label %561                                                                                    ;L741
 17629| 
 17630| 561: ; preds = %560
 17631|  %562 = gep %283, i64 104                                                                                              ;L1790<741
 17632|  %563 = load i64, ptr %562, , !!8                                                                                      ;L1790<741
 17633|  %564 = icmp eq i64 %563, 13                                                                                           ;L1790<741
 17634|  br i1 %564, label %565, label %569                                                                                    ;L1790<741
 17635| 
 17636| 565: ; preds = %561
 17638|  %566 = gep %283, i64 192                                                                                              ;L1791<741
 17639|  %567 = load i64, ptr %566, , !!8                                                                                      ;L1791<741
 17640|  %568 = icmp ugt i64 %567, %31                                                                                         ;L741
 17641|  br i1 %568, label %574, label %569                                                                                    ;L741
 17642| 
 17643| 569: ; preds = %565, %561, %560
 17644|  %570 = gep %518, i64 80                                                                                               ;L741
 17645|  %571 = getelementptr i64, ptr %570, i64 %214                                                                          ;L741
 17646|  %572 = load i64, ptr %571, , !!8                                                                                      ;L741
 17647|     ;; self = i64 %553
 17648|     ;; other = i64 %572
 17649|  %573 = call i64 @llvm.umax.i64(i64 %572, i64 %553)                                                                    ;L1039<741
 17650|  br label %574                                                                                                         ;L1039<741
 17651| 
 17652| 574: ; preds = %569, %565
 17653|  %575 = phi i64 [ %553, %565 ], [ %573, %569 ]                                                                         ;L0
 17654|     ;; nuke = i64 %575
 17655|  %576 = gep %518, i64 400                                                                                              ;L742
 17656|  %577 = getelementptr i64, ptr %576, i64 %214                                                                          ;L742
 17657|  %578 = load i64, ptr %577, , !!8                                                                                      ;L742
 17658|  %579 = gep %518, i64 440                                                                                              ;L742
 17659|  %580 = getelementptr i64, ptr %579, i64 %214                                                                          ;L742
 17660|  %581 = load i64, ptr %580, , !!8                                                                                      ;L742
 17661|  %582 = gep %518, i64 480                                                                                              ;L742
 17662|  %583 = getelementptr i64, ptr %582, i64 %214                                                                          ;L742
 17663|  %584 = load i64, ptr %583, , !!8                                                                                      ;L742
 17664|  %585 = add i64 %578, %273                                                                                             ;L742
 17665|  %586 = add i64 %585, %581                                                                                             ;L742
 17666|  %587 = add i64 %586, %584                                                                                             ;L742
 17667|     ;; det_dps = i64 %587
 17668|  %588 = add i64 %575, %272                                                                                             ;L743
 17669|     ;; det_nuke = i64 %588
 17670|     ;; rhs = i64 %588
 17671|  br label %271                                                                                                         ;L726
 17672| 
 17673| 589: ; preds = %245
 17674|  %590 = gep %242, i64 1648                                                                                             ;L708
 17675|  %591 = load i64, ptr %590, , !!8                                                                                      ;L708
 17676|  %592 = mul i64 %591, 100                                                                                              ;L708
 17677|  %593 = gep %242, i64 1576                                                                                             ;L708
 17678|  %594 = load i64, ptr %593, , !!8                                                                                      ;L708
 17679|     ;; self = i64 %594
 17680|     ;; other = i64 1
 17681|  %595 = call i64 @llvm.umax.i64(i64 %594, i64 1)                                                                       ;L1039<708
 17682|  %596 = udiv i64 %592, %595                                                                                            ;L708
 17683|  %597 = icmp ult i64 %596, 40                                                                                          ;L708
 17684|  br i1 %597, label %762, label %598                                                                                    ;L708
 17685| 
 17686| 598: ; preds = %589
 17687|  %599 = gep %242, i64 1472                                                                                             ;L711
 17688|  %600 = load i64, ptr %599, , !!8                                                                                      ;L711
 17690|     ;; self = ptr %4
 17691|     ;; id = i64 %600
 17692|     ;; iter[8..+8] = i64 2
 17693|     ;; iter[0..+8] = i64 1
 17694|     ;; t = i64 0
 17695|     ;; iter[8..+8] = i64 5
 17696|     ;; iter[0..+8] = i64 1
 17697|     ;; p = i64 0
 17698|  %601 = load ptr, ptr %118, , !!8                                                                                      ;L1918<711
 17699|  %602 = icmp eq ptr %601, null                                                                                         ;L1918<711
 17700|  br i1 %602, label %607, label %603                                                                                    ;L1918<711
 17701| 
 17702| 603: ; preds = %598
 17703|     ;; c = ptr %601
 17704|  %604 = gep %601, i64 1472                                                                                             ;L1919<711
 17705|  %605 = load i64, ptr %604, , !!29967, !!8                                                                             ;L1919<711
 17706|  %606 = icmp eq i64 %605, %600                                                                                         ;L1919<711
 17707|  br i1 %606, label %670, label %607                                                                                    ;L1919<711
 17708| 
 17709| 607: ; preds = %603, %598
 17710|     ;; iter[0..+8] = i64 2
 17711|     ;; p = i64 1
 17712|  %608 = load ptr, ptr %123, , !!8                                                                                      ;L1918<711
 17713|  %609 = icmp eq ptr %608, null                                                                                         ;L1918<711
 17714|  br i1 %609, label %614, label %610                                                                                    ;L1918<711
 17715| 
 17716| 610: ; preds = %607
 17717|     ;; c = ptr %608
 17718|  %611 = gep %608, i64 1472                                                                                             ;L1919<711
 17719|  %612 = load i64, ptr %611, , !!29967, !!8                                                                             ;L1919<711
 17720|  %613 = icmp eq i64 %612, %600                                                                                         ;L1919<711
 17721|  br i1 %613, label %670, label %614                                                                                    ;L1919<711
 17722| 
 17723| 614: ; preds = %610, %607
 17724|     ;; iter[0..+8] = i64 3
 17725|     ;; p = i64 2
 17726|  %615 = load ptr, ptr %124, , !!8                                                                                      ;L1918<711
 17727|  %616 = icmp eq ptr %615, null                                                                                         ;L1918<711
 17728|  br i1 %616, label %621, label %617                                                                                    ;L1918<711
 17729| 
 17730| 617: ; preds = %614
 17731|     ;; c = ptr %615
 17732|  %618 = gep %615, i64 1472                                                                                             ;L1919<711
 17733|  %619 = load i64, ptr %618, , !!29967, !!8                                                                             ;L1919<711
 17734|  %620 = icmp eq i64 %619, %600                                                                                         ;L1919<711
 17735|  br i1 %620, label %670, label %621                                                                                    ;L1919<711
 17736| 
 17737| 621: ; preds = %617, %614
 17738|     ;; iter[0..+8] = i64 4
 17739|     ;; p = i64 3
 17740|  %622 = load ptr, ptr %125, , !!8                                                                                      ;L1918<711
 17741|  %623 = icmp eq ptr %622, null                                                                                         ;L1918<711
 17742|  br i1 %623, label %628, label %624                                                                                    ;L1918<711
 17743| 
 17744| 624: ; preds = %621
 17745|     ;; c = ptr %622
 17746|  %625 = gep %622, i64 1472                                                                                             ;L1919<711
 17747|  %626 = load i64, ptr %625, , !!29967, !!8                                                                             ;L1919<711
 17748|  %627 = icmp eq i64 %626, %600                                                                                         ;L1919<711
 17749|  br i1 %627, label %670, label %628                                                                                    ;L1919<711
 17750| 
 17751| 628: ; preds = %624, %621
 17752|     ;; iter[0..+8] = i64 5
 17753|     ;; p = i64 4
 17754|  %629 = load ptr, ptr %126, , !!8                                                                                      ;L1918<711
 17755|  %630 = icmp eq ptr %629, null                                                                                         ;L1918<711
 17756|  br i1 %630, label %635, label %631                                                                                    ;L1918<711
 17757| 
 17758| 631: ; preds = %628
 17759|     ;; c = ptr %629
 17760|  %632 = gep %629, i64 1472                                                                                             ;L1919<711
 17761|  %633 = load i64, ptr %632, , !!29967, !!8                                                                             ;L1919<711
 17762|  %634 = icmp eq i64 %633, %600                                                                                         ;L1919<711
 17763|  br i1 %634, label %670, label %635                                                                                    ;L1919<711
 17764| 
 17765| 635: ; preds = %631, %628
 17766|     ;; iter[0..+8] = i64 2
 17767|     ;; t = i64 1
 17768|     ;; iter[8..+8] = i64 5
 17769|     ;; iter[0..+8] = i64 1
 17770|     ;; p = i64 0
 17771|  %636 = load ptr, ptr %127, , !!8                                                                                      ;L1918<711
 17772|  %637 = icmp eq ptr %636, null                                                                                         ;L1918<711
 17773|  br i1 %637, label %642, label %638                                                                                    ;L1918<711
 17774| 
 17775| 638: ; preds = %635
 17776|     ;; c = ptr %636
 17777|  %639 = gep %636, i64 1472                                                                                             ;L1919<711
 17778|  %640 = load i64, ptr %639, , !!29967, !!8                                                                             ;L1919<711
 17779|  %641 = icmp eq i64 %640, %600                                                                                         ;L1919<711
 17780|  br i1 %641, label %670, label %642                                                                                    ;L1919<711
 17781| 
 17782| 642: ; preds = %638, %635
 17783|     ;; iter[0..+8] = i64 2
 17784|     ;; p = i64 1
 17785|  %643 = load ptr, ptr %128, , !!8                                                                                      ;L1918<711
 17786|  %644 = icmp eq ptr %643, null                                                                                         ;L1918<711
 17787|  br i1 %644, label %649, label %645                                                                                    ;L1918<711
 17788| 
 17789| 645: ; preds = %642
 17790|     ;; c = ptr %643
 17791|  %646 = gep %643, i64 1472                                                                                             ;L1919<711
 17792|  %647 = load i64, ptr %646, , !!29967, !!8                                                                             ;L1919<711
 17793|  %648 = icmp eq i64 %647, %600                                                                                         ;L1919<711
 17794|  br i1 %648, label %670, label %649                                                                                    ;L1919<711
 17795| 
 17796| 649: ; preds = %645, %642
 17797|     ;; iter[0..+8] = i64 3
 17798|     ;; p = i64 2
 17799|  %650 = load ptr, ptr %129, , !!8                                                                                      ;L1918<711
 17800|  %651 = icmp eq ptr %650, null                                                                                         ;L1918<711
 17801|  br i1 %651, label %656, label %652                                                                                    ;L1918<711
 17802| 
 17803| 652: ; preds = %649
 17804|     ;; c = ptr %650
 17805|  %653 = gep %650, i64 1472                                                                                             ;L1919<711
 17806|  %654 = load i64, ptr %653, , !!29967, !!8                                                                             ;L1919<711
 17807|  %655 = icmp eq i64 %654, %600                                                                                         ;L1919<711
 17808|  br i1 %655, label %670, label %656                                                                                    ;L1919<711
 17809| 
 17810| 656: ; preds = %652, %649
 17811|     ;; iter[0..+8] = i64 4
 17812|     ;; p = i64 3
 17813|  %657 = load ptr, ptr %130, , !!8                                                                                      ;L1918<711
 17814|  %658 = icmp eq ptr %657, null                                                                                         ;L1918<711
 17815|  br i1 %658, label %663, label %659                                                                                    ;L1918<711
 17816| 
 17817| 659: ; preds = %656
 17818|     ;; c = ptr %657
 17819|  %660 = gep %657, i64 1472                                                                                             ;L1919<711
 17820|  %661 = load i64, ptr %660, , !!29967, !!8                                                                             ;L1919<711
 17821|  %662 = icmp eq i64 %661, %600                                                                                         ;L1919<711
 17822|  br i1 %662, label %670, label %663                                                                                    ;L1919<711
 17823| 
 17824| 663: ; preds = %659, %656
 17825|     ;; iter[0..+8] = i64 5
 17826|     ;; p = i64 4
 17827|  %664 = load ptr, ptr %131, , !!8                                                                                      ;L1918<711
 17828|  %665 = icmp eq ptr %664, null                                                                                         ;L1918<711
 17829|  br i1 %665, label %681, label %666                                                                                    ;L1918<711
 17830| 
 17831| 666: ; preds = %663
 17832|     ;; c = ptr %664
 17833|  %667 = gep %664, i64 1472                                                                                             ;L1919<711
 17834|  %668 = load i64, ptr %667, , !!29967, !!8                                                                             ;L1919<711
 17835|  %669 = icmp eq i64 %668, %600                                                                                         ;L1919<711
 17836|  br i1 %669, label %670, label %681                                                                                    ;L1919<711
 17837| 
 17838| 670: ; preds = %666, %659, %652, %645, %638, %631, %624, %617, %610, %603
 17839|  %671 = phi i64 [ 0, %603 ], [ 0, %610 ], [ 0, %617 ], [ 0, %624 ], [ 0, %631 ], [ 1, %638 ], [ 1, %645 ], [ 1, %652 ], [ 1, %659 ], [ 1, %666 ]
 17840|  %672 = phi i64 [ 0, %603 ], [ 1, %610 ], [ 2, %617 ], [ 3, %624 ], [ 4, %631 ], [ 0, %638 ], [ 1, %645 ], [ 2, %652 ], [ 3, %659 ], [ 4, %666 ]
 17841|  %673 = getelementptr [5 x ptr], ptr %132, i64 %671                                                                    ;L1920<711
 17842|  %674 = getelementptr ptr, ptr %673, i64 %672                                                                          ;L1920<711
 17843|  %675 = load ptr, ptr %674, , !!8                                                                                      ;L1920<711
 17844|     ;; self = ptr %675
 17845|  %676 = icmp eq ptr %675, null                                                                                         ;L1011<711
 17846|  br i1 %676, label %681, label %677                                                                                    ;L1011<711
 17847| 
 17848| 677: ; preds = %670
 17849|     ;; ap = ptr %675
 17850|  %678 = gep %675, i64 2352                                                                                             ;L712
 17851|  %679 = load i64, ptr %678, , !!8                                                                                      ;L712
 17852|  %680 = icmp ult i64 %679, 2                                                                                           ;L712
 17853|  br i1 %680, label %683, label %682                                                                                    ;L712
 17854| 
 17855| 681: ; preds = %670, %666, %663
 17856|  invoke void @core::option13unwrap_failed(ptr @anon.168add0ea037d45d276f5936ae758fe5.53) #30
 17857|  to label %218 unwind label %203                                                                                       ;L1013<711
 17858| 
 17859| 682: ; preds = %677
 17860|  invoke void @core::panicking18panic_bounds_check(i64 %679, i64 2, ptr @anon.168add0ea037d45d276f5936ae758fe5.54) #30
 17861|  to label %218 unwind label %203                                                                                       ;L712
 17862| 
 17863| 683: ; preds = %677
 17864|     ;; self = ptr %675
 17865|  %684 = gep %675, i64 2496                                                                                             ;L581<712
 17866|  %685 = load i32, ptr %684, , !!8                                                                                      ;L581<712
 17867|  %686 = zext nneg i32 %685 to i64                                                                                      ;L581<712
 17868|  %687 = getelementptr [5 x { [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64] }], ptr %133, i64 %679 ;L712
 17869|  %688 = gepS %687, i64 %686                                                                                            ;L712
 17870|     ;; c = ptr %688
 17871|     ;; nuke = i64 0
 17872|  %689 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_attack(ptr %242)
 17873|  to label %690 unwind label %203                                                                                       ;L714
 17874| 
 17875| 690: ; preds = %683
 17876|  br i1 %689, label %710, label %691                                                                                    ;L714
 17877| 
 17878| 691: ; preds = %690
 17879|  %692 = gep %242, i64 104                                                                                              ;L1748<714
 17880|  %693 = load i64, ptr %692, , !!8                                                                                      ;L1748<714
 17881|  switch i64 %693, label %90 [
 17882|  i64 0, label %710
 17883|  i64 1, label %702
 17884|  i64 2, label %694
 17885|  i64 3, label %710
 17886|  i64 4, label %695
 17887|  i64 5, label %696
 17888|  i64 6, label %696
 17889|  i64 7, label %695
 17890|  i64 8, label %697
 17891|  i64 9, label %698
 17892|  i64 10, label %699
 17893|  i64 11, label %700
 17894|  i64 12, label %701
 17895|  i64 13, label %697
 17896|  ]                                                                                                                     ;L1748<714
 17897| 
 17898| 694: ; preds = %691
 17899|     ;; info = ptr %242
 17900|  br label %702                                                                                                         ;L1758<714
 17901| 
 17902| 695: ; preds = %691, %691
 17903|     ;; info = ptr %242
 17904|  br label %702                                                                                                         ;L1750<714
 17905| 
 17906| 696: ; preds = %691, %691
 17907|     ;; info = ptr %242
 17908|  br label %702                                                                                                         ;L1760<714
 17909| 
 17910| 697: ; preds = %691, %691
 17911|     ;; info = ptr %242
 17912|  br label %702                                                                                                         ;L1753<714
 17913| 
 17914| 698: ; preds = %691
 17915|     ;; info = ptr %242
 17916|  br label %702                                                                                                         ;L1754<714
 17917| 
 17918| 699: ; preds = %691
 17919|     ;; info = ptr %242
 17920|  br label %702                                                                                                         ;L1755<714
 17921| 
 17922| 700: ; preds = %691
 17923|     ;; info = ptr %242
 17924|  br label %702                                                                                                         ;L1756<714
 17925| 
 17926| 701: ; preds = %691
 17927|     ;; info = ptr %242
 17928|  br label %702                                                                                                         ;L1757<714
 17929| 
 17930| 702: ; preds = %701, %700, %699, %698, %697, %696, %695, %694, %691
 17931|  %703 = phi i64 [ 232, %695 ], [ 208, %701 ], [ 216, %700 ], [ 240, %699 ], [ 200, %698 ], [ 176, %697 ], [ 272, %694 ], [ 184, %691 ], [ 496, %696 ]
 17932|  %704 = gep %242, i64 %703                                                                                             ;L0<714
 17933|  %705 = load i64, ptr %704, , !!8                                                                                      ;L0<714
 17934|  %706 = icmp ugt i64 %705, %31                                                                                         ;L714
 17935|  br i1 %706, label %707, label %710                                                                                    ;L714
 17936| 
 17937| 707: ; preds = %710, %702
 17938|  %708 = phi i64 [ 0, %702 ], [ %712, %710 ]                                                                            ;L0
 17939|     ;; nuke = i64 %708
 17940|  %709 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity9can_skill(ptr %242)
 17941|  to label %713 unwind label %203                                                                                       ;L715
 17942| 
 17943| 710: ; preds = %702, %691, %691, %690
 17944|  %711 = getelementptr i64, ptr %688, i64 %32                                                                           ;L714
 17945|  %712 = load i64, ptr %711, , !!8                                                                                      ;L714
 17946|     ;; self = i64 0
 17947|     ;; other = i64 %712
 17948|  br label %707                                                                                                         ;L1039<714
 17949| 
 17950| 713: ; preds = %707
 17951|  br i1 %709, label %725, label %714                                                                                    ;L715
 17952| 
 17953| 714: ; preds = %713
 17954|  %715 = gep %242, i64 104                                                                                              ;L1775<715
 17955|  %716 = load i64, ptr %715, , !!8                                                                                      ;L1775<715
 17956|  %717 = icmp eq i64 %716, 13                                                                                           ;L1775<715
 17957|  br i1 %717, label %718, label %725                                                                                    ;L1775<715
 17958| 
 17959| 718: ; preds = %714
 17960|     ;; champ = ptr %242
 17961|  %719 = gep %242, i64 184                                                                                              ;L1776<715
 17962|  %720 = load i64, ptr %719, , !!8                                                                                      ;L1776<715
 17963|  %721 = icmp ugt i64 %720, %31                                                                                         ;L715
 17964|  br i1 %721, label %722, label %725                                                                                    ;L715
 17965| 
 17966| 722: ; preds = %725, %718
 17967|  %723 = phi i64 [ %708, %718 ], [ %729, %725 ]                                                                         ;L0
 17968|     ;; nuke = i64 %723
 17969|  %724 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_skill2(ptr %242)
 17970|  to label %730 unwind label %203                                                                                       ;L716
 17971| 
 17972| 725: ; preds = %718, %714, %713
 17973|  %726 = gep %688, i64 40                                                                                               ;L715
 17974|  %727 = getelementptr i64, ptr %726, i64 %32                                                                           ;L715
 17975|  %728 = load i64, ptr %727, , !!8                                                                                      ;L715
 17976|     ;; self = i64 %708
 17977|     ;; other = i64 %728
 17978|  %729 = call i64 @llvm.umax.i64(i64 %728, i64 %708)                                                                    ;L1039<715
 17979|  br label %722                                                                                                         ;L1039<715
 17980| 
 17981| 730: ; preds = %722
 17982|  br i1 %724, label %739, label %731                                                                                    ;L716
 17983| 
 17984| 731: ; preds = %730
 17985|  %732 = gep %242, i64 104                                                                                              ;L1790<716
 17986|  %733 = load i64, ptr %732, , !!8                                                                                      ;L1790<716
 17987|  %734 = icmp eq i64 %733, 13                                                                                           ;L1790<716
 17988|  br i1 %734, label %735, label %739                                                                                    ;L1790<716
 17989| 
 17990| 735: ; preds = %731
 17991|     ;; champ = ptr %242
 17992|  %736 = gep %242, i64 192                                                                                              ;L1791<716
 17993|  %737 = load i64, ptr %736, , !!8                                                                                      ;L1791<716
 17994|  %738 = icmp ugt i64 %737, %31                                                                                         ;L716
 17995|  br i1 %738, label %744, label %739                                                                                    ;L716
 17996| 
 17997| 739: ; preds = %735, %731, %730
 17998|  %740 = gep %688, i64 80                                                                                               ;L716
 17999|  %741 = getelementptr i64, ptr %740, i64 %32                                                                           ;L716
 18000|  %742 = load i64, ptr %741, , !!8                                                                                      ;L716
 18001|     ;; self = i64 %723
 18002|     ;; other = i64 %742
 18003|  %743 = call i64 @llvm.umax.i64(i64 %742, i64 %723)                                                                    ;L1039<716
 18004|  br label %744                                                                                                         ;L1039<716
 18005| 
 18006| 744: ; preds = %739, %735
 18007|  %745 = phi i64 [ %723, %735 ], [ %743, %739 ]                                                                         ;L0
 18008|     ;; nuke = i64 %745
 18009|  %746 = gep %688, i64 400                                                                                              ;L717
 18010|  %747 = getelementptr i64, ptr %746, i64 %32                                                                           ;L717
 18011|  %748 = load i64, ptr %747, , !!8                                                                                      ;L717
 18012|  %749 = gep %688, i64 440                                                                                              ;L717
 18013|  %750 = getelementptr i64, ptr %749, i64 %32                                                                           ;L717
 18014|  %751 = load i64, ptr %750, , !!8                                                                                      ;L717
 18015|  %752 = gep %688, i64 480                                                                                              ;L717
 18016|  %753 = getelementptr i64, ptr %752, i64 %32                                                                           ;L717
 18017|  %754 = load i64, ptr %753, , !!8                                                                                      ;L717
 18018|  %755 = add i64 %748, %761                                                                                             ;L717
 18019|  %756 = add i64 %755, %751                                                                                             ;L717
 18020|  %757 = add i64 %756, %754                                                                                             ;L717
 18021|     ;; kill_dps = i64 %757
 18022|  %758 = add i64 %745, %760                                                                                             ;L718
 18023|     ;; kill_nuke = i64 %758
 18024|     ;; rhs = i64 %758
 18025|  br label %759                                                                                                         ;L704
 18026| 
 18027| 759: ; preds = %744, %230
 18028|  %760 = phi i64 [ %758, %744 ], [ 0, %230 ]
 18029|  %761 = phi i64 [ %757, %744 ], [ 0, %230 ]
 18030|  br label %234                                                                                                         ;L365<64<704
 18031| 
 18032| 762: ; preds = %589, %245
 18033|  br label %234                                                                                                         ;L1
 18034| }
