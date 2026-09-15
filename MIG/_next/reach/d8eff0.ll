 37862| define void @ai::score_parameter25calculate_score_parameter(ptr sret([5384 x i8]) %0, i64 %1, ptr readnone %2, ptr %3, ptr %4, ptr readnone %5) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 37863|  %7 = alloca [56 x i8],
 37867|  %8 = alloca [24 x i8],
 37868|  %9 = alloca [56 x i8],
 37872|  %10 = alloca [24 x i8],
 37873|  %11 = alloca [56 x i8],
 37877|  %12 = alloca [24 x i8],
 37878|  %13 = alloca [56 x i8],
 37882|  %14 = alloca [24 x i8],
 37902|  %15 = alloca [24 x i8],
 37903|  %16 = alloca [32 x i8],
 37918|  %17 = alloca [56 x i8],
 37919|  %18 = alloca [64 x i8],
 37922|  %19 = alloca [56 x i8],
 37923|  %20 = alloca [64 x i8],
 37926|  %21 = alloca [56 x i8],
 37927|  %22 = alloca [64 x i8],
 37929|  %23 = alloca [56 x i8],
 37930|  %24 = alloca [64 x i8],
 37976|  %25 = alloca [40 x i8],
 37977|  %26 = alloca [40 x i8],
 37978|  %27 = alloca [120 x i8],
 37981|  %28 = alloca [120 x i8],
 37982|  %29 = alloca [112 x i8],
 37983|     ;; self[8..+112] = ptr %29
 37984|  %30 = alloca [272 x i8],
 37985|  %31 = alloca [32 x i8],
 37986|  %32 = alloca [216 x i8],
 37987|  %33 = alloca [24 x i8],
 37988|  %34 = alloca [216 x i8],
 37990|  %35 = alloca [216 x i8],
 37991|  %36 = alloca [24 x i8],
 37992|  %37 = alloca [216 x i8],
 37994|  %38 = alloca [80 x i8],
 37995|  %39 = alloca [32 x i8],
 37996|  %40 = alloca [48 x i8],
 37997|  %41 = alloca [32 x i8],
 37998|  %42 = alloca [56 x i8],
 38001|  %43 = alloca [56 x i8],
 38002|  %44 = alloca [56 x i8],
 38003|     ;; a[0..+56] = ptr %44
 38004|     ;; self[0..+56] = ptr %44
 38005|  %45 = alloca [128 x i8],
 38006|  %46 = alloca [32 x i8],
 38007|  %47 = alloca [5384 x i8],
 38008|     ;; version = i64 %1
 38009|     ;; rnd = ptr %2
 38010|     ;; player = ptr %3
 38011|     ;; data = ptr %4
 38012|     ;; _debug = ptr %5
 38013|     ;; parameter = ptr %47
 38014|     ;; near_minions = ptr %46
 38015|     ;; self = ptr %43
 38016|     ;; self = ptr %42
 38017|     ;; near_allies_with_action = ptr %41
 38018|     ;; near_enemies_with_action = ptr %39
 38019|     ;; p = ptr %37
 38020|     ;; p = ptr %34
 38021|     ;; near_towers = ptr %31
 38022|     ;; self = ptr %28
 38023|     ;; self = ptr %27
 38024|     ;; iter = ptr %25
 38025|     ;; my_minions = ptr %24
 38026|     ;; self = ptr %23
 38027|     ;; my_minions = ptr %22
 38028|     ;; self = ptr %21
 38029|     ;; my_minions = ptr %20
 38030|     ;; self = ptr %19
 38031|     ;; my_minions = ptr %18
 38032|     ;; self = ptr %17
 38033|     ;; near_jungles = ptr %16
 38034|     ;; len = i64 5
 38035|     ;; count = i64 5
 38037|     ;; len = i64 5
 38038|     ;; count = i64 5
 38040|     ;; count = i64 1
 38042|     ;; default = i64 0
 38043|     ;; count = i64 1
 38045|     ;; default = i64 0
 38046|     ;; count = i64 1
 38047|     ;; count = i64 1
 38048|     ;; count = i64 1
 38050|     ;; default = i64 0
 38051|     ;; count = i64 1
 38052|     ;; count = i64 1
 38055|     ;; default = i64 0
 38056|     ;; count = i64 1
 38058|     ;; default = i64 0
 38059|     ;; count = i64 1
 38060|     ;; count = i64 1
 38061|     ;; count = i64 1
 38062|     ;; count = i64 1
 38063|     ;; count = i64 1
 38064|     ;; count = i64 1
 38065|     ;; count = i64 1
 38066|     ;; count = i64 1
 38068|     ;; count = i64 1
 38069|  %48 = gep %3, i64 2352                                                                                                ;L1462
 38070|  %49 = load i64, ptr %48, , !!8                                                                                        ;L1462
 38071|  %50 = icmp ult i64 %49, 2                                                                                             ;L1462
 38072|  br i1 %50, label %51, label %61                                                                                       ;L1462
 38073| 
 38074| 51: ; preds = %6
 38075|     ;; self = ptr %3
 38076|  %52 = gep %3, i64 2496                                                                                                ;L581<1462
 38077|  %53 = load i32, ptr %52, , !!8                                                                                        ;L581<1462
 38078|  %54 = zext nneg i32 %53 to i64                                                                                        ;L581<1462
 38079|  %55 = load ptr, ptr %4, , !!8, !!8                                                                                    ;L1462
 38080|  %56 = gep %55, i64 480                                                                                                ;L1462
 38081|  %57 = getelementptr [5 x ptr], ptr %56, i64 %49                                                                       ;L1462
 38082|  %58 = getelementptr ptr, ptr %57, i64 %54                                                                             ;L1462
 38083|  %59 = load ptr, ptr %58, , !!8                                                                                        ;L1462
 38084|     ;; self = ptr %59
 38085|  %60 = icmp eq ptr %59, null                                                                                           ;L1011<1462
 38086|  br i1 %60, label %145, label %62                                                                                      ;L1011<1462
 38087| 
 38088| 61: ; preds = %6
 38089|  tail call void @core::panicking18panic_bounds_check(i64 %49, i64 2, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.123) #25 ;L1462
 38090|  unreachable                                                                                                           ;L1462
 38091| 
 38092| 62: ; preds = %51
 38093|     ;; champ = ptr %59
 38094|     ;; self = ptr %59
 38095|     ;; other = ptr %59
 38096|     ;; self = ptr %59
 38097|     ;; self = ptr %59
 38098|     ;; self = ptr %59
 38099|     ;; self = ptr %59
 38100|     ;; self = ptr %59
 38101|     ;; self = ptr %59
 38102|     ;; self = ptr %59
 38103|     ;; self = ptr %59
 38104|     ;; self = ptr %59
 38105|     ;; self = ptr %59
 38106|     ;; caster = ptr %59
 38107|     ;; self = ptr %59
 38108|     ;; self = ptr %59
 38109|     ;; self = ptr %59
 38110|     ;; caster = ptr %59
 38111|     ;; self = ptr %59
 38112|     ;; self = ptr %59
 38113|     ;; self = ptr %59
 38114|     ;; caster = ptr %59
 38115|     ;; self = ptr %59
 38116|     ;; self = ptr %59
 38117|     ;; other = ptr %59
 38118|     ;; other = ptr %59
 38119|     ;; other = ptr %59
 38121|  %63 = gep %4, i64 8                                                                                                   ;L1466
 38122|  %64 = load ptr, ptr %63, , !!8, !!8                                                                                   ;L1466
 38123|     ;; self = ptr %64
 38124|  %65 = load ptr, ptr %64, , !!8, !!8                                                                                   ;L1466
 38125|     ;; self = ptr %3
 38126|  %66 = gep %47, i64 5368                                                                                               ;L1463
 38127|  store i64 %1, ptr %66,                                                                                                ;L1463
 38128|  %67 = gep %47, i64 5304                                                                                               ;L1463
 38129|  store ptr inttoptr (i64 8 to ptr), ptr %67,                                                                           ;L1463
 38130|  %68 = gep %47, i64 5312                                                                                               ;L1463
 38131|  store ptr %65, ptr %68,                                                                                               ;L1463
 38132|  %69 = gep %47, i64 5320                                                                                               ;L1463
 38133|  %70 = gep %47, i64 5328                                                                                               ;L1463
 38134|  %71 = gep %47, i64 5336                                                                                               ;L1463
 38135|  call void @llvm.memset.p0.i64(ptr %69, i8 0, i64 16, i1 false)                                                        ;L1463
 38136|  store ptr inttoptr (i64 8 to ptr), ptr %71,                                                                           ;L1463
 38137|  %72 = gep %47, i64 5344                                                                                               ;L1463
 38138|  store ptr %65, ptr %72,                                                                                               ;L1463
 38139|  %73 = gep %47, i64 5352                                                                                               ;L1463
 38140|  %74 = gep %47, i64 5360                                                                                               ;L1463
 38141|  %75 = gep %47, i64 2328                                                                                               ;L1463
 38142|  store i64 0, ptr %75,                                                                                                 ;L1463
 38143|  %76 = gep %47, i64 2336                                                                                               ;L1463
 38144|  %77 = gep %47, i64 2352                                                                                               ;L1463
 38145|  call void @llvm.memset.p0.i64(ptr %73, i8 0, i64 16, i1 false)                                                        ;L1463
 38146|  store ptr inttoptr (i64 8 to ptr), ptr %77,                                                                           ;L1463
 38147|  %78 = gep %47, i64 2360                                                                                               ;L1463
 38148|  store ptr %65, ptr %78,                                                                                               ;L1463
 38149|  %79 = gep %47, i64 2368                                                                                               ;L1463
 38150|  %80 = gep %47, i64 2376                                                                                               ;L1463
 38151|  %81 = gep %47, i64 2384                                                                                               ;L1463
 38152|  call void @llvm.memset.p0.i64(ptr %79, i8 0, i64 16, i1 false)                                                        ;L1463
 38153|  store ptr inttoptr (i64 8 to ptr), ptr %81,                                                                           ;L1463
 38154|  %82 = gep %47, i64 2392                                                                                               ;L1463
 38155|  store ptr %65, ptr %82,                                                                                               ;L1463
 38156|  %83 = gep %47, i64 2400                                                                                               ;L1463
 38157|  %84 = gep %47, i64 2416                                                                                               ;L1463
 38158|  %85 = gep %47, i64 2424                                                                                               ;L1463
 38159|  call void @llvm.memset.p0.i64(ptr %83, i8 0, i64 24, i1 false)                                                        ;L1463
 38160|  store i64 %49, ptr %85,                                                                                               ;L1463
 38161|  %86 = gep %47, i64 2432                                                                                               ;L1463
 38162|  store i64 %54, ptr %86,                                                                                               ;L1463
 38163|  %87 = gep %47, i64 2440                                                                                               ;L1463
 38164|  %88 = gep %47, i64 2448                                                                                               ;L1463
 38165|  %89 = gep %47, i64 2456                                                                                               ;L1463
 38166|  %90 = gep %47, i64 2464                                                                                               ;L1463
 38167|  %91 = gep %47, i64 2472                                                                                               ;L1463
 38168|  %92 = gep %47, i64 2480                                                                                               ;L1463
 38169|  %93 = gep %47, i64 2600                                                                                               ;L1463
 38170|  %94 = gep %47, i64 2656                                                                                               ;L1463
 38171|  %95 = gep %47, i64 2712                                                                                               ;L1463
 38172|  %96 = gep %47, i64 2768                                                                                               ;L1463
 38173|  %97 = gep %47, i64 2824                                                                                               ;L1463
 38174|  %98 = gep %47, i64 2880                                                                                               ;L1463
 38175|  %99 = gep %47, i64 2936                                                                                               ;L1463
 38176|  %100 = gep %47, i64 2992                                                                                              ;L1463
 38177|  %101 = gep %47, i64 3048                                                                                              ;L1463
 38178|  %102 = gep %47, i64 3104                                                                                              ;L1463
 38179|  %103 = gep %47, i64 3160                                                                                              ;L1463
 38180|  %104 = gep %47, i64 3216                                                                                              ;L1463
 38181|  %105 = gep %47, i64 3272                                                                                              ;L1463
 38182|  %106 = gep %47, i64 3328                                                                                              ;L1463
 38183|  %107 = gep %47, i64 3384                                                                                              ;L1463
 38184|  %108 = gep %47, i64 3440                                                                                              ;L1463
 38185|  %109 = gep %47, i64 3496                                                                                              ;L1463
 38186|  %110 = gep %47, i64 3552                                                                                              ;L1463
 38187|  %111 = gep %47, i64 3608                                                                                              ;L1463
 38188|  %112 = gep %47, i64 3664                                                                                              ;L1463
 38189|  %113 = gep %47, i64 3720                                                                                              ;L1463
 38190|  %114 = gep %47, i64 3776                                                                                              ;L1463
 38191|  %115 = gep %47, i64 3832                                                                                              ;L1463
 38192|  %116 = gep %47, i64 3888                                                                                              ;L1463
 38193|  %117 = gep %47, i64 3944                                                                                              ;L1463
 38194|  %118 = gep %47, i64 4000                                                                                              ;L1463
 38195|  %119 = gep %47, i64 4056                                                                                              ;L1463
 38196|  %120 = gep %47, i64 4112                                                                                              ;L1463
 38197|  %121 = gep %47, i64 4168                                                                                              ;L1463
 38198|  %122 = gep %47, i64 4224                                                                                              ;L1463
 38199|  %123 = gep %47, i64 4280                                                                                              ;L1463
 38200|  %124 = gep %47, i64 4336                                                                                              ;L1463
 38201|  %125 = gep %47, i64 4392                                                                                              ;L1463
 38202|  %126 = gep %47, i64 4448                                                                                              ;L1463
 38203|  %127 = gep %47, i64 4504                                                                                              ;L1463
 38204|  %128 = gep %47, i64 4560                                                                                              ;L1463
 38205|  %129 = gep %47, i64 4616                                                                                              ;L1463
 38206|  %130 = gep %47, i64 4672                                                                                              ;L1463
 38207|  %131 = gep %47, i64 4728                                                                                              ;L1463
 38208|  %132 = gep %47, i64 4784                                                                                              ;L1463
 38209|  %133 = gep %47, i64 4840                                                                                              ;L1463
 38210|  %134 = gep %47, i64 4896                                                                                              ;L1463
 38211|  %135 = gep %47, i64 4952                                                                                              ;L1463
 38212|  %136 = gep %47, i64 5008                                                                                              ;L1463
 38213|  %137 = gep %47, i64 5064                                                                                              ;L1463
 38214|  %138 = gep %47, i64 5120                                                                                              ;L1463
 38215|  %139 = gep %47, i64 5176                                                                                              ;L1463
 38216|  %140 = gep %47, i64 5232                                                                                              ;L1463
 38217|  %141 = gep %47, i64 5288                                                                                              ;L1463
 38218|  %142 = gep %47, i64 5296                                                                                              ;L1463
 38219|  store i64 0, ptr %47,                                                                                                 ;L1463
 38220|  %143 = gep %47, i64 5376                                                                                              ;L1463
 38221|  store i8 0, ptr %143,                                                                                                 ;L1463
 38222|  call void @llvm.memset.p0.i64(ptr %87, i8 0, i64 154, i1 false)                                                       ;L1463
 38223|  call void @llvm.memset.p0.i64(ptr %93, i8 0, i64 50, i1 false)                                                        ;L1463
 38224|  call void @llvm.memset.p0.i64(ptr %94, i8 0, i64 50, i1 false)                                                        ;L1463
 38225|  call void @llvm.memset.p0.i64(ptr %95, i8 0, i64 50, i1 false)                                                        ;L1463
 38226|  call void @llvm.memset.p0.i64(ptr %96, i8 0, i64 50, i1 false)                                                        ;L1463
 38227|  call void @llvm.memset.p0.i64(ptr %97, i8 0, i64 50, i1 false)                                                        ;L1463
 38228|  call void @llvm.memset.p0.i64(ptr %98, i8 0, i64 50, i1 false)                                                        ;L1463
 38229|  call void @llvm.memset.p0.i64(ptr %99, i8 0, i64 50, i1 false)                                                        ;L1463
 38230|  call void @llvm.memset.p0.i64(ptr %100, i8 0, i64 50, i1 false)                                                       ;L1463
 38231|  call void @llvm.memset.p0.i64(ptr %101, i8 0, i64 50, i1 false)                                                       ;L1463
 38232|  call void @llvm.memset.p0.i64(ptr %102, i8 0, i64 50, i1 false)                                                       ;L1463
 38233|  call void @llvm.memset.p0.i64(ptr %103, i8 0, i64 50, i1 false)                                                       ;L1463
 38234|  call void @llvm.memset.p0.i64(ptr %104, i8 0, i64 50, i1 false)                                                       ;L1463
 38235|  call void @llvm.memset.p0.i64(ptr %105, i8 0, i64 50, i1 false)                                                       ;L1463
 38236|  call void @llvm.memset.p0.i64(ptr %106, i8 0, i64 50, i1 false)                                                       ;L1463
 38237|  call void @llvm.memset.p0.i64(ptr %107, i8 0, i64 50, i1 false)                                                       ;L1463
 38238|  call void @llvm.memset.p0.i64(ptr %108, i8 0, i64 50, i1 false)                                                       ;L1463
 38239|  call void @llvm.memset.p0.i64(ptr %109, i8 0, i64 50, i1 false)                                                       ;L1463
 38240|  call void @llvm.memset.p0.i64(ptr %110, i8 0, i64 50, i1 false)                                                       ;L1463
 38241|  call void @llvm.memset.p0.i64(ptr %111, i8 0, i64 50, i1 false)                                                       ;L1463
 38242|  call void @llvm.memset.p0.i64(ptr %112, i8 0, i64 50, i1 false)                                                       ;L1463
 38243|  call void @llvm.memset.p0.i64(ptr %113, i8 0, i64 50, i1 false)                                                       ;L1463
 38244|  call void @llvm.memset.p0.i64(ptr %114, i8 0, i64 50, i1 false)                                                       ;L1463
 38245|  call void @llvm.memset.p0.i64(ptr %115, i8 0, i64 50, i1 false)                                                       ;L1463
 38246|  call void @llvm.memset.p0.i64(ptr %116, i8 0, i64 50, i1 false)                                                       ;L1463
 38247|  call void @llvm.memset.p0.i64(ptr %117, i8 0, i64 50, i1 false)                                                       ;L1463
 38248|  call void @llvm.memset.p0.i64(ptr %118, i8 0, i64 50, i1 false)                                                       ;L1463
 38249|  call void @llvm.memset.p0.i64(ptr %119, i8 0, i64 50, i1 false)                                                       ;L1463
 38250|  call void @llvm.memset.p0.i64(ptr %120, i8 0, i64 50, i1 false)                                                       ;L1463
 38251|  call void @llvm.memset.p0.i64(ptr %121, i8 0, i64 50, i1 false)                                                       ;L1463
 38252|  call void @llvm.memset.p0.i64(ptr %122, i8 0, i64 50, i1 false)                                                       ;L1463
 38253|  call void @llvm.memset.p0.i64(ptr %123, i8 0, i64 50, i1 false)                                                       ;L1463
 38254|  call void @llvm.memset.p0.i64(ptr %124, i8 0, i64 50, i1 false)                                                       ;L1463
 38255|  call void @llvm.memset.p0.i64(ptr %125, i8 0, i64 50, i1 false)                                                       ;L1463
 38256|  call void @llvm.memset.p0.i64(ptr %126, i8 0, i64 50, i1 false)                                                       ;L1463
 38257|  call void @llvm.memset.p0.i64(ptr %127, i8 0, i64 50, i1 false)                                                       ;L1463
 38258|  call void @llvm.memset.p0.i64(ptr %128, i8 0, i64 50, i1 false)                                                       ;L1463
 38259|  call void @llvm.memset.p0.i64(ptr %129, i8 0, i64 50, i1 false)                                                       ;L1463
 38260|  call void @llvm.memset.p0.i64(ptr %130, i8 0, i64 50, i1 false)                                                       ;L1463
 38261|  call void @llvm.memset.p0.i64(ptr %131, i8 0, i64 50, i1 false)                                                       ;L1463
 38262|  call void @llvm.memset.p0.i64(ptr %132, i8 0, i64 50, i1 false)                                                       ;L1463
 38263|  call void @llvm.memset.p0.i64(ptr %133, i8 0, i64 50, i1 false)                                                       ;L1463
 38264|  call void @llvm.memset.p0.i64(ptr %134, i8 0, i64 50, i1 false)                                                       ;L1463
 38265|  call void @llvm.memset.p0.i64(ptr %135, i8 0, i64 50, i1 false)                                                       ;L1463
 38266|  call void @llvm.memset.p0.i64(ptr %136, i8 0, i64 50, i1 false)                                                       ;L1463
 38267|  call void @llvm.memset.p0.i64(ptr %137, i8 0, i64 50, i1 false)                                                       ;L1463
 38268|  call void @llvm.memset.p0.i64(ptr %138, i8 0, i64 50, i1 false)                                                       ;L1463
 38269|  call void @llvm.memset.p0.i64(ptr %139, i8 0, i64 50, i1 false)                                                       ;L1463
 38270|  call void @llvm.memset.p0.i64(ptr %140, i8 0, i64 50, i1 false)                                                       ;L1463
 38271|  call void @llvm.memset.p0.i64(ptr %141, i8 0, i64 16, i1 false)                                                       ;L1463
 38276|  %144 = sub nuw nsw i64 1, %49                                                                                         ;L1497
 38277|  invoke void @gc::simulationNtB5_21AbstractGameWithCache12iter_minions(ptr sret([56 x i8]) %43, ptr %55, i64 %144)
 38278|  to label %150 unwind label %146                                                                                       ;L1497
 38279| 
 38280| 145: ; preds = %51
 38281|  tail call void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.124) #25                       ;L1013<1462
 38282|  unreachable                                                                                                           ;L1013<1462
 38283| 
 38284| 146: ; preds = %3398, %3397, %3395, %163, %151, %150, %62
 38285|  %147 = phi i8 [ %159, %163 ], [ 0, %3398 ], [ 1, %151 ], [ 1, %62 ], [ 1, %150 ], [ 0, %3397 ], [ 0, %3395 ]          ;L0
 38286|  %148 = cleanuppad within none []
 38287|  %149 = icmp eq i8 %147, 0                                                                                             ;L2418
 38288|  br i1 %149, label %3660, label %3659                                                                                  ;L2418
 38289| 
 38290| 150: ; preds = %62
 38291|     ;; predicate = ptr %59
 38292|  call void @llvm.memcpy.p0.p0.i64(ptr %44, ptr %43, i64 56, i1 false)                                                  ;L28<957<1497
 38293|     ;; self[56..+8] = ptr %59
 38294|     ;; a[56..+8] = ptr %59
 38297|  invoke void @gc::simulationNtB5_21AbstractGameWithCache12iter_minions(ptr sret([56 x i8]) %42, ptr %55, i64 %49)
 38298|  to label %151 unwind label %146                                                                                       ;L1498
 38299| 
 38300| 151: ; preds = %150
 38301|     ;; predicate = ptr %59
 38302|  %152 = gep %45, i64 64                                                                                                ;L37<515<1497
 38303|  call void @llvm.memcpy.p0.p0.i64(ptr %152, ptr %42, i64 56, i1 false)                                                 ;L28<957<1498
 38304|     ;; other[56..+8] = ptr %59
 38305|     ;; b[56..+8] = ptr %59
 38309|  call void @llvm.memcpy.p0.p0.i64(ptr %45, ptr %44, i64 56, i1 false), !!52860                                         ;L37<515<1497
 38310|  %153 = gep %45, i64 56                                                                                                ;L37<515<1497
 38311|  store ptr %59, ptr %153, , !!52860                                                                                    ;L37<515<1497
 38312|  %154 = gep %45, i64 120                                                                                               ;L37<515<1497
 38313|  store ptr %59, ptr %154, , !!52857                                                                                    ;L37<515<1497
 38315|  invoke void @core::iter8adapters5chain5ChainINtNtB29_6filter6FilterIB25_IB25_INtNtB29_6copied6CopiedINtNtNtB2d_5slice4iter4IterBU_EEB3t_EB3t_ENCNvNtCshdEBA0ozCnw_7game_ai15score_parameter25calculate_score_parameter0EIB2X_B3j_NCB4y_s_0EEEB4C_(ptr sret([32 x i8]) %46, ptr %45, ptr %65)
 38316|  to label %155 unwind label %146                                                                                       ;L1496
 38317| 
 38318| 155: ; preds = %151
 38320|  %156 = gep %59, i64 1472                                                                                              ;L1500
 38321|  %157 = load i64, ptr %156, , !!8                                                                                      ;L1500
 38322|     ;; id = i64 %157
 38323|  store i64 %157, ptr %84,                                                                                              ;L1500
 38324|  invoke void @ai::utils26precompute_champion_powers(i64 %1, ptr %4, ptr %75)
 38325|  to label %164 unwind label %158                                                                                       ;L1501
 38326| 
 38327| 158: ; preds = %3393, %3392, %3390, %194, %164, %155
 38328|  %159 = phi i8 [ %190, %194 ], [ 0, %3393 ], [ 1, %164 ], [ 1, %155 ], [ 0, %3392 ], [ 0, %3390 ]                      ;L0
 38329|  %160 = cleanuppad within none []
 38331|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %46) [ "funclet"(token %160) ]
 38332|  to label %163 unwind label %161                                                                                       ;L825<2418
 38333| 
 38334| 161: ; preds = %158
 38335|  %162 = cleanuppad within %160 []
 38337|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %46) [ "funclet"(token %162) ] ;L825<825<2418
 38338|  unreachable                                                                                                           ;L825<2418
 38339| 
 38340| 163: ; preds = %158
 38342|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %46) [ "funclet"(token %160) ] ;L825<825<2418
 38343|  cleanupret from %160 unwind label %146                                                                                ;L2418
 38344| 
 38345| 164: ; preds = %155
 38348|     ;; self[0..+8] = ptr %57
 38349|     ;; slice[0..+8] = ptr %57
 38350|     ;; self[8..+8] = i64 5
 38351|     ;; slice[8..+8] = i64 5
 38352|     ;; ptr = ptr %57
 38353|     ;; self = ptr %57
 38354|  %165 = gep %57, i64 40                                                                                                ;L961<100<1042<1505
 38355|  %166 = gep %4, i64 16                                                                                                 ;L1506
 38356|  %167 = load ptr, ptr %166, , !!8, !!8                                                                                 ;L1506
 38357|     ;; self[0..+8] = ptr %57
 38358|     ;; self[8..+8] = ptr %165
 38359|     ;; self[16..+8] = i64 0
 38360|     ;; self[24..+8] = ptr %167
 38361|     ;; self[32..+8] = ptr %3
 38362|     ;; self[40..+8] = ptr %59
 38363|  store ptr %57, ptr %40,                                                                                               ;L69<836<1508
 38364|  %168 = gep %40, i64 8                                                                                                 ;L69<836<1508
 38365|  store ptr %165, ptr %168,                                                                                             ;L69<836<1508
 38366|  %169 = gep %40, i64 16                                                                                                ;L69<836<1508
 38367|  store i64 0, ptr %169,                                                                                                ;L69<836<1508
 38368|  %170 = gep %40, i64 24                                                                                                ;L69<836<1508
 38369|  store ptr %167, ptr %170,                                                                                             ;L69<836<1508
 38370|  %171 = gep %40, i64 32                                                                                                ;L69<836<1508
 38371|  store ptr %3, ptr %171,                                                                                               ;L69<836<1508
 38372|  %172 = gep %40, i64 40                                                                                                ;L69<836<1508
 38373|  store ptr %59, ptr %172,                                                                                              ;L69<836<1508
 38374|  invoke void @core::iter8adapters3map3MapINtNtB2Q_6filter6FilterIB2M_INtNtB2Q_9enumerate9EnumerateINtNtNtB2U_5slice4iter4IterINtNtB2U_6option6OptionB28_EEENCNvNtCshdEBA0ozCnw_7game_ai15score_parameter25calculate_score_parameters0_0ENCB5r_s1_0ENCB5r_s2_0EEB5v_(ptr sret([32 x i8]) %41, ptr %40, ptr %65)
 38375|  to label %174 unwind label %158                                                                                       ;L1504
 38376| 
 38377| 173: ; preds = %3302, %3247, %3233, %3221, %3196, %3078, %3055, %3038, %2998, %2806, %2798, %2776, %2622, %2614, %2592, %2431, %2409, %2387, %2371, %2008, %1947, %1937, %1904, %1894, %1861, %1851, %1818, %1808, %1454, %1416, %1406, %1373, %1363, %1330, %1320, %1287, %1277, %747, %709, %666, %643, %600, %577, %534, %511, %468
 38378|  unreachable
 38379| 
 38380| 174: ; preds = %164
 38384|  %175 = getelementptr [5 x ptr], ptr %56, i64 %144                                                                     ;L1512
 38385|     ;; self[0..+8] = ptr %175
 38386|     ;; slice[0..+8] = ptr %175
 38387|     ;; self[8..+8] = i64 5
 38388|     ;; slice[8..+8] = i64 5
 38389|     ;; ptr = ptr %175
 38390|     ;; self = ptr %175
 38391|  %176 = gep %175, i64 40                                                                                               ;L961<100<1042<1512
 38392|  %177 = load ptr, ptr %55, , !!8, !!8                                                                                  ;L1514
 38393|  %178 = gep %55, i64 8                                                                                                 ;L1514
 38394|  %179 = load ptr, ptr %178, , !!8, !!8                                                                                 ;L1514
 38395|     ;; game[0..+8] = ptr %177
 38396|     ;; game[8..+8] = ptr %179
 38397|     ;; self[0..+8] = ptr %175
 38398|     ;; self[8..+8] = ptr %176
 38399|     ;; self[16..+8] = i64 0
 38400|     ;; self[24..+8] = ptr %167
 38401|     ;; self[32..+8] = ptr %3
 38402|     ;; self[40..+8] = ptr %177
 38403|     ;; self[48..+8] = ptr %179
 38404|     ;; self[56..+8] = ptr %167
 38405|     ;; self[64..+8] = ptr %3
 38406|     ;; self[72..+8] = ptr %59
 38407|  store ptr %175, ptr %38,                                                                                              ;L69<836<1517
 38408|  %180 = gep %38, i64 8                                                                                                 ;L69<836<1517
 38409|  store ptr %176, ptr %180,                                                                                             ;L69<836<1517
 38410|  %181 = gep %38, i64 16                                                                                                ;L69<836<1517
 38411|  store i64 0, ptr %181,                                                                                                ;L69<836<1517
 38412|  %182 = gep %38, i64 24                                                                                                ;L69<836<1517
 38413|  store ptr %167, ptr %182,                                                                                             ;L69<836<1517
 38414|  %183 = gep %38, i64 32                                                                                                ;L69<836<1517
 38415|  store ptr %3, ptr %183,                                                                                               ;L69<836<1517
 38416|  %184 = gep %38, i64 40                                                                                                ;L69<836<1517
 38417|  store ptr %177, ptr %184,                                                                                             ;L69<836<1517
 38418|  %185 = gep %38, i64 48                                                                                                ;L69<836<1517
 38419|  store ptr %179, ptr %185,                                                                                             ;L69<836<1517
 38420|  %186 = gep %38, i64 56                                                                                                ;L69<836<1517
 38421|  store ptr %167, ptr %186,                                                                                             ;L69<836<1517
 38422|  %187 = gep %38, i64 64                                                                                                ;L69<836<1517
 38423|  store ptr %3, ptr %187,                                                                                               ;L69<836<1517
 38424|  %188 = gep %38, i64 72                                                                                                ;L69<836<1517
 38425|  store ptr %59, ptr %188,                                                                                              ;L69<836<1517
 38426|  invoke void @core::iter8adapters3map3MapINtNtB2Q_6filter6FilterIB2M_INtNtB2Q_9enumerate9EnumerateINtNtNtB2U_5slice4iter4IterINtNtB2U_6option6OptionB28_EEENCNvNtCshdEBA0ozCnw_7game_ai15score_parameter25calculate_score_parameters3_0ENCB5r_s4_0ENCB5r_s5_0EEB5v_(ptr sret([32 x i8]) %39, ptr %38, ptr %65)
 38427|  to label %195 unwind label %189                                                                                       ;L1511
 38428| 
 38429| 189: ; preds = %3388, %3387, %3385, %246, %174
 38430|  %190 = phi i8 [ %242, %246 ], [ 0, %3388 ], [ 1, %174 ], [ 0, %3387 ], [ 0, %3385 ]                                   ;L0
 38431|  %191 = cleanuppad within none []
 38433|  invoke void @gc::simulation4game10blackboard11SmallActionRNtNtB14_6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %41) [ "funclet"(token %191) ]
 38434|  to label %194 unwind label %192                                                                                       ;L825<2418
 38435| 
 38436| 192: ; preds = %189
 38437|  %193 = cleanuppad within %191 []
 38439|  call void @gc::simulation4game10blackboard11SmallActionRNtNtB1b_6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %41) [ "funclet"(token %193) ] ;L825<825<2418
 38440|  unreachable                                                                                                           ;L825<2418
 38441| 
 38442| 194: ; preds = %189
 38444|  call void @gc::simulation4game10blackboard11SmallActionRNtNtB1b_6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %41) [ "funclet"(token %191) ] ;L825<825<2418
 38445|  cleanupret from %191 unwind label %158                                                                                ;L2418
 38446| 
 38447| 195: ; preds = %174
 38449|     ;; self = ptr %41
 38450|     ;; self = ptr %41
 38451|     ;; self = ptr %41
 38452|  %196 = load ptr, ptr %41, , !!8, !!8                                                                                  ;L138<2073<2136<1521
 38453|     ;; p = ptr %196
 38454|  %197 = gep %41, i64 24                                                                                                ;L2075<2136<1521
 38455|  %198 = load i64, ptr %197, , !!8                                                                                      ;L2075<2136<1521
 38456|     ;; len = i64 %198
 38457|     ;; count = i64 %198
 38458|     ;; self[0..+8] = ptr %196
 38459|     ;; slice[0..+8] = ptr %196
 38460|     ;; self[8..+8] = i64 %198
 38461|     ;; slice[8..+8] = i64 %198
 38462|     ;; ptr = ptr %196
 38463|     ;; self = ptr %196
 38464|  %199 = getelementptr { { i64, [2 x i64] }, i64, ptr }, ptr %196, i64 %198                                             ;L961<100<1042<2136<1521
 38465|     ;; iter[0..+8] = ptr %196
 38466|     ;; iter[8..+8] = ptr %199
 38467|  %200 = gep %37, i64 88
 38468|  %201 = gep %37, i64 96
 38469|  %202 = gep %37, i64 104
 38470|  %203 = gep %37, i64 112
 38471|  %204 = gep %37, i64 24
 38472|  %205 = gep %37, i64 32
 38473|  %206 = gep %37, i64 40
 38474|  %207 = gep %37, i64 152
 38475|  %208 = gep %37, i64 56
 38476|  %209 = gep %37, i64 64
 38477|  %210 = gep %37, i64 72
 38478|  %211 = gep %37, i64 160
 38479|  %212 = gep %37, i64 168
 38480|  br label %213                                                                                                         ;L1521
 38481| 
 38482| 213: ; preds = %286, %195
 38483|  %214 = phi ptr [ %196, %195 ], [ %217, %286 ]                                                                         ;L1521
 38484|     ;; iter[0..+8] = ptr %214
 38485|     ;; self = ptr undef
 38486|     ;; ptr = ptr %214
 38487|     ;; self = ptr %214
 38488|     ;; end_or_len = ptr %199
 38491|  %215 = icmp eq ptr %214, %199                                                                                         ;L1714<180<1521
 38492|  br i1 %215, label %223, label %216                                                                                    ;L180<1521
 38493| 
 38494| 216: ; preds = %213
 38495|  %217 = gep %214, i64 40                                                                                               ;L656<185<1521
 38496|     ;; iter[0..+8] = ptr %217
 38497|     ;; pos = ptr %214
 38498|     ;; a = ptr %214
 38499|     ;; e = ptr %214
 38500|  %218 = gep %214, i64 24                                                                                               ;L1522
 38501|  %219 = load i64, ptr %218, , !!8                                                                                      ;L1522
 38502|     ;; pos = i64 %219
 38503|  %220 = gep %214, i64 32                                                                                               ;L1523
 38504|  %221 = load ptr, ptr %220, , !!8, !!8                                                                                 ;L1523
 38505|  %222 = invoke i64 @gc::simulation6entityNtB5_6Entity18remain_action_time(ptr %221)
 38506|  to label %247 unwind label %241                                                                                       ;L1523
 38507| 
 38508| 223: ; preds = %213
 38509|     ;; self = ptr %39
 38510|     ;; self = ptr %39
 38511|     ;; self = ptr %39
 38512|  %224 = load ptr, ptr %39, , !!8, !!8                                                                                  ;L138<2073<2136<1551
 38513|     ;; p = ptr %224
 38514|  %225 = gep %39, i64 24                                                                                                ;L2075<2136<1551
 38515|  %226 = load i64, ptr %225, , !!8                                                                                      ;L2075<2136<1551
 38516|     ;; len = i64 %226
 38517|     ;; count = i64 %226
 38518|     ;; self[0..+8] = ptr %224
 38519|     ;; slice[0..+8] = ptr %224
 38520|     ;; self[8..+8] = i64 %226
 38521|     ;; slice[8..+8] = i64 %226
 38522|     ;; ptr = ptr %224
 38523|     ;; self = ptr %224
 38524|  %227 = getelementptr { { i64, [2 x i64] }, i64, ptr }, ptr %224, i64 %226                                             ;L961<100<1042<2136<1551
 38525|     ;; iter[0..+8] = ptr %224
 38526|     ;; iter[8..+8] = ptr %227
 38527|  %228 = gep %34, i64 88
 38528|  %229 = gep %34, i64 96
 38529|  %230 = gep %34, i64 104
 38530|  %231 = gep %34, i64 112
 38531|  %232 = gep %34, i64 24
 38532|  %233 = gep %34, i64 32
 38533|  %234 = gep %34, i64 40
 38534|  %235 = gep %34, i64 152
 38535|  %236 = gep %34, i64 56
 38536|  %237 = gep %34, i64 64
 38537|  %238 = gep %34, i64 72
 38538|  %239 = gep %34, i64 160
 38539|  %240 = gep %34, i64 168
 38540|  br label %294                                                                                                         ;L1551
 38541| 
 38542| 241: ; preds = %3383, %3382, %3380, %370, %355, %352, %351, %350, %319, %304, %297, %293, %292, %261, %216
 38543|  %242 = phi i8 [ %366, %370 ], [ 0, %3383 ], [ 1, %355 ], [ 1, %352 ], [ 0, %3380 ], [ 1, %304 ], [ 1, %351 ], [ 1, %350 ], [ 1, %261 ], [ 1, %319 ], [ 1, %297 ], [ 1, %293 ], [ 1, %292 ], [ 1, %216 ], [ 0, %3382 ] ;L0
 38544|  %243 = cleanuppad within none []
 38546|  invoke void @gc::simulation4game10blackboard11SmallActionRNtNtB14_6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %39) [ "funclet"(token %243) ]
 38547|  to label %246 unwind label %244                                                                                       ;L825<2418
 38548| 
 38549| 244: ; preds = %241
 38550|  %245 = cleanuppad within %243 []
 38552|  call void @gc::simulation4game10blackboard11SmallActionRNtNtB1b_6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %39) [ "funclet"(token %245) ] ;L825<825<2418
 38553|  unreachable                                                                                                           ;L825<2418
 38554| 
 38555| 246: ; preds = %241
 38557|  call void @gc::simulation4game10blackboard11SmallActionRNtNtB1b_6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %39) [ "funclet"(token %243) ] ;L825<825<2418
 38558|  cleanupret from %243 unwind label %189                                                                                ;L2418
 38559| 
 38560| 247: ; preds = %216
 38561|     ;; self = ptr %221
 38562|     ;; self = ptr %221
 38563|     ;; self = ptr %221
 38564|  %248 = gep %221, i64 712                                                                                              ;L614<609<296<1968<1864<3787<1523
 38565|  %249 = load ptr, ptr %248, , !!8, !!8                                                                                 ;L614<609<296<1968<1864<3787<1523
 38566|  %250 = gep %221, i64 720                                                                                              ;L1864<3787<1523
 38567|  %251 = load i64, ptr %250, , !!8                                                                                      ;L1864<3787<1523
 38568|     ;; len = i64 %251
 38569|     ;; count = i64 %251
 38570|     ;; self[0..+8] = ptr %249
 38571|     ;; slice[0..+8] = ptr %249
 38572|     ;; self[8..+8] = i64 %251
 38573|     ;; slice[8..+8] = i64 %251
 38574|     ;; ptr = ptr %249
 38575|     ;; self = ptr %249
 38576|  %252 = gepS %249, i64 %251                                                                                            ;L961<100<1042<1523
 38577|     ;; self[0..+8] = ptr %249
 38578|     ;; self[0..+8] = ptr %249
 38579|     ;; self[8..+8] = ptr %252
 38580|     ;; self[8..+8] = ptr %252
 38581|     ;; self[0..+8] = ptr %249
 38582|     ;; self[8..+8] = ptr %252
 38584|     ;; self = ptr undef
 38585|     ;; self = ptr undef
 38586|     ;; predicate = ptr undef
 38587|     ;; self = ptr undef
 38588|     ;; self = ptr undef
 38589|     ;; count = i64 1
 38590|  br label %253                                                                                                         ;L348<98<107<2706<3354<3255<1524
 38591| 
 38592| 253: ; preds = %256, %247
 38593|  %254 = phi ptr [ %257, %256 ], [ %249, %247 ]
 38595|     ;; ptr = ptr %254
 38596|     ;; self = ptr %254
 38597|     ;; end_or_len = ptr %252
 38600|  %255 = icmp eq ptr %254, %252                                                                                         ;L1714<180<348<98<107<2706<3354<3255<1524
 38601|  br i1 %255, label %267, label %256                                                                                    ;L180<348<98<107<2706<3354<3255<1524
 38602| 
 38603| 256: ; preds = %253
 38604|  %257 = gep %254, i64 40                                                                                               ;L656<185<348<98<107<2706<3354<3255<1524
 38605|     ;; self[0..+8] = ptr %257
 38606|     ;; x = ptr %254
 38611|     ;; self = ptr %254
 38612|  %258 = load i32, ptr %254, , !!53327, !!8                                                                             ;L521<1523<298<349<98<107<2706<3354<3255<1524
 38613|  %259 = add nsw i32 %258, -6                                                                                           ;L521<1523<298<349<98<107<2706<3354<3255<1524
 38614|  %260 = icmp ult i32 %259, -4                                                                                          ;L521<1523<298<349<98<107<2706<3354<3255<1524
 38615|  br i1 %260, label %261, label %253                                                                                    ;L349<98<107<2706<3354<3255<1524
 38616| 
 38617| 261: ; preds = %256
 38618|     ;; self = ptr %254
 38619|     ;; f = ptr undef
 38620|     ;; self = ptr undef
 38621|     ;; x = ptr %254
 38622|     ;; args = ptr %254
 38624|     ;; c = ptr %254
 38625|     ;; self = ptr %254
 38626|  %262 = icmp eq i32 %258, 10                                                                                           ;L546<1524<310<1162<107<2706<3354<3255<1524
 38627|  %263 = select i1 %262, i64 32, i64 8                                                                                  ;L0<1524<310<1162<107<2706<3354<3255<1524
 38628|  %264 = gep %254, i64 %263                                                                                             ;L0<1524<310<1162<107<2706<3354<3255<1524
 38629|  %265 = load i64, ptr %264, , !!53403, !!8                                                                             ;L0<1524<310<1162<107<2706<3354<3255<1524
 38630|     ;; self[0..+8] = ptr %257
 38631|     ;; first = i64 %265
 38632|  %266 = invoke i64 @core::iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtNtBc_5slice4iter4IterNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity7CCStateENCNvNtCshdEBA0ozCnw_7game_ai15score_parameter25calculate_score_parameters6_0ENCB2H_s7_0ENtNtNtBa_6traits8iterator8Iterator4foldjNCINvNvB45_6max_by4foldjNvYjNtNtBc_3cmp3Ord3cmpE0EB2L_(ptr %257, ptr %252, i64 %265)
 38633|  to label %267 unwind label %241                                                                                       ;L2707<3354<3255<1524
 38634| 
 38635| 267: ; preds = %261, %253
 38636|  %268 = phi i64 [ %266, %261 ], [ undef, %253 ]
 38639|     ;; self = i64 %222
 38641|  %269 = call i64 @llvm.umax.i64(i64 %268, i64 %222)                                                                    ;L1039<1523
 38642|  %270 = select i1 %255, i64 %222, i64 %269                                                                             ;L1039<1524
 38643|     ;; act_tick = i64 %270
 38645|  %271 = gep %221, i64 1472                                                                                             ;L1527
 38646|  %272 = load i64, ptr %271, , !!8                                                                                      ;L1527
 38648|  call void @llvm.memcpy.p0.p0.i64(ptr %36, ptr %214, i64 24, i1 false)                                                 ;L1541
 38649|  store i64 %272, ptr %200,                                                                                             ;L1526
 38650|  store i64 %49, ptr %201,                                                                                              ;L1526
 38651|  store i64 %219, ptr %202,                                                                                             ;L1526
 38652|  call void @llvm.memset.p0.i64(ptr %203, i8 0, i64 40, i1 false)                                                       ;L1526
 38653|  store ptr inttoptr (i64 8 to ptr), ptr %204,                                                                          ;L1526
 38654|  store ptr %65, ptr %205,                                                                                              ;L1526
 38655|  store i64 0, ptr %207,                                                                                                ;L1526
 38656|  call void @llvm.memset.p0.i64(ptr %206, i8 0, i64 16, i1 false)                                                       ;L1526
 38657|  store ptr inttoptr (i64 8 to ptr), ptr %208,                                                                          ;L1526
 38658|  store ptr %65, ptr %209,                                                                                              ;L1526
 38659|  call void @llvm.memset.p0.i64(ptr %210, i8 0, i64 16, i1 false)                                                       ;L1526
 38660|  store i64 %270, ptr %211,                                                                                             ;L1526
 38661|  call void @llvm.memcpy.p0.p0.i64(ptr %37, ptr %36, i64 24, i1 false)                                                  ;L1526
 38662|  call void @llvm.memset.p0.i64(ptr %212, i8 0, i64 48, i1 false)                                                       ;L1526
 38664|  invoke void @ai::utils26precompute_champion_powers(i64 %1, ptr %4, ptr %37)
 38665|  to label %276 unwind label %273                                                                                       ;L1547
 38666| 
 38667| 273: ; preds = %285, %283, %267
 38668|  %274 = phi i1 [ false, %283 ], [ true, %267 ], [ false, %285 ]                                                        ;L0
 38669|  %275 = cleanuppad within none []
 38670|  br i1 %274, label %293, label %292                                                                                    ;L1549
 38671| 
 38672| 276: ; preds = %267
 38674|  call void @llvm.memcpy.p0.p0.i64(ptr %35, ptr %37, i64 216, i1 false)                                                 ;L1548
 38675|     ;; self = ptr %67
 38676|     ;; self = ptr %67
 38677|     ;; value = ptr %35
 38679|     ;; additional = i64 1
 38680|     ;; needed_extra_cap = i64 1
 38681|     ;; needed_extra_cap = i64 1
 38682|     ;; strategy = i8 1
 38683|  %277 = load i64, ptr %70, , !!53472, !!8                                                                              ;L1428<1548
 38684|     ;; self = ptr %67
 38685|  %278 = load i64, ptr %69, , !!53472, !!8                                                                              ;L149<1428<1548
 38686|  %279 = icmp eq i64 %277, %278                                                                                         ;L1428<1548
 38687|  br i1 %279, label %280, label %286                                                                                    ;L1428<1548
 38688| 
 38689| 280: ; preds = %276
 38690|     ;; self = ptr %67
 38691|     ;; self = ptr %67
 38692|     ;; self = ptr %67
 38693|     ;; used_cap = i64 %277
 38694|     ;; used_cap = i64 %277
 38695|  invoke void @ai::score_parameter22ChampionScoreParameterE25reserve_internal_or_panicB17_(ptr %67, i64 %277, i64 1, i1 zeroext true)
 38696|  to label %281 unwind label %283, !!53472                                                                              ;L619<430<738<1429<1548
 38697| 
 38698| 281: ; preds = %280
 38699|  %282 = load i64, ptr %70, , !!53472                                                                                   ;L1432<1548
 38700|  br label %286                                                                                                         ;L619<430<738<1429<1548
 38701| 
 38702| 283: ; preds = %280
 38703|  %284 = cleanuppad within none []
 38704|  invoke fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai15score_parameter22ChampionScoreParameterEBF_(ptr %35) #27 [ "funclet"(token %284) ]
 38705|  to label %285 unwind label %273                                                                                       ;L1436<1548
 38706| 
 38707| 285: ; preds = %283
 38708|  cleanupret from %284 unwind label %273
 38709| 
 38710| 286: ; preds = %281, %276
 38711|  %287 = phi i64 [ %282, %281 ], [ %277, %276 ]                                                                         ;L1432<1548
 38712|     ;; self = ptr %67
 38713|  %288 = load ptr, ptr %67, , !!53472, !!8, !!8                                                                         ;L138<1432<1548
 38714|     ;; self = ptr %288
 38715|     ;; count = i64 %287
 38716|  %289 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %288, i64 %287 ;L961<1432<1548
 38717|     ;; end = ptr %289
 38718|     ;; dst = ptr %289
 38719|  call void @llvm.memcpy.p0.p0.i64(ptr %289, ptr %35, i64 216, i1 false)                                                ;L1933<1433<1548
 38720|  %290 = load i64, ptr %70, , !!53472, !!8                                                                              ;L1434<1548
 38721|  %291 = add i64 %290, 1                                                                                                ;L1434<1548
 38722|  store i64 %291, ptr %70, , !!53472                                                                                    ;L1434<1548
 38725|  br label %213                                                                                                         ;L1521
 38726| 
 38727| 292: ; preds = %273
 38728|  cleanupret from %275 unwind label %241
 38729| 
 38730| 293: ; preds = %273
 38731|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai15score_parameter22ChampionScoreParameterEBF_(ptr %37) #27 [ "funclet"(token %275) ] ;L1549
 38732|  cleanupret from %275 unwind label %241                                                                                ;L1549
 38733| 
 38734| 294: ; preds = %344, %223
 38735|  %295 = phi ptr [ %224, %223 ], [ %298, %344 ]                                                                         ;L1551
 38736|     ;; iter[0..+8] = ptr %295
 38737|     ;; self = ptr undef
 38738|     ;; ptr = ptr %295
 38739|     ;; self = ptr %295
 38740|     ;; end_or_len = ptr %227
 38743|  %296 = icmp eq ptr %295, %227                                                                                         ;L1714<180<1551
 38744|  br i1 %296, label %304, label %297                                                                                    ;L180<1551
 38745| 
 38746| 297: ; preds = %294
 38747|  %298 = gep %295, i64 40                                                                                               ;L656<185<1551
 38748|     ;; iter[0..+8] = ptr %298
 38749|     ;; pos = ptr %295
 38750|     ;; a = ptr %295
 38751|     ;; e = ptr %295
 38752|  %299 = gep %295, i64 24                                                                                               ;L1552
 38753|  %300 = load i64, ptr %299, , !!8                                                                                      ;L1552
 38754|     ;; pos = i64 %300
 38755|  %301 = gep %295, i64 32                                                                                               ;L1554
 38756|  %302 = load ptr, ptr %301, , !!8, !!8                                                                                 ;L1554
 38757|  %303 = invoke i64 @gc::simulation6entityNtB5_6Entity18remain_action_time(ptr %302)
 38758|  to label %305 unwind label %241                                                                                       ;L1554
 38759| 
 38760| 304: ; preds = %294
 38765|  invoke void @gc::simulationNtB5_21AbstractGameWithCache25iter_towers_without_nexus(ptr sret([120 x i8]) %28, ptr %55, i64 %144)
 38766|  to label %352 unwind label %241                                                                                       ;L1583
 38767| 
 38768| 305: ; preds = %297
 38769|     ;; self = ptr %302
 38770|     ;; self = ptr %302
 38771|     ;; self = ptr %302
 38772|  %306 = gep %302, i64 712                                                                                              ;L614<609<296<1968<1864<3787<1554
 38773|  %307 = load ptr, ptr %306, , !!8, !!8                                                                                 ;L614<609<296<1968<1864<3787<1554
 38774|  %308 = gep %302, i64 720                                                                                              ;L1864<3787<1554
 38775|  %309 = load i64, ptr %308, , !!8                                                                                      ;L1864<3787<1554
 38776|     ;; len = i64 %309
 38777|     ;; count = i64 %309
 38778|     ;; self[0..+8] = ptr %307
 38779|     ;; slice[0..+8] = ptr %307
 38780|     ;; self[8..+8] = i64 %309
 38781|     ;; slice[8..+8] = i64 %309
 38782|     ;; ptr = ptr %307
 38783|     ;; self = ptr %307
 38784|  %310 = gepS %307, i64 %309                                                                                            ;L961<100<1042<1554
 38785|     ;; self[0..+8] = ptr %307
 38786|     ;; self[0..+8] = ptr %307
 38787|     ;; self[8..+8] = ptr %310
 38788|     ;; self[8..+8] = ptr %310
 38789|     ;; self[0..+8] = ptr %307
 38790|     ;; self[8..+8] = ptr %310
 38792|     ;; self = ptr undef
 38793|     ;; self = ptr undef
 38794|     ;; predicate = ptr undef
 38795|     ;; self = ptr undef
 38796|     ;; self = ptr undef
 38797|     ;; count = i64 1
 38798|  br label %311                                                                                                         ;L348<98<107<2706<3354<3255<1555
 38799| 
 38800| 311: ; preds = %314, %305
 38801|  %312 = phi ptr [ %315, %314 ], [ %307, %305 ]
 38803|     ;; ptr = ptr %312
 38804|     ;; self = ptr %312
 38805|     ;; end_or_len = ptr %310
 38808|  %313 = icmp eq ptr %312, %310                                                                                         ;L1714<180<348<98<107<2706<3354<3255<1555
 38809|  br i1 %313, label %325, label %314                                                                                    ;L180<348<98<107<2706<3354<3255<1555
 38810| 
 38811| 314: ; preds = %311
 38812|  %315 = gep %312, i64 40                                                                                               ;L656<185<348<98<107<2706<3354<3255<1555
 38813|     ;; self[0..+8] = ptr %315
 38814|     ;; x = ptr %312
 38819|     ;; self = ptr %312
 38820|  %316 = load i32, ptr %312, , !!53644, !!8                                                                             ;L521<1554<298<349<98<107<2706<3354<3255<1555
 38821|  %317 = add nsw i32 %316, -6                                                                                           ;L521<1554<298<349<98<107<2706<3354<3255<1555
 38822|  %318 = icmp ult i32 %317, -4                                                                                          ;L521<1554<298<349<98<107<2706<3354<3255<1555
 38823|  br i1 %318, label %319, label %311                                                                                    ;L349<98<107<2706<3354<3255<1555
 38824| 
 38825| 319: ; preds = %314
 38826|     ;; self = ptr %312
 38827|     ;; f = ptr undef
 38828|     ;; self = ptr undef
 38829|     ;; x = ptr %312
 38830|     ;; args = ptr %312
 38832|     ;; c = ptr %312
 38833|     ;; self = ptr %312
 38834|  %320 = icmp eq i32 %316, 10                                                                                           ;L546<1555<310<1162<107<2706<3354<3255<1555
 38835|  %321 = select i1 %320, i64 32, i64 8                                                                                  ;L0<1555<310<1162<107<2706<3354<3255<1555
 38836|  %322 = gep %312, i64 %321                                                                                             ;L0<1555<310<1162<107<2706<3354<3255<1555
 38837|  %323 = load i64, ptr %322, , !!53713, !!8                                                                             ;L0<1555<310<1162<107<2706<3354<3255<1555
 38838|     ;; self[0..+8] = ptr %315
 38839|     ;; first = i64 %323
 38840|  %324 = invoke i64 @core::iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtNtBc_5slice4iter4IterNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity7CCStateENCNvNtCshdEBA0ozCnw_7game_ai15score_parameter25calculate_score_parameters8_0ENCB2H_s9_0ENtNtNtBa_6traits8iterator8Iterator4foldjNCINvNvB45_6max_by4foldjNvYjNtNtBc_3cmp3Ord3cmpE0EB2L_(ptr %315, ptr %310, i64 %323)
 38841|  to label %325 unwind label %241                                                                                       ;L2707<3354<3255<1555
 38842| 
 38843| 325: ; preds = %319, %311
 38844|  %326 = phi i64 [ %324, %319 ], [ undef, %311 ]
 38847|     ;; self = i64 %303
 38849|  %327 = call i64 @llvm.umax.i64(i64 %326, i64 %303)                                                                    ;L1039<1554
 38850|  %328 = select i1 %313, i64 %303, i64 %327                                                                             ;L1039<1555
 38851|     ;; act_tick = i64 %328
 38853|  %329 = gep %302, i64 1472                                                                                             ;L1558
 38854|  %330 = load i64, ptr %329, , !!8                                                                                      ;L1558
 38856|  call void @llvm.memcpy.p0.p0.i64(ptr %33, ptr %295, i64 24, i1 false)                                                 ;L1572
 38857|  store i64 %330, ptr %228,                                                                                             ;L1557
 38858|  store i64 %144, ptr %229,                                                                                             ;L1557
 38859|  store i64 %300, ptr %230,                                                                                             ;L1557
 38860|  call void @llvm.memset.p0.i64(ptr %231, i8 0, i64 40, i1 false)                                                       ;L1557
 38861|  store ptr inttoptr (i64 8 to ptr), ptr %232,                                                                          ;L1557
 38862|  store ptr %65, ptr %233,                                                                                              ;L1557
 38863|  store i64 0, ptr %235,                                                                                                ;L1557
 38864|  call void @llvm.memset.p0.i64(ptr %234, i8 0, i64 16, i1 false)                                                       ;L1557
 38865|  store ptr inttoptr (i64 8 to ptr), ptr %236,                                                                          ;L1557
 38866|  store ptr %65, ptr %237,                                                                                              ;L1557
 38867|  call void @llvm.memset.p0.i64(ptr %238, i8 0, i64 16, i1 false)                                                       ;L1557
 38868|  store i64 %328, ptr %239,                                                                                             ;L1557
 38869|  call void @llvm.memcpy.p0.p0.i64(ptr %34, ptr %33, i64 24, i1 false)                                                  ;L1557
 38870|  call void @llvm.memset.p0.i64(ptr %240, i8 0, i64 48, i1 false)                                                       ;L1557
 38872|  invoke void @ai::utils26precompute_champion_powers(i64 %1, ptr %4, ptr %34)
 38873|  to label %334 unwind label %331                                                                                       ;L1578
 38874| 
 38875| 331: ; preds = %343, %341, %325
 38876|  %332 = phi i1 [ false, %341 ], [ true, %325 ], [ false, %343 ]                                                        ;L0
 38877|  %333 = cleanuppad within none []
 38878|  br i1 %332, label %351, label %350                                                                                    ;L1580
 38879| 
 38880| 334: ; preds = %325
 38882|  call void @llvm.memcpy.p0.p0.i64(ptr %32, ptr %34, i64 216, i1 false)                                                 ;L1579
 38883|     ;; self = ptr %71
 38884|     ;; self = ptr %71
 38885|     ;; value = ptr %32
 38887|     ;; additional = i64 1
 38888|     ;; needed_extra_cap = i64 1
 38889|     ;; needed_extra_cap = i64 1
 38890|     ;; strategy = i8 1
 38891|  %335 = load i64, ptr %74, , !!53743, !!8                                                                              ;L1428<1579
 38892|     ;; self = ptr %71
 38893|  %336 = load i64, ptr %73, , !!53743, !!8                                                                              ;L149<1428<1579
 38894|  %337 = icmp eq i64 %335, %336                                                                                         ;L1428<1579
 38895|  br i1 %337, label %338, label %344                                                                                    ;L1428<1579
 38896| 
 38897| 338: ; preds = %334
 38898|     ;; self = ptr %71
 38899|     ;; self = ptr %71
 38900|     ;; self = ptr %71
 38901|     ;; used_cap = i64 %335
 38902|     ;; used_cap = i64 %335
 38903|  invoke void @ai::score_parameter22ChampionScoreParameterE25reserve_internal_or_panicB17_(ptr %71, i64 %335, i64 1, i1 zeroext true)
 38904|  to label %339 unwind label %341, !!53743                                                                              ;L619<430<738<1429<1579
 38905| 
 38906| 339: ; preds = %338
 38907|  %340 = load i64, ptr %74, , !!53743                                                                                   ;L1432<1579
 38908|  br label %344                                                                                                         ;L619<430<738<1429<1579
 38909| 
 38910| 341: ; preds = %338
 38911|  %342 = cleanuppad within none []
 38912|  invoke fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai15score_parameter22ChampionScoreParameterEBF_(ptr %32) #27 [ "funclet"(token %342) ]
 38913|  to label %343 unwind label %331                                                                                       ;L1436<1579
 38914| 
 38915| 343: ; preds = %341
 38916|  cleanupret from %342 unwind label %331
 38917| 
 38918| 344: ; preds = %339, %334
 38919|  %345 = phi i64 [ %340, %339 ], [ %335, %334 ]                                                                         ;L1432<1579
 38920|     ;; self = ptr %71
 38921|  %346 = load ptr, ptr %71, , !!53743, !!8, !!8                                                                         ;L138<1432<1579
 38922|     ;; self = ptr %346
 38923|     ;; count = i64 %345
 38924|  %347 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %346, i64 %345 ;L961<1432<1579
 38925|     ;; end = ptr %347
 38926|     ;; dst = ptr %347
 38927|  call void @llvm.memcpy.p0.p0.i64(ptr %347, ptr %32, i64 216, i1 false)                                                ;L1933<1433<1579
 38928|  %348 = load i64, ptr %74, , !!53743, !!8                                                                              ;L1434<1579
 38929|  %349 = add i64 %348, 1                                                                                                ;L1434<1579
 38930|  store i64 %349, ptr %74, , !!53743                                                                                    ;L1434<1579
 38933|  br label %294                                                                                                         ;L1551
 38934| 
 38935| 350: ; preds = %331
 38936|  cleanupret from %333 unwind label %241
 38937| 
 38938| 351: ; preds = %331
 38939|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai15score_parameter22ChampionScoreParameterEBF_(ptr %34) #27 [ "funclet"(token %333) ] ;L1580
 38940|  cleanupret from %333 unwind label %241                                                                                ;L1580
 38941| 
 38942| 352: ; preds = %304
 38943|     ;; predicate[0..+8] = ptr %59
 38944|     ;; predicate[8..+8] = ptr %39
 38945|  %353 = load i64, ptr %28,                                                                                             ;L28<957<1583
 38946|     ;; self[0..+8] = i64 %353
 38947|  %354 = gep %28, i64 8                                                                                                 ;L28<957<1583
 38948|  call void @llvm.memcpy.p0.p0.i64(ptr %29, ptr %354, i64 112, i1 false)                                                ;L28<957<1583
 38949|     ;; self[120..+8] = ptr %59
 38950|     ;; self[128..+8] = ptr %39
 38953|  invoke void @gc::simulationNtB5_21AbstractGameWithCache25iter_towers_without_nexus(ptr sret([120 x i8]) %27, ptr %55, i64 %49)
 38954|  to label %355 unwind label %241                                                                                       ;L1586
 38955| 
 38956| 355: ; preds = %352
 38957|     ;; predicate[0..+8] = ptr %59
 38958|     ;; predicate[8..+8] = ptr %41
 38959|  %356 = gep %30, i64 136                                                                                               ;L37<515<1585
 38960|  call void @llvm.memcpy.p0.p0.i64(ptr %356, ptr %27, i64 120, i1 false)                                                ;L28<957<1586
 38961|     ;; other[120..+8] = ptr %59
 38962|     ;; b[120..+8] = ptr %59
 38963|     ;; other[128..+8] = ptr %41
 38964|     ;; b[128..+8] = ptr %41
 38969|     ;; a[0..+8] = i64 %353
 38970|  %357 = gep %30, i64 8                                                                                                 ;L37<515<1585
 38971|  call void @llvm.memcpy.p0.p0.i64(ptr %357, ptr %29, i64 112, i1 false), !!53782                                       ;L515<1585
 38972|  %358 = gep %30, i64 120                                                                                               ;L515<1585
 38973|  store ptr %59, ptr %358, , !!53782                                                                                    ;L515<1585
 38974|  %359 = gep %30, i64 128                                                                                               ;L515<1585
 38975|  store ptr %39, ptr %359, , !!53782                                                                                    ;L515<1585
 38976|  store i64 %353, ptr %30, , !!53788                                                                                    ;L37<515<1585
 38977|  %360 = gep %30, i64 256                                                                                               ;L37<515<1585
 38978|  store ptr %59, ptr %360, , !!53779                                                                                    ;L37<515<1585
 38979|  %361 = gep %30, i64 264                                                                                               ;L37<515<1585
 38980|  store ptr %41, ptr %361, , !!53779                                                                                    ;L37<515<1585
 38982|  invoke void @core::iter8adapters5chain5ChainINtNtB29_6filter6FilterIB25_INtNtB29_7flatten7FlattenINtNtNtB2d_5array4iter8IntoIterINtNtB2d_6option6OptionBU_EKj6_EEINtNtB29_6copied6CopiedINtNtNtB2d_5slice4iter4IterBU_EEENCNvNtCshdEBA0ozCnw_7game_ai15score_parameter25calculate_score_parametersa_0EIB2X_B3j_NCB5L_sb_0EEEB5P_(ptr sret([32 x i8]) %31, ptr %30, ptr %65)
 38983|  to label %362 unwind label %241                                                                                       ;L1582
 38984| 
 38985| 362: ; preds = %355
 38987|  %363 = gep %179, i64 528                                                                                              ;L1593
 38988|  %364 = load ptr, ptr %363, , !!8                                                                                      ;L1593
 38989|  invoke void %364(ptr sret([40 x i8]) %26, ptr %177)
 38990|  to label %371 unwind label %365                                                                                       ;L1593
 38991| 
 38992| 365: ; preds = %3651, %3645, %3620, %3618, %3600, %3579, %3573, %3571, %3563, %3551, %3524, %3520, %3515, %3506, %3500, %3498, %3474, %3462, %3426, %3415, %3378, %3377, %3375, %3197, %3135, %3130, %3097, %3078, %3075, %3055, %3052, %3043, %3038, %3035, %3014, %2998, %2995, %2947, %2932, %2926, %2921, %2915, %2898, %2893, %2816, %2810, %2806, %2803, %2798, %2795, %2781, %2780, %2776, %2773, %2769, %2761, %2641, %2632, %2626, %2622, %2619, %2614, %2611, %2597, %2596, %2592, %2589, %2585, %2577, %2450, %2441, %2435, %2431, %2428, %2414, %2413, %2409, %2406, %2392, %2391, %2387, %2384, %2371, %2366, %2362, %2353, %2227, %2214, %2205, %2171, %2167, %2146, %2137, %2105, %2101, %2083, %2074, %2008, %2001, %1971, %1958, %1952, %1947, %1938, %1937, %1915, %1909, %1904, %1895, %1894, %1872, %1866, %1861, %1852, %1851, %1829, %1823, %1818, %1809, %1808, %1725, %1704, %1690, %1680, %1652, %1649, %1638, %1628, %1600, %1594, %1583, %1573, %1532, %1524, %1454, %1427, %1421, %1416, %1407, %1406, %1384, %1378, %1373, %1364, %1363, %1341, %1335, %1330, %1321, %1320, %1298, %1292, %1287, %1278, %1277, %1220, %1204, %1181, %1174, %1146, %1142, %1123, %1116, %1088, %1084, %1065, %1058, %1004, %994, %966, %963, %952, %942, %914, %908, %897, %887, %846, %825, %819, %747, %720, %714, %709, %700, %692, %686, %666, %654, %648, %643, %634, %626, %620, %600, %588, %582, %577, %568, %560, %554, %534, %522, %516, %511, %502, %494, %488, %468, %421, %405, %381, %377, %362
 38993|  %366 = phi i8 [ 1, %3651 ], [ 1, %3645 ], [ 1, %3620 ], [ 1, %3618 ], [ 1, %3600 ], [ 1, %3520 ], [ 1, %3579 ], [ 1, %3563 ], [ 1, %3573 ], [ 1, %3571 ], [ 1, %3551 ], [ 1, %3524 ], [ 1, %3515 ], [ 1, %3506 ], [ 1, %3474 ], [ 0, %3375 ], [ 1, %3500 ], [ 1, %3498 ], [ 1, %3462 ], [ 1, %3426 ], [ 1, %3415 ], [ 1, %381 ], [ 0, %3378 ], [ 1, %3197 ], [ 1, %3135 ], [ 1, %3130 ], [ 1, %3097 ], [ 1, %2947 ], [ 1, %3052 ], [ 1, %3055 ], [ 1, %3043 ], [ 1, %747 ], [ 1, %3035 ], [ 1, %3038 ], [ 1, %3014 ], [ 1, %522 ], [ 1, %2995 ], [ 1, %2998 ], [ 1, %3075 ], [ 1, %3078 ], [ 1, %2932 ], [ 1, %2926 ], [ 1, %2921 ], [ 1, %2391 ], [ 1, %2915 ], [ 1, %2898 ], [ 1, %2780 ], [ 1, %2893 ], [ 1, %405 ], [ 1, %2810 ], [ 1, %714 ], [ 1, %2773 ], [ 1, %2776 ], [ 1, %2803 ], [ 1, %2806 ], [ 1, %2795 ], [ 1, %2798 ], [ 1, %2781 ], [ 1, %2769 ], [ 1, %2761 ], [ 1, %2816 ], [ 1, %819 ], [ 1, %2596 ], [ 1, %2641 ], [ 1, %2626 ], [ 1, %709 ], [ 1, %2589 ], [ 1, %2592 ], [ 1, %2619 ], [ 1, %2622 ], [ 1, %2611 ], [ 1, %2614 ], [ 1, %2597 ], [ 1, %2585 ], [ 1, %2577 ], [ 1, %2632 ], [ 1, %700 ], [ 1, %2413 ], [ 1, %2450 ], [ 1, %2435 ], [ 0, %3377 ], [ 1, %2384 ], [ 1, %2387 ], [ 1, %516 ], [ 1, %377 ], [ 1, %2371 ], [ 1, %846 ], [ 1, %2406 ], [ 1, %2409 ], [ 1, %2392 ], [ 1, %2366 ], [ 1, %2428 ], [ 1, %2431 ], [ 1, %2414 ], [ 1, %2362 ], [ 1, %2353 ], [ 1, %2441 ], [ 1, %692 ], [ 1, %511 ], [ 1, %2227 ], [ 1, %2214 ], [ 1, %2205 ], [ 1, %2171 ], [ 1, %2167 ], [ 1, %502 ], [ 1, %2146 ], [ 1, %2137 ], [ 1, %2105 ], [ 1, %2101 ], [ 1, %421 ], [ 1, %2083 ], [ 1, %2074 ], [ 1, %494 ], [ 1, %2001 ], [ 1, %2008 ], [ 1, %1971 ], [ 1, %1829 ], [ 1, %1823 ], [ 1, %1818 ], [ 1, %1809 ], [ 1, %582 ], [ 1, %1808 ], [ 1, %1872 ], [ 1, %1866 ], [ 1, %1861 ], [ 1, %1852 ], [ 1, %588 ], [ 1, %1851 ], [ 1, %1915 ], [ 1, %1909 ], [ 1, %1904 ], [ 1, %1895 ], [ 1, %468 ], [ 1, %1894 ], [ 1, %1958 ], [ 1, %1952 ], [ 1, %1947 ], [ 1, %1938 ], [ 1, %488 ], [ 1, %1937 ], [ 1, %577 ], [ 1, %1725 ], [ 1, %1704 ], [ 1, %686 ], [ 1, %1690 ], [ 1, %1680 ], [ 1, %1652 ], [ 1, %1649 ], [ 1, %1638 ], [ 1, %1628 ], [ 1, %1600 ], [ 1, %1594 ], [ 1, %1583 ], [ 1, %1573 ], [ 1, %1532 ], [ 1, %1524 ], [ 1, %568 ], [ 1, %560 ], [ 1, %666 ], [ 1, %554 ], [ 1, %1454 ], [ 1, %1298 ], [ 1, %1292 ], [ 1, %1287 ], [ 1, %1278 ], [ 1, %643 ], [ 1, %1277 ], [ 1, %1341 ], [ 1, %1335 ], [ 1, %1330 ], [ 1, %1321 ], [ 1, %648 ], [ 1, %1320 ], [ 1, %1384 ], [ 1, %1378 ], [ 1, %1373 ], [ 1, %1364 ], [ 1, %654 ], [ 1, %1363 ], [ 1, %1427 ], [ 1, %1421 ], [ 1, %1416 ], [ 1, %1407 ], [ 1, %534 ], [ 1, %1406 ], [ 1, %634 ], [ 1, %1220 ], [ 1, %1204 ], [ 1, %626 ], [ 1, %1181 ], [ 1, %1174 ], [ 1, %1146 ], [ 1, %1142 ], [ 1, %620 ], [ 1, %1123 ], [ 1, %1116 ], [ 1, %1088 ], [ 1, %1084 ], [ 1, %600 ], [ 1, %1065 ], [ 1, %1058 ], [ 1, %720 ], [ 1, %825 ], [ 1, %362 ], [ 1, %1004 ], [ 1, %994 ], [ 1, %966 ], [ 1, %963 ], [ 1, %952 ], [ 1, %942 ], [ 1, %914 ], [ 1, %908 ], [ 1, %897 ], [ 1, %887 ] ;L0
 38994|  %367 = cleanuppad within none []
 38996|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %31) [ "funclet"(token %367) ]
 38997|  to label %370 unwind label %368                                                                                       ;L825<2418
 38998| 
 38999| 368: ; preds = %365
 39000|  %369 = cleanuppad within %367 []
 39002|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %31) [ "funclet"(token %369) ] ;L825<825<2418
 39003|  unreachable                                                                                                           ;L825<2418
 39004| 
 39005| 370: ; preds = %365
 39007|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %31) [ "funclet"(token %367) ] ;L825<825<2418
 39008|  cleanupret from %367 unwind label %241                                                                                ;L2418
 39009| 
 39010| 371: ; preds = %362
 39012|  call void @llvm.memcpy.p0.p0.i64(ptr %25, ptr %26, i64 40, i1 false)                                                  ;L1593
 39013|  %372 = gep %179, i64 496
 39014|  %373 = gep %59, i64 1632
 39015|  %374 = gep %59, i64 1640
 39016|  %375 = gep %59, i64 1136
 39017|  %376 = gep %59, i64 1664
 39018|  br label %377                                                                                                         ;L1593
 39019| 
 39020| 377: ; preds = %3591, %371
 39021|  %378 = invoke ptr @gc::simulationNtB5_14ProjectileIterNtNtNtNtCsjihNppCmMEE_4core4iter6traits8iterator8Iterator4next(ptr %25)
 39022|  to label %379 unwind label %365                                                                                       ;L1593
 39023| 
 39024| 379: ; preds = %377
 39025|  %380 = icmp eq ptr %378, null                                                                                         ;L1593
 39026|  br i1 %380, label %386, label %381                                                                                    ;L1593
 39027| 
 39028| 381: ; preds = %379
 39029|     ;; proj = ptr %378
 39030|  %382 = gep %378, i64 248                                                                                              ;L1594
 39031|  %383 = load i64, ptr %382, , !!8                                                                                      ;L1594
 39032|  %384 = load ptr, ptr %372, , !!8                                                                                      ;L1594
 39033|  %385 = invoke ptr %384(ptr %177, i64 %383)
 39034|  to label %3400 unwind label %365                                                                                      ;L1594
 39035| 
 39036| 386: ; preds = %379
 39038|     ;; self = ptr %39
 39039|     ;; self = ptr %39
 39040|     ;; self = ptr %39
 39041|  %387 = load ptr, ptr %39, , !!8, !!8                                                                                  ;L138<2073<2136<1686
 39042|     ;; p = ptr %387
 39043|  %388 = load i64, ptr %225, , !!8                                                                                      ;L2075<2136<1686
 39044|     ;; len = i64 %388
 39045|     ;; count = i64 %388
 39046|     ;; self[0..+8] = ptr %387
 39047|     ;; slice[0..+8] = ptr %387
 39048|     ;; self[8..+8] = i64 %388
 39049|     ;; slice[8..+8] = i64 %388
 39050|     ;; ptr = ptr %387
 39051|     ;; self = ptr %387
 39052|  %389 = getelementptr { { i64, [2 x i64] }, i64, ptr }, ptr %387, i64 %388                                             ;L961<100<1042<2136<1686
 39053|     ;; iter[0..+8] = ptr %387
 39054|     ;; iter[8..+8] = ptr %389
 39055|  br label %390                                                                                                         ;L1686
 39056| 
 39057| 390: ; preds = %1144, %386
 39058|  %391 = phi ptr [ %387, %386 ], [ %398, %1144 ]                                                                        ;L1686
 39059|     ;; iter[0..+8] = ptr %391
 39060|     ;; self = ptr undef
 39061|     ;; ptr = ptr %391
 39062|     ;; self = ptr %391
 39063|     ;; end_or_len = ptr %389
 39066|  %392 = icmp eq ptr %391, %389                                                                                         ;L1714<180<1686
 39067|  br i1 %392, label %393, label %397                                                                                    ;L180<1686
 39068| 
 39069| 393: ; preds = %390
 39070|     ;; self = ptr %41
 39071|     ;; self = ptr %41
 39072|     ;; self = ptr %41
 39073|  %394 = load ptr, ptr %41, , !!8, !!8                                                                                  ;L138<2073<2136<1883
 39074|     ;; p = ptr %394
 39075|  %395 = load i64, ptr %197, , !!8                                                                                      ;L2075<2136<1883
 39076|     ;; len = i64 %395
 39077|     ;; count = i64 %395
 39078|     ;; self[0..+8] = ptr %394
 39079|     ;; slice[0..+8] = ptr %394
 39080|     ;; self[8..+8] = i64 %395
 39081|     ;; slice[8..+8] = i64 %395
 39082|     ;; ptr = ptr %394
 39083|     ;; self = ptr %394
 39084|  %396 = getelementptr { { i64, [2 x i64] }, i64, ptr }, ptr %394, i64 %395                                             ;L961<100<1042<2136<1883
 39085|     ;; iter[0..+8] = ptr %394
 39086|     ;; iter[8..+8] = ptr %396
 39087|  br label %1192                                                                                                        ;L1883
 39088| 
 39089| 397: ; preds = %390
 39090|  %398 = gep %391, i64 40                                                                                               ;L656<185<1686
 39091|     ;; iter[0..+8] = ptr %398
 39092|  %399 = load i64, ptr %391,                                                                                            ;L1686
 39093|     ;; a[0..+8] = i64 %399
 39094|  %400 = gep %391, i64 8                                                                                                ;L1686
 39095|  %401 = load i64, ptr %400,                                                                                            ;L1686
 39096|     ;; a[8..+8] = i64 %401
 39098|  %402 = gep %391, i64 32                                                                                               ;L1686
 39099|  %403 = load ptr, ptr %402, , !!8, !!8                                                                                 ;L1686
 39100|     ;; e = ptr %403
 39101|     ;; self = ptr %403
 39102|     ;; self = ptr %403
 39103|     ;; self = ptr %403
 39104|     ;; self = ptr %403
 39105|     ;; self = ptr %403
 39106|     ;; self = ptr %403
 39107|     ;; self = ptr %403
 39108|     ;; self = ptr %403
 39109|     ;; self = ptr %403
 39110|     ;; self = ptr %403
 39111|     ;; caster = ptr %403
 39112|     ;; self = ptr %403
 39113|     ;; self = ptr %403
 39114|     ;; self = ptr %403
 39115|     ;; self = ptr %403
 39116|     ;; self = ptr %403
 39117|     ;; self = ptr %403
 39118|     ;; self = ptr %403
 39119|     ;; caster = ptr %403
 39120|     ;; self = ptr %403
 39121|     ;; caster = ptr %403
 39122|     ;; self = ptr %403
 39123|     ;; caster = ptr %403
 39124|     ;; self = ptr %403
 39125|     ;; self = ptr %403
 39126|     ;; self = ptr %403
 39127|     ;; caster = ptr %403
 39128|     ;; self = ptr %403
 39129|     ;; self = ptr %403
 39130|     ;; self = ptr %403
 39131|     ;; caster = ptr %403
 39132|     ;; self = ptr %403
 39135|     ;; __self_discr = i64 %399
 39136|     ;; __arg1_discr = i64 0
 39137|  %404 = icmp eq i64 %399, 0                                                                                            ;L81<1687
 39138|  br i1 %404, label %1144, label %405                                                                                   ;L1687
 39139| 
 39140| 405: ; preds = %397
 39141|  %406 = invoke i64 @gc::simulation6entityNtB5_6Entity18remain_action_time(ptr %403)
 39142|  to label %407 unwind label %365                                                                                       ;L1692
 39143| 
 39144| 407: ; preds = %405
 39145|     ;; self = ptr %403
 39146|     ;; self = ptr %403
 39147|     ;; self = ptr %403
 39148|  %408 = gep %403, i64 712                                                                                              ;L614<609<296<1968<1864<3787<1692
 39149|  %409 = load ptr, ptr %408, , !!8, !!8                                                                                 ;L614<609<296<1968<1864<3787<1692
 39150|  %410 = gep %403, i64 720                                                                                              ;L1864<3787<1692
 39151|  %411 = load i64, ptr %410, , !!8                                                                                      ;L1864<3787<1692
 39152|     ;; len = i64 %411
 39153|     ;; count = i64 %411
 39154|     ;; self[0..+8] = ptr %409
 39155|     ;; slice[0..+8] = ptr %409
 39156|     ;; self[8..+8] = i64 %411
 39157|     ;; slice[8..+8] = i64 %411
 39158|     ;; ptr = ptr %409
 39159|     ;; self = ptr %409
 39160|  %412 = gepS %409, i64 %411                                                                                            ;L961<100<1042<1692
 39161|     ;; self[0..+8] = ptr %409
 39162|     ;; self[0..+8] = ptr %409
 39163|     ;; self[8..+8] = ptr %412
 39164|     ;; self[8..+8] = ptr %412
 39165|     ;; self[0..+8] = ptr %409
 39166|     ;; self[8..+8] = ptr %412
 39168|     ;; self = ptr undef
 39169|     ;; self = ptr undef
 39170|     ;; predicate = ptr undef
 39171|     ;; self = ptr undef
 39172|     ;; self = ptr undef
 39173|     ;; count = i64 1
 39174|  br label %413                                                                                                         ;L348<98<107<2706<3354<3255<1693
 39175| 
 39176| 413: ; preds = %416, %407
 39177|  %414 = phi ptr [ %417, %416 ], [ %409, %407 ]
 39179|     ;; ptr = ptr %414
 39180|     ;; self = ptr %414
 39181|     ;; end_or_len = ptr %412
 39184|  %415 = icmp eq ptr %414, %412                                                                                         ;L1714<180<348<98<107<2706<3354<3255<1693
 39185|  br i1 %415, label %427, label %416                                                                                    ;L180<348<98<107<2706<3354<3255<1693
 39186| 
 39187| 416: ; preds = %413
 39188|  %417 = gep %414, i64 40                                                                                               ;L656<185<348<98<107<2706<3354<3255<1693
 39189|     ;; self[0..+8] = ptr %417
 39190|     ;; x = ptr %414
 39195|     ;; self = ptr %414
 39196|  %418 = load i32, ptr %414, , !!54081, !!8                                                                             ;L521<1692<298<349<98<107<2706<3354<3255<1693
 39197|  %419 = add nsw i32 %418, -6                                                                                           ;L521<1692<298<349<98<107<2706<3354<3255<1693
 39198|  %420 = icmp ult i32 %419, -4                                                                                          ;L521<1692<298<349<98<107<2706<3354<3255<1693
 39199|  br i1 %420, label %421, label %413                                                                                    ;L349<98<107<2706<3354<3255<1693
 39200| 
 39201| 421: ; preds = %416
 39202|     ;; self = ptr %414
 39203|     ;; f = ptr undef
 39204|     ;; self = ptr undef
 39205|     ;; x = ptr %414
 39206|     ;; args = ptr %414
 39208|     ;; c = ptr %414
 39209|     ;; self = ptr %414
 39210|  %422 = icmp eq i32 %418, 10                                                                                           ;L546<1693<310<1162<107<2706<3354<3255<1693
 39211|  %423 = select i1 %422, i64 32, i64 8                                                                                  ;L0<1693<310<1162<107<2706<3354<3255<1693
 39212|  %424 = gep %414, i64 %423                                                                                             ;L0<1693<310<1162<107<2706<3354<3255<1693
 39213|  %425 = load i64, ptr %424, , !!54150, !!8                                                                             ;L0<1693<310<1162<107<2706<3354<3255<1693
 39214|     ;; self[0..+8] = ptr %417
 39215|     ;; first = i64 %425
 39216|  %426 = invoke i64 @core::iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtNtBc_5slice4iter4IterNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity7CCStateENCNvNtCshdEBA0ozCnw_7game_ai15score_parameter25calculate_score_parametersd_0ENCB2H_se_0ENtNtNtBa_6traits8iterator8Iterator4foldjNCINvNvB45_6max_by4foldjNvYjNtNtBc_3cmp3Ord3cmpE0EB2L_(ptr %417, ptr %412, i64 %425)
 39217|  to label %427 unwind label %365                                                                                       ;L2707<3354<3255<1693
 39218| 
 39219| 427: ; preds = %421, %413
 39220|  %428 = phi i64 [ %426, %421 ], [ undef, %413 ]
 39223|     ;; self = i64 %406
 39225|  %429 = call i64 @llvm.umax.i64(i64 %428, i64 %406)                                                                    ;L1039<1692
 39226|  %430 = select i1 %415, i64 %406, i64 %429                                                                             ;L1039<1693
 39227|     ;; act_tick = i64 %430
 39228|  switch i64 %399, label %431 [
 39229|  i64 6, label %438
 39230|  i64 7, label %443
 39231|  i64 8, label %448
 39232|  i64 9, label %457
 39233|  ]                                                                                                                     ;L1695
 39234| 
 39235| 431: ; preds = %726, %722, %671, %660, %656, %605, %594, %590, %539, %528, %524, %473, %427
 39236|  %432 = gep %403, i64 1600                                                                                             ;L1779
 39237|  %433 = load i64, ptr %432, , !!8                                                                                      ;L1779
 39238|     ;; e_speed = i64 %433
 39239|     ;; speed = i64 %433
 39240|     ;; speed = i64 %433
 39241|     ;; e_atk_eff = ptr %403
 39242|     ;; self = ptr %403
 39243|  %434 = gep %403, i64 1168                                                                                             ;L742<1781
 39244|  %435 = gep %403, i64 1216                                                                                             ;L742<1781
 39245|  %436 = load i32, ptr %435, , !!8                                                                                      ;L742<1781
 39246|  %437 = icmp eq i32 %436, -1                                                                                           ;L742<1781
 39247|  br i1 %437, label %747, label %730                                                                                    ;L742<1781
 39248| 
 39249| 438: ; preds = %427
 39250|     ;; target_id = i64 %401
 39251|     ;; self = ptr %403
 39252|  %439 = gep %403, i64 1168                                                                                             ;L742<1697
 39253|  %440 = gep %403, i64 1216                                                                                             ;L742<1697
 39254|  %441 = load i32, ptr %440, , !!8                                                                                      ;L742<1697
 39255|  %442 = icmp eq i32 %441, -1                                                                                           ;L742<1697
 39256|  br i1 %442, label %468, label %466                                                                                    ;L742<1697
 39257| 
 39258| 443: ; preds = %427
 39259|     ;; target_id = i64 %401
 39260|     ;; self = ptr %403
 39261|  %444 = gep %403, i64 1224                                                                                             ;L742<1716
 39262|  %445 = gep %403, i64 1272                                                                                             ;L742<1716
 39263|  %446 = load i32, ptr %445, , !!8                                                                                      ;L742<1716
 39264|  %447 = icmp eq i32 %446, -1                                                                                           ;L742<1716
 39265|  br i1 %447, label %534, label %532                                                                                    ;L742<1716
 39266| 
 39267| 448: ; preds = %427
 39268|     ;; target_id = i64 %401
 39269|  %449 = gep %403, i64 1480                                                                                             ;L1693<1735
 39270|  %450 = load i64, ptr %449, , !!8                                                                                      ;L1693<1735
 39271|  %451 = icmp ugt i64 %450, 2                                                                                           ;L1693<1735
 39272|  %452 = gep %403, i64 1280                                                                                             ;L1693<1735
 39273|  %453 = select i1 %451, ptr %452, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                        ;L1693<1735
 39274|     ;; self = ptr %453
 39275|  %454 = gep %453, i64 48                                                                                               ;L742<1735
 39276|  %455 = load i32, ptr %454, , !!8                                                                                      ;L742<1735
 39277|  %456 = icmp eq i32 %455, -1                                                                                           ;L742<1735
 39278|  br i1 %456, label %600, label %598                                                                                    ;L742<1735
 39279| 
 39280| 457: ; preds = %427
 39281|     ;; target_id = i64 %401
 39282|  %458 = gep %403, i64 1480                                                                                             ;L1701<1756
 39283|  %459 = load i64, ptr %458, , !!8                                                                                      ;L1701<1756
 39284|  %460 = icmp ugt i64 %459, 4                                                                                           ;L1701<1756
 39285|  %461 = gep %403, i64 1336                                                                                             ;L1701<1756
 39286|  %462 = select i1 %460, ptr %461, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                        ;L1701<1756
 39287|     ;; self = ptr %462
 39288|  %463 = gep %462, i64 48                                                                                               ;L742<1756
 39289|  %464 = load i32, ptr %463, , !!8                                                                                      ;L742<1756
 39290|  %465 = icmp eq i32 %464, -1                                                                                           ;L742<1756
 39291|  br i1 %465, label %666, label %664                                                                                    ;L742<1756
 39292| 
 39293| 466: ; preds = %438
 39294|     ;; self = ptr %439
 39295|     ;; atk = ptr %439
 39296|  %467 = icmp eq i64 %401, %157                                                                                         ;L1698
 39297|  br i1 %467, label %481, label %469                                                                                    ;L1698
 39298| 
 39299| 468: ; preds = %438
 39300|     ;; self = ptr null
 39301|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.125) #25
 39302|  to label %173 unwind label %365                                                                                       ;L1013<1697
 39303| 
 39304| 469: ; preds = %499, %496, %466
 39305|     ;; self = ptr %47
 39306|     ;; self = ptr %47
 39307|  %470 = load ptr, ptr %67, , !!8, !!8                                                                                  ;L138<2083<1706
 39308|     ;; ptr = ptr %470
 39309|  %471 = load i64, ptr %70, , !!8                                                                                       ;L2085<1706
 39310|     ;; len = i64 %471
 39311|     ;; count = i64 %471
 39312|     ;; self[0..+8] = ptr %470
 39313|     ;; slice[0..+8] = ptr %470
 39314|     ;; self[8..+8] = i64 %471
 39315|     ;; slice[8..+8] = i64 %471
 39316|     ;; ptr = ptr %470
 39317|     ;; self = ptr %470
 39318|  %472 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %470, i64 %471 ;L961<240<1062<1706
 39319|     ;; predicate = ptr undef
 39320|     ;; self = ptr undef
 39321|     ;; self = ptr undef
 39322|     ;; count = i64 1
 39323|  br label %473                                                                                                         ;L348<1706
 39324| 
 39325| 473: ; preds = %476, %469
 39326|  %474 = phi ptr [ %477, %476 ], [ %470, %469 ]
 39327|     ;; ptr = ptr %474
 39328|     ;; self = ptr %474
 39329|     ;; end_or_len = ptr %472
 39332|  %475 = icmp eq ptr %474, %472                                                                                         ;L1714<180<348<1706
 39333|  br i1 %475, label %431, label %476                                                                                    ;L180<348<1706
 39334| 
 39335| 476: ; preds = %473
 39336|  %477 = gep %474, i64 216                                                                                              ;L656<185<348<1706
 39337|     ;; x = ptr %474
 39340|  %478 = gep %474, i64 88                                                                                               ;L1706<349<1706
 39341|  %479 = load i64, ptr %478, , !!54667, !!8                                                                             ;L1706<349<1706
 39342|  %480 = icmp eq i64 %479, %401                                                                                         ;L1706<349<1706
 39343|  br i1 %480, label %502, label %473                                                                                    ;L349<1706
 39344| 
 39345| 481: ; preds = %466
 39346|     ;; self = ptr %439
 39347|  %482 = add nsw i32 %441, -1                                                                                           ;L149<1699
 39348|  %483 = icmp ult i32 %482, 2                                                                                           ;L149<1699
 39349|  br i1 %483, label %488, label %484                                                                                    ;L149<1699
 39350| 
 39351| 484: ; preds = %481
 39352|  %485 = gep %403, i64 104                                                                                              ;L1564<1699
 39353|  %486 = load i64, ptr %485, , !!8                                                                                      ;L1564<1699
 39354|  %487 = icmp eq i64 %486, 13                                                                                           ;L1564<1699
 39355|  br i1 %487, label %490, label %488                                                                                    ;L1564<1699
 39356| 
 39357| 488: ; preds = %490, %484, %481
 39358|  %489 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %439, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 39359|  to label %499 unwind label %365                                                                                       ;L1700
 39360| 
 39361| 490: ; preds = %484
 39362|     ;; champ = ptr %403
 39363|  %491 = gep %403, i64 112                                                                                              ;L1565<1699
 39364|  %492 = load i64, ptr %491, , !!8                                                                                      ;L1565<1699
 39365|  %493 = icmp eq i64 %492, 3                                                                                            ;L1565<1699
 39366|  br i1 %493, label %494, label %488                                                                                    ;L1699
 39367| 
 39368| 494: ; preds = %490
 39369|  %495 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %439, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 39370|  to label %496 unwind label %365                                                                                       ;L1702
 39371| 
 39372| 496: ; preds = %494
 39373|  %497 = load i64, ptr %87, , !!8                                                                                       ;L1702
 39374|  %498 = add i64 %497, %495                                                                                             ;L1702
 39375|  store i64 %498, ptr %87,                                                                                              ;L1702
 39376|  br label %469                                                                                                         ;L1699
 39377| 
 39378| 499: ; preds = %488
 39379|  %500 = load i64, ptr %89, , !!8                                                                                       ;L1700
 39380|  %501 = add i64 %500, %489                                                                                             ;L1700
 39381|  store i64 %501, ptr %89,                                                                                              ;L1700
 39382|  br label %469                                                                                                         ;L1699
 39383| 
 39384| 502: ; preds = %476
 39385|     ;; target = ptr %474
 39386|  %503 = load ptr, ptr %372, , !!8                                                                                      ;L1707
 39387|  %504 = invoke ptr %503(ptr %177, i64 %401)
 39388|  to label %505 unwind label %365                                                                                       ;L1707
 39389| 
 39390| 505: ; preds = %502
 39391|     ;; self = ptr %504
 39392|  %506 = icmp eq ptr %504, null                                                                                         ;L1011<1707
 39393|  br i1 %506, label %511, label %507                                                                                    ;L1011<1707
 39394| 
 39395| 507: ; preds = %505
 39396|     ;; target_entity = ptr %504
 39397|     ;; self = ptr %439
 39398|  %508 = load i32, ptr %440, , !!8                                                                                      ;L149<1708
 39399|  %509 = add nsw i32 %508, -1                                                                                           ;L149<1708
 39400|  %510 = icmp ult i32 %509, 2                                                                                           ;L149<1708
 39401|  br i1 %510, label %516, label %512                                                                                    ;L149<1708
 39402| 
 39403| 511: ; preds = %505
 39404|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.126) #25
 39405|  to label %173 unwind label %365                                                                                       ;L1013<1707
 39406| 
 39407| 512: ; preds = %507
 39408|  %513 = gep %403, i64 104                                                                                              ;L1564<1708
 39409|  %514 = load i64, ptr %513, , !!8                                                                                      ;L1564<1708
 39410|  %515 = icmp eq i64 %514, 13                                                                                           ;L1564<1708
 39411|  br i1 %515, label %518, label %516                                                                                    ;L1564<1708
 39412| 
 39413| 516: ; preds = %518, %512, %507
 39414|  %517 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %439, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %504)
 39415|  to label %528 unwind label %365                                                                                       ;L1709
 39416| 
 39417| 518: ; preds = %512
 39418|     ;; champ = ptr %403
 39419|  %519 = gep %403, i64 112                                                                                              ;L1565<1708
 39420|  %520 = load i64, ptr %519, , !!8                                                                                      ;L1565<1708
 39421|  %521 = icmp eq i64 %520, 3                                                                                            ;L1565<1708
 39422|  br i1 %521, label %522, label %516                                                                                    ;L1708
 39423| 
 39424| 522: ; preds = %518
 39425|  %523 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %439, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %504)
 39426|  to label %524 unwind label %365                                                                                       ;L1711
 39427| 
 39428| 524: ; preds = %522
 39429|  %525 = gep %474, i64 112                                                                                              ;L1711
 39430|  %526 = load i64, ptr %525, , !!8                                                                                      ;L1711
 39431|  %527 = add i64 %526, %523                                                                                             ;L1711
 39432|  store i64 %527, ptr %525,                                                                                             ;L1711
 39433|  br label %431                                                                                                         ;L1708
 39434| 
 39435| 528: ; preds = %516
 39436|  %529 = gep %474, i64 128                                                                                              ;L1709
 39437|  %530 = load i64, ptr %529, , !!8                                                                                      ;L1709
 39438|  %531 = add i64 %530, %517                                                                                             ;L1709
 39439|  store i64 %531, ptr %529,                                                                                             ;L1709
 39440|  br label %431                                                                                                         ;L1708
 39441| 
 39442| 532: ; preds = %443
 39443|     ;; self = ptr %444
 39444|     ;; skill = ptr %444
 39445|  %533 = icmp eq i64 %401, %157                                                                                         ;L1717
 39446|  br i1 %533, label %547, label %535                                                                                    ;L1717
 39447| 
 39448| 534: ; preds = %443
 39449|     ;; self = ptr null
 39450|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.127) #25
 39451|  to label %173 unwind label %365                                                                                       ;L1013<1716
 39452| 
 39453| 535: ; preds = %565, %562, %532
 39454|     ;; self = ptr %47
 39455|     ;; self = ptr %47
 39456|  %536 = load ptr, ptr %67, , !!8, !!8                                                                                  ;L138<2083<1725
 39457|     ;; ptr = ptr %536
 39458|  %537 = load i64, ptr %70, , !!8                                                                                       ;L2085<1725
 39459|     ;; len = i64 %537
 39460|     ;; count = i64 %537
 39461|     ;; self[0..+8] = ptr %536
 39462|     ;; slice[0..+8] = ptr %536
 39463|     ;; self[8..+8] = i64 %537
 39464|     ;; slice[8..+8] = i64 %537
 39465|     ;; ptr = ptr %536
 39466|     ;; self = ptr %536
 39467|  %538 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %536, i64 %537 ;L961<240<1062<1725
 39468|     ;; predicate = ptr undef
 39469|     ;; self = ptr undef
 39470|     ;; self = ptr undef
 39471|     ;; count = i64 1
 39472|  br label %539                                                                                                         ;L348<1725
 39473| 
 39474| 539: ; preds = %542, %535
 39475|  %540 = phi ptr [ %543, %542 ], [ %536, %535 ]
 39476|     ;; ptr = ptr %540
 39477|     ;; self = ptr %540
 39478|     ;; end_or_len = ptr %538
 39481|  %541 = icmp eq ptr %540, %538                                                                                         ;L1714<180<348<1725
 39482|  br i1 %541, label %431, label %542                                                                                    ;L180<348<1725
 39483| 
 39484| 542: ; preds = %539
 39485|  %543 = gep %540, i64 216                                                                                              ;L656<185<348<1725
 39486|     ;; x = ptr %540
 39489|  %544 = gep %540, i64 88                                                                                               ;L1725<349<1725
 39490|  %545 = load i64, ptr %544, , !!54748, !!8                                                                             ;L1725<349<1725
 39491|  %546 = icmp eq i64 %545, %401                                                                                         ;L1725<349<1725
 39492|  br i1 %546, label %568, label %539                                                                                    ;L349<1725
 39493| 
 39494| 547: ; preds = %532
 39495|     ;; self = ptr %444
 39496|  %548 = add nsw i32 %446, -1                                                                                           ;L149<1718
 39497|  %549 = icmp ult i32 %548, 2                                                                                           ;L149<1718
 39498|  br i1 %549, label %554, label %550                                                                                    ;L149<1718
 39499| 
 39500| 550: ; preds = %547
 39501|  %551 = gep %403, i64 104                                                                                              ;L1571<1718
 39502|  %552 = load i64, ptr %551, , !!8                                                                                      ;L1571<1718
 39503|  %553 = icmp eq i64 %552, 13                                                                                           ;L1571<1718
 39504|  br i1 %553, label %556, label %554                                                                                    ;L1571<1718
 39505| 
 39506| 554: ; preds = %556, %550, %547
 39507|  %555 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %444, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 39508|  to label %565 unwind label %365                                                                                       ;L1719
 39509| 
 39510| 556: ; preds = %550
 39511|     ;; champ = ptr %403
 39512|  %557 = gep %403, i64 112                                                                                              ;L1572<1718
 39513|  %558 = load i64, ptr %557, , !!8                                                                                      ;L1572<1718
 39514|  %559 = icmp eq i64 %558, 4                                                                                            ;L1572<1718
 39515|  br i1 %559, label %560, label %554                                                                                    ;L1718
 39516| 
 39517| 560: ; preds = %556
 39518|  %561 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %444, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 39519|  to label %562 unwind label %365                                                                                       ;L1721
 39520| 
 39521| 562: ; preds = %560
 39522|  %563 = load i64, ptr %87, , !!8                                                                                       ;L1721
 39523|  %564 = add i64 %563, %561                                                                                             ;L1721
 39524|  store i64 %564, ptr %87,                                                                                              ;L1721
 39525|  br label %535                                                                                                         ;L1718
 39526| 
 39527| 565: ; preds = %554
 39528|  %566 = load i64, ptr %89, , !!8                                                                                       ;L1719
 39529|  %567 = add i64 %566, %555                                                                                             ;L1719
 39530|  store i64 %567, ptr %89,                                                                                              ;L1719
 39531|  br label %535                                                                                                         ;L1718
 39532| 
 39533| 568: ; preds = %542
 39534|     ;; target = ptr %540
 39535|  %569 = load ptr, ptr %372, , !!8                                                                                      ;L1726
 39536|  %570 = invoke ptr %569(ptr %177, i64 %401)
 39537|  to label %571 unwind label %365                                                                                       ;L1726
 39538| 
 39539| 571: ; preds = %568
 39540|     ;; self = ptr %570
 39541|  %572 = icmp eq ptr %570, null                                                                                         ;L1011<1726
 39542|  br i1 %572, label %577, label %573                                                                                    ;L1011<1726
 39543| 
 39544| 573: ; preds = %571
 39545|     ;; target_entity = ptr %570
 39546|     ;; self = ptr %444
 39547|  %574 = load i32, ptr %445, , !!8                                                                                      ;L149<1727
 39548|  %575 = add nsw i32 %574, -1                                                                                           ;L149<1727
 39549|  %576 = icmp ult i32 %575, 2                                                                                           ;L149<1727
 39550|  br i1 %576, label %582, label %578                                                                                    ;L149<1727
 39551| 
 39552| 577: ; preds = %571
 39553|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.128) #25
 39554|  to label %173 unwind label %365                                                                                       ;L1013<1726
 39555| 
 39556| 578: ; preds = %573
 39557|  %579 = gep %403, i64 104                                                                                              ;L1571<1727
 39558|  %580 = load i64, ptr %579, , !!8                                                                                      ;L1571<1727
 39559|  %581 = icmp eq i64 %580, 13                                                                                           ;L1571<1727
 39560|  br i1 %581, label %584, label %582                                                                                    ;L1571<1727
 39561| 
 39562| 582: ; preds = %584, %578, %573
 39563|  %583 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %444, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %570)
 39564|  to label %594 unwind label %365                                                                                       ;L1728
 39565| 
 39566| 584: ; preds = %578
 39567|     ;; champ = ptr %403
 39568|  %585 = gep %403, i64 112                                                                                              ;L1572<1727
 39569|  %586 = load i64, ptr %585, , !!8                                                                                      ;L1572<1727
 39570|  %587 = icmp eq i64 %586, 4                                                                                            ;L1572<1727
 39571|  br i1 %587, label %588, label %582                                                                                    ;L1727
 39572| 
 39573| 588: ; preds = %584
 39574|  %589 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %444, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %570)
 39575|  to label %590 unwind label %365                                                                                       ;L1730
 39576| 
 39577| 590: ; preds = %588
 39578|  %591 = gep %540, i64 112                                                                                              ;L1730
 39579|  %592 = load i64, ptr %591, , !!8                                                                                      ;L1730
 39580|  %593 = add i64 %592, %589                                                                                             ;L1730
 39581|  store i64 %593, ptr %591,                                                                                             ;L1730
 39582|  br label %431                                                                                                         ;L1727
 39583| 
 39584| 594: ; preds = %582
 39585|  %595 = gep %540, i64 128                                                                                              ;L1728
 39586|  %596 = load i64, ptr %595, , !!8                                                                                      ;L1728
 39587|  %597 = add i64 %596, %583                                                                                             ;L1728
 39588|  store i64 %597, ptr %595,                                                                                             ;L1728
 39589|  br label %431                                                                                                         ;L1727
 39590| 
 39591| 598: ; preds = %448
 39592|     ;; self = ptr %453
 39593|     ;; skill2 = ptr %453
 39594|  %599 = icmp eq i64 %401, %157                                                                                         ;L1736
 39595|  br i1 %599, label %613, label %601                                                                                    ;L1736
 39596| 
 39597| 600: ; preds = %448
 39598|     ;; self = ptr null
 39599|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.129) #25
 39600|  to label %173 unwind label %365                                                                                       ;L1013<1735
 39601| 
 39602| 601: ; preds = %631, %628, %598
 39603|     ;; self = ptr %47
 39604|     ;; self = ptr %47
 39605|  %602 = load ptr, ptr %67, , !!8, !!8                                                                                  ;L138<2083<1744
 39606|     ;; ptr = ptr %602
 39607|  %603 = load i64, ptr %70, , !!8                                                                                       ;L2085<1744
 39608|     ;; len = i64 %603
 39609|     ;; count = i64 %603
 39610|     ;; self[0..+8] = ptr %602
 39611|     ;; slice[0..+8] = ptr %602
 39612|     ;; self[8..+8] = i64 %603
 39613|     ;; slice[8..+8] = i64 %603
 39614|     ;; ptr = ptr %602
 39615|     ;; self = ptr %602
 39616|  %604 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %602, i64 %603 ;L961<240<1062<1744
 39617|     ;; predicate = ptr undef
 39618|     ;; self = ptr undef
 39619|     ;; self = ptr undef
 39620|     ;; count = i64 1
 39621|  br label %605                                                                                                         ;L348<1744
 39622| 
 39623| 605: ; preds = %608, %601
 39624|  %606 = phi ptr [ %609, %608 ], [ %602, %601 ]
 39625|     ;; ptr = ptr %606
 39626|     ;; self = ptr %606
 39627|     ;; end_or_len = ptr %604
 39630|  %607 = icmp eq ptr %606, %604                                                                                         ;L1714<180<348<1744
 39631|  br i1 %607, label %431, label %608                                                                                    ;L180<348<1744
 39632| 
 39633| 608: ; preds = %605
 39634|  %609 = gep %606, i64 216                                                                                              ;L656<185<348<1744
 39635|     ;; x = ptr %606
 39638|  %610 = gep %606, i64 88                                                                                               ;L1744<349<1744
 39639|  %611 = load i64, ptr %610, , !!54822, !!8                                                                             ;L1744<349<1744
 39640|  %612 = icmp eq i64 %611, %401                                                                                         ;L1744<349<1744
 39641|  br i1 %612, label %634, label %605                                                                                    ;L349<1744
 39642| 
 39643| 613: ; preds = %598
 39644|     ;; self = ptr %453
 39645|  %614 = add nsw i32 %455, -1                                                                                           ;L149<1737
 39646|  %615 = icmp ult i32 %614, 2                                                                                           ;L149<1737
 39647|  br i1 %615, label %620, label %616                                                                                    ;L149<1737
 39648| 
 39649| 616: ; preds = %613
 39650|  %617 = gep %403, i64 104                                                                                              ;L1579<1737
 39651|  %618 = load i64, ptr %617, , !!8                                                                                      ;L1579<1737
 39652|  %619 = icmp eq i64 %618, 13                                                                                           ;L1579<1737
 39653|  br i1 %619, label %622, label %620                                                                                    ;L1579<1737
 39654| 
 39655| 620: ; preds = %622, %616, %613
 39656|  %621 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %453, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 39657|  to label %631 unwind label %365                                                                                       ;L1738
 39658| 
 39659| 622: ; preds = %616
 39660|     ;; champ = ptr %403
 39661|  %623 = gep %403, i64 112                                                                                              ;L1580<1737
 39662|  %624 = load i64, ptr %623, , !!8                                                                                      ;L1580<1737
 39663|  %625 = icmp eq i64 %624, 5                                                                                            ;L1580<1737
 39664|  br i1 %625, label %626, label %620                                                                                    ;L1737
 39665| 
 39666| 626: ; preds = %622
 39667|  %627 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %453, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 39668|  to label %628 unwind label %365                                                                                       ;L1740
 39669| 
 39670| 628: ; preds = %626
 39671|  %629 = load i64, ptr %87, , !!8                                                                                       ;L1740
 39672|  %630 = add i64 %629, %627                                                                                             ;L1740
 39673|  store i64 %630, ptr %87,                                                                                              ;L1740
 39674|  br label %601                                                                                                         ;L1737
 39675| 
 39676| 631: ; preds = %620
 39677|  %632 = load i64, ptr %89, , !!8                                                                                       ;L1738
 39678|  %633 = add i64 %632, %621                                                                                             ;L1738
 39679|  store i64 %633, ptr %89,                                                                                              ;L1738
 39680|  br label %601                                                                                                         ;L1737
 39681| 
 39682| 634: ; preds = %608
 39683|     ;; target = ptr %606
 39684|  %635 = load ptr, ptr %372, , !!8                                                                                      ;L1745
 39685|  %636 = invoke ptr %635(ptr %177, i64 %401)
 39686|  to label %637 unwind label %365                                                                                       ;L1745
 39687| 
 39688| 637: ; preds = %634
 39689|     ;; self = ptr %636
 39690|  %638 = icmp eq ptr %636, null                                                                                         ;L1011<1745
 39691|  br i1 %638, label %643, label %639                                                                                    ;L1011<1745
 39692| 
 39693| 639: ; preds = %637
 39694|     ;; target_entity = ptr %636
 39695|     ;; self = ptr %453
 39696|  %640 = load i32, ptr %454, , !!8                                                                                      ;L149<1746
 39697|  %641 = add nsw i32 %640, -1                                                                                           ;L149<1746
 39698|  %642 = icmp ult i32 %641, 2                                                                                           ;L149<1746
 39699|  br i1 %642, label %648, label %644                                                                                    ;L149<1746
 39700| 
 39701| 643: ; preds = %637
 39702|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.130) #25
 39703|  to label %173 unwind label %365                                                                                       ;L1013<1745
 39704| 
 39705| 644: ; preds = %639
 39706|  %645 = gep %403, i64 104                                                                                              ;L1579<1746
 39707|  %646 = load i64, ptr %645, , !!8                                                                                      ;L1579<1746
 39708|  %647 = icmp eq i64 %646, 13                                                                                           ;L1579<1746
 39709|  br i1 %647, label %650, label %648                                                                                    ;L1579<1746
 39710| 
 39711| 648: ; preds = %650, %644, %639
 39712|  %649 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %453, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %636)
 39713|  to label %660 unwind label %365                                                                                       ;L1747
 39714| 
 39715| 650: ; preds = %644
 39716|     ;; champ = ptr %403
 39717|  %651 = gep %403, i64 112                                                                                              ;L1580<1746
 39718|  %652 = load i64, ptr %651, , !!8                                                                                      ;L1580<1746
 39719|  %653 = icmp eq i64 %652, 5                                                                                            ;L1580<1746
 39720|  br i1 %653, label %654, label %648                                                                                    ;L1746
 39721| 
 39722| 654: ; preds = %650
 39723|  %655 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %453, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %636)
 39724|  to label %656 unwind label %365                                                                                       ;L1749
 39725| 
 39726| 656: ; preds = %654
 39727|  %657 = gep %606, i64 112                                                                                              ;L1749
 39728|  %658 = load i64, ptr %657, , !!8                                                                                      ;L1749
 39729|  %659 = add i64 %658, %655                                                                                             ;L1749
 39730|  store i64 %659, ptr %657,                                                                                             ;L1749
 39731|  br label %431                                                                                                         ;L1746
 39732| 
 39733| 660: ; preds = %648
 39734|  %661 = gep %606, i64 128                                                                                              ;L1747
 39735|  %662 = load i64, ptr %661, , !!8                                                                                      ;L1747
 39736|  %663 = add i64 %662, %649                                                                                             ;L1747
 39737|  store i64 %663, ptr %661,                                                                                             ;L1747
 39738|  br label %431                                                                                                         ;L1746
 39739| 
 39740| 664: ; preds = %457
 39741|     ;; self = ptr %462
 39742|     ;; ult = ptr %462
 39743|  %665 = icmp eq i64 %401, %157                                                                                         ;L1757
 39744|  br i1 %665, label %679, label %667                                                                                    ;L1757
 39745| 
 39746| 666: ; preds = %457
 39747|     ;; self = ptr null
 39748|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.131) #25
 39749|  to label %173 unwind label %365                                                                                       ;L1013<1756
 39750| 
 39751| 667: ; preds = %697, %694, %664
 39752|     ;; self = ptr %47
 39753|     ;; self = ptr %47
 39754|  %668 = load ptr, ptr %67, , !!8, !!8                                                                                  ;L138<2083<1765
 39755|     ;; ptr = ptr %668
 39756|  %669 = load i64, ptr %70, , !!8                                                                                       ;L2085<1765
 39757|     ;; len = i64 %669
 39758|     ;; count = i64 %669
 39759|     ;; self[0..+8] = ptr %668
 39760|     ;; slice[0..+8] = ptr %668
 39761|     ;; self[8..+8] = i64 %669
 39762|     ;; slice[8..+8] = i64 %669
 39763|     ;; ptr = ptr %668
 39764|     ;; self = ptr %668
 39765|  %670 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %668, i64 %669 ;L961<240<1062<1765
 39766|     ;; predicate = ptr undef
 39767|     ;; self = ptr undef
 39768|     ;; self = ptr undef
 39769|     ;; count = i64 1
 39770|  br label %671                                                                                                         ;L348<1765
 39771| 
 39772| 671: ; preds = %674, %667
 39773|  %672 = phi ptr [ %675, %674 ], [ %668, %667 ]
 39774|     ;; ptr = ptr %672
 39775|     ;; self = ptr %672
 39776|     ;; end_or_len = ptr %670
 39779|  %673 = icmp eq ptr %672, %670                                                                                         ;L1714<180<348<1765
 39780|  br i1 %673, label %431, label %674                                                                                    ;L180<348<1765
 39781| 
 39782| 674: ; preds = %671
 39783|  %675 = gep %672, i64 216                                                                                              ;L656<185<348<1765
 39784|     ;; x = ptr %672
 39787|  %676 = gep %672, i64 88                                                                                               ;L1765<349<1765
 39788|  %677 = load i64, ptr %676, , !!54896, !!8                                                                             ;L1765<349<1765
 39789|  %678 = icmp eq i64 %677, %401                                                                                         ;L1765<349<1765
 39790|  br i1 %678, label %700, label %671                                                                                    ;L349<1765
 39791| 
 39792| 679: ; preds = %664
 39793|     ;; self = ptr %462
 39794|  %680 = add nsw i32 %464, -1                                                                                           ;L149<1758
 39795|  %681 = icmp ult i32 %680, 2                                                                                           ;L149<1758
 39796|  br i1 %681, label %686, label %682                                                                                    ;L149<1758
 39797| 
 39798| 682: ; preds = %679
 39799|  %683 = gep %403, i64 104                                                                                              ;L1586<1758
 39800|  %684 = load i64, ptr %683, , !!8                                                                                      ;L1586<1758
 39801|  %685 = icmp eq i64 %684, 13                                                                                           ;L1586<1758
 39802|  br i1 %685, label %688, label %686                                                                                    ;L1586<1758
 39803| 
 39804| 686: ; preds = %688, %682, %679
 39805|  %687 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %462, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 39806|  to label %697 unwind label %365                                                                                       ;L1759
 39807| 
 39808| 688: ; preds = %682
 39809|     ;; champ = ptr %403
 39810|  %689 = gep %403, i64 112                                                                                              ;L1587<1758
 39811|  %690 = load i64, ptr %689, , !!8                                                                                      ;L1587<1758
 39812|  %691 = icmp eq i64 %690, 6                                                                                            ;L1587<1758
 39813|  br i1 %691, label %692, label %686                                                                                    ;L1758
 39814| 
 39815| 692: ; preds = %688
 39816|  %693 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %462, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 39817|  to label %694 unwind label %365                                                                                       ;L1761
 39818| 
 39819| 694: ; preds = %692
 39820|  %695 = load i64, ptr %87, , !!8                                                                                       ;L1761
 39821|  %696 = add i64 %695, %693                                                                                             ;L1761
 39822|  store i64 %696, ptr %87,                                                                                              ;L1761
 39823|  br label %667                                                                                                         ;L1758
 39824| 
 39825| 697: ; preds = %686
 39826|  %698 = load i64, ptr %89, , !!8                                                                                       ;L1759
 39827|  %699 = add i64 %698, %687                                                                                             ;L1759
 39828|  store i64 %699, ptr %89,                                                                                              ;L1759
 39829|  br label %667                                                                                                         ;L1758
 39830| 
 39831| 700: ; preds = %674
 39832|     ;; target = ptr %672
 39833|  %701 = load ptr, ptr %372, , !!8                                                                                      ;L1766
 39834|  %702 = invoke ptr %701(ptr %177, i64 %401)
 39835|  to label %703 unwind label %365                                                                                       ;L1766
 39836| 
 39837| 703: ; preds = %700
 39838|     ;; self = ptr %702
 39839|  %704 = icmp eq ptr %702, null                                                                                         ;L1011<1766
 39840|  br i1 %704, label %709, label %705                                                                                    ;L1011<1766
 39841| 
 39842| 705: ; preds = %703
 39843|     ;; target_entity = ptr %702
 39844|     ;; self = ptr %462
 39845|  %706 = load i32, ptr %463, , !!8                                                                                      ;L149<1767
 39846|  %707 = add nsw i32 %706, -1                                                                                           ;L149<1767
 39847|  %708 = icmp ult i32 %707, 2                                                                                           ;L149<1767
 39848|  br i1 %708, label %714, label %710                                                                                    ;L149<1767
 39849| 
 39850| 709: ; preds = %703
 39851|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.132) #25
 39852|  to label %173 unwind label %365                                                                                       ;L1013<1766
 39853| 
 39854| 710: ; preds = %705
 39855|  %711 = gep %403, i64 104                                                                                              ;L1586<1767
 39856|  %712 = load i64, ptr %711, , !!8                                                                                      ;L1586<1767
 39857|  %713 = icmp eq i64 %712, 13                                                                                           ;L1586<1767
 39858|  br i1 %713, label %716, label %714                                                                                    ;L1586<1767
 39859| 
 39860| 714: ; preds = %716, %710, %705
 39861|  %715 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %462, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %702)
 39862|  to label %726 unwind label %365                                                                                       ;L1768
 39863| 
 39864| 716: ; preds = %710
 39865|     ;; champ = ptr %403
 39866|  %717 = gep %403, i64 112                                                                                              ;L1587<1767
 39867|  %718 = load i64, ptr %717, , !!8                                                                                      ;L1587<1767
 39868|  %719 = icmp eq i64 %718, 6                                                                                            ;L1587<1767
 39869|  br i1 %719, label %720, label %714                                                                                    ;L1767
 39870| 
 39871| 720: ; preds = %716
 39872|  %721 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %462, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %702)
 39873|  to label %722 unwind label %365                                                                                       ;L1770
 39874| 
 39875| 722: ; preds = %720
 39876|  %723 = gep %672, i64 112                                                                                              ;L1770
 39877|  %724 = load i64, ptr %723, , !!8                                                                                      ;L1770
 39878|  %725 = add i64 %724, %721                                                                                             ;L1770
 39879|  store i64 %725, ptr %723,                                                                                             ;L1770
 39880|  br label %431                                                                                                         ;L1767
 39881| 
 39882| 726: ; preds = %714
 39883|  %727 = gep %672, i64 128                                                                                              ;L1768
 39884|  %728 = load i64, ptr %727, , !!8                                                                                      ;L1768
 39885|  %729 = add i64 %728, %715                                                                                             ;L1768
 39886|  store i64 %729, ptr %727,                                                                                             ;L1768
 39887|  br label %431                                                                                                         ;L1767
 39888| 
 39889| 730: ; preds = %431
 39890|     ;; self = ptr %434
 39891|     ;; e_atk = ptr %434
 39892|     ;; atk = ptr %434
 39893|     ;; self = ptr %434
 39894|  %731 = gep %403, i64 1184                                                                                             ;L26<1782
 39895|  %732 = load i64, ptr %731, , !!8                                                                                      ;L26<1782
 39896|  %733 = gep %403, i64 1192                                                                                             ;L26<1782
 39897|  %734 = load i64, ptr %733, , !!8                                                                                      ;L26<1782
 39898|  %735 = gep %403, i64 1480                                                                                             ;L26<1782
 39899|  %736 = load i64, ptr %735, , !!8                                                                                      ;L26<1782
 39900|  %737 = add i64 %736, -1                                                                                               ;L26<1782
 39901|  %738 = mul i64 %737, %734                                                                                             ;L26<1782
 39902|  %739 = gep %403, i64 1080                                                                                             ;L26<1782
 39903|  %740 = load i64, ptr %739, , !!8                                                                                      ;L26<1782
 39904|  %741 = gep %403, i64 1136                                                                                             ;L1511<1782
 39905|  %742 = load i32, ptr %741, , !!8                                                                                      ;L1511<1782
 39906|  %743 = sext i32 %742 to i64                                                                                           ;L1511<1782
 39907|     ;; mult = i64 %743
 39908|     ;; mult = i64 %743
 39909|     ;; mult = i64 %743
 39910|     ;; mult = i64 %743
 39911|     ;; mult = i64 %743
 39912|     ;; mult = i64 %743
 39913|  %744 = icmp eq i32 %742, 0                                                                                            ;L1512<1782
 39914|  %745 = gep %403, i64 1664                                                                                             ;L0<1782
 39915|  %746 = load i64, ptr %745, , !!8                                                                                      ;L0<1782
 39916|  br i1 %744, label %752, label %748                                                                                    ;L1512<1782
 39917| 
 39918| 747: ; preds = %431
 39919|     ;; self = ptr null
 39920|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.133) #25
 39921|  to label %173 unwind label %365                                                                                       ;L1013<1781
 39922| 
 39923| 748: ; preds = %730
 39924|  %749 = add nsw i64 %743, 100                                                                                          ;L1515<1782
 39925|  %750 = mul i64 %746, %749                                                                                             ;L1515<1782
 39926|  %751 = udiv i64 %750, 100                                                                                             ;L1515<1782
 39927|  br label %752                                                                                                         ;L1512<1782
 39928| 
 39929| 752: ; preds = %748, %730
 39930|  %753 = phi i64 [ %751, %748 ], [ %746, %730 ]                                                                         ;L0<1782
 39932|  %754 = gep %403, i64 104                                                                                              ;L1748<1783
 39933|  %755 = load i64, ptr %754, , !!8                                                                                      ;L1748<1783
 39934|  switch i64 %755, label %756 [
 39935|  i64 0, label %770
 39936|  i64 1, label %765
 39937|  i64 2, label %757
 39938|  i64 3, label %770
 39939|  i64 4, label %758
 39940|  i64 5, label %759
 39941|  i64 6, label %759
 39942|  i64 7, label %758
 39943|  i64 8, label %760
 39944|  i64 9, label %761
 39945|  i64 10, label %762
 39946|  i64 11, label %763
 39947|  i64 12, label %764
 39948|  i64 13, label %772
 39949|  ]                                                                                                                     ;L1748<1783
 39950| 
 39951| 756: ; preds = %2031, %1706, %1459, %1031, %752
 39952|  unreachable
 39953| 
 39954| 757: ; preds = %752
 39955|     ;; info = ptr %403
 39956|  br label %765                                                                                                         ;L1758<1783
 39957| 
 39958| 758: ; preds = %752, %752
 39959|     ;; info = ptr %403
 39960|  br label %765                                                                                                         ;L1750<1783
 39961| 
 39962| 759: ; preds = %752, %752
 39963|     ;; info = ptr %403
 39964|  br label %765                                                                                                         ;L1760<1783
 39965| 
 39966| 760: ; preds = %752
 39967|     ;; info = ptr %403
 39968|  br label %765                                                                                                         ;L1753<1783
 39969| 
 39970| 761: ; preds = %752
 39971|     ;; info = ptr %403
 39972|  br label %765                                                                                                         ;L1754<1783
 39973| 
 39974| 762: ; preds = %752
 39975|     ;; info = ptr %403
 39976|  br label %765                                                                                                         ;L1755<1783
 39977| 
 39978| 763: ; preds = %752
 39979|     ;; info = ptr %403
 39980|  br label %765                                                                                                         ;L1756<1783
 39981| 
 39982| 764: ; preds = %752
 39983|     ;; info = ptr %403
 39984|  br label %765                                                                                                         ;L1757<1783
 39985| 
 39986| 765: ; preds = %764, %763, %762, %761, %760, %759, %758, %757, %752
 39987|  %766 = phi i64 [ 208, %764 ], [ 272, %757 ], [ 232, %758 ], [ 496, %759 ], [ 184, %752 ], [ 216, %763 ], [ 176, %760 ], [ 200, %761 ], [ 240, %762 ]
 39988|  %767 = gep %403, i64 %766                                                                                             ;L0<1783
 39989|  %768 = load i64, ptr %767, , !!8                                                                                      ;L0<1783
 39990|  %769 = call i64 @llvm.umax.i64(i64 %768, i64 %430)                                                                    ;L1039<1783
 39991|  br label %770                                                                                                         ;L1039<1783
 39992| 
 39993| 770: ; preds = %765, %752, %752
 39994|  %771 = phi i64 [ %430, %752 ], [ %430, %752 ], [ %769, %765 ]
 39995|     ;; self = i64 %430
 39996|     ;; other = i64 %771
 39997|     ;; e_attack_tick = i64 %771
 39998|     ;; attack_tick = i64 %771
 39999|     ;; self = i64 %430
 40000|     ;; other = i64 0
 40001|  br label %782                                                                                                         ;L1039<1784
 40002| 
 40003| 772: ; preds = %752
 40004|     ;; champ = ptr %403
 40005|  %773 = gep %403, i64 176                                                                                              ;L1749<1783
 40006|  %774 = load i64, ptr %773, , !!8                                                                                      ;L1749<1783
 40007|     ;; self = i64 %430
 40008|     ;; other = i64 %774
 40009|  %775 = call i64 @llvm.umax.i64(i64 %774, i64 %430)                                                                    ;L1039<1783
 40010|     ;; e_attack_tick = i64 %775
 40011|     ;; attack_tick = i64 %775
 40012|     ;; champ = ptr %403
 40013|  %776 = gep %403, i64 184                                                                                              ;L1776<1784
 40014|  %777 = load i64, ptr %776, , !!8                                                                                      ;L1776<1784
 40015|     ;; self = i64 %430
 40016|     ;; other = i64 %777
 40017|  %778 = call i64 @llvm.umax.i64(i64 %777, i64 %430)                                                                    ;L1039<1784
 40018|     ;; e_skill_tick = i64 %778
 40019|     ;; champ = ptr %403
 40020|  %779 = gep %403, i64 192                                                                                              ;L1791<1785
 40021|  %780 = load i64, ptr %779, , !!8                                                                                      ;L1791<1785
 40022|  %781 = call i64 @llvm.umax.i64(i64 %780, i64 %430)                                                                    ;L1039<1785
 40023|  br label %782                                                                                                         ;L1791<1785
 40024| 
 40025| 782: ; preds = %772, %770
 40026|  %783 = phi i1 [ true, %772 ], [ false, %770 ]
 40027|  %784 = phi i64 [ %775, %772 ], [ %771, %770 ]
 40028|  %785 = phi i64 [ %781, %772 ], [ %430, %770 ]                                                                         ;L0<1785
 40029|  %786 = phi i64 [ %778, %772 ], [ %430, %770 ]                                                                         ;L1784
 40030|     ;; e_skill_tick = i64 %786
 40031|     ;; self = i64 %430
 40032|     ;; other = i64 %785
 40033|     ;; e_skill2_tick = i64 %785
 40034|     ;; skill2_tick = i64 %785
 40035|     ;; e_skill_eff = ptr %403
 40036|     ;; self = ptr %403
 40037|  %787 = icmp ugt i64 %736, 2                                                                                           ;L1693<1787
 40038|  %788 = gep %403, i64 1280                                                                                             ;L1693<1787
 40039|  %789 = select i1 %787, ptr %788, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                        ;L1693<1787
 40040|     ;; e_skill2_eff = ptr %789
 40041|     ;; self = ptr %47
 40042|     ;; self = ptr %47
 40043|  %790 = load ptr, ptr %67, , !!8, !!8                                                                                  ;L138<2083<1788
 40044|     ;; ptr = ptr %790
 40045|  %791 = load i64, ptr %70, , !!8                                                                                       ;L2085<1788
 40046|     ;; len = i64 %791
 40047|     ;; count = i64 %791
 40048|     ;; self[0..+8] = ptr %790
 40049|     ;; slice[0..+8] = ptr %790
 40050|     ;; self[8..+8] = i64 %791
 40051|     ;; slice[8..+8] = i64 %791
 40052|     ;; ptr = ptr %790
 40053|     ;; self = ptr %790
 40054|  %792 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %790, i64 %791 ;L961<240<1062<1788
 40055|     ;; iter[0..+8] = ptr %790
 40056|     ;; iter[8..+8] = ptr %792
 40057|  %793 = add i64 %732, %740                                                                                             ;L1788
 40058|  %794 = add i64 %793, %738                                                                                             ;L1788
 40059|  %795 = add i64 %794, %753                                                                                             ;L1788
 40060|  %796 = gep %403, i64 1632
 40061|  %797 = gep %403, i64 1640
 40062|  %798 = icmp ult i64 %784, 121
 40063|  %799 = mul i64 %433, 20
 40064|  %800 = add i64 %799, %795
 40065|  %801 = gep %403, i64 1472
 40066|  %802 = gep %403, i64 1224
 40067|  %803 = gep %403, i64 1272
 40068|  %804 = gep %403, i64 1264
 40069|  %805 = gep %403, i64 1240
 40070|  %806 = gep %403, i64 1248
 40071|  %807 = gep %403, i64 1664
 40072|  %808 = add nsw i64 %743, 100
 40073|  %809 = icmp ult i64 %786, 121
 40074|  %810 = add i64 %740, %799
 40075|  %811 = gep %789, i64 48
 40076|  %812 = gep %789, i64 40
 40077|  %813 = gep %789, i64 16
 40078|  %814 = gep %789, i64 24
 40079|  %815 = icmp ult i64 %785, 121
 40080|  br label %816                                                                                                         ;L1788
 40081| 
 40082| 816: ; preds = %865, %782
 40083|  %817 = phi ptr [ %790, %782 ], [ %820, %865 ]                                                                         ;L1788
 40084|     ;; iter[0..+8] = ptr %817
 40085|     ;; self = ptr undef
 40086|     ;; ptr = ptr %817
 40087|     ;; self = ptr %817
 40088|     ;; end_or_len = ptr %792
 40091|  %818 = icmp eq ptr %817, %792                                                                                         ;L1714<180<1788
 40092|  br i1 %818, label %825, label %819                                                                                    ;L180<1788
 40093| 
 40094| 819: ; preds = %816
 40095|  %820 = gep %817, i64 216                                                                                              ;L656<185<1788
 40096|     ;; iter[0..+8] = ptr %820
 40097|     ;; a = ptr %817
 40098|  %821 = gep %817, i64 88                                                                                               ;L1789
 40099|  %822 = load i64, ptr %821, , !!8                                                                                      ;L1789
 40100|  %823 = load ptr, ptr %372, , !!8                                                                                      ;L1789
 40101|  %824 = invoke ptr %823(ptr %177, i64 %822)
 40102|  to label %844 unwind label %365                                                                                       ;L1789
 40103| 
 40104| 825: ; preds = %816
 40105|  %826 = load i64, ptr %796, , !!8                                                                                      ;L2158<1837
 40106|     ;; x1 = i64 %826
 40107|     ;; self = i64 %826
 40108|  %827 = load i64, ptr %797, , !!8                                                                                      ;L2158<1837
 40109|     ;; y1 = i64 %827
 40110|     ;; self = i64 %827
 40111|  %828 = load i64, ptr %373, , !!8                                                                                      ;L2158<1837
 40112|     ;; x2 = i64 %828
 40113|     ;; other = i64 %828
 40114|  %829 = load i64, ptr %374, , !!8                                                                                      ;L2158<1837
 40115|     ;; y2 = i64 %829
 40116|     ;; other = i64 %829
 40117|  %830 = icmp ult i64 %826, %828                                                                                        ;L3147<7<2158<1837
 40118|  %831 = sub nuw i64 %828, %826                                                                                         ;L3147<7<2158<1837
 40119|  %832 = sub nuw i64 %826, %828                                                                                         ;L3147<7<2158<1837
 40120|  %833 = select i1 %830, i64 %831, i64 %832                                                                             ;L3147<7<2158<1837
 40121|     ;; dx = i64 %833
 40122|  %834 = icmp ult i64 %827, %829                                                                                        ;L3147<8<2158<1837
 40123|  %835 = sub nuw i64 %829, %827                                                                                         ;L3147<8<2158<1837
 40124|  %836 = sub nuw i64 %827, %829                                                                                         ;L3147<8<2158<1837
 40125|  %837 = select i1 %834, i64 %835, i64 %836                                                                             ;L3147<8<2158<1837
 40126|     ;; dy = i64 %837
 40127|  %838 = mul i64 %833, %833                                                                                             ;L9<2158<1837
 40128|  %839 = mul i64 %837, %837                                                                                             ;L9<2158<1837
 40129|  %840 = add i64 %839, %838                                                                                             ;L9<2158<1837
 40130|     ;; dist = i64 %840
 40131|     ;; self = ptr %403
 40132|     ;; self = ptr %434
 40133|     ;; atk = ptr %434
 40134|     ;; self = ptr %434
 40135|  %841 = load i64, ptr %731, , !!8                                                                                      ;L26<1840
 40136|  %842 = load i64, ptr %733, , !!8                                                                                      ;L26<1840
 40137|  %843 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %434, ptr %403, ptr %59)
 40138|  to label %1015 unwind label %365                                                                                      ;L1840
 40139| 
 40140| 844: ; preds = %819
 40141|  %845 = icmp eq ptr %824, null                                                                                         ;L1789
 40142|  br i1 %845, label %865, label %846                                                                                    ;L1789
 40143| 
 40144| 846: ; preds = %844
 40145|     ;; ae = ptr %824
 40146|     ;; other = ptr %824
 40147|     ;; self = ptr %824
 40148|     ;; self = ptr %824
 40149|     ;; self = ptr %824
 40150|  %847 = load i64, ptr %796, , !!8                                                                                      ;L2158<1790
 40151|     ;; x1 = i64 %847
 40152|     ;; self = i64 %847
 40153|  %848 = load i64, ptr %797, , !!8                                                                                      ;L2158<1790
 40154|     ;; y1 = i64 %848
 40155|     ;; self = i64 %848
 40156|  %849 = gep %824, i64 1632                                                                                             ;L2158<1790
 40157|  %850 = load i64, ptr %849, , !!8                                                                                      ;L2158<1790
 40158|     ;; x2 = i64 %850
 40159|     ;; other = i64 %850
 40160|  %851 = gep %824, i64 1640                                                                                             ;L2158<1790
 40161|  %852 = load i64, ptr %851, , !!8                                                                                      ;L2158<1790
 40162|     ;; y2 = i64 %852
 40163|     ;; other = i64 %852
 40164|  %853 = icmp ult i64 %847, %850                                                                                        ;L3147<7<2158<1790
 40165|  %854 = sub nuw i64 %850, %847                                                                                         ;L3147<7<2158<1790
 40166|  %855 = sub nuw i64 %847, %850                                                                                         ;L3147<7<2158<1790
 40167|  %856 = select i1 %853, i64 %854, i64 %855                                                                             ;L3147<7<2158<1790
 40168|     ;; dx = i64 %856
 40169|  %857 = icmp ult i64 %848, %852                                                                                        ;L3147<8<2158<1790
 40170|  %858 = sub nuw i64 %852, %848                                                                                         ;L3147<8<2158<1790
 40171|  %859 = sub nuw i64 %848, %852                                                                                         ;L3147<8<2158<1790
 40172|  %860 = select i1 %857, i64 %858, i64 %859                                                                             ;L3147<8<2158<1790
 40173|     ;; dy = i64 %860
 40174|  %861 = mul i64 %856, %856                                                                                             ;L9<2158<1790
 40175|  %862 = mul i64 %860, %860                                                                                             ;L9<2158<1790
 40176|  %863 = add i64 %862, %861                                                                                             ;L9<2158<1790
 40177|     ;; dist = i64 %863
 40178|  %864 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %434, ptr %403, ptr %824)
 40179|  to label %866 unwind label %365                                                                                       ;L1793
 40180| 
 40181| 865: ; preds = %1007, %986, %984, %965, %910, %844
 40182|  br label %816                                                                                                         ;L1788
 40183| 
 40184| 866: ; preds = %846
 40185|  %867 = gep %824, i64 1136                                                                                             ;L1511<1793
 40186|  %868 = load i32, ptr %867, , !!8                                                                                      ;L1511<1793
 40187|  %869 = sext i32 %868 to i64                                                                                           ;L1511<1793
 40188|     ;; mult = i64 %869
 40189|     ;; mult = i64 %869
 40190|     ;; mult = i64 %869
 40191|  %870 = icmp eq i32 %868, 0                                                                                            ;L1512<1793
 40192|  %871 = gep %824, i64 1664                                                                                             ;L0<1793
 40193|  %872 = load i64, ptr %871, , !!8                                                                                      ;L0<1793
 40194|  br i1 %870, label %877, label %873                                                                                    ;L1512<1793
 40195| 
 40196| 873: ; preds = %866
 40197|  %874 = add nsw i64 %869, 100                                                                                          ;L1515<1793
 40198|  %875 = mul i64 %872, %874                                                                                             ;L1515<1793
 40199|  %876 = udiv i64 %875, 100                                                                                             ;L1515<1793
 40200|  br label %877                                                                                                         ;L1512<1793
 40201| 
 40202| 877: ; preds = %873, %866
 40203|  %878 = phi i64 [ %876, %873 ], [ %872, %866 ]                                                                         ;L0<1793
 40205|  br i1 %798, label %882, label %879                                                                                    ;L1797
 40206| 
 40207| 879: ; preds = %900, %882, %877
 40208|     ;; skill_tick = i64 %786
 40209|  %880 = load i32, ptr %803, , !!8                                                                                      ;L742<1807
 40210|  %881 = icmp eq i32 %880, -1                                                                                           ;L742<1807
 40211|  br i1 %881, label %910, label %908                                                                                    ;L742<1807
 40212| 
 40213| 882: ; preds = %877
 40215|  %883 = add i64 %800, %864                                                                                             ;L1793
 40216|  %884 = add i64 %883, %878                                                                                             ;L1797
 40217|  %885 = mul i64 %884, %884                                                                                             ;L1797
 40218|  %886 = icmp ult i64 %885, %863                                                                                        ;L1797
 40219|  br i1 %886, label %879, label %887                                                                                    ;L1797
 40220| 
 40221| 887: ; preds = %882
 40222|  %888 = load i64, ptr %801, , !!8                                                                                      ;L1799
 40223|  %889 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %434, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %824)
 40224|  to label %890 unwind label %365                                                                                       ;L1800
 40225| 
 40226| 890: ; preds = %887
 40227|  %891 = gep %817, i64 24                                                                                               ;L1798
 40228|     ;; value[0..+8] = i64 %888
 40229|     ;; src[0..+8] = i64 %888
 40230|     ;; value[8..+8] = i64 %784
 40231|     ;; src[8..+8] = i64 %784
 40232|     ;; value[16..+8] = i64 %889
 40233|     ;; src[16..+8] = i64 %889
 40234|     ;; self = ptr %891
 40235|     ;; self = ptr %891
 40236|     ;; additional = i64 1
 40237|     ;; needed_extra_cap = i64 1
 40238|     ;; needed_extra_cap = i64 1
 40239|     ;; strategy = i8 1
 40240|  %892 = gep %817, i64 48                                                                                               ;L1428<1798
 40241|  %893 = load i64, ptr %892, , !!55121, !!8                                                                             ;L1428<1798
 40242|     ;; self = ptr %891
 40243|  %894 = gep %817, i64 40                                                                                               ;L149<1428<1798
 40244|  %895 = load i64, ptr %894, , !!55121, !!8                                                                             ;L149<1428<1798
 40245|  %896 = icmp eq i64 %893, %895                                                                                         ;L1428<1798
 40246|  br i1 %896, label %897, label %900                                                                                    ;L1428<1798
 40247| 
 40248| 897: ; preds = %890
 40249|     ;; self = ptr %891
 40250|     ;; self = ptr %891
 40251|     ;; self = ptr %891
 40252|     ;; used_cap = i64 %893
 40253|     ;; used_cap = i64 %893
 40254|  invoke void @ai::score_parameter12PossibleGainE25reserve_internal_or_panicB17_(ptr %891, i64 %893, i64 1, i1 zeroext true)
 40255|  to label %898 unwind label %365                                                                                       ;L619<430<738<1429<1798
 40256| 
 40257| 898: ; preds = %897
 40258|  %899 = load i64, ptr %892, , !!55121                                                                                  ;L1432<1798
 40259|  br label %900                                                                                                         ;L1428<1798
 40260| 
 40261| 900: ; preds = %898, %890
 40262|  %901 = phi i64 [ %893, %890 ], [ %899, %898 ]                                                                         ;L1432<1798
 40263|     ;; self = ptr %891
 40264|  %902 = load ptr, ptr %891, , !!55121, !!8, !!8                                                                        ;L138<1432<1798
 40265|     ;; self = ptr %902
 40266|     ;; count = i64 %901
 40267|  %903 = gepS %902, i64 %901                                                                                            ;L961<1432<1798
 40268|     ;; end = ptr %903
 40269|     ;; dst = ptr %903
 40270|  store i64 %888, ptr %903,                                                                                             ;L1933<1433<1798
 40271|  %904 = gep %903, i64 8                                                                                                ;L1933<1433<1798
 40272|  store i64 %784, ptr %904,                                                                                             ;L1933<1433<1798
 40273|  %905 = gep %903, i64 16                                                                                               ;L1933<1433<1798
 40274|  store i64 %889, ptr %905,                                                                                             ;L1933<1433<1798
 40275|  %906 = load i64, ptr %892, , !!55121, !!8                                                                             ;L1434<1798
 40276|  %907 = add i64 %906, 1                                                                                                ;L1434<1798
 40277|  store i64 %907, ptr %892, , !!55121                                                                                   ;L1434<1798
 40278|  br label %879                                                                                                         ;L1797
 40279| 
 40280| 908: ; preds = %879
 40281|     ;; skill = ptr %802
 40282|     ;; self = ptr %802
 40283|  %909 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %804, ptr %403, ptr %824)
 40284|  to label %913 unwind label %365                                                                                       ;L1808
 40285| 
 40286| 910: ; preds = %955, %934, %932, %913, %879
 40287|     ;; self = ptr %789
 40288|  %911 = load i32, ptr %811, , !!8                                                                                      ;L742<1822
 40289|  %912 = icmp eq i32 %911, -1                                                                                           ;L742<1822
 40290|  br i1 %912, label %865, label %963                                                                                    ;L742<1822
 40291| 
 40292| 913: ; preds = %908
 40293|  br i1 %909, label %914, label %910                                                                                    ;L1808
 40294| 
 40295| 914: ; preds = %913
 40296|  %915 = load i64, ptr %805, , !!8                                                                                      ;L26<1809
 40297|  %916 = load i64, ptr %806, , !!8                                                                                      ;L26<1809
 40298|  %917 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %802, ptr %403, ptr %824)
 40299|  to label %918 unwind label %365                                                                                       ;L1809
 40300| 
 40301| 918: ; preds = %914
 40302|  %919 = mul i64 %916, %737                                                                                             ;L26<1809
 40303|  %920 = load i64, ptr %807, , !!8                                                                                      ;L0<1809
 40304|  br i1 %744, label %924, label %921                                                                                    ;L1512<1809
 40305| 
 40306| 921: ; preds = %918
 40307|  %922 = mul i64 %920, %808                                                                                             ;L1515<1809
 40308|  %923 = udiv i64 %922, 100                                                                                             ;L1515<1809
 40309|  br label %924                                                                                                         ;L1512<1809
 40310| 
 40311| 924: ; preds = %921, %918
 40312|  %925 = phi i64 [ %923, %921 ], [ %920, %918 ]                                                                         ;L0<1809
 40313|  %926 = gep %824, i64 1664                                                                                             ;L0<1809
 40314|  %927 = load i64, ptr %926, , !!8                                                                                      ;L0<1809
 40315|  br i1 %870, label %932, label %928                                                                                    ;L1512<1809
 40316| 
 40317| 928: ; preds = %924
 40318|  %929 = add nsw i64 %869, 100                                                                                          ;L1515<1809
 40319|  %930 = mul i64 %927, %929                                                                                             ;L1515<1809
 40320|  %931 = udiv i64 %930, 100                                                                                             ;L1515<1809
 40321|  br label %932                                                                                                         ;L1512<1809
 40322| 
 40323| 932: ; preds = %928, %924
 40324|  %933 = phi i64 [ %931, %928 ], [ %927, %924 ]                                                                         ;L0<1809
 40326|  br i1 %809, label %934, label %910                                                                                    ;L1810
 40327| 
 40328| 934: ; preds = %932
 40330|  %935 = add i64 %810, %915                                                                                             ;L26<1809
 40331|  %936 = add i64 %935, %919                                                                                             ;L1809
 40332|  %937 = add i64 %936, %917                                                                                             ;L1809
 40333|  %938 = add i64 %937, %925                                                                                             ;L1809
 40334|  %939 = add i64 %938, %933                                                                                             ;L1810
 40335|  %940 = mul i64 %939, %939                                                                                             ;L1810
 40336|  %941 = icmp ult i64 %940, %863                                                                                        ;L1810
 40337|  br i1 %941, label %910, label %942                                                                                    ;L1810
 40338| 
 40339| 942: ; preds = %934
 40340|  %943 = load i64, ptr %801, , !!8                                                                                      ;L1812
 40341|  %944 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %802, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %824)
 40342|  to label %945 unwind label %365                                                                                       ;L1813
 40343| 
 40344| 945: ; preds = %942
 40345|  %946 = gep %817, i64 24                                                                                               ;L1811
 40346|     ;; value[0..+8] = i64 %943
 40347|     ;; src[0..+8] = i64 %943
 40348|     ;; value[8..+8] = i64 %786
 40349|     ;; src[8..+8] = i64 %786
 40350|     ;; value[16..+8] = i64 %944
 40351|     ;; src[16..+8] = i64 %944
 40352|     ;; self = ptr %946
 40353|     ;; self = ptr %946
 40354|     ;; additional = i64 1
 40355|     ;; needed_extra_cap = i64 1
 40356|     ;; needed_extra_cap = i64 1
 40357|     ;; strategy = i8 1
 40358|  %947 = gep %817, i64 48                                                                                               ;L1428<1811
 40359|  %948 = load i64, ptr %947, , !!55167, !!8                                                                             ;L1428<1811
 40360|     ;; self = ptr %946
 40361|  %949 = gep %817, i64 40                                                                                               ;L149<1428<1811
 40362|  %950 = load i64, ptr %949, , !!55167, !!8                                                                             ;L149<1428<1811
 40363|  %951 = icmp eq i64 %948, %950                                                                                         ;L1428<1811
 40364|  br i1 %951, label %952, label %955                                                                                    ;L1428<1811
 40365| 
 40366| 952: ; preds = %945
 40367|     ;; self = ptr %946
 40368|     ;; self = ptr %946
 40369|     ;; self = ptr %946
 40370|     ;; used_cap = i64 %948
 40371|     ;; used_cap = i64 %948
 40372|  invoke void @ai::score_parameter12PossibleGainE25reserve_internal_or_panicB17_(ptr %946, i64 %948, i64 1, i1 zeroext true)
 40373|  to label %953 unwind label %365                                                                                       ;L619<430<738<1429<1811
 40374| 
 40375| 953: ; preds = %952
 40376|  %954 = load i64, ptr %947, , !!55167                                                                                  ;L1432<1811
 40377|  br label %955                                                                                                         ;L1428<1811
 40378| 
 40379| 955: ; preds = %953, %945
 40380|  %956 = phi i64 [ %948, %945 ], [ %954, %953 ]                                                                         ;L1432<1811
 40381|     ;; self = ptr %946
 40382|  %957 = load ptr, ptr %946, , !!55167, !!8, !!8                                                                        ;L138<1432<1811
 40383|     ;; self = ptr %957
 40384|     ;; count = i64 %956
 40385|  %958 = gepS %957, i64 %956                                                                                            ;L961<1432<1811
 40386|     ;; end = ptr %958
 40387|     ;; dst = ptr %958
 40388|  store i64 %943, ptr %958,                                                                                             ;L1933<1433<1811
 40389|  %959 = gep %958, i64 8                                                                                                ;L1933<1433<1811
 40390|  store i64 %786, ptr %959,                                                                                             ;L1933<1433<1811
 40391|  %960 = gep %958, i64 16                                                                                               ;L1933<1433<1811
 40392|  store i64 %944, ptr %960,                                                                                             ;L1933<1433<1811
 40393|  %961 = load i64, ptr %947, , !!55167, !!8                                                                             ;L1434<1811
 40394|  %962 = add i64 %961, 1                                                                                                ;L1434<1811
 40395|  store i64 %962, ptr %947, , !!55167                                                                                   ;L1434<1811
 40396|  br label %910                                                                                                         ;L1810
 40397| 
 40398| 963: ; preds = %910
 40399|     ;; skill2 = ptr %789
 40400|     ;; self = ptr %789
 40401|  %964 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %812, ptr %403, ptr %824)
 40402|  to label %965 unwind label %365                                                                                       ;L1823
 40403| 
 40404| 965: ; preds = %963
 40405|  br i1 %964, label %966, label %865                                                                                    ;L1823
 40406| 
 40407| 966: ; preds = %965
 40408|  %967 = load i64, ptr %813, , !!8                                                                                      ;L26<1824
 40409|  %968 = load i64, ptr %814, , !!8                                                                                      ;L26<1824
 40410|  %969 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %789, ptr %403, ptr %824)
 40411|  to label %970 unwind label %365                                                                                       ;L1824
 40412| 
 40413| 970: ; preds = %966
 40414|  %971 = mul i64 %968, %737                                                                                             ;L26<1824
 40415|  %972 = load i64, ptr %807, , !!8                                                                                      ;L0<1824
 40416|  br i1 %744, label %976, label %973                                                                                    ;L1512<1824
 40417| 
 40418| 973: ; preds = %970
 40419|  %974 = mul i64 %972, %808                                                                                             ;L1515<1824
 40420|  %975 = udiv i64 %974, 100                                                                                             ;L1515<1824
 40421|  br label %976                                                                                                         ;L1512<1824
 40422| 
 40423| 976: ; preds = %973, %970
 40424|  %977 = phi i64 [ %975, %973 ], [ %972, %970 ]                                                                         ;L0<1824
 40425|  %978 = gep %824, i64 1664                                                                                             ;L0<1824
 40426|  %979 = load i64, ptr %978, , !!8                                                                                      ;L0<1824
 40427|  br i1 %870, label %984, label %980                                                                                    ;L1512<1824
 40428| 
 40429| 980: ; preds = %976
 40430|  %981 = add nsw i64 %869, 100                                                                                          ;L1515<1824
 40431|  %982 = mul i64 %979, %981                                                                                             ;L1515<1824
 40432|  %983 = udiv i64 %982, 100                                                                                             ;L1515<1824
 40433|  br label %984                                                                                                         ;L1512<1824
 40434| 
 40435| 984: ; preds = %980, %976
 40436|  %985 = phi i64 [ %983, %980 ], [ %979, %976 ]                                                                         ;L0<1824
 40438|  br i1 %815, label %986, label %865                                                                                    ;L1825
 40439| 
 40440| 986: ; preds = %984
 40442|  %987 = add i64 %810, %967                                                                                             ;L26<1824
 40443|  %988 = add i64 %987, %971                                                                                             ;L1824
 40444|  %989 = add i64 %988, %969                                                                                             ;L1824
 40445|  %990 = add i64 %989, %977                                                                                             ;L1824
 40446|  %991 = add i64 %990, %985                                                                                             ;L1825
 40447|  %992 = mul i64 %991, %991                                                                                             ;L1825
 40448|  %993 = icmp ult i64 %992, %863                                                                                        ;L1825
 40449|  br i1 %993, label %865, label %994                                                                                    ;L1825
 40450| 
 40451| 994: ; preds = %986
 40452|  %995 = load i64, ptr %801, , !!8                                                                                      ;L1827
 40453|  %996 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %789, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %824)
 40454|  to label %997 unwind label %365                                                                                       ;L1828
 40455| 
 40456| 997: ; preds = %994
 40457|  %998 = gep %817, i64 24                                                                                               ;L1826
 40458|     ;; value[0..+8] = i64 %995
 40459|     ;; src[0..+8] = i64 %995
 40460|     ;; value[8..+8] = i64 %785
 40461|     ;; src[8..+8] = i64 %785
 40462|     ;; value[16..+8] = i64 %996
 40463|     ;; src[16..+8] = i64 %996
 40464|     ;; self = ptr %998
 40465|     ;; self = ptr %998
 40466|     ;; additional = i64 1
 40467|     ;; needed_extra_cap = i64 1
 40468|     ;; needed_extra_cap = i64 1
 40469|     ;; strategy = i8 1
 40470|  %999 = gep %817, i64 48                                                                                               ;L1428<1826
 40471|  %1000 = load i64, ptr %999, , !!55210, !!8                                                                            ;L1428<1826
 40472|     ;; self = ptr %998
 40473|  %1001 = gep %817, i64 40                                                                                              ;L149<1428<1826
 40474|  %1002 = load i64, ptr %1001, , !!55210, !!8                                                                           ;L149<1428<1826
 40475|  %1003 = icmp eq i64 %1000, %1002                                                                                      ;L1428<1826
 40476|  br i1 %1003, label %1004, label %1007                                                                                 ;L1428<1826
 40477| 
 40478| 1004: ; preds = %997
 40479|     ;; self = ptr %998
 40480|     ;; self = ptr %998
 40481|     ;; self = ptr %998
 40482|     ;; used_cap = i64 %1000
 40483|     ;; used_cap = i64 %1000
 40484|  invoke void @ai::score_parameter12PossibleGainE25reserve_internal_or_panicB17_(ptr %998, i64 %1000, i64 1, i1 zeroext true)
 40485|  to label %1005 unwind label %365                                                                                      ;L619<430<738<1429<1826
 40486| 
 40487| 1005: ; preds = %1004
 40488|  %1006 = load i64, ptr %999, , !!55210                                                                                 ;L1432<1826
 40489|  br label %1007                                                                                                        ;L1428<1826
 40490| 
 40491| 1007: ; preds = %1005, %997
 40492|  %1008 = phi i64 [ %1000, %997 ], [ %1006, %1005 ]                                                                     ;L1432<1826
 40493|     ;; self = ptr %998
 40494|  %1009 = load ptr, ptr %998, , !!55210, !!8, !!8                                                                       ;L138<1432<1826
 40495|     ;; self = ptr %1009
 40496|     ;; count = i64 %1008
 40497|  %1010 = gepS %1009, i64 %1008                                                                                         ;L961<1432<1826
 40498|     ;; end = ptr %1010
 40499|     ;; dst = ptr %1010
 40500|  store i64 %995, ptr %1010,                                                                                            ;L1933<1433<1826
 40501|  %1011 = gep %1010, i64 8                                                                                              ;L1933<1433<1826
 40502|  store i64 %785, ptr %1011,                                                                                            ;L1933<1433<1826
 40503|  %1012 = gep %1010, i64 16                                                                                             ;L1933<1433<1826
 40504|  store i64 %996, ptr %1012,                                                                                            ;L1933<1433<1826
 40505|  %1013 = load i64, ptr %999, , !!55210, !!8                                                                            ;L1434<1826
 40506|  %1014 = add i64 %1013, 1                                                                                              ;L1434<1826
 40507|  store i64 %1014, ptr %999, , !!55210                                                                                  ;L1434<1826
 40508|  br label %865                                                                                                         ;L1825
 40509| 
 40510| 1015: ; preds = %825
 40511|  %1016 = mul i64 %842, %737                                                                                            ;L26<1840
 40512|  %1017 = load i32, ptr %375, , !!8                                                                                     ;L1511<1840
 40513|  %1018 = sext i32 %1017 to i64                                                                                         ;L1511<1840
 40514|     ;; mult = i64 %1018
 40515|     ;; mult = i64 %1018
 40516|     ;; mult = i64 %1018
 40517|  %1019 = icmp eq i32 %1017, 0                                                                                          ;L1512<1840
 40518|  %1020 = load i64, ptr %376, , !!8                                                                                     ;L0<1840
 40519|  br i1 %1019, label %1025, label %1021                                                                                 ;L1512<1840
 40520| 
 40521| 1021: ; preds = %1015
 40522|  %1022 = add nsw i64 %1018, 100                                                                                        ;L1515<1840
 40523|  %1023 = mul i64 %1020, %1022                                                                                          ;L1515<1840
 40524|  %1024 = udiv i64 %1023, 100                                                                                           ;L1515<1840
 40525|  br label %1025                                                                                                        ;L1512<1840
 40526| 
 40527| 1025: ; preds = %1021, %1015
 40528|  %1026 = phi i64 [ %1024, %1021 ], [ %1020, %1015 ]                                                                    ;L0<1840
 40529|  %1027 = load i64, ptr %807, , !!8                                                                                     ;L0<1840
 40530|  br i1 %744, label %1031, label %1028                                                                                  ;L1512<1840
 40531| 
 40532| 1028: ; preds = %1025
 40533|  %1029 = mul i64 %1027, %808                                                                                           ;L1515<1840
 40534|  %1030 = udiv i64 %1029, 100                                                                                           ;L1515<1840
 40535|  br label %1031                                                                                                        ;L1512<1840
 40536| 
 40537| 1031: ; preds = %1028, %1025
 40538|  %1032 = phi i64 [ %1030, %1028 ], [ %1027, %1025 ]                                                                    ;L0<1840
 40540|  switch i64 %755, label %756 [
 40541|  i64 0, label %1046
 40542|  i64 1, label %1041
 40543|  i64 2, label %1033
 40544|  i64 3, label %1046
 40545|  i64 4, label %1034
 40546|  i64 5, label %1035
 40547|  i64 6, label %1035
 40548|  i64 7, label %1034
 40549|  i64 8, label %1036
 40550|  i64 9, label %1037
 40551|  i64 10, label %1038
 40552|  i64 11, label %1039
 40553|  i64 12, label %1040
 40554|  i64 13, label %1036
 40555|  ]                                                                                                                     ;L1748<1842
 40556| 
 40557| 1033: ; preds = %1031
 40558|     ;; info = ptr %403
 40559|  br label %1041                                                                                                        ;L1758<1842
 40560| 
 40561| 1034: ; preds = %1031, %1031
 40562|     ;; info = ptr %403
 40563|  br label %1041                                                                                                        ;L1750<1842
 40564| 
 40565| 1035: ; preds = %1031, %1031
 40566|     ;; info = ptr %403
 40567|  br label %1041                                                                                                        ;L1760<1842
 40568| 
 40569| 1036: ; preds = %1031, %1031
 40570|     ;; info = ptr %403
 40571|  br label %1041                                                                                                        ;L1753<1842
 40572| 
 40573| 1037: ; preds = %1031
 40574|     ;; info = ptr %403
 40575|  br label %1041                                                                                                        ;L1754<1842
 40576| 
 40577| 1038: ; preds = %1031
 40578|     ;; info = ptr %403
 40579|  br label %1041                                                                                                        ;L1755<1842
 40580| 
 40581| 1039: ; preds = %1031
 40582|     ;; info = ptr %403
 40583|  br label %1041                                                                                                        ;L1756<1842
 40584| 
 40585| 1040: ; preds = %1031
 40586|     ;; info = ptr %403
 40587|  br label %1041                                                                                                        ;L1757<1842
 40588| 
 40589| 1041: ; preds = %1040, %1039, %1038, %1037, %1036, %1035, %1034, %1033, %1031
 40590|  %1042 = phi i64 [ 216, %1039 ], [ 272, %1033 ], [ 232, %1034 ], [ 496, %1035 ], [ 184, %1031 ], [ 208, %1040 ], [ 176, %1036 ], [ 200, %1037 ], [ 240, %1038 ]
 40591|  %1043 = gep %403, i64 %1042                                                                                           ;L0<1842
 40592|  %1044 = load i64, ptr %1043, , !!8                                                                                    ;L0<1842
 40593|  %1045 = call i64 @llvm.umax.i64(i64 %1044, i64 %430)                                                                  ;L1039<1842
 40594|  br label %1046                                                                                                        ;L1039<1842
 40595| 
 40596| 1046: ; preds = %1041, %1031, %1031
 40597|  %1047 = phi i64 [ %430, %1031 ], [ %430, %1031 ], [ %1045, %1041 ]                                                    ;L0<1842
 40598|     ;; self = i64 %430
 40599|     ;; other = i64 %1047
 40600|     ;; attack_tick = i64 %1047
 40601|  %1048 = icmp ult i64 %1047, 121                                                                                       ;L1844
 40602|  br i1 %1048, label %1050, label %1049                                                                                 ;L1844
 40603| 
 40604| 1049: ; preds = %1068, %1050, %1046
 40605|  br i1 %783, label %1076, label %1080                                                                                  ;L1775<1852
 40606| 
 40607| 1050: ; preds = %1046
 40608|  %1051 = add i64 %810, %841                                                                                            ;L26<1840
 40609|  %1052 = add i64 %1051, %1016                                                                                          ;L1840
 40610|  %1053 = add i64 %1052, %843                                                                                           ;L1840
 40611|  %1054 = add i64 %1053, %1026                                                                                          ;L1840
 40612|  %1055 = add i64 %1054, %1032                                                                                          ;L1844
 40613|  %1056 = mul i64 %1055, %1055                                                                                          ;L1844
 40614|  %1057 = icmp ult i64 %1056, %840                                                                                      ;L1844
 40615|  br i1 %1057, label %1049, label %1058                                                                                 ;L1844
 40616| 
 40617| 1058: ; preds = %1050
 40618|  %1059 = load i64, ptr %801, , !!8                                                                                     ;L1846
 40619|  %1060 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %434, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 40620|  to label %1061 unwind label %365                                                                                      ;L1847
 40621| 
 40622| 1061: ; preds = %1058
 40623|     ;; value[0..+8] = i64 %1059
 40624|     ;; src[0..+8] = i64 %1059
 40625|     ;; value[8..+8] = i64 %1047
 40626|     ;; src[8..+8] = i64 %1047
 40627|     ;; value[16..+8] = i64 %1060
 40628|     ;; src[16..+8] = i64 %1060
 40629|     ;; self = ptr %77
 40630|     ;; self = ptr %77
 40631|     ;; additional = i64 1
 40632|     ;; needed_extra_cap = i64 1
 40633|     ;; needed_extra_cap = i64 1
 40634|     ;; strategy = i8 1
 40635|  %1062 = load i64, ptr %80, , !!55275, !!8                                                                             ;L1428<1845
 40636|     ;; self = ptr %77
 40637|  %1063 = load i64, ptr %79, , !!55275, !!8                                                                             ;L149<1428<1845
 40638|  %1064 = icmp eq i64 %1062, %1063                                                                                      ;L1428<1845
 40639|  br i1 %1064, label %1065, label %1068                                                                                 ;L1428<1845
 40640| 
 40641| 1065: ; preds = %1061
 40642|     ;; self = ptr %77
 40643|     ;; self = ptr %77
 40644|     ;; self = ptr %77
 40645|     ;; used_cap = i64 %1062
 40646|     ;; used_cap = i64 %1062
 40647|  invoke void @ai::score_parameter12PossibleGainE25reserve_internal_or_panicB17_(ptr %77, i64 %1062, i64 1, i1 zeroext true)
 40648|  to label %1066 unwind label %365                                                                                      ;L619<430<738<1429<1845
 40649| 
 40650| 1066: ; preds = %1065
 40651|  %1067 = load i64, ptr %80, , !!55275                                                                                  ;L1432<1845
 40652|  br label %1068                                                                                                        ;L1428<1845
 40653| 
 40654| 1068: ; preds = %1066, %1061
 40655|  %1069 = phi i64 [ %1062, %1061 ], [ %1067, %1066 ]                                                                    ;L1432<1845
 40656|     ;; self = ptr %77
 40657|  %1070 = load ptr, ptr %77, , !!55275, !!8, !!8                                                                        ;L138<1432<1845
 40658|     ;; self = ptr %1070
 40659|     ;; count = i64 %1069
 40660|  %1071 = gepS %1070, i64 %1069                                                                                         ;L961<1432<1845
 40661|     ;; end = ptr %1071
 40662|     ;; dst = ptr %1071
 40663|  store i64 %1059, ptr %1071,                                                                                           ;L1933<1433<1845
 40664|  %1072 = gep %1071, i64 8                                                                                              ;L1933<1433<1845
 40665|  store i64 %1047, ptr %1072,                                                                                           ;L1933<1433<1845
 40666|  %1073 = gep %1071, i64 16                                                                                             ;L1933<1433<1845
 40667|  store i64 %1060, ptr %1073,                                                                                           ;L1933<1433<1845
 40668|  %1074 = load i64, ptr %80, , !!55275, !!8                                                                             ;L1434<1845
 40669|  %1075 = add i64 %1074, 1                                                                                              ;L1434<1845
 40670|  store i64 %1075, ptr %80, , !!55275                                                                                   ;L1434<1845
 40671|  br label %1049                                                                                                        ;L1844
 40672| 
 40673| 1076: ; preds = %1049
 40674|     ;; champ = ptr %403
 40675|  %1077 = gep %403, i64 184                                                                                             ;L1776<1852
 40676|  %1078 = load i64, ptr %1077, , !!8                                                                                    ;L1776<1852
 40677|  %1079 = call i64 @llvm.umax.i64(i64 %1078, i64 %430)                                                                  ;L1039<1852
 40678|  br label %1080                                                                                                        ;L1776<1852
 40679| 
 40680| 1080: ; preds = %1076, %1049
 40681|  %1081 = phi i64 [ %1079, %1076 ], [ %430, %1049 ]                                                                     ;L0<1852
 40682|     ;; self = i64 %430
 40683|     ;; other = i64 %1081
 40684|     ;; skill_tick = i64 %1081
 40685|     ;; self = ptr %403
 40686|  %1082 = load i32, ptr %803, , !!8                                                                                     ;L742<1854
 40687|  %1083 = icmp eq i32 %1082, -1                                                                                         ;L742<1854
 40688|  br i1 %1083, label %1086, label %1084                                                                                 ;L742<1854
 40689| 
 40690| 1084: ; preds = %1080
 40691|     ;; skill = ptr %802
 40692|     ;; self = ptr %802
 40693|  %1085 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %804, ptr %403, ptr %59)
 40694|  to label %1087 unwind label %365                                                                                      ;L1855
 40695| 
 40696| 1086: ; preds = %1126, %1108, %1105, %1087, %1080
 40697|  br i1 %783, label %1134, label %1138                                                                                  ;L1790<1867
 40698| 
 40699| 1087: ; preds = %1084
 40700|  br i1 %1085, label %1088, label %1086                                                                                 ;L1855
 40701| 
 40702| 1088: ; preds = %1087
 40703|  %1089 = load i64, ptr %805, , !!8                                                                                     ;L26<1856
 40704|  %1090 = load i64, ptr %806, , !!8                                                                                     ;L26<1856
 40705|  %1091 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %802, ptr %403, ptr %59)
 40706|  to label %1092 unwind label %365                                                                                      ;L1856
 40707| 
 40708| 1092: ; preds = %1088
 40709|  %1093 = mul i64 %1090, %737                                                                                           ;L26<1856
 40710|  %1094 = load i64, ptr %376, , !!8                                                                                     ;L0<1856
 40711|  br i1 %1019, label %1099, label %1095                                                                                 ;L1512<1856
 40712| 
 40713| 1095: ; preds = %1092
 40714|  %1096 = add nsw i64 %1018, 100                                                                                        ;L1515<1856
 40715|  %1097 = mul i64 %1094, %1096                                                                                          ;L1515<1856
 40716|  %1098 = udiv i64 %1097, 100                                                                                           ;L1515<1856
 40717|  br label %1099                                                                                                        ;L1512<1856
 40718| 
 40719| 1099: ; preds = %1095, %1092
 40720|  %1100 = phi i64 [ %1098, %1095 ], [ %1094, %1092 ]                                                                    ;L0<1856
 40721|  %1101 = load i64, ptr %807, , !!8                                                                                     ;L0<1856
 40722|  br i1 %744, label %1105, label %1102                                                                                  ;L1512<1856
 40723| 
 40724| 1102: ; preds = %1099
 40725|  %1103 = mul i64 %1101, %808                                                                                           ;L1515<1856
 40726|  %1104 = udiv i64 %1103, 100                                                                                           ;L1515<1856
 40727|  br label %1105                                                                                                        ;L1512<1856
 40728| 
 40729| 1105: ; preds = %1102, %1099
 40730|  %1106 = phi i64 [ %1104, %1102 ], [ %1101, %1099 ]                                                                    ;L0<1856
 40732|  %1107 = icmp ult i64 %1081, 121                                                                                       ;L1857
 40733|  br i1 %1107, label %1108, label %1086                                                                                 ;L1857
 40734| 
 40735| 1108: ; preds = %1105
 40737|  %1109 = add i64 %810, %1089                                                                                           ;L26<1856
 40738|  %1110 = add i64 %1109, %1093                                                                                          ;L1856
 40739|  %1111 = add i64 %1110, %1091                                                                                          ;L1856
 40740|  %1112 = add i64 %1111, %1100                                                                                          ;L1856
 40741|  %1113 = add i64 %1112, %1106                                                                                          ;L1857
 40742|  %1114 = mul i64 %1113, %1113                                                                                          ;L1857
 40743|  %1115 = icmp ult i64 %1114, %840                                                                                      ;L1857
 40744|  br i1 %1115, label %1086, label %1116                                                                                 ;L1857
 40745| 
 40746| 1116: ; preds = %1108
 40747|  %1117 = load i64, ptr %801, , !!8                                                                                     ;L1859
 40748|  %1118 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %802, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 40749|  to label %1119 unwind label %365                                                                                      ;L1860
 40750| 
 40751| 1119: ; preds = %1116
 40752|     ;; value[0..+8] = i64 %1117
 40753|     ;; src[0..+8] = i64 %1117
 40754|     ;; value[8..+8] = i64 %1081
 40755|     ;; src[8..+8] = i64 %1081
 40756|     ;; value[16..+8] = i64 %1118
 40757|     ;; src[16..+8] = i64 %1118
 40758|     ;; self = ptr %77
 40759|     ;; self = ptr %77
 40760|     ;; additional = i64 1
 40761|     ;; needed_extra_cap = i64 1
 40762|     ;; needed_extra_cap = i64 1
 40763|     ;; strategy = i8 1
 40764|  %1120 = load i64, ptr %80, , !!55327, !!8                                                                             ;L1428<1858
 40765|     ;; self = ptr %77
 40766|  %1121 = load i64, ptr %79, , !!55327, !!8                                                                             ;L149<1428<1858
 40767|  %1122 = icmp eq i64 %1120, %1121                                                                                      ;L1428<1858
 40768|  br i1 %1122, label %1123, label %1126                                                                                 ;L1428<1858
 40769| 
 40770| 1123: ; preds = %1119
 40771|     ;; self = ptr %77
 40772|     ;; self = ptr %77
 40773|     ;; self = ptr %77
 40774|     ;; used_cap = i64 %1120
 40775|     ;; used_cap = i64 %1120
 40776|  invoke void @ai::score_parameter12PossibleGainE25reserve_internal_or_panicB17_(ptr %77, i64 %1120, i64 1, i1 zeroext true)
 40777|  to label %1124 unwind label %365                                                                                      ;L619<430<738<1429<1858
 40778| 
 40779| 1124: ; preds = %1123
 40780|  %1125 = load i64, ptr %80, , !!55327                                                                                  ;L1432<1858
 40781|  br label %1126                                                                                                        ;L1428<1858
 40782| 
 40783| 1126: ; preds = %1124, %1119
 40784|  %1127 = phi i64 [ %1120, %1119 ], [ %1125, %1124 ]                                                                    ;L1432<1858
 40785|     ;; self = ptr %77
 40786|  %1128 = load ptr, ptr %77, , !!55327, !!8, !!8                                                                        ;L138<1432<1858
 40787|     ;; self = ptr %1128
 40788|     ;; count = i64 %1127
 40789|  %1129 = gepS %1128, i64 %1127                                                                                         ;L961<1432<1858
 40790|     ;; end = ptr %1129
 40791|     ;; dst = ptr %1129
 40792|  store i64 %1117, ptr %1129,                                                                                           ;L1933<1433<1858
 40793|  %1130 = gep %1129, i64 8                                                                                              ;L1933<1433<1858
 40794|  store i64 %1081, ptr %1130,                                                                                           ;L1933<1433<1858
 40795|  %1131 = gep %1129, i64 16                                                                                             ;L1933<1433<1858
 40796|  store i64 %1118, ptr %1131,                                                                                           ;L1933<1433<1858
 40797|  %1132 = load i64, ptr %80, , !!55327, !!8                                                                             ;L1434<1858
 40798|  %1133 = add i64 %1132, 1                                                                                              ;L1434<1858
 40799|  store i64 %1133, ptr %80, , !!55327                                                                                   ;L1434<1858
 40800|  br label %1086                                                                                                        ;L1857
 40801| 
 40802| 1134: ; preds = %1086
 40803|     ;; champ = ptr %403
 40804|  %1135 = gep %403, i64 192                                                                                             ;L1791<1867
 40805|  %1136 = load i64, ptr %1135, , !!8                                                                                    ;L1791<1867
 40806|  %1137 = call i64 @llvm.umax.i64(i64 %1136, i64 %430)                                                                  ;L1039<1867
 40807|  br label %1138                                                                                                        ;L1791<1867
 40808| 
 40809| 1138: ; preds = %1134, %1086
 40810|  %1139 = phi i64 [ %1137, %1134 ], [ %430, %1086 ]                                                                     ;L0<1867
 40811|     ;; self = i64 %430
 40812|     ;; other = i64 %1139
 40813|     ;; skill2_tick = i64 %1139
 40814|     ;; self = ptr %789
 40815|  %1140 = load i32, ptr %811, , !!8                                                                                     ;L742<1869
 40816|  %1141 = icmp eq i32 %1140, -1                                                                                         ;L742<1869
 40817|  br i1 %1141, label %1144, label %1142                                                                                 ;L742<1869
 40818| 
 40819| 1142: ; preds = %1138
 40820|     ;; skill2 = ptr %789
 40821|     ;; self = ptr %789
 40822|  %1143 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %812, ptr %403, ptr %59)
 40823|  to label %1145 unwind label %365                                                                                      ;L1870
 40824| 
 40825| 1144: ; preds = %1184, %1166, %1163, %1145, %1138, %397
 40826|  br label %390                                                                                                         ;L1714<180<1686
 40827| 
 40828| 1145: ; preds = %1142
 40829|  br i1 %1143, label %1146, label %1144                                                                                 ;L1870
 40830| 
 40831| 1146: ; preds = %1145
 40832|  %1147 = load i64, ptr %813, , !!8                                                                                     ;L26<1871
 40833|  %1148 = load i64, ptr %814, , !!8                                                                                     ;L26<1871
 40834|  %1149 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %789, ptr %403, ptr %59)
 40835|  to label %1150 unwind label %365                                                                                      ;L1871
 40836| 
 40837| 1150: ; preds = %1146
 40838|  %1151 = mul i64 %1148, %737                                                                                           ;L26<1871
 40839|  %1152 = load i64, ptr %376, , !!8                                                                                     ;L0<1871
 40840|  br i1 %1019, label %1157, label %1153                                                                                 ;L1512<1871
 40841| 
 40842| 1153: ; preds = %1150
 40843|  %1154 = add nsw i64 %1018, 100                                                                                        ;L1515<1871
 40844|  %1155 = mul i64 %1152, %1154                                                                                          ;L1515<1871
 40845|  %1156 = udiv i64 %1155, 100                                                                                           ;L1515<1871
 40846|  br label %1157                                                                                                        ;L1512<1871
 40847| 
 40848| 1157: ; preds = %1153, %1150
 40849|  %1158 = phi i64 [ %1156, %1153 ], [ %1152, %1150 ]                                                                    ;L0<1871
 40850|  %1159 = load i64, ptr %807, , !!8                                                                                     ;L0<1871
 40851|  br i1 %744, label %1163, label %1160                                                                                  ;L1512<1871
 40852| 
 40853| 1160: ; preds = %1157
 40854|  %1161 = mul i64 %1159, %808                                                                                           ;L1515<1871
 40855|  %1162 = udiv i64 %1161, 100                                                                                           ;L1515<1871
 40856|  br label %1163                                                                                                        ;L1512<1871
 40857| 
 40858| 1163: ; preds = %1160, %1157
 40859|  %1164 = phi i64 [ %1162, %1160 ], [ %1159, %1157 ]                                                                    ;L0<1871
 40861|  %1165 = icmp ult i64 %1139, 121                                                                                       ;L1872
 40862|  br i1 %1165, label %1166, label %1144                                                                                 ;L1872
 40863| 
 40864| 1166: ; preds = %1163
 40866|  %1167 = add i64 %810, %1147                                                                                           ;L26<1871
 40867|  %1168 = add i64 %1167, %1151                                                                                          ;L1871
 40868|  %1169 = add i64 %1168, %1149                                                                                          ;L1871
 40869|  %1170 = add i64 %1169, %1158                                                                                          ;L1871
 40870|  %1171 = add i64 %1170, %1164                                                                                          ;L1872
 40871|  %1172 = mul i64 %1171, %1171                                                                                          ;L1872
 40872|  %1173 = icmp ult i64 %1172, %840                                                                                      ;L1872
 40873|  br i1 %1173, label %1144, label %1174                                                                                 ;L1872
 40874| 
 40875| 1174: ; preds = %1166
 40876|  %1175 = load i64, ptr %801, , !!8                                                                                     ;L1874
 40877|  %1176 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %789, ptr %64, ptr %403, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 40878|  to label %1177 unwind label %365                                                                                      ;L1875
 40879| 
 40880| 1177: ; preds = %1174
 40881|     ;; value[0..+8] = i64 %1175
 40882|     ;; src[0..+8] = i64 %1175
 40883|     ;; value[8..+8] = i64 %1139
 40884|     ;; src[8..+8] = i64 %1139
 40885|     ;; value[16..+8] = i64 %1176
 40886|     ;; src[16..+8] = i64 %1176
 40887|     ;; self = ptr %77
 40888|     ;; self = ptr %77
 40889|     ;; additional = i64 1
 40890|     ;; needed_extra_cap = i64 1
 40891|     ;; needed_extra_cap = i64 1
 40892|     ;; strategy = i8 1
 40893|  %1178 = load i64, ptr %80, , !!55377, !!8                                                                             ;L1428<1873
 40894|     ;; self = ptr %77
 40895|  %1179 = load i64, ptr %79, , !!55377, !!8                                                                             ;L149<1428<1873
 40896|  %1180 = icmp eq i64 %1178, %1179                                                                                      ;L1428<1873
 40897|  br i1 %1180, label %1181, label %1184                                                                                 ;L1428<1873
 40898| 
 40899| 1181: ; preds = %1177
 40900|     ;; self = ptr %77
 40901|     ;; self = ptr %77
 40902|     ;; self = ptr %77
 40903|     ;; used_cap = i64 %1178
 40904|     ;; used_cap = i64 %1178
 40905|  invoke void @ai::score_parameter12PossibleGainE25reserve_internal_or_panicB17_(ptr %77, i64 %1178, i64 1, i1 zeroext true)
 40906|  to label %1182 unwind label %365                                                                                      ;L619<430<738<1429<1873
 40907| 
 40908| 1182: ; preds = %1181
 40909|  %1183 = load i64, ptr %80, , !!55377                                                                                  ;L1432<1873
 40910|  br label %1184                                                                                                        ;L1428<1873
 40911| 
 40912| 1184: ; preds = %1182, %1177
 40913|  %1185 = phi i64 [ %1178, %1177 ], [ %1183, %1182 ]                                                                    ;L1432<1873
 40914|     ;; self = ptr %77
 40915|  %1186 = load ptr, ptr %77, , !!55377, !!8, !!8                                                                        ;L138<1432<1873
 40916|     ;; self = ptr %1186
 40917|     ;; count = i64 %1185
 40918|  %1187 = gepS %1186, i64 %1185                                                                                         ;L961<1432<1873
 40919|     ;; end = ptr %1187
 40920|     ;; dst = ptr %1187
 40921|  store i64 %1175, ptr %1187,                                                                                           ;L1933<1433<1873
 40922|  %1188 = gep %1187, i64 8                                                                                              ;L1933<1433<1873
 40923|  store i64 %1139, ptr %1188,                                                                                           ;L1933<1433<1873
 40924|  %1189 = gep %1187, i64 16                                                                                             ;L1933<1433<1873
 40925|  store i64 %1176, ptr %1189,                                                                                           ;L1933<1433<1873
 40926|  %1190 = load i64, ptr %80, , !!55377, !!8                                                                             ;L1434<1873
 40927|  %1191 = add i64 %1190, 1                                                                                              ;L1434<1873
 40928|  store i64 %1191, ptr %80, , !!55377                                                                                   ;L1434<1873
 40929|  br label %1144                                                                                                        ;L1872
 40930| 
 40931| 1192: ; preds = %1203, %393
 40932|  %1193 = phi ptr [ %394, %393 ], [ %1196, %1203 ]                                                                      ;L1883
 40933|     ;; iter[0..+8] = ptr %1193
 40934|     ;; self = ptr undef
 40935|     ;; ptr = ptr %1193
 40936|     ;; self = ptr %1193
 40937|     ;; end_or_len = ptr %396
 40940|  %1194 = icmp eq ptr %1193, %396                                                                                       ;L1714<180<1883
 40941|  br i1 %1194, label %1701, label %1195                                                                                 ;L180<1883
 40942| 
 40943| 1195: ; preds = %1192
 40944|  %1196 = gep %1193, i64 40                                                                                             ;L656<185<1883
 40945|     ;; iter[0..+8] = ptr %1196
 40946|  %1197 = load i64, ptr %1193,                                                                                          ;L1883
 40947|     ;; a[0..+8] = i64 %1197
 40948|  %1198 = gep %1193, i64 8                                                                                              ;L1883
 40949|  %1199 = load i64, ptr %1198,                                                                                          ;L1883
 40950|     ;; a[8..+8] = i64 %1199
 40952|  %1200 = gep %1193, i64 32                                                                                             ;L1883
 40953|  %1201 = load ptr, ptr %1200, , !!8, !!8                                                                               ;L1883
 40954|     ;; e = ptr %1201
 40955|     ;; self = ptr %1201
 40956|     ;; self = ptr %1201
 40957|     ;; self = ptr %1201
 40958|     ;; self = ptr %1201
 40959|     ;; self = ptr %1201
 40960|     ;; self = ptr %1201
 40961|     ;; caster = ptr %1201
 40962|     ;; self = ptr %1201
 40963|     ;; self = ptr %1201
 40964|     ;; self = ptr %1201
 40965|     ;; self = ptr %1201
 40966|     ;; self = ptr %1201
 40967|     ;; self = ptr %1201
 40968|     ;; caster = ptr %1201
 40969|     ;; self = ptr %1201
 40970|     ;; caster = ptr %1201
 40971|     ;; self = ptr %1201
 40974|     ;; __self_discr = i64 %1197
 40975|     ;; __arg1_discr = i64 0
 40976|  %1202 = icmp eq i64 %1197, 0                                                                                          ;L81<1884
 40977|  br i1 %1202, label %1203, label %1204                                                                                 ;L1884
 40978| 
 40979| 1203: ; preds = %1521, %1195
 40980|  br label %1192                                                                                                        ;L1714<180<1883
 40981| 
 40982| 1204: ; preds = %1195
 40983|  %1205 = invoke i64 @gc::simulation6entityNtB5_6Entity18remain_action_time(ptr %1201)
 40984|  to label %1206 unwind label %365                                                                                      ;L1888
 40985| 
 40986| 1206: ; preds = %1204
 40987|     ;; self = ptr %1201
 40988|     ;; self = ptr %1201
 40989|     ;; self = ptr %1201
 40990|  %1207 = gep %1201, i64 712                                                                                            ;L614<609<296<1968<1864<3787<1888
 40991|  %1208 = load ptr, ptr %1207, , !!8, !!8                                                                               ;L614<609<296<1968<1864<3787<1888
 40992|  %1209 = gep %1201, i64 720                                                                                            ;L1864<3787<1888
 40993|  %1210 = load i64, ptr %1209, , !!8                                                                                    ;L1864<3787<1888
 40994|     ;; len = i64 %1210
 40995|     ;; count = i64 %1210
 40996|     ;; self[0..+8] = ptr %1208
 40997|     ;; slice[0..+8] = ptr %1208
 40998|     ;; self[8..+8] = i64 %1210
 40999|     ;; slice[8..+8] = i64 %1210
 41000|     ;; ptr = ptr %1208
 41001|     ;; self = ptr %1208
 41002|  %1211 = gepS %1208, i64 %1210                                                                                         ;L961<100<1042<1888
 41003|     ;; self[0..+8] = ptr %1208
 41004|     ;; self[0..+8] = ptr %1208
 41005|     ;; self[8..+8] = ptr %1211
 41006|     ;; self[8..+8] = ptr %1211
 41007|     ;; self[0..+8] = ptr %1208
 41008|     ;; self[8..+8] = ptr %1211
 41010|     ;; self = ptr undef
 41011|     ;; self = ptr undef
 41012|     ;; predicate = ptr undef
 41013|     ;; self = ptr undef
 41014|     ;; self = ptr undef
 41015|     ;; count = i64 1
 41016|  br label %1212                                                                                                        ;L348<98<107<2706<3354<3255<1889
 41017| 
 41018| 1212: ; preds = %1215, %1206
 41019|  %1213 = phi ptr [ %1216, %1215 ], [ %1208, %1206 ]
 41021|     ;; ptr = ptr %1213
 41022|     ;; self = ptr %1213
 41023|     ;; end_or_len = ptr %1211
 41026|  %1214 = icmp eq ptr %1213, %1211                                                                                      ;L1714<180<348<98<107<2706<3354<3255<1889
 41027|  br i1 %1214, label %1226, label %1215                                                                                 ;L180<348<98<107<2706<3354<3255<1889
 41028| 
 41029| 1215: ; preds = %1212
 41030|  %1216 = gep %1213, i64 40                                                                                             ;L656<185<348<98<107<2706<3354<3255<1889
 41031|     ;; self[0..+8] = ptr %1216
 41032|     ;; x = ptr %1213
 41037|     ;; self = ptr %1213
 41038|  %1217 = load i32, ptr %1213, , !!55568, !!8                                                                           ;L521<1888<298<349<98<107<2706<3354<3255<1889
 41039|  %1218 = add nsw i32 %1217, -6                                                                                         ;L521<1888<298<349<98<107<2706<3354<3255<1889
 41040|  %1219 = icmp ult i32 %1218, -4                                                                                        ;L521<1888<298<349<98<107<2706<3354<3255<1889
 41041|  br i1 %1219, label %1220, label %1212                                                                                 ;L349<98<107<2706<3354<3255<1889
 41042| 
 41043| 1220: ; preds = %1215
 41044|     ;; self = ptr %1213
 41045|     ;; f = ptr undef
 41046|     ;; self = ptr undef
 41047|     ;; x = ptr %1213
 41048|     ;; args = ptr %1213
 41050|     ;; c = ptr %1213
 41051|     ;; self = ptr %1213
 41052|  %1221 = icmp eq i32 %1217, 10                                                                                         ;L546<1889<310<1162<107<2706<3354<3255<1889
 41053|  %1222 = select i1 %1221, i64 32, i64 8                                                                                ;L0<1889<310<1162<107<2706<3354<3255<1889
 41054|  %1223 = gep %1213, i64 %1222                                                                                          ;L0<1889<310<1162<107<2706<3354<3255<1889
 41055|  %1224 = load i64, ptr %1223, , !!55637, !!8                                                                           ;L0<1889<310<1162<107<2706<3354<3255<1889
 41056|     ;; self[0..+8] = ptr %1216
 41057|     ;; first = i64 %1224
 41058|  %1225 = invoke i64 @core::iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtNtBc_5slice4iter4IterNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity7CCStateENCNvNtCshdEBA0ozCnw_7game_ai15score_parameter25calculate_score_parametersj_0ENCB2H_sk_0ENtNtNtBa_6traits8iterator8Iterator4foldjNCINvNvB45_6max_by4foldjNvYjNtNtBc_3cmp3Ord3cmpE0EB2L_(ptr %1216, ptr %1211, i64 %1224)
 41059|  to label %1226 unwind label %365                                                                                      ;L2707<3354<3255<1889
 41060| 
 41061| 1226: ; preds = %1220, %1212
 41062|  %1227 = phi i64 [ %1225, %1220 ], [ undef, %1212 ]
 41065|     ;; self = i64 %1205
 41067|  %1228 = call i64 @llvm.umax.i64(i64 %1227, i64 %1205)                                                                 ;L1039<1888
 41068|  %1229 = select i1 %1214, i64 %1205, i64 %1228                                                                         ;L1039<1889
 41069|     ;; act_tick = i64 %1229
 41070|  switch i64 %1197, label %1230 [
 41071|  i64 6, label %1237
 41072|  i64 7, label %1242
 41073|  i64 8, label %1247
 41074|  i64 9, label %1256
 41075|  ]                                                                                                                     ;L1891
 41076| 
 41077| 1230: ; preds = %1433, %1429, %1398, %1390, %1386, %1355, %1347, %1343, %1312, %1304, %1300, %1269, %1226
 41078|  %1231 = gep %1201, i64 1600                                                                                           ;L1944
 41079|  %1232 = load i64, ptr %1231, , !!8                                                                                    ;L1944
 41080|     ;; e_speed = i64 %1232
 41081|     ;; speed = i64 %1232
 41082|     ;; e_atk_eff = ptr %1201
 41083|     ;; self = ptr %1201
 41084|  %1233 = gep %1201, i64 1168                                                                                           ;L742<1946
 41085|  %1234 = gep %1201, i64 1216                                                                                           ;L742<1946
 41086|  %1235 = load i32, ptr %1234, , !!8                                                                                    ;L742<1946
 41087|  %1236 = icmp eq i32 %1235, -1                                                                                         ;L742<1946
 41088|  br i1 %1236, label %1454, label %1437                                                                                 ;L742<1946
 41089| 
 41090| 1237: ; preds = %1226
 41091|     ;; target_id = i64 %1199
 41092|     ;; self = ptr %1201
 41093|  %1238 = gep %1201, i64 1168                                                                                           ;L742<1893
 41094|  %1239 = gep %1201, i64 1216                                                                                           ;L742<1893
 41095|  %1240 = load i32, ptr %1239, , !!8                                                                                    ;L742<1893
 41096|  %1241 = icmp eq i32 %1240, -1                                                                                         ;L742<1893
 41097|  br i1 %1241, label %1277, label %1265                                                                                 ;L742<1893
 41098| 
 41099| 1242: ; preds = %1226
 41100|     ;; target_id = i64 %1199
 41101|     ;; self = ptr %1201
 41102|  %1243 = gep %1201, i64 1224                                                                                           ;L742<1904
 41103|  %1244 = gep %1201, i64 1272                                                                                           ;L742<1904
 41104|  %1245 = load i32, ptr %1244, , !!8                                                                                    ;L742<1904
 41105|  %1246 = icmp eq i32 %1245, -1                                                                                         ;L742<1904
 41106|  br i1 %1246, label %1320, label %1308                                                                                 ;L742<1904
 41107| 
 41108| 1247: ; preds = %1226
 41109|     ;; target_id = i64 %1199
 41110|  %1248 = gep %1201, i64 1480                                                                                           ;L1693<1915
 41111|  %1249 = load i64, ptr %1248, , !!8                                                                                    ;L1693<1915
 41112|  %1250 = icmp ugt i64 %1249, 2                                                                                         ;L1693<1915
 41113|  %1251 = gep %1201, i64 1280                                                                                           ;L1693<1915
 41114|  %1252 = select i1 %1250, ptr %1251, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                     ;L1693<1915
 41115|     ;; self = ptr %1252
 41116|  %1253 = gep %1252, i64 48                                                                                             ;L742<1915
 41117|  %1254 = load i32, ptr %1253, , !!8                                                                                    ;L742<1915
 41118|  %1255 = icmp eq i32 %1254, -1                                                                                         ;L742<1915
 41119|  br i1 %1255, label %1363, label %1351                                                                                 ;L742<1915
 41120| 
 41121| 1256: ; preds = %1226
 41122|     ;; target_id = i64 %1199
 41123|  %1257 = gep %1201, i64 1480                                                                                           ;L1701<1928
 41124|  %1258 = load i64, ptr %1257, , !!8                                                                                    ;L1701<1928
 41125|  %1259 = icmp ugt i64 %1258, 4                                                                                         ;L1701<1928
 41126|  %1260 = gep %1201, i64 1336                                                                                           ;L1701<1928
 41127|  %1261 = select i1 %1259, ptr %1260, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                     ;L1701<1928
 41128|     ;; self = ptr %1261
 41129|  %1262 = gep %1261, i64 48                                                                                             ;L742<1928
 41130|  %1263 = load i32, ptr %1262, , !!8                                                                                    ;L742<1928
 41131|  %1264 = icmp eq i32 %1263, -1                                                                                         ;L742<1928
 41132|  br i1 %1264, label %1406, label %1394                                                                                 ;L742<1928
 41133| 
 41134| 1265: ; preds = %1237
 41135|     ;; self = ptr %1238
 41136|     ;; atk = ptr %1238
 41137|     ;; self = ptr %47
 41138|     ;; self = ptr %47
 41139|  %1266 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<1894
 41140|     ;; ptr = ptr %1266
 41141|  %1267 = load i64, ptr %74, , !!8                                                                                      ;L2085<1894
 41142|     ;; len = i64 %1267
 41143|     ;; count = i64 %1267
 41144|     ;; self[0..+8] = ptr %1266
 41145|     ;; slice[0..+8] = ptr %1266
 41146|     ;; self[8..+8] = i64 %1267
 41147|     ;; slice[8..+8] = i64 %1267
 41148|     ;; ptr = ptr %1266
 41149|     ;; self = ptr %1266
 41150|  %1268 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %1266, i64 %1267 ;L961<240<1062<1894
 41151|     ;; predicate = ptr undef
 41152|     ;; self = ptr undef
 41153|     ;; self = ptr undef
 41154|     ;; count = i64 1
 41155|  br label %1269                                                                                                        ;L348<1894
 41156| 
 41157| 1269: ; preds = %1272, %1265
 41158|  %1270 = phi ptr [ %1273, %1272 ], [ %1266, %1265 ]
 41159|     ;; ptr = ptr %1270
 41160|     ;; self = ptr %1270
 41161|     ;; end_or_len = ptr %1268
 41164|  %1271 = icmp eq ptr %1270, %1268                                                                                      ;L1714<180<348<1894
 41165|  br i1 %1271, label %1230, label %1272                                                                                 ;L180<348<1894
 41166| 
 41167| 1272: ; preds = %1269
 41168|  %1273 = gep %1270, i64 216                                                                                            ;L656<185<348<1894
 41169|     ;; x = ptr %1270
 41172|  %1274 = gep %1270, i64 88                                                                                             ;L1894<349<1894
 41173|  %1275 = load i64, ptr %1274, , !!55718, !!8                                                                           ;L1894<349<1894
 41174|  %1276 = icmp eq i64 %1275, %1199                                                                                      ;L1894<349<1894
 41175|  br i1 %1276, label %1278, label %1269                                                                                 ;L349<1894
 41176| 
 41177| 1277: ; preds = %1237
 41178|     ;; self = ptr null
 41179|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.134) #25
 41180|  to label %173 unwind label %365                                                                                       ;L1013<1893
 41181| 
 41182| 1278: ; preds = %1272
 41183|     ;; target = ptr %1270
 41184|  %1279 = load ptr, ptr %372, , !!8                                                                                     ;L1895
 41185|  %1280 = invoke ptr %1279(ptr %177, i64 %1199)
 41186|  to label %1281 unwind label %365                                                                                      ;L1895
 41187| 
 41188| 1281: ; preds = %1278
 41189|     ;; self = ptr %1280
 41190|  %1282 = icmp eq ptr %1280, null                                                                                       ;L1011<1895
 41191|  br i1 %1282, label %1287, label %1283                                                                                 ;L1011<1895
 41192| 
 41193| 1283: ; preds = %1281
 41194|     ;; target_entity = ptr %1280
 41195|     ;; self = ptr %1238
 41196|  %1284 = load i32, ptr %1239, , !!8                                                                                    ;L149<1896
 41197|  %1285 = add nsw i32 %1284, -1                                                                                         ;L149<1896
 41198|  %1286 = icmp ult i32 %1285, 2                                                                                         ;L149<1896
 41199|  br i1 %1286, label %1292, label %1288                                                                                 ;L149<1896
 41200| 
 41201| 1287: ; preds = %1281
 41202|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.135) #25
 41203|  to label %173 unwind label %365                                                                                       ;L1013<1895
 41204| 
 41205| 1288: ; preds = %1283
 41206|  %1289 = gep %1201, i64 104                                                                                            ;L1564<1896
 41207|  %1290 = load i64, ptr %1289, , !!8                                                                                    ;L1564<1896
 41208|  %1291 = icmp eq i64 %1290, 13                                                                                         ;L1564<1896
 41209|  br i1 %1291, label %1294, label %1292                                                                                 ;L1564<1896
 41210| 
 41211| 1292: ; preds = %1294, %1288, %1283
 41212|  %1293 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1238, ptr %64, ptr %1201, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1280)
 41213|  to label %1304 unwind label %365                                                                                      ;L1897
 41214| 
 41215| 1294: ; preds = %1288
 41216|     ;; champ = ptr %1201
 41217|  %1295 = gep %1201, i64 112                                                                                            ;L1565<1896
 41218|  %1296 = load i64, ptr %1295, , !!8                                                                                    ;L1565<1896
 41219|  %1297 = icmp eq i64 %1296, 3                                                                                          ;L1565<1896
 41220|  br i1 %1297, label %1298, label %1292                                                                                 ;L1896
 41221| 
 41222| 1298: ; preds = %1294
 41223|  %1299 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1238, ptr %64, ptr %1201, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1280)
 41224|  to label %1300 unwind label %365                                                                                      ;L1899
 41225| 
 41226| 1300: ; preds = %1298
 41227|  %1301 = gep %1270, i64 112                                                                                            ;L1899
 41228|  %1302 = load i64, ptr %1301, , !!8                                                                                    ;L1899
 41229|  %1303 = add i64 %1302, %1299                                                                                          ;L1899
 41230|  store i64 %1303, ptr %1301,                                                                                           ;L1899
 41231|  br label %1230                                                                                                        ;L1896
 41232| 
 41233| 1304: ; preds = %1292
 41234|  %1305 = gep %1270, i64 128                                                                                            ;L1897
 41235|  %1306 = load i64, ptr %1305, , !!8                                                                                    ;L1897
 41236|  %1307 = add i64 %1306, %1293                                                                                          ;L1897
 41237|  store i64 %1307, ptr %1305,                                                                                           ;L1897
 41238|  br label %1230                                                                                                        ;L1896
 41239| 
 41240| 1308: ; preds = %1242
 41241|     ;; self = ptr %1243
 41242|     ;; skill = ptr %1243
 41243|     ;; self = ptr %47
 41244|     ;; self = ptr %47
 41245|  %1309 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<1905
 41246|     ;; ptr = ptr %1309
 41247|  %1310 = load i64, ptr %74, , !!8                                                                                      ;L2085<1905
 41248|     ;; len = i64 %1310
 41249|     ;; count = i64 %1310
 41250|     ;; self[0..+8] = ptr %1309
 41251|     ;; slice[0..+8] = ptr %1309
 41252|     ;; self[8..+8] = i64 %1310
 41253|     ;; slice[8..+8] = i64 %1310
 41254|     ;; ptr = ptr %1309
 41255|     ;; self = ptr %1309
 41256|  %1311 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %1309, i64 %1310 ;L961<240<1062<1905
 41257|     ;; predicate = ptr undef
 41258|     ;; self = ptr undef
 41259|     ;; self = ptr undef
 41260|     ;; count = i64 1
 41261|  br label %1312                                                                                                        ;L348<1905
 41262| 
 41263| 1312: ; preds = %1315, %1308
 41264|  %1313 = phi ptr [ %1316, %1315 ], [ %1309, %1308 ]
 41265|     ;; ptr = ptr %1313
 41266|     ;; self = ptr %1313
 41267|     ;; end_or_len = ptr %1311
 41270|  %1314 = icmp eq ptr %1313, %1311                                                                                      ;L1714<180<348<1905
 41271|  br i1 %1314, label %1230, label %1315                                                                                 ;L180<348<1905
 41272| 
 41273| 1315: ; preds = %1312
 41274|  %1316 = gep %1313, i64 216                                                                                            ;L656<185<348<1905
 41275|     ;; x = ptr %1313
 41278|  %1317 = gep %1313, i64 88                                                                                             ;L1905<349<1905
 41279|  %1318 = load i64, ptr %1317, , !!55785, !!8                                                                           ;L1905<349<1905
 41280|  %1319 = icmp eq i64 %1318, %1199                                                                                      ;L1905<349<1905
 41281|  br i1 %1319, label %1321, label %1312                                                                                 ;L349<1905
 41282| 
 41283| 1320: ; preds = %1242
 41284|     ;; self = ptr null
 41285|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.136) #25
 41286|  to label %173 unwind label %365                                                                                       ;L1013<1904
 41287| 
 41288| 1321: ; preds = %1315
 41289|     ;; target = ptr %1313
 41290|  %1322 = load ptr, ptr %372, , !!8                                                                                     ;L1906
 41291|  %1323 = invoke ptr %1322(ptr %177, i64 %1199)
 41292|  to label %1324 unwind label %365                                                                                      ;L1906
 41293| 
 41294| 1324: ; preds = %1321
 41295|     ;; self = ptr %1323
 41296|  %1325 = icmp eq ptr %1323, null                                                                                       ;L1011<1906
 41297|  br i1 %1325, label %1330, label %1326                                                                                 ;L1011<1906
 41298| 
 41299| 1326: ; preds = %1324
 41300|     ;; target_entity = ptr %1323
 41301|     ;; self = ptr %1243
 41302|  %1327 = load i32, ptr %1244, , !!8                                                                                    ;L149<1907
 41303|  %1328 = add nsw i32 %1327, -1                                                                                         ;L149<1907
 41304|  %1329 = icmp ult i32 %1328, 2                                                                                         ;L149<1907
 41305|  br i1 %1329, label %1335, label %1331                                                                                 ;L149<1907
 41306| 
 41307| 1330: ; preds = %1324
 41308|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.137) #25
 41309|  to label %173 unwind label %365                                                                                       ;L1013<1906
 41310| 
 41311| 1331: ; preds = %1326
 41312|  %1332 = gep %1201, i64 104                                                                                            ;L1571<1907
 41313|  %1333 = load i64, ptr %1332, , !!8                                                                                    ;L1571<1907
 41314|  %1334 = icmp eq i64 %1333, 13                                                                                         ;L1571<1907
 41315|  br i1 %1334, label %1337, label %1335                                                                                 ;L1571<1907
 41316| 
 41317| 1335: ; preds = %1337, %1331, %1326
 41318|  %1336 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1243, ptr %64, ptr %1201, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1323)
 41319|  to label %1347 unwind label %365                                                                                      ;L1908
 41320| 
 41321| 1337: ; preds = %1331
 41322|     ;; champ = ptr %1201
 41323|  %1338 = gep %1201, i64 112                                                                                            ;L1572<1907
 41324|  %1339 = load i64, ptr %1338, , !!8                                                                                    ;L1572<1907
 41325|  %1340 = icmp eq i64 %1339, 4                                                                                          ;L1572<1907
 41326|  br i1 %1340, label %1341, label %1335                                                                                 ;L1907
 41327| 
 41328| 1341: ; preds = %1337
 41329|  %1342 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1243, ptr %64, ptr %1201, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1323)
 41330|  to label %1343 unwind label %365                                                                                      ;L1910
 41331| 
 41332| 1343: ; preds = %1341
 41333|  %1344 = gep %1313, i64 112                                                                                            ;L1910
 41334|  %1345 = load i64, ptr %1344, , !!8                                                                                    ;L1910
 41335|  %1346 = add i64 %1345, %1342                                                                                          ;L1910
 41336|  store i64 %1346, ptr %1344,                                                                                           ;L1910
 41337|  br label %1230                                                                                                        ;L1907
 41338| 
 41339| 1347: ; preds = %1335
 41340|  %1348 = gep %1313, i64 128                                                                                            ;L1908
 41341|  %1349 = load i64, ptr %1348, , !!8                                                                                    ;L1908
 41342|  %1350 = add i64 %1349, %1336                                                                                          ;L1908
 41343|  store i64 %1350, ptr %1348,                                                                                           ;L1908
 41344|  br label %1230                                                                                                        ;L1907
 41345| 
 41346| 1351: ; preds = %1247
 41347|     ;; self = ptr %1252
 41348|     ;; skill2 = ptr %1252
 41349|     ;; self = ptr %47
 41350|     ;; self = ptr %47
 41351|  %1352 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<1917
 41352|     ;; ptr = ptr %1352
 41353|  %1353 = load i64, ptr %74, , !!8                                                                                      ;L2085<1917
 41354|     ;; len = i64 %1353
 41355|     ;; count = i64 %1353
 41356|     ;; self[0..+8] = ptr %1352
 41357|     ;; slice[0..+8] = ptr %1352
 41358|     ;; self[8..+8] = i64 %1353
 41359|     ;; slice[8..+8] = i64 %1353
 41360|     ;; ptr = ptr %1352
 41361|     ;; self = ptr %1352
 41362|  %1354 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %1352, i64 %1353 ;L961<240<1062<1917
 41363|     ;; predicate = ptr undef
 41364|     ;; self = ptr undef
 41365|     ;; self = ptr undef
 41366|     ;; count = i64 1
 41367|  br label %1355                                                                                                        ;L348<1917
 41368| 
 41369| 1355: ; preds = %1358, %1351
 41370|  %1356 = phi ptr [ %1359, %1358 ], [ %1352, %1351 ]
 41371|     ;; ptr = ptr %1356
 41372|     ;; self = ptr %1356
 41373|     ;; end_or_len = ptr %1354
 41376|  %1357 = icmp eq ptr %1356, %1354                                                                                      ;L1714<180<348<1917
 41377|  br i1 %1357, label %1230, label %1358                                                                                 ;L180<348<1917
 41378| 
 41379| 1358: ; preds = %1355
 41380|  %1359 = gep %1356, i64 216                                                                                            ;L656<185<348<1917
 41381|     ;; x = ptr %1356
 41384|  %1360 = gep %1356, i64 88                                                                                             ;L1917<349<1917
 41385|  %1361 = load i64, ptr %1360, , !!55852, !!8                                                                           ;L1917<349<1917
 41386|  %1362 = icmp eq i64 %1361, %1199                                                                                      ;L1917<349<1917
 41387|  br i1 %1362, label %1364, label %1355                                                                                 ;L349<1917
 41388| 
 41389| 1363: ; preds = %1247
 41390|     ;; self = ptr null
 41391|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.138) #25
 41392|  to label %173 unwind label %365                                                                                       ;L1013<1915
 41393| 
 41394| 1364: ; preds = %1358
 41395|     ;; target = ptr %1356
 41396|  %1365 = load ptr, ptr %372, , !!8                                                                                     ;L1918
 41397|  %1366 = invoke ptr %1365(ptr %177, i64 %1199)
 41398|  to label %1367 unwind label %365                                                                                      ;L1918
 41399| 
 41400| 1367: ; preds = %1364
 41401|     ;; self = ptr %1366
 41402|  %1368 = icmp eq ptr %1366, null                                                                                       ;L1011<1918
 41403|  br i1 %1368, label %1373, label %1369                                                                                 ;L1011<1918
 41404| 
 41405| 1369: ; preds = %1367
 41406|     ;; target_entity = ptr %1366
 41407|     ;; self = ptr %1252
 41408|  %1370 = load i32, ptr %1253, , !!8                                                                                    ;L149<1919
 41409|  %1371 = add nsw i32 %1370, -1                                                                                         ;L149<1919
 41410|  %1372 = icmp ult i32 %1371, 2                                                                                         ;L149<1919
 41411|  br i1 %1372, label %1378, label %1374                                                                                 ;L149<1919
 41412| 
 41413| 1373: ; preds = %1367
 41414|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.139) #25
 41415|  to label %173 unwind label %365                                                                                       ;L1013<1918
 41416| 
 41417| 1374: ; preds = %1369
 41418|  %1375 = gep %1201, i64 104                                                                                            ;L1579<1919
 41419|  %1376 = load i64, ptr %1375, , !!8                                                                                    ;L1579<1919
 41420|  %1377 = icmp eq i64 %1376, 13                                                                                         ;L1579<1919
 41421|  br i1 %1377, label %1380, label %1378                                                                                 ;L1579<1919
 41422| 
 41423| 1378: ; preds = %1380, %1374, %1369
 41424|  %1379 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1252, ptr %64, ptr %1201, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1366)
 41425|  to label %1390 unwind label %365                                                                                      ;L1920
 41426| 
 41427| 1380: ; preds = %1374
 41428|     ;; champ = ptr %1201
 41429|  %1381 = gep %1201, i64 112                                                                                            ;L1580<1919
 41430|  %1382 = load i64, ptr %1381, , !!8                                                                                    ;L1580<1919
 41431|  %1383 = icmp eq i64 %1382, 5                                                                                          ;L1580<1919
 41432|  br i1 %1383, label %1384, label %1378                                                                                 ;L1919
 41433| 
 41434| 1384: ; preds = %1380
 41435|  %1385 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1252, ptr %64, ptr %1201, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1366)
 41436|  to label %1386 unwind label %365                                                                                      ;L1922
 41437| 
 41438| 1386: ; preds = %1384
 41439|  %1387 = gep %1356, i64 112                                                                                            ;L1922
 41440|  %1388 = load i64, ptr %1387, , !!8                                                                                    ;L1922
 41441|  %1389 = add i64 %1388, %1385                                                                                          ;L1922
 41442|  store i64 %1389, ptr %1387,                                                                                           ;L1922
 41443|  br label %1230                                                                                                        ;L1919
 41444| 
 41445| 1390: ; preds = %1378
 41446|  %1391 = gep %1356, i64 128                                                                                            ;L1920
 41447|  %1392 = load i64, ptr %1391, , !!8                                                                                    ;L1920
 41448|  %1393 = add i64 %1392, %1379                                                                                          ;L1920
 41449|  store i64 %1393, ptr %1391,                                                                                           ;L1920
 41450|  br label %1230                                                                                                        ;L1919
 41451| 
 41452| 1394: ; preds = %1256
 41453|     ;; self = ptr %1261
 41454|     ;; ult = ptr %1261
 41455|     ;; self = ptr %47
 41456|     ;; self = ptr %47
 41457|  %1395 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<1930
 41458|     ;; ptr = ptr %1395
 41459|  %1396 = load i64, ptr %74, , !!8                                                                                      ;L2085<1930
 41460|     ;; len = i64 %1396
 41461|     ;; count = i64 %1396
 41462|     ;; self[0..+8] = ptr %1395
 41463|     ;; slice[0..+8] = ptr %1395
 41464|     ;; self[8..+8] = i64 %1396
 41465|     ;; slice[8..+8] = i64 %1396
 41466|     ;; ptr = ptr %1395
 41467|     ;; self = ptr %1395
 41468|  %1397 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %1395, i64 %1396 ;L961<240<1062<1930
 41469|     ;; predicate = ptr undef
 41470|     ;; self = ptr undef
 41471|     ;; self = ptr undef
 41472|     ;; count = i64 1
 41473|  br label %1398                                                                                                        ;L348<1930
 41474| 
 41475| 1398: ; preds = %1401, %1394
 41476|  %1399 = phi ptr [ %1402, %1401 ], [ %1395, %1394 ]
 41477|     ;; ptr = ptr %1399
 41478|     ;; self = ptr %1399
 41479|     ;; end_or_len = ptr %1397
 41482|  %1400 = icmp eq ptr %1399, %1397                                                                                      ;L1714<180<348<1930
 41483|  br i1 %1400, label %1230, label %1401                                                                                 ;L180<348<1930
 41484| 
 41485| 1401: ; preds = %1398
 41486|  %1402 = gep %1399, i64 216                                                                                            ;L656<185<348<1930
 41487|     ;; x = ptr %1399
 41490|  %1403 = gep %1399, i64 88                                                                                             ;L1930<349<1930
 41491|  %1404 = load i64, ptr %1403, , !!55919, !!8                                                                           ;L1930<349<1930
 41492|  %1405 = icmp eq i64 %1404, %1199                                                                                      ;L1930<349<1930
 41493|  br i1 %1405, label %1407, label %1398                                                                                 ;L349<1930
 41494| 
 41495| 1406: ; preds = %1256
 41496|     ;; self = ptr null
 41497|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.140) #25
 41498|  to label %173 unwind label %365                                                                                       ;L1013<1928
 41499| 
 41500| 1407: ; preds = %1401
 41501|     ;; target = ptr %1399
 41502|  %1408 = load ptr, ptr %372, , !!8                                                                                     ;L1931
 41503|  %1409 = invoke ptr %1408(ptr %177, i64 %1199)
 41504|  to label %1410 unwind label %365                                                                                      ;L1931
 41505| 
 41506| 1410: ; preds = %1407
 41507|     ;; self = ptr %1409
 41508|  %1411 = icmp eq ptr %1409, null                                                                                       ;L1011<1931
 41509|  br i1 %1411, label %1416, label %1412                                                                                 ;L1011<1931
 41510| 
 41511| 1412: ; preds = %1410
 41512|     ;; target_entity = ptr %1409
 41513|     ;; self = ptr %1261
 41514|  %1413 = load i32, ptr %1262, , !!8                                                                                    ;L149<1932
 41515|  %1414 = add nsw i32 %1413, -1                                                                                         ;L149<1932
 41516|  %1415 = icmp ult i32 %1414, 2                                                                                         ;L149<1932
 41517|  br i1 %1415, label %1421, label %1417                                                                                 ;L149<1932
 41518| 
 41519| 1416: ; preds = %1410
 41520|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.141) #25
 41521|  to label %173 unwind label %365                                                                                       ;L1013<1931
 41522| 
 41523| 1417: ; preds = %1412
 41524|  %1418 = gep %1201, i64 104                                                                                            ;L1586<1932
 41525|  %1419 = load i64, ptr %1418, , !!8                                                                                    ;L1586<1932
 41526|  %1420 = icmp eq i64 %1419, 13                                                                                         ;L1586<1932
 41527|  br i1 %1420, label %1423, label %1421                                                                                 ;L1586<1932
 41528| 
 41529| 1421: ; preds = %1423, %1417, %1412
 41530|  %1422 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1261, ptr %64, ptr %1201, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1409)
 41531|  to label %1433 unwind label %365                                                                                      ;L1933
 41532| 
 41533| 1423: ; preds = %1417
 41534|     ;; champ = ptr %1201
 41535|  %1424 = gep %1201, i64 112                                                                                            ;L1587<1932
 41536|  %1425 = load i64, ptr %1424, , !!8                                                                                    ;L1587<1932
 41537|  %1426 = icmp eq i64 %1425, 6                                                                                          ;L1587<1932
 41538|  br i1 %1426, label %1427, label %1421                                                                                 ;L1932
 41539| 
 41540| 1427: ; preds = %1423
 41541|  %1428 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1261, ptr %64, ptr %1201, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1409)
 41542|  to label %1429 unwind label %365                                                                                      ;L1935
 41543| 
 41544| 1429: ; preds = %1427
 41545|  %1430 = gep %1399, i64 112                                                                                            ;L1935
 41546|  %1431 = load i64, ptr %1430, , !!8                                                                                    ;L1935
 41547|  %1432 = add i64 %1431, %1428                                                                                          ;L1935
 41548|  store i64 %1432, ptr %1430,                                                                                           ;L1935
 41549|  br label %1230                                                                                                        ;L1932
 41550| 
 41551| 1433: ; preds = %1421
 41552|  %1434 = gep %1399, i64 128                                                                                            ;L1933
 41553|  %1435 = load i64, ptr %1434, , !!8                                                                                    ;L1933
 41554|  %1436 = add i64 %1435, %1422                                                                                          ;L1933
 41555|  store i64 %1436, ptr %1434,                                                                                           ;L1933
 41556|  br label %1230                                                                                                        ;L1932
 41557| 
 41558| 1437: ; preds = %1230
 41559|     ;; self = ptr %1233
 41560|     ;; e_atk = ptr %1233
 41561|     ;; atk = ptr %1233
 41562|     ;; self = ptr %1233
 41563|  %1438 = gep %1201, i64 1184                                                                                           ;L26<1947
 41564|  %1439 = load i64, ptr %1438, , !!8                                                                                    ;L26<1947
 41565|  %1440 = gep %1201, i64 1192                                                                                           ;L26<1947
 41566|  %1441 = load i64, ptr %1440, , !!8                                                                                    ;L26<1947
 41567|  %1442 = gep %1201, i64 1480                                                                                           ;L26<1947
 41568|  %1443 = load i64, ptr %1442, , !!8                                                                                    ;L26<1947
 41569|  %1444 = add i64 %1443, -1                                                                                             ;L26<1947
 41570|  %1445 = mul i64 %1444, %1441                                                                                          ;L26<1947
 41571|  %1446 = gep %1201, i64 1080                                                                                           ;L26<1947
 41572|  %1447 = load i64, ptr %1446, , !!8                                                                                    ;L26<1947
 41573|  %1448 = gep %1201, i64 1136                                                                                           ;L1511<1947
 41574|  %1449 = load i32, ptr %1448, , !!8                                                                                    ;L1511<1947
 41575|  %1450 = sext i32 %1449 to i64                                                                                         ;L1511<1947
 41576|     ;; mult = i64 %1450
 41577|     ;; mult = i64 %1450
 41578|     ;; mult = i64 %1450
 41579|  %1451 = icmp eq i32 %1449, 0                                                                                          ;L1512<1947
 41580|  %1452 = gep %1201, i64 1664                                                                                           ;L0<1947
 41581|  %1453 = load i64, ptr %1452, , !!8                                                                                    ;L0<1947
 41582|  br i1 %1451, label %1459, label %1455                                                                                 ;L1512<1947
 41583| 
 41584| 1454: ; preds = %1230
 41585|     ;; self = ptr null
 41586|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.142) #25
 41587|  to label %173 unwind label %365                                                                                       ;L1013<1946
 41588| 
 41589| 1455: ; preds = %1437
 41590|  %1456 = add nsw i64 %1450, 100                                                                                        ;L1515<1947
 41591|  %1457 = mul i64 %1453, %1456                                                                                          ;L1515<1947
 41592|  %1458 = udiv i64 %1457, 100                                                                                           ;L1515<1947
 41593|  br label %1459                                                                                                        ;L1512<1947
 41594| 
 41595| 1459: ; preds = %1455, %1437
 41596|  %1460 = phi i64 [ %1458, %1455 ], [ %1453, %1437 ]                                                                    ;L0<1947
 41598|  %1461 = gep %1201, i64 104                                                                                            ;L1748<1948
 41599|  %1462 = load i64, ptr %1461, , !!8                                                                                    ;L1748<1948
 41600|  switch i64 %1462, label %756 [
 41601|  i64 0, label %1476
 41602|  i64 1, label %1471
 41603|  i64 2, label %1463
 41604|  i64 3, label %1476
 41605|  i64 4, label %1464
 41606|  i64 5, label %1465
 41607|  i64 6, label %1465
 41608|  i64 7, label %1464
 41609|  i64 8, label %1466
 41610|  i64 9, label %1467
 41611|  i64 10, label %1468
 41612|  i64 11, label %1469
 41613|  i64 12, label %1470
 41614|  i64 13, label %1478
 41615|  ]                                                                                                                     ;L1748<1948
 41616| 
 41617| 1463: ; preds = %1459
 41618|     ;; info = ptr %1201
 41619|  br label %1471                                                                                                        ;L1758<1948
 41620| 
 41621| 1464: ; preds = %1459, %1459
 41622|     ;; info = ptr %1201
 41623|  br label %1471                                                                                                        ;L1750<1948
 41624| 
 41625| 1465: ; preds = %1459, %1459
 41626|     ;; info = ptr %1201
 41627|  br label %1471                                                                                                        ;L1760<1948
 41628| 
 41629| 1466: ; preds = %1459
 41630|     ;; info = ptr %1201
 41631|  br label %1471                                                                                                        ;L1753<1948
 41632| 
 41633| 1467: ; preds = %1459
 41634|     ;; info = ptr %1201
 41635|  br label %1471                                                                                                        ;L1754<1948
 41636| 
 41637| 1468: ; preds = %1459
 41638|     ;; info = ptr %1201
 41639|  br label %1471                                                                                                        ;L1755<1948
 41640| 
 41641| 1469: ; preds = %1459
 41642|     ;; info = ptr %1201
 41643|  br label %1471                                                                                                        ;L1756<1948
 41644| 
 41645| 1470: ; preds = %1459
 41646|     ;; info = ptr %1201
 41647|  br label %1471                                                                                                        ;L1757<1948
 41648| 
 41649| 1471: ; preds = %1470, %1469, %1468, %1467, %1466, %1465, %1464, %1463, %1459
 41650|  %1472 = phi i64 [ 208, %1470 ], [ 272, %1463 ], [ 232, %1464 ], [ 496, %1465 ], [ 184, %1459 ], [ 216, %1469 ], [ 176, %1466 ], [ 200, %1467 ], [ 240, %1468 ]
 41651|  %1473 = gep %1201, i64 %1472                                                                                          ;L0<1948
 41652|  %1474 = load i64, ptr %1473, , !!8                                                                                    ;L0<1948
 41653|  %1475 = call i64 @llvm.umax.i64(i64 %1474, i64 %1229)                                                                 ;L1039<1948
 41654|  br label %1476                                                                                                        ;L1039<1948
 41655| 
 41656| 1476: ; preds = %1471, %1459, %1459
 41657|  %1477 = phi i64 [ %1229, %1459 ], [ %1229, %1459 ], [ %1475, %1471 ]
 41658|     ;; self = i64 %1229
 41659|     ;; other = i64 %1477
 41660|     ;; e_attack_tick = i64 %1477
 41661|     ;; attack_tick = i64 %1477
 41662|     ;; self = i64 %1229
 41663|     ;; other = i64 0
 41664|  br label %1488                                                                                                        ;L1039<1949
 41665| 
 41666| 1478: ; preds = %1459
 41667|     ;; champ = ptr %1201
 41668|  %1479 = gep %1201, i64 176                                                                                            ;L1749<1948
 41669|  %1480 = load i64, ptr %1479, , !!8                                                                                    ;L1749<1948
 41670|     ;; self = i64 %1229
 41671|     ;; other = i64 %1480
 41672|  %1481 = call i64 @llvm.umax.i64(i64 %1480, i64 %1229)                                                                 ;L1039<1948
 41673|     ;; e_attack_tick = i64 %1481
 41674|     ;; attack_tick = i64 %1481
 41675|     ;; champ = ptr %1201
 41676|  %1482 = gep %1201, i64 184                                                                                            ;L1776<1949
 41677|  %1483 = load i64, ptr %1482, , !!8                                                                                    ;L1776<1949
 41678|     ;; self = i64 %1229
 41679|     ;; other = i64 %1483
 41680|  %1484 = call i64 @llvm.umax.i64(i64 %1483, i64 %1229)                                                                 ;L1039<1949
 41681|     ;; e_skill_tick = i64 %1484
 41682|     ;; champ = ptr %1201
 41683|  %1485 = gep %1201, i64 192                                                                                            ;L1791<1950
 41684|  %1486 = load i64, ptr %1485, , !!8                                                                                    ;L1791<1950
 41685|  %1487 = call i64 @llvm.umax.i64(i64 %1486, i64 %1229)                                                                 ;L1039<1950
 41686|  br label %1488                                                                                                        ;L1791<1950
 41687| 
 41688| 1488: ; preds = %1478, %1476
 41689|  %1489 = phi i64 [ %1481, %1478 ], [ %1477, %1476 ]
 41690|  %1490 = phi i64 [ %1484, %1478 ], [ %1229, %1476 ]                                                                    ;L1949
 41691|  %1491 = phi i64 [ %1487, %1478 ], [ %1229, %1476 ]                                                                    ;L0<1950
 41692|     ;; e_skill_tick = i64 %1490
 41693|     ;; self = i64 %1229
 41694|     ;; other = i64 %1491
 41695|     ;; e_skill2_tick = i64 %1491
 41696|     ;; skill2_tick = i64 %1491
 41697|     ;; e_skill_eff = ptr %1201
 41698|     ;; self = ptr %1201
 41699|  %1492 = icmp ugt i64 %1443, 2                                                                                         ;L1693<1952
 41700|  %1493 = gep %1201, i64 1280                                                                                           ;L1693<1952
 41701|  %1494 = select i1 %1492, ptr %1493, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                     ;L1693<1952
 41702|     ;; e_skill2_eff = ptr %1494
 41703|     ;; self = ptr %47
 41704|     ;; self = ptr %47
 41705|  %1495 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<1953
 41706|     ;; ptr = ptr %1495
 41707|  %1496 = load i64, ptr %74, , !!8                                                                                      ;L2085<1953
 41708|     ;; len = i64 %1496
 41709|     ;; count = i64 %1496
 41710|     ;; self[0..+8] = ptr %1495
 41711|     ;; slice[0..+8] = ptr %1495
 41712|     ;; self[8..+8] = i64 %1496
 41713|     ;; slice[8..+8] = i64 %1496
 41714|     ;; ptr = ptr %1495
 41715|     ;; self = ptr %1495
 41716|  %1497 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %1495, i64 %1496 ;L961<240<1062<1953
 41717|     ;; iter[0..+8] = ptr %1495
 41718|     ;; iter[8..+8] = ptr %1497
 41719|  %1498 = add i64 %1439, %1447                                                                                          ;L1953
 41720|  %1499 = add i64 %1498, %1445                                                                                          ;L1953
 41721|  %1500 = add i64 %1499, %1460                                                                                          ;L1953
 41722|  %1501 = gep %1201, i64 1632
 41723|  %1502 = gep %1201, i64 1640
 41724|  %1503 = icmp ult i64 %1489, 121
 41725|  %1504 = mul i64 %1232, 20
 41726|  %1505 = add i64 %1504, %1500
 41727|  %1506 = gep %1201, i64 1472
 41728|  %1507 = gep %1201, i64 1224
 41729|  %1508 = gep %1201, i64 1272
 41730|  %1509 = gep %1201, i64 1264
 41731|  %1510 = gep %1201, i64 1240
 41732|  %1511 = gep %1201, i64 1248
 41733|  %1512 = gep %1201, i64 1664
 41734|  %1513 = add nsw i64 %1450, 100
 41735|  %1514 = icmp ult i64 %1490, 121
 41736|  %1515 = add i64 %1447, %1504
 41737|  %1516 = gep %1494, i64 48
 41738|  %1517 = gep %1494, i64 40
 41739|  %1518 = gep %1494, i64 16
 41740|  %1519 = gep %1494, i64 24
 41741|  %1520 = icmp ult i64 %1491, 121
 41742|  br label %1521                                                                                                        ;L1953
 41743| 
 41744| 1521: ; preds = %1551, %1488
 41745|  %1522 = phi ptr [ %1495, %1488 ], [ %1525, %1551 ]                                                                    ;L1953
 41746|     ;; iter[0..+8] = ptr %1522
 41747|     ;; self = ptr undef
 41748|     ;; ptr = ptr %1522
 41749|     ;; self = ptr %1522
 41750|     ;; end_or_len = ptr %1497
 41753|  %1523 = icmp eq ptr %1522, %1497                                                                                      ;L1714<180<1953
 41754|  br i1 %1523, label %1203, label %1524                                                                                 ;L180<1953
 41755| 
 41756| 1524: ; preds = %1521
 41757|  %1525 = gep %1522, i64 216                                                                                            ;L656<185<1953
 41758|     ;; iter[0..+8] = ptr %1525
 41759|     ;; a = ptr %1522
 41760|  %1526 = gep %1522, i64 88                                                                                             ;L1954
 41761|  %1527 = load i64, ptr %1526, , !!8                                                                                    ;L1954
 41762|  %1528 = load ptr, ptr %372, , !!8                                                                                     ;L1954
 41763|  %1529 = invoke ptr %1528(ptr %177, i64 %1527)
 41764|  to label %1530 unwind label %365                                                                                      ;L1954
 41765| 
 41766| 1530: ; preds = %1524
 41767|  %1531 = icmp eq ptr %1529, null                                                                                       ;L1954
 41768|  br i1 %1531, label %1551, label %1532                                                                                 ;L1954
 41769| 
 41770| 1532: ; preds = %1530
 41771|     ;; ae = ptr %1529
 41772|     ;; other = ptr %1529
 41773|     ;; self = ptr %1529
 41774|     ;; self = ptr %1529
 41775|     ;; self = ptr %1529
 41776|  %1533 = load i64, ptr %1501, , !!8                                                                                    ;L2158<1955
 41777|     ;; x1 = i64 %1533
 41778|     ;; self = i64 %1533
 41779|  %1534 = load i64, ptr %1502, , !!8                                                                                    ;L2158<1955
 41780|     ;; y1 = i64 %1534
 41781|     ;; self = i64 %1534
 41782|  %1535 = gep %1529, i64 1632                                                                                           ;L2158<1955
 41783|  %1536 = load i64, ptr %1535, , !!8                                                                                    ;L2158<1955
 41784|     ;; x2 = i64 %1536
 41785|     ;; other = i64 %1536
 41786|  %1537 = gep %1529, i64 1640                                                                                           ;L2158<1955
 41787|  %1538 = load i64, ptr %1537, , !!8                                                                                    ;L2158<1955
 41788|     ;; y2 = i64 %1538
 41789|     ;; other = i64 %1538
 41790|  %1539 = icmp ult i64 %1533, %1536                                                                                     ;L3147<7<2158<1955
 41791|  %1540 = sub nuw i64 %1536, %1533                                                                                      ;L3147<7<2158<1955
 41792|  %1541 = sub nuw i64 %1533, %1536                                                                                      ;L3147<7<2158<1955
 41793|  %1542 = select i1 %1539, i64 %1540, i64 %1541                                                                         ;L3147<7<2158<1955
 41794|     ;; dx = i64 %1542
 41795|  %1543 = icmp ult i64 %1534, %1538                                                                                     ;L3147<8<2158<1955
 41796|  %1544 = sub nuw i64 %1538, %1534                                                                                      ;L3147<8<2158<1955
 41797|  %1545 = sub nuw i64 %1534, %1538                                                                                      ;L3147<8<2158<1955
 41798|  %1546 = select i1 %1543, i64 %1544, i64 %1545                                                                         ;L3147<8<2158<1955
 41799|     ;; dy = i64 %1546
 41800|  %1547 = mul i64 %1542, %1542                                                                                          ;L9<2158<1955
 41801|  %1548 = mul i64 %1546, %1546                                                                                          ;L9<2158<1955
 41802|  %1549 = add i64 %1548, %1547                                                                                          ;L9<2158<1955
 41803|     ;; dist = i64 %1549
 41804|  %1550 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %1233, ptr %1201, ptr %1529)
 41805|  to label %1552 unwind label %365                                                                                      ;L1958
 41806| 
 41807| 1551: ; preds = %1693, %1672, %1670, %1651, %1596, %1530
 41808|  br label %1521                                                                                                        ;L1953
 41809| 
 41810| 1552: ; preds = %1532
 41811|  %1553 = gep %1529, i64 1136                                                                                           ;L1511<1958
 41812|  %1554 = load i32, ptr %1553, , !!8                                                                                    ;L1511<1958
 41813|  %1555 = sext i32 %1554 to i64                                                                                         ;L1511<1958
 41814|     ;; mult = i64 %1555
 41815|     ;; mult = i64 %1555
 41816|     ;; mult = i64 %1555
 41817|  %1556 = icmp eq i32 %1554, 0                                                                                          ;L1512<1958
 41818|  %1557 = gep %1529, i64 1664                                                                                           ;L0<1958
 41819|  %1558 = load i64, ptr %1557, , !!8                                                                                    ;L0<1958
 41820|  br i1 %1556, label %1563, label %1559                                                                                 ;L1512<1958
 41821| 
 41822| 1559: ; preds = %1552
 41823|  %1560 = add nsw i64 %1555, 100                                                                                        ;L1515<1958
 41824|  %1561 = mul i64 %1558, %1560                                                                                          ;L1515<1958
 41825|  %1562 = udiv i64 %1561, 100                                                                                           ;L1515<1958
 41826|  br label %1563                                                                                                        ;L1512<1958
 41827| 
 41828| 1563: ; preds = %1559, %1552
 41829|  %1564 = phi i64 [ %1562, %1559 ], [ %1558, %1552 ]                                                                    ;L0<1958
 41831|  br i1 %1503, label %1568, label %1565                                                                                 ;L1962
 41832| 
 41833| 1565: ; preds = %1586, %1568, %1563
 41834|     ;; skill_tick = i64 %1490
 41835|  %1566 = load i32, ptr %1508, , !!8                                                                                    ;L742<1972
 41836|  %1567 = icmp eq i32 %1566, -1                                                                                         ;L742<1972
 41837|  br i1 %1567, label %1596, label %1594                                                                                 ;L742<1972
 41838| 
 41839| 1568: ; preds = %1563
 41841|  %1569 = add i64 %1505, %1550                                                                                          ;L1958
 41842|  %1570 = add i64 %1569, %1564                                                                                          ;L1962
 41843|  %1571 = mul i64 %1570, %1570                                                                                          ;L1962
 41844|  %1572 = icmp ult i64 %1571, %1549                                                                                     ;L1962
 41845|  br i1 %1572, label %1565, label %1573                                                                                 ;L1962
 41846| 
 41847| 1573: ; preds = %1568
 41848|  %1574 = load i64, ptr %1506, , !!8                                                                                    ;L1964
 41849|  %1575 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1233, ptr %64, ptr %1201, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1529)
 41850|  to label %1576 unwind label %365                                                                                      ;L1965
 41851| 
 41852| 1576: ; preds = %1573
 41853|  %1577 = gep %1522, i64 24                                                                                             ;L1963
 41854|     ;; value[0..+8] = i64 %1574
 41855|     ;; src[0..+8] = i64 %1574
 41856|     ;; value[8..+8] = i64 %1489
 41857|     ;; src[8..+8] = i64 %1489
 41858|     ;; value[16..+8] = i64 %1575
 41859|     ;; src[16..+8] = i64 %1575
 41860|     ;; self = ptr %1577
 41861|     ;; self = ptr %1577
 41862|     ;; additional = i64 1
 41863|     ;; needed_extra_cap = i64 1
 41864|     ;; needed_extra_cap = i64 1
 41865|     ;; strategy = i8 1
 41866|  %1578 = gep %1522, i64 48                                                                                             ;L1428<1963
 41867|  %1579 = load i64, ptr %1578, , !!56067, !!8                                                                           ;L1428<1963
 41868|     ;; self = ptr %1577
 41869|  %1580 = gep %1522, i64 40                                                                                             ;L149<1428<1963
 41870|  %1581 = load i64, ptr %1580, , !!56067, !!8                                                                           ;L149<1428<1963
 41871|  %1582 = icmp eq i64 %1579, %1581                                                                                      ;L1428<1963
 41872|  br i1 %1582, label %1583, label %1586                                                                                 ;L1428<1963
 41873| 
 41874| 1583: ; preds = %1576
 41875|     ;; self = ptr %1577
 41876|     ;; self = ptr %1577
 41877|     ;; self = ptr %1577
 41878|     ;; used_cap = i64 %1579
 41879|     ;; used_cap = i64 %1579
 41880|  invoke void @ai::score_parameter12PossibleGainE25reserve_internal_or_panicB17_(ptr %1577, i64 %1579, i64 1, i1 zeroext true)
 41881|  to label %1584 unwind label %365                                                                                      ;L619<430<738<1429<1963
 41882| 
 41883| 1584: ; preds = %1583
 41884|  %1585 = load i64, ptr %1578, , !!56067                                                                                ;L1432<1963
 41885|  br label %1586                                                                                                        ;L1428<1963
 41886| 
 41887| 1586: ; preds = %1584, %1576
 41888|  %1587 = phi i64 [ %1579, %1576 ], [ %1585, %1584 ]                                                                    ;L1432<1963
 41889|     ;; self = ptr %1577
 41890|  %1588 = load ptr, ptr %1577, , !!56067, !!8, !!8                                                                      ;L138<1432<1963
 41891|     ;; self = ptr %1588
 41892|     ;; count = i64 %1587
 41893|  %1589 = gepS %1588, i64 %1587                                                                                         ;L961<1432<1963
 41894|     ;; end = ptr %1589
 41895|     ;; dst = ptr %1589
 41896|  store i64 %1574, ptr %1589,                                                                                           ;L1933<1433<1963
 41897|  %1590 = gep %1589, i64 8                                                                                              ;L1933<1433<1963
 41898|  store i64 %1489, ptr %1590,                                                                                           ;L1933<1433<1963
 41899|  %1591 = gep %1589, i64 16                                                                                             ;L1933<1433<1963
 41900|  store i64 %1575, ptr %1591,                                                                                           ;L1933<1433<1963
 41901|  %1592 = load i64, ptr %1578, , !!56067, !!8                                                                           ;L1434<1963
 41902|  %1593 = add i64 %1592, 1                                                                                              ;L1434<1963
 41903|  store i64 %1593, ptr %1578, , !!56067                                                                                 ;L1434<1963
 41904|  br label %1565                                                                                                        ;L1962
 41905| 
 41906| 1594: ; preds = %1565
 41907|     ;; skill = ptr %1507
 41908|     ;; self = ptr %1507
 41909|  %1595 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %1509, ptr %1201, ptr %1529)
 41910|  to label %1599 unwind label %365                                                                                      ;L1973
 41911| 
 41912| 1596: ; preds = %1641, %1620, %1618, %1599, %1565
 41913|     ;; self = ptr %1494
 41914|  %1597 = load i32, ptr %1516, , !!8                                                                                    ;L742<1987
 41915|  %1598 = icmp eq i32 %1597, -1                                                                                         ;L742<1987
 41916|  br i1 %1598, label %1551, label %1649                                                                                 ;L742<1987
 41917| 
 41918| 1599: ; preds = %1594
 41919|  br i1 %1595, label %1600, label %1596                                                                                 ;L1973
 41920| 
 41921| 1600: ; preds = %1599
 41922|  %1601 = load i64, ptr %1510, , !!8                                                                                    ;L26<1974
 41923|  %1602 = load i64, ptr %1511, , !!8                                                                                    ;L26<1974
 41924|  %1603 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %1507, ptr %1201, ptr %1529)
 41925|  to label %1604 unwind label %365                                                                                      ;L1974
 41926| 
 41927| 1604: ; preds = %1600
 41928|  %1605 = mul i64 %1602, %1444                                                                                          ;L26<1974
 41929|  %1606 = load i64, ptr %1512, , !!8                                                                                    ;L0<1974
 41930|  br i1 %1451, label %1610, label %1607                                                                                 ;L1512<1974
 41931| 
 41932| 1607: ; preds = %1604
 41933|  %1608 = mul i64 %1606, %1513                                                                                          ;L1515<1974
 41934|  %1609 = udiv i64 %1608, 100                                                                                           ;L1515<1974
 41935|  br label %1610                                                                                                        ;L1512<1974
 41936| 
 41937| 1610: ; preds = %1607, %1604
 41938|  %1611 = phi i64 [ %1609, %1607 ], [ %1606, %1604 ]                                                                    ;L0<1974
 41939|  %1612 = gep %1529, i64 1664                                                                                           ;L0<1974
 41940|  %1613 = load i64, ptr %1612, , !!8                                                                                    ;L0<1974
 41941|  br i1 %1556, label %1618, label %1614                                                                                 ;L1512<1974
 41942| 
 41943| 1614: ; preds = %1610
 41944|  %1615 = add nsw i64 %1555, 100                                                                                        ;L1515<1974
 41945|  %1616 = mul i64 %1613, %1615                                                                                          ;L1515<1974
 41946|  %1617 = udiv i64 %1616, 100                                                                                           ;L1515<1974
 41947|  br label %1618                                                                                                        ;L1512<1974
 41948| 
 41949| 1618: ; preds = %1614, %1610
 41950|  %1619 = phi i64 [ %1617, %1614 ], [ %1613, %1610 ]                                                                    ;L0<1974
 41952|  br i1 %1514, label %1620, label %1596                                                                                 ;L1975
 41953| 
 41954| 1620: ; preds = %1618
 41956|  %1621 = add i64 %1515, %1601                                                                                          ;L26<1974
 41957|  %1622 = add i64 %1621, %1605                                                                                          ;L1974
 41958|  %1623 = add i64 %1622, %1603                                                                                          ;L1974
 41959|  %1624 = add i64 %1623, %1611                                                                                          ;L1974
 41960|  %1625 = add i64 %1624, %1619                                                                                          ;L1975
 41961|  %1626 = mul i64 %1625, %1625                                                                                          ;L1975
 41962|  %1627 = icmp ult i64 %1626, %1549                                                                                     ;L1975
 41963|  br i1 %1627, label %1596, label %1628                                                                                 ;L1975
 41964| 
 41965| 1628: ; preds = %1620
 41966|  %1629 = load i64, ptr %1506, , !!8                                                                                    ;L1977
 41967|  %1630 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1507, ptr %64, ptr %1201, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1529)
 41968|  to label %1631 unwind label %365                                                                                      ;L1978
 41969| 
 41970| 1631: ; preds = %1628
 41971|  %1632 = gep %1522, i64 24                                                                                             ;L1976
 41972|     ;; value[0..+8] = i64 %1629
 41973|     ;; src[0..+8] = i64 %1629
 41974|     ;; value[8..+8] = i64 %1490
 41975|     ;; src[8..+8] = i64 %1490
 41976|     ;; value[16..+8] = i64 %1630
 41977|     ;; src[16..+8] = i64 %1630
 41978|     ;; self = ptr %1632
 41979|     ;; self = ptr %1632
 41980|     ;; additional = i64 1
 41981|     ;; needed_extra_cap = i64 1
 41982|     ;; needed_extra_cap = i64 1
 41983|     ;; strategy = i8 1
 41984|  %1633 = gep %1522, i64 48                                                                                             ;L1428<1976
 41985|  %1634 = load i64, ptr %1633, , !!56113, !!8                                                                           ;L1428<1976
 41986|     ;; self = ptr %1632
 41987|  %1635 = gep %1522, i64 40                                                                                             ;L149<1428<1976
 41988|  %1636 = load i64, ptr %1635, , !!56113, !!8                                                                           ;L149<1428<1976
 41989|  %1637 = icmp eq i64 %1634, %1636                                                                                      ;L1428<1976
 41990|  br i1 %1637, label %1638, label %1641                                                                                 ;L1428<1976
 41991| 
 41992| 1638: ; preds = %1631
 41993|     ;; self = ptr %1632
 41994|     ;; self = ptr %1632
 41995|     ;; self = ptr %1632
 41996|     ;; used_cap = i64 %1634
 41997|     ;; used_cap = i64 %1634
 41998|  invoke void @ai::score_parameter12PossibleGainE25reserve_internal_or_panicB17_(ptr %1632, i64 %1634, i64 1, i1 zeroext true)
 41999|  to label %1639 unwind label %365                                                                                      ;L619<430<738<1429<1976
 42000| 
 42001| 1639: ; preds = %1638
 42002|  %1640 = load i64, ptr %1633, , !!56113                                                                                ;L1432<1976
 42003|  br label %1641                                                                                                        ;L1428<1976
 42004| 
 42005| 1641: ; preds = %1639, %1631
 42006|  %1642 = phi i64 [ %1634, %1631 ], [ %1640, %1639 ]                                                                    ;L1432<1976
 42007|     ;; self = ptr %1632
 42008|  %1643 = load ptr, ptr %1632, , !!56113, !!8, !!8                                                                      ;L138<1432<1976
 42009|     ;; self = ptr %1643
 42010|     ;; count = i64 %1642
 42011|  %1644 = gepS %1643, i64 %1642                                                                                         ;L961<1432<1976
 42012|     ;; end = ptr %1644
 42013|     ;; dst = ptr %1644
 42014|  store i64 %1629, ptr %1644,                                                                                           ;L1933<1433<1976
 42015|  %1645 = gep %1644, i64 8                                                                                              ;L1933<1433<1976
 42016|  store i64 %1490, ptr %1645,                                                                                           ;L1933<1433<1976
 42017|  %1646 = gep %1644, i64 16                                                                                             ;L1933<1433<1976
 42018|  store i64 %1630, ptr %1646,                                                                                           ;L1933<1433<1976
 42019|  %1647 = load i64, ptr %1633, , !!56113, !!8                                                                           ;L1434<1976
 42020|  %1648 = add i64 %1647, 1                                                                                              ;L1434<1976
 42021|  store i64 %1648, ptr %1633, , !!56113                                                                                 ;L1434<1976
 42022|  br label %1596                                                                                                        ;L1975
 42023| 
 42024| 1649: ; preds = %1596
 42025|     ;; skill2 = ptr %1494
 42026|     ;; self = ptr %1494
 42027|  %1650 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %1517, ptr %1201, ptr %1529)
 42028|  to label %1651 unwind label %365                                                                                      ;L1988
 42029| 
 42030| 1651: ; preds = %1649
 42031|  br i1 %1650, label %1652, label %1551                                                                                 ;L1988
 42032| 
 42033| 1652: ; preds = %1651
 42034|  %1653 = load i64, ptr %1518, , !!8                                                                                    ;L26<1989
 42035|  %1654 = load i64, ptr %1519, , !!8                                                                                    ;L26<1989
 42036|  %1655 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %1494, ptr %1201, ptr %1529)
 42037|  to label %1656 unwind label %365                                                                                      ;L1989
 42038| 
 42039| 1656: ; preds = %1652
 42040|  %1657 = mul i64 %1654, %1444                                                                                          ;L26<1989
 42041|  %1658 = load i64, ptr %1512, , !!8                                                                                    ;L0<1989
 42042|  br i1 %1451, label %1662, label %1659                                                                                 ;L1512<1989
 42043| 
 42044| 1659: ; preds = %1656
 42045|  %1660 = mul i64 %1658, %1513                                                                                          ;L1515<1989
 42046|  %1661 = udiv i64 %1660, 100                                                                                           ;L1515<1989
 42047|  br label %1662                                                                                                        ;L1512<1989
 42048| 
 42049| 1662: ; preds = %1659, %1656
 42050|  %1663 = phi i64 [ %1661, %1659 ], [ %1658, %1656 ]                                                                    ;L0<1989
 42051|  %1664 = gep %1529, i64 1664                                                                                           ;L0<1989
 42052|  %1665 = load i64, ptr %1664, , !!8                                                                                    ;L0<1989
 42053|  br i1 %1556, label %1670, label %1666                                                                                 ;L1512<1989
 42054| 
 42055| 1666: ; preds = %1662
 42056|  %1667 = add nsw i64 %1555, 100                                                                                        ;L1515<1989
 42057|  %1668 = mul i64 %1665, %1667                                                                                          ;L1515<1989
 42058|  %1669 = udiv i64 %1668, 100                                                                                           ;L1515<1989
 42059|  br label %1670                                                                                                        ;L1512<1989
 42060| 
 42061| 1670: ; preds = %1666, %1662
 42062|  %1671 = phi i64 [ %1669, %1666 ], [ %1665, %1662 ]                                                                    ;L0<1989
 42064|  br i1 %1520, label %1672, label %1551                                                                                 ;L1990
 42065| 
 42066| 1672: ; preds = %1670
 42068|  %1673 = add i64 %1515, %1653                                                                                          ;L26<1989
 42069|  %1674 = add i64 %1673, %1657                                                                                          ;L1989
 42070|  %1675 = add i64 %1674, %1655                                                                                          ;L1989
 42071|  %1676 = add i64 %1675, %1663                                                                                          ;L1989
 42072|  %1677 = add i64 %1676, %1671                                                                                          ;L1990
 42073|  %1678 = mul i64 %1677, %1677                                                                                          ;L1990
 42074|  %1679 = icmp ult i64 %1678, %1549                                                                                     ;L1990
 42075|  br i1 %1679, label %1551, label %1680                                                                                 ;L1990
 42076| 
 42077| 1680: ; preds = %1672
 42078|  %1681 = load i64, ptr %1506, , !!8                                                                                    ;L1992
 42079|  %1682 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1494, ptr %64, ptr %1201, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1529)
 42080|  to label %1683 unwind label %365                                                                                      ;L1993
 42081| 
 42082| 1683: ; preds = %1680
 42083|  %1684 = gep %1522, i64 24                                                                                             ;L1991
 42084|     ;; value[0..+8] = i64 %1681
 42085|     ;; src[0..+8] = i64 %1681
 42086|     ;; value[8..+8] = i64 %1491
 42087|     ;; src[8..+8] = i64 %1491
 42088|     ;; value[16..+8] = i64 %1682
 42089|     ;; src[16..+8] = i64 %1682
 42090|     ;; self = ptr %1684
 42091|     ;; self = ptr %1684
 42092|     ;; additional = i64 1
 42093|     ;; needed_extra_cap = i64 1
 42094|     ;; needed_extra_cap = i64 1
 42095|     ;; strategy = i8 1
 42096|  %1685 = gep %1522, i64 48                                                                                             ;L1428<1991
 42097|  %1686 = load i64, ptr %1685, , !!56156, !!8                                                                           ;L1428<1991
 42098|     ;; self = ptr %1684
 42099|  %1687 = gep %1522, i64 40                                                                                             ;L149<1428<1991
 42100|  %1688 = load i64, ptr %1687, , !!56156, !!8                                                                           ;L149<1428<1991
 42101|  %1689 = icmp eq i64 %1686, %1688                                                                                      ;L1428<1991
 42102|  br i1 %1689, label %1690, label %1693                                                                                 ;L1428<1991
 42103| 
 42104| 1690: ; preds = %1683
 42105|     ;; self = ptr %1684
 42106|     ;; self = ptr %1684
 42107|     ;; self = ptr %1684
 42108|     ;; used_cap = i64 %1686
 42109|     ;; used_cap = i64 %1686
 42110|  invoke void @ai::score_parameter12PossibleGainE25reserve_internal_or_panicB17_(ptr %1684, i64 %1686, i64 1, i1 zeroext true)
 42111|  to label %1691 unwind label %365                                                                                      ;L619<430<738<1429<1991
 42112| 
 42113| 1691: ; preds = %1690
 42114|  %1692 = load i64, ptr %1685, , !!56156                                                                                ;L1432<1991
 42115|  br label %1693                                                                                                        ;L1428<1991
 42116| 
 42117| 1693: ; preds = %1691, %1683
 42118|  %1694 = phi i64 [ %1686, %1683 ], [ %1692, %1691 ]                                                                    ;L1432<1991
 42119|     ;; self = ptr %1684
 42120|  %1695 = load ptr, ptr %1684, , !!56156, !!8, !!8                                                                      ;L138<1432<1991
 42121|     ;; self = ptr %1695
 42122|     ;; count = i64 %1694
 42123|  %1696 = gepS %1695, i64 %1694                                                                                         ;L961<1432<1991
 42124|     ;; end = ptr %1696
 42125|     ;; dst = ptr %1696
 42126|  store i64 %1681, ptr %1696,                                                                                           ;L1933<1433<1991
 42127|  %1697 = gep %1696, i64 8                                                                                              ;L1933<1433<1991
 42128|  store i64 %1491, ptr %1697,                                                                                           ;L1933<1433<1991
 42129|  %1698 = gep %1696, i64 16                                                                                             ;L1933<1433<1991
 42130|  store i64 %1682, ptr %1698,                                                                                           ;L1933<1433<1991
 42131|  %1699 = load i64, ptr %1685, , !!56156, !!8                                                                           ;L1434<1991
 42132|  %1700 = add i64 %1699, 1                                                                                              ;L1434<1991
 42133|  store i64 %1700, ptr %1685, , !!56156                                                                                 ;L1434<1991
 42134|  br label %1551                                                                                                        ;L1990
 42135| 
 42136| 1701: ; preds = %1192
 42137|     ;; self = ptr %75
 42138|  %1702 = load i64, ptr %75, , !!8                                                                                      ;L264<2004
 42141|     ;; __self_discr = i64 %1702
 42142|     ;; __arg1_discr = i64 0
 42143|  %1703 = icmp eq i64 %1702, 0                                                                                          ;L81<264<2004
 42144|  br i1 %1703, label %1706, label %1704                                                                                 ;L2004
 42145| 
 42146| 1704: ; preds = %1701
 42147|  %1705 = invoke i64 @gc::simulation6entityNtB5_6Entity18remain_action_time(ptr %59)
 42148|  to label %1711 unwind label %365                                                                                      ;L2005
 42149| 
 42150| 1706: ; preds = %1968, %1701
 42151|     ;; self = ptr %64
 42152|  %1707 = gep %64, i64 56                                                                                               ;L295<2111
 42153|  %1708 = load i8, ptr %1707, , !!8                                                                                     ;L295<2111
 42154|  %1709 = gep %64, i64 8                                                                                                ;L0
 42155|  %1710 = load ptr, ptr %1709, , !!8, !!8                                                                               ;L0
 42159|  switch i8 %1708, label %756 [
 42160|  i8 0, label %2226
 42161|  i8 1, label %2227
 42162|  i8 2, label %2226
 42163|  i8 3, label %2227
 42164|  i8 4, label %2226
 42165|  i8 5, label %2225
 42166|  i8 6, label %2226
 42167|  i8 7, label %2226
 42168|  i8 8, label %2226
 42169|  ]                                                                                                                     ;L295<2111
 42170| 
 42171| 1711: ; preds = %1704
 42172|     ;; self = ptr %59
 42173|     ;; self = ptr %59
 42174|     ;; self = ptr %59
 42175|  %1712 = gep %59, i64 712                                                                                              ;L614<609<296<1968<1864<3787<2005
 42176|  %1713 = load ptr, ptr %1712, , !!8, !!8                                                                               ;L614<609<296<1968<1864<3787<2005
 42177|  %1714 = gep %59, i64 720                                                                                              ;L1864<3787<2005
 42178|  %1715 = load i64, ptr %1714, , !!8                                                                                    ;L1864<3787<2005
 42179|     ;; len = i64 %1715
 42180|     ;; count = i64 %1715
 42181|     ;; self[0..+8] = ptr %1713
 42182|     ;; slice[0..+8] = ptr %1713
 42183|     ;; self[8..+8] = i64 %1715
 42184|     ;; slice[8..+8] = i64 %1715
 42185|     ;; ptr = ptr %1713
 42186|     ;; self = ptr %1713
 42187|  %1716 = gepS %1713, i64 %1715                                                                                         ;L961<100<1042<2005
 42188|     ;; self[0..+8] = ptr %1713
 42189|     ;; self[0..+8] = ptr %1713
 42190|     ;; self[8..+8] = ptr %1716
 42191|     ;; self[8..+8] = ptr %1716
 42192|     ;; self[0..+8] = ptr %1713
 42193|     ;; self[8..+8] = ptr %1716
 42195|     ;; self = ptr undef
 42196|     ;; self = ptr undef
 42197|     ;; predicate = ptr undef
 42198|     ;; self = ptr undef
 42199|     ;; self = ptr undef
 42200|     ;; count = i64 1
 42201|  br label %1717                                                                                                        ;L348<98<107<2706<3354<3255<2006
 42202| 
 42203| 1717: ; preds = %1720, %1711
 42204|  %1718 = phi ptr [ %1721, %1720 ], [ %1713, %1711 ]
 42206|     ;; ptr = ptr %1718
 42207|     ;; self = ptr %1718
 42208|     ;; end_or_len = ptr %1716
 42211|  %1719 = icmp eq ptr %1718, %1716                                                                                      ;L1714<180<348<98<107<2706<3354<3255<2006
 42212|  br i1 %1719, label %1731, label %1720                                                                                 ;L180<348<98<107<2706<3354<3255<2006
 42213| 
 42214| 1720: ; preds = %1717
 42215|  %1721 = gep %1718, i64 40                                                                                             ;L656<185<348<98<107<2706<3354<3255<2006
 42216|     ;; self[0..+8] = ptr %1721
 42217|     ;; x = ptr %1718
 42222|     ;; self = ptr %1718
 42223|  %1722 = load i32, ptr %1718, , !!56316, !!8                                                                           ;L521<2005<298<349<98<107<2706<3354<3255<2006
 42224|  %1723 = add nsw i32 %1722, -6                                                                                         ;L521<2005<298<349<98<107<2706<3354<3255<2006
 42225|  %1724 = icmp ult i32 %1723, -4                                                                                        ;L521<2005<298<349<98<107<2706<3354<3255<2006
 42226|  br i1 %1724, label %1725, label %1717                                                                                 ;L349<98<107<2706<3354<3255<2006
 42227| 
 42228| 1725: ; preds = %1720
 42229|     ;; self = ptr %1718
 42230|     ;; f = ptr undef
 42231|     ;; self = ptr undef
 42232|     ;; x = ptr %1718
 42233|     ;; args = ptr %1718
 42235|     ;; c = ptr %1718
 42236|     ;; self = ptr %1718
 42237|  %1726 = icmp eq i32 %1722, 10                                                                                         ;L546<2006<310<1162<107<2706<3354<3255<2006
 42238|  %1727 = select i1 %1726, i64 32, i64 8                                                                                ;L0<2006<310<1162<107<2706<3354<3255<2006
 42239|  %1728 = gep %1718, i64 %1727                                                                                          ;L0<2006<310<1162<107<2706<3354<3255<2006
 42240|  %1729 = load i64, ptr %1728, , !!56385, !!8                                                                           ;L0<2006<310<1162<107<2706<3354<3255<2006
 42241|     ;; self[0..+8] = ptr %1721
 42242|     ;; first = i64 %1729
 42243|  %1730 = invoke i64 @core::iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtNtBc_5slice4iter4IterNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity7CCStateENCNvNtCshdEBA0ozCnw_7game_ai15score_parameter25calculate_score_parametersp_0ENCB2H_sq_0ENtNtNtBa_6traits8iterator8Iterator4foldjNCINvNvB45_6max_by4foldjNvYjNtNtBc_3cmp3Ord3cmpE0EB2L_(ptr %1721, ptr %1716, i64 %1729)
 42244|  to label %1731 unwind label %365                                                                                      ;L2707<3354<3255<2006
 42245| 
 42246| 1731: ; preds = %1725, %1717
 42247|  %1732 = phi i64 [ %1730, %1725 ], [ undef, %1717 ]
 42250|     ;; self = i64 %1705
 42252|  %1733 = call i64 @llvm.umax.i64(i64 %1732, i64 %1705)                                                                 ;L1039<2005
 42253|  %1734 = select i1 %1719, i64 %1705, i64 %1733                                                                         ;L1039<2006
 42254|     ;; act_tick = i64 %1734
 42255|  %1735 = load i64, ptr %75, , !!8                                                                                      ;L2008
 42256|  switch i64 %1735, label %1736 [
 42257|  i64 6, label %1764
 42258|  i64 7, label %1770
 42259|  i64 8, label %1776
 42260|  i64 9, label %1786
 42261|  ]                                                                                                                     ;L2008
 42262| 
 42263| 1736: ; preds = %1964, %1960, %1929, %1921, %1917, %1886, %1878, %1874, %1843, %1835, %1831, %1800, %1731
 42264|     ;; self = ptr %47
 42265|     ;; self = ptr %47
 42266|  %1737 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<2060
 42267|     ;; ptr = ptr %1737
 42268|  %1738 = load i64, ptr %74, , !!8                                                                                      ;L2085<2060
 42269|     ;; len = i64 %1738
 42270|     ;; count = i64 %1738
 42271|     ;; self[0..+8] = ptr %1737
 42272|     ;; slice[0..+8] = ptr %1737
 42273|     ;; self[8..+8] = i64 %1738
 42274|     ;; slice[8..+8] = i64 %1738
 42275|     ;; ptr = ptr %1737
 42276|     ;; self = ptr %1737
 42277|  %1739 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %1737, i64 %1738 ;L961<240<1062<2060
 42278|     ;; iter[0..+8] = ptr %1737
 42279|     ;; iter[8..+8] = ptr %1739
 42280|  %1740 = gep %59, i64 1600
 42281|  %1741 = gep %59, i64 1168
 42282|  %1742 = gep %59, i64 1216
 42283|  %1743 = gep %59, i64 1184
 42284|  %1744 = gep %59, i64 1192
 42285|  %1745 = gep %59, i64 1480
 42286|  %1746 = gep %59, i64 1080
 42287|  %1747 = gep %59, i64 104
 42288|  %1748 = gep %59, i64 176
 42289|  %1749 = gep %59, i64 208
 42290|  %1750 = gep %59, i64 216
 42291|  %1751 = gep %59, i64 240
 42292|  %1752 = gep %59, i64 200
 42293|  %1753 = gep %59, i64 232
 42294|  %1754 = gep %59, i64 496
 42295|  %1755 = gep %59, i64 272
 42296|  %1756 = gep %59, i64 184
 42297|  %1757 = gep %59, i64 1224
 42298|  %1758 = gep %59, i64 1272
 42299|  %1759 = gep %59, i64 1264
 42300|  %1760 = gep %59, i64 1240
 42301|  %1761 = gep %59, i64 1248
 42302|  %1762 = gep %59, i64 192
 42303|  %1763 = gep %59, i64 1280
 42304|  br label %1968                                                                                                        ;L2060
 42305| 
 42306| 1764: ; preds = %1731
 42307|  %1765 = load i64, ptr %76, , !!8                                                                                      ;L2009
 42308|     ;; target_id = i64 %1765
 42309|     ;; self = ptr %59
 42310|  %1766 = gep %59, i64 1168                                                                                             ;L742<2010
 42311|  %1767 = gep %59, i64 1216                                                                                             ;L742<2010
 42312|  %1768 = load i32, ptr %1767, , !!8                                                                                    ;L742<2010
 42313|  %1769 = icmp eq i32 %1768, -1                                                                                         ;L742<2010
 42314|  br i1 %1769, label %1808, label %1796                                                                                 ;L742<2010
 42315| 
 42316| 1770: ; preds = %1731
 42317|  %1771 = load i64, ptr %76, , !!8                                                                                      ;L2020
 42318|     ;; target_id = i64 %1771
 42319|     ;; self = ptr %59
 42320|  %1772 = gep %59, i64 1224                                                                                             ;L742<2021
 42321|  %1773 = gep %59, i64 1272                                                                                             ;L742<2021
 42322|  %1774 = load i32, ptr %1773, , !!8                                                                                    ;L742<2021
 42323|  %1775 = icmp eq i32 %1774, -1                                                                                         ;L742<2021
 42324|  br i1 %1775, label %1851, label %1839                                                                                 ;L742<2021
 42325| 
 42326| 1776: ; preds = %1731
 42327|  %1777 = load i64, ptr %76, , !!8                                                                                      ;L2031
 42328|     ;; target_id = i64 %1777
 42329|  %1778 = gep %59, i64 1480                                                                                             ;L1693<2032
 42330|  %1779 = load i64, ptr %1778, , !!8                                                                                    ;L1693<2032
 42331|  %1780 = icmp ugt i64 %1779, 2                                                                                         ;L1693<2032
 42332|  %1781 = gep %59, i64 1280                                                                                             ;L1693<2032
 42333|  %1782 = select i1 %1780, ptr %1781, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                     ;L1693<2032
 42334|     ;; self = ptr %1782
 42335|  %1783 = gep %1782, i64 48                                                                                             ;L742<2032
 42336|  %1784 = load i32, ptr %1783, , !!8                                                                                    ;L742<2032
 42337|  %1785 = icmp eq i32 %1784, -1                                                                                         ;L742<2032
 42338|  br i1 %1785, label %1894, label %1882                                                                                 ;L742<2032
 42339| 
 42340| 1786: ; preds = %1731
 42341|  %1787 = load i64, ptr %76, , !!8                                                                                      ;L2044
 42342|     ;; target_id = i64 %1787
 42343|  %1788 = gep %59, i64 1480                                                                                             ;L1701<2045
 42344|  %1789 = load i64, ptr %1788, , !!8                                                                                    ;L1701<2045
 42345|  %1790 = icmp ugt i64 %1789, 4                                                                                         ;L1701<2045
 42346|  %1791 = gep %59, i64 1336                                                                                             ;L1701<2045
 42347|  %1792 = select i1 %1790, ptr %1791, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                     ;L1701<2045
 42348|     ;; self = ptr %1792
 42349|  %1793 = gep %1792, i64 48                                                                                             ;L742<2045
 42350|  %1794 = load i32, ptr %1793, , !!8                                                                                    ;L742<2045
 42351|  %1795 = icmp eq i32 %1794, -1                                                                                         ;L742<2045
 42352|  br i1 %1795, label %1937, label %1925                                                                                 ;L742<2045
 42353| 
 42354| 1796: ; preds = %1764
 42355|     ;; self = ptr %1766
 42356|     ;; atk = ptr %1766
 42357|     ;; self = ptr %47
 42358|     ;; self = ptr %47
 42359|  %1797 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<2011
 42360|     ;; ptr = ptr %1797
 42361|  %1798 = load i64, ptr %74, , !!8                                                                                      ;L2085<2011
 42362|     ;; len = i64 %1798
 42363|     ;; count = i64 %1798
 42364|     ;; self[0..+8] = ptr %1797
 42365|     ;; slice[0..+8] = ptr %1797
 42366|     ;; self[8..+8] = i64 %1798
 42367|     ;; slice[8..+8] = i64 %1798
 42368|     ;; ptr = ptr %1797
 42369|     ;; self = ptr %1797
 42370|  %1799 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %1797, i64 %1798 ;L961<240<1062<2011
 42371|     ;; predicate = ptr undef
 42372|     ;; self = ptr undef
 42373|     ;; self = ptr undef
 42374|     ;; count = i64 1
 42375|  br label %1800                                                                                                        ;L348<2011
 42376| 
 42377| 1800: ; preds = %1803, %1796
 42378|  %1801 = phi ptr [ %1804, %1803 ], [ %1797, %1796 ]
 42379|     ;; ptr = ptr %1801
 42380|     ;; self = ptr %1801
 42381|     ;; end_or_len = ptr %1799
 42384|  %1802 = icmp eq ptr %1801, %1799                                                                                      ;L1714<180<348<2011
 42385|  br i1 %1802, label %1736, label %1803                                                                                 ;L180<348<2011
 42386| 
 42387| 1803: ; preds = %1800
 42388|  %1804 = gep %1801, i64 216                                                                                            ;L656<185<348<2011
 42389|     ;; x = ptr %1801
 42392|  %1805 = gep %1801, i64 88                                                                                             ;L2011<349<2011
 42393|  %1806 = load i64, ptr %1805, , !!56483, !!8                                                                           ;L2011<349<2011
 42394|  %1807 = icmp eq i64 %1806, %1765                                                                                      ;L2011<349<2011
 42395|  br i1 %1807, label %1809, label %1800                                                                                 ;L349<2011
 42396| 
 42397| 1808: ; preds = %1764
 42398|     ;; self = ptr null
 42399|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.143) #25
 42400|  to label %173 unwind label %365                                                                                       ;L1013<2010
 42401| 
 42402| 1809: ; preds = %1803
 42403|     ;; target = ptr %1801
 42404|  %1810 = load ptr, ptr %372, , !!8                                                                                     ;L2012
 42405|  %1811 = invoke ptr %1810(ptr %177, i64 %1765)
 42406|  to label %1812 unwind label %365                                                                                      ;L2012
 42407| 
 42408| 1812: ; preds = %1809
 42409|     ;; self = ptr %1811
 42410|  %1813 = icmp eq ptr %1811, null                                                                                       ;L1011<2012
 42411|  br i1 %1813, label %1818, label %1814                                                                                 ;L1011<2012
 42412| 
 42413| 1814: ; preds = %1812
 42414|     ;; target_entity = ptr %1811
 42415|     ;; self = ptr %1766
 42416|  %1815 = load i32, ptr %1767, , !!8                                                                                    ;L149<2013
 42417|  %1816 = add nsw i32 %1815, -1                                                                                         ;L149<2013
 42418|  %1817 = icmp ult i32 %1816, 2                                                                                         ;L149<2013
 42419|  br i1 %1817, label %1823, label %1819                                                                                 ;L149<2013
 42420| 
 42421| 1818: ; preds = %1812
 42422|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.144) #25
 42423|  to label %173 unwind label %365                                                                                       ;L1013<2012
 42424| 
 42425| 1819: ; preds = %1814
 42426|  %1820 = gep %59, i64 104                                                                                              ;L1564<2013
 42427|  %1821 = load i64, ptr %1820, , !!8                                                                                    ;L1564<2013
 42428|  %1822 = icmp eq i64 %1821, 13                                                                                         ;L1564<2013
 42429|  br i1 %1822, label %1825, label %1823                                                                                 ;L1564<2013
 42430| 
 42431| 1823: ; preds = %1825, %1819, %1814
 42432|  %1824 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1766, ptr %64, ptr %59, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1811)
 42433|  to label %1835 unwind label %365                                                                                      ;L2014
 42434| 
 42435| 1825: ; preds = %1819
 42436|     ;; champ = ptr %59
 42437|  %1826 = gep %59, i64 112                                                                                              ;L1565<2013
 42438|  %1827 = load i64, ptr %1826, , !!8                                                                                    ;L1565<2013
 42439|  %1828 = icmp eq i64 %1827, 3                                                                                          ;L1565<2013
 42440|  br i1 %1828, label %1829, label %1823                                                                                 ;L2013
 42441| 
 42442| 1829: ; preds = %1825
 42443|  %1830 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1766, ptr %64, ptr %59, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1811)
 42444|  to label %1831 unwind label %365                                                                                      ;L2016
 42445| 
 42446| 1831: ; preds = %1829
 42447|  %1832 = gep %1801, i64 112                                                                                            ;L2016
 42448|  %1833 = load i64, ptr %1832, , !!8                                                                                    ;L2016
 42449|  %1834 = add i64 %1833, %1830                                                                                          ;L2016
 42450|  store i64 %1834, ptr %1832,                                                                                           ;L2016
 42451|  br label %1736                                                                                                        ;L2013
 42452| 
 42453| 1835: ; preds = %1823
 42454|  %1836 = gep %1801, i64 128                                                                                            ;L2014
 42455|  %1837 = load i64, ptr %1836, , !!8                                                                                    ;L2014
 42456|  %1838 = add i64 %1837, %1824                                                                                          ;L2014
 42457|  store i64 %1838, ptr %1836,                                                                                           ;L2014
 42458|  br label %1736                                                                                                        ;L2013
 42459| 
 42460| 1839: ; preds = %1770
 42461|     ;; self = ptr %1772
 42462|     ;; skill = ptr %1772
 42463|     ;; self = ptr %47
 42464|     ;; self = ptr %47
 42465|  %1840 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<2022
 42466|     ;; ptr = ptr %1840
 42467|  %1841 = load i64, ptr %74, , !!8                                                                                      ;L2085<2022
 42468|     ;; len = i64 %1841
 42469|     ;; count = i64 %1841
 42470|     ;; self[0..+8] = ptr %1840
 42471|     ;; slice[0..+8] = ptr %1840
 42472|     ;; self[8..+8] = i64 %1841
 42473|     ;; slice[8..+8] = i64 %1841
 42474|     ;; ptr = ptr %1840
 42475|     ;; self = ptr %1840
 42476|  %1842 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %1840, i64 %1841 ;L961<240<1062<2022
 42477|     ;; predicate = ptr undef
 42478|     ;; self = ptr undef
 42479|     ;; self = ptr undef
 42480|     ;; count = i64 1
 42481|  br label %1843                                                                                                        ;L348<2022
 42482| 
 42483| 1843: ; preds = %1846, %1839
 42484|  %1844 = phi ptr [ %1847, %1846 ], [ %1840, %1839 ]
 42485|     ;; ptr = ptr %1844
 42486|     ;; self = ptr %1844
 42487|     ;; end_or_len = ptr %1842
 42490|  %1845 = icmp eq ptr %1844, %1842                                                                                      ;L1714<180<348<2022
 42491|  br i1 %1845, label %1736, label %1846                                                                                 ;L180<348<2022
 42492| 
 42493| 1846: ; preds = %1843
 42494|  %1847 = gep %1844, i64 216                                                                                            ;L656<185<348<2022
 42495|     ;; x = ptr %1844
 42498|  %1848 = gep %1844, i64 88                                                                                             ;L2022<349<2022
 42499|  %1849 = load i64, ptr %1848, , !!56550, !!8                                                                           ;L2022<349<2022
 42500|  %1850 = icmp eq i64 %1849, %1771                                                                                      ;L2022<349<2022
 42501|  br i1 %1850, label %1852, label %1843                                                                                 ;L349<2022
 42502| 
 42503| 1851: ; preds = %1770
 42504|     ;; self = ptr null
 42505|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.145) #25
 42506|  to label %173 unwind label %365                                                                                       ;L1013<2021
 42507| 
 42508| 1852: ; preds = %1846
 42509|     ;; target = ptr %1844
 42510|  %1853 = load ptr, ptr %372, , !!8                                                                                     ;L2023
 42511|  %1854 = invoke ptr %1853(ptr %177, i64 %1771)
 42512|  to label %1855 unwind label %365                                                                                      ;L2023
 42513| 
 42514| 1855: ; preds = %1852
 42515|     ;; self = ptr %1854
 42516|  %1856 = icmp eq ptr %1854, null                                                                                       ;L1011<2023
 42517|  br i1 %1856, label %1861, label %1857                                                                                 ;L1011<2023
 42518| 
 42519| 1857: ; preds = %1855
 42520|     ;; target_entity = ptr %1854
 42521|     ;; self = ptr %1772
 42522|  %1858 = load i32, ptr %1773, , !!8                                                                                    ;L149<2024
 42523|  %1859 = add nsw i32 %1858, -1                                                                                         ;L149<2024
 42524|  %1860 = icmp ult i32 %1859, 2                                                                                         ;L149<2024
 42525|  br i1 %1860, label %1866, label %1862                                                                                 ;L149<2024
 42526| 
 42527| 1861: ; preds = %1855
 42528|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.146) #25
 42529|  to label %173 unwind label %365                                                                                       ;L1013<2023
 42530| 
 42531| 1862: ; preds = %1857
 42532|  %1863 = gep %59, i64 104                                                                                              ;L1571<2024
 42533|  %1864 = load i64, ptr %1863, , !!8                                                                                    ;L1571<2024
 42534|  %1865 = icmp eq i64 %1864, 13                                                                                         ;L1571<2024
 42535|  br i1 %1865, label %1868, label %1866                                                                                 ;L1571<2024
 42536| 
 42537| 1866: ; preds = %1868, %1862, %1857
 42538|  %1867 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1772, ptr %64, ptr %59, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1854)
 42539|  to label %1878 unwind label %365                                                                                      ;L2025
 42540| 
 42541| 1868: ; preds = %1862
 42542|     ;; champ = ptr %59
 42543|  %1869 = gep %59, i64 112                                                                                              ;L1572<2024
 42544|  %1870 = load i64, ptr %1869, , !!8                                                                                    ;L1572<2024
 42545|  %1871 = icmp eq i64 %1870, 4                                                                                          ;L1572<2024
 42546|  br i1 %1871, label %1872, label %1866                                                                                 ;L2024
 42547| 
 42548| 1872: ; preds = %1868
 42549|  %1873 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1772, ptr %64, ptr %59, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1854)
 42550|  to label %1874 unwind label %365                                                                                      ;L2027
 42551| 
 42552| 1874: ; preds = %1872
 42553|  %1875 = gep %1844, i64 112                                                                                            ;L2027
 42554|  %1876 = load i64, ptr %1875, , !!8                                                                                    ;L2027
 42555|  %1877 = add i64 %1876, %1873                                                                                          ;L2027
 42556|  store i64 %1877, ptr %1875,                                                                                           ;L2027
 42557|  br label %1736                                                                                                        ;L2024
 42558| 
 42559| 1878: ; preds = %1866
 42560|  %1879 = gep %1844, i64 128                                                                                            ;L2025
 42561|  %1880 = load i64, ptr %1879, , !!8                                                                                    ;L2025
 42562|  %1881 = add i64 %1880, %1867                                                                                          ;L2025
 42563|  store i64 %1881, ptr %1879,                                                                                           ;L2025
 42564|  br label %1736                                                                                                        ;L2024
 42565| 
 42566| 1882: ; preds = %1776
 42567|     ;; self = ptr %1782
 42568|     ;; skill2 = ptr %1782
 42569|     ;; self = ptr %47
 42570|     ;; self = ptr %47
 42571|  %1883 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<2034
 42572|     ;; ptr = ptr %1883
 42573|  %1884 = load i64, ptr %74, , !!8                                                                                      ;L2085<2034
 42574|     ;; len = i64 %1884
 42575|     ;; count = i64 %1884
 42576|     ;; self[0..+8] = ptr %1883
 42577|     ;; slice[0..+8] = ptr %1883
 42578|     ;; self[8..+8] = i64 %1884
 42579|     ;; slice[8..+8] = i64 %1884
 42580|     ;; ptr = ptr %1883
 42581|     ;; self = ptr %1883
 42582|  %1885 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %1883, i64 %1884 ;L961<240<1062<2034
 42583|     ;; predicate = ptr undef
 42584|     ;; self = ptr undef
 42585|     ;; self = ptr undef
 42586|     ;; count = i64 1
 42587|  br label %1886                                                                                                        ;L348<2034
 42588| 
 42589| 1886: ; preds = %1889, %1882
 42590|  %1887 = phi ptr [ %1890, %1889 ], [ %1883, %1882 ]
 42591|     ;; ptr = ptr %1887
 42592|     ;; self = ptr %1887
 42593|     ;; end_or_len = ptr %1885
 42596|  %1888 = icmp eq ptr %1887, %1885                                                                                      ;L1714<180<348<2034
 42597|  br i1 %1888, label %1736, label %1889                                                                                 ;L180<348<2034
 42598| 
 42599| 1889: ; preds = %1886
 42600|  %1890 = gep %1887, i64 216                                                                                            ;L656<185<348<2034
 42601|     ;; x = ptr %1887
 42604|  %1891 = gep %1887, i64 88                                                                                             ;L2034<349<2034
 42605|  %1892 = load i64, ptr %1891, , !!56617, !!8                                                                           ;L2034<349<2034
 42606|  %1893 = icmp eq i64 %1892, %1777                                                                                      ;L2034<349<2034
 42607|  br i1 %1893, label %1895, label %1886                                                                                 ;L349<2034
 42608| 
 42609| 1894: ; preds = %1776
 42610|     ;; self = ptr null
 42611|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.147) #25
 42612|  to label %173 unwind label %365                                                                                       ;L1013<2032
 42613| 
 42614| 1895: ; preds = %1889
 42615|     ;; target = ptr %1887
 42616|  %1896 = load ptr, ptr %372, , !!8                                                                                     ;L2035
 42617|  %1897 = invoke ptr %1896(ptr %177, i64 %1777)
 42618|  to label %1898 unwind label %365                                                                                      ;L2035
 42619| 
 42620| 1898: ; preds = %1895
 42621|     ;; self = ptr %1897
 42622|  %1899 = icmp eq ptr %1897, null                                                                                       ;L1011<2035
 42623|  br i1 %1899, label %1904, label %1900                                                                                 ;L1011<2035
 42624| 
 42625| 1900: ; preds = %1898
 42626|     ;; target_entity = ptr %1897
 42627|     ;; self = ptr %1782
 42628|  %1901 = load i32, ptr %1783, , !!8                                                                                    ;L149<2036
 42629|  %1902 = add nsw i32 %1901, -1                                                                                         ;L149<2036
 42630|  %1903 = icmp ult i32 %1902, 2                                                                                         ;L149<2036
 42631|  br i1 %1903, label %1909, label %1905                                                                                 ;L149<2036
 42632| 
 42633| 1904: ; preds = %1898
 42634|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.148) #25
 42635|  to label %173 unwind label %365                                                                                       ;L1013<2035
 42636| 
 42637| 1905: ; preds = %1900
 42638|  %1906 = gep %59, i64 104                                                                                              ;L1579<2036
 42639|  %1907 = load i64, ptr %1906, , !!8                                                                                    ;L1579<2036
 42640|  %1908 = icmp eq i64 %1907, 13                                                                                         ;L1579<2036
 42641|  br i1 %1908, label %1911, label %1909                                                                                 ;L1579<2036
 42642| 
 42643| 1909: ; preds = %1911, %1905, %1900
 42644|  %1910 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1782, ptr %64, ptr %59, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1897)
 42645|  to label %1921 unwind label %365                                                                                      ;L2037
 42646| 
 42647| 1911: ; preds = %1905
 42648|     ;; champ = ptr %59
 42649|  %1912 = gep %59, i64 112                                                                                              ;L1580<2036
 42650|  %1913 = load i64, ptr %1912, , !!8                                                                                    ;L1580<2036
 42651|  %1914 = icmp eq i64 %1913, 5                                                                                          ;L1580<2036
 42652|  br i1 %1914, label %1915, label %1909                                                                                 ;L2036
 42653| 
 42654| 1915: ; preds = %1911
 42655|  %1916 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1782, ptr %64, ptr %59, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1897)
 42656|  to label %1917 unwind label %365                                                                                      ;L2039
 42657| 
 42658| 1917: ; preds = %1915
 42659|  %1918 = gep %1887, i64 112                                                                                            ;L2039
 42660|  %1919 = load i64, ptr %1918, , !!8                                                                                    ;L2039
 42661|  %1920 = add i64 %1919, %1916                                                                                          ;L2039
 42662|  store i64 %1920, ptr %1918,                                                                                           ;L2039
 42663|  br label %1736                                                                                                        ;L2036
 42664| 
 42665| 1921: ; preds = %1909
 42666|  %1922 = gep %1887, i64 128                                                                                            ;L2037
 42667|  %1923 = load i64, ptr %1922, , !!8                                                                                    ;L2037
 42668|  %1924 = add i64 %1923, %1910                                                                                          ;L2037
 42669|  store i64 %1924, ptr %1922,                                                                                           ;L2037
 42670|  br label %1736                                                                                                        ;L2036
 42671| 
 42672| 1925: ; preds = %1786
 42673|     ;; self = ptr %1792
 42674|     ;; ult = ptr %1792
 42675|     ;; self = ptr %47
 42676|     ;; self = ptr %47
 42677|  %1926 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<2047
 42678|     ;; ptr = ptr %1926
 42679|  %1927 = load i64, ptr %74, , !!8                                                                                      ;L2085<2047
 42680|     ;; len = i64 %1927
 42681|     ;; count = i64 %1927
 42682|     ;; self[0..+8] = ptr %1926
 42683|     ;; slice[0..+8] = ptr %1926
 42684|     ;; self[8..+8] = i64 %1927
 42685|     ;; slice[8..+8] = i64 %1927
 42686|     ;; ptr = ptr %1926
 42687|     ;; self = ptr %1926
 42688|  %1928 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %1926, i64 %1927 ;L961<240<1062<2047
 42689|     ;; predicate = ptr undef
 42690|     ;; self = ptr undef
 42691|     ;; self = ptr undef
 42692|     ;; count = i64 1
 42693|  br label %1929                                                                                                        ;L348<2047
 42694| 
 42695| 1929: ; preds = %1932, %1925
 42696|  %1930 = phi ptr [ %1933, %1932 ], [ %1926, %1925 ]
 42697|     ;; ptr = ptr %1930
 42698|     ;; self = ptr %1930
 42699|     ;; end_or_len = ptr %1928
 42702|  %1931 = icmp eq ptr %1930, %1928                                                                                      ;L1714<180<348<2047
 42703|  br i1 %1931, label %1736, label %1932                                                                                 ;L180<348<2047
 42704| 
 42705| 1932: ; preds = %1929
 42706|  %1933 = gep %1930, i64 216                                                                                            ;L656<185<348<2047
 42707|     ;; x = ptr %1930
 42710|  %1934 = gep %1930, i64 88                                                                                             ;L2047<349<2047
 42711|  %1935 = load i64, ptr %1934, , !!56684, !!8                                                                           ;L2047<349<2047
 42712|  %1936 = icmp eq i64 %1935, %1787                                                                                      ;L2047<349<2047
 42713|  br i1 %1936, label %1938, label %1929                                                                                 ;L349<2047
 42714| 
 42715| 1937: ; preds = %1786
 42716|     ;; self = ptr null
 42717|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.149) #25
 42718|  to label %173 unwind label %365                                                                                       ;L1013<2045
 42719| 
 42720| 1938: ; preds = %1932
 42721|     ;; target = ptr %1930
 42722|  %1939 = load ptr, ptr %372, , !!8                                                                                     ;L2048
 42723|  %1940 = invoke ptr %1939(ptr %177, i64 %1787)
 42724|  to label %1941 unwind label %365                                                                                      ;L2048
 42725| 
 42726| 1941: ; preds = %1938
 42727|     ;; self = ptr %1940
 42728|  %1942 = icmp eq ptr %1940, null                                                                                       ;L1011<2048
 42729|  br i1 %1942, label %1947, label %1943                                                                                 ;L1011<2048
 42730| 
 42731| 1943: ; preds = %1941
 42732|     ;; target_entity = ptr %1940
 42733|     ;; self = ptr %1792
 42734|  %1944 = load i32, ptr %1793, , !!8                                                                                    ;L149<2049
 42735|  %1945 = add nsw i32 %1944, -1                                                                                         ;L149<2049
 42736|  %1946 = icmp ult i32 %1945, 2                                                                                         ;L149<2049
 42737|  br i1 %1946, label %1952, label %1948                                                                                 ;L149<2049
 42738| 
 42739| 1947: ; preds = %1941
 42740|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.150) #25
 42741|  to label %173 unwind label %365                                                                                       ;L1013<2048
 42742| 
 42743| 1948: ; preds = %1943
 42744|  %1949 = gep %59, i64 104                                                                                              ;L1586<2049
 42745|  %1950 = load i64, ptr %1949, , !!8                                                                                    ;L1586<2049
 42746|  %1951 = icmp eq i64 %1950, 13                                                                                         ;L1586<2049
 42747|  br i1 %1951, label %1954, label %1952                                                                                 ;L1586<2049
 42748| 
 42749| 1952: ; preds = %1954, %1948, %1943
 42750|  %1953 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1792, ptr %64, ptr %59, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1940)
 42751|  to label %1964 unwind label %365                                                                                      ;L2050
 42752| 
 42753| 1954: ; preds = %1948
 42754|     ;; champ = ptr %59
 42755|  %1955 = gep %59, i64 112                                                                                              ;L1587<2049
 42756|  %1956 = load i64, ptr %1955, , !!8                                                                                    ;L1587<2049
 42757|  %1957 = icmp eq i64 %1956, 6                                                                                          ;L1587<2049
 42758|  br i1 %1957, label %1958, label %1952                                                                                 ;L2049
 42759| 
 42760| 1958: ; preds = %1954
 42761|  %1959 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1792, ptr %64, ptr %59, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1940)
 42762|  to label %1960 unwind label %365                                                                                      ;L2052
 42763| 
 42764| 1960: ; preds = %1958
 42765|  %1961 = gep %1930, i64 112                                                                                            ;L2052
 42766|  %1962 = load i64, ptr %1961, , !!8                                                                                    ;L2052
 42767|  %1963 = add i64 %1962, %1959                                                                                          ;L2052
 42768|  store i64 %1963, ptr %1961,                                                                                           ;L2052
 42769|  br label %1736                                                                                                        ;L2049
 42770| 
 42771| 1964: ; preds = %1952
 42772|  %1965 = gep %1930, i64 128                                                                                            ;L2050
 42773|  %1966 = load i64, ptr %1965, , !!8                                                                                    ;L2050
 42774|  %1967 = add i64 %1966, %1953                                                                                          ;L2050
 42775|  store i64 %1967, ptr %1965,                                                                                           ;L2050
 42776|  br label %1736                                                                                                        ;L2049
 42777| 
 42778| 1968: ; preds = %2000, %1736
 42779|  %1969 = phi ptr [ %1737, %1736 ], [ %1972, %2000 ]                                                                    ;L2060
 42780|     ;; iter[0..+8] = ptr %1969
 42781|     ;; self = ptr undef
 42782|     ;; ptr = ptr %1969
 42783|     ;; self = ptr %1969
 42784|     ;; end_or_len = ptr %1739
 42787|  %1970 = icmp eq ptr %1969, %1739                                                                                      ;L1714<180<2060
 42788|  br i1 %1970, label %1706, label %1971                                                                                 ;L180<2060
 42789| 
 42790| 1971: ; preds = %1968
 42791|  %1972 = gep %1969, i64 216                                                                                            ;L656<185<2060
 42792|     ;; iter[0..+8] = ptr %1972
 42793|     ;; a = ptr %1969
 42794|  %1973 = gep %1969, i64 88                                                                                             ;L2061
 42795|  %1974 = load i64, ptr %1973, , !!8                                                                                    ;L2061
 42796|  %1975 = load ptr, ptr %372, , !!8                                                                                     ;L2061
 42797|  %1976 = invoke ptr %1975(ptr %177, i64 %1974)
 42798|  to label %1977 unwind label %365                                                                                      ;L2061
 42799| 
 42800| 1977: ; preds = %1971
 42801|  %1978 = icmp eq ptr %1976, null                                                                                       ;L2061
 42802|  br i1 %1978, label %2000, label %1979                                                                                 ;L2061
 42803| 
 42804| 1979: ; preds = %1977
 42805|     ;; ae = ptr %1976
 42806|     ;; other = ptr %1976
 42807|     ;; self = ptr %1976
 42808|     ;; self = ptr %1976
 42809|     ;; self = ptr %1976
 42810|  %1980 = load i64, ptr %373, , !!8                                                                                     ;L2158<2062
 42811|     ;; x1 = i64 %1980
 42812|     ;; self = i64 %1980
 42813|  %1981 = load i64, ptr %374, , !!8                                                                                     ;L2158<2062
 42814|     ;; y1 = i64 %1981
 42815|     ;; self = i64 %1981
 42816|  %1982 = gep %1976, i64 1632                                                                                           ;L2158<2062
 42817|  %1983 = load i64, ptr %1982, , !!8                                                                                    ;L2158<2062
 42818|     ;; x2 = i64 %1983
 42819|     ;; other = i64 %1983
 42820|  %1984 = gep %1976, i64 1640                                                                                           ;L2158<2062
 42821|  %1985 = load i64, ptr %1984, , !!8                                                                                    ;L2158<2062
 42822|     ;; y2 = i64 %1985
 42823|     ;; other = i64 %1985
 42824|  %1986 = icmp ult i64 %1980, %1983                                                                                     ;L3147<7<2158<2062
 42825|  %1987 = sub nuw i64 %1983, %1980                                                                                      ;L3147<7<2158<2062
 42826|  %1988 = sub nuw i64 %1980, %1983                                                                                      ;L3147<7<2158<2062
 42827|  %1989 = select i1 %1986, i64 %1987, i64 %1988                                                                         ;L3147<7<2158<2062
 42828|     ;; dx = i64 %1989
 42829|  %1990 = icmp ult i64 %1981, %1985                                                                                     ;L3147<8<2158<2062
 42830|  %1991 = sub nuw i64 %1985, %1981                                                                                      ;L3147<8<2158<2062
 42831|  %1992 = sub nuw i64 %1981, %1985                                                                                      ;L3147<8<2158<2062
 42832|  %1993 = select i1 %1990, i64 %1991, i64 %1992                                                                         ;L3147<8<2158<2062
 42833|     ;; dy = i64 %1993
 42834|  %1994 = mul i64 %1989, %1989                                                                                          ;L9<2158<2062
 42835|  %1995 = mul i64 %1993, %1993                                                                                          ;L9<2158<2062
 42836|  %1996 = add i64 %1995, %1994                                                                                          ;L9<2158<2062
 42837|     ;; dist = i64 %1996
 42838|  %1997 = load i64, ptr %1740, , !!8                                                                                    ;L2063
 42839|     ;; speed = i64 %1997
 42840|     ;; self = ptr %59
 42841|  %1998 = load i32, ptr %1742, , !!8                                                                                    ;L742<2064
 42842|  %1999 = icmp eq i32 %1998, -1                                                                                         ;L742<2064
 42843|  br i1 %1999, label %2008, label %2001                                                                                 ;L742<2064
 42844| 
 42845| 2000: ; preds = %2217, %2195, %2192, %2170, %2160, %1977
 42846|  br label %1968                                                                                                        ;L2060
 42847| 
 42848| 2001: ; preds = %1979
 42849|     ;; self = ptr %1741
 42850|     ;; atk = ptr %1741
 42851|     ;; self = ptr %1741
 42852|  %2002 = load i64, ptr %1743, , !!8                                                                                    ;L26<2065
 42853|  %2003 = load i64, ptr %1744, , !!8                                                                                    ;L26<2065
 42854|  %2004 = load i64, ptr %1745, , !!8                                                                                    ;L26<2065
 42855|  %2005 = add i64 %2004, -1                                                                                             ;L26<2065
 42856|  %2006 = load i64, ptr %1746, , !!8                                                                                    ;L26<2065
 42857|  %2007 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %1741, ptr %59, ptr %1976)
 42858|  to label %2009 unwind label %365                                                                                      ;L2065
 42859| 
 42860| 2008: ; preds = %1979
 42861|     ;; self = ptr null
 42862|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.151) #25
 42863|  to label %173 unwind label %365                                                                                       ;L1013<2064
 42864| 
 42865| 2009: ; preds = %2001
 42866|  %2010 = mul i64 %2005, %2003                                                                                          ;L26<2065
 42867|  %2011 = gep %1976, i64 1136                                                                                           ;L1511<2065
 42868|  %2012 = load i32, ptr %2011, , !!8                                                                                    ;L1511<2065
 42869|  %2013 = sext i32 %2012 to i64                                                                                         ;L1511<2065
 42870|     ;; mult = i64 %2013
 42871|     ;; mult = i64 %2013
 42872|     ;; mult = i64 %2013
 42873|  %2014 = icmp eq i32 %2012, 0                                                                                          ;L1512<2065
 42874|  %2015 = gep %1976, i64 1664                                                                                           ;L0<2065
 42875|  %2016 = load i64, ptr %2015, , !!8                                                                                    ;L0<2065
 42876|  br i1 %2014, label %2021, label %2017                                                                                 ;L1512<2065
 42877| 
 42878| 2017: ; preds = %2009
 42879|  %2018 = add nsw i64 %2013, 100                                                                                        ;L1515<2065
 42880|  %2019 = mul i64 %2016, %2018                                                                                          ;L1515<2065
 42881|  %2020 = udiv i64 %2019, 100                                                                                           ;L1515<2065
 42882|  br label %2021                                                                                                        ;L1512<2065
 42883| 
 42884| 2021: ; preds = %2017, %2009
 42885|  %2022 = phi i64 [ %2020, %2017 ], [ %2016, %2009 ]                                                                    ;L0<2065
 42886|  %2023 = load i32, ptr %375, , !!8                                                                                     ;L1511<2065
 42887|  %2024 = sext i32 %2023 to i64                                                                                         ;L1511<2065
 42888|     ;; mult = i64 %2024
 42889|     ;; mult = i64 %2024
 42890|     ;; mult = i64 %2024
 42891|  %2025 = icmp eq i32 %2023, 0                                                                                          ;L1512<2065
 42892|  %2026 = load i64, ptr %376, , !!8                                                                                     ;L0<2065
 42893|  br i1 %2025, label %2031, label %2027                                                                                 ;L1512<2065
 42894| 
 42895| 2027: ; preds = %2021
 42896|  %2028 = add nsw i64 %2024, 100                                                                                        ;L1515<2065
 42897|  %2029 = mul i64 %2026, %2028                                                                                          ;L1515<2065
 42898|  %2030 = udiv i64 %2029, 100                                                                                           ;L1515<2065
 42899|  br label %2031                                                                                                        ;L1512<2065
 42900| 
 42901| 2031: ; preds = %2027, %2021
 42902|  %2032 = phi i64 [ %2030, %2027 ], [ %2026, %2021 ]                                                                    ;L0<2065
 42904|  %2033 = load i64, ptr %1747, , !!8                                                                                    ;L1748<2067
 42905|  switch i64 %2033, label %756 [
 42906|  i64 0, label %2058
 42907|  i64 1, label %2034
 42908|  i64 2, label %2036
 42909|  i64 3, label %2058
 42910|  i64 4, label %2038
 42911|  i64 5, label %2040
 42912|  i64 6, label %2042
 42913|  i64 7, label %2044
 42914|  i64 8, label %2046
 42915|  i64 9, label %2048
 42916|  i64 10, label %2050
 42917|  i64 11, label %2052
 42918|  i64 12, label %2054
 42919|  i64 13, label %2056
 42920|  ]                                                                                                                     ;L1748<2067
 42921| 
 42922| 2034: ; preds = %2031
 42923|     ;; info = ptr %59
 42924|  %2035 = load i64, ptr %1756, , !!8                                                                                    ;L1751<2067
 42925|  br label %2058                                                                                                        ;L1751<2067
 42926| 
 42927| 2036: ; preds = %2031
 42928|     ;; info = ptr %59
 42929|  %2037 = load i64, ptr %1755, , !!8                                                                                    ;L1758<2067
 42930|  br label %2058                                                                                                        ;L1758<2067
 42931| 
 42932| 2038: ; preds = %2031
 42933|     ;; info = ptr %59
 42934|  %2039 = load i64, ptr %1753, , !!8                                                                                    ;L1750<2067
 42935|  br label %2058                                                                                                        ;L1750<2067
 42936| 
 42937| 2040: ; preds = %2031
 42938|     ;; info = ptr %59
 42939|  %2041 = load i64, ptr %1754, , !!8                                                                                    ;L1760<2067
 42940|  br label %2058                                                                                                        ;L1760<2067
 42941| 
 42942| 2042: ; preds = %2031
 42943|     ;; info = ptr %59
 42944|  %2043 = load i64, ptr %1754, , !!8                                                                                    ;L1759<2067
 42945|  br label %2058                                                                                                        ;L1759<2067
 42946| 
 42947| 2044: ; preds = %2031
 42948|     ;; info = ptr %59
 42949|  %2045 = load i64, ptr %1753, , !!8                                                                                    ;L1752<2067
 42950|  br label %2058                                                                                                        ;L1752<2067
 42951| 
 42952| 2046: ; preds = %2031
 42953|     ;; info = ptr %59
 42954|  %2047 = load i64, ptr %1748, , !!8                                                                                    ;L1753<2067
 42955|  br label %2058                                                                                                        ;L1753<2067
 42956| 
 42957| 2048: ; preds = %2031
 42958|     ;; info = ptr %59
 42959|  %2049 = load i64, ptr %1752, , !!8                                                                                    ;L1754<2067
 42960|  br label %2058                                                                                                        ;L1754<2067
 42961| 
 42962| 2050: ; preds = %2031
 42963|     ;; info = ptr %59
 42964|  %2051 = load i64, ptr %1751, , !!8                                                                                    ;L1755<2067
 42965|  br label %2058                                                                                                        ;L1755<2067
 42966| 
 42967| 2052: ; preds = %2031
 42968|     ;; info = ptr %59
 42969|  %2053 = load i64, ptr %1750, , !!8                                                                                    ;L1756<2067
 42970|  br label %2058                                                                                                        ;L1756<2067
 42971| 
 42972| 2054: ; preds = %2031
 42973|     ;; info = ptr %59
 42974|  %2055 = load i64, ptr %1749, , !!8                                                                                    ;L1757<2067
 42975|  br label %2058                                                                                                        ;L1757<2067
 42976| 
 42977| 2056: ; preds = %2031
 42978|     ;; champ = ptr %59
 42979|  %2057 = load i64, ptr %1748, , !!8                                                                                    ;L1749<2067
 42980|  br label %2058                                                                                                        ;L1749<2067
 42981| 
 42982| 2058: ; preds = %2056, %2054, %2052, %2050, %2048, %2046, %2044, %2042, %2040, %2038, %2036, %2034, %2031, %2031
 42983|  %2059 = phi i64 [ %2057, %2056 ], [ %2035, %2034 ], [ %2037, %2036 ], [ 0, %2031 ], [ %2039, %2038 ], [ %2041, %2040 ], [ %2043, %2042 ], [ %2045, %2044 ], [ %2047, %2046 ], [ %2049, %2048 ], [ %2051, %2050 ], [ %2053, %2052 ], [ %2055, %2054 ], [ 0, %2031 ] ;L0<2067
 42984|     ;; self = i64 %1734
 42985|     ;; other = i64 %2059
 42986|  %2060 = call i64 @llvm.umax.i64(i64 %2059, i64 %1734)                                                                 ;L1039<2067
 42987|     ;; attack_tick = i64 %2060
 42988|  %2061 = icmp ult i64 %2060, 121                                                                                       ;L2069
 42989|  br i1 %2061, label %2064, label %2062                                                                                 ;L2069
 42990| 
 42991| 2062: ; preds = %2086, %2064, %2058
 42992|  %2063 = icmp eq i64 %2033, 13                                                                                         ;L1775<2077
 42993|  br i1 %2063, label %2094, label %2097                                                                                 ;L1775<2077
 42994| 
 42995| 2064: ; preds = %2058
 42996|  %2065 = mul i64 %1997, 20                                                                                             ;L2069
 42997|  %2066 = add i64 %2002, %2065                                                                                          ;L26<2065
 42998|  %2067 = add i64 %2066, %2006                                                                                          ;L26<2065
 42999|  %2068 = add i64 %2067, %2010                                                                                          ;L2065
 43000|  %2069 = add i64 %2068, %2007                                                                                          ;L2065
 43001|  %2070 = add i64 %2069, %2022                                                                                          ;L2065
 43002|  %2071 = add i64 %2070, %2032                                                                                          ;L2069
 43003|  %2072 = mul i64 %2071, %2071                                                                                          ;L2069
 43004|  %2073 = icmp ult i64 %2072, %1996                                                                                     ;L2069
 43005|  br i1 %2073, label %2062, label %2074                                                                                 ;L2069
 43006| 
 43007| 2074: ; preds = %2064
 43008|  %2075 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1741, ptr %64, ptr %59, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1976)
 43009|  to label %2076 unwind label %365                                                                                      ;L2072
 43010| 
 43011| 2076: ; preds = %2074
 43012|  %2077 = gep %1969, i64 24                                                                                             ;L2070
 43013|     ;; value[0..+8] = i64 %157
 43014|     ;; src[0..+8] = i64 %157
 43015|     ;; value[8..+8] = i64 %2060
 43016|     ;; src[8..+8] = i64 %2060
 43017|     ;; value[16..+8] = i64 %2075
 43018|     ;; src[16..+8] = i64 %2075
 43019|     ;; self = ptr %2077
 43020|     ;; self = ptr %2077
 43021|     ;; additional = i64 1
 43022|     ;; needed_extra_cap = i64 1
 43023|     ;; needed_extra_cap = i64 1
 43024|     ;; strategy = i8 1
 43025|  %2078 = gep %1969, i64 48                                                                                             ;L1428<2070
 43026|  %2079 = load i64, ptr %2078, , !!56796, !!8                                                                           ;L1428<2070
 43027|     ;; self = ptr %2077
 43028|  %2080 = gep %1969, i64 40                                                                                             ;L149<1428<2070
 43029|  %2081 = load i64, ptr %2080, , !!56796, !!8                                                                           ;L149<1428<2070
 43030|  %2082 = icmp eq i64 %2079, %2081                                                                                      ;L1428<2070
 43031|  br i1 %2082, label %2083, label %2086                                                                                 ;L1428<2070
 43032| 
 43033| 2083: ; preds = %2076
 43034|     ;; self = ptr %2077
 43035|     ;; self = ptr %2077
 43036|     ;; self = ptr %2077
 43037|     ;; used_cap = i64 %2079
 43038|     ;; used_cap = i64 %2079
 43039|  invoke void @ai::score_parameter12PossibleGainE25reserve_internal_or_panicB17_(ptr %2077, i64 %2079, i64 1, i1 zeroext true)
 43040|  to label %2084 unwind label %365                                                                                      ;L619<430<738<1429<2070
 43041| 
 43042| 2084: ; preds = %2083
 43043|  %2085 = load i64, ptr %2078, , !!56796                                                                                ;L1432<2070
 43044|  br label %2086                                                                                                        ;L1428<2070
 43045| 
 43046| 2086: ; preds = %2084, %2076
 43047|  %2087 = phi i64 [ %2079, %2076 ], [ %2085, %2084 ]                                                                    ;L1432<2070
 43048|     ;; self = ptr %2077
 43049|  %2088 = load ptr, ptr %2077, , !!56796, !!8, !!8                                                                      ;L138<1432<2070
 43050|     ;; self = ptr %2088
 43051|     ;; count = i64 %2087
 43052|  %2089 = gepS %2088, i64 %2087                                                                                         ;L961<1432<2070
 43053|     ;; end = ptr %2089
 43054|     ;; dst = ptr %2089
 43055|  store i64 %157, ptr %2089,                                                                                            ;L1933<1433<2070
 43056|  %2090 = gep %2089, i64 8                                                                                              ;L1933<1433<2070
 43057|  store i64 %2060, ptr %2090,                                                                                           ;L1933<1433<2070
 43058|  %2091 = gep %2089, i64 16                                                                                             ;L1933<1433<2070
 43059|  store i64 %2075, ptr %2091,                                                                                           ;L1933<1433<2070
 43060|  %2092 = load i64, ptr %2078, , !!56796, !!8                                                                           ;L1434<2070
 43061|  %2093 = add i64 %2092, 1                                                                                              ;L1434<2070
 43062|  store i64 %2093, ptr %2078, , !!56796                                                                                 ;L1434<2070
 43063|  br label %2062                                                                                                        ;L2069
 43064| 
 43065| 2094: ; preds = %2062
 43066|     ;; champ = ptr %59
 43067|  %2095 = load i64, ptr %1756, , !!8                                                                                    ;L1776<2077
 43068|  %2096 = call i64 @llvm.umax.i64(i64 %2095, i64 %1734)                                                                 ;L1039<2077
 43069|  br label %2097                                                                                                        ;L1776<2077
 43070| 
 43071| 2097: ; preds = %2094, %2062
 43072|  %2098 = phi i64 [ %2096, %2094 ], [ %1734, %2062 ]                                                                    ;L0<2077
 43073|     ;; self = i64 %1734
 43074|     ;; other = i64 %2098
 43075|     ;; skill_tick = i64 %2098
 43076|     ;; self = ptr %59
 43077|  %2099 = load i32, ptr %1758, , !!8                                                                                    ;L742<2079
 43078|  %2100 = icmp eq i32 %2099, -1                                                                                         ;L742<2079
 43079|  br i1 %2100, label %2103, label %2101                                                                                 ;L742<2079
 43080| 
 43081| 2101: ; preds = %2097
 43082|     ;; skill = ptr %1757
 43083|     ;; self = ptr %1757
 43084|  %2102 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %1759, ptr %59, ptr %1976)
 43085|  to label %2104 unwind label %365                                                                                      ;L2080
 43086| 
 43087| 2103: ; preds = %2149, %2127, %2124, %2104, %2097
 43088|  br i1 %2063, label %2157, label %2160                                                                                 ;L1790<2092
 43089| 
 43090| 2104: ; preds = %2101
 43091|  br i1 %2102, label %2105, label %2103                                                                                 ;L2080
 43092| 
 43093| 2105: ; preds = %2104
 43094|  %2106 = load i64, ptr %1760, , !!8                                                                                    ;L26<2081
 43095|  %2107 = load i64, ptr %1761, , !!8                                                                                    ;L26<2081
 43096|  %2108 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %1757, ptr %59, ptr %1976)
 43097|  to label %2109 unwind label %365                                                                                      ;L2081
 43098| 
 43099| 2109: ; preds = %2105
 43100|  %2110 = mul i64 %2107, %2005                                                                                          ;L26<2081
 43101|  %2111 = gep %1976, i64 1664                                                                                           ;L0<2081
 43102|  %2112 = load i64, ptr %2111, , !!8                                                                                    ;L0<2081
 43103|  br i1 %2014, label %2117, label %2113                                                                                 ;L1512<2081
 43104| 
 43105| 2113: ; preds = %2109
 43106|  %2114 = add nsw i64 %2013, 100                                                                                        ;L1515<2081
 43107|  %2115 = mul i64 %2112, %2114                                                                                          ;L1515<2081
 43108|  %2116 = udiv i64 %2115, 100                                                                                           ;L1515<2081
 43109|  br label %2117                                                                                                        ;L1512<2081
 43110| 
 43111| 2117: ; preds = %2113, %2109
 43112|  %2118 = phi i64 [ %2116, %2113 ], [ %2112, %2109 ]                                                                    ;L0<2081
 43113|  %2119 = load i64, ptr %376, , !!8                                                                                     ;L0<2081
 43114|  br i1 %2025, label %2124, label %2120                                                                                 ;L1512<2081
 43115| 
 43116| 2120: ; preds = %2117
 43117|  %2121 = add nsw i64 %2024, 100                                                                                        ;L1515<2081
 43118|  %2122 = mul i64 %2119, %2121                                                                                          ;L1515<2081
 43119|  %2123 = udiv i64 %2122, 100                                                                                           ;L1515<2081
 43120|  br label %2124                                                                                                        ;L1512<2081
 43121| 
 43122| 2124: ; preds = %2120, %2117
 43123|  %2125 = phi i64 [ %2123, %2120 ], [ %2119, %2117 ]                                                                    ;L0<2081
 43125|  %2126 = icmp ult i64 %2098, 121                                                                                       ;L2082
 43126|  br i1 %2126, label %2127, label %2103                                                                                 ;L2082
 43127| 
 43128| 2127: ; preds = %2124
 43130|  %2128 = mul i64 %1997, 20                                                                                             ;L2082
 43131|  %2129 = add i64 %2006, %2128                                                                                          ;L26<2081
 43132|  %2130 = add i64 %2129, %2106                                                                                          ;L26<2081
 43133|  %2131 = add i64 %2130, %2110                                                                                          ;L2081
 43134|  %2132 = add i64 %2131, %2108                                                                                          ;L2081
 43135|  %2133 = add i64 %2132, %2118                                                                                          ;L2081
 43136|  %2134 = add i64 %2133, %2125                                                                                          ;L2082
 43137|  %2135 = mul i64 %2134, %2134                                                                                          ;L2082
 43138|  %2136 = icmp ult i64 %2135, %1996                                                                                     ;L2082
 43139|  br i1 %2136, label %2103, label %2137                                                                                 ;L2082
 43140| 
 43141| 2137: ; preds = %2127
 43142|  %2138 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %1757, ptr %64, ptr %59, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1976)
 43143|  to label %2139 unwind label %365                                                                                      ;L2085
 43144| 
 43145| 2139: ; preds = %2137
 43146|  %2140 = gep %1969, i64 24                                                                                             ;L2083
 43147|     ;; value[0..+8] = i64 %157
 43148|     ;; src[0..+8] = i64 %157
 43149|     ;; value[8..+8] = i64 %2098
 43150|     ;; src[8..+8] = i64 %2098
 43151|     ;; value[16..+8] = i64 %2138
 43152|     ;; src[16..+8] = i64 %2138
 43153|     ;; self = ptr %2140
 43154|     ;; self = ptr %2140
 43155|     ;; additional = i64 1
 43156|     ;; needed_extra_cap = i64 1
 43157|     ;; needed_extra_cap = i64 1
 43158|     ;; strategy = i8 1
 43159|  %2141 = gep %1969, i64 48                                                                                             ;L1428<2083
 43160|  %2142 = load i64, ptr %2141, , !!56848, !!8                                                                           ;L1428<2083
 43161|     ;; self = ptr %2140
 43162|  %2143 = gep %1969, i64 40                                                                                             ;L149<1428<2083
 43163|  %2144 = load i64, ptr %2143, , !!56848, !!8                                                                           ;L149<1428<2083
 43164|  %2145 = icmp eq i64 %2142, %2144                                                                                      ;L1428<2083
 43165|  br i1 %2145, label %2146, label %2149                                                                                 ;L1428<2083
 43166| 
 43167| 2146: ; preds = %2139
 43168|     ;; self = ptr %2140
 43169|     ;; self = ptr %2140
 43170|     ;; self = ptr %2140
 43171|     ;; used_cap = i64 %2142
 43172|     ;; used_cap = i64 %2142
 43173|  invoke void @ai::score_parameter12PossibleGainE25reserve_internal_or_panicB17_(ptr %2140, i64 %2142, i64 1, i1 zeroext true)
 43174|  to label %2147 unwind label %365                                                                                      ;L619<430<738<1429<2083
 43175| 
 43176| 2147: ; preds = %2146
 43177|  %2148 = load i64, ptr %2141, , !!56848                                                                                ;L1432<2083
 43178|  br label %2149                                                                                                        ;L1428<2083
 43179| 
 43180| 2149: ; preds = %2147, %2139
 43181|  %2150 = phi i64 [ %2142, %2139 ], [ %2148, %2147 ]                                                                    ;L1432<2083
 43182|     ;; self = ptr %2140
 43183|  %2151 = load ptr, ptr %2140, , !!56848, !!8, !!8                                                                      ;L138<1432<2083
 43184|     ;; self = ptr %2151
 43185|     ;; count = i64 %2150
 43186|  %2152 = gepS %2151, i64 %2150                                                                                         ;L961<1432<2083
 43187|     ;; end = ptr %2152
 43188|     ;; dst = ptr %2152
 43189|  store i64 %157, ptr %2152,                                                                                            ;L1933<1433<2083
 43190|  %2153 = gep %2152, i64 8                                                                                              ;L1933<1433<2083
 43191|  store i64 %2098, ptr %2153,                                                                                           ;L1933<1433<2083
 43192|  %2154 = gep %2152, i64 16                                                                                             ;L1933<1433<2083
 43193|  store i64 %2138, ptr %2154,                                                                                           ;L1933<1433<2083
 43194|  %2155 = load i64, ptr %2141, , !!56848, !!8                                                                           ;L1434<2083
 43195|  %2156 = add i64 %2155, 1                                                                                              ;L1434<2083
 43196|  store i64 %2156, ptr %2141, , !!56848                                                                                 ;L1434<2083
 43197|  br label %2103                                                                                                        ;L2082
 43198| 
 43199| 2157: ; preds = %2103
 43200|     ;; champ = ptr %59
 43201|  %2158 = load i64, ptr %1762, , !!8                                                                                    ;L1791<2092
 43202|  %2159 = call i64 @llvm.umax.i64(i64 %2158, i64 %1734)                                                                 ;L1039<2092
 43203|  br label %2160                                                                                                        ;L1791<2092
 43204| 
 43205| 2160: ; preds = %2157, %2103
 43206|  %2161 = phi i64 [ %2159, %2157 ], [ %1734, %2103 ]                                                                    ;L0<2092
 43207|     ;; self = i64 %1734
 43208|     ;; other = i64 %2161
 43209|     ;; skill2_tick = i64 %2161
 43210|  %2162 = icmp ugt i64 %2004, 2                                                                                         ;L1693<2094
 43211|  %2163 = select i1 %2162, ptr %1763, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31                                     ;L1693<2094
 43212|     ;; self = ptr %2163
 43213|  %2164 = gep %2163, i64 48                                                                                             ;L742<2094
 43214|  %2165 = load i32, ptr %2164, , !!8                                                                                    ;L742<2094
 43215|  %2166 = icmp eq i32 %2165, -1                                                                                         ;L742<2094
 43216|  br i1 %2166, label %2000, label %2167                                                                                 ;L742<2094
 43217| 
 43218| 2167: ; preds = %2160
 43219|     ;; skill2 = ptr %2163
 43220|     ;; self = ptr %2163
 43221|  %2168 = gep %2163, i64 40                                                                                             ;L2095
 43222|  %2169 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %2168, ptr %59, ptr %1976)
 43223|  to label %2170 unwind label %365                                                                                      ;L2095
 43224| 
 43225| 2170: ; preds = %2167
 43226|  br i1 %2169, label %2171, label %2000                                                                                 ;L2095
 43227| 
 43228| 2171: ; preds = %2170
 43229|  %2172 = gep %2163, i64 16                                                                                             ;L26<2096
 43230|  %2173 = load i64, ptr %2172, , !!8                                                                                    ;L26<2096
 43231|  %2174 = gep %2163, i64 24                                                                                             ;L26<2096
 43232|  %2175 = load i64, ptr %2174, , !!8                                                                                    ;L26<2096
 43233|  %2176 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %2163, ptr %59, ptr %1976)
 43234|  to label %2177 unwind label %365                                                                                      ;L2096
 43235| 
 43236| 2177: ; preds = %2171
 43237|  %2178 = mul i64 %2175, %2005                                                                                          ;L26<2096
 43238|  %2179 = gep %1976, i64 1664                                                                                           ;L0<2096
 43239|  %2180 = load i64, ptr %2179, , !!8                                                                                    ;L0<2096
 43240|  br i1 %2014, label %2185, label %2181                                                                                 ;L1512<2096
 43241| 
 43242| 2181: ; preds = %2177
 43243|  %2182 = add nsw i64 %2013, 100                                                                                        ;L1515<2096
 43244|  %2183 = mul i64 %2180, %2182                                                                                          ;L1515<2096
 43245|  %2184 = udiv i64 %2183, 100                                                                                           ;L1515<2096
 43246|  br label %2185                                                                                                        ;L1512<2096
 43247| 
 43248| 2185: ; preds = %2181, %2177
 43249|  %2186 = phi i64 [ %2184, %2181 ], [ %2180, %2177 ]                                                                    ;L0<2096
 43250|  %2187 = load i64, ptr %376, , !!8                                                                                     ;L0<2096
 43251|  br i1 %2025, label %2192, label %2188                                                                                 ;L1512<2096
 43252| 
 43253| 2188: ; preds = %2185
 43254|  %2189 = add nsw i64 %2024, 100                                                                                        ;L1515<2096
 43255|  %2190 = mul i64 %2187, %2189                                                                                          ;L1515<2096
 43256|  %2191 = udiv i64 %2190, 100                                                                                           ;L1515<2096
 43257|  br label %2192                                                                                                        ;L1512<2096
 43258| 
 43259| 2192: ; preds = %2188, %2185
 43260|  %2193 = phi i64 [ %2191, %2188 ], [ %2187, %2185 ]                                                                    ;L0<2096
 43262|  %2194 = icmp ult i64 %2161, 121                                                                                       ;L2097
 43263|  br i1 %2194, label %2195, label %2000                                                                                 ;L2097
 43264| 
 43265| 2195: ; preds = %2192
 43267|  %2196 = mul i64 %1997, 20                                                                                             ;L2097
 43268|  %2197 = add i64 %2006, %2196                                                                                          ;L26<2096
 43269|  %2198 = add i64 %2197, %2173                                                                                          ;L26<2096
 43270|  %2199 = add i64 %2198, %2178                                                                                          ;L2096
 43271|  %2200 = add i64 %2199, %2176                                                                                          ;L2096
 43272|  %2201 = add i64 %2200, %2186                                                                                          ;L2096
 43273|  %2202 = add i64 %2201, %2193                                                                                          ;L2097
 43274|  %2203 = mul i64 %2202, %2202                                                                                          ;L2097
 43275|  %2204 = icmp ult i64 %2203, %1996                                                                                     ;L2097
 43276|  br i1 %2204, label %2000, label %2205                                                                                 ;L2097
 43277| 
 43278| 2205: ; preds = %2195
 43279|  %2206 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2163, ptr %64, ptr %59, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %1976)
 43280|  to label %2207 unwind label %365                                                                                      ;L2100
 43281| 
 43282| 2207: ; preds = %2205
 43283|  %2208 = gep %1969, i64 24                                                                                             ;L2098
 43284|     ;; value[0..+8] = i64 %157
 43285|     ;; src[0..+8] = i64 %157
 43286|     ;; value[8..+8] = i64 %2161
 43287|     ;; src[8..+8] = i64 %2161
 43288|     ;; value[16..+8] = i64 %2206
 43289|     ;; src[16..+8] = i64 %2206
 43290|     ;; self = ptr %2208
 43291|     ;; self = ptr %2208
 43292|     ;; additional = i64 1
 43293|     ;; needed_extra_cap = i64 1
 43294|     ;; needed_extra_cap = i64 1
 43295|     ;; strategy = i8 1
 43296|  %2209 = gep %1969, i64 48                                                                                             ;L1428<2098
 43297|  %2210 = load i64, ptr %2209, , !!56899, !!8                                                                           ;L1428<2098
 43298|     ;; self = ptr %2208
 43299|  %2211 = gep %1969, i64 40                                                                                             ;L149<1428<2098
 43300|  %2212 = load i64, ptr %2211, , !!56899, !!8                                                                           ;L149<1428<2098
 43301|  %2213 = icmp eq i64 %2210, %2212                                                                                      ;L1428<2098
 43302|  br i1 %2213, label %2214, label %2217                                                                                 ;L1428<2098
 43303| 
 43304| 2214: ; preds = %2207
 43305|     ;; self = ptr %2208
 43306|     ;; self = ptr %2208
 43307|     ;; self = ptr %2208
 43308|     ;; used_cap = i64 %2210
 43309|     ;; used_cap = i64 %2210
 43310|  invoke void @ai::score_parameter12PossibleGainE25reserve_internal_or_panicB17_(ptr %2208, i64 %2210, i64 1, i1 zeroext true)
 43311|  to label %2215 unwind label %365                                                                                      ;L619<430<738<1429<2098
 43312| 
 43313| 2215: ; preds = %2214
 43314|  %2216 = load i64, ptr %2209, , !!56899                                                                                ;L1432<2098
 43315|  br label %2217                                                                                                        ;L1428<2098
 43316| 
 43317| 2217: ; preds = %2215, %2207
 43318|  %2218 = phi i64 [ %2210, %2207 ], [ %2216, %2215 ]                                                                    ;L1432<2098
 43319|     ;; self = ptr %2208
 43320|  %2219 = load ptr, ptr %2208, , !!56899, !!8, !!8                                                                      ;L138<1432<2098
 43321|     ;; self = ptr %2219
 43322|     ;; count = i64 %2218
 43323|  %2220 = gepS %2219, i64 %2218                                                                                         ;L961<1432<2098
 43324|     ;; end = ptr %2220
 43325|     ;; dst = ptr %2220
 43326|  store i64 %157, ptr %2220,                                                                                            ;L1933<1433<2098
 43327|  %2221 = gep %2220, i64 8                                                                                              ;L1933<1433<2098
 43328|  store i64 %2161, ptr %2221,                                                                                           ;L1933<1433<2098
 43329|  %2222 = gep %2220, i64 16                                                                                             ;L1933<1433<2098
 43330|  store i64 %2206, ptr %2222,                                                                                           ;L1933<1433<2098
 43331|  %2223 = load i64, ptr %2209, , !!56899, !!8                                                                           ;L1434<2098
 43332|  %2224 = add i64 %2223, 1                                                                                              ;L1434<2098
 43333|  store i64 %2224, ptr %2209, , !!56899                                                                                 ;L1434<2098
 43334|  br label %2000                                                                                                        ;L2097
 43335| 
 43336| 2225: ; preds = %1706
 43337|  br label %2227                                                                                                        ;L2113
 43338| 
 43339| 2226: ; preds = %1706, %1706, %1706, %1706, %1706, %1706
 43340|  br label %2227                                                                                                        ;L2114
 43341| 
 43342| 2227: ; preds = %2226, %2225, %1706, %1706
 43343|  %2228 = phi i64 [ 5112, %2226 ], [ 5128, %2225 ], [ 5120, %1706 ], [ 5120, %1706 ]
 43344|  %2229 = gep %1710, i64 %2228                                                                                          ;L0
 43345|  %2230 = load i64, ptr %2229, , !!8                                                                                    ;L0
 43346|     ;; disable_tick = i64 %2230
 43347|  %2231 = gep %179, i64 40                                                                                              ;L2117
 43348|  %2232 = load ptr, ptr %2231, , !!8                                                                                    ;L2117
 43349|  %2233 = invoke i64 %2232(ptr %177)
 43350|  to label %2234 unwind label %365                                                                                      ;L2117
 43351| 
 43352| 2234: ; preds = %2227
 43353|  %2235 = icmp ugt i64 %2233, %2230                                                                                     ;L2117
 43354|  br i1 %2235, label %2822, label %2236                                                                                 ;L2117
 43355| 
 43356| 2236: ; preds = %2234
 43357|     ;; self = ptr %31
 43358|     ;; self = ptr %31
 43359|     ;; self = ptr %31
 43360|  %2237 = load ptr, ptr %31, , !!8, !!8                                                                                 ;L138<2073<2136<2118
 43361|     ;; p = ptr %2237
 43362|  %2238 = gep %31, i64 24                                                                                               ;L2075<2136<2118
 43363|  %2239 = load i64, ptr %2238, , !!8                                                                                    ;L2075<2136<2118
 43364|     ;; len = i64 %2239
 43365|     ;; count = i64 %2239
 43366|     ;; self[0..+8] = ptr %2237
 43367|     ;; slice[0..+8] = ptr %2237
 43368|     ;; self[8..+8] = i64 %2239
 43369|     ;; slice[8..+8] = i64 %2239
 43370|     ;; ptr = ptr %2237
 43371|     ;; self = ptr %2237
 43372|  %2240 = getelementptr ptr, ptr %2237, i64 %2239                                                                       ;L961<100<1042<2136<2118
 43373|     ;; iter[0..+8] = ptr %2237
 43374|     ;; iter[8..+8] = ptr %2240
 43375|  %2241 = gep %59, i64 8
 43376|  %2242 = gep %59, i64 1600
 43377|  %2243 = gep %24, i64 56
 43378|  %2244 = gep %14, i64 8
 43379|  %2245 = gep %14, i64 16
 43380|  %2246 = gep %22, i64 56
 43381|  %2247 = gep %12, i64 8
 43382|  %2248 = gep %12, i64 16
 43383|  br label %2249                                                                                                        ;L2118
 43384| 
 43385| 2249: ; preds = %2348, %2236
 43386|  %2250 = phi ptr [ %2237, %2236 ], [ %2253, %2348 ]                                                                    ;L2118
 43387|     ;; iter[0..+8] = ptr %2250
 43388|     ;; self = ptr undef
 43389|     ;; ptr = ptr %2250
 43390|     ;; self = ptr %2250
 43391|     ;; end_or_len = ptr %2240
 43394|  %2251 = icmp eq ptr %2250, %2240                                                                                      ;L1714<180<2118
 43395|  br i1 %2251, label %2259, label %2252                                                                                 ;L180<2118
 43396| 
 43397| 2252: ; preds = %2249
 43398|  %2253 = gep %2250, i64 8                                                                                              ;L656<185<2118
 43399|     ;; iter[0..+8] = ptr %2253
 43400|     ;; t = ptr %2250
 43401|  %2254 = load ptr, ptr %2250, , !!8, !!8                                                                               ;L2119
 43402|     ;; self = ptr %2254
 43403|     ;; other = ptr %59
 43404|  %2255 = load i64, ptr %2254, , !!8                                                                                    ;L1127<2119
 43405|  %2256 = gep %2254, i64 8                                                                                              ;L1127<2119
 43406|     ;; __self_discr = i64 %2255
 43407|  %2257 = load i64, ptr %59, , !!8                                                                                      ;L1127<2119
 43408|     ;; __arg1_discr = i64 %2257
 43409|  %2258 = icmp eq i64 %2255, %2257                                                                                      ;L1127<2119
 43410|  br i1 %2258, label %2266, label %2268                                                                                 ;L1127<2119
 43411| 
 43412| 2259: ; preds = %2249
 43413|     ;; self = ptr %47
 43414|     ;; self = ptr %47
 43415|  %2260 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<2170
 43416|     ;; ptr = ptr %2260
 43417|  %2261 = load i64, ptr %74, , !!8                                                                                      ;L2085<2170
 43418|     ;; len = i64 %2261
 43419|     ;; count = i64 %2261
 43420|     ;; self[0..+8] = ptr %2260
 43421|     ;; slice[0..+8] = ptr %2260
 43422|     ;; self[8..+8] = i64 %2261
 43423|     ;; slice[8..+8] = i64 %2261
 43424|     ;; ptr = ptr %2260
 43425|     ;; self = ptr %2260
 43426|  %2262 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %2260, i64 %2261 ;L961<240<1062<2170
 43427|     ;; iter[0..+8] = ptr %2260
 43428|     ;; iter[8..+8] = ptr %2262
 43429|  %2263 = gep %20, i64 56
 43430|  %2264 = gep %10, i64 8
 43431|  %2265 = gep %10, i64 16
 43432|  br label %2447                                                                                                        ;L2170
 43433| 
 43434| 2266: ; preds = %2252
 43435|  %2267 = icmp eq i64 %2255, 0                                                                                          ;L1127<2119
 43436|  br i1 %2267, label %2273, label %2348                                                                                 ;L1127<2119
 43437| 
 43438| 2268: ; preds = %2273, %2252
 43439|  %2269 = load i64, ptr %373, , !!8                                                                                     ;L2123
 43440|  %2270 = load i64, ptr %374, , !!8                                                                                     ;L2123
 43441|  %2271 = load i32, ptr %375, , !!8                                                                                     ;L1511<2123
 43442|     ;; mult = i32 %2271
 43443|  %2272 = icmp eq i32 %2271, 0                                                                                          ;L1512<2123
 43444|  br i1 %2272, label %2277, label %2279                                                                                 ;L1512<2123
 43445| 
 43446| 2273: ; preds = %2266
 43447|     ;; __self_0 = ptr %2254
 43448|     ;; self = ptr %2254
 43449|     ;; __arg1_0 = ptr %59
 43450|     ;; other = ptr %59
 43453|  %2274 = load i64, ptr %2256, , !!8                                                                                    ;L1878<2123<1127<2119
 43454|  %2275 = load i64, ptr %2241, , !!8                                                                                    ;L1878<2123<1127<2119
 43455|  %2276 = icmp eq i64 %2274, %2275                                                                                      ;L1878<2123<1127<2119
 43456|  br i1 %2276, label %2348, label %2268                                                                                 ;L2119
 43457| 
 43458| 2277: ; preds = %2268
 43459|  %2278 = load i64, ptr %376, , !!8                                                                                     ;L1513<2123
 43460|  br label %2285                                                                                                        ;L1512<2123
 43461| 
 43462| 2279: ; preds = %2268
 43463|  %2280 = sext i32 %2271 to i64                                                                                         ;L1511<2123
 43464|     ;; mult = i64 %2280
 43465|  %2281 = load i64, ptr %376, , !!8                                                                                     ;L1515<2123
 43466|  %2282 = add nsw i64 %2280, 100                                                                                        ;L1515<2123
 43467|  %2283 = mul i64 %2281, %2282                                                                                          ;L1515<2123
 43468|  %2284 = udiv i64 %2283, 100                                                                                           ;L1515<2123
 43469|  br label %2285                                                                                                        ;L1512<2123
 43470| 
 43471| 2285: ; preds = %2279, %2277
 43472|  %2286 = phi i64 [ %2278, %2277 ], [ %2284, %2279 ]                                                                    ;L0<2123
 43473|  %2287 = load i64, ptr %2242, , !!8                                                                                    ;L2123
 43474|  %2288 = mul i64 %2287, 30                                                                                             ;L2123
 43475|     ;; tower = ptr %2254
 43476|     ;; caster = ptr %2254
 43477|     ;; self = ptr %2254
 43478|     ;; x = i64 %2269
 43479|     ;; x1 = i64 %2269
 43480|     ;; self = i64 %2269
 43481|     ;; y = i64 %2270
 43482|     ;; y1 = i64 %2270
 43483|     ;; self = i64 %2270
 43484|     ;; r = i64 %2286
 43485|     ;; d = i64 %2288
 43486|  %2289 = gep %2254, i64 1216                                                                                           ;L1449<2123
 43487|  %2290 = load i32, ptr %2289, , !!8                                                                                    ;L1449<2123
 43488|  %2291 = icmp eq i32 %2290, -1                                                                                         ;L1449<2123
 43489|  br i1 %2291, label %2348, label %2292                                                                                 ;L1449<2123
 43490| 
 43491| 2292: ; preds = %2285
 43492|     ;; attack = ptr %2254
 43493|     ;; self = ptr %2254
 43494|  %2293 = gep %2254, i64 1184                                                                                           ;L26<1450<2123
 43495|  %2294 = load i64, ptr %2293, , !!8                                                                                    ;L26<1450<2123
 43496|  %2295 = gep %2254, i64 1192                                                                                           ;L26<1450<2123
 43497|  %2296 = load i64, ptr %2295, , !!8                                                                                    ;L26<1450<2123
 43498|  %2297 = gep %2254, i64 1480                                                                                           ;L26<1450<2123
 43499|  %2298 = load i64, ptr %2297, , !!8                                                                                    ;L26<1450<2123
 43500|  %2299 = add i64 %2298, -1                                                                                             ;L26<1450<2123
 43501|  %2300 = mul i64 %2299, %2296                                                                                          ;L26<1450<2123
 43502|  %2301 = gep %2254, i64 1080                                                                                           ;L26<1450<2123
 43503|  %2302 = load i64, ptr %2301, , !!8                                                                                    ;L26<1450<2123
 43505|  %2303 = gep %2254, i64 1632                                                                                           ;L1451<2123
 43506|  %2304 = load i64, ptr %2303, , !!8                                                                                    ;L1451<2123
 43507|     ;; x2 = i64 %2304
 43508|     ;; other = i64 %2304
 43509|  %2305 = gep %2254, i64 1640                                                                                           ;L1451<2123
 43510|  %2306 = load i64, ptr %2305, , !!8                                                                                    ;L1451<2123
 43511|     ;; y2 = i64 %2306
 43512|     ;; other = i64 %2306
 43513|  %2307 = icmp ult i64 %2269, %2304                                                                                     ;L3147<7<1451<2123
 43514|  %2308 = sub nuw i64 %2304, %2269                                                                                      ;L3147<7<1451<2123
 43515|  %2309 = sub nuw i64 %2269, %2304                                                                                      ;L3147<7<1451<2123
 43516|  %2310 = select i1 %2307, i64 %2308, i64 %2309                                                                         ;L3147<7<1451<2123
 43517|     ;; dx = i64 %2310
 43518|  %2311 = icmp ult i64 %2270, %2306                                                                                     ;L3147<8<1451<2123
 43519|  %2312 = sub nuw i64 %2306, %2270                                                                                      ;L3147<8<1451<2123
 43520|  %2313 = sub nuw i64 %2270, %2306                                                                                      ;L3147<8<1451<2123
 43521|  %2314 = select i1 %2311, i64 %2312, i64 %2313                                                                         ;L3147<8<1451<2123
 43522|     ;; dy = i64 %2314
 43523|  %2315 = mul i64 %2310, %2310                                                                                          ;L9<1451<2123
 43524|  %2316 = mul i64 %2314, %2314                                                                                          ;L9<1451<2123
 43525|  %2317 = add i64 %2316, %2315                                                                                          ;L9<1451<2123
 43526|     ;; dist = i64 %2317
 43527|  %2318 = gep %2254, i64 1136                                                                                           ;L1511<1452<2123
 43528|  %2319 = load i32, ptr %2318, , !!8                                                                                    ;L1511<1452<2123
 43529|     ;; mult = i32 %2319
 43530|  %2320 = icmp eq i32 %2319, 0                                                                                          ;L1512<1452<2123
 43531|  br i1 %2320, label %2321, label %2324                                                                                 ;L1512<1452<2123
 43532| 
 43533| 2321: ; preds = %2292
 43534|  %2322 = gep %2254, i64 1664                                                                                           ;L1513<1452<2123
 43535|  %2323 = load i64, ptr %2322, , !!8                                                                                    ;L1513<1452<2123
 43536|  br label %2331                                                                                                        ;L1512<1452<2123
 43537| 
 43538| 2324: ; preds = %2292
 43539|  %2325 = sext i32 %2319 to i64                                                                                         ;L1511<1452<2123
 43540|     ;; mult = i64 %2325
 43541|  %2326 = gep %2254, i64 1664                                                                                           ;L1515<1452<2123
 43542|  %2327 = load i64, ptr %2326, , !!8                                                                                    ;L1515<1452<2123
 43543|  %2328 = add nsw i64 %2325, 100                                                                                        ;L1515<1452<2123
 43544|  %2329 = mul i64 %2327, %2328                                                                                          ;L1515<1452<2123
 43545|  %2330 = udiv i64 %2329, 100                                                                                           ;L1515<1452<2123
 43546|  br label %2331                                                                                                        ;L1512<1452<2123
 43547| 
 43548| 2331: ; preds = %2324, %2321
 43549|  %2332 = phi i64 [ %2323, %2321 ], [ %2330, %2324 ]                                                                    ;L0<1452<2123
 43550|  %2333 = add i64 %2288, %2286                                                                                          ;L26<1450<2123
 43551|  %2334 = add i64 %2333, %2294                                                                                          ;L26<1450<2123
 43552|  %2335 = add i64 %2334, %2302                                                                                          ;L1452<2123
 43553|  %2336 = add i64 %2335, %2300                                                                                          ;L1452<2123
 43554|  %2337 = add i64 %2336, %2332                                                                                          ;L1452<2123
 43555|     ;; check = i64 %2337
 43556|  %2338 = mul i64 %2337, %2337                                                                                          ;L1453<2123
 43557|  %2339 = icmp ugt i64 %2317, %2338                                                                                     ;L1453<2123
 43558|  br i1 %2339, label %2348, label %2340                                                                                 ;L2123
 43559| 
 43560| 2340: ; preds = %2331
 43561|     ;; t = ptr %2254
 43562|  %2341 = gep %2254, i64 104                                                                                            ;L2129
 43563|  %2342 = load i64, ptr %2341, , !!8                                                                                    ;L2129
 43564|  %2343 = icmp eq i64 %2342, 2                                                                                          ;L2129
 43565|  br i1 %2343, label %2344, label %2348                                                                                 ;L2129
 43566| 
 43567| 2344: ; preds = %2340
 43568|     ;; info = ptr %2254
 43569|  %2345 = gep %2254, i64 136                                                                                            ;L2131
 43570|  %2346 = load i64, ptr %2345, , !!8                                                                                    ;L2131
 43571|  %2347 = trunc nuw i64 %2346 to i1                                                                                     ;L2131
 43572|  br i1 %2347, label %2349, label %2441                                                                                 ;L2131
 43573| 
 43574| 2348: ; preds = %2444, %2438, %2424, %2402, %2388, %2372, %2363, %2340, %2331, %2285, %2273, %2266
 43575|  br label %2249                                                                                                        ;L1714<180<2118
 43576| 
 43577| 2349: ; preds = %2344
 43578|  %2350 = gep %2254, i64 152                                                                                            ;L2131
 43579|  %2351 = load i64, ptr %2350, , !!8                                                                                    ;L2131
 43580|     ;; id = i64 %2351
 43581|  %2352 = icmp eq i64 %2351, %157                                                                                       ;L2132
 43582|  br i1 %2352, label %2435, label %2353                                                                                 ;L2132
 43583| 
 43584| 2353: ; preds = %2349
 43585|  %2354 = load ptr, ptr %372, , !!8                                                                                     ;L2134
 43586|  %2355 = invoke ptr %2354(ptr %177, i64 %2351)
 43587|  to label %2356 unwind label %365                                                                                      ;L2134
 43588| 
 43589| 2356: ; preds = %2353
 43590|  %2357 = icmp eq ptr %2355, null                                                                                       ;L2134
 43591|  br i1 %2357, label %2362, label %2358                                                                                 ;L2134
 43592| 
 43593| 2358: ; preds = %2356
 43594|     ;; other = ptr %2355
 43595|     ;; self = ptr %2355
 43596|  %2359 = gep %2355, i64 104                                                                                            ;L1404<2135
 43597|  %2360 = load i64, ptr %2359, , !!8                                                                                    ;L1404<2135
 43598|  %2361 = icmp eq i64 %2360, 13                                                                                         ;L2135
 43599|  br i1 %2361, label %2363, label %2366                                                                                 ;L2135
 43600| 
 43601| 2362: ; preds = %2356
 43604|  invoke void @gc::simulationNtB5_21AbstractGameWithCache12iter_minions(ptr sret([56 x i8]) %21, ptr %55, i64 %49)
 43605|  to label %2413 unwind label %365                                                                                      ;L2151
 43606| 
 43607| 2363: ; preds = %2358
 43608|  %2364 = call fastcc ptr @gc::simulationNtB5_21AbstractGameWithCache21player_by_champion_id(ptr %55, i64 %2351)        ;L2144
 43609|  %2365 = icmp eq ptr %2364, null                                                                                       ;L2144
 43610|  br i1 %2365, label %2348, label %2367                                                                                 ;L2144
 43611| 
 43612| 2366: ; preds = %2358
 43615|  invoke void @gc::simulationNtB5_21AbstractGameWithCache12iter_minions(ptr sret([56 x i8]) %23, ptr %55, i64 %49)
 43616|  to label %2391 unwind label %365                                                                                      ;L2136
 43617| 
 43618| 2367: ; preds = %2363
 43619|     ;; player = ptr %2364
 43620|  %2368 = gep %2364, i64 2352                                                                                           ;L2145
 43621|  %2369 = load i64, ptr %2368, , !!8                                                                                    ;L2145
 43622|  %2370 = icmp ult i64 %2369, 2                                                                                         ;L2145
 43623|  br i1 %2370, label %2372, label %2371                                                                                 ;L2145
 43624| 
 43625| 2371: ; preds = %2367
 43626|  invoke void @core::panicking18panic_bounds_check(i64 %2369, i64 2, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.152) #25
 43627|  to label %173 unwind label %365                                                                                       ;L2145
 43628| 
 43629| 2372: ; preds = %2367
 43630|     ;; self = ptr %2364
 43631|  %2373 = gep %2364, i64 2496                                                                                           ;L581<2145
 43632|  %2374 = load i32, ptr %2373, , !!8                                                                                    ;L581<2145
 43633|  %2375 = zext nneg i32 %2374 to i64                                                                                    ;L581<2145
 43634|  %2376 = getelementptr { { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, [5 x { i64, [2 x i64] }], [5 x { i64, { i8, [23 x i8] } }], [5 x { i8, [7 x i8], i64 }], [5 x i64], [5 x i64], [5 x i64], [5 x { i64, i64 }], i64, i64, i64, i64, i64, [5 x i32], i8, [3 x i8] }, ptr %167, i64 %2369 ;L2145
 43635|  %2377 = gep %2376, i64 120                                                                                            ;L2145
 43636|  %2378 = gepS %2377, i64 %2375                                                                                         ;L2145
 43637|  %2379 = load i64, ptr %2378,                                                                                          ;L2145
 43638|     ;; action[0..+8] = i64 %2379
 43639|     ;; self = ptr undef
 43641|  %2380 = icmp eq i64 %2379, 0                                                                                          ;L2439<2146
 43642|  br i1 %2380, label %2381, label %2348                                                                                 ;L2439<2146
 43643| 
 43644| 2381: ; preds = %2372
 43645|     ;; self = ptr %2254
 43646|  %2382 = load i32, ptr %2289, , !!8                                                                                    ;L742<2147
 43647|  %2383 = icmp eq i32 %2382, -1                                                                                         ;L742<2147
 43648|  br i1 %2383, label %2387, label %2384                                                                                 ;L742<2147
 43649| 
 43650| 2384: ; preds = %2381
 43651|  %2385 = gep %2254, i64 1168                                                                                           ;L742<2147
 43652|     ;; self = ptr %2385
 43653|  %2386 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2385, ptr %64, ptr %2254, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 43654|  to label %2388 unwind label %365                                                                                      ;L2147
 43655| 
 43656| 2387: ; preds = %2381
 43657|     ;; self = ptr null
 43658|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.153) #25
 43659|  to label %173 unwind label %365                                                                                       ;L1013<2147
 43660| 
 43661| 2388: ; preds = %2384
 43662|  %2389 = load i64, ptr %92, , !!8                                                                                      ;L2147
 43663|  %2390 = add i64 %2389, %2386                                                                                          ;L2147
 43664|  store i64 %2390, ptr %92,                                                                                             ;L2147
 43665|  br label %2348                                                                                                        ;L2146
 43666| 
 43667| 2391: ; preds = %2366
 43668|     ;; predicate = ptr %2254
 43669|  call void @llvm.memcpy.p0.p0.i64(ptr %24, ptr %23, i64 56, i1 false)                                                  ;L28<957<2137
 43670|  store ptr %2254, ptr %2243,                                                                                           ;L28<957<2137
 43672|     ;; self = ptr %24
 43673|     ;; default = i64 -1
 43674|     ;; init = i64 0
 43677|  invoke fastcc void @gc::simulation6entity6EntityEEB13_EB13_ENtNtNtB8_6traits8iterator8Iterator9size_hintCshdEBA0ozCnw_7game_ai(ptr %14, ptr %24)
 43678|  to label %2392 unwind label %365                                                                                      ;L141<2139
 43679| 
 43680| 2392: ; preds = %2391
 43681|  %2393 = load i64, ptr %2244, , !!57237, !!8                                                                           ;L141<2139
 43682|     ;; self[0..+8] = i64 %2393
 43684|  %2394 = load i64, ptr %2245, , !!57237                                                                                ;L1039<141<2139
 43688|  call void @llvm.memcpy.p0.p0.i64(ptr %13, ptr %24, i64 56, i1 false)                                                  ;L142<2139
 43689|     ;; self[56..+8] = ptr %2254
 43690|     ;; iter[56..+8] = ptr %2254
 43691|     ;; self[56..+8] = ptr %2254
 43692|  %2395 = invoke i64 @core::iter8adapters5chainINtB5_5ChainIBP_INtNtB7_6copied6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEEB14_EB14_ENtNtNtB9_6traits8iterator8Iterator4foldjNCINvNtB7_3map8map_foldB1Q_jjNCINvNvXs1_NtB7_6filterINtB4f_6FilterppEB2X_5count8to_usizeB1Q_NCNvNtCshdEBA0ozCnw_7game_ai15score_parameter25calculate_score_parametersv_0E0NCINvXsK_NtB31_5accumjNtB6u_3Sum3sumINtB3G_3MapBO_B44_EE0E0EB5b_(ptr %13, i64 0, ptr %2254)
 43693|  to label %2396 unwind label %365                                                                                      ;L128<52<3674<142<2139
 43694| 
 43695| 2396: ; preds = %2392
 43696|  %2397 = trunc nuw i64 %2393 to i1                                                                                     ;L1039<141<2139
 43697|     ;; total = i64 %2395
 43699|     ;; count = i64 %2395
 43701|  %2398 = icmp uge i64 %2394, %2395                                                                                     ;L251<145<2139
 43702|  %2399 = xor i1 %2397, true                                                                                            ;L251<145<2139
 43703|  %2400 = select i1 %2399, i1 true, i1 %2398                                                                            ;L251<145<2139
 43704|     ;; cond = i1 true
 43705|  call void @llvm.assume(i1 %2400)                                                                                      ;L210<251<145<2139
 43706|     ;; cnt = i64 %2395
 43707|  %2401 = icmp ult i64 %2395, 2                                                                                         ;L2141
 43708|  br i1 %2401, label %2403, label %2402                                                                                 ;L2141
 43709| 
 43710| 2402: ; preds = %2410, %2396
 43712|  br label %2348                                                                                                        ;L2135
 43713| 
 43714| 2403: ; preds = %2396
 43715|     ;; self = ptr %2254
 43716|  %2404 = load i32, ptr %2289, , !!8                                                                                    ;L742<2142
 43717|  %2405 = icmp eq i32 %2404, -1                                                                                         ;L742<2142
 43718|  br i1 %2405, label %2409, label %2406                                                                                 ;L742<2142
 43719| 
 43720| 2406: ; preds = %2403
 43721|  %2407 = gep %2254, i64 1168                                                                                           ;L742<2142
 43722|     ;; self = ptr %2407
 43723|  %2408 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2407, ptr %64, ptr %2254, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 43724|  to label %2410 unwind label %365                                                                                      ;L2142
 43725| 
 43726| 2409: ; preds = %2403
 43727|     ;; self = ptr null
 43728|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.154) #25
 43729|  to label %173 unwind label %365                                                                                       ;L1013<2142
 43730| 
 43731| 2410: ; preds = %2406
 43732|  %2411 = load i64, ptr %92, , !!8                                                                                      ;L2142
 43733|  %2412 = add i64 %2411, %2408                                                                                          ;L2142
 43734|  store i64 %2412, ptr %92,                                                                                             ;L2142
 43735|  br label %2402                                                                                                        ;L2141
 43736| 
 43737| 2413: ; preds = %2362
 43738|     ;; predicate = ptr %2254
 43739|  call void @llvm.memcpy.p0.p0.i64(ptr %22, ptr %21, i64 56, i1 false)                                                  ;L28<957<2152
 43740|  store ptr %2254, ptr %2246,                                                                                           ;L28<957<2152
 43742|     ;; self = ptr %22
 43743|     ;; default = i64 -1
 43744|     ;; init = i64 0
 43747|  invoke fastcc void @gc::simulation6entity6EntityEEB13_EB13_ENtNtNtB8_6traits8iterator8Iterator9size_hintCshdEBA0ozCnw_7game_ai(ptr %12, ptr %22)
 43748|  to label %2414 unwind label %365                                                                                      ;L141<2154
 43749| 
 43750| 2414: ; preds = %2413
 43751|  %2415 = load i64, ptr %2247, , !!57286, !!8                                                                           ;L141<2154
 43752|     ;; self[0..+8] = i64 %2415
 43754|  %2416 = load i64, ptr %2248, , !!57286                                                                                ;L1039<141<2154
 43758|  call void @llvm.memcpy.p0.p0.i64(ptr %11, ptr %22, i64 56, i1 false)                                                  ;L142<2154
 43759|     ;; self[56..+8] = ptr %2254
 43760|     ;; iter[56..+8] = ptr %2254
 43761|     ;; self[56..+8] = ptr %2254
 43762|  %2417 = invoke i64 @core::iter8adapters5chainINtB5_5ChainIBP_INtNtB7_6copied6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEEB14_EB14_ENtNtNtB9_6traits8iterator8Iterator4foldjNCINvNtB7_3map8map_foldB1Q_jjNCINvNvXs1_NtB7_6filterINtB4f_6FilterppEB2X_5count8to_usizeB1Q_NCNvNtCshdEBA0ozCnw_7game_ai15score_parameter25calculate_score_parametersw_0E0NCINvXsK_NtB31_5accumjNtB6u_3Sum3sumINtB3G_3MapBO_B44_EE0E0EB5b_(ptr %11, i64 0, ptr %2254)
 43763|  to label %2418 unwind label %365                                                                                      ;L128<52<3674<142<2154
 43764| 
 43765| 2418: ; preds = %2414
 43766|  %2419 = trunc nuw i64 %2415 to i1                                                                                     ;L1039<141<2154
 43767|     ;; total = i64 %2417
 43769|     ;; count = i64 %2417
 43771|  %2420 = icmp uge i64 %2416, %2417                                                                                     ;L251<145<2154
 43772|  %2421 = xor i1 %2419, true                                                                                            ;L251<145<2154
 43773|  %2422 = select i1 %2421, i1 true, i1 %2420                                                                            ;L251<145<2154
 43774|     ;; cond = i1 true
 43775|  call void @llvm.assume(i1 %2422)                                                                                      ;L210<251<145<2154
 43776|     ;; cnt = i64 %2417
 43777|  %2423 = icmp ult i64 %2417, 2                                                                                         ;L2156
 43778|  br i1 %2423, label %2425, label %2424                                                                                 ;L2156
 43779| 
 43780| 2424: ; preds = %2432, %2418
 43782|  br label %2348                                                                                                        ;L2134
 43783| 
 43784| 2425: ; preds = %2418
 43785|     ;; self = ptr %2254
 43786|  %2426 = load i32, ptr %2289, , !!8                                                                                    ;L742<2157
 43787|  %2427 = icmp eq i32 %2426, -1                                                                                         ;L742<2157
 43788|  br i1 %2427, label %2431, label %2428                                                                                 ;L742<2157
 43789| 
 43790| 2428: ; preds = %2425
 43791|  %2429 = gep %2254, i64 1168                                                                                           ;L742<2157
 43792|     ;; self = ptr %2429
 43793|  %2430 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2429, ptr %64, ptr %2254, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 43794|  to label %2432 unwind label %365                                                                                      ;L2157
 43795| 
 43796| 2431: ; preds = %2425
 43797|     ;; self = ptr null
 43798|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.155) #25
 43799|  to label %173 unwind label %365                                                                                       ;L1013<2157
 43800| 
 43801| 2432: ; preds = %2428
 43802|  %2433 = load i64, ptr %92, , !!8                                                                                      ;L2157
 43803|  %2434 = add i64 %2433, %2430                                                                                          ;L2157
 43804|  store i64 %2434, ptr %92,                                                                                             ;L2157
 43805|  br label %2424                                                                                                        ;L2156
 43806| 
 43807| 2435: ; preds = %2349
 43808|     ;; self = ptr %2254
 43809|  %2436 = gep %2254, i64 1168                                                                                           ;L742<2133
 43810|     ;; self = ptr %2436
 43811|  %2437 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2436, ptr %64, ptr %2254, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 43812|  to label %2438 unwind label %365                                                                                      ;L2133
 43813| 
 43814| 2438: ; preds = %2435
 43815|  %2439 = load i64, ptr %89, , !!8                                                                                      ;L2133
 43816|  %2440 = add i64 %2439, %2437                                                                                          ;L2133
 43817|  store i64 %2440, ptr %89,                                                                                             ;L2133
 43818|  br label %2348                                                                                                        ;L2132
 43819| 
 43820| 2441: ; preds = %2344
 43821|     ;; self = ptr %2254
 43822|  %2442 = gep %2254, i64 1168                                                                                           ;L742<2161
 43823|     ;; self = ptr %2442
 43824|  %2443 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2442, ptr %64, ptr %2254, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 43825|  to label %2444 unwind label %365                                                                                      ;L2161
 43826| 
 43827| 2444: ; preds = %2441
 43828|  %2445 = load i64, ptr %92, , !!8                                                                                      ;L2161
 43829|  %2446 = add i64 %2445, %2443                                                                                          ;L2161
 43830|  store i64 %2446, ptr %92,                                                                                             ;L2161
 43831|  br label %2348                                                                                                        ;L2131
 43832| 
 43833| 2447: ; preds = %2478, %2259
 43834|  %2448 = phi ptr [ %2260, %2259 ], [ %2451, %2478 ]                                                                    ;L2170
 43835|     ;; iter[0..+8] = ptr %2448
 43836|     ;; self = ptr undef
 43837|     ;; ptr = ptr %2448
 43838|     ;; self = ptr %2448
 43839|     ;; end_or_len = ptr %2262
 43842|  %2449 = icmp eq ptr %2448, %2262                                                                                      ;L1714<180<2170
 43843|  br i1 %2449, label %2456, label %2450                                                                                 ;L180<2170
 43844| 
 43845| 2450: ; preds = %2447
 43846|  %2451 = gep %2448, i64 216                                                                                            ;L656<185<2170
 43847|     ;; iter[0..+8] = ptr %2451
 43848|     ;; p = ptr %2448
 43849|  %2452 = gep %2448, i64 88                                                                                             ;L2171
 43850|  %2453 = load i64, ptr %2452, , !!8                                                                                    ;L2171
 43851|  %2454 = load ptr, ptr %372, , !!8                                                                                     ;L2171
 43852|  %2455 = invoke ptr %2454(ptr %177, i64 %2453)
 43853|  to label %2463 unwind label %365                                                                                      ;L2171
 43854| 
 43855| 2456: ; preds = %2447
 43856|     ;; self = ptr %47
 43857|     ;; self = ptr %47
 43858|  %2457 = load ptr, ptr %67, , !!8, !!8                                                                                 ;L138<2083<2217
 43859|     ;; ptr = ptr %2457
 43860|  %2458 = load i64, ptr %70, , !!8                                                                                      ;L2085<2217
 43861|     ;; len = i64 %2458
 43862|     ;; count = i64 %2458
 43863|     ;; self[0..+8] = ptr %2457
 43864|     ;; slice[0..+8] = ptr %2457
 43865|     ;; self[8..+8] = i64 %2458
 43866|     ;; slice[8..+8] = i64 %2458
 43867|     ;; ptr = ptr %2457
 43868|     ;; self = ptr %2457
 43869|  %2459 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %2457, i64 %2458 ;L961<240<1062<2217
 43870|     ;; iter[0..+8] = ptr %2457
 43871|     ;; iter[8..+8] = ptr %2459
 43872|  %2460 = gep %18, i64 56
 43873|  %2461 = gep %8, i64 8
 43874|  %2462 = gep %8, i64 16
 43875|  br label %2638                                                                                                        ;L2217
 43876| 
 43877| 2463: ; preds = %2450
 43878|  %2464 = icmp eq ptr %2455, null                                                                                       ;L2171
 43879|  br i1 %2464, label %2478, label %2465                                                                                 ;L2171
 43880| 
 43881| 2465: ; preds = %2463
 43882|     ;; champ = ptr %2455
 43883|     ;; self = ptr %2455
 43884|     ;; self = ptr %31
 43885|     ;; self = ptr %31
 43886|     ;; self = ptr %31
 43887|  %2466 = load ptr, ptr %31, , !!8, !!8                                                                                 ;L138<2073<2136<2172
 43888|     ;; p = ptr %2466
 43889|  %2467 = load i64, ptr %2238, , !!8                                                                                    ;L2075<2136<2172
 43890|     ;; len = i64 %2467
 43891|     ;; count = i64 %2467
 43892|     ;; self[0..+8] = ptr %2466
 43893|     ;; slice[0..+8] = ptr %2466
 43894|     ;; self[8..+8] = i64 %2467
 43895|     ;; slice[8..+8] = i64 %2467
 43896|     ;; ptr = ptr %2466
 43897|     ;; self = ptr %2466
 43898|  %2468 = getelementptr ptr, ptr %2466, i64 %2467                                                                       ;L961<100<1042<2136<2172
 43899|     ;; iter[0..+8] = ptr %2466
 43900|     ;; iter[8..+8] = ptr %2468
 43901|  %2469 = gep %2455, i64 8
 43902|  %2470 = gep %2455, i64 1632
 43903|  %2471 = gep %2455, i64 1640
 43904|  %2472 = gep %2455, i64 1136
 43905|  %2473 = gep %2455, i64 1664
 43906|  %2474 = gep %2455, i64 1600
 43907|  %2475 = gep %2448, i64 152
 43908|  %2476 = gep %2455, i64 1472
 43909|  %2477 = gep %2448, i64 128
 43910|  br label %2479                                                                                                        ;L2172
 43911| 
 43912| 2478: ; preds = %2479, %2463
 43913|  br label %2447                                                                                                        ;L2170
 43914| 
 43915| 2479: ; preds = %2571, %2465
 43916|  %2480 = phi ptr [ %2466, %2465 ], [ %2483, %2571 ]                                                                    ;L2172
 43917|     ;; iter[0..+8] = ptr %2480
 43918|     ;; self = ptr undef
 43919|     ;; ptr = ptr %2480
 43920|     ;; self = ptr %2480
 43921|     ;; end_or_len = ptr %2468
 43924|  %2481 = icmp eq ptr %2480, %2468                                                                                      ;L1714<180<2172
 43925|  br i1 %2481, label %2478, label %2482                                                                                 ;L180<2172
 43926| 
 43927| 2482: ; preds = %2479
 43928|  %2483 = gep %2480, i64 8                                                                                              ;L656<185<2172
 43929|     ;; iter[0..+8] = ptr %2483
 43930|     ;; t = ptr %2480
 43931|  %2484 = load ptr, ptr %2480, , !!8, !!8                                                                               ;L2173
 43932|     ;; self = ptr %2484
 43933|     ;; other = ptr %2455
 43934|  %2485 = load i64, ptr %2484, , !!8                                                                                    ;L1127<2173
 43935|  %2486 = gep %2484, i64 8                                                                                              ;L1127<2173
 43936|     ;; __self_discr = i64 %2485
 43937|  %2487 = load i64, ptr %2455, , !!8                                                                                    ;L1127<2173
 43938|     ;; __arg1_discr = i64 %2487
 43939|  %2488 = icmp eq i64 %2485, %2487                                                                                      ;L1127<2173
 43940|  br i1 %2488, label %2489, label %2491                                                                                 ;L1127<2173
 43941| 
 43942| 2489: ; preds = %2482
 43943|  %2490 = icmp eq i64 %2485, 0                                                                                          ;L1127<2173
 43944|  br i1 %2490, label %2496, label %2571                                                                                 ;L1127<2173
 43945| 
 43946| 2491: ; preds = %2496, %2482
 43947|  %2492 = load i64, ptr %2470, , !!8                                                                                    ;L2177
 43948|  %2493 = load i64, ptr %2471, , !!8                                                                                    ;L2177
 43949|  %2494 = load i32, ptr %2472, , !!8                                                                                    ;L1511<2177
 43950|     ;; mult = i32 %2494
 43951|  %2495 = icmp eq i32 %2494, 0                                                                                          ;L1512<2177
 43952|  br i1 %2495, label %2500, label %2502                                                                                 ;L1512<2177
 43953| 
 43954| 2496: ; preds = %2489
 43955|     ;; __self_0 = ptr %2484
 43956|     ;; self = ptr %2484
 43957|     ;; __arg1_0 = ptr %2455
 43958|     ;; other = ptr %2455
 43961|  %2497 = load i64, ptr %2486, , !!8                                                                                    ;L1878<2123<1127<2173
 43962|  %2498 = load i64, ptr %2469, , !!8                                                                                    ;L1878<2123<1127<2173
 43963|  %2499 = icmp eq i64 %2497, %2498                                                                                      ;L1878<2123<1127<2173
 43964|  br i1 %2499, label %2571, label %2491                                                                                 ;L2173
 43965| 
 43966| 2500: ; preds = %2491
 43967|  %2501 = load i64, ptr %2473, , !!8                                                                                    ;L1513<2177
 43968|  br label %2508                                                                                                        ;L1512<2177
 43969| 
 43970| 2502: ; preds = %2491
 43971|  %2503 = sext i32 %2494 to i64                                                                                         ;L1511<2177
 43972|     ;; mult = i64 %2503
 43973|  %2504 = load i64, ptr %2473, , !!8                                                                                    ;L1515<2177
 43974|  %2505 = add nsw i64 %2503, 100                                                                                        ;L1515<2177
 43975|  %2506 = mul i64 %2504, %2505                                                                                          ;L1515<2177
 43976|  %2507 = udiv i64 %2506, 100                                                                                           ;L1515<2177
 43977|  br label %2508                                                                                                        ;L1512<2177
 43978| 
 43979| 2508: ; preds = %2502, %2500
 43980|  %2509 = phi i64 [ %2501, %2500 ], [ %2507, %2502 ]                                                                    ;L0<2177
 43981|  %2510 = load i64, ptr %2474, , !!8                                                                                    ;L2177
 43982|  %2511 = mul i64 %2510, 30                                                                                             ;L2177
 43983|     ;; tower = ptr %2484
 43984|     ;; caster = ptr %2484
 43985|     ;; self = ptr %2484
 43986|     ;; x = i64 %2492
 43987|     ;; x1 = i64 %2492
 43988|     ;; self = i64 %2492
 43989|     ;; y = i64 %2493
 43990|     ;; y1 = i64 %2493
 43991|     ;; self = i64 %2493
 43992|     ;; r = i64 %2509
 43993|     ;; d = i64 %2511
 43994|  %2512 = gep %2484, i64 1216                                                                                           ;L1449<2177
 43995|  %2513 = load i32, ptr %2512, , !!8                                                                                    ;L1449<2177
 43996|  %2514 = icmp eq i32 %2513, -1                                                                                         ;L1449<2177
 43997|  br i1 %2514, label %2571, label %2515                                                                                 ;L1449<2177
 43998| 
 43999| 2515: ; preds = %2508
 44000|     ;; attack = ptr %2484
 44001|     ;; self = ptr %2484
 44002|  %2516 = gep %2484, i64 1184                                                                                           ;L26<1450<2177
 44003|  %2517 = load i64, ptr %2516, , !!8                                                                                    ;L26<1450<2177
 44004|  %2518 = gep %2484, i64 1192                                                                                           ;L26<1450<2177
 44005|  %2519 = load i64, ptr %2518, , !!8                                                                                    ;L26<1450<2177
 44006|  %2520 = gep %2484, i64 1480                                                                                           ;L26<1450<2177
 44007|  %2521 = load i64, ptr %2520, , !!8                                                                                    ;L26<1450<2177
 44008|  %2522 = add i64 %2521, -1                                                                                             ;L26<1450<2177
 44009|  %2523 = mul i64 %2522, %2519                                                                                          ;L26<1450<2177
 44010|  %2524 = gep %2484, i64 1080                                                                                           ;L26<1450<2177
 44011|  %2525 = load i64, ptr %2524, , !!8                                                                                    ;L26<1450<2177
 44013|  %2526 = gep %2484, i64 1632                                                                                           ;L1451<2177
 44014|  %2527 = load i64, ptr %2526, , !!8                                                                                    ;L1451<2177
 44015|     ;; x2 = i64 %2527
 44016|     ;; other = i64 %2527
 44017|  %2528 = gep %2484, i64 1640                                                                                           ;L1451<2177
 44018|  %2529 = load i64, ptr %2528, , !!8                                                                                    ;L1451<2177
 44019|     ;; y2 = i64 %2529
 44020|     ;; other = i64 %2529
 44021|  %2530 = icmp ult i64 %2492, %2527                                                                                     ;L3147<7<1451<2177
 44022|  %2531 = sub nuw i64 %2527, %2492                                                                                      ;L3147<7<1451<2177
 44023|  %2532 = sub nuw i64 %2492, %2527                                                                                      ;L3147<7<1451<2177
 44024|  %2533 = select i1 %2530, i64 %2531, i64 %2532                                                                         ;L3147<7<1451<2177
 44025|     ;; dx = i64 %2533
 44026|  %2534 = icmp ult i64 %2493, %2529                                                                                     ;L3147<8<1451<2177
 44027|  %2535 = sub nuw i64 %2529, %2493                                                                                      ;L3147<8<1451<2177
 44028|  %2536 = sub nuw i64 %2493, %2529                                                                                      ;L3147<8<1451<2177
 44029|  %2537 = select i1 %2534, i64 %2535, i64 %2536                                                                         ;L3147<8<1451<2177
 44030|     ;; dy = i64 %2537
 44031|  %2538 = mul i64 %2533, %2533                                                                                          ;L9<1451<2177
 44032|  %2539 = mul i64 %2537, %2537                                                                                          ;L9<1451<2177
 44033|  %2540 = add i64 %2539, %2538                                                                                          ;L9<1451<2177
 44034|     ;; dist = i64 %2540
 44035|  %2541 = gep %2484, i64 1136                                                                                           ;L1511<1452<2177
 44036|  %2542 = load i32, ptr %2541, , !!8                                                                                    ;L1511<1452<2177
 44037|     ;; mult = i32 %2542
 44038|  %2543 = icmp eq i32 %2542, 0                                                                                          ;L1512<1452<2177
 44039|  br i1 %2543, label %2544, label %2547                                                                                 ;L1512<1452<2177
 44040| 
 44041| 2544: ; preds = %2515
 44042|  %2545 = gep %2484, i64 1664                                                                                           ;L1513<1452<2177
 44043|  %2546 = load i64, ptr %2545, , !!8                                                                                    ;L1513<1452<2177
 44044|  br label %2554                                                                                                        ;L1512<1452<2177
 44045| 
 44046| 2547: ; preds = %2515
 44047|  %2548 = sext i32 %2542 to i64                                                                                         ;L1511<1452<2177
 44048|     ;; mult = i64 %2548
 44049|  %2549 = gep %2484, i64 1664                                                                                           ;L1515<1452<2177
 44050|  %2550 = load i64, ptr %2549, , !!8                                                                                    ;L1515<1452<2177
 44051|  %2551 = add nsw i64 %2548, 100                                                                                        ;L1515<1452<2177
 44052|  %2552 = mul i64 %2550, %2551                                                                                          ;L1515<1452<2177
 44053|  %2553 = udiv i64 %2552, 100                                                                                           ;L1515<1452<2177
 44054|  br label %2554                                                                                                        ;L1512<1452<2177
 44055| 
 44056| 2554: ; preds = %2547, %2544
 44057|  %2555 = phi i64 [ %2546, %2544 ], [ %2553, %2547 ]                                                                    ;L0<1452<2177
 44058|  %2556 = add i64 %2511, %2509                                                                                          ;L26<1450<2177
 44059|  %2557 = add i64 %2556, %2517                                                                                          ;L26<1450<2177
 44060|  %2558 = add i64 %2557, %2525                                                                                          ;L1452<2177
 44061|  %2559 = add i64 %2558, %2523                                                                                          ;L1452<2177
 44062|  %2560 = add i64 %2559, %2555                                                                                          ;L1452<2177
 44063|     ;; check = i64 %2560
 44064|  %2561 = mul i64 %2560, %2560                                                                                          ;L1453<2177
 44065|  %2562 = icmp ugt i64 %2540, %2561                                                                                     ;L1453<2177
 44066|  br i1 %2562, label %2571, label %2563                                                                                 ;L2177
 44067| 
 44068| 2563: ; preds = %2554
 44069|     ;; t = ptr %2484
 44070|  %2564 = gep %2484, i64 104                                                                                            ;L2183
 44071|  %2565 = load i64, ptr %2564, , !!8                                                                                    ;L2183
 44072|  %2566 = icmp eq i64 %2565, 2                                                                                          ;L2183
 44073|  br i1 %2566, label %2567, label %2571                                                                                 ;L2183
 44074| 
 44075| 2567: ; preds = %2563
 44076|     ;; info = ptr %2484
 44077|  %2568 = gep %2484, i64 136                                                                                            ;L2185
 44078|  %2569 = load i64, ptr %2568, , !!8                                                                                    ;L2185
 44079|  %2570 = trunc nuw i64 %2569 to i1                                                                                     ;L2185
 44080|  br i1 %2570, label %2572, label %2632                                                                                 ;L2185
 44081| 
 44082| 2571: ; preds = %2635, %2629, %2618, %2593, %2581, %2563, %2554, %2508, %2496, %2489
 44083|  br label %2479                                                                                                        ;L1714<180<2172
 44084| 
 44085| 2572: ; preds = %2567
 44086|  %2573 = gep %2484, i64 152                                                                                            ;L2185
 44087|  %2574 = load i64, ptr %2573, , !!8                                                                                    ;L2185
 44088|     ;; id = i64 %2574
 44089|  %2575 = load i64, ptr %2476, , !!8                                                                                    ;L2186
 44090|  %2576 = icmp eq i64 %2574, %2575                                                                                      ;L2186
 44091|  br i1 %2576, label %2626, label %2577                                                                                 ;L2186
 44092| 
 44093| 2577: ; preds = %2572
 44094|  %2578 = invoke ptr %2454(ptr %177, i64 %2574)
 44095|  to label %2579 unwind label %365                                                                                      ;L2188
 44096| 
 44097| 2579: ; preds = %2577
 44098|  %2580 = icmp eq ptr %2578, null                                                                                       ;L2188
 44099|  br i1 %2580, label %2585, label %2581                                                                                 ;L2188
 44100| 
 44101| 2581: ; preds = %2579
 44102|     ;; other = ptr %2578
 44103|     ;; self = ptr %2578
 44104|  %2582 = gep %2578, i64 104                                                                                            ;L1404<2189
 44105|  %2583 = load i64, ptr %2582, , !!8                                                                                    ;L1404<2189
 44106|  %2584 = icmp eq i64 %2583, 13                                                                                         ;L2189
 44107|  br i1 %2584, label %2571, label %2586                                                                                 ;L2189
 44108| 
 44109| 2585: ; preds = %2579
 44112|  invoke void @gc::simulationNtB5_21AbstractGameWithCache12iter_minions(ptr sret([56 x i8]) %19, ptr %55, i64 %144)
 44113|  to label %2596 unwind label %365                                                                                      ;L2193
 44114| 
 44115| 2586: ; preds = %2581
 44116|     ;; self = ptr %2484
 44117|  %2587 = load i32, ptr %2512, , !!8                                                                                    ;L742<2190
 44118|  %2588 = icmp eq i32 %2587, -1                                                                                         ;L742<2190
 44119|  br i1 %2588, label %2592, label %2589                                                                                 ;L742<2190
 44120| 
 44121| 2589: ; preds = %2586
 44122|  %2590 = gep %2484, i64 1168                                                                                           ;L742<2190
 44123|     ;; self = ptr %2590
 44124|  %2591 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2590, ptr %64, ptr %2484, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %2455)
 44125|  to label %2593 unwind label %365                                                                                      ;L2190
 44126| 
 44127| 2592: ; preds = %2586
 44128|     ;; self = ptr null
 44129|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.156) #25
 44130|  to label %173 unwind label %365                                                                                       ;L1013<2190
 44131| 
 44132| 2593: ; preds = %2589
 44133|  %2594 = load i64, ptr %2475, , !!8                                                                                    ;L2190
 44134|  %2595 = add i64 %2594, %2591                                                                                          ;L2190
 44135|  store i64 %2595, ptr %2475,                                                                                           ;L2190
 44136|  br label %2571                                                                                                        ;L2189
 44137| 
 44138| 2596: ; preds = %2585
 44139|     ;; predicate = ptr %2484
 44140|  call void @llvm.memcpy.p0.p0.i64(ptr %20, ptr %19, i64 56, i1 false)                                                  ;L28<957<2194
 44141|  store ptr %2484, ptr %2263,                                                                                           ;L28<957<2194
 44143|     ;; self = ptr %20
 44144|     ;; default = i64 -1
 44145|     ;; init = i64 0
 44148|  invoke fastcc void @gc::simulation6entity6EntityEEB13_EB13_ENtNtNtB8_6traits8iterator8Iterator9size_hintCshdEBA0ozCnw_7game_ai(ptr %10, ptr %20)
 44149|  to label %2597 unwind label %365                                                                                      ;L141<2196
 44150| 
 44151| 2597: ; preds = %2596
 44152|  %2598 = load i64, ptr %2264, , !!57469, !!8                                                                           ;L141<2196
 44153|     ;; self[0..+8] = i64 %2598
 44155|  %2599 = load i64, ptr %2265, , !!57469                                                                                ;L1039<141<2196
 44159|  call void @llvm.memcpy.p0.p0.i64(ptr %9, ptr %20, i64 56, i1 false)                                                   ;L142<2196
 44160|     ;; self[56..+8] = ptr %2484
 44161|     ;; iter[56..+8] = ptr %2484
 44162|     ;; self[56..+8] = ptr %2484
 44163|  %2600 = invoke i64 @core::iter8adapters5chainINtB5_5ChainIBP_INtNtB7_6copied6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEEB14_EB14_ENtNtNtB9_6traits8iterator8Iterator4foldjNCINvNtB7_3map8map_foldB1Q_jjNCINvNvXs1_NtB7_6filterINtB4f_6FilterppEB2X_5count8to_usizeB1Q_NCNvNtCshdEBA0ozCnw_7game_ai15score_parameter25calculate_score_parametersx_0E0NCINvXsK_NtB31_5accumjNtB6u_3Sum3sumINtB3G_3MapBO_B44_EE0E0EB5b_(ptr %9, i64 0, ptr %2484)
 44164|  to label %2601 unwind label %365                                                                                      ;L128<52<3674<142<2196
 44165| 
 44166| 2601: ; preds = %2597
 44167|  %2602 = trunc nuw i64 %2598 to i1                                                                                     ;L1039<141<2196
 44168|     ;; total = i64 %2600
 44170|     ;; count = i64 %2600
 44172|  %2603 = icmp uge i64 %2599, %2600                                                                                     ;L251<145<2196
 44173|  %2604 = xor i1 %2602, true                                                                                            ;L251<145<2196
 44174|  %2605 = select i1 %2604, i1 true, i1 %2603                                                                            ;L251<145<2196
 44175|     ;; cond = i1 true
 44176|  call void @llvm.assume(i1 %2605)                                                                                      ;L210<251<145<2196
 44177|     ;; cnt = i64 %2600
 44178|  %2606 = icmp ult i64 %2600, 3                                                                                         ;L2199
 44179|     ;; self = ptr %2484
 44180|     ;; self = ptr %2484
 44181|  %2607 = load i32, ptr %2512, , !!8                                                                                    ;L742<0
 44182|  %2608 = icmp eq i32 %2607, -1                                                                                         ;L742<0
 44183|  br i1 %2606, label %2610, label %2609                                                                                 ;L2199
 44184| 
 44185| 2609: ; preds = %2601
 44186|  br i1 %2608, label %2614, label %2611                                                                                 ;L742<2202
 44187| 
 44188| 2610: ; preds = %2601
 44189|  br i1 %2608, label %2622, label %2619                                                                                 ;L742<2200
 44190| 
 44191| 2611: ; preds = %2609
 44192|  %2612 = gep %2484, i64 1168                                                                                           ;L742<2202
 44193|     ;; self = ptr %2612
 44194|  %2613 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2612, ptr %64, ptr %2484, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %2455)
 44195|  to label %2615 unwind label %365                                                                                      ;L2202
 44196| 
 44197| 2614: ; preds = %2609
 44198|     ;; self = ptr null
 44199|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.157) #25
 44200|  to label %173 unwind label %365                                                                                       ;L1013<2202
 44201| 
 44202| 2615: ; preds = %2611
 44203|  %2616 = load i64, ptr %2475, , !!8                                                                                    ;L2202
 44204|  %2617 = add i64 %2616, %2613                                                                                          ;L2202
 44205|  store i64 %2617, ptr %2475,                                                                                           ;L2202
 44206|  br label %2618                                                                                                        ;L2199
 44207| 
 44208| 2618: ; preds = %2623, %2615
 44210|  br label %2571                                                                                                        ;L2188
 44211| 
 44212| 2619: ; preds = %2610
 44213|  %2620 = gep %2484, i64 1168                                                                                           ;L742<2200
 44214|     ;; self = ptr %2620
 44215|  %2621 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2620, ptr %64, ptr %2484, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %2455)
 44216|  to label %2623 unwind label %365                                                                                      ;L2200
 44217| 
 44218| 2622: ; preds = %2610
 44219|     ;; self = ptr null
 44220|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.158) #25
 44221|  to label %173 unwind label %365                                                                                       ;L1013<2200
 44222| 
 44223| 2623: ; preds = %2619
 44224|  %2624 = load i64, ptr %2477, , !!8                                                                                    ;L2200
 44225|  %2625 = add i64 %2624, %2621                                                                                          ;L2200
 44226|  store i64 %2625, ptr %2477,                                                                                           ;L2200
 44227|  br label %2618                                                                                                        ;L2199
 44228| 
 44229| 2626: ; preds = %2572
 44230|     ;; self = ptr %2484
 44231|  %2627 = gep %2484, i64 1168                                                                                           ;L742<2187
 44232|     ;; self = ptr %2627
 44233|  %2628 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2627, ptr %64, ptr %2484, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %2455)
 44234|  to label %2629 unwind label %365                                                                                      ;L2187
 44235| 
 44236| 2629: ; preds = %2626
 44237|  %2630 = load i64, ptr %2477, , !!8                                                                                    ;L2187
 44238|  %2631 = add i64 %2630, %2628                                                                                          ;L2187
 44239|  store i64 %2631, ptr %2477,                                                                                           ;L2187
 44240|  br label %2571                                                                                                        ;L2186
 44241| 
 44242| 2632: ; preds = %2567
 44243|     ;; self = ptr %2484
 44244|  %2633 = gep %2484, i64 1168                                                                                           ;L742<2206
 44245|     ;; self = ptr %2633
 44246|  %2634 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2633, ptr %64, ptr %2484, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %2455)
 44247|  to label %2635 unwind label %365                                                                                      ;L2206
 44248| 
 44249| 2635: ; preds = %2632
 44250|  %2636 = load i64, ptr %2475, , !!8                                                                                    ;L2206
 44251|  %2637 = add i64 %2636, %2634                                                                                          ;L2206
 44252|  store i64 %2637, ptr %2475,                                                                                           ;L2206
 44253|  br label %2571                                                                                                        ;L2185
 44254| 
 44255| 2638: ; preds = %2662, %2456
 44256|  %2639 = phi ptr [ %2457, %2456 ], [ %2642, %2662 ]                                                                    ;L2217
 44257|     ;; iter[0..+8] = ptr %2639
 44258|     ;; self = ptr undef
 44259|     ;; ptr = ptr %2639
 44260|     ;; self = ptr %2639
 44261|     ;; end_or_len = ptr %2459
 44264|  %2640 = icmp eq ptr %2639, %2459                                                                                      ;L1714<180<2217
 44265|  br i1 %2640, label %2822, label %2641                                                                                 ;L180<2217
 44266| 
 44267| 2641: ; preds = %2638
 44268|  %2642 = gep %2639, i64 216                                                                                            ;L656<185<2217
 44269|     ;; iter[0..+8] = ptr %2642
 44270|     ;; p = ptr %2639
 44271|  %2643 = gep %2639, i64 88                                                                                             ;L2218
 44272|  %2644 = load i64, ptr %2643, , !!8                                                                                    ;L2218
 44273|  %2645 = load ptr, ptr %372, , !!8                                                                                     ;L2218
 44274|  %2646 = invoke ptr %2645(ptr %177, i64 %2644)
 44275|  to label %2647 unwind label %365                                                                                      ;L2218
 44276| 
 44277| 2647: ; preds = %2641
 44278|  %2648 = icmp eq ptr %2646, null                                                                                       ;L2218
 44279|  br i1 %2648, label %2662, label %2649                                                                                 ;L2218
 44280| 
 44281| 2649: ; preds = %2647
 44282|     ;; champ = ptr %2646
 44283|     ;; self = ptr %2646
 44284|     ;; self = ptr %31
 44285|     ;; self = ptr %31
 44286|     ;; self = ptr %31
 44287|  %2650 = load ptr, ptr %31, , !!8, !!8                                                                                 ;L138<2073<2136<2219
 44288|     ;; p = ptr %2650
 44289|  %2651 = load i64, ptr %2238, , !!8                                                                                    ;L2075<2136<2219
 44290|     ;; len = i64 %2651
 44291|     ;; count = i64 %2651
 44292|     ;; self[0..+8] = ptr %2650
 44293|     ;; slice[0..+8] = ptr %2650
 44294|     ;; self[8..+8] = i64 %2651
 44295|     ;; slice[8..+8] = i64 %2651
 44296|     ;; ptr = ptr %2650
 44297|     ;; self = ptr %2650
 44298|  %2652 = getelementptr ptr, ptr %2650, i64 %2651                                                                       ;L961<100<1042<2136<2219
 44299|     ;; iter[0..+8] = ptr %2650
 44300|     ;; iter[8..+8] = ptr %2652
 44301|  %2653 = gep %2646, i64 8
 44302|  %2654 = gep %2646, i64 1632
 44303|  %2655 = gep %2646, i64 1640
 44304|  %2656 = gep %2646, i64 1136
 44305|  %2657 = gep %2646, i64 1664
 44306|  %2658 = gep %2646, i64 1600
 44307|  %2659 = gep %2639, i64 152
 44308|  %2660 = gep %2646, i64 1472
 44309|  %2661 = gep %2639, i64 128
 44310|  br label %2663                                                                                                        ;L2219
 44311| 
 44312| 2662: ; preds = %2663, %2647
 44313|  br label %2638                                                                                                        ;L2217
 44314| 
 44315| 2663: ; preds = %2755, %2649
 44316|  %2664 = phi ptr [ %2650, %2649 ], [ %2667, %2755 ]                                                                    ;L2219
 44317|     ;; iter[0..+8] = ptr %2664
 44318|     ;; self = ptr undef
 44319|     ;; ptr = ptr %2664
 44320|     ;; self = ptr %2664
 44321|     ;; end_or_len = ptr %2652
 44324|  %2665 = icmp eq ptr %2664, %2652                                                                                      ;L1714<180<2219
 44325|  br i1 %2665, label %2662, label %2666                                                                                 ;L180<2219
 44326| 
 44327| 2666: ; preds = %2663
 44328|  %2667 = gep %2664, i64 8                                                                                              ;L656<185<2219
 44329|     ;; iter[0..+8] = ptr %2667
 44330|     ;; t = ptr %2664
 44331|  %2668 = load ptr, ptr %2664, , !!8, !!8                                                                               ;L2220
 44332|     ;; self = ptr %2668
 44333|     ;; other = ptr %2646
 44334|  %2669 = load i64, ptr %2668, , !!8                                                                                    ;L1127<2220
 44335|  %2670 = gep %2668, i64 8                                                                                              ;L1127<2220
 44336|     ;; __self_discr = i64 %2669
 44337|  %2671 = load i64, ptr %2646, , !!8                                                                                    ;L1127<2220
 44338|     ;; __arg1_discr = i64 %2671
 44339|  %2672 = icmp eq i64 %2669, %2671                                                                                      ;L1127<2220
 44340|  br i1 %2672, label %2673, label %2675                                                                                 ;L1127<2220
 44341| 
 44342| 2673: ; preds = %2666
 44343|  %2674 = icmp eq i64 %2669, 0                                                                                          ;L1127<2220
 44344|  br i1 %2674, label %2680, label %2755                                                                                 ;L1127<2220
 44345| 
 44346| 2675: ; preds = %2680, %2666
 44347|  %2676 = load i64, ptr %2654, , !!8                                                                                    ;L2224
 44348|  %2677 = load i64, ptr %2655, , !!8                                                                                    ;L2224
 44349|  %2678 = load i32, ptr %2656, , !!8                                                                                    ;L1511<2224
 44350|     ;; mult = i32 %2678
 44351|  %2679 = icmp eq i32 %2678, 0                                                                                          ;L1512<2224
 44352|  br i1 %2679, label %2684, label %2686                                                                                 ;L1512<2224
 44353| 
 44354| 2680: ; preds = %2673
 44355|     ;; __self_0 = ptr %2668
 44356|     ;; self = ptr %2668
 44357|     ;; __arg1_0 = ptr %2646
 44358|     ;; other = ptr %2646
 44361|  %2681 = load i64, ptr %2670, , !!8                                                                                    ;L1878<2123<1127<2220
 44362|  %2682 = load i64, ptr %2653, , !!8                                                                                    ;L1878<2123<1127<2220
 44363|  %2683 = icmp eq i64 %2681, %2682                                                                                      ;L1878<2123<1127<2220
 44364|  br i1 %2683, label %2755, label %2675                                                                                 ;L2220
 44365| 
 44366| 2684: ; preds = %2675
 44367|  %2685 = load i64, ptr %2657, , !!8                                                                                    ;L1513<2224
 44368|  br label %2692                                                                                                        ;L1512<2224
 44369| 
 44370| 2686: ; preds = %2675
 44371|  %2687 = sext i32 %2678 to i64                                                                                         ;L1511<2224
 44372|     ;; mult = i64 %2687
 44373|  %2688 = load i64, ptr %2657, , !!8                                                                                    ;L1515<2224
 44374|  %2689 = add nsw i64 %2687, 100                                                                                        ;L1515<2224
 44375|  %2690 = mul i64 %2688, %2689                                                                                          ;L1515<2224
 44376|  %2691 = udiv i64 %2690, 100                                                                                           ;L1515<2224
 44377|  br label %2692                                                                                                        ;L1512<2224
 44378| 
 44379| 2692: ; preds = %2686, %2684
 44380|  %2693 = phi i64 [ %2685, %2684 ], [ %2691, %2686 ]                                                                    ;L0<2224
 44381|  %2694 = load i64, ptr %2658, , !!8                                                                                    ;L2224
 44382|  %2695 = mul i64 %2694, 30                                                                                             ;L2224
 44383|     ;; tower = ptr %2668
 44384|     ;; caster = ptr %2668
 44385|     ;; self = ptr %2668
 44386|     ;; x = i64 %2676
 44387|     ;; x1 = i64 %2676
 44388|     ;; self = i64 %2676
 44389|     ;; y = i64 %2677
 44390|     ;; y1 = i64 %2677
 44391|     ;; self = i64 %2677
 44392|     ;; r = i64 %2693
 44393|     ;; d = i64 %2695
 44394|  %2696 = gep %2668, i64 1216                                                                                           ;L1449<2224
 44395|  %2697 = load i32, ptr %2696, , !!8                                                                                    ;L1449<2224
 44396|  %2698 = icmp eq i32 %2697, -1                                                                                         ;L1449<2224
 44397|  br i1 %2698, label %2755, label %2699                                                                                 ;L1449<2224
 44398| 
 44399| 2699: ; preds = %2692
 44400|     ;; attack = ptr %2668
 44401|     ;; self = ptr %2668
 44402|  %2700 = gep %2668, i64 1184                                                                                           ;L26<1450<2224
 44403|  %2701 = load i64, ptr %2700, , !!8                                                                                    ;L26<1450<2224
 44404|  %2702 = gep %2668, i64 1192                                                                                           ;L26<1450<2224
 44405|  %2703 = load i64, ptr %2702, , !!8                                                                                    ;L26<1450<2224
 44406|  %2704 = gep %2668, i64 1480                                                                                           ;L26<1450<2224
 44407|  %2705 = load i64, ptr %2704, , !!8                                                                                    ;L26<1450<2224
 44408|  %2706 = add i64 %2705, -1                                                                                             ;L26<1450<2224
 44409|  %2707 = mul i64 %2706, %2703                                                                                          ;L26<1450<2224
 44410|  %2708 = gep %2668, i64 1080                                                                                           ;L26<1450<2224
 44411|  %2709 = load i64, ptr %2708, , !!8                                                                                    ;L26<1450<2224
 44413|  %2710 = gep %2668, i64 1632                                                                                           ;L1451<2224
 44414|  %2711 = load i64, ptr %2710, , !!8                                                                                    ;L1451<2224
 44415|     ;; x2 = i64 %2711
 44416|     ;; other = i64 %2711
 44417|  %2712 = gep %2668, i64 1640                                                                                           ;L1451<2224
 44418|  %2713 = load i64, ptr %2712, , !!8                                                                                    ;L1451<2224
 44419|     ;; y2 = i64 %2713
 44420|     ;; other = i64 %2713
 44421|  %2714 = icmp ult i64 %2676, %2711                                                                                     ;L3147<7<1451<2224
 44422|  %2715 = sub nuw i64 %2711, %2676                                                                                      ;L3147<7<1451<2224
 44423|  %2716 = sub nuw i64 %2676, %2711                                                                                      ;L3147<7<1451<2224
 44424|  %2717 = select i1 %2714, i64 %2715, i64 %2716                                                                         ;L3147<7<1451<2224
 44425|     ;; dx = i64 %2717
 44426|  %2718 = icmp ult i64 %2677, %2713                                                                                     ;L3147<8<1451<2224
 44427|  %2719 = sub nuw i64 %2713, %2677                                                                                      ;L3147<8<1451<2224
 44428|  %2720 = sub nuw i64 %2677, %2713                                                                                      ;L3147<8<1451<2224
 44429|  %2721 = select i1 %2718, i64 %2719, i64 %2720                                                                         ;L3147<8<1451<2224
 44430|     ;; dy = i64 %2721
 44431|  %2722 = mul i64 %2717, %2717                                                                                          ;L9<1451<2224
 44432|  %2723 = mul i64 %2721, %2721                                                                                          ;L9<1451<2224
 44433|  %2724 = add i64 %2723, %2722                                                                                          ;L9<1451<2224
 44434|     ;; dist = i64 %2724
 44435|  %2725 = gep %2668, i64 1136                                                                                           ;L1511<1452<2224
 44436|  %2726 = load i32, ptr %2725, , !!8                                                                                    ;L1511<1452<2224
 44437|     ;; mult = i32 %2726
 44438|  %2727 = icmp eq i32 %2726, 0                                                                                          ;L1512<1452<2224
 44439|  br i1 %2727, label %2728, label %2731                                                                                 ;L1512<1452<2224
 44440| 
 44441| 2728: ; preds = %2699
 44442|  %2729 = gep %2668, i64 1664                                                                                           ;L1513<1452<2224
 44443|  %2730 = load i64, ptr %2729, , !!8                                                                                    ;L1513<1452<2224
 44444|  br label %2738                                                                                                        ;L1512<1452<2224
 44445| 
 44446| 2731: ; preds = %2699
 44447|  %2732 = sext i32 %2726 to i64                                                                                         ;L1511<1452<2224
 44448|     ;; mult = i64 %2732
 44449|  %2733 = gep %2668, i64 1664                                                                                           ;L1515<1452<2224
 44450|  %2734 = load i64, ptr %2733, , !!8                                                                                    ;L1515<1452<2224
 44451|  %2735 = add nsw i64 %2732, 100                                                                                        ;L1515<1452<2224
 44452|  %2736 = mul i64 %2734, %2735                                                                                          ;L1515<1452<2224
 44453|  %2737 = udiv i64 %2736, 100                                                                                           ;L1515<1452<2224
 44454|  br label %2738                                                                                                        ;L1512<1452<2224
 44455| 
 44456| 2738: ; preds = %2731, %2728
 44457|  %2739 = phi i64 [ %2730, %2728 ], [ %2737, %2731 ]                                                                    ;L0<1452<2224
 44458|  %2740 = add i64 %2695, %2693                                                                                          ;L26<1450<2224
 44459|  %2741 = add i64 %2740, %2701                                                                                          ;L26<1450<2224
 44460|  %2742 = add i64 %2741, %2709                                                                                          ;L1452<2224
 44461|  %2743 = add i64 %2742, %2707                                                                                          ;L1452<2224
 44462|  %2744 = add i64 %2743, %2739                                                                                          ;L1452<2224
 44463|     ;; check = i64 %2744
 44464|  %2745 = mul i64 %2744, %2744                                                                                          ;L1453<2224
 44465|  %2746 = icmp ugt i64 %2724, %2745                                                                                     ;L1453<2224
 44466|  br i1 %2746, label %2755, label %2747                                                                                 ;L2224
 44467| 
 44468| 2747: ; preds = %2738
 44469|     ;; t = ptr %2668
 44470|  %2748 = gep %2668, i64 104                                                                                            ;L2230
 44471|  %2749 = load i64, ptr %2748, , !!8                                                                                    ;L2230
 44472|  %2750 = icmp eq i64 %2749, 2                                                                                          ;L2230
 44473|  br i1 %2750, label %2751, label %2755                                                                                 ;L2230
 44474| 
 44475| 2751: ; preds = %2747
 44476|     ;; info = ptr %2668
 44477|  %2752 = gep %2668, i64 136                                                                                            ;L2232
 44478|  %2753 = load i64, ptr %2752, , !!8                                                                                    ;L2232
 44479|  %2754 = trunc nuw i64 %2753 to i1                                                                                     ;L2232
 44480|  br i1 %2754, label %2756, label %2816                                                                                 ;L2232
 44481| 
 44482| 2755: ; preds = %2819, %2813, %2802, %2777, %2765, %2747, %2738, %2692, %2680, %2673
 44483|  br label %2663                                                                                                        ;L1714<180<2219
 44484| 
 44485| 2756: ; preds = %2751
 44486|  %2757 = gep %2668, i64 152                                                                                            ;L2232
 44487|  %2758 = load i64, ptr %2757, , !!8                                                                                    ;L2232
 44488|     ;; id = i64 %2758
 44489|  %2759 = load i64, ptr %2660, , !!8                                                                                    ;L2233
 44490|  %2760 = icmp eq i64 %2758, %2759                                                                                      ;L2233
 44491|  br i1 %2760, label %2810, label %2761                                                                                 ;L2233
 44492| 
 44493| 2761: ; preds = %2756
 44494|  %2762 = invoke ptr %2645(ptr %177, i64 %2758)
 44495|  to label %2763 unwind label %365                                                                                      ;L2235
 44496| 
 44497| 2763: ; preds = %2761
 44498|  %2764 = icmp eq ptr %2762, null                                                                                       ;L2235
 44499|  br i1 %2764, label %2769, label %2765                                                                                 ;L2235
 44500| 
 44501| 2765: ; preds = %2763
 44502|     ;; other = ptr %2762
 44503|     ;; self = ptr %2762
 44504|  %2766 = gep %2762, i64 104                                                                                            ;L1404<2236
 44505|  %2767 = load i64, ptr %2766, , !!8                                                                                    ;L1404<2236
 44506|  %2768 = icmp eq i64 %2767, 13                                                                                         ;L2236
 44507|  br i1 %2768, label %2755, label %2770                                                                                 ;L2236
 44508| 
 44509| 2769: ; preds = %2763
 44512|  invoke void @gc::simulationNtB5_21AbstractGameWithCache12iter_minions(ptr sret([56 x i8]) %17, ptr %55, i64 %49)
 44513|  to label %2780 unwind label %365                                                                                      ;L2240
 44514| 
 44515| 2770: ; preds = %2765
 44516|     ;; self = ptr %2668
 44517|  %2771 = load i32, ptr %2696, , !!8                                                                                    ;L742<2237
 44518|  %2772 = icmp eq i32 %2771, -1                                                                                         ;L742<2237
 44519|  br i1 %2772, label %2776, label %2773                                                                                 ;L742<2237
 44520| 
 44521| 2773: ; preds = %2770
 44522|  %2774 = gep %2668, i64 1168                                                                                           ;L742<2237
 44523|     ;; self = ptr %2774
 44524|  %2775 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2774, ptr %64, ptr %2668, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %2646)
 44525|  to label %2777 unwind label %365                                                                                      ;L2237
 44526| 
 44527| 2776: ; preds = %2770
 44528|     ;; self = ptr null
 44529|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.159) #25
 44530|  to label %173 unwind label %365                                                                                       ;L1013<2237
 44531| 
 44532| 2777: ; preds = %2773
 44533|  %2778 = load i64, ptr %2659, , !!8                                                                                    ;L2237
 44534|  %2779 = add i64 %2778, %2775                                                                                          ;L2237
 44535|  store i64 %2779, ptr %2659,                                                                                           ;L2237
 44536|  br label %2755                                                                                                        ;L2236
 44537| 
 44538| 2780: ; preds = %2769
 44539|     ;; predicate = ptr %2668
 44540|  call void @llvm.memcpy.p0.p0.i64(ptr %18, ptr %17, i64 56, i1 false)                                                  ;L28<957<2241
 44541|  store ptr %2668, ptr %2460,                                                                                           ;L28<957<2241
 44543|     ;; self = ptr %18
 44544|     ;; default = i64 -1
 44545|     ;; init = i64 0
 44548|  invoke fastcc void @gc::simulation6entity6EntityEEB13_EB13_ENtNtNtB8_6traits8iterator8Iterator9size_hintCshdEBA0ozCnw_7game_ai(ptr %8, ptr %18)
 44549|  to label %2781 unwind label %365                                                                                      ;L141<2243
 44550| 
 44551| 2781: ; preds = %2780
 44552|  %2782 = load i64, ptr %2461, , !!57638, !!8                                                                           ;L141<2243
 44553|     ;; self[0..+8] = i64 %2782
 44555|  %2783 = load i64, ptr %2462, , !!57638                                                                                ;L1039<141<2243
 44559|  call void @llvm.memcpy.p0.p0.i64(ptr %7, ptr %18, i64 56, i1 false)                                                   ;L142<2243
 44560|     ;; self[56..+8] = ptr %2668
 44561|     ;; iter[56..+8] = ptr %2668
 44562|     ;; self[56..+8] = ptr %2668
 44563|  %2784 = invoke i64 @core::iter8adapters5chainINtB5_5ChainIBP_INtNtB7_6copied6CopiedINtNtNtBb_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEEB14_EB14_ENtNtNtB9_6traits8iterator8Iterator4foldjNCINvNtB7_3map8map_foldB1Q_jjNCINvNvXs1_NtB7_6filterINtB4f_6FilterppEB2X_5count8to_usizeB1Q_NCNvNtCshdEBA0ozCnw_7game_ai15score_parameter25calculate_score_parametersy_0E0NCINvXsK_NtB31_5accumjNtB6u_3Sum3sumINtB3G_3MapBO_B44_EE0E0EB5b_(ptr %7, i64 0, ptr %2668)
 44564|  to label %2785 unwind label %365                                                                                      ;L128<52<3674<142<2243
 44565| 
 44566| 2785: ; preds = %2781
 44567|  %2786 = trunc nuw i64 %2782 to i1                                                                                     ;L1039<141<2243
 44568|     ;; total = i64 %2784
 44570|     ;; count = i64 %2784
 44572|  %2787 = icmp uge i64 %2783, %2784                                                                                     ;L251<145<2243
 44573|  %2788 = xor i1 %2786, true                                                                                            ;L251<145<2243
 44574|  %2789 = select i1 %2788, i1 true, i1 %2787                                                                            ;L251<145<2243
 44575|     ;; cond = i1 true
 44576|  call void @llvm.assume(i1 %2789)                                                                                      ;L210<251<145<2243
 44577|     ;; cnt = i64 %2784
 44578|  %2790 = icmp ult i64 %2784, 3                                                                                         ;L2246
 44579|     ;; self = ptr %2668
 44580|     ;; self = ptr %2668
 44581|  %2791 = load i32, ptr %2696, , !!8                                                                                    ;L742<0
 44582|  %2792 = icmp eq i32 %2791, -1                                                                                         ;L742<0
 44583|  br i1 %2790, label %2794, label %2793                                                                                 ;L2246
 44584| 
 44585| 2793: ; preds = %2785
 44586|  br i1 %2792, label %2798, label %2795                                                                                 ;L742<2249
 44587| 
 44588| 2794: ; preds = %2785
 44589|  br i1 %2792, label %2806, label %2803                                                                                 ;L742<2247
 44590| 
 44591| 2795: ; preds = %2793
 44592|  %2796 = gep %2668, i64 1168                                                                                           ;L742<2249
 44593|     ;; self = ptr %2796
 44594|  %2797 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2796, ptr %64, ptr %2668, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %2646)
 44595|  to label %2799 unwind label %365                                                                                      ;L2249
 44596| 
 44597| 2798: ; preds = %2793
 44598|     ;; self = ptr null
 44599|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.160) #25
 44600|  to label %173 unwind label %365                                                                                       ;L1013<2249
 44601| 
 44602| 2799: ; preds = %2795
 44603|  %2800 = load i64, ptr %2659, , !!8                                                                                    ;L2249
 44604|  %2801 = add i64 %2800, %2797                                                                                          ;L2249
 44605|  store i64 %2801, ptr %2659,                                                                                           ;L2249
 44606|  br label %2802                                                                                                        ;L2246
 44607| 
 44608| 2802: ; preds = %2807, %2799
 44610|  br label %2755                                                                                                        ;L2235
 44611| 
 44612| 2803: ; preds = %2794
 44613|  %2804 = gep %2668, i64 1168                                                                                           ;L742<2247
 44614|     ;; self = ptr %2804
 44615|  %2805 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2804, ptr %64, ptr %2668, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %2646)
 44616|  to label %2807 unwind label %365                                                                                      ;L2247
 44617| 
 44618| 2806: ; preds = %2794
 44619|     ;; self = ptr null
 44620|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.161) #25
 44621|  to label %173 unwind label %365                                                                                       ;L1013<2247
 44622| 
 44623| 2807: ; preds = %2803
 44624|  %2808 = load i64, ptr %2661, , !!8                                                                                    ;L2247
 44625|  %2809 = add i64 %2808, %2805                                                                                          ;L2247
 44626|  store i64 %2809, ptr %2661,                                                                                           ;L2247
 44627|  br label %2802                                                                                                        ;L2246
 44628| 
 44629| 2810: ; preds = %2756
 44630|     ;; self = ptr %2668
 44631|  %2811 = gep %2668, i64 1168                                                                                           ;L742<2234
 44632|     ;; self = ptr %2811
 44633|  %2812 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2811, ptr %64, ptr %2668, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %2646)
 44634|  to label %2813 unwind label %365                                                                                      ;L2234
 44635| 
 44636| 2813: ; preds = %2810
 44637|  %2814 = load i64, ptr %2661, , !!8                                                                                    ;L2234
 44638|  %2815 = add i64 %2814, %2812                                                                                          ;L2234
 44639|  store i64 %2815, ptr %2661,                                                                                           ;L2234
 44640|  br label %2755                                                                                                        ;L2233
 44641| 
 44642| 2816: ; preds = %2751
 44643|     ;; self = ptr %2668
 44644|  %2817 = gep %2668, i64 1168                                                                                           ;L742<2253
 44645|     ;; self = ptr %2817
 44646|  %2818 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2817, ptr %64, ptr %2668, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %2646)
 44647|  to label %2819 unwind label %365                                                                                      ;L2253
 44648| 
 44649| 2819: ; preds = %2816
 44650|  %2820 = load i64, ptr %2659, , !!8                                                                                    ;L2253
 44651|  %2821 = add i64 %2820, %2818                                                                                          ;L2253
 44652|  store i64 %2821, ptr %2659,                                                                                           ;L2253
 44653|  br label %2755                                                                                                        ;L2232
 44654| 
 44655| 2822: ; preds = %2638, %2234
 44656|  %2823 = gep %55, i64 240                                                                                              ;L2265
 44657|  %2824 = getelementptr { { ptr, ptr, i64 }, i64 }, ptr %2823, i64 %144                                                 ;L2265
 44658|     ;; self = ptr %2824
 44659|     ;; self = ptr %2824
 44660|  %2825 = load ptr, ptr %2824, , !!8, !!8                                                                               ;L138<2073<2265
 44661|     ;; p = ptr %2825
 44662|  %2826 = gep %2824, i64 24                                                                                             ;L2075<2265
 44663|  %2827 = load i64, ptr %2826, , !!8                                                                                    ;L2075<2265
 44664|     ;; len = i64 %2827
 44665|     ;; count = i64 %2827
 44666|     ;; self[0..+8] = ptr %2825
 44667|     ;; slice[0..+8] = ptr %2825
 44668|     ;; self[8..+8] = i64 %2827
 44669|     ;; slice[8..+8] = i64 %2827
 44670|     ;; ptr = ptr %2825
 44671|     ;; self = ptr %2825
 44672|  %2828 = getelementptr ptr, ptr %2825, i64 %2827                                                                       ;L961<100<1042<2265
 44673|     ;; iter[0..+8] = ptr %2825
 44674|     ;; iter[8..+8] = ptr %2828
 44675|  br label %2829                                                                                                        ;L2265
 44676| 
 44677| 2829: ; preds = %2867, %2822
 44678|  %2830 = phi ptr [ %2825, %2822 ], [ %2833, %2867 ]                                                                    ;L2265
 44679|     ;; iter[0..+8] = ptr %2830
 44680|     ;; self = ptr undef
 44681|     ;; ptr = ptr %2830
 44682|     ;; self = ptr %2830
 44683|     ;; end_or_len = ptr %2828
 44686|  %2831 = icmp eq ptr %2830, %2828                                                                                      ;L1714<180<2265
 44687|  br i1 %2831, label %2853, label %2832                                                                                 ;L180<2265
 44688| 
 44689| 2832: ; preds = %2829
 44690|  %2833 = gep %2830, i64 8                                                                                              ;L656<185<2265
 44691|     ;; iter[0..+8] = ptr %2833
 44692|  %2834 = load ptr, ptr %2830, , !!8, !!8                                                                               ;L2265
 44693|     ;; e = ptr %2834
 44694|     ;; self = ptr %2834
 44695|     ;; self = ptr %2834
 44696|  %2835 = gep %2834, i64 1632                                                                                           ;L2158<2266
 44697|  %2836 = load i64, ptr %2835, , !!8                                                                                    ;L2158<2266
 44698|     ;; x1 = i64 %2836
 44699|     ;; self = i64 %2836
 44700|     ;; x1 = i64 %2836
 44701|     ;; self = i64 %2836
 44702|  %2837 = gep %2834, i64 1640                                                                                           ;L2158<2266
 44703|  %2838 = load i64, ptr %2837, , !!8                                                                                    ;L2158<2266
 44704|     ;; y1 = i64 %2838
 44705|     ;; self = i64 %2838
 44706|     ;; y1 = i64 %2838
 44707|     ;; self = i64 %2838
 44708|  %2839 = load i64, ptr %373, , !!8                                                                                     ;L2158<2266
 44709|     ;; x2 = i64 %2839
 44710|     ;; other = i64 %2839
 44711|     ;; x2 = i64 %2839
 44712|     ;; other = i64 %2839
 44713|  %2840 = load i64, ptr %374, , !!8                                                                                     ;L2158<2266
 44714|     ;; y2 = i64 %2840
 44715|     ;; other = i64 %2840
 44716|     ;; y2 = i64 %2840
 44717|     ;; other = i64 %2840
 44718|  %2841 = icmp ult i64 %2836, %2839                                                                                     ;L3147<7<2158<2266
 44719|  %2842 = sub nuw i64 %2839, %2836                                                                                      ;L3147<7<2158<2266
 44720|  %2843 = sub nuw i64 %2836, %2839                                                                                      ;L3147<7<2158<2266
 44721|  %2844 = select i1 %2841, i64 %2842, i64 %2843                                                                         ;L3147<7<2158<2266
 44722|     ;; dx = i64 %2844
 44723|  %2845 = icmp ult i64 %2838, %2840                                                                                     ;L3147<8<2158<2266
 44724|  %2846 = sub nuw i64 %2840, %2838                                                                                      ;L3147<8<2158<2266
 44725|  %2847 = sub nuw i64 %2838, %2840                                                                                      ;L3147<8<2158<2266
 44726|  %2848 = select i1 %2845, i64 %2846, i64 %2847                                                                         ;L3147<8<2158<2266
 44727|     ;; dy = i64 %2848
 44728|  %2849 = mul i64 %2844, %2844                                                                                          ;L9<2158<2266
 44729|  %2850 = mul i64 %2848, %2848                                                                                          ;L9<2158<2266
 44730|  %2851 = add i64 %2850, %2849                                                                                          ;L9<2158<2266
 44731|  %2852 = icmp ugt i64 %2851, 22500000000                                                                               ;L2266
 44732|  br i1 %2852, label %2867, label %2859                                                                                 ;L2266
 44733| 
 44734| 2853: ; preds = %2829
 44735|     ;; player_direct_minion_risk_damage = i64 0
 44736|     ;; rhs = i64 0
 44737|     ;; self = ptr %46
 44738|     ;; self = ptr %46
 44739|     ;; self = ptr %46
 44740|  %2854 = load ptr, ptr %46, , !!8, !!8                                                                                 ;L138<2073<2136<2301
 44741|     ;; p = ptr %2854
 44742|  %2855 = gep %46, i64 24                                                                                               ;L2075<2136<2301
 44743|  %2856 = load i64, ptr %2855, , !!8                                                                                    ;L2075<2136<2301
 44744|     ;; len = i64 %2856
 44745|     ;; count = i64 %2856
 44746|     ;; self[0..+8] = ptr %2854
 44747|     ;; slice[0..+8] = ptr %2854
 44748|     ;; self[8..+8] = i64 %2856
 44749|     ;; slice[8..+8] = i64 %2856
 44750|     ;; ptr = ptr %2854
 44751|     ;; self = ptr %2854
 44752|  %2857 = getelementptr ptr, ptr %2854, i64 %2856                                                                       ;L961<100<1042<2136<2301
 44753|     ;; iter[0..+8] = ptr %2854
 44754|     ;; iter[8..+8] = ptr %2857
 44755|  %2858 = gep %59, i64 8
 44756|  br label %2937                                                                                                        ;L2301
 44757| 
 44758| 2859: ; preds = %2832
 44759|     ;; self = ptr %2834
 44760|  %2860 = gep %2834, i64 1168                                                                                           ;L742<2269
 44761|  %2861 = gep %2834, i64 1216                                                                                           ;L742<2269
 44762|  %2862 = load i32, ptr %2861, , !!8                                                                                    ;L742<2269
 44763|  %2863 = icmp eq i32 %2862, -1                                                                                         ;L742<2269
 44764|  br i1 %2863, label %2867, label %2864                                                                                 ;L742<2269
 44765| 
 44766| 2864: ; preds = %2859
 44767|     ;; atk = ptr %2860
 44768|  %2865 = gep %2834, i64 104                                                                                            ;L2270
 44769|  %2866 = load i64, ptr %2865, , !!8                                                                                    ;L2270
 44770|  switch i64 %2866, label %2879 [
 44771|  i64 7, label %2869
 44772|  i64 9, label %2869
 44773|  i64 10, label %2868
 44774|  ]                                                                                                                     ;L2270
 44775| 
 44776| 2867: ; preds = %2934, %2928, %2924, %2905, %2879, %2859, %2832
 44777|  br label %2829                                                                                                        ;L1714<180<2265
 44778| 
 44779| 2868: ; preds = %2864
 44780|     ;; info = ptr %2834
 44783|  br label %2869                                                                                                        ;L2272
 44784| 
 44785| 2869: ; preds = %2868, %2864, %2864
 44786|  %2870 = phi i64 [ 112, %2868 ], [ 136, %2864 ], [ 136, %2864 ]
 44787|  %2871 = phi i64 [ 120, %2868 ], [ 144, %2864 ], [ 144, %2864 ]
 44788|  %2872 = gep %2834, i64 %2870                                                                                          ;L0
 44789|  %2873 = gep %2834, i64 %2871                                                                                          ;L0
 44790|  %2874 = load i64, ptr %2873,                                                                                          ;L0
 44791|  %2875 = load i64, ptr %2872, , !!8                                                                                    ;L0
 44792|     ;; nearest_enemy[8..+8] = i64 %2874
 44793|     ;; nearest_enemy[0..+8] = i64 %2875
 44794|  %2876 = trunc nuw i64 %2875 to i1                                                                                     ;L2277
 44795|  br i1 %2876, label %2877, label %2879                                                                                 ;L2277
 44796| 
 44797| 2877: ; preds = %2869
 44798|     ;; id = i64 %2874
 44799|  %2878 = icmp eq i64 %2874, %157                                                                                       ;L2278
 44800|  br i1 %2878, label %2893, label %2881                                                                                 ;L2278
 44801| 
 44802| 2879: ; preds = %2869, %2864
 44803|  %2880 = icmp samesign ult i64 %2851, 2500000001                                                                       ;L2293
 44804|  br i1 %2880, label %2932, label %2867                                                                                 ;L2293
 44805| 
 44806| 2881: ; preds = %2895, %2877
 44807|     ;; self = ptr %47
 44808|     ;; self = ptr %47
 44809|  %2882 = load ptr, ptr %67, , !!8, !!8                                                                                 ;L138<2083<2282
 44810|     ;; ptr = ptr %2882
 44811|  %2883 = load i64, ptr %70, , !!8                                                                                      ;L2085<2282
 44812|     ;; len = i64 %2883
 44813|     ;; count = i64 %2883
 44814|     ;; self[0..+8] = ptr %2882
 44815|     ;; slice[0..+8] = ptr %2882
 44816|     ;; self[8..+8] = i64 %2883
 44817|     ;; slice[8..+8] = i64 %2883
 44818|     ;; ptr = ptr %2882
 44819|     ;; self = ptr %2882
 44820|  %2884 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %2882, i64 %2883 ;L961<240<1062<2282
 44821|     ;; predicate = ptr undef
 44822|     ;; self = ptr undef
 44823|     ;; self = ptr undef
 44824|     ;; count = i64 1
 44825|  br label %2885                                                                                                        ;L348<2282
 44826| 
 44827| 2885: ; preds = %2888, %2881
 44828|  %2886 = phi ptr [ %2889, %2888 ], [ %2882, %2881 ]
 44829|     ;; ptr = ptr %2886
 44830|     ;; self = ptr %2886
 44831|     ;; end_or_len = ptr %2884
 44834|  %2887 = icmp eq ptr %2886, %2884                                                                                      ;L1714<180<348<2282
 44835|  br i1 %2887, label %2901, label %2888                                                                                 ;L180<348<2282
 44836| 
 44837| 2888: ; preds = %2885
 44838|  %2889 = gep %2886, i64 216                                                                                            ;L656<185<348<2282
 44839|     ;; x = ptr %2886
 44842|  %2890 = gep %2886, i64 88                                                                                             ;L2282<349<2282
 44843|  %2891 = load i64, ptr %2890, , !!57818, !!8                                                                           ;L2282<349<2282
 44844|  %2892 = icmp eq i64 %2891, %2874                                                                                      ;L2282<349<2282
 44845|  br i1 %2892, label %2898, label %2885                                                                                 ;L349<2282
 44846| 
 44847| 2893: ; preds = %2877
 44848|  %2894 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2860, ptr %64, ptr %2834, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 44849|  to label %2895 unwind label %365                                                                                      ;L2279
 44850| 
 44851| 2895: ; preds = %2893
 44852|  %2896 = load i64, ptr %89, , !!8                                                                                      ;L2279
 44853|  %2897 = add i64 %2896, %2894                                                                                          ;L2279
 44854|  store i64 %2897, ptr %89,                                                                                             ;L2279
 44855|  br label %2881                                                                                                        ;L2278
 44856| 
 44857| 2898: ; preds = %2888
 44858|     ;; target = ptr %2886
 44859|  %2899 = load ptr, ptr %372, , !!8                                                                                     ;L2283
 44860|  %2900 = invoke ptr %2899(ptr %177, i64 %2874)
 44861|  to label %2913 unwind label %365                                                                                      ;L2283
 44862| 
 44863| 2901: ; preds = %2917, %2913, %2885
 44864|     ;; self = ptr %47
 44865|     ;; self = ptr %47
 44866|  %2902 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<2288
 44867|     ;; ptr = ptr %2902
 44868|  %2903 = load i64, ptr %74, , !!8                                                                                      ;L2085<2288
 44869|     ;; len = i64 %2903
 44870|     ;; count = i64 %2903
 44871|     ;; self[0..+8] = ptr %2902
 44872|     ;; slice[0..+8] = ptr %2902
 44873|     ;; self[8..+8] = i64 %2903
 44874|     ;; slice[8..+8] = i64 %2903
 44875|     ;; ptr = ptr %2902
 44876|     ;; self = ptr %2902
 44877|  %2904 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %2902, i64 %2903 ;L961<240<1062<2288
 44878|     ;; predicate = ptr undef
 44879|     ;; self = ptr undef
 44880|     ;; self = ptr undef
 44881|     ;; count = i64 1
 44882|  br label %2905                                                                                                        ;L348<2288
 44883| 
 44884| 2905: ; preds = %2908, %2901
 44885|  %2906 = phi ptr [ %2909, %2908 ], [ %2902, %2901 ]
 44886|     ;; ptr = ptr %2906
 44887|     ;; self = ptr %2906
 44888|     ;; end_or_len = ptr %2904
 44891|  %2907 = icmp eq ptr %2906, %2904                                                                                      ;L1714<180<348<2288
 44892|  br i1 %2907, label %2867, label %2908                                                                                 ;L180<348<2288
 44893| 
 44894| 2908: ; preds = %2905
 44895|  %2909 = gep %2906, i64 216                                                                                            ;L656<185<348<2288
 44896|     ;; x = ptr %2906
 44899|  %2910 = gep %2906, i64 88                                                                                             ;L2288<349<2288
 44900|  %2911 = load i64, ptr %2910, , !!57873, !!8                                                                           ;L2288<349<2288
 44901|  %2912 = icmp eq i64 %2911, %2874                                                                                      ;L2288<349<2288
 44902|  br i1 %2912, label %2921, label %2905                                                                                 ;L349<2288
 44903| 
 44904| 2913: ; preds = %2898
 44905|  %2914 = icmp eq ptr %2900, null                                                                                       ;L2283
 44906|  br i1 %2914, label %2901, label %2915                                                                                 ;L2283
 44907| 
 44908| 2915: ; preds = %2913
 44909|     ;; target_entity = ptr %2900
 44910|  %2916 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2860, ptr %64, ptr %2834, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %2900)
 44911|  to label %2917 unwind label %365                                                                                      ;L2284
 44912| 
 44913| 2917: ; preds = %2915
 44914|  %2918 = gep %2886, i64 128                                                                                            ;L2284
 44915|  %2919 = load i64, ptr %2918, , !!8                                                                                    ;L2284
 44916|  %2920 = add i64 %2919, %2916                                                                                          ;L2284
 44917|  store i64 %2920, ptr %2918,                                                                                           ;L2284
 44918|  br label %2901                                                                                                        ;L2283
 44919| 
 44920| 2921: ; preds = %2908
 44921|     ;; target = ptr %2906
 44922|  %2922 = load ptr, ptr %372, , !!8                                                                                     ;L2289
 44923|  %2923 = invoke ptr %2922(ptr %177, i64 %2874)
 44924|  to label %2924 unwind label %365                                                                                      ;L2289
 44925| 
 44926| 2924: ; preds = %2921
 44927|  %2925 = icmp eq ptr %2923, null                                                                                       ;L2289
 44928|  br i1 %2925, label %2867, label %2926                                                                                 ;L2289
 44929| 
 44930| 2926: ; preds = %2924
 44931|     ;; target_entity = ptr %2923
 44932|  %2927 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2860, ptr %64, ptr %2834, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %2923)
 44933|  to label %2928 unwind label %365                                                                                      ;L2290
 44934| 
 44935| 2928: ; preds = %2926
 44936|  %2929 = gep %2906, i64 128                                                                                            ;L2290
 44937|  %2930 = load i64, ptr %2929, , !!8                                                                                    ;L2290
 44938|  %2931 = add i64 %2930, %2927                                                                                          ;L2290
 44939|  store i64 %2931, ptr %2929,                                                                                           ;L2290
 44940|  br label %2867                                                                                                        ;L2289
 44941| 
 44942| 2932: ; preds = %2879
 44943|  %2933 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2860, ptr %64, ptr %2834, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 44944|  to label %2934 unwind label %365                                                                                      ;L2294
 44945| 
 44946| 2934: ; preds = %2932
 44947|  %2935 = load i64, ptr %89, , !!8                                                                                      ;L2294
 44948|  %2936 = add i64 %2935, %2933                                                                                          ;L2294
 44949|  store i64 %2936, ptr %89,                                                                                             ;L2294
 44950|  br label %2867                                                                                                        ;L2293
 44951| 
 44952| 2937: ; preds = %2953, %2853
 44953|  %2938 = phi i64 [ 0, %2853 ], [ %2954, %2953 ]                                                                        ;L2300
 44954|  %2939 = phi ptr [ %2854, %2853 ], [ %2942, %2953 ]                                                                    ;L2301
 44955|     ;; iter[0..+8] = ptr %2939
 44956|     ;; rhs = i64 %2938
 44957|     ;; player_direct_minion_risk_damage = i64 %2938
 44958|     ;; self = ptr undef
 44959|     ;; ptr = ptr %2939
 44960|     ;; self = ptr %2939
 44961|     ;; end_or_len = ptr %2857
 44964|  %2940 = icmp eq ptr %2939, %2857                                                                                      ;L1714<180<2301
 44965|  br i1 %2940, label %2947, label %2941                                                                                 ;L180<2301
 44966| 
 44967| 2941: ; preds = %2937
 44968|  %2942 = gep %2939, i64 8                                                                                              ;L656<185<2301
 44969|     ;; iter[0..+8] = ptr %2942
 44970|  %2943 = load ptr, ptr %2939, , !!8, !!8                                                                               ;L2301
 44971|     ;; m = ptr %2943
 44972|     ;; self = ptr %2943
 44973|  %2944 = gep %2943, i64 104                                                                                            ;L2302
 44974|  %2945 = load i64, ptr %2944, , !!8                                                                                    ;L2302
 44975|  %2946 = icmp eq i64 %2945, 1                                                                                          ;L2302
 44976|  br i1 %2946, label %2949, label %2953                                                                                 ;L2302
 44977| 
 44978| 2947: ; preds = %2937
 44979|  %2948 = invoke i64 %2232(ptr %177)
 44980|  to label %3083 unwind label %365                                                                                      ;L2337
 44981| 
 44982| 2949: ; preds = %2941
 44983|     ;; info = ptr %2943
 44984|  %2950 = gep %2943, i64 136                                                                                            ;L2304
 44985|  %2951 = load i64, ptr %2950, , !!8                                                                                    ;L2304
 44986|  %2952 = trunc nuw i64 %2951 to i1                                                                                     ;L2304
 44987|  br i1 %2952, label %2955, label %2959                                                                                 ;L2304
 44988| 
 44989| 2953: ; preds = %3079, %3071, %3065, %3056, %3046, %3021, %2959, %2941
 44990|  %2954 = phi i64 [ %2938, %2941 ], [ %2938, %3065 ], [ %3082, %3079 ], [ %2938, %3071 ], [ %2938, %2959 ], [ %2979, %3056 ], [ %2979, %3046 ], [ %2979, %3021 ] ;L0
 44991|     ;; rhs = i64 %2954
 44992|     ;; player_direct_minion_risk_damage = i64 %2954
 44993|  br label %2937                                                                                                        ;L2301
 44994| 
 44995| 2955: ; preds = %2949
 44996|  %2956 = gep %2943, i64 144                                                                                            ;L2304
 44997|  %2957 = load i64, ptr %2956, , !!8                                                                                    ;L2304
 44998|     ;; id = i64 %2957
 44999|  %2958 = icmp eq i64 %2957, %157                                                                                       ;L2305
 45000|  br i1 %2958, label %2991, label %2978                                                                                 ;L2305
 45001| 
 45002| 2959: ; preds = %2949
 45003|  %2960 = gep %2943, i64 1632                                                                                           ;L2158<2325
 45004|  %2961 = load i64, ptr %2960, , !!8                                                                                    ;L2158<2325
 45005|     ;; x1 = i64 %2961
 45006|     ;; self = i64 %2961
 45007|  %2962 = gep %2943, i64 1640                                                                                           ;L2158<2325
 45008|  %2963 = load i64, ptr %2962, , !!8                                                                                    ;L2158<2325
 45009|     ;; y1 = i64 %2963
 45010|     ;; self = i64 %2963
 45011|  %2964 = load i64, ptr %373, , !!8                                                                                     ;L2158<2325
 45012|     ;; x2 = i64 %2964
 45013|     ;; other = i64 %2964
 45014|  %2965 = load i64, ptr %374, , !!8                                                                                     ;L2158<2325
 45015|     ;; y2 = i64 %2965
 45016|     ;; other = i64 %2965
 45017|  %2966 = icmp ult i64 %2961, %2964                                                                                     ;L3147<7<2158<2325
 45018|  %2967 = sub nuw i64 %2964, %2961                                                                                      ;L3147<7<2158<2325
 45019|  %2968 = sub nuw i64 %2961, %2964                                                                                      ;L3147<7<2158<2325
 45020|  %2969 = select i1 %2966, i64 %2967, i64 %2968                                                                         ;L3147<7<2158<2325
 45021|     ;; dx = i64 %2969
 45022|  %2970 = icmp ult i64 %2963, %2965                                                                                     ;L3147<8<2158<2325
 45023|  %2971 = sub nuw i64 %2965, %2963                                                                                      ;L3147<8<2158<2325
 45024|  %2972 = sub nuw i64 %2963, %2965                                                                                      ;L3147<8<2158<2325
 45025|  %2973 = select i1 %2970, i64 %2971, i64 %2972                                                                         ;L3147<8<2158<2325
 45026|     ;; dy = i64 %2973
 45027|  %2974 = mul i64 %2969, %2969                                                                                          ;L9<2158<2325
 45028|  %2975 = mul i64 %2973, %2973                                                                                          ;L9<2158<2325
 45029|  %2976 = add i64 %2975, %2974                                                                                          ;L9<2158<2325
 45030|  %2977 = icmp ult i64 %2976, 2500000001                                                                                ;L2325
 45031|  br i1 %2977, label %3060, label %2953                                                                                 ;L2325
 45032| 
 45033| 2978: ; preds = %3010, %3008, %3006, %2955
 45034|  %2979 = phi i64 [ %2938, %3006 ], [ %3009, %3008 ], [ %2938, %3010 ], [ %2938, %2955 ]                                ;L0
 45035|     ;; rhs = i64 %2979
 45036|     ;; player_direct_minion_risk_damage = i64 %2979
 45037|     ;; self = ptr %47
 45038|     ;; self = ptr %47
 45039|  %2980 = load ptr, ptr %67, , !!8, !!8                                                                                 ;L138<2083<2313
 45040|     ;; ptr = ptr %2980
 45041|  %2981 = load i64, ptr %70, , !!8                                                                                      ;L2085<2313
 45042|     ;; len = i64 %2981
 45043|     ;; count = i64 %2981
 45044|     ;; self[0..+8] = ptr %2980
 45045|     ;; slice[0..+8] = ptr %2980
 45046|     ;; self[8..+8] = i64 %2981
 45047|     ;; slice[8..+8] = i64 %2981
 45048|     ;; ptr = ptr %2980
 45049|     ;; self = ptr %2980
 45050|  %2982 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %2980, i64 %2981 ;L961<240<1062<2313
 45051|     ;; predicate = ptr undef
 45052|     ;; self = ptr undef
 45053|     ;; self = ptr undef
 45054|     ;; count = i64 1
 45055|  br label %2983                                                                                                        ;L348<2313
 45056| 
 45057| 2983: ; preds = %2986, %2978
 45058|  %2984 = phi ptr [ %2987, %2986 ], [ %2980, %2978 ]
 45059|     ;; ptr = ptr %2984
 45060|     ;; self = ptr %2984
 45061|     ;; end_or_len = ptr %2982
 45064|  %2985 = icmp eq ptr %2984, %2982                                                                                      ;L1714<180<348<2313
 45065|  br i1 %2985, label %3017, label %2986                                                                                 ;L180<348<2313
 45066| 
 45067| 2986: ; preds = %2983
 45068|  %2987 = gep %2984, i64 216                                                                                            ;L656<185<348<2313
 45069|     ;; x = ptr %2984
 45072|  %2988 = gep %2984, i64 88                                                                                             ;L2313<349<2313
 45073|  %2989 = load i64, ptr %2988, , !!57963, !!8                                                                           ;L2313<349<2313
 45074|  %2990 = icmp eq i64 %2989, %2957                                                                                      ;L2313<349<2313
 45075|  br i1 %2990, label %3014, label %2983                                                                                 ;L349<2313
 45076| 
 45077| 2991: ; preds = %2955
 45078|     ;; self = ptr %2943
 45079|  %2992 = gep %2943, i64 1216                                                                                           ;L742<2306
 45080|  %2993 = load i32, ptr %2992, , !!8                                                                                    ;L742<2306
 45081|  %2994 = icmp eq i32 %2993, -1                                                                                         ;L742<2306
 45082|  br i1 %2994, label %2998, label %2995                                                                                 ;L742<2306
 45083| 
 45084| 2995: ; preds = %2991
 45085|  %2996 = gep %2943, i64 1168                                                                                           ;L742<2306
 45086|     ;; self = ptr %2996
 45087|  %2997 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %2996, ptr %64, ptr %2943, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 45088|  to label %2999 unwind label %365                                                                                      ;L2306
 45089| 
 45090| 2998: ; preds = %2991
 45091|     ;; self = ptr null
 45092|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.162) #25
 45093|  to label %173 unwind label %365                                                                                       ;L1013<2306
 45094| 
 45095| 2999: ; preds = %2995
 45096|     ;; damage = i64 %2997
 45097|  %3000 = load i64, ptr %89, , !!8                                                                                      ;L2307
 45098|  %3001 = add i64 %3000, %2997                                                                                          ;L2307
 45099|  store i64 %3001, ptr %89,                                                                                             ;L2307
 45100|     ;; self = ptr %2943
 45101|     ;; self = ptr %2943
 45102|     ;; other = ptr %59
 45103|     ;; other = ptr %59
 45104|  %3002 = load i64, ptr %2943, , !!8                                                                                    ;L1127<264<2308
 45105|  %3003 = gep %2943, i64 8                                                                                              ;L1127<264<2308
 45106|     ;; __self_discr = i64 %3002
 45107|  %3004 = load i64, ptr %59, , !!8                                                                                      ;L1127<264<2308
 45108|     ;; __arg1_discr = i64 %3004
 45109|  %3005 = icmp eq i64 %3002, %3004                                                                                      ;L1127<264<2308
 45110|  br i1 %3005, label %3006, label %3008                                                                                 ;L1127<264<2308
 45111| 
 45112| 3006: ; preds = %2999
 45113|  %3007 = icmp eq i64 %3002, 0                                                                                          ;L1127<264<2308
 45114|  br i1 %3007, label %3010, label %2978                                                                                 ;L1127<264<2308
 45115| 
 45116| 3008: ; preds = %3010, %2999
 45117|  %3009 = add i64 %2997, %2938                                                                                          ;L2309
 45118|     ;; player_direct_minion_risk_damage = i64 %3009
 45119|     ;; rhs = i64 %3009
 45120|  br label %2978                                                                                                        ;L2308
 45121| 
 45122| 3010: ; preds = %3006
 45123|     ;; __self_0 = ptr %2943
 45124|     ;; self = ptr %2943
 45125|     ;; __arg1_0 = ptr %59
 45126|     ;; other = ptr %59
 45129|  %3011 = load i64, ptr %3003, , !!8                                                                                    ;L1878<2123<1127<264<2308
 45130|  %3012 = load i64, ptr %2858, , !!8                                                                                    ;L1878<2123<1127<264<2308
 45131|  %3013 = icmp eq i64 %3011, %3012                                                                                      ;L1878<2123<1127<264<2308
 45132|  br i1 %3013, label %2978, label %3008                                                                                 ;L2308
 45133| 
 45134| 3014: ; preds = %2986
 45135|     ;; target = ptr %2984
 45136|  %3015 = load ptr, ptr %372, , !!8                                                                                     ;L2314
 45137|  %3016 = invoke ptr %3015(ptr %177, i64 %2957)
 45138|  to label %3029 unwind label %365                                                                                      ;L2314
 45139| 
 45140| 3017: ; preds = %3039, %3029, %2983
 45141|     ;; self = ptr %47
 45142|     ;; self = ptr %47
 45143|  %3018 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<2319
 45144|     ;; ptr = ptr %3018
 45145|  %3019 = load i64, ptr %74, , !!8                                                                                      ;L2085<2319
 45146|     ;; len = i64 %3019
 45147|     ;; count = i64 %3019
 45148|     ;; self[0..+8] = ptr %3018
 45149|     ;; slice[0..+8] = ptr %3018
 45150|     ;; self[8..+8] = i64 %3019
 45151|     ;; slice[8..+8] = i64 %3019
 45152|     ;; ptr = ptr %3018
 45153|     ;; self = ptr %3018
 45154|  %3020 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %3018, i64 %3019 ;L961<240<1062<2319
 45155|     ;; predicate = ptr undef
 45156|     ;; self = ptr undef
 45157|     ;; self = ptr undef
 45158|     ;; count = i64 1
 45159|  br label %3021                                                                                                        ;L348<2319
 45160| 
 45161| 3021: ; preds = %3024, %3017
 45162|  %3022 = phi ptr [ %3025, %3024 ], [ %3018, %3017 ]
 45163|     ;; ptr = ptr %3022
 45164|     ;; self = ptr %3022
 45165|     ;; end_or_len = ptr %3020
 45168|  %3023 = icmp eq ptr %3022, %3020                                                                                      ;L1714<180<348<2319
 45169|  br i1 %3023, label %2953, label %3024                                                                                 ;L180<348<2319
 45170| 
 45171| 3024: ; preds = %3021
 45172|  %3025 = gep %3022, i64 216                                                                                            ;L656<185<348<2319
 45173|     ;; x = ptr %3022
 45176|  %3026 = gep %3022, i64 88                                                                                             ;L2319<349<2319
 45177|  %3027 = load i64, ptr %3026, , !!58044, !!8                                                                           ;L2319<349<2319
 45178|  %3028 = icmp eq i64 %3027, %2957                                                                                      ;L2319<349<2319
 45179|  br i1 %3028, label %3043, label %3021                                                                                 ;L349<2319
 45180| 
 45181| 3029: ; preds = %3014
 45182|  %3030 = icmp eq ptr %3016, null                                                                                       ;L2314
 45183|  br i1 %3030, label %3017, label %3031                                                                                 ;L2314
 45184| 
 45185| 3031: ; preds = %3029
 45186|     ;; target_entity = ptr %3016
 45187|     ;; self = ptr %2943
 45188|  %3032 = gep %2943, i64 1216                                                                                           ;L742<2315
 45189|  %3033 = load i32, ptr %3032, , !!8                                                                                    ;L742<2315
 45190|  %3034 = icmp eq i32 %3033, -1                                                                                         ;L742<2315
 45191|  br i1 %3034, label %3038, label %3035                                                                                 ;L742<2315
 45192| 
 45193| 3035: ; preds = %3031
 45194|  %3036 = gep %2943, i64 1168                                                                                           ;L742<2315
 45195|     ;; self = ptr %3036
 45196|  %3037 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %3036, ptr %64, ptr %2943, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %3016)
 45197|  to label %3039 unwind label %365                                                                                      ;L2315
 45198| 
 45199| 3038: ; preds = %3031
 45200|     ;; self = ptr null
 45201|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.163) #25
 45202|  to label %173 unwind label %365                                                                                       ;L1013<2315
 45203| 
 45204| 3039: ; preds = %3035
 45205|  %3040 = gep %2984, i64 128                                                                                            ;L2315
 45206|  %3041 = load i64, ptr %3040, , !!8                                                                                    ;L2315
 45207|  %3042 = add i64 %3041, %3037                                                                                          ;L2315
 45208|  store i64 %3042, ptr %3040,                                                                                           ;L2315
 45209|  br label %3017                                                                                                        ;L2314
 45210| 
 45211| 3043: ; preds = %3024
 45212|     ;; target = ptr %3022
 45213|  %3044 = load ptr, ptr %372, , !!8                                                                                     ;L2320
 45214|  %3045 = invoke ptr %3044(ptr %177, i64 %2957)
 45215|  to label %3046 unwind label %365                                                                                      ;L2320
 45216| 
 45217| 3046: ; preds = %3043
 45218|  %3047 = icmp eq ptr %3045, null                                                                                       ;L2320
 45219|  br i1 %3047, label %2953, label %3048                                                                                 ;L2320
 45220| 
 45221| 3048: ; preds = %3046
 45222|     ;; target_entity = ptr %3045
 45223|     ;; self = ptr %2943
 45224|  %3049 = gep %2943, i64 1216                                                                                           ;L742<2321
 45225|  %3050 = load i32, ptr %3049, , !!8                                                                                    ;L742<2321
 45226|  %3051 = icmp eq i32 %3050, -1                                                                                         ;L742<2321
 45227|  br i1 %3051, label %3055, label %3052                                                                                 ;L742<2321
 45228| 
 45229| 3052: ; preds = %3048
 45230|  %3053 = gep %2943, i64 1168                                                                                           ;L742<2321
 45231|     ;; self = ptr %3053
 45232|  %3054 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %3053, ptr %64, ptr %2943, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %3045)
 45233|  to label %3056 unwind label %365                                                                                      ;L2321
 45234| 
 45235| 3055: ; preds = %3048
 45236|     ;; self = ptr null
 45237|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.164) #25
 45238|  to label %173 unwind label %365                                                                                       ;L1013<2321
 45239| 
 45240| 3056: ; preds = %3052
 45241|  %3057 = gep %3022, i64 128                                                                                            ;L2321
 45242|  %3058 = load i64, ptr %3057, , !!8                                                                                    ;L2321
 45243|  %3059 = add i64 %3058, %3054                                                                                          ;L2321
 45244|  store i64 %3059, ptr %3057,                                                                                           ;L2321
 45245|  br label %2953                                                                                                        ;L2320
 45246| 
 45247| 3060: ; preds = %2959
 45248|     ;; self = ptr %2943
 45249|     ;; self = ptr %2943
 45250|     ;; other = ptr %59
 45251|     ;; other = ptr %59
 45252|  %3061 = load i64, ptr %2943, , !!8                                                                                    ;L1127<264<2325
 45253|  %3062 = gep %2943, i64 8                                                                                              ;L1127<264<2325
 45254|     ;; __self_discr = i64 %3061
 45255|  %3063 = load i64, ptr %59, , !!8                                                                                      ;L1127<264<2325
 45256|     ;; __arg1_discr = i64 %3063
 45257|  %3064 = icmp eq i64 %3061, %3063                                                                                      ;L1127<264<2325
 45258|  br i1 %3064, label %3065, label %3067                                                                                 ;L1127<264<2325
 45259| 
 45260| 3065: ; preds = %3060
 45261|  %3066 = icmp eq i64 %3061, 0                                                                                          ;L1127<264<2325
 45262|  br i1 %3066, label %3071, label %2953                                                                                 ;L1127<264<2325
 45263| 
 45264| 3067: ; preds = %3071, %3060
 45265|     ;; self = ptr %2943
 45266|  %3068 = gep %2943, i64 1216                                                                                           ;L742<2326
 45267|  %3069 = load i32, ptr %3068, , !!8                                                                                    ;L742<2326
 45268|  %3070 = icmp eq i32 %3069, -1                                                                                         ;L742<2326
 45269|  br i1 %3070, label %3078, label %3075                                                                                 ;L742<2326
 45270| 
 45271| 3071: ; preds = %3065
 45272|     ;; __self_0 = ptr %2943
 45273|     ;; self = ptr %2943
 45274|     ;; __arg1_0 = ptr %59
 45275|     ;; other = ptr %59
 45278|  %3072 = load i64, ptr %3062, , !!8                                                                                    ;L1878<2123<1127<264<2325
 45279|  %3073 = load i64, ptr %2858, , !!8                                                                                    ;L1878<2123<1127<264<2325
 45280|  %3074 = icmp eq i64 %3072, %3073                                                                                      ;L1878<2123<1127<264<2325
 45281|  br i1 %3074, label %2953, label %3067                                                                                 ;L2325
 45282| 
 45283| 3075: ; preds = %3067
 45284|  %3076 = gep %2943, i64 1168                                                                                           ;L742<2326
 45285|     ;; self = ptr %3076
 45286|  %3077 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %3076, ptr %64, ptr %2943, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 45287|  to label %3079 unwind label %365                                                                                      ;L2326
 45288| 
 45289| 3078: ; preds = %3067
 45290|     ;; self = ptr null
 45291|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.165) #25
 45292|  to label %173 unwind label %365                                                                                       ;L1013<2326
 45293| 
 45294| 3079: ; preds = %3075
 45295|     ;; damage = i64 %3077
 45296|  %3080 = load i64, ptr %89, , !!8                                                                                      ;L2327
 45297|  %3081 = add i64 %3080, %3077                                                                                          ;L2327
 45298|  store i64 %3081, ptr %89,                                                                                             ;L2327
 45299|  %3082 = add i64 %3077, %2938                                                                                          ;L2328
 45300|     ;; player_direct_minion_risk_damage = i64 %3082
 45301|     ;; rhs = i64 %3082
 45302|  br label %2953                                                                                                        ;L2325
 45303| 
 45304| 3083: ; preds = %2947
 45305|     ;; tick = i64 %2948
 45306|     ;; tick = i64 %2948
 45307|     ;; self = ptr %64
 45308|  %3084 = gep %64, i64 8
 45309|  %3085 = load ptr, ptr %3084,                                                                                          ;L0
 45310|  switch i8 %1708, label %3086 [
 45311|  i8 0, label %3089
 45312|  i8 7, label %3089
 45313|  i8 8, label %3089
 45314|  i8 5, label %3089
 45315|  ]                                                                                                                     ;L263<399<2337
 45316| 
 45317| 3086: ; preds = %3083
 45318|  %3087 = gep %3085, i64 4856
 45319|  %3088 = load i64, ptr %3087,                                                                                          ;L2339
 45320|  br label %3130                                                                                                        ;L263<399<2337
 45321| 
 45322| 3089: ; preds = %3083, %3083, %3083, %3083
 45323|     ;; self = ptr %3085
 45324|  %3090 = gep %3085, i64 2216                                                                                           ;L703<399<2337
 45325|  %3091 = load i64, ptr %3090, , !!8                                                                                    ;L703<399<2337
 45326|     ;; self = i64 %3091
 45327|  %3092 = gep %3085, i64 4856                                                                                           ;L704<399<2337
 45328|  %3093 = load i64, ptr %3092, , !!8                                                                                    ;L704<399<2337
 45329|  %3094 = mul i64 %3093, 30                                                                                             ;L704<399<2337
 45330|     ;; rhs = i64 %3094
 45331|  %3095 = call i64 @llvm.usub.sat.i64(i64 %3091, i64 %3094)                                                             ;L2472<703<399<2337
 45332|  %3096 = icmp ult i64 %2948, %3095                                                                                     ;L703<399<2337
 45333|  br i1 %3096, label %3130, label %3097                                                                                 ;L2337
 45334| 
 45335| 3097: ; preds = %3089
 45336|  %3098 = load i64, ptr %373, , !!8                                                                                     ;L2341
 45337|  %3099 = load i64, ptr %374, , !!8                                                                                     ;L2341
 45338|     ;; version = i64 %1
 45339|     ;; data = ptr %4
 45340|     ;; target = ptr %59
 45341|     ;; x = i64 %3098
 45342|     ;; y = i64 %3099
 45343|     ;; window_tick = i64 %3093
 45344|  %3100 = invoke i64 @ai::minion_wave_risk32enemy_minion_wave_risk_damage_at(i64 poison, ptr %4, ptr %59, i64 %3098, i64 %3099, i64 %3093)
 45345|  to label %3101 unwind label %365                                                                                      ;L101<2341
 45346| 
 45347| 3101: ; preds = %3097
 45348|     ;; damage = i64 %3100
 45349|     ;; target = ptr %59
 45350|     ;; damage = i64 %3100
 45351|  %3102 = icmp eq i64 %3100, 0                                                                                          ;L65<102<2341
 45352|  br i1 %3102, label %3135, label %3103                                                                                 ;L65<102<2341
 45353| 
 45354| 3103: ; preds = %3101
 45355|  %3104 = gep %59, i64 1648                                                                                             ;L69<102<2341
 45356|  %3105 = load i64, ptr %3104, , !!58130, !!8                                                                           ;L69<102<2341
 45357|  %3106 = gep %59, i64 1576                                                                                             ;L69<102<2341
 45358|  %3107 = load i64, ptr %3106, , !!58130, !!8                                                                           ;L69<102<2341
 45359|     ;; self = i64 %3107
 45360|     ;; other = i64 1
 45361|  %3108 = call i64 @llvm.umax.i64(i64 %3107, i64 1)                                                                     ;L1039<69<102<2341
 45362|  %3109 = mul i64 %3105, 100                                                                                            ;L69<102<2341
 45363|  %3110 = udiv i64 %3109, %3108                                                                                         ;L69<102<2341
 45364|     ;; hp_pct = i64 %3110
 45365|     ;; self = i64 %3105
 45366|     ;; other = i64 1
 45367|  %3111 = call i64 @llvm.umax.i64(i64 %3105, i64 1)                                                                     ;L1039<70<102<2341
 45368|  %3112 = mul i64 %3100, 100                                                                                            ;L70<102<2341
 45369|  %3113 = udiv i64 %3112, %3111                                                                                         ;L70<102<2341
 45370|     ;; damage_pct = i64 %3113
 45371|  %3114 = icmp uge i64 %3100, %3105                                                                                     ;L71<102<2341
 45372|  %3115 = icmp ugt i64 %3113, 49
 45373|  %3116 = or i1 %3114, %3115                                                                                            ;L71<102<2341
 45374|  br i1 %3116, label %3129, label %3117                                                                                 ;L71<102<2341
 45375| 
 45376| 3117: ; preds = %3103
 45377|  %3118 = icmp ult i64 %3110, 66                                                                                        ;L73<102<2341
 45378|  %3119 = icmp samesign ugt i64 %3113, 29                                                                               ;L73<102<2341
 45379|  %3120 = and i1 %3118, %3119                                                                                           ;L73<102<2341
 45380|  br i1 %3120, label %3129, label %3121                                                                                 ;L73<102<2341
 45381| 
 45382| 3121: ; preds = %3117
 45383|  %3122 = icmp ult i64 %3110, 41                                                                                        ;L74<102<2341
 45384|  %3123 = icmp samesign ugt i64 %3113, 17                                                                               ;L74<102<2341
 45385|  %3124 = and i1 %3122, %3123                                                                                           ;L74<102<2341
 45386|  br i1 %3124, label %3129, label %3125                                                                                 ;L74<102<2341
 45387| 
 45388| 3125: ; preds = %3121
 45389|  %3126 = icmp ult i64 %3110, 26                                                                                        ;L75<102<2341
 45390|  %3127 = icmp samesign ugt i64 %3113, 9
 45391|  %3128 = select i1 %3126, i1 %3127, i1 false                                                                           ;L75<102<2341
 45392|  br i1 %3128, label %3129, label %3135                                                                                 ;L102<2341
 45393| 
 45394| 3129: ; preds = %3125, %3121, %3117, %3103
 45395|  br label %3135                                                                                                        ;L102<2341
 45396| 
 45397| 3130: ; preds = %3089, %3086
 45398|  %3131 = phi i64 [ %3088, %3086 ], [ %3093, %3089 ]                                                                    ;L2339
 45399|  %3132 = load i64, ptr %373, , !!8                                                                                     ;L2338
 45400|  %3133 = load i64, ptr %374, , !!8                                                                                     ;L2338
 45401|  %3134 = invoke i64 @ai::minion_wave_risk41enemy_minion_line_action_danger_damage_at(i64 poison, ptr %4, ptr %59, i64 %3132, i64 %3133, i64 %3131, i1 zeroext false, i1 zeroext false)
 45402|  to label %3135 unwind label %365                                                                                      ;L2338
 45403| 
 45404| 3135: ; preds = %3130, %3129, %3125, %3101
 45405|  %3136 = phi i64 [ %3134, %3130 ], [ %3100, %3129 ], [ 0, %3125 ], [ 0, %3101 ]                                        ;L0
 45406|     ;; self = i64 %3136
 45407|     ;; minion_wave_damage = i64 %3136
 45408|  %3137 = call i64 @llvm.usub.sat.i64(i64 %3136, i64 %2938)                                                             ;L2472<2343
 45409|  %3138 = load i64, ptr %89, , !!8                                                                                      ;L2343
 45410|  %3139 = add i64 %3138, %3137                                                                                          ;L2343
 45411|  store i64 %3139, ptr %89,                                                                                             ;L2343
 45414|     ;; self = ptr %55
 45415|     ;; self = ptr %55
 45416|  %3140 = gep %55, i64 208                                                                                              ;L138<2073<2347
 45417|  %3141 = load ptr, ptr %3140, , !!8, !!8                                                                               ;L138<2073<2347
 45418|     ;; p = ptr %3141
 45419|  %3142 = gep %55, i64 232                                                                                              ;L2075<2347
 45420|  %3143 = load i64, ptr %3142, , !!8                                                                                    ;L2075<2347
 45421|     ;; len = i64 %3143
 45422|     ;; count = i64 %3143
 45423|     ;; self[0..+8] = ptr %3141
 45424|     ;; slice[0..+8] = ptr %3141
 45425|     ;; self[8..+8] = i64 %3143
 45426|     ;; slice[8..+8] = i64 %3143
 45427|     ;; ptr = ptr %3141
 45428|     ;; self = ptr %3141
 45429|  %3144 = getelementptr ptr, ptr %3141, i64 %3143                                                                       ;L961<100<1042<2348
 45430|     ;; self[0..+8] = ptr %3141
 45431|     ;; self[8..+8] = ptr %3144
 45432|     ;; self[16..+8] = ptr %59
 45433|  store ptr %3141, ptr %15,                                                                                             ;L69<836<2349
 45434|  %3145 = gep %15, i64 8                                                                                                ;L69<836<2349
 45435|  store ptr %3144, ptr %3145,                                                                                           ;L69<836<2349
 45436|  %3146 = gep %15, i64 16                                                                                               ;L69<836<2349
 45437|  store ptr %59, ptr %3146,                                                                                             ;L69<836<2349
 45438|  invoke void @core::iter8adapters3map3MapINtNtB29_6filter6FilterINtNtNtB2d_5slice4iter4IterBU_ENCNvNtCshdEBA0ozCnw_7game_ai15score_parameter25calculate_score_parametersD_0ENCB3M_sE_0EEB3Q_(ptr sret([32 x i8]) %16, ptr %15, ptr %65)
 45439|  to label %3147 unwind label %365                                                                                      ;L2346
 45440| 
 45441| 3147: ; preds = %3135
 45443|     ;; self = ptr %16
 45444|     ;; self = ptr %16
 45445|     ;; self = ptr %16
 45446|  %3148 = load ptr, ptr %16, , !!8, !!8                                                                                 ;L138<2073<2136<2352
 45447|     ;; p = ptr %3148
 45448|  %3149 = gep %16, i64 24                                                                                               ;L2075<2136<2352
 45449|  %3150 = load i64, ptr %3149, , !!8                                                                                    ;L2075<2136<2352
 45450|     ;; len = i64 %3150
 45451|     ;; count = i64 %3150
 45452|     ;; self[0..+8] = ptr %3148
 45453|     ;; slice[0..+8] = ptr %3148
 45454|     ;; self[8..+8] = i64 %3150
 45455|     ;; slice[8..+8] = i64 %3150
 45456|     ;; ptr = ptr %3148
 45457|     ;; self = ptr %3148
 45458|  %3151 = getelementptr ptr, ptr %3148, i64 %3150                                                                       ;L961<100<1042<2136<2352
 45459|     ;; iter[0..+8] = ptr %3148
 45460|     ;; iter[8..+8] = ptr %3151
 45461|  br label %3152                                                                                                        ;L2352
 45462| 
 45463| 3152: ; preds = %3160, %3147
 45464|  %3153 = phi ptr [ %3148, %3147 ], [ %3156, %3160 ]                                                                    ;L2352
 45465|     ;; iter[0..+8] = ptr %3153
 45466|     ;; self = ptr undef
 45467|     ;; ptr = ptr %3153
 45468|     ;; self = ptr %3153
 45469|     ;; end_or_len = ptr %3151
 45472|  %3154 = icmp eq ptr %3153, %3151                                                                                      ;L1714<180<2352
 45473|  br i1 %3154, label %3340, label %3155                                                                                 ;L180<2352
 45474| 
 45475| 3155: ; preds = %3152
 45476|  %3156 = gep %3153, i64 8                                                                                              ;L656<185<2352
 45477|     ;; iter[0..+8] = ptr %3156
 45478|     ;; j = ptr %3153
 45479|  %3157 = load ptr, ptr %3153, , !!8, !!8                                                                               ;L2353
 45480|     ;; j = ptr %3157
 45481|  %3158 = gep %3157, i64 104                                                                                            ;L2354
 45482|  %3159 = load i64, ptr %3158, , !!8                                                                                    ;L2354
 45483|  switch i64 %3159, label %3160 [
 45484|  i64 4, label %3161
 45485|  i64 5, label %3165
 45486|  i64 6, label %3169
 45487|  ]                                                                                                                     ;L2354
 45488| 
 45489| 3160: ; preds = %3336, %3328, %3286, %3278, %3234, %3210, %3169, %3165, %3161, %3155
 45490|  br label %3152                                                                                                        ;L2352
 45491| 
 45492| 3161: ; preds = %3155
 45493|     ;; info = ptr %3157
 45494|  %3162 = gep %3157, i64 136                                                                                            ;L2356
 45495|  %3163 = load i64, ptr %3162, , !!8                                                                                    ;L2356
 45496|  %3164 = trunc nuw i64 %3163 to i1                                                                                     ;L2356
 45497|  br i1 %3164, label %3173, label %3160                                                                                 ;L2356
 45498| 
 45499| 3165: ; preds = %3155
 45500|     ;; info = ptr %3157
 45501|  %3166 = gep %3157, i64 136                                                                                            ;L2371
 45502|  %3167 = load i64, ptr %3166, , !!8                                                                                    ;L2371
 45503|  %3168 = trunc nuw i64 %3167 to i1                                                                                     ;L2371
 45504|  br i1 %3168, label %3238, label %3160                                                                                 ;L2371
 45505| 
 45506| 3169: ; preds = %3155
 45507|     ;; info = ptr %3157
 45508|  %3170 = gep %3157, i64 136                                                                                            ;L2392
 45509|  %3171 = load i64, ptr %3170, , !!8                                                                                    ;L2392
 45510|  %3172 = trunc nuw i64 %3171 to i1                                                                                     ;L2392
 45511|  br i1 %3172, label %3293, label %3160                                                                                 ;L2392
 45512| 
 45513| 3173: ; preds = %3161
 45514|  %3174 = gep %3157, i64 144                                                                                            ;L2356
 45515|  %3175 = load i64, ptr %3174, , !!8                                                                                    ;L2356
 45516|     ;; id = i64 %3175
 45517|  %3176 = icmp eq i64 %3175, %157                                                                                       ;L2357
 45518|  br i1 %3176, label %3189, label %3177                                                                                 ;L2357
 45519| 
 45520| 3177: ; preds = %3199, %3173
 45521|     ;; self = ptr %47
 45522|     ;; self = ptr %47
 45523|  %3178 = load ptr, ptr %67, , !!8, !!8                                                                                 ;L138<2083<2361
 45524|     ;; ptr = ptr %3178
 45525|  %3179 = load i64, ptr %70, , !!8                                                                                      ;L2085<2361
 45526|     ;; len = i64 %3179
 45527|     ;; count = i64 %3179
 45528|     ;; self[0..+8] = ptr %3178
 45529|     ;; slice[0..+8] = ptr %3178
 45530|     ;; self[8..+8] = i64 %3179
 45531|     ;; slice[8..+8] = i64 %3179
 45532|     ;; ptr = ptr %3178
 45533|     ;; self = ptr %3178
 45534|  %3180 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %3178, i64 %3179 ;L961<240<1062<2361
 45535|     ;; predicate = ptr undef
 45536|     ;; self = ptr undef
 45537|     ;; self = ptr undef
 45538|     ;; count = i64 1
 45539|  br label %3181                                                                                                        ;L348<2361
 45540| 
 45541| 3181: ; preds = %3184, %3177
 45542|  %3182 = phi ptr [ %3185, %3184 ], [ %3178, %3177 ]
 45543|     ;; ptr = ptr %3182
 45544|     ;; self = ptr %3182
 45545|     ;; end_or_len = ptr %3180
 45548|  %3183 = icmp eq ptr %3182, %3180                                                                                      ;L1714<180<348<2361
 45549|  br i1 %3183, label %3206, label %3184                                                                                 ;L180<348<2361
 45550| 
 45551| 3184: ; preds = %3181
 45552|  %3185 = gep %3182, i64 216                                                                                            ;L656<185<348<2361
 45553|     ;; x = ptr %3182
 45556|  %3186 = gep %3182, i64 88                                                                                             ;L2361<349<2361
 45557|  %3187 = load i64, ptr %3186, , !!58264, !!8                                                                           ;L2361<349<2361
 45558|  %3188 = icmp eq i64 %3187, %3175                                                                                      ;L2361<349<2361
 45559|  br i1 %3188, label %3202, label %3181                                                                                 ;L349<2361
 45560| 
 45561| 3189: ; preds = %3173
 45562|     ;; self = ptr %3157
 45563|  %3190 = gep %3157, i64 1216                                                                                           ;L742<2358
 45564|  %3191 = load i32, ptr %3190, , !!8                                                                                    ;L742<2358
 45565|  %3192 = icmp eq i32 %3191, -1                                                                                         ;L742<2358
 45566|  br i1 %3192, label %3196, label %3193                                                                                 ;L742<2358
 45567| 
 45568| 3193: ; preds = %3189
 45569|  %3194 = gep %3157, i64 1168                                                                                           ;L742<2358
 45570|     ;; self = ptr %3194
 45571|  %3195 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %3194, ptr %64, ptr %3157, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 45572|  to label %3199 unwind label %3197                                                                                     ;L2358
 45573| 
 45574| 3196: ; preds = %3189
 45575|     ;; self = ptr null
 45576|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.166) #25
 45577|  to label %173 unwind label %3197                                                                                      ;L1013<2358
 45578| 
 45579| 3197: ; preds = %3302, %3299, %3247, %3244, %3233, %3230, %3221, %3218, %3196, %3193
 45580|  %3198 = cleanuppad within none []
 45581|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEECshdEBA0ozCnw_7game_ai(ptr %16) #27 [ "funclet"(token %3198) ] ;L2418
 45582|  cleanupret from %3198 unwind label %365                                                                               ;L2418
 45583| 
 45584| 3199: ; preds = %3193
 45585|  %3200 = load i64, ptr %89, , !!8                                                                                      ;L2358
 45586|  %3201 = add i64 %3200, %3195                                                                                          ;L2358
 45587|  store i64 %3201, ptr %89,                                                                                             ;L2358
 45588|  br label %3177                                                                                                        ;L2357
 45589| 
 45590| 3202: ; preds = %3184
 45591|     ;; target = ptr %3182
 45592|     ;; self = ptr %3157
 45593|  %3203 = gep %3157, i64 1216                                                                                           ;L742<2362
 45594|  %3204 = load i32, ptr %3203, , !!8                                                                                    ;L742<2362
 45595|  %3205 = icmp eq i32 %3204, -1                                                                                         ;L742<2362
 45596|  br i1 %3205, label %3221, label %3218                                                                                 ;L742<2362
 45597| 
 45598| 3206: ; preds = %3222, %3181
 45599|     ;; self = ptr %47
 45600|     ;; self = ptr %47
 45601|  %3207 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<2365
 45602|     ;; ptr = ptr %3207
 45603|  %3208 = load i64, ptr %74, , !!8                                                                                      ;L2085<2365
 45604|     ;; len = i64 %3208
 45605|     ;; count = i64 %3208
 45606|     ;; self[0..+8] = ptr %3207
 45607|     ;; slice[0..+8] = ptr %3207
 45608|     ;; self[8..+8] = i64 %3208
 45609|     ;; slice[8..+8] = i64 %3208
 45610|     ;; ptr = ptr %3207
 45611|     ;; self = ptr %3207
 45612|  %3209 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %3207, i64 %3208 ;L961<240<1062<2365
 45613|     ;; predicate = ptr undef
 45614|     ;; self = ptr undef
 45615|     ;; self = ptr undef
 45616|     ;; count = i64 1
 45617|  br label %3210                                                                                                        ;L348<2365
 45618| 
 45619| 3210: ; preds = %3213, %3206
 45620|  %3211 = phi ptr [ %3214, %3213 ], [ %3207, %3206 ]
 45621|     ;; ptr = ptr %3211
 45622|     ;; self = ptr %3211
 45623|     ;; end_or_len = ptr %3209
 45626|  %3212 = icmp eq ptr %3211, %3209                                                                                      ;L1714<180<348<2365
 45627|  br i1 %3212, label %3160, label %3213                                                                                 ;L180<348<2365
 45628| 
 45629| 3213: ; preds = %3210
 45630|  %3214 = gep %3211, i64 216                                                                                            ;L656<185<348<2365
 45631|     ;; x = ptr %3211
 45634|  %3215 = gep %3211, i64 88                                                                                             ;L2365<349<2365
 45635|  %3216 = load i64, ptr %3215, , !!58326, !!8                                                                           ;L2365<349<2365
 45636|  %3217 = icmp eq i64 %3216, %3175                                                                                      ;L2365<349<2365
 45637|  br i1 %3217, label %3226, label %3210                                                                                 ;L349<2365
 45638| 
 45639| 3218: ; preds = %3202
 45640|  %3219 = gep %3157, i64 1168                                                                                           ;L742<2362
 45641|     ;; self = ptr %3219
 45642|  %3220 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %3219, ptr %64, ptr %3157, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 45643|  to label %3222 unwind label %3197                                                                                     ;L2362
 45644| 
 45645| 3221: ; preds = %3202
 45646|     ;; self = ptr null
 45647|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.167) #25
 45648|  to label %173 unwind label %3197                                                                                      ;L1013<2362
 45649| 
 45650| 3222: ; preds = %3218
 45651|  %3223 = gep %3182, i64 128                                                                                            ;L2362
 45652|  %3224 = load i64, ptr %3223, , !!8                                                                                    ;L2362
 45653|  %3225 = add i64 %3224, %3220                                                                                          ;L2362
 45654|  store i64 %3225, ptr %3223,                                                                                           ;L2362
 45655|  br label %3206                                                                                                        ;L2361
 45656| 
 45657| 3226: ; preds = %3213
 45658|     ;; target = ptr %3211
 45659|     ;; self = ptr %3157
 45660|  %3227 = gep %3157, i64 1216                                                                                           ;L742<2366
 45661|  %3228 = load i32, ptr %3227, , !!8                                                                                    ;L742<2366
 45662|  %3229 = icmp eq i32 %3228, -1                                                                                         ;L742<2366
 45663|  br i1 %3229, label %3233, label %3230                                                                                 ;L742<2366
 45664| 
 45665| 3230: ; preds = %3226
 45666|  %3231 = gep %3157, i64 1168                                                                                           ;L742<2366
 45667|     ;; self = ptr %3231
 45668|  %3232 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %3231, ptr %64, ptr %3157, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 45669|  to label %3234 unwind label %3197                                                                                     ;L2366
 45670| 
 45671| 3233: ; preds = %3226
 45672|     ;; self = ptr null
 45673|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.168) #25
 45674|  to label %173 unwind label %3197                                                                                      ;L1013<2366
 45675| 
 45676| 3234: ; preds = %3230
 45677|  %3235 = gep %3211, i64 128                                                                                            ;L2366
 45678|  %3236 = load i64, ptr %3235, , !!8                                                                                    ;L2366
 45679|  %3237 = add i64 %3236, %3232                                                                                          ;L2366
 45680|  store i64 %3237, ptr %3235,                                                                                           ;L2366
 45681|  br label %3160                                                                                                        ;L2365
 45682| 
 45683| 3238: ; preds = %3165
 45684|  %3239 = gep %3157, i64 144                                                                                            ;L2371
 45685|  %3240 = load i64, ptr %3239, , !!8                                                                                    ;L2371
 45686|     ;; id = i64 %3240
 45687|     ;; self = ptr %3157
 45688|  %3241 = gep %3157, i64 1216                                                                                           ;L742<2372
 45689|  %3242 = load i32, ptr %3241, , !!8                                                                                    ;L742<2372
 45690|  %3243 = icmp eq i32 %3242, -1                                                                                         ;L742<2372
 45691|  br i1 %3243, label %3247, label %3244                                                                                 ;L742<2372
 45692| 
 45693| 3244: ; preds = %3238
 45694|  %3245 = gep %3157, i64 1168                                                                                           ;L742<2372
 45695|     ;; self = ptr %3245
 45696|  %3246 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %3245, ptr %64, ptr %3157, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 45697|  to label %3248 unwind label %3197                                                                                     ;L2372
 45698| 
 45699| 3247: ; preds = %3238
 45700|     ;; self = ptr null
 45701|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.169) #25
 45702|  to label %173 unwind label %3197                                                                                      ;L1013<2372
 45703| 
 45704| 3248: ; preds = %3244
 45705|     ;; damage = i64 %3246
 45706|  %3249 = icmp eq i64 %3240, %157                                                                                       ;L2373
 45707|  br i1 %3249, label %3262, label %3250                                                                                 ;L2373
 45708| 
 45709| 3250: ; preds = %3262, %3248
 45710|     ;; self = ptr %47
 45711|     ;; self = ptr %47
 45712|  %3251 = load ptr, ptr %67, , !!8, !!8                                                                                 ;L138<2083<2378
 45713|     ;; ptr = ptr %3251
 45714|  %3252 = load i64, ptr %70, , !!8                                                                                      ;L2085<2378
 45715|     ;; len = i64 %3252
 45716|     ;; count = i64 %3252
 45717|     ;; self[0..+8] = ptr %3251
 45718|     ;; slice[0..+8] = ptr %3251
 45719|     ;; self[8..+8] = i64 %3252
 45720|     ;; slice[8..+8] = i64 %3252
 45721|     ;; ptr = ptr %3251
 45722|     ;; self = ptr %3251
 45723|  %3253 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %3251, i64 %3252 ;L961<240<1062<2378
 45724|     ;; predicate = ptr undef
 45725|     ;; self = ptr undef
 45726|     ;; self = ptr undef
 45727|     ;; count = i64 1
 45728|  br label %3254                                                                                                        ;L348<2378
 45729| 
 45730| 3254: ; preds = %3257, %3250
 45731|  %3255 = phi ptr [ %3258, %3257 ], [ %3251, %3250 ]
 45732|     ;; ptr = ptr %3255
 45733|     ;; self = ptr %3255
 45734|     ;; end_or_len = ptr %3253
 45737|  %3256 = icmp eq ptr %3255, %3253                                                                                      ;L1714<180<348<2378
 45738|  br i1 %3256, label %3274, label %3257                                                                                 ;L180<348<2378
 45739| 
 45740| 3257: ; preds = %3254
 45741|  %3258 = gep %3255, i64 216                                                                                            ;L656<185<348<2378
 45742|     ;; x = ptr %3255
 45745|  %3259 = gep %3255, i64 88                                                                                             ;L2378<349<2378
 45746|  %3260 = load i64, ptr %3259, , !!58396, !!8                                                                           ;L2378<349<2378
 45747|  %3261 = icmp eq i64 %3260, %3240                                                                                      ;L2378<349<2378
 45748|  br i1 %3261, label %3267, label %3254                                                                                 ;L349<2378
 45749| 
 45750| 3262: ; preds = %3248
 45751|  %3263 = load i64, ptr %89, , !!8                                                                                      ;L2374
 45752|  %3264 = add i64 %3263, %3246                                                                                          ;L2374
 45753|  store i64 %3264, ptr %89,                                                                                             ;L2374
 45754|  %3265 = load i64, ptr %90, , !!8                                                                                      ;L2375
 45755|  %3266 = add i64 %3265, %3246                                                                                          ;L2375
 45756|  store i64 %3266, ptr %90,                                                                                             ;L2375
 45757|  br label %3250                                                                                                        ;L2373
 45758| 
 45759| 3267: ; preds = %3257
 45760|     ;; target = ptr %3255
 45761|  %3268 = gep %3255, i64 128                                                                                            ;L2379
 45762|  %3269 = load i64, ptr %3268, , !!8                                                                                    ;L2379
 45763|  %3270 = add i64 %3269, %3246                                                                                          ;L2379
 45764|  store i64 %3270, ptr %3268,                                                                                           ;L2379
 45765|  %3271 = gep %3255, i64 136                                                                                            ;L2380
 45766|  %3272 = load i64, ptr %3271, , !!8                                                                                    ;L2380
 45767|  %3273 = add i64 %3272, %3246                                                                                          ;L2380
 45768|  store i64 %3273, ptr %3271,                                                                                           ;L2380
 45769|  br label %3274                                                                                                        ;L2378
 45770| 
 45771| 3274: ; preds = %3267, %3254
 45772|     ;; self = ptr %47
 45773|     ;; self = ptr %47
 45774|  %3275 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<2383
 45775|     ;; ptr = ptr %3275
 45776|  %3276 = load i64, ptr %74, , !!8                                                                                      ;L2085<2383
 45777|     ;; len = i64 %3276
 45778|     ;; count = i64 %3276
 45779|     ;; self[0..+8] = ptr %3275
 45780|     ;; slice[0..+8] = ptr %3275
 45781|     ;; self[8..+8] = i64 %3276
 45782|     ;; slice[8..+8] = i64 %3276
 45783|     ;; ptr = ptr %3275
 45784|     ;; self = ptr %3275
 45785|  %3277 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %3275, i64 %3276 ;L961<240<1062<2383
 45786|     ;; predicate = ptr undef
 45787|     ;; self = ptr undef
 45788|     ;; self = ptr undef
 45789|     ;; count = i64 1
 45790|  br label %3278                                                                                                        ;L348<2383
 45791| 
 45792| 3278: ; preds = %3281, %3274
 45793|  %3279 = phi ptr [ %3282, %3281 ], [ %3275, %3274 ]
 45794|     ;; ptr = ptr %3279
 45795|     ;; self = ptr %3279
 45796|     ;; end_or_len = ptr %3277
 45799|  %3280 = icmp eq ptr %3279, %3277                                                                                      ;L1714<180<348<2383
 45800|  br i1 %3280, label %3160, label %3281                                                                                 ;L180<348<2383
 45801| 
 45802| 3281: ; preds = %3278
 45803|  %3282 = gep %3279, i64 216                                                                                            ;L656<185<348<2383
 45804|     ;; x = ptr %3279
 45807|  %3283 = gep %3279, i64 88                                                                                             ;L2383<349<2383
 45808|  %3284 = load i64, ptr %3283, , !!58454, !!8                                                                           ;L2383<349<2383
 45809|  %3285 = icmp eq i64 %3284, %3240                                                                                      ;L2383<349<2383
 45810|  br i1 %3285, label %3286, label %3278                                                                                 ;L349<2383
 45811| 
 45812| 3286: ; preds = %3281
 45813|     ;; target = ptr %3279
 45814|  %3287 = gep %3279, i64 128                                                                                            ;L2384
 45815|  %3288 = load i64, ptr %3287, , !!8                                                                                    ;L2384
 45816|  %3289 = add i64 %3288, %3246                                                                                          ;L2384
 45817|  store i64 %3289, ptr %3287,                                                                                           ;L2384
 45818|  %3290 = gep %3279, i64 136                                                                                            ;L2385
 45819|  %3291 = load i64, ptr %3290, , !!8                                                                                    ;L2385
 45820|  %3292 = add i64 %3291, %3246                                                                                          ;L2385
 45821|  store i64 %3292, ptr %3290,                                                                                           ;L2385
 45822|  br label %3160                                                                                                        ;L2383
 45823| 
 45824| 3293: ; preds = %3169
 45825|  %3294 = gep %3157, i64 144                                                                                            ;L2392
 45826|  %3295 = load i64, ptr %3294, , !!8                                                                                    ;L2392
 45827|     ;; id = i64 %3295
 45828|     ;; self = ptr %3157
 45829|  %3296 = gep %3157, i64 1216                                                                                           ;L742<2393
 45830|  %3297 = load i32, ptr %3296, , !!8                                                                                    ;L742<2393
 45831|  %3298 = icmp eq i32 %3297, -1                                                                                         ;L742<2393
 45832|  br i1 %3298, label %3302, label %3299                                                                                 ;L742<2393
 45833| 
 45834| 3299: ; preds = %3293
 45835|  %3300 = gep %3157, i64 1168                                                                                           ;L742<2393
 45836|     ;; self = ptr %3300
 45837|  %3301 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %3300, ptr %64, ptr %3157, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.11, ptr %59)
 45838|  to label %3303 unwind label %3197                                                                                     ;L2393
 45839| 
 45840| 3302: ; preds = %3293
 45841|     ;; self = ptr null
 45842|  invoke void @core::option13unwrap_failed(ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.170) #25
 45843|  to label %173 unwind label %3197                                                                                      ;L1013<2393
 45844| 
 45845| 3303: ; preds = %3299
 45846|     ;; damage = i64 %3301
 45847|  %3304 = icmp eq i64 %3295, %157                                                                                       ;L2394
 45848|  br i1 %3304, label %3317, label %3305                                                                                 ;L2394
 45849| 
 45850| 3305: ; preds = %3317, %3303
 45851|     ;; self = ptr %47
 45852|     ;; self = ptr %47
 45853|  %3306 = load ptr, ptr %67, , !!8, !!8                                                                                 ;L138<2083<2398
 45854|     ;; ptr = ptr %3306
 45855|  %3307 = load i64, ptr %70, , !!8                                                                                      ;L2085<2398
 45856|     ;; len = i64 %3307
 45857|     ;; count = i64 %3307
 45858|     ;; self[0..+8] = ptr %3306
 45859|     ;; slice[0..+8] = ptr %3306
 45860|     ;; self[8..+8] = i64 %3307
 45861|     ;; slice[8..+8] = i64 %3307
 45862|     ;; ptr = ptr %3306
 45863|     ;; self = ptr %3306
 45864|  %3308 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %3306, i64 %3307 ;L961<240<1062<2398
 45865|     ;; predicate = ptr undef
 45866|     ;; self = ptr undef
 45867|     ;; self = ptr undef
 45868|     ;; count = i64 1
 45869|  br label %3309                                                                                                        ;L348<2398
 45870| 
 45871| 3309: ; preds = %3312, %3305
 45872|  %3310 = phi ptr [ %3313, %3312 ], [ %3306, %3305 ]
 45873|     ;; ptr = ptr %3310
 45874|     ;; self = ptr %3310
 45875|     ;; end_or_len = ptr %3308
 45878|  %3311 = icmp eq ptr %3310, %3308                                                                                      ;L1714<180<348<2398
 45879|  br i1 %3311, label %3324, label %3312                                                                                 ;L180<348<2398
 45880| 
 45881| 3312: ; preds = %3309
 45882|  %3313 = gep %3310, i64 216                                                                                            ;L656<185<348<2398
 45883|     ;; x = ptr %3310
 45886|  %3314 = gep %3310, i64 88                                                                                             ;L2398<349<2398
 45887|  %3315 = load i64, ptr %3314, , !!58518, !!8                                                                           ;L2398<349<2398
 45888|  %3316 = icmp eq i64 %3315, %3295                                                                                      ;L2398<349<2398
 45889|  br i1 %3316, label %3320, label %3309                                                                                 ;L349<2398
 45890| 
 45891| 3317: ; preds = %3303
 45892|  %3318 = load i64, ptr %90, , !!8                                                                                      ;L2395
 45893|  %3319 = add i64 %3318, %3301                                                                                          ;L2395
 45894|  store i64 %3319, ptr %90,                                                                                             ;L2395
 45895|  br label %3305                                                                                                        ;L2394
 45896| 
 45897| 3320: ; preds = %3312
 45898|     ;; target = ptr %3310
 45899|  %3321 = gep %3310, i64 136                                                                                            ;L2399
 45900|  %3322 = load i64, ptr %3321, , !!8                                                                                    ;L2399
 45901|  %3323 = add i64 %3322, %3301                                                                                          ;L2399
 45902|  store i64 %3323, ptr %3321,                                                                                           ;L2399
 45903|  br label %3324                                                                                                        ;L2398
 45904| 
 45905| 3324: ; preds = %3320, %3309
 45906|     ;; self = ptr %47
 45907|     ;; self = ptr %47
 45908|  %3325 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<2402
 45909|     ;; ptr = ptr %3325
 45910|  %3326 = load i64, ptr %74, , !!8                                                                                      ;L2085<2402
 45911|     ;; len = i64 %3326
 45912|     ;; count = i64 %3326
 45913|     ;; self[0..+8] = ptr %3325
 45914|     ;; slice[0..+8] = ptr %3325
 45915|     ;; self[8..+8] = i64 %3326
 45916|     ;; slice[8..+8] = i64 %3326
 45917|     ;; ptr = ptr %3325
 45918|     ;; self = ptr %3325
 45919|  %3327 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %3325, i64 %3326 ;L961<240<1062<2402
 45920|     ;; predicate = ptr undef
 45921|     ;; self = ptr undef
 45922|     ;; self = ptr undef
 45923|     ;; count = i64 1
 45924|  br label %3328                                                                                                        ;L348<2402
 45925| 
 45926| 3328: ; preds = %3331, %3324
 45927|  %3329 = phi ptr [ %3332, %3331 ], [ %3325, %3324 ]
 45928|     ;; ptr = ptr %3329
 45929|     ;; self = ptr %3329
 45930|     ;; end_or_len = ptr %3327
 45933|  %3330 = icmp eq ptr %3329, %3327                                                                                      ;L1714<180<348<2402
 45934|  br i1 %3330, label %3160, label %3331                                                                                 ;L180<348<2402
 45935| 
 45936| 3331: ; preds = %3328
 45937|  %3332 = gep %3329, i64 216                                                                                            ;L656<185<348<2402
 45938|     ;; x = ptr %3329
 45941|  %3333 = gep %3329, i64 88                                                                                             ;L2402<349<2402
 45942|  %3334 = load i64, ptr %3333, , !!58574, !!8                                                                           ;L2402<349<2402
 45943|  %3335 = icmp eq i64 %3334, %3295                                                                                      ;L2402<349<2402
 45944|  br i1 %3335, label %3336, label %3328                                                                                 ;L349<2402
 45945| 
 45946| 3336: ; preds = %3331
 45947|     ;; target = ptr %3329
 45948|  %3337 = gep %3329, i64 136                                                                                            ;L2403
 45949|  %3338 = load i64, ptr %3337, , !!8                                                                                    ;L2403
 45950|  %3339 = add i64 %3338, %3301                                                                                          ;L2403
 45951|  store i64 %3339, ptr %3337,                                                                                           ;L2403
 45952|  br label %3160                                                                                                        ;L2402
 45953| 
 45954| 3340: ; preds = %3152
 45955|  %3341 = load i64, ptr %373, , !!8                                                                                     ;L2412
 45956|     ;; coord = i64 %3341
 45957|  %3342 = udiv i64 %3341, 32000                                                                                         ;L14<2412
 45958|     ;; self = i64 %3342
 45959|     ;; other = i64 29
 45960|  %3343 = call i64 @llvm.umin.i64(i64 %3342, i64 29)                                                                    ;L1078<14<2412
 45961|  store i64 %3343, ptr %141,                                                                                            ;L2412
 45962|  %3344 = load i64, ptr %374, , !!8                                                                                     ;L2413
 45963|     ;; coord = i64 %3344
 45964|  %3345 = udiv i64 %3344, 32000                                                                                         ;L14<2413
 45965|     ;; self = i64 %3345
 45966|     ;; other = i64 29
 45967|  %3346 = call i64 @llvm.umin.i64(i64 %3345, i64 29)                                                                    ;L1078<14<2413
 45968|  store i64 %3346, ptr %142,                                                                                            ;L2413
 45972|     ;; self = ptr %47
 45977|     ;; count = i64 1
 45978|     ;; count = i64 1
 45979|     ;; self = ptr %47
 45980|  %3347 = load i64, ptr %89, , !!8                                                                                      ;L1444<376<2415
 45981|  %3348 = lshr i64 %3347, 1                                                                                             ;L1444<376<2415
 45982|  store i64 %3348, ptr %89,                                                                                             ;L1444<376<2415
 45983|     ;; self = ptr %47
 45984|     ;; self = ptr %47
 45985|  %3349 = load ptr, ptr %67, , !!8, !!8                                                                                 ;L138<2083<378<2415
 45986|     ;; ptr = ptr %3349
 45987|  %3350 = load i64, ptr %70, , !!8                                                                                      ;L2085<378<2415
 45988|     ;; len = i64 %3350
 45989|     ;; count = i64 %3350
 45990|     ;; self[0..+8] = ptr %3349
 45991|     ;; slice[0..+8] = ptr %3349
 45992|     ;; self[8..+8] = i64 %3350
 45993|     ;; slice[8..+8] = i64 %3350
 45994|     ;; ptr = ptr %3349
 45995|     ;; self = ptr %3349
 45996|  %3351 = mul nuw nsw i64 %3350, 216                                                                                    ;L961<240<1062<378<2415
 45997|  %3352 = gep %3349, i64 %3351                                                                                          ;L961<240<1062<378<2415
 45998|     ;; iter[0..+8] = ptr %3349
 45999|     ;; iter[8..+8] = ptr %3352
 46000|     ;; self = ptr undef
 46001|     ;; ptr = ptr %3349
 46002|     ;; self = ptr %3349
 46003|     ;; end_or_len = ptr %3352
 46006|  %3353 = icmp eq i64 %3350, 0                                                                                          ;L1714<180<378<2415
 46007|  br i1 %3353, label %3361, label %3354                                                                                 ;L180<378<2415
 46008| 
 46009| 3354: ; preds = %3354, %3340
 46010|  %3355 = phi ptr [ %3356, %3354 ], [ %3349, %3340 ]
 46011|  %3356 = gep %3355, i64 216                                                                                            ;L656<185<378<2415
 46012|     ;; iter[0..+8] = ptr %3356
 46013|     ;; a = ptr %3355
 46014|     ;; self = ptr %3355
 46015|  %3357 = gep %3355, i64 128                                                                                            ;L1444<379<2415
 46016|  %3358 = load i64, ptr %3357, , !!58599, !!8                                                                           ;L1444<379<2415
 46017|  %3359 = lshr i64 %3358, 1                                                                                             ;L1444<379<2415
 46018|  store i64 %3359, ptr %3357, , !!58599                                                                                 ;L1444<379<2415
 46019|     ;; self = ptr undef
 46020|     ;; ptr = ptr %3356
 46021|     ;; self = ptr %3356
 46022|     ;; end_or_len = ptr %3352
 46025|  %3360 = icmp eq ptr %3356, %3352                                                                                      ;L1714<180<378<2415
 46026|  br i1 %3360, label %3361, label %3354                                                                                 ;L180<378<2415
 46027| 
 46028| 3361: ; preds = %3354, %3340
 46029|     ;; self = ptr %47
 46030|     ;; self = ptr %47
 46031|  %3362 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<382<2415
 46032|     ;; ptr = ptr %3362
 46033|  %3363 = load i64, ptr %74, , !!8                                                                                      ;L2085<382<2415
 46034|     ;; len = i64 %3363
 46035|     ;; count = i64 %3363
 46036|     ;; self[0..+8] = ptr %3362
 46037|     ;; slice[0..+8] = ptr %3362
 46038|     ;; self[8..+8] = i64 %3363
 46039|     ;; slice[8..+8] = i64 %3363
 46040|     ;; ptr = ptr %3362
 46041|     ;; self = ptr %3362
 46042|  %3364 = mul nuw nsw i64 %3363, 216                                                                                    ;L961<240<1062<382<2415
 46043|  %3365 = gep %3362, i64 %3364                                                                                          ;L961<240<1062<382<2415
 46044|     ;; iter[0..+8] = ptr %3362
 46045|     ;; iter[8..+8] = ptr %3365
 46046|     ;; self = ptr undef
 46047|     ;; ptr = ptr %3362
 46048|     ;; self = ptr %3362
 46049|     ;; end_or_len = ptr %3365
 46052|  %3366 = icmp eq i64 %3363, 0                                                                                          ;L1714<180<382<2415
 46053|  br i1 %3366, label %3374, label %3367                                                                                 ;L180<382<2415
 46054| 
 46055| 3367: ; preds = %3367, %3361
 46056|  %3368 = phi ptr [ %3369, %3367 ], [ %3362, %3361 ]
 46057|  %3369 = gep %3368, i64 216                                                                                            ;L656<185<382<2415
 46058|     ;; iter[0..+8] = ptr %3369
 46059|     ;; e = ptr %3368
 46060|     ;; self = ptr %3368
 46061|  %3370 = gep %3368, i64 128                                                                                            ;L1444<383<2415
 46062|  %3371 = load i64, ptr %3370, , !!58599, !!8                                                                           ;L1444<383<2415
 46063|  %3372 = lshr i64 %3371, 1                                                                                             ;L1444<383<2415
 46064|  store i64 %3372, ptr %3370, , !!58599                                                                                 ;L1444<383<2415
 46065|     ;; self = ptr undef
 46066|     ;; ptr = ptr %3369
 46067|     ;; self = ptr %3369
 46068|     ;; end_or_len = ptr %3365
 46071|  %3373 = icmp eq ptr %3369, %3365                                                                                      ;L1714<180<382<2415
 46072|  br i1 %3373, label %3374, label %3367                                                                                 ;L180<382<2415
 46073| 
 46074| 3374: ; preds = %3367, %3361
 46075|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %47, i64 5384, i1 false)                                                 ;L2417
 46077|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %16)
 46078|  to label %3378 unwind label %3375                                                                                     ;L825<2418
 46079| 
 46080| 3375: ; preds = %3374
 46081|  %3376 = cleanuppad within none []
 46083|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %16) [ "funclet"(token %3376) ]
 46084|  to label %3377 unwind label %365                                                                                      ;L825<825<2418
 46085| 
 46086| 3377: ; preds = %3375
 46087|  cleanupret from %3376 unwind label %365
 46088| 
 46089| 3378: ; preds = %3374
 46091|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %16)
 46092|  to label %3379 unwind label %365                                                                                      ;L825<825<2418
 46093| 
 46094| 3379: ; preds = %3378
 46097|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %31)
 46098|  to label %3383 unwind label %3380                                                                                     ;L825<2418
 46099| 
 46100| 3380: ; preds = %3379
 46101|  %3381 = cleanuppad within none []
 46103|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %31) [ "funclet"(token %3381) ]
 46104|  to label %3382 unwind label %241                                                                                      ;L825<825<2418
 46105| 
 46106| 3382: ; preds = %3380
 46107|  cleanupret from %3381 unwind label %241
 46108| 
 46109| 3383: ; preds = %3379
 46111|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %31)
 46112|  to label %3384 unwind label %241                                                                                      ;L825<825<2418
 46113| 
 46114| 3384: ; preds = %3383
 46117|  invoke void @gc::simulation4game10blackboard11SmallActionRNtNtB14_6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %39)
 46118|  to label %3388 unwind label %3385                                                                                     ;L825<2418
 46119| 
 46120| 3385: ; preds = %3384
 46121|  %3386 = cleanuppad within none []
 46123|  invoke void @gc::simulation4game10blackboard11SmallActionRNtNtB1b_6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %39) [ "funclet"(token %3386) ]
 46124|  to label %3387 unwind label %189                                                                                      ;L825<825<2418
 46125| 
 46126| 3387: ; preds = %3385
 46127|  cleanupret from %3386 unwind label %189
 46128| 
 46129| 3388: ; preds = %3384
 46131|  invoke void @gc::simulation4game10blackboard11SmallActionRNtNtB1b_6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %39)
 46132|  to label %3389 unwind label %189                                                                                      ;L825<825<2418
 46133| 
 46134| 3389: ; preds = %3388
 46137|  invoke void @gc::simulation4game10blackboard11SmallActionRNtNtB14_6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %41)
 46138|  to label %3393 unwind label %3390                                                                                     ;L825<2418
 46139| 
 46140| 3390: ; preds = %3389
 46141|  %3391 = cleanuppad within none []
 46143|  invoke void @gc::simulation4game10blackboard11SmallActionRNtNtB1b_6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %41) [ "funclet"(token %3391) ]
 46144|  to label %3392 unwind label %158                                                                                      ;L825<825<2418
 46145| 
 46146| 3392: ; preds = %3390
 46147|  cleanupret from %3391 unwind label %158
 46148| 
 46149| 3393: ; preds = %3389
 46151|  invoke void @gc::simulation4game10blackboard11SmallActionRNtNtB1b_6entity6EntityEENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %41)
 46152|  to label %3394 unwind label %158                                                                                      ;L825<825<2418
 46153| 
 46154| 3394: ; preds = %3393
 46157|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %46)
 46158|  to label %3398 unwind label %3395                                                                                     ;L825<2418
 46159| 
 46160| 3395: ; preds = %3394
 46161|  %3396 = cleanuppad within none []
 46163|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %46) [ "funclet"(token %3396) ]
 46164|  to label %3397 unwind label %146                                                                                      ;L825<825<2418
 46165| 
 46166| 3397: ; preds = %3395
 46167|  cleanupret from %3396 unwind label %146
 46168| 
 46169| 3398: ; preds = %3394
 46171|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %46)
 46172|  to label %3399 unwind label %146                                                                                      ;L825<825<2418
 46173| 
 46174| 3399: ; preds = %3398
 46177|  ret void                                                                                                              ;L2418
 46178| 
 46179| 3400: ; preds = %381
 46180|     ;; caster = ptr %385
 46181|     ;; nearest_other_distance[0..+8] = i64 0
 46182|     ;; nearest_other_distance[8..+8] = i64 undef
 46183|  %3401 = icmp eq ptr %385, null                                                                                        ;L1597
 46184|  br i1 %3401, label %3591, label %3402                                                                                 ;L1597
 46185| 
 46186| 3402: ; preds = %3400
 46187|     ;; caster = ptr %385
 46188|     ;; self = ptr %47
 46189|     ;; self = ptr %47
 46190|  %3403 = load ptr, ptr %67, , !!8, !!8                                                                                 ;L138<2083<1598
 46191|     ;; ptr = ptr %3403
 46192|  %3404 = load i64, ptr %70, , !!8                                                                                      ;L2085<1598
 46193|     ;; len = i64 %3404
 46194|     ;; count = i64 %3404
 46195|     ;; self[0..+8] = ptr %3403
 46196|     ;; slice[0..+8] = ptr %3403
 46197|     ;; self[8..+8] = i64 %3404
 46198|     ;; slice[8..+8] = i64 %3404
 46199|     ;; ptr = ptr %3403
 46200|     ;; self = ptr %3403
 46201|  %3405 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %3403, i64 %3404 ;L961<240<1062<1598
 46202|     ;; iter[0..+8] = ptr %3403
 46203|     ;; iter[8..+8] = ptr %3405
 46204|  %3406 = gep %378, i64 300
 46205|  %3407 = gep %378, i64 64
 46206|  %3408 = gep %378, i64 72
 46207|  %3409 = gep %378, i64 80
 46208|  %3410 = gep %378, i64 256
 46209|  %3411 = gep %378, i64 264
 46210|  br label %3438                                                                                                        ;L1598
 46211| 
 46212| 3412: ; preds = %3451, %3438
 46213|  %3413 = phi ptr [ %3416, %3451 ], [ %3441, %3438 ]                                                                    ;L1598
 46214|     ;; iter[0..+8] = ptr %3413
 46215|     ;; nearest_other_distance[0..+8] = i64 %3440
 46216|     ;; nearest_other_distance[8..+8] = i64 %3439
 46217|     ;; self = ptr undef
 46218|     ;; ptr = ptr %3413
 46219|     ;; self = ptr %3413
 46220|     ;; end_or_len = ptr %3405
 46223|  %3414 = icmp eq ptr %3413, %3405                                                                                      ;L1714<180<1598
 46224|  br i1 %3414, label %3420, label %3415                                                                                 ;L180<1598
 46225| 
 46226| 3415: ; preds = %3412
 46227|  %3416 = gep %3413, i64 216                                                                                            ;L656<185<1598
 46228|     ;; iter[0..+8] = ptr %3416
 46229|     ;; a = ptr %3413
 46230|  %3417 = gep %3413, i64 88                                                                                             ;L1599
 46231|  %3418 = load i64, ptr %3417, , !!8                                                                                    ;L1599
 46232|  %3419 = invoke ptr %384(ptr %177, i64 %3418)
 46233|  to label %3424 unwind label %365                                                                                      ;L1599
 46234| 
 46235| 3420: ; preds = %3412
 46236|     ;; self = ptr %47
 46237|     ;; self = ptr %47
 46238|  %3421 = load ptr, ptr %71, , !!8, !!8                                                                                 ;L138<2083<1629
 46239|     ;; ptr = ptr %3421
 46240|  %3422 = load i64, ptr %74, , !!8                                                                                      ;L2085<1629
 46241|     ;; len = i64 %3422
 46242|     ;; count = i64 %3422
 46243|     ;; self[0..+8] = ptr %3421
 46244|     ;; slice[0..+8] = ptr %3421
 46245|     ;; self[8..+8] = i64 %3422
 46246|     ;; slice[8..+8] = i64 %3422
 46247|     ;; ptr = ptr %3421
 46248|     ;; self = ptr %3421
 46249|  %3423 = getelementptr { { i64, [2 x i64] }, { { ptr, ptr, i64 }, i64 }, { { ptr, ptr, i64 }, i64 }, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }, ptr %3421, i64 %3422 ;L961<240<1062<1629
 46250|     ;; iter[0..+8] = ptr %3421
 46251|     ;; iter[8..+8] = ptr %3423
 46252|  br label %3512                                                                                                        ;L1629
 46253| 
 46254| 3424: ; preds = %3415
 46255|  %3425 = icmp eq ptr %3419, null                                                                                       ;L1599
 46256|  br i1 %3425, label %3435, label %3426                                                                                 ;L1599
 46257| 
 46258| 3426: ; preds = %3424
 46259|     ;; ae = ptr %3419
 46260|     ;; self = ptr %3419
 46261|  %3427 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget16check_projectile(ptr %3406, ptr %378, ptr %3419)
 46262|  to label %3442 unwind label %365                                                                                      ;L1600
 46263| 
 46264| 3428: ; preds = %3511, %3505
 46265|  %3429 = phi i64 [ 120, %3505 ], [ 144, %3511 ]
 46266|  %3430 = phi i64 [ %3439, %3505 ], [ %3490, %3511 ]
 46267|  %3431 = phi i64 [ %3440, %3505 ], [ 1, %3511 ]
 46268|  %3432 = gep %3413, i64 %3429                                                                                          ;L0
 46269|  %3433 = load i64, ptr %3432, , !!8                                                                                    ;L0
 46270|  %3434 = add i64 %3433, 1                                                                                              ;L0
 46271|  store i64 %3434, ptr %3432,                                                                                           ;L0
 46272|  br label %3435                                                                                                        ;L1598
 46273| 
 46274| 3435: ; preds = %3511, %3505, %3492, %3428, %3424
 46275|  %3436 = phi i64 [ %3490, %3511 ], [ %3439, %3492 ], [ %3439, %3505 ], [ %3439, %3424 ], [ %3430, %3428 ]              ;L0
 46276|  %3437 = phi i64 [ 1, %3511 ], [ %3440, %3492 ], [ %3440, %3505 ], [ %3440, %3424 ], [ %3431, %3428 ]                  ;L0
 46277|     ;; nearest_other_distance[0..+8] = i64 %3437
 46278|     ;; nearest_other_distance[8..+8] = i64 %3436
 46279|  br label %3438                                                                                                        ;L1598
 46280| 
 46281| 3438: ; preds = %3435, %3402
 46282|  %3439 = phi i64 [ %3436, %3435 ], [ undef, %3402 ]
 46283|  %3440 = phi i64 [ %3437, %3435 ], [ 0, %3402 ]
 46284|  %3441 = phi ptr [ %3416, %3435 ], [ %3403, %3402 ]
 46285|  br label %3412                                                                                                        ;L180<1598
 46286| 
 46287| 3442: ; preds = %3426
 46288|  br i1 %3427, label %3443, label %3451                                                                                 ;L1600
 46289| 
 46290| 3443: ; preds = %3442
 46291|  %3444 = gep %3419, i64 1632                                                                                           ;L1604
 46292|  %3445 = load i64, ptr %3444, , !!8                                                                                    ;L1604
 46293|     ;; x2 = i64 %3445
 46294|     ;; other = i64 %3445
 46295|  %3446 = gep %3419, i64 1640                                                                                           ;L1604
 46296|  %3447 = load i64, ptr %3446, , !!8                                                                                    ;L1604
 46297|     ;; y2 = i64 %3447
 46298|     ;; other = i64 %3447
 46299|  %3448 = gep %3419, i64 1136                                                                                           ;L1511<1604
 46300|  %3449 = load i32, ptr %3448, , !!8                                                                                    ;L1511<1604
 46301|     ;; mult = i32 %3449
 46302|  %3450 = icmp eq i32 %3449, 0                                                                                          ;L1512<1604
 46303|  br i1 %3450, label %3452, label %3455                                                                                 ;L1512<1604
 46304| 
 46305| 3451: ; preds = %3465, %3442
 46306|  br label %3412                                                                                                        ;L1
 46307| 
 46308| 3452: ; preds = %3443
 46309|  %3453 = gep %3419, i64 1664                                                                                           ;L1513<1604
 46310|  %3454 = load i64, ptr %3453, , !!8                                                                                    ;L1513<1604
 46311|  br label %3462                                                                                                        ;L1512<1604
 46312| 
 46313| 3455: ; preds = %3443
 46314|  %3456 = sext i32 %3449 to i64                                                                                         ;L1511<1604
 46315|     ;; mult = i64 %3456
 46316|  %3457 = gep %3419, i64 1664                                                                                           ;L1515<1604
 46317|  %3458 = load i64, ptr %3457, , !!8                                                                                    ;L1515<1604
 46318|  %3459 = add nsw i64 %3456, 100                                                                                        ;L1515<1604
 46319|  %3460 = mul i64 %3458, %3459                                                                                          ;L1515<1604
 46320|  %3461 = udiv i64 %3460, 100                                                                                           ;L1515<1604
 46321|  br label %3462                                                                                                        ;L1512<1604
 46322| 
 46323| 3462: ; preds = %3455, %3452
 46324|  %3463 = phi i64 [ %3454, %3452 ], [ %3461, %3455 ]                                                                    ;L0<1604
 46325|  %3464 = invoke zeroext i1 @gc::simulation10projectileNtB5_10Projectile11is_in_orbit(ptr %378, i64 %3445, i64 %3447, i64 %3463)
 46326|  to label %3465 unwind label %365                                                                                      ;L1604
 46327| 
 46328| 3465: ; preds = %3462
 46329|  br i1 %3464, label %3466, label %3451                                                                                 ;L1604
 46330| 
 46331| 3466: ; preds = %3465
 46332|     ;; self = ptr %378
 46333|  %3467 = load i64, ptr %3407, , !!8                                                                                    ;L134<1608
 46334|  %3468 = icmp ne i64 %3467, 9                                                                                          ;L134<1608
 46335|  call void @llvm.assume(i1 %3468)                                                                                      ;L134<1608
 46336|  %3469 = add nsw i64 %3467, -2                                                                                         ;L134<1608
 46337|  %3470 = icmp samesign ugt i64 %3467, 1                                                                                ;L134<1608
 46338|  %3471 = select i1 %3470, i64 %3469, i64 7                                                                             ;L134<1608
 46339|  switch i64 %3471, label %3474 [
 46340|  i64 4, label %3492
 46341|  i64 5, label %3492
 46342|  i64 7, label %3472
 46343|  ]                                                                                                                     ;L134<1608
 46344| 
 46345| 3472: ; preds = %3466
 46346|  %3473 = icmp eq i64 %3467, 1                                                                                          ;L134<1608
 46347|  br i1 %3473, label %3492, label %3474                                                                                 ;L1608
 46348| 
 46349| 3474: ; preds = %3472, %3466
 46350|  %3475 = load i64, ptr %3410, , !!8                                                                                    ;L1616
 46351|     ;; x1 = i64 %3475
 46352|     ;; self = i64 %3475
 46353|  %3476 = load i64, ptr %3411, , !!8                                                                                    ;L1616
 46354|     ;; y1 = i64 %3476
 46355|     ;; self = i64 %3476
 46356|  %3477 = icmp ult i64 %3475, %3445                                                                                     ;L3147<7<1616
 46357|  %3478 = sub nuw i64 %3445, %3475                                                                                      ;L3147<7<1616
 46358|  %3479 = sub nuw i64 %3475, %3445                                                                                      ;L3147<7<1616
 46359|  %3480 = select i1 %3477, i64 %3478, i64 %3479                                                                         ;L3147<7<1616
 46360|     ;; dx = i64 %3480
 46361|  %3481 = icmp ult i64 %3476, %3447                                                                                     ;L3147<8<1616
 46362|  %3482 = sub nuw i64 %3447, %3476                                                                                      ;L3147<8<1616
 46363|  %3483 = sub nuw i64 %3476, %3447                                                                                      ;L3147<8<1616
 46364|  %3484 = select i1 %3481, i64 %3482, i64 %3483                                                                         ;L3147<8<1616
 46365|     ;; dy = i64 %3484
 46366|  %3485 = mul i64 %3480, %3480                                                                                          ;L9<1616
 46367|  %3486 = mul i64 %3484, %3484                                                                                          ;L9<1616
 46368|  %3487 = add i64 %3486, %3485                                                                                          ;L9<1616
 46369|     ;; dist = i64 %3487
 46370|     ;; self[0..+8] = i64 %3440
 46371|     ;; self[8..+8] = i64 %3439
 46372|     ;; f = ptr undef
 46373|  %3488 = trunc nuw i64 %3440 to i1                                                                                     ;L708<1617
 46374|  %3489 = call i64 @llvm.umin.i64(i64 %3487, i64 %3439)                                                                 ;L708<1617
 46375|  %3490 = select i1 %3488, i64 %3489, i64 %3487                                                                         ;L708<1617
 46376|     ;; nearest_other_distance[0..+8] = i64 1
 46377|     ;; nearest_other_distance[8..+8] = i64 %3490
 46378|  %3491 = invoke i64 @gc::simulation10projectileNtB5_10Projectile22expected_damage_target(ptr %378, ptr %64, ptr %385, ptr %3419)
 46379|  to label %3506 unwind label %365                                                                                      ;L1621
 46380| 
 46381| 3492: ; preds = %3472, %3466, %3466
 46382|  %3493 = phi ptr [ %3408, %3472 ], [ %3409, %3466 ], [ %3409, %3466 ]
 46383|  %3494 = gep %3419, i64 1472                                                                                           ;L1609
 46384|  %3495 = load i64, ptr %3494, , !!8                                                                                    ;L1609
 46385|  %3496 = load i64, ptr %3493, , !!8                                                                                    ;L0<1609
 46386|  %3497 = icmp eq i64 %3496, %3495                                                                                      ;L0<1609
 46387|  br i1 %3497, label %3498, label %3435                                                                                 ;L1609
 46388| 
 46389| 3498: ; preds = %3492
 46390|  %3499 = invoke i64 @gc::simulation10projectileNtB5_10Projectile22expected_damage_target(ptr %378, ptr %64, ptr %385, ptr %3419)
 46391|  to label %3500 unwind label %365                                                                                      ;L1610
 46392| 
 46393| 3500: ; preds = %3498
 46394|  %3501 = gep %3413, i64 112                                                                                            ;L1610
 46395|  %3502 = load i64, ptr %3501, , !!8                                                                                    ;L1610
 46396|  %3503 = add i64 %3502, %3499                                                                                          ;L1610
 46397|  store i64 %3503, ptr %3501,                                                                                           ;L1610
 46398|  %3504 = invoke zeroext i1 @gc::simulation10projectileNtB5_10Projectile6has_cc(ptr %378)
 46399|  to label %3505 unwind label %365                                                                                      ;L1611
 46400| 
 46401| 3505: ; preds = %3500
 46402|  br i1 %3504, label %3428, label %3435                                                                                 ;L1611
 46403| 
 46404| 3506: ; preds = %3474
 46405|  %3507 = gep %3413, i64 128                                                                                            ;L1621
 46406|  %3508 = load i64, ptr %3507, , !!8                                                                                    ;L1621
 46407|  %3509 = add i64 %3508, %3491                                                                                          ;L1621
 46408|  store i64 %3509, ptr %3507,                                                                                           ;L1621
 46409|  %3510 = invoke zeroext i1 @gc::simulation10projectileNtB5_10Projectile6has_cc(ptr %378)
 46410|  to label %3511 unwind label %365                                                                                      ;L1622
 46411| 
 46412| 3511: ; preds = %3506
 46413|  br i1 %3510, label %3428, label %3435                                                                                 ;L1622
 46414| 
 46415| 3512: ; preds = %3540, %3420
 46416|  %3513 = phi ptr [ %3421, %3420 ], [ %3516, %3540 ]                                                                    ;L1629
 46417|     ;; iter[0..+8] = ptr %3513
 46418|     ;; self = ptr undef
 46419|     ;; ptr = ptr %3513
 46420|     ;; self = ptr %3513
 46421|     ;; end_or_len = ptr %3423
 46424|  %3514 = icmp eq ptr %3513, %3423                                                                                      ;L1714<180<1629
 46425|  br i1 %3514, label %3520, label %3515                                                                                 ;L180<1629
 46426| 
 46427| 3515: ; preds = %3512
 46428|  %3516 = gep %3513, i64 216                                                                                            ;L656<185<1629
 46429|     ;; iter[0..+8] = ptr %3516
 46430|     ;; a = ptr %3513
 46431|  %3517 = gep %3513, i64 88                                                                                             ;L1630
 46432|  %3518 = load i64, ptr %3517, , !!8                                                                                    ;L1630
 46433|  %3519 = invoke ptr %384(ptr %177, i64 %3518)
 46434|  to label %3522 unwind label %365                                                                                      ;L1630
 46435| 
 46436| 3520: ; preds = %3512
 46437|  %3521 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget16check_projectile(ptr %3406, ptr %378, ptr %59)
 46438|  to label %3585 unwind label %365                                                                                      ;L1655
 46439| 
 46440| 3522: ; preds = %3515
 46441|  %3523 = icmp eq ptr %3519, null                                                                                       ;L1630
 46442|  br i1 %3523, label %3540, label %3524                                                                                 ;L1630
 46443| 
 46444| 3524: ; preds = %3522
 46445|     ;; ae = ptr %3519
 46446|     ;; self = ptr %3519
 46447|  %3525 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget16check_projectile(ptr %3406, ptr %378, ptr %3519)
 46448|  to label %3526 unwind label %365                                                                                      ;L1631
 46449| 
 46450| 3526: ; preds = %3524
 46451|  br i1 %3525, label %3527, label %3540                                                                                 ;L1631
 46452| 
 46453| 3527: ; preds = %3526
 46454|  %3528 = gep %3519, i64 1632                                                                                           ;L1635
 46455|  %3529 = load i64, ptr %3528, , !!8                                                                                    ;L1635
 46456|  %3530 = gep %3519, i64 1640                                                                                           ;L1635
 46457|  %3531 = load i64, ptr %3530, , !!8                                                                                    ;L1635
 46458|  %3532 = gep %3519, i64 1136                                                                                           ;L1511<1635
 46459|  %3533 = load i32, ptr %3532, , !!8                                                                                    ;L1511<1635
 46460|     ;; mult = i32 %3533
 46461|  %3534 = icmp eq i32 %3533, 0                                                                                          ;L1512<1635
 46462|  br i1 %3534, label %3541, label %3544                                                                                 ;L1512<1635
 46463| 
 46464| 3535: ; preds = %3584, %3578
 46465|  %3536 = phi i64 [ 120, %3578 ], [ 144, %3584 ]
 46466|  %3537 = gep %3513, i64 %3536                                                                                          ;L0
 46467|  %3538 = load i64, ptr %3537, , !!8                                                                                    ;L0
 46468|  %3539 = add i64 %3538, 1                                                                                              ;L0
 46469|  store i64 %3539, ptr %3537,                                                                                           ;L0
 46470|  br label %3540                                                                                                        ;L1714<180<1629
 46471| 
 46472| 3540: ; preds = %3584, %3578, %3565, %3554, %3535, %3526, %3522
 46473|  br label %3512                                                                                                        ;L1714<180<1629
 46474| 
 46475| 3541: ; preds = %3527
 46476|  %3542 = gep %3519, i64 1664                                                                                           ;L1513<1635
 46477|  %3543 = load i64, ptr %3542, , !!8                                                                                    ;L1513<1635
 46478|  br label %3551                                                                                                        ;L1512<1635
 46479| 
 46480| 3544: ; preds = %3527
 46481|  %3545 = sext i32 %3533 to i64                                                                                         ;L1511<1635
 46482|     ;; mult = i64 %3545
 46483|  %3546 = gep %3519, i64 1664                                                                                           ;L1515<1635
 46484|  %3547 = load i64, ptr %3546, , !!8                                                                                    ;L1515<1635
 46485|  %3548 = add nsw i64 %3545, 100                                                                                        ;L1515<1635
 46486|  %3549 = mul i64 %3547, %3548                                                                                          ;L1515<1635
 46487|  %3550 = udiv i64 %3549, 100                                                                                           ;L1515<1635
 46488|  br label %3551                                                                                                        ;L1512<1635
 46489| 
 46490| 3551: ; preds = %3544, %3541
 46491|  %3552 = phi i64 [ %3543, %3541 ], [ %3550, %3544 ]                                                                    ;L0<1635
 46492|  %3553 = invoke zeroext i1 @gc::simulation10projectileNtB5_10Projectile11is_in_orbit(ptr %378, i64 %3529, i64 %3531, i64 %3552)
 46493|  to label %3554 unwind label %365                                                                                      ;L1635
 46494| 
 46495| 3554: ; preds = %3551
 46496|  br i1 %3553, label %3555, label %3540                                                                                 ;L1635
 46497| 
 46498| 3555: ; preds = %3554
 46499|     ;; self = ptr %378
 46500|  %3556 = load i64, ptr %3407, , !!8                                                                                    ;L134<1639
 46501|  %3557 = icmp ne i64 %3556, 9                                                                                          ;L134<1639
 46502|  call void @llvm.assume(i1 %3557)                                                                                      ;L134<1639
 46503|  %3558 = add nsw i64 %3556, -2                                                                                         ;L134<1639
 46504|  %3559 = icmp samesign ugt i64 %3556, 1                                                                                ;L134<1639
 46505|  %3560 = select i1 %3559, i64 %3558, i64 7                                                                             ;L134<1639
 46506|  switch i64 %3560, label %3563 [
 46507|  i64 4, label %3565
 46508|  i64 5, label %3565
 46509|  i64 7, label %3561
 46510|  ]                                                                                                                     ;L134<1639
 46511| 
 46512| 3561: ; preds = %3555
 46513|  %3562 = icmp eq i64 %3556, 1                                                                                          ;L134<1639
 46514|  br i1 %3562, label %3565, label %3563                                                                                 ;L1639
 46515| 
 46516| 3563: ; preds = %3561, %3555
 46517|  %3564 = invoke i64 @gc::simulation10projectileNtB5_10Projectile22expected_damage_target(ptr %378, ptr %64, ptr %385, ptr %3519)
 46518|  to label %3579 unwind label %365                                                                                      ;L1647
 46519| 
 46520| 3565: ; preds = %3561, %3555, %3555
 46521|  %3566 = phi ptr [ %3408, %3561 ], [ %3409, %3555 ], [ %3409, %3555 ]
 46522|  %3567 = gep %3519, i64 1472                                                                                           ;L1640
 46523|  %3568 = load i64, ptr %3567, , !!8                                                                                    ;L1640
 46524|  %3569 = load i64, ptr %3566, , !!8                                                                                    ;L0<1640
 46525|  %3570 = icmp eq i64 %3569, %3568                                                                                      ;L0<1640
 46526|  br i1 %3570, label %3571, label %3540                                                                                 ;L1640
 46527| 
 46528| 3571: ; preds = %3565
 46529|  %3572 = invoke i64 @gc::simulation10projectileNtB5_10Projectile22expected_damage_target(ptr %378, ptr %64, ptr %385, ptr %3519)
 46530|  to label %3573 unwind label %365                                                                                      ;L1641
 46531| 
 46532| 3573: ; preds = %3571
 46533|  %3574 = gep %3513, i64 112                                                                                            ;L1641
 46534|  %3575 = load i64, ptr %3574, , !!8                                                                                    ;L1641
 46535|  %3576 = add i64 %3575, %3572                                                                                          ;L1641
 46536|  store i64 %3576, ptr %3574,                                                                                           ;L1641
 46537|  %3577 = invoke zeroext i1 @gc::simulation10projectileNtB5_10Projectile6has_cc(ptr %378)
 46538|  to label %3578 unwind label %365                                                                                      ;L1642
 46539| 
 46540| 3578: ; preds = %3573
 46541|  br i1 %3577, label %3535, label %3540                                                                                 ;L1642
 46542| 
 46543| 3579: ; preds = %3563
 46544|  %3580 = gep %3513, i64 128                                                                                            ;L1647
 46545|  %3581 = load i64, ptr %3580, , !!8                                                                                    ;L1647
 46546|  %3582 = add i64 %3581, %3564                                                                                          ;L1647
 46547|  store i64 %3582, ptr %3580,                                                                                           ;L1647
 46548|  %3583 = invoke zeroext i1 @gc::simulation10projectileNtB5_10Projectile6has_cc(ptr %378)
 46549|  to label %3584 unwind label %365                                                                                      ;L1648
 46550| 
 46551| 3584: ; preds = %3579
 46552|  br i1 %3583, label %3535, label %3540                                                                                 ;L1648
 46553| 
 46554| 3585: ; preds = %3520
 46555|  br i1 %3521, label %3586, label %3591                                                                                 ;L1655
 46556| 
 46557| 3586: ; preds = %3585
 46558|  %3587 = load i64, ptr %373, , !!8                                                                                     ;L1659
 46559|     ;; x2 = i64 %3587
 46560|     ;; other = i64 %3587
 46561|  %3588 = load i64, ptr %374, , !!8                                                                                     ;L1659
 46562|     ;; y2 = i64 %3588
 46563|     ;; other = i64 %3588
 46564|  %3589 = load i32, ptr %375, , !!8                                                                                     ;L1511<1659
 46565|     ;; mult = i32 %3589
 46566|  %3590 = icmp eq i32 %3589, 0                                                                                          ;L1512<1659
 46567|  br i1 %3590, label %3592, label %3594                                                                                 ;L1512<1659
 46568| 
 46569| 3591: ; preds = %3656, %3655, %3647, %3625, %3624, %3614, %3603, %3585, %3400
 46570|  br label %377                                                                                                         ;L1593
 46571| 
 46572| 3592: ; preds = %3586
 46573|  %3593 = load i64, ptr %376, , !!8                                                                                     ;L1513<1659
 46574|  br label %3600                                                                                                        ;L1512<1659
 46575| 
 46576| 3594: ; preds = %3586
 46577|  %3595 = sext i32 %3589 to i64                                                                                         ;L1511<1659
 46578|     ;; mult = i64 %3595
 46579|  %3596 = load i64, ptr %376, , !!8                                                                                     ;L1515<1659
 46580|  %3597 = add nsw i64 %3595, 100                                                                                        ;L1515<1659
 46581|  %3598 = mul i64 %3596, %3597                                                                                          ;L1515<1659
 46582|  %3599 = udiv i64 %3598, 100                                                                                           ;L1515<1659
 46583|  br label %3600                                                                                                        ;L1512<1659
 46584| 
 46585| 3600: ; preds = %3594, %3592
 46586|  %3601 = phi i64 [ %3593, %3592 ], [ %3599, %3594 ]                                                                    ;L0<1659
 46587|  %3602 = invoke zeroext i1 @gc::simulation10projectileNtB5_10Projectile11is_in_orbit(ptr %378, i64 %3587, i64 %3588, i64 %3601)
 46588|  to label %3603 unwind label %365                                                                                      ;L1659
 46589| 
 46590| 3603: ; preds = %3600
 46591|  br i1 %3602, label %3604, label %3591                                                                                 ;L1659
 46592| 
 46593| 3604: ; preds = %3603
 46594|     ;; self = ptr %378
 46595|  %3605 = load i64, ptr %3407, , !!8                                                                                    ;L134<1663
 46596|  %3606 = icmp ne i64 %3605, 9                                                                                          ;L134<1663
 46597|  call void @llvm.assume(i1 %3606)                                                                                      ;L134<1663
 46598|  %3607 = add nsw i64 %3605, -2                                                                                         ;L134<1663
 46599|  %3608 = icmp samesign ugt i64 %3605, 1                                                                                ;L134<1663
 46600|  %3609 = select i1 %3608, i64 %3607, i64 7                                                                             ;L134<1663
 46601|  switch i64 %3609, label %3612 [
 46602|  i64 7, label %3610
 46603|  i64 4, label %3614
 46604|  i64 5, label %3614
 46605|  ]                                                                                                                     ;L134<1663
 46606| 
 46607| 3610: ; preds = %3604
 46608|  %3611 = icmp eq i64 %3605, 1                                                                                          ;L134<1663
 46609|  br i1 %3611, label %3614, label %3612                                                                                 ;L1663
 46610| 
 46611| 3612: ; preds = %3610, %3604
 46619|  %3613 = trunc nuw i64 %3440 to i1                                                                                     ;L1672
 46620|  br i1 %3613, label %3628, label %3645                                                                                 ;L1672
 46621| 
 46622| 3614: ; preds = %3610, %3604, %3604
 46623|  %3615 = phi ptr [ %3409, %3604 ], [ %3409, %3604 ], [ %3408, %3610 ]
 46624|  %3616 = load i64, ptr %3615, , !!8                                                                                    ;L0<1664
 46625|  %3617 = icmp eq i64 %3616, %157                                                                                       ;L0<1664
 46626|  br i1 %3617, label %3618, label %3591                                                                                 ;L1664
 46627| 
 46628| 3618: ; preds = %3614
 46629|  %3619 = invoke i64 @gc::simulation10projectileNtB5_10Projectile22expected_damage_target(ptr %378, ptr %64, ptr %385, ptr %59)
 46630|  to label %3620 unwind label %365                                                                                      ;L1665
 46631| 
 46632| 3620: ; preds = %3618
 46633|  %3621 = load i64, ptr %87, , !!8                                                                                      ;L1665
 46634|  %3622 = add i64 %3621, %3619                                                                                          ;L1665
 46635|  store i64 %3622, ptr %87,                                                                                             ;L1665
 46636|  %3623 = invoke zeroext i1 @gc::simulation10projectileNtB5_10Projectile6has_cc(ptr %378)
 46637|  to label %3624 unwind label %365                                                                                      ;L1666
 46638| 
 46639| 3624: ; preds = %3620
 46640|  br i1 %3623, label %3625, label %3591                                                                                 ;L1666
 46641| 
 46642| 3625: ; preds = %3624
 46643|  %3626 = load i64, ptr %88, , !!8                                                                                      ;L1667
 46644|  %3627 = add i64 %3626, 1                                                                                              ;L1667
 46645|  store i64 %3627, ptr %88,                                                                                             ;L1667
 46646|  br label %3591                                                                                                        ;L1666
 46647| 
 46648| 3628: ; preds = %3612
 46649|  %3629 = load i64, ptr %3411, , !!8                                                                                    ;L1671
 46650|     ;; y1 = i64 %3629
 46651|     ;; self = i64 %3629
 46652|  %3630 = icmp ult i64 %3629, %3588                                                                                     ;L3147<8<1671
 46653|  %3631 = sub nuw i64 %3588, %3629                                                                                      ;L3147<8<1671
 46654|  %3632 = sub nuw i64 %3629, %3588                                                                                      ;L3147<8<1671
 46655|  %3633 = select i1 %3630, i64 %3631, i64 %3632                                                                         ;L3147<8<1671
 46656|     ;; dy = i64 %3633
 46657|  %3634 = load i64, ptr %3410, , !!8                                                                                    ;L1671
 46658|     ;; x1 = i64 %3634
 46659|     ;; self = i64 %3634
 46660|  %3635 = icmp ult i64 %3634, %3587                                                                                     ;L3147<7<1671
 46661|  %3636 = sub nuw i64 %3587, %3634                                                                                      ;L3147<7<1671
 46662|  %3637 = sub nuw i64 %3634, %3587                                                                                      ;L3147<7<1671
 46663|  %3638 = select i1 %3635, i64 %3636, i64 %3637                                                                         ;L3147<7<1671
 46664|     ;; dx = i64 %3638
 46665|     ;; dist = !DIArgList(i64 %3638, i64 %3633, i64 %3633, i64 %3638)
 46666|  %3639 = mul i64 %3638, %3638                                                                                          ;L9<1671
 46667|     ;; dist = !DIArgList(i64 %3639, i64 %3633, i64 %3633)
 46668|  %3640 = mul i64 %3633, %3633                                                                                          ;L9<1671
 46669|     ;; dist = !DIArgList(i64 %3639, i64 %3640)
 46670|  %3641 = add i64 %3639, %3640                                                                                          ;L9<1671
 46671|     ;; dist = i64 %3641
 46672|     ;; nearest_other_distance = i64 %3439
 46673|  %3642 = icmp ugt i64 %3641, %3439                                                                                     ;L1673
 46674|     ;; self = ptr %378
 46675|  %3643 = icmp eq i64 %3609, 0
 46676|  %3644 = and i1 %3643, %3642                                                                                           ;L1673
 46677|  br i1 %3644, label %3647, label %3645                                                                                 ;L1673
 46678| 
 46679| 3645: ; preds = %3647, %3628, %3612
 46680|  %3646 = invoke i64 @gc::simulation10projectileNtB5_10Projectile22expected_damage_target(ptr %378, ptr %64, ptr %385, ptr %59)
 46681|  to label %3651 unwind label %365                                                                                      ;L1678
 46682| 
 46683| 3647: ; preds = %3628
 46684|     ;; penetrate = ptr %378
 46685|  %3648 = gep %378, i64 120                                                                                             ;L122<1673
 46686|  %3649 = load i8, ptr %3648, , !!8                                                                                     ;L122<1673
 46687|  %3650 = trunc nuw i8 %3649 to i1                                                                                      ;L122<1673
 46688|  br i1 %3650, label %3645, label %3591                                                                                 ;L1673
 46689| 
 46690| 3651: ; preds = %3645
 46691|  %3652 = load i64, ptr %89, , !!8                                                                                      ;L1678
 46692|  %3653 = add i64 %3652, %3646                                                                                          ;L1678
 46693|  store i64 %3653, ptr %89,                                                                                             ;L1678
 46694|  %3654 = invoke zeroext i1 @gc::simulation10projectileNtB5_10Projectile6has_cc(ptr %378)
 46695|  to label %3655 unwind label %365                                                                                      ;L1679
 46696| 
 46697| 3655: ; preds = %3651
 46698|  br i1 %3654, label %3656, label %3591                                                                                 ;L1679
 46699| 
 46700| 3656: ; preds = %3655
 46701|  %3657 = load i64, ptr %91, , !!8                                                                                      ;L1680
 46702|  %3658 = add i64 %3657, 1                                                                                              ;L1680
 46703|  store i64 %3658, ptr %91,                                                                                             ;L1680
 46704|  br label %3591                                                                                                        ;L1679
 46705| 
 46706| 3659: ; preds = %146
 46707|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai15score_parameter22ChampionScoreParameterEEB1t_(ptr %67) #27 [ "funclet"(token %148) ] ;L2418
 46708|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai15score_parameter22ChampionScoreParameterEEB1t_(ptr %71) #27 [ "funclet"(token %148) ] ;L2418
 46709|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai15score_parameter22ChampionScoreParameterEBF_(ptr %75) #27 [ "funclet"(token %148) ] ;L2418
 46710|  br label %3660                                                                                                        ;L2418
 46711| 
 46712| 3660: ; preds = %3659, %146
 46713|  cleanupret from %148 unwind to caller                                                                                 ;L1461
 46714| }
