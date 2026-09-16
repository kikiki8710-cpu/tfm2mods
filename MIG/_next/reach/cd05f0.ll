 66196| define internal fastcc void @ai::plan_legacy8sub_plan6battle18base_battle_action(ptr %0, i64 %1, ptr %2, ptr %3, i64 %4, i64 %5) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
 66197|  %7 = alloca [16 x i8],
 66198|  %8 = alloca [16 x i8],
 66199|  %9 = alloca [24 x i8],
 66200|  %10 = alloca [184 x i8],
 66204|  %11 = alloca [288 x i8],
 66205|  %12 = alloca [24 x i8],
 66206|  %13 = alloca [184 x i8],
 66211|  %14 = alloca [288 x i8],
 66213|  %15 = alloca [24 x i8],
 66214|  %16 = alloca [184 x i8],
 66215|  %17 = alloca [24 x i8],
 66217|  %18 = alloca [24 x i8],
 66218|  %19 = alloca [184 x i8],
 66219|  %20 = alloca [24 x i8],
 66220|  %21 = alloca [184 x i8],
 66221|  %22 = alloca [24 x i8],
 66222|  %23 = alloca [184 x i8],
 66224|  %24 = alloca [24 x i8],
 66225|  %25 = alloca [184 x i8],
 66226|  %26 = alloca [24 x i8],
 66227|  %27 = alloca [184 x i8],
 66228|  %28 = alloca [24 x i8],
 66229|  %29 = alloca [184 x i8],
 66231|  %30 = alloca [24 x i8],
 66232|  %31 = alloca [184 x i8],
 66233|  %32 = alloca [24 x i8],
 66234|  %33 = alloca [184 x i8],
 66235|  %34 = alloca [24 x i8],
 66236|  %35 = alloca [184 x i8],
 66238|  %36 = alloca [24 x i8],
 66239|  %37 = alloca [184 x i8],
 66240|  %38 = alloca [16 x i8],
 66241|  %39 = alloca [288 x i8],
 66242|  %40 = alloca [24 x i8],
 66243|  %41 = alloca [184 x i8],
 66250|  %42 = alloca [288 x i8],
 66252|  %43 = alloca [24 x i8],
 66253|  %44 = alloca [184 x i8],
 66254|  %45 = alloca [24 x i8],
 66256|  %46 = alloca [24 x i8],
 66257|  %47 = alloca [184 x i8],
 66260|  %48 = alloca [288 x i8],
 66261|  %49 = alloca [24 x i8],
 66262|  %50 = alloca [184 x i8],
 66269|  %51 = alloca [288 x i8],
 66271|  %52 = alloca [24 x i8],
 66272|  %53 = alloca [184 x i8],
 66273|  %54 = alloca [24 x i8],
 66275|  %55 = alloca [24 x i8],
 66276|  %56 = alloca [184 x i8],
 66277|  %57 = alloca [56 x i8],
 66278|  %58 = alloca [56 x i8],
 66279|  %59 = alloca [24 x i8],
 66280|  %60 = alloca [184 x i8],
 66281|  %61 = alloca [24 x i8],
 66282|  %62 = alloca [184 x i8],
 66284|  %63 = alloca [24 x i8],
 66285|  %64 = alloca [184 x i8],
 66287|  %65 = alloca [32 x i8],
 66288|  %66 = alloca [136 x i8],
 66289|  %67 = alloca [136 x i8],
 66290|  %68 = alloca [32 x i8],
 66291|  %69 = alloca [64 x i8],
 66292|  %70 = alloca [32 x i8],
 66293|  %71 = alloca [24 x i8],
 66294|  %72 = alloca [32 x i8],
 66295|  %73 = alloca [8 x i8],
 66296|  store i64 %1, ptr %73,
 66297|     ;; version = ptr %73
 66299|     ;; player = ptr %2
 66301|     ;; data = ptr %3
 66304|     ;; near_allies = ptr %72
 66305|     ;; near_enemies_with_action = ptr %70
 66306|     ;; enemy_towers = ptr %68
 66307|     ;; self = ptr %66
 66308|     ;; candidates = ptr %65
 66309|     ;; iter = ptr %57
 66310|     ;; buff = ptr %51
 66311|     ;; buff = ptr %48
 66312|     ;; buff = ptr %42
 66313|     ;; buff = ptr %39
 66314|     ;; buff = ptr %14
 66315|     ;; buff = ptr %11
 66316|     ;; concrete = ptr %8
 66317|     ;; concrete = ptr %7
 66318|     ;; len = i64 5
 66319|     ;; count = i64 5
 66320|     ;; len = i64 5
 66321|     ;; count = i64 5
 66324|     ;; count = i64 1
 66325|     ;; count = i64 1
 66326|     ;; __arg1_discr = i64 0
 66327|     ;; count = i64 1
 66328|     ;; count = i64 1
 66329|     ;; count = i64 1
 66330|     ;; count = i64 1
 66332|     ;; count = i64 1
 66333|     ;; count = i64 1
 66334|     ;; count = i64 1
 66337|     ;; count = i64 1
 66338|     ;; count = i64 1
 66339|  %74 = gep %2, i64 2352                                                                                                ;L1306
 66340|  %75 = load i64, ptr %74, , !!8                                                                                        ;L1306
 66341|     ;; team = i64 %75
 66342|  %76 = icmp ult i64 %75, 2                                                                                             ;L1306
 66343|  br i1 %76, label %78, label %77                                                                                       ;L1306
 66344| 
 66345| 77: ; preds = %6
 66346|  tail call void @core::panicking18panic_bounds_check(i64 %75, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.268) #31 ;L1306
 66347|  unreachable                                                                                                           ;L1306
 66348| 
 66349| 78: ; preds = %6
 66350|     ;; self = ptr %2
 66351|  %79 = gep %2, i64 2496                                                                                                ;L581<1306
 66352|  %80 = load i32, ptr %79, , !!8                                                                                        ;L581<1306
 66353|  %81 = zext nneg i32 %80 to i64                                                                                        ;L581<1306
 66354|  %82 = load ptr, ptr %3, , !!8, !!8                                                                                    ;L1306
 66355|     ;; self = ptr %82
 66356|  %83 = gep %82, i64 480                                                                                                ;L1306
 66357|  %84 = getelementptr [5 x ptr], ptr %83, i64 %75                                                                       ;L1306
 66358|  %85 = getelementptr ptr, ptr %84, i64 %81                                                                             ;L1306
 66359|  %86 = load ptr, ptr %85, , !!8                                                                                        ;L1306
 66360|     ;; self = ptr %86
 66361|  %87 = icmp eq ptr %86, null                                                                                           ;L1011<1306
 66362|  br i1 %87, label %90, label %88                                                                                       ;L1011<1306
 66363| 
 66364| 88: ; preds = %78
 66365|     ;; champ = ptr %86
 66366|     ;; self = ptr %86
 66367|     ;; self = ptr %86
 66368|     ;; entity = ptr %86
 66369|     ;; other = ptr %86
 66370|     ;; other = ptr %86
 66371|     ;; entity = ptr %86
 66372|     ;; other = ptr %86
 66373|     ;; entity = ptr %86
 66374|     ;; other = ptr %86
 66375|     ;; other = ptr %86
 66376|     ;; entity = ptr %86
 66377|     ;; other = ptr %86
 66378|     ;; other = ptr %86
 66380|     ;; entity = ptr %86
 66381|     ;; other = ptr %86
 66382|     ;; other = ptr %86
 66383|     ;; other = ptr %86
 66384|     ;; entity = ptr %86
 66385|     ;; other = ptr %86
 66386|     ;; other = ptr %86
 66387|     ;; other = ptr %86
 66388|     ;; entity = ptr %86
 66389|     ;; other = ptr %86
 66390|     ;; other = ptr %86
 66391|     ;; other = ptr %86
 66392|     ;; self = ptr %86
 66393|     ;; entity = ptr %86
 66394|     ;; self = ptr %86
 66395|     ;; caster = ptr %86
 66396|     ;; self = ptr %86
 66397|     ;; other = ptr %86
 66398|     ;; self = ptr %86
 66399|     ;; self = ptr %86
 66400|     ;; caster = ptr %86
 66401|     ;; self = ptr %86
 66402|     ;; other = ptr %86
 66403|     ;; self = ptr %86
 66404|     ;; self = ptr %86
 66405|     ;; self = ptr %86
 66406|  %89 = icmp eq i64 %4, 3                                                                                               ;L1312
 66407|  br i1 %89, label %91, label %96                                                                                       ;L1312
 66408| 
 66409| 90: ; preds = %78
 66410|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.269) #31                       ;L1013<1306
 66411|  unreachable                                                                                                           ;L1013<1306
 66412| 
 66413| 91: ; preds = %88
 66414|  %92 = icmp ugt i64 %1, 1                                                                                              ;L1313
 66415|  br i1 %92, label %93, label %96                                                                                       ;L1313
 66416| 
 66417| 93: ; preds = %91
 66418|  %94 = tail call zeroext i1 @ai::plan_legacy3old13defense_nexus18base_defense_focus(ptr %2, ptr %3)                    ;L1313
 66419|  %95 = select i1 %94, i64 30, i64 0                                                                                    ;L1361
 66420|  br label %96                                                                                                          ;L1313
 66421| 
 66422| 96: ; preds = %93, %91, %88
 66423|  %97 = phi i64 [ 30, %88 ], [ %95, %93 ], [ 0, %91 ]                                                                   ;L0
 66427|     ;; self[0..+8] = ptr %84
 66428|     ;; slice[0..+8] = ptr %84
 66429|     ;; self[8..+8] = i64 5
 66430|     ;; slice[8..+8] = i64 5
 66431|     ;; ptr = ptr %84
 66432|     ;; self = ptr %84
 66433|  %98 = gep %84, i64 40                                                                                                 ;L961<100<1042<1905<1316
 66434|     ;; self[0..+8] = ptr %84
 66435|     ;; self[8..+8] = ptr %98
 66436|     ;; predicate = ptr %86
 66437|  store ptr %84, ptr %71,                                                                                               ;L28<957<1316
 66438|  %99 = gep %71, i64 8                                                                                                  ;L28<957<1316
 66439|  store ptr %98, ptr %99,                                                                                               ;L28<957<1316
 66440|  %100 = gep %71, i64 16                                                                                                ;L28<957<1316
 66441|  store ptr %86, ptr %100,                                                                                              ;L28<957<1316
 66442|  %101 = gep %3, i64 8                                                                                                  ;L1316
 66443|  %102 = load ptr, ptr %101, , !!8, !!8                                                                                 ;L1316
 66444|  %103 = load ptr, ptr %102, , !!8, !!8                                                                                 ;L1316
 66445|     ;; bump = ptr %103
 66446|  call void @core::iter8adapters6filter6FilterINtNtB29_10filter_map9FilterMapINtNtNtB2d_5slice4iter4IterINtNtB2d_6option6OptionBU_EENCNvMs3_BZ_NtBZ_21AbstractGameWithCache14iter_champions0ENCNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6battle18base_battle_action0EEB5r_(ptr sret([32 x i8]) %72, ptr %71, ptr %103) ;L1315
 66450|  %104 = sub nuw nsw i64 1, %75                                                                                         ;L1319
 66451|     ;; team = i64 %104
 66452|  %105 = getelementptr [5 x ptr], ptr %83, i64 %104                                                                     ;L1319
 66453|     ;; self[0..+8] = ptr %105
 66454|     ;; slice[0..+8] = ptr %105
 66455|     ;; self[8..+8] = i64 5
 66456|     ;; slice[8..+8] = i64 5
 66457|     ;; ptr = ptr %105
 66458|     ;; self = ptr %105
 66459|  %106 = gep %105, i64 40                                                                                               ;L961<100<1042<1319
 66460|  %107 = gep %3, i64 16                                                                                                 ;L1320
 66461|  %108 = load ptr, ptr %107, , !!8, !!8                                                                                 ;L1320
 66462|     ;; self[0..+8] = ptr %105
 66463|     ;; self[8..+8] = ptr %106
 66464|     ;; self[16..+8] = i64 0
 66465|     ;; self[24..+8] = ptr %108
 66466|     ;; self[32..+8] = ptr %2
 66467|     ;; self[40..+8] = ptr %3
 66468|     ;; self[48..+8] = ptr %2
 66469|     ;; self[56..+8] = ptr %73
 66470|  store ptr %105, ptr %69,                                                                                              ;L69<836<1325
 66471|  %109 = gep %69, i64 8                                                                                                 ;L69<836<1325
 66472|  store ptr %106, ptr %109,                                                                                             ;L69<836<1325
 66473|  %110 = gep %69, i64 16                                                                                                ;L69<836<1325
 66474|  store i64 0, ptr %110,                                                                                                ;L69<836<1325
 66475|  %111 = gep %69, i64 24                                                                                                ;L69<836<1325
 66476|  store ptr %108, ptr %111,                                                                                             ;L69<836<1325
 66477|  %112 = gep %69, i64 32                                                                                                ;L69<836<1325
 66478|  store ptr %2, ptr %112,                                                                                               ;L69<836<1325
 66479|  %113 = gep %69, i64 40                                                                                                ;L69<836<1325
 66480|  store ptr %3, ptr %113,                                                                                               ;L69<836<1325
 66481|  %114 = gep %69, i64 48                                                                                                ;L69<836<1325
 66482|  store ptr %2, ptr %114,                                                                                               ;L69<836<1325
 66483|  %115 = gep %69, i64 56                                                                                                ;L69<836<1325
 66484|  store ptr %73, ptr %115,                                                                                              ;L69<836<1325
 66485|  invoke void @core::iter8adapters3map3MapINtNtB2P_6filter6FilterIB2L_INtNtB2P_9enumerate9EnumerateINtNtNtB2T_5slice4iter4IterINtNtB2T_6option6OptionB27_EEENCNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6battle18base_battle_actions_0ENCB5q_s0_0ENCB5q_s1_0EEB5y_(ptr sret([32 x i8]) %70, ptr %69, ptr %103)
 66486|  to label %119 unwind label %116                                                                                       ;L1318
 66487| 
 66488| 116: ; preds = %3597, %120, %96
 66489|  %117 = cleanuppad within none []
 66490|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEECshdEBA0ozCnw_7game_ai(ptr %72) #30 [ "funclet"(token %117) ] ;L2039
 66491|  cleanupret from %117 unwind to caller                                                                                 ;L1305
 66492| 
 66493| 118: ; preds = %2952, %2661, %2400, %2166, %1835, %1515, %957, %639, %532, %424, %278
 66494|  unreachable
 66495| 
 66496| 119: ; preds = %96
 66501|  invoke void @gc::simulationNtB5_21AbstractGameWithCache11iter_towers(ptr sret([136 x i8]) %66, ptr %82, i64 %104)
 66502|  to label %122 unwind label %120                                                                                       ;L1329
 66503| 
 66504| 120: ; preds = %2868, %178, %122, %119
 66505|  %121 = cleanuppad within none []
 66506|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecTNtNtNtNtCs97f5S1uJLkH_9game_core10simulation4game10blackboard11SmallActionRNtNtB1w_6entity6EntityEEECshdEBA0ozCnw_7game_ai(ptr %70) #30 [ "funclet"(token %121) ] ;L2039
 66507|  cleanupret from %121 unwind label %116                                                                                ;L2039
 66508| 
 66509| 122: ; preds = %119
 66510|  call void @llvm.memcpy.p0.p0.i64(ptr %67, ptr %66, i64 136, i1 false)                                                 ;L28<957<1329
 66512|  invoke void @core::iter8adapters6filter6FilterINtNtB29_5chain5ChainIB2Z_INtNtB29_7flatten7FlattenINtNtNtB2d_5array4iter8IntoIterINtNtB2d_6option6OptionBU_EKj6_EEINtNtB29_6copied6CopiedINtNtNtB2d_5slice4iter4IterBU_EEEINtB4l_8IntoIterBU_EENCNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6battle18base_battle_actions2_0EEB6e_(ptr sret([32 x i8]) %68, ptr %67, ptr %103)
 66513|  to label %123 unwind label %120                                                                                       ;L1328
 66514| 
 66515| 123: ; preds = %122
 66518|  store ptr inttoptr (i64 8 to ptr), ptr %65,                                                                           ;L547<1331
 66519|  %124 = gep %65, i64 8                                                                                                 ;L547<1331
 66520|  store ptr %103, ptr %124,                                                                                             ;L547<1331
 66521|  %125 = gep %65, i64 16                                                                                                ;L547<1331
 66522|  %126 = gep %65, i64 24                                                                                                ;L547<1331
 66523|     ;; self = ptr %86
 66524|  %127 = gep %86, i64 1168                                                                                              ;L742<1333
 66525|  %128 = gep %86, i64 1216                                                                                              ;L742<1333
 66526|  call void @llvm.memset.p0.i64(ptr %125, i8 0, i64 16, i1 false)                                                       ;L547<1331
 66527|  %129 = load i32, ptr %128, , !!8                                                                                      ;L742<1333
 66528|  %130 = icmp eq i32 %129, -1                                                                                           ;L742<1333
 66530|     ;; self = ptr %86
 66531|  %131 = gep %86, i64 1272                                                                                              ;L742<1334
 66532|  %132 = load i32, ptr %131, , !!8                                                                                      ;L742<1334
 66533|  %133 = icmp eq i32 %132, -1                                                                                           ;L742<1334
 66534|  %134 = gep %86, i64 1224                                                                                              ;L742<1334
 66536|  %135 = gep %86, i64 1480                                                                                              ;L1693<1335
 66537|  %136 = load i64, ptr %135, , !!8                                                                                      ;L1693<1335
 66538|  %137 = icmp ugt i64 %136, 2                                                                                           ;L1693<1335
 66539|  %138 = gep %86, i64 1280                                                                                              ;L1693<1335
 66540|  %139 = select i1 %137, ptr %138, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.19                                        ;L1693<1335
 66541|     ;; self = ptr %139
 66542|  %140 = gep %139, i64 48                                                                                               ;L742<1335
 66543|  %141 = load i32, ptr %140, , !!8                                                                                      ;L742<1335
 66544|  %142 = icmp eq i32 %141, -1                                                                                           ;L742<1335
 66546|  %143 = icmp ugt i64 %136, 4                                                                                           ;L1701<1336
 66547|  %144 = gep %86, i64 1336                                                                                              ;L1701<1336
 66548|  %145 = select i1 %143, ptr %144, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.19                                        ;L1701<1336
 66549|     ;; self = ptr %145
 66550|  %146 = gep %145, i64 48                                                                                               ;L742<1336
 66551|  %147 = load i32, ptr %146, , !!8                                                                                      ;L742<1336
 66552|  %148 = icmp eq i32 %147, -1                                                                                           ;L742<1336
 66555|     ;; f = ptr %86
 66556|  br i1 %130, label %176, label %149                                                                                    ;L1161<1339
 66557| 
 66558| 149: ; preds = %123
 66560|  %150 = gep %86, i64 1184                                                                                              ;L1162<1339
 66561|  %151 = load i64, ptr %150, , !!8                                                                                      ;L1162<1339
 66562|  %152 = gep %86, i64 1192                                                                                              ;L1162<1339
 66563|  %153 = load i64, ptr %152, , !!8                                                                                      ;L1162<1339
 66567|     ;; caster = ptr %86
 66568|     ;; self = ptr %86
 66569|  %154 = gep %86, i64 1080                                                                                              ;L26<1339<1162<1339
 66570|  %155 = load i64, ptr %154, , !!8                                                                                      ;L26<1339<1162<1339
 66571|  %156 = gep %86, i64 1136                                                                                              ;L1511<1339<1162<1339
 66572|  %157 = load i32, ptr %156, , !!8                                                                                      ;L1511<1339<1162<1339
 66573|     ;; mult = i32 %157
 66574|  %158 = icmp eq i32 %157, 0                                                                                            ;L1512<1339<1162<1339
 66575|  br i1 %158, label %159, label %162                                                                                    ;L1512<1339<1162<1339
 66576| 
 66577| 159: ; preds = %149
 66578|  %160 = gep %86, i64 1664                                                                                              ;L1513<1339<1162<1339
 66579|  %161 = load i64, ptr %160, , !!8                                                                                      ;L1513<1339<1162<1339
 66580|  br label %169                                                                                                         ;L1512<1339<1162<1339
 66581| 
 66582| 162: ; preds = %149
 66583|  %163 = sext i32 %157 to i64                                                                                           ;L1511<1339<1162<1339
 66584|     ;; mult = i64 %163
 66585|  %164 = gep %86, i64 1664                                                                                              ;L1515<1339<1162<1339
 66586|  %165 = load i64, ptr %164, , !!8                                                                                      ;L1515<1339<1162<1339
 66587|  %166 = add nsw i64 %163, 100                                                                                          ;L1515<1339<1162<1339
 66588|  %167 = mul i64 %165, %166                                                                                             ;L1515<1339<1162<1339
 66589|  %168 = udiv i64 %167, 100                                                                                             ;L1515<1339<1162<1339
 66590|  br label %169                                                                                                         ;L1512<1339<1162<1339
 66591| 
 66592| 169: ; preds = %162, %159
 66593|  %170 = phi i64 [ %161, %159 ], [ %168, %162 ]                                                                         ;L0<1339<1162<1339
 66594|  %171 = add i64 %136, -1                                                                                               ;L26<1339<1162<1339
 66595|  %172 = mul i64 %153, %171                                                                                             ;L26<1339<1162<1339
 66596|  %173 = add i64 %155, %151                                                                                             ;L26<1339<1162<1339
 66597|  %174 = add i64 %173, %172                                                                                             ;L26<1339<1162<1339
 66598|  %175 = add i64 %174, %170                                                                                             ;L1339<1162<1339
 66599|  br label %176                                                                                                         ;L1339<1162<1339
 66600| 
 66601| 176: ; preds = %169, %123
 66602|  %177 = phi i64 [ undef, %123 ], [ %175, %169 ]                                                                        ;L0<1339
 66604|     ;; atk_base[8..+8] = i64 %177
 66606|     ;; f = ptr %86
 66607|  br i1 %133, label %207, label %180                                                                                    ;L1161<1340
 66608| 
 66609| 178: ; preds = %3576, %3573, %3547, %3491, %3478, %3464, %3447, %3445, %3436, %3433, %3425, %3414, %3407, %3311, %3265, %3220, %3190, %3176, %3160, %3158, %3148, %3143, %3132, %3122, %3118, %3116, %3099, %3093, %3079, %3066, %3004, %2997, %2967, %2956, %2953, %2952, %2895, %2886, %2848, %2841, %2831, %2792, %2789, %2786, %2779, %2769, %2730, %2727, %2723, %2716, %2706, %2667, %2663, %2661, %2641, %2618, %2608, %2568, %2560, %2556, %2552, %2545, %2535, %2495, %2487, %2483, %2479, %2472, %2462, %2422, %2414, %2410, %2400, %2369, %2359, %2318, %2314, %2310, %2303, %2293, %2252, %2248, %2244, %2237, %2227, %2186, %2182, %2166, %2129, %2118, %2108, %2105, %2097, %2090, %2084, %2061, %2049, %2047, %2038, %2035, %2027, %2016, %2009, %1947, %1892, %1873, %1851, %1844, %1835, %1795, %1698, %1685, %1673, %1663, %1658, %1647, %1637, %1633, %1631, %1614, %1608, %1594, %1581, %1538, %1531, %1519, %1516, %1515, %1450, %1443, %1432, %1428, %1424, %1376, %1326, %1278, %1233, %1191, %1169, %1167, %1158, %1155, %1147, %1136, %1131, %1069, %1014, %995, %973, %966, %957, %917, %820, %807, %795, %785, %780, %769, %759, %755, %753, %736, %732, %718, %705, %662, %655, %643, %640, %639, %590, %582, %572, %533, %532, %496, %482, %474, %466, %448, %444, %436, %425, %424, %412, %401, %398, %352, %341, %331, %318, %279, %278, %240
 66610|  %179 = cleanuppad within none []
 66611|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %65) #30 [ "funclet"(token %179) ] ;L2039
 66612|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEECshdEBA0ozCnw_7game_ai(ptr %68) #30 [ "funclet"(token %179) ] ;L2039
 66613|  cleanupret from %179 unwind label %120                                                                                ;L2039
 66614| 
 66615| 180: ; preds = %176
 66617|  %181 = gep %86, i64 1240                                                                                              ;L1162<1340
 66618|  %182 = load i64, ptr %181, , !!8                                                                                      ;L1162<1340
 66619|  %183 = gep %86, i64 1248                                                                                              ;L1162<1340
 66620|  %184 = load i64, ptr %183, , !!8                                                                                      ;L1162<1340
 66624|     ;; caster = ptr %86
 66625|     ;; self = ptr %86
 66626|  %185 = gep %86, i64 1080                                                                                              ;L26<1340<1162<1340
 66627|  %186 = load i64, ptr %185, , !!8                                                                                      ;L26<1340<1162<1340
 66628|  %187 = gep %86, i64 1136                                                                                              ;L1511<1340<1162<1340
 66629|  %188 = load i32, ptr %187, , !!8                                                                                      ;L1511<1340<1162<1340
 66630|     ;; mult = i32 %188
 66631|  %189 = icmp eq i32 %188, 0                                                                                            ;L1512<1340<1162<1340
 66632|  br i1 %189, label %190, label %193                                                                                    ;L1512<1340<1162<1340
 66633| 
 66634| 190: ; preds = %180
 66635|  %191 = gep %86, i64 1664                                                                                              ;L1513<1340<1162<1340
 66636|  %192 = load i64, ptr %191, , !!8                                                                                      ;L1513<1340<1162<1340
 66637|  br label %200                                                                                                         ;L1512<1340<1162<1340
 66638| 
 66639| 193: ; preds = %180
 66640|  %194 = sext i32 %188 to i64                                                                                           ;L1511<1340<1162<1340
 66641|     ;; mult = i64 %194
 66642|  %195 = gep %86, i64 1664                                                                                              ;L1515<1340<1162<1340
 66643|  %196 = load i64, ptr %195, , !!8                                                                                      ;L1515<1340<1162<1340
 66644|  %197 = add nsw i64 %194, 100                                                                                          ;L1515<1340<1162<1340
 66645|  %198 = mul i64 %196, %197                                                                                             ;L1515<1340<1162<1340
 66646|  %199 = udiv i64 %198, 100                                                                                             ;L1515<1340<1162<1340
 66647|  br label %200                                                                                                         ;L1512<1340<1162<1340
 66648| 
 66649| 200: ; preds = %193, %190
 66650|  %201 = phi i64 [ %192, %190 ], [ %199, %193 ]                                                                         ;L0<1340<1162<1340
 66651|  %202 = add i64 %136, -1                                                                                               ;L26<1340<1162<1340
 66652|  %203 = mul i64 %184, %202                                                                                             ;L26<1340<1162<1340
 66653|  %204 = add i64 %186, %182                                                                                             ;L26<1340<1162<1340
 66654|  %205 = add i64 %204, %203                                                                                             ;L26<1340<1162<1340
 66655|  %206 = add i64 %205, %201                                                                                             ;L1340<1162<1340
 66656|  br label %207                                                                                                         ;L1340<1162<1340
 66657| 
 66658| 207: ; preds = %200, %176
 66659|  %208 = phi i64 [ undef, %176 ], [ %206, %200 ]                                                                        ;L0<1340
 66661|     ;; skill_base[8..+8] = i64 %208
 66663|     ;; f = ptr %86
 66664|  br i1 %142, label %236, label %209                                                                                    ;L1161<1341
 66665| 
 66666| 209: ; preds = %207
 66668|  %210 = gep %139, i64 16                                                                                               ;L1162<1341
 66669|  %211 = load i64, ptr %210, , !!8                                                                                      ;L1162<1341
 66670|  %212 = gep %139, i64 24                                                                                               ;L1162<1341
 66671|  %213 = load i64, ptr %212, , !!8                                                                                      ;L1162<1341
 66675|     ;; caster = ptr %86
 66676|     ;; self = ptr %86
 66677|  %214 = gep %86, i64 1080                                                                                              ;L26<1341<1162<1341
 66678|  %215 = load i64, ptr %214, , !!8                                                                                      ;L26<1341<1162<1341
 66679|  %216 = gep %86, i64 1136                                                                                              ;L1511<1341<1162<1341
 66680|  %217 = load i32, ptr %216, , !!8                                                                                      ;L1511<1341<1162<1341
 66681|     ;; mult = i32 %217
 66682|  %218 = icmp eq i32 %217, 0                                                                                            ;L1512<1341<1162<1341
 66683|  br i1 %218, label %219, label %222                                                                                    ;L1512<1341<1162<1341
 66684| 
 66685| 219: ; preds = %209
 66686|  %220 = gep %86, i64 1664                                                                                              ;L1513<1341<1162<1341
 66687|  %221 = load i64, ptr %220, , !!8                                                                                      ;L1513<1341<1162<1341
 66688|  br label %229                                                                                                         ;L1512<1341<1162<1341
 66689| 
 66690| 222: ; preds = %209
 66691|  %223 = sext i32 %217 to i64                                                                                           ;L1511<1341<1162<1341
 66692|     ;; mult = i64 %223
 66693|  %224 = gep %86, i64 1664                                                                                              ;L1515<1341<1162<1341
 66694|  %225 = load i64, ptr %224, , !!8                                                                                      ;L1515<1341<1162<1341
 66695|  %226 = add nsw i64 %223, 100                                                                                          ;L1515<1341<1162<1341
 66696|  %227 = mul i64 %225, %226                                                                                             ;L1515<1341<1162<1341
 66697|  %228 = udiv i64 %227, 100                                                                                             ;L1515<1341<1162<1341
 66698|  br label %229                                                                                                         ;L1512<1341<1162<1341
 66699| 
 66700| 229: ; preds = %222, %219
 66701|  %230 = phi i64 [ %221, %219 ], [ %228, %222 ]                                                                         ;L0<1341<1162<1341
 66702|  %231 = add i64 %136, -1                                                                                               ;L26<1341<1162<1341
 66703|  %232 = mul i64 %213, %231                                                                                             ;L26<1341<1162<1341
 66704|  %233 = add i64 %215, %211                                                                                             ;L26<1341<1162<1341
 66705|  %234 = add i64 %233, %232                                                                                             ;L26<1341<1162<1341
 66706|  %235 = add i64 %234, %230                                                                                             ;L1341<1162<1341
 66707|  br label %236                                                                                                         ;L1341<1162<1341
 66708| 
 66709| 236: ; preds = %229, %207
 66710|  %237 = phi i64 [ undef, %207 ], [ %235, %229 ]                                                                        ;L0<1341
 66712|     ;; skill2_base[8..+8] = i64 %237
 66713|  %238 = gep %86, i64 1600                                                                                              ;L1343
 66714|  %239 = load i64, ptr %238, , !!8                                                                                      ;L1343
 66715|     ;; move_speed = i64 %239
 66716|     ;; self = i64 %239
 66717|     ;; self = i64 %239
 66718|     ;; self = i64 %239
 66719|     ;; self = i64 %239
 66720|  br i1 %130, label %446, label %240                                                                                    ;L1345
 66721| 
 66722| 240: ; preds = %236
 66723|  %241 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_attack(ptr %86)
 66724|  to label %242 unwind label %178                                                                                       ;L1345
 66725| 
 66726| 242: ; preds = %240
 66727|  br i1 %241, label %243, label %444                                                                                    ;L1345
 66728| 
 66729| 243: ; preds = %242
 66730|     ;; self = ptr %70
 66731|     ;; self = ptr %70
 66732|     ;; self = ptr %70
 66733|  %244 = load ptr, ptr %70, , !!8, !!8                                                                                  ;L138<2073<2136<1346
 66734|     ;; p = ptr %244
 66735|  %245 = gep %70, i64 24                                                                                                ;L2075<2136<1346
 66736|  %246 = load i64, ptr %245, , !!8                                                                                      ;L2075<2136<1346
 66737|     ;; len = i64 %246
 66738|     ;; count = i64 %246
 66739|     ;; self[0..+8] = ptr %244
 66740|     ;; slice[0..+8] = ptr %244
 66741|     ;; self[8..+8] = i64 %246
 66742|     ;; slice[8..+8] = i64 %246
 66743|     ;; ptr = ptr %244
 66744|     ;; self = ptr %244
 66745|  %247 = gepS }, ptr %244, i64 %246                                                                                     ;L961<100<1042<2136<1346
 66746|     ;; iter[0..+8] = ptr %244
 66747|     ;; iter[8..+8] = ptr %247
 66748|  %248 = gep %86, i64 8
 66749|  %249 = gep %86, i64 1632
 66750|  %250 = gep %86, i64 1640
 66751|  %251 = gep %64, i64 177
 66752|  br label %252                                                                                                         ;L1346
 66753| 
 66754| 252: ; preds = %348, %243
 66755|  %253 = phi ptr [ %244, %243 ], [ %256, %348 ]                                                                         ;L1346
 66756|     ;; iter[0..+8] = ptr %253
 66757|     ;; self = ptr undef
 66758|     ;; ptr = ptr %253
 66759|     ;; self = ptr %253
 66760|     ;; end_or_len = ptr %247
 66763|  %254 = icmp eq ptr %253, %247                                                                                         ;L1714<180<1346
 66764|  br i1 %254, label %261, label %255                                                                                    ;L180<1346
 66765| 
 66766| 255: ; preds = %252
 66767|  %256 = gep %253, i64 32                                                                                               ;L656<185<1346
 66768|     ;; iter[0..+8] = ptr %256
 66769|     ;; a = ptr %253
 66770|     ;; e = ptr %253
 66771|  %257 = gep %253, i64 24                                                                                               ;L1347
 66772|  %258 = load ptr, ptr %257, , !!8, !!8                                                                                 ;L1347
 66773|     ;; self = ptr %258
 66774|     ;; self = ptr %258
 66775|     ;; self = ptr %258
 66776|     ;; self = ptr %86
 66777|  %259 = load i64, ptr %86, , !!8                                                                                       ;L1136<1482<1347
 66778|  %260 = trunc nuw i64 %259 to i1                                                                                       ;L1136<1482<1347
 66779|  br i1 %260, label %279, label %270                                                                                    ;L1136<1482<1347
 66780| 
 66781| 261: ; preds = %252
 66782|     ;; self = ptr %68
 66783|     ;; self = ptr %68
 66784|     ;; self = ptr %68
 66785|  %262 = load ptr, ptr %68, , !!8, !!8                                                                                  ;L138<2073<2136<1375
 66786|     ;; p = ptr %262
 66787|  %263 = gep %68, i64 24                                                                                                ;L2075<2136<1375
 66788|  %264 = load i64, ptr %263, , !!8                                                                                      ;L2075<2136<1375
 66789|     ;; len = i64 %264
 66790|     ;; count = i64 %264
 66791|     ;; self[0..+8] = ptr %262
 66792|     ;; slice[0..+8] = ptr %262
 66793|     ;; self[8..+8] = i64 %264
 66794|     ;; slice[8..+8] = i64 %264
 66795|     ;; ptr = ptr %262
 66796|     ;; self = ptr %262
 66797|  %265 = getelementptr ptr, ptr %262, i64 %264                                                                          ;L961<100<1042<2136<1375
 66798|     ;; iter[0..+8] = ptr %262
 66799|     ;; iter[8..+8] = ptr %265
 66800|  %266 = mul i64 %239, %97
 66801|  %267 = add i64 %266, %177
 66802|  %268 = gep %60, i64 177
 66803|  %269 = gep %62, i64 177
 66804|  br label %349                                                                                                         ;L1375
 66805| 
 66806| 270: ; preds = %255
 66807|     ;; team = ptr %86
 66808|  %271 = load i64, ptr %248, , !!8                                                                                      ;L1137<1482<1347
 66809|     ;; team = i64 %271
 66810|  %272 = icmp ult i64 %271, 2                                                                                           ;L1483<1347
 66811|  br i1 %272, label %273, label %278                                                                                    ;L1483<1347
 66812| 
 66813| 273: ; preds = %270
 66815|  %274 = gep %258, i64 56                                                                                               ;L122<1483<1347
 66816|  %275 = gepS %274, i64 %271                                                                                            ;L122<1483<1347
 66817|  %276 = load i64, ptr %275, , !!8                                                                                      ;L122<1483<1347
 66818|  %277 = icmp eq i64 %276, 0                                                                                            ;L122<1483<1347
 66819|  br i1 %277, label %279, label %348                                                                                    ;L1347
 66820| 
 66821| 278: ; preds = %270
 66822|  invoke void @core::panicking18panic_bounds_check(i64 %271, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 66823|  to label %118 unwind label %178                                                                                       ;L1483<1347
 66824| 
 66825| 279: ; preds = %273, %255
 66826|  %280 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %127, ptr %86, ptr %258)
 66827|  to label %281 unwind label %178                                                                                       ;L1351
 66828| 
 66829| 281: ; preds = %279
 66830|  %282 = add i64 %280, %177                                                                                             ;L1351
 66831|  %283 = gep %258, i64 1136                                                                                             ;L1511<1351
 66832|  %284 = load i32, ptr %283, , !!8                                                                                      ;L1511<1351
 66833|     ;; mult = i32 %284
 66834|  %285 = icmp eq i32 %284, 0                                                                                            ;L1512<1351
 66835|  br i1 %285, label %286, label %289                                                                                    ;L1512<1351
 66836| 
 66837| 286: ; preds = %281
 66838|  %287 = gep %258, i64 1664                                                                                             ;L1513<1351
 66839|  %288 = load i64, ptr %287, , !!8                                                                                      ;L1513<1351
 66840|  br label %296                                                                                                         ;L1512<1351
 66841| 
 66842| 289: ; preds = %281
 66843|  %290 = sext i32 %284 to i64                                                                                           ;L1511<1351
 66844|     ;; mult = i64 %290
 66845|  %291 = gep %258, i64 1664                                                                                             ;L1515<1351
 66846|  %292 = load i64, ptr %291, , !!8                                                                                      ;L1515<1351
 66847|  %293 = add nsw i64 %290, 100                                                                                          ;L1515<1351
 66848|  %294 = mul i64 %292, %293                                                                                             ;L1515<1351
 66849|  %295 = udiv i64 %294, 100                                                                                             ;L1515<1351
 66850|  br label %296                                                                                                         ;L1512<1351
 66851| 
 66852| 296: ; preds = %289, %286
 66853|  %297 = phi i64 [ %288, %286 ], [ %295, %289 ]                                                                         ;L0<1351
 66854|  %298 = add i64 %282, %297                                                                                             ;L1351
 66855|     ;; range = i64 %298
 66856|  %299 = gep %258, i64 1632                                                                                             ;L2158<1352
 66857|  %300 = load i64, ptr %299, , !!8                                                                                      ;L2158<1352
 66858|     ;; x1 = i64 %300
 66859|     ;; self = i64 %300
 66860|  %301 = gep %258, i64 1640                                                                                             ;L2158<1352
 66861|  %302 = load i64, ptr %301, , !!8                                                                                      ;L2158<1352
 66862|     ;; y1 = i64 %302
 66863|     ;; self = i64 %302
 66864|  %303 = load i64, ptr %249, , !!8                                                                                      ;L2158<1352
 66865|     ;; x2 = i64 %303
 66866|     ;; other = i64 %303
 66867|  %304 = load i64, ptr %250, , !!8                                                                                      ;L2158<1352
 66868|     ;; y2 = i64 %304
 66869|     ;; other = i64 %304
 66870|  %305 = icmp ult i64 %300, %303                                                                                        ;L3147<7<2158<1352
 66871|  %306 = sub nuw i64 %303, %300                                                                                         ;L3147<7<2158<1352
 66872|  %307 = sub nuw i64 %300, %303                                                                                         ;L3147<7<2158<1352
 66873|  %308 = select i1 %305, i64 %306, i64 %307                                                                             ;L3147<7<2158<1352
 66874|     ;; dx = i64 %308
 66875|  %309 = icmp ult i64 %302, %304                                                                                        ;L3147<8<2158<1352
 66876|  %310 = sub nuw i64 %304, %302                                                                                         ;L3147<8<2158<1352
 66877|  %311 = sub nuw i64 %302, %304                                                                                         ;L3147<8<2158<1352
 66878|  %312 = select i1 %309, i64 %310, i64 %311                                                                             ;L3147<8<2158<1352
 66879|     ;; dy = i64 %312
 66880|  %313 = mul i64 %308, %308                                                                                             ;L9<2158<1352
 66881|  %314 = mul i64 %312, %312                                                                                             ;L9<2158<1352
 66882|  %315 = add i64 %314, %313                                                                                             ;L9<2158<1352
 66883|     ;; dist_sq = i64 %315
 66884|  %316 = load i64, ptr %253, , !!8                                                                                      ;L1355
 66887|     ;; __self_discr = i64 %316
 66888|     ;; __arg1_discr = i64 0
 66889|  %317 = icmp eq i64 %316, 0                                                                                            ;L81<1355
 66890|  br i1 %317, label %318, label %325                                                                                    ;L1355
 66891| 
 66892| 318: ; preds = %296
 66893|  %319 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10block_move(ptr %258)
 66894|  to label %320 unwind label %178                                                                                       ;L1355
 66895| 
 66896| 320: ; preds = %318
 66897|  br i1 %319, label %325, label %321                                                                                    ;L1355
 66898| 
 66899| 321: ; preds = %320
 66900|  %322 = gep %258, i64 1600                                                                                             ;L1356
 66901|  %323 = load i64, ptr %322, , !!8                                                                                      ;L1356
 66902|     ;; rhs = i64 %323
 66903|  %324 = call i64 @llvm.usub.sat.i64(i64 %239, i64 %323)                                                                ;L2472<1356
 66904|     ;; move_speed = i64 %324
 66905|  br label %325                                                                                                         ;L1355
 66906| 
 66907| 325: ; preds = %321, %320, %296
 66908|  %326 = phi i64 [ %324, %321 ], [ %239, %320 ], [ %239, %296 ]                                                         ;L0
 66909|     ;; move_speed = i64 %326
 66910|     ;; max_tick = i64 %97
 66911|  %327 = mul i64 %326, %97                                                                                              ;L1367
 66912|  %328 = add i64 %298, %327                                                                                             ;L1367
 66913|     ;; max_dist = i64 %328
 66914|  %329 = mul i64 %328, %328                                                                                             ;L1368
 66915|  %330 = icmp ugt i64 %315, %329                                                                                        ;L1368
 66916|  br i1 %330, label %348, label %331                                                                                    ;L1368
 66917| 
 66918| 331: ; preds = %325
 66921|  %332 = gep %258, i64 1472                                                                                             ;L1372
 66922|  %333 = load i64, ptr %332, , !!8                                                                                      ;L1372
 66923|  invoke void @ai::small_action4castNtB2_17SmallActionAttack3new(ptr sret([24 x i8]) %63, ptr %3, i64 %333)
 66924|  to label %334 unwind label %178                                                                                       ;L1372
 66925| 
 66926| 334: ; preds = %331
 66927|  call void @llvm.memcpy.p0.p0.i64(ptr %64, ptr %63, i64 24, i1 false)                                                  ;L1372
 66928|  store i8 15, ptr %251,                                                                                                ;L1372
 66931|     ;; self = ptr %65
 66932|     ;; self = ptr %65
 66933|     ;; value = ptr %64
 66934|     ;; src = ptr %64
 66935|     ;; additional = i64 1
 66936|     ;; needed_extra_cap = i64 1
 66937|     ;; needed_extra_cap = i64 1
 66938|     ;; strategy = i8 1
 66939|  %335 = load i64, ptr %126, , !!73571, !!8                                                                             ;L1428<1372
 66940|     ;; self = ptr %65
 66941|  %336 = load i64, ptr %125, , !!73571, !!8                                                                             ;L149<1428<1372
 66942|  %337 = icmp eq i64 %335, %336                                                                                         ;L1428<1372
 66943|  br i1 %337, label %338, label %343                                                                                    ;L1428<1372
 66944| 
 66945| 338: ; preds = %334
 66946|     ;; self = ptr %65
 66947|     ;; self = ptr %65
 66948|     ;; self = ptr %65
 66949|     ;; used_cap = i64 %335
 66950|     ;; used_cap = i64 %335
 66951|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %335, i64 1, i1 zeroext true)
 66952|  to label %339 unwind label %341, !!73571                                                                              ;L619<430<738<1429<1372
 66953| 
 66954| 339: ; preds = %338
 66955|  %340 = load i64, ptr %126, , !!73571                                                                                  ;L1432<1372
 66956|  br label %343                                                                                                         ;L619<430<738<1429<1372
 66957| 
 66958| 341: ; preds = %338
 66959|  %342 = cleanuppad within none []
 66960|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %64) #30 [ "funclet"(token %342) ], !!73556 ;L1436<1372
 66961|  cleanupret from %342 unwind label %178
 66962| 
 66963| 343: ; preds = %339, %334
 66964|  %344 = phi i64 [ %340, %339 ], [ %335, %334 ]                                                                         ;L1432<1372
 66965|     ;; self = ptr %65
 66966|  %345 = load ptr, ptr %65, , !!73571, !!8, !!8                                                                         ;L138<1432<1372
 66967|     ;; self = ptr %345
 66968|     ;; count = i64 %344
 66969|  %346 = gepS %345, i64 %344                                                                                            ;L961<1432<1372
 66970|     ;; end = ptr %346
 66971|     ;; dst = ptr %346
 66972|  call void @llvm.memcpy.p0.p0.i64(ptr %346, ptr %64, i64 184, i1 false), !!73556                                       ;L1933<1433<1372
 66973|  %347 = add i64 %344, 1                                                                                                ;L1434<1372
 66974|  store i64 %347, ptr %126, , !!73571                                                                                   ;L1434<1372
 66976|  br label %348                                                                                                         ;L1346
 66977| 
 66978| 348: ; preds = %343, %325, %273
 66979|  br label %252                                                                                                         ;L1714<180<1346
 66980| 
 66981| 349: ; preds = %443, %261
 66982|  %350 = phi ptr [ %262, %261 ], [ %353, %443 ]                                                                         ;L1375
 66983|     ;; iter[0..+8] = ptr %350
 66984|     ;; self = ptr undef
 66985|     ;; ptr = ptr %350
 66986|     ;; self = ptr %350
 66987|     ;; end_or_len = ptr %265
 66990|  %351 = icmp eq ptr %350, %265                                                                                         ;L1714<180<1375
 66991|  br i1 %351, label %444, label %352                                                                                    ;L180<1375
 66992| 
 66993| 352: ; preds = %349
 66994|  %353 = gep %350, i64 8                                                                                                ;L656<185<1375
 66995|     ;; iter[0..+8] = ptr %353
 66996|     ;; e = ptr %350
 66997|  %354 = load ptr, ptr %350, , !!8, !!8                                                                                 ;L1376
 66998|  %355 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %127, ptr %86, ptr %354)
 66999|  to label %356 unwind label %178                                                                                       ;L1376
 67000| 
 67001| 356: ; preds = %352
 67002|  %357 = load ptr, ptr %350, , !!8, !!8                                                                                 ;L1376
 67003|     ;; self = ptr %357
 67004|  %358 = gep %357, i64 1136                                                                                             ;L1511<1376
 67005|  %359 = load i32, ptr %358, , !!8                                                                                      ;L1511<1376
 67006|     ;; mult = i32 %359
 67007|  %360 = icmp eq i32 %359, 0                                                                                            ;L1512<1376
 67008|  br i1 %360, label %361, label %364                                                                                    ;L1512<1376
 67009| 
 67010| 361: ; preds = %356
 67011|  %362 = gep %357, i64 1664                                                                                             ;L1513<1376
 67012|  %363 = load i64, ptr %362, , !!8                                                                                      ;L1513<1376
 67013|  br label %371                                                                                                         ;L1512<1376
 67014| 
 67015| 364: ; preds = %356
 67016|  %365 = sext i32 %359 to i64                                                                                           ;L1511<1376
 67017|     ;; mult = i64 %365
 67018|  %366 = gep %357, i64 1664                                                                                             ;L1515<1376
 67019|  %367 = load i64, ptr %366, , !!8                                                                                      ;L1515<1376
 67020|  %368 = add nsw i64 %365, 100                                                                                          ;L1515<1376
 67021|  %369 = mul i64 %367, %368                                                                                             ;L1515<1376
 67022|  %370 = udiv i64 %369, 100                                                                                             ;L1515<1376
 67023|  br label %371                                                                                                         ;L1512<1376
 67024| 
 67025| 371: ; preds = %364, %361
 67026|  %372 = phi i64 [ %363, %361 ], [ %370, %364 ]                                                                         ;L0<1376
 67028|     ;; self = ptr %357
 67029|  %373 = gep %357, i64 1632                                                                                             ;L2158<1377
 67030|  %374 = load i64, ptr %373, , !!8                                                                                      ;L2158<1377
 67031|     ;; x1 = i64 %374
 67032|     ;; self = i64 %374
 67033|  %375 = gep %357, i64 1640                                                                                             ;L2158<1377
 67034|  %376 = load i64, ptr %375, , !!8                                                                                      ;L2158<1377
 67035|     ;; y1 = i64 %376
 67036|     ;; self = i64 %376
 67037|  %377 = load i64, ptr %249, , !!8                                                                                      ;L2158<1377
 67038|     ;; x2 = i64 %377
 67039|     ;; other = i64 %377
 67040|  %378 = load i64, ptr %250, , !!8                                                                                      ;L2158<1377
 67041|     ;; y2 = i64 %378
 67042|     ;; other = i64 %378
 67043|  %379 = icmp ult i64 %374, %377                                                                                        ;L3147<7<2158<1377
 67044|  %380 = sub nuw i64 %377, %374                                                                                         ;L3147<7<2158<1377
 67045|  %381 = sub nuw i64 %374, %377                                                                                         ;L3147<7<2158<1377
 67046|  %382 = select i1 %379, i64 %380, i64 %381                                                                             ;L3147<7<2158<1377
 67047|     ;; dx = i64 %382
 67048|  %383 = icmp ult i64 %376, %378                                                                                        ;L3147<8<2158<1377
 67049|  %384 = sub nuw i64 %378, %376                                                                                         ;L3147<8<2158<1377
 67050|  %385 = sub nuw i64 %376, %378                                                                                         ;L3147<8<2158<1377
 67051|  %386 = select i1 %383, i64 %384, i64 %385                                                                             ;L3147<8<2158<1377
 67052|     ;; dy = i64 %386
 67053|  %387 = mul i64 %382, %382                                                                                             ;L9<2158<1377
 67054|  %388 = mul i64 %386, %386                                                                                             ;L9<2158<1377
 67055|  %389 = add i64 %388, %387                                                                                             ;L9<2158<1377
 67056|     ;; dist_sq = i64 %389
 67057|     ;; max_tick = i64 %97
 67058|  %390 = add i64 %267, %355                                                                                             ;L1376
 67059|  %391 = add i64 %390, %372                                                                                             ;L1383
 67060|     ;; max_dist = i64 %391
 67061|  %392 = mul i64 %391, %391                                                                                             ;L1385
 67062|  %393 = icmp ugt i64 %389, %392                                                                                        ;L1385
 67063|  br i1 %393, label %443, label %394                                                                                    ;L1385
 67064| 
 67065| 394: ; preds = %371
 67066|     ;; self = ptr %357
 67067|  %395 = gep %357, i64 104                                                                                              ;L1400<1390
 67068|  %396 = load i64, ptr %395, , !!8                                                                                      ;L1400<1390
 67069|  %397 = icmp eq i64 %396, 3                                                                                            ;L1390
 67070|  br i1 %397, label %398, label %401                                                                                    ;L1390
 67071| 
 67072| 398: ; preds = %394
 67075|  %399 = gep %357, i64 1472                                                                                             ;L1391
 67076|  %400 = load i64, ptr %399, , !!8                                                                                      ;L1391
 67077|  invoke void @ai::small_action4castNtB2_17SmallActionAttack3new(ptr sret([24 x i8]) %61, ptr %3, i64 %400)
 67078|  to label %405 unwind label %178                                                                                       ;L1391
 67079| 
 67080| 401: ; preds = %394
 67081|  %402 = gep %357, i64 1648                                                                                             ;L1396
 67082|  %403 = load i64, ptr %402, , !!8                                                                                      ;L1396
 67083|  %404 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %127, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %357)
 67084|  to label %419 unwind label %178                                                                                       ;L1396
 67085| 
 67086| 405: ; preds = %398
 67087|  call void @llvm.memcpy.p0.p0.i64(ptr %62, ptr %61, i64 24, i1 false)                                                  ;L1391
 67088|  store i8 15, ptr %269,                                                                                                ;L1391
 67091|     ;; self = ptr %65
 67092|     ;; self = ptr %65
 67093|     ;; value = ptr %62
 67094|     ;; src = ptr %62
 67095|     ;; additional = i64 1
 67096|     ;; needed_extra_cap = i64 1
 67097|     ;; needed_extra_cap = i64 1
 67098|     ;; strategy = i8 1
 67099|  %406 = load i64, ptr %126, , !!73653, !!8                                                                             ;L1428<1391
 67100|     ;; self = ptr %65
 67101|  %407 = load i64, ptr %125, , !!73653, !!8                                                                             ;L149<1428<1391
 67102|  %408 = icmp eq i64 %406, %407                                                                                         ;L1428<1391
 67103|  br i1 %408, label %409, label %414                                                                                    ;L1428<1391
 67104| 
 67105| 409: ; preds = %405
 67106|     ;; self = ptr %65
 67107|     ;; self = ptr %65
 67108|     ;; self = ptr %65
 67109|     ;; used_cap = i64 %406
 67110|     ;; used_cap = i64 %406
 67111|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %406, i64 1, i1 zeroext true)
 67112|  to label %410 unwind label %412, !!73653                                                                              ;L619<430<738<1429<1391
 67113| 
 67114| 410: ; preds = %409
 67115|  %411 = load i64, ptr %126, , !!73653                                                                                  ;L1432<1391
 67116|  br label %414                                                                                                         ;L619<430<738<1429<1391
 67117| 
 67118| 412: ; preds = %409
 67119|  %413 = cleanuppad within none []
 67120|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %62) #30 [ "funclet"(token %413) ], !!73638 ;L1436<1391
 67121|  cleanupret from %413 unwind label %178
 67122| 
 67123| 414: ; preds = %410, %405
 67124|  %415 = phi i64 [ %411, %410 ], [ %406, %405 ]                                                                         ;L1432<1391
 67125|     ;; self = ptr %65
 67126|  %416 = load ptr, ptr %65, , !!73653, !!8, !!8                                                                         ;L138<1432<1391
 67127|     ;; self = ptr %416
 67128|     ;; count = i64 %415
 67129|  %417 = gepS %416, i64 %415                                                                                            ;L961<1432<1391
 67130|     ;; end = ptr %417
 67131|     ;; dst = ptr %417
 67132|  call void @llvm.memcpy.p0.p0.i64(ptr %417, ptr %62, i64 184, i1 false), !!73638                                       ;L1933<1433<1391
 67133|  %418 = add i64 %415, 1                                                                                                ;L1434<1391
 67134|  store i64 %418, ptr %126, , !!73653                                                                                   ;L1434<1391
 67136|  br label %443                                                                                                         ;L1
 67137| 
 67138| 419: ; preds = %401
 67139|  %420 = icmp eq i64 %404, 0                                                                                            ;L1396
 67140|  br i1 %420, label %424, label %421                                                                                    ;L1396
 67141| 
 67142| 421: ; preds = %419
 67143|  %422 = udiv i64 %403, %404                                                                                            ;L1396
 67144|     ;; remain_attack = i64 %422
 67145|  %423 = icmp ugt i64 %422, 2                                                                                           ;L1397
 67146|  br i1 %423, label %443, label %425                                                                                    ;L1397
 67147| 
 67148| 424: ; preds = %419
 67149|  invoke void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.270) #31
 67150|  to label %118 unwind label %178                                                                                       ;L1396
 67151| 
 67152| 425: ; preds = %421
 67155|  %426 = load ptr, ptr %350, , !!8, !!8                                                                                 ;L1401
 67156|  %427 = gep %426, i64 1472                                                                                             ;L1401
 67157|  %428 = load i64, ptr %427, , !!8                                                                                      ;L1401
 67158|  invoke void @ai::small_action4castNtB2_17SmallActionAttack3new(ptr sret([24 x i8]) %59, ptr %3, i64 %428)
 67159|  to label %429 unwind label %178                                                                                       ;L1401
 67160| 
 67161| 429: ; preds = %425
 67162|  call void @llvm.memcpy.p0.p0.i64(ptr %60, ptr %59, i64 24, i1 false)                                                  ;L1401
 67163|  store i8 15, ptr %268,                                                                                                ;L1401
 67166|     ;; self = ptr %65
 67167|     ;; self = ptr %65
 67168|     ;; value = ptr %60
 67169|     ;; src = ptr %60
 67170|     ;; additional = i64 1
 67171|     ;; needed_extra_cap = i64 1
 67172|     ;; needed_extra_cap = i64 1
 67173|     ;; strategy = i8 1
 67174|  %430 = load i64, ptr %126, , !!73692, !!8                                                                             ;L1428<1401
 67175|     ;; self = ptr %65
 67176|  %431 = load i64, ptr %125, , !!73692, !!8                                                                             ;L149<1428<1401
 67177|  %432 = icmp eq i64 %430, %431                                                                                         ;L1428<1401
 67178|  br i1 %432, label %433, label %438                                                                                    ;L1428<1401
 67179| 
 67180| 433: ; preds = %429
 67181|     ;; self = ptr %65
 67182|     ;; self = ptr %65
 67183|     ;; self = ptr %65
 67184|     ;; used_cap = i64 %430
 67185|     ;; used_cap = i64 %430
 67186|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %430, i64 1, i1 zeroext true)
 67187|  to label %434 unwind label %436, !!73692                                                                              ;L619<430<738<1429<1401
 67188| 
 67189| 434: ; preds = %433
 67190|  %435 = load i64, ptr %126, , !!73692                                                                                  ;L1432<1401
 67191|  br label %438                                                                                                         ;L619<430<738<1429<1401
 67192| 
 67193| 436: ; preds = %433
 67194|  %437 = cleanuppad within none []
 67195|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %60) #30 [ "funclet"(token %437) ], !!73677 ;L1436<1401
 67196|  cleanupret from %437 unwind label %178
 67197| 
 67198| 438: ; preds = %434, %429
 67199|  %439 = phi i64 [ %435, %434 ], [ %430, %429 ]                                                                         ;L1432<1401
 67200|     ;; self = ptr %65
 67201|  %440 = load ptr, ptr %65, , !!73692, !!8, !!8                                                                         ;L138<1432<1401
 67202|     ;; self = ptr %440
 67203|     ;; count = i64 %439
 67204|  %441 = gepS %440, i64 %439                                                                                            ;L961<1432<1401
 67205|     ;; end = ptr %441
 67206|     ;; dst = ptr %441
 67207|  call void @llvm.memcpy.p0.p0.i64(ptr %441, ptr %60, i64 184, i1 false), !!73677                                       ;L1933<1433<1401
 67208|  %442 = add i64 %439, 1                                                                                                ;L1434<1401
 67209|  store i64 %442, ptr %126, , !!73692                                                                                   ;L1434<1401
 67211|  br label %443                                                                                                         ;L1375
 67212| 
 67213| 443: ; preds = %438, %421, %414, %371
 67214|  br label %349                                                                                                         ;L1714<180<1375
 67215| 
 67216| 444: ; preds = %349, %242
 67217|  %445 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_attack(ptr %86)
 67218|  to label %447 unwind label %178                                                                                       ;L1409
 67219| 
 67220| 446: ; preds = %491, %447, %236
 67221|  br i1 %133, label %592, label %590                                                                                    ;L1432
 67222| 
 67223| 447: ; preds = %444
 67224|  br i1 %445, label %448, label %446                                                                                    ;L1409
 67225| 
 67226| 448: ; preds = %447
 67227|     ;; max_tick = i64 %97
 67228|  invoke void @gc::simulationNtB5_21AbstractGameWithCache12iter_minions(ptr sret([56 x i8]) %58, ptr %82, i64 %104)
 67229|  to label %449 unwind label %178                                                                                       ;L1411
 67230| 
 67231| 449: ; preds = %448
 67233|  call void @llvm.memcpy.p0.p0.i64(ptr %57, ptr %58, i64 56, i1 false)                                                  ;L1411
 67234|  %450 = gep %57, i64 8
 67235|  %451 = gep %57, i64 24
 67236|  %452 = gep %57, i64 40
 67237|  %453 = gep %82, i64 8
 67238|  %454 = gep %86, i64 8
 67239|  %455 = mul i64 %239, %97
 67240|  %456 = add i64 %455, %177
 67241|  %457 = gep %86, i64 1632
 67242|  %458 = gep %86, i64 1640
 67243|  %459 = gep %56, i64 177
 67244|  br label %460                                                                                                         ;L1411
 67245| 
 67246| 460: ; preds = %589, %449
 67247|     ;; self = ptr %57
 67248|     ;; opt = ptr %57
 67249|     ;; self = ptr %57
 67251|  %461 = load i64, ptr %57, , !!8                                                                                       ;L764<332<82<1411
 67252|  %462 = trunc nuw i64 %461 to i1                                                                                       ;L764<332<82<1411
 67253|  br i1 %462, label %463, label %479                                                                                    ;L764<332<82<1411
 67254| 
 67255| 463: ; preds = %460
 67258|     ;; self = ptr %450
 67259|     ;; opt = ptr %450
 67260|     ;; self = ptr %450
 67262|  %464 = load ptr, ptr %450, , !!8                                                                                      ;L764<332<82<250<332<82<1411
 67263|  %465 = icmp eq ptr %464, null                                                                                         ;L764<332<82<250<332<82<1411
 67264|  br i1 %465, label %471, label %466                                                                                    ;L764<332<82<250<332<82<1411
 67265| 
 67266| 466: ; preds = %463
 67269|  %467 = invoke ptr @gc::simulation6entity6EntityEENtNtNtB8_6traits8iterator8Iterator4nextCshdEBA0ozCnw_7game_ai(ptr %450)
 67270|  to label %468 unwind label %178                                                                                       ;L250<332<82<250<332<82<1411
 67271| 
 67272| 468: ; preds = %466
 67273|     ;; x = ptr %467
 67276|  %469 = icmp eq ptr %467, null                                                                                         ;L633<682<333<82<250<332<82<1411
 67277|  br i1 %469, label %470, label %486                                                                                    ;L333<82<250<332<82<1411
 67278| 
 67279| 470: ; preds = %468
 67280|  store ptr null, ptr %450,                                                                                             ;L334<82<250<332<82<1411
 67281|  br label %471                                                                                                         ;L333<82<250<332<82<1411
 67282| 
 67283| 471: ; preds = %470, %463
 67284|     ;; self = ptr null
 67285|     ;; f = ptr %451
 67287|     ;; self = ptr %451
 67288|  %472 = load ptr, ptr %451, , !!73883, !!8                                                                             ;L764<82<1653<82<250<332<82<1411
 67289|  %473 = icmp eq ptr %472, null                                                                                         ;L764<82<1653<82<250<332<82<1411
 67290|  br i1 %473, label %478, label %474                                                                                    ;L764<82<1653<82<250<332<82<1411
 67291| 
 67292| 474: ; preds = %471
 67293|  %475 = invoke ptr @gc::simulation6entity6EntityEENtNtNtB8_6traits8iterator8Iterator4nextCshdEBA0ozCnw_7game_ai(ptr %451)
 67294|  to label %476 unwind label %178                                                                                       ;L82<1653<82<250<332<82<1411
 67295| 
 67296| 476: ; preds = %474
 67297|     ;; x = ptr %475
 67300|  %477 = icmp eq ptr %475, null                                                                                         ;L633<682<333<82<1411
 67301|  br i1 %477, label %478, label %486                                                                                    ;L333<82<1411
 67302| 
 67303| 478: ; preds = %476, %471
 67304|  store i64 0, ptr %57,                                                                                                 ;L334<82<1411
 67305|  br label %479                                                                                                         ;L333<82<1411
 67306| 
 67307| 479: ; preds = %478, %460
 67308|     ;; self = ptr null
 67309|     ;; f = ptr %452
 67311|     ;; self = ptr %452
 67312|  %480 = load ptr, ptr %452, , !!73940, !!8                                                                             ;L764<82<1653<82<1411
 67313|  %481 = icmp eq ptr %480, null                                                                                         ;L764<82<1653<82<1411
 67314|  br i1 %481, label %491, label %482                                                                                    ;L764<82<1653<82<1411
 67315| 
 67316| 482: ; preds = %479
 67317|  %483 = invoke ptr @gc::simulation6entity6EntityEENtNtNtB8_6traits8iterator8Iterator4nextCshdEBA0ozCnw_7game_ai(ptr %452)
 67318|  to label %484 unwind label %178                                                                                       ;L82<1653<82<1411
 67319| 
 67320| 484: ; preds = %482
 67321|  %485 = icmp eq ptr %483, null                                                                                         ;L1411
 67322|  br i1 %485, label %491, label %486                                                                                    ;L1411
 67323| 
 67324| 486: ; preds = %484, %476, %468
 67325|  %487 = phi ptr [ %483, %484 ], [ %475, %476 ], [ %467, %468 ]
 67326|     ;; m = ptr %487
 67327|     ;; self = ptr %487
 67328|     ;; self = ptr %487
 67329|     ;; self = ptr %487
 67330|  %488 = gep %487, i64 104                                                                                              ;L1412
 67331|  %489 = load i64, ptr %488, , !!8                                                                                      ;L1412
 67332|  %490 = icmp eq i64 %489, 1                                                                                            ;L1412
 67333|  br i1 %490, label %492, label %589                                                                                    ;L1412
 67334| 
 67335| 491: ; preds = %484, %479
 67337|  br label %446                                                                                                         ;L1409
 67338| 
 67339| 492: ; preds = %486
 67340|     ;; info = ptr %487
 67341|  %493 = gep %487, i64 136                                                                                              ;L1413
 67342|  %494 = load i64, ptr %493, , !!8                                                                                      ;L1413
 67343|     ;; self[0..+8] = i64 %494
 67347|  %495 = trunc nuw i64 %494 to i1                                                                                       ;L1542<1413
 67348|  br i1 %495, label %496, label %589                                                                                    ;L1542<1413
 67349| 
 67350| 496: ; preds = %492
 67351|  %497 = load ptr, ptr %453, , !!8, !!8                                                                                 ;L1413
 67352|     ;; f[8..+8] = ptr %497
 67353|  %498 = load ptr, ptr %82, , !!8, !!8                                                                                  ;L1413
 67354|     ;; f[0..+8] = ptr %498
 67355|  %499 = gep %487, i64 144                                                                                              ;L1413
 67356|  %500 = load i64, ptr %499,                                                                                            ;L1413
 67357|     ;; self[8..+8] = i64 %500
 67358|     ;; x = i64 %500
 67359|     ;; e = i64 %500
 67360|  %501 = gep %497, i64 496                                                                                              ;L1413<1543<1413
 67361|  %502 = load ptr, ptr %501, , !!8                                                                                      ;L1413<1543<1413
 67362|  %503 = invoke ptr %502(ptr %498, i64 %500)
 67363|  to label %504 unwind label %178                                                                                       ;L1413<1543<1413
 67364| 
 67365| 504: ; preds = %496
 67366|     ;; self = ptr %503
 67367|     ;; f = ptr %2
 67368|  %505 = icmp eq ptr %503, null                                                                                         ;L659<1414
 67369|  br i1 %505, label %589, label %506                                                                                    ;L659<1414
 67370| 
 67371| 506: ; preds = %504
 67372|     ;; x = ptr %503
 67373|     ;; e = ptr %503
 67374|     ;; self = ptr %503
 67376|  %507 = load i64, ptr %503, , !!8                                                                                      ;L1127<1414<661<1414
 67377|     ;; __self_discr = i64 %507
 67378|  %508 = icmp eq i64 %507, 0                                                                                            ;L1127<1414<661<1414
 67379|  br i1 %508, label %509, label %589                                                                                    ;L1127<1414<661<1414
 67380| 
 67381| 509: ; preds = %506
 67382|  %510 = gep %503, i64 8                                                                                                ;L1127<1414<661<1414
 67383|     ;; __self_0 = ptr %503
 67384|     ;; self = ptr %503
 67389|  %511 = load i64, ptr %510, , !!8                                                                                      ;L1878<2123<1127<1414<661<1414
 67390|  %512 = icmp eq i64 %511, %75                                                                                          ;L1878<2123<1127<1414<661<1414
 67391|  br i1 %512, label %513, label %589                                                                                    ;L1414<661<1414
 67392| 
 67393| 513: ; preds = %509
 67394|     ;; self = ptr %503
 67395|  %514 = gep %503, i64 104                                                                                              ;L1400<1414<661<1414
 67396|  %515 = load i64, ptr %514, , !!8                                                                                      ;L1400<1414<661<1414
 67397|  switch i64 %515, label %589 [
 67398|  i64 3, label %516
 67399|  i64 2, label %519
 67400|  ]                                                                                                                     ;L1414<661<1414
 67401| 
 67402| 516: ; preds = %519, %513
 67403|     ;; self = ptr %86
 67404|  %517 = load i64, ptr %86, , !!8                                                                                       ;L1136<1482<1418
 67405|  %518 = trunc nuw i64 %517 to i1                                                                                       ;L1136<1482<1418
 67406|  br i1 %518, label %533, label %524                                                                                    ;L1136<1482<1418
 67407| 
 67408| 519: ; preds = %513
 67409|     ;; self = ptr %503
 67410|  %520 = gep %503, i64 296                                                                                              ;L91<1313<1414<661<1414
 67411|  %521 = load i8, ptr %520, , !!8                                                                                       ;L91<1313<1414<661<1414
 67412|  %522 = add nsw i8 %521, -3                                                                                            ;L91<1313<1414<661<1414
 67413|  %523 = icmp ult i8 %522, 2                                                                                            ;L91<1313<1414<661<1414
 67414|  br i1 %523, label %516, label %589                                                                                    ;L91<1313<1414<661<1414
 67415| 
 67416| 524: ; preds = %516
 67417|     ;; team = ptr %86
 67418|  %525 = load i64, ptr %454, , !!8                                                                                      ;L1137<1482<1418
 67419|     ;; team = i64 %525
 67420|  %526 = icmp ult i64 %525, 2                                                                                           ;L1483<1418
 67421|  br i1 %526, label %527, label %532                                                                                    ;L1483<1418
 67422| 
 67423| 527: ; preds = %524
 67425|  %528 = gep %487, i64 56                                                                                               ;L122<1483<1418
 67426|  %529 = gepS %528, i64 %525                                                                                            ;L122<1483<1418
 67427|  %530 = load i64, ptr %529, , !!8                                                                                      ;L122<1483<1418
 67428|  %531 = icmp eq i64 %530, 0                                                                                            ;L122<1483<1418
 67429|  br i1 %531, label %533, label %589                                                                                    ;L1418
 67430| 
 67431| 532: ; preds = %524
 67432|  invoke void @core::panicking18panic_bounds_check(i64 %525, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 67433|  to label %118 unwind label %178                                                                                       ;L1483<1418
 67434| 
 67435| 533: ; preds = %527, %516
 67436|  %534 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %127, ptr %86, ptr %487)
 67437|  to label %535 unwind label %178                                                                                       ;L1422
 67438| 
 67439| 535: ; preds = %533
 67440|  %536 = gep %487, i64 1136                                                                                             ;L1511<1422
 67441|  %537 = load i32, ptr %536, , !!8                                                                                      ;L1511<1422
 67442|     ;; mult = i32 %537
 67443|  %538 = icmp eq i32 %537, 0                                                                                            ;L1512<1422
 67444|  br i1 %538, label %539, label %542                                                                                    ;L1512<1422
 67445| 
 67446| 539: ; preds = %535
 67447|  %540 = gep %487, i64 1664                                                                                             ;L1513<1422
 67448|  %541 = load i64, ptr %540, , !!8                                                                                      ;L1513<1422
 67449|  br label %549                                                                                                         ;L1512<1422
 67450| 
 67451| 542: ; preds = %535
 67452|  %543 = sext i32 %537 to i64                                                                                           ;L1511<1422
 67453|     ;; mult = i64 %543
 67454|  %544 = gep %487, i64 1664                                                                                             ;L1515<1422
 67455|  %545 = load i64, ptr %544, , !!8                                                                                      ;L1515<1422
 67456|  %546 = add nsw i64 %543, 100                                                                                          ;L1515<1422
 67457|  %547 = mul i64 %545, %546                                                                                             ;L1515<1422
 67458|  %548 = udiv i64 %547, 100                                                                                             ;L1515<1422
 67459|  br label %549                                                                                                         ;L1512<1422
 67460| 
 67461| 549: ; preds = %542, %539
 67462|  %550 = phi i64 [ %541, %539 ], [ %548, %542 ]                                                                         ;L0<1422
 67464|  %551 = add i64 %456, %534                                                                                             ;L1422
 67465|  %552 = add i64 %551, %550                                                                                             ;L1423
 67466|     ;; max_dist = i64 %552
 67467|  %553 = gep %487, i64 1632                                                                                             ;L2158<1424
 67468|  %554 = load i64, ptr %553, , !!8                                                                                      ;L2158<1424
 67469|     ;; x1 = i64 %554
 67470|     ;; self = i64 %554
 67471|  %555 = gep %487, i64 1640                                                                                             ;L2158<1424
 67472|  %556 = load i64, ptr %555, , !!8                                                                                      ;L2158<1424
 67473|     ;; y1 = i64 %556
 67474|     ;; self = i64 %556
 67475|  %557 = load i64, ptr %457, , !!8                                                                                      ;L2158<1424
 67476|     ;; x2 = i64 %557
 67477|     ;; other = i64 %557
 67478|  %558 = load i64, ptr %458, , !!8                                                                                      ;L2158<1424
 67479|     ;; y2 = i64 %558
 67480|     ;; other = i64 %558
 67481|  %559 = icmp ult i64 %554, %557                                                                                        ;L3147<7<2158<1424
 67482|  %560 = sub nuw i64 %557, %554                                                                                         ;L3147<7<2158<1424
 67483|  %561 = sub nuw i64 %554, %557                                                                                         ;L3147<7<2158<1424
 67484|  %562 = select i1 %559, i64 %560, i64 %561                                                                             ;L3147<7<2158<1424
 67485|     ;; dx = i64 %562
 67486|  %563 = icmp ult i64 %556, %558                                                                                        ;L3147<8<2158<1424
 67487|  %564 = sub nuw i64 %558, %556                                                                                         ;L3147<8<2158<1424
 67488|  %565 = sub nuw i64 %556, %558                                                                                         ;L3147<8<2158<1424
 67489|  %566 = select i1 %563, i64 %564, i64 %565                                                                             ;L3147<8<2158<1424
 67490|     ;; dy = i64 %566
 67491|  %567 = mul i64 %562, %562                                                                                             ;L9<2158<1424
 67492|  %568 = mul i64 %566, %566                                                                                             ;L9<2158<1424
 67493|  %569 = add i64 %568, %567                                                                                             ;L9<2158<1424
 67494|  %570 = mul i64 %552, %552                                                                                             ;L1424
 67495|  %571 = icmp ugt i64 %569, %570                                                                                        ;L1424
 67496|  br i1 %571, label %589, label %572                                                                                    ;L1424
 67497| 
 67498| 572: ; preds = %549
 67501|  %573 = gep %487, i64 1472                                                                                             ;L1428
 67502|  %574 = load i64, ptr %573, , !!8                                                                                      ;L1428
 67503|  invoke void @ai::small_action4castNtB2_17SmallActionAttack3new(ptr sret([24 x i8]) %55, ptr %3, i64 %574)
 67504|  to label %575 unwind label %178                                                                                       ;L1428
 67505| 
 67506| 575: ; preds = %572
 67507|  call void @llvm.memcpy.p0.p0.i64(ptr %56, ptr %55, i64 24, i1 false)                                                  ;L1428
 67508|  store i8 15, ptr %459,                                                                                                ;L1428
 67511|     ;; self = ptr %65
 67512|     ;; self = ptr %65
 67513|     ;; value = ptr %56
 67514|     ;; src = ptr %56
 67515|     ;; additional = i64 1
 67516|     ;; needed_extra_cap = i64 1
 67517|     ;; needed_extra_cap = i64 1
 67518|     ;; strategy = i8 1
 67519|  %576 = load i64, ptr %126, , !!74065, !!8                                                                             ;L1428<1428
 67520|     ;; self = ptr %65
 67521|  %577 = load i64, ptr %125, , !!74065, !!8                                                                             ;L149<1428<1428
 67522|  %578 = icmp eq i64 %576, %577                                                                                         ;L1428<1428
 67523|  br i1 %578, label %579, label %584                                                                                    ;L1428<1428
 67524| 
 67525| 579: ; preds = %575
 67526|     ;; self = ptr %65
 67527|     ;; self = ptr %65
 67528|     ;; self = ptr %65
 67529|     ;; used_cap = i64 %576
 67530|     ;; used_cap = i64 %576
 67531|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %576, i64 1, i1 zeroext true)
 67532|  to label %580 unwind label %582, !!74065                                                                              ;L619<430<738<1429<1428
 67533| 
 67534| 580: ; preds = %579
 67535|  %581 = load i64, ptr %126, , !!74065                                                                                  ;L1432<1428
 67536|  br label %584                                                                                                         ;L619<430<738<1429<1428
 67537| 
 67538| 582: ; preds = %579
 67539|  %583 = cleanuppad within none []
 67540|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %56) #30 [ "funclet"(token %583) ], !!74050 ;L1436<1428
 67541|  cleanupret from %583 unwind label %178
 67542| 
 67543| 584: ; preds = %580, %575
 67544|  %585 = phi i64 [ %581, %580 ], [ %576, %575 ]                                                                         ;L1432<1428
 67545|     ;; self = ptr %65
 67546|  %586 = load ptr, ptr %65, , !!74065, !!8, !!8                                                                         ;L138<1432<1428
 67547|     ;; self = ptr %586
 67548|     ;; count = i64 %585
 67549|  %587 = gepS %586, i64 %585                                                                                            ;L961<1432<1428
 67550|     ;; end = ptr %587
 67551|     ;; dst = ptr %587
 67552|  call void @llvm.memcpy.p0.p0.i64(ptr %587, ptr %56, i64 184, i1 false), !!74050                                       ;L1933<1433<1428
 67553|  %588 = add i64 %585, 1                                                                                                ;L1434<1428
 67554|  store i64 %588, ptr %126, , !!74065                                                                                   ;L1434<1428
 67556|  br label %589                                                                                                         ;L1411
 67557| 
 67558| 589: ; preds = %584, %549, %527, %519, %513, %509, %506, %504, %492, %486
 67559|  br label %460                                                                                                         ;L764<332<82<1411
 67560| 
 67561| 590: ; preds = %446
 67562|  %591 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity9can_skill(ptr %86)
 67563|  to label %593 unwind label %178                                                                                       ;L1432
 67564| 
 67565| 592: ; preds = %1445, %1431, %1430, %1166, %1157, %593, %446
 67566|  br i1 %142, label %1452, label %1450                                                                                  ;L1592
 67567| 
 67568| 593: ; preds = %590
 67569|  br i1 %591, label %594, label %592                                                                                    ;L1432
 67570| 
 67571| 594: ; preds = %593
 67572|     ;; self = ptr %70
 67573|     ;; self = ptr %70
 67574|     ;; self = ptr %70
 67575|  %595 = load ptr, ptr %70, , !!8, !!8                                                                                  ;L138<2073<2136<1433
 67576|     ;; p = ptr %595
 67577|  %596 = gep %70, i64 24                                                                                                ;L2075<2136<1433
 67578|  %597 = load i64, ptr %596, , !!8                                                                                      ;L2075<2136<1433
 67579|     ;; len = i64 %597
 67580|     ;; count = i64 %597
 67581|     ;; self[0..+8] = ptr %595
 67582|     ;; slice[0..+8] = ptr %595
 67583|     ;; self[8..+8] = i64 %597
 67584|     ;; slice[8..+8] = i64 %597
 67585|     ;; ptr = ptr %595
 67586|     ;; self = ptr %595
 67587|  %598 = gepS }, ptr %595, i64 %597                                                                                     ;L961<100<1042<2136<1433
 67588|     ;; iter[0..+8] = ptr %595
 67589|     ;; iter[8..+8] = ptr %598
 67590|  %599 = gep %86, i64 8
 67591|  %600 = gep %86, i64 1264
 67592|  %601 = gep %86, i64 1408
 67593|  %602 = gep %86, i64 1416
 67594|  %603 = gep %82, i64 8
 67595|  %604 = gep %86, i64 1632
 67596|  %605 = gep %86, i64 1640
 67597|  %606 = gep %86, i64 1232
 67598|  %607 = gep %86, i64 1232
 67599|  %608 = gep %53, i64 177
 67600|  br label %609                                                                                                         ;L1433
 67601| 
 67602| 609: ; preds = %776, %594
 67603|  %610 = phi ptr [ %595, %594 ], [ %613, %776 ]                                                                         ;L1433
 67604|     ;; iter[0..+8] = ptr %610
 67605|     ;; self = ptr undef
 67606|     ;; ptr = ptr %610
 67607|     ;; self = ptr %610
 67608|     ;; end_or_len = ptr %598
 67611|  %611 = icmp eq ptr %610, %598                                                                                         ;L1714<180<1433
 67612|  br i1 %611, label %618, label %612                                                                                    ;L180<1433
 67613| 
 67614| 612: ; preds = %609
 67615|  %613 = gep %610, i64 32                                                                                               ;L656<185<1433
 67616|     ;; iter[0..+8] = ptr %613
 67617|     ;; a = ptr %610
 67618|     ;; e = ptr %610
 67619|  %614 = gep %610, i64 24                                                                                               ;L1434
 67620|  %615 = load ptr, ptr %614, , !!8, !!8                                                                                 ;L1434
 67621|     ;; self = ptr %615
 67622|     ;; self = ptr %615
 67623|     ;; self = ptr %615
 67624|     ;; self = ptr %86
 67625|  %616 = load i64, ptr %86, , !!8                                                                                       ;L1136<1482<1434
 67626|  %617 = trunc nuw i64 %616 to i1                                                                                       ;L1136<1482<1434
 67627|  br i1 %617, label %640, label %631                                                                                    ;L1136<1482<1434
 67628| 
 67629| 618: ; preds = %609
 67630|     ;; self = ptr %72
 67631|     ;; self = ptr %72
 67632|     ;; self = ptr %72
 67633|  %619 = load ptr, ptr %72, , !!8, !!8                                                                                  ;L138<2073<2136<1494
 67634|     ;; p = ptr %619
 67635|  %620 = gep %72, i64 24                                                                                                ;L2075<2136<1494
 67636|  %621 = load i64, ptr %620, , !!8                                                                                      ;L2075<2136<1494
 67637|     ;; len = i64 %621
 67638|     ;; count = i64 %621
 67639|     ;; self[0..+8] = ptr %619
 67640|     ;; slice[0..+8] = ptr %619
 67641|     ;; self[8..+8] = i64 %621
 67642|     ;; slice[8..+8] = i64 %621
 67643|     ;; ptr = ptr %619
 67644|     ;; self = ptr %619
 67645|  %622 = getelementptr ptr, ptr %619, i64 %621                                                                          ;L961<100<1042<2136<1494
 67646|     ;; iter[0..+8] = ptr %619
 67647|     ;; iter[8..+8] = ptr %622
 67648|  %623 = gep %51, i64 72
 67649|  %624 = gep %82, i64 240
 67650|  %625 = getelementptr { { ptr, ptr, i64 }, i64 }, ptr %624, i64 %104
 67651|  %626 = gep %625, i64 24
 67652|  %627 = gep %51, i64 136
 67653|  %628 = mul i64 %239, %97
 67654|  %629 = add i64 %628, %208
 67655|  %630 = gep %50, i64 177
 67656|  br label %777                                                                                                         ;L1494
 67657| 
 67658| 631: ; preds = %612
 67659|     ;; team = ptr %86
 67660|  %632 = load i64, ptr %599, , !!8                                                                                      ;L1137<1482<1434
 67661|     ;; team = i64 %632
 67662|  %633 = icmp ult i64 %632, 2                                                                                           ;L1483<1434
 67663|  br i1 %633, label %634, label %639                                                                                    ;L1483<1434
 67664| 
 67665| 634: ; preds = %631
 67667|  %635 = gep %615, i64 56                                                                                               ;L122<1483<1434
 67668|  %636 = gepS %635, i64 %632                                                                                            ;L122<1483<1434
 67669|  %637 = load i64, ptr %636, , !!8                                                                                      ;L122<1483<1434
 67670|  %638 = icmp eq i64 %637, 0                                                                                            ;L122<1483<1434
 67671|  br i1 %638, label %640, label %776                                                                                    ;L1434
 67672| 
 67673| 639: ; preds = %631
 67674|  invoke void @core::panicking18panic_bounds_check(i64 %632, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 67675|  to label %118 unwind label %178                                                                                       ;L1483<1434
 67676| 
 67677| 640: ; preds = %634, %612
 67678|  %641 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %600, ptr %86, ptr %615)
 67679|  to label %642 unwind label %178                                                                                       ;L1438
 67680| 
 67681| 642: ; preds = %640
 67682|  br i1 %641, label %643, label %776                                                                                    ;L1438
 67683| 
 67684| 643: ; preds = %642
 67685|  %644 = load ptr, ptr %601, , !!8, !!8                                                                                 ;L1442
 67686|  %645 = load ptr, ptr %602, , !!8, !!8                                                                                 ;L1442
 67687|  %646 = load ptr, ptr %82, , !!8, !!8                                                                                  ;L1442
 67688|  %647 = load ptr, ptr %603, , !!8, !!8                                                                                 ;L1442
 67689|  %648 = gep %645, i64 200                                                                                              ;L1442
 67690|  %649 = load ptr, ptr %648, , !!8                                                                                      ;L1442
 67691|  %650 = invoke zeroext i1 %649(ptr %644, ptr %646, ptr %647, ptr %86, ptr %615)
 67692|  to label %651 unwind label %178                                                                                       ;L1442
 67693| 
 67694| 651: ; preds = %643
 67695|  br i1 %650, label %652, label %776                                                                                    ;L1442
 67696| 
 67697| 652: ; preds = %651
 67698|  %653 = load i64, ptr %610, , !!8                                                                                      ;L1446
 67701|     ;; __self_discr = i64 %653
 67702|     ;; __arg1_discr = i64 0
 67703|  %654 = icmp eq i64 %653, 0                                                                                            ;L81<1446
 67704|  br i1 %654, label %655, label %662                                                                                    ;L1446
 67705| 
 67706| 655: ; preds = %652
 67707|  %656 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10block_move(ptr %615)
 67708|  to label %657 unwind label %178                                                                                       ;L1446
 67709| 
 67710| 657: ; preds = %655
 67711|  br i1 %656, label %662, label %658                                                                                    ;L1446
 67712| 
 67713| 658: ; preds = %657
 67714|  %659 = gep %615, i64 1600                                                                                             ;L1447
 67715|  %660 = load i64, ptr %659, , !!8                                                                                      ;L1447
 67716|     ;; rhs = i64 %660
 67717|  %661 = call i64 @llvm.usub.sat.i64(i64 %239, i64 %660)                                                                ;L2472<1447
 67718|     ;; move_speed = i64 %661
 67719|  br label %662                                                                                                         ;L1446
 67720| 
 67721| 662: ; preds = %658, %657, %652
 67722|  %663 = phi i64 [ %661, %658 ], [ %239, %657 ], [ %239, %652 ]                                                         ;L0
 67723|     ;; move_speed = i64 %663
 67724|  %664 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %134, ptr %86, ptr %615)
 67725|  to label %665 unwind label %178                                                                                       ;L1452
 67726| 
 67727| 665: ; preds = %662
 67728|  %666 = add i64 %664, %208                                                                                             ;L1452
 67729|  %667 = gep %615, i64 1136                                                                                             ;L1511<1452
 67730|  %668 = load i32, ptr %667, , !!8                                                                                      ;L1511<1452
 67731|     ;; mult = i32 %668
 67732|  %669 = icmp eq i32 %668, 0                                                                                            ;L1512<1452
 67733|  br i1 %669, label %670, label %673                                                                                    ;L1512<1452
 67734| 
 67735| 670: ; preds = %665
 67736|  %671 = gep %615, i64 1664                                                                                             ;L1513<1452
 67737|  %672 = load i64, ptr %671, , !!8                                                                                      ;L1513<1452
 67738|  br label %680                                                                                                         ;L1512<1452
 67739| 
 67740| 673: ; preds = %665
 67741|  %674 = sext i32 %668 to i64                                                                                           ;L1511<1452
 67742|     ;; mult = i64 %674
 67743|  %675 = gep %615, i64 1664                                                                                             ;L1515<1452
 67744|  %676 = load i64, ptr %675, , !!8                                                                                      ;L1515<1452
 67745|  %677 = add nsw i64 %674, 100                                                                                          ;L1515<1452
 67746|  %678 = mul i64 %676, %677                                                                                             ;L1515<1452
 67747|  %679 = udiv i64 %678, 100                                                                                             ;L1515<1452
 67748|  br label %680                                                                                                         ;L1512<1452
 67749| 
 67750| 680: ; preds = %673, %670
 67751|  %681 = phi i64 [ %672, %670 ], [ %679, %673 ]                                                                         ;L0<1452
 67753|  %682 = gep %615, i64 1632                                                                                             ;L2158<1453
 67754|  %683 = load i64, ptr %682, , !!8                                                                                      ;L2158<1453
 67755|     ;; x1 = i64 %683
 67756|     ;; self = i64 %683
 67757|  %684 = gep %615, i64 1640                                                                                             ;L2158<1453
 67758|  %685 = load i64, ptr %684, , !!8                                                                                      ;L2158<1453
 67759|     ;; y1 = i64 %685
 67760|     ;; self = i64 %685
 67761|  %686 = load i64, ptr %604, , !!8                                                                                      ;L2158<1453
 67762|     ;; x2 = i64 %686
 67763|     ;; other = i64 %686
 67764|  %687 = load i64, ptr %605, , !!8                                                                                      ;L2158<1453
 67765|     ;; y2 = i64 %687
 67766|     ;; other = i64 %687
 67767|  %688 = icmp ult i64 %683, %686                                                                                        ;L3147<7<2158<1453
 67768|  %689 = sub nuw i64 %686, %683                                                                                         ;L3147<7<2158<1453
 67769|  %690 = sub nuw i64 %683, %686                                                                                         ;L3147<7<2158<1453
 67770|  %691 = select i1 %688, i64 %689, i64 %690                                                                             ;L3147<7<2158<1453
 67771|     ;; dx = i64 %691
 67772|  %692 = icmp ult i64 %685, %687                                                                                        ;L3147<8<2158<1453
 67773|  %693 = sub nuw i64 %687, %685                                                                                         ;L3147<8<2158<1453
 67774|  %694 = sub nuw i64 %685, %687                                                                                         ;L3147<8<2158<1453
 67775|  %695 = select i1 %692, i64 %693, i64 %694                                                                             ;L3147<8<2158<1453
 67776|     ;; dy = i64 %695
 67777|  %696 = mul i64 %691, %691                                                                                             ;L9<2158<1453
 67778|  %697 = mul i64 %695, %695                                                                                             ;L9<2158<1453
 67779|  %698 = add i64 %697, %696                                                                                             ;L9<2158<1453
 67780|     ;; dist_sq = i64 %698
 67781|     ;; max_tick = i64 %97
 67782|  %699 = mul i64 %663, %97                                                                                              ;L1462
 67783|  %700 = add i64 %666, %699                                                                                             ;L1452
 67784|  %701 = add i64 %700, %681                                                                                             ;L1462
 67785|     ;; max_dist = i64 %701
 67786|  %702 = mul i64 %701, %701                                                                                             ;L1463
 67787|  %703 = icmp ugt i64 %698, %702                                                                                        ;L1463
 67788|  br i1 %703, label %776, label %704                                                                                    ;L1463
 67789| 
 67790| 704: ; preds = %680
 67791|  switch i64 %4, label %736 [
 67792|  i64 3, label %705
 67793|  i64 4, label %705
 67794|  i64 0, label %732
 67795|  i64 5, label %732
 67796|  ]                                                                                                                     ;L1467
 67797| 
 67798| 705: ; preds = %704, %704
 67801|  %706 = load ptr, ptr %134, , !!8, !!8                                                                                 ;L441<2127<2445<1469
 67802|  %707 = load ptr, ptr %606, , !!8, !!8                                                                                 ;L441<2127<2445<1469
 67803|  %708 = gep %707, i64 16                                                                                               ;L2445<1469
 67804|  %709 = load i64, ptr %708,                                                                                            ;L2445<1469
 67805|  %710 = add nsw i64 %709, -1                                                                                           ;L2445<1469
 67806|  %711 = and i64 %710, -16                                                                                              ;L2445<1469
 67807|  %712 = gep %706, i64 %711                                                                                             ;L2445<1469
 67808|  %713 = gep %712, i64 16                                                                                               ;L2445<1469
 67809|  %714 = gep %707, i64 104                                                                                              ;L1469
 67810|  %715 = load ptr, ptr %714, , !!8                                                                                      ;L1469
 67811|  %716 = invoke zeroext i1 %715(ptr %713)
 67812|  to label %717 unwind label %178                                                                                       ;L1469
 67813| 
 67814| 717: ; preds = %705
 67815|  br i1 %716, label %776, label %718                                                                                    ;L1469
 67816| 
 67817| 718: ; preds = %717
 67821|  %719 = load ptr, ptr %134, , !!8, !!8                                                                                 ;L441<2127<2445<1469
 67822|  %720 = load ptr, ptr %606, , !!8, !!8                                                                                 ;L441<2127<2445<1469
 67823|  %721 = gep %720, i64 16                                                                                               ;L2445<1469
 67824|  %722 = load i64, ptr %721,                                                                                            ;L2445<1469
 67825|  %723 = add nsw i64 %722, -1                                                                                           ;L2445<1469
 67826|  %724 = and i64 %723, -16                                                                                              ;L2445<1469
 67827|  %725 = gep %719, i64 %724                                                                                             ;L2445<1469
 67828|  %726 = gep %725, i64 16                                                                                               ;L2445<1469
 67829|  %727 = gep %720, i64 88                                                                                               ;L1469
 67830|  %728 = load ptr, ptr %727, , !!8                                                                                      ;L1469
 67831|  invoke void %728(ptr sret([24 x i8]) %54, ptr %726)
 67832|  to label %729 unwind label %178                                                                                       ;L1469
 67833| 
 67834| 729: ; preds = %718
 67835|     ;; self = ptr %54
 67836|  %730 = load i64, ptr %54, , !!8                                                                                       ;L633<1469
 67837|  %731 = icmp eq i64 %730, 0                                                                                            ;L1469
 67839|  br i1 %731, label %736, label %776                                                                                    ;L1469
 67840| 
 67841| 732: ; preds = %757, %749, %748, %704, %704
 67842|  %733 = gep %645, i64 192                                                                                              ;L1483
 67843|  %734 = load ptr, ptr %733, , !!8                                                                                      ;L1483
 67844|  %735 = invoke zeroext i1 %734(ptr %644, ptr %646, ptr %647, ptr %86)
 67845|  to label %758 unwind label %178                                                                                       ;L1483
 67846| 
 67847| 736: ; preds = %729, %704
 67850|  %737 = load ptr, ptr %134, , !!8, !!8                                                                                 ;L441<2127<2445<1476
 67851|  %738 = load ptr, ptr %607, , !!8, !!8                                                                                 ;L441<2127<2445<1476
 67852|  %739 = gep %738, i64 16                                                                                               ;L2445<1476
 67853|  %740 = load i64, ptr %739,                                                                                            ;L2445<1476
 67854|  %741 = add nsw i64 %740, -1                                                                                           ;L2445<1476
 67855|  %742 = and i64 %741, -16                                                                                              ;L2445<1476
 67856|  %743 = gep %737, i64 %742                                                                                             ;L2445<1476
 67857|  %744 = gep %743, i64 16                                                                                               ;L2445<1476
 67858|  %745 = gep %738, i64 288                                                                                              ;L1476
 67859|  %746 = load ptr, ptr %745, , !!8                                                                                      ;L1476
 67860|  %747 = invoke zeroext i1 %746(ptr %744)
 67861|  to label %748 unwind label %178                                                                                       ;L1476
 67862| 
 67863| 748: ; preds = %736
 67864|  br i1 %747, label %749, label %732                                                                                    ;L1476
 67865| 
 67866| 749: ; preds = %748
 67867|     ;; self = ptr %615
 67868|  %750 = gep %615, i64 104                                                                                              ;L1404<1476
 67869|  %751 = load i64, ptr %750, , !!8                                                                                      ;L1404<1476
 67870|  %752 = icmp eq i64 %751, 13                                                                                           ;L1476
 67871|  br i1 %752, label %753, label %732                                                                                    ;L1476
 67872| 
 67873| 753: ; preds = %749
 67874|  %754 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %134, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %615)
 67875|  to label %755 unwind label %178                                                                                       ;L1477
 67876| 
 67877| 755: ; preds = %753
 67878|     ;; skill_damage = i64 %754
 67879|  %756 = invoke zeroext i1 @ai::utils13is_dash_worth(ptr %3, ptr %2, ptr %86, ptr %615, i64 %754)
 67880|  to label %757 unwind label %178                                                                                       ;L1478
 67881| 
 67882| 757: ; preds = %755
 67883|  br i1 %756, label %732, label %776                                                                                    ;L1478
 67884| 
 67885| 758: ; preds = %732
 67886|  br i1 %735, label %759, label %776                                                                                    ;L1483
 67887| 
 67888| 759: ; preds = %758
 67891|  %760 = gep %615, i64 1472                                                                                             ;L1491
 67892|  %761 = load i64, ptr %760, , !!8                                                                                      ;L1491
 67893|  invoke void @ai::small_action4castNtB5_16SmallActionSkill3new(ptr sret([24 x i8]) %52, ptr %3, i64 %761)
 67894|  to label %762 unwind label %178                                                                                       ;L1491
 67895| 
 67896| 762: ; preds = %759
 67897|  call void @llvm.memcpy.p0.p0.i64(ptr %53, ptr %52, i64 24, i1 false)                                                  ;L1491
 67898|  store i8 16, ptr %608,                                                                                                ;L1491
 67901|     ;; self = ptr %65
 67902|     ;; self = ptr %65
 67903|     ;; value = ptr %53
 67904|     ;; src = ptr %53
 67905|     ;; additional = i64 1
 67906|     ;; needed_extra_cap = i64 1
 67907|     ;; needed_extra_cap = i64 1
 67908|     ;; strategy = i8 1
 67909|  %763 = load i64, ptr %126, , !!74247, !!8                                                                             ;L1428<1491
 67910|     ;; self = ptr %65
 67911|  %764 = load i64, ptr %125, , !!74247, !!8                                                                             ;L149<1428<1491
 67912|  %765 = icmp eq i64 %763, %764                                                                                         ;L1428<1491
 67913|  br i1 %765, label %766, label %771                                                                                    ;L1428<1491
 67914| 
 67915| 766: ; preds = %762
 67916|     ;; self = ptr %65
 67917|     ;; self = ptr %65
 67918|     ;; self = ptr %65
 67919|     ;; used_cap = i64 %763
 67920|     ;; used_cap = i64 %763
 67921|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %763, i64 1, i1 zeroext true)
 67922|  to label %767 unwind label %769, !!74247                                                                              ;L619<430<738<1429<1491
 67923| 
 67924| 767: ; preds = %766
 67925|  %768 = load i64, ptr %126, , !!74247                                                                                  ;L1432<1491
 67926|  br label %771                                                                                                         ;L619<430<738<1429<1491
 67927| 
 67928| 769: ; preds = %766
 67929|  %770 = cleanuppad within none []
 67930|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %53) #30 [ "funclet"(token %770) ], !!74232 ;L1436<1491
 67931|  cleanupret from %770 unwind label %178
 67932| 
 67933| 771: ; preds = %767, %762
 67934|  %772 = phi i64 [ %768, %767 ], [ %763, %762 ]                                                                         ;L1432<1491
 67935|     ;; self = ptr %65
 67936|  %773 = load ptr, ptr %65, , !!74247, !!8, !!8                                                                         ;L138<1432<1491
 67937|     ;; self = ptr %773
 67938|     ;; count = i64 %772
 67939|  %774 = gepS %773, i64 %772                                                                                            ;L961<1432<1491
 67940|     ;; end = ptr %774
 67941|     ;; dst = ptr %774
 67942|  call void @llvm.memcpy.p0.p0.i64(ptr %774, ptr %53, i64 184, i1 false), !!74232                                       ;L1933<1433<1491
 67943|  %775 = add i64 %772, 1                                                                                                ;L1434<1491
 67944|  store i64 %775, ptr %126, , !!74247                                                                                   ;L1434<1491
 67946|  br label %776                                                                                                         ;L1433
 67947| 
 67948| 776: ; preds = %771, %758, %757, %729, %717, %680, %651, %642, %634
 67949|  br label %609                                                                                                         ;L1714<180<1433
 67950| 
 67951| 777: ; preds = %1154, %618
 67952|  %778 = phi ptr [ %619, %618 ], [ %781, %1154 ]                                                                        ;L1494
 67953|     ;; iter[0..+8] = ptr %778
 67954|     ;; self = ptr undef
 67955|     ;; ptr = ptr %778
 67956|     ;; self = ptr %778
 67957|     ;; end_or_len = ptr %622
 67960|  %779 = icmp eq ptr %778, %622                                                                                         ;L1714<180<1494
 67961|  br i1 %779, label %1155, label %780                                                                                   ;L180<1494
 67962| 
 67963| 780: ; preds = %777
 67964|  %781 = gep %778, i64 8                                                                                                ;L656<185<1494
 67965|     ;; iter[0..+8] = ptr %781
 67966|     ;; e = ptr %778
 67967|  %782 = load ptr, ptr %778, , !!8, !!8                                                                                 ;L1495
 67968|  %783 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %600, ptr %86, ptr %782)
 67969|  to label %784 unwind label %178                                                                                       ;L1495
 67970| 
 67971| 784: ; preds = %780
 67972|  br i1 %783, label %785, label %1154                                                                                   ;L1495
 67973| 
 67974| 785: ; preds = %784
 67975|  %786 = load ptr, ptr %601, , !!8, !!8                                                                                 ;L1499
 67976|  %787 = load ptr, ptr %602, , !!8, !!8                                                                                 ;L1499
 67977|  %788 = load ptr, ptr %82, , !!8, !!8                                                                                  ;L1499
 67978|  %789 = load ptr, ptr %603, , !!8, !!8                                                                                 ;L1499
 67979|  %790 = load ptr, ptr %778, , !!8, !!8                                                                                 ;L1499
 67980|  %791 = gep %787, i64 200                                                                                              ;L1499
 67981|  %792 = load ptr, ptr %791, , !!8                                                                                      ;L1499
 67982|  %793 = invoke zeroext i1 %792(ptr %786, ptr %788, ptr %789, ptr %86, ptr %790)
 67983|  to label %794 unwind label %178                                                                                       ;L1499
 67984| 
 67985| 794: ; preds = %785
 67986|  br i1 %793, label %795, label %1154                                                                                   ;L1499
 67987| 
 67988| 795: ; preds = %794
 67991|  %796 = load ptr, ptr %134, , !!8, !!8                                                                                 ;L441<2127<2445<1504
 67992|  %797 = load ptr, ptr %606, , !!8, !!8                                                                                 ;L441<2127<2445<1504
 67993|  %798 = gep %797, i64 16                                                                                               ;L2445<1504
 67994|  %799 = load i64, ptr %798,                                                                                            ;L2445<1504
 67995|  %800 = add nsw i64 %799, -1                                                                                           ;L2445<1504
 67996|  %801 = and i64 %800, -16                                                                                              ;L2445<1504
 67997|  %802 = gep %796, i64 %801                                                                                             ;L2445<1504
 67998|  %803 = gep %802, i64 16                                                                                               ;L2445<1504
 67999|  %804 = gep %797, i64 64                                                                                               ;L1504
 68000|  %805 = load ptr, ptr %804, , !!8                                                                                      ;L1504
 68001|  %806 = invoke i64 %805(ptr %803, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54)
 68002|  to label %807 unwind label %178                                                                                       ;L1504
 68003| 
 68004| 807: ; preds = %795
 68005|  %808 = icmp eq i64 %806, 0                                                                                            ;L1504
 68006|     ;; has_heal = i1 %808
 68009|  %809 = load ptr, ptr %134, , !!8, !!8                                                                                 ;L441<2127<2445<1505
 68010|  %810 = load ptr, ptr %606, , !!8, !!8                                                                                 ;L441<2127<2445<1505
 68011|  %811 = gep %810, i64 16                                                                                               ;L2445<1505
 68012|  %812 = load i64, ptr %811,                                                                                            ;L2445<1505
 68013|  %813 = add nsw i64 %812, -1                                                                                           ;L2445<1505
 68014|  %814 = and i64 %813, -16                                                                                              ;L2445<1505
 68015|  %815 = gep %809, i64 %814                                                                                             ;L2445<1505
 68016|  %816 = gep %815, i64 16                                                                                               ;L2445<1505
 68017|  %817 = gep %810, i64 72                                                                                               ;L1505
 68018|  %818 = load ptr, ptr %817, , !!8                                                                                      ;L1505
 68019|  %819 = invoke i64 %818(ptr %816, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54)
 68020|  to label %820 unwind label %178                                                                                       ;L1505
 68021| 
 68022| 820: ; preds = %807
 68023|  %821 = icmp eq i64 %819, 0                                                                                            ;L1505
 68024|     ;; has_shield = i1 %821
 68026|  %822 = load i64, ptr %73, , !!8                                                                                       ;L1506
 68027|  invoke void @ai::fight_check18effect_buff_target(ptr sret([288 x i8]) %51, i64 %822, ptr %134, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54)
 68028|  to label %823 unwind label %178                                                                                       ;L1506
 68029| 
 68030| 823: ; preds = %820
 68031|     ;; self = ptr %51
 68032|  %824 = load i32, ptr %623, , !!8                                                                                      ;L633<1507
 68033|  %825 = icmp ne i32 %824, -1                                                                                           ;L633<1507
 68034|     ;; has_buff = i1 %825
 68035|     ;; self = ptr %70
 68036|     ;; self = ptr %70
 68037|  %826 = load ptr, ptr %70, , !!8, !!8                                                                                  ;L138<2073<1508
 68038|     ;; p = ptr %826
 68039|  %827 = load i64, ptr %596, , !!8                                                                                      ;L2075<1508
 68040|     ;; len = i64 %827
 68041|     ;; count = i64 %827
 68042|     ;; self[0..+8] = ptr %826
 68043|     ;; slice[0..+8] = ptr %826
 68044|     ;; self[8..+8] = i64 %827
 68045|     ;; slice[8..+8] = i64 %827
 68046|     ;; ptr = ptr %826
 68047|     ;; self = ptr %826
 68048|  %828 = shl nuw nsw i64 %827, 5                                                                                        ;L961<100<1042<1508
 68049|  %829 = gep %826, i64 %828                                                                                             ;L961<100<1042<1508
 68050|  %830 = load ptr, ptr %778, , !!8, !!8                                                                                 ;L1508
 68053|     ;; f[0..+8] = ptr %86
 68054|     ;; f[8..+8] = ptr %830
 68055|     ;; self = ptr undef
 68056|     ;; self = ptr undef
 68057|     ;; count = i64 1
 68058|     ;; ptr = ptr %826
 68059|     ;; self = ptr %826
 68060|     ;; end_or_len = ptr %829
 68063|  %831 = icmp eq i64 %827, 0                                                                                            ;L1714<180<331<1508
 68064|  br i1 %831, label %832, label %837                                                                                    ;L180<331<1508
 68065| 
 68066| 832: ; preds = %823
 68067|  %833 = gep %830, i64 1632
 68068|  %834 = load i64, ptr %833, , !!74346
 68069|  %835 = gep %830, i64 1640
 68070|  %836 = load i64, ptr %835, , !!74346
 68071|  br label %919                                                                                                         ;L180<331<1508
 68072| 
 68073| 837: ; preds = %823
 68074|  %838 = load i64, ptr %86, , !!74348, !!8
 68075|  %839 = trunc nuw i64 %838 to i1
 68076|  %840 = load i64, ptr %599, , !!74348
 68077|  %841 = gep %830, i64 1632
 68078|  %842 = load i64, ptr %841, , !!74350
 68079|  %843 = gep %830, i64 1640
 68080|  %844 = load i64, ptr %843, , !!74350
 68081|  br i1 %839, label %845, label %887
 68082| 
 68083| 845: ; preds = %837
 68084|     ;; ptr = ptr %826
 68085|  %846 = gep %826, i64 24                                                                                               ;L332<1508
 68086|  %847 = load ptr, ptr %846, , !!74352, !!8, !!8                                                                        ;L332<1508
 68087|  %848 = gep %847, i64 1632                                                                                             ;L2158<1508<332<1508
 68088|  %849 = load i64, ptr %848, , !!74352, !!8                                                                             ;L2158<1508<332<1508
 68089|  %850 = gep %847, i64 1640                                                                                             ;L2158<1508<332<1508
 68090|  %851 = load i64, ptr %850, , !!74352, !!8                                                                             ;L2158<1508<332<1508
 68091|  %852 = icmp ult i64 %849, %842                                                                                        ;L3147<7<2158<1508<332<1508
 68092|  %853 = sub nuw i64 %842, %849                                                                                         ;L3147<7<2158<1508<332<1508
 68093|  %854 = sub nuw i64 %849, %842                                                                                         ;L3147<7<2158<1508<332<1508
 68094|  %855 = select i1 %852, i64 %853, i64 %854                                                                             ;L3147<7<2158<1508<332<1508
 68095|  %856 = icmp ult i64 %851, %844                                                                                        ;L3147<8<2158<1508<332<1508
 68096|  %857 = sub nuw i64 %844, %851                                                                                         ;L3147<8<2158<1508<332<1508
 68097|  %858 = sub nuw i64 %851, %844                                                                                         ;L3147<8<2158<1508<332<1508
 68098|  %859 = select i1 %856, i64 %857, i64 %858                                                                             ;L3147<8<2158<1508<332<1508
 68099|  %860 = mul i64 %855, %855                                                                                             ;L9<2158<1508<332<1508
 68100|  %861 = mul i64 %859, %859                                                                                             ;L9<2158<1508<332<1508
 68101|  %862 = add i64 %861, %860                                                                                             ;L9<2158<1508<332<1508
 68102|  %863 = icmp ult i64 %862, 14400000001                                                                                 ;L1508<332<1508
 68103|  br i1 %863, label %947, label %883                                                                                    ;L332<1508
 68104| 
 68105| 864: ; preds = %883
 68106|     ;; ptr = ptr %885
 68107|     ;; x = ptr %885
 68108|  %865 = gep %884, i64 56                                                                                               ;L332<1508
 68109|  %866 = load ptr, ptr %865, , !!74352, !!8, !!8                                                                        ;L332<1508
 68114|     ;; self = ptr %866
 68115|     ;; self = ptr %866
 68116|     ;; entity = ptr %86
 68117|     ;; other = ptr %830
 68118|  %867 = gep %866, i64 1632                                                                                             ;L2158<1508<332<1508
 68119|  %868 = load i64, ptr %867, , !!74352, !!8                                                                             ;L2158<1508<332<1508
 68120|     ;; x1 = i64 %868
 68121|     ;; self = i64 %868
 68122|  %869 = gep %866, i64 1640                                                                                             ;L2158<1508<332<1508
 68123|  %870 = load i64, ptr %869, , !!74352, !!8                                                                             ;L2158<1508<332<1508
 68124|     ;; y1 = i64 %870
 68125|     ;; self = i64 %870
 68126|     ;; x2 = i64 %842
 68127|     ;; other = i64 %842
 68128|     ;; y2 = i64 %844
 68129|     ;; other = i64 %844
 68130|  %871 = icmp ult i64 %868, %842                                                                                        ;L3147<7<2158<1508<332<1508
 68131|  %872 = sub nuw i64 %842, %868                                                                                         ;L3147<7<2158<1508<332<1508
 68132|  %873 = sub nuw i64 %868, %842                                                                                         ;L3147<7<2158<1508<332<1508
 68133|  %874 = select i1 %871, i64 %872, i64 %873                                                                             ;L3147<7<2158<1508<332<1508
 68134|     ;; dx = i64 %874
 68135|  %875 = icmp ult i64 %870, %844                                                                                        ;L3147<8<2158<1508<332<1508
 68136|  %876 = sub nuw i64 %844, %870                                                                                         ;L3147<8<2158<1508<332<1508
 68137|  %877 = sub nuw i64 %870, %844                                                                                         ;L3147<8<2158<1508<332<1508
 68138|  %878 = select i1 %875, i64 %876, i64 %877                                                                             ;L3147<8<2158<1508<332<1508
 68139|     ;; dy = i64 %878
 68140|  %879 = mul i64 %874, %874                                                                                             ;L9<2158<1508<332<1508
 68141|  %880 = mul i64 %878, %878                                                                                             ;L9<2158<1508<332<1508
 68142|  %881 = add i64 %880, %879                                                                                             ;L9<2158<1508<332<1508
 68143|  %882 = icmp ult i64 %881, 14400000001                                                                                 ;L1508<332<1508
 68144|  br i1 %882, label %947, label %883                                                                                    ;L332<1508
 68145| 
 68146| 883: ; preds = %864, %845
 68147|  %884 = phi ptr [ %885, %864 ], [ %826, %845 ]
 68148|  %885 = gep %884, i64 32                                                                                               ;L656<185<331<1508
 68149|     ;; ptr = ptr %885
 68150|     ;; self = ptr %885
 68151|     ;; end_or_len = ptr %829
 68154|  %886 = icmp eq ptr %885, %829                                                                                         ;L1714<180<331<1508
 68155|  br i1 %886, label %919, label %864                                                                                    ;L180<331<1508
 68156| 
 68157| 887: ; preds = %837
 68158|  %888 = icmp ult i64 %840, 2
 68159|  br i1 %888, label %889, label %917
 68160| 
 68161| 889: ; preds = %915, %887
 68162|  %890 = phi ptr [ %891, %915 ], [ %826, %887 ]
 68163|     ;; ptr = ptr %890
 68164|  %891 = gep %890, i64 32                                                                                               ;L656<185<331<1508
 68165|     ;; x = ptr %890
 68166|  %892 = gep %890, i64 24                                                                                               ;L332<1508
 68167|  %893 = load ptr, ptr %892, , !!74352, !!8, !!8                                                                        ;L332<1508
 68172|     ;; self = ptr %893
 68173|     ;; self = ptr %893
 68174|     ;; entity = ptr %86
 68175|     ;; team = i64 %840
 68177|  %894 = gep %893, i64 56                                                                                               ;L122<1483<1508<332<1508
 68178|  %895 = gepS %894, i64 %840                                                                                            ;L122<1483<1508<332<1508
 68179|  %896 = load i64, ptr %895, , !!74352, !!8                                                                             ;L122<1483<1508<332<1508
 68180|  %897 = icmp eq i64 %896, 0                                                                                            ;L122<1483<1508<332<1508
 68181|  br i1 %897, label %898, label %915                                                                                    ;L1508<332<1508
 68182| 
 68183| 898: ; preds = %889
 68184|     ;; other = ptr %830
 68185|  %899 = gep %893, i64 1632                                                                                             ;L2158<1508<332<1508
 68186|  %900 = load i64, ptr %899, , !!74352, !!8                                                                             ;L2158<1508<332<1508
 68187|     ;; x1 = i64 %900
 68188|     ;; self = i64 %900
 68189|  %901 = gep %893, i64 1640                                                                                             ;L2158<1508<332<1508
 68190|  %902 = load i64, ptr %901, , !!74352, !!8                                                                             ;L2158<1508<332<1508
 68191|     ;; y1 = i64 %902
 68192|     ;; self = i64 %902
 68193|     ;; x2 = i64 %842
 68194|     ;; other = i64 %842
 68195|     ;; y2 = i64 %844
 68196|     ;; other = i64 %844
 68197|  %903 = icmp ult i64 %900, %842                                                                                        ;L3147<7<2158<1508<332<1508
 68198|  %904 = sub nuw i64 %842, %900                                                                                         ;L3147<7<2158<1508<332<1508
 68199|  %905 = sub nuw i64 %900, %842                                                                                         ;L3147<7<2158<1508<332<1508
 68200|  %906 = select i1 %903, i64 %904, i64 %905                                                                             ;L3147<7<2158<1508<332<1508
 68201|     ;; dx = i64 %906
 68202|  %907 = icmp ult i64 %902, %844                                                                                        ;L3147<8<2158<1508<332<1508
 68203|  %908 = sub nuw i64 %844, %902                                                                                         ;L3147<8<2158<1508<332<1508
 68204|  %909 = sub nuw i64 %902, %844                                                                                         ;L3147<8<2158<1508<332<1508
 68205|  %910 = select i1 %907, i64 %908, i64 %909                                                                             ;L3147<8<2158<1508<332<1508
 68206|     ;; dy = i64 %910
 68207|  %911 = mul i64 %906, %906                                                                                             ;L9<2158<1508<332<1508
 68208|  %912 = mul i64 %910, %910                                                                                             ;L9<2158<1508<332<1508
 68209|  %913 = add i64 %912, %911                                                                                             ;L9<2158<1508<332<1508
 68210|  %914 = icmp ult i64 %913, 14400000001                                                                                 ;L1508<332<1508
 68211|  br i1 %914, label %947, label %915                                                                                    ;L332<1508
 68212| 
 68213| 915: ; preds = %898, %889
 68214|     ;; ptr = ptr %891
 68215|     ;; self = ptr %891
 68216|     ;; end_or_len = ptr %829
 68219|  %916 = icmp eq ptr %891, %829                                                                                         ;L1714<180<331<1508
 68220|  br i1 %916, label %919, label %889                                                                                    ;L180<331<1508
 68221| 
 68222| 917: ; preds = %887
 68223|     ;; ptr = ptr %826
 68224|     ;; x = ptr %826
 68231|     ;; entity = ptr %86
 68232|     ;; team = i64 %840
 68233|  invoke void @core::panicking18panic_bounds_check(i64 %840, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 68234|  to label %918 unwind label %178                                                                                       ;L1483<1508<332<1508
 68235| 
 68236| 918: ; preds = %917
 68237|  unreachable                                                                                                           ;L1483<1508<332<1508
 68238| 
 68239| 919: ; preds = %915, %883, %832
 68240|  %920 = phi i64 [ %836, %832 ], [ %844, %883 ], [ %844, %915 ]
 68241|  %921 = phi i64 [ %834, %832 ], [ %842, %883 ], [ %842, %915 ]
 68242|     ;; self = ptr %625
 68243|     ;; self = ptr %625
 68244|  %922 = load ptr, ptr %625, , !!8, !!8                                                                                 ;L138<2073<1509
 68245|     ;; p = ptr %922
 68246|  %923 = load i64, ptr %626, , !!8                                                                                      ;L2075<1509
 68247|     ;; len = i64 %923
 68248|     ;; count = i64 %923
 68249|     ;; self[0..+8] = ptr %922
 68250|     ;; slice[0..+8] = ptr %922
 68251|     ;; self[8..+8] = i64 %923
 68252|     ;; slice[8..+8] = i64 %923
 68253|     ;; ptr = ptr %922
 68254|     ;; self = ptr %922
 68255|  %924 = getelementptr ptr, ptr %922, i64 %923                                                                          ;L961<100<1042<1509
 68257|     ;; f = ptr %830
 68258|     ;; self = ptr undef
 68259|     ;; self = ptr undef
 68260|     ;; count = i64 1
 68261|  br label %925                                                                                                         ;L331<1509
 68262| 
 68263| 925: ; preds = %928, %919
 68264|  %926 = phi ptr [ %929, %928 ], [ %922, %919 ]
 68265|     ;; ptr = ptr %926
 68266|     ;; self = ptr %926
 68267|     ;; end_or_len = ptr %924
 68270|  %927 = icmp eq ptr %926, %924                                                                                         ;L1714<180<331<1509
 68271|  br i1 %927, label %947, label %928                                                                                    ;L180<331<1509
 68272| 
 68273| 928: ; preds = %925
 68274|  %929 = gep %926, i64 8                                                                                                ;L656<185<331<1509
 68275|     ;; x = ptr %926
 68276|  %930 = load ptr, ptr %926, , !!74453, !!8, !!8                                                                        ;L332<1509
 68279|     ;; self = ptr %930
 68280|     ;; other = ptr %830
 68281|  %931 = gep %930, i64 1632                                                                                             ;L2158<1509<332<1509
 68282|  %932 = load i64, ptr %931, , !!74453, !!8                                                                             ;L2158<1509<332<1509
 68283|     ;; x1 = i64 %932
 68284|     ;; self = i64 %932
 68285|  %933 = gep %930, i64 1640                                                                                             ;L2158<1509<332<1509
 68286|  %934 = load i64, ptr %933, , !!74453, !!8                                                                             ;L2158<1509<332<1509
 68287|     ;; y1 = i64 %934
 68288|     ;; self = i64 %934
 68289|     ;; x2 = i64 %921
 68290|     ;; other = i64 %921
 68291|     ;; y2 = i64 %920
 68292|     ;; other = i64 %920
 68293|  %935 = icmp ult i64 %932, %921                                                                                        ;L3147<7<2158<1509<332<1509
 68294|  %936 = sub nuw i64 %921, %932                                                                                         ;L3147<7<2158<1509<332<1509
 68295|  %937 = sub nuw i64 %932, %921                                                                                         ;L3147<7<2158<1509<332<1509
 68296|  %938 = select i1 %935, i64 %936, i64 %937                                                                             ;L3147<7<2158<1509<332<1509
 68297|     ;; dx = i64 %938
 68298|  %939 = icmp ult i64 %934, %920                                                                                        ;L3147<8<2158<1509<332<1509
 68299|  %940 = sub nuw i64 %920, %934                                                                                         ;L3147<8<2158<1509<332<1509
 68300|  %941 = sub nuw i64 %934, %920                                                                                         ;L3147<8<2158<1509<332<1509
 68301|  %942 = select i1 %939, i64 %940, i64 %941                                                                             ;L3147<8<2158<1509<332<1509
 68302|     ;; dy = i64 %942
 68303|  %943 = mul i64 %938, %938                                                                                             ;L9<2158<1509<332<1509
 68304|  %944 = mul i64 %942, %942                                                                                             ;L9<2158<1509<332<1509
 68305|  %945 = add i64 %944, %943                                                                                             ;L9<2158<1509<332<1509
 68306|  %946 = icmp ult i64 %945, 14400000001                                                                                 ;L1509<332<1509
 68307|  br i1 %946, label %947, label %925                                                                                    ;L332<1509
 68308| 
 68309| 947: ; preds = %928, %925, %898, %864, %845
 68310|  %948 = phi i1 [ true, %845 ], [ false, %925 ], [ true, %864 ], [ true, %928 ], [ true, %898 ]                         ;L0
 68312|  %949 = gep %830, i64 1576                                                                                             ;L1511
 68313|  %950 = load i64, ptr %949, , !!8                                                                                      ;L1511
 68314|  %951 = icmp eq i64 %950, 0                                                                                            ;L1511
 68315|  br i1 %951, label %957, label %952                                                                                    ;L1511
 68316| 
 68317| 952: ; preds = %947
 68318|  %953 = gep %830, i64 1648                                                                                             ;L1511
 68319|  %954 = load i64, ptr %953, , !!8                                                                                      ;L1511
 68320|  %955 = mul i64 %954, 100                                                                                              ;L1511
 68321|  %956 = udiv i64 %955, %950                                                                                            ;L1511
 68322|     ;; hp_ratio = i64 %956
 68323|  br i1 %808, label %958, label %961                                                                                    ;L1513
 68324| 
 68325| 957: ; preds = %947
 68326|  invoke void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.271) #31
 68327|  to label %118 unwind label %178                                                                                       ;L1511
 68328| 
 68329| 958: ; preds = %952
 68330|  %959 = or i1 %821, %825                                                                                               ;L1519
 68331|  %960 = or i1 %959, %948                                                                                               ;L1519
 68332|  br i1 %960, label %973, label %972                                                                                    ;L1519
 68333| 
 68334| 961: ; preds = %952
 68335|  br i1 %825, label %973, label %962                                                                                    ;L1513
 68336| 
 68337| 962: ; preds = %961
 68338|  %963 = icmp ugt i64 %956, 79                                                                                          ;L1513
 68339|  br i1 %963, label %966, label %964                                                                                    ;L1513
 68340| 
 68341| 964: ; preds = %962
 68342|  %965 = or i1 %821, %948                                                                                               ;L1519
 68343|  br i1 %965, label %973, label %972                                                                                    ;L1519
 68344| 
 68345| 966: ; preds = %962
 68346|  %967 = load i64, ptr %73, , !!8                                                                                       ;L1515
 68347|  %968 = invoke zeroext i1 @ai::buff_value24aoe_heal_covers_low_ally(i64 %967, ptr %134, ptr %3, ptr %2, ptr %830)
 68348|  to label %969 unwind label %178                                                                                       ;L1515
 68349| 
 68350| 969: ; preds = %966
 68351|  %970 = or i1 %821, %948
 68352|  %971 = and i1 %970, %968                                                                                              ;L1515
 68353|  br i1 %971, label %973, label %972                                                                                    ;L1515
 68354| 
 68355| 972: ; preds = %1091, %998, %990, %969, %964, %958
 68357|  br label %1154                                                                                                        ;L1
 68358| 
 68359| 973: ; preds = %969, %964, %961, %958
 68360|     ;; self = ptr %51
 68361|     ;; default = i1 false
 68363|  %974 = load i32, ptr %627,                                                                                            ;L1226<1525
 68367|  %975 = load ptr, ptr %134, , !!8, !!8                                                                                 ;L441<2127<2445<1526
 68368|  %976 = load ptr, ptr %606, , !!8, !!8                                                                                 ;L441<2127<2445<1526
 68369|  %977 = gep %976, i64 16                                                                                               ;L2445<1526
 68370|  %978 = load i64, ptr %977,                                                                                            ;L2445<1526
 68371|  %979 = add nsw i64 %978, -1                                                                                           ;L2445<1526
 68372|  %980 = and i64 %979, -16                                                                                              ;L2445<1526
 68373|  %981 = gep %975, i64 %980                                                                                             ;L2445<1526
 68374|  %982 = gep %981, i64 16                                                                                               ;L2445<1526
 68375|  %983 = gep %976, i64 144                                                                                              ;L1526
 68376|  %984 = load ptr, ptr %983, , !!8                                                                                      ;L1526
 68377|  %985 = invoke zeroext i1 %984(ptr %982)
 68378|  to label %986 unwind label %178                                                                                       ;L1526
 68379| 
 68380| 986: ; preds = %973
 68381|  %987 = icmp eq i32 %974, 0                                                                                            ;L1226<1525
 68382|  %988 = select i1 %825, i1 %987, i1 false                                                                              ;L1226<1525
 68383|     ;; is_non_movespeed_buff = i1 %988
 68384|     ;; is_etc_buff = i1 %985
 68385|  %989 = or i1 %988, %985                                                                                               ;L1527
 68386|  br i1 %989, label %990, label %995                                                                                    ;L1527
 68387| 
 68388| 990: ; preds = %986
 68389|  %991 = load ptr, ptr %778, , !!8, !!8                                                                                 ;L1528
 68390|     ;; self = ptr %991
 68391|  %992 = gep %991, i64 1216                                                                                             ;L742<1528
 68392|  %993 = load i32, ptr %992, , !!8                                                                                      ;L742<1528
 68393|  %994 = icmp eq i32 %993, -1                                                                                           ;L742<1528
 68394|  br i1 %994, label %972, label %998                                                                                    ;L742<1528
 68395| 
 68396| 995: ; preds = %1071, %986
 68398|  %996 = load ptr, ptr %778, , !!8, !!8                                                                                 ;L1543
 68399|  %997 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %134, ptr %86, ptr %996)
 68400|  to label %1093 unwind label %178                                                                                      ;L1543
 68401| 
 68402| 998: ; preds = %990
 68403|  %999 = gep %991, i64 1168                                                                                             ;L742<1528
 68404|     ;; target_atk = ptr %999
 68405|     ;; self = ptr %70
 68406|     ;; self = ptr %70
 68407|  %1000 = load ptr, ptr %70, , !!8, !!8                                                                                 ;L138<2073<1529
 68408|     ;; p = ptr %1000
 68409|  %1001 = load i64, ptr %596, , !!8                                                                                     ;L2075<1529
 68410|     ;; len = i64 %1001
 68411|     ;; count = i64 %1001
 68412|     ;; self[0..+8] = ptr %1000
 68413|     ;; slice[0..+8] = ptr %1000
 68414|     ;; self[8..+8] = i64 %1001
 68415|     ;; slice[8..+8] = i64 %1001
 68416|     ;; ptr = ptr %1000
 68417|     ;; self = ptr %1000
 68418|  %1002 = shl nuw nsw i64 %1001, 5                                                                                      ;L961<100<1042<1529
 68419|  %1003 = gep %1000, i64 %1002                                                                                          ;L961<100<1042<1529
 68420|     ;; f[0..+8] = ptr %999
 68421|     ;; f[8..+8] = ptr %991
 68422|     ;; f[16..+8] = ptr %86
 68423|     ;; self = ptr undef
 68424|     ;; self = ptr undef
 68425|     ;; count = i64 1
 68426|     ;; ptr = ptr %1000
 68427|     ;; self = ptr %1000
 68428|     ;; end_or_len = ptr %1003
 68431|  %1004 = icmp eq i64 %1001, 0                                                                                          ;L1714<180<331<1529
 68432|  br i1 %1004, label %972, label %1005                                                                                  ;L180<331<1529
 68433| 
 68434| 1005: ; preds = %998
 68435|  %1006 = gep %991, i64 1184
 68436|  %1007 = gep %991, i64 1192
 68437|  %1008 = gep %991, i64 1480
 68438|  %1009 = gep %991, i64 1080
 68439|  %1010 = gep %991, i64 1136
 68440|  %1011 = gep %991, i64 1664
 68441|  %1012 = gep %991, i64 1632
 68442|  %1013 = gep %991, i64 1640
 68443|  br label %1014                                                                                                        ;L180<331<1529
 68444| 
 68445| 1014: ; preds = %1091, %1005
 68446|  %1015 = phi ptr [ %1000, %1005 ], [ %1016, %1091 ]
 68447|     ;; ptr = ptr %1015
 68448|  %1016 = gep %1015, i64 32                                                                                             ;L656<185<331<1529
 68449|     ;; x = ptr %1015
 68450|  %1017 = gep %1015, i64 24                                                                                             ;L332<1529
 68451|  %1018 = load ptr, ptr %1017, , !!74579, !!8, !!8                                                                      ;L332<1529
 68457|     ;; self = ptr %999
 68458|     ;; caster = ptr %991
 68459|  %1019 = load i64, ptr %1006, , !!74606, !!8                                                                           ;L26<1530<332<1529
 68460|  %1020 = load i64, ptr %1007, , !!74606, !!8                                                                           ;L26<1530<332<1529
 68461|  %1021 = load i64, ptr %1008, , !!74606, !!8                                                                           ;L26<1530<332<1529
 68462|  %1022 = add i64 %1021, -1                                                                                             ;L26<1530<332<1529
 68463|  %1023 = mul i64 %1022, %1020                                                                                          ;L26<1530<332<1529
 68464|  %1024 = load i64, ptr %1009, , !!74606, !!8                                                                           ;L26<1530<332<1529
 68465|     ;; self = ptr %1018
 68466|     ;; self = ptr %1018
 68467|     ;; self = ptr %1018
 68468|  %1025 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %999, ptr %991, ptr %1018)
 68469|  to label %1026 unwind label %178                                                                                      ;L1530<332<1529
 68470| 
 68471| 1026: ; preds = %1014
 68472|     ;; self = ptr %991
 68473|  %1027 = load i32, ptr %1010, , !!74606, !!8                                                                           ;L1511<1530<332<1529
 68474|     ;; mult = i32 %1027
 68475|  %1028 = icmp eq i32 %1027, 0                                                                                          ;L1512<1530<332<1529
 68476|  br i1 %1028, label %1029, label %1031                                                                                 ;L1512<1530<332<1529
 68477| 
 68478| 1029: ; preds = %1026
 68479|  %1030 = load i64, ptr %1011, , !!74606, !!8                                                                           ;L1513<1530<332<1529
 68480|  br label %1037                                                                                                        ;L1512<1530<332<1529
 68481| 
 68482| 1031: ; preds = %1026
 68483|  %1032 = sext i32 %1027 to i64                                                                                         ;L1511<1530<332<1529
 68484|     ;; mult = i64 %1032
 68485|  %1033 = load i64, ptr %1011, , !!74606, !!8                                                                           ;L1515<1530<332<1529
 68486|  %1034 = add nsw i64 %1032, 100                                                                                        ;L1515<1530<332<1529
 68487|  %1035 = mul i64 %1033, %1034                                                                                          ;L1515<1530<332<1529
 68488|  %1036 = udiv i64 %1035, 100                                                                                           ;L1515<1530<332<1529
 68489|  br label %1037                                                                                                        ;L1512<1530<332<1529
 68490| 
 68491| 1037: ; preds = %1031, %1029
 68492|  %1038 = phi i64 [ %1030, %1029 ], [ %1036, %1031 ]                                                                    ;L0<1530<332<1529
 68493|  %1039 = gep %1018, i64 1136                                                                                           ;L1511<1530<332<1529
 68494|  %1040 = load i32, ptr %1039, , !!74606, !!8                                                                           ;L1511<1530<332<1529
 68495|     ;; mult = i32 %1040
 68496|  %1041 = icmp eq i32 %1040, 0                                                                                          ;L1512<1530<332<1529
 68497|  br i1 %1041, label %1042, label %1045                                                                                 ;L1512<1530<332<1529
 68498| 
 68499| 1042: ; preds = %1037
 68500|  %1043 = gep %1018, i64 1664                                                                                           ;L1513<1530<332<1529
 68501|  %1044 = load i64, ptr %1043, , !!74606, !!8                                                                           ;L1513<1530<332<1529
 68502|  br label %1052                                                                                                        ;L1512<1530<332<1529
 68503| 
 68504| 1045: ; preds = %1037
 68505|  %1046 = sext i32 %1040 to i64                                                                                         ;L1511<1530<332<1529
 68506|     ;; mult = i64 %1046
 68507|  %1047 = gep %1018, i64 1664                                                                                           ;L1515<1530<332<1529
 68508|  %1048 = load i64, ptr %1047, , !!74606, !!8                                                                           ;L1515<1530<332<1529
 68509|  %1049 = add nsw i64 %1046, 100                                                                                        ;L1515<1530<332<1529
 68510|  %1050 = mul i64 %1048, %1049                                                                                          ;L1515<1530<332<1529
 68511|  %1051 = udiv i64 %1050, 100                                                                                           ;L1515<1530<332<1529
 68512|  br label %1052                                                                                                        ;L1512<1530<332<1529
 68513| 
 68514| 1052: ; preds = %1045, %1042
 68515|  %1053 = phi i64 [ %1044, %1042 ], [ %1051, %1045 ]                                                                    ;L0<1530<332<1529
 68516|  %1054 = add i64 %1024, %1019                                                                                          ;L26<1530<332<1529
 68517|  %1055 = add i64 %1054, %1023                                                                                          ;L26<1530<332<1529
 68518|  %1056 = add i64 %1055, %1025                                                                                          ;L1530<332<1529
 68519|  %1057 = add i64 %1056, %1038                                                                                          ;L1530<332<1529
 68520|  %1058 = add i64 %1057, %1053                                                                                          ;L1530<332<1529
 68521|     ;; attack_range = i64 %1058
 68522|     ;; entity = ptr %86
 68523|     ;; self = ptr %86
 68524|  %1059 = load i64, ptr %86, , !!74606, !!8                                                                             ;L1136<1482<1531<332<1529
 68525|  %1060 = trunc nuw i64 %1059 to i1                                                                                     ;L1136<1482<1531<332<1529
 68526|  br i1 %1060, label %1071, label %1061                                                                                 ;L1136<1482<1531<332<1529
 68527| 
 68528| 1061: ; preds = %1052
 68529|     ;; team = ptr %86
 68530|  %1062 = load i64, ptr %599, , !!74606, !!8                                                                            ;L1137<1482<1531<332<1529
 68531|     ;; team = i64 %1062
 68532|  %1063 = icmp ult i64 %1062, 2                                                                                         ;L1483<1531<332<1529
 68533|  br i1 %1063, label %1064, label %1069                                                                                 ;L1483<1531<332<1529
 68534| 
 68535| 1064: ; preds = %1061
 68537|  %1065 = gep %1018, i64 56                                                                                             ;L122<1483<1531<332<1529
 68538|  %1066 = gepS %1065, i64 %1062                                                                                         ;L122<1483<1531<332<1529
 68539|  %1067 = load i64, ptr %1066, , !!74606, !!8                                                                           ;L122<1483<1531<332<1529
 68540|  %1068 = icmp eq i64 %1067, 0                                                                                          ;L122<1483<1531<332<1529
 68541|  br i1 %1068, label %1071, label %1091                                                                                 ;L1531<332<1529
 68542| 
 68543| 1069: ; preds = %1061
 68544|  invoke void @core::panicking18panic_bounds_check(i64 %1062, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 68545|  to label %1070 unwind label %178                                                                                      ;L1483<1531<332<1529
 68546| 
 68547| 1070: ; preds = %1069
 68548|  unreachable                                                                                                           ;L1483<1531<332<1529
 68549| 
 68550| 1071: ; preds = %1064, %1052
 68551|     ;; other = ptr %991
 68552|  %1072 = gep %1018, i64 1632                                                                                           ;L2158<1531<332<1529
 68553|  %1073 = load i64, ptr %1072, , !!74606, !!8                                                                           ;L2158<1531<332<1529
 68554|     ;; x1 = i64 %1073
 68555|     ;; self = i64 %1073
 68556|  %1074 = gep %1018, i64 1640                                                                                           ;L2158<1531<332<1529
 68557|  %1075 = load i64, ptr %1074, , !!74606, !!8                                                                           ;L2158<1531<332<1529
 68558|     ;; y1 = i64 %1075
 68559|     ;; self = i64 %1075
 68560|  %1076 = load i64, ptr %1012, , !!74606, !!8                                                                           ;L2158<1531<332<1529
 68561|     ;; x2 = i64 %1076
 68562|     ;; other = i64 %1076
 68563|  %1077 = load i64, ptr %1013, , !!74606, !!8                                                                           ;L2158<1531<332<1529
 68564|     ;; y2 = i64 %1077
 68565|     ;; other = i64 %1077
 68566|  %1078 = icmp ult i64 %1073, %1076                                                                                     ;L3147<7<2158<1531<332<1529
 68567|  %1079 = sub nuw i64 %1076, %1073                                                                                      ;L3147<7<2158<1531<332<1529
 68568|  %1080 = sub nuw i64 %1073, %1076                                                                                      ;L3147<7<2158<1531<332<1529
 68569|  %1081 = select i1 %1078, i64 %1079, i64 %1080                                                                         ;L3147<7<2158<1531<332<1529
 68570|     ;; dx = i64 %1081
 68571|  %1082 = icmp ult i64 %1075, %1077                                                                                     ;L3147<8<2158<1531<332<1529
 68572|  %1083 = sub nuw i64 %1077, %1075                                                                                      ;L3147<8<2158<1531<332<1529
 68573|  %1084 = sub nuw i64 %1075, %1077                                                                                      ;L3147<8<2158<1531<332<1529
 68574|  %1085 = select i1 %1082, i64 %1083, i64 %1084                                                                         ;L3147<8<2158<1531<332<1529
 68575|     ;; dy = i64 %1085
 68576|  %1086 = mul i64 %1081, %1081                                                                                          ;L9<2158<1531<332<1529
 68577|  %1087 = mul i64 %1085, %1085                                                                                          ;L9<2158<1531<332<1529
 68578|  %1088 = add i64 %1087, %1086                                                                                          ;L9<2158<1531<332<1529
 68579|  %1089 = mul i64 %1058, %1058                                                                                          ;L1531<332<1529
 68580|  %1090 = icmp ugt i64 %1088, %1089                                                                                     ;L1531<332<1529
 68581|  br i1 %1090, label %1091, label %995                                                                                  ;L332<1529
 68582| 
 68583| 1091: ; preds = %1071, %1064
 68584|     ;; ptr = ptr %1016
 68585|     ;; self = ptr %1016
 68586|     ;; end_or_len = ptr %1003
 68589|  %1092 = icmp eq ptr %1016, %1003                                                                                      ;L1714<180<331<1529
 68590|  br i1 %1092, label %972, label %1014                                                                                  ;L180<331<1529
 68591| 
 68592| 1093: ; preds = %995
 68593|  %1094 = load ptr, ptr %778, , !!8, !!8                                                                                ;L1543
 68594|     ;; self = ptr %1094
 68595|  %1095 = gep %1094, i64 1136                                                                                           ;L1511<1543
 68596|  %1096 = load i32, ptr %1095, , !!8                                                                                    ;L1511<1543
 68597|     ;; mult = i32 %1096
 68598|  %1097 = icmp eq i32 %1096, 0                                                                                          ;L1512<1543
 68599|  br i1 %1097, label %1098, label %1101                                                                                 ;L1512<1543
 68600| 
 68601| 1098: ; preds = %1093
 68602|  %1099 = gep %1094, i64 1664                                                                                           ;L1513<1543
 68603|  %1100 = load i64, ptr %1099, , !!8                                                                                    ;L1513<1543
 68604|  br label %1108                                                                                                        ;L1512<1543
 68605| 
 68606| 1101: ; preds = %1093
 68607|  %1102 = sext i32 %1096 to i64                                                                                         ;L1511<1543
 68608|     ;; mult = i64 %1102
 68609|  %1103 = gep %1094, i64 1664                                                                                           ;L1515<1543
 68610|  %1104 = load i64, ptr %1103, , !!8                                                                                    ;L1515<1543
 68611|  %1105 = add nsw i64 %1102, 100                                                                                        ;L1515<1543
 68612|  %1106 = mul i64 %1104, %1105                                                                                          ;L1515<1543
 68613|  %1107 = udiv i64 %1106, 100                                                                                           ;L1515<1543
 68614|  br label %1108                                                                                                        ;L1512<1543
 68615| 
 68616| 1108: ; preds = %1101, %1098
 68617|  %1109 = phi i64 [ %1100, %1098 ], [ %1107, %1101 ]                                                                    ;L0<1543
 68619|     ;; self = ptr %1094
 68620|  %1110 = gep %1094, i64 1632                                                                                           ;L2158<1544
 68621|  %1111 = load i64, ptr %1110, , !!8                                                                                    ;L2158<1544
 68622|     ;; x1 = i64 %1111
 68623|     ;; self = i64 %1111
 68624|  %1112 = gep %1094, i64 1640                                                                                           ;L2158<1544
 68625|  %1113 = load i64, ptr %1112, , !!8                                                                                    ;L2158<1544
 68626|     ;; y1 = i64 %1113
 68627|     ;; self = i64 %1113
 68628|  %1114 = load i64, ptr %604, , !!8                                                                                     ;L2158<1544
 68629|     ;; x2 = i64 %1114
 68630|     ;; other = i64 %1114
 68631|  %1115 = load i64, ptr %605, , !!8                                                                                     ;L2158<1544
 68632|     ;; y2 = i64 %1115
 68633|     ;; other = i64 %1115
 68634|  %1116 = icmp ult i64 %1111, %1114                                                                                     ;L3147<7<2158<1544
 68635|  %1117 = sub nuw i64 %1114, %1111                                                                                      ;L3147<7<2158<1544
 68636|  %1118 = sub nuw i64 %1111, %1114                                                                                      ;L3147<7<2158<1544
 68637|  %1119 = select i1 %1116, i64 %1117, i64 %1118                                                                         ;L3147<7<2158<1544
 68638|     ;; dx = i64 %1119
 68639|  %1120 = icmp ult i64 %1113, %1115                                                                                     ;L3147<8<2158<1544
 68640|  %1121 = sub nuw i64 %1115, %1113                                                                                      ;L3147<8<2158<1544
 68641|  %1122 = sub nuw i64 %1113, %1115                                                                                      ;L3147<8<2158<1544
 68642|  %1123 = select i1 %1120, i64 %1121, i64 %1122                                                                         ;L3147<8<2158<1544
 68643|     ;; dy = i64 %1123
 68644|  %1124 = mul i64 %1119, %1119                                                                                          ;L9<2158<1544
 68645|  %1125 = mul i64 %1123, %1123                                                                                          ;L9<2158<1544
 68646|  %1126 = add i64 %1125, %1124                                                                                          ;L9<2158<1544
 68647|     ;; dist_sq = i64 %1126
 68648|     ;; max_tick = i64 %97
 68649|  %1127 = add i64 %629, %997                                                                                            ;L1543
 68650|  %1128 = add i64 %1127, %1109                                                                                          ;L1553
 68651|     ;; max_dist = i64 %1128
 68652|  %1129 = mul i64 %1128, %1128                                                                                          ;L1554
 68653|  %1130 = icmp ugt i64 %1126, %1129                                                                                     ;L1554
 68654|  br i1 %1130, label %1154, label %1131                                                                                 ;L1554
 68655| 
 68656| 1131: ; preds = %1108
 68657|  %1132 = gep %787, i64 192                                                                                             ;L1558
 68658|  %1133 = load ptr, ptr %1132, , !!8                                                                                    ;L1558
 68659|  %1134 = invoke zeroext i1 %1133(ptr %786, ptr %788, ptr %789, ptr %86)
 68660|  to label %1135 unwind label %178                                                                                      ;L1558
 68661| 
 68662| 1135: ; preds = %1131
 68663|  br i1 %1134, label %1136, label %1154                                                                                 ;L1558
 68664| 
 68665| 1136: ; preds = %1135
 68668|  %1137 = load ptr, ptr %778, , !!8, !!8                                                                                ;L1562
 68669|  %1138 = gep %1137, i64 1472                                                                                           ;L1562
 68670|  %1139 = load i64, ptr %1138, , !!8                                                                                    ;L1562
 68671|  invoke void @ai::small_action4castNtB5_16SmallActionSkill3new(ptr sret([24 x i8]) %49, ptr %3, i64 %1139)
 68672|  to label %1140 unwind label %178                                                                                      ;L1562
 68673| 
 68674| 1140: ; preds = %1136
 68675|  call void @llvm.memcpy.p0.p0.i64(ptr %50, ptr %49, i64 24, i1 false)                                                  ;L1562
 68676|  store i8 16, ptr %630,                                                                                                ;L1562
 68679|     ;; self = ptr %65
 68680|     ;; self = ptr %65
 68681|     ;; value = ptr %50
 68682|     ;; src = ptr %50
 68683|     ;; additional = i64 1
 68684|     ;; needed_extra_cap = i64 1
 68685|     ;; needed_extra_cap = i64 1
 68686|     ;; strategy = i8 1
 68687|  %1141 = load i64, ptr %126, , !!74729, !!8                                                                            ;L1428<1562
 68688|     ;; self = ptr %65
 68689|  %1142 = load i64, ptr %125, , !!74729, !!8                                                                            ;L149<1428<1562
 68690|  %1143 = icmp eq i64 %1141, %1142                                                                                      ;L1428<1562
 68691|  br i1 %1143, label %1144, label %1149                                                                                 ;L1428<1562
 68692| 
 68693| 1144: ; preds = %1140
 68694|     ;; self = ptr %65
 68695|     ;; self = ptr %65
 68696|     ;; self = ptr %65
 68697|     ;; used_cap = i64 %1141
 68698|     ;; used_cap = i64 %1141
 68699|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %1141, i64 1, i1 zeroext true)
 68700|  to label %1145 unwind label %1147, !!74729                                                                            ;L619<430<738<1429<1562
 68701| 
 68702| 1145: ; preds = %1144
 68703|  %1146 = load i64, ptr %126, , !!74729                                                                                 ;L1432<1562
 68704|  br label %1149                                                                                                        ;L619<430<738<1429<1562
 68705| 
 68706| 1147: ; preds = %1144
 68707|  %1148 = cleanuppad within none []
 68708|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %50) #30 [ "funclet"(token %1148) ], !!74714 ;L1436<1562
 68709|  cleanupret from %1148 unwind label %178
 68710| 
 68711| 1149: ; preds = %1145, %1140
 68712|  %1150 = phi i64 [ %1146, %1145 ], [ %1141, %1140 ]                                                                    ;L1432<1562
 68713|     ;; self = ptr %65
 68714|  %1151 = load ptr, ptr %65, , !!74729, !!8, !!8                                                                        ;L138<1432<1562
 68715|     ;; self = ptr %1151
 68716|     ;; count = i64 %1150
 68717|  %1152 = gepS %1151, i64 %1150                                                                                         ;L961<1432<1562
 68718|     ;; end = ptr %1152
 68719|     ;; dst = ptr %1152
 68720|  call void @llvm.memcpy.p0.p0.i64(ptr %1152, ptr %50, i64 184, i1 false), !!74714                                      ;L1933<1433<1562
 68721|  %1153 = add i64 %1150, 1                                                                                              ;L1434<1562
 68722|  store i64 %1153, ptr %126, , !!74729                                                                                  ;L1434<1562
 68724|  br label %1154                                                                                                        ;L1494
 68725| 
 68726| 1154: ; preds = %1149, %1135, %1108, %972, %794, %784
 68727|  br label %777                                                                                                         ;L1714<180<1494
 68728| 
 68729| 1155: ; preds = %777
 68730|  %1156 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %600, ptr %86, ptr %86)
 68731|  to label %1157 unwind label %178                                                                                      ;L1565
 68732| 
 68733| 1157: ; preds = %1155
 68734|  br i1 %1156, label %1158, label %592                                                                                  ;L1565
 68735| 
 68736| 1158: ; preds = %1157
 68737|  %1159 = load ptr, ptr %601, , !!8, !!8                                                                                ;L1565
 68738|  %1160 = load ptr, ptr %602, , !!8, !!8                                                                                ;L1565
 68739|  %1161 = load ptr, ptr %82, , !!8, !!8                                                                                 ;L1565
 68740|  %1162 = load ptr, ptr %603, , !!8, !!8                                                                                ;L1565
 68741|  %1163 = gep %1160, i64 200                                                                                            ;L1565
 68742|  %1164 = load ptr, ptr %1163, , !!8                                                                                    ;L1565
 68743|  %1165 = invoke zeroext i1 %1164(ptr %1159, ptr %1161, ptr %1162, ptr %86, ptr %86)
 68744|  to label %1166 unwind label %178                                                                                      ;L1565
 68745| 
 68746| 1166: ; preds = %1158
 68747|  br i1 %1165, label %1167, label %592                                                                                  ;L1565
 68748| 
 68749| 1167: ; preds = %1166
 68750|     ;; skip_self_buff = i8 0
 68752|  %1168 = load i64, ptr %73, , !!8                                                                                      ;L1569
 68753|  invoke void @ai::fight_check18effect_buff_target(ptr sret([288 x i8]) %48, i64 %1168, ptr %134, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54)
 68754|  to label %1169 unwind label %178                                                                                      ;L1569
 68755| 
 68756| 1169: ; preds = %1167
 68757|     ;; self = ptr %48
 68758|     ;; default = i1 false
 68760|  %1170 = gep %48, i64 72                                                                                               ;L1226<1570
 68761|  %1171 = load i32, ptr %1170, , !!8                                                                                    ;L1226<1570
 68762|  %1172 = gep %48, i64 136                                                                                              ;L1226<1570
 68763|  %1173 = load i32, ptr %1172,                                                                                          ;L1226<1570
 68767|  %1174 = load ptr, ptr %134, , !!8, !!8                                                                                ;L441<2127<2445<1571
 68768|  %1175 = load ptr, ptr %607, , !!8, !!8                                                                                ;L441<2127<2445<1571
 68769|  %1176 = gep %1175, i64 16                                                                                             ;L2445<1571
 68770|  %1177 = load i64, ptr %1176,                                                                                          ;L2445<1571
 68771|  %1178 = add nsw i64 %1177, -1                                                                                         ;L2445<1571
 68772|  %1179 = and i64 %1178, -16                                                                                            ;L2445<1571
 68773|  %1180 = gep %1174, i64 %1179                                                                                          ;L2445<1571
 68774|  %1181 = gep %1180, i64 16                                                                                             ;L2445<1571
 68775|  %1182 = gep %1175, i64 144                                                                                            ;L1571
 68776|  %1183 = load ptr, ptr %1182, , !!8                                                                                    ;L1571
 68777|  %1184 = invoke zeroext i1 %1183(ptr %1181)
 68778|  to label %1185 unwind label %178                                                                                      ;L1571
 68779| 
 68780| 1185: ; preds = %1169
 68781|  %1186 = icmp ne i32 %1171, -1                                                                                         ;L1226<1570
 68782|  %1187 = icmp eq i32 %1173, 0                                                                                          ;L1226<1570
 68783|  %1188 = select i1 %1186, i1 %1187, i1 false                                                                           ;L1226<1570
 68784|     ;; is_non_movespeed_buff = i1 %1188
 68785|     ;; is_etc_buff = i1 %1184
 68786|  %1189 = or i1 %1188, %1184                                                                                            ;L1572
 68787|  br i1 %1189, label %1190, label %1191                                                                                 ;L1572
 68788| 
 68789| 1190: ; preds = %1185
 68790|     ;; self = ptr %86
 68791|  br i1 %130, label %1430, label %1195                                                                                  ;L742<1573
 68792| 
 68793| 1191: ; preds = %1402, %1352, %1298, %1253, %1185
 68795|  %1192 = gep %1160, i64 192                                                                                            ;L1586
 68796|  %1193 = load ptr, ptr %1192, , !!8                                                                                    ;L1586
 68797|  %1194 = invoke zeroext i1 %1193(ptr %1159, ptr %1161, ptr %1162, ptr %86)
 68798|  to label %1431 unwind label %178                                                                                      ;L1586
 68799| 
 68800| 1195: ; preds = %1190
 68801|     ;; target_atk = ptr %127
 68802|     ;; self = ptr %70
 68803|     ;; self = ptr %70
 68804|  %1196 = load ptr, ptr %70, , !!8, !!8                                                                                 ;L138<2073<1574
 68805|     ;; p = ptr %1196
 68806|  %1197 = load i64, ptr %596, , !!8                                                                                     ;L2075<1574
 68807|     ;; len = i64 %1197
 68808|     ;; count = i64 %1197
 68809|     ;; self[0..+8] = ptr %1196
 68810|     ;; slice[0..+8] = ptr %1196
 68811|     ;; self[8..+8] = i64 %1197
 68812|     ;; slice[8..+8] = i64 %1197
 68813|     ;; ptr = ptr %1196
 68814|     ;; self = ptr %1196
 68815|  %1198 = shl nuw nsw i64 %1197, 5                                                                                      ;L961<100<1042<1574
 68816|  %1199 = gep %1196, i64 %1198                                                                                          ;L961<100<1042<1574
 68819|     ;; f[0..+8] = ptr %127
 68820|     ;; f[8..+8] = ptr %86
 68821|     ;; self = ptr undef
 68822|     ;; self = ptr undef
 68823|     ;; count = i64 1
 68824|     ;; ptr = ptr %1196
 68825|     ;; self = ptr %1196
 68826|     ;; end_or_len = ptr %1199
 68829|  %1200 = icmp eq i64 %1197, 0                                                                                          ;L1714<180<331<1574
 68830|  br i1 %1200, label %1430, label %1201                                                                                 ;L180<331<1574
 68831| 
 68832| 1201: ; preds = %1195
 68833|  %1202 = gep %86, i64 1184
 68834|  %1203 = load i64, ptr %1202, , !!74828, !!8
 68835|  %1204 = gep %86, i64 1192
 68836|  %1205 = load i64, ptr %1204, , !!74828, !!8
 68837|  %1206 = load i64, ptr %135, , !!74830, !!8
 68838|  %1207 = add i64 %1206, -1
 68839|  %1208 = mul i64 %1207, %1205
 68840|  %1209 = gep %86, i64 1080
 68841|  %1210 = load i64, ptr %1209, , !!74830, !!8
 68842|  %1211 = gep %86, i64 1136
 68843|  %1212 = load i32, ptr %1211, , !!74830
 68844|  %1213 = freeze i32 %1212
 68845|  %1214 = icmp eq i32 %1213, 0
 68846|  %1215 = sext i32 %1213 to i64
 68847|  %1216 = gep %86, i64 1664
 68848|  %1217 = load i64, ptr %1216, , !!74830
 68849|  %1218 = add nsw i64 %1215, 100
 68850|  %1219 = mul i64 %1218, %1217
 68851|  %1220 = udiv i64 %1219, 100
 68852|  %1221 = add i64 %1210, %1203
 68853|  %1222 = add i64 %1221, %1208
 68854|  %1223 = load i64, ptr %86, , !!74830
 68855|  %1224 = freeze i64 %1223
 68856|  %1225 = trunc i64 %1224 to i1
 68857|  %1226 = load i64, ptr %599, , !!74830
 68858|  %1227 = freeze i64 %1226
 68859|  %1228 = load i64, ptr %604, , !!74830
 68860|  %1229 = load i64, ptr %605, , !!74830
 68861|  br i1 %1225, label %1230, label %1321
 68862| 
 68863| 1230: ; preds = %1201
 68864|  br i1 %1214, label %1231, label %1276
 68865| 
 68866| 1231: ; preds = %1230
 68867|  %1232 = add i64 %1222, %1217                                                                                          ;L180<331<1574
 68868|  br label %1233                                                                                                        ;L180<331<1574
 68869| 
 68870| 1233: ; preds = %1274, %1231
 68871|  %1234 = phi ptr [ %1196, %1231 ], [ %1235, %1274 ]
 68872|     ;; ptr = ptr %1234
 68873|  %1235 = gep %1234, i64 32                                                                                             ;L656<185<331<1574
 68874|     ;; x = ptr %1234
 68875|  %1236 = gep %1234, i64 24                                                                                             ;L332<1574
 68876|  %1237 = load ptr, ptr %1236, , !!74834, !!8, !!8                                                                      ;L332<1574
 68881|     ;; self = ptr %1237
 68882|     ;; self = ptr %1237
 68883|     ;; self = ptr %1237
 68884|  %1238 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %127, ptr %86, ptr %1237)
 68885|  to label %1239 unwind label %178                                                                                      ;L1575<332<1574
 68886| 
 68887| 1239: ; preds = %1233
 68888|     ;; self = ptr %86
 68889|  %1240 = gep %1237, i64 1136                                                                                           ;L1511<1575<332<1574
 68890|  %1241 = load i32, ptr %1240, , !!74834, !!8                                                                           ;L1511<1575<332<1574
 68891|     ;; mult = i32 %1241
 68892|  %1242 = icmp eq i32 %1241, 0                                                                                          ;L1512<1575<332<1574
 68893|  br i1 %1242, label %1250, label %1243                                                                                 ;L1512<1575<332<1574
 68894| 
 68895| 1243: ; preds = %1239
 68896|  %1244 = sext i32 %1241 to i64                                                                                         ;L1511<1575<332<1574
 68897|     ;; mult = i64 %1244
 68898|  %1245 = gep %1237, i64 1664                                                                                           ;L1515<1575<332<1574
 68899|  %1246 = load i64, ptr %1245, , !!74834, !!8                                                                           ;L1515<1575<332<1574
 68900|  %1247 = add nsw i64 %1244, 100                                                                                        ;L1515<1575<332<1574
 68901|  %1248 = mul i64 %1246, %1247                                                                                          ;L1515<1575<332<1574
 68902|  %1249 = udiv i64 %1248, 100                                                                                           ;L1515<1575<332<1574
 68903|  br label %1253                                                                                                        ;L1512<1575<332<1574
 68904| 
 68905| 1250: ; preds = %1239
 68906|  %1251 = gep %1237, i64 1664                                                                                           ;L1513<1575<332<1574
 68907|  %1252 = load i64, ptr %1251, , !!74834, !!8                                                                           ;L1513<1575<332<1574
 68908|  br label %1253                                                                                                        ;L1512<1575<332<1574
 68909| 
 68910| 1253: ; preds = %1250, %1243
 68911|  %1254 = phi i64 [ %1252, %1250 ], [ %1249, %1243 ]                                                                    ;L0<1575<332<1574
 68912|  %1255 = add i64 %1232, %1238
 68913|  %1256 = add i64 %1255, %1254                                                                                          ;L1575<332<1574
 68914|     ;; attack_range = i64 %1256
 68915|     ;; entity = ptr %86
 68916|     ;; other = ptr %86
 68917|  %1257 = gep %1237, i64 1632                                                                                           ;L2158<1576<332<1574
 68918|  %1258 = load i64, ptr %1257, , !!74834, !!8                                                                           ;L2158<1576<332<1574
 68919|     ;; x1 = i64 %1258
 68920|     ;; self = i64 %1258
 68921|  %1259 = gep %1237, i64 1640                                                                                           ;L2158<1576<332<1574
 68922|  %1260 = load i64, ptr %1259, , !!74834, !!8                                                                           ;L2158<1576<332<1574
 68923|     ;; y1 = i64 %1260
 68924|     ;; self = i64 %1260
 68925|     ;; x2 = i64 %1228
 68926|     ;; other = i64 %1228
 68927|     ;; y2 = i64 %1229
 68928|     ;; other = i64 %1229
 68929|  %1261 = icmp ult i64 %1258, %1228                                                                                     ;L3147<7<2158<1576<332<1574
 68930|  %1262 = sub nuw i64 %1228, %1258                                                                                      ;L3147<7<2158<1576<332<1574
 68931|  %1263 = sub nuw i64 %1258, %1228                                                                                      ;L3147<7<2158<1576<332<1574
 68932|  %1264 = select i1 %1261, i64 %1262, i64 %1263                                                                         ;L3147<7<2158<1576<332<1574
 68933|     ;; dx = i64 %1264
 68934|  %1265 = icmp ult i64 %1260, %1229                                                                                     ;L3147<8<2158<1576<332<1574
 68935|  %1266 = sub nuw i64 %1229, %1260                                                                                      ;L3147<8<2158<1576<332<1574
 68936|  %1267 = sub nuw i64 %1260, %1229                                                                                      ;L3147<8<2158<1576<332<1574
 68937|  %1268 = select i1 %1265, i64 %1266, i64 %1267                                                                         ;L3147<8<2158<1576<332<1574
 68938|     ;; dy = i64 %1268
 68939|  %1269 = mul i64 %1264, %1264                                                                                          ;L9<2158<1576<332<1574
 68940|  %1270 = mul i64 %1268, %1268                                                                                          ;L9<2158<1576<332<1574
 68941|  %1271 = add i64 %1270, %1269                                                                                          ;L9<2158<1576<332<1574
 68942|  %1272 = mul i64 %1256, %1256                                                                                          ;L1576<332<1574
 68943|  %1273 = icmp ugt i64 %1271, %1272                                                                                     ;L1576<332<1574
 68944|  br i1 %1273, label %1274, label %1191                                                                                 ;L332<1574
 68945| 
 68946| 1274: ; preds = %1253
 68947|     ;; ptr = ptr %1235
 68948|     ;; self = ptr %1235
 68949|     ;; end_or_len = ptr %1199
 68952|  %1275 = icmp eq ptr %1235, %1199                                                                                      ;L1714<180<331<1574
 68953|  br i1 %1275, label %1430, label %1233                                                                                 ;L180<331<1574
 68954| 
 68955| 1276: ; preds = %1230
 68956|  %1277 = add i64 %1220, %1222                                                                                          ;L180<331<1574
 68957|  br label %1278                                                                                                        ;L180<331<1574
 68958| 
 68959| 1278: ; preds = %1319, %1276
 68960|  %1279 = phi ptr [ %1196, %1276 ], [ %1280, %1319 ]
 68961|     ;; ptr = ptr %1279
 68962|  %1280 = gep %1279, i64 32                                                                                             ;L656<185<331<1574
 68963|     ;; x = ptr %1279
 68964|  %1281 = gep %1279, i64 24                                                                                             ;L332<1574
 68965|  %1282 = load ptr, ptr %1281, , !!74834, !!8, !!8                                                                      ;L332<1574
 68970|     ;; self = ptr %1282
 68971|     ;; self = ptr %1282
 68972|     ;; self = ptr %1282
 68973|  %1283 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %127, ptr %86, ptr %1282)
 68974|  to label %1284 unwind label %178                                                                                      ;L1575<332<1574
 68975| 
 68976| 1284: ; preds = %1278
 68977|     ;; self = ptr %86
 68978|  %1285 = gep %1282, i64 1136                                                                                           ;L1511<1575<332<1574
 68979|  %1286 = load i32, ptr %1285, , !!74834, !!8                                                                           ;L1511<1575<332<1574
 68980|     ;; mult = i32 %1286
 68981|  %1287 = icmp eq i32 %1286, 0                                                                                          ;L1512<1575<332<1574
 68982|  br i1 %1287, label %1295, label %1288                                                                                 ;L1512<1575<332<1574
 68983| 
 68984| 1288: ; preds = %1284
 68985|  %1289 = sext i32 %1286 to i64                                                                                         ;L1511<1575<332<1574
 68986|     ;; mult = i64 %1289
 68987|  %1290 = gep %1282, i64 1664                                                                                           ;L1515<1575<332<1574
 68988|  %1291 = load i64, ptr %1290, , !!74834, !!8                                                                           ;L1515<1575<332<1574
 68989|  %1292 = add nsw i64 %1289, 100                                                                                        ;L1515<1575<332<1574
 68990|  %1293 = mul i64 %1291, %1292                                                                                          ;L1515<1575<332<1574
 68991|  %1294 = udiv i64 %1293, 100                                                                                           ;L1515<1575<332<1574
 68992|  br label %1298                                                                                                        ;L1512<1575<332<1574
 68993| 
 68994| 1295: ; preds = %1284
 68995|  %1296 = gep %1282, i64 1664                                                                                           ;L1513<1575<332<1574
 68996|  %1297 = load i64, ptr %1296, , !!74834, !!8                                                                           ;L1513<1575<332<1574
 68997|  br label %1298                                                                                                        ;L1512<1575<332<1574
 68998| 
 68999| 1298: ; preds = %1295, %1288
 69000|  %1299 = phi i64 [ %1297, %1295 ], [ %1294, %1288 ]                                                                    ;L0<1575<332<1574
 69001|  %1300 = add i64 %1277, %1283
 69002|  %1301 = add i64 %1300, %1299                                                                                          ;L1575<332<1574
 69003|     ;; attack_range = i64 %1301
 69004|     ;; entity = ptr %86
 69005|     ;; other = ptr %86
 69006|  %1302 = gep %1282, i64 1632                                                                                           ;L2158<1576<332<1574
 69007|  %1303 = load i64, ptr %1302, , !!74834, !!8                                                                           ;L2158<1576<332<1574
 69008|     ;; x1 = i64 %1303
 69009|     ;; self = i64 %1303
 69010|  %1304 = gep %1282, i64 1640                                                                                           ;L2158<1576<332<1574
 69011|  %1305 = load i64, ptr %1304, , !!74834, !!8                                                                           ;L2158<1576<332<1574
 69012|     ;; y1 = i64 %1305
 69013|     ;; self = i64 %1305
 69014|     ;; x2 = i64 %1228
 69015|     ;; other = i64 %1228
 69016|     ;; y2 = i64 %1229
 69017|     ;; other = i64 %1229
 69018|  %1306 = icmp ult i64 %1303, %1228                                                                                     ;L3147<7<2158<1576<332<1574
 69019|  %1307 = sub nuw i64 %1228, %1303                                                                                      ;L3147<7<2158<1576<332<1574
 69020|  %1308 = sub nuw i64 %1303, %1228                                                                                      ;L3147<7<2158<1576<332<1574
 69021|  %1309 = select i1 %1306, i64 %1307, i64 %1308                                                                         ;L3147<7<2158<1576<332<1574
 69022|     ;; dx = i64 %1309
 69023|  %1310 = icmp ult i64 %1305, %1229                                                                                     ;L3147<8<2158<1576<332<1574
 69024|  %1311 = sub nuw i64 %1229, %1305                                                                                      ;L3147<8<2158<1576<332<1574
 69025|  %1312 = sub nuw i64 %1305, %1229                                                                                      ;L3147<8<2158<1576<332<1574
 69026|  %1313 = select i1 %1310, i64 %1311, i64 %1312                                                                         ;L3147<8<2158<1576<332<1574
 69027|     ;; dy = i64 %1313
 69028|  %1314 = mul i64 %1309, %1309                                                                                          ;L9<2158<1576<332<1574
 69029|  %1315 = mul i64 %1313, %1313                                                                                          ;L9<2158<1576<332<1574
 69030|  %1316 = add i64 %1315, %1314                                                                                          ;L9<2158<1576<332<1574
 69031|  %1317 = mul i64 %1301, %1301                                                                                          ;L1576<332<1574
 69032|  %1318 = icmp ugt i64 %1316, %1317                                                                                     ;L1576<332<1574
 69033|  br i1 %1318, label %1319, label %1191                                                                                 ;L332<1574
 69034| 
 69035| 1319: ; preds = %1298
 69036|     ;; ptr = ptr %1280
 69037|     ;; self = ptr %1280
 69038|     ;; end_or_len = ptr %1199
 69041|  %1320 = icmp eq ptr %1280, %1199                                                                                      ;L1714<180<331<1574
 69042|  br i1 %1320, label %1430, label %1278                                                                                 ;L180<331<1574
 69043| 
 69044| 1321: ; preds = %1201
 69045|  %1322 = icmp ult i64 %1227, 2
 69046|  br i1 %1322, label %1323, label %1424
 69047| 
 69048| 1323: ; preds = %1321
 69049|  br i1 %1214, label %1324, label %1374
 69050| 
 69051| 1324: ; preds = %1323
 69052|  %1325 = add i64 %1222, %1217                                                                                          ;L180<331<1574
 69053|  br label %1326                                                                                                        ;L180<331<1574
 69054| 
 69055| 1326: ; preds = %1372, %1324
 69056|  %1327 = phi ptr [ %1196, %1324 ], [ %1328, %1372 ]
 69057|     ;; ptr = ptr %1327
 69058|  %1328 = gep %1327, i64 32                                                                                             ;L656<185<331<1574
 69059|     ;; x = ptr %1327
 69060|  %1329 = gep %1327, i64 24                                                                                             ;L332<1574
 69061|  %1330 = load ptr, ptr %1329, , !!74834, !!8, !!8                                                                      ;L332<1574
 69066|     ;; self = ptr %1330
 69067|     ;; self = ptr %1330
 69068|     ;; self = ptr %1330
 69069|  %1331 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %127, ptr %86, ptr %1330)
 69070|  to label %1332 unwind label %178                                                                                      ;L1575<332<1574
 69071| 
 69072| 1332: ; preds = %1326
 69073|     ;; self = ptr %86
 69074|  %1333 = gep %1330, i64 1136                                                                                           ;L1511<1575<332<1574
 69075|  %1334 = load i32, ptr %1333, , !!74834, !!8                                                                           ;L1511<1575<332<1574
 69076|     ;; mult = i32 %1334
 69077|  %1335 = icmp eq i32 %1334, 0                                                                                          ;L1512<1575<332<1574
 69078|  br i1 %1335, label %1343, label %1336                                                                                 ;L1512<1575<332<1574
 69079| 
 69080| 1336: ; preds = %1332
 69081|  %1337 = sext i32 %1334 to i64                                                                                         ;L1511<1575<332<1574
 69082|     ;; mult = i64 %1337
 69083|  %1338 = gep %1330, i64 1664                                                                                           ;L1515<1575<332<1574
 69084|  %1339 = load i64, ptr %1338, , !!74834, !!8                                                                           ;L1515<1575<332<1574
 69085|  %1340 = add nsw i64 %1337, 100                                                                                        ;L1515<1575<332<1574
 69086|  %1341 = mul i64 %1339, %1340                                                                                          ;L1515<1575<332<1574
 69087|  %1342 = udiv i64 %1341, 100                                                                                           ;L1515<1575<332<1574
 69088|  br label %1346                                                                                                        ;L1512<1575<332<1574
 69089| 
 69090| 1343: ; preds = %1332
 69091|  %1344 = gep %1330, i64 1664                                                                                           ;L1513<1575<332<1574
 69092|  %1345 = load i64, ptr %1344, , !!74834, !!8                                                                           ;L1513<1575<332<1574
 69093|  br label %1346                                                                                                        ;L1512<1575<332<1574
 69094| 
 69095| 1346: ; preds = %1343, %1336
 69096|  %1347 = phi i64 [ %1345, %1343 ], [ %1342, %1336 ]                                                                    ;L0<1575<332<1574
 69097|     ;; attack_range = !DIArgList(i64 %1331, i64 %1347, i64 %1325)
 69098|     ;; entity = ptr %86
 69099|     ;; team = i64 %1226
 69101|  %1348 = gep %1330, i64 56                                                                                             ;L122<1483<1576<332<1574
 69102|  %1349 = gepS %1348, i64 %1227                                                                                         ;L122<1483<1576<332<1574
 69103|  %1350 = load i64, ptr %1349, , !!74834, !!8                                                                           ;L122<1483<1576<332<1574
 69104|  %1351 = icmp eq i64 %1350, 0                                                                                          ;L122<1483<1576<332<1574
 69105|  br i1 %1351, label %1352, label %1372                                                                                 ;L1576<332<1574
 69106| 
 69107| 1352: ; preds = %1346
 69108|  %1353 = add i64 %1325, %1331
 69109|     ;; attack_range = !DIArgList(i64 %1353, i64 %1347)
 69110|  %1354 = add i64 %1353, %1347                                                                                          ;L1575<332<1574
 69111|     ;; attack_range = i64 %1354
 69112|     ;; other = ptr %86
 69113|  %1355 = gep %1330, i64 1632                                                                                           ;L2158<1576<332<1574
 69114|  %1356 = load i64, ptr %1355, , !!74834, !!8                                                                           ;L2158<1576<332<1574
 69115|     ;; x1 = i64 %1356
 69116|     ;; self = i64 %1356
 69117|  %1357 = gep %1330, i64 1640                                                                                           ;L2158<1576<332<1574
 69118|  %1358 = load i64, ptr %1357, , !!74834, !!8                                                                           ;L2158<1576<332<1574
 69119|     ;; y1 = i64 %1358
 69120|     ;; self = i64 %1358
 69121|     ;; x2 = i64 %1228
 69122|     ;; other = i64 %1228
 69123|     ;; y2 = i64 %1229
 69124|     ;; other = i64 %1229
 69125|  %1359 = icmp ult i64 %1356, %1228                                                                                     ;L3147<7<2158<1576<332<1574
 69126|  %1360 = sub nuw i64 %1228, %1356                                                                                      ;L3147<7<2158<1576<332<1574
 69127|  %1361 = sub nuw i64 %1356, %1228                                                                                      ;L3147<7<2158<1576<332<1574
 69128|  %1362 = select i1 %1359, i64 %1360, i64 %1361                                                                         ;L3147<7<2158<1576<332<1574
 69129|     ;; dx = i64 %1362
 69130|  %1363 = icmp ult i64 %1358, %1229                                                                                     ;L3147<8<2158<1576<332<1574
 69131|  %1364 = sub nuw i64 %1229, %1358                                                                                      ;L3147<8<2158<1576<332<1574
 69132|  %1365 = sub nuw i64 %1358, %1229                                                                                      ;L3147<8<2158<1576<332<1574
 69133|  %1366 = select i1 %1363, i64 %1364, i64 %1365                                                                         ;L3147<8<2158<1576<332<1574
 69134|     ;; dy = i64 %1366
 69135|  %1367 = mul i64 %1362, %1362                                                                                          ;L9<2158<1576<332<1574
 69136|  %1368 = mul i64 %1366, %1366                                                                                          ;L9<2158<1576<332<1574
 69137|  %1369 = add i64 %1368, %1367                                                                                          ;L9<2158<1576<332<1574
 69138|  %1370 = mul i64 %1354, %1354                                                                                          ;L1576<332<1574
 69139|  %1371 = icmp ugt i64 %1369, %1370                                                                                     ;L1576<332<1574
 69140|  br i1 %1371, label %1372, label %1191                                                                                 ;L332<1574
 69141| 
 69142| 1372: ; preds = %1352, %1346
 69143|     ;; ptr = ptr %1328
 69144|     ;; self = ptr %1328
 69145|     ;; end_or_len = ptr %1199
 69148|  %1373 = icmp eq ptr %1328, %1199                                                                                      ;L1714<180<331<1574
 69149|  br i1 %1373, label %1430, label %1326                                                                                 ;L180<331<1574
 69150| 
 69151| 1374: ; preds = %1323
 69152|  %1375 = add i64 %1220, %1222                                                                                          ;L180<331<1574
 69153|  br label %1376                                                                                                        ;L180<331<1574
 69154| 
 69155| 1376: ; preds = %1422, %1374
 69156|  %1377 = phi ptr [ %1196, %1374 ], [ %1378, %1422 ]
 69157|     ;; ptr = ptr %1377
 69158|  %1378 = gep %1377, i64 32                                                                                             ;L656<185<331<1574
 69159|     ;; x = ptr %1377
 69160|  %1379 = gep %1377, i64 24                                                                                             ;L332<1574
 69161|  %1380 = load ptr, ptr %1379, , !!74834, !!8, !!8                                                                      ;L332<1574
 69166|     ;; self = ptr %1380
 69167|     ;; self = ptr %1380
 69168|     ;; self = ptr %1380
 69169|  %1381 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %127, ptr %86, ptr %1380)
 69170|  to label %1382 unwind label %178                                                                                      ;L1575<332<1574
 69171| 
 69172| 1382: ; preds = %1376
 69173|     ;; self = ptr %86
 69174|  %1383 = gep %1380, i64 1136                                                                                           ;L1511<1575<332<1574
 69175|  %1384 = load i32, ptr %1383, , !!74834, !!8                                                                           ;L1511<1575<332<1574
 69176|     ;; mult = i32 %1384
 69177|  %1385 = icmp eq i32 %1384, 0                                                                                          ;L1512<1575<332<1574
 69178|  br i1 %1385, label %1393, label %1386                                                                                 ;L1512<1575<332<1574
 69179| 
 69180| 1386: ; preds = %1382
 69181|  %1387 = sext i32 %1384 to i64                                                                                         ;L1511<1575<332<1574
 69182|     ;; mult = i64 %1387
 69183|  %1388 = gep %1380, i64 1664                                                                                           ;L1515<1575<332<1574
 69184|  %1389 = load i64, ptr %1388, , !!74834, !!8                                                                           ;L1515<1575<332<1574
 69185|  %1390 = add nsw i64 %1387, 100                                                                                        ;L1515<1575<332<1574
 69186|  %1391 = mul i64 %1389, %1390                                                                                          ;L1515<1575<332<1574
 69187|  %1392 = udiv i64 %1391, 100                                                                                           ;L1515<1575<332<1574
 69188|  br label %1396                                                                                                        ;L1512<1575<332<1574
 69189| 
 69190| 1393: ; preds = %1382
 69191|  %1394 = gep %1380, i64 1664                                                                                           ;L1513<1575<332<1574
 69192|  %1395 = load i64, ptr %1394, , !!74834, !!8                                                                           ;L1513<1575<332<1574
 69193|  br label %1396                                                                                                        ;L1512<1575<332<1574
 69194| 
 69195| 1396: ; preds = %1393, %1386
 69196|  %1397 = phi i64 [ %1395, %1393 ], [ %1392, %1386 ]                                                                    ;L0<1575<332<1574
 69197|     ;; attack_range = !DIArgList(i64 %1381, i64 %1397, i64 %1375)
 69198|     ;; entity = ptr %86
 69199|     ;; team = i64 %1226
 69201|  %1398 = gep %1380, i64 56                                                                                             ;L122<1483<1576<332<1574
 69202|  %1399 = gepS %1398, i64 %1227                                                                                         ;L122<1483<1576<332<1574
 69203|  %1400 = load i64, ptr %1399, , !!74834, !!8                                                                           ;L122<1483<1576<332<1574
 69204|  %1401 = icmp eq i64 %1400, 0                                                                                          ;L122<1483<1576<332<1574
 69205|  br i1 %1401, label %1402, label %1422                                                                                 ;L1576<332<1574
 69206| 
 69207| 1402: ; preds = %1396
 69208|  %1403 = add i64 %1375, %1381
 69209|     ;; attack_range = !DIArgList(i64 %1403, i64 %1397)
 69210|  %1404 = add i64 %1403, %1397                                                                                          ;L1575<332<1574
 69211|     ;; attack_range = i64 %1404
 69212|     ;; other = ptr %86
 69213|  %1405 = gep %1380, i64 1632                                                                                           ;L2158<1576<332<1574
 69214|  %1406 = load i64, ptr %1405, , !!74834, !!8                                                                           ;L2158<1576<332<1574
 69215|     ;; x1 = i64 %1406
 69216|     ;; self = i64 %1406
 69217|  %1407 = gep %1380, i64 1640                                                                                           ;L2158<1576<332<1574
 69218|  %1408 = load i64, ptr %1407, , !!74834, !!8                                                                           ;L2158<1576<332<1574
 69219|     ;; y1 = i64 %1408
 69220|     ;; self = i64 %1408
 69221|     ;; x2 = i64 %1228
 69222|     ;; other = i64 %1228
 69223|     ;; y2 = i64 %1229
 69224|     ;; other = i64 %1229
 69225|  %1409 = icmp ult i64 %1406, %1228                                                                                     ;L3147<7<2158<1576<332<1574
 69226|  %1410 = sub nuw i64 %1228, %1406                                                                                      ;L3147<7<2158<1576<332<1574
 69227|  %1411 = sub nuw i64 %1406, %1228                                                                                      ;L3147<7<2158<1576<332<1574
 69228|  %1412 = select i1 %1409, i64 %1410, i64 %1411                                                                         ;L3147<7<2158<1576<332<1574
 69229|     ;; dx = i64 %1412
 69230|  %1413 = icmp ult i64 %1408, %1229                                                                                     ;L3147<8<2158<1576<332<1574
 69231|  %1414 = sub nuw i64 %1229, %1408                                                                                      ;L3147<8<2158<1576<332<1574
 69232|  %1415 = sub nuw i64 %1408, %1229                                                                                      ;L3147<8<2158<1576<332<1574
 69233|  %1416 = select i1 %1413, i64 %1414, i64 %1415                                                                         ;L3147<8<2158<1576<332<1574
 69234|     ;; dy = i64 %1416
 69235|  %1417 = mul i64 %1412, %1412                                                                                          ;L9<2158<1576<332<1574
 69236|  %1418 = mul i64 %1416, %1416                                                                                          ;L9<2158<1576<332<1574
 69237|  %1419 = add i64 %1418, %1417                                                                                          ;L9<2158<1576<332<1574
 69238|  %1420 = mul i64 %1404, %1404                                                                                          ;L1576<332<1574
 69239|  %1421 = icmp ugt i64 %1419, %1420                                                                                     ;L1576<332<1574
 69240|  br i1 %1421, label %1422, label %1191                                                                                 ;L332<1574
 69241| 
 69242| 1422: ; preds = %1402, %1396
 69243|     ;; ptr = ptr %1378
 69244|     ;; self = ptr %1378
 69245|     ;; end_or_len = ptr %1199
 69248|  %1423 = icmp eq ptr %1378, %1199                                                                                      ;L1714<180<331<1574
 69249|  br i1 %1423, label %1430, label %1376                                                                                 ;L180<331<1574
 69250| 
 69251| 1424: ; preds = %1321
 69252|     ;; ptr = ptr %1196
 69253|     ;; x = ptr %1196
 69254|  %1425 = gep %1196, i64 24                                                                                             ;L332<1574
 69255|  %1426 = load ptr, ptr %1425, , !!74913, !!8, !!8                                                                      ;L332<1574
 69260|     ;; self = ptr %1426
 69261|     ;; self = ptr %1426
 69262|     ;; self = ptr %1426
 69263|  %1427 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %127, ptr %86, ptr %1426)
 69264|  to label %1428 unwind label %178                                                                                      ;L1575<332<1574
 69265| 
 69266| 1428: ; preds = %1424
 69267|     ;; self = ptr %86
 69270|     ;; entity = ptr %86
 69271|     ;; team = i64 %1226
 69272|  invoke void @core::panicking18panic_bounds_check(i64 %1227, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 69273|  to label %1429 unwind label %178                                                                                      ;L1483<1576<332<1574
 69274| 
 69275| 1429: ; preds = %1428
 69276|  unreachable                                                                                                           ;L1483<1576<332<1574
 69277| 
 69278| 1430: ; preds = %1422, %1372, %1319, %1274, %1195, %1190
 69279|     ;; skip_self_buff = i8 1
 69281|  br label %592                                                                                                         ;L1586
 69282| 
 69283| 1431: ; preds = %1191
 69284|  br i1 %1194, label %1432, label %592                                                                                  ;L1586
 69285| 
 69286| 1432: ; preds = %1431
 69289|  %1433 = gep %86, i64 1472                                                                                             ;L1587
 69290|  %1434 = load i64, ptr %1433, , !!8                                                                                    ;L1587
 69291|  invoke void @ai::small_action4castNtB5_16SmallActionSkill3new(ptr sret([24 x i8]) %46, ptr %3, i64 %1434)
 69292|  to label %1435 unwind label %178                                                                                      ;L1587
 69293| 
 69294| 1435: ; preds = %1432
 69295|  call void @llvm.memcpy.p0.p0.i64(ptr %47, ptr %46, i64 24, i1 false)                                                  ;L1587
 69296|  %1436 = gep %47, i64 177                                                                                              ;L1587
 69297|  store i8 16, ptr %1436,                                                                                               ;L1587
 69300|     ;; self = ptr %65
 69301|     ;; self = ptr %65
 69302|     ;; value = ptr %47
 69303|     ;; src = ptr %47
 69304|     ;; additional = i64 1
 69305|     ;; needed_extra_cap = i64 1
 69306|     ;; needed_extra_cap = i64 1
 69307|     ;; strategy = i8 1
 69308|  %1437 = load i64, ptr %126, , !!74931, !!8                                                                            ;L1428<1587
 69309|     ;; self = ptr %65
 69310|  %1438 = load i64, ptr %125, , !!74931, !!8                                                                            ;L149<1428<1587
 69311|  %1439 = icmp eq i64 %1437, %1438                                                                                      ;L1428<1587
 69312|  br i1 %1439, label %1440, label %1445                                                                                 ;L1428<1587
 69313| 
 69314| 1440: ; preds = %1435
 69315|     ;; self = ptr %65
 69316|     ;; self = ptr %65
 69317|     ;; self = ptr %65
 69318|     ;; used_cap = i64 %1437
 69319|     ;; used_cap = i64 %1437
 69320|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %1437, i64 1, i1 zeroext true)
 69321|  to label %1441 unwind label %1443, !!74931                                                                            ;L619<430<738<1429<1587
 69322| 
 69323| 1441: ; preds = %1440
 69324|  %1442 = load i64, ptr %126, , !!74931                                                                                 ;L1432<1587
 69325|  br label %1445                                                                                                        ;L619<430<738<1429<1587
 69326| 
 69327| 1443: ; preds = %1440
 69328|  %1444 = cleanuppad within none []
 69329|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %47) #30 [ "funclet"(token %1444) ], !!74916 ;L1436<1587
 69330|  cleanupret from %1444 unwind label %178
 69331| 
 69332| 1445: ; preds = %1441, %1435
 69333|  %1446 = phi i64 [ %1442, %1441 ], [ %1437, %1435 ]                                                                    ;L1432<1587
 69334|     ;; self = ptr %65
 69335|  %1447 = load ptr, ptr %65, , !!74931, !!8, !!8                                                                        ;L138<1432<1587
 69336|     ;; self = ptr %1447
 69337|     ;; count = i64 %1446
 69338|  %1448 = gepS %1447, i64 %1446                                                                                         ;L961<1432<1587
 69339|     ;; end = ptr %1448
 69340|     ;; dst = ptr %1448
 69341|  call void @llvm.memcpy.p0.p0.i64(ptr %1448, ptr %47, i64 184, i1 false), !!74916                                      ;L1933<1433<1587
 69342|  %1449 = add i64 %1446, 1                                                                                              ;L1434<1587
 69343|  store i64 %1449, ptr %126, , !!74931                                                                                  ;L1434<1587
 69345|  br label %592                                                                                                         ;L1586
 69346| 
 69347| 1450: ; preds = %592
 69348|  %1451 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_skill2(ptr %86)
 69349|  to label %1468 unwind label %178                                                                                      ;L1592
 69350| 
 69351| 1452: ; preds = %2131, %2117, %2114, %2046, %2037, %1468, %592
 69352|     ;; self = ptr %82
 69353|     ;; self = ptr %82
 69354|  %1453 = gep %82, i64 208                                                                                              ;L138<2073<1761
 69355|  %1454 = load ptr, ptr %1453, , !!8, !!8                                                                               ;L138<2073<1761
 69356|     ;; p = ptr %1454
 69357|  %1455 = gep %82, i64 232                                                                                              ;L2075<1761
 69358|  %1456 = load i64, ptr %1455, , !!8                                                                                    ;L2075<1761
 69359|     ;; len = i64 %1456
 69360|     ;; count = i64 %1456
 69361|     ;; count = i64 %1456
 69362|     ;; self[0..+8] = ptr %1454
 69363|     ;; slice[0..+8] = ptr %1454
 69364|     ;; self[0..+8] = ptr %1454
 69365|     ;; slice[0..+8] = ptr %1454
 69366|     ;; self[8..+8] = i64 %1456
 69367|     ;; slice[8..+8] = i64 %1456
 69368|     ;; self[8..+8] = i64 %1456
 69369|     ;; slice[8..+8] = i64 %1456
 69370|     ;; ptr = ptr %1454
 69371|     ;; self = ptr %1454
 69372|  %1457 = getelementptr ptr, ptr %1454, i64 %1456                                                                       ;L961<100<1042<1761
 69373|     ;; iter[0..+8] = ptr %1454
 69374|     ;; iter[8..+8] = ptr %1457
 69375|  %1458 = gep %86, i64 8
 69376|  %1459 = gep %86, i64 1632
 69377|  %1460 = gep %86, i64 1640
 69378|  %1461 = mul i64 %239, %97
 69379|  %1462 = add i64 %1461, %177
 69380|  %1463 = gep %35, i64 177
 69381|  %1464 = gep %86, i64 1264
 69382|  %1465 = gep %33, i64 177
 69383|  %1466 = gep %139, i64 40
 69384|  %1467 = gep %31, i64 177
 69385|  br label %2136                                                                                                        ;L1761
 69386| 
 69387| 1468: ; preds = %1450
 69388|  br i1 %1451, label %1469, label %1452                                                                                 ;L1592
 69389| 
 69390| 1469: ; preds = %1468
 69391|     ;; self = ptr %70
 69392|     ;; self = ptr %70
 69393|     ;; self = ptr %70
 69394|  %1470 = load ptr, ptr %70, , !!8, !!8                                                                                 ;L138<2073<2136<1593
 69395|     ;; p = ptr %1470
 69396|  %1471 = gep %70, i64 24                                                                                               ;L2075<2136<1593
 69397|  %1472 = load i64, ptr %1471, , !!8                                                                                    ;L2075<2136<1593
 69398|     ;; len = i64 %1472
 69399|     ;; count = i64 %1472
 69400|     ;; self[0..+8] = ptr %1470
 69401|     ;; slice[0..+8] = ptr %1470
 69402|     ;; self[8..+8] = i64 %1472
 69403|     ;; slice[8..+8] = i64 %1472
 69404|     ;; ptr = ptr %1470
 69405|     ;; self = ptr %1470
 69406|  %1473 = gepS }, ptr %1470, i64 %1472                                                                                  ;L961<100<1042<2136<1593
 69407|     ;; iter[0..+8] = ptr %1470
 69408|     ;; iter[8..+8] = ptr %1473
 69409|  %1474 = gep %86, i64 8
 69410|  %1475 = gep %139, i64 40
 69411|  %1476 = select i1 %137, i64 1424, i64 1456
 69412|  %1477 = gep %86, i64 %1476
 69413|  %1478 = gep %1477, i64 8
 69414|  %1479 = gep %82, i64 8
 69415|  %1480 = gep %86, i64 1632
 69416|  %1481 = gep %86, i64 1640
 69417|  %1482 = gep %139, i64 8
 69418|  %1483 = gep %139, i64 8
 69419|  %1484 = gep %44, i64 177
 69420|  br label %1485                                                                                                        ;L1593
 69421| 
 69422| 1485: ; preds = %1654, %1469
 69423|  %1486 = phi ptr [ %1470, %1469 ], [ %1489, %1654 ]                                                                    ;L1593
 69424|     ;; iter[0..+8] = ptr %1486
 69425|     ;; self = ptr undef
 69426|     ;; ptr = ptr %1486
 69427|     ;; self = ptr %1486
 69428|     ;; end_or_len = ptr %1473
 69431|  %1487 = icmp eq ptr %1486, %1473                                                                                      ;L1714<180<1593
 69432|  br i1 %1487, label %1494, label %1488                                                                                 ;L180<1593
 69433| 
 69434| 1488: ; preds = %1485
 69435|  %1489 = gep %1486, i64 32                                                                                             ;L656<185<1593
 69436|     ;; iter[0..+8] = ptr %1489
 69437|     ;; a = ptr %1486
 69438|     ;; e = ptr %1486
 69439|  %1490 = gep %1486, i64 24                                                                                             ;L1594
 69440|  %1491 = load ptr, ptr %1490, , !!8, !!8                                                                               ;L1594
 69441|     ;; self = ptr %1491
 69442|     ;; self = ptr %1491
 69443|     ;; self = ptr %1491
 69444|     ;; self = ptr %86
 69445|  %1492 = load i64, ptr %86, , !!8                                                                                      ;L1136<1482<1594
 69446|  %1493 = trunc nuw i64 %1492 to i1                                                                                     ;L1136<1482<1594
 69447|  br i1 %1493, label %1516, label %1507                                                                                 ;L1136<1482<1594
 69448| 
 69449| 1494: ; preds = %1485
 69450|     ;; self = ptr %72
 69451|     ;; self = ptr %72
 69452|     ;; self = ptr %72
 69453|  %1495 = load ptr, ptr %72, , !!8, !!8                                                                                 ;L138<2073<2136<1650
 69454|     ;; p = ptr %1495
 69455|  %1496 = gep %72, i64 24                                                                                               ;L2075<2136<1650
 69456|  %1497 = load i64, ptr %1496, , !!8                                                                                    ;L2075<2136<1650
 69457|     ;; len = i64 %1497
 69458|     ;; count = i64 %1497
 69459|     ;; self[0..+8] = ptr %1495
 69460|     ;; slice[0..+8] = ptr %1495
 69461|     ;; self[8..+8] = i64 %1497
 69462|     ;; slice[8..+8] = i64 %1497
 69463|     ;; ptr = ptr %1495
 69464|     ;; self = ptr %1495
 69465|  %1498 = getelementptr ptr, ptr %1495, i64 %1497                                                                       ;L961<100<1042<2136<1650
 69466|     ;; iter[0..+8] = ptr %1495
 69467|     ;; iter[8..+8] = ptr %1498
 69468|  %1499 = gep %42, i64 72
 69469|  %1500 = gep %82, i64 240
 69470|  %1501 = getelementptr { { ptr, ptr, i64 }, i64 }, ptr %1500, i64 %104
 69471|  %1502 = gep %1501, i64 24
 69472|  %1503 = gep %42, i64 136
 69473|  %1504 = mul i64 %239, %97
 69474|  %1505 = add i64 %1504, %237
 69475|  %1506 = gep %41, i64 177
 69476|  br label %1655                                                                                                        ;L1650
 69477| 
 69478| 1507: ; preds = %1488
 69479|     ;; team = ptr %86
 69480|  %1508 = load i64, ptr %1474, , !!8                                                                                    ;L1137<1482<1594
 69481|     ;; team = i64 %1508
 69482|  %1509 = icmp ult i64 %1508, 2                                                                                         ;L1483<1594
 69483|  br i1 %1509, label %1510, label %1515                                                                                 ;L1483<1594
 69484| 
 69485| 1510: ; preds = %1507
 69487|  %1511 = gep %1491, i64 56                                                                                             ;L122<1483<1594
 69488|  %1512 = gepS %1511, i64 %1508                                                                                         ;L122<1483<1594
 69489|  %1513 = load i64, ptr %1512, , !!8                                                                                    ;L122<1483<1594
 69490|  %1514 = icmp eq i64 %1513, 0                                                                                          ;L122<1483<1594
 69491|  br i1 %1514, label %1516, label %1654                                                                                 ;L1594
 69492| 
 69493| 1515: ; preds = %1507
 69494|  invoke void @core::panicking18panic_bounds_check(i64 %1508, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 69495|  to label %118 unwind label %178                                                                                       ;L1483<1594
 69496| 
 69497| 1516: ; preds = %1510, %1488
 69498|  %1517 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %1475, ptr %86, ptr %1491)
 69499|  to label %1518 unwind label %178                                                                                      ;L1598
 69500| 
 69501| 1518: ; preds = %1516
 69502|  br i1 %1517, label %1519, label %1654                                                                                 ;L1598
 69503| 
 69504| 1519: ; preds = %1518
 69505|  %1520 = load ptr, ptr %1477, , !!8, !!8                                                                               ;L1602
 69506|  %1521 = load ptr, ptr %1478, , !!8, !!8                                                                               ;L1602
 69507|  %1522 = load ptr, ptr %82, , !!8, !!8                                                                                 ;L1602
 69508|  %1523 = load ptr, ptr %1479, , !!8, !!8                                                                               ;L1602
 69509|  %1524 = gep %1521, i64 200                                                                                            ;L1602
 69510|  %1525 = load ptr, ptr %1524, , !!8                                                                                    ;L1602
 69511|  %1526 = invoke zeroext i1 %1525(ptr %1520, ptr %1522, ptr %1523, ptr %86, ptr %1491)
 69512|  to label %1527 unwind label %178                                                                                      ;L1602
 69513| 
 69514| 1527: ; preds = %1519
 69515|  br i1 %1526, label %1528, label %1654                                                                                 ;L1602
 69516| 
 69517| 1528: ; preds = %1527
 69518|  %1529 = load i64, ptr %1486, , !!8                                                                                    ;L1606
 69521|     ;; __self_discr = i64 %1529
 69522|     ;; __arg1_discr = i64 0
 69523|  %1530 = icmp eq i64 %1529, 0                                                                                          ;L81<1606
 69524|  br i1 %1530, label %1531, label %1538                                                                                 ;L1606
 69525| 
 69526| 1531: ; preds = %1528
 69527|  %1532 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10block_move(ptr %1491)
 69528|  to label %1533 unwind label %178                                                                                      ;L1606
 69529| 
 69530| 1533: ; preds = %1531
 69531|  br i1 %1532, label %1538, label %1534                                                                                 ;L1606
 69532| 
 69533| 1534: ; preds = %1533
 69534|  %1535 = gep %1491, i64 1600                                                                                           ;L1607
 69535|  %1536 = load i64, ptr %1535, , !!8                                                                                    ;L1607
 69536|     ;; rhs = i64 %1536
 69537|  %1537 = call i64 @llvm.usub.sat.i64(i64 %239, i64 %1536)                                                              ;L2472<1607
 69538|     ;; move_speed = i64 %1537
 69539|  br label %1538                                                                                                        ;L1606
 69540| 
 69541| 1538: ; preds = %1534, %1533, %1528
 69542|  %1539 = phi i64 [ %1537, %1534 ], [ %239, %1533 ], [ %239, %1528 ]                                                    ;L0
 69543|     ;; move_speed = i64 %1539
 69544|  %1540 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %139, ptr %86, ptr %1491)
 69545|  to label %1541 unwind label %178                                                                                      ;L1612
 69546| 
 69547| 1541: ; preds = %1538
 69548|  %1542 = add i64 %1540, %237                                                                                           ;L1612
 69549|  %1543 = gep %1491, i64 1136                                                                                           ;L1511<1612
 69550|  %1544 = load i32, ptr %1543, , !!8                                                                                    ;L1511<1612
 69551|     ;; mult = i32 %1544
 69552|  %1545 = icmp eq i32 %1544, 0                                                                                          ;L1512<1612
 69553|  br i1 %1545, label %1546, label %1549                                                                                 ;L1512<1612
 69554| 
 69555| 1546: ; preds = %1541
 69556|  %1547 = gep %1491, i64 1664                                                                                           ;L1513<1612
 69557|  %1548 = load i64, ptr %1547, , !!8                                                                                    ;L1513<1612
 69558|  br label %1556                                                                                                        ;L1512<1612
 69559| 
 69560| 1549: ; preds = %1541
 69561|  %1550 = sext i32 %1544 to i64                                                                                         ;L1511<1612
 69562|     ;; mult = i64 %1550
 69563|  %1551 = gep %1491, i64 1664                                                                                           ;L1515<1612
 69564|  %1552 = load i64, ptr %1551, , !!8                                                                                    ;L1515<1612
 69565|  %1553 = add nsw i64 %1550, 100                                                                                        ;L1515<1612
 69566|  %1554 = mul i64 %1552, %1553                                                                                          ;L1515<1612
 69567|  %1555 = udiv i64 %1554, 100                                                                                           ;L1515<1612
 69568|  br label %1556                                                                                                        ;L1512<1612
 69569| 
 69570| 1556: ; preds = %1549, %1546
 69571|  %1557 = phi i64 [ %1548, %1546 ], [ %1555, %1549 ]                                                                    ;L0<1612
 69573|  %1558 = gep %1491, i64 1632                                                                                           ;L2158<1613
 69574|  %1559 = load i64, ptr %1558, , !!8                                                                                    ;L2158<1613
 69575|     ;; x1 = i64 %1559
 69576|     ;; self = i64 %1559
 69577|  %1560 = gep %1491, i64 1640                                                                                           ;L2158<1613
 69578|  %1561 = load i64, ptr %1560, , !!8                                                                                    ;L2158<1613
 69579|     ;; y1 = i64 %1561
 69580|     ;; self = i64 %1561
 69581|  %1562 = load i64, ptr %1480, , !!8                                                                                    ;L2158<1613
 69582|     ;; x2 = i64 %1562
 69583|     ;; other = i64 %1562
 69584|  %1563 = load i64, ptr %1481, , !!8                                                                                    ;L2158<1613
 69585|     ;; y2 = i64 %1563
 69586|     ;; other = i64 %1563
 69587|  %1564 = icmp ult i64 %1559, %1562                                                                                     ;L3147<7<2158<1613
 69588|  %1565 = sub nuw i64 %1562, %1559                                                                                      ;L3147<7<2158<1613
 69589|  %1566 = sub nuw i64 %1559, %1562                                                                                      ;L3147<7<2158<1613
 69590|  %1567 = select i1 %1564, i64 %1565, i64 %1566                                                                         ;L3147<7<2158<1613
 69591|     ;; dx = i64 %1567
 69592|  %1568 = icmp ult i64 %1561, %1563                                                                                     ;L3147<8<2158<1613
 69593|  %1569 = sub nuw i64 %1563, %1561                                                                                      ;L3147<8<2158<1613
 69594|  %1570 = sub nuw i64 %1561, %1563                                                                                      ;L3147<8<2158<1613
 69595|  %1571 = select i1 %1568, i64 %1569, i64 %1570                                                                         ;L3147<8<2158<1613
 69596|     ;; dy = i64 %1571
 69597|  %1572 = mul i64 %1567, %1567                                                                                          ;L9<2158<1613
 69598|  %1573 = mul i64 %1571, %1571                                                                                          ;L9<2158<1613
 69599|  %1574 = add i64 %1573, %1572                                                                                          ;L9<2158<1613
 69600|     ;; dist_sq = i64 %1574
 69601|     ;; max_tick = i64 %97
 69602|  %1575 = mul i64 %1539, %97                                                                                            ;L1622
 69603|  %1576 = add i64 %1542, %1575                                                                                          ;L1612
 69604|  %1577 = add i64 %1576, %1557                                                                                          ;L1622
 69605|     ;; max_dist = i64 %1577
 69606|  %1578 = mul i64 %1577, %1577                                                                                          ;L1623
 69607|  %1579 = icmp ugt i64 %1574, %1578                                                                                     ;L1623
 69608|  br i1 %1579, label %1654, label %1580                                                                                 ;L1623
 69609| 
 69610| 1580: ; preds = %1556
 69611|  switch i64 %4, label %1614 [
 69612|  i64 3, label %1581
 69613|  i64 4, label %1581
 69614|  i64 0, label %1608
 69615|  i64 5, label %1608
 69616|  ]                                                                                                                     ;L1627
 69617| 
 69618| 1581: ; preds = %1580, %1580
 69621|  %1582 = load ptr, ptr %139, , !!8, !!8                                                                                ;L441<2127<2445<1629
 69622|  %1583 = load ptr, ptr %1482, , !!8, !!8                                                                               ;L441<2127<2445<1629
 69623|  %1584 = gep %1583, i64 16                                                                                             ;L2445<1629
 69624|  %1585 = load i64, ptr %1584,                                                                                          ;L2445<1629
 69625|  %1586 = add nsw i64 %1585, -1                                                                                         ;L2445<1629
 69626|  %1587 = and i64 %1586, -16                                                                                            ;L2445<1629
 69627|  %1588 = gep %1582, i64 %1587                                                                                          ;L2445<1629
 69628|  %1589 = gep %1588, i64 16                                                                                             ;L2445<1629
 69629|  %1590 = gep %1583, i64 104                                                                                            ;L1629
 69630|  %1591 = load ptr, ptr %1590, , !!8                                                                                    ;L1629
 69631|  %1592 = invoke zeroext i1 %1591(ptr %1589)
 69632|  to label %1593 unwind label %178                                                                                      ;L1629
 69633| 
 69634| 1593: ; preds = %1581
 69635|  br i1 %1592, label %1654, label %1594                                                                                 ;L1629
 69636| 
 69637| 1594: ; preds = %1593
 69641|  %1595 = load ptr, ptr %139, , !!8, !!8                                                                                ;L441<2127<2445<1629
 69642|  %1596 = load ptr, ptr %1482, , !!8, !!8                                                                               ;L441<2127<2445<1629
 69643|  %1597 = gep %1596, i64 16                                                                                             ;L2445<1629
 69644|  %1598 = load i64, ptr %1597,                                                                                          ;L2445<1629
 69645|  %1599 = add nsw i64 %1598, -1                                                                                         ;L2445<1629
 69646|  %1600 = and i64 %1599, -16                                                                                            ;L2445<1629
 69647|  %1601 = gep %1595, i64 %1600                                                                                          ;L2445<1629
 69648|  %1602 = gep %1601, i64 16                                                                                             ;L2445<1629
 69649|  %1603 = gep %1596, i64 88                                                                                             ;L1629
 69650|  %1604 = load ptr, ptr %1603, , !!8                                                                                    ;L1629
 69651|  invoke void %1604(ptr sret([24 x i8]) %45, ptr %1602)
 69652|  to label %1605 unwind label %178                                                                                      ;L1629
 69653| 
 69654| 1605: ; preds = %1594
 69655|     ;; self = ptr %45
 69656|  %1606 = load i64, ptr %45, , !!8                                                                                      ;L633<1629
 69657|  %1607 = icmp eq i64 %1606, 0                                                                                          ;L1629
 69659|  br i1 %1607, label %1614, label %1654                                                                                 ;L1629
 69660| 
 69661| 1608: ; preds = %1635, %1627, %1626, %1580, %1580
 69662|  %1609 = load ptr, ptr %1477, , !!8, !!8                                                                               ;L1643
 69663|  %1610 = load ptr, ptr %1478, , !!8, !!8                                                                               ;L1643
 69664|  %1611 = gep %1610, i64 192                                                                                            ;L1643
 69665|  %1612 = load ptr, ptr %1611, , !!8                                                                                    ;L1643
 69666|  %1613 = invoke zeroext i1 %1612(ptr %1609, ptr %1522, ptr %1523, ptr %86)
 69667|  to label %1636 unwind label %178                                                                                      ;L1643
 69668| 
 69669| 1614: ; preds = %1605, %1580
 69672|  %1615 = load ptr, ptr %139, , !!8, !!8                                                                                ;L441<2127<2445<1636
 69673|  %1616 = load ptr, ptr %1483, , !!8, !!8                                                                               ;L441<2127<2445<1636
 69674|  %1617 = gep %1616, i64 16                                                                                             ;L2445<1636
 69675|  %1618 = load i64, ptr %1617,                                                                                          ;L2445<1636
 69676|  %1619 = add nsw i64 %1618, -1                                                                                         ;L2445<1636
 69677|  %1620 = and i64 %1619, -16                                                                                            ;L2445<1636
 69678|  %1621 = gep %1615, i64 %1620                                                                                          ;L2445<1636
 69679|  %1622 = gep %1621, i64 16                                                                                             ;L2445<1636
 69680|  %1623 = gep %1616, i64 288                                                                                            ;L1636
 69681|  %1624 = load ptr, ptr %1623, , !!8                                                                                    ;L1636
 69682|  %1625 = invoke zeroext i1 %1624(ptr %1622)
 69683|  to label %1626 unwind label %178                                                                                      ;L1636
 69684| 
 69685| 1626: ; preds = %1614
 69686|  br i1 %1625, label %1627, label %1608                                                                                 ;L1636
 69687| 
 69688| 1627: ; preds = %1626
 69689|     ;; self = ptr %1491
 69690|  %1628 = gep %1491, i64 104                                                                                            ;L1404<1636
 69691|  %1629 = load i64, ptr %1628, , !!8                                                                                    ;L1404<1636
 69692|  %1630 = icmp eq i64 %1629, 13                                                                                         ;L1636
 69693|  br i1 %1630, label %1631, label %1608                                                                                 ;L1636
 69694| 
 69695| 1631: ; preds = %1627
 69696|  %1632 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %139, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %1491)
 69697|  to label %1633 unwind label %178                                                                                      ;L1637
 69698| 
 69699| 1633: ; preds = %1631
 69700|     ;; skill_damage = i64 %1632
 69701|  %1634 = invoke zeroext i1 @ai::utils13is_dash_worth(ptr %3, ptr %2, ptr %86, ptr %1491, i64 %1632)
 69702|  to label %1635 unwind label %178                                                                                      ;L1638
 69703| 
 69704| 1635: ; preds = %1633
 69705|  br i1 %1634, label %1608, label %1654                                                                                 ;L1638
 69706| 
 69707| 1636: ; preds = %1608
 69708|  br i1 %1613, label %1637, label %1654                                                                                 ;L1643
 69709| 
 69710| 1637: ; preds = %1636
 69713|  %1638 = gep %1491, i64 1472                                                                                           ;L1647
 69714|  %1639 = load i64, ptr %1638, , !!8                                                                                    ;L1647
 69715|  invoke void @ai::small_action4castNtB5_17SmallActionSkill23new(ptr sret([24 x i8]) %43, ptr %3, i64 %1639)
 69716|  to label %1640 unwind label %178                                                                                      ;L1647
 69717| 
 69718| 1640: ; preds = %1637
 69719|  call void @llvm.memcpy.p0.p0.i64(ptr %44, ptr %43, i64 24, i1 false)                                                  ;L1647
 69720|  store i8 17, ptr %1484,                                                                                               ;L1647
 69723|     ;; self = ptr %65
 69724|     ;; self = ptr %65
 69725|     ;; value = ptr %44
 69726|     ;; src = ptr %44
 69727|     ;; additional = i64 1
 69728|     ;; needed_extra_cap = i64 1
 69729|     ;; needed_extra_cap = i64 1
 69730|     ;; strategy = i8 1
 69731|  %1641 = load i64, ptr %126, , !!75130, !!8                                                                            ;L1428<1647
 69732|     ;; self = ptr %65
 69733|  %1642 = load i64, ptr %125, , !!75130, !!8                                                                            ;L149<1428<1647
 69734|  %1643 = icmp eq i64 %1641, %1642                                                                                      ;L1428<1647
 69735|  br i1 %1643, label %1644, label %1649                                                                                 ;L1428<1647
 69736| 
 69737| 1644: ; preds = %1640
 69738|     ;; self = ptr %65
 69739|     ;; self = ptr %65
 69740|     ;; self = ptr %65
 69741|     ;; used_cap = i64 %1641
 69742|     ;; used_cap = i64 %1641
 69743|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %1641, i64 1, i1 zeroext true)
 69744|  to label %1645 unwind label %1647, !!75130                                                                            ;L619<430<738<1429<1647
 69745| 
 69746| 1645: ; preds = %1644
 69747|  %1646 = load i64, ptr %126, , !!75130                                                                                 ;L1432<1647
 69748|  br label %1649                                                                                                        ;L619<430<738<1429<1647
 69749| 
 69750| 1647: ; preds = %1644
 69751|  %1648 = cleanuppad within none []
 69752|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %44) #30 [ "funclet"(token %1648) ], !!75115 ;L1436<1647
 69753|  cleanupret from %1648 unwind label %178
 69754| 
 69755| 1649: ; preds = %1645, %1640
 69756|  %1650 = phi i64 [ %1646, %1645 ], [ %1641, %1640 ]                                                                    ;L1432<1647
 69757|     ;; self = ptr %65
 69758|  %1651 = load ptr, ptr %65, , !!75130, !!8, !!8                                                                        ;L138<1432<1647
 69759|     ;; self = ptr %1651
 69760|     ;; count = i64 %1650
 69761|  %1652 = gepS %1651, i64 %1650                                                                                         ;L961<1432<1647
 69762|     ;; end = ptr %1652
 69763|     ;; dst = ptr %1652
 69764|  call void @llvm.memcpy.p0.p0.i64(ptr %1652, ptr %44, i64 184, i1 false), !!75115                                      ;L1933<1433<1647
 69765|  %1653 = add i64 %1650, 1                                                                                              ;L1434<1647
 69766|  store i64 %1653, ptr %126, , !!75130                                                                                  ;L1434<1647
 69768|  br label %1654                                                                                                        ;L1593
 69769| 
 69770| 1654: ; preds = %1649, %1636, %1635, %1605, %1593, %1556, %1527, %1518, %1510
 69771|  br label %1485                                                                                                        ;L1714<180<1593
 69772| 
 69773| 1655: ; preds = %2034, %1494
 69774|  %1656 = phi ptr [ %1495, %1494 ], [ %1659, %2034 ]                                                                    ;L1650
 69775|     ;; iter[0..+8] = ptr %1656
 69776|     ;; self = ptr undef
 69777|     ;; ptr = ptr %1656
 69778|     ;; self = ptr %1656
 69779|     ;; end_or_len = ptr %1498
 69782|  %1657 = icmp eq ptr %1656, %1498                                                                                      ;L1714<180<1650
 69783|  br i1 %1657, label %2035, label %1658                                                                                 ;L180<1650
 69784| 
 69785| 1658: ; preds = %1655
 69786|  %1659 = gep %1656, i64 8                                                                                              ;L656<185<1650
 69787|     ;; iter[0..+8] = ptr %1659
 69788|     ;; e = ptr %1656
 69789|  %1660 = load ptr, ptr %1656, , !!8, !!8                                                                               ;L1651
 69790|  %1661 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %1475, ptr %86, ptr %1660)
 69791|  to label %1662 unwind label %178                                                                                      ;L1651
 69792| 
 69793| 1662: ; preds = %1658
 69794|  br i1 %1661, label %1663, label %2034                                                                                 ;L1651
 69795| 
 69796| 1663: ; preds = %1662
 69797|  %1664 = load ptr, ptr %1477, , !!8, !!8                                                                               ;L1655
 69798|  %1665 = load ptr, ptr %1478, , !!8, !!8                                                                               ;L1655
 69799|  %1666 = load ptr, ptr %82, , !!8, !!8                                                                                 ;L1655
 69800|  %1667 = load ptr, ptr %1479, , !!8, !!8                                                                               ;L1655
 69801|  %1668 = load ptr, ptr %1656, , !!8, !!8                                                                               ;L1655
 69802|  %1669 = gep %1665, i64 200                                                                                            ;L1655
 69803|  %1670 = load ptr, ptr %1669, , !!8                                                                                    ;L1655
 69804|  %1671 = invoke zeroext i1 %1670(ptr %1664, ptr %1666, ptr %1667, ptr %86, ptr %1668)
 69805|  to label %1672 unwind label %178                                                                                      ;L1655
 69806| 
 69807| 1672: ; preds = %1663
 69808|  br i1 %1671, label %1673, label %2034                                                                                 ;L1655
 69809| 
 69810| 1673: ; preds = %1672
 69813|  %1674 = load ptr, ptr %139, , !!8, !!8                                                                                ;L441<2127<2445<1660
 69814|  %1675 = load ptr, ptr %1482, , !!8, !!8                                                                               ;L441<2127<2445<1660
 69815|  %1676 = gep %1675, i64 16                                                                                             ;L2445<1660
 69816|  %1677 = load i64, ptr %1676,                                                                                          ;L2445<1660
 69817|  %1678 = add nsw i64 %1677, -1                                                                                         ;L2445<1660
 69818|  %1679 = and i64 %1678, -16                                                                                            ;L2445<1660
 69819|  %1680 = gep %1674, i64 %1679                                                                                          ;L2445<1660
 69820|  %1681 = gep %1680, i64 16                                                                                             ;L2445<1660
 69821|  %1682 = gep %1675, i64 64                                                                                             ;L1660
 69822|  %1683 = load ptr, ptr %1682, , !!8                                                                                    ;L1660
 69823|  %1684 = invoke i64 %1683(ptr %1681, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54)
 69824|  to label %1685 unwind label %178                                                                                      ;L1660
 69825| 
 69826| 1685: ; preds = %1673
 69827|  %1686 = icmp eq i64 %1684, 0                                                                                          ;L1660
 69828|     ;; has_heal = i1 %1686
 69831|  %1687 = load ptr, ptr %139, , !!8, !!8                                                                                ;L441<2127<2445<1661
 69832|  %1688 = load ptr, ptr %1482, , !!8, !!8                                                                               ;L441<2127<2445<1661
 69833|  %1689 = gep %1688, i64 16                                                                                             ;L2445<1661
 69834|  %1690 = load i64, ptr %1689,                                                                                          ;L2445<1661
 69835|  %1691 = add nsw i64 %1690, -1                                                                                         ;L2445<1661
 69836|  %1692 = and i64 %1691, -16                                                                                            ;L2445<1661
 69837|  %1693 = gep %1687, i64 %1692                                                                                          ;L2445<1661
 69838|  %1694 = gep %1693, i64 16                                                                                             ;L2445<1661
 69839|  %1695 = gep %1688, i64 72                                                                                             ;L1661
 69840|  %1696 = load ptr, ptr %1695, , !!8                                                                                    ;L1661
 69841|  %1697 = invoke i64 %1696(ptr %1694, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54)
 69842|  to label %1698 unwind label %178                                                                                      ;L1661
 69843| 
 69844| 1698: ; preds = %1685
 69845|  %1699 = icmp eq i64 %1697, 0                                                                                          ;L1661
 69846|     ;; has_shield = i1 %1699
 69848|  %1700 = load i64, ptr %73, , !!8                                                                                      ;L1662
 69849|  invoke void @ai::fight_check18effect_buff_target(ptr sret([288 x i8]) %42, i64 %1700, ptr %139, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54)
 69850|  to label %1701 unwind label %178                                                                                      ;L1662
 69851| 
 69852| 1701: ; preds = %1698
 69853|     ;; self = ptr %42
 69854|  %1702 = load i32, ptr %1499, , !!8                                                                                    ;L633<1663
 69855|  %1703 = icmp ne i32 %1702, -1                                                                                         ;L633<1663
 69856|     ;; has_buff = i1 %1703
 69857|     ;; self = ptr %70
 69858|     ;; self = ptr %70
 69859|  %1704 = load ptr, ptr %70, , !!8, !!8                                                                                 ;L138<2073<1664
 69860|     ;; p = ptr %1704
 69861|  %1705 = load i64, ptr %1471, , !!8                                                                                    ;L2075<1664
 69862|     ;; len = i64 %1705
 69863|     ;; count = i64 %1705
 69864|     ;; self[0..+8] = ptr %1704
 69865|     ;; slice[0..+8] = ptr %1704
 69866|     ;; self[8..+8] = i64 %1705
 69867|     ;; slice[8..+8] = i64 %1705
 69868|     ;; ptr = ptr %1704
 69869|     ;; self = ptr %1704
 69870|  %1706 = shl nuw nsw i64 %1705, 5                                                                                      ;L961<100<1042<1664
 69871|  %1707 = gep %1704, i64 %1706                                                                                          ;L961<100<1042<1664
 69872|  %1708 = load ptr, ptr %1656, , !!8, !!8                                                                               ;L1664
 69875|     ;; f[0..+8] = ptr %86
 69876|     ;; f[8..+8] = ptr %1708
 69877|     ;; self = ptr undef
 69878|     ;; self = ptr undef
 69879|     ;; count = i64 1
 69880|     ;; ptr = ptr %1704
 69881|     ;; self = ptr %1704
 69882|     ;; end_or_len = ptr %1707
 69885|  %1709 = icmp eq i64 %1705, 0                                                                                          ;L1714<180<331<1664
 69886|  br i1 %1709, label %1710, label %1715                                                                                 ;L180<331<1664
 69887| 
 69888| 1710: ; preds = %1701
 69889|  %1711 = gep %1708, i64 1632
 69890|  %1712 = load i64, ptr %1711, , !!75226
 69891|  %1713 = gep %1708, i64 1640
 69892|  %1714 = load i64, ptr %1713, , !!75226
 69893|  br label %1797                                                                                                        ;L180<331<1664
 69894| 
 69895| 1715: ; preds = %1701
 69896|  %1716 = load i64, ptr %86, , !!75228, !!8
 69897|  %1717 = trunc nuw i64 %1716 to i1
 69898|  %1718 = load i64, ptr %1474, , !!75228
 69899|  %1719 = gep %1708, i64 1632
 69900|  %1720 = load i64, ptr %1719, , !!75230
 69901|  %1721 = gep %1708, i64 1640
 69902|  %1722 = load i64, ptr %1721, , !!75230
 69903|  br i1 %1717, label %1723, label %1765
 69904| 
 69905| 1723: ; preds = %1715
 69906|     ;; ptr = ptr %1704
 69907|  %1724 = gep %1704, i64 24                                                                                             ;L332<1664
 69908|  %1725 = load ptr, ptr %1724, , !!75232, !!8, !!8                                                                      ;L332<1664
 69909|  %1726 = gep %1725, i64 1632                                                                                           ;L2158<1664<332<1664
 69910|  %1727 = load i64, ptr %1726, , !!75232, !!8                                                                           ;L2158<1664<332<1664
 69911|  %1728 = gep %1725, i64 1640                                                                                           ;L2158<1664<332<1664
 69912|  %1729 = load i64, ptr %1728, , !!75232, !!8                                                                           ;L2158<1664<332<1664
 69913|  %1730 = icmp ult i64 %1727, %1720                                                                                     ;L3147<7<2158<1664<332<1664
 69914|  %1731 = sub nuw i64 %1720, %1727                                                                                      ;L3147<7<2158<1664<332<1664
 69915|  %1732 = sub nuw i64 %1727, %1720                                                                                      ;L3147<7<2158<1664<332<1664
 69916|  %1733 = select i1 %1730, i64 %1731, i64 %1732                                                                         ;L3147<7<2158<1664<332<1664
 69917|  %1734 = icmp ult i64 %1729, %1722                                                                                     ;L3147<8<2158<1664<332<1664
 69918|  %1735 = sub nuw i64 %1722, %1729                                                                                      ;L3147<8<2158<1664<332<1664
 69919|  %1736 = sub nuw i64 %1729, %1722                                                                                      ;L3147<8<2158<1664<332<1664
 69920|  %1737 = select i1 %1734, i64 %1735, i64 %1736                                                                         ;L3147<8<2158<1664<332<1664
 69921|  %1738 = mul i64 %1733, %1733                                                                                          ;L9<2158<1664<332<1664
 69922|  %1739 = mul i64 %1737, %1737                                                                                          ;L9<2158<1664<332<1664
 69923|  %1740 = add i64 %1739, %1738                                                                                          ;L9<2158<1664<332<1664
 69924|  %1741 = icmp ult i64 %1740, 14400000001                                                                               ;L1664<332<1664
 69925|  br i1 %1741, label %1825, label %1761                                                                                 ;L332<1664
 69926| 
 69927| 1742: ; preds = %1761
 69928|     ;; ptr = ptr %1763
 69929|     ;; x = ptr %1763
 69930|  %1743 = gep %1762, i64 56                                                                                             ;L332<1664
 69931|  %1744 = load ptr, ptr %1743, , !!75232, !!8, !!8                                                                      ;L332<1664
 69936|     ;; self = ptr %1744
 69937|     ;; self = ptr %1744
 69938|     ;; entity = ptr %86
 69939|     ;; other = ptr %1708
 69940|  %1745 = gep %1744, i64 1632                                                                                           ;L2158<1664<332<1664
 69941|  %1746 = load i64, ptr %1745, , !!75232, !!8                                                                           ;L2158<1664<332<1664
 69942|     ;; x1 = i64 %1746
 69943|     ;; self = i64 %1746
 69944|  %1747 = gep %1744, i64 1640                                                                                           ;L2158<1664<332<1664
 69945|  %1748 = load i64, ptr %1747, , !!75232, !!8                                                                           ;L2158<1664<332<1664
 69946|     ;; y1 = i64 %1748
 69947|     ;; self = i64 %1748
 69948|     ;; x2 = i64 %1720
 69949|     ;; other = i64 %1720
 69950|     ;; y2 = i64 %1722
 69951|     ;; other = i64 %1722
 69952|  %1749 = icmp ult i64 %1746, %1720                                                                                     ;L3147<7<2158<1664<332<1664
 69953|  %1750 = sub nuw i64 %1720, %1746                                                                                      ;L3147<7<2158<1664<332<1664
 69954|  %1751 = sub nuw i64 %1746, %1720                                                                                      ;L3147<7<2158<1664<332<1664
 69955|  %1752 = select i1 %1749, i64 %1750, i64 %1751                                                                         ;L3147<7<2158<1664<332<1664
 69956|     ;; dx = i64 %1752
 69957|  %1753 = icmp ult i64 %1748, %1722                                                                                     ;L3147<8<2158<1664<332<1664
 69958|  %1754 = sub nuw i64 %1722, %1748                                                                                      ;L3147<8<2158<1664<332<1664
 69959|  %1755 = sub nuw i64 %1748, %1722                                                                                      ;L3147<8<2158<1664<332<1664
 69960|  %1756 = select i1 %1753, i64 %1754, i64 %1755                                                                         ;L3147<8<2158<1664<332<1664
 69961|     ;; dy = i64 %1756
 69962|  %1757 = mul i64 %1752, %1752                                                                                          ;L9<2158<1664<332<1664
 69963|  %1758 = mul i64 %1756, %1756                                                                                          ;L9<2158<1664<332<1664
 69964|  %1759 = add i64 %1758, %1757                                                                                          ;L9<2158<1664<332<1664
 69965|  %1760 = icmp ult i64 %1759, 14400000001                                                                               ;L1664<332<1664
 69966|  br i1 %1760, label %1825, label %1761                                                                                 ;L332<1664
 69967| 
 69968| 1761: ; preds = %1742, %1723
 69969|  %1762 = phi ptr [ %1763, %1742 ], [ %1704, %1723 ]
 69970|  %1763 = gep %1762, i64 32                                                                                             ;L656<185<331<1664
 69971|     ;; ptr = ptr %1763
 69972|     ;; self = ptr %1763
 69973|     ;; end_or_len = ptr %1707
 69976|  %1764 = icmp eq ptr %1763, %1707                                                                                      ;L1714<180<331<1664
 69977|  br i1 %1764, label %1797, label %1742                                                                                 ;L180<331<1664
 69978| 
 69979| 1765: ; preds = %1715
 69980|  %1766 = icmp ult i64 %1718, 2
 69981|  br i1 %1766, label %1767, label %1795
 69982| 
 69983| 1767: ; preds = %1793, %1765
 69984|  %1768 = phi ptr [ %1769, %1793 ], [ %1704, %1765 ]
 69985|     ;; ptr = ptr %1768
 69986|  %1769 = gep %1768, i64 32                                                                                             ;L656<185<331<1664
 69987|     ;; x = ptr %1768
 69988|  %1770 = gep %1768, i64 24                                                                                             ;L332<1664
 69989|  %1771 = load ptr, ptr %1770, , !!75232, !!8, !!8                                                                      ;L332<1664
 69994|     ;; self = ptr %1771
 69995|     ;; self = ptr %1771
 69996|     ;; entity = ptr %86
 69997|     ;; team = i64 %1718
 69999|  %1772 = gep %1771, i64 56                                                                                             ;L122<1483<1664<332<1664
 70000|  %1773 = gepS %1772, i64 %1718                                                                                         ;L122<1483<1664<332<1664
 70001|  %1774 = load i64, ptr %1773, , !!75232, !!8                                                                           ;L122<1483<1664<332<1664
 70002|  %1775 = icmp eq i64 %1774, 0                                                                                          ;L122<1483<1664<332<1664
 70003|  br i1 %1775, label %1776, label %1793                                                                                 ;L1664<332<1664
 70004| 
 70005| 1776: ; preds = %1767
 70006|     ;; other = ptr %1708
 70007|  %1777 = gep %1771, i64 1632                                                                                           ;L2158<1664<332<1664
 70008|  %1778 = load i64, ptr %1777, , !!75232, !!8                                                                           ;L2158<1664<332<1664
 70009|     ;; x1 = i64 %1778
 70010|     ;; self = i64 %1778
 70011|  %1779 = gep %1771, i64 1640                                                                                           ;L2158<1664<332<1664
 70012|  %1780 = load i64, ptr %1779, , !!75232, !!8                                                                           ;L2158<1664<332<1664
 70013|     ;; y1 = i64 %1780
 70014|     ;; self = i64 %1780
 70015|     ;; x2 = i64 %1720
 70016|     ;; other = i64 %1720
 70017|     ;; y2 = i64 %1722
 70018|     ;; other = i64 %1722
 70019|  %1781 = icmp ult i64 %1778, %1720                                                                                     ;L3147<7<2158<1664<332<1664
 70020|  %1782 = sub nuw i64 %1720, %1778                                                                                      ;L3147<7<2158<1664<332<1664
 70021|  %1783 = sub nuw i64 %1778, %1720                                                                                      ;L3147<7<2158<1664<332<1664
 70022|  %1784 = select i1 %1781, i64 %1782, i64 %1783                                                                         ;L3147<7<2158<1664<332<1664
 70023|     ;; dx = i64 %1784
 70024|  %1785 = icmp ult i64 %1780, %1722                                                                                     ;L3147<8<2158<1664<332<1664
 70025|  %1786 = sub nuw i64 %1722, %1780                                                                                      ;L3147<8<2158<1664<332<1664
 70026|  %1787 = sub nuw i64 %1780, %1722                                                                                      ;L3147<8<2158<1664<332<1664
 70027|  %1788 = select i1 %1785, i64 %1786, i64 %1787                                                                         ;L3147<8<2158<1664<332<1664
 70028|     ;; dy = i64 %1788
 70029|  %1789 = mul i64 %1784, %1784                                                                                          ;L9<2158<1664<332<1664
 70030|  %1790 = mul i64 %1788, %1788                                                                                          ;L9<2158<1664<332<1664
 70031|  %1791 = add i64 %1790, %1789                                                                                          ;L9<2158<1664<332<1664
 70032|  %1792 = icmp ult i64 %1791, 14400000001                                                                               ;L1664<332<1664
 70033|  br i1 %1792, label %1825, label %1793                                                                                 ;L332<1664
 70034| 
 70035| 1793: ; preds = %1776, %1767
 70036|     ;; ptr = ptr %1769
 70037|     ;; self = ptr %1769
 70038|     ;; end_or_len = ptr %1707
 70041|  %1794 = icmp eq ptr %1769, %1707                                                                                      ;L1714<180<331<1664
 70042|  br i1 %1794, label %1797, label %1767                                                                                 ;L180<331<1664
 70043| 
 70044| 1795: ; preds = %1765
 70045|     ;; ptr = ptr %1704
 70046|     ;; x = ptr %1704
 70053|     ;; entity = ptr %86
 70054|     ;; team = i64 %1718
 70055|  invoke void @core::panicking18panic_bounds_check(i64 %1718, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 70056|  to label %1796 unwind label %178                                                                                      ;L1483<1664<332<1664
 70057| 
 70058| 1796: ; preds = %1795
 70059|  unreachable                                                                                                           ;L1483<1664<332<1664
 70060| 
 70061| 1797: ; preds = %1793, %1761, %1710
 70062|  %1798 = phi i64 [ %1714, %1710 ], [ %1722, %1761 ], [ %1722, %1793 ]
 70063|  %1799 = phi i64 [ %1712, %1710 ], [ %1720, %1761 ], [ %1720, %1793 ]
 70064|     ;; self = ptr %1501
 70065|     ;; self = ptr %1501
 70066|  %1800 = load ptr, ptr %1501, , !!8, !!8                                                                               ;L138<2073<1665
 70067|     ;; p = ptr %1800
 70068|  %1801 = load i64, ptr %1502, , !!8                                                                                    ;L2075<1665
 70069|     ;; len = i64 %1801
 70070|     ;; count = i64 %1801
 70071|     ;; self[0..+8] = ptr %1800
 70072|     ;; slice[0..+8] = ptr %1800
 70073|     ;; self[8..+8] = i64 %1801
 70074|     ;; slice[8..+8] = i64 %1801
 70075|     ;; ptr = ptr %1800
 70076|     ;; self = ptr %1800
 70077|  %1802 = getelementptr ptr, ptr %1800, i64 %1801                                                                       ;L961<100<1042<1665
 70079|     ;; f = ptr %1708
 70080|     ;; self = ptr undef
 70081|     ;; self = ptr undef
 70082|     ;; count = i64 1
 70083|  br label %1803                                                                                                        ;L331<1665
 70084| 
 70085| 1803: ; preds = %1806, %1797
 70086|  %1804 = phi ptr [ %1807, %1806 ], [ %1800, %1797 ]
 70087|     ;; ptr = ptr %1804
 70088|     ;; self = ptr %1804
 70089|     ;; end_or_len = ptr %1802
 70092|  %1805 = icmp eq ptr %1804, %1802                                                                                      ;L1714<180<331<1665
 70093|  br i1 %1805, label %1825, label %1806                                                                                 ;L180<331<1665
 70094| 
 70095| 1806: ; preds = %1803
 70096|  %1807 = gep %1804, i64 8                                                                                              ;L656<185<331<1665
 70097|     ;; x = ptr %1804
 70098|  %1808 = load ptr, ptr %1804, , !!75333, !!8, !!8                                                                      ;L332<1665
 70101|     ;; self = ptr %1808
 70102|     ;; other = ptr %1708
 70103|  %1809 = gep %1808, i64 1632                                                                                           ;L2158<1665<332<1665
 70104|  %1810 = load i64, ptr %1809, , !!75333, !!8                                                                           ;L2158<1665<332<1665
 70105|     ;; x1 = i64 %1810
 70106|     ;; self = i64 %1810
 70107|  %1811 = gep %1808, i64 1640                                                                                           ;L2158<1665<332<1665
 70108|  %1812 = load i64, ptr %1811, , !!75333, !!8                                                                           ;L2158<1665<332<1665
 70109|     ;; y1 = i64 %1812
 70110|     ;; self = i64 %1812
 70111|     ;; x2 = i64 %1799
 70112|     ;; other = i64 %1799
 70113|     ;; y2 = i64 %1798
 70114|     ;; other = i64 %1798
 70115|  %1813 = icmp ult i64 %1810, %1799                                                                                     ;L3147<7<2158<1665<332<1665
 70116|  %1814 = sub nuw i64 %1799, %1810                                                                                      ;L3147<7<2158<1665<332<1665
 70117|  %1815 = sub nuw i64 %1810, %1799                                                                                      ;L3147<7<2158<1665<332<1665
 70118|  %1816 = select i1 %1813, i64 %1814, i64 %1815                                                                         ;L3147<7<2158<1665<332<1665
 70119|     ;; dx = i64 %1816
 70120|  %1817 = icmp ult i64 %1812, %1798                                                                                     ;L3147<8<2158<1665<332<1665
 70121|  %1818 = sub nuw i64 %1798, %1812                                                                                      ;L3147<8<2158<1665<332<1665
 70122|  %1819 = sub nuw i64 %1812, %1798                                                                                      ;L3147<8<2158<1665<332<1665
 70123|  %1820 = select i1 %1817, i64 %1818, i64 %1819                                                                         ;L3147<8<2158<1665<332<1665
 70124|     ;; dy = i64 %1820
 70125|  %1821 = mul i64 %1816, %1816                                                                                          ;L9<2158<1665<332<1665
 70126|  %1822 = mul i64 %1820, %1820                                                                                          ;L9<2158<1665<332<1665
 70127|  %1823 = add i64 %1822, %1821                                                                                          ;L9<2158<1665<332<1665
 70128|  %1824 = icmp ult i64 %1823, 14400000001                                                                               ;L1665<332<1665
 70129|  br i1 %1824, label %1825, label %1803                                                                                 ;L332<1665
 70130| 
 70131| 1825: ; preds = %1806, %1803, %1776, %1742, %1723
 70132|  %1826 = phi i1 [ true, %1723 ], [ false, %1803 ], [ true, %1742 ], [ true, %1806 ], [ true, %1776 ]                   ;L0
 70134|  %1827 = gep %1708, i64 1576                                                                                           ;L1667
 70135|  %1828 = load i64, ptr %1827, , !!8                                                                                    ;L1667
 70136|  %1829 = icmp eq i64 %1828, 0                                                                                          ;L1667
 70137|  br i1 %1829, label %1835, label %1830                                                                                 ;L1667
 70138| 
 70139| 1830: ; preds = %1825
 70140|  %1831 = gep %1708, i64 1648                                                                                           ;L1667
 70141|  %1832 = load i64, ptr %1831, , !!8                                                                                    ;L1667
 70142|  %1833 = mul i64 %1832, 100                                                                                            ;L1667
 70143|  %1834 = udiv i64 %1833, %1828                                                                                         ;L1667
 70144|     ;; hp_ratio = i64 %1834
 70145|  br i1 %1686, label %1836, label %1839                                                                                 ;L1669
 70146| 
 70147| 1835: ; preds = %1825
 70148|  invoke void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.272) #31
 70149|  to label %118 unwind label %178                                                                                       ;L1667
 70150| 
 70151| 1836: ; preds = %1830
 70152|  %1837 = or i1 %1699, %1703                                                                                            ;L1675
 70153|  %1838 = or i1 %1837, %1826                                                                                            ;L1675
 70154|  br i1 %1838, label %1851, label %1850                                                                                 ;L1675
 70155| 
 70156| 1839: ; preds = %1830
 70157|  br i1 %1703, label %1851, label %1840                                                                                 ;L1669
 70158| 
 70159| 1840: ; preds = %1839
 70160|  %1841 = icmp ugt i64 %1834, 79                                                                                        ;L1669
 70161|  br i1 %1841, label %1844, label %1842                                                                                 ;L1669
 70162| 
 70163| 1842: ; preds = %1840
 70164|  %1843 = or i1 %1699, %1826                                                                                            ;L1675
 70165|  br i1 %1843, label %1851, label %1850                                                                                 ;L1675
 70166| 
 70167| 1844: ; preds = %1840
 70168|  %1845 = load i64, ptr %73, , !!8                                                                                      ;L1671
 70169|  %1846 = invoke zeroext i1 @ai::buff_value24aoe_heal_covers_low_ally(i64 %1845, ptr %139, ptr %3, ptr %2, ptr %1708)
 70170|  to label %1847 unwind label %178                                                                                      ;L1671
 70171| 
 70172| 1847: ; preds = %1844
 70173|  %1848 = or i1 %1699, %1826
 70174|  %1849 = and i1 %1848, %1846                                                                                           ;L1671
 70175|  br i1 %1849, label %1851, label %1850                                                                                 ;L1671
 70176| 
 70177| 1850: ; preds = %1969, %1876, %1868, %1847, %1842, %1836
 70179|  br label %2034                                                                                                        ;L1
 70180| 
 70181| 1851: ; preds = %1847, %1842, %1839, %1836
 70182|     ;; self = ptr %42
 70183|     ;; default = i1 false
 70185|  %1852 = load i32, ptr %1503,                                                                                          ;L1226<1681
 70189|  %1853 = load ptr, ptr %139, , !!8, !!8                                                                                ;L441<2127<2445<1682
 70190|  %1854 = load ptr, ptr %1482, , !!8, !!8                                                                               ;L441<2127<2445<1682
 70191|  %1855 = gep %1854, i64 16                                                                                             ;L2445<1682
 70192|  %1856 = load i64, ptr %1855,                                                                                          ;L2445<1682
 70193|  %1857 = add nsw i64 %1856, -1                                                                                         ;L2445<1682
 70194|  %1858 = and i64 %1857, -16                                                                                            ;L2445<1682
 70195|  %1859 = gep %1853, i64 %1858                                                                                          ;L2445<1682
 70196|  %1860 = gep %1859, i64 16                                                                                             ;L2445<1682
 70197|  %1861 = gep %1854, i64 144                                                                                            ;L1682
 70198|  %1862 = load ptr, ptr %1861, , !!8                                                                                    ;L1682
 70199|  %1863 = invoke zeroext i1 %1862(ptr %1860)
 70200|  to label %1864 unwind label %178                                                                                      ;L1682
 70201| 
 70202| 1864: ; preds = %1851
 70203|  %1865 = icmp eq i32 %1852, 0                                                                                          ;L1226<1681
 70204|  %1866 = select i1 %1703, i1 %1865, i1 false                                                                           ;L1226<1681
 70205|     ;; is_non_movespeed_buff = i1 %1866
 70206|     ;; is_etc_buff = i1 %1863
 70207|  %1867 = or i1 %1866, %1863                                                                                            ;L1683
 70208|  br i1 %1867, label %1868, label %1873                                                                                 ;L1683
 70209| 
 70210| 1868: ; preds = %1864
 70211|  %1869 = load ptr, ptr %1656, , !!8, !!8                                                                               ;L1684
 70212|     ;; self = ptr %1869
 70213|  %1870 = gep %1869, i64 1216                                                                                           ;L742<1684
 70214|  %1871 = load i32, ptr %1870, , !!8                                                                                    ;L742<1684
 70215|  %1872 = icmp eq i32 %1871, -1                                                                                         ;L742<1684
 70216|  br i1 %1872, label %1850, label %1876                                                                                 ;L742<1684
 70217| 
 70218| 1873: ; preds = %1949, %1864
 70220|  %1874 = load ptr, ptr %1656, , !!8, !!8                                                                               ;L1699
 70221|  %1875 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %139, ptr %86, ptr %1874)
 70222|  to label %1971 unwind label %178                                                                                      ;L1699
 70223| 
 70224| 1876: ; preds = %1868
 70225|  %1877 = gep %1869, i64 1168                                                                                           ;L742<1684
 70226|     ;; target_atk = ptr %1877
 70227|     ;; self = ptr %70
 70228|     ;; self = ptr %70
 70229|  %1878 = load ptr, ptr %70, , !!8, !!8                                                                                 ;L138<2073<1685
 70230|     ;; p = ptr %1878
 70231|  %1879 = load i64, ptr %1471, , !!8                                                                                    ;L2075<1685
 70232|     ;; len = i64 %1879
 70233|     ;; count = i64 %1879
 70234|     ;; self[0..+8] = ptr %1878
 70235|     ;; slice[0..+8] = ptr %1878
 70236|     ;; self[8..+8] = i64 %1879
 70237|     ;; slice[8..+8] = i64 %1879
 70238|     ;; ptr = ptr %1878
 70239|     ;; self = ptr %1878
 70240|  %1880 = shl nuw nsw i64 %1879, 5                                                                                      ;L961<100<1042<1685
 70241|  %1881 = gep %1878, i64 %1880                                                                                          ;L961<100<1042<1685
 70242|     ;; f[0..+8] = ptr %1877
 70243|     ;; f[8..+8] = ptr %1869
 70244|     ;; f[16..+8] = ptr %86
 70245|     ;; self = ptr undef
 70246|     ;; self = ptr undef
 70247|     ;; count = i64 1
 70248|     ;; ptr = ptr %1878
 70249|     ;; self = ptr %1878
 70250|     ;; end_or_len = ptr %1881
 70253|  %1882 = icmp eq i64 %1879, 0                                                                                          ;L1714<180<331<1685
 70254|  br i1 %1882, label %1850, label %1883                                                                                 ;L180<331<1685
 70255| 
 70256| 1883: ; preds = %1876
 70257|  %1884 = gep %1869, i64 1184
 70258|  %1885 = gep %1869, i64 1192
 70259|  %1886 = gep %1869, i64 1480
 70260|  %1887 = gep %1869, i64 1080
 70261|  %1888 = gep %1869, i64 1136
 70262|  %1889 = gep %1869, i64 1664
 70263|  %1890 = gep %1869, i64 1632
 70264|  %1891 = gep %1869, i64 1640
 70265|  br label %1892                                                                                                        ;L180<331<1685
 70266| 
 70267| 1892: ; preds = %1969, %1883
 70268|  %1893 = phi ptr [ %1878, %1883 ], [ %1894, %1969 ]
 70269|     ;; ptr = ptr %1893
 70270|  %1894 = gep %1893, i64 32                                                                                             ;L656<185<331<1685
 70271|     ;; x = ptr %1893
 70272|  %1895 = gep %1893, i64 24                                                                                             ;L332<1685
 70273|  %1896 = load ptr, ptr %1895, , !!75459, !!8, !!8                                                                      ;L332<1685
 70279|     ;; self = ptr %1877
 70280|     ;; caster = ptr %1869
 70281|  %1897 = load i64, ptr %1884, , !!75486, !!8                                                                           ;L26<1686<332<1685
 70282|  %1898 = load i64, ptr %1885, , !!75486, !!8                                                                           ;L26<1686<332<1685
 70283|  %1899 = load i64, ptr %1886, , !!75486, !!8                                                                           ;L26<1686<332<1685
 70284|  %1900 = add i64 %1899, -1                                                                                             ;L26<1686<332<1685
 70285|  %1901 = mul i64 %1900, %1898                                                                                          ;L26<1686<332<1685
 70286|  %1902 = load i64, ptr %1887, , !!75486, !!8                                                                           ;L26<1686<332<1685
 70287|     ;; self = ptr %1896
 70288|     ;; self = ptr %1896
 70289|     ;; self = ptr %1896
 70290|  %1903 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %1877, ptr %1869, ptr %1896)
 70291|  to label %1904 unwind label %178                                                                                      ;L1686<332<1685
 70292| 
 70293| 1904: ; preds = %1892
 70294|     ;; self = ptr %1869
 70295|  %1905 = load i32, ptr %1888, , !!75486, !!8                                                                           ;L1511<1686<332<1685
 70296|     ;; mult = i32 %1905
 70297|  %1906 = icmp eq i32 %1905, 0                                                                                          ;L1512<1686<332<1685
 70298|  br i1 %1906, label %1907, label %1909                                                                                 ;L1512<1686<332<1685
 70299| 
 70300| 1907: ; preds = %1904
 70301|  %1908 = load i64, ptr %1889, , !!75486, !!8                                                                           ;L1513<1686<332<1685
 70302|  br label %1915                                                                                                        ;L1512<1686<332<1685
 70303| 
 70304| 1909: ; preds = %1904
 70305|  %1910 = sext i32 %1905 to i64                                                                                         ;L1511<1686<332<1685
 70306|     ;; mult = i64 %1910
 70307|  %1911 = load i64, ptr %1889, , !!75486, !!8                                                                           ;L1515<1686<332<1685
 70308|  %1912 = add nsw i64 %1910, 100                                                                                        ;L1515<1686<332<1685
 70309|  %1913 = mul i64 %1911, %1912                                                                                          ;L1515<1686<332<1685
 70310|  %1914 = udiv i64 %1913, 100                                                                                           ;L1515<1686<332<1685
 70311|  br label %1915                                                                                                        ;L1512<1686<332<1685
 70312| 
 70313| 1915: ; preds = %1909, %1907
 70314|  %1916 = phi i64 [ %1908, %1907 ], [ %1914, %1909 ]                                                                    ;L0<1686<332<1685
 70315|  %1917 = gep %1896, i64 1136                                                                                           ;L1511<1686<332<1685
 70316|  %1918 = load i32, ptr %1917, , !!75486, !!8                                                                           ;L1511<1686<332<1685
 70317|     ;; mult = i32 %1918
 70318|  %1919 = icmp eq i32 %1918, 0                                                                                          ;L1512<1686<332<1685
 70319|  br i1 %1919, label %1920, label %1923                                                                                 ;L1512<1686<332<1685
 70320| 
 70321| 1920: ; preds = %1915
 70322|  %1921 = gep %1896, i64 1664                                                                                           ;L1513<1686<332<1685
 70323|  %1922 = load i64, ptr %1921, , !!75486, !!8                                                                           ;L1513<1686<332<1685
 70324|  br label %1930                                                                                                        ;L1512<1686<332<1685
 70325| 
 70326| 1923: ; preds = %1915
 70327|  %1924 = sext i32 %1918 to i64                                                                                         ;L1511<1686<332<1685
 70328|     ;; mult = i64 %1924
 70329|  %1925 = gep %1896, i64 1664                                                                                           ;L1515<1686<332<1685
 70330|  %1926 = load i64, ptr %1925, , !!75486, !!8                                                                           ;L1515<1686<332<1685
 70331|  %1927 = add nsw i64 %1924, 100                                                                                        ;L1515<1686<332<1685
 70332|  %1928 = mul i64 %1926, %1927                                                                                          ;L1515<1686<332<1685
 70333|  %1929 = udiv i64 %1928, 100                                                                                           ;L1515<1686<332<1685
 70334|  br label %1930                                                                                                        ;L1512<1686<332<1685
 70335| 
 70336| 1930: ; preds = %1923, %1920
 70337|  %1931 = phi i64 [ %1922, %1920 ], [ %1929, %1923 ]                                                                    ;L0<1686<332<1685
 70338|  %1932 = add i64 %1902, %1897                                                                                          ;L26<1686<332<1685
 70339|  %1933 = add i64 %1932, %1901                                                                                          ;L26<1686<332<1685
 70340|  %1934 = add i64 %1933, %1903                                                                                          ;L1686<332<1685
 70341|  %1935 = add i64 %1934, %1916                                                                                          ;L1686<332<1685
 70342|  %1936 = add i64 %1935, %1931                                                                                          ;L1686<332<1685
 70343|     ;; attack_range = i64 %1936
 70344|     ;; entity = ptr %86
 70345|     ;; self = ptr %86
 70346|  %1937 = load i64, ptr %86, , !!75486, !!8                                                                             ;L1136<1482<1687<332<1685
 70347|  %1938 = trunc nuw i64 %1937 to i1                                                                                     ;L1136<1482<1687<332<1685
 70348|  br i1 %1938, label %1949, label %1939                                                                                 ;L1136<1482<1687<332<1685
 70349| 
 70350| 1939: ; preds = %1930
 70351|     ;; team = ptr %86
 70352|  %1940 = load i64, ptr %1474, , !!75486, !!8                                                                           ;L1137<1482<1687<332<1685
 70353|     ;; team = i64 %1940
 70354|  %1941 = icmp ult i64 %1940, 2                                                                                         ;L1483<1687<332<1685
 70355|  br i1 %1941, label %1942, label %1947                                                                                 ;L1483<1687<332<1685
 70356| 
 70357| 1942: ; preds = %1939
 70359|  %1943 = gep %1896, i64 56                                                                                             ;L122<1483<1687<332<1685
 70360|  %1944 = gepS %1943, i64 %1940                                                                                         ;L122<1483<1687<332<1685
 70361|  %1945 = load i64, ptr %1944, , !!75486, !!8                                                                           ;L122<1483<1687<332<1685
 70362|  %1946 = icmp eq i64 %1945, 0                                                                                          ;L122<1483<1687<332<1685
 70363|  br i1 %1946, label %1949, label %1969                                                                                 ;L1687<332<1685
 70364| 
 70365| 1947: ; preds = %1939
 70366|  invoke void @core::panicking18panic_bounds_check(i64 %1940, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 70367|  to label %1948 unwind label %178                                                                                      ;L1483<1687<332<1685
 70368| 
 70369| 1948: ; preds = %1947
 70370|  unreachable                                                                                                           ;L1483<1687<332<1685
 70371| 
 70372| 1949: ; preds = %1942, %1930
 70373|     ;; other = ptr %1869
 70374|  %1950 = gep %1896, i64 1632                                                                                           ;L2158<1687<332<1685
 70375|  %1951 = load i64, ptr %1950, , !!75486, !!8                                                                           ;L2158<1687<332<1685
 70376|     ;; x1 = i64 %1951
 70377|     ;; self = i64 %1951
 70378|  %1952 = gep %1896, i64 1640                                                                                           ;L2158<1687<332<1685
 70379|  %1953 = load i64, ptr %1952, , !!75486, !!8                                                                           ;L2158<1687<332<1685
 70380|     ;; y1 = i64 %1953
 70381|     ;; self = i64 %1953
 70382|  %1954 = load i64, ptr %1890, , !!75486, !!8                                                                           ;L2158<1687<332<1685
 70383|     ;; x2 = i64 %1954
 70384|     ;; other = i64 %1954
 70385|  %1955 = load i64, ptr %1891, , !!75486, !!8                                                                           ;L2158<1687<332<1685
 70386|     ;; y2 = i64 %1955
 70387|     ;; other = i64 %1955
 70388|  %1956 = icmp ult i64 %1951, %1954                                                                                     ;L3147<7<2158<1687<332<1685
 70389|  %1957 = sub nuw i64 %1954, %1951                                                                                      ;L3147<7<2158<1687<332<1685
 70390|  %1958 = sub nuw i64 %1951, %1954                                                                                      ;L3147<7<2158<1687<332<1685
 70391|  %1959 = select i1 %1956, i64 %1957, i64 %1958                                                                         ;L3147<7<2158<1687<332<1685
 70392|     ;; dx = i64 %1959
 70393|  %1960 = icmp ult i64 %1953, %1955                                                                                     ;L3147<8<2158<1687<332<1685
 70394|  %1961 = sub nuw i64 %1955, %1953                                                                                      ;L3147<8<2158<1687<332<1685
 70395|  %1962 = sub nuw i64 %1953, %1955                                                                                      ;L3147<8<2158<1687<332<1685
 70396|  %1963 = select i1 %1960, i64 %1961, i64 %1962                                                                         ;L3147<8<2158<1687<332<1685
 70397|     ;; dy = i64 %1963
 70398|  %1964 = mul i64 %1959, %1959                                                                                          ;L9<2158<1687<332<1685
 70399|  %1965 = mul i64 %1963, %1963                                                                                          ;L9<2158<1687<332<1685
 70400|  %1966 = add i64 %1965, %1964                                                                                          ;L9<2158<1687<332<1685
 70401|  %1967 = mul i64 %1936, %1936                                                                                          ;L1687<332<1685
 70402|  %1968 = icmp ugt i64 %1966, %1967                                                                                     ;L1687<332<1685
 70403|  br i1 %1968, label %1969, label %1873                                                                                 ;L332<1685
 70404| 
 70405| 1969: ; preds = %1949, %1942
 70406|     ;; ptr = ptr %1894
 70407|     ;; self = ptr %1894
 70408|     ;; end_or_len = ptr %1881
 70411|  %1970 = icmp eq ptr %1894, %1881                                                                                      ;L1714<180<331<1685
 70412|  br i1 %1970, label %1850, label %1892                                                                                 ;L180<331<1685
 70413| 
 70414| 1971: ; preds = %1873
 70415|  %1972 = load ptr, ptr %1656, , !!8, !!8                                                                               ;L1699
 70416|     ;; self = ptr %1972
 70417|  %1973 = gep %1972, i64 1136                                                                                           ;L1511<1699
 70418|  %1974 = load i32, ptr %1973, , !!8                                                                                    ;L1511<1699
 70419|     ;; mult = i32 %1974
 70420|  %1975 = icmp eq i32 %1974, 0                                                                                          ;L1512<1699
 70421|  br i1 %1975, label %1976, label %1979                                                                                 ;L1512<1699
 70422| 
 70423| 1976: ; preds = %1971
 70424|  %1977 = gep %1972, i64 1664                                                                                           ;L1513<1699
 70425|  %1978 = load i64, ptr %1977, , !!8                                                                                    ;L1513<1699
 70426|  br label %1986                                                                                                        ;L1512<1699
 70427| 
 70428| 1979: ; preds = %1971
 70429|  %1980 = sext i32 %1974 to i64                                                                                         ;L1511<1699
 70430|     ;; mult = i64 %1980
 70431|  %1981 = gep %1972, i64 1664                                                                                           ;L1515<1699
 70432|  %1982 = load i64, ptr %1981, , !!8                                                                                    ;L1515<1699
 70433|  %1983 = add nsw i64 %1980, 100                                                                                        ;L1515<1699
 70434|  %1984 = mul i64 %1982, %1983                                                                                          ;L1515<1699
 70435|  %1985 = udiv i64 %1984, 100                                                                                           ;L1515<1699
 70436|  br label %1986                                                                                                        ;L1512<1699
 70437| 
 70438| 1986: ; preds = %1979, %1976
 70439|  %1987 = phi i64 [ %1978, %1976 ], [ %1985, %1979 ]                                                                    ;L0<1699
 70441|     ;; self = ptr %1972
 70442|  %1988 = gep %1972, i64 1632                                                                                           ;L2158<1700
 70443|  %1989 = load i64, ptr %1988, , !!8                                                                                    ;L2158<1700
 70444|     ;; x1 = i64 %1989
 70445|     ;; self = i64 %1989
 70446|  %1990 = gep %1972, i64 1640                                                                                           ;L2158<1700
 70447|  %1991 = load i64, ptr %1990, , !!8                                                                                    ;L2158<1700
 70448|     ;; y1 = i64 %1991
 70449|     ;; self = i64 %1991
 70450|  %1992 = load i64, ptr %1480, , !!8                                                                                    ;L2158<1700
 70451|     ;; x2 = i64 %1992
 70452|     ;; other = i64 %1992
 70453|  %1993 = load i64, ptr %1481, , !!8                                                                                    ;L2158<1700
 70454|     ;; y2 = i64 %1993
 70455|     ;; other = i64 %1993
 70456|  %1994 = icmp ult i64 %1989, %1992                                                                                     ;L3147<7<2158<1700
 70457|  %1995 = sub nuw i64 %1992, %1989                                                                                      ;L3147<7<2158<1700
 70458|  %1996 = sub nuw i64 %1989, %1992                                                                                      ;L3147<7<2158<1700
 70459|  %1997 = select i1 %1994, i64 %1995, i64 %1996                                                                         ;L3147<7<2158<1700
 70460|     ;; dx = i64 %1997
 70461|  %1998 = icmp ult i64 %1991, %1993                                                                                     ;L3147<8<2158<1700
 70462|  %1999 = sub nuw i64 %1993, %1991                                                                                      ;L3147<8<2158<1700
 70463|  %2000 = sub nuw i64 %1991, %1993                                                                                      ;L3147<8<2158<1700
 70464|  %2001 = select i1 %1998, i64 %1999, i64 %2000                                                                         ;L3147<8<2158<1700
 70465|     ;; dy = i64 %2001
 70466|  %2002 = mul i64 %1997, %1997                                                                                          ;L9<2158<1700
 70467|  %2003 = mul i64 %2001, %2001                                                                                          ;L9<2158<1700
 70468|  %2004 = add i64 %2003, %2002                                                                                          ;L9<2158<1700
 70469|     ;; dist_sq = i64 %2004
 70470|     ;; max_tick = i64 %97
 70471|  %2005 = add i64 %1505, %1875                                                                                          ;L1699
 70472|  %2006 = add i64 %2005, %1987                                                                                          ;L1709
 70473|     ;; max_dist = i64 %2006
 70474|  %2007 = mul i64 %2006, %2006                                                                                          ;L1710
 70475|  %2008 = icmp ugt i64 %2004, %2007                                                                                     ;L1710
 70476|  br i1 %2008, label %2034, label %2009                                                                                 ;L1710
 70477| 
 70478| 2009: ; preds = %1986
 70479|  %2010 = load ptr, ptr %1477, , !!8, !!8                                                                               ;L1714
 70480|  %2011 = load ptr, ptr %1478, , !!8, !!8                                                                               ;L1714
 70481|  %2012 = gep %2011, i64 192                                                                                            ;L1714
 70482|  %2013 = load ptr, ptr %2012, , !!8                                                                                    ;L1714
 70483|  %2014 = invoke zeroext i1 %2013(ptr %2010, ptr %1666, ptr %1667, ptr %86)
 70484|  to label %2015 unwind label %178                                                                                      ;L1714
 70485| 
 70486| 2015: ; preds = %2009
 70487|  br i1 %2014, label %2016, label %2034                                                                                 ;L1714
 70488| 
 70489| 2016: ; preds = %2015
 70492|  %2017 = load ptr, ptr %1656, , !!8, !!8                                                                               ;L1718
 70493|  %2018 = gep %2017, i64 1472                                                                                           ;L1718
 70494|  %2019 = load i64, ptr %2018, , !!8                                                                                    ;L1718
 70495|  invoke void @ai::small_action4castNtB5_17SmallActionSkill23new(ptr sret([24 x i8]) %40, ptr %3, i64 %2019)
 70496|  to label %2020 unwind label %178                                                                                      ;L1718
 70497| 
 70498| 2020: ; preds = %2016
 70499|  call void @llvm.memcpy.p0.p0.i64(ptr %41, ptr %40, i64 24, i1 false)                                                  ;L1718
 70500|  store i8 17, ptr %1506,                                                                                               ;L1718
 70503|     ;; self = ptr %65
 70504|     ;; self = ptr %65
 70505|     ;; value = ptr %41
 70506|     ;; src = ptr %41
 70507|     ;; additional = i64 1
 70508|     ;; needed_extra_cap = i64 1
 70509|     ;; needed_extra_cap = i64 1
 70510|     ;; strategy = i8 1
 70511|  %2021 = load i64, ptr %126, , !!75609, !!8                                                                            ;L1428<1718
 70512|     ;; self = ptr %65
 70513|  %2022 = load i64, ptr %125, , !!75609, !!8                                                                            ;L149<1428<1718
 70514|  %2023 = icmp eq i64 %2021, %2022                                                                                      ;L1428<1718
 70515|  br i1 %2023, label %2024, label %2029                                                                                 ;L1428<1718
 70516| 
 70517| 2024: ; preds = %2020
 70518|     ;; self = ptr %65
 70519|     ;; self = ptr %65
 70520|     ;; self = ptr %65
 70521|     ;; used_cap = i64 %2021
 70522|     ;; used_cap = i64 %2021
 70523|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %2021, i64 1, i1 zeroext true)
 70524|  to label %2025 unwind label %2027, !!75609                                                                            ;L619<430<738<1429<1718
 70525| 
 70526| 2025: ; preds = %2024
 70527|  %2026 = load i64, ptr %126, , !!75609                                                                                 ;L1432<1718
 70528|  br label %2029                                                                                                        ;L619<430<738<1429<1718
 70529| 
 70530| 2027: ; preds = %2024
 70531|  %2028 = cleanuppad within none []
 70532|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %41) #30 [ "funclet"(token %2028) ], !!75594 ;L1436<1718
 70533|  cleanupret from %2028 unwind label %178
 70534| 
 70535| 2029: ; preds = %2025, %2020
 70536|  %2030 = phi i64 [ %2026, %2025 ], [ %2021, %2020 ]                                                                    ;L1432<1718
 70537|     ;; self = ptr %65
 70538|  %2031 = load ptr, ptr %65, , !!75609, !!8, !!8                                                                        ;L138<1432<1718
 70539|     ;; self = ptr %2031
 70540|     ;; count = i64 %2030
 70541|  %2032 = gepS %2031, i64 %2030                                                                                         ;L961<1432<1718
 70542|     ;; end = ptr %2032
 70543|     ;; dst = ptr %2032
 70544|  call void @llvm.memcpy.p0.p0.i64(ptr %2032, ptr %41, i64 184, i1 false), !!75594                                      ;L1933<1433<1718
 70545|  %2033 = add i64 %2030, 1                                                                                              ;L1434<1718
 70546|  store i64 %2033, ptr %126, , !!75609                                                                                  ;L1434<1718
 70548|  br label %2034                                                                                                        ;L1650
 70549| 
 70550| 2034: ; preds = %2029, %2015, %1986, %1850, %1672, %1662
 70551|  br label %1655                                                                                                        ;L1714<180<1650
 70552| 
 70553| 2035: ; preds = %1655
 70554|  %2036 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %1475, ptr %86, ptr %86)
 70555|  to label %2037 unwind label %178                                                                                      ;L1721
 70556| 
 70557| 2037: ; preds = %2035
 70558|  br i1 %2036, label %2038, label %1452                                                                                 ;L1721
 70559| 
 70560| 2038: ; preds = %2037
 70561|  %2039 = load ptr, ptr %1477, , !!8, !!8                                                                               ;L1721
 70562|  %2040 = load ptr, ptr %1478, , !!8, !!8                                                                               ;L1721
 70563|  %2041 = load ptr, ptr %82, , !!8, !!8                                                                                 ;L1721
 70564|  %2042 = load ptr, ptr %1479, , !!8, !!8                                                                               ;L1721
 70565|  %2043 = gep %2040, i64 200                                                                                            ;L1721
 70566|  %2044 = load ptr, ptr %2043, , !!8                                                                                    ;L1721
 70567|  %2045 = invoke zeroext i1 %2044(ptr %2039, ptr %2041, ptr %2042, ptr %86, ptr %86)
 70568|  to label %2046 unwind label %178                                                                                      ;L1721
 70569| 
 70570| 2046: ; preds = %2038
 70571|  br i1 %2045, label %2047, label %1452                                                                                 ;L1721
 70572| 
 70573| 2047: ; preds = %2046
 70574|     ;; skip_self_buff = i8 0
 70576|  %2048 = load i64, ptr %73, , !!8                                                                                      ;L1725
 70577|  invoke void @ai::fight_check18effect_buff_target(ptr sret([288 x i8]) %39, i64 %2048, ptr %139, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54)
 70578|  to label %2049 unwind label %178                                                                                      ;L1725
 70579| 
 70580| 2049: ; preds = %2047
 70583|  %2050 = load ptr, ptr %139, , !!8, !!8                                                                                ;L441<2127<2445<1726
 70584|  %2051 = load ptr, ptr %1482, , !!8, !!8                                                                               ;L441<2127<2445<1726
 70585|  %2052 = gep %2051, i64 16                                                                                             ;L2445<1726
 70586|  %2053 = load i64, ptr %2052,                                                                                          ;L2445<1726
 70587|  %2054 = add nsw i64 %2053, -1                                                                                         ;L2445<1726
 70588|  %2055 = and i64 %2054, -16                                                                                            ;L2445<1726
 70589|  %2056 = gep %2050, i64 %2055                                                                                          ;L2445<1726
 70590|  %2057 = gep %2056, i64 16                                                                                             ;L2445<1726
 70591|  %2058 = gep %2051, i64 72                                                                                             ;L1726
 70592|  %2059 = load ptr, ptr %2058, , !!8                                                                                    ;L1726
 70593|  %2060 = invoke i64 %2059(ptr %2057, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54)
 70594|  to label %2061 unwind label %178                                                                                      ;L1726
 70595| 
 70596| 2061: ; preds = %2049
 70597|  %2062 = icmp eq i64 %2060, 0                                                                                          ;L1726
 70598|     ;; has_shield = i1 %2062
 70599|     ;; self = ptr %39
 70600|     ;; default = i1 false
 70602|  %2063 = gep %39, i64 72                                                                                               ;L1226<1727
 70603|  %2064 = load i32, ptr %2063, , !!8                                                                                    ;L1226<1727
 70604|  %2065 = gep %39, i64 136                                                                                              ;L1226<1727
 70605|  %2066 = load i32, ptr %2065,                                                                                          ;L1226<1727
 70609|  %2067 = load ptr, ptr %139, , !!8, !!8                                                                                ;L441<2127<2445<1728
 70610|  %2068 = load ptr, ptr %1482, , !!8, !!8                                                                               ;L441<2127<2445<1728
 70611|  %2069 = gep %2068, i64 16                                                                                             ;L2445<1728
 70612|  %2070 = load i64, ptr %2069,                                                                                          ;L2445<1728
 70613|  %2071 = add nsw i64 %2070, -1                                                                                         ;L2445<1728
 70614|  %2072 = and i64 %2071, -16                                                                                            ;L2445<1728
 70615|  %2073 = gep %2067, i64 %2072                                                                                          ;L2445<1728
 70616|  %2074 = gep %2073, i64 16                                                                                             ;L2445<1728
 70617|  %2075 = gep %2068, i64 144                                                                                            ;L1728
 70618|  %2076 = load ptr, ptr %2075, , !!8                                                                                    ;L1728
 70619|  %2077 = invoke zeroext i1 %2076(ptr %2074)
 70620|  to label %2078 unwind label %178                                                                                      ;L1728
 70621| 
 70622| 2078: ; preds = %2061
 70623|  %2079 = icmp ne i32 %2064, -1                                                                                         ;L1226<1727
 70624|  %2080 = icmp eq i32 %2066, 0                                                                                          ;L1226<1727
 70625|  %2081 = select i1 %2079, i1 %2080, i1 false                                                                           ;L1226<1727
 70626|     ;; is_non_movespeed_buff = i1 %2081
 70627|     ;; is_etc_buff = i1 %2077
 70628|  %2082 = or i1 %2081, %2077                                                                                            ;L1729
 70629|  br i1 %2082, label %2083, label %2084                                                                                 ;L1729
 70630| 
 70631| 2083: ; preds = %2078
 70632|     ;; skill2_action = ptr %1477
 70633|  br i1 %2062, label %2096, label %2090                                                                                 ;L1731
 70634| 
 70635| 2084: ; preds = %2116, %2115, %2078
 70637|  %2085 = load ptr, ptr %1477, , !!8, !!8                                                                               ;L1753
 70638|  %2086 = load ptr, ptr %1478, , !!8, !!8                                                                               ;L1753
 70639|  %2087 = gep %2086, i64 192                                                                                            ;L1753
 70640|  %2088 = load ptr, ptr %2087, , !!8                                                                                    ;L1753
 70641|  %2089 = invoke zeroext i1 %2088(ptr %2085, ptr %2041, ptr %2042, ptr %86)
 70642|  to label %2117 unwind label %178                                                                                      ;L1753
 70643| 
 70644| 2090: ; preds = %2083
 70645|  %2091 = load ptr, ptr %1477, , !!8, !!8                                                                               ;L1732
 70646|  %2092 = load ptr, ptr %1478, , !!8, !!8                                                                               ;L1732
 70647|  %2093 = gep %2092, i64 104                                                                                            ;L1732
 70648|  %2094 = load ptr, ptr %2093, , !!8                                                                                    ;L1732
 70649|  %2095 = invoke { ptr, ptr } %2094(ptr %2091)
 70650|  to label %2097 unwind label %178                                                                                      ;L1732
 70651| 
 70652| 2096: ; preds = %2102, %2083
 70653|     ;; self = ptr %86
 70654|  br i1 %130, label %2114, label %2108                                                                                  ;L742<1740
 70655| 
 70656| 2097: ; preds = %2090
 70657|  %2098 = extractvalue { ptr, ptr } %2095, 0                                                                            ;L1732
 70658|  %2099 = extractvalue { ptr, ptr } %2095, 1                                                                            ;L1732
 70659|     ;; self[0..+8] = ptr %2098
 70660|     ;; self[0..+8] = ptr %2098
 70661|     ;; self[8..+8] = ptr %2099
 70662|     ;; self[8..+8] = ptr %2099
 70664|  %2100 = gep %2099, i64 24                                                                                             ;L201<229<1733
 70665|  %2101 = load ptr, ptr %2100, , !!8                                                                                    ;L201<229<1733
 70666|  invoke void %2101(ptr sret([16 x i8]) %8, ptr %2098)
 70667|  to label %2102 unwind label %178                                                                                      ;L201<229<1733
 70668| 
 70669| 2102: ; preds = %2097
 70672|     ;; other = ptr %8
 70673|  %2103 = load i128, ptr %8, , !!8                                                                                      ;L764<2450<204<229<1733
 70674|  %2104 = icmp eq i128 %2103, 168406848281932906149591046147716593956                                                   ;L764<2450<204<229<1733
 70676|  br i1 %2104, label %2105, label %2096                                                                                 ;L229<1733
 70677| 
 70678| 2105: ; preds = %2102
 70679|     ;; self = ptr %2098
 70680|     ;; f[0..+8] = ptr %2041
 70681|     ;; f[8..+8] = ptr %2042
 70682|     ;; f[16..+8] = ptr %86
 70683|  %2106 = icmp ne ptr %2098, null
 70684|  call void @llvm.assume(i1 %2106)
 70685|     ;; x = ptr %2098
 70686|     ;; action = ptr %2098
 70687|  %2107 = invoke zeroext i1 @gc::setting8champion8prisonerNtB5_20PrisonerSkill2Action42has_enemy_champion_target_or_action_threat(ptr %2098, ptr %2041, ptr %2042, ptr %86)
 70688|  to label %2116 unwind label %178                                                                                      ;L1734<1162<1734
 70689| 
 70690| 2108: ; preds = %2096
 70691|     ;; target_atk = ptr %127
 70693|     ;; self = ptr %70
 70694|     ;; self = ptr %70
 70695|  %2109 = load ptr, ptr %70, , !!8, !!8                                                                                 ;L138<2073<1741
 70696|     ;; p = ptr %2109
 70697|  %2110 = load i64, ptr %1471, , !!8                                                                                    ;L2075<1741
 70698|     ;; len = i64 %2110
 70699|     ;; count = i64 %2110
 70700|     ;; self[0..+8] = ptr %2109
 70701|     ;; slice[0..+8] = ptr %2109
 70702|     ;; self[8..+8] = i64 %2110
 70703|     ;; slice[8..+8] = i64 %2110
 70704|     ;; ptr = ptr %2109
 70705|     ;; self = ptr %2109
 70706|  %2111 = gepS }, ptr %2109, i64 %2110                                                                                  ;L961<100<1042<1741
 70707|     ;; end_or_len = ptr %2111
 70708|  store ptr %2109, ptr %38,                                                                                             ;L102<1042<1741
 70709|  %2112 = gep %38, i64 8                                                                                                ;L102<1042<1741
 70710|  store ptr %2111, ptr %2112,                                                                                           ;L102<1042<1741
 70711|  %2113 = invoke fastcc zeroext i1 @core::slice4iterINtB7_4IterTNtNtNtNtCs97f5S1uJLkH_9game_core10simulation4game10blackboard11SmallActionRNtNtBX_6entity6EntityEENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6battle18base_battle_actionsk_0EB3k_(ptr %38, ptr %127, ptr %86)
 70712|  to label %2115 unwind label %178                                                                                      ;L1741
 70713| 
 70714| 2114: ; preds = %2116, %2115, %2096
 70715|     ;; skip_self_buff = i8 1
 70717|  br label %1452                                                                                                        ;L1753
 70718| 
 70719| 2115: ; preds = %2108
 70720|     ;; has_enemy_in_attack_range = i1 %2113
 70722|  br i1 %2113, label %2084, label %2114                                                                                 ;L1748
 70723| 
 70724| 2116: ; preds = %2105
 70725|     ;; has_enemy_in_attack_range = i1 %2107
 70726|  br i1 %2107, label %2084, label %2114                                                                                 ;L1748
 70727| 
 70728| 2117: ; preds = %2084
 70729|  br i1 %2089, label %2118, label %1452                                                                                 ;L1753
 70730| 
 70731| 2118: ; preds = %2117
 70734|  %2119 = gep %86, i64 1472                                                                                             ;L1754
 70735|  %2120 = load i64, ptr %2119, , !!8                                                                                    ;L1754
 70736|  invoke void @ai::small_action4castNtB5_17SmallActionSkill23new(ptr sret([24 x i8]) %36, ptr %3, i64 %2120)
 70737|  to label %2121 unwind label %178                                                                                      ;L1754
 70738| 
 70739| 2121: ; preds = %2118
 70740|  call void @llvm.memcpy.p0.p0.i64(ptr %37, ptr %36, i64 24, i1 false)                                                  ;L1754
 70741|  %2122 = gep %37, i64 177                                                                                              ;L1754
 70742|  store i8 17, ptr %2122,                                                                                               ;L1754
 70745|     ;; self = ptr %65
 70746|     ;; self = ptr %65
 70747|     ;; value = ptr %37
 70748|     ;; src = ptr %37
 70749|     ;; additional = i64 1
 70750|     ;; needed_extra_cap = i64 1
 70751|     ;; needed_extra_cap = i64 1
 70752|     ;; strategy = i8 1
 70753|  %2123 = load i64, ptr %126, , !!75750, !!8                                                                            ;L1428<1754
 70754|     ;; self = ptr %65
 70755|  %2124 = load i64, ptr %125, , !!75750, !!8                                                                            ;L149<1428<1754
 70756|  %2125 = icmp eq i64 %2123, %2124                                                                                      ;L1428<1754
 70757|  br i1 %2125, label %2126, label %2131                                                                                 ;L1428<1754
 70758| 
 70759| 2126: ; preds = %2121
 70760|     ;; self = ptr %65
 70761|     ;; self = ptr %65
 70762|     ;; self = ptr %65
 70763|     ;; used_cap = i64 %2123
 70764|     ;; used_cap = i64 %2123
 70765|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %2123, i64 1, i1 zeroext true)
 70766|  to label %2127 unwind label %2129, !!75750                                                                            ;L619<430<738<1429<1754
 70767| 
 70768| 2127: ; preds = %2126
 70769|  %2128 = load i64, ptr %126, , !!75750                                                                                 ;L1432<1754
 70770|  br label %2131                                                                                                        ;L619<430<738<1429<1754
 70771| 
 70772| 2129: ; preds = %2126
 70773|  %2130 = cleanuppad within none []
 70774|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %37) #30 [ "funclet"(token %2130) ], !!75735 ;L1436<1754
 70775|  cleanupret from %2130 unwind label %178
 70776| 
 70777| 2131: ; preds = %2127, %2121
 70778|  %2132 = phi i64 [ %2128, %2127 ], [ %2123, %2121 ]                                                                    ;L1432<1754
 70779|     ;; self = ptr %65
 70780|  %2133 = load ptr, ptr %65, , !!75750, !!8, !!8                                                                        ;L138<1432<1754
 70781|     ;; self = ptr %2133
 70782|     ;; count = i64 %2132
 70783|  %2134 = gepS %2133, i64 %2132                                                                                         ;L961<1432<1754
 70784|     ;; end = ptr %2134
 70785|     ;; dst = ptr %2134
 70786|  call void @llvm.memcpy.p0.p0.i64(ptr %2134, ptr %37, i64 184, i1 false), !!75735                                      ;L1933<1433<1754
 70787|  %2135 = add i64 %2132, 1                                                                                              ;L1434<1754
 70788|  store i64 %2135, ptr %126, , !!75750                                                                                  ;L1434<1754
 70790|  br label %1452                                                                                                        ;L1753
 70791| 
 70792| 2136: ; preds = %2312, %1452
 70793|  %2137 = phi ptr [ %1454, %1452 ], [ %2146, %2312 ]                                                                    ;L1761
 70794|     ;; iter[0..+8] = ptr %2137
 70795|     ;; self = ptr undef
 70796|     ;; ptr = ptr %2137
 70797|     ;; self = ptr %2137
 70798|     ;; end_or_len = ptr %1457
 70801|  %2138 = icmp eq ptr %2137, %1457                                                                                      ;L1714<180<1761
 70802|  br i1 %2138, label %2139, label %2145                                                                                 ;L180<1761
 70803| 
 70804| 2139: ; preds = %2136
 70805|  %2140 = mul i64 %239, 15
 70806|  %2141 = add i64 %2140, %177
 70807|  %2142 = gep %29, i64 177
 70808|  %2143 = gep %27, i64 177
 70809|  %2144 = gep %25, i64 177
 70810|  br label %2376                                                                                                        ;L180<1798
 70811| 
 70812| 2145: ; preds = %2136
 70813|  %2146 = gep %2137, i64 8                                                                                              ;L656<185<1761
 70814|     ;; iter[0..+8] = ptr %2146
 70815|     ;; e = ptr %2137
 70816|  %2147 = load ptr, ptr %2137, , !!8, !!8                                                                               ;L1762
 70817|     ;; self = ptr %2147
 70818|  %2148 = gep %2147, i64 1721                                                                                           ;L1478<1762
 70819|  %2149 = load i8, ptr %2148, , !!8                                                                                     ;L1478<1762
 70820|  %2150 = trunc nuw i8 %2149 to i1                                                                                      ;L1478<1762
 70821|  br i1 %2150, label %2151, label %2312                                                                                 ;L1478<1762
 70822| 
 70823| 2151: ; preds = %2145
 70824|  %2152 = gep %2147, i64 1696                                                                                           ;L1478<1762
 70825|  %2153 = load i64, ptr %2152, , !!8                                                                                    ;L1478<1762
 70826|  %2154 = icmp eq i64 %2153, 0                                                                                          ;L1478<1762
 70827|  br i1 %2154, label %2155, label %2312                                                                                 ;L1762
 70828| 
 70829| 2155: ; preds = %2151
 70830|     ;; self = ptr %2147
 70831|     ;; self = ptr %86
 70832|  %2156 = load i64, ptr %86, , !!8                                                                                      ;L1136<1482<1762
 70833|  %2157 = trunc nuw i64 %2156 to i1                                                                                     ;L1136<1482<1762
 70834|  br i1 %2157, label %2167, label %2158                                                                                 ;L1136<1482<1762
 70835| 
 70836| 2158: ; preds = %2155
 70837|     ;; team = ptr %86
 70838|  %2159 = load i64, ptr %1458, , !!8                                                                                    ;L1137<1482<1762
 70839|     ;; team = i64 %2159
 70840|  %2160 = icmp ult i64 %2159, 2                                                                                         ;L1483<1762
 70841|  br i1 %2160, label %2161, label %2166                                                                                 ;L1483<1762
 70842| 
 70843| 2161: ; preds = %2158
 70845|  %2162 = gep %2147, i64 56                                                                                             ;L122<1483<1762
 70846|  %2163 = gepS %2162, i64 %2159                                                                                         ;L122<1483<1762
 70847|  %2164 = load i64, ptr %2163, , !!8                                                                                    ;L122<1483<1762
 70848|  %2165 = icmp eq i64 %2164, 0                                                                                          ;L122<1483<1762
 70849|  br i1 %2165, label %2167, label %2312                                                                                 ;L1762
 70850| 
 70851| 2166: ; preds = %2158
 70852|  invoke void @core::panicking18panic_bounds_check(i64 %2159, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 70853|  to label %118 unwind label %178                                                                                       ;L1483<1762
 70854| 
 70855| 2167: ; preds = %2161, %2155
 70856|     ;; self = ptr %2147
 70857|  %2168 = gep %2147, i64 104                                                                                            ;L1370<1765
 70858|  %2169 = load i64, ptr %2168, , !!8                                                                                    ;L1370<1765
 70859|  %2170 = add nsw i64 %2169, -5                                                                                         ;L1765
 70860|  %2171 = icmp ult i64 %2170, 2                                                                                         ;L1765
 70861|  br i1 %2171, label %2172, label %2312                                                                                 ;L1765
 70862| 
 70863| 2172: ; preds = %2167
 70864|  %2173 = gep %2147, i64 1648                                                                                           ;L1768
 70865|  %2174 = load i64, ptr %2173, , !!8                                                                                    ;L1768
 70866|  %2175 = mul i64 %2174, 100                                                                                            ;L1768
 70867|  %2176 = gep %2147, i64 1576                                                                                           ;L1768
 70868|  %2177 = load i64, ptr %2176, , !!8                                                                                    ;L1768
 70869|     ;; self = i64 %2177
 70870|     ;; other = i64 1
 70871|  %2178 = call i64 @llvm.umax.i64(i64 %2177, i64 1)                                                                     ;L1039<1768
 70872|  %2179 = udiv i64 %2175, %2178                                                                                         ;L1768
 70873|     ;; hp_ratio = i64 %2179
 70874|  %2180 = icmp ugt i64 %2179, 20                                                                                        ;L1769
 70875|  br i1 %2180, label %2312, label %2181                                                                                 ;L1769
 70876| 
 70877| 2181: ; preds = %2172
 70878|  br i1 %130, label %2184, label %2182                                                                                  ;L1773
 70879| 
 70880| 2182: ; preds = %2181
 70881|  %2183 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_attack(ptr %86)
 70882|  to label %2185 unwind label %178                                                                                      ;L1773
 70883| 
 70884| 2184: ; preds = %2239, %2204, %2185, %2181
 70885|  br i1 %133, label %2246, label %2244                                                                                  ;L1783
 70886| 
 70887| 2185: ; preds = %2182
 70888|  br i1 %2183, label %2186, label %2184                                                                                 ;L1773
 70889| 
 70890| 2186: ; preds = %2185
 70891|  %2187 = load ptr, ptr %2137, , !!8, !!8                                                                               ;L1774
 70892|  %2188 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %127, ptr %86, ptr %2187)
 70893|  to label %2189 unwind label %178                                                                                      ;L1774
 70894| 
 70895| 2189: ; preds = %2186
 70896|  %2190 = load ptr, ptr %2137, , !!8, !!8                                                                               ;L1774
 70897|     ;; self = ptr %2190
 70898|  %2191 = gep %2190, i64 1136                                                                                           ;L1511<1774
 70899|  %2192 = load i32, ptr %2191, , !!8                                                                                    ;L1511<1774
 70900|     ;; mult = i32 %2192
 70901|  %2193 = icmp eq i32 %2192, 0                                                                                          ;L1512<1774
 70902|  br i1 %2193, label %2194, label %2197                                                                                 ;L1512<1774
 70903| 
 70904| 2194: ; preds = %2189
 70905|  %2195 = gep %2190, i64 1664                                                                                           ;L1513<1774
 70906|  %2196 = load i64, ptr %2195, , !!8                                                                                    ;L1513<1774
 70907|  br label %2204                                                                                                        ;L1512<1774
 70908| 
 70909| 2197: ; preds = %2189
 70910|  %2198 = sext i32 %2192 to i64                                                                                         ;L1511<1774
 70911|     ;; mult = i64 %2198
 70912|  %2199 = gep %2190, i64 1664                                                                                           ;L1515<1774
 70913|  %2200 = load i64, ptr %2199, , !!8                                                                                    ;L1515<1774
 70914|  %2201 = add nsw i64 %2198, 100                                                                                        ;L1515<1774
 70915|  %2202 = mul i64 %2200, %2201                                                                                          ;L1515<1774
 70916|  %2203 = udiv i64 %2202, 100                                                                                           ;L1515<1774
 70917|  br label %2204                                                                                                        ;L1512<1774
 70918| 
 70919| 2204: ; preds = %2197, %2194
 70920|  %2205 = phi i64 [ %2196, %2194 ], [ %2203, %2197 ]                                                                    ;L0<1774
 70922|     ;; self = ptr %2190
 70923|  %2206 = gep %2190, i64 1632                                                                                           ;L2158<1775
 70924|  %2207 = load i64, ptr %2206, , !!8                                                                                    ;L2158<1775
 70925|     ;; x1 = i64 %2207
 70926|     ;; self = i64 %2207
 70927|  %2208 = gep %2190, i64 1640                                                                                           ;L2158<1775
 70928|  %2209 = load i64, ptr %2208, , !!8                                                                                    ;L2158<1775
 70929|     ;; y1 = i64 %2209
 70930|     ;; self = i64 %2209
 70931|  %2210 = load i64, ptr %1459, , !!8                                                                                    ;L2158<1775
 70932|     ;; x2 = i64 %2210
 70933|     ;; other = i64 %2210
 70934|  %2211 = load i64, ptr %1460, , !!8                                                                                    ;L2158<1775
 70935|     ;; y2 = i64 %2211
 70936|     ;; other = i64 %2211
 70937|  %2212 = icmp ult i64 %2207, %2210                                                                                     ;L3147<7<2158<1775
 70938|  %2213 = sub nuw i64 %2210, %2207                                                                                      ;L3147<7<2158<1775
 70939|  %2214 = sub nuw i64 %2207, %2210                                                                                      ;L3147<7<2158<1775
 70940|  %2215 = select i1 %2212, i64 %2213, i64 %2214                                                                         ;L3147<7<2158<1775
 70941|     ;; dx = i64 %2215
 70942|  %2216 = icmp ult i64 %2209, %2211                                                                                     ;L3147<8<2158<1775
 70943|  %2217 = sub nuw i64 %2211, %2209                                                                                      ;L3147<8<2158<1775
 70944|  %2218 = sub nuw i64 %2209, %2211                                                                                      ;L3147<8<2158<1775
 70945|  %2219 = select i1 %2216, i64 %2217, i64 %2218                                                                         ;L3147<8<2158<1775
 70946|     ;; dy = i64 %2219
 70947|  %2220 = mul i64 %2215, %2215                                                                                          ;L9<2158<1775
 70948|  %2221 = mul i64 %2219, %2219                                                                                          ;L9<2158<1775
 70949|  %2222 = add i64 %2221, %2220                                                                                          ;L9<2158<1775
 70950|     ;; dist_sq = i64 %2222
 70951|     ;; max_tick = i64 %97
 70952|  %2223 = add i64 %1462, %2188                                                                                          ;L1774
 70953|  %2224 = add i64 %2223, %2205                                                                                          ;L1777
 70954|     ;; max_dist = i64 %2224
 70955|  %2225 = mul i64 %2224, %2224                                                                                          ;L1778
 70956|  %2226 = icmp ugt i64 %2222, %2225                                                                                     ;L1778
 70957|  br i1 %2226, label %2184, label %2227                                                                                 ;L1778
 70958| 
 70959| 2227: ; preds = %2204
 70962|  %2228 = gep %2190, i64 1472                                                                                           ;L1779
 70963|  %2229 = load i64, ptr %2228, , !!8                                                                                    ;L1779
 70964|  invoke void @ai::small_action4castNtB2_17SmallActionAttack3new(ptr sret([24 x i8]) %34, ptr %3, i64 %2229)
 70965|  to label %2230 unwind label %178                                                                                      ;L1779
 70966| 
 70967| 2230: ; preds = %2227
 70968|  call void @llvm.memcpy.p0.p0.i64(ptr %35, ptr %34, i64 24, i1 false)                                                  ;L1779
 70969|  store i8 15, ptr %1463,                                                                                               ;L1779
 70972|     ;; self = ptr %65
 70973|     ;; self = ptr %65
 70974|     ;; value = ptr %35
 70975|     ;; src = ptr %35
 70976|     ;; additional = i64 1
 70977|     ;; needed_extra_cap = i64 1
 70978|     ;; needed_extra_cap = i64 1
 70979|     ;; strategy = i8 1
 70980|  %2231 = load i64, ptr %126, , !!75852, !!8                                                                            ;L1428<1779
 70981|     ;; self = ptr %65
 70982|  %2232 = load i64, ptr %125, , !!75852, !!8                                                                            ;L149<1428<1779
 70983|  %2233 = icmp eq i64 %2231, %2232                                                                                      ;L1428<1779
 70984|  br i1 %2233, label %2234, label %2239                                                                                 ;L1428<1779
 70985| 
 70986| 2234: ; preds = %2230
 70987|     ;; self = ptr %65
 70988|     ;; self = ptr %65
 70989|     ;; self = ptr %65
 70990|     ;; used_cap = i64 %2231
 70991|     ;; used_cap = i64 %2231
 70992|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %2231, i64 1, i1 zeroext true)
 70993|  to label %2235 unwind label %2237, !!75852                                                                            ;L619<430<738<1429<1779
 70994| 
 70995| 2235: ; preds = %2234
 70996|  %2236 = load i64, ptr %126, , !!75852                                                                                 ;L1432<1779
 70997|  br label %2239                                                                                                        ;L619<430<738<1429<1779
 70998| 
 70999| 2237: ; preds = %2234
 71000|  %2238 = cleanuppad within none []
 71001|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %35) #30 [ "funclet"(token %2238) ], !!75837 ;L1436<1779
 71002|  cleanupret from %2238 unwind label %178
 71003| 
 71004| 2239: ; preds = %2235, %2230
 71005|  %2240 = phi i64 [ %2236, %2235 ], [ %2231, %2230 ]                                                                    ;L1432<1779
 71006|     ;; self = ptr %65
 71007|  %2241 = load ptr, ptr %65, , !!75852, !!8, !!8                                                                        ;L138<1432<1779
 71008|     ;; self = ptr %2241
 71009|     ;; count = i64 %2240
 71010|  %2242 = gepS %2241, i64 %2240                                                                                         ;L961<1432<1779
 71011|     ;; end = ptr %2242
 71012|     ;; dst = ptr %2242
 71013|  call void @llvm.memcpy.p0.p0.i64(ptr %2242, ptr %35, i64 184, i1 false), !!75837                                      ;L1933<1433<1779
 71014|  %2243 = add i64 %2240, 1                                                                                              ;L1434<1779
 71015|  store i64 %2243, ptr %126, , !!75852                                                                                  ;L1434<1779
 71017|  br label %2184                                                                                                        ;L1778
 71018| 
 71019| 2244: ; preds = %2184
 71020|  %2245 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity9can_skill(ptr %86)
 71021|  to label %2247 unwind label %178                                                                                      ;L1783
 71022| 
 71023| 2246: ; preds = %2305, %2271, %2251, %2247, %2184
 71024|  br i1 %142, label %2312, label %2310                                                                                  ;L1789
 71025| 
 71026| 2247: ; preds = %2244
 71027|  br i1 %2245, label %2248, label %2246                                                                                 ;L1783
 71028| 
 71029| 2248: ; preds = %2247
 71030|  %2249 = load ptr, ptr %2137, , !!8, !!8                                                                               ;L1783
 71031|  %2250 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %1464, ptr %86, ptr %2249)
 71032|  to label %2251 unwind label %178                                                                                      ;L1783
 71033| 
 71034| 2251: ; preds = %2248
 71035|  br i1 %2250, label %2252, label %2246                                                                                 ;L1783
 71036| 
 71037| 2252: ; preds = %2251
 71038|  %2253 = load ptr, ptr %2137, , !!8, !!8                                                                               ;L1784
 71039|  %2254 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %134, ptr %86, ptr %2253)
 71040|  to label %2255 unwind label %178                                                                                      ;L1784
 71041| 
 71042| 2255: ; preds = %2252
 71043|  %2256 = add i64 %2254, %208                                                                                           ;L1784
 71044|  %2257 = load ptr, ptr %2137, , !!8, !!8                                                                               ;L1784
 71045|     ;; self = ptr %2257
 71046|  %2258 = gep %2257, i64 1136                                                                                           ;L1511<1784
 71047|  %2259 = load i32, ptr %2258, , !!8                                                                                    ;L1511<1784
 71048|     ;; mult = i32 %2259
 71049|  %2260 = icmp eq i32 %2259, 0                                                                                          ;L1512<1784
 71050|  br i1 %2260, label %2261, label %2264                                                                                 ;L1512<1784
 71051| 
 71052| 2261: ; preds = %2255
 71053|  %2262 = gep %2257, i64 1664                                                                                           ;L1513<1784
 71054|  %2263 = load i64, ptr %2262, , !!8                                                                                    ;L1513<1784
 71055|  br label %2271                                                                                                        ;L1512<1784
 71056| 
 71057| 2264: ; preds = %2255
 71058|  %2265 = sext i32 %2259 to i64                                                                                         ;L1511<1784
 71059|     ;; mult = i64 %2265
 71060|  %2266 = gep %2257, i64 1664                                                                                           ;L1515<1784
 71061|  %2267 = load i64, ptr %2266, , !!8                                                                                    ;L1515<1784
 71062|  %2268 = add nsw i64 %2265, 100                                                                                        ;L1515<1784
 71063|  %2269 = mul i64 %2267, %2268                                                                                          ;L1515<1784
 71064|  %2270 = udiv i64 %2269, 100                                                                                           ;L1515<1784
 71065|  br label %2271                                                                                                        ;L1512<1784
 71066| 
 71067| 2271: ; preds = %2264, %2261
 71068|  %2272 = phi i64 [ %2263, %2261 ], [ %2270, %2264 ]                                                                    ;L0<1784
 71069|  %2273 = add i64 %2256, %2272                                                                                          ;L1784
 71070|     ;; range = i64 %2273
 71071|     ;; self = ptr %2257
 71072|  %2274 = gep %2257, i64 1632                                                                                           ;L2158<1785
 71073|  %2275 = load i64, ptr %2274, , !!8                                                                                    ;L2158<1785
 71074|     ;; x1 = i64 %2275
 71075|     ;; self = i64 %2275
 71076|  %2276 = gep %2257, i64 1640                                                                                           ;L2158<1785
 71077|  %2277 = load i64, ptr %2276, , !!8                                                                                    ;L2158<1785
 71078|     ;; y1 = i64 %2277
 71079|     ;; self = i64 %2277
 71080|  %2278 = load i64, ptr %1459, , !!8                                                                                    ;L2158<1785
 71081|     ;; x2 = i64 %2278
 71082|     ;; other = i64 %2278
 71083|  %2279 = load i64, ptr %1460, , !!8                                                                                    ;L2158<1785
 71084|     ;; y2 = i64 %2279
 71085|     ;; other = i64 %2279
 71086|  %2280 = icmp ult i64 %2275, %2278                                                                                     ;L3147<7<2158<1785
 71087|  %2281 = sub nuw i64 %2278, %2275                                                                                      ;L3147<7<2158<1785
 71088|  %2282 = sub nuw i64 %2275, %2278                                                                                      ;L3147<7<2158<1785
 71089|  %2283 = select i1 %2280, i64 %2281, i64 %2282                                                                         ;L3147<7<2158<1785
 71090|     ;; dx = i64 %2283
 71091|  %2284 = icmp ult i64 %2277, %2279                                                                                     ;L3147<8<2158<1785
 71092|  %2285 = sub nuw i64 %2279, %2277                                                                                      ;L3147<8<2158<1785
 71093|  %2286 = sub nuw i64 %2277, %2279                                                                                      ;L3147<8<2158<1785
 71094|  %2287 = select i1 %2284, i64 %2285, i64 %2286                                                                         ;L3147<8<2158<1785
 71095|     ;; dy = i64 %2287
 71096|  %2288 = mul i64 %2283, %2283                                                                                          ;L9<2158<1785
 71097|  %2289 = mul i64 %2287, %2287                                                                                          ;L9<2158<1785
 71098|  %2290 = add i64 %2289, %2288                                                                                          ;L9<2158<1785
 71099|  %2291 = mul i64 %2273, %2273                                                                                          ;L1785
 71100|  %2292 = icmp ugt i64 %2290, %2291                                                                                     ;L1785
 71101|  br i1 %2292, label %2246, label %2293                                                                                 ;L1785
 71102| 
 71103| 2293: ; preds = %2271
 71106|  %2294 = gep %2257, i64 1472                                                                                           ;L1786
 71107|  %2295 = load i64, ptr %2294, , !!8                                                                                    ;L1786
 71108|  invoke void @ai::small_action4castNtB5_16SmallActionSkill3new(ptr sret([24 x i8]) %32, ptr %3, i64 %2295)
 71109|  to label %2296 unwind label %178                                                                                      ;L1786
 71110| 
 71111| 2296: ; preds = %2293
 71112|  call void @llvm.memcpy.p0.p0.i64(ptr %33, ptr %32, i64 24, i1 false)                                                  ;L1786
 71113|  store i8 16, ptr %1465,                                                                                               ;L1786
 71116|     ;; self = ptr %65
 71117|     ;; self = ptr %65
 71118|     ;; value = ptr %33
 71119|     ;; src = ptr %33
 71120|     ;; additional = i64 1
 71121|     ;; needed_extra_cap = i64 1
 71122|     ;; needed_extra_cap = i64 1
 71123|     ;; strategy = i8 1
 71124|  %2297 = load i64, ptr %126, , !!75911, !!8                                                                            ;L1428<1786
 71125|     ;; self = ptr %65
 71126|  %2298 = load i64, ptr %125, , !!75911, !!8                                                                            ;L149<1428<1786
 71127|  %2299 = icmp eq i64 %2297, %2298                                                                                      ;L1428<1786
 71128|  br i1 %2299, label %2300, label %2305                                                                                 ;L1428<1786
 71129| 
 71130| 2300: ; preds = %2296
 71131|     ;; self = ptr %65
 71132|     ;; self = ptr %65
 71133|     ;; self = ptr %65
 71134|     ;; used_cap = i64 %2297
 71135|     ;; used_cap = i64 %2297
 71136|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %2297, i64 1, i1 zeroext true)
 71137|  to label %2301 unwind label %2303, !!75911                                                                            ;L619<430<738<1429<1786
 71138| 
 71139| 2301: ; preds = %2300
 71140|  %2302 = load i64, ptr %126, , !!75911                                                                                 ;L1432<1786
 71141|  br label %2305                                                                                                        ;L619<430<738<1429<1786
 71142| 
 71143| 2303: ; preds = %2300
 71144|  %2304 = cleanuppad within none []
 71145|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %33) #30 [ "funclet"(token %2304) ], !!75896 ;L1436<1786
 71146|  cleanupret from %2304 unwind label %178
 71147| 
 71148| 2305: ; preds = %2301, %2296
 71149|  %2306 = phi i64 [ %2302, %2301 ], [ %2297, %2296 ]                                                                    ;L1432<1786
 71150|     ;; self = ptr %65
 71151|  %2307 = load ptr, ptr %65, , !!75911, !!8, !!8                                                                        ;L138<1432<1786
 71152|     ;; self = ptr %2307
 71153|     ;; count = i64 %2306
 71154|  %2308 = gepS %2307, i64 %2306                                                                                         ;L961<1432<1786
 71155|     ;; end = ptr %2308
 71156|     ;; dst = ptr %2308
 71157|  call void @llvm.memcpy.p0.p0.i64(ptr %2308, ptr %33, i64 184, i1 false), !!75896                                      ;L1933<1433<1786
 71158|  %2309 = add i64 %2306, 1                                                                                              ;L1434<1786
 71159|  store i64 %2309, ptr %126, , !!75911                                                                                  ;L1434<1786
 71161|  br label %2246                                                                                                        ;L1785
 71162| 
 71163| 2310: ; preds = %2246
 71164|  %2311 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_skill2(ptr %86)
 71165|  to label %2313 unwind label %178                                                                                      ;L1789
 71166| 
 71167| 2312: ; preds = %2371, %2337, %2317, %2313, %2246, %2172, %2167, %2161, %2151, %2145
 71168|  br label %2136                                                                                                        ;L1714<180<1761
 71169| 
 71170| 2313: ; preds = %2310
 71171|  br i1 %2311, label %2314, label %2312                                                                                 ;L1789
 71172| 
 71173| 2314: ; preds = %2313
 71174|  %2315 = load ptr, ptr %2137, , !!8, !!8                                                                               ;L1789
 71175|  %2316 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %1466, ptr %86, ptr %2315)
 71176|  to label %2317 unwind label %178                                                                                      ;L1789
 71177| 
 71178| 2317: ; preds = %2314
 71179|  br i1 %2316, label %2318, label %2312                                                                                 ;L1789
 71180| 
 71181| 2318: ; preds = %2317
 71182|  %2319 = load ptr, ptr %2137, , !!8, !!8                                                                               ;L1790
 71183|  %2320 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %139, ptr %86, ptr %2319)
 71184|  to label %2321 unwind label %178                                                                                      ;L1790
 71185| 
 71186| 2321: ; preds = %2318
 71187|  %2322 = add i64 %2320, %237                                                                                           ;L1790
 71188|  %2323 = load ptr, ptr %2137, , !!8, !!8                                                                               ;L1790
 71189|     ;; self = ptr %2323
 71190|  %2324 = gep %2323, i64 1136                                                                                           ;L1511<1790
 71191|  %2325 = load i32, ptr %2324, , !!8                                                                                    ;L1511<1790
 71192|     ;; mult = i32 %2325
 71193|  %2326 = icmp eq i32 %2325, 0                                                                                          ;L1512<1790
 71194|  br i1 %2326, label %2327, label %2330                                                                                 ;L1512<1790
 71195| 
 71196| 2327: ; preds = %2321
 71197|  %2328 = gep %2323, i64 1664                                                                                           ;L1513<1790
 71198|  %2329 = load i64, ptr %2328, , !!8                                                                                    ;L1513<1790
 71199|  br label %2337                                                                                                        ;L1512<1790
 71200| 
 71201| 2330: ; preds = %2321
 71202|  %2331 = sext i32 %2325 to i64                                                                                         ;L1511<1790
 71203|     ;; mult = i64 %2331
 71204|  %2332 = gep %2323, i64 1664                                                                                           ;L1515<1790
 71205|  %2333 = load i64, ptr %2332, , !!8                                                                                    ;L1515<1790
 71206|  %2334 = add nsw i64 %2331, 100                                                                                        ;L1515<1790
 71207|  %2335 = mul i64 %2333, %2334                                                                                          ;L1515<1790
 71208|  %2336 = udiv i64 %2335, 100                                                                                           ;L1515<1790
 71209|  br label %2337                                                                                                        ;L1512<1790
 71210| 
 71211| 2337: ; preds = %2330, %2327
 71212|  %2338 = phi i64 [ %2329, %2327 ], [ %2336, %2330 ]                                                                    ;L0<1790
 71213|  %2339 = add i64 %2322, %2338                                                                                          ;L1790
 71214|     ;; range = i64 %2339
 71215|     ;; self = ptr %2323
 71216|  %2340 = gep %2323, i64 1632                                                                                           ;L2158<1791
 71217|  %2341 = load i64, ptr %2340, , !!8                                                                                    ;L2158<1791
 71218|     ;; x1 = i64 %2341
 71219|     ;; self = i64 %2341
 71220|  %2342 = gep %2323, i64 1640                                                                                           ;L2158<1791
 71221|  %2343 = load i64, ptr %2342, , !!8                                                                                    ;L2158<1791
 71222|     ;; y1 = i64 %2343
 71223|     ;; self = i64 %2343
 71224|  %2344 = load i64, ptr %1459, , !!8                                                                                    ;L2158<1791
 71225|     ;; x2 = i64 %2344
 71226|     ;; other = i64 %2344
 71227|  %2345 = load i64, ptr %1460, , !!8                                                                                    ;L2158<1791
 71228|     ;; y2 = i64 %2345
 71229|     ;; other = i64 %2345
 71230|  %2346 = icmp ult i64 %2341, %2344                                                                                     ;L3147<7<2158<1791
 71231|  %2347 = sub nuw i64 %2344, %2341                                                                                      ;L3147<7<2158<1791
 71232|  %2348 = sub nuw i64 %2341, %2344                                                                                      ;L3147<7<2158<1791
 71233|  %2349 = select i1 %2346, i64 %2347, i64 %2348                                                                         ;L3147<7<2158<1791
 71234|     ;; dx = i64 %2349
 71235|  %2350 = icmp ult i64 %2343, %2345                                                                                     ;L3147<8<2158<1791
 71236|  %2351 = sub nuw i64 %2345, %2343                                                                                      ;L3147<8<2158<1791
 71237|  %2352 = sub nuw i64 %2343, %2345                                                                                      ;L3147<8<2158<1791
 71238|  %2353 = select i1 %2350, i64 %2351, i64 %2352                                                                         ;L3147<8<2158<1791
 71239|     ;; dy = i64 %2353
 71240|  %2354 = mul i64 %2349, %2349                                                                                          ;L9<2158<1791
 71241|  %2355 = mul i64 %2353, %2353                                                                                          ;L9<2158<1791
 71242|  %2356 = add i64 %2355, %2354                                                                                          ;L9<2158<1791
 71243|  %2357 = mul i64 %2339, %2339                                                                                          ;L1791
 71244|  %2358 = icmp ugt i64 %2356, %2357                                                                                     ;L1791
 71245|  br i1 %2358, label %2312, label %2359                                                                                 ;L1791
 71246| 
 71247| 2359: ; preds = %2337
 71250|  %2360 = gep %2323, i64 1472                                                                                           ;L1792
 71251|  %2361 = load i64, ptr %2360, , !!8                                                                                    ;L1792
 71252|  invoke void @ai::small_action4castNtB5_17SmallActionSkill23new(ptr sret([24 x i8]) %30, ptr %3, i64 %2361)
 71253|  to label %2362 unwind label %178                                                                                      ;L1792
 71254| 
 71255| 2362: ; preds = %2359
 71256|  call void @llvm.memcpy.p0.p0.i64(ptr %31, ptr %30, i64 24, i1 false)                                                  ;L1792
 71257|  store i8 17, ptr %1467,                                                                                               ;L1792
 71260|     ;; self = ptr %65
 71261|     ;; self = ptr %65
 71262|     ;; value = ptr %31
 71263|     ;; src = ptr %31
 71264|     ;; additional = i64 1
 71265|     ;; needed_extra_cap = i64 1
 71266|     ;; needed_extra_cap = i64 1
 71267|     ;; strategy = i8 1
 71268|  %2363 = load i64, ptr %126, , !!75969, !!8                                                                            ;L1428<1792
 71269|     ;; self = ptr %65
 71270|  %2364 = load i64, ptr %125, , !!75969, !!8                                                                            ;L149<1428<1792
 71271|  %2365 = icmp eq i64 %2363, %2364                                                                                      ;L1428<1792
 71272|  br i1 %2365, label %2366, label %2371                                                                                 ;L1428<1792
 71273| 
 71274| 2366: ; preds = %2362
 71275|     ;; self = ptr %65
 71276|     ;; self = ptr %65
 71277|     ;; self = ptr %65
 71278|     ;; used_cap = i64 %2363
 71279|     ;; used_cap = i64 %2363
 71280|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %2363, i64 1, i1 zeroext true)
 71281|  to label %2367 unwind label %2369, !!75969                                                                            ;L619<430<738<1429<1792
 71282| 
 71283| 2367: ; preds = %2366
 71284|  %2368 = load i64, ptr %126, , !!75969                                                                                 ;L1432<1792
 71285|  br label %2371                                                                                                        ;L619<430<738<1429<1792
 71286| 
 71287| 2369: ; preds = %2366
 71288|  %2370 = cleanuppad within none []
 71289|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %31) #30 [ "funclet"(token %2370) ], !!75954 ;L1436<1792
 71290|  cleanupret from %2370 unwind label %178
 71291| 
 71292| 2371: ; preds = %2367, %2362
 71293|  %2372 = phi i64 [ %2368, %2367 ], [ %2363, %2362 ]                                                                    ;L1432<1792
 71294|     ;; self = ptr %65
 71295|  %2373 = load ptr, ptr %65, , !!75969, !!8, !!8                                                                        ;L138<1432<1792
 71296|     ;; self = ptr %2373
 71297|     ;; count = i64 %2372
 71298|  %2374 = gepS %2373, i64 %2372                                                                                         ;L961<1432<1792
 71299|     ;; end = ptr %2374
 71300|     ;; dst = ptr %2374
 71301|  call void @llvm.memcpy.p0.p0.i64(ptr %2374, ptr %31, i64 184, i1 false), !!75954                                      ;L1933<1433<1792
 71302|  %2375 = add i64 %2372, 1                                                                                              ;L1434<1792
 71303|  store i64 %2375, ptr %126, , !!75969                                                                                  ;L1434<1792
 71305|  br label %2312                                                                                                        ;L1791
 71306| 
 71307| 2376: ; preds = %2554, %2139
 71308|  %2377 = phi ptr [ %1454, %2139 ], [ %2380, %2554 ]                                                                    ;L1798
 71309|     ;; iter[0..+8] = ptr %2377
 71310|     ;; self = ptr undef
 71311|     ;; ptr = ptr %2377
 71312|     ;; self = ptr %2377
 71313|     ;; end_or_len = ptr %1457
 71316|  %2378 = icmp eq ptr %2377, %1457                                                                                      ;L1714<180<1798
 71317|  br i1 %2378, label %2625, label %2379                                                                                 ;L180<1798
 71318| 
 71319| 2379: ; preds = %2376
 71320|  %2380 = gep %2377, i64 8                                                                                              ;L656<185<1798
 71321|     ;; iter[0..+8] = ptr %2380
 71322|     ;; e = ptr %2377
 71323|  %2381 = load ptr, ptr %2377, , !!8, !!8                                                                               ;L1799
 71324|     ;; self = ptr %2381
 71325|  %2382 = gep %2381, i64 1721                                                                                           ;L1478<1799
 71326|  %2383 = load i8, ptr %2382, , !!8                                                                                     ;L1478<1799
 71327|  %2384 = trunc nuw i8 %2383 to i1                                                                                      ;L1478<1799
 71328|  br i1 %2384, label %2385, label %2554                                                                                 ;L1478<1799
 71329| 
 71330| 2385: ; preds = %2379
 71331|  %2386 = gep %2381, i64 1696                                                                                           ;L1478<1799
 71332|  %2387 = load i64, ptr %2386, , !!8                                                                                    ;L1478<1799
 71333|  %2388 = icmp eq i64 %2387, 0                                                                                          ;L1478<1799
 71334|  br i1 %2388, label %2389, label %2554                                                                                 ;L1799
 71335| 
 71336| 2389: ; preds = %2385
 71337|     ;; self = ptr %2381
 71338|     ;; self = ptr %86
 71339|  %2390 = load i64, ptr %86, , !!8                                                                                      ;L1136<1482<1799
 71340|  %2391 = trunc nuw i64 %2390 to i1                                                                                     ;L1136<1482<1799
 71341|  br i1 %2391, label %2401, label %2392                                                                                 ;L1136<1482<1799
 71342| 
 71343| 2392: ; preds = %2389
 71344|     ;; team = ptr %86
 71345|  %2393 = load i64, ptr %1458, , !!8                                                                                    ;L1137<1482<1799
 71346|     ;; team = i64 %2393
 71347|  %2394 = icmp ult i64 %2393, 2                                                                                         ;L1483<1799
 71348|  br i1 %2394, label %2395, label %2400                                                                                 ;L1483<1799
 71349| 
 71350| 2395: ; preds = %2392
 71352|  %2396 = gep %2381, i64 56                                                                                             ;L122<1483<1799
 71353|  %2397 = gepS %2396, i64 %2393                                                                                         ;L122<1483<1799
 71354|  %2398 = load i64, ptr %2397, , !!8                                                                                    ;L122<1483<1799
 71355|  %2399 = icmp eq i64 %2398, 0                                                                                          ;L122<1483<1799
 71356|  br i1 %2399, label %2401, label %2554                                                                                 ;L1799
 71357| 
 71358| 2400: ; preds = %2392
 71359|  invoke void @core::panicking18panic_bounds_check(i64 %2393, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 71360|  to label %118 unwind label %178                                                                                       ;L1483<1799
 71361| 
 71362| 2401: ; preds = %2395, %2389
 71363|     ;; self = ptr %2381
 71364|  %2402 = gep %2381, i64 104                                                                                            ;L1370<1803
 71365|  %2403 = load i64, ptr %2402, , !!8                                                                                    ;L1370<1803
 71366|  %2404 = icmp eq i64 %2403, 4                                                                                          ;L1803
 71367|  br i1 %2404, label %2405, label %2554                                                                                 ;L1803
 71368| 
 71369| 2405: ; preds = %2401
 71370|  %2406 = gep %2381, i64 152                                                                                            ;L1378<1803
 71371|  %2407 = load i64, ptr %2406, , !!8                                                                                    ;L1378<1803
 71372|  %2408 = icmp eq i64 %2407, %104                                                                                       ;L1378<1803
 71373|  br i1 %2408, label %2409, label %2554                                                                                 ;L1378<1803
 71374| 
 71375| 2409: ; preds = %2405
 71376|  br i1 %130, label %2412, label %2410                                                                                  ;L1807
 71377| 
 71378| 2410: ; preds = %2409
 71379|  %2411 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_attack(ptr %86)
 71380|  to label %2413 unwind label %178                                                                                      ;L1807
 71381| 
 71382| 2412: ; preds = %2474, %2439, %2417, %2413, %2409
 71383|  br i1 %133, label %2481, label %2479                                                                                  ;L1817
 71384| 
 71385| 2413: ; preds = %2410
 71386|  br i1 %2411, label %2414, label %2412                                                                                 ;L1807
 71387| 
 71388| 2414: ; preds = %2413
 71389|  %2415 = load ptr, ptr %2377, , !!8, !!8                                                                               ;L1808
 71390|  %2416 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %127, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %2415)
 71391|  to label %2417 unwind label %178                                                                                      ;L1808
 71392| 
 71393| 2417: ; preds = %2414
 71394|     ;; expected_dmg = i64 %2416
 71395|  %2418 = load ptr, ptr %2377, , !!8, !!8                                                                               ;L1809
 71396|  %2419 = gep %2418, i64 1648                                                                                           ;L1809
 71397|  %2420 = load i64, ptr %2419, , !!8                                                                                    ;L1809
 71398|  %2421 = icmp ugt i64 %2420, %2416                                                                                     ;L1809
 71399|  br i1 %2421, label %2412, label %2422                                                                                 ;L1809
 71400| 
 71401| 2422: ; preds = %2417
 71402|  %2423 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %127, ptr %86, ptr %2418)
 71403|  to label %2424 unwind label %178                                                                                      ;L1810
 71404| 
 71405| 2424: ; preds = %2422
 71406|  %2425 = load ptr, ptr %2377, , !!8, !!8                                                                               ;L1810
 71407|     ;; self = ptr %2425
 71408|  %2426 = gep %2425, i64 1136                                                                                           ;L1511<1810
 71409|  %2427 = load i32, ptr %2426, , !!8                                                                                    ;L1511<1810
 71410|     ;; mult = i32 %2427
 71411|  %2428 = icmp eq i32 %2427, 0                                                                                          ;L1512<1810
 71412|  br i1 %2428, label %2429, label %2432                                                                                 ;L1512<1810
 71413| 
 71414| 2429: ; preds = %2424
 71415|  %2430 = gep %2425, i64 1664                                                                                           ;L1513<1810
 71416|  %2431 = load i64, ptr %2430, , !!8                                                                                    ;L1513<1810
 71417|  br label %2439                                                                                                        ;L1512<1810
 71418| 
 71419| 2432: ; preds = %2424
 71420|  %2433 = sext i32 %2427 to i64                                                                                         ;L1511<1810
 71421|     ;; mult = i64 %2433
 71422|  %2434 = gep %2425, i64 1664                                                                                           ;L1515<1810
 71423|  %2435 = load i64, ptr %2434, , !!8                                                                                    ;L1515<1810
 71424|  %2436 = add nsw i64 %2433, 100                                                                                        ;L1515<1810
 71425|  %2437 = mul i64 %2435, %2436                                                                                          ;L1515<1810
 71426|  %2438 = udiv i64 %2437, 100                                                                                           ;L1515<1810
 71427|  br label %2439                                                                                                        ;L1512<1810
 71428| 
 71429| 2439: ; preds = %2432, %2429
 71430|  %2440 = phi i64 [ %2431, %2429 ], [ %2438, %2432 ]                                                                    ;L0<1810
 71432|  %2441 = add i64 %2141, %2423                                                                                          ;L1810
 71433|  %2442 = add i64 %2441, %2440                                                                                          ;L1811
 71434|     ;; max_dist = i64 %2442
 71435|     ;; self = ptr %2425
 71436|  %2443 = gep %2425, i64 1632                                                                                           ;L2158<1812
 71437|  %2444 = load i64, ptr %2443, , !!8                                                                                    ;L2158<1812
 71438|     ;; x1 = i64 %2444
 71439|     ;; self = i64 %2444
 71440|  %2445 = gep %2425, i64 1640                                                                                           ;L2158<1812
 71441|  %2446 = load i64, ptr %2445, , !!8                                                                                    ;L2158<1812
 71442|     ;; y1 = i64 %2446
 71443|     ;; self = i64 %2446
 71444|  %2447 = load i64, ptr %1459, , !!8                                                                                    ;L2158<1812
 71445|     ;; x2 = i64 %2447
 71446|     ;; other = i64 %2447
 71447|  %2448 = load i64, ptr %1460, , !!8                                                                                    ;L2158<1812
 71448|     ;; y2 = i64 %2448
 71449|     ;; other = i64 %2448
 71450|  %2449 = icmp ult i64 %2444, %2447                                                                                     ;L3147<7<2158<1812
 71451|  %2450 = sub nuw i64 %2447, %2444                                                                                      ;L3147<7<2158<1812
 71452|  %2451 = sub nuw i64 %2444, %2447                                                                                      ;L3147<7<2158<1812
 71453|  %2452 = select i1 %2449, i64 %2450, i64 %2451                                                                         ;L3147<7<2158<1812
 71454|     ;; dx = i64 %2452
 71455|  %2453 = icmp ult i64 %2446, %2448                                                                                     ;L3147<8<2158<1812
 71456|  %2454 = sub nuw i64 %2448, %2446                                                                                      ;L3147<8<2158<1812
 71457|  %2455 = sub nuw i64 %2446, %2448                                                                                      ;L3147<8<2158<1812
 71458|  %2456 = select i1 %2453, i64 %2454, i64 %2455                                                                         ;L3147<8<2158<1812
 71459|     ;; dy = i64 %2456
 71460|  %2457 = mul i64 %2452, %2452                                                                                          ;L9<2158<1812
 71461|  %2458 = mul i64 %2456, %2456                                                                                          ;L9<2158<1812
 71462|  %2459 = add i64 %2458, %2457                                                                                          ;L9<2158<1812
 71463|  %2460 = mul i64 %2442, %2442                                                                                          ;L1812
 71464|  %2461 = icmp ugt i64 %2459, %2460                                                                                     ;L1812
 71465|  br i1 %2461, label %2412, label %2462                                                                                 ;L1812
 71466| 
 71467| 2462: ; preds = %2439
 71470|  %2463 = gep %2425, i64 1472                                                                                           ;L1813
 71471|  %2464 = load i64, ptr %2463, , !!8                                                                                    ;L1813
 71472|  invoke void @ai::small_action4castNtB2_17SmallActionAttack3new(ptr sret([24 x i8]) %28, ptr %3, i64 %2464)
 71473|  to label %2465 unwind label %178                                                                                      ;L1813
 71474| 
 71475| 2465: ; preds = %2462
 71476|  call void @llvm.memcpy.p0.p0.i64(ptr %29, ptr %28, i64 24, i1 false)                                                  ;L1813
 71477|  store i8 15, ptr %2142,                                                                                               ;L1813
 71480|     ;; self = ptr %65
 71481|     ;; self = ptr %65
 71482|     ;; value = ptr %29
 71483|     ;; src = ptr %29
 71484|     ;; additional = i64 1
 71485|     ;; needed_extra_cap = i64 1
 71486|     ;; needed_extra_cap = i64 1
 71487|     ;; strategy = i8 1
 71488|  %2466 = load i64, ptr %126, , !!76057, !!8                                                                            ;L1428<1813
 71489|     ;; self = ptr %65
 71490|  %2467 = load i64, ptr %125, , !!76057, !!8                                                                            ;L149<1428<1813
 71491|  %2468 = icmp eq i64 %2466, %2467                                                                                      ;L1428<1813
 71492|  br i1 %2468, label %2469, label %2474                                                                                 ;L1428<1813
 71493| 
 71494| 2469: ; preds = %2465
 71495|     ;; self = ptr %65
 71496|     ;; self = ptr %65
 71497|     ;; self = ptr %65
 71498|     ;; used_cap = i64 %2466
 71499|     ;; used_cap = i64 %2466
 71500|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %2466, i64 1, i1 zeroext true)
 71501|  to label %2470 unwind label %2472, !!76057                                                                            ;L619<430<738<1429<1813
 71502| 
 71503| 2470: ; preds = %2469
 71504|  %2471 = load i64, ptr %126, , !!76057                                                                                 ;L1432<1813
 71505|  br label %2474                                                                                                        ;L619<430<738<1429<1813
 71506| 
 71507| 2472: ; preds = %2469
 71508|  %2473 = cleanuppad within none []
 71509|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %29) #30 [ "funclet"(token %2473) ], !!76042 ;L1436<1813
 71510|  cleanupret from %2473 unwind label %178
 71511| 
 71512| 2474: ; preds = %2470, %2465
 71513|  %2475 = phi i64 [ %2471, %2470 ], [ %2466, %2465 ]                                                                    ;L1432<1813
 71514|     ;; self = ptr %65
 71515|  %2476 = load ptr, ptr %65, , !!76057, !!8, !!8                                                                        ;L138<1432<1813
 71516|     ;; self = ptr %2476
 71517|     ;; count = i64 %2475
 71518|  %2477 = gepS %2476, i64 %2475                                                                                         ;L961<1432<1813
 71519|     ;; end = ptr %2477
 71520|     ;; dst = ptr %2477
 71521|  call void @llvm.memcpy.p0.p0.i64(ptr %2477, ptr %29, i64 184, i1 false), !!76042                                      ;L1933<1433<1813
 71522|  %2478 = add i64 %2475, 1                                                                                              ;L1434<1813
 71523|  store i64 %2478, ptr %126, , !!76057                                                                                  ;L1434<1813
 71525|  br label %2412                                                                                                        ;L1812
 71526| 
 71527| 2479: ; preds = %2412
 71528|  %2480 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity9can_skill(ptr %86)
 71529|  to label %2482 unwind label %178                                                                                      ;L1817
 71530| 
 71531| 2481: ; preds = %2547, %2513, %2490, %2486, %2482, %2412
 71532|  br i1 %142, label %2554, label %2552                                                                                  ;L1826
 71533| 
 71534| 2482: ; preds = %2479
 71535|  br i1 %2480, label %2483, label %2481                                                                                 ;L1817
 71536| 
 71537| 2483: ; preds = %2482
 71538|  %2484 = load ptr, ptr %2377, , !!8, !!8                                                                               ;L1817
 71539|  %2485 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %1464, ptr %86, ptr %2484)
 71540|  to label %2486 unwind label %178                                                                                      ;L1817
 71541| 
 71542| 2486: ; preds = %2483
 71543|  br i1 %2485, label %2487, label %2481                                                                                 ;L1817
 71544| 
 71545| 2487: ; preds = %2486
 71546|  %2488 = load ptr, ptr %2377, , !!8, !!8                                                                               ;L1818
 71547|  %2489 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %134, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %2488)
 71548|  to label %2490 unwind label %178                                                                                      ;L1818
 71549| 
 71550| 2490: ; preds = %2487
 71551|     ;; expected_dmg = i64 %2489
 71552|  %2491 = load ptr, ptr %2377, , !!8, !!8                                                                               ;L1819
 71553|  %2492 = gep %2491, i64 1648                                                                                           ;L1819
 71554|  %2493 = load i64, ptr %2492, , !!8                                                                                    ;L1819
 71555|  %2494 = icmp ugt i64 %2493, %2489                                                                                     ;L1819
 71556|  br i1 %2494, label %2481, label %2495                                                                                 ;L1819
 71557| 
 71558| 2495: ; preds = %2490
 71559|  %2496 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %134, ptr %86, ptr %2491)
 71560|  to label %2497 unwind label %178                                                                                      ;L1820
 71561| 
 71562| 2497: ; preds = %2495
 71563|  %2498 = add i64 %2496, %208                                                                                           ;L1820
 71564|  %2499 = load ptr, ptr %2377, , !!8, !!8                                                                               ;L1820
 71565|     ;; self = ptr %2499
 71566|  %2500 = gep %2499, i64 1136                                                                                           ;L1511<1820
 71567|  %2501 = load i32, ptr %2500, , !!8                                                                                    ;L1511<1820
 71568|     ;; mult = i32 %2501
 71569|  %2502 = icmp eq i32 %2501, 0                                                                                          ;L1512<1820
 71570|  br i1 %2502, label %2503, label %2506                                                                                 ;L1512<1820
 71571| 
 71572| 2503: ; preds = %2497
 71573|  %2504 = gep %2499, i64 1664                                                                                           ;L1513<1820
 71574|  %2505 = load i64, ptr %2504, , !!8                                                                                    ;L1513<1820
 71575|  br label %2513                                                                                                        ;L1512<1820
 71576| 
 71577| 2506: ; preds = %2497
 71578|  %2507 = sext i32 %2501 to i64                                                                                         ;L1511<1820
 71579|     ;; mult = i64 %2507
 71580|  %2508 = gep %2499, i64 1664                                                                                           ;L1515<1820
 71581|  %2509 = load i64, ptr %2508, , !!8                                                                                    ;L1515<1820
 71582|  %2510 = add nsw i64 %2507, 100                                                                                        ;L1515<1820
 71583|  %2511 = mul i64 %2509, %2510                                                                                          ;L1515<1820
 71584|  %2512 = udiv i64 %2511, 100                                                                                           ;L1515<1820
 71585|  br label %2513                                                                                                        ;L1512<1820
 71586| 
 71587| 2513: ; preds = %2506, %2503
 71588|  %2514 = phi i64 [ %2505, %2503 ], [ %2512, %2506 ]                                                                    ;L0<1820
 71589|  %2515 = add i64 %2498, %2514                                                                                          ;L1820
 71590|     ;; range = i64 %2515
 71591|     ;; self = ptr %2499
 71592|  %2516 = gep %2499, i64 1632                                                                                           ;L2158<1821
 71593|  %2517 = load i64, ptr %2516, , !!8                                                                                    ;L2158<1821
 71594|     ;; x1 = i64 %2517
 71595|     ;; self = i64 %2517
 71596|  %2518 = gep %2499, i64 1640                                                                                           ;L2158<1821
 71597|  %2519 = load i64, ptr %2518, , !!8                                                                                    ;L2158<1821
 71598|     ;; y1 = i64 %2519
 71599|     ;; self = i64 %2519
 71600|  %2520 = load i64, ptr %1459, , !!8                                                                                    ;L2158<1821
 71601|     ;; x2 = i64 %2520
 71602|     ;; other = i64 %2520
 71603|  %2521 = load i64, ptr %1460, , !!8                                                                                    ;L2158<1821
 71604|     ;; y2 = i64 %2521
 71605|     ;; other = i64 %2521
 71606|  %2522 = icmp ult i64 %2517, %2520                                                                                     ;L3147<7<2158<1821
 71607|  %2523 = sub nuw i64 %2520, %2517                                                                                      ;L3147<7<2158<1821
 71608|  %2524 = sub nuw i64 %2517, %2520                                                                                      ;L3147<7<2158<1821
 71609|  %2525 = select i1 %2522, i64 %2523, i64 %2524                                                                         ;L3147<7<2158<1821
 71610|     ;; dx = i64 %2525
 71611|  %2526 = icmp ult i64 %2519, %2521                                                                                     ;L3147<8<2158<1821
 71612|  %2527 = sub nuw i64 %2521, %2519                                                                                      ;L3147<8<2158<1821
 71613|  %2528 = sub nuw i64 %2519, %2521                                                                                      ;L3147<8<2158<1821
 71614|  %2529 = select i1 %2526, i64 %2527, i64 %2528                                                                         ;L3147<8<2158<1821
 71615|     ;; dy = i64 %2529
 71616|  %2530 = mul i64 %2525, %2525                                                                                          ;L9<2158<1821
 71617|  %2531 = mul i64 %2529, %2529                                                                                          ;L9<2158<1821
 71618|  %2532 = add i64 %2531, %2530                                                                                          ;L9<2158<1821
 71619|  %2533 = mul i64 %2515, %2515                                                                                          ;L1821
 71620|  %2534 = icmp ugt i64 %2532, %2533                                                                                     ;L1821
 71621|  br i1 %2534, label %2481, label %2535                                                                                 ;L1821
 71622| 
 71623| 2535: ; preds = %2513
 71626|  %2536 = gep %2499, i64 1472                                                                                           ;L1822
 71627|  %2537 = load i64, ptr %2536, , !!8                                                                                    ;L1822
 71628|  invoke void @ai::small_action4castNtB5_16SmallActionSkill3new(ptr sret([24 x i8]) %26, ptr %3, i64 %2537)
 71629|  to label %2538 unwind label %178                                                                                      ;L1822
 71630| 
 71631| 2538: ; preds = %2535
 71632|  call void @llvm.memcpy.p0.p0.i64(ptr %27, ptr %26, i64 24, i1 false)                                                  ;L1822
 71633|  store i8 16, ptr %2143,                                                                                               ;L1822
 71636|     ;; self = ptr %65
 71637|     ;; self = ptr %65
 71638|     ;; value = ptr %27
 71639|     ;; src = ptr %27
 71640|     ;; additional = i64 1
 71641|     ;; needed_extra_cap = i64 1
 71642|     ;; needed_extra_cap = i64 1
 71643|     ;; strategy = i8 1
 71644|  %2539 = load i64, ptr %126, , !!76119, !!8                                                                            ;L1428<1822
 71645|     ;; self = ptr %65
 71646|  %2540 = load i64, ptr %125, , !!76119, !!8                                                                            ;L149<1428<1822
 71647|  %2541 = icmp eq i64 %2539, %2540                                                                                      ;L1428<1822
 71648|  br i1 %2541, label %2542, label %2547                                                                                 ;L1428<1822
 71649| 
 71650| 2542: ; preds = %2538
 71651|     ;; self = ptr %65
 71652|     ;; self = ptr %65
 71653|     ;; self = ptr %65
 71654|     ;; used_cap = i64 %2539
 71655|     ;; used_cap = i64 %2539
 71656|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %2539, i64 1, i1 zeroext true)
 71657|  to label %2543 unwind label %2545, !!76119                                                                            ;L619<430<738<1429<1822
 71658| 
 71659| 2543: ; preds = %2542
 71660|  %2544 = load i64, ptr %126, , !!76119                                                                                 ;L1432<1822
 71661|  br label %2547                                                                                                        ;L619<430<738<1429<1822
 71662| 
 71663| 2545: ; preds = %2542
 71664|  %2546 = cleanuppad within none []
 71665|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %27) #30 [ "funclet"(token %2546) ], !!76104 ;L1436<1822
 71666|  cleanupret from %2546 unwind label %178
 71667| 
 71668| 2547: ; preds = %2543, %2538
 71669|  %2548 = phi i64 [ %2544, %2543 ], [ %2539, %2538 ]                                                                    ;L1432<1822
 71670|     ;; self = ptr %65
 71671|  %2549 = load ptr, ptr %65, , !!76119, !!8, !!8                                                                        ;L138<1432<1822
 71672|     ;; self = ptr %2549
 71673|     ;; count = i64 %2548
 71674|  %2550 = gepS %2549, i64 %2548                                                                                         ;L961<1432<1822
 71675|     ;; end = ptr %2550
 71676|     ;; dst = ptr %2550
 71677|  call void @llvm.memcpy.p0.p0.i64(ptr %2550, ptr %27, i64 184, i1 false), !!76104                                      ;L1933<1433<1822
 71678|  %2551 = add i64 %2548, 1                                                                                              ;L1434<1822
 71679|  store i64 %2551, ptr %126, , !!76119                                                                                  ;L1434<1822
 71681|  br label %2481                                                                                                        ;L1821
 71682| 
 71683| 2552: ; preds = %2481
 71684|  %2553 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_skill2(ptr %86)
 71685|  to label %2555 unwind label %178                                                                                      ;L1826
 71686| 
 71687| 2554: ; preds = %2620, %2586, %2563, %2559, %2555, %2481, %2405, %2401, %2395, %2385, %2379
 71688|  br label %2376                                                                                                        ;L1714<180<1798
 71689| 
 71690| 2555: ; preds = %2552
 71691|  br i1 %2553, label %2556, label %2554                                                                                 ;L1826
 71692| 
 71693| 2556: ; preds = %2555
 71694|  %2557 = load ptr, ptr %2377, , !!8, !!8                                                                               ;L1826
 71695|  %2558 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %1466, ptr %86, ptr %2557)
 71696|  to label %2559 unwind label %178                                                                                      ;L1826
 71697| 
 71698| 2559: ; preds = %2556
 71699|  br i1 %2558, label %2560, label %2554                                                                                 ;L1826
 71700| 
 71701| 2560: ; preds = %2559
 71702|  %2561 = load ptr, ptr %2377, , !!8, !!8                                                                               ;L1827
 71703|  %2562 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %139, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %2561)
 71704|  to label %2563 unwind label %178                                                                                      ;L1827
 71705| 
 71706| 2563: ; preds = %2560
 71707|     ;; expected_dmg = i64 %2562
 71708|  %2564 = load ptr, ptr %2377, , !!8, !!8                                                                               ;L1828
 71709|  %2565 = gep %2564, i64 1648                                                                                           ;L1828
 71710|  %2566 = load i64, ptr %2565, , !!8                                                                                    ;L1828
 71711|  %2567 = icmp ugt i64 %2566, %2562                                                                                     ;L1828
 71712|  br i1 %2567, label %2554, label %2568                                                                                 ;L1828
 71713| 
 71714| 2568: ; preds = %2563
 71715|  %2569 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %139, ptr %86, ptr %2564)
 71716|  to label %2570 unwind label %178                                                                                      ;L1829
 71717| 
 71718| 2570: ; preds = %2568
 71719|  %2571 = add i64 %2569, %237                                                                                           ;L1829
 71720|  %2572 = load ptr, ptr %2377, , !!8, !!8                                                                               ;L1829
 71721|     ;; self = ptr %2572
 71722|  %2573 = gep %2572, i64 1136                                                                                           ;L1511<1829
 71723|  %2574 = load i32, ptr %2573, , !!8                                                                                    ;L1511<1829
 71724|     ;; mult = i32 %2574
 71725|  %2575 = icmp eq i32 %2574, 0                                                                                          ;L1512<1829
 71726|  br i1 %2575, label %2576, label %2579                                                                                 ;L1512<1829
 71727| 
 71728| 2576: ; preds = %2570
 71729|  %2577 = gep %2572, i64 1664                                                                                           ;L1513<1829
 71730|  %2578 = load i64, ptr %2577, , !!8                                                                                    ;L1513<1829
 71731|  br label %2586                                                                                                        ;L1512<1829
 71732| 
 71733| 2579: ; preds = %2570
 71734|  %2580 = sext i32 %2574 to i64                                                                                         ;L1511<1829
 71735|     ;; mult = i64 %2580
 71736|  %2581 = gep %2572, i64 1664                                                                                           ;L1515<1829
 71737|  %2582 = load i64, ptr %2581, , !!8                                                                                    ;L1515<1829
 71738|  %2583 = add nsw i64 %2580, 100                                                                                        ;L1515<1829
 71739|  %2584 = mul i64 %2582, %2583                                                                                          ;L1515<1829
 71740|  %2585 = udiv i64 %2584, 100                                                                                           ;L1515<1829
 71741|  br label %2586                                                                                                        ;L1512<1829
 71742| 
 71743| 2586: ; preds = %2579, %2576
 71744|  %2587 = phi i64 [ %2578, %2576 ], [ %2585, %2579 ]                                                                    ;L0<1829
 71745|  %2588 = add i64 %2571, %2587                                                                                          ;L1829
 71746|     ;; range = i64 %2588
 71747|     ;; self = ptr %2572
 71748|  %2589 = gep %2572, i64 1632                                                                                           ;L2158<1830
 71749|  %2590 = load i64, ptr %2589, , !!8                                                                                    ;L2158<1830
 71750|     ;; x1 = i64 %2590
 71751|     ;; self = i64 %2590
 71752|  %2591 = gep %2572, i64 1640                                                                                           ;L2158<1830
 71753|  %2592 = load i64, ptr %2591, , !!8                                                                                    ;L2158<1830
 71754|     ;; y1 = i64 %2592
 71755|     ;; self = i64 %2592
 71756|  %2593 = load i64, ptr %1459, , !!8                                                                                    ;L2158<1830
 71757|     ;; x2 = i64 %2593
 71758|     ;; other = i64 %2593
 71759|  %2594 = load i64, ptr %1460, , !!8                                                                                    ;L2158<1830
 71760|     ;; y2 = i64 %2594
 71761|     ;; other = i64 %2594
 71762|  %2595 = icmp ult i64 %2590, %2593                                                                                     ;L3147<7<2158<1830
 71763|  %2596 = sub nuw i64 %2593, %2590                                                                                      ;L3147<7<2158<1830
 71764|  %2597 = sub nuw i64 %2590, %2593                                                                                      ;L3147<7<2158<1830
 71765|  %2598 = select i1 %2595, i64 %2596, i64 %2597                                                                         ;L3147<7<2158<1830
 71766|     ;; dx = i64 %2598
 71767|  %2599 = icmp ult i64 %2592, %2594                                                                                     ;L3147<8<2158<1830
 71768|  %2600 = sub nuw i64 %2594, %2592                                                                                      ;L3147<8<2158<1830
 71769|  %2601 = sub nuw i64 %2592, %2594                                                                                      ;L3147<8<2158<1830
 71770|  %2602 = select i1 %2599, i64 %2600, i64 %2601                                                                         ;L3147<8<2158<1830
 71771|     ;; dy = i64 %2602
 71772|  %2603 = mul i64 %2598, %2598                                                                                          ;L9<2158<1830
 71773|  %2604 = mul i64 %2602, %2602                                                                                          ;L9<2158<1830
 71774|  %2605 = add i64 %2604, %2603                                                                                          ;L9<2158<1830
 71775|  %2606 = mul i64 %2588, %2588                                                                                          ;L1830
 71776|  %2607 = icmp ugt i64 %2605, %2606                                                                                     ;L1830
 71777|  br i1 %2607, label %2554, label %2608                                                                                 ;L1830
 71778| 
 71779| 2608: ; preds = %2586
 71782|  %2609 = gep %2572, i64 1472                                                                                           ;L1831
 71783|  %2610 = load i64, ptr %2609, , !!8                                                                                    ;L1831
 71784|  invoke void @ai::small_action4castNtB5_17SmallActionSkill23new(ptr sret([24 x i8]) %24, ptr %3, i64 %2610)
 71785|  to label %2611 unwind label %178                                                                                      ;L1831
 71786| 
 71787| 2611: ; preds = %2608
 71788|  call void @llvm.memcpy.p0.p0.i64(ptr %25, ptr %24, i64 24, i1 false)                                                  ;L1831
 71789|  store i8 17, ptr %2144,                                                                                               ;L1831
 71792|     ;; self = ptr %65
 71793|     ;; self = ptr %65
 71794|     ;; value = ptr %25
 71795|     ;; src = ptr %25
 71796|     ;; additional = i64 1
 71797|     ;; needed_extra_cap = i64 1
 71798|     ;; needed_extra_cap = i64 1
 71799|     ;; strategy = i8 1
 71800|  %2612 = load i64, ptr %126, , !!76180, !!8                                                                            ;L1428<1831
 71801|     ;; self = ptr %65
 71802|  %2613 = load i64, ptr %125, , !!76180, !!8                                                                            ;L149<1428<1831
 71803|  %2614 = icmp eq i64 %2612, %2613                                                                                      ;L1428<1831
 71804|  br i1 %2614, label %2615, label %2620                                                                                 ;L1428<1831
 71805| 
 71806| 2615: ; preds = %2611
 71807|     ;; self = ptr %65
 71808|     ;; self = ptr %65
 71809|     ;; self = ptr %65
 71810|     ;; used_cap = i64 %2612
 71811|     ;; used_cap = i64 %2612
 71812|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %2612, i64 1, i1 zeroext true)
 71813|  to label %2616 unwind label %2618, !!76180                                                                            ;L619<430<738<1429<1831
 71814| 
 71815| 2616: ; preds = %2615
 71816|  %2617 = load i64, ptr %126, , !!76180                                                                                 ;L1432<1831
 71817|  br label %2620                                                                                                        ;L619<430<738<1429<1831
 71818| 
 71819| 2618: ; preds = %2615
 71820|  %2619 = cleanuppad within none []
 71821|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %25) #30 [ "funclet"(token %2619) ], !!76165 ;L1436<1831
 71822|  cleanupret from %2619 unwind label %178
 71823| 
 71824| 2620: ; preds = %2616, %2611
 71825|  %2621 = phi i64 [ %2617, %2616 ], [ %2612, %2611 ]                                                                    ;L1432<1831
 71826|     ;; self = ptr %65
 71827|  %2622 = load ptr, ptr %65, , !!76180, !!8, !!8                                                                        ;L138<1432<1831
 71828|     ;; self = ptr %2622
 71829|     ;; count = i64 %2621
 71830|  %2623 = gepS %2622, i64 %2621                                                                                         ;L961<1432<1831
 71831|     ;; end = ptr %2623
 71832|     ;; dst = ptr %2623
 71833|  call void @llvm.memcpy.p0.p0.i64(ptr %2623, ptr %25, i64 184, i1 false), !!76165                                      ;L1933<1433<1831
 71834|  %2624 = add i64 %2621, 1                                                                                              ;L1434<1831
 71835|  store i64 %2624, ptr %126, , !!76180                                                                                  ;L1434<1831
 71837|  br label %2554                                                                                                        ;L1830
 71838| 
 71839| 2625: ; preds = %2376
 71840|  %2626 = gep %82, i64 240                                                                                              ;L1838
 71841|  %2627 = getelementptr { { ptr, ptr, i64 }, i64 }, ptr %2626, i64 %104                                                 ;L1838
 71842|     ;; self = ptr %2627
 71843|     ;; self = ptr %2627
 71844|  %2628 = load ptr, ptr %2627, , !!8, !!8                                                                               ;L138<2073<1838
 71845|     ;; p = ptr %2628
 71846|  %2629 = gep %2627, i64 24                                                                                             ;L2075<1838
 71847|  %2630 = load i64, ptr %2629, , !!8                                                                                    ;L2075<1838
 71848|     ;; len = i64 %2630
 71849|     ;; count = i64 %2630
 71850|     ;; self[0..+8] = ptr %2628
 71851|     ;; slice[0..+8] = ptr %2628
 71852|     ;; self[8..+8] = i64 %2630
 71853|     ;; slice[8..+8] = i64 %2630
 71854|     ;; ptr = ptr %2628
 71855|     ;; self = ptr %2628
 71856|  %2631 = getelementptr ptr, ptr %2628, i64 %2630                                                                       ;L961<100<1042<1838
 71857|     ;; iter[0..+8] = ptr %2628
 71858|     ;; iter[8..+8] = ptr %2631
 71859|  %2632 = gep %82, i64 8
 71860|  %2633 = gep %23, i64 177
 71861|  %2634 = add i64 %1461, %208
 71862|  %2635 = gep %21, i64 177
 71863|  %2636 = add i64 %1461, %237
 71864|  %2637 = gep %19, i64 177
 71865|  br label %2638                                                                                                        ;L1838
 71866| 
 71867| 2638: ; preds = %2649, %2625
 71868|  %2639 = phi ptr [ %2628, %2625 ], [ %2642, %2649 ]                                                                    ;L1838
 71869|     ;; iter[0..+8] = ptr %2639
 71870|     ;; self = ptr undef
 71871|     ;; ptr = ptr %2639
 71872|     ;; self = ptr %2639
 71873|     ;; end_or_len = ptr %2631
 71876|  %2640 = icmp eq ptr %2639, %2631                                                                                      ;L1714<180<1838
 71877|  br i1 %2640, label %2647, label %2641                                                                                 ;L180<1838
 71878| 
 71879| 2641: ; preds = %2638
 71880|  %2642 = gep %2639, i64 8                                                                                              ;L656<185<1838
 71881|     ;; iter[0..+8] = ptr %2642
 71882|  %2643 = load ptr, ptr %2639, , !!8, !!8                                                                               ;L1838
 71883|     ;; e = ptr %2643
 71884|     ;; self = ptr %2643
 71885|     ;; self = ptr %2643
 71886|     ;; self = ptr %2643
 71887|     ;; self = ptr %2643
 71888|     ;; self = ptr %2643
 71889|     ;; self = ptr %2643
 71890|     ;; self = ptr %2643
 71891|  %2644 = load ptr, ptr %82, , !!8, !!8                                                                                 ;L1839
 71892|  %2645 = load ptr, ptr %2632, , !!8, !!8                                                                               ;L1839
 71893|  %2646 = invoke zeroext i1 @gc::simulation5state6playerNtB2_13OperationData10can_target(ptr %3, ptr %2644, ptr %2645, ptr %2, ptr %2643)
 71894|  to label %2648 unwind label %178                                                                                      ;L1839
 71895| 
 71896| 2647: ; preds = %2638
 71897|  br i1 %148, label %2850, label %2848                                                                                  ;L1870
 71898| 
 71899| 2648: ; preds = %2641
 71900|  br i1 %2646, label %2650, label %2649                                                                                 ;L1839
 71901| 
 71902| 2649: ; preds = %2843, %2808, %2791, %2788, %2725, %2656, %2648
 71903|  br label %2638                                                                                                        ;L1714<180<1838
 71904| 
 71905| 2650: ; preds = %2648
 71906|     ;; self = ptr %86
 71907|  %2651 = load i64, ptr %86, , !!8                                                                                      ;L1136<1482<1839
 71908|  %2652 = trunc nuw i64 %2651 to i1                                                                                     ;L1136<1482<1839
 71909|  br i1 %2652, label %2662, label %2653                                                                                 ;L1136<1482<1839
 71910| 
 71911| 2653: ; preds = %2650
 71912|     ;; team = ptr %86
 71913|  %2654 = load i64, ptr %1458, , !!8                                                                                    ;L1137<1482<1839
 71914|     ;; team = i64 %2654
 71915|  %2655 = icmp ult i64 %2654, 2                                                                                         ;L1483<1839
 71916|  br i1 %2655, label %2656, label %2661                                                                                 ;L1483<1839
 71917| 
 71918| 2656: ; preds = %2653
 71920|  %2657 = gep %2643, i64 56                                                                                             ;L122<1483<1839
 71921|  %2658 = gepS %2657, i64 %2654                                                                                         ;L122<1483<1839
 71922|  %2659 = load i64, ptr %2658, , !!8                                                                                    ;L122<1483<1839
 71923|  %2660 = icmp eq i64 %2659, 0                                                                                          ;L122<1483<1839
 71924|  br i1 %2660, label %2662, label %2649                                                                                 ;L1839
 71925| 
 71926| 2661: ; preds = %2653
 71927|  invoke void @core::panicking18panic_bounds_check(i64 %2654, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 71928|  to label %118 unwind label %178                                                                                       ;L1483<1839
 71929| 
 71930| 2662: ; preds = %2656, %2650
 71931|     ;; max_tick = i64 %97
 71932|  br i1 %130, label %2665, label %2663                                                                                  ;L1845
 71933| 
 71934| 2663: ; preds = %2662
 71935|  %2664 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_attack(ptr %86)
 71936|  to label %2666 unwind label %178                                                                                      ;L1845
 71937| 
 71938| 2665: ; preds = %2718, %2683, %2666, %2662
 71939|  br i1 %133, label %2725, label %2723                                                                                  ;L1853
 71940| 
 71941| 2666: ; preds = %2663
 71942|  br i1 %2664, label %2667, label %2665                                                                                 ;L1845
 71943| 
 71944| 2667: ; preds = %2666
 71945|  %2668 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %127, ptr %86, ptr %2643)
 71946|  to label %2669 unwind label %178                                                                                      ;L1846
 71947| 
 71948| 2669: ; preds = %2667
 71949|  %2670 = gep %2643, i64 1136                                                                                           ;L1511<1846
 71950|  %2671 = load i32, ptr %2670, , !!8                                                                                    ;L1511<1846
 71951|     ;; mult = i32 %2671
 71952|  %2672 = icmp eq i32 %2671, 0                                                                                          ;L1512<1846
 71953|  br i1 %2672, label %2673, label %2676                                                                                 ;L1512<1846
 71954| 
 71955| 2673: ; preds = %2669
 71956|  %2674 = gep %2643, i64 1664                                                                                           ;L1513<1846
 71957|  %2675 = load i64, ptr %2674, , !!8                                                                                    ;L1513<1846
 71958|  br label %2683                                                                                                        ;L1512<1846
 71959| 
 71960| 2676: ; preds = %2669
 71961|  %2677 = sext i32 %2671 to i64                                                                                         ;L1511<1846
 71962|     ;; mult = i64 %2677
 71963|  %2678 = gep %2643, i64 1664                                                                                           ;L1515<1846
 71964|  %2679 = load i64, ptr %2678, , !!8                                                                                    ;L1515<1846
 71965|  %2680 = add nsw i64 %2677, 100                                                                                        ;L1515<1846
 71966|  %2681 = mul i64 %2679, %2680                                                                                          ;L1515<1846
 71967|  %2682 = udiv i64 %2681, 100                                                                                           ;L1515<1846
 71968|  br label %2683                                                                                                        ;L1512<1846
 71969| 
 71970| 2683: ; preds = %2676, %2673
 71971|  %2684 = phi i64 [ %2675, %2673 ], [ %2682, %2676 ]                                                                    ;L0<1846
 71973|  %2685 = add i64 %1462, %2668                                                                                          ;L1846
 71974|  %2686 = add i64 %2685, %2684                                                                                          ;L1847
 71975|     ;; max_dist = i64 %2686
 71976|  %2687 = gep %2643, i64 1632                                                                                           ;L2158<1848
 71977|  %2688 = load i64, ptr %2687, , !!8                                                                                    ;L2158<1848
 71978|     ;; x1 = i64 %2688
 71979|     ;; self = i64 %2688
 71980|  %2689 = gep %2643, i64 1640                                                                                           ;L2158<1848
 71981|  %2690 = load i64, ptr %2689, , !!8                                                                                    ;L2158<1848
 71982|     ;; y1 = i64 %2690
 71983|     ;; self = i64 %2690
 71984|  %2691 = load i64, ptr %1459, , !!8                                                                                    ;L2158<1848
 71985|     ;; x2 = i64 %2691
 71986|     ;; other = i64 %2691
 71987|  %2692 = load i64, ptr %1460, , !!8                                                                                    ;L2158<1848
 71988|     ;; y2 = i64 %2692
 71989|     ;; other = i64 %2692
 71990|  %2693 = icmp ult i64 %2688, %2691                                                                                     ;L3147<7<2158<1848
 71991|  %2694 = sub nuw i64 %2691, %2688                                                                                      ;L3147<7<2158<1848
 71992|  %2695 = sub nuw i64 %2688, %2691                                                                                      ;L3147<7<2158<1848
 71993|  %2696 = select i1 %2693, i64 %2694, i64 %2695                                                                         ;L3147<7<2158<1848
 71994|     ;; dx = i64 %2696
 71995|  %2697 = icmp ult i64 %2690, %2692                                                                                     ;L3147<8<2158<1848
 71996|  %2698 = sub nuw i64 %2692, %2690                                                                                      ;L3147<8<2158<1848
 71997|  %2699 = sub nuw i64 %2690, %2692                                                                                      ;L3147<8<2158<1848
 71998|  %2700 = select i1 %2697, i64 %2698, i64 %2699                                                                         ;L3147<8<2158<1848
 71999|     ;; dy = i64 %2700
 72000|  %2701 = mul i64 %2696, %2696                                                                                          ;L9<2158<1848
 72001|  %2702 = mul i64 %2700, %2700                                                                                          ;L9<2158<1848
 72002|  %2703 = add i64 %2702, %2701                                                                                          ;L9<2158<1848
 72003|  %2704 = mul i64 %2686, %2686                                                                                          ;L1848
 72004|  %2705 = icmp ugt i64 %2703, %2704                                                                                     ;L1848
 72005|  br i1 %2705, label %2665, label %2706                                                                                 ;L1848
 72006| 
 72007| 2706: ; preds = %2683
 72010|  %2707 = gep %2643, i64 1472                                                                                           ;L1849
 72011|  %2708 = load i64, ptr %2707, , !!8                                                                                    ;L1849
 72012|  invoke void @ai::small_action4castNtB2_17SmallActionAttack3new(ptr sret([24 x i8]) %22, ptr %3, i64 %2708)
 72013|  to label %2709 unwind label %178                                                                                      ;L1849
 72014| 
 72015| 2709: ; preds = %2706
 72016|  call void @llvm.memcpy.p0.p0.i64(ptr %23, ptr %22, i64 24, i1 false)                                                  ;L1849
 72017|  store i8 15, ptr %2633,                                                                                               ;L1849
 72020|     ;; self = ptr %65
 72021|     ;; self = ptr %65
 72022|     ;; value = ptr %23
 72023|     ;; src = ptr %23
 72024|     ;; additional = i64 1
 72025|     ;; needed_extra_cap = i64 1
 72026|     ;; needed_extra_cap = i64 1
 72027|     ;; strategy = i8 1
 72028|  %2710 = load i64, ptr %126, , !!76286, !!8                                                                            ;L1428<1849
 72029|     ;; self = ptr %65
 72030|  %2711 = load i64, ptr %125, , !!76286, !!8                                                                            ;L149<1428<1849
 72031|  %2712 = icmp eq i64 %2710, %2711                                                                                      ;L1428<1849
 72032|  br i1 %2712, label %2713, label %2718                                                                                 ;L1428<1849
 72033| 
 72034| 2713: ; preds = %2709
 72035|     ;; self = ptr %65
 72036|     ;; self = ptr %65
 72037|     ;; self = ptr %65
 72038|     ;; used_cap = i64 %2710
 72039|     ;; used_cap = i64 %2710
 72040|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %2710, i64 1, i1 zeroext true)
 72041|  to label %2714 unwind label %2716, !!76286                                                                            ;L619<430<738<1429<1849
 72042| 
 72043| 2714: ; preds = %2713
 72044|  %2715 = load i64, ptr %126, , !!76286                                                                                 ;L1432<1849
 72045|  br label %2718                                                                                                        ;L619<430<738<1429<1849
 72046| 
 72047| 2716: ; preds = %2713
 72048|  %2717 = cleanuppad within none []
 72049|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %23) #30 [ "funclet"(token %2717) ], !!76271 ;L1436<1849
 72050|  cleanupret from %2717 unwind label %178
 72051| 
 72052| 2718: ; preds = %2714, %2709
 72053|  %2719 = phi i64 [ %2715, %2714 ], [ %2710, %2709 ]                                                                    ;L1432<1849
 72054|     ;; self = ptr %65
 72055|  %2720 = load ptr, ptr %65, , !!76286, !!8, !!8                                                                        ;L138<1432<1849
 72056|     ;; self = ptr %2720
 72057|     ;; count = i64 %2719
 72058|  %2721 = gepS %2720, i64 %2719                                                                                         ;L961<1432<1849
 72059|     ;; end = ptr %2721
 72060|     ;; dst = ptr %2721
 72061|  call void @llvm.memcpy.p0.p0.i64(ptr %2721, ptr %23, i64 184, i1 false), !!76271                                      ;L1933<1433<1849
 72062|  %2722 = add i64 %2719, 1                                                                                              ;L1434<1849
 72063|  store i64 %2722, ptr %126, , !!76286                                                                                  ;L1434<1849
 72065|  br label %2665                                                                                                        ;L1848
 72066| 
 72067| 2723: ; preds = %2665
 72068|  %2724 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity9can_skill(ptr %86)
 72069|  to label %2726 unwind label %178                                                                                      ;L1853
 72070| 
 72071| 2725: ; preds = %2781, %2746, %2729, %2726, %2665
 72072|  br i1 %142, label %2649, label %2786                                                                                  ;L1861
 72073| 
 72074| 2726: ; preds = %2723
 72075|  br i1 %2724, label %2727, label %2725                                                                                 ;L1853
 72076| 
 72077| 2727: ; preds = %2726
 72078|  %2728 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %1464, ptr %86, ptr %2643)
 72079|  to label %2729 unwind label %178                                                                                      ;L1853
 72080| 
 72081| 2729: ; preds = %2727
 72082|  br i1 %2728, label %2730, label %2725                                                                                 ;L1853
 72083| 
 72084| 2730: ; preds = %2729
 72085|  %2731 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %134, ptr %86, ptr %2643)
 72086|  to label %2732 unwind label %178                                                                                      ;L1854
 72087| 
 72088| 2732: ; preds = %2730
 72089|  %2733 = gep %2643, i64 1136                                                                                           ;L1511<1854
 72090|  %2734 = load i32, ptr %2733, , !!8                                                                                    ;L1511<1854
 72091|     ;; mult = i32 %2734
 72092|  %2735 = icmp eq i32 %2734, 0                                                                                          ;L1512<1854
 72093|  br i1 %2735, label %2736, label %2739                                                                                 ;L1512<1854
 72094| 
 72095| 2736: ; preds = %2732
 72096|  %2737 = gep %2643, i64 1664                                                                                           ;L1513<1854
 72097|  %2738 = load i64, ptr %2737, , !!8                                                                                    ;L1513<1854
 72098|  br label %2746                                                                                                        ;L1512<1854
 72099| 
 72100| 2739: ; preds = %2732
 72101|  %2740 = sext i32 %2734 to i64                                                                                         ;L1511<1854
 72102|     ;; mult = i64 %2740
 72103|  %2741 = gep %2643, i64 1664                                                                                           ;L1515<1854
 72104|  %2742 = load i64, ptr %2741, , !!8                                                                                    ;L1515<1854
 72105|  %2743 = add nsw i64 %2740, 100                                                                                        ;L1515<1854
 72106|  %2744 = mul i64 %2742, %2743                                                                                          ;L1515<1854
 72107|  %2745 = udiv i64 %2744, 100                                                                                           ;L1515<1854
 72108|  br label %2746                                                                                                        ;L1512<1854
 72109| 
 72110| 2746: ; preds = %2739, %2736
 72111|  %2747 = phi i64 [ %2738, %2736 ], [ %2745, %2739 ]                                                                    ;L0<1854
 72113|  %2748 = add i64 %2634, %2731                                                                                          ;L1854
 72114|  %2749 = add i64 %2748, %2747                                                                                          ;L1855
 72115|     ;; max_dist = i64 %2749
 72116|  %2750 = gep %2643, i64 1632                                                                                           ;L2158<1856
 72117|  %2751 = load i64, ptr %2750, , !!8                                                                                    ;L2158<1856
 72118|     ;; x1 = i64 %2751
 72119|     ;; self = i64 %2751
 72120|  %2752 = gep %2643, i64 1640                                                                                           ;L2158<1856
 72121|  %2753 = load i64, ptr %2752, , !!8                                                                                    ;L2158<1856
 72122|     ;; y1 = i64 %2753
 72123|     ;; self = i64 %2753
 72124|  %2754 = load i64, ptr %1459, , !!8                                                                                    ;L2158<1856
 72125|     ;; x2 = i64 %2754
 72126|     ;; other = i64 %2754
 72127|  %2755 = load i64, ptr %1460, , !!8                                                                                    ;L2158<1856
 72128|     ;; y2 = i64 %2755
 72129|     ;; other = i64 %2755
 72130|  %2756 = icmp ult i64 %2751, %2754                                                                                     ;L3147<7<2158<1856
 72131|  %2757 = sub nuw i64 %2754, %2751                                                                                      ;L3147<7<2158<1856
 72132|  %2758 = sub nuw i64 %2751, %2754                                                                                      ;L3147<7<2158<1856
 72133|  %2759 = select i1 %2756, i64 %2757, i64 %2758                                                                         ;L3147<7<2158<1856
 72134|     ;; dx = i64 %2759
 72135|  %2760 = icmp ult i64 %2753, %2755                                                                                     ;L3147<8<2158<1856
 72136|  %2761 = sub nuw i64 %2755, %2753                                                                                      ;L3147<8<2158<1856
 72137|  %2762 = sub nuw i64 %2753, %2755                                                                                      ;L3147<8<2158<1856
 72138|  %2763 = select i1 %2760, i64 %2761, i64 %2762                                                                         ;L3147<8<2158<1856
 72139|     ;; dy = i64 %2763
 72140|  %2764 = mul i64 %2759, %2759                                                                                          ;L9<2158<1856
 72141|  %2765 = mul i64 %2763, %2763                                                                                          ;L9<2158<1856
 72142|  %2766 = add i64 %2765, %2764                                                                                          ;L9<2158<1856
 72143|  %2767 = mul i64 %2749, %2749                                                                                          ;L1856
 72144|  %2768 = icmp ugt i64 %2766, %2767                                                                                     ;L1856
 72145|  br i1 %2768, label %2725, label %2769                                                                                 ;L1856
 72146| 
 72147| 2769: ; preds = %2746
 72150|  %2770 = gep %2643, i64 1472                                                                                           ;L1857
 72151|  %2771 = load i64, ptr %2770, , !!8                                                                                    ;L1857
 72152|  invoke void @ai::small_action4castNtB5_16SmallActionSkill3new(ptr sret([24 x i8]) %20, ptr %3, i64 %2771)
 72153|  to label %2772 unwind label %178                                                                                      ;L1857
 72154| 
 72155| 2772: ; preds = %2769
 72156|  call void @llvm.memcpy.p0.p0.i64(ptr %21, ptr %20, i64 24, i1 false)                                                  ;L1857
 72157|  store i8 16, ptr %2635,                                                                                               ;L1857
 72160|     ;; self = ptr %65
 72161|     ;; self = ptr %65
 72162|     ;; value = ptr %21
 72163|     ;; src = ptr %21
 72164|     ;; additional = i64 1
 72165|     ;; needed_extra_cap = i64 1
 72166|     ;; needed_extra_cap = i64 1
 72167|     ;; strategy = i8 1
 72168|  %2773 = load i64, ptr %126, , !!76345, !!8                                                                            ;L1428<1857
 72169|     ;; self = ptr %65
 72170|  %2774 = load i64, ptr %125, , !!76345, !!8                                                                            ;L149<1428<1857
 72171|  %2775 = icmp eq i64 %2773, %2774                                                                                      ;L1428<1857
 72172|  br i1 %2775, label %2776, label %2781                                                                                 ;L1428<1857
 72173| 
 72174| 2776: ; preds = %2772
 72175|     ;; self = ptr %65
 72176|     ;; self = ptr %65
 72177|     ;; self = ptr %65
 72178|     ;; used_cap = i64 %2773
 72179|     ;; used_cap = i64 %2773
 72180|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %2773, i64 1, i1 zeroext true)
 72181|  to label %2777 unwind label %2779, !!76345                                                                            ;L619<430<738<1429<1857
 72182| 
 72183| 2777: ; preds = %2776
 72184|  %2778 = load i64, ptr %126, , !!76345                                                                                 ;L1432<1857
 72185|  br label %2781                                                                                                        ;L619<430<738<1429<1857
 72186| 
 72187| 2779: ; preds = %2776
 72188|  %2780 = cleanuppad within none []
 72189|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %21) #30 [ "funclet"(token %2780) ], !!76330 ;L1436<1857
 72190|  cleanupret from %2780 unwind label %178
 72191| 
 72192| 2781: ; preds = %2777, %2772
 72193|  %2782 = phi i64 [ %2778, %2777 ], [ %2773, %2772 ]                                                                    ;L1432<1857
 72194|     ;; self = ptr %65
 72195|  %2783 = load ptr, ptr %65, , !!76345, !!8, !!8                                                                        ;L138<1432<1857
 72196|     ;; self = ptr %2783
 72197|     ;; count = i64 %2782
 72198|  %2784 = gepS %2783, i64 %2782                                                                                         ;L961<1432<1857
 72199|     ;; end = ptr %2784
 72200|     ;; dst = ptr %2784
 72201|  call void @llvm.memcpy.p0.p0.i64(ptr %2784, ptr %21, i64 184, i1 false), !!76330                                      ;L1933<1433<1857
 72202|  %2785 = add i64 %2782, 1                                                                                              ;L1434<1857
 72203|  store i64 %2785, ptr %126, , !!76345                                                                                  ;L1434<1857
 72205|  br label %2725                                                                                                        ;L1856
 72206| 
 72207| 2786: ; preds = %2725
 72208|  %2787 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_skill2(ptr %86)
 72209|  to label %2788 unwind label %178                                                                                      ;L1861
 72210| 
 72211| 2788: ; preds = %2786
 72212|  br i1 %2787, label %2789, label %2649                                                                                 ;L1861
 72213| 
 72214| 2789: ; preds = %2788
 72215|  %2790 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %1466, ptr %86, ptr %2643)
 72216|  to label %2791 unwind label %178                                                                                      ;L1861
 72217| 
 72218| 2791: ; preds = %2789
 72219|  br i1 %2790, label %2792, label %2649                                                                                 ;L1861
 72220| 
 72221| 2792: ; preds = %2791
 72222|  %2793 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %139, ptr %86, ptr %2643)
 72223|  to label %2794 unwind label %178                                                                                      ;L1862
 72224| 
 72225| 2794: ; preds = %2792
 72226|  %2795 = gep %2643, i64 1136                                                                                           ;L1511<1862
 72227|  %2796 = load i32, ptr %2795, , !!8                                                                                    ;L1511<1862
 72228|     ;; mult = i32 %2796
 72229|  %2797 = icmp eq i32 %2796, 0                                                                                          ;L1512<1862
 72230|  br i1 %2797, label %2798, label %2801                                                                                 ;L1512<1862
 72231| 
 72232| 2798: ; preds = %2794
 72233|  %2799 = gep %2643, i64 1664                                                                                           ;L1513<1862
 72234|  %2800 = load i64, ptr %2799, , !!8                                                                                    ;L1513<1862
 72235|  br label %2808                                                                                                        ;L1512<1862
 72236| 
 72237| 2801: ; preds = %2794
 72238|  %2802 = sext i32 %2796 to i64                                                                                         ;L1511<1862
 72239|     ;; mult = i64 %2802
 72240|  %2803 = gep %2643, i64 1664                                                                                           ;L1515<1862
 72241|  %2804 = load i64, ptr %2803, , !!8                                                                                    ;L1515<1862
 72242|  %2805 = add nsw i64 %2802, 100                                                                                        ;L1515<1862
 72243|  %2806 = mul i64 %2804, %2805                                                                                          ;L1515<1862
 72244|  %2807 = udiv i64 %2806, 100                                                                                           ;L1515<1862
 72245|  br label %2808                                                                                                        ;L1512<1862
 72246| 
 72247| 2808: ; preds = %2801, %2798
 72248|  %2809 = phi i64 [ %2800, %2798 ], [ %2807, %2801 ]                                                                    ;L0<1862
 72250|  %2810 = add i64 %2636, %2793                                                                                          ;L1862
 72251|  %2811 = add i64 %2810, %2809                                                                                          ;L1863
 72252|     ;; max_dist = i64 %2811
 72253|  %2812 = gep %2643, i64 1632                                                                                           ;L2158<1864
 72254|  %2813 = load i64, ptr %2812, , !!8                                                                                    ;L2158<1864
 72255|     ;; x1 = i64 %2813
 72256|     ;; self = i64 %2813
 72257|  %2814 = gep %2643, i64 1640                                                                                           ;L2158<1864
 72258|  %2815 = load i64, ptr %2814, , !!8                                                                                    ;L2158<1864
 72259|     ;; y1 = i64 %2815
 72260|     ;; self = i64 %2815
 72261|  %2816 = load i64, ptr %1459, , !!8                                                                                    ;L2158<1864
 72262|     ;; x2 = i64 %2816
 72263|     ;; other = i64 %2816
 72264|  %2817 = load i64, ptr %1460, , !!8                                                                                    ;L2158<1864
 72265|     ;; y2 = i64 %2817
 72266|     ;; other = i64 %2817
 72267|  %2818 = icmp ult i64 %2813, %2816                                                                                     ;L3147<7<2158<1864
 72268|  %2819 = sub nuw i64 %2816, %2813                                                                                      ;L3147<7<2158<1864
 72269|  %2820 = sub nuw i64 %2813, %2816                                                                                      ;L3147<7<2158<1864
 72270|  %2821 = select i1 %2818, i64 %2819, i64 %2820                                                                         ;L3147<7<2158<1864
 72271|     ;; dx = i64 %2821
 72272|  %2822 = icmp ult i64 %2815, %2817                                                                                     ;L3147<8<2158<1864
 72273|  %2823 = sub nuw i64 %2817, %2815                                                                                      ;L3147<8<2158<1864
 72274|  %2824 = sub nuw i64 %2815, %2817                                                                                      ;L3147<8<2158<1864
 72275|  %2825 = select i1 %2822, i64 %2823, i64 %2824                                                                         ;L3147<8<2158<1864
 72276|     ;; dy = i64 %2825
 72277|  %2826 = mul i64 %2821, %2821                                                                                          ;L9<2158<1864
 72278|  %2827 = mul i64 %2825, %2825                                                                                          ;L9<2158<1864
 72279|  %2828 = add i64 %2827, %2826                                                                                          ;L9<2158<1864
 72280|  %2829 = mul i64 %2811, %2811                                                                                          ;L1864
 72281|  %2830 = icmp ugt i64 %2828, %2829                                                                                     ;L1864
 72282|  br i1 %2830, label %2649, label %2831                                                                                 ;L1864
 72283| 
 72284| 2831: ; preds = %2808
 72287|  %2832 = gep %2643, i64 1472                                                                                           ;L1865
 72288|  %2833 = load i64, ptr %2832, , !!8                                                                                    ;L1865
 72289|  invoke void @ai::small_action4castNtB5_17SmallActionSkill23new(ptr sret([24 x i8]) %18, ptr %3, i64 %2833)
 72290|  to label %2834 unwind label %178                                                                                      ;L1865
 72291| 
 72292| 2834: ; preds = %2831
 72293|  call void @llvm.memcpy.p0.p0.i64(ptr %19, ptr %18, i64 24, i1 false)                                                  ;L1865
 72294|  store i8 17, ptr %2637,                                                                                               ;L1865
 72297|     ;; self = ptr %65
 72298|     ;; self = ptr %65
 72299|     ;; value = ptr %19
 72300|     ;; src = ptr %19
 72301|     ;; additional = i64 1
 72302|     ;; needed_extra_cap = i64 1
 72303|     ;; needed_extra_cap = i64 1
 72304|     ;; strategy = i8 1
 72305|  %2835 = load i64, ptr %126, , !!76403, !!8                                                                            ;L1428<1865
 72306|     ;; self = ptr %65
 72307|  %2836 = load i64, ptr %125, , !!76403, !!8                                                                            ;L149<1428<1865
 72308|  %2837 = icmp eq i64 %2835, %2836                                                                                      ;L1428<1865
 72309|  br i1 %2837, label %2838, label %2843                                                                                 ;L1428<1865
 72310| 
 72311| 2838: ; preds = %2834
 72312|     ;; self = ptr %65
 72313|     ;; self = ptr %65
 72314|     ;; self = ptr %65
 72315|     ;; used_cap = i64 %2835
 72316|     ;; used_cap = i64 %2835
 72317|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %2835, i64 1, i1 zeroext true)
 72318|  to label %2839 unwind label %2841, !!76403                                                                            ;L619<430<738<1429<1865
 72319| 
 72320| 2839: ; preds = %2838
 72321|  %2840 = load i64, ptr %126, , !!76403                                                                                 ;L1432<1865
 72322|  br label %2843                                                                                                        ;L619<430<738<1429<1865
 72323| 
 72324| 2841: ; preds = %2838
 72325|  %2842 = cleanuppad within none []
 72326|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %19) #30 [ "funclet"(token %2842) ], !!76388 ;L1436<1865
 72327|  cleanupret from %2842 unwind label %178
 72328| 
 72329| 2843: ; preds = %2839, %2834
 72330|  %2844 = phi i64 [ %2840, %2839 ], [ %2835, %2834 ]                                                                    ;L1432<1865
 72331|     ;; self = ptr %65
 72332|  %2845 = load ptr, ptr %65, , !!76403, !!8, !!8                                                                        ;L138<1432<1865
 72333|     ;; self = ptr %2845
 72334|     ;; count = i64 %2844
 72335|  %2846 = gepS %2845, i64 %2844                                                                                         ;L961<1432<1865
 72336|     ;; end = ptr %2846
 72337|     ;; dst = ptr %2846
 72338|  call void @llvm.memcpy.p0.p0.i64(ptr %2846, ptr %19, i64 184, i1 false), !!76388                                      ;L1933<1433<1865
 72339|  %2847 = add i64 %2844, 1                                                                                              ;L1434<1865
 72340|  store i64 %2847, ptr %126, , !!76403                                                                                  ;L1434<1865
 72342|  br label %2649                                                                                                        ;L1864
 72343| 
 72344| 2848: ; preds = %2647
 72345|  %2849 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity7can_ult(ptr %86)
 72346|  to label %2885 unwind label %178                                                                                      ;L1870
 72347| 
 72348| 2850: ; preds = %3578, %3572, %3571, %3444, %3435, %2885, %2647
 72349|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %65, i64 32, i1 false)                                                   ;L2038
 72352|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %68)
 72353|  to label %2869 unwind label %2851                                                                                     ;L825<2039
 72354| 
 72355| 2851: ; preds = %2850
 72356|  %2852 = cleanuppad within none []
 72360|     ;; self = ptr %68
 72362|     ;; self = ptr %68
 72363|     ;; self = ptr %68
 72364|     ;; elem_size = i64 8
 72365|     ;; align = i64 8
 72366|  %2853 = gep %68, i64 16                                                                                               ;L159<703<714<825<825<2039
 72367|  %2854 = load i64, ptr %2853, , !!8                                                                                    ;L159<703<714<825<825<2039
 72368|  %2855 = icmp eq i64 %2854, 0                                                                                          ;L159<703<714<825<825<2039
 72369|  br i1 %2855, label %2868, label %2856                                                                                 ;L159<703<714<825<825<2039
 72370| 
 72371| 2856: ; preds = %2851
 72372|     ;; layout[0..+8] = i64 8
 72373|     ;; layout[8..+8] = i64 %2854
 72374|  %2857 = gep %68, i64 8                                                                                                ;L704<714<825<825<2039
 72375|  %2858 = load ptr, ptr %2857, , !!8, !!8                                                                               ;L704<714<825<825<2039
 72376|  %2859 = load ptr, ptr %68, , !!8, !!8                                                                                 ;L704<714<825<825<2039
 72377|  %2860 = gep %2858, i64 16                                                                                             ;L704<714<825<825<2039
 72378|  %2861 = load ptr, ptr %2860, , !!76453, !!8, !!8                                                                      ;L704<714<825<825<2039
 72381|     ;; ptr = ptr %2859
 72382|     ;; ptr = ptr %2859
 72383|     ;; layout[0..+8] = i64 8
 72384|     ;; layout[8..+8] = i64 %2854
 72385|     ;; footer = ptr %2861
 72386|     ;; footer = ptr %2861
 72387|     ;; self = ptr %2861
 72388|  %2862 = gep %2861, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<2039
 72389|  %2863 = load ptr, ptr %2862, , !!76453, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<2039
 72392|     ;; self = ptr %2859
 72393|  %2864 = icmp eq ptr %2863, %2859                                                                                      ;L1714<1700<1707<704<714<825<825<2039
 72394|  br i1 %2864, label %2865, label %2868                                                                                 ;L1707<704<714<825<825<2039
 72395| 
 72396| 2865: ; preds = %2856
 72397|  %2866 = shl i64 %2854, 3                                                                                              ;L166<703<714<825<825<2039
 72398|     ;; layout[8..+8] = i64 %2866
 72399|     ;; layout[8..+8] = i64 %2866
 72400|     ;; count = i64 %2866
 72401|  %2867 = gep %2859, i64 %2866                                                                                          ;L961<1708<704<714<825<825<2039
 72402|     ;; ptr = ptr %2867
 72403|     ;; val = ptr %2867
 72404|     ;; val = ptr %2867
 72405|     ;; self = ptr %2861
 72406|     ;; self = ptr %2861
 72407|  store ptr %2867, ptr %2862, , !!76453                                                                                 ;L931<513<437<1709<704<714<825<825<2039
 72408|  br label %2868                                                                                                        ;L1707<704<714<825<825<2039
 72409| 
 72410| 2868: ; preds = %2865, %2856, %2851
 72411|  cleanupret from %2852 unwind label %120
 72412| 
 72413| 2869: ; preds = %2850
 72417|     ;; self = ptr %68
 72419|     ;; self = ptr %68
 72420|     ;; self = ptr %68
 72421|     ;; elem_size = i64 8
 72422|     ;; align = i64 8
 72423|  %2870 = gep %68, i64 16                                                                                               ;L159<703<714<825<825<2039
 72424|  %2871 = load i64, ptr %2870, , !!8                                                                                    ;L159<703<714<825<825<2039
 72425|  %2872 = icmp eq i64 %2871, 0                                                                                          ;L159<703<714<825<825<2039
 72426|  br i1 %2872, label %3579, label %2873                                                                                 ;L159<703<714<825<825<2039
 72427| 
 72428| 2873: ; preds = %2869
 72429|     ;; layout[0..+8] = i64 8
 72430|     ;; layout[8..+8] = i64 %2871
 72431|  %2874 = gep %68, i64 8                                                                                                ;L704<714<825<825<2039
 72432|  %2875 = load ptr, ptr %2874, , !!8, !!8                                                                               ;L704<714<825<825<2039
 72433|  %2876 = load ptr, ptr %68, , !!8, !!8                                                                                 ;L704<714<825<825<2039
 72434|  %2877 = gep %2875, i64 16                                                                                             ;L704<714<825<825<2039
 72435|  %2878 = load ptr, ptr %2877, , !!76506, !!8, !!8                                                                      ;L704<714<825<825<2039
 72438|     ;; ptr = ptr %2876
 72439|     ;; ptr = ptr %2876
 72440|     ;; layout[0..+8] = i64 8
 72441|     ;; layout[8..+8] = i64 %2871
 72442|     ;; footer = ptr %2878
 72443|     ;; footer = ptr %2878
 72444|     ;; self = ptr %2878
 72445|  %2879 = gep %2878, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<2039
 72446|  %2880 = load ptr, ptr %2879, , !!76506, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<2039
 72449|     ;; self = ptr %2876
 72450|  %2881 = icmp eq ptr %2880, %2876                                                                                      ;L1714<1700<1707<704<714<825<825<2039
 72451|  br i1 %2881, label %2882, label %3579                                                                                 ;L1707<704<714<825<825<2039
 72452| 
 72453| 2882: ; preds = %2873
 72454|  %2883 = shl i64 %2871, 3                                                                                              ;L166<703<714<825<825<2039
 72455|     ;; layout[8..+8] = i64 %2883
 72456|     ;; layout[8..+8] = i64 %2883
 72457|     ;; count = i64 %2883
 72458|  %2884 = gep %2876, i64 %2883                                                                                          ;L961<1708<704<714<825<825<2039
 72459|     ;; ptr = ptr %2884
 72460|     ;; val = ptr %2884
 72461|     ;; val = ptr %2884
 72462|     ;; self = ptr %2878
 72463|     ;; self = ptr %2878
 72464|  store ptr %2884, ptr %2879, , !!76506                                                                                 ;L931<513<437<1709<704<714<825<825<2039
 72465|  br label %3579                                                                                                        ;L1707<704<714<825<825<2039
 72466| 
 72467| 2885: ; preds = %2848
 72468|  br i1 %2849, label %2886, label %2850                                                                                 ;L1870
 72469| 
 72470| 2886: ; preds = %2885
 72471|  %2887 = select i1 %143, i64 1440, i64 1456                                                                            ;L1677<1871
 72472|  %2888 = gep %86, i64 %2887                                                                                            ;L1677<1871
 72473|  %2889 = load ptr, ptr %2888, , !!8, !!8                                                                               ;L1871
 72474|  %2890 = gep %2888, i64 8                                                                                              ;L1871
 72475|  %2891 = load ptr, ptr %2890, , !!8, !!8                                                                               ;L1871
 72476|  %2892 = gep %2891, i64 104                                                                                            ;L1871
 72477|  %2893 = load ptr, ptr %2892, , !!8                                                                                    ;L1871
 72478|  %2894 = invoke { ptr, ptr } %2893(ptr %2889)
 72479|  to label %2895 unwind label %178                                                                                      ;L1871
 72480| 
 72481| 2895: ; preds = %2886
 72482|  %2896 = extractvalue { ptr, ptr } %2894, 0                                                                            ;L1871
 72483|  %2897 = extractvalue { ptr, ptr } %2894, 1                                                                            ;L1871
 72484|     ;; self[0..+8] = ptr %2896
 72485|     ;; self[0..+8] = ptr %2896
 72486|     ;; self[8..+8] = ptr %2897
 72487|     ;; self[8..+8] = ptr %2897
 72489|  %2898 = gep %2897, i64 24                                                                                             ;L201<229<1871
 72490|  %2899 = load ptr, ptr %2898, , !!8                                                                                    ;L201<229<1871
 72491|  invoke void %2899(ptr sret([16 x i8]) %7, ptr %2896)
 72492|  to label %2900 unwind label %178                                                                                      ;L201<229<1871
 72493| 
 72494| 2900: ; preds = %2895
 72497|     ;; other = ptr %7
 72498|  %2901 = load i128, ptr %7, , !!8                                                                                      ;L764<2450<204<229<1871
 72499|  %2902 = icmp eq i128 %2901, 129962296932191015333459514643667840978                                                   ;L764<2450<204<229<1871
 72501|  br i1 %2902, label %2903, label %2909                                                                                 ;L229<1871
 72502| 
 72503| 2903: ; preds = %2900
 72504|     ;; self = ptr %2896
 72505|  %2904 = icmp ne ptr %2896, null
 72506|  call void @llvm.assume(i1 %2904)
 72507|     ;; x = ptr %2896
 72508|     ;; gambler_ult = ptr %2896
 72509|  %2905 = gep %2896, i64 16                                                                                             ;L1872<661<1872
 72510|  %2906 = load i64, ptr %2905, , !!8                                                                                    ;L1872<661<1872
 72511|  %2907 = icmp eq i64 %2906, 0                                                                                          ;L1872<661<1872
 72512|     ;; is_v16_gambler_cc_ult = i1 %2907
 72513|  %2908 = select i1 %2907, i64 36000000, i64 8100000000                                                                 ;L1888
 72514|  br label %2909                                                                                                        ;L661<1872
 72515| 
 72516| 2909: ; preds = %2903, %2900
 72517|  %2910 = phi i64 [ %2908, %2903 ], [ 36000000, %2900 ]                                                                 ;L0<1872
 72519|     ;; self = ptr %70
 72520|     ;; self = ptr %70
 72521|     ;; self = ptr %70
 72522|  %2911 = load ptr, ptr %70, , !!8, !!8                                                                                 ;L138<2073<2136<1874
 72523|     ;; p = ptr %2911
 72524|  %2912 = gep %70, i64 24                                                                                               ;L2075<2136<1874
 72525|  %2913 = load i64, ptr %2912, , !!8                                                                                    ;L2075<2136<1874
 72526|     ;; len = i64 %2913
 72527|     ;; count = i64 %2913
 72528|     ;; self[0..+8] = ptr %2911
 72529|     ;; slice[0..+8] = ptr %2911
 72530|     ;; self[8..+8] = i64 %2913
 72531|     ;; slice[8..+8] = i64 %2913
 72532|     ;; ptr = ptr %2911
 72533|     ;; self = ptr %2911
 72534|  %2914 = gepS }, ptr %2911, i64 %2913                                                                                  ;L961<100<1042<2136<1874
 72535|     ;; iter[0..+8] = ptr %2911
 72536|     ;; iter[8..+8] = ptr %2914
 72537|  %2915 = gep %145, i64 40
 72538|  %2916 = gep %145, i64 16
 72539|  %2917 = gep %145, i64 24
 72540|  %2918 = gep %86, i64 1080
 72541|  %2919 = add i64 %136, -1
 72542|  %2920 = gep %86, i64 1136
 72543|  %2921 = gep %86, i64 1664
 72544|  %2922 = gep %145, i64 8
 72545|  %2923 = gep %145, i64 8
 72546|  %2924 = gep %16, i64 177
 72547|  br label %2925                                                                                                        ;L1874
 72548| 
 72549| 2925: ; preds = %3139, %2909
 72550|  %2926 = phi ptr [ %2911, %2909 ], [ %2929, %3139 ]                                                                    ;L1874
 72551|     ;; iter[0..+8] = ptr %2926
 72552|     ;; self = ptr undef
 72553|     ;; ptr = ptr %2926
 72554|     ;; self = ptr %2926
 72555|     ;; end_or_len = ptr %2914
 72558|  %2927 = icmp eq ptr %2926, %2914                                                                                      ;L1714<180<1874
 72559|  br i1 %2927, label %2934, label %2928                                                                                 ;L180<1874
 72560| 
 72561| 2928: ; preds = %2925
 72562|  %2929 = gep %2926, i64 32                                                                                             ;L656<185<1874
 72563|     ;; iter[0..+8] = ptr %2929
 72564|     ;; a = ptr %2926
 72565|     ;; e = ptr %2926
 72566|  %2930 = gep %2926, i64 24                                                                                             ;L1875
 72567|  %2931 = load ptr, ptr %2930, , !!8, !!8                                                                               ;L1875
 72568|     ;; self = ptr %2931
 72569|     ;; self = ptr %2931
 72570|     ;; self = ptr %2931
 72571|     ;; self = ptr %2931
 72572|     ;; self = ptr %86
 72573|  %2932 = load i64, ptr %86, , !!8                                                                                      ;L1136<1482<1875
 72574|  %2933 = trunc nuw i64 %2932 to i1                                                                                     ;L1136<1482<1875
 72575|  br i1 %2933, label %2953, label %2944                                                                                 ;L1136<1482<1875
 72576| 
 72577| 2934: ; preds = %2925
 72578|     ;; self = ptr %72
 72579|     ;; self = ptr %72
 72580|     ;; self = ptr %72
 72581|  %2935 = load ptr, ptr %72, , !!8, !!8                                                                                 ;L138<2073<2136<1938
 72582|     ;; p = ptr %2935
 72583|  %2936 = gep %72, i64 24                                                                                               ;L2075<2136<1938
 72584|  %2937 = load i64, ptr %2936, , !!8                                                                                    ;L2075<2136<1938
 72585|     ;; len = i64 %2937
 72586|     ;; count = i64 %2937
 72587|     ;; self[0..+8] = ptr %2935
 72588|     ;; slice[0..+8] = ptr %2935
 72589|     ;; self[8..+8] = i64 %2937
 72590|     ;; slice[8..+8] = i64 %2937
 72591|     ;; ptr = ptr %2935
 72592|     ;; self = ptr %2935
 72593|  %2938 = getelementptr ptr, ptr %2935, i64 %2937                                                                       ;L961<100<1042<2136<1938
 72594|     ;; iter[0..+8] = ptr %2935
 72595|     ;; iter[8..+8] = ptr %2938
 72596|  %2939 = gep %14, i64 72
 72597|  %2940 = mul i64 %239, 60
 72598|  %2941 = gep %86, i64 1184
 72599|  %2942 = gep %86, i64 1192
 72600|  %2943 = gep %13, i64 177
 72601|  br label %3140                                                                                                        ;L1938
 72602| 
 72603| 2944: ; preds = %2928
 72604|     ;; team = ptr %86
 72605|  %2945 = load i64, ptr %1458, , !!8                                                                                    ;L1137<1482<1875
 72606|     ;; team = i64 %2945
 72607|  %2946 = icmp ult i64 %2945, 2                                                                                         ;L1483<1875
 72608|  br i1 %2946, label %2947, label %2952                                                                                 ;L1483<1875
 72609| 
 72610| 2947: ; preds = %2944
 72612|  %2948 = gep %2931, i64 56                                                                                             ;L122<1483<1875
 72613|  %2949 = gepS %2948, i64 %2945                                                                                         ;L122<1483<1875
 72614|  %2950 = load i64, ptr %2949, , !!8                                                                                    ;L122<1483<1875
 72615|  %2951 = icmp eq i64 %2950, 0                                                                                          ;L122<1483<1875
 72616|  br i1 %2951, label %2953, label %3139                                                                                 ;L1875
 72617| 
 72618| 2952: ; preds = %2944
 72619|  invoke void @core::panicking18panic_bounds_check(i64 %2945, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 72620|  to label %118 unwind label %178                                                                                       ;L1483<1875
 72621| 
 72622| 2953: ; preds = %2947, %2928
 72623|  %2954 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %2915, ptr %86, ptr %2931)
 72624|  to label %2955 unwind label %178                                                                                      ;L1879
 72625| 
 72626| 2955: ; preds = %2953
 72627|  br i1 %2954, label %2956, label %3139                                                                                 ;L1879
 72628| 
 72629| 2956: ; preds = %2955
 72630|  %2957 = load ptr, ptr %2888, , !!8, !!8                                                                               ;L1883
 72631|  %2958 = load ptr, ptr %2890, , !!8, !!8                                                                               ;L1883
 72632|  %2959 = load ptr, ptr %82, , !!8, !!8                                                                                 ;L1883
 72633|  %2960 = load ptr, ptr %2632, , !!8, !!8                                                                               ;L1883
 72634|  %2961 = gep %2958, i64 200                                                                                            ;L1883
 72635|  %2962 = load ptr, ptr %2961, , !!8                                                                                    ;L1883
 72636|  %2963 = invoke zeroext i1 %2962(ptr %2957, ptr %2959, ptr %2960, ptr %86, ptr %2931)
 72637|  to label %2964 unwind label %178                                                                                      ;L1883
 72638| 
 72639| 2964: ; preds = %2956
 72640|  br i1 %2963, label %2965, label %3139                                                                                 ;L1883
 72641| 
 72642| 2965: ; preds = %2964
 72643|  switch i64 %4, label %2966 [
 72644|  i64 0, label %2967
 72645|  i64 1, label %2967
 72646|  i64 2, label %2967
 72647|  i64 3, label %2967
 72648|  i64 4, label %2994
 72649|  i64 5, label %2967
 72650|  i64 6, label %2967
 72651|  i64 7, label %2994
 72652|  ]                                                                                                                     ;L30<1887
 72653| 
 72654| 2966: ; preds = %2965
 72655|  unreachable
 72656| 
 72657| 2967: ; preds = %2965, %2965, %2965, %2965, %2965, %2965
 72658|     ;; self[8..+8] = i64 %5
 72659|     ;; f[0..+8] = ptr %2959
 72660|     ;; f[8..+8] = ptr %2960
 72661|     ;; x = i64 %5
 72662|     ;; id = i64 %5
 72663|  %2968 = gep %2960, i64 496                                                                                            ;L1887<1543<1887
 72664|  %2969 = load ptr, ptr %2968, , !!8                                                                                    ;L1887<1543<1887
 72665|  %2970 = invoke ptr %2969(ptr %2959, i64 %5)
 72666|  to label %2971 unwind label %178                                                                                      ;L1887<1543<1887
 72667| 
 72668| 2971: ; preds = %2967
 72669|  %2972 = icmp eq ptr %2970, null                                                                                       ;L1887
 72670|  br i1 %2972, label %2994, label %2973                                                                                 ;L1887
 72671| 
 72672| 2973: ; preds = %2971
 72673|     ;; focus = ptr %2970
 72674|     ;; other = ptr %2970
 72675|     ;; focus_radius = i64 %2910
 72676|  %2974 = gep %2931, i64 1632                                                                                           ;L2158<1889
 72677|  %2975 = load i64, ptr %2974, , !!8                                                                                    ;L2158<1889
 72678|     ;; x1 = i64 %2975
 72679|     ;; self = i64 %2975
 72680|  %2976 = gep %2931, i64 1640                                                                                           ;L2158<1889
 72681|  %2977 = load i64, ptr %2976, , !!8                                                                                    ;L2158<1889
 72682|     ;; y1 = i64 %2977
 72683|     ;; self = i64 %2977
 72684|  %2978 = gep %2970, i64 1632                                                                                           ;L2158<1889
 72685|  %2979 = load i64, ptr %2978, , !!8                                                                                    ;L2158<1889
 72686|     ;; x2 = i64 %2979
 72687|     ;; other = i64 %2979
 72688|  %2980 = gep %2970, i64 1640                                                                                           ;L2158<1889
 72689|  %2981 = load i64, ptr %2980, , !!8                                                                                    ;L2158<1889
 72690|     ;; y2 = i64 %2981
 72691|     ;; other = i64 %2981
 72692|  %2982 = icmp ult i64 %2975, %2979                                                                                     ;L3147<7<2158<1889
 72693|  %2983 = sub nuw i64 %2979, %2975                                                                                      ;L3147<7<2158<1889
 72694|  %2984 = sub nuw i64 %2975, %2979                                                                                      ;L3147<7<2158<1889
 72695|  %2985 = select i1 %2982, i64 %2983, i64 %2984                                                                         ;L3147<7<2158<1889
 72696|     ;; dx = i64 %2985
 72697|  %2986 = icmp ult i64 %2977, %2981                                                                                     ;L3147<8<2158<1889
 72698|  %2987 = sub nuw i64 %2981, %2977                                                                                      ;L3147<8<2158<1889
 72699|  %2988 = sub nuw i64 %2977, %2981                                                                                      ;L3147<8<2158<1889
 72700|  %2989 = select i1 %2986, i64 %2987, i64 %2988                                                                         ;L3147<8<2158<1889
 72701|     ;; dy = i64 %2989
 72702|  %2990 = mul i64 %2985, %2985                                                                                          ;L9<2158<1889
 72703|  %2991 = mul i64 %2989, %2989                                                                                          ;L9<2158<1889
 72704|  %2992 = add i64 %2991, %2990                                                                                          ;L9<2158<1889
 72705|  %2993 = icmp ult i64 %2992, %2910                                                                                     ;L1889
 72706|  br i1 %2993, label %2994, label %3139                                                                                 ;L1889
 72707| 
 72708| 2994: ; preds = %2973, %2971, %2965, %2965
 72709|  %2995 = load i64, ptr %2926, , !!8                                                                                    ;L1894
 72712|     ;; __self_discr = i64 %2995
 72713|     ;; __arg1_discr = i64 0
 72714|  %2996 = icmp eq i64 %2995, 0                                                                                          ;L81<1894
 72715|  br i1 %2996, label %2997, label %3004                                                                                 ;L1894
 72716| 
 72717| 2997: ; preds = %2994
 72718|  %2998 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10block_move(ptr %2931)
 72719|  to label %2999 unwind label %178                                                                                      ;L1894
 72720| 
 72721| 2999: ; preds = %2997
 72722|  br i1 %2998, label %3004, label %3000                                                                                 ;L1894
 72723| 
 72724| 3000: ; preds = %2999
 72725|  %3001 = gep %2931, i64 1600                                                                                           ;L1895
 72726|  %3002 = load i64, ptr %3001, , !!8                                                                                    ;L1895
 72727|     ;; rhs = i64 %3002
 72728|  %3003 = call i64 @llvm.usub.sat.i64(i64 %239, i64 %3002)                                                              ;L2472<1895
 72729|     ;; move_speed = i64 %3003
 72730|  br label %3004                                                                                                        ;L1894
 72731| 
 72732| 3004: ; preds = %3000, %2999, %2994
 72733|  %3005 = phi i64 [ %3003, %3000 ], [ %239, %2999 ], [ %239, %2994 ]                                                    ;L0
 72734|     ;; move_speed = i64 %3005
 72736|  %3006 = load i64, ptr %2916, , !!8                                                                                    ;L26<1900
 72737|  %3007 = load i64, ptr %2917, , !!8                                                                                    ;L26<1900
 72738|  %3008 = load i64, ptr %2918, , !!8                                                                                    ;L26<1900
 72739|  %3009 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %145, ptr %86, ptr %2931)
 72740|  to label %3010 unwind label %178                                                                                      ;L1900
 72741| 
 72742| 3010: ; preds = %3004
 72743|  %3011 = mul i64 %3007, %2919                                                                                          ;L26<1900
 72744|  %3012 = load i32, ptr %2920, , !!8                                                                                    ;L1511<1900
 72745|     ;; mult = i32 %3012
 72746|  %3013 = icmp eq i32 %3012, 0                                                                                          ;L1512<1900
 72747|  br i1 %3013, label %3014, label %3016                                                                                 ;L1512<1900
 72748| 
 72749| 3014: ; preds = %3010
 72750|  %3015 = load i64, ptr %2921, , !!8                                                                                    ;L1513<1900
 72751|  br label %3022                                                                                                        ;L1512<1900
 72752| 
 72753| 3016: ; preds = %3010
 72754|  %3017 = sext i32 %3012 to i64                                                                                         ;L1511<1900
 72755|     ;; mult = i64 %3017
 72756|  %3018 = load i64, ptr %2921, , !!8                                                                                    ;L1515<1900
 72757|  %3019 = add nsw i64 %3017, 100                                                                                        ;L1515<1900
 72758|  %3020 = mul i64 %3018, %3019                                                                                          ;L1515<1900
 72759|  %3021 = udiv i64 %3020, 100                                                                                           ;L1515<1900
 72760|  br label %3022                                                                                                        ;L1512<1900
 72761| 
 72762| 3022: ; preds = %3016, %3014
 72763|  %3023 = phi i64 [ %3015, %3014 ], [ %3021, %3016 ]                                                                    ;L0<1900
 72764|  %3024 = gep %2931, i64 1136                                                                                           ;L1511<1900
 72765|  %3025 = load i32, ptr %3024, , !!8                                                                                    ;L1511<1900
 72766|     ;; mult = i32 %3025
 72767|  %3026 = icmp eq i32 %3025, 0                                                                                          ;L1512<1900
 72768|  br i1 %3026, label %3027, label %3030                                                                                 ;L1512<1900
 72769| 
 72770| 3027: ; preds = %3022
 72771|  %3028 = gep %2931, i64 1664                                                                                           ;L1513<1900
 72772|  %3029 = load i64, ptr %3028, , !!8                                                                                    ;L1513<1900
 72773|  br label %3037                                                                                                        ;L1512<1900
 72774| 
 72775| 3030: ; preds = %3022
 72776|  %3031 = sext i32 %3025 to i64                                                                                         ;L1511<1900
 72777|     ;; mult = i64 %3031
 72778|  %3032 = gep %2931, i64 1664                                                                                           ;L1515<1900
 72779|  %3033 = load i64, ptr %3032, , !!8                                                                                    ;L1515<1900
 72780|  %3034 = add nsw i64 %3031, 100                                                                                        ;L1515<1900
 72781|  %3035 = mul i64 %3033, %3034                                                                                          ;L1515<1900
 72782|  %3036 = udiv i64 %3035, 100                                                                                           ;L1515<1900
 72783|  br label %3037                                                                                                        ;L1512<1900
 72784| 
 72785| 3037: ; preds = %3030, %3027
 72786|  %3038 = phi i64 [ %3029, %3027 ], [ %3036, %3030 ]                                                                    ;L0<1900
 72788|  %3039 = gep %2931, i64 1632                                                                                           ;L2158<1901
 72789|  %3040 = load i64, ptr %3039, , !!8                                                                                    ;L2158<1901
 72790|     ;; x1 = i64 %3040
 72791|     ;; self = i64 %3040
 72792|  %3041 = gep %2931, i64 1640                                                                                           ;L2158<1901
 72793|  %3042 = load i64, ptr %3041, , !!8                                                                                    ;L2158<1901
 72794|     ;; y1 = i64 %3042
 72795|     ;; self = i64 %3042
 72796|  %3043 = load i64, ptr %1459, , !!8                                                                                    ;L2158<1901
 72797|     ;; x2 = i64 %3043
 72798|     ;; other = i64 %3043
 72799|  %3044 = load i64, ptr %1460, , !!8                                                                                    ;L2158<1901
 72800|     ;; y2 = i64 %3044
 72801|     ;; other = i64 %3044
 72802|  %3045 = icmp ult i64 %3040, %3043                                                                                     ;L3147<7<2158<1901
 72803|  %3046 = sub nuw i64 %3043, %3040                                                                                      ;L3147<7<2158<1901
 72804|  %3047 = sub nuw i64 %3040, %3043                                                                                      ;L3147<7<2158<1901
 72805|  %3048 = select i1 %3045, i64 %3046, i64 %3047                                                                         ;L3147<7<2158<1901
 72806|     ;; dx = i64 %3048
 72807|  %3049 = icmp ult i64 %3042, %3044                                                                                     ;L3147<8<2158<1901
 72808|  %3050 = sub nuw i64 %3044, %3042                                                                                      ;L3147<8<2158<1901
 72809|  %3051 = sub nuw i64 %3042, %3044                                                                                      ;L3147<8<2158<1901
 72810|  %3052 = select i1 %3049, i64 %3050, i64 %3051                                                                         ;L3147<8<2158<1901
 72811|     ;; dy = i64 %3052
 72812|  %3053 = mul i64 %3048, %3048                                                                                          ;L9<2158<1901
 72813|  %3054 = mul i64 %3052, %3052                                                                                          ;L9<2158<1901
 72814|  %3055 = add i64 %3054, %3053                                                                                          ;L9<2158<1901
 72815|     ;; dist_sq = i64 %3055
 72816|     ;; max_tick = i64 %97
 72817|  %3056 = mul i64 %3005, %97                                                                                            ;L1910
 72818|  %3057 = add i64 %3056, %3006                                                                                          ;L26<1900
 72819|  %3058 = add i64 %3057, %3011                                                                                          ;L26<1900
 72820|  %3059 = add i64 %3058, %3008                                                                                          ;L1900
 72821|  %3060 = add i64 %3059, %3009                                                                                          ;L1900
 72822|  %3061 = add i64 %3060, %3023                                                                                          ;L1900
 72823|  %3062 = add i64 %3061, %3038                                                                                          ;L1910
 72824|     ;; max_dist = i64 %3062
 72825|  %3063 = mul i64 %3062, %3062                                                                                          ;L1911
 72826|  %3064 = icmp ugt i64 %3055, %3063                                                                                     ;L1911
 72827|  br i1 %3064, label %3139, label %3065                                                                                 ;L1911
 72828| 
 72829| 3065: ; preds = %3037
 72830|  switch i64 %4, label %3099 [
 72831|  i64 3, label %3066
 72832|  i64 4, label %3066
 72833|  i64 0, label %3093
 72834|  i64 5, label %3093
 72835|  ]                                                                                                                     ;L1915
 72836| 
 72837| 3066: ; preds = %3065, %3065
 72840|  %3067 = load ptr, ptr %145, , !!8, !!8                                                                                ;L441<2127<2445<1917
 72841|  %3068 = load ptr, ptr %2922, , !!8, !!8                                                                               ;L441<2127<2445<1917
 72842|  %3069 = gep %3068, i64 16                                                                                             ;L2445<1917
 72843|  %3070 = load i64, ptr %3069,                                                                                          ;L2445<1917
 72844|  %3071 = add nsw i64 %3070, -1                                                                                         ;L2445<1917
 72845|  %3072 = and i64 %3071, -16                                                                                            ;L2445<1917
 72846|  %3073 = gep %3067, i64 %3072                                                                                          ;L2445<1917
 72847|  %3074 = gep %3073, i64 16                                                                                             ;L2445<1917
 72848|  %3075 = gep %3068, i64 104                                                                                            ;L1917
 72849|  %3076 = load ptr, ptr %3075, , !!8                                                                                    ;L1917
 72850|  %3077 = invoke zeroext i1 %3076(ptr %3074)
 72851|  to label %3078 unwind label %178                                                                                      ;L1917
 72852| 
 72853| 3078: ; preds = %3066
 72854|  br i1 %3077, label %3139, label %3079                                                                                 ;L1917
 72855| 
 72856| 3079: ; preds = %3078
 72860|  %3080 = load ptr, ptr %145, , !!8, !!8                                                                                ;L441<2127<2445<1917
 72861|  %3081 = load ptr, ptr %2922, , !!8, !!8                                                                               ;L441<2127<2445<1917
 72862|  %3082 = gep %3081, i64 16                                                                                             ;L2445<1917
 72863|  %3083 = load i64, ptr %3082,                                                                                          ;L2445<1917
 72864|  %3084 = add nsw i64 %3083, -1                                                                                         ;L2445<1917
 72865|  %3085 = and i64 %3084, -16                                                                                            ;L2445<1917
 72866|  %3086 = gep %3080, i64 %3085                                                                                          ;L2445<1917
 72867|  %3087 = gep %3086, i64 16                                                                                             ;L2445<1917
 72868|  %3088 = gep %3081, i64 88                                                                                             ;L1917
 72869|  %3089 = load ptr, ptr %3088, , !!8                                                                                    ;L1917
 72870|  invoke void %3089(ptr sret([24 x i8]) %17, ptr %3087)
 72871|  to label %3090 unwind label %178                                                                                      ;L1917
 72872| 
 72873| 3090: ; preds = %3079
 72874|     ;; self = ptr %17
 72875|  %3091 = load i64, ptr %17, , !!8                                                                                      ;L633<1917
 72876|  %3092 = icmp eq i64 %3091, 0                                                                                          ;L1917
 72878|  br i1 %3092, label %3099, label %3139                                                                                 ;L1917
 72879| 
 72880| 3093: ; preds = %3120, %3112, %3111, %3065, %3065
 72881|  %3094 = load ptr, ptr %2888, , !!8, !!8                                                                               ;L1931
 72882|  %3095 = load ptr, ptr %2890, , !!8, !!8                                                                               ;L1931
 72883|  %3096 = gep %3095, i64 192                                                                                            ;L1931
 72884|  %3097 = load ptr, ptr %3096, , !!8                                                                                    ;L1931
 72885|  %3098 = invoke zeroext i1 %3097(ptr %3094, ptr %2959, ptr %2960, ptr %86)
 72886|  to label %3121 unwind label %178                                                                                      ;L1931
 72887| 
 72888| 3099: ; preds = %3090, %3065
 72891|  %3100 = load ptr, ptr %145, , !!8, !!8                                                                                ;L441<2127<2445<1924
 72892|  %3101 = load ptr, ptr %2923, , !!8, !!8                                                                               ;L441<2127<2445<1924
 72893|  %3102 = gep %3101, i64 16                                                                                             ;L2445<1924
 72894|  %3103 = load i64, ptr %3102,                                                                                          ;L2445<1924
 72895|  %3104 = add nsw i64 %3103, -1                                                                                         ;L2445<1924
 72896|  %3105 = and i64 %3104, -16                                                                                            ;L2445<1924
 72897|  %3106 = gep %3100, i64 %3105                                                                                          ;L2445<1924
 72898|  %3107 = gep %3106, i64 16                                                                                             ;L2445<1924
 72899|  %3108 = gep %3101, i64 288                                                                                            ;L1924
 72900|  %3109 = load ptr, ptr %3108, , !!8                                                                                    ;L1924
 72901|  %3110 = invoke zeroext i1 %3109(ptr %3107)
 72902|  to label %3111 unwind label %178                                                                                      ;L1924
 72903| 
 72904| 3111: ; preds = %3099
 72905|  br i1 %3110, label %3112, label %3093                                                                                 ;L1924
 72906| 
 72907| 3112: ; preds = %3111
 72908|     ;; self = ptr %2931
 72909|  %3113 = gep %2931, i64 104                                                                                            ;L1404<1924
 72910|  %3114 = load i64, ptr %3113, , !!8                                                                                    ;L1404<1924
 72911|  %3115 = icmp eq i64 %3114, 13                                                                                         ;L1924
 72912|  br i1 %3115, label %3116, label %3093                                                                                 ;L1924
 72913| 
 72914| 3116: ; preds = %3112
 72915|  %3117 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %145, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %2931)
 72916|  to label %3118 unwind label %178                                                                                      ;L1925
 72917| 
 72918| 3118: ; preds = %3116
 72919|     ;; skill_damage = i64 %3117
 72920|  %3119 = invoke zeroext i1 @ai::utils13is_dash_worth(ptr %3, ptr %2, ptr %86, ptr %2931, i64 %3117)
 72921|  to label %3120 unwind label %178                                                                                      ;L1926
 72922| 
 72923| 3120: ; preds = %3118
 72924|  br i1 %3119, label %3093, label %3139                                                                                 ;L1926
 72925| 
 72926| 3121: ; preds = %3093
 72927|  br i1 %3098, label %3122, label %3139                                                                                 ;L1931
 72928| 
 72929| 3122: ; preds = %3121
 72932|  %3123 = gep %2931, i64 1472                                                                                           ;L1935
 72933|  %3124 = load i64, ptr %3123, , !!8                                                                                    ;L1935
 72934|  invoke void @ai::small_action4castNtB5_14SmallActionUlt3new(ptr sret([24 x i8]) %15, ptr %3, i64 %3124)
 72935|  to label %3125 unwind label %178                                                                                      ;L1935
 72936| 
 72937| 3125: ; preds = %3122
 72938|  call void @llvm.memcpy.p0.p0.i64(ptr %16, ptr %15, i64 24, i1 false)                                                  ;L1935
 72939|  store i8 18, ptr %2924,                                                                                               ;L1935
 72942|     ;; self = ptr %65
 72943|     ;; self = ptr %65
 72944|     ;; value = ptr %16
 72945|     ;; src = ptr %16
 72946|     ;; additional = i64 1
 72947|     ;; needed_extra_cap = i64 1
 72948|     ;; needed_extra_cap = i64 1
 72949|     ;; strategy = i8 1
 72950|  %3126 = load i64, ptr %126, , !!76761, !!8                                                                            ;L1428<1935
 72951|     ;; self = ptr %65
 72952|  %3127 = load i64, ptr %125, , !!76761, !!8                                                                            ;L149<1428<1935
 72953|  %3128 = icmp eq i64 %3126, %3127                                                                                      ;L1428<1935
 72954|  br i1 %3128, label %3129, label %3134                                                                                 ;L1428<1935
 72955| 
 72956| 3129: ; preds = %3125
 72957|     ;; self = ptr %65
 72958|     ;; self = ptr %65
 72959|     ;; self = ptr %65
 72960|     ;; used_cap = i64 %3126
 72961|     ;; used_cap = i64 %3126
 72962|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %3126, i64 1, i1 zeroext true)
 72963|  to label %3130 unwind label %3132, !!76761                                                                            ;L619<430<738<1429<1935
 72964| 
 72965| 3130: ; preds = %3129
 72966|  %3131 = load i64, ptr %126, , !!76761                                                                                 ;L1432<1935
 72967|  br label %3134                                                                                                        ;L619<430<738<1429<1935
 72968| 
 72969| 3132: ; preds = %3129
 72970|  %3133 = cleanuppad within none []
 72971|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %16) #30 [ "funclet"(token %3133) ], !!76746 ;L1436<1935
 72972|  cleanupret from %3133 unwind label %178
 72973| 
 72974| 3134: ; preds = %3130, %3125
 72975|  %3135 = phi i64 [ %3131, %3130 ], [ %3126, %3125 ]                                                                    ;L1432<1935
 72976|     ;; self = ptr %65
 72977|  %3136 = load ptr, ptr %65, , !!76761, !!8, !!8                                                                        ;L138<1432<1935
 72978|     ;; self = ptr %3136
 72979|     ;; count = i64 %3135
 72980|  %3137 = gepS %3136, i64 %3135                                                                                         ;L961<1432<1935
 72981|     ;; end = ptr %3137
 72982|     ;; dst = ptr %3137
 72983|  call void @llvm.memcpy.p0.p0.i64(ptr %3137, ptr %16, i64 184, i1 false), !!76746                                      ;L1933<1433<1935
 72984|  %3138 = add i64 %3135, 1                                                                                              ;L1434<1935
 72985|  store i64 %3138, ptr %126, , !!76761                                                                                  ;L1434<1935
 72987|  br label %3139                                                                                                        ;L1874
 72988| 
 72989| 3139: ; preds = %3134, %3121, %3120, %3090, %3078, %3037, %2973, %2964, %2955, %2947
 72990|  br label %2925                                                                                                        ;L1714<180<1874
 72991| 
 72992| 3140: ; preds = %3432, %2934
 72993|  %3141 = phi ptr [ %2935, %2934 ], [ %3144, %3432 ]                                                                    ;L1938
 72994|     ;; iter[0..+8] = ptr %3141
 72995|     ;; self = ptr undef
 72996|     ;; ptr = ptr %3141
 72997|     ;; self = ptr %3141
 72998|     ;; end_or_len = ptr %2938
 73001|  %3142 = icmp eq ptr %3141, %2938                                                                                      ;L1714<180<1938
 73002|  br i1 %3142, label %3433, label %3143                                                                                 ;L180<1938
 73003| 
 73004| 3143: ; preds = %3140
 73005|  %3144 = gep %3141, i64 8                                                                                              ;L656<185<1938
 73006|     ;; iter[0..+8] = ptr %3144
 73007|     ;; e = ptr %3141
 73008|  %3145 = load ptr, ptr %3141, , !!8, !!8                                                                               ;L1939
 73009|  %3146 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %2915, ptr %86, ptr %3145)
 73010|  to label %3147 unwind label %178                                                                                      ;L1939
 73011| 
 73012| 3147: ; preds = %3143
 73013|  br i1 %3146, label %3148, label %3432                                                                                 ;L1939
 73014| 
 73015| 3148: ; preds = %3147
 73016|  %3149 = load ptr, ptr %2888, , !!8, !!8                                                                               ;L1943
 73017|  %3150 = load ptr, ptr %2890, , !!8, !!8                                                                               ;L1943
 73018|  %3151 = load ptr, ptr %82, , !!8, !!8                                                                                 ;L1943
 73019|  %3152 = load ptr, ptr %2632, , !!8, !!8                                                                               ;L1943
 73020|  %3153 = load ptr, ptr %3141, , !!8, !!8                                                                               ;L1943
 73021|  %3154 = gep %3150, i64 200                                                                                            ;L1943
 73022|  %3155 = load ptr, ptr %3154, , !!8                                                                                    ;L1943
 73023|  %3156 = invoke zeroext i1 %3155(ptr %3149, ptr %3151, ptr %3152, ptr %86, ptr %3153)
 73024|  to label %3157 unwind label %178                                                                                      ;L1943
 73025| 
 73026| 3157: ; preds = %3148
 73027|  br i1 %3156, label %3158, label %3432                                                                                 ;L1943
 73028| 
 73029| 3158: ; preds = %3157
 73031|  %3159 = load i64, ptr %73, , !!8                                                                                      ;L1949
 73032|  invoke void @ai::fight_check18effect_buff_target(ptr sret([288 x i8]) %14, i64 %3159, ptr %145, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54)
 73033|  to label %3160 unwind label %178                                                                                      ;L1949
 73034| 
 73035| 3160: ; preds = %3158
 73038|  %3161 = load ptr, ptr %145, , !!8, !!8                                                                                ;L441<2127<2445<1950
 73039|  %3162 = load ptr, ptr %2922, , !!8, !!8                                                                               ;L441<2127<2445<1950
 73040|  %3163 = gep %3162, i64 16                                                                                             ;L2445<1950
 73041|  %3164 = load i64, ptr %3163,                                                                                          ;L2445<1950
 73042|  %3165 = add nsw i64 %3164, -1                                                                                         ;L2445<1950
 73043|  %3166 = and i64 %3165, -16                                                                                            ;L2445<1950
 73044|  %3167 = gep %3161, i64 %3166                                                                                          ;L2445<1950
 73045|  %3168 = gep %3167, i64 16                                                                                             ;L2445<1950
 73046|  %3169 = gep %3162, i64 144                                                                                            ;L1950
 73047|  %3170 = load ptr, ptr %3169, , !!8                                                                                    ;L1950
 73048|  %3171 = invoke zeroext i1 %3170(ptr %3168)
 73049|  to label %3172 unwind label %178                                                                                      ;L1950
 73050| 
 73051| 3172: ; preds = %3160
 73052|     ;; is_etc_buff = i1 %3171
 73053|     ;; self = ptr %14
 73054|  %3173 = load i32, ptr %2939, , !!8                                                                                    ;L633<1954
 73055|  %3174 = icmp ne i32 %3173, -1                                                                                         ;L633<1954
 73056|  %3175 = or i1 %3171, %3174                                                                                            ;L1954
 73057|  br i1 %3175, label %3196, label %3176                                                                                 ;L1954
 73058| 
 73059| 3176: ; preds = %3172
 73062|  %3177 = load ptr, ptr %145, , !!8, !!8                                                                                ;L441<2127<2445<1955
 73063|  %3178 = load ptr, ptr %2922, , !!8, !!8                                                                               ;L441<2127<2445<1955
 73064|  %3179 = gep %3178, i64 16                                                                                             ;L2445<1955
 73065|  %3180 = load i64, ptr %3179,                                                                                          ;L2445<1955
 73066|  %3181 = add nsw i64 %3180, -1                                                                                         ;L2445<1955
 73067|  %3182 = and i64 %3181, -16                                                                                            ;L2445<1955
 73068|  %3183 = gep %3177, i64 %3182                                                                                          ;L2445<1955
 73069|  %3184 = gep %3183, i64 16                                                                                             ;L2445<1955
 73070|  %3185 = gep %3178, i64 176                                                                                            ;L1955
 73071|  %3186 = load ptr, ptr %3185, , !!8                                                                                    ;L1955
 73072|  %3187 = invoke i64 %3186(ptr %3184, ptr %102, ptr %86)
 73073|  to label %3188 unwind label %178                                                                                      ;L1955
 73074| 
 73075| 3188: ; preds = %3176
 73076|  %3189 = icmp eq i64 %3187, 0                                                                                          ;L1955
 73077|     ;; gate_applies = i1 %3189
 73078|  br i1 %3189, label %3190, label %3196                                                                                 ;L1956
 73079| 
 73080| 3190: ; preds = %3333, %3313, %3188
 73083|  %3191 = load i64, ptr %2916, , !!8                                                                                    ;L26<1983
 73084|  %3192 = load i64, ptr %2917, , !!8                                                                                    ;L26<1983
 73085|  %3193 = load i64, ptr %2918, , !!8                                                                                    ;L26<1983
 73086|  %3194 = load ptr, ptr %3141, , !!8, !!8                                                                               ;L1983
 73087|  %3195 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %145, ptr %86, ptr %3194)
 73088|  to label %3352 unwind label %178                                                                                      ;L1983
 73089| 
 73090| 3196: ; preds = %3188, %3172
 73091|  %3197 = load ptr, ptr %3141, , !!8, !!8                                                                               ;L1963
 73092|  %3198 = gep %3197, i64 1600                                                                                           ;L1963
 73093|  %3199 = load i64, ptr %3198, , !!8                                                                                    ;L1963
 73094|  %3200 = mul i64 %3199, 60                                                                                             ;L1963
 73095|     ;; engage_extra = i64 %3200
 73096|     ;; caster_extra = i64 %2940
 73097|     ;; self = ptr %3197
 73098|  %3201 = gep %3197, i64 1216                                                                                           ;L742<1965
 73099|  %3202 = load i32, ptr %3201, , !!8                                                                                    ;L742<1965
 73100|  %3203 = icmp eq i32 %3202, -1                                                                                         ;L742<1965
 73101|  br i1 %3203, label %3351, label %3204                                                                                 ;L742<1965
 73102| 
 73103| 3204: ; preds = %3196
 73104|  %3205 = gep %3197, i64 1168                                                                                           ;L742<1965
 73105|     ;; target_atk = ptr %3205
 73106|     ;; self = ptr %70
 73107|     ;; self = ptr %70
 73108|  %3206 = load ptr, ptr %70, , !!8, !!8                                                                                 ;L138<2073<1966
 73109|     ;; p = ptr %3206
 73110|  %3207 = load i64, ptr %2912, , !!8                                                                                    ;L2075<1966
 73111|     ;; len = i64 %3207
 73112|     ;; count = i64 %3207
 73113|     ;; self[0..+8] = ptr %3206
 73114|     ;; slice[0..+8] = ptr %3206
 73115|     ;; self[8..+8] = i64 %3207
 73116|     ;; slice[8..+8] = i64 %3207
 73117|     ;; ptr = ptr %3206
 73118|     ;; self = ptr %3206
 73119|  %3208 = shl nuw nsw i64 %3207, 5                                                                                      ;L961<100<1042<1966
 73120|  %3209 = gep %3206, i64 %3208                                                                                          ;L961<100<1042<1966
 73121|     ;; f[0..+8] = ptr %3205
 73122|     ;; f[8..+8] = ptr %3197
 73123|     ;; f[16..+8] = ptr undef
 73124|     ;; f[24..+8] = ptr %86
 73125|     ;; f[32..+8] = ptr undef
 73126|     ;; self = ptr undef
 73127|     ;; self = ptr undef
 73128|     ;; count = i64 1
 73129|     ;; ptr = ptr %3206
 73130|     ;; self = ptr %3206
 73131|     ;; end_or_len = ptr %3209
 73134|  %3210 = icmp eq i64 %3207, 0                                                                                          ;L1714<180<331<1966
 73135|  br i1 %3210, label %3351, label %3211                                                                                 ;L180<331<1966
 73136| 
 73137| 3211: ; preds = %3204
 73138|  %3212 = gep %3197, i64 1184
 73139|  %3213 = gep %3197, i64 1192
 73140|  %3214 = gep %3197, i64 1480
 73141|  %3215 = gep %3197, i64 1080
 73142|  %3216 = gep %3197, i64 1136
 73143|  %3217 = gep %3197, i64 1664
 73144|  %3218 = gep %3197, i64 1632
 73145|  %3219 = gep %3197, i64 1640
 73146|  br label %3220                                                                                                        ;L180<331<1966
 73147| 
 73148| 3220: ; preds = %3349, %3211
 73149|  %3221 = phi ptr [ %3206, %3211 ], [ %3222, %3349 ]
 73150|     ;; ptr = ptr %3221
 73151|  %3222 = gep %3221, i64 32                                                                                             ;L656<185<331<1966
 73152|     ;; x = ptr %3221
 73153|  %3223 = gep %3221, i64 24                                                                                             ;L332<1966
 73154|  %3224 = load ptr, ptr %3223, , !!76860, !!8, !!8                                                                      ;L332<1966
 73161|     ;; default = i64 0
 73163|     ;; self = ptr %3205
 73164|     ;; caster = ptr %3197
 73165|  %3225 = load i64, ptr %3212, , !!76899, !!8                                                                           ;L26<1967<332<1966
 73166|  %3226 = load i64, ptr %3213, , !!76899, !!8                                                                           ;L26<1967<332<1966
 73167|  %3227 = load i64, ptr %3214, , !!76899, !!8                                                                           ;L26<1967<332<1966
 73168|  %3228 = add i64 %3227, -1                                                                                             ;L26<1967<332<1966
 73169|  %3229 = mul i64 %3228, %3226                                                                                          ;L26<1967<332<1966
 73170|  %3230 = load i64, ptr %3215, , !!76899, !!8                                                                           ;L26<1967<332<1966
 73171|     ;; self = ptr %3224
 73173|     ;; self = ptr %3224
 73174|     ;; self = ptr %3224
 73175|     ;; self = ptr %3224
 73176|     ;; self = ptr %3224
 73177|  %3231 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %3205, ptr %3197, ptr %3224)
 73178|  to label %3232 unwind label %178                                                                                      ;L1967<332<1966
 73179| 
 73180| 3232: ; preds = %3220
 73181|     ;; self = ptr %3197
 73182|  %3233 = load i32, ptr %3216, , !!76899, !!8                                                                           ;L1511<1967<332<1966
 73183|     ;; mult = i32 %3233
 73184|  %3234 = icmp eq i32 %3233, 0                                                                                          ;L1512<1967<332<1966
 73185|  br i1 %3234, label %3235, label %3237                                                                                 ;L1512<1967<332<1966
 73186| 
 73187| 3235: ; preds = %3232
 73188|  %3236 = load i64, ptr %3217, , !!76899, !!8                                                                           ;L1513<1967<332<1966
 73189|  br label %3243                                                                                                        ;L1512<1967<332<1966
 73190| 
 73191| 3237: ; preds = %3232
 73192|  %3238 = sext i32 %3233 to i64                                                                                         ;L1511<1967<332<1966
 73193|     ;; mult = i64 %3238
 73194|  %3239 = load i64, ptr %3217, , !!76899, !!8                                                                           ;L1515<1967<332<1966
 73195|  %3240 = add nsw i64 %3238, 100                                                                                        ;L1515<1967<332<1966
 73196|  %3241 = mul i64 %3239, %3240                                                                                          ;L1515<1967<332<1966
 73197|  %3242 = udiv i64 %3241, 100                                                                                           ;L1515<1967<332<1966
 73198|  br label %3243                                                                                                        ;L1512<1967<332<1966
 73199| 
 73200| 3243: ; preds = %3237, %3235
 73201|  %3244 = phi i64 [ %3236, %3235 ], [ %3242, %3237 ]                                                                    ;L0<1967<332<1966
 73202|  %3245 = gep %3224, i64 1136                                                                                           ;L1511<1967<332<1966
 73203|  %3246 = load i32, ptr %3245, , !!76899, !!8                                                                           ;L1511<1967<332<1966
 73204|  %3247 = sext i32 %3246 to i64                                                                                         ;L1511<1967<332<1966
 73205|     ;; mult = i64 %3247
 73206|     ;; mult = i64 %3247
 73207|  %3248 = icmp eq i32 %3246, 0                                                                                          ;L1512<1967<332<1966
 73208|  %3249 = gep %3224, i64 1664                                                                                           ;L0<1967<332<1966
 73209|  %3250 = load i64, ptr %3249, , !!76899, !!8                                                                           ;L0<1967<332<1966
 73210|  br i1 %3248, label %3255, label %3251                                                                                 ;L1512<1967<332<1966
 73211| 
 73212| 3251: ; preds = %3243
 73213|  %3252 = add nsw i64 %3247, 100                                                                                        ;L1515<1967<332<1966
 73214|  %3253 = mul i64 %3252, %3250                                                                                          ;L1515<1967<332<1966
 73215|  %3254 = udiv i64 %3253, 100                                                                                           ;L1515<1967<332<1966
 73216|  br label %3255                                                                                                        ;L1512<1967<332<1966
 73217| 
 73218| 3255: ; preds = %3251, %3243
 73219|  %3256 = phi i64 [ %3254, %3251 ], [ %3250, %3243 ]                                                                    ;L0<1967<332<1966
 73220|  %3257 = add i64 %3225, %3200                                                                                          ;L26<1967<332<1966
 73221|  %3258 = add i64 %3257, %3230                                                                                          ;L26<1967<332<1966
 73222|  %3259 = add i64 %3258, %3229                                                                                          ;L1967<332<1966
 73223|  %3260 = add i64 %3259, %3231                                                                                          ;L1967<332<1966
 73224|  %3261 = add i64 %3260, %3244                                                                                          ;L1967<332<1966
 73225|  %3262 = add i64 %3261, %3256                                                                                          ;L1967<332<1966
 73226|     ;; attack_range = i64 %3262
 73227|     ;; self = ptr %86
 73228|  %3263 = load i32, ptr %128, , !!76899, !!8                                                                            ;L742<1968<332<1966
 73229|  %3264 = icmp eq i32 %3263, -1                                                                                         ;L742<1968<332<1966
 73230|  br i1 %3264, label %3277, label %3265                                                                                 ;L742<1968<332<1966
 73231| 
 73232| 3265: ; preds = %3255
 73233|     ;; self = ptr %127
 73234|     ;; f[0..+8] = ptr %86
 73235|     ;; f[8..+8] = ptr %3224
 73236|     ;; x = ptr %127
 73237|     ;; a = ptr %127
 73238|     ;; self = ptr %127
 73240|     ;; caster = ptr %86
 73241|  %3266 = load i64, ptr %2941, , !!76899, !!8                                                                           ;L26<1969<1162<1969<332<1966
 73242|  %3267 = load i64, ptr %2942, , !!76899, !!8                                                                           ;L26<1969<1162<1969<332<1966
 73243|  %3268 = load i64, ptr %135, , !!76899, !!8                                                                            ;L26<1969<1162<1969<332<1966
 73244|  %3269 = load i64, ptr %2918, , !!76899, !!8                                                                           ;L26<1969<1162<1969<332<1966
 73245|  %3270 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %127, ptr %86, ptr %3224)
 73246|  to label %3271 unwind label %178                                                                                      ;L1969<1162<1969<332<1966
 73247| 
 73248| 3271: ; preds = %3265
 73249|  %3272 = add i64 %3268, -1                                                                                             ;L26<1969<1162<1969<332<1966
 73250|  %3273 = mul i64 %3272, %3267                                                                                          ;L26<1969<1162<1969<332<1966
 73251|  %3274 = add i64 %3269, %3266                                                                                          ;L26<1969<1162<1969<332<1966
 73252|  %3275 = add i64 %3274, %3273                                                                                          ;L26<1969<1162<1969<332<1966
 73253|  %3276 = add i64 %3275, %3270                                                                                          ;L1969<1162<1969<332<1966
 73254|     ;; self[8..+8] = i64 %3276
 73255|     ;; self[0..+8] = i64 1
 73256|  br label %3277                                                                                                        ;L1043<1969<332<1966
 73257| 
 73258| 3277: ; preds = %3271, %3255
 73259|  %3278 = phi i64 [ %3276, %3271 ], [ 0, %3255 ]                                                                        ;L0<1969<332<1966
 73260|     ;; self = ptr %86
 73261|  %3279 = load i32, ptr %2920, , !!76899, !!8                                                                           ;L1511<1970<332<1966
 73262|     ;; mult = i32 %3279
 73263|  %3280 = icmp eq i32 %3279, 0                                                                                          ;L1512<1970<332<1966
 73264|  br i1 %3280, label %3281, label %3283                                                                                 ;L1512<1970<332<1966
 73265| 
 73266| 3281: ; preds = %3277
 73267|  %3282 = load i64, ptr %2921, , !!76899, !!8                                                                           ;L1513<1970<332<1966
 73268|  br label %3289                                                                                                        ;L1512<1970<332<1966
 73269| 
 73270| 3283: ; preds = %3277
 73271|  %3284 = sext i32 %3279 to i64                                                                                         ;L1511<1970<332<1966
 73272|     ;; mult = i64 %3284
 73273|  %3285 = load i64, ptr %2921, , !!76899, !!8                                                                           ;L1515<1970<332<1966
 73274|  %3286 = add nsw i64 %3284, 100                                                                                        ;L1515<1970<332<1966
 73275|  %3287 = mul i64 %3285, %3286                                                                                          ;L1515<1970<332<1966
 73276|  %3288 = udiv i64 %3287, 100                                                                                           ;L1515<1970<332<1966
 73277|  br label %3289                                                                                                        ;L1512<1970<332<1966
 73278| 
 73279| 3289: ; preds = %3283, %3281
 73280|  %3290 = phi i64 [ %3282, %3281 ], [ %3288, %3283 ]                                                                    ;L0<1970<332<1966
 73281|  %3291 = load i64, ptr %3249, , !!76899, !!8                                                                           ;L0<1970<332<1966
 73282|  br i1 %3248, label %3296, label %3292                                                                                 ;L1512<1970<332<1966
 73283| 
 73284| 3292: ; preds = %3289
 73285|  %3293 = add nsw i64 %3247, 100                                                                                        ;L1515<1970<332<1966
 73286|  %3294 = mul i64 %3291, %3293                                                                                          ;L1515<1970<332<1966
 73287|  %3295 = udiv i64 %3294, 100                                                                                           ;L1515<1970<332<1966
 73288|  br label %3296                                                                                                        ;L1512<1970<332<1966
 73289| 
 73290| 3296: ; preds = %3292, %3289
 73291|  %3297 = phi i64 [ %3295, %3292 ], [ %3291, %3289 ]                                                                    ;L0<1970<332<1966
 73292|  %3298 = add i64 %3278, %2940                                                                                          ;L1968<332<1966
 73293|  %3299 = add i64 %3298, %3290                                                                                          ;L1968<332<1966
 73294|  %3300 = add i64 %3299, %3297                                                                                          ;L1968<332<1966
 73295|     ;; caster_range = i64 %3300
 73296|     ;; entity = ptr %86
 73297|     ;; self = ptr %86
 73298|  %3301 = load i64, ptr %86, , !!76899, !!8                                                                             ;L1136<1482<1971<332<1966
 73299|  %3302 = trunc nuw i64 %3301 to i1                                                                                     ;L1136<1482<1971<332<1966
 73300|  br i1 %3302, label %3313, label %3303                                                                                 ;L1136<1482<1971<332<1966
 73301| 
 73302| 3303: ; preds = %3296
 73303|     ;; team = ptr %86
 73304|  %3304 = load i64, ptr %1458, , !!76899, !!8                                                                           ;L1137<1482<1971<332<1966
 73305|     ;; team = i64 %3304
 73306|  %3305 = icmp ult i64 %3304, 2                                                                                         ;L1483<1971<332<1966
 73307|  br i1 %3305, label %3306, label %3311                                                                                 ;L1483<1971<332<1966
 73308| 
 73309| 3306: ; preds = %3303
 73311|  %3307 = gep %3224, i64 56                                                                                             ;L122<1483<1971<332<1966
 73312|  %3308 = gepS %3307, i64 %3304                                                                                         ;L122<1483<1971<332<1966
 73313|  %3309 = load i64, ptr %3308, , !!76899, !!8                                                                           ;L122<1483<1971<332<1966
 73314|  %3310 = icmp eq i64 %3309, 0                                                                                          ;L122<1483<1971<332<1966
 73315|  br i1 %3310, label %3313, label %3349                                                                                 ;L1971<332<1966
 73316| 
 73317| 3311: ; preds = %3303
 73318|  invoke void @core::panicking18panic_bounds_check(i64 %3304, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 73319|  to label %3312 unwind label %178                                                                                      ;L1483<1971<332<1966
 73320| 
 73321| 3312: ; preds = %3311
 73322|  unreachable                                                                                                           ;L1483<1971<332<1966
 73323| 
 73324| 3313: ; preds = %3306, %3296
 73325|     ;; other = ptr %3197
 73326|  %3314 = gep %3224, i64 1632                                                                                           ;L2158<1971<332<1966
 73327|  %3315 = load i64, ptr %3314, , !!76899, !!8                                                                           ;L2158<1971<332<1966
 73328|     ;; x1 = i64 %3315
 73329|     ;; self = i64 %3315
 73330|     ;; x1 = i64 %3315
 73331|     ;; self = i64 %3315
 73332|  %3316 = gep %3224, i64 1640                                                                                           ;L2158<1971<332<1966
 73333|  %3317 = load i64, ptr %3316, , !!76899, !!8                                                                           ;L2158<1971<332<1966
 73334|     ;; y1 = i64 %3317
 73335|     ;; self = i64 %3317
 73336|     ;; y1 = i64 %3317
 73337|     ;; self = i64 %3317
 73338|  %3318 = load i64, ptr %3218, , !!76899, !!8                                                                           ;L2158<1971<332<1966
 73339|     ;; x2 = i64 %3318
 73340|     ;; other = i64 %3318
 73341|  %3319 = load i64, ptr %3219, , !!76899, !!8                                                                           ;L2158<1971<332<1966
 73342|     ;; y2 = i64 %3319
 73343|     ;; other = i64 %3319
 73344|  %3320 = icmp ult i64 %3315, %3318                                                                                     ;L3147<7<2158<1971<332<1966
 73345|  %3321 = sub nuw i64 %3318, %3315                                                                                      ;L3147<7<2158<1971<332<1966
 73346|  %3322 = sub nuw i64 %3315, %3318                                                                                      ;L3147<7<2158<1971<332<1966
 73347|  %3323 = select i1 %3320, i64 %3321, i64 %3322                                                                         ;L3147<7<2158<1971<332<1966
 73348|     ;; dx = i64 %3323
 73349|  %3324 = icmp ult i64 %3317, %3319                                                                                     ;L3147<8<2158<1971<332<1966
 73350|  %3325 = sub nuw i64 %3319, %3317                                                                                      ;L3147<8<2158<1971<332<1966
 73351|  %3326 = sub nuw i64 %3317, %3319                                                                                      ;L3147<8<2158<1971<332<1966
 73352|  %3327 = select i1 %3324, i64 %3325, i64 %3326                                                                         ;L3147<8<2158<1971<332<1966
 73353|     ;; dy = i64 %3327
 73354|  %3328 = mul i64 %3323, %3323                                                                                          ;L9<2158<1971<332<1966
 73355|  %3329 = mul i64 %3327, %3327                                                                                          ;L9<2158<1971<332<1966
 73356|  %3330 = add i64 %3329, %3328                                                                                          ;L9<2158<1971<332<1966
 73357|  %3331 = mul i64 %3262, %3262                                                                                          ;L1971<332<1966
 73358|  %3332 = icmp ugt i64 %3330, %3331                                                                                     ;L1971<332<1966
 73359|  br i1 %3332, label %3333, label %3190                                                                                 ;L1971<332<1966
 73360| 
 73361| 3333: ; preds = %3313
 73362|     ;; other = ptr %86
 73363|  %3334 = load i64, ptr %1459, , !!76899, !!8                                                                           ;L2158<1972<332<1966
 73364|     ;; x2 = i64 %3334
 73365|     ;; other = i64 %3334
 73366|  %3335 = load i64, ptr %1460, , !!76899, !!8                                                                           ;L2158<1972<332<1966
 73367|     ;; y2 = i64 %3335
 73368|     ;; other = i64 %3335
 73369|  %3336 = icmp ult i64 %3315, %3334                                                                                     ;L3147<7<2158<1972<332<1966
 73370|  %3337 = sub nuw i64 %3334, %3315                                                                                      ;L3147<7<2158<1972<332<1966
 73371|  %3338 = sub nuw i64 %3315, %3334                                                                                      ;L3147<7<2158<1972<332<1966
 73372|  %3339 = select i1 %3336, i64 %3337, i64 %3338                                                                         ;L3147<7<2158<1972<332<1966
 73373|     ;; dx = i64 %3339
 73374|  %3340 = icmp ult i64 %3317, %3335                                                                                     ;L3147<8<2158<1972<332<1966
 73375|  %3341 = sub nuw i64 %3335, %3317                                                                                      ;L3147<8<2158<1972<332<1966
 73376|  %3342 = sub nuw i64 %3317, %3335                                                                                      ;L3147<8<2158<1972<332<1966
 73377|  %3343 = select i1 %3340, i64 %3341, i64 %3342                                                                         ;L3147<8<2158<1972<332<1966
 73378|     ;; dy = i64 %3343
 73379|  %3344 = mul i64 %3339, %3339                                                                                          ;L9<2158<1972<332<1966
 73380|  %3345 = mul i64 %3343, %3343                                                                                          ;L9<2158<1972<332<1966
 73381|  %3346 = add i64 %3345, %3344                                                                                          ;L9<2158<1972<332<1966
 73382|  %3347 = mul i64 %3300, %3300                                                                                          ;L1972<332<1966
 73383|  %3348 = icmp ugt i64 %3346, %3347                                                                                     ;L1972<332<1966
 73384|  br i1 %3348, label %3349, label %3190                                                                                 ;L332<1966
 73385| 
 73386| 3349: ; preds = %3333, %3306
 73387|     ;; ptr = ptr %3222
 73388|     ;; self = ptr %3222
 73389|     ;; end_or_len = ptr %3209
 73392|  %3350 = icmp eq ptr %3222, %3209                                                                                      ;L1714<180<331<1966
 73393|  br i1 %3350, label %3351, label %3220                                                                                 ;L180<331<1966
 73394| 
 73395| 3351: ; preds = %3349, %3204, %3196
 73397|  br label %3432                                                                                                        ;L1
 73398| 
 73399| 3352: ; preds = %3190
 73400|  %3353 = mul i64 %3192, %2919                                                                                          ;L26<1983
 73401|  %3354 = load i32, ptr %2920, , !!8                                                                                    ;L1511<1983
 73402|     ;; mult = i32 %3354
 73403|  %3355 = icmp eq i32 %3354, 0                                                                                          ;L1512<1983
 73404|  br i1 %3355, label %3356, label %3358                                                                                 ;L1512<1983
 73405| 
 73406| 3356: ; preds = %3352
 73407|  %3357 = load i64, ptr %2921, , !!8                                                                                    ;L1513<1983
 73408|  br label %3364                                                                                                        ;L1512<1983
 73409| 
 73410| 3358: ; preds = %3352
 73411|  %3359 = sext i32 %3354 to i64                                                                                         ;L1511<1983
 73412|     ;; mult = i64 %3359
 73413|  %3360 = load i64, ptr %2921, , !!8                                                                                    ;L1515<1983
 73414|  %3361 = add nsw i64 %3359, 100                                                                                        ;L1515<1983
 73415|  %3362 = mul i64 %3360, %3361                                                                                          ;L1515<1983
 73416|  %3363 = udiv i64 %3362, 100                                                                                           ;L1515<1983
 73417|  br label %3364                                                                                                        ;L1512<1983
 73418| 
 73419| 3364: ; preds = %3358, %3356
 73420|  %3365 = phi i64 [ %3357, %3356 ], [ %3363, %3358 ]                                                                    ;L0<1983
 73421|  %3366 = load ptr, ptr %3141, , !!8, !!8                                                                               ;L1983
 73422|     ;; self = ptr %3366
 73423|  %3367 = gep %3366, i64 1136                                                                                           ;L1511<1983
 73424|  %3368 = load i32, ptr %3367, , !!8                                                                                    ;L1511<1983
 73425|     ;; mult = i32 %3368
 73426|  %3369 = icmp eq i32 %3368, 0                                                                                          ;L1512<1983
 73427|  br i1 %3369, label %3370, label %3373                                                                                 ;L1512<1983
 73428| 
 73429| 3370: ; preds = %3364
 73430|  %3371 = gep %3366, i64 1664                                                                                           ;L1513<1983
 73431|  %3372 = load i64, ptr %3371, , !!8                                                                                    ;L1513<1983
 73432|  br label %3380                                                                                                        ;L1512<1983
 73433| 
 73434| 3373: ; preds = %3364
 73435|  %3374 = sext i32 %3368 to i64                                                                                         ;L1511<1983
 73436|     ;; mult = i64 %3374
 73437|  %3375 = gep %3366, i64 1664                                                                                           ;L1515<1983
 73438|  %3376 = load i64, ptr %3375, , !!8                                                                                    ;L1515<1983
 73439|  %3377 = add nsw i64 %3374, 100                                                                                        ;L1515<1983
 73440|  %3378 = mul i64 %3376, %3377                                                                                          ;L1515<1983
 73441|  %3379 = udiv i64 %3378, 100                                                                                           ;L1515<1983
 73442|  br label %3380                                                                                                        ;L1512<1983
 73443| 
 73444| 3380: ; preds = %3373, %3370
 73445|  %3381 = phi i64 [ %3372, %3370 ], [ %3379, %3373 ]                                                                    ;L0<1983
 73447|     ;; self = ptr %3366
 73448|  %3382 = gep %3366, i64 1632                                                                                           ;L2158<1984
 73449|  %3383 = load i64, ptr %3382, , !!8                                                                                    ;L2158<1984
 73450|     ;; x1 = i64 %3383
 73451|     ;; self = i64 %3383
 73452|  %3384 = gep %3366, i64 1640                                                                                           ;L2158<1984
 73453|  %3385 = load i64, ptr %3384, , !!8                                                                                    ;L2158<1984
 73454|     ;; y1 = i64 %3385
 73455|     ;; self = i64 %3385
 73456|  %3386 = load i64, ptr %1459, , !!8                                                                                    ;L2158<1984
 73457|     ;; x2 = i64 %3386
 73458|     ;; other = i64 %3386
 73459|  %3387 = load i64, ptr %1460, , !!8                                                                                    ;L2158<1984
 73460|     ;; y2 = i64 %3387
 73461|     ;; other = i64 %3387
 73462|  %3388 = icmp ult i64 %3383, %3386                                                                                     ;L3147<7<2158<1984
 73463|  %3389 = sub nuw i64 %3386, %3383                                                                                      ;L3147<7<2158<1984
 73464|  %3390 = sub nuw i64 %3383, %3386                                                                                      ;L3147<7<2158<1984
 73465|  %3391 = select i1 %3388, i64 %3389, i64 %3390                                                                         ;L3147<7<2158<1984
 73466|     ;; dx = i64 %3391
 73467|  %3392 = icmp ult i64 %3385, %3387                                                                                     ;L3147<8<2158<1984
 73468|  %3393 = sub nuw i64 %3387, %3385                                                                                      ;L3147<8<2158<1984
 73469|  %3394 = sub nuw i64 %3385, %3387                                                                                      ;L3147<8<2158<1984
 73470|  %3395 = select i1 %3392, i64 %3393, i64 %3394                                                                         ;L3147<8<2158<1984
 73471|     ;; dy = i64 %3395
 73472|  %3396 = mul i64 %3391, %3391                                                                                          ;L9<2158<1984
 73473|  %3397 = mul i64 %3395, %3395                                                                                          ;L9<2158<1984
 73474|  %3398 = add i64 %3397, %3396                                                                                          ;L9<2158<1984
 73475|     ;; dist_sq = i64 %3398
 73476|     ;; max_tick = i64 %97
 73477|  %3399 = add i64 %3191, %1461                                                                                          ;L26<1983
 73478|  %3400 = add i64 %3399, %3353                                                                                          ;L26<1983
 73479|  %3401 = add i64 %3400, %3193                                                                                          ;L1983
 73480|  %3402 = add i64 %3401, %3195                                                                                          ;L1983
 73481|  %3403 = add i64 %3402, %3365                                                                                          ;L1983
 73482|  %3404 = add i64 %3403, %3381                                                                                          ;L1993
 73483|     ;; max_dist = i64 %3404
 73484|  %3405 = mul i64 %3404, %3404                                                                                          ;L1994
 73485|  %3406 = icmp ugt i64 %3398, %3405                                                                                     ;L1994
 73486|  br i1 %3406, label %3432, label %3407                                                                                 ;L1994
 73487| 
 73488| 3407: ; preds = %3380
 73489|  %3408 = load ptr, ptr %2888, , !!8, !!8                                                                               ;L1998
 73490|  %3409 = load ptr, ptr %2890, , !!8, !!8                                                                               ;L1998
 73491|  %3410 = gep %3409, i64 192                                                                                            ;L1998
 73492|  %3411 = load ptr, ptr %3410, , !!8                                                                                    ;L1998
 73493|  %3412 = invoke zeroext i1 %3411(ptr %3408, ptr %3151, ptr %3152, ptr %86)
 73494|  to label %3413 unwind label %178                                                                                      ;L1998
 73495| 
 73496| 3413: ; preds = %3407
 73497|  br i1 %3412, label %3414, label %3432                                                                                 ;L1998
 73498| 
 73499| 3414: ; preds = %3413
 73502|  %3415 = load ptr, ptr %3141, , !!8, !!8                                                                               ;L2002
 73503|  %3416 = gep %3415, i64 1472                                                                                           ;L2002
 73504|  %3417 = load i64, ptr %3416, , !!8                                                                                    ;L2002
 73505|  invoke void @ai::small_action4castNtB5_14SmallActionUlt3new(ptr sret([24 x i8]) %12, ptr %3, i64 %3417)
 73506|  to label %3418 unwind label %178                                                                                      ;L2002
 73507| 
 73508| 3418: ; preds = %3414
 73509|  call void @llvm.memcpy.p0.p0.i64(ptr %13, ptr %12, i64 24, i1 false)                                                  ;L2002
 73510|  store i8 18, ptr %2943,                                                                                               ;L2002
 73513|     ;; self = ptr %65
 73514|     ;; self = ptr %65
 73515|     ;; value = ptr %13
 73516|     ;; src = ptr %13
 73517|     ;; additional = i64 1
 73518|     ;; needed_extra_cap = i64 1
 73519|     ;; needed_extra_cap = i64 1
 73520|     ;; strategy = i8 1
 73521|  %3419 = load i64, ptr %126, , !!77103, !!8                                                                            ;L1428<2002
 73522|     ;; self = ptr %65
 73523|  %3420 = load i64, ptr %125, , !!77103, !!8                                                                            ;L149<1428<2002
 73524|  %3421 = icmp eq i64 %3419, %3420                                                                                      ;L1428<2002
 73525|  br i1 %3421, label %3422, label %3427                                                                                 ;L1428<2002
 73526| 
 73527| 3422: ; preds = %3418
 73528|     ;; self = ptr %65
 73529|     ;; self = ptr %65
 73530|     ;; self = ptr %65
 73531|     ;; used_cap = i64 %3419
 73532|     ;; used_cap = i64 %3419
 73533|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %65, i64 %3419, i64 1, i1 zeroext true)
 73534|  to label %3423 unwind label %3425, !!77103                                                                            ;L619<430<738<1429<2002
 73535| 
 73536| 3423: ; preds = %3422
 73537|  %3424 = load i64, ptr %126, , !!77103                                                                                 ;L1432<2002
 73538|  br label %3427                                                                                                        ;L619<430<738<1429<2002
 73539| 
 73540| 3425: ; preds = %3422
 73541|  %3426 = cleanuppad within none []
 73542|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %13) #30 [ "funclet"(token %3426) ], !!77088 ;L1436<2002
 73543|  cleanupret from %3426 unwind label %178
 73544| 
 73545| 3427: ; preds = %3423, %3418
 73546|  %3428 = phi i64 [ %3424, %3423 ], [ %3419, %3418 ]                                                                    ;L1432<2002
 73547|     ;; self = ptr %65
 73548|  %3429 = load ptr, ptr %65, , !!77103, !!8, !!8                                                                        ;L138<1432<2002
 73549|     ;; self = ptr %3429
 73550|     ;; count = i64 %3428
 73551|  %3430 = gepS %3429, i64 %3428                                                                                         ;L961<1432<2002
 73552|     ;; end = ptr %3430
 73553|     ;; dst = ptr %3430
 73554|  call void @llvm.memcpy.p0.p0.i64(ptr %3430, ptr %13, i64 184, i1 false), !!77088                                      ;L1933<1433<2002
 73555|  %3431 = add i64 %3428, 1                                                                                              ;L1434<2002
 73556|  store i64 %3431, ptr %126, , !!77103                                                                                  ;L1434<2002
 73558|  br label %3432                                                                                                        ;L1938
 73559| 
 73560| 3432: ; preds = %3427, %3413, %3380, %3351, %3157, %3147
 73561|  br label %3140                                                                                                        ;L1714<180<1938
 73562| 
 73563| 3433: ; preds = %3140
 73564|  %3434 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %2915, ptr %86, ptr %86)
 73565|  to label %3435 unwind label %178                                                                                      ;L2005
 73566| 
 73567| 3435: ; preds = %3433
 73568|  br i1 %3434, label %3436, label %2850                                                                                 ;L2005
 73569| 
 73570| 3436: ; preds = %3435
 73571|  %3437 = load ptr, ptr %2888, , !!8, !!8                                                                               ;L2005
 73572|  %3438 = load ptr, ptr %2890, , !!8, !!8                                                                               ;L2005
 73573|  %3439 = load ptr, ptr %82, , !!8, !!8                                                                                 ;L2005
 73574|  %3440 = load ptr, ptr %2632, , !!8, !!8                                                                               ;L2005
 73575|  %3441 = gep %3438, i64 200                                                                                            ;L2005
 73576|  %3442 = load ptr, ptr %3441, , !!8                                                                                    ;L2005
 73577|  %3443 = invoke zeroext i1 %3442(ptr %3437, ptr %3439, ptr %3440, ptr %86, ptr %86)
 73578|  to label %3444 unwind label %178                                                                                      ;L2005
 73579| 
 73580| 3444: ; preds = %3436
 73581|  br i1 %3443, label %3445, label %2850                                                                                 ;L2005
 73582| 
 73583| 3445: ; preds = %3444
 73584|     ;; skip_self_buff = i8 0
 73586|  %3446 = load i64, ptr %73, , !!8                                                                                      ;L2009
 73587|  invoke void @ai::fight_check18effect_buff_target(ptr sret([288 x i8]) %11, i64 %3446, ptr %145, ptr %102, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %86, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54)
 73588|  to label %3447 unwind label %178                                                                                      ;L2009
 73589| 
 73590| 3447: ; preds = %3445
 73593|  %3448 = load ptr, ptr %145, , !!8, !!8                                                                                ;L441<2127<2445<2010
 73594|  %3449 = load ptr, ptr %2922, , !!8, !!8                                                                               ;L441<2127<2445<2010
 73595|  %3450 = gep %3449, i64 16                                                                                             ;L2445<2010
 73596|  %3451 = load i64, ptr %3450,                                                                                          ;L2445<2010
 73597|  %3452 = add nsw i64 %3451, -1                                                                                         ;L2445<2010
 73598|  %3453 = and i64 %3452, -16                                                                                            ;L2445<2010
 73599|  %3454 = gep %3448, i64 %3453                                                                                          ;L2445<2010
 73600|  %3455 = gep %3454, i64 16                                                                                             ;L2445<2010
 73601|  %3456 = gep %3449, i64 144                                                                                            ;L2010
 73602|  %3457 = load ptr, ptr %3456, , !!8                                                                                    ;L2010
 73603|  %3458 = invoke zeroext i1 %3457(ptr %3455)
 73604|  to label %3459 unwind label %178                                                                                      ;L2010
 73605| 
 73606| 3459: ; preds = %3447
 73607|     ;; is_etc_buff = i1 %3458
 73608|     ;; self = ptr %11
 73609|  %3460 = gep %11, i64 72                                                                                               ;L633<2014
 73610|  %3461 = load i32, ptr %3460, , !!8                                                                                    ;L633<2014
 73611|  %3462 = icmp ne i32 %3461, -1                                                                                         ;L633<2014
 73612|  %3463 = or i1 %3458, %3462                                                                                            ;L2014
 73613|  br i1 %3463, label %3484, label %3464                                                                                 ;L2014
 73614| 
 73615| 3464: ; preds = %3459
 73618|  %3465 = load ptr, ptr %145, , !!8, !!8                                                                                ;L441<2127<2445<2015
 73619|  %3466 = load ptr, ptr %2922, , !!8, !!8                                                                               ;L441<2127<2445<2015
 73620|  %3467 = gep %3466, i64 16                                                                                             ;L2445<2015
 73621|  %3468 = load i64, ptr %3467,                                                                                          ;L2445<2015
 73622|  %3469 = add nsw i64 %3468, -1                                                                                         ;L2445<2015
 73623|  %3470 = and i64 %3469, -16                                                                                            ;L2445<2015
 73624|  %3471 = gep %3465, i64 %3470                                                                                          ;L2445<2015
 73625|  %3472 = gep %3471, i64 16                                                                                             ;L2445<2015
 73626|  %3473 = gep %3466, i64 176                                                                                            ;L2015
 73627|  %3474 = load ptr, ptr %3473, , !!8                                                                                    ;L2015
 73628|  %3475 = invoke i64 %3474(ptr %3472, ptr %102, ptr %86)
 73629|  to label %3476 unwind label %178                                                                                      ;L2015
 73630| 
 73631| 3476: ; preds = %3464
 73632|  %3477 = icmp eq i64 %3475, 0                                                                                          ;L2015
 73633|     ;; gate_applies = i1 %3477
 73634|  br i1 %3477, label %3478, label %3484                                                                                 ;L2016
 73635| 
 73636| 3478: ; preds = %3549, %3476
 73638|  %3479 = load ptr, ptr %2888, , !!8, !!8                                                                               ;L2032
 73639|  %3480 = load ptr, ptr %2890, , !!8, !!8                                                                               ;L2032
 73640|  %3481 = gep %3480, i64 192                                                                                            ;L2032
 73641|  %3482 = load ptr, ptr %3481, , !!8                                                                                    ;L2032
 73642|  %3483 = invoke zeroext i1 %3482(ptr %3479, ptr %3439, ptr %3440, ptr %86)
 73643|  to label %3572 unwind label %178                                                                                      ;L2032
 73644| 
 73645| 3484: ; preds = %3476, %3459
 73646|     ;; engage_extra = i64 %2940
 73647|     ;; self = ptr %86
 73648|  br i1 %130, label %3571, label %3485                                                                                  ;L742<2019
 73649| 
 73650| 3485: ; preds = %3484
 73651|     ;; target_atk = ptr %127
 73652|     ;; self = ptr %70
 73653|     ;; self = ptr %70
 73654|  %3486 = load ptr, ptr %70, , !!8, !!8                                                                                 ;L138<2073<2020
 73655|     ;; p = ptr %3486
 73656|  %3487 = load i64, ptr %2912, , !!8                                                                                    ;L2075<2020
 73657|     ;; len = i64 %3487
 73658|     ;; count = i64 %3487
 73659|     ;; self[0..+8] = ptr %3486
 73660|     ;; slice[0..+8] = ptr %3486
 73661|     ;; self[8..+8] = i64 %3487
 73662|     ;; slice[8..+8] = i64 %3487
 73663|     ;; ptr = ptr %3486
 73664|     ;; self = ptr %3486
 73665|  %3488 = shl nuw nsw i64 %3487, 5                                                                                      ;L961<100<1042<2020
 73666|  %3489 = gep %3486, i64 %3488                                                                                          ;L961<100<1042<2020
 73667|     ;; f[0..+8] = ptr %127
 73668|     ;; f[8..+8] = ptr %86
 73669|     ;; f[16..+8] = ptr undef
 73670|     ;; self = ptr undef
 73671|     ;; self = ptr undef
 73672|     ;; count = i64 1
 73673|     ;; ptr = ptr %3486
 73674|     ;; self = ptr %3486
 73675|     ;; end_or_len = ptr %3489
 73678|  %3490 = icmp eq i64 %3487, 0                                                                                          ;L1714<180<331<2020
 73679|  br i1 %3490, label %3571, label %3491                                                                                 ;L180<331<2020
 73680| 
 73681| 3491: ; preds = %3569, %3485
 73682|  %3492 = phi ptr [ %3493, %3569 ], [ %3486, %3485 ]
 73683|     ;; ptr = ptr %3492
 73684|  %3493 = gep %3492, i64 32                                                                                             ;L656<185<331<2020
 73685|     ;; x = ptr %3492
 73686|  %3494 = gep %3492, i64 24                                                                                             ;L332<2020
 73687|  %3495 = load ptr, ptr %3494, , !!77191, !!8, !!8                                                                      ;L332<2020
 73693|     ;; self = ptr %127
 73694|     ;; caster = ptr %86
 73695|  %3496 = load i64, ptr %2941, , !!77218, !!8                                                                           ;L26<2021<332<2020
 73696|  %3497 = load i64, ptr %2942, , !!77218, !!8                                                                           ;L26<2021<332<2020
 73697|  %3498 = load i64, ptr %135, , !!77218, !!8                                                                            ;L26<2021<332<2020
 73698|  %3499 = add i64 %3498, -1                                                                                             ;L26<2021<332<2020
 73699|  %3500 = mul i64 %3499, %3497                                                                                          ;L26<2021<332<2020
 73700|  %3501 = load i64, ptr %2918, , !!77218, !!8                                                                           ;L26<2021<332<2020
 73701|     ;; self = ptr %3495
 73702|     ;; self = ptr %3495
 73703|     ;; self = ptr %3495
 73704|  %3502 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %127, ptr %86, ptr %3495)
 73705|  to label %3503 unwind label %178                                                                                      ;L2021<332<2020
 73706| 
 73707| 3503: ; preds = %3491
 73708|     ;; self = ptr %86
 73709|  %3504 = load i32, ptr %2920, , !!77218, !!8                                                                           ;L1511<2021<332<2020
 73710|     ;; mult = i32 %3504
 73711|  %3505 = icmp eq i32 %3504, 0                                                                                          ;L1512<2021<332<2020
 73712|  br i1 %3505, label %3506, label %3508                                                                                 ;L1512<2021<332<2020
 73713| 
 73714| 3506: ; preds = %3503
 73715|  %3507 = load i64, ptr %2921, , !!77218, !!8                                                                           ;L1513<2021<332<2020
 73716|  br label %3514                                                                                                        ;L1512<2021<332<2020
 73717| 
 73718| 3508: ; preds = %3503
 73719|  %3509 = sext i32 %3504 to i64                                                                                         ;L1511<2021<332<2020
 73720|     ;; mult = i64 %3509
 73721|  %3510 = load i64, ptr %2921, , !!77218, !!8                                                                           ;L1515<2021<332<2020
 73722|  %3511 = add nsw i64 %3509, 100                                                                                        ;L1515<2021<332<2020
 73723|  %3512 = mul i64 %3510, %3511                                                                                          ;L1515<2021<332<2020
 73724|  %3513 = udiv i64 %3512, 100                                                                                           ;L1515<2021<332<2020
 73725|  br label %3514                                                                                                        ;L1512<2021<332<2020
 73726| 
 73727| 3514: ; preds = %3508, %3506
 73728|  %3515 = phi i64 [ %3507, %3506 ], [ %3513, %3508 ]                                                                    ;L0<2021<332<2020
 73729|  %3516 = gep %3495, i64 1136                                                                                           ;L1511<2021<332<2020
 73730|  %3517 = load i32, ptr %3516, , !!77218, !!8                                                                           ;L1511<2021<332<2020
 73731|     ;; mult = i32 %3517
 73732|  %3518 = icmp eq i32 %3517, 0                                                                                          ;L1512<2021<332<2020
 73733|  br i1 %3518, label %3519, label %3522                                                                                 ;L1512<2021<332<2020
 73734| 
 73735| 3519: ; preds = %3514
 73736|  %3520 = gep %3495, i64 1664                                                                                           ;L1513<2021<332<2020
 73737|  %3521 = load i64, ptr %3520, , !!77218, !!8                                                                           ;L1513<2021<332<2020
 73738|  br label %3529                                                                                                        ;L1512<2021<332<2020
 73739| 
 73740| 3522: ; preds = %3514
 73741|  %3523 = sext i32 %3517 to i64                                                                                         ;L1511<2021<332<2020
 73742|     ;; mult = i64 %3523
 73743|  %3524 = gep %3495, i64 1664                                                                                           ;L1515<2021<332<2020
 73744|  %3525 = load i64, ptr %3524, , !!77218, !!8                                                                           ;L1515<2021<332<2020
 73745|  %3526 = add nsw i64 %3523, 100                                                                                        ;L1515<2021<332<2020
 73746|  %3527 = mul i64 %3525, %3526                                                                                          ;L1515<2021<332<2020
 73747|  %3528 = udiv i64 %3527, 100                                                                                           ;L1515<2021<332<2020
 73748|  br label %3529                                                                                                        ;L1512<2021<332<2020
 73749| 
 73750| 3529: ; preds = %3522, %3519
 73751|  %3530 = phi i64 [ %3521, %3519 ], [ %3528, %3522 ]                                                                    ;L0<2021<332<2020
 73752|  %3531 = add i64 %3496, %2940                                                                                          ;L26<2021<332<2020
 73753|  %3532 = add i64 %3531, %3501                                                                                          ;L26<2021<332<2020
 73754|  %3533 = add i64 %3532, %3500                                                                                          ;L2021<332<2020
 73755|  %3534 = add i64 %3533, %3502                                                                                          ;L2021<332<2020
 73756|  %3535 = add i64 %3534, %3515                                                                                          ;L2021<332<2020
 73757|  %3536 = add i64 %3535, %3530                                                                                          ;L2021<332<2020
 73758|     ;; attack_range = i64 %3536
 73759|     ;; entity = ptr %86
 73760|     ;; self = ptr %86
 73761|  %3537 = load i64, ptr %86, , !!77218, !!8                                                                             ;L1136<1482<2022<332<2020
 73762|  %3538 = trunc nuw i64 %3537 to i1                                                                                     ;L1136<1482<2022<332<2020
 73763|  br i1 %3538, label %3549, label %3539                                                                                 ;L1136<1482<2022<332<2020
 73764| 
 73765| 3539: ; preds = %3529
 73766|     ;; team = ptr %86
 73767|  %3540 = load i64, ptr %1458, , !!77218, !!8                                                                           ;L1137<1482<2022<332<2020
 73768|     ;; team = i64 %3540
 73769|  %3541 = icmp ult i64 %3540, 2                                                                                         ;L1483<2022<332<2020
 73770|  br i1 %3541, label %3542, label %3547                                                                                 ;L1483<2022<332<2020
 73771| 
 73772| 3542: ; preds = %3539
 73774|  %3543 = gep %3495, i64 56                                                                                             ;L122<1483<2022<332<2020
 73775|  %3544 = gepS %3543, i64 %3540                                                                                         ;L122<1483<2022<332<2020
 73776|  %3545 = load i64, ptr %3544, , !!77218, !!8                                                                           ;L122<1483<2022<332<2020
 73777|  %3546 = icmp eq i64 %3545, 0                                                                                          ;L122<1483<2022<332<2020
 73778|  br i1 %3546, label %3549, label %3569                                                                                 ;L2022<332<2020
 73779| 
 73780| 3547: ; preds = %3539
 73781|  invoke void @core::panicking18panic_bounds_check(i64 %3540, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 73782|  to label %3548 unwind label %178                                                                                      ;L1483<2022<332<2020
 73783| 
 73784| 3548: ; preds = %3547
 73785|  unreachable                                                                                                           ;L1483<2022<332<2020
 73786| 
 73787| 3549: ; preds = %3542, %3529
 73788|     ;; other = ptr %86
 73789|  %3550 = gep %3495, i64 1632                                                                                           ;L2158<2022<332<2020
 73790|  %3551 = load i64, ptr %3550, , !!77218, !!8                                                                           ;L2158<2022<332<2020
 73791|     ;; x1 = i64 %3551
 73792|     ;; self = i64 %3551
 73793|  %3552 = gep %3495, i64 1640                                                                                           ;L2158<2022<332<2020
 73794|  %3553 = load i64, ptr %3552, , !!77218, !!8                                                                           ;L2158<2022<332<2020
 73795|     ;; y1 = i64 %3553
 73796|     ;; self = i64 %3553
 73797|  %3554 = load i64, ptr %1459, , !!77218, !!8                                                                           ;L2158<2022<332<2020
 73798|     ;; x2 = i64 %3554
 73799|     ;; other = i64 %3554
 73800|  %3555 = load i64, ptr %1460, , !!77218, !!8                                                                           ;L2158<2022<332<2020
 73801|     ;; y2 = i64 %3555
 73802|     ;; other = i64 %3555
 73803|  %3556 = icmp ult i64 %3551, %3554                                                                                     ;L3147<7<2158<2022<332<2020
 73804|  %3557 = sub nuw i64 %3554, %3551                                                                                      ;L3147<7<2158<2022<332<2020
 73805|  %3558 = sub nuw i64 %3551, %3554                                                                                      ;L3147<7<2158<2022<332<2020
 73806|  %3559 = select i1 %3556, i64 %3557, i64 %3558                                                                         ;L3147<7<2158<2022<332<2020
 73807|     ;; dx = i64 %3559
 73808|  %3560 = icmp ult i64 %3553, %3555                                                                                     ;L3147<8<2158<2022<332<2020
 73809|  %3561 = sub nuw i64 %3555, %3553                                                                                      ;L3147<8<2158<2022<332<2020
 73810|  %3562 = sub nuw i64 %3553, %3555                                                                                      ;L3147<8<2158<2022<332<2020
 73811|  %3563 = select i1 %3560, i64 %3561, i64 %3562                                                                         ;L3147<8<2158<2022<332<2020
 73812|     ;; dy = i64 %3563
 73813|  %3564 = mul i64 %3559, %3559                                                                                          ;L9<2158<2022<332<2020
 73814|  %3565 = mul i64 %3563, %3563                                                                                          ;L9<2158<2022<332<2020
 73815|  %3566 = add i64 %3565, %3564                                                                                          ;L9<2158<2022<332<2020
 73816|  %3567 = mul i64 %3536, %3536                                                                                          ;L2022<332<2020
 73817|  %3568 = icmp ugt i64 %3566, %3567                                                                                     ;L2022<332<2020
 73818|  br i1 %3568, label %3569, label %3478                                                                                 ;L332<2020
 73819| 
 73820| 3569: ; preds = %3549, %3542
 73821|     ;; ptr = ptr %3493
 73822|     ;; self = ptr %3493
 73823|     ;; end_or_len = ptr %3489
 73826|  %3570 = icmp eq ptr %3493, %3489                                                                                      ;L1714<180<331<2020
 73827|  br i1 %3570, label %3571, label %3491                                                                                 ;L180<331<2020
 73828| 
 73829| 3571: ; preds = %3569, %3485, %3484
 73830|     ;; skip_self_buff = i8 1
 73832|  br label %2850                                                                                                        ;L2032
 73833| 
 73834| 3572: ; preds = %3478
 73835|  br i1 %3483, label %3573, label %2850                                                                                 ;L2032
 73836| 
 73837| 3573: ; preds = %3572
 73840|  %3574 = gep %86, i64 1472                                                                                             ;L2033
 73841|  %3575 = load i64, ptr %3574, , !!8                                                                                    ;L2033
 73842|  invoke void @ai::small_action4castNtB5_14SmallActionUlt3new(ptr sret([24 x i8]) %9, ptr %3, i64 %3575)
 73843|  to label %3576 unwind label %178                                                                                      ;L2033
 73844| 
 73845| 3576: ; preds = %3573
 73846|  call void @llvm.memcpy.p0.p0.i64(ptr %10, ptr %9, i64 24, i1 false)                                                   ;L2033
 73847|  %3577 = gep %10, i64 177                                                                                              ;L2033
 73848|  store i8 18, ptr %3577,                                                                                               ;L2033
 73850|  invoke fastcc void @ai::small_action15SmallActionPlayE4pushBX_(ptr %65, ptr %10)
 73851|  to label %3578 unwind label %178                                                                                      ;L2033
 73852| 
 73853| 3578: ; preds = %3576
 73855|  br label %2850                                                                                                        ;L2032
 73856| 
 73857| 3579: ; preds = %2882, %2873, %2869
 73860|  invoke void @gc::simulation4game10blackboard11SmallActionRNtNtB13_6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %70)
 73861|  to label %3598 unwind label %3580                                                                                     ;L825<2039
 73862| 
 73863| 3580: ; preds = %3579
 73864|  %3581 = cleanuppad within none []
 73868|     ;; self = ptr %70
 73870|     ;; self = ptr %70
 73871|     ;; self = ptr %70
 73872|     ;; elem_size = i64 32
 73873|     ;; align = i64 8
 73874|  %3582 = gep %70, i64 16                                                                                               ;L159<703<714<825<825<2039
 73875|  %3583 = load i64, ptr %3582, , !!8                                                                                    ;L159<703<714<825<825<2039
 73876|  %3584 = icmp eq i64 %3583, 0                                                                                          ;L159<703<714<825<825<2039
 73877|  br i1 %3584, label %3597, label %3585                                                                                 ;L159<703<714<825<825<2039
 73878| 
 73879| 3585: ; preds = %3580
 73880|     ;; layout[0..+8] = i64 8
 73881|     ;; layout[8..+8] = i64 %3583
 73882|  %3586 = gep %70, i64 8                                                                                                ;L704<714<825<825<2039
 73883|  %3587 = load ptr, ptr %3586, , !!8, !!8                                                                               ;L704<714<825<825<2039
 73884|  %3588 = load ptr, ptr %70, , !!8, !!8                                                                                 ;L704<714<825<825<2039
 73885|  %3589 = gep %3587, i64 16                                                                                             ;L704<714<825<825<2039
 73886|  %3590 = load ptr, ptr %3589, , !!77328, !!8, !!8                                                                      ;L704<714<825<825<2039
 73889|     ;; ptr = ptr %3588
 73890|     ;; ptr = ptr %3588
 73891|     ;; layout[0..+8] = i64 8
 73892|     ;; layout[8..+8] = i64 %3583
 73893|     ;; footer = ptr %3590
 73894|     ;; footer = ptr %3590
 73895|     ;; self = ptr %3590
 73896|  %3591 = gep %3590, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<2039
 73897|  %3592 = load ptr, ptr %3591, , !!77328, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<2039
 73900|     ;; self = ptr %3588
 73901|  %3593 = icmp eq ptr %3592, %3588                                                                                      ;L1714<1700<1707<704<714<825<825<2039
 73902|  br i1 %3593, label %3594, label %3597                                                                                 ;L1707<704<714<825<825<2039
 73903| 
 73904| 3594: ; preds = %3585
 73905|  %3595 = shl i64 %3583, 5                                                                                              ;L166<703<714<825<825<2039
 73906|     ;; layout[8..+8] = i64 %3595
 73907|     ;; layout[8..+8] = i64 %3595
 73908|     ;; count = i64 %3595
 73909|  %3596 = gep %3588, i64 %3595                                                                                          ;L961<1708<704<714<825<825<2039
 73910|     ;; ptr = ptr %3596
 73911|     ;; val = ptr %3596
 73912|     ;; val = ptr %3596
 73913|     ;; self = ptr %3590
 73914|     ;; self = ptr %3590
 73915|  store ptr %3596, ptr %3591, , !!77328                                                                                 ;L931<513<437<1709<704<714<825<825<2039
 73916|  br label %3597                                                                                                        ;L1707<704<714<825<825<2039
 73917| 
 73918| 3597: ; preds = %3594, %3585, %3580
 73919|  cleanupret from %3581 unwind label %116
 73920| 
 73921| 3598: ; preds = %3579
 73925|     ;; self = ptr %70
 73927|     ;; self = ptr %70
 73928|     ;; self = ptr %70
 73929|     ;; elem_size = i64 32
 73930|     ;; align = i64 8
 73931|  %3599 = gep %70, i64 16                                                                                               ;L159<703<714<825<825<2039
 73932|  %3600 = load i64, ptr %3599, , !!8                                                                                    ;L159<703<714<825<825<2039
 73933|  %3601 = icmp eq i64 %3600, 0                                                                                          ;L159<703<714<825<825<2039
 73934|  br i1 %3601, label %3614, label %3602                                                                                 ;L159<703<714<825<825<2039
 73935| 
 73936| 3602: ; preds = %3598
 73937|     ;; layout[0..+8] = i64 8
 73938|     ;; layout[8..+8] = i64 %3600
 73939|  %3603 = gep %70, i64 8                                                                                                ;L704<714<825<825<2039
 73940|  %3604 = load ptr, ptr %3603, , !!8, !!8                                                                               ;L704<714<825<825<2039
 73941|  %3605 = load ptr, ptr %70, , !!8, !!8                                                                                 ;L704<714<825<825<2039
 73942|  %3606 = gep %3604, i64 16                                                                                             ;L704<714<825<825<2039
 73943|  %3607 = load ptr, ptr %3606, , !!77381, !!8, !!8                                                                      ;L704<714<825<825<2039
 73946|     ;; ptr = ptr %3605
 73947|     ;; ptr = ptr %3605
 73948|     ;; layout[0..+8] = i64 8
 73949|     ;; layout[8..+8] = i64 %3600
 73950|     ;; footer = ptr %3607
 73951|     ;; footer = ptr %3607
 73952|     ;; self = ptr %3607
 73953|  %3608 = gep %3607, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<2039
 73954|  %3609 = load ptr, ptr %3608, , !!77381, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<2039
 73957|     ;; self = ptr %3605
 73958|  %3610 = icmp eq ptr %3609, %3605                                                                                      ;L1714<1700<1707<704<714<825<825<2039
 73959|  br i1 %3610, label %3611, label %3614                                                                                 ;L1707<704<714<825<825<2039
 73960| 
 73961| 3611: ; preds = %3602
 73962|  %3612 = shl i64 %3600, 5                                                                                              ;L166<703<714<825<825<2039
 73963|     ;; layout[8..+8] = i64 %3612
 73964|     ;; layout[8..+8] = i64 %3612
 73965|     ;; count = i64 %3612
 73966|  %3613 = gep %3605, i64 %3612                                                                                          ;L961<1708<704<714<825<825<2039
 73967|     ;; ptr = ptr %3613
 73968|     ;; val = ptr %3613
 73969|     ;; val = ptr %3613
 73970|     ;; self = ptr %3607
 73971|     ;; self = ptr %3607
 73972|  store ptr %3613, ptr %3608, , !!77381                                                                                 ;L931<513<437<1709<704<714<825<825<2039
 73973|  br label %3614                                                                                                        ;L1707<704<714<825<825<2039
 73974| 
 73975| 3614: ; preds = %3611, %3602, %3598
 73978|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %72)
 73979|  to label %3633 unwind label %3615                                                                                     ;L825<2039
 73980| 
 73981| 3615: ; preds = %3614
 73982|  %3616 = cleanuppad within none []
 73986|     ;; self = ptr %72
 73988|     ;; self = ptr %72
 73989|     ;; self = ptr %72
 73990|     ;; elem_size = i64 8
 73991|     ;; align = i64 8
 73992|  %3617 = gep %72, i64 16                                                                                               ;L159<703<714<825<825<2039
 73993|  %3618 = load i64, ptr %3617, , !!8                                                                                    ;L159<703<714<825<825<2039
 73994|  %3619 = icmp eq i64 %3618, 0                                                                                          ;L159<703<714<825<825<2039
 73995|  br i1 %3619, label %3632, label %3620                                                                                 ;L159<703<714<825<825<2039
 73996| 
 73997| 3620: ; preds = %3615
 73998|     ;; layout[0..+8] = i64 8
 73999|     ;; layout[8..+8] = i64 %3618
 74000|  %3621 = gep %72, i64 8                                                                                                ;L704<714<825<825<2039
 74001|  %3622 = load ptr, ptr %3621, , !!8, !!8                                                                               ;L704<714<825<825<2039
 74002|  %3623 = load ptr, ptr %72, , !!8, !!8                                                                                 ;L704<714<825<825<2039
 74003|  %3624 = gep %3622, i64 16                                                                                             ;L704<714<825<825<2039
 74004|  %3625 = load ptr, ptr %3624, , !!77439, !!8, !!8                                                                      ;L704<714<825<825<2039
 74007|     ;; ptr = ptr %3623
 74008|     ;; ptr = ptr %3623
 74009|     ;; layout[0..+8] = i64 8
 74010|     ;; layout[8..+8] = i64 %3618
 74011|     ;; footer = ptr %3625
 74012|     ;; footer = ptr %3625
 74013|     ;; self = ptr %3625
 74014|  %3626 = gep %3625, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<2039
 74015|  %3627 = load ptr, ptr %3626, , !!77439, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<2039
 74018|     ;; self = ptr %3623
 74019|  %3628 = icmp eq ptr %3627, %3623                                                                                      ;L1714<1700<1707<704<714<825<825<2039
 74020|  br i1 %3628, label %3629, label %3632                                                                                 ;L1707<704<714<825<825<2039
 74021| 
 74022| 3629: ; preds = %3620
 74023|  %3630 = shl i64 %3618, 3                                                                                              ;L166<703<714<825<825<2039
 74024|     ;; layout[8..+8] = i64 %3630
 74025|     ;; layout[8..+8] = i64 %3630
 74026|     ;; count = i64 %3630
 74027|  %3631 = gep %3623, i64 %3630                                                                                          ;L961<1708<704<714<825<825<2039
 74028|     ;; ptr = ptr %3631
 74029|     ;; val = ptr %3631
 74030|     ;; val = ptr %3631
 74031|     ;; self = ptr %3625
 74032|     ;; self = ptr %3625
 74033|  store ptr %3631, ptr %3626, , !!77439                                                                                 ;L931<513<437<1709<704<714<825<825<2039
 74034|  br label %3632                                                                                                        ;L1707<704<714<825<825<2039
 74035| 
 74036| 3632: ; preds = %3629, %3620, %3615
 74037|  cleanupret from %3616 unwind to caller                                                                                ;L825<2039
 74038| 
 74039| 3633: ; preds = %3614
 74043|     ;; self = ptr %72
 74045|     ;; self = ptr %72
 74046|     ;; self = ptr %72
 74047|     ;; elem_size = i64 8
 74048|     ;; align = i64 8
 74049|  %3634 = gep %72, i64 16                                                                                               ;L159<703<714<825<825<2039
 74050|  %3635 = load i64, ptr %3634, , !!8                                                                                    ;L159<703<714<825<825<2039
 74051|  %3636 = icmp eq i64 %3635, 0                                                                                          ;L159<703<714<825<825<2039
 74052|  br i1 %3636, label %3649, label %3637                                                                                 ;L159<703<714<825<825<2039
 74053| 
 74054| 3637: ; preds = %3633
 74055|     ;; layout[0..+8] = i64 8
 74056|     ;; layout[8..+8] = i64 %3635
 74057|  %3638 = gep %72, i64 8                                                                                                ;L704<714<825<825<2039
 74058|  %3639 = load ptr, ptr %3638, , !!8, !!8                                                                               ;L704<714<825<825<2039
 74059|  %3640 = load ptr, ptr %72, , !!8, !!8                                                                                 ;L704<714<825<825<2039
 74060|  %3641 = gep %3639, i64 16                                                                                             ;L704<714<825<825<2039
 74061|  %3642 = load ptr, ptr %3641, , !!77492, !!8, !!8                                                                      ;L704<714<825<825<2039
 74064|     ;; ptr = ptr %3640
 74065|     ;; ptr = ptr %3640
 74066|     ;; layout[0..+8] = i64 8
 74067|     ;; layout[8..+8] = i64 %3635
 74068|     ;; footer = ptr %3642
 74069|     ;; footer = ptr %3642
 74070|     ;; self = ptr %3642
 74071|  %3643 = gep %3642, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<2039
 74072|  %3644 = load ptr, ptr %3643, , !!77492, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<2039
 74075|     ;; self = ptr %3640
 74076|  %3645 = icmp eq ptr %3644, %3640                                                                                      ;L1714<1700<1707<704<714<825<825<2039
 74077|  br i1 %3645, label %3646, label %3649                                                                                 ;L1707<704<714<825<825<2039
 74078| 
 74079| 3646: ; preds = %3637
 74080|  %3647 = shl i64 %3635, 3                                                                                              ;L166<703<714<825<825<2039
 74081|     ;; layout[8..+8] = i64 %3647
 74082|     ;; layout[8..+8] = i64 %3647
 74083|     ;; count = i64 %3647
 74084|  %3648 = gep %3640, i64 %3647                                                                                          ;L961<1708<704<714<825<825<2039
 74085|     ;; ptr = ptr %3648
 74086|     ;; val = ptr %3648
 74087|     ;; val = ptr %3648
 74088|     ;; self = ptr %3642
 74089|     ;; self = ptr %3642
 74090|  store ptr %3648, ptr %3643, , !!77492                                                                                 ;L931<513<437<1709<704<714<825<825<2039
 74091|  br label %3649                                                                                                        ;L1707<704<714<825<825<2039
 74092| 
 74093| 3649: ; preds = %3646, %3637, %3633
 74095|  ret void                                                                                                              ;L2039
 74096| }
