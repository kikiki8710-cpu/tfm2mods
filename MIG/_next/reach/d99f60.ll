 52737| define zeroext i1 @ai::tower_discipline41v30_line_champion_action_tower_aggro_risk(i64 %0, ptr %1, ptr %2, ptr %3) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 52738|  %5 = alloca [32 x i8],
 52739|  %6 = alloca [120 x i8],
 52740|  %7 = alloca [8 x i8],
 52741|  %8 = alloca [8 x i8],
 52743|     ;; version = i64 %0
 52744|     ;; data = ptr %1
 52745|     ;; player = ptr %2
 52746|     ;; action = ptr %3
 52747|     ;; stance_x = ptr %8
 52748|     ;; stance_y = ptr %7
 52752|  %9 = gep %1, i64 8                                                                                                    ;L160
 52753|  %10 = load ptr, ptr %9, , !!8, !!8                                                                                    ;L160
 52754|     ;; self = ptr %10
 52755|  %11 = load ptr, ptr %1, , !!8, !!8                                                                                    ;L160
 52756|  %12 = load ptr, ptr %11, , !!8, !!8                                                                                   ;L160
 52757|  %13 = gep %11, i64 8                                                                                                  ;L160
 52758|  %14 = load ptr, ptr %13, , !!8, !!8                                                                                   ;L160
 52759|  %15 = gep %14, i64 40                                                                                                 ;L160
 52760|  %16 = load ptr, ptr %15, , !!8                                                                                        ;L160
 52761|  %17 = tail call i64 %16(ptr %12)                                                                                      ;L160
 52762|     ;; tick = i64 %17
 52763|     ;; tick = i64 %17
 52764|     ;; self = ptr %10
 52765|  %18 = gep %10, i64 56                                                                                                 ;L263<399<160
 52766|  %19 = load i8, ptr %18, , !!8                                                                                         ;L263<399<160
 52767|  switch i8 %19, label %30 [
 52768|  i8 0, label %20
 52769|  i8 7, label %20
 52770|  i8 8, label %20
 52771|  i8 5, label %20
 52772|  ]                                                                                                                     ;L263<399<160
 52773| 
 52774| 20: ; preds = %4, %4, %4, %4
 52775|  %21 = gep %10, i64 8                                                                                                  ;L399<160
 52776|  %22 = load ptr, ptr %21, , !!8, !!8                                                                                   ;L399<160
 52777|     ;; self = ptr %22
 52778|  %23 = gep %22, i64 2216                                                                                               ;L703<399<160
 52779|  %24 = load i64, ptr %23, , !!8                                                                                        ;L703<399<160
 52780|     ;; self = i64 %24
 52781|  %25 = gep %22, i64 4856                                                                                               ;L704<399<160
 52782|  %26 = load i64, ptr %25, , !!8                                                                                        ;L704<399<160
 52783|  %27 = mul i64 %26, 30                                                                                                 ;L704<399<160
 52784|     ;; rhs = i64 %27
 52785|  %28 = tail call i64 @llvm.usub.sat.i64(i64 %24, i64 %27)                                                              ;L2472<703<399<160
 52786|  %29 = icmp ult i64 %17, %28                                                                                           ;L703<399<160
 52787|  br i1 %29, label %30, label %135                                                                                      ;L160
 52788| 
 52789| 30: ; preds = %20, %4
 52790|  %31 = tail call i64 %16(ptr %12)                                                                                      ;L161
 52791|  %32 = gep %10, i64 8                                                                                                  ;L161
 52792|  %33 = load ptr, ptr %32, , !!8, !!8                                                                                   ;L161
 52793|  %34 = gep %33, i64 5112                                                                                               ;L161
 52794|  %35 = load i64, ptr %34, , !!8                                                                                        ;L161
 52795|  %36 = icmp ult i64 %31, %35                                                                                           ;L161
 52796|  br i1 %36, label %37, label %135                                                                                      ;L161
 52797| 
 52798| 37: ; preds = %30
 52799|  %38 = gep %2, i64 2352                                                                                                ;L165
 52800|  %39 = load i64, ptr %38, , !!8                                                                                        ;L165
 52801|  %40 = icmp ult i64 %39, 2                                                                                             ;L165
 52802|  br i1 %40, label %42, label %41                                                                                       ;L165
 52803| 
 52804| 41: ; preds = %37
 52805|  tail call void @core::panicking18panic_bounds_check(i64 %39, i64 2, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.181) #25 ;L165
 52806|  unreachable                                                                                                           ;L165
 52807| 
 52808| 42: ; preds = %37
 52809|     ;; self = ptr %2
 52810|  %43 = gep %2, i64 2496                                                                                                ;L581<165
 52811|  %44 = load i32, ptr %43, , !!8                                                                                        ;L581<165
 52812|  %45 = zext nneg i32 %44 to i64                                                                                        ;L581<165
 52813|  %46 = gep %11, i64 480                                                                                                ;L165
 52814|  %47 = getelementptr [5 x ptr], ptr %46, i64 %39                                                                       ;L165
 52815|  %48 = getelementptr ptr, ptr %47, i64 %45                                                                             ;L165
 52816|  %49 = load ptr, ptr %48, , !!8                                                                                        ;L165
 52817|  %50 = icmp eq ptr %49, null                                                                                           ;L165
 52818|  br i1 %50, label %135, label %51                                                                                      ;L165
 52819| 
 52820| 51: ; preds = %42
 52821|     ;; champ = ptr %49
 52822|     ;; self = ptr %3
 52823|  %52 = gep %3, i64 177                                                                                                 ;L309<169
 52824|  %53 = load i8, ptr %52, , !!64931, !!8                                                                                ;L309<169
 52825|  %54 = icmp ne i8 %53, 10                                                                                              ;L309<169
 52826|  tail call void @llvm.assume(i1 %54)                                                                                   ;L309<169
 52827|  %55 = add nsw i8 %53, -3                                                                                              ;L309<169
 52828|  %56 = icmp samesign ugt i8 %53, 2                                                                                     ;L309<169
 52829|  %57 = select i1 %56, i8 %55, i8 7                                                                                     ;L309<169
 52830|  switch i8 %57, label %58 [
 52831|  i8 0, label %135
 52832|  i8 1, label %135
 52833|  i8 2, label %135
 52834|  i8 3, label %135
 52835|  i8 4, label %135
 52836|  i8 5, label %135
 52837|  i8 6, label %135
 52838|  i8 7, label %135
 52839|  i8 8, label %135
 52840|  i8 9, label %135
 52841|  i8 10, label %135
 52842|  i8 11, label %135
 52843|  i8 12, label %62
 52844|  i8 13, label %59
 52845|  i8 14, label %60
 52846|  i8 15, label %61
 52847|  i8 16, label %135
 52848|  ]                                                                                                                     ;L309<169
 52849| 
 52850| 58: ; preds = %51
 52851|  unreachable                                                                                                           ;L309<169
 52852| 
 52853| 59: ; preds = %51
 52855|     ;; small_action[0..+8] = i64 7
 52856|     ;; self = ptr undef
 52857|  br label %62                                                                                                          ;L104<170
 52858| 
 52859| 60: ; preds = %51
 52861|     ;; small_action[0..+8] = i64 8
 52862|     ;; self = ptr undef
 52863|  br label %62                                                                                                          ;L105<170
 52864| 
 52865| 61: ; preds = %51
 52867|     ;; small_action[0..+8] = i64 9
 52868|     ;; self = ptr undef
 52869|  br label %62                                                                                                          ;L106<170
 52870| 
 52871| 62: ; preds = %61, %60, %59, %51
 52872|  %63 = phi i64 [ 9, %61 ], [ 8, %60 ], [ 7, %59 ], [ 6, %51 ]
 52873|  %64 = gep %3, i64 8                                                                                                   ;L0<169
 52874|  %65 = load i64, ptr %64, , !!64931, !!8                                                                               ;L0<169
 52875|     ;; target_id = i64 %65
 52876|  %66 = gep %14, i64 496                                                                                                ;L173
 52877|  %67 = load ptr, ptr %66, , !!8                                                                                        ;L173
 52878|  %68 = tail call ptr %67(ptr %12, i64 %65)                                                                             ;L173
 52879|  %69 = icmp eq ptr %68, null                                                                                           ;L173
 52880|  br i1 %69, label %135, label %70                                                                                      ;L173
 52881| 
 52882| 70: ; preds = %62
 52883|     ;; target = ptr %68
 52884|     ;; self = ptr %68
 52885|     ;; other = ptr %49
 52886|  %71 = load i64, ptr %68, , !!8                                                                                        ;L1127<176
 52887|  %72 = gep %68, i64 8                                                                                                  ;L1127<176
 52888|     ;; __self_discr = i64 %71
 52889|  %73 = load i64, ptr %49, , !!8                                                                                        ;L1127<176
 52890|  %74 = gep %49, i64 8                                                                                                  ;L1127<176
 52891|     ;; __arg1_discr = i64 %73
 52892|  %75 = icmp eq i64 %71, %73                                                                                            ;L1127<176
 52893|  br i1 %75, label %76, label %78                                                                                       ;L1127<176
 52894| 
 52895| 76: ; preds = %70
 52896|  %77 = icmp eq i64 %71, 0                                                                                              ;L1127<176
 52897|  br i1 %77, label %82, label %135                                                                                      ;L1127<176
 52898| 
 52899| 78: ; preds = %82, %70
 52900|     ;; self = ptr %68
 52901|  %79 = gep %68, i64 104                                                                                                ;L1404<176
 52902|  %80 = load i64, ptr %79, , !!8                                                                                        ;L1404<176
 52903|  %81 = icmp eq i64 %80, 13                                                                                             ;L176
 52904|  br i1 %81, label %86, label %135                                                                                      ;L176
 52905| 
 52906| 82: ; preds = %76
 52907|     ;; __self_0 = ptr %68
 52908|     ;; self = ptr %68
 52909|     ;; __arg1_0 = ptr %49
 52910|     ;; other = ptr %49
 52913|  %83 = load i64, ptr %72, , !!8                                                                                        ;L1878<2123<1127<176
 52914|  %84 = load i64, ptr %74, , !!8                                                                                        ;L1878<2123<1127<176
 52915|  %85 = icmp eq i64 %83, %84                                                                                            ;L1878<2123<1127<176
 52916|  br i1 %85, label %135, label %78                                                                                      ;L176
 52917| 
 52918| 86: ; preds = %78
 52919|     ;; champ = ptr %49
 52920|     ;; self = ptr %49
 52921|     ;; self = ptr %49
 52923|  switch i64 %63, label %134 [
 52924|  i64 6, label %87
 52925|  i64 7, label %92
 52926|  i64 8, label %97
 52927|  i64 9, label %106
 52928|  ]                                                                                                                     ;L123<180
 52929| 
 52930| 87: ; preds = %86
 52931|     ;; self = ptr %49
 52932|  %88 = gep %49, i64 1216                                                                                               ;L742<124<180
 52933|  %89 = load i32, ptr %88, , !!8                                                                                        ;L742<124<180
 52934|  %90 = icmp eq i32 %89, -1                                                                                             ;L742<124<180
 52935|  %91 = gep %49, i64 1168
 52936|  br i1 %90, label %135, label %115                                                                                     ;L742<124<180
 52937| 
 52938| 92: ; preds = %86
 52939|     ;; self = ptr %49
 52940|  %93 = gep %49, i64 1272                                                                                               ;L742<125<180
 52941|  %94 = load i32, ptr %93, , !!8                                                                                        ;L742<125<180
 52942|  %95 = icmp eq i32 %94, -1                                                                                             ;L742<125<180
 52943|  %96 = gep %49, i64 1224
 52944|  br i1 %95, label %135, label %115                                                                                     ;L742<125<180
 52945| 
 52946| 97: ; preds = %86
 52947|  %98 = gep %49, i64 1480                                                                                               ;L1693<126<180
 52948|  %99 = load i64, ptr %98, , !!8                                                                                        ;L1693<126<180
 52949|  %100 = icmp ugt i64 %99, 2                                                                                            ;L1693<126<180
 52950|  %101 = gep %49, i64 1280                                                                                              ;L1693<126<180
 52951|  %102 = select i1 %100, ptr %101, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                        ;L1693<126<180
 52952|     ;; self = ptr %102
 52953|  %103 = gep %102, i64 48                                                                                               ;L742<126<180
 52954|  %104 = load i32, ptr %103, , !!8                                                                                      ;L742<126<180
 52955|  %105 = icmp eq i32 %104, -1                                                                                           ;L742<126<180
 52956|  br i1 %105, label %135, label %115                                                                                    ;L742<126<180
 52957| 
 52958| 106: ; preds = %86
 52959|  %107 = gep %49, i64 1480                                                                                              ;L1701<127<180
 52960|  %108 = load i64, ptr %107, , !!8                                                                                      ;L1701<127<180
 52961|  %109 = icmp ugt i64 %108, 4                                                                                           ;L1701<127<180
 52962|  %110 = gep %49, i64 1336                                                                                              ;L1701<127<180
 52963|  %111 = select i1 %109, ptr %110, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                        ;L1701<127<180
 52964|     ;; self = ptr %111
 52965|  %112 = gep %111, i64 48                                                                                               ;L742<127<180
 52966|  %113 = load i32, ptr %112, , !!8                                                                                      ;L742<127<180
 52967|  %114 = icmp eq i32 %113, -1                                                                                           ;L742<127<180
 52968|  br i1 %114, label %135, label %115                                                                                    ;L742<127<180
 52969| 
 52970| 115: ; preds = %106, %97, %92, %87
 52971|  %116 = phi ptr [ %96, %92 ], [ %111, %106 ], [ %91, %87 ], [ %102, %97 ]                                              ;L0<180
 52972|     ;; effect = ptr %116
 52973|  %117 = gep %116, i64 40                                                                                               ;L183
 52974|  %118 = tail call zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %117, ptr %49, ptr %68)         ;L183
 52975|  br i1 %118, label %119, label %135                                                                                    ;L183
 52976| 
 52977| 119: ; preds = %115
 52978|  %120 = tail call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %116, ptr %10, ptr %49, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %68) ;L187
 52979|  %121 = gep %68, i64 1648                                                                                              ;L187
 52980|  %122 = load i64, ptr %121, , !!8                                                                                      ;L187
 52981|  %123 = icmp ult i64 %120, %122                                                                                        ;L187
 52982|  br i1 %123, label %124, label %135                                                                                    ;L187
 52983| 
 52984| 124: ; preds = %119
 52985|  %125 = tail call fastcc { i64, i64 } @ai::tower_discipline22v30_line_action_stance(ptr %10, ptr %49, ptr %68, ptr %116) ;L191
 52986|  %126 = extractvalue { i64, i64 } %125, 0                                                                              ;L191
 52987|  %127 = extractvalue { i64, i64 } %125, 1                                                                              ;L191
 52989|  store i64 %126, ptr %8,                                                                                               ;L191
 52991|  store i64 %127, ptr %7,                                                                                               ;L191
 52993|  %128 = sub nuw nsw i64 1, %39                                                                                         ;L192
 52994|  call void @gc::simulationNtB5_21AbstractGameWithCache25iter_towers_without_nexus(ptr sret([120 x i8]) %6, ptr %11, i64 %128) ;L192
 52995|     ;; self = ptr %6
 52996|     ;; self = ptr %6
 52997|     ;; f[0..+8] = ptr %49
 52998|     ;; f[8..+8] = ptr %8
 52999|     ;; f[16..+8] = ptr %7
 53000|     ;; fold[0..+8] = ptr %49
 53001|     ;; fold[0..+8] = ptr %49
 53002|     ;; fold[8..+8] = ptr %8
 53003|     ;; fold[8..+8] = ptr %8
 53004|     ;; fold[16..+8] = ptr %7
 53005|     ;; fold[16..+8] = ptr %7
 53007|  %129 = gep %6, i64 120                                                                                                ;L157<2897<194
 53008|     ;; predicate = ptr %129
 53009|  store ptr %129, ptr %5,                                                                                               ;L86<157<2897<194
 53010|  %130 = gep %5, i64 8                                                                                                  ;L86<157<2897<194
 53011|  store ptr %49, ptr %130,                                                                                              ;L86<157<2897<194
 53012|  %131 = gep %5, i64 16                                                                                                 ;L86<157<2897<194
 53013|  store ptr %8, ptr %131,                                                                                               ;L86<157<2897<194
 53014|  %132 = gep %5, i64 24                                                                                                 ;L86<157<2897<194
 53015|  store ptr %7, ptr %132,                                                                                               ;L86<157<2897<194
 53016|  %133 = call zeroext i1 @core::iter8adapters5chainINtB5_5ChainINtNtB7_7flatten7FlattenINtNtNtBb_5array4iter8IntoIterINtNtBb_6option6OptionRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEKj6_EEINtNtB7_6copied6CopiedINtNtNtBb_5slice4iter4IterB2e_EEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNtB7_6filter15filter_try_foldB2e_uINtNtNtBb_3ops12control_flow11ControlFlowuENCNvNtCshdEBA0ozCnw_7game_ai16tower_discipline41v30_line_champion_action_tower_aggro_risk0NCINvNvB49_3any5checkB2e_NCB6d_s_0E0E0B5u_EB6h_(ptr %6, ptr %5) ;L157<2897<194
 53021|  br label %135                                                                                                         ;L196
 53022| 
 53023| 134: ; preds = %86
 53024|  unreachable
 53025| 
 53026| 135: ; preds = %124, %119, %115, %106, %97, %92, %87, %82, %78, %76, %62, %51, %51, %51, %51, %51, %51, %51, %51, %51, %51, %51, %51, %51, %42, %30, %20
 53027|  %136 = phi i1 [ false, %20 ], [ false, %42 ], [ %133, %124 ], [ false, %30 ], [ false, %51 ], [ false, %62 ], [ false, %119 ], [ false, %115 ], [ false, %82 ], [ false, %76 ], [ false, %78 ], [ false, %51 ], [ false, %51 ], [ false, %51 ], [ false, %51 ], [ false, %51 ], [ false, %51 ], [ false, %51 ], [ false, %97 ], [ false, %51 ], [ false, %51 ], [ false, %51 ], [ false, %51 ], [ false, %87 ], [ false, %92 ], [ false, %106 ], [ false, %51 ] ;L0
 53028|  ret i1 %136                                                                                                           ;L196
 53029| }
