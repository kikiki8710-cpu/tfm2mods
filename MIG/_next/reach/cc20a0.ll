 36309| define void @ai::plan_legacy8sub_plan6battleNtB2_13BattleSubPlan31calculate_score_parameter_value(ptr %0, i64 %1, ptr readnone %2, ptr %3, ptr %4, ptr %5) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
 36310|  %7 = alloca [32 x i8],
 36315|  %8 = alloca [128 x i8],
 36322|     ;; self = ptr %0
 36323|     ;; version = i64 %1
 36324|     ;; _rnd = ptr %2
 36325|     ;; player = ptr %3
 36326|     ;; data = ptr %4
 36327|     ;; parameter = ptr %5
 36329|     ;; trace_non_focus = i64 30
 36330|     ;; count = i64 1
 36331|     ;; count = i64 1
 36332|     ;; count = i64 1
 36333|     ;; count = i64 1
 36334|     ;; count = i64 1
 36335|     ;; count = i64 1
 36337|     ;; count = i64 1
 36338|     ;; count = i64 1
 36341|     ;; count = i64 1
 36342|     ;; count = i64 1
 36343|  %9 = gep %0, i64 16                                                                                                   ;L842
 36344|  %10 = load i64, ptr %9, , !!8                                                                                         ;L842
 36345|  %11 = gep %0, i64 24                                                                                                  ;L842
 36346|  switch i64 %10, label %12 [
 36347|  i64 0, label %29
 36348|  i64 1, label %48
 36349|  i64 2, label %48
 36350|  i64 3, label %48
 36351|  i64 4, label %13
 36352|  i64 5, label %29
 36353|  i64 6, label %19
 36354|  i64 7, label %13
 36355|  ]                                                                                                                     ;L842
 36356| 
 36357| 12: ; preds = %83, %6
 36358|  unreachable
 36359| 
 36360| 13: ; preds = %6, %6
 36361|  %14 = gep %5, i64 2496                                                                                                ;L904
 36362|  store i64 100, ptr %14,                                                                                               ;L904
 36363|  %15 = gep %5, i64 2504                                                                                                ;L905
 36364|  store i64 100, ptr %15,                                                                                               ;L905
 36365|  %16 = gep %3, i64 2352                                                                                                ;L909
 36366|  %17 = load i64, ptr %16, , !!8                                                                                        ;L909
 36367|  %18 = icmp ult i64 %17, 2                                                                                             ;L909
 36368|  br i1 %18, label %131, label %130                                                                                     ;L909
 36369| 
 36370| 19: ; preds = %6
 36371|  %20 = gep %5, i64 2496                                                                                                ;L870
 36372|  store i64 100, ptr %20,                                                                                               ;L870
 36373|  %21 = gep %5, i64 2504                                                                                                ;L871
 36374|  store i64 100, ptr %21,                                                                                               ;L871
 36375|     ;; self = ptr %5
 36376|     ;; self = ptr %5
 36377|  %22 = gep %5, i64 5336                                                                                                ;L138<2083<873
 36378|  %23 = load ptr, ptr %22, , !!8, !!8                                                                                   ;L138<2083<873
 36379|     ;; ptr = ptr %23
 36380|  %24 = gep %5, i64 5360                                                                                                ;L2085<873
 36381|  %25 = load i64, ptr %24, , !!8                                                                                        ;L2085<873
 36382|     ;; len = i64 %25
 36383|     ;; count = i64 %25
 36384|     ;; self[0..+8] = ptr %23
 36385|     ;; slice[0..+8] = ptr %23
 36386|     ;; self[8..+8] = i64 %25
 36387|     ;; slice[8..+8] = i64 %25
 36388|     ;; ptr = ptr %23
 36389|     ;; self = ptr %23
 36390|  %26 = mul nuw nsw i64 %25, 216                                                                                        ;L961<240<1062<873
 36391|  %27 = gep %23, i64 %26                                                                                                ;L961<240<1062<873
 36392|     ;; iter[0..+8] = ptr %23
 36393|     ;; iter[8..+8] = ptr %27
 36394|     ;; self = ptr undef
 36395|     ;; ptr = ptr %23
 36396|     ;; self = ptr %23
 36397|     ;; end_or_len = ptr %27
 36400|  %28 = icmp eq i64 %25, 0                                                                                              ;L1714<180<873
 36401|  br i1 %28, label %116, label %110                                                                                     ;L180<873
 36402| 
 36403| 29: ; preds = %6, %6
 36404|  %30 = load i64, ptr %11, , !!8                                                                                        ;L843
 36405|     ;; focus = i64 %30
 36406|  %31 = gep %5, i64 2496                                                                                                ;L844
 36407|  store i64 30, ptr %31,                                                                                                ;L844
 36408|  %32 = gep %5, i64 2504                                                                                                ;L845
 36409|  store i64 30, ptr %32,                                                                                                ;L845
 36410|     ;; self = ptr %5
 36411|     ;; self = ptr %5
 36412|  %33 = gep %5, i64 5336                                                                                                ;L138<2083<847
 36413|  %34 = load ptr, ptr %33, , !!8, !!8                                                                                   ;L138<2083<847
 36414|     ;; ptr = ptr %34
 36415|  %35 = gep %5, i64 5360                                                                                                ;L2085<847
 36416|  %36 = load i64, ptr %35, , !!8                                                                                        ;L2085<847
 36417|     ;; len = i64 %36
 36418|     ;; count = i64 %36
 36419|     ;; self[0..+8] = ptr %34
 36420|     ;; slice[0..+8] = ptr %34
 36421|     ;; self[8..+8] = i64 %36
 36422|     ;; slice[8..+8] = i64 %36
 36423|     ;; ptr = ptr %34
 36424|     ;; self = ptr %34
 36425|  %37 = mul nuw nsw i64 %36, 216                                                                                        ;L961<240<1062<847
 36426|  %38 = gep %34, i64 %37                                                                                                ;L961<240<1062<847
 36427|     ;; iter[0..+8] = ptr %34
 36428|     ;; iter[8..+8] = ptr %38
 36429|     ;; self = ptr undef
 36430|     ;; ptr = ptr %34
 36431|     ;; self = ptr %34
 36432|     ;; end_or_len = ptr %38
 36435|  %39 = icmp eq i64 %36, 0                                                                                              ;L1714<180<847
 36436|  br i1 %39, label %96, label %40                                                                                       ;L180<847
 36437| 
 36438| 40: ; preds = %29
 36439|  %41 = icmp eq i64 %10, 5
 36440|  br i1 %41, label %42, label %86
 36441| 
 36442| 42: ; preds = %42, %40
 36443|  %43 = phi ptr [ %44, %42 ], [ %34, %40 ]
 36444|  %44 = gep %43, i64 216                                                                                                ;L656<185<847
 36445|     ;; iter[0..+8] = ptr %44
 36446|     ;; e = ptr %43
 36447|  %45 = gep %43, i64 168                                                                                                ;L0
 36448|  store i64 60, ptr %45,                                                                                                ;L0
 36449|  %46 = gep %43, i64 176                                                                                                ;L0
 36450|  store i64 60, ptr %46,                                                                                                ;L0
 36451|     ;; iter[0..+8] = ptr %44
 36452|     ;; self = ptr undef
 36453|     ;; ptr = ptr %44
 36454|     ;; self = ptr %44
 36455|     ;; end_or_len = ptr %38
 36458|  %47 = icmp eq ptr %44, %38                                                                                            ;L1714<180<847
 36459|  br i1 %47, label %96, label %42                                                                                       ;L180<847
 36460| 
 36461| 48: ; preds = %6, %6, %6
 36462|  %49 = load i64, ptr %11, , !!8                                                                                        ;L883
 36463|     ;; focus = i64 %49
 36464|  %50 = gep %5, i64 2496                                                                                                ;L884
 36465|  store i64 50, ptr %50,                                                                                                ;L884
 36466|  %51 = gep %5, i64 2504                                                                                                ;L885
 36467|  store i64 50, ptr %51,                                                                                                ;L885
 36468|     ;; self = ptr %5
 36469|     ;; self = ptr %5
 36470|  %52 = gep %5, i64 5336                                                                                                ;L138<2083<887
 36471|  %53 = load ptr, ptr %52, , !!8, !!8                                                                                   ;L138<2083<887
 36472|     ;; ptr = ptr %53
 36473|  %54 = gep %5, i64 5360                                                                                                ;L2085<887
 36474|  %55 = load i64, ptr %54, , !!8                                                                                        ;L2085<887
 36475|     ;; len = i64 %55
 36476|     ;; count = i64 %55
 36477|     ;; self[0..+8] = ptr %53
 36478|     ;; slice[0..+8] = ptr %53
 36479|     ;; self[8..+8] = i64 %55
 36480|     ;; slice[8..+8] = i64 %55
 36481|     ;; ptr = ptr %53
 36482|     ;; self = ptr %53
 36483|  %56 = mul nuw nsw i64 %55, 216                                                                                        ;L961<240<1062<887
 36484|  %57 = gep %53, i64 %56                                                                                                ;L961<240<1062<887
 36485|     ;; iter[0..+8] = ptr %53
 36486|     ;; iter[8..+8] = ptr %57
 36487|     ;; self = ptr undef
 36488|     ;; ptr = ptr %53
 36489|     ;; self = ptr %53
 36490|     ;; end_or_len = ptr %57
 36493|  %58 = icmp eq i64 %55, 0                                                                                              ;L1714<180<887
 36494|  br i1 %58, label %69, label %59                                                                                       ;L180<887
 36495| 
 36496| 59: ; preds = %59, %48
 36497|  %60 = phi ptr [ %61, %59 ], [ %53, %48 ]
 36498|  %61 = gep %60, i64 216                                                                                                ;L656<185<887
 36499|     ;; iter[0..+8] = ptr %61
 36500|     ;; e = ptr %60
 36501|  %62 = gep %60, i64 88                                                                                                 ;L889
 36502|  %63 = load i64, ptr %62, , !!8                                                                                        ;L889
 36503|  %64 = icmp eq i64 %63, %49                                                                                            ;L889
 36504|  %65 = select i1 %64, i64 80, i64 50                                                                                   ;L889
 36505|  %66 = gep %60, i64 168                                                                                                ;L0
 36506|  store i64 %65, ptr %66,                                                                                               ;L0
 36507|  %67 = gep %60, i64 176                                                                                                ;L0
 36508|  store i64 %65, ptr %67,                                                                                               ;L0
 36509|     ;; iter[0..+8] = ptr %61
 36510|     ;; self = ptr undef
 36511|     ;; ptr = ptr %61
 36512|     ;; self = ptr %61
 36513|     ;; end_or_len = ptr %57
 36516|  %68 = icmp eq ptr %61, %57                                                                                            ;L1714<180<887
 36517|  br i1 %68, label %69, label %59                                                                                       ;L180<887
 36518| 
 36519| 69: ; preds = %59, %48
 36520|     ;; self = ptr %5
 36521|     ;; self = ptr %5
 36522|  %70 = gep %5, i64 5304                                                                                                ;L138<2083<898
 36523|  %71 = load ptr, ptr %70, , !!8, !!8                                                                                   ;L138<2083<898
 36524|     ;; ptr = ptr %71
 36525|  %72 = gep %5, i64 5328                                                                                                ;L2085<898
 36526|  %73 = load i64, ptr %72, , !!8                                                                                        ;L2085<898
 36527|     ;; len = i64 %73
 36528|     ;; count = i64 %73
 36529|     ;; self[0..+8] = ptr %71
 36530|     ;; slice[0..+8] = ptr %71
 36531|     ;; self[8..+8] = i64 %73
 36532|     ;; slice[8..+8] = i64 %73
 36533|     ;; ptr = ptr %71
 36534|     ;; self = ptr %71
 36535|  %74 = mul nuw nsw i64 %73, 216                                                                                        ;L961<240<1062<898
 36536|  %75 = gep %71, i64 %74                                                                                                ;L961<240<1062<898
 36537|     ;; iter[0..+8] = ptr %71
 36538|     ;; iter[8..+8] = ptr %75
 36539|     ;; self = ptr undef
 36540|     ;; ptr = ptr %71
 36541|     ;; self = ptr %71
 36542|     ;; end_or_len = ptr %75
 36545|  %76 = icmp eq i64 %73, 0                                                                                              ;L1714<180<898
 36546|  br i1 %76, label %83, label %77                                                                                       ;L180<898
 36547| 
 36548| 77: ; preds = %77, %69
 36549|  %78 = phi ptr [ %79, %77 ], [ %71, %69 ]
 36550|  %79 = gep %78, i64 216                                                                                                ;L656<185<898
 36551|     ;; iter[0..+8] = ptr %79
 36552|     ;; e = ptr %78
 36553|  %80 = gep %78, i64 168                                                                                                ;L899
 36554|  store i64 50, ptr %80,                                                                                                ;L899
 36555|  %81 = gep %78, i64 176                                                                                                ;L900
 36556|  store i64 50, ptr %81,                                                                                                ;L900
 36557|     ;; self = ptr undef
 36558|     ;; ptr = ptr %79
 36559|     ;; self = ptr %79
 36560|     ;; end_or_len = ptr %75
 36563|  %82 = icmp eq ptr %79, %75                                                                                            ;L1714<180<898
 36564|  br i1 %82, label %83, label %77                                                                                       ;L180<898
 36565| 
 36566| 83: ; preds = %177, %169, %124, %116, %104, %96, %77, %69
 36567|     ;; self = ptr %0
 36568|     ;; self = ptr %0
 36569|  %84 = gep %0, i64 44                                                                                                  ;L10<264<940
 36570|  %85 = load i8, ptr %84, , !!8                                                                                         ;L10<264<940
 36571|  switch i8 %85, label %12 [
 36572|  i8 0, label %183
 36573|  i8 6, label %191
 36574|  i8 1, label %215
 36575|  i8 2, label %184
 36576|  i8 3, label %185
 36577|  i8 4, label %189
 36578|  i8 5, label %190
 36579|  ]                                                                                                                     ;L940
 36580| 
 36581| 86: ; preds = %86, %40
 36582|  %87 = phi ptr [ %88, %86 ], [ %34, %40 ]
 36583|  %88 = gep %87, i64 216                                                                                                ;L656<185<847
 36584|     ;; iter[0..+8] = ptr %88
 36585|     ;; e = ptr %87
 36586|  %89 = gep %87, i64 88                                                                                                 ;L849
 36587|  %90 = load i64, ptr %89, , !!8                                                                                        ;L849
 36588|  %91 = icmp eq i64 %90, %30                                                                                            ;L849
 36589|  %92 = select i1 %91, i64 60, i64 30                                                                                   ;L849
 36590|  %93 = gep %87, i64 168                                                                                                ;L0
 36591|  store i64 %92, ptr %93,                                                                                               ;L0
 36592|  %94 = gep %87, i64 176                                                                                                ;L0
 36593|  store i64 %92, ptr %94,                                                                                               ;L0
 36594|     ;; iter[0..+8] = ptr %88
 36595|     ;; self = ptr undef
 36596|     ;; ptr = ptr %88
 36597|     ;; self = ptr %88
 36598|     ;; end_or_len = ptr %38
 36601|  %95 = icmp eq ptr %88, %38                                                                                            ;L1714<180<847
 36602|  br i1 %95, label %96, label %86                                                                                       ;L180<847
 36603| 
 36604| 96: ; preds = %86, %42, %29
 36605|     ;; self = ptr %5
 36606|     ;; self = ptr %5
 36607|  %97 = gep %5, i64 5304                                                                                                ;L138<2083<864
 36608|  %98 = load ptr, ptr %97, , !!8, !!8                                                                                   ;L138<2083<864
 36609|     ;; ptr = ptr %98
 36610|  %99 = gep %5, i64 5328                                                                                                ;L2085<864
 36611|  %100 = load i64, ptr %99, , !!8                                                                                       ;L2085<864
 36612|     ;; len = i64 %100
 36613|     ;; count = i64 %100
 36614|     ;; self[0..+8] = ptr %98
 36615|     ;; slice[0..+8] = ptr %98
 36616|     ;; self[8..+8] = i64 %100
 36617|     ;; slice[8..+8] = i64 %100
 36618|     ;; ptr = ptr %98
 36619|     ;; self = ptr %98
 36620|  %101 = mul nuw nsw i64 %100, 216                                                                                      ;L961<240<1062<864
 36621|  %102 = gep %98, i64 %101                                                                                              ;L961<240<1062<864
 36622|     ;; iter[0..+8] = ptr %98
 36623|     ;; iter[8..+8] = ptr %102
 36624|     ;; self = ptr undef
 36625|     ;; ptr = ptr %98
 36626|     ;; self = ptr %98
 36627|     ;; end_or_len = ptr %102
 36630|  %103 = icmp eq i64 %100, 0                                                                                            ;L1714<180<864
 36631|  br i1 %103, label %83, label %104                                                                                     ;L180<864
 36632| 
 36633| 104: ; preds = %104, %96
 36634|  %105 = phi ptr [ %106, %104 ], [ %98, %96 ]
 36635|  %106 = gep %105, i64 216                                                                                              ;L656<185<864
 36636|     ;; iter[0..+8] = ptr %106
 36637|     ;; e = ptr %105
 36638|  %107 = gep %105, i64 168                                                                                              ;L865
 36639|  store i64 30, ptr %107,                                                                                               ;L865
 36640|  %108 = gep %105, i64 176                                                                                              ;L866
 36641|  store i64 30, ptr %108,                                                                                               ;L866
 36642|     ;; self = ptr undef
 36643|     ;; ptr = ptr %106
 36644|     ;; self = ptr %106
 36645|     ;; end_or_len = ptr %102
 36648|  %109 = icmp eq ptr %106, %102                                                                                         ;L1714<180<864
 36649|  br i1 %109, label %83, label %104                                                                                     ;L180<864
 36650| 
 36651| 110: ; preds = %110, %19
 36652|  %111 = phi ptr [ %112, %110 ], [ %23, %19 ]
 36653|  %112 = gep %111, i64 216                                                                                              ;L656<185<873
 36654|     ;; iter[0..+8] = ptr %112
 36655|     ;; e = ptr %111
 36656|  %113 = gep %111, i64 168                                                                                              ;L874
 36657|  store i64 10, ptr %113,                                                                                               ;L874
 36658|  %114 = gep %111, i64 176                                                                                              ;L875
 36659|  store i64 10, ptr %114,                                                                                               ;L875
 36660|     ;; self = ptr undef
 36661|     ;; ptr = ptr %112
 36662|     ;; self = ptr %112
 36663|     ;; end_or_len = ptr %27
 36666|  %115 = icmp eq ptr %112, %27                                                                                          ;L1714<180<873
 36667|  br i1 %115, label %116, label %110                                                                                    ;L180<873
 36668| 
 36669| 116: ; preds = %110, %19
 36670|     ;; self = ptr %5
 36671|     ;; self = ptr %5
 36672|  %117 = gep %5, i64 5304                                                                                               ;L138<2083<878
 36673|  %118 = load ptr, ptr %117, , !!8, !!8                                                                                 ;L138<2083<878
 36674|     ;; ptr = ptr %118
 36675|  %119 = gep %5, i64 5328                                                                                               ;L2085<878
 36676|  %120 = load i64, ptr %119, , !!8                                                                                      ;L2085<878
 36677|     ;; len = i64 %120
 36678|     ;; count = i64 %120
 36679|     ;; self[0..+8] = ptr %118
 36680|     ;; slice[0..+8] = ptr %118
 36681|     ;; self[8..+8] = i64 %120
 36682|     ;; slice[8..+8] = i64 %120
 36683|     ;; ptr = ptr %118
 36684|     ;; self = ptr %118
 36685|  %121 = mul nuw nsw i64 %120, 216                                                                                      ;L961<240<1062<878
 36686|  %122 = gep %118, i64 %121                                                                                             ;L961<240<1062<878
 36687|     ;; iter[0..+8] = ptr %118
 36688|     ;; iter[8..+8] = ptr %122
 36689|     ;; self = ptr undef
 36690|     ;; ptr = ptr %118
 36691|     ;; self = ptr %118
 36692|     ;; end_or_len = ptr %122
 36695|  %123 = icmp eq i64 %120, 0                                                                                            ;L1714<180<878
 36696|  br i1 %123, label %83, label %124                                                                                     ;L180<878
 36697| 
 36698| 124: ; preds = %124, %116
 36699|  %125 = phi ptr [ %126, %124 ], [ %118, %116 ]
 36700|  %126 = gep %125, i64 216                                                                                              ;L656<185<878
 36701|     ;; iter[0..+8] = ptr %126
 36702|     ;; e = ptr %125
 36703|  %127 = gep %125, i64 168                                                                                              ;L879
 36704|  store i64 10, ptr %127,                                                                                               ;L879
 36705|  %128 = gep %125, i64 176                                                                                              ;L880
 36706|  store i64 10, ptr %128,                                                                                               ;L880
 36707|     ;; self = ptr undef
 36708|     ;; ptr = ptr %126
 36709|     ;; self = ptr %126
 36710|     ;; end_or_len = ptr %122
 36713|  %129 = icmp eq ptr %126, %122                                                                                         ;L1714<180<878
 36714|  br i1 %129, label %83, label %124                                                                                     ;L180<878
 36715| 
 36716| 130: ; preds = %13
 36717|  tail call void @core::panicking18panic_bounds_check(i64 %17, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.172) #31 ;L909
 36718|  unreachable                                                                                                           ;L909
 36719| 
 36720| 131: ; preds = %13
 36721|     ;; self = ptr %3
 36722|  %132 = gep %3, i64 2496                                                                                               ;L581<909
 36723|  %133 = load i32, ptr %132, , !!8                                                                                      ;L581<909
 36724|  %134 = zext nneg i32 %133 to i64                                                                                      ;L581<909
 36725|  %135 = load ptr, ptr %4, , !!8, !!8                                                                                   ;L909
 36726|  %136 = gep %135, i64 480                                                                                              ;L909
 36727|  %137 = getelementptr [5 x ptr], ptr %136, i64 %17                                                                     ;L909
 36728|  %138 = getelementptr ptr, ptr %137, i64 %134                                                                          ;L909
 36729|  %139 = load ptr, ptr %138, , !!8                                                                                      ;L909
 36730|     ;; self = ptr %139
 36731|  %140 = icmp eq ptr %139, null                                                                                         ;L1011<909
 36732|  br i1 %140, label %162, label %141                                                                                    ;L1011<909
 36733| 
 36734| 141: ; preds = %131
 36735|     ;; champ = ptr %139
 36736|  %142 = load i64, ptr %0, , !!8                                                                                        ;L910
 36737|  %143 = gep %0, i64 8                                                                                                  ;L910
 36738|  %144 = load i64, ptr %143,                                                                                            ;L910
 36739|  %145 = tail call zeroext i1 @ai::plan_legacy8sub_plan13battle_common29v15_can_keep_support_pressure(i64 %1, ptr %3, ptr %4, ptr %5, i64 %142, i64 %144) ;L910
 36740|     ;; can_support_fire = i1 %145
 36742|  call void @gc::simulationNtB5_21AbstractGameWithCache25iter_towers_without_nexus(ptr sret([120 x i8]) %8, ptr %135, i64 %17) ;L911
 36743|     ;; predicate = ptr %139
 36744|  %146 = gep %8, i64 120                                                                                                ;L28<957<912
 36745|  store ptr %139, ptr %146,                                                                                             ;L28<957<912
 36746|     ;; self = ptr %8
 36747|     ;; self = ptr %8
 36748|  %147 = gep %4, i64 16                                                                                                 ;L913
 36749|  %148 = load ptr, ptr %147, , !!8, !!8                                                                                 ;L913
 36750|     ;; f[0..+8] = ptr %135
 36751|     ;; f[8..+8] = ptr %148
 36752|     ;; f[16..+8] = ptr %3
 36753|     ;; fold[0..+8] = ptr %135
 36754|     ;; fold[0..+8] = ptr %135
 36755|     ;; fold[8..+8] = ptr %148
 36756|     ;; fold[8..+8] = ptr %148
 36757|     ;; fold[16..+8] = ptr %3
 36758|     ;; fold[16..+8] = ptr %3
 36760|     ;; predicate = ptr %146
 36761|  store ptr %146, ptr %7,                                                                                               ;L86<157<2897<913
 36762|  %149 = gep %7, i64 8                                                                                                  ;L86<157<2897<913
 36763|  store ptr %135, ptr %149,                                                                                             ;L86<157<2897<913
 36764|  %150 = gep %7, i64 16                                                                                                 ;L86<157<2897<913
 36765|  store ptr %148, ptr %150,                                                                                             ;L86<157<2897<913
 36766|  %151 = gep %7, i64 24                                                                                                 ;L86<157<2897<913
 36767|  store ptr %3, ptr %151,                                                                                               ;L86<157<2897<913
 36768|  %152 = call zeroext i1 @core::iter8adapters5chainINtB5_5ChainINtNtB7_7flatten7FlattenINtNtNtBb_5array4iter8IntoIterINtNtBb_6option6OptionRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEKj6_EEINtNtB7_6copied6CopiedINtNtNtBb_5slice4iter4IterB2e_EEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNtB7_6filter15filter_try_foldB2e_uINtNtNtBb_3ops12control_flow11ControlFlowuENCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6battleNtB6g_13BattleSubPlan31calculate_score_parameter_value0NCINvNvB49_3any5checkB2e_NCB6d_s_0E0E0B5u_EB6m_(ptr %8, ptr %7) ;L157<2897<913
 36770|     ;; near_tower_with_enemy = i1 %152
 36772|  %153 = select i1 %145, i64 35, i64 10                                                                                 ;L924
 36773|  %154 = select i1 %152, i64 50, i64 %153                                                                               ;L924
 36774|     ;; enemy_value = i64 %154
 36775|     ;; self = ptr %5
 36776|     ;; self = ptr %5
 36777|  %155 = gep %5, i64 5336                                                                                               ;L138<2083<927
 36778|  %156 = load ptr, ptr %155, , !!8, !!8                                                                                 ;L138<2083<927
 36779|     ;; ptr = ptr %156
 36780|  %157 = gep %5, i64 5360                                                                                               ;L2085<927
 36781|  %158 = load i64, ptr %157, , !!8                                                                                      ;L2085<927
 36782|     ;; len = i64 %158
 36783|     ;; count = i64 %158
 36784|     ;; self[0..+8] = ptr %156
 36785|     ;; slice[0..+8] = ptr %156
 36786|     ;; self[8..+8] = i64 %158
 36787|     ;; slice[8..+8] = i64 %158
 36788|     ;; ptr = ptr %156
 36789|     ;; self = ptr %156
 36790|  %159 = mul nuw nsw i64 %158, 216                                                                                      ;L961<240<1062<927
 36791|  %160 = gep %156, i64 %159                                                                                             ;L961<240<1062<927
 36792|     ;; iter[0..+8] = ptr %156
 36793|     ;; iter[8..+8] = ptr %160
 36794|     ;; self = ptr undef
 36795|     ;; ptr = ptr %156
 36796|     ;; self = ptr %156
 36797|     ;; end_or_len = ptr %160
 36800|  %161 = icmp eq i64 %158, 0                                                                                            ;L1714<180<927
 36801|  br i1 %161, label %169, label %163                                                                                    ;L180<927
 36802| 
 36803| 162: ; preds = %131
 36804|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.173) #31                       ;L1013<909
 36805|  unreachable                                                                                                           ;L1013<909
 36806| 
 36807| 163: ; preds = %163, %141
 36808|  %164 = phi ptr [ %165, %163 ], [ %156, %141 ]
 36809|  %165 = gep %164, i64 216                                                                                              ;L656<185<927
 36810|     ;; iter[0..+8] = ptr %165
 36811|     ;; e = ptr %164
 36812|  %166 = gep %164, i64 168                                                                                              ;L928
 36813|  store i64 %154, ptr %166,                                                                                             ;L928
 36814|  %167 = gep %164, i64 176                                                                                              ;L929
 36815|  store i64 %154, ptr %167,                                                                                             ;L929
 36816|     ;; self = ptr undef
 36817|     ;; ptr = ptr %165
 36818|     ;; self = ptr %165
 36819|     ;; end_or_len = ptr %160
 36822|  %168 = icmp eq ptr %165, %160                                                                                         ;L1714<180<927
 36823|  br i1 %168, label %169, label %163                                                                                    ;L180<927
 36824| 
 36825| 169: ; preds = %163, %141
 36826|     ;; self = ptr %5
 36827|     ;; self = ptr %5
 36828|  %170 = gep %5, i64 5304                                                                                               ;L138<2083<932
 36829|  %171 = load ptr, ptr %170, , !!8, !!8                                                                                 ;L138<2083<932
 36830|     ;; ptr = ptr %171
 36831|  %172 = gep %5, i64 5328                                                                                               ;L2085<932
 36832|  %173 = load i64, ptr %172, , !!8                                                                                      ;L2085<932
 36833|     ;; len = i64 %173
 36834|     ;; count = i64 %173
 36835|     ;; self[0..+8] = ptr %171
 36836|     ;; slice[0..+8] = ptr %171
 36837|     ;; self[8..+8] = i64 %173
 36838|     ;; slice[8..+8] = i64 %173
 36839|     ;; ptr = ptr %171
 36840|     ;; self = ptr %171
 36841|  %174 = mul nuw nsw i64 %173, 216                                                                                      ;L961<240<1062<932
 36842|  %175 = gep %171, i64 %174                                                                                             ;L961<240<1062<932
 36843|     ;; iter[0..+8] = ptr %171
 36844|     ;; iter[8..+8] = ptr %175
 36845|     ;; self = ptr undef
 36846|     ;; ptr = ptr %171
 36847|     ;; self = ptr %171
 36848|     ;; end_or_len = ptr %175
 36851|  %176 = icmp eq i64 %173, 0                                                                                            ;L1714<180<932
 36852|  br i1 %176, label %83, label %177                                                                                     ;L180<932
 36853| 
 36854| 177: ; preds = %177, %169
 36855|  %178 = phi ptr [ %179, %177 ], [ %171, %169 ]
 36856|  %179 = gep %178, i64 216                                                                                              ;L656<185<932
 36857|     ;; iter[0..+8] = ptr %179
 36858|     ;; e = ptr %178
 36859|  %180 = gep %178, i64 168                                                                                              ;L933
 36860|  store i64 100, ptr %180,                                                                                              ;L933
 36861|  %181 = gep %178, i64 176                                                                                              ;L934
 36862|  store i64 100, ptr %181,                                                                                              ;L934
 36863|     ;; self = ptr undef
 36864|     ;; ptr = ptr %179
 36865|     ;; self = ptr %179
 36866|     ;; end_or_len = ptr %175
 36869|  %182 = icmp eq ptr %179, %175                                                                                         ;L1714<180<932
 36870|  br i1 %182, label %83, label %177                                                                                     ;L180<932
 36871| 
 36872| 183: ; preds = %255, %239, %83
 36873|  ret void                                                                                                              ;L968
 36874| 
 36875| 184: ; preds = %83
 36876|     ;; modifier[32..+8] = i64 150
 36877|     ;; modifier[40..+8] = i64 0
 36880|     ;; modifier[48..+8] = i64 90
 36881|     ;; modifier[56..+8] = i64 90
 36882|     ;; modifier[80..+1] = i8 0
 36883|     ;; modifier[81..+1] = i8 0
 36884|     ;; modifier[64..+8] = i64 130
 36885|     ;; modifier[72..+8] = i64 100
 36888|  br label %215                                                                                                         ;L99<941
 36889| 
 36890| 185: ; preds = %83
 36891|     ;; modifier[32..+8] = i64 100
 36892|     ;; modifier[40..+8] = i64 0
 36895|     ;; modifier[48..+8] = i64 120
 36896|     ;; modifier[56..+8] = i64 100
 36897|     ;; modifier[80..+1] = i8 0
 36898|     ;; modifier[81..+1] = i8 0
 36899|     ;; modifier[64..+8] = i64 130
 36900|     ;; modifier[72..+8] = i64 100
 36902|     ;; modifier[24..+8] = i64 60
 36903|     ;; enemy_atk_mult = i64 60
 36904|     ;; reduced = i64 60
 36905|  %186 = gep %3, i64 2352                                                                                               ;L945
 36906|  %187 = load i64, ptr %186, , !!8                                                                                      ;L945
 36907|  %188 = icmp ult i64 %187, 2                                                                                           ;L945
 36908|  br i1 %188, label %193, label %192                                                                                    ;L945
 36909| 
 36910| 189: ; preds = %83
 36911|     ;; modifier[32..+8] = i64 100
 36912|     ;; modifier[40..+8] = i64 0
 36915|     ;; modifier[48..+8] = i64 70
 36916|     ;; modifier[56..+8] = i64 150
 36917|     ;; modifier[80..+1] = i8 0
 36918|     ;; modifier[81..+1] = i8 1
 36919|     ;; modifier[64..+8] = i64 110
 36920|     ;; modifier[72..+8] = i64 140
 36923|  br label %215                                                                                                         ;L113<941
 36924| 
 36925| 190: ; preds = %83
 36926|     ;; modifier[32..+8] = i64 50
 36927|     ;; modifier[40..+8] = i64 0
 36930|     ;; modifier[48..+8] = i64 130
 36931|     ;; modifier[56..+8] = i64 100
 36932|     ;; modifier[80..+1] = i8 0
 36933|     ;; modifier[81..+1] = i8 0
 36934|     ;; modifier[64..+8] = i64 90
 36935|     ;; modifier[72..+8] = i64 100
 36938|  br label %215                                                                                                         ;L119<941
 36939| 
 36940| 191: ; preds = %83
 36941|     ;; modifier[32..+8] = i64 200
 36942|     ;; modifier[40..+8] = i64 0
 36945|     ;; modifier[48..+8] = i64 100
 36946|     ;; modifier[56..+8] = i64 100
 36947|     ;; modifier[80..+1] = i8 0
 36948|     ;; modifier[81..+1] = i8 0
 36949|     ;; modifier[64..+8] = i64 150
 36950|     ;; modifier[72..+8] = i64 100
 36953|  br label %215                                                                                                         ;L124<941
 36954| 
 36955| 192: ; preds = %185
 36956|  call void @core::panicking18panic_bounds_check(i64 %187, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.174) #31   ;L945
 36957|  unreachable                                                                                                           ;L945
 36958| 
 36959| 193: ; preds = %185
 36960|     ;; self = ptr %3
 36961|  %194 = gep %3, i64 2496                                                                                               ;L581<945
 36962|  %195 = load i32, ptr %194, , !!8                                                                                      ;L581<945
 36963|  %196 = zext nneg i32 %195 to i64                                                                                      ;L581<945
 36964|  %197 = load ptr, ptr %4, , !!8, !!8                                                                                   ;L945
 36965|  %198 = gep %197, i64 480                                                                                              ;L945
 36966|  %199 = getelementptr [5 x ptr], ptr %198, i64 %187                                                                    ;L945
 36967|  %200 = getelementptr ptr, ptr %199, i64 %196                                                                          ;L945
 36968|  %201 = load ptr, ptr %200, , !!8                                                                                      ;L945
 36969|     ;; self = ptr %201
 36970|  %202 = icmp eq ptr %201, null                                                                                         ;L1011<945
 36971|  br i1 %202, label %214, label %203                                                                                    ;L1011<945
 36972| 
 36973| 203: ; preds = %193
 36974|     ;; champ = ptr %201
 36975|  %204 = call zeroext i1 @gc::simulation6entityNtB5_6Entity9can_skill(ptr %201)                                         ;L946
 36976|  %205 = zext i1 %204 to i64                                                                                            ;L946
 36977|  %206 = call zeroext i1 @gc::simulation6entityNtB5_6Entity10can_skill2(ptr %201)                                       ;L947
 36978|  %207 = zext i1 %206 to i64                                                                                            ;L947
 36979|  %208 = add nuw nsw i64 %207, %205                                                                                     ;L946
 36980|  %209 = call zeroext i1 @gc::simulation6entityNtB5_6Entity7can_ult(ptr %201)                                           ;L948
 36981|  %210 = zext i1 %209 to i64                                                                                            ;L948
 36982|     ;; usable = !DIArgList(i64 %208, i64 %210)
 36983|  %211 = or i64 %208, %210                                                                                              ;L949
 36984|  %212 = icmp eq i64 %211, 0                                                                                            ;L949
 36985|  %213 = select i1 %212, i64 60, i64 120                                                                                ;L949
 36986|  br label %215                                                                                                         ;L949
 36987| 
 36988| 214: ; preds = %193
 36989|  call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.175) #31                            ;L1013<945
 36990|  unreachable                                                                                                           ;L1013<945
 36991| 
 36992| 215: ; preds = %203, %191, %190, %189, %184, %83
 36993|  %216 = phi i64 [ 100, %83 ], [ %213, %203 ], [ 100, %191 ], [ 130, %190 ], [ 90, %184 ], [ 70, %189 ]                 ;L0
 36994|  %217 = phi i64 [ 100, %83 ], [ 100, %203 ], [ 100, %191 ], [ 100, %190 ], [ 90, %184 ], [ 150, %189 ]                 ;L0<941
 36995|  %218 = phi i64 [ 70, %83 ], [ 130, %203 ], [ 150, %191 ], [ 90, %190 ], [ 130, %184 ], [ 110, %189 ]                  ;L0<941
 36996|  %219 = phi i64 [ 140, %83 ], [ 100, %203 ], [ 100, %191 ], [ 100, %190 ], [ 100, %184 ], [ 140, %189 ]                ;L0<941
 36997|     ;; modifier[72..+8] = i64 %219
 36998|     ;; modifier[64..+8] = i64 %218
 36999|     ;; modifier[56..+8] = i64 %217
 37000|     ;; reduced = i64 %216
 37001|     ;; enemy_atk_mult = i64 %216
 37002|     ;; self = ptr %5
 37003|     ;; self = ptr %5
 37004|  %220 = gep %5, i64 5336                                                                                               ;L138<2083<955
 37005|  %221 = load ptr, ptr %220, , !!8, !!8                                                                                 ;L138<2083<955
 37006|     ;; ptr = ptr %221
 37007|  %222 = gep %5, i64 5360                                                                                               ;L2085<955
 37008|  %223 = load i64, ptr %222, , !!8                                                                                      ;L2085<955
 37009|     ;; len = i64 %223
 37010|     ;; count = i64 %223
 37011|     ;; self[0..+8] = ptr %221
 37012|     ;; slice[0..+8] = ptr %221
 37013|     ;; self[8..+8] = i64 %223
 37014|     ;; slice[8..+8] = i64 %223
 37015|     ;; ptr = ptr %221
 37016|     ;; self = ptr %221
 37017|  %224 = mul nuw nsw i64 %223, 216                                                                                      ;L961<240<1062<955
 37018|  %225 = gep %221, i64 %224                                                                                             ;L961<240<1062<955
 37019|     ;; iter[0..+8] = ptr %221
 37020|     ;; iter[8..+8] = ptr %225
 37021|     ;; self = ptr undef
 37022|     ;; ptr = ptr %221
 37023|     ;; self = ptr %221
 37024|     ;; end_or_len = ptr %225
 37027|  %226 = icmp eq i64 %223, 0                                                                                            ;L1714<180<955
 37028|  br i1 %226, label %239, label %227                                                                                    ;L180<955
 37029| 
 37030| 227: ; preds = %227, %215
 37031|  %228 = phi ptr [ %229, %227 ], [ %221, %215 ]
 37032|  %229 = gep %228, i64 216                                                                                              ;L656<185<955
 37033|     ;; iter[0..+8] = ptr %229
 37034|     ;; e = ptr %228
 37035|  %230 = gep %228, i64 168                                                                                              ;L956
 37036|  %231 = load i64, ptr %230, , !!8                                                                                      ;L956
 37037|  %232 = mul i64 %231, %216                                                                                             ;L956
 37038|  %233 = sdiv i64 %232, 100                                                                                             ;L956
 37039|  store i64 %233, ptr %230,                                                                                             ;L956
 37040|  %234 = gep %228, i64 176                                                                                              ;L957
 37041|  %235 = load i64, ptr %234, , !!8                                                                                      ;L957
 37042|  %236 = mul i64 %235, %217                                                                                             ;L957
 37043|  %237 = sdiv i64 %236, 100                                                                                             ;L957
 37044|  store i64 %237, ptr %234,                                                                                             ;L957
 37045|     ;; self = ptr undef
 37046|     ;; ptr = ptr %229
 37047|     ;; self = ptr %229
 37048|     ;; end_or_len = ptr %225
 37051|  %238 = icmp eq ptr %229, %225                                                                                         ;L1714<180<955
 37052|  br i1 %238, label %239, label %227                                                                                    ;L180<955
 37053| 
 37054| 239: ; preds = %227, %215
 37055|  %240 = gep %5, i64 2496                                                                                               ;L961
 37056|  %241 = load i64, ptr %240, , !!8                                                                                      ;L961
 37057|  %242 = mul i64 %241, %218                                                                                             ;L961
 37058|  %243 = sdiv i64 %242, 100                                                                                             ;L961
 37059|  store i64 %243, ptr %240,                                                                                             ;L961
 37060|  %244 = gep %5, i64 2504                                                                                               ;L962
 37061|  %245 = load i64, ptr %244, , !!8                                                                                      ;L962
 37062|  %246 = mul i64 %245, %218                                                                                             ;L962
 37063|  %247 = sdiv i64 %246, 100                                                                                             ;L962
 37064|  store i64 %247, ptr %244,                                                                                             ;L962
 37065|     ;; self = ptr %5
 37066|     ;; self = ptr %5
 37067|  %248 = gep %5, i64 5304                                                                                               ;L138<2083<963
 37068|  %249 = load ptr, ptr %248, , !!8, !!8                                                                                 ;L138<2083<963
 37069|     ;; ptr = ptr %249
 37070|  %250 = gep %5, i64 5328                                                                                               ;L2085<963
 37071|  %251 = load i64, ptr %250, , !!8                                                                                      ;L2085<963
 37072|     ;; len = i64 %251
 37073|     ;; count = i64 %251
 37074|     ;; self[0..+8] = ptr %249
 37075|     ;; slice[0..+8] = ptr %249
 37076|     ;; self[8..+8] = i64 %251
 37077|     ;; slice[8..+8] = i64 %251
 37078|     ;; ptr = ptr %249
 37079|     ;; self = ptr %249
 37080|  %252 = mul nuw nsw i64 %251, 216                                                                                      ;L961<240<1062<963
 37081|  %253 = gep %249, i64 %252                                                                                             ;L961<240<1062<963
 37082|     ;; iter[0..+8] = ptr %249
 37083|     ;; iter[8..+8] = ptr %253
 37084|     ;; self = ptr undef
 37085|     ;; ptr = ptr %249
 37086|     ;; self = ptr %249
 37087|     ;; end_or_len = ptr %253
 37090|  %254 = icmp eq i64 %251, 0                                                                                            ;L1714<180<963
 37091|  br i1 %254, label %183, label %255                                                                                    ;L180<963
 37092| 
 37093| 255: ; preds = %255, %239
 37094|  %256 = phi ptr [ %257, %255 ], [ %249, %239 ]
 37095|  %257 = gep %256, i64 216                                                                                              ;L656<185<963
 37096|     ;; iter[0..+8] = ptr %257
 37097|     ;; a = ptr %256
 37098|  %258 = gep %256, i64 168                                                                                              ;L964
 37099|  %259 = load i64, ptr %258, , !!8                                                                                      ;L964
 37100|  %260 = mul i64 %259, %219                                                                                             ;L964
 37101|  %261 = sdiv i64 %260, 100                                                                                             ;L964
 37102|  store i64 %261, ptr %258,                                                                                             ;L964
 37103|  %262 = gep %256, i64 176                                                                                              ;L965
 37104|  %263 = load i64, ptr %262, , !!8                                                                                      ;L965
 37105|  %264 = mul i64 %263, %219                                                                                             ;L965
 37106|  %265 = sdiv i64 %264, 100                                                                                             ;L965
 37107|  store i64 %265, ptr %262,                                                                                             ;L965
 37108|     ;; self = ptr undef
 37109|     ;; ptr = ptr %257
 37110|     ;; self = ptr %257
 37111|     ;; end_or_len = ptr %253
 37114|  %266 = icmp eq ptr %257, %253                                                                                         ;L1714<180<963
 37115|  br i1 %266, label %183, label %255                                                                                    ;L180<963
 37116| }
