 49922| define void @ai::utils26build_minion_wave_snapshot(ptr sret([2320 x i8]) %0, ptr %1, ptr %2, i64 %3, i64 %4) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
 49935|  %6 = alloca [40 x i8],
 49936|  %7 = alloca [40 x i8],
 49937|  %8 = alloca [64 x i8],
 49938|  %9 = alloca [64 x i8],
 49939|  %10 = alloca [96 x i8],
 49940|  %11 = alloca [3072 x i8],
 49941|  %12 = alloca [96 x i8],
 49942|  %13 = alloca [104 x i8],
 49943|  %14 = alloca [64 x i8],
 49944|  %15 = alloca [64 x i8],
 49945|  %16 = alloca [96 x i8],
 49946|  %17 = alloca [32 x i8],
 49947|  %18 = alloca [2320 x i8],
 49948|     ;; player = ptr %1
 49949|     ;; data = ptr %2
 49950|     ;; prediction_depth = i64 %3
 49951|     ;; source_quality = i64 %4
 49952|     ;; snapshot = ptr %18
 49953|     ;; targets = ptr %17
 49954|     ;; target_ids = ptr %16
 49955|     ;; iter = ptr %14
 49956|     ;; find_slot = ptr %13
 49957|     ;; dps_per_tick = ptr %12
 49958|     ;; one_shot_damages = ptr %11
 49959|     ;; one_shot_count = ptr %10
 49960|     ;; iter = ptr %8
 49961|     ;; iter = ptr %6
 49962|     ;; range_sq = i64 6400000000
 49964|     ;; n = i64 1
 49965|     ;; rhs = i64 1
 49966|     ;; n = i64 1
 49967|     ;; rhs = i64 1
 49968|     ;; n = i64 1
 49969|     ;; rhs = i64 1
 49970|     ;; n = i64 1
 49971|     ;; rhs = i64 1
 49972|  %19 = gep %1, i64 2352                                                                                                ;L617
 49973|  %20 = load i64, ptr %19, , !!8                                                                                        ;L617
 49974|  %21 = icmp ult i64 %20, 2                                                                                             ;L617
 49975|  br i1 %21, label %23, label %22                                                                                       ;L617
 49976| 
 49977| 22: ; preds = %5
 49978|  tail call void @core::panicking18panic_bounds_check(i64 %20, i64 2, ptr @anon.168add0ea037d45d276f5936ae758fe5.223) #30 ;L617
 49979|  unreachable                                                                                                           ;L617
 49980| 
 49981| 23: ; preds = %5
 49982|     ;; self = ptr %1
 49983|  %24 = gep %1, i64 2496                                                                                                ;L581<617
 49984|  %25 = load i32, ptr %24, , !!8                                                                                        ;L581<617
 49985|  %26 = zext nneg i32 %25 to i64                                                                                        ;L581<617
 49986|  %27 = load ptr, ptr %2, , !!8, !!8                                                                                    ;L617
 49987|  %28 = gep %27, i64 480                                                                                                ;L617
 49988|  %29 = getelementptr [5 x ptr], ptr %28, i64 %20                                                                       ;L617
 49989|  %30 = getelementptr ptr, ptr %29, i64 %26                                                                             ;L617
 49990|  %31 = load ptr, ptr %30, , !!8                                                                                        ;L617
 49991|     ;; self = ptr %31
 49992|  %32 = icmp eq ptr %31, null                                                                                           ;L1011<617
 49993|  br i1 %32, label %86, label %33                                                                                       ;L1011<617
 49994| 
 49995| 33: ; preds = %23
 49996|     ;; champ = ptr %31
 49998|  %34 = gep %18, i64 2112                                                                                               ;L75<618
 49999|  call void @llvm.memset.p0.i64(ptr %34, i8 0, i64 144, i1 false)                                                       ;L31<76<618
 50000|  call void @llvm.memset.p0.i64(ptr %18, i8 0, i64 144, i1 false)
 50001|  %35 = gep %18, i64 192                                                                                                ;L75<618
 50002|  call void @llvm.memset.p0.i64(ptr %35, i8 0, i64 144, i1 false)
 50003|  %36 = gep %18, i64 384                                                                                                ;L75<618
 50004|  call void @llvm.memset.p0.i64(ptr %36, i8 0, i64 144, i1 false)
 50005|  %37 = gep %18, i64 576                                                                                                ;L75<618
 50006|  call void @llvm.memset.p0.i64(ptr %37, i8 0, i64 144, i1 false)
 50007|  %38 = gep %18, i64 768                                                                                                ;L75<618
 50008|  call void @llvm.memset.p0.i64(ptr %38, i8 0, i64 144, i1 false)
 50009|  %39 = gep %18, i64 960                                                                                                ;L75<618
 50010|  call void @llvm.memset.p0.i64(ptr %39, i8 0, i64 144, i1 false)
 50011|  %40 = gep %18, i64 1152                                                                                               ;L75<618
 50012|  call void @llvm.memset.p0.i64(ptr %40, i8 0, i64 144, i1 false)
 50013|  %41 = gep %18, i64 1344                                                                                               ;L75<618
 50014|  call void @llvm.memset.p0.i64(ptr %41, i8 0, i64 144, i1 false)
 50015|  %42 = gep %18, i64 1536                                                                                               ;L75<618
 50016|  call void @llvm.memset.p0.i64(ptr %42, i8 0, i64 144, i1 false)
 50017|  %43 = gep %18, i64 1728                                                                                               ;L75<618
 50018|  call void @llvm.memset.p0.i64(ptr %43, i8 0, i64 144, i1 false)
 50019|  %44 = gep %18, i64 1920                                                                                               ;L75<618
 50020|  call void @llvm.memset.p0.i64(ptr %44, i8 0, i64 144, i1 false)
 50021|  %45 = gep %18, i64 144                                                                                                ;L75<618
 50022|  %46 = gep %18, i64 168                                                                                                ;L75<618
 50023|  %47 = gep %18, i64 184                                                                                                ;L75<618
 50024|  call void @llvm.memset.p0.i64(ptr %45, i8 0, i64 40, i1 false)                                                        ;L75<618
 50025|  store i64 -1, ptr %47,                                                                                                ;L75<618
 50026|  %48 = gep %18, i64 336                                                                                                ;L75<618
 50027|  %49 = gep %18, i64 376                                                                                                ;L75<618
 50028|  call void @llvm.memset.p0.i64(ptr %48, i8 0, i64 40, i1 false)                                                        ;L75<618
 50029|  store i64 -1, ptr %49,                                                                                                ;L75<618
 50030|  %50 = gep %18, i64 528                                                                                                ;L75<618
 50031|  %51 = gep %18, i64 568                                                                                                ;L75<618
 50032|  call void @llvm.memset.p0.i64(ptr %50, i8 0, i64 40, i1 false)                                                        ;L75<618
 50033|  store i64 -1, ptr %51,                                                                                                ;L75<618
 50034|  %52 = gep %18, i64 720                                                                                                ;L75<618
 50035|  %53 = gep %18, i64 760                                                                                                ;L75<618
 50036|  call void @llvm.memset.p0.i64(ptr %52, i8 0, i64 40, i1 false)                                                        ;L75<618
 50037|  store i64 -1, ptr %53,                                                                                                ;L75<618
 50038|  %54 = gep %18, i64 912                                                                                                ;L75<618
 50039|  %55 = gep %18, i64 952                                                                                                ;L75<618
 50040|  call void @llvm.memset.p0.i64(ptr %54, i8 0, i64 40, i1 false)                                                        ;L75<618
 50041|  store i64 -1, ptr %55,                                                                                                ;L75<618
 50042|  %56 = gep %18, i64 1104                                                                                               ;L75<618
 50043|  %57 = gep %18, i64 1144                                                                                               ;L75<618
 50044|  call void @llvm.memset.p0.i64(ptr %56, i8 0, i64 40, i1 false)                                                        ;L75<618
 50045|  store i64 -1, ptr %57,                                                                                                ;L75<618
 50046|  %58 = gep %18, i64 1296                                                                                               ;L75<618
 50047|  %59 = gep %18, i64 1336                                                                                               ;L75<618
 50048|  call void @llvm.memset.p0.i64(ptr %58, i8 0, i64 40, i1 false)                                                        ;L75<618
 50049|  store i64 -1, ptr %59,                                                                                                ;L75<618
 50050|  %60 = gep %18, i64 1488                                                                                               ;L75<618
 50051|  %61 = gep %18, i64 1528                                                                                               ;L75<618
 50052|  call void @llvm.memset.p0.i64(ptr %60, i8 0, i64 40, i1 false)                                                        ;L75<618
 50053|  store i64 -1, ptr %61,                                                                                                ;L75<618
 50054|  %62 = gep %18, i64 1680                                                                                               ;L75<618
 50055|  %63 = gep %18, i64 1720                                                                                               ;L75<618
 50056|  call void @llvm.memset.p0.i64(ptr %62, i8 0, i64 40, i1 false)                                                        ;L75<618
 50057|  store i64 -1, ptr %63,                                                                                                ;L75<618
 50058|  %64 = gep %18, i64 1872                                                                                               ;L75<618
 50059|  %65 = gep %18, i64 1912                                                                                               ;L75<618
 50060|  call void @llvm.memset.p0.i64(ptr %64, i8 0, i64 40, i1 false)                                                        ;L75<618
 50061|  store i64 -1, ptr %65,                                                                                                ;L75<618
 50062|  %66 = gep %18, i64 2064                                                                                               ;L75<618
 50063|  %67 = gep %18, i64 2104                                                                                               ;L75<618
 50064|  call void @llvm.memset.p0.i64(ptr %66, i8 0, i64 40, i1 false)                                                        ;L75<618
 50065|  store i64 -1, ptr %67,                                                                                                ;L75<618
 50066|  %68 = gep %18, i64 2256                                                                                               ;L75<618
 50067|  %69 = gep %18, i64 2296                                                                                               ;L75<618
 50068|  call void @llvm.memset.p0.i64(ptr %68, i8 0, i64 40, i1 false)                                                        ;L75<618
 50069|  store i64 -1, ptr %69,                                                                                                ;L75<618
 50070|  %70 = gep %18, i64 2304                                                                                               ;L75<618
 50071|  %71 = gep %18, i64 2312                                                                                               ;L75<618
 50072|  call void @llvm.memset.p0.i64(ptr %70, i8 0, i64 16, i1 false)                                                        ;L75<618
 50073|  %72 = load ptr, ptr %27, , !!8, !!8                                                                                   ;L619
 50074|  %73 = gep %27, i64 8                                                                                                  ;L619
 50075|  %74 = load ptr, ptr %73, , !!8, !!8                                                                                   ;L619
 50076|  %75 = gep %74, i64 40                                                                                                 ;L619
 50077|  %76 = load ptr, ptr %75, , !!8                                                                                        ;L619
 50078|  %77 = tail call i64 %76(ptr %72)                                                                                      ;L619
 50079|  store i64 %77, ptr %71,                                                                                               ;L619
 50081|  %78 = gep %2, i64 8                                                                                                   ;L627
 50082|  %79 = load ptr, ptr %78, , !!8, !!8                                                                                   ;L627
 50083|  %80 = load ptr, ptr %79, , !!8, !!8                                                                                   ;L627
 50084|     ;; bump = ptr %80
 50085|  store ptr inttoptr (i64 8 to ptr), ptr %17,                                                                           ;L547<627
 50086|  %81 = gep %17, i64 8                                                                                                  ;L547<627
 50087|  store ptr %80, ptr %81,                                                                                               ;L547<627
 50088|  %82 = gep %17, i64 16                                                                                                 ;L547<627
 50089|  %83 = gep %17, i64 24                                                                                                 ;L547<627
 50090|  call void @llvm.memset.p0.i64(ptr %82, i8 0, i64 16, i1 false)                                                        ;L547<627
 50092|  call void @llvm.memset.p0.i64(ptr %16, i8 -1, i64 96, i1 false)                                                       ;L628
 50093|  %84 = gep %74, i64 512                                                                                                ;L629
 50094|  %85 = load ptr, ptr %84, , !!8                                                                                        ;L629
 50095|  invoke void %85(ptr sret([64 x i8]) %15, ptr %72)
 50096|  to label %89 unwind label %87                                                                                         ;L629
 50097| 
 50098| 86: ; preds = %23
 50099|  tail call void @core::option13unwrap_failed(ptr @anon.168add0ea037d45d276f5936ae758fe5.224) #30                       ;L1013<617
 50100|  unreachable                                                                                                           ;L1013<617
 50101| 
 50102| 87: ; preds = %844, %832, %831, %826, %802, %714, %706, %702, %681, %676, %649, %631, %621, %618, %610, %603, %587, %586, %519, %506, %500, %178, %170, %156, %93, %33
 50103|  %88 = cleanuppad within none []
 50104|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEECshdEBA0ozCnw_7game_ai(ptr %17) #31 [ "funclet"(token %88) ] ;L765
 50105|  cleanupret from %88 unwind to caller                                                                                  ;L611
 50106| 
 50107| 89: ; preds = %33
 50109|  call void @llvm.memcpy.p0.p0.i64(ptr %14, ptr %15, i64 64, i1 false)                                                  ;L629
 50110|  %90 = gep %31, i64 8
 50111|  %91 = gep %31, i64 1632
 50112|  %92 = gep %31, i64 1640
 50113|  br label %93                                                                                                          ;L629
 50114| 
 50115| 93: ; preds = %166, %89
 50116|  %94 = invoke ptr @gc::simulationNtB5_10EntityIterNtNtNtNtCsjihNppCmMEE_4core4iter6traits8iterator8Iterator4next(ptr %14)
 50117|  to label %95 unwind label %87                                                                                         ;L629
 50118| 
 50119| 95: ; preds = %93
 50120|  %96 = icmp eq ptr %94, null                                                                                           ;L629
 50121|     ;; e = ptr %94
 50122|  %97 = load i64, ptr %70,
 50123|  %98 = icmp ugt i64 %97, 11
 50124|  %99 = select i1 %96, i1 true, i1 %98                                                                                  ;L629
 50125|  br i1 %99, label %100, label %102                                                                                     ;L629
 50126| 
 50127| 100: ; preds = %95
 50129|  %101 = icmp eq i64 %97, 0                                                                                             ;L649
 50130|  br i1 %101, label %167, label %170                                                                                    ;L649
 50131| 
 50132| 102: ; preds = %95
 50133|     ;; self = ptr %94
 50134|  %103 = gep %94, i64 104                                                                                               ;L1261<633
 50135|  %104 = load i64, ptr %103, , !!8                                                                                      ;L1261<633
 50136|  %105 = icmp eq i64 %104, 1                                                                                            ;L633
 50137|  br i1 %105, label %106, label %166                                                                                    ;L633
 50138| 
 50139| 106: ; preds = %102
 50140|     ;; self = ptr %94
 50141|     ;; other = ptr %31
 50142|  %107 = load i64, ptr %94, , !!8                                                                                       ;L1127<633
 50143|  %108 = gep %94, i64 8                                                                                                 ;L1127<633
 50144|     ;; __self_discr = i64 %107
 50145|  %109 = load i64, ptr %31, , !!8                                                                                       ;L1127<633
 50146|     ;; __arg1_discr = i64 %109
 50147|  %110 = icmp eq i64 %107, %109                                                                                         ;L1127<633
 50148|  br i1 %110, label %111, label %113                                                                                    ;L1127<633
 50149| 
 50150| 111: ; preds = %106
 50151|  %112 = icmp eq i64 %107, 0                                                                                            ;L1127<633
 50152|  br i1 %112, label %117, label %166                                                                                    ;L1127<633
 50153| 
 50154| 113: ; preds = %117, %106
 50155|  %114 = gep %94, i64 1721                                                                                              ;L633
 50156|  %115 = load i8, ptr %114, , !!8                                                                                       ;L633
 50157|  %116 = trunc nuw i8 %115 to i1                                                                                        ;L633
 50158|  br i1 %116, label %121, label %166                                                                                    ;L633
 50159| 
 50160| 117: ; preds = %111
 50161|     ;; __self_0 = ptr %94
 50162|     ;; self = ptr %94
 50163|     ;; __arg1_0 = ptr %31
 50164|     ;; other = ptr %31
 50167|  %118 = load i64, ptr %108, , !!8                                                                                      ;L1878<2123<1127<633
 50168|  %119 = load i64, ptr %90, , !!8                                                                                       ;L1878<2123<1127<633
 50169|  %120 = icmp eq i64 %118, %119                                                                                         ;L1878<2123<1127<633
 50170|  br i1 %120, label %166, label %113                                                                                    ;L633
 50171| 
 50172| 121: ; preds = %113
 50173|  %122 = load i64, ptr %91, , !!8                                                                                       ;L636
 50174|     ;; x1 = i64 %122
 50175|     ;; self = i64 %122
 50176|  %123 = load i64, ptr %92, , !!8                                                                                       ;L636
 50177|     ;; y1 = i64 %123
 50178|     ;; self = i64 %123
 50179|  %124 = gep %94, i64 1632                                                                                              ;L636
 50180|  %125 = load i64, ptr %124, , !!8                                                                                      ;L636
 50181|     ;; x2 = i64 %125
 50182|     ;; other = i64 %125
 50183|  %126 = gep %94, i64 1640                                                                                              ;L636
 50184|  %127 = load i64, ptr %126, , !!8                                                                                      ;L636
 50185|     ;; y2 = i64 %127
 50186|     ;; other = i64 %127
 50187|  %128 = icmp ult i64 %122, %125                                                                                        ;L3147<7<636
 50188|  %129 = sub nuw i64 %125, %122                                                                                         ;L3147<7<636
 50189|  %130 = sub nuw i64 %122, %125                                                                                         ;L3147<7<636
 50190|  %131 = select i1 %128, i64 %129, i64 %130                                                                             ;L3147<7<636
 50191|     ;; dx = i64 %131
 50192|  %132 = icmp ult i64 %123, %127                                                                                        ;L3147<8<636
 50193|  %133 = sub nuw i64 %127, %123                                                                                         ;L3147<8<636
 50194|  %134 = sub nuw i64 %123, %127                                                                                         ;L3147<8<636
 50195|  %135 = select i1 %132, i64 %133, i64 %134                                                                             ;L3147<8<636
 50196|     ;; dy = i64 %135
 50197|  %136 = mul i64 %131, %131                                                                                             ;L9<636
 50198|  %137 = mul i64 %135, %135                                                                                             ;L9<636
 50199|  %138 = add i64 %137, %136                                                                                             ;L9<636
 50200|  %139 = icmp ugt i64 %138, 6400000000                                                                                  ;L636
 50201|  br i1 %139, label %166, label %141                                                                                    ;L636
 50202| 
 50203| 140: ; preds = %844, %831, %714, %681, %618, %587, %586, %519
 50204|  unreachable
 50205| 
 50206| 141: ; preds = %121
 50207|  %142 = gepS %18, i64 %97                                                                                              ;L640
 50208|     ;; traj = ptr %142
 50209|  %143 = gep %94, i64 1472                                                                                              ;L641
 50210|  %144 = load i64, ptr %143, , !!8                                                                                      ;L641
 50211|  %145 = gep %142, i64 144                                                                                              ;L641
 50212|  store i64 %144, ptr %145,                                                                                             ;L641
 50213|  %146 = gep %94, i64 1648                                                                                              ;L642
 50214|  %147 = load i64, ptr %146, , !!8                                                                                      ;L642
 50215|  %148 = gep %142, i64 152                                                                                              ;L642
 50216|  store i64 %147, ptr %148,                                                                                             ;L642
 50217|  %149 = gep %94, i64 1576                                                                                              ;L643
 50218|  %150 = load i64, ptr %149, , !!8                                                                                      ;L643
 50219|  %151 = gep %142, i64 160                                                                                              ;L643
 50220|  store i64 %150, ptr %151,                                                                                             ;L643
 50221|  %152 = getelementptr i64, ptr %16, i64 %97                                                                            ;L644
 50222|  store i64 %144, ptr %152,                                                                                             ;L644
 50223|     ;; self = ptr %17
 50224|     ;; self = ptr %17
 50225|     ;; value = ptr %94
 50226|     ;; additional = i64 1
 50227|     ;; needed_extra_cap = i64 1
 50228|     ;; needed_extra_cap = i64 1
 50229|     ;; strategy = i8 1
 50230|  %153 = load i64, ptr %83, , !!61242, !!8                                                                              ;L1428<645
 50231|     ;; self = ptr %17
 50232|  %154 = load i64, ptr %82, , !!61242, !!8                                                                              ;L149<1428<645
 50233|  %155 = icmp eq i64 %153, %154                                                                                         ;L1428<645
 50234|  br i1 %155, label %156, label %159                                                                                    ;L1428<645
 50235| 
 50236| 156: ; preds = %141
 50237|     ;; self = ptr %17
 50238|     ;; self = ptr %17
 50239|     ;; self = ptr %17
 50240|     ;; used_cap = i64 %153
 50241|     ;; used_cap = i64 %153
 50242|  invoke void @gc::simulation6entity6EntityE25reserve_internal_or_panicB1a_(ptr %17, i64 %153, i64 1, i1 zeroext true)
 50243|  to label %157 unwind label %87                                                                                        ;L619<430<738<1429<645
 50244| 
 50245| 157: ; preds = %156
 50246|  %158 = load i64, ptr %83, , !!61242                                                                                   ;L1432<645
 50247|  br label %159                                                                                                         ;L1428<645
 50248| 
 50249| 159: ; preds = %157, %141
 50250|  %160 = phi i64 [ %153, %141 ], [ %158, %157 ]                                                                         ;L1432<645
 50251|     ;; self = ptr %17
 50252|  %161 = load ptr, ptr %17, , !!61242, !!8, !!8                                                                         ;L138<1432<645
 50253|     ;; self = ptr %161
 50254|     ;; count = i64 %160
 50255|  %162 = getelementptr ptr, ptr %161, i64 %160                                                                          ;L961<1432<645
 50256|     ;; end = ptr %162
 50257|     ;; dst = ptr %162
 50258|     ;; src = ptr %94
 50259|  store ptr %94, ptr %162, , !!61242                                                                                    ;L1933<1433<645
 50260|  %163 = load i64, ptr %83, , !!61242, !!8                                                                              ;L1434<645
 50261|  %164 = add i64 %163, 1                                                                                                ;L1434<645
 50262|  store i64 %164, ptr %83, , !!61242                                                                                    ;L1434<645
 50263|  %165 = add nuw nsw i64 %97, 1                                                                                         ;L646
 50264|  store i64 %165, ptr %70,                                                                                              ;L646
 50265|  br label %166                                                                                                         ;L629
 50266| 
 50267| 166: ; preds = %159, %121, %117, %113, %111, %102
 50268|  br label %93                                                                                                          ;L629
 50269| 
 50270| 167: ; preds = %100
 50271|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %18, i64 2320, i1 false)                                                 ;L650
 50274|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %17)
 50275|  to label %172 unwind label %168                                                                                       ;L825<765
 50276| 
 50277| 168: ; preds = %167
 50278|  %169 = cleanuppad within none []
 50280|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %17) [ "funclet"(token %169) ] ;L825<825<765
 50281|  cleanupret from %169 unwind to caller                                                                                 ;L825<765
 50282| 
 50283| 170: ; preds = %100
 50284|     ;; target_count = i64 %97
 50286|  call void @llvm.memcpy.p0.p0.i64(ptr %13, ptr %16, i64 96, i1 false)                                                  ;L654
 50287|  %171 = gep %13, i64 96                                                                                                ;L654
 50288|  store i64 %97, ptr %171,                                                                                              ;L654
 50290|  call void @llvm.memset.p0.i64(ptr %12, i8 0, i64 96, i1 false)                                                        ;L659
 50292|  call void @llvm.memset.p0.i64(ptr %11, i8 0, i64 3072, i1 false)
 50294|  call void @llvm.memset.p0.i64(ptr %10, i8 0, i64 96, i1 false)                                                        ;L662
 50295|  invoke void %85(ptr sret([64 x i8]) %9, ptr %72)
 50296|  to label %173 unwind label %87                                                                                        ;L665
 50297| 
 50298| 172: ; preds = %516, %167
 50299|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %17)   ;L825<825<765
 50302|  ret void                                                                                                              ;L765
 50303| 
 50304| 173: ; preds = %170
 50306|  call void @llvm.memcpy.p0.p0.i64(ptr %8, ptr %9, i64 64, i1 false)                                                    ;L665
 50307|  %174 = icmp eq i64 %4, 0
 50308|  %175 = icmp ult i64 %97, 13
 50309|  %176 = shl nuw nsw i64 %97, 3
 50310|  %177 = gep %13, i64 %176
 50311|  br label %178                                                                                                         ;L665
 50312| 
 50313| 178: ; preds = %816, %173
 50314|  %179 = invoke ptr @gc::simulationNtB5_10EntityIterNtNtNtNtCsjihNppCmMEE_4core4iter6traits8iterator8Iterator4next(ptr %8)
 50315|  to label %180 unwind label %87                                                                                        ;L665
 50316| 
 50317| 180: ; preds = %178
 50318|  %181 = icmp eq ptr %179, null                                                                                         ;L665
 50319|  br i1 %181, label %185, label %182                                                                                    ;L665
 50320| 
 50321| 182: ; preds = %180
 50322|     ;; src = ptr %179
 50323|  %183 = gep %179, i64 104                                                                                              ;L666
 50324|  %184 = load i64, ptr %183, , !!8                                                                                      ;L666
 50325|  switch i64 %184, label %816 [
 50326|  i64 1, label %720
 50327|  i64 2, label %725
 50328|  i64 7, label %726
 50329|  i64 8, label %731
 50330|  i64 9, label %736
 50331|  i64 10, label %741
 50332|  ]                                                                                                                     ;L666
 50333| 
 50334| 185: ; preds = %180
 50336|  %186 = icmp ugt i64 %4, 1                                                                                             ;L688
 50337|  br i1 %186, label %500, label %187                                                                                    ;L688
 50338| 
 50339| 187: ; preds = %515, %185
 50340|     ;; iter[0..+8] = i64 0
 50341|     ;; iter[8..+8] = i64 %97
 50342|  %188 = udiv i64 %3, 5
 50343|  %189 = call i64 @llvm.umin.i64(i64 %188, i64 18)
 50344|  %190 = icmp ult i64 %3, 5
 50345|     ;; iter[0..+8] = i64 0
 50346|     ;; self = ptr undef
 50347|     ;; self = ptr undef
 50348|     ;; self = ptr undef
 50349|     ;; other = ptr undef
 50350|  br i1 %190, label %194, label %191
 50351| 
 50352| 191: ; preds = %187
 50353|  %192 = add i64 %3, -5
 50354|  %193 = icmp ult i64 %192, 5
 50355|  br label %197                                                                                                         ;L725
 50356| 
 50357| 194: ; preds = %187
 50358|     ;; old = i64 0
 50359|     ;; start = i64 0
 50360|     ;; self = i64 0
 50361|     ;; slot = i64 0
 50362|     ;; iter[0..+8] = i64 1
 50363|     ;; traj = ptr %18
 50364|  %195 = load i64, ptr %12, , !!8                                                                                       ;L726
 50365|  store i64 %195, ptr %46,                                                                                              ;L726
 50366|     ;; num_checkpoints = i64 %189
 50367|     ;; iter[8..+8] = i64 %189
 50368|     ;; iter[0..+8] = i64 0
 50369|     ;; death_tick = i64 -1
 50371|     ;; self = ptr undef
 50372|     ;; self = ptr undef
 50373|     ;; self = ptr undef
 50374|     ;; other = ptr undef
 50375|  store i64 -1, ptr %47,                                                                                                ;L761
 50376|     ;; self = ptr undef
 50377|     ;; self = ptr undef
 50378|     ;; self = ptr undef
 50379|     ;; other = ptr undef
 50380|  %196 = icmp eq i64 %97, 1                                                                                             ;L1916<900<985<724
 50381|  br i1 %196, label %516, label %520                                                                                    ;L900<985<724
 50382| 
 50383| 197: ; preds = %306, %191
 50384|  %198 = phi i64 [ %199, %306 ], [ 0, %191 ]
 50385|     ;; old = i64 %198
 50386|     ;; start = i64 %198
 50387|     ;; self = i64 %198
 50388|  %199 = add nuw nsw i64 %198, 1                                                                                        ;L971<215<903<985<724
 50389|     ;; iter[0..+8] = i64 %199
 50390|     ;; slot = i64 %198
 50391|  %200 = icmp eq i64 %198, 12                                                                                           ;L725
 50392|  br i1 %200, label %519, label %201                                                                                    ;L725
 50393| 
 50394| 201: ; preds = %197
 50395|  %202 = gepS %18, i64 %198                                                                                             ;L725
 50396|     ;; traj = ptr %202
 50397|  %203 = getelementptr i64, ptr %12, i64 %198                                                                           ;L726
 50398|  %204 = load i64, ptr %203, , !!8                                                                                      ;L726
 50399|  %205 = gep %202, i64 168                                                                                              ;L726
 50400|  store i64 %204, ptr %205,                                                                                             ;L726
 50401|     ;; num_checkpoints = i64 %189
 50402|  %206 = gep %202, i64 152                                                                                              ;L729
 50403|  %207 = load i64, ptr %206, , !!8                                                                                      ;L729
 50404|     ;; iter[8..+8] = i64 %189
 50405|     ;; iter[0..+8] = i64 0
 50406|     ;; death_tick = i64 -1
 50407|     ;; hp = i64 %207
 50408|     ;; self = ptr undef
 50409|     ;; self = ptr undef
 50410|     ;; self = ptr undef
 50411|     ;; other = ptr undef
 50412|  %208 = mul i64 %204, -5
 50413|  %209 = getelementptr i64, ptr %10, i64 %198
 50414|  %210 = load i64, ptr %209, , !!8
 50415|  %211 = icmp eq i64 %210, 0
 50416|  %212 = gep %202, i64 176
 50417|  br i1 %211, label %263, label %213
 50418| 
 50419| 213: ; preds = %201
 50420|  %214 = getelementptr [16 x { i64, i64 }], ptr %11, i64 %198
 50421|  %215 = load i64, ptr %214,                                                                                            ;L740
 50422|  %216 = gep %214, i64 8
 50423|  %217 = icmp eq i64 %210, 1
 50424|  %218 = gep %214, i64 16
 50425|  %219 = gep %214, i64 24
 50426|  %220 = icmp eq i64 %210, 2
 50427|  %221 = gep %214, i64 32
 50428|  %222 = gep %214, i64 40
 50429|  %223 = icmp eq i64 %210, 3
 50430|  %224 = gep %214, i64 48
 50431|  %225 = gep %214, i64 56
 50432|  %226 = icmp eq i64 %210, 4
 50433|  %227 = gep %214, i64 64
 50434|  %228 = gep %214, i64 72
 50435|  %229 = icmp eq i64 %210, 5
 50436|  %230 = gep %214, i64 80
 50437|  %231 = gep %214, i64 88
 50438|  %232 = icmp eq i64 %210, 6
 50439|  %233 = gep %214, i64 96
 50440|  %234 = gep %214, i64 104
 50441|  %235 = icmp eq i64 %210, 7
 50442|  %236 = gep %214, i64 112
 50443|  %237 = gep %214, i64 120
 50444|  %238 = icmp eq i64 %210, 8
 50445|  %239 = gep %214, i64 128
 50446|  %240 = gep %214, i64 136
 50447|  %241 = icmp eq i64 %210, 9
 50448|  %242 = gep %214, i64 144
 50449|  %243 = gep %214, i64 152
 50450|  %244 = icmp eq i64 %210, 10
 50451|  %245 = gep %214, i64 160
 50452|  %246 = gep %214, i64 168
 50453|  %247 = icmp eq i64 %210, 11
 50454|  %248 = gep %214, i64 176
 50455|  %249 = gep %214, i64 184
 50456|  %250 = icmp eq i64 %210, 12
 50457|  %251 = gep %214, i64 192
 50458|  %252 = gep %214, i64 200
 50459|  %253 = icmp eq i64 %210, 13
 50460|  %254 = gep %214, i64 208
 50461|  %255 = gep %214, i64 216
 50462|  %256 = icmp eq i64 %210, 14
 50463|  %257 = gep %214, i64 224
 50464|  %258 = gep %214, i64 232
 50465|  %259 = icmp eq i64 %210, 15
 50466|  %260 = gep %214, i64 240
 50467|  %261 = gep %214, i64 248
 50468|  %262 = icmp eq i64 %210, 16
 50469|  br label %310                                                                                                         ;L900<985<739
 50470| 
 50471| 263: ; preds = %201
 50472|     ;; death_tick = i64 -1
 50473|     ;; hp = i64 %207
 50474|     ;; old = i64 0
 50475|     ;; start = i64 0
 50476|     ;; self = i64 0
 50477|     ;; iter[0..+8] = i64 1
 50478|     ;; i = i64 0
 50479|     ;; tick_offset = i64 1
 50480|  %264 = add i64 %207, %208                                                                                             ;L735
 50481|     ;; prev_tick = i64 0
 50482|     ;; iter[0..+8] = i64 0
 50483|     ;; iter[8..+8] = i64 %210
 50484|     ;; hp = i64 %264
 50485|     ;; self = ptr undef
 50486|     ;; self = ptr undef
 50487|     ;; self = ptr undef
 50488|     ;; other = ptr undef
 50489|  store i64 %264, ptr %202,                                                                                             ;L746
 50490|  store i64 1, ptr %212,                                                                                                ;L747
 50491|  %265 = icmp slt i64 %264, 1                                                                                           ;L749
 50492|  br i1 %265, label %266, label %276                                                                                    ;L749
 50493| 
 50494| 266: ; preds = %263
 50495|     ;; tick_offset = i64 5
 50496|     ;; prev_hp = i64 %207
 50497|  %267 = icmp sgt i64 %207, 0                                                                                           ;L752
 50498|  br i1 %267, label %268, label %276                                                                                    ;L752
 50499| 
 50500| 268: ; preds = %266
 50501|  %269 = mul i64 %207, 5                                                                                                ;L753
 50502|  %270 = icmp eq i64 %208, 1                                                                                            ;L753
 50503|  %271 = icmp eq i64 %269, -9223372036854775808                                                                         ;L753
 50504|  %272 = and i1 %270, %271                                                                                              ;L753
 50505|  br i1 %272, label %586, label %273                                                                                    ;L753
 50506| 
 50507| 273: ; preds = %268
 50508|  %274 = sub i64 0, %208                                                                                                ;L753
 50509|  %275 = sdiv i64 %269, %274                                                                                            ;L753
 50510|     ;; frac = i64 %275
 50511|     ;; death_tick = i64 %275
 50512|  br label %276                                                                                                         ;L752
 50513| 
 50514| 276: ; preds = %273, %266, %263
 50515|  %277 = phi i64 [ %275, %273 ], [ -1, %263 ], [ 5, %266 ]                                                              ;L0
 50516|     ;; iter[0..+8] = i64 1
 50517|     ;; death_tick = i64 %277
 50518|     ;; hp = i64 %264
 50519|     ;; self = ptr undef
 50520|     ;; self = ptr undef
 50521|     ;; self = ptr undef
 50522|     ;; other = ptr undef
 50523|  br i1 %193, label %306, label %278                                                                                    ;L900<985<732
 50524| 
 50525| 278: ; preds = %303, %276
 50526|  %279 = phi i64 [ %282, %303 ], [ 1, %276 ]
 50527|  %280 = phi i64 [ %304, %303 ], [ %277, %276 ]
 50528|  %281 = phi i64 [ %283, %303 ], [ %264, %276 ]
 50529|     ;; death_tick = i64 %280
 50530|     ;; hp = i64 %281
 50531|     ;; old = i64 %279
 50532|     ;; start = i64 %279
 50533|     ;; self = i64 %279
 50534|  %282 = add nuw nsw i64 %279, 1                                                                                        ;L971<215<903<985<732
 50535|     ;; iter[0..+8] = i64 %282
 50536|     ;; i = i64 %279
 50537|     ;; tick_offset = i64 %282
 50538|  %283 = add i64 %281, %208                                                                                             ;L735
 50539|     ;; hp = i64 %283
 50540|  %284 = mul nuw nsw i64 %279, 5                                                                                        ;L738
 50541|     ;; prev_tick = i64 %284
 50542|     ;; iter[0..+8] = i64 0
 50543|     ;; iter[8..+8] = i64 %210
 50544|     ;; self = ptr undef
 50545|     ;; self = ptr undef
 50546|     ;; self = ptr undef
 50547|     ;; other = ptr undef
 50548|  %285 = getelementptr i64, ptr %202, i64 %279                                                                          ;L746
 50549|  store i64 %283, ptr %285,                                                                                             ;L746
 50550|  store i64 %282, ptr %212,                                                                                             ;L747
 50551|  %286 = icmp slt i64 %283, 1                                                                                           ;L749
 50552|  %287 = icmp eq i64 %280, -1                                                                                           ;L749
 50553|  %288 = select i1 %286, i1 %287, i1 false                                                                              ;L749
 50554|  br i1 %288, label %289, label %303                                                                                    ;L749
 50555| 
 50556| 289: ; preds = %278
 50557|  %290 = mul nuw nsw i64 %282, 5                                                                                        ;L733
 50558|     ;; tick_offset = i64 %290
 50559|  %291 = gep %285, i64 -8                                                                                               ;L751
 50560|  %292 = load i64, ptr %291, , !!8                                                                                      ;L751
 50561|     ;; prev_hp = i64 %292
 50562|  %293 = icmp sgt i64 %292, 0                                                                                           ;L752
 50563|  br i1 %293, label %294, label %303                                                                                    ;L752
 50564| 
 50565| 294: ; preds = %289
 50566|  %295 = mul i64 %292, 5                                                                                                ;L753
 50567|  %296 = sub i64 %292, %283                                                                                             ;L753
 50568|  %297 = icmp eq i64 %296, -1                                                                                           ;L753
 50569|  %298 = icmp eq i64 %295, -9223372036854775808                                                                         ;L753
 50570|  %299 = and i1 %297, %298                                                                                              ;L753
 50571|  br i1 %299, label %586, label %300                                                                                    ;L753
 50572| 
 50573| 300: ; preds = %294
 50574|  %301 = sdiv i64 %295, %296                                                                                            ;L753
 50575|     ;; frac = i64 %301
 50576|  %302 = add i64 %301, %284                                                                                             ;L754
 50577|     ;; death_tick = i64 %302
 50578|  br label %303                                                                                                         ;L752
 50579| 
 50580| 303: ; preds = %300, %289, %278
 50581|  %304 = phi i64 [ %302, %300 ], [ %280, %278 ], [ %290, %289 ]                                                         ;L0
 50582|     ;; iter[0..+8] = i64 %282
 50583|     ;; death_tick = i64 %304
 50584|     ;; hp = i64 %283
 50585|     ;; self = ptr undef
 50586|     ;; self = ptr undef
 50587|     ;; self = ptr undef
 50588|     ;; other = ptr undef
 50589|  %305 = icmp eq i64 %282, %189                                                                                         ;L1916<900<985<732
 50590|  br i1 %305, label %306, label %278, !llvm.loop !61338                                                                 ;L900<985<732
 50591| 
 50592| 306: ; preds = %336, %303, %276
 50593|  %307 = phi i64 [ %304, %303 ], [ %277, %276 ], [ %337, %336 ]                                                         ;L900<985<732
 50594|  %308 = gep %202, i64 184                                                                                              ;L761
 50595|  store i64 %307, ptr %308,                                                                                             ;L761
 50596|     ;; iter[0..+8] = i64 %199
 50597|     ;; self = ptr undef
 50598|     ;; self = ptr undef
 50599|     ;; self = ptr undef
 50600|     ;; other = ptr undef
 50601|  %309 = icmp eq i64 %199, %97                                                                                          ;L1916<900<985<724
 50602|  br i1 %309, label %516, label %197                                                                                    ;L900<985<724
 50603| 
 50604| 310: ; preds = %336, %213
 50605|  %311 = phi i64 [ %314, %336 ], [ 0, %213 ]
 50606|  %312 = phi i64 [ %337, %336 ], [ -1, %213 ]
 50607|  %313 = phi i64 [ %495, %336 ], [ %207, %213 ]
 50608|     ;; death_tick = i64 %312
 50609|     ;; hp = i64 %313
 50610|     ;; old = i64 %311
 50611|     ;; start = i64 %311
 50612|     ;; self = i64 %311
 50613|  %314 = add nuw nsw i64 %311, 1                                                                                        ;L971<215<903<985<732
 50614|     ;; iter[0..+8] = i64 %314
 50615|     ;; i = i64 %311
 50616|  %315 = mul nuw nsw i64 %314, 5                                                                                        ;L733
 50617|     ;; tick_offset = i64 %315
 50618|  %316 = add i64 %313, %208                                                                                             ;L735
 50619|     ;; hp = i64 %316
 50620|  %317 = mul nuw nsw i64 %311, 5                                                                                        ;L738
 50621|     ;; prev_tick = i64 %317
 50622|     ;; iter[8..+8] = i64 %210
 50623|     ;; self = ptr undef
 50624|     ;; self = ptr undef
 50625|     ;; self = ptr undef
 50626|     ;; other = ptr undef
 50627|     ;; start = i64 0
 50628|     ;; self = i64 0
 50629|     ;; iter[0..+8] = i64 1
 50630|     ;; j = i64 0
 50631|     ;; arrival = i64 %215
 50633|  %318 = icmp ugt i64 %215, %317                                                                                        ;L741
 50634|  %319 = icmp ule i64 %215, %315                                                                                        ;L741
 50635|  %320 = and i1 %318, %319                                                                                              ;L741
 50636|  br i1 %320, label %339, label %342                                                                                    ;L741
 50637| 
 50638| 321: ; preds = %494
 50639|  %322 = icmp eq i64 %311, 0                                                                                            ;L751
 50640|  %323 = gep %496, i64 -8                                                                                               ;L751
 50641|  %324 = select i1 %322, ptr %206, ptr %323                                                                             ;L751
 50642|  %325 = load i64, ptr %324, , !!8                                                                                      ;L751
 50643|     ;; prev_hp = i64 %325
 50644|  %326 = icmp sgt i64 %325, 0                                                                                           ;L752
 50645|  br i1 %326, label %327, label %336                                                                                    ;L752
 50646| 
 50647| 327: ; preds = %321
 50648|  %328 = mul i64 %325, 5                                                                                                ;L753
 50649|  %329 = sub i64 %325, %495                                                                                             ;L753
 50650|  %330 = icmp eq i64 %329, -1                                                                                           ;L753
 50651|  %331 = icmp eq i64 %328, -9223372036854775808                                                                         ;L753
 50652|  %332 = and i1 %330, %331                                                                                              ;L753
 50653|  br i1 %332, label %586, label %333                                                                                    ;L753
 50654| 
 50655| 333: ; preds = %327
 50656|  %334 = sdiv i64 %328, %329                                                                                            ;L753
 50657|     ;; frac = i64 %334
 50658|  %335 = add i64 %334, %317                                                                                             ;L754
 50659|     ;; death_tick = i64 %335
 50660|  br label %336                                                                                                         ;L752
 50661| 
 50662| 336: ; preds = %494, %333, %321
 50663|  %337 = phi i64 [ %335, %333 ], [ %312, %494 ], [ %315, %321 ]                                                         ;L0
 50664|     ;; iter[0..+8] = i64 %314
 50665|     ;; death_tick = i64 %337
 50667|     ;; self = ptr undef
 50668|     ;; self = ptr undef
 50669|     ;; self = ptr undef
 50670|     ;; other = ptr undef
 50671|  %338 = icmp eq i64 %314, %189                                                                                         ;L1916<900<985<732
 50672|  br i1 %338, label %306, label %310                                                                                    ;L900<985<732
 50673| 
 50674| 339: ; preds = %310
 50675|  %340 = load i64, ptr %216, , !!8                                                                                      ;L740
 50676|     ;; dmg = i64 %340
 50677|  %341 = sub i64 %316, %340                                                                                             ;L742
 50678|     ;; hp = i64 %341
 50679|  br label %342                                                                                                         ;L741
 50680| 
 50681| 342: ; preds = %339, %310
 50682|  %343 = phi i64 [ %341, %339 ], [ %316, %310 ]                                                                         ;L0
 50683|     ;; iter[0..+8] = i64 1
 50684|     ;; hp = i64 %343
 50685|     ;; self = ptr undef
 50686|     ;; self = ptr undef
 50687|     ;; self = ptr undef
 50688|     ;; other = ptr undef
 50689|  br i1 %217, label %494, label %344                                                                                    ;L900<985<739
 50690| 
 50691| 344: ; preds = %342
 50692|     ;; hp = i64 %343
 50693|     ;; start = i64 1
 50694|     ;; self = i64 1
 50695|     ;; iter[0..+8] = i64 2
 50696|     ;; j = i64 1
 50697|  %345 = load i64, ptr %218, , !!8                                                                                      ;L740
 50698|     ;; arrival = i64 %345
 50700|  %346 = icmp ugt i64 %345, %317                                                                                        ;L741
 50701|  %347 = icmp ule i64 %345, %315                                                                                        ;L741
 50702|  %348 = and i1 %346, %347                                                                                              ;L741
 50703|  br i1 %348, label %349, label %352                                                                                    ;L741
 50704| 
 50705| 349: ; preds = %344
 50706|  %350 = load i64, ptr %219, , !!8                                                                                      ;L740
 50707|     ;; dmg = i64 %350
 50708|  %351 = sub i64 %343, %350                                                                                             ;L742
 50709|     ;; hp = i64 %351
 50710|  br label %352                                                                                                         ;L741
 50711| 
 50712| 352: ; preds = %349, %344
 50713|  %353 = phi i64 [ %351, %349 ], [ %343, %344 ]                                                                         ;L0
 50714|     ;; iter[0..+8] = i64 2
 50715|     ;; hp = i64 %353
 50716|     ;; self = ptr undef
 50717|     ;; self = ptr undef
 50718|     ;; self = ptr undef
 50719|     ;; other = ptr undef
 50720|  br i1 %220, label %494, label %354                                                                                    ;L900<985<739
 50721| 
 50722| 354: ; preds = %352
 50723|     ;; hp = i64 %353
 50724|     ;; start = i64 2
 50725|     ;; self = i64 2
 50726|     ;; iter[0..+8] = i64 3
 50727|     ;; j = i64 2
 50728|  %355 = load i64, ptr %221, , !!8                                                                                      ;L740
 50729|     ;; arrival = i64 %355
 50731|  %356 = icmp ugt i64 %355, %317                                                                                        ;L741
 50732|  %357 = icmp ule i64 %355, %315                                                                                        ;L741
 50733|  %358 = and i1 %356, %357                                                                                              ;L741
 50734|  br i1 %358, label %359, label %362                                                                                    ;L741
 50735| 
 50736| 359: ; preds = %354
 50737|  %360 = load i64, ptr %222, , !!8                                                                                      ;L740
 50738|     ;; dmg = i64 %360
 50739|  %361 = sub i64 %353, %360                                                                                             ;L742
 50740|     ;; hp = i64 %361
 50741|  br label %362                                                                                                         ;L741
 50742| 
 50743| 362: ; preds = %359, %354
 50744|  %363 = phi i64 [ %361, %359 ], [ %353, %354 ]                                                                         ;L0
 50745|     ;; iter[0..+8] = i64 3
 50746|     ;; hp = i64 %363
 50747|     ;; self = ptr undef
 50748|     ;; self = ptr undef
 50749|     ;; self = ptr undef
 50750|     ;; other = ptr undef
 50751|  br i1 %223, label %494, label %364                                                                                    ;L900<985<739
 50752| 
 50753| 364: ; preds = %362
 50754|     ;; hp = i64 %363
 50755|     ;; start = i64 3
 50756|     ;; self = i64 3
 50757|     ;; iter[0..+8] = i64 4
 50758|     ;; j = i64 3
 50759|  %365 = load i64, ptr %224, , !!8                                                                                      ;L740
 50760|     ;; arrival = i64 %365
 50762|  %366 = icmp ugt i64 %365, %317                                                                                        ;L741
 50763|  %367 = icmp ule i64 %365, %315                                                                                        ;L741
 50764|  %368 = and i1 %366, %367                                                                                              ;L741
 50765|  br i1 %368, label %369, label %372                                                                                    ;L741
 50766| 
 50767| 369: ; preds = %364
 50768|  %370 = load i64, ptr %225, , !!8                                                                                      ;L740
 50769|     ;; dmg = i64 %370
 50770|  %371 = sub i64 %363, %370                                                                                             ;L742
 50771|     ;; hp = i64 %371
 50772|  br label %372                                                                                                         ;L741
 50773| 
 50774| 372: ; preds = %369, %364
 50775|  %373 = phi i64 [ %371, %369 ], [ %363, %364 ]                                                                         ;L0
 50776|     ;; iter[0..+8] = i64 4
 50777|     ;; hp = i64 %373
 50778|     ;; self = ptr undef
 50779|     ;; self = ptr undef
 50780|     ;; self = ptr undef
 50781|     ;; other = ptr undef
 50782|  br i1 %226, label %494, label %374                                                                                    ;L900<985<739
 50783| 
 50784| 374: ; preds = %372
 50785|     ;; hp = i64 %373
 50786|     ;; start = i64 4
 50787|     ;; self = i64 4
 50788|     ;; iter[0..+8] = i64 5
 50789|     ;; j = i64 4
 50790|  %375 = load i64, ptr %227, , !!8                                                                                      ;L740
 50791|     ;; arrival = i64 %375
 50793|  %376 = icmp ugt i64 %375, %317                                                                                        ;L741
 50794|  %377 = icmp ule i64 %375, %315                                                                                        ;L741
 50795|  %378 = and i1 %376, %377                                                                                              ;L741
 50796|  br i1 %378, label %379, label %382                                                                                    ;L741
 50797| 
 50798| 379: ; preds = %374
 50799|  %380 = load i64, ptr %228, , !!8                                                                                      ;L740
 50800|     ;; dmg = i64 %380
 50801|  %381 = sub i64 %373, %380                                                                                             ;L742
 50802|     ;; hp = i64 %381
 50803|  br label %382                                                                                                         ;L741
 50804| 
 50805| 382: ; preds = %379, %374
 50806|  %383 = phi i64 [ %381, %379 ], [ %373, %374 ]                                                                         ;L0
 50807|     ;; iter[0..+8] = i64 5
 50808|     ;; hp = i64 %383
 50809|     ;; self = ptr undef
 50810|     ;; self = ptr undef
 50811|     ;; self = ptr undef
 50812|     ;; other = ptr undef
 50813|  br i1 %229, label %494, label %384                                                                                    ;L900<985<739
 50814| 
 50815| 384: ; preds = %382
 50816|     ;; hp = i64 %383
 50817|     ;; start = i64 5
 50818|     ;; self = i64 5
 50819|     ;; iter[0..+8] = i64 6
 50820|     ;; j = i64 5
 50821|  %385 = load i64, ptr %230, , !!8                                                                                      ;L740
 50822|     ;; arrival = i64 %385
 50824|  %386 = icmp ugt i64 %385, %317                                                                                        ;L741
 50825|  %387 = icmp ule i64 %385, %315                                                                                        ;L741
 50826|  %388 = and i1 %386, %387                                                                                              ;L741
 50827|  br i1 %388, label %389, label %392                                                                                    ;L741
 50828| 
 50829| 389: ; preds = %384
 50830|  %390 = load i64, ptr %231, , !!8                                                                                      ;L740
 50831|     ;; dmg = i64 %390
 50832|  %391 = sub i64 %383, %390                                                                                             ;L742
 50833|     ;; hp = i64 %391
 50834|  br label %392                                                                                                         ;L741
 50835| 
 50836| 392: ; preds = %389, %384
 50837|  %393 = phi i64 [ %391, %389 ], [ %383, %384 ]                                                                         ;L0
 50838|     ;; iter[0..+8] = i64 6
 50839|     ;; hp = i64 %393
 50840|     ;; self = ptr undef
 50841|     ;; self = ptr undef
 50842|     ;; self = ptr undef
 50843|     ;; other = ptr undef
 50844|  br i1 %232, label %494, label %394                                                                                    ;L900<985<739
 50845| 
 50846| 394: ; preds = %392
 50847|     ;; hp = i64 %393
 50848|     ;; start = i64 6
 50849|     ;; self = i64 6
 50850|     ;; iter[0..+8] = i64 7
 50851|     ;; j = i64 6
 50852|  %395 = load i64, ptr %233, , !!8                                                                                      ;L740
 50853|     ;; arrival = i64 %395
 50855|  %396 = icmp ugt i64 %395, %317                                                                                        ;L741
 50856|  %397 = icmp ule i64 %395, %315                                                                                        ;L741
 50857|  %398 = and i1 %396, %397                                                                                              ;L741
 50858|  br i1 %398, label %399, label %402                                                                                    ;L741
 50859| 
 50860| 399: ; preds = %394
 50861|  %400 = load i64, ptr %234, , !!8                                                                                      ;L740
 50862|     ;; dmg = i64 %400
 50863|  %401 = sub i64 %393, %400                                                                                             ;L742
 50864|     ;; hp = i64 %401
 50865|  br label %402                                                                                                         ;L741
 50866| 
 50867| 402: ; preds = %399, %394
 50868|  %403 = phi i64 [ %401, %399 ], [ %393, %394 ]                                                                         ;L0
 50869|     ;; iter[0..+8] = i64 7
 50870|     ;; hp = i64 %403
 50871|     ;; self = ptr undef
 50872|     ;; self = ptr undef
 50873|     ;; self = ptr undef
 50874|     ;; other = ptr undef
 50875|  br i1 %235, label %494, label %404                                                                                    ;L900<985<739
 50876| 
 50877| 404: ; preds = %402
 50878|     ;; hp = i64 %403
 50879|     ;; start = i64 7
 50880|     ;; self = i64 7
 50881|     ;; iter[0..+8] = i64 8
 50882|     ;; j = i64 7
 50883|  %405 = load i64, ptr %236, , !!8                                                                                      ;L740
 50884|     ;; arrival = i64 %405
 50886|  %406 = icmp ugt i64 %405, %317                                                                                        ;L741
 50887|  %407 = icmp ule i64 %405, %315                                                                                        ;L741
 50888|  %408 = and i1 %406, %407                                                                                              ;L741
 50889|  br i1 %408, label %409, label %412                                                                                    ;L741
 50890| 
 50891| 409: ; preds = %404
 50892|  %410 = load i64, ptr %237, , !!8                                                                                      ;L740
 50893|     ;; dmg = i64 %410
 50894|  %411 = sub i64 %403, %410                                                                                             ;L742
 50895|     ;; hp = i64 %411
 50896|  br label %412                                                                                                         ;L741
 50897| 
 50898| 412: ; preds = %409, %404
 50899|  %413 = phi i64 [ %411, %409 ], [ %403, %404 ]                                                                         ;L0
 50900|     ;; iter[0..+8] = i64 8
 50901|     ;; hp = i64 %413
 50902|     ;; self = ptr undef
 50903|     ;; self = ptr undef
 50904|     ;; self = ptr undef
 50905|     ;; other = ptr undef
 50906|  br i1 %238, label %494, label %414                                                                                    ;L900<985<739
 50907| 
 50908| 414: ; preds = %412
 50909|     ;; hp = i64 %413
 50910|     ;; start = i64 8
 50911|     ;; self = i64 8
 50912|     ;; iter[0..+8] = i64 9
 50913|     ;; j = i64 8
 50914|  %415 = load i64, ptr %239, , !!8                                                                                      ;L740
 50915|     ;; arrival = i64 %415
 50917|  %416 = icmp ugt i64 %415, %317                                                                                        ;L741
 50918|  %417 = icmp ule i64 %415, %315                                                                                        ;L741
 50919|  %418 = and i1 %416, %417                                                                                              ;L741
 50920|  br i1 %418, label %419, label %422                                                                                    ;L741
 50921| 
 50922| 419: ; preds = %414
 50923|  %420 = load i64, ptr %240, , !!8                                                                                      ;L740
 50924|     ;; dmg = i64 %420
 50925|  %421 = sub i64 %413, %420                                                                                             ;L742
 50926|     ;; hp = i64 %421
 50927|  br label %422                                                                                                         ;L741
 50928| 
 50929| 422: ; preds = %419, %414
 50930|  %423 = phi i64 [ %421, %419 ], [ %413, %414 ]                                                                         ;L0
 50931|     ;; iter[0..+8] = i64 9
 50932|     ;; hp = i64 %423
 50933|     ;; self = ptr undef
 50934|     ;; self = ptr undef
 50935|     ;; self = ptr undef
 50936|     ;; other = ptr undef
 50937|  br i1 %241, label %494, label %424                                                                                    ;L900<985<739
 50938| 
 50939| 424: ; preds = %422
 50940|     ;; hp = i64 %423
 50941|     ;; start = i64 9
 50942|     ;; self = i64 9
 50943|     ;; iter[0..+8] = i64 10
 50944|     ;; j = i64 9
 50945|  %425 = load i64, ptr %242, , !!8                                                                                      ;L740
 50946|     ;; arrival = i64 %425
 50948|  %426 = icmp ugt i64 %425, %317                                                                                        ;L741
 50949|  %427 = icmp ule i64 %425, %315                                                                                        ;L741
 50950|  %428 = and i1 %426, %427                                                                                              ;L741
 50951|  br i1 %428, label %429, label %432                                                                                    ;L741
 50952| 
 50953| 429: ; preds = %424
 50954|  %430 = load i64, ptr %243, , !!8                                                                                      ;L740
 50955|     ;; dmg = i64 %430
 50956|  %431 = sub i64 %423, %430                                                                                             ;L742
 50957|     ;; hp = i64 %431
 50958|  br label %432                                                                                                         ;L741
 50959| 
 50960| 432: ; preds = %429, %424
 50961|  %433 = phi i64 [ %431, %429 ], [ %423, %424 ]                                                                         ;L0
 50962|     ;; iter[0..+8] = i64 10
 50963|     ;; hp = i64 %433
 50964|     ;; self = ptr undef
 50965|     ;; self = ptr undef
 50966|     ;; self = ptr undef
 50967|     ;; other = ptr undef
 50968|  br i1 %244, label %494, label %434                                                                                    ;L900<985<739
 50969| 
 50970| 434: ; preds = %432
 50971|     ;; hp = i64 %433
 50972|     ;; start = i64 10
 50973|     ;; self = i64 10
 50974|     ;; iter[0..+8] = i64 11
 50975|     ;; j = i64 10
 50976|  %435 = load i64, ptr %245, , !!8                                                                                      ;L740
 50977|     ;; arrival = i64 %435
 50979|  %436 = icmp ugt i64 %435, %317                                                                                        ;L741
 50980|  %437 = icmp ule i64 %435, %315                                                                                        ;L741
 50981|  %438 = and i1 %436, %437                                                                                              ;L741
 50982|  br i1 %438, label %439, label %442                                                                                    ;L741
 50983| 
 50984| 439: ; preds = %434
 50985|  %440 = load i64, ptr %246, , !!8                                                                                      ;L740
 50986|     ;; dmg = i64 %440
 50987|  %441 = sub i64 %433, %440                                                                                             ;L742
 50988|     ;; hp = i64 %441
 50989|  br label %442                                                                                                         ;L741
 50990| 
 50991| 442: ; preds = %439, %434
 50992|  %443 = phi i64 [ %441, %439 ], [ %433, %434 ]                                                                         ;L0
 50993|     ;; iter[0..+8] = i64 11
 50994|     ;; hp = i64 %443
 50995|     ;; self = ptr undef
 50996|     ;; self = ptr undef
 50997|     ;; self = ptr undef
 50998|     ;; other = ptr undef
 50999|  br i1 %247, label %494, label %444                                                                                    ;L900<985<739
 51000| 
 51001| 444: ; preds = %442
 51002|     ;; hp = i64 %443
 51003|     ;; start = i64 11
 51004|     ;; self = i64 11
 51005|     ;; iter[0..+8] = i64 12
 51006|     ;; j = i64 11
 51007|  %445 = load i64, ptr %248, , !!8                                                                                      ;L740
 51008|     ;; arrival = i64 %445
 51010|  %446 = icmp ugt i64 %445, %317                                                                                        ;L741
 51011|  %447 = icmp ule i64 %445, %315                                                                                        ;L741
 51012|  %448 = and i1 %446, %447                                                                                              ;L741
 51013|  br i1 %448, label %449, label %452                                                                                    ;L741
 51014| 
 51015| 449: ; preds = %444
 51016|  %450 = load i64, ptr %249, , !!8                                                                                      ;L740
 51017|     ;; dmg = i64 %450
 51018|  %451 = sub i64 %443, %450                                                                                             ;L742
 51019|     ;; hp = i64 %451
 51020|  br label %452                                                                                                         ;L741
 51021| 
 51022| 452: ; preds = %449, %444
 51023|  %453 = phi i64 [ %451, %449 ], [ %443, %444 ]                                                                         ;L0
 51024|     ;; iter[0..+8] = i64 12
 51025|     ;; hp = i64 %453
 51026|     ;; self = ptr undef
 51027|     ;; self = ptr undef
 51028|     ;; self = ptr undef
 51029|     ;; other = ptr undef
 51030|  br i1 %250, label %494, label %454                                                                                    ;L900<985<739
 51031| 
 51032| 454: ; preds = %452
 51033|     ;; hp = i64 %453
 51034|     ;; start = i64 12
 51035|     ;; self = i64 12
 51036|     ;; iter[0..+8] = i64 13
 51037|     ;; j = i64 12
 51038|  %455 = load i64, ptr %251, , !!8                                                                                      ;L740
 51039|     ;; arrival = i64 %455
 51041|  %456 = icmp ugt i64 %455, %317                                                                                        ;L741
 51042|  %457 = icmp ule i64 %455, %315                                                                                        ;L741
 51043|  %458 = and i1 %456, %457                                                                                              ;L741
 51044|  br i1 %458, label %459, label %462                                                                                    ;L741
 51045| 
 51046| 459: ; preds = %454
 51047|  %460 = load i64, ptr %252, , !!8                                                                                      ;L740
 51048|     ;; dmg = i64 %460
 51049|  %461 = sub i64 %453, %460                                                                                             ;L742
 51050|     ;; hp = i64 %461
 51051|  br label %462                                                                                                         ;L741
 51052| 
 51053| 462: ; preds = %459, %454
 51054|  %463 = phi i64 [ %461, %459 ], [ %453, %454 ]                                                                         ;L0
 51055|     ;; iter[0..+8] = i64 13
 51056|     ;; hp = i64 %463
 51057|     ;; self = ptr undef
 51058|     ;; self = ptr undef
 51059|     ;; self = ptr undef
 51060|     ;; other = ptr undef
 51061|  br i1 %253, label %494, label %464                                                                                    ;L900<985<739
 51062| 
 51063| 464: ; preds = %462
 51064|     ;; hp = i64 %463
 51065|     ;; start = i64 13
 51066|     ;; self = i64 13
 51067|     ;; iter[0..+8] = i64 14
 51068|     ;; j = i64 13
 51069|  %465 = load i64, ptr %254, , !!8                                                                                      ;L740
 51070|     ;; arrival = i64 %465
 51072|  %466 = icmp ugt i64 %465, %317                                                                                        ;L741
 51073|  %467 = icmp ule i64 %465, %315                                                                                        ;L741
 51074|  %468 = and i1 %466, %467                                                                                              ;L741
 51075|  br i1 %468, label %469, label %472                                                                                    ;L741
 51076| 
 51077| 469: ; preds = %464
 51078|  %470 = load i64, ptr %255, , !!8                                                                                      ;L740
 51079|     ;; dmg = i64 %470
 51080|  %471 = sub i64 %463, %470                                                                                             ;L742
 51081|     ;; hp = i64 %471
 51082|  br label %472                                                                                                         ;L741
 51083| 
 51084| 472: ; preds = %469, %464
 51085|  %473 = phi i64 [ %471, %469 ], [ %463, %464 ]                                                                         ;L0
 51086|     ;; iter[0..+8] = i64 14
 51087|     ;; hp = i64 %473
 51088|     ;; self = ptr undef
 51089|     ;; self = ptr undef
 51090|     ;; self = ptr undef
 51091|     ;; other = ptr undef
 51092|  br i1 %256, label %494, label %474                                                                                    ;L900<985<739
 51093| 
 51094| 474: ; preds = %472
 51095|     ;; hp = i64 %473
 51096|     ;; start = i64 14
 51097|     ;; self = i64 14
 51098|     ;; iter[0..+8] = i64 15
 51099|     ;; j = i64 14
 51100|  %475 = load i64, ptr %257, , !!8                                                                                      ;L740
 51101|     ;; arrival = i64 %475
 51103|  %476 = icmp ugt i64 %475, %317                                                                                        ;L741
 51104|  %477 = icmp ule i64 %475, %315                                                                                        ;L741
 51105|  %478 = and i1 %476, %477                                                                                              ;L741
 51106|  br i1 %478, label %479, label %482                                                                                    ;L741
 51107| 
 51108| 479: ; preds = %474
 51109|  %480 = load i64, ptr %258, , !!8                                                                                      ;L740
 51110|     ;; dmg = i64 %480
 51111|  %481 = sub i64 %473, %480                                                                                             ;L742
 51112|     ;; hp = i64 %481
 51113|  br label %482                                                                                                         ;L741
 51114| 
 51115| 482: ; preds = %479, %474
 51116|  %483 = phi i64 [ %481, %479 ], [ %473, %474 ]                                                                         ;L0
 51117|     ;; iter[0..+8] = i64 15
 51118|     ;; hp = i64 %483
 51119|     ;; self = ptr undef
 51120|     ;; self = ptr undef
 51121|     ;; self = ptr undef
 51122|     ;; other = ptr undef
 51123|  br i1 %259, label %494, label %484                                                                                    ;L900<985<739
 51124| 
 51125| 484: ; preds = %482
 51126|     ;; hp = i64 %483
 51127|     ;; start = i64 15
 51128|     ;; self = i64 15
 51129|     ;; iter[0..+8] = i64 16
 51130|     ;; j = i64 15
 51131|  %485 = load i64, ptr %260, , !!8                                                                                      ;L740
 51132|     ;; arrival = i64 %485
 51134|  %486 = icmp ugt i64 %485, %317                                                                                        ;L741
 51135|  %487 = icmp ule i64 %485, %315                                                                                        ;L741
 51136|  %488 = and i1 %486, %487                                                                                              ;L741
 51137|  br i1 %488, label %489, label %492                                                                                    ;L741
 51138| 
 51139| 489: ; preds = %484
 51140|  %490 = load i64, ptr %261, , !!8                                                                                      ;L740
 51141|     ;; dmg = i64 %490
 51142|  %491 = sub i64 %483, %490                                                                                             ;L742
 51143|     ;; hp = i64 %491
 51144|  br label %492                                                                                                         ;L741
 51145| 
 51146| 492: ; preds = %489, %484
 51147|  %493 = phi i64 [ %491, %489 ], [ %483, %484 ]                                                                         ;L0
 51148|     ;; iter[0..+8] = i64 16
 51149|     ;; hp = i64 %493
 51150|     ;; self = ptr undef
 51151|     ;; self = ptr undef
 51152|     ;; self = ptr undef
 51153|     ;; other = ptr undef
 51154|  br i1 %262, label %494, label %587                                                                                    ;L900<985<739
 51155| 
 51156| 494: ; preds = %492, %482, %472, %462, %452, %442, %432, %422, %412, %402, %392, %382, %372, %362, %352, %342
 51157|  %495 = phi i64 [ %343, %342 ], [ %353, %352 ], [ %363, %362 ], [ %373, %372 ], [ %383, %382 ], [ %393, %392 ], [ %403, %402 ], [ %413, %412 ], [ %423, %422 ], [ %433, %432 ], [ %443, %442 ], [ %453, %452 ], [ %463, %462 ], [ %473, %472 ], [ %483, %482 ], [ %493, %492 ] ;L0
 51158|  %496 = getelementptr i64, ptr %202, i64 %311                                                                          ;L746
 51159|  store i64 %495, ptr %496,                                                                                             ;L746
 51160|  store i64 %314, ptr %212,                                                                                             ;L747
 51161|  %497 = icmp slt i64 %495, 1                                                                                           ;L749
 51162|  %498 = icmp eq i64 %312, -1                                                                                           ;L749
 51163|  %499 = select i1 %497, i1 %498, i1 false                                                                              ;L749
 51164|  br i1 %499, label %321, label %336                                                                                    ;L749
 51165| 
 51166| 500: ; preds = %185
 51167|  %501 = gep %74, i64 528                                                                                               ;L689
 51168|  %502 = load ptr, ptr %501, , !!8                                                                                      ;L689
 51169|  invoke void %502(ptr sret([40 x i8]) %7, ptr %72)
 51170|  to label %503 unwind label %87                                                                                        ;L689
 51171| 
 51172| 503: ; preds = %500
 51174|  call void @llvm.memcpy.p0.p0.i64(ptr %6, ptr %7, i64 40, i1 false)                                                    ;L689
 51175|  %504 = icmp eq i64 %4, 2
 51176|  %505 = gep %74, i64 496
 51177|  br label %506                                                                                                         ;L689
 51178| 
 51179| 506: ; preds = %648, %503
 51180|  %507 = invoke ptr @gc::simulationNtB5_14ProjectileIterNtNtNtNtCsjihNppCmMEE_4core4iter6traits8iterator8Iterator4next(ptr %6)
 51181|  to label %508 unwind label %87                                                                                        ;L689
 51182| 
 51183| 508: ; preds = %506
 51184|  %509 = icmp eq ptr %507, null                                                                                         ;L689
 51185|  br i1 %509, label %515, label %510                                                                                    ;L689
 51186| 
 51187| 510: ; preds = %508
 51188|     ;; p = ptr %507
 51189|  %511 = gep %507, i64 64                                                                                               ;L690
 51190|  %512 = load i64, ptr %511, , !!8                                                                                      ;L690
 51191|  %513 = icmp ne i64 %512, 9                                                                                            ;L690
 51192|  call void @llvm.assume(i1 %513)                                                                                       ;L690
 51193|  %514 = icmp eq i64 %512, 6                                                                                            ;L690
 51194|  br i1 %514, label %590, label %605                                                                                    ;L690
 51195| 
 51196| 515: ; preds = %508
 51198|  br label %187                                                                                                         ;L688
 51199| 
 51200| 516: ; preds = %580, %574, %568, %562, %556, %550, %544, %538, %532, %526, %520, %306, %194
 51201|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %18, i64 2320, i1 false)                                                 ;L764
 51208|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %17)
 51209|  to label %172 unwind label %517                                                                                       ;L825<765
 51210| 
 51211| 517: ; preds = %516
 51212|  %518 = cleanuppad within none []
 51214|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %17) [ "funclet"(token %518) ] ;L825<825<765
 51215|  cleanupret from %518 unwind to caller                                                                                 ;L825<765
 51216| 
 51217| 519: ; preds = %580, %197
 51218|  invoke void @core::panicking18panic_bounds_check(i64 12, i64 12, ptr @anon.168add0ea037d45d276f5936ae758fe5.225) #30
 51219|  to label %140 unwind label %87                                                                                        ;L725
 51220| 
 51221| 520: ; preds = %194
 51222|     ;; old = i64 1
 51223|     ;; start = i64 1
 51224|     ;; self = i64 1
 51225|     ;; slot = i64 1
 51226|     ;; iter[0..+8] = i64 2
 51227|     ;; traj = ptr %35
 51228|  %521 = gep %12, i64 8                                                                                                 ;L726
 51229|  %522 = load i64, ptr %521, , !!8                                                                                      ;L726
 51230|  %523 = gep %18, i64 360                                                                                               ;L726
 51231|  store i64 %522, ptr %523,                                                                                             ;L726
 51232|     ;; num_checkpoints = i64 %189
 51233|     ;; iter[8..+8] = i64 %189
 51234|     ;; iter[0..+8] = i64 0
 51235|     ;; death_tick = i64 -1
 51237|     ;; self = ptr undef
 51238|     ;; self = ptr undef
 51239|     ;; self = ptr undef
 51240|     ;; other = ptr undef
 51241|  %524 = gep %18, i64 376                                                                                               ;L761
 51242|  store i64 -1, ptr %524,                                                                                               ;L761
 51243|     ;; self = ptr undef
 51244|     ;; self = ptr undef
 51245|     ;; self = ptr undef
 51246|     ;; other = ptr undef
 51247|  %525 = icmp eq i64 %97, 2                                                                                             ;L1916<900<985<724
 51248|  br i1 %525, label %516, label %526                                                                                    ;L900<985<724
 51249| 
 51250| 526: ; preds = %520
 51251|     ;; old = i64 2
 51252|     ;; start = i64 2
 51253|     ;; self = i64 2
 51254|     ;; slot = i64 2
 51255|     ;; iter[0..+8] = i64 3
 51256|     ;; traj = ptr %36
 51257|  %527 = gep %12, i64 16                                                                                                ;L726
 51258|  %528 = load i64, ptr %527, , !!8                                                                                      ;L726
 51259|  %529 = gep %18, i64 552                                                                                               ;L726
 51260|  store i64 %528, ptr %529,                                                                                             ;L726
 51261|     ;; num_checkpoints = i64 %189
 51262|     ;; iter[8..+8] = i64 %189
 51263|     ;; iter[0..+8] = i64 0
 51264|     ;; death_tick = i64 -1
 51266|     ;; self = ptr undef
 51267|     ;; self = ptr undef
 51268|     ;; self = ptr undef
 51269|     ;; other = ptr undef
 51270|  %530 = gep %18, i64 568                                                                                               ;L761
 51271|  store i64 -1, ptr %530,                                                                                               ;L761
 51272|     ;; self = ptr undef
 51273|     ;; self = ptr undef
 51274|     ;; self = ptr undef
 51275|     ;; other = ptr undef
 51276|  %531 = icmp eq i64 %97, 3                                                                                             ;L1916<900<985<724
 51277|  br i1 %531, label %516, label %532                                                                                    ;L900<985<724
 51278| 
 51279| 532: ; preds = %526
 51280|     ;; old = i64 3
 51281|     ;; start = i64 3
 51282|     ;; self = i64 3
 51283|     ;; slot = i64 3
 51284|     ;; iter[0..+8] = i64 4
 51285|     ;; traj = ptr %37
 51286|  %533 = gep %12, i64 24                                                                                                ;L726
 51287|  %534 = load i64, ptr %533, , !!8                                                                                      ;L726
 51288|  %535 = gep %18, i64 744                                                                                               ;L726
 51289|  store i64 %534, ptr %535,                                                                                             ;L726
 51290|     ;; num_checkpoints = i64 %189
 51291|     ;; iter[8..+8] = i64 %189
 51292|     ;; iter[0..+8] = i64 0
 51293|     ;; death_tick = i64 -1
 51295|     ;; self = ptr undef
 51296|     ;; self = ptr undef
 51297|     ;; self = ptr undef
 51298|     ;; other = ptr undef
 51299|  %536 = gep %18, i64 760                                                                                               ;L761
 51300|  store i64 -1, ptr %536,                                                                                               ;L761
 51301|     ;; self = ptr undef
 51302|     ;; self = ptr undef
 51303|     ;; self = ptr undef
 51304|     ;; other = ptr undef
 51305|  %537 = icmp eq i64 %97, 4                                                                                             ;L1916<900<985<724
 51306|  br i1 %537, label %516, label %538                                                                                    ;L900<985<724
 51307| 
 51308| 538: ; preds = %532
 51309|     ;; old = i64 4
 51310|     ;; start = i64 4
 51311|     ;; self = i64 4
 51312|     ;; slot = i64 4
 51313|     ;; iter[0..+8] = i64 5
 51314|     ;; traj = ptr %38
 51315|  %539 = gep %12, i64 32                                                                                                ;L726
 51316|  %540 = load i64, ptr %539, , !!8                                                                                      ;L726
 51317|  %541 = gep %18, i64 936                                                                                               ;L726
 51318|  store i64 %540, ptr %541,                                                                                             ;L726
 51319|     ;; num_checkpoints = i64 %189
 51320|     ;; iter[8..+8] = i64 %189
 51321|     ;; iter[0..+8] = i64 0
 51322|     ;; death_tick = i64 -1
 51324|     ;; self = ptr undef
 51325|     ;; self = ptr undef
 51326|     ;; self = ptr undef
 51327|     ;; other = ptr undef
 51328|  %542 = gep %18, i64 952                                                                                               ;L761
 51329|  store i64 -1, ptr %542,                                                                                               ;L761
 51330|     ;; self = ptr undef
 51331|     ;; self = ptr undef
 51332|     ;; self = ptr undef
 51333|     ;; other = ptr undef
 51334|  %543 = icmp eq i64 %97, 5                                                                                             ;L1916<900<985<724
 51335|  br i1 %543, label %516, label %544                                                                                    ;L900<985<724
 51336| 
 51337| 544: ; preds = %538
 51338|     ;; old = i64 5
 51339|     ;; start = i64 5
 51340|     ;; self = i64 5
 51341|     ;; slot = i64 5
 51342|     ;; iter[0..+8] = i64 6
 51343|     ;; traj = ptr %39
 51344|  %545 = gep %12, i64 40                                                                                                ;L726
 51345|  %546 = load i64, ptr %545, , !!8                                                                                      ;L726
 51346|  %547 = gep %18, i64 1128                                                                                              ;L726
 51347|  store i64 %546, ptr %547,                                                                                             ;L726
 51348|     ;; num_checkpoints = i64 %189
 51349|     ;; iter[8..+8] = i64 %189
 51350|     ;; iter[0..+8] = i64 0
 51351|     ;; death_tick = i64 -1
 51353|     ;; self = ptr undef
 51354|     ;; self = ptr undef
 51355|     ;; self = ptr undef
 51356|     ;; other = ptr undef
 51357|  %548 = gep %18, i64 1144                                                                                              ;L761
 51358|  store i64 -1, ptr %548,                                                                                               ;L761
 51359|     ;; self = ptr undef
 51360|     ;; self = ptr undef
 51361|     ;; self = ptr undef
 51362|     ;; other = ptr undef
 51363|  %549 = icmp eq i64 %97, 6                                                                                             ;L1916<900<985<724
 51364|  br i1 %549, label %516, label %550                                                                                    ;L900<985<724
 51365| 
 51366| 550: ; preds = %544
 51367|     ;; old = i64 6
 51368|     ;; start = i64 6
 51369|     ;; self = i64 6
 51370|     ;; slot = i64 6
 51371|     ;; iter[0..+8] = i64 7
 51372|     ;; traj = ptr %40
 51373|  %551 = gep %12, i64 48                                                                                                ;L726
 51374|  %552 = load i64, ptr %551, , !!8                                                                                      ;L726
 51375|  %553 = gep %18, i64 1320                                                                                              ;L726
 51376|  store i64 %552, ptr %553,                                                                                             ;L726
 51377|     ;; num_checkpoints = i64 %189
 51378|     ;; iter[8..+8] = i64 %189
 51379|     ;; iter[0..+8] = i64 0
 51380|     ;; death_tick = i64 -1
 51382|     ;; self = ptr undef
 51383|     ;; self = ptr undef
 51384|     ;; self = ptr undef
 51385|     ;; other = ptr undef
 51386|  %554 = gep %18, i64 1336                                                                                              ;L761
 51387|  store i64 -1, ptr %554,                                                                                               ;L761
 51388|     ;; self = ptr undef
 51389|     ;; self = ptr undef
 51390|     ;; self = ptr undef
 51391|     ;; other = ptr undef
 51392|  %555 = icmp eq i64 %97, 7                                                                                             ;L1916<900<985<724
 51393|  br i1 %555, label %516, label %556                                                                                    ;L900<985<724
 51394| 
 51395| 556: ; preds = %550
 51396|     ;; old = i64 7
 51397|     ;; start = i64 7
 51398|     ;; self = i64 7
 51399|     ;; slot = i64 7
 51400|     ;; iter[0..+8] = i64 8
 51401|     ;; traj = ptr %41
 51402|  %557 = gep %12, i64 56                                                                                                ;L726
 51403|  %558 = load i64, ptr %557, , !!8                                                                                      ;L726
 51404|  %559 = gep %18, i64 1512                                                                                              ;L726
 51405|  store i64 %558, ptr %559,                                                                                             ;L726
 51406|     ;; num_checkpoints = i64 %189
 51407|     ;; iter[8..+8] = i64 %189
 51408|     ;; iter[0..+8] = i64 0
 51409|     ;; death_tick = i64 -1
 51411|     ;; self = ptr undef
 51412|     ;; self = ptr undef
 51413|     ;; self = ptr undef
 51414|     ;; other = ptr undef
 51415|  %560 = gep %18, i64 1528                                                                                              ;L761
 51416|  store i64 -1, ptr %560,                                                                                               ;L761
 51417|     ;; self = ptr undef
 51418|     ;; self = ptr undef
 51419|     ;; self = ptr undef
 51420|     ;; other = ptr undef
 51421|  %561 = icmp eq i64 %97, 8                                                                                             ;L1916<900<985<724
 51422|  br i1 %561, label %516, label %562                                                                                    ;L900<985<724
 51423| 
 51424| 562: ; preds = %556
 51425|     ;; old = i64 8
 51426|     ;; start = i64 8
 51427|     ;; self = i64 8
 51428|     ;; slot = i64 8
 51429|     ;; iter[0..+8] = i64 9
 51430|     ;; traj = ptr %42
 51431|  %563 = gep %12, i64 64                                                                                                ;L726
 51432|  %564 = load i64, ptr %563, , !!8                                                                                      ;L726
 51433|  %565 = gep %18, i64 1704                                                                                              ;L726
 51434|  store i64 %564, ptr %565,                                                                                             ;L726
 51435|     ;; num_checkpoints = i64 %189
 51436|     ;; iter[8..+8] = i64 %189
 51437|     ;; iter[0..+8] = i64 0
 51438|     ;; death_tick = i64 -1
 51440|     ;; self = ptr undef
 51441|     ;; self = ptr undef
 51442|     ;; self = ptr undef
 51443|     ;; other = ptr undef
 51444|  %566 = gep %18, i64 1720                                                                                              ;L761
 51445|  store i64 -1, ptr %566,                                                                                               ;L761
 51446|     ;; self = ptr undef
 51447|     ;; self = ptr undef
 51448|     ;; self = ptr undef
 51449|     ;; other = ptr undef
 51450|  %567 = icmp eq i64 %97, 9                                                                                             ;L1916<900<985<724
 51451|  br i1 %567, label %516, label %568                                                                                    ;L900<985<724
 51452| 
 51453| 568: ; preds = %562
 51454|     ;; old = i64 9
 51455|     ;; start = i64 9
 51456|     ;; self = i64 9
 51457|     ;; slot = i64 9
 51458|     ;; iter[0..+8] = i64 10
 51459|     ;; traj = ptr %43
 51460|  %569 = gep %12, i64 72                                                                                                ;L726
 51461|  %570 = load i64, ptr %569, , !!8                                                                                      ;L726
 51462|  %571 = gep %18, i64 1896                                                                                              ;L726
 51463|  store i64 %570, ptr %571,                                                                                             ;L726
 51464|     ;; num_checkpoints = i64 %189
 51465|     ;; iter[8..+8] = i64 %189
 51466|     ;; iter[0..+8] = i64 0
 51467|     ;; death_tick = i64 -1
 51469|     ;; self = ptr undef
 51470|     ;; self = ptr undef
 51471|     ;; self = ptr undef
 51472|     ;; other = ptr undef
 51473|  %572 = gep %18, i64 1912                                                                                              ;L761
 51474|  store i64 -1, ptr %572,                                                                                               ;L761
 51475|     ;; self = ptr undef
 51476|     ;; self = ptr undef
 51477|     ;; self = ptr undef
 51478|     ;; other = ptr undef
 51479|  %573 = icmp eq i64 %97, 10                                                                                            ;L1916<900<985<724
 51480|  br i1 %573, label %516, label %574                                                                                    ;L900<985<724
 51481| 
 51482| 574: ; preds = %568
 51483|     ;; old = i64 10
 51484|     ;; start = i64 10
 51485|     ;; self = i64 10
 51486|     ;; slot = i64 10
 51487|     ;; iter[0..+8] = i64 11
 51488|     ;; traj = ptr %44
 51489|  %575 = gep %12, i64 80                                                                                                ;L726
 51490|  %576 = load i64, ptr %575, , !!8                                                                                      ;L726
 51491|  %577 = gep %18, i64 2088                                                                                              ;L726
 51492|  store i64 %576, ptr %577,                                                                                             ;L726
 51493|     ;; num_checkpoints = i64 %189
 51494|     ;; iter[8..+8] = i64 %189
 51495|     ;; iter[0..+8] = i64 0
 51496|     ;; death_tick = i64 -1
 51498|     ;; self = ptr undef
 51499|     ;; self = ptr undef
 51500|     ;; self = ptr undef
 51501|     ;; other = ptr undef
 51502|  %578 = gep %18, i64 2104                                                                                              ;L761
 51503|  store i64 -1, ptr %578,                                                                                               ;L761
 51504|     ;; self = ptr undef
 51505|     ;; self = ptr undef
 51506|     ;; self = ptr undef
 51507|     ;; other = ptr undef
 51508|  %579 = icmp eq i64 %97, 11                                                                                            ;L1916<900<985<724
 51509|  br i1 %579, label %516, label %580                                                                                    ;L900<985<724
 51510| 
 51511| 580: ; preds = %574
 51512|     ;; old = i64 11
 51513|     ;; start = i64 11
 51514|     ;; self = i64 11
 51515|     ;; slot = i64 11
 51516|     ;; iter[0..+8] = i64 12
 51517|     ;; traj = ptr %34
 51518|  %581 = gep %12, i64 88                                                                                                ;L726
 51519|  %582 = load i64, ptr %581, , !!8                                                                                      ;L726
 51520|  %583 = gep %18, i64 2280                                                                                              ;L726
 51521|  store i64 %582, ptr %583,                                                                                             ;L726
 51522|     ;; num_checkpoints = i64 %189
 51523|     ;; iter[8..+8] = i64 %189
 51524|     ;; iter[0..+8] = i64 0
 51525|     ;; death_tick = i64 -1
 51527|     ;; self = ptr undef
 51528|     ;; self = ptr undef
 51529|     ;; self = ptr undef
 51530|     ;; other = ptr undef
 51531|  %584 = gep %18, i64 2296                                                                                              ;L761
 51532|  store i64 -1, ptr %584,                                                                                               ;L761
 51533|     ;; self = ptr undef
 51534|     ;; self = ptr undef
 51535|     ;; self = ptr undef
 51536|     ;; other = ptr undef
 51537|  %585 = icmp eq i64 %97, 12                                                                                            ;L1916<900<985<724
 51538|  br i1 %585, label %516, label %519                                                                                    ;L900<985<724
 51539| 
 51540| 586: ; preds = %327, %294, %268
 51541|  invoke void @core::panicking11panic_const24panic_const_div_overflow(ptr @anon.168add0ea037d45d276f5936ae758fe5.226) #30
 51542|  to label %140 unwind label %87                                                                                        ;L753
 51543| 
 51544| 587: ; preds = %492
 51545|  %588 = add i64 %210, -1
 51546|  %589 = call i64 @llvm.umin.i64(i64 %588, i64 16)
 51547|     ;; hp = i64 %493
 51548|     ;; start = i64 16
 51549|     ;; self = i64 16
 51550|     ;; iter[0..+8] = i64 17
 51551|     ;; j = i64 16
 51552|  invoke void @core::panicking18panic_bounds_check(i64 %589, i64 16, ptr @anon.168add0ea037d45d276f5936ae758fe5.227) #30
 51553|  to label %140 unwind label %87                                                                                        ;L740
 51554| 
 51555| 590: ; preds = %510
 51556|     ;; target_id = ptr %507
 51557|  %591 = gep %507, i64 72                                                                                               ;L690
 51558|     ;; speed = ptr %591
 51559|  %592 = gep %507, i64 80                                                                                               ;L691
 51560|  %593 = load i64, ptr %592, , !!8                                                                                      ;L691
 51564|     ;; id = i64 %593
 51566|     ;; target_count = ptr %13
 51567|     ;; self[0..+8] = i64 0
 51568|     ;; rhs = i64 0
 51569|     ;; self = ptr %13
 51570|     ;; index = i64 %97
 51571|     ;; index = i64 %97
 51572|     ;; self = i64 %97
 51573|     ;; self[8..+8] = i64 %97
 51574|     ;; new_len = i64 %97
 51575|     ;; self = i64 %97
 51576|     ;; self[0..+8] = ptr %13
 51577|     ;; slice[0..+8] = ptr %13
 51578|     ;; slice[0..+8] = ptr %13
 51579|     ;; self[8..+8] = i64 12
 51580|     ;; slice[8..+8] = i64 12
 51581|     ;; slice[8..+8] = i64 12
 51582|  br i1 %175, label %594, label %603                                                                                    ;L1050<437<529<19<391<655<691
 51583| 
 51584| 594: ; preds = %599, %590
 51585|  %595 = phi i64 [ %601, %599 ], [ 0, %590 ]
 51586|  %596 = phi ptr [ %600, %599 ], [ %13, %590 ]
 51587|     ;; i = i64 %595
 51588|     ;; ptr = ptr %596
 51589|     ;; x = ptr %596
 51590|  %597 = load i64, ptr %596, , !!61465, !!8                                                                             ;L384<655<691
 51593|     ;; t = i64 %597
 51594|  %598 = icmp eq i64 %597, %593                                                                                         ;L655<384<655<691
 51595|  br i1 %598, label %606, label %599                                                                                    ;L384<655<691
 51596| 
 51597| 599: ; preds = %594
 51598|  %600 = gep %596, i64 8                                                                                                ;L656<185<383<655<691
 51599|  %601 = add nuw nsw i64 %595, 1                                                                                        ;L390<655<691
 51600|     ;; i = i64 %601
 51601|     ;; ptr = ptr %600
 51602|     ;; self = ptr %600
 51603|     ;; end_or_len = ptr %177
 51606|  %602 = icmp eq ptr %600, %177                                                                                         ;L1714<180<383<655<691
 51607|  br i1 %602, label %648, label %594                                                                                    ;L180<383<655<691
 51608| 
 51609| 603: ; preds = %590
 51610|  invoke void @core::slice5index16slice_index_fail(i64 0, i64 %97, i64 12, ptr @anon.168add0ea037d45d276f5936ae758fe5.36) #30
 51611|  to label %604 unwind label %87                                                                                        ;L443<529<19<391<655<691
 51612| 
 51613| 604: ; preds = %603
 51614|  unreachable                                                                                                           ;L443<529<19<391<655<691
 51615| 
 51616| 605: ; preds = %510
 51617|  br i1 %504, label %648, label %649                                                                                    ;L703
 51618| 
 51619| 606: ; preds = %594
 51620|  %607 = icmp ult i64 %595, %97                                                                                         ;L387<655<691
 51621|     ;; cond = i1 true
 51622|  call void @llvm.assume(i1 %607)                                                                                       ;L210<387<655<691
 51623|     ;; slot = i64 %595
 51624|     ;; index = i64 %595
 51625|     ;; index = i64 %595
 51626|     ;; self = i64 %595
 51627|     ;; self = ptr %17
 51628|     ;; self = ptr %17
 51629|     ;; self = ptr %17
 51631|  %608 = load i64, ptr %83, , !!8                                                                                       ;L2075<2054<692
 51634|     ;; self[8..+8] = i64 %608
 51635|     ;; slice[8..+8] = i64 %608
 51636|  %609 = icmp ult i64 %595, %608                                                                                        ;L272<19<2054<692
 51637|  br i1 %609, label %610, label %618                                                                                    ;L272<19<2054<692
 51638| 
 51639| 610: ; preds = %606
 51640|  %611 = load ptr, ptr %17, , !!8, !!8                                                                                  ;L138<2073<2054<692
 51641|     ;; p = ptr %611
 51642|     ;; self[0..+8] = ptr %611
 51643|     ;; slice[0..+8] = ptr %611
 51644|  %612 = getelementptr ptr, ptr %611, i64 %595                                                                          ;L272<19<2054<692
 51645|  %613 = load ptr, ptr %612, , !!8, !!8                                                                                 ;L692
 51646|     ;; e = ptr %613
 51647|  %614 = gep %507, i64 248                                                                                              ;L693
 51648|  %615 = load i64, ptr %614, , !!8                                                                                      ;L693
 51649|  %616 = load ptr, ptr %505, , !!8                                                                                      ;L693
 51650|  %617 = invoke ptr %616(ptr %72, i64 %615)
 51651|  to label %619 unwind label %87                                                                                        ;L693
 51652| 
 51653| 618: ; preds = %606
 51654|  invoke void @core::panicking18panic_bounds_check(i64 %595, i64 %608, ptr @anon.168add0ea037d45d276f5936ae758fe5.228) #30
 51655|  to label %140 unwind label %87                                                                                        ;L272<19<2054<692
 51656| 
 51657| 619: ; preds = %610
 51658|  %620 = icmp eq ptr %617, null                                                                                         ;L693
 51659|  br i1 %620, label %648, label %621                                                                                    ;L693
 51660| 
 51661| 621: ; preds = %619
 51662|     ;; caster = ptr %617
 51663|  %622 = gep %507, i64 256                                                                                              ;L694
 51664|  %623 = load i64, ptr %622, , !!8                                                                                      ;L694
 51665|  %624 = gep %507, i64 264                                                                                              ;L694
 51666|  %625 = load i64, ptr %624, , !!8                                                                                      ;L694
 51667|  %626 = gep %613, i64 1632                                                                                             ;L694
 51668|  %627 = load i64, ptr %626, , !!8                                                                                      ;L694
 51669|  %628 = gep %613, i64 1640                                                                                             ;L694
 51670|  %629 = load i64, ptr %628, , !!8                                                                                      ;L694
 51671|  %630 = invoke i64 @gc::utils8distance(i64 %623, i64 %625, i64 %627, i64 %629)
 51672|  to label %631 unwind label %87                                                                                        ;L694
 51673| 
 51674| 631: ; preds = %621
 51675|     ;; dist = i64 %630
 51676|     ;; self = i64 %630
 51677|     ;; self = i64 %630
 51678|     ;; self = ptr %591
 51683|     ;; other = ptr %591
 51686|  %632 = load i64, ptr %591, , !!61608, !!8                                                                             ;L1916<2142<1038<695
 51687|  %633 = call i64 @llvm.umax.i64(i64 %632, i64 1)                                                                       ;L39<695
 51688|     ;; other = i64 %633
 51689|  %634 = udiv i64 %630, %633                                                                                            ;L493<39<695
 51690|  %635 = add i64 %634, 5                                                                                                ;L695
 51691|     ;; arrival_tick = i64 %635
 51692|  %636 = invoke i64 @gc::simulation10projectileNtB5_10Projectile22expected_damage_target(ptr %507, ptr %79, ptr %617, ptr %613)
 51693|  to label %637 unwind label %87                                                                                        ;L696
 51694| 
 51695| 637: ; preds = %631
 51696|     ;; dmg = i64 %636
 51697|  %638 = getelementptr i64, ptr %10, i64 %595                                                                           ;L697
 51698|  %639 = load i64, ptr %638, , !!8                                                                                      ;L697
 51699|  %640 = icmp ult i64 %639, 16                                                                                          ;L697
 51700|  %641 = icmp ule i64 %635, %3                                                                                          ;L697
 51701|  %642 = and i1 %641, %640                                                                                              ;L697
 51702|  br i1 %642, label %643, label %648                                                                                    ;L697
 51703| 
 51704| 643: ; preds = %637
 51705|  %644 = getelementptr [16 x { i64, i64 }], ptr %11, i64 %595                                                           ;L698
 51706|  %645 = gepS %644, i64 %639                                                                                            ;L698
 51707|  store i64 %635, ptr %645,                                                                                             ;L698
 51708|  %646 = gep %645, i64 8                                                                                                ;L698
 51709|  store i64 %636, ptr %646,                                                                                             ;L698
 51710|  %647 = add nuw nsw i64 %639, 1                                                                                        ;L699
 51711|  store i64 %647, ptr %638,                                                                                             ;L699
 51712|  br label %648                                                                                                         ;L697
 51713| 
 51714| 648: ; preds = %669, %663, %656, %654, %643, %637, %619, %605, %599
 51715|  br label %506                                                                                                         ;L689
 51716| 
 51717| 649: ; preds = %605
 51718|  %650 = gep %507, i64 248                                                                                              ;L705
 51719|  %651 = load i64, ptr %650, , !!8                                                                                      ;L705
 51720|  %652 = load ptr, ptr %505, , !!8                                                                                      ;L705
 51721|  %653 = invoke ptr %652(ptr %72, i64 %651)
 51722|  to label %654 unwind label %87                                                                                        ;L705
 51723| 
 51724| 654: ; preds = %649
 51725|  %655 = icmp eq ptr %653, null                                                                                         ;L705
 51726|  br i1 %655, label %648, label %656                                                                                    ;L705
 51727| 
 51728| 656: ; preds = %654
 51729|     ;; caster = ptr %653
 51730|     ;; self = ptr %653
 51731|     ;; other = ptr %31
 51732|  %657 = load i64, ptr %653, , !!8                                                                                      ;L1127<706
 51733|  %658 = gep %653, i64 8                                                                                                ;L1127<706
 51734|     ;; __self_discr = i64 %657
 51735|  %659 = load i64, ptr %31, , !!8                                                                                       ;L1127<706
 51736|     ;; __arg1_discr = i64 %659
 51737|  %660 = icmp eq i64 %657, %659                                                                                         ;L1127<706
 51738|  br i1 %660, label %661, label %648                                                                                    ;L1127<706
 51739| 
 51740| 661: ; preds = %656
 51741|  %662 = icmp eq i64 %657, 0                                                                                            ;L1127<706
 51742|  br i1 %662, label %663, label %667                                                                                    ;L1127<706
 51743| 
 51744| 663: ; preds = %661
 51745|     ;; __self_0 = ptr %653
 51746|     ;; self = ptr %653
 51747|     ;; __arg1_0 = ptr %31
 51748|     ;; other = ptr %31
 51751|  %664 = load i64, ptr %658, , !!8                                                                                      ;L1878<2123<1127<706
 51752|  %665 = load i64, ptr %90, , !!8                                                                                       ;L1878<2123<1127<706
 51753|  %666 = icmp eq i64 %664, %665                                                                                         ;L1878<2123<1127<706
 51754|  br i1 %666, label %667, label %648                                                                                    ;L706
 51755| 
 51756| 667: ; preds = %663, %661
 51757|     ;; iter[0..+8] = i64 0
 51758|     ;; iter[8..+8] = i64 %97
 51759|  %668 = gep %507, i64 300
 51760|  br label %669                                                                                                         ;L707
 51761| 
 51762| 669: ; preds = %691, %667
 51763|  %670 = phi i64 [ 0, %667 ], [ %673, %691 ]                                                                            ;L707
 51764|     ;; iter[0..+8] = i64 %670
 51765|     ;; self = ptr undef
 51766|     ;; self = ptr undef
 51767|     ;; self = ptr undef
 51768|     ;; other = ptr undef
 51769|  %671 = icmp ult i64 %670, %97                                                                                         ;L1916<900<985<707
 51770|  br i1 %671, label %672, label %648                                                                                    ;L900<985<707
 51771| 
 51772| 672: ; preds = %669
 51773|     ;; old = i64 %670
 51774|     ;; start = i64 %670
 51775|     ;; self = i64 %670
 51776|  %673 = add nuw i64 %670, 1                                                                                            ;L971<215<903<985<707
 51777|     ;; iter[0..+8] = i64 %673
 51778|     ;; slot = i64 %670
 51779|     ;; index = i64 %670
 51780|     ;; index = i64 %670
 51781|     ;; self = i64 %670
 51782|     ;; self = ptr %17
 51783|     ;; self = ptr %17
 51784|     ;; self = ptr %17
 51786|  %674 = load i64, ptr %83, , !!8                                                                                       ;L2075<2054<708
 51789|     ;; self[8..+8] = i64 %674
 51790|     ;; slice[8..+8] = i64 %674
 51791|  %675 = icmp ult i64 %670, %674                                                                                        ;L272<19<2054<708
 51792|  br i1 %675, label %676, label %681                                                                                    ;L272<19<2054<708
 51793| 
 51794| 676: ; preds = %672
 51795|  %677 = load ptr, ptr %17, , !!8, !!8                                                                                  ;L138<2073<2054<708
 51796|     ;; p = ptr %677
 51797|     ;; self[0..+8] = ptr %677
 51798|     ;; slice[0..+8] = ptr %677
 51799|  %678 = getelementptr ptr, ptr %677, i64 %670                                                                          ;L272<19<2054<708
 51800|  %679 = load ptr, ptr %678, , !!8, !!8                                                                                 ;L708
 51801|     ;; e = ptr %679
 51802|     ;; self = ptr %679
 51803|  %680 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget16check_projectile(ptr %668, ptr %507, ptr %679)
 51804|  to label %682 unwind label %87                                                                                        ;L709
 51805| 
 51806| 681: ; preds = %672
 51807|  invoke void @core::panicking18panic_bounds_check(i64 %670, i64 %674, ptr @anon.168add0ea037d45d276f5936ae758fe5.229) #30
 51808|  to label %140 unwind label %87                                                                                        ;L272<19<2054<708
 51809| 
 51810| 682: ; preds = %676
 51811|  br i1 %680, label %683, label %691                                                                                    ;L709
 51812| 
 51813| 683: ; preds = %682
 51814|  %684 = gep %679, i64 1632                                                                                             ;L709
 51815|  %685 = load i64, ptr %684, , !!8                                                                                      ;L709
 51816|  %686 = gep %679, i64 1640                                                                                             ;L709
 51817|  %687 = load i64, ptr %686, , !!8                                                                                      ;L709
 51818|  %688 = gep %679, i64 1136                                                                                             ;L1511<709
 51819|  %689 = load i32, ptr %688, , !!8                                                                                      ;L1511<709
 51820|     ;; mult = i32 %689
 51821|  %690 = icmp eq i32 %689, 0                                                                                            ;L1512<709
 51822|  br i1 %690, label %692, label %695                                                                                    ;L1512<709
 51823| 
 51824| 691: ; preds = %715, %710, %705, %682
 51825|  br label %669                                                                                                         ;L707
 51826| 
 51827| 692: ; preds = %683
 51828|  %693 = gep %679, i64 1664                                                                                             ;L1513<709
 51829|  %694 = load i64, ptr %693, , !!8                                                                                      ;L1513<709
 51830|  br label %702                                                                                                         ;L1512<709
 51831| 
 51832| 695: ; preds = %683
 51833|  %696 = sext i32 %689 to i64                                                                                           ;L1511<709
 51834|     ;; mult = i64 %696
 51835|  %697 = gep %679, i64 1664                                                                                             ;L1515<709
 51836|  %698 = load i64, ptr %697, , !!8                                                                                      ;L1515<709
 51837|  %699 = add nsw i64 %696, 100                                                                                          ;L1515<709
 51838|  %700 = mul i64 %698, %699                                                                                             ;L1515<709
 51839|  %701 = udiv i64 %700, 100                                                                                             ;L1515<709
 51840|  br label %702                                                                                                         ;L1512<709
 51841| 
 51842| 702: ; preds = %695, %692
 51843|  %703 = phi i64 [ %694, %692 ], [ %701, %695 ]                                                                         ;L0<709
 51844|  %704 = invoke zeroext i1 @gc::simulation10projectileNtB5_10Projectile11is_in_orbit(ptr %507, i64 %685, i64 %687, i64 %703)
 51845|  to label %705 unwind label %87                                                                                        ;L709
 51846| 
 51847| 705: ; preds = %702
 51848|  br i1 %704, label %706, label %691                                                                                    ;L709
 51849| 
 51850| 706: ; preds = %705
 51851|  %707 = invoke i64 @gc::simulation10projectileNtB5_10Projectile22expected_damage_target(ptr %507, ptr %79, ptr %653, ptr %679)
 51852|  to label %708 unwind label %87                                                                                        ;L710
 51853| 
 51854| 708: ; preds = %706
 51855|     ;; dmg = i64 %707
 51856|  %709 = icmp ult i64 %670, 12                                                                                          ;L711
 51857|  br i1 %709, label %710, label %714                                                                                    ;L711
 51858| 
 51859| 710: ; preds = %708
 51860|  %711 = getelementptr i64, ptr %10, i64 %670                                                                           ;L711
 51861|  %712 = load i64, ptr %711, , !!8                                                                                      ;L711
 51862|  %713 = icmp ult i64 %712, 16                                                                                          ;L711
 51863|  br i1 %713, label %715, label %691                                                                                    ;L711
 51864| 
 51865| 714: ; preds = %708
 51866|  invoke void @core::panicking18panic_bounds_check(i64 %670, i64 12, ptr @anon.168add0ea037d45d276f5936ae758fe5.230) #30
 51867|  to label %140 unwind label %87                                                                                        ;L711
 51868| 
 51869| 715: ; preds = %710
 51870|  %716 = getelementptr [16 x { i64, i64 }], ptr %11, i64 %670                                                           ;L712
 51871|  %717 = gepS %716, i64 %712                                                                                            ;L712
 51872|  store i64 1, ptr %717,                                                                                                ;L712
 51873|  %718 = gep %717, i64 8                                                                                                ;L712
 51874|  store i64 %707, ptr %718,                                                                                             ;L712
 51875|  %719 = add nuw nsw i64 %712, 1                                                                                        ;L713
 51876|  store i64 %719, ptr %711,                                                                                             ;L713
 51877|  br label %691                                                                                                         ;L711
 51878| 
 51879| 720: ; preds = %182
 51880|     ;; info = ptr %179
 51882|     ;; self = ptr %179
 51883|     ;; other = ptr %31
 51884|  %721 = load i64, ptr %179, , !!8                                                                                      ;L1127<667
 51885|  %722 = gep %179, i64 8                                                                                                ;L1127<667
 51886|     ;; __self_discr = i64 %721
 51887|  %723 = load i64, ptr %31, , !!8                                                                                       ;L1127<667
 51888|     ;; __arg1_discr = i64 %723
 51889|  %724 = icmp eq i64 %721, %723                                                                                         ;L1127<667
 51890|  br i1 %724, label %746, label %816                                                                                    ;L1127<667
 51891| 
 51892| 725: ; preds = %182
 51893|     ;; info = ptr %179
 51895|  br i1 %174, label %816, label %759                                                                                    ;L668
 51896| 
 51897| 726: ; preds = %182
 51898|     ;; info = ptr %179
 51900|     ;; self = ptr %179
 51901|     ;; other = ptr %31
 51902|  %727 = load i64, ptr %179, , !!8                                                                                      ;L1127<669
 51903|  %728 = gep %179, i64 8                                                                                                ;L1127<669
 51904|     ;; __self_discr = i64 %727
 51905|  %729 = load i64, ptr %31, , !!8                                                                                       ;L1127<669
 51906|     ;; __arg1_discr = i64 %729
 51907|  %730 = icmp eq i64 %727, %729                                                                                         ;L1127<669
 51908|  br i1 %730, label %776, label %816                                                                                    ;L1127<669
 51909| 
 51910| 731: ; preds = %182
 51911|     ;; info = ptr %179
 51913|     ;; self = ptr %179
 51914|     ;; other = ptr %31
 51915|  %732 = load i64, ptr %179, , !!8                                                                                      ;L1127<672
 51916|  %733 = gep %179, i64 8                                                                                                ;L1127<672
 51917|     ;; __self_discr = i64 %732
 51918|  %734 = load i64, ptr %31, , !!8                                                                                       ;L1127<672
 51919|     ;; __arg1_discr = i64 %734
 51920|  %735 = icmp eq i64 %732, %734                                                                                         ;L1127<672
 51921|  br i1 %735, label %782, label %816                                                                                    ;L1127<672
 51922| 
 51923| 736: ; preds = %182
 51924|     ;; info = ptr %179
 51926|     ;; self = ptr %179
 51927|     ;; other = ptr %31
 51928|  %737 = load i64, ptr %179, , !!8                                                                                      ;L1127<670
 51929|  %738 = gep %179, i64 8                                                                                                ;L1127<670
 51930|     ;; __self_discr = i64 %737
 51931|  %739 = load i64, ptr %31, , !!8                                                                                       ;L1127<670
 51932|     ;; __arg1_discr = i64 %739
 51933|  %740 = icmp eq i64 %737, %739                                                                                         ;L1127<670
 51934|  br i1 %740, label %804, label %816                                                                                    ;L1127<670
 51935| 
 51936| 741: ; preds = %182
 51937|     ;; info = ptr %179
 51939|     ;; self = ptr %179
 51940|     ;; other = ptr %31
 51941|  %742 = load i64, ptr %179, , !!8                                                                                      ;L1127<671
 51942|  %743 = gep %179, i64 8                                                                                                ;L1127<671
 51943|     ;; __self_discr = i64 %742
 51944|  %744 = load i64, ptr %31, , !!8                                                                                       ;L1127<671
 51945|     ;; __arg1_discr = i64 %744
 51946|  %745 = icmp eq i64 %742, %744                                                                                         ;L1127<671
 51947|  br i1 %745, label %810, label %816                                                                                    ;L1127<671
 51948| 
 51949| 746: ; preds = %720
 51950|  %747 = icmp eq i64 %721, 0                                                                                            ;L1127<667
 51951|  br i1 %747, label %748, label %752                                                                                    ;L1127<667
 51952| 
 51953| 748: ; preds = %746
 51954|     ;; __self_0 = ptr %179
 51955|     ;; self = ptr %179
 51956|     ;; __arg1_0 = ptr %31
 51957|     ;; other = ptr %31
 51960|  %749 = load i64, ptr %722, , !!8                                                                                      ;L1878<2123<1127<667
 51961|  %750 = load i64, ptr %90, , !!8                                                                                       ;L1878<2123<1127<667
 51962|  %751 = icmp eq i64 %749, %750                                                                                         ;L1878<2123<1127<667
 51963|  br i1 %751, label %752, label %816                                                                                    ;L667
 51964| 
 51965| 752: ; preds = %812, %810, %806, %804, %778, %776, %748, %746
 51966|  %753 = phi i64 [ 136, %806 ], [ 136, %778 ], [ 112, %812 ], [ 136, %776 ], [ 136, %804 ], [ 112, %810 ], [ 136, %746 ], [ 136, %748 ]
 51967|  %754 = phi i64 [ 144, %806 ], [ 144, %778 ], [ 120, %812 ], [ 144, %776 ], [ 144, %804 ], [ 120, %810 ], [ 144, %746 ], [ 144, %748 ]
 51968|  %755 = gep %179, i64 %753                                                                                             ;L0
 51969|  %756 = gep %179, i64 %754                                                                                             ;L0
 51970|  %757 = load i64, ptr %755, , !!8                                                                                      ;L0
 51971|     ;; target_id[0..+8] = i64 %757
 51973|  %758 = trunc nuw i64 %757 to i1                                                                                       ;L676
 51974|  br i1 %758, label %790, label %816                                                                                    ;L676
 51975| 
 51976| 759: ; preds = %725
 51977|     ;; self = ptr %179
 51978|     ;; other = ptr %31
 51979|  %760 = load i64, ptr %179, , !!8                                                                                      ;L1127<668
 51980|  %761 = gep %179, i64 8                                                                                                ;L1127<668
 51981|     ;; __self_discr = i64 %760
 51982|  %762 = load i64, ptr %31, , !!8                                                                                       ;L1127<668
 51983|     ;; __arg1_discr = i64 %762
 51984|  %763 = icmp eq i64 %760, %762                                                                                         ;L1127<668
 51985|  br i1 %763, label %764, label %816                                                                                    ;L1127<668
 51986| 
 51987| 764: ; preds = %759
 51988|  %765 = icmp eq i64 %760, 0                                                                                            ;L1127<668
 51989|  br i1 %765, label %766, label %770                                                                                    ;L1127<668
 51990| 
 51991| 766: ; preds = %764
 51992|     ;; __self_0 = ptr %179
 51993|     ;; self = ptr %179
 51994|     ;; __arg1_0 = ptr %31
 51995|     ;; other = ptr %31
 51998|  %767 = load i64, ptr %761, , !!8                                                                                      ;L1878<2123<1127<668
 51999|  %768 = load i64, ptr %90, , !!8                                                                                       ;L1878<2123<1127<668
 52000|  %769 = icmp eq i64 %767, %768                                                                                         ;L1878<2123<1127<668
 52001|  br i1 %769, label %770, label %816                                                                                    ;L668
 52002| 
 52003| 770: ; preds = %766, %764
 52004|  %771 = gep %179, i64 136                                                                                              ;L668
 52005|  %772 = load i64, ptr %771,                                                                                            ;L668
 52006|     ;; self[0..+8] = i64 %772
 52009|  %773 = trunc nuw i64 %772 to i1                                                                                       ;L1161<668
 52010|  br i1 %773, label %774, label %816                                                                                    ;L1161<668
 52011| 
 52012| 774: ; preds = %770
 52013|  %775 = gep %179, i64 152                                                                                              ;L668
 52016|     ;; target_id[0..+8] = i64 1
 52017|  br label %790                                                                                                         ;L668
 52018| 
 52019| 776: ; preds = %726
 52020|  %777 = icmp eq i64 %727, 0                                                                                            ;L1127<669
 52021|  br i1 %777, label %778, label %752                                                                                    ;L1127<669
 52022| 
 52023| 778: ; preds = %776
 52024|     ;; __self_0 = ptr %179
 52025|     ;; self = ptr %179
 52026|     ;; __arg1_0 = ptr %31
 52027|     ;; other = ptr %31
 52030|  %779 = load i64, ptr %728, , !!8                                                                                      ;L1878<2123<1127<669
 52031|  %780 = load i64, ptr %90, , !!8                                                                                       ;L1878<2123<1127<669
 52032|  %781 = icmp eq i64 %779, %780                                                                                         ;L1878<2123<1127<669
 52033|  br i1 %781, label %752, label %816                                                                                    ;L669
 52034| 
 52035| 782: ; preds = %731
 52036|  %783 = icmp eq i64 %732, 0                                                                                            ;L1127<672
 52037|  br i1 %783, label %784, label %788                                                                                    ;L1127<672
 52038| 
 52039| 784: ; preds = %782
 52040|     ;; __self_0 = ptr %179
 52041|     ;; self = ptr %179
 52042|     ;; __arg1_0 = ptr %31
 52043|     ;; other = ptr %31
 52046|  %785 = load i64, ptr %733, , !!8                                                                                      ;L1878<2123<1127<672
 52047|  %786 = load i64, ptr %90, , !!8                                                                                       ;L1878<2123<1127<672
 52048|  %787 = icmp eq i64 %785, %786                                                                                         ;L1878<2123<1127<672
 52049|  br i1 %787, label %788, label %816                                                                                    ;L672
 52050| 
 52051| 788: ; preds = %784, %782
 52052|  %789 = gep %179, i64 256                                                                                              ;L672
 52054|     ;; target_id[0..+8] = i64 1
 52055|  br label %790                                                                                                         ;L672
 52056| 
 52057| 790: ; preds = %788, %774, %752
 52058|  %791 = phi ptr [ %756, %752 ], [ %775, %774 ], [ %789, %788 ]
 52059|  %792 = load i64, ptr %791,                                                                                            ;L0
 52060|     ;; target_id[8..+8] = i64 %792
 52061|     ;; target_id = i64 %792
 52065|     ;; id = i64 %792
 52067|     ;; target_count = ptr %13
 52068|     ;; self[0..+8] = i64 0
 52069|     ;; rhs = i64 0
 52070|     ;; self = ptr %13
 52071|     ;; index = i64 %97
 52072|     ;; index = i64 %97
 52073|     ;; self = i64 %97
 52074|     ;; self[8..+8] = i64 %97
 52075|     ;; new_len = i64 %97
 52076|     ;; self = i64 %97
 52077|     ;; self[0..+8] = ptr %13
 52078|     ;; slice[0..+8] = ptr %13
 52079|     ;; slice[0..+8] = ptr %13
 52080|     ;; self[8..+8] = i64 12
 52081|     ;; slice[8..+8] = i64 12
 52082|     ;; slice[8..+8] = i64 12
 52083|  br i1 %175, label %793, label %802                                                                                    ;L1050<437<529<19<391<655<677
 52084| 
 52085| 793: ; preds = %798, %790
 52086|  %794 = phi i64 [ %800, %798 ], [ 0, %790 ]
 52087|  %795 = phi ptr [ %799, %798 ], [ %13, %790 ]
 52088|     ;; i = i64 %794
 52089|     ;; ptr = ptr %795
 52090|     ;; x = ptr %795
 52091|  %796 = load i64, ptr %795, , !!61771, !!8                                                                             ;L384<655<677
 52094|     ;; t = i64 %796
 52095|  %797 = icmp eq i64 %796, %792                                                                                         ;L655<384<655<677
 52096|  br i1 %797, label %817, label %798                                                                                    ;L384<655<677
 52097| 
 52098| 798: ; preds = %793
 52099|  %799 = gep %795, i64 8                                                                                                ;L656<185<383<655<677
 52100|  %800 = add nuw nsw i64 %794, 1                                                                                        ;L390<655<677
 52101|     ;; i = i64 %800
 52102|     ;; ptr = ptr %799
 52103|     ;; self = ptr %799
 52104|     ;; end_or_len = ptr %177
 52107|  %801 = icmp eq ptr %799, %177                                                                                         ;L1714<180<383<655<677
 52108|  br i1 %801, label %816, label %793                                                                                    ;L180<383<655<677
 52109| 
 52110| 802: ; preds = %790
 52111|  invoke void @core::slice5index16slice_index_fail(i64 0, i64 %97, i64 12, ptr @anon.168add0ea037d45d276f5936ae758fe5.36) #30
 52112|  to label %803 unwind label %87                                                                                        ;L443<529<19<391<655<677
 52113| 
 52114| 803: ; preds = %802
 52115|  unreachable                                                                                                           ;L443<529<19<391<655<677
 52116| 
 52117| 804: ; preds = %736
 52118|  %805 = icmp eq i64 %737, 0                                                                                            ;L1127<670
 52119|  br i1 %805, label %806, label %752                                                                                    ;L1127<670
 52120| 
 52121| 806: ; preds = %804
 52122|     ;; __self_0 = ptr %179
 52123|     ;; self = ptr %179
 52124|     ;; __arg1_0 = ptr %31
 52125|     ;; other = ptr %31
 52128|  %807 = load i64, ptr %738, , !!8                                                                                      ;L1878<2123<1127<670
 52129|  %808 = load i64, ptr %90, , !!8                                                                                       ;L1878<2123<1127<670
 52130|  %809 = icmp eq i64 %807, %808                                                                                         ;L1878<2123<1127<670
 52131|  br i1 %809, label %752, label %816                                                                                    ;L670
 52132| 
 52133| 810: ; preds = %741
 52134|  %811 = icmp eq i64 %742, 0                                                                                            ;L1127<671
 52135|  br i1 %811, label %812, label %752                                                                                    ;L1127<671
 52136| 
 52137| 812: ; preds = %810
 52138|     ;; __self_0 = ptr %179
 52139|     ;; self = ptr %179
 52140|     ;; __arg1_0 = ptr %31
 52141|     ;; other = ptr %31
 52144|  %813 = load i64, ptr %743, , !!8                                                                                      ;L1878<2123<1127<671
 52145|  %814 = load i64, ptr %90, , !!8                                                                                       ;L1878<2123<1127<671
 52146|  %815 = icmp eq i64 %813, %814                                                                                         ;L1878<2123<1127<671
 52147|  br i1 %815, label %752, label %816                                                                                    ;L671
 52148| 
 52149| 816: ; preds = %838, %817, %812, %806, %798, %784, %778, %770, %766, %759, %752, %748, %741, %736, %731, %726, %725, %720, %182
 52150|  br label %178                                                                                                         ;L665
 52151| 
 52152| 817: ; preds = %793
 52153|  %818 = icmp ult i64 %794, %97                                                                                         ;L387<655<677
 52154|     ;; cond = i1 true
 52155|  call void @llvm.assume(i1 %818)                                                                                       ;L210<387<655<677
 52156|     ;; slot = i64 %794
 52157|     ;; index = i64 %794
 52158|     ;; index = i64 %794
 52159|     ;; self = i64 %794
 52160|     ;; self = ptr %179
 52161|  %819 = gep %179, i64 1168                                                                                             ;L742<678
 52162|  %820 = gep %179, i64 1216                                                                                             ;L742<678
 52163|  %821 = load i32, ptr %820, , !!8                                                                                      ;L742<678
 52164|  %822 = icmp eq i32 %821, -1                                                                                           ;L742<678
 52165|  br i1 %822, label %816, label %823                                                                                    ;L742<678
 52166| 
 52167| 823: ; preds = %817
 52168|     ;; atk_eff = ptr %819
 52169|     ;; self = ptr %17
 52170|     ;; self = ptr %17
 52171|     ;; self = ptr %17
 52173|  %824 = load i64, ptr %83, , !!8                                                                                       ;L2075<2054<679
 52176|     ;; self[8..+8] = i64 %824
 52177|     ;; slice[8..+8] = i64 %824
 52178|  %825 = icmp ult i64 %794, %824                                                                                        ;L272<19<2054<679
 52179|  br i1 %825, label %826, label %831                                                                                    ;L272<19<2054<679
 52180| 
 52181| 826: ; preds = %823
 52182|  %827 = load ptr, ptr %17, , !!8, !!8                                                                                  ;L138<2073<2054<679
 52183|     ;; p = ptr %827
 52184|     ;; self[0..+8] = ptr %827
 52185|     ;; slice[0..+8] = ptr %827
 52186|  %828 = getelementptr ptr, ptr %827, i64 %794                                                                          ;L272<19<2054<679
 52187|  %829 = load ptr, ptr %828, , !!8, !!8                                                                                 ;L679
 52188|  %830 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %819, ptr %79, ptr %179, ptr @anon.168add0ea037d45d276f5936ae758fe5.6, ptr %829)
 52189|  to label %832 unwind label %87                                                                                        ;L679
 52190| 
 52191| 831: ; preds = %823
 52192|  invoke void @core::panicking18panic_bounds_check(i64 %794, i64 %824, ptr @anon.168add0ea037d45d276f5936ae758fe5.231) #30
 52193|  to label %140 unwind label %87                                                                                        ;L272<19<2054<679
 52194| 
 52195| 832: ; preds = %826
 52196|     ;; dmg = i64 %830
 52197|  %833 = invoke i64 @gc::simulation6entityNtB5_6Entity15attack_cooltime(ptr %179)
 52198|  to label %834 unwind label %87                                                                                        ;L680
 52199| 
 52200| 834: ; preds = %832
 52201|     ;; self = i64 %833
 52202|     ;; other = i64 1
 52204|  %835 = icmp eq i64 %833, -1                                                                                           ;L681
 52205|  %836 = icmp eq i64 %830, -9223372036854775808                                                                         ;L681
 52206|  %837 = and i1 %836, %835                                                                                              ;L681
 52207|  br i1 %837, label %844, label %838                                                                                    ;L681
 52208| 
 52209| 838: ; preds = %834
 52210|  %839 = call i64 @llvm.umax.i64(i64 %833, i64 1)                                                                       ;L1039<680
 52211|     ;; cooltime = i64 %839
 52212|  %840 = sdiv i64 %830, %839                                                                                            ;L681
 52213|  %841 = getelementptr i64, ptr %12, i64 %794                                                                           ;L681
 52214|  %842 = load i64, ptr %841, , !!8                                                                                      ;L681
 52215|  %843 = add i64 %842, %840                                                                                             ;L681
 52216|  store i64 %843, ptr %841,                                                                                             ;L681
 52217|  br label %816                                                                                                         ;L678
 52218| 
 52219| 844: ; preds = %834
 52220|  invoke void @core::panicking11panic_const24panic_const_div_overflow(ptr @anon.168add0ea037d45d276f5936ae758fe5.232) #30
 52221|  to label %140 unwind label %87                                                                                        ;L681
 52222| }
