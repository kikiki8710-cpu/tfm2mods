 40509| define void @ai::plan_legacy8sub_plan9epic_pokeNtB2_15EpicPokeSubPlan17action_candidates(ptr sret([32 x i8]) %0, ptr %1, i64 %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %7, ptr %8) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
 40510|  %10 = alloca [72 x i8],
 40511|  %11 = alloca [16 x i8],
 40512|  %12 = alloca [136 x i8],
 40513|  %13 = alloca [184 x i8],
 40514|  %14 = alloca [136 x i8],
 40515|  %15 = alloca [184 x i8],
 40522|  %16 = alloca [120 x i8],
 40523|  %17 = alloca [184 x i8],
 40524|  %18 = alloca [120 x i8],
 40525|  %19 = alloca [184 x i8],
 40526|  %20 = alloca [120 x i8],
 40527|  %21 = alloca [184 x i8],
 40528|  %22 = alloca [136 x i8],
 40529|  %23 = alloca [184 x i8],
 40530|  %24 = alloca [136 x i8],
 40531|  %25 = alloca [184 x i8],
 40532|  %26 = alloca [136 x i8],
 40533|  %27 = alloca [184 x i8],
 40535|  %28 = alloca [184 x i8],
 40536|  %29 = alloca [184 x i8],
 40537|  %30 = alloca [184 x i8],
 40538|  %31 = alloca [16 x i8],
 40540|  %32 = alloca [32 x i8],
 40541|  %33 = alloca [32 x i8],
 40542|  %34 = alloca [24 x i8],
 40543|  %35 = alloca [32 x i8],
 40544|  %36 = alloca [136 x i8],
 40545|  %37 = alloca [184 x i8],
 40546|  %38 = alloca [56 x i8],
 40551|  %39 = alloca [32 x i8],
 40552|  %40 = alloca [128 x i8],
 40553|  %41 = alloca [128 x i8],
 40554|  %42 = alloca [184 x i8],
 40555|  %43 = alloca [184 x i8],
 40556|  %44 = alloca [184 x i8],
 40557|  %45 = alloca [184 x i8],
 40558|  %46 = alloca [184 x i8],
 40559|  %47 = alloca [184 x i8],
 40560|  %48 = alloca [136 x i8],
 40561|  %49 = alloca [184 x i8],
 40562|  %50 = alloca [56 x i8],
 40563|  %51 = alloca [184 x i8],
 40564|  %52 = alloca [32 x i8],
 40569|  %53 = alloca [40 x i8],
 40570|  %54 = alloca [136 x i8],
 40571|  %55 = alloca [184 x i8],
 40582|  %56 = alloca [136 x i8],
 40583|  %57 = alloca [184 x i8],
 40585|  %58 = alloca [32 x i8],
 40586|  %59 = alloca [56 x i8],
 40587|  %60 = alloca [32 x i8],
 40588|  %61 = alloca [48 x i8],
 40589|  %62 = alloca [32 x i8],
 40590|  %63 = alloca [120 x i8],
 40591|  %64 = alloca [8 x i8],
 40592|  %65 = alloca [32 x i8],
 40593|  %66 = alloca [184 x i8],
 40594|  %67 = alloca [184 x i8],
 40595|  %68 = alloca [8 x i8],
 40596|  store i64 %2, ptr %68,
 40597|     ;; self = ptr %1
 40598|     ;; version = ptr %68
 40599|     ;; rnd = ptr %3
 40600|     ;; player = ptr %4
 40601|     ;; data = ptr %5
 40602|     ;; parameter = ptr %6
 40603|     ;; team_plan = ptr %7
 40604|     ;; debug = ptr %8
 40606|     ;; old_actions = ptr %65
 40607|     ;; nearest_enemy_tower = ptr %64
 40608|     ;; self = ptr %63
 40609|     ;; act_actions = ptr %62
 40610|     ;; move_actions = ptr %58
 40611|     ;; move_action_input = ptr %52
 40612|     ;; position_score = ptr %50
 40621|  call void @ai::plan_legacy9team_plan20objective_disciplineNtB4_8TeamPlan31v27_objective_discipline_action(ptr sret([184 x i8]) %67, ptr %7, i64 %2, ptr %3, ptr %4, ptr %5, i8 4) ;L15
 40622|  %69 = gep %67, i64 177                                                                                                ;L15
 40623|  %70 = load i8, ptr %69, , !!8                                                                                         ;L15
 40624|  %71 = icmp eq i8 %70, -1                                                                                              ;L15
 40625|  br i1 %71, label %76, label %72                                                                                       ;L15
 40626| 
 40627| 72: ; preds = %9
 40629|  call void @llvm.memcpy.p0.p0.i64(ptr %66, ptr %67, i64 184, i1 false)                                                 ;L15
 40630|  %73 = gep %5, i64 8                                                                                                   ;L16
 40631|  %74 = load ptr, ptr %73, , !!8, !!8                                                                                   ;L16
 40632|  %75 = load ptr, ptr %74, , !!8, !!8                                                                                   ;L16
 40633|  call void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE12from_iter_inABU_j1_EBY_(ptr sret([32 x i8]) %0, ptr %66, ptr %75) ;L16
 40636|  br label %80                                                                                                          ;L1
 40637| 
 40638| 76: ; preds = %9
 40641|  call void @ai::plan_legacy8sub_plan9epic_pokeNtB2_15EpicPokeSubPlan21action_candidates_old(ptr sret([32 x i8]) %65, ptr poison, i64 %2, ptr %3, ptr %4, ptr %5, ptr %6) ;L19
 40642|  %77 = gep %4, i64 2352                                                                                                ;L22
 40643|  %78 = load i64, ptr %77, , !!8                                                                                        ;L22
 40644|  %79 = icmp ult i64 %78, 2                                                                                             ;L22
 40645|  br i1 %79, label %85, label %81                                                                                       ;L22
 40646| 
 40647| 80: ; preds = %1613, %1505, %72
 40648|  ret void                                                                                                              ;L276
 40649| 
 40650| 81: ; preds = %76
 40651|  invoke void @core::panicking18panic_bounds_check(i64 %78, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.206) #31
 40652|  to label %84 unwind label %82                                                                                         ;L22
 40653| 
 40654| 82: ; preds = %1616, %1615, %1561, %1453, %167, %142, %136, %97, %95, %81
 40655|  %83 = cleanuppad within none []
 40656|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %65) #30 [ "funclet"(token %83) ] ;L276
 40657|  cleanupret from %83 unwind to caller                                                                                  ;L14
 40658| 
 40659| 84: ; preds = %1272, %97, %81
 40660|  unreachable
 40661| 
 40662| 85: ; preds = %76
 40663|     ;; self = ptr %4
 40664|  %86 = gep %4, i64 2496                                                                                                ;L581<22
 40665|  %87 = load i32, ptr %86, , !!8                                                                                        ;L581<22
 40666|  %88 = zext nneg i32 %87 to i64                                                                                        ;L581<22
 40667|  %89 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L22
 40668|  %90 = gep %89, i64 480                                                                                                ;L22
 40669|  %91 = getelementptr [5 x ptr], ptr %90, i64 %78                                                                       ;L22
 40670|  %92 = getelementptr ptr, ptr %91, i64 %88                                                                             ;L22
 40671|  %93 = load ptr, ptr %92, , !!8                                                                                        ;L22
 40672|     ;; self = ptr %93
 40673|  %94 = icmp eq ptr %93, null                                                                                           ;L1011<22
 40674|  br i1 %94, label %97, label %95                                                                                       ;L1011<22
 40675| 
 40676| 95: ; preds = %85
 40677|     ;; champ = ptr %93
 40680|  %96 = sub nuw nsw i64 1, %78                                                                                          ;L23
 40681|  invoke void @gc::simulationNtB5_21AbstractGameWithCache25iter_towers_without_nexus(ptr sret([120 x i8]) %63, ptr %89, i64 %96)
 40682|  to label %98 unwind label %82                                                                                         ;L23
 40683| 
 40684| 97: ; preds = %85
 40685|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.207) #31
 40686|  to label %84 unwind label %82                                                                                         ;L1013<22
 40687| 
 40688| 98: ; preds = %95
 40690|  call void @llvm.memcpy.p0.p0.i64(ptr %41, ptr %63, i64 120, i1 false)                                                 ;L28<957<24
 40693|     ;; f = ptr %93
 40694|     ;; self = ptr %41
 40697|     ;; f = ptr %93
 40698|  %99 = gep %41, i64 120                                                                                                ;L69<836<3387<25
 40699|  store ptr %93, ptr %99, , !!47450                                                                                     ;L69<836<3387<25
 40701|     ;; self = ptr %41
 40704|     ;; self = ptr %41
 40706|     ;; self = ptr %41
 40708|     ;; predicate = ptr %99
 40709|     ;; self = ptr %41
 40711|     ;; opt = ptr %41
 40712|     ;; self = ptr %41
 40714|  %100 = load i64, ptr %41, , !!47545, !!8                                                                              ;L764<332<169<98<107<2706<3416<3387<25
 40715|  %101 = icmp eq i64 %100, -1                                                                                           ;L764<332<169<98<107<2706<3416<3387<25
 40716|  br i1 %101, label %132, label %102                                                                                    ;L764<332<169<98<107<2706<3416<3387<25
 40717| 
 40718| 102: ; preds = %98
 40721|     ;; a = ptr %41
 40723|     ;; self = ptr %41
 40726|     ;; self = ptr %41
 40730|     ;; self = ptr %41
 40734|     ;; self = ptr %41
 40737|     ;; self = ptr %41
 40740|  %103 = trunc nuw i64 %100 to i1                                                                                       ;L396<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40741|  br i1 %103, label %104, label %130                                                                                    ;L396<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40742| 
 40743| 104: ; preds = %102
 40744|  %105 = gep %41, i64 8                                                                                                 ;L396<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40745|     ;; iter = ptr %105
 40747|     ;; self = ptr %105
 40752|     ;; self[0..+8] = ptr %105
 40753|     ;; self[8..+8] = i64 6
 40754|  %106 = gep %41, i64 24                                                                                                ;L214<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40755|     ;; data[0..+8] = ptr %106
 40756|     ;; data[8..+8] = i64 6
 40757|     ;; f[0..+8] = ptr %106
 40758|     ;; f[8..+8] = i64 6
 40762|     ;; self = ptr %105
 40763|     ;; self = ptr %105
 40764|     ;; self = ptr %105
 40766|     ;; rhs = i64 1
 40767|  %107 = load i64, ptr %105, , !!47731, !!8                                                                             ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40768|  %108 = gep %41, i64 16                                                                                                ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40769|  %109 = load i64, ptr %108, , !!47731, !!8                                                                             ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40770|  %110 = icmp ule i64 %107, %109                                                                                        ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40771|     ;; cond = i1 true
 40772|  call void @llvm.assume(i1 %110)                                                                                       ;L210<122<166<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40773|  %111 = icmp eq i64 %107, %109                                                                                         ;L167<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40774|  br i1 %111, label %130, label %112                                                                                    ;L167<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40775| 
 40776| 112: ; preds = %127, %104
 40777|  %113 = phi i64 [ %114, %127 ], [ %107, %104 ]
 40778|     ;; i = i64 %113
 40779|     ;; value = i64 %113
 40780|     ;; self = i64 %113
 40781|  %114 = add nuw nsw i64 %113, 1                                                                                        ;L971<63<169<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40783|     ;; f = ptr undef
 40785|     ;; idx = i64 %113
 40786|     ;; index = i64 %113
 40787|     ;; self = i64 %113
 40788|     ;; self[0..+8] = ptr %106
 40789|     ;; slice[0..+8] = ptr %106
 40790|     ;; self[8..+8] = i64 6
 40791|     ;; slice[8..+8] = i64 6
 40792|  %115 = icmp ult i64 %113, 6                                                                                           ;L252<646<219<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40793|  call void @llvm.assume(i1 %115)                                                                                       ;L252<646<219<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40794|  %116 = getelementptr ptr, ptr %106, i64 %113                                                                          ;L253<646<219<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40795|     ;; self = ptr %116
 40796|     ;; self = ptr %116
 40797|     ;; src = ptr %116
 40798|  %117 = load ptr, ptr %116, , !!47774, !!8                                                                             ;L1733<1171<798<219<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40799|     ;; elem = ptr %117
 40802|     ;; inner = ptr %117
 40803|  %118 = icmp eq ptr %117, null                                                                                         ;L819<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40804|  br i1 %118, label %127, label %119                                                                                    ;L819<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40805| 
 40806| 119: ; preds = %112
 40807|     ;; item = ptr %117
 40808|     ;; x = ptr %117
 40817|     ;; self = ptr %117
 40818|  %120 = gep %117, i64 1721                                                                                             ;L1478<24<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40819|  %121 = load i8, ptr %120, , !!47847, !!8                                                                              ;L1478<24<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40820|  %122 = trunc nuw i8 %121 to i1                                                                                        ;L1478<24<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40821|  %123 = gep %117, i64 1696                                                                                             ;L1478<24<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40822|  %124 = load i64, ptr %123, , !!47850                                                                                  ;L1478<24<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40823|  %125 = icmp eq i64 %124, 0                                                                                            ;L1478<24<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40824|  %126 = select i1 %122, i1 %125, i1 false                                                                              ;L1478<24<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40825|  br i1 %126, label %131, label %127                                                                                    ;L820<220<170<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40826| 
 40827| 127: ; preds = %119, %112
 40828|  %128 = icmp eq i64 %114, %109                                                                                         ;L167<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40829|  br i1 %128, label %129, label %112                                                                                    ;L167<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40830| 
 40831| 129: ; preds = %127
 40832|  store i64 %109, ptr %105, , !!47731                                                                                   ;L63<169<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40833|     ;; x = ptr null
 40834|  br label %130                                                                                                         ;L333<169<98<107<2706<3416<3387<25
 40835| 
 40836| 130: ; preds = %129, %104, %102
 40837|  store i64 -1, ptr %41, , !!47545                                                                                      ;L334<169<98<107<2706<3416<3387<25
 40838|  br label %132                                                                                                         ;L333<169<98<107<2706<3416<3387<25
 40839| 
 40840| 131: ; preds = %119
 40841|  store i64 %114, ptr %105, , !!47731                                                                                   ;L63<169<215<263<397<89<880<242<2971<169<332<169<98<107<2706<3416<3387<25
 40842|     ;; x = ptr %117
 40843|     ;; self = ptr %117
 40844|     ;; f[0..+8] = ptr %41
 40845|     ;; f[8..+8] = ptr %99
 40846|     ;; self = ptr %117
 40847|     ;; f = ptr %41
 40848|     ;; self = ptr %41
 40849|  br label %142                                                                                                         ;L1161<107<2706<3416<3387<25
 40850| 
 40851| 132: ; preds = %130, %98
 40852|  %133 = gep %41, i64 104                                                                                               ;L170<98<107<2706<3416<3387<25
 40853|     ;; self = ptr null
 40854|     ;; f[0..+8] = ptr %133
 40855|     ;; f[8..+8] = ptr %99
 40859|     ;; self = ptr %133
 40860|  %134 = load ptr, ptr %133, , !!47923, !!8                                                                             ;L764<170<1653<170<98<107<2706<3416<3387<25
 40861|  %135 = icmp eq ptr %134, null                                                                                         ;L764<170<1653<170<98<107<2706<3416<3387<25
 40862|  br i1 %135, label %167, label %136                                                                                    ;L764<170<1653<170<98<107<2706<3416<3387<25
 40863| 
 40864| 136: ; preds = %132
 40865|     ;; self = ptr %133
 40866|     ;; predicate = ptr %99
 40867|  %137 = invoke ptr @core::iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2p_4find5checkB1s_QNCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan9epic_pokeNtB3D_15EpicPokeSubPlan17action_candidates0E0INtNtNtBb_3ops12control_flow11ControlFlowB1s_EEB3J_(ptr %133, ptr %99)
 40868|  to label %138 unwind label %82                                                                                        ;L2971<170<1653<170<98<107<2706<3416<3387<25
 40869| 
 40870| 138: ; preds = %136
 40871|     ;; self = ptr %137
 40872|     ;; f = ptr %41
 40873|     ;; self = ptr %41
 40874|  %139 = icmp eq ptr %137, null                                                                                         ;L1161<107<2706<3416<3387<25
 40875|  br i1 %139, label %167, label %140                                                                                    ;L1161<107<2706<3416<3387<25
 40876| 
 40877| 140: ; preds = %138
 40878|  %141 = load ptr, ptr %99, , !!47403                                                                                   ;L310<1162<107<2706<3416<3387<25
 40879|  br label %142                                                                                                         ;L1161<107<2706<3416<3387<25
 40880| 
 40881| 142: ; preds = %140, %131
 40882|  %143 = phi ptr [ %93, %131 ], [ %141, %140 ]                                                                          ;L310<1162<107<2706<3416<3387<25
 40883|  %144 = phi ptr [ %117, %131 ], [ %137, %140 ]
 40884|     ;; f = ptr %99
 40885|     ;; self = ptr %99
 40886|     ;; x = ptr %144
 40887|     ;; args = ptr %144
 40889|     ;; x = ptr %144
 40893|     ;; self = ptr %144
 40894|     ;; other = ptr %143
 40895|  %145 = gep %144, i64 1632                                                                                             ;L2158<25<3379<310<1162<107<2706<3416<3387<25
 40896|  %146 = load i64, ptr %145, , !!47967, !!8                                                                             ;L2158<25<3379<310<1162<107<2706<3416<3387<25
 40897|     ;; x1 = i64 %146
 40898|     ;; self = i64 %146
 40899|  %147 = gep %144, i64 1640                                                                                             ;L2158<25<3379<310<1162<107<2706<3416<3387<25
 40900|  %148 = load i64, ptr %147, , !!47967, !!8                                                                             ;L2158<25<3379<310<1162<107<2706<3416<3387<25
 40901|     ;; y1 = i64 %148
 40902|     ;; self = i64 %148
 40903|  %149 = gep %143, i64 1632                                                                                             ;L2158<25<3379<310<1162<107<2706<3416<3387<25
 40904|  %150 = load i64, ptr %149, , !!47988, !!8                                                                             ;L2158<25<3379<310<1162<107<2706<3416<3387<25
 40905|     ;; x2 = i64 %150
 40906|     ;; other = i64 %150
 40907|  %151 = gep %143, i64 1640                                                                                             ;L2158<25<3379<310<1162<107<2706<3416<3387<25
 40908|  %152 = load i64, ptr %151, , !!47988, !!8                                                                             ;L2158<25<3379<310<1162<107<2706<3416<3387<25
 40909|     ;; y2 = i64 %152
 40910|     ;; other = i64 %152
 40911|  %153 = icmp ult i64 %146, %150                                                                                        ;L3147<7<2158<25<3379<310<1162<107<2706<3416<3387<25
 40912|  %154 = sub nuw i64 %150, %146                                                                                         ;L3147<7<2158<25<3379<310<1162<107<2706<3416<3387<25
 40913|  %155 = sub nuw i64 %146, %150                                                                                         ;L3147<7<2158<25<3379<310<1162<107<2706<3416<3387<25
 40914|  %156 = select i1 %153, i64 %154, i64 %155                                                                             ;L3147<7<2158<25<3379<310<1162<107<2706<3416<3387<25
 40915|     ;; dx = i64 %156
 40916|  %157 = icmp ult i64 %148, %152                                                                                        ;L3147<8<2158<25<3379<310<1162<107<2706<3416<3387<25
 40917|  %158 = sub nuw i64 %152, %148                                                                                         ;L3147<8<2158<25<3379<310<1162<107<2706<3416<3387<25
 40918|  %159 = sub nuw i64 %148, %152                                                                                         ;L3147<8<2158<25<3379<310<1162<107<2706<3416<3387<25
 40919|  %160 = select i1 %157, i64 %158, i64 %159                                                                             ;L3147<8<2158<25<3379<310<1162<107<2706<3416<3387<25
 40920|     ;; dy = i64 %160
 40921|  %161 = mul i64 %156, %156                                                                                             ;L9<2158<25<3379<310<1162<107<2706<3416<3387<25
 40922|  %162 = mul i64 %160, %160                                                                                             ;L9<2158<25<3379<310<1162<107<2706<3416<3387<25
 40923|  %163 = add i64 %162, %161                                                                                             ;L9<2158<25<3379<310<1162<107<2706<3416<3387<25
 40924|     ;; first[0..+8] = i64 %163
 40925|     ;; first[8..+8] = ptr %144
 40927|  call void @llvm.memcpy.p0.p0.i64(ptr %40, ptr %41, i64 128, i1 false), !!47403                                        ;L2707<3416<3387<25
 40928|  %164 = invoke { i64, ptr } @core::iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtB8_5chain5ChainINtNtB8_7flatten7FlattenINtNtNtBc_5array4iter8IntoIterINtNtBc_6option6OptionRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEKj6_EEINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterB2R_EEENCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan9epic_pokeNtB4R_15EpicPokeSubPlan17action_candidates0ENCINvNvNtNtNtBa_6traits8iterator8Iterator10min_by_key3keyB2R_yNCB4O_s_0E0EB6E_4foldTyB2R_ENCINvNvB6E_6min_by4foldB7S_INvB6C_7compareB2R_yEE0EB4X_(ptr %40, i64 %163, ptr %144)
 40929|  to label %165 unwind label %82                                                                                        ;L2707<3416<3387<25
 40930| 
 40931| 165: ; preds = %142
 40932|  %166 = extractvalue { i64, ptr } %164, 1                                                                              ;L2707<3416<3387<25
 40934|  br label %167                                                                                                         ;L2708<3416<3387<25
 40935| 
 40936| 167: ; preds = %165, %138, %132
 40937|  %168 = phi ptr [ %166, %165 ], [ null, %138 ], [ null, %132 ]                                                         ;L0<3416<3387<25
 40939|  store ptr %168, ptr %64,                                                                                              ;L23
 40942|     ;; self = ptr %65
 40943|     ;; self = ptr %65
 40944|  %169 = load ptr, ptr %65, , !!8, !!8                                                                                  ;L138<2073<28
 40945|     ;; p = ptr %169
 40946|  %170 = gep %65, i64 24                                                                                                ;L2075<28
 40947|  %171 = load i64, ptr %170, , !!8                                                                                      ;L2075<28
 40948|     ;; len = i64 %171
 40949|     ;; count = i64 %171
 40950|     ;; self[0..+8] = ptr %169
 40951|     ;; slice[0..+8] = ptr %169
 40952|     ;; self[8..+8] = i64 %171
 40953|     ;; slice[8..+8] = i64 %171
 40954|     ;; ptr = ptr %169
 40955|     ;; self = ptr %169
 40956|  %172 = gepS %169, i64 %171                                                                                            ;L961<100<1042<28
 40957|  %173 = load ptr, ptr %89, , !!8, !!8                                                                                  ;L29
 40958|  %174 = gep %89, i64 8                                                                                                 ;L29
 40959|  %175 = load ptr, ptr %174, , !!8, !!8                                                                                 ;L29
 40960|     ;; self[0..+8] = ptr %169
 40961|     ;; self[8..+8] = ptr %172
 40962|     ;; self[16..+8] = ptr %173
 40963|     ;; self[24..+8] = ptr %175
 40964|     ;; self[32..+8] = ptr %93
 40965|     ;; self[40..+8] = ptr %64
 40966|  store ptr %169, ptr %61,                                                                                              ;L69<836<91
 40967|  %176 = gep %61, i64 8                                                                                                 ;L69<836<91
 40968|  store ptr %172, ptr %176,                                                                                             ;L69<836<91
 40969|  %177 = gep %61, i64 16                                                                                                ;L69<836<91
 40970|  store ptr %173, ptr %177,                                                                                             ;L69<836<91
 40971|  %178 = gep %61, i64 24                                                                                                ;L69<836<91
 40972|  store ptr %175, ptr %178,                                                                                             ;L69<836<91
 40973|  %179 = gep %61, i64 32                                                                                                ;L69<836<91
 40974|  store ptr %93, ptr %179,                                                                                              ;L69<836<91
 40975|  %180 = gep %61, i64 40                                                                                                ;L69<836<91
 40976|  store ptr %64, ptr %180,                                                                                              ;L69<836<91
 40977|  %181 = gep %5, i64 8                                                                                                  ;L91
 40978|  %182 = load ptr, ptr %181, , !!8, !!8                                                                                 ;L91
 40979|  %183 = load ptr, ptr %182, , !!8, !!8                                                                                 ;L91
 40980|  invoke void @core::iter8adapters3map3MapINtNtB29_6filter6FilterINtNtNtB2d_5slice4iter4IterBU_ENCNvMNtNtNtBY_11plan_legacy8sub_plan9epic_pokeNtB3P_15EpicPokeSubPlan17action_candidatess0_0ENCB3M_s1_0EEBY_(ptr sret([32 x i8]) %62, ptr %61, ptr %183)
 40981|  to label %184 unwind label %82                                                                                        ;L28
 40982| 
 40983| 184: ; preds = %167
 40986|  store ptr %5, ptr %60,                                                                                                ;L93
 40987|  %185 = gep %60, i64 8                                                                                                 ;L93
 40988|  store ptr %93, ptr %185,                                                                                              ;L93
 40989|  %186 = gep %60, i64 16                                                                                                ;L93
 40990|  store ptr %4, ptr %186,                                                                                               ;L93
 40991|  %187 = gep %60, i64 24                                                                                                ;L93
 40992|  store ptr %68, ptr %187,                                                                                              ;L93
 40993|  invoke void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE6retainNCNvMNtNtNtBY_11plan_legacy8sub_plan9epic_pokeNtB22_15EpicPokeSubPlan17action_candidatess2_0EBY_(ptr %62, ptr %60)
 40994|  to label %191 unwind label %188                                                                                       ;L93
 40995| 
 40996| 188: ; preds = %1526, %1416, %1155, %1106, %1090, %1089, %191, %184
 40997|  %189 = phi i1 [ true, %1526 ], [ true, %1106 ], [ true, %1416 ], [ false, %1155 ], [ true, %1090 ], [ true, %191 ], [ true, %184 ], [ true, %1089 ] ;L0
 40998|  %190 = cleanuppad within none []
 40999|  br i1 %189, label %1616, label %1615                                                                                  ;L275
 41000| 
 41001| 191: ; preds = %184
 41004|  store ptr %1, ptr %59,                                                                                                ;L199
 41005|  %192 = gep %59, i64 8                                                                                                 ;L199
 41006|  store ptr %68, ptr %192,                                                                                              ;L199
 41007|  %193 = gep %59, i64 16                                                                                                ;L199
 41008|  store ptr %6, ptr %193,                                                                                               ;L199
 41009|  %194 = gep %59, i64 24                                                                                                ;L199
 41010|  store ptr %3, ptr %194,                                                                                               ;L199
 41011|  %195 = gep %59, i64 32                                                                                                ;L199
 41012|  store ptr %4, ptr %195,                                                                                               ;L199
 41013|  %196 = gep %59, i64 40                                                                                                ;L199
 41014|  store ptr %5, ptr %196,                                                                                               ;L199
 41015|  %197 = gep %59, i64 48                                                                                                ;L199
 41016|  store ptr %8, ptr %197,                                                                                               ;L199
 41017|  invoke void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE6retainNCNvMNtNtNtBY_11plan_legacy8sub_plan9epic_pokeNtB22_15EpicPokeSubPlan17action_candidatess3_0EBY_(ptr %62, ptr %59)
 41018|  to label %202 unwind label %188                                                                                       ;L199
 41019| 
 41020| 198: ; preds = %1082, %1045, %288, %283, %280, %279, %264, %263, %261, %251, %239, %227, %220
 41021|  %199 = phi i1 [ true, %1082 ], [ true, %279 ], [ false, %1045 ], [ true, %288 ], [ true, %283 ], [ true, %261 ], [ true, %280 ], [ true, %239 ], [ true, %263 ], [ true, %264 ], [ true, %251 ], [ true, %220 ], [ true, %227 ] ;L0<204
 41022|  %200 = cleanuppad within none []
 41023|  br i1 %199, label %1090, label %1089                                                                                  ;L424<204
 41024| 
 41025| 201: ; preds = %1079, %1026, %734, %709, %263
 41026|  unreachable
 41027| 
 41028| 202: ; preds = %191
 41031|  %203 = gep %6, i64 2544                                                                                               ;L204
 41032|  %204 = load i64, ptr %68, , !!8                                                                                       ;L204
 41036|     ;; version = i64 %204
 41038|     ;; rnd = ptr %3
 41039|     ;; player = ptr %4
 41040|     ;; data = ptr %5
 41041|     ;; positioning_score = ptr %203
 41042|     ;; debug = ptr %8
 41043|     ;; res = ptr %39
 41044|     ;; position_score = ptr %38
 41045|     ;; near_enemies = ptr %35
 41046|     ;; jrng = ptr %31
 41047|     ;; len = i64 5
 41048|     ;; count = i64 5
 41050|     ;; count = i64 5
 41051|     ;; index = i64 0
 41052|     ;; self = i64 0
 41053|     ;; count = i64 1
 41054|     ;; count = i64 1
 41055|     ;; index = i64 0
 41056|     ;; self = i64 0
 41057|     ;; count = i64 5
 41060|  %205 = load ptr, ptr %182, , !!48147, !!8, !!8                                                                        ;L279<204
 41061|     ;; bump = ptr %205
 41062|     ;; bump = ptr %205
 41063|  store ptr inttoptr (i64 8 to ptr), ptr %39, , !!48147                                                                 ;L547<279<204
 41064|  %206 = gep %39, i64 8                                                                                                 ;L547<279<204
 41065|  store ptr %205, ptr %206, , !!48147                                                                                   ;L547<279<204
 41066|  %207 = gep %39, i64 16                                                                                                ;L547<279<204
 41067|  %208 = gep %39, i64 24                                                                                                ;L547<279<204
 41068|  call void @llvm.memset.p0.i64(ptr %207, i8 0, i64 16, i1 false), !!48147                                              ;L547<279<204
 41069|     ;; self = ptr %89
 41070|     ;; self = ptr %89
 41071|     ;; self = ptr %89
 41072|  %209 = load ptr, ptr %92, , !!48170, !!8                                                                              ;L281<204
 41073|     ;; self = ptr %209
 41074|     ;; self = ptr %209
 41075|     ;; self = ptr %209
 41076|  %210 = icmp eq ptr %209, null                                                                                         ;L1011<281<204
 41077|  br i1 %210, label %263, label %211                                                                                    ;L1011<281<204
 41078| 
 41079| 211: ; preds = %202
 41080|     ;; champ = ptr %209
 41081|     ;; champ = ptr %209
 41082|     ;; champ = ptr %209
 41083|     ;; self = ptr %209
 41084|     ;; self = ptr %209
 41085|     ;; self = ptr %209
 41086|     ;; entity = ptr %209
 41087|     ;; caster = ptr %209
 41088|     ;; self = ptr %209
 41089|     ;; self = ptr %209
 41090|     ;; team = i64 %96
 41091|     ;; team = i64 %96
 41092|     ;; team = i64 %96
 41093|  %212 = getelementptr [5 x ptr], ptr %90, i64 %96                                                                      ;L1905<284<204
 41094|     ;; self[0..+8] = ptr %212
 41095|     ;; slice[0..+8] = ptr %212
 41096|     ;; self[8..+8] = i64 5
 41097|     ;; slice[8..+8] = i64 5
 41098|     ;; ptr = ptr %212
 41099|     ;; self = ptr %212
 41100|  %213 = gep %212, i64 40                                                                                               ;L961<100<1042<1905<284<204
 41101|     ;; self = ptr undef
 41102|     ;; self = ptr undef
 41103|     ;; f[0..+8] = ptr undef
 41104|     ;; f[8..+8] = ptr %4
 41105|     ;; f[16..+8] = ptr %5
 41106|     ;; f[24..+8] = ptr %209
 41107|     ;; fold[0..+8] = ptr undef
 41108|     ;; fold[8..+8] = ptr %4
 41109|     ;; fold[16..+8] = ptr %5
 41110|     ;; fold[24..+8] = ptr %209
 41113|     ;; f[8..+8] = ptr undef
 41114|     ;; f[16..+8] = ptr %4
 41115|     ;; f[24..+8] = ptr %5
 41116|     ;; f[32..+8] = ptr %209
 41117|     ;; self = ptr undef
 41120|     ;; self = ptr undef
 41121|     ;; count = i64 1
 41122|     ;; ptr = ptr %212
 41123|     ;; self = ptr %212
 41124|     ;; end_or_len = ptr %213
 41127|  br label %214                                                                                                         ;L180<2493<138<2897<284<204
 41128| 
 41129| 214: ; preds = %231, %211
 41130|  %215 = phi i64 [ 0, %211 ], [ %217, %231 ]
 41131|  %216 = gep %212, i64 %215                                                                                             ;L656<185<2493<138<2897<284<204
 41132|     ;; ptr = ptr %216
 41133|  %217 = add nuw nsw i64 %215, 8                                                                                        ;L656<185<2493<138<2897<284<204
 41134|     ;; x = ptr %216
 41135|  %218 = load ptr, ptr %216, , !!48260, !!8                                                                             ;L2494<138<2897<284<204
 41136|     ;; f = ptr undef
 41140|  %219 = icmp eq ptr %218, null                                                                                         ;L49<2494<138<2897<284<204
 41141|  br i1 %219, label %231, label %220                                                                                    ;L49<2494<138<2897<284<204
 41142| 
 41143| 220: ; preds = %214
 41144|     ;; x = ptr %218
 41147|     ;; x = ptr %218
 41152|     ;; c = ptr %218
 41153|     ;; self = ptr %218
 41154|     ;; self = ptr %218
 41155|     ;; self = ptr %218
 41156|     ;; self = ptr %218
 41157|     ;; self = ptr %218
 41158|  %221 = invoke zeroext i1 @ai::utils26nontarget_windup_perceived(i64 %204, ptr %4, ptr %5, ptr %218)
 41159|  to label %222 unwind label %198, !!48170                                                                              ;L285<2893<50<2494<138<2897<284<204
 41160| 
 41161| 222: ; preds = %220
 41162|  %223 = gep %218, i64 104
 41163|  %224 = load i64, ptr %223, , !!48318
 41164|  %225 = icmp eq i64 %224, 13
 41165|  %226 = select i1 %221, i1 %225, i1 false                                                                              ;L285<2893<50<2494<138<2897<284<204
 41166|  br i1 %226, label %233, label %231                                                                                    ;L285<2893<50<2494<138<2897<284<204
 41167| 
 41168| 227: ; preds = %253, %253, %243, %243, %241
 41169|  %228 = phi ptr [ %248, %243 ], [ %242, %241 ], [ %248, %243 ], [ %258, %253 ], [ %258, %253 ]
 41170|  %229 = invoke zeroext i1 @gc::simulation6effectNtB2_6Effect11is_in_range(ptr %228, ptr %218, ptr %209)
 41171|  to label %230 unwind label %198, !!48170                                                                              ;L0<2893<50<2494<138<2897<284<204
 41172| 
 41173| 230: ; preds = %227
 41174|  br i1 %229, label %264, label %231                                                                                    ;L2494<138<2897<284<204
 41175| 
 41176| 231: ; preds = %253, %243, %236, %233, %230, %222, %214
 41177|     ;; self = ptr undef
 41178|     ;; count = i64 1
 41179|     ;; ptr = !DIArgList(ptr %212, i64 %217)
 41180|     ;; self = !DIArgList(ptr %212, i64 %217)
 41181|     ;; end_or_len = ptr %213
 41184|  %232 = icmp eq i64 %217, 40                                                                                           ;L1714<180<2493<138<2897<284<204
 41185|  br i1 %232, label %264, label %214                                                                                    ;L180<2493<138<2897<284<204
 41186| 
 41187| 233: ; preds = %222
 41188|     ;; champ = ptr %218
 41189|  %234 = gep %218, i64 112                                                                                              ;L1572<286<2893<50<2494<138<2897<284<204
 41190|  %235 = load i64, ptr %234, , !!48318, !!8                                                                             ;L1572<286<2893<50<2494<138<2897<284<204
 41191|  switch i64 %235, label %231 [
 41192|  i64 4, label %236
 41193|  i64 5, label %243
 41194|  i64 6, label %253
 41195|  ]                                                                                                                     ;L286<2893<50<2494<138<2897<284<204
 41196| 
 41197| 236: ; preds = %233
 41198|     ;; self = ptr %218
 41199|  %237 = gep %218, i64 1272                                                                                             ;L742<286<2893<50<2494<138<2897<284<204
 41200|  %238 = load i32, ptr %237, , !!48318, !!8                                                                             ;L742<286<2893<50<2494<138<2897<284<204
 41201|  switch i32 %238, label %231 [
 41202|  i32 -1, label %239
 41203|  i32 1, label %241
 41204|  i32 2, label %241
 41205|  ]                                                                                                                     ;L742<286<2893<50<2494<138<2897<284<204
 41206| 
 41207| 239: ; preds = %236
 41208|     ;; self = ptr null
 41209|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.59) #31
 41210|  to label %240 unwind label %198, !!48170                                                                              ;L1013<286<2893<50<2494<138<2897<284<204
 41211| 
 41212| 240: ; preds = %239
 41213|  unreachable                                                                                                           ;L1013<286<2893<50<2494<138<2897<284<204
 41214| 
 41215| 241: ; preds = %236, %236
 41216|  %242 = gep %218, i64 1224                                                                                             ;L742<286<2893<50<2494<138<2897<284<204
 41217|     ;; self = ptr %218
 41218|     ;; self = ptr %242
 41219|  br label %227                                                                                                         ;L286<2893<50<2494<138<2897<284<204
 41220| 
 41221| 243: ; preds = %233
 41222|  %244 = gep %218, i64 1480                                                                                             ;L1693<288<2893<50<2494<138<2897<284<204
 41223|  %245 = load i64, ptr %244, , !!48318, !!8                                                                             ;L1693<288<2893<50<2494<138<2897<284<204
 41224|  %246 = icmp ugt i64 %245, 2                                                                                           ;L1693<288<2893<50<2494<138<2897<284<204
 41225|  %247 = gep %218, i64 1280                                                                                             ;L1693<288<2893<50<2494<138<2897<284<204
 41226|  %248 = select i1 %246, ptr %247, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.19                                        ;L1693<288<2893<50<2494<138<2897<284<204
 41227|     ;; self = ptr %248
 41228|  %249 = gep %248, i64 48                                                                                               ;L742<288<2893<50<2494<138<2897<284<204
 41229|  %250 = load i32, ptr %249, , !!48318, !!8                                                                             ;L742<288<2893<50<2494<138<2897<284<204
 41230|  switch i32 %250, label %231 [
 41231|  i32 -1, label %251
 41232|  i32 1, label %227
 41233|  i32 2, label %227
 41234|  ]                                                                                                                     ;L742<288<2893<50<2494<138<2897<284<204
 41235| 
 41236| 251: ; preds = %243
 41237|     ;; self = ptr null
 41238|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.60) #31
 41239|  to label %252 unwind label %198, !!48170                                                                              ;L1013<288<2893<50<2494<138<2897<284<204
 41240| 
 41241| 252: ; preds = %251
 41242|  unreachable                                                                                                           ;L1013<288<2893<50<2494<138<2897<284<204
 41243| 
 41244| 253: ; preds = %233
 41245|  %254 = gep %218, i64 1480                                                                                             ;L1701<290<2893<50<2494<138<2897<284<204
 41246|  %255 = load i64, ptr %254, , !!48318, !!8                                                                             ;L1701<290<2893<50<2494<138<2897<284<204
 41247|  %256 = icmp ugt i64 %255, 4                                                                                           ;L1701<290<2893<50<2494<138<2897<284<204
 41248|  %257 = gep %218, i64 1336                                                                                             ;L1701<290<2893<50<2494<138<2897<284<204
 41249|  %258 = select i1 %256, ptr %257, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.19                                        ;L1701<290<2893<50<2494<138<2897<284<204
 41250|     ;; self = ptr %258
 41251|  %259 = gep %258, i64 48                                                                                               ;L742<290<2893<50<2494<138<2897<284<204
 41252|  %260 = load i32, ptr %259, , !!48318, !!8                                                                             ;L742<290<2893<50<2494<138<2897<284<204
 41253|  switch i32 %260, label %231 [
 41254|  i32 -1, label %261
 41255|  i32 1, label %227
 41256|  i32 2, label %227
 41257|  ]                                                                                                                     ;L742<290<2893<50<2494<138<2897<284<204
 41258| 
 41259| 261: ; preds = %253
 41260|     ;; self = ptr null
 41261|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.61) #31
 41262|  to label %262 unwind label %198, !!48170                                                                              ;L1013<290<2893<50<2494<138<2897<284<204
 41263| 
 41264| 262: ; preds = %261
 41265|  unreachable                                                                                                           ;L1013<290<2893<50<2494<138<2897<284<204
 41266| 
 41267| 263: ; preds = %202
 41268|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.202) #31
 41269|  to label %201 unwind label %198, !!48170                                                                              ;L1013<281<204
 41270| 
 41271| 264: ; preds = %231, %230
 41272|  %265 = phi i1 [ false, %231 ], [ true, %230 ]                                                                         ;L1714<180<2493<138<2897<284<204
 41273|     ;; has_non_target_action_range = i1 %265
 41275|  %266 = gep %209, i64 1632                                                                                             ;L298<204
 41276|  %267 = load i64, ptr %266, , !!48170, !!8                                                                             ;L298<204
 41277|     ;; x1 = i64 %267
 41278|     ;; self = i64 %267
 41279|     ;; x1 = i64 %267
 41280|     ;; self = i64 %267
 41281|     ;; x1 = i64 %267
 41282|     ;; self = i64 %267
 41283|  %268 = gep %209, i64 1640                                                                                             ;L298<204
 41284|  %269 = load i64, ptr %268, , !!48170, !!8                                                                             ;L298<204
 41285|     ;; y1 = i64 %269
 41286|     ;; self = i64 %269
 41287|     ;; y1 = i64 %269
 41288|     ;; self = i64 %269
 41289|     ;; y1 = i64 %269
 41290|     ;; self = i64 %269
 41291|  invoke void @ai::position_eval26position_score_at_position(ptr sret([56 x i8]) %38, i64 %204, ptr %4, ptr %5, ptr %203, i64 %267, i64 %269, i8 11)
 41292|  to label %270 unwind label %198, !!48170                                                                              ;L297<204
 41293| 
 41294| 270: ; preds = %264
 41295|  %271 = gep %38, i64 48                                                                                                ;L299<204
 41296|  %272 = load i8, ptr %271, , !!48147, !!8                                                                              ;L299<204
 41297|  %273 = trunc nuw i8 %272 to i1                                                                                        ;L299<204
 41298|  %274 = gep %38, i64 49                                                                                                ;L299<204
 41299|  %275 = load i8, ptr %274, , !!48147                                                                                   ;L299<204
 41300|  %276 = trunc nuw i8 %275 to i1                                                                                        ;L299<204
 41301|     ;; on_trajectory = i1 %276
 41302|  %277 = or i1 %265, %276
 41303|  %278 = select i1 %273, i1 true, i1 %277                                                                               ;L299<204
 41304|  br i1 %278, label %279, label %280                                                                                    ;L299<204
 41305| 
 41306| 279: ; preds = %270
 41309|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %36, ptr %5, ptr %4, i64 5, i1 zeroext true)
 41310|  to label %1080 unwind label %198, !!48170                                                                             ;L301<204
 41311| 
 41312| 280: ; preds = %270
 41313|     ;; runaway = i8 0
 41314|  %281 = gep %4, i64 384                                                                                                ;L313<204
 41315|  %282 = invoke i64 @gc::simulation5state6playerNtB4_16AthleteParameter20positioning_accuracy(ptr %281)
 41316|  to label %283 unwind label %198, !!48170                                                                              ;L313<204
 41317| 
 41318| 283: ; preds = %280
 41319|     ;; positioning_accuracy = i64 %282
 41320|     ;; min_v = i64 %282
 41321|  %284 = sub i64 2000, %282                                                                                             ;L315<204
 41322|     ;; max_v = i64 %284
 41325|     ;; self[0..+8] = ptr %212
 41326|     ;; slice[0..+8] = ptr %212
 41327|     ;; self[8..+8] = i64 5
 41328|     ;; slice[8..+8] = i64 5
 41329|     ;; self = ptr %212
 41330|     ;; self[0..+8] = ptr %212
 41331|     ;; self[8..+8] = ptr %213
 41332|     ;; predicate = ptr %209
 41333|  store ptr %212, ptr %34, , !!48147                                                                                    ;L28<957<317<204
 41334|  %285 = gep %34, i64 8                                                                                                 ;L28<957<317<204
 41335|  store ptr %213, ptr %285, , !!48147                                                                                   ;L28<957<317<204
 41336|  %286 = gep %34, i64 16                                                                                                ;L28<957<317<204
 41337|  store ptr %209, ptr %286, , !!48147                                                                                   ;L28<957<317<204
 41338|  invoke void @core::iter8adapters6filter6FilterINtNtB29_10filter_map9FilterMapINtNtNtB2d_5slice4iter4IterINtNtB2d_6option6OptionBU_EENCNvMs3_BZ_NtBZ_21AbstractGameWithCache14iter_champions0ENCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan9epic_pokeNtB5m_15EpicPokeSubPlan15get_move_actions_0EEB5s_(ptr sret([32 x i8]) %35, ptr %34, ptr %205)
 41339|  to label %287 unwind label %198, !!48170                                                                              ;L317<204
 41340| 
 41341| 287: ; preds = %283
 41344|  invoke void @gc::simulation6entity6EntityENtNtCsjihNppCmMEE_4core5clone5Clone5cloneCshdEBA0ozCnw_7game_ai(ptr sret([32 x i8]) %33, ptr %35)
 41345|  to label %290 unwind label %288, !!48170                                                                              ;L318<204
 41346| 
 41347| 288: ; preds = %1079, %1071, %1062, %1026, %1019, %1010, %829, %814, %800, %797, %794, %791, %790, %789, %758, %734, %718, %710, %709, %697, %693, %684, %677, %656, %588, %562, %542, %536, %527, %507, %501, %472, %452, %446, %432, %430, %428, %387, %383, %381, %377, %375, %371, %365, %309, %294, %290, %287
 41348|  %289 = cleanuppad within none []
 41349|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEECshdEBA0ozCnw_7game_ai(ptr %35) #30 [ "funclet"(token %289) ], !!48170 ;L424<204
 41350|  cleanupret from %289 unwind label %198                                                                                ;L424<204
 41351| 
 41352| 290: ; preds = %287
 41354|  store ptr inttoptr (i64 8 to ptr), ptr %32, , !!48147                                                                 ;L547<319<204
 41355|  %291 = gep %32, i64 8                                                                                                 ;L547<319<204
 41356|  store ptr %205, ptr %291, , !!48147                                                                                   ;L547<319<204
 41357|  %292 = gep %32, i64 16                                                                                                ;L547<319<204
 41358|  call void @llvm.memset.p0.i64(ptr %292, i8 0, i64 16, i1 false), !!48147                                              ;L547<319<204
 41359|  %293 = invoke i64 @ai::fight_check19check_kill_die_tick(i64 %204, ptr %3, ptr %5, ptr %4, ptr %209, ptr %33, ptr %32, ptr %8)
 41360|  to label %294 unwind label %288, !!48170                                                                              ;L318<204
 41361| 
 41362| 294: ; preds = %290
 41363|     ;; me_die_tick = i64 %293
 41366|  %295 = load ptr, ptr %89, , !!48170, !!8, !!8                                                                         ;L321<204
 41367|  %296 = load ptr, ptr %174, , !!48170, !!8, !!8                                                                        ;L321<204
 41368|  %297 = gep %296, i64 64                                                                                               ;L321<204
 41369|  %298 = load ptr, ptr %297, , !!48170, !!8                                                                             ;L321<204
 41370|  %299 = invoke { i64, ptr } %298(ptr %295)
 41371|  to label %300 unwind label %288, !!48170                                                                              ;L321<204
 41372| 
 41373| 300: ; preds = %294
 41374|  %301 = extractvalue { i64, ptr } %299, 0                                                                              ;L321<204
 41375|     ;; self[0..+8] = i64 %301
 41377|  %302 = icmp eq i64 %301, 0                                                                                            ;L231<321<204
 41378|  br i1 %302, label %303, label %1079                                                                                   ;L231<321<204
 41379| 
 41380| 303: ; preds = %300
 41381|  %304 = extractvalue { i64, ptr } %299, 1                                                                              ;L321<204
 41382|     ;; self[8..+8] = ptr %304
 41383|  %305 = icmp ne ptr %304, null
 41384|  call void @llvm.assume(i1 %305)
 41385|     ;; self = ptr %304
 41386|     ;; self = ptr %304
 41387|     ;; self = ptr %304
 41388|     ;; self = ptr %304
 41389|  %306 = gep %304, i64 424                                                                                              ;L1864<3787<321<204
 41390|  %307 = load i64, ptr %306, , !!48170, !!8                                                                             ;L1864<3787<321<204
 41393|     ;; self[8..+8] = i64 %307
 41394|     ;; slice[8..+8] = i64 %307
 41395|  %308 = icmp eq i64 %307, 0                                                                                            ;L219<576<321<204
 41396|  br i1 %308, label %316, label %309                                                                                    ;L219<576<321<204
 41397| 
 41398| 309: ; preds = %303
 41399|  %310 = gep %304, i64 416                                                                                              ;L614<609<296<1968<1864<3787<321<204
 41400|  %311 = load ptr, ptr %310, , !!48170, !!8, !!8                                                                        ;L614<609<296<1968<1864<3787<321<204
 41401|     ;; self[0..+8] = ptr %311
 41402|     ;; slice[0..+8] = ptr %311
 41403|     ;; self = ptr %311
 41404|     ;; f[0..+8] = ptr %295
 41405|     ;; f[8..+8] = ptr %296
 41406|     ;; x = ptr %311
 41407|  %312 = gep %296, i64 496                                                                                              ;L1543<322<204
 41408|  %313 = load ptr, ptr %312, , !!48170                                                                                  ;L1543<322<204
 41409|  %314 = load i64, ptr %311, , !!48170, !!8                                                                             ;L1543<322<204
 41411|  %315 = invoke ptr %313(ptr %295, i64 %314)
 41412|  to label %316 unwind label %288, !!48170                                                                              ;L322<1543<322<204
 41413| 
 41414| 316: ; preds = %309, %303
 41415|  %317 = phi ptr [ null, %303 ], [ %315, %309 ]                                                                         ;L0<322<204
 41416|     ;; objective_entity = ptr %317
 41417|     ;; self = ptr %35
 41418|     ;; self = ptr %35
 41419|     ;; self = ptr %35
 41420|  %318 = load ptr, ptr %35, , !!48147, !!8, !!8                                                                         ;L138<2073<2136<324<204
 41421|     ;; p = ptr %318
 41422|  %319 = gep %35, i64 24                                                                                                ;L2075<2136<324<204
 41423|  %320 = load i64, ptr %319, , !!48147, !!8                                                                             ;L2075<2136<324<204
 41424|     ;; len = i64 %320
 41425|     ;; count = i64 %320
 41426|     ;; self[0..+8] = ptr %318
 41427|     ;; slice[0..+8] = ptr %318
 41428|     ;; self[8..+8] = i64 %320
 41429|     ;; slice[8..+8] = i64 %320
 41430|     ;; ptr = ptr %318
 41431|     ;; self = ptr %318
 41432|  %321 = getelementptr ptr, ptr %318, i64 %320                                                                          ;L961<100<1042<2136<324<204
 41433|     ;; iter[0..+8] = ptr %318
 41434|     ;; iter[8..+8] = ptr %321
 41435|  %322 = gep %31, i64 8
 41436|  %323 = icmp eq ptr %317, null
 41437|  %324 = gep %317, i64 1632
 41438|  %325 = gep %317, i64 1640
 41439|  %326 = gep %182, i64 8
 41440|  %327 = gep %29, i64 85
 41441|  %328 = gep %29, i64 88
 41442|  %329 = gep %29, i64 96
 41443|  %330 = gep %29, i64 104
 41444|  %331 = gep %29, i64 112
 41445|  %332 = gep %29, i64 120
 41446|  %333 = gep %29, i64 128
 41447|  %334 = gep %29, i64 136
 41448|  %335 = gep %29, i64 149
 41449|  %336 = gep %29, i64 177
 41450|  %337 = gep %30, i64 85
 41451|  %338 = gep %30, i64 88
 41452|  %339 = gep %30, i64 96
 41453|  %340 = gep %30, i64 104
 41454|  %341 = gep %30, i64 112
 41455|  %342 = gep %30, i64 120
 41456|  %343 = gep %30, i64 128
 41457|  %344 = gep %30, i64 136
 41458|  %345 = gep %30, i64 149
 41459|  %346 = gep %30, i64 177
 41460|  %347 = gep %28, i64 85
 41461|  %348 = gep %28, i64 88
 41462|  %349 = gep %28, i64 96
 41463|  %350 = gep %28, i64 104
 41464|  %351 = gep %28, i64 112
 41465|  %352 = gep %28, i64 120
 41466|  %353 = gep %28, i64 128
 41467|  %354 = gep %28, i64 136
 41468|  %355 = gep %28, i64 149
 41469|  %356 = gep %28, i64 177
 41470|  br label %357                                                                                                         ;L324<204
 41471| 
 41472| 357: ; preds = %479, %316
 41473|  %358 = phi ptr [ inttoptr (i64 8 to ptr), %316 ], [ %480, %479 ]
 41474|  %359 = phi ptr [ inttoptr (i64 8 to ptr), %316 ], [ %481, %479 ]
 41475|  %360 = phi ptr [ inttoptr (i64 8 to ptr), %316 ], [ %482, %479 ]
 41476|  %361 = phi i64 [ 0, %316 ], [ %483, %479 ]
 41477|  %362 = phi ptr [ %318, %316 ], [ %366, %479 ]                                                                         ;L324<204
 41478|  %363 = phi i1 [ false, %316 ], [ %484, %479 ]                                                                         ;L312<204
 41480|     ;; iter[0..+8] = ptr %362
 41481|     ;; self = ptr undef
 41482|     ;; ptr = ptr %362
 41483|     ;; self = ptr %362
 41484|     ;; end_or_len = ptr %321
 41487|  %364 = icmp eq ptr %362, %321                                                                                         ;L1714<180<324<204
 41488|  br i1 %364, label %570, label %365                                                                                    ;L180<324<204
 41489| 
 41490| 365: ; preds = %357
 41491|  %366 = gep %362, i64 8                                                                                                ;L656<185<324<204
 41492|     ;; iter[0..+8] = ptr %366
 41493|  %367 = load ptr, ptr %362, , !!48170, !!8, !!8                                                                        ;L324<204
 41494|     ;; enemy = ptr %367
 41495|     ;; other = ptr %367
 41497|  %368 = gep %367, i64 1472                                                                                             ;L325<204
 41498|  %369 = load i64, ptr %368, , !!48170, !!8                                                                             ;L325<204
 41499|  %370 = invoke { i64, i64 } @ai::utils18range_misjudge_rng(i64 %204, ptr %5, ptr %4, i64 %369)
 41500|  to label %371 unwind label %288, !!48170                                                                              ;L325<204
 41501| 
 41502| 371: ; preds = %365
 41503|  %372 = extractvalue { i64, i64 } %370, 0                                                                              ;L325<204
 41504|  %373 = extractvalue { i64, i64 } %370, 1                                                                              ;L325<204
 41505|  store i64 %372, ptr %31, , !!48147                                                                                    ;L325<204
 41506|  store i64 %373, ptr %322, , !!48147                                                                                   ;L325<204
 41507|  %374 = invoke i64 @ai::plan_legacy3old6battle17max_range_can_use(ptr %209, ptr %367)
 41508|  to label %375 unwind label %288, !!48170                                                                              ;L326<204
 41509| 
 41510| 375: ; preds = %371
 41511|  %376 = invoke i64 @ai::utils19range_misjudge_roll(ptr %3, ptr %31, i64 %282, i64 %284)
 41512|  to label %377 unwind label %288, !!48170                                                                              ;L326<204
 41513| 
 41514| 377: ; preds = %375
 41515|  %378 = mul i64 %376, %374                                                                                             ;L326<204
 41516|  %379 = udiv i64 %378, 1000                                                                                            ;L326<204
 41517|     ;; mr = i64 %379
 41518|  %380 = invoke i64 @ai::plan_legacy3old6battle17max_range_can_use(ptr %367, ptr %209)
 41519|  to label %381 unwind label %288, !!48170                                                                              ;L327<204
 41520| 
 41521| 381: ; preds = %377
 41522|  %382 = invoke i64 @ai::utils19range_misjudge_roll(ptr %3, ptr %31, i64 %282, i64 %284)
 41523|  to label %383 unwind label %288, !!48170                                                                              ;L327<204
 41524| 
 41525| 383: ; preds = %381
 41526|  %384 = mul i64 %382, %380                                                                                             ;L327<204
 41527|  %385 = udiv i64 %384, 1000                                                                                            ;L327<204
 41528|     ;; emr = i64 %385
 41529|  %386 = invoke i64 @ai::plan_legacy3old6battle24max_range_nearly_can_use(ptr %367, ptr %209, i64 40)
 41530|  to label %387 unwind label %288, !!48170                                                                              ;L328<204
 41531| 
 41532| 387: ; preds = %383
 41533|  %388 = invoke i64 @ai::utils19range_misjudge_roll(ptr %3, ptr %31, i64 %282, i64 %284)
 41534|  to label %389 unwind label %288, !!48170                                                                              ;L328<204
 41535| 
 41536| 389: ; preds = %387
 41537|  %390 = mul i64 %388, %386                                                                                             ;L328<204
 41538|  %391 = udiv i64 %390, 1000                                                                                            ;L328<204
 41539|     ;; emr_near = i64 %391
 41540|  %392 = gep %367, i64 1632                                                                                             ;L2158<329<204
 41541|  %393 = load i64, ptr %392, , !!48170, !!8                                                                             ;L2158<329<204
 41542|     ;; x2 = i64 %393
 41543|     ;; other = i64 %393
 41544|  %394 = gep %367, i64 1640                                                                                             ;L2158<329<204
 41545|  %395 = load i64, ptr %394, , !!48170, !!8                                                                             ;L2158<329<204
 41546|     ;; y2 = i64 %395
 41547|     ;; other = i64 %395
 41548|  %396 = icmp ult i64 %267, %393                                                                                        ;L3147<7<2158<329<204
 41549|  %397 = sub nuw i64 %393, %267                                                                                         ;L3147<7<2158<329<204
 41550|  %398 = sub nuw i64 %267, %393                                                                                         ;L3147<7<2158<329<204
 41551|  %399 = select i1 %396, i64 %397, i64 %398                                                                             ;L3147<7<2158<329<204
 41552|     ;; dx = i64 %399
 41553|  %400 = icmp ult i64 %269, %395                                                                                        ;L3147<8<2158<329<204
 41554|  %401 = sub nuw i64 %395, %269                                                                                         ;L3147<8<2158<329<204
 41555|  %402 = sub nuw i64 %269, %395                                                                                         ;L3147<8<2158<329<204
 41556|  %403 = select i1 %400, i64 %401, i64 %402                                                                             ;L3147<8<2158<329<204
 41557|     ;; dy = i64 %403
 41558|  %404 = mul i64 %399, %399                                                                                             ;L9<2158<329<204
 41559|  %405 = mul i64 %403, %403                                                                                             ;L9<2158<329<204
 41560|  %406 = add i64 %405, %404                                                                                             ;L9<2158<329<204
 41561|     ;; dist = i64 %406
 41562|     ;; self = ptr %317
 41563|     ;; f = ptr %367
 41564|  br i1 %323, label %422, label %407                                                                                    ;L708<332<204
 41565| 
 41566| 407: ; preds = %389
 41567|     ;; x = ptr %317
 41568|  %408 = load i64, ptr %324, , !!48170, !!8                                                                             ;L710<332<204
 41569|  %409 = load i64, ptr %325, , !!48170, !!8                                                                             ;L710<332<204
 41574|     ;; x1 = i64 %393
 41575|     ;; self = i64 %393
 41576|     ;; y1 = i64 %395
 41577|     ;; self = i64 %395
 41578|     ;; x2 = i64 %408
 41579|     ;; other = i64 %408
 41580|     ;; y2 = i64 %409
 41581|     ;; other = i64 %409
 41582|  %410 = icmp ult i64 %393, %408                                                                                        ;L3147<7<2158<332<710<332<204
 41583|  %411 = sub nuw i64 %408, %393                                                                                         ;L3147<7<2158<332<710<332<204
 41584|  %412 = sub nuw i64 %393, %408                                                                                         ;L3147<7<2158<332<710<332<204
 41585|  %413 = select i1 %410, i64 %411, i64 %412                                                                             ;L3147<7<2158<332<710<332<204
 41586|     ;; dx = i64 %413
 41587|  %414 = icmp ult i64 %395, %409                                                                                        ;L3147<8<2158<332<710<332<204
 41588|  %415 = sub nuw i64 %409, %395                                                                                         ;L3147<8<2158<332<710<332<204
 41589|  %416 = sub nuw i64 %395, %409                                                                                         ;L3147<8<2158<332<710<332<204
 41590|  %417 = select i1 %414, i64 %415, i64 %416                                                                             ;L3147<8<2158<332<710<332<204
 41591|     ;; dy = i64 %417
 41592|  %418 = mul i64 %413, %413                                                                                             ;L9<2158<332<710<332<204
 41593|  %419 = mul i64 %417, %417                                                                                             ;L9<2158<332<710<332<204
 41594|  %420 = add i64 %419, %418                                                                                             ;L9<2158<332<710<332<204
 41595|  %421 = icmp ult i64 %420, 40000000001                                                                                 ;L332<710<332<204
 41596|  br label %422                                                                                                         ;L332<710<332<204
 41597| 
 41598| 422: ; preds = %407, %389
 41599|  %423 = phi i1 [ true, %389 ], [ %421, %407 ]                                                                          ;L0<332<204
 41601|  %424 = load ptr, ptr %326, , !!48170, !!8, !!8                                                                        ;L335<204
 41602|  %425 = gep %424, i64 4856                                                                                             ;L335<204
 41603|  %426 = load i64, ptr %425, , !!48170, !!8                                                                             ;L335<204
 41604|  %427 = icmp ult i64 %293, %426                                                                                        ;L335<204
 41605|  br i1 %427, label %428, label %430                                                                                    ;L335<204
 41606| 
 41607| 428: ; preds = %422
 41608|  %429 = invoke i64 @ai::plan_legacy3old6battle24max_range_nearly_can_use(ptr %367, ptr %209, i64 60)
 41609|  to label %432 unwind label %288, !!48170                                                                              ;L350<204
 41610| 
 41611| 430: ; preds = %422
 41612|  %431 = invoke i64 @gc::simulation6entityNtB5_6Entity18remain_action_time(ptr %367)
 41613|  to label %485 unwind label %288, !!48170                                                                              ;L336<204
 41614| 
 41615| 432: ; preds = %428
 41616|  %433 = invoke i64 @ai::utils19range_misjudge_roll(ptr %3, ptr %31, i64 %282, i64 %284)
 41617|  to label %434 unwind label %288, !!48170                                                                              ;L350<204
 41618| 
 41619| 434: ; preds = %432
 41620|  %435 = mul i64 %433, %429                                                                                             ;L350<204
 41621|  %436 = udiv i64 %435, 1000                                                                                            ;L350<204
 41622|     ;; emr = i64 %436
 41623|  %437 = mul i64 %379, %379                                                                                             ;L352<204
 41624|  %438 = icmp ugt i64 %406, %437                                                                                        ;L352<204
 41625|  %439 = icmp samesign ult i64 %436, %379                                                                               ;L352<204
 41626|  %440 = and i1 %438, %439                                                                                              ;L352<204
 41627|  br i1 %440, label %445, label %441                                                                                    ;L352<204
 41628| 
 41629| 441: ; preds = %434
 41630|  %442 = mul i64 %436, %436                                                                                             ;L357<204
 41631|  %443 = icmp ule i64 %406, %442                                                                                        ;L357<204
 41632|  %444 = select i1 %443, i1 true, i1 %363                                                                               ;L357<204
 41633|  br label %479                                                                                                         ;L357<204
 41634| 
 41635| 445: ; preds = %434
 41636|  br i1 %423, label %446, label %479                                                                                    ;L354<204
 41637| 
 41638| 446: ; preds = %445
 41640|     ;; data = ptr %5
 41641|     ;; target = i64 %369
 41642|     ;; end_delay = i64 5
 41644|     ;; default = i64 0
 41646|     ;; default = i64 0
 41647|  %447 = load ptr, ptr %89, , !!48699, !!8, !!8                                                                         ;L43<355<204
 41648|  %448 = load ptr, ptr %174, , !!48699, !!8, !!8                                                                        ;L43<355<204
 41649|  %449 = gep %448, i64 496                                                                                              ;L43<355<204
 41650|  %450 = load ptr, ptr %449, , !!48699, !!8                                                                             ;L43<355<204
 41651|  %451 = invoke ptr %450(ptr %447, i64 %369)
 41652|  to label %452 unwind label %288, !!48170                                                                              ;L43<355<204
 41653| 
 41654| 452: ; preds = %446
 41655|     ;; target_entity = ptr %451
 41656|     ;; self = ptr %451
 41657|  %453 = gep %448, i64 40                                                                                               ;L45<355<204
 41658|  %454 = load ptr, ptr %453, , !!48699, !!8                                                                             ;L45<355<204
 41659|  %455 = invoke i64 %454(ptr %447)
 41660|  to label %456 unwind label %288, !!48170                                                                              ;L45<355<204
 41661| 
 41662| 456: ; preds = %452
 41663|  %457 = icmp eq ptr %451, null                                                                                         ;L1161<47<355<204
 41664|  br i1 %457, label %463, label %458                                                                                    ;L1161<47<355<204
 41665| 
 41666| 458: ; preds = %456
 41667|     ;; x = ptr %451
 41668|     ;; t = ptr %451
 41669|  %459 = gep %451, i64 1632                                                                                             ;L47<1162<47<355<204
 41670|  %460 = load i64, ptr %459, , !!48699, !!8                                                                             ;L47<1162<47<355<204
 41671|     ;; self[8..+8] = i64 %460
 41672|     ;; self[0..+8] = i64 1
 41673|     ;; self = ptr %451
 41674|     ;; x = ptr %451
 41675|     ;; t = ptr %451
 41676|  %461 = gep %451, i64 1640                                                                                             ;L48<1162<48<355<204
 41677|  %462 = load i64, ptr %461, , !!48699, !!8                                                                             ;L48<1162<48<355<204
 41678|     ;; self[8..+8] = i64 %462
 41679|     ;; self[0..+8] = i64 1
 41680|  br label %463                                                                                                         ;L1043<48<355<204
 41681| 
 41682| 463: ; preds = %458, %456
 41683|  %464 = phi i64 [ %462, %458 ], [ 0, %456 ]                                                                            ;L0<48<355<204
 41684|  %465 = phi i64 [ %460, %458 ], [ 0, %456 ]                                                                            ;L0<47<355<204
 41685|  store i64 0, ptr %28, , !!48147                                                                                       ;L355<204
 41686|  store i8 2, ptr %347, , !!48147                                                                                       ;L355<204
 41687|  store i64 %455, ptr %348, , !!48147                                                                                   ;L355<204
 41688|  store i64 %369, ptr %349, , !!48147                                                                                   ;L355<204
 41689|  store i64 %465, ptr %350, , !!48147                                                                                   ;L355<204
 41690|  store i64 %464, ptr %351, , !!48147                                                                                   ;L355<204
 41691|  store i64 15000, ptr %352, , !!48147                                                                                  ;L355<204
 41692|  store i64 5, ptr %353, , !!48147                                                                                      ;L355<204
 41693|  call void @llvm.memset.p0.i64(ptr %354, i8 0, i64 13, i1 false), !!48147                                              ;L355<204
 41694|  store i8 2, ptr %355, , !!48147                                                                                       ;L355<204
 41695|  store i8 14, ptr %356, , !!48147                                                                                      ;L355<204
 41697|     ;; self = ptr %39
 41698|     ;; self = ptr %39
 41699|     ;; value = ptr %28
 41700|     ;; src = ptr %28
 41701|     ;; additional = i64 1
 41702|     ;; needed_extra_cap = i64 1
 41703|     ;; needed_extra_cap = i64 1
 41704|     ;; strategy = i8 1
 41705|     ;; self = ptr %39
 41706|  %466 = load i64, ptr %207, , !!48734, !!8                                                                             ;L149<1428<355<204
 41707|  %467 = icmp eq i64 %361, %466                                                                                         ;L1428<355<204
 41708|  br i1 %467, label %468, label %474                                                                                    ;L1428<355<204
 41709| 
 41710| 468: ; preds = %463
 41711|     ;; self = ptr %39
 41712|     ;; self = ptr %39
 41713|     ;; self = ptr %39
 41714|     ;; used_cap = i64 %361
 41715|     ;; used_cap = i64 %361
 41716|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %39, i64 %361, i64 1, i1 zeroext true)
 41717|  to label %469 unwind label %472, !!48740                                                                              ;L619<430<738<1429<355<204
 41718| 
 41719| 469: ; preds = %468
 41720|  %470 = load i64, ptr %208, , !!48734                                                                                  ;L1432<355<204
 41721|  %471 = load ptr, ptr %39, , !!48734                                                                                   ;L138<1432<355<204
 41722|  br label %474                                                                                                         ;L619<430<738<1429<355<204
 41723| 
 41724| 472: ; preds = %468
 41725|  %473 = cleanuppad within none []
 41726|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %28) #30 [ "funclet"(token %473) ], !!48745 ;L1436<355<204
 41727|  cleanupret from %473 unwind label %288
 41728| 
 41729| 474: ; preds = %469, %463
 41730|  %475 = phi ptr [ %471, %469 ], [ %358, %463 ]                                                                         ;L138<1432<355<204
 41731|  %476 = phi i64 [ %470, %469 ], [ %361, %463 ]                                                                         ;L1432<355<204
 41732|     ;; self = ptr %39
 41733|     ;; self = ptr %475
 41734|     ;; count = i64 %476
 41735|  %477 = gepS %475, i64 %476                                                                                            ;L961<1432<355<204
 41736|     ;; end = ptr %477
 41737|     ;; dst = ptr %477
 41738|  call void @llvm.memcpy.p0.p0.i64(ptr %477, ptr %28, i64 184, i1 false), !!48745                                       ;L1933<1433<355<204
 41739|  %478 = add i64 %476, 1                                                                                                ;L1434<355<204
 41740|  store i64 %478, ptr %208, , !!48734                                                                                   ;L1434<355<204
 41742|  br label %479                                                                                                         ;L354<204
 41743| 
 41744| 479: ; preds = %564, %529, %500, %496, %494, %474, %445, %441
 41745|  %480 = phi ptr [ %565, %564 ], [ %358, %494 ], [ %530, %529 ], [ %358, %500 ], [ %358, %441 ], [ %358, %496 ], [ %475, %474 ], [ %358, %445 ]
 41746|  %481 = phi ptr [ %566, %564 ], [ %359, %494 ], [ %531, %529 ], [ %359, %500 ], [ %359, %441 ], [ %359, %496 ], [ %475, %474 ], [ %359, %445 ]
 41747|  %482 = phi ptr [ %566, %564 ], [ %360, %494 ], [ %532, %529 ], [ %360, %500 ], [ %360, %441 ], [ %360, %496 ], [ %475, %474 ], [ %360, %445 ]
 41748|  %483 = phi i64 [ %569, %564 ], [ %361, %494 ], [ %535, %529 ], [ %361, %500 ], [ %361, %441 ], [ %361, %496 ], [ %478, %474 ], [ %361, %445 ]
 41749|  %484 = phi i1 [ %363, %564 ], [ %363, %494 ], [ %363, %529 ], [ %363, %500 ], [ %444, %441 ], [ %499, %496 ], [ %363, %474 ], [ %363, %445 ] ;L0<204
 41752|  br label %357                                                                                                         ;L324<204
 41753| 
 41754| 485: ; preds = %430
 41755|  %486 = icmp ugt i64 %431, 10                                                                                          ;L336<204
 41756|  %487 = icmp ugt i64 %378, 999                                                                                         ;L336<204
 41757|  %488 = and i1 %487, %486                                                                                              ;L336<204
 41758|  %489 = mul i64 %379, %379                                                                                             ;L0<204
 41759|  %490 = icmp ugt i64 %406, %489                                                                                        ;L0<204
 41760|  br i1 %488, label %494, label %491                                                                                    ;L336<204
 41761| 
 41762| 491: ; preds = %485
 41763|  %492 = icmp samesign ult i64 %385, %379                                                                               ;L340<204
 41764|  %493 = and i1 %492, %490                                                                                              ;L340<204
 41765|  br i1 %493, label %500, label %496                                                                                    ;L340<204
 41766| 
 41767| 494: ; preds = %485
 41768|  %495 = and i1 %490, %423                                                                                              ;L337<204
 41769|  br i1 %495, label %536, label %479                                                                                    ;L337<204
 41770| 
 41771| 496: ; preds = %491
 41772|  %497 = mul i64 %391, %391                                                                                             ;L345<204
 41773|  %498 = icmp ule i64 %406, %497                                                                                        ;L345<204
 41774|  %499 = select i1 %498, i1 true, i1 %363                                                                               ;L345<204
 41775|  br label %479                                                                                                         ;L345<204
 41776| 
 41777| 500: ; preds = %491
 41778|  br i1 %423, label %501, label %479                                                                                    ;L342<204
 41779| 
 41780| 501: ; preds = %500
 41782|     ;; data = ptr %5
 41783|     ;; target = i64 %369
 41784|     ;; end_delay = i64 5
 41786|     ;; default = i64 0
 41788|     ;; default = i64 0
 41789|  %502 = load ptr, ptr %89, , !!48768, !!8, !!8                                                                         ;L43<343<204
 41790|  %503 = load ptr, ptr %174, , !!48768, !!8, !!8                                                                        ;L43<343<204
 41791|  %504 = gep %503, i64 496                                                                                              ;L43<343<204
 41792|  %505 = load ptr, ptr %504, , !!48768, !!8                                                                             ;L43<343<204
 41793|  %506 = invoke ptr %505(ptr %502, i64 %369)
 41794|  to label %507 unwind label %288, !!48170                                                                              ;L43<343<204
 41795| 
 41796| 507: ; preds = %501
 41797|     ;; target_entity = ptr %506
 41798|     ;; self = ptr %506
 41799|  %508 = gep %503, i64 40                                                                                               ;L45<343<204
 41800|  %509 = load ptr, ptr %508, , !!48768, !!8                                                                             ;L45<343<204
 41801|  %510 = invoke i64 %509(ptr %502)
 41802|  to label %511 unwind label %288, !!48170                                                                              ;L45<343<204
 41803| 
 41804| 511: ; preds = %507
 41805|  %512 = icmp eq ptr %506, null                                                                                         ;L1161<47<343<204
 41806|  br i1 %512, label %518, label %513                                                                                    ;L1161<47<343<204
 41807| 
 41808| 513: ; preds = %511
 41809|     ;; x = ptr %506
 41810|     ;; t = ptr %506
 41811|  %514 = gep %506, i64 1632                                                                                             ;L47<1162<47<343<204
 41812|  %515 = load i64, ptr %514, , !!48768, !!8                                                                             ;L47<1162<47<343<204
 41813|     ;; self[8..+8] = i64 %515
 41814|     ;; self[0..+8] = i64 1
 41815|     ;; self = ptr %506
 41816|     ;; x = ptr %506
 41817|     ;; t = ptr %506
 41818|  %516 = gep %506, i64 1640                                                                                             ;L48<1162<48<343<204
 41819|  %517 = load i64, ptr %516, , !!48768, !!8                                                                             ;L48<1162<48<343<204
 41820|     ;; self[8..+8] = i64 %517
 41821|     ;; self[0..+8] = i64 1
 41822|  br label %518                                                                                                         ;L1043<48<343<204
 41823| 
 41824| 518: ; preds = %513, %511
 41825|  %519 = phi i64 [ %517, %513 ], [ 0, %511 ]                                                                            ;L0<48<343<204
 41826|  %520 = phi i64 [ %515, %513 ], [ 0, %511 ]                                                                            ;L0<47<343<204
 41827|  store i64 0, ptr %29, , !!48147                                                                                       ;L343<204
 41828|  store i8 2, ptr %327, , !!48147                                                                                       ;L343<204
 41829|  store i64 %510, ptr %328, , !!48147                                                                                   ;L343<204
 41830|  store i64 %369, ptr %329, , !!48147                                                                                   ;L343<204
 41831|  store i64 %520, ptr %330, , !!48147                                                                                   ;L343<204
 41832|  store i64 %519, ptr %331, , !!48147                                                                                   ;L343<204
 41833|  store i64 15000, ptr %332, , !!48147                                                                                  ;L343<204
 41834|  store i64 5, ptr %333, , !!48147                                                                                      ;L343<204
 41835|  call void @llvm.memset.p0.i64(ptr %334, i8 0, i64 13, i1 false), !!48147                                              ;L343<204
 41836|  store i8 2, ptr %335, , !!48147                                                                                       ;L343<204
 41837|  store i8 14, ptr %336, , !!48147                                                                                      ;L343<204
 41839|     ;; self = ptr %39
 41840|     ;; self = ptr %39
 41841|     ;; value = ptr %29
 41842|     ;; src = ptr %29
 41843|     ;; additional = i64 1
 41844|     ;; needed_extra_cap = i64 1
 41845|     ;; needed_extra_cap = i64 1
 41846|     ;; strategy = i8 1
 41847|     ;; self = ptr %39
 41848|  %521 = load i64, ptr %207, , !!48803, !!8                                                                             ;L149<1428<343<204
 41849|  %522 = icmp eq i64 %361, %521                                                                                         ;L1428<343<204
 41850|  br i1 %522, label %523, label %529                                                                                    ;L1428<343<204
 41851| 
 41852| 523: ; preds = %518
 41853|     ;; self = ptr %39
 41854|     ;; self = ptr %39
 41855|     ;; self = ptr %39
 41856|     ;; used_cap = i64 %361
 41857|     ;; used_cap = i64 %361
 41858|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %39, i64 %361, i64 1, i1 zeroext true)
 41859|  to label %524 unwind label %527, !!48809                                                                              ;L619<430<738<1429<343<204
 41860| 
 41861| 524: ; preds = %523
 41862|  %525 = load i64, ptr %208, , !!48803                                                                                  ;L1432<343<204
 41863|  %526 = load ptr, ptr %39, , !!48803                                                                                   ;L138<1432<343<204
 41864|  br label %529                                                                                                         ;L619<430<738<1429<343<204
 41865| 
 41866| 527: ; preds = %523
 41867|  %528 = cleanuppad within none []
 41868|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %29) #30 [ "funclet"(token %528) ], !!48814 ;L1436<343<204
 41869|  cleanupret from %528 unwind label %288
 41870| 
 41871| 529: ; preds = %524, %518
 41872|  %530 = phi ptr [ %526, %524 ], [ %358, %518 ]
 41873|  %531 = phi ptr [ %526, %524 ], [ %359, %518 ]
 41874|  %532 = phi ptr [ %526, %524 ], [ %360, %518 ]                                                                         ;L138<1432<343<204
 41875|  %533 = phi i64 [ %525, %524 ], [ %361, %518 ]                                                                         ;L1432<343<204
 41876|     ;; self = ptr %39
 41877|     ;; self = ptr %532
 41878|     ;; count = i64 %533
 41879|  %534 = gepS %532, i64 %533                                                                                            ;L961<1432<343<204
 41880|     ;; end = ptr %534
 41881|     ;; dst = ptr %534
 41882|  call void @llvm.memcpy.p0.p0.i64(ptr %534, ptr %29, i64 184, i1 false), !!48814                                       ;L1933<1433<343<204
 41883|  %535 = add i64 %533, 1                                                                                                ;L1434<343<204
 41884|  store i64 %535, ptr %208, , !!48803                                                                                   ;L1434<343<204
 41886|  br label %479                                                                                                         ;L342<204
 41887| 
 41888| 536: ; preds = %494
 41890|     ;; data = ptr %5
 41891|     ;; target = i64 %369
 41892|     ;; end_delay = i64 5
 41894|     ;; default = i64 0
 41896|     ;; default = i64 0
 41897|  %537 = load ptr, ptr %89, , !!48832, !!8, !!8                                                                         ;L43<338<204
 41898|  %538 = load ptr, ptr %174, , !!48832, !!8, !!8                                                                        ;L43<338<204
 41899|  %539 = gep %538, i64 496                                                                                              ;L43<338<204
 41900|  %540 = load ptr, ptr %539, , !!48832, !!8                                                                             ;L43<338<204
 41901|  %541 = invoke ptr %540(ptr %537, i64 %369)
 41902|  to label %542 unwind label %288, !!48170                                                                              ;L43<338<204
 41903| 
 41904| 542: ; preds = %536
 41905|     ;; target_entity = ptr %541
 41906|     ;; self = ptr %541
 41907|  %543 = gep %538, i64 40                                                                                               ;L45<338<204
 41908|  %544 = load ptr, ptr %543, , !!48832, !!8                                                                             ;L45<338<204
 41909|  %545 = invoke i64 %544(ptr %537)
 41910|  to label %546 unwind label %288, !!48170                                                                              ;L45<338<204
 41911| 
 41912| 546: ; preds = %542
 41913|  %547 = icmp eq ptr %541, null                                                                                         ;L1161<47<338<204
 41914|  br i1 %547, label %553, label %548                                                                                    ;L1161<47<338<204
 41915| 
 41916| 548: ; preds = %546
 41917|     ;; x = ptr %541
 41918|     ;; t = ptr %541
 41919|  %549 = gep %541, i64 1632                                                                                             ;L47<1162<47<338<204
 41920|  %550 = load i64, ptr %549, , !!48832, !!8                                                                             ;L47<1162<47<338<204
 41921|     ;; self[8..+8] = i64 %550
 41922|     ;; self[0..+8] = i64 1
 41923|     ;; self = ptr %541
 41924|     ;; x = ptr %541
 41925|     ;; t = ptr %541
 41926|  %551 = gep %541, i64 1640                                                                                             ;L48<1162<48<338<204
 41927|  %552 = load i64, ptr %551, , !!48832, !!8                                                                             ;L48<1162<48<338<204
 41928|     ;; self[8..+8] = i64 %552
 41929|     ;; self[0..+8] = i64 1
 41930|  br label %553                                                                                                         ;L1043<48<338<204
 41931| 
 41932| 553: ; preds = %548, %546
 41933|  %554 = phi i64 [ %552, %548 ], [ 0, %546 ]                                                                            ;L0<48<338<204
 41934|  %555 = phi i64 [ %550, %548 ], [ 0, %546 ]                                                                            ;L0<47<338<204
 41935|  store i64 0, ptr %30, , !!48147                                                                                       ;L338<204
 41936|  store i8 2, ptr %337, , !!48147                                                                                       ;L338<204
 41937|  store i64 %545, ptr %338, , !!48147                                                                                   ;L338<204
 41938|  store i64 %369, ptr %339, , !!48147                                                                                   ;L338<204
 41939|  store i64 %555, ptr %340, , !!48147                                                                                   ;L338<204
 41940|  store i64 %554, ptr %341, , !!48147                                                                                   ;L338<204
 41941|  store i64 15000, ptr %342, , !!48147                                                                                  ;L338<204
 41942|  store i64 5, ptr %343, , !!48147                                                                                      ;L338<204
 41943|  call void @llvm.memset.p0.i64(ptr %344, i8 0, i64 13, i1 false), !!48147                                              ;L338<204
 41944|  store i8 2, ptr %345, , !!48147                                                                                       ;L338<204
 41945|  store i8 14, ptr %346, , !!48147                                                                                      ;L338<204
 41947|     ;; self = ptr %39
 41948|     ;; self = ptr %39
 41949|     ;; value = ptr %30
 41950|     ;; src = ptr %30
 41951|     ;; additional = i64 1
 41952|     ;; needed_extra_cap = i64 1
 41953|     ;; needed_extra_cap = i64 1
 41954|     ;; strategy = i8 1
 41955|     ;; self = ptr %39
 41956|  %556 = load i64, ptr %207, , !!48867, !!8                                                                             ;L149<1428<338<204
 41957|  %557 = icmp eq i64 %361, %556                                                                                         ;L1428<338<204
 41958|  br i1 %557, label %558, label %564                                                                                    ;L1428<338<204
 41959| 
 41960| 558: ; preds = %553
 41961|     ;; self = ptr %39
 41962|     ;; self = ptr %39
 41963|     ;; self = ptr %39
 41964|     ;; used_cap = i64 %361
 41965|     ;; used_cap = i64 %361
 41966|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %39, i64 %361, i64 1, i1 zeroext true)
 41967|  to label %559 unwind label %562, !!48873                                                                              ;L619<430<738<1429<338<204
 41968| 
 41969| 559: ; preds = %558
 41970|  %560 = load i64, ptr %208, , !!48867                                                                                  ;L1432<338<204
 41971|  %561 = load ptr, ptr %39, , !!48867                                                                                   ;L138<1432<338<204
 41972|  br label %564                                                                                                         ;L619<430<738<1429<338<204
 41973| 
 41974| 562: ; preds = %558
 41975|  %563 = cleanuppad within none []
 41976|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %30) #30 [ "funclet"(token %563) ], !!48878 ;L1436<338<204
 41977|  cleanupret from %563 unwind label %288
 41978| 
 41979| 564: ; preds = %559, %553
 41980|  %565 = phi ptr [ %561, %559 ], [ %358, %553 ]
 41981|  %566 = phi ptr [ %561, %559 ], [ %359, %553 ]                                                                         ;L138<1432<338<204
 41982|  %567 = phi i64 [ %560, %559 ], [ %361, %553 ]                                                                         ;L1432<338<204
 41983|     ;; self = ptr %39
 41984|     ;; self = ptr %566
 41985|     ;; count = i64 %567
 41986|  %568 = gepS %566, i64 %567                                                                                            ;L961<1432<338<204
 41987|     ;; end = ptr %568
 41988|     ;; dst = ptr %568
 41989|  call void @llvm.memcpy.p0.p0.i64(ptr %568, ptr %30, i64 184, i1 false), !!48878                                       ;L1933<1433<338<204
 41990|  %569 = add i64 %567, 1                                                                                                ;L1434<338<204
 41991|  store i64 %569, ptr %208, , !!48867                                                                                   ;L1434<338<204
 41993|  br label %479                                                                                                         ;L337<204
 41994| 
 41995| 570: ; preds = %357
 41996|  %571 = gep %89, i64 240                                                                                               ;L364<204
 41997|  %572 = getelementptr { { ptr, ptr, i64 }, i64 }, ptr %571, i64 %96                                                    ;L364<204
 41998|     ;; self = ptr %572
 41999|     ;; self = ptr %572
 42000|  %573 = load ptr, ptr %572, , !!48170, !!8, !!8                                                                        ;L138<2073<364<204
 42001|     ;; p = ptr %573
 42002|  %574 = gep %572, i64 24                                                                                               ;L2075<364<204
 42003|  %575 = load i64, ptr %574, , !!48170, !!8                                                                             ;L2075<364<204
 42004|     ;; len = i64 %575
 42005|     ;; count = i64 %575
 42006|     ;; count = i64 %575
 42007|     ;; self[0..+8] = ptr %573
 42008|     ;; slice[0..+8] = ptr %573
 42009|     ;; self[0..+8] = ptr %573
 42010|     ;; slice[0..+8] = ptr %573
 42011|     ;; self[8..+8] = i64 %575
 42012|     ;; slice[8..+8] = i64 %575
 42013|     ;; self[8..+8] = i64 %575
 42014|     ;; slice[8..+8] = i64 %575
 42015|     ;; ptr = ptr %573
 42016|     ;; self = ptr %573
 42017|  %576 = getelementptr ptr, ptr %573, i64 %575                                                                          ;L961<100<1042<364<204
 42018|     ;; iter[0..+8] = ptr %573
 42019|     ;; iter[8..+8] = ptr %576
 42020|  %577 = gep %209, i64 1136
 42021|  %578 = gep %209, i64 1664
 42022|  br label %579                                                                                                         ;L364<204
 42023| 
 42024| 579: ; preds = %599, %570
 42025|  %580 = phi ptr [ %573, %570 ], [ %583, %599 ]                                                                         ;L364<204
 42026|     ;; iter[0..+8] = ptr %580
 42027|     ;; self = ptr undef
 42028|     ;; ptr = ptr %580
 42029|     ;; self = ptr %580
 42030|     ;; end_or_len = ptr %576
 42033|  %581 = icmp eq ptr %580, %576                                                                                         ;L1714<180<364<204
 42034|  br i1 %581, label %652, label %582                                                                                    ;L180<364<204
 42035| 
 42036| 582: ; preds = %579
 42037|  %583 = gep %580, i64 8                                                                                                ;L656<185<364<204
 42038|     ;; iter[0..+8] = ptr %583
 42039|  %584 = load ptr, ptr %580, , !!48170, !!8, !!8                                                                        ;L364<204
 42040|     ;; e = ptr %584
 42041|     ;; caster = ptr %584
 42042|     ;; self = ptr %584
 42043|     ;; other = ptr %584
 42044|     ;; self = ptr %584
 42045|  %585 = gep %584, i64 1216                                                                                             ;L742<365<204
 42046|  %586 = load i32, ptr %585, , !!48170, !!8                                                                             ;L742<365<204
 42047|  %587 = icmp eq i32 %586, -1                                                                                           ;L742<365<204
 42048|  br i1 %587, label %599, label %588                                                                                    ;L742<365<204
 42049| 
 42050| 588: ; preds = %582
 42051|  %589 = gep %584, i64 1168                                                                                             ;L742<365<204
 42052|     ;; atk = ptr %589
 42053|     ;; self = ptr %589
 42054|  %590 = gep %584, i64 1184                                                                                             ;L26<366<204
 42055|  %591 = load i64, ptr %590, , !!48170, !!8                                                                             ;L26<366<204
 42056|  %592 = gep %584, i64 1192                                                                                             ;L26<366<204
 42057|  %593 = load i64, ptr %592, , !!48170, !!8                                                                             ;L26<366<204
 42058|  %594 = gep %584, i64 1480                                                                                             ;L26<366<204
 42059|  %595 = load i64, ptr %594, , !!48170, !!8                                                                             ;L26<366<204
 42060|  %596 = gep %584, i64 1080                                                                                             ;L26<366<204
 42061|  %597 = load i64, ptr %596, , !!48170, !!8                                                                             ;L26<366<204
 42062|  %598 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %589, ptr %584, ptr %209)
 42063|  to label %600 unwind label %288, !!48170                                                                              ;L366<204
 42064| 
 42065| 599: ; preds = %628, %582
 42066|  br label %579                                                                                                         ;L364<204
 42067| 
 42068| 600: ; preds = %588
 42069|  %601 = add i64 %595, -1                                                                                               ;L26<366<204
 42070|  %602 = mul i64 %601, %593                                                                                             ;L26<366<204
 42071|  %603 = gep %584, i64 1136                                                                                             ;L1511<366<204
 42072|  %604 = load i32, ptr %603, , !!48170, !!8                                                                             ;L1511<366<204
 42073|     ;; mult = i32 %604
 42074|  %605 = icmp eq i32 %604, 0                                                                                            ;L1512<366<204
 42075|  br i1 %605, label %606, label %609                                                                                    ;L1512<366<204
 42076| 
 42077| 606: ; preds = %600
 42078|  %607 = gep %584, i64 1664                                                                                             ;L1513<366<204
 42079|  %608 = load i64, ptr %607, , !!48170, !!8                                                                             ;L1513<366<204
 42080|  br label %616                                                                                                         ;L1512<366<204
 42081| 
 42082| 609: ; preds = %600
 42083|  %610 = sext i32 %604 to i64                                                                                           ;L1511<366<204
 42084|     ;; mult = i64 %610
 42085|  %611 = gep %584, i64 1664                                                                                             ;L1515<366<204
 42086|  %612 = load i64, ptr %611, , !!48170, !!8                                                                             ;L1515<366<204
 42087|  %613 = add nsw i64 %610, 100                                                                                          ;L1515<366<204
 42088|  %614 = mul i64 %612, %613                                                                                             ;L1515<366<204
 42089|  %615 = udiv i64 %614, 100                                                                                             ;L1515<366<204
 42090|  br label %616                                                                                                         ;L1512<366<204
 42091| 
 42092| 616: ; preds = %609, %606
 42093|  %617 = phi i64 [ %608, %606 ], [ %615, %609 ]                                                                         ;L0<366<204
 42094|  %618 = load i32, ptr %577, , !!48170, !!8                                                                             ;L1511<366<204
 42095|     ;; mult = i32 %618
 42096|  %619 = icmp eq i32 %618, 0                                                                                            ;L1512<366<204
 42097|  br i1 %619, label %620, label %622                                                                                    ;L1512<366<204
 42098| 
 42099| 620: ; preds = %616
 42100|  %621 = load i64, ptr %578, , !!48170, !!8                                                                             ;L1513<366<204
 42101|  br label %628                                                                                                         ;L1512<366<204
 42102| 
 42103| 622: ; preds = %616
 42104|  %623 = sext i32 %618 to i64                                                                                           ;L1511<366<204
 42105|     ;; mult = i64 %623
 42106|  %624 = load i64, ptr %578, , !!48170, !!8                                                                             ;L1515<366<204
 42107|  %625 = add nsw i64 %623, 100                                                                                          ;L1515<366<204
 42108|  %626 = mul i64 %624, %625                                                                                             ;L1515<366<204
 42109|  %627 = udiv i64 %626, 100                                                                                             ;L1515<366<204
 42110|  br label %628                                                                                                         ;L1512<366<204
 42111| 
 42112| 628: ; preds = %622, %620
 42113|  %629 = phi i64 [ %621, %620 ], [ %627, %622 ]                                                                         ;L0<366<204
 42114|  %630 = add i64 %597, %591                                                                                             ;L26<366<204
 42115|  %631 = add i64 %630, %602                                                                                             ;L26<366<204
 42116|  %632 = add i64 %631, %598                                                                                             ;L366<204
 42117|  %633 = add i64 %632, %617                                                                                             ;L366<204
 42118|  %634 = add i64 %633, %629                                                                                             ;L366<204
 42119|     ;; range = i64 %634
 42120|  %635 = gep %584, i64 1632                                                                                             ;L2158<367<204
 42121|  %636 = load i64, ptr %635, , !!48170, !!8                                                                             ;L2158<367<204
 42122|     ;; x2 = i64 %636
 42123|     ;; other = i64 %636
 42124|  %637 = gep %584, i64 1640                                                                                             ;L2158<367<204
 42125|  %638 = load i64, ptr %637, , !!48170, !!8                                                                             ;L2158<367<204
 42126|     ;; y2 = i64 %638
 42127|     ;; other = i64 %638
 42128|  %639 = icmp ult i64 %267, %636                                                                                        ;L3147<7<2158<367<204
 42129|  %640 = sub nuw i64 %636, %267                                                                                         ;L3147<7<2158<367<204
 42130|  %641 = sub nuw i64 %267, %636                                                                                         ;L3147<7<2158<367<204
 42131|  %642 = select i1 %639, i64 %640, i64 %641                                                                             ;L3147<7<2158<367<204
 42132|     ;; dx = i64 %642
 42133|  %643 = icmp ult i64 %269, %638                                                                                        ;L3147<8<2158<367<204
 42134|  %644 = sub nuw i64 %638, %269                                                                                         ;L3147<8<2158<367<204
 42135|  %645 = sub nuw i64 %269, %638                                                                                         ;L3147<8<2158<367<204
 42136|  %646 = select i1 %643, i64 %644, i64 %645                                                                             ;L3147<8<2158<367<204
 42137|     ;; dy = i64 %646
 42138|  %647 = mul i64 %642, %642                                                                                             ;L9<2158<367<204
 42139|  %648 = mul i64 %646, %646                                                                                             ;L9<2158<367<204
 42140|  %649 = add i64 %648, %647                                                                                             ;L9<2158<367<204
 42141|  %650 = mul i64 %634, %634                                                                                             ;L367<204
 42142|  %651 = icmp ugt i64 %649, %650                                                                                        ;L367<204
 42143|  br i1 %651, label %599, label %652                                                                                    ;L367<204
 42144| 
 42145| 652: ; preds = %628, %579
 42146|  %653 = phi i1 [ %363, %579 ], [ true, %628 ]                                                                          ;L0<204
 42148|  %654 = icmp eq i64 %361, 0                                                                                            ;L376<204
 42149|  br i1 %654, label %656, label %655                                                                                    ;L376<204
 42150| 
 42151| 655: ; preds = %1021, %988, %836, %652
 42152|  br i1 %653, label %1062, label %1027                                                                                  ;L419<204
 42153| 
 42154| 656: ; preds = %652
 42155|  %657 = udiv i64 %269, 32000                                                                                           ;L378<204
 42156|     ;; self = i64 %657
 42157|     ;; min = i64 0
 42158|     ;; max = i64 29
 42159|  %658 = call i64 @llvm.umin.i64(i64 %657, i64 29)                                                                      ;L2027<378<204
 42160|  %659 = udiv i64 %267, 32000                                                                                           ;L378<204
 42161|     ;; self = i64 %659
 42162|     ;; min = i64 0
 42163|     ;; max = i64 29
 42164|  %660 = call i64 @llvm.umin.i64(i64 %659, i64 29)                                                                      ;L2027<378<204
 42165|  %661 = gep %182, i64 32                                                                                               ;L378<204
 42166|  %662 = load ptr, ptr %661, , !!48170, !!8, !!8                                                                        ;L378<204
 42167|  %663 = gep %662, i64 14520                                                                                            ;L378<204
 42168|  %664 = getelementptr [30 x i64], ptr %663, i64 %658                                                                   ;L378<204
 42169|  %665 = getelementptr i64, ptr %664, i64 %660                                                                          ;L378<204
 42170|  %666 = load i64, ptr %665, , !!48170, !!8                                                                             ;L378<204
 42171|     ;; region = i64 %666
 42172|  %667 = invoke { i64, ptr } %298(ptr %295)
 42173|  to label %668 unwind label %288, !!48170                                                                              ;L381<204
 42174| 
 42175| 668: ; preds = %656
 42176|  %669 = extractvalue { i64, ptr } %667, 0                                                                              ;L381<204
 42177|     ;; self[0..+8] = i64 %669
 42179|  %670 = icmp eq i64 %669, 0                                                                                            ;L231<381<204
 42180|  br i1 %670, label %671, label %1026                                                                                   ;L231<381<204
 42181| 
 42182| 671: ; preds = %668
 42183|  %672 = extractvalue { i64, ptr } %667, 1                                                                              ;L381<204
 42184|     ;; self[8..+8] = ptr %672
 42185|  %673 = icmp ne ptr %672, null
 42186|  call void @llvm.assume(i1 %673)
 42187|     ;; self = ptr %672
 42188|     ;; self = ptr %672
 42189|     ;; self = ptr %672
 42190|     ;; self = ptr %672
 42191|  %674 = gep %672, i64 424                                                                                              ;L1864<3787<381<204
 42192|  %675 = load i64, ptr %674, , !!48170, !!8                                                                             ;L1864<3787<381<204
 42195|     ;; self[8..+8] = i64 %675
 42196|     ;; slice[8..+8] = i64 %675
 42197|  %676 = icmp eq i64 %675, 0                                                                                            ;L219<576<381<204
 42198|  br i1 %676, label %684, label %677                                                                                    ;L219<576<381<204
 42199| 
 42200| 677: ; preds = %671
 42201|  %678 = gep %672, i64 416                                                                                              ;L614<609<296<1968<1864<3787<381<204
 42202|  %679 = load ptr, ptr %678, , !!48170, !!8, !!8                                                                        ;L614<609<296<1968<1864<3787<381<204
 42203|     ;; self[0..+8] = ptr %679
 42204|     ;; slice[0..+8] = ptr %679
 42205|     ;; self = ptr %679
 42206|     ;; f[0..+8] = ptr %295
 42207|     ;; f[8..+8] = ptr %296
 42208|     ;; x = ptr %679
 42209|  %680 = gep %296, i64 496                                                                                              ;L1543<381<204
 42210|  %681 = load ptr, ptr %680, , !!48170                                                                                  ;L1543<381<204
 42211|  %682 = load i64, ptr %679, , !!48170, !!8                                                                             ;L1543<381<204
 42213|  %683 = invoke ptr %681(ptr %295, i64 %682)
 42214|  to label %685 unwind label %288, !!48170                                                                              ;L381<1543<381<204
 42215| 
 42216| 684: ; preds = %685, %671
 42219|  invoke void @ai::small_action6aroundNtB5_23SmallActionAroundRegion3new(ptr sret([120 x i8]) %16, i64 %204, ptr %3, ptr %5, ptr %4, i64 7, i64 5)
 42220|  to label %821 unwind label %288, !!48170                                                                              ;L406<204
 42221| 
 42222| 685: ; preds = %677
 42223|  %686 = icmp eq ptr %683, null                                                                                         ;L381<204
 42224|  br i1 %686, label %684, label %687                                                                                    ;L381<204
 42225| 
 42226| 687: ; preds = %685
 42227|     ;; epic = ptr %683
 42228|     ;; self = ptr %683
 42229|     ;; self = ptr %683
 42230|     ;; other = ptr %683
 42231|  %688 = gep %683, i64 104                                                                                              ;L382<204
 42232|  %689 = load i64, ptr %688, , !!48170, !!8                                                                             ;L382<204
 42233|  %690 = icmp eq i64 %689, 5                                                                                            ;L382<204
 42234|  br i1 %690, label %691, label %693                                                                                    ;L382<204
 42235| 
 42236| 691: ; preds = %687
 42237|  %692 = icmp eq i64 %666, 7                                                                                            ;L383<204
 42238|  br i1 %692, label %694, label %697                                                                                    ;L383<204
 42239| 
 42240| 693: ; preds = %687
 42243|  invoke void @ai::small_action6aroundNtB5_23SmallActionAroundRegion3new(ptr sret([120 x i8]) %18, i64 %204, ptr %3, ptr %5, ptr %4, i64 7, i64 5)
 42244|  to label %806 unwind label %288, !!48170                                                                              ;L403<204
 42245| 
 42246| 694: ; preds = %691
 42247|     ;; self = ptr %209
 42248|  %695 = load i64, ptr %209, , !!48170, !!8                                                                             ;L1136<1482<386<204
 42249|  %696 = trunc nuw i64 %695 to i1                                                                                       ;L1136<1482<386<204
 42250|  br i1 %696, label %713, label %700                                                                                    ;L1136<1482<386<204
 42251| 
 42252| 697: ; preds = %691
 42255|  %698 = gep %683, i64 1472                                                                                             ;L384<204
 42256|  %699 = load i64, ptr %698, , !!48170, !!8                                                                             ;L384<204
 42257|  invoke void @ai::small_action6aroundNtB2_17SmallActionAround3new(ptr sret([136 x i8]) %26, i64 %204, ptr %3, ptr %5, ptr %4, i64 %699, i64 5)
 42258|  to label %797 unwind label %288, !!48170                                                                              ;L384<204
 42259| 
 42260| 700: ; preds = %694
 42261|  %701 = gep %209, i64 8                                                                                                ;L1136<1482<386<204
 42262|     ;; team = ptr %209
 42263|  %702 = load i64, ptr %701, , !!48170, !!8                                                                             ;L1137<1482<386<204
 42264|     ;; team = i64 %702
 42265|  %703 = icmp ult i64 %702, 2                                                                                           ;L1483<386<204
 42266|  br i1 %703, label %704, label %709                                                                                    ;L1483<386<204
 42267| 
 42268| 704: ; preds = %700
 42270|  %705 = gep %683, i64 56                                                                                               ;L122<1483<386<204
 42271|  %706 = gepS %705, i64 %702                                                                                            ;L122<1483<386<204
 42272|  %707 = load i64, ptr %706, , !!48170, !!8                                                                             ;L122<1483<386<204
 42273|  %708 = icmp eq i64 %707, 0                                                                                            ;L122<1483<386<204
 42274|  br i1 %708, label %713, label %710                                                                                    ;L386<204
 42275| 
 42276| 709: ; preds = %700
 42277|  invoke void @core::panicking18panic_bounds_check(i64 %702, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 42278|  to label %201 unwind label %288, !!48170                                                                              ;L1483<386<204
 42279| 
 42280| 710: ; preds = %704
 42283|  %711 = gep %683, i64 1472                                                                                             ;L387<204
 42284|  %712 = load i64, ptr %711, , !!48170, !!8                                                                             ;L387<204
 42285|  invoke void @ai::small_action6aroundNtB2_17SmallActionAround3new(ptr sret([136 x i8]) %24, i64 %204, ptr %3, ptr %5, ptr %4, i64 %712, i64 5)
 42286|  to label %718 unwind label %288, !!48170                                                                              ;L387<204
 42287| 
 42288| 713: ; preds = %704, %694
 42289|     ;; self = ptr %683
 42290|  %714 = gep %683, i64 1168                                                                                             ;L742<389<204
 42291|  %715 = gep %683, i64 1216                                                                                             ;L742<389<204
 42292|  %716 = load i32, ptr %715, , !!48170, !!8                                                                             ;L742<389<204
 42293|  %717 = icmp eq i32 %716, -1                                                                                           ;L742<389<204
 42294|  br i1 %717, label %734, label %721                                                                                    ;L742<389<204
 42295| 
 42296| 718: ; preds = %710
 42297|  call void @llvm.memcpy.p0.p0.i64(ptr %25, ptr %24, i64 136, i1 false), !!48147                                        ;L387<204
 42298|  %719 = gep %25, i64 177                                                                                               ;L387<204
 42299|  store i8 5, ptr %719, , !!48147                                                                                       ;L387<204
 42301|  invoke fastcc void @ai::small_action15SmallActionPlayE4pushBX_(ptr %39, ptr %25)
 42302|  to label %720 unwind label %288, !!48170                                                                              ;L387<204
 42303| 
 42304| 720: ; preds = %718
 42306|  br label %800                                                                                                         ;L386<204
 42307| 
 42308| 721: ; preds = %713
 42309|     ;; self = ptr %714
 42310|     ;; self = ptr %714
 42311|  %722 = gep %683, i64 1184                                                                                             ;L26<389<204
 42312|  %723 = load i64, ptr %722, , !!48170, !!8                                                                             ;L26<389<204
 42313|  %724 = gep %683, i64 1192                                                                                             ;L26<389<204
 42314|  %725 = load i64, ptr %724, , !!48170, !!8                                                                             ;L26<389<204
 42315|  %726 = gep %209, i64 1480                                                                                             ;L26<389<204
 42316|  %727 = load i64, ptr %726, , !!48170, !!8                                                                             ;L26<389<204
 42317|  %728 = add i64 %727, -1                                                                                               ;L26<389<204
 42318|  %729 = mul i64 %728, %725                                                                                             ;L26<389<204
 42319|  %730 = gep %209, i64 1080                                                                                             ;L26<389<204
 42320|  %731 = load i64, ptr %730, , !!48170, !!8                                                                             ;L26<389<204
 42321|  %732 = load i32, ptr %577, , !!48170, !!8                                                                             ;L1511<389<204
 42322|     ;; mult = i32 %732
 42323|  %733 = icmp eq i32 %732, 0                                                                                            ;L1512<389<204
 42324|  br i1 %733, label %735, label %737                                                                                    ;L1512<389<204
 42325| 
 42326| 734: ; preds = %713
 42327|     ;; self = ptr null
 42328|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.203) #31
 42329|  to label %201 unwind label %288, !!48170                                                                              ;L1013<389<204
 42330| 
 42331| 735: ; preds = %721
 42332|  %736 = load i64, ptr %578, , !!48170, !!8                                                                             ;L1513<389<204
 42333|  br label %743                                                                                                         ;L1512<389<204
 42334| 
 42335| 737: ; preds = %721
 42336|  %738 = sext i32 %732 to i64                                                                                           ;L1511<389<204
 42337|     ;; mult = i64 %738
 42338|  %739 = load i64, ptr %578, , !!48170, !!8                                                                             ;L1515<389<204
 42339|  %740 = add nsw i64 %738, 100                                                                                          ;L1515<389<204
 42340|  %741 = mul i64 %739, %740                                                                                             ;L1515<389<204
 42341|  %742 = udiv i64 %741, 100                                                                                             ;L1515<389<204
 42342|  br label %743                                                                                                         ;L1512<389<204
 42343| 
 42344| 743: ; preds = %737, %735
 42345|  %744 = phi i64 [ %736, %735 ], [ %742, %737 ]                                                                         ;L0<389<204
 42346|  %745 = gep %683, i64 1136                                                                                             ;L1511<389<204
 42347|  %746 = load i32, ptr %745, , !!48170, !!8                                                                             ;L1511<389<204
 42348|     ;; mult = i32 %746
 42349|  %747 = icmp eq i32 %746, 0                                                                                            ;L1512<389<204
 42350|  br i1 %747, label %748, label %751                                                                                    ;L1512<389<204
 42351| 
 42352| 748: ; preds = %743
 42353|  %749 = gep %683, i64 1664                                                                                             ;L1513<389<204
 42354|  %750 = load i64, ptr %749, , !!48170, !!8                                                                             ;L1513<389<204
 42355|  br label %758                                                                                                         ;L1512<389<204
 42356| 
 42357| 751: ; preds = %743
 42358|  %752 = sext i32 %746 to i64                                                                                           ;L1511<389<204
 42359|     ;; mult = i64 %752
 42360|  %753 = gep %683, i64 1664                                                                                             ;L1515<389<204
 42361|  %754 = load i64, ptr %753, , !!48170, !!8                                                                             ;L1515<389<204
 42362|  %755 = add nsw i64 %752, 100                                                                                          ;L1515<389<204
 42363|  %756 = mul i64 %754, %755                                                                                             ;L1515<389<204
 42364|  %757 = udiv i64 %756, 100                                                                                             ;L1515<389<204
 42365|  br label %758                                                                                                         ;L1512<389<204
 42366| 
 42367| 758: ; preds = %751, %748
 42368|  %759 = phi i64 [ %750, %748 ], [ %757, %751 ]                                                                         ;L0<389<204
 42369|  %760 = add i64 %723, 20000                                                                                            ;L26<389<204
 42370|  %761 = add i64 %760, %731                                                                                             ;L26<389<204
 42371|  %762 = add i64 %761, %729                                                                                             ;L389<204
 42372|  %763 = add i64 %762, %744                                                                                             ;L389<204
 42373|  %764 = add i64 %763, %759                                                                                             ;L389<204
 42374|     ;; range = i64 %764
 42375|     ;; self = ptr %683
 42376|     ;; self = ptr %714
 42377|  %765 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %714, ptr %182, ptr %683, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %209)
 42378|  to label %766 unwind label %288, !!48170                                                                              ;L390<204
 42379| 
 42380| 766: ; preds = %758
 42381|     ;; dmg = i64 %765
 42382|  %767 = shl i64 %765, 1                                                                                                ;L392<204
 42383|  %768 = gep %209, i64 1648                                                                                             ;L392<204
 42384|  %769 = load i64, ptr %768, , !!48170, !!8                                                                             ;L392<204
 42385|  %770 = icmp ult i64 %767, %769                                                                                        ;L392<204
 42386|  br i1 %770, label %789, label %771                                                                                    ;L392<204
 42387| 
 42388| 771: ; preds = %766
 42389|  %772 = mul i64 %764, %764                                                                                             ;L392<204
 42390|  %773 = gep %683, i64 1632                                                                                             ;L2158<392<204
 42391|  %774 = load i64, ptr %773, , !!48170, !!8                                                                             ;L2158<392<204
 42392|     ;; x2 = i64 %774
 42393|     ;; other = i64 %774
 42394|  %775 = gep %683, i64 1640                                                                                             ;L2158<392<204
 42395|  %776 = load i64, ptr %775, , !!48170, !!8                                                                             ;L2158<392<204
 42396|     ;; y2 = i64 %776
 42397|     ;; other = i64 %776
 42398|  %777 = icmp ult i64 %267, %774                                                                                        ;L3147<7<2158<392<204
 42399|  %778 = sub nuw i64 %774, %267                                                                                         ;L3147<7<2158<392<204
 42400|  %779 = sub nuw i64 %267, %774                                                                                         ;L3147<7<2158<392<204
 42401|  %780 = select i1 %777, i64 %778, i64 %779                                                                             ;L3147<7<2158<392<204
 42402|     ;; dx = i64 %780
 42403|  %781 = icmp ult i64 %269, %776                                                                                        ;L3147<8<2158<392<204
 42404|  %782 = sub nuw i64 %776, %269                                                                                         ;L3147<8<2158<392<204
 42405|  %783 = sub nuw i64 %269, %776                                                                                         ;L3147<8<2158<392<204
 42406|  %784 = select i1 %781, i64 %782, i64 %783                                                                             ;L3147<8<2158<392<204
 42407|     ;; dy = i64 %784
 42408|  %785 = mul i64 %780, %780                                                                                             ;L9<2158<392<204
 42409|  %786 = mul i64 %784, %784                                                                                             ;L9<2158<392<204
 42410|  %787 = add i64 %786, %785                                                                                             ;L9<2158<392<204
 42411|  %788 = icmp ult i64 %772, %787                                                                                        ;L392<204
 42412|  br i1 %788, label %789, label %790                                                                                    ;L392<204
 42413| 
 42414| 789: ; preds = %771, %766
 42417|  invoke void @ai::small_action6aroundNtB5_23SmallActionAroundRegion3new(ptr sret([120 x i8]) %20, i64 %204, ptr %3, ptr %5, ptr %4, i64 7, i64 5)
 42418|  to label %791 unwind label %288, !!48170                                                                              ;L395<204
 42419| 
 42420| 790: ; preds = %771
 42423|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway3new(ptr sret([136 x i8]) %22, ptr %5, ptr %4, i64 5)
 42424|  to label %794 unwind label %288, !!48170                                                                              ;L393<204
 42425| 
 42426| 791: ; preds = %789
 42427|  call void @llvm.memcpy.p0.p0.i64(ptr %21, ptr %20, i64 120, i1 false), !!48147                                        ;L395<204
 42428|  %792 = gep %21, i64 177                                                                                               ;L395<204
 42429|  store i8 7, ptr %792, , !!48147                                                                                       ;L395<204
 42431|  invoke fastcc void @ai::small_action15SmallActionPlayE4pushBX_(ptr %39, ptr %21)
 42432|  to label %793 unwind label %288, !!48170                                                                              ;L395<204
 42433| 
 42434| 793: ; preds = %791
 42436|  br label %800                                                                                                         ;L392<204
 42437| 
 42438| 794: ; preds = %790
 42439|  call void @llvm.memcpy.p0.p0.i64(ptr %23, ptr %22, i64 136, i1 false), !!48147                                        ;L393<204
 42440|  %795 = gep %23, i64 177                                                                                               ;L393<204
 42441|  store i8 3, ptr %795, , !!48147                                                                                       ;L393<204
 42443|  invoke fastcc void @ai::small_action15SmallActionPlayE4pushBX_(ptr %39, ptr %23)
 42444|  to label %796 unwind label %288, !!48170                                                                              ;L393<204
 42445| 
 42446| 796: ; preds = %794
 42448|  br label %800                                                                                                         ;L392<204
 42449| 
 42450| 797: ; preds = %697
 42451|  call void @llvm.memcpy.p0.p0.i64(ptr %27, ptr %26, i64 136, i1 false), !!48147                                        ;L384<204
 42452|  %798 = gep %27, i64 177                                                                                               ;L384<204
 42453|  store i8 5, ptr %798, , !!48147                                                                                       ;L384<204
 42455|  invoke fastcc void @ai::small_action15SmallActionPlayE4pushBX_(ptr %39, ptr %27)
 42456|  to label %799 unwind label %288, !!48170                                                                              ;L384<204
 42457| 
 42458| 799: ; preds = %797
 42460|  br label %800                                                                                                         ;L383<204
 42461| 
 42462| 800: ; preds = %831, %816, %799, %796, %793, %720
 42463|  %801 = gep %209, i64 1472                                                                                             ;L409<204
 42464|  %802 = load i64, ptr %801, , !!48170, !!8                                                                             ;L409<204
 42465|  %803 = gep %296, i64 248                                                                                              ;L409<204
 42466|  %804 = load ptr, ptr %803, , !!48170, !!8                                                                             ;L409<204
 42467|  %805 = invoke zeroext i1 %804(ptr %295, i64 %96, i64 %802)
 42468|  to label %836 unwind label %288, !!48170                                                                              ;L409<204
 42469| 
 42470| 806: ; preds = %693
 42471|  call void @llvm.memcpy.p0.p0.i64(ptr %19, ptr %18, i64 120, i1 false), !!48147                                        ;L403<204
 42472|  %807 = gep %19, i64 177                                                                                               ;L403<204
 42473|  store i8 7, ptr %807, , !!48147                                                                                       ;L403<204
 42476|     ;; self = ptr %39
 42477|     ;; self = ptr %39
 42478|     ;; value = ptr %19
 42479|     ;; src = ptr %19
 42480|     ;; additional = i64 1
 42481|     ;; needed_extra_cap = i64 1
 42482|     ;; needed_extra_cap = i64 1
 42483|     ;; strategy = i8 1
 42484|     ;; self = ptr %39
 42485|  %808 = load i64, ptr %207, , !!49100, !!8                                                                             ;L149<1428<403<204
 42486|  %809 = icmp eq i64 %808, 0                                                                                            ;L1428<403<204
 42487|  br i1 %809, label %810, label %816                                                                                    ;L1428<403<204
 42488| 
 42489| 810: ; preds = %806
 42490|     ;; self = ptr %39
 42491|     ;; self = ptr %39
 42492|     ;; self = ptr %39
 42493|     ;; used_cap = i64 %361
 42494|     ;; used_cap = i64 %361
 42495|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %39, i64 0, i64 1, i1 zeroext true)
 42496|  to label %811 unwind label %814, !!49106                                                                              ;L619<430<738<1429<403<204
 42497| 
 42498| 811: ; preds = %810
 42499|  %812 = load i64, ptr %208, , !!49100                                                                                  ;L1432<403<204
 42500|  %813 = load ptr, ptr %39, , !!49100                                                                                   ;L138<1432<403<204
 42501|  br label %816                                                                                                         ;L619<430<738<1429<403<204
 42502| 
 42503| 814: ; preds = %810
 42504|  %815 = cleanuppad within none []
 42505|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %19) #30 [ "funclet"(token %815) ], !!49111 ;L1436<403<204
 42506|  cleanupret from %815 unwind label %288
 42507| 
 42508| 816: ; preds = %811, %806
 42509|  %817 = phi ptr [ %813, %811 ], [ %358, %806 ]                                                                         ;L138<1432<403<204
 42510|  %818 = phi i64 [ %812, %811 ], [ 0, %806 ]                                                                            ;L1432<403<204
 42511|     ;; self = ptr %39
 42512|     ;; self = ptr %817
 42513|     ;; count = i64 %818
 42514|  %819 = gepS %817, i64 %818                                                                                            ;L961<1432<403<204
 42515|     ;; end = ptr %819
 42516|     ;; dst = ptr %819
 42517|  call void @llvm.memcpy.p0.p0.i64(ptr %819, ptr %19, i64 184, i1 false), !!49111                                       ;L1933<1433<403<204
 42518|  %820 = add i64 %818, 1                                                                                                ;L1434<403<204
 42519|  store i64 %820, ptr %208, , !!49100                                                                                   ;L1434<403<204
 42521|  br label %800                                                                                                         ;L382<204
 42522| 
 42523| 821: ; preds = %684
 42524|  call void @llvm.memcpy.p0.p0.i64(ptr %17, ptr %16, i64 120, i1 false), !!48147                                        ;L406<204
 42525|  %822 = gep %17, i64 177                                                                                               ;L406<204
 42526|  store i8 7, ptr %822, , !!48147                                                                                       ;L406<204
 42529|     ;; self = ptr %39
 42530|     ;; self = ptr %39
 42531|     ;; value = ptr %17
 42532|     ;; src = ptr %17
 42533|     ;; additional = i64 1
 42534|     ;; needed_extra_cap = i64 1
 42535|     ;; needed_extra_cap = i64 1
 42536|     ;; strategy = i8 1
 42537|     ;; self = ptr %39
 42538|  %823 = load i64, ptr %207, , !!49136, !!8                                                                             ;L149<1428<406<204
 42539|  %824 = icmp eq i64 %823, 0                                                                                            ;L1428<406<204
 42540|  br i1 %824, label %825, label %831                                                                                    ;L1428<406<204
 42541| 
 42542| 825: ; preds = %821
 42543|     ;; self = ptr %39
 42544|     ;; self = ptr %39
 42545|     ;; self = ptr %39
 42546|     ;; used_cap = i64 %361
 42547|     ;; used_cap = i64 %361
 42548|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %39, i64 0, i64 1, i1 zeroext true)
 42549|  to label %826 unwind label %829, !!49142                                                                              ;L619<430<738<1429<406<204
 42550| 
 42551| 826: ; preds = %825
 42552|  %827 = load i64, ptr %208, , !!49136                                                                                  ;L1432<406<204
 42553|  %828 = load ptr, ptr %39, , !!49136                                                                                   ;L138<1432<406<204
 42554|  br label %831                                                                                                         ;L619<430<738<1429<406<204
 42555| 
 42556| 829: ; preds = %825
 42557|  %830 = cleanuppad within none []
 42558|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %17) #30 [ "funclet"(token %830) ], !!49147 ;L1436<406<204
 42559|  cleanupret from %830 unwind label %288
 42560| 
 42561| 831: ; preds = %826, %821
 42562|  %832 = phi ptr [ %828, %826 ], [ %358, %821 ]                                                                         ;L138<1432<406<204
 42563|  %833 = phi i64 [ %827, %826 ], [ 0, %821 ]                                                                            ;L1432<406<204
 42564|     ;; self = ptr %39
 42565|     ;; self = ptr %832
 42566|     ;; count = i64 %833
 42567|  %834 = gepS %832, i64 %833                                                                                            ;L961<1432<406<204
 42568|     ;; end = ptr %834
 42569|     ;; dst = ptr %834
 42570|  call void @llvm.memcpy.p0.p0.i64(ptr %834, ptr %17, i64 184, i1 false), !!49147                                       ;L1933<1433<406<204
 42571|  %835 = add i64 %833, 1                                                                                                ;L1434<406<204
 42572|  store i64 %835, ptr %208, , !!49136                                                                                   ;L1434<406<204
 42574|  br label %800                                                                                                         ;L381<204
 42575| 
 42576| 836: ; preds = %800
 42577|  br i1 %805, label %837, label %655                                                                                    ;L409<204
 42578| 
 42579| 837: ; preds = %836
 42580|     ;; self[0..+8] = ptr %212
 42581|     ;; slice[0..+8] = ptr %212
 42582|     ;; self[8..+8] = i64 5
 42583|     ;; slice[8..+8] = i64 5
 42584|     ;; self = ptr %212
 42585|     ;; self = ptr undef
 42586|     ;; self = ptr undef
 42587|     ;; f = ptr %209
 42588|     ;; fold = ptr %209
 42591|     ;; f[8..+8] = ptr %209
 42592|     ;; self = ptr undef
 42595|     ;; self = ptr undef
 42596|     ;; count = i64 1
 42597|     ;; ptr = ptr %212
 42598|     ;; self = ptr %212
 42599|     ;; end_or_len = ptr %213
 42602|  %838 = load i64, ptr %209, , !!49172
 42603|  %839 = gep %209, i64 8
 42604|  %840 = load i64, ptr %839, , !!49172
 42605|  %841 = load i64, ptr %266, , !!49172
 42606|  %842 = load i64, ptr %268, , !!49172
 42607|  %843 = icmp eq i64 %838, 0
 42608|     ;; x = ptr %212
 42609|  %844 = load ptr, ptr %212, , !!49176, !!8                                                                             ;L2494<138<2897<411<204
 42614|  %845 = icmp eq ptr %844, null                                                                                         ;L49<2494<138<2897<411<204
 42615|  br i1 %845, label %868, label %846                                                                                    ;L49<2494<138<2897<411<204
 42616| 
 42617| 846: ; preds = %837
 42618|     ;; x = ptr %844
 42621|     ;; x = ptr %844
 42623|     ;; c = ptr %844
 42624|     ;; self = ptr %844
 42625|     ;; self = ptr %844
 42626|     ;; self = ptr %844
 42627|     ;; other = ptr %209
 42628|     ;; other = ptr %209
 42629|  %847 = load i64, ptr %844, , !!49176, !!8                                                                             ;L1127<264<411<2893<50<2494<138<2897<411<204
 42630|  %848 = gep %844, i64 8                                                                                                ;L1127<264<411<2893<50<2494<138<2897<411<204
 42631|     ;; __self_discr = i64 %847
 42632|     ;; __arg1_discr = i64 %838
 42633|  %849 = icmp eq i64 %847, %838                                                                                         ;L1127<264<411<2893<50<2494<138<2897<411<204
 42634|  br i1 %849, label %850, label %851                                                                                    ;L1127<264<411<2893<50<2494<138<2897<411<204
 42635| 
 42636| 850: ; preds = %846
 42637|  br i1 %843, label %985, label %868                                                                                    ;L1127<264<411<2893<50<2494<138<2897<411<204
 42638| 
 42639| 851: ; preds = %985, %846
 42640|     ;; other = ptr %209
 42641|  %852 = gep %844, i64 1632                                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42642|  %853 = load i64, ptr %852, , !!49176, !!8                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42643|     ;; x1 = i64 %853
 42644|     ;; self = i64 %853
 42645|  %854 = gep %844, i64 1640                                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42646|  %855 = load i64, ptr %854, , !!49176, !!8                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42647|     ;; y1 = i64 %855
 42648|     ;; self = i64 %855
 42649|     ;; x2 = i64 %841
 42650|     ;; other = i64 %841
 42651|     ;; y2 = i64 %842
 42652|     ;; other = i64 %842
 42653|  %856 = icmp ult i64 %853, %841                                                                                        ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42654|  %857 = sub nuw i64 %841, %853                                                                                         ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42655|  %858 = sub nuw i64 %853, %841                                                                                         ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42656|  %859 = select i1 %856, i64 %857, i64 %858                                                                             ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42657|     ;; dx = i64 %859
 42658|  %860 = icmp ult i64 %855, %842                                                                                        ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42659|  %861 = sub nuw i64 %842, %855                                                                                         ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42660|  %862 = sub nuw i64 %855, %842                                                                                         ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42661|  %863 = select i1 %860, i64 %861, i64 %862                                                                             ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42662|     ;; dy = i64 %863
 42663|  %864 = mul i64 %859, %859                                                                                             ;L9<2158<411<2893<50<2494<138<2897<411<204
 42664|  %865 = mul i64 %863, %863                                                                                             ;L9<2158<411<2893<50<2494<138<2897<411<204
 42665|  %866 = add i64 %865, %864                                                                                             ;L9<2158<411<2893<50<2494<138<2897<411<204
 42666|  %867 = icmp ult i64 %866, 22500000001                                                                                 ;L411<2893<50<2494<138<2897<411<204
 42667|  br i1 %867, label %1010, label %868                                                                                   ;L2494<138<2897<411<204
 42668| 
 42669| 868: ; preds = %985, %851, %850, %837
 42670|     ;; self = ptr undef
 42671|     ;; count = i64 1
 42672|     ;; ptr = !DIArgList(ptr %212, i64 8)
 42673|     ;; self = !DIArgList(ptr %212, i64 8)
 42674|     ;; end_or_len = ptr %213
 42677|  %869 = gep %212, i64 8                                                                                                ;L656<185<2493<138<2897<411<204
 42678|     ;; ptr = ptr %869
 42679|     ;; x = ptr %869
 42680|  %870 = load ptr, ptr %869, , !!49176, !!8                                                                             ;L2494<138<2897<411<204
 42685|  %871 = icmp eq ptr %870, null                                                                                         ;L49<2494<138<2897<411<204
 42686|  br i1 %871, label %897, label %872                                                                                    ;L49<2494<138<2897<411<204
 42687| 
 42688| 872: ; preds = %868
 42689|     ;; x = ptr %870
 42692|     ;; x = ptr %870
 42694|     ;; c = ptr %870
 42695|     ;; self = ptr %870
 42696|     ;; self = ptr %870
 42697|     ;; self = ptr %870
 42698|     ;; other = ptr %209
 42699|     ;; other = ptr %209
 42700|  %873 = load i64, ptr %870, , !!49176, !!8                                                                             ;L1127<264<411<2893<50<2494<138<2897<411<204
 42701|  %874 = gep %870, i64 8                                                                                                ;L1127<264<411<2893<50<2494<138<2897<411<204
 42702|     ;; __self_discr = i64 %873
 42703|     ;; __arg1_discr = i64 %838
 42704|  %875 = icmp eq i64 %873, %838                                                                                         ;L1127<264<411<2893<50<2494<138<2897<411<204
 42705|  br i1 %875, label %876, label %880                                                                                    ;L1127<264<411<2893<50<2494<138<2897<411<204
 42706| 
 42707| 876: ; preds = %872
 42708|  br i1 %843, label %877, label %897                                                                                    ;L1127<264<411<2893<50<2494<138<2897<411<204
 42709| 
 42710| 877: ; preds = %876
 42711|     ;; __self_0 = ptr %870
 42712|     ;; self = ptr %870
 42713|     ;; __arg1_0 = ptr %209
 42714|     ;; other = ptr %209
 42717|  %878 = load i64, ptr %874, , !!49176, !!8                                                                             ;L1878<2123<1127<264<411<2893<50<2494<138<2897<411<204
 42718|  %879 = icmp eq i64 %878, %840                                                                                         ;L1878<2123<1127<264<411<2893<50<2494<138<2897<411<204
 42719|  br i1 %879, label %897, label %880                                                                                    ;L411<2893<50<2494<138<2897<411<204
 42720| 
 42721| 880: ; preds = %877, %872
 42722|     ;; other = ptr %209
 42723|  %881 = gep %870, i64 1632                                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42724|  %882 = load i64, ptr %881, , !!49176, !!8                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42725|     ;; x1 = i64 %882
 42726|     ;; self = i64 %882
 42727|  %883 = gep %870, i64 1640                                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42728|  %884 = load i64, ptr %883, , !!49176, !!8                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42729|     ;; y1 = i64 %884
 42730|     ;; self = i64 %884
 42731|     ;; x2 = i64 %841
 42732|     ;; other = i64 %841
 42733|     ;; y2 = i64 %842
 42734|     ;; other = i64 %842
 42735|  %885 = icmp ult i64 %882, %841                                                                                        ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42736|  %886 = sub nuw i64 %841, %882                                                                                         ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42737|  %887 = sub nuw i64 %882, %841                                                                                         ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42738|  %888 = select i1 %885, i64 %886, i64 %887                                                                             ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42739|     ;; dx = i64 %888
 42740|  %889 = icmp ult i64 %884, %842                                                                                        ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42741|  %890 = sub nuw i64 %842, %884                                                                                         ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42742|  %891 = sub nuw i64 %884, %842                                                                                         ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42743|  %892 = select i1 %889, i64 %890, i64 %891                                                                             ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42744|     ;; dy = i64 %892
 42745|  %893 = mul i64 %888, %888                                                                                             ;L9<2158<411<2893<50<2494<138<2897<411<204
 42746|  %894 = mul i64 %892, %892                                                                                             ;L9<2158<411<2893<50<2494<138<2897<411<204
 42747|  %895 = add i64 %894, %893                                                                                             ;L9<2158<411<2893<50<2494<138<2897<411<204
 42748|  %896 = icmp ult i64 %895, 22500000001                                                                                 ;L411<2893<50<2494<138<2897<411<204
 42749|  br i1 %896, label %1010, label %897                                                                                   ;L2494<138<2897<411<204
 42750| 
 42751| 897: ; preds = %880, %877, %876, %868
 42752|     ;; self = ptr undef
 42753|     ;; count = i64 1
 42754|     ;; ptr = !DIArgList(ptr %212, i64 16)
 42755|     ;; self = !DIArgList(ptr %212, i64 16)
 42756|     ;; end_or_len = ptr %213
 42759|  %898 = gep %212, i64 16                                                                                               ;L656<185<2493<138<2897<411<204
 42760|     ;; ptr = ptr %898
 42761|     ;; x = ptr %898
 42762|  %899 = load ptr, ptr %898, , !!49176, !!8                                                                             ;L2494<138<2897<411<204
 42767|  %900 = icmp eq ptr %899, null                                                                                         ;L49<2494<138<2897<411<204
 42768|  br i1 %900, label %926, label %901                                                                                    ;L49<2494<138<2897<411<204
 42769| 
 42770| 901: ; preds = %897
 42771|     ;; x = ptr %899
 42774|     ;; x = ptr %899
 42776|     ;; c = ptr %899
 42777|     ;; self = ptr %899
 42778|     ;; self = ptr %899
 42779|     ;; self = ptr %899
 42780|     ;; other = ptr %209
 42781|     ;; other = ptr %209
 42782|  %902 = load i64, ptr %899, , !!49176, !!8                                                                             ;L1127<264<411<2893<50<2494<138<2897<411<204
 42783|  %903 = gep %899, i64 8                                                                                                ;L1127<264<411<2893<50<2494<138<2897<411<204
 42784|     ;; __self_discr = i64 %902
 42785|     ;; __arg1_discr = i64 %838
 42786|  %904 = icmp eq i64 %902, %838                                                                                         ;L1127<264<411<2893<50<2494<138<2897<411<204
 42787|  br i1 %904, label %905, label %909                                                                                    ;L1127<264<411<2893<50<2494<138<2897<411<204
 42788| 
 42789| 905: ; preds = %901
 42790|  br i1 %843, label %906, label %926                                                                                    ;L1127<264<411<2893<50<2494<138<2897<411<204
 42791| 
 42792| 906: ; preds = %905
 42793|     ;; __self_0 = ptr %899
 42794|     ;; self = ptr %899
 42795|     ;; __arg1_0 = ptr %209
 42796|     ;; other = ptr %209
 42799|  %907 = load i64, ptr %903, , !!49176, !!8                                                                             ;L1878<2123<1127<264<411<2893<50<2494<138<2897<411<204
 42800|  %908 = icmp eq i64 %907, %840                                                                                         ;L1878<2123<1127<264<411<2893<50<2494<138<2897<411<204
 42801|  br i1 %908, label %926, label %909                                                                                    ;L411<2893<50<2494<138<2897<411<204
 42802| 
 42803| 909: ; preds = %906, %901
 42804|     ;; other = ptr %209
 42805|  %910 = gep %899, i64 1632                                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42806|  %911 = load i64, ptr %910, , !!49176, !!8                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42807|     ;; x1 = i64 %911
 42808|     ;; self = i64 %911
 42809|  %912 = gep %899, i64 1640                                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42810|  %913 = load i64, ptr %912, , !!49176, !!8                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42811|     ;; y1 = i64 %913
 42812|     ;; self = i64 %913
 42813|     ;; x2 = i64 %841
 42814|     ;; other = i64 %841
 42815|     ;; y2 = i64 %842
 42816|     ;; other = i64 %842
 42817|  %914 = icmp ult i64 %911, %841                                                                                        ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42818|  %915 = sub nuw i64 %841, %911                                                                                         ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42819|  %916 = sub nuw i64 %911, %841                                                                                         ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42820|  %917 = select i1 %914, i64 %915, i64 %916                                                                             ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42821|     ;; dx = i64 %917
 42822|  %918 = icmp ult i64 %913, %842                                                                                        ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42823|  %919 = sub nuw i64 %842, %913                                                                                         ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42824|  %920 = sub nuw i64 %913, %842                                                                                         ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42825|  %921 = select i1 %918, i64 %919, i64 %920                                                                             ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42826|     ;; dy = i64 %921
 42827|  %922 = mul i64 %917, %917                                                                                             ;L9<2158<411<2893<50<2494<138<2897<411<204
 42828|  %923 = mul i64 %921, %921                                                                                             ;L9<2158<411<2893<50<2494<138<2897<411<204
 42829|  %924 = add i64 %923, %922                                                                                             ;L9<2158<411<2893<50<2494<138<2897<411<204
 42830|  %925 = icmp ult i64 %924, 22500000001                                                                                 ;L411<2893<50<2494<138<2897<411<204
 42831|  br i1 %925, label %1010, label %926                                                                                   ;L2494<138<2897<411<204
 42832| 
 42833| 926: ; preds = %909, %906, %905, %897
 42834|     ;; self = ptr undef
 42835|     ;; count = i64 1
 42836|     ;; ptr = !DIArgList(ptr %212, i64 24)
 42837|     ;; self = !DIArgList(ptr %212, i64 24)
 42838|     ;; end_or_len = ptr %213
 42841|  %927 = gep %212, i64 24                                                                                               ;L656<185<2493<138<2897<411<204
 42842|     ;; ptr = ptr %927
 42843|     ;; x = ptr %927
 42844|  %928 = load ptr, ptr %927, , !!49176, !!8                                                                             ;L2494<138<2897<411<204
 42849|  %929 = icmp eq ptr %928, null                                                                                         ;L49<2494<138<2897<411<204
 42850|  br i1 %929, label %955, label %930                                                                                    ;L49<2494<138<2897<411<204
 42851| 
 42852| 930: ; preds = %926
 42853|     ;; x = ptr %928
 42856|     ;; x = ptr %928
 42858|     ;; c = ptr %928
 42859|     ;; self = ptr %928
 42860|     ;; self = ptr %928
 42861|     ;; self = ptr %928
 42862|     ;; other = ptr %209
 42863|     ;; other = ptr %209
 42864|  %931 = load i64, ptr %928, , !!49176, !!8                                                                             ;L1127<264<411<2893<50<2494<138<2897<411<204
 42865|  %932 = gep %928, i64 8                                                                                                ;L1127<264<411<2893<50<2494<138<2897<411<204
 42866|     ;; __self_discr = i64 %931
 42867|     ;; __arg1_discr = i64 %838
 42868|  %933 = icmp eq i64 %931, %838                                                                                         ;L1127<264<411<2893<50<2494<138<2897<411<204
 42869|  br i1 %933, label %934, label %938                                                                                    ;L1127<264<411<2893<50<2494<138<2897<411<204
 42870| 
 42871| 934: ; preds = %930
 42872|  br i1 %843, label %935, label %955                                                                                    ;L1127<264<411<2893<50<2494<138<2897<411<204
 42873| 
 42874| 935: ; preds = %934
 42875|     ;; __self_0 = ptr %928
 42876|     ;; self = ptr %928
 42877|     ;; __arg1_0 = ptr %209
 42878|     ;; other = ptr %209
 42881|  %936 = load i64, ptr %932, , !!49176, !!8                                                                             ;L1878<2123<1127<264<411<2893<50<2494<138<2897<411<204
 42882|  %937 = icmp eq i64 %936, %840                                                                                         ;L1878<2123<1127<264<411<2893<50<2494<138<2897<411<204
 42883|  br i1 %937, label %955, label %938                                                                                    ;L411<2893<50<2494<138<2897<411<204
 42884| 
 42885| 938: ; preds = %935, %930
 42886|     ;; other = ptr %209
 42887|  %939 = gep %928, i64 1632                                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42888|  %940 = load i64, ptr %939, , !!49176, !!8                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42889|     ;; x1 = i64 %940
 42890|     ;; self = i64 %940
 42891|  %941 = gep %928, i64 1640                                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42892|  %942 = load i64, ptr %941, , !!49176, !!8                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42893|     ;; y1 = i64 %942
 42894|     ;; self = i64 %942
 42895|     ;; x2 = i64 %841
 42896|     ;; other = i64 %841
 42897|     ;; y2 = i64 %842
 42898|     ;; other = i64 %842
 42899|  %943 = icmp ult i64 %940, %841                                                                                        ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42900|  %944 = sub nuw i64 %841, %940                                                                                         ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42901|  %945 = sub nuw i64 %940, %841                                                                                         ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42902|  %946 = select i1 %943, i64 %944, i64 %945                                                                             ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42903|     ;; dx = i64 %946
 42904|  %947 = icmp ult i64 %942, %842                                                                                        ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42905|  %948 = sub nuw i64 %842, %942                                                                                         ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42906|  %949 = sub nuw i64 %942, %842                                                                                         ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42907|  %950 = select i1 %947, i64 %948, i64 %949                                                                             ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42908|     ;; dy = i64 %950
 42909|  %951 = mul i64 %946, %946                                                                                             ;L9<2158<411<2893<50<2494<138<2897<411<204
 42910|  %952 = mul i64 %950, %950                                                                                             ;L9<2158<411<2893<50<2494<138<2897<411<204
 42911|  %953 = add i64 %952, %951                                                                                             ;L9<2158<411<2893<50<2494<138<2897<411<204
 42912|  %954 = icmp ult i64 %953, 22500000001                                                                                 ;L411<2893<50<2494<138<2897<411<204
 42913|  br i1 %954, label %1010, label %955                                                                                   ;L2494<138<2897<411<204
 42914| 
 42915| 955: ; preds = %938, %935, %934, %926
 42916|     ;; self = ptr undef
 42917|     ;; count = i64 1
 42918|     ;; ptr = !DIArgList(ptr %212, i64 32)
 42919|     ;; self = !DIArgList(ptr %212, i64 32)
 42920|     ;; end_or_len = ptr %213
 42923|  %956 = gep %212, i64 32                                                                                               ;L656<185<2493<138<2897<411<204
 42924|     ;; ptr = ptr %956
 42925|     ;; x = ptr %956
 42926|  %957 = load ptr, ptr %956, , !!49176, !!8                                                                             ;L2494<138<2897<411<204
 42931|  %958 = icmp eq ptr %957, null                                                                                         ;L49<2494<138<2897<411<204
 42932|  br i1 %958, label %984, label %959                                                                                    ;L49<2494<138<2897<411<204
 42933| 
 42934| 959: ; preds = %955
 42935|     ;; x = ptr %957
 42938|     ;; x = ptr %957
 42940|     ;; c = ptr %957
 42941|     ;; self = ptr %957
 42942|     ;; self = ptr %957
 42943|     ;; self = ptr %957
 42944|     ;; other = ptr %209
 42945|     ;; other = ptr %209
 42946|  %960 = load i64, ptr %957, , !!49176, !!8                                                                             ;L1127<264<411<2893<50<2494<138<2897<411<204
 42947|  %961 = gep %957, i64 8                                                                                                ;L1127<264<411<2893<50<2494<138<2897<411<204
 42948|     ;; __self_discr = i64 %960
 42949|     ;; __arg1_discr = i64 %838
 42950|  %962 = icmp eq i64 %960, %838                                                                                         ;L1127<264<411<2893<50<2494<138<2897<411<204
 42951|  br i1 %962, label %963, label %967                                                                                    ;L1127<264<411<2893<50<2494<138<2897<411<204
 42952| 
 42953| 963: ; preds = %959
 42954|  br i1 %843, label %964, label %984                                                                                    ;L1127<264<411<2893<50<2494<138<2897<411<204
 42955| 
 42956| 964: ; preds = %963
 42957|     ;; __self_0 = ptr %957
 42958|     ;; self = ptr %957
 42959|     ;; __arg1_0 = ptr %209
 42960|     ;; other = ptr %209
 42963|  %965 = load i64, ptr %961, , !!49176, !!8                                                                             ;L1878<2123<1127<264<411<2893<50<2494<138<2897<411<204
 42964|  %966 = icmp eq i64 %965, %840                                                                                         ;L1878<2123<1127<264<411<2893<50<2494<138<2897<411<204
 42965|  br i1 %966, label %984, label %967                                                                                    ;L411<2893<50<2494<138<2897<411<204
 42966| 
 42967| 967: ; preds = %964, %959
 42968|     ;; other = ptr %209
 42969|  %968 = gep %957, i64 1632                                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42970|  %969 = load i64, ptr %968, , !!49176, !!8                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42971|     ;; x1 = i64 %969
 42972|     ;; self = i64 %969
 42973|  %970 = gep %957, i64 1640                                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42974|  %971 = load i64, ptr %970, , !!49176, !!8                                                                             ;L2158<411<2893<50<2494<138<2897<411<204
 42975|     ;; y1 = i64 %971
 42976|     ;; self = i64 %971
 42977|     ;; x2 = i64 %841
 42978|     ;; other = i64 %841
 42979|     ;; y2 = i64 %842
 42980|     ;; other = i64 %842
 42981|  %972 = icmp ult i64 %969, %841                                                                                        ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42982|  %973 = sub nuw i64 %841, %969                                                                                         ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42983|  %974 = sub nuw i64 %969, %841                                                                                         ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42984|  %975 = select i1 %972, i64 %973, i64 %974                                                                             ;L3147<7<2158<411<2893<50<2494<138<2897<411<204
 42985|     ;; dx = i64 %975
 42986|  %976 = icmp ult i64 %971, %842                                                                                        ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42987|  %977 = sub nuw i64 %842, %971                                                                                         ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42988|  %978 = sub nuw i64 %971, %842                                                                                         ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42989|  %979 = select i1 %976, i64 %977, i64 %978                                                                             ;L3147<8<2158<411<2893<50<2494<138<2897<411<204
 42990|     ;; dy = i64 %979
 42991|  %980 = mul i64 %975, %975                                                                                             ;L9<2158<411<2893<50<2494<138<2897<411<204
 42992|  %981 = mul i64 %979, %979                                                                                             ;L9<2158<411<2893<50<2494<138<2897<411<204
 42993|  %982 = add i64 %981, %980                                                                                             ;L9<2158<411<2893<50<2494<138<2897<411<204
 42994|  %983 = icmp ult i64 %982, 22500000001                                                                                 ;L411<2893<50<2494<138<2897<411<204
 42995|  br i1 %983, label %1010, label %984                                                                                   ;L2494<138<2897<411<204
 42996| 
 42997| 984: ; preds = %967, %964, %963, %955
 42998|     ;; self = ptr undef
 42999|     ;; count = i64 1
 43000|     ;; ptr = !DIArgList(ptr %212, i64 40)
 43001|     ;; self = !DIArgList(ptr %212, i64 40)
 43002|     ;; end_or_len = ptr %213
 43007|     ;; self = ptr %573
 43008|     ;; f = ptr %209
 43009|     ;; self = ptr undef
 43010|     ;; self = ptr undef
 43011|     ;; count = i64 1
 43012|  br label %988                                                                                                         ;L331<412<204
 43013| 
 43014| 985: ; preds = %850
 43015|     ;; __self_0 = ptr %844
 43016|     ;; self = ptr %844
 43017|     ;; __arg1_0 = ptr %209
 43018|     ;; other = ptr %209
 43021|  %986 = load i64, ptr %848, , !!49176, !!8                                                                             ;L1878<2123<1127<264<411<2893<50<2494<138<2897<411<204
 43022|  %987 = icmp eq i64 %986, %840                                                                                         ;L1878<2123<1127<264<411<2893<50<2494<138<2897<411<204
 43023|  br i1 %987, label %868, label %851                                                                                    ;L411<2893<50<2494<138<2897<411<204
 43024| 
 43025| 988: ; preds = %991, %984
 43026|  %989 = phi ptr [ %992, %991 ], [ %573, %984 ]
 43027|     ;; ptr = ptr %989
 43028|     ;; self = ptr %989
 43029|     ;; end_or_len = ptr %576
 43032|  %990 = icmp eq ptr %989, %576                                                                                         ;L1714<180<331<412<204
 43033|  br i1 %990, label %655, label %991                                                                                    ;L180<331<412<204
 43034| 
 43035| 991: ; preds = %988
 43036|  %992 = gep %989, i64 8                                                                                                ;L656<185<331<412<204
 43037|     ;; x = ptr %989
 43038|  %993 = load ptr, ptr %989, , !!49313, !!8, !!8                                                                        ;L332<412<204
 43041|     ;; self = ptr %993
 43042|     ;; other = ptr %209
 43043|  %994 = gep %993, i64 1632                                                                                             ;L2158<412<332<412<204
 43044|  %995 = load i64, ptr %994, , !!49313, !!8                                                                             ;L2158<412<332<412<204
 43045|     ;; x1 = i64 %995
 43046|     ;; self = i64 %995
 43047|  %996 = gep %993, i64 1640                                                                                             ;L2158<412<332<412<204
 43048|  %997 = load i64, ptr %996, , !!49313, !!8                                                                             ;L2158<412<332<412<204
 43049|     ;; y1 = i64 %997
 43050|     ;; self = i64 %997
 43051|     ;; x2 = i64 %841
 43052|     ;; other = i64 %841
 43053|     ;; y2 = i64 %842
 43054|     ;; other = i64 %842
 43055|  %998 = icmp ult i64 %995, %841                                                                                        ;L3147<7<2158<412<332<412<204
 43056|  %999 = sub nuw i64 %841, %995                                                                                         ;L3147<7<2158<412<332<412<204
 43057|  %1000 = sub nuw i64 %995, %841                                                                                        ;L3147<7<2158<412<332<412<204
 43058|  %1001 = select i1 %998, i64 %999, i64 %1000                                                                           ;L3147<7<2158<412<332<412<204
 43059|     ;; dx = i64 %1001
 43060|  %1002 = icmp ult i64 %997, %842                                                                                       ;L3147<8<2158<412<332<412<204
 43061|  %1003 = sub nuw i64 %842, %997                                                                                        ;L3147<8<2158<412<332<412<204
 43062|  %1004 = sub nuw i64 %997, %842                                                                                        ;L3147<8<2158<412<332<412<204
 43063|  %1005 = select i1 %1002, i64 %1003, i64 %1004                                                                         ;L3147<8<2158<412<332<412<204
 43064|     ;; dy = i64 %1005
 43065|  %1006 = mul i64 %1001, %1001                                                                                          ;L9<2158<412<332<412<204
 43066|  %1007 = mul i64 %1005, %1005                                                                                          ;L9<2158<412<332<412<204
 43067|  %1008 = add i64 %1007, %1006                                                                                          ;L9<2158<412<332<412<204
 43068|  %1009 = icmp ult i64 %1008, 22500000001                                                                               ;L412<332<412<204
 43069|  br i1 %1009, label %1010, label %988                                                                                  ;L332<412<204
 43070| 
 43071| 1010: ; preds = %991, %967, %938, %909, %880, %851
 43074|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway3new(ptr sret([136 x i8]) %14, ptr %5, ptr %4, i64 5)
 43075|  to label %1011 unwind label %288, !!48170                                                                             ;L414<204
 43076| 
 43077| 1011: ; preds = %1010
 43078|  call void @llvm.memcpy.p0.p0.i64(ptr %15, ptr %14, i64 136, i1 false), !!48147                                        ;L414<204
 43079|  %1012 = gep %15, i64 177                                                                                              ;L414<204
 43080|  store i8 3, ptr %1012, , !!48147                                                                                      ;L414<204
 43083|     ;; self = ptr %39
 43084|     ;; self = ptr %39
 43085|     ;; value = ptr %15
 43086|     ;; src = ptr %15
 43087|     ;; additional = i64 1
 43088|     ;; needed_extra_cap = i64 1
 43089|     ;; needed_extra_cap = i64 1
 43090|     ;; strategy = i8 1
 43091|  %1013 = load i64, ptr %208, , !!49375, !!8                                                                            ;L1428<414<204
 43092|     ;; self = ptr %39
 43093|  %1014 = load i64, ptr %207, , !!49375, !!8                                                                            ;L149<1428<414<204
 43094|  %1015 = icmp eq i64 %1013, %1014                                                                                      ;L1428<414<204
 43095|  br i1 %1015, label %1016, label %1021                                                                                 ;L1428<414<204
 43096| 
 43097| 1016: ; preds = %1011
 43098|     ;; self = ptr %39
 43099|     ;; self = ptr %39
 43100|     ;; self = ptr %39
 43101|     ;; used_cap = i64 %1013
 43102|     ;; used_cap = i64 %1013
 43103|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %39, i64 %1013, i64 1, i1 zeroext true)
 43104|  to label %1017 unwind label %1019, !!49383                                                                            ;L619<430<738<1429<414<204
 43105| 
 43106| 1017: ; preds = %1016
 43107|  %1018 = load i64, ptr %208, , !!49375                                                                                 ;L1432<414<204
 43108|  br label %1021                                                                                                        ;L619<430<738<1429<414<204
 43109| 
 43110| 1019: ; preds = %1016
 43111|  %1020 = cleanuppad within none []
 43112|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %15) #30 [ "funclet"(token %1020) ], !!49386 ;L1436<414<204
 43113|  cleanupret from %1020 unwind label %288
 43114| 
 43115| 1021: ; preds = %1017, %1011
 43116|  %1022 = phi i64 [ %1018, %1017 ], [ %1013, %1011 ]                                                                    ;L1432<414<204
 43117|     ;; self = ptr %39
 43118|  %1023 = load ptr, ptr %39, , !!49375, !!8, !!8                                                                        ;L138<1432<414<204
 43119|     ;; self = ptr %1023
 43120|     ;; count = i64 %1022
 43121|  %1024 = gepS %1023, i64 %1022                                                                                         ;L961<1432<414<204
 43122|     ;; end = ptr %1024
 43123|     ;; dst = ptr %1024
 43124|  call void @llvm.memcpy.p0.p0.i64(ptr %1024, ptr %15, i64 184, i1 false), !!49386                                      ;L1933<1433<414<204
 43125|  %1025 = add i64 %1022, 1                                                                                              ;L1434<414<204
 43126|  store i64 %1025, ptr %208, , !!49375                                                                                  ;L1434<414<204
 43128|  br label %655                                                                                                         ;L413<204
 43129| 
 43130| 1026: ; preds = %668
 43131|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.204) #31
 43132|  to label %201 unwind label %288, !!48170                                                                              ;L1013<381<204
 43133| 
 43134| 1027: ; preds = %1073, %655
 43135|  call void @llvm.memcpy.p0.p0.i64(ptr %58, ptr %39, i64 32, i1 false), !!49399                                         ;L423<204
 43137|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %35)
 43138|  to label %1046 unwind label %1028, !!48170                                                                            ;L825<424<204
 43139| 
 43140| 1028: ; preds = %1027
 43141|  %1029 = cleanuppad within none []
 43145|     ;; self = ptr %35
 43147|     ;; self = ptr %35
 43148|     ;; self = ptr %35
 43149|     ;; elem_size = i64 8
 43150|     ;; align = i64 8
 43151|  %1030 = gep %35, i64 16                                                                                               ;L159<703<714<825<825<424<204
 43152|  %1031 = load i64, ptr %1030, , !!48147, !!8                                                                           ;L159<703<714<825<825<424<204
 43153|  %1032 = icmp eq i64 %1031, 0                                                                                          ;L159<703<714<825<825<424<204
 43154|  br i1 %1032, label %1045, label %1033                                                                                 ;L159<703<714<825<825<424<204
 43155| 
 43156| 1033: ; preds = %1028
 43157|     ;; layout[0..+8] = i64 8
 43158|     ;; layout[8..+8] = i64 %1031
 43159|  %1034 = gep %35, i64 8                                                                                                ;L704<714<825<825<424<204
 43160|  %1035 = load ptr, ptr %1034, , !!48147, !!8, !!8                                                                      ;L704<714<825<825<424<204
 43161|  %1036 = load ptr, ptr %35, , !!48147, !!8, !!8                                                                        ;L704<714<825<825<424<204
 43162|  %1037 = gep %1035, i64 16                                                                                             ;L704<714<825<825<424<204
 43163|  %1038 = load ptr, ptr %1037, , !!49430, !!8, !!8                                                                      ;L704<714<825<825<424<204
 43166|     ;; ptr = ptr %1036
 43167|     ;; ptr = ptr %1036
 43168|     ;; layout[0..+8] = i64 8
 43169|     ;; layout[8..+8] = i64 %1031
 43170|     ;; footer = ptr %1038
 43171|     ;; footer = ptr %1038
 43172|     ;; self = ptr %1038
 43173|  %1039 = gep %1038, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<424<204
 43174|  %1040 = load ptr, ptr %1039, , !!49430, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<424<204
 43177|     ;; self = ptr %1036
 43178|  %1041 = icmp eq ptr %1040, %1036                                                                                      ;L1714<1700<1707<704<714<825<825<424<204
 43179|  br i1 %1041, label %1042, label %1045                                                                                 ;L1707<704<714<825<825<424<204
 43180| 
 43181| 1042: ; preds = %1033
 43182|  %1043 = shl i64 %1031, 3                                                                                              ;L166<703<714<825<825<424<204
 43183|     ;; layout[8..+8] = i64 %1043
 43184|     ;; layout[8..+8] = i64 %1043
 43185|     ;; count = i64 %1043
 43186|  %1044 = gep %1036, i64 %1043                                                                                          ;L961<1708<704<714<825<825<424<204
 43187|     ;; ptr = ptr %1044
 43188|     ;; val = ptr %1044
 43189|     ;; val = ptr %1044
 43190|     ;; self = ptr %1038
 43191|     ;; self = ptr %1038
 43192|  store ptr %1044, ptr %1039, , !!49430                                                                                 ;L931<513<437<1709<704<714<825<825<424<204
 43193|  br label %1045                                                                                                        ;L1707<704<714<825<825<424<204
 43194| 
 43195| 1045: ; preds = %1042, %1033, %1028
 43196|  cleanupret from %1029 unwind label %198
 43197| 
 43198| 1046: ; preds = %1027
 43202|     ;; self = ptr %35
 43204|     ;; self = ptr %35
 43205|     ;; self = ptr %35
 43206|     ;; elem_size = i64 8
 43207|     ;; align = i64 8
 43208|  %1047 = gep %35, i64 16                                                                                               ;L159<703<714<825<825<424<204
 43209|  %1048 = load i64, ptr %1047, , !!48147, !!8                                                                           ;L159<703<714<825<825<424<204
 43210|  %1049 = icmp eq i64 %1048, 0                                                                                          ;L159<703<714<825<825<424<204
 43211|  br i1 %1049, label %1078, label %1050                                                                                 ;L159<703<714<825<825<424<204
 43212| 
 43213| 1050: ; preds = %1046
 43214|     ;; layout[0..+8] = i64 8
 43215|     ;; layout[8..+8] = i64 %1048
 43216|  %1051 = gep %35, i64 8                                                                                                ;L704<714<825<825<424<204
 43217|  %1052 = load ptr, ptr %1051, , !!48147, !!8, !!8                                                                      ;L704<714<825<825<424<204
 43218|  %1053 = load ptr, ptr %35, , !!48147, !!8, !!8                                                                        ;L704<714<825<825<424<204
 43219|  %1054 = gep %1052, i64 16                                                                                             ;L704<714<825<825<424<204
 43220|  %1055 = load ptr, ptr %1054, , !!49483, !!8, !!8                                                                      ;L704<714<825<825<424<204
 43223|     ;; ptr = ptr %1053
 43224|     ;; ptr = ptr %1053
 43225|     ;; layout[0..+8] = i64 8
 43226|     ;; layout[8..+8] = i64 %1048
 43227|     ;; footer = ptr %1055
 43228|     ;; footer = ptr %1055
 43229|     ;; self = ptr %1055
 43230|  %1056 = gep %1055, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<424<204
 43231|  %1057 = load ptr, ptr %1056, , !!49483, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<424<204
 43234|     ;; self = ptr %1053
 43235|  %1058 = icmp eq ptr %1057, %1053                                                                                      ;L1714<1700<1707<704<714<825<825<424<204
 43236|  br i1 %1058, label %1059, label %1078                                                                                 ;L1707<704<714<825<825<424<204
 43237| 
 43238| 1059: ; preds = %1050
 43239|  %1060 = shl i64 %1048, 3                                                                                              ;L166<703<714<825<825<424<204
 43240|     ;; layout[8..+8] = i64 %1060
 43241|     ;; layout[8..+8] = i64 %1060
 43242|     ;; count = i64 %1060
 43243|  %1061 = gep %1053, i64 %1060                                                                                          ;L961<1708<704<714<825<825<424<204
 43244|     ;; ptr = ptr %1061
 43245|     ;; val = ptr %1061
 43246|     ;; val = ptr %1061
 43247|     ;; self = ptr %1055
 43248|     ;; self = ptr %1055
 43249|  store ptr %1061, ptr %1056, , !!49483                                                                                 ;L931<513<437<1709<704<714<825<825<424<204
 43250|  br label %1078                                                                                                        ;L1707<704<714<825<825<424<204
 43251| 
 43252| 1062: ; preds = %655
 43255|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %12, ptr %5, ptr %4, i64 5, i1 zeroext false)
 43256|  to label %1063 unwind label %288, !!48170                                                                             ;L420<204
 43257| 
 43258| 1063: ; preds = %1062
 43259|  call void @llvm.memcpy.p0.p0.i64(ptr %13, ptr %12, i64 136, i1 false), !!48147                                        ;L420<204
 43260|  %1064 = gep %13, i64 177                                                                                              ;L420<204
 43261|  store i8 3, ptr %1064, , !!48147                                                                                      ;L420<204
 43264|     ;; self = ptr %39
 43265|     ;; self = ptr %39
 43266|     ;; value = ptr %13
 43267|     ;; src = ptr %13
 43268|     ;; additional = i64 1
 43269|     ;; needed_extra_cap = i64 1
 43270|     ;; needed_extra_cap = i64 1
 43271|     ;; strategy = i8 1
 43272|  %1065 = load i64, ptr %208, , !!49527, !!8                                                                            ;L1428<420<204
 43273|     ;; self = ptr %39
 43274|  %1066 = load i64, ptr %207, , !!49527, !!8                                                                            ;L149<1428<420<204
 43275|  %1067 = icmp eq i64 %1065, %1066                                                                                      ;L1428<420<204
 43276|  br i1 %1067, label %1068, label %1073                                                                                 ;L1428<420<204
 43277| 
 43278| 1068: ; preds = %1063
 43279|     ;; self = ptr %39
 43280|     ;; self = ptr %39
 43281|     ;; self = ptr %39
 43282|     ;; used_cap = i64 %1065
 43283|     ;; used_cap = i64 %1065
 43284|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %39, i64 %1065, i64 1, i1 zeroext true)
 43285|  to label %1069 unwind label %1071, !!49535                                                                            ;L619<430<738<1429<420<204
 43286| 
 43287| 1069: ; preds = %1068
 43288|  %1070 = load i64, ptr %208, , !!49527                                                                                 ;L1432<420<204
 43289|  br label %1073                                                                                                        ;L619<430<738<1429<420<204
 43290| 
 43291| 1071: ; preds = %1068
 43292|  %1072 = cleanuppad within none []
 43293|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %13) #30 [ "funclet"(token %1072) ], !!49538 ;L1436<420<204
 43294|  cleanupret from %1072 unwind label %288
 43295| 
 43296| 1073: ; preds = %1069, %1063
 43297|  %1074 = phi i64 [ %1070, %1069 ], [ %1065, %1063 ]                                                                    ;L1432<420<204
 43298|     ;; self = ptr %39
 43299|  %1075 = load ptr, ptr %39, , !!49527, !!8, !!8                                                                        ;L138<1432<420<204
 43300|     ;; self = ptr %1075
 43301|     ;; count = i64 %1074
 43302|  %1076 = gepS %1075, i64 %1074                                                                                         ;L961<1432<420<204
 43303|     ;; end = ptr %1076
 43304|     ;; dst = ptr %1076
 43305|  call void @llvm.memcpy.p0.p0.i64(ptr %1076, ptr %13, i64 184, i1 false), !!49538                                      ;L1933<1433<420<204
 43306|  %1077 = add i64 %1074, 1                                                                                              ;L1434<420<204
 43307|  store i64 %1077, ptr %208, , !!49527                                                                                  ;L1434<420<204
 43309|  br label %1027                                                                                                        ;L419<204
 43310| 
 43311| 1078: ; preds = %1059, %1050, %1046
 43313|  br label %1091                                                                                                        ;L424<204
 43314| 
 43315| 1079: ; preds = %300
 43316|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.205) #31
 43317|  to label %201 unwind label %288, !!48170                                                                              ;L1013<321<204
 43318| 
 43319| 1080: ; preds = %279
 43320|  call void @llvm.memcpy.p0.p0.i64(ptr %37, ptr %36, i64 136, i1 false), !!48147                                        ;L301<204
 43321|  %1081 = gep %37, i64 177                                                                                              ;L301<204
 43322|  store i8 3, ptr %1081, , !!48147                                                                                      ;L301<204
 43325|     ;; self = ptr %39
 43326|     ;; self = ptr %39
 43327|     ;; value = ptr %37
 43328|     ;; src = ptr %37
 43329|     ;; additional = i64 1
 43330|     ;; needed_extra_cap = i64 1
 43331|     ;; needed_extra_cap = i64 1
 43332|     ;; strategy = i8 1
 43333|     ;; self = ptr %39
 43334|     ;; self = ptr %39
 43335|     ;; self = ptr %39
 43336|     ;; self = ptr %39
 43337|     ;; used_cap = i64 0
 43338|     ;; used_cap = i64 0
 43339|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %39, i64 0, i64 1, i1 zeroext true)
 43340|  to label %1084 unwind label %1082, !!49568                                                                            ;L619<430<738<1429<301<204
 43341| 
 43342| 1082: ; preds = %1080
 43343|  %1083 = cleanuppad within none []
 43344|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %37) #30 [ "funclet"(token %1083) ], !!49571 ;L1436<301<204
 43345|  cleanupret from %1083 unwind label %198
 43346| 
 43347| 1084: ; preds = %1080
 43348|  %1085 = load ptr, ptr %39, , !!49574                                                                                  ;L138<1432<301<204
 43349|  %1086 = load i64, ptr %208, , !!49574                                                                                 ;L1432<301<204
 43350|     ;; self = ptr %39
 43351|     ;; self = ptr %1085
 43352|     ;; count = i64 %1086
 43353|  %1087 = gepS %1085, i64 %1086                                                                                         ;L961<1432<301<204
 43354|     ;; end = ptr %1087
 43355|     ;; dst = ptr %1087
 43356|  call void @llvm.memcpy.p0.p0.i64(ptr %1087, ptr %37, i64 184, i1 false), !!49571                                      ;L1933<1433<301<204
 43357|  %1088 = add i64 %1086, 1                                                                                              ;L1434<301<204
 43358|  store i64 %1088, ptr %208, , !!49574                                                                                  ;L1434<301<204
 43360|  call void @llvm.memcpy.p0.p0.i64(ptr %58, ptr %39, i64 32, i1 false), !!49399                                         ;L302<204
 43361|  br label %1091                                                                                                        ;L424<204
 43362| 
 43363| 1089: ; preds = %1090, %198
 43364|  cleanupret from %200 unwind label %188
 43365| 
 43366| 1090: ; preds = %198
 43367|  invoke fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %39) #30 [ "funclet"(token %200) ]
 43368|  to label %1089 unwind label %188                                                                                      ;L424<204
 43369| 
 43370| 1091: ; preds = %1084, %1078
 43373|  %1092 = load ptr, ptr %64, , !!8                                                                                      ;L207
 43374|  %1093 = icmp eq ptr %1092, null                                                                                       ;L207
 43375|  br i1 %1093, label %1098, label %1094                                                                                 ;L207
 43376| 
 43377| 1094: ; preds = %1091
 43378|     ;; nearest_tower = ptr %1092
 43379|  %1095 = gep %1092, i64 104                                                                                            ;L208
 43380|  %1096 = load i64, ptr %1095, , !!8                                                                                    ;L208
 43381|  %1097 = icmp eq i64 %1096, 2                                                                                          ;L208
 43382|  br i1 %1097, label %1102, label %1098                                                                                 ;L208
 43383| 
 43384| 1098: ; preds = %1129, %1108, %1102, %1094, %1091
 43385|     ;; self = ptr %62
 43386|     ;; self = ptr %62
 43387|  %1099 = gep %62, i64 24                                                                                               ;L1617<1636<217
 43388|  %1100 = load i64, ptr %1099, , !!8                                                                                    ;L1617<1636<217
 43389|  %1101 = icmp eq i64 %1100, 0                                                                                          ;L217
 43390|  br i1 %1101, label %1134, label %1137                                                                                 ;L217
 43391| 
 43392| 1102: ; preds = %1094
 43393|     ;; info = ptr %1092
 43394|  %1103 = gep %1092, i64 136                                                                                            ;L209
 43395|  %1104 = load i64, ptr %1103,                                                                                          ;L209
 43396|     ;; self[0..+8] = i64 %1104
 43399|  %1105 = trunc nuw i64 %1104 to i1                                                                                     ;L1161<209
 43400|  br i1 %1105, label %1108, label %1098                                                                                 ;L1161<209
 43401| 
 43402| 1106: ; preds = %1506, %1433, %1396, %1393, %1391, %1390, %1389, %1379, %1373, %1369, %1366, %1365, %1276, %1272, %1269, %1265, %1263, %1259, %1235, %1202, %1197, %1195, %1191, %1186, %1134, %1127, %1116, %1115, %1114
 43403|  %1107 = cleanuppad within none []
 43404|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %58) #30 [ "funclet"(token %1107) ] ;L275
 43405|  cleanupret from %1107 unwind label %188                                                                               ;L275
 43406| 
 43407| 1108: ; preds = %1102
 43408|  %1109 = gep %1092, i64 152                                                                                            ;L209
 43409|  %1110 = load i64, ptr %1109,                                                                                          ;L209
 43410|     ;; self[16..+8] = i64 %1110
 43412|     ;; self = ptr undef
 43413|  %1111 = gep %93, i64 1472                                                                                             ;L209
 43414|  %1112 = load i64, ptr %1111, , !!8                                                                                    ;L209
 43416|     ;; l = ptr undef
 43417|     ;; self = ptr undef
 43420|  %1113 = icmp eq i64 %1110, %1112                                                                                      ;L1878<2440<209
 43421|  br i1 %1113, label %1114, label %1098                                                                                 ;L209
 43422| 
 43423| 1114: ; preds = %1108
 43424|     ;; self = ptr %62
 43425|  invoke void @ai::small_action15SmallActionPlayE8truncateBX_(ptr %62, i64 0)
 43426|  to label %1115 unwind label %1106                                                                                     ;L1599<210
 43427| 
 43428| 1115: ; preds = %1114
 43429|     ;; self = ptr %58
 43430|  invoke void @ai::small_action15SmallActionPlayE8truncateBX_(ptr %58, i64 0)
 43431|  to label %1116 unwind label %1106                                                                                     ;L1599<211
 43432| 
 43433| 1116: ; preds = %1115
 43436|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %56, ptr %5, ptr %4, i64 5, i1 zeroext true)
 43437|  to label %1117 unwind label %1106                                                                                     ;L212
 43438| 
 43439| 1117: ; preds = %1116
 43440|  call void @llvm.memcpy.p0.p0.i64(ptr %57, ptr %56, i64 136, i1 false)                                                 ;L212
 43441|  %1118 = gep %57, i64 177                                                                                              ;L212
 43442|  store i8 3, ptr %1118,                                                                                                ;L212
 43445|     ;; self = ptr %58
 43446|     ;; self = ptr %58
 43447|     ;; value = ptr %57
 43448|     ;; src = ptr %57
 43449|     ;; additional = i64 1
 43450|     ;; needed_extra_cap = i64 1
 43451|     ;; needed_extra_cap = i64 1
 43452|     ;; strategy = i8 1
 43453|  %1119 = gep %58, i64 24                                                                                               ;L1428<212
 43454|  %1120 = load i64, ptr %1119, , !!49633, !!8                                                                           ;L1428<212
 43455|     ;; self = ptr %58
 43456|  %1121 = gep %58, i64 16                                                                                               ;L149<1428<212
 43457|  %1122 = load i64, ptr %1121, , !!49633, !!8                                                                           ;L149<1428<212
 43458|  %1123 = icmp eq i64 %1120, %1122                                                                                      ;L1428<212
 43459|  br i1 %1123, label %1124, label %1129                                                                                 ;L1428<212
 43460| 
 43461| 1124: ; preds = %1117
 43462|     ;; self = ptr %58
 43463|     ;; self = ptr %58
 43464|     ;; self = ptr %58
 43465|     ;; used_cap = i64 %1120
 43466|     ;; used_cap = i64 %1120
 43467|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %58, i64 %1120, i64 1, i1 zeroext true)
 43468|  to label %1125 unwind label %1127, !!49633                                                                            ;L619<430<738<1429<212
 43469| 
 43470| 1125: ; preds = %1124
 43471|  %1126 = load i64, ptr %1119, , !!49633                                                                                ;L1432<212
 43472|  br label %1129                                                                                                        ;L619<430<738<1429<212
 43473| 
 43474| 1127: ; preds = %1124
 43475|  %1128 = cleanuppad within none []
 43476|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %57) #30 [ "funclet"(token %1128) ], !!49618 ;L1436<212
 43477|  cleanupret from %1128 unwind label %1106
 43478| 
 43479| 1129: ; preds = %1125, %1117
 43480|  %1130 = phi i64 [ %1126, %1125 ], [ %1120, %1117 ]                                                                    ;L1432<212
 43481|     ;; self = ptr %58
 43482|  %1131 = load ptr, ptr %58, , !!49633, !!8, !!8                                                                        ;L138<1432<212
 43483|     ;; self = ptr %1131
 43484|     ;; count = i64 %1130
 43485|  %1132 = gepS %1131, i64 %1130                                                                                         ;L961<1432<212
 43486|     ;; end = ptr %1132
 43487|     ;; dst = ptr %1132
 43488|  call void @llvm.memcpy.p0.p0.i64(ptr %1132, ptr %57, i64 184, i1 false), !!49618                                      ;L1933<1433<212
 43489|  %1133 = add i64 %1130, 1                                                                                              ;L1434<212
 43490|  store i64 %1133, ptr %1119, , !!49633                                                                                 ;L1434<212
 43492|  br label %1098                                                                                                        ;L209
 43493| 
 43494| 1134: ; preds = %1098
 43495|  %1135 = gep %4, i64 384                                                                                               ;L218
 43496|  %1136 = invoke i64 @gc::simulation5state6playerNtB4_16AthleteParameter20positioning_accuracy(ptr %1135)
 43497|  to label %1172 unwind label %1106                                                                                     ;L218
 43498| 
 43499| 1137: ; preds = %1098
 43500|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %62, i64 32, i1 false)                                                   ;L273
 43502|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %58)
 43503|  to label %1156 unwind label %1138                                                                                     ;L825<275
 43504| 
 43505| 1138: ; preds = %1137
 43506|  %1139 = cleanuppad within none []
 43510|     ;; self = ptr %58
 43512|     ;; self = ptr %58
 43513|     ;; self = ptr %58
 43514|     ;; elem_size = i64 184
 43515|     ;; align = i64 8
 43516|  %1140 = gep %58, i64 16                                                                                               ;L159<703<714<825<825<275
 43517|  %1141 = load i64, ptr %1140, , !!8                                                                                    ;L159<703<714<825<825<275
 43518|  %1142 = icmp eq i64 %1141, 0                                                                                          ;L159<703<714<825<825<275
 43519|  br i1 %1142, label %1155, label %1143                                                                                 ;L159<703<714<825<825<275
 43520| 
 43521| 1143: ; preds = %1138
 43522|     ;; layout[0..+8] = i64 8
 43523|     ;; layout[8..+8] = i64 %1141
 43524|  %1144 = gep %58, i64 8                                                                                                ;L704<714<825<825<275
 43525|  %1145 = load ptr, ptr %1144, , !!8, !!8                                                                               ;L704<714<825<825<275
 43526|  %1146 = load ptr, ptr %58, , !!8, !!8                                                                                 ;L704<714<825<825<275
 43527|  %1147 = gep %1145, i64 16                                                                                             ;L704<714<825<825<275
 43528|  %1148 = load ptr, ptr %1147, , !!49684, !!8, !!8                                                                      ;L704<714<825<825<275
 43531|     ;; ptr = ptr %1146
 43532|     ;; ptr = ptr %1146
 43533|     ;; layout[0..+8] = i64 8
 43534|     ;; layout[8..+8] = i64 %1141
 43535|     ;; footer = ptr %1148
 43536|     ;; footer = ptr %1148
 43537|     ;; self = ptr %1148
 43538|  %1149 = gep %1148, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<275
 43539|  %1150 = load ptr, ptr %1149, , !!49684, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<275
 43542|     ;; self = ptr %1146
 43543|  %1151 = icmp eq ptr %1150, %1146                                                                                      ;L1714<1700<1707<704<714<825<825<275
 43544|  br i1 %1151, label %1152, label %1155                                                                                 ;L1707<704<714<825<825<275
 43545| 
 43546| 1152: ; preds = %1143
 43547|  %1153 = mul i64 %1141, 184                                                                                            ;L166<703<714<825<825<275
 43548|     ;; layout[8..+8] = i64 %1153
 43549|     ;; layout[8..+8] = i64 %1153
 43550|     ;; count = i64 %1153
 43551|  %1154 = gep %1146, i64 %1153                                                                                          ;L961<1708<704<714<825<825<275
 43552|     ;; ptr = ptr %1154
 43553|     ;; val = ptr %1154
 43554|     ;; val = ptr %1154
 43555|     ;; self = ptr %1148
 43556|     ;; self = ptr %1148
 43557|  store ptr %1154, ptr %1149, , !!49684                                                                                 ;L931<513<437<1709<704<714<825<825<275
 43558|  br label %1155                                                                                                        ;L1707<704<714<825<825<275
 43559| 
 43560| 1155: ; preds = %1152, %1143, %1138
 43561|  cleanupret from %1139 unwind label %188
 43562| 
 43563| 1156: ; preds = %1137
 43567|     ;; self = ptr %58
 43569|     ;; self = ptr %58
 43570|     ;; self = ptr %58
 43571|     ;; elem_size = i64 184
 43572|     ;; align = i64 8
 43573|  %1157 = gep %58, i64 16                                                                                               ;L159<703<714<825<825<275
 43574|  %1158 = load i64, ptr %1157, , !!8                                                                                    ;L159<703<714<825<825<275
 43575|  %1159 = icmp eq i64 %1158, 0                                                                                          ;L159<703<714<825<825<275
 43576|  br i1 %1159, label %1614, label %1160                                                                                 ;L159<703<714<825<825<275
 43577| 
 43578| 1160: ; preds = %1156
 43579|     ;; layout[0..+8] = i64 8
 43580|     ;; layout[8..+8] = i64 %1158
 43581|  %1161 = gep %58, i64 8                                                                                                ;L704<714<825<825<275
 43582|  %1162 = load ptr, ptr %1161, , !!8, !!8                                                                               ;L704<714<825<825<275
 43583|  %1163 = load ptr, ptr %58, , !!8, !!8                                                                                 ;L704<714<825<825<275
 43584|  %1164 = gep %1162, i64 16                                                                                             ;L704<714<825<825<275
 43585|  %1165 = load ptr, ptr %1164, , !!49737, !!8, !!8                                                                      ;L704<714<825<825<275
 43588|     ;; ptr = ptr %1163
 43589|     ;; ptr = ptr %1163
 43590|     ;; layout[0..+8] = i64 8
 43591|     ;; layout[8..+8] = i64 %1158
 43592|     ;; footer = ptr %1165
 43593|     ;; footer = ptr %1165
 43594|     ;; self = ptr %1165
 43595|  %1166 = gep %1165, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<275
 43596|  %1167 = load ptr, ptr %1166, , !!49737, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<275
 43599|     ;; self = ptr %1163
 43600|  %1168 = icmp eq ptr %1167, %1163                                                                                      ;L1714<1700<1707<704<714<825<825<275
 43601|  br i1 %1168, label %1169, label %1614                                                                                 ;L1707<704<714<825<825<275
 43602| 
 43603| 1169: ; preds = %1160
 43604|  %1170 = mul i64 %1158, 184                                                                                            ;L166<703<714<825<825<275
 43605|     ;; layout[8..+8] = i64 %1170
 43606|     ;; layout[8..+8] = i64 %1170
 43607|     ;; count = i64 %1170
 43608|  %1171 = gep %1163, i64 %1170                                                                                          ;L961<1708<704<714<825<825<275
 43609|     ;; ptr = ptr %1171
 43610|     ;; val = ptr %1171
 43611|     ;; val = ptr %1171
 43612|     ;; self = ptr %1165
 43613|     ;; self = ptr %1165
 43614|  store ptr %1171, ptr %1166, , !!49737                                                                                 ;L931<513<437<1709<704<714<825<825<275
 43615|  br label %1614                                                                                                        ;L1707<704<714<825<825<275
 43616| 
 43617| 1172: ; preds = %1134
 43618|     ;; positioning_accuracy = i64 %1136
 43619|     ;; min_v = i64 %1136
 43620|  %1173 = sub i64 2000, %1136                                                                                           ;L220
 43621|     ;; max_v = i64 %1173
 43622|     ;; self = ptr undef
 43623|     ;; self = ptr undef
 43624|     ;; f[0..+8] = ptr %68
 43625|     ;; f[8..+8] = ptr %5
 43626|     ;; f[16..+8] = ptr %4
 43627|     ;; f[24..+8] = ptr %93
 43628|     ;; f[32..+8] = ptr %3
 43629|     ;; f[40..+8] = ptr undef
 43630|     ;; f[48..+8] = ptr undef
 43631|     ;; fold[0..+8] = ptr %68
 43632|     ;; fold[8..+8] = ptr %5
 43633|     ;; fold[16..+8] = ptr %4
 43634|     ;; fold[24..+8] = ptr %93
 43635|     ;; fold[32..+8] = ptr %3
 43636|     ;; fold[40..+8] = ptr undef
 43637|     ;; fold[48..+8] = ptr undef
 43640|     ;; f[8..+8] = ptr %68
 43641|     ;; f[16..+8] = ptr %5
 43642|     ;; f[24..+8] = ptr %4
 43643|     ;; f[32..+8] = ptr %93
 43644|     ;; f[40..+8] = ptr %3
 43645|     ;; f[48..+8] = ptr undef
 43646|     ;; f[56..+8] = ptr undef
 43647|     ;; self = ptr undef
 43650|     ;; self = ptr undef
 43651|     ;; count = i64 1
 43652|     ;; ptr = ptr %212
 43653|     ;; self = ptr %212
 43654|     ;; end_or_len = ptr %213
 43657|  %1174 = gep %11, i64 8
 43658|  %1175 = gep %5, i64 16
 43659|  %1176 = gep %93, i64 1632
 43660|  %1177 = gep %93, i64 1640
 43661|  %1178 = load ptr, ptr %1175, , !!8
 43662|  %1179 = getelementptr { { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, [5 x { i64, [2 x i64] }], [5 x { i64, { i8, [23 x i8] } }], [5 x { i8, [7 x i8], i64 }], [5 x i64], [5 x i64], [5 x i64], [5 x { i64, i64 }], i64, i64, i64, i64, i64, [5 x i32], i8, [3 x i8] }, ptr %1178, i64 %96
 43663|  br label %1180                                                                                                        ;L180<2493<138<2897<224
 43664| 
 43665| 1180: ; preds = %1240, %1172
 43666|  %1181 = phi i64 [ 0, %1172 ], [ %1183, %1240 ]
 43667|  %1182 = gep %212, i64 %1181                                                                                           ;L656<185<2493<138<2897<224
 43668|     ;; ptr = ptr %1182
 43669|  %1183 = add nuw nsw i64 %1181, 8                                                                                      ;L656<185<2493<138<2897<224
 43670|     ;; x = ptr %1182
 43671|  %1184 = load ptr, ptr %1182, , !!49796, !!8                                                                           ;L2494<138<2897<224
 43672|     ;; f = ptr undef
 43676|  %1185 = icmp eq ptr %1184, null                                                                                       ;L49<2494<138<2897<224
 43677|  br i1 %1185, label %1240, label %1186                                                                                 ;L49<2494<138<2897<224
 43678| 
 43679| 1186: ; preds = %1180
 43680|     ;; x = ptr %1184
 43684|     ;; x = ptr %1184
 43693|     ;; c = ptr %1184
 43694|     ;; self = ptr %1184
 43695|     ;; self = ptr %1184
 43696|     ;; jrng = ptr %11
 43698|  %1187 = load i64, ptr %68, , !!49859, !!8                                                                             ;L225<2893<50<2494<138<2897<224
 43699|  %1188 = gep %1184, i64 1472                                                                                           ;L225<2893<50<2494<138<2897<224
 43700|  %1189 = load i64, ptr %1188, , !!49865, !!8                                                                           ;L225<2893<50<2494<138<2897<224
 43701|  %1190 = invoke { i64, i64 } @ai::utils18range_misjudge_rng(i64 %1187, ptr %5, ptr %4, i64 %1189)
 43702|  to label %1191 unwind label %1106                                                                                     ;L225<2893<50<2494<138<2897<224
 43703| 
 43704| 1191: ; preds = %1186
 43705|  %1192 = extractvalue { i64, i64 } %1190, 0                                                                            ;L225<2893<50<2494<138<2897<224
 43706|  %1193 = extractvalue { i64, i64 } %1190, 1                                                                            ;L225<2893<50<2494<138<2897<224
 43707|  store i64 %1192, ptr %11, , !!49859                                                                                   ;L225<2893<50<2494<138<2897<224
 43708|  store i64 %1193, ptr %1174, , !!49859                                                                                 ;L225<2893<50<2494<138<2897<224
 43709|  %1194 = invoke i64 @ai::plan_legacy3old6battle17max_range_can_use(ptr %1184, ptr %93)
 43710|  to label %1195 unwind label %1106                                                                                     ;L226<2893<50<2494<138<2897<224
 43711| 
 43712| 1195: ; preds = %1191
 43713|  %1196 = invoke i64 @ai::utils19range_misjudge_roll(ptr %3, ptr %11, i64 %1136, i64 %1173)
 43714|  to label %1197 unwind label %1106                                                                                     ;L226<2893<50<2494<138<2897<224
 43715| 
 43716| 1197: ; preds = %1195
 43717|  %1198 = mul i64 %1196, %1194                                                                                          ;L226<2893<50<2494<138<2897<224
 43718|  %1199 = udiv i64 %1198, 1000                                                                                          ;L226<2893<50<2494<138<2897<224
 43719|  %1200 = add nuw nsw i64 %1199, 10000                                                                                  ;L226<2893<50<2494<138<2897<224
 43720|     ;; emr = i64 %1200
 43721|  %1201 = invoke i64 @ai::plan_legacy3old6battle17max_range_can_use(ptr %93, ptr %1184)
 43722|  to label %1202 unwind label %1106                                                                                     ;L227<2893<50<2494<138<2897<224
 43723| 
 43724| 1202: ; preds = %1197
 43725|     ;; mr = i64 %1201
 43726|  %1203 = load ptr, ptr %89, , !!49865, !!8, !!8                                                                        ;L229<2893<50<2494<138<2897<224
 43727|  %1204 = load ptr, ptr %174, , !!49865, !!8, !!8                                                                       ;L229<2893<50<2494<138<2897<224
 43728|  %1205 = invoke zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %1179, ptr %1203, ptr %1204, ptr %4, ptr %1184)
 43729|  to label %1206 unwind label %1106                                                                                     ;L229<2893<50<2494<138<2897<224
 43730| 
 43731| 1206: ; preds = %1202
 43732|  br i1 %1205, label %1207, label %1242                                                                                 ;L229<2893<50<2494<138<2897<224
 43733| 
 43734| 1207: ; preds = %1206
 43735|     ;; other = ptr %93
 43736|  %1208 = gep %1184, i64 1632                                                                                           ;L2158<230<2893<50<2494<138<2897<224
 43737|  %1209 = load i64, ptr %1208, , !!49865, !!8                                                                           ;L2158<230<2893<50<2494<138<2897<224
 43738|     ;; x1 = i64 %1209
 43739|     ;; self = i64 %1209
 43740|  %1210 = gep %1184, i64 1640                                                                                           ;L2158<230<2893<50<2494<138<2897<224
 43741|  %1211 = load i64, ptr %1210, , !!49865, !!8                                                                           ;L2158<230<2893<50<2494<138<2897<224
 43742|     ;; y1 = i64 %1211
 43743|     ;; self = i64 %1211
 43744|  %1212 = load i64, ptr %1176, , !!49865, !!8                                                                           ;L2158<230<2893<50<2494<138<2897<224
 43745|     ;; x2 = i64 %1212
 43746|     ;; other = i64 %1212
 43747|  %1213 = load i64, ptr %1177, , !!49865, !!8                                                                           ;L2158<230<2893<50<2494<138<2897<224
 43748|     ;; y2 = i64 %1213
 43749|     ;; other = i64 %1213
 43750|  %1214 = icmp ult i64 %1209, %1212                                                                                     ;L3147<7<2158<230<2893<50<2494<138<2897<224
 43751|  %1215 = sub nuw i64 %1212, %1209                                                                                      ;L3147<7<2158<230<2893<50<2494<138<2897<224
 43752|  %1216 = sub nuw i64 %1209, %1212                                                                                      ;L3147<7<2158<230<2893<50<2494<138<2897<224
 43753|  %1217 = select i1 %1214, i64 %1215, i64 %1216                                                                         ;L3147<7<2158<230<2893<50<2494<138<2897<224
 43754|     ;; dx = i64 %1217
 43755|  %1218 = icmp ult i64 %1211, %1213                                                                                     ;L3147<8<2158<230<2893<50<2494<138<2897<224
 43756|  %1219 = sub nuw i64 %1213, %1211                                                                                      ;L3147<8<2158<230<2893<50<2494<138<2897<224
 43757|  %1220 = sub nuw i64 %1211, %1213                                                                                      ;L3147<8<2158<230<2893<50<2494<138<2897<224
 43758|  %1221 = select i1 %1218, i64 %1219, i64 %1220                                                                         ;L3147<8<2158<230<2893<50<2494<138<2897<224
 43759|     ;; dy = i64 %1221
 43760|  %1222 = mul i64 %1217, %1217                                                                                          ;L9<2158<230<2893<50<2494<138<2897<224
 43761|  %1223 = mul i64 %1221, %1221                                                                                          ;L9<2158<230<2893<50<2494<138<2897<224
 43762|  %1224 = add i64 %1223, %1222                                                                                          ;L9<2158<230<2893<50<2494<138<2897<224
 43763|  %1225 = mul i64 %1200, %1200                                                                                          ;L230<2893<50<2494<138<2897<224
 43764|  %1226 = icmp ugt i64 %1224, %1225                                                                                     ;L230<2893<50<2494<138<2897<224
 43765|  br i1 %1226, label %1242, label %1227                                                                                 ;L230<2893<50<2494<138<2897<224
 43766| 
 43767| 1227: ; preds = %1207
 43768|  %1228 = gep %1184, i64 104                                                                                            ;L1548<231<2893<50<2494<138<2897<224
 43769|  %1229 = load i64, ptr %1228, , !!49865, !!8                                                                           ;L1548<231<2893<50<2494<138<2897<224
 43770|  %1230 = icmp ne i64 %1229, 13                                                                                         ;L1548<231<2893<50<2494<138<2897<224
 43771|  %1231 = gep %1184, i64 112
 43772|  %1232 = load i64, ptr %1231, , !!49865
 43773|  %1233 = icmp samesign ult i64 %1232, 3
 43774|  %1234 = select i1 %1230, i1 true, i1 %1233                                                                            ;L1548<231<2893<50<2494<138<2897<224
 43775|  br i1 %1234, label %1235, label %1242                                                                                 ;L1548<231<2893<50<2494<138<2897<224
 43776| 
 43777| 1235: ; preds = %1227
 43778|  %1236 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity11block_input(ptr %1184)
 43779|  to label %1237 unwind label %1106                                                                                     ;L231<2893<50<2494<138<2897<224
 43780| 
 43781| 1237: ; preds = %1235
 43782|  %1238 = icmp ne i64 %1201, 0
 43783|  %1239 = or i1 %1238, %1236                                                                                            ;L233<2893<50<2494<138<2897<224
 43785|  br i1 %1239, label %1240, label %1265                                                                                 ;L2494<138<2897<224
 43786| 
 43787| 1240: ; preds = %1242, %1237, %1180
 43788|     ;; self = ptr undef
 43789|     ;; count = i64 1
 43790|     ;; ptr = !DIArgList(ptr %212, i64 %1183)
 43791|     ;; self = !DIArgList(ptr %212, i64 %1183)
 43792|     ;; end_or_len = ptr %213
 43795|  %1241 = icmp eq i64 %1183, 40                                                                                         ;L1714<180<2493<138<2897<224
 43796|  br i1 %1241, label %1243, label %1180                                                                                 ;L180<2493<138<2897<224
 43797| 
 43798| 1242: ; preds = %1227, %1207, %1206
 43800|  br label %1240                                                                                                        ;L2494<138<2897<224
 43801| 
 43802| 1243: ; preds = %1240
 43803|     ;; self = ptr %58
 43804|     ;; self = ptr %58
 43805|  %1244 = load ptr, ptr %58, , !!8, !!8                                                                                 ;L138<2073<237
 43806|     ;; p = ptr %1244
 43807|  %1245 = gep %58, i64 24                                                                                               ;L2075<237
 43808|  %1246 = load i64, ptr %1245, , !!8                                                                                    ;L2075<237
 43809|     ;; len = i64 %1246
 43810|     ;; count = i64 %1246
 43811|     ;; self[0..+8] = ptr %1244
 43812|     ;; slice[0..+8] = ptr %1244
 43813|     ;; self[8..+8] = i64 %1246
 43814|     ;; slice[8..+8] = i64 %1246
 43815|     ;; ptr = ptr %1244
 43816|     ;; self = ptr %1244
 43817|  %1247 = mul nuw nsw i64 %1246, 184                                                                                    ;L961<100<1042<237
 43818|  %1248 = gep %1244, i64 %1247                                                                                          ;L961<100<1042<237
 43819|     ;; f[0..+8] = ptr %1
 43820|     ;; f[0..+8] = ptr %1
 43821|     ;; f[8..+8] = ptr %68
 43822|     ;; f[8..+8] = ptr %68
 43823|     ;; f[16..+8] = ptr %6
 43824|     ;; f[16..+8] = ptr %6
 43825|     ;; f[24..+8] = ptr %3
 43826|     ;; f[24..+8] = ptr %3
 43827|     ;; f[32..+8] = ptr %4
 43828|     ;; f[32..+8] = ptr %4
 43829|     ;; f[40..+8] = ptr %5
 43830|     ;; f[40..+8] = ptr %5
 43831|     ;; f[48..+8] = ptr %8
 43832|     ;; f[48..+8] = ptr %8
 43833|     ;; self[0..+8] = ptr %1244
 43834|     ;; self[8..+8] = ptr %1248
 43835|     ;; self = ptr %10
 43839|     ;; self[0..+8] = ptr %1244
 43840|     ;; self[8..+8] = ptr %1248
 43841|  %1249 = gep %10, i64 8                                                                                                ;L69<836<3325<237
 43842|  store ptr %1248, ptr %1249, , !!49987                                                                                 ;L69<836<3325<237
 43843|  %1250 = gep %10, i64 16                                                                                               ;L69<836<3325<237
 43844|  store ptr %1, ptr %1250,                                                                                              ;L69<836<3325<237
 43845|  %1251 = gep %10, i64 24                                                                                               ;L69<836<3325<237
 43846|  store ptr %68, ptr %1251,                                                                                             ;L69<836<3325<237
 43847|  %1252 = gep %10, i64 32                                                                                               ;L69<836<3325<237
 43848|  store ptr %6, ptr %1252,                                                                                              ;L69<836<3325<237
 43849|  %1253 = gep %10, i64 40                                                                                               ;L69<836<3325<237
 43850|  store ptr %3, ptr %1253,                                                                                              ;L69<836<3325<237
 43851|  %1254 = gep %10, i64 48                                                                                               ;L69<836<3325<237
 43852|  store ptr %4, ptr %1254,                                                                                              ;L69<836<3325<237
 43853|  %1255 = gep %10, i64 56                                                                                               ;L69<836<3325<237
 43854|  store ptr %5, ptr %1255,                                                                                              ;L69<836<3325<237
 43855|  %1256 = gep %10, i64 64                                                                                               ;L69<836<3325<237
 43856|  store ptr %8, ptr %1256,                                                                                              ;L69<836<3325<237
 43858|     ;; self = ptr %10
 43861|     ;; self = ptr %10
 43862|     ;; self = ptr %10
 43863|     ;; count = i64 1
 43864|     ;; ptr = ptr %1244
 43865|     ;; self = ptr %1244
 43866|     ;; end_or_len = ptr %1248
 43869|  %1257 = icmp eq i64 %1246, 0                                                                                          ;L1714<180<107<2706<3354<3325<237
 43870|  br i1 %1257, label %1258, label %1259                                                                                 ;L180<107<2706<3354<3325<237
 43871| 
 43872| 1258: ; preds = %1243
 43874|     ;; self = ptr null
 43875|  br label %1272                                                                                                        ;L1011<237
 43876| 
 43877| 1259: ; preds = %1243
 43878|  %1260 = gep %1244, i64 184                                                                                            ;L656<185<107<2706<3354<3325<237
 43879|  store ptr %1260, ptr %10, , !!49975                                                                                   ;L185<107<2706<3354<3325<237
 43880|     ;; self = ptr %1244
 43881|     ;; f = ptr %1250
 43882|     ;; self = ptr %1250
 43883|     ;; x = ptr %1244
 43884|     ;; args = ptr %1244
 43885|     ;; x = ptr %1244
 43895|  %1261 = load i64, ptr %68, , !!50092, !!8                                                                             ;L237<3317<310<1162<107<2706<3354<3325<237
 43896|  %1262 = invoke i64 @ai::plan_legacy8sub_plan9epic_pokeNtB2_15EpicPokeSubPlan5score(ptr poison, i64 %1261, ptr %6, ptr %3, ptr %4, ptr %5, ptr %1244, ptr %8)
 43897|  to label %1263 unwind label %1106                                                                                     ;L237<3317<310<1162<107<2706<3354<3325<237
 43898| 
 43899| 1263: ; preds = %1259
 43900|     ;; first[0..+8] = i64 %1262
 43901|     ;; first[8..+8] = ptr %1244
 43902|  %1264 = invoke { i64, ptr } @core::iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayENCINvNvNtNtNtBa_6traits8iterator8Iterator10max_by_key3keyRB1n_xNCNvMNtNtNtB1r_11plan_legacy8sub_plan9epic_pokeNtB3p_15EpicPokeSubPlan17action_candidatess6_0E0EB2q_4foldTxB3e_ENCINvNvB2q_6max_by4foldB51_INvB2o_7compareB3e_xEE0EB1r_(ptr %10, i64 %1262, ptr %1244)
 43903|  to label %1266 unwind label %1106                                                                                     ;L2707<3354<3325<237
 43904| 
 43905| 1265: ; preds = %1237
 43908|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %54, ptr %5, ptr %4, i64 5, i1 zeroext false)
 43909|  to label %1506 unwind label %1106                                                                                     ;L234
 43910| 
 43911| 1266: ; preds = %1263
 43912|  %1267 = extractvalue { i64, ptr } %1264, 1                                                                            ;L2707<3354<3325<237
 43914|     ;; self = ptr %1267
 43915|  %1268 = icmp eq ptr %1267, null                                                                                       ;L1011<237
 43916|  br i1 %1268, label %1272, label %1269                                                                                 ;L1011<237
 43917| 
 43918| 1269: ; preds = %1266
 43919|     ;; best_move_action = ptr %1267
 43921|  %1270 = gep %175, i64 528                                                                                             ;L241
 43922|  %1271 = load ptr, ptr %1270, , !!8                                                                                    ;L241
 43923|  invoke void %1271(ptr sret([40 x i8]) %53, ptr %173)
 43924|  to label %1273 unwind label %1106                                                                                     ;L241
 43925| 
 43926| 1272: ; preds = %1266, %1258
 43927|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.208) #31
 43928|  to label %84 unwind label %1106                                                                                       ;L1013<237
 43929| 
 43930| 1273: ; preds = %1269
 43931|     ;; self = ptr %53
 43932|     ;; f[0..+8] = ptr %4
 43933|     ;; f[8..+8] = ptr %93
 43935|     ;; f[0..+8] = ptr %4
 43936|     ;; f[8..+8] = ptr %93
 43937|     ;; self = ptr %53
 43940|  %1274 = load i64, ptr %1176, , !!50145
 43941|  %1275 = load i64, ptr %1177, , !!50145
 43942|  br label %1276                                                                                                        ;L2493<2897<241
 43943| 
 43944| 1276: ; preds = %1313, %1273
 43945|  %1277 = invoke ptr @gc::simulationNtB5_14ProjectileIterNtNtNtNtCsjihNppCmMEE_4core4iter6traits8iterator8Iterator4next(ptr %53)
 43946|  to label %1278 unwind label %1106                                                                                     ;L2493<2897<241
 43947| 
 43948| 1278: ; preds = %1276
 43949|  %1279 = icmp eq ptr %1277, null                                                                                       ;L2493<2897<241
 43950|  br i1 %1279, label %1314, label %1280                                                                                 ;L2493<2897<241
 43951| 
 43952| 1280: ; preds = %1278
 43953|     ;; x = ptr %1277
 43956|     ;; x = ptr %1277
 43959|     ;; p = ptr %1277
 43960|     ;; __arg1_discr = i64 0
 43961|     ;; self = ptr %1277
 43962|     ;; self = ptr %1277
 43965|  %1281 = load i64, ptr %1277, , !!50199, !!8                                                                           ;L1127<264<242<2893<2494<2897<241
 43966|     ;; __self_discr = i64 %1281
 43967|  %1282 = icmp eq i64 %1281, 0                                                                                          ;L1127<264<242<2893<2494<2897<241
 43968|  br i1 %1282, label %1283, label %1287                                                                                 ;L1127<264<242<2893<2494<2897<241
 43969| 
 43970| 1283: ; preds = %1280
 43971|  %1284 = gep %1277, i64 8                                                                                              ;L1127<264<242<2893<2494<2897<241
 43972|     ;; __self_0 = ptr %1277
 43973|     ;; self = ptr %1277
 43978|  %1285 = load i64, ptr %1284, , !!50199, !!8                                                                           ;L1878<2123<1127<264<242<2893<2494<2897<241
 43979|  %1286 = icmp eq i64 %1285, %78                                                                                        ;L1878<2123<1127<264<242<2893<2494<2897<241
 43980|  br i1 %1286, label %1313, label %1287                                                                                 ;L242<2893<2494<2897<241
 43981| 
 43982| 1287: ; preds = %1283, %1280
 43983|     ;; self = ptr %1277
 43984|  %1288 = gep %1277, i64 64                                                                                             ;L134<242<2893<2494<2897<241
 43985|  %1289 = load i64, ptr %1288, , !!50199, !!8                                                                           ;L134<242<2893<2494<2897<241
 43986|  %1290 = icmp ne i64 %1289, 9                                                                                          ;L134<242<2893<2494<2897<241
 43987|  call void @llvm.assume(i1 %1290)                                                                                      ;L134<242<2893<2494<2897<241
 43988|  %1291 = add nsw i64 %1289, -2                                                                                         ;L134<242<2893<2494<2897<241
 43989|  %1292 = icmp samesign ugt i64 %1289, 1                                                                                ;L134<242<2893<2494<2897<241
 43990|  %1293 = select i1 %1292, i64 %1291, i64 7                                                                             ;L134<242<2893<2494<2897<241
 43991|  switch i64 %1293, label %1296 [
 43992|  i64 4, label %1313
 43993|  i64 5, label %1313
 43994|  i64 7, label %1294
 43995|  ]                                                                                                                     ;L134<242<2893<2494<2897<241
 43996| 
 43997| 1294: ; preds = %1287
 43998|  %1295 = icmp eq i64 %1289, 1                                                                                          ;L134<242<2893<2494<2897<241
 43999|  br i1 %1295, label %1313, label %1296                                                                                 ;L242<2893<2494<2897<241
 44000| 
 44001| 1296: ; preds = %1294, %1287
 44002|     ;; x1 = i64 %1274
 44003|     ;; self = i64 %1274
 44004|     ;; y1 = i64 %1275
 44005|     ;; self = i64 %1275
 44006|  %1297 = gep %1277, i64 256                                                                                            ;L243<2893<2494<2897<241
 44007|  %1298 = load i64, ptr %1297, , !!50199, !!8                                                                           ;L243<2893<2494<2897<241
 44008|     ;; x2 = i64 %1298
 44009|     ;; other = i64 %1298
 44010|  %1299 = gep %1277, i64 264                                                                                            ;L243<2893<2494<2897<241
 44011|  %1300 = load i64, ptr %1299, , !!50199, !!8                                                                           ;L243<2893<2494<2897<241
 44012|     ;; y2 = i64 %1300
 44013|     ;; other = i64 %1300
 44014|  %1301 = icmp ult i64 %1274, %1298                                                                                     ;L3147<7<243<2893<2494<2897<241
 44015|  %1302 = sub nuw i64 %1298, %1274                                                                                      ;L3147<7<243<2893<2494<2897<241
 44016|  %1303 = sub nuw i64 %1274, %1298                                                                                      ;L3147<7<243<2893<2494<2897<241
 44017|  %1304 = select i1 %1301, i64 %1302, i64 %1303                                                                         ;L3147<7<243<2893<2494<2897<241
 44018|     ;; dx = i64 %1304
 44019|  %1305 = icmp ult i64 %1275, %1300                                                                                     ;L3147<8<243<2893<2494<2897<241
 44020|  %1306 = sub nuw i64 %1300, %1275                                                                                      ;L3147<8<243<2893<2494<2897<241
 44021|  %1307 = sub nuw i64 %1275, %1300                                                                                      ;L3147<8<243<2893<2494<2897<241
 44022|  %1308 = select i1 %1305, i64 %1306, i64 %1307                                                                         ;L3147<8<243<2893<2494<2897<241
 44023|     ;; dy = i64 %1308
 44024|  %1309 = mul i64 %1304, %1304                                                                                          ;L9<243<2893<2494<2897<241
 44025|  %1310 = mul i64 %1308, %1308                                                                                          ;L9<243<2893<2494<2897<241
 44026|  %1311 = add i64 %1310, %1309                                                                                          ;L9<243<2893<2494<2897<241
 44027|  %1312 = icmp ult i64 %1311, 176400000000                                                                              ;L243<2893<2494<2897<241
 44028|  br i1 %1312, label %1364, label %1313                                                                                 ;L2494<2897<241
 44029| 
 44030| 1313: ; preds = %1296, %1294, %1287, %1287, %1283
 44031|  br label %1276                                                                                                        ;L2493<2897<241
 44032| 
 44033| 1314: ; preds = %1278
 44035|     ;; self = ptr undef
 44036|     ;; self = ptr undef
 44038|     ;; self = ptr undef
 44041|     ;; self = ptr undef
 44042|     ;; count = i64 1
 44043|     ;; self = ptr %212
 44044|     ;; end_or_len = ptr %213
 44047|     ;; ptr = ptr %212
 44048|     ;; x = ptr %212
 44049|  %1315 = load ptr, ptr %212, , !!50259, !!8                                                                            ;L2494<138<2897<244
 44053|  %1316 = icmp eq ptr %1315, null                                                                                       ;L49<2494<138<2897<244
 44054|  br i1 %1316, label %1323, label %1317                                                                                 ;L49<2494<138<2897<244
 44055| 
 44056| 1317: ; preds = %1314
 44057|     ;; x = ptr %1315
 44058|  %1318 = gep %1315, i64 776                                                                                            ;L50<2494<138<2897<244
 44059|  %1319 = load i64, ptr %1318, , !!50259, !!8                                                                           ;L50<2494<138<2897<244
 44064|  %1320 = icmp sgt i64 %1319, -1                                                                                        ;L245<2893<50<2494<138<2897<244
 44065|  %1321 = icmp eq i64 %1319, -9223372036854775805                                                                       ;L245<2893<50<2494<138<2897<244
 44066|  %1322 = or i1 %1320, %1321                                                                                            ;L245<2893<50<2494<138<2897<244
 44067|  br i1 %1322, label %1365, label %1323                                                                                 ;L2494<138<2897<244
 44068| 
 44069| 1323: ; preds = %1317, %1314
 44070|     ;; self = ptr undef
 44071|     ;; count = i64 1
 44072|     ;; ptr = !DIArgList(ptr %212, i64 8)
 44073|     ;; self = !DIArgList(ptr %212, i64 8)
 44074|     ;; end_or_len = ptr %213
 44077|  %1324 = gep %212, i64 8                                                                                               ;L656<185<2493<138<2897<244
 44078|     ;; ptr = ptr %1324
 44079|     ;; x = ptr %1324
 44080|  %1325 = load ptr, ptr %1324, , !!50259, !!8                                                                           ;L2494<138<2897<244
 44084|  %1326 = icmp eq ptr %1325, null                                                                                       ;L49<2494<138<2897<244
 44085|  br i1 %1326, label %1333, label %1327                                                                                 ;L49<2494<138<2897<244
 44086| 
 44087| 1327: ; preds = %1323
 44088|     ;; x = ptr %1325
 44089|  %1328 = gep %1325, i64 776                                                                                            ;L50<2494<138<2897<244
 44090|  %1329 = load i64, ptr %1328, , !!50259, !!8                                                                           ;L50<2494<138<2897<244
 44095|  %1330 = icmp sgt i64 %1329, -1                                                                                        ;L245<2893<50<2494<138<2897<244
 44096|  %1331 = icmp eq i64 %1329, -9223372036854775805                                                                       ;L245<2893<50<2494<138<2897<244
 44097|  %1332 = or i1 %1330, %1331                                                                                            ;L245<2893<50<2494<138<2897<244
 44098|  br i1 %1332, label %1365, label %1333                                                                                 ;L2494<138<2897<244
 44099| 
 44100| 1333: ; preds = %1327, %1323
 44101|     ;; self = ptr undef
 44102|     ;; count = i64 1
 44103|     ;; ptr = !DIArgList(ptr %212, i64 16)
 44104|     ;; self = !DIArgList(ptr %212, i64 16)
 44105|     ;; end_or_len = ptr %213
 44108|  %1334 = gep %212, i64 16                                                                                              ;L656<185<2493<138<2897<244
 44109|     ;; ptr = ptr %1334
 44110|     ;; x = ptr %1334
 44111|  %1335 = load ptr, ptr %1334, , !!50259, !!8                                                                           ;L2494<138<2897<244
 44115|  %1336 = icmp eq ptr %1335, null                                                                                       ;L49<2494<138<2897<244
 44116|  br i1 %1336, label %1343, label %1337                                                                                 ;L49<2494<138<2897<244
 44117| 
 44118| 1337: ; preds = %1333
 44119|     ;; x = ptr %1335
 44120|  %1338 = gep %1335, i64 776                                                                                            ;L50<2494<138<2897<244
 44121|  %1339 = load i64, ptr %1338, , !!50259, !!8                                                                           ;L50<2494<138<2897<244
 44126|  %1340 = icmp sgt i64 %1339, -1                                                                                        ;L245<2893<50<2494<138<2897<244
 44127|  %1341 = icmp eq i64 %1339, -9223372036854775805                                                                       ;L245<2893<50<2494<138<2897<244
 44128|  %1342 = or i1 %1340, %1341                                                                                            ;L245<2893<50<2494<138<2897<244
 44129|  br i1 %1342, label %1365, label %1343                                                                                 ;L2494<138<2897<244
 44130| 
 44131| 1343: ; preds = %1337, %1333
 44132|     ;; self = ptr undef
 44133|     ;; count = i64 1
 44134|     ;; ptr = !DIArgList(ptr %212, i64 24)
 44135|     ;; self = !DIArgList(ptr %212, i64 24)
 44136|     ;; end_or_len = ptr %213
 44139|  %1344 = gep %212, i64 24                                                                                              ;L656<185<2493<138<2897<244
 44140|     ;; ptr = ptr %1344
 44141|     ;; x = ptr %1344
 44142|  %1345 = load ptr, ptr %1344, , !!50259, !!8                                                                           ;L2494<138<2897<244
 44146|  %1346 = icmp eq ptr %1345, null                                                                                       ;L49<2494<138<2897<244
 44147|  br i1 %1346, label %1353, label %1347                                                                                 ;L49<2494<138<2897<244
 44148| 
 44149| 1347: ; preds = %1343
 44150|     ;; x = ptr %1345
 44151|  %1348 = gep %1345, i64 776                                                                                            ;L50<2494<138<2897<244
 44152|  %1349 = load i64, ptr %1348, , !!50259, !!8                                                                           ;L50<2494<138<2897<244
 44157|  %1350 = icmp sgt i64 %1349, -1                                                                                        ;L245<2893<50<2494<138<2897<244
 44158|  %1351 = icmp eq i64 %1349, -9223372036854775805                                                                       ;L245<2893<50<2494<138<2897<244
 44159|  %1352 = or i1 %1350, %1351                                                                                            ;L245<2893<50<2494<138<2897<244
 44160|  br i1 %1352, label %1365, label %1353                                                                                 ;L2494<138<2897<244
 44161| 
 44162| 1353: ; preds = %1347, %1343
 44163|     ;; self = ptr undef
 44164|     ;; count = i64 1
 44165|     ;; ptr = !DIArgList(ptr %212, i64 32)
 44166|     ;; self = !DIArgList(ptr %212, i64 32)
 44167|     ;; end_or_len = ptr %213
 44170|  %1354 = gep %212, i64 32                                                                                              ;L656<185<2493<138<2897<244
 44171|     ;; ptr = ptr %1354
 44172|     ;; x = ptr %1354
 44173|  %1355 = load ptr, ptr %1354, , !!50259, !!8                                                                           ;L2494<138<2897<244
 44177|  %1356 = icmp eq ptr %1355, null                                                                                       ;L49<2494<138<2897<244
 44178|  br i1 %1356, label %1363, label %1357                                                                                 ;L49<2494<138<2897<244
 44179| 
 44180| 1357: ; preds = %1353
 44181|     ;; x = ptr %1355
 44182|  %1358 = gep %1355, i64 776                                                                                            ;L50<2494<138<2897<244
 44183|  %1359 = load i64, ptr %1358, , !!50259, !!8                                                                           ;L50<2494<138<2897<244
 44188|  %1360 = icmp sgt i64 %1359, -1                                                                                        ;L245<2893<50<2494<138<2897<244
 44189|  %1361 = icmp eq i64 %1359, -9223372036854775805                                                                       ;L245<2893<50<2494<138<2897<244
 44190|  %1362 = or i1 %1360, %1361                                                                                            ;L245<2893<50<2494<138<2897<244
 44191|  br i1 %1362, label %1365, label %1363                                                                                 ;L2494<138<2897<244
 44192| 
 44193| 1363: ; preds = %1357, %1353
 44194|     ;; self = ptr undef
 44195|     ;; count = i64 1
 44196|     ;; ptr = !DIArgList(ptr %212, i64 40)
 44197|     ;; self = !DIArgList(ptr %212, i64 40)
 44198|     ;; end_or_len = ptr %213
 44201|     ;; trajectory_possible = i1 false
 44203|  br label %1366                                                                                                        ;L251
 44204| 
 44205| 1364: ; preds = %1296
 44207|     ;; trajectory_possible = i8 1
 44208|  br label %1365                                                                                                        ;L246
 44209| 
 44210| 1365: ; preds = %1364, %1357, %1347, %1337, %1327, %1317
 44213|  invoke fastcc void @ai::small_actionNtB4_15SmallActionPlayNtNtCsjihNppCmMEE_4core5clone5Clone5clone(ptr %51, ptr %1267)
 44214|  to label %1367 unwind label %1106                                                                                     ;L247
 44215| 
 44216| 1366: ; preds = %1371, %1363
 44219|  invoke fastcc void @ai::small_actionNtB4_15SmallActionPlayNtNtCsjihNppCmMEE_4core5clone5Clone5clone(ptr %42, ptr %1267)
 44220|  to label %1433 unwind label %1106                                                                                     ;L268
 44221| 
 44222| 1367: ; preds = %1365
 44223|  %1368 = load i64, ptr %68, , !!8                                                                                      ;L247
 44224|  invoke void @ai::small_actionNtB2_15SmallActionPlay9get_input(ptr sret([32 x i8]) %52, ptr %51, i64 %1368, ptr %3, ptr %4, ptr %5, ptr %203, ptr %8)
 44225|  to label %1371 unwind label %1369                                                                                     ;L247
 44226| 
 44227| 1369: ; preds = %1367
 44228|  %1370 = cleanuppad within none []
 44229|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %51) #30 [ "funclet"(token %1370) ] ;L248
 44230|  cleanupret from %1370 unwind label %1106                                                                              ;L248
 44231| 
 44232| 1371: ; preds = %1367
 44233|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %51)           ;L248
 44235|  %1372 = load i64, ptr %52, , !!8                                                                                      ;L251
 44236|  switch i64 %1372, label %1379 [
 44237|  i64 -1, label %1366
 44238|  i64 0, label %1373
 44239|  ]                                                                                                                     ;L251
 44240| 
 44241| 1373: ; preds = %1371
 44242|  %1374 = gep %52, i64 16                                                                                               ;L251
 44243|  %1375 = load i64, ptr %1374,                                                                                          ;L251
 44244|     ;; move_action_input[16..+8] = i64 %1375
 44245|  %1376 = gep %52, i64 8                                                                                                ;L251
 44246|  %1377 = load i64, ptr %1376,                                                                                          ;L251
 44247|     ;; move_action_input[8..+8] = i64 %1377
 44248|     ;; x = i64 %1377
 44249|     ;; y = i64 %1375
 44251|  %1378 = load i64, ptr %68, , !!8                                                                                      ;L255
 44252|  invoke void @ai::position_eval26position_score_at_position(ptr sret([56 x i8]) %50, i64 %1378, ptr %4, ptr %5, ptr %203, i64 %1377, i64 %1375, i8 11)
 44253|  to label %1380 unwind label %1106                                                                                     ;L255
 44254| 
 44255| 1379: ; preds = %1371
 44258|  invoke fastcc void @ai::small_actionNtB4_15SmallActionPlayNtNtCsjihNppCmMEE_4core5clone5Clone5clone(ptr %44, ptr %1267)
 44259|  to label %1396 unwind label %1106                                                                                     ;L265
 44260| 
 44261| 1380: ; preds = %1373
 44262|  %1381 = gep %50, i64 48                                                                                               ;L257
 44263|  %1382 = load i8, ptr %1381, , !!8                                                                                     ;L257
 44264|  %1383 = trunc nuw i8 %1382 to i1                                                                                      ;L257
 44265|  br i1 %1383, label %1388, label %1384                                                                                 ;L257
 44266| 
 44267| 1384: ; preds = %1380
 44268|  %1385 = gep %50, i64 49                                                                                               ;L257
 44269|  %1386 = load i8, ptr %1385, , !!8                                                                                     ;L257
 44270|  %1387 = trunc nuw i8 %1386 to i1                                                                                      ;L257
 44271|     ;; on_trajectory = i1 %1387
 44273|  br i1 %1387, label %1390, label %1389                                                                                 ;L259
 44274| 
 44275| 1388: ; preds = %1380
 44276|     ;; on_trajectory = i8 1
 44278|  br label %1390                                                                                                        ;L259
 44279| 
 44280| 1389: ; preds = %1384
 44283|  invoke fastcc void @ai::small_actionNtB4_15SmallActionPlayNtNtCsjihNppCmMEE_4core5clone5Clone5clone(ptr %46, ptr %1267)
 44284|  to label %1391 unwind label %1106                                                                                     ;L262
 44285| 
 44286| 1390: ; preds = %1388, %1384
 44289|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %48, ptr %5, ptr %4, i64 5, i1 zeroext false)
 44290|  to label %1393 unwind label %1106                                                                                     ;L260
 44291| 
 44292| 1391: ; preds = %1389
 44293|  call void @llvm.memcpy.p0.p0.i64(ptr %47, ptr %46, i64 184, i1 false)                                                 ;L262
 44295|  invoke void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE12from_iter_inABU_j1_EBY_(ptr sret([32 x i8]) %0, ptr %47, ptr %183)
 44296|  to label %1392 unwind label %1106                                                                                     ;L262
 44297| 
 44298| 1392: ; preds = %1391
 44300|  br label %1398                                                                                                        ;L259
 44301| 
 44302| 1393: ; preds = %1390
 44303|  call void @llvm.memcpy.p0.p0.i64(ptr %49, ptr %48, i64 136, i1 false)                                                 ;L260
 44305|  %1394 = gep %49, i64 177                                                                                              ;L260
 44306|  store i8 3, ptr %1394,                                                                                                ;L260
 44307|  invoke void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE12from_iter_inABU_j1_EBY_(ptr sret([32 x i8]) %0, ptr %49, ptr %183)
 44308|  to label %1395 unwind label %1106                                                                                     ;L260
 44309| 
 44310| 1395: ; preds = %1393
 44312|  br label %1398                                                                                                        ;L259
 44313| 
 44314| 1396: ; preds = %1379
 44315|  call void @llvm.memcpy.p0.p0.i64(ptr %45, ptr %44, i64 184, i1 false)                                                 ;L265
 44317|  invoke void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE12from_iter_inABU_j1_EBY_(ptr sret([32 x i8]) %0, ptr %45, ptr %183)
 44318|  to label %1397 unwind label %1106                                                                                     ;L265
 44319| 
 44320| 1397: ; preds = %1396
 44322|  br label %1398                                                                                                        ;L252
 44323| 
 44324| 1398: ; preds = %1434, %1397, %1395, %1392
 44327|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %58)
 44328|  to label %1417 unwind label %1399                                                                                     ;L825<275
 44329| 
 44330| 1399: ; preds = %1398
 44331|  %1400 = cleanuppad within none []
 44335|     ;; self = ptr %58
 44337|     ;; self = ptr %58
 44338|     ;; self = ptr %58
 44339|     ;; elem_size = i64 184
 44340|     ;; align = i64 8
 44341|  %1401 = gep %58, i64 16                                                                                               ;L159<703<714<825<825<275
 44342|  %1402 = load i64, ptr %1401, , !!8                                                                                    ;L159<703<714<825<825<275
 44343|  %1403 = icmp eq i64 %1402, 0                                                                                          ;L159<703<714<825<825<275
 44344|  br i1 %1403, label %1416, label %1404                                                                                 ;L159<703<714<825<825<275
 44345| 
 44346| 1404: ; preds = %1399
 44347|     ;; layout[0..+8] = i64 8
 44348|     ;; layout[8..+8] = i64 %1402
 44349|  %1405 = gep %58, i64 8                                                                                                ;L704<714<825<825<275
 44350|  %1406 = load ptr, ptr %1405, , !!8, !!8                                                                               ;L704<714<825<825<275
 44351|  %1407 = load ptr, ptr %58, , !!8, !!8                                                                                 ;L704<714<825<825<275
 44352|  %1408 = gep %1406, i64 16                                                                                             ;L704<714<825<825<275
 44353|  %1409 = load ptr, ptr %1408, , !!50352, !!8, !!8                                                                      ;L704<714<825<825<275
 44356|     ;; ptr = ptr %1407
 44357|     ;; ptr = ptr %1407
 44358|     ;; layout[0..+8] = i64 8
 44359|     ;; layout[8..+8] = i64 %1402
 44360|     ;; footer = ptr %1409
 44361|     ;; footer = ptr %1409
 44362|     ;; self = ptr %1409
 44363|  %1410 = gep %1409, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<275
 44364|  %1411 = load ptr, ptr %1410, , !!50352, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<275
 44367|     ;; self = ptr %1407
 44368|  %1412 = icmp eq ptr %1411, %1407                                                                                      ;L1714<1700<1707<704<714<825<825<275
 44369|  br i1 %1412, label %1413, label %1416                                                                                 ;L1707<704<714<825<825<275
 44370| 
 44371| 1413: ; preds = %1404
 44372|  %1414 = mul i64 %1402, 184                                                                                            ;L166<703<714<825<825<275
 44373|     ;; layout[8..+8] = i64 %1414
 44374|     ;; layout[8..+8] = i64 %1414
 44375|     ;; count = i64 %1414
 44376|  %1415 = gep %1407, i64 %1414                                                                                          ;L961<1708<704<714<825<825<275
 44377|     ;; ptr = ptr %1415
 44378|     ;; val = ptr %1415
 44379|     ;; val = ptr %1415
 44380|     ;; self = ptr %1409
 44381|     ;; self = ptr %1409
 44382|  store ptr %1415, ptr %1410, , !!50352                                                                                 ;L931<513<437<1709<704<714<825<825<275
 44383|  br label %1416                                                                                                        ;L1707<704<714<825<825<275
 44384| 
 44385| 1416: ; preds = %1413, %1404, %1399
 44386|  cleanupret from %1400 unwind label %188
 44387| 
 44388| 1417: ; preds = %1398
 44392|     ;; self = ptr %58
 44394|     ;; self = ptr %58
 44395|     ;; self = ptr %58
 44396|     ;; elem_size = i64 184
 44397|     ;; align = i64 8
 44398|  %1418 = gep %58, i64 16                                                                                               ;L159<703<714<825<825<275
 44399|  %1419 = load i64, ptr %1418, , !!8                                                                                    ;L159<703<714<825<825<275
 44400|  %1420 = icmp eq i64 %1419, 0                                                                                          ;L159<703<714<825<825<275
 44401|  br i1 %1420, label %1435, label %1421                                                                                 ;L159<703<714<825<825<275
 44402| 
 44403| 1421: ; preds = %1417
 44404|     ;; layout[0..+8] = i64 8
 44405|     ;; layout[8..+8] = i64 %1419
 44406|  %1422 = gep %58, i64 8                                                                                                ;L704<714<825<825<275
 44407|  %1423 = load ptr, ptr %1422, , !!8, !!8                                                                               ;L704<714<825<825<275
 44408|  %1424 = load ptr, ptr %58, , !!8, !!8                                                                                 ;L704<714<825<825<275
 44409|  %1425 = gep %1423, i64 16                                                                                             ;L704<714<825<825<275
 44410|  %1426 = load ptr, ptr %1425, , !!50405, !!8, !!8                                                                      ;L704<714<825<825<275
 44413|     ;; ptr = ptr %1424
 44414|     ;; ptr = ptr %1424
 44415|     ;; layout[0..+8] = i64 8
 44416|     ;; layout[8..+8] = i64 %1419
 44417|     ;; footer = ptr %1426
 44418|     ;; footer = ptr %1426
 44419|     ;; self = ptr %1426
 44420|  %1427 = gep %1426, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<275
 44421|  %1428 = load ptr, ptr %1427, , !!50405, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<275
 44424|     ;; self = ptr %1424
 44425|  %1429 = icmp eq ptr %1428, %1424                                                                                      ;L1714<1700<1707<704<714<825<825<275
 44426|  br i1 %1429, label %1430, label %1435                                                                                 ;L1707<704<714<825<825<275
 44427| 
 44428| 1430: ; preds = %1421
 44429|  %1431 = mul i64 %1419, 184                                                                                            ;L166<703<714<825<825<275
 44430|     ;; layout[8..+8] = i64 %1431
 44431|     ;; layout[8..+8] = i64 %1431
 44432|     ;; count = i64 %1431
 44433|  %1432 = gep %1424, i64 %1431                                                                                          ;L961<1708<704<714<825<825<275
 44434|     ;; ptr = ptr %1432
 44435|     ;; val = ptr %1432
 44436|     ;; val = ptr %1432
 44437|     ;; self = ptr %1426
 44438|     ;; self = ptr %1426
 44439|  store ptr %1432, ptr %1427, , !!50405                                                                                 ;L931<513<437<1709<704<714<825<825<275
 44440|  br label %1435                                                                                                        ;L1707<704<714<825<825<275
 44441| 
 44442| 1433: ; preds = %1366
 44443|  call void @llvm.memcpy.p0.p0.i64(ptr %43, ptr %42, i64 184, i1 false)                                                 ;L268
 44445|  invoke void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE12from_iter_inABU_j1_EBY_(ptr sret([32 x i8]) %0, ptr %43, ptr %183)
 44446|  to label %1434 unwind label %1106                                                                                     ;L268
 44447| 
 44448| 1434: ; preds = %1433
 44450|  br label %1398                                                                                                        ;L251
 44451| 
 44452| 1435: ; preds = %1430, %1421, %1417
 44455|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %62)
 44456|  to label %1454 unwind label %1436                                                                                     ;L825<275
 44457| 
 44458| 1436: ; preds = %1435
 44459|  %1437 = cleanuppad within none []
 44463|     ;; self = ptr %62
 44465|     ;; self = ptr %62
 44466|     ;; self = ptr %62
 44467|     ;; elem_size = i64 184
 44468|     ;; align = i64 8
 44469|  %1438 = gep %62, i64 16                                                                                               ;L159<703<714<825<825<275
 44470|  %1439 = load i64, ptr %1438, , !!8                                                                                    ;L159<703<714<825<825<275
 44471|  %1440 = icmp eq i64 %1439, 0                                                                                          ;L159<703<714<825<825<275
 44472|  br i1 %1440, label %1453, label %1441                                                                                 ;L159<703<714<825<825<275
 44473| 
 44474| 1441: ; preds = %1436
 44475|     ;; layout[0..+8] = i64 8
 44476|     ;; layout[8..+8] = i64 %1439
 44477|  %1442 = gep %62, i64 8                                                                                                ;L704<714<825<825<275
 44478|  %1443 = load ptr, ptr %1442, , !!8, !!8                                                                               ;L704<714<825<825<275
 44479|  %1444 = load ptr, ptr %62, , !!8, !!8                                                                                 ;L704<714<825<825<275
 44480|  %1445 = gep %1443, i64 16                                                                                             ;L704<714<825<825<275
 44481|  %1446 = load ptr, ptr %1445, , !!50464, !!8, !!8                                                                      ;L704<714<825<825<275
 44484|     ;; ptr = ptr %1444
 44485|     ;; ptr = ptr %1444
 44486|     ;; layout[0..+8] = i64 8
 44487|     ;; layout[8..+8] = i64 %1439
 44488|     ;; footer = ptr %1446
 44489|     ;; footer = ptr %1446
 44490|     ;; self = ptr %1446
 44491|  %1447 = gep %1446, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<275
 44492|  %1448 = load ptr, ptr %1447, , !!50464, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<275
 44495|     ;; self = ptr %1444
 44496|  %1449 = icmp eq ptr %1448, %1444                                                                                      ;L1714<1700<1707<704<714<825<825<275
 44497|  br i1 %1449, label %1450, label %1453                                                                                 ;L1707<704<714<825<825<275
 44498| 
 44499| 1450: ; preds = %1441
 44500|  %1451 = mul i64 %1439, 184                                                                                            ;L166<703<714<825<825<275
 44501|     ;; layout[8..+8] = i64 %1451
 44502|     ;; layout[8..+8] = i64 %1451
 44503|     ;; count = i64 %1451
 44504|  %1452 = gep %1444, i64 %1451                                                                                          ;L961<1708<704<714<825<825<275
 44505|     ;; ptr = ptr %1452
 44506|     ;; val = ptr %1452
 44507|     ;; val = ptr %1452
 44508|     ;; self = ptr %1446
 44509|     ;; self = ptr %1446
 44510|  store ptr %1452, ptr %1447, , !!50464                                                                                 ;L931<513<437<1709<704<714<825<825<275
 44511|  br label %1453                                                                                                        ;L1707<704<714<825<825<275
 44512| 
 44513| 1453: ; preds = %1450, %1441, %1436
 44514|  cleanupret from %1437 unwind label %82
 44515| 
 44516| 1454: ; preds = %1435
 44520|     ;; self = ptr %62
 44522|     ;; self = ptr %62
 44523|     ;; self = ptr %62
 44524|     ;; elem_size = i64 184
 44525|     ;; align = i64 8
 44526|  %1455 = gep %62, i64 16                                                                                               ;L159<703<714<825<825<275
 44527|  %1456 = load i64, ptr %1455, , !!8                                                                                    ;L159<703<714<825<825<275
 44528|  %1457 = icmp eq i64 %1456, 0                                                                                          ;L159<703<714<825<825<275
 44529|  br i1 %1457, label %1470, label %1458                                                                                 ;L159<703<714<825<825<275
 44530| 
 44531| 1458: ; preds = %1454
 44532|     ;; layout[0..+8] = i64 8
 44533|     ;; layout[8..+8] = i64 %1456
 44534|  %1459 = gep %62, i64 8                                                                                                ;L704<714<825<825<275
 44535|  %1460 = load ptr, ptr %1459, , !!8, !!8                                                                               ;L704<714<825<825<275
 44536|  %1461 = load ptr, ptr %62, , !!8, !!8                                                                                 ;L704<714<825<825<275
 44537|  %1462 = gep %1460, i64 16                                                                                             ;L704<714<825<825<275
 44538|  %1463 = load ptr, ptr %1462, , !!50517, !!8, !!8                                                                      ;L704<714<825<825<275
 44541|     ;; ptr = ptr %1461
 44542|     ;; ptr = ptr %1461
 44543|     ;; layout[0..+8] = i64 8
 44544|     ;; layout[8..+8] = i64 %1456
 44545|     ;; footer = ptr %1463
 44546|     ;; footer = ptr %1463
 44547|     ;; self = ptr %1463
 44548|  %1464 = gep %1463, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<275
 44549|  %1465 = load ptr, ptr %1464, , !!50517, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<275
 44552|     ;; self = ptr %1461
 44553|  %1466 = icmp eq ptr %1465, %1461                                                                                      ;L1714<1700<1707<704<714<825<825<275
 44554|  br i1 %1466, label %1467, label %1470                                                                                 ;L1707<704<714<825<825<275
 44555| 
 44556| 1467: ; preds = %1458
 44557|  %1468 = mul i64 %1456, 184                                                                                            ;L166<703<714<825<825<275
 44558|     ;; layout[8..+8] = i64 %1468
 44559|     ;; layout[8..+8] = i64 %1468
 44560|     ;; count = i64 %1468
 44561|  %1469 = gep %1461, i64 %1468                                                                                          ;L961<1708<704<714<825<825<275
 44562|     ;; ptr = ptr %1469
 44563|     ;; val = ptr %1469
 44564|     ;; val = ptr %1469
 44565|     ;; self = ptr %1463
 44566|     ;; self = ptr %1463
 44567|  store ptr %1469, ptr %1464, , !!50517                                                                                 ;L931<513<437<1709<704<714<825<825<275
 44568|  br label %1470                                                                                                        ;L1707<704<714<825<825<275
 44569| 
 44570| 1470: ; preds = %1614, %1467, %1458, %1454
 44574|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %65)
 44575|  to label %1489 unwind label %1471                                                                                     ;L825<276
 44576| 
 44577| 1471: ; preds = %1470
 44578|  %1472 = cleanuppad within none []
 44582|     ;; self = ptr %65
 44584|     ;; self = ptr %65
 44585|     ;; self = ptr %65
 44586|     ;; elem_size = i64 184
 44587|     ;; align = i64 8
 44588|  %1473 = gep %65, i64 16                                                                                               ;L159<703<714<825<825<276
 44589|  %1474 = load i64, ptr %1473, , !!8                                                                                    ;L159<703<714<825<825<276
 44590|  %1475 = icmp eq i64 %1474, 0                                                                                          ;L159<703<714<825<825<276
 44591|  br i1 %1475, label %1488, label %1476                                                                                 ;L159<703<714<825<825<276
 44592| 
 44593| 1476: ; preds = %1471
 44594|     ;; layout[0..+8] = i64 8
 44595|     ;; layout[8..+8] = i64 %1474
 44596|  %1477 = gep %65, i64 8                                                                                                ;L704<714<825<825<276
 44597|  %1478 = load ptr, ptr %1477, , !!8, !!8                                                                               ;L704<714<825<825<276
 44598|  %1479 = load ptr, ptr %65, , !!8, !!8                                                                                 ;L704<714<825<825<276
 44599|  %1480 = gep %1478, i64 16                                                                                             ;L704<714<825<825<276
 44600|  %1481 = load ptr, ptr %1480, , !!50576, !!8, !!8                                                                      ;L704<714<825<825<276
 44603|     ;; ptr = ptr %1479
 44604|     ;; ptr = ptr %1479
 44605|     ;; layout[0..+8] = i64 8
 44606|     ;; layout[8..+8] = i64 %1474
 44607|     ;; footer = ptr %1481
 44608|     ;; footer = ptr %1481
 44609|     ;; self = ptr %1481
 44610|  %1482 = gep %1481, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<276
 44611|  %1483 = load ptr, ptr %1482, , !!50576, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<276
 44614|     ;; self = ptr %1479
 44615|  %1484 = icmp eq ptr %1483, %1479                                                                                      ;L1714<1700<1707<704<714<825<825<276
 44616|  br i1 %1484, label %1485, label %1488                                                                                 ;L1707<704<714<825<825<276
 44617| 
 44618| 1485: ; preds = %1476
 44619|  %1486 = mul i64 %1474, 184                                                                                            ;L166<703<714<825<825<276
 44620|     ;; layout[8..+8] = i64 %1486
 44621|     ;; layout[8..+8] = i64 %1486
 44622|     ;; count = i64 %1486
 44623|  %1487 = gep %1479, i64 %1486                                                                                          ;L961<1708<704<714<825<825<276
 44624|     ;; ptr = ptr %1487
 44625|     ;; val = ptr %1487
 44626|     ;; val = ptr %1487
 44627|     ;; self = ptr %1481
 44628|     ;; self = ptr %1481
 44629|  store ptr %1487, ptr %1482, , !!50576                                                                                 ;L931<513<437<1709<704<714<825<825<276
 44630|  br label %1488                                                                                                        ;L1707<704<714<825<825<276
 44631| 
 44632| 1488: ; preds = %1485, %1476, %1471
 44633|  cleanupret from %1472 unwind to caller                                                                                ;L825<276
 44634| 
 44635| 1489: ; preds = %1470
 44639|     ;; self = ptr %65
 44641|     ;; self = ptr %65
 44642|     ;; self = ptr %65
 44643|     ;; elem_size = i64 184
 44644|     ;; align = i64 8
 44645|  %1490 = gep %65, i64 16                                                                                               ;L159<703<714<825<825<276
 44646|  %1491 = load i64, ptr %1490, , !!8                                                                                    ;L159<703<714<825<825<276
 44647|  %1492 = icmp eq i64 %1491, 0                                                                                          ;L159<703<714<825<825<276
 44648|  br i1 %1492, label %1505, label %1493                                                                                 ;L159<703<714<825<825<276
 44649| 
 44650| 1493: ; preds = %1489
 44651|     ;; layout[0..+8] = i64 8
 44652|     ;; layout[8..+8] = i64 %1491
 44653|  %1494 = gep %65, i64 8                                                                                                ;L704<714<825<825<276
 44654|  %1495 = load ptr, ptr %1494, , !!8, !!8                                                                               ;L704<714<825<825<276
 44655|  %1496 = load ptr, ptr %65, , !!8, !!8                                                                                 ;L704<714<825<825<276
 44656|  %1497 = gep %1495, i64 16                                                                                             ;L704<714<825<825<276
 44657|  %1498 = load ptr, ptr %1497, , !!50629, !!8, !!8                                                                      ;L704<714<825<825<276
 44660|     ;; ptr = ptr %1496
 44661|     ;; ptr = ptr %1496
 44662|     ;; layout[0..+8] = i64 8
 44663|     ;; layout[8..+8] = i64 %1491
 44664|     ;; footer = ptr %1498
 44665|     ;; footer = ptr %1498
 44666|     ;; self = ptr %1498
 44667|  %1499 = gep %1498, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<276
 44668|  %1500 = load ptr, ptr %1499, , !!50629, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<276
 44671|     ;; self = ptr %1496
 44672|  %1501 = icmp eq ptr %1500, %1496                                                                                      ;L1714<1700<1707<704<714<825<825<276
 44673|  br i1 %1501, label %1502, label %1505                                                                                 ;L1707<704<714<825<825<276
 44674| 
 44675| 1502: ; preds = %1493
 44676|  %1503 = mul i64 %1491, 184                                                                                            ;L166<703<714<825<825<276
 44677|     ;; layout[8..+8] = i64 %1503
 44678|     ;; layout[8..+8] = i64 %1503
 44679|     ;; count = i64 %1503
 44680|  %1504 = gep %1496, i64 %1503                                                                                          ;L961<1708<704<714<825<825<276
 44681|     ;; ptr = ptr %1504
 44682|     ;; val = ptr %1504
 44683|     ;; val = ptr %1504
 44684|     ;; self = ptr %1498
 44685|     ;; self = ptr %1498
 44686|  store ptr %1504, ptr %1499, , !!50629                                                                                 ;L931<513<437<1709<704<714<825<825<276
 44687|  br label %1505                                                                                                        ;L1707<704<714<825<825<276
 44688| 
 44689| 1505: ; preds = %1502, %1493, %1489
 44691|  br label %80                                                                                                          ;L276
 44692| 
 44693| 1506: ; preds = %1265
 44694|  call void @llvm.memcpy.p0.p0.i64(ptr %55, ptr %54, i64 136, i1 false)                                                 ;L234
 44696|  %1507 = gep %55, i64 177                                                                                              ;L234
 44697|  store i8 3, ptr %1507,                                                                                                ;L234
 44698|  invoke void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayE12from_iter_inABU_j1_EBY_(ptr sret([32 x i8]) %0, ptr %55, ptr %183)
 44699|  to label %1508 unwind label %1106                                                                                     ;L234
 44700| 
 44701| 1508: ; preds = %1506
 44704|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %58)
 44705|  to label %1527 unwind label %1509                                                                                     ;L825<275
 44706| 
 44707| 1509: ; preds = %1508
 44708|  %1510 = cleanuppad within none []
 44712|     ;; self = ptr %58
 44714|     ;; self = ptr %58
 44715|     ;; self = ptr %58
 44716|     ;; elem_size = i64 184
 44717|     ;; align = i64 8
 44718|  %1511 = gep %58, i64 16                                                                                               ;L159<703<714<825<825<275
 44719|  %1512 = load i64, ptr %1511, , !!8                                                                                    ;L159<703<714<825<825<275
 44720|  %1513 = icmp eq i64 %1512, 0                                                                                          ;L159<703<714<825<825<275
 44721|  br i1 %1513, label %1526, label %1514                                                                                 ;L159<703<714<825<825<275
 44722| 
 44723| 1514: ; preds = %1509
 44724|     ;; layout[0..+8] = i64 8
 44725|     ;; layout[8..+8] = i64 %1512
 44726|  %1515 = gep %58, i64 8                                                                                                ;L704<714<825<825<275
 44727|  %1516 = load ptr, ptr %1515, , !!8, !!8                                                                               ;L704<714<825<825<275
 44728|  %1517 = load ptr, ptr %58, , !!8, !!8                                                                                 ;L704<714<825<825<275
 44729|  %1518 = gep %1516, i64 16                                                                                             ;L704<714<825<825<275
 44730|  %1519 = load ptr, ptr %1518, , !!50687, !!8, !!8                                                                      ;L704<714<825<825<275
 44733|     ;; ptr = ptr %1517
 44734|     ;; ptr = ptr %1517
 44735|     ;; layout[0..+8] = i64 8
 44736|     ;; layout[8..+8] = i64 %1512
 44737|     ;; footer = ptr %1519
 44738|     ;; footer = ptr %1519
 44739|     ;; self = ptr %1519
 44740|  %1520 = gep %1519, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<275
 44741|  %1521 = load ptr, ptr %1520, , !!50687, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<275
 44744|     ;; self = ptr %1517
 44745|  %1522 = icmp eq ptr %1521, %1517                                                                                      ;L1714<1700<1707<704<714<825<825<275
 44746|  br i1 %1522, label %1523, label %1526                                                                                 ;L1707<704<714<825<825<275
 44747| 
 44748| 1523: ; preds = %1514
 44749|  %1524 = mul i64 %1512, 184                                                                                            ;L166<703<714<825<825<275
 44750|     ;; layout[8..+8] = i64 %1524
 44751|     ;; layout[8..+8] = i64 %1524
 44752|     ;; count = i64 %1524
 44753|  %1525 = gep %1517, i64 %1524                                                                                          ;L961<1708<704<714<825<825<275
 44754|     ;; ptr = ptr %1525
 44755|     ;; val = ptr %1525
 44756|     ;; val = ptr %1525
 44757|     ;; self = ptr %1519
 44758|     ;; self = ptr %1519
 44759|  store ptr %1525, ptr %1520, , !!50687                                                                                 ;L931<513<437<1709<704<714<825<825<275
 44760|  br label %1526                                                                                                        ;L1707<704<714<825<825<275
 44761| 
 44762| 1526: ; preds = %1523, %1514, %1509
 44763|  cleanupret from %1510 unwind label %188
 44764| 
 44765| 1527: ; preds = %1508
 44769|     ;; self = ptr %58
 44771|     ;; self = ptr %58
 44772|     ;; self = ptr %58
 44773|     ;; elem_size = i64 184
 44774|     ;; align = i64 8
 44775|  %1528 = gep %58, i64 16                                                                                               ;L159<703<714<825<825<275
 44776|  %1529 = load i64, ptr %1528, , !!8                                                                                    ;L159<703<714<825<825<275
 44777|  %1530 = icmp eq i64 %1529, 0                                                                                          ;L159<703<714<825<825<275
 44778|  br i1 %1530, label %1543, label %1531                                                                                 ;L159<703<714<825<825<275
 44779| 
 44780| 1531: ; preds = %1527
 44781|     ;; layout[0..+8] = i64 8
 44782|     ;; layout[8..+8] = i64 %1529
 44783|  %1532 = gep %58, i64 8                                                                                                ;L704<714<825<825<275
 44784|  %1533 = load ptr, ptr %1532, , !!8, !!8                                                                               ;L704<714<825<825<275
 44785|  %1534 = load ptr, ptr %58, , !!8, !!8                                                                                 ;L704<714<825<825<275
 44786|  %1535 = gep %1533, i64 16                                                                                             ;L704<714<825<825<275
 44787|  %1536 = load ptr, ptr %1535, , !!50740, !!8, !!8                                                                      ;L704<714<825<825<275
 44790|     ;; ptr = ptr %1534
 44791|     ;; ptr = ptr %1534
 44792|     ;; layout[0..+8] = i64 8
 44793|     ;; layout[8..+8] = i64 %1529
 44794|     ;; footer = ptr %1536
 44795|     ;; footer = ptr %1536
 44796|     ;; self = ptr %1536
 44797|  %1537 = gep %1536, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<275
 44798|  %1538 = load ptr, ptr %1537, , !!50740, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<275
 44801|     ;; self = ptr %1534
 44802|  %1539 = icmp eq ptr %1538, %1534                                                                                      ;L1714<1700<1707<704<714<825<825<275
 44803|  br i1 %1539, label %1540, label %1543                                                                                 ;L1707<704<714<825<825<275
 44804| 
 44805| 1540: ; preds = %1531
 44806|  %1541 = mul i64 %1529, 184                                                                                            ;L166<703<714<825<825<275
 44807|     ;; layout[8..+8] = i64 %1541
 44808|     ;; layout[8..+8] = i64 %1541
 44809|     ;; count = i64 %1541
 44810|  %1542 = gep %1534, i64 %1541                                                                                          ;L961<1708<704<714<825<825<275
 44811|     ;; ptr = ptr %1542
 44812|     ;; val = ptr %1542
 44813|     ;; val = ptr %1542
 44814|     ;; self = ptr %1536
 44815|     ;; self = ptr %1536
 44816|  store ptr %1542, ptr %1537, , !!50740                                                                                 ;L931<513<437<1709<704<714<825<825<275
 44817|  br label %1543                                                                                                        ;L1707<704<714<825<825<275
 44818| 
 44819| 1543: ; preds = %1540, %1531, %1527
 44822|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %62)
 44823|  to label %1562 unwind label %1544                                                                                     ;L825<275
 44824| 
 44825| 1544: ; preds = %1543
 44826|  %1545 = cleanuppad within none []
 44830|     ;; self = ptr %62
 44832|     ;; self = ptr %62
 44833|     ;; self = ptr %62
 44834|     ;; elem_size = i64 184
 44835|     ;; align = i64 8
 44836|  %1546 = gep %62, i64 16                                                                                               ;L159<703<714<825<825<275
 44837|  %1547 = load i64, ptr %1546, , !!8                                                                                    ;L159<703<714<825<825<275
 44838|  %1548 = icmp eq i64 %1547, 0                                                                                          ;L159<703<714<825<825<275
 44839|  br i1 %1548, label %1561, label %1549                                                                                 ;L159<703<714<825<825<275
 44840| 
 44841| 1549: ; preds = %1544
 44842|     ;; layout[0..+8] = i64 8
 44843|     ;; layout[8..+8] = i64 %1547
 44844|  %1550 = gep %62, i64 8                                                                                                ;L704<714<825<825<275
 44845|  %1551 = load ptr, ptr %1550, , !!8, !!8                                                                               ;L704<714<825<825<275
 44846|  %1552 = load ptr, ptr %62, , !!8, !!8                                                                                 ;L704<714<825<825<275
 44847|  %1553 = gep %1551, i64 16                                                                                             ;L704<714<825<825<275
 44848|  %1554 = load ptr, ptr %1553, , !!50798, !!8, !!8                                                                      ;L704<714<825<825<275
 44851|     ;; ptr = ptr %1552
 44852|     ;; ptr = ptr %1552
 44853|     ;; layout[0..+8] = i64 8
 44854|     ;; layout[8..+8] = i64 %1547
 44855|     ;; footer = ptr %1554
 44856|     ;; footer = ptr %1554
 44857|     ;; self = ptr %1554
 44858|  %1555 = gep %1554, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<275
 44859|  %1556 = load ptr, ptr %1555, , !!50798, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<275
 44862|     ;; self = ptr %1552
 44863|  %1557 = icmp eq ptr %1556, %1552                                                                                      ;L1714<1700<1707<704<714<825<825<275
 44864|  br i1 %1557, label %1558, label %1561                                                                                 ;L1707<704<714<825<825<275
 44865| 
 44866| 1558: ; preds = %1549
 44867|  %1559 = mul i64 %1547, 184                                                                                            ;L166<703<714<825<825<275
 44868|     ;; layout[8..+8] = i64 %1559
 44869|     ;; layout[8..+8] = i64 %1559
 44870|     ;; count = i64 %1559
 44871|  %1560 = gep %1552, i64 %1559                                                                                          ;L961<1708<704<714<825<825<275
 44872|     ;; ptr = ptr %1560
 44873|     ;; val = ptr %1560
 44874|     ;; val = ptr %1560
 44875|     ;; self = ptr %1554
 44876|     ;; self = ptr %1554
 44877|  store ptr %1560, ptr %1555, , !!50798                                                                                 ;L931<513<437<1709<704<714<825<825<275
 44878|  br label %1561                                                                                                        ;L1707<704<714<825<825<275
 44879| 
 44880| 1561: ; preds = %1558, %1549, %1544
 44881|  cleanupret from %1545 unwind label %82
 44882| 
 44883| 1562: ; preds = %1543
 44887|     ;; self = ptr %62
 44889|     ;; self = ptr %62
 44890|     ;; self = ptr %62
 44891|     ;; elem_size = i64 184
 44892|     ;; align = i64 8
 44893|  %1563 = gep %62, i64 16                                                                                               ;L159<703<714<825<825<275
 44894|  %1564 = load i64, ptr %1563, , !!8                                                                                    ;L159<703<714<825<825<275
 44895|  %1565 = icmp eq i64 %1564, 0                                                                                          ;L159<703<714<825<825<275
 44896|  br i1 %1565, label %1578, label %1566                                                                                 ;L159<703<714<825<825<275
 44897| 
 44898| 1566: ; preds = %1562
 44899|     ;; layout[0..+8] = i64 8
 44900|     ;; layout[8..+8] = i64 %1564
 44901|  %1567 = gep %62, i64 8                                                                                                ;L704<714<825<825<275
 44902|  %1568 = load ptr, ptr %1567, , !!8, !!8                                                                               ;L704<714<825<825<275
 44903|  %1569 = load ptr, ptr %62, , !!8, !!8                                                                                 ;L704<714<825<825<275
 44904|  %1570 = gep %1568, i64 16                                                                                             ;L704<714<825<825<275
 44905|  %1571 = load ptr, ptr %1570, , !!50851, !!8, !!8                                                                      ;L704<714<825<825<275
 44908|     ;; ptr = ptr %1569
 44909|     ;; ptr = ptr %1569
 44910|     ;; layout[0..+8] = i64 8
 44911|     ;; layout[8..+8] = i64 %1564
 44912|     ;; footer = ptr %1571
 44913|     ;; footer = ptr %1571
 44914|     ;; self = ptr %1571
 44915|  %1572 = gep %1571, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<275
 44916|  %1573 = load ptr, ptr %1572, , !!50851, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<275
 44919|     ;; self = ptr %1569
 44920|  %1574 = icmp eq ptr %1573, %1569                                                                                      ;L1714<1700<1707<704<714<825<825<275
 44921|  br i1 %1574, label %1575, label %1578                                                                                 ;L1707<704<714<825<825<275
 44922| 
 44923| 1575: ; preds = %1566
 44924|  %1576 = mul i64 %1564, 184                                                                                            ;L166<703<714<825<825<275
 44925|     ;; layout[8..+8] = i64 %1576
 44926|     ;; layout[8..+8] = i64 %1576
 44927|     ;; count = i64 %1576
 44928|  %1577 = gep %1569, i64 %1576                                                                                          ;L961<1708<704<714<825<825<275
 44929|     ;; ptr = ptr %1577
 44930|     ;; val = ptr %1577
 44931|     ;; val = ptr %1577
 44932|     ;; self = ptr %1571
 44933|     ;; self = ptr %1571
 44934|  store ptr %1577, ptr %1572, , !!50851                                                                                 ;L931<513<437<1709<704<714<825<825<275
 44935|  br label %1578                                                                                                        ;L1707<704<714<825<825<275
 44936| 
 44937| 1578: ; preds = %1575, %1566, %1562
 44941|  invoke void @ai::small_action15SmallActionPlayENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB10_(ptr %65)
 44942|  to label %1597 unwind label %1579                                                                                     ;L825<276
 44943| 
 44944| 1579: ; preds = %1578
 44945|  %1580 = cleanuppad within none []
 44949|     ;; self = ptr %65
 44951|     ;; self = ptr %65
 44952|     ;; self = ptr %65
 44953|     ;; elem_size = i64 184
 44954|     ;; align = i64 8
 44955|  %1581 = gep %65, i64 16                                                                                               ;L159<703<714<825<825<276
 44956|  %1582 = load i64, ptr %1581, , !!8                                                                                    ;L159<703<714<825<825<276
 44957|  %1583 = icmp eq i64 %1582, 0                                                                                          ;L159<703<714<825<825<276
 44958|  br i1 %1583, label %1596, label %1584                                                                                 ;L159<703<714<825<825<276
 44959| 
 44960| 1584: ; preds = %1579
 44961|     ;; layout[0..+8] = i64 8
 44962|     ;; layout[8..+8] = i64 %1582
 44963|  %1585 = gep %65, i64 8                                                                                                ;L704<714<825<825<276
 44964|  %1586 = load ptr, ptr %1585, , !!8, !!8                                                                               ;L704<714<825<825<276
 44965|  %1587 = load ptr, ptr %65, , !!8, !!8                                                                                 ;L704<714<825<825<276
 44966|  %1588 = gep %1586, i64 16                                                                                             ;L704<714<825<825<276
 44967|  %1589 = load ptr, ptr %1588, , !!50909, !!8, !!8                                                                      ;L704<714<825<825<276
 44970|     ;; ptr = ptr %1587
 44971|     ;; ptr = ptr %1587
 44972|     ;; layout[0..+8] = i64 8
 44973|     ;; layout[8..+8] = i64 %1582
 44974|     ;; footer = ptr %1589
 44975|     ;; footer = ptr %1589
 44976|     ;; self = ptr %1589
 44977|  %1590 = gep %1589, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<276
 44978|  %1591 = load ptr, ptr %1590, , !!50909, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<276
 44981|     ;; self = ptr %1587
 44982|  %1592 = icmp eq ptr %1591, %1587                                                                                      ;L1714<1700<1707<704<714<825<825<276
 44983|  br i1 %1592, label %1593, label %1596                                                                                 ;L1707<704<714<825<825<276
 44984| 
 44985| 1593: ; preds = %1584
 44986|  %1594 = mul i64 %1582, 184                                                                                            ;L166<703<714<825<825<276
 44987|     ;; layout[8..+8] = i64 %1594
 44988|     ;; layout[8..+8] = i64 %1594
 44989|     ;; count = i64 %1594
 44990|  %1595 = gep %1587, i64 %1594                                                                                          ;L961<1708<704<714<825<825<276
 44991|     ;; ptr = ptr %1595
 44992|     ;; val = ptr %1595
 44993|     ;; val = ptr %1595
 44994|     ;; self = ptr %1589
 44995|     ;; self = ptr %1589
 44996|  store ptr %1595, ptr %1590, , !!50909                                                                                 ;L931<513<437<1709<704<714<825<825<276
 44997|  br label %1596                                                                                                        ;L1707<704<714<825<825<276
 44998| 
 44999| 1596: ; preds = %1593, %1584, %1579
 45000|  cleanupret from %1580 unwind to caller                                                                                ;L825<276
 45001| 
 45002| 1597: ; preds = %1578
 45006|     ;; self = ptr %65
 45008|     ;; self = ptr %65
 45009|     ;; self = ptr %65
 45010|     ;; elem_size = i64 184
 45011|     ;; align = i64 8
 45012|  %1598 = gep %65, i64 16                                                                                               ;L159<703<714<825<825<276
 45013|  %1599 = load i64, ptr %1598, , !!8                                                                                    ;L159<703<714<825<825<276
 45014|  %1600 = icmp eq i64 %1599, 0                                                                                          ;L159<703<714<825<825<276
 45015|  br i1 %1600, label %1613, label %1601                                                                                 ;L159<703<714<825<825<276
 45016| 
 45017| 1601: ; preds = %1597
 45018|     ;; layout[0..+8] = i64 8
 45019|     ;; layout[8..+8] = i64 %1599
 45020|  %1602 = gep %65, i64 8                                                                                                ;L704<714<825<825<276
 45021|  %1603 = load ptr, ptr %1602, , !!8, !!8                                                                               ;L704<714<825<825<276
 45022|  %1604 = load ptr, ptr %65, , !!8, !!8                                                                                 ;L704<714<825<825<276
 45023|  %1605 = gep %1603, i64 16                                                                                             ;L704<714<825<825<276
 45024|  %1606 = load ptr, ptr %1605, , !!50962, !!8, !!8                                                                      ;L704<714<825<825<276
 45027|     ;; ptr = ptr %1604
 45028|     ;; ptr = ptr %1604
 45029|     ;; layout[0..+8] = i64 8
 45030|     ;; layout[8..+8] = i64 %1599
 45031|     ;; footer = ptr %1606
 45032|     ;; footer = ptr %1606
 45033|     ;; self = ptr %1606
 45034|  %1607 = gep %1606, i64 32                                                                                             ;L2447<555<1700<1707<704<714<825<825<276
 45035|  %1608 = load ptr, ptr %1607, , !!50962, !!8, !!8                                                                      ;L555<1700<1707<704<714<825<825<276
 45038|     ;; self = ptr %1604
 45039|  %1609 = icmp eq ptr %1608, %1604                                                                                      ;L1714<1700<1707<704<714<825<825<276
 45040|  br i1 %1609, label %1610, label %1613                                                                                 ;L1707<704<714<825<825<276
 45041| 
 45042| 1610: ; preds = %1601
 45043|  %1611 = mul i64 %1599, 184                                                                                            ;L166<703<714<825<825<276
 45044|     ;; layout[8..+8] = i64 %1611
 45045|     ;; layout[8..+8] = i64 %1611
 45046|     ;; count = i64 %1611
 45047|  %1612 = gep %1604, i64 %1611                                                                                          ;L961<1708<704<714<825<825<276
 45048|     ;; ptr = ptr %1612
 45049|     ;; val = ptr %1612
 45050|     ;; val = ptr %1612
 45051|     ;; self = ptr %1606
 45052|     ;; self = ptr %1606
 45053|  store ptr %1612, ptr %1607, , !!50962                                                                                 ;L931<513<437<1709<704<714<825<825<276
 45054|  br label %1613                                                                                                        ;L1707<704<714<825<825<276
 45055| 
 45056| 1613: ; preds = %1610, %1601, %1597
 45058|  br label %80                                                                                                          ;L1
 45059| 
 45060| 1614: ; preds = %1169, %1160, %1156
 45062|  br label %1470                                                                                                        ;L275
 45063| 
 45064| 1615: ; preds = %188
 45065|  cleanupret from %190 unwind label %82
 45066| 
 45067| 1616: ; preds = %188
 45068|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %62) #30 [ "funclet"(token %190) ] ;L275
 45069|  cleanupret from %190 unwind label %82                                                                                 ;L275
 45070| }
