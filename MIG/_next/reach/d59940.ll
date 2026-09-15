 37635| define i64 @ai::action_score22calculate_action_score(i64 %0, ptr %1, ptr %2, ptr %3, ptr %4, ptr %5, ptr %6, i64 %7, ptr %8, i8 %9, ptr readnone %10) unnamed_addr #2 personality ptr @__CxxFrameHandler3 {
 37636|  %12 = alloca [64 x i8],
 37637|  %13 = alloca [24 x i8],
 37638|  %14 = alloca [24 x i8],
 37639|  %15 = alloca [64 x i8],
 37640|     ;; self[16..+64] = ptr %15
 37642|  %16 = alloca [24 x i8],
 37643|  %17 = alloca [24 x i8],
 37644|  %18 = alloca [24 x i8],
 37645|  %19 = alloca [24 x i8],
 37646|  %20 = alloca [24 x i8],
 37647|  %21 = alloca [24 x i8],
 37648|  %22 = alloca [40 x i8],
 37649|  %23 = alloca [40 x i8],
 37651|  %24 = alloca [64 x i8],
 37652|  %25 = alloca [64 x i8],
 37659|  %26 = alloca [24 x i8],
 37660|  %27 = alloca [16 x i8],
 37661|     ;; version = i64 %0
 37662|     ;; rnd = ptr %1
 37663|     ;; player = ptr %2
 37664|     ;; data = ptr %3
 37665|     ;; parameter = ptr %4
 37666|     ;; action = ptr %5
 37667|     ;; effect = ptr %6
 37668|     ;; speed_mult = i64 %7
 37669|     ;; t = ptr %8
 37670|     ;; self = ptr %8
 37671|     ;; ty = i8 %9
 37672|     ;; _debug = ptr %10
 37673|     ;; jrng = ptr %27
 37674|     ;; iter = ptr %24
 37675|     ;; iter = ptr %22
 37677|     ;; start = i64 0
 37678|     ;; end = i64 1000
 37679|     ;; n = i64 1
 37680|     ;; rhs = i64 1
 37681|     ;; n = i64 1
 37682|     ;; rhs = i64 1
 37683|     ;; start = i64 0
 37684|     ;; end = i64 1000
 37685|     ;; start = i64 0
 37686|     ;; end = i64 1000
 37687|     ;; start = i64 0
 37688|     ;; end = i64 1000
 37689|     ;; start = i64 0
 37690|     ;; end = i64 1000
 37691|     ;; start = i64 0
 37692|     ;; end = i64 1000
 37694|     ;; end = i64 1000
 37695|     ;; start = i64 0
 37696|     ;; end = i64 1000
 37697|     ;; start = i64 0
 37698|     ;; end = i64 1000
 37699|     ;; start = i64 0
 37700|     ;; end = i64 1000
 37701|     ;; start = i64 0
 37702|     ;; end = i64 1000
 37703|     ;; start = i64 0
 37704|     ;; end = i64 1000
 37705|     ;; start = i64 0
 37706|     ;; end = i64 1000
 37707|     ;; start = i64 0
 37708|     ;; end = i64 1000
 37709|     ;; start = i64 0
 37710|     ;; end = i64 1000
 37711|     ;; start = i64 0
 37712|     ;; end = i64 1000
 37713|     ;; start = i64 0
 37714|     ;; end = i64 1000
 37715|     ;; start = i64 0
 37716|     ;; end = i64 1000
 37717|     ;; start = i64 0
 37718|     ;; end = i64 1000
 37719|     ;; start = i64 0
 37720|     ;; end = i64 1000
 37721|  %28 = gep %2, i64 2352                                                                                                ;L10
 37722|  %29 = load i64, ptr %28, , !!8                                                                                        ;L10
 37723|  %30 = icmp ult i64 %29, 2                                                                                             ;L10
 37724|  br i1 %30, label %31, label %41                                                                                       ;L10
 37725| 
 37726| 31: ; preds = %11
 37727|     ;; self = ptr %2
 37728|  %32 = gep %2, i64 2496                                                                                                ;L581<10
 37729|  %33 = load i32, ptr %32, , !!8                                                                                        ;L581<10
 37730|  %34 = zext nneg i32 %33 to i64                                                                                        ;L581<10
 37731|  %35 = load ptr, ptr %3, , !!8, !!8                                                                                    ;L10
 37732|  %36 = gep %35, i64 480                                                                                                ;L10
 37733|  %37 = getelementptr [5 x ptr], ptr %36, i64 %29                                                                       ;L10
 37734|  %38 = getelementptr ptr, ptr %37, i64 %34                                                                             ;L10
 37735|  %39 = load ptr, ptr %38, , !!8                                                                                        ;L10
 37736|     ;; self = ptr %39
 37737|     ;; self = ptr %39
 37738|  %40 = icmp eq ptr %39, null                                                                                           ;L1011<10
 37739|  br i1 %40, label %70, label %42                                                                                       ;L1011<10
 37740| 
 37741| 41: ; preds = %11
 37742|  tail call void @core::panicking18panic_bounds_check(i64 %29, i64 2, ptr @anon.fdc8f8a40baf2989242b37eb35661f10.124) #29 ;L10
 37743|  unreachable                                                                                                           ;L10
 37744| 
 37745| 42: ; preds = %31
 37746|     ;; champ = ptr %39
 37747|     ;; champ = ptr %39
 37748|  %43 = gep %2, i64 384                                                                                                 ;L11
 37749|  %44 = tail call i64 @gc::simulation5state6playerNtB4_16AthleteParameter17last_hit_accuracy(ptr %43)                   ;L11
 37750|     ;; accuracy = i64 %44
 37751|     ;; start = i64 %44
 37752|  %45 = sub i64 1000, %44                                                                                               ;L12
 37753|     ;; base = i64 %45
 37754|     ;; min_v = i64 %44
 37755|  %46 = sub i64 2000, %44                                                                                               ;L13
 37756|     ;; max_v = i64 %46
 37758|  %47 = gep %8, i64 1472                                                                                                ;L18
 37759|  %48 = load i64, ptr %47, , !!8                                                                                        ;L18
 37760|  %49 = tail call { i64, i64 } @ai::utils18range_misjudge_rng(i64 %0, ptr %3, ptr %2, i64 %48)                          ;L18
 37761|  %50 = extractvalue { i64, i64 } %49, 0                                                                                ;L18
 37762|  %51 = extractvalue { i64, i64 } %49, 1                                                                                ;L18
 37763|  store i64 %50, ptr %27,                                                                                               ;L18
 37764|  %52 = gep %27, i64 8                                                                                                  ;L18
 37765|  store i64 %51, ptr %52,                                                                                               ;L18
 37766|  %53 = gep %3, i64 8                                                                                                   ;L19
 37767|  %54 = load ptr, ptr %53, , !!8, !!8                                                                                   ;L19
 37768|     ;; self = ptr %54
 37769|     ;; self = ptr %54
 37770|     ;; self = ptr %54
 37771|  %55 = tail call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %6, ptr %54, ptr %39, ptr @anon.fdc8f8a40baf2989242b37eb35661f10.17, ptr %8) ;L19
 37772|  %56 = call i64 @ai::utils23range_misjudge_roll_i64(ptr %1, ptr %27, i64 %44, i64 %46)                                 ;L19
 37773|  %57 = mul i64 %56, %55                                                                                                ;L19
 37774|  %58 = sdiv i64 %57, 1000                                                                                              ;L19
 37775|     ;; value = i64 %58
 37776|  %59 = gep %8, i64 1648                                                                                                ;L20
 37777|  %60 = load i64, ptr %59, , !!8                                                                                        ;L20
 37778|     ;; hp = i64 %60
 37779|  %61 = call i64 @ai::utils23range_misjudge_roll_i64(ptr %1, ptr %27, i64 %44, i64 %46)                                 ;L20
 37780|  %62 = mul i64 %61, %60                                                                                                ;L20
 37781|  %63 = sdiv i64 %62, 1000                                                                                              ;L20
 37782|     ;; hp = i64 %63
 37783|  %64 = gep %8, i64 1576                                                                                                ;L21
 37784|  %65 = load i64, ptr %64, , !!8                                                                                        ;L21
 37785|  %66 = call i64 @ai::utils23range_misjudge_roll_i64(ptr %1, ptr %27, i64 %44, i64 %46)                                 ;L21
 37786|  %67 = mul i64 %66, %65                                                                                                ;L21
 37787|  %68 = sdiv i64 %67, 1000                                                                                              ;L21
 37788|     ;; total = i64 %68
 37789|  %69 = icmp eq i64 %7, 0                                                                                               ;L22
 37790|  br i1 %69, label %92, label %71                                                                                       ;L22
 37791| 
 37792| 70: ; preds = %31
 37793|  tail call void @core::option13unwrap_failed(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.125) #29                       ;L1013<10
 37794|  unreachable                                                                                                           ;L1013<10
 37795| 
 37796| 71: ; preds = %42
 37797|  %72 = gep %6, i64 32                                                                                                  ;L22
 37798|  %73 = load i64, ptr %72, , !!8                                                                                        ;L22
 37799|  %74 = mul i64 %73, 100                                                                                                ;L22
 37800|  %75 = udiv i64 %74, %7                                                                                                ;L22
 37801|  %76 = call i64 @ai::utils23range_misjudge_roll_i64(ptr %1, ptr %27, i64 %44, i64 %46)                                 ;L22
 37802|  %77 = mul i64 %76, %75                                                                                                ;L22
 37803|  %78 = udiv i64 %77, 1000                                                                                              ;L22
 37804|     ;; start_timing = i64 %78
 37805|  %79 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L23
 37806|  %80 = gep %5, i64 8                                                                                                   ;L23
 37807|  %81 = load ptr, ptr %80, , !!8, !!8                                                                                   ;L23
 37808|  %82 = gep %81, i64 144                                                                                                ;L23
 37809|  %83 = load ptr, ptr %82, , !!8                                                                                        ;L23
 37810|  %84 = call i64 %83(ptr %79, ptr %39)                                                                                  ;L23
 37811|  %85 = mul i64 %84, 100                                                                                                ;L23
 37812|  %86 = udiv i64 %85, %7                                                                                                ;L23
 37813|  %87 = call i64 @ai::utils23range_misjudge_roll_i64(ptr %1, ptr %27, i64 %44, i64 %46)                                 ;L23
 37814|  %88 = mul i64 %87, %86                                                                                                ;L23
 37815|  %89 = udiv i64 %88, 1000                                                                                              ;L23
 37816|     ;; cooltime = i64 %89
 37817|     ;; self = ptr %8
 37818|  %90 = gep %8, i64 104                                                                                                 ;L1261<26
 37819|  %91 = load i64, ptr %90, , !!8                                                                                        ;L1261<26
 37820|  switch i64 %91, label %919 [
 37821|  i64 1, label %93
 37822|  i64 13, label %871
 37823|  ]                                                                                                                     ;L26
 37824| 
 37825| 92: ; preds = %42
 37826|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.126) #29  ;L22
 37827|  unreachable                                                                                                           ;L22
 37828| 
 37829| 93: ; preds = %71
 37830|     ;; info = ptr %8
 37831|  %94 = gep %8, i64 136                                                                                                 ;L29
 37832|  %95 = load i64, ptr %94, , !!8                                                                                        ;L29
 37833|     ;; self[0..+8] = i64 %95
 37835|  %96 = load ptr, ptr %35, , !!8, !!8                                                                                   ;L29
 37836|  %97 = gep %35, i64 8                                                                                                  ;L29
 37837|  %98 = load ptr, ptr %97, , !!8, !!8                                                                                   ;L29
 37838|     ;; f[0..+8] = ptr %96
 37839|     ;; f[8..+8] = ptr %98
 37840|  %99 = trunc nuw i64 %95 to i1                                                                                         ;L1542<29
 37841|  br i1 %99, label %100, label %107                                                                                     ;L1542<29
 37842| 
 37843| 100: ; preds = %93
 37844|  %101 = gep %8, i64 144                                                                                                ;L29
 37845|  %102 = load i64, ptr %101,                                                                                            ;L29
 37846|     ;; self[8..+8] = i64 %102
 37847|     ;; x = i64 %102
 37848|     ;; e = i64 %102
 37849|  %103 = gep %98, i64 496                                                                                               ;L29<1543<29
 37850|  %104 = load ptr, ptr %103, , !!8                                                                                      ;L29<1543<29
 37851|  %105 = call ptr %104(ptr %96, i64 %102)                                                                               ;L29<1543<29
 37852|  %106 = icmp eq ptr %105, null                                                                                         ;L29
 37853|  br i1 %106, label %107, label %122                                                                                    ;L29
 37854| 
 37855| 107: ; preds = %140, %122, %100, %93
 37856|  %108 = gep %98, i64 64                                                                                                ;L55
 37857|  %109 = load ptr, ptr %108, , !!8                                                                                      ;L55
 37858|  %110 = call { i64, ptr } %109(ptr %96)                                                                                ;L55
 37859|  %111 = extractvalue { i64, ptr } %110, 0                                                                              ;L55
 37860|     ;; self[0..+8] = i64 %111
 37862|  %112 = icmp ne i64 %111, 0                                                                                            ;L231<55
 37863|  %113 = extractvalue { i64, ptr } %110, 1
 37865|     ;; default = i64 0
 37867|  %114 = icmp eq ptr %113, null                                                                                         ;L1226<55
 37868|  %115 = select i1 %112, i1 true, i1 %114                                                                               ;L1226<55
 37869|  br i1 %115, label %175, label %116                                                                                    ;L1226<55
 37870| 
 37871| 116: ; preds = %107
 37872|     ;; t = ptr %113
 37874|     ;; m = ptr %113
 37875|     ;; self = ptr %113
 37876|  %117 = sub nuw nsw i64 1, %29                                                                                         ;L55<1227<55
 37877|     ;; team = i64 %117
 37878|  %118 = gep %113, i64 576                                                                                              ;L210<55<1227<55
 37879|  %119 = getelementptr i64, ptr %118, i64 %117                                                                          ;L210<55<1227<55
 37880|  %120 = load i64, ptr %119, , !!8                                                                                      ;L210<55<1227<55
 37881|  %121 = icmp eq i64 %120, 0                                                                                            ;L55
 37882|  br i1 %121, label %175, label %541                                                                                    ;L55
 37883| 
 37884| 122: ; preds = %100
 37885|     ;; target = ptr %105
 37886|     ;; self = ptr %105
 37887|  %123 = gep %105, i64 104                                                                                              ;L1386<30
 37888|  %124 = load i64, ptr %123, , !!8                                                                                      ;L1386<30
 37889|  switch i64 %124, label %107 [
 37890|  i64 2, label %125
 37891|  i64 3, label %159
 37892|  ]                                                                                                                     ;L30
 37893| 
 37894| 125: ; preds = %122
 37895|  %126 = gep %98, i64 64                                                                                                ;L32
 37896|  %127 = load ptr, ptr %126, , !!8                                                                                      ;L32
 37897|  %128 = call { i64, ptr } %127(ptr %96)                                                                                ;L32
 37898|  %129 = extractvalue { i64, ptr } %128, 0                                                                              ;L32
 37899|     ;; self[0..+8] = i64 %129
 37901|  %130 = icmp ne i64 %129, 0                                                                                            ;L231<32
 37902|  %131 = extractvalue { i64, ptr } %128, 1
 37904|     ;; default = i64 0
 37906|  %132 = icmp eq ptr %131, null                                                                                         ;L1226<32
 37907|  %133 = select i1 %130, i1 true, i1 %132                                                                               ;L1226<32
 37908|  br i1 %133, label %140, label %134                                                                                    ;L1226<32
 37909| 
 37910| 134: ; preds = %125
 37911|     ;; t = ptr %131
 37913|     ;; m = ptr %131
 37914|     ;; self = ptr %131
 37915|  %135 = sub nuw nsw i64 1, %29                                                                                         ;L32<1227<32
 37916|     ;; team = i64 %135
 37917|  %136 = gep %131, i64 576                                                                                              ;L210<32<1227<32
 37918|  %137 = getelementptr i64, ptr %136, i64 %135                                                                          ;L210<32<1227<32
 37919|  %138 = load i64, ptr %137, , !!8                                                                                      ;L210<32<1227<32
 37920|  %139 = icmp eq i64 %138, 0                                                                                            ;L32
 37921|  br i1 %139, label %140, label %541                                                                                    ;L32
 37922| 
 37923| 140: ; preds = %134, %125
 37924|     ;; self = ptr %105
 37925|     ;; self = ptr %105
 37926|  %141 = gep %105, i64 296                                                                                              ;L91<1313<37
 37927|  %142 = load i8, ptr %141, , !!8                                                                                       ;L91<1313<37
 37928|  %143 = add nsw i8 %142, -3                                                                                            ;L91<1313<37
 37929|  %144 = icmp ult i8 %143, 2                                                                                            ;L91<1313<37
 37930|  br i1 %144, label %145, label %107                                                                                    ;L91<1313<37
 37931| 
 37932| 145: ; preds = %140
 37933|  %146 = call { i64, ptr } %127(ptr %96)                                                                                ;L38
 37934|  %147 = extractvalue { i64, ptr } %146, 0                                                                              ;L38
 37935|     ;; self[0..+8] = i64 %147
 37937|  %148 = icmp ne i64 %147, 0                                                                                            ;L231<38
 37938|  %149 = extractvalue { i64, ptr } %146, 1
 37940|     ;; default = i64 0
 37942|  %150 = icmp eq ptr %149, null                                                                                         ;L1226<38
 37943|  %151 = select i1 %148, i1 true, i1 %150                                                                               ;L1226<38
 37944|  br i1 %151, label %158, label %152                                                                                    ;L1226<38
 37945| 
 37946| 152: ; preds = %145
 37947|     ;; t = ptr %149
 37949|     ;; m = ptr %149
 37950|     ;; self = ptr %149
 37951|  %153 = sub nuw nsw i64 1, %29                                                                                         ;L38<1227<38
 37952|     ;; team = i64 %153
 37953|  %154 = gep %149, i64 576                                                                                              ;L210<38<1227<38
 37954|  %155 = getelementptr i64, ptr %154, i64 %153                                                                          ;L210<38<1227<38
 37955|  %156 = load i64, ptr %155, , !!8                                                                                      ;L210<38<1227<38
 37956|  %157 = icmp eq i64 %156, 0                                                                                            ;L38
 37957|  br i1 %157, label %158, label %541                                                                                    ;L0
 37958| 
 37959| 158: ; preds = %152, %145
 37960|  br label %541                                                                                                         ;L0
 37961| 
 37962| 159: ; preds = %122
 37963|  %160 = gep %98, i64 64                                                                                                ;L46
 37964|  %161 = load ptr, ptr %160, , !!8                                                                                      ;L46
 37965|  %162 = call { i64, ptr } %161(ptr %96)                                                                                ;L46
 37966|  %163 = extractvalue { i64, ptr } %162, 0                                                                              ;L46
 37967|     ;; self[0..+8] = i64 %163
 37969|  %164 = icmp ne i64 %163, 0                                                                                            ;L231<46
 37970|  %165 = extractvalue { i64, ptr } %162, 1
 37972|     ;; default = i64 0
 37974|  %166 = icmp eq ptr %165, null                                                                                         ;L1226<46
 37975|  %167 = select i1 %164, i1 true, i1 %166                                                                               ;L1226<46
 37976|  br i1 %167, label %174, label %168                                                                                    ;L1226<46
 37977| 
 37978| 168: ; preds = %159
 37979|     ;; t = ptr %165
 37981|     ;; m = ptr %165
 37982|     ;; self = ptr %165
 37983|  %169 = sub nuw nsw i64 1, %29                                                                                         ;L46<1227<46
 37984|     ;; team = i64 %169
 37985|  %170 = gep %165, i64 576                                                                                              ;L210<46<1227<46
 37986|  %171 = getelementptr i64, ptr %170, i64 %169                                                                          ;L210<46<1227<46
 37987|  %172 = load i64, ptr %171, , !!8                                                                                      ;L210<46<1227<46
 37988|  %173 = icmp eq i64 %172, 0                                                                                            ;L46
 37989|  br i1 %173, label %174, label %541                                                                                    ;L0
 37990| 
 37991| 174: ; preds = %168, %159
 37992|  br label %541                                                                                                         ;L0
 37993| 
 37994| 175: ; preds = %116, %107
 37995|  %176 = load i64, ptr %4, , !!8                                                                                        ;L62
 37996|  %177 = trunc nuw i64 %176 to i1                                                                                       ;L62
 37997|  br i1 %177, label %178, label %267                                                                                    ;L62
 37998| 
 37999| 178: ; preds = %175
 38000|  %179 = gep %4, i64 8                                                                                                  ;L62
 38001|     ;; snapshot = ptr %179
 38006|     ;; self = ptr %179
 38007|     ;; entity_id = i64 %48
 38008|  %180 = gep %4, i64 2312                                                                                               ;L85<63
 38009|  %181 = load i64, ptr %180, , !!8                                                                                      ;L85<63
 38010|     ;; iter[8..+8] = i64 %181
 38011|     ;; iter[0..+8] = i64 0
 38012|     ;; self = ptr undef
 38013|     ;; self = ptr undef
 38014|     ;; self = ptr undef
 38015|     ;; other = ptr undef
 38016|  %182 = icmp eq i64 %181, 0                                                                                            ;L1916<900<985<85<63
 38017|  br i1 %182, label %267, label %183                                                                                    ;L900<985<85<63
 38018| 
 38019| 183: ; preds = %178
 38020|     ;; i = i64 0
 38021|     ;; iter[0..+8] = i64 1
 38022|  %184 = gep %4, i64 152                                                                                                ;L86<63
 38023|  %185 = load i64, ptr %184, , !!8                                                                                      ;L86<63
 38024|  %186 = icmp eq i64 %185, %48                                                                                          ;L86<63
 38025|  br i1 %186, label %275, label %187                                                                                    ;L86<63
 38026| 
 38027| 187: ; preds = %183
 38028|     ;; iter[0..+8] = i64 1
 38029|     ;; self = ptr undef
 38030|     ;; self = ptr undef
 38031|     ;; self = ptr undef
 38032|     ;; other = ptr undef
 38033|  %188 = icmp eq i64 %181, 1                                                                                            ;L1916<900<985<85<63
 38034|  br i1 %188, label %267, label %189                                                                                    ;L900<985<85<63
 38035| 
 38036| 189: ; preds = %187
 38037|     ;; i = i64 1
 38038|     ;; iter[0..+8] = i64 2
 38039|  %190 = gep %4, i64 200                                                                                                ;L86<63
 38040|  %191 = gep %4, i64 344                                                                                                ;L86<63
 38041|  %192 = load i64, ptr %191, , !!8                                                                                      ;L86<63
 38042|  %193 = icmp eq i64 %192, %48                                                                                          ;L86<63
 38043|  br i1 %193, label %275, label %194                                                                                    ;L86<63
 38044| 
 38045| 194: ; preds = %189
 38046|     ;; iter[0..+8] = i64 2
 38047|     ;; self = ptr undef
 38048|     ;; self = ptr undef
 38049|     ;; self = ptr undef
 38050|     ;; other = ptr undef
 38051|  %195 = icmp eq i64 %181, 2                                                                                            ;L1916<900<985<85<63
 38052|  br i1 %195, label %267, label %196                                                                                    ;L900<985<85<63
 38053| 
 38054| 196: ; preds = %194
 38055|     ;; i = i64 2
 38056|     ;; iter[0..+8] = i64 3
 38057|  %197 = gep %4, i64 392                                                                                                ;L86<63
 38058|  %198 = gep %4, i64 536                                                                                                ;L86<63
 38059|  %199 = load i64, ptr %198, , !!8                                                                                      ;L86<63
 38060|  %200 = icmp eq i64 %199, %48                                                                                          ;L86<63
 38061|  br i1 %200, label %275, label %201                                                                                    ;L86<63
 38062| 
 38063| 201: ; preds = %196
 38064|     ;; iter[0..+8] = i64 3
 38065|     ;; self = ptr undef
 38066|     ;; self = ptr undef
 38067|     ;; self = ptr undef
 38068|     ;; other = ptr undef
 38069|  %202 = icmp eq i64 %181, 3                                                                                            ;L1916<900<985<85<63
 38070|  br i1 %202, label %267, label %203                                                                                    ;L900<985<85<63
 38071| 
 38072| 203: ; preds = %201
 38073|     ;; i = i64 3
 38074|     ;; iter[0..+8] = i64 4
 38075|  %204 = gep %4, i64 584                                                                                                ;L86<63
 38076|  %205 = gep %4, i64 728                                                                                                ;L86<63
 38077|  %206 = load i64, ptr %205, , !!8                                                                                      ;L86<63
 38078|  %207 = icmp eq i64 %206, %48                                                                                          ;L86<63
 38079|  br i1 %207, label %275, label %208                                                                                    ;L86<63
 38080| 
 38081| 208: ; preds = %203
 38082|     ;; iter[0..+8] = i64 4
 38083|     ;; self = ptr undef
 38084|     ;; self = ptr undef
 38085|     ;; self = ptr undef
 38086|     ;; other = ptr undef
 38087|  %209 = icmp eq i64 %181, 4                                                                                            ;L1916<900<985<85<63
 38088|  br i1 %209, label %267, label %210                                                                                    ;L900<985<85<63
 38089| 
 38090| 210: ; preds = %208
 38091|     ;; i = i64 4
 38092|     ;; iter[0..+8] = i64 5
 38093|  %211 = gep %4, i64 776                                                                                                ;L86<63
 38094|  %212 = gep %4, i64 920                                                                                                ;L86<63
 38095|  %213 = load i64, ptr %212, , !!8                                                                                      ;L86<63
 38096|  %214 = icmp eq i64 %213, %48                                                                                          ;L86<63
 38097|  br i1 %214, label %275, label %215                                                                                    ;L86<63
 38098| 
 38099| 215: ; preds = %210
 38100|     ;; iter[0..+8] = i64 5
 38101|     ;; self = ptr undef
 38102|     ;; self = ptr undef
 38103|     ;; self = ptr undef
 38104|     ;; other = ptr undef
 38105|  %216 = icmp eq i64 %181, 5                                                                                            ;L1916<900<985<85<63
 38106|  br i1 %216, label %267, label %217                                                                                    ;L900<985<85<63
 38107| 
 38108| 217: ; preds = %215
 38109|     ;; i = i64 5
 38110|     ;; iter[0..+8] = i64 6
 38111|  %218 = gep %4, i64 968                                                                                                ;L86<63
 38112|  %219 = gep %4, i64 1112                                                                                               ;L86<63
 38113|  %220 = load i64, ptr %219, , !!8                                                                                      ;L86<63
 38114|  %221 = icmp eq i64 %220, %48                                                                                          ;L86<63
 38115|  br i1 %221, label %275, label %222                                                                                    ;L86<63
 38116| 
 38117| 222: ; preds = %217
 38118|     ;; iter[0..+8] = i64 6
 38119|     ;; self = ptr undef
 38120|     ;; self = ptr undef
 38121|     ;; self = ptr undef
 38122|     ;; other = ptr undef
 38123|  %223 = icmp eq i64 %181, 6                                                                                            ;L1916<900<985<85<63
 38124|  br i1 %223, label %267, label %224                                                                                    ;L900<985<85<63
 38125| 
 38126| 224: ; preds = %222
 38127|     ;; i = i64 6
 38128|     ;; iter[0..+8] = i64 7
 38129|  %225 = gep %4, i64 1160                                                                                               ;L86<63
 38130|  %226 = gep %4, i64 1304                                                                                               ;L86<63
 38131|  %227 = load i64, ptr %226, , !!8                                                                                      ;L86<63
 38132|  %228 = icmp eq i64 %227, %48                                                                                          ;L86<63
 38133|  br i1 %228, label %275, label %229                                                                                    ;L86<63
 38134| 
 38135| 229: ; preds = %224
 38136|     ;; iter[0..+8] = i64 7
 38137|     ;; self = ptr undef
 38138|     ;; self = ptr undef
 38139|     ;; self = ptr undef
 38140|     ;; other = ptr undef
 38141|  %230 = icmp eq i64 %181, 7                                                                                            ;L1916<900<985<85<63
 38142|  br i1 %230, label %267, label %231                                                                                    ;L900<985<85<63
 38143| 
 38144| 231: ; preds = %229
 38145|     ;; i = i64 7
 38146|     ;; iter[0..+8] = i64 8
 38147|  %232 = gep %4, i64 1352                                                                                               ;L86<63
 38148|  %233 = gep %4, i64 1496                                                                                               ;L86<63
 38149|  %234 = load i64, ptr %233, , !!8                                                                                      ;L86<63
 38150|  %235 = icmp eq i64 %234, %48                                                                                          ;L86<63
 38151|  br i1 %235, label %275, label %236                                                                                    ;L86<63
 38152| 
 38153| 236: ; preds = %231
 38154|     ;; iter[0..+8] = i64 8
 38155|     ;; self = ptr undef
 38156|     ;; self = ptr undef
 38157|     ;; self = ptr undef
 38158|     ;; other = ptr undef
 38159|  %237 = icmp eq i64 %181, 8                                                                                            ;L1916<900<985<85<63
 38160|  br i1 %237, label %267, label %238                                                                                    ;L900<985<85<63
 38161| 
 38162| 238: ; preds = %236
 38163|     ;; i = i64 8
 38164|     ;; iter[0..+8] = i64 9
 38165|  %239 = gep %4, i64 1544                                                                                               ;L86<63
 38166|  %240 = gep %4, i64 1688                                                                                               ;L86<63
 38167|  %241 = load i64, ptr %240, , !!8                                                                                      ;L86<63
 38168|  %242 = icmp eq i64 %241, %48                                                                                          ;L86<63
 38169|  br i1 %242, label %275, label %243                                                                                    ;L86<63
 38170| 
 38171| 243: ; preds = %238
 38172|     ;; iter[0..+8] = i64 9
 38173|     ;; self = ptr undef
 38174|     ;; self = ptr undef
 38175|     ;; self = ptr undef
 38176|     ;; other = ptr undef
 38177|  %244 = icmp eq i64 %181, 9                                                                                            ;L1916<900<985<85<63
 38178|  br i1 %244, label %267, label %245                                                                                    ;L900<985<85<63
 38179| 
 38180| 245: ; preds = %243
 38181|     ;; i = i64 9
 38182|     ;; iter[0..+8] = i64 10
 38183|  %246 = gep %4, i64 1736                                                                                               ;L86<63
 38184|  %247 = gep %4, i64 1880                                                                                               ;L86<63
 38185|  %248 = load i64, ptr %247, , !!8                                                                                      ;L86<63
 38186|  %249 = icmp eq i64 %248, %48                                                                                          ;L86<63
 38187|  br i1 %249, label %275, label %250                                                                                    ;L86<63
 38188| 
 38189| 250: ; preds = %245
 38190|     ;; iter[0..+8] = i64 10
 38191|     ;; self = ptr undef
 38192|     ;; self = ptr undef
 38193|     ;; self = ptr undef
 38194|     ;; other = ptr undef
 38195|  %251 = icmp eq i64 %181, 10                                                                                           ;L1916<900<985<85<63
 38196|  br i1 %251, label %267, label %252                                                                                    ;L900<985<85<63
 38197| 
 38198| 252: ; preds = %250
 38199|     ;; i = i64 10
 38200|     ;; iter[0..+8] = i64 11
 38201|  %253 = gep %4, i64 1928                                                                                               ;L86<63
 38202|  %254 = gep %4, i64 2072                                                                                               ;L86<63
 38203|  %255 = load i64, ptr %254, , !!8                                                                                      ;L86<63
 38204|  %256 = icmp eq i64 %255, %48                                                                                          ;L86<63
 38205|  br i1 %256, label %275, label %257                                                                                    ;L86<63
 38206| 
 38207| 257: ; preds = %252
 38208|     ;; iter[0..+8] = i64 11
 38209|     ;; self = ptr undef
 38210|     ;; self = ptr undef
 38211|     ;; self = ptr undef
 38212|     ;; other = ptr undef
 38213|  %258 = icmp eq i64 %181, 11                                                                                           ;L1916<900<985<85<63
 38214|  br i1 %258, label %267, label %259                                                                                    ;L900<985<85<63
 38215| 
 38216| 259: ; preds = %257
 38217|     ;; i = i64 11
 38218|     ;; iter[0..+8] = i64 12
 38219|  %260 = gep %4, i64 2120                                                                                               ;L86<63
 38220|  %261 = gep %4, i64 2264                                                                                               ;L86<63
 38221|  %262 = load i64, ptr %261, , !!8                                                                                      ;L86<63
 38222|  %263 = icmp eq i64 %262, %48                                                                                          ;L86<63
 38223|  br i1 %263, label %275, label %264                                                                                    ;L86<63
 38224| 
 38225| 264: ; preds = %259
 38226|     ;; iter[0..+8] = i64 12
 38227|     ;; self = ptr undef
 38228|     ;; self = ptr undef
 38229|     ;; self = ptr undef
 38230|     ;; other = ptr undef
 38231|  %265 = icmp eq i64 %181, 12                                                                                           ;L1916<900<985<85<63
 38232|  br i1 %265, label %267, label %266                                                                                    ;L900<985<85<63
 38233| 
 38234| 266: ; preds = %264
 38235|     ;; iter[0..+8] = i64 12
 38236|     ;; i = i64 12
 38237|  call void @core::panicking18panic_bounds_check(i64 12, i64 12, ptr @anon.fdc8f8a40baf2989242b37eb35661f10.97) #29, !!43560 ;L86<63
 38238|  unreachable                                                                                                           ;L86<63
 38239| 
 38240| 267: ; preds = %264, %257, %250, %243, %236, %229, %222, %215, %208, %201, %194, %187, %178, %175
 38241|     ;; applyed_damage = i64 0
 38242|     ;; expected_damage = i64 0
 38243|  %268 = load ptr, ptr %35, , !!8, !!8                                                                                  ;L199
 38244|  %269 = load ptr, ptr %97, , !!8, !!8                                                                                  ;L199
 38245|  %270 = gep %269, i64 40                                                                                               ;L199
 38246|  %271 = load ptr, ptr %270, , !!8                                                                                      ;L199
 38247|  %272 = call i64 %271(ptr %268)                                                                                        ;L199
 38248|     ;; tick = i64 %272
 38249|     ;; tick = i64 %272
 38250|     ;; self = ptr %54
 38251|  %273 = gep %54, i64 56                                                                                                ;L263<399<199
 38252|  %274 = load i8, ptr %273, , !!8                                                                                       ;L263<399<199
 38253|  switch i8 %274, label %417 [
 38254|  i8 0, label %407
 38255|  i8 7, label %407
 38256|  i8 8, label %407
 38257|  i8 5, label %407
 38258|  ]                                                                                                                     ;L263<399<199
 38259| 
 38260| 275: ; preds = %259, %252, %245, %238, %231, %224, %217, %210, %203, %196, %189, %183
 38261|  %276 = phi ptr [ %253, %252 ], [ %179, %183 ], [ %225, %224 ], [ %190, %189 ], [ %246, %245 ], [ %197, %196 ], [ %232, %231 ], [ %204, %203 ], [ %260, %259 ], [ %211, %210 ], [ %239, %238 ], [ %218, %217 ] ;L0<63
 38262|     ;; traj = ptr %276
 38263|  %277 = mul i64 %45, %45                                                                                               ;L68
 38264|  %278 = udiv i64 %277, 1000                                                                                            ;L68
 38265|     ;; self = i64 %278
 38266|     ;; other = i64 1000
 38267|  %279 = call i64 @llvm.umin.i64(i64 %278, i64 1000)                                                                    ;L1078<68
 38268|     ;; error_prob = i64 %279
 38269|  store i64 0, ptr %26,                                                                                                 ;L391<72
 38270|  %280 = gep %26, i64 8                                                                                                 ;L391<72
 38271|  store i64 1000, ptr %280,                                                                                             ;L391<72
 38272|  %281 = gep %26, i64 16                                                                                                ;L391<72
 38273|  store i8 0, ptr %281,                                                                                                 ;L391<72
 38275|  call void @llvm.memcpy.p0.p0.i64(ptr %14, ptr %26, i64 24, i1 false)                                                  ;L72
 38276|  %282 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %1, ptr %14)                            ;L72
 38278|  %283 = icmp ult i64 %282, %279                                                                                        ;L72
 38279|  br i1 %283, label %541, label %284                                                                                    ;L72
 38280| 
 38281| 284: ; preds = %275
 38282|  %285 = load ptr, ptr %35, , !!8, !!8                                                                                  ;L76
 38283|  %286 = load ptr, ptr %97, , !!8, !!8                                                                                  ;L76
 38284|  %287 = gep %286, i64 40                                                                                               ;L76
 38285|  %288 = load ptr, ptr %287, , !!8                                                                                      ;L76
 38286|  %289 = call i64 %288(ptr %285)                                                                                        ;L76
 38287|     ;; tick = i64 %289
 38288|     ;; tick = i64 %289
 38289|     ;; self = ptr %54
 38290|  %290 = gep %54, i64 56                                                                                                ;L263<399<76
 38291|  %291 = load i8, ptr %290, , !!8                                                                                       ;L263<399<76
 38292|  switch i8 %291, label %302 [
 38293|  i8 0, label %292
 38294|  i8 7, label %292
 38295|  i8 8, label %292
 38296|  i8 5, label %292
 38297|  ]                                                                                                                     ;L263<399<76
 38298| 
 38299| 292: ; preds = %284, %284, %284, %284
 38300|  %293 = gep %54, i64 8                                                                                                 ;L399<76
 38301|  %294 = load ptr, ptr %293, , !!8, !!8                                                                                 ;L399<76
 38302|     ;; self = ptr %294
 38303|  %295 = gep %294, i64 2216                                                                                             ;L703<399<76
 38304|  %296 = load i64, ptr %295, , !!8                                                                                      ;L703<399<76
 38305|     ;; self = i64 %296
 38306|  %297 = gep %294, i64 4856                                                                                             ;L704<399<76
 38307|  %298 = load i64, ptr %297, , !!8                                                                                      ;L704<399<76
 38308|  %299 = mul i64 %298, 30                                                                                               ;L704<399<76
 38309|     ;; rhs = i64 %299
 38310|  %300 = call i64 @llvm.usub.sat.i64(i64 %296, i64 %299)                                                                ;L2472<703<399<76
 38311|  %301 = icmp ult i64 %289, %300                                                                                        ;L703<399<76
 38312|  br i1 %301, label %302, label %304                                                                                    ;L76
 38313| 
 38314| 302: ; preds = %292, %284
 38315|  %303 = icmp ne i32 %33, 1                                                                                             ;L77
 38316|  br label %304                                                                                                         ;L0
 38317| 
 38318| 304: ; preds = %302, %292
 38319|  %305 = phi i1 [ %303, %302 ], [ false, %292 ]                                                                         ;L0
 38321|  %306 = call fastcc i64 @ai::utilsNtB4_18MinionHpTrajectory10hp_at_tick(ptr %276, i64 %78)                             ;L89
 38322|  %307 = call i64 @ai::utils23range_misjudge_roll_i64(ptr %1, ptr %27, i64 %44, i64 %46)                                ;L89
 38323|  %308 = mul i64 %307, %306                                                                                             ;L89
 38324|  %309 = sdiv i64 %308, 1000                                                                                            ;L89
 38325|     ;; predicted_hp = i64 %309
 38326|  %310 = gep %276, i64 184                                                                                              ;L90
 38327|  %311 = load i64, ptr %310, , !!8                                                                                      ;L90
 38328|     ;; death_tick = i64 %311
 38329|  %312 = icmp sgt i64 %308, 999                                                                                         ;L93
 38330|  br i1 %312, label %313, label %318                                                                                    ;L93
 38331| 
 38332| 313: ; preds = %304
 38333|  %314 = add nsw i64 %58, 5                                                                                             ;L93
 38334|  %315 = icmp sgt i64 %309, %314                                                                                        ;L93
 38335|     ;; can_last_hit = i1 %315
 38336|  %316 = add nuw nsw i64 %89, %78                                                                                       ;L95
 38337|  %317 = icmp ule i64 %311, %316                                                                                        ;L95
 38338|     ;; will_die_soon = i1 %317
 38339|  br i1 %315, label %321, label %325                                                                                    ;L97
 38340| 
 38341| 318: ; preds = %321, %304
 38342|  %319 = mul nsw i64 %58, 3                                                                                             ;L159
 38343|  %320 = icmp sgt i64 %309, %319                                                                                        ;L159
 38344|  br i1 %320, label %337, label %336                                                                                    ;L159
 38345| 
 38346| 321: ; preds = %313
 38347|  %322 = shl nsw i64 %58, 1                                                                                             ;L152
 38348|  %323 = icmp sle i64 %309, %322                                                                                        ;L152
 38349|  %324 = and i1 %317, %323                                                                                              ;L152
 38350|  br i1 %324, label %333, label %318                                                                                    ;L152
 38351| 
 38352| 325: ; preds = %313
 38353|  %326 = add nuw nsw i64 %78, 5                                                                                         ;L100
 38354|  %327 = icmp ugt i64 %311, %326                                                                                        ;L100
 38355|  %328 = select i1 %317, i64 25, i64 15                                                                                 ;L100
 38356|  %329 = select i1 %327, i64 %328, i64 30                                                                               ;L100
 38357|     ;; urgency = i64 %329
 38358|     ;; concurrent_lastable = i64 0
 38359|     ;; iter[0..+8] = i64 0
 38360|     ;; iter[8..+8] = i64 %181
 38361|     ;; iter[0..+8] = i64 0
 38362|     ;; iter[8..+8] = i64 %181
 38363|  br label %330                                                                                                         ;L113
 38364| 
 38365| 330: ; preds = %400, %325
 38366|  %331 = phi i64 [ %394, %400 ], [ 0, %325 ]
 38367|  %332 = phi i64 [ %406, %400 ], [ 0, %325 ]
 38368|  br label %355                                                                                                         ;L900<985<113
 38369| 
 38370| 333: ; preds = %321
 38371|  %334 = icmp eq i8 %9, 0                                                                                               ;L155
 38372|  %335 = select i1 %334, i64 -9999, i64 5                                                                               ;L95
 38373|  br label %541                                                                                                         ;L95
 38374| 
 38375| 336: ; preds = %318
 38376|  switch i8 %9, label %339 [
 38377|  i8 0, label %541
 38378|  i8 2, label %338
 38379|  ]                                                                                                                     ;L178
 38380| 
 38381| 337: ; preds = %318
 38382|  switch i8 %9, label %344 [
 38383|  i8 0, label %541
 38384|  i8 2, label %343
 38385|  ]                                                                                                                     ;L161
 38386| 
 38387| 338: ; preds = %336
 38388|  br label %541                                                                                                         ;L1
 38389| 
 38390| 339: ; preds = %336
 38391|  %340 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %1, ptr %26)                            ;L184
 38392|  %341 = icmp ult i64 %340, %279                                                                                        ;L184
 38393|  %342 = select i1 %341, i64 10, i64 -9999                                                                              ;L184
 38394|  br label %541                                                                                                         ;L184
 38395| 
 38396| 343: ; preds = %337
 38397|  br label %541                                                                                                         ;L1
 38398| 
 38399| 344: ; preds = %337
 38400|  %345 = icmp ugt i64 %88, 29999                                                                                        ;L168
 38401|  %346 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %1, ptr %26)                            ;L0
 38402|  %347 = icmp ult i64 %346, %279                                                                                        ;L0
 38403|  br i1 %345, label %349, label %348                                                                                    ;L168
 38404| 
 38405| 348: ; preds = %344
 38406|  br i1 %305, label %353, label %351                                                                                    ;L171
 38407| 
 38408| 349: ; preds = %344
 38409|  %350 = select i1 %347, i64 10, i64 -9999                                                                              ;L169
 38410|  br label %541                                                                                                         ;L169
 38411| 
 38412| 351: ; preds = %348
 38413|  %352 = select i1 %347, i64 -9999, i64 10                                                                              ;L174
 38414|  br label %541                                                                                                         ;L174
 38415| 
 38416| 353: ; preds = %348
 38417|  %354 = select i1 %347, i64 10, i64 -9999                                                                              ;L173
 38418|  br label %541                                                                                                         ;L173
 38419| 
 38420| 355: ; preds = %393, %330
 38421|  %356 = phi i64 [ %394, %393 ], [ %331, %330 ]                                                                         ;L113
 38422|     ;; concurrent_lastable = i64 %332
 38423|     ;; iter[0..+8] = i64 %356
 38424|     ;; self = ptr undef
 38425|     ;; self = ptr undef
 38426|     ;; self = ptr undef
 38427|     ;; other = ptr undef
 38428|  %357 = icmp ult i64 %356, %181                                                                                        ;L1916<900<985<113
 38429|  br i1 %357, label %360, label %358                                                                                    ;L900<985<113
 38430| 
 38431| 358: ; preds = %355
 38432|  %359 = icmp sgt i64 %332, 0                                                                                           ;L126
 38433|  br i1 %359, label %390, label %362                                                                                    ;L126
 38434| 
 38435| 360: ; preds = %355
 38436|     ;; old = i64 %356
 38437|     ;; start = i64 %356
 38438|     ;; self = i64 %356
 38439|     ;; iter[0..+8] = i64 %356
 38440|     ;; i = i64 %356
 38441|  %361 = icmp ult i64 %356, 12                                                                                          ;L114
 38442|  br i1 %361, label %393, label %399                                                                                    ;L114
 38443| 
 38444| 362: ; preds = %367, %358
 38445|  %363 = phi i64 [ %369, %367 ], [ 0, %358 ]                                                                            ;L0
 38446|     ;; multi_bonus = i64 %363
 38447|  br i1 %305, label %375, label %372                                                                                    ;L147
 38448| 
 38449| 364: ; preds = %390, %377
 38450|  %365 = phi i64 [ %378, %377 ], [ %392, %390 ]                                                                         ;L0
 38451|     ;; iter[0..+8] = i64 %365
 38452|     ;; earlier_count = i64 %391
 38453|     ;; self = ptr undef
 38454|     ;; self = ptr undef
 38455|     ;; self = ptr undef
 38456|     ;; other = ptr undef
 38457|  %366 = icmp ult i64 %365, %181                                                                                        ;L1916<900<985<129
 38458|  br i1 %366, label %370, label %367                                                                                    ;L900<985<129
 38459| 
 38460| 367: ; preds = %364
 38461|  %368 = icmp eq i64 %391, 0                                                                                            ;L138
 38462|  %369 = select i1 %368, i64 5, i64 -5                                                                                  ;L138
 38463|  br label %362                                                                                                         ;L138
 38464| 
 38465| 370: ; preds = %364
 38466|     ;; old = i64 %365
 38467|     ;; start = i64 %365
 38468|     ;; self = i64 %365
 38469|     ;; iter[0..+8] = i64 %365
 38470|     ;; i = i64 %365
 38471|  %371 = icmp ult i64 %365, 12                                                                                          ;L130
 38472|  br i1 %371, label %377, label %383                                                                                    ;L130
 38473| 
 38474| 372: ; preds = %362
 38475|  %373 = add nuw nsw i64 %329, 3                                                                                        ;L150
 38476|  %374 = add nsw i64 %373, %363                                                                                         ;L150
 38477|  br label %541                                                                                                         ;L150
 38478| 
 38479| 375: ; preds = %362
 38480|  %376 = add nsw i64 %363, %329                                                                                         ;L149
 38481|  br label %541                                                                                                         ;L149
 38482| 
 38483| 377: ; preds = %370
 38484|  %378 = add nuw nsw i64 %365, 1                                                                                        ;L971<215<903<985<129
 38485|     ;; iter[0..+8] = i64 %378
 38487|  %379 = gepS %179, i64 %365                                                                                            ;L131
 38488|  %380 = gep %379, i64 144                                                                                              ;L131
 38489|  %381 = load i64, ptr %380, , !!8                                                                                      ;L131
 38490|  %382 = icmp eq i64 %381, %48                                                                                          ;L131
 38491|  br i1 %382, label %364, label %384                                                                                    ;L131
 38492| 
 38493| 383: ; preds = %370
 38494|  call void @core::panicking18panic_bounds_check(i64 12, i64 12, ptr @anon.fdc8f8a40baf2989242b37eb35661f10.127) #29    ;L130
 38495|  unreachable                                                                                                           ;L130
 38496| 
 38497| 384: ; preds = %377
 38498|  %385 = gep %379, i64 184                                                                                              ;L134
 38499|  %386 = load i64, ptr %385, , !!8                                                                                      ;L134
 38500|  %387 = icmp ult i64 %386, %311                                                                                        ;L134
 38501|  %388 = zext i1 %387 to i64                                                                                            ;L134
 38502|  %389 = add i64 %391, %388                                                                                             ;L134
 38503|     ;; earlier_count = i64 %389
 38504|  br label %390                                                                                                         ;L129
 38505| 
 38506| 390: ; preds = %384, %358
 38507|  %391 = phi i64 [ %389, %384 ], [ 0, %358 ]
 38508|  %392 = phi i64 [ %378, %384 ], [ 0, %358 ]
 38509|  br label %364                                                                                                         ;L900<985<129
 38510| 
 38511| 393: ; preds = %360
 38512|  %394 = add nuw nsw i64 %356, 1                                                                                        ;L971<215<903<985<113
 38513|     ;; iter[0..+8] = i64 %394
 38514|  %395 = gepS %179, i64 %356                                                                                            ;L114
 38515|     ;; other = ptr %395
 38516|  %396 = gep %395, i64 144                                                                                              ;L115
 38517|  %397 = load i64, ptr %396, , !!8                                                                                      ;L115
 38518|  %398 = icmp eq i64 %397, %48                                                                                          ;L115
 38519|  br i1 %398, label %355, label %400                                                                                    ;L115
 38520| 
 38521| 399: ; preds = %360
 38522|  call void @core::panicking18panic_bounds_check(i64 12, i64 12, ptr @anon.fdc8f8a40baf2989242b37eb35661f10.128) #29    ;L114
 38523|  unreachable                                                                                                           ;L114
 38524| 
 38525| 400: ; preds = %393
 38526|  %401 = call fastcc i64 @ai::utilsNtB4_18MinionHpTrajectory10hp_at_tick(ptr %395, i64 %316)                            ;L119
 38527|     ;; other_hp_at_cd = i64 %401
 38528|  %402 = icmp sgt i64 %401, 0                                                                                           ;L120
 38529|  %403 = icmp sle i64 %401, %314                                                                                        ;L120
 38530|  %404 = and i1 %402, %403                                                                                              ;L120
 38531|  %405 = zext i1 %404 to i64                                                                                            ;L120
 38532|  %406 = add i64 %332, %405                                                                                             ;L120
 38533|     ;; concurrent_lastable = i64 %406
 38534|  br label %330                                                                                                         ;L113
 38535| 
 38536| 407: ; preds = %267, %267, %267, %267
 38537|  %408 = gep %54, i64 8                                                                                                 ;L399<199
 38538|  %409 = load ptr, ptr %408, , !!8, !!8                                                                                 ;L399<199
 38539|     ;; self = ptr %409
 38540|  %410 = gep %409, i64 2216                                                                                             ;L703<399<199
 38541|  %411 = load i64, ptr %410, , !!8                                                                                      ;L703<399<199
 38542|     ;; self = i64 %411
 38543|  %412 = gep %409, i64 4856                                                                                             ;L704<399<199
 38544|  %413 = load i64, ptr %412, , !!8                                                                                      ;L704<399<199
 38545|  %414 = mul i64 %413, 30                                                                                               ;L704<399<199
 38546|     ;; rhs = i64 %414
 38547|  %415 = call i64 @llvm.usub.sat.i64(i64 %411, i64 %414)                                                                ;L2472<703<399<199
 38548|  %416 = icmp ult i64 %272, %415                                                                                        ;L703<399<199
 38549|  br i1 %416, label %417, label %419                                                                                    ;L199
 38550| 
 38551| 417: ; preds = %407, %267
 38552|  %418 = icmp ne i32 %33, 1                                                                                             ;L200
 38553|  br label %419                                                                                                         ;L0
 38554| 
 38555| 419: ; preds = %417, %407
 38556|  %420 = phi i1 [ %418, %417 ], [ false, %407 ]                                                                         ;L0
 38558|  %421 = gep %269, i64 512                                                                                              ;L211
 38559|  %422 = load ptr, ptr %421, , !!8                                                                                      ;L211
 38560|  call void %422(ptr sret([64 x i8]) %25, ptr %268)                                                                     ;L211
 38562|  call void @llvm.memcpy.p0.p0.i64(ptr %24, ptr %25, i64 64, i1 false)                                                  ;L211
 38563|  %423 = add nuw nsw i64 %89, %78
 38564|  br label %424                                                                                                         ;L211
 38565| 
 38566| 424: ; preds = %597, %419
 38567|  %425 = phi i64 [ 0, %419 ], [ %598, %597 ]                                                                            ;L195
 38568|  %426 = phi i64 [ 0, %419 ], [ %599, %597 ]                                                                            ;L197
 38569|     ;; expected_damage = i64 %426
 38570|     ;; applyed_damage = i64 %425
 38571|  %427 = call ptr @gc::simulationNtB5_10EntityIterNtNtNtNtCsjihNppCmMEE_4core4iter6traits8iterator8Iterator4next(ptr %24) ;L211
 38572|  %428 = icmp eq ptr %427, null                                                                                         ;L211
 38573|  br i1 %428, label %432, label %429                                                                                    ;L211
 38574| 
 38575| 429: ; preds = %424
 38576|     ;; e = ptr %427
 38577|  %430 = gep %427, i64 104                                                                                              ;L212
 38578|  %431 = load i64, ptr %430, , !!8                                                                                      ;L212
 38579|  switch i64 %431, label %597 [
 38580|  i64 1, label %600
 38581|  i64 2, label %605
 38582|  i64 7, label %610
 38583|  i64 8, label %615
 38584|  i64 9, label %620
 38585|  i64 10, label %625
 38586|  ]                                                                                                                     ;L212
 38587| 
 38588| 432: ; preds = %424
 38590|  %433 = gep %269, i64 528                                                                                              ;L297
 38591|  %434 = load ptr, ptr %433, , !!8                                                                                      ;L297
 38592|  call void %434(ptr sret([40 x i8]) %23, ptr %268)                                                                     ;L297
 38594|  call void @llvm.memcpy.p0.p0.i64(ptr %22, ptr %23, i64 40, i1 false)                                                  ;L297
 38595|  %435 = gep %269, i64 496
 38596|  %436 = gep %8, i64 1632
 38597|  %437 = load i64, ptr %436,
 38598|  %438 = gep %8, i64 1640
 38599|  %439 = load i64, ptr %438,
 38600|  %440 = gep %8, i64 1136
 38601|  %441 = load i32, ptr %440,
 38602|  %442 = icmp eq i32 %441, 0
 38603|  %443 = sext i32 %441 to i64
 38604|  %444 = gep %8, i64 1664
 38605|  %445 = load i64, ptr %444,
 38606|  %446 = add nsw i64 %443, 100
 38607|  %447 = mul i64 %445, %446
 38608|  %448 = udiv i64 %447, 100
 38609|  %449 = select i1 %442, i64 %445, i64 %448
 38610|  br label %450                                                                                                         ;L297
 38611| 
 38612| 450: ; preds = %577, %432
 38613|  %451 = phi i64 [ %425, %432 ], [ %578, %577 ]                                                                         ;L0
 38614|  %452 = phi i64 [ %426, %432 ], [ %579, %577 ]                                                                         ;L0
 38615|     ;; expected_damage = i64 %452
 38616|     ;; applyed_damage = i64 %451
 38617|  %453 = call ptr @gc::simulationNtB5_14ProjectileIterNtNtNtNtCsjihNppCmMEE_4core4iter6traits8iterator8Iterator4next(ptr %22) ;L297
 38618|  %454 = icmp eq ptr %453, null                                                                                         ;L297
 38619|  br i1 %454, label %460, label %455                                                                                    ;L297
 38620| 
 38621| 455: ; preds = %450
 38622|     ;; p = ptr %453
 38623|  %456 = gep %453, i64 64                                                                                               ;L298
 38624|  %457 = load i64, ptr %456, , !!8                                                                                      ;L298
 38625|  %458 = icmp ne i64 %457, 9                                                                                            ;L298
 38626|  call void @llvm.assume(i1 %458)                                                                                       ;L298
 38627|  %459 = icmp eq i64 %457, 6                                                                                            ;L298
 38628|  br i1 %459, label %543, label %547                                                                                    ;L298
 38629| 
 38630| 460: ; preds = %450
 38632|  %461 = call i64 @ai::utils23range_misjudge_roll_i64(ptr %1, ptr %27, i64 %44, i64 %46)                                ;L319
 38633|  %462 = mul i64 %461, %451                                                                                             ;L319
 38634|  %463 = udiv i64 %462, 1000                                                                                            ;L319
 38635|     ;; applyed_damage = i64 %463
 38636|  %464 = call i64 @ai::utils23range_misjudge_roll_i64(ptr %1, ptr %27, i64 %44, i64 %46)                                ;L320
 38637|  %465 = mul i64 %464, %452                                                                                             ;L320
 38638|  %466 = udiv i64 %465, 1000                                                                                            ;L320
 38639|     ;; expected_damage = i64 %466
 38641|  store i64 %44, ptr %21,                                                                                               ;L391<321
 38642|  %467 = gep %21, i64 8                                                                                                 ;L391<321
 38643|  store i64 1000, ptr %467,                                                                                             ;L391<321
 38644|  %468 = gep %21, i64 16                                                                                                ;L391<321
 38645|  store i8 0, ptr %468,                                                                                                 ;L391<321
 38646|  %469 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %1, ptr %21)                            ;L321
 38648|  %470 = sub i64 1000, %469                                                                                             ;L321
 38649|     ;; error_prob = i64 %470
 38650|  %471 = add nsw i64 %58, -5                                                                                            ;L325
 38651|  %472 = add nsw i64 %471, %463                                                                                         ;L325
 38652|  %473 = icmp slt i64 %472, %63                                                                                         ;L325
 38653|  br i1 %473, label %474, label %480                                                                                    ;L325
 38654| 
 38655| 474: ; preds = %460
 38656|  %475 = mul nsw i64 %68, 3                                                                                             ;L365
 38657|  %476 = sdiv i64 %475, 10                                                                                              ;L365
 38658|  %477 = add nsw i64 %476, %58                                                                                          ;L365
 38659|  %478 = add nsw i64 %477, %466                                                                                         ;L365
 38660|  %479 = icmp slt i64 %478, %63                                                                                         ;L365
 38661|  br i1 %479, label %486, label %485                                                                                    ;L365
 38662| 
 38663| 480: ; preds = %460
 38664|  store i64 0, ptr %20,                                                                                                 ;L391<329
 38665|  %481 = gep %20, i64 8                                                                                                 ;L391<329
 38666|  store i64 1000, ptr %481,                                                                                             ;L391<329
 38667|  %482 = gep %20, i64 16                                                                                                ;L391<329
 38668|  store i8 0, ptr %482,                                                                                                 ;L391<329
 38670|  call void @llvm.memcpy.p0.p0.i64(ptr %13, ptr %20, i64 24, i1 false)                                                  ;L329
 38671|  %483 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %1, ptr %13)                            ;L329
 38673|  %484 = icmp ult i64 %483, %470                                                                                        ;L329
 38674|  br i1 %484, label %541, label %516                                                                                    ;L329
 38675| 
 38676| 485: ; preds = %474
 38677|  switch i8 %9, label %488 [
 38678|  i8 0, label %541
 38679|  i8 2, label %487
 38680|  ]                                                                                                                     ;L393
 38681| 
 38682| 486: ; preds = %474
 38683|  switch i8 %9, label %495 [
 38684|  i8 0, label %541
 38685|  i8 2, label %494
 38686|  ]                                                                                                                     ;L368
 38687| 
 38688| 487: ; preds = %485
 38689|  br label %541                                                                                                         ;L1
 38690| 
 38691| 488: ; preds = %485
 38693|  store i64 0, ptr %16,                                                                                                 ;L391<402
 38694|  %489 = gep %16, i64 8                                                                                                 ;L391<402
 38695|  store i64 1000, ptr %489,                                                                                             ;L391<402
 38696|  %490 = gep %16, i64 16                                                                                                ;L391<402
 38697|  store i8 0, ptr %490,                                                                                                 ;L391<402
 38698|  %491 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %1, ptr %16)                            ;L402
 38700|  %492 = icmp ult i64 %491, %470                                                                                        ;L402
 38701|  %493 = select i1 %492, i64 10, i64 -9999                                                                              ;L402
 38702|  br label %541                                                                                                         ;L402
 38703| 
 38704| 494: ; preds = %486
 38705|  br label %541                                                                                                         ;L1
 38706| 
 38707| 495: ; preds = %486
 38708|  %496 = icmp ugt i64 %88, 29999                                                                                        ;L378
 38709|  br i1 %496, label %498, label %497                                                                                    ;L378
 38710| 
 38711| 497: ; preds = %495
 38712|  br i1 %420, label %510, label %504                                                                                    ;L385
 38713| 
 38714| 498: ; preds = %495
 38716|  store i64 0, ptr %19,                                                                                                 ;L391<379
 38717|  %499 = gep %19, i64 8                                                                                                 ;L391<379
 38718|  store i64 1000, ptr %499,                                                                                             ;L391<379
 38719|  %500 = gep %19, i64 16                                                                                                ;L391<379
 38720|  store i8 0, ptr %500,                                                                                                 ;L391<379
 38721|  %501 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %1, ptr %19)                            ;L379
 38723|  %502 = icmp ult i64 %501, %470                                                                                        ;L379
 38724|  %503 = select i1 %502, i64 10, i64 -9999                                                                              ;L379
 38725|  br label %541                                                                                                         ;L379
 38726| 
 38727| 504: ; preds = %497
 38729|  store i64 0, ptr %17,                                                                                                 ;L391<388
 38730|  %505 = gep %17, i64 8                                                                                                 ;L391<388
 38731|  store i64 1000, ptr %505,                                                                                             ;L391<388
 38732|  %506 = gep %17, i64 16                                                                                                ;L391<388
 38733|  store i8 0, ptr %506,                                                                                                 ;L391<388
 38734|  %507 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %1, ptr %17)                            ;L388
 38736|  %508 = icmp ult i64 %507, %470                                                                                        ;L388
 38737|  %509 = select i1 %508, i64 -9999, i64 10                                                                              ;L388
 38738|  br label %541                                                                                                         ;L388
 38739| 
 38740| 510: ; preds = %497
 38742|  store i64 0, ptr %18,                                                                                                 ;L391<387
 38743|  %511 = gep %18, i64 8                                                                                                 ;L391<387
 38744|  store i64 1000, ptr %511,                                                                                             ;L391<387
 38745|  %512 = gep %18, i64 16                                                                                                ;L391<387
 38746|  store i8 0, ptr %512,                                                                                                 ;L391<387
 38747|  %513 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %1, ptr %18)                            ;L387
 38749|  %514 = icmp ult i64 %513, %470                                                                                        ;L387
 38750|  %515 = select i1 %514, i64 10, i64 -9999                                                                              ;L387
 38751|  br label %541                                                                                                         ;L387
 38752| 
 38753| 516: ; preds = %480
 38754|  %517 = mul nsw i64 %68, 3                                                                                             ;L333
 38755|  %518 = sdiv i64 %517, 10                                                                                              ;L333
 38756|  %519 = add nsw i64 %466, %518                                                                                         ;L333
 38757|  %520 = icmp slt i64 %519, %63                                                                                         ;L333
 38758|  br i1 %520, label %524, label %521                                                                                    ;L333
 38759| 
 38760| 521: ; preds = %516
 38761|  %522 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %1, ptr %20)                            ;L0
 38762|  %523 = icmp ult i64 %522, %470                                                                                        ;L0
 38763|  br i1 %420, label %527, label %525                                                                                    ;L359
 38764| 
 38765| 524: ; preds = %516
 38766|  switch i8 %9, label %530 [
 38767|  i8 0, label %541
 38768|  i8 2, label %529
 38769|  ]                                                                                                                     ;L335
 38770| 
 38771| 525: ; preds = %521
 38772|  %526 = select i1 %523, i64 -9999, i64 20                                                                              ;L362
 38773|  br label %541                                                                                                         ;L362
 38774| 
 38775| 527: ; preds = %521
 38776|  %528 = select i1 %523, i64 -9999, i64 15                                                                              ;L361
 38777|  br label %541                                                                                                         ;L361
 38778| 
 38779| 529: ; preds = %524
 38780|  br label %541                                                                                                         ;L1
 38781| 
 38782| 530: ; preds = %524
 38783|  %531 = icmp ugt i64 %88, 29999                                                                                        ;L345
 38784|  %532 = call i64 @core::ops5range14RangeInclusivejEECshdEBA0ozCnw_7game_ai(ptr %1, ptr %20)                            ;L0
 38785|  %533 = icmp ult i64 %532, %470                                                                                        ;L0
 38786|  br i1 %531, label %535, label %534                                                                                    ;L345
 38787| 
 38788| 534: ; preds = %530
 38789|  br i1 %420, label %539, label %537                                                                                    ;L352
 38790| 
 38791| 535: ; preds = %530
 38792|  %536 = select i1 %533, i64 10, i64 -9999                                                                              ;L346
 38793|  br label %541                                                                                                         ;L346
 38794| 
 38795| 537: ; preds = %534
 38796|  %538 = select i1 %533, i64 -9999, i64 10                                                                              ;L355
 38797|  br label %541                                                                                                         ;L355
 38798| 
 38799| 539: ; preds = %534
 38800|  %540 = select i1 %533, i64 10, i64 -9999                                                                              ;L354
 38801|  br label %541                                                                                                         ;L354
 38802| 
 38803| 541: ; preds = %1070, %539, %537, %535, %529, %527, %525, %524, %510, %504, %498, %494, %488, %487, %486, %485, %480, %375, %372, %353, %351, %349, %343, %339, %338, %337, %336, %333, %275, %174, %168, %158, %152, %134, %116
 38804|  %542 = phi i64 [ 100, %168 ], [ -9999, %524 ], [ 20, %116 ], [ %354, %353 ], [ 70, %152 ], [ 30, %134 ], [ -9999, %275 ], [ %342, %339 ], [ %376, %375 ], [ %374, %372 ], [ %352, %351 ], [ %350, %349 ], [ -9999, %336 ], [ 10, %343 ], [ %335, %333 ], [ 10, %338 ], [ -9999, %337 ], [ -9999, %486 ], [ -9999, %480 ], [ 10, %529 ], [ -9999, %485 ], [ 10, %494 ], [ 10, %487 ], [ 50, %158 ], [ 70, %174 ], [ %1073, %1070 ], [ %509, %504 ], [ %526, %525 ], [ %493, %488 ], [ %515, %510 ], [ %528, %527 ], [ %540, %539 ], [ %536, %535 ], [ %538, %537 ], [ %503, %498 ] ;L0
 38806|  ret i64 %542                                                                                                          ;L518
 38807| 
 38808| 543: ; preds = %455
 38809|     ;; target_id = ptr %453
 38810|     ;; speed = ptr %453
 38811|  %544 = gep %453, i64 80                                                                                               ;L299
 38812|  %545 = load i64, ptr %544, , !!8                                                                                      ;L299
 38813|  %546 = icmp eq i64 %545, %48                                                                                          ;L299
 38814|  br i1 %546, label %553, label %577                                                                                    ;L299
 38815| 
 38816| 547: ; preds = %455
 38817|  %548 = gep %453, i64 248                                                                                              ;L309
 38818|  %549 = load i64, ptr %548, , !!8                                                                                      ;L309
 38819|  %550 = load ptr, ptr %435, , !!8                                                                                      ;L309
 38820|  %551 = call ptr %550(ptr %268, i64 %549)                                                                              ;L309
 38821|  %552 = icmp eq ptr %551, null                                                                                         ;L309
 38822|  br i1 %552, label %577, label %580                                                                                    ;L309
 38823| 
 38824| 553: ; preds = %543
 38825|  %554 = gep %453, i64 248                                                                                              ;L300
 38826|  %555 = load i64, ptr %554, , !!8                                                                                      ;L300
 38827|  %556 = load ptr, ptr %435, , !!8                                                                                      ;L300
 38828|  %557 = call ptr %556(ptr %268, i64 %555)                                                                              ;L300
 38829|  %558 = icmp eq ptr %557, null                                                                                         ;L300
 38830|  br i1 %558, label %577, label %559                                                                                    ;L300
 38831| 
 38832| 559: ; preds = %553
 38833|     ;; caster = ptr %557
 38834|  %560 = gep %453, i64 256                                                                                              ;L301
 38835|  %561 = load i64, ptr %560, , !!8                                                                                      ;L301
 38836|  %562 = gep %453, i64 264                                                                                              ;L301
 38837|  %563 = load i64, ptr %562, , !!8                                                                                      ;L301
 38838|  %564 = call i64 @gc::utils8distance(i64 %561, i64 %563, i64 %437, i64 %439)                                           ;L301
 38839|     ;; dist = i64 %564
 38840|  %565 = call i64 @gc::simulation10projectileNtB5_10Projectile22expected_damage_target(ptr %453, ptr %54, ptr %557, ptr %8) ;L302
 38841|     ;; dmg = i64 %565
 38842|  %566 = gep %453, i64 72                                                                                               ;L303
 38843|  %567 = load i64, ptr %566, , !!8                                                                                      ;L303
 38844|  %568 = icmp eq i64 %567, 0                                                                                            ;L303
 38845|  br i1 %568, label %576, label %569                                                                                    ;L303
 38846| 
 38847| 569: ; preds = %559
 38848|  %570 = udiv i64 %564, %567                                                                                            ;L303
 38849|  %571 = add i64 %570, 5                                                                                                ;L303
 38850|  %572 = icmp ult i64 %78, %571                                                                                         ;L303
 38851|  %573 = select i1 %572, i64 0, i64 %565                                                                                ;L303
 38852|  %574 = add i64 %573, %451                                                                                             ;L303
 38853|     ;; applyed_damage = i64 %574
 38854|  %575 = add i64 %565, %452                                                                                             ;L306
 38855|     ;; expected_damage = i64 %575
 38856|  br label %577                                                                                                         ;L300
 38857| 
 38858| 576: ; preds = %559
 38859|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.129) #29  ;L303
 38860|  unreachable                                                                                                           ;L303
 38861| 
 38862| 577: ; preds = %585, %583, %580, %569, %553, %547, %543
 38863|  %578 = phi i64 [ %451, %553 ], [ %451, %543 ], [ %574, %569 ], [ %587, %585 ], [ %451, %583 ], [ %451, %580 ], [ %451, %547 ] ;L0
 38864|  %579 = phi i64 [ %452, %553 ], [ %452, %543 ], [ %575, %569 ], [ %588, %585 ], [ %452, %583 ], [ %452, %580 ], [ %452, %547 ] ;L0
 38865|     ;; expected_damage = i64 %579
 38866|     ;; applyed_damage = i64 %578
 38867|  br label %450                                                                                                         ;L297
 38868| 
 38869| 580: ; preds = %547
 38870|     ;; caster = ptr %551
 38871|  %581 = gep %453, i64 300                                                                                              ;L310
 38872|  %582 = call zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget16check_projectile(ptr %581, ptr %453, ptr %8)  ;L310
 38873|  br i1 %582, label %583, label %577                                                                                    ;L310
 38874| 
 38875| 583: ; preds = %580
 38876|     ;; mult = i32 %441
 38877|  %584 = call zeroext i1 @gc::simulation10projectileNtB5_10Projectile11is_in_orbit(ptr %453, i64 %437, i64 %439, i64 %449) ;L310
 38878|  br i1 %584, label %585, label %577                                                                                    ;L310
 38879| 
 38880| 585: ; preds = %583
 38881|  %586 = call i64 @gc::simulation10projectileNtB5_10Projectile22expected_damage_target(ptr %453, ptr %54, ptr %551, ptr %8) ;L311
 38882|     ;; dmg = i64 %586
 38883|  %587 = add i64 %586, %451                                                                                             ;L312
 38884|     ;; applyed_damage = i64 %587
 38885|  %588 = add i64 %586, %452                                                                                             ;L313
 38886|     ;; expected_damage = i64 %588
 38887|  br label %577                                                                                                         ;L310
 38888| 
 38889| 589: ; preds = %861, %820, %780, %744, %700, %656
 38890|  %590 = phi i64 [ %801, %820 ], [ %761, %780 ], [ %725, %744 ], [ %684, %700 ], [ %640, %656 ], [ %841, %861 ]
 38891|  %591 = phi i64 [ %800, %820 ], [ %760, %780 ], [ %724, %744 ], [ %683, %700 ], [ %639, %656 ], [ %840, %861 ]
 38892|  %592 = phi i64 [ %821, %820 ], [ %781, %780 ], [ %745, %744 ], [ %701, %700 ], [ %657, %656 ], [ %862, %861 ]
 38893|  %593 = udiv i64 %423, %590                                                                                            ;L0
 38894|  %594 = call i64 @llvm.umax.i64(i64 %593, i64 1)                                                                       ;L1039<0
 38895|  %595 = mul i64 %594, %591                                                                                             ;L0
 38896|  %596 = add i64 %595, %426                                                                                             ;L0
 38897|  br label %597                                                                                                         ;L211
 38898| 
 38899| 597: ; preds = %830, %790, %714, %673, %629, %625, %620, %615, %610, %605, %600, %589, %429
 38900|  %598 = phi i64 [ %425, %429 ], [ %425, %790 ], [ %425, %629 ], [ %425, %600 ], [ %425, %620 ], [ %425, %625 ], [ %425, %673 ], [ %425, %714 ], [ %425, %610 ], [ %425, %830 ], [ %425, %615 ], [ %425, %605 ], [ %592, %589 ] ;L0
 38901|  %599 = phi i64 [ %426, %429 ], [ %426, %790 ], [ %426, %629 ], [ %426, %600 ], [ %426, %620 ], [ %426, %625 ], [ %426, %673 ], [ %426, %714 ], [ %426, %610 ], [ %426, %830 ], [ %426, %615 ], [ %426, %605 ], [ %596, %589 ] ;L0
 38902|     ;; expected_damage = i64 %599
 38903|     ;; applyed_damage = i64 %598
 38904|  br label %424                                                                                                         ;L211
 38905| 
 38906| 600: ; preds = %429
 38907|     ;; info = ptr %427
 38908|     ;; self = ptr %427
 38910|  %601 = gep %427, i64 112                                                                                              ;L2439<214
 38911|  %602 = gep %427, i64 136                                                                                              ;L2439<214
 38912|  %603 = load i64, ptr %602, , !!8                                                                                      ;L2439<214
 38913|  %604 = trunc nuw i64 %603 to i1                                                                                       ;L2439<214
 38914|  br i1 %604, label %629, label %597                                                                                    ;L2439<214
 38915| 
 38916| 605: ; preds = %429
 38917|     ;; info = ptr %427
 38918|  %606 = gep %427, i64 112                                                                                              ;L228
 38919|  %607 = gep %427, i64 136                                                                                              ;L228
 38920|  %608 = load i64, ptr %607,                                                                                            ;L228
 38921|     ;; self[0..+8] = i64 %608
 38924|  %609 = trunc nuw i64 %608 to i1                                                                                       ;L1161<228
 38925|  br i1 %609, label %673, label %597                                                                                    ;L1161<228
 38926| 
 38927| 610: ; preds = %429
 38928|     ;; info = ptr %427
 38929|     ;; self = ptr %427
 38931|  %611 = gep %427, i64 112                                                                                              ;L2439<241
 38932|  %612 = gep %427, i64 136                                                                                              ;L2439<241
 38933|  %613 = load i64, ptr %612, , !!8                                                                                      ;L2439<241
 38934|  %614 = trunc nuw i64 %613 to i1                                                                                       ;L2439<241
 38935|  br i1 %614, label %714, label %597                                                                                    ;L2439<241
 38936| 
 38937| 615: ; preds = %429
 38938|     ;; info = ptr %427
 38939|  %616 = gep %427, i64 112                                                                                              ;L280
 38940|  %617 = gep %427, i64 256                                                                                              ;L280
 38941|  %618 = load i64, ptr %617, , !!8                                                                                      ;L280
 38942|  %619 = icmp eq i64 %618, %48                                                                                          ;L280
 38943|  br i1 %619, label %754, label %597                                                                                    ;L280
 38944| 
 38945| 620: ; preds = %429
 38946|     ;; info = ptr %427
 38947|     ;; self = ptr %427
 38949|  %621 = gep %427, i64 112                                                                                              ;L2439<254
 38950|  %622 = gep %427, i64 136                                                                                              ;L2439<254
 38951|  %623 = load i64, ptr %622, , !!8                                                                                      ;L2439<254
 38952|  %624 = trunc nuw i64 %623 to i1                                                                                       ;L2439<254
 38953|  br i1 %624, label %790, label %597                                                                                    ;L2439<254
 38954| 
 38955| 625: ; preds = %429
 38956|     ;; info = ptr %427
 38957|     ;; self = ptr %427
 38959|  %626 = gep %427, i64 112                                                                                              ;L2439<267
 38960|  %627 = load i64, ptr %626, , !!8                                                                                      ;L2439<267
 38961|  %628 = trunc nuw i64 %627 to i1                                                                                       ;L2439<267
 38962|  br i1 %628, label %830, label %597                                                                                    ;L2439<267
 38963| 
 38964| 629: ; preds = %600
 38965|  %630 = gep %427, i64 144                                                                                              ;L2439<214
 38966|     ;; l = ptr %427
 38967|     ;; self = ptr %427
 38970|  %631 = load i64, ptr %630, , !!8                                                                                      ;L1878<2440<214
 38971|  %632 = icmp eq i64 %631, %48                                                                                          ;L1878<2440<214
 38972|  br i1 %632, label %633, label %597                                                                                    ;L214
 38973| 
 38974| 633: ; preds = %629
 38975|     ;; self = ptr %427
 38976|  %634 = gep %427, i64 1216                                                                                             ;L742<215
 38977|  %635 = load i32, ptr %634, , !!8                                                                                      ;L742<215
 38978|  %636 = icmp eq i32 %635, -1                                                                                           ;L742<215
 38979|  br i1 %636, label %645, label %637                                                                                    ;L742<215
 38980| 
 38981| 637: ; preds = %633
 38982|  %638 = gep %427, i64 1168                                                                                             ;L742<215
 38983|     ;; self = ptr %638
 38984|  %639 = call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %638, ptr %54, ptr %427, ptr @anon.fdc8f8a40baf2989242b37eb35661f10.17, ptr %8) ;L215
 38985|     ;; dmg = i64 %639
 38986|  %640 = call i64 @gc::simulation6entityNtB5_6Entity15attack_cooltime(ptr %427)                                         ;L216
 38987|     ;; attack_cooltime = i64 %640
 38988|     ;; self = ptr %427
 38989|     ;; self = ptr %638
 38990|  %641 = gep %427, i64 1200                                                                                             ;L217
 38991|  %642 = load i64, ptr %641, , !!8                                                                                      ;L217
 38992|  %643 = call i64 @gc::simulation6entityNtB5_6Entity17attack_speed_mult(ptr %427)                                       ;L217
 38993|  %644 = icmp eq i64 %643, 0                                                                                            ;L217
 38994|  br i1 %644, label %651, label %646                                                                                    ;L217
 38995| 
 38996| 645: ; preds = %633
 38997|     ;; self = ptr null
 38998|  call void @core::option13unwrap_failed(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.130) #29                            ;L1013<215
 38999|  unreachable                                                                                                           ;L1013<215
 39000| 
 39001| 646: ; preds = %637
 39002|  %647 = mul i64 %642, 100                                                                                              ;L217
 39003|  %648 = udiv i64 %647, %643                                                                                            ;L217
 39004|     ;; attack_start_timing = i64 %648
 39005|     ;; self = i64 %648
 39006|  %649 = load i64, ptr %601, , !!8                                                                                      ;L218
 39007|  %650 = trunc nuw i64 %649 to i1                                                                                       ;L218
 39008|  br i1 %650, label %652, label %656                                                                                    ;L218
 39009| 
 39010| 651: ; preds = %637
 39011|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.131) #29  ;L217
 39012|  unreachable                                                                                                           ;L217
 39013| 
 39014| 652: ; preds = %646
 39015|     ;; target_id = ptr %427
 39016|     ;; time = ptr %427
 39017|  %653 = gep %427, i64 280                                                                                              ;L219
 39018|  %654 = load i8, ptr %653, , !!8                                                                                       ;L219
 39019|  %655 = trunc nuw i8 %654 to i1                                                                                        ;L219
 39020|  br i1 %655, label %656, label %659                                                                                    ;L219
 39021| 
 39022| 656: ; preds = %666, %659, %652, %646
 39023|  %657 = phi i64 [ %425, %652 ], [ %425, %646 ], [ %671, %666 ], [ %425, %659 ]                                         ;L0
 39024|     ;; applyed_damage = i64 %657
 39025|  %658 = icmp eq i64 %640, 0                                                                                            ;L224
 39026|  br i1 %658, label %672, label %589                                                                                    ;L224
 39027| 
 39028| 659: ; preds = %652
 39029|  %660 = gep %427, i64 128                                                                                              ;L219
 39030|  %661 = load i64, ptr %660, , !!8                                                                                      ;L219
 39031|     ;; rhs = i64 %661
 39032|  %662 = icmp ult i64 %661, %648                                                                                        ;L219
 39033|  %663 = call i64 @llvm.usub.sat.i64(i64 %648, i64 %661)
 39034|  %664 = icmp ult i64 %663, %78                                                                                         ;L219
 39035|  %665 = and i1 %662, %664                                                                                              ;L219
 39036|  br i1 %665, label %666, label %656                                                                                    ;L219
 39037| 
 39038| 666: ; preds = %659
 39039|  %667 = gep %427, i64 120                                                                                              ;L219
 39040|  %668 = load i64, ptr %667, , !!8                                                                                      ;L219
 39041|  %669 = icmp eq i64 %668, %48                                                                                          ;L219
 39042|  %670 = select i1 %669, i64 %639, i64 0                                                                                ;L219
 39043|  %671 = add i64 %670, %425                                                                                             ;L219
 39044|  br label %656                                                                                                         ;L219
 39045| 
 39046| 672: ; preds = %656
 39047|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.132) #29  ;L224
 39048|  unreachable                                                                                                           ;L224
 39049| 
 39050| 673: ; preds = %605
 39051|  %674 = gep %427, i64 152                                                                                              ;L228
 39052|  %675 = load i64, ptr %674,                                                                                            ;L228
 39053|     ;; self[16..+8] = i64 %675
 39054|     ;; self = ptr undef
 39056|     ;; l = ptr undef
 39057|     ;; self = ptr undef
 39060|  %676 = icmp eq i64 %675, %48                                                                                          ;L1878<2440<228
 39061|  br i1 %676, label %677, label %597                                                                                    ;L228
 39062| 
 39063| 677: ; preds = %673
 39064|     ;; self = ptr %427
 39065|  %678 = gep %427, i64 1216                                                                                             ;L742<229
 39066|  %679 = load i32, ptr %678, , !!8                                                                                      ;L742<229
 39067|  %680 = icmp eq i32 %679, -1                                                                                           ;L742<229
 39068|  br i1 %680, label %689, label %681                                                                                    ;L742<229
 39069| 
 39070| 681: ; preds = %677
 39071|  %682 = gep %427, i64 1168                                                                                             ;L742<229
 39072|     ;; self = ptr %682
 39073|  %683 = call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %682, ptr %54, ptr %427, ptr @anon.fdc8f8a40baf2989242b37eb35661f10.17, ptr %8) ;L229
 39074|     ;; dmg = i64 %683
 39075|  %684 = call i64 @gc::simulation6entityNtB5_6Entity15attack_cooltime(ptr %427)                                         ;L230
 39076|     ;; attack_cooltime = i64 %684
 39077|     ;; self = ptr %427
 39078|     ;; self = ptr %682
 39079|  %685 = gep %427, i64 1200                                                                                             ;L231
 39080|  %686 = load i64, ptr %685, , !!8                                                                                      ;L231
 39081|  %687 = call i64 @gc::simulation6entityNtB5_6Entity17attack_speed_mult(ptr %427)                                       ;L231
 39082|  %688 = icmp eq i64 %687, 0                                                                                            ;L231
 39083|  br i1 %688, label %695, label %690                                                                                    ;L231
 39084| 
 39085| 689: ; preds = %677
 39086|     ;; self = ptr null
 39087|  call void @core::option13unwrap_failed(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.133) #29                            ;L1013<229
 39088|  unreachable                                                                                                           ;L1013<229
 39089| 
 39090| 690: ; preds = %681
 39091|  %691 = mul i64 %686, 100                                                                                              ;L231
 39092|  %692 = udiv i64 %691, %687                                                                                            ;L231
 39093|     ;; attack_start_timing = i64 %692
 39094|     ;; self = i64 %692
 39095|  %693 = load i64, ptr %606, , !!8                                                                                      ;L232
 39096|  %694 = trunc nuw i64 %693 to i1                                                                                       ;L232
 39097|  br i1 %694, label %696, label %700                                                                                    ;L232
 39098| 
 39099| 695: ; preds = %681
 39100|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.134) #29  ;L231
 39101|  unreachable                                                                                                           ;L231
 39102| 
 39103| 696: ; preds = %690
 39104|     ;; target_id = ptr %427
 39105|     ;; time = ptr %427
 39106|  %697 = gep %427, i64 128                                                                                              ;L233
 39107|  %698 = load i64, ptr %697, , !!8                                                                                      ;L233
 39108|     ;; rhs = i64 %698
 39109|  %699 = icmp ult i64 %698, %692                                                                                        ;L233
 39110|  br i1 %699, label %703, label %700                                                                                    ;L233
 39111| 
 39112| 700: ; preds = %707, %703, %696, %690
 39113|  %701 = phi i64 [ %425, %690 ], [ %712, %707 ], [ %425, %703 ], [ %425, %696 ]                                         ;L0
 39114|     ;; applyed_damage = i64 %701
 39115|  %702 = icmp eq i64 %684, 0                                                                                            ;L237
 39116|  br i1 %702, label %713, label %589                                                                                    ;L237
 39117| 
 39118| 703: ; preds = %696
 39119|  %704 = add i64 %692, 15                                                                                               ;L2472<233
 39120|  %705 = sub i64 %704, %698                                                                                             ;L233
 39121|  %706 = icmp ult i64 %705, %78                                                                                         ;L233
 39122|  br i1 %706, label %707, label %700                                                                                    ;L233
 39123| 
 39124| 707: ; preds = %703
 39125|  %708 = gep %427, i64 120                                                                                              ;L233
 39126|  %709 = load i64, ptr %708, , !!8                                                                                      ;L233
 39127|  %710 = icmp eq i64 %709, %48                                                                                          ;L233
 39128|  %711 = select i1 %710, i64 %683, i64 0                                                                                ;L233
 39129|  %712 = add i64 %711, %425                                                                                             ;L233
 39130|  br label %700                                                                                                         ;L233
 39131| 
 39132| 713: ; preds = %700
 39133|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.135) #29  ;L237
 39134|  unreachable                                                                                                           ;L237
 39135| 
 39136| 714: ; preds = %610
 39137|  %715 = gep %427, i64 144                                                                                              ;L2439<241
 39138|     ;; l = ptr %427
 39139|     ;; self = ptr %427
 39142|  %716 = load i64, ptr %715, , !!8                                                                                      ;L1878<2440<241
 39143|  %717 = icmp eq i64 %716, %48                                                                                          ;L1878<2440<241
 39144|  br i1 %717, label %718, label %597                                                                                    ;L241
 39145| 
 39146| 718: ; preds = %714
 39147|     ;; self = ptr %427
 39148|  %719 = gep %427, i64 1216                                                                                             ;L742<242
 39149|  %720 = load i32, ptr %719, , !!8                                                                                      ;L742<242
 39150|  %721 = icmp eq i32 %720, -1                                                                                           ;L742<242
 39151|  br i1 %721, label %730, label %722                                                                                    ;L742<242
 39152| 
 39153| 722: ; preds = %718
 39154|  %723 = gep %427, i64 1168                                                                                             ;L742<242
 39155|     ;; self = ptr %723
 39156|  %724 = call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %723, ptr %54, ptr %427, ptr @anon.fdc8f8a40baf2989242b37eb35661f10.17, ptr %8) ;L242
 39157|     ;; dmg = i64 %724
 39158|  %725 = call i64 @gc::simulation6entityNtB5_6Entity15attack_cooltime(ptr %427)                                         ;L243
 39159|     ;; attack_cooltime = i64 %725
 39160|     ;; self = ptr %427
 39161|     ;; self = ptr %723
 39162|  %726 = gep %427, i64 1200                                                                                             ;L244
 39163|  %727 = load i64, ptr %726, , !!8                                                                                      ;L244
 39164|  %728 = call i64 @gc::simulation6entityNtB5_6Entity17attack_speed_mult(ptr %427)                                       ;L244
 39165|  %729 = icmp eq i64 %728, 0                                                                                            ;L244
 39166|  br i1 %729, label %734, label %731                                                                                    ;L244
 39167| 
 39168| 730: ; preds = %718
 39169|     ;; self = ptr null
 39170|  call void @core::option13unwrap_failed(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.136) #29                            ;L1013<242
 39171|  unreachable                                                                                                           ;L1013<242
 39172| 
 39173| 731: ; preds = %722
 39176|  %732 = load i64, ptr %611, , !!8                                                                                      ;L245
 39177|  %733 = trunc nuw i64 %732 to i1                                                                                       ;L245
 39178|  br i1 %733, label %735, label %744                                                                                    ;L245
 39179| 
 39180| 734: ; preds = %722
 39181|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.137) #29  ;L244
 39182|  unreachable                                                                                                           ;L244
 39183| 
 39184| 735: ; preds = %731
 39185|  %736 = mul i64 %727, 100                                                                                              ;L244
 39186|  %737 = udiv i64 %736, %728                                                                                            ;L244
 39187|     ;; attack_start_timing = i64 %737
 39188|     ;; self = i64 %737
 39189|     ;; target_id = ptr %427
 39190|     ;; time = ptr %427
 39191|  %738 = gep %427, i64 128                                                                                              ;L246
 39192|  %739 = load i64, ptr %738, , !!8                                                                                      ;L246
 39193|     ;; rhs = i64 %739
 39194|  %740 = icmp ult i64 %739, %737                                                                                        ;L246
 39195|  %741 = call i64 @llvm.usub.sat.i64(i64 %737, i64 %739)
 39196|  %742 = icmp ult i64 %741, %78                                                                                         ;L246
 39197|  %743 = and i1 %740, %742                                                                                              ;L246
 39198|  br i1 %743, label %747, label %744                                                                                    ;L246
 39199| 
 39200| 744: ; preds = %747, %735, %731
 39201|  %745 = phi i64 [ %425, %731 ], [ %752, %747 ], [ %425, %735 ]                                                         ;L0
 39202|     ;; applyed_damage = i64 %745
 39203|  %746 = icmp eq i64 %725, 0                                                                                            ;L250
 39204|  br i1 %746, label %753, label %589                                                                                    ;L250
 39205| 
 39206| 747: ; preds = %735
 39207|  %748 = gep %427, i64 120                                                                                              ;L246
 39208|  %749 = load i64, ptr %748, , !!8                                                                                      ;L246
 39209|  %750 = icmp eq i64 %749, %48                                                                                          ;L246
 39210|  %751 = select i1 %750, i64 %724, i64 0                                                                                ;L246
 39211|  %752 = add i64 %751, %425                                                                                             ;L246
 39212|  br label %744                                                                                                         ;L246
 39213| 
 39214| 753: ; preds = %744
 39215|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.138) #29  ;L250
 39216|  unreachable                                                                                                           ;L250
 39217| 
 39218| 754: ; preds = %615
 39219|     ;; self = ptr %427
 39220|  %755 = gep %427, i64 1216                                                                                             ;L742<281
 39221|  %756 = load i32, ptr %755, , !!8                                                                                      ;L742<281
 39222|  %757 = icmp eq i32 %756, -1                                                                                           ;L742<281
 39223|  br i1 %757, label %766, label %758                                                                                    ;L742<281
 39224| 
 39225| 758: ; preds = %754
 39226|  %759 = gep %427, i64 1168                                                                                             ;L742<281
 39227|     ;; self = ptr %759
 39228|  %760 = call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %759, ptr %54, ptr %427, ptr @anon.fdc8f8a40baf2989242b37eb35661f10.17, ptr %8) ;L281
 39229|     ;; dmg = i64 %760
 39230|  %761 = call i64 @gc::simulation6entityNtB5_6Entity15attack_cooltime(ptr %427)                                         ;L282
 39231|     ;; attack_cooltime = i64 %761
 39232|     ;; self = ptr %427
 39233|     ;; self = ptr %759
 39234|  %762 = gep %427, i64 1200                                                                                             ;L283
 39235|  %763 = load i64, ptr %762, , !!8                                                                                      ;L283
 39236|  %764 = call i64 @gc::simulation6entityNtB5_6Entity17attack_speed_mult(ptr %427)                                       ;L283
 39237|  %765 = icmp eq i64 %764, 0                                                                                            ;L283
 39238|  br i1 %765, label %770, label %767                                                                                    ;L283
 39239| 
 39240| 766: ; preds = %754
 39241|     ;; self = ptr null
 39242|  call void @core::option13unwrap_failed(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.139) #29                            ;L1013<281
 39243|  unreachable                                                                                                           ;L1013<281
 39244| 
 39245| 767: ; preds = %758
 39248|  %768 = load i64, ptr %616, , !!8                                                                                      ;L284
 39249|  %769 = trunc nuw i64 %768 to i1                                                                                       ;L284
 39250|  br i1 %769, label %771, label %780                                                                                    ;L284
 39251| 
 39252| 770: ; preds = %758
 39253|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.140) #29  ;L283
 39254|  unreachable                                                                                                           ;L283
 39255| 
 39256| 771: ; preds = %767
 39257|  %772 = mul i64 %763, 100                                                                                              ;L283
 39258|  %773 = udiv i64 %772, %764                                                                                            ;L283
 39259|     ;; attack_start_timing = i64 %773
 39260|     ;; self = i64 %773
 39261|     ;; target_id = ptr %427
 39262|     ;; time = ptr %427
 39263|  %774 = gep %427, i64 128                                                                                              ;L285
 39264|  %775 = load i64, ptr %774, , !!8                                                                                      ;L285
 39265|     ;; rhs = i64 %775
 39266|  %776 = icmp ult i64 %775, %773                                                                                        ;L285
 39267|  %777 = call i64 @llvm.usub.sat.i64(i64 %773, i64 %775)
 39268|  %778 = icmp ult i64 %777, %78                                                                                         ;L285
 39269|  %779 = and i1 %776, %778                                                                                              ;L285
 39270|  br i1 %779, label %783, label %780                                                                                    ;L285
 39271| 
 39272| 780: ; preds = %783, %771, %767
 39273|  %781 = phi i64 [ %425, %767 ], [ %788, %783 ], [ %425, %771 ]                                                         ;L0
 39274|     ;; applyed_damage = i64 %781
 39275|  %782 = icmp eq i64 %761, 0                                                                                            ;L289
 39276|  br i1 %782, label %789, label %589                                                                                    ;L289
 39277| 
 39278| 783: ; preds = %771
 39279|  %784 = gep %427, i64 120                                                                                              ;L285
 39280|  %785 = load i64, ptr %784, , !!8                                                                                      ;L285
 39281|  %786 = icmp eq i64 %785, %48                                                                                          ;L285
 39282|  %787 = select i1 %786, i64 %760, i64 0                                                                                ;L285
 39283|  %788 = add i64 %787, %425                                                                                             ;L285
 39284|  br label %780                                                                                                         ;L285
 39285| 
 39286| 789: ; preds = %780
 39287|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.141) #29  ;L289
 39288|  unreachable                                                                                                           ;L289
 39289| 
 39290| 790: ; preds = %620
 39291|  %791 = gep %427, i64 144                                                                                              ;L2439<254
 39292|     ;; l = ptr %427
 39293|     ;; self = ptr %427
 39296|  %792 = load i64, ptr %791, , !!8                                                                                      ;L1878<2440<254
 39297|  %793 = icmp eq i64 %792, %48                                                                                          ;L1878<2440<254
 39298|  br i1 %793, label %794, label %597                                                                                    ;L254
 39299| 
 39300| 794: ; preds = %790
 39301|     ;; self = ptr %427
 39302|  %795 = gep %427, i64 1216                                                                                             ;L742<255
 39303|  %796 = load i32, ptr %795, , !!8                                                                                      ;L742<255
 39304|  %797 = icmp eq i32 %796, -1                                                                                           ;L742<255
 39305|  br i1 %797, label %806, label %798                                                                                    ;L742<255
 39306| 
 39307| 798: ; preds = %794
 39308|  %799 = gep %427, i64 1168                                                                                             ;L742<255
 39309|     ;; self = ptr %799
 39310|  %800 = call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %799, ptr %54, ptr %427, ptr @anon.fdc8f8a40baf2989242b37eb35661f10.17, ptr %8) ;L255
 39311|     ;; dmg = i64 %800
 39312|  %801 = call i64 @gc::simulation6entityNtB5_6Entity15attack_cooltime(ptr %427)                                         ;L256
 39313|     ;; attack_cooltime = i64 %801
 39314|     ;; self = ptr %427
 39315|     ;; self = ptr %799
 39316|  %802 = gep %427, i64 1200                                                                                             ;L257
 39317|  %803 = load i64, ptr %802, , !!8                                                                                      ;L257
 39318|  %804 = call i64 @gc::simulation6entityNtB5_6Entity17attack_speed_mult(ptr %427)                                       ;L257
 39319|  %805 = icmp eq i64 %804, 0                                                                                            ;L257
 39320|  br i1 %805, label %810, label %807                                                                                    ;L257
 39321| 
 39322| 806: ; preds = %794
 39323|     ;; self = ptr null
 39324|  call void @core::option13unwrap_failed(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.142) #29                            ;L1013<255
 39325|  unreachable                                                                                                           ;L1013<255
 39326| 
 39327| 807: ; preds = %798
 39330|  %808 = load i64, ptr %621, , !!8                                                                                      ;L258
 39331|  %809 = trunc nuw i64 %808 to i1                                                                                       ;L258
 39332|  br i1 %809, label %811, label %820                                                                                    ;L258
 39333| 
 39334| 810: ; preds = %798
 39335|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.143) #29  ;L257
 39336|  unreachable                                                                                                           ;L257
 39337| 
 39338| 811: ; preds = %807
 39339|  %812 = mul i64 %803, 100                                                                                              ;L257
 39340|  %813 = udiv i64 %812, %804                                                                                            ;L257
 39341|     ;; attack_start_timing = i64 %813
 39342|     ;; self = i64 %813
 39343|     ;; target_id = ptr %427
 39344|     ;; time = ptr %427
 39345|  %814 = gep %427, i64 128                                                                                              ;L259
 39346|  %815 = load i64, ptr %814, , !!8                                                                                      ;L259
 39347|     ;; rhs = i64 %815
 39348|  %816 = icmp ult i64 %815, %813                                                                                        ;L259
 39349|  %817 = call i64 @llvm.usub.sat.i64(i64 %813, i64 %815)
 39350|  %818 = icmp ult i64 %817, %78                                                                                         ;L259
 39351|  %819 = and i1 %816, %818                                                                                              ;L259
 39352|  br i1 %819, label %823, label %820                                                                                    ;L259
 39353| 
 39354| 820: ; preds = %823, %811, %807
 39355|  %821 = phi i64 [ %425, %807 ], [ %828, %823 ], [ %425, %811 ]                                                         ;L0
 39356|     ;; applyed_damage = i64 %821
 39357|  %822 = icmp eq i64 %801, 0                                                                                            ;L263
 39358|  br i1 %822, label %829, label %589                                                                                    ;L263
 39359| 
 39360| 823: ; preds = %811
 39361|  %824 = gep %427, i64 120                                                                                              ;L259
 39362|  %825 = load i64, ptr %824, , !!8                                                                                      ;L259
 39363|  %826 = icmp eq i64 %825, %48                                                                                          ;L259
 39364|  %827 = select i1 %826, i64 %800, i64 0                                                                                ;L259
 39365|  %828 = add i64 %827, %425                                                                                             ;L259
 39366|  br label %820                                                                                                         ;L259
 39367| 
 39368| 829: ; preds = %820
 39369|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.144) #29  ;L263
 39370|  unreachable                                                                                                           ;L263
 39371| 
 39372| 830: ; preds = %625
 39373|  %831 = gep %427, i64 120                                                                                              ;L2439<267
 39374|     ;; l = ptr %427
 39375|     ;; self = ptr %427
 39378|  %832 = load i64, ptr %831, , !!8                                                                                      ;L1878<2440<267
 39379|  %833 = icmp eq i64 %832, %48                                                                                          ;L1878<2440<267
 39380|  br i1 %833, label %834, label %597                                                                                    ;L267
 39381| 
 39382| 834: ; preds = %830
 39383|     ;; self = ptr %427
 39384|  %835 = gep %427, i64 1216                                                                                             ;L742<268
 39385|  %836 = load i32, ptr %835, , !!8                                                                                      ;L742<268
 39386|  %837 = icmp eq i32 %836, -1                                                                                           ;L742<268
 39387|  br i1 %837, label %846, label %838                                                                                    ;L742<268
 39388| 
 39389| 838: ; preds = %834
 39390|  %839 = gep %427, i64 1168                                                                                             ;L742<268
 39391|     ;; self = ptr %839
 39392|  %840 = call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %839, ptr %54, ptr %427, ptr @anon.fdc8f8a40baf2989242b37eb35661f10.17, ptr %8) ;L268
 39393|     ;; dmg = i64 %840
 39394|  %841 = call i64 @gc::simulation6entityNtB5_6Entity15attack_cooltime(ptr %427)                                         ;L269
 39395|     ;; attack_cooltime = i64 %841
 39396|     ;; self = ptr %427
 39397|     ;; self = ptr %839
 39398|  %842 = gep %427, i64 1200                                                                                             ;L270
 39399|  %843 = load i64, ptr %842, , !!8                                                                                      ;L270
 39400|  %844 = call i64 @gc::simulation6entityNtB5_6Entity17attack_speed_mult(ptr %427)                                       ;L270
 39401|  %845 = icmp eq i64 %844, 0                                                                                            ;L270
 39402|  br i1 %845, label %851, label %847                                                                                    ;L270
 39403| 
 39404| 846: ; preds = %834
 39405|     ;; self = ptr null
 39406|  call void @core::option13unwrap_failed(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.145) #29                            ;L1013<268
 39407|  unreachable                                                                                                           ;L1013<268
 39408| 
 39409| 847: ; preds = %838
 39412|  %848 = gep %427, i64 176                                                                                              ;L271
 39413|  %849 = load i8, ptr %848, , !!8                                                                                       ;L271
 39414|  %850 = icmp eq i8 %849, 1                                                                                             ;L271
 39415|  br i1 %850, label %852, label %861                                                                                    ;L271
 39416| 
 39417| 851: ; preds = %838
 39418|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.146) #29  ;L270
 39419|  unreachable                                                                                                           ;L270
 39420| 
 39421| 852: ; preds = %847
 39422|  %853 = mul i64 %843, 100                                                                                              ;L270
 39423|  %854 = udiv i64 %853, %844                                                                                            ;L270
 39424|     ;; attack_start_timing = i64 %854
 39425|     ;; self = i64 %854
 39426|     ;; target_id = ptr %427
 39427|     ;; time = ptr %427
 39428|  %855 = gep %427, i64 192                                                                                              ;L272
 39429|  %856 = load i64, ptr %855, , !!8                                                                                      ;L272
 39430|     ;; rhs = i64 %856
 39431|  %857 = icmp ult i64 %856, %854                                                                                        ;L272
 39432|  %858 = call i64 @llvm.usub.sat.i64(i64 %854, i64 %856)
 39433|  %859 = icmp ult i64 %858, %78                                                                                         ;L272
 39434|  %860 = and i1 %857, %859                                                                                              ;L272
 39435|  br i1 %860, label %864, label %861                                                                                    ;L272
 39436| 
 39437| 861: ; preds = %864, %852, %847
 39438|  %862 = phi i64 [ %425, %847 ], [ %869, %864 ], [ %425, %852 ]                                                         ;L0
 39439|     ;; applyed_damage = i64 %862
 39440|  %863 = icmp eq i64 %841, 0                                                                                            ;L276
 39441|  br i1 %863, label %870, label %589                                                                                    ;L276
 39442| 
 39443| 864: ; preds = %852
 39444|  %865 = gep %427, i64 184                                                                                              ;L272
 39445|  %866 = load i64, ptr %865, , !!8                                                                                      ;L272
 39446|  %867 = icmp eq i64 %866, %48                                                                                          ;L272
 39447|  %868 = select i1 %867, i64 %840, i64 0                                                                                ;L272
 39448|  %869 = add i64 %868, %425                                                                                             ;L272
 39449|  br label %861                                                                                                         ;L272
 39450| 
 39451| 870: ; preds = %861
 39452|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.147) #29  ;L276
 39453|  unreachable                                                                                                           ;L276
 39454| 
 39455| 871: ; preds = %71
 39456|     ;; self = ptr %8
 39457|     ;; self = ptr %8
 39458|     ;; other = ptr %39
 39459|     ;; other = ptr %39
 39460|  %872 = load i64, ptr %8, , !!8                                                                                        ;L1127<264<414
 39461|  %873 = gep %8, i64 8                                                                                                  ;L1127<264<414
 39462|     ;; __self_discr = i64 %872
 39463|  %874 = load i64, ptr %39, , !!8                                                                                       ;L1127<264<414
 39464|  %875 = gep %39, i64 8                                                                                                 ;L1127<264<414
 39465|     ;; __arg1_discr = i64 %874
 39466|  %876 = icmp eq i64 %872, %874                                                                                         ;L1127<264<414
 39467|  br i1 %876, label %877, label %879                                                                                    ;L1127<264<414
 39468| 
 39469| 877: ; preds = %871
 39470|  %878 = icmp eq i64 %872, 0                                                                                            ;L1127<264<414
 39471|  br i1 %878, label %882, label %919                                                                                    ;L1127<264<414
 39472| 
 39473| 879: ; preds = %882, %871
 39474|  %880 = call fastcc ptr @gc::simulationNtB5_21AbstractGameWithCache21player_by_champion_id(ptr %35, i64 %48)           ;L416
 39475|  %881 = icmp eq ptr %880, null                                                                                         ;L416
 39476|  br i1 %881, label %919, label %886                                                                                    ;L416
 39477| 
 39478| 882: ; preds = %877
 39479|     ;; __self_0 = ptr %8
 39480|     ;; self = ptr %8
 39481|     ;; __arg1_0 = ptr %39
 39482|     ;; other = ptr %39
 39485|  %883 = load i64, ptr %873, , !!8                                                                                      ;L1878<2123<1127<264<414
 39486|  %884 = load i64, ptr %875, , !!8                                                                                      ;L1878<2123<1127<264<414
 39487|  %885 = icmp eq i64 %883, %884                                                                                         ;L1878<2123<1127<264<414
 39488|     ;; bonus_applicable = i1 %885
 39489|  br i1 %885, label %919, label %879                                                                                    ;L415
 39490| 
 39491| 886: ; preds = %879
 39492|     ;; eplayer = ptr %880
 39493|  %887 = sub nuw nsw i64 1, %29                                                                                         ;L417
 39494|     ;; self = ptr %880
 39495|  %888 = gep %880, i64 2496                                                                                             ;L581<417
 39496|  %889 = load i32, ptr %888, , !!8                                                                                      ;L581<417
 39497|  %890 = zext nneg i32 %889 to i64                                                                                      ;L581<417
 39498|  %891 = gep %3, i64 16                                                                                                 ;L417
 39499|  %892 = load ptr, ptr %891, , !!8, !!8                                                                                 ;L417
 39500|  %893 = getelementptr { { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, [5 x { i64, [2 x i64] }], [5 x { i64, { i8, [23 x i8] } }], [5 x { i8, [7 x i8], i64 }], [5 x i64], [5 x i64], [5 x i64], [5 x { i64, i64 }], i64, i64, i64, i64, i64, [5 x i32], i8, [3 x i8] }, ptr %892, i64 %887 ;L417
 39501|  %894 = gep %893, i64 120                                                                                              ;L417
 39502|  %895 = gepS %894, i64 %890                                                                                            ;L417
 39503|  %896 = load i64, ptr %895, , !!8                                                                                      ;L417
 39504|     ;; action[0..+8] = i64 %896
 39507|  %897 = add nsw i64 %896, -6                                                                                           ;L417
 39508|  %898 = icmp ult i64 %897, 3                                                                                           ;L417
 39509|  br i1 %898, label %899, label %919                                                                                    ;L417
 39510| 
 39511| 899: ; preds = %886
 39512|  %900 = gep %895, i64 8                                                                                                ;L417
 39513|  %901 = load i64, ptr %900,                                                                                            ;L417
 39514|     ;; action[8..+8] = i64 %901
 39515|     ;; target_id = i64 %901
 39516|  %902 = load ptr, ptr %35, , !!8, !!8                                                                                  ;L419
 39517|  %903 = gep %35, i64 8                                                                                                 ;L419
 39518|  %904 = load ptr, ptr %903, , !!8, !!8                                                                                 ;L419
 39519|  %905 = gep %904, i64 496                                                                                              ;L419
 39520|  %906 = load ptr, ptr %905, , !!8                                                                                      ;L419
 39521|  %907 = call ptr %906(ptr %902, i64 %901)                                                                              ;L419
 39522|  %908 = icmp eq ptr %907, null                                                                                         ;L419
 39523|  br i1 %908, label %919, label %909                                                                                    ;L419
 39524| 
 39525| 909: ; preds = %899
 39526|     ;; t = ptr %907
 39527|     ;; self = ptr %907
 39528|  %910 = gep %907, i64 104                                                                                              ;L1386<420
 39529|  %911 = load i64, ptr %910, , !!8                                                                                      ;L1386<420
 39530|  switch i64 %911, label %919 [
 39531|  i64 2, label %912
 39532|  i64 3, label %918
 39533|  ]                                                                                                                     ;L420
 39534| 
 39535| 912: ; preds = %909
 39536|     ;; bonus = i64 10
 39537|     ;; self = ptr %907
 39538|     ;; self = ptr %907
 39539|  %913 = gep %907, i64 296                                                                                              ;L91<1313<424
 39540|  %914 = load i8, ptr %913, , !!8                                                                                       ;L91<1313<424
 39541|  %915 = add nsw i8 %914, -3                                                                                            ;L91<1313<424
 39542|  %916 = icmp ult i8 %915, 2                                                                                            ;L91<1313<424
 39543|  %917 = select i1 %916, i64 80, i64 10                                                                                 ;L91<1313<424
 39544|  br label %919                                                                                                         ;L91<1313<424
 39545| 
 39546| 918: ; preds = %909
 39547|     ;; bonus = i64 100
 39548|  br label %919                                                                                                         ;L428
 39549| 
 39550| 919: ; preds = %918, %912, %909, %899, %886, %882, %879, %877, %71
 39551|  %920 = phi i64 [ 0, %71 ], [ 0, %877 ], [ 0, %882 ], [ 0, %879 ], [ 0, %886 ], [ 100, %918 ], [ 0, %909 ], [ %917, %912 ], [ 0, %899 ] ;L411
 39552|     ;; bonus = i64 %920
 39553|     ;; self = ptr %2
 39554|  %921 = call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %6, ptr %54, ptr %39, ptr @anon.fdc8f8a40baf2989242b37eb35661f10.17, ptr %8) ;L438
 39555|     ;; value = i64 %921
 39556|  %922 = load ptr, ptr %35, , !!8, !!8                                                                                  ;L440
 39557|  %923 = gep %35, i64 8                                                                                                 ;L440
 39558|  %924 = load ptr, ptr %923, , !!8, !!8                                                                                 ;L440
 39559|  %925 = gep %924, i64 40                                                                                               ;L440
 39560|  %926 = load ptr, ptr %925, , !!8                                                                                      ;L440
 39561|  %927 = call i64 %926(ptr %922)                                                                                        ;L440
 39562|     ;; tick = i64 %927
 39563|     ;; tick = i64 %927
 39564|     ;; self = ptr %54
 39565|  %928 = gep %924, i64 64                                                                                               ;L452
 39566|  %929 = load ptr, ptr %928, , !!8                                                                                      ;L452
 39567|  %930 = call { i64, ptr } %929(ptr %922)                                                                               ;L452
 39568|  %931 = extractvalue { i64, ptr } %930, 0                                                                              ;L452
 39569|     ;; self[0..+8] = i64 %931
 39571|  %932 = icmp ne i64 %931, 0                                                                                            ;L231<452
 39572|  %933 = extractvalue { i64, ptr } %930, 1
 39574|     ;; default = i64 0
 39576|  %934 = icmp eq ptr %933, null                                                                                         ;L1226<452
 39577|  %935 = select i1 %932, i1 true, i1 %934                                                                               ;L1226<452
 39578|  br i1 %935, label %941, label %936                                                                                    ;L1226<452
 39579| 
 39580| 936: ; preds = %919
 39581|     ;; t = ptr %933
 39583|     ;; m = ptr %933
 39584|     ;; self = ptr %933
 39585|     ;; team = i64 %29
 39586|  %937 = gep %933, i64 576                                                                                              ;L210<452<1227<452
 39587|  %938 = getelementptr i64, ptr %937, i64 %29                                                                           ;L210<452<1227<452
 39588|  %939 = load i64, ptr %938, , !!8                                                                                      ;L210<452<1227<452
 39589|  %940 = icmp eq i64 %939, 0                                                                                            ;L452
 39590|  br label %941                                                                                                         ;L1230<452
 39591| 
 39592| 941: ; preds = %936, %919
 39593|  %942 = phi i1 [ %940, %936 ], [ true, %919 ]                                                                          ;L0<452
 39594|     ;; has_epic_buff = i1 %942
 39595|  switch i64 %91, label %1034 [
 39596|  i64 3, label %943
 39597|  i64 2, label %947
 39598|  ]                                                                                                                     ;L454
 39599| 
 39600| 943: ; preds = %1063, %1061, %1055, %1034, %941
 39601|  %944 = phi i64 [ 0, %1034 ], [ 200, %941 ], [ %1062, %1061 ], [ %1064, %1063 ], [ %1058, %1055 ]                      ;L0
 39602|     ;; coef = i64 %944
 39603|  %945 = mul i64 %944, %921                                                                                             ;L516
 39604|  %946 = icmp eq i64 %60, 0                                                                                             ;L516
 39605|  br i1 %946, label %1069, label %1065                                                                                  ;L516
 39606| 
 39607| 947: ; preds = %941
 39608|     ;; info = ptr %8
 39610|  %948 = gep %924, i64 512                                                                                              ;L457
 39611|  %949 = load ptr, ptr %948, , !!8                                                                                      ;L457
 39612|  call void %949(ptr sret([64 x i8]) %15, ptr %922)                                                                     ;L457
 39613|     ;; predicate[0..+8] = ptr %39
 39614|     ;; predicate[8..+8] = ptr %8
 39615|     ;; self[0..+8] = ptr %39
 39616|     ;; self[8..+8] = ptr %8
 39620|     ;; init = i64 0
 39624|  call void @llvm.memcpy.p0.p0.i64(ptr %12, ptr %15, i64 64, i1 false)                                                  ;L142<459
 39625|     ;; self[0..+8] = ptr %39
 39626|     ;; iter[0..+8] = ptr %39
 39627|     ;; self[0..+8] = ptr %39
 39628|     ;; self[8..+8] = ptr %8
 39629|     ;; iter[8..+8] = ptr %8
 39630|     ;; self[8..+8] = ptr %8
 39633|     ;; f[0..+8] = ptr %39
 39634|     ;; f[8..+8] = ptr %8
 39635|     ;; self = ptr %12
 39636|     ;; init = i64 0
 39637|     ;; accum = i64 0
 39638|  %950 = load i64, ptr %39, , !!44137
 39639|  %951 = freeze i64 %950
 39640|  %952 = gep %39, i64 8
 39641|  %953 = load i64, ptr %952, , !!44137
 39642|  %954 = gep %8, i64 1216
 39643|  %955 = load i32, ptr %954, , !!44139
 39644|  %956 = freeze i32 %955
 39645|  %957 = icmp eq i32 %956, -1
 39646|  %958 = gep %8, i64 1168
 39647|  %959 = icmp eq i64 %951, 0
 39648|  br i1 %957, label %982, label %960
 39649| 
 39650| 960: ; preds = %947
 39651|  br i1 %959, label %961, label %1009
 39652| 
 39653| 961: ; preds = %979, %960
 39654|  %962 = phi i64 [ %981, %979 ], [ 0, %960 ]                                                                            ;L0<128<52<3674<142<459
 39655|     ;; accum = i64 %962
 39656|  %963 = call ptr @gc::simulationNtB5_10EntityIterNtNtNtNtCsjihNppCmMEE_4core4iter6traits8iterator8Iterator4next(ptr %12), !!44141 ;L2670<128<52<3674<142<459
 39657|  %964 = icmp eq ptr %963, null                                                                                         ;L2670<128<52<3674<142<459
 39658|  br i1 %964, label %1027, label %965                                                                                   ;L2670<128<52<3674<142<459
 39659| 
 39660| 965: ; preds = %961
 39661|     ;; x = ptr %963
 39664|     ;; acc = i64 %962
 39665|     ;; elt = ptr %963
 39667|     ;; x = ptr %963
 39672|     ;; self = ptr %963
 39673|     ;; other = ptr %39
 39674|  %966 = load i64, ptr %963, , !!44141, !!8                                                                             ;L1127<458<138<88<2671<128<52<3674<142<459
 39675|     ;; __self_discr = i64 %966
 39676|     ;; __arg1_discr = i64 %950
 39677|  %967 = icmp eq i64 %966, 0                                                                                            ;L1127<458<138<88<2671<128<52<3674<142<459
 39678|  br i1 %967, label %968, label %979                                                                                    ;L1127<458<138<88<2671<128<52<3674<142<459
 39679| 
 39680| 968: ; preds = %965
 39681|  %969 = gep %963, i64 8                                                                                                ;L1127<458<138<88<2671<128<52<3674<142<459
 39682|     ;; __self_0 = ptr %963
 39683|     ;; self = ptr %963
 39684|     ;; __arg1_0 = ptr %39
 39685|     ;; other = ptr %39
 39688|  %970 = load i64, ptr %969, , !!44141, !!8                                                                             ;L1878<2123<1127<458<138<88<2671<128<52<3674<142<459
 39689|  %971 = icmp eq i64 %970, %953                                                                                         ;L1878<2123<1127<458<138<88<2671<128<52<3674<142<459
 39690|     ;; self = ptr %963
 39691|  %972 = gep %963, i64 104
 39692|  %973 = load i64, ptr %972, , !!44141
 39693|  %974 = icmp eq i64 %973, 1
 39694|  %975 = select i1 %971, i1 %974, i1 false                                                                              ;L458<138<88<2671<128<52<3674<142<459
 39695|  br i1 %975, label %976, label %979                                                                                    ;L458<138<88<2671<128<52<3674<142<459
 39696| 
 39697| 976: ; preds = %968
 39698|     ;; self = ptr %958
 39699|  %977 = call zeroext i1 @gc::simulation6effectNtB2_6Effect11is_in_range(ptr %958, ptr %8, ptr %963), !!44141           ;L459<138<88<2671<128<52<3674<142<459
 39700|  %978 = zext i1 %977 to i64                                                                                            ;L138<88<2671<128<52<3674<142<459
 39701|  br label %979                                                                                                         ;L458<138<88<2671<128<52<3674<142<459
 39702| 
 39703| 979: ; preds = %976, %968, %965
 39704|  %980 = phi i64 [ %978, %976 ], [ 0, %968 ], [ 0, %965 ]                                                               ;L0<138<88<2671<128<52<3674<142<459
 39706|     ;; a = i64 %962
 39707|     ;; b = i64 %980
 39708|  %981 = add i64 %980, %962                                                                                             ;L55<88<2671<128<52<3674<142<459
 39709|     ;; accum = i64 %981
 39710|  br label %961                                                                                                         ;L2670<128<52<3674<142<459
 39711| 
 39712| 982: ; preds = %947
 39713|  br i1 %959, label %983, label %998
 39714| 
 39715| 983: ; preds = %997, %982
 39717|  %984 = call ptr @gc::simulationNtB5_10EntityIterNtNtNtNtCsjihNppCmMEE_4core4iter6traits8iterator8Iterator4next(ptr %12), !!44242 ;L2670<128<52<3674<142<459
 39718|  %985 = icmp eq ptr %984, null                                                                                         ;L2670<128<52<3674<142<459
 39719|  br i1 %985, label %1027, label %986                                                                                   ;L2670<128<52<3674<142<459
 39720| 
 39721| 986: ; preds = %983
 39722|     ;; x = ptr %984
 39726|     ;; elt = ptr %984
 39728|     ;; x = ptr %984
 39733|     ;; self = ptr %984
 39734|     ;; other = ptr %39
 39735|  %987 = load i64, ptr %984, , !!44242, !!8                                                                             ;L1127<458<138<88<2671<128<52<3674<142<459
 39736|     ;; __self_discr = i64 %987
 39737|     ;; __arg1_discr = i64 %950
 39738|  %988 = icmp eq i64 %987, 0                                                                                            ;L1127<458<138<88<2671<128<52<3674<142<459
 39739|  br i1 %988, label %989, label %997                                                                                    ;L1127<458<138<88<2671<128<52<3674<142<459
 39740| 
 39741| 989: ; preds = %986
 39742|  %990 = gep %984, i64 8                                                                                                ;L1127<458<138<88<2671<128<52<3674<142<459
 39743|     ;; __self_0 = ptr %984
 39744|     ;; self = ptr %984
 39745|     ;; __arg1_0 = ptr %39
 39746|     ;; other = ptr %39
 39749|  %991 = load i64, ptr %990, , !!44242, !!8                                                                             ;L1878<2123<1127<458<138<88<2671<128<52<3674<142<459
 39750|  %992 = icmp eq i64 %991, %953                                                                                         ;L1878<2123<1127<458<138<88<2671<128<52<3674<142<459
 39751|     ;; self = ptr %984
 39752|  %993 = gep %984, i64 104
 39753|  %994 = load i64, ptr %993, , !!44242
 39754|  %995 = icmp eq i64 %994, 1
 39755|  %996 = select i1 %992, i1 %995, i1 false                                                                              ;L458<138<88<2671<128<52<3674<142<459
 39756|  br i1 %996, label %1023, label %997                                                                                   ;L458<138<88<2671<128<52<3674<142<459
 39757| 
 39758| 997: ; preds = %989, %986
 39763|  br label %983                                                                                                         ;L2670<128<52<3674<142<459
 39764| 
 39765| 998: ; preds = %1008, %982
 39767|  %999 = call ptr @gc::simulationNtB5_10EntityIterNtNtNtNtCsjihNppCmMEE_4core4iter6traits8iterator8Iterator4next(ptr %12), !!44242 ;L2670<128<52<3674<142<459
 39768|  %1000 = icmp eq ptr %999, null                                                                                        ;L2670<128<52<3674<142<459
 39769|  br i1 %1000, label %1027, label %1001                                                                                 ;L2670<128<52<3674<142<459
 39770| 
 39771| 1001: ; preds = %998
 39772|     ;; x = ptr %999
 39776|     ;; elt = ptr %999
 39778|     ;; x = ptr %999
 39783|     ;; self = ptr %999
 39784|     ;; other = ptr %39
 39785|  %1002 = load i64, ptr %999, , !!44242, !!8                                                                            ;L1127<458<138<88<2671<128<52<3674<142<459
 39786|     ;; __self_discr = i64 %1002
 39787|     ;; __arg1_discr = i64 %950
 39788|  %1003 = icmp eq i64 %1002, %951                                                                                       ;L1127<458<138<88<2671<128<52<3674<142<459
 39789|  br i1 %1003, label %1004, label %1008                                                                                 ;L1127<458<138<88<2671<128<52<3674<142<459
 39790| 
 39791| 1004: ; preds = %1001
 39792|     ;; self = ptr %999
 39793|  %1005 = gep %999, i64 104                                                                                             ;L1261<459<138<88<2671<128<52<3674<142<459
 39794|  %1006 = load i64, ptr %1005, , !!44242, !!8                                                                           ;L1261<459<138<88<2671<128<52<3674<142<459
 39795|  %1007 = icmp eq i64 %1006, 1                                                                                          ;L459<138<88<2671<128<52<3674<142<459
 39796|  br i1 %1007, label %1023, label %1008                                                                                 ;L459<138<88<2671<128<52<3674<142<459
 39797| 
 39798| 1008: ; preds = %1004, %1001
 39803|  br label %998                                                                                                         ;L2670<128<52<3674<142<459
 39804| 
 39805| 1009: ; preds = %1024, %960
 39806|  %1010 = phi i64 [ %1026, %1024 ], [ 0, %960 ]                                                                         ;L0<128<52<3674<142<459
 39807|     ;; accum = i64 %1010
 39808|  %1011 = call ptr @gc::simulationNtB5_10EntityIterNtNtNtNtCsjihNppCmMEE_4core4iter6traits8iterator8Iterator4next(ptr %12), !!44141 ;L2670<128<52<3674<142<459
 39809|  %1012 = icmp eq ptr %1011, null                                                                                       ;L2670<128<52<3674<142<459
 39810|  br i1 %1012, label %1027, label %1013                                                                                 ;L2670<128<52<3674<142<459
 39811| 
 39812| 1013: ; preds = %1009
 39813|     ;; x = ptr %1011
 39816|     ;; acc = i64 %1010
 39817|     ;; elt = ptr %1011
 39819|     ;; x = ptr %1011
 39824|     ;; self = ptr %1011
 39825|     ;; other = ptr %39
 39826|  %1014 = load i64, ptr %1011, , !!44141, !!8                                                                           ;L1127<458<138<88<2671<128<52<3674<142<459
 39827|     ;; __self_discr = i64 %1014
 39828|     ;; __arg1_discr = i64 %950
 39829|  %1015 = icmp eq i64 %1014, %951                                                                                       ;L1127<458<138<88<2671<128<52<3674<142<459
 39830|  br i1 %1015, label %1016, label %1024                                                                                 ;L1127<458<138<88<2671<128<52<3674<142<459
 39831| 
 39832| 1016: ; preds = %1013
 39833|     ;; self = ptr %1011
 39834|  %1017 = gep %1011, i64 104                                                                                            ;L1261<459<138<88<2671<128<52<3674<142<459
 39835|  %1018 = load i64, ptr %1017, , !!44141, !!8                                                                           ;L1261<459<138<88<2671<128<52<3674<142<459
 39836|  %1019 = icmp eq i64 %1018, 1                                                                                          ;L459<138<88<2671<128<52<3674<142<459
 39837|  br i1 %1019, label %1020, label %1024                                                                                 ;L459<138<88<2671<128<52<3674<142<459
 39838| 
 39839| 1020: ; preds = %1016
 39840|     ;; self = ptr %958
 39841|  %1021 = call zeroext i1 @gc::simulation6effectNtB2_6Effect11is_in_range(ptr %958, ptr %8, ptr %1011), !!44141         ;L459<138<88<2671<128<52<3674<142<459
 39842|  %1022 = zext i1 %1021 to i64                                                                                          ;L138<88<2671<128<52<3674<142<459
 39843|  br label %1024                                                                                                        ;L458<138<88<2671<128<52<3674<142<459
 39844| 
 39845| 1023: ; preds = %1004, %989
 39846|     ;; self = ptr null
 39847|  call void @core::option13unwrap_failed(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.32) #29, !!44245                    ;L1013<459<138<88<2671<128<52<3674<142<459
 39848|  unreachable                                                                                                           ;L1013<459<138<88<2671<128<52<3674<142<459
 39849| 
 39850| 1024: ; preds = %1020, %1016, %1013
 39851|  %1025 = phi i64 [ %1022, %1020 ], [ 0, %1016 ], [ 0, %1013 ]                                                          ;L0<138<88<2671<128<52<3674<142<459
 39853|     ;; a = i64 %1010
 39854|     ;; b = i64 %1025
 39855|  %1026 = add i64 %1025, %1010                                                                                          ;L55<88<2671<128<52<3674<142<459
 39856|     ;; accum = i64 %1026
 39857|  br label %1009                                                                                                        ;L2670<128<52<3674<142<459
 39858| 
 39859| 1027: ; preds = %1009, %998, %983, %961
 39860|  %1028 = phi i64 [ %962, %961 ], [ 0, %983 ], [ 0, %998 ], [ %1010, %1009 ]                                            ;L2674<128<52<3674<142<459
 39862|     ;; in_range_minion = i64 %1028
 39864|  %1029 = gep %8, i64 136                                                                                               ;L460
 39865|  %1030 = load i64, ptr %1029,                                                                                          ;L460
 39866|     ;; self[0..+8] = i64 %1030
 39868|  %1031 = gep %8, i64 152                                                                                               ;L460
 39869|  %1032 = load i64, ptr %1031,                                                                                          ;L460
 39870|     ;; self[16..+8] = i64 %1032
 39871|     ;; f[0..+8] = ptr %39
 39873|     ;; f[8..+8] = ptr %922
 39875|     ;; f[16..+8] = ptr %924
 39877|     ;; f[24..+8] = ptr %54
 39879|     ;; f[32..+8] = ptr %8
 39881|     ;; f[40..+8] = ptr undef
 39883|  %1033 = trunc nuw i64 %1030 to i1                                                                                     ;L659<460
 39884|  br i1 %1033, label %1035, label %1055                                                                                 ;L659<460
 39885| 
 39886| 1034: ; preds = %941
 39887|     ;; self = ptr %8
 39888|     ;; coef = i64 0
 39889|  br label %943                                                                                                         ;L484
 39890| 
 39891| 1035: ; preds = %1027
 39893|     ;; x[8..+8] = i64 %1032
 39898|     ;; e[8..+8] = i64 %1032
 39899|  %1036 = gep %39, i64 1472                                                                                             ;L460<661<460
 39900|  %1037 = load i64, ptr %1036, , !!44266, !!8                                                                           ;L460<661<460
 39901|  %1038 = icmp eq i64 %1032, %1037                                                                                      ;L460<661<460
 39902|  br i1 %1038, label %1055, label %1039                                                                                 ;L460<661<460
 39903| 
 39904| 1039: ; preds = %1035
 39905|  %1040 = gep %924, i64 496                                                                                             ;L460<661<460
 39906|  %1041 = load ptr, ptr %1040, , !!44266, !!8                                                                           ;L460<661<460
 39907|  %1042 = call ptr %1041(ptr %922, i64 %1032), !!44266                                                                  ;L460<661<460
 39908|     ;; self = ptr %1042
 39909|     ;; f[0..+8] = ptr %8
 39911|     ;; f[8..+8] = ptr %54
 39913|  %1043 = icmp eq ptr %1042, null                                                                                       ;L659<461<661<460
 39914|  br i1 %1043, label %1053, label %1044                                                                                 ;L659<461<661<460
 39915| 
 39916| 1044: ; preds = %1039
 39917|     ;; x = ptr %1042
 39918|     ;; e = ptr %1042
 39919|     ;; self = ptr %8
 39920|  br i1 %957, label %1052, label %1045                                                                                  ;L742<461<661<461<661<460
 39921| 
 39922| 1045: ; preds = %1044
 39923|  %1046 = gep %1042, i64 1648                                                                                           ;L461<661<461<661<460
 39924|  %1047 = load i64, ptr %1046, , !!44266, !!8                                                                           ;L461<661<461<661<460
 39925|     ;; self = ptr %958
 39926|  %1048 = call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %958, ptr %54, ptr %8, ptr @anon.fdc8f8a40baf2989242b37eb35661f10.17, ptr %1042), !!44266 ;L461<661<461<661<460
 39927|  %1049 = icmp uge i64 %1047, %1048                                                                                     ;L461<661<461<661<460
 39928|  %1050 = icmp ugt i64 %1028, 1
 39929|  %1051 = select i1 %1049, i1 true, i1 %1050                                                                            ;L460<661<460
 39930|  br i1 %1051, label %1059, label %1055                                                                                 ;L460<661<460
 39931| 
 39932| 1052: ; preds = %1044
 39933|     ;; self = ptr null
 39934|  call void @core::option13unwrap_failed(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.33) #29, !!44266                    ;L1013<461<661<461<661<460
 39935|  unreachable                                                                                                           ;L1013<461<661<461<661<460
 39936| 
 39937| 1053: ; preds = %1039
 39938|  %1054 = icmp ugt i64 %1028, 1                                                                                         ;L461<661<460
 39939|  br i1 %1054, label %1059, label %1055                                                                                 ;L460
 39940| 
 39941| 1055: ; preds = %1053, %1045, %1035, %1027
 39942|  %1056 = call i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %6, ptr %54, ptr %39, ptr @anon.fdc8f8a40baf2989242b37eb35661f10.17, ptr %8) ;L476
 39943|     ;; expected_damage = i64 %1056
 39944|  %1057 = icmp sgt i64 %60, %1056                                                                                       ;L478
 39945|  %1058 = select i1 %1057, i64 0, i64 30                                                                                ;L478
 39946|  br label %943                                                                                                         ;L478
 39947| 
 39948| 1059: ; preds = %1053, %1045
 39949|  %1060 = icmp eq i8 %9, 2                                                                                              ;L462
 39950|  br i1 %1060, label %1061, label %1063                                                                                 ;L462
 39951| 
 39952| 1061: ; preds = %1059
 39953|  %1062 = select i1 %942, i64 160, i64 240                                                                              ;L463
 39954|  br label %943                                                                                                         ;L463
 39955| 
 39956| 1063: ; preds = %1059
 39957|  %1064 = select i1 %942, i64 80, i64 160                                                                               ;L469
 39958|  br label %943                                                                                                         ;L469
 39959| 
 39960| 1065: ; preds = %943
 39961|  %1066 = icmp eq i64 %60, -1                                                                                           ;L516
 39962|  %1067 = icmp eq i64 %945, -9223372036854775808                                                                        ;L516
 39963|  %1068 = and i1 %1066, %1067                                                                                           ;L516
 39964|  br i1 %1068, label %1074, label %1070                                                                                 ;L516
 39965| 
 39966| 1069: ; preds = %943
 39967|  call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.148) #29  ;L516
 39968|  unreachable                                                                                                           ;L516
 39969| 
 39970| 1070: ; preds = %1065
 39971|  %1071 = sdiv i64 %945, %60                                                                                            ;L516
 39972|     ;; self = i64 %1071
 39973|     ;; other = i64 %944
 39974|  %1072 = call i64 @llvm.smin.i64(i64 %944, i64 %1071)                                                                  ;L1078<516
 39975|  %1073 = add nsw i64 %1072, %920                                                                                       ;L516
 39976|  br label %541                                                                                                         ;L26
 39977| 
 39978| 1074: ; preds = %1065
 39979|  call void @core::panicking11panic_const24panic_const_div_overflow(ptr @anon.fdc8f8a40baf2989242b37eb35661f10.148) #29 ;L516
 39980|  unreachable                                                                                                           ;L516
 39981| }
