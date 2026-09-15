 18966| define void @ai::plan_legacy8sub_plan12attack_nexusNtB2_18AttackNexusSubPlan17action_candidates(ptr sret([32 x i8]) %0, ptr readnone %1, i64 %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr readnone %7) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 18967|  %9 = alloca [184 x i8],
 18968|  %10 = alloca [8 x i8],
 18969|  %11 = alloca [144 x i8],
 18970|  %12 = alloca [144 x i8],
 18971|  %13 = alloca [136 x i8],
 18972|  %14 = alloca [8 x i8],
 18973|  %15 = alloca [8 x i8],
 18974|  %16 = alloca [24 x i8],
 18975|  %17 = alloca [184 x i8],
 18976|  %18 = alloca [24 x i8],
 18977|  %19 = alloca [184 x i8],
 18978|  %20 = alloca [24 x i8],
 18979|  %21 = alloca [184 x i8],
 18980|  %22 = alloca [64 x i8],
 18981|  %23 = alloca [32 x i8],
 18982|  %24 = alloca [56 x i8],
 18983|  %25 = alloca [136 x i8],
 18984|  %26 = alloca [184 x i8],
 18985|  %27 = alloca [32 x i8],
 18986|  %28 = alloca [32 x i8],
 18987|  %29 = alloca [184 x i8],
 18988|  %30 = alloca [32 x i8],
 18989|  %31 = alloca [32 x i8],
 18990|  %32 = alloca [136 x i8],
 18991|  %33 = alloca [184 x i8],
 18992|  %34 = alloca [136 x i8],
 18993|  %35 = alloca [184 x i8],
 18994|  %36 = alloca [56 x i8],
 18999|  %37 = alloca [32 x i8],
 19003|     ;; version = i64 %2
 19004|     ;; self = ptr %1
 19005|     ;; rnd = ptr %3
 19006|     ;; player = ptr %4
 19007|     ;; data = ptr %5
 19008|     ;; parameter = ptr %6
 19009|     ;; debug = ptr %7
 19010|     ;; res = ptr %37
 19011|     ;; position_score = ptr %36
 19014|  %38 = gep %5, i64 8                                                                                                   ;L119
 19015|  %39 = load ptr, ptr %38, , !!8, !!8                                                                                   ;L119
 19016|  %40 = load ptr, ptr %39, , !!8, !!8                                                                                   ;L119
 19017|     ;; bump = ptr %40
 19018|  store ptr inttoptr (i64 8 to ptr), ptr %37,                                                                           ;L547<119
 19019|  %41 = gep %37, i64 8                                                                                                  ;L547<119
 19020|  store ptr %40, ptr %41,                                                                                               ;L547<119
 19021|  %42 = gep %37, i64 16                                                                                                 ;L547<119
 19022|  %43 = gep %37, i64 24                                                                                                 ;L547<119
 19023|  %44 = gep %4, i64 2352                                                                                                ;L122
 19024|  call void @llvm.memset.p0.i64(ptr %42, i8 0, i64 16, i1 false)                                                        ;L547<119
 19025|  %45 = load i64, ptr %44, , !!8                                                                                        ;L122
 19026|  %46 = icmp ult i64 %45, 2                                                                                             ;L122
 19027|  br i1 %46, label %51, label %47                                                                                       ;L122
 19028| 
 19029| 47: ; preds = %8
 19030|  invoke void @core::panicking18panic_bounds_check(i64 %45, i64 2, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.105) #35
 19031|  to label %50 unwind label %48                                                                                         ;L122
 19032| 
 19033| 48: ; preds = %777, %763, %762, %760, %730, %658, %655, %614, %590, %548, %541, %540, %537, %252, %250, %219, %186, %179, %174, %172, %163, %158, %142, %140, %130, %114, %113, %111, %101, %89, %77, %70, %47
 19034|  %49 = cleanuppad within none []
 19035|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %37) #34 [ "funclet"(token %49) ] ;L156
 19036|  cleanupret from %49 unwind to caller                                                                                  ;L118
 19037| 
 19038| 50: ; preds = %113, %47
 19039|  unreachable
 19040| 
 19041| 51: ; preds = %8
 19042|     ;; self = ptr %4
 19043|  %52 = gep %4, i64 2496                                                                                                ;L581<122
 19044|  %53 = load i32, ptr %52, , !!8                                                                                        ;L581<122
 19045|  %54 = zext nneg i32 %53 to i64                                                                                        ;L581<122
 19046|  %55 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L122
 19047|     ;; self = ptr %55
 19048|  %56 = gep %55, i64 480                                                                                                ;L122
 19049|  %57 = getelementptr [5 x ptr], ptr %56, i64 %45                                                                       ;L122
 19050|  %58 = getelementptr ptr, ptr %57, i64 %54                                                                             ;L122
 19051|  %59 = load ptr, ptr %58, , !!8                                                                                        ;L122
 19052|     ;; self = ptr %59
 19053|  %60 = icmp eq ptr %59, null                                                                                           ;L1011<122
 19054|  br i1 %60, label %113, label %61                                                                                      ;L1011<122
 19055| 
 19056| 61: ; preds = %51
 19057|     ;; champ = ptr %59
 19058|     ;; team = !DIArgList(i64 1, i64 %45)
 19059|  %62 = sub nuw nsw i64 1, %45                                                                                          ;L125
 19060|     ;; team = i64 %62
 19061|  %63 = getelementptr [5 x ptr], ptr %56, i64 %62                                                                       ;L1905<125
 19062|     ;; self = ptr undef
 19063|     ;; self = ptr undef
 19064|     ;; f[0..+8] = ptr undef
 19065|     ;; f[8..+8] = ptr %4
 19066|     ;; f[16..+8] = ptr %5
 19067|     ;; f[24..+8] = ptr %59
 19068|     ;; fold[0..+8] = ptr undef
 19069|     ;; fold[8..+8] = ptr %4
 19070|     ;; fold[16..+8] = ptr %5
 19071|     ;; fold[24..+8] = ptr %59
 19074|     ;; f[8..+8] = ptr undef
 19075|     ;; f[16..+8] = ptr %4
 19076|     ;; f[24..+8] = ptr %5
 19077|     ;; f[32..+8] = ptr %59
 19078|     ;; self = ptr undef
 19081|     ;; self = ptr undef
 19082|     ;; count = i64 1
 19083|     ;; ptr = ptr %63
 19084|     ;; self = ptr %63
 19085|     ;; end_or_len = ptr %63
 19088|  br label %64                                                                                                          ;L180<2493<138<2897<125
 19089| 
 19090| 64: ; preds = %81, %61
 19091|  %65 = phi i64 [ 0, %61 ], [ %67, %81 ]
 19092|  %66 = gep %63, i64 %65                                                                                                ;L656<185<2493<138<2897<125
 19093|     ;; ptr = ptr %66
 19094|  %67 = add nuw nsw i64 %65, 8                                                                                          ;L656<185<2493<138<2897<125
 19095|     ;; x = ptr %66
 19096|  %68 = load ptr, ptr %66, , !!32960, !!8                                                                               ;L2494<138<2897<125
 19097|     ;; f = ptr undef
 19101|  %69 = icmp eq ptr %68, null                                                                                           ;L49<2494<138<2897<125
 19102|  br i1 %69, label %81, label %70                                                                                       ;L49<2494<138<2897<125
 19103| 
 19104| 70: ; preds = %64
 19105|     ;; x = ptr %68
 19108|     ;; x = ptr %68
 19113|     ;; c = ptr %68
 19114|     ;; self = ptr %68
 19115|     ;; self = ptr %68
 19116|     ;; self = ptr %68
 19117|     ;; self = ptr %68
 19118|     ;; self = ptr %68
 19119|  %71 = invoke zeroext i1 @ai::utils26nontarget_windup_perceived(i64 %2, ptr %4, ptr %5, ptr %68)
 19120|  to label %72 unwind label %48                                                                                         ;L126<2893<50<2494<138<2897<125
 19121| 
 19122| 72: ; preds = %70
 19123|  %73 = gep %68, i64 104
 19124|  %74 = load i64, ptr %73, , !!33018
 19125|  %75 = icmp eq i64 %74, 13
 19126|  %76 = select i1 %71, i1 %75, i1 false                                                                                 ;L126<2893<50<2494<138<2897<125
 19127|  br i1 %76, label %83, label %81                                                                                       ;L126<2893<50<2494<138<2897<125
 19128| 
 19129| 77: ; preds = %103, %103, %93, %93, %91
 19130|  %78 = phi ptr [ %98, %93 ], [ %92, %91 ], [ %98, %93 ], [ %108, %103 ], [ %108, %103 ]
 19131|  %79 = invoke zeroext i1 @gc::simulation6effectNtB2_6Effect11is_in_range(ptr %78, ptr %68, ptr %59)
 19132|  to label %80 unwind label %48                                                                                         ;L0<2893<50<2494<138<2897<125
 19133| 
 19134| 80: ; preds = %77
 19135|  br i1 %79, label %114, label %81                                                                                      ;L2494<138<2897<125
 19136| 
 19137| 81: ; preds = %103, %93, %86, %83, %80, %72, %64
 19138|     ;; self = ptr undef
 19139|     ;; count = i64 1
 19140|     ;; ptr = !DIArgList(ptr %63, i64 %67)
 19141|     ;; self = !DIArgList(ptr %63, i64 %67)
 19142|     ;; end_or_len = ptr %63
 19145|  %82 = icmp eq i64 %67, 40                                                                                             ;L1714<180<2493<138<2897<125
 19146|  br i1 %82, label %114, label %64                                                                                      ;L180<2493<138<2897<125
 19147| 
 19148| 83: ; preds = %72
 19149|     ;; champ = ptr %68
 19150|  %84 = gep %68, i64 112                                                                                                ;L1572<127<2893<50<2494<138<2897<125
 19151|  %85 = load i64, ptr %84, , !!33018, !!8                                                                               ;L1572<127<2893<50<2494<138<2897<125
 19152|  switch i64 %85, label %81 [
 19153|  i64 4, label %86
 19154|  i64 5, label %93
 19155|  i64 6, label %103
 19156|  ]                                                                                                                     ;L127<2893<50<2494<138<2897<125
 19157| 
 19158| 86: ; preds = %83
 19159|     ;; self = ptr %68
 19160|  %87 = gep %68, i64 1272                                                                                               ;L742<127<2893<50<2494<138<2897<125
 19161|  %88 = load i32, ptr %87, , !!33018, !!8                                                                               ;L742<127<2893<50<2494<138<2897<125
 19162|  switch i32 %88, label %81 [
 19163|  i32 -1, label %89
 19164|  i32 1, label %91
 19165|  i32 2, label %91
 19166|  ]                                                                                                                     ;L742<127<2893<50<2494<138<2897<125
 19167| 
 19168| 89: ; preds = %86
 19169|     ;; self = ptr null
 19170|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.35) #35
 19171|  to label %90 unwind label %48                                                                                         ;L1013<127<2893<50<2494<138<2897<125
 19172| 
 19173| 90: ; preds = %89
 19174|  unreachable                                                                                                           ;L1013<127<2893<50<2494<138<2897<125
 19175| 
 19176| 91: ; preds = %86, %86
 19177|  %92 = gep %68, i64 1224                                                                                               ;L742<127<2893<50<2494<138<2897<125
 19178|     ;; self = ptr %68
 19179|     ;; self = ptr %92
 19180|  br label %77                                                                                                          ;L127<2893<50<2494<138<2897<125
 19181| 
 19182| 93: ; preds = %83
 19183|  %94 = gep %68, i64 1480                                                                                               ;L1693<129<2893<50<2494<138<2897<125
 19184|  %95 = load i64, ptr %94, , !!33018, !!8                                                                               ;L1693<129<2893<50<2494<138<2897<125
 19185|  %96 = icmp ugt i64 %95, 2                                                                                             ;L1693<129<2893<50<2494<138<2897<125
 19186|  %97 = gep %68, i64 1280                                                                                               ;L1693<129<2893<50<2494<138<2897<125
 19187|  %98 = select i1 %96, ptr %97, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.22                                           ;L1693<129<2893<50<2494<138<2897<125
 19188|     ;; self = ptr %98
 19189|  %99 = gep %98, i64 48                                                                                                 ;L742<129<2893<50<2494<138<2897<125
 19190|  %100 = load i32, ptr %99, , !!33018, !!8                                                                              ;L742<129<2893<50<2494<138<2897<125
 19191|  switch i32 %100, label %81 [
 19192|  i32 -1, label %101
 19193|  i32 1, label %77
 19194|  i32 2, label %77
 19195|  ]                                                                                                                     ;L742<129<2893<50<2494<138<2897<125
 19196| 
 19197| 101: ; preds = %93
 19198|     ;; self = ptr null
 19199|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.36) #35
 19200|  to label %102 unwind label %48                                                                                        ;L1013<129<2893<50<2494<138<2897<125
 19201| 
 19202| 102: ; preds = %101
 19203|  unreachable                                                                                                           ;L1013<129<2893<50<2494<138<2897<125
 19204| 
 19205| 103: ; preds = %83
 19206|  %104 = gep %68, i64 1480                                                                                              ;L1701<131<2893<50<2494<138<2897<125
 19207|  %105 = load i64, ptr %104, , !!33018, !!8                                                                             ;L1701<131<2893<50<2494<138<2897<125
 19208|  %106 = icmp ugt i64 %105, 4                                                                                           ;L1701<131<2893<50<2494<138<2897<125
 19209|  %107 = gep %68, i64 1336                                                                                              ;L1701<131<2893<50<2494<138<2897<125
 19210|  %108 = select i1 %106, ptr %107, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.22                                        ;L1701<131<2893<50<2494<138<2897<125
 19211|     ;; self = ptr %108
 19212|  %109 = gep %108, i64 48                                                                                               ;L742<131<2893<50<2494<138<2897<125
 19213|  %110 = load i32, ptr %109, , !!33018, !!8                                                                             ;L742<131<2893<50<2494<138<2897<125
 19214|  switch i32 %110, label %81 [
 19215|  i32 -1, label %111
 19216|  i32 1, label %77
 19217|  i32 2, label %77
 19218|  ]                                                                                                                     ;L742<131<2893<50<2494<138<2897<125
 19219| 
 19220| 111: ; preds = %103
 19221|     ;; self = ptr null
 19222|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.37) #35
 19223|  to label %112 unwind label %48                                                                                        ;L1013<131<2893<50<2494<138<2897<125
 19224| 
 19225| 112: ; preds = %111
 19226|  unreachable                                                                                                           ;L1013<131<2893<50<2494<138<2897<125
 19227| 
 19228| 113: ; preds = %51
 19229|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.106) #35
 19230|  to label %50 unwind label %48                                                                                         ;L1013<122
 19231| 
 19232| 114: ; preds = %81, %80
 19233|  %115 = phi i1 [ false, %81 ], [ true, %80 ]                                                                           ;L1714<180<2493<138<2897<125
 19234|     ;; has_non_target_action_range = i1 %115
 19236|  %116 = gep %6, i64 2544                                                                                               ;L138
 19237|  %117 = gep %59, i64 1632                                                                                              ;L139
 19238|  %118 = load i64, ptr %117, , !!8                                                                                      ;L139
 19239|  %119 = gep %59, i64 1640                                                                                              ;L139
 19240|  %120 = load i64, ptr %119, , !!8                                                                                      ;L139
 19241|  invoke void @ai::position_eval26position_score_at_position(ptr sret([56 x i8]) %36, i64 %2, ptr %4, ptr %5, ptr %116, i64 %118, i64 %120, i8 2)
 19242|  to label %121 unwind label %48                                                                                        ;L138
 19243| 
 19244| 121: ; preds = %114
 19245|  %122 = gep %36, i64 48                                                                                                ;L140
 19246|  %123 = load i8, ptr %122, , !!8                                                                                       ;L140
 19247|  %124 = trunc nuw i8 %123 to i1                                                                                        ;L140
 19248|  %125 = gep %36, i64 49                                                                                                ;L140
 19249|  %126 = load i8, ptr %125,                                                                                             ;L140
 19250|  %127 = trunc nuw i8 %126 to i1                                                                                        ;L140
 19251|     ;; on_trajectory = i1 %127
 19252|  %128 = or i1 %115, %127
 19253|  %129 = select i1 %124, i1 true, i1 %128                                                                               ;L140
 19254|  br i1 %129, label %130, label %131                                                                                    ;L140
 19255| 
 19256| 130: ; preds = %121
 19259|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %34, ptr %5, ptr %4, i64 5, i1 zeroext true)
 19260|  to label %769 unwind label %48                                                                                        ;L142
 19261| 
 19262| 131: ; preds = %121
 19265|     ;; version = i64 %2
 19266|     ;; rnd = ptr %3
 19267|     ;; player = ptr %4
 19268|     ;; data = ptr %5
 19269|     ;; res = ptr %27
 19271|  %132 = load ptr, ptr %39, , !!33099, !!8, !!8                                                                         ;L12<147
 19272|     ;; bump = ptr %132
 19273|  store ptr inttoptr (i64 8 to ptr), ptr %27, , !!33099                                                                 ;L547<12<147
 19274|  %133 = gep %27, i64 8                                                                                                 ;L547<12<147
 19275|  store ptr %132, ptr %133, , !!33099                                                                                   ;L547<12<147
 19276|  %134 = gep %27, i64 16                                                                                                ;L547<12<147
 19277|  %135 = gep %27, i64 24                                                                                                ;L547<12<147
 19278|  call void @llvm.memset.p0.i64(ptr %134, i8 0, i64 16, i1 false), !!33099                                              ;L547<12<147
 19279|  %136 = gep %55, i64 368                                                                                               ;L14<147
 19280|  %137 = getelementptr ptr, ptr %136, i64 %62                                                                           ;L14<147
 19281|  %138 = load ptr, ptr %137, , !!33099, !!8                                                                             ;L14<147
 19282|     ;; self = ptr %138
 19283|  %139 = icmp eq ptr %138, null                                                                                         ;L1011<14<147
 19284|  br i1 %139, label %147, label %144                                                                                    ;L1011<14<147
 19285| 
 19286| 140: ; preds = %156, %147, %144
 19287|  %141 = cleanuppad within none []
 19288|  invoke fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %27) #34 [ "funclet"(token %141) ]
 19289|  to label %142 unwind label %48                                                                                        ;L19<147
 19290| 
 19291| 142: ; preds = %140
 19292|  cleanupret from %141 unwind label %48
 19293| 
 19294| 143: ; preds = %147
 19295|  unreachable
 19296| 
 19297| 144: ; preds = %131
 19298|     ;; nexus = ptr %138
 19301|  %145 = gep %138, i64 1472                                                                                             ;L16<147
 19302|  %146 = load i64, ptr %145, , !!33099, !!8                                                                             ;L16<147
 19303|  invoke void @ai::small_action6aroundNtB2_17SmallActionAround3new(ptr sret([136 x i8]) %25, i64 %2, ptr %3, ptr %5, ptr %4, i64 %146, i64 5)
 19304|  to label %148 unwind label %140, !!33123                                                                              ;L16<147
 19305| 
 19306| 147: ; preds = %131
 19307|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.104) #35
 19308|  to label %143 unwind label %140, !!33099                                                                              ;L1013<14<147
 19309| 
 19310| 148: ; preds = %144
 19311|  call void @llvm.memcpy.p0.p0.i64(ptr %26, ptr %25, i64 136, i1 false), !!33099                                        ;L16<147
 19312|  %149 = gep %26, i64 177                                                                                               ;L16<147
 19313|  store i8 5, ptr %149, , !!33099                                                                                       ;L16<147
 19315|     ;; self = ptr %27
 19316|     ;; self = ptr %27
 19317|     ;; value = ptr %26
 19318|     ;; src = ptr %26
 19319|     ;; additional = i64 1
 19320|     ;; needed_extra_cap = i64 1
 19321|     ;; needed_extra_cap = i64 1
 19322|     ;; strategy = i8 1
 19323|  %150 = load i64, ptr %135, , !!33140, !!8                                                                             ;L1428<16<147
 19324|     ;; self = ptr %27
 19325|  %151 = load i64, ptr %134, , !!33140, !!8                                                                             ;L149<1428<16<147
 19326|  %152 = icmp eq i64 %150, %151                                                                                         ;L1428<16<147
 19327|  br i1 %152, label %153, label %158                                                                                    ;L1428<16<147
 19328| 
 19329| 153: ; preds = %148
 19330|     ;; self = ptr %27
 19331|     ;; self = ptr %27
 19332|     ;; self = ptr %27
 19333|     ;; used_cap = i64 %150
 19334|     ;; used_cap = i64 %150
 19335|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %27, i64 %150, i64 1, i1 zeroext true)
 19336|  to label %154 unwind label %156, !!33148                                                                              ;L619<430<738<1429<16<147
 19337| 
 19338| 154: ; preds = %153
 19339|  %155 = load i64, ptr %135, , !!33140                                                                                  ;L1432<16<147
 19340|  br label %158                                                                                                         ;L619<430<738<1429<16<147
 19341| 
 19342| 156: ; preds = %153
 19343|  %157 = cleanuppad within none []
 19344|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %26) #34 [ "funclet"(token %157) ], !!33123 ;L1436<16<147
 19345|  cleanupret from %157 unwind label %140
 19346| 
 19347| 158: ; preds = %154, %148
 19348|  %159 = phi i64 [ %155, %154 ], [ %150, %148 ]                                                                         ;L1434<16<147
 19349|     ;; self = ptr %27
 19350|  %160 = load ptr, ptr %27, , !!33140, !!8, !!8                                                                         ;L138<1432<16<147
 19351|     ;; self = ptr %160
 19352|     ;; count = i64 %159
 19353|  %161 = gepS %160, i64 %159                                                                                            ;L961<1432<16<147
 19354|     ;; end = ptr %161
 19355|     ;; dst = ptr %161
 19356|  call void @llvm.memcpy.p0.p0.i64(ptr %161, ptr %26, i64 184, i1 false), !!33123                                       ;L1933<1433<16<147
 19357|  %162 = add i64 %159, 1                                                                                                ;L1434<16<147
 19360|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %37, ptr %160, i64 %162)
 19361|  to label %163 unwind label %48                                                                                        ;L147
 19362| 
 19363| 163: ; preds = %158
 19366|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway3new(ptr sret([136 x i8]) %32, ptr %5, ptr %4, i64 5)
 19367|  to label %164 unwind label %48                                                                                        ;L148
 19368| 
 19369| 164: ; preds = %163
 19370|  call void @llvm.memcpy.p0.p0.i64(ptr %33, ptr %32, i64 136, i1 false)                                                 ;L148
 19371|  %165 = gep %33, i64 177                                                                                               ;L148
 19372|  store i8 3, ptr %165,                                                                                                 ;L148
 19374|     ;; self = ptr %37
 19375|     ;; self = ptr %37
 19376|     ;; value = ptr %33
 19377|     ;; src = ptr %33
 19378|     ;; additional = i64 1
 19379|     ;; needed_extra_cap = i64 1
 19380|     ;; needed_extra_cap = i64 1
 19381|     ;; strategy = i8 1
 19382|  %166 = load i64, ptr %43, , !!33177, !!8                                                                              ;L1428<148
 19383|     ;; self = ptr %37
 19384|  %167 = load i64, ptr %42, , !!33177, !!8                                                                              ;L149<1428<148
 19385|  %168 = icmp eq i64 %166, %167                                                                                         ;L1428<148
 19386|  br i1 %168, label %169, label %174                                                                                    ;L1428<148
 19387| 
 19388| 169: ; preds = %164
 19389|     ;; self = ptr %37
 19390|     ;; self = ptr %37
 19391|     ;; self = ptr %37
 19392|     ;; used_cap = i64 %166
 19393|     ;; used_cap = i64 %166
 19394|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %37, i64 %166, i64 1, i1 zeroext true)
 19395|  to label %170 unwind label %172, !!33177                                                                              ;L619<430<738<1429<148
 19396| 
 19397| 170: ; preds = %169
 19398|  %171 = load i64, ptr %43, , !!33177                                                                                   ;L1432<148
 19399|  br label %174                                                                                                         ;L619<430<738<1429<148
 19400| 
 19401| 172: ; preds = %169
 19402|  %173 = cleanuppad within none []
 19403|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %33) #34 [ "funclet"(token %173) ] ;L1436<148
 19404|  cleanupret from %173 unwind label %48
 19405| 
 19406| 174: ; preds = %170, %164
 19407|  %175 = phi i64 [ %171, %170 ], [ %166, %164 ]                                                                         ;L1434<148
 19408|     ;; self = ptr %37
 19409|  %176 = load ptr, ptr %37, , !!33177, !!8, !!8                                                                         ;L138<1432<148
 19410|     ;; self = ptr %176
 19411|     ;; count = i64 %175
 19412|  %177 = gepS %176, i64 %175                                                                                            ;L961<1432<148
 19413|     ;; end = ptr %177
 19414|     ;; dst = ptr %177
 19415|  call void @llvm.memcpy.p0.p0.i64(ptr %177, ptr %33, i64 184, i1 false)                                                ;L1933<1433<148
 19416|  %178 = add i64 %175, 1                                                                                                ;L1434<148
 19417|  store i64 %178, ptr %43, , !!33177                                                                                    ;L1434<148
 19420|  invoke void @ai::fight_check13battle_action(ptr sret([32 x i8]) %31, i64 %2, ptr %3, ptr %4, ptr %5, i64 5)
 19421|  to label %179 unwind label %48                                                                                        ;L149
 19422| 
 19423| 179: ; preds = %174
 19424|  %180 = load ptr, ptr %31, , !!8, !!8                                                                                  ;L149
 19425|  %181 = gep %31, i64 24                                                                                                ;L149
 19426|  %182 = load i64, ptr %181, , !!8                                                                                      ;L149
 19427|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %37, ptr %180, i64 %182)
 19428|  to label %183 unwind label %48                                                                                        ;L149
 19429| 
 19430| 183: ; preds = %179
 19432|     ;; near_enemy_minions[0..+56] = ptr %24
 19436|     ;; data = ptr %5
 19437|     ;; res = ptr %23
 19438|     ;; iter = ptr %22
 19439|  %184 = load ptr, ptr %58, , !!33265, !!8                                                                              ;L22<150
 19440|     ;; self = ptr %184
 19441|  %185 = icmp eq ptr %184, null                                                                                         ;L1011<22<150
 19442|  br i1 %185, label %219, label %186                                                                                    ;L1011<22<150
 19443| 
 19444| 186: ; preds = %183
 19445|     ;; champ = ptr %184
 19446|     ;; entity = ptr %184
 19447|     ;; caster = ptr %184
 19448|     ;; self = ptr %184
 19449|     ;; other = ptr %184
 19450|     ;; caster = ptr %184
 19451|     ;; self = ptr %184
 19452|     ;; other = ptr %184
 19453|     ;; self = ptr %184
 19454|     ;; caster = ptr %184
 19455|     ;; self = ptr %184
 19456|     ;; other = ptr %184
 19458|  invoke void @gc::simulationNtB5_21AbstractGameWithCache12iter_minions(ptr sret([56 x i8]) %24, ptr %55, i64 %62)
 19459|  to label %187 unwind label %48                                                                                        ;L23<150
 19460| 
 19461| 187: ; preds = %186
 19462|     ;; near_enemy_minions[56..+8] = ptr %184
 19464|  %188 = load ptr, ptr %39, , !!33265, !!8, !!8                                                                         ;L25<150
 19465|     ;; bump = ptr %188
 19466|  store ptr inttoptr (i64 8 to ptr), ptr %23, , !!33329                                                                 ;L547<25<150
 19467|  %189 = gep %23, i64 8                                                                                                 ;L547<25<150
 19468|  store ptr %188, ptr %189, , !!33329                                                                                   ;L547<25<150
 19469|  %190 = gep %23, i64 16                                                                                                ;L547<25<150
 19470|  %191 = gep %23, i64 24                                                                                                ;L547<25<150
 19471|  %192 = gep %184, i64 1600                                                                                             ;L26<150
 19472|  call void @llvm.memset.p0.i64(ptr %190, i8 0, i64 16, i1 false), !!33329                                              ;L547<25<150
 19473|  %193 = load i64, ptr %192, , !!33265, !!8                                                                             ;L26<150
 19474|     ;; move_speed = i64 %193
 19476|  call void @llvm.memcpy.p0.p0.i64(ptr %22, ptr %24, i64 56, i1 false), !!33329                                         ;L28<150
 19477|  %194 = gep %22, i64 56                                                                                                ;L28<150
 19478|  store ptr %184, ptr %194, , !!33329                                                                                   ;L28<150
 19479|  %195 = gep %22, i64 8
 19480|  %196 = gep %22, i64 24
 19481|  %197 = gep %22, i64 40
 19482|  %198 = gep %184, i64 8
 19483|  %199 = gep %184, i64 1216
 19484|  %200 = gep %184, i64 1168
 19485|  %201 = gep %184, i64 1184
 19486|  %202 = gep %184, i64 1192
 19487|  %203 = gep %184, i64 1480
 19488|  %204 = gep %184, i64 1080
 19489|  %205 = gep %184, i64 1136
 19490|  %206 = gep %184, i64 1664
 19491|  %207 = gep %184, i64 1632
 19492|  %208 = gep %184, i64 1640
 19493|  %209 = mul i64 %193, 30
 19494|  %210 = gep %21, i64 177
 19495|  %211 = gep %184, i64 1224
 19496|  %212 = gep %184, i64 1272
 19497|  %213 = gep %184, i64 1264
 19498|  %214 = gep %184, i64 1240
 19499|  %215 = gep %184, i64 1248
 19500|  %216 = gep %19, i64 177
 19501|  %217 = gep %184, i64 1280
 19502|  %218 = gep %17, i64 177
 19503|  br label %221                                                                                                         ;L28<150
 19504| 
 19505| 219: ; preds = %183
 19506|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.107) #35
 19507|  to label %220 unwind label %48                                                                                        ;L1013<22<150
 19508| 
 19509| 220: ; preds = %219
 19510|  unreachable                                                                                                           ;L1013<22<150
 19511| 
 19512| 221: ; preds = %452, %187
 19513|     ;; self = ptr %22
 19516|  store ptr %194, ptr %15, , !!33356
 19517|     ;; self = ptr %22
 19518|     ;; predicate = ptr %15
 19520|     ;; opt = ptr %22
 19521|     ;; self = ptr %22
 19522|     ;; f = ptr %15
 19523|  %222 = load i64, ptr %22, , !!33408, !!8                                                                              ;L764<332<169<98<28<150
 19524|  %223 = trunc nuw i64 %222 to i1                                                                                       ;L764<332<169<98<28<150
 19525|  br i1 %223, label %224, label %243                                                                                    ;L764<332<169<98<28<150
 19526| 
 19527| 224: ; preds = %221
 19529|     ;; predicate = ptr %15
 19530|     ;; a = ptr %195
 19533|  store ptr %15, ptr %14, , !!33428
 19534|     ;; self = ptr %195
 19535|     ;; predicate = ptr %14
 19536|     ;; opt = ptr %195
 19537|     ;; self = ptr %195
 19538|     ;; f = ptr %14
 19539|  %225 = load ptr, ptr %195, , !!33472, !!8                                                                             ;L764<332<169<169<332<169<98<28<150
 19540|  %226 = icmp eq ptr %225, null                                                                                         ;L764<332<169<169<332<169<98<28<150
 19541|  br i1 %226, label %233, label %227                                                                                    ;L764<332<169<169<332<169<98<28<150
 19542| 
 19543| 227: ; preds = %224
 19544|     ;; predicate = ptr %14
 19545|     ;; a = ptr %195
 19546|     ;; self = ptr %195
 19547|     ;; predicate = ptr %14
 19548|  %228 = invoke ptr @core::iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2p_4find5checkB1s_QQQNCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan12attack_nexusNtB3F_18AttackNexusSubPlan20attack_minion_action0E0INtNtNtBb_3ops12control_flow11ControlFlowB1s_EEB3L_(ptr %195, ptr %14)
 19549|  to label %229 unwind label %250, !!33265                                                                              ;L2971<169<332<169<169<332<169<98<28<150
 19550| 
 19551| 229: ; preds = %227
 19552|     ;; x = ptr %228
 19555|  %230 = icmp eq ptr %228, null                                                                                         ;L633<682<333<169<169<332<169<98<28<150
 19556|  br i1 %230, label %232, label %231                                                                                    ;L333<169<169<332<169<98<28<150
 19557| 
 19558| 231: ; preds = %229
 19560|     ;; x = ptr %228
 19563|  br label %253                                                                                                         ;L333<169<98<28<150
 19564| 
 19565| 232: ; preds = %229
 19566|  store ptr null, ptr %195, , !!33472                                                                                   ;L334<169<169<332<169<98<28<150
 19567|  br label %233                                                                                                         ;L333<169<169<332<169<98<28<150
 19568| 
 19569| 233: ; preds = %232, %224
 19570|     ;; self = ptr null
 19571|     ;; f[0..+8] = ptr %196
 19576|     ;; self = ptr %196
 19577|  %234 = load ptr, ptr %196, , !!33562, !!8                                                                             ;L764<170<1653<170<169<332<169<98<28<150
 19578|  %235 = icmp eq ptr %234, null                                                                                         ;L764<170<1653<170<169<332<169<98<28<150
 19579|  br i1 %235, label %236, label %237                                                                                    ;L764<170<1653<170<169<332<169<98<28<150
 19580| 
 19581| 236: ; preds = %233
 19583|     ;; x = ptr null
 19586|  br label %242                                                                                                         ;L333<169<98<28<150
 19587| 
 19588| 237: ; preds = %233
 19589|  %238 = load ptr, ptr %14, , !!33428, !!8, !!8                                                                         ;L170<169<332<169<98<28<150
 19590|     ;; f[8..+8] = ptr %238
 19591|     ;; self = ptr %196
 19592|     ;; predicate = ptr %238
 19593|  %239 = invoke ptr @core::iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2p_4find5checkB1s_QQNCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan12attack_nexusNtB3E_18AttackNexusSubPlan20attack_minion_action0E0INtNtNtBb_3ops12control_flow11ControlFlowB1s_EEB3K_(ptr %196, ptr %238)
 19594|  to label %240 unwind label %250, !!33265                                                                              ;L2971<170<1653<170<169<332<169<98<28<150
 19595| 
 19596| 240: ; preds = %237
 19598|     ;; x = ptr %239
 19601|  %241 = icmp eq ptr %239, null                                                                                         ;L633<682<333<169<98<28<150
 19602|  br i1 %241, label %242, label %253                                                                                    ;L333<169<98<28<150
 19603| 
 19604| 242: ; preds = %240, %236
 19605|  store i64 0, ptr %22, , !!33408                                                                                       ;L334<169<98<28<150
 19606|  br label %243                                                                                                         ;L333<169<98<28<150
 19607| 
 19608| 243: ; preds = %242, %221
 19609|     ;; self = ptr null
 19610|     ;; f[0..+8] = ptr %197
 19615|     ;; self = ptr %197
 19616|  %244 = load ptr, ptr %197, , !!33621, !!8                                                                             ;L764<170<1653<170<98<28<150
 19617|  %245 = icmp eq ptr %244, null                                                                                         ;L764<170<1653<170<98<28<150
 19618|  br i1 %245, label %246, label %247                                                                                    ;L764<170<1653<170<98<28<150
 19619| 
 19620| 246: ; preds = %243
 19622|  br label %537                                                                                                         ;L28<150
 19623| 
 19624| 247: ; preds = %243
 19625|  %248 = load ptr, ptr %15, , !!33356, !!8, !!8                                                                         ;L170<98<28<150
 19626|     ;; f[8..+8] = ptr %248
 19627|     ;; self = ptr %197
 19628|     ;; predicate = ptr %248
 19629|  %249 = invoke ptr @core::iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2p_4find5checkB1s_QNCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan12attack_nexusNtB3D_18AttackNexusSubPlan20attack_minion_action0E0INtNtNtBb_3ops12control_flow11ControlFlowB1s_EEB3J_(ptr %197, ptr %248)
 19630|  to label %255 unwind label %250, !!33265                                                                              ;L2971<170<1653<170<98<28<150
 19631| 
 19632| 250: ; preds = %530, %520, %458, %454, %450, %443, %433, %372, %369, %359, %352, %342, %286, %280, %271, %269, %247, %237, %227
 19633|  %251 = cleanuppad within none []
 19634|  invoke fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %23) #34 [ "funclet"(token %251) ]
 19635|  to label %252 unwind label %48                                                                                        ;L74<150
 19636| 
 19637| 252: ; preds = %250
 19638|  cleanupret from %251 unwind label %48
 19639| 
 19640| 253: ; preds = %240, %231
 19641|  %254 = phi ptr [ %239, %240 ], [ %228, %231 ]
 19643|  br label %257                                                                                                         ;L28<150
 19644| 
 19645| 255: ; preds = %247
 19647|  %256 = icmp eq ptr %249, null                                                                                         ;L28<150
 19648|  br i1 %256, label %537, label %257                                                                                    ;L28<150
 19649| 
 19650| 257: ; preds = %255, %253
 19651|  %258 = phi ptr [ %254, %253 ], [ %249, %255 ]
 19652|     ;; target = ptr %258
 19653|     ;; self = ptr %258
 19654|     ;; self = ptr %258
 19655|     ;; self = ptr %258
 19656|     ;; self = ptr %258
 19657|     ;; self = ptr %258
 19658|     ;; self = ptr %258
 19659|     ;; self = ptr %258
 19660|     ;; self = ptr %184
 19661|  %259 = load i64, ptr %184, , !!33265, !!8                                                                             ;L1136<1482<29<150
 19662|  %260 = trunc nuw i64 %259 to i1                                                                                       ;L1136<1482<29<150
 19663|  br i1 %260, label %271, label %261                                                                                    ;L1136<1482<29<150
 19664| 
 19665| 261: ; preds = %257
 19666|     ;; team = ptr %184
 19667|  %262 = load i64, ptr %198, , !!33265, !!8                                                                             ;L1137<1482<29<150
 19668|     ;; team = i64 %262
 19669|  %263 = icmp ult i64 %262, 2                                                                                           ;L1483<29<150
 19670|  br i1 %263, label %264, label %269                                                                                    ;L1483<29<150
 19671| 
 19672| 264: ; preds = %261
 19674|  %265 = gep %258, i64 56                                                                                               ;L122<1483<29<150
 19675|  %266 = gepS %265, i64 %262                                                                                            ;L122<1483<29<150
 19676|  %267 = load i64, ptr %266, , !!33265, !!8                                                                             ;L122<1483<29<150
 19677|  %268 = icmp eq i64 %267, 0                                                                                            ;L122<1483<29<150
 19678|  br i1 %268, label %271, label %452                                                                                    ;L29<150
 19679| 
 19680| 269: ; preds = %261
 19681|  invoke void @core::panicking18panic_bounds_check(i64 %262, i64 2, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.26) #35
 19682|  to label %270 unwind label %250, !!33265                                                                              ;L1483<29<150
 19683| 
 19684| 270: ; preds = %286, %269
 19685|  unreachable
 19686| 
 19687| 271: ; preds = %264, %257
 19688|  %272 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_attack(ptr %184)
 19689|  to label %273 unwind label %250, !!33265                                                                              ;L33<150
 19690| 
 19691| 273: ; preds = %271
 19692|  br i1 %272, label %277, label %274                                                                                    ;L33<150
 19693| 
 19694| 274: ; preds = %354, %315, %273
 19695|     ;; self = ptr %184
 19696|  %275 = load i32, ptr %212, , !!33265, !!8                                                                             ;L742<46<150
 19697|  %276 = icmp eq i32 %275, -1                                                                                           ;L742<46<150
 19698|  br i1 %276, label %361, label %359                                                                                    ;L742<46<150
 19699| 
 19700| 277: ; preds = %273
 19701|     ;; self = ptr %184
 19702|  %278 = load i32, ptr %199, , !!33265, !!8                                                                             ;L742<34<150
 19703|  %279 = icmp eq i32 %278, -1                                                                                           ;L742<34<150
 19704|  br i1 %279, label %286, label %280                                                                                    ;L742<34<150
 19705| 
 19706| 280: ; preds = %277
 19707|     ;; self = ptr %200
 19708|     ;; atk = ptr %200
 19709|     ;; self = ptr %200
 19710|  %281 = load i64, ptr %201, , !!33265, !!8                                                                             ;L26<36<150
 19711|  %282 = load i64, ptr %202, , !!33265, !!8                                                                             ;L26<36<150
 19712|  %283 = load i64, ptr %203, , !!33265, !!8                                                                             ;L26<36<150
 19713|  %284 = load i64, ptr %204, , !!33265, !!8                                                                             ;L26<36<150
 19714|  %285 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %200, ptr %184, ptr %258)
 19715|  to label %287 unwind label %250, !!33265                                                                              ;L36<150
 19716| 
 19717| 286: ; preds = %277
 19718|     ;; self = ptr null
 19719|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.108) #35
 19720|  to label %270 unwind label %250, !!33265                                                                              ;L1013<34<150
 19721| 
 19722| 287: ; preds = %280
 19723|  %288 = add i64 %283, -1                                                                                               ;L26<36<150
 19724|  %289 = mul i64 %288, %282                                                                                             ;L26<36<150
 19725|  %290 = load i32, ptr %205, , !!33265, !!8                                                                             ;L1511<36<150
 19726|     ;; mult = i32 %290
 19727|  %291 = icmp eq i32 %290, 0                                                                                            ;L1512<36<150
 19728|  br i1 %291, label %292, label %294                                                                                    ;L1512<36<150
 19729| 
 19730| 292: ; preds = %287
 19731|  %293 = load i64, ptr %206, , !!33265, !!8                                                                             ;L1513<36<150
 19732|  br label %300                                                                                                         ;L1512<36<150
 19733| 
 19734| 294: ; preds = %287
 19735|  %295 = sext i32 %290 to i64                                                                                           ;L1511<36<150
 19736|     ;; mult = i64 %295
 19737|  %296 = load i64, ptr %206, , !!33265, !!8                                                                             ;L1515<36<150
 19738|  %297 = add nsw i64 %295, 100                                                                                          ;L1515<36<150
 19739|  %298 = mul i64 %296, %297                                                                                             ;L1515<36<150
 19740|  %299 = udiv i64 %298, 100                                                                                             ;L1515<36<150
 19741|  br label %300                                                                                                         ;L1512<36<150
 19742| 
 19743| 300: ; preds = %294, %292
 19744|  %301 = phi i64 [ %293, %292 ], [ %299, %294 ]                                                                         ;L0<36<150
 19745|  %302 = gep %258, i64 1136                                                                                             ;L1511<36<150
 19746|  %303 = load i32, ptr %302, , !!33265, !!8                                                                             ;L1511<36<150
 19747|     ;; mult = i32 %303
 19748|  %304 = icmp eq i32 %303, 0                                                                                            ;L1512<36<150
 19749|  br i1 %304, label %305, label %308                                                                                    ;L1512<36<150
 19750| 
 19751| 305: ; preds = %300
 19752|  %306 = gep %258, i64 1664                                                                                             ;L1513<36<150
 19753|  %307 = load i64, ptr %306, , !!33265, !!8                                                                             ;L1513<36<150
 19754|  br label %315                                                                                                         ;L1512<36<150
 19755| 
 19756| 308: ; preds = %300
 19757|  %309 = sext i32 %303 to i64                                                                                           ;L1511<36<150
 19758|     ;; mult = i64 %309
 19759|  %310 = gep %258, i64 1664                                                                                             ;L1515<36<150
 19760|  %311 = load i64, ptr %310, , !!33265, !!8                                                                             ;L1515<36<150
 19761|  %312 = add nsw i64 %309, 100                                                                                          ;L1515<36<150
 19762|  %313 = mul i64 %311, %312                                                                                             ;L1515<36<150
 19763|  %314 = udiv i64 %313, 100                                                                                             ;L1515<36<150
 19764|  br label %315                                                                                                         ;L1512<36<150
 19765| 
 19766| 315: ; preds = %308, %305
 19767|  %316 = phi i64 [ %307, %305 ], [ %314, %308 ]                                                                         ;L0<36<150
 19769|  %317 = gep %258, i64 1632                                                                                             ;L2158<37<150
 19770|  %318 = load i64, ptr %317, , !!33265, !!8                                                                             ;L2158<37<150
 19771|     ;; x1 = i64 %318
 19772|     ;; self = i64 %318
 19773|  %319 = gep %258, i64 1640                                                                                             ;L2158<37<150
 19774|  %320 = load i64, ptr %319, , !!33265, !!8                                                                             ;L2158<37<150
 19775|     ;; y1 = i64 %320
 19776|     ;; self = i64 %320
 19777|  %321 = load i64, ptr %207, , !!33265, !!8                                                                             ;L2158<37<150
 19778|     ;; x2 = i64 %321
 19779|     ;; other = i64 %321
 19780|  %322 = load i64, ptr %208, , !!33265, !!8                                                                             ;L2158<37<150
 19781|     ;; y2 = i64 %322
 19782|     ;; other = i64 %322
 19783|  %323 = icmp ult i64 %318, %321                                                                                        ;L3147<7<2158<37<150
 19784|  %324 = sub nuw i64 %321, %318                                                                                         ;L3147<7<2158<37<150
 19785|  %325 = sub nuw i64 %318, %321                                                                                         ;L3147<7<2158<37<150
 19786|  %326 = select i1 %323, i64 %324, i64 %325                                                                             ;L3147<7<2158<37<150
 19787|     ;; dx = i64 %326
 19788|  %327 = icmp ult i64 %320, %322                                                                                        ;L3147<8<2158<37<150
 19789|  %328 = sub nuw i64 %322, %320                                                                                         ;L3147<8<2158<37<150
 19790|  %329 = sub nuw i64 %320, %322                                                                                         ;L3147<8<2158<37<150
 19791|  %330 = select i1 %327, i64 %328, i64 %329                                                                             ;L3147<8<2158<37<150
 19792|     ;; dy = i64 %330
 19793|  %331 = mul i64 %326, %326                                                                                             ;L9<2158<37<150
 19794|  %332 = mul i64 %330, %330                                                                                             ;L9<2158<37<150
 19795|  %333 = add i64 %332, %331                                                                                             ;L9<2158<37<150
 19796|     ;; dist_sq = i64 %333
 19797|  %334 = add i64 %281, %209                                                                                             ;L26<36<150
 19798|  %335 = add i64 %334, %284                                                                                             ;L26<36<150
 19799|  %336 = add i64 %335, %289                                                                                             ;L36<150
 19800|  %337 = add i64 %336, %285                                                                                             ;L36<150
 19801|  %338 = add i64 %337, %301                                                                                             ;L36<150
 19802|  %339 = add i64 %338, %316                                                                                             ;L40<150
 19803|     ;; max_dist = i64 %339
 19804|  %340 = mul i64 %339, %339                                                                                             ;L41<150
 19805|  %341 = icmp ugt i64 %333, %340                                                                                        ;L41<150
 19806|  br i1 %341, label %274, label %342                                                                                    ;L41<150
 19807| 
 19808| 342: ; preds = %315
 19811|  %343 = gep %258, i64 1472                                                                                             ;L42<150
 19812|  %344 = load i64, ptr %343, , !!33265, !!8                                                                             ;L42<150
 19813|  invoke void @ai::small_action4castNtB2_17SmallActionAttack3new(ptr sret([24 x i8]) %20, ptr %5, i64 %344)
 19814|  to label %345 unwind label %250, !!33265                                                                              ;L42<150
 19815| 
 19816| 345: ; preds = %342
 19817|  call void @llvm.memcpy.p0.p0.i64(ptr %21, ptr %20, i64 24, i1 false), !!33329                                         ;L42<150
 19818|  store i8 15, ptr %210, , !!33329                                                                                      ;L42<150
 19820|     ;; self = ptr %23
 19821|     ;; self = ptr %23
 19822|     ;; value = ptr %21
 19823|     ;; src = ptr %21
 19824|     ;; additional = i64 1
 19825|     ;; needed_extra_cap = i64 1
 19826|     ;; needed_extra_cap = i64 1
 19827|     ;; strategy = i8 1
 19828|  %346 = load i64, ptr %191, , !!33747, !!8                                                                             ;L1428<42<150
 19829|     ;; self = ptr %23
 19830|  %347 = load i64, ptr %190, , !!33747, !!8                                                                             ;L149<1428<42<150
 19831|  %348 = icmp eq i64 %346, %347                                                                                         ;L1428<42<150
 19832|  br i1 %348, label %349, label %354                                                                                    ;L1428<42<150
 19833| 
 19834| 349: ; preds = %345
 19835|     ;; self = ptr %23
 19836|     ;; self = ptr %23
 19837|     ;; self = ptr %23
 19838|     ;; used_cap = i64 %346
 19839|     ;; used_cap = i64 %346
 19840|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %23, i64 %346, i64 1, i1 zeroext true)
 19841|  to label %350 unwind label %352, !!33755                                                                              ;L619<430<738<1429<42<150
 19842| 
 19843| 350: ; preds = %349
 19844|  %351 = load i64, ptr %191, , !!33747                                                                                  ;L1432<42<150
 19845|  br label %354                                                                                                         ;L619<430<738<1429<42<150
 19846| 
 19847| 352: ; preds = %349
 19848|  %353 = cleanuppad within none []
 19849|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %21) #34 [ "funclet"(token %353) ], !!33265 ;L1436<42<150
 19850|  cleanupret from %353 unwind label %250
 19851| 
 19852| 354: ; preds = %350, %345
 19853|  %355 = phi i64 [ %351, %350 ], [ %346, %345 ]                                                                         ;L1434<42<150
 19854|     ;; self = ptr %23
 19855|  %356 = load ptr, ptr %23, , !!33747, !!8, !!8                                                                         ;L138<1432<42<150
 19856|     ;; self = ptr %356
 19857|     ;; count = i64 %355
 19858|  %357 = gepS %356, i64 %355                                                                                            ;L961<1432<42<150
 19859|     ;; end = ptr %357
 19860|     ;; dst = ptr %357
 19861|  call void @llvm.memcpy.p0.p0.i64(ptr %357, ptr %21, i64 184, i1 false), !!33265                                       ;L1933<1433<42<150
 19862|  %358 = add i64 %355, 1                                                                                                ;L1434<42<150
 19863|  store i64 %358, ptr %191, , !!33747                                                                                   ;L1434<42<150
 19865|  br label %274                                                                                                         ;L41<150
 19866| 
 19867| 359: ; preds = %274
 19868|     ;; skill = ptr %211
 19869|     ;; self = ptr %211
 19870|  %360 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity9can_skill(ptr %184)
 19871|  to label %368 unwind label %250, !!33265                                                                              ;L47<150
 19872| 
 19873| 361: ; preds = %445, %406, %371, %368, %274
 19874|  %362 = load i64, ptr %203, , !!33265, !!8                                                                             ;L1693<59<150
 19875|  %363 = icmp ugt i64 %362, 2                                                                                           ;L1693<59<150
 19876|  %364 = select i1 %363, ptr %217, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.22                                        ;L1693<59<150
 19877|     ;; self = ptr %364
 19878|  %365 = gep %364, i64 48                                                                                               ;L742<59<150
 19879|  %366 = load i32, ptr %365, , !!33265, !!8                                                                             ;L742<59<150
 19880|  %367 = icmp eq i32 %366, -1                                                                                           ;L742<59<150
 19881|  br i1 %367, label %452, label %450                                                                                    ;L742<59<150
 19882| 
 19883| 368: ; preds = %359
 19884|  br i1 %360, label %369, label %361                                                                                    ;L47<150
 19885| 
 19886| 369: ; preds = %368
 19887|  %370 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %213, ptr %184, ptr %258)
 19888|  to label %371 unwind label %250, !!33265                                                                              ;L47<150
 19889| 
 19890| 371: ; preds = %369
 19891|  br i1 %370, label %372, label %361                                                                                    ;L47<150
 19892| 
 19893| 372: ; preds = %371
 19894|  %373 = load i64, ptr %214, , !!33265, !!8                                                                             ;L26<48<150
 19895|  %374 = load i64, ptr %215, , !!33265, !!8                                                                             ;L26<48<150
 19896|  %375 = load i64, ptr %203, , !!33265, !!8                                                                             ;L26<48<150
 19897|  %376 = load i64, ptr %204, , !!33265, !!8                                                                             ;L26<48<150
 19898|  %377 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %211, ptr %184, ptr %258)
 19899|  to label %378 unwind label %250, !!33265                                                                              ;L48<150
 19900| 
 19901| 378: ; preds = %372
 19902|  %379 = add i64 %375, -1                                                                                               ;L26<48<150
 19903|  %380 = mul i64 %379, %374                                                                                             ;L26<48<150
 19904|  %381 = load i32, ptr %205, , !!33265, !!8                                                                             ;L1511<48<150
 19905|     ;; mult = i32 %381
 19906|  %382 = icmp eq i32 %381, 0                                                                                            ;L1512<48<150
 19907|  br i1 %382, label %383, label %385                                                                                    ;L1512<48<150
 19908| 
 19909| 383: ; preds = %378
 19910|  %384 = load i64, ptr %206, , !!33265, !!8                                                                             ;L1513<48<150
 19911|  br label %391                                                                                                         ;L1512<48<150
 19912| 
 19913| 385: ; preds = %378
 19914|  %386 = sext i32 %381 to i64                                                                                           ;L1511<48<150
 19915|     ;; mult = i64 %386
 19916|  %387 = load i64, ptr %206, , !!33265, !!8                                                                             ;L1515<48<150
 19917|  %388 = add nsw i64 %386, 100                                                                                          ;L1515<48<150
 19918|  %389 = mul i64 %387, %388                                                                                             ;L1515<48<150
 19919|  %390 = udiv i64 %389, 100                                                                                             ;L1515<48<150
 19920|  br label %391                                                                                                         ;L1512<48<150
 19921| 
 19922| 391: ; preds = %385, %383
 19923|  %392 = phi i64 [ %384, %383 ], [ %390, %385 ]                                                                         ;L0<48<150
 19924|  %393 = gep %258, i64 1136                                                                                             ;L1511<48<150
 19925|  %394 = load i32, ptr %393, , !!33265, !!8                                                                             ;L1511<48<150
 19926|     ;; mult = i32 %394
 19927|  %395 = icmp eq i32 %394, 0                                                                                            ;L1512<48<150
 19928|  br i1 %395, label %396, label %399                                                                                    ;L1512<48<150
 19929| 
 19930| 396: ; preds = %391
 19931|  %397 = gep %258, i64 1664                                                                                             ;L1513<48<150
 19932|  %398 = load i64, ptr %397, , !!33265, !!8                                                                             ;L1513<48<150
 19933|  br label %406                                                                                                         ;L1512<48<150
 19934| 
 19935| 399: ; preds = %391
 19936|  %400 = sext i32 %394 to i64                                                                                           ;L1511<48<150
 19937|     ;; mult = i64 %400
 19938|  %401 = gep %258, i64 1664                                                                                             ;L1515<48<150
 19939|  %402 = load i64, ptr %401, , !!33265, !!8                                                                             ;L1515<48<150
 19940|  %403 = add nsw i64 %400, 100                                                                                          ;L1515<48<150
 19941|  %404 = mul i64 %402, %403                                                                                             ;L1515<48<150
 19942|  %405 = udiv i64 %404, 100                                                                                             ;L1515<48<150
 19943|  br label %406                                                                                                         ;L1512<48<150
 19944| 
 19945| 406: ; preds = %399, %396
 19946|  %407 = phi i64 [ %398, %396 ], [ %405, %399 ]                                                                         ;L0<48<150
 19948|  %408 = gep %258, i64 1632                                                                                             ;L2158<49<150
 19949|  %409 = load i64, ptr %408, , !!33265, !!8                                                                             ;L2158<49<150
 19950|     ;; x1 = i64 %409
 19951|     ;; self = i64 %409
 19952|  %410 = gep %258, i64 1640                                                                                             ;L2158<49<150
 19953|  %411 = load i64, ptr %410, , !!33265, !!8                                                                             ;L2158<49<150
 19954|     ;; y1 = i64 %411
 19955|     ;; self = i64 %411
 19956|  %412 = load i64, ptr %207, , !!33265, !!8                                                                             ;L2158<49<150
 19957|     ;; x2 = i64 %412
 19958|     ;; other = i64 %412
 19959|  %413 = load i64, ptr %208, , !!33265, !!8                                                                             ;L2158<49<150
 19960|     ;; y2 = i64 %413
 19961|     ;; other = i64 %413
 19962|  %414 = icmp ult i64 %409, %412                                                                                        ;L3147<7<2158<49<150
 19963|  %415 = sub nuw i64 %412, %409                                                                                         ;L3147<7<2158<49<150
 19964|  %416 = sub nuw i64 %409, %412                                                                                         ;L3147<7<2158<49<150
 19965|  %417 = select i1 %414, i64 %415, i64 %416                                                                             ;L3147<7<2158<49<150
 19966|     ;; dx = i64 %417
 19967|  %418 = icmp ult i64 %411, %413                                                                                        ;L3147<8<2158<49<150
 19968|  %419 = sub nuw i64 %413, %411                                                                                         ;L3147<8<2158<49<150
 19969|  %420 = sub nuw i64 %411, %413                                                                                         ;L3147<8<2158<49<150
 19970|  %421 = select i1 %418, i64 %419, i64 %420                                                                             ;L3147<8<2158<49<150
 19971|     ;; dy = i64 %421
 19972|  %422 = mul i64 %417, %417                                                                                             ;L9<2158<49<150
 19973|  %423 = mul i64 %421, %421                                                                                             ;L9<2158<49<150
 19974|  %424 = add i64 %423, %422                                                                                             ;L9<2158<49<150
 19975|     ;; dist_sq = i64 %424
 19976|  %425 = add i64 %373, %209                                                                                             ;L26<48<150
 19977|  %426 = add i64 %425, %376                                                                                             ;L26<48<150
 19978|  %427 = add i64 %426, %380                                                                                             ;L48<150
 19979|  %428 = add i64 %427, %377                                                                                             ;L48<150
 19980|  %429 = add i64 %428, %392                                                                                             ;L48<150
 19981|  %430 = add i64 %429, %407                                                                                             ;L52<150
 19982|     ;; max_dist = i64 %430
 19983|  %431 = mul i64 %430, %430                                                                                             ;L53<150
 19984|  %432 = icmp ugt i64 %424, %431                                                                                        ;L53<150
 19985|  br i1 %432, label %361, label %433                                                                                    ;L53<150
 19986| 
 19987| 433: ; preds = %406
 19990|  %434 = gep %258, i64 1472                                                                                             ;L54<150
 19991|  %435 = load i64, ptr %434, , !!33265, !!8                                                                             ;L54<150
 19992|  invoke void @ai::small_action4castNtB5_16SmallActionSkill3new(ptr sret([24 x i8]) %18, ptr %5, i64 %435)
 19993|  to label %436 unwind label %250, !!33265                                                                              ;L54<150
 19994| 
 19995| 436: ; preds = %433
 19996|  call void @llvm.memcpy.p0.p0.i64(ptr %19, ptr %18, i64 24, i1 false), !!33329                                         ;L54<150
 19997|  store i8 16, ptr %216, , !!33329                                                                                      ;L54<150
 19999|     ;; self = ptr %23
 20000|     ;; self = ptr %23
 20001|     ;; value = ptr %19
 20002|     ;; src = ptr %19
 20003|     ;; additional = i64 1
 20004|     ;; needed_extra_cap = i64 1
 20005|     ;; needed_extra_cap = i64 1
 20006|     ;; strategy = i8 1
 20007|  %437 = load i64, ptr %191, , !!33819, !!8                                                                             ;L1428<54<150
 20008|     ;; self = ptr %23
 20009|  %438 = load i64, ptr %190, , !!33819, !!8                                                                             ;L149<1428<54<150
 20010|  %439 = icmp eq i64 %437, %438                                                                                         ;L1428<54<150
 20011|  br i1 %439, label %440, label %445                                                                                    ;L1428<54<150
 20012| 
 20013| 440: ; preds = %436
 20014|     ;; self = ptr %23
 20015|     ;; self = ptr %23
 20016|     ;; self = ptr %23
 20017|     ;; used_cap = i64 %437
 20018|     ;; used_cap = i64 %437
 20019|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %23, i64 %437, i64 1, i1 zeroext true)
 20020|  to label %441 unwind label %443, !!33827                                                                              ;L619<430<738<1429<54<150
 20021| 
 20022| 441: ; preds = %440
 20023|  %442 = load i64, ptr %191, , !!33819                                                                                  ;L1432<54<150
 20024|  br label %445                                                                                                         ;L619<430<738<1429<54<150
 20025| 
 20026| 443: ; preds = %440
 20027|  %444 = cleanuppad within none []
 20028|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %19) #34 [ "funclet"(token %444) ], !!33265 ;L1436<54<150
 20029|  cleanupret from %444 unwind label %250
 20030| 
 20031| 445: ; preds = %441, %436
 20032|  %446 = phi i64 [ %442, %441 ], [ %437, %436 ]                                                                         ;L1434<54<150
 20033|     ;; self = ptr %23
 20034|  %447 = load ptr, ptr %23, , !!33819, !!8, !!8                                                                         ;L138<1432<54<150
 20035|     ;; self = ptr %447
 20036|     ;; count = i64 %446
 20037|  %448 = gepS %447, i64 %446                                                                                            ;L961<1432<54<150
 20038|     ;; end = ptr %448
 20039|     ;; dst = ptr %448
 20040|  call void @llvm.memcpy.p0.p0.i64(ptr %448, ptr %19, i64 184, i1 false), !!33265                                       ;L1933<1433<54<150
 20041|  %449 = add i64 %446, 1                                                                                                ;L1434<54<150
 20042|  store i64 %449, ptr %191, , !!33819                                                                                   ;L1434<54<150
 20044|  br label %361                                                                                                         ;L53<150
 20045| 
 20046| 450: ; preds = %361
 20047|     ;; skill2 = ptr %364
 20048|     ;; self = ptr %364
 20049|  %451 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_skill2(ptr %184)
 20050|  to label %453 unwind label %250, !!33265                                                                              ;L60<150
 20051| 
 20052| 452: ; preds = %532, %493, %457, %453, %361, %264
 20053|  br label %221                                                                                                         ;L98<28<150
 20054| 
 20055| 453: ; preds = %450
 20056|  br i1 %451, label %454, label %452                                                                                    ;L60<150
 20057| 
 20058| 454: ; preds = %453
 20059|  %455 = gep %364, i64 40                                                                                               ;L60<150
 20060|  %456 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %455, ptr %184, ptr %258)
 20061|  to label %457 unwind label %250, !!33265                                                                              ;L60<150
 20062| 
 20063| 457: ; preds = %454
 20064|  br i1 %456, label %458, label %452                                                                                    ;L60<150
 20065| 
 20066| 458: ; preds = %457
 20067|  %459 = gep %364, i64 16                                                                                               ;L26<61<150
 20068|  %460 = load i64, ptr %459, , !!33265, !!8                                                                             ;L26<61<150
 20069|  %461 = gep %364, i64 24                                                                                               ;L26<61<150
 20070|  %462 = load i64, ptr %461, , !!33265, !!8                                                                             ;L26<61<150
 20071|  %463 = load i64, ptr %204, , !!33265, !!8                                                                             ;L26<61<150
 20072|  %464 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %364, ptr %184, ptr %258)
 20073|  to label %465 unwind label %250, !!33265                                                                              ;L61<150
 20074| 
 20075| 465: ; preds = %458
 20076|  %466 = add i64 %362, -1                                                                                               ;L26<61<150
 20077|  %467 = mul i64 %462, %466                                                                                             ;L26<61<150
 20078|  %468 = load i32, ptr %205, , !!33265, !!8                                                                             ;L1511<61<150
 20079|     ;; mult = i32 %468
 20080|  %469 = icmp eq i32 %468, 0                                                                                            ;L1512<61<150
 20081|  br i1 %469, label %470, label %472                                                                                    ;L1512<61<150
 20082| 
 20083| 470: ; preds = %465
 20084|  %471 = load i64, ptr %206, , !!33265, !!8                                                                             ;L1513<61<150
 20085|  br label %478                                                                                                         ;L1512<61<150
 20086| 
 20087| 472: ; preds = %465
 20088|  %473 = sext i32 %468 to i64                                                                                           ;L1511<61<150
 20089|     ;; mult = i64 %473
 20090|  %474 = load i64, ptr %206, , !!33265, !!8                                                                             ;L1515<61<150
 20091|  %475 = add nsw i64 %473, 100                                                                                          ;L1515<61<150
 20092|  %476 = mul i64 %474, %475                                                                                             ;L1515<61<150
 20093|  %477 = udiv i64 %476, 100                                                                                             ;L1515<61<150
 20094|  br label %478                                                                                                         ;L1512<61<150
 20095| 
 20096| 478: ; preds = %472, %470
 20097|  %479 = phi i64 [ %471, %470 ], [ %477, %472 ]                                                                         ;L0<61<150
 20098|  %480 = gep %258, i64 1136                                                                                             ;L1511<61<150
 20099|  %481 = load i32, ptr %480, , !!33265, !!8                                                                             ;L1511<61<150
 20100|     ;; mult = i32 %481
 20101|  %482 = icmp eq i32 %481, 0                                                                                            ;L1512<61<150
 20102|  br i1 %482, label %483, label %486                                                                                    ;L1512<61<150
 20103| 
 20104| 483: ; preds = %478
 20105|  %484 = gep %258, i64 1664                                                                                             ;L1513<61<150
 20106|  %485 = load i64, ptr %484, , !!33265, !!8                                                                             ;L1513<61<150
 20107|  br label %493                                                                                                         ;L1512<61<150
 20108| 
 20109| 486: ; preds = %478
 20110|  %487 = sext i32 %481 to i64                                                                                           ;L1511<61<150
 20111|     ;; mult = i64 %487
 20112|  %488 = gep %258, i64 1664                                                                                             ;L1515<61<150
 20113|  %489 = load i64, ptr %488, , !!33265, !!8                                                                             ;L1515<61<150
 20114|  %490 = add nsw i64 %487, 100                                                                                          ;L1515<61<150
 20115|  %491 = mul i64 %489, %490                                                                                             ;L1515<61<150
 20116|  %492 = udiv i64 %491, 100                                                                                             ;L1515<61<150
 20117|  br label %493                                                                                                         ;L1512<61<150
 20118| 
 20119| 493: ; preds = %486, %483
 20120|  %494 = phi i64 [ %485, %483 ], [ %492, %486 ]                                                                         ;L0<61<150
 20122|  %495 = gep %258, i64 1632                                                                                             ;L2158<62<150
 20123|  %496 = load i64, ptr %495, , !!33265, !!8                                                                             ;L2158<62<150
 20124|     ;; x1 = i64 %496
 20125|     ;; self = i64 %496
 20126|  %497 = gep %258, i64 1640                                                                                             ;L2158<62<150
 20127|  %498 = load i64, ptr %497, , !!33265, !!8                                                                             ;L2158<62<150
 20128|     ;; y1 = i64 %498
 20129|     ;; self = i64 %498
 20130|  %499 = load i64, ptr %207, , !!33265, !!8                                                                             ;L2158<62<150
 20131|     ;; x2 = i64 %499
 20132|     ;; other = i64 %499
 20133|  %500 = load i64, ptr %208, , !!33265, !!8                                                                             ;L2158<62<150
 20134|     ;; y2 = i64 %500
 20135|     ;; other = i64 %500
 20136|  %501 = icmp ult i64 %496, %499                                                                                        ;L3147<7<2158<62<150
 20137|  %502 = sub nuw i64 %499, %496                                                                                         ;L3147<7<2158<62<150
 20138|  %503 = sub nuw i64 %496, %499                                                                                         ;L3147<7<2158<62<150
 20139|  %504 = select i1 %501, i64 %502, i64 %503                                                                             ;L3147<7<2158<62<150
 20140|     ;; dx = i64 %504
 20141|  %505 = icmp ult i64 %498, %500                                                                                        ;L3147<8<2158<62<150
 20142|  %506 = sub nuw i64 %500, %498                                                                                         ;L3147<8<2158<62<150
 20143|  %507 = sub nuw i64 %498, %500                                                                                         ;L3147<8<2158<62<150
 20144|  %508 = select i1 %505, i64 %506, i64 %507                                                                             ;L3147<8<2158<62<150
 20145|     ;; dy = i64 %508
 20146|  %509 = mul i64 %504, %504                                                                                             ;L9<2158<62<150
 20147|  %510 = mul i64 %508, %508                                                                                             ;L9<2158<62<150
 20148|  %511 = add i64 %510, %509                                                                                             ;L9<2158<62<150
 20149|     ;; dist_sq = i64 %511
 20150|  %512 = add i64 %460, %209                                                                                             ;L26<61<150
 20151|  %513 = add i64 %512, %467                                                                                             ;L26<61<150
 20152|  %514 = add i64 %513, %463                                                                                             ;L61<150
 20153|  %515 = add i64 %514, %464                                                                                             ;L61<150
 20154|  %516 = add i64 %515, %479                                                                                             ;L61<150
 20155|  %517 = add i64 %516, %494                                                                                             ;L65<150
 20156|     ;; max_dist = i64 %517
 20157|  %518 = mul i64 %517, %517                                                                                             ;L66<150
 20158|  %519 = icmp ugt i64 %511, %518                                                                                        ;L66<150
 20159|  br i1 %519, label %452, label %520                                                                                    ;L66<150
 20160| 
 20161| 520: ; preds = %493
 20164|  %521 = gep %258, i64 1472                                                                                             ;L67<150
 20165|  %522 = load i64, ptr %521, , !!33265, !!8                                                                             ;L67<150
 20166|  invoke void @ai::small_action4castNtB5_17SmallActionSkill23new(ptr sret([24 x i8]) %16, ptr %5, i64 %522)
 20167|  to label %523 unwind label %250, !!33265                                                                              ;L67<150
 20168| 
 20169| 523: ; preds = %520
 20170|  call void @llvm.memcpy.p0.p0.i64(ptr %17, ptr %16, i64 24, i1 false), !!33329                                         ;L67<150
 20171|  store i8 17, ptr %218, , !!33329                                                                                      ;L67<150
 20173|     ;; self = ptr %23
 20174|     ;; self = ptr %23
 20175|     ;; value = ptr %17
 20176|     ;; src = ptr %17
 20177|     ;; additional = i64 1
 20178|     ;; needed_extra_cap = i64 1
 20179|     ;; needed_extra_cap = i64 1
 20180|     ;; strategy = i8 1
 20181|  %524 = load i64, ptr %191, , !!33888, !!8                                                                             ;L1428<67<150
 20182|     ;; self = ptr %23
 20183|  %525 = load i64, ptr %190, , !!33888, !!8                                                                             ;L149<1428<67<150
 20184|  %526 = icmp eq i64 %524, %525                                                                                         ;L1428<67<150
 20185|  br i1 %526, label %527, label %532                                                                                    ;L1428<67<150
 20186| 
 20187| 527: ; preds = %523
 20188|     ;; self = ptr %23
 20189|     ;; self = ptr %23
 20190|     ;; self = ptr %23
 20191|     ;; used_cap = i64 %524
 20192|     ;; used_cap = i64 %524
 20193|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %23, i64 %524, i64 1, i1 zeroext true)
 20194|  to label %528 unwind label %530, !!33896                                                                              ;L619<430<738<1429<67<150
 20195| 
 20196| 528: ; preds = %527
 20197|  %529 = load i64, ptr %191, , !!33888                                                                                  ;L1432<67<150
 20198|  br label %532                                                                                                         ;L619<430<738<1429<67<150
 20199| 
 20200| 530: ; preds = %527
 20201|  %531 = cleanuppad within none []
 20202|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %17) #34 [ "funclet"(token %531) ], !!33265 ;L1436<67<150
 20203|  cleanupret from %531 unwind label %250
 20204| 
 20205| 532: ; preds = %528, %523
 20206|  %533 = phi i64 [ %529, %528 ], [ %524, %523 ]                                                                         ;L1434<67<150
 20207|     ;; self = ptr %23
 20208|  %534 = load ptr, ptr %23, , !!33888, !!8, !!8                                                                         ;L138<1432<67<150
 20209|     ;; self = ptr %534
 20210|     ;; count = i64 %533
 20211|  %535 = gepS %534, i64 %533                                                                                            ;L961<1432<67<150
 20212|     ;; end = ptr %535
 20213|     ;; dst = ptr %535
 20214|  call void @llvm.memcpy.p0.p0.i64(ptr %535, ptr %17, i64 184, i1 false), !!33265                                       ;L1933<1433<67<150
 20215|  %536 = add i64 %533, 1                                                                                                ;L1434<67<150
 20216|  store i64 %536, ptr %191, , !!33888                                                                                   ;L1434<67<150
 20218|  br label %452                                                                                                         ;L66<150
 20219| 
 20220| 537: ; preds = %255, %246
 20222|  %538 = load ptr, ptr %23, , !!33910, !!8, !!8                                                                         ;L73<150
 20223|  %539 = load i64, ptr %191, , !!33910                                                                                  ;L73<150
 20226|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %37, ptr %538, i64 %539)
 20227|  to label %540 unwind label %48                                                                                        ;L150
 20228| 
 20229| 540: ; preds = %537
 20231|  invoke void @ai::fight_check20attack_summon_action(ptr sret([32 x i8]) %30, ptr %4, ptr %5)
 20232|  to label %541 unwind label %48                                                                                        ;L151
 20233| 
 20234| 541: ; preds = %540
 20235|  %542 = load ptr, ptr %30, , !!8, !!8                                                                                  ;L151
 20236|  %543 = gep %30, i64 24                                                                                                ;L151
 20237|  %544 = load i64, ptr %543, , !!8                                                                                      ;L151
 20238|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %37, ptr %542, i64 %544)
 20239|  to label %545 unwind label %48                                                                                        ;L151
 20240| 
 20241| 545: ; preds = %541
 20246|     ;; _version = i64 %2
 20248|     ;; player = ptr %4
 20249|     ;; data = ptr %5
 20254|  %546 = load ptr, ptr %58, , !!33963, !!8                                                                              ;L77<152
 20255|     ;; self = ptr %546
 20256|  %547 = icmp eq ptr %546, null                                                                                         ;L2775<77<152
 20257|  br i1 %547, label %742, label %548                                                                                    ;L2775<77<152
 20258| 
 20259| 548: ; preds = %545
 20260|     ;; champ = ptr %546
 20261|     ;; other = ptr %546
 20262|     ;; caster = ptr %546
 20263|     ;; self = ptr %546
 20265|  invoke void @gc::simulationNtB5_21AbstractGameWithCache11iter_towers(ptr sret([136 x i8]) %13, ptr %55, i64 %62)
 20266|  to label %549 unwind label %48                                                                                        ;L78<152
 20267| 
 20268| 549: ; preds = %548
 20269|     ;; self = ptr %13
 20270|     ;; f = ptr %546
 20271|     ;; self = ptr %12
 20275|     ;; self = ptr %13
 20276|     ;; f = ptr %546
 20277|  call void @llvm.memcpy.p0.p0.i64(ptr %12, ptr %13, i64 136, i1 false), !!34079                                        ;L69<836<3387<80<152
 20278|  %550 = gep %12, i64 136                                                                                               ;L69<836<3387<80<152
 20279|  store ptr %546, ptr %550, , !!34082                                                                                   ;L69<836<3387<80<152
 20281|     ;; self = ptr %12
 20284|     ;; self = ptr %12
 20286|     ;; self = ptr %12
 20289|  store ptr %550, ptr %10, , !!34132
 20290|     ;; self = ptr %12
 20291|     ;; predicate = ptr %10
 20292|  %551 = gep %12, i64 16                                                                                                ;L169<98<107<2706<3416<3387<80<152
 20294|     ;; opt = ptr %551
 20295|     ;; self = ptr %551
 20296|     ;; f = ptr %10
 20297|  %552 = load i64, ptr %551, , !!34185, !!8                                                                             ;L764<332<169<98<107<2706<3416<3387<80<152
 20298|  %553 = icmp eq i64 %552, -2                                                                                           ;L764<332<169<98<107<2706<3416<3387<80<152
 20299|  br i1 %553, label %597, label %554                                                                                    ;L764<332<169<98<107<2706<3416<3387<80<152
 20300| 
 20301| 554: ; preds = %549
 20303|     ;; predicate = ptr %10
 20304|     ;; a = ptr %551
 20306|     ;; predicate = ptr %10
 20307|     ;; self = ptr %551
 20309|     ;; opt = ptr %551
 20310|     ;; self = ptr %551
 20312|  %555 = icmp eq i64 %552, -1                                                                                           ;L764<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20313|  br i1 %555, label %586, label %556                                                                                    ;L764<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20314| 
 20315| 556: ; preds = %554
 20318|     ;; a = ptr %551
 20320|     ;; self = ptr %551
 20323|     ;; self = ptr %551
 20327|     ;; self = ptr %551
 20331|     ;; self = ptr %551
 20334|     ;; self = ptr %551
 20337|  %557 = trunc nuw i64 %552 to i1                                                                                       ;L396<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20338|  br i1 %557, label %558, label %584                                                                                    ;L396<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20339| 
 20340| 558: ; preds = %556
 20341|  %559 = gep %12, i64 24                                                                                                ;L396<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20342|     ;; iter = ptr %559
 20344|     ;; self = ptr %559
 20349|     ;; self[0..+8] = ptr %559
 20350|     ;; self[8..+8] = i64 6
 20351|  %560 = gep %12, i64 40                                                                                                ;L214<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20352|     ;; data[0..+8] = ptr %560
 20353|     ;; data[8..+8] = i64 6
 20354|     ;; f[0..+8] = ptr %560
 20355|     ;; f[8..+8] = i64 6
 20359|     ;; self = ptr %559
 20360|     ;; self = ptr %559
 20361|     ;; self = ptr %559
 20363|     ;; rhs = i64 1
 20364|  %561 = load i64, ptr %559, , !!34429, !!8                                                                             ;L122<166<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20365|  %562 = gep %12, i64 32                                                                                                ;L122<166<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20366|  %563 = load i64, ptr %562, , !!34429, !!8                                                                             ;L122<166<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20367|  %564 = icmp ule i64 %561, %563                                                                                        ;L122<166<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20368|     ;; cond = i1 true
 20369|  call void @llvm.assume(i1 %564)                                                                                       ;L210<122<166<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20370|  %565 = icmp eq i64 %561, %563                                                                                         ;L167<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20371|  br i1 %565, label %584, label %566                                                                                    ;L167<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20372| 
 20373| 566: ; preds = %581, %558
 20374|  %567 = phi i64 [ %568, %581 ], [ %561, %558 ]
 20375|     ;; i = i64 %567
 20376|     ;; value = i64 %567
 20377|     ;; self = i64 %567
 20378|  %568 = add nuw nsw i64 %567, 1                                                                                        ;L971<63<169<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20380|     ;; f = ptr undef
 20382|     ;; idx = i64 %567
 20383|     ;; index = i64 %567
 20384|     ;; self = i64 %567
 20385|     ;; self[0..+8] = ptr %560
 20386|     ;; slice[0..+8] = ptr %560
 20387|     ;; self[8..+8] = i64 6
 20388|     ;; slice[8..+8] = i64 6
 20389|  %569 = icmp ult i64 %567, 6                                                                                           ;L252<646<219<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20390|  call void @llvm.assume(i1 %569)                                                                                       ;L252<646<219<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20391|  %570 = getelementptr ptr, ptr %560, i64 %567                                                                          ;L253<646<219<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20392|     ;; self = ptr %570
 20393|     ;; self = ptr %570
 20394|     ;; src = ptr %570
 20395|  %571 = load ptr, ptr %570, , !!34474, !!8                                                                             ;L1733<1171<798<219<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20396|     ;; elem = ptr %571
 20399|     ;; inner = ptr %571
 20400|  %572 = icmp eq ptr %571, null                                                                                         ;L819<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20401|  br i1 %572, label %581, label %573                                                                                    ;L819<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20402| 
 20403| 573: ; preds = %566
 20404|     ;; item = ptr %571
 20405|     ;; x = ptr %571
 20416|     ;; self = ptr %571
 20417|  %574 = gep %571, i64 1721                                                                                             ;L1478<79<298<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20418|  %575 = load i8, ptr %574, , !!34557, !!8                                                                              ;L1478<79<298<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20419|  %576 = trunc nuw i8 %575 to i1                                                                                        ;L1478<79<298<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20420|  %577 = gep %571, i64 1696                                                                                             ;L1478<79<298<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20421|  %578 = load i64, ptr %577, , !!34560                                                                                  ;L1478<79<298<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20422|  %579 = icmp eq i64 %578, 0                                                                                            ;L1478<79<298<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20423|  %580 = select i1 %576, i1 %579, i1 false                                                                              ;L1478<79<298<298<298<2967<820<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20424|  br i1 %580, label %585, label %581                                                                                    ;L820<220<170<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20425| 
 20426| 581: ; preds = %573, %566
 20427|  %582 = icmp eq i64 %568, %563                                                                                         ;L167<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20428|  br i1 %582, label %583, label %566                                                                                    ;L167<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20429| 
 20430| 583: ; preds = %581
 20431|  store i64 %563, ptr %559, , !!34429                                                                                   ;L63<169<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20432|     ;; x = ptr null
 20433|  br label %584                                                                                                         ;L333<169<169<332<169<98<107<2706<3416<3387<80<152
 20434| 
 20435| 584: ; preds = %583, %558, %556
 20436|  store i64 -1, ptr %551, , !!34567                                                                                     ;L334<169<169<332<169<98<107<2706<3416<3387<80<152
 20437|  br label %586                                                                                                         ;L333<169<169<332<169<98<107<2706<3416<3387<80<152
 20438| 
 20439| 585: ; preds = %573
 20440|  store i64 %568, ptr %559, , !!34429                                                                                   ;L63<169<215<263<397<89<880<242<2971<169<332<169<169<332<169<98<107<2706<3416<3387<80<152
 20441|     ;; x = ptr %571
 20442|     ;; self = ptr %571
 20443|     ;; f[0..+8] = ptr %551
 20444|     ;; f[8..+8] = ptr %10
 20445|     ;; x = ptr %571
 20448|  br label %595                                                                                                         ;L333<169<98<107<2706<3416<3387<80<152
 20449| 
 20450| 586: ; preds = %584, %554
 20451|  %587 = gep %12, i64 120                                                                                               ;L170<169<332<169<98<107<2706<3416<3387<80<152
 20452|     ;; self = ptr null
 20453|     ;; f[0..+8] = ptr %587
 20454|     ;; f[8..+8] = ptr %10
 20458|     ;; self = ptr %587
 20459|  %588 = load ptr, ptr %587, , !!34623, !!8                                                                             ;L764<170<1653<170<169<332<169<98<107<2706<3416<3387<80<152
 20460|  %589 = icmp eq ptr %588, null                                                                                         ;L764<170<1653<170<169<332<169<98<107<2706<3416<3387<80<152
 20461|  br i1 %589, label %594, label %590                                                                                    ;L764<170<1653<170<169<332<169<98<107<2706<3416<3387<80<152
 20462| 
 20463| 590: ; preds = %586
 20464|     ;; self = ptr %587
 20465|     ;; predicate = ptr %10
 20466|  %591 = invoke ptr @core::iter8adapters6copiedINtB5_6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB2p_4find5checkB1s_QQNCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan12attack_nexusNtB3E_18AttackNexusSubPlan19attack_tower_action0E0INtNtNtBb_3ops12control_flow11ControlFlowB1s_EEB3K_(ptr %587, ptr %10)
 20467|  to label %592 unwind label %48                                                                                        ;L2971<170<1653<170<169<332<169<98<107<2706<3416<3387<80<152
 20468| 
 20469| 592: ; preds = %590
 20470|     ;; x = ptr %591
 20473|  %593 = icmp eq ptr %591, null                                                                                         ;L633<682<333<169<98<107<2706<3416<3387<80<152
 20474|  br i1 %593, label %594, label %595                                                                                    ;L333<169<98<107<2706<3416<3387<80<152
 20475| 
 20476| 594: ; preds = %592, %586
 20477|  store i64 -2, ptr %551, , !!34185                                                                                     ;L334<169<98<107<2706<3416<3387<80<152
 20478|  br label %597                                                                                                         ;L333<169<98<107<2706<3416<3387<80<152
 20479| 
 20480| 595: ; preds = %592, %585
 20481|  %596 = phi ptr [ %591, %592 ], [ %571, %585 ]                                                                         ;L0<169<98<107<2706<3416<3387<80<152
 20483|     ;; self = ptr %596
 20484|     ;; f[0..+8] = ptr %12
 20486|     ;; self = ptr %596
 20487|     ;; f = ptr %12
 20488|     ;; self = ptr %12
 20489|  br label %614                                                                                                         ;L1161<107<2706<3416<3387<80<152
 20490| 
 20491| 597: ; preds = %594, %549
 20492|     ;; self = ptr null
 20493|     ;; f[0..+8] = ptr %12
 20499|     ;; self = ptr %12
 20500|  %598 = load i64, ptr %12, , !!34713, !!8                                                                              ;L764<170<1653<170<98<107<2706<3416<3387<80<152
 20501|  %599 = trunc nuw i64 %598 to i1                                                                                       ;L764<170<1653<170<98<107<2706<3416<3387<80<152
 20502|  br i1 %599, label %600, label %613                                                                                    ;L764<170<1653<170<98<107<2706<3416<3387<80<152
 20503| 
 20504| 600: ; preds = %597
 20505|  %601 = gep %12, i64 8                                                                                                 ;L764<170<1653<170<98<107<2706<3416<3387<80<152
 20507|     ;; self = ptr %601
 20511|     ;; self = ptr %601
 20514|  %602 = load ptr, ptr %601, , !!34713
 20515|     ;; self = ptr %601
 20516|  %603 = icmp eq ptr %602, null                                                                                         ;L2493<2971<170<1653<170<98<107<2706<3416<3387<80<152
 20517|  br i1 %603, label %613, label %604                                                                                    ;L2493<2971<170<1653<170<98<107<2706<3416<3387<80<152
 20518| 
 20519| 604: ; preds = %600
 20520|     ;; x = ptr %602
 20521|     ;; x = ptr %602
 20528|     ;; self = ptr %602
 20529|  %605 = gep %602, i64 1721                                                                                             ;L1478<79<298<2967<2494<2971<170<1653<170<98<107<2706<3416<3387<80<152
 20530|  %606 = load i8, ptr %605, , !!34793, !!8                                                                              ;L1478<79<298<2967<2494<2971<170<1653<170<98<107<2706<3416<3387<80<152
 20531|  %607 = trunc nuw i8 %606 to i1                                                                                        ;L1478<79<298<2967<2494<2971<170<1653<170<98<107<2706<3416<3387<80<152
 20532|  %608 = gep %602, i64 1696                                                                                             ;L1478<79<298<2967<2494<2971<170<1653<170<98<107<2706<3416<3387<80<152
 20533|  %609 = load i64, ptr %608, , !!34793                                                                                  ;L1478<79<298<2967<2494<2971<170<1653<170<98<107<2706<3416<3387<80<152
 20534|  %610 = icmp eq i64 %609, 0                                                                                            ;L1478<79<298<2967<2494<2971<170<1653<170<98<107<2706<3416<3387<80<152
 20535|  %611 = select i1 %607, i1 %610, i1 false                                                                              ;L2494<2971<170<1653<170<98<107<2706<3416<3387<80<152
 20536|  br i1 %611, label %612, label %613                                                                                    ;L2494<2971<170<1653<170<98<107<2706<3416<3387<80<152
 20537| 
 20538| 612: ; preds = %604
 20539|  store ptr null, ptr %601, , !!34713                                                                                   ;L0<1898<2494<2629<2493<2971<170<1653<170<98<107<2706<3416<3387<80<152
 20540|     ;; self = ptr %602
 20541|     ;; f = ptr %12
 20542|     ;; self = ptr %12
 20543|  br label %614                                                                                                         ;L1161<107<2706<3416<3387<80<152
 20544| 
 20545| 613: ; preds = %604, %600, %597
 20548|     ;; self = ptr null
 20550|  br label %742                                                                                                         ;L2775<78<152
 20551| 
 20552| 614: ; preds = %612, %595
 20553|  %615 = phi ptr [ %596, %595 ], [ %602, %612 ]
 20555|     ;; f = ptr %550
 20556|     ;; self = ptr %550
 20557|     ;; x = ptr %615
 20558|     ;; args = ptr %615
 20559|  %616 = load ptr, ptr %550, , !!34056, !!8, !!8                                                                        ;L310<1162<107<2706<3416<3387<80<152
 20561|     ;; x = ptr %615
 20565|     ;; self = ptr %615
 20566|     ;; other = ptr %616
 20567|  %617 = gep %615, i64 1632                                                                                             ;L2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20568|  %618 = load i64, ptr %617, , !!34847, !!8                                                                             ;L2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20569|     ;; x1 = i64 %618
 20570|     ;; self = i64 %618
 20571|  %619 = gep %615, i64 1640                                                                                             ;L2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20572|  %620 = load i64, ptr %619, , !!34847, !!8                                                                             ;L2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20573|     ;; y1 = i64 %620
 20574|     ;; self = i64 %620
 20575|  %621 = gep %616, i64 1632                                                                                             ;L2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20576|  %622 = load i64, ptr %621, , !!34868, !!8                                                                             ;L2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20577|     ;; x2 = i64 %622
 20578|     ;; other = i64 %622
 20579|  %623 = gep %616, i64 1640                                                                                             ;L2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20580|  %624 = load i64, ptr %623, , !!34868, !!8                                                                             ;L2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20581|     ;; y2 = i64 %624
 20582|     ;; other = i64 %624
 20583|  %625 = icmp ult i64 %618, %622                                                                                        ;L3147<7<2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20584|  %626 = sub nuw i64 %622, %618                                                                                         ;L3147<7<2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20585|  %627 = sub nuw i64 %618, %622                                                                                         ;L3147<7<2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20586|  %628 = select i1 %625, i64 %626, i64 %627                                                                             ;L3147<7<2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20587|     ;; dx = i64 %628
 20588|  %629 = icmp ult i64 %620, %624                                                                                        ;L3147<8<2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20589|  %630 = sub nuw i64 %624, %620                                                                                         ;L3147<8<2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20590|  %631 = sub nuw i64 %620, %624                                                                                         ;L3147<8<2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20591|  %632 = select i1 %629, i64 %630, i64 %631                                                                             ;L3147<8<2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20592|     ;; dy = i64 %632
 20593|  %633 = mul i64 %628, %628                                                                                             ;L9<2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20594|  %634 = mul i64 %632, %632                                                                                             ;L9<2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20595|  %635 = add i64 %634, %633                                                                                             ;L9<2158<80<3379<310<1162<107<2706<3416<3387<80<152
 20596|     ;; first[0..+8] = i64 %635
 20597|     ;; first[8..+8] = ptr %615
 20599|  call void @llvm.memcpy.p0.p0.i64(ptr %11, ptr %12, i64 144, i1 false), !!34056                                        ;L2707<3416<3387<80<152
 20600|  %636 = invoke { i64, ptr } @core::iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtB8_5chain5ChainIB1k_INtNtB8_7flatten7FlattenINtNtNtBc_5array4iter8IntoIterINtNtBc_6option6OptionRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEKj6_EEINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterB2W_EEEINtB2D_8IntoIterB2W_EENCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan12attack_nexusNtB5i_18AttackNexusSubPlan19attack_tower_action0ENCINvNvNtNtNtBa_6traits8iterator8Iterator10min_by_key3keyB2W_yNCB5f_s_0E0EB7e_4foldTyB2W_ENCINvNvB7e_6min_by4foldB8s_INvB7c_7compareB2W_yEE0EB5o_(ptr %11, i64 %635, ptr %615)
 20601|  to label %637 unwind label %48                                                                                        ;L2707<3416<3387<80<152
 20602| 
 20603| 637: ; preds = %614
 20604|  %638 = extractvalue { i64, ptr } %636, 1                                                                              ;L2707<3416<3387<80<152
 20607|     ;; self = ptr %638
 20609|  %639 = icmp eq ptr %638, null                                                                                         ;L2775<78<152
 20610|  br i1 %639, label %742, label %640                                                                                    ;L2775<78<152
 20611| 
 20612| 640: ; preds = %637
 20613|     ;; nearest_enemy_tower = ptr %638
 20614|     ;; self = ptr %638
 20615|     ;; self = ptr %638
 20616|  %641 = gep %546, i64 1648                                                                                             ;L82<152
 20617|  %642 = load i64, ptr %641, , !!33963, !!8                                                                             ;L82<152
 20618|  %643 = gep %546, i64 1576                                                                                             ;L82<152
 20619|  %644 = load i64, ptr %643, , !!33963, !!8                                                                             ;L82<152
 20620|     ;; self = i64 %644
 20621|     ;; other = i64 1
 20622|  %645 = call i64 @llvm.umax.i64(i64 %644, i64 1)                                                                       ;L1039<82<152
 20623|  %646 = mul i64 %642, 100                                                                                              ;L82<152
 20624|  %647 = udiv i64 %646, %645                                                                                            ;L82<152
 20625|     ;; hp_ratio = i64 %647
 20626|  %648 = icmp ult i64 %647, 56                                                                                          ;L83<152
 20627|  br i1 %648, label %655, label %649                                                                                    ;L83<152
 20628| 
 20629| 649: ; preds = %657, %640
 20630|  %650 = gep %546, i64 1600                                                                                             ;L88<152
 20631|  %651 = load i64, ptr %650, , !!34889, !!8                                                                             ;L88<152
 20632|     ;; move_speed = i64 %651
 20633|     ;; self = ptr %546
 20634|  %652 = gep %546, i64 1216                                                                                             ;L742<89<152
 20635|  %653 = load i32, ptr %652, , !!34889, !!8                                                                             ;L742<89<152
 20636|  %654 = icmp eq i32 %653, -1                                                                                           ;L742<89<152
 20637|  br i1 %654, label %742, label %658                                                                                    ;L742<89<152
 20638| 
 20639| 655: ; preds = %640
 20640|  %656 = invoke zeroext i1 @ai::tower_discipline29can_tower_focused_when_attack(ptr %39, ptr %55, ptr %4, ptr %638)
 20641|  to label %657 unwind label %48                                                                                        ;L84<152
 20642| 
 20643| 657: ; preds = %655
 20644|  br i1 %656, label %742, label %649                                                                                    ;L84<152
 20645| 
 20646| 658: ; preds = %649
 20647|  %659 = gep %546, i64 1168                                                                                             ;L742<89<152
 20648|     ;; atk = ptr %659
 20649|     ;; self = ptr %659
 20650|  %660 = gep %638, i64 1632                                                                                             ;L2158<92<152
 20651|  %661 = load i64, ptr %660, , !!34889, !!8                                                                             ;L2158<92<152
 20652|     ;; x1 = i64 %661
 20653|     ;; self = i64 %661
 20654|  %662 = gep %638, i64 1640                                                                                             ;L2158<92<152
 20655|  %663 = load i64, ptr %662, , !!34889, !!8                                                                             ;L2158<92<152
 20656|     ;; y1 = i64 %663
 20657|     ;; self = i64 %663
 20658|  %664 = gep %546, i64 1632                                                                                             ;L2158<92<152
 20659|  %665 = load i64, ptr %664, , !!34889, !!8                                                                             ;L2158<92<152
 20660|     ;; x2 = i64 %665
 20661|     ;; other = i64 %665
 20662|  %666 = gep %546, i64 1640                                                                                             ;L2158<92<152
 20663|  %667 = load i64, ptr %666, , !!34889, !!8                                                                             ;L2158<92<152
 20664|     ;; y2 = i64 %667
 20665|     ;; other = i64 %667
 20666|  %668 = icmp ult i64 %661, %665                                                                                        ;L3147<7<2158<92<152
 20667|  %669 = sub nuw i64 %665, %661                                                                                         ;L3147<7<2158<92<152
 20668|  %670 = sub nuw i64 %661, %665                                                                                         ;L3147<7<2158<92<152
 20669|  %671 = select i1 %668, i64 %669, i64 %670                                                                             ;L3147<7<2158<92<152
 20670|     ;; dx = i64 %671
 20671|  %672 = icmp ult i64 %663, %667                                                                                        ;L3147<8<2158<92<152
 20672|  %673 = sub nuw i64 %667, %663                                                                                         ;L3147<8<2158<92<152
 20673|  %674 = sub nuw i64 %663, %667                                                                                         ;L3147<8<2158<92<152
 20674|  %675 = select i1 %672, i64 %673, i64 %674                                                                             ;L3147<8<2158<92<152
 20675|     ;; dy = i64 %675
 20676|  %676 = mul i64 %671, %671                                                                                             ;L9<2158<92<152
 20677|  %677 = mul i64 %675, %675                                                                                             ;L9<2158<92<152
 20678|  %678 = add i64 %677, %676                                                                                             ;L9<2158<92<152
 20679|     ;; dist = i64 %678
 20680|  %679 = gep %546, i64 1184                                                                                             ;L26<93<152
 20681|  %680 = load i64, ptr %679, , !!34889, !!8                                                                             ;L26<93<152
 20682|  %681 = gep %546, i64 1192                                                                                             ;L26<93<152
 20683|  %682 = load i64, ptr %681, , !!34889, !!8                                                                             ;L26<93<152
 20684|  %683 = gep %546, i64 1480                                                                                             ;L26<93<152
 20685|  %684 = load i64, ptr %683, , !!34889, !!8                                                                             ;L26<93<152
 20686|  %685 = add i64 %684, -1                                                                                               ;L26<93<152
 20687|  %686 = mul i64 %685, %682                                                                                             ;L26<93<152
 20688|  %687 = gep %546, i64 1080                                                                                             ;L26<93<152
 20689|  %688 = load i64, ptr %687, , !!34889, !!8                                                                             ;L26<93<152
 20690|  %689 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %659, ptr %546, ptr %638)
 20691|  to label %690 unwind label %48                                                                                        ;L93<152
 20692| 
 20693| 690: ; preds = %658
 20694|  %691 = gep %546, i64 1136                                                                                             ;L1511<93<152
 20695|  %692 = load i32, ptr %691, , !!34889, !!8                                                                             ;L1511<93<152
 20696|     ;; mult = i32 %692
 20697|  %693 = icmp eq i32 %692, 0                                                                                            ;L1512<93<152
 20698|  br i1 %693, label %694, label %697                                                                                    ;L1512<93<152
 20699| 
 20700| 694: ; preds = %690
 20701|  %695 = gep %546, i64 1664                                                                                             ;L1513<93<152
 20702|  %696 = load i64, ptr %695, , !!34889, !!8                                                                             ;L1513<93<152
 20703|  br label %704                                                                                                         ;L1512<93<152
 20704| 
 20705| 697: ; preds = %690
 20706|  %698 = sext i32 %692 to i64                                                                                           ;L1511<93<152
 20707|     ;; mult = i64 %698
 20708|  %699 = gep %546, i64 1664                                                                                             ;L1515<93<152
 20709|  %700 = load i64, ptr %699, , !!34889, !!8                                                                             ;L1515<93<152
 20710|  %701 = add nsw i64 %698, 100                                                                                          ;L1515<93<152
 20711|  %702 = mul i64 %700, %701                                                                                             ;L1515<93<152
 20712|  %703 = udiv i64 %702, 100                                                                                             ;L1515<93<152
 20713|  br label %704                                                                                                         ;L1512<93<152
 20714| 
 20715| 704: ; preds = %697, %694
 20716|  %705 = phi i64 [ %696, %694 ], [ %703, %697 ]                                                                         ;L0<93<152
 20717|  %706 = gep %638, i64 1136                                                                                             ;L1511<93<152
 20718|  %707 = load i32, ptr %706, , !!34889, !!8                                                                             ;L1511<93<152
 20719|     ;; mult = i32 %707
 20720|  %708 = icmp eq i32 %707, 0                                                                                            ;L1512<93<152
 20721|  br i1 %708, label %709, label %712                                                                                    ;L1512<93<152
 20722| 
 20723| 709: ; preds = %704
 20724|  %710 = gep %638, i64 1664                                                                                             ;L1513<93<152
 20725|  %711 = load i64, ptr %710, , !!34889, !!8                                                                             ;L1513<93<152
 20726|  br label %719                                                                                                         ;L1512<93<152
 20727| 
 20728| 712: ; preds = %704
 20729|  %713 = sext i32 %707 to i64                                                                                           ;L1511<93<152
 20730|     ;; mult = i64 %713
 20731|  %714 = gep %638, i64 1664                                                                                             ;L1515<93<152
 20732|  %715 = load i64, ptr %714, , !!34889, !!8                                                                             ;L1515<93<152
 20733|  %716 = add nsw i64 %713, 100                                                                                          ;L1515<93<152
 20734|  %717 = mul i64 %715, %716                                                                                             ;L1515<93<152
 20735|  %718 = udiv i64 %717, 100                                                                                             ;L1515<93<152
 20736|  br label %719                                                                                                         ;L1512<93<152
 20737| 
 20738| 719: ; preds = %712, %709
 20739|  %720 = phi i64 [ %711, %709 ], [ %718, %712 ]                                                                         ;L0<93<152
 20741|  %721 = mul i64 %651, 30                                                                                               ;L94<152
 20742|  %722 = add i64 %680, %721                                                                                             ;L26<93<152
 20743|  %723 = add i64 %722, %688                                                                                             ;L26<93<152
 20744|  %724 = add i64 %723, %686                                                                                             ;L93<152
 20745|  %725 = add i64 %724, %689                                                                                             ;L93<152
 20746|  %726 = add i64 %725, %705                                                                                             ;L93<152
 20747|  %727 = add i64 %726, %720                                                                                             ;L94<152
 20748|     ;; max_dist = i64 %727
 20749|  %728 = mul i64 %727, %727                                                                                             ;L95<152
 20750|  %729 = icmp ugt i64 %678, %728                                                                                        ;L95<152
 20751|  br i1 %729, label %742, label %730                                                                                    ;L95<152
 20752| 
 20753| 730: ; preds = %719
 20754|  %731 = gep %638, i64 1472                                                                                             ;L99<152
 20755|  %732 = load i64, ptr %731, , !!34889, !!8                                                                             ;L99<152
 20756|  invoke void @ai::small_action4castNtB2_17SmallActionAttack3new(ptr sret([24 x i8]) %29, ptr %5, i64 %732)
 20757|  to label %733 unwind label %48                                                                                        ;L99<152
 20758| 
 20759| 733: ; preds = %730
 20760|  %734 = gep %29, i64 177                                                                                               ;L99<152
 20761|  store i8 15, ptr %734, , !!34944                                                                                      ;L99<152
 20763|     ;; iter[0..+177] = ptr %29
 20771|     ;; self = ptr %29
 20772|     ;; self = ptr %37
 20773|     ;; self = ptr %37
 20774|     ;; iter = ptr %29
 20775|     ;; iter = ptr %29
 20776|     ;; t = ptr %9
 20777|     ;; strategy = i8 1
 20778|     ;; additional = i64 1
 20779|     ;; needed_extra_cap = i64 1
 20780|     ;; needed_extra_cap = i64 1
 20781|     ;; self = ptr %37
 20782|     ;; self = ptr %37
 20783|     ;; self = ptr %37
 20784|  %735 = load i64, ptr %43, , !!35046, !!8                                                                              ;L738<2153<152
 20785|     ;; used_cap = i64 %735
 20786|     ;; used_cap = i64 %735
 20787|     ;; rhs = i64 %735
 20788|  %736 = load i64, ptr %42, , !!35046, !!8                                                                              ;L149<614<430<738<2153<152
 20789|     ;; self = i64 %736
 20790|  %737 = icmp eq i64 %736, %735                                                                                         ;L614<430<738<2153<152
 20791|  br i1 %737, label %741, label %743                                                                                    ;L614<430<738<2153<152
 20792| 
 20793| 738: ; preds = %753, %741
 20794|  %739 = phi i1 [ true, %753 ], [ false, %741 ]                                                                         ;L0<152
 20795|  %740 = cleanuppad within none []
 20799|  br i1 %739, label %760, label %761                                                                                    ;L2158<152
 20800| 
 20801| 741: ; preds = %733
 20802|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %37, i64 %735, i64 1, i1 zeroext true)
 20803|  to label %743 unwind label %738, !!35046                                                                              ;L619<430<738<2153<152
 20804| 
 20805| 742: ; preds = %719, %657, %649, %637, %613, %545
 20806|     ;; iter[0..+177] = ptr %29
 20814|     ;; self = ptr %29
 20815|     ;; self = ptr %37
 20816|     ;; self = ptr %37
 20817|     ;; iter = ptr %29
 20818|     ;; iter = ptr %29
 20819|     ;; t = ptr %9
 20820|     ;; strategy = i8 1
 20821|     ;; additional = i64 0
 20822|     ;; needed_extra_cap = i64 0
 20823|     ;; needed_extra_cap = i64 0
 20824|     ;; self = ptr %37
 20825|     ;; self = ptr %37
 20826|     ;; self = ptr %37
 20831|     ;; iter[177..+1] = i8 -1
 20832|  br label %762                                                                                                         ;L2155<152
 20833| 
 20834| 743: ; preds = %741, %733
 20835|     ;; iter[177..+1] = i8 15
 20836|  %744 = gep %29, i64 178                                                                                               ;L2155<152
 20837|  %745 = gep %9, i64 177
 20838|  %746 = gep %9, i64 178
 20840|  call void @llvm.memcpy.p0.p0.i64(ptr %9, ptr %29, i64 177, i1 false), !!34945                                         ;L2155<152
 20841|  store i8 15, ptr %745, , !!35065                                                                                      ;L2155<152
 20842|  call void @llvm.memcpy.p0.p0.i64(ptr %746, ptr %744, i64 6, i1 false), !!34945                                        ;L2155<152
 20843|     ;; self = ptr %37
 20844|     ;; self = ptr %37
 20845|     ;; value = ptr %9
 20846|     ;; src = ptr %9
 20847|     ;; additional = i64 1
 20848|     ;; needed_extra_cap = i64 1
 20849|     ;; needed_extra_cap = i64 1
 20850|     ;; strategy = i8 1
 20851|  %747 = load i64, ptr %43, , !!35081, !!8                                                                              ;L1428<2156<152
 20852|     ;; self = ptr %37
 20853|  %748 = load i64, ptr %42, , !!35081, !!8                                                                              ;L149<1428<2156<152
 20854|  %749 = icmp eq i64 %747, %748                                                                                         ;L1428<2156<152
 20855|  br i1 %749, label %750, label %755                                                                                    ;L1428<2156<152
 20856| 
 20857| 750: ; preds = %743
 20858|     ;; self = ptr %37
 20859|     ;; self = ptr %37
 20860|     ;; self = ptr %37
 20861|     ;; used_cap = i64 %747
 20862|     ;; used_cap = i64 %747
 20863|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %37, i64 %747, i64 1, i1 zeroext true)
 20864|  to label %751 unwind label %753, !!35081                                                                              ;L619<430<738<1429<2156<152
 20865| 
 20866| 751: ; preds = %750
 20867|  %752 = load i64, ptr %43, , !!35081                                                                                   ;L1432<2156<152
 20868|  br label %755                                                                                                         ;L619<430<738<1429<2156<152
 20869| 
 20870| 753: ; preds = %750
 20871|  %754 = cleanuppad within none []
 20872|     ;; iter[177..+1] = i8 -1
 20873|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %9) #34 [ "funclet"(token %754) ], !!35046 ;L1436<2156<152
 20877|  cleanupret from %754 unwind label %738                                                                                ;L2157<152
 20878| 
 20879| 755: ; preds = %751, %743
 20880|  %756 = phi i64 [ %752, %751 ], [ %747, %743 ]                                                                         ;L1434<2156<152
 20881|     ;; self = ptr %37
 20882|  %757 = load ptr, ptr %37, , !!35081, !!8, !!8                                                                         ;L138<1432<2156<152
 20883|     ;; self = ptr %757
 20884|     ;; count = i64 %756
 20885|  %758 = gepS %757, i64 %756                                                                                            ;L961<1432<2156<152
 20886|     ;; end = ptr %758
 20887|     ;; dst = ptr %758
 20888|  call void @llvm.memcpy.p0.p0.i64(ptr %758, ptr %9, i64 184, i1 false), !!35046                                        ;L1933<1433<2156<152
 20889|  %759 = add i64 %756, 1                                                                                                ;L1434<2156<152
 20890|  store i64 %759, ptr %43, , !!35081                                                                                    ;L1434<2156<152
 20892|     ;; self = ptr undef
 20893|  br label %762                                                                                                         ;L0<1898<2494<2629<2155<152
 20894| 
 20895| 760: ; preds = %761, %738
 20896|  cleanupret from %740 unwind label %48
 20897| 
 20898| 761: ; preds = %738
 20899|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %29) [ "funclet"(token %740) ] ;L825<825<825<2158<152
 20900|  br label %760                                                                                                         ;L825<825<825<2158<152
 20901| 
 20902| 762: ; preds = %755, %742
 20903|     ;; iter[177..+1] = i8 -1
 20909|  invoke void @ai::fight_check29attack_structure_skill_action(ptr sret([32 x i8]) %28, ptr %4, ptr %5)
 20910|  to label %763 unwind label %48                                                                                        ;L153
 20911| 
 20912| 763: ; preds = %762
 20913|  %764 = load ptr, ptr %28, , !!8, !!8                                                                                  ;L153
 20914|  %765 = gep %28, i64 24                                                                                                ;L153
 20915|  %766 = load i64, ptr %765, , !!8                                                                                      ;L153
 20916|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %37, ptr %764, i64 %766)
 20917|  to label %767 unwind label %48                                                                                        ;L153
 20918| 
 20919| 767: ; preds = %763
 20921|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %37, i64 32, i1 false)                                                   ;L155
 20922|  br label %768                                                                                                         ;L156
 20923| 
 20924| 768: ; preds = %779, %767
 20926|  ret void                                                                                                              ;L156
 20927| 
 20928| 769: ; preds = %130
 20929|  call void @llvm.memcpy.p0.p0.i64(ptr %35, ptr %34, i64 136, i1 false)                                                 ;L142
 20930|  %770 = gep %35, i64 177                                                                                               ;L142
 20931|  store i8 3, ptr %770,                                                                                                 ;L142
 20933|     ;; self = ptr %37
 20934|     ;; self = ptr %37
 20935|     ;; value = ptr %35
 20936|     ;; src = ptr %35
 20937|     ;; additional = i64 1
 20938|     ;; needed_extra_cap = i64 1
 20939|     ;; needed_extra_cap = i64 1
 20940|     ;; strategy = i8 1
 20941|  %771 = load i64, ptr %43, , !!35134, !!8                                                                              ;L1428<142
 20942|     ;; self = ptr %37
 20943|  %772 = load i64, ptr %42, , !!35134, !!8                                                                              ;L149<1428<142
 20944|  %773 = icmp eq i64 %771, %772                                                                                         ;L1428<142
 20945|  br i1 %773, label %774, label %779                                                                                    ;L1428<142
 20946| 
 20947| 774: ; preds = %769
 20948|     ;; self = ptr %37
 20949|     ;; self = ptr %37
 20950|     ;; self = ptr %37
 20951|     ;; used_cap = i64 %771
 20952|     ;; used_cap = i64 %771
 20953|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %37, i64 %771, i64 1, i1 zeroext true)
 20954|  to label %775 unwind label %777, !!35134                                                                              ;L619<430<738<1429<142
 20955| 
 20956| 775: ; preds = %774
 20957|  %776 = load i64, ptr %43, , !!35134                                                                                   ;L1432<142
 20958|  br label %779                                                                                                         ;L619<430<738<1429<142
 20959| 
 20960| 777: ; preds = %774
 20961|  %778 = cleanuppad within none []
 20962|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %35) #34 [ "funclet"(token %778) ] ;L1436<142
 20963|  cleanupret from %778 unwind label %48
 20964| 
 20965| 779: ; preds = %775, %769
 20966|  %780 = phi i64 [ %776, %775 ], [ %771, %769 ]                                                                         ;L1434<142
 20967|     ;; self = ptr %37
 20968|  %781 = load ptr, ptr %37, , !!35134, !!8, !!8                                                                         ;L138<1432<142
 20969|     ;; self = ptr %781
 20970|     ;; count = i64 %780
 20971|  %782 = gepS %781, i64 %780                                                                                            ;L961<1432<142
 20972|     ;; end = ptr %782
 20973|     ;; dst = ptr %782
 20974|  call void @llvm.memcpy.p0.p0.i64(ptr %782, ptr %35, i64 184, i1 false)                                                ;L1933<1433<142
 20975|  %783 = add i64 %780, 1                                                                                                ;L1434<142
 20976|  store i64 %783, ptr %43, , !!35134                                                                                    ;L1434<142
 20978|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %37, i64 32, i1 false)                                                   ;L143
 20980|  br label %768                                                                                                         ;L156
 20981| }
