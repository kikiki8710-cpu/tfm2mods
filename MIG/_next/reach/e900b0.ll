 38800| define void @_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster9get_input(ptr sret([64 x i8]) %0, ptr %1, ptr %2, ptr %3, ptr %4) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 38801|  %6 = alloca [24 x i8],
 38802|  %7 = alloca [32 x i8],
 38803|  %8 = alloca [320 x i8],
 38804|  %9 = alloca [320 x i8],
 38805|  %10 = alloca [40 x i8],
 38806|  %11 = alloca [320 x i8],
 38807|  %12 = alloca [320 x i8],
 38808|  %13 = alloca [320 x i8],
 38809|  %14 = alloca [16 x i8],
 38810|  %15 = alloca [24 x i8],
 38811|  %16 = alloca [24 x i8],
 38812|  %17 = alloca [24 x i8],
 38813|  %18 = alloca [16 x i8],
 38814|  %19 = alloca [24 x i8],
 38815|  %20 = alloca [24 x i8],
 38816|  %21 = alloca [24 x i8],
 38817|  %22 = alloca [24 x i8],
 38818|  %23 = alloca [24 x i8],
 38819|  %24 = alloca [208 x i8],
 38820|  %25 = alloca [320 x i8],
 38821|  %26 = alloca [24 x i8],
 38822|  %27 = alloca [320 x i8],
 38823|  %28 = alloca [96 x i8],
 38824|  %29 = alloca [16 x i8],
 38825|  %30 = alloca [24 x i8],
 38826|  %31 = alloca [24 x i8],
 38827|  %32 = alloca [24 x i8],
 38828|  %33 = alloca [24 x i8],
 38829|  %34 = alloca [16 x i8],
 38830|  %35 = alloca [24 x i8],
 38831|  %36 = alloca [24 x i8],
 38832|  %37 = alloca [24 x i8],
 38833|  %38 = alloca [24 x i8],
 38834|  %39 = alloca [24 x i8],
 38835|  %40 = alloca [56 x i8],
 38836|  %41 = alloca [8 x i8],
 38837|  %42 = alloca [1 x i8],
 38838|  %43 = alloca [320 x i8],
 38839|  %44 = alloca [24 x i8],
 38840|  %45 = alloca [320 x i8],
 38841|  %46 = alloca [24 x i8],
 38842|  %47 = alloca [32 x i8],
 38843|  %48 = alloca [136 x i8],
 38844|  %49 = alloca [177 x i8],
 38845|  %50 = alloca [24 x i8],
 38846|  %51 = alloca [32 x i8],
 38847|  %52 = alloca [24 x i8],
 38848|     ;; self = ptr %1
 38849|     ;; rnd = ptr %2
 38850|     ;; self = ptr %2
 38851|     ;; self = ptr %2
 38852|     ;; self = ptr %2
 38853|     ;; self = ptr %2
 38854|     ;; self = ptr %2
 38855|     ;; player = ptr %3
 38856|     ;; data = ptr %4
 38857|     ;; turn_event = ptr %51
 38858|     ;; _t_sai = ptr %50
 38859|     ;; input = ptr %47
 38860|     ;; _x = ptr %46
 38861|     ;; rd = ptr %42
 38862|     ;; threat = ptr %41
 38863|     ;; key = ptr %39
 38864|     ;; value = ptr %39
 38865|     ;; key = ptr %39
 38866|     ;; value = ptr %35
 38867|     ;; value = ptr %31
 38868|     ;; value = ptr %24
 38869|     ;; value = ptr %19
 38870|     ;; value = ptr %16
 38871|     ;; raw = ptr %10
 38874|     ;; phase = i64 26
 38875|     ;; order = i8 0
 38876|     ;; len = i64 5
 38877|     ;; count = i64 5
 38878|     ;; val = i64 1
 38879|     ;; order = i8 0
 38880|     ;; val = i64 1
 38881|     ;; order = i8 0
 38882|     ;; count = i64 5
 38883|     ;; count = i64 5
 38884|     ;; team = i64 1
 38885|  %53 = load ptr, ptr %4, , !!8, !!8                                                                                    ;L844
 38886|     ;; self = ptr %53
 38887|     ;; self = ptr %53
 38888|     ;; self = ptr %53
 38889|  %54 = load ptr, ptr %53, , !!8, !!8                                                                                   ;L844
 38890|  %55 = gep %53, i64 8                                                                                                  ;L844
 38891|  %56 = load ptr, ptr %55, , !!8, !!8                                                                                   ;L844
 38892|  %57 = gep %56, i64 40                                                                                                 ;L844
 38893|  %58 = load ptr, ptr %57, , !!8                                                                                        ;L844
 38894|  %59 = tail call i64 %58(ptr %54)                                                                                      ;L844
 38895|  %60 = gep %1, i64 10560                                                                                               ;L844
 38896|  %61 = load i64, ptr %60, , !!8                                                                                        ;L844
 38897|  %62 = icmp ult i64 %59, %61                                                                                           ;L844
 38898|  br i1 %62, label %63, label %72                                                                                       ;L844
 38899| 
 38900| 63: ; preds = %5
 38901|  %64 = gep %4, i64 8                                                                                                   ;L848
 38902|  %65 = load ptr, ptr %64, , !!8, !!8                                                                                   ;L848
 38903|  %66 = load ptr, ptr %65, , !!8, !!8                                                                                   ;L848
 38904|  store i64 -1, ptr %0,                                                                                                 ;L848
 38905|  %67 = gep %0, i64 32                                                                                                  ;L848
 38906|  store ptr inttoptr (i64 8 to ptr), ptr %67,                                                                           ;L848
 38907|  %68 = gep %0, i64 40                                                                                                  ;L848
 38908|  store ptr %66, ptr %68,                                                                                               ;L848
 38909|  %69 = gep %0, i64 48                                                                                                  ;L848
 38910|  call void @llvm.memset.p0.i64(ptr %69, i8 0, i64 16, i1 false)                                                        ;L848
 38911|  br label %1330                                                                                                        ;L1083
 38912| 
 38913| 70: ; preds = %1332, %1331, %84
 38914|  %71 = cleanuppad within none []
 38915|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtNtCs97f5S1uJLkH_9game_core10simulation12ai_interface9TurnEventEECshdEBA0ozCnw_7game_ai(ptr %51) #34 [ "funclet"(token %71) ] ;L1083
 38916|  cleanupret from %71 unwind to caller                                                                                  ;L843
 38917| 
 38918| 72: ; preds = %5
 38920|  %73 = gep %3, i64 384                                                                                                 ;L851
 38921|  %74 = tail call i64 @gc::simulation5state6playerNtB4_16AthleteParameter15input_delay_min(ptr %73)                     ;L851
 38922|     ;; start = i64 %74
 38923|  %75 = tail call i64 @gc::simulation5state6playerNtB4_16AthleteParameter15input_delay_max(ptr %73)                     ;L851
 38924|     ;; end = i64 %75
 38925|  store i64 %74, ptr %52,                                                                                               ;L391<851
 38926|  %76 = gep %52, i64 8                                                                                                  ;L391<851
 38927|  store i64 %75, ptr %76,                                                                                               ;L391<851
 38928|  %77 = gep %52, i64 16                                                                                                 ;L391<851
 38929|  store i8 0, ptr %77,                                                                                                  ;L391<851
 38930|  %78 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %2, ptr %52)                             ;L851
 38932|  %79 = udiv i64 %78, 100                                                                                               ;L851
 38933|     ;; delay = i64 %79
 38934|  %80 = call i64 %58(ptr %54)                                                                                           ;L852
 38935|  %81 = add i64 %80, %79                                                                                                ;L852
 38936|  store i64 %81, ptr %60,                                                                                               ;L852
 38938|  call void @_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster12update_state(ptr sret([32 x i8]) %51, ptr %1, ptr %2, ptr %3, ptr %4) ;L854
 38940|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 38941|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 38942|     ;; order = i8 0
 38943|  %82 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                     ;L3904<741<176<855
 38944|  %83 = icmp eq i8 %82, 0                                                                                               ;L176<855
 38945|  br i1 %83, label %86, label %84                                                                                       ;L176<855
 38946| 
 38947| 84: ; preds = %72
 38948|  %85 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()
 38949|  to label %93 unwind label %70                                                                                         ;L179<855
 38950| 
 38951| 86: ; preds = %93, %72
 38952|  %87 = phi i32 [ %95, %93 ], [ -1, %72 ]
 38953|  %88 = gep %50, i64 16                                                                                                 ;L0<855
 38954|  store i32 %87, ptr %88,                                                                                               ;L0<855
 38955|  %89 = gep %1, i64 10328                                                                                               ;L856
 38956|  %90 = gep %1, i64 10512                                                                                               ;L856
 38957|  %91 = load i64, ptr %90, , !!8                                                                                        ;L856
 38958|  %92 = gep %1, i64 7568                                                                                                ;L856
 38959|  invoke void @ai::small_actionNtB2_15SmallActionPlay9get_input(ptr sret([32 x i8]) %47, ptr %89, i64 %91, ptr %2, ptr %3, ptr %4, ptr %92, ptr %1)
 38960|  to label %100 unwind label %97                                                                                        ;L856
 38961| 
 38962| 93: ; preds = %84
 38963|  %94 = extractvalue { i64, i32 } %85, 0                                                                                ;L179<855
 38964|  %95 = extractvalue { i64, i32 } %85, 1                                                                                ;L179<855
 38965|  store i64 26, ptr %50,                                                                                                ;L179<855
 38966|  %96 = gep %50, i64 8                                                                                                  ;L179<855
 38967|  store i64 %94, ptr %96,                                                                                               ;L179<855
 38968|  br label %86                                                                                                          ;L180<855
 38969| 
 38970| 97: ; preds = %1186, %1133, %1130, %1109, %1107, %657, %654, %647, %645, %635, %630, %603, %563, %435, %417, %391, %384, %383, %359, %354, %340, %338, %326, %319, %317, %307, %305, %245, %235, %232, %215, %206, %196, %157, %148, %147, %142, %130, %129, %115, %113, %86
 38971|  %98 = phi i1 [ false, %657 ], [ false, %1133 ], [ false, %338 ], [ false, %326 ], [ false, %383 ], [ true, %147 ], [ false, %384 ], [ false, %319 ], [ false, %359 ], [ false, %317 ], [ false, %1109 ], [ false, %1186 ], [ false, %1130 ], [ false, %307 ], [ false, %305 ], [ false, %1107 ], [ false, %654 ], [ false, %630 ], [ false, %647 ], [ false, %645 ], [ false, %635 ], [ false, %603 ], [ false, %354 ], [ true, %142 ], [ false, %245 ], [ false, %340 ], [ false, %563 ], [ true, %130 ], [ false, %232 ], [ false, %215 ], [ true, %148 ], [ false, %115 ], [ true, %206 ], [ false, %391 ], [ true, %196 ], [ true, %129 ], [ false, %113 ], [ true, %86 ], [ false, %435 ], [ true, %157 ], [ false, %417 ], [ false, %235 ] ;L0
 38972|  %99 = cleanuppad within none []
 38973|  br i1 %98, label %1332, label %1331                                                                                   ;L1083
 38974| 
 38975| 100: ; preds = %86
 38976|  %101 = load i64, ptr %90, , !!8                                                                                       ;L865
 38977|  %102 = icmp ugt i64 %101, 1                                                                                           ;L865
 38978|     ;; self = ptr %47
 38979|     ;; self = ptr %47
 38980|  %103 = load i64, ptr %47,
 38981|  %104 = icmp eq i64 %103, -1
 38982|  %105 = select i1 %102, i1 %104, i1 false                                                                              ;L865
 38983|  br i1 %105, label %129, label %106                                                                                    ;L865
 38984| 
 38985| 106: ; preds = %206, %200, %193, %159, %150, %135, %132, %100
 38987|  call void @llvm.memcpy.p0.p0.i64(ptr %46, ptr %50, i64 24, i1 false)                                                  ;L906
 38990|  %107 = gep %46, i64 16                                                                                                ;L825<1004<906
 38991|  %108 = load i32, ptr %107, , !!8                                                                                      ;L825<1004<906
 38992|  %109 = icmp eq i32 %108, -1                                                                                           ;L825<1004<906
 38993|  br i1 %109, label %209, label %110                                                                                    ;L825<1004<906
 38994| 
 38995| 110: ; preds = %106
 38999|     ;; self = ptr %46
 39000|     ;; order = i8 0
 39001|     ;; order = i8 0
 39002|     ;; val = i64 1
 39003|     ;; order = i8 0
 39004|     ;; val = i64 1
 39005|     ;; order = i8 0
 39006|  %111 = load i64, ptr %46, , !!8                                                                                       ;L185<825<825<1004<906
 39007|  %112 = icmp ult i64 %111, 132                                                                                         ;L185<825<825<1004<906
 39008|  br i1 %112, label %115, label %113                                                                                    ;L185<825<825<1004<906
 39009| 
 39010| 113: ; preds = %110
 39011|  invoke void @core::panicking18panic_bounds_check(i64 %111, i64 132, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.230) #35
 39012|  to label %114 unwind label %97                                                                                        ;L185<825<825<1004<906
 39013| 
 39014| 114: ; preds = %113
 39015|  unreachable                                                                                                           ;L185<825<825<1004<906
 39016| 
 39017| 115: ; preds = %110
 39018|     ;; self = !DIArgList(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 %111)
 39019|  %116 = gep %46, i64 8                                                                                                 ;L185<825<825<1004<906
 39020|  %117 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %116)
 39021|  to label %118 unwind label %97                                                                                        ;L185<825<825<1004<906
 39022| 
 39023| 118: ; preds = %115
 39024|  %119 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %111                                 ;L185<825<825<1004<906
 39025|     ;; self = ptr %119
 39026|  %120 = extractvalue { i64, i32 } %117, 0                                                                              ;L185<825<825<1004<906
 39027|  %121 = extractvalue { i64, i32 } %117, 1                                                                              ;L185<825<825<1004<906
 39029|  %122 = mul i64 %120, 1000000000                                                                                       ;L632<185<825<825<1004<906
 39030|  %123 = icmp ult i32 %121, 1000000000                                                                                  ;L49<632<185<825<825<1004<906
 39031|  call void @llvm.assume(i1 %123)                                                                                       ;L49<632<185<825<825<1004<906
 39032|  %124 = zext nneg i32 %121 to i64                                                                                      ;L632<185<825<825<1004<906
 39033|  %125 = add i64 %122, %124                                                                                             ;L632<185<825<825<1004<906
 39034|     ;; val = i64 %125
 39035|     ;; val = i64 %125
 39036|     ;; dst = ptr %119
 39037|  %126 = atomicrmw add ptr %119, i64 %125 monotonic, , !!52006                                                          ;L3937<3162<185<825<825<1004<906
 39038|  %127 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %111                                 ;L186<825<825<1004<906
 39039|     ;; self = ptr %127
 39040|     ;; dst = ptr %127
 39041|  %128 = atomicrmw add ptr %127, i64 1 monotonic, , !!52006                                                             ;L3937<3162<186<825<825<1004<906
 39042|  br label %209                                                                                                         ;L825<1004<906
 39043| 
 39044| 129: ; preds = %100
 39045|  invoke fastcc void @_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster19update_small_action(ptr %1, ptr %2, ptr %3, ptr %4)
 39046|  to label %130 unwind label %97                                                                                        ;L869
 39047| 
 39048| 130: ; preds = %129
 39049|  %131 = load i64, ptr %90, , !!8                                                                                       ;L870
 39050|  invoke void @ai::small_actionNtB2_15SmallActionPlay9get_input(ptr sret([32 x i8]) %47, ptr %89, i64 %131, ptr %2, ptr %3, ptr %4, ptr %92, ptr %1)
 39051|  to label %132 unwind label %97                                                                                        ;L870
 39052| 
 39053| 132: ; preds = %130
 39054|  %133 = load i64, ptr %90, , !!8                                                                                       ;L876
 39055|  %134 = icmp ugt i64 %133, 1                                                                                           ;L876
 39056|  br i1 %134, label %135, label %106                                                                                    ;L876
 39057| 
 39058| 135: ; preds = %132
 39059|     ;; self = ptr %47
 39060|     ;; self = ptr %47
 39061|  %136 = load i64, ptr %47, , !!8                                                                                       ;L633<682<876
 39062|  %137 = icmp eq i64 %136, -1                                                                                           ;L633<682<876
 39063|  %138 = gep %1, i64 10698                                                                                              ;L876
 39064|  %139 = load i8, ptr %138,                                                                                             ;L876
 39065|  %140 = trunc nuw i8 %139 to i1                                                                                        ;L876
 39066|  %141 = select i1 %137, i1 %140, i1 false                                                                              ;L876
 39067|  br i1 %141, label %142, label %106                                                                                    ;L876
 39068| 
 39069| 142: ; preds = %135
 39070|  %143 = gep %1, i64 1328                                                                                               ;L877
 39071|  %144 = gep %1, i64 7008                                                                                               ;L877
 39072|  %145 = load i64, ptr %144, , !!8                                                                                      ;L877
 39073|  %146 = add i64 %145, 1                                                                                                ;L877
 39074|  store i64 %146, ptr %144,                                                                                             ;L877
 39075|  invoke void @ai::plan_legacy7handlerNtB2_17LegacyPlanHandler23v3_fall_back_to_passive(ptr %143, i64 %133, ptr %2, ptr %3, ptr %4, ptr %1)
 39076|  to label %147 unwind label %97                                                                                        ;L878
 39077| 
 39078| 147: ; preds = %142
 39079|  invoke fastcc void @_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster19update_small_action(ptr %1, ptr %2, ptr %3, ptr %4)
 39080|  to label %148 unwind label %97                                                                                        ;L879
 39081| 
 39082| 148: ; preds = %147
 39083|  %149 = load i64, ptr %90, , !!8                                                                                       ;L880
 39084|  invoke void @ai::small_actionNtB2_15SmallActionPlay9get_input(ptr sret([32 x i8]) %47, ptr %89, i64 %149, ptr %2, ptr %3, ptr %4, ptr %92, ptr %1)
 39085|  to label %150 unwind label %97                                                                                        ;L880
 39086| 
 39087| 150: ; preds = %148
 39088|     ;; self = ptr %47
 39089|     ;; self = ptr %47
 39090|  %151 = load i64, ptr %47, , !!8                                                                                       ;L633<682<888
 39091|  %152 = icmp eq i64 %151, -1                                                                                           ;L633<682<888
 39092|  br i1 %152, label %153, label %106                                                                                    ;L888
 39093| 
 39094| 153: ; preds = %150
 39095|  %154 = gep %3, i64 2352                                                                                               ;L889
 39096|  %155 = load i64, ptr %154, , !!8                                                                                      ;L889
 39097|     ;; team = i64 %155
 39098|  %156 = icmp ult i64 %155, 2                                                                                           ;L889
 39099|  br i1 %156, label %159, label %157                                                                                    ;L889
 39100| 
 39101| 157: ; preds = %153
 39102|  invoke void @core::panicking18panic_bounds_check(i64 %155, i64 2, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.156) #35
 39103|  to label %158 unwind label %97                                                                                        ;L889
 39104| 
 39105| 158: ; preds = %232, %157
 39106|  unreachable
 39107| 
 39108| 159: ; preds = %153
 39109|     ;; self = ptr %3
 39110|  %160 = gep %3, i64 2496                                                                                               ;L581<889
 39111|  %161 = load i32, ptr %160, , !!8                                                                                      ;L581<889
 39112|  %162 = zext nneg i32 %161 to i64                                                                                      ;L581<889
 39113|  %163 = gep %53, i64 480                                                                                               ;L889
 39114|  %164 = getelementptr [5 x ptr], ptr %163, i64 %155                                                                    ;L889
 39115|  %165 = getelementptr ptr, ptr %164, i64 %162                                                                          ;L889
 39116|  %166 = load ptr, ptr %165, , !!8                                                                                      ;L889
 39117|  %167 = icmp eq ptr %166, null                                                                                         ;L889
 39118|  br i1 %167, label %106, label %168                                                                                    ;L889
 39119| 
 39120| 168: ; preds = %159
 39121|     ;; champ = ptr %166
 39123|  %169 = gep %4, i64 8                                                                                                  ;L890
 39124|  %170 = load ptr, ptr %169, , !!8, !!8                                                                                 ;L890
 39125|  %171 = gep %170, i64 32                                                                                               ;L890
 39126|  %172 = load ptr, ptr %171, , !!8, !!8                                                                                 ;L890
 39127|     ;; self = ptr %172
 39128|  %173 = gep %172, i64 28016                                                                                            ;L235<890
 39129|  %174 = gepS %173, i64 %155                                                                                            ;L235<890
 39130|  %175 = load i64, ptr %174, , !!8                                                                                      ;L235<890
 39131|     ;; flx = i64 %175
 39133|  %176 = gep %174, i64 16                                                                                               ;L235<890
 39134|  %177 = load i64, ptr %176, , !!8                                                                                      ;L235<890
 39135|     ;; frx = i64 %177
 39137|  %178 = gep %166, i64 1632                                                                                             ;L895
 39138|  %179 = load i64, ptr %178, , !!8                                                                                      ;L895
 39139|  %180 = icmp uge i64 %179, %175                                                                                        ;L895
 39140|  %181 = icmp ule i64 %179, %177                                                                                        ;L895
 39141|  %182 = and i1 %180, %181                                                                                              ;L895
 39142|  br i1 %182, label %183, label %196                                                                                    ;L895
 39143| 
 39144| 183: ; preds = %168
 39145|  %184 = gep %174, i64 24                                                                                               ;L235<890
 39146|  %185 = load i64, ptr %184, , !!8                                                                                      ;L235<890
 39147|     ;; fry = i64 %185
 39148|  %186 = gep %174, i64 8                                                                                                ;L235<890
 39149|  %187 = load i64, ptr %186, , !!8                                                                                      ;L235<890
 39150|     ;; fly = i64 %187
 39151|  %188 = gep %166, i64 1640                                                                                             ;L895
 39152|  %189 = load i64, ptr %188, , !!8                                                                                      ;L895
 39153|  %190 = icmp uge i64 %189, %187                                                                                        ;L895
 39154|  %191 = icmp ule i64 %189, %185                                                                                        ;L895
 39155|  %192 = and i1 %190, %191                                                                                              ;L895
 39156|  br i1 %192, label %193, label %196                                                                                    ;L895
 39157| 
 39158| 193: ; preds = %183
 39159|  %194 = load i64, ptr %90, , !!8                                                                                       ;L896
 39160|  %195 = icmp ugt i64 %194, 1                                                                                           ;L896
 39161|  br i1 %195, label %200, label %106                                                                                    ;L896
 39162| 
 39163| 196: ; preds = %200, %183, %168
 39164|  %197 = gep %1, i64 7016                                                                                               ;L898
 39165|  %198 = load i64, ptr %197, , !!8                                                                                      ;L898
 39166|  %199 = add i64 %198, 1                                                                                                ;L898
 39167|  store i64 %199, ptr %197,                                                                                             ;L898
 39170|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway3new(ptr sret([136 x i8]) %48, ptr %4, ptr %3, i64 5)
 39171|  to label %206 unwind label %97                                                                                        ;L899
 39172| 
 39173| 200: ; preds = %193
 39174|  %201 = gep %166, i64 1648                                                                                             ;L896
 39175|  %202 = load i64, ptr %201, , !!8                                                                                      ;L896
 39176|  %203 = gep %166, i64 1576                                                                                             ;L896
 39177|  %204 = load i64, ptr %203, , !!8                                                                                      ;L896
 39178|  %205 = icmp ult i64 %202, %204                                                                                        ;L896
 39179|     ;; in_fountain = i1 %205
 39180|  br i1 %205, label %106, label %196                                                                                    ;L897
 39181| 
 39182| 206: ; preds = %196
 39183|  call void @llvm.memcpy.p0.p0.i64(ptr %49, ptr %48, i64 136, i1 false)                                                 ;L899
 39185|  call void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %89)                  ;L899
 39186|  call void @llvm.memcpy.p0.p0.i64(ptr %89, ptr %49, i64 177, i1 false)                                                 ;L899
 39187|  %207 = gep %1, i64 10505                                                                                              ;L899
 39188|  store i8 3, ptr %207,                                                                                                 ;L899
 39190|  %208 = load i64, ptr %90, , !!8                                                                                       ;L900
 39191|  invoke void @ai::small_actionNtB2_15SmallActionPlay9get_input(ptr sret([32 x i8]) %47, ptr %89, i64 %208, ptr %2, ptr %3, ptr %4, ptr %92, ptr %1)
 39192|  to label %106 unwind label %97                                                                                        ;L900
 39193| 
 39194| 209: ; preds = %118, %106
 39196|  %210 = gep %1, i64 10505                                                                                              ;L909
 39197|  %211 = load i8, ptr %210, , !!8                                                                                       ;L909
 39198|  %212 = icmp ne i8 %211, 10                                                                                            ;L909
 39199|  call void @llvm.assume(i1 %212)                                                                                       ;L909
 39200|  %213 = add nsw i8 %211, -15                                                                                           ;L909
 39201|  %214 = icmp ult i8 %213, 4                                                                                            ;L909
 39202|  br i1 %214, label %215, label %219                                                                                    ;L909
 39203| 
 39204| 215: ; preds = %209
 39205|  %216 = invoke i64 %58(ptr %54)
 39206|  to label %217 unwind label %97                                                                                        ;L911
 39207| 
 39208| 217: ; preds = %215
 39209|  %218 = gep %1, i64 10656                                                                                              ;L911
 39210|  store i64 %216, ptr %218,                                                                                             ;L911
 39211|  br label %219                                                                                                         ;L909
 39212| 
 39213| 219: ; preds = %217, %209
 39214|  %220 = gep %3, i64 2352                                                                                               ;L914
 39215|  %221 = load i64, ptr %220, , !!8                                                                                      ;L914
 39216|     ;; team = i64 %221
 39217|     ;; team = i64 %221
 39218|     ;; team = i64 %221
 39219|     ;; team = i64 %221
 39220|     ;; team = i64 %221
 39221|  %222 = icmp ult i64 %221, 2                                                                                           ;L914
 39222|  br i1 %222, label %223, label %232                                                                                    ;L914
 39223| 
 39224| 223: ; preds = %219
 39225|     ;; self = ptr %3
 39226|  %224 = gep %3, i64 2496                                                                                               ;L581<914
 39227|  %225 = load i32, ptr %224, , !!8                                                                                      ;L581<914
 39228|  %226 = zext nneg i32 %225 to i64                                                                                      ;L581<914
 39229|  %227 = gep %53, i64 480                                                                                               ;L914
 39230|  %228 = getelementptr [5 x ptr], ptr %227, i64 %221                                                                    ;L914
 39231|  %229 = getelementptr ptr, ptr %228, i64 %226                                                                          ;L914
 39232|  %230 = load ptr, ptr %229, , !!8                                                                                      ;L914
 39233|  %231 = icmp ne ptr %230, null                                                                                         ;L914
 39234|  br i1 %231, label %233, label %235                                                                                    ;L914
 39235| 
 39236| 232: ; preds = %219
 39237|  invoke void @core::panicking18panic_bounds_check(i64 %221, i64 2, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.157) #35
 39238|  to label %158 unwind label %97                                                                                        ;L914
 39239| 
 39240| 233: ; preds = %223
 39241|     ;; champ = ptr %230
 39242|  %234 = load i64, ptr %47, , !!8                                                                                       ;L915
 39243|  switch i64 %234, label %276 [
 39244|  i64 -1, label %239
 39245|  i64 0, label %245
 39246|  ]                                                                                                                     ;L915
 39247| 
 39248| 235: ; preds = %404, %323, %314, %295, %280, %276, %257, %239, %223
 39249|  %236 = gep %56, i64 64                                                                                                ;L948
 39250|  %237 = load ptr, ptr %236, , !!8                                                                                      ;L948
 39251|  %238 = invoke { i64, ptr } %237(ptr %54)
 39252|  to label %409 unwind label %97                                                                                        ;L948
 39253| 
 39254| 239: ; preds = %276, %257, %233
 39255|  %240 = gep %230, i64 1648                                                                                             ;L923
 39256|  %241 = load i64, ptr %240, , !!8                                                                                      ;L923
 39257|  %242 = gep %230, i64 1576                                                                                             ;L923
 39258|  %243 = load i64, ptr %242, , !!8                                                                                      ;L923
 39259|  %244 = icmp ult i64 %241, %243                                                                                        ;L923
 39260|  br i1 %244, label %235, label %280                                                                                    ;L923
 39261| 
 39262| 245: ; preds = %233
 39263|  %246 = gep %47, i64 8                                                                                                 ;L917
 39264|  %247 = load i64, ptr %246, , !!8                                                                                      ;L917
 39265|     ;; x = i64 %247
 39266|  %248 = gep %47, i64 16                                                                                                ;L917
 39267|  %249 = load i64, ptr %248, , !!8                                                                                      ;L917
 39268|     ;; y = i64 %249
 39269|  %250 = gep %4, i64 8                                                                                                  ;L918
 39270|  %251 = load ptr, ptr %250, , !!8, !!8                                                                                 ;L918
 39271|  %252 = gep %251, i64 32                                                                                               ;L918
 39272|  %253 = load ptr, ptr %252, , !!8, !!8                                                                                 ;L918
 39273|  %254 = gep %251, i64 8                                                                                                ;L918
 39274|  %255 = load ptr, ptr %254, , !!8, !!8                                                                                 ;L918
 39275|  %256 = invoke { i64, i64 } @gc::simulation4gameNtB5_4Game15adjust_position(ptr %253, ptr %255, i64 %247, i64 %249)
 39276|  to label %257 unwind label %97                                                                                        ;L918
 39277| 
 39278| 257: ; preds = %245
 39279|  %258 = extractvalue { i64, i64 } %256, 0                                                                              ;L918
 39280|  %259 = extractvalue { i64, i64 } %256, 1                                                                              ;L918
 39281|     ;; ax = i64 %258
 39282|     ;; x2 = i64 %258
 39283|     ;; other = i64 %258
 39284|     ;; ay = i64 %259
 39285|     ;; y2 = i64 %259
 39286|     ;; other = i64 %259
 39287|  %260 = gep %230, i64 1632                                                                                             ;L919
 39288|  %261 = load i64, ptr %260, , !!8                                                                                      ;L919
 39289|     ;; x1 = i64 %261
 39290|     ;; self = i64 %261
 39291|  %262 = gep %230, i64 1640                                                                                             ;L919
 39292|  %263 = load i64, ptr %262, , !!8                                                                                      ;L919
 39293|     ;; y1 = i64 %263
 39294|     ;; self = i64 %263
 39295|  %264 = icmp ult i64 %261, %258                                                                                        ;L3147<7<919
 39296|  %265 = sub nuw i64 %258, %261                                                                                         ;L3147<7<919
 39297|  %266 = sub nuw i64 %261, %258                                                                                         ;L3147<7<919
 39298|  %267 = select i1 %264, i64 %265, i64 %266                                                                             ;L3147<7<919
 39299|     ;; dx = i64 %267
 39300|  %268 = icmp ult i64 %263, %259                                                                                        ;L3147<8<919
 39301|  %269 = sub nuw i64 %259, %263                                                                                         ;L3147<8<919
 39302|  %270 = sub nuw i64 %263, %259                                                                                         ;L3147<8<919
 39303|  %271 = select i1 %268, i64 %269, i64 %270                                                                             ;L3147<8<919
 39304|     ;; dy = i64 %271
 39305|  %272 = mul i64 %267, %267                                                                                             ;L9<919
 39306|  %273 = mul i64 %271, %271                                                                                             ;L9<919
 39307|  %274 = add i64 %273, %272                                                                                             ;L9<919
 39308|  %275 = icmp ult i64 %274, 4000001                                                                                     ;L919
 39309|     ;; stays = i1 %275
 39310|  br i1 %275, label %239, label %235                                                                                    ;L923
 39311| 
 39312| 276: ; preds = %233
 39313|  %277 = load i8, ptr %210, , !!8                                                                                       ;L921
 39314|  %278 = icmp ne i8 %277, 10                                                                                            ;L921
 39315|  call void @llvm.assume(i1 %278)                                                                                       ;L921
 39316|  %279 = icmp eq i8 %277, 19                                                                                            ;L921
 39317|     ;; stays = i1 %279
 39318|  br i1 %279, label %239, label %235                                                                                    ;L923
 39319| 
 39320| 280: ; preds = %239
 39321|  %281 = gep %4, i64 8                                                                                                  ;L924
 39322|  %282 = load ptr, ptr %281, , !!8, !!8                                                                                 ;L924
 39324|  %283 = gep %282, i64 32                                                                                               ;L924
 39325|  %284 = load ptr, ptr %283, , !!8, !!8                                                                                 ;L924
 39326|     ;; self = ptr %284
 39327|  %285 = gep %284, i64 28016                                                                                            ;L235<924
 39328|  %286 = gepS %285, i64 %221                                                                                            ;L235<924
 39329|  %287 = load i64, ptr %286, , !!8                                                                                      ;L235<924
 39330|     ;; flx = i64 %287
 39332|  %288 = gep %286, i64 16                                                                                               ;L235<924
 39333|  %289 = load i64, ptr %288, , !!8                                                                                      ;L235<924
 39334|     ;; frx = i64 %289
 39336|  %290 = gep %230, i64 1632                                                                                             ;L925
 39337|  %291 = load i64, ptr %290, , !!8                                                                                      ;L925
 39338|  %292 = icmp uge i64 %291, %287                                                                                        ;L925
 39339|  %293 = icmp ule i64 %291, %289                                                                                        ;L925
 39340|  %294 = and i1 %292, %293                                                                                              ;L925
 39341|  br i1 %294, label %295, label %235                                                                                    ;L925
 39342| 
 39343| 295: ; preds = %280
 39344|  %296 = gep %286, i64 24                                                                                               ;L235<924
 39345|  %297 = load i64, ptr %296, , !!8                                                                                      ;L235<924
 39346|     ;; fry = i64 %297
 39347|  %298 = gep %286, i64 8                                                                                                ;L235<924
 39348|  %299 = load i64, ptr %298, , !!8                                                                                      ;L235<924
 39349|     ;; fly = i64 %299
 39350|  %300 = gep %230, i64 1640                                                                                             ;L925
 39351|  %301 = load i64, ptr %300, , !!8                                                                                      ;L925
 39352|  %302 = icmp uge i64 %301, %299                                                                                        ;L925
 39353|  %303 = icmp ule i64 %301, %297                                                                                        ;L925
 39354|  %304 = and i1 %302, %303                                                                                              ;L925
 39355|  br i1 %304, label %305, label %235                                                                                    ;L925
 39356| 
 39357| 305: ; preds = %295
 39358|  %306 = load i64, ptr %90, , !!8                                                                                       ;L926
 39360|     ;; self = ptr %2
 39364|     ;; self = ptr %2
 39365|  invoke void @core::clone5Clone5cloneCshdEBA0ozCnw_7game_ai(ptr sret([256 x i8]) %13, ptr %2)
 39366|  to label %307 unwind label %97                                                                                        ;L124<148<33<926
 39367| 
 39368| 307: ; preds = %305
 39369|  %308 = gep %2, i64 256                                                                                                ;L125<148<33<926
 39370|  %309 = load i64, ptr %308, , !!52122, !!8                                                                             ;L125<148<33<926
 39371|  %310 = gep %2, i64 272                                                                                                ;L127<148<33<926
 39372|     ;; self = ptr %310
 39373|  %311 = gep %13, i64 272                                                                                               ;L115<148<33<926
 39374|  call void @llvm.memcpy.p0.p0.i64(ptr %311, ptr %310, i64 48, i1 false)                                                ;L73<127<148<33<926
 39375|  %312 = gep %13, i64 256                                                                                               ;L115<148<33<926
 39376|  store i64 %309, ptr %312, , !!52125                                                                                   ;L115<148<33<926
 39377|  call void @llvm.memcpy.p0.p0.i64(ptr %45, ptr %13, i64 320, i1 false)                                                 ;L148<33<926
 39379|  %313 = invoke { i64, i64 } @ai::buy_item(i64 poison, ptr %45, ptr %3, ptr poison, ptr poison, ptr %282)
 39380|  to label %314 unwind label %97                                                                                        ;L926
 39381| 
 39382| 314: ; preds = %307
 39383|  %315 = extractvalue { i64, i64 } %313, 0                                                                              ;L926
 39384|  %316 = icmp eq i64 %315, 1                                                                                            ;L926
 39386|  br i1 %316, label %235, label %317                                                                                    ;L926
 39387| 
 39388| 317: ; preds = %314
 39390|  %318 = load i64, ptr %90, , !!8                                                                                       ;L927
 39392|     ;; self = ptr %2
 39396|     ;; self = ptr %2
 39397|  invoke void @core::clone5Clone5cloneCshdEBA0ozCnw_7game_ai(ptr sret([256 x i8]) %12, ptr %2)
 39398|  to label %319 unwind label %97                                                                                        ;L124<148<33<927
 39399| 
 39400| 319: ; preds = %317
 39401|  %320 = load i64, ptr %308, , !!52155, !!8                                                                             ;L125<148<33<927
 39402|     ;; self = ptr %310
 39403|  %321 = gep %12, i64 272                                                                                               ;L115<148<33<927
 39404|  call void @llvm.memcpy.p0.p0.i64(ptr %321, ptr %310, i64 48, i1 false)                                                ;L73<127<148<33<927
 39405|  %322 = gep %12, i64 256                                                                                               ;L115<148<33<927
 39406|  store i64 %320, ptr %322, , !!52158                                                                                   ;L115<148<33<927
 39407|  call void @llvm.memcpy.p0.p0.i64(ptr %43, ptr %12, i64 320, i1 false)                                                 ;L148<33<927
 39409|  invoke void @ai::upgrade_item(ptr sret([24 x i8]) %44, i64 poison, ptr %43, ptr %3, ptr poison, ptr poison, ptr %282)
 39410|  to label %323 unwind label %97                                                                                        ;L927
 39411| 
 39412| 323: ; preds = %319
 39413|     ;; self = ptr %44
 39414|  %324 = load i64, ptr %44, , !!8                                                                                       ;L633<927
 39415|  %325 = icmp eq i64 %324, 0                                                                                            ;L430<927
 39416|     ;; can_buy = i1 %325
 39419|  br i1 %325, label %326, label %235                                                                                    ;L928
 39420| 
 39421| 326: ; preds = %323
 39422|  %327 = gep %1, i64 10664                                                                                              ;L929
 39423|  %328 = load i64, ptr %327, , !!8                                                                                      ;L929
 39424|  %329 = add i64 %328, 1                                                                                                ;L929
 39425|  store i64 %329, ptr %327,                                                                                             ;L929
 39426|  %330 = gep %1, i64 2840                                                                                               ;L930
 39427|  %331 = load i64, ptr %330, , !!8                                                                                      ;L930
 39428|  %332 = call fastcc i64 @ai::plan_legacy5typesNtB2_7BigPlan15freeze_plan_cls(i64 %331)                                 ;L930
 39429|  %333 = gep %1, i64 1232                                                                                               ;L930
 39430|  %334 = getelementptr i64, ptr %333, i64 %332                                                                          ;L930
 39431|  %335 = load i64, ptr %334, , !!8                                                                                      ;L930
 39432|  %336 = add i64 %335, 1                                                                                                ;L930
 39433|  store i64 %336, ptr %334,                                                                                             ;L930
 39435|  %337 = load i64, ptr %90, , !!8                                                                                       ;L932
 39436|     ;; self = ptr %2
 39438|     ;; self = ptr %2
 39439|  invoke void @core::clone5Clone5cloneCshdEBA0ozCnw_7game_ai(ptr sret([256 x i8]) %11, ptr %2)
 39440|  to label %338 unwind label %97                                                                                        ;L124<148<33<932
 39441| 
 39442| 338: ; preds = %326
 39443|     ;; self = ptr %310
 39445|  %339 = invoke fastcc zeroext i1 @ai::plan_legacy7handlerNtB2_17LegacyPlanHandler14v3_repair_done(i64 %337, i64 %221, i32 %225, ptr %53)
 39446|  to label %340 unwind label %97                                                                                        ;L932
 39447| 
 39448| 340: ; preds = %338
 39449|  %341 = zext i1 %339 to i8                                                                                             ;L932
 39450|  store i8 %341, ptr %42,                                                                                               ;L932
 39453|     ;; team = !DIArgList(i64 1, i64 %221)
 39454|  %342 = sub nuw nsw i64 1, %221                                                                                        ;L933
 39455|     ;; team = i64 %342
 39456|  %343 = getelementptr [5 x ptr], ptr %227, i64 %342                                                                    ;L1905<933
 39457|     ;; self[0..+8] = ptr %343
 39458|     ;; slice[0..+8] = ptr %343
 39459|     ;; self[8..+8] = i64 5
 39460|     ;; slice[8..+8] = i64 5
 39461|     ;; ptr = ptr %343
 39462|     ;; self = ptr %343
 39463|  %344 = gep %343, i64 40                                                                                               ;L961<100<1042<1905<933
 39464|     ;; self[0..+8] = ptr %343
 39465|     ;; self[8..+8] = ptr %344
 39466|  %345 = gep %4, i64 16                                                                                                 ;L934
 39467|  %346 = load ptr, ptr %345, , !!8, !!8                                                                                 ;L934
 39468|     ;; predicate[0..+8] = ptr %54
 39469|     ;; predicate[8..+8] = ptr %56
 39470|     ;; predicate[16..+8] = ptr %346
 39471|     ;; predicate[24..+8] = ptr %3
 39472|     ;; predicate[32..+8] = ptr %230
 39473|  store ptr %343, ptr %40,                                                                                              ;L28<957<934
 39474|  %347 = gep %40, i64 8                                                                                                 ;L28<957<934
 39475|  store ptr %344, ptr %347,                                                                                             ;L28<957<934
 39476|  %348 = gep %40, i64 16                                                                                                ;L28<957<934
 39477|  store ptr %54, ptr %348,                                                                                              ;L28<957<934
 39478|  %349 = gep %40, i64 24                                                                                                ;L28<957<934
 39479|  store ptr %56, ptr %349,                                                                                              ;L28<957<934
 39480|  %350 = gep %40, i64 32                                                                                                ;L28<957<934
 39481|  store ptr %346, ptr %350,                                                                                             ;L28<957<934
 39482|  %351 = gep %40, i64 40                                                                                                ;L28<957<934
 39483|  store ptr %3, ptr %351,                                                                                               ;L28<957<934
 39484|  %352 = gep %40, i64 48                                                                                                ;L28<957<934
 39485|  store ptr %230, ptr %352,                                                                                             ;L28<957<934
 39486|  %353 = invoke fastcc i64 @gc::simulation6entity6EntityEENCNvMs3_B2n_NtB2n_21AbstractGameWithCache14iter_champions0ENCNvMs1_CshdEBA0ozCnw_7game_aiNtB4k_15AgentVerHamster9get_inputs_0ENtNtNtB9_6traits8iterator8Iterator5countB4k_(ptr %40)
 39487|  to label %354 unwind label %97                                                                                        ;L935
 39488| 
 39489| 354: ; preds = %340
 39490|  store i64 %353, ptr %41,                                                                                              ;L933
 39492|     ;; args[0..+8] = ptr %42
 39493|  %355 = gep %1, i64 10698                                                                                              ;L936
 39494|     ;; args[8..+8] = ptr %355
 39495|     ;; args[16..+8] = ptr %41
 39497|  invoke void @ai::plan_legacy5typesNtB2_7BigPlan11debug_label(ptr sret([24 x i8]) %38, ptr %330)
 39498|  to label %356 unwind label %97                                                                                        ;L937
 39499| 
 39500| 356: ; preds = %354
 39501|     ;; args[24..+8] = ptr %38
 39504|  %357 = gep %1, i64 3224                                                                                               ;L938
 39505|     ;; args = ptr %357
 39507|  store ptr %357, ptr %34,                                                                                              ;L938
 39508|  %358 = gep %34, i64 8                                                                                                 ;L938
 39509|  store ptr @ai::plan_legacy8sub_planNtB5_7SubPlanNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %358,                      ;L938
 39510|     ;; args[0..+8] = ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.144
 39511|     ;; args[8..+8] = ptr %34
 39512|     ;; self[0..+8] = ptr null
 39513|     ;; self[8..+8] = i64 undef
 39517|  invoke void @_RNvNvNtCs9LexZzt9XJB_5alloc3fmt6format12format_inner(ptr sret([24 x i8]) %35, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.144, ptr %34)
 39518|  to label %361 unwind label %359                                                                                       ;L659<1275<659<938
 39519| 
 39520| 359: ; preds = %382, %364, %361, %356
 39521|  %360 = cleanuppad within none []
 39522|  call void @core::ptr9drop_glueNtNtCs9LexZzt9XJB_5alloc6string6StringECshdEBA0ozCnw_7game_ai(ptr %38) #34 [ "funclet"(token %360) ] ;L936
 39523|  cleanupret from %360 unwind label %97                                                                                 ;L936
 39524| 
 39525| 361: ; preds = %356
 39527|  call void @llvm.memcpy.p0.p0.i64(ptr %36, ptr %35, i64 24, i1 false)                                                  ;L938
 39528|  invoke fastcc void @_RNCNvMs1_CshdEBA0ozCnw_7game_aiNtB7_15AgentVerHamster9get_input0B7_(ptr %37, ptr %36)
 39529|  to label %362 unwind label %359                                                                                       ;L938
 39530| 
 39531| 362: ; preds = %361
 39533|     ;; args[32..+8] = ptr %37
 39537|  call fastcc void @ai::small_actionNtB2_15SmallActionPlay10get_action(ptr %30, ptr %89)                                ;L939
 39538|     ;; args = ptr %30
 39540|  store ptr %30, ptr %29,                                                                                               ;L939
 39541|  %363 = gep %29, i64 8                                                                                                 ;L939
 39542|  store ptr @gc::simulation4game10blackboardNtB5_11SmallActionNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %363,          ;L939
 39543|     ;; args[0..+8] = ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.144
 39544|     ;; args[8..+8] = ptr %29
 39545|     ;; self[0..+8] = ptr null
 39546|     ;; self[8..+8] = i64 undef
 39550|  invoke void @_RNvNvNtCs9LexZzt9XJB_5alloc3fmt6format12format_inner(ptr sret([24 x i8]) %31, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.144, ptr %29)
 39551|  to label %366 unwind label %364                                                                                       ;L659<1275<659<939
 39552| 
 39553| 364: ; preds = %381, %379, %366, %362
 39554|  %365 = cleanuppad within none []
 39555|  call void @core::ptr9drop_glueNtNtCs9LexZzt9XJB_5alloc6string6StringECshdEBA0ozCnw_7game_ai(ptr %37) #34 [ "funclet"(token %365) ] ;L936
 39556|  cleanupret from %365 unwind label %359                                                                                ;L936
 39557| 
 39558| 366: ; preds = %362
 39561|  call void @llvm.memcpy.p0.p0.i64(ptr %32, ptr %31, i64 24, i1 false)                                                  ;L939
 39562|  invoke fastcc void @_RNCNvMs1_CshdEBA0ozCnw_7game_aiNtB7_15AgentVerHamster9get_input0B7_(ptr %33, ptr %32)
 39563|  to label %367 unwind label %364                                                                                       ;L939
 39564| 
 39565| 367: ; preds = %366
 39567|     ;; args[40..+8] = ptr %33
 39569|  store ptr %42, ptr %28,                                                                                               ;L936
 39570|  %368 = gep %28, i64 8                                                                                                 ;L936
 39571|  store ptr @core::fmtbNtB5_7Display3fmt, ptr %368,                                                                     ;L936
 39572|  %369 = gep %28, i64 16                                                                                                ;L936
 39573|  store ptr %355, ptr %369,                                                                                             ;L936
 39574|  %370 = gep %28, i64 24                                                                                                ;L936
 39575|  store ptr @core::fmtbNtB5_7Display3fmt, ptr %370,                                                                     ;L936
 39576|  %371 = gep %28, i64 32                                                                                                ;L936
 39577|  store ptr %41, ptr %371,                                                                                              ;L936
 39578|  %372 = gep %28, i64 40                                                                                                ;L936
 39579|  store ptr @core::fmt3num3impjNtB9_7Display3fmt, ptr %372,                                                             ;L936
 39580|  %373 = gep %28, i64 48                                                                                                ;L936
 39581|  store ptr %38, ptr %373,                                                                                              ;L936
 39582|  %374 = gep %28, i64 56                                                                                                ;L936
 39583|  store ptr @core::fmt7Display3fmt, ptr %374,                                                                           ;L936
 39584|  %375 = gep %28, i64 64                                                                                                ;L936
 39585|  store ptr %37, ptr %375,                                                                                              ;L936
 39586|  %376 = gep %28, i64 72                                                                                                ;L936
 39587|  store ptr @core::fmt7Display3fmt, ptr %376,                                                                           ;L936
 39588|  %377 = gep %28, i64 80                                                                                                ;L936
 39589|  store ptr %33, ptr %377,                                                                                              ;L936
 39590|  %378 = gep %28, i64 88                                                                                                ;L936
 39591|  store ptr @core::fmt7Display3fmt, ptr %378,                                                                           ;L936
 39592|     ;; args[0..+8] = ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.155
 39593|     ;; args[8..+8] = ptr %28
 39594|     ;; self[0..+8] = ptr null
 39595|     ;; self[8..+8] = i64 undef
 39599|  invoke void @_RNvNvNtCs9LexZzt9XJB_5alloc3fmt6format12format_inner(ptr sret([24 x i8]) %39, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.155, ptr %28)
 39600|  to label %381 unwind label %379                                                                                       ;L659<1275<659<936
 39601| 
 39602| 379: ; preds = %367
 39603|  %380 = cleanuppad within none []
 39604|  call void @core::ptr9drop_glueNtNtCs9LexZzt9XJB_5alloc6string6StringECshdEBA0ozCnw_7game_ai(ptr %33) #34 [ "funclet"(token %380) ] ;L936
 39605|  cleanupret from %380 unwind label %364                                                                                ;L936
 39606| 
 39607| 381: ; preds = %367
 39609|  invoke void @core::ptr9drop_glueNtNtCs9LexZzt9XJB_5alloc6string6StringECshdEBA0ozCnw_7game_ai(ptr %33)
 39610|  to label %382 unwind label %364                                                                                       ;L936
 39611| 
 39612| 382: ; preds = %381
 39614|  invoke void @core::ptr9drop_glueNtNtCs9LexZzt9XJB_5alloc6string6StringECshdEBA0ozCnw_7game_ai(ptr %37)
 39615|  to label %383 unwind label %359                                                                                       ;L936
 39616| 
 39617| 383: ; preds = %382
 39619|  invoke void @core::ptr9drop_glueNtNtCs9LexZzt9XJB_5alloc6string6StringECshdEBA0ozCnw_7game_ai(ptr %38)
 39620|  to label %384 unwind label %97                                                                                        ;L936
 39621| 
 39622| 384: ; preds = %383
 39624|     ;; self = ptr %1
 39626|  %385 = gep %1, i64 672                                                                                                ;L1014<940
 39627|  invoke void @_RNvMNtCs5gUUnHMsxBL_9hashbrown11rustc_entryINtNtB4_3map7HashMapNtNtCs9LexZzt9XJB_5alloc6string6StringjNtNtNtCs9ec1k27omRZ_3std4hash6random11RandomStateE11rustc_entryCshdEBA0ozCnw_7game_ai(ptr sret([40 x i8]) %10, ptr %385, ptr %39)
 39628|  to label %386 unwind label %97                                                                                        ;L1014<940
 39629| 
 39630| 386: ; preds = %384
 39631|  %387 = load i64, ptr %10, , !!8                                                                                       ;L3008<1014<940
 39632|  %388 = icmp eq i64 %387, -1                                                                                           ;L3008<1014<940
 39633|  %389 = gep %10, i64 8                                                                                                 ;L0<1014<940
 39634|  %390 = load ptr, ptr %389,                                                                                            ;L0<1014<940
 39635|  br i1 %388, label %403, label %391                                                                                    ;L3008<1014<940
 39636| 
 39637| 391: ; preds = %386
 39638|  %392 = gep %10, i64 16                                                                                                ;L3010<1014<940
 39639|  %393 = load ptr, ptr %392,                                                                                            ;L3010<1014<940
 39640|  %394 = gep %10, i64 24                                                                                                ;L3010<1014<940
 39641|  %395 = load ptr, ptr %394, , !!8, !!8                                                                                 ;L3010<1014<940
 39642|  %396 = gep %10, i64 32                                                                                                ;L3010<1014<940
 39643|  %397 = load i64, ptr %396,                                                                                            ;L3010<1014<940
 39644|     ;; self[32..+8] = i64 %397
 39645|     ;; self[24..+8] = ptr %395
 39646|     ;; self[0..+8] = i64 %387
 39647|     ;; self[0..+8] = i64 %387
 39648|     ;; entry[0..+8] = i64 %387
 39649|     ;; self[0..+8] = i64 %387
 39650|     ;; self[8..+8] = ptr %390
 39651|     ;; self[8..+8] = ptr %390
 39652|     ;; entry[8..+8] = ptr %390
 39653|     ;; self[8..+8] = ptr %390
 39654|     ;; self[16..+8] = ptr %393
 39655|     ;; self[16..+8] = ptr %393
 39656|     ;; entry[16..+8] = ptr %393
 39657|     ;; self[16..+8] = ptr %393
 39659|     ;; default = i64 0
 39660|     ;; entry[32..+8] = i64 %397
 39661|     ;; self[32..+8] = i64 %397
 39662|     ;; self[32..+8] = i64 %397
 39663|     ;; entry[24..+8] = ptr %395
 39664|     ;; self[24..+8] = ptr %395
 39665|     ;; self[24..+8] = ptr %395
 39666|     ;; value = i64 0
 39667|     ;; value = i64 0
 39669|  store i64 %387, ptr %7,                                                                                               ;L576<2911<2519<940
 39670|  %398 = gep %7, i64 8                                                                                                  ;L576<2911<2519<940
 39671|  store ptr %390, ptr %398,                                                                                             ;L576<2911<2519<940
 39672|  %399 = gep %7, i64 16                                                                                                 ;L576<2911<2519<940
 39673|  store ptr %393, ptr %399,                                                                                             ;L576<2911<2519<940
 39674|  %400 = gep %7, i64 24                                                                                                 ;L576<2911<2519<940
 39675|  store i64 0, ptr %400, , !!52295                                                                                      ;L576<2911<2519<940
 39676|  %401 = invoke ptr @_RNvMs6_NtCs5gUUnHMsxBL_9hashbrown3rawINtB5_8RawTableTNtNtCs9LexZzt9XJB_5alloc6string6StringjEE14insert_no_growCshdEBA0ozCnw_7game_ai(ptr %395, i64 %397, ptr %7)
 39677|  to label %402 unwind label %97                                                                                        ;L576<2911<2519<940
 39678| 
 39679| 402: ; preds = %391
 39681|  br label %404                                                                                                         ;L2521<940
 39682| 
 39683| 403: ; preds = %386
 39686|     ;; self[0..+8] = i64 -1
 39687|     ;; self[0..+8] = i64 -1
 39688|     ;; entry[0..+8] = i64 -1
 39689|     ;; self[0..+8] = i64 -1
 39690|     ;; self[8..+8] = ptr %390
 39691|     ;; self[8..+8] = ptr %390
 39692|     ;; entry[8..+8] = ptr %390
 39693|     ;; self[8..+8] = ptr %390
 39699|     ;; default = i64 0
 39700|  br label %404                                                                                                         ;L2521<940
 39701| 
 39702| 404: ; preds = %403, %402
 39703|  %405 = phi ptr [ %401, %402 ], [ %390, %403 ]
 39704|  %406 = gep %405, i64 -8                                                                                               ;L0<940
 39705|  %407 = load i64, ptr %406, , !!8                                                                                      ;L940
 39706|  %408 = add i64 %407, 1                                                                                                ;L940
 39707|  store i64 %408, ptr %406,                                                                                             ;L940
 39710|  br label %235                                                                                                         ;L928
 39711| 
 39712| 409: ; preds = %235
 39713|  %410 = extractvalue { i64, ptr } %238, 0                                                                              ;L948
 39714|  %411 = icmp eq i64 %410, 2                                                                                            ;L948
 39715|  br i1 %411, label %412, label %421                                                                                    ;L948
 39716| 
 39717| 412: ; preds = %409
 39718|  %413 = gep %1, i64 2840                                                                                               ;L949
 39719|  %414 = load i64, ptr %413, , !!8                                                                                      ;L949
 39720|  %415 = icmp ne i64 %414, 6                                                                                            ;L949
 39721|  call void @llvm.assume(i1 %415)                                                                                       ;L949
 39722|  %416 = icmp samesign ult i64 %414, 2                                                                                  ;L949
 39723|  br i1 %416, label %417, label %421                                                                                    ;L949
 39724| 
 39725| 417: ; preds = %412
 39726|     ;; b = ptr %1
 39727|  %418 = gep %1, i64 3160                                                                                               ;L950
 39728|  %419 = load i64, ptr %418, , !!8                                                                                      ;L950
 39729|  %420 = invoke i64 %58(ptr %54)
 39730|  to label %428 unwind label %97                                                                                        ;L950
 39731| 
 39732| 421: ; preds = %430, %428, %412, %409
 39733|  %422 = gep %1, i64 10584                                                                                              ;L967
 39734|  %423 = load i64, ptr %422, , !!8                                                                                      ;L967
 39735|  %424 = add i64 %423, 1                                                                                                ;L967
 39736|  store i64 %424, ptr %422,                                                                                             ;L967
 39737|  %425 = gep %1, i64 10698                                                                                              ;L968
 39738|  %426 = load i8, ptr %425, , !!8                                                                                       ;L968
 39739|  %427 = trunc nuw i8 %426 to i1                                                                                        ;L968
 39740|  br i1 %427, label %437, label %435                                                                                    ;L968
 39741| 
 39742| 428: ; preds = %417
 39743|  %429 = icmp eq i64 %419, %420                                                                                         ;L950
 39744|  br i1 %429, label %430, label %421                                                                                    ;L950
 39745| 
 39746| 430: ; preds = %428
 39747|  %431 = load i64, ptr %47, , !!8                                                                                       ;L952
 39748|     ;; ii = i64 %431
 39749|  %432 = getelementptr { { { i64 } } }, ptr @gc::simulation6entity17CNT_DM_IDLE_INPUT, i64 %431                         ;L961
 39750|  %433 = gep %432, i64 8                                                                                                ;L961
 39751|     ;; self = ptr %433
 39752|     ;; dst = ptr %433
 39753|  %434 = atomicrmw add ptr %433, i64 1 monotonic,                                                                       ;L3937<3162<961
 39754|  br label %421                                                                                                         ;L950
 39755| 
 39756| 435: ; preds = %437, %421
 39757|  %436 = invoke zeroext i1 @ai::small_actionNtB2_15SmallActionPlay15is_premise_lost(ptr %89, ptr %4)
 39758|  to label %441 unwind label %97                                                                                        ;L969
 39759| 
 39760| 437: ; preds = %421
 39761|  %438 = gep %1, i64 10648                                                                                              ;L968
 39762|  %439 = load i64, ptr %438, , !!8                                                                                      ;L968
 39763|  %440 = add i64 %439, 1                                                                                                ;L968
 39764|  store i64 %440, ptr %438,                                                                                             ;L968
 39765|  br label %435                                                                                                         ;L968
 39766| 
 39767| 441: ; preds = %435
 39768|     ;; target_dead = i1 %436
 39769|     ;; self = ptr %47
 39770|     ;; self = ptr %47
 39771|  %442 = load i64, ptr %47, , !!8                                                                                       ;L633<682<970
 39772|  %443 = icmp eq i64 %442, -1                                                                                           ;L633<682<970
 39773|  br i1 %443, label %451, label %444                                                                                    ;L970
 39774| 
 39775| 444: ; preds = %534, %451, %441
 39776|  %445 = gep %4, i64 8                                                                                                  ;L983
 39777|  %446 = load ptr, ptr %445, , !!8, !!8                                                                                 ;L983
 39778|  %447 = gep %446, i64 59                                                                                               ;L983
 39779|  %448 = load i8, ptr %447, , !!8                                                                                       ;L983
 39780|  %449 = trunc nuw i8 %448 to i1                                                                                        ;L983
 39781|  %450 = and i1 %231, %449                                                                                              ;L983
 39782|  br i1 %450, label %540, label %1129                                                                                   ;L983
 39783| 
 39784| 451: ; preds = %441
 39785|  %452 = load i8, ptr %425, , !!8                                                                                       ;L971
 39786|  %453 = shl nuw nsw i8 %452, 1                                                                                         ;L971
 39787|  %454 = zext i1 %436 to i8                                                                                             ;L971
 39788|  %455 = or disjoint i8 %453, %454                                                                                      ;L971
 39789|  %456 = zext nneg i8 %455 to i64                                                                                       ;L971
 39790|  %457 = gep %1, i64 1040                                                                                               ;L971
 39791|  %458 = getelementptr i64, ptr %457, i64 %456                                                                          ;L971
 39792|  %459 = load i64, ptr %458, , !!8                                                                                      ;L971
 39793|  %460 = add i64 %459, 1                                                                                                ;L971
 39794|  store i64 %460, ptr %458,                                                                                             ;L971
 39795|  %461 = trunc nuw i8 %452 to i1                                                                                        ;L972
 39796|  br i1 %461, label %462, label %444                                                                                    ;L972
 39797| 
 39798| 462: ; preds = %451
 39799|  %463 = load i8, ptr %210, , !!8                                                                                       ;L973
 39801|  %464 = icmp ne i8 %463, 10                                                                                            ;L482<973
 39802|  call void @llvm.assume(i1 %464)                                                                                       ;L482<973
 39803|  %465 = add nsw i8 %463, -3                                                                                            ;L482<973
 39804|  %466 = icmp samesign ugt i8 %463, 2                                                                                   ;L482<973
 39805|  %467 = select i1 %466, i8 %465, i8 7                                                                                  ;L482<973
 39806|  switch i8 %467, label %468 [
 39807|  i8 0, label %474
 39808|  i8 1, label %474
 39809|  i8 2, label %469
 39810|  i8 3, label %469
 39811|  i8 4, label %469
 39812|  i8 5, label %474
 39813|  i8 6, label %469
 39814|  i8 7, label %469
 39815|  i8 8, label %469
 39816|  i8 9, label %469
 39817|  i8 10, label %470
 39818|  i8 11, label %471
 39819|  i8 12, label %472
 39820|  i8 13, label %472
 39821|  i8 14, label %472
 39822|  i8 15, label %472
 39823|  i8 16, label %473
 39824|  ]                                                                                                                     ;L482<973
 39825| 
 39826| 468: ; preds = %462
 39827|  unreachable                                                                                                           ;L482<973
 39828| 
 39829| 469: ; preds = %462, %462, %462, %462, %462, %462, %462
 39830|  br label %474                                                                                                         ;L487<973
 39831| 
 39832| 470: ; preds = %462
 39833|  br label %474                                                                                                         ;L490<973
 39834| 
 39835| 471: ; preds = %462
 39836|  br label %474                                                                                                         ;L483<973
 39837| 
 39838| 472: ; preds = %462, %462, %462, %462
 39839|  br label %474                                                                                                         ;L489<973
 39840| 
 39841| 473: ; preds = %462
 39842|  br label %474                                                                                                         ;L491<973
 39843| 
 39844| 474: ; preds = %473, %472, %471, %470, %469, %462, %462, %462
 39845|  %475 = phi i64 [ 5, %473 ], [ 2, %469 ], [ 4, %470 ], [ 0, %471 ], [ 3, %472 ], [ 1, %462 ], [ 1, %462 ], [ 1, %462 ] ;L0<973
 39846|  %476 = gep %1, i64 1280                                                                                               ;L973
 39847|  %477 = getelementptr i64, ptr %476, i64 %475                                                                          ;L973
 39848|  %478 = load i64, ptr %477, , !!8                                                                                      ;L973
 39849|  %479 = add i64 %478, 1                                                                                                ;L973
 39850|  store i64 %479, ptr %477,                                                                                             ;L973
 39851|     ;; self = ptr %3
 39852|  br i1 %231, label %483, label %480                                                                                    ;L974
 39853| 
 39854| 480: ; preds = %474
 39855|  %481 = gep %4, i64 8                                                                                                  ;L983
 39856|  %482 = load ptr, ptr %481, , !!8, !!8                                                                                 ;L983
 39857|     ;; self = ptr %3
 39858|  br label %1133                                                                                                        ;L1048
 39859| 
 39860| 483: ; preds = %474
 39861|  %484 = load ptr, ptr %229, , !!8, !!8                                                                                 ;L974
 39862|     ;; champ = ptr %484
 39864|  %485 = gep %4, i64 8                                                                                                  ;L975
 39865|  %486 = load ptr, ptr %485, , !!8, !!8                                                                                 ;L975
 39866|  %487 = gep %486, i64 32                                                                                               ;L975
 39867|  %488 = load ptr, ptr %487, , !!8, !!8                                                                                 ;L975
 39868|     ;; self = ptr %488
 39869|  %489 = gep %488, i64 28016                                                                                            ;L235<975
 39870|  %490 = gepS %489, i64 %221                                                                                            ;L235<975
 39871|  %491 = load i64, ptr %490, , !!8                                                                                      ;L235<975
 39872|     ;; flx = i64 %491
 39874|  %492 = gep %490, i64 16                                                                                               ;L235<975
 39875|  %493 = load i64, ptr %492, , !!8                                                                                      ;L235<975
 39876|     ;; frx = i64 %493
 39878|  %494 = gep %484, i64 1632                                                                                             ;L976
 39879|  %495 = load i64, ptr %494, , !!8                                                                                      ;L976
 39880|  %496 = icmp uge i64 %495, %491                                                                                        ;L976
 39881|  %497 = icmp ule i64 %495, %493                                                                                        ;L976
 39882|  %498 = and i1 %496, %497                                                                                              ;L976
 39883|  br i1 %498, label %499, label %509                                                                                    ;L976
 39884| 
 39885| 499: ; preds = %483
 39886|  %500 = gep %490, i64 24                                                                                               ;L235<975
 39887|  %501 = load i64, ptr %500, , !!8                                                                                      ;L235<975
 39888|     ;; fry = i64 %501
 39889|  %502 = gep %490, i64 8                                                                                                ;L235<975
 39890|  %503 = load i64, ptr %502, , !!8                                                                                      ;L235<975
 39891|     ;; fly = i64 %503
 39892|  %504 = gep %484, i64 1640                                                                                             ;L976
 39893|  %505 = load i64, ptr %504, , !!8                                                                                      ;L976
 39894|  %506 = icmp uge i64 %505, %503                                                                                        ;L976
 39895|  %507 = icmp ule i64 %505, %501                                                                                        ;L976
 39896|  %508 = and i1 %506, %507                                                                                              ;L976
 39897|  br i1 %508, label %534, label %509                                                                                    ;L976
 39898| 
 39899| 509: ; preds = %499, %483
 39900|  %510 = gep %53, i64 368                                                                                               ;L977
 39901|  %511 = getelementptr ptr, ptr %510, i64 %221                                                                          ;L977
 39902|  %512 = load ptr, ptr %511, , !!8                                                                                      ;L977
 39903|     ;; self = ptr %512
 39904|     ;; f = ptr %484
 39905|  %513 = icmp eq ptr %512, null                                                                                         ;L659<977
 39906|  br i1 %513, label %534, label %514                                                                                    ;L659<977
 39907| 
 39908| 514: ; preds = %509
 39909|     ;; x = ptr %512
 39910|  %515 = gep %484, i64 1640                                                                                             ;L661<977
 39911|  %516 = load i64, ptr %515, , !!8                                                                                      ;L661<977
 39912|  %517 = gep %512, i64 1632                                                                                             ;L661<977
 39913|  %518 = load i64, ptr %517, , !!8                                                                                      ;L661<977
 39914|  %519 = gep %512, i64 1640                                                                                             ;L661<977
 39915|  %520 = load i64, ptr %519, , !!8                                                                                      ;L661<977
 39920|     ;; x1 = i64 %495
 39921|     ;; self = i64 %495
 39922|     ;; y1 = i64 %516
 39923|     ;; self = i64 %516
 39924|     ;; x2 = i64 %518
 39925|     ;; other = i64 %518
 39926|     ;; y2 = i64 %520
 39927|     ;; other = i64 %520
 39928|  %521 = icmp ult i64 %495, %518                                                                                        ;L3147<7<2158<977<661<977
 39929|  %522 = sub nuw i64 %518, %495                                                                                         ;L3147<7<2158<977<661<977
 39930|  %523 = sub nuw i64 %495, %518                                                                                         ;L3147<7<2158<977<661<977
 39931|  %524 = select i1 %521, i64 %522, i64 %523                                                                             ;L3147<7<2158<977<661<977
 39932|     ;; dx = i64 %524
 39933|  %525 = icmp ult i64 %516, %520                                                                                        ;L3147<8<2158<977<661<977
 39934|  %526 = sub nuw i64 %520, %516                                                                                         ;L3147<8<2158<977<661<977
 39935|  %527 = sub nuw i64 %516, %520                                                                                         ;L3147<8<2158<977<661<977
 39936|  %528 = select i1 %525, i64 %526, i64 %527                                                                             ;L3147<8<2158<977<661<977
 39937|     ;; dy = i64 %528
 39938|  %529 = mul i64 %524, %524                                                                                             ;L9<2158<977<661<977
 39939|  %530 = mul i64 %528, %528                                                                                             ;L9<2158<977<661<977
 39940|  %531 = add i64 %530, %529                                                                                             ;L9<2158<977<661<977
 39941|  %532 = icmp ult i64 %531, 67600000001                                                                                 ;L977<661<977
 39942|  %533 = select i1 %532, i64 1, i64 2                                                                                   ;L977
 39943|  br label %534                                                                                                         ;L977
 39944| 
 39945| 534: ; preds = %514, %509, %499
 39946|  %535 = phi i64 [ 0, %499 ], [ %533, %514 ], [ 2, %509 ]                                                               ;L0
 39947|  %536 = gep %1, i64 10672                                                                                              ;L976
 39948|  %537 = getelementptr i64, ptr %536, i64 %535                                                                          ;L976
 39949|  %538 = load i64, ptr %537, , !!8                                                                                      ;L976
 39950|  %539 = add i64 %538, 1                                                                                                ;L976
 39951|  store i64 %539, ptr %537,                                                                                             ;L976
 39952|  br label %444                                                                                                         ;L974
 39953| 
 39954| 540: ; preds = %444
 39955|  %541 = load ptr, ptr %229, , !!8, !!8                                                                                 ;L984
 39956|     ;; champ = ptr %541
 39957|  switch i64 %442, label %573 [
 39958|  i64 -1, label %579
 39959|  i64 0, label %563
 39960|  ]                                                                                                                     ;L985
 39961| 
 39962| 542: ; preds = %563
 39963|  %543 = extractvalue { i64, i64 } %572, 0                                                                              ;L989
 39964|  %544 = extractvalue { i64, i64 } %572, 1                                                                              ;L989
 39965|     ;; ax = i64 %543
 39966|     ;; x2 = i64 %543
 39967|     ;; other = i64 %543
 39968|     ;; ay = i64 %544
 39969|     ;; y2 = i64 %544
 39970|     ;; other = i64 %544
 39971|  %545 = gep %541, i64 1632                                                                                             ;L990
 39972|  %546 = load i64, ptr %545, , !!8                                                                                      ;L990
 39973|     ;; x1 = i64 %546
 39974|     ;; self = i64 %546
 39975|  %547 = gep %541, i64 1640                                                                                             ;L990
 39976|  %548 = load i64, ptr %547, , !!8                                                                                      ;L990
 39977|     ;; y1 = i64 %548
 39978|     ;; self = i64 %548
 39979|  %549 = icmp ult i64 %546, %543                                                                                        ;L3147<7<990
 39980|  %550 = sub nuw i64 %543, %546                                                                                         ;L3147<7<990
 39981|  %551 = sub nuw i64 %546, %543                                                                                         ;L3147<7<990
 39982|  %552 = select i1 %549, i64 %550, i64 %551                                                                             ;L3147<7<990
 39983|     ;; dx = i64 %552
 39984|  %553 = icmp ult i64 %548, %544                                                                                        ;L3147<8<990
 39985|  %554 = sub nuw i64 %544, %548                                                                                         ;L3147<8<990
 39986|  %555 = sub nuw i64 %548, %544                                                                                         ;L3147<8<990
 39987|  %556 = select i1 %553, i64 %554, i64 %555                                                                             ;L3147<8<990
 39988|     ;; dy = i64 %556
 39989|  %557 = mul i64 %552, %552                                                                                             ;L9<990
 39990|  %558 = mul i64 %556, %556                                                                                             ;L9<990
 39991|  %559 = add i64 %558, %557                                                                                             ;L9<990
 39992|  %560 = freeze i64 %559                                                                                                ;L990
 39993|  %561 = icmp ult i64 %560, 4000001                                                                                     ;L990
 39994|     ;; self[0..+1] = i1 %561
 39995|     ;; self[1..+1] = i8 1
 39997|  %562 = select i1 %561, i8 1, i8 2                                                                                     ;L1651<993
 39998|  br label %573                                                                                                         ;L1651<993
 39999| 
 40000| 563: ; preds = %540
 40001|  %564 = gep %47, i64 8                                                                                                 ;L988
 40002|  %565 = load i64, ptr %564, , !!8                                                                                      ;L988
 40003|     ;; x = i64 %565
 40004|  %566 = gep %47, i64 16                                                                                                ;L988
 40005|  %567 = load i64, ptr %566, , !!8                                                                                      ;L988
 40006|     ;; y = i64 %567
 40007|  %568 = gep %446, i64 32                                                                                               ;L989
 40008|  %569 = load ptr, ptr %568, , !!8, !!8                                                                                 ;L989
 40009|  %570 = gep %446, i64 8                                                                                                ;L989
 40010|  %571 = load ptr, ptr %570, , !!8, !!8                                                                                 ;L989
 40011|  %572 = invoke { i64, i64 } @gc::simulation4gameNtB5_4Game15adjust_position(ptr %569, ptr %571, i64 %565, i64 %567)
 40012|  to label %542 unwind label %97                                                                                        ;L989
 40013| 
 40014| 573: ; preds = %542, %540
 40015|  %574 = phi i1 [ %561, %542 ], [ false, %540 ]
 40016|  %575 = phi i8 [ %562, %542 ], [ 2, %540 ]                                                                             ;L1651<993
 40017|  %576 = load i8, ptr %210,                                                                                             ;L993
 40018|  %577 = icmp eq i8 %576, 19
 40019|  %578 = select i1 %574, i1 true, i1 %577                                                                               ;L1651<993
 40020|     ;; kind[0..+1] = i1 %578
 40021|     ;; kind[1..+1] = i8 %575
 40022|  br i1 %578, label %579, label %1130                                                                                   ;L994
 40023| 
 40024| 579: ; preds = %573, %540
 40025|  %580 = phi i8 [ %575, %573 ], [ 0, %540 ]
 40026|     ;; kind = i8 %580
 40028|  %581 = gep %446, i64 32                                                                                               ;L995
 40029|  %582 = load ptr, ptr %581, , !!8, !!8                                                                                 ;L995
 40030|     ;; self = ptr %582
 40031|  %583 = gep %582, i64 28016                                                                                            ;L235<995
 40032|  %584 = gepS %583, i64 %221                                                                                            ;L235<995
 40033|  %585 = load i64, ptr %584, , !!8                                                                                      ;L235<995
 40034|     ;; flx = i64 %585
 40036|  %586 = gep %584, i64 16                                                                                               ;L235<995
 40037|  %587 = load i64, ptr %586, , !!8                                                                                      ;L235<995
 40038|     ;; frx = i64 %587
 40040|  %588 = gep %541, i64 1632                                                                                             ;L996
 40041|  %589 = load i64, ptr %588, , !!8                                                                                      ;L996
 40042|  %590 = icmp uge i64 %589, %585                                                                                        ;L996
 40043|  %591 = icmp ule i64 %589, %587                                                                                        ;L996
 40044|  %592 = and i1 %590, %591                                                                                              ;L996
 40045|  br i1 %592, label %593, label %605                                                                                    ;L996
 40046| 
 40047| 593: ; preds = %579
 40048|  %594 = gep %584, i64 24                                                                                               ;L235<995
 40049|  %595 = load i64, ptr %594, , !!8                                                                                      ;L235<995
 40050|     ;; fry = i64 %595
 40051|  %596 = gep %584, i64 8                                                                                                ;L235<995
 40052|  %597 = load i64, ptr %596, , !!8                                                                                      ;L235<995
 40053|     ;; fly = i64 %597
 40054|  %598 = gep %541, i64 1640                                                                                             ;L996
 40055|  %599 = load i64, ptr %598, , !!8                                                                                      ;L996
 40056|  %600 = icmp uge i64 %599, %597                                                                                        ;L996
 40057|  %601 = icmp ule i64 %599, %595                                                                                        ;L996
 40058|  %602 = and i1 %600, %601                                                                                              ;L996
 40059|  br i1 %602, label %603, label %605                                                                                    ;L996
 40060| 
 40061| 603: ; preds = %593
 40062|     ;; pos_band = i8 0
 40063|  %604 = load i64, ptr %90, , !!8                                                                                       ;L1002
 40065|     ;; self = ptr %2
 40069|     ;; self = ptr %2
 40070|  invoke void @core::clone5Clone5cloneCshdEBA0ozCnw_7game_ai(ptr sret([256 x i8]) %9, ptr %2)
 40071|  to label %635 unwind label %97                                                                                        ;L124<148<33<1002
 40072| 
 40073| 605: ; preds = %593, %579
 40074|  %606 = gep %53, i64 368                                                                                               ;L998
 40075|  %607 = getelementptr ptr, ptr %606, i64 %221                                                                          ;L998
 40076|  %608 = load ptr, ptr %607, , !!8                                                                                      ;L998
 40077|     ;; self = ptr %608
 40078|     ;; f = ptr %541
 40079|  %609 = icmp eq ptr %608, null                                                                                         ;L659<999
 40080|  br i1 %609, label %630, label %610                                                                                    ;L659<999
 40081| 
 40082| 610: ; preds = %605
 40083|     ;; x = ptr %608
 40084|  %611 = gep %541, i64 1640                                                                                             ;L661<999
 40085|  %612 = load i64, ptr %611, , !!8                                                                                      ;L661<999
 40086|  %613 = gep %608, i64 1632                                                                                             ;L661<999
 40087|  %614 = load i64, ptr %613, , !!8                                                                                      ;L661<999
 40088|  %615 = gep %608, i64 1640                                                                                             ;L661<999
 40089|  %616 = load i64, ptr %615, , !!8                                                                                      ;L661<999
 40094|     ;; x1 = i64 %589
 40095|     ;; self = i64 %589
 40096|     ;; y1 = i64 %612
 40097|     ;; self = i64 %612
 40098|     ;; x2 = i64 %614
 40099|     ;; other = i64 %614
 40100|     ;; y2 = i64 %616
 40101|     ;; other = i64 %616
 40102|  %617 = icmp ult i64 %589, %614                                                                                        ;L3147<7<2158<999<661<999
 40103|  %618 = sub nuw i64 %614, %589                                                                                         ;L3147<7<2158<999<661<999
 40104|  %619 = sub nuw i64 %589, %614                                                                                         ;L3147<7<2158<999<661<999
 40105|  %620 = select i1 %617, i64 %618, i64 %619                                                                             ;L3147<7<2158<999<661<999
 40106|     ;; dx = i64 %620
 40107|  %621 = icmp ult i64 %612, %616                                                                                        ;L3147<8<2158<999<661<999
 40108|  %622 = sub nuw i64 %616, %612                                                                                         ;L3147<8<2158<999<661<999
 40109|  %623 = sub nuw i64 %612, %616                                                                                         ;L3147<8<2158<999<661<999
 40110|  %624 = select i1 %621, i64 %622, i64 %623                                                                             ;L3147<8<2158<999<661<999
 40111|     ;; dy = i64 %624
 40112|  %625 = mul i64 %620, %620                                                                                             ;L9<2158<999<661<999
 40113|  %626 = mul i64 %624, %624                                                                                             ;L9<2158<999<661<999
 40114|  %627 = add i64 %626, %625                                                                                             ;L9<2158<999<661<999
 40115|  %628 = icmp ult i64 %627, 67600000001                                                                                 ;L999<661<999
 40116|  %629 = select i1 %628, i8 1, i8 2                                                                                     ;L998
 40117|  br label %630                                                                                                         ;L998
 40118| 
 40119| 630: ; preds = %651, %642, %610, %605
 40120|  %631 = phi i8 [ 0, %610 ], [ %653, %651 ], [ 0, %605 ], [ 1, %642 ]                                                   ;L0
 40121|  %632 = phi i8 [ %629, %610 ], [ 0, %651 ], [ 2, %605 ], [ 0, %642 ]                                                   ;L0
 40122|     ;; pos_band = i8 %632
 40124|  %633 = gep %1, i64 7544                                                                                               ;L1004
 40125|     ;; self = ptr %633
 40127|  %634 = invoke i64 %58(ptr %54)
 40128|  to label %654 unwind label %97                                                                                        ;L1005
 40129| 
 40130| 635: ; preds = %603
 40131|  %636 = gep %2, i64 256                                                                                                ;L125<148<33<1002
 40132|  %637 = load i64, ptr %636, , !!52449, !!8                                                                             ;L125<148<33<1002
 40133|  %638 = gep %2, i64 272                                                                                                ;L127<148<33<1002
 40134|     ;; self = ptr %638
 40135|  %639 = gep %9, i64 272                                                                                                ;L115<148<33<1002
 40136|  call void @llvm.memcpy.p0.p0.i64(ptr %639, ptr %638, i64 48, i1 false)                                                ;L73<127<148<33<1002
 40137|  %640 = gep %9, i64 256                                                                                                ;L115<148<33<1002
 40138|  store i64 %637, ptr %640, , !!52452                                                                                   ;L115<148<33<1002
 40139|  call void @llvm.memcpy.p0.p0.i64(ptr %27, ptr %9, i64 320, i1 false)                                                  ;L148<33<1002
 40141|  %641 = invoke { i64, i64 } @ai::buy_item(i64 poison, ptr %27, ptr %3, ptr poison, ptr poison, ptr %446)
 40142|  to label %642 unwind label %97                                                                                        ;L1002
 40143| 
 40144| 642: ; preds = %635
 40145|  %643 = extractvalue { i64, i64 } %641, 0                                                                              ;L1002
 40146|  %644 = icmp eq i64 %643, 1                                                                                            ;L1002
 40148|  br i1 %644, label %630, label %645                                                                                    ;L1002
 40149| 
 40150| 645: ; preds = %642
 40152|  %646 = load i64, ptr %90, , !!8                                                                                       ;L1003
 40154|     ;; self = ptr %2
 40158|     ;; self = ptr %2
 40159|  invoke void @core::clone5Clone5cloneCshdEBA0ozCnw_7game_ai(ptr sret([256 x i8]) %8, ptr %2)
 40160|  to label %647 unwind label %97                                                                                        ;L124<148<33<1003
 40161| 
 40162| 647: ; preds = %645
 40163|  %648 = load i64, ptr %636, , !!52531, !!8                                                                             ;L125<148<33<1003
 40164|     ;; self = ptr %638
 40165|  %649 = gep %8, i64 272                                                                                                ;L115<148<33<1003
 40166|  call void @llvm.memcpy.p0.p0.i64(ptr %649, ptr %638, i64 48, i1 false)                                                ;L73<127<148<33<1003
 40167|  %650 = gep %8, i64 256                                                                                                ;L115<148<33<1003
 40168|  store i64 %648, ptr %650, , !!52534                                                                                   ;L115<148<33<1003
 40169|  call void @llvm.memcpy.p0.p0.i64(ptr %25, ptr %8, i64 320, i1 false)                                                  ;L148<33<1003
 40171|  invoke void @ai::upgrade_item(ptr sret([24 x i8]) %26, i64 poison, ptr %25, ptr %3, ptr poison, ptr poison, ptr %446)
 40172|  to label %651 unwind label %97                                                                                        ;L1003
 40173| 
 40174| 651: ; preds = %647
 40175|     ;; self = ptr %26
 40176|  %652 = load i64, ptr %26, , !!8                                                                                       ;L633<1003
 40177|     ;; can_buy = i64 %652
 40180|  %653 = trunc nuw nsw i64 %652 to i8                                                                                   ;L1004
 40181|  br label %630                                                                                                         ;L1001
 40182| 
 40183| 654: ; preds = %630
 40185|  %655 = gep %1, i64 2840                                                                                               ;L1009
 40186|  invoke void @ai::plan_legacy5typesNtB2_7BigPlan11debug_label(ptr sret([24 x i8]) %23, ptr %655)
 40187|  to label %656 unwind label %97                                                                                        ;L1009
 40188| 
 40189| 656: ; preds = %654
 40191|  invoke void @ai::plan_legacy5typesNtB2_7BigPlan8get_name(ptr sret([24 x i8]) %22, ptr %655)
 40192|  to label %659 unwind label %657                                                                                       ;L1010
 40193| 
 40194| 657: ; preds = %662, %656
 40195|  %658 = cleanuppad within none []
 40196|  call void @core::ptr9drop_glueNtNtCs9LexZzt9XJB_5alloc6string6StringECshdEBA0ozCnw_7game_ai(ptr %23) #34 [ "funclet"(token %658) ] ;L1043
 40197|  cleanupret from %658 unwind label %97                                                                                 ;L1043
 40198| 
 40199| 659: ; preds = %656
 40202|  %660 = gep %1, i64 3224                                                                                               ;L1011
 40203|     ;; args = ptr %660
 40205|  store ptr %660, ptr %18,                                                                                              ;L1011
 40206|  %661 = gep %18, i64 8                                                                                                 ;L1011
 40207|  store ptr @ai::plan_legacy8sub_planNtB5_7SubPlanNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %661,                      ;L1011
 40208|     ;; args[0..+8] = ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.144
 40209|     ;; args[8..+8] = ptr %18
 40210|     ;; self[0..+8] = ptr null
 40211|     ;; self[8..+8] = i64 undef
 40215|  invoke void @_RNvNvNtCs9LexZzt9XJB_5alloc3fmt6format12format_inner(ptr sret([24 x i8]) %19, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.144, ptr %18)
 40216|  to label %664 unwind label %662                                                                                       ;L659<1275<659<1011
 40217| 
 40218| 662: ; preds = %824, %746, %745, %743, %672, %670, %659
 40219|  %663 = cleanuppad within none []
 40220|  call void @core::ptr9drop_glueNtNtCs9LexZzt9XJB_5alloc6string6StringECshdEBA0ozCnw_7game_ai(ptr %22) #34 [ "funclet"(token %663) ] ;L1043
 40221|  cleanupret from %663 unwind label %657                                                                                ;L1043
 40222| 
 40223| 664: ; preds = %659
 40225|  call void @llvm.memcpy.p0.p0.i64(ptr %20, ptr %19, i64 24, i1 false)                                                  ;L1011
 40231|     ;; s = ptr %20
 40239|     ;; len = i64 0
 40240|     ;; self = ptr %20
 40241|     ;; self = ptr %20
 40242|  %665 = gep %20, i64 8                                                                                                 ;L614<609<296<1968<1864<1064<2836<1000<1011
 40243|  %666 = load ptr, ptr %665, , !!52563, !!8, !!8                                                                        ;L614<609<296<1968<1864<1064<2836<1000<1011
 40244|  %667 = gep %20, i64 16                                                                                                ;L1864<1064<2836<1000<1011
 40245|  %668 = load i64, ptr %667, , !!52563, !!8                                                                             ;L1864<1064<2836<1000<1011
 40246|     ;; self[0..+8] = ptr %666
 40247|     ;; haystack[0..+8] = ptr %666
 40248|     ;; self[8..+8] = i64 %668
 40249|     ;; haystack[8..+8] = i64 %668
 40251|     ;; haystack[0..+8] = ptr %666
 40252|     ;; self[0..+8] = ptr %666
 40253|     ;; haystack[8..+8] = i64 %668
 40254|     ;; self[8..+8] = i64 %668
 40255|     ;; self[0..+8] = ptr %666
 40256|     ;; self[8..+8] = i64 %668
 40257|  %669 = gep %666, i64 %668                                                                                             ;L961<100<1042<1064<1121<680<739<1659<1000<1011
 40258|     ;; self = ptr undef
 40259|     ;; self = ptr undef
 40262|     ;; self = ptr undef
 40263|     ;; self = ptr undef
 40264|  br label %673                                                                                                         ;L250<787<663<512<1000<1011
 40265| 
 40266| 670: ; preds = %734, %726
 40267|  %671 = cleanuppad within none []
 40268|  invoke void @core::ptr9drop_glueNtNtCs9LexZzt9XJB_5alloc6string6StringECshdEBA0ozCnw_7game_ai(ptr %20) #34 [ "funclet"(token %671) ]
 40269|  to label %672 unwind label %662                                                                                       ;L1000<1011
 40270| 
 40271| 672: ; preds = %670
 40272|  cleanupret from %671 unwind label %662
 40273| 
 40274| 673: ; preds = %717, %664
 40275|  %674 = phi i64 [ %723, %717 ], [ 0, %664 ]
 40276|  %675 = phi ptr [ %718, %717 ], [ %666, %664 ]
 40277|     ;; self = ptr undef
 40278|     ;; s = ptr undef
 40279|     ;; self = ptr undef
 40280|     ;; pointee_size = i64 1
 40281|     ;; end = ptr %669
 40282|     ;; self = ptr %669
 40283|     ;; subtracted = ptr %675
 40284|     ;; self = ptr %669
 40285|     ;; origin = ptr %675
 40286|     ;; origin = ptr %675
 40287|     ;; self = ptr %669
 40288|  %676 = ptrtoint ptr %675 to i64                                                                                       ;L729<887<950<57<695<251<787<663<512<1000<1011
 40289|     ;; pre_len = !DIArgList(ptr %669, i64 %676)
 40290|     ;; self = ptr undef
 40292|     ;; self = ptr undef
 40293|     ;; pointee_size = i64 1
 40294|     ;; end = ptr %669
 40295|     ;; self = ptr %669
 40296|     ;; subtracted = ptr %675
 40297|     ;; self = ptr %669
 40298|     ;; origin = ptr %675
 40299|     ;; origin = ptr %675
 40300|     ;; self = ptr %669
 40301|     ;; pre_len = !DIArgList(ptr %669, i64 %676)
 40302|     ;; self = ptr undef
 40303|     ;; bytes = ptr undef
 40304|     ;; width = i32 2
 40305|     ;; self = ptr undef
 40306|     ;; count = i64 1
 40307|     ;; ptr = ptr %675
 40308|     ;; self = ptr %675
 40309|     ;; end_or_len = ptr %669
 40312|  %677 = icmp eq ptr %675, %669                                                                                         ;L1714<180<37<42<184<696<251<787<663<512<1000<1011
 40313|  br i1 %677, label %726, label %678                                                                                    ;L180<37<42<184<696<251<787<663<512<1000<1011
 40314| 
 40315| 678: ; preds = %673
 40316|  %679 = gep %675, i64 1                                                                                                ;L656<185<37<42<184<696<251<787<663<512<1000<1011
 40317|  %680 = load i8, ptr %675, , !!52848, !!8                                                                              ;L37<42<184<696<251<787<663<512<1000<1011
 40318|     ;; x = i8 %680
 40319|     ;; byte = i8 %680
 40320|  %681 = icmp sgt i8 %680, -1                                                                                           ;L38<42<184<696<251<787<663<512<1000<1011
 40321|  br i1 %681, label %693, label %682                                                                                    ;L38<42<184<696<251<787<663<512<1000<1011
 40322| 
 40323| 682: ; preds = %678
 40324|  %683 = and i8 %680, 31                                                                                                ;L11<45<42<184<696<251<787<663<512<1000<1011
 40325|  %684 = zext nneg i8 %683 to i32                                                                                       ;L11<45<42<184<696<251<787<663<512<1000<1011
 40326|     ;; init = i32 %684
 40327|     ;; ch = i32 %684
 40328|     ;; self = ptr undef
 40329|     ;; count = i64 1
 40330|     ;; ptr = ptr %679
 40331|     ;; self = ptr %679
 40332|     ;; end_or_len = ptr %669
 40335|  %685 = icmp ne ptr %679, %669                                                                                         ;L1714<180<48<42<184<696<251<787<663<512<1000<1011
 40336|  call void @llvm.assume(i1 %685)                                                                                       ;L180<48<42<184<696<251<787<663<512<1000<1011
 40337|  %686 = gep %675, i64 2                                                                                                ;L656<185<48<42<184<696<251<787<663<512<1000<1011
 40338|  %687 = load i8, ptr %679, , !!52848, !!8                                                                              ;L48<42<184<696<251<787<663<512<1000<1011
 40339|     ;; y = i8 %687
 40340|     ;; byte = i8 %687
 40341|  %688 = shl nuw nsw i32 %684, 6                                                                                        ;L17<49<42<184<696<251<787<663<512<1000<1011
 40342|  %689 = and i8 %687, 63                                                                                                ;L17<49<42<184<696<251<787<663<512<1000<1011
 40343|  %690 = zext nneg i8 %689 to i32                                                                                       ;L17<49<42<184<696<251<787<663<512<1000<1011
 40344|  %691 = or disjoint i32 %688, %690                                                                                     ;L17<49<42<184<696<251<787<663<512<1000<1011
 40345|     ;; ch = i32 %691
 40346|  %692 = icmp samesign ugt i8 %680, -33                                                                                 ;L50<42<184<696<251<787<663<512<1000<1011
 40347|  br i1 %692, label %695, label %717                                                                                    ;L50<42<184<696<251<787<663<512<1000<1011
 40348| 
 40349| 693: ; preds = %678
 40350|  %694 = zext nneg i8 %680 to i32                                                                                       ;L39<42<184<696<251<787<663<512<1000<1011
 40351|  br label %717                                                                                                         ;L1<42<184<696<251<787<663<512<1000<1011
 40352| 
 40353| 695: ; preds = %682
 40354|     ;; self = ptr undef
 40355|     ;; count = i64 1
 40356|     ;; ptr = ptr %686
 40357|     ;; self = ptr %686
 40358|     ;; end_or_len = ptr %669
 40361|  %696 = icmp ne ptr %686, %669                                                                                         ;L1714<180<55<42<184<696<251<787<663<512<1000<1011
 40362|  call void @llvm.assume(i1 %696)                                                                                       ;L180<55<42<184<696<251<787<663<512<1000<1011
 40363|  %697 = gep %675, i64 3                                                                                                ;L656<185<55<42<184<696<251<787<663<512<1000<1011
 40364|  %698 = load i8, ptr %686, , !!52848, !!8                                                                              ;L55<42<184<696<251<787<663<512<1000<1011
 40365|     ;; z = i8 %698
 40366|     ;; byte = i8 %698
 40367|     ;; ch = i32 %690
 40368|  %699 = shl nuw nsw i32 %690, 6                                                                                        ;L17<56<42<184<696<251<787<663<512<1000<1011
 40369|  %700 = and i8 %698, 63                                                                                                ;L17<56<42<184<696<251<787<663<512<1000<1011
 40370|  %701 = zext nneg i8 %700 to i32                                                                                       ;L17<56<42<184<696<251<787<663<512<1000<1011
 40371|  %702 = or disjoint i32 %699, %701                                                                                     ;L17<56<42<184<696<251<787<663<512<1000<1011
 40372|     ;; y_z = i32 %702
 40373|     ;; ch = i32 %702
 40374|  %703 = shl nuw nsw i32 %684, 12                                                                                       ;L57<42<184<696<251<787<663<512<1000<1011
 40375|  %704 = or disjoint i32 %702, %703                                                                                     ;L57<42<184<696<251<787<663<512<1000<1011
 40376|     ;; ch = i32 %704
 40377|  %705 = icmp samesign ugt i8 %680, -17                                                                                 ;L58<42<184<696<251<787<663<512<1000<1011
 40378|  br i1 %705, label %706, label %717                                                                                    ;L58<42<184<696<251<787<663<512<1000<1011
 40379| 
 40380| 706: ; preds = %695
 40381|     ;; self = ptr undef
 40382|     ;; count = i64 1
 40383|     ;; ptr = ptr %697
 40384|     ;; self = ptr %697
 40385|     ;; end_or_len = ptr %669
 40388|  %707 = icmp ne ptr %697, %669                                                                                         ;L1714<180<63<42<184<696<251<787<663<512<1000<1011
 40389|  call void @llvm.assume(i1 %707)                                                                                       ;L180<63<42<184<696<251<787<663<512<1000<1011
 40390|  %708 = gep %675, i64 4                                                                                                ;L656<185<63<42<184<696<251<787<663<512<1000<1011
 40391|  %709 = load i8, ptr %697, , !!52848, !!8                                                                              ;L63<42<184<696<251<787<663<512<1000<1011
 40392|     ;; w = i8 %709
 40393|     ;; byte = i8 %709
 40394|  %710 = shl nuw nsw i32 %684, 18                                                                                       ;L64<42<184<696<251<787<663<512<1000<1011
 40395|  %711 = and i32 %710, 1835008                                                                                          ;L64<42<184<696<251<787<663<512<1000<1011
 40396|  %712 = shl nuw nsw i32 %702, 6                                                                                        ;L17<64<42<184<696<251<787<663<512<1000<1011
 40397|  %713 = and i8 %709, 63                                                                                                ;L17<64<42<184<696<251<787<663<512<1000<1011
 40398|  %714 = zext nneg i8 %713 to i32                                                                                       ;L17<64<42<184<696<251<787<663<512<1000<1011
 40399|  %715 = or disjoint i32 %712, %714                                                                                     ;L17<64<42<184<696<251<787<663<512<1000<1011
 40400|  %716 = or disjoint i32 %715, %711                                                                                     ;L64<42<184<696<251<787<663<512<1000<1011
 40401|     ;; ch = i32 %716
 40402|  br label %717                                                                                                         ;L58<42<184<696<251<787<663<512<1000<1011
 40403| 
 40404| 717: ; preds = %706, %695, %693, %682
 40405|  %718 = phi ptr [ %697, %695 ], [ %708, %706 ], [ %686, %682 ], [ %679, %693 ]                                         ;L57<697<251<787<663<512<1000<1011
 40406|  %719 = phi i32 [ %704, %695 ], [ %716, %706 ], [ %691, %682 ], [ %694, %693 ]
 40407|     ;; self[0..+4] = i32 1
 40408|     ;; self[4..+4] = i32 %719
 40409|     ;; x = i32 %719
 40410|     ;; ch = i32 %719
 40411|     ;; i = i32 %719
 40412|     ;; i = i32 %719
 40413|  %720 = icmp samesign ult i32 %719, 1114112                                                                            ;L34<239<42<1162<42<184<696<251<787<663<512<1000<1011
 40414|  call void @llvm.assume(i1 %720)                                                                                       ;L34<239<42<1162<42<184<696<251<787<663<512<1000<1011
 40415|     ;; ch = i32 %719
 40416|     ;; index = i64 %674
 40417|     ;; self = ptr undef
 40418|     ;; pointee_size = i64 1
 40419|     ;; end = ptr %669
 40420|     ;; self = ptr %669
 40421|     ;; subtracted = ptr %718
 40422|     ;; self = ptr %669
 40423|     ;; origin = ptr %718
 40424|     ;; origin = ptr %718
 40425|     ;; self = ptr %669
 40426|  %721 = ptrtoint ptr %718 to i64                                                                                       ;L729<887<950<57<188<696<251<787<663<512<1000<1011
 40427|     ;; len = !DIArgList(ptr %669, i64 %721)
 40428|  %722 = sub i64 %674, %676                                                                                             ;L189<696<251<787<663<512<1000<1011
 40429|  %723 = add i64 %722, %721                                                                                             ;L189<696<251<787<663<512<1000<1011
 40430|     ;; i = i64 %674
 40431|     ;; c = i32 %719
 40432|     ;; self = ptr undef
 40433|     ;; pointee_size = i64 1
 40434|     ;; end = ptr %669
 40435|     ;; self = ptr %669
 40436|     ;; subtracted = ptr %718
 40437|     ;; self = ptr %669
 40438|     ;; origin = ptr %718
 40439|     ;; origin = ptr %718
 40440|     ;; self = ptr %669
 40441|     ;; len = !DIArgList(ptr %669, i64 %721)
 40444|     ;; c = i32 %719
 40446|     ;; c = i32 %719
 40447|  %724 = and i32 %719, 2097143                                                                                          ;L1000<641<699<251<787<663<512<1000<1011
 40448|  %725 = icmp eq i32 %724, 32                                                                                           ;L1000<641<699<251<787<663<512<1000<1011
 40449|  br i1 %725, label %726, label %673                                                                                    ;L251<787<663<512<1000<1011
 40450| 
 40451| 726: ; preds = %717, %673
 40452|  %727 = phi i64 [ %674, %717 ], [ %668, %673 ]                                                                         ;L0<512<1000<1011
 40453|     ;; self[0..+8] = ptr %666
 40454|     ;; s[0..+8] = ptr %666
 40455|     ;; s[0..+8] = ptr %666
 40456|     ;; self[0..+8] = ptr %666
 40457|     ;; self[0..+8] = ptr %666
 40458|     ;; self[8..+8] = i64 %727
 40459|     ;; s[8..+8] = i64 %727
 40460|     ;; s[8..+8] = i64 %727
 40461|     ;; self[8..+8] = i64 %727
 40462|     ;; self[8..+8] = i64 %727
 40463|     ;; self[0..+8] = ptr %666
 40464|     ;; self[0..+8] = ptr %666
 40465|     ;; self[0..+8] = ptr %666
 40466|     ;; s[0..+8] = ptr %666
 40467|     ;; self[8..+8] = i64 %727
 40468|     ;; self[8..+8] = i64 %727
 40469|     ;; self[8..+8] = i64 %727
 40470|     ;; s[8..+8] = i64 %727
 40471|     ;; len = i64 %727
 40472|     ;; capacity = i64 %727
 40473|     ;; capacity = i64 %727
 40474|     ;; count = i64 %727
 40475|     ;; count = i64 %727
 40476|     ;; capacity = i64 %727
 40477|     ;; additional = i64 %727
 40478|     ;; elem_layout[0..+8] = i64 1
 40479|     ;; elem_layout[0..+8] = i64 1
 40480|     ;; elem_layout[8..+8] = i64 1
 40481|     ;; elem_layout[8..+8] = i64 1
 40483|  invoke void @_RNvMs4_NtCs9LexZzt9XJB_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCshdEBA0ozCnw_7game_ai(ptr sret([24 x i8]) %6, i64 %727, i1 zeroext false, i64 1, i64 1)
 40484|  to label %728 unwind label %670, !!53010                                                                              ;L434<177<977<448<400<376<842<251<3127<3058<2907<1000<1011
 40485| 
 40486| 728: ; preds = %726
 40487|  %729 = load i64, ptr %6, , !!53010, !!8                                                                               ;L434<177<977<448<400<376<842<251<3127<3058<2907<1000<1011
 40488|  %730 = trunc nuw i64 %729 to i1                                                                                       ;L434<177<977<448<400<376<842<251<3127<3058<2907<1000<1011
 40489|  %731 = gep %6, i64 8                                                                                                  ;L0<177<977<448<400<376<842<251<3127<3058<2907<1000<1011
 40490|  %732 = load i64, ptr %731, , !!53010, !!8                                                                             ;L0<177<977<448<400<376<842<251<3127<3058<2907<1000<1011
 40491|  %733 = gep %6, i64 16                                                                                                 ;L0<177<977<448<400<376<842<251<3127<3058<2907<1000<1011
 40492|  br i1 %730, label %734, label %736                                                                                    ;L434<177<977<448<400<376<842<251<3127<3058<2907<1000<1011
 40493| 
 40494| 734: ; preds = %728
 40495|  %735 = load i64, ptr %733, , !!53010                                                                                  ;L442<177<977<448<400<376<842<251<3127<3058<2907<1000<1011
 40496|     ;; err[0..+8] = i64 %732
 40497|     ;; err[8..+8] = i64 %735
 40498|  invoke void @_RNvNtCs9LexZzt9XJB_5alloc7raw_vec12handle_error(i64 %732, i64 %735) #35
 40499|  to label %748 unwind label %670, !!53010                                                                              ;L442<177<977<448<400<376<842<251<3127<3058<2907<1000<1011
 40500| 
 40501| 736: ; preds = %728
 40502|  %737 = load ptr, ptr %733, , !!53010, !!8, !!8                                                                        ;L435<177<977<448<400<376<842<251<3127<3058<2907<1000<1011
 40503|     ;; this[0..+8] = i64 %732
 40504|     ;; this[8..+8] = ptr %737
 40506|  %738 = icmp ule i64 %727, %732                                                                                        ;L767<438<177<977<448<400<376<842<251<3127<3058<2907<1000<1011
 40507|     ;; cond = i1 true
 40508|  call void @llvm.assume(i1 %738)                                                                                       ;L210<438<177<977<448<400<376<842<251<3127<3058<2907<1000<1011
 40510|     ;; bytes[0..+8] = i64 %732
 40511|     ;; v[0..+8] = i64 %732
 40512|     ;; bytes[8..+8] = ptr %737
 40513|     ;; v[8..+8] = ptr %737
 40514|     ;; bytes[16..+8] = i64 0
 40515|     ;; v[16..+8] = i64 0
 40516|  %739 = icmp eq i64 %727, 0                                                                                            ;L452<400<376<842<251<3127<3058<2907<1000<1011
 40517|  br i1 %739, label %740, label %747                                                                                    ;L452<400<376<842<251<3127<3058<2907<1000<1011
 40518| 
 40519| 740: ; preds = %747, %736
 40520|     ;; v[16..+8] = i64 %727
 40521|     ;; bytes[16..+8] = i64 %727
 40522|  store i64 %732, ptr %21, , !!52566                                                                                    ;L1023<251<3127<3058<2907<1000<1011
 40523|  %741 = gep %21, i64 8                                                                                                 ;L1023<251<3127<3058<2907<1000<1011
 40524|  store ptr %737, ptr %741, , !!52566                                                                                   ;L1023<251<3127<3058<2907<1000<1011
 40525|  %742 = gep %21, i64 16                                                                                                ;L1023<251<3127<3058<2907<1000<1011
 40526|  store i64 %727, ptr %742, , !!52566                                                                                   ;L1023<251<3127<3058<2907<1000<1011
 40529|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %20)
 40530|  to label %746 unwind label %743, !!52563                                                                              ;L825<825<1000<1011
 40531| 
 40532| 743: ; preds = %740
 40533|  %744 = cleanuppad within none []
 40535|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %20) [ "funclet"(token %744) ]
 40536|  to label %745 unwind label %662                                                                                       ;L825<825<825<1000<1011
 40537| 
 40538| 745: ; preds = %743
 40539|  cleanupret from %744 unwind label %662
 40540| 
 40541| 746: ; preds = %740
 40543|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %20)
 40544|  to label %749 unwind label %662                                                                                       ;L825<825<825<1000<1011
 40545| 
 40546| 747: ; preds = %736
 40547|     ;; self = ptr %666
 40548|     ;; src = ptr %666
 40549|     ;; dest = ptr %737
 40550|     ;; dst = ptr %737
 40551|  call void @llvm.memcpy.p0.p0.i64(ptr %737, ptr %666, i64 %727, i1 false), !!53010                                     ;L551<1252<454<400<376<842<251<3127<3058<2907<1000<1011
 40552|     ;; bytes[16..+8] = i64 %727
 40553|     ;; v[16..+8] = i64 %727
 40554|  br label %740                                                                                                         ;L452<400<376<842<251<3127<3058<2907<1000<1011
 40555| 
 40556| 748: ; preds = %734
 40557|  unreachable
 40558| 
 40559| 749: ; preds = %746
 40566|     ;; self = ptr %89
 40567|  %750 = load i8, ptr %210, , !!53044, !!8                                                                              ;L309<1012
 40568|  %751 = icmp ne i8 %750, 10                                                                                            ;L309<1012
 40569|  call void @llvm.assume(i1 %751)                                                                                       ;L309<1012
 40570|  %752 = add nsw i8 %750, -3                                                                                            ;L309<1012
 40571|  %753 = icmp samesign ugt i8 %750, 2                                                                                   ;L309<1012
 40572|  %754 = select i1 %753, i8 %752, i8 7                                                                                  ;L309<1012
 40573|  switch i8 %754, label %755 [
 40574|  i8 0, label %826
 40575|  i8 1, label %826
 40576|  i8 2, label %756
 40577|  i8 3, label %760
 40578|  i8 4, label %764
 40579|  i8 5, label %826
 40580|  i8 6, label %771
 40581|  i8 7, label %778
 40582|  i8 8, label %785
 40583|  i8 9, label %792
 40584|  i8 10, label %799
 40585|  i8 11, label %803
 40586|  i8 12, label %807
 40587|  i8 13, label %811
 40588|  i8 14, label %815
 40589|  i8 15, label %819
 40590|  i8 16, label %823
 40591|  ]                                                                                                                     ;L309<1012
 40592| 
 40593| 755: ; preds = %749
 40594|  unreachable                                                                                                           ;L309<1012
 40595| 
 40596| 756: ; preds = %749
 40597|     ;; action = ptr %89
 40598|     ;; self = ptr %89
 40599|  %757 = gep %1, i64 10336                                                                                              ;L285<316<1012
 40600|  %758 = load i64, ptr %757, , !!53044, !!8                                                                             ;L285<316<1012
 40601|  %759 = gep %15, i64 8                                                                                                 ;L285<316<1012
 40602|  store i64 %758, ptr %759, , !!53047                                                                                   ;L285<316<1012
 40603|  br label %826                                                                                                         ;L316<1012
 40604| 
 40605| 760: ; preds = %749
 40606|     ;; action = ptr %89
 40607|     ;; self = ptr %89
 40608|  %761 = gep %1, i64 10336                                                                                              ;L491<313<1012
 40609|  %762 = load i64, ptr %761, , !!53044, !!8                                                                             ;L491<313<1012
 40610|  %763 = gep %15, i64 8                                                                                                 ;L491<313<1012
 40611|  store i64 %762, ptr %763, , !!53047                                                                                   ;L491<313<1012
 40612|  br label %826                                                                                                         ;L313<1012
 40613| 
 40614| 764: ; preds = %749
 40615|     ;; action = ptr %89
 40616|     ;; self = ptr %89
 40617|  %765 = gep %1, i64 10344                                                                                              ;L665<314<1012
 40618|  %766 = load i64, ptr %765, , !!53044, !!8                                                                             ;L665<314<1012
 40619|  %767 = gep %1, i64 10352                                                                                              ;L665<314<1012
 40620|  %768 = load i64, ptr %767, , !!53044, !!8                                                                             ;L665<314<1012
 40621|  %769 = gep %15, i64 8                                                                                                 ;L665<314<1012
 40622|  store i64 %766, ptr %769, , !!53047                                                                                   ;L665<314<1012
 40623|  %770 = gep %15, i64 16                                                                                                ;L665<314<1012
 40624|  store i64 %768, ptr %770, , !!53047                                                                                   ;L665<314<1012
 40625|  br label %826                                                                                                         ;L314<1012
 40626| 
 40627| 771: ; preds = %749
 40628|     ;; action = ptr %89
 40629|     ;; self = ptr %89
 40630|  %772 = gep %1, i64 10336                                                                                              ;L800<312<1012
 40631|  %773 = load i64, ptr %772, , !!53044, !!8                                                                             ;L800<312<1012
 40632|  %774 = gep %1, i64 10344                                                                                              ;L800<312<1012
 40633|  %775 = load i64, ptr %774, , !!53044, !!8                                                                             ;L800<312<1012
 40634|  %776 = gep %15, i64 8                                                                                                 ;L800<312<1012
 40635|  store i64 %773, ptr %776, , !!53047                                                                                   ;L800<312<1012
 40636|  %777 = gep %15, i64 16                                                                                                ;L800<312<1012
 40637|  store i64 %775, ptr %777, , !!53047                                                                                   ;L800<312<1012
 40638|  br label %826                                                                                                         ;L312<1012
 40639| 
 40640| 778: ; preds = %749
 40641|     ;; action = ptr %89
 40642|     ;; self = ptr %89
 40643|  %779 = gep %1, i64 10376                                                                                              ;L1035<317<1012
 40644|  %780 = load i64, ptr %779, , !!53044, !!8                                                                             ;L1035<317<1012
 40645|  %781 = gep %1, i64 10384                                                                                              ;L1035<317<1012
 40646|  %782 = load i64, ptr %781, , !!53044, !!8                                                                             ;L1035<317<1012
 40647|  %783 = gep %15, i64 8                                                                                                 ;L1035<317<1012
 40648|  store i64 %780, ptr %783, , !!53047                                                                                   ;L1035<317<1012
 40649|  %784 = gep %15, i64 16                                                                                                ;L1035<317<1012
 40650|  store i64 %782, ptr %784, , !!53047                                                                                   ;L1035<317<1012
 40651|  br label %826                                                                                                         ;L317<1012
 40652| 
 40653| 785: ; preds = %749
 40654|     ;; action = ptr %89
 40655|     ;; self = ptr %89
 40656|  %786 = gep %1, i64 10336                                                                                              ;L1123<318<1012
 40657|  %787 = load i64, ptr %786, , !!53044, !!8                                                                             ;L1123<318<1012
 40658|  %788 = gep %1, i64 10344                                                                                              ;L1123<318<1012
 40659|  %789 = load i64, ptr %788, , !!53044, !!8                                                                             ;L1123<318<1012
 40660|  %790 = gep %15, i64 8                                                                                                 ;L1123<318<1012
 40661|  store i64 %787, ptr %790, , !!53047                                                                                   ;L1123<318<1012
 40662|  %791 = gep %15, i64 16                                                                                                ;L1123<318<1012
 40663|  store i64 %789, ptr %791, , !!53047                                                                                   ;L1123<318<1012
 40664|  br label %826                                                                                                         ;L318<1012
 40665| 
 40666| 792: ; preds = %749
 40667|     ;; action = ptr %89
 40668|     ;; self = ptr %89
 40669|  %793 = gep %1, i64 10352                                                                                              ;L1241<319<1012
 40670|  %794 = load i64, ptr %793, , !!53044, !!8                                                                             ;L1241<319<1012
 40671|  %795 = gep %1, i64 10360                                                                                              ;L1241<319<1012
 40672|  %796 = load i64, ptr %795, , !!53044, !!8                                                                             ;L1241<319<1012
 40673|  %797 = gep %15, i64 8                                                                                                 ;L1241<319<1012
 40674|  store i64 %794, ptr %797, , !!53047                                                                                   ;L1241<319<1012
 40675|  %798 = gep %15, i64 16                                                                                                ;L1241<319<1012
 40676|  store i64 %796, ptr %798, , !!53047                                                                                   ;L1241<319<1012
 40677|  br label %826                                                                                                         ;L319<1012
 40678| 
 40679| 799: ; preds = %749
 40680|     ;; action = ptr %89
 40681|     ;; self = ptr %89
 40682|  %800 = gep %1, i64 10336                                                                                              ;L653<320<1012
 40683|  %801 = load i64, ptr %800, , !!53044, !!8                                                                             ;L653<320<1012
 40684|  %802 = gep %15, i64 8                                                                                                 ;L653<320<1012
 40685|  store i64 %801, ptr %802, , !!53047                                                                                   ;L653<320<1012
 40686|  br label %826                                                                                                         ;L320<1012
 40687| 
 40688| 803: ; preds = %749
 40689|     ;; action = ptr %89
 40690|     ;; self = ptr %89
 40691|  %804 = gep %1, i64 10424                                                                                              ;L404<321<1012
 40692|  %805 = load i64, ptr %804, , !!53044, !!8                                                                             ;L404<321<1012
 40693|  %806 = gep %15, i64 8                                                                                                 ;L404<321<1012
 40694|  store i64 %805, ptr %806, , !!53047                                                                                   ;L404<321<1012
 40695|  br label %826                                                                                                         ;L321<1012
 40696| 
 40697| 807: ; preds = %749
 40698|     ;; action = ptr %89
 40699|     ;; self = ptr %89
 40700|  %808 = gep %1, i64 10336                                                                                              ;L94<322<1012
 40701|  %809 = load i64, ptr %808, , !!53044, !!8                                                                             ;L94<322<1012
 40702|  %810 = gep %15, i64 8                                                                                                 ;L94<322<1012
 40703|  store i64 %809, ptr %810, , !!53047                                                                                   ;L94<322<1012
 40704|  br label %826                                                                                                         ;L322<1012
 40705| 
 40706| 811: ; preds = %749
 40707|     ;; action = ptr %89
 40708|     ;; self = ptr %89
 40709|  %812 = gep %1, i64 10336                                                                                              ;L160<323<1012
 40710|  %813 = load i64, ptr %812, , !!53044, !!8                                                                             ;L160<323<1012
 40711|  %814 = gep %15, i64 8                                                                                                 ;L160<323<1012
 40712|  store i64 %813, ptr %814, , !!53047                                                                                   ;L160<323<1012
 40713|  br label %826                                                                                                         ;L323<1012
 40714| 
 40715| 815: ; preds = %749
 40716|     ;; action = ptr %89
 40717|     ;; self = ptr %89
 40718|  %816 = gep %1, i64 10336                                                                                              ;L222<324<1012
 40719|  %817 = load i64, ptr %816, , !!53044, !!8                                                                             ;L222<324<1012
 40720|  %818 = gep %15, i64 8                                                                                                 ;L222<324<1012
 40721|  store i64 %817, ptr %818, , !!53047                                                                                   ;L222<324<1012
 40722|  br label %826                                                                                                         ;L324<1012
 40723| 
 40724| 819: ; preds = %749
 40725|     ;; action = ptr %89
 40726|     ;; self = ptr %89
 40727|  %820 = gep %1, i64 10336                                                                                              ;L287<325<1012
 40728|  %821 = load i64, ptr %820, , !!53044, !!8                                                                             ;L287<325<1012
 40729|  %822 = gep %15, i64 8                                                                                                 ;L287<325<1012
 40730|  store i64 %821, ptr %822, , !!53047                                                                                   ;L287<325<1012
 40731|  br label %826                                                                                                         ;L325<1012
 40732| 
 40733| 823: ; preds = %749
 40734|  br label %826                                                                                                         ;L326<1012
 40735| 
 40736| 824: ; preds = %846, %826
 40737|  %825 = cleanuppad within none []
 40738|  call void @core::ptr9drop_glueNtNtCs9LexZzt9XJB_5alloc6string6StringECshdEBA0ozCnw_7game_ai(ptr %21) #34 [ "funclet"(token %825) ] ;L1043
 40739|  cleanupret from %825 unwind label %662                                                                                ;L1043
 40740| 
 40741| 826: ; preds = %823, %819, %815, %811, %807, %803, %799, %792, %785, %778, %771, %764, %760, %756, %749, %749, %749
 40742|  %827 = phi i64 [ 10, %823 ], [ 9, %819 ], [ 8, %815 ], [ 7, %811 ], [ 6, %807 ], [ 4, %803 ], [ 2, %799 ], [ 3, %792 ], [ 3, %785 ], [ 3, %778 ], [ 1, %771 ], [ 0, %749 ], [ 3, %764 ], [ 2, %760 ], [ 2, %756 ], [ 0, %749 ], [ 0, %749 ]
 40743|  store i64 %827, ptr %15, , !!53047                                                                                    ;L0<1012
 40744|     ;; args = ptr %15
 40746|  store ptr %15, ptr %14,                                                                                               ;L1012
 40747|  %828 = gep %14, i64 8                                                                                                 ;L1012
 40748|  store ptr @gc::simulation4game10blackboardNtB5_11SmallActionNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %828,          ;L1012
 40749|     ;; args[0..+8] = ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.144
 40750|     ;; args[8..+8] = ptr %14
 40751|     ;; self[0..+8] = ptr null
 40752|     ;; self[8..+8] = i64 undef
 40756|  invoke void @_RNvNvNtCs9LexZzt9XJB_5alloc3fmt6format12format_inner(ptr sret([24 x i8]) %16, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.144, ptr %14)
 40757|  to label %829 unwind label %824                                                                                       ;L659<1275<659<1012
 40758| 
 40759| 829: ; preds = %826
 40762|  call void @llvm.memcpy.p0.p0.i64(ptr %17, ptr %16, i64 24, i1 false)                                                  ;L614<1012
 40764|     ;; self = ptr %89
 40765|  %830 = load i8, ptr %210, , !!53161, !!8                                                                              ;L230<1013
 40766|  %831 = icmp ne i8 %830, 10                                                                                            ;L230<1013
 40767|  call void @llvm.assume(i1 %831)                                                                                       ;L230<1013
 40768|  %832 = add nsw i8 %830, -3                                                                                            ;L230<1013
 40769|  %833 = icmp samesign ugt i8 %830, 2                                                                                   ;L230<1013
 40770|  %834 = select i1 %833, i8 %832, i8 7                                                                                  ;L230<1013
 40771|  switch i8 %834, label %848 [
 40772|  i8 0, label %839
 40773|  i8 1, label %835
 40774|  i8 2, label %836
 40775|  i8 3, label %836
 40776|  i8 6, label %839
 40777|  i8 7, label %839
 40778|  i8 8, label %839
 40779|  i8 9, label %837
 40780|  i8 10, label %836
 40781|  i8 11, label %838
 40782|  ]                                                                                                                     ;L230<1013
 40783| 
 40784| 835: ; preds = %829
 40785|  br label %839                                                                                                         ;L232<1013
 40786| 
 40787| 836: ; preds = %829, %829, %829
 40788|  br label %839                                                                                                         ;L234<1013
 40789| 
 40790| 837: ; preds = %829
 40791|  br label %839                                                                                                         ;L238<1013
 40792| 
 40793| 838: ; preds = %829
 40794|  br label %839                                                                                                         ;L240<1013
 40795| 
 40796| 839: ; preds = %838, %837, %836, %835, %829, %829, %829, %829
 40797|  %840 = phi i64 [ 104, %838 ], [ 80, %835 ], [ 24, %837 ], [ 8, %829 ], [ 8, %829 ], [ 8, %829 ], [ 8, %829 ], [ 16, %836 ]
 40798|  %841 = phi i64 [ 112, %838 ], [ 88, %835 ], [ 32, %837 ], [ 16, %829 ], [ 16, %829 ], [ 16, %829 ], [ 16, %829 ], [ 24, %836 ]
 40799|  %842 = gep %89, i64 %840                                                                                              ;L0<1013
 40800|  %843 = load i64, ptr %842, , !!53161, !!8                                                                             ;L0<1013
 40801|  %844 = gep %89, i64 %841                                                                                              ;L0<1013
 40802|  %845 = load i64, ptr %844, , !!53161, !!8                                                                             ;L0<1013
 40803|  br label %848                                                                                                         ;L0<1013
 40804| 
 40805| 846: ; preds = %1071, %1057, %872
 40806|  %847 = cleanuppad within none []
 40807|  call void @core::ptr9drop_glueNtNtCs9LexZzt9XJB_5alloc6string6StringECshdEBA0ozCnw_7game_ai(ptr %17) #34 [ "funclet"(token %847) ] ;L1043
 40808|  cleanupret from %847 unwind label %824                                                                                ;L1043
 40809| 
 40810| 848: ; preds = %839, %829
 40811|  %849 = phi i64 [ undef, %829 ], [ %843, %839 ]
 40812|  %850 = phi i64 [ undef, %829 ], [ %845, %839 ]
 40813|  %851 = phi i64 [ 0, %829 ], [ 1, %839 ]
 40814|  %852 = gep %541, i64 1640                                                                                             ;L1014
 40815|  %853 = load i64, ptr %852, , !!8                                                                                      ;L1014
 40816|  %854 = gep %541, i64 1648                                                                                             ;L1016
 40817|  %855 = load i64, ptr %854, , !!8                                                                                      ;L1016
 40818|  %856 = mul i64 %855, 100                                                                                              ;L1016
 40819|  %857 = gep %541, i64 1576                                                                                             ;L1016
 40820|  %858 = load i64, ptr %857, , !!8                                                                                      ;L1016
 40821|     ;; self = i64 %858
 40822|     ;; other = i64 1
 40823|  %859 = call i64 @llvm.umax.i64(i64 %858, i64 1)                                                                       ;L1039<1016
 40824|  %860 = udiv i64 %856, %859                                                                                            ;L1016
 40825|     ;; team = !DIArgList(i64 1, i64 %221)
 40826|  %861 = sub nuw nsw i64 1, %221                                                                                        ;L1017
 40827|     ;; team = i64 %861
 40828|  %862 = getelementptr [5 x ptr], ptr %227, i64 %861                                                                    ;L1905<1017
 40829|     ;; self[0..+8] = ptr %862
 40830|     ;; slice[0..+8] = ptr %862
 40831|     ;; self[8..+8] = i64 5
 40832|     ;; slice[8..+8] = i64 5
 40833|     ;; self = ptr %862
 40834|  %863 = gep %4, i64 16                                                                                                 ;L1018
 40835|  %864 = load ptr, ptr %863, , !!8, !!8                                                                                 ;L1018
 40836|     ;; self[0..+8] = ptr %862
 40837|     ;; self[8..+8] = ptr %862
 40838|     ;; self[16..+8] = ptr %54
 40839|     ;; self[24..+8] = ptr %56
 40840|     ;; self[32..+8] = ptr %864
 40841|     ;; self[40..+8] = ptr %3
 40842|     ;; self[48..+8] = ptr %541
 40843|     ;; init = i64 0
 40846|     ;; self[0..+8] = ptr %862
 40847|     ;; iter[0..+8] = ptr %862
 40848|     ;; self[0..+8] = ptr %862
 40849|     ;; self[8..+8] = ptr %862
 40850|     ;; iter[8..+8] = ptr %862
 40851|     ;; self[8..+8] = ptr %862
 40852|     ;; self[16..+8] = ptr %54
 40853|     ;; iter[16..+8] = ptr %54
 40854|     ;; self[16..+8] = ptr %54
 40855|     ;; self[24..+8] = ptr %56
 40856|     ;; iter[24..+8] = ptr %56
 40857|     ;; self[24..+8] = ptr %56
 40858|     ;; self[32..+8] = ptr %864
 40859|     ;; iter[32..+8] = ptr %864
 40860|     ;; self[32..+8] = ptr %864
 40861|     ;; self[40..+8] = ptr %3
 40862|     ;; iter[40..+8] = ptr %3
 40863|     ;; self[40..+8] = ptr %3
 40864|     ;; fold[0..+8] = ptr %54
 40865|     ;; fold[8..+8] = ptr %56
 40866|     ;; fold[16..+8] = ptr %864
 40867|     ;; fold[24..+8] = ptr %3
 40868|     ;; self[0..+8] = ptr %862
 40869|     ;; self[8..+8] = ptr %862
 40870|     ;; init = i64 0
 40871|     ;; f[0..+8] = ptr %54
 40872|     ;; f[8..+8] = ptr %56
 40873|     ;; f[16..+8] = ptr %864
 40874|     ;; f[24..+8] = ptr %3
 40875|     ;; self[0..+8] = ptr %862
 40876|     ;; self[8..+8] = ptr %862
 40877|     ;; init = i64 0
 40878|     ;; rhs = i64 1
 40879|     ;; self[48..+8] = ptr %541
 40880|     ;; iter[48..+8] = ptr %541
 40881|     ;; self[48..+8] = ptr %541
 40882|     ;; f[32..+8] = ptr %541
 40883|     ;; fold[32..+8] = ptr %541
 40884|     ;; acc = i64 0
 40885|     ;; i = i64 0
 40886|     ;; self = i64 0
 40887|     ;; len = i64 5
 40888|  %865 = getelementptr { { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, [5 x { i64, [2 x i64] }], [5 x { i64, { i8, [23 x i8] } }], [5 x { i8, [7 x i8], i64 }], [5 x i64], [5 x i64], [5 x i64], [5 x { i64, i64 }], i64, i64, i64, i64, i64, [5 x i32], i8, [3 x i8] }, ptr %864, i64 %861
 40889|  br label %866                                                                                                         ;L28<146<128<52<3674<142<1019
 40890| 
 40891| 866: ; preds = %898, %848
 40892|  %867 = phi i64 [ 0, %848 ], [ %900, %898 ]                                                                            ;L0<146<128<52<3674<142<1019
 40893|  %868 = phi i64 [ 0, %848 ], [ %899, %898 ]                                                                            ;L0<146<128<52<3674<142<1019
 40894|     ;; acc = i64 %868
 40895|     ;; self = i64 %867
 40896|     ;; i = i64 %867
 40897|     ;; self = ptr %862
 40898|     ;; count = i64 %867
 40899|  %869 = getelementptr ptr, ptr %862, i64 %867                                                                          ;L656<279<146<128<52<3674<142<1019
 40900|  %870 = load ptr, ptr %869, , !!53300, !!8                                                                             ;L279<146<128<52<3674<142<1019
 40902|     ;; acc = i64 %868
 40904|  %871 = icmp eq ptr %870, null                                                                                         ;L39<279<146<128<52<3674<142<1019
 40905|  br i1 %871, label %898, label %872                                                                                    ;L39<279<146<128<52<3674<142<1019
 40906| 
 40907| 872: ; preds = %866
 40908|     ;; x = ptr %870
 40910|     ;; acc = i64 %868
 40911|     ;; elt = ptr %870
 40912|     ;; x = ptr %870
 40918|  %873 = invoke zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %865, ptr %54, ptr %56, ptr %3, ptr %870)
 40919|  to label %874 unwind label %846                                                                                       ;L1018<138<88<40<279<146<128<52<3674<142<1019
 40920| 
 40921| 874: ; preds = %872
 40922|  br i1 %873, label %875, label %895                                                                                    ;L1018<138<88<40<279<146<128<52<3674<142<1019
 40923| 
 40924| 875: ; preds = %874
 40925|     ;; self = ptr %870
 40927|  %876 = gep %870, i64 1632                                                                                             ;L2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40928|  %877 = load i64, ptr %876, , !!53374, !!8                                                                             ;L2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40929|     ;; x1 = i64 %877
 40930|     ;; self = i64 %877
 40931|  %878 = gep %870, i64 1640                                                                                             ;L2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40932|  %879 = load i64, ptr %878, , !!53374, !!8                                                                             ;L2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40933|     ;; y1 = i64 %879
 40934|     ;; self = i64 %879
 40935|  %880 = load i64, ptr %588, , !!53374, !!8                                                                             ;L2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40936|     ;; x2 = i64 %880
 40937|     ;; other = i64 %880
 40938|  %881 = load i64, ptr %852, , !!53374, !!8                                                                             ;L2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40939|     ;; y2 = i64 %881
 40940|     ;; other = i64 %881
 40941|  %882 = icmp ult i64 %877, %880                                                                                        ;L3147<7<2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40942|  %883 = sub nuw i64 %880, %877                                                                                         ;L3147<7<2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40943|  %884 = sub nuw i64 %877, %880                                                                                         ;L3147<7<2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40944|  %885 = select i1 %882, i64 %883, i64 %884                                                                             ;L3147<7<2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40945|     ;; dx = i64 %885
 40946|  %886 = icmp ult i64 %879, %881                                                                                        ;L3147<8<2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40947|  %887 = sub nuw i64 %881, %879                                                                                         ;L3147<8<2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40948|  %888 = sub nuw i64 %879, %881                                                                                         ;L3147<8<2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40949|  %889 = select i1 %886, i64 %887, i64 %888                                                                             ;L3147<8<2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40950|     ;; dy = i64 %889
 40951|  %890 = mul i64 %885, %885                                                                                             ;L9<2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40952|  %891 = mul i64 %889, %889                                                                                             ;L9<2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40953|  %892 = add i64 %891, %890                                                                                             ;L9<2158<1019<138<88<40<279<146<128<52<3674<142<1019
 40954|  %893 = icmp ult i64 %892, 22500000001                                                                                 ;L1019<138<88<40<279<146<128<52<3674<142<1019
 40955|  %894 = zext i1 %893 to i64                                                                                            ;L138<88<40<279<146<128<52<3674<142<1019
 40956|  br label %895                                                                                                         ;L1018<138<88<40<279<146<128<52<3674<142<1019
 40957| 
 40958| 895: ; preds = %875, %874
 40959|  %896 = phi i64 [ %894, %875 ], [ 0, %874 ]                                                                            ;L0<138<88<40<279<146<128<52<3674<142<1019
 40961|     ;; a = i64 %868
 40962|     ;; b = i64 %896
 40963|  %897 = add i64 %896, %868                                                                                             ;L55<88<40<279<146<128<52<3674<142<1019
 40964|  br label %898                                                                                                         ;L42<279<146<128<52<3674<142<1019
 40965| 
 40966| 898: ; preds = %895, %866
 40967|  %899 = phi i64 [ %897, %895 ], [ %868, %866 ]                                                                         ;L0<279<146<128<52<3674<142<1019
 40968|     ;; acc = i64 %899
 40969|  %900 = add nuw i64 %867, 1                                                                                            ;L971<283<146<128<52<3674<142<1019
 40970|     ;; i = i64 %900
 40971|     ;; self = i64 %900
 40972|  %901 = icmp eq i64 %900, 5                                                                                            ;L284<146<128<52<3674<142<1019
 40973|  br i1 %901, label %902, label %866                                                                                    ;L284<146<128<52<3674<142<1019
 40974| 
 40975| 902: ; preds = %898
 40976|     ;; self[0..+8] = ptr %228
 40977|     ;; slice[0..+8] = ptr %228
 40978|     ;; self[8..+8] = i64 5
 40979|     ;; slice[8..+8] = i64 5
 40980|     ;; self = ptr %228
 40981|     ;; self[0..+8] = ptr %228
 40982|     ;; self[8..+8] = ptr %228
 40983|     ;; self[16..+8] = ptr %541
 40984|     ;; init = i64 0
 40987|     ;; self[0..+8] = ptr %228
 40988|     ;; iter[0..+8] = ptr %228
 40989|     ;; self[0..+8] = ptr %228
 40990|     ;; self[8..+8] = ptr %228
 40991|     ;; iter[8..+8] = ptr %228
 40992|     ;; self[8..+8] = ptr %228
 40993|     ;; self[16..+8] = ptr %541
 40994|     ;; iter[16..+8] = ptr %541
 40995|     ;; self[16..+8] = ptr %541
 40997|     ;; self[0..+8] = ptr %228
 40998|     ;; self[8..+8] = ptr %228
 40999|     ;; init = i64 0
 41000|     ;; fold = ptr %541
 41002|     ;; f = ptr %541
 41003|     ;; self[0..+8] = ptr %228
 41004|     ;; self[8..+8] = ptr %228
 41005|     ;; init = i64 0
 41006|     ;; acc = i64 0
 41007|     ;; i = i64 0
 41008|     ;; len = i64 5
 41009|  %903 = gep %541, i64 1472
 41010|  %904 = load i64, ptr %903, , !!53539
 41011|  %905 = load i64, ptr %588, , !!53539
 41012|  %906 = load i64, ptr %852, , !!53539
 41013|     ;; self = ptr %228
 41014|     ;; count = i64 0
 41015|  %907 = load ptr, ptr %228, , !!53549, !!8                                                                             ;L279<146<128<52<3674<142<1021
 41017|     ;; acc = i64 0
 41019|  %908 = icmp eq ptr %907, null                                                                                         ;L39<279<146<128<52<3674<142<1021
 41020|  br i1 %908, label %931, label %909                                                                                    ;L39<279<146<128<52<3674<142<1021
 41021| 
 41022| 909: ; preds = %902
 41023|     ;; x = ptr %907
 41025|     ;; acc = i64 0
 41026|     ;; elt = ptr %907
 41027|     ;; x = ptr %907
 41031|  %910 = gep %907, i64 1472                                                                                             ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41032|  %911 = load i64, ptr %910, , !!53549, !!8                                                                             ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41033|  %912 = icmp eq i64 %911, %904                                                                                         ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41034|  br i1 %912, label %931, label %913                                                                                    ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41035| 
 41036| 913: ; preds = %909
 41037|     ;; self = ptr %907
 41038|     ;; other = ptr %541
 41039|  %914 = gep %907, i64 1632                                                                                             ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41040|  %915 = load i64, ptr %914, , !!53549, !!8                                                                             ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41041|     ;; x1 = i64 %915
 41042|     ;; self = i64 %915
 41043|  %916 = gep %907, i64 1640                                                                                             ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41044|  %917 = load i64, ptr %916, , !!53549, !!8                                                                             ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41045|     ;; y1 = i64 %917
 41046|     ;; self = i64 %917
 41047|     ;; x2 = i64 %905
 41048|     ;; other = i64 %905
 41049|     ;; y2 = i64 %906
 41050|     ;; other = i64 %906
 41051|  %918 = icmp ult i64 %915, %905                                                                                        ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41052|  %919 = sub nuw i64 %905, %915                                                                                         ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41053|  %920 = sub nuw i64 %915, %905                                                                                         ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41054|  %921 = select i1 %918, i64 %919, i64 %920                                                                             ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41055|     ;; dx = i64 %921
 41056|  %922 = icmp ult i64 %917, %906                                                                                        ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41057|  %923 = sub nuw i64 %906, %917                                                                                         ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41058|  %924 = sub nuw i64 %917, %906                                                                                         ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41059|  %925 = select i1 %922, i64 %923, i64 %924                                                                             ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41060|     ;; dy = i64 %925
 41061|  %926 = mul i64 %921, %921                                                                                             ;L9<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41062|  %927 = mul i64 %925, %925                                                                                             ;L9<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41063|  %928 = add i64 %927, %926                                                                                             ;L9<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41064|  %929 = icmp ult i64 %928, 22500000001                                                                                 ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41065|  %930 = zext i1 %929 to i64                                                                                            ;L138<88<40<279<146<128<52<3674<142<1021
 41066|  br label %931                                                                                                         ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41067| 
 41068| 931: ; preds = %913, %909, %902
 41069|  %932 = phi i64 [ 0, %902 ], [ %930, %913 ], [ 0, %909 ]                                                               ;L0<279<146<128<52<3674<142<1021
 41070|     ;; acc = i64 %932
 41071|     ;; i = i64 1
 41072|     ;; self = ptr %228
 41073|     ;; count = i64 1
 41074|  %933 = gep %228, i64 8                                                                                                ;L656<279<146<128<52<3674<142<1021
 41075|  %934 = load ptr, ptr %933, , !!53549, !!8                                                                             ;L279<146<128<52<3674<142<1021
 41077|     ;; acc = i64 %932
 41079|  %935 = icmp eq ptr %934, null                                                                                         ;L39<279<146<128<52<3674<142<1021
 41080|  br i1 %935, label %961, label %936                                                                                    ;L39<279<146<128<52<3674<142<1021
 41081| 
 41082| 936: ; preds = %931
 41083|     ;; x = ptr %934
 41085|     ;; acc = i64 %932
 41086|     ;; elt = ptr %934
 41087|     ;; x = ptr %934
 41091|  %937 = gep %934, i64 1472                                                                                             ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41092|  %938 = load i64, ptr %937, , !!53549, !!8                                                                             ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41093|  %939 = icmp eq i64 %938, %904                                                                                         ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41094|  br i1 %939, label %958, label %940                                                                                    ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41095| 
 41096| 940: ; preds = %936
 41097|     ;; self = ptr %934
 41098|     ;; other = ptr %541
 41099|  %941 = gep %934, i64 1632                                                                                             ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41100|  %942 = load i64, ptr %941, , !!53549, !!8                                                                             ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41101|     ;; x1 = i64 %942
 41102|     ;; self = i64 %942
 41103|  %943 = gep %934, i64 1640                                                                                             ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41104|  %944 = load i64, ptr %943, , !!53549, !!8                                                                             ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41105|     ;; y1 = i64 %944
 41106|     ;; self = i64 %944
 41107|     ;; x2 = i64 %905
 41108|     ;; other = i64 %905
 41109|     ;; y2 = i64 %906
 41110|     ;; other = i64 %906
 41111|  %945 = icmp ult i64 %942, %905                                                                                        ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41112|  %946 = sub nuw i64 %905, %942                                                                                         ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41113|  %947 = sub nuw i64 %942, %905                                                                                         ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41114|  %948 = select i1 %945, i64 %946, i64 %947                                                                             ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41115|     ;; dx = i64 %948
 41116|  %949 = icmp ult i64 %944, %906                                                                                        ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41117|  %950 = sub nuw i64 %906, %944                                                                                         ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41118|  %951 = sub nuw i64 %944, %906                                                                                         ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41119|  %952 = select i1 %949, i64 %950, i64 %951                                                                             ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41120|     ;; dy = i64 %952
 41121|  %953 = mul i64 %948, %948                                                                                             ;L9<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41122|  %954 = mul i64 %952, %952                                                                                             ;L9<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41123|  %955 = add i64 %954, %953                                                                                             ;L9<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41124|  %956 = icmp ult i64 %955, 22500000001                                                                                 ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41125|  %957 = zext i1 %956 to i64                                                                                            ;L138<88<40<279<146<128<52<3674<142<1021
 41126|  br label %958                                                                                                         ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41127| 
 41128| 958: ; preds = %940, %936
 41129|  %959 = phi i64 [ %957, %940 ], [ 0, %936 ]                                                                            ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41131|     ;; a = i64 %932
 41132|     ;; b = i64 %959
 41133|  %960 = add nuw nsw i64 %959, %932                                                                                     ;L55<88<40<279<146<128<52<3674<142<1021
 41134|  br label %961                                                                                                         ;L42<279<146<128<52<3674<142<1021
 41135| 
 41136| 961: ; preds = %958, %931
 41137|  %962 = phi i64 [ %960, %958 ], [ %932, %931 ]                                                                         ;L0<279<146<128<52<3674<142<1021
 41138|     ;; acc = i64 %962
 41139|     ;; i = i64 2
 41140|     ;; self = ptr %228
 41141|     ;; count = i64 2
 41142|  %963 = gep %228, i64 16                                                                                               ;L656<279<146<128<52<3674<142<1021
 41143|  %964 = load ptr, ptr %963, , !!53549, !!8                                                                             ;L279<146<128<52<3674<142<1021
 41145|     ;; acc = i64 %962
 41147|  %965 = icmp eq ptr %964, null                                                                                         ;L39<279<146<128<52<3674<142<1021
 41148|  br i1 %965, label %991, label %966                                                                                    ;L39<279<146<128<52<3674<142<1021
 41149| 
 41150| 966: ; preds = %961
 41151|     ;; x = ptr %964
 41153|     ;; acc = i64 %962
 41154|     ;; elt = ptr %964
 41155|     ;; x = ptr %964
 41159|  %967 = gep %964, i64 1472                                                                                             ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41160|  %968 = load i64, ptr %967, , !!53549, !!8                                                                             ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41161|  %969 = icmp eq i64 %968, %904                                                                                         ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41162|  br i1 %969, label %988, label %970                                                                                    ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41163| 
 41164| 970: ; preds = %966
 41165|     ;; self = ptr %964
 41166|     ;; other = ptr %541
 41167|  %971 = gep %964, i64 1632                                                                                             ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41168|  %972 = load i64, ptr %971, , !!53549, !!8                                                                             ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41169|     ;; x1 = i64 %972
 41170|     ;; self = i64 %972
 41171|  %973 = gep %964, i64 1640                                                                                             ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41172|  %974 = load i64, ptr %973, , !!53549, !!8                                                                             ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41173|     ;; y1 = i64 %974
 41174|     ;; self = i64 %974
 41175|     ;; x2 = i64 %905
 41176|     ;; other = i64 %905
 41177|     ;; y2 = i64 %906
 41178|     ;; other = i64 %906
 41179|  %975 = icmp ult i64 %972, %905                                                                                        ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41180|  %976 = sub nuw i64 %905, %972                                                                                         ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41181|  %977 = sub nuw i64 %972, %905                                                                                         ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41182|  %978 = select i1 %975, i64 %976, i64 %977                                                                             ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41183|     ;; dx = i64 %978
 41184|  %979 = icmp ult i64 %974, %906                                                                                        ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41185|  %980 = sub nuw i64 %906, %974                                                                                         ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41186|  %981 = sub nuw i64 %974, %906                                                                                         ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41187|  %982 = select i1 %979, i64 %980, i64 %981                                                                             ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41188|     ;; dy = i64 %982
 41189|  %983 = mul i64 %978, %978                                                                                             ;L9<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41190|  %984 = mul i64 %982, %982                                                                                             ;L9<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41191|  %985 = add i64 %984, %983                                                                                             ;L9<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41192|  %986 = icmp ult i64 %985, 22500000001                                                                                 ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41193|  %987 = zext i1 %986 to i64                                                                                            ;L138<88<40<279<146<128<52<3674<142<1021
 41194|  br label %988                                                                                                         ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41195| 
 41196| 988: ; preds = %970, %966
 41197|  %989 = phi i64 [ %987, %970 ], [ 0, %966 ]                                                                            ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41199|     ;; a = i64 %962
 41200|     ;; b = i64 %989
 41201|  %990 = add nuw nsw i64 %989, %962                                                                                     ;L55<88<40<279<146<128<52<3674<142<1021
 41202|  br label %991                                                                                                         ;L42<279<146<128<52<3674<142<1021
 41203| 
 41204| 991: ; preds = %988, %961
 41205|  %992 = phi i64 [ %990, %988 ], [ %962, %961 ]                                                                         ;L0<279<146<128<52<3674<142<1021
 41206|     ;; acc = i64 %992
 41207|     ;; i = i64 3
 41208|     ;; self = ptr %228
 41209|     ;; count = i64 3
 41210|  %993 = gep %228, i64 24                                                                                               ;L656<279<146<128<52<3674<142<1021
 41211|  %994 = load ptr, ptr %993, , !!53549, !!8                                                                             ;L279<146<128<52<3674<142<1021
 41213|     ;; acc = i64 %992
 41215|  %995 = icmp eq ptr %994, null                                                                                         ;L39<279<146<128<52<3674<142<1021
 41216|  br i1 %995, label %1021, label %996                                                                                   ;L39<279<146<128<52<3674<142<1021
 41217| 
 41218| 996: ; preds = %991
 41219|     ;; x = ptr %994
 41221|     ;; acc = i64 %992
 41222|     ;; elt = ptr %994
 41223|     ;; x = ptr %994
 41227|  %997 = gep %994, i64 1472                                                                                             ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41228|  %998 = load i64, ptr %997, , !!53549, !!8                                                                             ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41229|  %999 = icmp eq i64 %998, %904                                                                                         ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41230|  br i1 %999, label %1018, label %1000                                                                                  ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41231| 
 41232| 1000: ; preds = %996
 41233|     ;; self = ptr %994
 41234|     ;; other = ptr %541
 41235|  %1001 = gep %994, i64 1632                                                                                            ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41236|  %1002 = load i64, ptr %1001, , !!53549, !!8                                                                           ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41237|     ;; x1 = i64 %1002
 41238|     ;; self = i64 %1002
 41239|  %1003 = gep %994, i64 1640                                                                                            ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41240|  %1004 = load i64, ptr %1003, , !!53549, !!8                                                                           ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41241|     ;; y1 = i64 %1004
 41242|     ;; self = i64 %1004
 41243|     ;; x2 = i64 %905
 41244|     ;; other = i64 %905
 41245|     ;; y2 = i64 %906
 41246|     ;; other = i64 %906
 41247|  %1005 = icmp ult i64 %1002, %905                                                                                      ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41248|  %1006 = sub nuw i64 %905, %1002                                                                                       ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41249|  %1007 = sub nuw i64 %1002, %905                                                                                       ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41250|  %1008 = select i1 %1005, i64 %1006, i64 %1007                                                                         ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41251|     ;; dx = i64 %1008
 41252|  %1009 = icmp ult i64 %1004, %906                                                                                      ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41253|  %1010 = sub nuw i64 %906, %1004                                                                                       ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41254|  %1011 = sub nuw i64 %1004, %906                                                                                       ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41255|  %1012 = select i1 %1009, i64 %1010, i64 %1011                                                                         ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41256|     ;; dy = i64 %1012
 41257|  %1013 = mul i64 %1008, %1008                                                                                          ;L9<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41258|  %1014 = mul i64 %1012, %1012                                                                                          ;L9<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41259|  %1015 = add i64 %1014, %1013                                                                                          ;L9<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41260|  %1016 = icmp ult i64 %1015, 22500000001                                                                               ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41261|  %1017 = zext i1 %1016 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<1021
 41262|  br label %1018                                                                                                        ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41263| 
 41264| 1018: ; preds = %1000, %996
 41265|  %1019 = phi i64 [ %1017, %1000 ], [ 0, %996 ]                                                                         ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41267|     ;; a = i64 %992
 41268|     ;; b = i64 %1019
 41269|  %1020 = add nuw nsw i64 %1019, %992                                                                                   ;L55<88<40<279<146<128<52<3674<142<1021
 41270|  br label %1021                                                                                                        ;L42<279<146<128<52<3674<142<1021
 41271| 
 41272| 1021: ; preds = %1018, %991
 41273|  %1022 = phi i64 [ %1020, %1018 ], [ %992, %991 ]                                                                      ;L0<279<146<128<52<3674<142<1021
 41274|     ;; acc = i64 %1022
 41275|     ;; i = i64 4
 41276|     ;; self = ptr %228
 41277|     ;; count = i64 4
 41278|  %1023 = gep %228, i64 32                                                                                              ;L656<279<146<128<52<3674<142<1021
 41279|  %1024 = load ptr, ptr %1023, , !!53549, !!8                                                                           ;L279<146<128<52<3674<142<1021
 41281|     ;; acc = i64 %1022
 41283|  %1025 = icmp eq ptr %1024, null                                                                                       ;L39<279<146<128<52<3674<142<1021
 41284|  br i1 %1025, label %1051, label %1026                                                                                 ;L39<279<146<128<52<3674<142<1021
 41285| 
 41286| 1026: ; preds = %1021
 41287|     ;; x = ptr %1024
 41289|     ;; acc = i64 %1022
 41290|     ;; elt = ptr %1024
 41291|     ;; x = ptr %1024
 41295|  %1027 = gep %1024, i64 1472                                                                                           ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41296|  %1028 = load i64, ptr %1027, , !!53549, !!8                                                                           ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41297|  %1029 = icmp eq i64 %1028, %904                                                                                       ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41298|  br i1 %1029, label %1048, label %1030                                                                                 ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41299| 
 41300| 1030: ; preds = %1026
 41301|     ;; self = ptr %1024
 41302|     ;; other = ptr %541
 41303|  %1031 = gep %1024, i64 1632                                                                                           ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41304|  %1032 = load i64, ptr %1031, , !!53549, !!8                                                                           ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41305|     ;; x1 = i64 %1032
 41306|     ;; self = i64 %1032
 41307|  %1033 = gep %1024, i64 1640                                                                                           ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41308|  %1034 = load i64, ptr %1033, , !!53549, !!8                                                                           ;L2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41309|     ;; y1 = i64 %1034
 41310|     ;; self = i64 %1034
 41311|     ;; x2 = i64 %905
 41312|     ;; other = i64 %905
 41313|     ;; y2 = i64 %906
 41314|     ;; other = i64 %906
 41315|  %1035 = icmp ult i64 %1032, %905                                                                                      ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41316|  %1036 = sub nuw i64 %905, %1032                                                                                       ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41317|  %1037 = sub nuw i64 %1032, %905                                                                                       ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41318|  %1038 = select i1 %1035, i64 %1036, i64 %1037                                                                         ;L3147<7<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41319|     ;; dx = i64 %1038
 41320|  %1039 = icmp ult i64 %1034, %906                                                                                      ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41321|  %1040 = sub nuw i64 %906, %1034                                                                                       ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41322|  %1041 = sub nuw i64 %1034, %906                                                                                       ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41323|  %1042 = select i1 %1039, i64 %1040, i64 %1041                                                                         ;L3147<8<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41324|     ;; dy = i64 %1042
 41325|  %1043 = mul i64 %1038, %1038                                                                                          ;L9<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41326|  %1044 = mul i64 %1042, %1042                                                                                          ;L9<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41327|  %1045 = add i64 %1044, %1043                                                                                          ;L9<2158<1021<138<88<40<279<146<128<52<3674<142<1021
 41328|  %1046 = icmp ult i64 %1045, 22500000001                                                                               ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41329|  %1047 = zext i1 %1046 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<1021
 41330|  br label %1048                                                                                                        ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41331| 
 41332| 1048: ; preds = %1030, %1026
 41333|  %1049 = phi i64 [ %1047, %1030 ], [ 0, %1026 ]                                                                        ;L1021<138<88<40<279<146<128<52<3674<142<1021
 41335|     ;; a = i64 %1022
 41336|     ;; b = i64 %1049
 41337|  %1050 = add nuw nsw i64 %1049, %1022                                                                                  ;L55<88<40<279<146<128<52<3674<142<1021
 41338|  br label %1051                                                                                                        ;L42<279<146<128<52<3674<142<1021
 41339| 
 41340| 1051: ; preds = %1048, %1021
 41341|  %1052 = phi i64 [ %1050, %1048 ], [ %1022, %1021 ]                                                                    ;L0<279<146<128<52<3674<142<1021
 41342|     ;; acc = i64 %1052
 41343|     ;; i = i64 5
 41344|  %1053 = load i8, ptr %425, , !!8                                                                                      ;L1023
 41345|  %1054 = gep %1, i64 10656                                                                                             ;L1024
 41346|  %1055 = load i64, ptr %1054, , !!8                                                                                    ;L1024
 41347|  %1056 = icmp eq i64 %1055, -1                                                                                         ;L1024
 41348|  br i1 %1056, label %1059, label %1057                                                                                 ;L1024
 41349| 
 41350| 1057: ; preds = %1051
 41351|  %1058 = invoke i64 %58(ptr %54)
 41352|  to label %1067 unwind label %846                                                                                      ;L1025
 41353| 
 41354| 1059: ; preds = %1067, %1051
 41355|  %1060 = phi i64 [ %1069, %1067 ], [ -1, %1051 ]                                                                       ;L0
 41356|     ;; self = ptr %89
 41357|  %1061 = load i8, ptr %210, , !!53662, !!8                                                                             ;L309<1026
 41358|  %1062 = icmp ne i8 %1061, 10                                                                                          ;L309<1026
 41359|  call void @llvm.assume(i1 %1062)                                                                                      ;L309<1026
 41360|  %1063 = add nsw i8 %1061, -3                                                                                          ;L309<1026
 41361|  %1064 = icmp samesign ugt i8 %1061, 2                                                                                 ;L309<1026
 41362|  %1065 = select i1 %1064, i8 %1063, i8 7                                                                               ;L309<1026
 41363|  switch i8 %1065, label %1066 [
 41364|  i8 16, label %1078
 41365|  i8 15, label %1070
 41366|  i8 2, label %1070
 41367|  i8 3, label %1070
 41368|  i8 4, label %1078
 41369|  i8 14, label %1070
 41370|  i8 6, label %1078
 41371|  i8 7, label %1078
 41372|  i8 8, label %1078
 41373|  i8 9, label %1078
 41374|  i8 10, label %1070
 41375|  i8 11, label %1071
 41376|  i8 12, label %1070
 41377|  i8 13, label %1070
 41378|  i8 0, label %1078
 41379|  i8 1, label %1078
 41380|  i8 5, label %1078
 41381|  ]                                                                                                                     ;L309<1026
 41382| 
 41383| 1066: ; preds = %1059
 41384|  unreachable                                                                                                           ;L309<1026
 41385| 
 41386| 1067: ; preds = %1057
 41387|     ;; self = i64 %1058
 41388|  %1068 = load i64, ptr %1054, , !!8                                                                                    ;L1025
 41389|     ;; rhs = i64 %1068
 41390|  %1069 = call i64 @llvm.usub.sat.i64(i64 %1058, i64 %1068)                                                             ;L2472<1025
 41391|  br label %1059                                                                                                        ;L1024
 41392| 
 41393| 1070: ; preds = %1059, %1059, %1059, %1059, %1059, %1059, %1059
 41394|     ;; self[0..+8] = i64 1
 41396|     ;; f = ptr %89
 41397|  br label %1071                                                                                                        ;L1651<1027
 41398| 
 41399| 1071: ; preds = %1070, %1059
 41400|  %1072 = phi i64 [ 10336, %1070 ], [ 10424, %1059 ]
 41401|  %1073 = gep %1, i64 %1072                                                                                             ;L0
 41402|  %1074 = load i64, ptr %1073, , !!8, !!8                                                                               ;L0
 41403|     ;; id = i64 %1074
 41404|  %1075 = gep %56, i64 496                                                                                              ;L1032
 41405|  %1076 = load ptr, ptr %1075, , !!8                                                                                    ;L1032
 41406|  %1077 = invoke ptr %1076(ptr %54, i64 %1074)
 41407|  to label %1110 unwind label %846                                                                                      ;L1032
 41408| 
 41409| 1078: ; preds = %1123, %1122, %1121, %1117, %1116, %1115, %1112, %1110, %1059, %1059, %1059, %1059, %1059, %1059, %1059, %1059, %1059
 41410|  %1079 = phi i64 [ 9, %1117 ], [ 9, %1121 ], [ 6, %1110 ], [ 6, %1115 ], [ 9, %1116 ], [ 9, %1112 ], [ 6, %1122 ], [ 9, %1123 ], [ 0, %1059 ], [ 0, %1059 ], [ 0, %1059 ], [ 0, %1059 ], [ 0, %1059 ], [ 0, %1059 ], [ 0, %1059 ], [ 0, %1059 ], [ 0, %1059 ] ;L0
 41411|  %1080 = phi ptr [ @anon.b0108feec1ab8ff62b7a37c1a95c251f.162, %1117 ], [ @anon.b0108feec1ab8ff62b7a37c1a95c251f.163, %1121 ], [ @anon.b0108feec1ab8ff62b7a37c1a95c251f.158, %1110 ], [ @anon.b0108feec1ab8ff62b7a37c1a95c251f.160, %1115 ], [ @anon.b0108feec1ab8ff62b7a37c1a95c251f.161, %1116 ], [ @anon.b0108feec1ab8ff62b7a37c1a95c251f.159, %1112 ], [ @anon.b0108feec1ab8ff62b7a37c1a95c251f.165, %1122 ], [ @anon.b0108feec1ab8ff62b7a37c1a95c251f.164, %1123 ], [ inttoptr (i64 1 to ptr), %1059 ], [ inttoptr (i64 1 to ptr), %1059 ], [ inttoptr (i64 1 to ptr), %1059 ], [ inttoptr (i64 1 to ptr), %1059 ], [ inttoptr (i64 1 to ptr), %1059 ], [ inttoptr (i64 1 to ptr), %1059 ], [ inttoptr (i64 1 to ptr), %1059 ], [ inttoptr (i64 1 to ptr), %1059 ], [ inttoptr (i64 1 to ptr), %1059 ] ;L0
 41412|  %1081 = gep %24, i64 136                                                                                              ;L1004
 41413|  store i64 %634, ptr %1081,                                                                                            ;L1004
 41414|  %1082 = gep %24, i64 144                                                                                              ;L1004
 41415|  store i64 %221, ptr %1082,                                                                                            ;L1004
 41416|  %1083 = gep %24, i64 200                                                                                              ;L1004
 41417|  store i32 %225, ptr %1083,                                                                                            ;L1004
 41418|  %1084 = gep %24, i64 206                                                                                              ;L1004
 41419|  store i8 %580, ptr %1084,                                                                                             ;L1004
 41420|  %1085 = gep %24, i64 24                                                                                               ;L1004
 41421|  call void @llvm.memcpy.p0.p0.i64(ptr %1085, ptr %23, i64 24, i1 false)                                                ;L1004
 41422|  %1086 = gep %24, i64 48                                                                                               ;L1004
 41423|  call void @llvm.memcpy.p0.p0.i64(ptr %1086, ptr %22, i64 24, i1 false)                                                ;L1004
 41424|  %1087 = gep %24, i64 72                                                                                               ;L1004
 41425|  call void @llvm.memcpy.p0.p0.i64(ptr %1087, ptr %21, i64 24, i1 false)                                                ;L1004
 41426|  %1088 = gep %24, i64 96                                                                                               ;L1004
 41427|  call void @llvm.memcpy.p0.p0.i64(ptr %1088, ptr %17, i64 24, i1 false)                                                ;L1004
 41428|  store i64 %851, ptr %24,                                                                                              ;L1004
 41429|  %1089 = gep %24, i64 8                                                                                                ;L1004
 41430|  store i64 %849, ptr %1089,                                                                                            ;L1004
 41431|  %1090 = gep %24, i64 16                                                                                               ;L1004
 41432|  store i64 %850, ptr %1090,                                                                                            ;L1004
 41433|  %1091 = gep %24, i64 152                                                                                              ;L1004
 41434|  store i64 %589, ptr %1091,                                                                                            ;L1004
 41435|  %1092 = gep %24, i64 160                                                                                              ;L1004
 41436|  store i64 %853, ptr %1092,                                                                                            ;L1004
 41437|  %1093 = gep %24, i64 207                                                                                              ;L1004
 41438|  store i8 %632, ptr %1093,                                                                                             ;L1004
 41439|  %1094 = gep %24, i64 168                                                                                              ;L1004
 41440|  store i64 %860, ptr %1094,                                                                                            ;L1004
 41441|  %1095 = gep %24, i64 176                                                                                              ;L1004
 41442|  store i64 %899, ptr %1095,                                                                                            ;L1004
 41443|  %1096 = gep %24, i64 184                                                                                              ;L1004
 41444|  store i64 %1052, ptr %1096,                                                                                           ;L1004
 41445|  %1097 = gep %24, i64 204                                                                                              ;L1004
 41446|  store i8 %631, ptr %1097,                                                                                             ;L1004
 41447|  %1098 = gep %24, i64 205                                                                                              ;L1004
 41448|  store i8 %1053, ptr %1098,                                                                                            ;L1004
 41449|  %1099 = gep %24, i64 192                                                                                              ;L1004
 41450|  store i64 %1060, ptr %1099,                                                                                           ;L1004
 41451|  %1100 = gep %24, i64 120                                                                                              ;L1004
 41452|  store ptr %1080, ptr %1100,                                                                                           ;L1004
 41453|  %1101 = gep %24, i64 128                                                                                              ;L1004
 41454|  store i64 %1079, ptr %1101,                                                                                           ;L1004
 41459|     ;; self = ptr %633
 41460|     ;; self = ptr %633
 41461|     ;; value = ptr %24
 41463|     ;; elem_size = i64 208
 41464|  %1102 = gep %1, i64 7560                                                                                              ;L1037<1004<1004
 41465|  %1103 = load i64, ptr %1102, , !!53738, !!8                                                                           ;L1037<1004<1004
 41466|     ;; len = i64 %1103
 41467|     ;; count = i64 %1103
 41468|     ;; self = ptr %633
 41469|  %1104 = load i64, ptr %633, , !!53738, !!8                                                                            ;L619<309<1040<1004<1004
 41470|  %1105 = icmp eq i64 %1103, %1104                                                                                      ;L1040<1004<1004
 41471|  br i1 %1105, label %1106, label %1124                                                                                 ;L1040<1004<1004
 41472| 
 41473| 1106: ; preds = %1078
 41474|  invoke void @ai::StayEventE8grow_oneBO_(ptr %633)
 41475|  to label %1124 unwind label %1107, !!53738                                                                            ;L1041<1004<1004
 41476| 
 41477| 1107: ; preds = %1106
 41478|  %1108 = cleanuppad within none []
 41479|  invoke fastcc void @core::ptr9drop_glueNtCshdEBA0ozCnw_7game_ai9StayEventEBD_(ptr %24) #34 [ "funclet"(token %1108) ]
 41480|  to label %1109 unwind label %97                                                                                       ;L1050<1004<1004
 41481| 
 41482| 1109: ; preds = %1107
 41483|  cleanupret from %1108 unwind label %97
 41484| 
 41485| 1110: ; preds = %1071
 41486|  %1111 = icmp eq ptr %1077, null                                                                                       ;L1032
 41487|  br i1 %1111, label %1078, label %1112                                                                                 ;L1032
 41488| 
 41489| 1112: ; preds = %1110
 41490|     ;; self = ptr %1077
 41491|  %1113 = gep %1077, i64 104                                                                                            ;L1404<1034
 41492|  %1114 = load i64, ptr %1113, , !!8                                                                                    ;L1404<1034
 41493|  switch i64 %1114, label %1121 [
 41494|  i64 13, label %1078
 41495|  i64 5, label %1115
 41496|  i64 6, label %1116
 41497|  i64 4, label %1117
 41498|  i64 2, label %1122
 41499|  i64 3, label %1123
 41500|  ]                                                                                                                     ;L1034
 41501| 
 41502| 1115: ; preds = %1112
 41503|  br label %1078                                                                                                        ;L1035
 41504| 
 41505| 1116: ; preds = %1112
 41506|  br label %1078                                                                                                        ;L1036
 41507| 
 41508| 1117: ; preds = %1112
 41509|  %1118 = gep %1077, i64 152                                                                                            ;L1378<1037
 41510|  %1119 = load i64, ptr %1118, , !!8                                                                                    ;L1378<1037
 41511|  %1120 = icmp ult i64 %1119, 2                                                                                         ;L1378<1037
 41512|  br i1 %1120, label %1078, label %1121                                                                                 ;L1378<1037
 41513| 
 41514| 1121: ; preds = %1117, %1112
 41515|  br label %1078                                                                                                        ;L1040
 41516| 
 41517| 1122: ; preds = %1112
 41518|  br label %1078                                                                                                        ;L1038
 41519| 
 41520| 1123: ; preds = %1112
 41521|  br label %1078                                                                                                        ;L1039
 41522| 
 41523| 1124: ; preds = %1106, %1078
 41524|  %1125 = gep %1, i64 7552                                                                                              ;L614<609<296<2052<1044<1004<1004
 41525|  %1126 = load ptr, ptr %1125, , !!53738, !!8, !!8                                                                      ;L614<609<296<2052<1044<1004<1004
 41526|     ;; self = ptr %1126
 41527|  %1127 = getelementptr { { i64, [2 x i64] }, { { { { i64, ptr, {} }, {} }, i64 } }, { { { { i64, ptr, {} }, {} }, i64 } }, { { { { i64, ptr, {} }, {} }, i64 } }, { { { { i64, ptr, {} }, {} }, i64 } }, { ptr, i64 }, i64, i64, { i64, i64 }, i64, i64, i64, i64, i32, i8, i8, i8, i8 }, ptr %1126, i64 %1103 ;L961<1044<1004<1004
 41528|     ;; end = ptr %1127
 41529|     ;; dst = ptr %1127
 41530|  call void @llvm.memcpy.p0.p0.i64(ptr %1127, ptr %24, i64 208, i1 false)                                               ;L1933<1045<1004<1004
 41531|  %1128 = add i64 %1103, 1                                                                                              ;L1046<1004<1004
 41532|  store i64 %1128, ptr %1102, , !!53738                                                                                 ;L1046<1004<1004
 41534|  br label %1129                                                                                                        ;L994
 41535| 
 41536| 1129: ; preds = %1124, %444
 41537|     ;; self = ptr %3
 41538|  br i1 %231, label %1130, label %1133                                                                                  ;L1048
 41539| 
 41540| 1130: ; preds = %1129, %573
 41541|  %1131 = load ptr, ptr %229, , !!8, !!8                                                                                ;L1048
 41542|     ;; champ = ptr %1131
 41543|  %1132 = invoke i64 %58(ptr %54)
 41544|  to label %1186 unwind label %97                                                                                       ;L1049
 41545| 
 41546| 1133: ; preds = %1324, %1319, %1217, %1210, %1204, %1129, %480
 41547|  %1134 = phi ptr [ %482, %480 ], [ %446, %1204 ], [ %446, %1319 ], [ %446, %1324 ], [ %446, %1210 ], [ %446, %1217 ], [ %446, %1129 ]
 41549|     ;; self = ptr %1
 41552|  %1135 = load ptr, ptr %53, , !!53794, !!8, !!8                                                                        ;L1089<1080
 41553|  %1136 = load ptr, ptr %55, , !!53794, !!8, !!8                                                                        ;L1089<1080
 41554|  %1137 = gep %1136, i64 40                                                                                             ;L1089<1080
 41555|  %1138 = load ptr, ptr %1137, , !!53794, !!8                                                                           ;L1089<1080
 41556|  %1139 = invoke i64 %1138(ptr %1135)
 41557|  to label %1140 unwind label %97                                                                                       ;L1089<1080
 41558| 
 41559| 1140: ; preds = %1133
 41560|     ;; tick = i64 %1139
 41561|     ;; self = i64 %1139
 41562|  %1141 = gep %1134, i64 8                                                                                              ;L1090<1080
 41563|  %1142 = load ptr, ptr %1141, , !!53794, !!8, !!8                                                                      ;L1090<1080
 41564|  %1143 = gep %1142, i64 4856                                                                                           ;L1090<1080
 41565|  %1144 = load i64, ptr %1143, , !!53794, !!8                                                                           ;L1090<1080
 41566|  %1145 = shl i64 %1144, 1                                                                                              ;L1090<1080
 41567|     ;; abandon_threshold = i64 %1145
 41568|  %1146 = load i8, ptr %210, , !!8                                                                                      ;L1091<1080
 41569|  %1147 = icmp ne i8 %1146, 10                                                                                          ;L1091<1080
 41570|  call void @llvm.assume(i1 %1147)                                                                                      ;L1091<1080
 41571|  %1148 = icmp eq i8 %1146, 14                                                                                          ;L1091<1080
 41572|  br i1 %1148, label %1149, label %1328                                                                                 ;L1091<1080
 41573| 
 41574| 1149: ; preds = %1140
 41575|     ;; t = ptr %1
 41576|     ;; self = ptr %1
 41577|     ;; self = ptr %1
 41578|     ;; self = ptr %1
 41579|  %1150 = gep %1, i64 10424                                                                                             ;L428<1092<1080
 41580|  %1151 = load i64, ptr %1150, , !!8                                                                                    ;L428<1092<1080
 41581|     ;; tid = i64 %1151
 41582|     ;; self = ptr %1
 41583|  %1152 = gep %1, i64 10477                                                                                             ;L2439<424<1093<1080
 41584|  %1153 = load i8, ptr %1152, , !!8                                                                                     ;L2439<424<1093<1080
 41585|  %1154 = and i8 %1153, 1                                                                                               ;L2439<424<1093<1080
 41586|  %1155 = icmp eq i8 %1154, 0                                                                                           ;L2439<424<1093<1080
 41587|  %1156 = gep %1, i64 10536                                                                                             ;L0<1080
 41588|  %1157 = load i64, ptr %1156, , !!8                                                                                    ;L0<1080
 41589|  %1158 = icmp eq i64 %1157, %1151                                                                                      ;L0<1080
 41590|  br i1 %1155, label %1159, label %1160                                                                                 ;L2439<424<1093<1080
 41591| 
 41592| 1159: ; preds = %1149
 41593|  br i1 %1158, label %1166, label %1328                                                                                 ;L1104<1080
 41594| 
 41595| 1160: ; preds = %1149
 41596|  %1161 = gep %1, i64 10552                                                                                             ;L1094<1080
 41597|  %1162 = load i64, ptr %1161,                                                                                          ;L1094<1080
 41598|  %1163 = icmp uge i64 %1139, %1162                                                                                     ;L1094<1080
 41599|  %1164 = select i1 %1158, i1 %1163, i1 false                                                                           ;L1094<1080
 41600|  %1165 = gep %1, i64 10544                                                                                             ;L0<1080
 41601|  br i1 %1164, label %1174, label %1173                                                                                 ;L1094<1080
 41602| 
 41603| 1166: ; preds = %1159
 41604|  %1167 = gep %1, i64 10552                                                                                             ;L1106<1080
 41605|  %1168 = load i64, ptr %1167, , !!8                                                                                    ;L1106<1080
 41606|     ;; rhs = i64 %1168
 41607|  %1169 = call i64 @llvm.usub.sat.i64(i64 %1139, i64 %1168)                                                             ;L2472<1106<1080
 41608|     ;; dec = i64 %1169
 41609|     ;; rhs = i64 %1169
 41610|  %1170 = gep %1, i64 10544                                                                                             ;L1107<1080
 41611|  %1171 = load i64, ptr %1170, , !!8                                                                                    ;L1107<1080
 41612|     ;; self = i64 %1171
 41613|  %1172 = call i64 @llvm.usub.sat.i64(i64 %1171, i64 %1169)                                                             ;L2472<1107<1080
 41614|  store i64 %1172, ptr %1170,                                                                                           ;L1107<1080
 41615|  store i64 %1139, ptr %1167,                                                                                           ;L1108<1080
 41616|  br label %1328                                                                                                        ;L1104<1080
 41617| 
 41618| 1173: ; preds = %1160
 41619|  store i64 %1151, ptr %1156,                                                                                           ;L1097<1080
 41620|  br label %1178                                                                                                        ;L1094<1080
 41621| 
 41622| 1174: ; preds = %1160
 41623|  %1175 = sub nuw i64 %1139, %1162                                                                                      ;L1095<1080
 41624|  %1176 = load i64, ptr %1165, , !!8                                                                                    ;L1095<1080
 41625|  %1177 = add i64 %1175, %1176                                                                                          ;L1095<1080
 41626|  br label %1178                                                                                                        ;L1094<1080
 41627| 
 41628| 1178: ; preds = %1174, %1173
 41629|  %1179 = phi i64 [ %1177, %1174 ], [ 0, %1173 ]
 41630|  store i64 %1179, ptr %1165,                                                                                           ;L0<1080
 41631|  store i64 %1139, ptr %1161,                                                                                           ;L1100<1080
 41632|  %1180 = icmp ult i64 %1179, %1145                                                                                     ;L1101<1080
 41633|  %1181 = gep %1, i64 10474
 41634|  %1182 = load i8, ptr %1181,
 41635|  %1183 = trunc nuw i8 %1182 to i1
 41636|  %1184 = select i1 %1180, i1 true, i1 %1183                                                                            ;L1101<1080
 41637|  br i1 %1184, label %1328, label %1185                                                                                 ;L1101<1080
 41638| 
 41639| 1185: ; preds = %1178
 41640|  store i8 1, ptr %1181,                                                                                                ;L433<1102<1080
 41641|  br label %1328                                                                                                        ;L1101<1080
 41642| 
 41643| 1186: ; preds = %1130
 41644|     ;; tick = i64 %1132
 41645|     ;; self = i64 %1132
 41646|  %1187 = gep %1131, i64 1632                                                                                           ;L1050
 41647|  %1188 = load i64, ptr %1187, , !!8                                                                                    ;L1050
 41648|  %1189 = gep %1131, i64 1640                                                                                           ;L1050
 41649|  %1190 = load i64, ptr %1189, , !!8                                                                                    ;L1050
 41650|  %1191 = gep %1, i64 1072                                                                                              ;L1050
 41651|  %1192 = load i64, ptr %1191, , !!8                                                                                    ;L1050
 41652|  %1193 = gep %1, i64 1080                                                                                              ;L1050
 41653|  %1194 = load i64, ptr %1193, , !!8                                                                                    ;L1050
 41654|  %1195 = invoke i64 @gc::utils8distance(i64 %1188, i64 %1190, i64 %1192, i64 %1194)
 41655|  to label %1196 unwind label %97                                                                                       ;L1050
 41656| 
 41657| 1196: ; preds = %1186
 41658|  %1197 = icmp ugt i64 %1195, 8000                                                                                      ;L1050
 41659|     ;; moved = i1 %1197
 41660|     ;; self = ptr %47
 41661|  %1198 = icmp ne i64 %442, -1                                                                                          ;L633<1051
 41662|  %1199 = or i1 %1197, %1198                                                                                            ;L1051
 41663|  %1200 = gep %1, i64 10592                                                                                             ;L1051
 41664|  %1201 = load i64, ptr %1200,                                                                                          ;L1051
 41665|  %1202 = icmp eq i64 %1201, 0                                                                                          ;L1051
 41666|  %1203 = select i1 %1199, i1 true, i1 %1202                                                                            ;L1051
 41667|  br i1 %1203, label %1204, label %1206                                                                                 ;L1051
 41668| 
 41669| 1204: ; preds = %1196
 41670|  store i64 %1132, ptr %1200,                                                                                           ;L1052
 41671|  store i64 %1188, ptr %1191,                                                                                           ;L1053
 41672|  store i64 %1190, ptr %1193,                                                                                           ;L1053
 41673|  %1205 = gep %1, i64 10699                                                                                             ;L1054
 41674|  store i8 0, ptr %1205,                                                                                                ;L1054
 41675|  br label %1133                                                                                                        ;L1051
 41676| 
 41677| 1206: ; preds = %1196
 41678|  %1207 = gep %1, i64 10699                                                                                             ;L1055
 41679|  %1208 = load i8, ptr %1207, , !!8                                                                                     ;L1055
 41680|  %1209 = trunc nuw i8 %1208 to i1                                                                                      ;L1055
 41681|  br i1 %1209, label %1217, label %1210                                                                                 ;L1055
 41682| 
 41683| 1210: ; preds = %1206
 41684|     ;; rhs = i64 %1201
 41685|  %1211 = call i64 @llvm.usub.sat.i64(i64 %1132, i64 %1201)                                                             ;L2472<1057
 41686|  %1212 = gep %446, i64 8                                                                                               ;L1057
 41687|  %1213 = load ptr, ptr %1212, , !!8, !!8                                                                               ;L1057
 41688|  %1214 = gep %1213, i64 4856                                                                                           ;L1057
 41689|  %1215 = load i64, ptr %1214, , !!8                                                                                    ;L1057
 41690|  %1216 = icmp ult i64 %1211, %1215                                                                                     ;L1057
 41691|  br i1 %1216, label %1133, label %1221                                                                                 ;L1057
 41692| 
 41693| 1217: ; preds = %1206
 41694|  %1218 = gep %1, i64 10616                                                                                             ;L1056
 41695|  %1219 = load i64, ptr %1218, , !!8                                                                                    ;L1056
 41696|  %1220 = add i64 %1219, 1                                                                                              ;L1056
 41697|  store i64 %1220, ptr %1218,                                                                                           ;L1056
 41698|  br label %1133                                                                                                        ;L1055
 41699| 
 41700| 1221: ; preds = %1210
 41701|  store i8 1, ptr %1207,                                                                                                ;L1058
 41702|  %1222 = load i8, ptr %210, , !!8                                                                                      ;L1059
 41704|  %1223 = icmp ne i8 %1222, 10                                                                                          ;L482<1059
 41705|  call void @llvm.assume(i1 %1223)                                                                                      ;L482<1059
 41706|  %1224 = add nsw i8 %1222, -3                                                                                          ;L482<1059
 41707|  %1225 = icmp samesign ugt i8 %1222, 2                                                                                 ;L482<1059
 41708|  %1226 = select i1 %1225, i8 %1224, i8 7                                                                               ;L482<1059
 41709|  switch i8 %1226, label %1227 [
 41710|  i8 0, label %1233
 41711|  i8 1, label %1233
 41712|  i8 2, label %1228
 41713|  i8 3, label %1228
 41714|  i8 4, label %1228
 41715|  i8 5, label %1233
 41716|  i8 6, label %1228
 41717|  i8 7, label %1228
 41718|  i8 8, label %1228
 41719|  i8 9, label %1228
 41720|  i8 10, label %1229
 41721|  i8 11, label %1230
 41722|  i8 12, label %1231
 41723|  i8 13, label %1231
 41724|  i8 14, label %1231
 41725|  i8 15, label %1231
 41726|  i8 16, label %1232
 41727|  ]                                                                                                                     ;L482<1059
 41728| 
 41729| 1227: ; preds = %1221
 41730|  unreachable                                                                                                           ;L482<1059
 41731| 
 41732| 1228: ; preds = %1221, %1221, %1221, %1221, %1221, %1221, %1221
 41733|  br label %1233                                                                                                        ;L487<1059
 41734| 
 41735| 1229: ; preds = %1221
 41736|  br label %1233                                                                                                        ;L490<1059
 41737| 
 41738| 1230: ; preds = %1221
 41739|  br label %1233                                                                                                        ;L483<1059
 41740| 
 41741| 1231: ; preds = %1221, %1221, %1221, %1221
 41742|  br label %1233                                                                                                        ;L489<1059
 41743| 
 41744| 1232: ; preds = %1221
 41745|  br label %1233                                                                                                        ;L491<1059
 41746| 
 41747| 1233: ; preds = %1232, %1231, %1230, %1229, %1228, %1221, %1221, %1221
 41748|  %1234 = phi i64 [ 5, %1232 ], [ 2, %1228 ], [ 4, %1229 ], [ 0, %1230 ], [ 3, %1231 ], [ 1, %1221 ], [ 1, %1221 ], [ 1, %1221 ] ;L0<1059
 41749|     ;; cls = i64 %1234
 41750|  %1235 = gep %1, i64 1088                                                                                              ;L1060
 41751|  %1236 = getelementptr i64, ptr %1235, i64 %1234                                                                       ;L1060
 41752|  %1237 = load i64, ptr %1236, , !!8                                                                                    ;L1060
 41753|  %1238 = add i64 %1237, 1                                                                                              ;L1060
 41754|  store i64 %1238, ptr %1236,                                                                                           ;L1060
 41755|  %1239 = gep %1, i64 2840                                                                                              ;L1061
 41756|  %1240 = load i64, ptr %1239, , !!8                                                                                    ;L1061
 41758|  %1241 = icmp ne i64 %1240, 6                                                                                          ;L122<1061
 41759|  call void @llvm.assume(i1 %1241)                                                                                      ;L122<1061
 41760|  %1242 = add nsw i64 %1240, -2                                                                                         ;L122<1061
 41761|  %1243 = icmp samesign ugt i64 %1240, 1                                                                                ;L122<1061
 41762|  %1244 = select i1 %1243, i64 %1242, i64 4                                                                             ;L122<1061
 41763|  switch i64 %1244, label %1250 [
 41764|  i64 0, label %1245
 41765|  i64 1, label %1246
 41766|  i64 2, label %1246
 41767|  i64 3, label %1247
 41768|  i64 4, label %1247
 41769|  i64 5, label %1248
 41770|  i64 6, label %1245
 41771|  i64 7, label %1247
 41772|  i64 8, label %1249
 41773|  i64 9, label %1249
 41774|  ]                                                                                                                     ;L122<1061
 41775| 
 41776| 1245: ; preds = %1233, %1233
 41777|  br label %1250                                                                                                        ;L126<1061
 41778| 
 41779| 1246: ; preds = %1233, %1233
 41780|  br label %1250                                                                                                        ;L124<1061
 41781| 
 41782| 1247: ; preds = %1233, %1233, %1233
 41783|  br label %1250                                                                                                        ;L123<1061
 41784| 
 41785| 1248: ; preds = %1233
 41786|  br label %1250                                                                                                        ;L125<1061
 41787| 
 41788| 1249: ; preds = %1233, %1233
 41789|  br label %1250                                                                                                        ;L127<1061
 41790| 
 41791| 1250: ; preds = %1249, %1248, %1247, %1246, %1245, %1233
 41792|  %1251 = phi i64 [ 4, %1249 ], [ 1, %1246 ], [ 2, %1248 ], [ 3, %1245 ], [ 0, %1247 ], [ 5, %1233 ]                    ;L0<1061
 41793|  %1252 = gep %1, i64 1136                                                                                              ;L1061
 41794|  %1253 = getelementptr i64, ptr %1252, i64 %1251                                                                       ;L1061
 41795|  %1254 = load i64, ptr %1253, , !!8                                                                                    ;L1061
 41796|  %1255 = add i64 %1254, 1                                                                                              ;L1061
 41797|  store i64 %1255, ptr %1253,                                                                                           ;L1061
 41799|  %1256 = gep %446, i64 32                                                                                              ;L1063
 41800|  %1257 = load ptr, ptr %1256, , !!8, !!8                                                                               ;L1063
 41801|     ;; self = ptr %1257
 41802|  %1258 = gep %1257, i64 28016                                                                                          ;L235<1063
 41803|  %1259 = gepS %1258, i64 %221                                                                                          ;L235<1063
 41804|  %1260 = load i64, ptr %1259, , !!8                                                                                    ;L235<1063
 41805|     ;; flx = i64 %1260
 41806|  %1261 = gep %1259, i64 8                                                                                              ;L235<1063
 41807|  %1262 = load i64, ptr %1261, , !!8                                                                                    ;L235<1063
 41808|     ;; fly = i64 %1262
 41809|  %1263 = gep %1259, i64 16                                                                                             ;L235<1063
 41810|  %1264 = load i64, ptr %1263, , !!8                                                                                    ;L235<1063
 41811|     ;; frx = i64 %1264
 41812|  %1265 = gep %1259, i64 24                                                                                             ;L235<1063
 41813|  %1266 = load i64, ptr %1265, , !!8                                                                                    ;L235<1063
 41814|     ;; fry = i64 %1266
 41815|  %1267 = icmp uge i64 %1188, %1260                                                                                     ;L1064
 41816|  %1268 = icmp ule i64 %1188, %1264                                                                                     ;L1064
 41817|  %1269 = and i1 %1267, %1268                                                                                           ;L1064
 41818|  %1270 = icmp uge i64 %1190, %1262                                                                                     ;L1064
 41819|  %1271 = and i1 %1270, %1269                                                                                           ;L1064
 41820|  %1272 = icmp ule i64 %1190, %1266                                                                                     ;L1064
 41821|  %1273 = and i1 %1272, %1271                                                                                           ;L1064
 41822|  br i1 %1273, label %1312, label %1274                                                                                 ;L1064
 41823| 
 41824| 1274: ; preds = %1250
 41825|  %1275 = gep %53, i64 368                                                                                              ;L1066
 41826|  %1276 = getelementptr ptr, ptr %1275, i64 %221                                                                        ;L1066
 41827|  %1277 = load ptr, ptr %1276, , !!8                                                                                    ;L1066
 41828|     ;; self = ptr %1277
 41829|     ;; f = ptr %1131
 41830|  %1278 = icmp eq ptr %1277, null                                                                                       ;L659<1067
 41831|  br i1 %1278, label %1303, label %1279                                                                                 ;L659<1067
 41832| 
 41833| 1279: ; preds = %1274
 41834|     ;; x = ptr %1277
 41835|  %1280 = load i64, ptr %1187, , !!8                                                                                    ;L661<1067
 41836|  %1281 = load i64, ptr %1189, , !!8                                                                                    ;L661<1067
 41837|  %1282 = gep %1277, i64 1632                                                                                           ;L661<1067
 41838|  %1283 = load i64, ptr %1282, , !!8                                                                                    ;L661<1067
 41839|  %1284 = gep %1277, i64 1640                                                                                           ;L661<1067
 41840|  %1285 = load i64, ptr %1284, , !!8                                                                                    ;L661<1067
 41845|     ;; x1 = i64 %1280
 41846|     ;; self = i64 %1280
 41847|     ;; y1 = i64 %1281
 41848|     ;; self = i64 %1281
 41849|     ;; x2 = i64 %1283
 41850|     ;; other = i64 %1283
 41851|     ;; y2 = i64 %1285
 41852|     ;; other = i64 %1285
 41853|  %1286 = icmp ult i64 %1280, %1283                                                                                     ;L3147<7<2158<1067<661<1067
 41854|  %1287 = sub nuw i64 %1283, %1280                                                                                      ;L3147<7<2158<1067<661<1067
 41855|  %1288 = sub nuw i64 %1280, %1283                                                                                      ;L3147<7<2158<1067<661<1067
 41856|  %1289 = select i1 %1286, i64 %1287, i64 %1288                                                                         ;L3147<7<2158<1067<661<1067
 41857|     ;; dx = i64 %1289
 41858|  %1290 = icmp ult i64 %1281, %1285                                                                                     ;L3147<8<2158<1067<661<1067
 41859|  %1291 = sub nuw i64 %1285, %1281                                                                                      ;L3147<8<2158<1067<661<1067
 41860|  %1292 = sub nuw i64 %1281, %1285                                                                                      ;L3147<8<2158<1067<661<1067
 41861|  %1293 = select i1 %1290, i64 %1291, i64 %1292                                                                         ;L3147<8<2158<1067<661<1067
 41862|     ;; dy = i64 %1293
 41863|  %1294 = mul i64 %1289, %1289                                                                                          ;L9<2158<1067<661<1067
 41864|  %1295 = mul i64 %1293, %1293                                                                                          ;L9<2158<1067<661<1067
 41865|  %1296 = add i64 %1295, %1294                                                                                          ;L9<2158<1067<661<1067
 41866|  %1297 = icmp ugt i64 %1296, 67600000000                                                                               ;L1067<661<1067
 41867|  %1298 = select i1 %1297, i64 2, i64 1                                                                                 ;L1066
 41868|     ;; pos_cls = i64 %1298
 41869|  %1299 = gep %1, i64 10624                                                                                             ;L1072
 41870|  %1300 = getelementptr i64, ptr %1299, i64 %1298                                                                       ;L1072
 41871|  %1301 = load i64, ptr %1300, , !!8                                                                                    ;L1072
 41872|  %1302 = add i64 %1301, 1                                                                                              ;L1072
 41873|  store i64 %1302, ptr %1300,                                                                                           ;L1072
 41874|  br i1 %1297, label %1307, label %1316                                                                                 ;L1073
 41875| 
 41876| 1303: ; preds = %1274
 41877|     ;; pos_cls = i64 %1298
 41878|  %1304 = gep %1, i64 10640                                                                                             ;L1072
 41879|  %1305 = load i64, ptr %1304, , !!8                                                                                    ;L1072
 41880|  %1306 = add i64 %1305, 1                                                                                              ;L1072
 41881|  store i64 %1306, ptr %1304,                                                                                           ;L1072
 41882|  br label %1307                                                                                                        ;L1073
 41883| 
 41884| 1307: ; preds = %1303, %1279
 41885|  %1308 = gep %1, i64 1184                                                                                              ;L1073
 41886|  %1309 = getelementptr i64, ptr %1308, i64 %1234                                                                       ;L1073
 41887|  %1310 = load i64, ptr %1309, , !!8                                                                                    ;L1073
 41888|  %1311 = add i64 %1310, 1                                                                                              ;L1073
 41889|  store i64 %1311, ptr %1309,                                                                                           ;L1073
 41890|  br label %1316                                                                                                        ;L1073
 41891| 
 41892| 1312: ; preds = %1250
 41893|     ;; pos_cls = i64 %1298
 41894|  %1313 = gep %1, i64 10624                                                                                             ;L1072
 41895|  %1314 = load i64, ptr %1313, , !!8                                                                                    ;L1072
 41896|  %1315 = add i64 %1314, 1                                                                                              ;L1072
 41897|  store i64 %1315, ptr %1313,                                                                                           ;L1072
 41898|  br label %1316                                                                                                        ;L1073
 41899| 
 41900| 1316: ; preds = %1312, %1307, %1279
 41901|  %1317 = load i8, ptr %425, , !!8                                                                                      ;L1074
 41902|  %1318 = trunc nuw i8 %1317 to i1                                                                                      ;L1074
 41903|  br i1 %1318, label %1320, label %1319                                                                                 ;L1074
 41904| 
 41905| 1319: ; preds = %1320, %1316
 41906|  br i1 %436, label %1324, label %1133                                                                                  ;L1075
 41907| 
 41908| 1320: ; preds = %1316
 41909|  %1321 = gep %1, i64 10600                                                                                             ;L1074
 41910|  %1322 = load i64, ptr %1321, , !!8                                                                                    ;L1074
 41911|  %1323 = add i64 %1322, 1                                                                                              ;L1074
 41912|  store i64 %1323, ptr %1321,                                                                                           ;L1074
 41913|  br label %1319                                                                                                        ;L1074
 41914| 
 41915| 1324: ; preds = %1319
 41916|  %1325 = gep %1, i64 10608                                                                                             ;L1075
 41917|  %1326 = load i64, ptr %1325, , !!8                                                                                    ;L1075
 41918|  %1327 = add i64 %1326, 1                                                                                              ;L1075
 41919|  store i64 %1327, ptr %1325,                                                                                           ;L1075
 41920|  br label %1133                                                                                                        ;L1075
 41921| 
 41922| 1328: ; preds = %1185, %1178, %1166, %1159, %1140
 41923|  %1329 = gep %0, i64 32                                                                                                ;L1082
 41924|  call void @llvm.memcpy.p0.p0.i64(ptr %1329, ptr %51, i64 32, i1 false)                                                ;L1082
 41925|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %47, i64 32, i1 false)                                                   ;L1082
 41928|  br label %1330                                                                                                        ;L1083
 41929| 
 41930| 1330: ; preds = %1328, %63
 41931|  ret void                                                                                                              ;L1083
 41932| 
 41933| 1331: ; preds = %97
 41934|  cleanupret from %99 unwind label %70
 41935| 
 41936| 1332: ; preds = %97
 41937|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %50) #34 [ "funclet"(token %99) ] ;L1083
 41938|  cleanupret from %99 unwind label %70                                                                                  ;L1083
 41939| }
