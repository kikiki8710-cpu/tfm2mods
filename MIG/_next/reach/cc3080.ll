 37119| define i64 @ai::plan_legacy8sub_plan6battleNtB2_13BattleSubPlan5score(ptr %0, i64 %1, ptr %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %7) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
 37120|  %9 = alloca [288 x i8],
 37122|  %10 = alloca [56 x i8],
 37123|     ;; self = ptr %0
 37124|     ;; version = i64 %1
 37125|     ;; parameter = ptr %2
 37126|     ;; rnd = ptr %3
 37127|     ;; player = ptr %4
 37128|     ;; data = ptr %5
 37129|     ;; action = ptr %6
 37130|     ;; debug = ptr %7
 37131|     ;; ult_effect = ptr %10
 37132|     ;; default = i64 0
 37133|     ;; team = i64 1
 37134|     ;; self = i64 80000
 37135|     ;; default = i64 0
 37136|     ;; team = i64 1
 37137|     ;; self = i64 80000
 37138|     ;; default = i64 0
 37139|     ;; team = i64 1
 37140|     ;; val = i64 1
 37141|     ;; order = i8 0
 37142|     ;; val = i64 1
 37143|     ;; order = i8 0
 37144|     ;; self = i64 80000
 37145|     ;; default = i64 0
 37146|     ;; team = i64 1
 37147|  %11 = gep %4, i64 2352                                                                                                ;L617
 37148|  %12 = load i64, ptr %11, , !!8                                                                                        ;L617
 37149|  %13 = icmp ult i64 %12, 2                                                                                             ;L617
 37150|  br i1 %13, label %15, label %14                                                                                       ;L617
 37151| 
 37152| 14: ; preds = %8
 37153|  tail call void @core::panicking18panic_bounds_check(i64 %12, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.176) #31 ;L617
 37154|  unreachable                                                                                                           ;L617
 37155| 
 37156| 15: ; preds = %8
 37157|     ;; self = ptr %4
 37158|  %16 = gep %4, i64 2496                                                                                                ;L581<617
 37159|  %17 = load i32, ptr %16, , !!8                                                                                        ;L581<617
 37160|  %18 = zext nneg i32 %17 to i64                                                                                        ;L581<617
 37161|  %19 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L617
 37162|  %20 = gep %19, i64 480                                                                                                ;L617
 37163|  %21 = getelementptr [5 x ptr], ptr %20, i64 %12                                                                       ;L617
 37164|  %22 = getelementptr ptr, ptr %21, i64 %18                                                                             ;L617
 37165|  %23 = load ptr, ptr %22, , !!8                                                                                        ;L617
 37166|     ;; self = ptr %23
 37167|     ;; self = ptr %23
 37168|     ;; self = ptr %23
 37169|     ;; self = ptr %23
 37170|     ;; self = ptr %23
 37171|  %24 = icmp eq ptr %23, null                                                                                           ;L1011<617
 37172|  br i1 %24, label %34, label %25                                                                                       ;L1011<617
 37173| 
 37174| 25: ; preds = %15
 37175|     ;; champ = ptr %23
 37176|     ;; self = ptr %23
 37177|     ;; self = ptr %23
 37178|     ;; self = ptr %23
 37179|     ;; x = ptr %23
 37180|     ;; self = ptr %23
 37181|     ;; x = ptr %23
 37182|     ;; self = ptr %23
 37183|     ;; self = ptr %23
 37184|     ;; x = ptr %23
 37185|     ;; self = ptr %23
 37186|     ;; self = ptr %23
 37187|     ;; x = ptr %23
 37188|     ;; other = ptr %23
 37189|     ;; other = ptr %23
 37190|  %26 = tail call i64 @ai::action_score17interaction_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %6, ptr %7)      ;L618
 37191|     ;; base = i64 %26
 37192|     ;; self = ptr %6
 37193|  %27 = gep %6, i64 177                                                                                                 ;L309<620
 37194|  %28 = load i8, ptr %27, , !!44509, !!8                                                                                ;L309<620
 37195|  %29 = icmp ne i8 %28, 10                                                                                              ;L309<620
 37196|  tail call void @llvm.assume(i1 %29)                                                                                   ;L309<620
 37197|  %30 = add nsw i8 %28, -3                                                                                              ;L309<620
 37198|  %31 = icmp samesign ugt i8 %28, 2                                                                                     ;L309<620
 37199|  %32 = select i1 %31, i8 %30, i8 7                                                                                     ;L309<620
 37200|  switch i8 %32, label %33 [
 37201|  i8 0, label %35
 37202|  i8 1, label %35
 37203|  i8 2, label %624
 37204|  i8 3, label %624
 37205|  i8 4, label %624
 37206|  i8 5, label %35
 37207|  i8 6, label %624
 37208|  i8 7, label %624
 37209|  i8 8, label %624
 37210|  i8 9, label %624
 37211|  i8 10, label %624
 37212|  i8 11, label %42
 37213|  i8 12, label %48
 37214|  i8 13, label %58
 37215|  i8 14, label %65
 37216|  i8 15, label %76
 37217|  i8 16, label %624
 37218|  ]                                                                                                                     ;L309<620
 37219| 
 37220| 33: ; preds = %25
 37221|  unreachable                                                                                                           ;L309<620
 37222| 
 37223| 34: ; preds = %15
 37224|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.177) #31                       ;L1013<617
 37225|  unreachable                                                                                                           ;L1013<617
 37226| 
 37227| 35: ; preds = %25, %25, %25
 37228|  %36 = tail call i64 @ai::plan_legacy8sub_plan13battle_common33v17_runaway_counterattack_penalty(i64 %1, ptr %4, ptr %5, ptr %2, ptr %23) ;L799
 37229|     ;; counterattack_penalty = i64 %36
 37230|  %37 = tail call i64 @ai::plan_legacy8sub_plan13battle_common37v21_runaway_defensive_cc_hold_penalty(i64 %1, ptr %4, ptr %5, ptr %2, ptr %23) ;L800
 37231|     ;; defensive_cc_penalty = i64 %37
 37232|  %38 = load i64, ptr %0, , !!8                                                                                         ;L801
 37233|  %39 = gep %0, i64 8                                                                                                   ;L801
 37234|  %40 = load i64, ptr %39,                                                                                              ;L801
 37235|  %41 = tail call zeroext i1 @ai::plan_legacy8sub_plan13battle_common29v15_can_keep_support_pressure(i64 %1, ptr %4, ptr %5, ptr %2, i64 %38, i64 %40) ;L801
 37236|  br i1 %41, label %90, label %87                                                                                       ;L801
 37237| 
 37238| 42: ; preds = %25
 37239|     ;; action = ptr %6
 37240|     ;; self = ptr %6
 37241|  %43 = gep %6, i64 96                                                                                                  ;L404<321<620
 37242|  %44 = load i64, ptr %43, , !!44509, !!8                                                                               ;L404<321<620
 37243|     ;; target_id = i64 %44
 37244|  %45 = gep %0, i64 40                                                                                                  ;L808
 37245|  %46 = load i8, ptr %45, , !!8                                                                                         ;L808
 37246|  %47 = trunc nuw i8 %46 to i1                                                                                          ;L808
 37247|  br i1 %47, label %101, label %94                                                                                      ;L808
 37248| 
 37249| 48: ; preds = %25
 37250|     ;; action = ptr %6
 37251|     ;; self = ptr %6
 37252|  %49 = gep %6, i64 8                                                                                                   ;L94<322<620
 37253|  %50 = load i64, ptr %49, , !!44509, !!8                                                                               ;L94<322<620
 37254|     ;; target_id = i64 %50
 37255|  %51 = load ptr, ptr %19, , !!8, !!8                                                                                   ;L622
 37256|  %52 = gep %19, i64 8                                                                                                  ;L622
 37257|  %53 = load ptr, ptr %52, , !!8, !!8                                                                                   ;L622
 37258|  %54 = gep %53, i64 496                                                                                                ;L622
 37259|  %55 = load ptr, ptr %54, , !!8                                                                                        ;L622
 37260|  %56 = tail call ptr %55(ptr %51, i64 %50)                                                                             ;L622
 37261|  %57 = icmp eq ptr %56, null                                                                                           ;L622
 37262|  br i1 %57, label %624, label %185                                                                                     ;L622
 37263| 
 37264| 58: ; preds = %25
 37265|     ;; action = ptr %6
 37266|     ;; self = ptr %6
 37267|  %59 = gep %6, i64 8                                                                                                   ;L160<323<620
 37268|  %60 = load i64, ptr %59, , !!44509, !!8                                                                               ;L160<323<620
 37269|     ;; target_id = i64 %60
 37270|     ;; self = ptr %23
 37271|  %61 = gep %23, i64 1224                                                                                               ;L742<654
 37272|  %62 = gep %23, i64 1272                                                                                               ;L742<654
 37273|  %63 = load i32, ptr %62, , !!8                                                                                        ;L742<654
 37274|  %64 = icmp eq i32 %63, -1                                                                                             ;L742<654
 37275|  br i1 %64, label %265, label %248                                                                                     ;L742<654
 37276| 
 37277| 65: ; preds = %25
 37278|     ;; action = ptr %6
 37279|     ;; self = ptr %6
 37280|  %66 = gep %6, i64 8                                                                                                   ;L222<324<620
 37281|  %67 = load i64, ptr %66, , !!44509, !!8                                                                               ;L222<324<620
 37282|     ;; target_id = i64 %67
 37283|  %68 = gep %23, i64 1480                                                                                               ;L1693<696
 37284|  %69 = load i64, ptr %68, , !!8                                                                                        ;L1693<696
 37285|  %70 = icmp ugt i64 %69, 2                                                                                             ;L1693<696
 37286|  %71 = gep %23, i64 1280                                                                                               ;L1693<696
 37287|  %72 = select i1 %70, ptr %71, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.19                                           ;L1693<696
 37288|     ;; self = ptr %72
 37289|  %73 = gep %72, i64 48                                                                                                 ;L742<696
 37290|  %74 = load i32, ptr %73, , !!8                                                                                        ;L742<696
 37291|  %75 = icmp eq i32 %74, -1                                                                                             ;L742<696
 37292|  br i1 %75, label %363, label %346                                                                                     ;L742<696
 37293| 
 37294| 76: ; preds = %25
 37295|     ;; action = ptr %6
 37296|     ;; self = ptr %6
 37297|  %77 = gep %6, i64 8                                                                                                   ;L287<325<620
 37298|  %78 = load i64, ptr %77, , !!44509, !!8                                                                               ;L287<325<620
 37299|     ;; target_id = i64 %78
 37300|  %79 = gep %23, i64 1480                                                                                               ;L1701<738
 37301|  %80 = load i64, ptr %79, , !!8                                                                                        ;L1701<738
 37302|  %81 = icmp ugt i64 %80, 4                                                                                             ;L1701<738
 37303|  %82 = gep %23, i64 1336                                                                                               ;L1701<738
 37304|  %83 = select i1 %81, ptr %82, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.19                                           ;L1701<738
 37305|     ;; self = ptr %83
 37306|  %84 = gep %83, i64 48                                                                                                 ;L742<738
 37307|  %85 = load i32, ptr %84, , !!8                                                                                        ;L742<738
 37308|  %86 = icmp eq i32 %85, -1                                                                                             ;L742<738
 37309|  br i1 %86, label %467, label %450                                                                                     ;L742<738
 37310| 
 37311| 87: ; preds = %35
 37312|  %88 = add i64 %36, %37                                                                                                ;L804
 37313|  %89 = sub i64 %26, %88                                                                                                ;L804
 37314|  br label %624                                                                                                         ;L801
 37315| 
 37316| 90: ; preds = %35
 37317|  %91 = add i64 %26, -6                                                                                                 ;L802
 37318|  %92 = add i64 %36, %37                                                                                                ;L802
 37319|  %93 = sub i64 %91, %92                                                                                                ;L802
 37320|  br label %624                                                                                                         ;L801
 37321| 
 37322| 94: ; preds = %145, %143, %139, %115, %107, %101, %42
 37324|     ;; other = ptr %0
 37325|  %95 = load i64, ptr %0, , !!8                                                                                         ;L2439<819
 37326|  %96 = gep %0, i64 8                                                                                                   ;L2439<819
 37327|  %97 = trunc nuw i64 %95 to i1                                                                                         ;L2439<819
 37328|  %98 = load i64, ptr %96,
 37329|  %99 = icmp eq i64 %44, %98
 37330|  %100 = select i1 %97, i1 %99, i1 false                                                                                ;L2439<819
 37331|  br i1 %100, label %151, label %624                                                                                    ;L2439<819
 37332| 
 37333| 101: ; preds = %42
 37334|  %102 = gep %6, i64 144                                                                                                ;L808
 37335|  %103 = load i8, ptr %102,                                                                                             ;L808
 37337|  %104 = icmp eq i8 %28, 14                                                                                             ;L285<808
 37338|  %105 = trunc nuw i8 %103 to i1                                                                                        ;L285<808
 37339|  %106 = select i1 %104, i1 %105, i1 false                                                                              ;L285<808
 37340|  br i1 %106, label %107, label %94                                                                                     ;L808
 37341| 
 37342| 107: ; preds = %101
 37343|  %108 = load ptr, ptr %19, , !!8, !!8                                                                                  ;L809
 37344|  %109 = gep %19, i64 8                                                                                                 ;L809
 37345|  %110 = load ptr, ptr %109, , !!8, !!8                                                                                 ;L809
 37346|  %111 = gep %110, i64 496                                                                                              ;L809
 37347|  %112 = load ptr, ptr %111, , !!8                                                                                      ;L809
 37348|  %113 = tail call ptr %112(ptr %108, i64 %44)                                                                          ;L809
 37349|  %114 = icmp eq ptr %113, null                                                                                         ;L809
 37350|  br i1 %114, label %94, label %115                                                                                     ;L809
 37351| 
 37352| 115: ; preds = %107
 37353|     ;; target = ptr %113
 37354|     ;; self = ptr %113
 37355|  %116 = tail call i64 @ai::plan_legacy3old6battle16max_range_cached(ptr %5, ptr %23, ptr %113)                         ;L810
 37356|  %117 = add i64 %116, 25000                                                                                            ;L810
 37357|     ;; mr = i64 %117
 37358|  %118 = gep %113, i64 1632                                                                                             ;L2158<811
 37359|  %119 = load i64, ptr %118, , !!8                                                                                      ;L2158<811
 37360|     ;; x1 = i64 %119
 37361|     ;; self = i64 %119
 37362|  %120 = gep %113, i64 1640                                                                                             ;L2158<811
 37363|  %121 = load i64, ptr %120, , !!8                                                                                      ;L2158<811
 37364|     ;; y1 = i64 %121
 37365|     ;; self = i64 %121
 37366|  %122 = gep %23, i64 1632                                                                                              ;L2158<811
 37367|  %123 = load i64, ptr %122, , !!8                                                                                      ;L2158<811
 37368|     ;; x2 = i64 %123
 37369|     ;; other = i64 %123
 37370|  %124 = gep %23, i64 1640                                                                                              ;L2158<811
 37371|  %125 = load i64, ptr %124, , !!8                                                                                      ;L2158<811
 37372|     ;; y2 = i64 %125
 37373|     ;; other = i64 %125
 37374|  %126 = icmp ult i64 %119, %123                                                                                        ;L3147<7<2158<811
 37375|  %127 = sub nuw i64 %123, %119                                                                                         ;L3147<7<2158<811
 37376|  %128 = sub nuw i64 %119, %123                                                                                         ;L3147<7<2158<811
 37377|  %129 = select i1 %126, i64 %127, i64 %128                                                                             ;L3147<7<2158<811
 37378|     ;; dx = i64 %129
 37379|  %130 = icmp ult i64 %121, %125                                                                                        ;L3147<8<2158<811
 37380|  %131 = sub nuw i64 %125, %121                                                                                         ;L3147<8<2158<811
 37381|  %132 = sub nuw i64 %121, %125                                                                                         ;L3147<8<2158<811
 37382|  %133 = select i1 %130, i64 %131, i64 %132                                                                             ;L3147<8<2158<811
 37383|     ;; dy = i64 %133
 37384|  %134 = mul i64 %129, %129                                                                                             ;L9<2158<811
 37385|  %135 = mul i64 %133, %133                                                                                             ;L9<2158<811
 37386|  %136 = add i64 %135, %134                                                                                             ;L9<2158<811
 37387|  %137 = mul i64 %117, %117                                                                                             ;L811
 37388|  %138 = icmp ugt i64 %136, %137                                                                                        ;L811
 37389|  br i1 %138, label %139, label %94                                                                                     ;L811
 37390| 
 37391| 139: ; preds = %115
 37392|  %140 = gep %5, i64 8                                                                                                  ;L812
 37393|  %141 = load ptr, ptr %140, , !!8, !!8                                                                                 ;L812
 37394|  %142 = tail call zeroext i1 @ai::tower_discipline17can_tower_focused(ptr %141, ptr %19, ptr %4, i64 %123, i64 %125)   ;L812
 37395|  br i1 %142, label %94, label %143                                                                                     ;L812
 37396| 
 37397| 143: ; preds = %139
 37398|  %144 = tail call zeroext i1 @ai::tower_discipline17can_tower_focused(ptr %141, ptr %19, ptr %4, i64 %119, i64 %121)   ;L813
 37399|  br i1 %144, label %145, label %94                                                                                     ;L813
 37400| 
 37401| 145: ; preds = %143
 37402|  %146 = gep %4, i64 2344                                                                                               ;L814
 37403|  %147 = load i64, ptr %146, , !!8                                                                                      ;L814
 37404|  %148 = tail call zeroext i1 @ai::tower_discipline23can_trace_without_tower(ptr %141, ptr %19, i64 %147, i64 %119, i64 %121, i64 %117) ;L814
 37405|  br i1 %148, label %149, label %94                                                                                     ;L814
 37406| 
 37407| 149: ; preds = %145
 37408|  %150 = add i64 %26, -30                                                                                               ;L815
 37409|  br label %624                                                                                                         ;L1
 37410| 
 37411| 151: ; preds = %94
 37412|  %152 = load ptr, ptr %19, , !!8, !!8                                                                                  ;L821
 37413|  %153 = gep %19, i64 8                                                                                                 ;L821
 37414|  %154 = load ptr, ptr %153, , !!8, !!8                                                                                 ;L821
 37415|  %155 = gep %154, i64 496                                                                                              ;L821
 37416|  %156 = load ptr, ptr %155, , !!8                                                                                      ;L821
 37417|  %157 = tail call ptr %156(ptr %152, i64 %44)                                                                          ;L821
 37418|  %158 = icmp eq ptr %157, null                                                                                         ;L821
 37419|  br i1 %158, label %624, label %159                                                                                    ;L821
 37420| 
 37421| 159: ; preds = %151
 37422|     ;; target = ptr %157
 37423|     ;; self = ptr %157
 37424|  %160 = tail call i64 @ai::plan_legacy3old6battle16max_range_cached(ptr %5, ptr %23, ptr %157)                         ;L822
 37425|  %161 = add i64 %160, 25000                                                                                            ;L822
 37426|     ;; mr = i64 %161
 37427|  %162 = gep %157, i64 1632                                                                                             ;L2158<823
 37428|  %163 = load i64, ptr %162, , !!8                                                                                      ;L2158<823
 37429|     ;; x1 = i64 %163
 37430|     ;; self = i64 %163
 37431|  %164 = gep %157, i64 1640                                                                                             ;L2158<823
 37432|  %165 = load i64, ptr %164, , !!8                                                                                      ;L2158<823
 37433|     ;; y1 = i64 %165
 37434|     ;; self = i64 %165
 37435|  %166 = gep %23, i64 1632                                                                                              ;L2158<823
 37436|  %167 = load i64, ptr %166, , !!8                                                                                      ;L2158<823
 37437|     ;; x2 = i64 %167
 37438|     ;; other = i64 %167
 37439|  %168 = gep %23, i64 1640                                                                                              ;L2158<823
 37440|  %169 = load i64, ptr %168, , !!8                                                                                      ;L2158<823
 37441|     ;; y2 = i64 %169
 37442|     ;; other = i64 %169
 37443|  %170 = icmp ult i64 %163, %167                                                                                        ;L3147<7<2158<823
 37444|  %171 = sub nuw i64 %167, %163                                                                                         ;L3147<7<2158<823
 37445|  %172 = sub nuw i64 %163, %167                                                                                         ;L3147<7<2158<823
 37446|  %173 = select i1 %170, i64 %171, i64 %172                                                                             ;L3147<7<2158<823
 37447|     ;; dx = i64 %173
 37448|  %174 = icmp ult i64 %165, %169                                                                                        ;L3147<8<2158<823
 37449|  %175 = sub nuw i64 %169, %165                                                                                         ;L3147<8<2158<823
 37450|  %176 = sub nuw i64 %165, %169                                                                                         ;L3147<8<2158<823
 37451|  %177 = select i1 %174, i64 %175, i64 %176                                                                             ;L3147<8<2158<823
 37452|     ;; dy = i64 %177
 37453|  %178 = mul i64 %173, %173                                                                                             ;L9<2158<823
 37454|  %179 = mul i64 %177, %177                                                                                             ;L9<2158<823
 37455|  %180 = add i64 %179, %178                                                                                             ;L9<2158<823
 37456|  %181 = mul i64 %161, %161                                                                                             ;L823
 37457|  %182 = icmp ugt i64 %180, %181                                                                                        ;L823
 37458|  %183 = add i64 %26, 10
 37459|  %184 = select i1 %182, i64 %183, i64 %26                                                                              ;L823
 37460|  br label %624                                                                                                         ;L823
 37461| 
 37462| 185: ; preds = %48
 37463|     ;; t = ptr %56
 37464|     ;; self = ptr %23
 37465|  %186 = gep %23, i64 1168                                                                                              ;L742<623
 37466|  %187 = gep %23, i64 1216                                                                                              ;L742<623
 37467|  %188 = load i32, ptr %187, , !!8                                                                                      ;L742<623
 37468|  %189 = icmp eq i32 %188, -1                                                                                           ;L742<623
 37469|  br i1 %189, label %200, label %190                                                                                    ;L742<623
 37470| 
 37471| 190: ; preds = %185
 37472|     ;; self = ptr %186
 37473|     ;; attack_effect = ptr %186
 37474|  %191 = gep %23, i64 1392                                                                                              ;L1494<624
 37475|  %192 = tail call i64 @gc::simulation6entityNtB5_6Entity17attack_speed_mult(ptr %23)                                   ;L624
 37476|  %193 = tail call i64 @ai::action_score22calculate_action_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %191, ptr %186, i64 %192, ptr %56, i8 1, ptr %7) ;L624
 37477|  %194 = add i64 %193, %26                                                                                              ;L624
 37478|     ;; score = i64 %194
 37479|  %195 = gep %0, i64 16                                                                                                 ;L625
 37480|  %196 = tail call i64 @ai::plan_legacy8sub_plan13battle_common31v17_runaway_counterattack_bonus(i64 %1, ptr %4, ptr %5, ptr %2, ptr %195, ptr %23, ptr %56, ptr %186) ;L625
 37481|  %197 = add i64 %194, %196                                                                                             ;L625
 37482|     ;; score = i64 %197
 37483|     ;; self = ptr %56
 37484|  %198 = gep %56, i64 104                                                                                               ;L1400<626
 37485|  %199 = load i64, ptr %198, , !!8                                                                                      ;L1400<626
 37486|  switch i64 %199, label %624 [
 37487|  i64 3, label %201
 37488|  i64 5, label %203
 37489|  i64 6, label %203
 37490|  i64 2, label %215
 37491|  i64 4, label %223
 37492|  ]                                                                                                                     ;L626
 37493| 
 37494| 200: ; preds = %185
 37495|     ;; self = ptr null
 37496|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.178) #31                       ;L1013<623
 37497|  unreachable                                                                                                           ;L1013<623
 37498| 
 37499| 201: ; preds = %190
 37500|  %202 = add i64 %197, 200                                                                                              ;L627
 37501|  br label %624                                                                                                         ;L626
 37502| 
 37503| 203: ; preds = %190, %190
 37504|     ;; self = ptr %23
 37505|     ;; self = ptr %186
 37506|  %204 = gep %5, i64 8                                                                                                  ;L629
 37507|  %205 = load ptr, ptr %204, , !!8, !!8                                                                                 ;L629
 37508|  %206 = tail call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %186, ptr %205, ptr %23, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %56) ;L629
 37509|     ;; dmg = i64 %206
 37510|  %207 = gep %56, i64 1648                                                                                              ;L630
 37511|  %208 = load i64, ptr %207, , !!8                                                                                      ;L630
 37512|  %209 = icmp ult i64 %206, %208                                                                                        ;L630
 37513|  br i1 %209, label %210, label %213                                                                                    ;L630
 37514| 
 37515| 210: ; preds = %203
 37516|  %211 = sdiv i64 %197, 3                                                                                               ;L633
 37517|     ;; self = i64 %211
 37518|     ;; other = i64 1
 37519|  %212 = tail call i64 @llvm.smax.i64(i64 %211, i64 1)                                                                  ;L1039<633
 37520|  br label %624                                                                                                         ;L630
 37521| 
 37522| 213: ; preds = %203
 37523|  %214 = add i64 %197, 100                                                                                              ;L631
 37524|  br label %624                                                                                                         ;L630
 37525| 
 37526| 215: ; preds = %190
 37527|  %216 = icmp ugt i64 %1, 1                                                                                             ;L635
 37528|  br i1 %216, label %217, label %624                                                                                    ;L635
 37529| 
 37530| 217: ; preds = %215
 37531|  %218 = gep %5, i64 8                                                                                                  ;L636
 37532|  %219 = load ptr, ptr %218,                                                                                            ;L636
 37533|  %220 = tail call fastcc zeroext i1 @ai::plan_legacy8sub_plan6battle23v3_tower_burst_feasible(i64 %12, ptr %19, ptr %219, ptr %56) ;L636
 37534|  %221 = add i64 %197, 100
 37535|  %222 = select i1 %220, i64 %221, i64 %197                                                                             ;L636
 37536|  br label %624                                                                                                         ;L636
 37537| 
 37538| 223: ; preds = %190
 37539|  %224 = sub nuw nsw i64 1, %12                                                                                         ;L638
 37540|     ;; team = i64 %224
 37541|  %225 = gep %56, i64 152                                                                                               ;L1378<638
 37542|  %226 = load i64, ptr %225, , !!8                                                                                      ;L1378<638
 37543|  %227 = icmp eq i64 %226, %224                                                                                         ;L1378<638
 37544|  br i1 %227, label %232, label %228                                                                                    ;L1378<638
 37545| 
 37546| 228: ; preds = %223
 37547|     ;; self = ptr %56
 37548|  %229 = icmp ult i64 %226, 2                                                                                           ;L1378<643
 37549|  br i1 %229, label %230, label %624                                                                                    ;L1378<643
 37550| 
 37551| 230: ; preds = %228
 37552|     ;; self = i64 %197
 37553|     ;; other = i64 1
 37554|  %231 = tail call i64 @llvm.smax.i64(i64 %197, i64 1)                                                                  ;L1039<644
 37555|  br label %624                                                                                                         ;L643
 37556| 
 37557| 232: ; preds = %223
 37558|     ;; self = ptr %4
 37560|     ;; f[8..+8] = ptr %56
 37564|     ;; c = ptr %23
 37566|     ;; self = ptr %23
 37567|  %233 = load i32, ptr %187, , !!44786, !!8                                                                             ;L742<640<1543<640
 37568|  %234 = icmp eq i32 %233, -1                                                                                           ;L742<640<1543<640
 37569|  br i1 %234, label %239, label %235                                                                                    ;L742<640<1543<640
 37570| 
 37571| 235: ; preds = %232
 37572|  %236 = gep %5, i64 8                                                                                                  ;L640
 37573|  %237 = load ptr, ptr %236, , !!8, !!8                                                                                 ;L640
 37574|     ;; f[0..+8] = ptr %237
 37575|     ;; self = ptr %186
 37576|     ;; f[0..+8] = ptr %237
 37577|     ;; f[8..+8] = ptr %23
 37578|     ;; f[16..+8] = ptr %56
 37579|     ;; x = ptr %186
 37580|     ;; a = ptr %186
 37583|  %238 = tail call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %186, ptr %237, ptr %23, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %56) ;L640<1162<640<1543<640
 37584|  br label %239                                                                                                         ;L1162<640<1543<640
 37585| 
 37586| 239: ; preds = %235, %232
 37587|  %240 = phi i64 [ %238, %235 ], [ 0, %232 ]                                                                            ;L0<640<1543<640
 37590|     ;; dmg_est = i64 %240
 37591|  %241 = gep %56, i64 1648                                                                                              ;L642
 37592|  %242 = load i64, ptr %241, , !!8                                                                                      ;L642
 37593|  %243 = icmp ult i64 %240, %242                                                                                        ;L642
 37594|  br i1 %243, label %244, label %246                                                                                    ;L642
 37595| 
 37596| 244: ; preds = %239
 37597|     ;; self = i64 %197
 37598|     ;; other = i64 1
 37599|  %245 = tail call i64 @llvm.smax.i64(i64 %197, i64 1)                                                                  ;L1039<642
 37600|  br label %624                                                                                                         ;L642
 37601| 
 37602| 246: ; preds = %239
 37603|  %247 = add i64 %197, 80                                                                                               ;L642
 37604|  br label %624                                                                                                         ;L642
 37605| 
 37606| 248: ; preds = %58
 37607|     ;; self = ptr %61
 37608|     ;; self = ptr %61
 37609|     ;; self = ptr %61
 37610|  %249 = load ptr, ptr %61, , !!8, !!8                                                                                  ;L441<2127<2445<654
 37611|  %250 = gep %23, i64 1232                                                                                              ;L441<2127<2445<654
 37612|  %251 = load ptr, ptr %250, , !!8, !!8                                                                                 ;L441<2127<2445<654
 37613|  %252 = gep %251, i64 16                                                                                               ;L2445<654
 37614|  %253 = load i64, ptr %252,                                                                                            ;L2445<654
 37615|  %254 = add nsw i64 %253, -1                                                                                           ;L2445<654
 37616|  %255 = and i64 %254, -16                                                                                              ;L2445<654
 37617|  %256 = gep %249, i64 %255                                                                                             ;L2445<654
 37618|  %257 = gep %256, i64 16                                                                                               ;L2445<654
 37619|  %258 = gep %251, i64 96                                                                                               ;L654
 37620|  %259 = load ptr, ptr %258, , !!8                                                                                      ;L654
 37621|  %260 = tail call zeroext i1 %259(ptr %257)                                                                            ;L654
 37622|  %261 = gep %0, i64 16                                                                                                 ;L654
 37623|  %262 = load i64, ptr %261,                                                                                            ;L654
 37626|     ;; __self_discr = i64 %262
 37627|     ;; __arg1_discr = i64 4
 37628|  %263 = icmp eq i64 %262, 4
 37629|  %264 = select i1 %260, i1 %263, i1 false                                                                              ;L654
 37630|  br i1 %264, label %624, label %266                                                                                    ;L654
 37631| 
 37632| 265: ; preds = %58
 37633|     ;; self = ptr null
 37634|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.179) #31                       ;L1013<654
 37635|  unreachable                                                                                                           ;L1013<654
 37636| 
 37637| 266: ; preds = %248
 37638|  %267 = load ptr, ptr %19, , !!8, !!8                                                                                  ;L658
 37639|  %268 = gep %19, i64 8                                                                                                 ;L658
 37640|  %269 = load ptr, ptr %268, , !!8, !!8                                                                                 ;L658
 37641|  %270 = gep %269, i64 496                                                                                              ;L658
 37642|  %271 = load ptr, ptr %270, , !!8                                                                                      ;L658
 37643|  %272 = tail call ptr %271(ptr %267, i64 %60)                                                                          ;L658
 37644|  %273 = icmp eq ptr %272, null                                                                                         ;L658
 37645|  br i1 %273, label %624, label %274                                                                                    ;L658
 37646| 
 37647| 274: ; preds = %266
 37648|     ;; t = ptr %272
 37649|     ;; self = ptr %23
 37650|     ;; self = ptr %61
 37651|     ;; skill_effect = ptr %61
 37652|  %275 = gep %23, i64 1408                                                                                              ;L1665<660
 37653|  %276 = tail call i64 @gc::simulation6entityNtB5_6Entity15cooldown_reduce(ptr %23, i1 zeroext false)                   ;L660
 37654|  %277 = tail call i64 @ai::action_score22calculate_action_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %275, ptr %61, i64 %276, ptr %272, i8 1, ptr %7) ;L660
 37655|  %278 = add i64 %277, %26                                                                                              ;L660
 37656|     ;; score = i64 %278
 37659|     ;; __self_discr = i64 %262
 37660|     ;; __arg1_discr = i64 4
 37661|  br i1 %263, label %279, label %283                                                                                    ;L9<662
 37662| 
 37663| 279: ; preds = %274
 37664|  %280 = tail call { i64, i64 } @ai::fight_check14effect_cc_time(i64 %1, ptr %61)                                       ;L663
 37665|  %281 = extractvalue { i64, i64 } %280, 0                                                                              ;L663
 37666|  %282 = trunc nuw i64 %281 to i1                                                                                       ;L663
 37667|  br i1 %282, label %294, label %283                                                                                    ;L663
 37668| 
 37669| 283: ; preds = %294, %279, %274
 37670|  %284 = phi i64 [ %278, %279 ], [ %304, %294 ], [ %278, %274 ]                                                         ;L0
 37671|     ;; score = i64 %284
 37672|  %285 = tail call { i64, i64 } @ai::fight_check14effect_cc_time(i64 %1, ptr %61)                                       ;L669
 37673|  %286 = extractvalue { i64, i64 } %285, 0                                                                              ;L669
 37674|  %287 = extractvalue { i64, i64 } %285, 1                                                                              ;L669
 37675|  %288 = tail call i64 @ai::plan_legacy8sub_plan13battle_common30v21_runaway_defensive_cc_bonus(i64 %1, ptr %4, ptr %5, ptr %2, ptr %261, ptr %23, ptr %272, ptr %61, i64 %286, i64 %287) ;L669
 37676|  %289 = add i64 %288, %284                                                                                             ;L669
 37677|     ;; score = i64 %289
 37678|  %290 = tail call i64 @ai::plan_legacy8sub_plan13battle_common31v17_runaway_counterattack_bonus(i64 %1, ptr %4, ptr %5, ptr %2, ptr %261, ptr %23, ptr %272, ptr %61) ;L670
 37679|  %291 = add i64 %289, %290                                                                                             ;L670
 37680|     ;; score = i64 %291
 37681|     ;; self = ptr %272
 37682|  %292 = gep %272, i64 104                                                                                              ;L1400<671
 37683|  %293 = load i64, ptr %292, , !!8                                                                                      ;L1400<671
 37684|  switch i64 %293, label %624 [
 37685|  i64 3, label %305
 37686|  i64 5, label %307
 37687|  i64 6, label %307
 37688|  i64 4, label %319
 37689|  ]                                                                                                                     ;L671
 37690| 
 37691| 294: ; preds = %279
 37692|  %295 = extractvalue { i64, i64 } %280, 1                                                                              ;L663
 37693|     ;; cc_time = i64 %295
 37694|  %296 = tail call i64 @gc::simulation6entityNtB5_6Entity8distance(ptr %272, ptr %23)                                   ;L664
 37695|     ;; dist = i64 %296
 37696|     ;; rhs = i64 %296
 37697|  %297 = tail call i64 @llvm.usub.sat.i64(i64 80000, i64 %296)                                                          ;L2472<665
 37698|  %298 = trunc nuw nsw i64 %297 to i32                                                                                  ;L665
 37699|  %299 = udiv i32 %298, 10000                                                                                           ;L665
 37700|  %300 = zext nneg i32 %299 to i64                                                                                      ;L665
 37701|     ;; proximity_bonus = i64 %300
 37702|  %301 = mul i64 %295, 3                                                                                                ;L666
 37703|  %302 = add i64 %301, 3                                                                                                ;L666
 37704|  %303 = mul i64 %302, %300                                                                                             ;L666
 37705|  %304 = add i64 %303, %278                                                                                             ;L666
 37706|     ;; score = i64 %304
 37707|  br label %283                                                                                                         ;L663
 37708| 
 37709| 305: ; preds = %283
 37710|  %306 = add i64 %291, 200                                                                                              ;L672
 37711|  br label %624                                                                                                         ;L671
 37712| 
 37713| 307: ; preds = %283, %283
 37714|  %308 = gep %5, i64 8                                                                                                  ;L674
 37715|  %309 = load ptr, ptr %308, , !!8, !!8                                                                                 ;L674
 37716|  %310 = tail call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %61, ptr %309, ptr %23, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %272) ;L674
 37717|     ;; dmg = i64 %310
 37718|  %311 = gep %272, i64 1648                                                                                             ;L675
 37719|  %312 = load i64, ptr %311, , !!8                                                                                      ;L675
 37720|  %313 = icmp ult i64 %310, %312                                                                                        ;L675
 37721|  br i1 %313, label %314, label %317                                                                                    ;L675
 37722| 
 37723| 314: ; preds = %307
 37724|  %315 = sdiv i64 %291, 3                                                                                               ;L678
 37725|     ;; self = i64 %315
 37726|     ;; other = i64 1
 37727|  %316 = tail call i64 @llvm.smax.i64(i64 %315, i64 1)                                                                  ;L1039<678
 37728|  br label %624                                                                                                         ;L675
 37729| 
 37730| 317: ; preds = %307
 37731|  %318 = add i64 %291, 100                                                                                              ;L676
 37732|  br label %624                                                                                                         ;L675
 37733| 
 37734| 319: ; preds = %283
 37735|  %320 = sub nuw nsw i64 1, %12                                                                                         ;L680
 37736|     ;; team = i64 %320
 37737|  %321 = gep %272, i64 152                                                                                              ;L1378<680
 37738|  %322 = load i64, ptr %321, , !!8                                                                                      ;L1378<680
 37739|  %323 = icmp eq i64 %322, %320                                                                                         ;L1378<680
 37740|  br i1 %323, label %328, label %324                                                                                    ;L1378<680
 37741| 
 37742| 324: ; preds = %319
 37743|     ;; self = ptr %272
 37744|  %325 = icmp ult i64 %322, 2                                                                                           ;L1378<685
 37745|  br i1 %325, label %326, label %624                                                                                    ;L1378<685
 37746| 
 37747| 326: ; preds = %324
 37748|     ;; self = i64 %291
 37749|     ;; other = i64 1
 37750|  %327 = tail call i64 @llvm.smax.i64(i64 %291, i64 1)                                                                  ;L1039<686
 37751|  br label %624                                                                                                         ;L685
 37752| 
 37753| 328: ; preds = %319
 37754|     ;; self = ptr %4
 37756|     ;; f[8..+8] = ptr %272
 37760|     ;; c = ptr %23
 37762|     ;; self = ptr %23
 37763|  %329 = gep %23, i64 1216                                                                                              ;L742<682<1543<682
 37764|  %330 = load i32, ptr %329, , !!44937, !!8                                                                             ;L742<682<1543<682
 37765|  %331 = icmp eq i32 %330, -1                                                                                           ;L742<682<1543<682
 37766|  br i1 %331, label %337, label %332                                                                                    ;L742<682<1543<682
 37767| 
 37768| 332: ; preds = %328
 37769|  %333 = gep %5, i64 8                                                                                                  ;L682
 37770|  %334 = load ptr, ptr %333, , !!8, !!8                                                                                 ;L682
 37771|     ;; f[0..+8] = ptr %334
 37772|  %335 = gep %23, i64 1168                                                                                              ;L742<682<1543<682
 37773|     ;; self = ptr %335
 37774|     ;; f[0..+8] = ptr %334
 37775|     ;; f[8..+8] = ptr %23
 37776|     ;; f[16..+8] = ptr %272
 37777|     ;; x = ptr %335
 37778|     ;; a = ptr %335
 37781|  %336 = tail call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %335, ptr %334, ptr %23, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %272) ;L682<1162<682<1543<682
 37782|  br label %337                                                                                                         ;L1162<682<1543<682
 37783| 
 37784| 337: ; preds = %332, %328
 37785|  %338 = phi i64 [ %336, %332 ], [ 0, %328 ]                                                                            ;L0<682<1543<682
 37788|     ;; dmg_est = i64 %338
 37789|  %339 = gep %272, i64 1648                                                                                             ;L684
 37790|  %340 = load i64, ptr %339, , !!8                                                                                      ;L684
 37791|  %341 = icmp ult i64 %338, %340                                                                                        ;L684
 37792|  br i1 %341, label %342, label %344                                                                                    ;L684
 37793| 
 37794| 342: ; preds = %337
 37795|     ;; self = i64 %291
 37796|     ;; other = i64 1
 37797|  %343 = tail call i64 @llvm.smax.i64(i64 %291, i64 1)                                                                  ;L1039<684
 37798|  br label %624                                                                                                         ;L684
 37799| 
 37800| 344: ; preds = %337
 37801|  %345 = add i64 %291, 80                                                                                               ;L684
 37802|  br label %624                                                                                                         ;L684
 37803| 
 37804| 346: ; preds = %65
 37805|     ;; self = ptr %72
 37806|     ;; self = ptr %72
 37807|     ;; self = ptr %72
 37808|  %347 = load ptr, ptr %72, , !!8, !!8                                                                                  ;L441<2127<2445<696
 37809|  %348 = gep %72, i64 8                                                                                                 ;L441<2127<2445<696
 37810|  %349 = load ptr, ptr %348, , !!8, !!8                                                                                 ;L441<2127<2445<696
 37811|  %350 = gep %349, i64 16                                                                                               ;L2445<696
 37812|  %351 = load i64, ptr %350,                                                                                            ;L2445<696
 37813|  %352 = add nsw i64 %351, -1                                                                                           ;L2445<696
 37814|  %353 = and i64 %352, -16                                                                                              ;L2445<696
 37815|  %354 = gep %347, i64 %353                                                                                             ;L2445<696
 37816|  %355 = gep %354, i64 16                                                                                               ;L2445<696
 37817|  %356 = gep %349, i64 96                                                                                               ;L696
 37818|  %357 = load ptr, ptr %356, , !!8                                                                                      ;L696
 37819|  %358 = tail call zeroext i1 %357(ptr %355)                                                                            ;L696
 37820|  %359 = gep %0, i64 16                                                                                                 ;L696
 37821|  %360 = load i64, ptr %359,                                                                                            ;L696
 37824|     ;; __self_discr = i64 %360
 37825|     ;; __arg1_discr = i64 4
 37826|  %361 = icmp eq i64 %360, 4
 37827|  %362 = select i1 %358, i1 %361, i1 false                                                                              ;L696
 37828|  br i1 %362, label %624, label %364                                                                                    ;L696
 37829| 
 37830| 363: ; preds = %65
 37831|     ;; self = ptr null
 37832|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.180) #31                       ;L1013<696
 37833|  unreachable                                                                                                           ;L1013<696
 37834| 
 37835| 364: ; preds = %346
 37836|  %365 = load ptr, ptr %19, , !!8, !!8                                                                                  ;L700
 37837|  %366 = gep %19, i64 8                                                                                                 ;L700
 37838|  %367 = load ptr, ptr %366, , !!8, !!8                                                                                 ;L700
 37839|  %368 = gep %367, i64 496                                                                                              ;L700
 37840|  %369 = load ptr, ptr %368, , !!8                                                                                      ;L700
 37841|  %370 = tail call ptr %369(ptr %365, i64 %67)                                                                          ;L700
 37842|  %371 = icmp eq ptr %370, null                                                                                         ;L700
 37843|  br i1 %371, label %624, label %372                                                                                    ;L700
 37844| 
 37845| 372: ; preds = %364
 37846|     ;; t = ptr %370
 37847|  br i1 %70, label %373, label %377                                                                                     ;L1693<701
 37848| 
 37849| 373: ; preds = %372
 37850|     ;; self = ptr %71
 37851|  %374 = gep %23, i64 1328                                                                                              ;L742<701
 37852|  %375 = load i32, ptr %374, , !!8                                                                                      ;L742<701
 37853|  %376 = icmp eq i32 %375, -1                                                                                           ;L742<701
 37854|  br i1 %376, label %377, label %378                                                                                    ;L742<701
 37855| 
 37856| 377: ; preds = %373, %372
 37857|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.181) #31                       ;L1013<701
 37858|  unreachable                                                                                                           ;L1013<701
 37859| 
 37860| 378: ; preds = %373
 37861|     ;; self = ptr %71
 37862|     ;; skill2_effect = ptr %71
 37863|  %379 = gep %23, i64 1424                                                                                              ;L1670<702
 37864|  %380 = tail call i64 @gc::simulation6entityNtB5_6Entity15cooldown_reduce(ptr %23, i1 zeroext false)                   ;L702
 37865|  %381 = tail call i64 @ai::action_score22calculate_action_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %379, ptr %71, i64 %380, ptr %370, i8 1, ptr %7) ;L702
 37866|  %382 = add i64 %381, %26                                                                                              ;L702
 37867|     ;; score = i64 %382
 37870|     ;; __self_discr = i64 %360
 37871|     ;; __arg1_discr = i64 4
 37872|  br i1 %361, label %383, label %387                                                                                    ;L9<704
 37873| 
 37874| 383: ; preds = %378
 37875|  %384 = tail call { i64, i64 } @ai::fight_check14effect_cc_time(i64 %1, ptr %71)                                       ;L705
 37876|  %385 = extractvalue { i64, i64 } %384, 0                                                                              ;L705
 37877|  %386 = trunc nuw i64 %385 to i1                                                                                       ;L705
 37878|  br i1 %386, label %398, label %387                                                                                    ;L705
 37879| 
 37880| 387: ; preds = %398, %383, %378
 37881|  %388 = phi i64 [ %382, %383 ], [ %408, %398 ], [ %382, %378 ]                                                         ;L0
 37882|     ;; score = i64 %388
 37883|  %389 = tail call { i64, i64 } @ai::fight_check14effect_cc_time(i64 %1, ptr %71)                                       ;L711
 37884|  %390 = extractvalue { i64, i64 } %389, 0                                                                              ;L711
 37885|  %391 = extractvalue { i64, i64 } %389, 1                                                                              ;L711
 37886|  %392 = tail call i64 @ai::plan_legacy8sub_plan13battle_common30v21_runaway_defensive_cc_bonus(i64 %1, ptr %4, ptr %5, ptr %2, ptr %359, ptr %23, ptr %370, ptr %71, i64 %390, i64 %391) ;L711
 37887|  %393 = add i64 %392, %388                                                                                             ;L711
 37888|     ;; score = i64 %393
 37889|  %394 = tail call i64 @ai::plan_legacy8sub_plan13battle_common31v17_runaway_counterattack_bonus(i64 %1, ptr %4, ptr %5, ptr %2, ptr %359, ptr %23, ptr %370, ptr %71) ;L712
 37890|  %395 = add i64 %393, %394                                                                                             ;L712
 37891|     ;; score = i64 %395
 37892|     ;; self = ptr %370
 37893|  %396 = gep %370, i64 104                                                                                              ;L1400<713
 37894|  %397 = load i64, ptr %396, , !!8                                                                                      ;L1400<713
 37895|  switch i64 %397, label %624 [
 37896|  i64 3, label %409
 37897|  i64 5, label %411
 37898|  i64 6, label %411
 37899|  i64 4, label %423
 37900|  ]                                                                                                                     ;L713
 37901| 
 37902| 398: ; preds = %383
 37903|  %399 = extractvalue { i64, i64 } %384, 1                                                                              ;L705
 37904|     ;; cc_time = i64 %399
 37905|  %400 = tail call i64 @gc::simulation6entityNtB5_6Entity8distance(ptr %370, ptr %23)                                   ;L706
 37906|     ;; dist = i64 %400
 37907|     ;; rhs = i64 %400
 37908|  %401 = tail call i64 @llvm.usub.sat.i64(i64 80000, i64 %400)                                                          ;L2472<707
 37909|  %402 = trunc nuw nsw i64 %401 to i32                                                                                  ;L707
 37910|  %403 = udiv i32 %402, 10000                                                                                           ;L707
 37911|  %404 = zext nneg i32 %403 to i64                                                                                      ;L707
 37912|     ;; proximity_bonus = i64 %404
 37913|  %405 = mul i64 %399, 3                                                                                                ;L708
 37914|  %406 = add i64 %405, 3                                                                                                ;L708
 37915|  %407 = mul i64 %406, %404                                                                                             ;L708
 37916|  %408 = add i64 %407, %382                                                                                             ;L708
 37917|     ;; score = i64 %408
 37918|  br label %387                                                                                                         ;L705
 37919| 
 37920| 409: ; preds = %387
 37921|  %410 = add i64 %395, 200                                                                                              ;L714
 37922|  br label %624                                                                                                         ;L713
 37923| 
 37924| 411: ; preds = %387, %387
 37925|  %412 = gep %5, i64 8                                                                                                  ;L716
 37926|  %413 = load ptr, ptr %412, , !!8, !!8                                                                                 ;L716
 37927|  %414 = tail call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %71, ptr %413, ptr %23, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %370) ;L716
 37928|     ;; dmg = i64 %414
 37929|  %415 = gep %370, i64 1648                                                                                             ;L717
 37930|  %416 = load i64, ptr %415, , !!8                                                                                      ;L717
 37931|  %417 = icmp ult i64 %414, %416                                                                                        ;L717
 37932|  br i1 %417, label %418, label %421                                                                                    ;L717
 37933| 
 37934| 418: ; preds = %411
 37935|  %419 = sdiv i64 %395, 3                                                                                               ;L720
 37936|     ;; self = i64 %419
 37937|     ;; other = i64 1
 37938|  %420 = tail call i64 @llvm.smax.i64(i64 %419, i64 1)                                                                  ;L1039<720
 37939|  br label %624                                                                                                         ;L717
 37940| 
 37941| 421: ; preds = %411
 37942|  %422 = add i64 %395, 100                                                                                              ;L718
 37943|  br label %624                                                                                                         ;L717
 37944| 
 37945| 423: ; preds = %387
 37946|  %424 = sub nuw nsw i64 1, %12                                                                                         ;L722
 37947|     ;; team = i64 %424
 37948|  %425 = gep %370, i64 152                                                                                              ;L1378<722
 37949|  %426 = load i64, ptr %425, , !!8                                                                                      ;L1378<722
 37950|  %427 = icmp eq i64 %426, %424                                                                                         ;L1378<722
 37951|  br i1 %427, label %432, label %428                                                                                    ;L1378<722
 37952| 
 37953| 428: ; preds = %423
 37954|     ;; self = ptr %370
 37955|  %429 = icmp ult i64 %426, 2                                                                                           ;L1378<727
 37956|  br i1 %429, label %430, label %624                                                                                    ;L1378<727
 37957| 
 37958| 430: ; preds = %428
 37959|     ;; self = i64 %395
 37960|     ;; other = i64 1
 37961|  %431 = tail call i64 @llvm.smax.i64(i64 %395, i64 1)                                                                  ;L1039<728
 37962|  br label %624                                                                                                         ;L727
 37963| 
 37964| 432: ; preds = %423
 37965|     ;; self = ptr %4
 37967|     ;; f[8..+8] = ptr %370
 37971|     ;; c = ptr %23
 37973|     ;; self = ptr %23
 37974|  %433 = gep %23, i64 1216                                                                                              ;L742<724<1543<724
 37975|  %434 = load i32, ptr %433, , !!45057, !!8                                                                             ;L742<724<1543<724
 37976|  %435 = icmp eq i32 %434, -1                                                                                           ;L742<724<1543<724
 37977|  br i1 %435, label %441, label %436                                                                                    ;L742<724<1543<724
 37978| 
 37979| 436: ; preds = %432
 37980|  %437 = gep %5, i64 8                                                                                                  ;L724
 37981|  %438 = load ptr, ptr %437, , !!8, !!8                                                                                 ;L724
 37982|     ;; f[0..+8] = ptr %438
 37983|  %439 = gep %23, i64 1168                                                                                              ;L742<724<1543<724
 37984|     ;; self = ptr %439
 37985|     ;; f[0..+8] = ptr %438
 37986|     ;; f[8..+8] = ptr %23
 37987|     ;; f[16..+8] = ptr %370
 37988|     ;; x = ptr %439
 37989|     ;; a = ptr %439
 37992|  %440 = tail call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %439, ptr %438, ptr %23, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %370) ;L724<1162<724<1543<724
 37993|  br label %441                                                                                                         ;L1162<724<1543<724
 37994| 
 37995| 441: ; preds = %436, %432
 37996|  %442 = phi i64 [ %440, %436 ], [ 0, %432 ]                                                                            ;L0<724<1543<724
 37999|     ;; dmg_est = i64 %442
 38000|  %443 = gep %370, i64 1648                                                                                             ;L726
 38001|  %444 = load i64, ptr %443, , !!8                                                                                      ;L726
 38002|  %445 = icmp ult i64 %442, %444                                                                                        ;L726
 38003|  br i1 %445, label %446, label %448                                                                                    ;L726
 38004| 
 38005| 446: ; preds = %441
 38006|     ;; self = i64 %395
 38007|     ;; other = i64 1
 38008|  %447 = tail call i64 @llvm.smax.i64(i64 %395, i64 1)                                                                  ;L1039<726
 38009|  br label %624                                                                                                         ;L726
 38010| 
 38011| 448: ; preds = %441
 38012|  %449 = add i64 %395, 80                                                                                               ;L726
 38013|  br label %624                                                                                                         ;L726
 38014| 
 38015| 450: ; preds = %76
 38016|     ;; self = ptr %83
 38017|     ;; self = ptr %83
 38018|     ;; self = ptr %83
 38019|  %451 = load ptr, ptr %83, , !!8, !!8                                                                                  ;L441<2127<2445<738
 38020|  %452 = gep %83, i64 8                                                                                                 ;L441<2127<2445<738
 38021|  %453 = load ptr, ptr %452, , !!8, !!8                                                                                 ;L441<2127<2445<738
 38022|  %454 = gep %453, i64 16                                                                                               ;L2445<738
 38023|  %455 = load i64, ptr %454,                                                                                            ;L2445<738
 38024|  %456 = add nsw i64 %455, -1                                                                                           ;L2445<738
 38025|  %457 = and i64 %456, -16                                                                                              ;L2445<738
 38026|  %458 = gep %451, i64 %457                                                                                             ;L2445<738
 38027|  %459 = gep %458, i64 16                                                                                               ;L2445<738
 38028|  %460 = gep %453, i64 96                                                                                               ;L738
 38029|  %461 = load ptr, ptr %460, , !!8                                                                                      ;L738
 38030|  %462 = tail call zeroext i1 %461(ptr %459)                                                                            ;L738
 38031|  %463 = gep %0, i64 16                                                                                                 ;L738
 38032|  %464 = load i64, ptr %463,                                                                                            ;L738
 38035|     ;; __self_discr = i64 %464
 38036|     ;; __arg1_discr = i64 4
 38037|  %465 = icmp eq i64 %464, 4
 38038|  %466 = select i1 %462, i1 %465, i1 false                                                                              ;L738
 38039|  br i1 %466, label %624, label %468                                                                                    ;L738
 38040| 
 38041| 467: ; preds = %76
 38042|     ;; self = ptr null
 38043|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.182) #31                       ;L1013<738
 38044|  unreachable                                                                                                           ;L1013<738
 38045| 
 38046| 468: ; preds = %450
 38047|  %469 = load ptr, ptr %19, , !!8, !!8                                                                                  ;L742
 38048|  %470 = gep %19, i64 8                                                                                                 ;L742
 38049|  %471 = load ptr, ptr %470, , !!8, !!8                                                                                 ;L742
 38050|  %472 = gep %471, i64 496                                                                                              ;L742
 38051|  %473 = load ptr, ptr %472, , !!8                                                                                      ;L742
 38052|  %474 = tail call ptr %473(ptr %469, i64 %78)                                                                          ;L742
 38053|  %475 = icmp eq ptr %474, null                                                                                         ;L742
 38054|  br i1 %475, label %624, label %476                                                                                    ;L742
 38055| 
 38056| 476: ; preds = %468
 38057|     ;; t = ptr %474
 38059|     ;; ult_action = ptr %479
 38060|     ;; ult_action = ptr %479
 38061|  %477 = select i1 %81, i64 1440, i64 1456                                                                              ;L1701<744
 38062|  %478 = select i1 %81, ptr %82, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.19                                          ;L1701<744
 38063|  %479 = gep %23, i64 %477                                                                                              ;L0<743
 38064|     ;; self = ptr %478
 38065|     ;; ult_action = ptr %479
 38066|  %480 = gep %478, i64 48                                                                                               ;L742<744
 38067|  %481 = load i32, ptr %480, , !!8                                                                                      ;L742<744
 38068|  %482 = icmp eq i32 %481, -1                                                                                           ;L742<744
 38069|  br i1 %482, label %497, label %483                                                                                    ;L742<744
 38070| 
 38071| 483: ; preds = %476
 38072|     ;; self = ptr %478
 38073|     ;; self = ptr %478
 38074|  %484 = gep %478, i64 16                                                                                               ;L13<744
 38075|  %485 = load i64, ptr %484, , !!8                                                                                      ;L13<744
 38076|  %486 = gep %478, i64 24                                                                                               ;L13<744
 38077|  %487 = load i64, ptr %486, , !!8                                                                                      ;L13<744
 38078|  %488 = gep %478, i64 32                                                                                               ;L13<744
 38079|  %489 = load i64, ptr %488, , !!8                                                                                      ;L13<744
 38080|     ;; self = ptr %478
 38081|  %490 = gep %478, i64 40                                                                                               ;L170<13<744
 38082|  %491 = load i32, ptr %490, , !!8                                                                                      ;L170<13<744
 38083|     ;; self = ptr %478
 38084|     ;; self = ptr %478
 38085|  %492 = load ptr, ptr %478, , !!8, !!8                                                                                 ;L441<2127<2411<13<744
 38086|  %493 = gep %478, i64 8                                                                                                ;L441<2127<2411<13<744
 38087|  %494 = load ptr, ptr %493, , !!8, !!8                                                                                 ;L441<2127<2411<13<744
 38088|     ;; self = ptr %492
 38089|     ;; dst = ptr %492
 38090|  %495 = atomicrmw add ptr %492, i64 1 monotonic,                                                                       ;L3937<3162<2411<13<744
 38091|     ;; old_size = i64 %495
 38092|  %496 = icmp slt i64 %495, 0                                                                                           ;L2428<13<744
 38093|  br i1 %496, label %509, label %498                                                                                    ;L2428<13<744
 38094| 
 38095| 497: ; preds = %476
 38096|     ;; self = ptr null
 38097|  tail call void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.183) #31                       ;L1013<744
 38098|  unreachable                                                                                                           ;L1013<744
 38099| 
 38100| 498: ; preds = %483
 38101|     ;; self = ptr %478
 38102|  %499 = gep %478, i64 44                                                                                               ;L483<13<744
 38103|  %500 = load i32, ptr %499, , !!8                                                                                      ;L483<13<744
 38104|  %501 = gep %10, i64 16                                                                                                ;L13<744
 38105|  store i64 %485, ptr %501,                                                                                             ;L13<744
 38106|  %502 = gep %10, i64 24                                                                                                ;L13<744
 38107|  store i64 %487, ptr %502,                                                                                             ;L13<744
 38108|  %503 = gep %10, i64 32                                                                                                ;L13<744
 38109|  store i64 %489, ptr %503,                                                                                             ;L13<744
 38110|  %504 = gep %10, i64 48                                                                                                ;L13<744
 38111|  store i32 %481, ptr %504,                                                                                             ;L13<744
 38112|  %505 = gep %10, i64 40                                                                                                ;L13<744
 38113|  store i32 %491, ptr %505,                                                                                             ;L13<744
 38114|  store ptr %492, ptr %10,                                                                                              ;L13<744
 38115|  %506 = gep %10, i64 8                                                                                                 ;L13<744
 38116|  store ptr %494, ptr %506,                                                                                             ;L13<744
 38117|  %507 = gep %10, i64 44                                                                                                ;L13<744
 38118|  store i32 %500, ptr %507,                                                                                             ;L13<744
 38119|  %508 = invoke { i64, i64 } @ai::plan_legacy8sub_plan13battle_common21effective_ult_cc_time(i64 %1, ptr %479, ptr %10)
 38120|  to label %517 unwind label %510                                                                                       ;L745
 38121| 
 38122| 509: ; preds = %483
 38123|  tail call void @llvm.trap()                                                                                           ;L2429<13<744
 38124|  unreachable                                                                                                           ;L2429<13<744
 38125| 
 38126| 510: ; preds = %609, %563, %559, %550, %548, %543, %532, %529, %521, %517, %498
 38127|  %511 = cleanuppad within none []
 38133|     ;; self = ptr %10
 38134|     ;; self = ptr %10
 38135|     ;; val = i64 1
 38136|     ;; order = i8 1
 38137|     ;; val = i64 1
 38138|     ;; order = i8 1
 38139|  %512 = load ptr, ptr %10, , !!8, !!8                                                                                  ;L441<2127<2831<825<825<794
 38140|     ;; self = ptr %512
 38141|     ;; dst = ptr %512
 38142|  %513 = atomicrmw sub ptr %512, i64 1 release, , !!45152                                                               ;L3956<3193<2831<825<825<794
 38143|  %514 = icmp eq i64 %513, 1                                                                                            ;L2831<825<825<794
 38144|  br i1 %514, label %515, label %516                                                                                    ;L2831<825<825<794
 38145| 
 38146| 515: ; preds = %510
 38147|     ;; order = i8 2
 38148|  fence acquire                                                                                                         ;L4387<64<825<825<794
 38149|  call void @gc::simulation6effect4type10EffectTypeEL_E9drop_slowBP_(ptr %10) [ "funclet"(token %511) ]                 ;L2874<825<825<794
 38150|  br label %516                                                                                                         ;L2874<825<825<794
 38151| 
 38152| 516: ; preds = %515, %510
 38153|  cleanupret from %511 unwind to caller                                                                                 ;L616
 38154| 
 38155| 517: ; preds = %498
 38156|  %518 = extractvalue { i64, i64 } %508, 0                                                                              ;L745
 38157|  %519 = extractvalue { i64, i64 } %508, 1                                                                              ;L745
 38158|     ;; ult_cc_time[0..+8] = i64 %518
 38159|     ;; ult_cc_time[8..+8] = i64 %519
 38160|  %520 = invoke i64 @gc::simulation6entityNtB5_6Entity15cooldown_reduce(ptr %23, i1 zeroext true)
 38161|  to label %521 unwind label %510                                                                                       ;L746
 38162| 
 38163| 521: ; preds = %517
 38164|  %522 = invoke i64 @ai::action_score22calculate_action_score(i64 %1, ptr %3, ptr %4, ptr %5, ptr %2, ptr %479, ptr %10, i64 %520, ptr %474, i8 1, ptr %7)
 38165|  to label %523 unwind label %510                                                                                       ;L746
 38166| 
 38167| 523: ; preds = %521
 38168|  %524 = add i64 %522, %26                                                                                              ;L746
 38169|     ;; score = i64 %524
 38170|  %525 = load i64, ptr %463, , !!8                                                                                      ;L748
 38173|     ;; __self_discr = i64 %525
 38174|     ;; __arg1_discr = i64 4
 38175|  %526 = icmp eq i64 %525, 4                                                                                            ;L9<748
 38176|  %527 = trunc nuw i64 %518 to i1
 38177|  %528 = select i1 %526, i1 %527, i1 false                                                                              ;L9<748
 38178|  br i1 %528, label %532, label %529                                                                                    ;L9<748
 38179| 
 38180| 529: ; preds = %534, %523
 38181|  %530 = phi i64 [ %542, %534 ], [ %524, %523 ]                                                                         ;L0
 38182|     ;; score = i64 %530
 38183|  %531 = invoke i64 @ai::plan_legacy8sub_plan13battle_common30v21_runaway_defensive_cc_bonus(i64 %1, ptr %4, ptr %5, ptr %2, ptr %463, ptr %23, ptr %474, ptr %10, i64 %518, i64 %519)
 38184|  to label %543 unwind label %510                                                                                       ;L757
 38185| 
 38186| 532: ; preds = %523
 38187|     ;; cc_time = i64 %519
 38188|  %533 = invoke i64 @gc::simulation6entityNtB5_6Entity8distance(ptr %474, ptr %23)
 38189|  to label %534 unwind label %510                                                                                       ;L750
 38190| 
 38191| 534: ; preds = %532
 38192|     ;; dist = i64 %533
 38193|     ;; rhs = i64 %533
 38194|  %535 = call i64 @llvm.usub.sat.i64(i64 80000, i64 %533)                                                               ;L2472<751
 38195|  %536 = trunc nuw nsw i64 %535 to i32                                                                                  ;L751
 38196|  %537 = udiv i32 %536, 10000                                                                                           ;L751
 38197|  %538 = zext nneg i32 %537 to i64                                                                                      ;L751
 38198|     ;; proximity_bonus = i64 %538
 38199|  %539 = mul i64 %519, 3                                                                                                ;L752
 38200|  %540 = add i64 %539, 3                                                                                                ;L752
 38201|  %541 = mul i64 %540, %538                                                                                             ;L752
 38202|  %542 = add i64 %541, %524                                                                                             ;L752
 38203|     ;; score = i64 %542
 38204|  br label %529                                                                                                         ;L749
 38205| 
 38206| 543: ; preds = %529
 38207|     ;; score = !DIArgList(i64 %530, i64 %531)
 38208|  %544 = load i64, ptr %0, , !!8                                                                                        ;L759
 38209|  %545 = gep %0, i64 8                                                                                                  ;L759
 38210|  %546 = load i64, ptr %545,                                                                                            ;L759
 38211|  %547 = invoke i64 @ai::plan_legacy8sub_plan13battle_common24v16_gambler_ult_cc_bonus(ptr %4, ptr %5, ptr %2, ptr %463, i64 %544, i64 %546, ptr %479, ptr %10, ptr %23, ptr %474)
 38212|  to label %548 unwind label %510                                                                                       ;L759
 38213| 
 38214| 548: ; preds = %543
 38215|     ;; score = !DIArgList(i64 %530, i64 %547, i64 %531)
 38216|  %549 = invoke i64 @ai::plan_legacy8sub_plan13battle_common25v16_knight_ult_zone_bonus(ptr %4, ptr %5, ptr %2, ptr %479, ptr %23, ptr %474)
 38217|  to label %550 unwind label %510                                                                                       ;L760
 38218| 
 38219| 550: ; preds = %548
 38220|     ;; score = !DIArgList(i64 %530, i64 %549, i64 %547, i64 %531)
 38221|  %551 = invoke i64 @ai::plan_legacy8sub_plan13battle_common31v17_runaway_counterattack_bonus(i64 %1, ptr %4, ptr %5, ptr %2, ptr %463, ptr %23, ptr %474, ptr %10)
 38222|  to label %552 unwind label %510                                                                                       ;L761
 38223| 
 38224| 552: ; preds = %550
 38225|  %553 = add i64 %531, %530                                                                                             ;L757
 38226|     ;; score = !DIArgList(i64 %553, i64 %549, i64 %547)
 38227|  %554 = add i64 %553, %547                                                                                             ;L759
 38228|     ;; score = !DIArgList(i64 %554, i64 %549)
 38229|  %555 = add i64 %554, %549                                                                                             ;L760
 38230|     ;; score = i64 %555
 38231|  %556 = add i64 %555, %551                                                                                             ;L761
 38232|     ;; score = i64 %556
 38233|     ;; self = ptr %474
 38234|  %557 = gep %474, i64 104                                                                                              ;L1404<763
 38235|  %558 = load i64, ptr %557, , !!8                                                                                      ;L1404<763
 38236|  switch i64 %558, label %589 [
 38237|  i64 13, label %559
 38238|  i64 3, label %587
 38239|  i64 4, label %596
 38240|  ]                                                                                                                     ;L763
 38241| 
 38242| 559: ; preds = %552
 38243|  %560 = gep %5, i64 8                                                                                                  ;L764
 38244|  %561 = load ptr, ptr %560, , !!8, !!8                                                                                 ;L764
 38245|  %562 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %10, ptr %561, ptr %23, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %474)
 38246|  to label %563 unwind label %510                                                                                       ;L764
 38247| 
 38248| 563: ; preds = %559
 38249|     ;; ult_damage = i64 %562
 38250|  %564 = gep %474, i64 1648                                                                                             ;L767
 38251|  %565 = load i64, ptr %564, , !!8                                                                                      ;L767
 38252|  %566 = icmp ult i64 %562, %565                                                                                        ;L767
 38253|  %567 = add i64 %556, 40
 38254|  %568 = select i1 %566, i64 %556, i64 %567                                                                             ;L767
 38255|     ;; score = i64 %568
 38256|     ;; self = ptr undef
 38257|     ;; has_cc = i64 %518
 38259|  invoke void @ai::fight_check18effect_buff_target(ptr sret([288 x i8]) %9, i64 %1, ptr %10, ptr %561, ptr %23, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %23, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54)
 38260|  to label %569 unwind label %510                                                                                       ;L773
 38261| 
 38262| 569: ; preds = %563
 38263|  %570 = icmp eq i64 %518, 1                                                                                            ;L430<772
 38264|     ;; has_cc = i1 %570
 38265|     ;; self = ptr %9
 38266|  %571 = gep %9, i64 72                                                                                                 ;L633<773
 38267|  %572 = load i32, ptr %571, , !!8                                                                                      ;L633<773
 38268|  %573 = icmp ne i32 %572, -1                                                                                           ;L633<773
 38269|     ;; has_buff = i1 %573
 38272|     ;; other = i64 1
 38274|  %574 = or i1 %570, %573                                                                                               ;L776
 38275|  br i1 %574, label %589, label %575                                                                                    ;L776
 38276| 
 38277| 575: ; preds = %569
 38278|  %576 = gep %474, i64 1576                                                                                             ;L774
 38279|  %577 = load i64, ptr %576, , !!8                                                                                      ;L774
 38280|     ;; self = i64 %577
 38281|  %578 = call i64 @llvm.umax.i64(i64 %577, i64 1)                                                                       ;L1039<774
 38282|  %579 = mul i64 %565, 100                                                                                              ;L774
 38283|  %580 = udiv i64 %579, %578                                                                                            ;L774
 38284|     ;; hp_ratio = i64 %580
 38285|  %581 = mul i64 %562, 3                                                                                                ;L776
 38286|  %582 = icmp ult i64 %581, %565                                                                                        ;L776
 38287|  %583 = icmp ugt i64 %580, 70                                                                                          ;L776
 38288|  %584 = and i1 %582, %583                                                                                              ;L776
 38289|  %585 = add i64 %568, -30
 38290|  %586 = select i1 %584, i64 %585, i64 %568                                                                             ;L776
 38291|  br label %589                                                                                                         ;L776
 38292| 
 38293| 587: ; preds = %552
 38294|     ;; score = i64 %556
 38295|  %588 = add i64 %556, 200                                                                                              ;L783
 38296|  br label %589                                                                                                         ;L782
 38297| 
 38298| 589: ; preds = %622, %620, %603, %601, %587, %575, %569, %552
 38299|  %590 = phi i64 [ %623, %622 ], [ %556, %552 ], [ %588, %587 ], [ %604, %603 ], [ %556, %601 ], [ %621, %620 ], [ %586, %575 ], [ %568, %569 ] ;L0
 38305|     ;; self = ptr %10
 38306|     ;; self = ptr %10
 38307|     ;; val = i64 1
 38308|     ;; order = i8 1
 38309|     ;; val = i64 1
 38310|     ;; order = i8 1
 38311|  %591 = load ptr, ptr %10, , !!8, !!8                                                                                  ;L441<2127<2831<825<825<794
 38312|     ;; self = ptr %591
 38313|     ;; dst = ptr %591
 38314|  %592 = atomicrmw sub ptr %591, i64 1 release, , !!45238                                                               ;L3956<3193<2831<825<825<794
 38315|  %593 = icmp eq i64 %592, 1                                                                                            ;L2831<825<825<794
 38316|  br i1 %593, label %594, label %595                                                                                    ;L2831<825<825<794
 38317| 
 38318| 594: ; preds = %589
 38319|     ;; order = i8 2
 38320|  fence acquire                                                                                                         ;L4387<64<825<825<794
 38321|  call void @gc::simulation6effect4type10EffectTypeEL_E9drop_slowBP_(ptr %10)                                           ;L2874<825<825<794
 38322|  br label %595                                                                                                         ;L2874<825<825<794
 38323| 
 38324| 595: ; preds = %594, %589
 38326|  br label %624                                                                                                         ;L742
 38327| 
 38328| 596: ; preds = %552
 38329|  %597 = sub nuw nsw i64 1, %12                                                                                         ;L784
 38330|     ;; team = i64 %597
 38331|  %598 = gep %474, i64 152                                                                                              ;L1378<784
 38332|  %599 = load i64, ptr %598, , !!8                                                                                      ;L1378<784
 38333|  %600 = icmp eq i64 %599, %597                                                                                         ;L1378<784
 38334|  br i1 %600, label %605, label %601                                                                                    ;L1378<784
 38335| 
 38336| 601: ; preds = %596
 38337|     ;; self = ptr %474
 38338|  %602 = icmp ult i64 %599, 2                                                                                           ;L1378<789
 38339|  br i1 %602, label %603, label %589                                                                                    ;L1378<789
 38340| 
 38341| 603: ; preds = %601
 38342|     ;; self = i64 %556
 38343|     ;; other = i64 1
 38344|  %604 = call i64 @llvm.smax.i64(i64 %556, i64 1)                                                                       ;L1039<790
 38345|  br label %589                                                                                                         ;L1039<790
 38346| 
 38347| 605: ; preds = %596
 38348|     ;; self = ptr %4
 38350|     ;; f[8..+8] = ptr %474
 38354|     ;; c = ptr %23
 38356|     ;; self = ptr %23
 38357|  %606 = gep %23, i64 1216                                                                                              ;L742<786<1543<786
 38358|  %607 = load i32, ptr %606, , !!45304, !!8                                                                             ;L742<786<1543<786
 38359|  %608 = icmp eq i32 %607, -1                                                                                           ;L742<786<1543<786
 38360|  br i1 %608, label %614, label %609                                                                                    ;L742<786<1543<786
 38361| 
 38362| 609: ; preds = %605
 38363|  %610 = gep %5, i64 8                                                                                                  ;L786
 38364|  %611 = load ptr, ptr %610, , !!8, !!8                                                                                 ;L786
 38365|     ;; f[0..+8] = ptr %611
 38366|  %612 = gep %23, i64 1168                                                                                              ;L742<786<1543<786
 38367|     ;; self = ptr %612
 38368|     ;; f[0..+8] = ptr %611
 38369|     ;; f[8..+8] = ptr %23
 38370|     ;; f[16..+8] = ptr %474
 38371|     ;; x = ptr %612
 38372|     ;; a = ptr %612
 38375|  %613 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %612, ptr %611, ptr %23, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %474)
 38376|  to label %614 unwind label %510                                                                                       ;L786<1162<786<1543<786
 38377| 
 38378| 614: ; preds = %609, %605
 38379|  %615 = phi i64 [ undef, %605 ], [ %613, %609 ]                                                                        ;L0<786<1543<786
 38382|  %616 = select i1 %608, i64 0, i64 %615                                                                                ;L1039<787
 38383|     ;; dmg_est = i64 %616
 38384|  %617 = gep %474, i64 1648                                                                                             ;L788
 38385|  %618 = load i64, ptr %617, , !!8                                                                                      ;L788
 38386|  %619 = icmp ult i64 %616, %618                                                                                        ;L788
 38387|  br i1 %619, label %620, label %622                                                                                    ;L788
 38388| 
 38389| 620: ; preds = %614
 38390|     ;; self = i64 %556
 38391|     ;; other = i64 1
 38392|  %621 = call i64 @llvm.smax.i64(i64 %556, i64 1)                                                                       ;L1039<788
 38393|  br label %589                                                                                                         ;L1039<788
 38394| 
 38395| 622: ; preds = %614
 38396|  %623 = add i64 %556, 80                                                                                               ;L788
 38397|  br label %589                                                                                                         ;L788
 38398| 
 38399| 624: ; preds = %595, %468, %450, %448, %446, %430, %428, %421, %418, %409, %387, %364, %346, %344, %342, %326, %324, %317, %314, %305, %283, %266, %248, %246, %244, %230, %228, %217, %215, %213, %210, %201, %190, %159, %151, %149, %94, %90, %87, %48, %25, %25, %25, %25, %25, %25, %25, %25, %25
 38400|  %625 = phi i64 [ %590, %595 ], [ %395, %387 ], [ %93, %90 ], [ %89, %87 ], [ %26, %94 ], [ %197, %215 ], [ %184, %159 ], [ -99999, %346 ], [ %291, %283 ], [ -99999, %468 ], [ %26, %151 ], [ %247, %246 ], [ %245, %244 ], [ %231, %230 ], [ -99999, %48 ], [ %202, %201 ], [ %214, %213 ], [ %212, %210 ], [ -99999, %450 ], [ %197, %228 ], [ %197, %190 ], [ %345, %344 ], [ %343, %342 ], [ %327, %326 ], [ -99999, %266 ], [ %306, %305 ], [ %318, %317 ], [ %316, %314 ], [ %291, %324 ], [ -99999, %364 ], [ %449, %448 ], [ %447, %446 ], [ %431, %430 ], [ %420, %418 ], [ %410, %409 ], [ %422, %421 ], [ %395, %428 ], [ %150, %149 ], [ -99999, %248 ], [ %26, %25 ], [ %222, %217 ], [ %26, %25 ], [ %26, %25 ], [ %26, %25 ], [ %26, %25 ], [ %26, %25 ], [ %26, %25 ], [ %26, %25 ], [ %26, %25 ] ;L0
 38401|  ret i64 %625                                                                                                          ;L837
 38402| }
