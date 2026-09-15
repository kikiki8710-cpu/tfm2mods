 34283| define void @_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster12update_state(ptr sret([32 x i8]) %0, ptr %1, ptr %2, ptr %3, ptr %4) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 34284|  %6 = alloca [224 x i8],
 34285|  %7 = alloca [224 x i8],
 34286|  %8 = alloca [24 x i8],
 34287|  %9 = alloca [24 x i8],
 34288|  %10 = alloca [24 x i8],
 34289|  %11 = alloca [24 x i8],
 34290|  %12 = alloca [24 x i8],
 34291|  %13 = alloca [32 x i8],
 34292|  %14 = alloca [24 x i8],
 34293|  %15 = alloca [24 x i8],
 34294|     ;; self = ptr %1
 34295|     ;; rnd = ptr %2
 34296|     ;; player = ptr %3
 34297|     ;; data = ptr %4
 34298|     ;; _t_ev = ptr %15
 34299|     ;; _t_plan = ptr %14
 34300|     ;; res = ptr %13
 34301|     ;; _x = ptr %12
 34302|     ;; _t_sa = ptr %11
 34303|     ;; _t_sus = ptr %10
 34304|     ;; _t_usa = ptr %9
 34305|     ;; _x = ptr %8
 34306|     ;; phase = i64 23
 34307|     ;; order = i8 0
 34308|     ;; phase = i64 24
 34309|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 34310|     ;; order = i8 0
 34311|     ;; phase = i64 25
 34312|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 34313|     ;; order = i8 0
 34314|     ;; phase = i64 28
 34315|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 34316|     ;; order = i8 0
 34317|     ;; phase = i64 29
 34318|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 34319|     ;; order = i8 0
 34321|     ;; self = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 34322|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 34323|     ;; order = i8 0
 34324|  %16 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                     ;L3904<741<176<558
 34325|  %17 = icmp eq i8 %16, 0                                                                                               ;L176<558
 34326|  br i1 %17, label %23, label %18                                                                                       ;L176<558
 34327| 
 34328| 18: ; preds = %5
 34329|  %19 = tail call { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()                                       ;L179<558
 34330|  %20 = extractvalue { i64, i32 } %19, 0                                                                                ;L179<558
 34331|  %21 = extractvalue { i64, i32 } %19, 1                                                                                ;L179<558
 34332|  store i64 23, ptr %15,                                                                                                ;L179<558
 34333|  %22 = gep %15, i64 8                                                                                                  ;L179<558
 34334|  store i64 %20, ptr %22,                                                                                               ;L179<558
 34335|  br label %23                                                                                                          ;L180<558
 34336| 
 34337| 23: ; preds = %18, %5
 34338|  %24 = phi i32 [ %21, %18 ], [ -1, %5 ]                                                                                ;L0<558
 34339|  %25 = gep %15, i64 16                                                                                                 ;L0<558
 34340|  store i32 %24, ptr %25,                                                                                               ;L0<558
 34341|  %26 = load ptr, ptr %4, , !!8, !!8                                                                                    ;L559
 34342|  %27 = gep %4, i64 8                                                                                                   ;L559
 34343|  %28 = load ptr, ptr %27,                                                                                              ;L559
 34344|  invoke fastcc void @_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster12update_event(ptr %1, ptr %2, ptr %3, ptr %26, ptr %28)
 34345|  to label %31 unwind label %29                                                                                         ;L559
 34346| 
 34347| 29: ; preds = %23
 34348|  %30 = cleanuppad within none []
 34349|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %15) #34 [ "funclet"(token %30) ] ;L560
 34350|  cleanupret from %30 unwind to caller                                                                                  ;L560
 34351| 
 34352| 31: ; preds = %23
 34355|  %32 = load i32, ptr %25, , !!8                                                                                        ;L825<560
 34356|  %33 = icmp eq i32 %32, -1                                                                                             ;L825<560
 34357|  br i1 %33, label %51, label %34                                                                                       ;L825<560
 34358| 
 34359| 34: ; preds = %31
 34363|     ;; self = ptr %15
 34364|     ;; order = i8 0
 34365|     ;; order = i8 0
 34366|     ;; val = i64 1
 34367|     ;; order = i8 0
 34368|     ;; val = i64 1
 34369|     ;; order = i8 0
 34370|  %35 = load i64, ptr %15, , !!8                                                                                        ;L185<825<825<560
 34371|  %36 = icmp ult i64 %35, 132                                                                                           ;L185<825<825<560
 34372|  br i1 %36, label %38, label %37                                                                                       ;L185<825<825<560
 34373| 
 34374| 37: ; preds = %34
 34375|  tail call void @core::panicking18panic_bounds_check(i64 %35, i64 132, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.230) #35, !!48120 ;L185<825<825<560
 34376|  unreachable                                                                                                           ;L185<825<825<560
 34377| 
 34378| 38: ; preds = %34
 34379|  %39 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %35                                   ;L185<825<825<560
 34380|     ;; self = ptr %39
 34381|  %40 = gep %15, i64 8                                                                                                  ;L185<825<825<560
 34382|  %41 = call { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %40)                                 ;L185<825<825<560
 34383|  %42 = extractvalue { i64, i32 } %41, 0                                                                                ;L185<825<825<560
 34384|  %43 = extractvalue { i64, i32 } %41, 1                                                                                ;L185<825<825<560
 34386|  %44 = mul i64 %42, 1000000000                                                                                         ;L632<185<825<825<560
 34387|  %45 = icmp ult i32 %43, 1000000000                                                                                    ;L49<632<185<825<825<560
 34388|  call void @llvm.assume(i1 %45)                                                                                        ;L49<632<185<825<825<560
 34389|  %46 = zext nneg i32 %43 to i64                                                                                        ;L632<185<825<825<560
 34390|  %47 = add i64 %44, %46                                                                                                ;L632<185<825<825<560
 34391|     ;; val = i64 %47
 34392|     ;; val = i64 %47
 34393|     ;; dst = ptr %39
 34394|  %48 = atomicrmw add ptr %39, i64 %47 monotonic, , !!48120                                                             ;L3937<3162<185<825<825<560
 34395|  %49 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %35                                   ;L186<825<825<560
 34396|     ;; self = ptr %49
 34397|     ;; dst = ptr %49
 34398|  %50 = atomicrmw add ptr %49, i64 1 monotonic, , !!48120                                                               ;L3937<3162<186<825<825<560
 34399|  br label %51                                                                                                          ;L825<560
 34400| 
 34401| 51: ; preds = %38, %31
 34404|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 34405|     ;; order = i8 0
 34406|  %52 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                     ;L3904<741<176<562
 34407|  %53 = icmp eq i8 %52, 0                                                                                               ;L176<562
 34408|  br i1 %53, label %59, label %54                                                                                       ;L176<562
 34409| 
 34410| 54: ; preds = %51
 34411|  %55 = call { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()                                            ;L179<562
 34412|  %56 = extractvalue { i64, i32 } %55, 0                                                                                ;L179<562
 34413|  %57 = extractvalue { i64, i32 } %55, 1                                                                                ;L179<562
 34414|  store i64 24, ptr %14,                                                                                                ;L179<562
 34415|  %58 = gep %14, i64 8                                                                                                  ;L179<562
 34416|  store i64 %56, ptr %58,                                                                                               ;L179<562
 34417|  br label %59                                                                                                          ;L180<562
 34418| 
 34419| 59: ; preds = %54, %51
 34420|  %60 = phi i32 [ %57, %54 ], [ -1, %51 ]                                                                               ;L0<562
 34421|  %61 = gep %14, i64 16                                                                                                 ;L0<562
 34422|  store i32 %60, ptr %61,                                                                                               ;L0<562
 34423|  %62 = invoke zeroext i1 @ai::utils22player_awareness_lapse(ptr %3, ptr %4)
 34424|  to label %66 unwind label %63                                                                                         ;L566
 34425| 
 34426| 63: ; preds = %113, %79, %73, %59
 34427|  %64 = phi i1 [ false, %113 ], [ true, %73 ], [ true, %79 ], [ true, %59 ]                                             ;L0
 34428|  %65 = cleanuppad within none []
 34429|  br i1 %64, label %341, label %340                                                                                     ;L610
 34430| 
 34431| 66: ; preds = %59
 34432|  %67 = gep %1, i64 10698                                                                                               ;L566
 34433|  %68 = zext i1 %62 to i8                                                                                               ;L566
 34434|  store i8 %68, ptr %67,                                                                                                ;L566
 34436|  %69 = gep %1, i64 10512                                                                                               ;L569
 34437|  %70 = load i64, ptr %69, , !!8                                                                                        ;L569
 34438|  %71 = icmp ugt i64 %70, 1                                                                                             ;L569
 34439|  br i1 %71, label %73, label %72                                                                                       ;L569
 34440| 
 34441| 72: ; preds = %66
 34442|  br i1 %62, label %85, label %79                                                                                       ;L571
 34443| 
 34444| 73: ; preds = %66
 34446|     ;; self = ptr %1
 34447|     ;; rnd = ptr %2
 34448|     ;; player = ptr %3
 34449|     ;; data = ptr %4
 34450|     ;; lapse = i1 %62
 34451|  %74 = gep %1, i64 1328                                                                                                ;L689<570
 34452|  invoke void @ai::plan_legacy7handlerNtB2_17LegacyPlanHandler6update(ptr %74, i64 %70, ptr %2, ptr %3, ptr %4, ptr %1, i1 zeroext %62)
 34453|  to label %75 unwind label %63                                                                                         ;L689<570
 34454| 
 34455| 75: ; preds = %73
 34456|  %76 = load ptr, ptr %28, , !!48138, !!8, !!8                                                                          ;L690<570
 34457|     ;; bump = ptr %76
 34458|  store ptr inttoptr (i64 8 to ptr), ptr %13, , !!48165                                                                 ;L547<690<570
 34459|  %77 = gep %13, i64 8                                                                                                  ;L547<690<570
 34460|  store ptr %76, ptr %77, , !!48165                                                                                     ;L547<690<570
 34461|  %78 = gep %13, i64 16                                                                                                 ;L547<690<570
 34462|  br label %89                                                                                                          ;L691<570
 34463| 
 34464| 79: ; preds = %72
 34466|     ;; self = ptr %1
 34467|     ;; rnd = ptr %2
 34468|     ;; player = ptr %3
 34469|     ;; data = ptr %4
 34470|  %80 = gep %1, i64 1328                                                                                                ;L683<574
 34471|  invoke void @ai::plan_legacy7handlerNtB2_17LegacyPlanHandler6update(ptr %80, i64 %70, ptr %2, ptr %3, ptr %4, ptr %1, i1 zeroext false)
 34472|  to label %81 unwind label %63                                                                                         ;L683<574
 34473| 
 34474| 81: ; preds = %79
 34475|  %82 = load ptr, ptr %28, , !!48171, !!8, !!8                                                                          ;L684<574
 34476|     ;; bump = ptr %82
 34477|  store ptr inttoptr (i64 8 to ptr), ptr %13, , !!48192                                                                 ;L547<684<574
 34478|  %83 = gep %13, i64 8                                                                                                  ;L547<684<574
 34479|  store ptr %82, ptr %83, , !!48192                                                                                     ;L547<684<574
 34480|  %84 = gep %13, i64 16                                                                                                 ;L547<684<574
 34481|  br label %89                                                                                                          ;L685<574
 34482| 
 34483| 85: ; preds = %72
 34484|  %86 = load ptr, ptr %28, , !!8, !!8                                                                                   ;L572
 34485|     ;; bump = ptr %86
 34486|  store ptr inttoptr (i64 8 to ptr), ptr %13,                                                                           ;L547<572
 34487|  %87 = gep %13, i64 8                                                                                                  ;L547<572
 34488|  store ptr %86, ptr %87,                                                                                               ;L547<572
 34489|  %88 = gep %13, i64 16                                                                                                 ;L547<572
 34490|  br label %89                                                                                                          ;L571
 34491| 
 34492| 89: ; preds = %85, %81, %75
 34493|  %90 = phi ptr [ %84, %81 ], [ %78, %75 ], [ %88, %85 ]
 34494|  call void @llvm.memset.p0.i64(ptr %90, i8 0, i64 16, i1 false)                                                        ;L0
 34496|  call void @llvm.memcpy.p0.p0.i64(ptr %12, ptr %14, i64 24, i1 false)                                                  ;L576
 34499|  %91 = gep %12, i64 16                                                                                                 ;L825<1004<576
 34500|  %92 = load i32, ptr %91, , !!8                                                                                        ;L825<1004<576
 34501|  %93 = icmp eq i32 %92, -1                                                                                             ;L825<1004<576
 34502|  br i1 %93, label %115, label %94                                                                                      ;L825<1004<576
 34503| 
 34504| 94: ; preds = %89
 34508|     ;; self = ptr %12
 34509|     ;; order = i8 0
 34510|     ;; order = i8 0
 34511|     ;; val = i64 1
 34512|     ;; order = i8 0
 34513|     ;; val = i64 1
 34514|     ;; order = i8 0
 34515|  %95 = load i64, ptr %12, , !!8                                                                                        ;L185<825<825<1004<576
 34516|  %96 = icmp ult i64 %95, 132                                                                                           ;L185<825<825<1004<576
 34517|  br i1 %96, label %99, label %97                                                                                       ;L185<825<825<1004<576
 34518| 
 34519| 97: ; preds = %94
 34520|  invoke void @core::panicking18panic_bounds_check(i64 %95, i64 132, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.230) #35
 34521|  to label %98 unwind label %113                                                                                        ;L185<825<825<1004<576
 34522| 
 34523| 98: ; preds = %97
 34524|  unreachable                                                                                                           ;L185<825<825<1004<576
 34525| 
 34526| 99: ; preds = %94
 34527|     ;; self = !DIArgList(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 %95)
 34528|  %100 = gep %12, i64 8                                                                                                 ;L185<825<825<1004<576
 34529|  %101 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %100)
 34530|  to label %102 unwind label %113                                                                                       ;L185<825<825<1004<576
 34531| 
 34532| 102: ; preds = %99
 34533|  %103 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %95                                  ;L185<825<825<1004<576
 34534|     ;; self = ptr %103
 34535|  %104 = extractvalue { i64, i32 } %101, 0                                                                              ;L185<825<825<1004<576
 34536|  %105 = extractvalue { i64, i32 } %101, 1                                                                              ;L185<825<825<1004<576
 34538|  %106 = mul i64 %104, 1000000000                                                                                       ;L632<185<825<825<1004<576
 34539|  %107 = icmp ult i32 %105, 1000000000                                                                                  ;L49<632<185<825<825<1004<576
 34540|  call void @llvm.assume(i1 %107)                                                                                       ;L49<632<185<825<825<1004<576
 34541|  %108 = zext nneg i32 %105 to i64                                                                                      ;L632<185<825<825<1004<576
 34542|  %109 = add i64 %106, %108                                                                                             ;L632<185<825<825<1004<576
 34543|     ;; val = i64 %109
 34544|     ;; val = i64 %109
 34545|     ;; dst = ptr %103
 34546|  %110 = atomicrmw add ptr %103, i64 %109 monotonic, , !!48232                                                          ;L3937<3162<185<825<825<1004<576
 34547|  %111 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %95                                  ;L186<825<825<1004<576
 34548|     ;; self = ptr %111
 34549|     ;; dst = ptr %111
 34550|  %112 = atomicrmw add ptr %111, i64 1 monotonic, , !!48232                                                             ;L3937<3162<186<825<825<1004<576
 34551|  br label %115                                                                                                         ;L825<1004<576
 34552| 
 34553| 113: ; preds = %339, %338, %118, %99, %97
 34554|  %114 = cleanuppad within none []
 34555|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtNtCs97f5S1uJLkH_9game_core10simulation12ai_interface9TurnEventEECshdEBA0ozCnw_7game_ai(ptr %13) #34 [ "funclet"(token %114) ] ;L610
 34556|  cleanupret from %114 unwind label %63                                                                                 ;L610
 34557| 
 34558| 115: ; preds = %102, %89
 34561|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 34562|     ;; order = i8 0
 34563|  %116 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                    ;L3904<741<176<577
 34564|  %117 = icmp eq i8 %116, 0                                                                                             ;L176<577
 34565|  br i1 %117, label %127, label %118                                                                                    ;L176<577
 34566| 
 34567| 118: ; preds = %115
 34568|  %119 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()
 34569|  to label %120 unwind label %113                                                                                       ;L179<577
 34570| 
 34571| 120: ; preds = %118
 34572|  %121 = extractvalue { i64, i32 } %119, 0                                                                              ;L179<577
 34573|  %122 = extractvalue { i64, i32 } %119, 1                                                                              ;L179<577
 34574|  store i64 25, ptr %11,                                                                                                ;L179<577
 34575|  %123 = gep %11, i64 8                                                                                                 ;L179<577
 34576|  store i64 %121, ptr %123,                                                                                             ;L179<577
 34577|  br label %127                                                                                                         ;L180<577
 34578| 
 34579| 124: ; preds = %337, %333, %332, %330, %327, %326, %324, %321, %303, %300, %290, %259, %249, %247, %230, %204, %195, %181, %179, %168, %163, %148, %145, %134
 34580|  %125 = phi i1 [ false, %337 ], [ false, %333 ], [ false, %332 ], [ false, %330 ], [ false, %327 ], [ false, %326 ], [ false, %324 ], [ false, %321 ], [ false, %181 ], [ true, %303 ], [ true, %300 ], [ true, %290 ], [ true, %247 ], [ true, %230 ], [ true, %259 ], [ true, %249 ], [ true, %168 ], [ true, %163 ], [ true, %148 ], [ true, %145 ], [ true, %134 ], [ true, %204 ], [ false, %179 ], [ true, %195 ] ;L0
 34581|  %126 = cleanuppad within none []
 34582|  br i1 %125, label %339, label %338                                                                                    ;L610
 34583| 
 34584| 127: ; preds = %120, %115
 34585|  %128 = phi i32 [ %122, %120 ], [ -1, %115 ]
 34586|  %129 = gep %11, i64 16                                                                                                ;L0<577
 34587|  store i32 %128, ptr %129,                                                                                             ;L0<577
 34589|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 34590|     ;; order = i8 0
 34591|  %130 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                    ;L3904<741<176<579
 34592|  %131 = icmp eq i8 %130, 0                                                                                             ;L176<579
 34593|  br i1 %131, label %132, label %134                                                                                    ;L176<579
 34594| 
 34595| 132: ; preds = %127
 34596|  %133 = gep %10, i64 16                                                                                                ;L177<579
 34597|  store i32 -1, ptr %133,                                                                                               ;L177<579
 34598|  br label %136                                                                                                         ;L180<579
 34599| 
 34600| 134: ; preds = %127
 34601|  %135 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()
 34602|  to label %139 unwind label %124                                                                                       ;L179<579
 34603| 
 34604| 136: ; preds = %139, %132
 34605|  %137 = phi i1 [ %144, %139 ], [ true, %132 ]
 34606|  %138 = gep %1, i64 10328                                                                                              ;L580
 34607|  invoke void @ai::small_actionNtB2_15SmallActionPlay12update_state(ptr %138, ptr %2, ptr %3, ptr %4, ptr %1)
 34608|  to label %147 unwind label %145                                                                                       ;L580
 34609| 
 34610| 139: ; preds = %134
 34611|  %140 = extractvalue { i64, i32 } %135, 0                                                                              ;L179<579
 34612|  %141 = extractvalue { i64, i32 } %135, 1                                                                              ;L179<579
 34613|  store i64 28, ptr %10,                                                                                                ;L179<579
 34614|  %142 = gep %10, i64 8                                                                                                 ;L179<579
 34615|  store i64 %140, ptr %142,                                                                                             ;L179<579
 34616|  %143 = gep %10, i64 16                                                                                                ;L179<579
 34617|  store i32 %141, ptr %143,                                                                                             ;L179<579
 34618|  %144 = icmp eq i32 %141, -1                                                                                           ;L825<581
 34619|  br label %136                                                                                                         ;L180<579
 34620| 
 34621| 145: ; preds = %136
 34622|  %146 = cleanuppad within none []
 34623|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %10) #34 [ "funclet"(token %146) ] ;L581
 34624|  cleanupret from %146 unwind label %124                                                                                ;L581
 34625| 
 34626| 147: ; preds = %136
 34628|  br i1 %137, label %160, label %148                                                                                    ;L825<581
 34629| 
 34630| 148: ; preds = %147
 34632|     ;; self = ptr %10
 34633|     ;; order = i8 0
 34634|     ;; order = i8 0
 34635|     ;; val = i64 1
 34636|     ;; order = i8 0
 34637|     ;; val = i64 1
 34638|     ;; order = i8 0
 34639|     ;; self = !DIArgList(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 28)
 34640|  %149 = gep %10, i64 8                                                                                                 ;L185<825<825<581
 34641|  %150 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %149)
 34642|  to label %151 unwind label %124                                                                                       ;L185<825<825<581
 34643| 
 34644| 151: ; preds = %148
 34645|     ;; self = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 224)
 34646|  %152 = extractvalue { i64, i32 } %150, 0                                                                              ;L185<825<825<581
 34647|  %153 = extractvalue { i64, i32 } %150, 1                                                                              ;L185<825<825<581
 34649|  %154 = mul i64 %152, 1000000000                                                                                       ;L632<185<825<825<581
 34650|  %155 = icmp ult i32 %153, 1000000000                                                                                  ;L49<632<185<825<825<581
 34651|  call void @llvm.assume(i1 %155)                                                                                       ;L49<632<185<825<825<581
 34652|  %156 = zext nneg i32 %153 to i64                                                                                      ;L632<185<825<825<581
 34653|  %157 = add i64 %154, %156                                                                                             ;L632<185<825<825<581
 34654|     ;; val = i64 %157
 34655|     ;; val = i64 %157
 34656|     ;; dst = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 224)
 34657|  %158 = atomicrmw add ptr getelementptr (i8, ptr @gc::simulation4prof11PHASE_NANOS, i64 224), i64 %157 monotonic, , !!48279 ;L3937<3162<185<825<825<581
 34658|     ;; self = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 224)
 34659|     ;; dst = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 224)
 34660|  %159 = atomicrmw add ptr getelementptr (i8, ptr @gc::simulation4prof11PHASE_CALLS, i64 224), i64 1 monotonic, , !!48279 ;L3937<3162<186<825<825<581
 34661|  br label %160                                                                                                         ;L825<581
 34662| 
 34663| 160: ; preds = %151, %147
 34665|  %161 = load i64, ptr %69, , !!8                                                                                       ;L589
 34666|  %162 = icmp ugt i64 %161, 1                                                                                           ;L589
 34667|  br i1 %162, label %163, label %168                                                                                    ;L589
 34668| 
 34669| 163: ; preds = %160
 34670|  %164 = invoke zeroext i1 @ai::small_actionNtB2_15SmallActionPlay15is_premise_lost(ptr %138, ptr %4)
 34671|  to label %165 unwind label %124                                                                                       ;L589
 34672| 
 34673| 165: ; preds = %163
 34675|  br i1 %164, label %285, label %166                                                                                    ;L590
 34676| 
 34677| 166: ; preds = %165
 34678|  %167 = load i64, ptr %69,                                                                                             ;L590
 34679|  br label %168                                                                                                         ;L590
 34680| 
 34681| 168: ; preds = %166, %160
 34682|  %169 = phi i64 [ %167, %166 ], [ %161, %160 ]                                                                         ;L590
 34683|  %170 = invoke zeroext i1 @ai::small_actionNtB2_15SmallActionPlay6is_end(ptr %138, ptr %2, i64 %169, ptr %3, ptr %4)
 34684|  to label %171 unwind label %124                                                                                       ;L590
 34685| 
 34686| 171: ; preds = %168
 34687|  br i1 %170, label %195, label %172                                                                                    ;L590
 34688| 
 34689| 172: ; preds = %315, %283, %282, %281, %280, %279, %278, %277, %276, %275, %274, %272, %271, %265, %171
 34691|  call void @llvm.memcpy.p0.p0.i64(ptr %8, ptr %11, i64 24, i1 false)                                                   ;L598
 34694|  %173 = gep %8, i64 16                                                                                                 ;L825<1004<598
 34695|  %174 = load i32, ptr %173, , !!8                                                                                      ;L825<1004<598
 34696|  %175 = icmp eq i32 %174, -1                                                                                           ;L825<1004<598
 34697|  br i1 %175, label %316, label %176                                                                                    ;L825<1004<598
 34698| 
 34699| 176: ; preds = %172
 34703|     ;; self = ptr %8
 34704|     ;; order = i8 0
 34705|     ;; order = i8 0
 34706|     ;; val = i64 1
 34707|     ;; order = i8 0
 34708|     ;; val = i64 1
 34709|     ;; order = i8 0
 34710|  %177 = load i64, ptr %8, , !!8                                                                                        ;L185<825<825<1004<598
 34711|  %178 = icmp ult i64 %177, 132                                                                                         ;L185<825<825<1004<598
 34712|  br i1 %178, label %181, label %179                                                                                    ;L185<825<825<1004<598
 34713| 
 34714| 179: ; preds = %176
 34715|  invoke void @core::panicking18panic_bounds_check(i64 %177, i64 132, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.230) #35
 34716|  to label %180 unwind label %124                                                                                       ;L185<825<825<1004<598
 34717| 
 34718| 180: ; preds = %179
 34719|  unreachable                                                                                                           ;L185<825<825<1004<598
 34720| 
 34721| 181: ; preds = %176
 34722|     ;; self = !DIArgList(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 %177)
 34723|  %182 = gep %8, i64 8                                                                                                  ;L185<825<825<1004<598
 34724|  %183 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %182)
 34725|  to label %184 unwind label %124                                                                                       ;L185<825<825<1004<598
 34726| 
 34727| 184: ; preds = %181
 34728|  %185 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_NANOS, i64 %177                                 ;L185<825<825<1004<598
 34729|     ;; self = ptr %185
 34730|  %186 = extractvalue { i64, i32 } %183, 0                                                                              ;L185<825<825<1004<598
 34731|  %187 = extractvalue { i64, i32 } %183, 1                                                                              ;L185<825<825<1004<598
 34733|  %188 = mul i64 %186, 1000000000                                                                                       ;L632<185<825<825<1004<598
 34734|  %189 = icmp ult i32 %187, 1000000000                                                                                  ;L49<632<185<825<825<1004<598
 34735|  call void @llvm.assume(i1 %189)                                                                                       ;L49<632<185<825<825<1004<598
 34736|  %190 = zext nneg i32 %187 to i64                                                                                      ;L632<185<825<825<1004<598
 34737|  %191 = add i64 %188, %190                                                                                             ;L632<185<825<825<1004<598
 34738|     ;; val = i64 %191
 34739|     ;; val = i64 %191
 34740|     ;; dst = ptr %185
 34741|  %192 = atomicrmw add ptr %185, i64 %191 monotonic, , !!48317                                                          ;L3937<3162<185<825<825<1004<598
 34742|  %193 = getelementptr { { { i64 } } }, ptr @gc::simulation4prof11PHASE_CALLS, i64 %177                                 ;L186<825<825<1004<598
 34743|     ;; self = ptr %193
 34744|     ;; dst = ptr %193
 34745|  %194 = atomicrmw add ptr %193, i64 1 monotonic, , !!48317                                                             ;L3937<3162<186<825<825<1004<598
 34746|  br label %316                                                                                                         ;L825<1004<598
 34747| 
 34748| 195: ; preds = %171
 34749|  %196 = gep %4, i64 16                                                                                                 ;L591
 34750|  %197 = load ptr, ptr %196,                                                                                            ;L591
 34752|     ;; self = ptr %1
 34753|     ;; player = ptr %3
 34755|  %198 = load ptr, ptr %26, , !!48326, !!8, !!8                                                                         ;L797<591
 34756|  %199 = gep %26, i64 8                                                                                                 ;L797<591
 34757|  %200 = load ptr, ptr %199, , !!48326, !!8, !!8                                                                        ;L797<591
 34758|  %201 = gep %200, i64 40                                                                                               ;L797<591
 34759|  %202 = load ptr, ptr %201, , !!48326, !!8                                                                             ;L797<591
 34760|  %203 = invoke i64 %202(ptr %198)
 34761|  to label %204 unwind label %124                                                                                       ;L797<591
 34762| 
 34763| 204: ; preds = %195
 34764|     ;; tick = i64 %203
 34765|  %205 = gep %200, i64 64                                                                                               ;L801<591
 34766|  %206 = load ptr, ptr %205, , !!48326, !!8                                                                             ;L801<591
 34767|  %207 = invoke { i64, ptr } %206(ptr %198)
 34768|  to label %208 unwind label %124                                                                                       ;L801<591
 34769| 
 34770| 208: ; preds = %204
 34771|  %209 = extractvalue { i64, ptr } %207, 0                                                                              ;L801<591
 34772|  %210 = icmp eq i64 %209, 2                                                                                            ;L801<591
 34773|  br i1 %210, label %285, label %211                                                                                    ;L801<591
 34774| 
 34775| 211: ; preds = %208
 34776|  %212 = gep %1, i64 10568                                                                                              ;L806<591
 34777|  %213 = load i64, ptr %212, , !!48326, !!8                                                                             ;L806<591
 34778|  %214 = add i64 %213, 10                                                                                               ;L806<591
 34779|  %215 = icmp ult i64 %203, %214                                                                                        ;L806<591
 34780|  br i1 %215, label %216, label %285                                                                                    ;L806<591
 34781| 
 34782| 216: ; preds = %211
 34783|     ;; self = ptr %1
 34784|  %217 = gep %1, i64 10505                                                                                              ;L463<811<591
 34785|  %218 = load i8, ptr %217, , !!48326, !!8                                                                              ;L463<811<591
 34786|  %219 = icmp ne i8 %218, 10                                                                                            ;L463<811<591
 34787|  call void @llvm.assume(i1 %219)                                                                                       ;L463<811<591
 34788|  %220 = add nsw i8 %218, -15                                                                                           ;L463<811<591
 34789|  %221 = icmp ult i8 %220, 4                                                                                            ;L463<811<591
 34790|  %222 = gep %1, i64 10344
 34791|  %223 = load i8, ptr %222,
 34792|  %224 = trunc nuw i8 %223 to i1
 34793|  %225 = select i1 %221, i1 %224, i1 false                                                                              ;L463<811<591
 34794|  br i1 %225, label %285, label %226                                                                                    ;L463<811<591
 34795| 
 34796| 226: ; preds = %216
 34797|  %227 = gep %3, i64 2352                                                                                               ;L816<591
 34798|  %228 = load i64, ptr %227, , !!8                                                                                      ;L816<591
 34799|  %229 = icmp ult i64 %228, 2                                                                                           ;L816<591
 34800|  br i1 %229, label %232, label %230                                                                                    ;L816<591
 34801| 
 34802| 230: ; preds = %226
 34803|  invoke void @core::panicking18panic_bounds_check(i64 %228, i64 2, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.146) #35
 34804|  to label %231 unwind label %124                                                                                       ;L816<591
 34805| 
 34806| 231: ; preds = %230
 34807|  unreachable                                                                                                           ;L816<591
 34808| 
 34809| 232: ; preds = %226
 34810|     ;; self = ptr %3
 34811|  %233 = gep %3, i64 2496                                                                                               ;L581<816<591
 34812|  %234 = load i32, ptr %233, , !!8                                                                                      ;L581<816<591
 34813|  %235 = zext nneg i32 %234 to i64                                                                                      ;L581<816<591
 34814|  %236 = gep %26, i64 480                                                                                               ;L816<591
 34815|  %237 = getelementptr [5 x ptr], ptr %236, i64 %228                                                                    ;L816<591
 34816|  %238 = getelementptr ptr, ptr %237, i64 %235                                                                          ;L816<591
 34817|  %239 = load ptr, ptr %238, , !!48326, !!8                                                                             ;L816<591
 34818|     ;; self = ptr %239
 34819|  %240 = icmp eq ptr %239, null                                                                                         ;L1011<816<591
 34820|  br i1 %240, label %247, label %241                                                                                    ;L1011<816<591
 34821| 
 34822| 241: ; preds = %232
 34823|     ;; champ = ptr %239
 34824|  %242 = gep %239, i64 1648                                                                                             ;L817<591
 34825|  %243 = load i64, ptr %242, , !!48326, !!8                                                                             ;L817<591
 34826|  %244 = gep %1, i64 10576                                                                                              ;L817<591
 34827|  %245 = load i64, ptr %244, , !!48326, !!8                                                                             ;L817<591
 34828|  %246 = icmp ult i64 %243, %245                                                                                        ;L817<591
 34829|  br i1 %246, label %285, label %249                                                                                    ;L817<591
 34830| 
 34831| 247: ; preds = %232
 34832|  invoke void @core::option13unwrap_failed(ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.147) #35
 34833|  to label %248 unwind label %124                                                                                       ;L1013<816<591
 34834| 
 34835| 248: ; preds = %247
 34836|  unreachable                                                                                                           ;L1013<816<591
 34837| 
 34838| 249: ; preds = %241
 34839|  %250 = gep %239, i64 1632                                                                                             ;L822<591
 34840|  %251 = load i64, ptr %250, , !!48326                                                                                  ;L822<591
 34841|  %252 = gep %239, i64 1640                                                                                             ;L822<591
 34842|  %253 = load i64, ptr %252, , !!48326                                                                                  ;L822<591
 34843|  %254 = invoke fastcc i16 @_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster20count_nearby_enemies(i64 %251, i64 %253, ptr %3, ptr %26, ptr %197)
 34844|  to label %255 unwind label %124                                                                                       ;L822<591
 34845| 
 34846| 255: ; preds = %249
 34847|     ;; current_enemies = i16 %254
 34848|  %256 = gep %1, i64 10696                                                                                              ;L823<591
 34849|  %257 = load i16, ptr %256, , !!48326, !!8                                                                             ;L823<591
 34850|  %258 = icmp eq i16 %254, %257                                                                                         ;L823<591
 34851|  br i1 %258, label %259, label %285                                                                                    ;L591
 34852| 
 34853| 259: ; preds = %255
 34854|  %260 = load ptr, ptr %26, , !!8, !!8                                                                                  ;L592
 34855|  %261 = load ptr, ptr %199, , !!8, !!8                                                                                 ;L592
 34856|  %262 = gep %261, i64 40                                                                                               ;L592
 34857|  %263 = load ptr, ptr %262, , !!8                                                                                      ;L592
 34858|  %264 = invoke i64 %263(ptr %260)
 34859|  to label %265 unwind label %124                                                                                       ;L592
 34860| 
 34861| 265: ; preds = %259
 34862|     ;; self = ptr %138
 34863|     ;; tick = i64 %264
 34864|  %266 = load i8, ptr %217, , !!8                                                                                       ;L445<592
 34865|  %267 = icmp ne i8 %266, 10                                                                                            ;L445<592
 34866|  call void @llvm.assume(i1 %267)                                                                                       ;L445<592
 34867|  %268 = add nsw i8 %266, -3                                                                                            ;L445<592
 34868|  %269 = icmp samesign ugt i8 %266, 2                                                                                   ;L445<592
 34869|  %270 = select i1 %269, i8 %268, i8 7                                                                                  ;L445<592
 34870|  switch i8 %270, label %172 [
 34871|  i8 0, label %271
 34872|  i8 1, label %272
 34873|  i8 2, label %274
 34874|  i8 3, label %275
 34875|  i8 4, label %276
 34876|  i8 5, label %277
 34877|  i8 6, label %278
 34878|  i8 7, label %279
 34879|  i8 8, label %280
 34880|  i8 9, label %281
 34881|  i8 10, label %282
 34882|  i8 11, label %283
 34883|  ]                                                                                                                     ;L445<592
 34884| 
 34885| 271: ; preds = %265
 34886|     ;; a = ptr %138
 34887|  store i64 %264, ptr %138,                                                                                             ;L446<592
 34888|  br label %172                                                                                                         ;L446<592
 34889| 
 34890| 272: ; preds = %265
 34891|     ;; a = ptr %138
 34892|  %273 = gep %1, i64 10400                                                                                              ;L447<592
 34893|  store i64 %264, ptr %273,                                                                                             ;L447<592
 34894|  br label %172                                                                                                         ;L447<592
 34895| 
 34896| 274: ; preds = %265
 34897|     ;; a = ptr %138
 34898|  store i64 %264, ptr %138,                                                                                             ;L448<592
 34899|  br label %172                                                                                                         ;L448<592
 34900| 
 34901| 275: ; preds = %265
 34902|     ;; a = ptr %138
 34903|  store i64 %264, ptr %138,                                                                                             ;L449<592
 34904|  br label %172                                                                                                         ;L449<592
 34905| 
 34906| 276: ; preds = %265
 34907|     ;; a = ptr %138
 34908|  store i64 %264, ptr %138,                                                                                             ;L450<592
 34909|  br label %172                                                                                                         ;L450<592
 34910| 
 34911| 277: ; preds = %265
 34912|     ;; a = ptr %138
 34913|  store i64 %264, ptr %138,                                                                                             ;L456<592
 34914|  br label %172                                                                                                         ;L456<592
 34915| 
 34916| 278: ; preds = %265
 34917|     ;; a = ptr %138
 34918|  store i64 %264, ptr %138,                                                                                             ;L451<592
 34919|  br label %172                                                                                                         ;L451<592
 34920| 
 34921| 279: ; preds = %265
 34922|     ;; a = ptr %138
 34923|  store i64 %264, ptr %138,                                                                                             ;L452<592
 34924|  br label %172                                                                                                         ;L452<592
 34925| 
 34926| 280: ; preds = %265
 34927|     ;; a = ptr %138
 34928|  store i64 %264, ptr %138,                                                                                             ;L453<592
 34929|  br label %172                                                                                                         ;L453<592
 34930| 
 34931| 281: ; preds = %265
 34932|     ;; a = ptr %138
 34933|  store i64 %264, ptr %138,                                                                                             ;L454<592
 34934|  br label %172                                                                                                         ;L454<592
 34935| 
 34936| 282: ; preds = %265
 34937|     ;; a = ptr %138
 34938|  store i64 %264, ptr %138,                                                                                             ;L455<592
 34939|  br label %172                                                                                                         ;L455<592
 34940| 
 34941| 283: ; preds = %265
 34942|     ;; a = ptr %138
 34943|  %284 = gep %1, i64 10416                                                                                              ;L457<592
 34944|  store i64 %264, ptr %284,                                                                                             ;L457<592
 34945|  br label %172                                                                                                         ;L457<592
 34946| 
 34947| 285: ; preds = %255, %241, %216, %211, %208, %165
 34949|     ;; dst = ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED
 34950|     ;; order = i8 0
 34951|  %286 = load atomic i8, ptr @gc::simulation4prof7ENABLED monotonic,                                                    ;L3904<741<176<594
 34952|  %287 = icmp eq i8 %286, 0                                                                                             ;L176<594
 34953|  br i1 %287, label %288, label %290                                                                                    ;L176<594
 34954| 
 34955| 288: ; preds = %285
 34956|  %289 = gep %9, i64 16                                                                                                 ;L177<594
 34957|  store i32 -1, ptr %289,                                                                                               ;L177<594
 34958|  br label %292                                                                                                         ;L180<594
 34959| 
 34960| 290: ; preds = %285
 34961|  %291 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now()
 34962|  to label %294 unwind label %124                                                                                       ;L179<594
 34963| 
 34964| 292: ; preds = %294, %288
 34965|  %293 = phi i1 [ %299, %294 ], [ true, %288 ]
 34966|  invoke fastcc void @_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster19update_small_action(ptr %1, ptr %2, ptr %3, ptr %4)
 34967|  to label %302 unwind label %300                                                                                       ;L595
 34968| 
 34969| 294: ; preds = %290
 34970|  %295 = extractvalue { i64, i32 } %291, 0                                                                              ;L179<594
 34971|  %296 = extractvalue { i64, i32 } %291, 1                                                                              ;L179<594
 34972|  store i64 29, ptr %9,                                                                                                 ;L179<594
 34973|  %297 = gep %9, i64 8                                                                                                  ;L179<594
 34974|  store i64 %295, ptr %297,                                                                                             ;L179<594
 34975|  %298 = gep %9, i64 16                                                                                                 ;L179<594
 34976|  store i32 %296, ptr %298,                                                                                             ;L179<594
 34977|  %299 = icmp eq i32 %296, -1                                                                                           ;L825<596
 34978|  br label %292                                                                                                         ;L180<594
 34979| 
 34980| 300: ; preds = %292
 34981|  %301 = cleanuppad within none []
 34982|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %9) #34 [ "funclet"(token %301) ] ;L596
 34983|  cleanupret from %301 unwind label %124                                                                                ;L596
 34984| 
 34985| 302: ; preds = %292
 34987|  br i1 %293, label %315, label %303                                                                                    ;L825<596
 34988| 
 34989| 303: ; preds = %302
 34991|     ;; self = ptr %9
 34992|     ;; order = i8 0
 34993|     ;; order = i8 0
 34994|     ;; val = i64 1
 34995|     ;; order = i8 0
 34996|     ;; val = i64 1
 34997|     ;; order = i8 0
 34998|     ;; self = !DIArgList(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 29)
 34999|  %304 = gep %9, i64 8                                                                                                  ;L185<825<825<596
 35000|  %305 = invoke { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr %304)
 35001|  to label %306 unwind label %124                                                                                       ;L185<825<825<596
 35002| 
 35003| 306: ; preds = %303
 35004|     ;; self = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 232)
 35005|  %307 = extractvalue { i64, i32 } %305, 0                                                                              ;L185<825<825<596
 35006|  %308 = extractvalue { i64, i32 } %305, 1                                                                              ;L185<825<825<596
 35008|  %309 = mul i64 %307, 1000000000                                                                                       ;L632<185<825<825<596
 35009|  %310 = icmp ult i32 %308, 1000000000                                                                                  ;L49<632<185<825<825<596
 35010|  call void @llvm.assume(i1 %310)                                                                                       ;L49<632<185<825<825<596
 35011|  %311 = zext nneg i32 %308 to i64                                                                                      ;L632<185<825<825<596
 35012|  %312 = add i64 %309, %311                                                                                             ;L632<185<825<825<596
 35013|     ;; val = i64 %312
 35014|     ;; val = i64 %312
 35015|     ;; dst = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 232)
 35016|  %313 = atomicrmw add ptr getelementptr (i8, ptr @gc::simulation4prof11PHASE_NANOS, i64 232), i64 %312 monotonic, , !!48478 ;L3937<3162<185<825<825<596
 35017|     ;; self = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 232)
 35018|     ;; dst = ptr getelementptr inbounds nuw (i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 232)
 35019|  %314 = atomicrmw add ptr getelementptr (i8, ptr @gc::simulation4prof11PHASE_CALLS, i64 232), i64 1 monotonic, , !!48478 ;L3937<3162<186<825<825<596
 35020|  br label %315                                                                                                         ;L825<596
 35021| 
 35022| 315: ; preds = %306, %302
 35024|  br label %172                                                                                                         ;L591
 35025| 
 35026| 316: ; preds = %184, %172
 35028|  %317 = gep %28, i64 59                                                                                                ;L600
 35029|  %318 = load i8, ptr %317, , !!8                                                                                       ;L600
 35030|  %319 = trunc nuw i8 %318 to i1                                                                                        ;L600
 35031|  br i1 %319, label %321, label %320                                                                                    ;L600
 35032| 
 35033| 320: ; preds = %337, %335, %316
 35034|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %13, i64 32, i1 false)                                                   ;L609
 35038|  ret void                                                                                                              ;L610
 35039| 
 35040| 321: ; preds = %316
 35042|  %322 = gep %1, i64 224                                                                                                ;L601
 35043|  invoke fastcc void @gc::simulation4game5frameNtB5_14DebugFrameDataNtNtCsjihNppCmMEE_4core5clone5Clone5clone(ptr %7, ptr %322)
 35044|  to label %323 unwind label %124                                                                                       ;L601
 35045| 
 35046| 323: ; preds = %321
 35047|  invoke void @gc::simulation4game5frameNtB4_14DebugFrameData5merge(ptr %1, ptr %7)
 35048|  to label %326 unwind label %324                                                                                       ;L601
 35049| 
 35050| 324: ; preds = %323
 35051|  %325 = cleanuppad within none []
 35052|  call void @core::ptr9drop_glueNtNtNtNtCs97f5S1uJLkH_9game_core10simulation4game5frame14DebugFrameDataECshdEBA0ozCnw_7game_ai(ptr %7) #34 [ "funclet"(token %325) ] ;L601
 35053|  cleanupret from %325 unwind label %124                                                                                ;L601
 35054| 
 35055| 326: ; preds = %323
 35056|  invoke void @core::ptr9drop_glueNtNtNtNtCs97f5S1uJLkH_9game_core10simulation4game5frame14DebugFrameDataECshdEBA0ozCnw_7game_ai(ptr %7)
 35057|  to label %327 unwind label %124                                                                                       ;L601
 35058| 
 35059| 327: ; preds = %326
 35062|  %328 = gep %1, i64 448                                                                                                ;L602
 35063|  invoke fastcc void @gc::simulation4game5frameNtB5_14DebugFrameDataNtNtCsjihNppCmMEE_4core5clone5Clone5clone(ptr %6, ptr %328)
 35064|  to label %329 unwind label %124                                                                                       ;L602
 35065| 
 35066| 329: ; preds = %327
 35067|  invoke void @gc::simulation4game5frameNtB4_14DebugFrameData5merge(ptr %1, ptr %6)
 35068|  to label %332 unwind label %330                                                                                       ;L602
 35069| 
 35070| 330: ; preds = %329
 35071|  %331 = cleanuppad within none []
 35072|  call void @core::ptr9drop_glueNtNtNtNtCs97f5S1uJLkH_9game_core10simulation4game5frame14DebugFrameDataECshdEBA0ozCnw_7game_ai(ptr %6) #34 [ "funclet"(token %331) ] ;L602
 35073|  cleanupret from %331 unwind label %124                                                                                ;L602
 35074| 
 35075| 332: ; preds = %329
 35076|  invoke void @core::ptr9drop_glueNtNtNtNtCs97f5S1uJLkH_9game_core10simulation4game5frame14DebugFrameDataECshdEBA0ozCnw_7game_ai(ptr %6)
 35077|  to label %333 unwind label %124                                                                                       ;L602
 35078| 
 35079| 333: ; preds = %332
 35081|  %334 = invoke ptr @ai::small_actionNtB2_15SmallActionPlay11path_finder(ptr %138)
 35082|  to label %335 unwind label %124                                                                                       ;L604
 35083| 
 35084| 335: ; preds = %333
 35085|  %336 = icmp eq ptr %334, null                                                                                         ;L604
 35086|  br i1 %336, label %320, label %337                                                                                    ;L604
 35087| 
 35088| 337: ; preds = %335
 35089|     ;; path_finder = ptr %334
 35090|  invoke void @ai::path_finderNtB2_10PathFinder10draw_debug(ptr %334, ptr %3, ptr %4, ptr %1)
 35091|  to label %320 unwind label %124                                                                                       ;L605
 35092| 
 35093| 338: ; preds = %124
 35094|  cleanupret from %126 unwind label %113
 35095| 
 35096| 339: ; preds = %124
 35097|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %11) #34 [ "funclet"(token %126) ] ;L610
 35098|  cleanupret from %126 unwind label %113                                                                                ;L610
 35099| 
 35100| 340: ; preds = %63
 35101|  cleanupret from %65 unwind to caller
 35102| 
 35103| 341: ; preds = %63
 35104|  call fastcc void @core::ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEECshdEBA0ozCnw_7game_ai(ptr %14) #34 [ "funclet"(token %65) ] ;L610
 35105|  cleanupret from %65 unwind to caller                                                                                  ;L610
 35106| }
