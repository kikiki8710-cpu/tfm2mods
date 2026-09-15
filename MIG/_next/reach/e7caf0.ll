 13287| define void @ai::plan_legacy8sub_plan11serpen_pokeNtB2_17SerpenPokeSubPlan17action_candidates(ptr sret([32 x i8]) %0, ptr %1, i64 %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %7, ptr %8) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 13288|  %10 = alloca [72 x i8],
 13289|  %11 = alloca [16 x i8],
 13290|  %12 = alloca [136 x i8],
 13291|  %13 = alloca [184 x i8],
 13292|  %14 = alloca [136 x i8],
 13293|  %15 = alloca [184 x i8],
 13300|  %16 = alloca [120 x i8],
 13301|  %17 = alloca [184 x i8],
 13302|  %18 = alloca [120 x i8],
 13303|  %19 = alloca [184 x i8],
 13304|  %20 = alloca [136 x i8],
 13305|  %21 = alloca [184 x i8],
 13306|  %22 = alloca [136 x i8],
 13307|  %23 = alloca [184 x i8],
 13308|  %24 = alloca [136 x i8],
 13309|  %25 = alloca [184 x i8],
 13311|  %26 = alloca [152 x i8],
 13312|  %27 = alloca [184 x i8],
 13313|  %28 = alloca [152 x i8],
 13314|  %29 = alloca [184 x i8],
 13315|  %30 = alloca [152 x i8],
 13316|  %31 = alloca [184 x i8],
 13317|  %32 = alloca [16 x i8],
 13319|  %33 = alloca [32 x i8],
 13320|  %34 = alloca [32 x i8],
 13321|  %35 = alloca [24 x i8],
 13322|  %36 = alloca [32 x i8],
 13323|  %37 = alloca [136 x i8],
 13324|  %38 = alloca [184 x i8],
 13325|  %39 = alloca [56 x i8],
 13330|  %40 = alloca [32 x i8],
 13331|  %41 = alloca [128 x i8],
 13332|  %42 = alloca [128 x i8],
 13333|  %43 = alloca [184 x i8],
 13334|  %44 = alloca [184 x i8],
 13335|  %45 = alloca [184 x i8],
 13336|  %46 = alloca [184 x i8],
 13337|  %47 = alloca [184 x i8],
 13338|  %48 = alloca [184 x i8],
 13339|  %49 = alloca [136 x i8],
 13340|  %50 = alloca [184 x i8],
 13341|  %51 = alloca [56 x i8],
 13342|  %52 = alloca [184 x i8],
 13343|  %53 = alloca [32 x i8],
 13348|  %54 = alloca [40 x i8],
 13349|  %55 = alloca [136 x i8],
 13350|  %56 = alloca [184 x i8],
 13361|  %57 = alloca [136 x i8],
 13362|  %58 = alloca [184 x i8],
 13364|  %59 = alloca [32 x i8],
 13365|  %60 = alloca [56 x i8],
 13366|  %61 = alloca [32 x i8],
 13367|  %62 = alloca [48 x i8],
 13368|  %63 = alloca [32 x i8],
 13369|  %64 = alloca [120 x i8],
 13370|  %65 = alloca [8 x i8],
 13371|  %66 = alloca [32 x i8],
 13372|  %67 = alloca [184 x i8],
 13373|  %68 = alloca [184 x i8],
 13374|  %69 = alloca [8 x i8],
 13375|  store i64 %2, ptr %69,
 13376|     ;; self = ptr %1
 13377|     ;; version = ptr %69
 13378|     ;; rnd = ptr %3
 13379|     ;; player = ptr %4
 13380|     ;; data = ptr %5
 13381|     ;; parameter = ptr %6
 13382|     ;; team_plan = ptr %7
 13383|     ;; debug = ptr %8
 13385|     ;; old_actions = ptr %66
 13386|     ;; nearest_enemy_tower = ptr %65
 13387|     ;; self = ptr %64
 13388|     ;; act_actions = ptr %63
 13389|     ;; move_actions = ptr %59
 13390|     ;; move_action_input = ptr %53
 13391|     ;; position_score = ptr %51
 13400|  call void @ai::plan_legacy9team_plan20objective_disciplineNtB4_8TeamPlan31v27_objective_discipline_action(ptr sret([184 x i8]) %68, ptr %7, i64 %2, ptr %3, ptr %4, ptr %5, i8 5) ;L16
 13401|  %70 = gep %68, i64 177                                                                                                ;L16
 13402|  %71 = load i8, ptr %70, , !!8                                                                                         ;L16
 13403|  %72 = icmp eq i8 %71, -1                                                                                              ;L16
 13404|  br i1 %72, label %77, label %73                                                                                       ;L16
 13405| 
 13406| 73: ; preds = %9
 13408|  call void @llvm.memcpy.p0.p0.i64(ptr %67, ptr %68, i64 184, i1 false)                                                 ;L16
 13409|  %74 = gep %5, i64 8                                                                                                   ;L17
 13410|  %75 = load ptr, ptr %74, , !!8, !!8                                                                                   ;L17
 13411|  %76 = load ptr, ptr %75, , !!8, !!8                                                                                   ;L17
 13412|  call void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE12from_iter_inABU_j1_EBY_(ptr sret([32 x i8]) %0, ptr %67, ptr %76) ;L17
 13415|  br label %81                                                                                                          ;L1
 13416| 
 13417| 77: ; preds = %9
 13420|  call void @ai::plan_legacy8sub_plan11serpen_pokeNtB2_17SerpenPokeSubPlan21action_candidates_old(ptr sret([32 x i8]) %66, ptr poison, i64 %2, ptr %3, ptr %4, ptr %5, ptr %6) ;L20
 13421|  %78 = gep %4, i64 2352                                                                                                ;L23
 13422|  %79 = load i64, ptr %78, , !!8                                                                                        ;L23
 13423|  %80 = icmp ult i64 %79, 2                                                                                             ;L23
 13424|  br i1 %80, label %86, label %82                                                                                       ;L23
 13425| 
 13426| 81: ; preds = %1290, %1274, %73
 13427|  ret void                                                                                                              ;L277
 13428| 
 13429| 82: ; preds = %77
 13430|  invoke void @core::panicking18panic_bounds_check(i64 %79, i64 2, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.93) #35
 13431|  to label %85 unwind label %83                                                                                         ;L23
 13432| 
 13433| 83: ; preds = %1293, %1292, %1286, %1285, %1283, %1270, %1269, %1267, %168, %143, %137, %98, %96, %82
 13434|  %84 = cleanuppad within none []
 13435|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %66) #34 [ "funclet"(token %84) ] ;L277
 13436|  cleanupret from %84 unwind to caller                                                                                  ;L15
 13437| 
 13438| 85: ; preds = %1133, %98, %82
 13439|  unreachable
 13440| 
 13441| 86: ; preds = %77
 13442|     ;; self = ptr %4
 13443|  %87 = gep %4, i64 2496                                                                                                ;L581<23
 13444|  %88 = load i32, ptr %87, , !!8                                                                                        ;L581<23
 13445|  %89 = zext nneg i32 %88 to i64                                                                                        ;L581<23
 13446|  %90 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L23
 13447|  %91 = gep %90, i64 480                                                                                                ;L23
 13448|  %92 = getelementptr [5 x ptr], ptr %91, i64 %79                                                                       ;L23
 13449|  %93 = getelementptr ptr, ptr %92, i64 %89                                                                             ;L23
 13450|  %94 = load ptr, ptr %93, , !!8                                                                                        ;L23
 13451|     ;; self = ptr %94
 13452|  %95 = icmp eq ptr %94, null                                                                                           ;L1011<23
 13453|  br i1 %95, label %98, label %96                                                                                       ;L1011<23
 13454| 
 13455| 96: ; preds = %86
 13456|     ;; champ = ptr %94
 13459|  %97 = sub nuw nsw i64 1, %79                                                                                          ;L24
 13460|  invoke void @gc::simulationNtB5_21AbstractGameWithCache25iter_towers_without_nexus(ptr sret([120 x i8]) %64, ptr %90, i64 %97)
 13461|  to label %99 unwind label %83                                                                                         ;L24
 13462| 
 13463| 98: ; preds = %86
 13464|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.94) #35
 13465|  to label %85 unwind label %83                                                                                         ;L1013<23
 13466| 
 13467| 99: ; preds = %96
 13469|  call void @llvm.memcpy.p0.p0.i64(ptr %42, ptr %64, i64 120, i1 false)                                                 ;L28<957<25
 13472|     ;; f = ptr %94
 13473|     ;; self = ptr %42
 13476|     ;; f = ptr %94
 13477|  %100 = gep %42, i64 120                                                                                               ;L69<836<3387<26
 13478|  store ptr %94, ptr %100, , !!28710                                                                                    ;L69<836<3387<26
 13480|     ;; self = ptr %42
 13483|     ;; self = ptr %42
 13485|     ;; self = ptr %42
 13487|     ;; predicate = ptr %100
 13488|     ;; self = ptr %42
 13490|     ;; opt = ptr %42
 13491|     ;; self = ptr %42
 13493|  %101 = load i64, ptr %42, , !!28818, !!8                                                                              ;L764<332<169<98<107<2706<3416<3387<26
 13494|  %102 = icmp eq i64 %101, -1                                                                                           ;L764<332<169<98<107<2706<3416<3387<26
 13495|  br i1 %102, label %133, label %103                                                                                    ;L764<332<169<98<107<2706<3416<3387<26
 13496| 
 13497| 103: ; preds = %99
 13500|     ;; a = ptr %42
 13502|     ;; self = ptr %42
 13505|     ;; self = ptr %42
 13509|     ;; self = ptr %42
 13513|     ;; self = ptr %42
 13516|     ;; self = ptr %42
 13519|  %104 = trunc nuw i64 %101 to i1                                                                                       ;L396<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13520|  br i1 %104, label %105, label %131                                                                                    ;L396<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13521| 
 13522| 105: ; preds = %103
 13523|  %106 = gep %42, i64 8                                                                                                 ;L396<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13524|     ;; iter = ptr %106
 13526|     ;; self = ptr %106
 13531|     ;; self[0..+8] = ptr %106
 13532|     ;; self[8..+8] = i64 6
 13533|  %107 = gep %42, i64 24                                                                                                ;L214<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13534|     ;; data[0..+8] = ptr %107
 13535|     ;; data[8..+8] = i64 6
 13536|     ;; f[0..+8] = ptr %107
 13537|     ;; f[8..+8] = i64 6
 13541|     ;; self = ptr %106
 13542|     ;; self = ptr %106
 13543|     ;; self = ptr %106
 13545|     ;; rhs = i64 1
 13546|  %108 = load i64, ptr %106, , !!29046, !!8                                                                             ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13547|  %109 = gep %42, i64 16                                                                                                ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13548|  %110 = load i64, ptr %109, , !!29046, !!8                                                                             ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13549|  %111 = icmp ule i64 %108, %110                                                                                        ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13550|     ;; cond = i1 true
 13551|  call void @llvm.assume(i1 %111)                                                                                       ;L210<122<166<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13552|  %112 = icmp eq i64 %108, %110                                                                                         ;L167<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13553|  br i1 %112, label %131, label %113                                                                                    ;L167<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13554| 
 13555| 113: ; preds = %128, %105
 13556|  %114 = phi i64 [ %115, %128 ], [ %108, %105 ]
 13557|     ;; i = i64 %114
 13558|     ;; value = i64 %114
 13559|     ;; self = i64 %114
 13560|  %115 = add nuw nsw i64 %114, 1                                                                                        ;L971<63<169<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13562|     ;; f = ptr undef
 13564|     ;; idx = i64 %114
 13565|     ;; index = i64 %114
 13566|     ;; self = i64 %114
 13567|     ;; self[0..+8] = ptr %107
 13568|     ;; slice[0..+8] = ptr %107
 13569|     ;; self[8..+8] = i64 6
 13570|     ;; slice[8..+8] = i64 6
 13571|  %116 = icmp ult i64 %114, 6                                                                                           ;L252<646<219<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13572|  call void @llvm.assume(i1 %116)                                                                                       ;L252<646<219<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13573|  %117 = getelementptr ptr, ptr %107, i64 %114                                                                          ;L253<646<219<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13574|     ;; self = ptr %117
 13575|     ;; self = ptr %117
 13576|     ;; src = ptr %117
 13577|  %118 = load ptr, ptr %117, , !!29112, !!8                                                                             ;L1733<1171<798<219<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13578|     ;; elem = ptr %118
 13581|     ;; inner = ptr %118
 13582|  %119 = icmp eq ptr %118, null                                                                                         ;L819<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13583|  br i1 %119, label %128, label %120                                                                                    ;L819<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13584| 
 13585| 120: ; preds = %113
 13586|     ;; item = ptr %118
 13587|     ;; x = ptr %118
 13596|     ;; self = ptr %118
 13597|  %121 = gep %118, i64 1721                                                                                             ;L1478<25<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13598|  %122 = load i8, ptr %121, , !!29193, !!8                                                                              ;L1478<25<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13599|  %123 = trunc nuw i8 %122 to i1                                                                                        ;L1478<25<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13600|  %124 = gep %118, i64 1696                                                                                             ;L1478<25<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13601|  %125 = load i64, ptr %124, , !!29196                                                                                  ;L1478<25<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13602|  %126 = icmp eq i64 %125, 0                                                                                            ;L1478<25<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13603|  %127 = select i1 %123, i1 %126, i1 false                                                                              ;L1478<25<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13604|  br i1 %127, label %132, label %128                                                                                    ;L820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13605| 
 13606| 128: ; preds = %120, %113
 13607|  %129 = icmp eq i64 %115, %110                                                                                         ;L167<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13608|  br i1 %129, label %130, label %113                                                                                    ;L167<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13609| 
 13610| 130: ; preds = %128
 13611|  store i64 %110, ptr %106, , !!29046                                                                                   ;L63<169<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13612|     ;; x = ptr null
 13613|  br label %131                                                                                                         ;L333<169<98<107<2706<3416<3387<26
 13614| 
 13615| 131: ; preds = %130, %105, %103
 13616|  store i64 -1, ptr %42, , !!28818                                                                                      ;L334<169<98<107<2706<3416<3387<26
 13617|  br label %133                                                                                                         ;L333<169<98<107<2706<3416<3387<26
 13618| 
 13619| 132: ; preds = %120
 13620|  store i64 %115, ptr %106, , !!29046                                                                                   ;L63<169<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<26
 13621|     ;; x = ptr %118
 13622|     ;; self = ptr %118
 13623|     ;; f[0..+8] = ptr %42
 13624|     ;; f[8..+8] = ptr %100
 13625|     ;; self = ptr %118
 13626|     ;; f = ptr %42
 13627|     ;; self = ptr %42
 13628|  br label %143                                                                                                         ;L1161<107<2706<3416<3387<26
 13629| 
 13630| 133: ; preds = %131, %99
 13631|  %134 = gep %42, i64 104                                                                                               ;L170<98<107<2706<3416<3387<26
 13632|     ;; self = ptr null
 13633|     ;; f[0..+8] = ptr %134
 13634|     ;; f[8..+8] = ptr %100
 13638|     ;; self = ptr %134
 13639|  %135 = load ptr, ptr %134, , !!29278, !!8                                                                             ;L764<170<1653<170<98<107<2706<3416<3387<26
 13640|  %136 = icmp eq ptr %135, null                                                                                         ;L764<170<1653<170<98<107<2706<3416<3387<26
 13641|  br i1 %136, label %168, label %137                                                                                    ;L764<170<1653<170<98<107<2706<3416<3387<26
 13642| 
 13643| 137: ; preds = %133
 13644|     ;; self = ptr %134
 13645|     ;; predicate = ptr %100
 13646|  %138 = invoke ptr @core::iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2p_4find5checkB1s_QNCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan11serpen_pokeNtB3D_17SerpenPokeSubPlan17action_candidates0E0INtNtNtBb_3ops12control_flow11ControlFlowB1s_EEB3J_(ptr %134, ptr %100)
 13647|  to label %139 unwind label %83                                                                                        ;L2971<170<1653<170<98<107<2706<3416<3387<26
 13648| 
 13649| 139: ; preds = %137
 13650|     ;; self = ptr %138
 13651|     ;; f = ptr %42
 13652|     ;; self = ptr %42
 13653|  %140 = icmp eq ptr %138, null                                                                                         ;L1161<107<2706<3416<3387<26
 13654|  br i1 %140, label %168, label %141                                                                                    ;L1161<107<2706<3416<3387<26
 13655| 
 13656| 141: ; preds = %139
 13657|  %142 = load ptr, ptr %100, , !!28655                                                                                  ;L310<1162<107<2706<3416<3387<26
 13658|  br label %143                                                                                                         ;L1161<107<2706<3416<3387<26
 13659| 
 13660| 143: ; preds = %141, %132
 13661|  %144 = phi ptr [ %94, %132 ], [ %142, %141 ]                                                                          ;L310<1162<107<2706<3416<3387<26
 13662|  %145 = phi ptr [ %118, %132 ], [ %138, %141 ]
 13663|     ;; f = ptr %100
 13664|     ;; self = ptr %100
 13665|     ;; x = ptr %145
 13666|     ;; args = ptr %145
 13668|     ;; x = ptr %145
 13672|     ;; self = ptr %145
 13673|     ;; other = ptr %144
 13674|  %146 = gep %145, i64 1632                                                                                             ;L2158<26<3379<310<1162<107<2706<3416<3387<26
 13675|  %147 = load i64, ptr %146, , !!29323, !!8                                                                             ;L2158<26<3379<310<1162<107<2706<3416<3387<26
 13676|     ;; x1 = i64 %147
 13677|     ;; self = i64 %147
 13678|  %148 = gep %145, i64 1640                                                                                             ;L2158<26<3379<310<1162<107<2706<3416<3387<26
 13679|  %149 = load i64, ptr %148, , !!29323, !!8                                                                             ;L2158<26<3379<310<1162<107<2706<3416<3387<26
 13680|     ;; y1 = i64 %149
 13681|     ;; self = i64 %149
 13682|  %150 = gep %144, i64 1632                                                                                             ;L2158<26<3379<310<1162<107<2706<3416<3387<26
 13683|  %151 = load i64, ptr %150, , !!29344, !!8                                                                             ;L2158<26<3379<310<1162<107<2706<3416<3387<26
 13684|     ;; x2 = i64 %151
 13685|     ;; other = i64 %151
 13686|  %152 = gep %144, i64 1640                                                                                             ;L2158<26<3379<310<1162<107<2706<3416<3387<26
 13687|  %153 = load i64, ptr %152, , !!29344, !!8                                                                             ;L2158<26<3379<310<1162<107<2706<3416<3387<26
 13688|     ;; y2 = i64 %153
 13689|     ;; other = i64 %153
 13690|  %154 = icmp ult i64 %147, %151                                                                                        ;L3147<7<2158<26<3379<310<1162<107<2706<3416<3387<26
 13691|  %155 = sub nuw i64 %151, %147                                                                                         ;L3147<7<2158<26<3379<310<1162<107<2706<3416<3387<26
 13692|  %156 = sub nuw i64 %147, %151                                                                                         ;L3147<7<2158<26<3379<310<1162<107<2706<3416<3387<26
 13693|  %157 = select i1 %154, i64 %155, i64 %156                                                                             ;L3147<7<2158<26<3379<310<1162<107<2706<3416<3387<26
 13694|     ;; dx = i64 %157
 13695|  %158 = icmp ult i64 %149, %153                                                                                        ;L3147<8<2158<26<3379<310<1162<107<2706<3416<3387<26
 13696|  %159 = sub nuw i64 %153, %149                                                                                         ;L3147<8<2158<26<3379<310<1162<107<2706<3416<3387<26
 13697|  %160 = sub nuw i64 %149, %153                                                                                         ;L3147<8<2158<26<3379<310<1162<107<2706<3416<3387<26
 13698|  %161 = select i1 %158, i64 %159, i64 %160                                                                             ;L3147<8<2158<26<3379<310<1162<107<2706<3416<3387<26
 13699|     ;; dy = i64 %161
 13700|  %162 = mul i64 %157, %157                                                                                             ;L9<2158<26<3379<310<1162<107<2706<3416<3387<26
 13701|  %163 = mul i64 %161, %161                                                                                             ;L9<2158<26<3379<310<1162<107<2706<3416<3387<26
 13702|  %164 = add i64 %163, %162                                                                                             ;L9<2158<26<3379<310<1162<107<2706<3416<3387<26
 13703|     ;; first[0..+8] = i64 %164
 13704|     ;; first[8..+8] = ptr %145
 13706|  call void @llvm.memcpy.p0.p0.i64(ptr %41, ptr %42, i64 128, i1 false), !!28655                                        ;L2707<3416<3387<26
 13707|  %165 = invoke { i64, ptr } @core::iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtB8_5chain5ChainINtNtB8_7flatten7FlattenINtNtNtBc_5array4iter8IntoIterINtNtBc_6option6OptionRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEKj6_EEINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterB2R_EEENCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan11serpen_pokeNtB4R_17SerpenPokeSubPlan17action_candidates0ENCINvNvNtNtNtBa_6traits8iterator8Iterator10min_by_key3keyB2R_yNCB4O_s_0E0EB6J_4foldTyB2R_ENCINvNvB6J_6min_by4foldB7X_INvB6H_7compareB2R_yEE0EB4X_(ptr %41, i64 %164, ptr %145)
 13708|  to label %166 unwind label %83                                                                                        ;L2707<3416<3387<26
 13709| 
 13710| 166: ; preds = %143
 13711|  %167 = extractvalue { i64, ptr } %165, 1                                                                              ;L2707<3416<3387<26
 13713|  br label %168                                                                                                         ;L2708<3416<3387<26
 13714| 
 13715| 168: ; preds = %166, %139, %133
 13716|  %169 = phi ptr [ %167, %166 ], [ null, %139 ], [ null, %133 ]                                                         ;L0<3416<3387<26
 13718|  store ptr %169, ptr %65,                                                                                              ;L24
 13721|     ;; self = ptr %66
 13722|     ;; self = ptr %66
 13723|  %170 = load ptr, ptr %66, , !!8, !!8                                                                                  ;L138<2073<29
 13724|     ;; p = ptr %170
 13725|  %171 = gep %66, i64 24                                                                                                ;L2075<29
 13726|  %172 = load i64, ptr %171, , !!8                                                                                      ;L2075<29
 13727|     ;; len = i64 %172
 13728|     ;; count = i64 %172
 13729|     ;; self[0..+8] = ptr %170
 13730|     ;; slice[0..+8] = ptr %170
 13731|     ;; self[8..+8] = i64 %172
 13732|     ;; slice[8..+8] = i64 %172
 13733|     ;; ptr = ptr %170
 13734|     ;; self = ptr %170
 13735|  %173 = gepS %170, i64 %172                                                                                            ;L961<100<1042<29
 13736|  %174 = load ptr, ptr %90, , !!8, !!8                                                                                  ;L30
 13737|  %175 = gep %90, i64 8                                                                                                 ;L30
 13738|  %176 = load ptr, ptr %175, , !!8, !!8                                                                                 ;L30
 13739|     ;; self[0..+8] = ptr %170
 13740|     ;; self[8..+8] = ptr %173
 13741|     ;; self[16..+8] = ptr %174
 13742|     ;; self[24..+8] = ptr %176
 13743|     ;; self[32..+8] = ptr %94
 13744|     ;; self[40..+8] = ptr %65
 13745|  store ptr %170, ptr %62,                                                                                              ;L69<836<92
 13746|  %177 = gep %62, i64 8                                                                                                 ;L69<836<92
 13747|  store ptr %173, ptr %177,                                                                                             ;L69<836<92
 13748|  %178 = gep %62, i64 16                                                                                                ;L69<836<92
 13749|  store ptr %174, ptr %178,                                                                                             ;L69<836<92
 13750|  %179 = gep %62, i64 24                                                                                                ;L69<836<92
 13751|  store ptr %176, ptr %179,                                                                                             ;L69<836<92
 13752|  %180 = gep %62, i64 32                                                                                                ;L69<836<92
 13753|  store ptr %94, ptr %180,                                                                                              ;L69<836<92
 13754|  %181 = gep %62, i64 40                                                                                                ;L69<836<92
 13755|  store ptr %65, ptr %181,                                                                                              ;L69<836<92
 13756|  %182 = gep %5, i64 8                                                                                                  ;L92
 13757|  %183 = load ptr, ptr %182, , !!8, !!8                                                                                 ;L92
 13758|  %184 = load ptr, ptr %183, , !!8, !!8                                                                                 ;L92
 13759|  invoke void @core::iter8adapters3map3MapINtNtB29_6filter6FilterINtNtNtB2d_5slice4iter4IterBU_ENCNvMNtNtNtBY_11plan_legacy8sub_plan11serpen_pokeNtB3P_17SerpenPokeSubPlan17action_candidatess0_0ENCB3M_s1_0EEBY_(ptr sret([32 x i8]) %63, ptr %62, ptr %184)
 13760|  to label %185 unwind label %83                                                                                        ;L29
 13761| 
 13762| 185: ; preds = %168
 13765|  store ptr %5, ptr %61,                                                                                                ;L94
 13766|  %186 = gep %61, i64 8                                                                                                 ;L94
 13767|  store ptr %94, ptr %186,                                                                                              ;L94
 13768|  %187 = gep %61, i64 16                                                                                                ;L94
 13769|  store ptr %4, ptr %187,                                                                                               ;L94
 13770|  %188 = gep %61, i64 24                                                                                                ;L94
 13771|  store ptr %69, ptr %188,                                                                                              ;L94
 13772|  invoke void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE6retainNCNvMNtNtNtBY_11plan_legacy8sub_plan11serpen_pokeNtB22_17SerpenPokeSubPlan17action_candidatess2_0EBY_(ptr %63, ptr %61)
 13773|  to label %192 unwind label %189                                                                                       ;L94
 13774| 
 13775| 189: ; preds = %1281, %1280, %1278, %1263, %1262, %1260, %1032, %1031, %1029, %997, %981, %980, %192, %185
 13776|  %190 = phi i1 [ true, %1281 ], [ true, %997 ], [ true, %1263 ], [ false, %1032 ], [ true, %981 ], [ true, %192 ], [ true, %185 ], [ true, %980 ], [ false, %1031 ], [ false, %1029 ], [ true, %1262 ], [ true, %1260 ], [ true, %1280 ], [ true, %1278 ] ;L0
 13777|  %191 = cleanuppad within none []
 13778|  br i1 %190, label %1293, label %1292                                                                                  ;L276
 13779| 
 13780| 192: ; preds = %185
 13783|  store ptr %1, ptr %60,                                                                                                ;L200
 13784|  %193 = gep %60, i64 8                                                                                                 ;L200
 13785|  store ptr %69, ptr %193,                                                                                              ;L200
 13786|  %194 = gep %60, i64 16                                                                                                ;L200
 13787|  store ptr %6, ptr %194,                                                                                               ;L200
 13788|  %195 = gep %60, i64 24                                                                                                ;L200
 13789|  store ptr %3, ptr %195,                                                                                               ;L200
 13790|  %196 = gep %60, i64 32                                                                                                ;L200
 13791|  store ptr %4, ptr %196,                                                                                               ;L200
 13792|  %197 = gep %60, i64 40                                                                                                ;L200
 13793|  store ptr %5, ptr %197,                                                                                               ;L200
 13794|  %198 = gep %60, i64 48                                                                                                ;L200
 13795|  store ptr %8, ptr %198,                                                                                               ;L200
 13796|  invoke void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE6retainNCNvMNtNtNtBY_11plan_legacy8sub_plan11serpen_pokeNtB22_17SerpenPokeSubPlan17action_candidatess3_0EBY_(ptr %63, ptr %60)
 13797|  to label %203 unwind label %189                                                                                       ;L200
 13798| 
 13799| 199: ; preds = %973, %946, %945, %943, %289, %284, %281, %280, %265, %264, %262, %252, %240, %228, %221
 13800|  %200 = phi i1 [ true, %973 ], [ true, %280 ], [ false, %946 ], [ true, %289 ], [ true, %284 ], [ true, %262 ], [ true, %281 ], [ false, %943 ], [ true, %264 ], [ true, %265 ], [ true, %252 ], [ true, %221 ], [ true, %228 ], [ true, %240 ], [ false, %945 ] ;L0<205
 13801|  %201 = cleanuppad within none []
 13802|  br i1 %200, label %981, label %980                                                                                    ;L416<205
 13803| 
 13804| 202: ; preds = %964, %941, %643, %604, %264
 13805|  unreachable
 13806| 
 13807| 203: ; preds = %192
 13810|  %204 = gep %6, i64 2544                                                                                               ;L205
 13811|  %205 = load i64, ptr %69, , !!8                                                                                       ;L205
 13815|     ;; version = i64 %205
 13817|     ;; rnd = ptr %3
 13818|     ;; player = ptr %4
 13819|     ;; data = ptr %5
 13820|     ;; positioning_score = ptr %204
 13821|     ;; debug = ptr %8
 13822|     ;; res = ptr %40
 13823|     ;; position_score = ptr %39
 13824|     ;; near_enemies = ptr %36
 13825|     ;; jrng = ptr %32
 13826|     ;; len = i64 5
 13827|     ;; count = i64 5
 13829|     ;; count = i64 5
 13830|     ;; index = i64 0
 13831|     ;; self = i64 0
 13832|     ;; count = i64 1
 13833|     ;; count = i64 1
 13834|     ;; index = i64 0
 13835|     ;; self = i64 0
 13836|     ;; count = i64 5
 13839|  %206 = load ptr, ptr %183, , !!29528, !!8, !!8                                                                        ;L280<205
 13840|     ;; bump = ptr %206
 13841|     ;; bump = ptr %206
 13842|  store ptr inttoptr (i64 8 to ptr), ptr %40, , !!29528                                                                 ;L547<280<205
 13843|  %207 = gep %40, i64 8                                                                                                 ;L547<280<205
 13844|  store ptr %206, ptr %207, , !!29528                                                                                   ;L547<280<205
 13845|  %208 = gep %40, i64 16                                                                                                ;L547<280<205
 13846|  %209 = gep %40, i64 24                                                                                                ;L547<280<205
 13847|  call void @llvm.memset.p0.i64(ptr %208, i8 0, i64 16, i1 false), !!29528                                              ;L547<280<205
 13848|     ;; self = ptr %90
 13849|     ;; self = ptr %90
 13850|     ;; self = ptr %90
 13851|  %210 = load ptr, ptr %93, , !!29557, !!8                                                                              ;L282<205
 13852|     ;; self = ptr %210
 13853|     ;; self = ptr %210
 13854|  %211 = icmp eq ptr %210, null                                                                                         ;L1011<282<205
 13855|  br i1 %211, label %264, label %212                                                                                    ;L1011<282<205
 13856| 
 13857| 212: ; preds = %203
 13858|     ;; champ = ptr %210
 13859|     ;; champ = ptr %210
 13860|     ;; self = ptr %210
 13861|     ;; self = ptr %210
 13862|     ;; self = ptr %210
 13863|     ;; entity = ptr %210
 13864|     ;; other = ptr %210
 13865|     ;; caster = ptr %210
 13866|     ;; self = ptr %210
 13867|     ;; self = ptr %210
 13868|     ;; team = i64 %97
 13869|     ;; team = i64 %97
 13870|     ;; team = i64 %97
 13871|  %213 = getelementptr [5 x ptr], ptr %91, i64 %97                                                                      ;L1905<285<205
 13872|     ;; self[0..+8] = ptr %213
 13873|     ;; slice[0..+8] = ptr %213
 13874|     ;; self[8..+8] = i64 5
 13875|     ;; slice[8..+8] = i64 5
 13876|     ;; ptr = ptr %213
 13877|     ;; self = ptr %213
 13878|  %214 = gep %213, i64 40                                                                                               ;L961<100<1042<1905<285<205
 13879|     ;; self = ptr undef
 13880|     ;; self = ptr undef
 13881|     ;; f[0..+8] = ptr undef
 13882|     ;; f[8..+8] = ptr %4
 13883|     ;; f[16..+8] = ptr %5
 13884|     ;; f[24..+8] = ptr %210
 13885|     ;; fold[0..+8] = ptr undef
 13886|     ;; fold[8..+8] = ptr %4
 13887|     ;; fold[16..+8] = ptr %5
 13888|     ;; fold[24..+8] = ptr %210
 13891|     ;; f[8..+8] = ptr undef
 13892|     ;; f[16..+8] = ptr %4
 13893|     ;; f[24..+8] = ptr %5
 13894|     ;; f[32..+8] = ptr %210
 13895|     ;; self = ptr undef
 13898|     ;; self = ptr undef
 13899|     ;; count = i64 1
 13900|     ;; ptr = ptr %213
 13901|     ;; self = ptr %213
 13902|     ;; end_or_len = ptr %214
 13905|  br label %215                                                                                                         ;L180<2493<138<2897<285<205
 13906| 
 13907| 215: ; preds = %232, %212
 13908|  %216 = phi i64 [ 0, %212 ], [ %218, %232 ]
 13909|  %217 = gep %213, i64 %216                                                                                             ;L656<185<2493<138<2897<285<205
 13910|     ;; ptr = ptr %217
 13911|  %218 = add nuw nsw i64 %216, 8                                                                                        ;L656<185<2493<138<2897<285<205
 13912|     ;; x = ptr %217
 13913|  %219 = load ptr, ptr %217, , !!29664, !!8                                                                             ;L2494<138<2897<285<205
 13914|     ;; f = ptr undef
 13918|  %220 = icmp eq ptr %219, null                                                                                         ;L49<2494<138<2897<285<205
 13919|  br i1 %220, label %232, label %221                                                                                    ;L49<2494<138<2897<285<205
 13920| 
 13921| 221: ; preds = %215
 13922|     ;; x = ptr %219
 13925|     ;; x = ptr %219
 13930|     ;; c = ptr %219
 13931|     ;; self = ptr %219
 13932|     ;; self = ptr %219
 13933|     ;; self = ptr %219
 13934|     ;; self = ptr %219
 13935|     ;; self = ptr %219
 13936|  %222 = invoke zeroext i1 @ai::utils26nontarget_windup_perceived(i64 %205, ptr %4, ptr %5, ptr %219)
 13937|  to label %223 unwind label %199, !!29557                                                                              ;L286<2893<50<2494<138<2897<285<205
 13938| 
 13939| 223: ; preds = %221
 13940|  %224 = gep %219, i64 104
 13941|  %225 = load i64, ptr %224, , !!29728
 13942|  %226 = icmp eq i64 %225, 13
 13943|  %227 = select i1 %222, i1 %226, i1 false                                                                              ;L286<2893<50<2494<138<2897<285<205
 13944|  br i1 %227, label %234, label %232                                                                                    ;L286<2893<50<2494<138<2897<285<205
 13945| 
 13946| 228: ; preds = %254, %254, %244, %244, %242
 13947|  %229 = phi ptr [ %249, %244 ], [ %243, %242 ], [ %249, %244 ], [ %259, %254 ], [ %259, %254 ]
 13948|  %230 = invoke zeroext i1 @gc::simulation6effectNtB2_6Effect11is_in_range(ptr %229, ptr %219, ptr %210)
 13949|  to label %231 unwind label %199, !!29557                                                                              ;L0<2893<50<2494<138<2897<285<205
 13950| 
 13951| 231: ; preds = %228
 13952|  br i1 %230, label %265, label %232                                                                                    ;L2494<138<2897<285<205
 13953| 
 13954| 232: ; preds = %254, %244, %237, %234, %231, %223, %215
 13955|     ;; self = ptr undef
 13956|     ;; count = i64 1
 13957|     ;; ptr = !DIArgList(ptr %213, i64 %218)
 13958|     ;; self = !DIArgList(ptr %213, i64 %218)
 13959|     ;; end_or_len = ptr %214
 13962|  %233 = icmp eq i64 %218, 40                                                                                           ;L1714<180<2493<138<2897<285<205
 13963|  br i1 %233, label %265, label %215                                                                                    ;L180<2493<138<2897<285<205
 13964| 
 13965| 234: ; preds = %223
 13966|     ;; champ = ptr %219
 13967|  %235 = gep %219, i64 112                                                                                              ;L1572<287<2893<50<2494<138<2897<285<205
 13968|  %236 = load i64, ptr %235, , !!29728, !!8                                                                             ;L1572<287<2893<50<2494<138<2897<285<205
 13969|  switch i64 %236, label %232 [
 13970|  i64 4, label %237
 13971|  i64 5, label %244
 13972|  i64 6, label %254
 13973|  ]                                                                                                                     ;L287<2893<50<2494<138<2897<285<205
 13974| 
 13975| 237: ; preds = %234
 13976|     ;; self = ptr %219
 13977|  %238 = gep %219, i64 1272                                                                                             ;L742<287<2893<50<2494<138<2897<285<205
 13978|  %239 = load i32, ptr %238, , !!29728, !!8                                                                             ;L742<287<2893<50<2494<138<2897<285<205
 13979|  switch i32 %239, label %232 [
 13980|  i32 -1, label %240
 13981|  i32 1, label %242
 13982|  i32 2, label %242
 13983|  ]                                                                                                                     ;L742<287<2893<50<2494<138<2897<285<205
 13984| 
 13985| 240: ; preds = %237
 13986|     ;; self = ptr null
 13987|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.21) #35
 13988|  to label %241 unwind label %199, !!29557                                                                              ;L1013<287<2893<50<2494<138<2897<285<205
 13989| 
 13990| 241: ; preds = %240
 13991|  unreachable                                                                                                           ;L1013<287<2893<50<2494<138<2897<285<205
 13992| 
 13993| 242: ; preds = %237, %237
 13994|  %243 = gep %219, i64 1224                                                                                             ;L742<287<2893<50<2494<138<2897<285<205
 13995|     ;; self = ptr %219
 13996|     ;; self = ptr %243
 13997|  br label %228                                                                                                         ;L287<2893<50<2494<138<2897<285<205
 13998| 
 13999| 244: ; preds = %234
 14000|  %245 = gep %219, i64 1480                                                                                             ;L1693<289<2893<50<2494<138<2897<285<205
 14001|  %246 = load i64, ptr %245, , !!29728, !!8                                                                             ;L1693<289<2893<50<2494<138<2897<285<205
 14002|  %247 = icmp ugt i64 %246, 2                                                                                           ;L1693<289<2893<50<2494<138<2897<285<205
 14003|  %248 = gep %219, i64 1280                                                                                             ;L1693<289<2893<50<2494<138<2897<285<205
 14004|  %249 = select i1 %247, ptr %248, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.22                                        ;L1693<289<2893<50<2494<138<2897<285<205
 14005|     ;; self = ptr %249
 14006|  %250 = gep %249, i64 48                                                                                               ;L742<289<2893<50<2494<138<2897<285<205
 14007|  %251 = load i32, ptr %250, , !!29728, !!8                                                                             ;L742<289<2893<50<2494<138<2897<285<205
 14008|  switch i32 %251, label %232 [
 14009|  i32 -1, label %252
 14010|  i32 1, label %228
 14011|  i32 2, label %228
 14012|  ]                                                                                                                     ;L742<289<2893<50<2494<138<2897<285<205
 14013| 
 14014| 252: ; preds = %244
 14015|     ;; self = ptr null
 14016|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.23) #35
 14017|  to label %253 unwind label %199, !!29557                                                                              ;L1013<289<2893<50<2494<138<2897<285<205
 14018| 
 14019| 253: ; preds = %252
 14020|  unreachable                                                                                                           ;L1013<289<2893<50<2494<138<2897<285<205
 14021| 
 14022| 254: ; preds = %234
 14023|  %255 = gep %219, i64 1480                                                                                             ;L1701<291<2893<50<2494<138<2897<285<205
 14024|  %256 = load i64, ptr %255, , !!29728, !!8                                                                             ;L1701<291<2893<50<2494<138<2897<285<205
 14025|  %257 = icmp ugt i64 %256, 4                                                                                           ;L1701<291<2893<50<2494<138<2897<285<205
 14026|  %258 = gep %219, i64 1336                                                                                             ;L1701<291<2893<50<2494<138<2897<285<205
 14027|  %259 = select i1 %257, ptr %258, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.22                                        ;L1701<291<2893<50<2494<138<2897<285<205
 14028|     ;; self = ptr %259
 14029|  %260 = gep %259, i64 48                                                                                               ;L742<291<2893<50<2494<138<2897<285<205
 14030|  %261 = load i32, ptr %260, , !!29728, !!8                                                                             ;L742<291<2893<50<2494<138<2897<285<205
 14031|  switch i32 %261, label %232 [
 14032|  i32 -1, label %262
 14033|  i32 1, label %228
 14034|  i32 2, label %228
 14035|  ]                                                                                                                     ;L742<291<2893<50<2494<138<2897<285<205
 14036| 
 14037| 262: ; preds = %254
 14038|     ;; self = ptr null
 14039|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.24) #35
 14040|  to label %263 unwind label %199, !!29557                                                                              ;L1013<291<2893<50<2494<138<2897<285<205
 14041| 
 14042| 263: ; preds = %262
 14043|  unreachable                                                                                                           ;L1013<291<2893<50<2494<138<2897<285<205
 14044| 
 14045| 264: ; preds = %203
 14046|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.89) #35
 14047|  to label %202 unwind label %199, !!29557                                                                              ;L1013<282<205
 14048| 
 14049| 265: ; preds = %232, %231
 14050|  %266 = phi i1 [ false, %232 ], [ true, %231 ]                                                                         ;L1714<180<2493<138<2897<285<205
 14051|     ;; has_non_target_action_range = i1 %266
 14053|  %267 = gep %210, i64 1632                                                                                             ;L299<205
 14054|  %268 = load i64, ptr %267, , !!29557, !!8                                                                             ;L299<205
 14055|     ;; x1 = i64 %268
 14056|     ;; self = i64 %268
 14057|     ;; x1 = i64 %268
 14058|     ;; self = i64 %268
 14059|     ;; x2 = i64 %268
 14060|     ;; other = i64 %268
 14061|     ;; x1 = i64 %268
 14062|     ;; self = i64 %268
 14063|  %269 = gep %210, i64 1640                                                                                             ;L299<205
 14064|  %270 = load i64, ptr %269, , !!29557, !!8                                                                             ;L299<205
 14065|     ;; y1 = i64 %270
 14066|     ;; self = i64 %270
 14067|     ;; y1 = i64 %270
 14068|     ;; self = i64 %270
 14069|     ;; y2 = i64 %270
 14070|     ;; other = i64 %270
 14071|     ;; y1 = i64 %270
 14072|     ;; self = i64 %270
 14073|  invoke void @ai::position_eval26position_score_at_position(ptr sret([56 x i8]) %39, i64 %205, ptr %4, ptr %5, ptr %204, i64 %268, i64 %270, i8 11)
 14074|  to label %271 unwind label %199, !!29557                                                                              ;L298<205
 14075| 
 14076| 271: ; preds = %265
 14077|  %272 = gep %39, i64 48                                                                                                ;L300<205
 14078|  %273 = load i8, ptr %272, , !!29528, !!8                                                                              ;L300<205
 14079|  %274 = trunc nuw i8 %273 to i1                                                                                        ;L300<205
 14080|  %275 = gep %39, i64 49                                                                                                ;L300<205
 14081|  %276 = load i8, ptr %275, , !!29528                                                                                   ;L300<205
 14082|  %277 = trunc nuw i8 %276 to i1                                                                                        ;L300<205
 14083|     ;; on_trajectory = i1 %277
 14084|  %278 = or i1 %266, %277
 14085|  %279 = select i1 %274, i1 true, i1 %278                                                                               ;L300<205
 14086|  br i1 %279, label %280, label %281                                                                                    ;L300<205
 14087| 
 14088| 280: ; preds = %271
 14091|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %37, ptr %5, ptr %4, i64 5, i1 zeroext true)
 14092|  to label %965 unwind label %199, !!29557                                                                              ;L302<205
 14093| 
 14094| 281: ; preds = %271
 14095|     ;; runaway = i8 0
 14096|  %282 = gep %4, i64 384                                                                                                ;L314<205
 14097|  %283 = invoke i64 @gc::simulation5state6playerNtB4_16AthleteParameter20positioning_accuracy(ptr %282)
 14098|  to label %284 unwind label %199, !!29557                                                                              ;L314<205
 14099| 
 14100| 284: ; preds = %281
 14101|     ;; positioning_accuracy = i64 %283
 14102|     ;; min_v = i64 %283
 14103|  %285 = sub i64 2000, %283                                                                                             ;L316<205
 14104|     ;; max_v = i64 %285
 14107|     ;; self[0..+8] = ptr %213
 14108|     ;; slice[0..+8] = ptr %213
 14109|     ;; self[8..+8] = i64 5
 14110|     ;; slice[8..+8] = i64 5
 14111|     ;; self = ptr %213
 14112|     ;; self[0..+8] = ptr %213
 14113|     ;; self[8..+8] = ptr %214
 14114|     ;; predicate = ptr %210
 14115|  store ptr %213, ptr %35, , !!29528                                                                                    ;L28<957<318<205
 14116|  %286 = gep %35, i64 8                                                                                                 ;L28<957<318<205
 14117|  store ptr %214, ptr %286, , !!29528                                                                                   ;L28<957<318<205
 14118|  %287 = gep %35, i64 16                                                                                                ;L28<957<318<205
 14119|  store ptr %210, ptr %287, , !!29528                                                                                   ;L28<957<318<205
 14120|  invoke void @core::iter8adapters6filter6FilterINtNtB29_10filter_map9FilterMapINtNtNtB2d_5slice4iter4IterINtNtB2d_6option6OptionBU_EENCNvMs3_BZ_NtBZ_21AbstractGameWithCache14iter_champions0ENCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan11serpen_pokeNtB5m_17SerpenPokeSubPlan15get_move_actions_0EEB5s_(ptr sret([32 x i8]) %36, ptr %35, ptr %206)
 14121|  to label %288 unwind label %199, !!29557                                                                              ;L318<205
 14122| 
 14123| 288: ; preds = %284
 14126|  invoke void @gc::simulation6entity6EntityENtNtCsjihNppCmMEE_4core5clone5Clone5cloneCshdEBA0ozCnw_7game_ai(ptr sret([32 x i8]) %34, ptr %36)
 14127|  to label %291 unwind label %289, !!29557                                                                              ;L319<205
 14128| 
 14129| 289: ; preds = %964, %956, %947, %941, %934, %925, %744, %729, %715, %708, %697, %694, %693, %690, %667, %643, %605, %604, %594, %584, %577, %565, %497, %472, %464, %457, %449, %424, %416, %402, %400, %398, %357, %353, %351, %347, %345, %341, %335, %310, %295, %291, %288
 14130|  %290 = cleanuppad within none []
 14131|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEECshdEBA0ozCnw_7game_ai(ptr %36) #34 [ "funclet"(token %290) ], !!29557 ;L416<205
 14132|  cleanupret from %290 unwind label %199                                                                                ;L416<205
 14133| 
 14134| 291: ; preds = %288
 14136|  store ptr inttoptr (i64 8 to ptr), ptr %33, , !!29528                                                                 ;L547<320<205
 14137|  %292 = gep %33, i64 8                                                                                                 ;L547<320<205
 14138|  store ptr %206, ptr %292, , !!29528                                                                                   ;L547<320<205
 14139|  %293 = gep %33, i64 16                                                                                                ;L547<320<205
 14140|  call void @llvm.memset.p0.i64(ptr %293, i8 0, i64 16, i1 false), !!29528                                              ;L547<320<205
 14141|  %294 = invoke i64 @ai::fight_check19check_kill_die_tick(i64 %205, ptr %3, ptr %5, ptr %4, ptr %210, ptr %34, ptr %33, ptr %8)
 14142|  to label %295 unwind label %289, !!29557                                                                              ;L319<205
 14143| 
 14144| 295: ; preds = %291
 14145|     ;; me_die_tick = i64 %294
 14148|  %296 = load ptr, ptr %90, , !!29557, !!8, !!8                                                                         ;L322<205
 14149|  %297 = load ptr, ptr %175, , !!29557, !!8, !!8                                                                        ;L322<205
 14150|  %298 = gep %297, i64 64                                                                                               ;L322<205
 14151|  %299 = load ptr, ptr %298, , !!29557, !!8                                                                             ;L322<205
 14152|  %300 = invoke { i64, ptr } %299(ptr %296)
 14153|  to label %301 unwind label %289, !!29557                                                                              ;L322<205
 14154| 
 14155| 301: ; preds = %295
 14156|  %302 = extractvalue { i64, ptr } %300, 0                                                                              ;L322<205
 14157|     ;; self[0..+8] = i64 %302
 14159|  %303 = icmp eq i64 %302, 0                                                                                            ;L231<322<205
 14160|  br i1 %303, label %304, label %964                                                                                    ;L231<322<205
 14161| 
 14162| 304: ; preds = %301
 14163|  %305 = extractvalue { i64, ptr } %300, 1                                                                              ;L322<205
 14164|     ;; self[8..+8] = ptr %305
 14165|  %306 = icmp ne ptr %305, null
 14166|  call void @llvm.assume(i1 %306)
 14167|     ;; self = ptr %305
 14168|     ;; self = ptr %305
 14169|     ;; self = ptr %305
 14170|     ;; self = ptr %305
 14171|  %307 = gep %305, i64 472                                                                                              ;L1864<3787<322<205
 14172|  %308 = load i64, ptr %307, , !!29557, !!8                                                                             ;L1864<3787<322<205
 14175|     ;; self[8..+8] = i64 %308
 14176|     ;; slice[8..+8] = i64 %308
 14177|  %309 = icmp eq i64 %308, 0                                                                                            ;L219<576<322<205
 14178|  br i1 %309, label %317, label %310                                                                                    ;L219<576<322<205
 14179| 
 14180| 310: ; preds = %304
 14181|  %311 = gep %305, i64 464                                                                                              ;L614<609<296<1968<1864<3787<322<205
 14182|  %312 = load ptr, ptr %311, , !!29557, !!8, !!8                                                                        ;L614<609<296<1968<1864<3787<322<205
 14183|     ;; self[0..+8] = ptr %312
 14184|     ;; slice[0..+8] = ptr %312
 14185|     ;; self = ptr %312
 14186|     ;; f[0..+8] = ptr %296
 14187|     ;; f[8..+8] = ptr %297
 14188|     ;; x = ptr %312
 14189|  %313 = gep %297, i64 496                                                                                              ;L1543<323<205
 14190|  %314 = load ptr, ptr %313, , !!29557                                                                                  ;L1543<323<205
 14191|  %315 = load i64, ptr %312, , !!29557, !!8                                                                             ;L1543<323<205
 14193|  %316 = invoke ptr %314(ptr %296, i64 %315)
 14194|  to label %317 unwind label %289, !!29557                                                                              ;L323<1543<323<205
 14195| 
 14196| 317: ; preds = %310, %304
 14197|  %318 = phi ptr [ null, %304 ], [ %316, %310 ]                                                                         ;L0<323<205
 14198|     ;; objective_entity = ptr %318
 14199|     ;; self = ptr %36
 14200|     ;; self = ptr %36
 14201|     ;; self = ptr %36
 14202|  %319 = load ptr, ptr %36, , !!29528, !!8, !!8                                                                         ;L138<2073<2136<325<205
 14203|     ;; p = ptr %319
 14204|  %320 = gep %36, i64 24                                                                                                ;L2075<2136<325<205
 14205|  %321 = load i64, ptr %320, , !!29528, !!8                                                                             ;L2075<2136<325<205
 14206|     ;; len = i64 %321
 14207|     ;; count = i64 %321
 14208|     ;; self[0..+8] = ptr %319
 14209|     ;; slice[0..+8] = ptr %319
 14210|     ;; self[8..+8] = i64 %321
 14211|     ;; slice[8..+8] = i64 %321
 14212|     ;; ptr = ptr %319
 14213|     ;; self = ptr %319
 14214|  %322 = getelementptr ptr, ptr %319, i64 %321                                                                          ;L961<100<1042<2136<325<205
 14215|     ;; iter[0..+8] = ptr %319
 14216|     ;; iter[8..+8] = ptr %322
 14217|  %323 = gep %32, i64 8
 14218|  %324 = icmp eq ptr %318, null
 14219|  %325 = gep %318, i64 1632
 14220|  %326 = gep %318, i64 1640
 14221|  %327 = gep %183, i64 8
 14222|  %328 = gep %29, i64 177
 14223|  %329 = gep %31, i64 177
 14224|  %330 = gep %27, i64 177
 14225|  br label %331                                                                                                         ;L325<205
 14226| 
 14227| 331: ; preds = %431, %317
 14228|  %332 = phi ptr [ %319, %317 ], [ %336, %431 ]                                                                         ;L325<205
 14229|  %333 = phi i1 [ false, %317 ], [ %432, %431 ]                                                                         ;L313<205
 14231|     ;; iter[0..+8] = ptr %332
 14232|     ;; self = ptr undef
 14233|     ;; ptr = ptr %332
 14234|     ;; self = ptr %332
 14235|     ;; end_or_len = ptr %322
 14238|  %334 = icmp eq ptr %332, %322                                                                                         ;L1714<180<325<205
 14239|  br i1 %334, label %479, label %335                                                                                    ;L180<325<205
 14240| 
 14241| 335: ; preds = %331
 14242|  %336 = gep %332, i64 8                                                                                                ;L656<185<325<205
 14243|     ;; iter[0..+8] = ptr %336
 14244|  %337 = load ptr, ptr %332, , !!29557, !!8, !!8                                                                        ;L325<205
 14245|     ;; enemy = ptr %337
 14246|     ;; other = ptr %337
 14248|  %338 = gep %337, i64 1472                                                                                             ;L326<205
 14249|  %339 = load i64, ptr %338, , !!29557, !!8                                                                             ;L326<205
 14250|  %340 = invoke { i64, i64 } @ai::utils18range_misjudge_rng(i64 %205, ptr %5, ptr %4, i64 %339)
 14251|  to label %341 unwind label %289, !!29557                                                                              ;L326<205
 14252| 
 14253| 341: ; preds = %335
 14254|  %342 = extractvalue { i64, i64 } %340, 0                                                                              ;L326<205
 14255|  %343 = extractvalue { i64, i64 } %340, 1                                                                              ;L326<205
 14256|  store i64 %342, ptr %32, , !!29528                                                                                    ;L326<205
 14257|  store i64 %343, ptr %323, , !!29528                                                                                   ;L326<205
 14258|  %344 = invoke i64 @ai::plan_legacy3old6battle17max_range_can_use(ptr %210, ptr %337)
 14259|  to label %345 unwind label %289, !!29557                                                                              ;L327<205
 14260| 
 14261| 345: ; preds = %341
 14262|  %346 = invoke i64 @ai::utils19range_misjudge_roll(ptr %3, ptr %32, i64 %283, i64 %285)
 14263|  to label %347 unwind label %289, !!29557                                                                              ;L327<205
 14264| 
 14265| 347: ; preds = %345
 14266|  %348 = mul i64 %346, %344                                                                                             ;L327<205
 14267|  %349 = udiv i64 %348, 1000                                                                                            ;L327<205
 14268|     ;; mr = i64 %349
 14269|  %350 = invoke i64 @ai::plan_legacy3old6battle17max_range_can_use(ptr %337, ptr %210)
 14270|  to label %351 unwind label %289, !!29557                                                                              ;L328<205
 14271| 
 14272| 351: ; preds = %347
 14273|  %352 = invoke i64 @ai::utils19range_misjudge_roll(ptr %3, ptr %32, i64 %283, i64 %285)
 14274|  to label %353 unwind label %289, !!29557                                                                              ;L328<205
 14275| 
 14276| 353: ; preds = %351
 14277|  %354 = mul i64 %352, %350                                                                                             ;L328<205
 14278|  %355 = udiv i64 %354, 1000                                                                                            ;L328<205
 14279|     ;; emr = i64 %355
 14280|  %356 = invoke i64 @ai::plan_legacy3old6battle24max_range_nearly_can_use(ptr %337, ptr %210, i64 40)
 14281|  to label %357 unwind label %289, !!29557                                                                              ;L329<205
 14282| 
 14283| 357: ; preds = %353
 14284|  %358 = invoke i64 @ai::utils19range_misjudge_roll(ptr %3, ptr %32, i64 %283, i64 %285)
 14285|  to label %359 unwind label %289, !!29557                                                                              ;L329<205
 14286| 
 14287| 359: ; preds = %357
 14288|  %360 = mul i64 %358, %356                                                                                             ;L329<205
 14289|  %361 = udiv i64 %360, 1000                                                                                            ;L329<205
 14290|     ;; emr_near = i64 %361
 14291|  %362 = gep %337, i64 1632                                                                                             ;L2158<330<205
 14292|  %363 = load i64, ptr %362, , !!29557, !!8                                                                             ;L2158<330<205
 14293|     ;; x2 = i64 %363
 14294|     ;; other = i64 %363
 14295|  %364 = gep %337, i64 1640                                                                                             ;L2158<330<205
 14296|  %365 = load i64, ptr %364, , !!29557, !!8                                                                             ;L2158<330<205
 14297|     ;; y2 = i64 %365
 14298|     ;; other = i64 %365
 14299|  %366 = icmp ult i64 %268, %363                                                                                        ;L3147<7<2158<330<205
 14300|  %367 = sub nuw i64 %363, %268                                                                                         ;L3147<7<2158<330<205
 14301|  %368 = sub nuw i64 %268, %363                                                                                         ;L3147<7<2158<330<205
 14302|  %369 = select i1 %366, i64 %367, i64 %368                                                                             ;L3147<7<2158<330<205
 14303|     ;; dx = i64 %369
 14304|  %370 = icmp ult i64 %270, %365                                                                                        ;L3147<8<2158<330<205
 14305|  %371 = sub nuw i64 %365, %270                                                                                         ;L3147<8<2158<330<205
 14306|  %372 = sub nuw i64 %270, %365                                                                                         ;L3147<8<2158<330<205
 14307|  %373 = select i1 %370, i64 %371, i64 %372                                                                             ;L3147<8<2158<330<205
 14308|     ;; dy = i64 %373
 14309|  %374 = mul i64 %369, %369                                                                                             ;L9<2158<330<205
 14310|  %375 = mul i64 %373, %373                                                                                             ;L9<2158<330<205
 14311|  %376 = add i64 %375, %374                                                                                             ;L9<2158<330<205
 14312|     ;; dist = i64 %376
 14313|     ;; self = ptr %318
 14314|     ;; f = ptr %337
 14315|  br i1 %324, label %392, label %377                                                                                    ;L708<333<205
 14316| 
 14317| 377: ; preds = %359
 14318|     ;; x = ptr %318
 14319|  %378 = load i64, ptr %325, , !!29557, !!8                                                                             ;L710<333<205
 14320|  %379 = load i64, ptr %326, , !!29557, !!8                                                                             ;L710<333<205
 14325|     ;; x1 = i64 %363
 14326|     ;; self = i64 %363
 14327|     ;; y1 = i64 %365
 14328|     ;; self = i64 %365
 14329|     ;; x2 = i64 %378
 14330|     ;; other = i64 %378
 14331|     ;; y2 = i64 %379
 14332|     ;; other = i64 %379
 14333|  %380 = icmp ult i64 %363, %378                                                                                        ;L3147<7<2158<333<710<333<205
 14334|  %381 = sub nuw i64 %378, %363                                                                                         ;L3147<7<2158<333<710<333<205
 14335|  %382 = sub nuw i64 %363, %378                                                                                         ;L3147<7<2158<333<710<333<205
 14336|  %383 = select i1 %380, i64 %381, i64 %382                                                                             ;L3147<7<2158<333<710<333<205
 14337|     ;; dx = i64 %383
 14338|  %384 = icmp ult i64 %365, %379                                                                                        ;L3147<8<2158<333<710<333<205
 14339|  %385 = sub nuw i64 %379, %365                                                                                         ;L3147<8<2158<333<710<333<205
 14340|  %386 = sub nuw i64 %365, %379                                                                                         ;L3147<8<2158<333<710<333<205
 14341|  %387 = select i1 %384, i64 %385, i64 %386                                                                             ;L3147<8<2158<333<710<333<205
 14342|     ;; dy = i64 %387
 14343|  %388 = mul i64 %383, %383                                                                                             ;L9<2158<333<710<333<205
 14344|  %389 = mul i64 %387, %387                                                                                             ;L9<2158<333<710<333<205
 14345|  %390 = add i64 %389, %388                                                                                             ;L9<2158<333<710<333<205
 14346|  %391 = icmp ult i64 %390, 40000000001                                                                                 ;L333<710<333<205
 14347|  br label %392                                                                                                         ;L333<710<333<205
 14348| 
 14349| 392: ; preds = %377, %359
 14350|  %393 = phi i1 [ true, %359 ], [ %391, %377 ]                                                                          ;L0<333<205
 14352|  %394 = load ptr, ptr %327, , !!29557, !!8, !!8                                                                        ;L336<205
 14353|  %395 = gep %394, i64 4856                                                                                             ;L336<205
 14354|  %396 = load i64, ptr %395, , !!29557, !!8                                                                             ;L336<205
 14355|  %397 = icmp ult i64 %294, %396                                                                                        ;L336<205
 14356|  br i1 %397, label %398, label %400                                                                                    ;L336<205
 14357| 
 14358| 398: ; preds = %392
 14359|  %399 = invoke i64 @ai::plan_legacy3old6battle24max_range_nearly_can_use(ptr %337, ptr %210, i64 60)
 14360|  to label %402 unwind label %289, !!29557                                                                              ;L351<205
 14361| 
 14362| 400: ; preds = %392
 14363|  %401 = invoke i64 @gc::simulation6entityNtB5_6Entity18remain_action_time(ptr %337)
 14364|  to label %433 unwind label %289, !!29557                                                                              ;L337<205
 14365| 
 14366| 402: ; preds = %398
 14367|  %403 = invoke i64 @ai::utils19range_misjudge_roll(ptr %3, ptr %32, i64 %283, i64 %285)
 14368|  to label %404 unwind label %289, !!29557                                                                              ;L351<205
 14369| 
 14370| 404: ; preds = %402
 14371|  %405 = mul i64 %403, %399                                                                                             ;L351<205
 14372|  %406 = udiv i64 %405, 1000                                                                                            ;L351<205
 14373|     ;; emr = i64 %406
 14374|  %407 = mul i64 %349, %349                                                                                             ;L353<205
 14375|  %408 = icmp ugt i64 %376, %407                                                                                        ;L353<205
 14376|  %409 = icmp samesign ult i64 %406, %349                                                                               ;L353<205
 14377|  %410 = and i1 %408, %409                                                                                              ;L353<205
 14378|  br i1 %410, label %415, label %411                                                                                    ;L353<205
 14379| 
 14380| 411: ; preds = %404
 14381|  %412 = mul i64 %406, %406                                                                                             ;L358<205
 14382|  %413 = icmp ule i64 %376, %412                                                                                        ;L358<205
 14383|  %414 = select i1 %413, i1 true, i1 %333                                                                               ;L358<205
 14384|  br label %431                                                                                                         ;L358<205
 14385| 
 14386| 415: ; preds = %404
 14387|  br i1 %393, label %416, label %431                                                                                    ;L355<205
 14388| 
 14389| 416: ; preds = %415
 14392|  invoke void @ai::small_action5traceNtB2_16SmallActionTrace3new(ptr sret([152 x i8]) %26, ptr %5, i64 %339, i64 5)
 14393|  to label %417 unwind label %289, !!29557                                                                              ;L356<205
 14394| 
 14395| 417: ; preds = %416
 14396|  call void @llvm.memcpy.p0.p0.i64(ptr %27, ptr %26, i64 152, i1 false), !!29528                                        ;L356<205
 14397|  store i8 14, ptr %330, , !!29528                                                                                      ;L356<205
 14399|     ;; self = ptr %40
 14400|     ;; self = ptr %40
 14401|     ;; value = ptr %27
 14402|     ;; src = ptr %27
 14403|     ;; additional = i64 1
 14404|     ;; needed_extra_cap = i64 1
 14405|     ;; needed_extra_cap = i64 1
 14406|     ;; strategy = i8 1
 14407|  %418 = load i64, ptr %209, , !!30146, !!8                                                                             ;L1428<356<205
 14408|     ;; self = ptr %40
 14409|  %419 = load i64, ptr %208, , !!30146, !!8                                                                             ;L149<1428<356<205
 14410|  %420 = icmp eq i64 %418, %419                                                                                         ;L1428<356<205
 14411|  br i1 %420, label %421, label %426                                                                                    ;L1428<356<205
 14412| 
 14413| 421: ; preds = %417
 14414|     ;; self = ptr %40
 14415|     ;; self = ptr %40
 14416|     ;; self = ptr %40
 14417|     ;; used_cap = i64 %418
 14418|     ;; used_cap = i64 %418
 14419|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %40, i64 %418, i64 1, i1 zeroext true)
 14420|  to label %422 unwind label %424, !!30154                                                                              ;L619<430<738<1429<356<205
 14421| 
 14422| 422: ; preds = %421
 14423|  %423 = load i64, ptr %209, , !!30146                                                                                  ;L1432<356<205
 14424|  br label %426                                                                                                         ;L619<430<738<1429<356<205
 14425| 
 14426| 424: ; preds = %421
 14427|  %425 = cleanuppad within none []
 14428|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %27) #34 [ "funclet"(token %425) ], !!29557 ;L1436<356<205
 14429|  cleanupret from %425 unwind label %289
 14430| 
 14431| 426: ; preds = %422, %417
 14432|  %427 = phi i64 [ %423, %422 ], [ %418, %417 ]                                                                         ;L1434<356<205
 14433|     ;; self = ptr %40
 14434|  %428 = load ptr, ptr %40, , !!30146, !!8, !!8                                                                         ;L138<1432<356<205
 14435|     ;; self = ptr %428
 14436|     ;; count = i64 %427
 14437|  %429 = gepS %428, i64 %427                                                                                            ;L961<1432<356<205
 14438|     ;; end = ptr %429
 14439|     ;; dst = ptr %429
 14440|  call void @llvm.memcpy.p0.p0.i64(ptr %429, ptr %27, i64 184, i1 false), !!29557                                       ;L1933<1433<356<205
 14441|  %430 = add i64 %427, 1                                                                                                ;L1434<356<205
 14442|  store i64 %430, ptr %209, , !!30146                                                                                   ;L1434<356<205
 14444|  br label %431                                                                                                         ;L355<205
 14445| 
 14446| 431: ; preds = %474, %459, %448, %444, %442, %426, %415, %411
 14447|  %432 = phi i1 [ %333, %474 ], [ %333, %442 ], [ %333, %459 ], [ %333, %448 ], [ %414, %411 ], [ %447, %444 ], [ %333, %426 ], [ %333, %415 ] ;L0<205
 14450|  br label %331                                                                                                         ;L325<205
 14451| 
 14452| 433: ; preds = %400
 14453|  %434 = icmp ugt i64 %401, 10                                                                                          ;L337<205
 14454|  %435 = icmp ugt i64 %348, 999                                                                                         ;L337<205
 14455|  %436 = and i1 %435, %434                                                                                              ;L337<205
 14456|  %437 = mul i64 %349, %349                                                                                             ;L0<205
 14457|  %438 = icmp ugt i64 %376, %437                                                                                        ;L0<205
 14458|  br i1 %436, label %442, label %439                                                                                    ;L337<205
 14459| 
 14460| 439: ; preds = %433
 14461|  %440 = icmp samesign ult i64 %355, %349                                                                               ;L341<205
 14462|  %441 = and i1 %440, %438                                                                                              ;L341<205
 14463|  br i1 %441, label %448, label %444                                                                                    ;L341<205
 14464| 
 14465| 442: ; preds = %433
 14466|  %443 = and i1 %438, %393                                                                                              ;L338<205
 14467|  br i1 %443, label %464, label %431                                                                                    ;L338<205
 14468| 
 14469| 444: ; preds = %439
 14470|  %445 = mul i64 %361, %361                                                                                             ;L346<205
 14471|  %446 = icmp ule i64 %376, %445                                                                                        ;L346<205
 14472|  %447 = select i1 %446, i1 true, i1 %333                                                                               ;L346<205
 14473|  br label %431                                                                                                         ;L346<205
 14474| 
 14475| 448: ; preds = %439
 14476|  br i1 %393, label %449, label %431                                                                                    ;L343<205
 14477| 
 14478| 449: ; preds = %448
 14481|  invoke void @ai::small_action5traceNtB2_16SmallActionTrace3new(ptr sret([152 x i8]) %28, ptr %5, i64 %339, i64 5)
 14482|  to label %450 unwind label %289, !!29557                                                                              ;L344<205
 14483| 
 14484| 450: ; preds = %449
 14485|  call void @llvm.memcpy.p0.p0.i64(ptr %29, ptr %28, i64 152, i1 false), !!29528                                        ;L344<205
 14486|  store i8 14, ptr %328, , !!29528                                                                                      ;L344<205
 14488|     ;; self = ptr %40
 14489|     ;; self = ptr %40
 14490|     ;; value = ptr %29
 14491|     ;; src = ptr %29
 14492|     ;; additional = i64 1
 14493|     ;; needed_extra_cap = i64 1
 14494|     ;; needed_extra_cap = i64 1
 14495|     ;; strategy = i8 1
 14496|  %451 = load i64, ptr %209, , !!30187, !!8                                                                             ;L1428<344<205
 14497|     ;; self = ptr %40
 14498|  %452 = load i64, ptr %208, , !!30187, !!8                                                                             ;L149<1428<344<205
 14499|  %453 = icmp eq i64 %451, %452                                                                                         ;L1428<344<205
 14500|  br i1 %453, label %454, label %459                                                                                    ;L1428<344<205
 14501| 
 14502| 454: ; preds = %450
 14503|     ;; self = ptr %40
 14504|     ;; self = ptr %40
 14505|     ;; self = ptr %40
 14506|     ;; used_cap = i64 %451
 14507|     ;; used_cap = i64 %451
 14508|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %40, i64 %451, i64 1, i1 zeroext true)
 14509|  to label %455 unwind label %457, !!30195                                                                              ;L619<430<738<1429<344<205
 14510| 
 14511| 455: ; preds = %454
 14512|  %456 = load i64, ptr %209, , !!30187                                                                                  ;L1432<344<205
 14513|  br label %459                                                                                                         ;L619<430<738<1429<344<205
 14514| 
 14515| 457: ; preds = %454
 14516|  %458 = cleanuppad within none []
 14517|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %29) #34 [ "funclet"(token %458) ], !!29557 ;L1436<344<205
 14518|  cleanupret from %458 unwind label %289
 14519| 
 14520| 459: ; preds = %455, %450
 14521|  %460 = phi i64 [ %456, %455 ], [ %451, %450 ]                                                                         ;L1434<344<205
 14522|     ;; self = ptr %40
 14523|  %461 = load ptr, ptr %40, , !!30187, !!8, !!8                                                                         ;L138<1432<344<205
 14524|     ;; self = ptr %461
 14525|     ;; count = i64 %460
 14526|  %462 = gepS %461, i64 %460                                                                                            ;L961<1432<344<205
 14527|     ;; end = ptr %462
 14528|     ;; dst = ptr %462
 14529|  call void @llvm.memcpy.p0.p0.i64(ptr %462, ptr %29, i64 184, i1 false), !!29557                                       ;L1933<1433<344<205
 14530|  %463 = add i64 %460, 1                                                                                                ;L1434<344<205
 14531|  store i64 %463, ptr %209, , !!30187                                                                                   ;L1434<344<205
 14533|  br label %431                                                                                                         ;L343<205
 14534| 
 14535| 464: ; preds = %442
 14538|  invoke void @ai::small_action5traceNtB2_16SmallActionTrace3new(ptr sret([152 x i8]) %30, ptr %5, i64 %339, i64 5)
 14539|  to label %465 unwind label %289, !!29557                                                                              ;L339<205
 14540| 
 14541| 465: ; preds = %464
 14542|  call void @llvm.memcpy.p0.p0.i64(ptr %31, ptr %30, i64 152, i1 false), !!29528                                        ;L339<205
 14543|  store i8 14, ptr %329, , !!29528                                                                                      ;L339<205
 14545|     ;; self = ptr %40
 14546|     ;; self = ptr %40
 14547|     ;; value = ptr %31
 14548|     ;; src = ptr %31
 14549|     ;; additional = i64 1
 14550|     ;; needed_extra_cap = i64 1
 14551|     ;; needed_extra_cap = i64 1
 14552|     ;; strategy = i8 1
 14553|  %466 = load i64, ptr %209, , !!30223, !!8                                                                             ;L1428<339<205
 14554|     ;; self = ptr %40
 14555|  %467 = load i64, ptr %208, , !!30223, !!8                                                                             ;L149<1428<339<205
 14556|  %468 = icmp eq i64 %466, %467                                                                                         ;L1428<339<205
 14557|  br i1 %468, label %469, label %474                                                                                    ;L1428<339<205
 14558| 
 14559| 469: ; preds = %465
 14560|     ;; self = ptr %40
 14561|     ;; self = ptr %40
 14562|     ;; self = ptr %40
 14563|     ;; used_cap = i64 %466
 14564|     ;; used_cap = i64 %466
 14565|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %40, i64 %466, i64 1, i1 zeroext true)
 14566|  to label %470 unwind label %472, !!30231                                                                              ;L619<430<738<1429<339<205
 14567| 
 14568| 470: ; preds = %469
 14569|  %471 = load i64, ptr %209, , !!30223                                                                                  ;L1432<339<205
 14570|  br label %474                                                                                                         ;L619<430<738<1429<339<205
 14571| 
 14572| 472: ; preds = %469
 14573|  %473 = cleanuppad within none []
 14574|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %31) #34 [ "funclet"(token %473) ], !!29557 ;L1436<339<205
 14575|  cleanupret from %473 unwind label %289
 14576| 
 14577| 474: ; preds = %470, %465
 14578|  %475 = phi i64 [ %471, %470 ], [ %466, %465 ]                                                                         ;L1434<339<205
 14579|     ;; self = ptr %40
 14580|  %476 = load ptr, ptr %40, , !!30223, !!8, !!8                                                                         ;L138<1432<339<205
 14581|     ;; self = ptr %476
 14582|     ;; count = i64 %475
 14583|  %477 = gepS %476, i64 %475                                                                                            ;L961<1432<339<205
 14584|     ;; end = ptr %477
 14585|     ;; dst = ptr %477
 14586|  call void @llvm.memcpy.p0.p0.i64(ptr %477, ptr %31, i64 184, i1 false), !!29557                                       ;L1933<1433<339<205
 14587|  %478 = add i64 %475, 1                                                                                                ;L1434<339<205
 14588|  store i64 %478, ptr %209, , !!30223                                                                                   ;L1434<339<205
 14590|  br label %431                                                                                                         ;L338<205
 14591| 
 14592| 479: ; preds = %331
 14593|  %480 = gep %90, i64 240                                                                                               ;L365<205
 14594|  %481 = getelementptr { { ptr, ptr, i64 }, i64 }, ptr %480, i64 %97                                                    ;L365<205
 14595|     ;; self = ptr %481
 14596|     ;; self = ptr %481
 14597|  %482 = load ptr, ptr %481, , !!29557, !!8, !!8                                                                        ;L138<2073<365<205
 14598|     ;; p = ptr %482
 14599|  %483 = gep %481, i64 24                                                                                               ;L2075<365<205
 14600|  %484 = load i64, ptr %483, , !!29557, !!8                                                                             ;L2075<365<205
 14601|     ;; len = i64 %484
 14602|     ;; count = i64 %484
 14603|     ;; count = i64 %484
 14604|     ;; self[0..+8] = ptr %482
 14605|     ;; slice[0..+8] = ptr %482
 14606|     ;; self[0..+8] = ptr %482
 14607|     ;; slice[0..+8] = ptr %482
 14608|     ;; self[8..+8] = i64 %484
 14609|     ;; slice[8..+8] = i64 %484
 14610|     ;; self[8..+8] = i64 %484
 14611|     ;; slice[8..+8] = i64 %484
 14612|     ;; ptr = ptr %482
 14613|     ;; self = ptr %482
 14614|  %485 = getelementptr ptr, ptr %482, i64 %484                                                                          ;L961<100<1042<365<205
 14615|     ;; iter[0..+8] = ptr %482
 14616|     ;; iter[8..+8] = ptr %485
 14617|  %486 = gep %210, i64 1136
 14618|  %487 = gep %210, i64 1664
 14619|  br label %488                                                                                                         ;L365<205
 14620| 
 14621| 488: ; preds = %508, %479
 14622|  %489 = phi ptr [ %482, %479 ], [ %492, %508 ]                                                                         ;L365<205
 14623|     ;; iter[0..+8] = ptr %489
 14624|     ;; self = ptr undef
 14625|     ;; ptr = ptr %489
 14626|     ;; self = ptr %489
 14627|     ;; end_or_len = ptr %485
 14630|  %490 = icmp eq ptr %489, %485                                                                                         ;L1714<180<365<205
 14631|  br i1 %490, label %561, label %491                                                                                    ;L180<365<205
 14632| 
 14633| 491: ; preds = %488
 14634|  %492 = gep %489, i64 8                                                                                                ;L656<185<365<205
 14635|     ;; iter[0..+8] = ptr %492
 14636|  %493 = load ptr, ptr %489, , !!29557, !!8, !!8                                                                        ;L365<205
 14637|     ;; e = ptr %493
 14638|     ;; caster = ptr %493
 14639|     ;; self = ptr %493
 14640|     ;; other = ptr %493
 14641|     ;; self = ptr %493
 14642|  %494 = gep %493, i64 1216                                                                                             ;L742<366<205
 14643|  %495 = load i32, ptr %494, , !!29557, !!8                                                                             ;L742<366<205
 14644|  %496 = icmp eq i32 %495, -1                                                                                           ;L742<366<205
 14645|  br i1 %496, label %508, label %497                                                                                    ;L742<366<205
 14646| 
 14647| 497: ; preds = %491
 14648|  %498 = gep %493, i64 1168                                                                                             ;L742<366<205
 14649|     ;; atk = ptr %498
 14650|     ;; self = ptr %498
 14651|  %499 = gep %493, i64 1184                                                                                             ;L26<367<205
 14652|  %500 = load i64, ptr %499, , !!29557, !!8                                                                             ;L26<367<205
 14653|  %501 = gep %493, i64 1192                                                                                             ;L26<367<205
 14654|  %502 = load i64, ptr %501, , !!29557, !!8                                                                             ;L26<367<205
 14655|  %503 = gep %493, i64 1480                                                                                             ;L26<367<205
 14656|  %504 = load i64, ptr %503, , !!29557, !!8                                                                             ;L26<367<205
 14657|  %505 = gep %493, i64 1080                                                                                             ;L26<367<205
 14658|  %506 = load i64, ptr %505, , !!29557, !!8                                                                             ;L26<367<205
 14659|  %507 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %498, ptr %493, ptr %210)
 14660|  to label %509 unwind label %289, !!29557                                                                              ;L367<205
 14661| 
 14662| 508: ; preds = %537, %491
 14663|  br label %488                                                                                                         ;L365<205
 14664| 
 14665| 509: ; preds = %497
 14666|  %510 = add i64 %504, -1                                                                                               ;L26<367<205
 14667|  %511 = mul i64 %510, %502                                                                                             ;L26<367<205
 14668|  %512 = gep %493, i64 1136                                                                                             ;L1511<367<205
 14669|  %513 = load i32, ptr %512, , !!29557, !!8                                                                             ;L1511<367<205
 14670|     ;; mult = i32 %513
 14671|  %514 = icmp eq i32 %513, 0                                                                                            ;L1512<367<205
 14672|  br i1 %514, label %515, label %518                                                                                    ;L1512<367<205
 14673| 
 14674| 515: ; preds = %509
 14675|  %516 = gep %493, i64 1664                                                                                             ;L1513<367<205
 14676|  %517 = load i64, ptr %516, , !!29557, !!8                                                                             ;L1513<367<205
 14677|  br label %525                                                                                                         ;L1512<367<205
 14678| 
 14679| 518: ; preds = %509
 14680|  %519 = sext i32 %513 to i64                                                                                           ;L1511<367<205
 14681|     ;; mult = i64 %519
 14682|  %520 = gep %493, i64 1664                                                                                             ;L1515<367<205
 14683|  %521 = load i64, ptr %520, , !!29557, !!8                                                                             ;L1515<367<205
 14684|  %522 = add nsw i64 %519, 100                                                                                          ;L1515<367<205
 14685|  %523 = mul i64 %521, %522                                                                                             ;L1515<367<205
 14686|  %524 = udiv i64 %523, 100                                                                                             ;L1515<367<205
 14687|  br label %525                                                                                                         ;L1512<367<205
 14688| 
 14689| 525: ; preds = %518, %515
 14690|  %526 = phi i64 [ %517, %515 ], [ %524, %518 ]                                                                         ;L0<367<205
 14691|  %527 = load i32, ptr %486, , !!29557, !!8                                                                             ;L1511<367<205
 14692|     ;; mult = i32 %527
 14693|  %528 = icmp eq i32 %527, 0                                                                                            ;L1512<367<205
 14694|  br i1 %528, label %529, label %531                                                                                    ;L1512<367<205
 14695| 
 14696| 529: ; preds = %525
 14697|  %530 = load i64, ptr %487, , !!29557, !!8                                                                             ;L1513<367<205
 14698|  br label %537                                                                                                         ;L1512<367<205
 14699| 
 14700| 531: ; preds = %525
 14701|  %532 = sext i32 %527 to i64                                                                                           ;L1511<367<205
 14702|     ;; mult = i64 %532
 14703|  %533 = load i64, ptr %487, , !!29557, !!8                                                                             ;L1515<367<205
 14704|  %534 = add nsw i64 %532, 100                                                                                          ;L1515<367<205
 14705|  %535 = mul i64 %533, %534                                                                                             ;L1515<367<205
 14706|  %536 = udiv i64 %535, 100                                                                                             ;L1515<367<205
 14707|  br label %537                                                                                                         ;L1512<367<205
 14708| 
 14709| 537: ; preds = %531, %529
 14710|  %538 = phi i64 [ %530, %529 ], [ %536, %531 ]                                                                         ;L0<367<205
 14711|  %539 = add i64 %506, %500                                                                                             ;L26<367<205
 14712|  %540 = add i64 %539, %511                                                                                             ;L26<367<205
 14713|  %541 = add i64 %540, %507                                                                                             ;L367<205
 14714|  %542 = add i64 %541, %526                                                                                             ;L367<205
 14715|  %543 = add i64 %542, %538                                                                                             ;L367<205
 14716|     ;; range = i64 %543
 14717|  %544 = gep %493, i64 1632                                                                                             ;L2158<368<205
 14718|  %545 = load i64, ptr %544, , !!29557, !!8                                                                             ;L2158<368<205
 14719|     ;; x2 = i64 %545
 14720|     ;; other = i64 %545
 14721|  %546 = gep %493, i64 1640                                                                                             ;L2158<368<205
 14722|  %547 = load i64, ptr %546, , !!29557, !!8                                                                             ;L2158<368<205
 14723|     ;; y2 = i64 %547
 14724|     ;; other = i64 %547
 14725|  %548 = icmp ult i64 %268, %545                                                                                        ;L3147<7<2158<368<205
 14726|  %549 = sub nuw i64 %545, %268                                                                                         ;L3147<7<2158<368<205
 14727|  %550 = sub nuw i64 %268, %545                                                                                         ;L3147<7<2158<368<205
 14728|  %551 = select i1 %548, i64 %549, i64 %550                                                                             ;L3147<7<2158<368<205
 14729|     ;; dx = i64 %551
 14730|  %552 = icmp ult i64 %270, %547                                                                                        ;L3147<8<2158<368<205
 14731|  %553 = sub nuw i64 %547, %270                                                                                         ;L3147<8<2158<368<205
 14732|  %554 = sub nuw i64 %270, %547                                                                                         ;L3147<8<2158<368<205
 14733|  %555 = select i1 %552, i64 %553, i64 %554                                                                             ;L3147<8<2158<368<205
 14734|     ;; dy = i64 %555
 14735|  %556 = mul i64 %551, %551                                                                                             ;L9<2158<368<205
 14736|  %557 = mul i64 %555, %555                                                                                             ;L9<2158<368<205
 14737|  %558 = add i64 %557, %556                                                                                             ;L9<2158<368<205
 14738|  %559 = mul i64 %543, %543                                                                                             ;L368<205
 14739|  %560 = icmp ugt i64 %558, %559                                                                                        ;L368<205
 14740|  br i1 %560, label %508, label %561                                                                                    ;L368<205
 14741| 
 14742| 561: ; preds = %537, %488
 14743|  %562 = phi i1 [ %333, %488 ], [ true, %537 ]                                                                          ;L0<205
 14745|     ;; self = ptr %40
 14746|     ;; self = ptr %40
 14747|  %563 = load i64, ptr %209, , !!29528, !!8                                                                             ;L1617<1636<377<205
 14748|  %564 = icmp eq i64 %563, 0                                                                                            ;L377<205
 14749|  br i1 %564, label %565, label %567                                                                                    ;L377<205
 14750| 
 14751| 565: ; preds = %561
 14752|  %566 = invoke { i64, ptr } %299(ptr %296)
 14753|  to label %568 unwind label %289, !!29557                                                                              ;L378<205
 14754| 
 14755| 567: ; preds = %936, %903, %751, %561
 14756|  br i1 %562, label %947, label %942                                                                                    ;L411<205
 14757| 
 14758| 568: ; preds = %565
 14759|  %569 = extractvalue { i64, ptr } %566, 0                                                                              ;L378<205
 14760|     ;; self[0..+8] = i64 %569
 14762|  %570 = icmp eq i64 %569, 0                                                                                            ;L231<378<205
 14763|  br i1 %570, label %571, label %941                                                                                    ;L231<378<205
 14764| 
 14765| 571: ; preds = %568
 14766|  %572 = extractvalue { i64, ptr } %566, 1                                                                              ;L378<205
 14767|     ;; self[8..+8] = ptr %572
 14768|  %573 = icmp ne ptr %572, null
 14769|  call void @llvm.assume(i1 %573)
 14770|     ;; self = ptr %572
 14771|     ;; self = ptr %572
 14772|     ;; self = ptr %572
 14773|     ;; self = ptr %572
 14774|  %574 = gep %572, i64 472                                                                                              ;L1864<3787<378<205
 14775|  %575 = load i64, ptr %574, , !!29557, !!8                                                                             ;L1864<3787<378<205
 14778|     ;; self[8..+8] = i64 %575
 14779|     ;; slice[8..+8] = i64 %575
 14780|  %576 = icmp eq i64 %575, 0                                                                                            ;L219<576<378<205
 14781|  br i1 %576, label %584, label %577                                                                                    ;L219<576<378<205
 14782| 
 14783| 577: ; preds = %571
 14784|  %578 = gep %572, i64 464                                                                                              ;L614<609<296<1968<1864<3787<378<205
 14785|  %579 = load ptr, ptr %578, , !!29557, !!8, !!8                                                                        ;L614<609<296<1968<1864<3787<378<205
 14786|     ;; self[0..+8] = ptr %579
 14787|     ;; slice[0..+8] = ptr %579
 14788|     ;; self = ptr %579
 14789|     ;; f[0..+8] = ptr %296
 14790|     ;; f[8..+8] = ptr %297
 14791|     ;; x = ptr %579
 14792|  %580 = gep %297, i64 496                                                                                              ;L1543<378<205
 14793|  %581 = load ptr, ptr %580, , !!29557                                                                                  ;L1543<378<205
 14794|  %582 = load i64, ptr %579, , !!29557, !!8                                                                             ;L1543<378<205
 14796|  %583 = invoke ptr %581(ptr %296, i64 %582)
 14797|  to label %585 unwind label %289, !!29557                                                                              ;L378<1543<378<205
 14798| 
 14799| 584: ; preds = %585, %571
 14802|  invoke void @ai::small_action6aroundNtB5_23SmallActionAroundRegion3new(ptr sret([120 x i8]) %16, i64 %205, ptr %3, ptr %5, ptr %4, i64 2, i64 5)
 14803|  to label %736 unwind label %289, !!29557                                                                              ;L398<205
 14804| 
 14805| 585: ; preds = %577
 14806|  %586 = icmp eq ptr %583, null                                                                                         ;L378<205
 14807|  br i1 %586, label %584, label %587                                                                                    ;L378<205
 14808| 
 14809| 587: ; preds = %585
 14810|     ;; serpen = ptr %583
 14811|     ;; self = ptr %583
 14812|     ;; self = ptr %583
 14813|     ;; self = ptr %583
 14814|     ;; other = ptr %583
 14815|  %588 = gep %583, i64 104                                                                                              ;L379<205
 14816|  %589 = load i64, ptr %588, , !!29557, !!8                                                                             ;L379<205
 14817|  %590 = icmp eq i64 %589, 6                                                                                            ;L379<205
 14818|  br i1 %590, label %591, label %594                                                                                    ;L379<205
 14819| 
 14820| 591: ; preds = %587
 14821|     ;; self = ptr %210
 14822|  %592 = load i64, ptr %210, , !!29557, !!8                                                                             ;L1136<1482<380<205
 14823|  %593 = trunc nuw i64 %592 to i1                                                                                       ;L1136<1482<380<205
 14824|  br i1 %593, label %608, label %595                                                                                    ;L1136<1482<380<205
 14825| 
 14826| 594: ; preds = %587
 14829|  invoke void @ai::small_action6aroundNtB5_23SmallActionAroundRegion3new(ptr sret([120 x i8]) %18, i64 %205, ptr %3, ptr %5, ptr %4, i64 2, i64 5)
 14830|  to label %721 unwind label %289, !!29557                                                                              ;L395<205
 14831| 
 14832| 595: ; preds = %591
 14833|  %596 = gep %210, i64 8                                                                                                ;L1136<1482<380<205
 14834|     ;; team = ptr %210
 14835|  %597 = load i64, ptr %596, , !!29557, !!8                                                                             ;L1137<1482<380<205
 14836|     ;; team = i64 %597
 14837|  %598 = icmp ult i64 %597, 2                                                                                           ;L1483<380<205
 14838|  br i1 %598, label %599, label %604                                                                                    ;L1483<380<205
 14839| 
 14840| 599: ; preds = %595
 14842|  %600 = gep %583, i64 56                                                                                               ;L122<1483<380<205
 14843|  %601 = gepS %600, i64 %597                                                                                            ;L122<1483<380<205
 14844|  %602 = load i64, ptr %601, , !!29557, !!8                                                                             ;L122<1483<380<205
 14845|  %603 = icmp eq i64 %602, 0                                                                                            ;L122<1483<380<205
 14846|  br i1 %603, label %608, label %605                                                                                    ;L380<205
 14847| 
 14848| 604: ; preds = %595
 14849|  invoke void @core::panicking18panic_bounds_check(i64 %597, i64 2, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.26) #35
 14850|  to label %202 unwind label %289, !!29557                                                                              ;L1483<380<205
 14851| 
 14852| 605: ; preds = %608, %599
 14855|  %606 = gep %583, i64 1472                                                                                             ;L381<205
 14856|  %607 = load i64, ptr %606, , !!29557, !!8                                                                             ;L381<205
 14857|  invoke void @ai::small_action6aroundNtB2_17SmallActionAround3new(ptr sret([136 x i8]) %24, i64 %205, ptr %3, ptr %5, ptr %4, i64 %607, i64 5)
 14858|  to label %700 unwind label %289, !!29557                                                                              ;L381<205
 14859| 
 14860| 608: ; preds = %599, %591
 14861|  %609 = gep %583, i64 1632                                                                                             ;L2158<380<205
 14862|  %610 = load i64, ptr %609, , !!29557, !!8                                                                             ;L2158<380<205
 14863|     ;; x1 = i64 %610
 14864|     ;; self = i64 %610
 14865|     ;; x2 = i64 %610
 14866|     ;; other = i64 %610
 14867|  %611 = gep %583, i64 1640                                                                                             ;L2158<380<205
 14868|  %612 = load i64, ptr %611, , !!29557, !!8                                                                             ;L2158<380<205
 14869|     ;; y1 = i64 %612
 14870|     ;; self = i64 %612
 14871|     ;; y2 = i64 %612
 14872|     ;; other = i64 %612
 14873|  %613 = icmp ult i64 %610, %268                                                                                        ;L3147<7<2158<380<205
 14874|  %614 = sub nuw i64 %268, %610                                                                                         ;L3147<7<2158<380<205
 14875|  %615 = sub nuw i64 %610, %268                                                                                         ;L3147<7<2158<380<205
 14876|  %616 = select i1 %613, i64 %614, i64 %615                                                                             ;L3147<7<2158<380<205
 14877|     ;; dx = i64 %616
 14878|  %617 = icmp ult i64 %612, %270                                                                                        ;L3147<8<2158<380<205
 14879|  %618 = sub nuw i64 %270, %612                                                                                         ;L3147<8<2158<380<205
 14880|  %619 = sub nuw i64 %612, %270                                                                                         ;L3147<8<2158<380<205
 14881|  %620 = select i1 %617, i64 %618, i64 %619                                                                             ;L3147<8<2158<380<205
 14882|     ;; dy = i64 %620
 14883|  %621 = mul i64 %616, %616                                                                                             ;L9<2158<380<205
 14884|  %622 = mul i64 %620, %620                                                                                             ;L9<2158<380<205
 14885|  %623 = add i64 %622, %621                                                                                             ;L9<2158<380<205
 14886|  %624 = icmp ugt i64 %623, 22500000000                                                                                 ;L380<205
 14887|  br i1 %624, label %605, label %625                                                                                    ;L380<205
 14888| 
 14889| 625: ; preds = %608
 14890|     ;; self = ptr %583
 14891|  %626 = gep %583, i64 1168                                                                                             ;L742<383<205
 14892|  %627 = gep %583, i64 1216                                                                                             ;L742<383<205
 14893|  %628 = load i32, ptr %627, , !!29557, !!8                                                                             ;L742<383<205
 14894|  %629 = icmp eq i32 %628, -1                                                                                           ;L742<383<205
 14895|  br i1 %629, label %643, label %630                                                                                    ;L742<383<205
 14896| 
 14897| 630: ; preds = %625
 14898|     ;; self = ptr %626
 14899|     ;; self = ptr %626
 14900|  %631 = gep %583, i64 1184                                                                                             ;L26<383<205
 14901|  %632 = load i64, ptr %631, , !!29557, !!8                                                                             ;L26<383<205
 14902|  %633 = gep %583, i64 1192                                                                                             ;L26<383<205
 14903|  %634 = load i64, ptr %633, , !!29557, !!8                                                                             ;L26<383<205
 14904|  %635 = gep %210, i64 1480                                                                                             ;L26<383<205
 14905|  %636 = load i64, ptr %635, , !!29557, !!8                                                                             ;L26<383<205
 14906|  %637 = add i64 %636, -1                                                                                               ;L26<383<205
 14907|  %638 = mul i64 %637, %634                                                                                             ;L26<383<205
 14908|  %639 = gep %210, i64 1080                                                                                             ;L26<383<205
 14909|  %640 = load i64, ptr %639, , !!29557, !!8                                                                             ;L26<383<205
 14910|  %641 = load i32, ptr %486, , !!29557, !!8                                                                             ;L1511<383<205
 14911|     ;; mult = i32 %641
 14912|  %642 = icmp eq i32 %641, 0                                                                                            ;L1512<383<205
 14913|  br i1 %642, label %644, label %646                                                                                    ;L1512<383<205
 14914| 
 14915| 643: ; preds = %625
 14916|     ;; self = ptr null
 14917|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.90) #35
 14918|  to label %202 unwind label %289, !!29557                                                                              ;L1013<383<205
 14919| 
 14920| 644: ; preds = %630
 14921|  %645 = load i64, ptr %487, , !!29557, !!8                                                                             ;L1513<383<205
 14922|  br label %652                                                                                                         ;L1512<383<205
 14923| 
 14924| 646: ; preds = %630
 14925|  %647 = sext i32 %641 to i64                                                                                           ;L1511<383<205
 14926|     ;; mult = i64 %647
 14927|  %648 = load i64, ptr %487, , !!29557, !!8                                                                             ;L1515<383<205
 14928|  %649 = add nsw i64 %647, 100                                                                                          ;L1515<383<205
 14929|  %650 = mul i64 %648, %649                                                                                             ;L1515<383<205
 14930|  %651 = udiv i64 %650, 100                                                                                             ;L1515<383<205
 14931|  br label %652                                                                                                         ;L1512<383<205
 14932| 
 14933| 652: ; preds = %646, %644
 14934|  %653 = phi i64 [ %645, %644 ], [ %651, %646 ]                                                                         ;L0<383<205
 14935|  %654 = gep %583, i64 1136                                                                                             ;L1511<383<205
 14936|  %655 = load i32, ptr %654, , !!29557, !!8                                                                             ;L1511<383<205
 14937|     ;; mult = i32 %655
 14938|  %656 = icmp eq i32 %655, 0                                                                                            ;L1512<383<205
 14939|  br i1 %656, label %657, label %660                                                                                    ;L1512<383<205
 14940| 
 14941| 657: ; preds = %652
 14942|  %658 = gep %583, i64 1664                                                                                             ;L1513<383<205
 14943|  %659 = load i64, ptr %658, , !!29557, !!8                                                                             ;L1513<383<205
 14944|  br label %667                                                                                                         ;L1512<383<205
 14945| 
 14946| 660: ; preds = %652
 14947|  %661 = sext i32 %655 to i64                                                                                           ;L1511<383<205
 14948|     ;; mult = i64 %661
 14949|  %662 = gep %583, i64 1664                                                                                             ;L1515<383<205
 14950|  %663 = load i64, ptr %662, , !!29557, !!8                                                                             ;L1515<383<205
 14951|  %664 = add nsw i64 %661, 100                                                                                          ;L1515<383<205
 14952|  %665 = mul i64 %663, %664                                                                                             ;L1515<383<205
 14953|  %666 = udiv i64 %665, 100                                                                                             ;L1515<383<205
 14954|  br label %667                                                                                                         ;L1512<383<205
 14955| 
 14956| 667: ; preds = %660, %657
 14957|  %668 = phi i64 [ %659, %657 ], [ %666, %660 ]                                                                         ;L0<383<205
 14958|  %669 = add i64 %632, 20000                                                                                            ;L26<383<205
 14959|  %670 = add i64 %669, %640                                                                                             ;L26<383<205
 14960|  %671 = add i64 %670, %638                                                                                             ;L383<205
 14961|  %672 = add i64 %671, %653                                                                                             ;L383<205
 14962|  %673 = add i64 %672, %668                                                                                             ;L383<205
 14963|     ;; range = i64 %673
 14964|     ;; self = ptr %583
 14965|     ;; self = ptr %626
 14966|  %674 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %626, ptr %183, ptr %583, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.41, ptr %210)
 14967|  to label %675 unwind label %289, !!29557                                                                              ;L384<205
 14968| 
 14969| 675: ; preds = %667
 14970|     ;; dmg = i64 %674
 14971|  %676 = shl i64 %674, 1                                                                                                ;L386<205
 14972|  %677 = gep %210, i64 1648                                                                                             ;L386<205
 14973|  %678 = load i64, ptr %677, , !!29557, !!8                                                                             ;L386<205
 14974|  %679 = icmp ult i64 %676, %678                                                                                        ;L386<205
 14975|  br i1 %679, label %690, label %680                                                                                    ;L386<205
 14976| 
 14977| 680: ; preds = %675
 14978|  %681 = mul i64 %673, %673                                                                                             ;L386<205
 14979|  %682 = icmp ult i64 %268, %610                                                                                        ;L3147<7<2158<386<205
 14980|  %683 = select i1 %682, i64 %615, i64 %614                                                                             ;L3147<7<2158<386<205
 14981|     ;; dx = i64 %683
 14982|  %684 = icmp ult i64 %270, %612                                                                                        ;L3147<8<2158<386<205
 14983|  %685 = select i1 %684, i64 %619, i64 %618                                                                             ;L3147<8<2158<386<205
 14984|     ;; dy = i64 %685
 14985|  %686 = mul i64 %683, %683                                                                                             ;L9<2158<386<205
 14986|  %687 = mul i64 %685, %685                                                                                             ;L9<2158<386<205
 14987|  %688 = add i64 %687, %686                                                                                             ;L9<2158<386<205
 14988|  %689 = icmp ult i64 %681, %688                                                                                        ;L386<205
 14989|  br i1 %689, label %690, label %693                                                                                    ;L386<205
 14990| 
 14991| 690: ; preds = %680, %675
 14994|  %691 = gep %583, i64 1472                                                                                             ;L389<205
 14995|  %692 = load i64, ptr %691, , !!29557, !!8                                                                             ;L389<205
 14996|  invoke void @ai::small_action6aroundNtB2_17SmallActionAround3new(ptr sret([136 x i8]) %20, i64 %205, ptr %3, ptr %5, ptr %4, i64 %692, i64 5)
 14997|  to label %694 unwind label %289, !!29557                                                                              ;L389<205
 14998| 
 14999| 693: ; preds = %680
 15002|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway3new(ptr sret([136 x i8]) %22, ptr %5, ptr %4, i64 5)
 15003|  to label %697 unwind label %289, !!29557                                                                              ;L387<205
 15004| 
 15005| 694: ; preds = %690
 15006|  call void @llvm.memcpy.p0.p0.i64(ptr %21, ptr %20, i64 136, i1 false), !!29528                                        ;L389<205
 15007|  %695 = gep %21, i64 177                                                                                               ;L389<205
 15008|  store i8 5, ptr %695, , !!29528                                                                                       ;L389<205
 15010|  invoke fastcc void @ai::small_action15SmallActionPlayE4pushBX_(ptr %40, ptr %21)
 15011|  to label %696 unwind label %289, !!29557                                                                              ;L389<205
 15012| 
 15013| 696: ; preds = %694
 15015|  br label %715                                                                                                         ;L386<205
 15016| 
 15017| 697: ; preds = %693
 15018|  call void @llvm.memcpy.p0.p0.i64(ptr %23, ptr %22, i64 136, i1 false), !!29528                                        ;L387<205
 15019|  %698 = gep %23, i64 177                                                                                               ;L387<205
 15020|  store i8 3, ptr %698, , !!29528                                                                                       ;L387<205
 15022|  invoke fastcc void @ai::small_action15SmallActionPlayE4pushBX_(ptr %40, ptr %23)
 15023|  to label %699 unwind label %289, !!29557                                                                              ;L387<205
 15024| 
 15025| 699: ; preds = %697
 15027|  br label %715                                                                                                         ;L386<205
 15028| 
 15029| 700: ; preds = %605
 15030|  call void @llvm.memcpy.p0.p0.i64(ptr %25, ptr %24, i64 136, i1 false), !!29528                                        ;L381<205
 15031|  %701 = gep %25, i64 177                                                                                               ;L381<205
 15032|  store i8 5, ptr %701, , !!29528                                                                                       ;L381<205
 15034|     ;; self = ptr %40
 15035|     ;; self = ptr %40
 15036|     ;; value = ptr %25
 15037|     ;; src = ptr %25
 15038|     ;; additional = i64 1
 15039|     ;; needed_extra_cap = i64 1
 15040|     ;; needed_extra_cap = i64 1
 15041|     ;; strategy = i8 1
 15042|  %702 = load i64, ptr %209, , !!30467, !!8                                                                             ;L1428<381<205
 15043|     ;; self = ptr %40
 15044|  %703 = load i64, ptr %208, , !!30467, !!8                                                                             ;L149<1428<381<205
 15045|  %704 = icmp eq i64 %702, %703                                                                                         ;L1428<381<205
 15046|  br i1 %704, label %705, label %710                                                                                    ;L1428<381<205
 15047| 
 15048| 705: ; preds = %700
 15049|     ;; self = ptr %40
 15050|     ;; self = ptr %40
 15051|     ;; self = ptr %40
 15052|     ;; used_cap = i64 %702
 15053|     ;; used_cap = i64 %702
 15054|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %40, i64 %702, i64 1, i1 zeroext true)
 15055|  to label %706 unwind label %708, !!30475                                                                              ;L619<430<738<1429<381<205
 15056| 
 15057| 706: ; preds = %705
 15058|  %707 = load i64, ptr %209, , !!30467                                                                                  ;L1432<381<205
 15059|  br label %710                                                                                                         ;L619<430<738<1429<381<205
 15060| 
 15061| 708: ; preds = %705
 15062|  %709 = cleanuppad within none []
 15063|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %25) #34 [ "funclet"(token %709) ], !!29557 ;L1436<381<205
 15064|  cleanupret from %709 unwind label %289
 15065| 
 15066| 710: ; preds = %706, %700
 15067|  %711 = phi i64 [ %707, %706 ], [ %702, %700 ]                                                                         ;L1434<381<205
 15068|     ;; self = ptr %40
 15069|  %712 = load ptr, ptr %40, , !!30467, !!8, !!8                                                                         ;L138<1432<381<205
 15070|     ;; self = ptr %712
 15071|     ;; count = i64 %711
 15072|  %713 = gepS %712, i64 %711                                                                                            ;L961<1432<381<205
 15073|     ;; end = ptr %713
 15074|     ;; dst = ptr %713
 15075|  call void @llvm.memcpy.p0.p0.i64(ptr %713, ptr %25, i64 184, i1 false), !!29557                                       ;L1933<1433<381<205
 15076|  %714 = add i64 %711, 1                                                                                                ;L1434<381<205
 15077|  store i64 %714, ptr %209, , !!30467                                                                                   ;L1434<381<205
 15079|  br label %715                                                                                                         ;L380<205
 15080| 
 15081| 715: ; preds = %746, %731, %710, %699, %696
 15082|  %716 = gep %210, i64 1472                                                                                             ;L401<205
 15083|  %717 = load i64, ptr %716, , !!29557, !!8                                                                             ;L401<205
 15084|  %718 = gep %297, i64 248                                                                                              ;L401<205
 15085|  %719 = load ptr, ptr %718, , !!29557, !!8                                                                             ;L401<205
 15086|  %720 = invoke zeroext i1 %719(ptr %296, i64 %97, i64 %717)
 15087|  to label %751 unwind label %289, !!29557                                                                              ;L401<205
 15088| 
 15089| 721: ; preds = %594
 15090|  call void @llvm.memcpy.p0.p0.i64(ptr %19, ptr %18, i64 120, i1 false), !!29528                                        ;L395<205
 15091|  %722 = gep %19, i64 177                                                                                               ;L395<205
 15092|  store i8 7, ptr %722, , !!29528                                                                                       ;L395<205
 15094|     ;; self = ptr %40
 15095|     ;; self = ptr %40
 15096|     ;; value = ptr %19
 15097|     ;; src = ptr %19
 15098|     ;; additional = i64 1
 15099|     ;; needed_extra_cap = i64 1
 15100|     ;; needed_extra_cap = i64 1
 15101|     ;; strategy = i8 1
 15102|  %723 = load i64, ptr %209, , !!30503, !!8                                                                             ;L1428<395<205
 15103|     ;; self = ptr %40
 15104|  %724 = load i64, ptr %208, , !!30503, !!8                                                                             ;L149<1428<395<205
 15105|  %725 = icmp eq i64 %723, %724                                                                                         ;L1428<395<205
 15106|  br i1 %725, label %726, label %731                                                                                    ;L1428<395<205
 15107| 
 15108| 726: ; preds = %721
 15109|     ;; self = ptr %40
 15110|     ;; self = ptr %40
 15111|     ;; self = ptr %40
 15112|     ;; used_cap = i64 %723
 15113|     ;; used_cap = i64 %723
 15114|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %40, i64 %723, i64 1, i1 zeroext true)
 15115|  to label %727 unwind label %729, !!30511                                                                              ;L619<430<738<1429<395<205
 15116| 
 15117| 727: ; preds = %726
 15118|  %728 = load i64, ptr %209, , !!30503                                                                                  ;L1432<395<205
 15119|  br label %731                                                                                                         ;L619<430<738<1429<395<205
 15120| 
 15121| 729: ; preds = %726
 15122|  %730 = cleanuppad within none []
 15123|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %19) #34 [ "funclet"(token %730) ], !!29557 ;L1436<395<205
 15124|  cleanupret from %730 unwind label %289
 15125| 
 15126| 731: ; preds = %727, %721
 15127|  %732 = phi i64 [ %728, %727 ], [ %723, %721 ]                                                                         ;L1434<395<205
 15128|     ;; self = ptr %40
 15129|  %733 = load ptr, ptr %40, , !!30503, !!8, !!8                                                                         ;L138<1432<395<205
 15130|     ;; self = ptr %733
 15131|     ;; count = i64 %732
 15132|  %734 = gepS %733, i64 %732                                                                                            ;L961<1432<395<205
 15133|     ;; end = ptr %734
 15134|     ;; dst = ptr %734
 15135|  call void @llvm.memcpy.p0.p0.i64(ptr %734, ptr %19, i64 184, i1 false), !!29557                                       ;L1933<1433<395<205
 15136|  %735 = add i64 %732, 1                                                                                                ;L1434<395<205
 15137|  store i64 %735, ptr %209, , !!30503                                                                                   ;L1434<395<205
 15139|  br label %715                                                                                                         ;L379<205
 15140| 
 15141| 736: ; preds = %584
 15142|  call void @llvm.memcpy.p0.p0.i64(ptr %17, ptr %16, i64 120, i1 false), !!29528                                        ;L398<205
 15143|  %737 = gep %17, i64 177                                                                                               ;L398<205
 15144|  store i8 7, ptr %737, , !!29528                                                                                       ;L398<205
 15146|     ;; self = ptr %40
 15147|     ;; self = ptr %40
 15148|     ;; value = ptr %17
 15149|     ;; src = ptr %17
 15150|     ;; additional = i64 1
 15151|     ;; needed_extra_cap = i64 1
 15152|     ;; needed_extra_cap = i64 1
 15153|     ;; strategy = i8 1
 15154|  %738 = load i64, ptr %209, , !!30538, !!8                                                                             ;L1428<398<205
 15155|     ;; self = ptr %40
 15156|  %739 = load i64, ptr %208, , !!30538, !!8                                                                             ;L149<1428<398<205
 15157|  %740 = icmp eq i64 %738, %739                                                                                         ;L1428<398<205
 15158|  br i1 %740, label %741, label %746                                                                                    ;L1428<398<205
 15159| 
 15160| 741: ; preds = %736
 15161|     ;; self = ptr %40
 15162|     ;; self = ptr %40
 15163|     ;; self = ptr %40
 15164|     ;; used_cap = i64 %738
 15165|     ;; used_cap = i64 %738
 15166|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %40, i64 %738, i64 1, i1 zeroext true)
 15167|  to label %742 unwind label %744, !!30546                                                                              ;L619<430<738<1429<398<205
 15168| 
 15169| 742: ; preds = %741
 15170|  %743 = load i64, ptr %209, , !!30538                                                                                  ;L1432<398<205
 15171|  br label %746                                                                                                         ;L619<430<738<1429<398<205
 15172| 
 15173| 744: ; preds = %741
 15174|  %745 = cleanuppad within none []
 15175|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %17) #34 [ "funclet"(token %745) ], !!29557 ;L1436<398<205
 15176|  cleanupret from %745 unwind label %289
 15177| 
 15178| 746: ; preds = %742, %736
 15179|  %747 = phi i64 [ %743, %742 ], [ %738, %736 ]                                                                         ;L1434<398<205
 15180|     ;; self = ptr %40
 15181|  %748 = load ptr, ptr %40, , !!30538, !!8, !!8                                                                         ;L138<1432<398<205
 15182|     ;; self = ptr %748
 15183|     ;; count = i64 %747
 15184|  %749 = gepS %748, i64 %747                                                                                            ;L961<1432<398<205
 15185|     ;; end = ptr %749
 15186|     ;; dst = ptr %749
 15187|  call void @llvm.memcpy.p0.p0.i64(ptr %749, ptr %17, i64 184, i1 false), !!29557                                       ;L1933<1433<398<205
 15188|  %750 = add i64 %747, 1                                                                                                ;L1434<398<205
 15189|  store i64 %750, ptr %209, , !!30538                                                                                   ;L1434<398<205
 15191|  br label %715                                                                                                         ;L378<205
 15192| 
 15193| 751: ; preds = %715
 15194|  br i1 %720, label %752, label %567                                                                                    ;L401<205
 15195| 
 15196| 752: ; preds = %751
 15197|     ;; self[0..+8] = ptr %213
 15198|     ;; slice[0..+8] = ptr %213
 15199|     ;; self[8..+8] = i64 5
 15200|     ;; slice[8..+8] = i64 5
 15201|     ;; self = ptr %213
 15202|     ;; self = ptr undef
 15203|     ;; self = ptr undef
 15204|     ;; f = ptr %210
 15205|     ;; fold = ptr %210
 15208|     ;; f[8..+8] = ptr %210
 15209|     ;; self = ptr undef
 15212|     ;; self = ptr undef
 15213|     ;; count = i64 1
 15214|     ;; ptr = ptr %213
 15215|     ;; self = ptr %213
 15216|     ;; end_or_len = ptr %214
 15219|  %753 = load i64, ptr %210, , !!30575
 15220|  %754 = gep %210, i64 8
 15221|  %755 = load i64, ptr %754, , !!30575
 15222|  %756 = load i64, ptr %267, , !!30575
 15223|  %757 = load i64, ptr %269, , !!30575
 15224|  %758 = icmp eq i64 %753, 0
 15225|     ;; x = ptr %213
 15226|  %759 = load ptr, ptr %213, , !!30579, !!8                                                                             ;L2494<138<2897<403<205
 15231|  %760 = icmp eq ptr %759, null                                                                                         ;L49<2494<138<2897<403<205
 15232|  br i1 %760, label %783, label %761                                                                                    ;L49<2494<138<2897<403<205
 15233| 
 15234| 761: ; preds = %752
 15235|     ;; x = ptr %759
 15238|     ;; x = ptr %759
 15240|     ;; c = ptr %759
 15241|     ;; self = ptr %759
 15242|     ;; self = ptr %759
 15243|     ;; self = ptr %759
 15244|     ;; other = ptr %210
 15245|     ;; other = ptr %210
 15246|  %762 = load i64, ptr %759, , !!30579, !!8                                                                             ;L1127<264<403<2893<50<2494<138<2897<403<205
 15247|  %763 = gep %759, i64 8                                                                                                ;L1127<264<403<2893<50<2494<138<2897<403<205
 15248|     ;; __self_discr = i64 %762
 15249|     ;; __arg1_discr = i64 %753
 15250|  %764 = icmp eq i64 %762, %753                                                                                         ;L1127<264<403<2893<50<2494<138<2897<403<205
 15251|  br i1 %764, label %765, label %766                                                                                    ;L1127<264<403<2893<50<2494<138<2897<403<205
 15252| 
 15253| 765: ; preds = %761
 15254|  br i1 %758, label %900, label %783                                                                                    ;L1127<264<403<2893<50<2494<138<2897<403<205
 15255| 
 15256| 766: ; preds = %900, %761
 15257|     ;; other = ptr %210
 15258|  %767 = gep %759, i64 1632                                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15259|  %768 = load i64, ptr %767, , !!30579, !!8                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15260|     ;; x1 = i64 %768
 15261|     ;; self = i64 %768
 15262|  %769 = gep %759, i64 1640                                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15263|  %770 = load i64, ptr %769, , !!30579, !!8                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15264|     ;; y1 = i64 %770
 15265|     ;; self = i64 %770
 15266|     ;; x2 = i64 %756
 15267|     ;; other = i64 %756
 15268|     ;; y2 = i64 %757
 15269|     ;; other = i64 %757
 15270|  %771 = icmp ult i64 %768, %756                                                                                        ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15271|  %772 = sub nuw i64 %756, %768                                                                                         ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15272|  %773 = sub nuw i64 %768, %756                                                                                         ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15273|  %774 = select i1 %771, i64 %772, i64 %773                                                                             ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15274|     ;; dx = i64 %774
 15275|  %775 = icmp ult i64 %770, %757                                                                                        ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15276|  %776 = sub nuw i64 %757, %770                                                                                         ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15277|  %777 = sub nuw i64 %770, %757                                                                                         ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15278|  %778 = select i1 %775, i64 %776, i64 %777                                                                             ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15279|     ;; dy = i64 %778
 15280|  %779 = mul i64 %774, %774                                                                                             ;L9<2158<403<2893<50<2494<138<2897<403<205
 15281|  %780 = mul i64 %778, %778                                                                                             ;L9<2158<403<2893<50<2494<138<2897<403<205
 15282|  %781 = add i64 %780, %779                                                                                             ;L9<2158<403<2893<50<2494<138<2897<403<205
 15283|  %782 = icmp ult i64 %781, 22500000001                                                                                 ;L403<2893<50<2494<138<2897<403<205
 15284|  br i1 %782, label %925, label %783                                                                                    ;L2494<138<2897<403<205
 15285| 
 15286| 783: ; preds = %900, %766, %765, %752
 15287|     ;; self = ptr undef
 15288|     ;; count = i64 1
 15289|     ;; ptr = !DIArgList(ptr %213, i64 8)
 15290|     ;; self = !DIArgList(ptr %213, i64 8)
 15291|     ;; end_or_len = ptr %214
 15294|  %784 = gep %213, i64 8                                                                                                ;L656<185<2493<138<2897<403<205
 15295|     ;; ptr = ptr %784
 15296|     ;; x = ptr %784
 15297|  %785 = load ptr, ptr %784, , !!30579, !!8                                                                             ;L2494<138<2897<403<205
 15302|  %786 = icmp eq ptr %785, null                                                                                         ;L49<2494<138<2897<403<205
 15303|  br i1 %786, label %812, label %787                                                                                    ;L49<2494<138<2897<403<205
 15304| 
 15305| 787: ; preds = %783
 15306|     ;; x = ptr %785
 15309|     ;; x = ptr %785
 15311|     ;; c = ptr %785
 15312|     ;; self = ptr %785
 15313|     ;; self = ptr %785
 15314|     ;; self = ptr %785
 15315|     ;; other = ptr %210
 15316|     ;; other = ptr %210
 15317|  %788 = load i64, ptr %785, , !!30579, !!8                                                                             ;L1127<264<403<2893<50<2494<138<2897<403<205
 15318|  %789 = gep %785, i64 8                                                                                                ;L1127<264<403<2893<50<2494<138<2897<403<205
 15319|     ;; __self_discr = i64 %788
 15320|     ;; __arg1_discr = i64 %753
 15321|  %790 = icmp eq i64 %788, %753                                                                                         ;L1127<264<403<2893<50<2494<138<2897<403<205
 15322|  br i1 %790, label %791, label %795                                                                                    ;L1127<264<403<2893<50<2494<138<2897<403<205
 15323| 
 15324| 791: ; preds = %787
 15325|  br i1 %758, label %792, label %812                                                                                    ;L1127<264<403<2893<50<2494<138<2897<403<205
 15326| 
 15327| 792: ; preds = %791
 15328|     ;; __self_0 = ptr %785
 15329|     ;; self = ptr %785
 15330|     ;; __arg1_0 = ptr %210
 15331|     ;; other = ptr %210
 15334|  %793 = load i64, ptr %789, , !!30579, !!8                                                                             ;L1878<2123<1127<264<403<2893<50<2494<138<2897<403<205
 15335|  %794 = icmp eq i64 %793, %755                                                                                         ;L1878<2123<1127<264<403<2893<50<2494<138<2897<403<205
 15336|  br i1 %794, label %812, label %795                                                                                    ;L403<2893<50<2494<138<2897<403<205
 15337| 
 15338| 795: ; preds = %792, %787
 15339|     ;; other = ptr %210
 15340|  %796 = gep %785, i64 1632                                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15341|  %797 = load i64, ptr %796, , !!30579, !!8                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15342|     ;; x1 = i64 %797
 15343|     ;; self = i64 %797
 15344|  %798 = gep %785, i64 1640                                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15345|  %799 = load i64, ptr %798, , !!30579, !!8                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15346|     ;; y1 = i64 %799
 15347|     ;; self = i64 %799
 15348|     ;; x2 = i64 %756
 15349|     ;; other = i64 %756
 15350|     ;; y2 = i64 %757
 15351|     ;; other = i64 %757
 15352|  %800 = icmp ult i64 %797, %756                                                                                        ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15353|  %801 = sub nuw i64 %756, %797                                                                                         ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15354|  %802 = sub nuw i64 %797, %756                                                                                         ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15355|  %803 = select i1 %800, i64 %801, i64 %802                                                                             ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15356|     ;; dx = i64 %803
 15357|  %804 = icmp ult i64 %799, %757                                                                                        ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15358|  %805 = sub nuw i64 %757, %799                                                                                         ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15359|  %806 = sub nuw i64 %799, %757                                                                                         ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15360|  %807 = select i1 %804, i64 %805, i64 %806                                                                             ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15361|     ;; dy = i64 %807
 15362|  %808 = mul i64 %803, %803                                                                                             ;L9<2158<403<2893<50<2494<138<2897<403<205
 15363|  %809 = mul i64 %807, %807                                                                                             ;L9<2158<403<2893<50<2494<138<2897<403<205
 15364|  %810 = add i64 %809, %808                                                                                             ;L9<2158<403<2893<50<2494<138<2897<403<205
 15365|  %811 = icmp ult i64 %810, 22500000001                                                                                 ;L403<2893<50<2494<138<2897<403<205
 15366|  br i1 %811, label %925, label %812                                                                                    ;L2494<138<2897<403<205
 15367| 
 15368| 812: ; preds = %795, %792, %791, %783
 15369|     ;; self = ptr undef
 15370|     ;; count = i64 1
 15371|     ;; ptr = !DIArgList(ptr %213, i64 16)
 15372|     ;; self = !DIArgList(ptr %213, i64 16)
 15373|     ;; end_or_len = ptr %214
 15376|  %813 = gep %213, i64 16                                                                                               ;L656<185<2493<138<2897<403<205
 15377|     ;; ptr = ptr %813
 15378|     ;; x = ptr %813
 15379|  %814 = load ptr, ptr %813, , !!30579, !!8                                                                             ;L2494<138<2897<403<205
 15384|  %815 = icmp eq ptr %814, null                                                                                         ;L49<2494<138<2897<403<205
 15385|  br i1 %815, label %841, label %816                                                                                    ;L49<2494<138<2897<403<205
 15386| 
 15387| 816: ; preds = %812
 15388|     ;; x = ptr %814
 15391|     ;; x = ptr %814
 15393|     ;; c = ptr %814
 15394|     ;; self = ptr %814
 15395|     ;; self = ptr %814
 15396|     ;; self = ptr %814
 15397|     ;; other = ptr %210
 15398|     ;; other = ptr %210
 15399|  %817 = load i64, ptr %814, , !!30579, !!8                                                                             ;L1127<264<403<2893<50<2494<138<2897<403<205
 15400|  %818 = gep %814, i64 8                                                                                                ;L1127<264<403<2893<50<2494<138<2897<403<205
 15401|     ;; __self_discr = i64 %817
 15402|     ;; __arg1_discr = i64 %753
 15403|  %819 = icmp eq i64 %817, %753                                                                                         ;L1127<264<403<2893<50<2494<138<2897<403<205
 15404|  br i1 %819, label %820, label %824                                                                                    ;L1127<264<403<2893<50<2494<138<2897<403<205
 15405| 
 15406| 820: ; preds = %816
 15407|  br i1 %758, label %821, label %841                                                                                    ;L1127<264<403<2893<50<2494<138<2897<403<205
 15408| 
 15409| 821: ; preds = %820
 15410|     ;; __self_0 = ptr %814
 15411|     ;; self = ptr %814
 15412|     ;; __arg1_0 = ptr %210
 15413|     ;; other = ptr %210
 15416|  %822 = load i64, ptr %818, , !!30579, !!8                                                                             ;L1878<2123<1127<264<403<2893<50<2494<138<2897<403<205
 15417|  %823 = icmp eq i64 %822, %755                                                                                         ;L1878<2123<1127<264<403<2893<50<2494<138<2897<403<205
 15418|  br i1 %823, label %841, label %824                                                                                    ;L403<2893<50<2494<138<2897<403<205
 15419| 
 15420| 824: ; preds = %821, %816
 15421|     ;; other = ptr %210
 15422|  %825 = gep %814, i64 1632                                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15423|  %826 = load i64, ptr %825, , !!30579, !!8                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15424|     ;; x1 = i64 %826
 15425|     ;; self = i64 %826
 15426|  %827 = gep %814, i64 1640                                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15427|  %828 = load i64, ptr %827, , !!30579, !!8                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15428|     ;; y1 = i64 %828
 15429|     ;; self = i64 %828
 15430|     ;; x2 = i64 %756
 15431|     ;; other = i64 %756
 15432|     ;; y2 = i64 %757
 15433|     ;; other = i64 %757
 15434|  %829 = icmp ult i64 %826, %756                                                                                        ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15435|  %830 = sub nuw i64 %756, %826                                                                                         ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15436|  %831 = sub nuw i64 %826, %756                                                                                         ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15437|  %832 = select i1 %829, i64 %830, i64 %831                                                                             ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15438|     ;; dx = i64 %832
 15439|  %833 = icmp ult i64 %828, %757                                                                                        ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15440|  %834 = sub nuw i64 %757, %828                                                                                         ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15441|  %835 = sub nuw i64 %828, %757                                                                                         ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15442|  %836 = select i1 %833, i64 %834, i64 %835                                                                             ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15443|     ;; dy = i64 %836
 15444|  %837 = mul i64 %832, %832                                                                                             ;L9<2158<403<2893<50<2494<138<2897<403<205
 15445|  %838 = mul i64 %836, %836                                                                                             ;L9<2158<403<2893<50<2494<138<2897<403<205
 15446|  %839 = add i64 %838, %837                                                                                             ;L9<2158<403<2893<50<2494<138<2897<403<205
 15447|  %840 = icmp ult i64 %839, 22500000001                                                                                 ;L403<2893<50<2494<138<2897<403<205
 15448|  br i1 %840, label %925, label %841                                                                                    ;L2494<138<2897<403<205
 15449| 
 15450| 841: ; preds = %824, %821, %820, %812
 15451|     ;; self = ptr undef
 15452|     ;; count = i64 1
 15453|     ;; ptr = !DIArgList(ptr %213, i64 24)
 15454|     ;; self = !DIArgList(ptr %213, i64 24)
 15455|     ;; end_or_len = ptr %214
 15458|  %842 = gep %213, i64 24                                                                                               ;L656<185<2493<138<2897<403<205
 15459|     ;; ptr = ptr %842
 15460|     ;; x = ptr %842
 15461|  %843 = load ptr, ptr %842, , !!30579, !!8                                                                             ;L2494<138<2897<403<205
 15466|  %844 = icmp eq ptr %843, null                                                                                         ;L49<2494<138<2897<403<205
 15467|  br i1 %844, label %870, label %845                                                                                    ;L49<2494<138<2897<403<205
 15468| 
 15469| 845: ; preds = %841
 15470|     ;; x = ptr %843
 15473|     ;; x = ptr %843
 15475|     ;; c = ptr %843
 15476|     ;; self = ptr %843
 15477|     ;; self = ptr %843
 15478|     ;; self = ptr %843
 15479|     ;; other = ptr %210
 15480|     ;; other = ptr %210
 15481|  %846 = load i64, ptr %843, , !!30579, !!8                                                                             ;L1127<264<403<2893<50<2494<138<2897<403<205
 15482|  %847 = gep %843, i64 8                                                                                                ;L1127<264<403<2893<50<2494<138<2897<403<205
 15483|     ;; __self_discr = i64 %846
 15484|     ;; __arg1_discr = i64 %753
 15485|  %848 = icmp eq i64 %846, %753                                                                                         ;L1127<264<403<2893<50<2494<138<2897<403<205
 15486|  br i1 %848, label %849, label %853                                                                                    ;L1127<264<403<2893<50<2494<138<2897<403<205
 15487| 
 15488| 849: ; preds = %845
 15489|  br i1 %758, label %850, label %870                                                                                    ;L1127<264<403<2893<50<2494<138<2897<403<205
 15490| 
 15491| 850: ; preds = %849
 15492|     ;; __self_0 = ptr %843
 15493|     ;; self = ptr %843
 15494|     ;; __arg1_0 = ptr %210
 15495|     ;; other = ptr %210
 15498|  %851 = load i64, ptr %847, , !!30579, !!8                                                                             ;L1878<2123<1127<264<403<2893<50<2494<138<2897<403<205
 15499|  %852 = icmp eq i64 %851, %755                                                                                         ;L1878<2123<1127<264<403<2893<50<2494<138<2897<403<205
 15500|  br i1 %852, label %870, label %853                                                                                    ;L403<2893<50<2494<138<2897<403<205
 15501| 
 15502| 853: ; preds = %850, %845
 15503|     ;; other = ptr %210
 15504|  %854 = gep %843, i64 1632                                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15505|  %855 = load i64, ptr %854, , !!30579, !!8                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15506|     ;; x1 = i64 %855
 15507|     ;; self = i64 %855
 15508|  %856 = gep %843, i64 1640                                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15509|  %857 = load i64, ptr %856, , !!30579, !!8                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15510|     ;; y1 = i64 %857
 15511|     ;; self = i64 %857
 15512|     ;; x2 = i64 %756
 15513|     ;; other = i64 %756
 15514|     ;; y2 = i64 %757
 15515|     ;; other = i64 %757
 15516|  %858 = icmp ult i64 %855, %756                                                                                        ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15517|  %859 = sub nuw i64 %756, %855                                                                                         ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15518|  %860 = sub nuw i64 %855, %756                                                                                         ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15519|  %861 = select i1 %858, i64 %859, i64 %860                                                                             ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15520|     ;; dx = i64 %861
 15521|  %862 = icmp ult i64 %857, %757                                                                                        ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15522|  %863 = sub nuw i64 %757, %857                                                                                         ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15523|  %864 = sub nuw i64 %857, %757                                                                                         ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15524|  %865 = select i1 %862, i64 %863, i64 %864                                                                             ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15525|     ;; dy = i64 %865
 15526|  %866 = mul i64 %861, %861                                                                                             ;L9<2158<403<2893<50<2494<138<2897<403<205
 15527|  %867 = mul i64 %865, %865                                                                                             ;L9<2158<403<2893<50<2494<138<2897<403<205
 15528|  %868 = add i64 %867, %866                                                                                             ;L9<2158<403<2893<50<2494<138<2897<403<205
 15529|  %869 = icmp ult i64 %868, 22500000001                                                                                 ;L403<2893<50<2494<138<2897<403<205
 15530|  br i1 %869, label %925, label %870                                                                                    ;L2494<138<2897<403<205
 15531| 
 15532| 870: ; preds = %853, %850, %849, %841
 15533|     ;; self = ptr undef
 15534|     ;; count = i64 1
 15535|     ;; ptr = !DIArgList(ptr %213, i64 32)
 15536|     ;; self = !DIArgList(ptr %213, i64 32)
 15537|     ;; end_or_len = ptr %214
 15540|  %871 = gep %213, i64 32                                                                                               ;L656<185<2493<138<2897<403<205
 15541|     ;; ptr = ptr %871
 15542|     ;; x = ptr %871
 15543|  %872 = load ptr, ptr %871, , !!30579, !!8                                                                             ;L2494<138<2897<403<205
 15548|  %873 = icmp eq ptr %872, null                                                                                         ;L49<2494<138<2897<403<205
 15549|  br i1 %873, label %899, label %874                                                                                    ;L49<2494<138<2897<403<205
 15550| 
 15551| 874: ; preds = %870
 15552|     ;; x = ptr %872
 15555|     ;; x = ptr %872
 15557|     ;; c = ptr %872
 15558|     ;; self = ptr %872
 15559|     ;; self = ptr %872
 15560|     ;; self = ptr %872
 15561|     ;; other = ptr %210
 15562|     ;; other = ptr %210
 15563|  %875 = load i64, ptr %872, , !!30579, !!8                                                                             ;L1127<264<403<2893<50<2494<138<2897<403<205
 15564|  %876 = gep %872, i64 8                                                                                                ;L1127<264<403<2893<50<2494<138<2897<403<205
 15565|     ;; __self_discr = i64 %875
 15566|     ;; __arg1_discr = i64 %753
 15567|  %877 = icmp eq i64 %875, %753                                                                                         ;L1127<264<403<2893<50<2494<138<2897<403<205
 15568|  br i1 %877, label %878, label %882                                                                                    ;L1127<264<403<2893<50<2494<138<2897<403<205
 15569| 
 15570| 878: ; preds = %874
 15571|  br i1 %758, label %879, label %899                                                                                    ;L1127<264<403<2893<50<2494<138<2897<403<205
 15572| 
 15573| 879: ; preds = %878
 15574|     ;; __self_0 = ptr %872
 15575|     ;; self = ptr %872
 15576|     ;; __arg1_0 = ptr %210
 15577|     ;; other = ptr %210
 15580|  %880 = load i64, ptr %876, , !!30579, !!8                                                                             ;L1878<2123<1127<264<403<2893<50<2494<138<2897<403<205
 15581|  %881 = icmp eq i64 %880, %755                                                                                         ;L1878<2123<1127<264<403<2893<50<2494<138<2897<403<205
 15582|  br i1 %881, label %899, label %882                                                                                    ;L403<2893<50<2494<138<2897<403<205
 15583| 
 15584| 882: ; preds = %879, %874
 15585|     ;; other = ptr %210
 15586|  %883 = gep %872, i64 1632                                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15587|  %884 = load i64, ptr %883, , !!30579, !!8                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15588|     ;; x1 = i64 %884
 15589|     ;; self = i64 %884
 15590|  %885 = gep %872, i64 1640                                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15591|  %886 = load i64, ptr %885, , !!30579, !!8                                                                             ;L2158<403<2893<50<2494<138<2897<403<205
 15592|     ;; y1 = i64 %886
 15593|     ;; self = i64 %886
 15594|     ;; x2 = i64 %756
 15595|     ;; other = i64 %756
 15596|     ;; y2 = i64 %757
 15597|     ;; other = i64 %757
 15598|  %887 = icmp ult i64 %884, %756                                                                                        ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15599|  %888 = sub nuw i64 %756, %884                                                                                         ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15600|  %889 = sub nuw i64 %884, %756                                                                                         ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15601|  %890 = select i1 %887, i64 %888, i64 %889                                                                             ;L3147<7<2158<403<2893<50<2494<138<2897<403<205
 15602|     ;; dx = i64 %890
 15603|  %891 = icmp ult i64 %886, %757                                                                                        ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15604|  %892 = sub nuw i64 %757, %886                                                                                         ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15605|  %893 = sub nuw i64 %886, %757                                                                                         ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15606|  %894 = select i1 %891, i64 %892, i64 %893                                                                             ;L3147<8<2158<403<2893<50<2494<138<2897<403<205
 15607|     ;; dy = i64 %894
 15608|  %895 = mul i64 %890, %890                                                                                             ;L9<2158<403<2893<50<2494<138<2897<403<205
 15609|  %896 = mul i64 %894, %894                                                                                             ;L9<2158<403<2893<50<2494<138<2897<403<205
 15610|  %897 = add i64 %896, %895                                                                                             ;L9<2158<403<2893<50<2494<138<2897<403<205
 15611|  %898 = icmp ult i64 %897, 22500000001                                                                                 ;L403<2893<50<2494<138<2897<403<205
 15612|  br i1 %898, label %925, label %899                                                                                    ;L2494<138<2897<403<205
 15613| 
 15614| 899: ; preds = %882, %879, %878, %870
 15615|     ;; self = ptr undef
 15616|     ;; count = i64 1
 15617|     ;; ptr = !DIArgList(ptr %213, i64 40)
 15618|     ;; self = !DIArgList(ptr %213, i64 40)
 15619|     ;; end_or_len = ptr %214
 15624|     ;; self = ptr %482
 15625|     ;; f = ptr %210
 15626|     ;; self = ptr undef
 15627|     ;; self = ptr undef
 15628|     ;; count = i64 1
 15629|  br label %903                                                                                                         ;L331<404<205
 15630| 
 15631| 900: ; preds = %765
 15632|     ;; __self_0 = ptr %759
 15633|     ;; self = ptr %759
 15634|     ;; __arg1_0 = ptr %210
 15635|     ;; other = ptr %210
 15638|  %901 = load i64, ptr %763, , !!30579, !!8                                                                             ;L1878<2123<1127<264<403<2893<50<2494<138<2897<403<205
 15639|  %902 = icmp eq i64 %901, %755                                                                                         ;L1878<2123<1127<264<403<2893<50<2494<138<2897<403<205
 15640|  br i1 %902, label %783, label %766                                                                                    ;L403<2893<50<2494<138<2897<403<205
 15641| 
 15642| 903: ; preds = %906, %899
 15643|  %904 = phi ptr [ %907, %906 ], [ %482, %899 ]
 15644|     ;; ptr = ptr %904
 15645|     ;; self = ptr %904
 15646|     ;; end_or_len = ptr %485
 15649|  %905 = icmp eq ptr %904, %485                                                                                         ;L1714<180<331<404<205
 15650|  br i1 %905, label %567, label %906                                                                                    ;L180<331<404<205
 15651| 
 15652| 906: ; preds = %903
 15653|  %907 = gep %904, i64 8                                                                                                ;L656<185<331<404<205
 15654|     ;; x = ptr %904
 15655|  %908 = load ptr, ptr %904, , !!30716, !!8, !!8                                                                        ;L332<404<205
 15658|     ;; self = ptr %908
 15659|     ;; other = ptr %210
 15660|  %909 = gep %908, i64 1632                                                                                             ;L2158<404<332<404<205
 15661|  %910 = load i64, ptr %909, , !!30716, !!8                                                                             ;L2158<404<332<404<205
 15662|     ;; x1 = i64 %910
 15663|     ;; self = i64 %910
 15664|  %911 = gep %908, i64 1640                                                                                             ;L2158<404<332<404<205
 15665|  %912 = load i64, ptr %911, , !!30716, !!8                                                                             ;L2158<404<332<404<205
 15666|     ;; y1 = i64 %912
 15667|     ;; self = i64 %912
 15668|     ;; x2 = i64 %756
 15669|     ;; other = i64 %756
 15670|     ;; y2 = i64 %757
 15671|     ;; other = i64 %757
 15672|  %913 = icmp ult i64 %910, %756                                                                                        ;L3147<7<2158<404<332<404<205
 15673|  %914 = sub nuw i64 %756, %910                                                                                         ;L3147<7<2158<404<332<404<205
 15674|  %915 = sub nuw i64 %910, %756                                                                                         ;L3147<7<2158<404<332<404<205
 15675|  %916 = select i1 %913, i64 %914, i64 %915                                                                             ;L3147<7<2158<404<332<404<205
 15676|     ;; dx = i64 %916
 15677|  %917 = icmp ult i64 %912, %757                                                                                        ;L3147<8<2158<404<332<404<205
 15678|  %918 = sub nuw i64 %757, %912                                                                                         ;L3147<8<2158<404<332<404<205
 15679|  %919 = sub nuw i64 %912, %757                                                                                         ;L3147<8<2158<404<332<404<205
 15680|  %920 = select i1 %917, i64 %918, i64 %919                                                                             ;L3147<8<2158<404<332<404<205
 15681|     ;; dy = i64 %920
 15682|  %921 = mul i64 %916, %916                                                                                             ;L9<2158<404<332<404<205
 15683|  %922 = mul i64 %920, %920                                                                                             ;L9<2158<404<332<404<205
 15684|  %923 = add i64 %922, %921                                                                                             ;L9<2158<404<332<404<205
 15685|  %924 = icmp ult i64 %923, 22500000001                                                                                 ;L404<332<404<205
 15686|  br i1 %924, label %925, label %903                                                                                    ;L332<404<205
 15687| 
 15688| 925: ; preds = %906, %882, %853, %824, %795, %766
 15691|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway3new(ptr sret([136 x i8]) %14, ptr %5, ptr %4, i64 5)
 15692|  to label %926 unwind label %289, !!29557                                                                              ;L406<205
 15693| 
 15694| 926: ; preds = %925
 15695|  call void @llvm.memcpy.p0.p0.i64(ptr %15, ptr %14, i64 136, i1 false), !!29528                                        ;L406<205
 15696|  %927 = gep %15, i64 177                                                                                               ;L406<205
 15697|  store i8 3, ptr %927, , !!29528                                                                                       ;L406<205
 15699|     ;; self = ptr %40
 15700|     ;; self = ptr %40
 15701|     ;; value = ptr %15
 15702|     ;; src = ptr %15
 15703|     ;; additional = i64 1
 15704|     ;; needed_extra_cap = i64 1
 15705|     ;; needed_extra_cap = i64 1
 15706|     ;; strategy = i8 1
 15707|  %928 = load i64, ptr %209, , !!30778, !!8                                                                             ;L1428<406<205
 15708|     ;; self = ptr %40
 15709|  %929 = load i64, ptr %208, , !!30778, !!8                                                                             ;L149<1428<406<205
 15710|  %930 = icmp eq i64 %928, %929                                                                                         ;L1428<406<205
 15711|  br i1 %930, label %931, label %936                                                                                    ;L1428<406<205
 15712| 
 15713| 931: ; preds = %926
 15714|     ;; self = ptr %40
 15715|     ;; self = ptr %40
 15716|     ;; self = ptr %40
 15717|     ;; used_cap = i64 %928
 15718|     ;; used_cap = i64 %928
 15719|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %40, i64 %928, i64 1, i1 zeroext true)
 15720|  to label %932 unwind label %934, !!30786                                                                              ;L619<430<738<1429<406<205
 15721| 
 15722| 932: ; preds = %931
 15723|  %933 = load i64, ptr %209, , !!30778                                                                                  ;L1432<406<205
 15724|  br label %936                                                                                                         ;L619<430<738<1429<406<205
 15725| 
 15726| 934: ; preds = %931
 15727|  %935 = cleanuppad within none []
 15728|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %15) #34 [ "funclet"(token %935) ], !!29557 ;L1436<406<205
 15729|  cleanupret from %935 unwind label %289
 15730| 
 15731| 936: ; preds = %932, %926
 15732|  %937 = phi i64 [ %933, %932 ], [ %928, %926 ]                                                                         ;L1434<406<205
 15733|     ;; self = ptr %40
 15734|  %938 = load ptr, ptr %40, , !!30778, !!8, !!8                                                                         ;L138<1432<406<205
 15735|     ;; self = ptr %938
 15736|     ;; count = i64 %937
 15737|  %939 = gepS %938, i64 %937                                                                                            ;L961<1432<406<205
 15738|     ;; end = ptr %939
 15739|     ;; dst = ptr %939
 15740|  call void @llvm.memcpy.p0.p0.i64(ptr %939, ptr %15, i64 184, i1 false), !!29557                                       ;L1933<1433<406<205
 15741|  %940 = add i64 %937, 1                                                                                                ;L1434<406<205
 15742|  store i64 %940, ptr %209, , !!30778                                                                                   ;L1434<406<205
 15744|  br label %567                                                                                                         ;L405<205
 15745| 
 15746| 941: ; preds = %568
 15747|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.91) #35
 15748|  to label %202 unwind label %289, !!29557                                                                              ;L1013<378<205
 15749| 
 15750| 942: ; preds = %958, %567
 15751|  call void @llvm.memcpy.p0.p0.i64(ptr %59, ptr %40, i64 32, i1 false), !!30801                                         ;L415<205
 15753|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %36)
 15754|  to label %946 unwind label %943, !!29557                                                                              ;L825<416<205
 15755| 
 15756| 943: ; preds = %942
 15757|  %944 = cleanuppad within none []
 15759|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %36) [ "funclet"(token %944) ]
 15760|  to label %945 unwind label %199, !!29557                                                                              ;L825<825<416<205
 15761| 
 15762| 945: ; preds = %943
 15763|  cleanupret from %944 unwind label %199
 15764| 
 15765| 946: ; preds = %942
 15767|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %36)
 15768|  to label %963 unwind label %199, !!29557                                                                              ;L825<825<416<205
 15769| 
 15770| 947: ; preds = %567
 15773|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %12, ptr %5, ptr %4, i64 5, i1 zeroext false)
 15774|  to label %948 unwind label %289, !!29557                                                                              ;L412<205
 15775| 
 15776| 948: ; preds = %947
 15777|  call void @llvm.memcpy.p0.p0.i64(ptr %13, ptr %12, i64 136, i1 false), !!29528                                        ;L412<205
 15778|  %949 = gep %13, i64 177                                                                                               ;L412<205
 15779|  store i8 3, ptr %949, , !!29528                                                                                       ;L412<205
 15781|     ;; self = ptr %40
 15782|     ;; self = ptr %40
 15783|     ;; value = ptr %13
 15784|     ;; src = ptr %13
 15785|     ;; additional = i64 1
 15786|     ;; needed_extra_cap = i64 1
 15787|     ;; needed_extra_cap = i64 1
 15788|     ;; strategy = i8 1
 15789|  %950 = load i64, ptr %209, , !!30827, !!8                                                                             ;L1428<412<205
 15790|     ;; self = ptr %40
 15791|  %951 = load i64, ptr %208, , !!30827, !!8                                                                             ;L149<1428<412<205
 15792|  %952 = icmp eq i64 %950, %951                                                                                         ;L1428<412<205
 15793|  br i1 %952, label %953, label %958                                                                                    ;L1428<412<205
 15794| 
 15795| 953: ; preds = %948
 15796|     ;; self = ptr %40
 15797|     ;; self = ptr %40
 15798|     ;; self = ptr %40
 15799|     ;; used_cap = i64 %950
 15800|     ;; used_cap = i64 %950
 15801|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %40, i64 %950, i64 1, i1 zeroext true)
 15802|  to label %954 unwind label %956, !!30835                                                                              ;L619<430<738<1429<412<205
 15803| 
 15804| 954: ; preds = %953
 15805|  %955 = load i64, ptr %209, , !!30827                                                                                  ;L1432<412<205
 15806|  br label %958                                                                                                         ;L619<430<738<1429<412<205
 15807| 
 15808| 956: ; preds = %953
 15809|  %957 = cleanuppad within none []
 15810|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %13) #34 [ "funclet"(token %957) ], !!29557 ;L1436<412<205
 15811|  cleanupret from %957 unwind label %289
 15812| 
 15813| 958: ; preds = %954, %948
 15814|  %959 = phi i64 [ %955, %954 ], [ %950, %948 ]                                                                         ;L1434<412<205
 15815|     ;; self = ptr %40
 15816|  %960 = load ptr, ptr %40, , !!30827, !!8, !!8                                                                         ;L138<1432<412<205
 15817|     ;; self = ptr %960
 15818|     ;; count = i64 %959
 15819|  %961 = gepS %960, i64 %959                                                                                            ;L961<1432<412<205
 15820|     ;; end = ptr %961
 15821|     ;; dst = ptr %961
 15822|  call void @llvm.memcpy.p0.p0.i64(ptr %961, ptr %13, i64 184, i1 false), !!29557                                       ;L1933<1433<412<205
 15823|  %962 = add i64 %959, 1                                                                                                ;L1434<412<205
 15824|  store i64 %962, ptr %209, , !!30827                                                                                   ;L1434<412<205
 15826|  br label %942                                                                                                         ;L411<205
 15827| 
 15828| 963: ; preds = %946
 15830|  br label %982                                                                                                         ;L416<205
 15831| 
 15832| 964: ; preds = %301
 15833|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.92) #35
 15834|  to label %202 unwind label %289, !!29557                                                                              ;L1013<322<205
 15835| 
 15836| 965: ; preds = %280
 15837|  call void @llvm.memcpy.p0.p0.i64(ptr %38, ptr %37, i64 136, i1 false), !!29528                                        ;L302<205
 15838|  %966 = gep %38, i64 177                                                                                               ;L302<205
 15839|  store i8 3, ptr %966, , !!29528                                                                                       ;L302<205
 15841|     ;; self = ptr %40
 15842|     ;; self = ptr %40
 15843|     ;; value = ptr %38
 15844|     ;; src = ptr %38
 15845|     ;; additional = i64 1
 15846|     ;; needed_extra_cap = i64 1
 15847|     ;; needed_extra_cap = i64 1
 15848|     ;; strategy = i8 1
 15849|  %967 = load i64, ptr %209, , !!30863, !!8                                                                             ;L1428<302<205
 15850|     ;; self = ptr %40
 15851|  %968 = load i64, ptr %208, , !!30863, !!8                                                                             ;L149<1428<302<205
 15852|  %969 = icmp eq i64 %967, %968                                                                                         ;L1428<302<205
 15853|  br i1 %969, label %970, label %975                                                                                    ;L1428<302<205
 15854| 
 15855| 970: ; preds = %965
 15856|     ;; self = ptr %40
 15857|     ;; self = ptr %40
 15858|     ;; self = ptr %40
 15859|     ;; used_cap = i64 %967
 15860|     ;; used_cap = i64 %967
 15861|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %40, i64 %967, i64 1, i1 zeroext true)
 15862|  to label %971 unwind label %973, !!30871                                                                              ;L619<430<738<1429<302<205
 15863| 
 15864| 971: ; preds = %970
 15865|  %972 = load i64, ptr %209, , !!30863                                                                                  ;L1432<302<205
 15866|  br label %975                                                                                                         ;L619<430<738<1429<302<205
 15867| 
 15868| 973: ; preds = %970
 15869|  %974 = cleanuppad within none []
 15870|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %38) #34 [ "funclet"(token %974) ], !!29557 ;L1436<302<205
 15871|  cleanupret from %974 unwind label %199
 15872| 
 15873| 975: ; preds = %971, %965
 15874|  %976 = phi i64 [ %972, %971 ], [ %967, %965 ]                                                                         ;L1434<302<205
 15875|     ;; self = ptr %40
 15876|  %977 = load ptr, ptr %40, , !!30863, !!8, !!8                                                                         ;L138<1432<302<205
 15877|     ;; self = ptr %977
 15878|     ;; count = i64 %976
 15879|  %978 = gepS %977, i64 %976                                                                                            ;L961<1432<302<205
 15880|     ;; end = ptr %978
 15881|     ;; dst = ptr %978
 15882|  call void @llvm.memcpy.p0.p0.i64(ptr %978, ptr %38, i64 184, i1 false), !!29557                                       ;L1933<1433<302<205
 15883|  %979 = add i64 %976, 1                                                                                                ;L1434<302<205
 15884|  store i64 %979, ptr %209, , !!30863                                                                                   ;L1434<302<205
 15886|  call void @llvm.memcpy.p0.p0.i64(ptr %59, ptr %40, i64 32, i1 false), !!30801                                         ;L303<205
 15887|  br label %982                                                                                                         ;L416<205
 15888| 
 15889| 980: ; preds = %981, %199
 15890|  cleanupret from %201 unwind label %189
 15891| 
 15892| 981: ; preds = %199
 15893|  invoke fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %40) #34 [ "funclet"(token %201) ]
 15894|  to label %980 unwind label %189                                                                                       ;L416<205
 15895| 
 15896| 982: ; preds = %975, %963
 15899|  %983 = load ptr, ptr %65, , !!8                                                                                       ;L208
 15900|  %984 = icmp eq ptr %983, null                                                                                         ;L208
 15901|  br i1 %984, label %989, label %985                                                                                    ;L208
 15902| 
 15903| 985: ; preds = %982
 15904|     ;; nearest_tower = ptr %983
 15905|  %986 = gep %983, i64 104                                                                                              ;L209
 15906|  %987 = load i64, ptr %986, , !!8                                                                                      ;L209
 15907|  %988 = icmp eq i64 %987, 2                                                                                            ;L209
 15908|  br i1 %988, label %993, label %989                                                                                    ;L209
 15909| 
 15910| 989: ; preds = %1020, %999, %993, %985, %982
 15911|     ;; self = ptr %63
 15912|     ;; self = ptr %63
 15913|  %990 = gep %63, i64 24                                                                                                ;L1617<1636<218
 15914|  %991 = load i64, ptr %990, , !!8                                                                                      ;L1617<1636<218
 15915|  %992 = icmp eq i64 %991, 0                                                                                            ;L218
 15916|  br i1 %992, label %1025, label %1028                                                                                  ;L218
 15917| 
 15918| 993: ; preds = %985
 15919|     ;; info = ptr %983
 15920|  %994 = gep %983, i64 136                                                                                              ;L210
 15921|  %995 = load i64, ptr %994,                                                                                            ;L210
 15922|     ;; self[0..+8] = i64 %995
 15925|  %996 = trunc nuw i64 %995 to i1                                                                                       ;L1161<210
 15926|  br i1 %996, label %999, label %989                                                                                    ;L1161<210
 15927| 
 15928| 997: ; preds = %1275, %1264, %1257, %1254, %1252, %1251, %1250, %1240, %1234, %1230, %1227, %1226, %1137, %1133, %1130, %1126, %1124, %1120, %1096, %1063, %1058, %1056, %1052, %1047, %1025, %1018, %1007, %1006, %1005
 15929|  %998 = cleanuppad within none []
 15930|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %59) #34 [ "funclet"(token %998) ] ;L276
 15931|  cleanupret from %998 unwind label %189                                                                                ;L276
 15932| 
 15933| 999: ; preds = %993
 15934|  %1000 = gep %983, i64 152                                                                                             ;L210
 15935|  %1001 = load i64, ptr %1000,                                                                                          ;L210
 15936|     ;; self[16..+8] = i64 %1001
 15938|     ;; self = ptr undef
 15939|  %1002 = gep %94, i64 1472                                                                                             ;L210
 15940|  %1003 = load i64, ptr %1002, , !!8                                                                                    ;L210
 15942|     ;; l = ptr undef
 15943|     ;; self = ptr undef
 15946|  %1004 = icmp eq i64 %1001, %1003                                                                                      ;L1878<2440<210
 15947|  br i1 %1004, label %1005, label %989                                                                                  ;L210
 15948| 
 15949| 1005: ; preds = %999
 15950|     ;; self = ptr %63
 15951|  invoke void @ai::small_action15SmallActionPlayE8truncateBX_(ptr %63, i64 0)
 15952|  to label %1006 unwind label %997                                                                                      ;L1599<211
 15953| 
 15954| 1006: ; preds = %1005
 15955|     ;; self = ptr %59
 15956|  invoke void @ai::small_action15SmallActionPlayE8truncateBX_(ptr %59, i64 0)
 15957|  to label %1007 unwind label %997                                                                                      ;L1599<212
 15958| 
 15959| 1007: ; preds = %1006
 15962|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %57, ptr %5, ptr %4, i64 5, i1 zeroext true)
 15963|  to label %1008 unwind label %997                                                                                      ;L213
 15964| 
 15965| 1008: ; preds = %1007
 15966|  call void @llvm.memcpy.p0.p0.i64(ptr %58, ptr %57, i64 136, i1 false)                                                 ;L213
 15967|  %1009 = gep %58, i64 177                                                                                              ;L213
 15968|  store i8 3, ptr %1009,                                                                                                ;L213
 15970|     ;; self = ptr %59
 15971|     ;; self = ptr %59
 15972|     ;; value = ptr %58
 15973|     ;; src = ptr %58
 15974|     ;; additional = i64 1
 15975|     ;; needed_extra_cap = i64 1
 15976|     ;; needed_extra_cap = i64 1
 15977|     ;; strategy = i8 1
 15978|  %1010 = gep %59, i64 24                                                                                               ;L1428<213
 15979|  %1011 = load i64, ptr %1010, , !!30934, !!8                                                                           ;L1428<213
 15980|     ;; self = ptr %59
 15981|  %1012 = gep %59, i64 16                                                                                               ;L149<1428<213
 15982|  %1013 = load i64, ptr %1012, , !!30934, !!8                                                                           ;L149<1428<213
 15983|  %1014 = icmp eq i64 %1011, %1013                                                                                      ;L1428<213
 15984|  br i1 %1014, label %1015, label %1020                                                                                 ;L1428<213
 15985| 
 15986| 1015: ; preds = %1008
 15987|     ;; self = ptr %59
 15988|     ;; self = ptr %59
 15989|     ;; self = ptr %59
 15990|     ;; used_cap = i64 %1011
 15991|     ;; used_cap = i64 %1011
 15992|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %59, i64 %1011, i64 1, i1 zeroext true)
 15993|  to label %1016 unwind label %1018, !!30934                                                                            ;L619<430<738<1429<213
 15994| 
 15995| 1016: ; preds = %1015
 15996|  %1017 = load i64, ptr %1010, , !!30934                                                                                ;L1432<213
 15997|  br label %1020                                                                                                        ;L619<430<738<1429<213
 15998| 
 15999| 1018: ; preds = %1015
 16000|  %1019 = cleanuppad within none []
 16001|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %58) #34 [ "funclet"(token %1019) ] ;L1436<213
 16002|  cleanupret from %1019 unwind label %997
 16003| 
 16004| 1020: ; preds = %1016, %1008
 16005|  %1021 = phi i64 [ %1017, %1016 ], [ %1011, %1008 ]                                                                    ;L1434<213
 16006|     ;; self = ptr %59
 16007|  %1022 = load ptr, ptr %59, , !!30934, !!8, !!8                                                                        ;L138<1432<213
 16008|     ;; self = ptr %1022
 16009|     ;; count = i64 %1021
 16010|  %1023 = gepS %1022, i64 %1021                                                                                         ;L961<1432<213
 16011|     ;; end = ptr %1023
 16012|     ;; dst = ptr %1023
 16013|  call void @llvm.memcpy.p0.p0.i64(ptr %1023, ptr %58, i64 184, i1 false)                                               ;L1933<1433<213
 16014|  %1024 = add i64 %1021, 1                                                                                              ;L1434<213
 16015|  store i64 %1024, ptr %1010, , !!30934                                                                                 ;L1434<213
 16017|  br label %989                                                                                                         ;L210
 16018| 
 16019| 1025: ; preds = %989
 16020|  %1026 = gep %4, i64 384                                                                                               ;L219
 16021|  %1027 = invoke i64 @gc::simulation5state6playerNtB4_16AthleteParameter20positioning_accuracy(ptr %1026)
 16022|  to label %1033 unwind label %997                                                                                      ;L219
 16023| 
 16024| 1028: ; preds = %989
 16025|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %63, i64 32, i1 false)                                                   ;L274
 16027|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %59)
 16028|  to label %1032 unwind label %1029                                                                                     ;L825<276
 16029| 
 16030| 1029: ; preds = %1028
 16031|  %1030 = cleanuppad within none []
 16033|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %59) [ "funclet"(token %1030) ]
 16034|  to label %1031 unwind label %189                                                                                      ;L825<825<276
 16035| 
 16036| 1031: ; preds = %1029
 16037|  cleanupret from %1030 unwind label %189
 16038| 
 16039| 1032: ; preds = %1028
 16041|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %59)
 16042|  to label %1291 unwind label %189                                                                                      ;L825<825<276
 16043| 
 16044| 1033: ; preds = %1025
 16045|     ;; positioning_accuracy = i64 %1027
 16046|     ;; min_v = i64 %1027
 16047|  %1034 = sub i64 2000, %1027                                                                                           ;L221
 16048|     ;; max_v = i64 %1034
 16049|     ;; self = ptr undef
 16050|     ;; self = ptr undef
 16051|     ;; f[0..+8] = ptr %69
 16052|     ;; f[8..+8] = ptr %5
 16053|     ;; f[16..+8] = ptr %4
 16054|     ;; f[24..+8] = ptr %94
 16055|     ;; f[32..+8] = ptr %3
 16056|     ;; f[40..+8] = ptr undef
 16057|     ;; f[48..+8] = ptr undef
 16058|     ;; fold[0..+8] = ptr %69
 16059|     ;; fold[8..+8] = ptr %5
 16060|     ;; fold[16..+8] = ptr %4
 16061|     ;; fold[24..+8] = ptr %94
 16062|     ;; fold[32..+8] = ptr %3
 16063|     ;; fold[40..+8] = ptr undef
 16064|     ;; fold[48..+8] = ptr undef
 16067|     ;; f[8..+8] = ptr %69
 16068|     ;; f[16..+8] = ptr %5
 16069|     ;; f[24..+8] = ptr %4
 16070|     ;; f[32..+8] = ptr %94
 16071|     ;; f[40..+8] = ptr %3
 16072|     ;; f[48..+8] = ptr undef
 16073|     ;; f[56..+8] = ptr undef
 16074|     ;; self = ptr undef
 16077|     ;; self = ptr undef
 16078|     ;; count = i64 1
 16079|     ;; ptr = ptr %213
 16080|     ;; self = ptr %213
 16081|     ;; end_or_len = ptr %214
 16084|  %1035 = gep %11, i64 8
 16085|  %1036 = gep %5, i64 16
 16086|  %1037 = gep %94, i64 1632
 16087|  %1038 = gep %94, i64 1640
 16088|  %1039 = load ptr, ptr %1036, , !!8
 16089|  %1040 = getelementptr { { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, [5 x { i64, [2 x i64] }], [5 x { i64, { i8, [23 x i8] } }], [5 x { i8, [7 x i8], i64 }], [5 x i64], [5 x i64], [5 x i64], [5 x { i64, i64 }], i64, i64, i64, i64, i64, [5 x i32], i8, [3 x i8] }, ptr %1039, i64 %97
 16090|  br label %1041                                                                                                        ;L180<2493<138<2897<225
 16091| 
 16092| 1041: ; preds = %1101, %1033
 16093|  %1042 = phi i64 [ 0, %1033 ], [ %1044, %1101 ]
 16094|  %1043 = gep %213, i64 %1042                                                                                           ;L656<185<2493<138<2897<225
 16095|     ;; ptr = ptr %1043
 16096|  %1044 = add nuw nsw i64 %1042, 8                                                                                      ;L656<185<2493<138<2897<225
 16097|     ;; x = ptr %1043
 16098|  %1045 = load ptr, ptr %1043, , !!30995, !!8                                                                           ;L2494<138<2897<225
 16099|     ;; f = ptr undef
 16103|  %1046 = icmp eq ptr %1045, null                                                                                       ;L49<2494<138<2897<225
 16104|  br i1 %1046, label %1101, label %1047                                                                                 ;L49<2494<138<2897<225
 16105| 
 16106| 1047: ; preds = %1041
 16107|     ;; x = ptr %1045
 16111|     ;; x = ptr %1045
 16120|     ;; c = ptr %1045
 16121|     ;; self = ptr %1045
 16122|     ;; self = ptr %1045
 16123|     ;; jrng = ptr %11
 16125|  %1048 = load i64, ptr %69, , !!31059, !!8                                                                             ;L226<2893<50<2494<138<2897<225
 16126|  %1049 = gep %1045, i64 1472                                                                                           ;L226<2893<50<2494<138<2897<225
 16127|  %1050 = load i64, ptr %1049, , !!31065, !!8                                                                           ;L226<2893<50<2494<138<2897<225
 16128|  %1051 = invoke { i64, i64 } @ai::utils18range_misjudge_rng(i64 %1048, ptr %5, ptr %4, i64 %1050)
 16129|  to label %1052 unwind label %997                                                                                      ;L226<2893<50<2494<138<2897<225
 16130| 
 16131| 1052: ; preds = %1047
 16132|  %1053 = extractvalue { i64, i64 } %1051, 0                                                                            ;L226<2893<50<2494<138<2897<225
 16133|  %1054 = extractvalue { i64, i64 } %1051, 1                                                                            ;L226<2893<50<2494<138<2897<225
 16134|  store i64 %1053, ptr %11, , !!31059                                                                                   ;L226<2893<50<2494<138<2897<225
 16135|  store i64 %1054, ptr %1035, , !!31059                                                                                 ;L226<2893<50<2494<138<2897<225
 16136|  %1055 = invoke i64 @ai::plan_legacy3old6battle17max_range_can_use(ptr %1045, ptr %94)
 16137|  to label %1056 unwind label %997                                                                                      ;L227<2893<50<2494<138<2897<225
 16138| 
 16139| 1056: ; preds = %1052
 16140|  %1057 = invoke i64 @ai::utils19range_misjudge_roll(ptr %3, ptr %11, i64 %1027, i64 %1034)
 16141|  to label %1058 unwind label %997                                                                                      ;L227<2893<50<2494<138<2897<225
 16142| 
 16143| 1058: ; preds = %1056
 16144|  %1059 = mul i64 %1057, %1055                                                                                          ;L227<2893<50<2494<138<2897<225
 16145|  %1060 = udiv i64 %1059, 1000                                                                                          ;L227<2893<50<2494<138<2897<225
 16146|  %1061 = add nuw nsw i64 %1060, 10000                                                                                  ;L227<2893<50<2494<138<2897<225
 16147|     ;; emr = i64 %1061
 16148|  %1062 = invoke i64 @ai::plan_legacy3old6battle17max_range_can_use(ptr %94, ptr %1045)
 16149|  to label %1063 unwind label %997                                                                                      ;L228<2893<50<2494<138<2897<225
 16150| 
 16151| 1063: ; preds = %1058
 16152|     ;; mr = i64 %1062
 16153|  %1064 = load ptr, ptr %90, , !!31065, !!8, !!8                                                                        ;L230<2893<50<2494<138<2897<225
 16154|  %1065 = load ptr, ptr %175, , !!31065, !!8, !!8                                                                       ;L230<2893<50<2494<138<2897<225
 16155|  %1066 = invoke zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %1040, ptr %1064, ptr %1065, ptr %4, ptr %1045)
 16156|  to label %1067 unwind label %997                                                                                      ;L230<2893<50<2494<138<2897<225
 16157| 
 16158| 1067: ; preds = %1063
 16159|  br i1 %1066, label %1068, label %1103                                                                                 ;L230<2893<50<2494<138<2897<225
 16160| 
 16161| 1068: ; preds = %1067
 16162|     ;; other = ptr %94
 16163|  %1069 = gep %1045, i64 1632                                                                                           ;L2158<231<2893<50<2494<138<2897<225
 16164|  %1070 = load i64, ptr %1069, , !!31065, !!8                                                                           ;L2158<231<2893<50<2494<138<2897<225
 16165|     ;; x1 = i64 %1070
 16166|     ;; self = i64 %1070
 16167|  %1071 = gep %1045, i64 1640                                                                                           ;L2158<231<2893<50<2494<138<2897<225
 16168|  %1072 = load i64, ptr %1071, , !!31065, !!8                                                                           ;L2158<231<2893<50<2494<138<2897<225
 16169|     ;; y1 = i64 %1072
 16170|     ;; self = i64 %1072
 16171|  %1073 = load i64, ptr %1037, , !!31065, !!8                                                                           ;L2158<231<2893<50<2494<138<2897<225
 16172|     ;; x2 = i64 %1073
 16173|     ;; other = i64 %1073
 16174|  %1074 = load i64, ptr %1038, , !!31065, !!8                                                                           ;L2158<231<2893<50<2494<138<2897<225
 16175|     ;; y2 = i64 %1074
 16176|     ;; other = i64 %1074
 16177|  %1075 = icmp ult i64 %1070, %1073                                                                                     ;L3147<7<2158<231<2893<50<2494<138<2897<225
 16178|  %1076 = sub nuw i64 %1073, %1070                                                                                      ;L3147<7<2158<231<2893<50<2494<138<2897<225
 16179|  %1077 = sub nuw i64 %1070, %1073                                                                                      ;L3147<7<2158<231<2893<50<2494<138<2897<225
 16180|  %1078 = select i1 %1075, i64 %1076, i64 %1077                                                                         ;L3147<7<2158<231<2893<50<2494<138<2897<225
 16181|     ;; dx = i64 %1078
 16182|  %1079 = icmp ult i64 %1072, %1074                                                                                     ;L3147<8<2158<231<2893<50<2494<138<2897<225
 16183|  %1080 = sub nuw i64 %1074, %1072                                                                                      ;L3147<8<2158<231<2893<50<2494<138<2897<225
 16184|  %1081 = sub nuw i64 %1072, %1074                                                                                      ;L3147<8<2158<231<2893<50<2494<138<2897<225
 16185|  %1082 = select i1 %1079, i64 %1080, i64 %1081                                                                         ;L3147<8<2158<231<2893<50<2494<138<2897<225
 16186|     ;; dy = i64 %1082
 16187|  %1083 = mul i64 %1078, %1078                                                                                          ;L9<2158<231<2893<50<2494<138<2897<225
 16188|  %1084 = mul i64 %1082, %1082                                                                                          ;L9<2158<231<2893<50<2494<138<2897<225
 16189|  %1085 = add i64 %1084, %1083                                                                                          ;L9<2158<231<2893<50<2494<138<2897<225
 16190|  %1086 = mul i64 %1061, %1061                                                                                          ;L231<2893<50<2494<138<2897<225
 16191|  %1087 = icmp ugt i64 %1085, %1086                                                                                     ;L231<2893<50<2494<138<2897<225
 16192|  br i1 %1087, label %1103, label %1088                                                                                 ;L231<2893<50<2494<138<2897<225
 16193| 
 16194| 1088: ; preds = %1068
 16195|  %1089 = gep %1045, i64 104                                                                                            ;L1548<232<2893<50<2494<138<2897<225
 16196|  %1090 = load i64, ptr %1089, , !!31065, !!8                                                                           ;L1548<232<2893<50<2494<138<2897<225
 16197|  %1091 = icmp ne i64 %1090, 13                                                                                         ;L1548<232<2893<50<2494<138<2897<225
 16198|  %1092 = gep %1045, i64 112
 16199|  %1093 = load i64, ptr %1092, , !!31065
 16200|  %1094 = icmp samesign ult i64 %1093, 3
 16201|  %1095 = select i1 %1091, i1 true, i1 %1094                                                                            ;L1548<232<2893<50<2494<138<2897<225
 16202|  br i1 %1095, label %1096, label %1103                                                                                 ;L1548<232<2893<50<2494<138<2897<225
 16203| 
 16204| 1096: ; preds = %1088
 16205|  %1097 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity11block_input(ptr %1045)
 16206|  to label %1098 unwind label %997                                                                                      ;L232<2893<50<2494<138<2897<225
 16207| 
 16208| 1098: ; preds = %1096
 16209|  %1099 = icmp ne i64 %1062, 0
 16210|  %1100 = or i1 %1099, %1097                                                                                            ;L234<2893<50<2494<138<2897<225
 16212|  br i1 %1100, label %1101, label %1126                                                                                 ;L2494<138<2897<225
 16213| 
 16214| 1101: ; preds = %1103, %1098, %1041
 16215|     ;; self = ptr undef
 16216|     ;; count = i64 1
 16217|     ;; ptr = !DIArgList(ptr %213, i64 %1044)
 16218|     ;; self = !DIArgList(ptr %213, i64 %1044)
 16219|     ;; end_or_len = ptr %214
 16222|  %1102 = icmp eq i64 %1044, 40                                                                                         ;L1714<180<2493<138<2897<225
 16223|  br i1 %1102, label %1104, label %1041                                                                                 ;L180<2493<138<2897<225
 16224| 
 16225| 1103: ; preds = %1088, %1068, %1067
 16227|  br label %1101                                                                                                        ;L2494<138<2897<225
 16228| 
 16229| 1104: ; preds = %1101
 16230|     ;; self = ptr %59
 16231|     ;; self = ptr %59
 16232|  %1105 = load ptr, ptr %59, , !!8, !!8                                                                                 ;L138<2073<238
 16233|     ;; p = ptr %1105
 16234|  %1106 = gep %59, i64 24                                                                                               ;L2075<238
 16235|  %1107 = load i64, ptr %1106, , !!8                                                                                    ;L2075<238
 16236|     ;; len = i64 %1107
 16237|     ;; count = i64 %1107
 16238|     ;; self[0..+8] = ptr %1105
 16239|     ;; slice[0..+8] = ptr %1105
 16240|     ;; self[8..+8] = i64 %1107
 16241|     ;; slice[8..+8] = i64 %1107
 16242|     ;; ptr = ptr %1105
 16243|     ;; self = ptr %1105
 16244|  %1108 = mul nuw nsw i64 %1107, 184                                                                                    ;L961<100<1042<238
 16245|  %1109 = gep %1105, i64 %1108                                                                                          ;L961<100<1042<238
 16246|     ;; f[0..+8] = ptr %1
 16247|     ;; f[0..+8] = ptr %1
 16248|     ;; f[8..+8] = ptr %69
 16249|     ;; f[8..+8] = ptr %69
 16250|     ;; f[16..+8] = ptr %6
 16251|     ;; f[16..+8] = ptr %6
 16252|     ;; f[24..+8] = ptr %3
 16253|     ;; f[24..+8] = ptr %3
 16254|     ;; f[32..+8] = ptr %4
 16255|     ;; f[32..+8] = ptr %4
 16256|     ;; f[40..+8] = ptr %5
 16257|     ;; f[40..+8] = ptr %5
 16258|     ;; f[48..+8] = ptr %8
 16259|     ;; f[48..+8] = ptr %8
 16260|     ;; self[0..+8] = ptr %1105
 16261|     ;; self[8..+8] = ptr %1109
 16262|     ;; self = ptr %10
 16266|     ;; self[0..+8] = ptr %1105
 16267|     ;; self[8..+8] = ptr %1109
 16268|  %1110 = gep %10, i64 8                                                                                                ;L69<836<3325<238
 16269|  store ptr %1109, ptr %1110, , !!31196                                                                                 ;L69<836<3325<238
 16270|  %1111 = gep %10, i64 16                                                                                               ;L69<836<3325<238
 16271|  store ptr %1, ptr %1111,                                                                                              ;L69<836<3325<238
 16272|  %1112 = gep %10, i64 24                                                                                               ;L69<836<3325<238
 16273|  store ptr %69, ptr %1112,                                                                                             ;L69<836<3325<238
 16274|  %1113 = gep %10, i64 32                                                                                               ;L69<836<3325<238
 16275|  store ptr %6, ptr %1113,                                                                                              ;L69<836<3325<238
 16276|  %1114 = gep %10, i64 40                                                                                               ;L69<836<3325<238
 16277|  store ptr %3, ptr %1114,                                                                                              ;L69<836<3325<238
 16278|  %1115 = gep %10, i64 48                                                                                               ;L69<836<3325<238
 16279|  store ptr %4, ptr %1115,                                                                                              ;L69<836<3325<238
 16280|  %1116 = gep %10, i64 56                                                                                               ;L69<836<3325<238
 16281|  store ptr %5, ptr %1116,                                                                                              ;L69<836<3325<238
 16282|  %1117 = gep %10, i64 64                                                                                               ;L69<836<3325<238
 16283|  store ptr %8, ptr %1117,                                                                                              ;L69<836<3325<238
 16285|     ;; self = ptr %10
 16288|     ;; self = ptr %10
 16289|     ;; self = ptr %10
 16290|     ;; count = i64 1
 16291|     ;; ptr = ptr %1105
 16292|     ;; self = ptr %1105
 16293|     ;; end_or_len = ptr %1109
 16296|  %1118 = icmp eq i64 %1107, 0                                                                                          ;L1714<180<107<2706<3354<3325<238
 16297|  br i1 %1118, label %1119, label %1120                                                                                 ;L180<107<2706<3354<3325<238
 16298| 
 16299| 1119: ; preds = %1104
 16301|     ;; self = ptr null
 16302|  br label %1133                                                                                                        ;L1011<238
 16303| 
 16304| 1120: ; preds = %1104
 16305|  %1121 = gep %1105, i64 184                                                                                            ;L656<185<107<2706<3354<3325<238
 16306|  store ptr %1121, ptr %10, , !!31184                                                                                   ;L185<107<2706<3354<3325<238
 16307|     ;; self = ptr %1105
 16308|     ;; f = ptr %1111
 16309|     ;; self = ptr %1111
 16310|     ;; x = ptr %1105
 16311|     ;; args = ptr %1105
 16312|     ;; x = ptr %1105
 16322|  %1122 = load i64, ptr %69, , !!31340, !!8                                                                             ;L238<3317<310<1162<107<2706<3354<3325<238
 16323|  %1123 = invoke i64 @ai::plan_legacy8sub_plan11serpen_pokeNtB2_17SerpenPokeSubPlan5score(ptr poison, i64 %1122, ptr %6, ptr %3, ptr %4, ptr %5, ptr %1105, ptr %8)
 16324|  to label %1124 unwind label %997                                                                                      ;L238<3317<310<1162<107<2706<3354<3325<238
 16325| 
 16326| 1124: ; preds = %1120
 16327|     ;; first[0..+8] = i64 %1123
 16328|     ;; first[8..+8] = ptr %1105
 16329|  %1125 = invoke { i64, ptr } @core::iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayENCINvNvNtNtNtBa_6traits8iterator8Iterator10max_by_key3keyRB1n_xNCNvMNtNtNtB1r_11plan_legacy8sub_plan11serpen_pokeNtB3p_17SerpenPokeSubPlan17action_candidatess6_0E0EB2q_4foldTxB3e_ENCINvNvB2q_6max_by4foldB56_INvB2o_7compareB3e_xEE0EB1r_(ptr %10, i64 %1123, ptr %1105)
 16330|  to label %1127 unwind label %997                                                                                      ;L2707<3354<3325<238
 16331| 
 16332| 1126: ; preds = %1098
 16335|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %55, ptr %5, ptr %4, i64 5, i1 zeroext false)
 16336|  to label %1275 unwind label %997                                                                                      ;L235
 16337| 
 16338| 1127: ; preds = %1124
 16339|  %1128 = extractvalue { i64, ptr } %1125, 1                                                                            ;L2707<3354<3325<238
 16341|     ;; self = ptr %1128
 16342|  %1129 = icmp eq ptr %1128, null                                                                                       ;L1011<238
 16343|  br i1 %1129, label %1133, label %1130                                                                                 ;L1011<238
 16344| 
 16345| 1130: ; preds = %1127
 16346|     ;; best_move_action = ptr %1128
 16348|  %1131 = gep %176, i64 528                                                                                             ;L242
 16349|  %1132 = load ptr, ptr %1131, , !!8                                                                                    ;L242
 16350|  invoke void %1132(ptr sret([40 x i8]) %54, ptr %174)
 16351|  to label %1134 unwind label %997                                                                                      ;L242
 16352| 
 16353| 1133: ; preds = %1127, %1119
 16354|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.95) #35
 16355|  to label %85 unwind label %997                                                                                        ;L1013<238
 16356| 
 16357| 1134: ; preds = %1130
 16358|     ;; self = ptr %54
 16359|     ;; f[0..+8] = ptr %4
 16360|     ;; f[8..+8] = ptr %94
 16362|     ;; f[0..+8] = ptr %4
 16363|     ;; f[8..+8] = ptr %94
 16364|     ;; self = ptr %54
 16367|  %1135 = load i64, ptr %1037, , !!31396
 16368|  %1136 = load i64, ptr %1038, , !!31396
 16369|  br label %1137                                                                                                        ;L2493<2897<242
 16370| 
 16371| 1137: ; preds = %1174, %1134
 16372|  %1138 = invoke ptr @gc::simulationNtB5_14ProjectileIterNtNtNtNtCsjihNppCmMEE_4core4iter6traits8iterator8Iterator4next(ptr %54)
 16373|  to label %1139 unwind label %997                                                                                      ;L2493<2897<242
 16374| 
 16375| 1139: ; preds = %1137
 16376|  %1140 = icmp eq ptr %1138, null                                                                                       ;L2493<2897<242
 16377|  br i1 %1140, label %1175, label %1141                                                                                 ;L2493<2897<242
 16378| 
 16379| 1141: ; preds = %1139
 16380|     ;; x = ptr %1138
 16383|     ;; x = ptr %1138
 16386|     ;; p = ptr %1138
 16387|     ;; __arg1_discr = i64 0
 16388|     ;; self = ptr %1138
 16389|     ;; self = ptr %1138
 16392|  %1142 = load i64, ptr %1138, , !!31450, !!8                                                                           ;L1127<264<243<2893<2494<2897<242
 16393|     ;; __self_discr = i64 %1142
 16394|  %1143 = icmp eq i64 %1142, 0                                                                                          ;L1127<264<243<2893<2494<2897<242
 16395|  br i1 %1143, label %1144, label %1148                                                                                 ;L1127<264<243<2893<2494<2897<242
 16396| 
 16397| 1144: ; preds = %1141
 16398|  %1145 = gep %1138, i64 8                                                                                              ;L1127<264<243<2893<2494<2897<242
 16399|     ;; __self_0 = ptr %1138
 16400|     ;; self = ptr %1138
 16405|  %1146 = load i64, ptr %1145, , !!31450, !!8                                                                           ;L1878<2123<1127<264<243<2893<2494<2897<242
 16406|  %1147 = icmp eq i64 %1146, %79                                                                                        ;L1878<2123<1127<264<243<2893<2494<2897<242
 16407|  br i1 %1147, label %1174, label %1148                                                                                 ;L243<2893<2494<2897<242
 16408| 
 16409| 1148: ; preds = %1144, %1141
 16410|     ;; self = ptr %1138
 16411|  %1149 = gep %1138, i64 64                                                                                             ;L134<243<2893<2494<2897<242
 16412|  %1150 = load i64, ptr %1149, , !!31450, !!8                                                                           ;L134<243<2893<2494<2897<242
 16413|  %1151 = icmp ne i64 %1150, 9                                                                                          ;L134<243<2893<2494<2897<242
 16414|  call void @llvm.assume(i1 %1151)                                                                                      ;L134<243<2893<2494<2897<242
 16415|  %1152 = add nsw i64 %1150, -2                                                                                         ;L134<243<2893<2494<2897<242
 16416|  %1153 = icmp samesign ugt i64 %1150, 1                                                                                ;L134<243<2893<2494<2897<242
 16417|  %1154 = select i1 %1153, i64 %1152, i64 7                                                                             ;L134<243<2893<2494<2897<242
 16418|  switch i64 %1154, label %1157 [
 16419|  i64 4, label %1174
 16420|  i64 5, label %1174
 16421|  i64 7, label %1155
 16422|  ]                                                                                                                     ;L134<243<2893<2494<2897<242
 16423| 
 16424| 1155: ; preds = %1148
 16425|  %1156 = icmp eq i64 %1150, 1                                                                                          ;L134<243<2893<2494<2897<242
 16426|  br i1 %1156, label %1174, label %1157                                                                                 ;L243<2893<2494<2897<242
 16427| 
 16428| 1157: ; preds = %1155, %1148
 16429|     ;; x1 = i64 %1135
 16430|     ;; self = i64 %1135
 16431|     ;; y1 = i64 %1136
 16432|     ;; self = i64 %1136
 16433|  %1158 = gep %1138, i64 256                                                                                            ;L244<2893<2494<2897<242
 16434|  %1159 = load i64, ptr %1158, , !!31450, !!8                                                                           ;L244<2893<2494<2897<242
 16435|     ;; x2 = i64 %1159
 16436|     ;; other = i64 %1159
 16437|  %1160 = gep %1138, i64 264                                                                                            ;L244<2893<2494<2897<242
 16438|  %1161 = load i64, ptr %1160, , !!31450, !!8                                                                           ;L244<2893<2494<2897<242
 16439|     ;; y2 = i64 %1161
 16440|     ;; other = i64 %1161
 16441|  %1162 = icmp ult i64 %1135, %1159                                                                                     ;L3147<7<244<2893<2494<2897<242
 16442|  %1163 = sub nuw i64 %1159, %1135                                                                                      ;L3147<7<244<2893<2494<2897<242
 16443|  %1164 = sub nuw i64 %1135, %1159                                                                                      ;L3147<7<244<2893<2494<2897<242
 16444|  %1165 = select i1 %1162, i64 %1163, i64 %1164                                                                         ;L3147<7<244<2893<2494<2897<242
 16445|     ;; dx = i64 %1165
 16446|  %1166 = icmp ult i64 %1136, %1161                                                                                     ;L3147<8<244<2893<2494<2897<242
 16447|  %1167 = sub nuw i64 %1161, %1136                                                                                      ;L3147<8<244<2893<2494<2897<242
 16448|  %1168 = sub nuw i64 %1136, %1161                                                                                      ;L3147<8<244<2893<2494<2897<242
 16449|  %1169 = select i1 %1166, i64 %1167, i64 %1168                                                                         ;L3147<8<244<2893<2494<2897<242
 16450|     ;; dy = i64 %1169
 16451|  %1170 = mul i64 %1165, %1165                                                                                          ;L9<244<2893<2494<2897<242
 16452|  %1171 = mul i64 %1169, %1169                                                                                          ;L9<244<2893<2494<2897<242
 16453|  %1172 = add i64 %1171, %1170                                                                                          ;L9<244<2893<2494<2897<242
 16454|  %1173 = icmp ult i64 %1172, 176400000000                                                                              ;L244<2893<2494<2897<242
 16455|  br i1 %1173, label %1225, label %1174                                                                                 ;L2494<2897<242
 16456| 
 16457| 1174: ; preds = %1157, %1155, %1148, %1148, %1144
 16458|  br label %1137                                                                                                        ;L2493<2897<242
 16459| 
 16460| 1175: ; preds = %1139
 16462|     ;; self = ptr undef
 16463|     ;; self = ptr undef
 16465|     ;; self = ptr undef
 16468|     ;; self = ptr undef
 16469|     ;; count = i64 1
 16470|     ;; self = ptr %213
 16471|     ;; end_or_len = ptr %214
 16474|     ;; ptr = ptr %213
 16475|     ;; x = ptr %213
 16476|  %1176 = load ptr, ptr %213, , !!31516, !!8                                                                            ;L2494<138<2897<245
 16480|  %1177 = icmp eq ptr %1176, null                                                                                       ;L49<2494<138<2897<245
 16481|  br i1 %1177, label %1184, label %1178                                                                                 ;L49<2494<138<2897<245
 16482| 
 16483| 1178: ; preds = %1175
 16484|     ;; x = ptr %1176
 16485|  %1179 = gep %1176, i64 776                                                                                            ;L50<2494<138<2897<245
 16486|  %1180 = load i64, ptr %1179, , !!31516, !!8                                                                           ;L50<2494<138<2897<245
 16491|  %1181 = icmp sgt i64 %1180, -1                                                                                        ;L246<2893<50<2494<138<2897<245
 16492|  %1182 = icmp eq i64 %1180, -9223372036854775805                                                                       ;L246<2893<50<2494<138<2897<245
 16493|  %1183 = or i1 %1181, %1182                                                                                            ;L246<2893<50<2494<138<2897<245
 16494|  br i1 %1183, label %1226, label %1184                                                                                 ;L2494<138<2897<245
 16495| 
 16496| 1184: ; preds = %1178, %1175
 16497|     ;; self = ptr undef
 16498|     ;; count = i64 1
 16499|     ;; ptr = !DIArgList(ptr %213, i64 8)
 16500|     ;; self = !DIArgList(ptr %213, i64 8)
 16501|     ;; end_or_len = ptr %214
 16504|  %1185 = gep %213, i64 8                                                                                               ;L656<185<2493<138<2897<245
 16505|     ;; ptr = ptr %1185
 16506|     ;; x = ptr %1185
 16507|  %1186 = load ptr, ptr %1185, , !!31516, !!8                                                                           ;L2494<138<2897<245
 16511|  %1187 = icmp eq ptr %1186, null                                                                                       ;L49<2494<138<2897<245
 16512|  br i1 %1187, label %1194, label %1188                                                                                 ;L49<2494<138<2897<245
 16513| 
 16514| 1188: ; preds = %1184
 16515|     ;; x = ptr %1186
 16516|  %1189 = gep %1186, i64 776                                                                                            ;L50<2494<138<2897<245
 16517|  %1190 = load i64, ptr %1189, , !!31516, !!8                                                                           ;L50<2494<138<2897<245
 16522|  %1191 = icmp sgt i64 %1190, -1                                                                                        ;L246<2893<50<2494<138<2897<245
 16523|  %1192 = icmp eq i64 %1190, -9223372036854775805                                                                       ;L246<2893<50<2494<138<2897<245
 16524|  %1193 = or i1 %1191, %1192                                                                                            ;L246<2893<50<2494<138<2897<245
 16525|  br i1 %1193, label %1226, label %1194                                                                                 ;L2494<138<2897<245
 16526| 
 16527| 1194: ; preds = %1188, %1184
 16528|     ;; self = ptr undef
 16529|     ;; count = i64 1
 16530|     ;; ptr = !DIArgList(ptr %213, i64 16)
 16531|     ;; self = !DIArgList(ptr %213, i64 16)
 16532|     ;; end_or_len = ptr %214
 16535|  %1195 = gep %213, i64 16                                                                                              ;L656<185<2493<138<2897<245
 16536|     ;; ptr = ptr %1195
 16537|     ;; x = ptr %1195
 16538|  %1196 = load ptr, ptr %1195, , !!31516, !!8                                                                           ;L2494<138<2897<245
 16542|  %1197 = icmp eq ptr %1196, null                                                                                       ;L49<2494<138<2897<245
 16543|  br i1 %1197, label %1204, label %1198                                                                                 ;L49<2494<138<2897<245
 16544| 
 16545| 1198: ; preds = %1194
 16546|     ;; x = ptr %1196
 16547|  %1199 = gep %1196, i64 776                                                                                            ;L50<2494<138<2897<245
 16548|  %1200 = load i64, ptr %1199, , !!31516, !!8                                                                           ;L50<2494<138<2897<245
 16553|  %1201 = icmp sgt i64 %1200, -1                                                                                        ;L246<2893<50<2494<138<2897<245
 16554|  %1202 = icmp eq i64 %1200, -9223372036854775805                                                                       ;L246<2893<50<2494<138<2897<245
 16555|  %1203 = or i1 %1201, %1202                                                                                            ;L246<2893<50<2494<138<2897<245
 16556|  br i1 %1203, label %1226, label %1204                                                                                 ;L2494<138<2897<245
 16557| 
 16558| 1204: ; preds = %1198, %1194
 16559|     ;; self = ptr undef
 16560|     ;; count = i64 1
 16561|     ;; ptr = !DIArgList(ptr %213, i64 24)
 16562|     ;; self = !DIArgList(ptr %213, i64 24)
 16563|     ;; end_or_len = ptr %214
 16566|  %1205 = gep %213, i64 24                                                                                              ;L656<185<2493<138<2897<245
 16567|     ;; ptr = ptr %1205
 16568|     ;; x = ptr %1205
 16569|  %1206 = load ptr, ptr %1205, , !!31516, !!8                                                                           ;L2494<138<2897<245
 16573|  %1207 = icmp eq ptr %1206, null                                                                                       ;L49<2494<138<2897<245
 16574|  br i1 %1207, label %1214, label %1208                                                                                 ;L49<2494<138<2897<245
 16575| 
 16576| 1208: ; preds = %1204
 16577|     ;; x = ptr %1206
 16578|  %1209 = gep %1206, i64 776                                                                                            ;L50<2494<138<2897<245
 16579|  %1210 = load i64, ptr %1209, , !!31516, !!8                                                                           ;L50<2494<138<2897<245
 16584|  %1211 = icmp sgt i64 %1210, -1                                                                                        ;L246<2893<50<2494<138<2897<245
 16585|  %1212 = icmp eq i64 %1210, -9223372036854775805                                                                       ;L246<2893<50<2494<138<2897<245
 16586|  %1213 = or i1 %1211, %1212                                                                                            ;L246<2893<50<2494<138<2897<245
 16587|  br i1 %1213, label %1226, label %1214                                                                                 ;L2494<138<2897<245
 16588| 
 16589| 1214: ; preds = %1208, %1204
 16590|     ;; self = ptr undef
 16591|     ;; count = i64 1
 16592|     ;; ptr = !DIArgList(ptr %213, i64 32)
 16593|     ;; self = !DIArgList(ptr %213, i64 32)
 16594|     ;; end_or_len = ptr %214
 16597|  %1215 = gep %213, i64 32                                                                                              ;L656<185<2493<138<2897<245
 16598|     ;; ptr = ptr %1215
 16599|     ;; x = ptr %1215
 16600|  %1216 = load ptr, ptr %1215, , !!31516, !!8                                                                           ;L2494<138<2897<245
 16604|  %1217 = icmp eq ptr %1216, null                                                                                       ;L49<2494<138<2897<245
 16605|  br i1 %1217, label %1224, label %1218                                                                                 ;L49<2494<138<2897<245
 16606| 
 16607| 1218: ; preds = %1214
 16608|     ;; x = ptr %1216
 16609|  %1219 = gep %1216, i64 776                                                                                            ;L50<2494<138<2897<245
 16610|  %1220 = load i64, ptr %1219, , !!31516, !!8                                                                           ;L50<2494<138<2897<245
 16615|  %1221 = icmp sgt i64 %1220, -1                                                                                        ;L246<2893<50<2494<138<2897<245
 16616|  %1222 = icmp eq i64 %1220, -9223372036854775805                                                                       ;L246<2893<50<2494<138<2897<245
 16617|  %1223 = or i1 %1221, %1222                                                                                            ;L246<2893<50<2494<138<2897<245
 16618|  br i1 %1223, label %1226, label %1224                                                                                 ;L2494<138<2897<245
 16619| 
 16620| 1224: ; preds = %1218, %1214
 16621|     ;; self = ptr undef
 16622|     ;; count = i64 1
 16623|     ;; ptr = !DIArgList(ptr %213, i64 40)
 16624|     ;; self = !DIArgList(ptr %213, i64 40)
 16625|     ;; end_or_len = ptr %214
 16628|     ;; trajectory_possible = i1 false
 16630|  br label %1227                                                                                                        ;L252
 16631| 
 16632| 1225: ; preds = %1157
 16634|     ;; trajectory_possible = i8 1
 16635|  br label %1226                                                                                                        ;L247
 16636| 
 16637| 1226: ; preds = %1225, %1218, %1208, %1198, %1188, %1178
 16640|  invoke fastcc void @ai::small_actionNtB4_15SmallActionPlayNtNtCsjihNppCmMEE_4core5clone5Clone5clone(ptr %52, ptr %1128)
 16641|  to label %1228 unwind label %997                                                                                      ;L248
 16642| 
 16643| 1227: ; preds = %1232, %1224
 16646|  invoke fastcc void @ai::small_actionNtB4_15SmallActionPlayNtNtCsjihNppCmMEE_4core5clone5Clone5clone(ptr %43, ptr %1128)
 16647|  to label %1264 unwind label %997                                                                                      ;L269
 16648| 
 16649| 1228: ; preds = %1226
 16650|  %1229 = load i64, ptr %69, , !!8                                                                                      ;L248
 16651|  invoke void @ai::small_actionNtB2_15SmallActionPlay9get_input(ptr sret([32 x i8]) %53, ptr %52, i64 %1229, ptr %3, ptr %4, ptr %5, ptr %204, ptr %8)
 16652|  to label %1232 unwind label %1230                                                                                     ;L248
 16653| 
 16654| 1230: ; preds = %1228
 16655|  %1231 = cleanuppad within none []
 16656|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %52) #34 [ "funclet"(token %1231) ] ;L249
 16657|  cleanupret from %1231 unwind label %997                                                                               ;L249
 16658| 
 16659| 1232: ; preds = %1228
 16660|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %52)                  ;L249
 16662|  %1233 = load i64, ptr %53, , !!8                                                                                      ;L252
 16663|  switch i64 %1233, label %1240 [
 16664|  i64 -1, label %1227
 16665|  i64 0, label %1234
 16666|  ]                                                                                                                     ;L252
 16667| 
 16668| 1234: ; preds = %1232
 16669|  %1235 = gep %53, i64 16                                                                                               ;L252
 16670|  %1236 = load i64, ptr %1235,                                                                                          ;L252
 16671|     ;; move_action_input[16..+8] = i64 %1236
 16672|  %1237 = gep %53, i64 8                                                                                                ;L252
 16673|  %1238 = load i64, ptr %1237,                                                                                          ;L252
 16674|     ;; move_action_input[8..+8] = i64 %1238
 16675|     ;; x = i64 %1238
 16676|     ;; y = i64 %1236
 16678|  %1239 = load i64, ptr %69, , !!8                                                                                      ;L256
 16679|  invoke void @ai::position_eval26position_score_at_position(ptr sret([56 x i8]) %51, i64 %1239, ptr %4, ptr %5, ptr %204, i64 %1238, i64 %1236, i8 11)
 16680|  to label %1241 unwind label %997                                                                                      ;L256
 16681| 
 16682| 1240: ; preds = %1232
 16685|  invoke fastcc void @ai::small_actionNtB4_15SmallActionPlayNtNtCsjihNppCmMEE_4core5clone5Clone5clone(ptr %45, ptr %1128)
 16686|  to label %1257 unwind label %997                                                                                      ;L266
 16687| 
 16688| 1241: ; preds = %1234
 16689|  %1242 = gep %51, i64 48                                                                                               ;L258
 16690|  %1243 = load i8, ptr %1242, , !!8                                                                                     ;L258
 16691|  %1244 = trunc nuw i8 %1243 to i1                                                                                      ;L258
 16692|  br i1 %1244, label %1249, label %1245                                                                                 ;L258
 16693| 
 16694| 1245: ; preds = %1241
 16695|  %1246 = gep %51, i64 49                                                                                               ;L258
 16696|  %1247 = load i8, ptr %1246, , !!8                                                                                     ;L258
 16697|  %1248 = trunc nuw i8 %1247 to i1                                                                                      ;L258
 16698|     ;; on_trajectory = i1 %1248
 16700|  br i1 %1248, label %1251, label %1250                                                                                 ;L260
 16701| 
 16702| 1249: ; preds = %1241
 16703|     ;; on_trajectory = i8 1
 16705|  br label %1251                                                                                                        ;L260
 16706| 
 16707| 1250: ; preds = %1245
 16710|  invoke fastcc void @ai::small_actionNtB4_15SmallActionPlayNtNtCsjihNppCmMEE_4core5clone5Clone5clone(ptr %47, ptr %1128)
 16711|  to label %1252 unwind label %997                                                                                      ;L263
 16712| 
 16713| 1251: ; preds = %1249, %1245
 16716|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %49, ptr %5, ptr %4, i64 5, i1 zeroext false)
 16717|  to label %1254 unwind label %997                                                                                      ;L261
 16718| 
 16719| 1252: ; preds = %1250
 16720|  call void @llvm.memcpy.p0.p0.i64(ptr %48, ptr %47, i64 184, i1 false)                                                 ;L263
 16722|  invoke void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE12from_iter_inABU_j1_EBY_(ptr sret([32 x i8]) %0, ptr %48, ptr %184)
 16723|  to label %1253 unwind label %997                                                                                      ;L263
 16724| 
 16725| 1253: ; preds = %1252
 16727|  br label %1259                                                                                                        ;L260
 16728| 
 16729| 1254: ; preds = %1251
 16730|  call void @llvm.memcpy.p0.p0.i64(ptr %50, ptr %49, i64 136, i1 false)                                                 ;L261
 16732|  %1255 = gep %50, i64 177                                                                                              ;L261
 16733|  store i8 3, ptr %1255,                                                                                                ;L261
 16734|  invoke void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE12from_iter_inABU_j1_EBY_(ptr sret([32 x i8]) %0, ptr %50, ptr %184)
 16735|  to label %1256 unwind label %997                                                                                      ;L261
 16736| 
 16737| 1256: ; preds = %1254
 16739|  br label %1259                                                                                                        ;L260
 16740| 
 16741| 1257: ; preds = %1240
 16742|  call void @llvm.memcpy.p0.p0.i64(ptr %46, ptr %45, i64 184, i1 false)                                                 ;L266
 16744|  invoke void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE12from_iter_inABU_j1_EBY_(ptr sret([32 x i8]) %0, ptr %46, ptr %184)
 16745|  to label %1258 unwind label %997                                                                                      ;L266
 16746| 
 16747| 1258: ; preds = %1257
 16749|  br label %1259                                                                                                        ;L253
 16750| 
 16751| 1259: ; preds = %1265, %1258, %1256, %1253
 16754|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %59)
 16755|  to label %1263 unwind label %1260                                                                                     ;L825<276
 16756| 
 16757| 1260: ; preds = %1259
 16758|  %1261 = cleanuppad within none []
 16760|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %59) [ "funclet"(token %1261) ]
 16761|  to label %1262 unwind label %189                                                                                      ;L825<825<276
 16762| 
 16763| 1262: ; preds = %1260
 16764|  cleanupret from %1261 unwind label %189
 16765| 
 16766| 1263: ; preds = %1259
 16768|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %59)
 16769|  to label %1266 unwind label %189                                                                                      ;L825<825<276
 16770| 
 16771| 1264: ; preds = %1227
 16772|  call void @llvm.memcpy.p0.p0.i64(ptr %44, ptr %43, i64 184, i1 false)                                                 ;L269
 16774|  invoke void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE12from_iter_inABU_j1_EBY_(ptr sret([32 x i8]) %0, ptr %44, ptr %184)
 16775|  to label %1265 unwind label %997                                                                                      ;L269
 16776| 
 16777| 1265: ; preds = %1264
 16779|  br label %1259                                                                                                        ;L252
 16780| 
 16781| 1266: ; preds = %1263
 16784|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %63)
 16785|  to label %1270 unwind label %1267                                                                                     ;L825<276
 16786| 
 16787| 1267: ; preds = %1266
 16788|  %1268 = cleanuppad within none []
 16790|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %63) [ "funclet"(token %1268) ]
 16791|  to label %1269 unwind label %83                                                                                       ;L825<825<276
 16792| 
 16793| 1269: ; preds = %1267
 16794|  cleanupret from %1268 unwind label %83
 16795| 
 16796| 1270: ; preds = %1266
 16798|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %63)
 16799|  to label %1271 unwind label %83                                                                                       ;L825<825<276
 16800| 
 16801| 1271: ; preds = %1291, %1270
 16805|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %66)
 16806|  to label %1274 unwind label %1272                                                                                     ;L825<277
 16807| 
 16808| 1272: ; preds = %1271
 16809|  %1273 = cleanuppad within none []
 16811|  call void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %66) [ "funclet"(token %1273) ] ;L825<825<277
 16812|  cleanupret from %1273 unwind to caller                                                                                ;L825<277
 16813| 
 16814| 1274: ; preds = %1271
 16816|  call void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %66)                ;L825<825<277
 16818|  br label %81                                                                                                          ;L277
 16819| 
 16820| 1275: ; preds = %1126
 16821|  call void @llvm.memcpy.p0.p0.i64(ptr %56, ptr %55, i64 136, i1 false)                                                 ;L235
 16823|  %1276 = gep %56, i64 177                                                                                              ;L235
 16824|  store i8 3, ptr %1276,                                                                                                ;L235
 16825|  invoke void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE12from_iter_inABU_j1_EBY_(ptr sret([32 x i8]) %0, ptr %56, ptr %184)
 16826|  to label %1277 unwind label %997                                                                                      ;L235
 16827| 
 16828| 1277: ; preds = %1275
 16831|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %59)
 16832|  to label %1281 unwind label %1278                                                                                     ;L825<276
 16833| 
 16834| 1278: ; preds = %1277
 16835|  %1279 = cleanuppad within none []
 16837|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %59) [ "funclet"(token %1279) ]
 16838|  to label %1280 unwind label %189                                                                                      ;L825<825<276
 16839| 
 16840| 1280: ; preds = %1278
 16841|  cleanupret from %1279 unwind label %189
 16842| 
 16843| 1281: ; preds = %1277
 16845|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %59)
 16846|  to label %1282 unwind label %189                                                                                      ;L825<825<276
 16847| 
 16848| 1282: ; preds = %1281
 16851|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %63)
 16852|  to label %1286 unwind label %1283                                                                                     ;L825<276
 16853| 
 16854| 1283: ; preds = %1282
 16855|  %1284 = cleanuppad within none []
 16857|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %63) [ "funclet"(token %1284) ]
 16858|  to label %1285 unwind label %83                                                                                       ;L825<825<276
 16859| 
 16860| 1285: ; preds = %1283
 16861|  cleanupret from %1284 unwind label %83
 16862| 
 16863| 1286: ; preds = %1282
 16865|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %63)
 16866|  to label %1287 unwind label %83                                                                                       ;L825<825<276
 16867| 
 16868| 1287: ; preds = %1286
 16872|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %66)
 16873|  to label %1290 unwind label %1288                                                                                     ;L825<277
 16874| 
 16875| 1288: ; preds = %1287
 16876|  %1289 = cleanuppad within none []
 16878|  call void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %66) [ "funclet"(token %1289) ] ;L825<825<277
 16879|  cleanupret from %1289 unwind to caller                                                                                ;L825<277
 16880| 
 16881| 1290: ; preds = %1287
 16883|  call void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB17_(ptr %66)                ;L825<825<277
 16885|  br label %81                                                                                                          ;L1
 16886| 
 16887| 1291: ; preds = %1032
 16889|  br label %1271                                                                                                        ;L276
 16890| 
 16891| 1292: ; preds = %189
 16892|  cleanupret from %191 unwind label %83
 16893| 
 16894| 1293: ; preds = %189
 16895|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %63) #34 [ "funclet"(token %191) ] ;L276
 16896|  cleanupret from %191 unwind label %83                                                                                 ;L276
 16897| }
