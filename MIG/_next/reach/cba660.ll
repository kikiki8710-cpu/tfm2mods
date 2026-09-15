 25360| define void @ai::plan_legacy8sub_plan5stealNtB2_12StealSubPlan17action_candidates(ptr sret([32 x i8]) %0, ptr %1, i64 %2, ptr %3, ptr %4, ptr %5, ptr %6) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
 25361|  %8 = alloca [120 x i8],
 25362|  %9 = alloca [184 x i8],
 25363|  %10 = alloca [136 x i8],
 25364|  %11 = alloca [184 x i8],
 25369|  %12 = alloca [184 x i8],
 25370|  %13 = alloca [184 x i8],
 25371|  %14 = alloca [24 x i8],
 25372|  %15 = alloca [184 x i8],
 25373|  %16 = alloca [24 x i8],
 25374|  %17 = alloca [184 x i8],
 25375|  %18 = alloca [24 x i8],
 25376|  %19 = alloca [184 x i8],
 25377|  %20 = alloca [24 x i8],
 25378|  %21 = alloca [184 x i8],
 25379|  %22 = alloca [184 x i8],
 25380|  %23 = alloca [184 x i8],
 25381|  %24 = alloca [136 x i8],
 25382|  %25 = alloca [184 x i8],
 25383|  %26 = alloca [32 x i8],
 25384|     ;; self = ptr %1
 25385|     ;; _version = i64 %2
 25386|     ;; rnd = ptr %3
 25387|     ;; player = ptr %4
 25388|     ;; data = ptr %5
 25389|     ;; _parameter = ptr %6
 25390|     ;; res = ptr %26
 25393|  %27 = gep %5, i64 8                                                                                                   ;L34
 25394|  %28 = load ptr, ptr %27, , !!8, !!8                                                                                   ;L34
 25395|  %29 = load ptr, ptr %28, , !!8, !!8                                                                                   ;L34
 25396|     ;; bump = ptr %29
 25397|  store ptr inttoptr (i64 8 to ptr), ptr %26,                                                                           ;L547<34
 25398|  %30 = gep %26, i64 8                                                                                                  ;L547<34
 25399|  store ptr %29, ptr %30,                                                                                               ;L547<34
 25400|  %31 = gep %26, i64 16                                                                                                 ;L547<34
 25401|  %32 = gep %26, i64 24                                                                                                 ;L547<34
 25402|  %33 = gep %4, i64 2352                                                                                                ;L36
 25403|  call void @llvm.memset.p0.i64(ptr %31, i8 0, i64 16, i1 false)                                                        ;L547<34
 25404|  %34 = load i64, ptr %33, , !!8                                                                                        ;L36
 25405|  %35 = icmp ult i64 %34, 2                                                                                             ;L36
 25406|  br i1 %35, label %40, label %36                                                                                       ;L36
 25407| 
 25408| 36: ; preds = %7
 25409|  invoke void @core::panicking18panic_bounds_check(i64 %34, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.157) #31
 25410|  to label %39 unwind label %37                                                                                         ;L36
 25411| 
 25412| 37: ; preds = %896, %887, %882, %877, %872, %864, %854, %847, %838, %835, %834, %830, %821, %819, %805, %800, %789, %788, %779, %761, %509, %506, %437, %433, %430, %427, %424, %355, %351, %341, %338, %335, %264, %260, %248, %241, %236, %164, %150, %149, %141, %132, %130, %112, %103, %96, %94, %84, %82, %63, %53, %36
 25413|  %38 = cleanuppad within none []
 25414|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %26) #30 [ "funclet"(token %38) ] ;L120
 25415|  cleanupret from %38 unwind to caller                                                                                  ;L33
 25416| 
 25417| 39: ; preds = %835, %149, %36
 25418|  unreachable
 25419| 
 25420| 40: ; preds = %7
 25421|     ;; self = ptr %4
 25422|  %41 = gep %4, i64 2496                                                                                                ;L581<36
 25423|  %42 = load i32, ptr %41, , !!8                                                                                        ;L581<36
 25424|  %43 = zext nneg i32 %42 to i64                                                                                        ;L581<36
 25425|  %44 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L36
 25426|     ;; self = ptr %44
 25427|  %45 = gep %44, i64 480                                                                                                ;L36
 25428|  %46 = getelementptr [5 x ptr], ptr %45, i64 %34                                                                       ;L36
 25429|  %47 = getelementptr ptr, ptr %46, i64 %43                                                                             ;L36
 25430|     ;; self = ptr %47
 25431|     ;; self = ptr %47
 25432|  %48 = load ptr, ptr %47, , !!8                                                                                        ;L633<682<36
 25433|  %49 = icmp eq ptr %48, null                                                                                           ;L633<682<36
 25434|  br i1 %49, label %96, label %50                                                                                       ;L36
 25435| 
 25436| 50: ; preds = %40
 25437|  %51 = gep %1, i64 8                                                                                                   ;L41
 25438|  %52 = load i8, ptr %51, , !!8                                                                                         ;L41
 25441|     ;; index = i64 0
 25442|     ;; self = i64 0
 25443|     ;; self = i8 %52
 25444|  switch i8 %52, label %53 [
 25445|  i8 2, label %872
 25446|  i8 0, label %63
 25447|  ]                                                                                                                     ;L2775<26<41
 25448| 
 25449| 53: ; preds = %50
 25450|  %54 = load ptr, ptr %44, , !!8, !!8                                                                                   ;L28<41
 25451|  %55 = gep %44, i64 8                                                                                                  ;L28<41
 25452|  %56 = load ptr, ptr %55, , !!8, !!8                                                                                   ;L28<41
 25453|  %57 = gep %56, i64 64                                                                                                 ;L28<41
 25454|  %58 = load ptr, ptr %57, , !!8                                                                                        ;L28<41
 25455|  %59 = invoke { i64, ptr } %58(ptr %54)
 25456|  to label %60 unwind label %37                                                                                         ;L28<41
 25457| 
 25458| 60: ; preds = %53
 25459|  %61 = extractvalue { i64, ptr } %59, 0                                                                                ;L28<41
 25460|     ;; self[0..+8] = i64 %61
 25462|  %62 = icmp eq i64 %61, 0                                                                                              ;L231<28<41
 25463|  br i1 %62, label %73, label %94                                                                                       ;L231<28<41
 25464| 
 25465| 63: ; preds = %50
 25466|  %64 = load ptr, ptr %44, , !!8, !!8                                                                                   ;L27<41
 25467|  %65 = gep %44, i64 8                                                                                                  ;L27<41
 25468|  %66 = load ptr, ptr %65, , !!8, !!8                                                                                   ;L27<41
 25469|  %67 = gep %66, i64 64                                                                                                 ;L27<41
 25470|  %68 = load ptr, ptr %67, , !!8                                                                                        ;L27<41
 25471|  %69 = invoke { i64, ptr } %68(ptr %64)
 25472|  to label %70 unwind label %37                                                                                         ;L27<41
 25473| 
 25474| 70: ; preds = %63
 25475|  %71 = extractvalue { i64, ptr } %69, 0                                                                                ;L27<41
 25476|     ;; self[0..+8] = i64 %71
 25478|  %72 = icmp eq i64 %71, 0                                                                                              ;L231<27<41
 25479|  br i1 %72, label %73, label %82                                                                                       ;L231<27<41
 25480| 
 25481| 73: ; preds = %70, %60
 25482|  %74 = phi { i64, ptr } [ %59, %60 ], [ %69, %70 ]
 25483|  %75 = phi i64 [ 456, %60 ], [ 408, %70 ]
 25484|  %76 = extractvalue { i64, ptr } %74, 1                                                                                ;L0<41
 25485|  %77 = icmp ne ptr %76, null
 25486|  tail call void @llvm.assume(i1 %77)
 25487|  %78 = gep %76, i64 %75                                                                                                ;L0<41
 25488|     ;; self = ptr %78
 25489|     ;; self = ptr %78
 25490|     ;; self = ptr %78
 25491|     ;; live_list = ptr %78
 25492|  %79 = gep %78, i64 16                                                                                                 ;L1864<3787<30<41
 25493|  %80 = load i64, ptr %79, , !!8                                                                                        ;L1864<3787<30<41
 25496|     ;; self[8..+8] = i64 %80
 25497|     ;; slice[8..+8] = i64 %80
 25498|  %81 = icmp eq i64 %80, 0                                                                                              ;L219<576<30<41
 25499|  br i1 %81, label %99, label %84                                                                                       ;L219<576<30<41
 25500| 
 25501| 82: ; preds = %70
 25502|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.155) #31
 25503|  to label %83 unwind label %37                                                                                         ;L1013<27<41
 25504| 
 25505| 83: ; preds = %82
 25506|  unreachable                                                                                                           ;L1013<27<41
 25507| 
 25508| 84: ; preds = %73
 25509|  %85 = gep %78, i64 8                                                                                                  ;L614<609<296<1968<1864<3787<30<41
 25510|  %86 = load ptr, ptr %85, , !!8, !!8                                                                                   ;L614<609<296<1968<1864<3787<30<41
 25511|     ;; self[0..+8] = ptr %86
 25512|     ;; slice[0..+8] = ptr %86
 25513|     ;; self = ptr %86
 25514|  %87 = load ptr, ptr %44, , !!8, !!8                                                                                   ;L30<41
 25515|  %88 = gep %44, i64 8                                                                                                  ;L30<41
 25516|  %89 = load ptr, ptr %88, , !!8, !!8                                                                                   ;L30<41
 25517|     ;; f[0..+8] = ptr %87
 25518|     ;; f[8..+8] = ptr %89
 25519|     ;; x = ptr %86
 25520|     ;; id = ptr %86
 25521|  %90 = load i64, ptr %86, , !!8                                                                                        ;L30<1543<30<41
 25522|  %91 = gep %89, i64 496                                                                                                ;L30<1543<30<41
 25523|  %92 = load ptr, ptr %91, , !!8                                                                                        ;L30<1543<30<41
 25524|  %93 = invoke ptr %92(ptr %87, i64 %90)
 25525|  to label %97 unwind label %37                                                                                         ;L30<1543<30<41
 25526| 
 25527| 94: ; preds = %60
 25528|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.156) #31
 25529|  to label %95 unwind label %37                                                                                         ;L1013<28<41
 25530| 
 25531| 95: ; preds = %94
 25532|  unreachable                                                                                                           ;L1013<28<41
 25533| 
 25534| 96: ; preds = %40
 25537|  invoke void @ai::small_action12move_actionsNtB5_17SmallActionRecall3new(ptr sret([136 x i8]) %24, ptr %5, ptr %4, i64 5)
 25538|  to label %894 unwind label %37                                                                                        ;L37
 25539| 
 25540| 97: ; preds = %84
 25543|  %98 = icmp eq ptr %93, null                                                                                           ;L633<682<41
 25544|  br i1 %98, label %99, label %101                                                                                      ;L41
 25545| 
 25546| 99: ; preds = %97, %73
 25547|  %100 = icmp eq i8 %52, 1                                                                                              ;L43
 25548|  br i1 %100, label %877, label %872                                                                                    ;L43
 25549| 
 25550| 101: ; preds = %97
 25551|     ;; self = ptr %4
 25552|  %102 = load ptr, ptr %47, , !!8, !!8                                                                                  ;L50
 25553|     ;; self = ptr %102
 25554|     ;; champ = ptr %102
 25555|     ;; caster = ptr %102
 25556|     ;; self = ptr %102
 25557|     ;; other = ptr %102
 25558|     ;; caster = ptr %102
 25559|     ;; self = ptr %102
 25560|     ;; other = ptr %102
 25561|     ;; self = ptr %102
 25562|     ;; caster = ptr %102
 25563|     ;; self = ptr %102
 25564|     ;; other = ptr %102
 25565|     ;; self = ptr %102
 25566|     ;; caster = ptr %102
 25567|     ;; self = ptr %102
 25568|     ;; other = ptr %102
 25571|     ;; index = i64 0
 25572|     ;; self = i64 0
 25573|     ;; self = i8 %52
 25574|  switch i8 %52, label %103 [
 25575|  i8 2, label %149
 25576|  i8 0, label %112
 25577|  ]                                                                                                                     ;L2775<26<51
 25578| 
 25579| 103: ; preds = %101
 25580|  %104 = load ptr, ptr %44, , !!8, !!8                                                                                  ;L28<51
 25581|  %105 = load ptr, ptr %88, , !!8, !!8                                                                                  ;L28<51
 25582|  %106 = gep %105, i64 64                                                                                               ;L28<51
 25583|  %107 = load ptr, ptr %106, , !!8                                                                                      ;L28<51
 25584|  %108 = invoke { i64, ptr } %107(ptr %104)
 25585|  to label %109 unwind label %37                                                                                        ;L28<51
 25586| 
 25587| 109: ; preds = %103
 25588|  %110 = extractvalue { i64, ptr } %108, 0                                                                              ;L28<51
 25589|     ;; self[0..+8] = i64 %110
 25591|  %111 = icmp eq i64 %110, 0                                                                                            ;L231<28<51
 25592|  br i1 %111, label %121, label %141                                                                                    ;L231<28<51
 25593| 
 25594| 112: ; preds = %101
 25595|  %113 = load ptr, ptr %44, , !!8, !!8                                                                                  ;L27<51
 25596|  %114 = load ptr, ptr %88, , !!8, !!8                                                                                  ;L27<51
 25597|  %115 = gep %114, i64 64                                                                                               ;L27<51
 25598|  %116 = load ptr, ptr %115, , !!8                                                                                      ;L27<51
 25599|  %117 = invoke { i64, ptr } %116(ptr %113)
 25600|  to label %118 unwind label %37                                                                                        ;L27<51
 25601| 
 25602| 118: ; preds = %112
 25603|  %119 = extractvalue { i64, ptr } %117, 0                                                                              ;L27<51
 25604|     ;; self[0..+8] = i64 %119
 25606|  %120 = icmp eq i64 %119, 0                                                                                            ;L231<27<51
 25607|  br i1 %120, label %121, label %130                                                                                    ;L231<27<51
 25608| 
 25609| 121: ; preds = %118, %109
 25610|  %122 = phi { i64, ptr } [ %108, %109 ], [ %117, %118 ]
 25611|  %123 = phi i64 [ 456, %109 ], [ 408, %118 ]
 25612|  %124 = extractvalue { i64, ptr } %122, 1                                                                              ;L0<51
 25613|  %125 = icmp ne ptr %124, null
 25614|  tail call void @llvm.assume(i1 %125)
 25615|  %126 = gep %124, i64 %123                                                                                             ;L0<51
 25616|     ;; self = ptr %126
 25617|     ;; self = ptr %126
 25618|     ;; self = ptr %126
 25619|     ;; live_list = ptr %126
 25620|  %127 = gep %126, i64 16                                                                                               ;L1864<3787<30<51
 25621|  %128 = load i64, ptr %127, , !!8                                                                                      ;L1864<3787<30<51
 25624|     ;; self[8..+8] = i64 %128
 25625|     ;; slice[8..+8] = i64 %128
 25626|  %129 = icmp eq i64 %128, 0                                                                                            ;L219<576<30<51
 25627|  br i1 %129, label %149, label %132                                                                                    ;L219<576<30<51
 25628| 
 25629| 130: ; preds = %118
 25630|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.155) #31
 25631|  to label %131 unwind label %37                                                                                        ;L1013<27<51
 25632| 
 25633| 131: ; preds = %130
 25634|  unreachable                                                                                                           ;L1013<27<51
 25635| 
 25636| 132: ; preds = %121
 25637|  %133 = gep %126, i64 8                                                                                                ;L614<609<296<1968<1864<3787<30<51
 25638|  %134 = load ptr, ptr %133, , !!8, !!8                                                                                 ;L614<609<296<1968<1864<3787<30<51
 25639|     ;; self[0..+8] = ptr %134
 25640|     ;; slice[0..+8] = ptr %134
 25641|     ;; self = ptr %134
 25642|  %135 = load ptr, ptr %44, , !!8, !!8                                                                                  ;L30<51
 25643|  %136 = load ptr, ptr %88, , !!8, !!8                                                                                  ;L30<51
 25644|     ;; f[0..+8] = ptr %135
 25645|     ;; f[8..+8] = ptr %136
 25646|     ;; x = ptr %134
 25647|     ;; id = ptr %134
 25648|  %137 = load i64, ptr %134, , !!8                                                                                      ;L30<1543<30<51
 25649|  %138 = gep %136, i64 496                                                                                              ;L30<1543<30<51
 25650|  %139 = load ptr, ptr %138, , !!8                                                                                      ;L30<1543<30<51
 25651|  %140 = invoke ptr %139(ptr %135, i64 %137)
 25652|  to label %143 unwind label %37                                                                                        ;L30<1543<30<51
 25653| 
 25654| 141: ; preds = %109
 25655|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.156) #31
 25656|  to label %142 unwind label %37                                                                                        ;L1013<28<51
 25657| 
 25658| 142: ; preds = %141
 25659|  unreachable                                                                                                           ;L1013<28<51
 25660| 
 25661| 143: ; preds = %132
 25662|     ;; self = ptr %140
 25663|  %144 = icmp eq ptr %140, null                                                                                         ;L1011<51
 25664|  br i1 %144, label %149, label %145                                                                                    ;L1011<51
 25665| 
 25666| 145: ; preds = %143
 25667|     ;; target = ptr %140
 25668|     ;; self = ptr %140
 25669|     ;; self = ptr %140
 25670|     ;; self = ptr %140
 25671|     ;; self = ptr %140
 25672|     ;; self = ptr %140
 25673|     ;; self = ptr %140
 25674|     ;; self = ptr %140
 25675|     ;; self = ptr %140
 25676|  %146 = gep %1, i64 9                                                                                                  ;L53
 25677|  %147 = load i8, ptr %146, , !!8                                                                                       ;L53
 25678|  %148 = trunc nuw i8 %147 to i1                                                                                        ;L53
 25679|  br i1 %148, label %150, label %512                                                                                    ;L53
 25680| 
 25681| 149: ; preds = %143, %121, %101
 25682|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.158) #31
 25683|  to label %39 unwind label %37                                                                                         ;L1013<51
 25684| 
 25685| 150: ; preds = %145
 25686|  %151 = gep %102, i64 1600                                                                                             ;L54
 25687|  %152 = load i64, ptr %151, , !!8                                                                                      ;L54
 25688|     ;; move_speed = i64 %152
 25689|  %153 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_attack(ptr %102)
 25690|  to label %154 unwind label %37                                                                                        ;L56
 25691| 
 25692| 154: ; preds = %150
 25693|  br i1 %153, label %160, label %155                                                                                    ;L56
 25694| 
 25695| 155: ; preds = %243, %206, %160, %154
 25696|     ;; self = ptr %102
 25697|  %156 = gep %102, i64 1224                                                                                             ;L742<67
 25698|  %157 = gep %102, i64 1272                                                                                             ;L742<67
 25699|  %158 = load i32, ptr %157, , !!8                                                                                      ;L742<67
 25700|  %159 = icmp eq i32 %158, -1                                                                                           ;L742<67
 25701|  br i1 %159, label %250, label %248                                                                                    ;L742<67
 25702| 
 25703| 160: ; preds = %154
 25704|     ;; self = ptr %102
 25705|  %161 = gep %102, i64 1216                                                                                             ;L742<57
 25706|  %162 = load i32, ptr %161, , !!8                                                                                      ;L742<57
 25707|  %163 = icmp eq i32 %162, -1                                                                                           ;L742<57
 25708|  br i1 %163, label %155, label %164                                                                                    ;L742<57
 25709| 
 25710| 164: ; preds = %160
 25711|  %165 = gep %102, i64 1168                                                                                             ;L742<57
 25712|     ;; atk = ptr %165
 25713|     ;; self = ptr %165
 25714|  %166 = gep %102, i64 1184                                                                                             ;L26<58
 25715|  %167 = load i64, ptr %166, , !!8                                                                                      ;L26<58
 25716|  %168 = gep %102, i64 1192                                                                                             ;L26<58
 25717|  %169 = load i64, ptr %168, , !!8                                                                                      ;L26<58
 25718|  %170 = gep %102, i64 1480                                                                                             ;L26<58
 25719|  %171 = load i64, ptr %170, , !!8                                                                                      ;L26<58
 25720|  %172 = gep %102, i64 1080                                                                                             ;L26<58
 25721|  %173 = load i64, ptr %172, , !!8                                                                                      ;L26<58
 25722|  %174 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %165, ptr %102, ptr %140)
 25723|  to label %175 unwind label %37                                                                                        ;L58
 25724| 
 25725| 175: ; preds = %164
 25726|  %176 = add i64 %171, -1                                                                                               ;L26<58
 25727|  %177 = mul i64 %176, %169                                                                                             ;L26<58
 25728|  %178 = gep %102, i64 1136                                                                                             ;L1511<58
 25729|  %179 = load i32, ptr %178, , !!8                                                                                      ;L1511<58
 25730|     ;; mult = i32 %179
 25731|  %180 = icmp eq i32 %179, 0                                                                                            ;L1512<58
 25732|  br i1 %180, label %181, label %184                                                                                    ;L1512<58
 25733| 
 25734| 181: ; preds = %175
 25735|  %182 = gep %102, i64 1664                                                                                             ;L1513<58
 25736|  %183 = load i64, ptr %182, , !!8                                                                                      ;L1513<58
 25737|  br label %191                                                                                                         ;L1512<58
 25738| 
 25739| 184: ; preds = %175
 25740|  %185 = sext i32 %179 to i64                                                                                           ;L1511<58
 25741|     ;; mult = i64 %185
 25742|  %186 = gep %102, i64 1664                                                                                             ;L1515<58
 25743|  %187 = load i64, ptr %186, , !!8                                                                                      ;L1515<58
 25744|  %188 = add nsw i64 %185, 100                                                                                          ;L1515<58
 25745|  %189 = mul i64 %187, %188                                                                                             ;L1515<58
 25746|  %190 = udiv i64 %189, 100                                                                                             ;L1515<58
 25747|  br label %191                                                                                                         ;L1512<58
 25748| 
 25749| 191: ; preds = %184, %181
 25750|  %192 = phi i64 [ %183, %181 ], [ %190, %184 ]                                                                         ;L0<58
 25751|  %193 = gep %140, i64 1136                                                                                             ;L1511<58
 25752|  %194 = load i32, ptr %193, , !!8                                                                                      ;L1511<58
 25753|     ;; mult = i32 %194
 25754|  %195 = icmp eq i32 %194, 0                                                                                            ;L1512<58
 25755|  br i1 %195, label %196, label %199                                                                                    ;L1512<58
 25756| 
 25757| 196: ; preds = %191
 25758|  %197 = gep %140, i64 1664                                                                                             ;L1513<58
 25759|  %198 = load i64, ptr %197, , !!8                                                                                      ;L1513<58
 25760|  br label %206                                                                                                         ;L1512<58
 25761| 
 25762| 199: ; preds = %191
 25763|  %200 = sext i32 %194 to i64                                                                                           ;L1511<58
 25764|     ;; mult = i64 %200
 25765|  %201 = gep %140, i64 1664                                                                                             ;L1515<58
 25766|  %202 = load i64, ptr %201, , !!8                                                                                      ;L1515<58
 25767|  %203 = add nsw i64 %200, 100                                                                                          ;L1515<58
 25768|  %204 = mul i64 %202, %203                                                                                             ;L1515<58
 25769|  %205 = udiv i64 %204, 100                                                                                             ;L1515<58
 25770|  br label %206                                                                                                         ;L1512<58
 25771| 
 25772| 206: ; preds = %199, %196
 25773|  %207 = phi i64 [ %198, %196 ], [ %205, %199 ]                                                                         ;L0<58
 25775|  %208 = gep %140, i64 1632                                                                                             ;L2158<59
 25776|  %209 = load i64, ptr %208, , !!8                                                                                      ;L2158<59
 25777|     ;; x1 = i64 %209
 25778|     ;; self = i64 %209
 25779|  %210 = gep %140, i64 1640                                                                                             ;L2158<59
 25780|  %211 = load i64, ptr %210, , !!8                                                                                      ;L2158<59
 25781|     ;; y1 = i64 %211
 25782|     ;; self = i64 %211
 25783|  %212 = gep %102, i64 1632                                                                                             ;L2158<59
 25784|  %213 = load i64, ptr %212, , !!8                                                                                      ;L2158<59
 25785|     ;; x2 = i64 %213
 25786|     ;; other = i64 %213
 25787|  %214 = gep %102, i64 1640                                                                                             ;L2158<59
 25788|  %215 = load i64, ptr %214, , !!8                                                                                      ;L2158<59
 25789|     ;; y2 = i64 %215
 25790|     ;; other = i64 %215
 25791|  %216 = icmp ult i64 %209, %213                                                                                        ;L3147<7<2158<59
 25792|  %217 = sub nuw i64 %213, %209                                                                                         ;L3147<7<2158<59
 25793|  %218 = sub nuw i64 %209, %213                                                                                         ;L3147<7<2158<59
 25794|  %219 = select i1 %216, i64 %217, i64 %218                                                                             ;L3147<7<2158<59
 25795|     ;; dx = i64 %219
 25796|  %220 = icmp ult i64 %211, %215                                                                                        ;L3147<8<2158<59
 25797|  %221 = sub nuw i64 %215, %211                                                                                         ;L3147<8<2158<59
 25798|  %222 = sub nuw i64 %211, %215                                                                                         ;L3147<8<2158<59
 25799|  %223 = select i1 %220, i64 %221, i64 %222                                                                             ;L3147<8<2158<59
 25800|     ;; dy = i64 %223
 25801|  %224 = mul i64 %219, %219                                                                                             ;L9<2158<59
 25802|  %225 = mul i64 %223, %223                                                                                             ;L9<2158<59
 25803|  %226 = add i64 %225, %224                                                                                             ;L9<2158<59
 25804|     ;; dist_sq = i64 %226
 25805|  %227 = mul i64 %152, 30                                                                                               ;L60
 25806|  %228 = add i64 %167, %227                                                                                             ;L26<58
 25807|  %229 = add i64 %228, %173                                                                                             ;L26<58
 25808|  %230 = add i64 %229, %177                                                                                             ;L58
 25809|  %231 = add i64 %230, %174                                                                                             ;L58
 25810|  %232 = add i64 %231, %192                                                                                             ;L58
 25811|  %233 = add i64 %232, %207                                                                                             ;L60
 25812|     ;; max_dist = i64 %233
 25813|  %234 = mul i64 %233, %233                                                                                             ;L61
 25814|  %235 = icmp ugt i64 %226, %234                                                                                        ;L61
 25815|  br i1 %235, label %155, label %236                                                                                    ;L61
 25816| 
 25817| 236: ; preds = %206
 25820|  %237 = gep %140, i64 1472                                                                                             ;L62
 25821|  %238 = load i64, ptr %237, , !!8                                                                                      ;L62
 25822|  invoke void @ai::small_action4castNtB2_17SmallActionAttack3new(ptr sret([24 x i8]) %20, ptr %5, i64 %238)
 25823|  to label %239 unwind label %37                                                                                        ;L62
 25824| 
 25825| 239: ; preds = %236
 25826|  call void @llvm.memcpy.p0.p0.i64(ptr %21, ptr %20, i64 24, i1 false)                                                  ;L62
 25827|  %240 = gep %21, i64 177                                                                                               ;L62
 25828|  store i8 15, ptr %240,                                                                                                ;L62
 25831|     ;; self = ptr %26
 25832|     ;; self = ptr %26
 25833|     ;; value = ptr %21
 25834|     ;; src = ptr %21
 25835|     ;; additional = i64 1
 25836|     ;; needed_extra_cap = i64 1
 25837|     ;; needed_extra_cap = i64 1
 25838|     ;; strategy = i8 1
 25839|     ;; self = ptr %26
 25840|     ;; self = ptr %26
 25841|     ;; self = ptr %26
 25842|     ;; self = ptr %26
 25843|     ;; used_cap = i64 0
 25844|     ;; used_cap = i64 0
 25845|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %26, i64 0, i64 1, i1 zeroext true)
 25846|  to label %243 unwind label %241, !!33655                                                                              ;L619<430<738<1429<62
 25847| 
 25848| 241: ; preds = %239
 25849|  %242 = cleanuppad within none []
 25850|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %21) #30 [ "funclet"(token %242) ], !!33636 ;L1436<62
 25851|  cleanupret from %242 unwind label %37
 25852| 
 25853| 243: ; preds = %239
 25854|  %244 = load ptr, ptr %26, , !!33655                                                                                   ;L138<1432<62
 25855|  %245 = load i64, ptr %32, , !!33655                                                                                   ;L1432<62
 25856|     ;; self = ptr %26
 25857|     ;; self = ptr %244
 25858|     ;; count = i64 %245
 25859|  %246 = gepS %244, i64 %245                                                                                            ;L961<1432<62
 25860|     ;; end = ptr %246
 25861|     ;; dst = ptr %246
 25862|  call void @llvm.memcpy.p0.p0.i64(ptr %246, ptr %21, i64 184, i1 false), !!33636                                       ;L1933<1433<62
 25863|  %247 = add i64 %245, 1                                                                                                ;L1434<62
 25864|  store i64 %247, ptr %32, , !!33655                                                                                    ;L1434<62
 25866|  br label %155                                                                                                         ;L61
 25867| 
 25868| 248: ; preds = %155
 25869|     ;; skill = ptr %156
 25870|     ;; self = ptr %156
 25871|  %249 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity9can_skill(ptr %102)
 25872|  to label %259 unwind label %37                                                                                        ;L68
 25873| 
 25874| 250: ; preds = %340, %305, %263, %259, %155
 25875|  %251 = gep %102, i64 1480                                                                                             ;L1693<78
 25876|  %252 = load i64, ptr %251, , !!8                                                                                      ;L1693<78
 25877|  %253 = icmp ugt i64 %252, 2                                                                                           ;L1693<78
 25878|  %254 = gep %102, i64 1280                                                                                             ;L1693<78
 25879|  %255 = select i1 %253, ptr %254, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.19                                        ;L1693<78
 25880|     ;; self = ptr %255
 25881|  %256 = gep %255, i64 48                                                                                               ;L742<78
 25882|  %257 = load i32, ptr %256, , !!8                                                                                      ;L742<78
 25883|  %258 = icmp eq i32 %257, -1                                                                                           ;L742<78
 25884|  br i1 %258, label %343, label %341                                                                                    ;L742<78
 25885| 
 25886| 259: ; preds = %248
 25887|  br i1 %249, label %260, label %250                                                                                    ;L68
 25888| 
 25889| 260: ; preds = %259
 25890|  %261 = gep %102, i64 1264                                                                                             ;L68
 25891|  %262 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %261, ptr %102, ptr %140)
 25892|  to label %263 unwind label %37                                                                                        ;L68
 25893| 
 25894| 263: ; preds = %260
 25895|  br i1 %262, label %264, label %250                                                                                    ;L68
 25896| 
 25897| 264: ; preds = %263
 25898|  %265 = gep %102, i64 1240                                                                                             ;L26<69
 25899|  %266 = load i64, ptr %265, , !!8                                                                                      ;L26<69
 25900|  %267 = gep %102, i64 1248                                                                                             ;L26<69
 25901|  %268 = load i64, ptr %267, , !!8                                                                                      ;L26<69
 25902|  %269 = gep %102, i64 1480                                                                                             ;L26<69
 25903|  %270 = load i64, ptr %269, , !!8                                                                                      ;L26<69
 25904|  %271 = gep %102, i64 1080                                                                                             ;L26<69
 25905|  %272 = load i64, ptr %271, , !!8                                                                                      ;L26<69
 25906|  %273 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %156, ptr %102, ptr %140)
 25907|  to label %274 unwind label %37                                                                                        ;L69
 25908| 
 25909| 274: ; preds = %264
 25910|  %275 = add i64 %270, -1                                                                                               ;L26<69
 25911|  %276 = mul i64 %275, %268                                                                                             ;L26<69
 25912|  %277 = gep %102, i64 1136                                                                                             ;L1511<69
 25913|  %278 = load i32, ptr %277, , !!8                                                                                      ;L1511<69
 25914|     ;; mult = i32 %278
 25915|  %279 = icmp eq i32 %278, 0                                                                                            ;L1512<69
 25916|  br i1 %279, label %280, label %283                                                                                    ;L1512<69
 25917| 
 25918| 280: ; preds = %274
 25919|  %281 = gep %102, i64 1664                                                                                             ;L1513<69
 25920|  %282 = load i64, ptr %281, , !!8                                                                                      ;L1513<69
 25921|  br label %290                                                                                                         ;L1512<69
 25922| 
 25923| 283: ; preds = %274
 25924|  %284 = sext i32 %278 to i64                                                                                           ;L1511<69
 25925|     ;; mult = i64 %284
 25926|  %285 = gep %102, i64 1664                                                                                             ;L1515<69
 25927|  %286 = load i64, ptr %285, , !!8                                                                                      ;L1515<69
 25928|  %287 = add nsw i64 %284, 100                                                                                          ;L1515<69
 25929|  %288 = mul i64 %286, %287                                                                                             ;L1515<69
 25930|  %289 = udiv i64 %288, 100                                                                                             ;L1515<69
 25931|  br label %290                                                                                                         ;L1512<69
 25932| 
 25933| 290: ; preds = %283, %280
 25934|  %291 = phi i64 [ %282, %280 ], [ %289, %283 ]                                                                         ;L0<69
 25935|  %292 = gep %140, i64 1136                                                                                             ;L1511<69
 25936|  %293 = load i32, ptr %292, , !!8                                                                                      ;L1511<69
 25937|     ;; mult = i32 %293
 25938|  %294 = icmp eq i32 %293, 0                                                                                            ;L1512<69
 25939|  br i1 %294, label %295, label %298                                                                                    ;L1512<69
 25940| 
 25941| 295: ; preds = %290
 25942|  %296 = gep %140, i64 1664                                                                                             ;L1513<69
 25943|  %297 = load i64, ptr %296, , !!8                                                                                      ;L1513<69
 25944|  br label %305                                                                                                         ;L1512<69
 25945| 
 25946| 298: ; preds = %290
 25947|  %299 = sext i32 %293 to i64                                                                                           ;L1511<69
 25948|     ;; mult = i64 %299
 25949|  %300 = gep %140, i64 1664                                                                                             ;L1515<69
 25950|  %301 = load i64, ptr %300, , !!8                                                                                      ;L1515<69
 25951|  %302 = add nsw i64 %299, 100                                                                                          ;L1515<69
 25952|  %303 = mul i64 %301, %302                                                                                             ;L1515<69
 25953|  %304 = udiv i64 %303, 100                                                                                             ;L1515<69
 25954|  br label %305                                                                                                         ;L1512<69
 25955| 
 25956| 305: ; preds = %298, %295
 25957|  %306 = phi i64 [ %297, %295 ], [ %304, %298 ]                                                                         ;L0<69
 25959|  %307 = gep %140, i64 1632                                                                                             ;L2158<70
 25960|  %308 = load i64, ptr %307, , !!8                                                                                      ;L2158<70
 25961|     ;; x1 = i64 %308
 25962|     ;; self = i64 %308
 25963|  %309 = gep %140, i64 1640                                                                                             ;L2158<70
 25964|  %310 = load i64, ptr %309, , !!8                                                                                      ;L2158<70
 25965|     ;; y1 = i64 %310
 25966|     ;; self = i64 %310
 25967|  %311 = gep %102, i64 1632                                                                                             ;L2158<70
 25968|  %312 = load i64, ptr %311, , !!8                                                                                      ;L2158<70
 25969|     ;; x2 = i64 %312
 25970|     ;; other = i64 %312
 25971|  %313 = gep %102, i64 1640                                                                                             ;L2158<70
 25972|  %314 = load i64, ptr %313, , !!8                                                                                      ;L2158<70
 25973|     ;; y2 = i64 %314
 25974|     ;; other = i64 %314
 25975|  %315 = icmp ult i64 %308, %312                                                                                        ;L3147<7<2158<70
 25976|  %316 = sub nuw i64 %312, %308                                                                                         ;L3147<7<2158<70
 25977|  %317 = sub nuw i64 %308, %312                                                                                         ;L3147<7<2158<70
 25978|  %318 = select i1 %315, i64 %316, i64 %317                                                                             ;L3147<7<2158<70
 25979|     ;; dx = i64 %318
 25980|  %319 = icmp ult i64 %310, %314                                                                                        ;L3147<8<2158<70
 25981|  %320 = sub nuw i64 %314, %310                                                                                         ;L3147<8<2158<70
 25982|  %321 = sub nuw i64 %310, %314                                                                                         ;L3147<8<2158<70
 25983|  %322 = select i1 %319, i64 %320, i64 %321                                                                             ;L3147<8<2158<70
 25984|     ;; dy = i64 %322
 25985|  %323 = mul i64 %318, %318                                                                                             ;L9<2158<70
 25986|  %324 = mul i64 %322, %322                                                                                             ;L9<2158<70
 25987|  %325 = add i64 %324, %323                                                                                             ;L9<2158<70
 25988|     ;; dist_sq = i64 %325
 25989|  %326 = mul i64 %152, 30                                                                                               ;L71
 25990|  %327 = add i64 %266, %326                                                                                             ;L26<69
 25991|  %328 = add i64 %327, %272                                                                                             ;L26<69
 25992|  %329 = add i64 %328, %276                                                                                             ;L69
 25993|  %330 = add i64 %329, %273                                                                                             ;L69
 25994|  %331 = add i64 %330, %291                                                                                             ;L69
 25995|  %332 = add i64 %331, %306                                                                                             ;L71
 25996|     ;; max_dist = i64 %332
 25997|  %333 = mul i64 %332, %332                                                                                             ;L72
 25998|  %334 = icmp ugt i64 %325, %333                                                                                        ;L72
 25999|  br i1 %334, label %250, label %335                                                                                    ;L72
 26000| 
 26001| 335: ; preds = %305
 26004|  %336 = gep %140, i64 1472                                                                                             ;L73
 26005|  %337 = load i64, ptr %336, , !!8                                                                                      ;L73
 26006|  invoke void @ai::small_action4castNtB5_16SmallActionSkill3new(ptr sret([24 x i8]) %18, ptr %5, i64 %337)
 26007|  to label %338 unwind label %37                                                                                        ;L73
 26008| 
 26009| 338: ; preds = %335
 26010|  call void @llvm.memcpy.p0.p0.i64(ptr %19, ptr %18, i64 24, i1 false)                                                  ;L73
 26011|  %339 = gep %19, i64 177                                                                                               ;L73
 26012|  store i8 16, ptr %339,                                                                                                ;L73
 26014|  invoke fastcc void @ai::small_action15SmallActionPlayE4pushBX_(ptr %26, ptr %19)
 26015|  to label %340 unwind label %37                                                                                        ;L73
 26016| 
 26017| 340: ; preds = %338
 26019|  br label %250                                                                                                         ;L72
 26020| 
 26021| 341: ; preds = %250
 26022|     ;; skill2 = ptr %255
 26023|     ;; self = ptr %255
 26024|  %342 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_skill2(ptr %102)
 26025|  to label %350 unwind label %37                                                                                        ;L79
 26026| 
 26027| 343: ; preds = %429, %394, %354, %350, %250
 26028|  %344 = icmp ugt i64 %252, 4                                                                                           ;L1701<89
 26029|  %345 = gep %102, i64 1336                                                                                             ;L1701<89
 26030|  %346 = select i1 %344, ptr %345, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.19                                        ;L1701<89
 26031|     ;; self = ptr %346
 26032|  %347 = gep %346, i64 48                                                                                               ;L742<89
 26033|  %348 = load i32, ptr %347, , !!8                                                                                      ;L742<89
 26034|  %349 = icmp eq i32 %348, -1                                                                                           ;L742<89
 26035|  br i1 %349, label %779, label %430                                                                                    ;L742<89
 26036| 
 26037| 350: ; preds = %341
 26038|  br i1 %342, label %351, label %343                                                                                    ;L79
 26039| 
 26040| 351: ; preds = %350
 26041|  %352 = gep %255, i64 40                                                                                               ;L79
 26042|  %353 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %352, ptr %102, ptr %140)
 26043|  to label %354 unwind label %37                                                                                        ;L79
 26044| 
 26045| 354: ; preds = %351
 26046|  br i1 %353, label %355, label %343                                                                                    ;L79
 26047| 
 26048| 355: ; preds = %354
 26049|  %356 = gep %255, i64 16                                                                                               ;L26<80
 26050|  %357 = load i64, ptr %356, , !!8                                                                                      ;L26<80
 26051|  %358 = gep %255, i64 24                                                                                               ;L26<80
 26052|  %359 = load i64, ptr %358, , !!8                                                                                      ;L26<80
 26053|  %360 = gep %102, i64 1080                                                                                             ;L26<80
 26054|  %361 = load i64, ptr %360, , !!8                                                                                      ;L26<80
 26055|  %362 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %255, ptr %102, ptr %140)
 26056|  to label %363 unwind label %37                                                                                        ;L80
 26057| 
 26058| 363: ; preds = %355
 26059|  %364 = add i64 %252, -1                                                                                               ;L26<80
 26060|  %365 = mul i64 %359, %364                                                                                             ;L26<80
 26061|  %366 = gep %102, i64 1136                                                                                             ;L1511<80
 26062|  %367 = load i32, ptr %366, , !!8                                                                                      ;L1511<80
 26063|     ;; mult = i32 %367
 26064|  %368 = icmp eq i32 %367, 0                                                                                            ;L1512<80
 26065|  br i1 %368, label %369, label %372                                                                                    ;L1512<80
 26066| 
 26067| 369: ; preds = %363
 26068|  %370 = gep %102, i64 1664                                                                                             ;L1513<80
 26069|  %371 = load i64, ptr %370, , !!8                                                                                      ;L1513<80
 26070|  br label %379                                                                                                         ;L1512<80
 26071| 
 26072| 372: ; preds = %363
 26073|  %373 = sext i32 %367 to i64                                                                                           ;L1511<80
 26074|     ;; mult = i64 %373
 26075|  %374 = gep %102, i64 1664                                                                                             ;L1515<80
 26076|  %375 = load i64, ptr %374, , !!8                                                                                      ;L1515<80
 26077|  %376 = add nsw i64 %373, 100                                                                                          ;L1515<80
 26078|  %377 = mul i64 %375, %376                                                                                             ;L1515<80
 26079|  %378 = udiv i64 %377, 100                                                                                             ;L1515<80
 26080|  br label %379                                                                                                         ;L1512<80
 26081| 
 26082| 379: ; preds = %372, %369
 26083|  %380 = phi i64 [ %371, %369 ], [ %378, %372 ]                                                                         ;L0<80
 26084|  %381 = gep %140, i64 1136                                                                                             ;L1511<80
 26085|  %382 = load i32, ptr %381, , !!8                                                                                      ;L1511<80
 26086|     ;; mult = i32 %382
 26087|  %383 = icmp eq i32 %382, 0                                                                                            ;L1512<80
 26088|  br i1 %383, label %384, label %387                                                                                    ;L1512<80
 26089| 
 26090| 384: ; preds = %379
 26091|  %385 = gep %140, i64 1664                                                                                             ;L1513<80
 26092|  %386 = load i64, ptr %385, , !!8                                                                                      ;L1513<80
 26093|  br label %394                                                                                                         ;L1512<80
 26094| 
 26095| 387: ; preds = %379
 26096|  %388 = sext i32 %382 to i64                                                                                           ;L1511<80
 26097|     ;; mult = i64 %388
 26098|  %389 = gep %140, i64 1664                                                                                             ;L1515<80
 26099|  %390 = load i64, ptr %389, , !!8                                                                                      ;L1515<80
 26100|  %391 = add nsw i64 %388, 100                                                                                          ;L1515<80
 26101|  %392 = mul i64 %390, %391                                                                                             ;L1515<80
 26102|  %393 = udiv i64 %392, 100                                                                                             ;L1515<80
 26103|  br label %394                                                                                                         ;L1512<80
 26104| 
 26105| 394: ; preds = %387, %384
 26106|  %395 = phi i64 [ %386, %384 ], [ %393, %387 ]                                                                         ;L0<80
 26108|  %396 = gep %140, i64 1632                                                                                             ;L2158<81
 26109|  %397 = load i64, ptr %396, , !!8                                                                                      ;L2158<81
 26110|     ;; x1 = i64 %397
 26111|     ;; self = i64 %397
 26112|  %398 = gep %140, i64 1640                                                                                             ;L2158<81
 26113|  %399 = load i64, ptr %398, , !!8                                                                                      ;L2158<81
 26114|     ;; y1 = i64 %399
 26115|     ;; self = i64 %399
 26116|  %400 = gep %102, i64 1632                                                                                             ;L2158<81
 26117|  %401 = load i64, ptr %400, , !!8                                                                                      ;L2158<81
 26118|     ;; x2 = i64 %401
 26119|     ;; other = i64 %401
 26120|  %402 = gep %102, i64 1640                                                                                             ;L2158<81
 26121|  %403 = load i64, ptr %402, , !!8                                                                                      ;L2158<81
 26122|     ;; y2 = i64 %403
 26123|     ;; other = i64 %403
 26124|  %404 = icmp ult i64 %397, %401                                                                                        ;L3147<7<2158<81
 26125|  %405 = sub nuw i64 %401, %397                                                                                         ;L3147<7<2158<81
 26126|  %406 = sub nuw i64 %397, %401                                                                                         ;L3147<7<2158<81
 26127|  %407 = select i1 %404, i64 %405, i64 %406                                                                             ;L3147<7<2158<81
 26128|     ;; dx = i64 %407
 26129|  %408 = icmp ult i64 %399, %403                                                                                        ;L3147<8<2158<81
 26130|  %409 = sub nuw i64 %403, %399                                                                                         ;L3147<8<2158<81
 26131|  %410 = sub nuw i64 %399, %403                                                                                         ;L3147<8<2158<81
 26132|  %411 = select i1 %408, i64 %409, i64 %410                                                                             ;L3147<8<2158<81
 26133|     ;; dy = i64 %411
 26134|  %412 = mul i64 %407, %407                                                                                             ;L9<2158<81
 26135|  %413 = mul i64 %411, %411                                                                                             ;L9<2158<81
 26136|  %414 = add i64 %413, %412                                                                                             ;L9<2158<81
 26137|     ;; dist_sq = i64 %414
 26138|  %415 = mul i64 %152, 30                                                                                               ;L82
 26139|  %416 = add i64 %357, %415                                                                                             ;L26<80
 26140|  %417 = add i64 %416, %365                                                                                             ;L26<80
 26141|  %418 = add i64 %417, %361                                                                                             ;L80
 26142|  %419 = add i64 %418, %362                                                                                             ;L80
 26143|  %420 = add i64 %419, %380                                                                                             ;L80
 26144|  %421 = add i64 %420, %395                                                                                             ;L82
 26145|     ;; max_dist = i64 %421
 26146|  %422 = mul i64 %421, %421                                                                                             ;L83
 26147|  %423 = icmp ugt i64 %414, %422                                                                                        ;L83
 26148|  br i1 %423, label %343, label %424                                                                                    ;L83
 26149| 
 26150| 424: ; preds = %394
 26153|  %425 = gep %140, i64 1472                                                                                             ;L84
 26154|  %426 = load i64, ptr %425, , !!8                                                                                      ;L84
 26155|  invoke void @ai::small_action4castNtB5_17SmallActionSkill23new(ptr sret([24 x i8]) %16, ptr %5, i64 %426)
 26156|  to label %427 unwind label %37                                                                                        ;L84
 26157| 
 26158| 427: ; preds = %424
 26159|  call void @llvm.memcpy.p0.p0.i64(ptr %17, ptr %16, i64 24, i1 false)                                                  ;L84
 26160|  %428 = gep %17, i64 177                                                                                               ;L84
 26161|  store i8 17, ptr %428,                                                                                                ;L84
 26163|  invoke fastcc void @ai::small_action15SmallActionPlayE4pushBX_(ptr %26, ptr %17)
 26164|  to label %429 unwind label %37                                                                                        ;L84
 26165| 
 26166| 429: ; preds = %427
 26168|  br label %343                                                                                                         ;L83
 26169| 
 26170| 430: ; preds = %343
 26171|     ;; ult = ptr %346
 26172|     ;; self = ptr %346
 26173|  %431 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity7can_ult(ptr %102)
 26174|  to label %432 unwind label %37                                                                                        ;L90
 26175| 
 26176| 432: ; preds = %430
 26177|  br i1 %431, label %433, label %779                                                                                    ;L90
 26178| 
 26179| 433: ; preds = %432
 26180|  %434 = gep %346, i64 40                                                                                               ;L90
 26181|  %435 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %434, ptr %102, ptr %140)
 26182|  to label %436 unwind label %37                                                                                        ;L90
 26183| 
 26184| 436: ; preds = %433
 26185|  br i1 %435, label %437, label %779                                                                                    ;L90
 26186| 
 26187| 437: ; preds = %436
 26188|  %438 = gep %346, i64 16                                                                                               ;L26<91
 26189|  %439 = load i64, ptr %438, , !!8                                                                                      ;L26<91
 26190|  %440 = gep %346, i64 24                                                                                               ;L26<91
 26191|  %441 = load i64, ptr %440, , !!8                                                                                      ;L26<91
 26192|  %442 = gep %102, i64 1080                                                                                             ;L26<91
 26193|  %443 = load i64, ptr %442, , !!8                                                                                      ;L26<91
 26194|  %444 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %346, ptr %102, ptr %140)
 26195|  to label %445 unwind label %37                                                                                        ;L91
 26196| 
 26197| 445: ; preds = %437
 26198|  %446 = add i64 %252, -1                                                                                               ;L26<91
 26199|  %447 = mul i64 %441, %446                                                                                             ;L26<91
 26200|  %448 = gep %102, i64 1136                                                                                             ;L1511<91
 26201|  %449 = load i32, ptr %448, , !!8                                                                                      ;L1511<91
 26202|     ;; mult = i32 %449
 26203|  %450 = icmp eq i32 %449, 0                                                                                            ;L1512<91
 26204|  br i1 %450, label %451, label %454                                                                                    ;L1512<91
 26205| 
 26206| 451: ; preds = %445
 26207|  %452 = gep %102, i64 1664                                                                                             ;L1513<91
 26208|  %453 = load i64, ptr %452, , !!8                                                                                      ;L1513<91
 26209|  br label %461                                                                                                         ;L1512<91
 26210| 
 26211| 454: ; preds = %445
 26212|  %455 = sext i32 %449 to i64                                                                                           ;L1511<91
 26213|     ;; mult = i64 %455
 26214|  %456 = gep %102, i64 1664                                                                                             ;L1515<91
 26215|  %457 = load i64, ptr %456, , !!8                                                                                      ;L1515<91
 26216|  %458 = add nsw i64 %455, 100                                                                                          ;L1515<91
 26217|  %459 = mul i64 %457, %458                                                                                             ;L1515<91
 26218|  %460 = udiv i64 %459, 100                                                                                             ;L1515<91
 26219|  br label %461                                                                                                         ;L1512<91
 26220| 
 26221| 461: ; preds = %454, %451
 26222|  %462 = phi i64 [ %453, %451 ], [ %460, %454 ]                                                                         ;L0<91
 26223|  %463 = gep %140, i64 1136                                                                                             ;L1511<91
 26224|  %464 = load i32, ptr %463, , !!8                                                                                      ;L1511<91
 26225|     ;; mult = i32 %464
 26226|  %465 = icmp eq i32 %464, 0                                                                                            ;L1512<91
 26227|  br i1 %465, label %466, label %469                                                                                    ;L1512<91
 26228| 
 26229| 466: ; preds = %461
 26230|  %467 = gep %140, i64 1664                                                                                             ;L1513<91
 26231|  %468 = load i64, ptr %467, , !!8                                                                                      ;L1513<91
 26232|  br label %476                                                                                                         ;L1512<91
 26233| 
 26234| 469: ; preds = %461
 26235|  %470 = sext i32 %464 to i64                                                                                           ;L1511<91
 26236|     ;; mult = i64 %470
 26237|  %471 = gep %140, i64 1664                                                                                             ;L1515<91
 26238|  %472 = load i64, ptr %471, , !!8                                                                                      ;L1515<91
 26239|  %473 = add nsw i64 %470, 100                                                                                          ;L1515<91
 26240|  %474 = mul i64 %472, %473                                                                                             ;L1515<91
 26241|  %475 = udiv i64 %474, 100                                                                                             ;L1515<91
 26242|  br label %476                                                                                                         ;L1512<91
 26243| 
 26244| 476: ; preds = %469, %466
 26245|  %477 = phi i64 [ %468, %466 ], [ %475, %469 ]                                                                         ;L0<91
 26247|  %478 = gep %140, i64 1632                                                                                             ;L2158<92
 26248|  %479 = load i64, ptr %478, , !!8                                                                                      ;L2158<92
 26249|     ;; x1 = i64 %479
 26250|     ;; self = i64 %479
 26251|  %480 = gep %140, i64 1640                                                                                             ;L2158<92
 26252|  %481 = load i64, ptr %480, , !!8                                                                                      ;L2158<92
 26253|     ;; y1 = i64 %481
 26254|     ;; self = i64 %481
 26255|  %482 = gep %102, i64 1632                                                                                             ;L2158<92
 26256|  %483 = load i64, ptr %482, , !!8                                                                                      ;L2158<92
 26257|     ;; x2 = i64 %483
 26258|     ;; other = i64 %483
 26259|  %484 = gep %102, i64 1640                                                                                             ;L2158<92
 26260|  %485 = load i64, ptr %484, , !!8                                                                                      ;L2158<92
 26261|     ;; y2 = i64 %485
 26262|     ;; other = i64 %485
 26263|  %486 = icmp ult i64 %479, %483                                                                                        ;L3147<7<2158<92
 26264|  %487 = sub nuw i64 %483, %479                                                                                         ;L3147<7<2158<92
 26265|  %488 = sub nuw i64 %479, %483                                                                                         ;L3147<7<2158<92
 26266|  %489 = select i1 %486, i64 %487, i64 %488                                                                             ;L3147<7<2158<92
 26267|     ;; dx = i64 %489
 26268|  %490 = icmp ult i64 %481, %485                                                                                        ;L3147<8<2158<92
 26269|  %491 = sub nuw i64 %485, %481                                                                                         ;L3147<8<2158<92
 26270|  %492 = sub nuw i64 %481, %485                                                                                         ;L3147<8<2158<92
 26271|  %493 = select i1 %490, i64 %491, i64 %492                                                                             ;L3147<8<2158<92
 26272|     ;; dy = i64 %493
 26273|  %494 = mul i64 %489, %489                                                                                             ;L9<2158<92
 26274|  %495 = mul i64 %493, %493                                                                                             ;L9<2158<92
 26275|  %496 = add i64 %495, %494                                                                                             ;L9<2158<92
 26276|     ;; dist_sq = i64 %496
 26277|  %497 = mul i64 %152, 30                                                                                               ;L93
 26278|  %498 = add i64 %439, %497                                                                                             ;L26<91
 26279|  %499 = add i64 %498, %447                                                                                             ;L26<91
 26280|  %500 = add i64 %499, %443                                                                                             ;L91
 26281|  %501 = add i64 %500, %444                                                                                             ;L91
 26282|  %502 = add i64 %501, %462                                                                                             ;L91
 26283|  %503 = add i64 %502, %477                                                                                             ;L93
 26284|     ;; max_dist = i64 %503
 26285|  %504 = mul i64 %503, %503                                                                                             ;L94
 26286|  %505 = icmp ugt i64 %496, %504                                                                                        ;L94
 26287|  br i1 %505, label %779, label %506                                                                                    ;L94
 26288| 
 26289| 506: ; preds = %476
 26292|  %507 = gep %140, i64 1472                                                                                             ;L95
 26293|  %508 = load i64, ptr %507, , !!8                                                                                      ;L95
 26294|  invoke void @ai::small_action4castNtB5_14SmallActionUlt3new(ptr sret([24 x i8]) %14, ptr %5, i64 %508)
 26295|  to label %509 unwind label %37                                                                                        ;L95
 26296| 
 26297| 509: ; preds = %506
 26298|  call void @llvm.memcpy.p0.p0.i64(ptr %15, ptr %14, i64 24, i1 false)                                                  ;L95
 26299|  %510 = gep %15, i64 177                                                                                               ;L95
 26300|  store i8 18, ptr %510,                                                                                                ;L95
 26302|  invoke fastcc void @ai::small_action15SmallActionPlayE4pushBX_(ptr %26, ptr %15)
 26303|  to label %511 unwind label %37                                                                                        ;L95
 26304| 
 26305| 511: ; preds = %509
 26307|  br label %779                                                                                                         ;L94
 26308| 
 26309| 512: ; preds = %145
 26310|     ;; team = !DIArgList(i64 1, i64 %34)
 26311|  %513 = sub nuw nsw i64 1, %34                                                                                         ;L107
 26312|     ;; team = i64 %513
 26313|  %514 = getelementptr [5 x ptr], ptr %45, i64 %513                                                                     ;L1905<107
 26314|     ;; self = ptr undef
 26315|     ;; self = ptr undef
 26316|     ;; f = ptr %102
 26317|     ;; fold = ptr %102
 26320|     ;; f[8..+8] = ptr %102
 26321|     ;; self = ptr undef
 26324|     ;; self = ptr undef
 26325|     ;; count = i64 1
 26326|     ;; ptr = ptr %514
 26327|     ;; self = ptr %514
 26328|     ;; end_or_len = ptr %514
 26329|  %515 = load i64, ptr %102, , !!33786
 26330|  %516 = freeze i64 %515
 26331|  %517 = trunc i64 %516 to i1
 26332|  %518 = gep %102, i64 8
 26333|  %519 = load i64, ptr %518, , !!33786
 26334|  %520 = freeze i64 %519
 26335|  %521 = gep %102, i64 1632
 26336|  %522 = load i64, ptr %521, , !!33786
 26337|  %523 = gep %102, i64 1640
 26338|  %524 = load i64, ptr %523, , !!33786
 26339|  br i1 %517, label %525, label %629
 26340| 
 26341| 525: ; preds = %512
 26342|     ;; ptr = ptr %514
 26343|     ;; x = ptr %514
 26344|  %526 = load ptr, ptr %514, , !!33790, !!8                                                                             ;L2494<138<2897<107
 26349|  %527 = icmp eq ptr %526, null                                                                                         ;L49<2494<138<2897<107
 26350|  br i1 %527, label %545, label %528                                                                                    ;L49<2494<138<2897<107
 26351| 
 26352| 528: ; preds = %525
 26353|     ;; x = ptr %526
 26356|     ;; x = ptr %526
 26358|     ;; c = ptr %526
 26359|     ;; self = ptr %526
 26360|     ;; self = ptr %526
 26361|     ;; entity = ptr %102
 26362|     ;; other = ptr %102
 26363|  %529 = gep %526, i64 1632                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26364|  %530 = load i64, ptr %529, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26365|     ;; x1 = i64 %530
 26366|     ;; self = i64 %530
 26367|  %531 = gep %526, i64 1640                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26368|  %532 = load i64, ptr %531, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26369|     ;; y1 = i64 %532
 26370|     ;; self = i64 %532
 26371|     ;; x2 = i64 %522
 26372|     ;; other = i64 %522
 26373|     ;; y2 = i64 %524
 26374|     ;; other = i64 %524
 26375|  %533 = icmp ult i64 %530, %522                                                                                        ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26376|  %534 = sub nuw i64 %522, %530                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26377|  %535 = sub nuw i64 %530, %522                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26378|  %536 = select i1 %533, i64 %534, i64 %535                                                                             ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26379|     ;; dx = i64 %536
 26380|  %537 = icmp ult i64 %532, %524                                                                                        ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26381|  %538 = sub nuw i64 %524, %532                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26382|  %539 = sub nuw i64 %532, %524                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26383|  %540 = select i1 %537, i64 %538, i64 %539                                                                             ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26384|     ;; dy = i64 %540
 26385|  %541 = mul i64 %536, %536                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26386|  %542 = mul i64 %540, %540                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26387|  %543 = add i64 %542, %541                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26388|  %544 = icmp ult i64 %543, 16900000001                                                                                 ;L107<2893<50<2494<138<2897<107
 26389|  br i1 %544, label %788, label %545                                                                                    ;L2494<138<2897<107
 26390| 
 26391| 545: ; preds = %528, %525
 26392|     ;; self = ptr undef
 26393|     ;; count = i64 1
 26394|     ;; ptr = !DIArgList(ptr %514, i64 8)
 26395|     ;; self = !DIArgList(ptr %514, i64 8)
 26396|     ;; end_or_len = ptr %514
 26397|  %546 = gep %514, i64 8                                                                                                ;L656<185<2493<138<2897<107
 26398|     ;; ptr = ptr %546
 26399|     ;; x = ptr %546
 26400|  %547 = load ptr, ptr %546, , !!33790, !!8                                                                             ;L2494<138<2897<107
 26405|  %548 = icmp eq ptr %547, null                                                                                         ;L49<2494<138<2897<107
 26406|  br i1 %548, label %566, label %549                                                                                    ;L49<2494<138<2897<107
 26407| 
 26408| 549: ; preds = %545
 26409|     ;; x = ptr %547
 26412|     ;; x = ptr %547
 26414|     ;; c = ptr %547
 26415|     ;; self = ptr %547
 26416|     ;; self = ptr %547
 26417|     ;; entity = ptr %102
 26418|     ;; other = ptr %102
 26419|  %550 = gep %547, i64 1632                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26420|  %551 = load i64, ptr %550, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26421|     ;; x1 = i64 %551
 26422|     ;; self = i64 %551
 26423|  %552 = gep %547, i64 1640                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26424|  %553 = load i64, ptr %552, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26425|     ;; y1 = i64 %553
 26426|     ;; self = i64 %553
 26427|     ;; x2 = i64 %522
 26428|     ;; other = i64 %522
 26429|     ;; y2 = i64 %524
 26430|     ;; other = i64 %524
 26431|  %554 = icmp ult i64 %551, %522                                                                                        ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26432|  %555 = sub nuw i64 %522, %551                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26433|  %556 = sub nuw i64 %551, %522                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26434|  %557 = select i1 %554, i64 %555, i64 %556                                                                             ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26435|     ;; dx = i64 %557
 26436|  %558 = icmp ult i64 %553, %524                                                                                        ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26437|  %559 = sub nuw i64 %524, %553                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26438|  %560 = sub nuw i64 %553, %524                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26439|  %561 = select i1 %558, i64 %559, i64 %560                                                                             ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26440|     ;; dy = i64 %561
 26441|  %562 = mul i64 %557, %557                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26442|  %563 = mul i64 %561, %561                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26443|  %564 = add i64 %563, %562                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26444|  %565 = icmp ult i64 %564, 16900000001                                                                                 ;L107<2893<50<2494<138<2897<107
 26445|  br i1 %565, label %788, label %566                                                                                    ;L2494<138<2897<107
 26446| 
 26447| 566: ; preds = %549, %545
 26448|     ;; self = ptr undef
 26449|     ;; count = i64 1
 26450|     ;; ptr = !DIArgList(ptr %514, i64 16)
 26451|     ;; self = !DIArgList(ptr %514, i64 16)
 26452|     ;; end_or_len = ptr %514
 26453|  %567 = gep %514, i64 16                                                                                               ;L656<185<2493<138<2897<107
 26454|     ;; ptr = ptr %567
 26455|     ;; x = ptr %567
 26456|  %568 = load ptr, ptr %567, , !!33790, !!8                                                                             ;L2494<138<2897<107
 26461|  %569 = icmp eq ptr %568, null                                                                                         ;L49<2494<138<2897<107
 26462|  br i1 %569, label %587, label %570                                                                                    ;L49<2494<138<2897<107
 26463| 
 26464| 570: ; preds = %566
 26465|     ;; x = ptr %568
 26468|     ;; x = ptr %568
 26470|     ;; c = ptr %568
 26471|     ;; self = ptr %568
 26472|     ;; self = ptr %568
 26473|     ;; entity = ptr %102
 26474|     ;; other = ptr %102
 26475|  %571 = gep %568, i64 1632                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26476|  %572 = load i64, ptr %571, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26477|     ;; x1 = i64 %572
 26478|     ;; self = i64 %572
 26479|  %573 = gep %568, i64 1640                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26480|  %574 = load i64, ptr %573, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26481|     ;; y1 = i64 %574
 26482|     ;; self = i64 %574
 26483|     ;; x2 = i64 %522
 26484|     ;; other = i64 %522
 26485|     ;; y2 = i64 %524
 26486|     ;; other = i64 %524
 26487|  %575 = icmp ult i64 %572, %522                                                                                        ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26488|  %576 = sub nuw i64 %522, %572                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26489|  %577 = sub nuw i64 %572, %522                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26490|  %578 = select i1 %575, i64 %576, i64 %577                                                                             ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26491|     ;; dx = i64 %578
 26492|  %579 = icmp ult i64 %574, %524                                                                                        ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26493|  %580 = sub nuw i64 %524, %574                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26494|  %581 = sub nuw i64 %574, %524                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26495|  %582 = select i1 %579, i64 %580, i64 %581                                                                             ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26496|     ;; dy = i64 %582
 26497|  %583 = mul i64 %578, %578                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26498|  %584 = mul i64 %582, %582                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26499|  %585 = add i64 %584, %583                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26500|  %586 = icmp ult i64 %585, 16900000001                                                                                 ;L107<2893<50<2494<138<2897<107
 26501|  br i1 %586, label %788, label %587                                                                                    ;L2494<138<2897<107
 26502| 
 26503| 587: ; preds = %570, %566
 26504|     ;; self = ptr undef
 26505|     ;; count = i64 1
 26506|     ;; ptr = !DIArgList(ptr %514, i64 24)
 26507|     ;; self = !DIArgList(ptr %514, i64 24)
 26508|     ;; end_or_len = ptr %514
 26509|  %588 = gep %514, i64 24                                                                                               ;L656<185<2493<138<2897<107
 26510|     ;; ptr = ptr %588
 26511|     ;; x = ptr %588
 26512|  %589 = load ptr, ptr %588, , !!33790, !!8                                                                             ;L2494<138<2897<107
 26517|  %590 = icmp eq ptr %589, null                                                                                         ;L49<2494<138<2897<107
 26518|  br i1 %590, label %608, label %591                                                                                    ;L49<2494<138<2897<107
 26519| 
 26520| 591: ; preds = %587
 26521|     ;; x = ptr %589
 26524|     ;; x = ptr %589
 26526|     ;; c = ptr %589
 26527|     ;; self = ptr %589
 26528|     ;; self = ptr %589
 26529|     ;; entity = ptr %102
 26530|     ;; other = ptr %102
 26531|  %592 = gep %589, i64 1632                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26532|  %593 = load i64, ptr %592, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26533|     ;; x1 = i64 %593
 26534|     ;; self = i64 %593
 26535|  %594 = gep %589, i64 1640                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26536|  %595 = load i64, ptr %594, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26537|     ;; y1 = i64 %595
 26538|     ;; self = i64 %595
 26539|     ;; x2 = i64 %522
 26540|     ;; other = i64 %522
 26541|     ;; y2 = i64 %524
 26542|     ;; other = i64 %524
 26543|  %596 = icmp ult i64 %593, %522                                                                                        ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26544|  %597 = sub nuw i64 %522, %593                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26545|  %598 = sub nuw i64 %593, %522                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26546|  %599 = select i1 %596, i64 %597, i64 %598                                                                             ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26547|     ;; dx = i64 %599
 26548|  %600 = icmp ult i64 %595, %524                                                                                        ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26549|  %601 = sub nuw i64 %524, %595                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26550|  %602 = sub nuw i64 %595, %524                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26551|  %603 = select i1 %600, i64 %601, i64 %602                                                                             ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26552|     ;; dy = i64 %603
 26553|  %604 = mul i64 %599, %599                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26554|  %605 = mul i64 %603, %603                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26555|  %606 = add i64 %605, %604                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26556|  %607 = icmp ult i64 %606, 16900000001                                                                                 ;L107<2893<50<2494<138<2897<107
 26557|  br i1 %607, label %788, label %608                                                                                    ;L2494<138<2897<107
 26558| 
 26559| 608: ; preds = %591, %587
 26560|     ;; self = ptr undef
 26561|     ;; count = i64 1
 26562|     ;; ptr = !DIArgList(ptr %514, i64 32)
 26563|     ;; self = !DIArgList(ptr %514, i64 32)
 26564|     ;; end_or_len = ptr %514
 26565|  %609 = gep %514, i64 32                                                                                               ;L656<185<2493<138<2897<107
 26566|     ;; ptr = ptr %609
 26567|     ;; x = ptr %609
 26568|  %610 = load ptr, ptr %609, , !!33790, !!8                                                                             ;L2494<138<2897<107
 26573|  %611 = icmp eq ptr %610, null                                                                                         ;L49<2494<138<2897<107
 26574|  br i1 %611, label %786, label %612                                                                                    ;L49<2494<138<2897<107
 26575| 
 26576| 612: ; preds = %608
 26577|     ;; x = ptr %610
 26580|     ;; x = ptr %610
 26582|     ;; c = ptr %610
 26583|     ;; self = ptr %610
 26584|     ;; self = ptr %610
 26585|     ;; entity = ptr %102
 26586|     ;; other = ptr %102
 26587|  %613 = gep %610, i64 1632                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26588|  %614 = load i64, ptr %613, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26589|     ;; x1 = i64 %614
 26590|     ;; self = i64 %614
 26591|  %615 = gep %610, i64 1640                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26592|  %616 = load i64, ptr %615, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26593|     ;; y1 = i64 %616
 26594|     ;; self = i64 %616
 26595|     ;; x2 = i64 %522
 26596|     ;; other = i64 %522
 26597|     ;; y2 = i64 %524
 26598|     ;; other = i64 %524
 26599|  %617 = icmp ult i64 %614, %522                                                                                        ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26600|  %618 = sub nuw i64 %522, %614                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26601|  %619 = sub nuw i64 %614, %522                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26602|  %620 = select i1 %617, i64 %618, i64 %619                                                                             ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26603|     ;; dx = i64 %620
 26604|  %621 = icmp ult i64 %616, %524                                                                                        ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26605|  %622 = sub nuw i64 %524, %616                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26606|  %623 = sub nuw i64 %616, %524                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26607|  %624 = select i1 %621, i64 %622, i64 %623                                                                             ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26608|     ;; dy = i64 %624
 26609|  %625 = mul i64 %620, %620                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26610|  %626 = mul i64 %624, %624                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26611|  %627 = add i64 %626, %625                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26612|  %628 = icmp ult i64 %627, 16900000001                                                                                 ;L107<2893<50<2494<138<2897<107
 26613|  br i1 %628, label %788, label %786                                                                                    ;L2494<138<2897<107
 26614| 
 26615| 629: ; preds = %512
 26616|  %630 = icmp ult i64 %520, 2
 26617|     ;; ptr = ptr %514
 26618|     ;; ptr = ptr %514
 26619|     ;; x = ptr %514
 26620|     ;; x = ptr %514
 26621|  %631 = load ptr, ptr %514, , !!33790, !!8                                                                             ;L2494<138<2897<107
 26630|  %632 = icmp eq ptr %631, null                                                                                         ;L49<2494<138<2897<107
 26631|  br i1 %630, label %634, label %633
 26632| 
 26633| 633: ; preds = %629
 26634|  br i1 %632, label %763, label %761                                                                                    ;L49<2494<138<2897<107
 26635| 
 26636| 634: ; preds = %629
 26637|  br i1 %632, label %657, label %635                                                                                    ;L49<2494<138<2897<107
 26638| 
 26639| 635: ; preds = %634
 26640|     ;; x = ptr %631
 26643|     ;; x = ptr %631
 26645|     ;; c = ptr %631
 26646|     ;; self = ptr %631
 26647|     ;; self = ptr %631
 26648|     ;; entity = ptr %102
 26649|     ;; team = i64 %519
 26651|  %636 = gep %631, i64 56                                                                                               ;L122<1483<107<2893<50<2494<138<2897<107
 26652|  %637 = gepS %636, i64 %520                                                                                            ;L122<1483<107<2893<50<2494<138<2897<107
 26653|  %638 = load i64, ptr %637, , !!33790, !!8                                                                             ;L122<1483<107<2893<50<2494<138<2897<107
 26654|  %639 = icmp eq i64 %638, 0                                                                                            ;L122<1483<107<2893<50<2494<138<2897<107
 26655|  br i1 %639, label %640, label %657                                                                                    ;L107<2893<50<2494<138<2897<107
 26656| 
 26657| 640: ; preds = %635
 26658|     ;; other = ptr %102
 26659|  %641 = gep %631, i64 1632                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26660|  %642 = load i64, ptr %641, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26661|     ;; x1 = i64 %642
 26662|     ;; self = i64 %642
 26663|  %643 = gep %631, i64 1640                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26664|  %644 = load i64, ptr %643, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26665|     ;; y1 = i64 %644
 26666|     ;; self = i64 %644
 26667|     ;; x2 = i64 %522
 26668|     ;; other = i64 %522
 26669|     ;; y2 = i64 %524
 26670|     ;; other = i64 %524
 26671|  %645 = icmp ult i64 %642, %522                                                                                        ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26672|  %646 = sub nuw i64 %522, %642                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26673|  %647 = sub nuw i64 %642, %522                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26674|  %648 = select i1 %645, i64 %646, i64 %647                                                                             ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26675|     ;; dx = i64 %648
 26676|  %649 = icmp ult i64 %644, %524                                                                                        ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26677|  %650 = sub nuw i64 %524, %644                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26678|  %651 = sub nuw i64 %644, %524                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26679|  %652 = select i1 %649, i64 %650, i64 %651                                                                             ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26680|     ;; dy = i64 %652
 26681|  %653 = mul i64 %648, %648                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26682|  %654 = mul i64 %652, %652                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26683|  %655 = add i64 %654, %653                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26684|  %656 = icmp ult i64 %655, 16900000001                                                                                 ;L107<2893<50<2494<138<2897<107
 26685|  br i1 %656, label %788, label %657                                                                                    ;L2494<138<2897<107
 26686| 
 26687| 657: ; preds = %640, %635, %634
 26688|     ;; self = ptr undef
 26689|     ;; count = i64 1
 26690|     ;; ptr = !DIArgList(ptr %514, i64 8)
 26691|     ;; self = !DIArgList(ptr %514, i64 8)
 26692|     ;; end_or_len = ptr %514
 26693|  %658 = gep %514, i64 8                                                                                                ;L656<185<2493<138<2897<107
 26694|     ;; ptr = ptr %658
 26695|     ;; x = ptr %658
 26696|  %659 = load ptr, ptr %658, , !!33790, !!8                                                                             ;L2494<138<2897<107
 26701|  %660 = icmp eq ptr %659, null                                                                                         ;L49<2494<138<2897<107
 26702|  br i1 %660, label %683, label %661                                                                                    ;L49<2494<138<2897<107
 26703| 
 26704| 661: ; preds = %657
 26705|     ;; x = ptr %659
 26708|     ;; x = ptr %659
 26710|     ;; c = ptr %659
 26711|     ;; self = ptr %659
 26712|     ;; self = ptr %659
 26713|     ;; entity = ptr %102
 26714|     ;; team = i64 %519
 26716|  %662 = gep %659, i64 56                                                                                               ;L122<1483<107<2893<50<2494<138<2897<107
 26717|  %663 = gepS %662, i64 %520                                                                                            ;L122<1483<107<2893<50<2494<138<2897<107
 26718|  %664 = load i64, ptr %663, , !!33790, !!8                                                                             ;L122<1483<107<2893<50<2494<138<2897<107
 26719|  %665 = icmp eq i64 %664, 0                                                                                            ;L122<1483<107<2893<50<2494<138<2897<107
 26720|  br i1 %665, label %666, label %683                                                                                    ;L107<2893<50<2494<138<2897<107
 26721| 
 26722| 666: ; preds = %661
 26723|     ;; other = ptr %102
 26724|  %667 = gep %659, i64 1632                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26725|  %668 = load i64, ptr %667, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26726|     ;; x1 = i64 %668
 26727|     ;; self = i64 %668
 26728|  %669 = gep %659, i64 1640                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26729|  %670 = load i64, ptr %669, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26730|     ;; y1 = i64 %670
 26731|     ;; self = i64 %670
 26732|     ;; x2 = i64 %522
 26733|     ;; other = i64 %522
 26734|     ;; y2 = i64 %524
 26735|     ;; other = i64 %524
 26736|  %671 = icmp ult i64 %668, %522                                                                                        ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26737|  %672 = sub nuw i64 %522, %668                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26738|  %673 = sub nuw i64 %668, %522                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26739|  %674 = select i1 %671, i64 %672, i64 %673                                                                             ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26740|     ;; dx = i64 %674
 26741|  %675 = icmp ult i64 %670, %524                                                                                        ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26742|  %676 = sub nuw i64 %524, %670                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26743|  %677 = sub nuw i64 %670, %524                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26744|  %678 = select i1 %675, i64 %676, i64 %677                                                                             ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26745|     ;; dy = i64 %678
 26746|  %679 = mul i64 %674, %674                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26747|  %680 = mul i64 %678, %678                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26748|  %681 = add i64 %680, %679                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26749|  %682 = icmp ult i64 %681, 16900000001                                                                                 ;L107<2893<50<2494<138<2897<107
 26750|  br i1 %682, label %788, label %683                                                                                    ;L2494<138<2897<107
 26751| 
 26752| 683: ; preds = %666, %661, %657
 26753|     ;; self = ptr undef
 26754|     ;; count = i64 1
 26755|     ;; ptr = !DIArgList(ptr %514, i64 16)
 26756|     ;; self = !DIArgList(ptr %514, i64 16)
 26757|     ;; end_or_len = ptr %514
 26758|  %684 = gep %514, i64 16                                                                                               ;L656<185<2493<138<2897<107
 26759|     ;; ptr = ptr %684
 26760|     ;; x = ptr %684
 26761|  %685 = load ptr, ptr %684, , !!33790, !!8                                                                             ;L2494<138<2897<107
 26766|  %686 = icmp eq ptr %685, null                                                                                         ;L49<2494<138<2897<107
 26767|  br i1 %686, label %709, label %687                                                                                    ;L49<2494<138<2897<107
 26768| 
 26769| 687: ; preds = %683
 26770|     ;; x = ptr %685
 26773|     ;; x = ptr %685
 26775|     ;; c = ptr %685
 26776|     ;; self = ptr %685
 26777|     ;; self = ptr %685
 26778|     ;; entity = ptr %102
 26779|     ;; team = i64 %519
 26781|  %688 = gep %685, i64 56                                                                                               ;L122<1483<107<2893<50<2494<138<2897<107
 26782|  %689 = gepS %688, i64 %520                                                                                            ;L122<1483<107<2893<50<2494<138<2897<107
 26783|  %690 = load i64, ptr %689, , !!33790, !!8                                                                             ;L122<1483<107<2893<50<2494<138<2897<107
 26784|  %691 = icmp eq i64 %690, 0                                                                                            ;L122<1483<107<2893<50<2494<138<2897<107
 26785|  br i1 %691, label %692, label %709                                                                                    ;L107<2893<50<2494<138<2897<107
 26786| 
 26787| 692: ; preds = %687
 26788|     ;; other = ptr %102
 26789|  %693 = gep %685, i64 1632                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26790|  %694 = load i64, ptr %693, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26791|     ;; x1 = i64 %694
 26792|     ;; self = i64 %694
 26793|  %695 = gep %685, i64 1640                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26794|  %696 = load i64, ptr %695, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26795|     ;; y1 = i64 %696
 26796|     ;; self = i64 %696
 26797|     ;; x2 = i64 %522
 26798|     ;; other = i64 %522
 26799|     ;; y2 = i64 %524
 26800|     ;; other = i64 %524
 26801|  %697 = icmp ult i64 %694, %522                                                                                        ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26802|  %698 = sub nuw i64 %522, %694                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26803|  %699 = sub nuw i64 %694, %522                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26804|  %700 = select i1 %697, i64 %698, i64 %699                                                                             ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26805|     ;; dx = i64 %700
 26806|  %701 = icmp ult i64 %696, %524                                                                                        ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26807|  %702 = sub nuw i64 %524, %696                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26808|  %703 = sub nuw i64 %696, %524                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26809|  %704 = select i1 %701, i64 %702, i64 %703                                                                             ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26810|     ;; dy = i64 %704
 26811|  %705 = mul i64 %700, %700                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26812|  %706 = mul i64 %704, %704                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26813|  %707 = add i64 %706, %705                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26814|  %708 = icmp ult i64 %707, 16900000001                                                                                 ;L107<2893<50<2494<138<2897<107
 26815|  br i1 %708, label %788, label %709                                                                                    ;L2494<138<2897<107
 26816| 
 26817| 709: ; preds = %692, %687, %683
 26818|     ;; self = ptr undef
 26819|     ;; count = i64 1
 26820|     ;; ptr = !DIArgList(ptr %514, i64 24)
 26821|     ;; self = !DIArgList(ptr %514, i64 24)
 26822|     ;; end_or_len = ptr %514
 26823|  %710 = gep %514, i64 24                                                                                               ;L656<185<2493<138<2897<107
 26824|     ;; ptr = ptr %710
 26825|     ;; x = ptr %710
 26826|  %711 = load ptr, ptr %710, , !!33790, !!8                                                                             ;L2494<138<2897<107
 26831|  %712 = icmp eq ptr %711, null                                                                                         ;L49<2494<138<2897<107
 26832|  br i1 %712, label %735, label %713                                                                                    ;L49<2494<138<2897<107
 26833| 
 26834| 713: ; preds = %709
 26835|     ;; x = ptr %711
 26838|     ;; x = ptr %711
 26840|     ;; c = ptr %711
 26841|     ;; self = ptr %711
 26842|     ;; self = ptr %711
 26843|     ;; entity = ptr %102
 26844|     ;; team = i64 %519
 26846|  %714 = gep %711, i64 56                                                                                               ;L122<1483<107<2893<50<2494<138<2897<107
 26847|  %715 = gepS %714, i64 %520                                                                                            ;L122<1483<107<2893<50<2494<138<2897<107
 26848|  %716 = load i64, ptr %715, , !!33790, !!8                                                                             ;L122<1483<107<2893<50<2494<138<2897<107
 26849|  %717 = icmp eq i64 %716, 0                                                                                            ;L122<1483<107<2893<50<2494<138<2897<107
 26850|  br i1 %717, label %718, label %735                                                                                    ;L107<2893<50<2494<138<2897<107
 26851| 
 26852| 718: ; preds = %713
 26853|     ;; other = ptr %102
 26854|  %719 = gep %711, i64 1632                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26855|  %720 = load i64, ptr %719, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26856|     ;; x1 = i64 %720
 26857|     ;; self = i64 %720
 26858|  %721 = gep %711, i64 1640                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26859|  %722 = load i64, ptr %721, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26860|     ;; y1 = i64 %722
 26861|     ;; self = i64 %722
 26862|     ;; x2 = i64 %522
 26863|     ;; other = i64 %522
 26864|     ;; y2 = i64 %524
 26865|     ;; other = i64 %524
 26866|  %723 = icmp ult i64 %720, %522                                                                                        ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26867|  %724 = sub nuw i64 %522, %720                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26868|  %725 = sub nuw i64 %720, %522                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26869|  %726 = select i1 %723, i64 %724, i64 %725                                                                             ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26870|     ;; dx = i64 %726
 26871|  %727 = icmp ult i64 %722, %524                                                                                        ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26872|  %728 = sub nuw i64 %524, %722                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26873|  %729 = sub nuw i64 %722, %524                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26874|  %730 = select i1 %727, i64 %728, i64 %729                                                                             ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26875|     ;; dy = i64 %730
 26876|  %731 = mul i64 %726, %726                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26877|  %732 = mul i64 %730, %730                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26878|  %733 = add i64 %732, %731                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26879|  %734 = icmp ult i64 %733, 16900000001                                                                                 ;L107<2893<50<2494<138<2897<107
 26880|  br i1 %734, label %788, label %735                                                                                    ;L2494<138<2897<107
 26881| 
 26882| 735: ; preds = %718, %713, %709
 26883|     ;; self = ptr undef
 26884|     ;; count = i64 1
 26885|     ;; ptr = !DIArgList(ptr %514, i64 32)
 26886|     ;; self = !DIArgList(ptr %514, i64 32)
 26887|     ;; end_or_len = ptr %514
 26888|  %736 = gep %514, i64 32                                                                                               ;L656<185<2493<138<2897<107
 26889|     ;; ptr = ptr %736
 26890|     ;; x = ptr %736
 26891|  %737 = load ptr, ptr %736, , !!33790, !!8                                                                             ;L2494<138<2897<107
 26896|  %738 = icmp eq ptr %737, null                                                                                         ;L49<2494<138<2897<107
 26897|  br i1 %738, label %786, label %739                                                                                    ;L49<2494<138<2897<107
 26898| 
 26899| 739: ; preds = %735
 26900|     ;; x = ptr %737
 26903|     ;; x = ptr %737
 26905|     ;; c = ptr %737
 26906|     ;; self = ptr %737
 26907|     ;; self = ptr %737
 26908|     ;; entity = ptr %102
 26909|     ;; team = i64 %519
 26911|  %740 = gep %737, i64 56                                                                                               ;L122<1483<107<2893<50<2494<138<2897<107
 26912|  %741 = gepS %740, i64 %520                                                                                            ;L122<1483<107<2893<50<2494<138<2897<107
 26913|  %742 = load i64, ptr %741, , !!33790, !!8                                                                             ;L122<1483<107<2893<50<2494<138<2897<107
 26914|  %743 = icmp eq i64 %742, 0                                                                                            ;L122<1483<107<2893<50<2494<138<2897<107
 26915|  br i1 %743, label %744, label %786                                                                                    ;L107<2893<50<2494<138<2897<107
 26916| 
 26917| 744: ; preds = %739
 26918|     ;; other = ptr %102
 26919|  %745 = gep %737, i64 1632                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26920|  %746 = load i64, ptr %745, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26921|     ;; x1 = i64 %746
 26922|     ;; self = i64 %746
 26923|  %747 = gep %737, i64 1640                                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26924|  %748 = load i64, ptr %747, , !!33790, !!8                                                                             ;L2158<107<2893<50<2494<138<2897<107
 26925|     ;; y1 = i64 %748
 26926|     ;; self = i64 %748
 26927|     ;; x2 = i64 %522
 26928|     ;; other = i64 %522
 26929|     ;; y2 = i64 %524
 26930|     ;; other = i64 %524
 26931|  %749 = icmp ult i64 %746, %522                                                                                        ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26932|  %750 = sub nuw i64 %522, %746                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26933|  %751 = sub nuw i64 %746, %522                                                                                         ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26934|  %752 = select i1 %749, i64 %750, i64 %751                                                                             ;L3147<7<2158<107<2893<50<2494<138<2897<107
 26935|     ;; dx = i64 %752
 26936|  %753 = icmp ult i64 %748, %524                                                                                        ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26937|  %754 = sub nuw i64 %524, %748                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26938|  %755 = sub nuw i64 %748, %524                                                                                         ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26939|  %756 = select i1 %753, i64 %754, i64 %755                                                                             ;L3147<8<2158<107<2893<50<2494<138<2897<107
 26940|     ;; dy = i64 %756
 26941|  %757 = mul i64 %752, %752                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26942|  %758 = mul i64 %756, %756                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26943|  %759 = add i64 %758, %757                                                                                             ;L9<2158<107<2893<50<2494<138<2897<107
 26944|  %760 = icmp ult i64 %759, 16900000001                                                                                 ;L107<2893<50<2494<138<2897<107
 26945|  br i1 %760, label %788, label %786                                                                                    ;L2494<138<2897<107
 26946| 
 26947| 761: ; preds = %775, %771, %767, %763, %633
 26948|     ;; x = ptr %631
 26951|     ;; x = ptr %631
 26953|     ;; c = ptr %631
 26954|     ;; self = ptr %631
 26955|     ;; self = ptr %631
 26956|     ;; entity = ptr %102
 26957|     ;; team = i64 %519
 26958|  invoke void @core::panicking18panic_bounds_check(i64 %520, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 26959|  to label %762 unwind label %37                                                                                        ;L1483<107<2893<50<2494<138<2897<107
 26960| 
 26961| 762: ; preds = %761
 26962|  unreachable                                                                                                           ;L1483<107<2893<50<2494<138<2897<107
 26963| 
 26964| 763: ; preds = %633
 26965|     ;; self = ptr undef
 26966|     ;; count = i64 1
 26967|     ;; ptr = !DIArgList(ptr %514, i64 8)
 26968|     ;; self = !DIArgList(ptr %514, i64 8)
 26969|     ;; end_or_len = ptr %514
 26970|  %764 = gep %514, i64 8                                                                                                ;L2494<138<2897<107
 26971|     ;; ptr = ptr %764
 26972|     ;; x = ptr %764
 26973|  %765 = load ptr, ptr %764, , !!33790, !!8                                                                             ;L2494<138<2897<107
 26978|  %766 = icmp eq ptr %765, null                                                                                         ;L49<2494<138<2897<107
 26979|  br i1 %766, label %767, label %761                                                                                    ;L49<2494<138<2897<107
 26980| 
 26981| 767: ; preds = %763
 26982|     ;; self = ptr undef
 26983|     ;; count = i64 1
 26984|     ;; ptr = !DIArgList(ptr %514, i64 16)
 26985|     ;; self = !DIArgList(ptr %514, i64 16)
 26986|     ;; end_or_len = ptr %514
 26987|  %768 = gep %514, i64 16                                                                                               ;L2494<138<2897<107
 26988|     ;; ptr = ptr %768
 26989|     ;; x = ptr %768
 26990|  %769 = load ptr, ptr %768, , !!33790, !!8                                                                             ;L2494<138<2897<107
 26995|  %770 = icmp eq ptr %769, null                                                                                         ;L49<2494<138<2897<107
 26996|  br i1 %770, label %771, label %761                                                                                    ;L49<2494<138<2897<107
 26997| 
 26998| 771: ; preds = %767
 26999|     ;; self = ptr undef
 27000|     ;; count = i64 1
 27001|     ;; ptr = !DIArgList(ptr %514, i64 24)
 27002|     ;; self = !DIArgList(ptr %514, i64 24)
 27003|     ;; end_or_len = ptr %514
 27004|  %772 = gep %514, i64 24                                                                                               ;L2494<138<2897<107
 27005|     ;; ptr = ptr %772
 27006|     ;; x = ptr %772
 27007|  %773 = load ptr, ptr %772, , !!33790, !!8                                                                             ;L2494<138<2897<107
 27012|  %774 = icmp eq ptr %773, null                                                                                         ;L49<2494<138<2897<107
 27013|  br i1 %774, label %775, label %761                                                                                    ;L49<2494<138<2897<107
 27014| 
 27015| 775: ; preds = %771
 27016|     ;; self = ptr undef
 27017|     ;; count = i64 1
 27018|     ;; ptr = !DIArgList(ptr %514, i64 32)
 27019|     ;; self = !DIArgList(ptr %514, i64 32)
 27020|     ;; end_or_len = ptr %514
 27021|  %776 = gep %514, i64 32                                                                                               ;L2494<138<2897<107
 27022|     ;; ptr = ptr %776
 27023|     ;; x = ptr %776
 27024|  %777 = load ptr, ptr %776, , !!33790, !!8                                                                             ;L2494<138<2897<107
 27029|  %778 = icmp eq ptr %777, null                                                                                         ;L49<2494<138<2897<107
 27030|  br i1 %778, label %786, label %761                                                                                    ;L49<2494<138<2897<107
 27031| 
 27032| 779: ; preds = %511, %476, %436, %432, %343
 27033|  %780 = gep %28, i64 8                                                                                                 ;L102
 27034|  %781 = load ptr, ptr %780, , !!8, !!8                                                                                 ;L102
 27035|  %782 = gep %781, i64 4856                                                                                             ;L102
 27036|  %783 = load i64, ptr %782, , !!8                                                                                      ;L102
 27037|     ;; tps = i64 %783
 27038|  %784 = mul i64 %783, 5                                                                                                ;L103
 27039|  %785 = invoke { i64, i64 } @ai::plan_legacy5steal22steal_damage_entry_pos(ptr %28, ptr %102, ptr %140, i64 %784)
 27040|  to label %854 unwind label %37                                                                                        ;L103
 27041| 
 27042| 786: ; preds = %775, %744, %739, %735, %612, %608
 27043|     ;; danger_near = i1 false
 27044|  %787 = icmp eq i8 %52, 2                                                                                              ;L111
 27045|  br i1 %787, label %871, label %789                                                                                    ;L111
 27046| 
 27047| 788: ; preds = %744, %718, %692, %666, %640, %612, %591, %570, %549, %528
 27048|     ;; danger_near = i1 true
 27051|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %10, ptr %5, ptr %4, i64 5, i1 zeroext true)
 27052|  to label %845 unwind label %37                                                                                        ;L110
 27053| 
 27054| 789: ; preds = %786
 27055|  %790 = trunc nuw i8 %52 to i1                                                                                         ;L111
 27056|     ;; target = i1 %790
 27057|  %791 = gep %28, i64 32                                                                                                ;L112
 27058|  %792 = load ptr, ptr %791, , !!8, !!8                                                                                 ;L112
 27059|  %793 = invoke i64 @ai::plan_legacy5steal15steal_wait_bush(i1 zeroext %790, i64 %34, ptr %102, ptr %792)
 27060|  to label %794 unwind label %37                                                                                        ;L112
 27061| 
 27062| 794: ; preds = %789
 27063|     ;; bush = i64 %793
 27068|     ;; index = i64 0
 27069|     ;; self = i64 0
 27070|     ;; self = i8 %52
 27071|  %795 = icmp eq i8 %52, 0                                                                                              ;L2775<26<113
 27072|  %796 = load ptr, ptr %44, , !!8, !!8                                                                                  ;L0<113
 27073|  %797 = load ptr, ptr %88, , !!8, !!8                                                                                  ;L0<113
 27074|  %798 = gep %797, i64 64                                                                                               ;L0<113
 27075|  %799 = load ptr, ptr %798, , !!8                                                                                      ;L0<113
 27076|  br i1 %795, label %805, label %800                                                                                    ;L2775<26<113
 27077| 
 27078| 800: ; preds = %794
 27079|  %801 = invoke { i64, ptr } %799(ptr %796)
 27080|  to label %802 unwind label %37                                                                                        ;L28<113
 27081| 
 27082| 802: ; preds = %800
 27083|  %803 = extractvalue { i64, ptr } %801, 0                                                                              ;L28<113
 27084|     ;; self[0..+8] = i64 %803
 27086|  %804 = icmp eq i64 %803, 0                                                                                            ;L231<28<113
 27087|  br i1 %804, label %810, label %830                                                                                    ;L231<28<113
 27088| 
 27089| 805: ; preds = %794
 27090|  %806 = invoke { i64, ptr } %799(ptr %796)
 27091|  to label %807 unwind label %37                                                                                        ;L27<113
 27092| 
 27093| 807: ; preds = %805
 27094|  %808 = extractvalue { i64, ptr } %806, 0                                                                              ;L27<113
 27095|     ;; self[0..+8] = i64 %808
 27097|  %809 = icmp eq i64 %808, 0                                                                                            ;L231<27<113
 27098|  br i1 %809, label %810, label %819                                                                                    ;L231<27<113
 27099| 
 27100| 810: ; preds = %807, %802
 27101|  %811 = phi { i64, ptr } [ %801, %802 ], [ %806, %807 ]
 27102|  %812 = phi i64 [ 456, %802 ], [ 408, %807 ]
 27103|  %813 = extractvalue { i64, ptr } %811, 1                                                                              ;L0<113
 27104|  %814 = icmp ne ptr %813, null
 27105|  tail call void @llvm.assume(i1 %814)
 27106|  %815 = gep %813, i64 %812                                                                                             ;L0<113
 27107|     ;; self = ptr %815
 27108|     ;; self = ptr %815
 27109|     ;; self = ptr %815
 27110|     ;; live_list = ptr %815
 27111|  %816 = gep %815, i64 16                                                                                               ;L1864<3787<30<113
 27112|  %817 = load i64, ptr %816, , !!8                                                                                      ;L1864<3787<30<113
 27115|     ;; self[8..+8] = i64 %817
 27116|     ;; slice[8..+8] = i64 %817
 27117|  %818 = icmp eq i64 %817, 0                                                                                            ;L219<576<30<113
 27118|  br i1 %818, label %835, label %821                                                                                    ;L219<576<30<113
 27119| 
 27120| 819: ; preds = %807
 27121|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.155) #31
 27122|  to label %820 unwind label %37                                                                                        ;L1013<27<113
 27123| 
 27124| 820: ; preds = %819
 27125|  unreachable                                                                                                           ;L1013<27<113
 27126| 
 27127| 821: ; preds = %810
 27128|  %822 = gep %815, i64 8                                                                                                ;L614<609<296<1968<1864<3787<30<113
 27129|  %823 = load ptr, ptr %822, , !!8, !!8                                                                                 ;L614<609<296<1968<1864<3787<30<113
 27130|     ;; self[0..+8] = ptr %823
 27131|     ;; slice[0..+8] = ptr %823
 27132|     ;; self = ptr %823
 27133|  %824 = load ptr, ptr %44, , !!8, !!8                                                                                  ;L30<113
 27134|  %825 = load ptr, ptr %88, , !!8, !!8                                                                                  ;L30<113
 27135|     ;; f[0..+8] = ptr %824
 27136|     ;; f[8..+8] = ptr %825
 27137|     ;; x = ptr %823
 27138|     ;; id = ptr %823
 27139|  %826 = load i64, ptr %823, , !!8                                                                                      ;L30<1543<30<113
 27140|  %827 = gep %825, i64 496                                                                                              ;L30<1543<30<113
 27141|  %828 = load ptr, ptr %827, , !!8                                                                                      ;L30<1543<30<113
 27142|  %829 = invoke ptr %828(ptr %824, i64 %826)
 27143|  to label %832 unwind label %37                                                                                        ;L30<1543<30<113
 27144| 
 27145| 830: ; preds = %802
 27146|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.156) #31
 27147|  to label %831 unwind label %37                                                                                        ;L1013<28<113
 27148| 
 27149| 831: ; preds = %830
 27150|  unreachable                                                                                                           ;L1013<28<113
 27151| 
 27152| 832: ; preds = %821
 27153|     ;; self = ptr %829
 27154|  %833 = icmp eq ptr %829, null                                                                                         ;L1011<113
 27155|  br i1 %833, label %835, label %834                                                                                    ;L1011<113
 27156| 
 27157| 834: ; preds = %832
 27158|  invoke void @ai::small_action6aroundNtB5_21SmallActionAroundBush15new_with_target(ptr sret([120 x i8]) %8, ptr %5, ptr %829, i64 %793, i8 1)
 27159|  to label %836 unwind label %37                                                                                        ;L113
 27160| 
 27161| 835: ; preds = %832, %810
 27162|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.159) #31
 27163|  to label %39 unwind label %37                                                                                         ;L1013<113
 27164| 
 27165| 836: ; preds = %834
 27166|  call void @llvm.memcpy.p0.p0.i64(ptr %9, ptr %8, i64 120, i1 false)                                                   ;L113
 27167|  %837 = gep %9, i64 177                                                                                                ;L113
 27168|  store i8 12, ptr %837,                                                                                                ;L113
 27171|     ;; self = ptr %26
 27172|     ;; self = ptr %26
 27173|     ;; value = ptr %9
 27174|     ;; src = ptr %9
 27175|     ;; additional = i64 1
 27176|     ;; needed_extra_cap = i64 1
 27177|     ;; needed_extra_cap = i64 1
 27178|     ;; strategy = i8 1
 27179|     ;; self = ptr %26
 27180|     ;; self = ptr %26
 27181|     ;; self = ptr %26
 27182|     ;; self = ptr %26
 27183|     ;; used_cap = i64 0
 27184|     ;; used_cap = i64 0
 27185|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %26, i64 0, i64 1, i1 zeroext true)
 27186|  to label %840 unwind label %838, !!33953                                                                              ;L619<430<738<1429<113
 27187| 
 27188| 838: ; preds = %836
 27189|  %839 = cleanuppad within none []
 27190|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %9) #30 [ "funclet"(token %839) ], !!33934 ;L1436<113
 27191|  cleanupret from %839 unwind label %37
 27192| 
 27193| 840: ; preds = %836
 27194|  %841 = load ptr, ptr %26, , !!33953                                                                                   ;L138<1432<113
 27195|  %842 = load i64, ptr %32, , !!33953                                                                                   ;L1432<113
 27196|     ;; self = ptr %26
 27197|     ;; self = ptr %841
 27198|     ;; count = i64 %842
 27199|  %843 = gepS %841, i64 %842                                                                                            ;L961<1432<113
 27200|     ;; end = ptr %843
 27201|     ;; dst = ptr %843
 27202|  call void @llvm.memcpy.p0.p0.i64(ptr %843, ptr %9, i64 184, i1 false), !!33934                                        ;L1933<1433<113
 27203|  %844 = add i64 %842, 1                                                                                                ;L1434<113
 27204|  store i64 %844, ptr %32, , !!33953                                                                                    ;L1434<113
 27206|  br label %871                                                                                                         ;L111
 27207| 
 27208| 845: ; preds = %788
 27209|  call void @llvm.memcpy.p0.p0.i64(ptr %11, ptr %10, i64 136, i1 false)                                                 ;L110
 27210|  %846 = gep %11, i64 177                                                                                               ;L110
 27211|  store i8 3, ptr %846,                                                                                                 ;L110
 27214|     ;; self = ptr %26
 27215|     ;; self = ptr %26
 27216|     ;; value = ptr %11
 27217|     ;; src = ptr %11
 27218|     ;; additional = i64 1
 27219|     ;; needed_extra_cap = i64 1
 27220|     ;; needed_extra_cap = i64 1
 27221|     ;; strategy = i8 1
 27222|     ;; self = ptr %26
 27223|     ;; self = ptr %26
 27224|     ;; self = ptr %26
 27225|     ;; self = ptr %26
 27226|     ;; used_cap = i64 0
 27227|     ;; used_cap = i64 0
 27228|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %26, i64 0, i64 1, i1 zeroext true)
 27229|  to label %849 unwind label %847, !!33986                                                                              ;L619<430<738<1429<110
 27230| 
 27231| 847: ; preds = %845
 27232|  %848 = cleanuppad within none []
 27233|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %11) #30 [ "funclet"(token %848) ], !!33967 ;L1436<110
 27234|  cleanupret from %848 unwind label %37
 27235| 
 27236| 849: ; preds = %845
 27237|  %850 = load ptr, ptr %26, , !!33986                                                                                   ;L138<1432<110
 27238|  %851 = load i64, ptr %32, , !!33986                                                                                   ;L1432<110
 27239|     ;; self = ptr %26
 27240|     ;; self = ptr %850
 27241|     ;; count = i64 %851
 27242|  %852 = gepS %850, i64 %851                                                                                            ;L961<1432<110
 27243|     ;; end = ptr %852
 27244|     ;; dst = ptr %852
 27245|  call void @llvm.memcpy.p0.p0.i64(ptr %852, ptr %11, i64 184, i1 false), !!33967                                       ;L1933<1433<110
 27246|  %853 = add i64 %851, 1                                                                                                ;L1434<110
 27247|  store i64 %853, ptr %32, , !!33986                                                                                    ;L1434<110
 27249|  br label %871                                                                                                         ;L109
 27250| 
 27251| 854: ; preds = %779
 27252|  %855 = extractvalue { i64, i64 } %785, 0                                                                              ;L103
 27253|  %856 = extractvalue { i64, i64 } %785, 1                                                                              ;L103
 27254|     ;; x = i64 %855
 27255|     ;; y = i64 %856
 27258|  invoke void @ai::small_action6aroundNtB5_25SmallActionAroundPosition17new_with_out_line(ptr sret([184 x i8]) %12, ptr %3, ptr %5, i64 %855, i64 %856, i64 5, i8 1)
 27259|  to label %857 unwind label %37                                                                                        ;L104
 27260| 
 27261| 857: ; preds = %854
 27262|  call void @llvm.memcpy.p0.p0.i64(ptr %13, ptr %12, i64 184, i1 false)                                                 ;L104
 27265|     ;; self = ptr %26
 27266|     ;; self = ptr %26
 27267|     ;; value = ptr %13
 27268|     ;; src = ptr %13
 27269|     ;; additional = i64 1
 27270|     ;; needed_extra_cap = i64 1
 27271|     ;; needed_extra_cap = i64 1
 27272|     ;; strategy = i8 1
 27273|  %858 = load i64, ptr %32, , !!34017, !!8                                                                              ;L1428<104
 27274|     ;; self = ptr %26
 27275|  %859 = load i64, ptr %31, , !!34017, !!8                                                                              ;L149<1428<104
 27276|  %860 = icmp eq i64 %858, %859                                                                                         ;L1428<104
 27277|  br i1 %860, label %861, label %866                                                                                    ;L1428<104
 27278| 
 27279| 861: ; preds = %857
 27280|     ;; self = ptr %26
 27281|     ;; self = ptr %26
 27282|     ;; self = ptr %26
 27283|     ;; used_cap = i64 %858
 27284|     ;; used_cap = i64 %858
 27285|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %26, i64 %858, i64 1, i1 zeroext true)
 27286|  to label %862 unwind label %864, !!34017                                                                              ;L619<430<738<1429<104
 27287| 
 27288| 862: ; preds = %861
 27289|  %863 = load i64, ptr %32, , !!34017                                                                                   ;L1432<104
 27290|  br label %866                                                                                                         ;L619<430<738<1429<104
 27291| 
 27292| 864: ; preds = %861
 27293|  %865 = cleanuppad within none []
 27294|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %13) #30 [ "funclet"(token %865) ], !!34002 ;L1436<104
 27295|  cleanupret from %865 unwind label %37
 27296| 
 27297| 866: ; preds = %862, %857
 27298|  %867 = phi i64 [ %863, %862 ], [ %858, %857 ]                                                                         ;L1432<104
 27299|     ;; self = ptr %26
 27300|  %868 = load ptr, ptr %26, , !!34017, !!8, !!8                                                                         ;L138<1432<104
 27301|     ;; self = ptr %868
 27302|     ;; count = i64 %867
 27303|  %869 = gepS %868, i64 %867                                                                                            ;L961<1432<104
 27304|     ;; end = ptr %869
 27305|     ;; dst = ptr %869
 27306|  call void @llvm.memcpy.p0.p0.i64(ptr %869, ptr %13, i64 184, i1 false), !!34002                                       ;L1933<1433<104
 27307|  %870 = add i64 %867, 1                                                                                                ;L1434<104
 27308|  store i64 %870, ptr %32, , !!34017                                                                                    ;L1434<104
 27310|  br label %871                                                                                                         ;L120
 27311| 
 27312| 871: ; preds = %898, %889, %866, %849, %840, %786
 27313|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %26, i64 32, i1 false)                                                   ;L0
 27315|  ret void                                                                                                              ;L120
 27316| 
 27317| 872: ; preds = %99, %50
 27318|  %873 = gep %28, i64 32                                                                                                ;L45
 27319|  %874 = load ptr, ptr %873, , !!8, !!8                                                                                 ;L45
 27320|  %875 = icmp eq i64 %34, 0                                                                                             ;L45
 27321|  %876 = invoke { i64, i64 } @gc::simulation7map_defNtB2_6MapDef8camp_pos(ptr %874, i8 4, i1 zeroext %875)
 27322|  to label %882 unwind label %37                                                                                        ;L45
 27323| 
 27324| 877: ; preds = %99
 27325|  %878 = gep %28, i64 32                                                                                                ;L44
 27326|  %879 = load ptr, ptr %878, , !!8, !!8                                                                                 ;L44
 27327|  %880 = icmp eq i64 %34, 0                                                                                             ;L44
 27328|  %881 = invoke { i64, i64 } @gc::simulation7map_defNtB2_6MapDef8camp_pos(ptr %879, i8 5, i1 zeroext %880)
 27329|  to label %882 unwind label %37                                                                                        ;L44
 27330| 
 27331| 882: ; preds = %877, %872
 27332|  %883 = phi { i64, i64 } [ %881, %877 ], [ %876, %872 ]
 27333|  %884 = extractvalue { i64, i64 } %883, 0                                                                              ;L0
 27334|  %885 = extractvalue { i64, i64 } %883, 1                                                                              ;L0
 27335|     ;; camp_pos[0..+8] = i64 %884
 27336|     ;; camp_pos[8..+8] = i64 %885
 27339|  invoke void @ai::small_action6aroundNtB5_25SmallActionAroundPosition3new(ptr sret([184 x i8]) %22, ptr %3, ptr %5, i64 %884, i64 %885, i64 5)
 27340|  to label %886 unwind label %37                                                                                        ;L47
 27341| 
 27342| 886: ; preds = %882
 27343|  call void @llvm.memcpy.p0.p0.i64(ptr %23, ptr %22, i64 184, i1 false)                                                 ;L47
 27346|     ;; self = ptr %26
 27347|     ;; self = ptr %26
 27348|     ;; value = ptr %23
 27349|     ;; src = ptr %23
 27350|     ;; additional = i64 1
 27351|     ;; needed_extra_cap = i64 1
 27352|     ;; needed_extra_cap = i64 1
 27353|     ;; strategy = i8 1
 27354|     ;; self = ptr %26
 27355|     ;; self = ptr %26
 27356|     ;; self = ptr %26
 27357|     ;; self = ptr %26
 27358|     ;; used_cap = i64 0
 27359|     ;; used_cap = i64 0
 27360|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %26, i64 0, i64 1, i1 zeroext true)
 27361|  to label %889 unwind label %887, !!34060                                                                              ;L619<430<738<1429<47
 27362| 
 27363| 887: ; preds = %886
 27364|  %888 = cleanuppad within none []
 27365|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %23) #30 [ "funclet"(token %888) ], !!34041 ;L1436<47
 27366|  cleanupret from %888 unwind label %37
 27367| 
 27368| 889: ; preds = %886
 27369|  %890 = load ptr, ptr %26, , !!34060                                                                                   ;L138<1432<47
 27370|  %891 = load i64, ptr %32, , !!34060                                                                                   ;L1432<47
 27371|     ;; self = ptr %26
 27372|     ;; self = ptr %890
 27373|     ;; count = i64 %891
 27374|  %892 = gepS %890, i64 %891                                                                                            ;L961<1432<47
 27375|     ;; end = ptr %892
 27376|     ;; dst = ptr %892
 27377|  call void @llvm.memcpy.p0.p0.i64(ptr %892, ptr %23, i64 184, i1 false), !!34041                                       ;L1933<1433<47
 27378|  %893 = add i64 %891, 1                                                                                                ;L1434<47
 27379|  store i64 %893, ptr %32, , !!34060                                                                                    ;L1434<47
 27381|  br label %871                                                                                                         ;L1
 27382| 
 27383| 894: ; preds = %96
 27384|  call void @llvm.memcpy.p0.p0.i64(ptr %25, ptr %24, i64 136, i1 false)                                                 ;L37
 27385|  %895 = gep %25, i64 177                                                                                               ;L37
 27386|  store i8 4, ptr %895,                                                                                                 ;L37
 27389|     ;; self = ptr %26
 27390|     ;; self = ptr %26
 27391|     ;; value = ptr %25
 27392|     ;; src = ptr %25
 27393|     ;; additional = i64 1
 27394|     ;; needed_extra_cap = i64 1
 27395|     ;; needed_extra_cap = i64 1
 27396|     ;; strategy = i8 1
 27397|     ;; self = ptr %26
 27398|     ;; self = ptr %26
 27399|     ;; self = ptr %26
 27400|     ;; self = ptr %26
 27401|     ;; used_cap = i64 0
 27402|     ;; used_cap = i64 0
 27403|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %26, i64 0, i64 1, i1 zeroext true)
 27404|  to label %898 unwind label %896, !!34094                                                                              ;L619<430<738<1429<37
 27405| 
 27406| 896: ; preds = %894
 27407|  %897 = cleanuppad within none []
 27408|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %25) #30 [ "funclet"(token %897) ], !!34075 ;L1436<37
 27409|  cleanupret from %897 unwind label %37
 27410| 
 27411| 898: ; preds = %894
 27412|  %899 = load ptr, ptr %26, , !!34094                                                                                   ;L138<1432<37
 27413|  %900 = load i64, ptr %32, , !!34094                                                                                   ;L1432<37
 27414|     ;; self = ptr %26
 27415|     ;; self = ptr %899
 27416|     ;; count = i64 %900
 27417|  %901 = gepS %899, i64 %900                                                                                            ;L961<1432<37
 27418|     ;; end = ptr %901
 27419|     ;; dst = ptr %901
 27420|  call void @llvm.memcpy.p0.p0.i64(ptr %901, ptr %25, i64 184, i1 false), !!34075                                       ;L1933<1433<37
 27421|  %902 = add i64 %900, 1                                                                                                ;L1434<37
 27422|  store i64 %902, ptr %32, , !!34094                                                                                    ;L1434<37
 27424|  br label %871                                                                                                         ;L1
 27425| }
