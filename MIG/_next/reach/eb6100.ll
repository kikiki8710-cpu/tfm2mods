 23700| define void @ai::fight_check13battle_action(ptr sret([32 x i8]) %0, i64 %1, ptr readnone %2, ptr %3, ptr %4, i64 %5) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 23701|  %7 = alloca [16 x i8],
 23702|  %8 = alloca [288 x i8],
 23703|  %9 = alloca [288 x i8],
 23704|  %10 = alloca [24 x i8],
 23705|  %11 = alloca [184 x i8],
 23706|  %12 = alloca [24 x i8],
 23707|  %13 = alloca [184 x i8],
 23709|  %14 = alloca [24 x i8],
 23710|  %15 = alloca [184 x i8],
 23712|  %16 = alloca [24 x i8],
 23713|  %17 = alloca [184 x i8],
 23714|  %18 = alloca [24 x i8],
 23715|  %19 = alloca [184 x i8],
 23717|  %20 = alloca [24 x i8],
 23718|  %21 = alloca [184 x i8],
 23720|  %22 = alloca [24 x i8],
 23721|  %23 = alloca [184 x i8],
 23723|  %24 = alloca [32 x i8],
 23724|  %25 = alloca [56 x i8],
 23725|  %26 = alloca [32 x i8],
 23726|  %27 = alloca [24 x i8],
 23727|  %28 = alloca [32 x i8],
 23728|  %29 = alloca [24 x i8],
 23731|     ;; player = ptr %3
 23732|     ;; data = ptr %4
 23734|     ;; _t = ptr %29
 23735|     ;; near_allies = ptr %28
 23736|     ;; near_enemies_with_action = ptr %26
 23737|     ;; candidates = ptr %24
 23738|     ;; phase = i64 71
 23739|     ;; order = i8 0
 23740|     ;; len = i64 5
 23741|     ;; count = i64 5
 23742|     ;; len = i64 5
 23743|     ;; count = i64 5
 23745|     ;; count = i64 1
 23746|     ;; count = i64 1
 23747|     ;; count = i64 1
 23748|     ;; count = i64 1
 23749|     ;; count = i64 1
 23751|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 23752|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 23753|     ;; order = i8 0
 23754|  %30 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                     ;L3904<741<176<621
 23755|  %31 = icmp eq i8 %30, 0                                                                                               ;L176<621
 23756|  br i1 %31, label %37, label %32                                                                                       ;L176<621
 23757| 
 23758| 32: ; preds = %6
 23759|  %33 = tail call { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()                                       ;L179<621
 23760|  %34 = extractvalue { i64, i32 } %33, 0                                                                                ;L179<621
 23761|  %35 = extractvalue { i64, i32 } %33, 1                                                                                ;L179<621
 23762|  store i64 71, ptr %29,                                                                                                ;L179<621
 23763|  %36 = gep %29, i64 8                                                                                                  ;L179<621
 23764|  store i64 %34, ptr %36,                                                                                               ;L179<621
 23765|  br label %37                                                                                                          ;L180<621
 23766| 
 23767| 37: ; preds = %32, %6
 23768|  %38 = phi i32 [ %35, %32 ], [ -1, %6 ]                                                                                ;L0<621
 23769|  %39 = gep %29, i64 16                                                                                                 ;L0<621
 23770|  store i32 %38, ptr %39,                                                                                               ;L0<621
 23771|  %40 = gep %3, i64 2352                                                                                                ;L622
 23772|  %41 = load i64, ptr %40, , !!8                                                                                        ;L622
 23773|     ;; team = i64 %41
 23774|  %42 = icmp ult i64 %41, 2                                                                                             ;L622
 23775|  br i1 %42, label %47, label %43                                                                                       ;L622
 23776| 
 23777| 43: ; preds = %37
 23778|  invoke void @core::panicking18panic_bounds_check(i64 %41, i64 2, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.168) #30
 23779|  to label %46 unwind label %44                                                                                         ;L622
 23780| 
 23781| 44: ; preds = %845, %844, %842, %77, %64, %57, %43
 23782|  %45 = cleanuppad within none []
 23783|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %29) #32 [ "funclet"(token %45) ] ;L786
 23784|  cleanupret from %45 unwind to caller                                                                                  ;L620
 23785| 
 23786| 46: ; preds = %562, %269, %141, %64, %43
 23787|  unreachable
 23788| 
 23789| 47: ; preds = %37
 23790|     ;; self = ptr %3
 23791|  %48 = gep %3, i64 2496                                                                                                ;L581<622
 23792|  %49 = load i32, ptr %48, , !!8                                                                                        ;L581<622
 23793|  %50 = zext nneg i32 %49 to i64                                                                                        ;L581<622
 23794|  %51 = load ptr, ptr %4, , !!8, !!8                                                                                    ;L622
 23795|     ;; self = ptr %51
 23796|  %52 = gep %51, i64 480                                                                                                ;L622
 23797|  %53 = getelementptr [5 x ptr], ptr %52, i64 %41                                                                       ;L622
 23798|  %54 = getelementptr ptr, ptr %53, i64 %50                                                                             ;L622
 23799|  %55 = load ptr, ptr %54, , !!8                                                                                        ;L622
 23800|     ;; self = ptr %55
 23801|  %56 = icmp eq ptr %55, null                                                                                           ;L1011<622
 23802|  br i1 %56, label %64, label %57                                                                                       ;L1011<622
 23803| 
 23804| 57: ; preds = %47
 23805|     ;; champ = ptr %55
 23806|     ;; self = ptr %55
 23807|     ;; entity = ptr %55
 23808|     ;; caster = ptr %55
 23809|     ;; self = ptr %55
 23810|     ;; other = ptr %55
 23811|     ;; entity = ptr %55
 23812|     ;; caster = ptr %55
 23813|     ;; self = ptr %55
 23814|     ;; other = ptr %55
 23815|     ;; caster = ptr %55
 23816|     ;; self = ptr %55
 23817|     ;; other = ptr %55
 23818|     ;; entity = ptr %55
 23819|     ;; caster = ptr %55
 23820|     ;; self = ptr %55
 23821|     ;; other = ptr %55
 23822|     ;; caster = ptr %55
 23823|     ;; self = ptr %55
 23824|     ;; other = ptr %55
 23827|     ;; self[0..+8] = ptr %53
 23828|     ;; slice[0..+8] = ptr %53
 23829|     ;; self[8..+8] = i64 5
 23830|     ;; slice[8..+8] = i64 5
 23831|     ;; ptr = ptr %53
 23832|     ;; self = ptr %53
 23833|  %58 = gep %53, i64 40                                                                                                 ;L961<100<1042<1905<625
 23834|     ;; self[0..+8] = ptr %53
 23835|     ;; self[8..+8] = ptr %58
 23836|     ;; predicate = ptr %55
 23837|  store ptr %53, ptr %27,                                                                                               ;L28<957<625
 23838|  %59 = gep %27, i64 8                                                                                                  ;L28<957<625
 23839|  store ptr %58, ptr %59,                                                                                               ;L28<957<625
 23840|  %60 = gep %27, i64 16                                                                                                 ;L28<957<625
 23841|  store ptr %55, ptr %60,                                                                                               ;L28<957<625
 23842|  %61 = gep %4, i64 8                                                                                                   ;L625
 23843|  %62 = load ptr, ptr %61, , !!8, !!8                                                                                   ;L625
 23844|  %63 = load ptr, ptr %62, , !!8, !!8                                                                                   ;L625
 23845|     ;; bump = ptr %63
 23846|  invoke void @core::iter8adapters6filter6FilterINtNtB29_10filter_map9FilterMapINtNtNtB2d_5slice4iter4IterINtNtB2d_6option6OptionBU_EENCNvMs3_BZ_NtBZ_21AbstractGameWithCache14iter_champions0ENCNvNtCshdEBA0ozCnw_7game_ai11fight_check13battle_action0EEB5n_(ptr sret([32 x i8]) %28, ptr %27, ptr %63)
 23847|  to label %65 unwind label %44                                                                                         ;L624
 23848| 
 23849| 64: ; preds = %47
 23850|  invoke void @core::option13unwrap_failed(ptr @anon.e5f9a016f1220e5cee86475419aee1e5.169) #30
 23851|  to label %46 unwind label %44                                                                                         ;L1013<622
 23852| 
 23853| 65: ; preds = %57
 23857|  %66 = sub nuw nsw i64 1, %41                                                                                          ;L628
 23858|  %67 = getelementptr [5 x ptr], ptr %52, i64 %66                                                                       ;L628
 23859|     ;; self[0..+8] = ptr %67
 23860|     ;; slice[0..+8] = ptr %67
 23861|     ;; self[8..+8] = i64 5
 23862|     ;; slice[8..+8] = i64 5
 23863|     ;; ptr = ptr %67
 23864|     ;; self = ptr %67
 23865|  %68 = gep %67, i64 40                                                                                                 ;L961<100<1042<628
 23866|  %69 = gep %4, i64 16                                                                                                  ;L629
 23867|  %70 = load ptr, ptr %69, , !!8, !!8                                                                                   ;L629
 23868|     ;; self[0..+8] = ptr %67
 23869|     ;; self[8..+8] = ptr %68
 23870|     ;; self[16..+8] = i64 0
 23871|     ;; self[24..+8] = ptr %70
 23872|     ;; self[32..+8] = ptr %3
 23873|     ;; self[40..+8] = ptr %4
 23874|     ;; self[48..+8] = ptr %3
 23875|  store ptr %67, ptr %25,                                                                                               ;L69<836<631
 23876|  %71 = gep %25, i64 8                                                                                                  ;L69<836<631
 23877|  store ptr %68, ptr %71,                                                                                               ;L69<836<631
 23878|  %72 = gep %25, i64 16                                                                                                 ;L69<836<631
 23879|  store i64 0, ptr %72,                                                                                                 ;L69<836<631
 23880|  %73 = gep %25, i64 24                                                                                                 ;L69<836<631
 23881|  store ptr %70, ptr %73,                                                                                               ;L69<836<631
 23882|  %74 = gep %25, i64 32                                                                                                 ;L69<836<631
 23883|  store ptr %3, ptr %74,                                                                                                ;L69<836<631
 23884|  %75 = gep %25, i64 40                                                                                                 ;L69<836<631
 23885|  store ptr %4, ptr %75,                                                                                                ;L69<836<631
 23886|  %76 = gep %25, i64 48                                                                                                 ;L69<836<631
 23887|  store ptr %3, ptr %76,                                                                                                ;L69<836<631
 23888|  invoke void @core::iter8adapters3map3MapINtNtB2P_6filter6FilterIB2L_INtNtB2P_9enumerate9EnumerateINtNtNtB2T_5slice4iter4IterINtNtB2T_6option6OptionB27_EEENCNvNtCshdEBA0ozCnw_7game_ai11fight_check13battle_actions_0ENCB5q_s0_0ENCB5q_s1_0EEB5u_(ptr sret([32 x i8]) %26, ptr %25, ptr %63)
 23889|  to label %79 unwind label %77                                                                                         ;L627
 23890| 
 23891| 77: ; preds = %527, %526, %524, %105, %65
 23892|  %78 = cleanuppad within none []
 23893|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEECshdEBA0ozCnw_7game_ai(ptr %28) #32 [ "funclet"(token %78) ] ;L786
 23894|  cleanupret from %78 unwind label %44                                                                                  ;L786
 23895| 
 23896| 79: ; preds = %65
 23899|  store ptr inttoptr (i64 8 to ptr), ptr %24,                                                                           ;L547<634
 23900|  %80 = gep %24, i64 8                                                                                                  ;L547<634
 23901|  store ptr %63, ptr %80,                                                                                               ;L547<634
 23902|  %81 = gep %24, i64 16                                                                                                 ;L547<634
 23903|  %82 = gep %24, i64 24                                                                                                 ;L547<634
 23904|     ;; self = ptr %55
 23905|  %83 = gep %55, i64 1216                                                                                               ;L742<636
 23906|  call void @llvm.memset.p0.i64(ptr %81, i8 0, i64 16, i1 false)                                                        ;L547<634
 23907|  %84 = load i32, ptr %83, , !!8                                                                                        ;L742<636
 23908|  %85 = icmp eq i32 %84, -1                                                                                             ;L742<636
 23909|  %86 = gep %55, i64 1168                                                                                               ;L742<636
 23911|     ;; self = ptr %55
 23912|  %87 = gep %55, i64 1272                                                                                               ;L742<637
 23913|  %88 = load i32, ptr %87, , !!8                                                                                        ;L742<637
 23914|  %89 = icmp eq i32 %88, -1                                                                                             ;L742<637
 23915|  %90 = gep %55, i64 1224                                                                                               ;L742<637
 23916|  %91 = select i1 %89, ptr null, ptr %90                                                                                ;L742<637
 23917|     ;; skill = ptr %91
 23918|  %92 = gep %55, i64 1480                                                                                               ;L1693<638
 23919|  %93 = load i64, ptr %92, , !!8                                                                                        ;L1693<638
 23920|  %94 = icmp ugt i64 %93, 2                                                                                             ;L1693<638
 23921|  %95 = gep %55, i64 1280                                                                                               ;L1693<638
 23922|  %96 = select i1 %94, ptr %95, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.58                                           ;L1693<638
 23923|     ;; self = ptr %96
 23924|  %97 = gep %96, i64 48                                                                                                 ;L742<638
 23925|  %98 = load i32, ptr %97, , !!8                                                                                        ;L742<638
 23926|  %99 = icmp eq i32 %98, -1                                                                                             ;L742<638
 23927|  %100 = select i1 %99, ptr null, ptr %96                                                                               ;L742<638
 23928|     ;; skill2 = ptr %100
 23929|  %101 = gep %55, i64 1600                                                                                              ;L640
 23930|  %102 = load i64, ptr %101, , !!8                                                                                      ;L640
 23931|     ;; move_speed = i64 %102
 23932|     ;; self = i64 %102
 23933|     ;; self = i64 %102
 23934|     ;; self = i64 %102
 23935|  br i1 %85, label %108, label %103                                                                                     ;L642
 23936| 
 23937| 103: ; preds = %79
 23938|  %104 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_attack(ptr %55)
 23939|  to label %107 unwind label %105                                                                                       ;L642
 23940| 
 23941| 105: ; preds = %834, %823, %815, %811, %803, %790, %787, %779, %769, %708, %703, %677, %669, %659, %657, %654, %637, %576, %569, %563, %562, %520, %513, %502, %497, %494, %486, %476, %415, %410, %384, %376, %366, %364, %361, %344, %283, %276, %270, %269, %231, %223, %213, %200, %142, %141, %103
 23942|  %106 = cleanuppad within none []
 23943|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %24) #32 [ "funclet"(token %106) ] ;L786
 23944|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecTNtNtNtNtCs97f5S1uJLkH_9game_core10simulation4game10blackboard11SmallActionRNtNtB1w_6entity6EntityEEECshdEBA0ozCnw_7game_ai(ptr %26) #32 [ "funclet"(token %106) ] ;L786
 23945|  cleanupret from %106 unwind label %77                                                                                 ;L786
 23946| 
 23947| 107: ; preds = %103
 23948|  br i1 %104, label %109, label %108                                                                                    ;L642
 23949| 
 23950| 108: ; preds = %124, %107, %79
 23951|  br i1 %89, label %234, label %231                                                                                     ;L667
 23952| 
 23953| 109: ; preds = %107
 23954|     ;; self = ptr %26
 23955|     ;; self = ptr %26
 23956|     ;; self = ptr %26
 23957|  %110 = load ptr, ptr %26, , !!8, !!8                                                                                  ;L138<2073<2136<643
 23958|     ;; p = ptr %110
 23959|  %111 = gep %26, i64 24                                                                                                ;L2075<2136<643
 23960|  %112 = load i64, ptr %111, , !!8                                                                                      ;L2075<2136<643
 23961|     ;; len = i64 %112
 23962|     ;; count = i64 %112
 23963|     ;; self[0..+8] = ptr %110
 23964|     ;; slice[0..+8] = ptr %110
 23965|     ;; self[8..+8] = i64 %112
 23966|     ;; slice[8..+8] = i64 %112
 23967|     ;; ptr = ptr %110
 23968|     ;; self = ptr %110
 23969|  %113 = gepS }, ptr %110, i64 %112                                                                                     ;L961<100<1042<2136<643
 23970|     ;; iter[0..+8] = ptr %110
 23971|     ;; iter[8..+8] = ptr %113
 23972|  %114 = gep %55, i64 8
 23973|  %115 = gep %55, i64 1184
 23974|  %116 = gep %55, i64 1192
 23975|  %117 = gep %55, i64 1080
 23976|  %118 = add i64 %93, -1
 23977|  %119 = gep %55, i64 1136
 23978|  %120 = gep %55, i64 1664
 23979|  %121 = gep %55, i64 1632
 23980|  %122 = gep %55, i64 1640
 23981|  %123 = gep %23, i64 177
 23982|  br label %124                                                                                                         ;L643
 23983| 
 23984| 124: ; preds = %230, %109
 23985|  %125 = phi ptr [ %110, %109 ], [ %128, %230 ]                                                                         ;L643
 23986|     ;; iter[0..+8] = ptr %125
 23987|     ;; self = ptr undef
 23988|     ;; ptr = ptr %125
 23989|     ;; self = ptr %125
 23990|     ;; end_or_len = ptr %113
 23993|  %126 = icmp eq ptr %125, %113                                                                                         ;L1714<180<643
 23994|  br i1 %126, label %108, label %127                                                                                    ;L180<643
 23995| 
 23996| 127: ; preds = %124
 23997|  %128 = gep %125, i64 32                                                                                               ;L656<185<643
 23998|     ;; iter[0..+8] = ptr %128
 23999|     ;; a = ptr %125
 24000|     ;; e = ptr %125
 24001|  %129 = gep %125, i64 24                                                                                               ;L644
 24002|  %130 = load ptr, ptr %129, , !!8, !!8                                                                                 ;L644
 24003|     ;; self = ptr %130
 24004|     ;; self = ptr %130
 24005|     ;; self = ptr %130
 24006|     ;; self = ptr %55
 24007|  %131 = load i64, ptr %55, , !!8                                                                                       ;L1136<1482<644
 24008|  %132 = trunc nuw i64 %131 to i1                                                                                       ;L1136<1482<644
 24009|  br i1 %132, label %142, label %133                                                                                    ;L1136<1482<644
 24010| 
 24011| 133: ; preds = %127
 24012|     ;; team = ptr %55
 24013|  %134 = load i64, ptr %114, , !!8                                                                                      ;L1137<1482<644
 24014|     ;; team = i64 %134
 24015|  %135 = icmp ult i64 %134, 2                                                                                           ;L1483<644
 24016|  br i1 %135, label %136, label %141                                                                                    ;L1483<644
 24017| 
 24018| 136: ; preds = %133
 24020|  %137 = gep %130, i64 56                                                                                               ;L122<1483<644
 24021|  %138 = gepS %137, i64 %134                                                                                            ;L122<1483<644
 24022|  %139 = load i64, ptr %138, , !!8                                                                                      ;L122<1483<644
 24023|  %140 = icmp eq i64 %139, 0                                                                                            ;L122<1483<644
 24024|  br i1 %140, label %142, label %230                                                                                    ;L644
 24025| 
 24026| 141: ; preds = %133
 24027|  invoke void @core::panicking18panic_bounds_check(i64 %134, i64 2, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.62) #30
 24028|  to label %46 unwind label %105                                                                                        ;L1483<644
 24029| 
 24030| 142: ; preds = %136, %127
 24032|  %143 = load i64, ptr %115, , !!8                                                                                      ;L26<648
 24033|  %144 = load i64, ptr %116, , !!8                                                                                      ;L26<648
 24034|  %145 = load i64, ptr %117, , !!8                                                                                      ;L26<648
 24035|  %146 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %86, ptr %55, ptr %130)
 24036|  to label %147 unwind label %105                                                                                       ;L648
 24037| 
 24038| 147: ; preds = %142
 24039|  %148 = mul i64 %144, %118                                                                                             ;L26<648
 24040|  %149 = add i64 %148, %143                                                                                             ;L26<648
 24041|  %150 = add i64 %149, %145                                                                                             ;L26<648
 24042|  %151 = add i64 %150, %146                                                                                             ;L648
 24043|  %152 = load i32, ptr %119, , !!8                                                                                      ;L1511<648
 24044|     ;; mult = i32 %152
 24045|  %153 = icmp eq i32 %152, 0                                                                                            ;L1512<648
 24046|  br i1 %153, label %154, label %156                                                                                    ;L1512<648
 24047| 
 24048| 154: ; preds = %147
 24049|  %155 = load i64, ptr %120, , !!8                                                                                      ;L1513<648
 24050|  br label %162                                                                                                         ;L1512<648
 24051| 
 24052| 156: ; preds = %147
 24053|  %157 = sext i32 %152 to i64                                                                                           ;L1511<648
 24054|     ;; mult = i64 %157
 24055|  %158 = load i64, ptr %120, , !!8                                                                                      ;L1515<648
 24056|  %159 = add nsw i64 %157, 100                                                                                          ;L1515<648
 24057|  %160 = mul i64 %158, %159                                                                                             ;L1515<648
 24058|  %161 = udiv i64 %160, 100                                                                                             ;L1515<648
 24059|  br label %162                                                                                                         ;L1512<648
 24060| 
 24061| 162: ; preds = %156, %154
 24062|  %163 = phi i64 [ %155, %154 ], [ %161, %156 ]                                                                         ;L0<648
 24063|  %164 = add i64 %151, %163                                                                                             ;L648
 24064|  %165 = gep %130, i64 1136                                                                                             ;L1511<648
 24065|  %166 = load i32, ptr %165, , !!8                                                                                      ;L1511<648
 24066|     ;; mult = i32 %166
 24067|  %167 = icmp eq i32 %166, 0                                                                                            ;L1512<648
 24068|  br i1 %167, label %168, label %171                                                                                    ;L1512<648
 24069| 
 24070| 168: ; preds = %162
 24071|  %169 = gep %130, i64 1664                                                                                             ;L1513<648
 24072|  %170 = load i64, ptr %169, , !!8                                                                                      ;L1513<648
 24073|  br label %178                                                                                                         ;L1512<648
 24074| 
 24075| 171: ; preds = %162
 24076|  %172 = sext i32 %166 to i64                                                                                           ;L1511<648
 24077|     ;; mult = i64 %172
 24078|  %173 = gep %130, i64 1664                                                                                             ;L1515<648
 24079|  %174 = load i64, ptr %173, , !!8                                                                                      ;L1515<648
 24080|  %175 = add nsw i64 %172, 100                                                                                          ;L1515<648
 24081|  %176 = mul i64 %174, %175                                                                                             ;L1515<648
 24082|  %177 = udiv i64 %176, 100                                                                                             ;L1515<648
 24083|  br label %178                                                                                                         ;L1512<648
 24084| 
 24085| 178: ; preds = %171, %168
 24086|  %179 = phi i64 [ %170, %168 ], [ %177, %171 ]                                                                         ;L0<648
 24087|  %180 = add i64 %164, %179                                                                                             ;L648
 24088|     ;; range = i64 %180
 24089|  %181 = gep %130, i64 1632                                                                                             ;L2158<649
 24090|  %182 = load i64, ptr %181, , !!8                                                                                      ;L2158<649
 24091|     ;; x1 = i64 %182
 24092|     ;; self = i64 %182
 24093|  %183 = gep %130, i64 1640                                                                                             ;L2158<649
 24094|  %184 = load i64, ptr %183, , !!8                                                                                      ;L2158<649
 24095|     ;; y1 = i64 %184
 24096|     ;; self = i64 %184
 24097|  %185 = load i64, ptr %121, , !!8                                                                                      ;L2158<649
 24098|     ;; x2 = i64 %185
 24099|     ;; other = i64 %185
 24100|  %186 = load i64, ptr %122, , !!8                                                                                      ;L2158<649
 24101|     ;; y2 = i64 %186
 24102|     ;; other = i64 %186
 24103|  %187 = icmp ult i64 %182, %185                                                                                        ;L3147<7<2158<649
 24104|  %188 = sub nuw i64 %185, %182                                                                                         ;L3147<7<2158<649
 24105|  %189 = sub nuw i64 %182, %185                                                                                         ;L3147<7<2158<649
 24106|  %190 = select i1 %187, i64 %188, i64 %189                                                                             ;L3147<7<2158<649
 24107|     ;; dx = i64 %190
 24108|  %191 = icmp ult i64 %184, %186                                                                                        ;L3147<8<2158<649
 24109|  %192 = sub nuw i64 %186, %184                                                                                         ;L3147<8<2158<649
 24110|  %193 = sub nuw i64 %184, %186                                                                                         ;L3147<8<2158<649
 24111|  %194 = select i1 %191, i64 %192, i64 %193                                                                             ;L3147<8<2158<649
 24112|     ;; dy = i64 %194
 24113|  %195 = mul i64 %190, %190                                                                                             ;L9<2158<649
 24114|  %196 = mul i64 %194, %194                                                                                             ;L9<2158<649
 24115|  %197 = add i64 %196, %195                                                                                             ;L9<2158<649
 24116|     ;; dist_sq = i64 %197
 24117|  %198 = load i64, ptr %125, , !!8                                                                                      ;L652
 24120|     ;; __self_discr = i64 %198
 24121|     ;; __arg1_discr = i64 0
 24122|  %199 = icmp eq i64 %198, 0                                                                                            ;L81<652
 24123|  br i1 %199, label %200, label %207                                                                                    ;L652
 24124| 
 24125| 200: ; preds = %178
 24126|  %201 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10block_move(ptr %130)
 24127|  to label %202 unwind label %105                                                                                       ;L652
 24128| 
 24129| 202: ; preds = %200
 24130|  br i1 %201, label %207, label %203                                                                                    ;L652
 24131| 
 24132| 203: ; preds = %202
 24133|  %204 = gep %130, i64 1600                                                                                             ;L653
 24134|  %205 = load i64, ptr %204, , !!8                                                                                      ;L653
 24135|     ;; rhs = i64 %205
 24136|  %206 = call i64 @llvm.usub.sat.i64(i64 %102, i64 %205)                                                                ;L2472<653
 24137|     ;; move_speed = i64 %206
 24138|  br label %207                                                                                                         ;L652
 24139| 
 24140| 207: ; preds = %203, %202, %178
 24141|  %208 = phi i64 [ %206, %203 ], [ %102, %202 ], [ %102, %178 ]                                                         ;L0
 24142|     ;; move_speed = i64 %208
 24143|  %209 = mul i64 %208, 30                                                                                               ;L658
 24144|  %210 = add i64 %180, %209                                                                                             ;L658
 24145|     ;; max_dist = i64 %210
 24146|  %211 = mul i64 %210, %210                                                                                             ;L659
 24147|  %212 = icmp ugt i64 %197, %211                                                                                        ;L659
 24148|  br i1 %212, label %230, label %213                                                                                    ;L659
 24149| 
 24150| 213: ; preds = %207
 24153|  %214 = gep %130, i64 1472                                                                                             ;L663
 24154|  %215 = load i64, ptr %214, , !!8                                                                                      ;L663
 24155|  invoke void @ai::small_action4castNtB2_17SmallActionAttack3new(ptr sret([24 x i8]) %22, ptr %4, i64 %215)
 24156|  to label %216 unwind label %105                                                                                       ;L663
 24157| 
 24158| 216: ; preds = %213
 24159|  call void @llvm.memcpy.p0.p0.i64(ptr %23, ptr %22, i64 24, i1 false)                                                  ;L663
 24160|  store i8 15, ptr %123,                                                                                                ;L663
 24162|     ;; self = ptr %24
 24163|     ;; self = ptr %24
 24164|     ;; value = ptr %23
 24165|     ;; src = ptr %23
 24166|     ;; additional = i64 1
 24167|     ;; needed_extra_cap = i64 1
 24168|     ;; needed_extra_cap = i64 1
 24169|     ;; strategy = i8 1
 24170|  %217 = load i64, ptr %82, , !!37833, !!8                                                                              ;L1428<663
 24171|     ;; self = ptr %24
 24172|  %218 = load i64, ptr %81, , !!37833, !!8                                                                              ;L149<1428<663
 24173|  %219 = icmp eq i64 %217, %218                                                                                         ;L1428<663
 24174|  br i1 %219, label %220, label %225                                                                                    ;L1428<663
 24175| 
 24176| 220: ; preds = %216
 24177|     ;; self = ptr %24
 24178|     ;; self = ptr %24
 24179|     ;; self = ptr %24
 24180|     ;; used_cap = i64 %217
 24181|     ;; used_cap = i64 %217
 24182|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %24, i64 %217, i64 1, i1 zeroext true)
 24183|  to label %221 unwind label %223, !!37833                                                                              ;L619<430<738<1429<663
 24184| 
 24185| 221: ; preds = %220
 24186|  %222 = load i64, ptr %82, , !!37833                                                                                   ;L1432<663
 24187|  br label %225                                                                                                         ;L619<430<738<1429<663
 24188| 
 24189| 223: ; preds = %220
 24190|  %224 = cleanuppad within none []
 24191|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %23) #32 [ "funclet"(token %224) ] ;L1436<663
 24192|  cleanupret from %224 unwind label %105
 24193| 
 24194| 225: ; preds = %221, %216
 24195|  %226 = phi i64 [ %222, %221 ], [ %217, %216 ]                                                                         ;L1434<663
 24196|     ;; self = ptr %24
 24197|  %227 = load ptr, ptr %24, , !!37833, !!8, !!8                                                                         ;L138<1432<663
 24198|     ;; self = ptr %227
 24199|     ;; count = i64 %226
 24200|  %228 = gepS %227, i64 %226                                                                                            ;L961<1432<663
 24201|     ;; end = ptr %228
 24202|     ;; dst = ptr %228
 24203|  call void @llvm.memcpy.p0.p0.i64(ptr %228, ptr %23, i64 184, i1 false)                                                ;L1933<1433<663
 24204|  %229 = add i64 %226, 1                                                                                                ;L1434<663
 24205|  store i64 %229, ptr %82, , !!37833                                                                                    ;L1434<663
 24207|  br label %230                                                                                                         ;L643
 24208| 
 24209| 230: ; preds = %225, %207, %136
 24210|  br label %124                                                                                                         ;L1714<180<643
 24211| 
 24212| 231: ; preds = %108
 24213|  %232 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity9can_skill(ptr %55)
 24214|  to label %233 unwind label %105                                                                                       ;L667
 24215| 
 24216| 233: ; preds = %231
 24217|  br i1 %232, label %235, label %234                                                                                    ;L667
 24218| 
 24219| 234: ; preds = %515, %501, %496, %233, %108
 24220|  br i1 %99, label %523, label %520                                                                                     ;L726
 24221| 
 24222| 235: ; preds = %233
 24223|     ;; self = ptr %26
 24224|     ;; self = ptr %26
 24225|     ;; self = ptr %26
 24226|  %236 = load ptr, ptr %26, , !!8, !!8                                                                                  ;L138<2073<2136<668
 24227|     ;; p = ptr %236
 24228|  %237 = gep %26, i64 24                                                                                                ;L2075<2136<668
 24229|  %238 = load i64, ptr %237, , !!8                                                                                      ;L2075<2136<668
 24230|     ;; len = i64 %238
 24231|     ;; count = i64 %238
 24232|     ;; self[0..+8] = ptr %236
 24233|     ;; slice[0..+8] = ptr %236
 24234|     ;; self[8..+8] = i64 %238
 24235|     ;; slice[8..+8] = i64 %238
 24236|     ;; ptr = ptr %236
 24237|     ;; self = ptr %236
 24238|  %239 = gepS }, ptr %236, i64 %238                                                                                     ;L961<100<1042<2136<668
 24239|     ;; iter[0..+8] = ptr %236
 24240|     ;; iter[8..+8] = ptr %239
 24241|  %240 = gep %55, i64 8
 24242|  %241 = gep %55, i64 1264
 24243|  %242 = gep %55, i64 1240
 24244|  %243 = gep %55, i64 1248
 24245|  %244 = gep %55, i64 1080
 24246|  %245 = add i64 %93, -1
 24247|  %246 = gep %55, i64 1136
 24248|  %247 = gep %55, i64 1664
 24249|  %248 = gep %55, i64 1632
 24250|  %249 = gep %55, i64 1640
 24251|  %250 = gep %55, i64 1232
 24252|  %251 = gep %21, i64 177
 24253|  br label %252                                                                                                         ;L668
 24254| 
 24255| 252: ; preds = %383, %235
 24256|  %253 = phi ptr [ %236, %235 ], [ %256, %383 ]                                                                         ;L668
 24257|     ;; iter[0..+8] = ptr %253
 24258|     ;; self = ptr undef
 24259|     ;; ptr = ptr %253
 24260|     ;; self = ptr %253
 24261|     ;; end_or_len = ptr %239
 24264|  %254 = icmp eq ptr %253, %239                                                                                         ;L1714<180<668
 24265|  br i1 %254, label %384, label %255                                                                                    ;L180<668
 24266| 
 24267| 255: ; preds = %252
 24268|  %256 = gep %253, i64 32                                                                                               ;L656<185<668
 24269|     ;; iter[0..+8] = ptr %256
 24270|     ;; a = ptr %253
 24271|     ;; e = ptr %253
 24272|  %257 = gep %253, i64 24                                                                                               ;L669
 24273|  %258 = load ptr, ptr %257, , !!8, !!8                                                                                 ;L669
 24274|     ;; self = ptr %258
 24275|     ;; self = ptr %258
 24276|     ;; self = ptr %258
 24277|     ;; self = ptr %55
 24278|  %259 = load i64, ptr %55, , !!8                                                                                       ;L1136<1482<669
 24279|  %260 = trunc nuw i64 %259 to i1                                                                                       ;L1136<1482<669
 24280|  br i1 %260, label %270, label %261                                                                                    ;L1136<1482<669
 24281| 
 24282| 261: ; preds = %255
 24283|     ;; team = ptr %55
 24284|  %262 = load i64, ptr %240, , !!8                                                                                      ;L1137<1482<669
 24285|     ;; team = i64 %262
 24286|  %263 = icmp ult i64 %262, 2                                                                                           ;L1483<669
 24287|  br i1 %263, label %264, label %269                                                                                    ;L1483<669
 24288| 
 24289| 264: ; preds = %261
 24291|  %265 = gep %258, i64 56                                                                                               ;L122<1483<669
 24292|  %266 = gepS %265, i64 %262                                                                                            ;L122<1483<669
 24293|  %267 = load i64, ptr %266, , !!8                                                                                      ;L122<1483<669
 24294|  %268 = icmp eq i64 %267, 0                                                                                            ;L122<1483<669
 24295|  br i1 %268, label %270, label %383                                                                                    ;L669
 24296| 
 24297| 269: ; preds = %261
 24298|  invoke void @core::panicking18panic_bounds_check(i64 %262, i64 2, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.62) #30
 24299|  to label %46 unwind label %105                                                                                        ;L1483<669
 24300| 
 24301| 270: ; preds = %264, %255
 24302|  %271 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %241, ptr %55, ptr %258)
 24303|  to label %272 unwind label %105                                                                                       ;L673
 24304| 
 24305| 272: ; preds = %270
 24306|  br i1 %271, label %273, label %383                                                                                    ;L673
 24307| 
 24308| 273: ; preds = %272
 24309|  %274 = load i64, ptr %253, , !!8                                                                                      ;L677
 24312|     ;; __self_discr = i64 %274
 24313|     ;; __arg1_discr = i64 0
 24314|  %275 = icmp eq i64 %274, 0                                                                                            ;L81<677
 24315|  br i1 %275, label %276, label %283                                                                                    ;L677
 24316| 
 24317| 276: ; preds = %273
 24318|  %277 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10block_move(ptr %258)
 24319|  to label %278 unwind label %105                                                                                       ;L677
 24320| 
 24321| 278: ; preds = %276
 24322|  br i1 %277, label %283, label %279                                                                                    ;L677
 24323| 
 24324| 279: ; preds = %278
 24325|  %280 = gep %258, i64 1600                                                                                             ;L678
 24326|  %281 = load i64, ptr %280, , !!8                                                                                      ;L678
 24327|     ;; rhs = i64 %281
 24328|  %282 = call i64 @llvm.usub.sat.i64(i64 %102, i64 %281)                                                                ;L2472<678
 24329|     ;; move_speed = i64 %282
 24330|  br label %283                                                                                                         ;L677
 24331| 
 24332| 283: ; preds = %279, %278, %273
 24333|  %284 = phi i64 [ %282, %279 ], [ %102, %278 ], [ %102, %273 ]                                                         ;L0
 24334|     ;; move_speed = i64 %284
 24335|     ;; self = ptr %91
 24336|  %285 = load i64, ptr %242, , !!8                                                                                      ;L26<683
 24337|  %286 = load i64, ptr %243, , !!8                                                                                      ;L26<683
 24338|  %287 = load i64, ptr %244, , !!8                                                                                      ;L26<683
 24339|  %288 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %90, ptr %55, ptr %258)
 24340|  to label %289 unwind label %105                                                                                       ;L683
 24341| 
 24342| 289: ; preds = %283
 24343|  %290 = mul i64 %286, %245                                                                                             ;L26<683
 24344|  %291 = load i32, ptr %246, , !!8                                                                                      ;L1511<683
 24345|     ;; mult = i32 %291
 24346|  %292 = icmp eq i32 %291, 0                                                                                            ;L1512<683
 24347|  br i1 %292, label %293, label %295                                                                                    ;L1512<683
 24348| 
 24349| 293: ; preds = %289
 24350|  %294 = load i64, ptr %247, , !!8                                                                                      ;L1513<683
 24351|  br label %301                                                                                                         ;L1512<683
 24352| 
 24353| 295: ; preds = %289
 24354|  %296 = sext i32 %291 to i64                                                                                           ;L1511<683
 24355|     ;; mult = i64 %296
 24356|  %297 = load i64, ptr %247, , !!8                                                                                      ;L1515<683
 24357|  %298 = add nsw i64 %296, 100                                                                                          ;L1515<683
 24358|  %299 = mul i64 %297, %298                                                                                             ;L1515<683
 24359|  %300 = udiv i64 %299, 100                                                                                             ;L1515<683
 24360|  br label %301                                                                                                         ;L1512<683
 24361| 
 24362| 301: ; preds = %295, %293
 24363|  %302 = phi i64 [ %294, %293 ], [ %300, %295 ]                                                                         ;L0<683
 24364|  %303 = gep %258, i64 1136                                                                                             ;L1511<683
 24365|  %304 = load i32, ptr %303, , !!8                                                                                      ;L1511<683
 24366|     ;; mult = i32 %304
 24367|  %305 = icmp eq i32 %304, 0                                                                                            ;L1512<683
 24368|  br i1 %305, label %306, label %309                                                                                    ;L1512<683
 24369| 
 24370| 306: ; preds = %301
 24371|  %307 = gep %258, i64 1664                                                                                             ;L1513<683
 24372|  %308 = load i64, ptr %307, , !!8                                                                                      ;L1513<683
 24373|  br label %316                                                                                                         ;L1512<683
 24374| 
 24375| 309: ; preds = %301
 24376|  %310 = sext i32 %304 to i64                                                                                           ;L1511<683
 24377|     ;; mult = i64 %310
 24378|  %311 = gep %258, i64 1664                                                                                             ;L1515<683
 24379|  %312 = load i64, ptr %311, , !!8                                                                                      ;L1515<683
 24380|  %313 = add nsw i64 %310, 100                                                                                          ;L1515<683
 24381|  %314 = mul i64 %312, %313                                                                                             ;L1515<683
 24382|  %315 = udiv i64 %314, 100                                                                                             ;L1515<683
 24383|  br label %316                                                                                                         ;L1512<683
 24384| 
 24385| 316: ; preds = %309, %306
 24386|  %317 = phi i64 [ %308, %306 ], [ %315, %309 ]                                                                         ;L0<683
 24388|  %318 = gep %258, i64 1632                                                                                             ;L2158<684
 24389|  %319 = load i64, ptr %318, , !!8                                                                                      ;L2158<684
 24390|     ;; x1 = i64 %319
 24391|     ;; self = i64 %319
 24392|  %320 = gep %258, i64 1640                                                                                             ;L2158<684
 24393|  %321 = load i64, ptr %320, , !!8                                                                                      ;L2158<684
 24394|     ;; y1 = i64 %321
 24395|     ;; self = i64 %321
 24396|  %322 = load i64, ptr %248, , !!8                                                                                      ;L2158<684
 24397|     ;; x2 = i64 %322
 24398|     ;; other = i64 %322
 24399|  %323 = load i64, ptr %249, , !!8                                                                                      ;L2158<684
 24400|     ;; y2 = i64 %323
 24401|     ;; other = i64 %323
 24402|  %324 = icmp ult i64 %319, %322                                                                                        ;L3147<7<2158<684
 24403|  %325 = sub nuw i64 %322, %319                                                                                         ;L3147<7<2158<684
 24404|  %326 = sub nuw i64 %319, %322                                                                                         ;L3147<7<2158<684
 24405|  %327 = select i1 %324, i64 %325, i64 %326                                                                             ;L3147<7<2158<684
 24406|     ;; dx = i64 %327
 24407|  %328 = icmp ult i64 %321, %323                                                                                        ;L3147<8<2158<684
 24408|  %329 = sub nuw i64 %323, %321                                                                                         ;L3147<8<2158<684
 24409|  %330 = sub nuw i64 %321, %323                                                                                         ;L3147<8<2158<684
 24410|  %331 = select i1 %328, i64 %329, i64 %330                                                                             ;L3147<8<2158<684
 24411|     ;; dy = i64 %331
 24412|  %332 = mul i64 %327, %327                                                                                             ;L9<2158<684
 24413|  %333 = mul i64 %331, %331                                                                                             ;L9<2158<684
 24414|  %334 = add i64 %333, %332                                                                                             ;L9<2158<684
 24415|     ;; dist_sq = i64 %334
 24416|  %335 = mul i64 %284, 30                                                                                               ;L687
 24417|  %336 = add i64 %335, %285                                                                                             ;L26<683
 24418|  %337 = add i64 %336, %290                                                                                             ;L26<683
 24419|  %338 = add i64 %337, %287                                                                                             ;L683
 24420|  %339 = add i64 %338, %288                                                                                             ;L683
 24421|  %340 = add i64 %339, %302                                                                                             ;L683
 24422|  %341 = add i64 %340, %317                                                                                             ;L687
 24423|     ;; max_dist = i64 %341
 24424|  %342 = mul i64 %341, %341                                                                                             ;L688
 24425|  %343 = icmp ugt i64 %334, %342                                                                                        ;L688
 24426|  br i1 %343, label %383, label %344                                                                                    ;L688
 24427| 
 24428| 344: ; preds = %316
 24429|     ;; self = ptr %91
 24430|     ;; self = ptr %91
 24431|  %345 = load ptr, ptr %90, , !!8, !!8                                                                                  ;L441<2127<2445<693
 24432|  %346 = load ptr, ptr %250, , !!8, !!8                                                                                 ;L441<2127<2445<693
 24433|  %347 = gep %346, i64 16                                                                                               ;L2445<693
 24434|  %348 = load i64, ptr %347,                                                                                            ;L2445<693
 24435|  %349 = add nsw i64 %348, -1                                                                                           ;L2445<693
 24436|  %350 = and i64 %349, -16                                                                                              ;L2445<693
 24437|  %351 = gep %345, i64 %350                                                                                             ;L2445<693
 24438|  %352 = gep %351, i64 16                                                                                               ;L2445<693
 24439|  %353 = gep %346, i64 288                                                                                              ;L693
 24440|  %354 = load ptr, ptr %353, , !!8                                                                                      ;L693
 24441|  %355 = invoke zeroext i1 %354(ptr %352)
 24442|  to label %356 unwind label %105                                                                                       ;L693
 24443| 
 24444| 356: ; preds = %344
 24445|  br i1 %355, label %357, label %361                                                                                    ;L693
 24446| 
 24447| 357: ; preds = %356
 24448|     ;; self = ptr %258
 24449|  %358 = gep %258, i64 104                                                                                              ;L1404<693
 24450|  %359 = load i64, ptr %358, , !!8                                                                                      ;L1404<693
 24451|  %360 = icmp eq i64 %359, 13                                                                                           ;L693
 24452|  br i1 %360, label %364, label %361                                                                                    ;L693
 24453| 
 24454| 361: ; preds = %368, %357, %356
 24457|  %362 = gep %258, i64 1472                                                                                             ;L700
 24458|  %363 = load i64, ptr %362, , !!8                                                                                      ;L700
 24459|  invoke void @ai::small_action4castNtB5_16SmallActionSkill3new(ptr sret([24 x i8]) %20, ptr %4, i64 %363)
 24460|  to label %369 unwind label %105                                                                                       ;L700
 24461| 
 24462| 364: ; preds = %357
 24463|  %365 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %90, ptr %62, ptr %55, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.56, ptr %258)
 24464|  to label %366 unwind label %105                                                                                       ;L694
 24465| 
 24466| 366: ; preds = %364
 24467|     ;; skill_damage = i64 %365
 24468|  %367 = invoke zeroext i1 @ai::utils13is_dash_worth(ptr %4, ptr %3, ptr %55, ptr %258, i64 %365)
 24469|  to label %368 unwind label %105                                                                                       ;L695
 24470| 
 24471| 368: ; preds = %366
 24472|  br i1 %367, label %361, label %383                                                                                    ;L695
 24473| 
 24474| 369: ; preds = %361
 24475|  call void @llvm.memcpy.p0.p0.i64(ptr %21, ptr %20, i64 24, i1 false)                                                  ;L700
 24476|  store i8 16, ptr %251,                                                                                                ;L700
 24478|     ;; self = ptr %24
 24479|     ;; self = ptr %24
 24480|     ;; value = ptr %21
 24481|     ;; src = ptr %21
 24482|     ;; additional = i64 1
 24483|     ;; needed_extra_cap = i64 1
 24484|     ;; needed_extra_cap = i64 1
 24485|     ;; strategy = i8 1
 24486|  %370 = load i64, ptr %82, , !!37972, !!8                                                                              ;L1428<700
 24487|     ;; self = ptr %24
 24488|  %371 = load i64, ptr %81, , !!37972, !!8                                                                              ;L149<1428<700
 24489|  %372 = icmp eq i64 %370, %371                                                                                         ;L1428<700
 24490|  br i1 %372, label %373, label %378                                                                                    ;L1428<700
 24491| 
 24492| 373: ; preds = %369
 24493|     ;; self = ptr %24
 24494|     ;; self = ptr %24
 24495|     ;; self = ptr %24
 24496|     ;; used_cap = i64 %370
 24497|     ;; used_cap = i64 %370
 24498|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %24, i64 %370, i64 1, i1 zeroext true)
 24499|  to label %374 unwind label %376, !!37972                                                                              ;L619<430<738<1429<700
 24500| 
 24501| 374: ; preds = %373
 24502|  %375 = load i64, ptr %82, , !!37972                                                                                   ;L1432<700
 24503|  br label %378                                                                                                         ;L619<430<738<1429<700
 24504| 
 24505| 376: ; preds = %373
 24506|  %377 = cleanuppad within none []
 24507|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %21) #32 [ "funclet"(token %377) ] ;L1436<700
 24508|  cleanupret from %377 unwind label %105
 24509| 
 24510| 378: ; preds = %374, %369
 24511|  %379 = phi i64 [ %375, %374 ], [ %370, %369 ]                                                                         ;L1434<700
 24512|     ;; self = ptr %24
 24513|  %380 = load ptr, ptr %24, , !!37972, !!8, !!8                                                                         ;L138<1432<700
 24514|     ;; self = ptr %380
 24515|     ;; count = i64 %379
 24516|  %381 = gepS %380, i64 %379                                                                                            ;L961<1432<700
 24517|     ;; end = ptr %381
 24518|     ;; dst = ptr %381
 24519|  call void @llvm.memcpy.p0.p0.i64(ptr %381, ptr %21, i64 184, i1 false)                                                ;L1933<1433<700
 24520|  %382 = add i64 %379, 1                                                                                                ;L1434<700
 24521|  store i64 %382, ptr %82, , !!37972                                                                                    ;L1434<700
 24523|  br label %383                                                                                                         ;L668
 24524| 
 24525| 383: ; preds = %378, %368, %316, %272, %264
 24526|  br label %252                                                                                                         ;L1714<180<668
 24527| 
 24528| 384: ; preds = %252
 24529|  %385 = load ptr, ptr %90, , !!8, !!8                                                                                  ;L703
 24530|  %386 = gep %91, i64 8                                                                                                 ;L703
 24531|  %387 = load ptr, ptr %386, , !!8, !!8                                                                                 ;L703
 24534|     ;; champ = ptr %55
 24538|  %388 = gep %387, i64 16                                                                                               ;L2445<1183<703
 24539|  %389 = load i64, ptr %388, , !!38003                                                                                  ;L2445<1183<703
 24540|  %390 = add nsw i64 %389, -1                                                                                           ;L2445<1183<703
 24541|  %391 = and i64 %390, -16                                                                                              ;L2445<1183<703
 24542|  %392 = gep %385, i64 %391                                                                                             ;L2445<1183<703
 24543|  %393 = gep %392, i64 16                                                                                               ;L2445<1183<703
 24544|  %394 = gep %387, i64 160                                                                                              ;L1183<703
 24545|  %395 = load ptr, ptr %394, , !!38003, !!8                                                                             ;L1183<703
 24546|  invoke void %395(ptr sret([288 x i8]) %9, ptr %393, ptr %62, ptr %55, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.56)
 24547|  to label %396 unwind label %105                                                                                       ;L1183<703
 24548| 
 24549| 396: ; preds = %384
 24550|     ;; self = ptr %9
 24551|  %397 = gep %9, i64 72                                                                                                 ;L633<1183<703
 24552|  %398 = load i32, ptr %397, , !!38003, !!8                                                                             ;L633<1183<703
 24553|  %399 = icmp eq i32 %398, -1                                                                                           ;L633<1183<703
 24555|  %400 = select i1 %399, i64 30, i64 90                                                                                 ;L1183<703
 24556|     ;; walk = i64 %400
 24557|     ;; self = ptr %28
 24558|     ;; self = ptr %28
 24559|     ;; self = ptr %28
 24560|  %401 = load ptr, ptr %28, , !!8, !!8                                                                                  ;L138<2073<2136<704
 24561|     ;; p = ptr %401
 24562|  %402 = gep %28, i64 24                                                                                                ;L2075<2136<704
 24563|  %403 = load i64, ptr %402, , !!8                                                                                      ;L2075<2136<704
 24564|     ;; len = i64 %403
 24565|     ;; count = i64 %403
 24566|     ;; self[0..+8] = ptr %401
 24567|     ;; slice[0..+8] = ptr %401
 24568|     ;; self[8..+8] = i64 %403
 24569|     ;; slice[8..+8] = i64 %403
 24570|     ;; ptr = ptr %401
 24571|     ;; self = ptr %401
 24572|  %404 = getelementptr ptr, ptr %401, i64 %403                                                                          ;L961<100<1042<2136<704
 24573|     ;; iter[0..+8] = ptr %401
 24574|     ;; iter[8..+8] = ptr %404
 24575|  %405 = mul i64 %400, %102
 24576|  %406 = gep %19, i64 177
 24577|  br label %407                                                                                                         ;L704
 24578| 
 24579| 407: ; preds = %493, %396
 24580|  %408 = phi ptr [ %401, %396 ], [ %411, %493 ]                                                                         ;L704
 24581|     ;; iter[0..+8] = ptr %408
 24582|     ;; self = ptr undef
 24583|     ;; ptr = ptr %408
 24584|     ;; self = ptr %408
 24585|     ;; end_or_len = ptr %404
 24588|  %409 = icmp eq ptr %408, %404                                                                                         ;L1714<180<704
 24589|  br i1 %409, label %494, label %410                                                                                    ;L180<704
 24590| 
 24591| 410: ; preds = %407
 24592|  %411 = gep %408, i64 8                                                                                                ;L656<185<704
 24593|     ;; iter[0..+8] = ptr %411
 24594|     ;; e = ptr %408
 24595|  %412 = load ptr, ptr %408, , !!8, !!8                                                                                 ;L705
 24596|  %413 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %241, ptr %55, ptr %412)
 24597|  to label %414 unwind label %105                                                                                       ;L705
 24598| 
 24599| 414: ; preds = %410
 24600|  br i1 %413, label %415, label %493                                                                                    ;L705
 24601| 
 24602| 415: ; preds = %414
 24603|     ;; self = ptr %91
 24604|  %416 = load i64, ptr %242, , !!8                                                                                      ;L26<709
 24605|  %417 = load i64, ptr %243, , !!8                                                                                      ;L26<709
 24606|  %418 = load i64, ptr %244, , !!8                                                                                      ;L26<709
 24607|  %419 = load ptr, ptr %408, , !!8, !!8                                                                                 ;L709
 24608|  %420 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %90, ptr %55, ptr %419)
 24609|  to label %421 unwind label %105                                                                                       ;L709
 24610| 
 24611| 421: ; preds = %415
 24612|  %422 = mul i64 %417, %245                                                                                             ;L26<709
 24613|  %423 = load i32, ptr %246, , !!8                                                                                      ;L1511<709
 24614|     ;; mult = i32 %423
 24615|  %424 = icmp eq i32 %423, 0                                                                                            ;L1512<709
 24616|  br i1 %424, label %425, label %427                                                                                    ;L1512<709
 24617| 
 24618| 425: ; preds = %421
 24619|  %426 = load i64, ptr %247, , !!8                                                                                      ;L1513<709
 24620|  br label %433                                                                                                         ;L1512<709
 24621| 
 24622| 427: ; preds = %421
 24623|  %428 = sext i32 %423 to i64                                                                                           ;L1511<709
 24624|     ;; mult = i64 %428
 24625|  %429 = load i64, ptr %247, , !!8                                                                                      ;L1515<709
 24626|  %430 = add nsw i64 %428, 100                                                                                          ;L1515<709
 24627|  %431 = mul i64 %429, %430                                                                                             ;L1515<709
 24628|  %432 = udiv i64 %431, 100                                                                                             ;L1515<709
 24629|  br label %433                                                                                                         ;L1512<709
 24630| 
 24631| 433: ; preds = %427, %425
 24632|  %434 = phi i64 [ %426, %425 ], [ %432, %427 ]                                                                         ;L0<709
 24633|  %435 = load ptr, ptr %408, , !!8, !!8                                                                                 ;L709
 24634|     ;; self = ptr %435
 24635|  %436 = gep %435, i64 1136                                                                                             ;L1511<709
 24636|  %437 = load i32, ptr %436, , !!8                                                                                      ;L1511<709
 24637|     ;; mult = i32 %437
 24638|  %438 = icmp eq i32 %437, 0                                                                                            ;L1512<709
 24639|  br i1 %438, label %439, label %442                                                                                    ;L1512<709
 24640| 
 24641| 439: ; preds = %433
 24642|  %440 = gep %435, i64 1664                                                                                             ;L1513<709
 24643|  %441 = load i64, ptr %440, , !!8                                                                                      ;L1513<709
 24644|  br label %449                                                                                                         ;L1512<709
 24645| 
 24646| 442: ; preds = %433
 24647|  %443 = sext i32 %437 to i64                                                                                           ;L1511<709
 24648|     ;; mult = i64 %443
 24649|  %444 = gep %435, i64 1664                                                                                             ;L1515<709
 24650|  %445 = load i64, ptr %444, , !!8                                                                                      ;L1515<709
 24651|  %446 = add nsw i64 %443, 100                                                                                          ;L1515<709
 24652|  %447 = mul i64 %445, %446                                                                                             ;L1515<709
 24653|  %448 = udiv i64 %447, 100                                                                                             ;L1515<709
 24654|  br label %449                                                                                                         ;L1512<709
 24655| 
 24656| 449: ; preds = %442, %439
 24657|  %450 = phi i64 [ %441, %439 ], [ %448, %442 ]                                                                         ;L0<709
 24659|     ;; self = ptr %435
 24660|  %451 = gep %435, i64 1632                                                                                             ;L2158<710
 24661|  %452 = load i64, ptr %451, , !!8                                                                                      ;L2158<710
 24662|     ;; x1 = i64 %452
 24663|     ;; self = i64 %452
 24664|  %453 = gep %435, i64 1640                                                                                             ;L2158<710
 24665|  %454 = load i64, ptr %453, , !!8                                                                                      ;L2158<710
 24666|     ;; y1 = i64 %454
 24667|     ;; self = i64 %454
 24668|  %455 = load i64, ptr %248, , !!8                                                                                      ;L2158<710
 24669|     ;; x2 = i64 %455
 24670|     ;; other = i64 %455
 24671|  %456 = load i64, ptr %249, , !!8                                                                                      ;L2158<710
 24672|     ;; y2 = i64 %456
 24673|     ;; other = i64 %456
 24674|  %457 = icmp ult i64 %452, %455                                                                                        ;L3147<7<2158<710
 24675|  %458 = sub nuw i64 %455, %452                                                                                         ;L3147<7<2158<710
 24676|  %459 = sub nuw i64 %452, %455                                                                                         ;L3147<7<2158<710
 24677|  %460 = select i1 %457, i64 %458, i64 %459                                                                             ;L3147<7<2158<710
 24678|     ;; dx = i64 %460
 24679|  %461 = icmp ult i64 %454, %456                                                                                        ;L3147<8<2158<710
 24680|  %462 = sub nuw i64 %456, %454                                                                                         ;L3147<8<2158<710
 24681|  %463 = sub nuw i64 %454, %456                                                                                         ;L3147<8<2158<710
 24682|  %464 = select i1 %461, i64 %462, i64 %463                                                                             ;L3147<8<2158<710
 24683|     ;; dy = i64 %464
 24684|  %465 = mul i64 %460, %460                                                                                             ;L9<2158<710
 24685|  %466 = mul i64 %464, %464                                                                                             ;L9<2158<710
 24686|  %467 = add i64 %466, %465                                                                                             ;L9<2158<710
 24687|     ;; dist_sq = i64 %467
 24688|  %468 = add i64 %416, %405                                                                                             ;L26<709
 24689|  %469 = add i64 %468, %422                                                                                             ;L26<709
 24690|  %470 = add i64 %469, %418                                                                                             ;L709
 24691|  %471 = add i64 %470, %420                                                                                             ;L709
 24692|  %472 = add i64 %471, %434                                                                                             ;L709
 24693|  %473 = add i64 %472, %450                                                                                             ;L713
 24694|     ;; max_dist = i64 %473
 24695|  %474 = mul i64 %473, %473                                                                                             ;L714
 24696|  %475 = icmp ugt i64 %467, %474                                                                                        ;L714
 24697|  br i1 %475, label %493, label %476                                                                                    ;L714
 24698| 
 24699| 476: ; preds = %449
 24702|  %477 = gep %435, i64 1472                                                                                             ;L718
 24703|  %478 = load i64, ptr %477, , !!8                                                                                      ;L718
 24704|  invoke void @ai::small_action4castNtB5_16SmallActionSkill3new(ptr sret([24 x i8]) %18, ptr %4, i64 %478)
 24705|  to label %479 unwind label %105                                                                                       ;L718
 24706| 
 24707| 479: ; preds = %476
 24708|  call void @llvm.memcpy.p0.p0.i64(ptr %19, ptr %18, i64 24, i1 false)                                                  ;L718
 24709|  store i8 16, ptr %406,                                                                                                ;L718
 24711|     ;; self = ptr %24
 24712|     ;; self = ptr %24
 24713|     ;; value = ptr %19
 24714|     ;; src = ptr %19
 24715|     ;; additional = i64 1
 24716|     ;; needed_extra_cap = i64 1
 24717|     ;; needed_extra_cap = i64 1
 24718|     ;; strategy = i8 1
 24719|  %480 = load i64, ptr %82, , !!38135, !!8                                                                              ;L1428<718
 24720|     ;; self = ptr %24
 24721|  %481 = load i64, ptr %81, , !!38135, !!8                                                                              ;L149<1428<718
 24722|  %482 = icmp eq i64 %480, %481                                                                                         ;L1428<718
 24723|  br i1 %482, label %483, label %488                                                                                    ;L1428<718
 24724| 
 24725| 483: ; preds = %479
 24726|     ;; self = ptr %24
 24727|     ;; self = ptr %24
 24728|     ;; self = ptr %24
 24729|     ;; used_cap = i64 %480
 24730|     ;; used_cap = i64 %480
 24731|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %24, i64 %480, i64 1, i1 zeroext true)
 24732|  to label %484 unwind label %486, !!38135                                                                              ;L619<430<738<1429<718
 24733| 
 24734| 484: ; preds = %483
 24735|  %485 = load i64, ptr %82, , !!38135                                                                                   ;L1432<718
 24736|  br label %488                                                                                                         ;L619<430<738<1429<718
 24737| 
 24738| 486: ; preds = %483
 24739|  %487 = cleanuppad within none []
 24740|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %19) #32 [ "funclet"(token %487) ] ;L1436<718
 24741|  cleanupret from %487 unwind label %105
 24742| 
 24743| 488: ; preds = %484, %479
 24744|  %489 = phi i64 [ %485, %484 ], [ %480, %479 ]                                                                         ;L1434<718
 24745|     ;; self = ptr %24
 24746|  %490 = load ptr, ptr %24, , !!38135, !!8, !!8                                                                         ;L138<1432<718
 24747|     ;; self = ptr %490
 24748|     ;; count = i64 %489
 24749|  %491 = gepS %490, i64 %489                                                                                            ;L961<1432<718
 24750|     ;; end = ptr %491
 24751|     ;; dst = ptr %491
 24752|  call void @llvm.memcpy.p0.p0.i64(ptr %491, ptr %19, i64 184, i1 false)                                                ;L1933<1433<718
 24753|  %492 = add i64 %489, 1                                                                                                ;L1434<718
 24754|  store i64 %492, ptr %82, , !!38135                                                                                    ;L1434<718
 24756|  br label %493                                                                                                         ;L704
 24757| 
 24758| 493: ; preds = %488, %449, %414
 24759|  br label %407                                                                                                         ;L1714<180<704
 24760| 
 24761| 494: ; preds = %407
 24762|  %495 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %241, ptr %55, ptr %55)
 24763|  to label %496 unwind label %105                                                                                       ;L721
 24764| 
 24765| 496: ; preds = %494
 24766|  br i1 %495, label %497, label %234                                                                                    ;L721
 24767| 
 24768| 497: ; preds = %496
 24769|  %498 = load ptr, ptr %90, , !!8, !!8                                                                                  ;L721
 24770|  %499 = load ptr, ptr %386, , !!8, !!8                                                                                 ;L721
 24771|  %500 = invoke fastcc zeroext i1 @ai::fight_check31should_add_self_etc_buff_action(ptr %3, ptr %4, ptr %55, ptr %498, ptr %499)
 24772|  to label %501 unwind label %105                                                                                       ;L721
 24773| 
 24774| 501: ; preds = %497
 24775|  br i1 %500, label %502, label %234                                                                                    ;L721
 24776| 
 24777| 502: ; preds = %501
 24780|  %503 = gep %55, i64 1472                                                                                              ;L722
 24781|  %504 = load i64, ptr %503, , !!8                                                                                      ;L722
 24782|  invoke void @ai::small_action4castNtB5_16SmallActionSkill3new(ptr sret([24 x i8]) %16, ptr %4, i64 %504)
 24783|  to label %505 unwind label %105                                                                                       ;L722
 24784| 
 24785| 505: ; preds = %502
 24786|  call void @llvm.memcpy.p0.p0.i64(ptr %17, ptr %16, i64 24, i1 false)                                                  ;L722
 24787|  %506 = gep %17, i64 177                                                                                               ;L722
 24788|  store i8 16, ptr %506,                                                                                                ;L722
 24790|     ;; self = ptr %24
 24791|     ;; self = ptr %24
 24792|     ;; value = ptr %17
 24793|     ;; src = ptr %17
 24794|     ;; additional = i64 1
 24795|     ;; needed_extra_cap = i64 1
 24796|     ;; needed_extra_cap = i64 1
 24797|     ;; strategy = i8 1
 24798|  %507 = load i64, ptr %82, , !!38171, !!8                                                                              ;L1428<722
 24799|     ;; self = ptr %24
 24800|  %508 = load i64, ptr %81, , !!38171, !!8                                                                              ;L149<1428<722
 24801|  %509 = icmp eq i64 %507, %508                                                                                         ;L1428<722
 24802|  br i1 %509, label %510, label %515                                                                                    ;L1428<722
 24803| 
 24804| 510: ; preds = %505
 24805|     ;; self = ptr %24
 24806|     ;; self = ptr %24
 24807|     ;; self = ptr %24
 24808|     ;; used_cap = i64 %507
 24809|     ;; used_cap = i64 %507
 24810|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %24, i64 %507, i64 1, i1 zeroext true)
 24811|  to label %511 unwind label %513, !!38171                                                                              ;L619<430<738<1429<722
 24812| 
 24813| 511: ; preds = %510
 24814|  %512 = load i64, ptr %82, , !!38171                                                                                   ;L1432<722
 24815|  br label %515                                                                                                         ;L619<430<738<1429<722
 24816| 
 24817| 513: ; preds = %510
 24818|  %514 = cleanuppad within none []
 24819|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %17) #32 [ "funclet"(token %514) ] ;L1436<722
 24820|  cleanupret from %514 unwind label %105
 24821| 
 24822| 515: ; preds = %511, %505
 24823|  %516 = phi i64 [ %512, %511 ], [ %507, %505 ]                                                                         ;L1434<722
 24824|     ;; self = ptr %24
 24825|  %517 = load ptr, ptr %24, , !!38171, !!8, !!8                                                                         ;L138<1432<722
 24826|     ;; self = ptr %517
 24827|     ;; count = i64 %516
 24828|  %518 = gepS %517, i64 %516                                                                                            ;L961<1432<722
 24829|     ;; end = ptr %518
 24830|     ;; dst = ptr %518
 24831|  call void @llvm.memcpy.p0.p0.i64(ptr %518, ptr %17, i64 184, i1 false)                                                ;L1933<1433<722
 24832|  %519 = add i64 %516, 1                                                                                                ;L1434<722
 24833|  store i64 %519, ptr %82, , !!38171                                                                                    ;L1434<722
 24835|  br label %234                                                                                                         ;L721
 24836| 
 24837| 520: ; preds = %234
 24838|  %521 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_skill2(ptr %55)
 24839|  to label %522 unwind label %105                                                                                       ;L726
 24840| 
 24841| 522: ; preds = %520
 24842|  br i1 %521, label %528, label %523                                                                                    ;L726
 24843| 
 24844| 523: ; preds = %836, %821, %789, %522, %234
 24845|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %24, i64 32, i1 false)                                                   ;L785
 24848|  invoke void @gc::simulation4game10blackboard11SmallActionRNtNtB13_6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %26)
 24849|  to label %527 unwind label %524                                                                                       ;L825<786
 24850| 
 24851| 524: ; preds = %523
 24852|  %525 = cleanuppad within none []
 24854|  invoke void @gc::simulation4game10blackboard11SmallActionRNtNtB1a_6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %26) [ "funclet"(token %525) ]
 24855|  to label %526 unwind label %77                                                                                        ;L825<825<786
 24856| 
 24857| 526: ; preds = %524
 24858|  cleanupret from %525 unwind label %77
 24859| 
 24860| 527: ; preds = %523
 24862|  invoke void @gc::simulation4game10blackboard11SmallActionRNtNtB1a_6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %26)
 24863|  to label %841 unwind label %77                                                                                        ;L825<825<786
 24864| 
 24865| 528: ; preds = %522
 24866|     ;; self = ptr %26
 24867|     ;; self = ptr %26
 24868|     ;; self = ptr %26
 24869|  %529 = load ptr, ptr %26, , !!8, !!8                                                                                  ;L138<2073<2136<727
 24870|     ;; p = ptr %529
 24871|  %530 = gep %26, i64 24                                                                                                ;L2075<2136<727
 24872|  %531 = load i64, ptr %530, , !!8                                                                                      ;L2075<2136<727
 24873|     ;; len = i64 %531
 24874|     ;; count = i64 %531
 24875|     ;; self[0..+8] = ptr %529
 24876|     ;; slice[0..+8] = ptr %529
 24877|     ;; self[8..+8] = i64 %531
 24878|     ;; slice[8..+8] = i64 %531
 24879|     ;; ptr = ptr %529
 24880|     ;; self = ptr %529
 24881|  %532 = gepS }, ptr %529, i64 %531                                                                                     ;L961<100<1042<2136<727
 24882|     ;; iter[0..+8] = ptr %529
 24883|     ;; iter[8..+8] = ptr %532
 24884|  %533 = gep %55, i64 8
 24885|  %534 = gep %96, i64 40
 24886|  %535 = gep %96, i64 16
 24887|  %536 = gep %96, i64 24
 24888|  %537 = gep %55, i64 1080
 24889|  %538 = add i64 %93, -1
 24890|  %539 = gep %55, i64 1136
 24891|  %540 = gep %55, i64 1664
 24892|  %541 = gep %55, i64 1632
 24893|  %542 = gep %55, i64 1640
 24894|  %543 = gep %96, i64 8
 24895|  %544 = gep %15, i64 177
 24896|  br label %545                                                                                                         ;L727
 24897| 
 24898| 545: ; preds = %676, %528
 24899|  %546 = phi ptr [ %529, %528 ], [ %549, %676 ]                                                                         ;L727
 24900|     ;; iter[0..+8] = ptr %546
 24901|     ;; self = ptr undef
 24902|     ;; ptr = ptr %546
 24903|     ;; self = ptr %546
 24904|     ;; end_or_len = ptr %532
 24907|  %547 = icmp eq ptr %546, %532                                                                                         ;L1714<180<727
 24908|  br i1 %547, label %677, label %548                                                                                    ;L180<727
 24909| 
 24910| 548: ; preds = %545
 24911|  %549 = gep %546, i64 32                                                                                               ;L656<185<727
 24912|     ;; iter[0..+8] = ptr %549
 24913|     ;; a = ptr %546
 24914|     ;; e = ptr %546
 24915|  %550 = gep %546, i64 24                                                                                               ;L728
 24916|  %551 = load ptr, ptr %550, , !!8, !!8                                                                                 ;L728
 24917|     ;; self = ptr %551
 24918|     ;; self = ptr %551
 24919|     ;; self = ptr %551
 24920|     ;; self = ptr %55
 24921|  %552 = load i64, ptr %55, , !!8                                                                                       ;L1136<1482<728
 24922|  %553 = trunc nuw i64 %552 to i1                                                                                       ;L1136<1482<728
 24923|  br i1 %553, label %563, label %554                                                                                    ;L1136<1482<728
 24924| 
 24925| 554: ; preds = %548
 24926|     ;; team = ptr %55
 24927|  %555 = load i64, ptr %533, , !!8                                                                                      ;L1137<1482<728
 24928|     ;; team = i64 %555
 24929|  %556 = icmp ult i64 %555, 2                                                                                           ;L1483<728
 24930|  br i1 %556, label %557, label %562                                                                                    ;L1483<728
 24931| 
 24932| 557: ; preds = %554
 24934|  %558 = gep %551, i64 56                                                                                               ;L122<1483<728
 24935|  %559 = gepS %558, i64 %555                                                                                            ;L122<1483<728
 24936|  %560 = load i64, ptr %559, , !!8                                                                                      ;L122<1483<728
 24937|  %561 = icmp eq i64 %560, 0                                                                                            ;L122<1483<728
 24938|  br i1 %561, label %563, label %676                                                                                    ;L728
 24939| 
 24940| 562: ; preds = %554
 24941|  invoke void @core::panicking18panic_bounds_check(i64 %555, i64 2, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.62) #30
 24942|  to label %46 unwind label %105                                                                                        ;L1483<728
 24943| 
 24944| 563: ; preds = %557, %548
 24945|  %564 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %534, ptr %55, ptr %551)
 24946|  to label %565 unwind label %105                                                                                       ;L732
 24947| 
 24948| 565: ; preds = %563
 24949|  br i1 %564, label %566, label %676                                                                                    ;L732
 24950| 
 24951| 566: ; preds = %565
 24952|  %567 = load i64, ptr %546, , !!8                                                                                      ;L736
 24955|     ;; __self_discr = i64 %567
 24956|     ;; __arg1_discr = i64 0
 24957|  %568 = icmp eq i64 %567, 0                                                                                            ;L81<736
 24958|  br i1 %568, label %569, label %576                                                                                    ;L736
 24959| 
 24960| 569: ; preds = %566
 24961|  %570 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10block_move(ptr %551)
 24962|  to label %571 unwind label %105                                                                                       ;L736
 24963| 
 24964| 571: ; preds = %569
 24965|  br i1 %570, label %576, label %572                                                                                    ;L736
 24966| 
 24967| 572: ; preds = %571
 24968|  %573 = gep %551, i64 1600                                                                                             ;L737
 24969|  %574 = load i64, ptr %573, , !!8                                                                                      ;L737
 24970|     ;; rhs = i64 %574
 24971|  %575 = call i64 @llvm.usub.sat.i64(i64 %102, i64 %574)                                                                ;L2472<737
 24972|     ;; move_speed = i64 %575
 24973|  br label %576                                                                                                         ;L736
 24974| 
 24975| 576: ; preds = %572, %571, %566
 24976|  %577 = phi i64 [ %575, %572 ], [ %102, %571 ], [ %102, %566 ]                                                         ;L0
 24977|     ;; move_speed = i64 %577
 24978|     ;; self = ptr %100
 24979|  %578 = load i64, ptr %535, , !!8                                                                                      ;L26<742
 24980|  %579 = load i64, ptr %536, , !!8                                                                                      ;L26<742
 24981|  %580 = load i64, ptr %537, , !!8                                                                                      ;L26<742
 24982|  %581 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %96, ptr %55, ptr %551)
 24983|  to label %582 unwind label %105                                                                                       ;L742
 24984| 
 24985| 582: ; preds = %576
 24986|  %583 = mul i64 %579, %538                                                                                             ;L26<742
 24987|  %584 = load i32, ptr %539, , !!8                                                                                      ;L1511<742
 24988|     ;; mult = i32 %584
 24989|  %585 = icmp eq i32 %584, 0                                                                                            ;L1512<742
 24990|  br i1 %585, label %586, label %588                                                                                    ;L1512<742
 24991| 
 24992| 586: ; preds = %582
 24993|  %587 = load i64, ptr %540, , !!8                                                                                      ;L1513<742
 24994|  br label %594                                                                                                         ;L1512<742
 24995| 
 24996| 588: ; preds = %582
 24997|  %589 = sext i32 %584 to i64                                                                                           ;L1511<742
 24998|     ;; mult = i64 %589
 24999|  %590 = load i64, ptr %540, , !!8                                                                                      ;L1515<742
 25000|  %591 = add nsw i64 %589, 100                                                                                          ;L1515<742
 25001|  %592 = mul i64 %590, %591                                                                                             ;L1515<742
 25002|  %593 = udiv i64 %592, 100                                                                                             ;L1515<742
 25003|  br label %594                                                                                                         ;L1512<742
 25004| 
 25005| 594: ; preds = %588, %586
 25006|  %595 = phi i64 [ %587, %586 ], [ %593, %588 ]                                                                         ;L0<742
 25007|  %596 = gep %551, i64 1136                                                                                             ;L1511<742
 25008|  %597 = load i32, ptr %596, , !!8                                                                                      ;L1511<742
 25009|     ;; mult = i32 %597
 25010|  %598 = icmp eq i32 %597, 0                                                                                            ;L1512<742
 25011|  br i1 %598, label %599, label %602                                                                                    ;L1512<742
 25012| 
 25013| 599: ; preds = %594
 25014|  %600 = gep %551, i64 1664                                                                                             ;L1513<742
 25015|  %601 = load i64, ptr %600, , !!8                                                                                      ;L1513<742
 25016|  br label %609                                                                                                         ;L1512<742
 25017| 
 25018| 602: ; preds = %594
 25019|  %603 = sext i32 %597 to i64                                                                                           ;L1511<742
 25020|     ;; mult = i64 %603
 25021|  %604 = gep %551, i64 1664                                                                                             ;L1515<742
 25022|  %605 = load i64, ptr %604, , !!8                                                                                      ;L1515<742
 25023|  %606 = add nsw i64 %603, 100                                                                                          ;L1515<742
 25024|  %607 = mul i64 %605, %606                                                                                             ;L1515<742
 25025|  %608 = udiv i64 %607, 100                                                                                             ;L1515<742
 25026|  br label %609                                                                                                         ;L1512<742
 25027| 
 25028| 609: ; preds = %602, %599
 25029|  %610 = phi i64 [ %601, %599 ], [ %608, %602 ]                                                                         ;L0<742
 25031|  %611 = gep %551, i64 1632                                                                                             ;L2158<743
 25032|  %612 = load i64, ptr %611, , !!8                                                                                      ;L2158<743
 25033|     ;; x1 = i64 %612
 25034|     ;; self = i64 %612
 25035|  %613 = gep %551, i64 1640                                                                                             ;L2158<743
 25036|  %614 = load i64, ptr %613, , !!8                                                                                      ;L2158<743
 25037|     ;; y1 = i64 %614
 25038|     ;; self = i64 %614
 25039|  %615 = load i64, ptr %541, , !!8                                                                                      ;L2158<743
 25040|     ;; x2 = i64 %615
 25041|     ;; other = i64 %615
 25042|  %616 = load i64, ptr %542, , !!8                                                                                      ;L2158<743
 25043|     ;; y2 = i64 %616
 25044|     ;; other = i64 %616
 25045|  %617 = icmp ult i64 %612, %615                                                                                        ;L3147<7<2158<743
 25046|  %618 = sub nuw i64 %615, %612                                                                                         ;L3147<7<2158<743
 25047|  %619 = sub nuw i64 %612, %615                                                                                         ;L3147<7<2158<743
 25048|  %620 = select i1 %617, i64 %618, i64 %619                                                                             ;L3147<7<2158<743
 25049|     ;; dx = i64 %620
 25050|  %621 = icmp ult i64 %614, %616                                                                                        ;L3147<8<2158<743
 25051|  %622 = sub nuw i64 %616, %614                                                                                         ;L3147<8<2158<743
 25052|  %623 = sub nuw i64 %614, %616                                                                                         ;L3147<8<2158<743
 25053|  %624 = select i1 %621, i64 %622, i64 %623                                                                             ;L3147<8<2158<743
 25054|     ;; dy = i64 %624
 25055|  %625 = mul i64 %620, %620                                                                                             ;L9<2158<743
 25056|  %626 = mul i64 %624, %624                                                                                             ;L9<2158<743
 25057|  %627 = add i64 %626, %625                                                                                             ;L9<2158<743
 25058|     ;; dist_sq = i64 %627
 25059|  %628 = mul i64 %577, 30                                                                                               ;L746
 25060|  %629 = add i64 %628, %578                                                                                             ;L26<742
 25061|  %630 = add i64 %629, %583                                                                                             ;L26<742
 25062|  %631 = add i64 %630, %580                                                                                             ;L742
 25063|  %632 = add i64 %631, %581                                                                                             ;L742
 25064|  %633 = add i64 %632, %595                                                                                             ;L742
 25065|  %634 = add i64 %633, %610                                                                                             ;L746
 25066|     ;; max_dist = i64 %634
 25067|  %635 = mul i64 %634, %634                                                                                             ;L747
 25068|  %636 = icmp ugt i64 %627, %635                                                                                        ;L747
 25069|  br i1 %636, label %676, label %637                                                                                    ;L747
 25070| 
 25071| 637: ; preds = %609
 25072|     ;; self = ptr %100
 25073|     ;; self = ptr %100
 25074|  %638 = load ptr, ptr %96, , !!8, !!8                                                                                  ;L441<2127<2445<752
 25075|  %639 = load ptr, ptr %543, , !!8, !!8                                                                                 ;L441<2127<2445<752
 25076|  %640 = gep %639, i64 16                                                                                               ;L2445<752
 25077|  %641 = load i64, ptr %640,                                                                                            ;L2445<752
 25078|  %642 = add nsw i64 %641, -1                                                                                           ;L2445<752
 25079|  %643 = and i64 %642, -16                                                                                              ;L2445<752
 25080|  %644 = gep %638, i64 %643                                                                                             ;L2445<752
 25081|  %645 = gep %644, i64 16                                                                                               ;L2445<752
 25082|  %646 = gep %639, i64 288                                                                                              ;L752
 25083|  %647 = load ptr, ptr %646, , !!8                                                                                      ;L752
 25084|  %648 = invoke zeroext i1 %647(ptr %645)
 25085|  to label %649 unwind label %105                                                                                       ;L752
 25086| 
 25087| 649: ; preds = %637
 25088|  br i1 %648, label %650, label %654                                                                                    ;L752
 25089| 
 25090| 650: ; preds = %649
 25091|     ;; self = ptr %551
 25092|  %651 = gep %551, i64 104                                                                                              ;L1404<752
 25093|  %652 = load i64, ptr %651, , !!8                                                                                      ;L1404<752
 25094|  %653 = icmp eq i64 %652, 13                                                                                           ;L752
 25095|  br i1 %653, label %657, label %654                                                                                    ;L752
 25096| 
 25097| 654: ; preds = %661, %650, %649
 25100|  %655 = gep %551, i64 1472                                                                                             ;L759
 25101|  %656 = load i64, ptr %655, , !!8                                                                                      ;L759
 25102|  invoke void @ai::small_action4castNtB5_17SmallActionSkill23new(ptr sret([24 x i8]) %14, ptr %4, i64 %656)
 25103|  to label %662 unwind label %105                                                                                       ;L759
 25104| 
 25105| 657: ; preds = %650
 25106|  %658 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %96, ptr %62, ptr %55, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.56, ptr %551)
 25107|  to label %659 unwind label %105                                                                                       ;L753
 25108| 
 25109| 659: ; preds = %657
 25110|     ;; skill_damage = i64 %658
 25111|  %660 = invoke zeroext i1 @ai::utils13is_dash_worth(ptr %4, ptr %3, ptr %55, ptr %551, i64 %658)
 25112|  to label %661 unwind label %105                                                                                       ;L754
 25113| 
 25114| 661: ; preds = %659
 25115|  br i1 %660, label %654, label %676                                                                                    ;L754
 25116| 
 25117| 662: ; preds = %654
 25118|  call void @llvm.memcpy.p0.p0.i64(ptr %15, ptr %14, i64 24, i1 false)                                                  ;L759
 25119|  store i8 17, ptr %544,                                                                                                ;L759
 25121|     ;; self = ptr %24
 25122|     ;; self = ptr %24
 25123|     ;; value = ptr %15
 25124|     ;; src = ptr %15
 25125|     ;; additional = i64 1
 25126|     ;; needed_extra_cap = i64 1
 25127|     ;; needed_extra_cap = i64 1
 25128|     ;; strategy = i8 1
 25129|  %663 = load i64, ptr %82, , !!38311, !!8                                                                              ;L1428<759
 25130|     ;; self = ptr %24
 25131|  %664 = load i64, ptr %81, , !!38311, !!8                                                                              ;L149<1428<759
 25132|  %665 = icmp eq i64 %663, %664                                                                                         ;L1428<759
 25133|  br i1 %665, label %666, label %671                                                                                    ;L1428<759
 25134| 
 25135| 666: ; preds = %662
 25136|     ;; self = ptr %24
 25137|     ;; self = ptr %24
 25138|     ;; self = ptr %24
 25139|     ;; used_cap = i64 %663
 25140|     ;; used_cap = i64 %663
 25141|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %24, i64 %663, i64 1, i1 zeroext true)
 25142|  to label %667 unwind label %669, !!38311                                                                              ;L619<430<738<1429<759
 25143| 
 25144| 667: ; preds = %666
 25145|  %668 = load i64, ptr %82, , !!38311                                                                                   ;L1432<759
 25146|  br label %671                                                                                                         ;L619<430<738<1429<759
 25147| 
 25148| 669: ; preds = %666
 25149|  %670 = cleanuppad within none []
 25150|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %15) #32 [ "funclet"(token %670) ] ;L1436<759
 25151|  cleanupret from %670 unwind label %105
 25152| 
 25153| 671: ; preds = %667, %662
 25154|  %672 = phi i64 [ %668, %667 ], [ %663, %662 ]                                                                         ;L1434<759
 25155|     ;; self = ptr %24
 25156|  %673 = load ptr, ptr %24, , !!38311, !!8, !!8                                                                         ;L138<1432<759
 25157|     ;; self = ptr %673
 25158|     ;; count = i64 %672
 25159|  %674 = gepS %673, i64 %672                                                                                            ;L961<1432<759
 25160|     ;; end = ptr %674
 25161|     ;; dst = ptr %674
 25162|  call void @llvm.memcpy.p0.p0.i64(ptr %674, ptr %15, i64 184, i1 false)                                                ;L1933<1433<759
 25163|  %675 = add i64 %672, 1                                                                                                ;L1434<759
 25164|  store i64 %675, ptr %82, , !!38311                                                                                    ;L1434<759
 25166|  br label %676                                                                                                         ;L727
 25167| 
 25168| 676: ; preds = %671, %661, %609, %565, %557
 25169|  br label %545                                                                                                         ;L1714<180<727
 25170| 
 25171| 677: ; preds = %545
 25172|  %678 = load ptr, ptr %96, , !!8, !!8                                                                                  ;L762
 25173|  %679 = gep %100, i64 8                                                                                                ;L762
 25174|  %680 = load ptr, ptr %679, , !!8, !!8                                                                                 ;L762
 25177|     ;; champ = ptr %55
 25181|  %681 = gep %680, i64 16                                                                                               ;L2445<1183<762
 25182|  %682 = load i64, ptr %681, , !!38334                                                                                  ;L2445<1183<762
 25183|  %683 = add nsw i64 %682, -1                                                                                           ;L2445<1183<762
 25184|  %684 = and i64 %683, -16                                                                                              ;L2445<1183<762
 25185|  %685 = gep %678, i64 %684                                                                                             ;L2445<1183<762
 25186|  %686 = gep %685, i64 16                                                                                               ;L2445<1183<762
 25187|  %687 = gep %680, i64 160                                                                                              ;L1183<762
 25188|  %688 = load ptr, ptr %687, , !!38334, !!8                                                                             ;L1183<762
 25189|  invoke void %688(ptr sret([288 x i8]) %8, ptr %686, ptr %62, ptr %55, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.56)
 25190|  to label %689 unwind label %105                                                                                       ;L1183<762
 25191| 
 25192| 689: ; preds = %677
 25193|     ;; self = ptr %8
 25194|  %690 = gep %8, i64 72                                                                                                 ;L633<1183<762
 25195|  %691 = load i32, ptr %690, , !!38334, !!8                                                                             ;L633<1183<762
 25196|  %692 = icmp eq i32 %691, -1                                                                                           ;L633<1183<762
 25198|  %693 = select i1 %692, i64 30, i64 90                                                                                 ;L1183<762
 25199|     ;; walk = i64 %693
 25200|     ;; self = ptr %28
 25201|     ;; self = ptr %28
 25202|     ;; self = ptr %28
 25203|  %694 = load ptr, ptr %28, , !!8, !!8                                                                                  ;L138<2073<2136<763
 25204|     ;; p = ptr %694
 25205|  %695 = gep %28, i64 24                                                                                                ;L2075<2136<763
 25206|  %696 = load i64, ptr %695, , !!8                                                                                      ;L2075<2136<763
 25207|     ;; len = i64 %696
 25208|     ;; count = i64 %696
 25209|     ;; self[0..+8] = ptr %694
 25210|     ;; slice[0..+8] = ptr %694
 25211|     ;; self[8..+8] = i64 %696
 25212|     ;; slice[8..+8] = i64 %696
 25213|     ;; ptr = ptr %694
 25214|     ;; self = ptr %694
 25215|  %697 = getelementptr ptr, ptr %694, i64 %696                                                                          ;L961<100<1042<2136<763
 25216|     ;; iter[0..+8] = ptr %694
 25217|     ;; iter[8..+8] = ptr %697
 25218|  %698 = mul i64 %693, %102
 25219|  %699 = gep %13, i64 177
 25220|  br label %700                                                                                                         ;L763
 25221| 
 25222| 700: ; preds = %786, %689
 25223|  %701 = phi ptr [ %694, %689 ], [ %704, %786 ]                                                                         ;L763
 25224|     ;; iter[0..+8] = ptr %701
 25225|     ;; self = ptr undef
 25226|     ;; ptr = ptr %701
 25227|     ;; self = ptr %701
 25228|     ;; end_or_len = ptr %697
 25231|  %702 = icmp eq ptr %701, %697                                                                                         ;L1714<180<763
 25232|  br i1 %702, label %787, label %703                                                                                    ;L180<763
 25233| 
 25234| 703: ; preds = %700
 25235|  %704 = gep %701, i64 8                                                                                                ;L656<185<763
 25236|     ;; iter[0..+8] = ptr %704
 25237|     ;; e = ptr %701
 25238|  %705 = load ptr, ptr %701, , !!8, !!8                                                                                 ;L764
 25239|  %706 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %534, ptr %55, ptr %705)
 25240|  to label %707 unwind label %105                                                                                       ;L764
 25241| 
 25242| 707: ; preds = %703
 25243|  br i1 %706, label %708, label %786                                                                                    ;L764
 25244| 
 25245| 708: ; preds = %707
 25246|     ;; self = ptr %100
 25247|  %709 = load i64, ptr %535, , !!8                                                                                      ;L26<768
 25248|  %710 = load i64, ptr %536, , !!8                                                                                      ;L26<768
 25249|  %711 = load i64, ptr %537, , !!8                                                                                      ;L26<768
 25250|  %712 = load ptr, ptr %701, , !!8, !!8                                                                                 ;L768
 25251|  %713 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %96, ptr %55, ptr %712)
 25252|  to label %714 unwind label %105                                                                                       ;L768
 25253| 
 25254| 714: ; preds = %708
 25255|  %715 = mul i64 %710, %538                                                                                             ;L26<768
 25256|  %716 = load i32, ptr %539, , !!8                                                                                      ;L1511<768
 25257|     ;; mult = i32 %716
 25258|  %717 = icmp eq i32 %716, 0                                                                                            ;L1512<768
 25259|  br i1 %717, label %718, label %720                                                                                    ;L1512<768
 25260| 
 25261| 718: ; preds = %714
 25262|  %719 = load i64, ptr %540, , !!8                                                                                      ;L1513<768
 25263|  br label %726                                                                                                         ;L1512<768
 25264| 
 25265| 720: ; preds = %714
 25266|  %721 = sext i32 %716 to i64                                                                                           ;L1511<768
 25267|     ;; mult = i64 %721
 25268|  %722 = load i64, ptr %540, , !!8                                                                                      ;L1515<768
 25269|  %723 = add nsw i64 %721, 100                                                                                          ;L1515<768
 25270|  %724 = mul i64 %722, %723                                                                                             ;L1515<768
 25271|  %725 = udiv i64 %724, 100                                                                                             ;L1515<768
 25272|  br label %726                                                                                                         ;L1512<768
 25273| 
 25274| 726: ; preds = %720, %718
 25275|  %727 = phi i64 [ %719, %718 ], [ %725, %720 ]                                                                         ;L0<768
 25276|  %728 = load ptr, ptr %701, , !!8, !!8                                                                                 ;L768
 25277|     ;; self = ptr %728
 25278|  %729 = gep %728, i64 1136                                                                                             ;L1511<768
 25279|  %730 = load i32, ptr %729, , !!8                                                                                      ;L1511<768
 25280|     ;; mult = i32 %730
 25281|  %731 = icmp eq i32 %730, 0                                                                                            ;L1512<768
 25282|  br i1 %731, label %732, label %735                                                                                    ;L1512<768
 25283| 
 25284| 732: ; preds = %726
 25285|  %733 = gep %728, i64 1664                                                                                             ;L1513<768
 25286|  %734 = load i64, ptr %733, , !!8                                                                                      ;L1513<768
 25287|  br label %742                                                                                                         ;L1512<768
 25288| 
 25289| 735: ; preds = %726
 25290|  %736 = sext i32 %730 to i64                                                                                           ;L1511<768
 25291|     ;; mult = i64 %736
 25292|  %737 = gep %728, i64 1664                                                                                             ;L1515<768
 25293|  %738 = load i64, ptr %737, , !!8                                                                                      ;L1515<768
 25294|  %739 = add nsw i64 %736, 100                                                                                          ;L1515<768
 25295|  %740 = mul i64 %738, %739                                                                                             ;L1515<768
 25296|  %741 = udiv i64 %740, 100                                                                                             ;L1515<768
 25297|  br label %742                                                                                                         ;L1512<768
 25298| 
 25299| 742: ; preds = %735, %732
 25300|  %743 = phi i64 [ %734, %732 ], [ %741, %735 ]                                                                         ;L0<768
 25302|     ;; self = ptr %728
 25303|  %744 = gep %728, i64 1632                                                                                             ;L2158<769
 25304|  %745 = load i64, ptr %744, , !!8                                                                                      ;L2158<769
 25305|     ;; x1 = i64 %745
 25306|     ;; self = i64 %745
 25307|  %746 = gep %728, i64 1640                                                                                             ;L2158<769
 25308|  %747 = load i64, ptr %746, , !!8                                                                                      ;L2158<769
 25309|     ;; y1 = i64 %747
 25310|     ;; self = i64 %747
 25311|  %748 = load i64, ptr %541, , !!8                                                                                      ;L2158<769
 25312|     ;; x2 = i64 %748
 25313|     ;; other = i64 %748
 25314|  %749 = load i64, ptr %542, , !!8                                                                                      ;L2158<769
 25315|     ;; y2 = i64 %749
 25316|     ;; other = i64 %749
 25317|  %750 = icmp ult i64 %745, %748                                                                                        ;L3147<7<2158<769
 25318|  %751 = sub nuw i64 %748, %745                                                                                         ;L3147<7<2158<769
 25319|  %752 = sub nuw i64 %745, %748                                                                                         ;L3147<7<2158<769
 25320|  %753 = select i1 %750, i64 %751, i64 %752                                                                             ;L3147<7<2158<769
 25321|     ;; dx = i64 %753
 25322|  %754 = icmp ult i64 %747, %749                                                                                        ;L3147<8<2158<769
 25323|  %755 = sub nuw i64 %749, %747                                                                                         ;L3147<8<2158<769
 25324|  %756 = sub nuw i64 %747, %749                                                                                         ;L3147<8<2158<769
 25325|  %757 = select i1 %754, i64 %755, i64 %756                                                                             ;L3147<8<2158<769
 25326|     ;; dy = i64 %757
 25327|  %758 = mul i64 %753, %753                                                                                             ;L9<2158<769
 25328|  %759 = mul i64 %757, %757                                                                                             ;L9<2158<769
 25329|  %760 = add i64 %759, %758                                                                                             ;L9<2158<769
 25330|     ;; dist_sq = i64 %760
 25331|  %761 = add i64 %709, %698                                                                                             ;L26<768
 25332|  %762 = add i64 %761, %715                                                                                             ;L26<768
 25333|  %763 = add i64 %762, %711                                                                                             ;L768
 25334|  %764 = add i64 %763, %713                                                                                             ;L768
 25335|  %765 = add i64 %764, %727                                                                                             ;L768
 25336|  %766 = add i64 %765, %743                                                                                             ;L772
 25337|     ;; max_dist = i64 %766
 25338|  %767 = mul i64 %766, %766                                                                                             ;L773
 25339|  %768 = icmp ugt i64 %760, %767                                                                                        ;L773
 25340|  br i1 %768, label %786, label %769                                                                                    ;L773
 25341| 
 25342| 769: ; preds = %742
 25345|  %770 = gep %728, i64 1472                                                                                             ;L777
 25346|  %771 = load i64, ptr %770, , !!8                                                                                      ;L777
 25347|  invoke void @ai::small_action4castNtB5_17SmallActionSkill23new(ptr sret([24 x i8]) %12, ptr %4, i64 %771)
 25348|  to label %772 unwind label %105                                                                                       ;L777
 25349| 
 25350| 772: ; preds = %769
 25351|  call void @llvm.memcpy.p0.p0.i64(ptr %13, ptr %12, i64 24, i1 false)                                                  ;L777
 25352|  store i8 17, ptr %699,                                                                                                ;L777
 25354|     ;; self = ptr %24
 25355|     ;; self = ptr %24
 25356|     ;; value = ptr %13
 25357|     ;; src = ptr %13
 25358|     ;; additional = i64 1
 25359|     ;; needed_extra_cap = i64 1
 25360|     ;; needed_extra_cap = i64 1
 25361|     ;; strategy = i8 1
 25362|  %773 = load i64, ptr %82, , !!38421, !!8                                                                              ;L1428<777
 25363|     ;; self = ptr %24
 25364|  %774 = load i64, ptr %81, , !!38421, !!8                                                                              ;L149<1428<777
 25365|  %775 = icmp eq i64 %773, %774                                                                                         ;L1428<777
 25366|  br i1 %775, label %776, label %781                                                                                    ;L1428<777
 25367| 
 25368| 776: ; preds = %772
 25369|     ;; self = ptr %24
 25370|     ;; self = ptr %24
 25371|     ;; self = ptr %24
 25372|     ;; used_cap = i64 %773
 25373|     ;; used_cap = i64 %773
 25374|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %24, i64 %773, i64 1, i1 zeroext true)
 25375|  to label %777 unwind label %779, !!38421                                                                              ;L619<430<738<1429<777
 25376| 
 25377| 777: ; preds = %776
 25378|  %778 = load i64, ptr %82, , !!38421                                                                                   ;L1432<777
 25379|  br label %781                                                                                                         ;L619<430<738<1429<777
 25380| 
 25381| 779: ; preds = %776
 25382|  %780 = cleanuppad within none []
 25383|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %13) #32 [ "funclet"(token %780) ] ;L1436<777
 25384|  cleanupret from %780 unwind label %105
 25385| 
 25386| 781: ; preds = %777, %772
 25387|  %782 = phi i64 [ %778, %777 ], [ %773, %772 ]                                                                         ;L1434<777
 25388|     ;; self = ptr %24
 25389|  %783 = load ptr, ptr %24, , !!38421, !!8, !!8                                                                         ;L138<1432<777
 25390|     ;; self = ptr %783
 25391|     ;; count = i64 %782
 25392|  %784 = gepS %783, i64 %782                                                                                            ;L961<1432<777
 25393|     ;; end = ptr %784
 25394|     ;; dst = ptr %784
 25395|  call void @llvm.memcpy.p0.p0.i64(ptr %784, ptr %13, i64 184, i1 false)                                                ;L1933<1433<777
 25396|  %785 = add i64 %782, 1                                                                                                ;L1434<777
 25397|  store i64 %785, ptr %82, , !!38421                                                                                    ;L1434<777
 25399|  br label %786                                                                                                         ;L763
 25400| 
 25401| 786: ; preds = %781, %742, %707
 25402|  br label %700                                                                                                         ;L1714<180<763
 25403| 
 25404| 787: ; preds = %700
 25405|  %788 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %534, ptr %55, ptr %55)
 25406|  to label %789 unwind label %105                                                                                       ;L780
 25407| 
 25408| 789: ; preds = %787
 25409|  br i1 %788, label %790, label %523                                                                                    ;L780
 25410| 
 25411| 790: ; preds = %789
 25412|  %791 = load ptr, ptr %96,                                                                                             ;L780
 25413|  %792 = load ptr, ptr %679,                                                                                            ;L780
 25415|     ;; player = ptr %3
 25416|     ;; data = ptr %4
 25417|     ;; champ = ptr %55
 25418|     ;; self = ptr %55
 25420|     ;; concrete = ptr %7
 25422|  %793 = load i64, ptr %92, , !!38497, !!8                                                                              ;L1669<613<780
 25423|  %794 = icmp ugt i64 %793, 2                                                                                           ;L1669<613<780
 25424|  %795 = select i1 %794, i64 1424, i64 1456                                                                             ;L1669<613<780
 25425|  %796 = gep %55, i64 %795                                                                                              ;L1669<613<780
 25426|     ;; skill2_action = ptr %796
 25427|  %797 = load ptr, ptr %796, , !!38497, !!8, !!8                                                                        ;L614<780
 25428|  %798 = gep %796, i64 8                                                                                                ;L614<780
 25429|  %799 = load ptr, ptr %798, , !!38497, !!8, !!8                                                                        ;L614<780
 25430|  %800 = gep %799, i64 104                                                                                              ;L614<780
 25431|  %801 = load ptr, ptr %800, , !!38502, !!8                                                                             ;L614<780
 25432|  %802 = invoke { ptr, ptr } %801(ptr %797)
 25433|  to label %803 unwind label %105                                                                                       ;L614<780
 25434| 
 25435| 803: ; preds = %790
 25436|  %804 = extractvalue { ptr, ptr } %802, 0                                                                              ;L614<780
 25437|  %805 = extractvalue { ptr, ptr } %802, 1                                                                              ;L614<780
 25438|     ;; self[0..+8] = ptr %804
 25439|     ;; self[0..+8] = ptr %804
 25440|     ;; self[8..+8] = ptr %805
 25441|     ;; self[8..+8] = ptr %805
 25443|  %806 = gep %805, i64 24                                                                                               ;L201<229<614<780
 25444|  %807 = load ptr, ptr %806, , !!38502, !!8                                                                             ;L201<229<614<780
 25445|  invoke void %807(ptr sret([16 x i8]) %7, ptr %804)
 25446|  to label %808 unwind label %105                                                                                       ;L201<229<614<780
 25447| 
 25448| 808: ; preds = %803
 25451|     ;; other = ptr %7
 25452|  %809 = load i128, ptr %7, , !!38502, !!8                                                                              ;L764<2450<204<229<614<780
 25453|  %810 = icmp eq i128 %809, 168406848281932906149591046147716593956                                                     ;L764<2450<204<229<614<780
 25455|  br i1 %810, label %815, label %811                                                                                    ;L229<614<780
 25456| 
 25457| 811: ; preds = %808
 25458|  %812 = icmp ne ptr %791, null
 25459|  call void @llvm.assume(i1 %812)
 25460|  %813 = icmp ne ptr %792, null
 25461|  call void @llvm.assume(i1 %813)
 25462|  %814 = invoke fastcc zeroext i1 @ai::fight_check31should_add_self_etc_buff_action(ptr %3, ptr %4, ptr %55, ptr %791, ptr %792)
 25463|  to label %821 unwind label %105                                                                                       ;L617<780
 25464| 
 25465| 815: ; preds = %808
 25466|  %816 = icmp ne ptr %804, null
 25467|  call void @llvm.assume(i1 %816)
 25468|     ;; action = ptr %804
 25469|  %817 = load ptr, ptr %51, , !!38502, !!8, !!8                                                                         ;L615<780
 25470|  %818 = gep %51, i64 8                                                                                                 ;L615<780
 25471|  %819 = load ptr, ptr %818, , !!38502, !!8, !!8                                                                        ;L615<780
 25472|  %820 = invoke zeroext i1 @gc::setting8champion8prisonerNtB5_20PrisonerSkill2Action42has_enemy_champion_target_or_action_threat(ptr %804, ptr %817, ptr %819, ptr %55)
 25473|  to label %821 unwind label %105                                                                                       ;L615<780
 25474| 
 25475| 821: ; preds = %815, %811
 25476|  %822 = phi i1 [ %814, %811 ], [ %820, %815 ]
 25477|  br i1 %822, label %823, label %523                                                                                    ;L780
 25478| 
 25479| 823: ; preds = %821
 25482|  %824 = gep %55, i64 1472                                                                                              ;L781
 25483|  %825 = load i64, ptr %824, , !!8                                                                                      ;L781
 25484|  invoke void @ai::small_action4castNtB5_17SmallActionSkill23new(ptr sret([24 x i8]) %10, ptr %4, i64 %825)
 25485|  to label %826 unwind label %105                                                                                       ;L781
 25486| 
 25487| 826: ; preds = %823
 25488|  call void @llvm.memcpy.p0.p0.i64(ptr %11, ptr %10, i64 24, i1 false)                                                  ;L781
 25489|  %827 = gep %11, i64 177                                                                                               ;L781
 25490|  store i8 17, ptr %827,                                                                                                ;L781
 25492|     ;; self = ptr %24
 25493|     ;; self = ptr %24
 25494|     ;; value = ptr %11
 25495|     ;; src = ptr %11
 25496|     ;; additional = i64 1
 25497|     ;; needed_extra_cap = i64 1
 25498|     ;; needed_extra_cap = i64 1
 25499|     ;; strategy = i8 1
 25500|  %828 = load i64, ptr %82, , !!38547, !!8                                                                              ;L1428<781
 25501|     ;; self = ptr %24
 25502|  %829 = load i64, ptr %81, , !!38547, !!8                                                                              ;L149<1428<781
 25503|  %830 = icmp eq i64 %828, %829                                                                                         ;L1428<781
 25504|  br i1 %830, label %831, label %836                                                                                    ;L1428<781
 25505| 
 25506| 831: ; preds = %826
 25507|     ;; self = ptr %24
 25508|     ;; self = ptr %24
 25509|     ;; self = ptr %24
 25510|     ;; used_cap = i64 %828
 25511|     ;; used_cap = i64 %828
 25512|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %24, i64 %828, i64 1, i1 zeroext true)
 25513|  to label %832 unwind label %834, !!38547                                                                              ;L619<430<738<1429<781
 25514| 
 25515| 832: ; preds = %831
 25516|  %833 = load i64, ptr %82, , !!38547                                                                                   ;L1432<781
 25517|  br label %836                                                                                                         ;L619<430<738<1429<781
 25518| 
 25519| 834: ; preds = %831
 25520|  %835 = cleanuppad within none []
 25521|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %11) #32 [ "funclet"(token %835) ] ;L1436<781
 25522|  cleanupret from %835 unwind label %105
 25523| 
 25524| 836: ; preds = %832, %826
 25525|  %837 = phi i64 [ %833, %832 ], [ %828, %826 ]                                                                         ;L1434<781
 25526|     ;; self = ptr %24
 25527|  %838 = load ptr, ptr %24, , !!38547, !!8, !!8                                                                         ;L138<1432<781
 25528|     ;; self = ptr %838
 25529|     ;; count = i64 %837
 25530|  %839 = gepS %838, i64 %837                                                                                            ;L961<1432<781
 25531|     ;; end = ptr %839
 25532|     ;; dst = ptr %839
 25533|  call void @llvm.memcpy.p0.p0.i64(ptr %839, ptr %11, i64 184, i1 false)                                                ;L1933<1433<781
 25534|  %840 = add i64 %837, 1                                                                                                ;L1434<781
 25535|  store i64 %840, ptr %82, , !!38547                                                                                    ;L1434<781
 25537|  br label %523                                                                                                         ;L780
 25538| 
 25539| 841: ; preds = %527
 25542|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %28)
 25543|  to label %845 unwind label %842                                                                                       ;L825<786
 25544| 
 25545| 842: ; preds = %841
 25546|  %843 = cleanuppad within none []
 25548|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %28) [ "funclet"(token %843) ]
 25549|  to label %844 unwind label %44                                                                                        ;L825<825<786
 25550| 
 25551| 844: ; preds = %842
 25552|  cleanupret from %843 unwind label %44
 25553| 
 25554| 845: ; preds = %841
 25556|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %28)
 25557|  to label %846 unwind label %44                                                                                        ;L825<825<786
 25558| 
 25559| 846: ; preds = %845
 25563|  %847 = load i32, ptr %39, , !!8                                                                                       ;L825<786
 25564|  %848 = icmp eq i32 %847, -1                                                                                           ;L825<786
 25565|  br i1 %848, label %866, label %849                                                                                    ;L825<786
 25566| 
 25567| 849: ; preds = %846
 25571|     ;; self = ptr %29
 25572|     ;; order = i8 0
 25573|     ;; order = i8 0
 25574|     ;; val = i64 1
 25575|     ;; order = i8 0
 25576|     ;; val = i64 1
 25577|     ;; order = i8 0
 25578|  %850 = load i64, ptr %29, , !!8                                                                                       ;L185<825<825<786
 25579|  %851 = icmp ult i64 %850, 132                                                                                         ;L185<825<825<786
 25580|  br i1 %851, label %853, label %852                                                                                    ;L185<825<825<786
 25581| 
 25582| 852: ; preds = %849
 25583|  call void @core::panicking18panic_bounds_check(i64 %850, i64 132, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.258) #30, !!38601 ;L185<825<825<786
 25584|  unreachable                                                                                                           ;L185<825<825<786
 25585| 
 25586| 853: ; preds = %849
 25587|  %854 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %850                                 ;L185<825<825<786
 25588|     ;; self = ptr %854
 25589|  %855 = gep %29, i64 8                                                                                                 ;L185<825<825<786
 25590|  %856 = call { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %855)                               ;L185<825<825<786
 25591|  %857 = extractvalue { i64, i32 } %856, 0                                                                              ;L185<825<825<786
 25592|  %858 = extractvalue { i64, i32 } %856, 1                                                                              ;L185<825<825<786
 25594|  %859 = mul i64 %857, 1000000000                                                                                       ;L632<185<825<825<786
 25595|  %860 = icmp ult i32 %858, 1000000000                                                                                  ;L49<632<185<825<825<786
 25596|  call void @llvm.assume(i1 %860)                                                                                       ;L49<632<185<825<825<786
 25597|  %861 = zext nneg i32 %858 to i64                                                                                      ;L632<185<825<825<786
 25598|  %862 = add i64 %859, %861                                                                                             ;L632<185<825<825<786
 25599|     ;; val = i64 %862
 25600|     ;; val = i64 %862
 25601|     ;; dst = ptr %854
 25602|  %863 = atomicrmw add ptr %854, i64 %862 monotonic, , !!38601                                                          ;L3937<3162<185<825<825<786
 25603|  %864 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %850                                 ;L186<825<825<786
 25604|     ;; self = ptr %864
 25605|     ;; dst = ptr %864
 25606|  %865 = atomicrmw add ptr %864, i64 1 monotonic, , !!38601                                                             ;L3937<3162<186<825<825<786
 25607|  br label %866                                                                                                         ;L825<786
 25608| 
 25609| 866: ; preds = %853, %846
 25611|  ret void                                                                                                              ;L786
 25612| }
