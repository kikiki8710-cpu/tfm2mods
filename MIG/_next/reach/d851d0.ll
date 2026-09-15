 24749| define internal fastcc void @ai::position_eval25position_eval_at_uncached(ptr %0, i64 %1, ptr %2, ptr %3, i64 %4, i64 %5, i8 %6) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 24750|  %8 = alloca [120 x i8],
 24751|  %9 = alloca [8 x i8],
 24752|  %10 = alloca [8 x i8],
 24753|  %11 = alloca [8 x i8],
 24754|  %12 = alloca [8 x i8],
 24755|  %13 = alloca [8 x i8],
 24756|  %14 = alloca [8 x i8],
 24757|  %15 = alloca [8 x i8],
 24758|  %16 = alloca [1 x i8],
 24759|  %17 = alloca [8 x i8],
 24760|  %18 = alloca [120 x i8],
 24761|  %19 = alloca [8 x i8],
 24762|  %20 = alloca [8 x i8],
 24763|  %21 = alloca [8 x i8],
 24764|  %22 = alloca [8 x i8],
 24765|  %23 = alloca [8 x i8],
 24766|  %24 = alloca [8 x i8],
 24767|  %25 = alloca [8 x i8],
 24768|  %26 = alloca [1 x i8],
 24769|  %27 = alloca [8 x i8],
 24770|  %28 = alloca [48 x i8],
 24771|  %29 = alloca [16 x i8],
 24772|  %30 = alloca [8 x i8],
 24773|  %31 = alloca [8 x i8],
 24774|  %32 = alloca [48 x i8],
 24775|  %33 = alloca [16 x i8],
 24776|  %34 = alloca [8 x i8],
 24777|  %35 = alloca [8 x i8],
 24778|  %36 = alloca [48 x i8],
 24779|  %37 = alloca [16 x i8],
 24780|  %38 = alloca [8 x i8],
 24781|  %39 = alloca [8 x i8],
 24782|  %40 = alloca [48 x i8],
 24783|  %41 = alloca [16 x i8],
 24784|  %42 = alloca [8 x i8],
 24785|  %43 = alloca [8 x i8],
 24786|  %44 = alloca [48 x i8],
 24787|  %45 = alloca [16 x i8],
 24788|  %46 = alloca [8 x i8],
 24789|  %47 = alloca [8 x i8],
 24790|  %48 = alloca [8 x i8],
 24791|  %49 = alloca [8 x i8],
 24792|  %50 = alloca [48 x i8],
 24793|  %51 = alloca [16 x i8],
 24794|  %52 = alloca [8 x i8],
 24795|  %53 = alloca [8 x i8],
 24796|  %54 = alloca [8 x i8],
 24797|  %55 = alloca [48 x i8],
 24798|  %56 = alloca [16 x i8],
 24799|  %57 = alloca [8 x i8],
 24800|  %58 = alloca [8 x i8],
 24801|  %59 = alloca [48 x i8],
 24802|  %60 = alloca [16 x i8],
 24803|  %61 = alloca [8 x i8],
 24804|  %62 = alloca [8 x i8],
 24805|  %63 = alloca [48 x i8],
 24806|  %64 = alloca [8 x i8],
 24807|  %65 = alloca [8 x i8],
 24808|  %66 = alloca [8 x i8],
 24809|  %67 = alloca [56 x i8],
 24810|  %68 = alloca [8 x i8],
 24811|  %69 = alloca [8 x i8],
 24812|  %70 = alloca [8 x i8],
 24813|  %71 = alloca [8 x i8],
 24814|  %72 = alloca [24 x i8],
 24818|  %73 = alloca [24 x i8],
 24819|  %74 = alloca [24 x i8],
 24826|  %75 = alloca [24 x i8],
 24827|  %76 = alloca [24 x i8],
 24837|  %77 = alloca [40 x i8],
 24838|  %78 = alloca [40 x i8],
 24839|  %79 = alloca [24 x i8],
 24842|  %80 = alloca [24 x i8],
 24843|  %81 = alloca [24 x i8],
 24848|  %82 = alloca [64 x i8],
 24849|  %83 = alloca [56 x i8],
 24855|  %84 = alloca [296 x i8],
 24856|  %85 = alloca [120 x i8],
 24859|  %86 = alloca [120 x i8],
 24860|  %87 = alloca [112 x i8],
 24861|     ;; self[8..+112] = ptr %87
 24866|  %88 = alloca [8 x i8],
 24868|  %89 = alloca [24 x i8],
 24869|  %90 = alloca [24 x i8],
 24870|  %91 = alloca [40 x i8],
 24871|     ;; src[0..+424] = ptr %92
 24872|     ;; value[0..+424] = ptr %92
 24873|  %92 = alloca [424 x i8],
 24877|  %93 = alloca [32 x i8],
 24878|     ;; src[0..+424] = ptr %94
 24879|     ;; value[0..+424] = ptr %94
 24880|  %94 = alloca [424 x i8],
 24884|  %95 = alloca [32 x i8],
 24885|  %96 = alloca [24 x i8],
 24886|  %97 = alloca [16 x i8],
 24887|  %98 = alloca [24 x i8],
 24888|  %99 = alloca [8 x i8],
 24889|  %100 = alloca [8 x i8],
 24891|     ;; version = i64 %1
 24892|  store i64 %4, ptr %100,
 24893|  store i64 %5, ptr %99,
 24894|     ;; score = ptr %0
 24895|     ;; player = ptr %2
 24896|     ;; data = ptr %3
 24897|     ;; x = ptr %100
 24898|     ;; x1 = ptr %100
 24899|     ;; self = ptr %100
 24900|     ;; x1 = ptr %100
 24901|     ;; self = ptr %100
 24902|     ;; y = ptr %99
 24903|     ;; y1 = ptr %99
 24904|     ;; self = ptr %99
 24905|     ;; y1 = ptr %99
 24906|     ;; self = ptr %99
 24907|     ;; purpose = i8 %6
 24908|     ;; _t = ptr %98
 24909|     ;; cell_dist_sq = ptr %97
 24910|     ;; _t_peu1 = ptr %96
 24911|     ;; near_enemies_with_action = ptr %95
 24912|     ;; a = ptr %94
 24913|     ;; near_allies_with_action = ptr %93
 24914|     ;; a = ptr %92
 24915|     ;; pe_ctx = ptr %91
 24916|     ;; _x = ptr %90
 24917|     ;; _t_peu2 = ptr %89
 24918|     ;; player_team = ptr %88
 24919|     ;; self = ptr %86
 24920|     ;; self = ptr %85
 24921|     ;; iter = ptr %84
 24922|     ;; self = ptr %83
 24923|     ;; iter = ptr %82
 24924|     ;; _x = ptr %81
 24925|     ;; _t_peu3 = ptr %80
 24926|     ;; _t_proj = ptr %79
 24927|     ;; iter = ptr %77
 24928|     ;; _x = ptr %76
 24929|     ;; _t_peu4 = ptr %75
 24930|     ;; _x = ptr %74
 24931|     ;; _t_peu5 = ptr %73
 24932|     ;; _x = ptr %72
 24933|     ;; phase = i64 48
 24934|     ;; order = i8 0
 24935|     ;; phase = i64 112
 24936|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 24937|     ;; order = i8 0
 24938|     ;; n = i64 1
 24939|     ;; rhs = i64 1
 24940|     ;; n = i64 1
 24941|     ;; rhs = i64 1
 24942|     ;; phase = i64 113
 24943|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 24944|     ;; order = i8 0
 24945|     ;; count = i64 1
 24947|     ;; phase = i64 114
 24948|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 24949|     ;; order = i8 0
 24950|     ;; count = i64 1
 24951|     ;; phase = i64 73
 24952|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 24953|     ;; order = i8 0
 24955|     ;; count = i64 1
 24956|     ;; count = i64 1
 24957|     ;; count = i64 1
 24958|     ;; count = i64 1
 24959|     ;; count = i64 1
 24960|     ;; count = i64 1
 24961|     ;; count = i64 1
 24962|     ;; count = i64 1
 24963|     ;; phase = i64 115
 24964|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 24965|     ;; order = i8 0
 24966|     ;; count = i64 1
 24967|     ;; init[0..+8] = i64 0
 24968|     ;; init[8..+8] = i64 0
 24970|     ;; phase = i64 116
 24971|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 24972|     ;; order = i8 0
 24973|     ;; count = i64 1
 24974|     ;; count = i64 1
 24975|     ;; count = i64 1
 24976|     ;; rhs = i64 -7046029254386353131
 24977|     ;; rhs = i64 -7046029254386353131
 24978|     ;; rhs = i64 -7046029254386353131
 24980|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 24981|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 24982|     ;; order = i8 0
 24983|  %101 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                    ;L3904<741<176<373
 24984|  %102 = icmp eq i8 %101, 0                                                                                             ;L176<373
 24985|  br i1 %102, label %108, label %103                                                                                    ;L176<373
 24986| 
 24987| 103: ; preds = %7
 24988|  %104 = tail call { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()                                      ;L179<373
 24989|  %105 = extractvalue { i64, i32 } %104, 0                                                                              ;L179<373
 24990|  %106 = extractvalue { i64, i32 } %104, 1                                                                              ;L179<373
 24991|  store i64 48, ptr %98,                                                                                                ;L179<373
 24992|  %107 = gep %98, i64 8                                                                                                 ;L179<373
 24993|  store i64 %105, ptr %107,                                                                                             ;L179<373
 24994|  br label %108                                                                                                         ;L180<373
 24995| 
 24996| 108: ; preds = %103, %7
 24997|  %109 = phi i32 [ %106, %103 ], [ -1, %7 ]                                                                             ;L0<373
 24998|  %110 = gep %98, i64 16                                                                                                ;L0<373
 24999|  store i32 %109, ptr %110,                                                                                             ;L0<373
 25000|  %111 = gep %2, i64 2352                                                                                               ;L374
 25001|  %112 = load i64, ptr %111, , !!8                                                                                      ;L374
 25002|  %113 = icmp ult i64 %112, 2                                                                                           ;L374
 25003|  br i1 %113, label %114, label %124                                                                                    ;L374
 25004| 
 25005| 114: ; preds = %108
 25006|     ;; self = ptr %2
 25007|  %115 = gep %2, i64 2496                                                                                               ;L581<374
 25008|  %116 = load i32, ptr %115, , !!8                                                                                      ;L581<374
 25009|  %117 = zext nneg i32 %116 to i64                                                                                      ;L581<374
 25010|  %118 = load ptr, ptr %3, , !!8, !!8                                                                                   ;L374
 25011|  %119 = gep %118, i64 480                                                                                              ;L374
 25012|  %120 = getelementptr [5 x ptr], ptr %119, i64 %112                                                                    ;L374
 25013|  %121 = getelementptr ptr, ptr %120, i64 %117                                                                          ;L374
 25014|  %122 = load ptr, ptr %121, , !!8                                                                                      ;L374
 25015|  %123 = icmp eq ptr %122, null                                                                                         ;L374
 25016|  br i1 %123, label %128, label %129                                                                                    ;L374
 25017| 
 25018| 124: ; preds = %108
 25019|  invoke void @core::panicking18panic_bounds_check(i64 %112, i64 2, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.98) #25
 25020|  to label %127 unwind label %125                                                                                       ;L374
 25021| 
 25022| 125: ; preds = %4101, %4100, %192, %178, %174, %162, %157, %145, %124
 25023|  %126 = cleanuppad within none []
 25024|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %98) #27 [ "funclet"(token %126) ] ;L1181
 25025|  cleanupret from %126 unwind to caller                                                                                 ;L371
 25026| 
 25027| 127: ; preds = %4062, %4061, %4048, %3979, %3978, %3960, %3909, %3685, %3644, %595, %529, %445, %124
 25028|  unreachable
 25029| 
 25030| 128: ; preds = %114
 25031|  call void @llvm.memset.p0.i64(ptr %0, i8 0, i64 50, i1 false)                                                         ;L367<375
 25032|  br label %4102                                                                                                        ;L375
 25033| 
 25034| 129: ; preds = %114
 25035|     ;; champ = ptr %122
 25036|     ;; self = ptr %122
 25037|     ;; self = ptr %122
 25038|     ;; self = ptr %122
 25039|     ;; self = ptr %122
 25040|     ;; self = ptr %122
 25041|     ;; self = ptr %122
 25042|     ;; self = ptr %122
 25043|     ;; self = ptr %122
 25044|     ;; self = ptr %122
 25045|     ;; self = ptr %122
 25046|     ;; self = ptr %122
 25047|     ;; self = ptr %122
 25048|     ;; self = ptr %122
 25049|     ;; self = ptr %122
 25050|     ;; self = ptr %122
 25051|     ;; self = ptr %122
 25052|     ;; self = ptr %122
 25053|     ;; self = ptr %122
 25054|     ;; self = ptr %122
 25055|     ;; self = ptr %122
 25056|     ;; self = ptr %122
 25057|     ;; self = ptr %122
 25058|     ;; self = ptr %122
 25059|  %130 = udiv i64 %4, 32000                                                                                             ;L378
 25060|     ;; self = i64 %130
 25061|     ;; min = i64 0
 25062|     ;; max = i64 29
 25063|  %131 = tail call i64 @llvm.umin.i64(i64 %130, i64 29)                                                                 ;L2027<378
 25064|     ;; xi = i64 %131
 25065|  %132 = udiv i64 %5, 32000                                                                                             ;L379
 25066|     ;; self = i64 %132
 25067|     ;; min = i64 0
 25068|     ;; max = i64 29
 25069|  %133 = tail call i64 @llvm.umin.i64(i64 %132, i64 29)                                                                 ;L2027<379
 25070|     ;; yi = i64 %133
 25071|  %134 = gep %3, i64 8                                                                                                  ;L380
 25072|  %135 = load ptr, ptr %134, , !!8, !!8                                                                                 ;L380
 25073|  %136 = gep %135, i64 32                                                                                               ;L380
 25074|  %137 = load ptr, ptr %136, , !!8, !!8                                                                                 ;L380
 25075|  %138 = gep %137, i64 120                                                                                              ;L380
 25076|  %139 = getelementptr [30 x i64], ptr %138, i64 %133                                                                   ;L380
 25077|  %140 = getelementptr i64, ptr %139, i64 %131                                                                          ;L380
 25078|  %141 = load i64, ptr %140, , !!8                                                                                      ;L380
 25079|  %142 = icmp eq i64 %141, 0                                                                                            ;L380
 25080|  br i1 %142, label %145, label %143                                                                                    ;L380
 25081| 
 25082| 143: ; preds = %129
 25083|  store i64 9999, ptr %0,                                                                                               ;L381
 25084|  %144 = gep %0, i64 8                                                                                                  ;L381
 25085|  call void @llvm.memset.p0.i64(ptr %144, i8 0, i64 42, i1 false)                                                       ;L381
 25086|  br label %4102                                                                                                        ;L1
 25087| 
 25088| 145: ; preds = %129
 25089|  %146 = gep %122, i64 1648                                                                                             ;L384
 25090|  %147 = load i64, ptr %146, , !!8                                                                                      ;L384
 25091|     ;; self = i64 %147
 25092|     ;; other = i64 1
 25093|  %148 = tail call i64 @llvm.umax.i64(i64 %147, i64 1)                                                                  ;L1039<384
 25094|  %149 = udiv i64 4294967296, %148                                                                                      ;L384
 25095|     ;; inv_hp_q32 = i64 %149
 25096|     ;; score[0..+8] = i64 0
 25097|     ;; score[0..+8] = i64 0
 25098|     ;; score[8..+8] = i64 0
 25099|     ;; score[8..+8] = i64 0
 25100|     ;; score[16..+8] = i64 0
 25101|     ;; score[16..+8] = i64 0
 25102|     ;; score[24..+8] = i64 0
 25103|     ;; score[24..+8] = i64 0
 25104|     ;; score[32..+8] = i64 0
 25105|     ;; score[32..+8] = i64 0
 25106|     ;; score[48..+1] = i8 0
 25107|     ;; score[48..+1] = i8 0
 25108|     ;; score[49..+1] = i8 0
 25109|     ;; score[49..+1] = i8 0
 25110|     ;; score[40..+8] = i64 0
 25111|     ;; score[40..+8] = i64 0
 25112|     ;; tower_well_risk = i64 0
 25113|  %150 = sub nuw nsw i64 1, %112                                                                                        ;L393
 25114|     ;; enemy_team = i64 %150
 25115|  %151 = load ptr, ptr %118, , !!8, !!8                                                                                 ;L394
 25116|  %152 = gep %118, i64 8                                                                                                ;L394
 25117|  %153 = load ptr, ptr %152, , !!8, !!8                                                                                 ;L394
 25118|  %154 = gep %153, i64 256                                                                                              ;L394
 25119|  %155 = load ptr, ptr %154, , !!8                                                                                      ;L394
 25120|  %156 = invoke zeroext i1 %155(ptr %151, i64 %150, i64 %131, i64 %133)
 25121|  to label %157 unwind label %125                                                                                       ;L394
 25122| 
 25123| 157: ; preds = %145
 25124|     ;; visible = i1 %156
 25126|  store ptr %100, ptr %97,                                                                                              ;L395
 25127|  %158 = gep %97, i64 8                                                                                                 ;L395
 25128|  store ptr %99, ptr %158,                                                                                              ;L395
 25129|  %159 = load i64, ptr %100, , !!8                                                                                      ;L397
 25130|  %160 = load i64, ptr %99, , !!8                                                                                       ;L397
 25131|  %161 = invoke zeroext i1 @ai::path_finder20is_enemy_well_danger(i64 %1, ptr %2, i64 %159, i64 %160)
 25132|  to label %162 unwind label %125                                                                                       ;L397
 25133| 
 25134| 162: ; preds = %157
 25135|  %163 = select i1 %161, i64 9999, i64 0                                                                                ;L397
 25136|     ;; score[0..+8] = i64 %163
 25137|     ;; score[0..+8] = i64 %163
 25138|     ;; score[8..+8] = i64 %163
 25139|     ;; score[8..+8] = i64 %163
 25140|     ;; tower_well_risk = i64 %163
 25141|     ;; self = ptr %2
 25142|  %164 = gep %118, i64 640                                                                                              ;L403
 25143|  %165 = getelementptr [5 x { [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64] }], ptr %164, i64 %112 ;L403
 25144|  %166 = gepS %165, i64 %117                                                                                            ;L403
 25145|     ;; champ_cache = ptr %166
 25146|  %167 = gep %3, i64 16                                                                                                 ;L405
 25147|  %168 = load ptr, ptr %167,                                                                                            ;L405
 25149|     ;; player = ptr %2
 25150|     ;; seed = ptr %71
 25151|     ;; tick = ptr %70
 25152|     ;; team = ptr %69
 25153|     ;; enemy_team = ptr %68
 25155|  %169 = load ptr, ptr %118, , !!41175, !!8, !!8                                                                        ;L256<405
 25156|  %170 = load ptr, ptr %152, , !!41175, !!8, !!8                                                                        ;L256<405
 25157|  %171 = gep %170, i64 32                                                                                               ;L256<405
 25158|  %172 = load ptr, ptr %171, , !!41175, !!8                                                                             ;L256<405
 25159|  %173 = invoke i64 %172(ptr %169)
 25160|  to label %174 unwind label %125                                                                                       ;L256<405
 25161| 
 25162| 174: ; preds = %162
 25163|  store i64 %173, ptr %71, , !!41175                                                                                    ;L256<405
 25165|  %175 = gep %170, i64 40                                                                                               ;L257<405
 25166|  %176 = load ptr, ptr %175, , !!41175, !!8                                                                             ;L257<405
 25167|  %177 = invoke i64 %176(ptr %169)
 25168|  to label %178 unwind label %125                                                                                       ;L257<405
 25169| 
 25170| 178: ; preds = %174
 25171|  store i64 %177, ptr %70, , !!41175                                                                                    ;L257<405
 25173|  store i64 %112, ptr %69, , !!41175                                                                                    ;L258<405
 25175|  store i64 %150, ptr %68, , !!41175                                                                                    ;L259<405
 25177|  %179 = icmp ne ptr %168, null
 25178|  call void @llvm.assume(i1 %179)
 25179|  store ptr %71, ptr %67, , !!41175                                                                                     ;L260<405
 25180|  %180 = gep %67, i64 8                                                                                                 ;L260<405
 25181|  store ptr %70, ptr %180, , !!41175                                                                                    ;L260<405
 25182|  %181 = gep %67, i64 16                                                                                                ;L260<405
 25183|  store ptr %69, ptr %181, , !!41175                                                                                    ;L260<405
 25184|  %182 = gep %67, i64 24                                                                                                ;L260<405
 25185|  store ptr %118, ptr %182, , !!41175                                                                                   ;L260<405
 25186|  %183 = gep %67, i64 32                                                                                                ;L260<405
 25187|  store ptr %168, ptr %183, , !!41175                                                                                   ;L260<405
 25188|  %184 = gep %67, i64 40                                                                                                ;L260<405
 25189|  store ptr %68, ptr %184, , !!41175                                                                                    ;L260<405
 25190|  %185 = gep %67, i64 48                                                                                                ;L260<405
 25191|  store ptr %2, ptr %185, , !!41175                                                                                     ;L260<405
 25192|  %186 = invoke { i8, i8 } @core::cell7RefCellNtNtCshdEBA0ozCnw_7game_ai13position_eval11PeCandMasksEE4withNCNvB1x_13pe_cand_masks0ThhEEB1z_(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.95, ptr %67)
 25193|  to label %187 unwind label %125                                                                                       ;L260<405
 25194| 
 25195| 187: ; preds = %178
 25201|  %188 = extractvalue { i8, i8 } %186, 0                                                                                ;L405
 25202|  %189 = extractvalue { i8, i8 } %186, 1                                                                                ;L405
 25203|     ;; enemy_mask = i8 %188
 25204|     ;; ally_mask = i8 %189
 25206|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 25207|     ;; order = i8 0
 25208|  %190 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                    ;L3904<741<176<406
 25209|  %191 = icmp eq i8 %190, 0                                                                                             ;L176<406
 25210|  br i1 %191, label %194, label %192                                                                                    ;L176<406
 25211| 
 25212| 192: ; preds = %187
 25213|  %193 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()
 25214|  to label %218 unwind label %125                                                                                       ;L179<406
 25215| 
 25216| 194: ; preds = %218, %187
 25217|  %195 = phi i32 [ %220, %218 ], [ -1, %187 ]
 25218|  %196 = gep %96, i64 16                                                                                                ;L0<406
 25219|  store i32 %195, ptr %196,                                                                                             ;L0<406
 25221|  %197 = load ptr, ptr %135, , !!8, !!8                                                                                 ;L407
 25222|     ;; bump = ptr %197
 25223|     ;; bump = ptr %197
 25224|  store ptr inttoptr (i64 8 to ptr), ptr %95,                                                                           ;L547<407
 25225|  %198 = gep %95, i64 8                                                                                                 ;L547<407
 25226|  store ptr %197, ptr %198,                                                                                             ;L547<407
 25227|  %199 = gep %95, i64 16                                                                                                ;L547<407
 25228|  %200 = gep %95, i64 24                                                                                                ;L547<407
 25229|     ;; iter[0..+8] = i64 0
 25230|     ;; iter[8..+8] = i64 5
 25231|  %201 = getelementptr [5 x ptr], ptr %119, i64 %150
 25232|  %202 = gep %118, i64 560
 25233|  %203 = getelementptr [5 x ptr], ptr %202, i64 %150
 25234|  %204 = gep %8, i64 8
 25235|  %205 = gep %8, i64 16
 25236|  %206 = gep %8, i64 24
 25237|  %207 = gep %8, i64 32
 25238|  %208 = gep %8, i64 40
 25239|  %209 = gep %8, i64 48
 25240|  %210 = gep %8, i64 56
 25241|  %211 = gep %8, i64 64
 25242|  %212 = gep %8, i64 72
 25243|  %213 = gep %8, i64 80
 25244|  %214 = gep %8, i64 88
 25245|  %215 = gep %8, i64 96
 25246|  %216 = gep %8, i64 104
 25247|  %217 = gep %8, i64 112
 25248|  call void @llvm.memset.p0.i64(ptr %199, i8 0, i64 16, i1 false)                                                       ;L547<407
 25249|  br label %222                                                                                                         ;L408
 25250| 
 25251| 218: ; preds = %192
 25252|  %219 = extractvalue { i64, i32 } %193, 0                                                                              ;L179<406
 25253|  %220 = extractvalue { i64, i32 } %193, 1                                                                              ;L179<406
 25254|  store i64 112, ptr %96,                                                                                               ;L179<406
 25255|  %221 = gep %96, i64 8                                                                                                 ;L179<406
 25256|  store i64 %219, ptr %221,                                                                                             ;L179<406
 25257|  br label %194                                                                                                         ;L180<406
 25258| 
 25259| 222: ; preds = %4098, %194
 25260|  %223 = phi i64 [ 0, %194 ], [ %4099, %4098 ]
 25261|  %224 = phi i64 [ 0, %194 ], [ %250, %4098 ]                                                                           ;L408
 25262|     ;; iter[0..+8] = i64 %224
 25263|     ;; self = ptr undef
 25264|     ;; self = ptr undef
 25265|     ;; self = ptr undef
 25266|     ;; other = ptr undef
 25267|  %225 = icmp samesign ult i64 %224, 5                                                                                  ;L1916<900<985<408
 25268|  br i1 %225, label %249, label %226                                                                                    ;L900<985<408
 25269| 
 25270| 226: ; preds = %222
 25271|     ;; self = ptr %95
 25272|     ;; self = ptr %95
 25273|  %227 = icmp ne i64 %223, 0                                                                                            ;L1636<424
 25274|     ;; has_near_enemy = i1 %227
 25276|  store ptr inttoptr (i64 8 to ptr), ptr %93,                                                                           ;L547<425
 25277|  %228 = gep %93, i64 8                                                                                                 ;L547<425
 25278|  store ptr %197, ptr %228,                                                                                             ;L547<425
 25279|  %229 = gep %93, i64 16                                                                                                ;L547<425
 25280|  %230 = gep %93, i64 24                                                                                                ;L547<425
 25281|     ;; iter[0..+8] = i64 0
 25282|     ;; iter[8..+8] = i64 5
 25283|  %231 = gep %122, i64 1472
 25284|  %232 = getelementptr [5 x ptr], ptr %202, i64 %112
 25285|  %233 = zext i1 %227 to i8
 25286|  %234 = zext i1 %227 to i64
 25287|  %235 = gep %18, i64 8
 25288|  %236 = gep %18, i64 16
 25289|  %237 = gep %18, i64 24
 25290|  %238 = gep %18, i64 32
 25291|  %239 = gep %18, i64 40
 25292|  %240 = gep %18, i64 48
 25293|  %241 = gep %18, i64 56
 25294|  %242 = gep %18, i64 64
 25295|  %243 = gep %18, i64 72
 25296|  %244 = gep %18, i64 80
 25297|  %245 = gep %18, i64 88
 25298|  %246 = gep %18, i64 96
 25299|  %247 = gep %18, i64 104
 25300|  %248 = gep %18, i64 112
 25301|  call void @llvm.memset.p0.i64(ptr %229, i8 0, i64 16, i1 false)                                                       ;L547<425
 25302|  br label %255                                                                                                         ;L426
 25303| 
 25304| 249: ; preds = %222
 25305|     ;; old = i64 %224
 25306|     ;; start = i64 %224
 25307|     ;; self = i64 %224
 25308|  %250 = add nuw nsw i64 %224, 1                                                                                        ;L971<215<903<985<408
 25309|     ;; iter[0..+8] = i64 %250
 25310|     ;; index = i64 %224
 25311|  %251 = trunc nuw nsw i64 %224 to i8                                                                                   ;L409
 25312|  %252 = shl nuw nsw i8 1, %251                                                                                         ;L409
 25313|  %253 = and i8 %252, %188                                                                                              ;L409
 25314|  %254 = icmp eq i8 %253, 0                                                                                             ;L409
 25315|  br i1 %254, label %4098, label %4016                                                                                  ;L409
 25316| 
 25317| 255: ; preds = %4015, %226
 25318|  %256 = phi i64 [ 0, %226 ], [ %275, %4015 ]                                                                           ;L426
 25319|     ;; iter[0..+8] = i64 %256
 25320|     ;; self = ptr undef
 25321|     ;; self = ptr undef
 25322|     ;; self = ptr undef
 25323|     ;; other = ptr undef
 25324|  %257 = icmp samesign ult i64 %256, 5                                                                                  ;L1916<900<985<426
 25325|  br i1 %257, label %274, label %258                                                                                    ;L900<985<426
 25326| 
 25327| 258: ; preds = %255
 25330|  store i64 %1, ptr %66, , !!41235
 25331|     ;; version = ptr %66
 25332|     ;; player = ptr %2
 25333|     ;; data = ptr %3
 25334|     ;; champ = ptr %122
 25335|     ;; seed = ptr %65
 25336|     ;; tick = ptr %64
 25338|  %259 = load ptr, ptr %118, , !!41235, !!8, !!8                                                                        ;L344<442
 25339|  %260 = load ptr, ptr %152, , !!41235, !!8, !!8                                                                        ;L344<442
 25340|  %261 = gep %260, i64 32                                                                                               ;L344<442
 25341|  %262 = load ptr, ptr %261, , !!41235, !!8                                                                             ;L344<442
 25342|  %263 = invoke i64 %262(ptr %259)
 25343|  to label %264 unwind label %280                                                                                       ;L344<442
 25344| 
 25345| 264: ; preds = %258
 25346|  store i64 %263, ptr %65, , !!41235                                                                                    ;L344<442
 25348|  %265 = gep %260, i64 40                                                                                               ;L345<442
 25349|  %266 = load ptr, ptr %265, , !!41235, !!8                                                                             ;L345<442
 25350|  %267 = invoke i64 %266(ptr %259)
 25351|  to label %268 unwind label %280                                                                                       ;L345<442
 25352| 
 25353| 268: ; preds = %264
 25354|  store i64 %267, ptr %64, , !!41235                                                                                    ;L345<442
 25356|  store ptr %65, ptr %63, , !!41235                                                                                     ;L346<442
 25357|  %269 = gep %63, i64 8                                                                                                 ;L346<442
 25358|  store ptr %64, ptr %269, , !!41235                                                                                    ;L346<442
 25359|  %270 = gep %63, i64 16                                                                                                ;L346<442
 25360|  store ptr %2, ptr %270, , !!41235                                                                                     ;L346<442
 25361|  %271 = gep %63, i64 24                                                                                                ;L346<442
 25362|  store ptr %3, ptr %271, , !!41235                                                                                     ;L346<442
 25363|  %272 = gep %63, i64 32                                                                                                ;L346<442
 25364|  store ptr %122, ptr %272, , !!41235                                                                                   ;L346<442
 25365|  %273 = gep %63, i64 40                                                                                                ;L346<442
 25366|  store ptr %66, ptr %273, , !!41235                                                                                    ;L346<442
 25367|  invoke void @core::cell4CellINtNtBZ_6option6OptionNtNtCshdEBA0ozCnw_7game_ai13position_eval11PePlayerCtxEEE4withNCNvB1Q_13pe_player_ctx0B1O_EB1S_(ptr sret([40 x i8]) %91, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.96, ptr %63)
 25368|  to label %283 unwind label %280                                                                                       ;L346<442
 25369| 
 25370| 274: ; preds = %255
 25371|     ;; old = i64 %256
 25372|     ;; start = i64 %256
 25373|     ;; self = i64 %256
 25374|  %275 = add nuw nsw i64 %256, 1                                                                                        ;L971<215<903<985<426
 25375|     ;; iter[0..+8] = i64 %275
 25376|     ;; index = i64 %256
 25377|  %276 = trunc nuw nsw i64 %256 to i8                                                                                   ;L427
 25378|  %277 = shl nuw nsw i8 1, %276                                                                                         ;L427
 25379|  %278 = and i8 %277, %189                                                                                              ;L427
 25380|  %279 = icmp eq i8 %278, 0                                                                                             ;L427
 25381|  br i1 %279, label %4015, label %3928                                                                                  ;L427
 25382| 
 25383| 280: ; preds = %4003, %3995, %3991, %3980, %3979, %3978, %3960, %3927, %3926, %318, %301, %299, %268, %264, %258
 25384|  %281 = phi i1 [ true, %4003 ], [ true, %3995 ], [ false, %3926 ], [ true, %3979 ], [ true, %3978 ], [ false, %318 ], [ false, %299 ], [ true, %264 ], [ true, %3960 ], [ false, %301 ], [ true, %268 ], [ false, %3927 ], [ true, %258 ], [ true, %3980 ], [ true, %3991 ] ;L0
 25385|  %282 = cleanuppad within none []
 25386|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecTNtNtCshdEBA0ozCnw_7game_ai15score_parameter22EntityPositioningCacheRNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player11PlayerStateRNtNtB2C_6entity6EntityyEEEB1u_(ptr %93) #27 [ "funclet"(token %282) ] ;L1181
 25387|  cleanupret from %282 unwind label %3278                                                                               ;L1181
 25388| 
 25389| 283: ; preds = %268
 25394|  %284 = gep %91, i64 33                                                                                                ;L443
 25395|  %285 = load i8, ptr %284, , !!8                                                                                       ;L443
 25396|  %286 = trunc nuw i8 %285 to i1                                                                                        ;L443
 25397|     ;; skill_linear_move = i1 %286
 25398|  %287 = gep %91, i64 34                                                                                                ;L444
 25399|  %288 = load i8, ptr %287, , !!8                                                                                       ;L444
 25400|  %289 = trunc nuw i8 %288 to i1                                                                                        ;L444
 25401|     ;; skill2_linear_move = i1 %289
 25402|  %290 = gep %91, i64 35                                                                                                ;L445
 25403|  %291 = load i8, ptr %290, , !!8                                                                                       ;L445
 25404|  %292 = trunc nuw i8 %291 to i1                                                                                        ;L445
 25405|     ;; ult_linear_move = i1 %292
 25407|  call void @llvm.memcpy.p0.p0.i64(ptr %90, ptr %96, i64 24, i1 false)                                                  ;L447
 25410|  %293 = gep %90, i64 16                                                                                                ;L825<1004<447
 25411|  %294 = load i32, ptr %293, , !!8                                                                                      ;L825<1004<447
 25412|  %295 = icmp eq i32 %294, -1                                                                                           ;L825<1004<447
 25413|  br i1 %295, label %315, label %296                                                                                    ;L825<1004<447
 25414| 
 25415| 296: ; preds = %283
 25419|     ;; self = ptr %90
 25420|     ;; order = i8 0
 25421|     ;; order = i8 0
 25422|     ;; val = i64 1
 25423|     ;; order = i8 0
 25424|     ;; val = i64 1
 25425|     ;; order = i8 0
 25426|  %297 = load i64, ptr %90, , !!8                                                                                       ;L185<825<825<1004<447
 25427|  %298 = icmp ult i64 %297, 132                                                                                         ;L185<825<825<1004<447
 25428|  br i1 %298, label %301, label %299                                                                                    ;L185<825<825<1004<447
 25429| 
 25430| 299: ; preds = %296
 25431|  invoke void @core::panicking18panic_bounds_check(i64 %297, i64 132, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.208) #25
 25432|  to label %300 unwind label %280                                                                                       ;L185<825<825<1004<447
 25433| 
 25434| 300: ; preds = %299
 25435|  unreachable                                                                                                           ;L185<825<825<1004<447
 25436| 
 25437| 301: ; preds = %296
 25438|     ;; self = !DIArgList(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 %297)
 25439|  %302 = gep %90, i64 8                                                                                                 ;L185<825<825<1004<447
 25440|  %303 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %302)
 25441|  to label %304 unwind label %280                                                                                       ;L185<825<825<1004<447
 25442| 
 25443| 304: ; preds = %301
 25444|  %305 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %297                                 ;L185<825<825<1004<447
 25445|     ;; self = ptr %305
 25446|  %306 = extractvalue { i64, i32 } %303, 0                                                                              ;L185<825<825<1004<447
 25447|  %307 = extractvalue { i64, i32 } %303, 1                                                                              ;L185<825<825<1004<447
 25449|  %308 = mul i64 %306, 1000000000                                                                                       ;L632<185<825<825<1004<447
 25450|  %309 = icmp ult i32 %307, 1000000000                                                                                  ;L49<632<185<825<825<1004<447
 25451|  call void @llvm.assume(i1 %309)                                                                                       ;L49<632<185<825<825<1004<447
 25452|  %310 = zext nneg i32 %307 to i64                                                                                      ;L632<185<825<825<1004<447
 25453|  %311 = add i64 %308, %310                                                                                             ;L632<185<825<825<1004<447
 25454|     ;; val = i64 %311
 25455|     ;; val = i64 %311
 25456|     ;; dst = ptr %305
 25457|  %312 = atomicrmw add ptr %305, i64 %311 monotonic, , !!41303                                                          ;L3937<3162<185<825<825<1004<447
 25458|  %313 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %297                                 ;L186<825<825<1004<447
 25459|     ;; self = ptr %313
 25460|     ;; dst = ptr %313
 25461|  %314 = atomicrmw add ptr %313, i64 1 monotonic, , !!41303                                                             ;L3937<3162<186<825<825<1004<447
 25462|  br label %315                                                                                                         ;L825<1004<447
 25463| 
 25464| 315: ; preds = %304, %283
 25467|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 25468|     ;; order = i8 0
 25469|  %316 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                    ;L3904<741<176<448
 25470|  %317 = icmp eq i8 %316, 0                                                                                             ;L176<448
 25471|  br i1 %317, label %320, label %318                                                                                    ;L176<448
 25472| 
 25473| 318: ; preds = %315
 25474|  %319 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()
 25475|  to label %344 unwind label %280                                                                                       ;L179<448
 25476| 
 25477| 320: ; preds = %344, %315
 25478|  %321 = phi i32 [ %346, %344 ], [ -1, %315 ]
 25479|  %322 = gep %89, i64 16                                                                                                ;L0<448
 25480|  store i32 %321, ptr %322,                                                                                             ;L0<448
 25481|     ;; self = ptr %118
 25482|     ;; self = ptr %118
 25483|  %323 = gep %118, i64 208                                                                                              ;L138<2073<449
 25484|  %324 = load ptr, ptr %323, , !!8, !!8                                                                                 ;L138<2073<449
 25485|     ;; p = ptr %324
 25486|  %325 = gep %118, i64 232                                                                                              ;L2075<449
 25487|  %326 = load i64, ptr %325, , !!8                                                                                      ;L2075<449
 25488|     ;; len = i64 %326
 25489|     ;; count = i64 %326
 25490|     ;; self[0..+8] = ptr %324
 25491|     ;; slice[0..+8] = ptr %324
 25492|     ;; self[8..+8] = i64 %326
 25493|     ;; slice[8..+8] = i64 %326
 25494|     ;; ptr = ptr %324
 25495|     ;; self = ptr %324
 25496|  %327 = getelementptr ptr, ptr %324, i64 %326                                                                          ;L961<100<1042<449
 25497|     ;; iter[0..+8] = ptr %324
 25498|     ;; iter[8..+8] = ptr %327
 25499|  %328 = gep %56, i64 8
 25500|  %329 = gep %55, i64 8
 25501|  %330 = gep %55, i64 16
 25502|  %331 = gep %55, i64 24
 25503|  %332 = gep %55, i64 32
 25504|  %333 = gep %55, i64 40
 25505|  %334 = mul nuw nsw i64 %149, 100
 25506|  %335 = zext nneg i64 %334 to i128
 25507|  %336 = gep %60, i64 8
 25508|  %337 = gep %59, i64 8
 25509|  %338 = gep %59, i64 16
 25510|  %339 = gep %59, i64 24
 25511|  %340 = gep %59, i64 32
 25512|  %341 = gep %59, i64 40
 25513|  %342 = gep %122, i64 1136
 25514|  %343 = gep %122, i64 1664
 25515|  br label %396                                                                                                         ;L449
 25516| 
 25517| 344: ; preds = %318
 25518|  %345 = extractvalue { i64, i32 } %319, 0                                                                              ;L179<448
 25519|  %346 = extractvalue { i64, i32 } %319, 1                                                                              ;L179<448
 25520|  store i64 113, ptr %89,                                                                                               ;L179<448
 25521|  %347 = gep %89, i64 8                                                                                                 ;L179<448
 25522|  store i64 %345, ptr %347,                                                                                             ;L179<448
 25523|  br label %320                                                                                                         ;L180<448
 25524| 
 25525| 348: ; preds = %396, %387
 25526|  %349 = phi ptr [ %352, %387 ], [ %398, %396 ]                                                                         ;L449
 25527|     ;; score[0..+8] = i64 %397
 25528|     ;; score[0..+8] = i64 %397
 25529|     ;; iter[0..+8] = ptr %349
 25530|     ;; self = ptr undef
 25531|     ;; ptr = ptr %349
 25532|     ;; self = ptr %349
 25533|     ;; end_or_len = ptr %327
 25536|  %350 = icmp eq ptr %349, %327                                                                                         ;L1714<180<449
 25537|  br i1 %350, label %378, label %351                                                                                    ;L180<449
 25538| 
 25539| 351: ; preds = %348
 25540|  %352 = gep %349, i64 8                                                                                                ;L656<185<449
 25541|     ;; iter[0..+8] = ptr %352
 25542|  %353 = load ptr, ptr %349, , !!8, !!8                                                                                 ;L449
 25543|     ;; j = ptr %353
 25544|     ;; caster = ptr %353
 25545|     ;; self = ptr %353
 25546|  %354 = gep %353, i64 1632                                                                                             ;L450
 25547|  %355 = load i64, ptr %354, , !!8                                                                                      ;L450
 25548|  %356 = gep %353, i64 1640                                                                                             ;L450
 25549|  %357 = load i64, ptr %356,                                                                                            ;L450
 25553|  %358 = load i64, ptr %399, , !!8                                                                                      ;L395<450
 25554|     ;; a = i64 %358
 25555|     ;; self = i64 %358
 25556|     ;; b = i64 %355
 25557|     ;; other = i64 %355
 25558|  %359 = icmp ult i64 %358, %355                                                                                        ;L3147<8<395<450
 25559|  %360 = sub nuw i64 %355, %358                                                                                         ;L3147<8<395<450
 25560|  %361 = sub nuw i64 %358, %355                                                                                         ;L3147<8<395<450
 25561|  %362 = select i1 %359, i64 %360, i64 %361                                                                             ;L3147<8<395<450
 25562|     ;; rhs = i64 %362
 25563|     ;; rhs = i64 %362
 25564|     ;; rhs = i64 %362
 25565|     ;; diff = i64 %362
 25566|     ;; self = i64 %362
 25567|     ;; self = i64 %362
 25568|     ;; self = i64 %362
 25569|  %363 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %362, i64 %362)                                              ;L3178<1288<2517<9<395<450
 25570|  %364 = extractvalue { i64, i1 } %363, 0                                                                               ;L3178<1288<2517<9<395<450
 25571|  %365 = extractvalue { i64, i1 } %363, 1                                                                               ;L3178<1288<2517<9<395<450
 25572|     ;; a = i64 %364
 25573|     ;; self = i64 %364
 25574|     ;; b = i1 %365
 25575|     ;; b = i1 %365
 25576|  br i1 %365, label %366, label %367                                                                                    ;L459<1289<2517<9<395<450
 25577| 
 25578| 366: ; preds = %351
 25579|     ;; a = i64 -1
 25580|     ;; self = i64 -1
 25581|  br label %367                                                                                                         ;L2519<9<395<450
 25582| 
 25583| 367: ; preds = %366, %351
 25584|  %368 = phi i64 [ -1, %366 ], [ %364, %351 ]                                                                           ;L0<9<395<450
 25585|     ;; self = i64 %368
 25586|     ;; a = i64 %368
 25587|  call void @llvm.assume(i1 %401)
 25588|  %369 = load i64, ptr %400, , !!8                                                                                      ;L395<450
 25589|     ;; a = i64 %369
 25590|     ;; self = i64 %369
 25591|     ;; b = i64 %357
 25592|     ;; other = i64 %357
 25593|  %370 = icmp ult i64 %369, %357                                                                                        ;L3147<8<395<450
 25594|  %371 = sub nuw i64 %357, %369                                                                                         ;L3147<8<395<450
 25595|  %372 = sub nuw i64 %369, %357                                                                                         ;L3147<8<395<450
 25596|  %373 = select i1 %370, i64 %371, i64 %372                                                                             ;L3147<8<395<450
 25597|     ;; rhs = i64 %373
 25598|     ;; rhs = i64 %373
 25599|     ;; rhs = i64 %373
 25600|     ;; diff = i64 %373
 25601|     ;; self = i64 %373
 25602|     ;; self = i64 %373
 25603|     ;; self = i64 %373
 25604|  %374 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %373, i64 %373)                                              ;L3178<1288<2517<9<395<450
 25605|  %375 = extractvalue { i64, i1 } %374, 0                                                                               ;L3178<1288<2517<9<395<450
 25606|  %376 = extractvalue { i64, i1 } %374, 1                                                                               ;L3178<1288<2517<9<395<450
 25607|     ;; a = i64 %375
 25608|     ;; rhs = i64 %375
 25609|     ;; b = i1 %376
 25610|     ;; b = i1 %376
 25611|  br i1 %376, label %377, label %387                                                                                    ;L459<1289<2517<9<395<450
 25612| 
 25613| 377: ; preds = %367
 25614|     ;; a = i64 -1
 25615|     ;; rhs = i64 -1
 25616|  br label %387                                                                                                         ;L2519<9<395<450
 25617| 
 25618| 378: ; preds = %348
 25619|  %379 = gep %91, i64 24                                                                                                ;L489
 25620|  %380 = load i64, ptr %379, , !!8                                                                                      ;L489
 25621|     ;; disable_tick = i64 %380
 25622|  %381 = gep %153, i64 40                                                                                               ;L490
 25623|  %382 = load ptr, ptr %381, , !!8                                                                                      ;L490
 25624|  %383 = invoke i64 %382(ptr %151)
 25625|  to label %542 unwind label %384                                                                                       ;L490
 25626| 
 25627| 384: ; preds = %3910, %3909, %3901, %3895, %3891, %3884, %3878, %3825, %3814, %3746, %3731, %3686, %3685, %3677, %3667, %3663, %3657, %3644, %3569, %3565, %3559, %3542, %3538, %3532, %3510, %3509, %1037, %1021, %1011, %1001, %959, %957, %900, %893, %882, %873, %858, %854, %843, %829, %733, %598, %596, %595, %593, %578, %548, %529, %524, %510, %506, %500, %447, %445, %441, %426, %422, %416, %378
 25628|  %385 = phi i1 [ true, %595 ], [ true, %3644 ], [ true, %3559 ], [ true, %3565 ], [ true, %3569 ], [ true, %510 ], [ true, %3538 ], [ true, %3542 ], [ true, %506 ], [ true, %1001 ], [ false, %3510 ], [ false, %3509 ], [ false, %1037 ], [ true, %1011 ], [ false, %959 ], [ true, %1021 ], [ true, %882 ], [ true, %900 ], [ true, %873 ], [ true, %593 ], [ true, %578 ], [ true, %598 ], [ true, %3532 ], [ true, %596 ], [ true, %378 ], [ true, %447 ], [ true, %441 ], [ true, %445 ], [ true, %422 ], [ true, %426 ], [ true, %524 ], [ true, %529 ], [ true, %416 ], [ true, %500 ], [ true, %548 ], [ true, %893 ], [ false, %957 ], [ true, %3895 ], [ true, %3891 ], [ true, %3884 ], [ true, %3878 ], [ true, %3825 ], [ true, %3814 ], [ true, %3667 ], [ true, %3663 ], [ true, %3657 ], [ true, %858 ], [ true, %854 ], [ true, %843 ], [ true, %829 ], [ true, %733 ], [ true, %3910 ], [ true, %3909 ], [ true, %3901 ], [ true, %3746 ], [ true, %3731 ], [ true, %3686 ], [ true, %3685 ], [ true, %3677 ] ;L0
 25629|  %386 = cleanuppad within none []
 25630|  br i1 %385, label %3927, label %3926                                                                                  ;L1181
 25631| 
 25632| 387: ; preds = %377, %367
 25633|  %388 = phi i64 [ -1, %377 ], [ %375, %367 ]                                                                           ;L0<9<395<450
 25634|     ;; rhs = i64 %388
 25635|     ;; a = i64 %388
 25636|  %389 = call i64 @llvm.uadd.sat.i64(i64 %368, i64 %388)                                                                ;L2428<395<450
 25637|     ;; jd = i64 %389
 25638|  %390 = icmp ugt i64 %389, 22499999999                                                                                 ;L451
 25639|  br i1 %390, label %348, label %391                                                                                    ;L451
 25640| 
 25641| 391: ; preds = %387
 25642|  %392 = gep %353, i64 104                                                                                              ;L454
 25643|  %393 = load i64, ptr %392, , !!8                                                                                      ;L454
 25644|  switch i64 %393, label %394 [
 25645|  i64 4, label %402
 25646|  i64 5, label %406
 25647|  ]                                                                                                                     ;L454
 25648| 
 25649| 394: ; preds = %540, %535, %530, %497, %487, %457, %411, %406, %402, %391
 25650|  %395 = phi i64 [ %397, %391 ], [ %458, %457 ], [ %397, %487 ], [ %499, %497 ], [ %397, %411 ], [ %397, %402 ], [ %397, %406 ], [ %541, %540 ], [ %539, %535 ], [ %397, %530 ] ;L0
 25651|     ;; score[0..+8] = i64 %395
 25652|     ;; score[0..+8] = i64 %395
 25653|  br label %396                                                                                                         ;L449
 25654| 
 25655| 396: ; preds = %394, %320
 25656|  %397 = phi i64 [ %395, %394 ], [ %163, %320 ]
 25657|  %398 = phi ptr [ %352, %394 ], [ %324, %320 ]
 25658|  %399 = load ptr, ptr %97, , !!8
 25659|  %400 = load ptr, ptr %158,
 25660|  %401 = icmp ne ptr %400, null
 25661|  br label %348                                                                                                         ;L180<449
 25662| 
 25663| 402: ; preds = %391
 25664|     ;; info = ptr %353
 25665|     ;; self = ptr %353
 25667|  %403 = gep %353, i64 136                                                                                              ;L2439<456
 25668|  %404 = load i64, ptr %403, , !!8                                                                                      ;L2439<456
 25669|  %405 = trunc nuw i64 %404 to i1                                                                                       ;L2439<456
 25670|  br i1 %405, label %411, label %394                                                                                    ;L2439<456
 25671| 
 25672| 406: ; preds = %391
 25673|     ;; info = ptr %353
 25674|     ;; self = ptr %353
 25675|  %407 = gep %353, i64 136                                                                                              ;L633<473
 25676|  %408 = load i64, ptr %407, , !!8                                                                                      ;L633<473
 25677|  %409 = gep %353, i64 144                                                                                              ;L633<473
 25678|  %410 = icmp eq i64 %408, 0                                                                                            ;L473
 25679|  br i1 %410, label %394, label %500                                                                                    ;L473
 25680| 
 25681| 411: ; preds = %402
 25682|  %412 = gep %353, i64 144                                                                                              ;L2439<456
 25683|  %413 = load i64, ptr %231, , !!8                                                                                      ;L456
 25684|     ;; l = ptr %353
 25685|     ;; self = ptr %353
 25688|  %414 = load i64, ptr %412, , !!8                                                                                      ;L1878<2440<456
 25689|  %415 = icmp eq i64 %414, %413                                                                                         ;L1878<2440<456
 25690|  br i1 %415, label %416, label %394                                                                                    ;L456
 25691| 
 25692| 416: ; preds = %411
 25696|     ;; attacker = ptr %353
 25697|     ;; target = ptr %122
 25698|     ;; seed = ptr %62
 25699|     ;; tick = ptr %61
 25700|     ;; key = ptr %60
 25702|  %417 = load ptr, ptr %118, , !!41524, !!8, !!8                                                                        ;L131<457
 25703|  %418 = load ptr, ptr %152, , !!41524, !!8, !!8                                                                        ;L131<457
 25704|  %419 = gep %418, i64 32                                                                                               ;L131<457
 25705|  %420 = load ptr, ptr %419, , !!41524, !!8                                                                             ;L131<457
 25706|  %421 = invoke i64 %420(ptr %417)
 25707|  to label %422 unwind label %384                                                                                       ;L131<457
 25708| 
 25709| 422: ; preds = %416
 25710|  store i64 %421, ptr %62, , !!41524                                                                                    ;L131<457
 25712|  %423 = gep %418, i64 40                                                                                               ;L132<457
 25713|  %424 = load ptr, ptr %423, , !!41524, !!8                                                                             ;L132<457
 25714|  %425 = invoke i64 %424(ptr %417)
 25715|  to label %426 unwind label %384                                                                                       ;L132<457
 25716| 
 25717| 426: ; preds = %422
 25718|  store i64 %425, ptr %61, , !!41524                                                                                    ;L132<457
 25720|  %427 = gep %353, i64 1472                                                                                             ;L133<457
 25721|  %428 = load i64, ptr %427, , !!41503, !!8                                                                             ;L133<457
 25722|  %429 = load i64, ptr %231, , !!41499, !!8                                                                             ;L133<457
 25723|  store i64 %428, ptr %60, , !!41524                                                                                    ;L133<457
 25724|  store i64 %429, ptr %336, , !!41524                                                                                   ;L133<457
 25726|  store ptr %62, ptr %59, , !!41524                                                                                     ;L134<457
 25727|  store ptr %61, ptr %337, , !!41524                                                                                    ;L134<457
 25728|  store ptr %60, ptr %338, , !!41524                                                                                    ;L134<457
 25729|  store ptr %353, ptr %339, , !!41524                                                                                   ;L134<457
 25730|  store ptr %135, ptr %340, , !!41524                                                                                   ;L134<457
 25731|  store ptr %122, ptr %341, , !!41524                                                                                   ;L134<457
 25732|  %430 = invoke i64 @core::cell7RefCellNtNtCshdEBA0ozCnw_7game_ai13position_eval17AttackDamageCacheEE4withNCNvB1x_29expected_attack_damage_cached0jEB1z_(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.119, ptr %59)
 25733|  to label %431 unwind label %384                                                                                       ;L134<457
 25734| 
 25735| 431: ; preds = %426
 25740|     ;; damage = i64 %430
 25741|     ;; x = i64 %430
 25742|     ;; inv_hp_q32 = i64 %149
 25743|  %432 = zext i64 %430 to i128                                                                                          ;L387<458
 25744|  %433 = mul nuw nsw i128 %335, %432                                                                                    ;L387<458
 25745|  %434 = lshr i128 %433, 32                                                                                             ;L387<458
 25746|     ;; self = i128 %434
 25747|     ;; other = i128 150
 25748|  %435 = call i128 @llvm.umin.i128(i128 %434, i128 150)                                                                 ;L1078<387<458
 25749|  %436 = trunc nuw nsw i128 %435 to i64                                                                                 ;L387<458
 25750|     ;; ratio = i64 %436
 25751|     ;; atk = ptr %353
 25752|     ;; self = ptr %353
 25753|  %437 = gep %353, i64 1168                                                                                             ;L742<460
 25754|  %438 = gep %353, i64 1216                                                                                             ;L742<460
 25755|  %439 = load i32, ptr %438, , !!8                                                                                      ;L742<460
 25756|  %440 = icmp eq i32 %439, -1                                                                                           ;L742<460
 25757|  br i1 %440, label %445, label %441                                                                                    ;L742<460
 25758| 
 25759| 441: ; preds = %431
 25760|     ;; self = ptr %437
 25761|     ;; atk = ptr %437
 25762|     ;; self = ptr %437
 25763|  %442 = load i64, ptr %100, , !!8                                                                                      ;L461
 25764|  %443 = load i64, ptr %99, , !!8                                                                                       ;L461
 25765|  %444 = invoke zeroext i1 @gc::simulation6effectNtB2_6Effect15is_in_range_pos(ptr %437, ptr %353, ptr %122, i64 %442, i64 %443)
 25766|  to label %446 unwind label %384                                                                                       ;L461
 25767| 
 25768| 445: ; preds = %431
 25769|     ;; self = ptr null
 25770|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.99) #25
 25771|  to label %127 unwind label %384                                                                                       ;L1013<460
 25772| 
 25773| 446: ; preds = %441
 25774|  br i1 %444, label %457, label %447                                                                                    ;L461
 25775| 
 25776| 447: ; preds = %446
 25777|  %448 = gep %353, i64 1184                                                                                             ;L26<464
 25778|  %449 = load i64, ptr %448, , !!8                                                                                      ;L26<464
 25779|  %450 = gep %353, i64 1192                                                                                             ;L26<464
 25780|  %451 = load i64, ptr %450, , !!8                                                                                      ;L26<464
 25781|  %452 = gep %353, i64 1480                                                                                             ;L26<464
 25782|  %453 = load i64, ptr %452, , !!8                                                                                      ;L26<464
 25783|  %454 = gep %353, i64 1080                                                                                             ;L26<464
 25784|  %455 = load i64, ptr %454, , !!8                                                                                      ;L26<464
 25785|  %456 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %437, ptr %353, ptr %122)
 25786|  to label %459 unwind label %384                                                                                       ;L464
 25787| 
 25788| 457: ; preds = %446
 25789|  %458 = add i64 %397, %436                                                                                             ;L462
 25790|     ;; score[0..+8] = i64 %458
 25791|     ;; score[0..+8] = i64 %458
 25792|  br label %394                                                                                                         ;L461
 25793| 
 25794| 459: ; preds = %447
 25795|  %460 = add i64 %453, -1                                                                                               ;L26<464
 25796|  %461 = mul i64 %460, %451                                                                                             ;L26<464
 25797|  %462 = gep %353, i64 1136                                                                                             ;L1511<465
 25798|  %463 = load i32, ptr %462, , !!8                                                                                      ;L1511<465
 25799|     ;; mult = i32 %463
 25800|  %464 = icmp eq i32 %463, 0                                                                                            ;L1512<465
 25801|  br i1 %464, label %465, label %468                                                                                    ;L1512<465
 25802| 
 25803| 465: ; preds = %459
 25804|  %466 = gep %353, i64 1664                                                                                             ;L1513<465
 25805|  %467 = load i64, ptr %466, , !!8                                                                                      ;L1513<465
 25806|  br label %475                                                                                                         ;L1512<465
 25807| 
 25808| 468: ; preds = %459
 25809|  %469 = sext i32 %463 to i64                                                                                           ;L1511<465
 25810|     ;; mult = i64 %469
 25811|  %470 = gep %353, i64 1664                                                                                             ;L1515<465
 25812|  %471 = load i64, ptr %470, , !!8                                                                                      ;L1515<465
 25813|  %472 = add nsw i64 %469, 100                                                                                          ;L1515<465
 25814|  %473 = mul i64 %471, %472                                                                                             ;L1515<465
 25815|  %474 = udiv i64 %473, 100                                                                                             ;L1515<465
 25816|  br label %475                                                                                                         ;L1512<465
 25817| 
 25818| 475: ; preds = %468, %465
 25819|  %476 = phi i64 [ %467, %465 ], [ %474, %468 ]                                                                         ;L0<465
 25820|  %477 = load i32, ptr %342, , !!8                                                                                      ;L1511<465
 25821|     ;; mult = i32 %477
 25822|  %478 = icmp eq i32 %477, 0                                                                                            ;L1512<465
 25823|  br i1 %478, label %479, label %481                                                                                    ;L1512<465
 25824| 
 25825| 479: ; preds = %475
 25826|  %480 = load i64, ptr %343, , !!8                                                                                      ;L1513<465
 25827|  br label %487                                                                                                         ;L1512<465
 25828| 
 25829| 481: ; preds = %475
 25830|  %482 = sext i32 %477 to i64                                                                                           ;L1511<465
 25831|     ;; mult = i64 %482
 25832|  %483 = load i64, ptr %343, , !!8                                                                                      ;L1515<465
 25833|  %484 = add nsw i64 %482, 100                                                                                          ;L1515<465
 25834|  %485 = mul i64 %483, %484                                                                                             ;L1515<465
 25835|  %486 = udiv i64 %485, 100                                                                                             ;L1515<465
 25836|  br label %487                                                                                                         ;L1512<465
 25837| 
 25838| 487: ; preds = %481, %479
 25839|  %488 = phi i64 [ %480, %479 ], [ %486, %481 ]                                                                         ;L0<465
 25841|  %489 = add i64 %449, 32000                                                                                            ;L26<464
 25842|  %490 = add i64 %489, %455                                                                                             ;L26<464
 25843|  %491 = add i64 %490, %461                                                                                             ;L464
 25844|  %492 = add i64 %491, %456                                                                                             ;L464
 25845|  %493 = add i64 %492, %476                                                                                             ;L464
 25846|  %494 = add i64 %493, %488                                                                                             ;L466
 25847|  %495 = mul i64 %494, %494                                                                                             ;L466
 25848|  %496 = icmp ugt i64 %389, %495                                                                                        ;L466
 25849|  br i1 %496, label %394, label %497                                                                                    ;L466
 25850| 
 25851| 497: ; preds = %487
 25852|  %498 = lshr i64 %436, 1                                                                                               ;L467
 25853|  %499 = add i64 %498, %397                                                                                             ;L467
 25854|     ;; score[0..+8] = i64 %499
 25855|     ;; score[0..+8] = i64 %499
 25856|  br label %394                                                                                                         ;L466
 25857| 
 25858| 500: ; preds = %406
 25862|     ;; attacker = ptr %353
 25863|     ;; target = ptr %122
 25864|     ;; seed = ptr %58
 25865|     ;; tick = ptr %57
 25866|     ;; key = ptr %56
 25868|  %501 = load ptr, ptr %118, , !!41606, !!8, !!8                                                                        ;L131<474
 25869|  %502 = load ptr, ptr %152, , !!41606, !!8, !!8                                                                        ;L131<474
 25870|  %503 = gep %502, i64 32                                                                                               ;L131<474
 25871|  %504 = load ptr, ptr %503, , !!41606, !!8                                                                             ;L131<474
 25872|  %505 = invoke i64 %504(ptr %501)
 25873|  to label %506 unwind label %384                                                                                       ;L131<474
 25874| 
 25875| 506: ; preds = %500
 25876|  store i64 %505, ptr %58, , !!41606                                                                                    ;L131<474
 25878|  %507 = gep %502, i64 40                                                                                               ;L132<474
 25879|  %508 = load ptr, ptr %507, , !!41606, !!8                                                                             ;L132<474
 25880|  %509 = invoke i64 %508(ptr %501)
 25881|  to label %510 unwind label %384                                                                                       ;L132<474
 25882| 
 25883| 510: ; preds = %506
 25884|  store i64 %509, ptr %57, , !!41606                                                                                    ;L132<474
 25886|  %511 = gep %353, i64 1472                                                                                             ;L133<474
 25887|  %512 = load i64, ptr %511, , !!41598, !!8                                                                             ;L133<474
 25888|  %513 = load i64, ptr %231, , !!41594, !!8                                                                             ;L133<474
 25889|  store i64 %512, ptr %56, , !!41606                                                                                    ;L133<474
 25890|  store i64 %513, ptr %328, , !!41606                                                                                   ;L133<474
 25892|  store ptr %58, ptr %55, , !!41606                                                                                     ;L134<474
 25893|  store ptr %57, ptr %329, , !!41606                                                                                    ;L134<474
 25894|  store ptr %56, ptr %330, , !!41606                                                                                    ;L134<474
 25895|  store ptr %353, ptr %331, , !!41606                                                                                   ;L134<474
 25896|  store ptr %135, ptr %332, , !!41606                                                                                   ;L134<474
 25897|  store ptr %122, ptr %333, , !!41606                                                                                   ;L134<474
 25898|  %514 = invoke i64 @core::cell7RefCellNtNtCshdEBA0ozCnw_7game_ai13position_eval17AttackDamageCacheEE4withNCNvB1x_29expected_attack_damage_cached0jEB1z_(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.119, ptr %55)
 25899|  to label %515 unwind label %384                                                                                       ;L134<474
 25900| 
 25901| 515: ; preds = %510
 25906|     ;; damage = i64 %514
 25907|     ;; x = i64 %514
 25908|     ;; inv_hp_q32 = i64 %149
 25909|  %516 = zext i64 %514 to i128                                                                                          ;L387<475
 25910|  %517 = mul nuw nsw i128 %335, %516                                                                                    ;L387<475
 25911|  %518 = lshr i128 %517, 32                                                                                             ;L387<475
 25912|     ;; self = i128 %518
 25913|     ;; other = i128 150
 25914|  %519 = call i128 @llvm.umin.i128(i128 %518, i128 150)                                                                 ;L1078<387<475
 25915|  %520 = trunc nuw nsw i128 %519 to i64                                                                                 ;L387<475
 25916|     ;; ratio = i64 %520
 25917|     ;; self = ptr %353
 25918|  %521 = gep %353, i64 1216                                                                                             ;L742<476
 25919|  %522 = load i32, ptr %521, , !!8                                                                                      ;L742<476
 25920|  %523 = icmp eq i32 %522, -1                                                                                           ;L742<476
 25921|  br i1 %523, label %529, label %524                                                                                    ;L742<476
 25922| 
 25923| 524: ; preds = %515
 25924|  %525 = gep %353, i64 1168                                                                                             ;L742<476
 25925|     ;; self = ptr %525
 25926|  %526 = load i64, ptr %100, , !!8                                                                                      ;L476
 25927|  %527 = load i64, ptr %99, , !!8                                                                                       ;L476
 25928|  %528 = invoke zeroext i1 @gc::simulation6effectNtB2_6Effect15is_in_range_pos(ptr %525, ptr %353, ptr %122, i64 %526, i64 %527)
 25929|  to label %530 unwind label %384                                                                                       ;L476
 25930| 
 25931| 529: ; preds = %515
 25932|     ;; self = ptr null
 25933|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.100) #25
 25934|  to label %127 unwind label %384                                                                                       ;L1013<476
 25935| 
 25936| 530: ; preds = %524
 25937|  br i1 %528, label %531, label %394                                                                                    ;L476
 25938| 
 25939| 531: ; preds = %530
 25940|     ;; self = ptr %353
 25942|  %532 = load i64, ptr %231, , !!8                                                                                      ;L477
 25943|     ;; l = ptr %353
 25944|     ;; self = ptr %353
 25947|  %533 = load i64, ptr %409, , !!8                                                                                      ;L1878<2440<477
 25948|  %534 = icmp eq i64 %533, %532                                                                                         ;L1878<2440<477
 25949|  br i1 %534, label %540, label %535                                                                                    ;L477
 25950| 
 25951| 535: ; preds = %531
 25952|  %536 = trunc nuw i128 %519 to i8                                                                                      ;L480
 25953|  %537 = udiv i8 %536, 3                                                                                                ;L480
 25954|  %538 = zext nneg i8 %537 to i64                                                                                       ;L480
 25955|  %539 = add i64 %397, %538                                                                                             ;L480
 25956|     ;; score[0..+8] = i64 %539
 25957|     ;; score[0..+8] = i64 %539
 25958|  br label %394                                                                                                         ;L477
 25959| 
 25960| 540: ; preds = %531
 25961|  %541 = add i64 %397, %520                                                                                             ;L478
 25962|     ;; score[0..+8] = i64 %541
 25963|     ;; score[0..+8] = i64 %541
 25964|  br label %394                                                                                                         ;L477
 25965| 
 25966| 542: ; preds = %378
 25967|  %543 = icmp ugt i64 %383, %380                                                                                        ;L490
 25968|  br i1 %543, label %548, label %544                                                                                    ;L490
 25969| 
 25970| 544: ; preds = %542
 25972|     ;; self = ptr %122
 25973|  %545 = load i64, ptr %122, , !!8                                                                                      ;L1136<491
 25974|  %546 = gep %122, i64 8                                                                                                ;L1136<491
 25975|     ;; __arg1_discr = i64 %545
 25976|  %547 = trunc nuw i64 %545 to i1                                                                                       ;L1136<491
 25977|  br i1 %547, label %595, label %596                                                                                    ;L1136<491
 25978| 
 25979| 548: ; preds = %867, %542
 25980|  %549 = phi i64 [ 0, %542 ], [ %638, %867 ]                                                                            ;L367<390
 25981|  %550 = phi i64 [ %163, %542 ], [ %639, %867 ]                                                                         ;L399
 25982|  %551 = phi i64 [ %397, %542 ], [ %640, %867 ]                                                                         ;L398
 25983|  %552 = phi i64 [ %163, %542 ], [ %641, %867 ]                                                                         ;L400
 25984|     ;; score[0..+8] = i64 %551
 25985|     ;; score[0..+8] = i64 %551
 25986|     ;; score[8..+8] = i64 %550
 25987|     ;; score[8..+8] = i64 %550
 25988|     ;; score[16..+8] = i64 %549
 25989|     ;; score[16..+8] = i64 %549
 25990|     ;; tower_well_risk = i64 %552
 25991|  %553 = load i64, ptr %122,                                                                                            ;L573
 25992|  %554 = gep %122, i64 8                                                                                                ;L573
 25993|  %555 = load i64, ptr %554,                                                                                            ;L573
 25994|     ;; version = i64 %1
 25997|     ;; purpose = i8 %6
 25998|     ;; self = ptr %135
 25999|  %556 = load ptr, ptr %118, , !!8, !!8                                                                                 ;L23<573
 26000|  %557 = load ptr, ptr %152, , !!8, !!8                                                                                 ;L23<573
 26001|  %558 = gep %557, i64 40                                                                                               ;L23<573
 26002|  %559 = load ptr, ptr %558, , !!8                                                                                      ;L23<573
 26003|  %560 = invoke i64 %559(ptr %556)
 26004|  to label %561 unwind label %384                                                                                       ;L23<573
 26005| 
 26006| 561: ; preds = %548
 26007|     ;; tick = i64 %560
 26008|     ;; tick = i64 %560
 26009|     ;; self = ptr %135
 26010|  %562 = gep %135, i64 56                                                                                               ;L263<399<23<573
 26011|  %563 = load i8, ptr %562, , !!8                                                                                       ;L263<399<23<573
 26012|  switch i8 %563, label %574 [
 26013|  i8 0, label %564
 26014|  i8 7, label %564
 26015|  i8 8, label %564
 26016|  i8 5, label %564
 26017|  ]                                                                                                                     ;L263<399<23<573
 26018| 
 26019| 564: ; preds = %561, %561, %561, %561
 26020|  %565 = gep %135, i64 8                                                                                                ;L399<23<573
 26021|  %566 = load ptr, ptr %565, , !!8, !!8                                                                                 ;L399<23<573
 26022|     ;; self = ptr %566
 26023|  %567 = gep %566, i64 2216                                                                                             ;L703<399<23<573
 26024|  %568 = load i64, ptr %567, , !!8                                                                                      ;L703<399<23<573
 26025|     ;; self = i64 %568
 26026|  %569 = gep %566, i64 4856                                                                                             ;L704<399<23<573
 26027|  %570 = load i64, ptr %569, , !!8                                                                                      ;L704<399<23<573
 26028|  %571 = mul i64 %570, 30                                                                                               ;L704<399<23<573
 26029|     ;; rhs = i64 %571
 26030|  %572 = call i64 @llvm.usub.sat.i64(i64 %568, i64 %571)                                                                ;L2472<703<399<23<573
 26031|  %573 = icmp ult i64 %560, %572                                                                                        ;L703<399<23<573
 26032|  br i1 %573, label %574, label %873                                                                                    ;L23<573
 26033| 
 26034| 574: ; preds = %564, %561
 26035|  %575 = icmp ne i8 %6, 9                                                                                               ;L24<573
 26036|  call void @llvm.assume(i1 %575)                                                                                       ;L24<573
 26037|  switch i8 %6, label %873 [
 26038|  i8 10, label %576
 26039|  i8 0, label %576
 26040|  i8 8, label %576
 26041|  i8 1, label %576
 26042|  ]                                                                                                                     ;L24<573
 26043| 
 26044| 576: ; preds = %574, %574, %574, %574
 26048|  %577 = trunc nuw i64 %553 to i1                                                                                       ;L1136<93<25<573
 26049|  br i1 %577, label %882, label %578                                                                                    ;L1136<93<25<573
 26050| 
 26051| 578: ; preds = %576
 26052|     ;; target_team = i64 %555
 26053|  %579 = load ptr, ptr %118, , !!41747, !!8, !!8                                                                        ;L96<25<573
 26054|  %580 = load ptr, ptr %152, , !!41747, !!8, !!8                                                                        ;L96<25<573
 26055|  %581 = gep %580, i64 64                                                                                               ;L96<25<573
 26056|  %582 = load ptr, ptr %581, , !!41747, !!8                                                                             ;L96<25<573
 26057|  %583 = invoke { i64, ptr } %582(ptr %579)
 26058|  to label %584 unwind label %384                                                                                       ;L96<25<573
 26059| 
 26060| 584: ; preds = %578
 26061|  %585 = extractvalue { i64, ptr } %583, 0                                                                              ;L96<25<573
 26062|     ;; self[0..+8] = i64 %585
 26064|  %586 = icmp ne i64 %585, 0                                                                                            ;L231<96<25<573
 26065|  %587 = extractvalue { i64, ptr } %583, 1
 26067|     ;; default = i64 0
 26069|  %588 = icmp eq ptr %587, null                                                                                         ;L1226<96<25<573
 26070|  %589 = select i1 %586, i1 true, i1 %588                                                                               ;L1226<96<25<573
 26071|  br i1 %589, label %882, label %590                                                                                    ;L1226<96<25<573
 26072| 
 26073| 590: ; preds = %584
 26074|     ;; t = ptr %587
 26076|     ;; m = ptr %587
 26077|     ;; self = ptr %587
 26078|  %591 = sub i64 1, %555                                                                                                ;L96<1227<96<25<573
 26079|     ;; team = i64 %591
 26080|  %592 = icmp ult i64 %591, 2                                                                                           ;L210<96<1227<96<25<573
 26081|  br i1 %592, label %868, label %593                                                                                    ;L210<96<1227<96<25<573
 26082| 
 26083| 593: ; preds = %590
 26084|  invoke void @core::panicking18panic_bounds_check(i64 %591, i64 2, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.17) #25
 26085|  to label %594 unwind label %384                                                                                       ;L210<96<1227<96<25<573
 26086| 
 26087| 594: ; preds = %593
 26088|  unreachable                                                                                                           ;L210<96<1227<96<25<573
 26089| 
 26090| 595: ; preds = %544
 26093|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.104) #25
 26094|  to label %127 unwind label %384                                                                                       ;L1013<491
 26095| 
 26096| 596: ; preds = %544
 26097|     ;; team = ptr %122
 26098|  %597 = load i64, ptr %546, , !!8                                                                                      ;L1137<491
 26099|     ;; self[8..+8] = i64 %597
 26100|     ;; self[0..+8] = i64 1
 26101|  store i64 %597, ptr %88,                                                                                              ;L1012<491
 26104|  invoke void @gc::simulationNtB5_21AbstractGameWithCache25iter_towers_without_nexus(ptr sret([120 x i8]) %86, ptr %118, i64 %150)
 26105|  to label %598 unwind label %384                                                                                       ;L492
 26106| 
 26107| 598: ; preds = %596
 26108|     ;; predicate[0..+8] = ptr %97
 26109|     ;; predicate[8..+8] = ptr %95
 26110|  %599 = load i64, ptr %86,                                                                                             ;L28<957<493
 26111|     ;; self[0..+8] = i64 %599
 26112|  %600 = gep %86, i64 8                                                                                                 ;L28<957<493
 26113|  call void @llvm.memcpy.p0.p0.i64(ptr %87, ptr %600, i64 112, i1 false)                                                ;L28<957<493
 26114|     ;; self[120..+8] = ptr %97
 26115|     ;; self[128..+8] = ptr %95
 26118|  invoke void @gc::simulationNtB5_21AbstractGameWithCache25iter_towers_without_nexus(ptr sret([120 x i8]) %85, ptr %118, i64 %112)
 26119|  to label %601 unwind label %384                                                                                       ;L495
 26120| 
 26121| 601: ; preds = %598
 26122|     ;; predicate[0..+8] = ptr %97
 26123|     ;; predicate[8..+8] = ptr %93
 26124|  %602 = gep %84, i64 136                                                                                               ;L507
 26126|  call void @llvm.memcpy.p0.p0.i64(ptr %602, ptr %85, i64 120, i1 false)                                                ;L28<957<496
 26127|     ;; other[120..+8] = ptr %97
 26128|     ;; b[120..+8] = ptr %97
 26129|     ;; other[128..+8] = ptr %93
 26130|     ;; b[128..+8] = ptr %93
 26133|     ;; a[0..+8] = i64 %599
 26134|  %603 = gep %84, i64 8                                                                                                 ;L507
 26135|  call void @llvm.memcpy.p0.p0.i64(ptr %603, ptr %87, i64 112, i1 false)                                                ;L515<495
 26136|     ;; self[120..+8] = ptr %97
 26137|     ;; self[128..+8] = ptr %95
 26138|     ;; self[0..+8] = i64 %599
 26139|     ;; self[256..+8] = ptr %97
 26140|     ;; self[264..+8] = ptr %93
 26142|     ;; f[0..+8] = ptr %122
 26143|     ;; f[8..+8] = ptr %3
 26144|     ;; f[16..+8] = ptr %88
 26145|     ;; near_towers[0..+8] = i64 %599
 26146|     ;; near_towers[120..+8] = ptr %97
 26147|     ;; near_towers[128..+8] = ptr %95
 26148|     ;; near_towers[256..+8] = ptr %97
 26149|     ;; near_towers[264..+8] = ptr %93
 26150|     ;; near_towers[272..+8] = ptr %122
 26151|     ;; near_towers[280..+8] = ptr %3
 26152|     ;; near_towers[288..+8] = ptr %88
 26153|  store i64 %599, ptr %84,                                                                                              ;L507
 26154|  %604 = gep %84, i64 120                                                                                               ;L507
 26155|  store ptr %97, ptr %604,                                                                                              ;L507
 26156|  %605 = gep %84, i64 128                                                                                               ;L507
 26157|  store ptr %95, ptr %605,                                                                                              ;L507
 26158|  %606 = gep %84, i64 256                                                                                               ;L507
 26159|  store ptr %97, ptr %606,                                                                                              ;L507
 26160|  %607 = gep %84, i64 264                                                                                               ;L507
 26161|  store ptr %93, ptr %607,                                                                                              ;L507
 26162|  %608 = gep %84, i64 272                                                                                               ;L507
 26163|  store ptr %122, ptr %608,                                                                                             ;L507
 26164|  %609 = gep %84, i64 280                                                                                               ;L507
 26165|  store ptr %3, ptr %609,                                                                                               ;L507
 26166|  %610 = gep %84, i64 288                                                                                               ;L507
 26167|  store ptr %88, ptr %610,                                                                                              ;L507
 26168|  %611 = gep %84, i64 24
 26169|  %612 = gep %84, i64 16
 26170|  %613 = gep %84, i64 104
 26171|  %614 = gep %84, i64 144
 26172|  %615 = gep %84, i64 160
 26173|  %616 = gep %84, i64 152
 26174|  %617 = gep %84, i64 240
 26175|  %618 = gep %51, i64 8
 26176|  %619 = gep %50, i64 8
 26177|  %620 = gep %50, i64 16
 26178|  %621 = gep %50, i64 24
 26179|  %622 = gep %50, i64 32
 26180|  %623 = gep %50, i64 40
 26181|  %624 = gep %33, i64 8
 26182|  %625 = gep %32, i64 8
 26183|  %626 = gep %32, i64 16
 26184|  %627 = gep %32, i64 24
 26185|  %628 = gep %32, i64 32
 26186|  %629 = gep %32, i64 40
 26187|  %630 = gep %135, i64 8
 26188|  %631 = gep %29, i64 8
 26189|  %632 = gep %28, i64 8
 26190|  %633 = gep %28, i64 16
 26191|  %634 = gep %28, i64 24
 26192|  %635 = gep %28, i64 32
 26193|  %636 = gep %28, i64 40
 26194|  br label %637                                                                                                         ;L507
 26195| 
 26196| 637: ; preds = %3672, %601
 26197|  %638 = phi i64 [ %3673, %3672 ], [ 0, %601 ]
 26198|  %639 = phi i64 [ %3674, %3672 ], [ %163, %601 ]
 26199|  %640 = phi i64 [ %3675, %3672 ], [ %397, %601 ]
 26200|  %641 = phi i64 [ %3676, %3672 ], [ %163, %601 ]
 26201|  br label %642                                                                                                         ;L764<332<82<107<507
 26202| 
 26203| 642: ; preds = %3875, %637
 26204|     ;; score[0..+8] = i64 %640
 26205|     ;; score[0..+8] = i64 %640
 26206|     ;; score[8..+8] = i64 %639
 26207|     ;; score[8..+8] = i64 %639
 26208|     ;; score[16..+8] = i64 %638
 26209|     ;; score[16..+8] = i64 %638
 26210|     ;; tower_well_risk = i64 %641
 26211|     ;; self = ptr %84
 26212|     ;; self = ptr %84
 26214|     ;; opt = ptr %84
 26215|     ;; self = ptr %84
 26217|  %643 = load i64, ptr %84, , !!8                                                                                       ;L764<332<82<107<507
 26218|  %644 = icmp eq i64 %643, -2                                                                                           ;L764<332<82<107<507
 26219|  br i1 %644, label %738, label %645                                                                                    ;L764<332<82<107<507
 26220| 
 26221| 645: ; preds = %642
 26226|     ;; self = ptr %84
 26229|     ;; predicate = ptr %604
 26230|     ;; self = ptr %84
 26232|     ;; opt = ptr %84
 26233|     ;; self = ptr %84
 26235|  %646 = icmp eq i64 %643, -1                                                                                           ;L764<332<169<98<250<332<82<107<507
 26236|  br i1 %646, label %730, label %647                                                                                    ;L764<332<169<98<250<332<82<107<507
 26237| 
 26238| 647: ; preds = %645
 26241|     ;; a = ptr %84
 26243|     ;; self = ptr %84
 26246|     ;; self = ptr %84
 26250|     ;; self = ptr %84
 26254|     ;; self = ptr %84
 26257|     ;; self = ptr %84
 26260|  %648 = trunc nuw i64 %643 to i1                                                                                       ;L396<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26261|  br i1 %648, label %649, label %729                                                                                    ;L396<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26262| 
 26263| 649: ; preds = %647
 26264|     ;; iter = ptr %603
 26266|     ;; self = ptr %603
 26271|     ;; self[0..+8] = ptr %603
 26272|     ;; self[8..+8] = i64 6
 26273|     ;; data[0..+8] = ptr %611
 26274|     ;; data[8..+8] = i64 6
 26275|     ;; f[0..+8] = ptr %611
 26276|     ;; f[8..+8] = i64 6
 26280|     ;; self = ptr %603
 26281|     ;; self = ptr %603
 26282|     ;; self = ptr %603
 26284|     ;; rhs = i64 1
 26285|  %650 = load i64, ptr %603, , !!42196, !!8                                                                             ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26286|  %651 = load i64, ptr %612, , !!42196, !!8                                                                             ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26287|  %652 = icmp ule i64 %650, %651                                                                                        ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26288|     ;; cond = i1 true
 26289|  call void @llvm.assume(i1 %652)                                                                                       ;L210<122<166<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26290|  %653 = load ptr, ptr %604, , !!41918, !!8
 26291|  %654 = load ptr, ptr %605, , !!41918
 26292|  %655 = gep %653, i64 8
 26293|  %656 = icmp ne ptr %654, null
 26294|  %657 = gep %654, i64 24
 26295|  br label %658                                                                                                         ;L167<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26296| 
 26297| 658: ; preds = %726, %649
 26298|  %659 = phi i64 [ %662, %726 ], [ %650, %649 ]
 26299|  %660 = icmp eq i64 %659, %651                                                                                         ;L167<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26300|  br i1 %660, label %729, label %661                                                                                    ;L167<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26301| 
 26302| 661: ; preds = %658
 26303|     ;; i = i64 %659
 26304|     ;; value = i64 %659
 26305|     ;; self = i64 %659
 26306|  %662 = add nuw nsw i64 %659, 1                                                                                        ;L971<63<169<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26307|  store i64 %662, ptr %603, , !!42196                                                                                   ;L63<169<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26309|     ;; f = ptr undef
 26311|     ;; idx = i64 %659
 26312|     ;; index = i64 %659
 26313|     ;; self = i64 %659
 26314|     ;; self[0..+8] = ptr %611
 26315|     ;; slice[0..+8] = ptr %611
 26316|     ;; self[8..+8] = i64 6
 26317|     ;; slice[8..+8] = i64 6
 26318|  %663 = icmp ult i64 %659, 6                                                                                           ;L252<646<219<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26319|  call void @llvm.assume(i1 %663)                                                                                       ;L252<646<219<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26320|  %664 = getelementptr ptr, ptr %611, i64 %659                                                                          ;L253<646<219<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26321|     ;; self = ptr %664
 26322|     ;; self = ptr %664
 26323|     ;; src = ptr %664
 26324|  %665 = load ptr, ptr %664, , !!42269, !!8                                                                             ;L1733<1171<798<219<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26325|     ;; elem = ptr %665
 26329|     ;; inner = ptr %665
 26330|  %666 = icmp eq ptr %665, null                                                                                         ;L819<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26331|  br i1 %666, label %726, label %667                                                                                    ;L819<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26332| 
 26333| 667: ; preds = %661
 26334|     ;; item = ptr %665
 26336|     ;; x = ptr %665
 26348|  %668 = load ptr, ptr %653, , !!42375, !!8, !!8                                                                        ;L493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26349|  %669 = load ptr, ptr %655, , !!42375                                                                                  ;L493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26350|  %670 = gep %665, i64 1632                                                                                             ;L493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26351|  %671 = load i64, ptr %670, , !!42380, !!8                                                                             ;L493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26352|  %672 = gep %665, i64 1640                                                                                             ;L493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26353|  %673 = load i64, ptr %672, , !!42380                                                                                  ;L493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26357|  %674 = load i64, ptr %668, , !!42375, !!8                                                                             ;L395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26358|     ;; a = i64 %674
 26359|     ;; self = i64 %674
 26360|     ;; b = i64 %671
 26361|     ;; other = i64 %671
 26362|  %675 = icmp ult i64 %674, %671                                                                                        ;L3147<8<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26363|  %676 = sub nuw i64 %671, %674                                                                                         ;L3147<8<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26364|  %677 = sub nuw i64 %674, %671                                                                                         ;L3147<8<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26365|  %678 = select i1 %675, i64 %676, i64 %677                                                                             ;L3147<8<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26366|     ;; rhs = i64 %678
 26367|     ;; rhs = i64 %678
 26368|     ;; rhs = i64 %678
 26369|     ;; diff = i64 %678
 26370|     ;; self = i64 %678
 26371|     ;; self = i64 %678
 26372|     ;; self = i64 %678
 26373|  %679 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %678, i64 %678)                                              ;L3178<1288<2517<9<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26374|  %680 = extractvalue { i64, i1 } %679, 0                                                                               ;L3178<1288<2517<9<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26375|  %681 = extractvalue { i64, i1 } %679, 1                                                                               ;L3178<1288<2517<9<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26376|     ;; a = i64 %680
 26377|     ;; self = i64 %680
 26378|     ;; b = i1 %681
 26379|     ;; b = i1 %681
 26380|  br i1 %681, label %682, label %683                                                                                    ;L459<1289<2517<9<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26381| 
 26382| 682: ; preds = %667
 26383|     ;; a = i64 -1
 26384|     ;; self = i64 -1
 26385|  br label %683                                                                                                         ;L2519<9<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26386| 
 26387| 683: ; preds = %682, %667
 26388|  %684 = phi i64 [ -1, %682 ], [ %680, %667 ]                                                                           ;L0<9<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26389|     ;; self = i64 %684
 26390|     ;; a = i64 %684
 26391|  %685 = icmp ne ptr %669, null
 26392|  call void @llvm.assume(i1 %685)
 26393|  %686 = load i64, ptr %669, , !!42375, !!8                                                                             ;L395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26394|     ;; a = i64 %686
 26395|     ;; self = i64 %686
 26396|     ;; b = i64 %673
 26397|     ;; other = i64 %673
 26398|  %687 = icmp ult i64 %686, %673                                                                                        ;L3147<8<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26399|  %688 = sub nuw i64 %673, %686                                                                                         ;L3147<8<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26400|  %689 = sub nuw i64 %686, %673                                                                                         ;L3147<8<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26401|  %690 = select i1 %687, i64 %688, i64 %689                                                                             ;L3147<8<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26402|     ;; rhs = i64 %690
 26403|     ;; rhs = i64 %690
 26404|     ;; rhs = i64 %690
 26405|     ;; diff = i64 %690
 26406|     ;; self = i64 %690
 26407|     ;; self = i64 %690
 26408|     ;; self = i64 %690
 26409|  %691 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %690, i64 %690)                                              ;L3178<1288<2517<9<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26410|  %692 = extractvalue { i64, i1 } %691, 0                                                                               ;L3178<1288<2517<9<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26411|  %693 = extractvalue { i64, i1 } %691, 1                                                                               ;L3178<1288<2517<9<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26412|     ;; a = i64 %692
 26413|     ;; rhs = i64 %692
 26414|     ;; b = i1 %693
 26415|     ;; b = i1 %693
 26416|  br i1 %693, label %694, label %695                                                                                    ;L459<1289<2517<9<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26417| 
 26418| 694: ; preds = %683
 26419|     ;; a = i64 -1
 26420|     ;; rhs = i64 -1
 26421|  br label %695                                                                                                         ;L2519<9<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26422| 
 26423| 695: ; preds = %694, %683
 26424|  %696 = phi i64 [ -1, %694 ], [ %692, %683 ]                                                                           ;L0<9<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26425|     ;; rhs = i64 %696
 26426|     ;; a = i64 %696
 26427|  %697 = call i64 @llvm.uadd.sat.i64(i64 %684, i64 %696)                                                                ;L2428<395<493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26428|  %698 = icmp ult i64 %697, 22500000000                                                                                 ;L493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26429|  br i1 %698, label %833, label %699                                                                                    ;L493<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26430| 
 26431| 699: ; preds = %695
 26432|  call void @llvm.assume(i1 %656)
 26433|     ;; self = ptr %654
 26434|     ;; self = ptr %654
 26435|  %700 = load ptr, ptr %654, , !!42375, !!8, !!8                                                                        ;L138<2073<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26436|     ;; p = ptr %700
 26437|  %701 = load i64, ptr %657, , !!42375, !!8                                                                             ;L2075<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26438|     ;; len = i64 %701
 26439|     ;; count = i64 %701
 26440|     ;; self[0..+8] = ptr %700
 26441|     ;; slice[0..+8] = ptr %700
 26442|     ;; self[8..+8] = i64 %701
 26443|     ;; slice[8..+8] = i64 %701
 26444|     ;; ptr = ptr %700
 26445|     ;; self = ptr %700
 26446|  %702 = gepS, ptr, i64 }, ptr %700, i64 %701                                                                           ;L961<100<1042<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26447|     ;; f = ptr %665
 26448|     ;; self = ptr undef
 26449|     ;; self = ptr undef
 26450|     ;; count = i64 1
 26451|  br label %703                                                                                                         ;L331<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26452| 
 26453| 703: ; preds = %706, %699
 26454|  %704 = phi ptr [ %707, %706 ], [ %700, %699 ]
 26455|     ;; ptr = ptr %704
 26456|     ;; self = ptr %704
 26457|     ;; end_or_len = ptr %702
 26460|  %705 = icmp eq ptr %704, %702                                                                                         ;L1714<180<331<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26461|  br i1 %705, label %726, label %706                                                                                    ;L180<331<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26462| 
 26463| 706: ; preds = %703
 26464|  %707 = gep %704, i64 448                                                                                              ;L656<185<331<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26465|     ;; x = ptr %704
 26466|  %708 = gep %704, i64 432                                                                                              ;L332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26467|  %709 = load ptr, ptr %708, , !!42502, !!8, !!8                                                                        ;L332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26471|     ;; self = ptr %709
 26472|     ;; other = ptr %665
 26473|  %710 = gep %709, i64 1632                                                                                             ;L2158<494<332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26474|  %711 = load i64, ptr %710, , !!42502, !!8                                                                             ;L2158<494<332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26475|     ;; x1 = i64 %711
 26476|     ;; self = i64 %711
 26477|  %712 = gep %709, i64 1640                                                                                             ;L2158<494<332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26478|  %713 = load i64, ptr %712, , !!42502, !!8                                                                             ;L2158<494<332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26479|     ;; y1 = i64 %713
 26480|     ;; self = i64 %713
 26481|     ;; x2 = i64 %671
 26482|     ;; other = i64 %671
 26483|     ;; y2 = i64 %673
 26484|     ;; other = i64 %673
 26485|  %714 = icmp ult i64 %711, %671                                                                                        ;L3147<7<2158<494<332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26486|  %715 = sub nuw i64 %671, %711                                                                                         ;L3147<7<2158<494<332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26487|  %716 = sub nuw i64 %711, %671                                                                                         ;L3147<7<2158<494<332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26488|  %717 = select i1 %714, i64 %715, i64 %716                                                                             ;L3147<7<2158<494<332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26489|     ;; dx = i64 %717
 26490|  %718 = icmp ult i64 %713, %673                                                                                        ;L3147<8<2158<494<332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26491|  %719 = sub nuw i64 %673, %713                                                                                         ;L3147<8<2158<494<332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26492|  %720 = sub nuw i64 %713, %673                                                                                         ;L3147<8<2158<494<332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26493|  %721 = select i1 %718, i64 %719, i64 %720                                                                             ;L3147<8<2158<494<332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26494|     ;; dy = i64 %721
 26495|  %722 = mul i64 %717, %717                                                                                             ;L9<2158<494<332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26496|  %723 = mul i64 %721, %721                                                                                             ;L9<2158<494<332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26497|  %724 = add i64 %723, %722                                                                                             ;L9<2158<494<332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26498|  %725 = icmp ult i64 %724, 4900000000                                                                                  ;L494<332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26499|  br i1 %725, label %726, label %703                                                                                    ;L332<494<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26500| 
 26501| 726: ; preds = %706, %703, %661
 26502|  %727 = phi ptr [ null, %661 ], [ null, %703 ], [ %665, %706 ]                                                         ;L0<220<170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26503|  %728 = icmp eq ptr %727, null                                                                                         ;L170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26504|  br i1 %728, label %658, label %833                                                                                    ;L170<215<263<397<89<880<242<2971<169<332<169<98<250<332<82<107<507
 26505| 
 26506| 729: ; preds = %658, %647
 26507|     ;; x = ptr null
 26508|  store i64 -1, ptr %84, , !!41922                                                                                      ;L334<169<98<250<332<82<107<507
 26509|  br label %730                                                                                                         ;L333<169<98<250<332<82<107<507
 26510| 
 26511| 730: ; preds = %729, %645
 26512|     ;; self = ptr null
 26513|     ;; f[0..+8] = ptr %613
 26514|     ;; f[8..+8] = ptr %604
 26518|     ;; self = ptr %613
 26519|  %731 = load ptr, ptr %613, , !!42601, !!8                                                                             ;L764<170<1653<170<98<250<332<82<107<507
 26520|  %732 = icmp eq ptr %731, null                                                                                         ;L764<170<1653<170<98<250<332<82<107<507
 26521|  br i1 %732, label %737, label %733                                                                                    ;L764<170<1653<170<98<250<332<82<107<507
 26522| 
 26523| 733: ; preds = %730
 26524|     ;; self = ptr %613
 26525|     ;; predicate = ptr %604
 26526|  %734 = invoke ptr @core::iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2p_4find5checkB1s_QNCNvNtCshdEBA0ozCnw_7game_ai13position_eval25position_eval_at_uncacheds_0E0INtNtNtBb_3ops12control_flow11ControlFlowB1s_EEB3E_(ptr %613, ptr %604)
 26527|  to label %735 unwind label %384                                                                                       ;L2971<170<1653<170<98<250<332<82<107<507
 26528| 
 26529| 735: ; preds = %733
 26530|     ;; x = ptr %734
 26533|  %736 = icmp eq ptr %734, null                                                                                         ;L633<682<333<82<107<507
 26534|  br i1 %736, label %737, label %833                                                                                    ;L333<82<107<507
 26535| 
 26536| 737: ; preds = %735, %730
 26537|  store i64 -2, ptr %84,                                                                                                ;L334<82<107<507
 26538|  br label %738                                                                                                         ;L333<82<107<507
 26539| 
 26540| 738: ; preds = %737, %642
 26542|     ;; self = ptr null
 26543|     ;; f = ptr %602
 26546|     ;; self = ptr %602
 26547|  %739 = load i64, ptr %602, , !!42679, !!8                                                                             ;L764<82<1653<82<107<507
 26548|  %740 = icmp eq i64 %739, -2                                                                                           ;L764<82<1653<82<107<507
 26549|  br i1 %740, label %867, label %741                                                                                    ;L764<82<1653<82<107<507
 26550| 
 26551| 741: ; preds = %738
 26553|     ;; self = ptr %602
 26556|     ;; predicate = ptr %606
 26557|     ;; self = ptr %602
 26559|     ;; opt = ptr %602
 26560|     ;; self = ptr %602
 26562|  %742 = icmp eq i64 %739, -1                                                                                           ;L764<332<169<98<82<1653<82<107<507
 26563|  br i1 %742, label %826, label %743                                                                                    ;L764<332<169<98<82<1653<82<107<507
 26564| 
 26565| 743: ; preds = %741
 26568|     ;; a = ptr %602
 26570|     ;; self = ptr %602
 26573|     ;; self = ptr %602
 26577|     ;; self = ptr %602
 26581|     ;; self = ptr %602
 26584|     ;; self = ptr %602
 26587|  %744 = trunc nuw i64 %739 to i1                                                                                       ;L396<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26588|  br i1 %744, label %745, label %825                                                                                    ;L396<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26589| 
 26590| 745: ; preds = %743
 26591|     ;; iter = ptr %614
 26593|     ;; self = ptr %614
 26598|     ;; self[0..+8] = ptr %614
 26599|     ;; self[8..+8] = i64 6
 26600|     ;; data[0..+8] = ptr %615
 26601|     ;; data[8..+8] = i64 6
 26602|     ;; f[0..+8] = ptr %615
 26603|     ;; f[8..+8] = i64 6
 26607|     ;; self = ptr %614
 26608|     ;; self = ptr %614
 26609|     ;; self = ptr %614
 26611|     ;; rhs = i64 1
 26612|  %746 = load i64, ptr %614, , !!42924, !!8                                                                             ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26613|  %747 = load i64, ptr %616, , !!42924, !!8                                                                             ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26614|  %748 = icmp ule i64 %746, %747                                                                                        ;L122<166<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26615|     ;; cond = i1 true
 26616|  call void @llvm.assume(i1 %748)                                                                                       ;L210<122<166<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26617|  %749 = load ptr, ptr %606, , !!42934, !!8
 26618|  %750 = load ptr, ptr %607, , !!42934
 26619|  %751 = gep %749, i64 8
 26620|  %752 = icmp ne ptr %750, null
 26621|  %753 = gep %750, i64 24
 26622|  br label %754                                                                                                         ;L167<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26623| 
 26624| 754: ; preds = %822, %745
 26625|  %755 = phi i64 [ %758, %822 ], [ %746, %745 ]
 26626|  %756 = icmp eq i64 %755, %747                                                                                         ;L167<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26627|  br i1 %756, label %825, label %757                                                                                    ;L167<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26628| 
 26629| 757: ; preds = %754
 26630|     ;; i = i64 %755
 26631|     ;; value = i64 %755
 26632|     ;; self = i64 %755
 26633|  %758 = add nuw nsw i64 %755, 1                                                                                        ;L971<63<169<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26634|  store i64 %758, ptr %614, , !!42924                                                                                   ;L63<169<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26636|     ;; f = ptr undef
 26638|     ;; idx = i64 %755
 26639|     ;; index = i64 %755
 26640|     ;; self = i64 %755
 26641|     ;; self[0..+8] = ptr %615
 26642|     ;; slice[0..+8] = ptr %615
 26643|     ;; self[8..+8] = i64 6
 26644|     ;; slice[8..+8] = i64 6
 26645|  %759 = icmp ult i64 %755, 6                                                                                           ;L252<646<219<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26646|  call void @llvm.assume(i1 %759)                                                                                       ;L252<646<219<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26647|  %760 = getelementptr ptr, ptr %615, i64 %755                                                                          ;L253<646<219<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26648|     ;; self = ptr %760
 26649|     ;; self = ptr %760
 26650|     ;; src = ptr %760
 26651|  %761 = load ptr, ptr %760, , !!42971, !!8                                                                             ;L1733<1171<798<219<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26652|     ;; elem = ptr %761
 26656|     ;; inner = ptr %761
 26657|  %762 = icmp eq ptr %761, null                                                                                         ;L819<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26658|  br i1 %762, label %822, label %763                                                                                    ;L819<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26659| 
 26660| 763: ; preds = %757
 26661|     ;; item = ptr %761
 26663|     ;; x = ptr %761
 26675|  %764 = load ptr, ptr %749, , !!43070, !!8, !!8                                                                        ;L496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26676|  %765 = load ptr, ptr %751, , !!43070                                                                                  ;L496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26677|  %766 = gep %761, i64 1632                                                                                             ;L496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26678|  %767 = load i64, ptr %766, , !!43075, !!8                                                                             ;L496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26679|  %768 = gep %761, i64 1640                                                                                             ;L496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26680|  %769 = load i64, ptr %768, , !!43075                                                                                  ;L496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26684|  %770 = load i64, ptr %764, , !!43070, !!8                                                                             ;L395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26685|     ;; a = i64 %770
 26686|     ;; self = i64 %770
 26687|     ;; b = i64 %767
 26688|     ;; other = i64 %767
 26689|  %771 = icmp ult i64 %770, %767                                                                                        ;L3147<8<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26690|  %772 = sub nuw i64 %767, %770                                                                                         ;L3147<8<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26691|  %773 = sub nuw i64 %770, %767                                                                                         ;L3147<8<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26692|  %774 = select i1 %771, i64 %772, i64 %773                                                                             ;L3147<8<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26693|     ;; rhs = i64 %774
 26694|     ;; rhs = i64 %774
 26695|     ;; rhs = i64 %774
 26696|     ;; diff = i64 %774
 26697|     ;; self = i64 %774
 26698|     ;; self = i64 %774
 26699|     ;; self = i64 %774
 26700|  %775 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %774, i64 %774)                                              ;L3178<1288<2517<9<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26701|  %776 = extractvalue { i64, i1 } %775, 0                                                                               ;L3178<1288<2517<9<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26702|  %777 = extractvalue { i64, i1 } %775, 1                                                                               ;L3178<1288<2517<9<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26703|     ;; a = i64 %776
 26704|     ;; self = i64 %776
 26705|     ;; b = i1 %777
 26706|     ;; b = i1 %777
 26707|  br i1 %777, label %778, label %779                                                                                    ;L459<1289<2517<9<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26708| 
 26709| 778: ; preds = %763
 26710|     ;; a = i64 -1
 26711|     ;; self = i64 -1
 26712|  br label %779                                                                                                         ;L2519<9<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26713| 
 26714| 779: ; preds = %778, %763
 26715|  %780 = phi i64 [ -1, %778 ], [ %776, %763 ]                                                                           ;L0<9<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26716|     ;; self = i64 %780
 26717|     ;; a = i64 %780
 26718|  %781 = icmp ne ptr %765, null
 26719|  call void @llvm.assume(i1 %781)
 26720|  %782 = load i64, ptr %765, , !!43070, !!8                                                                             ;L395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26721|     ;; a = i64 %782
 26722|     ;; self = i64 %782
 26723|     ;; b = i64 %769
 26724|     ;; other = i64 %769
 26725|  %783 = icmp ult i64 %782, %769                                                                                        ;L3147<8<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26726|  %784 = sub nuw i64 %769, %782                                                                                         ;L3147<8<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26727|  %785 = sub nuw i64 %782, %769                                                                                         ;L3147<8<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26728|  %786 = select i1 %783, i64 %784, i64 %785                                                                             ;L3147<8<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26729|     ;; rhs = i64 %786
 26730|     ;; rhs = i64 %786
 26731|     ;; rhs = i64 %786
 26732|     ;; diff = i64 %786
 26733|     ;; self = i64 %786
 26734|     ;; self = i64 %786
 26735|     ;; self = i64 %786
 26736|  %787 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %786, i64 %786)                                              ;L3178<1288<2517<9<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26737|  %788 = extractvalue { i64, i1 } %787, 0                                                                               ;L3178<1288<2517<9<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26738|  %789 = extractvalue { i64, i1 } %787, 1                                                                               ;L3178<1288<2517<9<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26739|     ;; a = i64 %788
 26740|     ;; rhs = i64 %788
 26741|     ;; b = i1 %789
 26742|     ;; b = i1 %789
 26743|  br i1 %789, label %790, label %791                                                                                    ;L459<1289<2517<9<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26744| 
 26745| 790: ; preds = %779
 26746|     ;; a = i64 -1
 26747|     ;; rhs = i64 -1
 26748|  br label %791                                                                                                         ;L2519<9<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26749| 
 26750| 791: ; preds = %790, %779
 26751|  %792 = phi i64 [ -1, %790 ], [ %788, %779 ]                                                                           ;L0<9<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26752|     ;; rhs = i64 %792
 26753|     ;; a = i64 %792
 26754|  %793 = call i64 @llvm.uadd.sat.i64(i64 %780, i64 %792)                                                                ;L2428<395<496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26755|  %794 = icmp ult i64 %793, 22500000000                                                                                 ;L496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26756|  br i1 %794, label %833, label %795                                                                                    ;L496<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26757| 
 26758| 795: ; preds = %791
 26759|  call void @llvm.assume(i1 %752)
 26760|     ;; self = ptr %750
 26761|     ;; self = ptr %750
 26762|  %796 = load ptr, ptr %750, , !!43070, !!8, !!8                                                                        ;L138<2073<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26763|     ;; p = ptr %796
 26764|  %797 = load i64, ptr %753, , !!43070, !!8                                                                             ;L2075<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26765|     ;; len = i64 %797
 26766|     ;; count = i64 %797
 26767|     ;; self[0..+8] = ptr %796
 26768|     ;; slice[0..+8] = ptr %796
 26769|     ;; self[8..+8] = i64 %797
 26770|     ;; slice[8..+8] = i64 %797
 26771|     ;; ptr = ptr %796
 26772|     ;; self = ptr %796
 26773|  %798 = gepS, ptr, i64 }, ptr %796, i64 %797                                                                           ;L961<100<1042<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26774|     ;; f = ptr %761
 26775|     ;; self = ptr undef
 26776|     ;; self = ptr undef
 26777|     ;; count = i64 1
 26778|  br label %799                                                                                                         ;L331<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26779| 
 26780| 799: ; preds = %802, %795
 26781|  %800 = phi ptr [ %803, %802 ], [ %796, %795 ]
 26782|     ;; ptr = ptr %800
 26783|     ;; self = ptr %800
 26784|     ;; end_or_len = ptr %798
 26787|  %801 = icmp eq ptr %800, %798                                                                                         ;L1714<180<331<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26788|  br i1 %801, label %822, label %802                                                                                    ;L180<331<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26789| 
 26790| 802: ; preds = %799
 26791|  %803 = gep %800, i64 448                                                                                              ;L656<185<331<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26792|     ;; x = ptr %800
 26793|  %804 = gep %800, i64 432                                                                                              ;L332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26794|  %805 = load ptr, ptr %804, , !!43174, !!8, !!8                                                                        ;L332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26798|     ;; self = ptr %805
 26799|     ;; other = ptr %761
 26800|  %806 = gep %805, i64 1632                                                                                             ;L2158<497<332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26801|  %807 = load i64, ptr %806, , !!43174, !!8                                                                             ;L2158<497<332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26802|     ;; x1 = i64 %807
 26803|     ;; self = i64 %807
 26804|  %808 = gep %805, i64 1640                                                                                             ;L2158<497<332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26805|  %809 = load i64, ptr %808, , !!43174, !!8                                                                             ;L2158<497<332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26806|     ;; y1 = i64 %809
 26807|     ;; self = i64 %809
 26808|     ;; x2 = i64 %767
 26809|     ;; other = i64 %767
 26810|     ;; y2 = i64 %769
 26811|     ;; other = i64 %769
 26812|  %810 = icmp ult i64 %807, %767                                                                                        ;L3147<7<2158<497<332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26813|  %811 = sub nuw i64 %767, %807                                                                                         ;L3147<7<2158<497<332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26814|  %812 = sub nuw i64 %807, %767                                                                                         ;L3147<7<2158<497<332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26815|  %813 = select i1 %810, i64 %811, i64 %812                                                                             ;L3147<7<2158<497<332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26816|     ;; dx = i64 %813
 26817|  %814 = icmp ult i64 %809, %769                                                                                        ;L3147<8<2158<497<332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26818|  %815 = sub nuw i64 %769, %809                                                                                         ;L3147<8<2158<497<332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26819|  %816 = sub nuw i64 %809, %769                                                                                         ;L3147<8<2158<497<332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26820|  %817 = select i1 %814, i64 %815, i64 %816                                                                             ;L3147<8<2158<497<332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26821|     ;; dy = i64 %817
 26822|  %818 = mul i64 %813, %813                                                                                             ;L9<2158<497<332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26823|  %819 = mul i64 %817, %817                                                                                             ;L9<2158<497<332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26824|  %820 = add i64 %819, %818                                                                                             ;L9<2158<497<332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26825|  %821 = icmp ult i64 %820, 4900000000                                                                                  ;L497<332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26826|  br i1 %821, label %822, label %799                                                                                    ;L332<497<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26827| 
 26828| 822: ; preds = %802, %799, %757
 26829|  %823 = phi ptr [ null, %757 ], [ null, %799 ], [ %761, %802 ]                                                         ;L0<220<170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26830|  %824 = icmp eq ptr %823, null                                                                                         ;L170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26831|  br i1 %824, label %754, label %833                                                                                    ;L170<215<263<397<89<880<242<2971<169<332<169<98<82<1653<82<107<507
 26832| 
 26833| 825: ; preds = %754, %743
 26834|     ;; x = ptr null
 26835|  store i64 -1, ptr %602, , !!43226                                                                                     ;L334<169<98<82<1653<82<107<507
 26836|  br label %826                                                                                                         ;L333<169<98<82<1653<82<107<507
 26837| 
 26838| 826: ; preds = %825, %741
 26839|     ;; self = ptr null
 26840|     ;; f[0..+8] = ptr %617
 26841|     ;; f[8..+8] = ptr %606
 26845|     ;; self = ptr %617
 26846|  %827 = load ptr, ptr %617, , !!43270, !!8                                                                             ;L764<170<1653<170<98<82<1653<82<107<507
 26847|  %828 = icmp eq ptr %827, null                                                                                         ;L764<170<1653<170<98<82<1653<82<107<507
 26848|  br i1 %828, label %867, label %829                                                                                    ;L764<170<1653<170<98<82<1653<82<107<507
 26849| 
 26850| 829: ; preds = %826
 26851|     ;; self = ptr %617
 26852|     ;; predicate = ptr %606
 26853|  %830 = invoke ptr @core::iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2p_4find5checkB1s_QNCNvNtCshdEBA0ozCnw_7game_ai13position_eval25position_eval_at_uncacheds0_0E0INtNtNtBb_3ops12control_flow11ControlFlowB1s_EEB3E_(ptr %617, ptr %606)
 26854|  to label %831 unwind label %384                                                                                       ;L2971<170<1653<170<98<82<1653<82<107<507
 26855| 
 26856| 831: ; preds = %829
 26857|     ;; self = ptr %830
 26858|     ;; f = ptr %608
 26859|     ;; self = ptr %608
 26860|  %832 = icmp eq ptr %830, null                                                                                         ;L1161<107<507
 26861|  br i1 %832, label %867, label %833                                                                                    ;L1161<107<507
 26862| 
 26863| 833: ; preds = %831, %822, %791, %735, %726, %695
 26864|  %834 = phi ptr [ %830, %831 ], [ %823, %822 ], [ %734, %735 ], [ %761, %791 ], [ %727, %726 ], [ %665, %695 ]
 26865|     ;; x = ptr %834
 26866|     ;; args = ptr %834
 26872|     ;; t = ptr %834
 26873|     ;; self = ptr %834
 26874|     ;; self = ptr %834
 26875|  %835 = load ptr, ptr %608, , !!43318, !!8, !!8                                                                        ;L499<310<1162<107<507
 26876|     ;; other = ptr %835
 26877|     ;; other = ptr %835
 26878|  %836 = load i64, ptr %834, , !!43314, !!8                                                                             ;L1127<264<499<310<1162<107<507
 26879|  %837 = gep %834, i64 8                                                                                                ;L1127<264<499<310<1162<107<507
 26880|     ;; __self_discr = i64 %836
 26881|  %838 = load i64, ptr %835, , !!43352, !!8                                                                             ;L1127<264<499<310<1162<107<507
 26882|  %839 = gep %835, i64 8                                                                                                ;L1127<264<499<310<1162<107<507
 26883|     ;; __arg1_discr = i64 %838
 26884|  %840 = icmp eq i64 %836, %838                                                                                         ;L1127<264<499<310<1162<107<507
 26885|  br i1 %840, label %841, label %843                                                                                    ;L1127<264<499<310<1162<107<507
 26886| 
 26887| 841: ; preds = %833
 26888|  %842 = icmp eq i64 %836, 0                                                                                            ;L1127<264<499<310<1162<107<507
 26889|  br i1 %842, label %863, label %3635                                                                                   ;L1127<264<499<310<1162<107<507
 26890| 
 26891| 843: ; preds = %863, %833
 26892|  %844 = load ptr, ptr %609, , !!43318, !!8, !!8                                                                        ;L500<310<1162<107<507
 26893|  %845 = load ptr, ptr %610, , !!43318, !!8, !!8                                                                        ;L500<310<1162<107<507
 26894|  %846 = load i64, ptr %845, , !!43352, !!8                                                                             ;L500<310<1162<107<507
 26895|  %847 = load ptr, ptr %844, , !!43352, !!8, !!8                                                                        ;L500<310<1162<107<507
 26898|  store i64 %846, ptr %54, , !!43359
 26900|     ;; tower = ptr %834
 26901|     ;; minion_team = ptr %54
 26902|     ;; seed = ptr %53
 26903|     ;; tick = ptr %52
 26904|     ;; key = ptr %51
 26906|  %848 = load ptr, ptr %847, , !!43359, !!8, !!8                                                                        ;L185<500<310<1162<107<507
 26907|  %849 = gep %847, i64 8                                                                                                ;L185<500<310<1162<107<507
 26908|  %850 = load ptr, ptr %849, , !!43359, !!8, !!8                                                                        ;L185<500<310<1162<107<507
 26909|  %851 = gep %850, i64 32                                                                                               ;L185<500<310<1162<107<507
 26910|  %852 = load ptr, ptr %851, , !!43359, !!8                                                                             ;L185<500<310<1162<107<507
 26911|  %853 = invoke i64 %852(ptr %848)
 26912|  to label %854 unwind label %384                                                                                       ;L185<500<310<1162<107<507
 26913| 
 26914| 854: ; preds = %843
 26915|  store i64 %853, ptr %53, , !!43359                                                                                    ;L185<500<310<1162<107<507
 26917|  %855 = gep %850, i64 40                                                                                               ;L186<500<310<1162<107<507
 26918|  %856 = load ptr, ptr %855, , !!43359, !!8                                                                             ;L186<500<310<1162<107<507
 26919|  %857 = invoke i64 %856(ptr %848)
 26920|  to label %858 unwind label %384                                                                                       ;L186<500<310<1162<107<507
 26921| 
 26922| 858: ; preds = %854
 26923|  store i64 %857, ptr %52, , !!43359                                                                                    ;L186<500<310<1162<107<507
 26925|  %859 = gep %834, i64 1472                                                                                             ;L187<500<310<1162<107<507
 26926|  %860 = load i64, ptr %859, , !!43314, !!8                                                                             ;L187<500<310<1162<107<507
 26927|  store i64 %860, ptr %51, , !!43359                                                                                    ;L187<500<310<1162<107<507
 26928|  store i64 %846, ptr %618, , !!43359                                                                                   ;L187<500<310<1162<107<507
 26930|  store ptr %53, ptr %50, , !!43359                                                                                     ;L188<500<310<1162<107<507
 26931|  store ptr %52, ptr %619, , !!43359                                                                                    ;L188<500<310<1162<107<507
 26932|  store ptr %51, ptr %620, , !!43359                                                                                    ;L188<500<310<1162<107<507
 26933|  store ptr %847, ptr %621, , !!43359                                                                                   ;L188<500<310<1162<107<507
 26934|  store ptr %54, ptr %622, , !!43359                                                                                    ;L188<500<310<1162<107<507
 26935|  store ptr %834, ptr %623, , !!43359                                                                                   ;L188<500<310<1162<107<507
 26936|  %861 = invoke i64 @core::cell7RefCellNtNtCshdEBA0ozCnw_7game_ai13position_eval19TowerMinionCntCacheEE4withNCNvB1x_34tower_minion_in_range_count_cached0jEB1z_(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.121, ptr %50)
 26937|  to label %862 unwind label %384                                                                                       ;L188<500<310<1162<107<507
 26938| 
 26939| 862: ; preds = %858
 26945|  br label %3635                                                                                                        ;L499<310<1162<107<507
 26946| 
 26947| 863: ; preds = %841
 26948|     ;; __self_0 = ptr %834
 26949|     ;; self = ptr %834
 26950|     ;; __arg1_0 = ptr %835
 26951|     ;; other = ptr %835
 26954|  %864 = load i64, ptr %837, , !!43314, !!8                                                                             ;L1878<2123<1127<264<499<310<1162<107<507
 26955|  %865 = load i64, ptr %839, , !!43352, !!8                                                                             ;L1878<2123<1127<264<499<310<1162<107<507
 26956|  %866 = icmp eq i64 %864, %865                                                                                         ;L1878<2123<1127<264<499<310<1162<107<507
 26957|  br i1 %866, label %3635, label %843                                                                                   ;L499<310<1162<107<507
 26958| 
 26959| 867: ; preds = %831, %826, %738
 26962|  br label %548                                                                                                         ;L490
 26963| 
 26964| 868: ; preds = %590
 26965|  %869 = gep %587, i64 576                                                                                              ;L210<96<1227<96<25<573
 26966|  %870 = getelementptr i64, ptr %869, i64 %591                                                                          ;L210<96<1227<96<25<573
 26967|  %871 = load i64, ptr %870, , !!8                                                                                      ;L210<96<1227<96<25<573
 26968|  %872 = icmp eq i64 %871, 0                                                                                            ;L96<25<573
 26969|  br i1 %872, label %882, label %873                                                                                    ;L573
 26970| 
 26971| 873: ; preds = %868, %574, %564
 26972|  %874 = gep %135, i64 8                                                                                                ;L575
 26973|  %875 = load ptr, ptr %874, , !!8, !!8                                                                                 ;L575
 26974|  %876 = gep %875, i64 4856                                                                                             ;L575
 26975|  %877 = load i64, ptr %876, , !!8                                                                                      ;L575
 26976|  %878 = lshr i64 %877, 1                                                                                               ;L575
 26977|  %879 = load i64, ptr %100, , !!8                                                                                      ;L574
 26978|  %880 = load i64, ptr %99, , !!8                                                                                       ;L574
 26979|  %881 = invoke i64 @ai::minion_wave_risk32enemy_minion_wave_risk_damage_at(i64 poison, ptr %3, ptr %122, i64 %879, i64 %880, i64 %878)
 26980|  to label %883 unwind label %384                                                                                       ;L574
 26981| 
 26982| 882: ; preds = %868, %584, %576
 26983|     ;; range_minion_attack[0..+8] = i64 0
 26984|     ;; range_minion_attack[8..+8] = i64 undef
 26985|     ;; melee_minion_attack[0..+8] = i64 0
 26986|     ;; melee_minion_attack[8..+8] = i64 undef
 26988|  invoke void @gc::simulationNtB5_21AbstractGameWithCache12iter_minions(ptr sret([56 x i8]) %83, ptr %118, i64 %150)
 26989|  to label %973 unwind label %384                                                                                       ;L580
 26990| 
 26991| 883: ; preds = %873
 26992|     ;; minion_wave_damage = i64 %881
 26993|  %884 = gep %122, i64 1576                                                                                             ;L576
 26994|  %885 = load i64, ptr %884,                                                                                            ;L576
 26995|  %886 = load i64, ptr %146, , !!8                                                                                      ;L576
 26998|     ;; purpose = i8 %6
 26999|     ;; damage = i64 %881
 27000|     ;; damage = i64 %881
 27001|     ;; hp = i64 %886
 27002|  %887 = icmp eq i64 %881, 0                                                                                            ;L53<7<576
 27003|  br i1 %887, label %946, label %888                                                                                    ;L53<7<576
 27004| 
 27005| 888: ; preds = %883
 27006|  %889 = mul i64 %881, 100                                                                                              ;L56<7<576
 27007|     ;; self = i64 %886
 27008|     ;; other = i64 1
 27009|  %890 = icmp eq i64 %886, -1                                                                                           ;L56<7<576
 27010|  %891 = icmp eq i64 %889, -9223372036854775808                                                                         ;L56<7<576
 27011|  %892 = and i1 %891, %890                                                                                              ;L56<7<576
 27012|  br i1 %892, label %893, label %895                                                                                    ;L56<7<576
 27013| 
 27014| 893: ; preds = %888
 27015|  invoke void @core::panicking11panic_const24panic_const_div_overflow(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.122) #25
 27016|  to label %894 unwind label %384                                                                                       ;L56<7<576
 27017| 
 27018| 894: ; preds = %893
 27019|  unreachable                                                                                                           ;L56<7<576
 27020| 
 27021| 895: ; preds = %888
 27022|  %896 = call i64 @llvm.umax.i64(i64 %886, i64 1)                                                                       ;L1039<56<7<576
 27023|  %897 = sdiv i64 %889, %896                                                                                            ;L56<7<576
 27024|     ;; self = i64 %897
 27025|     ;; other = i64 150
 27026|  %898 = call i64 @llvm.smin.i64(i64 %897, i64 140)                                                                     ;L1078<7<576
 27027|     ;; self = i64 %898
 27028|     ;; other = i64 140
 27029|     ;; risk = i64 %898
 27030|  %899 = icmp eq i64 %897, 0                                                                                            ;L8<576
 27031|  br i1 %899, label %946, label %900                                                                                    ;L8<576
 27032| 
 27033| 900: ; preds = %895
 27034|     ;; self = ptr %135
 27035|  %901 = load ptr, ptr %118, , !!8, !!8                                                                                 ;L12<576
 27036|  %902 = load ptr, ptr %152, , !!8, !!8                                                                                 ;L12<576
 27037|  %903 = gep %902, i64 40                                                                                               ;L12<576
 27038|  %904 = load ptr, ptr %903, , !!8                                                                                      ;L12<576
 27039|  %905 = invoke i64 %904(ptr %901)
 27040|  to label %906 unwind label %384                                                                                       ;L12<576
 27041| 
 27042| 906: ; preds = %900
 27043|     ;; tick = i64 %905
 27044|     ;; tick = i64 %905
 27045|     ;; self = ptr %135
 27046|  %907 = load i8, ptr %562, , !!8                                                                                       ;L263<399<12<576
 27047|  switch i8 %907, label %917 [
 27048|  i8 0, label %908
 27049|  i8 7, label %908
 27050|  i8 8, label %908
 27051|  i8 5, label %908
 27052|  ]                                                                                                                     ;L263<399<12<576
 27053| 
 27054| 908: ; preds = %906, %906, %906, %906
 27055|  %909 = load ptr, ptr %874, , !!8, !!8                                                                                 ;L399<12<576
 27056|     ;; self = ptr %909
 27057|  %910 = gep %909, i64 2216                                                                                             ;L703<399<12<576
 27058|  %911 = load i64, ptr %910, , !!8                                                                                      ;L703<399<12<576
 27059|     ;; self = i64 %911
 27060|  %912 = gep %909, i64 4856                                                                                             ;L704<399<12<576
 27061|  %913 = load i64, ptr %912, , !!8                                                                                      ;L704<399<12<576
 27062|  %914 = mul i64 %913, 30                                                                                               ;L704<399<12<576
 27063|     ;; rhs = i64 %914
 27064|  %915 = call i64 @llvm.usub.sat.i64(i64 %911, i64 %914)                                                                ;L2472<703<399<12<576
 27065|  %916 = icmp ult i64 %905, %915                                                                                        ;L703<399<12<576
 27066|  br i1 %916, label %917, label %946                                                                                    ;L12<576
 27067| 
 27068| 917: ; preds = %908, %906
 27069|  %918 = icmp ne i8 %6, 9                                                                                               ;L13<576
 27070|  call void @llvm.assume(i1 %918)                                                                                       ;L13<576
 27071|  %919 = icmp samesign ult i8 %6, 2                                                                                     ;L13<576
 27072|  %920 = and i8 %6, 14                                                                                                  ;L13<576
 27073|  %921 = icmp eq i8 %920, 8                                                                                             ;L13<576
 27074|  %922 = or i1 %919, %921                                                                                               ;L13<576
 27075|  br i1 %922, label %923, label %946                                                                                    ;L13<576
 27076| 
 27077| 923: ; preds = %917
 27079|     ;; damage = i64 %881
 27080|     ;; self = i64 %885
 27081|     ;; other = i64 1
 27082|  %924 = call i64 @llvm.umax.i64(i64 %885, i64 1)                                                                       ;L1039<69<14<576
 27083|  %925 = mul i64 %886, 100                                                                                              ;L69<14<576
 27084|  %926 = udiv i64 %925, %924                                                                                            ;L69<14<576
 27085|     ;; hp_pct = i64 %926
 27086|     ;; self = i64 %886
 27087|     ;; other = i64 1
 27088|  %927 = udiv i64 %889, %896                                                                                            ;L70<14<576
 27089|     ;; damage_pct = i64 %927
 27090|  %928 = icmp uge i64 %881, %886                                                                                        ;L71<14<576
 27091|  %929 = icmp ugt i64 %927, 49
 27092|  %930 = or i1 %928, %929                                                                                               ;L71<14<576
 27093|  br i1 %930, label %946, label %931                                                                                    ;L71<14<576
 27094| 
 27095| 931: ; preds = %923
 27096|  %932 = icmp ult i64 %926, 66                                                                                          ;L73<14<576
 27097|  %933 = icmp samesign ugt i64 %927, 29                                                                                 ;L73<14<576
 27098|  %934 = and i1 %932, %933                                                                                              ;L73<14<576
 27099|  br i1 %934, label %946, label %935                                                                                    ;L73<14<576
 27100| 
 27101| 935: ; preds = %931
 27102|  %936 = icmp ult i64 %926, 41                                                                                          ;L74<14<576
 27103|  %937 = icmp samesign ugt i64 %927, 17                                                                                 ;L74<14<576
 27104|  %938 = and i1 %936, %937                                                                                              ;L74<14<576
 27105|  br i1 %938, label %946, label %939                                                                                    ;L74<14<576
 27106| 
 27107| 939: ; preds = %935
 27108|  %940 = icmp ult i64 %926, 26                                                                                          ;L75<14<576
 27109|  %941 = icmp samesign ugt i64 %927, 9
 27110|  %942 = select i1 %940, i1 %941, i1 false                                                                              ;L75<14<576
 27111|  br i1 %942, label %946, label %943                                                                                    ;L14<576
 27112| 
 27113| 943: ; preds = %939
 27114|  %944 = sdiv i64 %898, 4                                                                                               ;L15<576
 27115|     ;; self = i64 %944
 27116|     ;; other = i64 18
 27117|  %945 = call i64 @llvm.smin.i64(i64 %944, i64 18)                                                                      ;L1078<15<576
 27118|     ;; risk = i64 %945
 27119|  br label %946                                                                                                         ;L12<576
 27120| 
 27121| 946: ; preds = %943, %939, %935, %931, %923, %917, %908, %895, %883
 27122|  %947 = phi i64 [ %898, %908 ], [ %898, %917 ], [ %898, %939 ], [ %945, %943 ], [ 0, %895 ], [ %898, %923 ], [ %898, %935 ], [ %898, %931 ], [ 0, %883 ] ;L0<576
 27123|     ;; risk = i64 %947
 27124|  %948 = add i64 %947, %551                                                                                             ;L576
 27125|     ;; score[0..+8] = i64 %948
 27126|     ;; score[0..+8] = i64 %948
 27127|  br label %949                                                                                                         ;L573
 27128| 
 27129| 949: ; preds = %1033, %946
 27130|  %950 = phi i64 [ %991, %1033 ], [ %948, %946 ]                                                                        ;L0
 27131|     ;; score[0..+8] = i64 %950
 27132|     ;; score[0..+8] = i64 %950
 27134|  call void @llvm.memcpy.p0.p0.i64(ptr %81, ptr %89, i64 24, i1 false)                                                  ;L605
 27137|  %951 = gep %81, i64 16                                                                                                ;L825<1004<605
 27138|  %952 = load i32, ptr %951, , !!8                                                                                      ;L825<1004<605
 27139|  %953 = icmp eq i32 %952, -1                                                                                           ;L825<1004<605
 27140|  br i1 %953, label %1034, label %954                                                                                   ;L825<1004<605
 27141| 
 27142| 954: ; preds = %949
 27146|     ;; self = ptr %81
 27147|     ;; order = i8 0
 27148|     ;; order = i8 0
 27149|     ;; val = i64 1
 27150|     ;; order = i8 0
 27151|     ;; val = i64 1
 27152|     ;; order = i8 0
 27153|  %955 = load i64, ptr %81, , !!8                                                                                       ;L185<825<825<1004<605
 27154|  %956 = icmp ult i64 %955, 132                                                                                         ;L185<825<825<1004<605
 27155|  br i1 %956, label %959, label %957                                                                                    ;L185<825<825<1004<605
 27156| 
 27157| 957: ; preds = %954
 27158|  invoke void @core::panicking18panic_bounds_check(i64 %955, i64 132, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.208) #25
 27159|  to label %958 unwind label %384                                                                                       ;L185<825<825<1004<605
 27160| 
 27161| 958: ; preds = %957
 27162|  unreachable                                                                                                           ;L185<825<825<1004<605
 27163| 
 27164| 959: ; preds = %954
 27165|     ;; self = !DIArgList(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 %955)
 27166|  %960 = gep %81, i64 8                                                                                                 ;L185<825<825<1004<605
 27167|  %961 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %960)
 27168|  to label %962 unwind label %384                                                                                       ;L185<825<825<1004<605
 27169| 
 27170| 962: ; preds = %959
 27171|  %963 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %955                                 ;L185<825<825<1004<605
 27172|     ;; self = ptr %963
 27173|  %964 = extractvalue { i64, i32 } %961, 0                                                                              ;L185<825<825<1004<605
 27174|  %965 = extractvalue { i64, i32 } %961, 1                                                                              ;L185<825<825<1004<605
 27176|  %966 = mul i64 %964, 1000000000                                                                                       ;L632<185<825<825<1004<605
 27177|  %967 = icmp ult i32 %965, 1000000000                                                                                  ;L49<632<185<825<825<1004<605
 27178|  call void @llvm.assume(i1 %967)                                                                                       ;L49<632<185<825<825<1004<605
 27179|  %968 = zext nneg i32 %965 to i64                                                                                      ;L632<185<825<825<1004<605
 27180|  %969 = add i64 %966, %968                                                                                             ;L632<185<825<825<1004<605
 27181|     ;; val = i64 %969
 27182|     ;; val = i64 %969
 27183|     ;; dst = ptr %963
 27184|  %970 = atomicrmw add ptr %963, i64 %969 monotonic, , !!43537                                                          ;L3937<3162<185<825<825<1004<605
 27185|  %971 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %955                                 ;L186<825<825<1004<605
 27186|     ;; self = ptr %971
 27187|     ;; dst = ptr %971
 27188|  %972 = atomicrmw add ptr %971, i64 1 monotonic, , !!43537                                                             ;L3937<3162<186<825<825<1004<605
 27189|  br label %1034                                                                                                        ;L825<1004<605
 27190| 
 27191| 973: ; preds = %882
 27192|     ;; predicate = ptr %97
 27194|  call void @llvm.memcpy.p0.p0.i64(ptr %82, ptr %83, i64 56, i1 false)                                                  ;L28<957<581
 27196|  %974 = gep %82, i64 56                                                                                                ;L580
 27197|  store ptr %97, ptr %974,                                                                                              ;L580
 27198|  %975 = gep %82, i64 8
 27199|  %976 = gep %82, i64 24
 27200|  %977 = gep %82, i64 40
 27201|  %978 = gep %41, i64 8
 27202|  %979 = gep %40, i64 8
 27203|  %980 = gep %40, i64 16
 27204|  %981 = gep %40, i64 24
 27205|  %982 = gep %40, i64 32
 27206|  %983 = gep %40, i64 40
 27207|  %984 = gep %37, i64 8
 27208|  %985 = gep %36, i64 8
 27209|  %986 = gep %36, i64 16
 27210|  %987 = gep %36, i64 24
 27211|  %988 = gep %36, i64 32
 27212|  %989 = gep %36, i64 40
 27213|  br label %990                                                                                                         ;L580
 27214| 
 27215| 990: ; preds = %3629, %973
 27216|  %991 = phi i64 [ %551, %973 ], [ %3634, %3629 ]                                                                       ;L0
 27217|  %992 = phi i64 [ undef, %973 ], [ %3600, %3629 ]                                                                      ;L579
 27218|  %993 = phi i64 [ 0, %973 ], [ %3599, %3629 ]                                                                          ;L579
 27219|  %994 = phi i64 [ undef, %973 ], [ %3598, %3629 ]                                                                      ;L0
 27220|  %995 = phi i64 [ 0, %973 ], [ %3597, %3629 ]                                                                          ;L0
 27221|     ;; score[0..+8] = i64 %991
 27222|     ;; score[0..+8] = i64 %991
 27223|     ;; range_minion_attack[0..+8] = i64 %995
 27224|     ;; range_minion_attack[8..+8] = i64 %994
 27225|     ;; melee_minion_attack[0..+8] = i64 %993
 27226|     ;; melee_minion_attack[8..+8] = i64 %992
 27227|     ;; self = ptr %82
 27230|  store ptr %974, ptr %49, , !!43565
 27231|     ;; self = ptr %82
 27232|     ;; predicate = ptr %49
 27234|     ;; opt = ptr %82
 27235|     ;; self = ptr %82
 27236|     ;; f = ptr %49
 27237|  %996 = load i64, ptr %82, , !!43617, !!8                                                                              ;L764<332<169<98<580
 27238|  %997 = trunc nuw i64 %996 to i1                                                                                       ;L764<332<169<98<580
 27239|  br i1 %997, label %998, label %1017                                                                                   ;L764<332<169<98<580
 27240| 
 27241| 998: ; preds = %990
 27243|     ;; predicate = ptr %49
 27244|     ;; a = ptr %975
 27247|  store ptr %49, ptr %48, , !!43637
 27248|     ;; self = ptr %975
 27249|     ;; predicate = ptr %48
 27250|     ;; opt = ptr %975
 27251|     ;; self = ptr %975
 27252|     ;; f = ptr %48
 27253|  %999 = load ptr, ptr %975, , !!43681, !!8                                                                             ;L764<332<169<169<332<169<98<580
 27254|  %1000 = icmp eq ptr %999, null                                                                                        ;L764<332<169<169<332<169<98<580
 27255|  br i1 %1000, label %1007, label %1001                                                                                 ;L764<332<169<169<332<169<98<580
 27256| 
 27257| 1001: ; preds = %998
 27258|     ;; predicate = ptr %48
 27259|     ;; a = ptr %975
 27260|     ;; self = ptr %975
 27261|     ;; predicate = ptr %48
 27262|  %1002 = invoke ptr @core::iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2p_4find5checkB1s_QQQNCNvNtCshdEBA0ozCnw_7game_ai13position_eval25position_eval_at_uncacheds4_0E0INtNtNtBb_3ops12control_flow11ControlFlowB1s_EEB3G_(ptr %975, ptr %48)
 27263|  to label %1003 unwind label %384                                                                                      ;L2971<169<332<169<169<332<169<98<580
 27264| 
 27265| 1003: ; preds = %1001
 27266|     ;; x = ptr %1002
 27269|  %1004 = icmp eq ptr %1002, null                                                                                       ;L633<682<333<169<169<332<169<98<580
 27270|  br i1 %1004, label %1006, label %1005                                                                                 ;L333<169<169<332<169<98<580
 27271| 
 27272| 1005: ; preds = %1003
 27274|     ;; x = ptr %1002
 27277|  br label %1024                                                                                                        ;L333<169<98<580
 27278| 
 27279| 1006: ; preds = %1003
 27280|  store ptr null, ptr %975, , !!43681                                                                                   ;L334<169<169<332<169<98<580
 27281|  br label %1007                                                                                                        ;L333<169<169<332<169<98<580
 27282| 
 27283| 1007: ; preds = %1006, %998
 27284|     ;; self = ptr null
 27285|     ;; f[0..+8] = ptr %976
 27290|     ;; self = ptr %976
 27291|  %1008 = load ptr, ptr %976, , !!43771, !!8                                                                            ;L764<170<1653<170<169<332<169<98<580
 27292|  %1009 = icmp eq ptr %1008, null                                                                                       ;L764<170<1653<170<169<332<169<98<580
 27293|  br i1 %1009, label %1010, label %1011                                                                                 ;L764<170<1653<170<169<332<169<98<580
 27294| 
 27295| 1010: ; preds = %1007
 27297|     ;; x = ptr null
 27300|  br label %1016                                                                                                        ;L333<169<98<580
 27301| 
 27302| 1011: ; preds = %1007
 27303|  %1012 = load ptr, ptr %48, , !!43637, !!8, !!8                                                                        ;L170<169<332<169<98<580
 27304|     ;; f[8..+8] = ptr %1012
 27305|     ;; self = ptr %976
 27306|     ;; predicate = ptr %1012
 27307|  %1013 = invoke ptr @core::iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2p_4find5checkB1s_QQNCNvNtCshdEBA0ozCnw_7game_ai13position_eval25position_eval_at_uncacheds4_0E0INtNtNtBb_3ops12control_flow11ControlFlowB1s_EEB3F_(ptr %976, ptr %1012)
 27308|  to label %1014 unwind label %384                                                                                      ;L2971<170<1653<170<169<332<169<98<580
 27309| 
 27310| 1014: ; preds = %1011
 27312|     ;; x = ptr %1013
 27315|  %1015 = icmp eq ptr %1013, null                                                                                       ;L633<682<333<169<98<580
 27316|  br i1 %1015, label %1016, label %1024                                                                                 ;L333<169<98<580
 27317| 
 27318| 1016: ; preds = %1014, %1010
 27319|  store i64 0, ptr %82, , !!43617                                                                                       ;L334<169<98<580
 27320|  br label %1017                                                                                                        ;L333<169<98<580
 27321| 
 27322| 1017: ; preds = %1016, %990
 27323|     ;; self = ptr null
 27324|     ;; f[0..+8] = ptr %977
 27329|     ;; self = ptr %977
 27330|  %1018 = load ptr, ptr %977, , !!43830, !!8                                                                            ;L764<170<1653<170<98<580
 27331|  %1019 = icmp eq ptr %1018, null                                                                                       ;L764<170<1653<170<98<580
 27332|  br i1 %1019, label %1020, label %1021                                                                                 ;L764<170<1653<170<98<580
 27333| 
 27334| 1020: ; preds = %1017
 27336|  br label %1033                                                                                                        ;L580
 27337| 
 27338| 1021: ; preds = %1017
 27339|  %1022 = load ptr, ptr %49, , !!43565, !!8, !!8                                                                        ;L170<98<580
 27340|     ;; f[8..+8] = ptr %1022
 27341|     ;; self = ptr %977
 27342|     ;; predicate = ptr %1022
 27343|  %1023 = invoke ptr @core::iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2p_4find5checkB1s_QNCNvNtCshdEBA0ozCnw_7game_ai13position_eval25position_eval_at_uncacheds4_0E0INtNtNtBb_3ops12control_flow11ControlFlowB1s_EEB3E_(ptr %977, ptr %1022)
 27344|  to label %1026 unwind label %384                                                                                      ;L2971<170<1653<170<98<580
 27345| 
 27346| 1024: ; preds = %1014, %1005
 27347|  %1025 = phi ptr [ %1013, %1014 ], [ %1002, %1005 ]
 27349|  br label %1028                                                                                                        ;L580
 27350| 
 27351| 1026: ; preds = %1021
 27353|  %1027 = icmp eq ptr %1023, null                                                                                       ;L580
 27354|  br i1 %1027, label %1033, label %1028                                                                                 ;L580
 27355| 
 27356| 1028: ; preds = %1026, %1024
 27357|  %1029 = phi ptr [ %1025, %1024 ], [ %1023, %1026 ]
 27358|     ;; m = ptr %1029
 27359|  %1030 = gep %1029, i64 104                                                                                            ;L582
 27360|  %1031 = load i64, ptr %1030, , !!8                                                                                    ;L582
 27361|  %1032 = icmp eq i64 %1031, 1                                                                                          ;L582
 27362|  br i1 %1032, label %3511, label %3526                                                                                 ;L582
 27363| 
 27364| 1033: ; preds = %1026, %1020
 27366|  br label %949                                                                                                         ;L573
 27367| 
 27368| 1034: ; preds = %962, %949
 27371|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 27372|     ;; order = i8 0
 27373|  %1035 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                   ;L3904<741<176<606
 27374|  %1036 = icmp eq i8 %1035, 0                                                                                           ;L176<606
 27375|  br i1 %1036, label %1043, label %1037                                                                                 ;L176<606
 27376| 
 27377| 1037: ; preds = %1034
 27378|  %1038 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()
 27379|  to label %1039 unwind label %384                                                                                      ;L179<606
 27380| 
 27381| 1039: ; preds = %1037
 27382|  %1040 = extractvalue { i64, i32 } %1038, 0                                                                            ;L179<606
 27383|  %1041 = extractvalue { i64, i32 } %1038, 1                                                                            ;L179<606
 27384|  store i64 114, ptr %80,                                                                                               ;L179<606
 27385|  %1042 = gep %80, i64 8                                                                                                ;L179<606
 27386|  store i64 %1040, ptr %1042,                                                                                           ;L179<606
 27387|  br label %1043                                                                                                        ;L180<606
 27388| 
 27389| 1043: ; preds = %1039, %1034
 27390|  %1044 = phi i32 [ %1041, %1039 ], [ -1, %1034 ]
 27391|  %1045 = gep %80, i64 16                                                                                               ;L0<606
 27392|  store i32 %1044, ptr %1045,                                                                                           ;L0<606
 27393|  %1046 = gep %118, i64 240                                                                                             ;L607
 27394|  %1047 = getelementptr { { ptr, ptr, i64 }, i64 }, ptr %1046, i64 %150                                                 ;L607
 27395|     ;; self = ptr %1047
 27396|     ;; self = ptr %1047
 27397|  %1048 = load ptr, ptr %1047, , !!8, !!8                                                                               ;L138<2073<607
 27398|     ;; p = ptr %1048
 27399|  %1049 = gep %1047, i64 24                                                                                             ;L2075<607
 27400|  %1050 = load i64, ptr %1049, , !!8                                                                                    ;L2075<607
 27401|     ;; len = i64 %1050
 27402|     ;; count = i64 %1050
 27403|     ;; self[0..+8] = ptr %1048
 27404|     ;; slice[0..+8] = ptr %1048
 27405|     ;; self[8..+8] = i64 %1050
 27406|     ;; slice[8..+8] = i64 %1050
 27407|     ;; ptr = ptr %1048
 27408|     ;; self = ptr %1048
 27409|  %1051 = getelementptr ptr, ptr %1048, i64 %1050                                                                       ;L961<100<1042<607
 27410|     ;; iter[0..+8] = ptr %1048
 27411|     ;; iter[8..+8] = ptr %1051
 27412|  %1052 = gep %45, i64 8
 27413|  %1053 = gep %44, i64 8
 27414|  %1054 = gep %44, i64 16
 27415|  %1055 = gep %44, i64 24
 27416|  %1056 = gep %44, i64 32
 27417|  %1057 = gep %44, i64 40
 27418|  %1058 = load ptr, ptr %97,
 27419|  %1059 = load ptr, ptr %158,
 27420|  br label %1123                                                                                                        ;L607
 27421| 
 27422| 1060: ; preds = %3315, %3314, %1984, %1796, %1794, %1334, %1296, %1294, %1254, %1243, %1129, %1114, %1110, %1104
 27423|  %1061 = phi i1 [ true, %1254 ], [ false, %3315 ], [ false, %3314 ], [ false, %1984 ], [ true, %1296 ], [ false, %1796 ], [ false, %1794 ], [ true, %1104 ], [ true, %1294 ], [ true, %1129 ], [ true, %1334 ], [ true, %1110 ], [ true, %1243 ], [ true, %1114 ] ;L0
 27424|  %1062 = cleanuppad within none []
 27425|  br i1 %1061, label %3510, label %3509                                                                                 ;L1181
 27426| 
 27427| 1063: ; preds = %1123, %1093
 27428|  %1064 = phi ptr [ %1067, %1093 ], [ %1127, %1123 ]                                                                    ;L607
 27429|     ;; score[0..+8] = i64 %1126
 27430|     ;; score[0..+8] = i64 %1126
 27431|     ;; iter[0..+8] = ptr %1064
 27432|     ;; self = ptr undef
 27433|     ;; ptr = ptr %1064
 27434|     ;; self = ptr %1064
 27435|     ;; end_or_len = ptr %1051
 27438|  %1065 = icmp eq ptr %1064, %1051                                                                                      ;L1714<180<607
 27439|  br i1 %1065, label %1240, label %1066                                                                                 ;L180<607
 27440| 
 27441| 1066: ; preds = %1063
 27442|  %1067 = gep %1064, i64 8                                                                                              ;L656<185<607
 27443|     ;; iter[0..+8] = ptr %1067
 27444|  %1068 = load ptr, ptr %1064, , !!8, !!8                                                                               ;L607
 27445|     ;; e = ptr %1068
 27446|     ;; caster = ptr %1068
 27447|     ;; self = ptr %1068
 27448|  %1069 = gep %1068, i64 1632                                                                                           ;L608
 27449|  %1070 = load i64, ptr %1069, , !!8                                                                                    ;L608
 27450|  %1071 = gep %1068, i64 1640                                                                                           ;L608
 27451|  %1072 = load i64, ptr %1071,                                                                                          ;L608
 27455|  %1073 = load i64, ptr %1125, , !!8                                                                                    ;L395<608
 27456|     ;; a = i64 %1073
 27457|     ;; self = i64 %1073
 27458|     ;; b = i64 %1070
 27459|     ;; other = i64 %1070
 27460|  %1074 = icmp ult i64 %1073, %1070                                                                                     ;L3147<8<395<608
 27461|  %1075 = sub nuw i64 %1070, %1073                                                                                      ;L3147<8<395<608
 27462|  %1076 = sub nuw i64 %1073, %1070                                                                                      ;L3147<8<395<608
 27463|  %1077 = select i1 %1074, i64 %1075, i64 %1076                                                                         ;L3147<8<395<608
 27464|     ;; rhs = i64 %1077
 27465|     ;; rhs = i64 %1077
 27466|     ;; rhs = i64 %1077
 27467|     ;; diff = i64 %1077
 27468|     ;; self = i64 %1077
 27469|     ;; self = i64 %1077
 27470|     ;; self = i64 %1077
 27471|  %1078 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %1077, i64 %1077)                                           ;L3178<1288<2517<9<395<608
 27472|  %1079 = extractvalue { i64, i1 } %1078, 0                                                                             ;L3178<1288<2517<9<395<608
 27473|  %1080 = extractvalue { i64, i1 } %1078, 1                                                                             ;L3178<1288<2517<9<395<608
 27474|     ;; a = i64 %1079
 27475|     ;; self = i64 %1079
 27476|     ;; b = i1 %1080
 27477|     ;; b = i1 %1080
 27478|  br i1 %1080, label %1081, label %1082                                                                                 ;L459<1289<2517<9<395<608
 27479| 
 27480| 1081: ; preds = %1066
 27481|     ;; a = i64 -1
 27482|     ;; self = i64 -1
 27483|  br label %1082                                                                                                        ;L2519<9<395<608
 27484| 
 27485| 1082: ; preds = %1081, %1066
 27486|  %1083 = phi i64 [ -1, %1081 ], [ %1079, %1066 ]                                                                       ;L0<9<395<608
 27487|     ;; self = i64 %1083
 27488|     ;; a = i64 %1083
 27489|  call void @llvm.assume(i1 %1128)
 27490|  %1084 = load i64, ptr %1124, , !!8                                                                                    ;L395<608
 27491|     ;; a = i64 %1084
 27492|     ;; self = i64 %1084
 27493|     ;; b = i64 %1072
 27494|     ;; other = i64 %1072
 27495|  %1085 = icmp ult i64 %1084, %1072                                                                                     ;L3147<8<395<608
 27496|  %1086 = sub nuw i64 %1072, %1084                                                                                      ;L3147<8<395<608
 27497|  %1087 = sub nuw i64 %1084, %1072                                                                                      ;L3147<8<395<608
 27498|  %1088 = select i1 %1085, i64 %1086, i64 %1087                                                                         ;L3147<8<395<608
 27499|     ;; rhs = i64 %1088
 27500|     ;; rhs = i64 %1088
 27501|     ;; rhs = i64 %1088
 27502|     ;; diff = i64 %1088
 27503|     ;; self = i64 %1088
 27504|     ;; self = i64 %1088
 27505|     ;; self = i64 %1088
 27506|  %1089 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %1088, i64 %1088)                                           ;L3178<1288<2517<9<395<608
 27507|  %1090 = extractvalue { i64, i1 } %1089, 0                                                                             ;L3178<1288<2517<9<395<608
 27508|  %1091 = extractvalue { i64, i1 } %1089, 1                                                                             ;L3178<1288<2517<9<395<608
 27509|     ;; a = i64 %1090
 27510|     ;; rhs = i64 %1090
 27511|     ;; b = i1 %1091
 27512|     ;; b = i1 %1091
 27513|  br i1 %1091, label %1092, label %1093                                                                                 ;L459<1289<2517<9<395<608
 27514| 
 27515| 1092: ; preds = %1082
 27516|     ;; a = i64 -1
 27517|     ;; rhs = i64 -1
 27518|  br label %1093                                                                                                        ;L2519<9<395<608
 27519| 
 27520| 1093: ; preds = %1092, %1082
 27521|  %1094 = phi i64 [ -1, %1092 ], [ %1090, %1082 ]                                                                       ;L0<9<395<608
 27522|     ;; rhs = i64 %1094
 27523|     ;; a = i64 %1094
 27524|  %1095 = call i64 @llvm.uadd.sat.i64(i64 %1083, i64 %1094)                                                             ;L2428<395<608
 27525|  %1096 = icmp ugt i64 %1095, 22500000000                                                                               ;L608
 27526|  br i1 %1096, label %1063, label %1097                                                                                 ;L608
 27527| 
 27528| 1097: ; preds = %1093
 27529|  %1098 = gep %1068, i64 1632
 27530|  %1099 = gep %1068, i64 1640
 27531|     ;; self = ptr %1068
 27532|  %1100 = gep %1068, i64 1168                                                                                           ;L742<611
 27533|  %1101 = gep %1068, i64 1216                                                                                           ;L742<611
 27534|  %1102 = load i32, ptr %1101, , !!8                                                                                    ;L742<611
 27535|  %1103 = icmp eq i32 %1102, -1                                                                                         ;L742<611
 27536|  br i1 %1103, label %1119, label %1104                                                                                 ;L742<611
 27537| 
 27538| 1104: ; preds = %1097
 27539|     ;; atk = ptr %1100
 27540|     ;; self = ptr %1100
 27544|     ;; attacker = ptr %1068
 27545|     ;; target = ptr %122
 27546|     ;; seed = ptr %47
 27547|     ;; tick = ptr %46
 27548|     ;; key = ptr %45
 27550|  %1105 = load ptr, ptr %118, , !!43942, !!8, !!8                                                                       ;L131<612
 27551|  %1106 = load ptr, ptr %152, , !!43942, !!8, !!8                                                                       ;L131<612
 27552|  %1107 = gep %1106, i64 32                                                                                             ;L131<612
 27553|  %1108 = load ptr, ptr %1107, , !!43942, !!8                                                                           ;L131<612
 27554|  %1109 = invoke i64 %1108(ptr %1105)
 27555|  to label %1110 unwind label %1060                                                                                     ;L131<612
 27556| 
 27557| 1110: ; preds = %1104
 27558|  store i64 %1109, ptr %47, , !!43942                                                                                   ;L131<612
 27560|  %1111 = gep %1106, i64 40                                                                                             ;L132<612
 27561|  %1112 = load ptr, ptr %1111, , !!43942, !!8                                                                           ;L132<612
 27562|  %1113 = invoke i64 %1112(ptr %1105)
 27563|  to label %1114 unwind label %1060                                                                                     ;L132<612
 27564| 
 27565| 1114: ; preds = %1110
 27566|  store i64 %1113, ptr %46, , !!43942                                                                                   ;L132<612
 27568|  %1115 = gep %1068, i64 1472                                                                                           ;L133<612
 27569|  %1116 = load i64, ptr %1115, , !!43934, !!8                                                                           ;L133<612
 27570|  %1117 = load i64, ptr %231, , !!43930, !!8                                                                            ;L133<612
 27571|  store i64 %1116, ptr %45, , !!43942                                                                                   ;L133<612
 27572|  store i64 %1117, ptr %1052, , !!43942                                                                                 ;L133<612
 27574|  store ptr %47, ptr %44, , !!43942                                                                                     ;L134<612
 27575|  store ptr %46, ptr %1053, , !!43942                                                                                   ;L134<612
 27576|  store ptr %45, ptr %1054, , !!43942                                                                                   ;L134<612
 27577|  store ptr %1068, ptr %1055, , !!43942                                                                                 ;L134<612
 27578|  store ptr %135, ptr %1056, , !!43942                                                                                  ;L134<612
 27579|  store ptr %122, ptr %1057, , !!43942                                                                                  ;L134<612
 27580|  %1118 = invoke i64 @core::cell7RefCellNtNtCshdEBA0ozCnw_7game_ai13position_eval17AttackDamageCacheEE4withNCNvB1x_29expected_attack_damage_cached0jEB1z_(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.119, ptr %44)
 27581|  to label %1129 unwind label %1060                                                                                     ;L134<612
 27582| 
 27583| 1119: ; preds = %1238, %1235, %1230, %1226, %1097
 27584|  %1120 = phi ptr [ %1124, %1097 ], [ %1197, %1226 ], [ %1197, %1230 ], [ %1197, %1238 ], [ %1197, %1235 ]
 27585|  %1121 = phi ptr [ %1125, %1097 ], [ %1196, %1226 ], [ %1196, %1230 ], [ %1196, %1238 ], [ %1196, %1235 ]
 27586|  %1122 = phi i64 [ %1126, %1097 ], [ %1126, %1226 ], [ %1234, %1230 ], [ %1239, %1238 ], [ %1237, %1235 ]              ;L0
 27587|     ;; score[0..+8] = i64 %1122
 27588|     ;; score[0..+8] = i64 %1122
 27589|  br label %1123                                                                                                        ;L607
 27590| 
 27591| 1123: ; preds = %1119, %1043
 27592|  %1124 = phi ptr [ %1120, %1119 ], [ %1059, %1043 ]
 27593|  %1125 = phi ptr [ %1121, %1119 ], [ %1058, %1043 ]
 27594|  %1126 = phi i64 [ %1122, %1119 ], [ %950, %1043 ]
 27595|  %1127 = phi ptr [ %1067, %1119 ], [ %1048, %1043 ]
 27596|  %1128 = icmp ne ptr %1124, null
 27597|  br label %1063                                                                                                        ;L180<607
 27598| 
 27599| 1129: ; preds = %1114
 27604|     ;; damage = i64 %1118
 27605|     ;; x = i64 %1118
 27606|     ;; inv_hp_q32 = i64 %149
 27607|  %1130 = zext i64 %1118 to i128                                                                                        ;L387<613
 27608|  %1131 = mul nuw nsw i128 %335, %1130                                                                                  ;L387<613
 27609|  %1132 = lshr i128 %1131, 32                                                                                           ;L387<613
 27610|     ;; self = i128 %1132
 27611|     ;; other = i128 150
 27612|  %1133 = call i128 @llvm.umin.i128(i128 %1132, i128 150)                                                               ;L1078<387<613
 27613|  %1134 = trunc nuw nsw i128 %1133 to i64                                                                               ;L387<613
 27614|     ;; ratio = i64 %1134
 27615|  %1135 = gep %1068, i64 1184                                                                                           ;L26<614
 27616|  %1136 = load i64, ptr %1135, , !!8                                                                                    ;L26<614
 27617|  %1137 = gep %1068, i64 1192                                                                                           ;L26<614
 27618|  %1138 = load i64, ptr %1137, , !!8                                                                                    ;L26<614
 27619|  %1139 = gep %1068, i64 1480                                                                                           ;L26<614
 27620|  %1140 = load i64, ptr %1139, , !!8                                                                                    ;L26<614
 27621|  %1141 = gep %1068, i64 1080                                                                                           ;L26<614
 27622|  %1142 = load i64, ptr %1141, , !!8                                                                                    ;L26<614
 27623|  %1143 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %1100, ptr %1068, ptr %122)
 27624|  to label %1144 unwind label %1060                                                                                     ;L614
 27625| 
 27626| 1144: ; preds = %1129
 27627|  %1145 = add i64 %1140, -1                                                                                             ;L26<614
 27628|  %1146 = mul i64 %1145, %1138                                                                                          ;L26<614
 27629|  %1147 = gep %1068, i64 1136                                                                                           ;L1511<614
 27630|  %1148 = load i32, ptr %1147, , !!8                                                                                    ;L1511<614
 27631|     ;; mult = i32 %1148
 27632|  %1149 = icmp eq i32 %1148, 0                                                                                          ;L1512<614
 27633|  br i1 %1149, label %1150, label %1153                                                                                 ;L1512<614
 27634| 
 27635| 1150: ; preds = %1144
 27636|  %1151 = gep %1068, i64 1664                                                                                           ;L1513<614
 27637|  %1152 = load i64, ptr %1151, , !!8                                                                                    ;L1513<614
 27638|  br label %1160                                                                                                        ;L1512<614
 27639| 
 27640| 1153: ; preds = %1144
 27641|  %1154 = sext i32 %1148 to i64                                                                                         ;L1511<614
 27642|     ;; mult = i64 %1154
 27643|  %1155 = gep %1068, i64 1664                                                                                           ;L1515<614
 27644|  %1156 = load i64, ptr %1155, , !!8                                                                                    ;L1515<614
 27645|  %1157 = add nsw i64 %1154, 100                                                                                        ;L1515<614
 27646|  %1158 = mul i64 %1156, %1157                                                                                          ;L1515<614
 27647|  %1159 = udiv i64 %1158, 100                                                                                           ;L1515<614
 27648|  br label %1160                                                                                                        ;L1512<614
 27649| 
 27650| 1160: ; preds = %1153, %1150
 27651|  %1161 = phi i64 [ %1152, %1150 ], [ %1159, %1153 ]                                                                    ;L0<614
 27652|  %1162 = load i32, ptr %342, , !!8                                                                                     ;L1511<614
 27653|     ;; mult = i32 %1162
 27654|  %1163 = icmp eq i32 %1162, 0                                                                                          ;L1512<614
 27655|  br i1 %1163, label %1164, label %1166                                                                                 ;L1512<614
 27656| 
 27657| 1164: ; preds = %1160
 27658|  %1165 = load i64, ptr %343, , !!8                                                                                     ;L1513<614
 27659|  br label %1172                                                                                                        ;L1512<614
 27660| 
 27661| 1166: ; preds = %1160
 27662|  %1167 = sext i32 %1162 to i64                                                                                         ;L1511<614
 27663|     ;; mult = i64 %1167
 27664|  %1168 = load i64, ptr %343, , !!8                                                                                     ;L1515<614
 27665|  %1169 = add nsw i64 %1167, 100                                                                                        ;L1515<614
 27666|  %1170 = mul i64 %1168, %1169                                                                                          ;L1515<614
 27667|  %1171 = udiv i64 %1170, 100                                                                                           ;L1515<614
 27668|  br label %1172                                                                                                        ;L1512<614
 27669| 
 27670| 1172: ; preds = %1166, %1164
 27671|  %1173 = phi i64 [ %1165, %1164 ], [ %1171, %1166 ]                                                                    ;L0<614
 27672|  %1174 = add i64 %1142, %1136                                                                                          ;L26<614
 27673|  %1175 = add i64 %1174, %1146                                                                                          ;L26<614
 27674|  %1176 = add i64 %1175, %1143                                                                                          ;L614
 27675|  %1177 = add i64 %1176, %1161                                                                                          ;L614
 27676|  %1178 = add i64 %1177, %1173                                                                                          ;L614
 27677|     ;; range = i64 %1178
 27678|  %1179 = add i64 %1178, 32000                                                                                          ;L615
 27679|     ;; range_ext = i64 %1179
 27680|  %1180 = gep %1068, i64 104                                                                                            ;L616
 27681|  %1181 = load i64, ptr %1180, , !!8                                                                                    ;L616
 27682|  switch i64 %1181, label %1194 [
 27683|  i64 7, label %1183
 27684|  i64 9, label %1183
 27685|  i64 10, label %1182
 27686|  ]                                                                                                                     ;L616
 27687| 
 27688| 1182: ; preds = %1172
 27689|     ;; info = ptr %1068
 27692|  br label %1183                                                                                                        ;L618
 27693| 
 27694| 1183: ; preds = %1182, %1172, %1172
 27695|  %1184 = phi i64 [ 112, %1182 ], [ 136, %1172 ], [ 136, %1172 ]
 27696|  %1185 = phi i64 [ 120, %1182 ], [ 144, %1172 ], [ 144, %1172 ]
 27697|  %1186 = gep %1068, i64 %1184                                                                                          ;L0
 27698|  %1187 = load i64, ptr %1186, , !!8                                                                                    ;L0
 27699|     ;; nearest_enemy[0..+8] = i64 %1187
 27701|     ;; self = ptr undef
 27703|  %1188 = trunc nuw i64 %1187 to i1                                                                                     ;L2439<622
 27704|  br i1 %1188, label %1189, label %1194                                                                                 ;L2439<622
 27705| 
 27706| 1189: ; preds = %1183
 27707|  %1190 = gep %1068, i64 %1185                                                                                          ;L0
 27708|  %1191 = load i64, ptr %231, , !!8                                                                                     ;L622
 27709|  %1192 = load i64, ptr %1190,                                                                                          ;L0
 27710|     ;; nearest_enemy[8..+8] = i64 %1192
 27711|     ;; l = ptr undef
 27712|     ;; self = ptr undef
 27715|  %1193 = icmp eq i64 %1192, %1191                                                                                      ;L1878<2440<622
 27716|     ;; targeting_me = i1 %1193
 27717|  br label %1194                                                                                                        ;L2445<622
 27718| 
 27719| 1194: ; preds = %1189, %1183, %1172
 27720|  %1195 = phi i1 [ %1193, %1189 ], [ false, %1172 ], [ false, %1183 ]                                                   ;L0<622
 27722|  %1196 = load ptr, ptr %97, , !!8, !!8                                                                                 ;L623
 27723|  %1197 = load ptr, ptr %158,                                                                                           ;L623
 27724|  %1198 = load i64, ptr %1098, , !!8                                                                                    ;L623
 27725|  %1199 = load i64, ptr %1099,                                                                                          ;L623
 27729|  %1200 = load i64, ptr %1196, , !!8                                                                                    ;L395<623
 27730|     ;; a = i64 %1200
 27731|     ;; self = i64 %1200
 27732|     ;; b = i64 %1198
 27733|     ;; other = i64 %1198
 27734|  %1201 = icmp ult i64 %1200, %1198                                                                                     ;L3147<8<395<623
 27735|  %1202 = sub nuw i64 %1198, %1200                                                                                      ;L3147<8<395<623
 27736|  %1203 = sub nuw i64 %1200, %1198                                                                                      ;L3147<8<395<623
 27737|  %1204 = select i1 %1201, i64 %1202, i64 %1203                                                                         ;L3147<8<395<623
 27738|     ;; rhs = i64 %1204
 27739|     ;; rhs = i64 %1204
 27740|     ;; rhs = i64 %1204
 27741|     ;; diff = i64 %1204
 27742|     ;; self = i64 %1204
 27743|     ;; self = i64 %1204
 27744|     ;; self = i64 %1204
 27745|  %1205 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %1204, i64 %1204)                                           ;L3178<1288<2517<9<395<623
 27746|  %1206 = extractvalue { i64, i1 } %1205, 0                                                                             ;L3178<1288<2517<9<395<623
 27747|  %1207 = extractvalue { i64, i1 } %1205, 1                                                                             ;L3178<1288<2517<9<395<623
 27748|     ;; a = i64 %1206
 27749|     ;; self = i64 %1206
 27750|     ;; b = i1 %1207
 27751|     ;; b = i1 %1207
 27752|  br i1 %1207, label %1208, label %1209                                                                                 ;L459<1289<2517<9<395<623
 27753| 
 27754| 1208: ; preds = %1194
 27755|     ;; a = i64 -1
 27756|     ;; self = i64 -1
 27757|  br label %1209                                                                                                        ;L2519<9<395<623
 27758| 
 27759| 1209: ; preds = %1208, %1194
 27760|  %1210 = phi i64 [ -1, %1208 ], [ %1206, %1194 ]                                                                       ;L0<9<395<623
 27761|     ;; self = i64 %1210
 27762|     ;; a = i64 %1210
 27763|  %1211 = icmp ne ptr %1197, null
 27764|  call void @llvm.assume(i1 %1211)
 27765|  %1212 = load i64, ptr %1197, , !!8                                                                                    ;L395<623
 27766|     ;; a = i64 %1212
 27767|     ;; self = i64 %1212
 27768|     ;; b = i64 %1199
 27769|     ;; other = i64 %1199
 27770|  %1213 = icmp ult i64 %1212, %1199                                                                                     ;L3147<8<395<623
 27771|  %1214 = sub nuw i64 %1199, %1212                                                                                      ;L3147<8<395<623
 27772|  %1215 = sub nuw i64 %1212, %1199                                                                                      ;L3147<8<395<623
 27773|  %1216 = select i1 %1213, i64 %1214, i64 %1215                                                                         ;L3147<8<395<623
 27774|     ;; rhs = i64 %1216
 27775|     ;; rhs = i64 %1216
 27776|     ;; rhs = i64 %1216
 27777|     ;; diff = i64 %1216
 27778|     ;; self = i64 %1216
 27779|     ;; self = i64 %1216
 27780|     ;; self = i64 %1216
 27781|  %1217 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %1216, i64 %1216)                                           ;L3178<1288<2517<9<395<623
 27782|  %1218 = extractvalue { i64, i1 } %1217, 0                                                                             ;L3178<1288<2517<9<395<623
 27783|  %1219 = extractvalue { i64, i1 } %1217, 1                                                                             ;L3178<1288<2517<9<395<623
 27784|     ;; a = i64 %1218
 27785|     ;; rhs = i64 %1218
 27786|     ;; b = i1 %1219
 27787|     ;; b = i1 %1219
 27788|  br i1 %1219, label %1220, label %1221                                                                                 ;L459<1289<2517<9<395<623
 27789| 
 27790| 1220: ; preds = %1209
 27791|     ;; a = i64 -1
 27792|     ;; rhs = i64 -1
 27793|  br label %1221                                                                                                        ;L2519<9<395<623
 27794| 
 27795| 1221: ; preds = %1220, %1209
 27796|  %1222 = phi i64 [ -1, %1220 ], [ %1218, %1209 ]                                                                       ;L0<9<395<623
 27797|     ;; rhs = i64 %1222
 27798|     ;; a = i64 %1222
 27799|  %1223 = call i64 @llvm.uadd.sat.i64(i64 %1210, i64 %1222)                                                             ;L2428<395<623
 27800|     ;; dist_sq = i64 %1223
 27801|  %1224 = mul i64 %1178, %1178                                                                                          ;L624
 27802|  %1225 = icmp ugt i64 %1223, %1224                                                                                     ;L624
 27803|  br i1 %1225, label %1226, label %1229                                                                                 ;L624
 27804| 
 27805| 1226: ; preds = %1221
 27806|  %1227 = mul i64 %1179, %1179                                                                                          ;L630
 27807|  %1228 = icmp ugt i64 %1223, %1227                                                                                     ;L630
 27808|  br i1 %1228, label %1119, label %1230                                                                                 ;L630
 27809| 
 27810| 1229: ; preds = %1221
 27811|  br i1 %1195, label %1238, label %1235                                                                                 ;L625
 27812| 
 27813| 1230: ; preds = %1226
 27814|  %1231 = trunc nuw i128 %1133 to i8                                                                                    ;L631
 27815|  %1232 = udiv i8 %1231, 3                                                                                              ;L631
 27816|  %1233 = zext nneg i8 %1232 to i64                                                                                     ;L631
 27817|  %1234 = add i64 %1126, %1233                                                                                          ;L631
 27818|     ;; score[0..+8] = i64 %1234
 27819|     ;; score[0..+8] = i64 %1234
 27820|  br label %1119                                                                                                        ;L630
 27821| 
 27822| 1235: ; preds = %1229
 27823|  %1236 = lshr i64 %1134, 1                                                                                             ;L628
 27824|  %1237 = add i64 %1236, %1126                                                                                          ;L628
 27825|     ;; score[0..+8] = i64 %1237
 27826|     ;; score[0..+8] = i64 %1237
 27827|  br label %1119                                                                                                        ;L625
 27828| 
 27829| 1238: ; preds = %1229
 27830|  %1239 = add i64 %1126, %1134                                                                                          ;L626
 27831|     ;; score[0..+8] = i64 %1239
 27832|     ;; score[0..+8] = i64 %1239
 27833|  br label %1119                                                                                                        ;L625
 27834| 
 27835| 1240: ; preds = %1063
 27836|     ;; has_pcc = i8 0
 27838|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 27839|     ;; order = i8 0
 27840|  %1241 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                   ;L3904<741<176<638
 27841|  %1242 = icmp eq i8 %1241, 0                                                                                           ;L176<638
 27842|  br i1 %1242, label %1245, label %1243                                                                                 ;L176<638
 27843| 
 27844| 1243: ; preds = %1240
 27845|  %1244 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()
 27846|  to label %1250 unwind label %1060                                                                                     ;L179<638
 27847| 
 27848| 1245: ; preds = %1250, %1240
 27849|  %1246 = phi i32 [ %1252, %1250 ], [ -1, %1240 ]
 27850|  %1247 = gep %79, i64 16                                                                                               ;L0<638
 27851|  store i32 %1246, ptr %1247,                                                                                           ;L0<638
 27852|  %1248 = gep %153, i64 528                                                                                             ;L639
 27853|  %1249 = load ptr, ptr %1248, , !!8                                                                                    ;L639
 27854|  invoke void %1249(ptr sret([40 x i8]) %78, ptr %151)
 27855|  to label %1256 unwind label %1254                                                                                     ;L639
 27856| 
 27857| 1250: ; preds = %1243
 27858|  %1251 = extractvalue { i64, i32 } %1244, 0                                                                            ;L179<638
 27859|  %1252 = extractvalue { i64, i32 } %1244, 1                                                                            ;L179<638
 27860|  store i64 73, ptr %79,                                                                                                ;L179<638
 27861|  %1253 = gep %79, i64 8                                                                                                ;L179<638
 27862|  store i64 %1251, ptr %1253,                                                                                           ;L179<638
 27863|  br label %1245                                                                                                        ;L180<638
 27864| 
 27865| 1254: ; preds = %3504, %3492, %3483, %3436, %3409, %3391, %3385, %3371, %3356, %3349, %1264, %1245
 27866|  %1255 = cleanuppad within none []
 27867|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %79) #27 [ "funclet"(token %1255) ] ;L687
 27868|  cleanupret from %1255 unwind label %1060                                                                              ;L687
 27869| 
 27870| 1256: ; preds = %1245
 27872|  call void @llvm.memcpy.p0.p0.i64(ptr %77, ptr %78, i64 40, i1 false)                                                  ;L639
 27873|  %1257 = gep %153, i64 496
 27874|  %1258 = icmp eq i64 %147, -1
 27875|  br label %1259                                                                                                        ;L639
 27876| 
 27877| 1259: ; preds = %3487, %1256
 27878|  %1260 = phi i8 [ %3488, %3487 ], [ 0, %1256 ]
 27879|  %1261 = phi i8 [ %3489, %3487 ], [ 0, %1256 ]
 27880|  %1262 = phi i64 [ %3490, %3487 ], [ %1126, %1256 ]
 27881|  %1263 = phi i8 [ %3491, %3487 ], [ 0, %1256 ]
 27882|  br label %1264                                                                                                        ;L639
 27883| 
 27884| 1264: ; preds = %3477, %1259
 27885|     ;; score[0..+8] = i64 %1262
 27886|     ;; score[0..+8] = i64 %1262
 27887|     ;; score[48..+1] = i8 %1261
 27888|     ;; score[48..+1] = i8 %1261
 27889|     ;; score[49..+1] = i8 %1260
 27890|     ;; score[49..+1] = i8 %1260
 27891|     ;; has_pcc = i8 %1263
 27892|  %1265 = invoke ptr @gc::simulationNtB5_14ProjectileIterNtNtNtNtCsjihNppCmMEE_4core4iter6traits8iterator8Iterator4next(ptr %77)
 27893|  to label %1266 unwind label %1254                                                                                     ;L639
 27894| 
 27895| 1266: ; preds = %1264
 27896|  %1267 = icmp eq ptr %1265, null                                                                                       ;L639
 27897|  br i1 %1267, label %1287, label %1268                                                                                 ;L639
 27898| 
 27899| 1268: ; preds = %1266
 27900|     ;; projectile = ptr %1265
 27901|  %1269 = gep %1265, i64 256                                                                                            ;L640
 27902|  %1270 = load i64, ptr %1269, , !!8                                                                                    ;L640
 27903|     ;; x2 = i64 %1270
 27904|     ;; other = i64 %1270
 27905|     ;; x2 = i64 %1270
 27906|     ;; other = i64 %1270
 27907|     ;; x1 = i64 %1270
 27908|     ;; self = i64 %1270
 27909|  %1271 = gep %1265, i64 264                                                                                            ;L640
 27910|  %1272 = load i64, ptr %1271, , !!8                                                                                    ;L640
 27911|     ;; y2 = i64 %1272
 27912|     ;; other = i64 %1272
 27913|     ;; y2 = i64 %1272
 27914|     ;; other = i64 %1272
 27915|     ;; y1 = i64 %1272
 27916|     ;; self = i64 %1272
 27917|  %1273 = load i64, ptr %100, , !!8                                                                                     ;L3147<7<640
 27918|  %1274 = icmp ult i64 %1273, %1270                                                                                     ;L3147<7<640
 27919|  %1275 = sub nuw i64 %1270, %1273                                                                                      ;L3147<7<640
 27920|  %1276 = sub nuw i64 %1273, %1270                                                                                      ;L3147<7<640
 27921|  %1277 = select i1 %1274, i64 %1275, i64 %1276                                                                         ;L3147<7<640
 27922|     ;; dx = i64 %1277
 27923|  %1278 = load i64, ptr %99, , !!8                                                                                      ;L3147<8<640
 27924|  %1279 = icmp ult i64 %1278, %1272                                                                                     ;L3147<8<640
 27925|  %1280 = sub nuw i64 %1272, %1278                                                                                      ;L3147<8<640
 27926|  %1281 = sub nuw i64 %1278, %1272                                                                                      ;L3147<8<640
 27927|  %1282 = select i1 %1279, i64 %1280, i64 %1281                                                                         ;L3147<8<640
 27928|     ;; dy = i64 %1282
 27929|  %1283 = mul i64 %1277, %1277                                                                                          ;L9<640
 27930|  %1284 = mul i64 %1282, %1282                                                                                          ;L9<640
 27931|  %1285 = add i64 %1284, %1283                                                                                          ;L9<640
 27932|  %1286 = icmp ugt i64 %1285, 62499999999                                                                               ;L640
 27933|  br i1 %1286, label %3477, label %3316                                                                                 ;L640
 27934| 
 27935| 1287: ; preds = %1266
 27939|  %1288 = gep %79, i64 16                                                                                               ;L825<687
 27940|  %1289 = load i32, ptr %1288, , !!8                                                                                    ;L825<687
 27941|  %1290 = icmp eq i32 %1289, -1                                                                                         ;L825<687
 27942|  br i1 %1290, label %1310, label %1291                                                                                 ;L825<687
 27943| 
 27944| 1291: ; preds = %1287
 27948|     ;; self = ptr %79
 27949|     ;; order = i8 0
 27950|     ;; order = i8 0
 27951|     ;; val = i64 1
 27952|     ;; order = i8 0
 27953|     ;; val = i64 1
 27954|     ;; order = i8 0
 27955|  %1292 = load i64, ptr %79, , !!8                                                                                      ;L185<825<825<687
 27956|  %1293 = icmp ult i64 %1292, 132                                                                                       ;L185<825<825<687
 27957|  br i1 %1293, label %1296, label %1294                                                                                 ;L185<825<825<687
 27958| 
 27959| 1294: ; preds = %1291
 27960|  invoke void @core::panicking18panic_bounds_check(i64 %1292, i64 132, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.208) #25
 27961|  to label %1295 unwind label %1060                                                                                     ;L185<825<825<687
 27962| 
 27963| 1295: ; preds = %1294
 27964|  unreachable                                                                                                           ;L185<825<825<687
 27965| 
 27966| 1296: ; preds = %1291
 27967|     ;; self = !DIArgList(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 %1292)
 27968|  %1297 = gep %79, i64 8                                                                                                ;L185<825<825<687
 27969|  %1298 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %1297)
 27970|  to label %1299 unwind label %1060                                                                                     ;L185<825<825<687
 27971| 
 27972| 1299: ; preds = %1296
 27973|  %1300 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %1292                               ;L185<825<825<687
 27974|     ;; self = ptr %1300
 27975|  %1301 = extractvalue { i64, i32 } %1298, 0                                                                            ;L185<825<825<687
 27976|  %1302 = extractvalue { i64, i32 } %1298, 1                                                                            ;L185<825<825<687
 27978|  %1303 = mul i64 %1301, 1000000000                                                                                     ;L632<185<825<825<687
 27979|  %1304 = icmp ult i32 %1302, 1000000000                                                                                ;L49<632<185<825<825<687
 27980|  call void @llvm.assume(i1 %1304)                                                                                      ;L49<632<185<825<825<687
 27981|  %1305 = zext nneg i32 %1302 to i64                                                                                    ;L632<185<825<825<687
 27982|  %1306 = add i64 %1303, %1305                                                                                          ;L632<185<825<825<687
 27983|     ;; val = i64 %1306
 27984|     ;; val = i64 %1306
 27985|     ;; dst = ptr %1300
 27986|  %1307 = atomicrmw add ptr %1300, i64 %1306 monotonic, , !!44086                                                       ;L3937<3162<185<825<825<687
 27987|  %1308 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %1292                               ;L186<825<825<687
 27988|     ;; self = ptr %1308
 27989|     ;; dst = ptr %1308
 27990|  %1309 = atomicrmw add ptr %1308, i64 1 monotonic, , !!44086                                                           ;L3937<3162<186<825<825<687
 27991|  br label %1310                                                                                                        ;L825<687
 27992| 
 27993| 1310: ; preds = %1299, %1287
 27995|  %1311 = load i64, ptr %100, , !!8                                                                                     ;L689
 27996|  %1312 = load i64, ptr %99, , !!8                                                                                      ;L689
 27997|     ;; team = i64 %150
 27998|     ;; x = i64 %1311
 27999|     ;; y = i64 %1312
 28000|     ;; lx0 = i64 0
 28001|     ;; ly0 = i64 800000
 28002|     ;; rx0 = i64 64000
 28003|     ;; ry0 = i64 960000
 28004|     ;; lx1 = i64 0
 28005|     ;; ly1 = i64 896000
 28006|     ;; rx1 = i64 160000
 28007|     ;; ry1 = i64 960000
 28008|     ;; lx0 = i64 800000
 28009|     ;; ly0 = i64 0
 28010|     ;; rx0 = i64 960000
 28011|     ;; ry0 = i64 64000
 28012|     ;; lx1 = i64 896000
 28013|     ;; ly1 = i64 0
 28014|     ;; rx1 = i64 960000
 28015|     ;; ry1 = i64 160000
 28016|  %1313 = icmp eq i64 %112, 1                                                                                           ;L5655<689
 28017|  br i1 %1313, label %1319, label %1314                                                                                 ;L5655<689
 28018| 
 28019| 1314: ; preds = %1310
 28020|  %1315 = add i64 %1311, -800000                                                                                        ;L5664<689
 28021|  %1316 = icmp ult i64 %1315, 160001                                                                                    ;L5664<689
 28022|  %1317 = icmp ult i64 %1312, 64001                                                                                     ;L5664<689
 28023|  %1318 = and i1 %1316, %1317                                                                                           ;L5664<689
 28024|  br i1 %1318, label %1337, label %1324                                                                                 ;L5664<689
 28025| 
 28026| 1319: ; preds = %1310
 28027|  %1320 = icmp ult i64 %1311, 64001                                                                                     ;L5659<689
 28028|  %1321 = add i64 %1312, -800000                                                                                        ;L5659<689
 28029|  %1322 = icmp ult i64 %1321, 160001                                                                                    ;L5659<689
 28030|  %1323 = and i1 %1320, %1322                                                                                           ;L5659<689
 28031|  br i1 %1323, label %1337, label %1329                                                                                 ;L5659<689
 28032| 
 28033| 1324: ; preds = %1314
 28034|  %1325 = add i64 %1311, -896000                                                                                        ;L5664<689
 28035|  %1326 = icmp ult i64 %1325, 64001                                                                                     ;L5664<689
 28036|  %1327 = icmp ult i64 %1312, 160001
 28037|  %1328 = and i1 %1326, %1327                                                                                           ;L5664<689
 28038|  br i1 %1328, label %1337, label %1334                                                                                 ;L689
 28039| 
 28040| 1329: ; preds = %1319
 28041|  %1330 = icmp ult i64 %1311, 160001                                                                                    ;L5659<689
 28042|  %1331 = add i64 %1312, -896000                                                                                        ;L5659<689
 28043|  %1332 = icmp ult i64 %1331, 64001                                                                                     ;L5659<689
 28044|  %1333 = and i1 %1330, %1332                                                                                           ;L5659<689
 28045|  br i1 %1333, label %1337, label %1334                                                                                 ;L689
 28046| 
 28047| 1334: ; preds = %1337, %1329, %1324
 28048|  %1335 = phi i64 [ %1347, %1337 ], [ %1262, %1329 ], [ %1262, %1324 ]                                                  ;L0
 28049|     ;; score[0..+8] = i64 %1335
 28050|     ;; score[0..+8] = i64 %1335
 28051|     ;; max_ratio = i64 0
 28052|  %1336 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity15is_block_attack(ptr %122)
 28053|  to label %1348 unwind label %1060                                                                                     ;L694
 28054| 
 28055| 1337: ; preds = %1329, %1324, %1319, %1314
 28056|  %1338 = gep %135, i64 8                                                                                               ;L690
 28057|  %1339 = load ptr, ptr %1338, , !!8, !!8                                                                               ;L690
 28058|  %1340 = gep %1339, i64 5264                                                                                           ;L690
 28059|  %1341 = load i64, ptr %1340, , !!8                                                                                    ;L690
 28060|     ;; x = i64 %1341
 28061|     ;; inv_hp_q32 = i64 %149
 28062|  %1342 = zext i64 %1341 to i128                                                                                        ;L387<690
 28063|  %1343 = mul nuw nsw i128 %335, %1342                                                                                  ;L387<690
 28064|  %1344 = lshr i128 %1343, 32                                                                                           ;L387<690
 28065|     ;; self = i128 %1344
 28066|     ;; other = i128 150
 28067|  %1345 = call i128 @llvm.umin.i128(i128 %1344, i128 150)                                                               ;L1078<387<690
 28068|  %1346 = trunc nuw nsw i128 %1345 to i64                                                                               ;L387<690
 28069|  %1347 = add i64 %1262, %1346                                                                                          ;L690
 28070|     ;; score[0..+8] = i64 %1347
 28071|     ;; score[0..+8] = i64 %1347
 28072|  br label %1334                                                                                                        ;L689
 28073| 
 28074| 1348: ; preds = %1334
 28075|  br i1 %1336, label %1355, label %1349                                                                                 ;L694
 28076| 
 28077| 1349: ; preds = %1348
 28078|     ;; self = ptr %95
 28079|     ;; self = ptr %95
 28080|  %1350 = load ptr, ptr %95, , !!8, !!8                                                                                 ;L138<2073<695
 28081|     ;; p = ptr %1350
 28082|  %1351 = load i64, ptr %200, , !!8                                                                                     ;L2075<695
 28083|     ;; len = i64 %1351
 28084|     ;; count = i64 %1351
 28085|     ;; self[0..+8] = ptr %1350
 28086|     ;; slice[0..+8] = ptr %1350
 28087|     ;; self[8..+8] = i64 %1351
 28088|     ;; slice[8..+8] = i64 %1351
 28089|     ;; ptr = ptr %1350
 28090|     ;; self = ptr %1350
 28091|  %1352 = mul nuw nsw i64 %1351, 448                                                                                    ;L961<100<1042<695
 28092|  %1353 = gep %1350, i64 %1352                                                                                          ;L961<100<1042<695
 28093|     ;; iter[0..+8] = ptr %1350
 28094|     ;; iter[8..+8] = ptr %1353
 28095|     ;; max_ratio = i64 0
 28096|     ;; self = ptr undef
 28097|     ;; ptr = ptr %1350
 28098|     ;; self = ptr %1350
 28099|     ;; end_or_len = ptr %1353
 28102|  %1354 = icmp eq i64 %1351, 0                                                                                          ;L1714<180<695
 28103|  br i1 %1354, label %1355, label %1360                                                                                 ;L180<695
 28104| 
 28105| 1355: ; preds = %1380, %1349, %1348
 28106|  %1356 = phi i64 [ 0, %1348 ], [ 0, %1349 ], [ %1381, %1380 ]                                                          ;L693
 28107|     ;; max_ratio = i64 %1356
 28108|     ;; self = ptr %122
 28109|  %1357 = gep %122, i64 1272                                                                                            ;L633<706
 28110|  %1358 = load i32, ptr %1357, , !!8                                                                                    ;L633<706
 28111|  %1359 = icmp eq i32 %1358, -1                                                                                         ;L633<706
 28112|  br i1 %1359, label %1397, label %1383                                                                                 ;L706
 28113| 
 28114| 1360: ; preds = %1380, %1349
 28115|  %1361 = phi ptr [ %1363, %1380 ], [ %1350, %1349 ]
 28116|  %1362 = phi i64 [ %1381, %1380 ], [ 0, %1349 ]
 28117|     ;; max_ratio = i64 %1362
 28118|  %1363 = gep %1361, i64 448                                                                                            ;L656<185<695
 28119|     ;; iter[0..+8] = ptr %1363
 28120|     ;; cache = ptr %1361
 28121|     ;; dist = ptr %1361
 28122|  %1364 = gep %1361, i64 32                                                                                             ;L696
 28123|  %1365 = load i64, ptr %1364, , !!8                                                                                    ;L696
 28124|     ;; ratio = i64 %1365
 28125|  %1366 = gep %1361, i64 440                                                                                            ;L697
 28126|  %1367 = load i64, ptr %1366, , !!8                                                                                    ;L697
 28127|     ;; dist = i64 %1367
 28128|  %1368 = gep %1361, i64 200                                                                                            ;L698
 28129|  %1369 = load i64, ptr %1368, , !!8                                                                                    ;L698
 28130|  %1370 = icmp ugt i64 %1367, %1369                                                                                     ;L698
 28131|  br i1 %1370, label %1371, label %1375                                                                                 ;L698
 28132| 
 28133| 1371: ; preds = %1360
 28134|  %1372 = gep %1361, i64 208                                                                                            ;L700
 28135|  %1373 = load i64, ptr %1372, , !!8                                                                                    ;L700
 28136|  %1374 = icmp ugt i64 %1367, %1373                                                                                     ;L700
 28137|  br i1 %1374, label %1380, label %1377                                                                                 ;L700
 28138| 
 28139| 1375: ; preds = %1360
 28140|     ;; self = i64 %1362
 28141|     ;; other = i64 %1365
 28142|  %1376 = call i64 @llvm.smax.i64(i64 %1365, i64 %1362)                                                                 ;L1039<699
 28143|  br label %1380                                                                                                        ;L1039<699
 28144| 
 28145| 1377: ; preds = %1371
 28146|  %1378 = sdiv i64 %1365, 2                                                                                             ;L701
 28147|     ;; self = i64 %1362
 28148|     ;; other = i64 %1378
 28149|  %1379 = call i64 @llvm.smax.i64(i64 %1378, i64 %1362)                                                                 ;L1039<701
 28150|  br label %1380                                                                                                        ;L1039<701
 28151| 
 28152| 1380: ; preds = %1377, %1375, %1371
 28153|  %1381 = phi i64 [ %1379, %1377 ], [ %1362, %1371 ], [ %1376, %1375 ]                                                  ;L0
 28154|     ;; iter[0..+8] = ptr %1363
 28155|     ;; max_ratio = i64 %1381
 28156|     ;; self = ptr undef
 28157|     ;; ptr = ptr %1363
 28158|     ;; self = ptr %1363
 28159|     ;; end_or_len = ptr %1353
 28162|  %1382 = icmp eq ptr %1363, %1353                                                                                      ;L1714<180<695
 28163|  br i1 %1382, label %1355, label %1360                                                                                 ;L180<695
 28164| 
 28165| 1383: ; preds = %1355
 28166|  %1384 = gep %122, i64 104                                                                                             ;L1775<706
 28167|  %1385 = load i64, ptr %1384, , !!8                                                                                    ;L1775<706
 28168|  %1386 = icmp eq i64 %1385, 13                                                                                         ;L1775<706
 28169|  br i1 %1386, label %1387, label %1391                                                                                 ;L1775<706
 28170| 
 28171| 1387: ; preds = %1383
 28172|     ;; champ = ptr %122
 28173|  %1388 = gep %122, i64 184                                                                                             ;L1776<706
 28174|  %1389 = load i64, ptr %1388, , !!8                                                                                    ;L1776<706
 28175|  %1390 = icmp ult i64 %1389, 181                                                                                       ;L706
 28176|  br i1 %1390, label %1391, label %1397                                                                                 ;L706
 28177| 
 28178| 1391: ; preds = %1387, %1383
 28179|     ;; self = ptr %95
 28180|     ;; self = ptr %95
 28181|  %1392 = load ptr, ptr %95, , !!8, !!8                                                                                 ;L138<2073<707
 28182|     ;; p = ptr %1392
 28183|  %1393 = load i64, ptr %200, , !!8                                                                                     ;L2075<707
 28184|     ;; len = i64 %1393
 28185|     ;; count = i64 %1393
 28186|     ;; self[0..+8] = ptr %1392
 28187|     ;; slice[0..+8] = ptr %1392
 28188|     ;; self[8..+8] = i64 %1393
 28189|     ;; slice[8..+8] = i64 %1393
 28190|     ;; ptr = ptr %1392
 28191|     ;; self = ptr %1392
 28192|  %1394 = mul nuw nsw i64 %1393, 448                                                                                    ;L961<100<1042<707
 28193|  %1395 = gep %1392, i64 %1394                                                                                          ;L961<100<1042<707
 28194|     ;; iter[0..+8] = ptr %1392
 28195|     ;; iter[8..+8] = ptr %1395
 28196|     ;; max_ratio = i64 %1356
 28197|     ;; self = ptr undef
 28198|     ;; ptr = ptr %1392
 28199|     ;; self = ptr %1392
 28200|     ;; end_or_len = ptr %1395
 28203|  %1396 = icmp eq i64 %1393, 0                                                                                          ;L1714<180<707
 28204|  br i1 %1396, label %1420, label %1407                                                                                 ;L180<707
 28205| 
 28206| 1397: ; preds = %1522, %1420, %1387, %1355
 28207|  %1398 = phi i64 [ %1356, %1387 ], [ %1356, %1355 ], [ %1421, %1420 ], [ %1523, %1522 ]                                ;L693
 28208|     ;; max_ratio = i64 %1398
 28209|  %1399 = gep %122, i64 1480                                                                                            ;L1693<752
 28210|  %1400 = load i64, ptr %1399, , !!8                                                                                    ;L1693<752
 28211|  %1401 = icmp ugt i64 %1400, 2                                                                                         ;L1693<752
 28212|  %1402 = gep %122, i64 1280                                                                                            ;L1693<752
 28213|  %1403 = select i1 %1401, ptr %1402, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                     ;L1693<752
 28214|     ;; self = ptr %1403
 28215|  %1404 = gep %1403, i64 48                                                                                             ;L633<752
 28216|  %1405 = load i32, ptr %1404, , !!8                                                                                    ;L633<752
 28217|  %1406 = icmp eq i32 %1405, -1                                                                                         ;L633<752
 28218|  br i1 %1406, label %1588, label %1574                                                                                 ;L752
 28219| 
 28220| 1407: ; preds = %1438, %1391
 28221|  %1408 = phi ptr [ %1410, %1438 ], [ %1392, %1391 ]
 28222|  %1409 = phi i64 [ %1439, %1438 ], [ %1356, %1391 ]
 28223|     ;; max_ratio = i64 %1409
 28224|  %1410 = gep %1408, i64 448                                                                                            ;L656<185<707
 28225|     ;; iter[0..+8] = ptr %1410
 28226|     ;; cache = ptr %1408
 28227|     ;; e = ptr %1408
 28228|     ;; dist = ptr %1408
 28229|  %1411 = gep %1408, i64 40                                                                                             ;L708
 28230|  %1412 = load i64, ptr %1411, , !!8                                                                                    ;L708
 28231|     ;; ratio = i64 %1412
 28232|  %1413 = gep %1408, i64 440                                                                                            ;L709
 28233|  %1414 = load i64, ptr %1413, , !!8                                                                                    ;L709
 28234|     ;; dist = i64 %1414
 28235|  %1415 = gep %1408, i64 88                                                                                             ;L710
 28236|  %1416 = load i64, ptr %1415, , !!8                                                                                    ;L710
 28237|     ;; range = i64 %1416
 28238|  %1417 = gep %1408, i64 216                                                                                            ;L0
 28239|  %1418 = load i64, ptr %1417, , !!8                                                                                    ;L0
 28240|  %1419 = icmp ugt i64 %1414, %1418                                                                                     ;L0
 28241|  br i1 %286, label %1428, label %1427                                                                                  ;L711
 28242| 
 28243| 1420: ; preds = %1438, %1391
 28244|  %1421 = phi i64 [ %1356, %1391 ], [ %1439, %1438 ]                                                                    ;L0
 28245|     ;; self = ptr %93
 28246|     ;; self = ptr %93
 28247|  %1422 = load ptr, ptr %93, , !!8, !!8                                                                                 ;L138<2073<728
 28248|     ;; p = ptr %1422
 28249|  %1423 = load i64, ptr %230, , !!8                                                                                     ;L2075<728
 28250|     ;; len = i64 %1423
 28251|     ;; count = i64 %1423
 28252|     ;; self[0..+8] = ptr %1422
 28253|     ;; slice[0..+8] = ptr %1422
 28254|     ;; self[8..+8] = i64 %1423
 28255|     ;; slice[8..+8] = i64 %1423
 28256|     ;; ptr = ptr %1422
 28257|     ;; self = ptr %1422
 28258|  %1424 = mul nuw nsw i64 %1423, 448                                                                                    ;L961<100<1042<728
 28259|  %1425 = gep %1422, i64 %1424                                                                                          ;L961<100<1042<728
 28260|     ;; iter[0..+8] = ptr %1422
 28261|     ;; iter[8..+8] = ptr %1425
 28262|     ;; max_ratio = i64 %1421
 28263|     ;; self = ptr undef
 28264|     ;; ptr = ptr %1422
 28265|     ;; self = ptr %1422
 28266|     ;; end_or_len = ptr %1425
 28269|  %1426 = icmp eq i64 %1423, 0                                                                                          ;L1714<180<728
 28270|  br i1 %1426, label %1397, label %1496                                                                                 ;L180<728
 28271| 
 28272| 1427: ; preds = %1407
 28273|  br i1 %1419, label %1429, label %1433                                                                                 ;L721
 28274| 
 28275| 1428: ; preds = %1407
 28276|  br i1 %1419, label %1441, label %1445                                                                                 ;L712
 28277| 
 28278| 1429: ; preds = %1427
 28279|  %1430 = gep %1408, i64 224                                                                                            ;L723
 28280|  %1431 = load i64, ptr %1430, , !!8                                                                                    ;L723
 28281|  %1432 = icmp ugt i64 %1414, %1431                                                                                     ;L723
 28282|  br i1 %1432, label %1438, label %1435                                                                                 ;L723
 28283| 
 28284| 1433: ; preds = %1427
 28285|     ;; self = i64 %1409
 28286|     ;; other = i64 %1412
 28287|  %1434 = call i64 @llvm.smax.i64(i64 %1412, i64 %1409)                                                                 ;L1039<722
 28288|  br label %1438                                                                                                        ;L1039<722
 28289| 
 28290| 1435: ; preds = %1429
 28291|  %1436 = sdiv i64 %1412, 2                                                                                             ;L724
 28292|     ;; self = i64 %1409
 28293|     ;; other = i64 %1436
 28294|  %1437 = call i64 @llvm.smax.i64(i64 %1436, i64 %1409)                                                                 ;L1039<724
 28295|  br label %1438                                                                                                        ;L1039<724
 28296| 
 28297| 1438: ; preds = %1493, %1491, %1457, %1455, %1451, %1435, %1433, %1429
 28298|  %1439 = phi i64 [ %1456, %1455 ], [ %1434, %1433 ], [ %1437, %1435 ], [ %1409, %1429 ], [ %1495, %1493 ], [ %1459, %1457 ], [ %1409, %1451 ], [ %1492, %1491 ] ;L0
 28299|     ;; iter[0..+8] = ptr %1410
 28300|     ;; max_ratio = i64 %1439
 28301|     ;; self = ptr undef
 28302|     ;; ptr = ptr %1410
 28303|     ;; self = ptr %1410
 28304|     ;; end_or_len = ptr %1395
 28307|  %1440 = icmp eq ptr %1410, %1395                                                                                      ;L1714<180<707
 28308|  br i1 %1440, label %1420, label %1407                                                                                 ;L180<707
 28309| 
 28310| 1441: ; preds = %1428
 28311|  %1442 = gep %1408, i64 232                                                                                            ;L714
 28312|  %1443 = load i64, ptr %1442, , !!8                                                                                    ;L714
 28313|  %1444 = icmp ugt i64 %1414, %1443                                                                                     ;L714
 28314|  br i1 %1444, label %1451, label %1455                                                                                 ;L714
 28315| 
 28316| 1445: ; preds = %1428
 28317|  %1446 = gep %1408, i64 432                                                                                            ;L712
 28318|  %1447 = load ptr, ptr %1446, , !!8, !!8                                                                               ;L712
 28319|     ;; self = ptr %1447
 28320|  %1448 = gep %1447, i64 1136                                                                                           ;L1511<712
 28321|  %1449 = load i32, ptr %1448, , !!8                                                                                    ;L1511<712
 28322|     ;; mult = i32 %1449
 28323|  %1450 = icmp eq i32 %1449, 0                                                                                          ;L1512<712
 28324|  br i1 %1450, label %1460, label %1463                                                                                 ;L1512<712
 28325| 
 28326| 1451: ; preds = %1441
 28327|  %1452 = gep %1408, i64 224                                                                                            ;L718
 28328|  %1453 = load i64, ptr %1452, , !!8                                                                                    ;L718
 28329|  %1454 = icmp ugt i64 %1414, %1453                                                                                     ;L718
 28330|  br i1 %1454, label %1438, label %1457                                                                                 ;L718
 28331| 
 28332| 1455: ; preds = %1487, %1441
 28333|     ;; self = i64 %1409
 28334|     ;; other = i64 %1412
 28335|  %1456 = call i64 @llvm.smax.i64(i64 %1412, i64 %1409)                                                                 ;L1039<715
 28336|  br label %1438                                                                                                        ;L1039<715
 28337| 
 28338| 1457: ; preds = %1451
 28339|  %1458 = sdiv i64 %1412, 3                                                                                             ;L719
 28340|     ;; self = i64 %1409
 28341|     ;; other = i64 %1458
 28342|  %1459 = call i64 @llvm.smax.i64(i64 %1458, i64 %1409)                                                                 ;L1039<719
 28343|  br label %1438                                                                                                        ;L1039<719
 28344| 
 28345| 1460: ; preds = %1445
 28346|  %1461 = gep %1447, i64 1664                                                                                           ;L1513<712
 28347|  %1462 = load i64, ptr %1461, , !!8                                                                                    ;L1513<712
 28348|  br label %1470                                                                                                        ;L1512<712
 28349| 
 28350| 1463: ; preds = %1445
 28351|  %1464 = sext i32 %1449 to i64                                                                                         ;L1511<712
 28352|     ;; mult = i64 %1464
 28353|  %1465 = gep %1447, i64 1664                                                                                           ;L1515<712
 28354|  %1466 = load i64, ptr %1465, , !!8                                                                                    ;L1515<712
 28355|  %1467 = add nsw i64 %1464, 100                                                                                        ;L1515<712
 28356|  %1468 = mul i64 %1466, %1467                                                                                          ;L1515<712
 28357|  %1469 = udiv i64 %1468, 100                                                                                           ;L1515<712
 28358|  br label %1470                                                                                                        ;L1512<712
 28359| 
 28360| 1470: ; preds = %1463, %1460
 28361|  %1471 = phi i64 [ %1462, %1460 ], [ %1469, %1463 ]                                                                    ;L0<712
 28362|  %1472 = add i64 %1471, 80000                                                                                          ;L712
 28363|  %1473 = load i32, ptr %342, , !!8                                                                                     ;L1511<712
 28364|     ;; mult = i32 %1473
 28365|  %1474 = icmp eq i32 %1473, 0                                                                                          ;L1512<712
 28366|  br i1 %1474, label %1475, label %1477                                                                                 ;L1512<712
 28367| 
 28368| 1475: ; preds = %1470
 28369|  %1476 = load i64, ptr %343, , !!8                                                                                     ;L1513<712
 28370|  br label %1483                                                                                                        ;L1512<712
 28371| 
 28372| 1477: ; preds = %1470
 28373|  %1478 = sext i32 %1473 to i64                                                                                         ;L1511<712
 28374|     ;; mult = i64 %1478
 28375|  %1479 = load i64, ptr %343, , !!8                                                                                     ;L1515<712
 28376|  %1480 = add nsw i64 %1478, 100                                                                                        ;L1515<712
 28377|  %1481 = mul i64 %1479, %1480                                                                                          ;L1515<712
 28378|  %1482 = udiv i64 %1481, 100                                                                                           ;L1515<712
 28379|  br label %1483                                                                                                        ;L1512<712
 28380| 
 28381| 1483: ; preds = %1477, %1475
 28382|  %1484 = phi i64 [ %1476, %1475 ], [ %1482, %1477 ]                                                                    ;L0<712
 28383|  %1485 = add i64 %1472, %1484                                                                                          ;L712
 28384|  %1486 = icmp ugt i64 %1416, %1485                                                                                     ;L712
 28385|  br i1 %1486, label %1487, label %1491                                                                                 ;L712
 28386| 
 28387| 1487: ; preds = %1483
 28388|  %1488 = gep %1408, i64 232                                                                                            ;L714
 28389|  %1489 = load i64, ptr %1488, , !!8                                                                                    ;L714
 28390|  %1490 = icmp ugt i64 %1414, %1489                                                                                     ;L714
 28391|  br i1 %1490, label %1493, label %1455                                                                                 ;L714
 28392| 
 28393| 1491: ; preds = %1483
 28394|     ;; self = i64 %1409
 28395|     ;; other = i64 %1412
 28396|  %1492 = call i64 @llvm.smax.i64(i64 %1412, i64 %1409)                                                                 ;L1039<713
 28397|  br label %1438                                                                                                        ;L1039<713
 28398| 
 28399| 1493: ; preds = %1487
 28400|  %1494 = sdiv i64 %1412, 2                                                                                             ;L717
 28401|     ;; self = i64 %1409
 28402|     ;; other = i64 %1494
 28403|  %1495 = call i64 @llvm.smax.i64(i64 %1494, i64 %1409)                                                                 ;L1039<717
 28404|  br label %1438                                                                                                        ;L1039<717
 28405| 
 28406| 1496: ; preds = %1522, %1420
 28407|  %1497 = phi ptr [ %1499, %1522 ], [ %1422, %1420 ]
 28408|  %1498 = phi i64 [ %1523, %1522 ], [ %1421, %1420 ]
 28409|     ;; max_ratio = i64 %1498
 28410|  %1499 = gep %1497, i64 448                                                                                            ;L656<185<728
 28411|     ;; iter[0..+8] = ptr %1499
 28412|     ;; cache = ptr %1497
 28413|     ;; a = ptr %1497
 28414|     ;; dist = ptr %1497
 28415|  %1500 = gep %1497, i64 40                                                                                             ;L729
 28416|  %1501 = load i64, ptr %1500, , !!8                                                                                    ;L729
 28417|     ;; ratio = i64 %1501
 28418|  %1502 = gep %1497, i64 440                                                                                            ;L730
 28419|  %1503 = load i64, ptr %1502, , !!8                                                                                    ;L730
 28420|     ;; dist = i64 %1503
 28421|  %1504 = gep %1497, i64 88                                                                                             ;L733
 28422|  %1505 = load i64, ptr %1504, , !!8                                                                                    ;L733
 28423|     ;; range = i64 %1505
 28424|  %1506 = gep %1497, i64 216                                                                                            ;L733
 28425|  %1507 = load i64, ptr %1506, , !!8                                                                                    ;L733
 28426|     ;; r_sq = i64 %1507
 28427|  %1508 = gep %1497, i64 232                                                                                            ;L733
 28428|  %1509 = load i64, ptr %1508, , !!8                                                                                    ;L733
 28429|     ;; r_half_sq = i64 %1509
 28430|  %1510 = gep %1497, i64 224                                                                                            ;L733
 28431|  %1511 = load i64, ptr %1510, , !!8                                                                                    ;L733
 28432|     ;; r_ext_sq = i64 %1511
 28433|  %1512 = icmp ugt i64 %1503, %1507                                                                                     ;L0
 28434|  br i1 %286, label %1514, label %1513                                                                                  ;L734
 28435| 
 28436| 1513: ; preds = %1496
 28437|  br i1 %1512, label %1515, label %1517                                                                                 ;L744
 28438| 
 28439| 1514: ; preds = %1496
 28440|  br i1 %1512, label %1525, label %1527                                                                                 ;L735
 28441| 
 28442| 1515: ; preds = %1513
 28443|  %1516 = icmp ugt i64 %1503, %1511                                                                                     ;L746
 28444|  br i1 %1516, label %1522, label %1519                                                                                 ;L746
 28445| 
 28446| 1517: ; preds = %1513
 28447|     ;; self = i64 %1498
 28448|     ;; other = i64 %1501
 28449|  %1518 = call i64 @llvm.smax.i64(i64 %1501, i64 %1498)                                                                 ;L1039<745
 28450|  br label %1522                                                                                                        ;L1039<745
 28451| 
 28452| 1519: ; preds = %1515
 28453|  %1520 = sdiv i64 %1501, 2                                                                                             ;L747
 28454|     ;; self = i64 %1498
 28455|     ;; other = i64 %1520
 28456|  %1521 = call i64 @llvm.smax.i64(i64 %1520, i64 %1498)                                                                 ;L1039<747
 28457|  br label %1522                                                                                                        ;L1039<747
 28458| 
 28459| 1522: ; preds = %1571, %1569, %1537, %1535, %1533, %1519, %1517, %1515
 28460|  %1523 = phi i64 [ %1536, %1535 ], [ %1518, %1517 ], [ %1521, %1519 ], [ %1498, %1515 ], [ %1573, %1571 ], [ %1539, %1537 ], [ %1498, %1533 ], [ %1570, %1569 ] ;L0
 28461|     ;; iter[0..+8] = ptr %1499
 28462|     ;; max_ratio = i64 %1523
 28463|     ;; self = ptr undef
 28464|     ;; ptr = ptr %1499
 28465|     ;; self = ptr %1499
 28466|     ;; end_or_len = ptr %1425
 28469|  %1524 = icmp eq ptr %1499, %1425                                                                                      ;L1714<180<728
 28470|  br i1 %1524, label %1397, label %1496                                                                                 ;L180<728
 28471| 
 28472| 1525: ; preds = %1514
 28473|  %1526 = icmp ugt i64 %1503, %1509                                                                                     ;L737
 28474|  br i1 %1526, label %1533, label %1535                                                                                 ;L737
 28475| 
 28476| 1527: ; preds = %1514
 28477|  %1528 = gep %1497, i64 432                                                                                            ;L735
 28478|  %1529 = load ptr, ptr %1528, , !!8, !!8                                                                               ;L735
 28479|     ;; self = ptr %1529
 28480|  %1530 = gep %1529, i64 1136                                                                                           ;L1511<735
 28481|  %1531 = load i32, ptr %1530, , !!8                                                                                    ;L1511<735
 28482|     ;; mult = i32 %1531
 28483|  %1532 = icmp eq i32 %1531, 0                                                                                          ;L1512<735
 28484|  br i1 %1532, label %1540, label %1543                                                                                 ;L1512<735
 28485| 
 28486| 1533: ; preds = %1525
 28487|  %1534 = icmp ugt i64 %1503, %1511                                                                                     ;L741
 28488|  br i1 %1534, label %1522, label %1537                                                                                 ;L741
 28489| 
 28490| 1535: ; preds = %1567, %1525
 28491|     ;; self = i64 %1498
 28492|     ;; other = i64 %1501
 28493|  %1536 = call i64 @llvm.smax.i64(i64 %1501, i64 %1498)                                                                 ;L1039<738
 28494|  br label %1522                                                                                                        ;L1039<738
 28495| 
 28496| 1537: ; preds = %1533
 28497|  %1538 = sdiv i64 %1501, 3                                                                                             ;L742
 28498|     ;; self = i64 %1498
 28499|     ;; other = i64 %1538
 28500|  %1539 = call i64 @llvm.smax.i64(i64 %1538, i64 %1498)                                                                 ;L1039<742
 28501|  br label %1522                                                                                                        ;L1039<742
 28502| 
 28503| 1540: ; preds = %1527
 28504|  %1541 = gep %1529, i64 1664                                                                                           ;L1513<735
 28505|  %1542 = load i64, ptr %1541, , !!8                                                                                    ;L1513<735
 28506|  br label %1550                                                                                                        ;L1512<735
 28507| 
 28508| 1543: ; preds = %1527
 28509|  %1544 = sext i32 %1531 to i64                                                                                         ;L1511<735
 28510|     ;; mult = i64 %1544
 28511|  %1545 = gep %1529, i64 1664                                                                                           ;L1515<735
 28512|  %1546 = load i64, ptr %1545, , !!8                                                                                    ;L1515<735
 28513|  %1547 = add nsw i64 %1544, 100                                                                                        ;L1515<735
 28514|  %1548 = mul i64 %1546, %1547                                                                                          ;L1515<735
 28515|  %1549 = udiv i64 %1548, 100                                                                                           ;L1515<735
 28516|  br label %1550                                                                                                        ;L1512<735
 28517| 
 28518| 1550: ; preds = %1543, %1540
 28519|  %1551 = phi i64 [ %1542, %1540 ], [ %1549, %1543 ]                                                                    ;L0<735
 28520|  %1552 = add i64 %1551, 80000                                                                                          ;L735
 28521|  %1553 = load i32, ptr %342, , !!8                                                                                     ;L1511<735
 28522|     ;; mult = i32 %1553
 28523|  %1554 = icmp eq i32 %1553, 0                                                                                          ;L1512<735
 28524|  br i1 %1554, label %1555, label %1557                                                                                 ;L1512<735
 28525| 
 28526| 1555: ; preds = %1550
 28527|  %1556 = load i64, ptr %343, , !!8                                                                                     ;L1513<735
 28528|  br label %1563                                                                                                        ;L1512<735
 28529| 
 28530| 1557: ; preds = %1550
 28531|  %1558 = sext i32 %1553 to i64                                                                                         ;L1511<735
 28532|     ;; mult = i64 %1558
 28533|  %1559 = load i64, ptr %343, , !!8                                                                                     ;L1515<735
 28534|  %1560 = add nsw i64 %1558, 100                                                                                        ;L1515<735
 28535|  %1561 = mul i64 %1559, %1560                                                                                          ;L1515<735
 28536|  %1562 = udiv i64 %1561, 100                                                                                           ;L1515<735
 28537|  br label %1563                                                                                                        ;L1512<735
 28538| 
 28539| 1563: ; preds = %1557, %1555
 28540|  %1564 = phi i64 [ %1556, %1555 ], [ %1562, %1557 ]                                                                    ;L0<735
 28541|  %1565 = add i64 %1552, %1564                                                                                          ;L735
 28542|  %1566 = icmp ugt i64 %1505, %1565                                                                                     ;L735
 28543|  br i1 %1566, label %1567, label %1569                                                                                 ;L735
 28544| 
 28545| 1567: ; preds = %1563
 28546|  %1568 = icmp ugt i64 %1503, %1509                                                                                     ;L737
 28547|  br i1 %1568, label %1571, label %1535                                                                                 ;L737
 28548| 
 28549| 1569: ; preds = %1563
 28550|     ;; self = i64 %1498
 28551|     ;; other = i64 %1501
 28552|  %1570 = call i64 @llvm.smax.i64(i64 %1501, i64 %1498)                                                                 ;L1039<736
 28553|  br label %1522                                                                                                        ;L1039<736
 28554| 
 28555| 1571: ; preds = %1567
 28556|  %1572 = sdiv i64 %1501, 2                                                                                             ;L740
 28557|     ;; self = i64 %1498
 28558|     ;; other = i64 %1572
 28559|  %1573 = call i64 @llvm.smax.i64(i64 %1572, i64 %1498)                                                                 ;L1039<740
 28560|  br label %1522                                                                                                        ;L1039<740
 28561| 
 28562| 1574: ; preds = %1397
 28563|  %1575 = gep %122, i64 104                                                                                             ;L1790<752
 28564|  %1576 = load i64, ptr %1575, , !!8                                                                                    ;L1790<752
 28565|  %1577 = icmp eq i64 %1576, 13                                                                                         ;L1790<752
 28566|  br i1 %1577, label %1578, label %1582                                                                                 ;L1790<752
 28567| 
 28568| 1578: ; preds = %1574
 28569|     ;; champ = ptr %122
 28570|  %1579 = gep %122, i64 192                                                                                             ;L1791<752
 28571|  %1580 = load i64, ptr %1579, , !!8                                                                                    ;L1791<752
 28572|  %1581 = icmp ult i64 %1580, 181                                                                                       ;L752
 28573|  br i1 %1581, label %1582, label %1588                                                                                 ;L752
 28574| 
 28575| 1582: ; preds = %1578, %1574
 28576|     ;; self = ptr %95
 28577|     ;; self = ptr %95
 28578|  %1583 = load ptr, ptr %95, , !!8, !!8                                                                                 ;L138<2073<753
 28579|     ;; p = ptr %1583
 28580|  %1584 = load i64, ptr %200, , !!8                                                                                     ;L2075<753
 28581|     ;; len = i64 %1584
 28582|     ;; count = i64 %1584
 28583|     ;; self[0..+8] = ptr %1583
 28584|     ;; slice[0..+8] = ptr %1583
 28585|     ;; self[8..+8] = i64 %1584
 28586|     ;; slice[8..+8] = i64 %1584
 28587|     ;; ptr = ptr %1583
 28588|     ;; self = ptr %1583
 28589|  %1585 = mul nuw nsw i64 %1584, 448                                                                                    ;L961<100<1042<753
 28590|  %1586 = gep %1583, i64 %1585                                                                                          ;L961<100<1042<753
 28591|     ;; iter[0..+8] = ptr %1583
 28592|     ;; iter[8..+8] = ptr %1586
 28593|     ;; max_ratio = i64 %1398
 28594|     ;; self = ptr undef
 28595|     ;; ptr = ptr %1583
 28596|     ;; self = ptr %1583
 28597|     ;; end_or_len = ptr %1586
 28600|  %1587 = icmp eq i64 %1584, 0                                                                                          ;L1714<180<753
 28601|  br i1 %1587, label %1609, label %1596                                                                                 ;L180<753
 28602| 
 28603| 1588: ; preds = %1709, %1609, %1578, %1397
 28604|  %1589 = phi i64 [ %1398, %1578 ], [ %1398, %1397 ], [ %1610, %1609 ], [ %1710, %1709 ]                                ;L693
 28605|     ;; max_ratio = i64 %1589
 28606|  %1590 = icmp ugt i64 %1400, 4                                                                                         ;L1701<796
 28607|  %1591 = gep %122, i64 1336                                                                                            ;L1701<796
 28608|  %1592 = select i1 %1590, ptr %1591, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                     ;L1701<796
 28609|     ;; self = ptr %1592
 28610|  %1593 = gep %1592, i64 48                                                                                             ;L633<796
 28611|  %1594 = load i32, ptr %1593, , !!8                                                                                    ;L633<796
 28612|  %1595 = icmp eq i32 %1594, -1                                                                                         ;L633<796
 28613|  br i1 %1595, label %1781, label %1767                                                                                 ;L796
 28614| 
 28615| 1596: ; preds = %1627, %1582
 28616|  %1597 = phi ptr [ %1599, %1627 ], [ %1583, %1582 ]
 28617|  %1598 = phi i64 [ %1628, %1627 ], [ %1398, %1582 ]
 28618|     ;; max_ratio = i64 %1598
 28619|  %1599 = gep %1597, i64 448                                                                                            ;L656<185<753
 28620|     ;; iter[0..+8] = ptr %1599
 28621|     ;; cache = ptr %1597
 28622|     ;; e = ptr %1597
 28623|     ;; dist = ptr %1597
 28624|  %1600 = gep %1597, i64 48                                                                                             ;L754
 28625|  %1601 = load i64, ptr %1600, , !!8                                                                                    ;L754
 28626|     ;; ratio = i64 %1601
 28627|  %1602 = gep %1597, i64 440                                                                                            ;L755
 28628|  %1603 = load i64, ptr %1602, , !!8                                                                                    ;L755
 28629|     ;; dist = i64 %1603
 28630|  %1604 = gep %1597, i64 96                                                                                             ;L756
 28631|  %1605 = load i64, ptr %1604, , !!8                                                                                    ;L756
 28632|     ;; range = i64 %1605
 28633|  %1606 = gep %1597, i64 240                                                                                            ;L0
 28634|  %1607 = load i64, ptr %1606, , !!8                                                                                    ;L0
 28635|  %1608 = icmp ugt i64 %1603, %1607                                                                                     ;L0
 28636|  br i1 %289, label %1617, label %1616                                                                                  ;L757
 28637| 
 28638| 1609: ; preds = %1627, %1582
 28639|  %1610 = phi i64 [ %1398, %1582 ], [ %1628, %1627 ]                                                                    ;L0
 28640|     ;; self = ptr %93
 28641|     ;; self = ptr %93
 28642|  %1611 = load ptr, ptr %93, , !!8, !!8                                                                                 ;L138<2073<774
 28643|     ;; p = ptr %1611
 28644|  %1612 = load i64, ptr %230, , !!8                                                                                     ;L2075<774
 28645|     ;; len = i64 %1612
 28646|     ;; count = i64 %1612
 28647|     ;; self[0..+8] = ptr %1611
 28648|     ;; slice[0..+8] = ptr %1611
 28649|     ;; self[8..+8] = i64 %1612
 28650|     ;; slice[8..+8] = i64 %1612
 28651|     ;; ptr = ptr %1611
 28652|     ;; self = ptr %1611
 28653|  %1613 = mul nuw nsw i64 %1612, 448                                                                                    ;L961<100<1042<774
 28654|  %1614 = gep %1611, i64 %1613                                                                                          ;L961<100<1042<774
 28655|     ;; iter[0..+8] = ptr %1611
 28656|     ;; iter[8..+8] = ptr %1614
 28657|     ;; max_ratio = i64 %1610
 28658|     ;; self = ptr undef
 28659|     ;; ptr = ptr %1611
 28660|     ;; self = ptr %1611
 28661|     ;; end_or_len = ptr %1614
 28664|  %1615 = icmp eq i64 %1612, 0                                                                                          ;L1714<180<774
 28665|  br i1 %1615, label %1588, label %1685                                                                                 ;L180<774
 28666| 
 28667| 1616: ; preds = %1596
 28668|  br i1 %1608, label %1618, label %1622                                                                                 ;L767
 28669| 
 28670| 1617: ; preds = %1596
 28671|  br i1 %1608, label %1630, label %1634                                                                                 ;L758
 28672| 
 28673| 1618: ; preds = %1616
 28674|  %1619 = gep %1597, i64 248                                                                                            ;L769
 28675|  %1620 = load i64, ptr %1619, , !!8                                                                                    ;L769
 28676|  %1621 = icmp ugt i64 %1603, %1620                                                                                     ;L769
 28677|  br i1 %1621, label %1627, label %1624                                                                                 ;L769
 28678| 
 28679| 1622: ; preds = %1616
 28680|     ;; self = i64 %1598
 28681|     ;; other = i64 %1601
 28682|  %1623 = call i64 @llvm.smax.i64(i64 %1601, i64 %1598)                                                                 ;L1039<768
 28683|  br label %1627                                                                                                        ;L1039<768
 28684| 
 28685| 1624: ; preds = %1618
 28686|  %1625 = sdiv i64 %1601, 2                                                                                             ;L770
 28687|     ;; self = i64 %1598
 28688|     ;; other = i64 %1625
 28689|  %1626 = call i64 @llvm.smax.i64(i64 %1625, i64 %1598)                                                                 ;L1039<770
 28690|  br label %1627                                                                                                        ;L1039<770
 28691| 
 28692| 1627: ; preds = %1682, %1680, %1646, %1644, %1640, %1624, %1622, %1618
 28693|  %1628 = phi i64 [ %1645, %1644 ], [ %1623, %1622 ], [ %1626, %1624 ], [ %1598, %1618 ], [ %1684, %1682 ], [ %1648, %1646 ], [ %1598, %1640 ], [ %1681, %1680 ] ;L0
 28694|     ;; iter[0..+8] = ptr %1599
 28695|     ;; max_ratio = i64 %1628
 28696|     ;; self = ptr undef
 28697|     ;; ptr = ptr %1599
 28698|     ;; self = ptr %1599
 28699|     ;; end_or_len = ptr %1586
 28702|  %1629 = icmp eq ptr %1599, %1586                                                                                      ;L1714<180<753
 28703|  br i1 %1629, label %1609, label %1596                                                                                 ;L180<753
 28704| 
 28705| 1630: ; preds = %1617
 28706|  %1631 = gep %1597, i64 256                                                                                            ;L760
 28707|  %1632 = load i64, ptr %1631, , !!8                                                                                    ;L760
 28708|  %1633 = icmp ugt i64 %1603, %1632                                                                                     ;L760
 28709|  br i1 %1633, label %1640, label %1644                                                                                 ;L760
 28710| 
 28711| 1634: ; preds = %1617
 28712|  %1635 = gep %1597, i64 432                                                                                            ;L758
 28713|  %1636 = load ptr, ptr %1635, , !!8, !!8                                                                               ;L758
 28714|     ;; self = ptr %1636
 28715|  %1637 = gep %1636, i64 1136                                                                                           ;L1511<758
 28716|  %1638 = load i32, ptr %1637, , !!8                                                                                    ;L1511<758
 28717|     ;; mult = i32 %1638
 28718|  %1639 = icmp eq i32 %1638, 0                                                                                          ;L1512<758
 28719|  br i1 %1639, label %1649, label %1652                                                                                 ;L1512<758
 28720| 
 28721| 1640: ; preds = %1630
 28722|  %1641 = gep %1597, i64 248                                                                                            ;L764
 28723|  %1642 = load i64, ptr %1641, , !!8                                                                                    ;L764
 28724|  %1643 = icmp ugt i64 %1603, %1642                                                                                     ;L764
 28725|  br i1 %1643, label %1627, label %1646                                                                                 ;L764
 28726| 
 28727| 1644: ; preds = %1676, %1630
 28728|     ;; self = i64 %1598
 28729|     ;; other = i64 %1601
 28730|  %1645 = call i64 @llvm.smax.i64(i64 %1601, i64 %1598)                                                                 ;L1039<761
 28731|  br label %1627                                                                                                        ;L1039<761
 28732| 
 28733| 1646: ; preds = %1640
 28734|  %1647 = sdiv i64 %1601, 3                                                                                             ;L765
 28735|     ;; self = i64 %1598
 28736|     ;; other = i64 %1647
 28737|  %1648 = call i64 @llvm.smax.i64(i64 %1647, i64 %1598)                                                                 ;L1039<765
 28738|  br label %1627                                                                                                        ;L1039<765
 28739| 
 28740| 1649: ; preds = %1634
 28741|  %1650 = gep %1636, i64 1664                                                                                           ;L1513<758
 28742|  %1651 = load i64, ptr %1650, , !!8                                                                                    ;L1513<758
 28743|  br label %1659                                                                                                        ;L1512<758
 28744| 
 28745| 1652: ; preds = %1634
 28746|  %1653 = sext i32 %1638 to i64                                                                                         ;L1511<758
 28747|     ;; mult = i64 %1653
 28748|  %1654 = gep %1636, i64 1664                                                                                           ;L1515<758
 28749|  %1655 = load i64, ptr %1654, , !!8                                                                                    ;L1515<758
 28750|  %1656 = add nsw i64 %1653, 100                                                                                        ;L1515<758
 28751|  %1657 = mul i64 %1655, %1656                                                                                          ;L1515<758
 28752|  %1658 = udiv i64 %1657, 100                                                                                           ;L1515<758
 28753|  br label %1659                                                                                                        ;L1512<758
 28754| 
 28755| 1659: ; preds = %1652, %1649
 28756|  %1660 = phi i64 [ %1651, %1649 ], [ %1658, %1652 ]                                                                    ;L0<758
 28757|  %1661 = add i64 %1660, 80000                                                                                          ;L758
 28758|  %1662 = load i32, ptr %342, , !!8                                                                                     ;L1511<758
 28759|     ;; mult = i32 %1662
 28760|  %1663 = icmp eq i32 %1662, 0                                                                                          ;L1512<758
 28761|  br i1 %1663, label %1664, label %1666                                                                                 ;L1512<758
 28762| 
 28763| 1664: ; preds = %1659
 28764|  %1665 = load i64, ptr %343, , !!8                                                                                     ;L1513<758
 28765|  br label %1672                                                                                                        ;L1512<758
 28766| 
 28767| 1666: ; preds = %1659
 28768|  %1667 = sext i32 %1662 to i64                                                                                         ;L1511<758
 28769|     ;; mult = i64 %1667
 28770|  %1668 = load i64, ptr %343, , !!8                                                                                     ;L1515<758
 28771|  %1669 = add nsw i64 %1667, 100                                                                                        ;L1515<758
 28772|  %1670 = mul i64 %1668, %1669                                                                                          ;L1515<758
 28773|  %1671 = udiv i64 %1670, 100                                                                                           ;L1515<758
 28774|  br label %1672                                                                                                        ;L1512<758
 28775| 
 28776| 1672: ; preds = %1666, %1664
 28777|  %1673 = phi i64 [ %1665, %1664 ], [ %1671, %1666 ]                                                                    ;L0<758
 28778|  %1674 = add i64 %1661, %1673                                                                                          ;L758
 28779|  %1675 = icmp ugt i64 %1605, %1674                                                                                     ;L758
 28780|  br i1 %1675, label %1676, label %1680                                                                                 ;L758
 28781| 
 28782| 1676: ; preds = %1672
 28783|  %1677 = gep %1597, i64 256                                                                                            ;L760
 28784|  %1678 = load i64, ptr %1677, , !!8                                                                                    ;L760
 28785|  %1679 = icmp ugt i64 %1603, %1678                                                                                     ;L760
 28786|  br i1 %1679, label %1682, label %1644                                                                                 ;L760
 28787| 
 28788| 1680: ; preds = %1672
 28789|     ;; self = i64 %1598
 28790|     ;; other = i64 %1601
 28791|  %1681 = call i64 @llvm.smax.i64(i64 %1601, i64 %1598)                                                                 ;L1039<759
 28792|  br label %1627                                                                                                        ;L1039<759
 28793| 
 28794| 1682: ; preds = %1676
 28795|  %1683 = sdiv i64 %1601, 2                                                                                             ;L763
 28796|     ;; self = i64 %1598
 28797|     ;; other = i64 %1683
 28798|  %1684 = call i64 @llvm.smax.i64(i64 %1683, i64 %1598)                                                                 ;L1039<763
 28799|  br label %1627                                                                                                        ;L1039<763
 28800| 
 28801| 1685: ; preds = %1709, %1609
 28802|  %1686 = phi ptr [ %1688, %1709 ], [ %1611, %1609 ]
 28803|  %1687 = phi i64 [ %1710, %1709 ], [ %1610, %1609 ]
 28804|     ;; max_ratio = i64 %1687
 28805|  %1688 = gep %1686, i64 448                                                                                            ;L656<185<774
 28806|     ;; iter[0..+8] = ptr %1688
 28807|     ;; cache = ptr %1686
 28808|     ;; a = ptr %1686
 28809|     ;; dist = ptr %1686
 28810|  %1689 = gep %1686, i64 48                                                                                             ;L775
 28811|  %1690 = load i64, ptr %1689, , !!8                                                                                    ;L775
 28812|     ;; ratio = i64 %1690
 28813|  %1691 = gep %1686, i64 440                                                                                            ;L776
 28814|  %1692 = load i64, ptr %1691, , !!8                                                                                    ;L776
 28815|     ;; dist = i64 %1692
 28816|  %1693 = gep %1686, i64 96                                                                                             ;L777
 28817|  %1694 = load i64, ptr %1693, , !!8                                                                                    ;L777
 28818|     ;; range = i64 %1694
 28819|  %1695 = gep %1686, i64 240                                                                                            ;L0
 28820|  %1696 = load i64, ptr %1695, , !!8                                                                                    ;L0
 28821|  %1697 = icmp ugt i64 %1692, %1696                                                                                     ;L0
 28822|  br i1 %289, label %1699, label %1698                                                                                  ;L778
 28823| 
 28824| 1698: ; preds = %1685
 28825|  br i1 %1697, label %1700, label %1704                                                                                 ;L788
 28826| 
 28827| 1699: ; preds = %1685
 28828|  br i1 %1697, label %1712, label %1716                                                                                 ;L779
 28829| 
 28830| 1700: ; preds = %1698
 28831|  %1701 = gep %1686, i64 248                                                                                            ;L790
 28832|  %1702 = load i64, ptr %1701, , !!8                                                                                    ;L790
 28833|  %1703 = icmp ugt i64 %1692, %1702                                                                                     ;L790
 28834|  br i1 %1703, label %1709, label %1706                                                                                 ;L790
 28835| 
 28836| 1704: ; preds = %1698
 28837|     ;; self = i64 %1687
 28838|     ;; other = i64 %1690
 28839|  %1705 = call i64 @llvm.smax.i64(i64 %1690, i64 %1687)                                                                 ;L1039<789
 28840|  br label %1709                                                                                                        ;L1039<789
 28841| 
 28842| 1706: ; preds = %1700
 28843|  %1707 = sdiv i64 %1690, 2                                                                                             ;L791
 28844|     ;; self = i64 %1687
 28845|     ;; other = i64 %1707
 28846|  %1708 = call i64 @llvm.smax.i64(i64 %1707, i64 %1687)                                                                 ;L1039<791
 28847|  br label %1709                                                                                                        ;L1039<791
 28848| 
 28849| 1709: ; preds = %1764, %1762, %1728, %1726, %1722, %1706, %1704, %1700
 28850|  %1710 = phi i64 [ %1727, %1726 ], [ %1705, %1704 ], [ %1708, %1706 ], [ %1687, %1700 ], [ %1766, %1764 ], [ %1730, %1728 ], [ %1687, %1722 ], [ %1763, %1762 ] ;L0
 28851|     ;; iter[0..+8] = ptr %1688
 28852|     ;; max_ratio = i64 %1710
 28853|     ;; self = ptr undef
 28854|     ;; ptr = ptr %1688
 28855|     ;; self = ptr %1688
 28856|     ;; end_or_len = ptr %1614
 28859|  %1711 = icmp eq ptr %1688, %1614                                                                                      ;L1714<180<774
 28860|  br i1 %1711, label %1588, label %1685                                                                                 ;L180<774
 28861| 
 28862| 1712: ; preds = %1699
 28863|  %1713 = gep %1686, i64 256                                                                                            ;L781
 28864|  %1714 = load i64, ptr %1713, , !!8                                                                                    ;L781
 28865|  %1715 = icmp ugt i64 %1692, %1714                                                                                     ;L781
 28866|  br i1 %1715, label %1722, label %1726                                                                                 ;L781
 28867| 
 28868| 1716: ; preds = %1699
 28869|  %1717 = gep %1686, i64 432                                                                                            ;L779
 28870|  %1718 = load ptr, ptr %1717, , !!8, !!8                                                                               ;L779
 28871|     ;; self = ptr %1718
 28872|  %1719 = gep %1718, i64 1136                                                                                           ;L1511<779
 28873|  %1720 = load i32, ptr %1719, , !!8                                                                                    ;L1511<779
 28874|     ;; mult = i32 %1720
 28875|  %1721 = icmp eq i32 %1720, 0                                                                                          ;L1512<779
 28876|  br i1 %1721, label %1731, label %1734                                                                                 ;L1512<779
 28877| 
 28878| 1722: ; preds = %1712
 28879|  %1723 = gep %1686, i64 248                                                                                            ;L785
 28880|  %1724 = load i64, ptr %1723, , !!8                                                                                    ;L785
 28881|  %1725 = icmp ugt i64 %1692, %1724                                                                                     ;L785
 28882|  br i1 %1725, label %1709, label %1728                                                                                 ;L785
 28883| 
 28884| 1726: ; preds = %1758, %1712
 28885|     ;; self = i64 %1687
 28886|     ;; other = i64 %1690
 28887|  %1727 = call i64 @llvm.smax.i64(i64 %1690, i64 %1687)                                                                 ;L1039<782
 28888|  br label %1709                                                                                                        ;L1039<782
 28889| 
 28890| 1728: ; preds = %1722
 28891|  %1729 = sdiv i64 %1690, 3                                                                                             ;L786
 28892|     ;; self = i64 %1687
 28893|     ;; other = i64 %1729
 28894|  %1730 = call i64 @llvm.smax.i64(i64 %1729, i64 %1687)                                                                 ;L1039<786
 28895|  br label %1709                                                                                                        ;L1039<786
 28896| 
 28897| 1731: ; preds = %1716
 28898|  %1732 = gep %1718, i64 1664                                                                                           ;L1513<779
 28899|  %1733 = load i64, ptr %1732, , !!8                                                                                    ;L1513<779
 28900|  br label %1741                                                                                                        ;L1512<779
 28901| 
 28902| 1734: ; preds = %1716
 28903|  %1735 = sext i32 %1720 to i64                                                                                         ;L1511<779
 28904|     ;; mult = i64 %1735
 28905|  %1736 = gep %1718, i64 1664                                                                                           ;L1515<779
 28906|  %1737 = load i64, ptr %1736, , !!8                                                                                    ;L1515<779
 28907|  %1738 = add nsw i64 %1735, 100                                                                                        ;L1515<779
 28908|  %1739 = mul i64 %1737, %1738                                                                                          ;L1515<779
 28909|  %1740 = udiv i64 %1739, 100                                                                                           ;L1515<779
 28910|  br label %1741                                                                                                        ;L1512<779
 28911| 
 28912| 1741: ; preds = %1734, %1731
 28913|  %1742 = phi i64 [ %1733, %1731 ], [ %1740, %1734 ]                                                                    ;L0<779
 28914|  %1743 = add i64 %1742, 80000                                                                                          ;L779
 28915|  %1744 = load i32, ptr %342, , !!8                                                                                     ;L1511<779
 28916|     ;; mult = i32 %1744
 28917|  %1745 = icmp eq i32 %1744, 0                                                                                          ;L1512<779
 28918|  br i1 %1745, label %1746, label %1748                                                                                 ;L1512<779
 28919| 
 28920| 1746: ; preds = %1741
 28921|  %1747 = load i64, ptr %343, , !!8                                                                                     ;L1513<779
 28922|  br label %1754                                                                                                        ;L1512<779
 28923| 
 28924| 1748: ; preds = %1741
 28925|  %1749 = sext i32 %1744 to i64                                                                                         ;L1511<779
 28926|     ;; mult = i64 %1749
 28927|  %1750 = load i64, ptr %343, , !!8                                                                                     ;L1515<779
 28928|  %1751 = add nsw i64 %1749, 100                                                                                        ;L1515<779
 28929|  %1752 = mul i64 %1750, %1751                                                                                          ;L1515<779
 28930|  %1753 = udiv i64 %1752, 100                                                                                           ;L1515<779
 28931|  br label %1754                                                                                                        ;L1512<779
 28932| 
 28933| 1754: ; preds = %1748, %1746
 28934|  %1755 = phi i64 [ %1747, %1746 ], [ %1753, %1748 ]                                                                    ;L0<779
 28935|  %1756 = add i64 %1743, %1755                                                                                          ;L779
 28936|  %1757 = icmp ugt i64 %1694, %1756                                                                                     ;L779
 28937|  br i1 %1757, label %1758, label %1762                                                                                 ;L779
 28938| 
 28939| 1758: ; preds = %1754
 28940|  %1759 = gep %1686, i64 256                                                                                            ;L781
 28941|  %1760 = load i64, ptr %1759, , !!8                                                                                    ;L781
 28942|  %1761 = icmp ugt i64 %1692, %1760                                                                                     ;L781
 28943|  br i1 %1761, label %1764, label %1726                                                                                 ;L781
 28944| 
 28945| 1762: ; preds = %1754
 28946|     ;; self = i64 %1687
 28947|     ;; other = i64 %1690
 28948|  %1763 = call i64 @llvm.smax.i64(i64 %1690, i64 %1687)                                                                 ;L1039<780
 28949|  br label %1709                                                                                                        ;L1039<780
 28950| 
 28951| 1764: ; preds = %1758
 28952|  %1765 = sdiv i64 %1690, 2                                                                                             ;L784
 28953|     ;; self = i64 %1687
 28954|     ;; other = i64 %1765
 28955|  %1766 = call i64 @llvm.smax.i64(i64 %1765, i64 %1687)                                                                 ;L1039<784
 28956|  br label %1709                                                                                                        ;L1039<784
 28957| 
 28958| 1767: ; preds = %1588
 28959|  %1768 = gep %122, i64 104                                                                                             ;L1805<796
 28960|  %1769 = load i64, ptr %1768, , !!8                                                                                    ;L1805<796
 28961|  %1770 = icmp eq i64 %1769, 13                                                                                         ;L1805<796
 28962|  br i1 %1770, label %1771, label %1775                                                                                 ;L1805<796
 28963| 
 28964| 1771: ; preds = %1767
 28965|     ;; champ = ptr %122
 28966|  %1772 = gep %122, i64 200                                                                                             ;L1806<796
 28967|  %1773 = load i64, ptr %1772, , !!8                                                                                    ;L1806<796
 28968|  %1774 = icmp ult i64 %1773, 181                                                                                       ;L796
 28969|  br i1 %1774, label %1775, label %1781                                                                                 ;L796
 28970| 
 28971| 1775: ; preds = %1771, %1767
 28972|     ;; self = ptr %95
 28973|     ;; self = ptr %95
 28974|  %1776 = load ptr, ptr %95, , !!8, !!8                                                                                 ;L138<2073<797
 28975|     ;; p = ptr %1776
 28976|  %1777 = load i64, ptr %200, , !!8                                                                                     ;L2075<797
 28977|     ;; len = i64 %1777
 28978|     ;; count = i64 %1777
 28979|     ;; self[0..+8] = ptr %1776
 28980|     ;; slice[0..+8] = ptr %1776
 28981|     ;; self[8..+8] = i64 %1777
 28982|     ;; slice[8..+8] = i64 %1777
 28983|     ;; ptr = ptr %1776
 28984|     ;; self = ptr %1776
 28985|  %1778 = mul nuw nsw i64 %1777, 448                                                                                    ;L961<100<1042<797
 28986|  %1779 = gep %1776, i64 %1778                                                                                          ;L961<100<1042<797
 28987|     ;; iter[0..+8] = ptr %1776
 28988|     ;; iter[8..+8] = ptr %1779
 28989|     ;; max_ratio = i64 %1589
 28990|     ;; self = ptr undef
 28991|     ;; ptr = ptr %1776
 28992|     ;; self = ptr %1776
 28993|     ;; end_or_len = ptr %1779
 28996|  %1780 = icmp eq i64 %1777, 0                                                                                          ;L1714<180<797
 28997|  br i1 %1780, label %1823, label %1810                                                                                 ;L180<797
 28998| 
 28999| 1781: ; preds = %1923, %1823, %1771, %1588
 29000|  %1782 = phi i64 [ %1589, %1771 ], [ %1589, %1588 ], [ %1824, %1823 ], [ %1924, %1923 ]                                ;L693
 29001|     ;; max_ratio = i64 %1782
 29002|  %1783 = add i64 %1782, %549                                                                                           ;L840
 29003|     ;; score[16..+8] = i64 %1783
 29004|     ;; score[16..+8] = i64 %1783
 29005|     ;; score[24..+8] = i64 %1782
 29006|     ;; score[24..+8] = i64 %1782
 29007|  %1784 = gep %91, i64 32                                                                                               ;L846
 29008|  %1785 = load i8, ptr %1784, , !!8                                                                                     ;L846
 29009|     ;; my_role = i8 %1785
 29010|  %1786 = and i8 %1785, 6                                                                                               ;L847
 29011|  %1787 = icmp eq i8 %1786, 2                                                                                           ;L847
 29014|  call void @llvm.memcpy.p0.p0.i64(ptr %76, ptr %80, i64 24, i1 false)                                                  ;L850
 29017|  %1788 = gep %76, i64 16                                                                                               ;L825<1004<850
 29018|  %1789 = load i32, ptr %1788, , !!8                                                                                    ;L825<1004<850
 29019|  %1790 = icmp eq i32 %1789, -1                                                                                         ;L825<1004<850
 29020|  br i1 %1790, label %1981, label %1791                                                                                 ;L825<1004<850
 29021| 
 29022| 1791: ; preds = %1781
 29026|     ;; self = ptr %76
 29027|     ;; order = i8 0
 29028|     ;; order = i8 0
 29029|     ;; val = i64 1
 29030|     ;; order = i8 0
 29031|     ;; val = i64 1
 29032|     ;; order = i8 0
 29033|  %1792 = load i64, ptr %76, , !!8                                                                                      ;L185<825<825<1004<850
 29034|  %1793 = icmp ult i64 %1792, 132                                                                                       ;L185<825<825<1004<850
 29035|  br i1 %1793, label %1796, label %1794                                                                                 ;L185<825<825<1004<850
 29036| 
 29037| 1794: ; preds = %1791
 29038|  invoke void @core::panicking18panic_bounds_check(i64 %1792, i64 132, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.208) #25
 29039|  to label %1795 unwind label %1060                                                                                     ;L185<825<825<1004<850
 29040| 
 29041| 1795: ; preds = %1794
 29042|  unreachable                                                                                                           ;L185<825<825<1004<850
 29043| 
 29044| 1796: ; preds = %1791
 29045|     ;; self = !DIArgList(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 %1792)
 29046|  %1797 = gep %76, i64 8                                                                                                ;L185<825<825<1004<850
 29047|  %1798 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %1797)
 29048|  to label %1799 unwind label %1060                                                                                     ;L185<825<825<1004<850
 29049| 
 29050| 1799: ; preds = %1796
 29051|  %1800 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %1792                               ;L185<825<825<1004<850
 29052|     ;; self = ptr %1800
 29053|  %1801 = extractvalue { i64, i32 } %1798, 0                                                                            ;L185<825<825<1004<850
 29054|  %1802 = extractvalue { i64, i32 } %1798, 1                                                                            ;L185<825<825<1004<850
 29056|  %1803 = mul i64 %1801, 1000000000                                                                                     ;L632<185<825<825<1004<850
 29057|  %1804 = icmp ult i32 %1802, 1000000000                                                                                ;L49<632<185<825<825<1004<850
 29058|  call void @llvm.assume(i1 %1804)                                                                                      ;L49<632<185<825<825<1004<850
 29059|  %1805 = zext nneg i32 %1802 to i64                                                                                    ;L632<185<825<825<1004<850
 29060|  %1806 = add i64 %1803, %1805                                                                                          ;L632<185<825<825<1004<850
 29061|     ;; val = i64 %1806
 29062|     ;; val = i64 %1806
 29063|     ;; dst = ptr %1800
 29064|  %1807 = atomicrmw add ptr %1800, i64 %1806 monotonic, , !!44649                                                       ;L3937<3162<185<825<825<1004<850
 29065|  %1808 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %1792                               ;L186<825<825<1004<850
 29066|     ;; self = ptr %1808
 29067|     ;; dst = ptr %1808
 29068|  %1809 = atomicrmw add ptr %1808, i64 1 monotonic, , !!44649                                                           ;L3937<3162<186<825<825<1004<850
 29069|  br label %1981                                                                                                        ;L825<1004<850
 29070| 
 29071| 1810: ; preds = %1841, %1775
 29072|  %1811 = phi ptr [ %1813, %1841 ], [ %1776, %1775 ]
 29073|  %1812 = phi i64 [ %1842, %1841 ], [ %1589, %1775 ]
 29074|     ;; max_ratio = i64 %1812
 29075|  %1813 = gep %1811, i64 448                                                                                            ;L656<185<797
 29076|     ;; iter[0..+8] = ptr %1813
 29077|     ;; cache = ptr %1811
 29078|     ;; e = ptr %1811
 29079|     ;; dist = ptr %1811
 29080|  %1814 = gep %1811, i64 56                                                                                             ;L798
 29081|  %1815 = load i64, ptr %1814, , !!8                                                                                    ;L798
 29082|     ;; ratio = i64 %1815
 29083|  %1816 = gep %1811, i64 440                                                                                            ;L799
 29084|  %1817 = load i64, ptr %1816, , !!8                                                                                    ;L799
 29085|     ;; dist = i64 %1817
 29086|  %1818 = gep %1811, i64 104                                                                                            ;L800
 29087|  %1819 = load i64, ptr %1818, , !!8                                                                                    ;L800
 29088|     ;; range = i64 %1819
 29089|  %1820 = gep %1811, i64 264                                                                                            ;L0
 29090|  %1821 = load i64, ptr %1820, , !!8                                                                                    ;L0
 29091|  %1822 = icmp ugt i64 %1817, %1821                                                                                     ;L0
 29092|  br i1 %292, label %1831, label %1830                                                                                  ;L801
 29093| 
 29094| 1823: ; preds = %1841, %1775
 29095|  %1824 = phi i64 [ %1589, %1775 ], [ %1842, %1841 ]                                                                    ;L0
 29096|     ;; self = ptr %93
 29097|     ;; self = ptr %93
 29098|  %1825 = load ptr, ptr %93, , !!8, !!8                                                                                 ;L138<2073<818
 29099|     ;; p = ptr %1825
 29100|  %1826 = load i64, ptr %230, , !!8                                                                                     ;L2075<818
 29101|     ;; len = i64 %1826
 29102|     ;; count = i64 %1826
 29103|     ;; self[0..+8] = ptr %1825
 29104|     ;; slice[0..+8] = ptr %1825
 29105|     ;; self[8..+8] = i64 %1826
 29106|     ;; slice[8..+8] = i64 %1826
 29107|     ;; ptr = ptr %1825
 29108|     ;; self = ptr %1825
 29109|  %1827 = mul nuw nsw i64 %1826, 448                                                                                    ;L961<100<1042<818
 29110|  %1828 = gep %1825, i64 %1827                                                                                          ;L961<100<1042<818
 29111|     ;; iter[0..+8] = ptr %1825
 29112|     ;; iter[8..+8] = ptr %1828
 29113|     ;; max_ratio = i64 %1824
 29114|     ;; self = ptr undef
 29115|     ;; ptr = ptr %1825
 29116|     ;; self = ptr %1825
 29117|     ;; end_or_len = ptr %1828
 29120|  %1829 = icmp eq i64 %1826, 0                                                                                          ;L1714<180<818
 29121|  br i1 %1829, label %1781, label %1899                                                                                 ;L180<818
 29122| 
 29123| 1830: ; preds = %1810
 29124|  br i1 %1822, label %1832, label %1836                                                                                 ;L811
 29125| 
 29126| 1831: ; preds = %1810
 29127|  br i1 %1822, label %1844, label %1848                                                                                 ;L802
 29128| 
 29129| 1832: ; preds = %1830
 29130|  %1833 = gep %1811, i64 272                                                                                            ;L813
 29131|  %1834 = load i64, ptr %1833, , !!8                                                                                    ;L813
 29132|  %1835 = icmp ugt i64 %1817, %1834                                                                                     ;L813
 29133|  br i1 %1835, label %1841, label %1838                                                                                 ;L813
 29134| 
 29135| 1836: ; preds = %1830
 29136|     ;; self = i64 %1812
 29137|     ;; other = i64 %1815
 29138|  %1837 = call i64 @llvm.smax.i64(i64 %1815, i64 %1812)                                                                 ;L1039<812
 29139|  br label %1841                                                                                                        ;L1039<812
 29140| 
 29141| 1838: ; preds = %1832
 29142|  %1839 = sdiv i64 %1815, 2                                                                                             ;L814
 29143|     ;; self = i64 %1812
 29144|     ;; other = i64 %1839
 29145|  %1840 = call i64 @llvm.smax.i64(i64 %1839, i64 %1812)                                                                 ;L1039<814
 29146|  br label %1841                                                                                                        ;L1039<814
 29147| 
 29148| 1841: ; preds = %1896, %1894, %1860, %1858, %1854, %1838, %1836, %1832
 29149|  %1842 = phi i64 [ %1859, %1858 ], [ %1837, %1836 ], [ %1840, %1838 ], [ %1812, %1832 ], [ %1898, %1896 ], [ %1862, %1860 ], [ %1812, %1854 ], [ %1895, %1894 ] ;L0
 29150|     ;; iter[0..+8] = ptr %1813
 29151|     ;; max_ratio = i64 %1842
 29152|     ;; self = ptr undef
 29153|     ;; ptr = ptr %1813
 29154|     ;; self = ptr %1813
 29155|     ;; end_or_len = ptr %1779
 29158|  %1843 = icmp eq ptr %1813, %1779                                                                                      ;L1714<180<797
 29159|  br i1 %1843, label %1823, label %1810                                                                                 ;L180<797
 29160| 
 29161| 1844: ; preds = %1831
 29162|  %1845 = gep %1811, i64 280                                                                                            ;L804
 29163|  %1846 = load i64, ptr %1845, , !!8                                                                                    ;L804
 29164|  %1847 = icmp ugt i64 %1817, %1846                                                                                     ;L804
 29165|  br i1 %1847, label %1854, label %1858                                                                                 ;L804
 29166| 
 29167| 1848: ; preds = %1831
 29168|  %1849 = gep %1811, i64 432                                                                                            ;L802
 29169|  %1850 = load ptr, ptr %1849, , !!8, !!8                                                                               ;L802
 29170|     ;; self = ptr %1850
 29171|  %1851 = gep %1850, i64 1136                                                                                           ;L1511<802
 29172|  %1852 = load i32, ptr %1851, , !!8                                                                                    ;L1511<802
 29173|     ;; mult = i32 %1852
 29174|  %1853 = icmp eq i32 %1852, 0                                                                                          ;L1512<802
 29175|  br i1 %1853, label %1863, label %1866                                                                                 ;L1512<802
 29176| 
 29177| 1854: ; preds = %1844
 29178|  %1855 = gep %1811, i64 272                                                                                            ;L808
 29179|  %1856 = load i64, ptr %1855, , !!8                                                                                    ;L808
 29180|  %1857 = icmp ugt i64 %1817, %1856                                                                                     ;L808
 29181|  br i1 %1857, label %1841, label %1860                                                                                 ;L808
 29182| 
 29183| 1858: ; preds = %1890, %1844
 29184|     ;; self = i64 %1812
 29185|     ;; other = i64 %1815
 29186|  %1859 = call i64 @llvm.smax.i64(i64 %1815, i64 %1812)                                                                 ;L1039<805
 29187|  br label %1841                                                                                                        ;L1039<805
 29188| 
 29189| 1860: ; preds = %1854
 29190|  %1861 = sdiv i64 %1815, 3                                                                                             ;L809
 29191|     ;; self = i64 %1812
 29192|     ;; other = i64 %1861
 29193|  %1862 = call i64 @llvm.smax.i64(i64 %1861, i64 %1812)                                                                 ;L1039<809
 29194|  br label %1841                                                                                                        ;L1039<809
 29195| 
 29196| 1863: ; preds = %1848
 29197|  %1864 = gep %1850, i64 1664                                                                                           ;L1513<802
 29198|  %1865 = load i64, ptr %1864, , !!8                                                                                    ;L1513<802
 29199|  br label %1873                                                                                                        ;L1512<802
 29200| 
 29201| 1866: ; preds = %1848
 29202|  %1867 = sext i32 %1852 to i64                                                                                         ;L1511<802
 29203|     ;; mult = i64 %1867
 29204|  %1868 = gep %1850, i64 1664                                                                                           ;L1515<802
 29205|  %1869 = load i64, ptr %1868, , !!8                                                                                    ;L1515<802
 29206|  %1870 = add nsw i64 %1867, 100                                                                                        ;L1515<802
 29207|  %1871 = mul i64 %1869, %1870                                                                                          ;L1515<802
 29208|  %1872 = udiv i64 %1871, 100                                                                                           ;L1515<802
 29209|  br label %1873                                                                                                        ;L1512<802
 29210| 
 29211| 1873: ; preds = %1866, %1863
 29212|  %1874 = phi i64 [ %1865, %1863 ], [ %1872, %1866 ]                                                                    ;L0<802
 29213|  %1875 = add i64 %1874, 80000                                                                                          ;L802
 29214|  %1876 = load i32, ptr %342, , !!8                                                                                     ;L1511<802
 29215|     ;; mult = i32 %1876
 29216|  %1877 = icmp eq i32 %1876, 0                                                                                          ;L1512<802
 29217|  br i1 %1877, label %1878, label %1880                                                                                 ;L1512<802
 29218| 
 29219| 1878: ; preds = %1873
 29220|  %1879 = load i64, ptr %343, , !!8                                                                                     ;L1513<802
 29221|  br label %1886                                                                                                        ;L1512<802
 29222| 
 29223| 1880: ; preds = %1873
 29224|  %1881 = sext i32 %1876 to i64                                                                                         ;L1511<802
 29225|     ;; mult = i64 %1881
 29226|  %1882 = load i64, ptr %343, , !!8                                                                                     ;L1515<802
 29227|  %1883 = add nsw i64 %1881, 100                                                                                        ;L1515<802
 29228|  %1884 = mul i64 %1882, %1883                                                                                          ;L1515<802
 29229|  %1885 = udiv i64 %1884, 100                                                                                           ;L1515<802
 29230|  br label %1886                                                                                                        ;L1512<802
 29231| 
 29232| 1886: ; preds = %1880, %1878
 29233|  %1887 = phi i64 [ %1879, %1878 ], [ %1885, %1880 ]                                                                    ;L0<802
 29234|  %1888 = add i64 %1875, %1887                                                                                          ;L802
 29235|  %1889 = icmp ugt i64 %1819, %1888                                                                                     ;L802
 29236|  br i1 %1889, label %1890, label %1894                                                                                 ;L802
 29237| 
 29238| 1890: ; preds = %1886
 29239|  %1891 = gep %1811, i64 280                                                                                            ;L804
 29240|  %1892 = load i64, ptr %1891, , !!8                                                                                    ;L804
 29241|  %1893 = icmp ugt i64 %1817, %1892                                                                                     ;L804
 29242|  br i1 %1893, label %1896, label %1858                                                                                 ;L804
 29243| 
 29244| 1894: ; preds = %1886
 29245|     ;; self = i64 %1812
 29246|     ;; other = i64 %1815
 29247|  %1895 = call i64 @llvm.smax.i64(i64 %1815, i64 %1812)                                                                 ;L1039<803
 29248|  br label %1841                                                                                                        ;L1039<803
 29249| 
 29250| 1896: ; preds = %1890
 29251|  %1897 = sdiv i64 %1815, 2                                                                                             ;L807
 29252|     ;; self = i64 %1812
 29253|     ;; other = i64 %1897
 29254|  %1898 = call i64 @llvm.smax.i64(i64 %1897, i64 %1812)                                                                 ;L1039<807
 29255|  br label %1841                                                                                                        ;L1039<807
 29256| 
 29257| 1899: ; preds = %1923, %1823
 29258|  %1900 = phi ptr [ %1902, %1923 ], [ %1825, %1823 ]
 29259|  %1901 = phi i64 [ %1924, %1923 ], [ %1824, %1823 ]
 29260|     ;; max_ratio = i64 %1901
 29261|  %1902 = gep %1900, i64 448                                                                                            ;L656<185<818
 29262|     ;; iter[0..+8] = ptr %1902
 29263|     ;; cache = ptr %1900
 29264|     ;; a = ptr %1900
 29265|     ;; dist = ptr %1900
 29266|  %1903 = gep %1900, i64 56                                                                                             ;L819
 29267|  %1904 = load i64, ptr %1903, , !!8                                                                                    ;L819
 29268|     ;; ratio = i64 %1904
 29269|  %1905 = gep %1900, i64 440                                                                                            ;L820
 29270|  %1906 = load i64, ptr %1905, , !!8                                                                                    ;L820
 29271|     ;; dist = i64 %1906
 29272|  %1907 = gep %1900, i64 104                                                                                            ;L821
 29273|  %1908 = load i64, ptr %1907, , !!8                                                                                    ;L821
 29274|     ;; range = i64 %1908
 29275|  %1909 = gep %1900, i64 264                                                                                            ;L0
 29276|  %1910 = load i64, ptr %1909, , !!8                                                                                    ;L0
 29277|  %1911 = icmp ugt i64 %1906, %1910                                                                                     ;L0
 29278|  br i1 %292, label %1913, label %1912                                                                                  ;L822
 29279| 
 29280| 1912: ; preds = %1899
 29281|  br i1 %1911, label %1914, label %1918                                                                                 ;L832
 29282| 
 29283| 1913: ; preds = %1899
 29284|  br i1 %1911, label %1926, label %1930                                                                                 ;L823
 29285| 
 29286| 1914: ; preds = %1912
 29287|  %1915 = gep %1900, i64 272                                                                                            ;L834
 29288|  %1916 = load i64, ptr %1915, , !!8                                                                                    ;L834
 29289|  %1917 = icmp ugt i64 %1906, %1916                                                                                     ;L834
 29290|  br i1 %1917, label %1923, label %1920                                                                                 ;L834
 29291| 
 29292| 1918: ; preds = %1912
 29293|     ;; self = i64 %1901
 29294|     ;; other = i64 %1904
 29295|  %1919 = call i64 @llvm.smax.i64(i64 %1904, i64 %1901)                                                                 ;L1039<833
 29296|  br label %1923                                                                                                        ;L1039<833
 29297| 
 29298| 1920: ; preds = %1914
 29299|  %1921 = sdiv i64 %1904, 2                                                                                             ;L835
 29300|     ;; self = i64 %1901
 29301|     ;; other = i64 %1921
 29302|  %1922 = call i64 @llvm.smax.i64(i64 %1921, i64 %1901)                                                                 ;L1039<835
 29303|  br label %1923                                                                                                        ;L1039<835
 29304| 
 29305| 1923: ; preds = %1978, %1976, %1942, %1940, %1936, %1920, %1918, %1914
 29306|  %1924 = phi i64 [ %1941, %1940 ], [ %1919, %1918 ], [ %1922, %1920 ], [ %1901, %1914 ], [ %1980, %1978 ], [ %1944, %1942 ], [ %1901, %1936 ], [ %1977, %1976 ] ;L0
 29307|     ;; iter[0..+8] = ptr %1902
 29308|     ;; max_ratio = i64 %1924
 29309|     ;; self = ptr undef
 29310|     ;; ptr = ptr %1902
 29311|     ;; self = ptr %1902
 29312|     ;; end_or_len = ptr %1828
 29315|  %1925 = icmp eq ptr %1902, %1828                                                                                      ;L1714<180<818
 29316|  br i1 %1925, label %1781, label %1899                                                                                 ;L180<818
 29317| 
 29318| 1926: ; preds = %1913
 29319|  %1927 = gep %1900, i64 280                                                                                            ;L825
 29320|  %1928 = load i64, ptr %1927, , !!8                                                                                    ;L825
 29321|  %1929 = icmp ugt i64 %1906, %1928                                                                                     ;L825
 29322|  br i1 %1929, label %1936, label %1940                                                                                 ;L825
 29323| 
 29324| 1930: ; preds = %1913
 29325|  %1931 = gep %1900, i64 432                                                                                            ;L823
 29326|  %1932 = load ptr, ptr %1931, , !!8, !!8                                                                               ;L823
 29327|     ;; self = ptr %1932
 29328|  %1933 = gep %1932, i64 1136                                                                                           ;L1511<823
 29329|  %1934 = load i32, ptr %1933, , !!8                                                                                    ;L1511<823
 29330|     ;; mult = i32 %1934
 29331|  %1935 = icmp eq i32 %1934, 0                                                                                          ;L1512<823
 29332|  br i1 %1935, label %1945, label %1948                                                                                 ;L1512<823
 29333| 
 29334| 1936: ; preds = %1926
 29335|  %1937 = gep %1900, i64 272                                                                                            ;L829
 29336|  %1938 = load i64, ptr %1937, , !!8                                                                                    ;L829
 29337|  %1939 = icmp ugt i64 %1906, %1938                                                                                     ;L829
 29338|  br i1 %1939, label %1923, label %1942                                                                                 ;L829
 29339| 
 29340| 1940: ; preds = %1972, %1926
 29341|     ;; self = i64 %1901
 29342|     ;; other = i64 %1904
 29343|  %1941 = call i64 @llvm.smax.i64(i64 %1904, i64 %1901)                                                                 ;L1039<826
 29344|  br label %1923                                                                                                        ;L1039<826
 29345| 
 29346| 1942: ; preds = %1936
 29347|  %1943 = sdiv i64 %1904, 3                                                                                             ;L830
 29348|     ;; self = i64 %1901
 29349|     ;; other = i64 %1943
 29350|  %1944 = call i64 @llvm.smax.i64(i64 %1943, i64 %1901)                                                                 ;L1039<830
 29351|  br label %1923                                                                                                        ;L1039<830
 29352| 
 29353| 1945: ; preds = %1930
 29354|  %1946 = gep %1932, i64 1664                                                                                           ;L1513<823
 29355|  %1947 = load i64, ptr %1946, , !!8                                                                                    ;L1513<823
 29356|  br label %1955                                                                                                        ;L1512<823
 29357| 
 29358| 1948: ; preds = %1930
 29359|  %1949 = sext i32 %1934 to i64                                                                                         ;L1511<823
 29360|     ;; mult = i64 %1949
 29361|  %1950 = gep %1932, i64 1664                                                                                           ;L1515<823
 29362|  %1951 = load i64, ptr %1950, , !!8                                                                                    ;L1515<823
 29363|  %1952 = add nsw i64 %1949, 100                                                                                        ;L1515<823
 29364|  %1953 = mul i64 %1951, %1952                                                                                          ;L1515<823
 29365|  %1954 = udiv i64 %1953, 100                                                                                           ;L1515<823
 29366|  br label %1955                                                                                                        ;L1512<823
 29367| 
 29368| 1955: ; preds = %1948, %1945
 29369|  %1956 = phi i64 [ %1947, %1945 ], [ %1954, %1948 ]                                                                    ;L0<823
 29370|  %1957 = add i64 %1956, 80000                                                                                          ;L823
 29371|  %1958 = load i32, ptr %342, , !!8                                                                                     ;L1511<823
 29372|     ;; mult = i32 %1958
 29373|  %1959 = icmp eq i32 %1958, 0                                                                                          ;L1512<823
 29374|  br i1 %1959, label %1960, label %1962                                                                                 ;L1512<823
 29375| 
 29376| 1960: ; preds = %1955
 29377|  %1961 = load i64, ptr %343, , !!8                                                                                     ;L1513<823
 29378|  br label %1968                                                                                                        ;L1512<823
 29379| 
 29380| 1962: ; preds = %1955
 29381|  %1963 = sext i32 %1958 to i64                                                                                         ;L1511<823
 29382|     ;; mult = i64 %1963
 29383|  %1964 = load i64, ptr %343, , !!8                                                                                     ;L1515<823
 29384|  %1965 = add nsw i64 %1963, 100                                                                                        ;L1515<823
 29385|  %1966 = mul i64 %1964, %1965                                                                                          ;L1515<823
 29386|  %1967 = udiv i64 %1966, 100                                                                                           ;L1515<823
 29387|  br label %1968                                                                                                        ;L1512<823
 29388| 
 29389| 1968: ; preds = %1962, %1960
 29390|  %1969 = phi i64 [ %1961, %1960 ], [ %1967, %1962 ]                                                                    ;L0<823
 29391|  %1970 = add i64 %1957, %1969                                                                                          ;L823
 29392|  %1971 = icmp ugt i64 %1908, %1970                                                                                     ;L823
 29393|  br i1 %1971, label %1972, label %1976                                                                                 ;L823
 29394| 
 29395| 1972: ; preds = %1968
 29396|  %1973 = gep %1900, i64 280                                                                                            ;L825
 29397|  %1974 = load i64, ptr %1973, , !!8                                                                                    ;L825
 29398|  %1975 = icmp ugt i64 %1906, %1974                                                                                     ;L825
 29399|  br i1 %1975, label %1978, label %1940                                                                                 ;L825
 29400| 
 29401| 1976: ; preds = %1968
 29402|     ;; self = i64 %1901
 29403|     ;; other = i64 %1904
 29404|  %1977 = call i64 @llvm.smax.i64(i64 %1904, i64 %1901)                                                                 ;L1039<824
 29405|  br label %1923                                                                                                        ;L1039<824
 29406| 
 29407| 1978: ; preds = %1972
 29408|  %1979 = sdiv i64 %1904, 2                                                                                             ;L828
 29409|     ;; self = i64 %1901
 29410|     ;; other = i64 %1979
 29411|  %1980 = call i64 @llvm.smax.i64(i64 %1979, i64 %1901)                                                                 ;L1039<828
 29412|  br label %1923                                                                                                        ;L1039<828
 29413| 
 29414| 1981: ; preds = %1799, %1781
 29417|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 29418|     ;; order = i8 0
 29419|  %1982 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                   ;L3904<741<176<851
 29420|  %1983 = icmp eq i8 %1982, 0                                                                                           ;L176<851
 29421|  br i1 %1983, label %1986, label %1984                                                                                 ;L176<851
 29422| 
 29423| 1984: ; preds = %1981
 29424|  %1985 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()
 29425|  to label %2000 unwind label %1060                                                                                     ;L179<851
 29426| 
 29427| 1986: ; preds = %2000, %1981
 29428|  %1987 = phi i32 [ %2002, %2000 ], [ -1, %1981 ]
 29429|  %1988 = gep %75, i64 16                                                                                               ;L0<851
 29430|  store i32 %1987, ptr %1988,                                                                                           ;L0<851
 29431|     ;; champ_threat_risk = i64 0
 29432|     ;; self = ptr %95
 29433|     ;; self = ptr %95
 29434|  %1989 = load ptr, ptr %95, , !!8, !!8                                                                                 ;L138<2073<853
 29435|     ;; p = ptr %1989
 29436|  %1990 = load i64, ptr %200, , !!8                                                                                     ;L2075<853
 29437|     ;; len = i64 %1990
 29438|     ;; count = i64 %1990
 29439|     ;; self[0..+8] = ptr %1989
 29440|     ;; slice[0..+8] = ptr %1989
 29441|     ;; self[8..+8] = i64 %1990
 29442|     ;; slice[8..+8] = i64 %1990
 29443|     ;; ptr = ptr %1989
 29444|     ;; self = ptr %1989
 29445|  %1991 = gepS, ptr, i64 }, ptr %1989, i64 %1990                                                                        ;L961<100<1042<853
 29446|     ;; iter[0..+8] = ptr %1989
 29447|     ;; iter[8..+8] = ptr %1991
 29448|  %1992 = gep %153, i64 248
 29449|  %1993 = trunc nuw i8 %1263 to i1
 29450|  br label %1994                                                                                                        ;L853
 29451| 
 29452| 1994: ; preds = %2688, %1986
 29453|  %1995 = phi i8 [ %2620, %2688 ], [ %1261, %1986 ]
 29454|  %1996 = phi i64 [ %2689, %2688 ], [ 0, %1986 ]
 29455|  %1997 = phi i64 [ %2691, %2688 ], [ %1335, %1986 ]
 29456|  %1998 = phi ptr [ %2018, %2688 ], [ %1989, %1986 ]
 29457|  %1999 = phi i64 [ %2692, %2688 ], [ 0, %1986 ]
 29458|  br label %2004                                                                                                        ;L180<853
 29459| 
 29460| 2000: ; preds = %1984
 29461|  %2001 = extractvalue { i64, i32 } %1985, 0                                                                            ;L179<851
 29462|  %2002 = extractvalue { i64, i32 } %1985, 1                                                                            ;L179<851
 29463|  store i64 115, ptr %75,                                                                                               ;L179<851
 29464|  %2003 = gep %75, i64 8                                                                                                ;L179<851
 29465|  store i64 %2001, ptr %2003,                                                                                           ;L179<851
 29466|  br label %1986                                                                                                        ;L180<851
 29467| 
 29468| 2004: ; preds = %2017, %1994
 29469|  %2005 = phi ptr [ %2018, %2017 ], [ %1998, %1994 ]                                                                    ;L853
 29470|     ;; score[0..+8] = i64 %1997
 29471|     ;; score[0..+8] = i64 %1997
 29472|     ;; score[40..+8] = i64 %1996
 29473|     ;; score[40..+8] = i64 %1996
 29474|     ;; score[48..+1] = i8 %1995
 29475|     ;; score[48..+1] = i8 %1995
 29476|     ;; champ_threat_risk = i64 %1999
 29477|     ;; iter[0..+8] = ptr %2005
 29478|     ;; self = ptr undef
 29479|     ;; ptr = ptr %2005
 29480|     ;; self = ptr %2005
 29481|     ;; end_or_len = ptr %1991
 29484|  %2006 = icmp eq ptr %2005, %1991                                                                                      ;L1714<180<853
 29485|  br i1 %2006, label %2013, label %2007                                                                                 ;L180<853
 29486| 
 29487| 2007: ; preds = %2004
 29488|     ;; iter[0..+8] = ptr %2005
 29489|     ;; cache = ptr %2005
 29490|     ;; e = ptr %2005
 29491|     ;; dist = ptr %2005
 29492|     ;; attack_ratio = i64 0
 29493|     ;; skill_ratio = i64 0
 29494|     ;; skill2_ratio = i64 0
 29495|     ;; ult_ratio = i64 0
 29496|     ;; rush_ratio = i64 0
 29497|     ;; has_cc = i8 0
 29498|  %2008 = gep %2005, i64 440                                                                                            ;L860
 29499|  %2009 = load i64, ptr %2008, , !!8                                                                                    ;L860
 29500|     ;; dist = i64 %2009
 29501|  %2010 = load i64, ptr %231, , !!8                                                                                     ;L862
 29502|  %2011 = load ptr, ptr %1992, , !!8                                                                                    ;L862
 29503|  %2012 = invoke zeroext i1 %2011(ptr %151, i64 %150, i64 %2010)
 29504|  to label %2017 unwind label %2015                                                                                     ;L862
 29505| 
 29506| 2013: ; preds = %2004
 29507|  %2014 = icmp sgt i64 %1999, 0                                                                                         ;L1024
 29508|  br i1 %2014, label %2693, label %2708                                                                                 ;L1024
 29509| 
 29510| 2015: ; preds = %3313, %3312, %2763, %2718, %2716, %2656, %2590, %2573, %2569, %2545, %2512, %2507, %2497, %2448, %2424, %2397, %2388, %2378, %2321, %2308, %2299, %2271, %2214, %2201, %2192, %2164, %2107, %2096, %2088, %2059, %2054, %2046, %2039, %2030, %2007
 29511|  %2016 = cleanuppad within none []
 29512|  br i1 %2006, label %3314, label %3315                                                                                 ;L1181
 29513| 
 29514| 2017: ; preds = %2007
 29515|  %2018 = gep %2005, i64 448                                                                                            ;L656<185<853
 29516|     ;; iter[0..+8] = ptr %2018
 29517|  %2019 = icmp ult i64 %2009, 10000000000                                                                               ;L862
 29518|  %2020 = or i1 %2019, %2012                                                                                            ;L862
 29519|  br i1 %2020, label %2021, label %2004                                                                                 ;L862
 29520| 
 29521| 2021: ; preds = %2017
 29522|  %2022 = load i64, ptr %2005, , !!8                                                                                    ;L866
 29523|     ;; ratio = i64 %2022
 29524|  %2023 = gep %2005, i64 112                                                                                            ;L867
 29525|  %2024 = load i64, ptr %2023, , !!8                                                                                    ;L867
 29526|  %2025 = icmp ugt i64 %2009, %2024                                                                                     ;L867
 29527|  br i1 %2025, label %2026, label %2046                                                                                 ;L867
 29528| 
 29529| 2026: ; preds = %2021
 29530|  %2027 = gep %2005, i64 120                                                                                            ;L870
 29531|  %2028 = load i64, ptr %2027, , !!8                                                                                    ;L870
 29532|  %2029 = icmp ugt i64 %2009, %2028                                                                                     ;L870
 29533|  br i1 %2029, label %2039, label %2030                                                                                 ;L870
 29534| 
 29535| 2030: ; preds = %2026
 29536|     ;; self = i64 0
 29537|     ;; other = i64 %2022
 29539|  %2031 = gep %2005, i64 432                                                                                            ;L872
 29540|  %2032 = load ptr, ptr %2031, , !!8, !!8                                                                               ;L872
 29541|  %2033 = invoke { i64, i64 } @ai::fight_check19slot_cc_time_cached(i64 %1, ptr %3, ptr %2032, i8 0)
 29542|  to label %2034 unwind label %2015                                                                                     ;L872
 29543| 
 29544| 2034: ; preds = %2030
 29545|  %2035 = sdiv i64 %2022, 2                                                                                             ;L871
 29546|     ;; other = i64 %2035
 29547|  %2036 = call i64 @llvm.smax.i64(i64 %2035, i64 0)                                                                     ;L1039<871
 29548|     ;; attack_ratio = i64 %2036
 29549|  %2037 = extractvalue { i64, i64 } %2033, 0                                                                            ;L872
 29551|  %2038 = icmp eq i64 %2037, 1                                                                                          ;L430<872
 29552|     ;; has_cc = i1 %2038
 29553|  br label %2039                                                                                                        ;L870
 29554| 
 29555| 2039: ; preds = %2050, %2034, %2026
 29556|  %2040 = phi i64 [ %2051, %2050 ], [ %2036, %2034 ], [ 0, %2026 ]                                                      ;L0
 29557|  %2041 = phi i1 [ %2053, %2050 ], [ %2038, %2034 ], [ false, %2026 ]
 29558|  %2042 = zext i1 %2041 to i8                                                                                           ;L0
 29559|     ;; has_cc = i8 %2042
 29560|     ;; attack_ratio = i64 %2040
 29561|  %2043 = gep %2005, i64 432                                                                                            ;L874
 29562|  %2044 = load ptr, ptr %2043, , !!8, !!8                                                                               ;L874
 29563|     ;; self = ptr %2044
 29564|     ;; self = ptr %2044
 29565|     ;; self = ptr %2044
 29566|     ;; self = ptr %2044
 29567|     ;; self = ptr %2044
 29568|  %2045 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity15is_block_attack(ptr %2044)
 29569|  to label %2054 unwind label %2015                                                                                     ;L874
 29570| 
 29571| 2046: ; preds = %2021
 29572|     ;; self = i64 0
 29573|     ;; other = i64 %2022
 29575|  %2047 = gep %2005, i64 432                                                                                            ;L869
 29576|  %2048 = load ptr, ptr %2047, , !!8, !!8                                                                               ;L869
 29577|  %2049 = invoke { i64, i64 } @ai::fight_check19slot_cc_time_cached(i64 %1, ptr %3, ptr %2048, i8 0)
 29578|  to label %2050 unwind label %2015                                                                                     ;L869
 29579| 
 29580| 2050: ; preds = %2046
 29581|  %2051 = call i64 @llvm.smax.i64(i64 %2022, i64 0)                                                                     ;L1039<868
 29582|     ;; attack_ratio = i64 %2051
 29583|  %2052 = extractvalue { i64, i64 } %2049, 0                                                                            ;L869
 29585|  %2053 = icmp eq i64 %2052, 1                                                                                          ;L430<869
 29586|     ;; has_cc = i1 %2053
 29587|  br label %2039                                                                                                        ;L867
 29588| 
 29589| 2054: ; preds = %2039
 29590|  %2055 = udiv i64 %2040, 3                                                                                             ;L874
 29591|  %2056 = select i1 %2045, i64 %2055, i64 %2040                                                                         ;L874
 29592|     ;; attack_ratio = i64 %2056
 29593|  %2057 = invoke zeroext i1 @ai::fight_check17slot_ready_cached(ptr %3, ptr %2044, i8 1)
 29594|  to label %2058 unwind label %2015                                                                                     ;L878
 29595| 
 29596| 2058: ; preds = %2054
 29597|  br i1 %2057, label %2063, label %2059                                                                                 ;L878
 29598| 
 29599| 2059: ; preds = %2160, %2158, %2124, %2122, %2121, %2098, %2094, %2090, %2085, %2081, %2058
 29600|  %2060 = phi i64 [ %2123, %2122 ], [ %2095, %2094 ], [ %2087, %2085 ], [ %2095, %2098 ], [ 0, %2058 ], [ %2087, %2090 ], [ 0, %2081 ], [ %2162, %2160 ], [ %2126, %2124 ], [ 0, %2121 ], [ %2159, %2158 ] ;L0
 29601|  %2061 = phi i8 [ %2103, %2122 ], [ 1, %2094 ], [ 1, %2085 ], [ %2101, %2098 ], [ %2042, %2058 ], [ %2093, %2090 ], [ %2042, %2081 ], [ %2103, %2160 ], [ %2103, %2124 ], [ %2103, %2121 ], [ %2103, %2158 ] ;L0
 29602|     ;; has_cc = i8 %2061
 29603|     ;; skill_ratio = i64 %2060
 29604|  %2062 = invoke zeroext i1 @ai::fight_check17slot_ready_cached(ptr %3, ptr %2044, i8 2)
 29605|  to label %2163 unwind label %2015                                                                                     ;L903
 29606| 
 29607| 2063: ; preds = %2058
 29608|  %2064 = gep %2005, i64 8                                                                                              ;L879
 29609|  %2065 = load i64, ptr %2064, , !!8                                                                                    ;L879
 29610|     ;; ratio = i64 %2065
 29611|  %2066 = gep %2005, i64 64                                                                                             ;L880
 29612|  %2067 = load i64, ptr %2066, , !!8                                                                                    ;L880
 29613|     ;; range = i64 %2067
 29614|  %2068 = gep %2005, i64 416                                                                                            ;L881
 29615|  %2069 = load i8, ptr %2068, , !!8                                                                                     ;L881
 29616|  %2070 = trunc nuw i8 %2069 to i1                                                                                      ;L881
 29617|  br i1 %2070, label %2075, label %2071                                                                                 ;L881
 29618| 
 29619| 2071: ; preds = %2063
 29620|  %2072 = gep %2005, i64 128                                                                                            ;L894
 29621|  %2073 = load i64, ptr %2072, , !!8                                                                                    ;L894
 29622|  %2074 = icmp ugt i64 %2009, %2073                                                                                     ;L894
 29623|  br i1 %2074, label %2081, label %2094                                                                                 ;L894
 29624| 
 29625| 2075: ; preds = %2063
 29626|  %2076 = gep %2005, i64 136                                                                                            ;L882
 29627|  %2077 = load i64, ptr %2076, , !!8                                                                                    ;L882
 29628|  %2078 = icmp ugt i64 %2009, %2077                                                                                     ;L882
 29629|  %2079 = select i1 %2078, i1 true, i1 %2041                                                                            ;L882
 29630|  %2080 = select i1 %2078, i8 %2042, i8 1                                                                               ;L882
 29631|  br i1 %2079, label %2102, label %2107                                                                                 ;L882
 29632| 
 29633| 2081: ; preds = %2071
 29634|  %2082 = gep %2005, i64 136                                                                                            ;L897
 29635|  %2083 = load i64, ptr %2082, , !!8                                                                                    ;L897
 29636|  %2084 = icmp ugt i64 %2009, %2083                                                                                     ;L897
 29637|  br i1 %2084, label %2059, label %2085                                                                                 ;L897
 29638| 
 29639| 2085: ; preds = %2081
 29640|  %2086 = sdiv i64 %2065, 2                                                                                             ;L898
 29641|     ;; self = i64 0
 29642|     ;; other = i64 %2086
 29643|  %2087 = call i64 @llvm.smax.i64(i64 %2086, i64 0)                                                                     ;L1039<898
 29644|     ;; skill_ratio = i64 %2087
 29645|  br i1 %2041, label %2059, label %2088                                                                                 ;L899
 29646| 
 29647| 2088: ; preds = %2085
 29648|  %2089 = invoke { i64, i64 } @ai::fight_check19slot_cc_time_cached(i64 %1, ptr %3, ptr %2044, i8 1)
 29649|  to label %2090 unwind label %2015                                                                                     ;L899
 29650| 
 29651| 2090: ; preds = %2088
 29652|  %2091 = extractvalue { i64, i64 } %2089, 0                                                                            ;L899
 29654|  %2092 = icmp eq i64 %2091, 1                                                                                          ;L430<899
 29655|  %2093 = zext i1 %2092 to i8                                                                                           ;L430<899
 29656|     ;; has_cc = i8 %2093
 29657|  br label %2059                                                                                                        ;L899
 29658| 
 29659| 2094: ; preds = %2071
 29660|     ;; self = i64 0
 29661|     ;; other = i64 %2065
 29662|  %2095 = call i64 @llvm.smax.i64(i64 %2065, i64 0)                                                                     ;L1039<895
 29663|     ;; skill_ratio = i64 %2095
 29664|  br i1 %2041, label %2059, label %2096                                                                                 ;L896
 29665| 
 29666| 2096: ; preds = %2094
 29667|  %2097 = invoke { i64, i64 } @ai::fight_check19slot_cc_time_cached(i64 %1, ptr %3, ptr %2044, i8 1)
 29668|  to label %2098 unwind label %2015                                                                                     ;L896
 29669| 
 29670| 2098: ; preds = %2096
 29671|  %2099 = extractvalue { i64, i64 } %2097, 0                                                                            ;L896
 29673|  %2100 = icmp eq i64 %2099, 1                                                                                          ;L430<896
 29674|  %2101 = zext i1 %2100 to i8                                                                                           ;L430<896
 29675|     ;; has_cc = i8 %2101
 29676|  br label %2059                                                                                                        ;L896
 29677| 
 29678| 2102: ; preds = %2109, %2075
 29679|  %2103 = phi i8 [ %2080, %2075 ], [ %2112, %2109 ]                                                                     ;L0
 29680|     ;; has_cc = i8 %2103
 29681|  %2104 = gep %2005, i64 128                                                                                            ;L885
 29682|  %2105 = load i64, ptr %2104, , !!8                                                                                    ;L885
 29683|  %2106 = icmp ugt i64 %2009, %2105                                                                                     ;L885
 29684|  br i1 %2106, label %2113, label %2117                                                                                 ;L885
 29685| 
 29686| 2107: ; preds = %2075
 29687|  %2108 = invoke { i64, i64 } @ai::fight_check19slot_cc_time_cached(i64 %1, ptr %3, ptr %2044, i8 1)
 29688|  to label %2109 unwind label %2015                                                                                     ;L883
 29689| 
 29690| 2109: ; preds = %2107
 29691|  %2110 = extractvalue { i64, i64 } %2108, 0                                                                            ;L883
 29693|  %2111 = icmp eq i64 %2110, 1                                                                                          ;L430<883
 29694|  %2112 = zext i1 %2111 to i8                                                                                           ;L430<883
 29695|     ;; has_cc = i8 %2112
 29696|  br label %2102                                                                                                        ;L883
 29697| 
 29698| 2113: ; preds = %2102
 29699|  %2114 = gep %2005, i64 144                                                                                            ;L887
 29700|  %2115 = load i64, ptr %2114, , !!8                                                                                    ;L887
 29701|  %2116 = icmp ugt i64 %2009, %2115                                                                                     ;L887
 29702|  br i1 %2116, label %2121, label %2122                                                                                 ;L887
 29703| 
 29704| 2117: ; preds = %2102
 29705|  %2118 = gep %2044, i64 1136                                                                                           ;L1511<885
 29706|  %2119 = load i32, ptr %2118, , !!8                                                                                    ;L1511<885
 29707|     ;; mult = i32 %2119
 29708|  %2120 = icmp eq i32 %2119, 0                                                                                          ;L1512<885
 29709|  br i1 %2120, label %2127, label %2130                                                                                 ;L1512<885
 29710| 
 29711| 2121: ; preds = %2113
 29712|  br i1 %2078, label %2059, label %2124                                                                                 ;L891
 29713| 
 29714| 2122: ; preds = %2154, %2113
 29715|     ;; self = i64 0
 29716|     ;; other = i64 %2065
 29717|  %2123 = call i64 @llvm.smax.i64(i64 %2065, i64 0)                                                                     ;L1039<888
 29718|  br label %2059                                                                                                        ;L1039<888
 29719| 
 29720| 2124: ; preds = %2121
 29721|  %2125 = sdiv i64 %2065, 3                                                                                             ;L892
 29722|     ;; self = i64 0
 29723|     ;; other = i64 %2125
 29724|  %2126 = call i64 @llvm.smax.i64(i64 %2125, i64 0)                                                                     ;L1039<892
 29725|  br label %2059                                                                                                        ;L1039<892
 29726| 
 29727| 2127: ; preds = %2117
 29728|  %2128 = gep %2044, i64 1664                                                                                           ;L1513<885
 29729|  %2129 = load i64, ptr %2128, , !!8                                                                                    ;L1513<885
 29730|  br label %2137                                                                                                        ;L1512<885
 29731| 
 29732| 2130: ; preds = %2117
 29733|  %2131 = sext i32 %2119 to i64                                                                                         ;L1511<885
 29734|     ;; mult = i64 %2131
 29735|  %2132 = gep %2044, i64 1664                                                                                           ;L1515<885
 29736|  %2133 = load i64, ptr %2132, , !!8                                                                                    ;L1515<885
 29737|  %2134 = add nsw i64 %2131, 100                                                                                        ;L1515<885
 29738|  %2135 = mul i64 %2133, %2134                                                                                          ;L1515<885
 29739|  %2136 = udiv i64 %2135, 100                                                                                           ;L1515<885
 29740|  br label %2137                                                                                                        ;L1512<885
 29741| 
 29742| 2137: ; preds = %2130, %2127
 29743|  %2138 = phi i64 [ %2129, %2127 ], [ %2136, %2130 ]                                                                    ;L0<885
 29744|  %2139 = add i64 %2138, 80000                                                                                          ;L885
 29745|  %2140 = load i32, ptr %342, , !!8                                                                                     ;L1511<885
 29746|     ;; mult = i32 %2140
 29747|  %2141 = icmp eq i32 %2140, 0                                                                                          ;L1512<885
 29748|  br i1 %2141, label %2142, label %2144                                                                                 ;L1512<885
 29749| 
 29750| 2142: ; preds = %2137
 29751|  %2143 = load i64, ptr %343, , !!8                                                                                     ;L1513<885
 29752|  br label %2150                                                                                                        ;L1512<885
 29753| 
 29754| 2144: ; preds = %2137
 29755|  %2145 = sext i32 %2140 to i64                                                                                         ;L1511<885
 29756|     ;; mult = i64 %2145
 29757|  %2146 = load i64, ptr %343, , !!8                                                                                     ;L1515<885
 29758|  %2147 = add nsw i64 %2145, 100                                                                                        ;L1515<885
 29759|  %2148 = mul i64 %2146, %2147                                                                                          ;L1515<885
 29760|  %2149 = udiv i64 %2148, 100                                                                                           ;L1515<885
 29761|  br label %2150                                                                                                        ;L1512<885
 29762| 
 29763| 2150: ; preds = %2144, %2142
 29764|  %2151 = phi i64 [ %2143, %2142 ], [ %2149, %2144 ]                                                                    ;L0<885
 29765|  %2152 = add i64 %2139, %2151                                                                                          ;L885
 29766|  %2153 = icmp ugt i64 %2067, %2152                                                                                     ;L885
 29767|  br i1 %2153, label %2154, label %2158                                                                                 ;L885
 29768| 
 29769| 2154: ; preds = %2150
 29770|  %2155 = gep %2005, i64 144                                                                                            ;L887
 29771|  %2156 = load i64, ptr %2155, , !!8                                                                                    ;L887
 29772|  %2157 = icmp ugt i64 %2009, %2156                                                                                     ;L887
 29773|  br i1 %2157, label %2160, label %2122                                                                                 ;L887
 29774| 
 29775| 2158: ; preds = %2150
 29776|     ;; self = i64 0
 29777|     ;; other = i64 %2065
 29778|  %2159 = call i64 @llvm.smax.i64(i64 %2065, i64 0)                                                                     ;L1039<886
 29779|  br label %2059                                                                                                        ;L1039<886
 29780| 
 29781| 2160: ; preds = %2154
 29782|  %2161 = sdiv i64 %2065, 2                                                                                             ;L890
 29783|     ;; self = i64 0
 29784|     ;; other = i64 %2161
 29785|  %2162 = call i64 @llvm.smax.i64(i64 %2161, i64 0)                                                                     ;L1039<890
 29786|  br label %2059                                                                                                        ;L1039<890
 29787| 
 29788| 2163: ; preds = %2059
 29789|  br i1 %2062, label %2168, label %2164                                                                                 ;L903
 29790| 
 29791| 2164: ; preds = %2267, %2265, %2231, %2229, %2228, %2203, %2198, %2194, %2188, %2184, %2163
 29792|  %2165 = phi i64 [ %2230, %2229 ], [ %2199, %2198 ], [ %2190, %2188 ], [ %2199, %2203 ], [ 0, %2163 ], [ %2190, %2194 ], [ 0, %2184 ], [ %2269, %2267 ], [ %2233, %2231 ], [ 0, %2228 ], [ %2266, %2265 ] ;L0
 29793|  %2166 = phi i8 [ %2208, %2229 ], [ 1, %2198 ], [ 1, %2188 ], [ %2206, %2203 ], [ %2061, %2163 ], [ %2197, %2194 ], [ %2061, %2184 ], [ %2208, %2267 ], [ %2208, %2231 ], [ %2208, %2228 ], [ %2208, %2265 ] ;L0
 29794|     ;; has_cc = i8 %2166
 29795|     ;; skill2_ratio = i64 %2165
 29796|  %2167 = invoke zeroext i1 @ai::fight_check17slot_ready_cached(ptr %3, ptr %2044, i8 3)
 29797|  to label %2270 unwind label %2015                                                                                     ;L929
 29798| 
 29799| 2168: ; preds = %2163
 29800|  %2169 = gep %2005, i64 16                                                                                             ;L904
 29801|  %2170 = load i64, ptr %2169, , !!8                                                                                    ;L904
 29802|     ;; ratio = i64 %2170
 29803|  %2171 = gep %2005, i64 72                                                                                             ;L905
 29804|  %2172 = load i64, ptr %2171, , !!8                                                                                    ;L905
 29805|     ;; range = i64 %2172
 29806|  %2173 = gep %2005, i64 417                                                                                            ;L906
 29807|  %2174 = load i8, ptr %2173, , !!8                                                                                     ;L906
 29808|  %2175 = trunc nuw i8 %2174 to i1                                                                                      ;L906
 29809|  br i1 %2175, label %2180, label %2176                                                                                 ;L906
 29810| 
 29811| 2176: ; preds = %2168
 29812|  %2177 = gep %2005, i64 152                                                                                            ;L919
 29813|  %2178 = load i64, ptr %2177, , !!8                                                                                    ;L919
 29814|  %2179 = icmp ugt i64 %2009, %2178                                                                                     ;L919
 29815|  br i1 %2179, label %2184, label %2198                                                                                 ;L919
 29816| 
 29817| 2180: ; preds = %2168
 29818|  %2181 = gep %2005, i64 160                                                                                            ;L907
 29819|  %2182 = load i64, ptr %2181, , !!8                                                                                    ;L907
 29820|  %2183 = icmp ugt i64 %2009, %2182                                                                                     ;L907
 29821|  br i1 %2183, label %2207, label %2212                                                                                 ;L907
 29822| 
 29823| 2184: ; preds = %2176
 29824|  %2185 = gep %2005, i64 160                                                                                            ;L923
 29825|  %2186 = load i64, ptr %2185, , !!8                                                                                    ;L923
 29826|  %2187 = icmp ugt i64 %2009, %2186                                                                                     ;L923
 29827|  br i1 %2187, label %2164, label %2188                                                                                 ;L923
 29828| 
 29829| 2188: ; preds = %2184
 29830|  %2189 = sdiv i64 %2170, 2                                                                                             ;L924
 29831|     ;; self = i64 0
 29832|     ;; other = i64 %2189
 29833|  %2190 = call i64 @llvm.smax.i64(i64 %2189, i64 0)                                                                     ;L1039<924
 29834|     ;; skill2_ratio = i64 %2190
 29835|  %2191 = trunc nuw i8 %2061 to i1                                                                                      ;L925
 29836|  br i1 %2191, label %2164, label %2192                                                                                 ;L925
 29837| 
 29838| 2192: ; preds = %2188
 29839|  %2193 = invoke { i64, i64 } @ai::fight_check19slot_cc_time_cached(i64 %1, ptr %3, ptr %2044, i8 2)
 29840|  to label %2194 unwind label %2015                                                                                     ;L925
 29841| 
 29842| 2194: ; preds = %2192
 29843|  %2195 = extractvalue { i64, i64 } %2193, 0                                                                            ;L925
 29845|  %2196 = icmp eq i64 %2195, 1                                                                                          ;L430<925
 29846|  %2197 = zext i1 %2196 to i8                                                                                           ;L430<925
 29847|     ;; has_cc = i8 %2197
 29848|  br label %2164                                                                                                        ;L925
 29849| 
 29850| 2198: ; preds = %2176
 29851|     ;; self = i64 0
 29852|     ;; other = i64 %2170
 29853|  %2199 = call i64 @llvm.smax.i64(i64 %2170, i64 0)                                                                     ;L1039<920
 29854|     ;; skill2_ratio = i64 %2199
 29855|  %2200 = trunc nuw i8 %2061 to i1                                                                                      ;L922
 29856|  br i1 %2200, label %2164, label %2201                                                                                 ;L922
 29857| 
 29858| 2201: ; preds = %2198
 29859|  %2202 = invoke { i64, i64 } @ai::fight_check19slot_cc_time_cached(i64 %1, ptr %3, ptr %2044, i8 2)
 29860|  to label %2203 unwind label %2015                                                                                     ;L922
 29861| 
 29862| 2203: ; preds = %2201
 29863|  %2204 = extractvalue { i64, i64 } %2202, 0                                                                            ;L922
 29865|  %2205 = icmp eq i64 %2204, 1                                                                                          ;L430<922
 29866|  %2206 = zext i1 %2205 to i8                                                                                           ;L430<922
 29867|     ;; has_cc = i8 %2206
 29868|  br label %2164                                                                                                        ;L922
 29869| 
 29870| 2207: ; preds = %2216, %2212, %2180
 29871|  %2208 = phi i8 [ %2061, %2180 ], [ %2219, %2216 ], [ 1, %2212 ]                                                       ;L0
 29872|     ;; has_cc = i8 %2208
 29873|  %2209 = gep %2005, i64 152                                                                                            ;L910
 29874|  %2210 = load i64, ptr %2209, , !!8                                                                                    ;L910
 29875|  %2211 = icmp ugt i64 %2009, %2210                                                                                     ;L910
 29876|  br i1 %2211, label %2220, label %2224                                                                                 ;L910
 29877| 
 29878| 2212: ; preds = %2180
 29879|  %2213 = trunc nuw i8 %2061 to i1                                                                                      ;L908
 29880|  br i1 %2213, label %2207, label %2214                                                                                 ;L908
 29881| 
 29882| 2214: ; preds = %2212
 29883|  %2215 = invoke { i64, i64 } @ai::fight_check19slot_cc_time_cached(i64 %1, ptr %3, ptr %2044, i8 2)
 29884|  to label %2216 unwind label %2015                                                                                     ;L908
 29885| 
 29886| 2216: ; preds = %2214
 29887|  %2217 = extractvalue { i64, i64 } %2215, 0                                                                            ;L908
 29889|  %2218 = icmp eq i64 %2217, 1                                                                                          ;L430<908
 29890|  %2219 = zext i1 %2218 to i8                                                                                           ;L430<908
 29891|     ;; has_cc = i8 %2219
 29892|  br label %2207                                                                                                        ;L908
 29893| 
 29894| 2220: ; preds = %2207
 29895|  %2221 = gep %2005, i64 168                                                                                            ;L912
 29896|  %2222 = load i64, ptr %2221, , !!8                                                                                    ;L912
 29897|  %2223 = icmp ugt i64 %2009, %2222                                                                                     ;L912
 29898|  br i1 %2223, label %2228, label %2229                                                                                 ;L912
 29899| 
 29900| 2224: ; preds = %2207
 29901|  %2225 = gep %2044, i64 1136                                                                                           ;L1511<910
 29902|  %2226 = load i32, ptr %2225, , !!8                                                                                    ;L1511<910
 29903|     ;; mult = i32 %2226
 29904|  %2227 = icmp eq i32 %2226, 0                                                                                          ;L1512<910
 29905|  br i1 %2227, label %2234, label %2237                                                                                 ;L1512<910
 29906| 
 29907| 2228: ; preds = %2220
 29908|  br i1 %2183, label %2164, label %2231                                                                                 ;L916
 29909| 
 29910| 2229: ; preds = %2261, %2220
 29911|     ;; self = i64 0
 29912|     ;; other = i64 %2170
 29913|  %2230 = call i64 @llvm.smax.i64(i64 %2170, i64 0)                                                                     ;L1039<913
 29914|  br label %2164                                                                                                        ;L1039<913
 29915| 
 29916| 2231: ; preds = %2228
 29917|  %2232 = sdiv i64 %2170, 3                                                                                             ;L917
 29918|     ;; self = i64 0
 29919|     ;; other = i64 %2232
 29920|  %2233 = call i64 @llvm.smax.i64(i64 %2232, i64 0)                                                                     ;L1039<917
 29921|  br label %2164                                                                                                        ;L1039<917
 29922| 
 29923| 2234: ; preds = %2224
 29924|  %2235 = gep %2044, i64 1664                                                                                           ;L1513<910
 29925|  %2236 = load i64, ptr %2235, , !!8                                                                                    ;L1513<910
 29926|  br label %2244                                                                                                        ;L1512<910
 29927| 
 29928| 2237: ; preds = %2224
 29929|  %2238 = sext i32 %2226 to i64                                                                                         ;L1511<910
 29930|     ;; mult = i64 %2238
 29931|  %2239 = gep %2044, i64 1664                                                                                           ;L1515<910
 29932|  %2240 = load i64, ptr %2239, , !!8                                                                                    ;L1515<910
 29933|  %2241 = add nsw i64 %2238, 100                                                                                        ;L1515<910
 29934|  %2242 = mul i64 %2240, %2241                                                                                          ;L1515<910
 29935|  %2243 = udiv i64 %2242, 100                                                                                           ;L1515<910
 29936|  br label %2244                                                                                                        ;L1512<910
 29937| 
 29938| 2244: ; preds = %2237, %2234
 29939|  %2245 = phi i64 [ %2236, %2234 ], [ %2243, %2237 ]                                                                    ;L0<910
 29940|  %2246 = add i64 %2245, 80000                                                                                          ;L910
 29941|  %2247 = load i32, ptr %342, , !!8                                                                                     ;L1511<910
 29942|     ;; mult = i32 %2247
 29943|  %2248 = icmp eq i32 %2247, 0                                                                                          ;L1512<910
 29944|  br i1 %2248, label %2249, label %2251                                                                                 ;L1512<910
 29945| 
 29946| 2249: ; preds = %2244
 29947|  %2250 = load i64, ptr %343, , !!8                                                                                     ;L1513<910
 29948|  br label %2257                                                                                                        ;L1512<910
 29949| 
 29950| 2251: ; preds = %2244
 29951|  %2252 = sext i32 %2247 to i64                                                                                         ;L1511<910
 29952|     ;; mult = i64 %2252
 29953|  %2253 = load i64, ptr %343, , !!8                                                                                     ;L1515<910
 29954|  %2254 = add nsw i64 %2252, 100                                                                                        ;L1515<910
 29955|  %2255 = mul i64 %2253, %2254                                                                                          ;L1515<910
 29956|  %2256 = udiv i64 %2255, 100                                                                                           ;L1515<910
 29957|  br label %2257                                                                                                        ;L1512<910
 29958| 
 29959| 2257: ; preds = %2251, %2249
 29960|  %2258 = phi i64 [ %2250, %2249 ], [ %2256, %2251 ]                                                                    ;L0<910
 29961|  %2259 = add i64 %2246, %2258                                                                                          ;L910
 29962|  %2260 = icmp ugt i64 %2172, %2259                                                                                     ;L910
 29963|  br i1 %2260, label %2261, label %2265                                                                                 ;L910
 29964| 
 29965| 2261: ; preds = %2257
 29966|  %2262 = gep %2005, i64 168                                                                                            ;L912
 29967|  %2263 = load i64, ptr %2262, , !!8                                                                                    ;L912
 29968|  %2264 = icmp ugt i64 %2009, %2263                                                                                     ;L912
 29969|  br i1 %2264, label %2267, label %2229                                                                                 ;L912
 29970| 
 29971| 2265: ; preds = %2257
 29972|     ;; self = i64 0
 29973|     ;; other = i64 %2170
 29974|  %2266 = call i64 @llvm.smax.i64(i64 %2170, i64 0)                                                                     ;L1039<911
 29975|  br label %2164                                                                                                        ;L1039<911
 29976| 
 29977| 2267: ; preds = %2261
 29978|  %2268 = sdiv i64 %2170, 2                                                                                             ;L915
 29979|     ;; self = i64 0
 29980|     ;; other = i64 %2268
 29981|  %2269 = call i64 @llvm.smax.i64(i64 %2268, i64 0)                                                                     ;L1039<915
 29982|  br label %2164                                                                                                        ;L1039<915
 29983| 
 29984| 2270: ; preds = %2164
 29985|  br i1 %2167, label %2275, label %2271                                                                                 ;L929
 29986| 
 29987| 2271: ; preds = %2374, %2372, %2338, %2336, %2335, %2310, %2305, %2301, %2295, %2291, %2270
 29988|  %2272 = phi i64 [ %2337, %2336 ], [ %2306, %2305 ], [ %2297, %2295 ], [ %2306, %2310 ], [ 0, %2270 ], [ %2297, %2301 ], [ 0, %2291 ], [ %2376, %2374 ], [ %2340, %2338 ], [ 0, %2335 ], [ %2373, %2372 ] ;L0
 29989|  %2273 = phi i8 [ %2315, %2336 ], [ 1, %2305 ], [ 1, %2295 ], [ %2313, %2310 ], [ %2166, %2270 ], [ %2304, %2301 ], [ %2166, %2291 ], [ %2315, %2374 ], [ %2315, %2338 ], [ %2315, %2335 ], [ %2315, %2372 ] ;L0
 29990|     ;; has_cc = i8 %2273
 29991|     ;; ult_ratio = i64 %2272
 29992|  %2274 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity14is_block_skill(ptr %2044)
 29993|  to label %2377 unwind label %2015                                                                                     ;L955
 29994| 
 29995| 2275: ; preds = %2270
 29996|  %2276 = gep %2005, i64 24                                                                                             ;L930
 29997|  %2277 = load i64, ptr %2276, , !!8                                                                                    ;L930
 29998|     ;; ratio = i64 %2277
 29999|  %2278 = gep %2005, i64 80                                                                                             ;L931
 30000|  %2279 = load i64, ptr %2278, , !!8                                                                                    ;L931
 30001|     ;; range = i64 %2279
 30002|  %2280 = gep %2005, i64 418                                                                                            ;L932
 30003|  %2281 = load i8, ptr %2280, , !!8                                                                                     ;L932
 30004|  %2282 = trunc nuw i8 %2281 to i1                                                                                      ;L932
 30005|  br i1 %2282, label %2287, label %2283                                                                                 ;L932
 30006| 
 30007| 2283: ; preds = %2275
 30008|  %2284 = gep %2005, i64 176                                                                                            ;L945
 30009|  %2285 = load i64, ptr %2284, , !!8                                                                                    ;L945
 30010|  %2286 = icmp ugt i64 %2009, %2285                                                                                     ;L945
 30011|  br i1 %2286, label %2291, label %2305                                                                                 ;L945
 30012| 
 30013| 2287: ; preds = %2275
 30014|  %2288 = gep %2005, i64 184                                                                                            ;L933
 30015|  %2289 = load i64, ptr %2288, , !!8                                                                                    ;L933
 30016|  %2290 = icmp ugt i64 %2009, %2289                                                                                     ;L933
 30017|  br i1 %2290, label %2314, label %2319                                                                                 ;L933
 30018| 
 30019| 2291: ; preds = %2283
 30020|  %2292 = gep %2005, i64 184                                                                                            ;L949
 30021|  %2293 = load i64, ptr %2292, , !!8                                                                                    ;L949
 30022|  %2294 = icmp ugt i64 %2009, %2293                                                                                     ;L949
 30023|  br i1 %2294, label %2271, label %2295                                                                                 ;L949
 30024| 
 30025| 2295: ; preds = %2291
 30026|  %2296 = sdiv i64 %2277, 2                                                                                             ;L950
 30027|     ;; self = i64 0
 30028|     ;; other = i64 %2296
 30029|  %2297 = call i64 @llvm.smax.i64(i64 %2296, i64 0)                                                                     ;L1039<950
 30030|     ;; ult_ratio = i64 %2297
 30031|  %2298 = trunc nuw i8 %2166 to i1                                                                                      ;L951
 30032|  br i1 %2298, label %2271, label %2299                                                                                 ;L951
 30033| 
 30034| 2299: ; preds = %2295
 30035|  %2300 = invoke { i64, i64 } @ai::fight_check19slot_cc_time_cached(i64 %1, ptr %3, ptr %2044, i8 3)
 30036|  to label %2301 unwind label %2015                                                                                     ;L951
 30037| 
 30038| 2301: ; preds = %2299
 30039|  %2302 = extractvalue { i64, i64 } %2300, 0                                                                            ;L951
 30041|  %2303 = icmp eq i64 %2302, 1                                                                                          ;L430<951
 30042|  %2304 = zext i1 %2303 to i8                                                                                           ;L430<951
 30043|     ;; has_cc = i8 %2304
 30044|  br label %2271                                                                                                        ;L951
 30045| 
 30046| 2305: ; preds = %2283
 30047|     ;; self = i64 0
 30048|     ;; other = i64 %2277
 30049|  %2306 = call i64 @llvm.smax.i64(i64 %2277, i64 0)                                                                     ;L1039<946
 30050|     ;; ult_ratio = i64 %2306
 30051|  %2307 = trunc nuw i8 %2166 to i1                                                                                      ;L948
 30052|  br i1 %2307, label %2271, label %2308                                                                                 ;L948
 30053| 
 30054| 2308: ; preds = %2305
 30055|  %2309 = invoke { i64, i64 } @ai::fight_check19slot_cc_time_cached(i64 %1, ptr %3, ptr %2044, i8 3)
 30056|  to label %2310 unwind label %2015                                                                                     ;L948
 30057| 
 30058| 2310: ; preds = %2308
 30059|  %2311 = extractvalue { i64, i64 } %2309, 0                                                                            ;L948
 30061|  %2312 = icmp eq i64 %2311, 1                                                                                          ;L430<948
 30062|  %2313 = zext i1 %2312 to i8                                                                                           ;L430<948
 30063|     ;; has_cc = i8 %2313
 30064|  br label %2271                                                                                                        ;L948
 30065| 
 30066| 2314: ; preds = %2323, %2319, %2287
 30067|  %2315 = phi i8 [ %2166, %2287 ], [ %2326, %2323 ], [ 1, %2319 ]                                                       ;L0
 30068|     ;; has_cc = i8 %2315
 30069|  %2316 = gep %2005, i64 176                                                                                            ;L936
 30070|  %2317 = load i64, ptr %2316, , !!8                                                                                    ;L936
 30071|  %2318 = icmp ugt i64 %2009, %2317                                                                                     ;L936
 30072|  br i1 %2318, label %2327, label %2331                                                                                 ;L936
 30073| 
 30074| 2319: ; preds = %2287
 30075|  %2320 = trunc nuw i8 %2166 to i1                                                                                      ;L934
 30076|  br i1 %2320, label %2314, label %2321                                                                                 ;L934
 30077| 
 30078| 2321: ; preds = %2319
 30079|  %2322 = invoke { i64, i64 } @ai::fight_check19slot_cc_time_cached(i64 %1, ptr %3, ptr %2044, i8 3)
 30080|  to label %2323 unwind label %2015                                                                                     ;L934
 30081| 
 30082| 2323: ; preds = %2321
 30083|  %2324 = extractvalue { i64, i64 } %2322, 0                                                                            ;L934
 30085|  %2325 = icmp eq i64 %2324, 1                                                                                          ;L430<934
 30086|  %2326 = zext i1 %2325 to i8                                                                                           ;L430<934
 30087|     ;; has_cc = i8 %2326
 30088|  br label %2314                                                                                                        ;L934
 30089| 
 30090| 2327: ; preds = %2314
 30091|  %2328 = gep %2005, i64 192                                                                                            ;L938
 30092|  %2329 = load i64, ptr %2328, , !!8                                                                                    ;L938
 30093|  %2330 = icmp ugt i64 %2009, %2329                                                                                     ;L938
 30094|  br i1 %2330, label %2335, label %2336                                                                                 ;L938
 30095| 
 30096| 2331: ; preds = %2314
 30097|  %2332 = gep %2044, i64 1136                                                                                           ;L1511<936
 30098|  %2333 = load i32, ptr %2332, , !!8                                                                                    ;L1511<936
 30099|     ;; mult = i32 %2333
 30100|  %2334 = icmp eq i32 %2333, 0                                                                                          ;L1512<936
 30101|  br i1 %2334, label %2341, label %2344                                                                                 ;L1512<936
 30102| 
 30103| 2335: ; preds = %2327
 30104|  br i1 %2290, label %2271, label %2338                                                                                 ;L942
 30105| 
 30106| 2336: ; preds = %2368, %2327
 30107|     ;; self = i64 0
 30108|     ;; other = i64 %2277
 30109|  %2337 = call i64 @llvm.smax.i64(i64 %2277, i64 0)                                                                     ;L1039<939
 30110|  br label %2271                                                                                                        ;L1039<939
 30111| 
 30112| 2338: ; preds = %2335
 30113|  %2339 = sdiv i64 %2277, 3                                                                                             ;L943
 30114|     ;; self = i64 0
 30115|     ;; other = i64 %2339
 30116|  %2340 = call i64 @llvm.smax.i64(i64 %2339, i64 0)                                                                     ;L1039<943
 30117|  br label %2271                                                                                                        ;L1039<943
 30118| 
 30119| 2341: ; preds = %2331
 30120|  %2342 = gep %2044, i64 1664                                                                                           ;L1513<936
 30121|  %2343 = load i64, ptr %2342, , !!8                                                                                    ;L1513<936
 30122|  br label %2351                                                                                                        ;L1512<936
 30123| 
 30124| 2344: ; preds = %2331
 30125|  %2345 = sext i32 %2333 to i64                                                                                         ;L1511<936
 30126|     ;; mult = i64 %2345
 30127|  %2346 = gep %2044, i64 1664                                                                                           ;L1515<936
 30128|  %2347 = load i64, ptr %2346, , !!8                                                                                    ;L1515<936
 30129|  %2348 = add nsw i64 %2345, 100                                                                                        ;L1515<936
 30130|  %2349 = mul i64 %2347, %2348                                                                                          ;L1515<936
 30131|  %2350 = udiv i64 %2349, 100                                                                                           ;L1515<936
 30132|  br label %2351                                                                                                        ;L1512<936
 30133| 
 30134| 2351: ; preds = %2344, %2341
 30135|  %2352 = phi i64 [ %2343, %2341 ], [ %2350, %2344 ]                                                                    ;L0<936
 30136|  %2353 = add i64 %2352, 80000                                                                                          ;L936
 30137|  %2354 = load i32, ptr %342, , !!8                                                                                     ;L1511<936
 30138|     ;; mult = i32 %2354
 30139|  %2355 = icmp eq i32 %2354, 0                                                                                          ;L1512<936
 30140|  br i1 %2355, label %2356, label %2358                                                                                 ;L1512<936
 30141| 
 30142| 2356: ; preds = %2351
 30143|  %2357 = load i64, ptr %343, , !!8                                                                                     ;L1513<936
 30144|  br label %2364                                                                                                        ;L1512<936
 30145| 
 30146| 2358: ; preds = %2351
 30147|  %2359 = sext i32 %2354 to i64                                                                                         ;L1511<936
 30148|     ;; mult = i64 %2359
 30149|  %2360 = load i64, ptr %343, , !!8                                                                                     ;L1515<936
 30150|  %2361 = add nsw i64 %2359, 100                                                                                        ;L1515<936
 30151|  %2362 = mul i64 %2360, %2361                                                                                          ;L1515<936
 30152|  %2363 = udiv i64 %2362, 100                                                                                           ;L1515<936
 30153|  br label %2364                                                                                                        ;L1512<936
 30154| 
 30155| 2364: ; preds = %2358, %2356
 30156|  %2365 = phi i64 [ %2357, %2356 ], [ %2363, %2358 ]                                                                    ;L0<936
 30157|  %2366 = add i64 %2353, %2365                                                                                          ;L936
 30158|  %2367 = icmp ugt i64 %2279, %2366                                                                                     ;L936
 30159|  br i1 %2367, label %2368, label %2372                                                                                 ;L936
 30160| 
 30161| 2368: ; preds = %2364
 30162|  %2369 = gep %2005, i64 192                                                                                            ;L938
 30163|  %2370 = load i64, ptr %2369, , !!8                                                                                    ;L938
 30164|  %2371 = icmp ugt i64 %2009, %2370                                                                                     ;L938
 30165|  br i1 %2371, label %2374, label %2336                                                                                 ;L938
 30166| 
 30167| 2372: ; preds = %2364
 30168|     ;; self = i64 0
 30169|     ;; other = i64 %2277
 30170|  %2373 = call i64 @llvm.smax.i64(i64 %2277, i64 0)                                                                     ;L1039<937
 30171|  br label %2271                                                                                                        ;L1039<937
 30172| 
 30173| 2374: ; preds = %2368
 30174|  %2375 = sdiv i64 %2277, 2                                                                                             ;L941
 30175|     ;; self = i64 0
 30176|     ;; other = i64 %2375
 30177|  %2376 = call i64 @llvm.smax.i64(i64 %2375, i64 0)                                                                     ;L1039<941
 30178|  br label %2271                                                                                                        ;L1039<941
 30179| 
 30180| 2377: ; preds = %2271
 30181|  br i1 %2274, label %2383, label %2378                                                                                 ;L955
 30182| 
 30183| 2378: ; preds = %2383, %2377
 30184|  %2379 = phi i64 [ %2385, %2383 ], [ %2165, %2377 ]                                                                    ;L0
 30185|  %2380 = phi i64 [ %2384, %2383 ], [ %2060, %2377 ]                                                                    ;L0
 30186|  %2381 = phi i64 [ %2386, %2383 ], [ %2272, %2377 ]                                                                    ;L0
 30187|     ;; ult_ratio = i64 %2381
 30188|     ;; skill_ratio = i64 %2380
 30189|     ;; skill2_ratio = i64 %2379
 30190|  %2382 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity19is_block_move_skill(ptr %2044)
 30191|  to label %2387 unwind label %2015                                                                                     ;L960
 30192| 
 30193| 2383: ; preds = %2377
 30194|  %2384 = udiv i64 %2060, 3                                                                                             ;L956
 30195|     ;; skill_ratio = i64 %2384
 30196|  %2385 = udiv i64 %2165, 3                                                                                             ;L957
 30197|     ;; skill2_ratio = i64 %2385
 30198|  %2386 = udiv i64 %2272, 3                                                                                             ;L958
 30199|     ;; ult_ratio = i64 %2386
 30200|  br label %2378                                                                                                        ;L955
 30201| 
 30202| 2387: ; preds = %2378
 30203|  br i1 %2382, label %2393, label %2388                                                                                 ;L960
 30204| 
 30205| 2388: ; preds = %2462, %2461, %2440, %2387
 30206|  %2389 = phi i64 [ %2441, %2462 ], [ %2379, %2387 ], [ %2441, %2461 ], [ %2441, %2440 ]                                ;L0
 30207|  %2390 = phi i64 [ %2415, %2462 ], [ %2380, %2387 ], [ %2415, %2461 ], [ %2415, %2440 ]                                ;L0
 30208|  %2391 = phi i64 [ %2463, %2462 ], [ %2381, %2387 ], [ %2381, %2461 ], [ %2381, %2440 ]                                ;L0
 30209|     ;; ult_ratio = i64 %2391
 30210|     ;; skill_ratio = i64 %2390
 30211|     ;; skill2_ratio = i64 %2389
 30212|  %2392 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity11block_input(ptr %2044)
 30213|  to label %2464 unwind label %2015                                                                                     ;L971
 30214| 
 30215| 2393: ; preds = %2387
 30216|     ;; self = ptr %2044
 30217|  %2394 = gep %2044, i64 1272                                                                                           ;L742<961
 30218|  %2395 = load i32, ptr %2394, , !!8                                                                                    ;L742<961
 30219|  %2396 = icmp eq i32 %2395, -1                                                                                         ;L742<961
 30222|     ;; default = i1 false
 30224|  br i1 %2396, label %2414, label %2397                                                                                 ;L1226<961
 30225| 
 30226| 2397: ; preds = %2393
 30227|  %2398 = gep %2044, i64 1224                                                                                           ;L742<961
 30229|  %2399 = load ptr, ptr %2398, , !!8, !!8                                                                               ;L1227<961
 30230|  %2400 = gep %2044, i64 1232                                                                                           ;L1227<961
 30231|  %2401 = load ptr, ptr %2400, , !!8, !!8                                                                               ;L1227<961
 30235|  %2402 = gep %2401, i64 16                                                                                             ;L2445<961<1227<961
 30236|  %2403 = load i64, ptr %2402, , !!44997                                                                                ;L2445<961<1227<961
 30237|  %2404 = add nsw i64 %2403, -1                                                                                         ;L2445<961<1227<961
 30238|  %2405 = and i64 %2404, -16                                                                                            ;L2445<961<1227<961
 30239|  %2406 = gep %2399, i64 %2405                                                                                          ;L2445<961<1227<961
 30240|  %2407 = gep %2406, i64 16                                                                                             ;L2445<961<1227<961
 30241|  %2408 = gep %2401, i64 288                                                                                            ;L961<1227<961
 30242|  %2409 = load ptr, ptr %2408, , !!44997, !!8                                                                           ;L961<1227<961
 30243|  %2410 = invoke zeroext i1 %2409(ptr %2407)
 30244|  to label %2411 unwind label %2015                                                                                     ;L961<1227<961
 30245| 
 30246| 2411: ; preds = %2397
 30247|  %2412 = udiv i64 %2380, 3                                                                                             ;L961
 30248|  %2413 = select i1 %2410, i64 %2412, i64 %2380                                                                         ;L961
 30249|  br label %2414                                                                                                        ;L961
 30250| 
 30251| 2414: ; preds = %2411, %2393
 30252|  %2415 = phi i64 [ %2380, %2393 ], [ %2413, %2411 ]                                                                    ;L961
 30253|     ;; skill_ratio = i64 %2415
 30254|  %2416 = gep %2044, i64 1480                                                                                           ;L1693<964
 30255|  %2417 = load i64, ptr %2416, , !!8                                                                                    ;L1693<964
 30256|  %2418 = icmp ugt i64 %2417, 2                                                                                         ;L1693<964
 30257|  %2419 = gep %2044, i64 1280                                                                                           ;L1693<964
 30258|  %2420 = select i1 %2418, ptr %2419, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                     ;L1693<964
 30259|     ;; self = ptr %2420
 30260|  %2421 = gep %2420, i64 48                                                                                             ;L742<964
 30261|  %2422 = load i32, ptr %2421, , !!8                                                                                    ;L742<964
 30262|  %2423 = icmp eq i32 %2422, -1                                                                                         ;L742<964
 30265|     ;; default = i1 false
 30267|  br i1 %2423, label %2440, label %2424                                                                                 ;L1226<964
 30268| 
 30269| 2424: ; preds = %2414
 30271|  %2425 = load ptr, ptr %2420, , !!8, !!8                                                                               ;L1227<964
 30272|  %2426 = gep %2420, i64 8                                                                                              ;L1227<964
 30273|  %2427 = load ptr, ptr %2426, , !!8, !!8                                                                               ;L1227<964
 30277|  %2428 = gep %2427, i64 16                                                                                             ;L2445<964<1227<964
 30278|  %2429 = load i64, ptr %2428, , !!45037                                                                                ;L2445<964<1227<964
 30279|  %2430 = add nsw i64 %2429, -1                                                                                         ;L2445<964<1227<964
 30280|  %2431 = and i64 %2430, -16                                                                                            ;L2445<964<1227<964
 30281|  %2432 = gep %2425, i64 %2431                                                                                          ;L2445<964<1227<964
 30282|  %2433 = gep %2432, i64 16                                                                                             ;L2445<964<1227<964
 30283|  %2434 = gep %2427, i64 288                                                                                            ;L964<1227<964
 30284|  %2435 = load ptr, ptr %2434, , !!45037, !!8                                                                           ;L964<1227<964
 30285|  %2436 = invoke zeroext i1 %2435(ptr %2433)
 30286|  to label %2437 unwind label %2015                                                                                     ;L964<1227<964
 30287| 
 30288| 2437: ; preds = %2424
 30289|  %2438 = udiv i64 %2379, 3                                                                                             ;L964
 30290|  %2439 = select i1 %2436, i64 %2438, i64 %2379                                                                         ;L964
 30291|  br label %2440                                                                                                        ;L964
 30292| 
 30293| 2440: ; preds = %2437, %2414
 30294|  %2441 = phi i64 [ %2379, %2414 ], [ %2439, %2437 ]                                                                    ;L964
 30295|     ;; skill2_ratio = i64 %2441
 30296|  %2442 = icmp ugt i64 %2417, 4                                                                                         ;L1701<967
 30297|  %2443 = gep %2044, i64 1336                                                                                           ;L1701<967
 30298|  %2444 = select i1 %2442, ptr %2443, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                     ;L1701<967
 30299|     ;; self = ptr %2444
 30300|  %2445 = gep %2444, i64 48                                                                                             ;L742<967
 30301|  %2446 = load i32, ptr %2445, , !!8                                                                                    ;L742<967
 30302|  %2447 = icmp eq i32 %2446, -1                                                                                         ;L742<967
 30305|     ;; default = i1 false
 30307|  br i1 %2447, label %2388, label %2448                                                                                 ;L1226<967
 30308| 
 30309| 2448: ; preds = %2440
 30311|  %2449 = load ptr, ptr %2444, , !!8, !!8                                                                               ;L1227<967
 30312|  %2450 = gep %2444, i64 8                                                                                              ;L1227<967
 30313|  %2451 = load ptr, ptr %2450, , !!8, !!8                                                                               ;L1227<967
 30317|  %2452 = gep %2451, i64 16                                                                                             ;L2445<967<1227<967
 30318|  %2453 = load i64, ptr %2452, , !!45077                                                                                ;L2445<967<1227<967
 30319|  %2454 = add nsw i64 %2453, -1                                                                                         ;L2445<967<1227<967
 30320|  %2455 = and i64 %2454, -16                                                                                            ;L2445<967<1227<967
 30321|  %2456 = gep %2449, i64 %2455                                                                                          ;L2445<967<1227<967
 30322|  %2457 = gep %2456, i64 16                                                                                             ;L2445<967<1227<967
 30323|  %2458 = gep %2451, i64 288                                                                                            ;L967<1227<967
 30324|  %2459 = load ptr, ptr %2458, , !!45077, !!8                                                                           ;L967<1227<967
 30325|  %2460 = invoke zeroext i1 %2459(ptr %2457)
 30326|  to label %2461 unwind label %2015                                                                                     ;L967<1227<967
 30327| 
 30328| 2461: ; preds = %2448
 30329|  br i1 %2460, label %2462, label %2388                                                                                 ;L967
 30330| 
 30331| 2462: ; preds = %2461
 30332|  %2463 = udiv i64 %2381, 3                                                                                             ;L968
 30333|     ;; ult_ratio = i64 %2463
 30334|  br label %2388                                                                                                        ;L967
 30335| 
 30336| 2464: ; preds = %2388
 30337|  br i1 %2392, label %2475, label %2465                                                                                 ;L971
 30338| 
 30339| 2465: ; preds = %2475, %2464
 30340|  %2466 = phi i64 [ %2478, %2475 ], [ %2389, %2464 ]                                                                    ;L0
 30341|  %2467 = phi i64 [ %2477, %2475 ], [ %2390, %2464 ]                                                                    ;L0
 30342|  %2468 = phi i64 [ %2476, %2475 ], [ %2056, %2464 ]                                                                    ;L0
 30343|  %2469 = phi i64 [ %2479, %2475 ], [ %2391, %2464 ]                                                                    ;L0
 30344|     ;; ult_ratio = i64 %2469
 30345|     ;; attack_ratio = i64 %2468
 30346|     ;; skill_ratio = i64 %2467
 30347|     ;; skill2_ratio = i64 %2466
 30348|  %2470 = gep %2044, i64 776                                                                                            ;L979
 30349|  %2471 = load i64, ptr %2470, , !!8                                                                                    ;L979
 30350|  %2472 = xor i64 %2471, -9223372036854775808                                                                           ;L979
 30351|  %2473 = icmp slt i64 %2471, 0                                                                                         ;L979
 30352|  %2474 = select i1 %2473, i64 %2472, i64 4                                                                             ;L979
 30353|  switch i64 %2474, label %2480 [
 30354|  i64 3, label %2486
 30355|  i64 4, label %2492
 30356|  ]                                                                                                                     ;L978
 30357| 
 30358| 2475: ; preds = %2464
 30359|  %2476 = lshr i64 %2056, 1                                                                                             ;L972
 30360|     ;; attack_ratio = i64 %2476
 30361|  %2477 = lshr i64 %2390, 1                                                                                             ;L973
 30362|     ;; skill_ratio = i64 %2477
 30363|  %2478 = lshr i64 %2389, 1                                                                                             ;L974
 30364|     ;; skill2_ratio = i64 %2478
 30365|  %2479 = lshr i64 %2391, 1                                                                                             ;L975
 30366|     ;; ult_ratio = i64 %2479
 30367|  br label %2465                                                                                                        ;L971
 30368| 
 30369| 2480: ; preds = %2605, %2534, %2509, %2506, %2465
 30370|  %2481 = phi i8 [ %1995, %2465 ], [ %1995, %2509 ], [ %1995, %2534 ], [ 1, %2605 ], [ %1995, %2506 ]                   ;L0
 30371|  %2482 = phi i64 [ 0, %2465 ], [ 0, %2509 ], [ 0, %2534 ], [ %2581, %2605 ], [ 0, %2506 ]                              ;L0
 30372|  %2483 = phi i8 [ %2273, %2465 ], [ %2273, %2509 ], [ %2273, %2534 ], [ %2606, %2605 ], [ %2273, %2506 ]               ;L0
 30373|     ;; score[48..+1] = i8 %2481
 30374|     ;; score[48..+1] = i8 %2481
 30375|     ;; has_cc = i8 %2483
 30376|     ;; rush_ratio = i64 %2482
 30377|  %2484 = trunc nuw i8 %2483 to i1                                                                                      ;L995
 30378|  %2485 = select i1 %2484, i1 true, i1 %1993                                                                            ;L995
 30379|  br i1 %2485, label %2607, label %2614                                                                                 ;L995
 30380| 
 30381| 2486: ; preds = %2465
 30382|  %2487 = gep %2044, i64 860                                                                                            ;L978
 30383|     ;; casting_target = ptr %2487
 30384|  %2488 = gep %2044, i64 816                                                                                            ;L978
 30385|     ;; ex = ptr %2488
 30386|  %2489 = gep %2044, i64 824                                                                                            ;L978
 30387|     ;; ey = ptr %2489
 30388|  %2490 = gep %2044, i64 832                                                                                            ;L978
 30389|     ;; range = ptr %2490
 30390|  %2491 = gep %2044, i64 784                                                                                            ;L978
 30391|     ;; applyed_effect = ptr %2491
 30392|     ;; self = ptr %2491
 30393|     ;; self = ptr %2491
 30394|     ;; self = ptr %2491
 30395|     ;; start_tick = ptr %2504
 30396|  br label %2497                                                                                                        ;L978
 30397| 
 30398| 2492: ; preds = %2465
 30399|  %2493 = gep %2044, i64 876                                                                                            ;L979
 30400|     ;; casting_target = ptr %2493
 30401|  %2494 = gep %2044, i64 832                                                                                            ;L979
 30402|     ;; ex = ptr %2494
 30403|  %2495 = gep %2044, i64 840                                                                                            ;L979
 30404|     ;; ey = ptr %2495
 30405|  %2496 = gep %2044, i64 848                                                                                            ;L979
 30406|     ;; range = ptr %2496
 30407|     ;; applyed_effect = ptr %2470
 30408|     ;; self = ptr %2470
 30409|     ;; self = ptr %2470
 30410|     ;; self = ptr %2470
 30411|     ;; start_tick = ptr %2504
 30412|  br label %2497                                                                                                        ;L978
 30413| 
 30414| 2497: ; preds = %2492, %2486
 30415|  %2498 = phi i64 [ 856, %2492 ], [ 840, %2486 ]
 30416|  %2499 = phi ptr [ %2493, %2492 ], [ %2487, %2486 ]                                                                    ;L0
 30417|  %2500 = phi ptr [ %2494, %2492 ], [ %2488, %2486 ]                                                                    ;L0
 30418|  %2501 = phi ptr [ %2495, %2492 ], [ %2489, %2486 ]                                                                    ;L0
 30419|  %2502 = phi ptr [ %2496, %2492 ], [ %2490, %2486 ]                                                                    ;L0
 30420|  %2503 = phi ptr [ %2470, %2492 ], [ %2491, %2486 ]                                                                    ;L0
 30421|  %2504 = gep %2044, i64 %2498                                                                                          ;L0
 30422|     ;; start_tick = ptr %2504
 30423|     ;; self = ptr %2503
 30424|     ;; self = ptr %2503
 30425|     ;; self = ptr %2503
 30426|     ;; applyed_effect = ptr %2503
 30427|     ;; range = ptr %2502
 30428|     ;; ey = ptr %2501
 30429|     ;; ex = ptr %2500
 30430|     ;; casting_target = ptr %2499
 30431|  %2505 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %2499, ptr %2044, ptr %122)
 30432|  to label %2506 unwind label %2015                                                                                     ;L980
 30433| 
 30434| 2506: ; preds = %2497
 30435|  br i1 %2505, label %2507, label %2480                                                                                 ;L980
 30436| 
 30437| 2507: ; preds = %2506
 30438|  %2508 = invoke i64 %382(ptr %151)
 30439|  to label %2509 unwind label %2015                                                                                     ;L981
 30440| 
 30441| 2509: ; preds = %2507
 30442|  %2510 = load i64, ptr %2504, , !!8                                                                                    ;L981
 30443|  %2511 = icmp ult i64 %2508, %2510                                                                                     ;L981
 30444|  br i1 %2511, label %2480, label %2512                                                                                 ;L981
 30445| 
 30446| 2512: ; preds = %2509
 30447|  %2513 = load i64, ptr %100, , !!8                                                                                     ;L982
 30448|  %2514 = load i64, ptr %99, , !!8                                                                                      ;L982
 30449|  %2515 = gep %2044, i64 1632                                                                                           ;L982
 30450|  %2516 = load i64, ptr %2515, , !!8                                                                                    ;L982
 30451|  %2517 = gep %2044, i64 1640                                                                                           ;L982
 30452|  %2518 = load i64, ptr %2517, , !!8                                                                                    ;L982
 30453|  %2519 = load i64, ptr %2500, , !!8                                                                                    ;L982
 30454|  %2520 = load i64, ptr %2501, , !!8                                                                                    ;L982
 30455|  %2521 = invoke i64 @gc::utils20dist_to_line_segment(i64 %2513, i64 %2514, i64 %2516, i64 %2518, i64 %2519, i64 %2520)
 30456|  to label %2522 unwind label %2015                                                                                     ;L982
 30457| 
 30458| 2522: ; preds = %2512
 30459|  %2523 = load i64, ptr %2502, , !!8                                                                                    ;L983
 30460|  %2524 = load i32, ptr %342, , !!8                                                                                     ;L1511<983
 30461|     ;; mult = i32 %2524
 30462|  %2525 = icmp eq i32 %2524, 0                                                                                          ;L1512<983
 30463|  br i1 %2525, label %2526, label %2528                                                                                 ;L1512<983
 30464| 
 30465| 2526: ; preds = %2522
 30466|  %2527 = load i64, ptr %343, , !!8                                                                                     ;L1513<983
 30467|  br label %2534                                                                                                        ;L1512<983
 30468| 
 30469| 2528: ; preds = %2522
 30470|  %2529 = sext i32 %2524 to i64                                                                                         ;L1511<983
 30471|     ;; mult = i64 %2529
 30472|  %2530 = load i64, ptr %343, , !!8                                                                                     ;L1515<983
 30473|  %2531 = add nsw i64 %2529, 100                                                                                        ;L1515<983
 30474|  %2532 = mul i64 %2530, %2531                                                                                          ;L1515<983
 30475|  %2533 = udiv i64 %2532, 100                                                                                           ;L1515<983
 30476|  br label %2534                                                                                                        ;L1512<983
 30477| 
 30478| 2534: ; preds = %2528, %2526
 30479|  %2535 = phi i64 [ %2527, %2526 ], [ %2533, %2528 ]                                                                    ;L0<983
 30480|  %2536 = add i64 %2523, 20000                                                                                          ;L983
 30481|  %2537 = add i64 %2536, %2535                                                                                          ;L983
 30482|  %2538 = icmp ugt i64 %2521, %2537                                                                                     ;L982
 30483|  br i1 %2538, label %2480, label %2539                                                                                 ;L982
 30484| 
 30485| 2539: ; preds = %2534
 30486|     ;; self = ptr %2503
 30487|     ;; self = ptr %2503
 30488|     ;; self = ptr %2503
 30489|  %2540 = gep %2503, i64 8                                                                                              ;L614<609<296<1968<1864<3787<984
 30490|  %2541 = load ptr, ptr %2540, , !!8, !!8                                                                               ;L614<609<296<1968<1864<3787<984
 30491|  %2542 = gep %2503, i64 16                                                                                             ;L1864<3787<984
 30492|  %2543 = load i64, ptr %2542, , !!8                                                                                    ;L1864<3787<984
 30493|     ;; count = i64 %2543
 30494|     ;; self[0..+8] = ptr %2541
 30495|     ;; slice[0..+8] = ptr %2541
 30496|     ;; self[8..+8] = i64 %2543
 30497|     ;; slice[8..+8] = i64 %2543
 30498|     ;; self = ptr %2541
 30499|     ;; self[0..+8] = ptr %2541
 30500|     ;; self[8..+8] = !DIArgList(ptr %2541, i64 %2543)
 30501|     ;; self[16..+8] = ptr %135
 30502|     ;; self[24..+8] = ptr %2044
 30503|     ;; f[0..+8] = ptr %135
 30504|     ;; f[8..+8] = ptr %2044
 30505|     ;; self[0..+8] = ptr %2541
 30506|     ;; self[8..+8] = !DIArgList(ptr %2541, i64 %2543)
 30507|     ;; init[0..+8] = i64 0
 30508|     ;; init[8..+8] = i64 0
 30509|     ;; rhs = i64 1
 30510|     ;; end = !DIArgList(ptr %2541, i64 %2543)
 30513|  %2544 = icmp eq i64 %2543, 0                                                                                          ;L1714<44<128<986
 30514|  br i1 %2544, label %2569, label %2545                                                                                 ;L25<128<986
 30515| 
 30516| 2545: ; preds = %2562, %2539
 30517|  %2546 = phi i64 [ %2567, %2562 ], [ 0, %2539 ]                                                                        ;L0<128<986
 30518|  %2547 = phi i64 [ %2566, %2562 ], [ 0, %2539 ]                                                                        ;L0<128<986
 30519|  %2548 = phi i64 [ %2565, %2562 ], [ 0, %2539 ]                                                                        ;L0<128<986
 30520|     ;; acc[0..+8] = i64 %2548
 30521|     ;; acc[8..+8] = i64 %2547
 30522|     ;; self = i64 %2546
 30523|     ;; i = i64 %2546
 30524|     ;; self = ptr %2541
 30525|     ;; count = i64 %2546
 30526|  %2549 = getelementptr { { { { { ptr, ptr } } }, {}, {} }, i32, [1 x i32] }, ptr %2541, i64 %2546                      ;L656<279<128<986
 30527|  %2550 = load ptr, ptr %2549, , !!45249, !!8, !!8                                                                      ;L279<128<986
 30528|  %2551 = gep %2549, i64 8                                                                                              ;L279<128<986
 30529|  %2552 = load ptr, ptr %2551, , !!45249, !!8, !!8                                                                      ;L279<128<986
 30531|     ;; acc[0..+8] = i64 %2548
 30532|     ;; acc[8..+8] = i64 %2547
 30539|  %2553 = gep %2552, i64 16                                                                                             ;L2445<985<88<279<128<986
 30540|  %2554 = load i64, ptr %2553,                                                                                          ;L2445<985<88<279<128<986
 30541|  %2555 = add nsw i64 %2554, -1                                                                                         ;L2445<985<88<279<128<986
 30542|  %2556 = and i64 %2555, -16                                                                                            ;L2445<985<88<279<128<986
 30543|  %2557 = gep %2550, i64 %2556                                                                                          ;L2445<985<88<279<128<986
 30544|  %2558 = gep %2557, i64 16                                                                                             ;L2445<985<88<279<128<986
 30545|  %2559 = gep %2552, i64 40                                                                                             ;L985<88<279<128<986
 30546|  %2560 = load ptr, ptr %2559, , !!8                                                                                    ;L985<88<279<128<986
 30547|  %2561 = invoke { i64, i64 } %2560(ptr %2558, ptr %135, ptr %2044, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11)
 30548|  to label %2562 unwind label %2015                                                                                     ;L985<88<279<128<986
 30549| 
 30550| 2562: ; preds = %2545
 30551|  %2563 = extractvalue { i64, i64 } %2561, 0                                                                            ;L88<279<128<986
 30552|  %2564 = extractvalue { i64, i64 } %2561, 1                                                                            ;L88<279<128<986
 30558|     ;; ad = i64 %2548
 30559|     ;; ap = i64 %2547
 30560|     ;; ad2 = i64 %2563
 30561|     ;; ap2 = i64 %2564
 30562|  %2565 = add i64 %2563, %2548                                                                                          ;L986<88<279<128<986
 30563|  %2566 = add i64 %2564, %2547                                                                                          ;L986<88<279<128<986
 30564|     ;; acc[0..+8] = i64 %2565
 30565|     ;; acc[8..+8] = i64 %2566
 30566|  %2567 = add nuw i64 %2546, 1                                                                                          ;L971<283<128<986
 30567|     ;; i = i64 %2567
 30568|     ;; self = i64 %2567
 30569|  %2568 = icmp eq i64 %2567, %2543                                                                                      ;L284<128<986
 30570|  br i1 %2568, label %2569, label %2545                                                                                 ;L284<128<986
 30571| 
 30572| 2569: ; preds = %2562, %2539
 30573|  %2570 = phi i64 [ 0, %2539 ], [ %2566, %2562 ]                                                                        ;L0<128<986
 30574|  %2571 = phi i64 [ 0, %2539 ], [ %2565, %2562 ]                                                                        ;L0<128<986
 30575|     ;; ad = i64 %2571
 30577|  %2572 = invoke i64 @_RINvNtCs97f5S1uJLkH_9game_core5utils10get_damageNtNtNtB4_10simulation6entity6EntityBK_ECshdEBA0ozCnw_7game_ai(ptr %2044, ptr %122, i64 %2571, i32 1, i32 0)
 30578|  to label %2573 unwind label %2015                                                                                     ;L987
 30579| 
 30580| 2573: ; preds = %2569
 30581|     ;; ap = i64 %2570
 30582|  %2574 = invoke i64 @_RINvNtCs97f5S1uJLkH_9game_core5utils10get_damageNtNtNtB4_10simulation6entity6EntityBK_ECshdEBA0ozCnw_7game_ai(ptr %2044, ptr %122, i64 %2570, i32 1, i32 1)
 30583|  to label %2575 unwind label %2015                                                                                     ;L988
 30584| 
 30585| 2575: ; preds = %2573
 30586|  %2576 = add i64 %2574, %2572                                                                                          ;L987
 30587|     ;; dmg = i64 %2576
 30588|     ;; x = i64 %2576
 30589|     ;; inv_hp_q32 = i64 %149
 30590|  %2577 = zext i64 %2576 to i128                                                                                        ;L387<989
 30591|  %2578 = mul nuw nsw i128 %335, %2577                                                                                  ;L387<989
 30592|  %2579 = lshr i128 %2578, 32                                                                                           ;L387<989
 30593|     ;; self = i128 %2579
 30594|     ;; other = i128 150
 30595|  %2580 = call i128 @llvm.umin.i128(i128 %2579, i128 150)                                                               ;L1078<387<989
 30596|  %2581 = trunc nuw nsw i128 %2580 to i64                                                                               ;L387<989
 30597|     ;; self = i64 0
 30598|     ;; other = i64 %2581
 30599|     ;; rush_ratio = i64 %2581
 30600|  %2582 = trunc nuw i8 %2273 to i1                                                                                      ;L990
 30601|  br i1 %2582, label %2607, label %2583                                                                                 ;L990
 30602| 
 30603| 2583: ; preds = %2575
 30604|  %2584 = load ptr, ptr %2540, , !!8, !!8                                                                               ;L614<609<296<1968<1864<3787<990
 30605|  %2585 = load i64, ptr %2542, , !!8                                                                                    ;L1864<3787<990
 30606|     ;; len = i64 %2585
 30607|     ;; count = i64 %2585
 30608|     ;; self[0..+8] = ptr %2584
 30609|     ;; slice[0..+8] = ptr %2584
 30610|     ;; self[8..+8] = i64 %2585
 30611|     ;; slice[8..+8] = i64 %2585
 30612|     ;; ptr = ptr %2584
 30613|     ;; self = ptr %2584
 30614|  %2586 = getelementptr { { { { { ptr, ptr } } }, {}, {} }, i32, [1 x i32] }, ptr %2584, i64 %2585                      ;L961<100<1042<990
 30615|     ;; f = ptr undef
 30616|     ;; self = ptr undef
 30617|     ;; self = ptr undef
 30618|     ;; count = i64 1
 30619|  br label %2587                                                                                                        ;L331<990
 30620| 
 30621| 2587: ; preds = %2601, %2583
 30622|  %2588 = phi ptr [ %2602, %2601 ], [ %2584, %2583 ]
 30623|     ;; ptr = ptr %2588
 30624|     ;; self = ptr %2588
 30625|     ;; end_or_len = ptr %2586
 30628|  %2589 = icmp ne ptr %2588, %2586                                                                                      ;L1714<180<331<990
 30629|  br i1 %2589, label %2590, label %2605                                                                                 ;L180<331<990
 30630| 
 30631| 2590: ; preds = %2587
 30632|     ;; x = ptr %2588
 30633|  %2591 = load ptr, ptr %2588, , !!45355, !!8, !!8                                                                      ;L332<990
 30634|  %2592 = gep %2588, i64 8                                                                                              ;L332<990
 30635|  %2593 = load ptr, ptr %2592, , !!45355, !!8, !!8                                                                      ;L332<990
 30641|  %2594 = gep %2593, i64 16                                                                                             ;L2445<4217<990<332<990
 30642|  %2595 = load i64, ptr %2594, , !!45355                                                                                ;L2445<4217<990<332<990
 30643|  %2596 = add nsw i64 %2595, -1                                                                                         ;L2445<4217<990<332<990
 30644|  %2597 = and i64 %2596, -16                                                                                            ;L2445<4217<990<332<990
 30645|  %2598 = gep %2591, i64 %2597                                                                                          ;L2445<4217<990<332<990
 30646|  %2599 = gep %2598, i64 16                                                                                             ;L2445<4217<990<332<990
 30647|  %2600 = invoke { i64, i64 } @ai::fight_check19effect_type_cc_time(i64 %1, ptr %2599, ptr %2593)
 30648|  to label %2601 unwind label %2015                                                                                     ;L990<332<990
 30649| 
 30650| 2601: ; preds = %2590
 30651|  %2602 = gep %2588, i64 24                                                                                             ;L656<185<331<990
 30652|  %2603 = extractvalue { i64, i64 } %2600, 0                                                                            ;L990<332<990
 30654|  %2604 = icmp eq i64 %2603, 1                                                                                          ;L430<990<332<990
 30655|  br i1 %2604, label %2605, label %2587                                                                                 ;L332<990
 30656| 
 30657| 2605: ; preds = %2601, %2587
 30658|  %2606 = zext i1 %2589 to i8                                                                                           ;L990
 30659|     ;; has_cc = i8 %2606
 30660|     ;; score[48..+1] = i8 1
 30661|     ;; score[48..+1] = i8 1
 30662|  br label %2480                                                                                                        ;L980
 30663| 
 30664| 2607: ; preds = %2575, %2480
 30665|  %2608 = phi i8 [ %2481, %2480 ], [ 1, %2575 ]                                                                         ;L0
 30666|  %2609 = phi i64 [ %2482, %2480 ], [ %2581, %2575 ]                                                                    ;L0
 30667|     ;; score[48..+1] = i8 %2608
 30668|     ;; score[48..+1] = i8 %2608
 30669|     ;; rush_ratio = i64 %2609
 30670|  %2610 = add nuw i64 %2467, %2466                                                                                      ;L996
 30671|  %2611 = add i64 %2610, %2468                                                                                          ;L996
 30672|  %2612 = add i64 %2611, %2469                                                                                          ;L996
 30673|  %2613 = add i64 %2612, %2609                                                                                          ;L996
 30674|     ;; risk = i64 %2613
 30675|  br label %2619                                                                                                        ;L995
 30676| 
 30677| 2614: ; preds = %2480
 30678|     ;; self = i64 %2468
 30679|     ;; other = i64 %2467
 30680|  %2615 = call i64 @llvm.smax.i64(i64 %2467, i64 %2468)                                                                 ;L1039<998
 30681|     ;; self = i64 %2615
 30682|     ;; other = i64 %2466
 30683|  %2616 = call i64 @llvm.smax.i64(i64 %2466, i64 %2615)                                                                 ;L1039<998
 30684|     ;; self = i64 %2616
 30685|     ;; other = i64 %2469
 30686|  %2617 = call i64 @llvm.smax.i64(i64 %2469, i64 %2616)                                                                 ;L1039<998
 30687|  %2618 = add nuw i64 %2482, %2617                                                                                      ;L998
 30688|     ;; risk = i64 %2618
 30689|  br label %2619                                                                                                        ;L995
 30690| 
 30691| 2619: ; preds = %2614, %2607
 30692|  %2620 = phi i8 [ %2608, %2607 ], [ %2481, %2614 ]                                                                     ;L367<390
 30693|  %2621 = phi i64 [ %2613, %2607 ], [ %2618, %2614 ]                                                                    ;L0
 30694|     ;; score[48..+1] = i8 %2620
 30695|     ;; score[48..+1] = i8 %2620
 30696|     ;; risk = i64 %2621
 30697|  %2622 = icmp sgt i64 %2621, 0                                                                                         ;L1001
 30698|  %2623 = select i1 %1787, i1 %2622, i1 false                                                                           ;L1001
 30699|  br i1 %2623, label %2624, label %2682                                                                                 ;L1001
 30700| 
 30701| 2624: ; preds = %2619
 30702|     ;; cell_to_e_sq = i64 %2009
 30703|     ;; self = ptr %93
 30704|     ;; self = ptr %93
 30705|  %2625 = load ptr, ptr %93, , !!8, !!8                                                                                 ;L138<2073<1003
 30706|     ;; p = ptr %2625
 30707|  %2626 = load i64, ptr %230, , !!8                                                                                     ;L2075<1003
 30708|     ;; len = i64 %2626
 30709|     ;; count = i64 %2626
 30710|     ;; self[0..+8] = ptr %2625
 30711|     ;; slice[0..+8] = ptr %2625
 30712|     ;; self[8..+8] = i64 %2626
 30713|     ;; slice[8..+8] = i64 %2626
 30714|     ;; ptr = ptr %2625
 30715|     ;; self = ptr %2625
 30716|  %2627 = mul nuw nsw i64 %2626, 448                                                                                    ;L961<100<1042<1003
 30717|  %2628 = gep %2625, i64 %2627                                                                                          ;L961<100<1042<1003
 30718|     ;; f[0..+8] = ptr %2044
 30719|     ;; f[8..+8] = ptr undef
 30720|     ;; f[16..+8] = ptr %100
 30721|     ;; f[24..+8] = ptr %99
 30722|     ;; self = ptr undef
 30723|     ;; self = ptr undef
 30724|     ;; count = i64 1
 30725|     ;; ptr = ptr %2625
 30726|     ;; self = ptr %2625
 30727|     ;; end_or_len = ptr %2628
 30730|  %2629 = icmp eq i64 %2626, 0                                                                                          ;L1714<180<331<1003
 30731|  br i1 %2629, label %2682, label %2630                                                                                 ;L180<331<1003
 30732| 
 30733| 2630: ; preds = %2624
 30734|  %2631 = gep %2044, i64 1632
 30735|  %2632 = gep %2044, i64 1640
 30736|  br label %2633                                                                                                        ;L180<331<1003
 30737| 
 30738| 2633: ; preds = %2678, %2630
 30739|  %2634 = phi ptr [ %2625, %2630 ], [ %2635, %2678 ]
 30740|     ;; ptr = ptr %2634
 30741|  %2635 = gep %2634, i64 448                                                                                            ;L656<185<331<1003
 30742|     ;; x = ptr %2634
 30743|  %2636 = gep %2634, i64 432                                                                                            ;L332<1003
 30744|  %2637 = load ptr, ptr %2636, , !!45440, !!8, !!8                                                                      ;L332<1003
 30751|     ;; self = ptr %2637
 30752|     ;; self = ptr %2637
 30753|     ;; other = ptr %2044
 30754|  %2638 = gep %2637, i64 1632                                                                                           ;L2158<1004<332<1003
 30755|  %2639 = load i64, ptr %2638, , !!45473, !!8                                                                           ;L2158<1004<332<1003
 30756|     ;; x1 = i64 %2639
 30757|     ;; self = i64 %2639
 30758|  %2640 = gep %2637, i64 1640                                                                                           ;L2158<1004<332<1003
 30759|  %2641 = load i64, ptr %2640, , !!45473, !!8                                                                           ;L2158<1004<332<1003
 30760|     ;; y1 = i64 %2641
 30761|     ;; self = i64 %2641
 30762|  %2642 = load i64, ptr %2631, , !!45473, !!8                                                                           ;L2158<1004<332<1003
 30763|     ;; x2 = i64 %2642
 30764|     ;; other = i64 %2642
 30765|  %2643 = load i64, ptr %2632, , !!45473, !!8                                                                           ;L2158<1004<332<1003
 30766|     ;; y2 = i64 %2643
 30767|     ;; other = i64 %2643
 30768|  %2644 = icmp ult i64 %2639, %2642                                                                                     ;L3147<7<2158<1004<332<1003
 30769|  %2645 = sub nuw i64 %2642, %2639                                                                                      ;L3147<7<2158<1004<332<1003
 30770|  %2646 = sub nuw i64 %2639, %2642                                                                                      ;L3147<7<2158<1004<332<1003
 30771|  %2647 = select i1 %2644, i64 %2645, i64 %2646                                                                         ;L3147<7<2158<1004<332<1003
 30772|     ;; dx = i64 %2647
 30773|  %2648 = icmp ult i64 %2641, %2643                                                                                     ;L3147<8<2158<1004<332<1003
 30774|  %2649 = sub nuw i64 %2643, %2641                                                                                      ;L3147<8<2158<1004<332<1003
 30775|  %2650 = sub nuw i64 %2641, %2643                                                                                      ;L3147<8<2158<1004<332<1003
 30776|  %2651 = select i1 %2648, i64 %2649, i64 %2650                                                                         ;L3147<8<2158<1004<332<1003
 30777|     ;; dy = i64 %2651
 30778|  %2652 = mul i64 %2647, %2647                                                                                          ;L9<2158<1004<332<1003
 30779|  %2653 = mul i64 %2651, %2651                                                                                          ;L9<2158<1004<332<1003
 30780|  %2654 = add i64 %2653, %2652                                                                                          ;L9<2158<1004<332<1003
 30781|  %2655 = icmp ult i64 %2654, %2009                                                                                     ;L1004<332<1003
 30782|  br i1 %2655, label %2656, label %2678                                                                                 ;L1004<332<1003
 30783| 
 30784| 2656: ; preds = %2633
 30785|  %2657 = load i64, ptr %100, , !!45473, !!8                                                                            ;L1005<332<1003
 30786|  %2658 = load i64, ptr %99, , !!45473, !!8                                                                             ;L1005<332<1003
 30787|  %2659 = invoke i64 @gc::utils20dist_to_line_segment(i64 %2639, i64 %2641, i64 %2657, i64 %2658, i64 %2642, i64 %2643)
 30788|  to label %2660 unwind label %2015                                                                                     ;L1005<332<1003
 30789| 
 30790| 2660: ; preds = %2656
 30791|  %2661 = gep %2637, i64 1136                                                                                           ;L1511<1006<332<1003
 30792|  %2662 = load i32, ptr %2661, , !!45473, !!8                                                                           ;L1511<1006<332<1003
 30793|     ;; mult = i32 %2662
 30794|  %2663 = icmp eq i32 %2662, 0                                                                                          ;L1512<1006<332<1003
 30795|  br i1 %2663, label %2664, label %2667                                                                                 ;L1512<1006<332<1003
 30796| 
 30797| 2664: ; preds = %2660
 30798|  %2665 = gep %2637, i64 1664                                                                                           ;L1513<1006<332<1003
 30799|  %2666 = load i64, ptr %2665, , !!45473, !!8                                                                           ;L1513<1006<332<1003
 30800|  br label %2674                                                                                                        ;L1512<1006<332<1003
 30801| 
 30802| 2667: ; preds = %2660
 30803|  %2668 = sext i32 %2662 to i64                                                                                         ;L1511<1006<332<1003
 30804|     ;; mult = i64 %2668
 30805|  %2669 = gep %2637, i64 1664                                                                                           ;L1515<1006<332<1003
 30806|  %2670 = load i64, ptr %2669, , !!45473, !!8                                                                           ;L1515<1006<332<1003
 30807|  %2671 = add nsw i64 %2668, 100                                                                                        ;L1515<1006<332<1003
 30808|  %2672 = mul i64 %2670, %2671                                                                                          ;L1515<1006<332<1003
 30809|  %2673 = udiv i64 %2672, 100                                                                                           ;L1515<1006<332<1003
 30810|  br label %2674                                                                                                        ;L1512<1006<332<1003
 30811| 
 30812| 2674: ; preds = %2667, %2664
 30813|  %2675 = phi i64 [ %2666, %2664 ], [ %2673, %2667 ]                                                                    ;L0<1006<332<1003
 30814|  %2676 = add i64 %2675, 28000                                                                                          ;L1006<332<1003
 30815|  %2677 = icmp ugt i64 %2659, %2676                                                                                     ;L1005<332<1003
 30816|  br i1 %2677, label %2678, label %2680                                                                                 ;L332<1003
 30817| 
 30818| 2678: ; preds = %2674, %2633
 30819|     ;; ptr = ptr %2635
 30820|     ;; self = ptr %2635
 30821|     ;; end_or_len = ptr %2628
 30824|  %2679 = icmp eq ptr %2635, %2628                                                                                      ;L1714<180<331<1003
 30825|  br i1 %2679, label %2682, label %2633                                                                                 ;L180<331<1003
 30826| 
 30827| 2680: ; preds = %2674
 30828|  %2681 = lshr i64 %2621, 1                                                                                             ;L1008
 30829|     ;; risk = i64 %2681
 30830|  br label %2682                                                                                                        ;L1001
 30831| 
 30832| 2682: ; preds = %2680, %2678, %2624, %2619
 30833|  %2683 = phi i64 [ %2681, %2680 ], [ %2621, %2619 ], [ %2621, %2624 ], [ %2621, %2678 ]                                ;L0
 30834|     ;; risk = i64 %2683
 30835|  br i1 %156, label %2688, label %2684                                                                                  ;L1012
 30836| 
 30837| 2684: ; preds = %2682
 30838|  %2685 = sdiv i64 %2683, 2                                                                                             ;L1012
 30839|     ;; added = i64 %2685
 30840|  %2686 = add i64 %2683, %1996                                                                                          ;L1015
 30841|  %2687 = sub i64 %2686, %2685                                                                                          ;L1015
 30842|     ;; score[40..+8] = i64 %2687
 30843|     ;; score[40..+8] = i64 %2687
 30844|  br label %2688                                                                                                        ;L1013
 30845| 
 30846| 2688: ; preds = %2684, %2682
 30847|  %2689 = phi i64 [ %1996, %2682 ], [ %2687, %2684 ]                                                                    ;L0
 30848|  %2690 = phi i64 [ %2683, %2682 ], [ %2685, %2684 ]                                                                    ;L1012
 30849|     ;; score[40..+8] = i64 %2689
 30850|     ;; score[40..+8] = i64 %2689
 30851|     ;; added = i64 %2690
 30852|  %2691 = add i64 %2690, %1997                                                                                          ;L1017
 30853|     ;; score[0..+8] = i64 %2691
 30854|     ;; score[0..+8] = i64 %2691
 30855|  %2692 = add i64 %2690, %1999                                                                                          ;L1018
 30856|     ;; champ_threat_risk = i64 %2692
 30857|  br label %1994                                                                                                        ;L853
 30858| 
 30859| 2693: ; preds = %2013
 30860|     ;; self = ptr %93
 30861|     ;; self = ptr %93
 30862|  %2694 = load ptr, ptr %93, , !!8, !!8                                                                                 ;L138<2073<1025
 30863|     ;; p = ptr %2694
 30864|  %2695 = load i64, ptr %230, , !!8                                                                                     ;L2075<1025
 30865|     ;; count = i64 %2695
 30866|     ;; self[0..+8] = ptr %2694
 30867|     ;; slice[0..+8] = ptr %2694
 30868|     ;; self[8..+8] = i64 %2695
 30869|     ;; slice[8..+8] = i64 %2695
 30870|     ;; self = ptr %2694
 30871|     ;; self[0..+8] = ptr %2694
 30872|     ;; self[8..+8] = !DIArgList(ptr %2694, i64 %2695)
 30873|     ;; init = i64 0
 30875|     ;; before = i64 %2695
 30876|     ;; self[0..+8] = ptr %2694
 30877|     ;; iter[0..+8] = ptr %2694
 30878|     ;; self[0..+8] = ptr %2694
 30879|     ;; self[8..+8] = !DIArgList(ptr %2694, i64 %2695)
 30880|     ;; iter[8..+8] = !DIArgList(ptr %2694, i64 %2695)
 30881|     ;; self[8..+8] = !DIArgList(ptr %2694, i64 %2695)
 30882|     ;; self[0..+8] = ptr %2694
 30883|     ;; self[8..+8] = !DIArgList(ptr %2694, i64 %2695)
 30884|     ;; init = i64 0
 30886|     ;; rhs = i64 1
 30887|     ;; end = !DIArgList(ptr %2694, i64 %2695)
 30890|  %2696 = icmp eq i64 %2695, 0                                                                                          ;L1714<44<128<52<3674<142<1025
 30891|  br i1 %2696, label %2732, label %2697                                                                                 ;L25<128<52<3674<142<1025
 30892| 
 30893| 2697: ; preds = %2697, %2693
 30894|  %2698 = phi i64 [ %2706, %2697 ], [ 0, %2693 ]                                                                        ;L0<128<52<3674<142<1025
 30895|  %2699 = phi i64 [ %2705, %2697 ], [ 0, %2693 ]                                                                        ;L0<128<52<3674<142<1025
 30896|     ;; acc = i64 %2699
 30897|     ;; self = i64 %2698
 30898|     ;; i = i64 %2698
 30899|     ;; self = ptr %2694
 30900|     ;; count = i64 %2698
 30901|  %2700 = gepS, ptr, i64 }, ptr %2694, i64 %2698                                                                        ;L656<279<128<52<3674<142<1025
 30902|  %2701 = gep %2700, i64 440                                                                                            ;L279<128<52<3674<142<1025
 30903|  %2702 = load i64, ptr %2701, , !!8                                                                                    ;L279<128<52<3674<142<1025
 30904|     ;; acc = i64 %2699
 30910|  %2703 = icmp ult i64 %2702, 14400000001                                                                               ;L1025<138<88<279<128<52<3674<142<1025
 30911|  %2704 = zext i1 %2703 to i64                                                                                          ;L138<88<279<128<52<3674<142<1025
 30913|     ;; a = i64 %2699
 30914|     ;; b = i64 %2704
 30915|  %2705 = add i64 %2699, %2704                                                                                          ;L55<88<279<128<52<3674<142<1025
 30916|     ;; acc = i64 %2705
 30917|  %2706 = add nuw i64 %2698, 1                                                                                          ;L971<283<128<52<3674<142<1025
 30918|     ;; i = i64 %2706
 30919|     ;; self = i64 %2706
 30920|  %2707 = icmp eq i64 %2706, %2695                                                                                      ;L284<128<52<3674<142<1025
 30921|  br i1 %2707, label %2732, label %2697                                                                                 ;L284<128<52<3674<142<1025
 30922| 
 30923| 2708: ; preds = %2754, %2749, %2732, %2013
 30924|  %2709 = phi i64 [ %2759, %2754 ], [ %1997, %2749 ], [ %1997, %2013 ], [ %1997, %2732 ]                                ;L0
 30925|     ;; score[0..+8] = i64 %2709
 30926|     ;; score[0..+8] = i64 %2709
 30928|  call void @llvm.memcpy.p0.p0.i64(ptr %74, ptr %75, i64 24, i1 false)                                                  ;L1034
 30931|  %2710 = gep %74, i64 16                                                                                               ;L825<1004<1034
 30932|  %2711 = load i32, ptr %2710, , !!8                                                                                    ;L825<1004<1034
 30933|  %2712 = icmp eq i32 %2711, -1                                                                                         ;L825<1004<1034
 30934|  br i1 %2712, label %2760, label %2713                                                                                 ;L825<1004<1034
 30935| 
 30936| 2713: ; preds = %2708
 30940|     ;; self = ptr %74
 30941|     ;; order = i8 0
 30942|     ;; order = i8 0
 30943|     ;; val = i64 1
 30944|     ;; order = i8 0
 30945|     ;; val = i64 1
 30946|     ;; order = i8 0
 30947|  %2714 = load i64, ptr %74, , !!8                                                                                      ;L185<825<825<1004<1034
 30948|  %2715 = icmp ult i64 %2714, 132                                                                                       ;L185<825<825<1004<1034
 30949|  br i1 %2715, label %2718, label %2716                                                                                 ;L185<825<825<1004<1034
 30950| 
 30951| 2716: ; preds = %2713
 30952|  invoke void @core::panicking18panic_bounds_check(i64 %2714, i64 132, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.208) #25
 30953|  to label %2717 unwind label %2015                                                                                     ;L185<825<825<1004<1034
 30954| 
 30955| 2717: ; preds = %2716
 30956|  unreachable                                                                                                           ;L185<825<825<1004<1034
 30957| 
 30958| 2718: ; preds = %2713
 30959|     ;; self = !DIArgList(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 %2714)
 30960|  %2719 = gep %74, i64 8                                                                                                ;L185<825<825<1004<1034
 30961|  %2720 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %2719)
 30962|  to label %2721 unwind label %2015                                                                                     ;L185<825<825<1004<1034
 30963| 
 30964| 2721: ; preds = %2718
 30965|  %2722 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %2714                               ;L185<825<825<1004<1034
 30966|     ;; self = ptr %2722
 30967|  %2723 = extractvalue { i64, i32 } %2720, 0                                                                            ;L185<825<825<1004<1034
 30968|  %2724 = extractvalue { i64, i32 } %2720, 1                                                                            ;L185<825<825<1004<1034
 30970|  %2725 = mul i64 %2723, 1000000000                                                                                     ;L632<185<825<825<1004<1034
 30971|  %2726 = icmp ult i32 %2724, 1000000000                                                                                ;L49<632<185<825<825<1004<1034
 30972|  call void @llvm.assume(i1 %2726)                                                                                      ;L49<632<185<825<825<1004<1034
 30973|  %2727 = zext nneg i32 %2724 to i64                                                                                    ;L632<185<825<825<1004<1034
 30974|  %2728 = add i64 %2725, %2727                                                                                          ;L632<185<825<825<1004<1034
 30975|     ;; val = i64 %2728
 30976|     ;; val = i64 %2728
 30977|     ;; dst = ptr %2722
 30978|  %2729 = atomicrmw add ptr %2722, i64 %2728 monotonic, , !!45727                                                       ;L3937<3162<185<825<825<1004<1034
 30979|  %2730 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %2714                               ;L186<825<825<1004<1034
 30980|     ;; self = ptr %2730
 30981|     ;; dst = ptr %2730
 30982|  %2731 = atomicrmw add ptr %2730, i64 1 monotonic, , !!45727                                                           ;L3937<3162<186<825<825<1004<1034
 30983|  br label %2760                                                                                                        ;L825<1004<1034
 30984| 
 30985| 2732: ; preds = %2697, %2693
 30986|  %2733 = phi i64 [ 0, %2693 ], [ %2705, %2697 ]                                                                        ;L0<128<52<3674<142<1025
 30987|     ;; total = i64 %2733
 30988|     ;; count = i64 %2733
 30989|     ;; upper = i64 %2695
 30990|  %2734 = icmp ule i64 %2733, %2695                                                                                     ;L251<145<1025
 30991|     ;; cond = i1 true
 30992|  call void @llvm.assume(i1 %2734)                                                                                      ;L210<251<145<1025
 30993|     ;; cell_a = i64 %2733
 30994|     ;; self = ptr %95
 30995|     ;; self = ptr %95
 30996|  %2735 = load ptr, ptr %95, , !!8, !!8                                                                                 ;L138<2073<1026
 30997|     ;; p = ptr %2735
 30998|  %2736 = load i64, ptr %200, , !!8                                                                                     ;L2075<1026
 30999|     ;; count = i64 %2736
 31000|     ;; self[0..+8] = ptr %2735
 31001|     ;; slice[0..+8] = ptr %2735
 31002|     ;; self[8..+8] = i64 %2736
 31003|     ;; slice[8..+8] = i64 %2736
 31004|     ;; self = ptr %2735
 31005|     ;; self[0..+8] = ptr %2735
 31006|     ;; self[8..+8] = !DIArgList(ptr %2735, i64 %2736)
 31007|     ;; init = i64 0
 31009|     ;; before = i64 %2736
 31010|     ;; self[0..+8] = ptr %2735
 31011|     ;; iter[0..+8] = ptr %2735
 31012|     ;; self[0..+8] = ptr %2735
 31013|     ;; self[8..+8] = !DIArgList(ptr %2735, i64 %2736)
 31014|     ;; iter[8..+8] = !DIArgList(ptr %2735, i64 %2736)
 31015|     ;; self[8..+8] = !DIArgList(ptr %2735, i64 %2736)
 31016|     ;; self[0..+8] = ptr %2735
 31017|     ;; self[8..+8] = !DIArgList(ptr %2735, i64 %2736)
 31018|     ;; init = i64 0
 31020|     ;; rhs = i64 1
 31021|     ;; end = !DIArgList(ptr %2735, i64 %2736)
 31024|  %2737 = icmp eq i64 %2736, 0                                                                                          ;L1714<44<128<52<3674<142<1026
 31025|  br i1 %2737, label %2708, label %2738                                                                                 ;L25<128<52<3674<142<1026
 31026| 
 31027| 2738: ; preds = %2738, %2732
 31028|  %2739 = phi i64 [ %2747, %2738 ], [ 0, %2732 ]                                                                        ;L0<128<52<3674<142<1026
 31029|  %2740 = phi i64 [ %2746, %2738 ], [ 0, %2732 ]                                                                        ;L0<128<52<3674<142<1026
 31030|     ;; acc = i64 %2740
 31031|     ;; self = i64 %2739
 31032|     ;; i = i64 %2739
 31033|     ;; self = ptr %2735
 31034|     ;; count = i64 %2739
 31035|  %2741 = gepS, ptr, i64 }, ptr %2735, i64 %2739                                                                        ;L656<279<128<52<3674<142<1026
 31036|  %2742 = gep %2741, i64 440                                                                                            ;L279<128<52<3674<142<1026
 31037|  %2743 = load i64, ptr %2742, , !!8                                                                                    ;L279<128<52<3674<142<1026
 31038|     ;; acc = i64 %2740
 31044|  %2744 = icmp ult i64 %2743, 14400000001                                                                               ;L1026<138<88<279<128<52<3674<142<1026
 31045|  %2745 = zext i1 %2744 to i64                                                                                          ;L138<88<279<128<52<3674<142<1026
 31047|     ;; a = i64 %2740
 31048|     ;; b = i64 %2745
 31049|  %2746 = add i64 %2740, %2745                                                                                          ;L55<88<279<128<52<3674<142<1026
 31050|     ;; acc = i64 %2746
 31051|  %2747 = add nuw i64 %2739, 1                                                                                          ;L971<283<128<52<3674<142<1026
 31052|     ;; i = i64 %2747
 31053|     ;; self = i64 %2747
 31054|  %2748 = icmp eq i64 %2747, %2736                                                                                      ;L284<128<52<3674<142<1026
 31055|  br i1 %2748, label %2749, label %2738                                                                                 ;L284<128<52<3674<142<1026
 31056| 
 31057| 2749: ; preds = %2738
 31058|     ;; total = i64 %2746
 31059|     ;; count = i64 %2746
 31060|     ;; upper = i64 %2736
 31061|  %2750 = icmp ule i64 %2746, %2736                                                                                     ;L251<145<1026
 31062|     ;; cond = i1 true
 31063|  call void @llvm.assume(i1 %2750)                                                                                      ;L210<251<145<1026
 31064|     ;; cell_e = i64 %2746
 31065|  %2751 = icmp eq i64 %2733, 0                                                                                          ;L1029
 31066|  %2752 = icmp ugt i64 %2746, 1                                                                                         ;L1029
 31067|  %2753 = and i1 %2751, %2752                                                                                           ;L1029
 31068|  br i1 %2753, label %2754, label %2708                                                                                 ;L1029
 31069| 
 31070| 2754: ; preds = %2749
 31071|     ;; self = i64 %2746
 31072|     ;; other = i64 4
 31073|  %2755 = call i64 @llvm.smin.i64(i64 %2746, i64 4)                                                                     ;L1078<1030
 31074|  %2756 = mul i64 %1999, 25                                                                                             ;L1030
 31075|  %2757 = mul i64 %2756, %2755                                                                                          ;L1030
 31076|  %2758 = sdiv i64 %2757, 100                                                                                           ;L1030
 31077|  %2759 = add i64 %2758, %1997                                                                                          ;L1030
 31078|     ;; score[0..+8] = i64 %2759
 31079|     ;; score[0..+8] = i64 %2759
 31080|  br label %2708                                                                                                        ;L1029
 31081| 
 31082| 2760: ; preds = %2721, %2708
 31085|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 31086|     ;; order = i8 0
 31087|  %2761 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                   ;L3904<741<176<1035
 31088|  %2762 = icmp eq i8 %2761, 0                                                                                           ;L176<1035
 31089|  br i1 %2762, label %2765, label %2763                                                                                 ;L176<1035
 31090| 
 31091| 2763: ; preds = %2760
 31092|  %2764 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()
 31093|  to label %2773 unwind label %2015                                                                                     ;L179<1035
 31094| 
 31095| 2765: ; preds = %2773, %2760
 31096|  %2766 = phi i32 [ %2775, %2773 ], [ -1, %2760 ]
 31097|  %2767 = gep %73, i64 16                                                                                               ;L0<1035
 31098|  store i32 %2766, ptr %2767,                                                                                           ;L0<1035
 31099|     ;; self = ptr %93
 31100|     ;; self = ptr %93
 31101|  %2768 = load ptr, ptr %93, , !!8, !!8                                                                                 ;L138<2073<1036
 31102|     ;; p = ptr %2768
 31103|  %2769 = load i64, ptr %230, , !!8                                                                                     ;L2075<1036
 31104|     ;; len = i64 %2769
 31105|     ;; count = i64 %2769
 31106|     ;; self[0..+8] = ptr %2768
 31107|     ;; slice[0..+8] = ptr %2768
 31108|     ;; self[8..+8] = i64 %2769
 31109|     ;; slice[8..+8] = i64 %2769
 31110|     ;; ptr = ptr %2768
 31111|     ;; self = ptr %2768
 31112|  %2770 = mul nuw nsw i64 %2769, 448                                                                                    ;L961<100<1042<1036
 31113|  %2771 = gep %2768, i64 %2770                                                                                          ;L961<100<1042<1036
 31114|     ;; iter[0..+8] = ptr %2768
 31115|     ;; iter[8..+8] = ptr %2771
 31116|     ;; score[16..+8] = i64 %1783
 31117|     ;; score[16..+8] = i64 %1783
 31118|     ;; self = ptr undef
 31119|     ;; ptr = ptr %2768
 31120|     ;; self = ptr %2768
 31121|     ;; end_or_len = ptr %2771
 31124|  %2772 = icmp eq i64 %2769, 0                                                                                          ;L1714<180<1036
 31125|  br i1 %2772, label %2788, label %2777                                                                                 ;L180<1036
 31126| 
 31127| 2773: ; preds = %2763
 31128|  %2774 = extractvalue { i64, i32 } %2764, 0                                                                            ;L179<1035
 31129|  %2775 = extractvalue { i64, i32 } %2764, 1                                                                            ;L179<1035
 31130|  store i64 116, ptr %73,                                                                                               ;L179<1035
 31131|  %2776 = gep %73, i64 8                                                                                                ;L179<1035
 31132|  store i64 %2774, ptr %2776,                                                                                           ;L179<1035
 31133|  br label %2765                                                                                                        ;L180<1035
 31134| 
 31135| 2777: ; preds = %3037, %2765
 31136|  %2778 = phi ptr [ %2780, %3037 ], [ %2768, %2765 ]
 31137|  %2779 = phi i64 [ %3039, %3037 ], [ %1783, %2765 ]
 31138|     ;; score[16..+8] = i64 %2779
 31139|  %2780 = gep %2778, i64 448                                                                                            ;L656<185<1036
 31140|     ;; iter[0..+8] = ptr %2780
 31141|     ;; cache = ptr %2778
 31142|     ;; a = ptr %2778
 31143|     ;; dist = ptr %2778
 31144|     ;; max_ratio = i64 0
 31145|  %2781 = gep %2778, i64 440                                                                                            ;L1038
 31146|  %2782 = load i64, ptr %2781, , !!8                                                                                    ;L1038
 31147|     ;; dist = i64 %2782
 31148|  %2783 = gep %2778, i64 432                                                                                            ;L1039
 31149|  %2784 = load ptr, ptr %2783, , !!8, !!8                                                                               ;L1039
 31150|     ;; self = ptr %2784
 31151|     ;; self = ptr %2784
 31152|     ;; self = ptr %2784
 31153|     ;; self = ptr %2784
 31154|     ;; self = ptr %2784
 31155|     ;; self = ptr %2784
 31156|  %2785 = gep %2784, i64 1272                                                                                           ;L633<1039
 31157|  %2786 = load i32, ptr %2785, , !!8                                                                                    ;L633<1039
 31158|  %2787 = icmp eq i32 %2786, -1                                                                                         ;L633<1039
 31159|  br i1 %2787, label %2869, label %2791                                                                                 ;L1039
 31160| 
 31161| 2788: ; preds = %3037, %2765
 31162|  %2789 = phi i64 [ %1783, %2765 ], [ %3039, %3037 ]                                                                    ;L0
 31163|  %2790 = icmp samesign ult i8 %1785, 2                                                                                 ;L1101
 31164|  br i1 %2790, label %3150, label %3154                                                                                 ;L1101
 31165| 
 31166| 2791: ; preds = %2777
 31167|  %2792 = gep %2778, i64 8                                                                                              ;L1040
 31168|  %2793 = load i64, ptr %2792, , !!8                                                                                    ;L1040
 31169|     ;; ratio = i64 %2793
 31170|  %2794 = gep %2778, i64 64                                                                                             ;L1041
 31171|  %2795 = load i64, ptr %2794, , !!8                                                                                    ;L1041
 31172|     ;; range = i64 %2795
 31173|  %2796 = gep %2778, i64 416                                                                                            ;L1042
 31174|  %2797 = load i8, ptr %2796, , !!8                                                                                     ;L1042
 31175|  %2798 = trunc nuw i8 %2797 to i1                                                                                      ;L1042
 31176|  %2799 = gep %2778, i64 128                                                                                            ;L0
 31177|  %2800 = load i64, ptr %2799, , !!8                                                                                    ;L0
 31178|  %2801 = icmp ugt i64 %2782, %2800                                                                                     ;L0
 31179|  br i1 %2798, label %2803, label %2802                                                                                 ;L1042
 31180| 
 31181| 2802: ; preds = %2791
 31182|  br i1 %2801, label %2804, label %2808                                                                                 ;L1052
 31183| 
 31184| 2803: ; preds = %2791
 31185|  br i1 %2801, label %2816, label %2820                                                                                 ;L1043
 31186| 
 31187| 2804: ; preds = %2802
 31188|  %2805 = gep %2778, i64 136                                                                                            ;L1054
 31189|  %2806 = load i64, ptr %2805, , !!8                                                                                    ;L1054
 31190|  %2807 = icmp ugt i64 %2782, %2806                                                                                     ;L1054
 31191|  br i1 %2807, label %2869, label %2810                                                                                 ;L1054
 31192| 
 31193| 2808: ; preds = %2802
 31194|     ;; self = i64 0
 31195|     ;; other = i64 %2793
 31196|  %2809 = call i64 @llvm.smax.i64(i64 %2793, i64 0)                                                                     ;L1039<1053
 31197|  br label %2869                                                                                                        ;L1039<1053
 31198| 
 31199| 2810: ; preds = %2804
 31200|  %2811 = sdiv i64 %2793, 2                                                                                             ;L1055
 31201|     ;; self = i64 0
 31202|     ;; other = i64 %2811
 31203|  %2812 = call i64 @llvm.smax.i64(i64 %2811, i64 0)                                                                     ;L1039<1055
 31204|  br label %2869                                                                                                        ;L1039<1055
 31205| 
 31206| 2813: ; preds = %3248, %3246, %3231, %3201, %3169, %3167, %3138, %3115, %3044
 31207|  %2814 = phi i1 [ true, %3138 ], [ true, %3044 ], [ true, %3115 ], [ false, %3248 ], [ false, %3246 ], [ false, %3231 ], [ false, %3201 ], [ false, %3167 ], [ false, %3169 ] ;L0
 31208|  %2815 = cleanuppad within none []
 31209|  br i1 %2814, label %3313, label %3312                                                                                 ;L1181
 31210| 
 31211| 2816: ; preds = %2803
 31212|  %2817 = gep %2778, i64 144                                                                                            ;L1045
 31213|  %2818 = load i64, ptr %2817, , !!8                                                                                    ;L1045
 31214|  %2819 = icmp ugt i64 %2782, %2818                                                                                     ;L1045
 31215|  br i1 %2819, label %2824, label %2828                                                                                 ;L1045
 31216| 
 31217| 2820: ; preds = %2803
 31218|  %2821 = gep %2784, i64 1136                                                                                           ;L1511<1043
 31219|  %2822 = load i32, ptr %2821, , !!8                                                                                    ;L1511<1043
 31220|     ;; mult = i32 %2822
 31221|  %2823 = icmp eq i32 %2822, 0                                                                                          ;L1512<1043
 31222|  br i1 %2823, label %2833, label %2836                                                                                 ;L1512<1043
 31223| 
 31224| 2824: ; preds = %2816
 31225|  %2825 = gep %2778, i64 136                                                                                            ;L1049
 31226|  %2826 = load i64, ptr %2825, , !!8                                                                                    ;L1049
 31227|  %2827 = icmp ugt i64 %2782, %2826                                                                                     ;L1049
 31228|  br i1 %2827, label %2869, label %2830                                                                                 ;L1049
 31229| 
 31230| 2828: ; preds = %2860, %2816
 31231|     ;; self = i64 0
 31232|     ;; other = i64 %2793
 31233|  %2829 = call i64 @llvm.smax.i64(i64 %2793, i64 0)                                                                     ;L1039<1046
 31234|  br label %2869                                                                                                        ;L1039<1046
 31235| 
 31236| 2830: ; preds = %2824
 31237|  %2831 = sdiv i64 %2793, 3                                                                                             ;L1050
 31238|     ;; self = i64 0
 31239|     ;; other = i64 %2831
 31240|  %2832 = call i64 @llvm.smax.i64(i64 %2831, i64 0)                                                                     ;L1039<1050
 31241|  br label %2869                                                                                                        ;L1039<1050
 31242| 
 31243| 2833: ; preds = %2820
 31244|  %2834 = gep %2784, i64 1664                                                                                           ;L1513<1043
 31245|  %2835 = load i64, ptr %2834, , !!8                                                                                    ;L1513<1043
 31246|  br label %2843                                                                                                        ;L1512<1043
 31247| 
 31248| 2836: ; preds = %2820
 31249|  %2837 = sext i32 %2822 to i64                                                                                         ;L1511<1043
 31250|     ;; mult = i64 %2837
 31251|  %2838 = gep %2784, i64 1664                                                                                           ;L1515<1043
 31252|  %2839 = load i64, ptr %2838, , !!8                                                                                    ;L1515<1043
 31253|  %2840 = add nsw i64 %2837, 100                                                                                        ;L1515<1043
 31254|  %2841 = mul i64 %2839, %2840                                                                                          ;L1515<1043
 31255|  %2842 = udiv i64 %2841, 100                                                                                           ;L1515<1043
 31256|  br label %2843                                                                                                        ;L1512<1043
 31257| 
 31258| 2843: ; preds = %2836, %2833
 31259|  %2844 = phi i64 [ %2835, %2833 ], [ %2842, %2836 ]                                                                    ;L0<1043
 31260|  %2845 = add i64 %2844, 80000                                                                                          ;L1043
 31261|  %2846 = load i32, ptr %342, , !!8                                                                                     ;L1511<1043
 31262|     ;; mult = i32 %2846
 31263|  %2847 = icmp eq i32 %2846, 0                                                                                          ;L1512<1043
 31264|  br i1 %2847, label %2848, label %2850                                                                                 ;L1512<1043
 31265| 
 31266| 2848: ; preds = %2843
 31267|  %2849 = load i64, ptr %343, , !!8                                                                                     ;L1513<1043
 31268|  br label %2856                                                                                                        ;L1512<1043
 31269| 
 31270| 2850: ; preds = %2843
 31271|  %2851 = sext i32 %2846 to i64                                                                                         ;L1511<1043
 31272|     ;; mult = i64 %2851
 31273|  %2852 = load i64, ptr %343, , !!8                                                                                     ;L1515<1043
 31274|  %2853 = add nsw i64 %2851, 100                                                                                        ;L1515<1043
 31275|  %2854 = mul i64 %2852, %2853                                                                                          ;L1515<1043
 31276|  %2855 = udiv i64 %2854, 100                                                                                           ;L1515<1043
 31277|  br label %2856                                                                                                        ;L1512<1043
 31278| 
 31279| 2856: ; preds = %2850, %2848
 31280|  %2857 = phi i64 [ %2849, %2848 ], [ %2855, %2850 ]                                                                    ;L0<1043
 31281|  %2858 = add i64 %2845, %2857                                                                                          ;L1043
 31282|  %2859 = icmp ugt i64 %2795, %2858                                                                                     ;L1043
 31283|  br i1 %2859, label %2860, label %2864                                                                                 ;L1043
 31284| 
 31285| 2860: ; preds = %2856
 31286|  %2861 = gep %2778, i64 144                                                                                            ;L1045
 31287|  %2862 = load i64, ptr %2861, , !!8                                                                                    ;L1045
 31288|  %2863 = icmp ugt i64 %2782, %2862                                                                                     ;L1045
 31289|  br i1 %2863, label %2866, label %2828                                                                                 ;L1045
 31290| 
 31291| 2864: ; preds = %2856
 31292|     ;; self = i64 0
 31293|     ;; other = i64 %2793
 31294|  %2865 = call i64 @llvm.smax.i64(i64 %2793, i64 0)                                                                     ;L1039<1044
 31295|  br label %2869                                                                                                        ;L1039<1044
 31296| 
 31297| 2866: ; preds = %2860
 31298|  %2867 = sdiv i64 %2793, 2                                                                                             ;L1048
 31299|     ;; self = i64 0
 31300|     ;; other = i64 %2867
 31301|  %2868 = call i64 @llvm.smax.i64(i64 %2867, i64 0)                                                                     ;L1039<1048
 31302|  br label %2869                                                                                                        ;L1039<1048
 31303| 
 31304| 2869: ; preds = %2866, %2864, %2830, %2828, %2824, %2810, %2808, %2804, %2777
 31305|  %2870 = phi i64 [ %2829, %2828 ], [ %2809, %2808 ], [ %2812, %2810 ], [ 0, %2777 ], [ 0, %2804 ], [ %2868, %2866 ], [ %2832, %2830 ], [ 0, %2824 ], [ %2865, %2864 ] ;L0
 31306|     ;; max_ratio = i64 %2870
 31307|  %2871 = gep %2784, i64 1480                                                                                           ;L1693<1058
 31308|  %2872 = load i64, ptr %2871, , !!8                                                                                    ;L1693<1058
 31309|  %2873 = icmp ugt i64 %2872, 2                                                                                         ;L1693<1058
 31310|  %2874 = gep %2784, i64 1280                                                                                           ;L1693<1058
 31311|  %2875 = select i1 %2873, ptr %2874, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                     ;L1693<1058
 31312|     ;; self = ptr %2875
 31313|  %2876 = gep %2875, i64 48                                                                                             ;L633<1058
 31314|  %2877 = load i32, ptr %2876, , !!8                                                                                    ;L633<1058
 31315|  %2878 = icmp eq i32 %2877, -1                                                                                         ;L633<1058
 31316|  br i1 %2878, label %2954, label %2879                                                                                 ;L1058
 31317| 
 31318| 2879: ; preds = %2869
 31319|  %2880 = gep %2778, i64 16                                                                                             ;L1059
 31320|  %2881 = load i64, ptr %2880, , !!8                                                                                    ;L1059
 31321|     ;; ratio = i64 %2881
 31322|  %2882 = gep %2778, i64 72                                                                                             ;L1060
 31323|  %2883 = load i64, ptr %2882, , !!8                                                                                    ;L1060
 31324|     ;; range = i64 %2883
 31325|  %2884 = gep %2778, i64 417                                                                                            ;L1061
 31326|  %2885 = load i8, ptr %2884, , !!8                                                                                     ;L1061
 31327|  %2886 = trunc nuw i8 %2885 to i1                                                                                      ;L1061
 31328|  %2887 = gep %2778, i64 152                                                                                            ;L0
 31329|  %2888 = load i64, ptr %2887, , !!8                                                                                    ;L0
 31330|  %2889 = icmp ugt i64 %2782, %2888                                                                                     ;L0
 31331|  br i1 %2886, label %2891, label %2890                                                                                 ;L1061
 31332| 
 31333| 2890: ; preds = %2879
 31334|  br i1 %2889, label %2892, label %2896                                                                                 ;L1071
 31335| 
 31336| 2891: ; preds = %2879
 31337|  br i1 %2889, label %2901, label %2905                                                                                 ;L1062
 31338| 
 31339| 2892: ; preds = %2890
 31340|  %2893 = gep %2778, i64 160                                                                                            ;L1073
 31341|  %2894 = load i64, ptr %2893, , !!8                                                                                    ;L1073
 31342|  %2895 = icmp ugt i64 %2782, %2894                                                                                     ;L1073
 31343|  br i1 %2895, label %2954, label %2898                                                                                 ;L1073
 31344| 
 31345| 2896: ; preds = %2890
 31346|     ;; self = i64 %2870
 31347|     ;; other = i64 %2881
 31348|  %2897 = call i64 @llvm.smax.i64(i64 %2881, i64 %2870)                                                                 ;L1039<1072
 31349|  br label %2954                                                                                                        ;L1039<1072
 31350| 
 31351| 2898: ; preds = %2892
 31352|  %2899 = sdiv i64 %2881, 2                                                                                             ;L1074
 31353|     ;; self = i64 %2870
 31354|     ;; other = i64 %2899
 31355|  %2900 = call i64 @llvm.smax.i64(i64 %2899, i64 %2870)                                                                 ;L1039<1074
 31356|  br label %2954                                                                                                        ;L1039<1074
 31357| 
 31358| 2901: ; preds = %2891
 31359|  %2902 = gep %2778, i64 168                                                                                            ;L1064
 31360|  %2903 = load i64, ptr %2902, , !!8                                                                                    ;L1064
 31361|  %2904 = icmp ugt i64 %2782, %2903                                                                                     ;L1064
 31362|  br i1 %2904, label %2909, label %2913                                                                                 ;L1064
 31363| 
 31364| 2905: ; preds = %2891
 31365|  %2906 = gep %2784, i64 1136                                                                                           ;L1511<1062
 31366|  %2907 = load i32, ptr %2906, , !!8                                                                                    ;L1511<1062
 31367|     ;; mult = i32 %2907
 31368|  %2908 = icmp eq i32 %2907, 0                                                                                          ;L1512<1062
 31369|  br i1 %2908, label %2918, label %2921                                                                                 ;L1512<1062
 31370| 
 31371| 2909: ; preds = %2901
 31372|  %2910 = gep %2778, i64 160                                                                                            ;L1068
 31373|  %2911 = load i64, ptr %2910, , !!8                                                                                    ;L1068
 31374|  %2912 = icmp ugt i64 %2782, %2911                                                                                     ;L1068
 31375|  br i1 %2912, label %2954, label %2915                                                                                 ;L1068
 31376| 
 31377| 2913: ; preds = %2945, %2901
 31378|     ;; self = i64 %2870
 31379|     ;; other = i64 %2881
 31380|  %2914 = call i64 @llvm.smax.i64(i64 %2881, i64 %2870)                                                                 ;L1039<1065
 31381|  br label %2954                                                                                                        ;L1039<1065
 31382| 
 31383| 2915: ; preds = %2909
 31384|  %2916 = sdiv i64 %2881, 3                                                                                             ;L1069
 31385|     ;; self = i64 %2870
 31386|     ;; other = i64 %2916
 31387|  %2917 = call i64 @llvm.smax.i64(i64 %2916, i64 %2870)                                                                 ;L1039<1069
 31388|  br label %2954                                                                                                        ;L1039<1069
 31389| 
 31390| 2918: ; preds = %2905
 31391|  %2919 = gep %2784, i64 1664                                                                                           ;L1513<1062
 31392|  %2920 = load i64, ptr %2919, , !!8                                                                                    ;L1513<1062
 31393|  br label %2928                                                                                                        ;L1512<1062
 31394| 
 31395| 2921: ; preds = %2905
 31396|  %2922 = sext i32 %2907 to i64                                                                                         ;L1511<1062
 31397|     ;; mult = i64 %2922
 31398|  %2923 = gep %2784, i64 1664                                                                                           ;L1515<1062
 31399|  %2924 = load i64, ptr %2923, , !!8                                                                                    ;L1515<1062
 31400|  %2925 = add nsw i64 %2922, 100                                                                                        ;L1515<1062
 31401|  %2926 = mul i64 %2924, %2925                                                                                          ;L1515<1062
 31402|  %2927 = udiv i64 %2926, 100                                                                                           ;L1515<1062
 31403|  br label %2928                                                                                                        ;L1512<1062
 31404| 
 31405| 2928: ; preds = %2921, %2918
 31406|  %2929 = phi i64 [ %2920, %2918 ], [ %2927, %2921 ]                                                                    ;L0<1062
 31407|  %2930 = add i64 %2929, 80000                                                                                          ;L1062
 31408|  %2931 = load i32, ptr %342, , !!8                                                                                     ;L1511<1062
 31409|     ;; mult = i32 %2931
 31410|  %2932 = icmp eq i32 %2931, 0                                                                                          ;L1512<1062
 31411|  br i1 %2932, label %2933, label %2935                                                                                 ;L1512<1062
 31412| 
 31413| 2933: ; preds = %2928
 31414|  %2934 = load i64, ptr %343, , !!8                                                                                     ;L1513<1062
 31415|  br label %2941                                                                                                        ;L1512<1062
 31416| 
 31417| 2935: ; preds = %2928
 31418|  %2936 = sext i32 %2931 to i64                                                                                         ;L1511<1062
 31419|     ;; mult = i64 %2936
 31420|  %2937 = load i64, ptr %343, , !!8                                                                                     ;L1515<1062
 31421|  %2938 = add nsw i64 %2936, 100                                                                                        ;L1515<1062
 31422|  %2939 = mul i64 %2937, %2938                                                                                          ;L1515<1062
 31423|  %2940 = udiv i64 %2939, 100                                                                                           ;L1515<1062
 31424|  br label %2941                                                                                                        ;L1512<1062
 31425| 
 31426| 2941: ; preds = %2935, %2933
 31427|  %2942 = phi i64 [ %2934, %2933 ], [ %2940, %2935 ]                                                                    ;L0<1062
 31428|  %2943 = add i64 %2930, %2942                                                                                          ;L1062
 31429|  %2944 = icmp ugt i64 %2883, %2943                                                                                     ;L1062
 31430|  br i1 %2944, label %2945, label %2949                                                                                 ;L1062
 31431| 
 31432| 2945: ; preds = %2941
 31433|  %2946 = gep %2778, i64 168                                                                                            ;L1064
 31434|  %2947 = load i64, ptr %2946, , !!8                                                                                    ;L1064
 31435|  %2948 = icmp ugt i64 %2782, %2947                                                                                     ;L1064
 31436|  br i1 %2948, label %2951, label %2913                                                                                 ;L1064
 31437| 
 31438| 2949: ; preds = %2941
 31439|     ;; self = i64 %2870
 31440|     ;; other = i64 %2881
 31441|  %2950 = call i64 @llvm.smax.i64(i64 %2881, i64 %2870)                                                                 ;L1039<1063
 31442|  br label %2954                                                                                                        ;L1039<1063
 31443| 
 31444| 2951: ; preds = %2945
 31445|  %2952 = sdiv i64 %2881, 2                                                                                             ;L1067
 31446|     ;; self = i64 %2870
 31447|     ;; other = i64 %2952
 31448|  %2953 = call i64 @llvm.smax.i64(i64 %2952, i64 %2870)                                                                 ;L1039<1067
 31449|  br label %2954                                                                                                        ;L1039<1067
 31450| 
 31451| 2954: ; preds = %2951, %2949, %2915, %2913, %2909, %2898, %2896, %2892, %2869
 31452|  %2955 = phi i64 [ %2914, %2913 ], [ %2897, %2896 ], [ %2900, %2898 ], [ %2870, %2869 ], [ %2870, %2892 ], [ %2953, %2951 ], [ %2917, %2915 ], [ %2870, %2909 ], [ %2950, %2949 ] ;L0
 31453|     ;; max_ratio = i64 %2955
 31454|  %2956 = icmp ugt i64 %2872, 4                                                                                         ;L1701<1077
 31455|  %2957 = gep %2784, i64 1336                                                                                           ;L1701<1077
 31456|  %2958 = select i1 %2956, ptr %2957, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                     ;L1701<1077
 31457|     ;; self = ptr %2958
 31458|  %2959 = gep %2958, i64 48                                                                                             ;L633<1077
 31459|  %2960 = load i32, ptr %2959, , !!8                                                                                    ;L633<1077
 31460|  %2961 = icmp eq i32 %2960, -1                                                                                         ;L633<1077
 31461|  br i1 %2961, label %3037, label %2962                                                                                 ;L1077
 31462| 
 31463| 2962: ; preds = %2954
 31464|  %2963 = gep %2778, i64 24                                                                                             ;L1078
 31465|  %2964 = load i64, ptr %2963, , !!8                                                                                    ;L1078
 31466|     ;; ratio = i64 %2964
 31467|  %2965 = gep %2778, i64 80                                                                                             ;L1079
 31468|  %2966 = load i64, ptr %2965, , !!8                                                                                    ;L1079
 31469|     ;; range = i64 %2966
 31470|  %2967 = gep %2778, i64 418                                                                                            ;L1080
 31471|  %2968 = load i8, ptr %2967, , !!8                                                                                     ;L1080
 31472|  %2969 = trunc nuw i8 %2968 to i1                                                                                      ;L1080
 31473|  %2970 = gep %2778, i64 176                                                                                            ;L0
 31474|  %2971 = load i64, ptr %2970, , !!8                                                                                    ;L0
 31475|  %2972 = icmp ugt i64 %2782, %2971                                                                                     ;L0
 31476|  br i1 %2969, label %2974, label %2973                                                                                 ;L1080
 31477| 
 31478| 2973: ; preds = %2962
 31479|  br i1 %2972, label %2975, label %2979                                                                                 ;L1090
 31480| 
 31481| 2974: ; preds = %2962
 31482|  br i1 %2972, label %2984, label %2988                                                                                 ;L1081
 31483| 
 31484| 2975: ; preds = %2973
 31485|  %2976 = gep %2778, i64 184                                                                                            ;L1092
 31486|  %2977 = load i64, ptr %2976, , !!8                                                                                    ;L1092
 31487|  %2978 = icmp ugt i64 %2782, %2977                                                                                     ;L1092
 31488|  br i1 %2978, label %3037, label %2981                                                                                 ;L1092
 31489| 
 31490| 2979: ; preds = %2973
 31491|     ;; self = i64 %2955
 31492|     ;; other = i64 %2964
 31493|  %2980 = call i64 @llvm.smax.i64(i64 %2964, i64 %2955)                                                                 ;L1039<1091
 31494|  br label %3037                                                                                                        ;L1039<1091
 31495| 
 31496| 2981: ; preds = %2975
 31497|  %2982 = sdiv i64 %2964, 2                                                                                             ;L1093
 31498|     ;; self = i64 %2955
 31499|     ;; other = i64 %2982
 31500|  %2983 = call i64 @llvm.smax.i64(i64 %2982, i64 %2955)                                                                 ;L1039<1093
 31501|  br label %3037                                                                                                        ;L1039<1093
 31502| 
 31503| 2984: ; preds = %2974
 31504|  %2985 = gep %2778, i64 192                                                                                            ;L1083
 31505|  %2986 = load i64, ptr %2985, , !!8                                                                                    ;L1083
 31506|  %2987 = icmp ugt i64 %2782, %2986                                                                                     ;L1083
 31507|  br i1 %2987, label %2992, label %2996                                                                                 ;L1083
 31508| 
 31509| 2988: ; preds = %2974
 31510|  %2989 = gep %2784, i64 1136                                                                                           ;L1511<1081
 31511|  %2990 = load i32, ptr %2989, , !!8                                                                                    ;L1511<1081
 31512|     ;; mult = i32 %2990
 31513|  %2991 = icmp eq i32 %2990, 0                                                                                          ;L1512<1081
 31514|  br i1 %2991, label %3001, label %3004                                                                                 ;L1512<1081
 31515| 
 31516| 2992: ; preds = %2984
 31517|  %2993 = gep %2778, i64 184                                                                                            ;L1087
 31518|  %2994 = load i64, ptr %2993, , !!8                                                                                    ;L1087
 31519|  %2995 = icmp ugt i64 %2782, %2994                                                                                     ;L1087
 31520|  br i1 %2995, label %3037, label %2998                                                                                 ;L1087
 31521| 
 31522| 2996: ; preds = %3028, %2984
 31523|     ;; self = i64 %2955
 31524|     ;; other = i64 %2964
 31525|  %2997 = call i64 @llvm.smax.i64(i64 %2964, i64 %2955)                                                                 ;L1039<1084
 31526|  br label %3037                                                                                                        ;L1039<1084
 31527| 
 31528| 2998: ; preds = %2992
 31529|  %2999 = sdiv i64 %2964, 3                                                                                             ;L1088
 31530|     ;; self = i64 %2955
 31531|     ;; other = i64 %2999
 31532|  %3000 = call i64 @llvm.smax.i64(i64 %2999, i64 %2955)                                                                 ;L1039<1088
 31533|  br label %3037                                                                                                        ;L1039<1088
 31534| 
 31535| 3001: ; preds = %2988
 31536|  %3002 = gep %2784, i64 1664                                                                                           ;L1513<1081
 31537|  %3003 = load i64, ptr %3002, , !!8                                                                                    ;L1513<1081
 31538|  br label %3011                                                                                                        ;L1512<1081
 31539| 
 31540| 3004: ; preds = %2988
 31541|  %3005 = sext i32 %2990 to i64                                                                                         ;L1511<1081
 31542|     ;; mult = i64 %3005
 31543|  %3006 = gep %2784, i64 1664                                                                                           ;L1515<1081
 31544|  %3007 = load i64, ptr %3006, , !!8                                                                                    ;L1515<1081
 31545|  %3008 = add nsw i64 %3005, 100                                                                                        ;L1515<1081
 31546|  %3009 = mul i64 %3007, %3008                                                                                          ;L1515<1081
 31547|  %3010 = udiv i64 %3009, 100                                                                                           ;L1515<1081
 31548|  br label %3011                                                                                                        ;L1512<1081
 31549| 
 31550| 3011: ; preds = %3004, %3001
 31551|  %3012 = phi i64 [ %3003, %3001 ], [ %3010, %3004 ]                                                                    ;L0<1081
 31552|  %3013 = add i64 %3012, 80000                                                                                          ;L1081
 31553|  %3014 = load i32, ptr %342, , !!8                                                                                     ;L1511<1081
 31554|     ;; mult = i32 %3014
 31555|  %3015 = icmp eq i32 %3014, 0                                                                                          ;L1512<1081
 31556|  br i1 %3015, label %3016, label %3018                                                                                 ;L1512<1081
 31557| 
 31558| 3016: ; preds = %3011
 31559|  %3017 = load i64, ptr %343, , !!8                                                                                     ;L1513<1081
 31560|  br label %3024                                                                                                        ;L1512<1081
 31561| 
 31562| 3018: ; preds = %3011
 31563|  %3019 = sext i32 %3014 to i64                                                                                         ;L1511<1081
 31564|     ;; mult = i64 %3019
 31565|  %3020 = load i64, ptr %343, , !!8                                                                                     ;L1515<1081
 31566|  %3021 = add nsw i64 %3019, 100                                                                                        ;L1515<1081
 31567|  %3022 = mul i64 %3020, %3021                                                                                          ;L1515<1081
 31568|  %3023 = udiv i64 %3022, 100                                                                                           ;L1515<1081
 31569|  br label %3024                                                                                                        ;L1512<1081
 31570| 
 31571| 3024: ; preds = %3018, %3016
 31572|  %3025 = phi i64 [ %3017, %3016 ], [ %3023, %3018 ]                                                                    ;L0<1081
 31573|  %3026 = add i64 %3013, %3025                                                                                          ;L1081
 31574|  %3027 = icmp ugt i64 %2966, %3026                                                                                     ;L1081
 31575|  br i1 %3027, label %3028, label %3032                                                                                 ;L1081
 31576| 
 31577| 3028: ; preds = %3024
 31578|  %3029 = gep %2778, i64 192                                                                                            ;L1083
 31579|  %3030 = load i64, ptr %3029, , !!8                                                                                    ;L1083
 31580|  %3031 = icmp ugt i64 %2782, %3030                                                                                     ;L1083
 31581|  br i1 %3031, label %3034, label %2996                                                                                 ;L1083
 31582| 
 31583| 3032: ; preds = %3024
 31584|     ;; self = i64 %2955
 31585|     ;; other = i64 %2964
 31586|  %3033 = call i64 @llvm.smax.i64(i64 %2964, i64 %2955)                                                                 ;L1039<1082
 31587|  br label %3037                                                                                                        ;L1039<1082
 31588| 
 31589| 3034: ; preds = %3028
 31590|  %3035 = sdiv i64 %2964, 2                                                                                             ;L1086
 31591|     ;; self = i64 %2955
 31592|     ;; other = i64 %3035
 31593|  %3036 = call i64 @llvm.smax.i64(i64 %3035, i64 %2955)                                                                 ;L1039<1086
 31594|  br label %3037                                                                                                        ;L1039<1086
 31595| 
 31596| 3037: ; preds = %3034, %3032, %2998, %2996, %2992, %2981, %2979, %2975, %2954
 31597|  %3038 = phi i64 [ %2997, %2996 ], [ %2980, %2979 ], [ %2983, %2981 ], [ %2955, %2954 ], [ %2955, %2975 ], [ %3036, %3034 ], [ %3000, %2998 ], [ %2955, %2992 ], [ %3033, %3032 ] ;L0
 31598|     ;; max_ratio = i64 %3038
 31599|  %3039 = add i64 %3038, %2779                                                                                          ;L1096
 31600|     ;; score[16..+8] = i64 %3039
 31601|     ;; score[16..+8] = i64 %3039
 31602|     ;; iter[0..+8] = ptr %2780
 31603|     ;; self = ptr undef
 31604|     ;; ptr = ptr %2780
 31605|     ;; self = ptr %2780
 31606|     ;; end_or_len = ptr %2771
 31609|  %3040 = icmp eq ptr %2780, %2771                                                                                      ;L1714<180<1036
 31610|  br i1 %3040, label %2788, label %2777                                                                                 ;L180<1036
 31611| 
 31612| 3041: ; preds = %3153, %3150
 31613|  %3042 = phi ptr [ %3045, %3153 ], [ %3152, %3150 ]                                                                    ;L1103
 31614|     ;; iter[0..+8] = ptr %3042
 31615|     ;; enabled = i64 %3151
 31616|     ;; self = ptr undef
 31617|     ;; ptr = ptr %3042
 31618|     ;; self = ptr %3042
 31619|     ;; end_or_len = ptr %2771
 31622|  %3043 = icmp eq ptr %3042, %2771                                                                                      ;L1714<180<1103
 31623|  br i1 %3043, label %3049, label %3044                                                                                 ;L180<1103
 31624| 
 31625| 3044: ; preds = %3041
 31626|  %3045 = gep %3042, i64 448                                                                                            ;L656<185<1103
 31627|     ;; iter[0..+8] = ptr %3045
 31628|     ;; ap = ptr %3042
 31629|     ;; a = ptr %3042
 31630|  %3046 = gep %3042, i64 424                                                                                            ;L1105
 31631|  %3047 = load ptr, ptr %3046, , !!8, !!8                                                                               ;L1105
 31632|  %3048 = invoke i8 @ai::utils15get_battle_role(i64 %1, ptr %135, ptr %118, ptr %3047)
 31633|  to label %3051 unwind label %2813                                                                                     ;L1105
 31634| 
 31635| 3049: ; preds = %3041
 31636|  %3050 = add i64 %3151, %2789                                                                                          ;L1127
 31637|     ;; score[16..+8] = i64 %3050
 31638|     ;; score[16..+8] = i64 %3050
 31639|  br label %3154                                                                                                        ;L1101
 31640| 
 31641| 3051: ; preds = %3044
 31642|  %3052 = and i8 %3048, 6                                                                                               ;L1105
 31643|  %3053 = icmp eq i8 %3052, 2                                                                                           ;L1105
 31644|  br i1 %3053, label %3054, label %3153                                                                                 ;L1105
 31645| 
 31646| 3054: ; preds = %3051
 31647|  %3055 = gep %3042, i64 432                                                                                            ;L1109
 31648|  %3056 = load ptr, ptr %3055, , !!8, !!8                                                                               ;L1109
 31649|     ;; self = ptr %3056
 31650|  %3057 = gep %3056, i64 1632                                                                                           ;L1109
 31651|  %3058 = load i64, ptr %3057, , !!8                                                                                    ;L1109
 31652|     ;; x1 = i64 %3058
 31653|     ;; self = i64 %3058
 31654|  %3059 = load i64, ptr %100, , !!8                                                                                     ;L1109
 31655|     ;; a = i64 %3059
 31656|     ;; self = i64 %3059
 31657|     ;; b = i64 %3058
 31658|     ;; other = i64 %3058
 31659|  %3060 = icmp ult i64 %3059, %3058                                                                                     ;L3147<8<1109
 31660|  %3061 = sub nuw i64 %3058, %3059                                                                                      ;L3147<8<1109
 31661|  %3062 = sub nuw i64 %3059, %3058                                                                                      ;L3147<8<1109
 31662|  %3063 = select i1 %3060, i64 %3061, i64 %3062                                                                         ;L3147<8<1109
 31663|     ;; rhs = i64 %3063
 31664|     ;; rhs = i64 %3063
 31665|     ;; rhs = i64 %3063
 31666|     ;; diff = i64 %3063
 31667|     ;; self = i64 %3063
 31668|     ;; self = i64 %3063
 31669|     ;; self = i64 %3063
 31670|  %3064 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %3063, i64 %3063)                                           ;L3178<1288<2517<9<1109
 31671|  %3065 = extractvalue { i64, i1 } %3064, 0                                                                             ;L3178<1288<2517<9<1109
 31672|  %3066 = extractvalue { i64, i1 } %3064, 1                                                                             ;L3178<1288<2517<9<1109
 31673|     ;; a = i64 %3065
 31674|     ;; b = i1 %3066
 31675|     ;; b = i1 %3066
 31676|  br i1 %3066, label %3067, label %3068                                                                                 ;L459<1289<2517<9<1109
 31677| 
 31678| 3067: ; preds = %3054
 31679|     ;; a = i64 -1
 31680|  br label %3068                                                                                                        ;L2519<9<1109
 31681| 
 31682| 3068: ; preds = %3067, %3054
 31683|  %3069 = phi i64 [ -1, %3067 ], [ %3065, %3054 ]                                                                       ;L0<9<1109
 31684|     ;; a = i64 %3069
 31685|     ;; self = i64 %3069
 31686|  %3070 = gep %3056, i64 1640                                                                                           ;L1109
 31687|  %3071 = load i64, ptr %3070, , !!8                                                                                    ;L1109
 31688|     ;; y1 = i64 %3071
 31689|     ;; self = i64 %3071
 31690|  %3072 = load i64, ptr %99, , !!8                                                                                      ;L1109
 31691|     ;; a = i64 %3072
 31692|     ;; self = i64 %3072
 31693|     ;; b = i64 %3071
 31694|     ;; other = i64 %3071
 31695|  %3073 = icmp ult i64 %3072, %3071                                                                                     ;L3147<8<1109
 31696|  %3074 = sub nuw i64 %3071, %3072                                                                                      ;L3147<8<1109
 31697|  %3075 = sub nuw i64 %3072, %3071                                                                                      ;L3147<8<1109
 31698|  %3076 = select i1 %3073, i64 %3074, i64 %3075                                                                         ;L3147<8<1109
 31699|     ;; rhs = i64 %3076
 31700|     ;; rhs = i64 %3076
 31701|     ;; rhs = i64 %3076
 31702|     ;; diff = i64 %3076
 31703|     ;; self = i64 %3076
 31704|     ;; self = i64 %3076
 31705|     ;; self = i64 %3076
 31706|  %3077 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %3076, i64 %3076)                                           ;L3178<1288<2517<9<1109
 31707|  %3078 = extractvalue { i64, i1 } %3077, 0                                                                             ;L3178<1288<2517<9<1109
 31708|  %3079 = extractvalue { i64, i1 } %3077, 1                                                                             ;L3178<1288<2517<9<1109
 31709|     ;; a = i64 %3078
 31710|     ;; b = i1 %3079
 31711|     ;; b = i1 %3079
 31712|  br i1 %3079, label %3080, label %3081                                                                                 ;L459<1289<2517<9<1109
 31713| 
 31714| 3080: ; preds = %3068
 31715|     ;; a = i64 -1
 31716|  br label %3081                                                                                                        ;L2519<9<1109
 31717| 
 31718| 3081: ; preds = %3080, %3068
 31719|  %3082 = phi i64 [ -1, %3080 ], [ %3078, %3068 ]                                                                       ;L0<9<1109
 31720|     ;; a = i64 %3082
 31721|     ;; rhs = i64 %3082
 31722|  %3083 = call i64 @llvm.uadd.sat.i64(i64 %3069, i64 %3082)                                                             ;L2428<1109
 31723|  %3084 = icmp ugt i64 %3083, 14400000000                                                                               ;L1109
 31724|  br i1 %3084, label %3153, label %3085                                                                                 ;L1109
 31725| 
 31726| 3085: ; preds = %3081
 31727|     ;; self = ptr %95
 31728|     ;; self = ptr %95
 31729|  %3086 = load ptr, ptr %95, , !!8, !!8                                                                                 ;L138<2073<1113
 31730|     ;; p = ptr %3086
 31731|  %3087 = load i64, ptr %200, , !!8                                                                                     ;L2075<1113
 31732|     ;; len = i64 %3087
 31733|     ;; count = i64 %3087
 31734|     ;; self[0..+8] = ptr %3086
 31735|     ;; slice[0..+8] = ptr %3086
 31736|     ;; self[8..+8] = i64 %3087
 31737|     ;; slice[8..+8] = i64 %3087
 31738|     ;; ptr = ptr %3086
 31739|     ;; self = ptr %3086
 31740|  %3088 = gepS, ptr, i64 }, ptr %3086, i64 %3087                                                                        ;L961<100<1042<1113
 31741|     ;; iter[0..+8] = ptr %3086
 31742|     ;; iter[8..+8] = ptr %3088
 31743|  br label %3089                                                                                                        ;L1113
 31744| 
 31745| 3089: ; preds = %3114, %3085
 31746|  %3090 = phi ptr [ %3086, %3085 ], [ %3093, %3114 ]                                                                    ;L1113
 31747|     ;; iter[0..+8] = ptr %3090
 31748|     ;; self = ptr undef
 31749|     ;; ptr = ptr %3090
 31750|     ;; self = ptr %3090
 31751|     ;; end_or_len = ptr %3088
 31754|  %3091 = icmp eq ptr %3090, %3088                                                                                      ;L1714<180<1113
 31755|  br i1 %3091, label %3148, label %3092                                                                                 ;L180<1113
 31756| 
 31757| 3092: ; preds = %3089
 31758|  %3093 = gep %3090, i64 448                                                                                            ;L656<185<1113
 31759|     ;; iter[0..+8] = ptr %3093
 31760|     ;; e = ptr %3090
 31761|     ;; ce_dist = ptr %3090
 31762|  %3094 = gep %3090, i64 432                                                                                            ;L1114
 31763|  %3095 = load ptr, ptr %3094, , !!8, !!8                                                                               ;L1114
 31764|     ;; other = ptr %3095
 31765|  %3096 = gep %3095, i64 1632                                                                                           ;L2158<1114
 31766|  %3097 = load i64, ptr %3096, , !!8                                                                                    ;L2158<1114
 31767|     ;; x2 = i64 %3097
 31768|     ;; other = i64 %3097
 31769|  %3098 = gep %3095, i64 1640                                                                                           ;L2158<1114
 31770|  %3099 = load i64, ptr %3098, , !!8                                                                                    ;L2158<1114
 31771|     ;; y2 = i64 %3099
 31772|     ;; other = i64 %3099
 31773|  %3100 = icmp ult i64 %3058, %3097                                                                                     ;L3147<7<2158<1114
 31774|  %3101 = sub nuw i64 %3097, %3058                                                                                      ;L3147<7<2158<1114
 31775|  %3102 = sub nuw i64 %3058, %3097                                                                                      ;L3147<7<2158<1114
 31776|  %3103 = select i1 %3100, i64 %3101, i64 %3102                                                                         ;L3147<7<2158<1114
 31777|     ;; dx = i64 %3103
 31778|  %3104 = icmp ult i64 %3071, %3099                                                                                     ;L3147<8<2158<1114
 31779|  %3105 = sub nuw i64 %3099, %3071                                                                                      ;L3147<8<2158<1114
 31780|  %3106 = sub nuw i64 %3071, %3099                                                                                      ;L3147<8<2158<1114
 31781|  %3107 = select i1 %3104, i64 %3105, i64 %3106                                                                         ;L3147<8<2158<1114
 31782|     ;; dy = i64 %3107
 31783|  %3108 = mul i64 %3103, %3103                                                                                          ;L9<2158<1114
 31784|  %3109 = mul i64 %3107, %3107                                                                                          ;L9<2158<1114
 31785|  %3110 = add i64 %3109, %3108                                                                                          ;L9<2158<1114
 31786|     ;; a_to_e_sq = i64 %3110
 31787|  %3111 = gep %3090, i64 440                                                                                            ;L1115
 31788|  %3112 = load i64, ptr %3111, , !!8                                                                                    ;L1115
 31789|     ;; cell_to_e_sq = i64 %3112
 31790|  %3113 = icmp ult i64 %3112, %3110                                                                                     ;L1116
 31791|  br i1 %3113, label %3115, label %3114                                                                                 ;L1116
 31792| 
 31793| 3114: ; preds = %3130, %3092
 31794|  br label %3089                                                                                                        ;L1113
 31795| 
 31796| 3115: ; preds = %3092
 31797|  %3116 = load i64, ptr %100, , !!8                                                                                     ;L1117
 31798|  %3117 = load i64, ptr %99, , !!8                                                                                      ;L1117
 31799|  %3118 = invoke i64 @gc::utils20dist_to_line_segment(i64 %3116, i64 %3117, i64 %3058, i64 %3071, i64 %3097, i64 %3099)
 31800|  to label %3119 unwind label %2813                                                                                     ;L1117
 31801| 
 31802| 3119: ; preds = %3115
 31803|  %3120 = load i32, ptr %342, , !!8                                                                                     ;L1511<1118
 31804|     ;; mult = i32 %3120
 31805|  %3121 = icmp eq i32 %3120, 0                                                                                          ;L1512<1118
 31806|  br i1 %3121, label %3122, label %3124                                                                                 ;L1512<1118
 31807| 
 31808| 3122: ; preds = %3119
 31809|  %3123 = load i64, ptr %343, , !!8                                                                                     ;L1513<1118
 31810|  br label %3130                                                                                                        ;L1512<1118
 31811| 
 31812| 3124: ; preds = %3119
 31813|  %3125 = sext i32 %3120 to i64                                                                                         ;L1511<1118
 31814|     ;; mult = i64 %3125
 31815|  %3126 = load i64, ptr %343, , !!8                                                                                     ;L1515<1118
 31816|  %3127 = add nsw i64 %3125, 100                                                                                        ;L1515<1118
 31817|  %3128 = mul i64 %3126, %3127                                                                                          ;L1515<1118
 31818|  %3129 = udiv i64 %3128, 100                                                                                           ;L1515<1118
 31819|  br label %3130                                                                                                        ;L1512<1118
 31820| 
 31821| 3130: ; preds = %3124, %3122
 31822|  %3131 = phi i64 [ %3123, %3122 ], [ %3129, %3124 ]                                                                    ;L0<1118
 31823|  %3132 = add i64 %3131, 28000                                                                                          ;L1118
 31824|  %3133 = icmp ugt i64 %3118, %3132                                                                                     ;L1117
 31825|  br i1 %3133, label %3114, label %3134                                                                                 ;L1117
 31826| 
 31827| 3134: ; preds = %3130
 31828|     ;; self = ptr %3056
 31829|  %3135 = gep %3056, i64 1216                                                                                           ;L742<1119
 31830|  %3136 = load i32, ptr %3135, , !!8                                                                                    ;L742<1119
 31831|  %3137 = icmp eq i32 %3136, -1                                                                                         ;L742<1119
 31832|  br i1 %3137, label %3148, label %3138                                                                                 ;L742<1119
 31833| 
 31834| 3138: ; preds = %3134
 31835|  %3139 = gep %3056, i64 1168                                                                                           ;L742<1119
 31836|     ;; atk = ptr %3139
 31837|  %3140 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %3139, ptr %135, ptr %3056, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %3095)
 31838|  to label %3141 unwind label %2813                                                                                     ;L1120
 31839| 
 31840| 3141: ; preds = %3138
 31841|     ;; dmg = i64 %3140
 31842|  %3142 = mul i64 %3140, 100                                                                                            ;L1121
 31843|  %3143 = gep %3095, i64 1576                                                                                           ;L1121
 31844|  %3144 = load i64, ptr %3143, , !!8                                                                                    ;L1121
 31845|     ;; self = i64 %3144
 31846|     ;; other = i64 1
 31847|  %3145 = call i64 @llvm.umax.i64(i64 %3144, i64 1)                                                                     ;L1039<1121
 31848|  %3146 = udiv i64 %3142, %3145                                                                                         ;L1121
 31849|  %3147 = add i64 %3146, %3151                                                                                          ;L1121
 31850|     ;; enabled = i64 %3147
 31851|  br label %3148                                                                                                        ;L1119
 31852| 
 31853| 3148: ; preds = %3141, %3134, %3089
 31854|  %3149 = phi i64 [ %3151, %3134 ], [ %3147, %3141 ], [ %3151, %3089 ]                                                  ;L0
 31855|     ;; enabled = i64 %3149
 31856|  br label %3150                                                                                                        ;L1103
 31857| 
 31858| 3150: ; preds = %3148, %2788
 31859|  %3151 = phi i64 [ %3149, %3148 ], [ 0, %2788 ]
 31860|  %3152 = phi ptr [ %3045, %3148 ], [ %2768, %2788 ]
 31861|  br label %3041                                                                                                        ;L180<1103
 31862| 
 31863| 3153: ; preds = %3081, %3051
 31864|  br label %3041                                                                                                        ;L1
 31865| 
 31866| 3154: ; preds = %3049, %2788
 31867|  %3155 = phi i64 [ %3050, %3049 ], [ %2789, %2788 ]                                                                    ;L0
 31868|     ;; score[16..+8] = i64 %3155
 31869|     ;; score[16..+8] = i64 %3155
 31870|  %3156 = sub i64 %2709, %552                                                                                           ;L1134
 31871|     ;; self = i64 %3156
 31872|     ;; other = i64 0
 31873|  %3157 = call i64 @llvm.smax.i64(i64 %3156, i64 0)                                                                     ;L1039<1134
 31874|     ;; non_tower_risk = i64 %3157
 31875|  %3158 = sub nsw i64 %1782, %3157                                                                                      ;L1135
 31876|     ;; trade_net = i64 %3158
 31877|  %3159 = call i64 @llvm.smax.i64(i64 %3158, i64 0)                                                                     ;L1136
 31878|  %3160 = add i64 %3155, %3159                                                                                          ;L1136
 31879|     ;; score[16..+8] = i64 %3160
 31880|     ;; score[16..+8] = i64 %3160
 31882|  call void @llvm.memcpy.p0.p0.i64(ptr %72, ptr %73, i64 24, i1 false)                                                  ;L1141
 31885|  %3161 = gep %72, i64 16                                                                                               ;L825<1004<1141
 31886|  %3162 = load i32, ptr %3161, , !!8                                                                                    ;L825<1004<1141
 31887|  %3163 = icmp eq i32 %3162, -1                                                                                         ;L825<1004<1141
 31888|  br i1 %3163, label %3183, label %3164                                                                                 ;L825<1004<1141
 31889| 
 31890| 3164: ; preds = %3154
 31894|     ;; self = ptr %72
 31895|     ;; order = i8 0
 31896|     ;; order = i8 0
 31897|     ;; val = i64 1
 31898|     ;; order = i8 0
 31899|     ;; val = i64 1
 31900|     ;; order = i8 0
 31901|  %3165 = load i64, ptr %72, , !!8                                                                                      ;L185<825<825<1004<1141
 31902|  %3166 = icmp ult i64 %3165, 132                                                                                       ;L185<825<825<1004<1141
 31903|  br i1 %3166, label %3169, label %3167                                                                                 ;L185<825<825<1004<1141
 31904| 
 31905| 3167: ; preds = %3164
 31906|  invoke void @core::panicking18panic_bounds_check(i64 %3165, i64 132, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.208) #25
 31907|  to label %3168 unwind label %2813                                                                                     ;L185<825<825<1004<1141
 31908| 
 31909| 3168: ; preds = %3167
 31910|  unreachable                                                                                                           ;L185<825<825<1004<1141
 31911| 
 31912| 3169: ; preds = %3164
 31913|     ;; self = !DIArgList(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 %3165)
 31914|  %3170 = gep %72, i64 8                                                                                                ;L185<825<825<1004<1141
 31915|  %3171 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %3170)
 31916|  to label %3172 unwind label %2813                                                                                     ;L185<825<825<1004<1141
 31917| 
 31918| 3172: ; preds = %3169
 31919|  %3173 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %3165                               ;L185<825<825<1004<1141
 31920|     ;; self = ptr %3173
 31921|  %3174 = extractvalue { i64, i32 } %3171, 0                                                                            ;L185<825<825<1004<1141
 31922|  %3175 = extractvalue { i64, i32 } %3171, 1                                                                            ;L185<825<825<1004<1141
 31924|  %3176 = mul i64 %3174, 1000000000                                                                                     ;L632<185<825<825<1004<1141
 31925|  %3177 = icmp ult i32 %3175, 1000000000                                                                                ;L49<632<185<825<825<1004<1141
 31926|  call void @llvm.assume(i1 %3177)                                                                                      ;L49<632<185<825<825<1004<1141
 31927|  %3178 = zext nneg i32 %3175 to i64                                                                                    ;L632<185<825<825<1004<1141
 31928|  %3179 = add i64 %3176, %3178                                                                                          ;L632<185<825<825<1004<1141
 31929|     ;; val = i64 %3179
 31930|     ;; val = i64 %3179
 31931|     ;; dst = ptr %3173
 31932|  %3180 = atomicrmw add ptr %3173, i64 %3179 monotonic, , !!46311                                                       ;L3937<3162<185<825<825<1004<1141
 31933|  %3181 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %3165                               ;L186<825<825<1004<1141
 31934|     ;; self = ptr %3181
 31935|     ;; dst = ptr %3181
 31936|  %3182 = atomicrmw add ptr %3181, i64 1 monotonic, , !!46311                                                           ;L3937<3162<186<825<825<1004<1141
 31937|  br label %3183                                                                                                        ;L825<1004<1141
 31938| 
 31939| 3183: ; preds = %3172, %3154
 31941|     ;; purpose = i8 %6
 31942|     ;; _version = i64 %1
 31947|  %3184 = icmp ne i8 %6, 9                                                                                              ;L31<1142
 31948|  call void @llvm.assume(i1 %3184)                                                                                      ;L31<1142
 31949|  %3185 = add nsw i8 %6, -2                                                                                             ;L31<1142
 31950|  %3186 = icmp samesign ugt i8 %6, 1                                                                                    ;L31<1142
 31951|  %3187 = select i1 %3186, i8 %3185, i8 7                                                                               ;L31<1142
 31952|  switch i8 %3187, label %3201 [
 31953|  i8 7, label %3188
 31954|  i8 8, label %3190
 31955|  ]                                                                                                                     ;L31<1142
 31956| 
 31957| 3188: ; preds = %3183
 31958|  %3189 = trunc nuw i8 %6 to i1                                                                                         ;L31<1142
 31959|  br i1 %3189, label %3195, label %3198                                                                                 ;L31<1142
 31960| 
 31961| 3190: ; preds = %3183
 31962|  %3191 = mul i64 %2709, 3                                                                                              ;L39<1142
 31963|  %3192 = sdiv i64 %3191, 2                                                                                             ;L39<1142
 31964|     ;; score[0..+8] = i64 %3192
 31965|     ;; score[0..+8] = i64 %3192
 31966|  %3193 = mul i64 %550, 3                                                                                               ;L40<1142
 31967|  %3194 = sdiv i64 %3193, 2                                                                                             ;L40<1142
 31968|     ;; score[8..+8] = i64 %3194
 31969|     ;; score[8..+8] = i64 %3194
 31970|  br label %3201                                                                                                        ;L38<1142
 31971| 
 31972| 3195: ; preds = %3188
 31973|  %3196 = mul i64 %2709, 120                                                                                            ;L33<1142
 31974|  %3197 = sdiv i64 %3196, 100                                                                                           ;L33<1142
 31975|     ;; score[0..+8] = i64 %3197
 31976|     ;; score[0..+8] = i64 %3197
 31977|  br label %3201                                                                                                        ;L32<1142
 31978| 
 31979| 3198: ; preds = %3188
 31980|  %3199 = mul i64 %3160, 120                                                                                            ;L36<1142
 31981|  %3200 = sdiv i64 %3199, 100                                                                                           ;L36<1142
 31982|     ;; score[16..+8] = i64 %3200
 31983|     ;; score[16..+8] = i64 %3200
 31984|  br label %3201                                                                                                        ;L35<1142
 31985| 
 31986| 3201: ; preds = %3198, %3195, %3190, %3183
 31987|  %3202 = phi i64 [ %3160, %3183 ], [ %3160, %3195 ], [ %3200, %3198 ], [ %3160, %3190 ]                                ;L0
 31988|  %3203 = phi i64 [ %550, %3183 ], [ %550, %3195 ], [ %550, %3198 ], [ %3194, %3190 ]                                   ;L0
 31989|  %3204 = phi i64 [ %2709, %3183 ], [ %3197, %3195 ], [ %2709, %3198 ], [ %3192, %3190 ]                                ;L0
 31990|     ;; score[0..+8] = i64 %3204
 31991|     ;; score[0..+8] = i64 %3204
 31992|     ;; score[8..+8] = i64 %3203
 31993|     ;; score[8..+8] = i64 %3203
 31994|     ;; score[16..+8] = i64 %3202
 31995|     ;; score[16..+8] = i64 %3202
 31996|  store i64 %3204, ptr %0,                                                                                              ;L44<1142
 31997|  %3205 = gep %0, i64 8                                                                                                 ;L44<1142
 31998|  store i64 %3203, ptr %3205,                                                                                           ;L44<1142
 31999|  %3206 = gep %0, i64 16                                                                                                ;L44<1142
 32000|  store i64 %3202, ptr %3206,                                                                                           ;L44<1142
 32001|  %3207 = gep %0, i64 24                                                                                                ;L44<1142
 32002|  store i64 %1782, ptr %3207,                                                                                           ;L44<1142
 32003|  %3208 = gep %0, i64 32                                                                                                ;L44<1142
 32004|  store i64 0, ptr %3208,                                                                                               ;L44<1142
 32005|  %3209 = gep %0, i64 40                                                                                                ;L44<1142
 32006|  store i64 %1996, ptr %3209,                                                                                           ;L44<1142
 32007|  %3210 = gep %0, i64 48                                                                                                ;L44<1142
 32008|  store i8 %1995, ptr %3210,                                                                                            ;L44<1142
 32009|  %3211 = gep %0, i64 49                                                                                                ;L44<1142
 32010|  store i8 %1260, ptr %3211,                                                                                            ;L44<1142
 32011|  %3212 = gep %2, i64 384                                                                                               ;L1148
 32012|  %3213 = invoke i64 @gc::simulation5state6playerNtB4_16AthleteParameter20positioning_accuracy(ptr %3212)
 32013|  to label %3214 unwind label %2813                                                                                     ;L1148
 32014| 
 32015| 3214: ; preds = %3201
 32016|     ;; acc = i64 %3213
 32017|  %3215 = icmp ugt i64 %1, 1                                                                                            ;L1152
 32018|  br i1 %3215, label %3216, label %3221                                                                                 ;L1152
 32019| 
 32020| 3216: ; preds = %3214
 32021|  %3217 = add nsw i8 %3187, -3                                                                                          ;L1152
 32022|  %3218 = icmp ult i8 %3217, -2                                                                                         ;L1152
 32023|  %3219 = icmp slt i64 %3213, 1000
 32024|  %3220 = and i1 %3218, %3219                                                                                           ;L1152
 32025|  br i1 %3220, label %3228, label %3223                                                                                 ;L1152
 32026| 
 32027| 3221: ; preds = %3214
 32028|  %3222 = icmp slt i64 %3213, 1000                                                                                      ;L1153
 32029|  br i1 %3222, label %3228, label %3223                                                                                 ;L1153
 32030| 
 32031| 3223: ; preds = %3258, %3228, %3221, %3216
 32038|  invoke void @ai::score_parameter22EntityPositioningCacheRNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player11PlayerStateRNtNtB29_6entity6EntityyEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB11_(ptr %93)
 32039|  to label %3227 unwind label %3224                                                                                     ;L825<1181
 32040| 
 32041| 3224: ; preds = %3223
 32042|  %3225 = cleanuppad within none []
 32044|  invoke void @ai::score_parameter22EntityPositioningCacheRNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player11PlayerStateRNtNtB2g_6entity6EntityyEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB18_(ptr %93) [ "funclet"(token %3225) ]
 32045|  to label %3226 unwind label %3278                                                                                     ;L825<825<1181
 32046| 
 32047| 3226: ; preds = %3224
 32048|  cleanupret from %3225 unwind label %3278
 32049| 
 32050| 3227: ; preds = %3223
 32052|  invoke void @ai::score_parameter22EntityPositioningCacheRNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player11PlayerStateRNtNtB2g_6entity6EntityyEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB18_(ptr %93)
 32053|  to label %3281 unwind label %3278                                                                                     ;L825<825<1181
 32054| 
 32055| 3228: ; preds = %3221, %3216
 32056|  %3229 = sub i64 %3204, %3203                                                                                          ;L1154
 32057|     ;; base_risk = i64 %3229
 32058|  %3230 = icmp eq i64 %3204, %3203                                                                                      ;L1155
 32059|  br i1 %3230, label %3223, label %3231                                                                                 ;L1155
 32060| 
 32061| 3231: ; preds = %3228
 32062|  %3232 = gep %153, i64 64                                                                                              ;L1157
 32063|  %3233 = load ptr, ptr %3232, , !!8                                                                                    ;L1157
 32064|  %3234 = invoke { i64, ptr } %3233(ptr %151)
 32065|  to label %3235 unwind label %2813                                                                                     ;L1157
 32066| 
 32067| 3235: ; preds = %3231
 32068|  %3236 = extractvalue { i64, ptr } %3234, 0                                                                            ;L1157
 32069|  %3237 = icmp eq i64 %3236, 2                                                                                          ;L1157
 32070|  br i1 %3237, label %3238, label %3240                                                                                 ;L1157
 32071| 
 32072| 3238: ; preds = %3235
 32073|  %3239 = sub i64 1000, %3213                                                                                           ;L1158
 32074|     ;; spread = i64 %3239
 32075|  br label %3244                                                                                                        ;L1157
 32076| 
 32077| 3240: ; preds = %3235
 32078|  %3241 = shl i64 %3213, 1                                                                                              ;L1160
 32079|  %3242 = sub i64 2000, %3241                                                                                           ;L1160
 32080|  %3243 = sdiv i64 %3242, 3                                                                                             ;L1160
 32081|     ;; spread = i64 %3243
 32082|  br label %3244                                                                                                        ;L1157
 32083| 
 32084| 3244: ; preds = %3240, %3238
 32085|  %3245 = phi i64 [ %3239, %3238 ], [ %3243, %3240 ]                                                                    ;L0
 32086|     ;; spread = i64 %3245
 32087|  br i1 %3215, label %3248, label %3246                                                                                 ;L1165
 32088| 
 32089| 3246: ; preds = %3244
 32090|  %3247 = invoke i64 %382(ptr %151)
 32091|  to label %3258 unwind label %2813                                                                                     ;L1168
 32092| 
 32093| 3248: ; preds = %3244
 32094|  %3249 = invoke i64 %382(ptr %151)
 32095|  to label %3250 unwind label %2813                                                                                     ;L1166
 32096| 
 32097| 3250: ; preds = %3248
 32098|  %3251 = gep %135, i64 8                                                                                               ;L1166
 32099|  %3252 = load ptr, ptr %3251, , !!8, !!8                                                                               ;L1166
 32100|  %3253 = gep %3252, i64 4856                                                                                           ;L1166
 32101|  %3254 = load i64, ptr %3253, , !!8                                                                                    ;L1166
 32102|  %3255 = mul i64 %3254, 6                                                                                              ;L1166
 32103|     ;; self = i64 %3255
 32104|     ;; other = i64 1
 32105|  %3256 = call i64 @llvm.umax.i64(i64 %3255, i64 1)                                                                     ;L1039<1166
 32106|  %3257 = udiv i64 %3249, %3256                                                                                         ;L1166
 32107|     ;; t_salt = i64 %3257
 32108|  br label %3258                                                                                                        ;L1165
 32109| 
 32110| 3258: ; preds = %3250, %3246
 32111|  %3259 = phi i64 [ %3257, %3250 ], [ %3247, %3246 ]                                                                    ;L0
 32112|     ;; t_salt = i64 %3259
 32117|  %3260 = shl i64 %3245, 1                                                                                              ;L1175
 32118|  %3261 = or disjoint i64 %3260, 1                                                                                      ;L1175
 32119|  %3262 = gep %2, i64 2344                                                                                              ;L1170
 32120|  %3263 = load i64, ptr %3262, , !!8                                                                                    ;L1170
 32121|     ;; h = !DIArgList(i64 %3263, i64 %3263, i64 %132, i64 %132, i64 %130, i64 %130, i64 %3259, i64 %3259)
 32122|     ;; self = !DIArgList(i64 %3263, i64 %132, i64 %130, i64 %3259)
 32123|     ;; self = !DIArgList(i64 %3263, i64 %130, i64 %3259)
 32124|     ;; self = !DIArgList(i64 %3263, i64 %3259)
 32125|  %3264 = xor i64 %3263, %3259                                                                                          ;L1171
 32126|     ;; h = !DIArgList(i64 %3264, i64 %3264, i64 %132, i64 %132, i64 %130, i64 %130)
 32127|     ;; self = !DIArgList(i64 %3264, i64 %132, i64 %130)
 32128|     ;; self = !DIArgList(i64 %3264, i64 %130)
 32129|     ;; self = i64 %3264
 32130|  %3265 = mul i64 %3264, -7046029254386353131                                                                           ;L2660<1171
 32131|     ;; h = !DIArgList(i64 %3265, i64 %3265, i64 %132, i64 %132, i64 %130, i64 %130)
 32132|     ;; self = !DIArgList(i64 %3265, i64 %132, i64 %130)
 32133|     ;; self = !DIArgList(i64 %3265, i64 %130)
 32134|  %3266 = xor i64 %3265, %130                                                                                           ;L1172
 32135|     ;; h = !DIArgList(i64 %3266, i64 %3266, i64 %132, i64 %132)
 32136|     ;; self = !DIArgList(i64 %3266, i64 %132)
 32137|     ;; self = i64 %3266
 32138|  %3267 = mul i64 %3266, -7046029254386353131                                                                           ;L2660<1172
 32139|     ;; h = !DIArgList(i64 %3267, i64 %3267, i64 %132, i64 %132)
 32140|     ;; self = !DIArgList(i64 %3267, i64 %132)
 32141|  %3268 = xor i64 %3267, %132                                                                                           ;L1173
 32142|     ;; h = !DIArgList(i64 %3268, i64 %3268)
 32143|     ;; self = i64 %3268
 32144|  %3269 = mul i64 %3268, -7046029254386353131                                                                           ;L2660<1173
 32145|     ;; h = !DIArgList(i64 %3269, i64 %3269)
 32146|  %3270 = lshr i64 %3269, 31                                                                                            ;L1174
 32147|     ;; h = !DIArgList(i64 %3269, i64 %3270)
 32148|  %3271 = xor i64 %3270, %3269                                                                                          ;L1174
 32149|     ;; h = i64 %3271
 32150|  %3272 = urem i64 %3271, %3261                                                                                         ;L1175
 32151|  %3273 = sub i64 %3272, %3245                                                                                          ;L1175
 32152|  %3274 = add i64 %3273, 1000                                                                                           ;L1175
 32153|     ;; factor = i64 %3274
 32154|  %3275 = mul i64 %3274, %3229                                                                                          ;L1176
 32155|  %3276 = sdiv i64 %3275, 1000                                                                                          ;L1176
 32156|  %3277 = add i64 %3276, %3203                                                                                          ;L1176
 32157|  store i64 %3277, ptr %0,                                                                                              ;L1176
 32158|  br label %3223                                                                                                        ;L1155
 32159| 
 32160| 3278: ; preds = %4086, %4078, %4074, %4063, %4062, %4061, %4048, %3227, %3226, %3224, %280
 32161|  %3279 = phi i1 [ true, %4086 ], [ true, %4078 ], [ true, %4048 ], [ true, %4062 ], [ true, %4061 ], [ %281, %280 ], [ false, %3227 ], [ false, %3224 ], [ false, %3226 ], [ true, %4063 ], [ true, %4074 ] ;L0
 32162|  %3280 = cleanuppad within none []
 32163|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecTNtNtCshdEBA0ozCnw_7game_ai15score_parameter22EntityPositioningCacheRNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player11PlayerStateRNtNtB2C_6entity6EntityyEEEB1u_(ptr %95) #27 [ "funclet"(token %3280) ] ;L1181
 32164|  cleanupret from %3280 unwind label %3286                                                                              ;L1181
 32165| 
 32166| 3281: ; preds = %3227
 32169|  invoke void @ai::score_parameter22EntityPositioningCacheRNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player11PlayerStateRNtNtB29_6entity6EntityyEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB11_(ptr %95)
 32170|  to label %3285 unwind label %3282                                                                                     ;L825<1181
 32171| 
 32172| 3282: ; preds = %3281
 32173|  %3283 = cleanuppad within none []
 32175|  invoke void @ai::score_parameter22EntityPositioningCacheRNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player11PlayerStateRNtNtB2g_6entity6EntityyEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB18_(ptr %95) [ "funclet"(token %3283) ]
 32176|  to label %3284 unwind label %3286                                                                                     ;L825<825<1181
 32177| 
 32178| 3284: ; preds = %3282
 32179|  cleanupret from %3283 unwind label %3286
 32180| 
 32181| 3285: ; preds = %3281
 32183|  invoke void @ai::score_parameter22EntityPositioningCacheRNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player11PlayerStateRNtNtB2g_6entity6EntityyEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropB18_(ptr %95)
 32184|  to label %3289 unwind label %3286                                                                                     ;L825<825<1181
 32185| 
 32186| 3286: ; preds = %3285, %3284, %3282, %3278
 32187|  %3287 = phi i1 [ %3279, %3278 ], [ false, %3285 ], [ false, %3284 ], [ false, %3282 ]                                 ;L0
 32188|  %3288 = cleanuppad within none []
 32189|  br i1 %3287, label %4101, label %4100                                                                                 ;L1181
 32190| 
 32191| 3289: ; preds = %3285
 32197|  %3290 = load i32, ptr %110, , !!8                                                                                     ;L825<1181
 32198|  %3291 = icmp eq i32 %3290, -1                                                                                         ;L825<1181
 32199|  br i1 %3291, label %3311, label %3292                                                                                 ;L825<1181
 32200| 
 32201| 3292: ; preds = %3289
 32205|     ;; self = ptr %98
 32206|     ;; order = i8 0
 32207|     ;; order = i8 0
 32208|     ;; val = i64 1
 32209|     ;; order = i8 0
 32210|     ;; val = i64 1
 32211|     ;; order = i8 0
 32212|  %3293 = load i64, ptr %98, , !!8                                                                                      ;L185<825<825<1181
 32213|  %3294 = icmp ult i64 %3293, 132                                                                                       ;L185<825<825<1181
 32214|  br i1 %3294, label %3296, label %3295                                                                                 ;L185<825<825<1181
 32215| 
 32216| 3295: ; preds = %3292
 32217|  call void @core::panicking18panic_bounds_check(i64 %3293, i64 132, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.208) #25, !!46407 ;L185<825<825<1181
 32218|  unreachable                                                                                                           ;L185<825<825<1181
 32219| 
 32220| 3296: ; preds = %3292
 32221|  %3297 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %3293                               ;L185<825<825<1181
 32222|     ;; self = ptr %3297
 32223|  %3298 = gep %98, i64 8                                                                                                ;L185<825<825<1181
 32224|  %3299 = call { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %3298)                             ;L185<825<825<1181
 32225|  %3300 = extractvalue { i64, i32 } %3299, 0                                                                            ;L185<825<825<1181
 32226|  %3301 = extractvalue { i64, i32 } %3299, 1                                                                            ;L185<825<825<1181
 32228|  %3302 = mul i64 %3300, 1000000000                                                                                     ;L632<185<825<825<1181
 32229|  %3303 = icmp ult i32 %3301, 1000000000                                                                                ;L49<632<185<825<825<1181
 32230|  call void @llvm.assume(i1 %3303)                                                                                      ;L49<632<185<825<825<1181
 32231|  %3304 = zext nneg i32 %3301 to i64                                                                                    ;L632<185<825<825<1181
 32232|  %3305 = add i64 %3302, %3304                                                                                          ;L632<185<825<825<1181
 32233|     ;; val = i64 %3305
 32234|     ;; val = i64 %3305
 32235|     ;; dst = ptr %3297
 32236|  %3306 = atomicrmw add ptr %3297, i64 %3305 monotonic, , !!46407                                                       ;L3937<3162<185<825<825<1181
 32237|  %3307 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %3293                               ;L186<825<825<1181
 32238|     ;; self = ptr %3307
 32239|     ;; dst = ptr %3307
 32240|  br label %3308                                                                                                        ;L825<1181
 32241| 
 32242| 3308: ; preds = %4104, %3296
 32243|  %3309 = phi ptr [ %3307, %3296 ], [ getelementptr (i8, ptr @gc::simulation4prof11PHASE_CALLS, i64 384), %4104 ]
 32244|  %3310 = atomicrmw add ptr %3309, i64 1 monotonic, , !!8                                                               ;L3937<3162<186<825<825<1181
 32245|  br label %3311                                                                                                        ;L1181
 32246| 
 32247| 3311: ; preds = %4102, %3308, %3289
 32249|  ret void                                                                                                              ;L1181
 32250| 
 32251| 3312: ; preds = %2813
 32252|  cleanupret from %2815 unwind label %2015
 32253| 
 32254| 3313: ; preds = %2813
 32255|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %73) #27 [ "funclet"(token %2815) ] ;L1181
 32256|  cleanupret from %2815 unwind label %2015                                                                              ;L1181
 32257| 
 32258| 3314: ; preds = %2015
 32259|  cleanupret from %2016 unwind label %1060
 32260| 
 32261| 3315: ; preds = %2015
 32262|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %75) #27 [ "funclet"(token %2016) ] ;L1181
 32263|  cleanupret from %2016 unwind label %1060                                                                              ;L1181
 32264| 
 32265| 3316: ; preds = %1268
 32266|  %3317 = gep %1265, i64 305                                                                                            ;L641
 32267|  %3318 = load i8, ptr %3317, , !!8                                                                                     ;L641
 32268|  %3319 = trunc nuw i8 %3318 to i1                                                                                      ;L641
 32269|  br i1 %3319, label %3325, label %3320                                                                                 ;L641
 32270| 
 32271| 3320: ; preds = %3316
 32272|     ;; self = ptr %1265
 32273|     ;; other = ptr %122
 32274|  %3321 = load i64, ptr %1265, , !!8                                                                                    ;L1127<641
 32275|  %3322 = gep %1265, i64 8                                                                                              ;L1127<641
 32276|     ;; __self_discr = i64 %3321
 32277|  %3323 = load i64, ptr %122, , !!8                                                                                     ;L1127<641
 32278|     ;; __arg1_discr = i64 %3323
 32279|  %3324 = icmp eq i64 %3321, %3323                                                                                      ;L1127<641
 32280|  br i1 %3324, label %3332, label %3477                                                                                 ;L1127<641
 32281| 
 32282| 3325: ; preds = %3342, %3316
 32283|     ;; self = ptr %1265
 32284|  %3326 = gep %1265, i64 64                                                                                             ;L134<642
 32285|  %3327 = load i64, ptr %3326, , !!8                                                                                    ;L134<642
 32286|  %3328 = icmp ne i64 %3327, 9                                                                                          ;L134<642
 32287|  call void @llvm.assume(i1 %3328)                                                                                      ;L134<642
 32288|  %3329 = add nsw i64 %3327, -2                                                                                         ;L134<642
 32289|  %3330 = icmp samesign ugt i64 %3327, 1                                                                                ;L134<642
 32290|  %3331 = select i1 %3330, i64 %3329, i64 7                                                                             ;L134<642
 32291|  switch i64 %3331, label %3349 [
 32292|  i64 4, label %3477
 32293|  i64 5, label %3477
 32294|  i64 7, label %3347
 32295|  ]                                                                                                                     ;L134<642
 32296| 
 32297| 3332: ; preds = %3320
 32298|  %3333 = icmp eq i64 %3321, 0                                                                                          ;L1127<641
 32299|  br i1 %3333, label %3334, label %3338                                                                                 ;L1127<641
 32300| 
 32301| 3334: ; preds = %3332
 32302|     ;; __self_0 = ptr %1265
 32303|     ;; self = ptr %1265
 32304|     ;; __arg1_0 = ptr %122
 32305|     ;; other = ptr %122
 32308|  %3335 = load i64, ptr %3322, , !!8                                                                                    ;L1878<2123<1127<641
 32309|  %3336 = load i64, ptr %554, , !!8                                                                                     ;L1878<2123<1127<641
 32310|  %3337 = icmp eq i64 %3335, %3336                                                                                      ;L1878<2123<1127<641
 32311|  br i1 %3337, label %3338, label %3477                                                                                 ;L641
 32312| 
 32313| 3338: ; preds = %3334, %3332
 32314|     ;; self = ptr %1265
 32315|     ;; self = ptr %1265
 32316|     ;; self = ptr %1265
 32318|  %3339 = gep %1265, i64 160                                                                                            ;L1864<1064<2815<2680<641
 32319|  %3340 = load i64, ptr %3339, , !!8                                                                                    ;L1864<1064<2815<2680<641
 32321|     ;; self[8..+8] = i64 %3340
 32322|     ;; other[0..+8] = ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.67
 32323|     ;; other[8..+8] = i64 10
 32325|     ;; self[8..+8] = i64 %3340
 32327|     ;; other[0..+8] = ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.67
 32328|     ;; other[8..+8] = i64 10
 32330|     ;; len = i64 %3340
 32331|     ;; len = i64 %3340
 32332|     ;; size = i64 %3340
 32333|  %3341 = icmp eq i64 %3340, 10                                                                                         ;L21<2123<30<2680<641
 32334|  br i1 %3341, label %3342, label %3477                                                                                 ;L21<2123<30<2680<641
 32335| 
 32336| 3342: ; preds = %3338
 32337|  %3343 = gep %1265, i64 152                                                                                            ;L614<609<296<1968<1864<1064<2815<2680<641
 32338|  %3344 = load ptr, ptr %3343, , !!8, !!8                                                                               ;L614<609<296<1968<1864<1064<2815<2680<641
 32339|     ;; self[0..+8] = ptr %3344
 32340|     ;; self[0..+8] = ptr %3344
 32341|     ;; lhs = ptr %3344
 32342|     ;; rhs = ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.67
 32343|  %3345 = call i32 @memcmp(ptr %3344, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.67, i64 10)                            ;L157<24<2123<30<2680<641
 32344|  %3346 = icmp eq i32 %3345, 0                                                                                          ;L157<24<2123<30<2680<641
 32345|  br i1 %3346, label %3325, label %3477                                                                                 ;L641
 32346| 
 32347| 3347: ; preds = %3325
 32348|  %3348 = icmp eq i64 %3327, 1                                                                                          ;L134<642
 32349|  br i1 %3348, label %3477, label %3349                                                                                 ;L642
 32350| 
 32351| 3349: ; preds = %3347, %3325
 32352|  %3350 = gep %1265, i64 248                                                                                            ;L645
 32353|  %3351 = load i64, ptr %3350, , !!8                                                                                    ;L645
 32354|  %3352 = load ptr, ptr %1257, , !!8                                                                                    ;L645
 32355|  %3353 = invoke ptr %3352(ptr %151, i64 %3351)
 32356|  to label %3354 unwind label %1254                                                                                     ;L645
 32357| 
 32358| 3354: ; preds = %3349
 32359|  %3355 = icmp eq ptr %3353, null                                                                                       ;L645
 32360|  br i1 %3355, label %3477, label %3356                                                                                 ;L645
 32361| 
 32362| 3356: ; preds = %3354
 32363|     ;; caster = ptr %3353
 32364|  %3357 = gep %1265, i64 300                                                                                            ;L648
 32365|  %3358 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget16check_projectile(ptr %3357, ptr %1265, ptr %122)
 32366|  to label %3359 unwind label %1254                                                                                     ;L648
 32367| 
 32368| 3359: ; preds = %3356
 32369|  br i1 %3358, label %3360, label %3477                                                                                 ;L648
 32370| 
 32371| 3360: ; preds = %3359
 32372|  %3361 = load i32, ptr %342, , !!8                                                                                     ;L1511<651
 32373|     ;; mult = i32 %3361
 32374|  %3362 = icmp eq i32 %3361, 0                                                                                          ;L1512<651
 32375|  br i1 %3362, label %3363, label %3365                                                                                 ;L1512<651
 32376| 
 32377| 3363: ; preds = %3360
 32378|  %3364 = load i64, ptr %343, , !!8                                                                                     ;L1513<651
 32379|  br label %3371                                                                                                        ;L1512<651
 32380| 
 32381| 3365: ; preds = %3360
 32382|  %3366 = sext i32 %3361 to i64                                                                                         ;L1511<651
 32383|     ;; mult = i64 %3366
 32384|  %3367 = load i64, ptr %343, , !!8                                                                                     ;L1515<651
 32385|  %3368 = add nsw i64 %3366, 100                                                                                        ;L1515<651
 32386|  %3369 = mul i64 %3367, %3368                                                                                          ;L1515<651
 32387|  %3370 = udiv i64 %3369, 100                                                                                           ;L1515<651
 32388|  br label %3371                                                                                                        ;L1512<651
 32389| 
 32390| 3371: ; preds = %3365, %3363
 32391|  %3372 = phi i64 [ %3364, %3363 ], [ %3370, %3365 ]                                                                    ;L0<651
 32392|  %3373 = add i64 %3372, 18000                                                                                          ;L651
 32393|  %3374 = load i64, ptr %100, , !!8                                                                                     ;L651
 32394|  %3375 = load i64, ptr %99, , !!8                                                                                      ;L651
 32395|  %3376 = invoke zeroext i1 @gc::simulation10projectileNtB5_10Projectile11is_in_orbit(ptr %1265, i64 %3374, i64 %3375, i64 %3373)
 32396|  to label %3377 unwind label %1254                                                                                     ;L651
 32397| 
 32398| 3377: ; preds = %3371
 32399|  br i1 %3376, label %3378, label %3477                                                                                 ;L651
 32400| 
 32401| 3378: ; preds = %3377
 32402|     ;; self = ptr %1265
 32403|     ;; other = ptr %122
 32404|  %3379 = load i64, ptr %1265, , !!8                                                                                    ;L1127<655
 32405|  %3380 = gep %1265, i64 8                                                                                              ;L1127<655
 32406|     ;; __self_discr = i64 %3379
 32407|  %3381 = load i64, ptr %122, , !!8                                                                                     ;L1127<655
 32408|     ;; __arg1_discr = i64 %3381
 32409|  %3382 = icmp eq i64 %3379, %3381                                                                                      ;L1127<655
 32410|  br i1 %3382, label %3383, label %3385                                                                                 ;L1127<655
 32411| 
 32412| 3383: ; preds = %3378
 32413|  %3384 = icmp eq i64 %3379, 0                                                                                          ;L1127<655
 32414|  br i1 %3384, label %3387, label %3391                                                                                 ;L1127<655
 32415| 
 32416| 3385: ; preds = %3387, %3378
 32417|  %3386 = invoke i64 @gc::simulation10projectileNtB5_10Projectile22expected_damage_target(ptr %1265, ptr %135, ptr %3353, ptr %122)
 32418|  to label %3393 unwind label %1254                                                                                     ;L663
 32419| 
 32420| 3387: ; preds = %3383
 32421|     ;; __self_0 = ptr %1265
 32422|     ;; self = ptr %1265
 32423|     ;; __arg1_0 = ptr %122
 32424|     ;; other = ptr %122
 32427|  %3388 = load i64, ptr %3380, , !!8                                                                                    ;L1878<2123<1127<655
 32428|  %3389 = load i64, ptr %554, , !!8                                                                                     ;L1878<2123<1127<655
 32429|  %3390 = icmp eq i64 %3388, %3389                                                                                      ;L1878<2123<1127<655
 32430|  br i1 %3390, label %3391, label %3385                                                                                 ;L655
 32431| 
 32432| 3391: ; preds = %3387, %3383
 32433|  %3392 = invoke i64 @gc::simulation10projectileNtB5_10Projectile20expected_heal_target(ptr %1265, ptr %135, ptr %3353, ptr %122)
 32434|  to label %3492 unwind label %1254                                                                                     ;L657
 32435| 
 32436| 3393: ; preds = %3385
 32437|     ;; x = i64 %3386
 32438|     ;; inv_hp_q32 = i64 %149
 32439|  %3394 = zext i64 %3386 to i128                                                                                        ;L387<663
 32440|  %3395 = mul nuw nsw i128 %335, %3394                                                                                  ;L387<663
 32441|  %3396 = lshr i128 %3395, 32                                                                                           ;L387<663
 32442|     ;; self = i128 %3396
 32443|     ;; other = i128 150
 32444|  %3397 = call i128 @llvm.umin.i128(i128 %3396, i128 150)                                                               ;L1078<387<663
 32445|  %3398 = trunc nuw nsw i128 %3397 to i64                                                                               ;L387<663
 32446|     ;; ratio = i64 %3398
 32447|     ;; nearest_other_distance[0..+8] = i64 0
 32448|     ;; nearest_other_distance[8..+8] = i64 undef
 32449|     ;; self = ptr %93
 32450|     ;; self = ptr %93
 32451|  %3399 = load ptr, ptr %93, , !!8, !!8                                                                                 ;L138<2073<665
 32452|     ;; p = ptr %3399
 32453|  %3400 = load i64, ptr %230, , !!8                                                                                     ;L2075<665
 32454|     ;; len = i64 %3400
 32455|     ;; count = i64 %3400
 32456|     ;; self[0..+8] = ptr %3399
 32457|     ;; slice[0..+8] = ptr %3399
 32458|     ;; self[8..+8] = i64 %3400
 32459|     ;; slice[8..+8] = i64 %3400
 32460|     ;; ptr = ptr %3399
 32461|     ;; self = ptr %3399
 32462|  %3401 = gepS, ptr, i64 }, ptr %3399, i64 %3400                                                                        ;L961<100<1042<665
 32463|     ;; iter[0..+8] = ptr %3399
 32464|     ;; iter[8..+8] = ptr %3401
 32465|  br label %3402                                                                                                        ;L665
 32466| 
 32467| 3402: ; preds = %3440, %3393
 32468|  %3403 = phi i64 [ %3453, %3440 ], [ undef, %3393 ]
 32469|  %3404 = phi i1 [ true, %3440 ], [ false, %3393 ]
 32470|  %3405 = phi ptr [ %3410, %3440 ], [ %3399, %3393 ]
 32471|  br label %3406                                                                                                        ;L180<665
 32472| 
 32473| 3406: ; preds = %3425, %3402
 32474|  %3407 = phi ptr [ %3410, %3425 ], [ %3405, %3402 ]                                                                    ;L665
 32475|     ;; iter[0..+8] = ptr %3407
 32477|     ;; nearest_other_distance[8..+8] = i64 %3403
 32478|     ;; self = ptr undef
 32479|     ;; ptr = ptr %3407
 32480|     ;; self = ptr %3407
 32481|     ;; end_or_len = ptr %3401
 32484|  %3408 = icmp eq ptr %3407, %3401                                                                                      ;L1714<180<665
 32485|  br i1 %3408, label %3414, label %3409                                                                                 ;L180<665
 32486| 
 32487| 3409: ; preds = %3406
 32488|  %3410 = gep %3407, i64 448                                                                                            ;L656<185<665
 32489|     ;; iter[0..+8] = ptr %3410
 32490|     ;; a = ptr %3407
 32491|  %3411 = gep %3407, i64 432                                                                                            ;L666
 32492|  %3412 = load ptr, ptr %3411, , !!8, !!8                                                                               ;L666
 32493|     ;; self = ptr %3412
 32494|  %3413 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget16check_projectile(ptr %3357, ptr %1265, ptr %3412)
 32495|  to label %3416 unwind label %1254                                                                                     ;L666
 32496| 
 32497| 3414: ; preds = %3406
 32501|     ;; self = ptr %1265
 32502|  %3415 = icmp eq i64 %3331, 0                                                                                          ;L121<675
 32503|  br i1 %3415, label %3454, label %3473                                                                                 ;L121<675
 32504| 
 32505| 3416: ; preds = %3409
 32506|  br i1 %3413, label %3417, label %3425                                                                                 ;L666
 32507| 
 32508| 3417: ; preds = %3416
 32509|  %3418 = gep %3412, i64 1632                                                                                           ;L666
 32510|  %3419 = load i64, ptr %3418, , !!8                                                                                    ;L666
 32511|     ;; x2 = i64 %3419
 32512|     ;; other = i64 %3419
 32513|  %3420 = gep %3412, i64 1640                                                                                           ;L666
 32514|  %3421 = load i64, ptr %3420, , !!8                                                                                    ;L666
 32515|     ;; y2 = i64 %3421
 32516|     ;; other = i64 %3421
 32517|  %3422 = gep %3412, i64 1136                                                                                           ;L1511<666
 32518|  %3423 = load i32, ptr %3422, , !!8                                                                                    ;L1511<666
 32519|     ;; mult = i32 %3423
 32520|  %3424 = icmp eq i32 %3423, 0                                                                                          ;L1512<666
 32521|  br i1 %3424, label %3426, label %3429                                                                                 ;L1512<666
 32522| 
 32523| 3425: ; preds = %3439, %3416
 32524|  br label %3406                                                                                                        ;L1
 32525| 
 32526| 3426: ; preds = %3417
 32527|  %3427 = gep %3412, i64 1664                                                                                           ;L1513<666
 32528|  %3428 = load i64, ptr %3427, , !!8                                                                                    ;L1513<666
 32529|  br label %3436                                                                                                        ;L1512<666
 32530| 
 32531| 3429: ; preds = %3417
 32532|  %3430 = sext i32 %3423 to i64                                                                                         ;L1511<666
 32533|     ;; mult = i64 %3430
 32534|  %3431 = gep %3412, i64 1664                                                                                           ;L1515<666
 32535|  %3432 = load i64, ptr %3431, , !!8                                                                                    ;L1515<666
 32536|  %3433 = add nsw i64 %3430, 100                                                                                        ;L1515<666
 32537|  %3434 = mul i64 %3432, %3433                                                                                          ;L1515<666
 32538|  %3435 = udiv i64 %3434, 100                                                                                           ;L1515<666
 32539|  br label %3436                                                                                                        ;L1512<666
 32540| 
 32541| 3436: ; preds = %3429, %3426
 32542|  %3437 = phi i64 [ %3428, %3426 ], [ %3435, %3429 ]                                                                    ;L0<666
 32543|  %3438 = invoke zeroext i1 @gc::simulation10projectileNtB5_10Projectile11is_in_orbit(ptr %1265, i64 %3419, i64 %3421, i64 %3437)
 32544|  to label %3439 unwind label %1254                                                                                     ;L666
 32545| 
 32546| 3439: ; preds = %3436
 32547|  br i1 %3438, label %3440, label %3425                                                                                 ;L666
 32548| 
 32549| 3440: ; preds = %3439
 32550|  %3441 = icmp ult i64 %1270, %3419                                                                                     ;L3147<7<669
 32551|  %3442 = sub nuw i64 %3419, %1270                                                                                      ;L3147<7<669
 32552|  %3443 = sub nuw i64 %1270, %3419                                                                                      ;L3147<7<669
 32553|  %3444 = select i1 %3441, i64 %3442, i64 %3443                                                                         ;L3147<7<669
 32554|     ;; dx = i64 %3444
 32555|  %3445 = icmp ult i64 %1272, %3421                                                                                     ;L3147<8<669
 32556|  %3446 = sub nuw i64 %3421, %1272                                                                                      ;L3147<8<669
 32557|  %3447 = sub nuw i64 %1272, %3421                                                                                      ;L3147<8<669
 32558|  %3448 = select i1 %3445, i64 %3446, i64 %3447                                                                         ;L3147<8<669
 32559|     ;; dy = i64 %3448
 32560|  %3449 = mul i64 %3444, %3444                                                                                          ;L9<669
 32561|  %3450 = mul i64 %3448, %3448                                                                                          ;L9<669
 32562|  %3451 = add i64 %3450, %3449                                                                                          ;L9<669
 32563|     ;; dist = i64 %3451
 32565|     ;; self[8..+8] = i64 %3403
 32566|     ;; f = ptr undef
 32567|  %3452 = call i64 @llvm.umin.i64(i64 %3451, i64 %3403)                                                                 ;L708<670
 32568|  %3453 = select i1 %3404, i64 %3452, i64 %3451                                                                         ;L708<670
 32569|     ;; nearest_other_distance[0..+8] = i64 1
 32570|     ;; nearest_other_distance[8..+8] = i64 %3453
 32571|  br label %3402                                                                                                        ;L665
 32572| 
 32573| 3454: ; preds = %3414
 32574|  %3455 = load i64, ptr %100,                                                                                           ;L3147<7<674
 32575|  %3456 = sub i64 %1270, %3455                                                                                          ;L3147<7<674
 32576|  %3457 = sub i64 %3455, %1270                                                                                          ;L3147<7<674
 32577|  %3458 = select i1 %1274, i64 %3456, i64 %3457                                                                         ;L3147<7<674
 32578|     ;; dx = i64 %3458
 32579|  %3459 = mul i64 %3458, %3458                                                                                          ;L9<674
 32580|  %3460 = load i64, ptr %99,                                                                                            ;L3147<8<674
 32581|  %3461 = sub i64 %1272, %3460                                                                                          ;L3147<8<674
 32582|  %3462 = sub i64 %3460, %1272                                                                                          ;L3147<8<674
 32583|  %3463 = select i1 %1279, i64 %3461, i64 %3462                                                                         ;L3147<8<674
 32584|     ;; dy = i64 %3463
 32585|     ;; d = !DIArgList(i64 %3459, i64 %3463, i64 %3463)
 32586|  %3464 = mul i64 %3463, %3463                                                                                          ;L9<674
 32587|     ;; d = !DIArgList(i64 %3459, i64 %3464)
 32588|  %3465 = add i64 %3464, %3459                                                                                          ;L9<674
 32589|     ;; d = i64 %3465
 32590|     ;; penetrate = ptr %1265
 32591|  %3466 = gep %1265, i64 120                                                                                            ;L122<675
 32592|  %3467 = load i8, ptr %3466, , !!8                                                                                     ;L122<675
 32593|  %3468 = trunc nuw i8 %3467 to i1                                                                                      ;L122<675
 32594|  %3469 = xor i1 %3468, true                                                                                            ;L675
 32595|  %3470 = and i1 %3404, %3469                                                                                           ;L675
 32596|  %3471 = icmp ult i64 %3403, %3465
 32597|  %3472 = select i1 %3470, i1 %3471, i1 false                                                                           ;L675
 32598|  br i1 %3472, label %3477, label %3478                                                                                 ;L675
 32599| 
 32600| 3473: ; preds = %3414
 32601|     ;; score[0..+8] = !DIArgList(i64 %1262, i64 %3398)
 32602|     ;; score[0..+8] = !DIArgList(i64 %1262, i64 %3398)
 32603|  %3474 = icmp eq i64 %3331, 2                                                                                          ;L679
 32604|  %3475 = select i1 %3474, i8 1, i8 %1260                                                                               ;L679
 32605|  %3476 = select i1 %3474, i8 %1261, i8 1                                                                               ;L679
 32606|  br label %3478                                                                                                        ;L679
 32607| 
 32608| 3477: ; preds = %3454, %3377, %3359, %3354, %3347, %3342, %3338, %3334, %3325, %3325, %3320, %1268
 32609|  br label %1264                                                                                                        ;L1
 32610| 
 32611| 3478: ; preds = %3473, %3454
 32612|  %3479 = phi i8 [ %3475, %3473 ], [ %1260, %3454 ]                                                                     ;L0
 32613|  %3480 = phi i8 [ %3476, %3473 ], [ 1, %3454 ]                                                                         ;L0
 32614|  %3481 = add i64 %1262, %3398                                                                                          ;L678
 32615|     ;; score[48..+1] = i8 %3480
 32616|     ;; score[48..+1] = i8 %3480
 32617|     ;; score[49..+1] = i8 %3479
 32618|     ;; score[49..+1] = i8 %3479
 32619|  %3482 = trunc nuw i8 %1263 to i1                                                                                      ;L684
 32620|  br i1 %3482, label %3487, label %3483                                                                                 ;L684
 32621| 
 32622| 3483: ; preds = %3478
 32623|  %3484 = invoke zeroext i1 @gc::simulation10projectileNtB5_10Projectile6has_cc(ptr %1265)
 32624|  to label %3485 unwind label %1254                                                                                     ;L684
 32625| 
 32626| 3485: ; preds = %3483
 32627|  %3486 = zext i1 %3484 to i8                                                                                           ;L684
 32628|     ;; has_pcc = i8 %3486
 32629|  br label %3487                                                                                                        ;L684
 32630| 
 32631| 3487: ; preds = %3506, %3485, %3478
 32632|  %3488 = phi i8 [ %1260, %3506 ], [ %3479, %3478 ], [ %3479, %3485 ]                                                   ;L0
 32633|  %3489 = phi i8 [ %1261, %3506 ], [ %3480, %3478 ], [ %3480, %3485 ]                                                   ;L367<390
 32634|  %3490 = phi i64 [ %3508, %3506 ], [ %3481, %3478 ], [ %3481, %3485 ]                                                  ;L0
 32635|  %3491 = phi i8 [ %1263, %3506 ], [ 1, %3478 ], [ %3486, %3485 ]                                                       ;L0
 32636|     ;; score[0..+8] = i64 %3490
 32637|     ;; score[0..+8] = i64 %3490
 32638|     ;; score[48..+1] = i8 %3489
 32639|     ;; score[48..+1] = i8 %3489
 32640|     ;; score[49..+1] = i8 %3488
 32641|     ;; score[49..+1] = i8 %3488
 32642|     ;; has_pcc = i8 %3491
 32643|  br label %1259                                                                                                        ;L639
 32644| 
 32645| 3492: ; preds = %3391
 32646|  %3493 = invoke i64 @gc::simulation10projectileNtB5_10Projectile22expected_shield_target(ptr %1265, ptr %135, ptr %3353, ptr %122)
 32647|  to label %3494 unwind label %1254                                                                                     ;L658
 32648| 
 32649| 3494: ; preds = %3492
 32650|  %3495 = add i64 %3493, %3392                                                                                          ;L657
 32651|     ;; damage = i64 %3495
 32652|     ;; hp = i64 %147
 32653|  %3496 = icmp eq i64 %3495, 0                                                                                          ;L53<656
 32654|  br i1 %3496, label %3506, label %3497                                                                                 ;L53<656
 32655| 
 32656| 3497: ; preds = %3494
 32657|  %3498 = mul i64 %3495, 100                                                                                            ;L56<656
 32658|     ;; self = i64 %147
 32659|     ;; other = i64 1
 32660|  %3499 = icmp eq i64 %3498, -9223372036854775808                                                                       ;L56<656
 32661|  %3500 = and i1 %1258, %3499                                                                                           ;L56<656
 32662|  br i1 %3500, label %3504, label %3501                                                                                 ;L56<656
 32663| 
 32664| 3501: ; preds = %3497
 32665|  %3502 = sdiv i64 %3498, %148                                                                                          ;L56<656
 32666|     ;; self = i64 %3502
 32667|     ;; other = i64 150
 32668|  %3503 = call i64 @llvm.smin.i64(i64 %3502, i64 150)                                                                   ;L1078<56<656
 32669|  br label %3506                                                                                                        ;L57<656
 32670| 
 32671| 3504: ; preds = %3497
 32672|  invoke void @core::panicking11panic_const24panic_const_div_overflow(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.122) #25
 32673|  to label %3505 unwind label %1254                                                                                     ;L56<656
 32674| 
 32675| 3505: ; preds = %3504
 32676|  unreachable                                                                                                           ;L56<656
 32677| 
 32678| 3506: ; preds = %3501, %3494
 32679|  %3507 = phi i64 [ %3503, %3501 ], [ 0, %3494 ]                                                                        ;L0<656
 32680|     ;; ratio = i64 %3507
 32681|  %3508 = sub i64 %1262, %3507                                                                                          ;L661
 32682|     ;; score[0..+8] = i64 %3508
 32683|     ;; score[0..+8] = i64 %3508
 32684|  br label %3487                                                                                                        ;L655
 32685| 
 32686| 3509: ; preds = %1060
 32687|  cleanupret from %1062 unwind label %384
 32688| 
 32689| 3510: ; preds = %1060
 32690|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %80) #27 [ "funclet"(token %1062) ] ;L1181
 32691|  cleanupret from %1062 unwind label %384                                                                               ;L1181
 32692| 
 32693| 3511: ; preds = %1028
 32694|     ;; info = ptr %1029
 32695|  %3512 = gep %1029, i64 280                                                                                            ;L584
 32696|  %3513 = load i8, ptr %3512, , !!8                                                                                     ;L584
 32697|     ;; is_range = i8 %3513
 32698|     ;; self = ptr %1029
 32699|     ;; self = ptr %1029
 32700|  %3514 = gep %1029, i64 136                                                                                            ;L633<682<585
 32701|  %3515 = load i64, ptr %3514, , !!8                                                                                    ;L633<682<585
 32702|  %3516 = trunc nuw i64 %3515 to i1                                                                                     ;L682<585
 32703|  %3517 = xor i1 %3516, true                                                                                            ;L682<585
 32704|     ;; is_target_none = i64 %3515
 32707|     ;; f = ptr %122
 32708|  br i1 %3516, label %3518, label %3523                                                                                 ;L659<586
 32709| 
 32710| 3518: ; preds = %3511
 32711|  %3519 = gep %1029, i64 144                                                                                            ;L633<682<585
 32712|  %3520 = load i64, ptr %3519,                                                                                          ;L586
 32713|     ;; self[8..+8] = i64 %3520
 32714|     ;; x = i64 %3520
 32715|  %3521 = load i64, ptr %231, , !!8                                                                                     ;L661<586
 32717|     ;; x = i64 %3520
 32718|  %3522 = icmp ne i64 %3520, %3521                                                                                      ;L586<661<586
 32719|  br label %3523                                                                                                        ;L586<661<586
 32720| 
 32721| 3523: ; preds = %3518, %3511
 32722|  %3524 = phi i1 [ false, %3511 ], [ %3522, %3518 ]                                                                     ;L0<586
 32724|  %3525 = trunc nuw i8 %3513 to i1                                                                                      ;L590
 32727|  br i1 %3525, label %3530, label %3526                                                                                 ;L590
 32728| 
 32729| 3526: ; preds = %3523, %1028
 32730|  %3527 = phi i1 [ %3517, %3523 ], [ false, %1028 ]
 32731|  %3528 = phi i1 [ %3524, %3523 ], [ false, %1028 ]
 32732|     ;; self = ptr undef
 32733|     ;; self = ptr undef
 32734|  %3529 = icmp eq i64 %993, 0                                                                                           ;L430<682<594
 32735|  br i1 %3529, label %3532, label %3553                                                                                 ;L594
 32736| 
 32737| 3530: ; preds = %3523
 32738|     ;; self = ptr undef
 32739|     ;; self = ptr undef
 32740|  %3531 = icmp eq i64 %995, 0                                                                                           ;L430<682<591
 32741|  br i1 %3531, label %3559, label %3553                                                                                 ;L591
 32742| 
 32743| 3532: ; preds = %3526
 32747|     ;; attacker = ptr %1029
 32748|     ;; target = ptr %122
 32749|     ;; seed = ptr %43
 32750|     ;; tick = ptr %42
 32751|     ;; key = ptr %41
 32753|  %3533 = load ptr, ptr %118, , !!46657, !!8, !!8                                                                       ;L131<595
 32754|  %3534 = load ptr, ptr %152, , !!46657, !!8, !!8                                                                       ;L131<595
 32755|  %3535 = gep %3534, i64 32                                                                                             ;L131<595
 32756|  %3536 = load ptr, ptr %3535, , !!46657, !!8                                                                           ;L131<595
 32757|  %3537 = invoke i64 %3536(ptr %3533)
 32758|  to label %3538 unwind label %384                                                                                      ;L131<595
 32759| 
 32760| 3538: ; preds = %3532
 32761|  store i64 %3537, ptr %43, , !!46657                                                                                   ;L131<595
 32763|  %3539 = gep %3534, i64 40                                                                                             ;L132<595
 32764|  %3540 = load ptr, ptr %3539, , !!46657, !!8                                                                           ;L132<595
 32765|  %3541 = invoke i64 %3540(ptr %3533)
 32766|  to label %3542 unwind label %384                                                                                      ;L132<595
 32767| 
 32768| 3542: ; preds = %3538
 32769|  store i64 %3541, ptr %42, , !!46657                                                                                   ;L132<595
 32771|  %3543 = gep %1029, i64 1472                                                                                           ;L133<595
 32772|  %3544 = load i64, ptr %3543, , !!46649, !!8                                                                           ;L133<595
 32773|  %3545 = load i64, ptr %231, , !!46645, !!8                                                                            ;L133<595
 32774|  store i64 %3544, ptr %41, , !!46657                                                                                   ;L133<595
 32775|  store i64 %3545, ptr %978, , !!46657                                                                                  ;L133<595
 32777|  store ptr %43, ptr %40, , !!46657                                                                                     ;L134<595
 32778|  store ptr %42, ptr %979, , !!46657                                                                                    ;L134<595
 32779|  store ptr %41, ptr %980, , !!46657                                                                                    ;L134<595
 32780|  store ptr %1029, ptr %981, , !!46657                                                                                  ;L134<595
 32781|  store ptr %135, ptr %982, , !!46657                                                                                   ;L134<595
 32782|  store ptr %122, ptr %983, , !!46657                                                                                   ;L134<595
 32783|  %3546 = invoke i64 @core::cell7RefCellNtNtCshdEBA0ozCnw_7game_ai13position_eval17AttackDamageCacheEE4withNCNvB1x_29expected_attack_damage_cached0jEB1z_(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.119, ptr %40)
 32784|  to label %3547 unwind label %384                                                                                      ;L134<595
 32785| 
 32786| 3547: ; preds = %3542
 32791|     ;; x = i64 %3546
 32792|     ;; inv_hp_q32 = i64 %149
 32793|  %3548 = zext i64 %3546 to i128                                                                                        ;L387<595
 32794|  %3549 = mul nuw nsw i128 %335, %3548                                                                                  ;L387<595
 32795|  %3550 = lshr i128 %3549, 32                                                                                           ;L387<595
 32796|     ;; self = i128 %3550
 32797|     ;; other = i128 150
 32798|  %3551 = call i128 @llvm.umin.i128(i128 %3550, i128 150)                                                               ;L1078<387<595
 32799|  %3552 = trunc nuw nsw i128 %3551 to i64                                                                               ;L387<595
 32800|     ;; range_minion_attack[0..+8] = i64 %995
 32801|     ;; range_minion_attack[8..+8] = i64 %994
 32802|     ;; melee_minion_attack[0..+8] = i64 1
 32803|     ;; melee_minion_attack[8..+8] = i64 %3552
 32804|     ;; ratio = i64 %3552
 32805|  br i1 %3527, label %3587, label %3580                                                                                 ;L598
 32806| 
 32807| 3553: ; preds = %3530, %3526
 32808|  %3554 = phi i1 [ %3517, %3530 ], [ %3527, %3526 ]
 32809|  %3555 = phi i1 [ %3524, %3530 ], [ %3528, %3526 ]
 32810|  %3556 = phi i64 [ %994, %3530 ], [ %992, %3526 ]                                                                      ;L1012<597
 32811|  %3557 = phi i64 [ %993, %3530 ], [ 1, %3526 ]                                                                         ;L0
 32812|  %3558 = phi i64 [ 1, %3530 ], [ %995, %3526 ]                                                                         ;L0
 32813|     ;; range_minion_attack[0..+8] = i64 %3558
 32814|     ;; range_minion_attack[8..+8] = i64 %994
 32815|     ;; melee_minion_attack[0..+8] = i64 %3557
 32816|     ;; melee_minion_attack[8..+8] = i64 %992
 32817|     ;; ratio = i64 %3556
 32818|  br i1 %3554, label %3587, label %3580                                                                                 ;L598
 32819| 
 32820| 3559: ; preds = %3530
 32824|     ;; attacker = ptr %1029
 32825|     ;; target = ptr %122
 32826|     ;; seed = ptr %39
 32827|     ;; tick = ptr %38
 32828|     ;; key = ptr %37
 32830|  %3560 = load ptr, ptr %118, , !!46697, !!8, !!8                                                                       ;L131<592
 32831|  %3561 = load ptr, ptr %152, , !!46697, !!8, !!8                                                                       ;L131<592
 32832|  %3562 = gep %3561, i64 32                                                                                             ;L131<592
 32833|  %3563 = load ptr, ptr %3562, , !!46697, !!8                                                                           ;L131<592
 32834|  %3564 = invoke i64 %3563(ptr %3560)
 32835|  to label %3565 unwind label %384                                                                                      ;L131<592
 32836| 
 32837| 3565: ; preds = %3559
 32838|  store i64 %3564, ptr %39, , !!46697                                                                                   ;L131<592
 32840|  %3566 = gep %3561, i64 40                                                                                             ;L132<592
 32841|  %3567 = load ptr, ptr %3566, , !!46697, !!8                                                                           ;L132<592
 32842|  %3568 = invoke i64 %3567(ptr %3560)
 32843|  to label %3569 unwind label %384                                                                                      ;L132<592
 32844| 
 32845| 3569: ; preds = %3565
 32846|  store i64 %3568, ptr %38, , !!46697                                                                                   ;L132<592
 32848|  %3570 = gep %1029, i64 1472                                                                                           ;L133<592
 32849|  %3571 = load i64, ptr %3570, , !!46689, !!8                                                                           ;L133<592
 32850|  %3572 = load i64, ptr %231, , !!46685, !!8                                                                            ;L133<592
 32851|  store i64 %3571, ptr %37, , !!46697                                                                                   ;L133<592
 32852|  store i64 %3572, ptr %984, , !!46697                                                                                  ;L133<592
 32854|  store ptr %39, ptr %36, , !!46697                                                                                     ;L134<592
 32855|  store ptr %38, ptr %985, , !!46697                                                                                    ;L134<592
 32856|  store ptr %37, ptr %986, , !!46697                                                                                    ;L134<592
 32857|  store ptr %1029, ptr %987, , !!46697                                                                                  ;L134<592
 32858|  store ptr %135, ptr %988, , !!46697                                                                                   ;L134<592
 32859|  store ptr %122, ptr %989, , !!46697                                                                                   ;L134<592
 32860|  %3573 = invoke i64 @core::cell7RefCellNtNtCshdEBA0ozCnw_7game_ai13position_eval17AttackDamageCacheEE4withNCNvB1x_29expected_attack_damage_cached0jEB1z_(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.119, ptr %36)
 32861|  to label %3574 unwind label %384                                                                                      ;L134<592
 32862| 
 32863| 3574: ; preds = %3569
 32868|     ;; x = i64 %3573
 32869|     ;; inv_hp_q32 = i64 %149
 32870|  %3575 = zext i64 %3573 to i128                                                                                        ;L387<592
 32871|  %3576 = mul nuw nsw i128 %335, %3575                                                                                  ;L387<592
 32872|  %3577 = lshr i128 %3576, 32                                                                                           ;L387<592
 32873|     ;; self = i128 %3577
 32874|     ;; other = i128 150
 32875|  %3578 = call i128 @llvm.umin.i128(i128 %3577, i128 150)                                                               ;L1078<387<592
 32876|  %3579 = trunc nuw nsw i128 %3578 to i64                                                                               ;L387<592
 32877|     ;; range_minion_attack[0..+8] = i64 1
 32878|     ;; range_minion_attack[8..+8] = i64 %3579
 32879|     ;; melee_minion_attack[0..+8] = i64 %993
 32880|     ;; melee_minion_attack[8..+8] = i64 %992
 32881|     ;; ratio = i64 %3579
 32882|  br i1 %3516, label %3580, label %3587                                                                                 ;L598
 32883| 
 32884| 3580: ; preds = %3574, %3553, %3547
 32885|  %3581 = phi i64 [ %995, %3547 ], [ %3558, %3553 ], [ 1, %3574 ]
 32886|  %3582 = phi i64 [ %994, %3547 ], [ %994, %3553 ], [ %3579, %3574 ]
 32887|  %3583 = phi i64 [ 1, %3547 ], [ %3557, %3553 ], [ %993, %3574 ]
 32888|  %3584 = phi i64 [ %3552, %3547 ], [ %992, %3553 ], [ %992, %3574 ]
 32889|  %3585 = phi i64 [ %3552, %3547 ], [ %3556, %3553 ], [ %3579, %3574 ]
 32890|  %3586 = phi i1 [ %3528, %3547 ], [ %3555, %3553 ], [ %3524, %3574 ]
 32891|  br i1 %3586, label %3594, label %3596                                                                                 ;L598
 32892| 
 32893| 3587: ; preds = %3574, %3553, %3547
 32894|  %3588 = phi i64 [ %995, %3547 ], [ %3558, %3553 ], [ 1, %3574 ]
 32895|  %3589 = phi i64 [ %994, %3547 ], [ %994, %3553 ], [ %3579, %3574 ]
 32896|  %3590 = phi i64 [ 1, %3547 ], [ %3557, %3553 ], [ %993, %3574 ]
 32897|  %3591 = phi i64 [ %3552, %3547 ], [ %992, %3553 ], [ %992, %3574 ]
 32898|  %3592 = phi i64 [ %3552, %3547 ], [ %3556, %3553 ], [ %3579, %3574 ]
 32899|  %3593 = sdiv i64 %3592, 2                                                                                             ;L598
 32900|     ;; ratio = i64 %3593
 32901|  br label %3596                                                                                                        ;L598
 32902| 
 32903| 3594: ; preds = %3580
 32904|  %3595 = sdiv i64 %3585, 3                                                                                             ;L598
 32905|     ;; ratio = i64 %3595
 32906|  br label %3596                                                                                                        ;L598
 32907| 
 32908| 3596: ; preds = %3594, %3587, %3580
 32909|  %3597 = phi i64 [ %3588, %3587 ], [ %3581, %3594 ], [ %3581, %3580 ]
 32910|  %3598 = phi i64 [ %3589, %3587 ], [ %3582, %3594 ], [ %3582, %3580 ]
 32911|  %3599 = phi i64 [ %3590, %3587 ], [ %3583, %3594 ], [ %3583, %3580 ]
 32912|  %3600 = phi i64 [ %3591, %3587 ], [ %3584, %3594 ], [ %3584, %3580 ]
 32913|  %3601 = phi i64 [ %3593, %3587 ], [ %3595, %3594 ], [ %3585, %3580 ]                                                  ;L598
 32914|     ;; ratio = i64 %3601
 32915|  %3602 = load ptr, ptr %97, , !!8, !!8                                                                                 ;L599
 32916|  %3603 = load ptr, ptr %158,                                                                                           ;L599
 32917|  %3604 = gep %1029, i64 1632                                                                                           ;L599
 32918|  %3605 = load i64, ptr %3604, , !!8                                                                                    ;L599
 32919|  %3606 = gep %1029, i64 1640                                                                                           ;L599
 32920|  %3607 = load i64, ptr %3606,                                                                                          ;L599
 32924|  %3608 = load i64, ptr %3602, , !!8                                                                                    ;L395<599
 32925|     ;; a = i64 %3608
 32926|     ;; self = i64 %3608
 32927|     ;; b = i64 %3605
 32928|     ;; other = i64 %3605
 32929|  %3609 = icmp ult i64 %3608, %3605                                                                                     ;L3147<8<395<599
 32930|  %3610 = sub nuw i64 %3605, %3608                                                                                      ;L3147<8<395<599
 32931|  %3611 = sub nuw i64 %3608, %3605                                                                                      ;L3147<8<395<599
 32932|  %3612 = select i1 %3609, i64 %3610, i64 %3611                                                                         ;L3147<8<395<599
 32933|     ;; rhs = i64 %3612
 32934|     ;; rhs = i64 %3612
 32935|     ;; rhs = i64 %3612
 32936|     ;; diff = i64 %3612
 32937|     ;; self = i64 %3612
 32938|     ;; self = i64 %3612
 32939|     ;; self = i64 %3612
 32940|  %3613 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %3612, i64 %3612)                                           ;L3178<1288<2517<9<395<599
 32941|  %3614 = extractvalue { i64, i1 } %3613, 0                                                                             ;L3178<1288<2517<9<395<599
 32942|  %3615 = extractvalue { i64, i1 } %3613, 1                                                                             ;L3178<1288<2517<9<395<599
 32943|     ;; a = i64 %3614
 32944|     ;; self = i64 %3614
 32945|     ;; b = i1 %3615
 32946|     ;; b = i1 %3615
 32947|  br i1 %3615, label %3616, label %3617                                                                                 ;L459<1289<2517<9<395<599
 32948| 
 32949| 3616: ; preds = %3596
 32950|     ;; a = i64 -1
 32951|     ;; self = i64 -1
 32952|  br label %3617                                                                                                        ;L2519<9<395<599
 32953| 
 32954| 3617: ; preds = %3616, %3596
 32955|  %3618 = phi i64 [ -1, %3616 ], [ %3614, %3596 ]                                                                       ;L0<9<395<599
 32956|     ;; self = i64 %3618
 32957|     ;; a = i64 %3618
 32958|  %3619 = icmp ne ptr %3603, null
 32959|  call void @llvm.assume(i1 %3619)
 32960|  %3620 = load i64, ptr %3603, , !!8                                                                                    ;L395<599
 32961|     ;; a = i64 %3620
 32962|     ;; self = i64 %3620
 32963|     ;; b = i64 %3607
 32964|     ;; other = i64 %3607
 32965|  %3621 = icmp ult i64 %3620, %3607                                                                                     ;L3147<8<395<599
 32966|  %3622 = sub nuw i64 %3607, %3620                                                                                      ;L3147<8<395<599
 32967|  %3623 = sub nuw i64 %3620, %3607                                                                                      ;L3147<8<395<599
 32968|  %3624 = select i1 %3621, i64 %3622, i64 %3623                                                                         ;L3147<8<395<599
 32969|     ;; rhs = i64 %3624
 32970|     ;; rhs = i64 %3624
 32971|     ;; rhs = i64 %3624
 32972|     ;; diff = i64 %3624
 32973|     ;; self = i64 %3624
 32974|     ;; self = i64 %3624
 32975|     ;; self = i64 %3624
 32976|  %3625 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %3624, i64 %3624)                                           ;L3178<1288<2517<9<395<599
 32977|  %3626 = extractvalue { i64, i1 } %3625, 0                                                                             ;L3178<1288<2517<9<395<599
 32978|  %3627 = extractvalue { i64, i1 } %3625, 1                                                                             ;L3178<1288<2517<9<395<599
 32979|     ;; a = i64 %3626
 32980|     ;; rhs = i64 %3626
 32981|     ;; b = i1 %3627
 32982|     ;; b = i1 %3627
 32983|  br i1 %3627, label %3628, label %3629                                                                                 ;L459<1289<2517<9<395<599
 32984| 
 32985| 3628: ; preds = %3617
 32986|     ;; a = i64 -1
 32987|     ;; rhs = i64 -1
 32988|  br label %3629                                                                                                        ;L2519<9<395<599
 32989| 
 32990| 3629: ; preds = %3628, %3617
 32991|  %3630 = phi i64 [ -1, %3628 ], [ %3626, %3617 ]                                                                       ;L0<9<395<599
 32992|     ;; rhs = i64 %3630
 32993|     ;; a = i64 %3630
 32994|  %3631 = call i64 @llvm.uadd.sat.i64(i64 %3618, i64 %3630)                                                             ;L2428<395<599
 32995|  %3632 = icmp ult i64 %3631, 4096000001                                                                                ;L599
 32996|  %3633 = select i1 %3632, i64 %3601, i64 0                                                                             ;L599
 32997|  %3634 = add i64 %3633, %991                                                                                           ;L599
 32998|     ;; score[0..+8] = i64 %3634
 32999|     ;; score[0..+8] = i64 %3634
 33000|  br label %990                                                                                                         ;L580
 33001| 
 33002| 3635: ; preds = %863, %862, %841
 33003|  %3636 = phi i64 [ %861, %862 ], [ 0, %841 ], [ 0, %863 ]                                                              ;L0<310<1162<107<507
 33004|     ;; cnt = i64 %3636
 33005|     ;; t = ptr %834
 33006|     ;; caster = ptr %834
 33007|     ;; self = ptr %834
 33008|     ;; t_atk = ptr %834
 33009|     ;; self = ptr %834
 33010|  %3637 = gep %834, i64 1168                                                                                            ;L742<510
 33011|  %3638 = gep %834, i64 1216                                                                                            ;L742<510
 33012|  %3639 = load i32, ptr %3638, , !!8                                                                                    ;L742<510
 33013|  %3640 = icmp eq i32 %3639, -1                                                                                         ;L742<510
 33014|  br i1 %3640, label %3644, label %3641                                                                                 ;L742<510
 33015| 
 33016| 3641: ; preds = %3635
 33017|     ;; self = ptr %3637
 33018|     ;; t_atk = ptr %3637
 33019|     ;; self = ptr %3637
 33020|     ;; self = ptr %834
 33021|     ;; other = ptr %122
 33022|  %3642 = load i64, ptr %834, , !!8                                                                                     ;L1127<511
 33023|     ;; __self_discr = i64 %3642
 33024|  %3643 = icmp eq i64 %3642, 0                                                                                          ;L1127<511
 33025|  br i1 %3643, label %3649, label %3645                                                                                 ;L1127<511
 33026| 
 33027| 3644: ; preds = %3635
 33028|     ;; self = ptr null
 33029|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.101) #25
 33030|  to label %127 unwind label %384                                                                                       ;L1013<510
 33031| 
 33032| 3645: ; preds = %3649, %3641
 33033|  %3646 = gep %834, i64 104                                                                                             ;L525
 33034|  %3647 = load i64, ptr %3646, , !!8                                                                                    ;L525
 33035|  %3648 = icmp eq i64 %3647, 2                                                                                          ;L525
 33036|  br i1 %3648, label %3657, label %3672                                                                                 ;L525
 33037| 
 33038| 3649: ; preds = %3641
 33039|     ;; __self_0 = ptr %834
 33040|     ;; self = ptr %834
 33041|     ;; __arg1_0 = ptr %122
 33042|     ;; other = ptr %122
 33045|  %3650 = load i64, ptr %837, , !!8                                                                                     ;L1878<2123<1127<511
 33046|  %3651 = load i64, ptr %546, , !!8                                                                                     ;L1878<2123<1127<511
 33047|  %3652 = icmp eq i64 %3650, %3651                                                                                      ;L1878<2123<1127<511
 33048|  br i1 %3652, label %3653, label %3645                                                                                 ;L511
 33049| 
 33050| 3653: ; preds = %3649
 33051|  %3654 = gep %834, i64 104                                                                                             ;L512
 33052|  %3655 = load i64, ptr %3654, , !!8                                                                                    ;L512
 33053|  %3656 = icmp eq i64 %3655, 2                                                                                          ;L512
 33054|  br i1 %3656, label %3871, label %3672                                                                                 ;L512
 33055| 
 33056| 3657: ; preds = %3645
 33057|     ;; info = ptr %834
 33061|     ;; attacker = ptr %834
 33062|     ;; target = ptr %122
 33063|     ;; seed = ptr %35
 33064|     ;; tick = ptr %34
 33065|     ;; key = ptr %33
 33067|  %3658 = load ptr, ptr %118, , !!46783, !!8, !!8                                                                       ;L131<526
 33068|  %3659 = load ptr, ptr %152, , !!46783, !!8, !!8                                                                       ;L131<526
 33069|  %3660 = gep %3659, i64 32                                                                                             ;L131<526
 33070|  %3661 = load ptr, ptr %3660, , !!46783, !!8                                                                           ;L131<526
 33071|  %3662 = invoke i64 %3661(ptr %3658)
 33072|  to label %3663 unwind label %384                                                                                      ;L131<526
 33073| 
 33074| 3663: ; preds = %3657
 33075|  store i64 %3662, ptr %35, , !!46783                                                                                   ;L131<526
 33077|  %3664 = gep %3659, i64 40                                                                                             ;L132<526
 33078|  %3665 = load ptr, ptr %3664, , !!46783, !!8                                                                           ;L132<526
 33079|  %3666 = invoke i64 %3665(ptr %3658)
 33080|  to label %3667 unwind label %384                                                                                      ;L132<526
 33081| 
 33082| 3667: ; preds = %3663
 33083|  store i64 %3666, ptr %34, , !!46783                                                                                   ;L132<526
 33085|  %3668 = gep %834, i64 1472                                                                                            ;L133<526
 33086|  %3669 = load i64, ptr %3668, , !!46775, !!8                                                                           ;L133<526
 33087|  %3670 = load i64, ptr %231, , !!46771, !!8                                                                            ;L133<526
 33088|  store i64 %3669, ptr %33, , !!46783                                                                                   ;L133<526
 33089|  store i64 %3670, ptr %624, , !!46783                                                                                  ;L133<526
 33091|  store ptr %35, ptr %32, , !!46783                                                                                     ;L134<526
 33092|  store ptr %34, ptr %625, , !!46783                                                                                    ;L134<526
 33093|  store ptr %33, ptr %626, , !!46783                                                                                    ;L134<526
 33094|  store ptr %834, ptr %627, , !!46783                                                                                   ;L134<526
 33095|  store ptr %135, ptr %628, , !!46783                                                                                   ;L134<526
 33096|  store ptr %122, ptr %629, , !!46783                                                                                   ;L134<526
 33097|  %3671 = invoke i64 @core::cell7RefCellNtNtCshdEBA0ozCnw_7game_ai13position_eval17AttackDamageCacheEE4withNCNvB1x_29expected_attack_damage_cached0jEB1z_(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.119, ptr %32)
 33098|  to label %3677 unwind label %384                                                                                      ;L134<526
 33099| 
 33100| 3672: ; preds = %3921, %3920, %3866, %3745, %3653, %3645
 33101|  %3673 = phi i64 [ %3925, %3921 ], [ %638, %3920 ], [ %638, %3645 ], [ %638, %3866 ], [ %638, %3745 ], [ %638, %3653 ] ;L0
 33102|  %3674 = phi i64 [ %639, %3921 ], [ %639, %3920 ], [ %639, %3645 ], [ %3867, %3866 ], [ %639, %3745 ], [ %639, %3653 ] ;L399
 33103|  %3675 = phi i64 [ %640, %3921 ], [ %640, %3920 ], [ %640, %3645 ], [ %3869, %3866 ], [ %640, %3745 ], [ %640, %3653 ] ;L0
 33104|  %3676 = phi i64 [ %641, %3921 ], [ %641, %3920 ], [ %641, %3645 ], [ %3870, %3866 ], [ %641, %3745 ], [ %641, %3653 ] ;L0
 33105|     ;; score[0..+8] = i64 %3675
 33106|     ;; score[0..+8] = i64 %3675
 33107|     ;; score[8..+8] = i64 %3674
 33108|     ;; score[8..+8] = i64 %3674
 33109|     ;; score[16..+8] = i64 %3673
 33110|     ;; score[16..+8] = i64 %3673
 33111|     ;; tower_well_risk = i64 %3676
 33112|  br label %637                                                                                                         ;L507
 33113| 
 33114| 3677: ; preds = %3667
 33119|     ;; damage = i64 %3671
 33120|  %3678 = load ptr, ptr %630, , !!8, !!8                                                                                ;L527
 33121|  %3679 = gep %3678, i64 4856                                                                                           ;L527
 33122|  %3680 = load i64, ptr %3679, , !!8                                                                                    ;L527
 33123|  %3681 = mul i64 %3680, %3671                                                                                          ;L527
 33124|  %3682 = invoke i64 @gc::simulation6entityNtB5_6Entity15attack_cooltime(ptr %834)
 33125|  to label %3683 unwind label %384                                                                                      ;L527
 33126| 
 33127| 3683: ; preds = %3677
 33128|  %3684 = icmp eq i64 %3682, 0                                                                                          ;L527
 33129|  br i1 %3684, label %3685, label %3686                                                                                 ;L527
 33130| 
 33131| 3685: ; preds = %3683
 33132|  invoke void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.102) #25
 33133|  to label %127 unwind label %384                                                                                       ;L527
 33134| 
 33135| 3686: ; preds = %3683
 33136|  %3687 = udiv i64 %3681, %3682                                                                                         ;L527
 33137|  %3688 = add i64 %3687, %3671                                                                                          ;L527
 33138|     ;; damage = i64 %3688
 33139|     ;; x = i64 %3688
 33140|     ;; inv_hp_q32 = i64 %149
 33141|  %3689 = zext i64 %3688 to i128                                                                                        ;L387<528
 33142|  %3690 = mul nuw nsw i128 %335, %3689                                                                                  ;L387<528
 33143|  %3691 = lshr i128 %3690, 32                                                                                           ;L387<528
 33144|     ;; self = i128 %3691
 33145|     ;; other = i128 150
 33146|  %3692 = call i128 @llvm.umin.i128(i128 %3691, i128 150)                                                               ;L1078<387<528
 33147|  %3693 = trunc nuw nsw i128 %3692 to i64                                                                               ;L387<528
 33148|     ;; ratio = i64 %3693
 33149|  %3694 = gep %834, i64 1184                                                                                            ;L26<529
 33150|  %3695 = load i64, ptr %3694, , !!8                                                                                    ;L26<529
 33151|  %3696 = gep %834, i64 1192                                                                                            ;L26<529
 33152|  %3697 = load i64, ptr %3696, , !!8                                                                                    ;L26<529
 33153|  %3698 = gep %834, i64 1480                                                                                            ;L26<529
 33154|  %3699 = load i64, ptr %3698, , !!8                                                                                    ;L26<529
 33155|  %3700 = gep %834, i64 1080                                                                                            ;L26<529
 33156|  %3701 = load i64, ptr %3700, , !!8                                                                                    ;L26<529
 33157|  %3702 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %3637, ptr %834, ptr %122)
 33158|  to label %3703 unwind label %384                                                                                      ;L529
 33159| 
 33160| 3703: ; preds = %3686
 33161|  %3704 = add i64 %3699, -1                                                                                             ;L26<529
 33162|  %3705 = mul i64 %3704, %3697                                                                                          ;L26<529
 33163|  %3706 = gep %834, i64 1136                                                                                            ;L1511<530
 33164|  %3707 = load i32, ptr %3706, , !!8                                                                                    ;L1511<530
 33165|     ;; mult = i32 %3707
 33166|  %3708 = icmp eq i32 %3707, 0                                                                                          ;L1512<530
 33167|  br i1 %3708, label %3709, label %3712                                                                                 ;L1512<530
 33168| 
 33169| 3709: ; preds = %3703
 33170|  %3710 = gep %834, i64 1664                                                                                            ;L1513<530
 33171|  %3711 = load i64, ptr %3710, , !!8                                                                                    ;L1513<530
 33172|  br label %3719                                                                                                        ;L1512<530
 33173| 
 33174| 3712: ; preds = %3703
 33175|  %3713 = sext i32 %3707 to i64                                                                                         ;L1511<530
 33176|     ;; mult = i64 %3713
 33177|  %3714 = gep %834, i64 1664                                                                                            ;L1515<530
 33178|  %3715 = load i64, ptr %3714, , !!8                                                                                    ;L1515<530
 33179|  %3716 = add nsw i64 %3713, 100                                                                                        ;L1515<530
 33180|  %3717 = mul i64 %3715, %3716                                                                                          ;L1515<530
 33181|  %3718 = udiv i64 %3717, 100                                                                                           ;L1515<530
 33182|  br label %3719                                                                                                        ;L1512<530
 33183| 
 33184| 3719: ; preds = %3712, %3709
 33185|  %3720 = phi i64 [ %3711, %3709 ], [ %3718, %3712 ]                                                                    ;L0<530
 33186|  %3721 = load i32, ptr %342, , !!8                                                                                     ;L1511<530
 33187|     ;; mult = i32 %3721
 33188|  %3722 = icmp eq i32 %3721, 0                                                                                          ;L1512<530
 33189|  br i1 %3722, label %3723, label %3725                                                                                 ;L1512<530
 33190| 
 33191| 3723: ; preds = %3719
 33192|  %3724 = load i64, ptr %343, , !!8                                                                                     ;L1513<530
 33193|  br label %3731                                                                                                        ;L1512<530
 33194| 
 33195| 3725: ; preds = %3719
 33196|  %3726 = sext i32 %3721 to i64                                                                                         ;L1511<530
 33197|     ;; mult = i64 %3726
 33198|  %3727 = load i64, ptr %343, , !!8                                                                                     ;L1515<530
 33199|  %3728 = add nsw i64 %3726, 100                                                                                        ;L1515<530
 33200|  %3729 = mul i64 %3727, %3728                                                                                          ;L1515<530
 33201|  %3730 = udiv i64 %3729, 100                                                                                           ;L1515<530
 33202|  br label %3731                                                                                                        ;L1512<530
 33203| 
 33204| 3731: ; preds = %3725, %3723
 33205|  %3732 = phi i64 [ %3724, %3723 ], [ %3730, %3725 ]                                                                    ;L0<530
 33206|  %3733 = add i64 %3701, %3695                                                                                          ;L26<529
 33207|  %3734 = add i64 %3733, %3705                                                                                          ;L26<529
 33208|  %3735 = add i64 %3734, %3702                                                                                          ;L529
 33209|  %3736 = add i64 %3735, %3720                                                                                          ;L529
 33210|  %3737 = add i64 %3736, %3732                                                                                          ;L529
 33211|     ;; range = i64 %3737
 33212|  %3738 = gep %834, i64 1632                                                                                            ;L532
 33213|  %3739 = load i64, ptr %3738, , !!8                                                                                    ;L532
 33214|  %3740 = gep %834, i64 1640                                                                                            ;L532
 33215|  %3741 = load i64, ptr %3740, , !!8                                                                                    ;L532
 33216|  %3742 = load i64, ptr %100, , !!8                                                                                     ;L532
 33217|  %3743 = load i64, ptr %99, , !!8                                                                                      ;L532
 33218|  %3744 = invoke zeroext i1 @gc::simulation6effectNtB2_6Effect14is_in_range_ex(ptr %3637, ptr %834, ptr %122, i64 %3739, i64 %3741, i64 %3742, i64 %3743, i64 18000)
 33219|  to label %3745 unwind label %384                                                                                      ;L532
 33220| 
 33221| 3745: ; preds = %3731
 33222|  br i1 %3744, label %3746, label %3672                                                                                 ;L532
 33223| 
 33224| 3746: ; preds = %3745
 33225|  %3747 = invoke { i64, i64 } @ai::tower_discipline16v47_siege_stance(i64 %1, ptr %3, ptr %2, ptr %834)
 33226|  to label %3748 unwind label %384                                                                                      ;L536
 33227| 
 33228| 3748: ; preds = %3746
 33229|  %3749 = extractvalue { i64, i64 } %3747, 0                                                                            ;L536
 33230|     ;; v47_soaker[0..+8] = i64 %3749
 33232|     ;; self = ptr undef
 33233|  %3750 = load i64, ptr %231, , !!8                                                                                     ;L537
 33235|  %3751 = trunc nuw i64 %3749 to i1                                                                                     ;L2439<537
 33236|  %3752 = extractvalue { i64, i64 } %3747, 1                                                                            ;L2439<537
 33237|  %3753 = icmp eq i64 %3752, %3750                                                                                      ;L2439<537
 33238|  %3754 = select i1 %3751, i1 %3753, i1 false                                                                           ;L2439<537
 33240|  %3755 = gep %834, i64 136                                                                                             ;L539
 33241|  %3756 = load i64, ptr %3755,                                                                                          ;L539
 33242|     ;; self[0..+8] = i64 %3756
 33245|  %3757 = trunc nuw i64 %3756 to i1                                                                                     ;L1161<539
 33246|  br i1 %3757, label %3784, label %3758                                                                                 ;L1161<539
 33247| 
 33248| 3758: ; preds = %3784, %3748
 33249|  %3759 = load ptr, ptr %97, , !!8, !!8                                                                                 ;L544
 33250|  %3760 = load ptr, ptr %158,                                                                                           ;L544
 33251|  %3761 = load i64, ptr %3738, , !!8                                                                                    ;L544
 33252|  %3762 = load i64, ptr %3740,                                                                                          ;L544
 33256|  %3763 = load i64, ptr %3759, , !!8                                                                                    ;L395<544
 33257|     ;; a = i64 %3763
 33258|     ;; self = i64 %3763
 33259|     ;; b = i64 %3761
 33260|     ;; other = i64 %3761
 33261|  %3764 = icmp ult i64 %3763, %3761                                                                                     ;L3147<8<395<544
 33262|  %3765 = sub nuw i64 %3761, %3763                                                                                      ;L3147<8<395<544
 33263|  %3766 = sub nuw i64 %3763, %3761                                                                                      ;L3147<8<395<544
 33264|  %3767 = select i1 %3764, i64 %3765, i64 %3766                                                                         ;L3147<8<395<544
 33265|     ;; rhs = i64 %3767
 33266|     ;; rhs = i64 %3767
 33267|     ;; rhs = i64 %3767
 33268|     ;; diff = i64 %3767
 33269|     ;; self = i64 %3767
 33270|     ;; self = i64 %3767
 33271|     ;; self = i64 %3767
 33272|  %3768 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %3767, i64 %3767)                                           ;L3178<1288<2517<9<395<544
 33273|  %3769 = extractvalue { i64, i1 } %3768, 0                                                                             ;L3178<1288<2517<9<395<544
 33274|  %3770 = extractvalue { i64, i1 } %3768, 1                                                                             ;L3178<1288<2517<9<395<544
 33275|     ;; a = i64 %3769
 33276|     ;; self = i64 %3769
 33277|     ;; b = i1 %3770
 33278|     ;; b = i1 %3770
 33279|  br i1 %3770, label %3771, label %3772                                                                                 ;L459<1289<2517<9<395<544
 33280| 
 33281| 3771: ; preds = %3758
 33282|     ;; a = i64 -1
 33283|     ;; self = i64 -1
 33284|  br label %3772                                                                                                        ;L2519<9<395<544
 33285| 
 33286| 3772: ; preds = %3771, %3758
 33287|  %3773 = phi i64 [ -1, %3771 ], [ %3769, %3758 ]                                                                       ;L0<9<395<544
 33288|     ;; self = i64 %3773
 33289|     ;; a = i64 %3773
 33290|  %3774 = icmp ne ptr %3760, null
 33291|  call void @llvm.assume(i1 %3774)
 33292|  %3775 = load i64, ptr %3760, , !!8                                                                                    ;L395<544
 33293|     ;; a = i64 %3775
 33294|     ;; self = i64 %3775
 33295|     ;; b = i64 %3762
 33296|     ;; other = i64 %3762
 33297|  %3776 = icmp ult i64 %3775, %3762                                                                                     ;L3147<8<395<544
 33298|  %3777 = sub nuw i64 %3762, %3775                                                                                      ;L3147<8<395<544
 33299|  %3778 = sub nuw i64 %3775, %3762                                                                                      ;L3147<8<395<544
 33300|  %3779 = select i1 %3776, i64 %3777, i64 %3778                                                                         ;L3147<8<395<544
 33301|     ;; rhs = i64 %3779
 33302|     ;; rhs = i64 %3779
 33303|     ;; rhs = i64 %3779
 33304|     ;; diff = i64 %3779
 33305|     ;; self = i64 %3779
 33306|     ;; self = i64 %3779
 33307|     ;; self = i64 %3779
 33308|  %3780 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %3779, i64 %3779)                                           ;L3178<1288<2517<9<395<544
 33309|  %3781 = extractvalue { i64, i1 } %3780, 0                                                                             ;L3178<1288<2517<9<395<544
 33310|  %3782 = extractvalue { i64, i1 } %3780, 1                                                                             ;L3178<1288<2517<9<395<544
 33311|     ;; a = i64 %3781
 33312|     ;; rhs = i64 %3781
 33313|     ;; b = i1 %3782
 33314|     ;; b = i1 %3782
 33315|  br i1 %3782, label %3783, label %3793                                                                                 ;L459<1289<2517<9<395<544
 33316| 
 33317| 3783: ; preds = %3772
 33318|     ;; a = i64 -1
 33319|     ;; rhs = i64 -1
 33320|  br label %3793                                                                                                        ;L2519<9<395<544
 33321| 
 33322| 3784: ; preds = %3748
 33323|  %3785 = gep %834, i64 152                                                                                             ;L539
 33324|  %3786 = load i64, ptr %3785,                                                                                          ;L539
 33325|     ;; self[16..+8] = i64 %3786
 33327|     ;; self = ptr undef
 33329|     ;; l = ptr undef
 33330|     ;; self = ptr undef
 33333|  %3787 = icmp eq i64 %3786, %3750                                                                                      ;L1878<2440<539
 33334|  br i1 %3787, label %3788, label %3758                                                                                 ;L539
 33335| 
 33336| 3788: ; preds = %3784
 33337|  %3789 = trunc nuw i128 %3692 to i8                                                                                    ;L540
 33338|  %3790 = udiv i8 %3789, 3                                                                                              ;L540
 33339|  %3791 = zext nneg i8 %3790 to i64                                                                                     ;L540
 33340|  %3792 = select i1 %3754, i64 %3791, i64 %3693                                                                         ;L540
 33341|     ;; score[0..+8] = !DIArgList(i64 %640, i64 %3792)
 33342|     ;; score[0..+8] = !DIArgList(i64 %640, i64 %3792)
 33343|     ;; tower_well_risk = !DIArgList(i64 %641, i64 %3792)
 33344|  br label %3866                                                                                                        ;L539
 33345| 
 33346| 3793: ; preds = %3783, %3772
 33347|  %3794 = phi i64 [ -1, %3783 ], [ %3781, %3772 ]                                                                       ;L0<9<395<544
 33348|     ;; rhs = i64 %3794
 33349|     ;; a = i64 %3794
 33350|  %3795 = call i64 @llvm.uadd.sat.i64(i64 %3773, i64 %3794)                                                             ;L2428<395<544
 33351|     ;; dist = i64 %3795
 33352|  %3796 = lshr i64 %3737, 1                                                                                             ;L545
 33353|  %3797 = mul i64 %3796, %3796                                                                                          ;L545
 33354|  %3798 = icmp ult i64 %3795, %3797                                                                                     ;L545
 33355|  %3799 = select i1 %3798, i64 100, i64 50                                                                              ;L545
 33356|     ;; coef = i64 %3799
 33357|     ;; risk = i64 0
 33358|  br i1 %3754, label %3802, label %3800                                                                                 ;L547
 33359| 
 33360| 3800: ; preds = %3793
 33361|     ;; self = ptr undef
 33362|  %3801 = icmp eq i64 %3749, 1                                                                                          ;L549
 33363|  br i1 %3801, label %3806, label %3829                                                                                 ;L549
 33364| 
 33365| 3802: ; preds = %3793
 33366|  %3803 = trunc nuw i128 %3692 to i8                                                                                    ;L548
 33367|  %3804 = udiv i8 %3803, 3                                                                                              ;L548
 33368|  %3805 = zext nneg i8 %3804 to i64                                                                                     ;L548
 33369|     ;; risk = i64 %3805
 33370|  br label %3863                                                                                                        ;L547
 33371| 
 33372| 3806: ; preds = %3800
 33374|     ;; data = ptr %3
 33375|     ;; tower = ptr %834
 33376|     ;; my_champion_id = i64 %3750
 33377|  %3807 = load i64, ptr %3646, , !!46891, !!8                                                                           ;L598<549
 33378|  %3808 = icmp eq i64 %3807, 2                                                                                          ;L598<549
 33379|     ;; info = ptr %834
 33380|  %3809 = and i1 %3808, %3757                                                                                           ;L598<549
 33381|  br i1 %3809, label %3810, label %3829                                                                                 ;L598<549
 33382| 
 33383| 3810: ; preds = %3806
 33384|  %3811 = gep %834, i64 152                                                                                             ;L601<549
 33385|  %3812 = load i64, ptr %3811, , !!46891, !!8                                                                           ;L601<549
 33386|     ;; target_id = i64 %3812
 33387|  %3813 = icmp eq i64 %3812, %3750                                                                                      ;L604<549
 33388|  br i1 %3813, label %3829, label %3814                                                                                 ;L604<549
 33389| 
 33390| 3814: ; preds = %3810
 33391|  %3815 = load ptr, ptr %118, , !!46898, !!8, !!8                                                                       ;L607<549
 33392|  %3816 = load ptr, ptr %152, , !!46898, !!8, !!8                                                                       ;L607<549
 33393|  %3817 = gep %3816, i64 496                                                                                            ;L607<549
 33394|  %3818 = load ptr, ptr %3817, , !!46898, !!8                                                                           ;L607<549
 33395|  %3819 = invoke ptr %3818(ptr %3815, i64 %3812)
 33396|  to label %3820 unwind label %384                                                                                      ;L607<549
 33397| 
 33398| 3820: ; preds = %3814
 33399|  %3821 = icmp eq ptr %3819, null                                                                                       ;L607<549
 33400|     ;; target = ptr %3819
 33401|  %3822 = load i32, ptr %3638, , !!46891
 33402|  %3823 = icmp eq i32 %3822, -1
 33403|  %3824 = select i1 %3821, i1 true, i1 %3823                                                                            ;L607<549
 33404|  br i1 %3824, label %3829, label %3825                                                                                 ;L607<549
 33405| 
 33406| 3825: ; preds = %3820
 33407|     ;; attack = ptr %3637
 33408|  %3826 = gep %3819, i64 1648                                                                                           ;L613<549
 33409|  %3827 = load i64, ptr %3826, , !!46898, !!8                                                                           ;L613<549
 33410|  %3828 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %3637, ptr %135, ptr %834, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %3819)
 33411|  to label %3835 unwind label %384                                                                                      ;L613<549
 33412| 
 33413| 3829: ; preds = %3820, %3810, %3806, %3800
 33414|     ;; self = ptr %95
 33415|     ;; self = ptr %95
 33416|  %3830 = load i64, ptr %200, , !!8                                                                                     ;L1617<1636<551
 33417|  %3831 = icmp eq i64 %3830, 0                                                                                          ;L551
 33418|  br i1 %3831, label %3842, label %3844                                                                                 ;L551
 33419| 
 33420| 3832: ; preds = %3835
 33421|     ;; self = ptr %95
 33422|     ;; self = ptr %95
 33423|  %3833 = load i64, ptr %200, , !!8                                                                                     ;L1617<1636<551
 33424|  %3834 = icmp eq i64 %3833, 0                                                                                          ;L551
 33425|  br i1 %3834, label %3842, label %3861                                                                                 ;L551
 33426| 
 33427| 3835: ; preds = %3825
 33428|  %3836 = icmp ugt i64 %3827, %3828                                                                                     ;L613<549
 33429|  br i1 %3836, label %3837, label %3832                                                                                 ;L549
 33430| 
 33431| 3837: ; preds = %3835
 33432|  %3838 = trunc nuw nsw i64 %3799 to i16                                                                                ;L550
 33433|  %3839 = mul nuw nsw i16 %3838, 3                                                                                      ;L550
 33434|  %3840 = udiv i16 %3839, 100                                                                                           ;L550
 33435|  %3841 = zext nneg i16 %3840 to i64                                                                                    ;L550
 33436|     ;; risk = i64 %3841
 33437|  br label %3863                                                                                                        ;L549
 33438| 
 33439| 3842: ; preds = %3832, %3829
 33440|  %3843 = icmp ult i64 %3636, 2                                                                                         ;L557
 33441|  br i1 %3843, label %3863, label %3846                                                                                 ;L557
 33442| 
 33443| 3844: ; preds = %3829
 33446|  %3845 = icmp eq i64 %3756, 1                                                                                          ;L430<682<552
 33447|  br i1 %3845, label %3861, label %3863                                                                                 ;L552
 33448| 
 33449| 3846: ; preds = %3842
 33450|  %3847 = icmp eq i64 %3636, 2                                                                                          ;L559
 33451|  br i1 %3847, label %3848, label %3856                                                                                 ;L559
 33452| 
 33453| 3848: ; preds = %3846
 33454|  %3849 = trunc nuw nsw i128 %3692 to i16                                                                               ;L560
 33455|  %3850 = shl nuw nsw i16 %3849, 1                                                                                      ;L560
 33456|  %3851 = udiv i16 %3850, 3                                                                                             ;L560
 33457|  %3852 = trunc nuw nsw i64 %3799 to i16                                                                                ;L560
 33458|  %3853 = mul nuw nsw i16 %3851, %3852                                                                                  ;L560
 33459|  %3854 = udiv i16 %3853, 100                                                                                           ;L560
 33460|  %3855 = zext nneg i16 %3854 to i64                                                                                    ;L560
 33461|     ;; risk = i64 %3855
 33462|  br label %3863                                                                                                        ;L559
 33463| 
 33464| 3856: ; preds = %3846
 33465|  %3857 = trunc nuw nsw i64 %3799 to i16                                                                                ;L562
 33466|  %3858 = mul nuw nsw i16 %3857, 3                                                                                      ;L562
 33467|  %3859 = udiv i16 %3858, 100                                                                                           ;L562
 33468|  %3860 = zext nneg i16 %3859 to i64                                                                                    ;L562
 33469|     ;; risk = i64 %3860
 33470|  br label %3863                                                                                                        ;L559
 33471| 
 33472| 3861: ; preds = %3844, %3832
 33473|  %3862 = lshr i64 %3693, 1                                                                                             ;L555
 33474|     ;; risk = i64 %3862
 33475|  br label %3863                                                                                                        ;L552
 33476| 
 33477| 3863: ; preds = %3861, %3856, %3848, %3844, %3842, %3837, %3802
 33478|  %3864 = phi i64 [ %3805, %3802 ], [ %3841, %3837 ], [ %3862, %3861 ], [ %3855, %3848 ], [ %3860, %3856 ], [ %3693, %3842 ], [ %3693, %3844 ] ;L0
 33479|     ;; risk = i64 %3864
 33480|     ;; score[0..+8] = !DIArgList(i64 %640, i64 %3864)
 33481|     ;; score[0..+8] = !DIArgList(i64 %640, i64 %3864)
 33482|  %3865 = add i64 %3864, %639                                                                                           ;L565
 33483|     ;; score[8..+8] = i64 %3865
 33484|     ;; score[8..+8] = i64 %3865
 33485|     ;; tower_well_risk = !DIArgList(i64 %641, i64 %3864)
 33486|  br label %3866                                                                                                        ;L539
 33487| 
 33488| 3866: ; preds = %3863, %3788
 33489|  %3867 = phi i64 [ %639, %3788 ], [ %3865, %3863 ]                                                                     ;L0
 33490|  %3868 = phi i64 [ %3792, %3788 ], [ %3864, %3863 ]
 33491|  %3869 = add i64 %3868, %640                                                                                           ;L0
 33492|     ;; score[0..+8] = i64 %3869
 33493|     ;; score[0..+8] = i64 %3869
 33494|     ;; score[8..+8] = i64 %3867
 33495|     ;; score[8..+8] = i64 %3867
 33496|  %3870 = add i64 %3868, %641                                                                                           ;L0
 33497|     ;; tower_well_risk = i64 %3870
 33498|  br label %3672                                                                                                        ;L532
 33499| 
 33500| 3871: ; preds = %3653
 33501|     ;; self = ptr %95
 33502|     ;; self = ptr %95
 33503|  %3872 = load ptr, ptr %95, , !!8, !!8                                                                                 ;L138<2073<513
 33504|     ;; p = ptr %3872
 33505|  %3873 = load i64, ptr %200, , !!8                                                                                     ;L2075<513
 33506|     ;; len = i64 %3873
 33507|     ;; count = i64 %3873
 33508|     ;; self[0..+8] = ptr %3872
 33509|     ;; slice[0..+8] = ptr %3872
 33510|     ;; self[8..+8] = i64 %3873
 33511|     ;; slice[8..+8] = i64 %3873
 33512|     ;; ptr = ptr %3872
 33513|     ;; self = ptr %3872
 33514|  %3874 = gepS, ptr, i64 }, ptr %3872, i64 %3873                                                                        ;L961<100<1042<513
 33515|     ;; predicate[0..+8] = ptr %3637
 33516|     ;; predicate[8..+8] = ptr %834
 33517|     ;; self = ptr undef
 33518|     ;; self = ptr undef
 33519|     ;; count = i64 1
 33520|  br label %3875                                                                                                        ;L348<514
 33521| 
 33522| 3875: ; preds = %3882, %3871
 33523|  %3876 = phi ptr [ %3883, %3882 ], [ %3872, %3871 ]
 33524|     ;; ptr = ptr %3876
 33525|     ;; self = ptr %3876
 33526|     ;; end_or_len = ptr %3874
 33529|  %3877 = icmp eq ptr %3876, %3874                                                                                      ;L1714<180<348<514
 33530|  br i1 %3877, label %642, label %3878                                                                                  ;L180<348<514
 33531| 
 33532| 3878: ; preds = %3875
 33533|     ;; x = ptr %3876
 33537|     ;; e = ptr %3876
 33538|  %3879 = gep %3876, i64 432                                                                                            ;L514<349<514
 33539|  %3880 = load ptr, ptr %3879, , !!46976, !!8, !!8                                                                      ;L514<349<514
 33540|  %3881 = invoke zeroext i1 @gc::simulation6effectNtB2_6Effect11is_in_range(ptr %3637, ptr %834, ptr %3880)
 33541|  to label %3882 unwind label %384                                                                                      ;L514<349<514
 33542| 
 33543| 3882: ; preds = %3878
 33544|  %3883 = gep %3876, i64 448                                                                                            ;L656<185<348<514
 33545|  br i1 %3881, label %3884, label %3875                                                                                 ;L349<514
 33546| 
 33547| 3884: ; preds = %3882
 33548|     ;; any_enemy_in_tower_range = ptr %3876
 33549|     ;; target = ptr %3876
 33550|  %3885 = load ptr, ptr %3879, , !!8, !!8                                                                               ;L518
 33554|     ;; attacker = ptr %834
 33555|     ;; target = ptr %3885
 33556|     ;; seed = ptr %31
 33557|     ;; tick = ptr %30
 33558|     ;; key = ptr %29
 33560|  %3886 = load ptr, ptr %118, , !!46995, !!8, !!8                                                                       ;L131<518
 33561|  %3887 = load ptr, ptr %152, , !!46995, !!8, !!8                                                                       ;L131<518
 33562|  %3888 = gep %3887, i64 32                                                                                             ;L131<518
 33563|  %3889 = load ptr, ptr %3888, , !!46995, !!8                                                                           ;L131<518
 33564|  %3890 = invoke i64 %3889(ptr %3886)
 33565|  to label %3891 unwind label %384                                                                                      ;L131<518
 33566| 
 33567| 3891: ; preds = %3884
 33568|  store i64 %3890, ptr %31, , !!46995                                                                                   ;L131<518
 33570|  %3892 = gep %3887, i64 40                                                                                             ;L132<518
 33571|  %3893 = load ptr, ptr %3892, , !!46995, !!8                                                                           ;L132<518
 33572|  %3894 = invoke i64 %3893(ptr %3886)
 33573|  to label %3895 unwind label %384                                                                                      ;L132<518
 33574| 
 33575| 3895: ; preds = %3891
 33576|  store i64 %3894, ptr %30, , !!46995                                                                                   ;L132<518
 33578|  %3896 = gep %834, i64 1472                                                                                            ;L133<518
 33579|  %3897 = load i64, ptr %3896, , !!46987, !!8                                                                           ;L133<518
 33580|  %3898 = gep %3885, i64 1472                                                                                           ;L133<518
 33581|  %3899 = load i64, ptr %3898, , !!46984, !!8                                                                           ;L133<518
 33582|  store i64 %3897, ptr %29, , !!46995                                                                                   ;L133<518
 33583|  store i64 %3899, ptr %631, , !!46995                                                                                  ;L133<518
 33585|  store ptr %31, ptr %28, , !!46995                                                                                     ;L134<518
 33586|  store ptr %30, ptr %632, , !!46995                                                                                    ;L134<518
 33587|  store ptr %29, ptr %633, , !!46995                                                                                    ;L134<518
 33588|  store ptr %834, ptr %634, , !!46995                                                                                   ;L134<518
 33589|  store ptr %135, ptr %635, , !!46995                                                                                   ;L134<518
 33590|  store ptr %3885, ptr %636, , !!46995                                                                                  ;L134<518
 33591|  %3900 = invoke i64 @core::cell7RefCellNtNtCshdEBA0ozCnw_7game_ai13position_eval17AttackDamageCacheEE4withNCNvB1x_29expected_attack_damage_cached0jEB1z_(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.119, ptr %28)
 33592|  to label %3901 unwind label %384                                                                                      ;L134<518
 33593| 
 33594| 3901: ; preds = %3895
 33599|     ;; damage = i64 %3900
 33600|  %3902 = load ptr, ptr %630, , !!8, !!8                                                                                ;L519
 33601|  %3903 = gep %3902, i64 4856                                                                                           ;L519
 33602|  %3904 = load i64, ptr %3903, , !!8                                                                                    ;L519
 33603|  %3905 = mul i64 %3904, %3900                                                                                          ;L519
 33604|  %3906 = invoke i64 @gc::simulation6entityNtB5_6Entity15attack_cooltime(ptr %834)
 33605|  to label %3907 unwind label %384                                                                                      ;L519
 33606| 
 33607| 3907: ; preds = %3901
 33608|  %3908 = icmp eq i64 %3906, 0                                                                                          ;L519
 33609|  br i1 %3908, label %3909, label %3910                                                                                 ;L519
 33610| 
 33611| 3909: ; preds = %3907
 33612|  invoke void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.103) #25
 33613|  to label %127 unwind label %384                                                                                       ;L519
 33614| 
 33615| 3910: ; preds = %3907
 33616|  %3911 = udiv i64 %3905, %3906                                                                                         ;L519
 33617|  %3912 = add i64 %3911, %3900                                                                                          ;L519
 33618|     ;; damage = i64 %3912
 33619|     ;; x = i64 %3912
 33620|     ;; inv_hp_q32 = i64 %149
 33621|  %3913 = zext i64 %3912 to i128                                                                                        ;L387<520
 33622|  %3914 = mul nuw nsw i128 %335, %3913                                                                                  ;L387<520
 33623|  %3915 = lshr i128 %3914, 32                                                                                           ;L387<520
 33624|     ;; self = i128 %3915
 33625|     ;; other = i128 150
 33626|  %3916 = call i128 @llvm.umin.i128(i128 %3915, i128 150)                                                               ;L1078<387<520
 33627|     ;; ratio = i128 %3916
 33628|  %3917 = load i64, ptr %100, , !!8                                                                                     ;L521
 33629|  %3918 = load i64, ptr %99, , !!8                                                                                      ;L521
 33630|  %3919 = invoke zeroext i1 @gc::simulation6effectNtB2_6Effect15is_in_range_pos(ptr %3637, ptr %834, ptr %122, i64 %3917, i64 %3918)
 33631|  to label %3920 unwind label %384                                                                                      ;L521
 33632| 
 33633| 3920: ; preds = %3910
 33634|  br i1 %3919, label %3921, label %3672                                                                                 ;L521
 33635| 
 33636| 3921: ; preds = %3920
 33637|  %3922 = trunc nuw i128 %3916 to i8                                                                                    ;L522
 33638|  %3923 = udiv i8 %3922, 3                                                                                              ;L522
 33639|  %3924 = zext nneg i8 %3923 to i64                                                                                     ;L522
 33640|  %3925 = add i64 %638, %3924                                                                                           ;L522
 33641|     ;; score[16..+8] = i64 %3925
 33642|     ;; score[16..+8] = i64 %3925
 33643|  br label %3672                                                                                                        ;L521
 33644| 
 33645| 3926: ; preds = %384
 33646|  cleanupret from %386 unwind label %280
 33647| 
 33648| 3927: ; preds = %384
 33649|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %89) #27 [ "funclet"(token %386) ] ;L1181
 33650|  cleanupret from %386 unwind label %280                                                                                ;L1181
 33651| 
 33652| 3928: ; preds = %274
 33653|  %3929 = getelementptr ptr, ptr %120, i64 %256                                                                         ;L430
 33654|  %3930 = load ptr, ptr %3929, , !!8                                                                                    ;L430
 33655|     ;; self = ptr %3930
 33656|  %3931 = icmp eq ptr %3930, null                                                                                       ;L1011<430
 33657|  br i1 %3931, label %3960, label %3932                                                                                 ;L1011<430
 33658| 
 33659| 3932: ; preds = %3928
 33660|     ;; e = ptr %3930
 33661|  %3933 = load ptr, ptr %97, , !!8, !!8                                                                                 ;L431
 33662|  %3934 = load ptr, ptr %158,                                                                                           ;L431
 33663|  %3935 = gep %3930, i64 1632                                                                                           ;L431
 33664|  %3936 = load i64, ptr %3935, , !!8                                                                                    ;L431
 33665|  %3937 = gep %3930, i64 1640                                                                                           ;L431
 33666|  %3938 = load i64, ptr %3937,                                                                                          ;L431
 33670|  %3939 = load i64, ptr %3933, , !!8                                                                                    ;L395<431
 33671|     ;; a = i64 %3939
 33672|     ;; self = i64 %3939
 33673|     ;; b = i64 %3936
 33674|     ;; other = i64 %3936
 33675|  %3940 = icmp ult i64 %3939, %3936                                                                                     ;L3147<8<395<431
 33676|  %3941 = sub nuw i64 %3936, %3939                                                                                      ;L3147<8<395<431
 33677|  %3942 = sub nuw i64 %3939, %3936                                                                                      ;L3147<8<395<431
 33678|  %3943 = select i1 %3940, i64 %3941, i64 %3942                                                                         ;L3147<8<395<431
 33679|     ;; rhs = i64 %3943
 33680|     ;; rhs = i64 %3943
 33681|     ;; rhs = i64 %3943
 33682|     ;; diff = i64 %3943
 33683|     ;; self = i64 %3943
 33684|     ;; self = i64 %3943
 33685|     ;; self = i64 %3943
 33686|  %3944 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %3943, i64 %3943)                                           ;L3178<1288<2517<9<395<431
 33687|  %3945 = extractvalue { i64, i1 } %3944, 0                                                                             ;L3178<1288<2517<9<395<431
 33688|  %3946 = extractvalue { i64, i1 } %3944, 1                                                                             ;L3178<1288<2517<9<395<431
 33689|     ;; a = i64 %3945
 33690|     ;; self = i64 %3945
 33691|     ;; b = i1 %3946
 33692|     ;; b = i1 %3946
 33693|  br i1 %3946, label %3947, label %3948                                                                                 ;L459<1289<2517<9<395<431
 33694| 
 33695| 3947: ; preds = %3932
 33696|     ;; a = i64 -1
 33697|     ;; self = i64 -1
 33698|  br label %3948                                                                                                        ;L2519<9<395<431
 33699| 
 33700| 3948: ; preds = %3947, %3932
 33701|  %3949 = phi i64 [ -1, %3947 ], [ %3945, %3932 ]                                                                       ;L0<9<395<431
 33702|     ;; self = i64 %3949
 33703|     ;; a = i64 %3949
 33704|  %3950 = icmp ne ptr %3934, null
 33705|  call void @llvm.assume(i1 %3950)
 33706|  %3951 = load i64, ptr %3934, , !!8                                                                                    ;L395<431
 33707|     ;; a = i64 %3951
 33708|     ;; self = i64 %3951
 33709|     ;; b = i64 %3938
 33710|     ;; other = i64 %3938
 33711|  %3952 = icmp ult i64 %3951, %3938                                                                                     ;L3147<8<395<431
 33712|  %3953 = sub nuw i64 %3938, %3951                                                                                      ;L3147<8<395<431
 33713|  %3954 = sub nuw i64 %3951, %3938                                                                                      ;L3147<8<395<431
 33714|  %3955 = select i1 %3952, i64 %3953, i64 %3954                                                                         ;L3147<8<395<431
 33715|     ;; rhs = i64 %3955
 33716|     ;; rhs = i64 %3955
 33717|     ;; rhs = i64 %3955
 33718|     ;; diff = i64 %3955
 33719|     ;; self = i64 %3955
 33720|     ;; self = i64 %3955
 33721|     ;; self = i64 %3955
 33722|  %3956 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %3955, i64 %3955)                                           ;L3178<1288<2517<9<395<431
 33723|  %3957 = extractvalue { i64, i1 } %3956, 0                                                                             ;L3178<1288<2517<9<395<431
 33724|  %3958 = extractvalue { i64, i1 } %3956, 1                                                                             ;L3178<1288<2517<9<395<431
 33725|     ;; a = i64 %3957
 33726|     ;; rhs = i64 %3957
 33727|     ;; b = i1 %3958
 33728|     ;; b = i1 %3958
 33729|  br i1 %3958, label %3959, label %3961                                                                                 ;L459<1289<2517<9<395<431
 33730| 
 33731| 3959: ; preds = %3948
 33732|     ;; a = i64 -1
 33733|     ;; rhs = i64 -1
 33734|  br label %3961                                                                                                        ;L2519<9<395<431
 33735| 
 33736| 3960: ; preds = %3928
 33737|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.105) #25
 33738|  to label %127 unwind label %280                                                                                       ;L1013<430
 33739| 
 33740| 3961: ; preds = %3959, %3948
 33741|  %3962 = phi i64 [ -1, %3959 ], [ %3957, %3948 ]                                                                       ;L0<9<395<431
 33742|     ;; rhs = i64 %3962
 33743|     ;; a = i64 %3962
 33744|  %3963 = call i64 @llvm.uadd.sat.i64(i64 %3949, i64 %3962)                                                             ;L2428<395<431
 33745|     ;; dist = i64 %3963
 33746|  %3964 = gep %3930, i64 1472                                                                                           ;L432
 33747|  %3965 = load i64, ptr %3964, , !!8                                                                                    ;L432
 33748|  %3966 = load i64, ptr %231, , !!8                                                                                     ;L432
 33749|  %3967 = icmp eq i64 %3965, %3966                                                                                      ;L432
 33750|  %3968 = icmp ugt i64 %3963, 40000000000                                                                               ;L432
 33751|  %3969 = or i1 %3968, %3967                                                                                            ;L432
 33752|  br i1 %3969, label %4015, label %3970                                                                                 ;L432
 33753| 
 33754| 3970: ; preds = %3961
 33755|  %3971 = getelementptr ptr, ptr %232, i64 %256                                                                         ;L436
 33756|  %3972 = load ptr, ptr %3971, , !!8                                                                                    ;L436
 33757|     ;; self = ptr %3972
 33758|  %3973 = icmp eq ptr %3972, null                                                                                       ;L1011<436
 33759|  br i1 %3973, label %3978, label %3974                                                                                 ;L1011<436
 33760| 
 33761| 3974: ; preds = %3970
 33762|     ;; eplayer = ptr %3972
 33763|  %3975 = gep %3972, i64 2352                                                                                           ;L437
 33764|  %3976 = load i64, ptr %3975, , !!8                                                                                    ;L437
 33765|  %3977 = icmp ult i64 %3976, 2                                                                                         ;L437
 33766|  br i1 %3977, label %3980, label %3979                                                                                 ;L437
 33767| 
 33768| 3978: ; preds = %3970
 33769|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.106) #25
 33770|  to label %127 unwind label %280                                                                                       ;L1013<436
 33771| 
 33772| 3979: ; preds = %3974
 33773|  invoke void @core::panicking18panic_bounds_check(i64 %3976, i64 2, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.107) #25
 33774|  to label %127 unwind label %280                                                                                       ;L437
 33775| 
 33776| 3980: ; preds = %3974
 33777|     ;; self = ptr %3972
 33778|  %3981 = gep %3972, i64 2496                                                                                           ;L581<437
 33779|  %3982 = load i32, ptr %3981, , !!8                                                                                    ;L581<437
 33780|  %3983 = zext nneg i32 %3982 to i64                                                                                    ;L581<437
 33781|  %3984 = getelementptr [5 x { [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64] }], ptr %164, i64 %3976 ;L437
 33782|  %3985 = gepS %3984, i64 %3983                                                                                         ;L437
 33783|     ;; ecache = ptr %3985
 33787|  store i64 %1, ptr %27, , !!47089
 33788|  store i8 %233, ptr %26, , !!47089
 33789|     ;; version = ptr %27
 33790|     ;; data = ptr %3
 33791|     ;; champ = ptr %122
 33792|     ;; e = ptr %3930
 33793|     ;; player = ptr %2
 33794|     ;; eplayer = ptr %3972
 33795|     ;; champ_cache = ptr %166
 33796|     ;; e_cache = ptr %3985
 33797|     ;; has_near_enemy = ptr %26
 33798|     ;; seed = ptr %25
 33799|     ;; tick = ptr %24
 33800|     ;; pt = ptr %23
 33801|     ;; pp = ptr %22
 33802|     ;; et = ptr %21
 33803|     ;; ep = ptr %20
 33804|     ;; hn = ptr %19
 33806|  %3986 = load ptr, ptr %118, , !!47089, !!8, !!8                                                                       ;L211<438
 33807|  %3987 = load ptr, ptr %152, , !!47089, !!8, !!8                                                                       ;L211<438
 33808|  %3988 = gep %3987, i64 32                                                                                             ;L211<438
 33809|  %3989 = load ptr, ptr %3988, , !!47089, !!8                                                                           ;L211<438
 33810|  %3990 = invoke i64 %3989(ptr %3986)
 33811|  to label %3991 unwind label %280                                                                                      ;L211<438
 33812| 
 33813| 3991: ; preds = %3980
 33814|  store i64 %3990, ptr %25, , !!47089                                                                                   ;L211<438
 33816|  %3992 = gep %3987, i64 40                                                                                             ;L212<438
 33817|  %3993 = load ptr, ptr %3992, , !!47089, !!8                                                                           ;L212<438
 33818|  %3994 = invoke i64 %3993(ptr %3986)
 33819|  to label %3995 unwind label %280                                                                                      ;L212<438
 33820| 
 33821| 3995: ; preds = %3991
 33822|  store i64 %3994, ptr %24, , !!47089                                                                                   ;L212<438
 33823|     ;; self = ptr %2
 33825|  store i64 %112, ptr %23, , !!47089                                                                                    ;L213<438
 33827|  store i64 %117, ptr %22, , !!47089                                                                                    ;L213<438
 33828|  %3996 = load i64, ptr %3975, , !!47140, !!8                                                                           ;L214<438
 33829|     ;; self = ptr %3972
 33830|  %3997 = load i32, ptr %3981, , !!47140, !!8                                                                           ;L581<214<438
 33831|  %3998 = zext nneg i32 %3997 to i64                                                                                    ;L581<214<438
 33833|  store i64 %3996, ptr %21, , !!47089                                                                                   ;L214<438
 33835|  store i64 %3998, ptr %20, , !!47089                                                                                   ;L214<438
 33837|  store i64 %234, ptr %19, , !!47089                                                                                    ;L215<438
 33839|  store ptr %25, ptr %18, , !!47089                                                                                     ;L216<438
 33840|  store ptr %24, ptr %235, , !!47089                                                                                    ;L216<438
 33841|  store ptr %23, ptr %236, , !!47089                                                                                    ;L216<438
 33842|  store ptr %22, ptr %237, , !!47089                                                                                    ;L216<438
 33843|  store ptr %21, ptr %238, , !!47089                                                                                    ;L216<438
 33844|  store ptr %20, ptr %239, , !!47089                                                                                    ;L216<438
 33845|  store ptr %19, ptr %240, , !!47089                                                                                    ;L216<438
 33846|  store ptr %27, ptr %241, , !!47089                                                                                    ;L216<438
 33847|  store ptr %122, ptr %242, , !!47089                                                                                   ;L216<438
 33848|  store ptr %3930, ptr %243, , !!47089                                                                                  ;L216<438
 33849|  store ptr %2, ptr %244, , !!47089                                                                                     ;L216<438
 33850|  store ptr %3972, ptr %245, , !!47089                                                                                  ;L216<438
 33851|  store ptr %166, ptr %246, , !!47089                                                                                   ;L216<438
 33852|  store ptr %3985, ptr %247, , !!47089                                                                                  ;L216<438
 33853|  store ptr %26, ptr %248, , !!47089                                                                                    ;L216<438
 33854|  invoke void @core::cell7RefCellNtNtCshdEBA0ozCnw_7game_ai13position_eval8EpcCacheEE4withNCNvB1x_31entity_positioning_cache_cached0NtNtB1z_15score_parameter22EntityPositioningCacheEB1z_(ptr sret([424 x i8]) %92, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.120, ptr %18)
 33855|  to label %3999 unwind label %280                                                                                      ;L216<438
 33856| 
 33857| 3999: ; preds = %3995
 33868|     ;; value[424..+8] = ptr %3972
 33869|     ;; src[424..+8] = ptr %3972
 33870|     ;; value[432..+8] = ptr %3930
 33871|     ;; src[432..+8] = ptr %3930
 33872|     ;; value[440..+8] = i64 %3963
 33873|     ;; src[440..+8] = i64 %3963
 33874|     ;; self = ptr %93
 33875|     ;; self = ptr %93
 33876|     ;; additional = i64 1
 33877|     ;; needed_extra_cap = i64 1
 33878|     ;; needed_extra_cap = i64 1
 33879|     ;; strategy = i8 1
 33880|  %4000 = load i64, ptr %230, , !!47188, !!8                                                                            ;L1428<439
 33881|     ;; self = ptr %93
 33882|  %4001 = load i64, ptr %229, , !!47188, !!8                                                                            ;L149<1428<439
 33883|  %4002 = icmp eq i64 %4000, %4001                                                                                      ;L1428<439
 33884|  br i1 %4002, label %4003, label %4006                                                                                 ;L1428<439
 33885| 
 33886| 4003: ; preds = %3999
 33887|     ;; self = ptr %93
 33888|     ;; self = ptr %93
 33889|     ;; self = ptr %93
 33890|     ;; used_cap = i64 %4000
 33891|     ;; used_cap = i64 %4000
 33892|  invoke void @ai::score_parameter22EntityPositioningCacheRNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player11PlayerStateRNtNtB2g_6entity6EntityyEE25reserve_internal_or_panicB18_(ptr %93, i64 %4000, i64 1, i1 zeroext true)
 33893|  to label %4004 unwind label %280                                                                                      ;L619<430<738<1429<439
 33894| 
 33895| 4004: ; preds = %4003
 33896|  %4005 = load i64, ptr %230, , !!47188                                                                                 ;L1432<439
 33897|  br label %4006                                                                                                        ;L1428<439
 33898| 
 33899| 4006: ; preds = %4004, %3999
 33900|  %4007 = phi i64 [ %4000, %3999 ], [ %4005, %4004 ]                                                                    ;L1432<439
 33901|     ;; self = ptr %93
 33902|  %4008 = load ptr, ptr %93, , !!47188, !!8, !!8                                                                        ;L138<1432<439
 33903|     ;; self = ptr %4008
 33904|     ;; count = i64 %4007
 33905|  %4009 = gepS, ptr, i64 }, ptr %4008, i64 %4007                                                                        ;L961<1432<439
 33906|     ;; end = ptr %4009
 33907|     ;; dst = ptr %4009
 33908|  call void @llvm.memcpy.p0.p0.i64(ptr %4009, ptr %92, i64 424, i1 false)                                               ;L1933<1433<439
 33909|  %4010 = gep %4009, i64 424                                                                                            ;L1933<1433<439
 33910|  store ptr %3972, ptr %4010,                                                                                           ;L1933<1433<439
 33911|  %4011 = gep %4009, i64 432                                                                                            ;L1933<1433<439
 33912|  store ptr %3930, ptr %4011,                                                                                           ;L1933<1433<439
 33913|  %4012 = gep %4009, i64 440                                                                                            ;L1933<1433<439
 33914|  store i64 %3963, ptr %4012,                                                                                           ;L1933<1433<439
 33915|  %4013 = load i64, ptr %230, , !!47188, !!8                                                                            ;L1434<439
 33916|  %4014 = add i64 %4013, 1                                                                                              ;L1434<439
 33917|  store i64 %4014, ptr %230, , !!47188                                                                                  ;L1434<439
 33918|  br label %4015                                                                                                        ;L426
 33919| 
 33920| 4015: ; preds = %4006, %3961, %274
 33921|  br label %255                                                                                                         ;L1916<900<985<426
 33922| 
 33923| 4016: ; preds = %249
 33924|  %4017 = getelementptr ptr, ptr %201, i64 %224                                                                         ;L412
 33925|  %4018 = load ptr, ptr %4017, , !!8                                                                                    ;L412
 33926|     ;; self = ptr %4018
 33927|  %4019 = icmp eq ptr %4018, null                                                                                       ;L1011<412
 33928|  br i1 %4019, label %4048, label %4020                                                                                 ;L1011<412
 33929| 
 33930| 4020: ; preds = %4016
 33931|     ;; e = ptr %4018
 33932|  %4021 = load ptr, ptr %97, , !!8, !!8                                                                                 ;L413
 33933|  %4022 = load ptr, ptr %158,                                                                                           ;L413
 33934|  %4023 = gep %4018, i64 1632                                                                                           ;L413
 33935|  %4024 = load i64, ptr %4023, , !!8                                                                                    ;L413
 33936|  %4025 = gep %4018, i64 1640                                                                                           ;L413
 33937|  %4026 = load i64, ptr %4025,                                                                                          ;L413
 33941|  %4027 = load i64, ptr %4021, , !!8                                                                                    ;L395<413
 33942|     ;; a = i64 %4027
 33943|     ;; self = i64 %4027
 33944|     ;; b = i64 %4024
 33945|     ;; other = i64 %4024
 33946|  %4028 = icmp ult i64 %4027, %4024                                                                                     ;L3147<8<395<413
 33947|  %4029 = sub nuw i64 %4024, %4027                                                                                      ;L3147<8<395<413
 33948|  %4030 = sub nuw i64 %4027, %4024                                                                                      ;L3147<8<395<413
 33949|  %4031 = select i1 %4028, i64 %4029, i64 %4030                                                                         ;L3147<8<395<413
 33950|     ;; rhs = i64 %4031
 33951|     ;; rhs = i64 %4031
 33952|     ;; rhs = i64 %4031
 33953|     ;; diff = i64 %4031
 33954|     ;; self = i64 %4031
 33955|     ;; self = i64 %4031
 33956|     ;; self = i64 %4031
 33957|  %4032 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %4031, i64 %4031)                                           ;L3178<1288<2517<9<395<413
 33958|  %4033 = extractvalue { i64, i1 } %4032, 0                                                                             ;L3178<1288<2517<9<395<413
 33959|  %4034 = extractvalue { i64, i1 } %4032, 1                                                                             ;L3178<1288<2517<9<395<413
 33960|     ;; a = i64 %4033
 33961|     ;; self = i64 %4033
 33962|     ;; b = i1 %4034
 33963|     ;; b = i1 %4034
 33964|  br i1 %4034, label %4035, label %4036                                                                                 ;L459<1289<2517<9<395<413
 33965| 
 33966| 4035: ; preds = %4020
 33967|     ;; a = i64 -1
 33968|     ;; self = i64 -1
 33969|  br label %4036                                                                                                        ;L2519<9<395<413
 33970| 
 33971| 4036: ; preds = %4035, %4020
 33972|  %4037 = phi i64 [ -1, %4035 ], [ %4033, %4020 ]                                                                       ;L0<9<395<413
 33973|     ;; self = i64 %4037
 33974|     ;; a = i64 %4037
 33975|  %4038 = icmp ne ptr %4022, null
 33976|  call void @llvm.assume(i1 %4038)
 33977|  %4039 = load i64, ptr %4022, , !!8                                                                                    ;L395<413
 33978|     ;; a = i64 %4039
 33979|     ;; self = i64 %4039
 33980|     ;; b = i64 %4026
 33981|     ;; other = i64 %4026
 33982|  %4040 = icmp ult i64 %4039, %4026                                                                                     ;L3147<8<395<413
 33983|  %4041 = sub nuw i64 %4026, %4039                                                                                      ;L3147<8<395<413
 33984|  %4042 = sub nuw i64 %4039, %4026                                                                                      ;L3147<8<395<413
 33985|  %4043 = select i1 %4040, i64 %4041, i64 %4042                                                                         ;L3147<8<395<413
 33986|     ;; rhs = i64 %4043
 33987|     ;; rhs = i64 %4043
 33988|     ;; rhs = i64 %4043
 33989|     ;; diff = i64 %4043
 33990|     ;; self = i64 %4043
 33991|     ;; self = i64 %4043
 33992|     ;; self = i64 %4043
 33993|  %4044 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %4043, i64 %4043)                                           ;L3178<1288<2517<9<395<413
 33994|  %4045 = extractvalue { i64, i1 } %4044, 0                                                                             ;L3178<1288<2517<9<395<413
 33995|  %4046 = extractvalue { i64, i1 } %4044, 1                                                                             ;L3178<1288<2517<9<395<413
 33996|     ;; a = i64 %4045
 33997|     ;; rhs = i64 %4045
 33998|     ;; b = i1 %4046
 33999|     ;; b = i1 %4046
 34000|  br i1 %4046, label %4047, label %4049                                                                                 ;L459<1289<2517<9<395<413
 34001| 
 34002| 4047: ; preds = %4036
 34003|     ;; a = i64 -1
 34004|     ;; rhs = i64 -1
 34005|  br label %4049                                                                                                        ;L2519<9<395<413
 34006| 
 34007| 4048: ; preds = %4016
 34008|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.108) #25
 34009|  to label %127 unwind label %3278                                                                                      ;L1013<412
 34010| 
 34011| 4049: ; preds = %4047, %4036
 34012|  %4050 = phi i64 [ -1, %4047 ], [ %4045, %4036 ]                                                                       ;L0<9<395<413
 34013|     ;; rhs = i64 %4050
 34014|     ;; a = i64 %4050
 34015|  %4051 = call i64 @llvm.uadd.sat.i64(i64 %4037, i64 %4050)                                                             ;L2428<395<413
 34016|     ;; dist = i64 %4051
 34017|  %4052 = icmp ugt i64 %4051, 40000000000                                                                               ;L414
 34018|  br i1 %4052, label %4098, label %4053                                                                                 ;L414
 34019| 
 34020| 4053: ; preds = %4049
 34021|  %4054 = getelementptr ptr, ptr %203, i64 %224                                                                         ;L418
 34022|  %4055 = load ptr, ptr %4054, , !!8                                                                                    ;L418
 34023|     ;; self = ptr %4055
 34024|  %4056 = icmp eq ptr %4055, null                                                                                       ;L1011<418
 34025|  br i1 %4056, label %4061, label %4057                                                                                 ;L1011<418
 34026| 
 34027| 4057: ; preds = %4053
 34028|     ;; eplayer = ptr %4055
 34029|  %4058 = gep %4055, i64 2352                                                                                           ;L419
 34030|  %4059 = load i64, ptr %4058, , !!8                                                                                    ;L419
 34031|  %4060 = icmp ult i64 %4059, 2                                                                                         ;L419
 34032|  br i1 %4060, label %4063, label %4062                                                                                 ;L419
 34033| 
 34034| 4061: ; preds = %4053
 34035|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.109) #25
 34036|  to label %127 unwind label %3278                                                                                      ;L1013<418
 34037| 
 34038| 4062: ; preds = %4057
 34039|  invoke void @core::panicking18panic_bounds_check(i64 %4059, i64 2, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.110) #25
 34040|  to label %127 unwind label %3278                                                                                      ;L419
 34041| 
 34042| 4063: ; preds = %4057
 34043|     ;; self = ptr %4055
 34044|  %4064 = gep %4055, i64 2496                                                                                           ;L581<419
 34045|  %4065 = load i32, ptr %4064, , !!8                                                                                    ;L581<419
 34046|  %4066 = zext nneg i32 %4065 to i64                                                                                    ;L581<419
 34047|  %4067 = getelementptr [5 x { [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64] }], ptr %164, i64 %4059 ;L419
 34048|  %4068 = gepS %4067, i64 %4066                                                                                         ;L419
 34049|     ;; ecache = ptr %4068
 34053|  store i64 %1, ptr %17, , !!47278
 34054|  store i8 0, ptr %16, , !!47278
 34055|     ;; version = ptr %17
 34056|     ;; data = ptr %3
 34057|     ;; champ = ptr %122
 34058|     ;; e = ptr %4018
 34059|     ;; player = ptr %2
 34060|     ;; eplayer = ptr %4055
 34061|     ;; champ_cache = ptr %166
 34062|     ;; e_cache = ptr %4068
 34063|     ;; has_near_enemy = ptr %16
 34064|     ;; seed = ptr %15
 34065|     ;; tick = ptr %14
 34066|     ;; pt = ptr %13
 34067|     ;; pp = ptr %12
 34068|     ;; et = ptr %11
 34069|     ;; ep = ptr %10
 34070|     ;; hn = ptr %9
 34072|  %4069 = load ptr, ptr %118, , !!47278, !!8, !!8                                                                       ;L211<420
 34073|  %4070 = load ptr, ptr %152, , !!47278, !!8, !!8                                                                       ;L211<420
 34074|  %4071 = gep %4070, i64 32                                                                                             ;L211<420
 34075|  %4072 = load ptr, ptr %4071, , !!47278, !!8                                                                           ;L211<420
 34076|  %4073 = invoke i64 %4072(ptr %4069)
 34077|  to label %4074 unwind label %3278                                                                                     ;L211<420
 34078| 
 34079| 4074: ; preds = %4063
 34080|  store i64 %4073, ptr %15, , !!47278                                                                                   ;L211<420
 34082|  %4075 = gep %4070, i64 40                                                                                             ;L212<420
 34083|  %4076 = load ptr, ptr %4075, , !!47278, !!8                                                                           ;L212<420
 34084|  %4077 = invoke i64 %4076(ptr %4069)
 34085|  to label %4078 unwind label %3278                                                                                     ;L212<420
 34086| 
 34087| 4078: ; preds = %4074
 34088|  store i64 %4077, ptr %14, , !!47278                                                                                   ;L212<420
 34089|     ;; self = ptr %2
 34091|  store i64 %112, ptr %13, , !!47278                                                                                    ;L213<420
 34093|  store i64 %117, ptr %12, , !!47278                                                                                    ;L213<420
 34094|  %4079 = load i64, ptr %4058, , !!47301, !!8                                                                           ;L214<420
 34095|     ;; self = ptr %4055
 34096|  %4080 = load i32, ptr %4064, , !!47301, !!8                                                                           ;L581<214<420
 34097|  %4081 = zext nneg i32 %4080 to i64                                                                                    ;L581<214<420
 34099|  store i64 %4079, ptr %11, , !!47278                                                                                   ;L214<420
 34101|  store i64 %4081, ptr %10, , !!47278                                                                                   ;L214<420
 34103|  store i64 0, ptr %9, , !!47278                                                                                        ;L215<420
 34105|  store ptr %15, ptr %8, , !!47278                                                                                      ;L216<420
 34106|  store ptr %14, ptr %204, , !!47278                                                                                    ;L216<420
 34107|  store ptr %13, ptr %205, , !!47278                                                                                    ;L216<420
 34108|  store ptr %12, ptr %206, , !!47278                                                                                    ;L216<420
 34109|  store ptr %11, ptr %207, , !!47278                                                                                    ;L216<420
 34110|  store ptr %10, ptr %208, , !!47278                                                                                    ;L216<420
 34111|  store ptr %9, ptr %209, , !!47278                                                                                     ;L216<420
 34112|  store ptr %17, ptr %210, , !!47278                                                                                    ;L216<420
 34113|  store ptr %122, ptr %211, , !!47278                                                                                   ;L216<420
 34114|  store ptr %4018, ptr %212, , !!47278                                                                                  ;L216<420
 34115|  store ptr %2, ptr %213, , !!47278                                                                                     ;L216<420
 34116|  store ptr %4055, ptr %214, , !!47278                                                                                  ;L216<420
 34117|  store ptr %166, ptr %215, , !!47278                                                                                   ;L216<420
 34118|  store ptr %4068, ptr %216, , !!47278                                                                                  ;L216<420
 34119|  store ptr %16, ptr %217, , !!47278                                                                                    ;L216<420
 34120|  invoke void @core::cell7RefCellNtNtCshdEBA0ozCnw_7game_ai13position_eval8EpcCacheEE4withNCNvB1x_31entity_positioning_cache_cached0NtNtB1z_15score_parameter22EntityPositioningCacheEB1z_(ptr sret([424 x i8]) %94, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.120, ptr %8)
 34121|  to label %4082 unwind label %3278                                                                                     ;L216<420
 34122| 
 34123| 4082: ; preds = %4078
 34134|     ;; value[424..+8] = ptr %4055
 34135|     ;; src[424..+8] = ptr %4055
 34136|     ;; value[432..+8] = ptr %4018
 34137|     ;; src[432..+8] = ptr %4018
 34138|     ;; value[440..+8] = i64 %4051
 34139|     ;; src[440..+8] = i64 %4051
 34140|     ;; self = ptr %95
 34141|     ;; self = ptr %95
 34142|     ;; additional = i64 1
 34143|     ;; needed_extra_cap = i64 1
 34144|     ;; needed_extra_cap = i64 1
 34145|     ;; strategy = i8 1
 34146|  %4083 = load i64, ptr %200, , !!47325, !!8                                                                            ;L1428<421
 34147|     ;; self = ptr %95
 34148|  %4084 = load i64, ptr %199, , !!47325, !!8                                                                            ;L149<1428<421
 34149|  %4085 = icmp eq i64 %4083, %4084                                                                                      ;L1428<421
 34150|  br i1 %4085, label %4086, label %4089                                                                                 ;L1428<421
 34151| 
 34152| 4086: ; preds = %4082
 34153|     ;; self = ptr %95
 34154|     ;; self = ptr %95
 34155|     ;; self = ptr %95
 34156|     ;; used_cap = i64 %4083
 34157|     ;; used_cap = i64 %4083
 34158|  invoke void @ai::score_parameter22EntityPositioningCacheRNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player11PlayerStateRNtNtB2g_6entity6EntityyEE25reserve_internal_or_panicB18_(ptr %95, i64 %4083, i64 1, i1 zeroext true)
 34159|  to label %4087 unwind label %3278                                                                                     ;L619<430<738<1429<421
 34160| 
 34161| 4087: ; preds = %4086
 34162|  %4088 = load i64, ptr %200, , !!47325                                                                                 ;L1432<421
 34163|  br label %4089                                                                                                        ;L1428<421
 34164| 
 34165| 4089: ; preds = %4087, %4082
 34166|  %4090 = phi i64 [ %4083, %4082 ], [ %4088, %4087 ]                                                                    ;L1432<421
 34167|     ;; self = ptr %95
 34168|  %4091 = load ptr, ptr %95, , !!47325, !!8, !!8                                                                        ;L138<1432<421
 34169|     ;; self = ptr %4091
 34170|     ;; count = i64 %4090
 34171|  %4092 = gepS, ptr, i64 }, ptr %4091, i64 %4090                                                                        ;L961<1432<421
 34172|     ;; end = ptr %4092
 34173|     ;; dst = ptr %4092
 34174|  call void @llvm.memcpy.p0.p0.i64(ptr %4092, ptr %94, i64 424, i1 false)                                               ;L1933<1433<421
 34175|  %4093 = gep %4092, i64 424                                                                                            ;L1933<1433<421
 34176|  store ptr %4055, ptr %4093,                                                                                           ;L1933<1433<421
 34177|  %4094 = gep %4092, i64 432                                                                                            ;L1933<1433<421
 34178|  store ptr %4018, ptr %4094,                                                                                           ;L1933<1433<421
 34179|  %4095 = gep %4092, i64 440                                                                                            ;L1933<1433<421
 34180|  store i64 %4051, ptr %4095,                                                                                           ;L1933<1433<421
 34181|  %4096 = load i64, ptr %200, , !!47325, !!8                                                                            ;L1434<421
 34182|  %4097 = add i64 %4096, 1                                                                                              ;L1434<421
 34183|  store i64 %4097, ptr %200, , !!47325                                                                                  ;L1434<421
 34184|  br label %4098                                                                                                        ;L408
 34185| 
 34186| 4098: ; preds = %4089, %4049, %249
 34187|  %4099 = phi i64 [ %223, %249 ], [ %223, %4049 ], [ %4097, %4089 ]
 34188|  br label %222                                                                                                         ;L1916<900<985<408
 34189| 
 34190| 4100: ; preds = %3286
 34191|  cleanupret from %3288 unwind label %125
 34192| 
 34193| 4101: ; preds = %3286
 34194|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %96) #27 [ "funclet"(token %3288) ] ;L1181
 34195|  cleanupret from %3288 unwind label %125                                                                               ;L1181
 34196| 
 34197| 4102: ; preds = %143, %128
 34199|  %4103 = icmp eq i32 %109, -1                                                                                          ;L825<1181
 34200|  br i1 %4103, label %3311, label %4104                                                                                 ;L825<1181
 34201| 
 34202| 4104: ; preds = %4102
 34204|     ;; self = ptr %98
 34205|     ;; order = i8 0
 34206|     ;; order = i8 0
 34207|     ;; val = i64 1
 34208|     ;; order = i8 0
 34209|     ;; val = i64 1
 34210|     ;; order = i8 0
 34211|     ;; self = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 384)
 34212|  %4105 = gep %98, i64 8                                                                                                ;L185<825<825<1181
 34213|  %4106 = call { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %4105)                             ;L185<825<825<1181
 34214|  %4107 = extractvalue { i64, i32 } %4106, 0                                                                            ;L185<825<825<1181
 34215|  %4108 = extractvalue { i64, i32 } %4106, 1                                                                            ;L185<825<825<1181
 34217|  %4109 = mul i64 %4107, 1000000000                                                                                     ;L632<185<825<825<1181
 34218|  %4110 = icmp ult i32 %4108, 1000000000                                                                                ;L49<632<185<825<825<1181
 34219|  call void @llvm.assume(i1 %4110)                                                                                      ;L49<632<185<825<825<1181
 34220|  %4111 = zext nneg i32 %4108 to i64                                                                                    ;L632<185<825<825<1181
 34221|  %4112 = add i64 %4109, %4111                                                                                          ;L632<185<825<825<1181
 34222|     ;; val = i64 %4112
 34223|     ;; val = i64 %4112
 34224|     ;; dst = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 384)
 34225|  %4113 = atomicrmw add ptr getelementptr (i8, ptr @gc::simulation4prof11PHASE_NANOS, i64 384), i64 %4112 monotonic, , !!47363 ;L3937<3162<185<825<825<1181
 34226|     ;; self = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 384)
 34227|     ;; dst = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 384)
 34228|  br label %3308                                                                                                        ;L825<1181
 34229| }
