 99743| define hidden void @ai::small_action12move_actionsNtB5_17SmallActionRecall9get_input(ptr sret([32 x i8]) %0, ptr %1, i64 %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr readnone %7) unnamed_addr #2 personality ptr @__CxxFrameHandler3 {
 99744|  %9 = alloca [0 x i8],
 99745|  %10 = alloca [8 x i8],
 99746|  %11 = alloca [32 x i8],
 99747|  %12 = alloca [8 x i8],
 99748|  %13 = alloca [32 x i8],
 99749|  %14 = alloca [104 x i8],
 99750|  %15 = alloca [104 x i8],
 99751|  %16 = alloca [72 x i8],
 99752|  %17 = alloca [392 x i8],
 99753|  %18 = alloca [168 x i8],
 99754|  %19 = alloca [88 x i8],
 99755|  %20 = alloca [8 x i8],
 99756|  %21 = alloca [80 x i8],
 99757|  %22 = alloca [496 x i8],
 99758|  %23 = alloca [416 x i8],
 99759|  %24 = alloca [56 x i8],
 99760|  %25 = alloca [56 x i8],
 99761|  %26 = alloca [32 x i8],
 99762|  %27 = alloca [32 x i8],
 99763|  %28 = alloca [32 x i8],
 99764|  %29 = alloca [56 x i8],
 99765|  %30 = alloca [24 x i8],
 99766|  %31 = alloca [56 x i8],
 99773|  %32 = alloca [32 x i8],
 99774|  %33 = alloca [16 x i8],
 99775|  %34 = alloca [8 x i8],
 99776|  %35 = alloca [32 x i8],
 99781|  %36 = alloca [56 x i8],
 99785|  %37 = alloca [88 x i8],
 99786|  %38 = alloca [80 x i8],
 99787|  %39 = alloca [1 x i8],
 99788|  %40 = alloca [8 x i8],
 99789|  store i64 %2, ptr %40,
 99790|     ;; self = ptr %1
 99791|     ;; version = ptr %40
 99792|     ;; rnd = ptr %3
 99793|     ;; player = ptr %4
 99794|     ;; data = ptr %5
 99795|     ;; positioning_score = ptr %6
 99796|     ;; _debug = ptr %7
 99797|     ;; direct_heal = ptr %39
 99798|     ;; enemy_positions = ptr %38
 99799|     ;; s = ptr %36
 99800|     ;; candidates = ptr %35
 99801|     ;; b1_free = ptr %34
 99802|     ;; iter = ptr %33
 99803|     ;; flee_ok = ptr %32
 99804|     ;; cell_score = ptr %31
 99805|     ;; nxt = ptr %30
 99806|     ;; next_score = ptr %29
 99807|     ;; pre_score_data = ptr %25
 99808|     ;; s = ptr %24
 99809|     ;; tower_dodge = ptr %23
 99810|     ;; pve_hazard_recall = ptr %22
 99811|     ;; enemy_positions = ptr %21
 99812|     ;; visible_enemy_count = ptr %20
 99813|     ;; enemy_hazard = ptr %18
 99814|     ;; zone_hazard = ptr %17
 99816|     ;; self[0..+8] = i64 0
 99817|     ;; rhs = i64 0
 99818|     ;; self[0..+8] = i64 0
 99819|     ;; rhs = i64 0
 99820|     ;; len = i64 5
 99821|     ;; count = i64 5
 99822|     ;; self[0..+8] = i64 0
 99823|     ;; rhs = i64 0
 99824|     ;; n = i64 1
 99825|     ;; rhs = i64 1
 99826|     ;; rhs = i64 1
 99827|     ;; n = i64 1
 99828|     ;; rhs = i64 1
 99829|     ;; rhs = i64 1
 99830|  %41 = gep %4, i64 2352                                                                                                ;L682
 99831|  %42 = load i64, ptr %41, , !!8                                                                                        ;L682
 99832|     ;; team = i64 %42
 99833|  %43 = icmp eq i64 %42, 0                                                                                              ;L682
 99834|  %44 = select i1 %43, i64 32000, i64 928000                                                                            ;L682
 99835|  %45 = select i1 %43, i64 928000, i64 32000                                                                            ;L682
 99836|     ;; healp[8..+8] = i64 %45
 99837|     ;; healp[0..+8] = i64 %44
 99838|  %46 = icmp ult i64 %42, 2                                                                                             ;L688
 99839|  br i1 %46, label %48, label %47                                                                                       ;L688
 99840| 
 99841| 47: ; preds = %8
 99842|  tail call void @core::panicking18panic_bounds_check(i64 %42, i64 2, ptr @anon.afb017e85830050c32e5158c55060899.75) #25 ;L688
 99843|  unreachable                                                                                                           ;L688
 99844| 
 99845| 48: ; preds = %8
 99846|     ;; self = ptr %4
 99847|  %49 = gep %4, i64 2496                                                                                                ;L581<688
 99848|  %50 = load i32, ptr %49, , !!8                                                                                        ;L581<688
 99849|  %51 = zext nneg i32 %50 to i64                                                                                        ;L581<688
 99850|  %52 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L688
 99851|     ;; self = ptr %52
 99852|  %53 = gep %52, i64 480                                                                                                ;L688
 99853|  %54 = getelementptr [5 x ptr], ptr %53, i64 %42                                                                       ;L688
 99854|  %55 = getelementptr ptr, ptr %54, i64 %51                                                                             ;L688
 99855|  %56 = load ptr, ptr %55, , !!8                                                                                        ;L688
 99856|     ;; self = ptr %56
 99857|  %57 = icmp eq ptr %56, null                                                                                           ;L1011<688
 99858|  br i1 %57, label %73, label %58                                                                                       ;L1011<688
 99859| 
 99860| 58: ; preds = %48
 99861|     ;; champ = ptr %56
 99862|  %59 = gep %5, i64 8                                                                                                   ;L689
 99863|  %60 = load ptr, ptr %59, , !!8, !!8                                                                                   ;L689
 99864|  %61 = gep %60, i64 32                                                                                                 ;L689
 99865|  %62 = load ptr, ptr %61, , !!8, !!8                                                                                   ;L689
 99866|     ;; self = ptr %62
 99867|  %63 = gep %62, i64 28016                                                                                              ;L235<689
 99868|  %64 = gepS %63, i64 %42                                                                                               ;L235<689
 99869|  %65 = load i64, ptr %64, , !!8                                                                                        ;L235<689
 99870|     ;; lx = i64 %65
 99872|  %66 = gep %64, i64 16                                                                                                 ;L235<689
 99873|  %67 = load i64, ptr %66, , !!8                                                                                        ;L235<689
 99874|     ;; rx = i64 %67
 99876|  %68 = gep %56, i64 1632                                                                                               ;L691
 99877|  %69 = load i64, ptr %68, , !!8                                                                                        ;L691
 99878|     ;; x1 = i64 %69
 99879|     ;; self = i64 %69
 99880|     ;; x1 = i64 %69
 99881|     ;; self = i64 %69
 99882|     ;; x1 = i64 %69
 99883|     ;; self = i64 %69
 99884|  %70 = icmp uge i64 %69, %65                                                                                           ;L691
 99885|  %71 = icmp ule i64 %69, %67                                                                                           ;L691
 99886|  %72 = and i1 %70, %71                                                                                                 ;L691
 99887|  br i1 %72, label %77, label %74                                                                                       ;L691
 99888| 
 99889| 73: ; preds = %48
 99890|  tail call void @core::option13unwrap_failed(ptr @anon.afb017e85830050c32e5158c55060899.76) #25                        ;L1013<688
 99891|  unreachable                                                                                                           ;L1013<688
 99892| 
 99893| 74: ; preds = %77, %58
 99895|  %75 = icmp ult i64 %2, 2                                                                                              ;L700
 99896|     ;; self = i1 %75
 99897|  br i1 %75, label %76, label %88                                                                                       ;L700
 99898| 
 99899| 76: ; preds = %74
 99900|  store i8 0, ptr %39,                                                                                                  ;L700
 99901|  br label %94                                                                                                          ;L701
 99902| 
 99903| 77: ; preds = %58
 99904|  %78 = gep %64, i64 24                                                                                                 ;L235<689
 99905|  %79 = load i64, ptr %78, , !!8                                                                                        ;L235<689
 99906|     ;; ry = i64 %79
 99907|  %80 = gep %64, i64 8                                                                                                  ;L235<689
 99908|  %81 = load i64, ptr %80, , !!8                                                                                        ;L235<689
 99909|     ;; ly = i64 %81
 99910|  %82 = gep %56, i64 1640                                                                                               ;L691
 99911|  %83 = load i64, ptr %82, , !!8                                                                                        ;L691
 99912|  %84 = icmp uge i64 %83, %81                                                                                           ;L691
 99913|  %85 = icmp ule i64 %83, %79                                                                                           ;L691
 99914|  %86 = and i1 %84, %85                                                                                                 ;L691
 99915|  br i1 %86, label %87, label %74                                                                                       ;L691
 99916| 
 99917| 87: ; preds = %77
 99918|  store i64 -1, ptr %0,                                                                                                 ;L692
 99919|  br label %1018                                                                                                        ;L1
 99920| 
 99921| 88: ; preds = %74
 99922|  %89 = gep %56, i64 1640                                                                                               ;L700
 99923|  %90 = load i64, ptr %89, , !!8                                                                                        ;L700
 99924|  %91 = tail call i64 @gc::utils8distance(i64 %69, i64 %90, i64 %44, i64 %45)                                           ;L700
 99925|  %92 = icmp ult i64 %91, 200001                                                                                        ;L700
 99926|  %93 = zext i1 %92 to i8                                                                                               ;L700
 99927|  store i8 %93, ptr %39,                                                                                                ;L700
 99928|  br i1 %92, label %98, label %94                                                                                       ;L701
 99929| 
 99930| 94: ; preds = %98, %88, %76
 99931|  %95 = phi i1 [ false, %76 ], [ true, %98 ], [ false, %88 ]
 99933|  call void @ai::path_finder31collect_visible_enemy_positions(ptr sret([88 x i8]) %37, ptr %4, ptr %5)                  ;L707
 99935|  call void @llvm.memcpy.p0.p0.i64(ptr %38, ptr %37, i64 80, i1 false)                                                  ;L707
 99936|  %96 = gep %37, i64 80                                                                                                 ;L707
 99937|  %97 = load i64, ptr %96, , !!8                                                                                        ;L707
 99938|     ;; visible_enemy_count = i64 %97
 99939|     ;; index = i64 %97
 99940|     ;; index = i64 %97
 99941|     ;; self = i64 %97
 99942|     ;; self[8..+8] = i64 %97
 99943|     ;; new_len = i64 %97
 99944|     ;; self = i64 %97
 99945|     ;; len = i64 %97
 99946|     ;; count = i64 %97
 99947|     ;; index = i64 %97
 99948|     ;; index = i64 %97
 99949|     ;; self = i64 %97
 99950|     ;; self[8..+8] = i64 %97
 99951|     ;; new_len = i64 %97
 99952|     ;; self = i64 %97
 99953|     ;; count = i64 %97
 99954|     ;; index = i64 %97
 99955|     ;; index = i64 %97
 99956|     ;; self = i64 %97
 99957|     ;; self[8..+8] = i64 %97
 99958|     ;; new_len = i64 %97
 99959|     ;; self = i64 %97
 99960|     ;; count = i64 %97
 99962|  br i1 %75, label %104, label %102                                                                                     ;L710
 99963| 
 99964| 98: ; preds = %88
 99965|  %99 = gep %1, i64 80                                                                                                  ;L702
 99966|  store i64 %44, ptr %99,                                                                                               ;L702
 99967|  %100 = gep %1, i64 88                                                                                                 ;L703
 99968|  store i64 %45, ptr %100,                                                                                              ;L703
 99969|  %101 = gep %1, i64 128                                                                                                ;L704
 99970|  store i8 1, ptr %101,                                                                                                 ;L704
 99971|  br label %94                                                                                                          ;L701
 99972| 
 99973| 102: ; preds = %94
 99974|  %103 = tail call zeroext i1 @ai::path_finder23enemy_knows_my_position(ptr %4, ptr %5)                                 ;L710
 99975|  br label %104                                                                                                         ;L710
 99976| 
 99977| 104: ; preds = %102, %94
 99978|  %105 = phi i1 [ %103, %102 ], [ false, %94 ]                                                                          ;L710
 99980|     ;; eff_risk = ptr undef
 99981|  %106 = gep %56, i64 1640                                                                                              ;L714
 99982|  %107 = load i64, ptr %106, , !!8                                                                                      ;L714
 99983|     ;; y1 = i64 %107
 99984|     ;; self = i64 %107
 99985|     ;; y1 = i64 %107
 99986|     ;; self = i64 %107
 99987|     ;; y1 = i64 %107
 99988|     ;; self = i64 %107
 99989|  %108 = tail call i64 @gc::utils8distance(i64 %69, i64 %107, i64 %44, i64 %45)                                         ;L714
 99990|     ;; dist_to_heal_area = i64 %108
 99991|  %109 = gep %56, i64 1600                                                                                              ;L716
 99992|  %110 = load i64, ptr %109, , !!8                                                                                      ;L716
 99993|  %111 = icmp eq i64 %110, 0                                                                                            ;L716
 99994|  br i1 %111, label %114, label %112                                                                                    ;L716
 99995| 
 99996| 112: ; preds = %104
 99998|  %113 = tail call zeroext i1 @ai::small_action4cast14is_safe_recall(i64 %2, ptr %3, ptr %4, ptr %5, ptr %6)            ;L719
 99999|  br i1 %113, label %116, label %115                                                                                    ;L719
100000| 
100001| 114: ; preds = %104
100002|  tail call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.afb017e85830050c32e5158c55060899.77) #25 ;L716
100003|  unreachable                                                                                                           ;L716
100004| 
100005| 115: ; preds = %116, %112
100006|  br i1 %95, label %136, label %126                                                                                     ;L721
100007| 
100008| 116: ; preds = %112
100009|  %117 = udiv i64 %108, %110                                                                                            ;L716
100010|     ;; time = i64 %117
100011|  %118 = gep %60, i64 8                                                                                                 ;L719
100012|  %119 = load ptr, ptr %118, , !!8, !!8                                                                                 ;L719
100013|  %120 = gep %119, i64 4856                                                                                             ;L719
100014|  %121 = load i64, ptr %120, , !!8                                                                                      ;L719
100015|  %122 = mul i64 %121, 3                                                                                                ;L719
100016|  %123 = add i64 %122, 60                                                                                               ;L719
100017|  %124 = icmp ult i64 %117, %123                                                                                        ;L719
100018|  br i1 %124, label %115, label %125                                                                                    ;L719
100019| 
100020| 125: ; preds = %116
100021|  store i64 1, ptr %0,                                                                                                  ;L720
100022|  br label %1017                                                                                                        ;L1
100023| 
100024| 126: ; preds = %115
100025|  %127 = load ptr, ptr %52, , !!8, !!8                                                                                  ;L723
100026|  %128 = gep %52, i64 8                                                                                                 ;L723
100027|  %129 = load ptr, ptr %128, , !!8, !!8                                                                                 ;L723
100028|  %130 = gep %129, i64 40                                                                                               ;L723
100029|  %131 = load ptr, ptr %130, , !!8                                                                                      ;L723
100030|  %132 = tail call i64 %131(ptr %127)                                                                                   ;L723
100031|     ;; now_tick = i64 %132
100032|     ;; self = i64 %132
100033|     ;; self = ptr %1
100034|  %133 = gep %1, i64 69                                                                                                 ;L742<725
100035|  %134 = load i8, ptr %133, , !!8                                                                                       ;L742<725
100036|  %135 = icmp eq i8 %134, 2                                                                                             ;L742<725
100037|  br i1 %135, label %152, label %148                                                                                    ;L742<725
100038| 
100039| 136: ; preds = %782, %306, %115
100040|  %137 = phi i64 [ %2, %306 ], [ %783, %782 ], [ %2, %115 ]                                                             ;L954
100042|  %138 = gep %1, i64 80                                                                                                 ;L943
100043|  %139 = load i64, ptr %138, , !!8                                                                                      ;L943
100044|  %140 = gep %1, i64 88                                                                                                 ;L943
100045|  %141 = load i64, ptr %140, , !!8                                                                                      ;L943
100046|  call void @ai::path_finderNtB5_17TowerDodgeContext18new_tower_avoid_v3(ptr sret([416 x i8]) %23, i64 %137, ptr %4, ptr %5, i64 %139, i64 %141) ;L943
100048|  call void @ai::path_finderNtB4_16PveHazardContext3new(ptr sret([496 x i8]) %22, i64 %137, ptr %4, ptr %5)             ;L944
100049|  %142 = call i32 @ai::tower_discipline20v3_deadly_edge_cells(i64 %137, ptr %4, ptr %5)                                 ;L947
100050|     ;; policy[0..+4] = i32 %142
100051|     ;; policy[4..+1] = i64 %137
100053|  call void @ai::path_finder29collect_known_enemy_positions(ptr sret([88 x i8]) %19, i64 %137, ptr %4, ptr %5)          ;L951
100055|  call void @llvm.memcpy.p0.p0.i64(ptr %21, ptr %19, i64 80, i1 false)                                                  ;L951
100057|  %143 = gep %19, i64 80                                                                                                ;L951
100058|  %144 = load i64, ptr %143, , !!8                                                                                      ;L951
100059|  store i64 %144, ptr %20,                                                                                              ;L951
100062|  call void @ai::path_finderNtB5_18EnemyHazardContext3new(ptr sret([168 x i8]) %18, i64 %137, ptr %4, ptr %5)           ;L953
100064|  call void @ai::path_finderNtB5_17ZoneHazardContext3new(ptr sret([392 x i8]) %17, i64 %137, ptr %4, ptr %5)            ;L954
100065|     ;; self = ptr %1
100066|     ;; self = ptr %1
100067|  %145 = gep %1, i64 69                                                                                                 ;L633<682<955
100068|  %146 = load i8, ptr %145, , !!8                                                                                       ;L633<682<955
100069|  %147 = icmp eq i8 %146, 2                                                                                             ;L633<682<955
100070|  br i1 %147, label %979, label %1001                                                                                   ;L955
100071| 
100072| 148: ; preds = %126
100073|     ;; pf = ptr undef
100074|  %149 = gep %1, i64 16                                                                                                 ;L726
100075|  %150 = load i64, ptr %149, , !!8                                                                                      ;L726
100076|  %151 = icmp eq i64 %150, 0                                                                                            ;L726
100077|  br i1 %151, label %152, label %202                                                                                    ;L726
100078| 
100079| 152: ; preds = %148, %126
100080|  %153 = gep %1, i64 80                                                                                                 ;L731
100081|  %154 = load i64, ptr %153, , !!8                                                                                      ;L731
100082|     ;; x2 = i64 %154
100083|     ;; other = i64 %154
100084|  %155 = gep %1, i64 88                                                                                                 ;L731
100085|  %156 = load i64, ptr %155, , !!8                                                                                      ;L731
100086|     ;; y2 = i64 %156
100087|     ;; other = i64 %156
100088|  %157 = icmp ult i64 %69, %154                                                                                         ;L3147<7<731
100089|  %158 = sub nuw i64 %154, %69                                                                                          ;L3147<7<731
100090|  %159 = sub nuw i64 %69, %154                                                                                          ;L3147<7<731
100091|  %160 = select i1 %157, i64 %158, i64 %159                                                                             ;L3147<7<731
100092|     ;; dx = i64 %160
100093|  %161 = icmp ult i64 %107, %156                                                                                        ;L3147<8<731
100094|  %162 = sub nuw i64 %156, %107                                                                                         ;L3147<8<731
100095|  %163 = sub nuw i64 %107, %156                                                                                         ;L3147<8<731
100096|  %164 = select i1 %161, i64 %162, i64 %163                                                                             ;L3147<8<731
100097|     ;; dy = i64 %164
100098|  %165 = mul i64 %160, %160                                                                                             ;L9<731
100099|  %166 = mul i64 %164, %164                                                                                             ;L9<731
100100|  %167 = add i64 %166, %165                                                                                             ;L9<731
100101|     ;; n = i64 %167
100102|  %168 = icmp eq i64 %167, 0                                                                                            ;L66<731
100103|  br i1 %168, label %208, label %169                                                                                    ;L66<731
100104| 
100105| 169: ; preds = %152
100107|  %170 = icmp ugt i64 %167, 65535                                                                                       ;L74<731
100108|  %171 = tail call i64 @llvm.ctlz.i64(i64 %167, i1 true)                                                                ;L74<731
100109|  %172 = trunc nuw nsw i64 %171 to i32                                                                                  ;L74<731
100110|  %173 = sub nsw i32 49, %172                                                                                           ;L74<731
100111|  %174 = and i32 %173, -2                                                                                               ;L74<731
100112|  %175 = select i1 %170, i32 %174, i32 0                                                                                ;L74<731
100113|     ;; s = i32 %175
100114|  %176 = zext nneg i32 %175 to i64                                                                                      ;L75<731
100115|  %177 = lshr i64 %167, %176                                                                                            ;L75<731
100116|     ;; m = i64 %177
100117|  %178 = lshr i64 %177, 8                                                                                               ;L76<731
100118|  %179 = icmp ult i64 %177, 65536                                                                                       ;L76<731
100119|  br i1 %179, label %181, label %180                                                                                    ;L76<731
100120| 
100121| 180: ; preds = %169
100122|  tail call void @core::panicking18panic_bounds_check(i64 %178, i64 256, ptr @anon.afb017e85830050c32e5158c55060899.124) #25 ;L76<731
100123|  unreachable                                                                                                           ;L76<731
100124| 
100125| 181: ; preds = %169
100127|  %182 = getelementptr i16, ptr @anon.afb017e85830050c32e5158c55060899.122, i64 %178                                    ;L76<731
100128|  %183 = load i16, ptr %182, , !!8                                                                                      ;L76<731
100129|     ;; x = !DIArgList(i16 %183, i32 %175)
100130|  %184 = zext i16 %183 to i64                                                                                           ;L76<731
100131|     ;; x = !DIArgList(i64 %184, i32 %175)
100132|  %185 = lshr exact i32 %175, 1                                                                                         ;L76<731
100133|     ;; x = !DIArgList(i64 %184, i32 %185)
100134|  %186 = zext nneg i32 %185 to i64                                                                                      ;L76<731
100135|     ;; x = !DIArgList(i64 %184, i64 %186)
100136|  %187 = shl nuw nsw i64 %184, %186                                                                                     ;L76<731
100137|     ;; x = i64 %187
100138|  %188 = add nuw nsw i64 %187, 1                                                                                        ;L76<731
100139|     ;; x = i64 %188
100140|  %189 = udiv i64 %167, %188                                                                                            ;L77<731
100141|  %190 = add i64 %188, %189                                                                                             ;L77<731
100142|     ;; y = i64 %190
100143|  %191 = lshr i64 %190, 1                                                                                               ;L0<731
100144|  %192 = icmp samesign ugt i64 %191, %187                                                                               ;L78<731
100145|  br i1 %192, label %208, label %193                                                                                    ;L78<731
100146| 
100147| 193: ; preds = %196, %181
100148|  %194 = phi i64 [ %199, %196 ], [ %191, %181 ]
100149|     ;; x = i64 %194
100150|  %195 = icmp eq i64 %194, 0                                                                                            ;L80<731
100151|  br i1 %195, label %201, label %196                                                                                    ;L80<731
100152| 
100153| 196: ; preds = %193
100154|  %197 = udiv i64 %167, %194                                                                                            ;L80<731
100155|  %198 = add i64 %197, %194                                                                                             ;L80<731
100156|     ;; y = i64 %198
100157|  %199 = lshr i64 %198, 1                                                                                               ;L0<731
100158|     ;; x = i64 %194
100159|     ;; y = i64 %199
100160|  %200 = icmp samesign ult i64 %199, %194                                                                               ;L78<731
100161|  br i1 %200, label %193, label %208                                                                                    ;L78<731
100162| 
100163| 201: ; preds = %193
100164|  tail call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.afb017e85830050c32e5158c55060899.125) #25 ;L80<731
100165|  unreachable                                                                                                           ;L80<731
100166| 
100167| 202: ; preds = %148
100168|     ;; pf = ptr %1
100169|  %203 = gep %1, i64 24                                                                                                 ;L727
100170|  %204 = load i64, ptr %203, , !!8                                                                                      ;L727
100171|  %205 = add i64 %150, -1                                                                                               ;L727
100172|     ;; self = i64 %204
100173|     ;; other = i64 %205
100174|  %206 = tail call i64 @llvm.umin.i64(i64 %205, i64 %204)                                                               ;L1078<727
100175|     ;; idx = i64 %206
100176|  %207 = icmp ult i64 %206, 70                                                                                          ;L728
100177|  br i1 %207, label %213, label %270                                                                                    ;L728
100178| 
100179| 208: ; preds = %265, %196, %181, %152
100180|  %209 = phi i64 [ %269, %265 ], [ 0, %152 ], [ %188, %181 ], [ %194, %196 ]                                            ;L0
100181|     ;; prog_now = i64 %209
100182|  %210 = gep %1, i64 112                                                                                                ;L733
100183|  %211 = load i64, ptr %210, , !!8                                                                                      ;L733
100184|  %212 = icmp ult i64 %209, %211                                                                                        ;L733
100185|  br i1 %212, label %271, label %273                                                                                    ;L733
100186| 
100187| 213: ; preds = %202
100188|  %214 = gep %1, i64 40                                                                                                 ;L728
100189|  %215 = load ptr, ptr %214, , !!8, !!8                                                                                 ;L728
100190|  %216 = gepS %215, i64 %206                                                                                            ;L728
100191|  %217 = load i64, ptr %216, , !!8                                                                                      ;L728
100192|     ;; wx = i64 %217
100193|     ;; x2 = i64 %217
100194|     ;; other = i64 %217
100195|  %218 = gep %216, i64 8                                                                                                ;L728
100196|  %219 = load i64, ptr %218, , !!8                                                                                      ;L728
100197|     ;; wy = i64 %219
100198|     ;; y2 = i64 %219
100199|     ;; other = i64 %219
100200|  %220 = icmp ult i64 %69, %217                                                                                         ;L3147<7<729
100201|  %221 = sub nuw i64 %217, %69                                                                                          ;L3147<7<729
100202|  %222 = sub nuw i64 %69, %217                                                                                          ;L3147<7<729
100203|  %223 = select i1 %220, i64 %221, i64 %222                                                                             ;L3147<7<729
100204|     ;; dx = i64 %223
100205|  %224 = icmp ult i64 %107, %219                                                                                        ;L3147<8<729
100206|  %225 = sub nuw i64 %219, %107                                                                                         ;L3147<8<729
100207|  %226 = sub nuw i64 %107, %219                                                                                         ;L3147<8<729
100208|  %227 = select i1 %224, i64 %225, i64 %226                                                                             ;L3147<8<729
100209|     ;; dy = i64 %227
100210|  %228 = mul i64 %223, %223                                                                                             ;L9<729
100211|  %229 = mul i64 %227, %227                                                                                             ;L9<729
100212|  %230 = add i64 %229, %228                                                                                             ;L9<729
100213|     ;; n = i64 %230
100214|  %231 = icmp eq i64 %230, 0                                                                                            ;L66<729
100215|  br i1 %231, label %265, label %232                                                                                    ;L66<729
100216| 
100217| 232: ; preds = %213
100219|  %233 = icmp ugt i64 %230, 65535                                                                                       ;L74<729
100220|  %234 = tail call i64 @llvm.ctlz.i64(i64 %230, i1 true)                                                                ;L74<729
100221|  %235 = trunc nuw nsw i64 %234 to i32                                                                                  ;L74<729
100222|  %236 = sub nsw i32 49, %235                                                                                           ;L74<729
100223|  %237 = and i32 %236, -2                                                                                               ;L74<729
100224|  %238 = select i1 %233, i32 %237, i32 0                                                                                ;L74<729
100225|     ;; s = i32 %238
100226|  %239 = zext nneg i32 %238 to i64                                                                                      ;L75<729
100227|  %240 = lshr i64 %230, %239                                                                                            ;L75<729
100228|     ;; m = i64 %240
100229|  %241 = lshr i64 %240, 8                                                                                               ;L76<729
100230|  %242 = icmp ult i64 %240, 65536                                                                                       ;L76<729
100231|  br i1 %242, label %244, label %243                                                                                    ;L76<729
100232| 
100233| 243: ; preds = %232
100234|  tail call void @core::panicking18panic_bounds_check(i64 %241, i64 256, ptr @anon.afb017e85830050c32e5158c55060899.124) #25 ;L76<729
100235|  unreachable                                                                                                           ;L76<729
100236| 
100237| 244: ; preds = %232
100239|  %245 = getelementptr i16, ptr @anon.afb017e85830050c32e5158c55060899.122, i64 %241                                    ;L76<729
100240|  %246 = load i16, ptr %245, , !!8                                                                                      ;L76<729
100241|     ;; x = !DIArgList(i16 %246, i32 %238)
100242|  %247 = zext i16 %246 to i64                                                                                           ;L76<729
100243|     ;; x = !DIArgList(i64 %247, i32 %238)
100244|  %248 = lshr exact i32 %238, 1                                                                                         ;L76<729
100245|     ;; x = !DIArgList(i64 %247, i32 %248)
100246|  %249 = zext nneg i32 %248 to i64                                                                                      ;L76<729
100247|     ;; x = !DIArgList(i64 %247, i64 %249)
100248|  %250 = shl nuw nsw i64 %247, %249                                                                                     ;L76<729
100249|     ;; x = i64 %250
100250|  %251 = add nuw nsw i64 %250, 1                                                                                        ;L76<729
100251|     ;; x = i64 %251
100252|  %252 = udiv i64 %230, %251                                                                                            ;L77<729
100253|  %253 = add i64 %251, %252                                                                                             ;L77<729
100254|     ;; y = i64 %253
100255|  %254 = lshr i64 %253, 1                                                                                               ;L0<729
100256|  %255 = icmp samesign ugt i64 %254, %250                                                                               ;L78<729
100257|  br i1 %255, label %265, label %256                                                                                    ;L78<729
100258| 
100259| 256: ; preds = %259, %244
100260|  %257 = phi i64 [ %262, %259 ], [ %254, %244 ]
100261|     ;; x = i64 %257
100262|  %258 = icmp eq i64 %257, 0                                                                                            ;L80<729
100263|  br i1 %258, label %264, label %259                                                                                    ;L80<729
100264| 
100265| 259: ; preds = %256
100266|  %260 = udiv i64 %230, %257                                                                                            ;L80<729
100267|  %261 = add i64 %260, %257                                                                                             ;L80<729
100268|     ;; y = i64 %261
100269|  %262 = lshr i64 %261, 1                                                                                               ;L0<729
100270|     ;; x = i64 %257
100271|     ;; y = i64 %262
100272|  %263 = icmp samesign ult i64 %262, %257                                                                               ;L78<729
100273|  br i1 %263, label %256, label %265                                                                                    ;L78<729
100274| 
100275| 264: ; preds = %256
100276|  tail call void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.afb017e85830050c32e5158c55060899.125) #25 ;L80<729
100277|  unreachable                                                                                                           ;L80<729
100278| 
100279| 265: ; preds = %259, %244, %213
100280|  %266 = phi i64 [ 0, %213 ], [ %251, %244 ], [ %257, %259 ]                                                            ;L0<729
100281|     ;; x = i64 %266
100282|  %267 = sub i64 %205, %206                                                                                             ;L729
100283|  %268 = mul i64 %267, 32000                                                                                            ;L729
100284|  %269 = add i64 %266, %268                                                                                             ;L729
100285|     ;; prog_now = i64 %269
100286|  br label %208                                                                                                         ;L730
100287| 
100288| 270: ; preds = %202
100289|  tail call void @core::panicking18panic_bounds_check(i64 %206, i64 70, ptr @anon.afb017e85830050c32e5158c55060899.78) #25 ;L728
100290|  unreachable                                                                                                           ;L728
100291| 
100292| 271: ; preds = %208
100293|  store i64 %209, ptr %210,                                                                                             ;L734
100294|  %272 = gep %1, i64 120                                                                                                ;L735
100295|  store i64 %132, ptr %272,                                                                                             ;L735
100296|  br label %273                                                                                                         ;L733
100297| 
100298| 273: ; preds = %271, %208
100299|  %274 = gep %1, i64 128                                                                                                ;L738
100300|  %275 = load i8, ptr %274, , !!8                                                                                       ;L738
100301|  %276 = trunc nuw i8 %275 to i1                                                                                        ;L738
100302|  br i1 %276, label %277, label %323                                                                                    ;L738
100303| 
100304| 277: ; preds = %273
100305|  %278 = gep %1, i64 120                                                                                                ;L739
100306|  %279 = load i64, ptr %278, , !!8                                                                                      ;L739
100307|     ;; rhs = i64 %279
100308|  %280 = tail call i64 @llvm.usub.sat.i64(i64 %132, i64 %279)                                                           ;L2472<739
100309|  %281 = icmp ugt i64 %280, 119                                                                                         ;L739
100311|  %282 = gep %1, i64 80                                                                                                 ;L0
100312|  %283 = load i64, ptr %282, , !!8                                                                                      ;L0
100313|  br i1 %281, label %284, label %291                                                                                    ;L740
100314| 
100315| 284: ; preds = %277
100316|  %285 = udiv i64 %283, 32000                                                                                           ;L741
100317|     ;; self = i64 %285
100318|     ;; min = i64 0
100319|     ;; max = i64 29
100320|  %286 = tail call i64 @llvm.umin.i64(i64 %285, i64 29)                                                                 ;L2027<741
100321|  %287 = gep %1, i64 88                                                                                                 ;L741
100322|  %288 = load i64, ptr %287, , !!8                                                                                      ;L741
100323|  %289 = udiv i64 %288, 32000                                                                                           ;L741
100324|     ;; self = i64 %289
100325|     ;; min = i64 0
100326|     ;; max = i64 29
100327|  %290 = tail call i64 @llvm.umin.i64(i64 %289, i64 29)                                                                 ;L2027<741
100329|     ;; stale_goal_cell[8..+8] = i64 %286
100330|     ;; stale_goal_cell[16..+8] = i64 %290
100331|  store i8 0, ptr %274,                                                                                                 ;L746
100332|  br label %323                                                                                                         ;L745
100333| 
100334| 291: ; preds = %277
100335|     ;; x2 = i64 %283
100336|     ;; other = i64 %283
100337|  %292 = gep %1, i64 88                                                                                                 ;L751
100338|  %293 = load i64, ptr %292, , !!8                                                                                      ;L751
100339|     ;; y2 = i64 %293
100340|     ;; other = i64 %293
100341|  %294 = icmp ult i64 %69, %283                                                                                         ;L3147<7<751
100342|  %295 = sub nuw i64 %283, %69                                                                                          ;L3147<7<751
100343|  %296 = sub nuw i64 %69, %283                                                                                          ;L3147<7<751
100344|  %297 = select i1 %294, i64 %295, i64 %296                                                                             ;L3147<7<751
100345|     ;; dx = i64 %297
100346|  %298 = icmp ult i64 %107, %293                                                                                        ;L3147<8<751
100347|  %299 = sub nuw i64 %293, %107                                                                                         ;L3147<8<751
100348|  %300 = sub nuw i64 %107, %293                                                                                         ;L3147<8<751
100349|  %301 = select i1 %298, i64 %299, i64 %300                                                                             ;L3147<8<751
100350|     ;; dy = i64 %301
100351|  %302 = mul i64 %297, %297                                                                                             ;L9<751
100352|  %303 = mul i64 %301, %301                                                                                             ;L9<751
100353|  %304 = add i64 %303, %302                                                                                             ;L9<751
100354|  %305 = icmp ugt i64 %304, 143999999                                                                                   ;L751
100355|  br i1 %305, label %306, label %323                                                                                    ;L751
100356| 
100357| 306: ; preds = %291
100359|  call void @ai::small_action29positioning_score_at_position(ptr sret([56 x i8]) %36, i64 %2, ptr %4, ptr %5, ptr %6, i64 %283, i64 %293, i8 4) ;L752
100360|  %307 = load i64, ptr %36, , !!8                                                                                       ;L753
100361|  %308 = gep %36, i64 40                                                                                                ;L753
100362|  %309 = load i64, ptr %308,                                                                                            ;L753
100365|  %310 = select i1 %105, i64 %309, i64 0                                                                                ;L711<753
100366|  %311 = add i64 %310, %307                                                                                             ;L711<753
100367|  %312 = gep %36, i64 48                                                                                                ;L754
100368|  %313 = load i8, ptr %312, , !!8                                                                                       ;L754
100369|  %314 = trunc nuw i8 %313 to i1                                                                                        ;L754
100370|  %315 = gep %36, i64 49                                                                                                ;L754
100371|  %316 = load i8, ptr %315,                                                                                             ;L754
100372|  %317 = trunc nuw i8 %316 to i1                                                                                        ;L754
100373|  %318 = select i1 %314, i1 true, i1 %317                                                                               ;L754
100374|  %319 = call i64 @ai::small_action22positioning_risk_value(i64 %2, ptr %4, i64 %311, i1 zeroext %318)                  ;L753
100375|     ;; risk_now = i64 %319
100376|  %320 = gep %1, i64 104                                                                                                ;L755
100377|  %321 = load i64, ptr %320, , !!8                                                                                      ;L755
100378|  %322 = icmp sgt i64 %319, %321                                                                                        ;L755
100379|     ;; keep_committed_goal = i1 %322
100381|  br i1 %322, label %323, label %136                                                                                    ;L757
100382| 
100383| 323: ; preds = %306, %291, %284, %273
100384|  %324 = phi i1 [ false, %291 ], [ false, %306 ], [ false, %273 ], [ true, %284 ]
100385|  %325 = phi i64 [ undef, %291 ], [ undef, %306 ], [ undef, %273 ], [ %290, %284 ]
100386|  %326 = phi i64 [ undef, %291 ], [ undef, %306 ], [ undef, %273 ], [ %286, %284 ]
100387|     ;; self = ptr %4
100388|     ;; champ = ptr %56
100389|     ;; other = ptr %56
100390|  %327 = load i64, ptr %68, , !!8                                                                                       ;L760
100391|     ;; x2 = i64 %327
100392|     ;; other = i64 %327
100393|     ;; x1 = i64 %327
100394|     ;; self = i64 %327
100395|     ;; x1 = i64 %327
100396|     ;; self = i64 %327
100397|  %328 = udiv i64 %327, 32000                                                                                           ;L760
100398|     ;; self = i64 %328
100399|     ;; min = i64 0
100400|     ;; max = i64 29
100401|  %329 = call i64 @llvm.umin.i64(i64 %328, i64 29)                                                                      ;L2027<760
100402|     ;; champ_x = i64 %329
100403|     ;; champ_x = i64 %329
100404|  %330 = load i64, ptr %106, , !!8                                                                                      ;L761
100405|     ;; y2 = i64 %330
100406|     ;; other = i64 %330
100407|     ;; y1 = i64 %330
100408|     ;; self = i64 %330
100409|     ;; y1 = i64 %330
100410|     ;; self = i64 %330
100411|  %331 = udiv i64 %330, 32000                                                                                           ;L761
100412|     ;; self = i64 %331
100413|     ;; min = i64 0
100414|     ;; max = i64 29
100415|  %332 = call i64 @llvm.umin.i64(i64 %331, i64 29)                                                                      ;L2027<761
100416|     ;; champ_y = i64 %332
100417|  %333 = udiv i64 %44, 32000                                                                                            ;L762
100418|     ;; hx = i64 %333
100419|  %334 = udiv i64 %45, 32000                                                                                            ;L763
100420|     ;; hy = i64 %334
100421|  %335 = gep %6, i64 2744                                                                                               ;L764
100422|  %336 = load i64, ptr %335, , !!8                                                                                      ;L764
100423|     ;; cx = i64 %336
100424|  %337 = gep %6, i64 2752                                                                                               ;L765
100425|  %338 = load i64, ptr %337, , !!8                                                                                      ;L765
100426|     ;; cy = i64 %338
100428|  %339 = load ptr, ptr %60, , !!8, !!8                                                                                  ;L766
100429|     ;; bump = ptr %339
100430|     ;; bump = ptr %339
100431|  store ptr inttoptr (i64 8 to ptr), ptr %35,                                                                           ;L547<766
100432|  %340 = gep %35, i64 8                                                                                                 ;L547<766
100433|  store ptr %339, ptr %340,                                                                                             ;L547<766
100434|  %341 = gep %35, i64 16                                                                                                ;L547<766
100435|  %342 = gep %35, i64 24                                                                                                ;L547<766
100436|     ;; self = ptr %38
100437|     ;; self[0..+8] = ptr %38
100438|     ;; slice[0..+8] = ptr %38
100439|     ;; slice[0..+8] = ptr %38
100440|     ;; self[8..+8] = i64 5
100441|     ;; slice[8..+8] = i64 5
100442|     ;; slice[8..+8] = i64 5
100443|  %343 = icmp ult i64 %97, 6
100444|  call void @llvm.memset.p0.i64(ptr %341, i8 0, i64 16, i1 false)                                                       ;L547<766
100445|  br i1 %343, label %345, label %344                                                                                    ;L1050<437<529<19<391<769
100446| 
100447| 344: ; preds = %323
100448|  invoke void @core::slice5index16slice_index_fail(i64 0, i64 %97, i64 5, ptr @anon.afb017e85830050c32e5158c55060899.81) #25
100449|  to label %448 unwind label %373                                                                                       ;L443<529<19<391<769
100450| 
100451| 345: ; preds = %323
100452|     ;; self[0..+8] = ptr %38
100453|     ;; slice[0..+8] = ptr %38
100454|     ;; self[8..+8] = i64 %97
100455|     ;; slice[8..+8] = i64 %97
100456|     ;; ptr = ptr %38
100457|     ;; self = ptr %38
100458|  %346 = shl nuw nsw i64 %97, 4                                                                                         ;L961<100<1042<769
100459|  %347 = gep %38, i64 %346                                                                                              ;L961<100<1042<769
100460|     ;; self[0..+8] = ptr %38
100461|     ;; self[8..+8] = ptr %347
100462|     ;; self[16..+8] = ptr %56
100463|     ;; init = i64 0
100465|     ;; before = i64 %97
100466|     ;; self[0..+8] = ptr %38
100467|     ;; iter[0..+8] = ptr %38
100468|     ;; self[0..+8] = ptr %38
100469|     ;; self[8..+8] = ptr %347
100470|     ;; iter[8..+8] = ptr %347
100471|     ;; self[8..+8] = ptr %347
100472|     ;; self[16..+8] = ptr %56
100473|     ;; iter[16..+8] = ptr %56
100474|     ;; self[16..+8] = ptr %56
100475|     ;; f = ptr %56
100476|     ;; self[0..+8] = ptr %38
100477|     ;; self[8..+8] = ptr %347
100478|     ;; init = i64 0
100479|     ;; rhs = i64 1
100480|     ;; end = ptr %347
100483|  %348 = icmp eq i64 %97, 0                                                                                             ;L1714<44<128<52<3674<142<771
100484|  br i1 %348, label %382, label %349                                                                                    ;L25<128<52<3674<142<771
100485| 
100486| 349: ; preds = %349, %345
100487|  %350 = phi i64 [ %371, %349 ], [ 0, %345 ]                                                                            ;L0<128<52<3674<142<771
100488|  %351 = phi i64 [ %370, %349 ], [ 0, %345 ]                                                                            ;L0<128<52<3674<142<771
100489|     ;; acc = i64 %351
100490|     ;; self = i64 %350
100491|     ;; i = i64 %350
100492|     ;; self = ptr %38
100493|     ;; count = i64 %350
100494|  %352 = gepS %38, i64 %350                                                                                             ;L656<279<128<52<3674<142<771
100495|  %353 = load i64, ptr %352, , !!61522, !!8                                                                             ;L279<128<52<3674<142<771
100496|  %354 = gep %352, i64 8                                                                                                ;L279<128<52<3674<142<771
100497|  %355 = load i64, ptr %354, , !!61522, !!8                                                                             ;L279<128<52<3674<142<771
100499|     ;; acc = i64 %351
100507|     ;; x1 = i64 %327
100508|     ;; self = i64 %327
100509|     ;; y1 = i64 %330
100510|     ;; self = i64 %330
100511|     ;; x2 = i64 %353
100512|     ;; other = i64 %353
100513|     ;; y2 = i64 %355
100514|     ;; other = i64 %355
100515|  %356 = icmp ult i64 %327, %353                                                                                        ;L3147<7<770<138<88<279<128<52<3674<142<771
100516|  %357 = sub nuw i64 %353, %327                                                                                         ;L3147<7<770<138<88<279<128<52<3674<142<771
100517|  %358 = sub nuw i64 %327, %353                                                                                         ;L3147<7<770<138<88<279<128<52<3674<142<771
100518|  %359 = select i1 %356, i64 %357, i64 %358                                                                             ;L3147<7<770<138<88<279<128<52<3674<142<771
100519|     ;; dx = i64 %359
100520|  %360 = icmp ult i64 %330, %355                                                                                        ;L3147<8<770<138<88<279<128<52<3674<142<771
100521|  %361 = sub nuw i64 %355, %330                                                                                         ;L3147<8<770<138<88<279<128<52<3674<142<771
100522|  %362 = sub nuw i64 %330, %355                                                                                         ;L3147<8<770<138<88<279<128<52<3674<142<771
100523|  %363 = select i1 %360, i64 %361, i64 %362                                                                             ;L3147<8<770<138<88<279<128<52<3674<142<771
100524|     ;; dy = i64 %363
100525|  %364 = mul i64 %359, %359                                                                                             ;L9<770<138<88<279<128<52<3674<142<771
100526|  %365 = mul i64 %363, %363                                                                                             ;L9<770<138<88<279<128<52<3674<142<771
100527|  %366 = add i64 %365, %364                                                                                             ;L9<770<138<88<279<128<52<3674<142<771
100528|  %367 = freeze i64 %366                                                                                                ;L770<138<88<279<128<52<3674<142<771
100529|  %368 = icmp ult i64 %367, 40000000000                                                                                 ;L770<138<88<279<128<52<3674<142<771
100530|  %369 = zext i1 %368 to i64                                                                                            ;L138<88<279<128<52<3674<142<771
100532|     ;; a = i64 %351
100533|     ;; b = i64 %369
100534|  %370 = add i64 %351, %369                                                                                             ;L55<88<279<128<52<3674<142<771
100535|     ;; acc = i64 %370
100536|  %371 = add nuw nsw i64 %350, 1                                                                                        ;L971<283<128<52<3674<142<771
100537|     ;; i = i64 %371
100538|     ;; self = i64 %371
100539|  %372 = icmp eq i64 %371, %97                                                                                          ;L284<128<52<3674<142<771
100540|  br i1 %372, label %376, label %349                                                                                    ;L284<128<52<3674<142<771
100541| 
100542| 373: ; preds = %773, %447, %436, %415, %408, %344
100543|  %374 = phi i1 [ %439, %447 ], [ true, %773 ], [ true, %436 ], [ true, %415 ], [ true, %408 ], [ true, %344 ]          ;L766
100544|  %375 = cleanuppad within none []
100545|  br i1 %374, label %972, label %971                                                                                    ;L939
100546| 
100547| 376: ; preds = %349
100548|     ;; total = i64 %370
100549|     ;; count = i64 %370
100550|     ;; upper = i64 %97
100551|  %377 = icmp ule i64 %370, %97                                                                                         ;L251<145<771
100552|     ;; cond = i1 true
100553|  call void @llvm.assume(i1 %377)                                                                                       ;L210<251<145<771
100554|     ;; nearby = i64 %370
100555|  %378 = icmp ugt i64 %370, 2                                                                                           ;L772
100556|  %379 = icmp eq i64 %370, 2                                                                                            ;L772
100557|  %380 = select i1 %379, i64 3, i64 1                                                                                   ;L772
100558|  %381 = select i1 %378, i64 5, i64 %380                                                                                ;L772
100559|     ;; risk_weight = i64 %381
100560|  br i1 %75, label %422, label %384                                                                                     ;L776
100561| 
100562| 382: ; preds = %345
100563|     ;; total = i64 0
100564|     ;; count = i64 0
100565|     ;; upper = i64 %97
100566|     ;; cond = i1 true
100567|     ;; nearby = i64 0
100568|     ;; risk_weight = i64 1
100569|  br i1 %75, label %422, label %383                                                                                     ;L776
100570| 
100571| 383: ; preds = %382
100572|     ;; self = ptr %38
100573|     ;; self[0..+8] = ptr %38
100574|     ;; slice[0..+8] = ptr %38
100575|     ;; slice[0..+8] = ptr %38
100576|     ;; self[8..+8] = i64 5
100577|     ;; slice[8..+8] = i64 5
100578|     ;; slice[8..+8] = i64 5
100579|     ;; self[0..+8] = ptr %38
100580|     ;; slice[0..+8] = ptr %38
100581|     ;; self[8..+8] = i64 %97
100582|     ;; slice[8..+8] = i64 %97
100583|     ;; self = ptr %38
100584|     ;; self[0..+8] = ptr %38
100585|     ;; self[0..+8] = ptr %38
100586|     ;; self[8..+8] = ptr %347
100587|     ;; self[8..+8] = ptr %347
100588|     ;; self[16..+8] = ptr %56
100589|     ;; self[16..+8] = ptr %56
100590|     ;; f = ptr %56
100591|     ;; self = ptr %13
100594|     ;; f = ptr %56
100595|     ;; self = ptr %13
100597|     ;; self = ptr %13
100598|     ;; self = ptr %13
100599|     ;; predicate = ptr %13
100600|     ;; self = ptr %13
100601|     ;; self = ptr %13
100602|     ;; count = i64 1
100603|     ;; ptr = ptr %38
100604|     ;; self = ptr %38
100605|     ;; end_or_len = ptr %347
100608|  br label %410                                                                                                         ;L180<348<98<107<2706<3416<3387<779
100609| 
100610| 384: ; preds = %376
100611|     ;; self = ptr %38
100612|     ;; self[0..+8] = ptr %38
100613|     ;; slice[0..+8] = ptr %38
100614|     ;; slice[0..+8] = ptr %38
100615|     ;; self[8..+8] = i64 5
100616|     ;; slice[8..+8] = i64 5
100617|     ;; slice[8..+8] = i64 5
100618|     ;; self[0..+8] = ptr %38
100619|     ;; slice[0..+8] = ptr %38
100620|     ;; self[8..+8] = i64 %97
100621|     ;; slice[8..+8] = i64 %97
100622|     ;; self = ptr %38
100623|     ;; self[0..+8] = ptr %38
100624|     ;; self[0..+8] = ptr %38
100625|     ;; self[8..+8] = ptr %347
100626|     ;; self[8..+8] = ptr %347
100627|     ;; self[16..+8] = ptr %56
100628|     ;; self[16..+8] = ptr %56
100629|     ;; f = ptr %56
100630|     ;; self = ptr %13
100634|     ;; f = ptr %56
100635|  %385 = gep %13, i64 8                                                                                                 ;L69<836<3387<779
100636|  store ptr %347, ptr %385, , !!61789                                                                                   ;L69<836<3387<779
100637|  %386 = gep %13, i64 16                                                                                                ;L69<836<3387<779
100638|  store ptr %56, ptr %386, , !!61789                                                                                    ;L69<836<3387<779
100639|  %387 = gep %13, i64 24                                                                                                ;L69<836<3387<779
100640|  store ptr %56, ptr %387, , !!61792                                                                                    ;L69<836<3387<779
100641|     ;; self = ptr %13
100643|     ;; self = ptr %13
100644|     ;; self = ptr %13
100645|     ;; predicate = ptr %13
100646|     ;; self = ptr %13
100647|     ;; self = ptr %13
100648|     ;; count = i64 1
100649|     ;; ptr = ptr %38
100650|     ;; self = ptr %38
100651|     ;; end_or_len = ptr %347
100654|  br label %390                                                                                                         ;L180<348<98<107<2706<3416<3387<779
100655| 
100656| 388: ; preds = %390
100657|     ;; ptr = ptr %392
100658|     ;; self = ptr %392
100659|     ;; end_or_len = ptr %347
100662|  %389 = icmp eq ptr %392, %347                                                                                         ;L1714<180<348<98<107<2706<3416<3387<779
100663|  br i1 %389, label %410, label %390                                                                                    ;L180<348<98<107<2706<3416<3387<779
100664| 
100665| 390: ; preds = %388, %384
100666|  %391 = phi ptr [ %392, %388 ], [ %38, %384 ]
100667|     ;; ptr = ptr %391
100668|  %392 = gep %391, i64 16                                                                                               ;L656<185<348<98<107<2706<3416<3387<779
100669|     ;; x = ptr %391
100674|     ;; ex = ptr %391
100675|     ;; ey = ptr %391
100676|     ;; x1 = i64 %327
100677|     ;; self = i64 %327
100678|     ;; y1 = i64 %330
100679|     ;; self = i64 %330
100680|  %393 = load i64, ptr %391, , !!61843, !!8                                                                             ;L778<298<349<98<107<2706<3416<3387<779
100681|     ;; x2 = i64 %393
100682|     ;; other = i64 %393
100683|  %394 = gep %391, i64 8                                                                                                ;L778<298<349<98<107<2706<3416<3387<779
100684|  %395 = load i64, ptr %394, , !!61843, !!8                                                                             ;L778<298<349<98<107<2706<3416<3387<779
100685|     ;; y2 = i64 %395
100686|     ;; other = i64 %395
100687|  %396 = icmp ult i64 %327, %393                                                                                        ;L3147<7<778<298<349<98<107<2706<3416<3387<779
100688|  %397 = sub nuw i64 %393, %327                                                                                         ;L3147<7<778<298<349<98<107<2706<3416<3387<779
100689|  %398 = sub nuw i64 %327, %393                                                                                         ;L3147<7<778<298<349<98<107<2706<3416<3387<779
100690|  %399 = select i1 %396, i64 %397, i64 %398                                                                             ;L3147<7<778<298<349<98<107<2706<3416<3387<779
100691|     ;; dx = i64 %399
100692|  %400 = icmp ult i64 %330, %395                                                                                        ;L3147<8<778<298<349<98<107<2706<3416<3387<779
100693|  %401 = sub nuw i64 %395, %330                                                                                         ;L3147<8<778<298<349<98<107<2706<3416<3387<779
100694|  %402 = sub nuw i64 %330, %395                                                                                         ;L3147<8<778<298<349<98<107<2706<3416<3387<779
100695|  %403 = select i1 %400, i64 %401, i64 %402                                                                             ;L3147<8<778<298<349<98<107<2706<3416<3387<779
100696|     ;; dy = i64 %403
100697|  %404 = mul i64 %399, %399                                                                                             ;L9<778<298<349<98<107<2706<3416<3387<779
100698|  %405 = mul i64 %403, %403                                                                                             ;L9<778<298<349<98<107<2706<3416<3387<779
100699|  %406 = add i64 %405, %404                                                                                             ;L9<778<298<349<98<107<2706<3416<3387<779
100700|  %407 = icmp ult i64 %406, 62500000000                                                                                 ;L778<298<349<98<107<2706<3416<3387<779
100701|  br i1 %407, label %408, label %388                                                                                    ;L349<98<107<2706<3416<3387<779
100702| 
100703| 408: ; preds = %390
100704|  store ptr %392, ptr %13, , !!61777                                                                                    ;L185<348<98<107<2706<3416<3387<779
100705|     ;; first[0..+8] = i64 %406
100706|     ;; first[8..+8] = ptr %391
100707|  %409 = invoke { i64, ptr } @core::iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtNtBc_5slice4iter4IterTyyEENCNvMs1_NtNtCshdEBA0ozCnw_7game_ai12small_action12move_actionsNtB1W_17SmallActionRecall9get_inputs0_0ENCINvNvNtNtNtBa_6traits8iterator8Iterator10min_by_key3keyRB1J_yNCB1Q_s1_0E0EB3z_4foldTyB4n_ENCINvNvB3z_6min_by4foldB4P_INvB3x_7compareB4n_yEE0EB20_(ptr %13, i64 %406, ptr %391)
100708|  to label %412 unwind label %373                                                                                       ;L2707<3416<3387<779
100709| 
100710| 410: ; preds = %388, %383
100711|  %411 = phi i64 [ 1, %383 ], [ %381, %388 ]
100713|     ;; self = ptr null
100714|  br label %424                                                                                                         ;L2137<780
100715| 
100716| 412: ; preds = %408
100717|  %413 = extractvalue { i64, ptr } %409, 1                                                                              ;L2707<3416<3387<779
100719|     ;; self = ptr %413
100720|  %414 = icmp eq ptr %413, null                                                                                         ;L2137<780
100721|  br i1 %414, label %424, label %415                                                                                    ;L2137<780
100722| 
100723| 415: ; preds = %412
100724|  %416 = load i64, ptr %413, , !!8                                                                                      ;L2138<780
100725|  %417 = gep %413, i64 8                                                                                                ;L2138<780
100726|  %418 = load i64, ptr %417, , !!8                                                                                      ;L2138<780
100727|     ;; f2_enemy_anchor[8..+8] = i64 %416
100728|     ;; f2_enemy_anchor[16..+8] = i64 %418
100729|     ;; f2_enemy_anchor[0..+8] = i64 1
100730|     ;; self[0..+8] = i64 1
100731|     ;; self[8..+8] = i64 %416
100732|     ;; self[16..+8] = i64 %418
100733|     ;; f = ptr %56
100734|     ;; x[0..+8] = i64 %416
100735|     ;; x[8..+8] = i64 %418
100736|  %419 = load i64, ptr %68, , !!8                                                                                       ;L1162<784
100737|  %420 = load i64, ptr %106, , !!8                                                                                      ;L1162<784
100741|     ;; ex = i64 %416
100742|     ;; ey = i64 %418
100743|  %421 = invoke i64 @gc::utils8distance(i64 %419, i64 %420, i64 %416, i64 %418)
100744|  to label %424 unwind label %373                                                                                       ;L784<1162<784
100745| 
100746| 422: ; preds = %382, %376
100747|  %423 = phi i64 [ 1, %382 ], [ %381, %376 ]
100749|     ;; f2_enemy_anchor[8..+8] = i64 undef
100750|     ;; f2_enemy_anchor[16..+8] = i64 undef
100752|     ;; f2_now_dist[8..+8] = i64 undef
100754|     ;; f = ptr %62
100755|  br label %430                                                                                                         ;L66<787
100756| 
100757| 424: ; preds = %415, %412, %410
100758|  %425 = phi i64 [ %381, %412 ], [ %381, %415 ], [ %411, %410 ]
100759|  %426 = phi i64 [ undef, %412 ], [ %421, %415 ], [ undef, %410 ]                                                       ;L0<784
100760|  %427 = phi i64 [ undef, %412 ], [ %418, %415 ], [ undef, %410 ]
100761|  %428 = phi i64 [ undef, %412 ], [ %416, %415 ], [ undef, %410 ]
100762|  %429 = phi i1 [ false, %412 ], [ true, %415 ], [ false, %410 ]                                                        ;L0<780
100764|     ;; f2_enemy_anchor[8..+8] = i64 %428
100765|     ;; f2_enemy_anchor[16..+8] = i64 %427
100767|     ;; f2_now_dist[8..+8] = i64 %426
100769|     ;; f = ptr %62
100770|  br i1 %75, label %430, label %436                                                                                     ;L66<787
100771| 
100772| 430: ; preds = %424, %422
100773|  %431 = phi i1 [ false, %422 ], [ %429, %424 ]
100774|  %432 = phi i64 [ undef, %422 ], [ %428, %424 ]
100775|  %433 = phi i64 [ undef, %422 ], [ %427, %424 ]
100776|  %434 = phi i64 [ undef, %422 ], [ %426, %424 ]
100777|  %435 = phi i64 [ %423, %422 ], [ %425, %424 ]
100778|  store ptr null, ptr %34,                                                                                              ;L66<787
100779|     ;; b1_home_w_per_unit = i64 0
100780|     ;; c1_chaser[0..+8] = i64 0
100781|  br label %515                                                                                                         ;L808
100782| 
100783| 436: ; preds = %424
100785|  %437 = invoke ptr @ai::free_dist16shared_free_dist(ptr %62)
100786|  to label %449 unwind label %373                                                                                       ;L787<66<787
100787| 
100788| 438: ; preds = %961, %940, %939, %938, %778, %777, %775, %552, %514, %512, %470
100789|  %439 = phi i1 [ true, %775 ], [ true, %961 ], [ true, %940 ], [ true, %470 ], [ true, %514 ], [ %630, %939 ], [ %630, %938 ], [ true, %778 ], [ true, %552 ], [ true, %512 ], [ true, %777 ] ;L766
100790|  %440 = cleanuppad within none []
100793|  %441 = load ptr, ptr %34, , !!8                                                                                       ;L825<939
100794|  %442 = icmp eq ptr %441, null                                                                                         ;L825<939
100795|  br i1 %442, label %447, label %443                                                                                    ;L825<939
100796| 
100797| 443: ; preds = %438
100799|     ;; self = ptr %34
100800|     ;; val = i64 1
100801|     ;; order = i8 1
100802|     ;; val = i64 1
100803|     ;; order = i8 1
100804|     ;; self = ptr %441
100805|     ;; dst = ptr %441
100806|  %444 = atomicrmw sub ptr %441, i64 1 release, , !!61973                                                               ;L3956<3193<2831<825<825<939
100807|  %445 = icmp eq i64 %444, 1                                                                                            ;L2831<825<825<939
100808|  br i1 %445, label %446, label %447                                                                                    ;L2831<825<825<939
100809| 
100810| 446: ; preds = %443
100811|     ;; order = i8 2
100812|  fence acquire                                                                                                         ;L4387<64<825<825<939
100813|  call void @ai::free_dist11FreeDistMapE9drop_slowBK_(ptr %34) [ "funclet"(token %440) ]                                ;L2874<825<825<939
100814|  br label %447                                                                                                         ;L2874<825<825<939
100815| 
100816| 447: ; preds = %446, %443, %438
100817|  cleanupret from %440 unwind label %373                                                                                ;L939
100818| 
100819| 448: ; preds = %669, %514, %344
100820|  unreachable
100821| 
100822| 449: ; preds = %436
100823|  store ptr %437, ptr %34,                                                                                              ;L66<787
100824|  %450 = load i64, ptr %109, , !!8                                                                                      ;L789
100825|     ;; self = i64 %450
100826|     ;; other = i64 1
100827|  %451 = call i64 @llvm.umax.i64(i64 %450, i64 1)                                                                       ;L1039<789
100828|  %452 = udiv i64 32000, %451                                                                                           ;L789
100829|     ;; self = i64 %452
100830|     ;; other = i64 1
100831|  %453 = call i64 @llvm.umax.i64(i64 %452, i64 1)                                                                       ;L1039<789
100832|     ;; edge_ticks = i64 %453
100833|     ;; rhs = i64 %453
100834|     ;; rhs = i64 %453
100835|     ;; rhs = i64 %453
100836|     ;; edge_dmg_milli = i64 0
100837|     ;; self = i64 0
100838|  %454 = sub nuw nsw i64 1, %42                                                                                         ;L791
100839|     ;; team = i64 %454
100840|  %455 = getelementptr [5 x ptr], ptr %53, i64 %454                                                                     ;L1905<791
100841|     ;; self[0..+8] = ptr %455
100842|     ;; slice[0..+8] = ptr %455
100843|     ;; self[8..+8] = i64 5
100844|     ;; slice[8..+8] = i64 5
100845|     ;; ptr = ptr %455
100846|     ;; self = ptr %455
100847|  %456 = gep %455, i64 40                                                                                               ;L961<100<1042<1905<791
100849|  store ptr %455, ptr %33,                                                                                              ;L791
100850|  %457 = gep %33, i64 8                                                                                                 ;L791
100851|  store ptr %456, ptr %457,                                                                                             ;L791
100852|  %458 = gep %33, i64 16
100853|  %459 = gep %5, i64 16
100854|  %460 = load ptr, ptr %459, , !!8
100855|  %461 = getelementptr { { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, [5 x { i64, [2 x i64] }], [5 x { i64, { i8, [23 x i8] } }], [5 x { i8, [7 x i8], i64 }], [5 x i64], [5 x i64], [5 x i64], [5 x { i64, i64 }], i64, i64, i64, i64, i64, [5 x i32], i8, [3 x i8] }, ptr %460, i64 %454
100856|  br label %462                                                                                                         ;L791
100857| 
100858| 462: ; preds = %968, %449
100859|  %463 = phi i64 [ %970, %968 ], [ 0, %449 ]
100860|  br label %464                                                                                                         ;L365<64<791
100861| 
100862| 464: ; preds = %960, %462
100863|     ;; self = i64 %463
100864|     ;; edge_dmg_milli = i64 %463
100865|     ;; self = ptr %33
100868|  store ptr %458, ptr %12, , !!62067
100869|     ;; self = ptr %33
100870|     ;; self = ptr %33
100871|     ;; f = ptr %12
100872|     ;; count = i64 1
100873|  %465 = load ptr, ptr %457, , !!62088, !!8, !!8
100874|  %466 = load ptr, ptr %33, , !!62088
100875|  br label %467                                                                                                         ;L365<64<791
100876| 
100877| 467: ; preds = %473, %464
100878|  %468 = phi ptr [ %471, %473 ], [ %466, %464 ]
100879|     ;; ptr = ptr %468
100880|     ;; self = ptr %468
100881|     ;; end_or_len = ptr %465
100884|  %469 = icmp eq ptr %468, %465                                                                                         ;L1714<180<365<64<791
100885|  br i1 %469, label %475, label %470                                                                                    ;L180<365<64<791
100886| 
100887| 470: ; preds = %467
100888|  %471 = gep %468, i64 8                                                                                                ;L656<185<365<64<791
100889|  store ptr %471, ptr %33, , !!62088                                                                                    ;L185<365<64<791
100890|     ;; x = ptr %468
100891|  %472 = invoke ptr @gc::simulationNtBW_21AbstractGameWithCache14iter_champions0INtB7_5FnMutTRINtNtBb_6option6OptionRNtNtBW_6entity6EntityEEE8call_mutCshdEBA0ozCnw_7game_ai(ptr %12, ptr %468)
100892|  to label %473 unwind label %438                                                                                       ;L366<64<791
100893| 
100894| 473: ; preds = %470
100895|  %474 = icmp eq ptr %472, null                                                                                         ;L366<64<791
100896|  br i1 %474, label %467, label %940                                                                                    ;L366<64<791
100897| 
100898| 475: ; preds = %467
100901|  %476 = gep %56, i64 1648                                                                                              ;L801
100902|  %477 = load i64, ptr %476, , !!8                                                                                      ;L801
100903|     ;; self = i64 %477
100904|     ;; other = i64 1
100905|  %478 = call i64 @llvm.umax.i64(i64 %477, i64 1)                                                                       ;L1039<801
100906|  %479 = mul i64 %478, 10                                                                                               ;L801
100907|  %480 = icmp eq i64 %479, 0                                                                                            ;L801
100908|  br i1 %480, label %514, label %481                                                                                    ;L801
100909| 
100910| 481: ; preds = %475
100911|  %482 = udiv i64 %463, %479                                                                                            ;L801
100912|     ;; chase_pct_per_cell = i64 %482
100913|  %483 = mul i64 %482, %425                                                                                             ;L802
100914|  %484 = sdiv i64 %483, 5                                                                                               ;L802
100915|  %485 = add nsw i64 %484, 2                                                                                            ;L802
100916|     ;; b1_home_w_per_unit = i64 %485
100917|     ;; self = ptr %38
100918|     ;; self[0..+8] = ptr %38
100919|     ;; slice[0..+8] = ptr %38
100920|     ;; slice[0..+8] = ptr %38
100921|     ;; self[8..+8] = i64 5
100922|     ;; slice[8..+8] = i64 5
100923|     ;; slice[8..+8] = i64 5
100924|     ;; self[0..+8] = ptr %38
100925|     ;; slice[0..+8] = ptr %38
100926|     ;; self[8..+8] = i64 %97
100927|     ;; slice[8..+8] = i64 %97
100928|     ;; self = ptr %38
100929|     ;; self[0..+8] = ptr %38
100930|     ;; self[0..+8] = ptr %38
100931|     ;; self[8..+8] = ptr %347
100932|     ;; self[8..+8] = ptr %347
100933|     ;; self[16..+8] = ptr %56
100934|     ;; self[16..+8] = ptr %56
100935|     ;; f = ptr %56
100936|     ;; self = ptr %11
100940|     ;; f = ptr %56
100941|  %486 = gep %11, i64 8                                                                                                 ;L69<836<3387<811
100942|  store ptr %347, ptr %486, , !!62185                                                                                   ;L69<836<3387<811
100943|  %487 = gep %11, i64 16                                                                                                ;L69<836<3387<811
100944|  store ptr %56, ptr %487, , !!62185                                                                                    ;L69<836<3387<811
100945|  %488 = gep %11, i64 24                                                                                                ;L69<836<3387<811
100946|  store ptr %56, ptr %488, , !!62188                                                                                    ;L69<836<3387<811
100947|     ;; self = ptr %11
100949|     ;; self = ptr %11
100950|     ;; self = ptr %11
100951|     ;; predicate = ptr %11
100952|     ;; self = ptr %11
100953|     ;; self = ptr %11
100954|     ;; count = i64 1
100955|     ;; ptr = ptr %38
100956|     ;; self = ptr %38
100957|     ;; end_or_len = ptr %347
100960|  br i1 %348, label %532, label %489                                                                                    ;L180<348<98<107<2706<3416<3387<811
100961| 
100962| 489: ; preds = %481
100963|     ;; predicate = ptr %11
100964|  %490 = load i64, ptr %68, , !!62260, !!8
100965|  %491 = load i64, ptr %106, , !!62260, !!8
100966|  br label %494                                                                                                         ;L180<348<98<107<2706<3416<3387<811
100967| 
100968| 492: ; preds = %494
100969|     ;; ptr = ptr %496
100970|     ;; self = ptr %496
100971|     ;; end_or_len = ptr %347
100974|  %493 = icmp eq ptr %496, %347                                                                                         ;L1714<180<348<98<107<2706<3416<3387<811
100975|  br i1 %493, label %532, label %494                                                                                    ;L180<348<98<107<2706<3416<3387<811
100976| 
100977| 494: ; preds = %492, %489
100978|  %495 = phi ptr [ %38, %489 ], [ %496, %492 ]
100979|     ;; ptr = ptr %495
100980|  %496 = gep %495, i64 16                                                                                               ;L656<185<348<98<107<2706<3416<3387<811
100981|     ;; x = ptr %495
100986|     ;; ex = ptr %495
100987|     ;; ey = ptr %495
100988|     ;; x1 = i64 %490
100989|     ;; self = i64 %490
100990|     ;; y1 = i64 %491
100991|     ;; self = i64 %491
100992|  %497 = load i64, ptr %495, , !!62260, !!8                                                                             ;L810<298<349<98<107<2706<3416<3387<811
100993|     ;; x2 = i64 %497
100994|     ;; other = i64 %497
100995|  %498 = gep %495, i64 8                                                                                                ;L810<298<349<98<107<2706<3416<3387<811
100996|  %499 = load i64, ptr %498, , !!62260, !!8                                                                             ;L810<298<349<98<107<2706<3416<3387<811
100997|     ;; y2 = i64 %499
100998|     ;; other = i64 %499
100999|  %500 = icmp ult i64 %490, %497                                                                                        ;L3147<7<810<298<349<98<107<2706<3416<3387<811
101000|  %501 = sub nuw i64 %497, %490                                                                                         ;L3147<7<810<298<349<98<107<2706<3416<3387<811
101001|  %502 = sub nuw i64 %490, %497                                                                                         ;L3147<7<810<298<349<98<107<2706<3416<3387<811
101002|  %503 = select i1 %500, i64 %501, i64 %502                                                                             ;L3147<7<810<298<349<98<107<2706<3416<3387<811
101003|     ;; dx = i64 %503
101004|  %504 = icmp ult i64 %491, %499                                                                                        ;L3147<8<810<298<349<98<107<2706<3416<3387<811
101005|  %505 = sub nuw i64 %499, %491                                                                                         ;L3147<8<810<298<349<98<107<2706<3416<3387<811
101006|  %506 = sub nuw i64 %491, %499                                                                                         ;L3147<8<810<298<349<98<107<2706<3416<3387<811
101007|  %507 = select i1 %504, i64 %505, i64 %506                                                                             ;L3147<8<810<298<349<98<107<2706<3416<3387<811
101008|     ;; dy = i64 %507
101009|  %508 = mul i64 %503, %503                                                                                             ;L9<810<298<349<98<107<2706<3416<3387<811
101010|  %509 = mul i64 %507, %507                                                                                             ;L9<810<298<349<98<107<2706<3416<3387<811
101011|  %510 = add i64 %509, %508                                                                                             ;L9<810<298<349<98<107<2706<3416<3387<811
101012|  %511 = icmp ult i64 %510, 40000000000                                                                                 ;L810<298<349<98<107<2706<3416<3387<811
101013|  br i1 %511, label %512, label %492                                                                                    ;L349<98<107<2706<3416<3387<811
101014| 
101015| 512: ; preds = %494
101016|  store ptr %496, ptr %11, , !!62172                                                                                    ;L185<348<98<107<2706<3416<3387<811
101017|     ;; first[0..+8] = i64 %510
101018|     ;; first[8..+8] = ptr %495
101019|  %513 = invoke { i64, ptr } @core::iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtNtBc_5slice4iter4IterTyyEENCNvMs1_NtNtCshdEBA0ozCnw_7game_ai12small_action12move_actionsNtB1W_17SmallActionRecall9get_inputs4_0ENCINvNvNtNtNtBa_6traits8iterator8Iterator10min_by_key3keyRB1J_yNCB1Q_s5_0E0EB3z_4foldTyB4n_ENCINvNvB3z_6min_by4foldB4P_INvB3x_7compareB4n_yEE0EB20_(ptr %11, i64 %510, ptr %495)
101020|  to label %533 unwind label %438                                                                                       ;L2707<3416<3387<811
101021| 
101022| 514: ; preds = %475
101023|  invoke void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.afb017e85830050c32e5158c55060899.79) #25
101024|  to label %448 unwind label %438                                                                                       ;L801
101025| 
101026| 515: ; preds = %536, %533, %532, %430
101027|  %516 = phi i1 [ %431, %430 ], [ %429, %536 ], [ %429, %533 ], [ %429, %532 ]
101028|  %517 = phi i64 [ %432, %430 ], [ %428, %536 ], [ %428, %533 ], [ %428, %532 ]
101029|  %518 = phi i64 [ %433, %430 ], [ %427, %536 ], [ %427, %533 ], [ %427, %532 ]
101030|  %519 = phi i64 [ %434, %430 ], [ %426, %536 ], [ %426, %533 ], [ %426, %532 ]
101031|  %520 = phi i64 [ %435, %430 ], [ %425, %536 ], [ %425, %533 ], [ %425, %532 ]
101032|  %521 = phi i64 [ undef, %430 ], [ %539, %536 ], [ undef, %533 ], [ undef, %532 ]
101033|  %522 = phi i64 [ undef, %430 ], [ %537, %536 ], [ undef, %533 ], [ undef, %532 ]
101034|  %523 = phi i1 [ true, %430 ], [ false, %536 ], [ true, %533 ], [ true, %532 ]                                         ;L0
101035|  %524 = phi i64 [ 0, %430 ], [ %485, %536 ], [ %485, %533 ], [ %485, %532 ]                                            ;L802
101036|     ;; b1_home_w_per_unit = i64 %524
101038|     ;; c1_chaser[8..+8] = i64 %522
101039|     ;; c1_chaser[16..+8] = i64 %521
101040|     ;; self = ptr %34
101041|  %525 = load ptr, ptr %34, , !!8                                                                                       ;L742<816
101042|  %526 = icmp eq ptr %525, null                                                                                         ;L742<816
101043|  br i1 %526, label %527, label %540                                                                                    ;L742<816
101044| 
101045| 527: ; preds = %515
101046|  %528 = call i64 @llvm.umin.i64(i64 %333, i64 29)
101047|  %529 = call i64 @llvm.umin.i64(i64 %334, i64 29)
101048|  %530 = mul nuw nsw i64 %529, 30
101049|  %531 = add nuw nsw i64 %530, %528
101050|  br label %559                                                                                                         ;L742<816
101051| 
101052| 532: ; preds = %492, %481
101054|     ;; self = ptr null
101055|  br label %515                                                                                                         ;L2137<812
101056| 
101057| 533: ; preds = %512
101058|  %534 = extractvalue { i64, ptr } %513, 1                                                                              ;L2707<3416<3387<811
101060|     ;; self = ptr %534
101061|  %535 = icmp eq ptr %534, null                                                                                         ;L2137<812
101062|  br i1 %535, label %515, label %536                                                                                    ;L2137<812
101063| 
101064| 536: ; preds = %533
101065|  %537 = load i64, ptr %534, , !!8                                                                                      ;L2138<812
101066|  %538 = gep %534, i64 8                                                                                                ;L2138<812
101067|  %539 = load i64, ptr %538, , !!8                                                                                      ;L2138<812
101068|     ;; c1_chaser[8..+8] = i64 %537
101069|     ;; c1_chaser[16..+8] = i64 %539
101070|     ;; c1_chaser[0..+8] = i64 1
101071|  br label %515                                                                                                         ;L2138<812
101072| 
101073| 540: ; preds = %515
101074|     ;; self = ptr %34
101075|     ;; f[0..+8] = ptr undef
101076|     ;; champ_x = ptr undef
101077|     ;; f[8..+8] = ptr undef
101078|     ;; f[16..+8] = ptr undef
101079|     ;; f[24..+8] = ptr undef
101080|     ;; x = ptr %34
101086|     ;; min = i64 0
101087|     ;; max = i64 29
101089|     ;; min = i64 0
101090|     ;; max = i64 29
101091|  %541 = gep %525, i64 32                                                                                               ;L816<1162<816
101092|  %542 = load i64, ptr %541, , !!62360                                                                                  ;L816<1162<816
101094|     ;; sx = i64 %329
101095|     ;; sy = i64 %332
101100|     ;; self = i64 %334
101101|  %543 = call i64 @llvm.umin.i64(i64 %334, i64 29)                                                                      ;L2025<817<1162<816
101102|     ;; ty = i64 %543
101103|     ;; y = i64 %543
101104|     ;; self = i64 %333
101105|  %544 = call i64 @llvm.umin.i64(i64 %333, i64 29)                                                                      ;L2025<817<1162<816
101106|     ;; tx = i64 %544
101107|     ;; x = i64 %544
101109|  %545 = mul nuw nsw i64 %332, 30                                                                                       ;L31<112<816<1162<816
101110|  %546 = add nuw nsw i64 %545, %329                                                                                     ;L31<112<816<1162<816
101111|  %547 = mul nuw nsw i64 %546, 900                                                                                      ;L112<816<1162<816
101112|  %548 = mul nuw nsw i64 %543, 30                                                                                       ;L31<112<816<1162<816
101113|  %549 = add nuw nsw i64 %548, %544                                                                                     ;L31<112<816<1162<816
101114|  %550 = add nuw nsw i64 %549, %547                                                                                     ;L112<816<1162<816
101115|     ;; index = i64 %550
101116|     ;; index = i64 %550
101117|     ;; self = i64 %550
101120|     ;; self[8..+8] = i64 %542
101121|     ;; slice[8..+8] = i64 %542
101122|  %551 = icmp ult i64 %550, %542                                                                                        ;L272<19<3864<112<816<1162<816
101123|  br i1 %551, label %554, label %552                                                                                    ;L272<19<3864<112<816<1162<816
101124| 
101125| 552: ; preds = %540
101126|  invoke void @core::panicking18panic_bounds_check(i64 %550, i64 %542, ptr @anon.afb017e85830050c32e5158c55060899.26) #25
101127|  to label %553 unwind label %438                                                                                       ;L272<19<3864<112<816<1162<816
101128| 
101129| 553: ; preds = %552
101130|  unreachable                                                                                                           ;L272<19<3864<112<816<1162<816
101131| 
101132| 554: ; preds = %540
101133|  %555 = gep %525, i64 24                                                                                               ;L816<1162<816
101134|  %556 = load ptr, ptr %555, , !!62360, !!8, !!8                                                                        ;L816<1162<816
101135|     ;; self[0..+8] = ptr %556
101136|     ;; slice[0..+8] = ptr %556
101137|  %557 = getelementptr i16, ptr %556, i64 %550                                                                          ;L272<19<3864<112<816<1162<816
101138|  %558 = load i16, ptr %557, , !!62360, !!8                                                                             ;L112<816<1162<816
101139|  br label %559                                                                                                         ;L817<1162<816
101140| 
101141| 559: ; preds = %554, %527
101142|  %560 = phi i64 [ %531, %527 ], [ %549, %554 ]
101143|  %561 = phi i16 [ undef, %527 ], [ %558, %554 ]                                                                        ;L0<816
101145|     ;; c1_home_me[2..+2] = i16 %561
101147|  store ptr inttoptr (i64 8 to ptr), ptr %32,                                                                           ;L547<818
101148|  %562 = gep %32, i64 8                                                                                                 ;L547<818
101149|  store ptr %339, ptr %562,                                                                                             ;L547<818
101150|  %563 = gep %32, i64 16                                                                                                ;L547<818
101151|  %564 = gep %32, i64 24                                                                                                ;L547<818
101152|     ;; iter[0..+8] = i64 0
101153|     ;; iter[8..+8] = i64 7
101154|  %565 = add i64 %336, -3
101155|  %566 = add i64 %338, -3
101156|  %567 = gep %62, i64 120
101157|  %568 = gep %60, i64 24
101158|  %569 = gep %31, i64 40
101159|  %570 = gep %30, i64 8
101160|  %571 = gep %30, i64 16
101161|  %572 = gep %29, i64 40
101162|  %573 = gep %31, i64 48
101163|  %574 = gep %31, i64 49
101164|  %575 = icmp ult i64 %327, %522
101165|  %576 = sub nuw i64 %522, %327
101166|  %577 = sub nuw i64 %327, %522
101167|  %578 = select i1 %575, i64 %576, i64 %577
101168|  %579 = icmp ult i64 %330, %521
101169|  %580 = sub nuw i64 %521, %330
101170|  %581 = sub nuw i64 %330, %521
101171|  %582 = select i1 %579, i64 %580, i64 %581
101172|  %583 = mul i64 %578, %578
101173|  %584 = mul i64 %582, %582
101174|  %585 = add i64 %583, %584
101175|  call void @llvm.memset.p0.i64(ptr %563, i8 0, i64 16, i1 false)                                                       ;L547<818
101176|  br label %586                                                                                                         ;L820
101177| 
101178| 586: ; preds = %784, %559
101179|  %587 = phi i64 [ 0, %559 ], [ %594, %784 ]                                                                            ;L820
101180|     ;; iter[0..+8] = i64 %587
101181|     ;; self = ptr undef
101182|     ;; self = ptr undef
101183|     ;; self = ptr undef
101184|     ;; other = ptr undef
101185|  %588 = icmp samesign ult i64 %587, 7                                                                                  ;L1916<900<985<820
101186|  br i1 %588, label %593, label %589                                                                                    ;L900<985<820
101187| 
101188| 589: ; preds = %586
101189|     ;; self = ptr %32
101190|     ;; self = ptr %32
101191|  %590 = load i64, ptr %564,
101192|  %591 = icmp eq i64 %590, 0                                                                                            ;L877
101193|  %592 = select i1 %75, i1 true, i1 %591                                                                                ;L877
101194|  br i1 %592, label %609, label %613                                                                                    ;L877
101195| 
101196| 593: ; preds = %586
101197|     ;; old = i64 %587
101198|     ;; start = i64 %587
101199|     ;; self = i64 %587
101200|     ;; self = i64 %587
101201|  %594 = add nuw nsw i64 %587, 1                                                                                        ;L2564<2648<682<199<903<985<820
101202|     ;; b = i1 false
101203|     ;; iter[0..+8] = i64 %594
101204|     ;; dx = i64 %587
101205|     ;; iter[0..+8] = i64 0
101206|     ;; iter[8..+8] = i64 7
101207|  %595 = add i64 %565, %587
101208|  %596 = icmp ugt i64 %595, 29
101209|  %597 = getelementptr i64, ptr %567, i64 %595                                                                          ;L821
101210|  %598 = icmp eq i64 %326, %595
101211|  %599 = mul nuw nsw i64 %595, 32000
101212|  %600 = add nuw nsw i64 %599, 16000
101213|  %601 = sub nsw i64 %333, %595
101214|  %602 = call i64 @llvm.abs.i64(i64 %601, i1 true)
101215|  %603 = icmp ult i64 %600, %522
101216|  %604 = sub nuw i64 %522, %600
101217|  %605 = sub nuw nsw i64 %600, %522
101218|  %606 = select i1 %603, i64 %604, i64 %605
101219|  %607 = mul i64 %606, %606
101220|  %608 = and i1 %598, %324                                                                                              ;L821
101221|  br label %784                                                                                                         ;L821
101222| 
101223| 609: ; preds = %620, %589
101224|  %610 = phi i8 [ 1, %589 ], [ 0, %620 ]                                                                                ;L0
101225|     ;; self = ptr %35
101226|     ;; self = ptr %35
101227|  %611 = load i64, ptr %342, , !!8                                                                                      ;L1617<1636<881
101228|  %612 = icmp eq i64 %611, 0                                                                                            ;L1636<881
101229|     ;; picked_real_candidate = i1 %612
101230|     ;; self = ptr %35
101231|     ;; self = ptr %35
101232|  br i1 %612, label %740, label %621                                                                                    ;L882
101233| 
101234| 613: ; preds = %589
101236|  call void @llvm.memcpy.p0.p0.i64(ptr %28, ptr %32, i64 32, i1 false)                                                  ;L878
101238|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %35)
101239|  to label %617 unwind label %614                                                                                       ;L825<878
101240| 
101241| 614: ; preds = %613
101242|  %615 = cleanuppad within none []
101244|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %35) [ "funclet"(token %615) ]
101245|  to label %616 unwind label %618                                                                                       ;L825<825<878
101246| 
101247| 616: ; preds = %614
101248|  cleanupret from %615 unwind label %618
101249| 
101250| 617: ; preds = %613
101252|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %35)
101253|  to label %620 unwind label %618                                                                                       ;L825<825<878
101254| 
101255| 618: ; preds = %617, %616, %614
101256|  %619 = cleanuppad within none []
101257|  call void @llvm.memcpy.p0.p0.i64(ptr %35, ptr %28, i64 32, i1 false)                                                  ;L878
101258|  cleanupret from %619 unwind label %628                                                                                ;L878
101259| 
101260| 620: ; preds = %617
101261|  call void @llvm.memcpy.p0.p0.i64(ptr %35, ptr %28, i64 32, i1 false)                                                  ;L878
101263|  br label %609                                                                                                         ;L877
101264| 
101265| 621: ; preds = %609
101266|     ;; self = ptr %35
101267|     ;; self = ptr %35
101268|  %622 = load ptr, ptr %35, , !!8, !!8                                                                                  ;L138<2083<886
101269|     ;; self[0..+8] = ptr %622
101270|     ;; self[8..+8] = i64 %611
101271|     ;; f = ptr %9
101273|  store ptr %9, ptr %10, , !!62478
101274|     ;; v[0..+8] = ptr %622
101275|     ;; v[0..+8] = ptr %622
101276|     ;; v[8..+8] = i64 %611
101277|     ;; v[8..+8] = i64 %611
101278|     ;; is_less = ptr %10
101279|     ;; is_less = ptr %10
101280|     ;; len = i64 %611
101281|  %623 = icmp eq i64 %611, 1                                                                                            ;L38<860<252<886
101282|     ;; b = i1 %623
101283|  br i1 %623, label %633, label %624                                                                                    ;L436<38<860<252<886
101284| 
101285| 624: ; preds = %621
101286|  %625 = icmp samesign ult i64 %611, 21                                                                                 ;L78<860<252<886
101287|     ;; b = i1 %625
101288|  br i1 %625, label %627, label %626                                                                                    ;L436<78<860<252<886
101289| 
101290| 626: ; preds = %624
101291|  invoke void @core::slice4sort6stable14driftsort_mainTyyxENCINvMNtCs9LexZzt9XJB_5alloc5sliceSBZ_11sort_by_keyxNCNvMs1_NtNtCshdEBA0ozCnw_7game_ai12small_action12move_actionsNtB22_17SmallActionRecall9get_inputs7_0E0INtNtB1c_3vec3VecBZ_EEB26_(ptr %622, i64 %611, ptr %10)
101292|  to label %633 unwind label %628                                                                                       ;L83<860<252<886
101293| 
101294| 627: ; preds = %624
101295|  invoke void @core::slice4sort6shared9smallsort25insertion_sort_shift_leftTyyxENCINvMNtCs9LexZzt9XJB_5alloc5sliceSB1m_11sort_by_keyxNCNvMs1_NtNtCshdEBA0ozCnw_7game_ai12small_action12move_actionsNtB2q_17SmallActionRecall9get_inputs7_0E0EB2u_(ptr %622, i64 %611, i64 1, ptr %10)
101296|  to label %633 unwind label %628                                                                                       ;L79<860<252<886
101297| 
101298| 628: ; preds = %925, %903, %869, %858, %833, %814, %808, %801, %800, %751, %743, %711, %670, %669, %665, %663, %640, %638, %636, %633, %627, %626, %618
101299|  %629 = phi i8 [ 1, %925 ], [ 1, %903 ], [ %610, %633 ], [ %610, %640 ], [ 1, %858 ], [ 1, %869 ], [ %610, %626 ], [ %610, %670 ], [ 1, %833 ], [ %610, %638 ], [ %610, %636 ], [ 1, %814 ], [ %610, %665 ], [ 1, %808 ], [ 1, %800 ], [ 1, %801 ], [ %610, %627 ], [ 0, %618 ], [ %610, %751 ], [ %610, %669 ], [ %610, %743 ], [ %610, %711 ], [ %610, %663 ] ;L0
101300|  %630 = phi i1 [ true, %925 ], [ true, %903 ], [ true, %633 ], [ false, %640 ], [ true, %858 ], [ true, %869 ], [ true, %626 ], [ true, %670 ], [ true, %833 ], [ true, %638 ], [ true, %636 ], [ true, %814 ], [ true, %665 ], [ true, %808 ], [ true, %800 ], [ true, %801 ], [ true, %627 ], [ true, %618 ], [ true, %751 ], [ true, %669 ], [ true, %743 ], [ true, %711 ], [ true, %663 ] ;L0
101301|  %631 = cleanuppad within none []
101302|  %632 = trunc nuw i8 %629 to i1                                                                                        ;L939
101303|  br i1 %632, label %939, label %938                                                                                    ;L939
101304| 
101305| 633: ; preds = %627, %626, %621
101307|  %634 = gep %4, i64 384                                                                                                ;L888
101308|  %635 = invoke i64 @gc::simulation5state6playerNtB4_16AthleteParameter29positioning_runaway_min_range(ptr %634)
101309|  to label %636 unwind label %628                                                                                       ;L888
101310| 
101311| 636: ; preds = %633
101312|  %637 = invoke i64 @gc::simulation5state6playerNtB4_16AthleteParameter29positioning_runaway_max_range(ptr %634)
101313|  to label %638 unwind label %628                                                                                       ;L889
101314| 
101315| 638: ; preds = %636
101316|  %639 = invoke { i64, i64 } @ai::small_action25positioning_choice_window(i64 %2, ptr %4, i64 %635, i64 %637, i1 zeroext true)
101317|  to label %640 unwind label %628                                                                                       ;L887
101318| 
101319| 640: ; preds = %638
101320|  %641 = extractvalue { i64, i64 } %639, 0                                                                              ;L887
101321|     ;; min_range = i64 %641
101323|     ;; self = ptr %35
101324|  %642 = load i64, ptr %342, , !!8                                                                                      ;L1617<890
101325|  %643 = mul i64 %642, %641                                                                                             ;L890
101326|  %644 = udiv i64 %643, 1000                                                                                            ;L890
101327|  %645 = add i64 %642, -1                                                                                               ;L890
101328|     ;; self = i64 %644
101329|     ;; other = i64 %645
101330|  %646 = call i64 @llvm.umin.i64(i64 %645, i64 %644)                                                                    ;L1078<890
101331|  %647 = extractvalue { i64, i64 } %639, 1                                                                              ;L887
101332|     ;; max_range = i64 %647
101333|     ;; min_idx = i64 %646
101334|     ;; self = ptr %35
101335|  %648 = mul i64 %642, %647                                                                                             ;L891
101336|  %649 = udiv i64 %648, 1000                                                                                            ;L891
101337|     ;; self = i64 %649
101338|     ;; min = i64 %646
101339|     ;; max = i64 %645
101340|  %650 = icmp samesign ult i64 %649, %646                                                                               ;L2025<891
101341|  %651 = select i1 %650, i64 %644, i64 %649                                                                             ;L2025<891
101342|  %652 = call i64 @llvm.umin.i64(i64 %651, i64 %645)                                                                    ;L2025<891
101343|     ;; max_idx = i64 %652
101346|  %653 = load ptr, ptr %35, , !!8, !!8                                                                                  ;L893
101348|     ;; begin = ptr %653
101349|     ;; self = ptr %653
101350|     ;; count = i64 %642
101351|  %654 = gepS %653, i64 %642                                                                                            ;L961<2119<893
101352|     ;; self[0..+8] = ptr %653
101353|     ;; self[8..+8] = ptr %654
101354|     ;; self[16..+8] = i64 %646
101355|  %655 = sub nsw i64 %652, %646                                                                                         ;L895
101356|  %656 = add nsw i64 %655, 1                                                                                            ;L895
101357|     ;; n = i64 %656
101358|  store ptr %653, ptr %26,                                                                                              ;L24<1451<895
101359|  %657 = gep %26, i64 8                                                                                                 ;L24<1451<895
101360|  store ptr %654, ptr %657,                                                                                             ;L24<1451<895
101361|  %658 = gep %26, i64 16                                                                                                ;L24<1451<895
101362|  store i64 %646, ptr %658,                                                                                             ;L24<1451<895
101363|  %659 = gep %26, i64 24                                                                                                ;L24<1451<895
101364|  store i64 %656, ptr %659,                                                                                             ;L24<1451<895
101365|  invoke void @core::iter8adapters4take4TakeINtNtB1j_4skip4SkipINtB3_8IntoIterBU_EEEECshdEBA0ozCnw_7game_ai(ptr sret([32 x i8]) %27, ptr %26, ptr %339)
101366|  to label %660 unwind label %628                                                                                       ;L893
101367| 
101368| 660: ; preds = %640
101370|  call void @llvm.memcpy.p0.p0.i64(ptr %35, ptr %27, i64 32, i1 false)                                                  ;L893
101372|     ;; self = ptr %35
101373|     ;; self = ptr %35
101374|     ;; self = ptr %35
101375|     ;; self = ptr %35
101376|  %661 = load ptr, ptr %35, , !!8, !!8                                                                                  ;L138<2073<0
101377|  %662 = load i64, ptr %342, , !!8                                                                                      ;L2075<0
101378|  br i1 %75, label %663, label %665                                                                                     ;L898
101379| 
101380| 663: ; preds = %660
101381|     ;; p = ptr %661
101382|  %664 = invoke ptr @_RINvXNtCsMBkRBYhlca_4rand3seqSTyyxENtB3_11SliceRandom6chooseNtNtNtB5_4rngs3std6StdRngECshdEBA0ozCnw_7game_ai(ptr %661, i64 %662, ptr %3)
101383|  to label %667 unwind label %628                                                                                       ;L901
101384| 
101385| 665: ; preds = %660
101386|     ;; p = ptr %661
101387|  %666 = invoke ptr @_RINvNtCshdEBA0ozCnw_7game_ai12small_action23positioning_window_pickTyyxEEB4_(i64 %2, ptr %3, ptr %4, ptr %5, ptr %661, i64 %662)
101388|  to label %670 unwind label %628                                                                                       ;L899
101389| 
101390| 667: ; preds = %663
101391|     ;; self = ptr %664
101392|  %668 = icmp eq ptr %664, null                                                                                         ;L1011<901
101393|  br i1 %668, label %669, label %670                                                                                    ;L1011<901
101394| 
101395| 669: ; preds = %667
101396|  invoke void @core::option13unwrap_failed(ptr @anon.afb017e85830050c32e5158c55060899.80) #25
101397|  to label %448 unwind label %628                                                                                       ;L1013<901
101398| 
101399| 670: ; preds = %667, %665
101400|  %671 = phi ptr [ %664, %667 ], [ %666, %665 ]                                                                         ;L0
101401|     ;; x = ptr %671
101402|  %672 = gep %671, i64 8                                                                                                ;L898
101403|     ;; y = ptr %672
101404|  %673 = gep %671, i64 16                                                                                               ;L898
101405|     ;; s = ptr %673
101406|  %674 = gep %1, i64 80                                                                                                 ;L903
101407|  %675 = load i64, ptr %674, , !!8                                                                                      ;L903
101408|  %676 = udiv i64 %675, 32000                                                                                           ;L903
101409|     ;; self = i64 %676
101410|     ;; min = i64 0
101411|     ;; max = i64 29
101412|  %677 = call i64 @llvm.umin.i64(i64 %676, i64 29)                                                                      ;L2027<903
101413|     ;; pre_xi = i64 %677
101414|  %678 = gep %1, i64 88                                                                                                 ;L904
101415|  %679 = load i64, ptr %678, , !!8                                                                                      ;L904
101416|  %680 = udiv i64 %679, 32000                                                                                           ;L904
101417|     ;; self = i64 %680
101418|     ;; min = i64 0
101419|     ;; max = i64 29
101420|  %681 = call i64 @llvm.umin.i64(i64 %680, i64 29)                                                                      ;L2027<904
101421|     ;; pre_yi = i64 %681
101423|  invoke void @ai::small_action29positioning_score_at_position(ptr sret([56 x i8]) %25, i64 %2, ptr %4, ptr %5, ptr %6, i64 %675, i64 %679, i8 4)
101424|  to label %682 unwind label %628                                                                                       ;L906
101425| 
101426| 682: ; preds = %670
101427|  %683 = load i64, ptr %25, , !!8                                                                                       ;L907
101428|  %684 = gep %25, i64 40                                                                                                ;L907
101429|  %685 = load i64, ptr %684,                                                                                            ;L907
101432|  %686 = select i1 %105, i64 %685, i64 0                                                                                ;L711<907
101433|  %687 = add i64 %686, %683                                                                                             ;L711<907
101434|     ;; pre_risk = i64 %687
101435|     ;; self = ptr %34
101436|  %688 = load ptr, ptr %34, , !!8                                                                                       ;L742<908
101437|  %689 = icmp eq ptr %688, null                                                                                         ;L742<908
101438|  br i1 %689, label %690, label %703                                                                                    ;L742<908
101439| 
101440| 690: ; preds = %682
101441|  %691 = sub nsw i64 %333, %677                                                                                         ;L914
101442|     ;; self = i64 %691
101443|  %692 = call i64 @llvm.abs.i64(i64 %691, i1 true)                                                                      ;L3648<914
101444|  %693 = sub nsw i64 %334, %681                                                                                         ;L914
101445|     ;; self = i64 %693
101446|  %694 = call i64 @llvm.abs.i64(i64 %693, i1 true)                                                                      ;L3648<914
101447|  %695 = add nuw nsw i64 %694, %692                                                                                     ;L914
101448|     ;; pre_for_nexus = i64 %695
101449|  %696 = mul nsw i64 %695, -10                                                                                          ;L915
101450|  %697 = mul i64 %687, %520                                                                                             ;L915
101451|  %698 = sub i64 %696, %697                                                                                             ;L915
101452|     ;; pre_score = i64 %698
101453|  br label %699                                                                                                         ;L908
101454| 
101455| 699: ; preds = %713, %690
101456|  %700 = phi i64 [ %724, %713 ], [ %698, %690 ]                                                                         ;L0
101457|     ;; pre_score = i64 %700
101458|  %701 = load i64, ptr %673, , !!8                                                                                      ;L917
101459|  %702 = icmp slt i64 %700, %701                                                                                        ;L917
101460|  br i1 %702, label %743, label %725                                                                                    ;L917
101461| 
101462| 703: ; preds = %682
101463|     ;; free = ptr %34
101464|     ;; self = i64 %333
101465|     ;; min = i64 0
101466|     ;; max = i64 29
101467|     ;; self = i64 %334
101468|     ;; min = i64 0
101469|     ;; max = i64 29
101470|  %704 = gep %688, i64 32                                                                                               ;L909
101471|  %705 = load i64, ptr %704,                                                                                            ;L909
101477|     ;; sy = i64 %681
101478|     ;; sx = i64 %677
101480|  %706 = mul nuw nsw i64 %681, 30                                                                                       ;L31<112<909
101481|  %707 = add nuw nsw i64 %706, %677                                                                                     ;L31<112<909
101482|  %708 = mul nuw nsw i64 %707, 900                                                                                      ;L112<909
101483|  %709 = add nuw nsw i64 %560, %708                                                                                     ;L112<909
101484|     ;; index = i64 %709
101485|     ;; index = i64 %709
101486|     ;; self = i64 %709
101489|     ;; self[8..+8] = i64 %705
101490|     ;; slice[8..+8] = i64 %705
101491|  %710 = icmp ult i64 %709, %705                                                                                        ;L272<19<3864<112<909
101492|  br i1 %710, label %713, label %711                                                                                    ;L272<19<3864<112<909
101493| 
101494| 711: ; preds = %703
101495|  invoke void @core::panicking18panic_bounds_check(i64 %709, i64 %705, ptr @anon.afb017e85830050c32e5158c55060899.26) #25
101496|  to label %712 unwind label %628                                                                                       ;L272<19<3864<112<909
101497| 
101498| 712: ; preds = %711
101499|  unreachable                                                                                                           ;L272<19<3864<112<909
101500| 
101501| 713: ; preds = %703
101502|  %714 = gep %688, i64 24                                                                                               ;L909
101503|  %715 = load ptr, ptr %714, , !!8, !!8                                                                                 ;L909
101504|     ;; self[0..+8] = ptr %715
101505|     ;; slice[0..+8] = ptr %715
101506|  %716 = getelementptr i16, ptr %715, i64 %709                                                                          ;L272<19<3864<112<909
101507|  %717 = load i16, ptr %716, , !!8                                                                                      ;L112<909
101508|     ;; d = i16 %717
101509|  %718 = icmp eq i16 %717, -1                                                                                           ;L911
101510|  %719 = select i1 %718, i16 150, i16 %717                                                                              ;L911
101511|     ;; d = i16 %719
101512|  %720 = zext i16 %719 to i64                                                                                           ;L911
101513|     ;; d = i64 %720
101514|  %721 = mul i64 %524, %720                                                                                             ;L912
101515|  %722 = mul i64 %687, %520                                                                                             ;L912
101516|  %723 = add i64 %722, %721                                                                                             ;L912
101517|  %724 = sub i64 0, %723                                                                                                ;L912
101518|     ;; pre_score = i64 %724
101519|  br label %699                                                                                                         ;L908
101520| 
101521| 725: ; preds = %699
101522|     ;; x2 = i64 %675
101523|     ;; other = i64 %675
101524|     ;; y2 = i64 %679
101525|     ;; other = i64 %679
101526|  %726 = icmp ult i64 %327, %675                                                                                        ;L3147<7<917
101527|  %727 = sub nuw i64 %675, %327                                                                                         ;L3147<7<917
101528|  %728 = sub nuw i64 %327, %675                                                                                         ;L3147<7<917
101529|  %729 = select i1 %726, i64 %727, i64 %728                                                                             ;L3147<7<917
101530|     ;; dx = i64 %729
101531|  %730 = icmp ult i64 %330, %679                                                                                        ;L3147<8<917
101532|  %731 = sub nuw i64 %679, %330                                                                                         ;L3147<8<917
101533|  %732 = sub nuw i64 %330, %679                                                                                         ;L3147<8<917
101534|  %733 = select i1 %730, i64 %731, i64 %732                                                                             ;L3147<8<917
101535|     ;; dy = i64 %733
101536|  %734 = mul i64 %729, %729                                                                                             ;L9<917
101537|  %735 = mul i64 %733, %733                                                                                             ;L9<917
101538|  %736 = add i64 %735, %734                                                                                             ;L9<917
101539|  %737 = icmp ugt i64 %736, 143999999                                                                                   ;L917
101540|  %738 = select i1 %737, ptr %674, ptr %671                                                                             ;L917
101541|  %739 = select i1 %737, ptr %678, ptr %672                                                                             ;L917
101542|  br label %743                                                                                                         ;L917
101543| 
101544| 740: ; preds = %609
101545|  %741 = gep %1, i64 80                                                                                                 ;L883
101546|  store i64 %44, ptr %741,                                                                                              ;L883
101547|  %742 = gep %1, i64 88                                                                                                 ;L884
101548|  store i64 %45, ptr %742,                                                                                              ;L884
101549|  store i8 0, ptr %274,                                                                                                 ;L934
101550|  br label %748                                                                                                         ;L928
101551| 
101552| 743: ; preds = %725, %699
101553|  %744 = phi ptr [ %671, %699 ], [ %738, %725 ]
101554|  %745 = phi ptr [ %672, %699 ], [ %739, %725 ]
101555|  %746 = load i64, ptr %745, , !!8                                                                                      ;L0
101556|  %747 = load i64, ptr %744, , !!8                                                                                      ;L0
101557|     ;; gy = i64 %746
101558|     ;; gx = i64 %747
101559|  store i64 %747, ptr %674,                                                                                             ;L923
101560|  store i64 %746, ptr %678,                                                                                             ;L924
101563|  invoke void @ai::small_action29positioning_score_at_position(ptr sret([56 x i8]) %24, i64 %2, ptr %4, ptr %5, ptr %6, i64 %747, i64 %746, i8 4)
101564|  to label %751 unwind label %628                                                                                       ;L929
101565| 
101566| 748: ; preds = %765, %740
101567|  store i64 -1, ptr %210,                                                                                               ;L937
101568|  %749 = gep %1, i64 120                                                                                                ;L938
101569|  store i64 %132, ptr %749,                                                                                             ;L938
101570|  %750 = trunc nuw i8 %610 to i1                                                                                        ;L939
101571|  br i1 %750, label %774, label %767                                                                                    ;L939
101572| 
101573| 751: ; preds = %743
101574|  %752 = load i64, ptr %24, , !!8                                                                                       ;L930
101575|  %753 = gep %24, i64 40                                                                                                ;L930
101576|  %754 = load i64, ptr %753,                                                                                            ;L930
101579|  %755 = select i1 %105, i64 %754, i64 0                                                                                ;L711<930
101580|  %756 = add i64 %755, %752                                                                                             ;L711<930
101581|  %757 = gep %24, i64 48                                                                                                ;L931
101582|  %758 = load i8, ptr %757, , !!8                                                                                       ;L931
101583|  %759 = trunc nuw i8 %758 to i1                                                                                        ;L931
101584|  %760 = gep %24, i64 49                                                                                                ;L931
101585|  %761 = load i8, ptr %760,                                                                                             ;L931
101586|  %762 = trunc nuw i8 %761 to i1                                                                                        ;L931
101587|  %763 = select i1 %759, i1 true, i1 %762                                                                               ;L931
101588|  %764 = invoke i64 @ai::small_action22positioning_risk_value(i64 %2, ptr %4, i64 %756, i1 zeroext %763)
101589|  to label %765 unwind label %628                                                                                       ;L930
101590| 
101591| 765: ; preds = %751
101592|  %766 = gep %1, i64 104                                                                                                ;L930
101593|  store i64 %764, ptr %766,                                                                                             ;L930
101594|  store i8 1, ptr %274,                                                                                                 ;L932
101596|  br label %748                                                                                                         ;L928
101597| 
101598| 767: ; preds = %778, %748
101602|  %768 = load ptr, ptr %34, , !!8                                                                                       ;L825<939
101603|  %769 = icmp eq ptr %768, null                                                                                         ;L825<939
101604|  br i1 %769, label %779, label %770                                                                                    ;L825<939
101605| 
101606| 770: ; preds = %767
101608|     ;; self = ptr %34
101609|     ;; val = i64 1
101610|     ;; order = i8 1
101611|     ;; val = i64 1
101612|     ;; order = i8 1
101613|     ;; self = ptr %768
101614|     ;; dst = ptr %768
101615|  %771 = atomicrmw sub ptr %768, i64 1 release, , !!62742                                                               ;L3956<3193<2831<825<825<939
101616|  %772 = icmp eq i64 %771, 1                                                                                            ;L2831<825<825<939
101617|  br i1 %772, label %773, label %779                                                                                    ;L2831<825<825<939
101618| 
101619| 773: ; preds = %770
101620|     ;; order = i8 2
101621|  fence acquire                                                                                                         ;L4387<64<825<825<939
101622|  invoke void @ai::free_dist11FreeDistMapE9drop_slowBK_(ptr %34)
101623|  to label %779 unwind label %373                                                                                       ;L2874<825<825<939
101624| 
101625| 774: ; preds = %748
101627|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %32)
101628|  to label %778 unwind label %775                                                                                       ;L825<939
101629| 
101630| 775: ; preds = %774
101631|  %776 = cleanuppad within none []
101633|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %32) [ "funclet"(token %776) ]
101634|  to label %777 unwind label %438                                                                                       ;L825<825<939
101635| 
101636| 777: ; preds = %775
101637|  cleanupret from %776 unwind label %438
101638| 
101639| 778: ; preds = %774
101641|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %32)
101642|  to label %767 unwind label %438                                                                                       ;L825<825<939
101643| 
101644| 779: ; preds = %773, %770, %767
101647|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %35)
101648|  to label %782 unwind label %780                                                                                       ;L825<939
101649| 
101650| 780: ; preds = %779
101651|  %781 = cleanuppad within none []
101653|  call void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %35) [ "funclet"(token %781) ]                          ;L825<825<939
101654|  cleanupret from %781 unwind to caller                                                                                 ;L825<939
101655| 
101656| 782: ; preds = %779
101658|  call void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %35)                                                    ;L825<825<939
101660|  %783 = load i64, ptr %40,                                                                                             ;L943
101661|  br label %136                                                                                                         ;L757
101662| 
101663| 784: ; preds = %929, %593
101664|  %785 = phi i64 [ 0, %593 ], [ %788, %929 ]                                                                            ;L821
101665|     ;; iter[0..+8] = i64 %785
101666|     ;; self = ptr undef
101667|     ;; self = ptr undef
101668|     ;; self = ptr undef
101669|     ;; other = ptr undef
101670|  %786 = icmp samesign ult i64 %785, 7                                                                                  ;L1916<900<985<821
101671|  br i1 %786, label %787, label %586                                                                                    ;L900<985<821
101672| 
101673| 787: ; preds = %784
101674|     ;; old = i64 %785
101675|     ;; start = i64 %785
101676|     ;; self = i64 %785
101677|     ;; self = i64 %785
101678|  %788 = add nuw nsw i64 %785, 1                                                                                        ;L2564<2648<682<199<903<985<821
101679|     ;; b = i1 false
101680|     ;; iter[0..+8] = i64 %788
101681|     ;; dy = i64 %785
101682|     ;; xi = i64 %595
101683|  %789 = add i64 %566, %785                                                                                             ;L823
101684|     ;; yi = i64 %789
101685|  %790 = icmp ugt i64 %789, 29                                                                                          ;L824
101686|  %791 = or i1 %596, %790                                                                                               ;L824
101687|  br i1 %791, label %929, label %792                                                                                    ;L824
101688| 
101689| 792: ; preds = %787
101690|  %793 = getelementptr [30 x i64], ptr %597, i64 %789                                                                   ;L824
101691|  %794 = load i64, ptr %793, , !!8                                                                                      ;L824
101692|  %795 = icmp ne i64 %794, 0                                                                                            ;L824
101693|     ;; self = ptr undef
101695|  %796 = icmp eq i64 %325, %789
101696|  %797 = and i1 %796, %608
101697|  %798 = or i1 %795, %797                                                                                               ;L824
101698|  br i1 %798, label %929, label %799                                                                                    ;L824
101699| 
101700| 799: ; preds = %792
101701|  br i1 %516, label %801, label %800                                                                                    ;L833
101702| 
101703| 800: ; preds = %805, %799
101705|  invoke void @ai::small_action25positioning_score_at_cell(ptr sret([56 x i8]) %31, i64 %2, ptr %4, ptr %5, ptr %6, i64 %595, i64 %789, i8 4)
101706|  to label %808 unwind label %628                                                                                       ;L838
101707| 
101708| 801: ; preds = %799
101709|     ;; ex = i64 %517
101710|     ;; ey = i64 %518
101711|     ;; dn = i64 %519
101712|  %802 = mul nuw nsw i64 %789, 32000                                                                                    ;L834
101713|  %803 = add nuw nsw i64 %802, 16000                                                                                    ;L834
101714|  %804 = invoke i64 @gc::utils8distance(i64 %600, i64 %803, i64 %517, i64 %518)
101715|  to label %805 unwind label %628                                                                                       ;L834
101716| 
101717| 805: ; preds = %801
101718|  %806 = add i64 %804, 8000                                                                                             ;L834
101719|  %807 = icmp ult i64 %806, %519                                                                                        ;L834
101720|  br i1 %807, label %929, label %800                                                                                    ;L834
101721| 
101722| 808: ; preds = %800
101724|  %809 = load ptr, ptr %568, , !!8, !!8                                                                                 ;L839
101725|  %810 = gep %809, i64 24                                                                                               ;L839
101726|  invoke void @gc::settingNtB5_7PathMap9find_path(ptr sret([24 x i8]) %30, ptr %810, i64 %329, i64 %332, i64 %595, i64 %789)
101727|  to label %811 unwind label %628                                                                                       ;L839
101728| 
101729| 811: ; preds = %808
101730|  %812 = load i64, ptr %30, , !!8                                                                                       ;L840
101731|  %813 = trunc nuw i64 %812 to i1                                                                                       ;L840
101732|  br i1 %813, label %814, label %817                                                                                    ;L840
101733| 
101734| 814: ; preds = %811
101735|  %815 = load i64, ptr %570, , !!8                                                                                      ;L840
101736|     ;; nx = i64 %815
101737|  %816 = load i64, ptr %571, , !!8                                                                                      ;L840
101738|     ;; ny = i64 %816
101740|  invoke void @ai::small_action25positioning_score_at_cell(ptr sret([56 x i8]) %29, i64 %2, ptr %4, ptr %5, ptr %6, i64 %815, i64 %816, i8 4)
101741|  to label %822 unwind label %628                                                                                       ;L841
101742| 
101743| 817: ; preds = %811
101744|  %818 = load i64, ptr %31, , !!8                                                                                       ;L844
101745|  %819 = load i64, ptr %569,                                                                                            ;L844
101748|  %820 = select i1 %105, i64 %819, i64 0                                                                                ;L711<844
101749|  %821 = add i64 %820, %818                                                                                             ;L711<844
101750|  br label %833                                                                                                         ;L711<844
101751| 
101752| 822: ; preds = %814
101753|  %823 = load i64, ptr %29, , !!8                                                                                       ;L842
101754|  %824 = load i64, ptr %572,                                                                                            ;L842
101757|  %825 = select i1 %105, i64 %824, i64 0                                                                                ;L711<842
101758|  %826 = load i64, ptr %31, , !!8                                                                                       ;L842
101759|  %827 = load i64, ptr %569,                                                                                            ;L842
101762|  %828 = select i1 %105, i64 %827, i64 0                                                                                ;L711<842
101763|  %829 = add i64 %825, %823                                                                                             ;L711<842
101764|  %830 = add i64 %829, %826                                                                                             ;L711<842
101765|  %831 = add i64 %830, %828                                                                                             ;L842
101766|  %832 = sdiv i64 %831, 2                                                                                               ;L842
101767|     ;; risk = i64 %832
101769|  br label %833                                                                                                         ;L840
101770| 
101771| 833: ; preds = %822, %817
101772|  %834 = phi i64 [ %832, %822 ], [ %821, %817 ]                                                                         ;L0
101773|     ;; risk = i64 %834
101774|  %835 = load i8, ptr %573, , !!8                                                                                       ;L847
101775|  %836 = trunc nuw i8 %835 to i1                                                                                        ;L847
101776|  %837 = load i8, ptr %574,                                                                                             ;L847
101777|  %838 = trunc nuw i8 %837 to i1                                                                                        ;L847
101778|  %839 = select i1 %836, i1 true, i1 %838                                                                               ;L847
101779|  %840 = invoke i64 @ai::small_action22positioning_risk_value(i64 %2, ptr %4, i64 %834, i1 zeroext %839)
101780|  to label %841 unwind label %628                                                                                       ;L846
101781| 
101782| 841: ; preds = %833
101783|     ;; risk = i64 %840
101784|     ;; self = ptr %34
101785|  %842 = load ptr, ptr %34, , !!8                                                                                       ;L742<849
101786|  %843 = icmp eq ptr %842, null                                                                                         ;L742<849
101787|  br i1 %843, label %844, label %861                                                                                    ;L742<849
101788| 
101789| 844: ; preds = %841
101790|     ;; self = i64 %601
101791|  %845 = sub nsw i64 %334, %789                                                                                         ;L854
101792|     ;; self = i64 %845
101793|  %846 = call i64 @llvm.abs.i64(i64 %845, i1 true)                                                                      ;L3648<854
101794|  %847 = add nuw nsw i64 %846, %602                                                                                     ;L854
101795|     ;; for_nexus = i64 %847
101796|  %848 = mul nsw i64 %847, -10                                                                                          ;L855
101797|  %849 = mul i64 %840, %520                                                                                             ;L855
101798|  %850 = sub i64 %848, %849                                                                                             ;L855
101799|     ;; score = i64 %850
101800|  br label %851                                                                                                         ;L849
101801| 
101802| 851: ; preds = %871, %844
101803|  %852 = phi i64 [ %882, %871 ], [ %850, %844 ]                                                                         ;L0
101804|     ;; score = i64 %852
101805|     ;; wx = i64 %600
101806|     ;; x1 = i64 %600
101807|     ;; self = i64 %600
101808|  %853 = mul nuw nsw i64 %789, 32000                                                                                    ;L858
101809|  %854 = add nuw nsw i64 %853, 16000                                                                                    ;L858
101810|     ;; wy = i64 %854
101811|     ;; y1 = i64 %854
101812|     ;; self = i64 %854
101813|     ;; value[0..+8] = i64 %600
101814|     ;; src[0..+8] = i64 %600
101815|     ;; value[8..+8] = i64 %854
101816|     ;; src[8..+8] = i64 %854
101817|     ;; value[16..+8] = i64 %852
101818|     ;; src[16..+8] = i64 %852
101819|     ;; self = ptr %35
101820|     ;; self = ptr %35
101821|     ;; additional = i64 1
101822|     ;; needed_extra_cap = i64 1
101823|     ;; needed_extra_cap = i64 1
101824|     ;; strategy = i8 1
101825|  %855 = load i64, ptr %342, , !!62882, !!8                                                                             ;L1428<859
101826|     ;; self = ptr %35
101827|  %856 = load i64, ptr %341, , !!62882, !!8                                                                             ;L149<1428<859
101828|  %857 = icmp eq i64 %855, %856                                                                                         ;L1428<859
101829|  br i1 %857, label %858, label %883                                                                                    ;L1428<859
101830| 
101831| 858: ; preds = %851
101832|     ;; self = ptr %35
101833|     ;; self = ptr %35
101834|     ;; self = ptr %35
101835|     ;; used_cap = i64 %855
101836|     ;; used_cap = i64 %855
101837|  invoke void @_RNvMs2_NtNtCshWfHDMLkPaX_7bumpalo11collections7raw_vecINtB5_6RawVecTyyxEE25reserve_internal_or_panicCshdEBA0ozCnw_7game_ai(ptr %35, i64 %855, i64 1, i1 zeroext true)
101838|  to label %859 unwind label %628                                                                                       ;L619<430<738<1429<859
101839| 
101840| 859: ; preds = %858
101841|  %860 = load i64, ptr %342, , !!62882                                                                                  ;L1432<859
101842|  br label %883                                                                                                         ;L1428<859
101843| 
101844| 861: ; preds = %841
101845|     ;; free = ptr %34
101846|     ;; self = i64 %333
101847|     ;; min = i64 0
101848|     ;; max = i64 29
101849|     ;; self = i64 %334
101850|     ;; min = i64 0
101851|     ;; max = i64 29
101852|  %862 = gep %842, i64 32                                                                                               ;L850
101853|  %863 = load i64, ptr %862,                                                                                            ;L850
101855|     ;; sx = i64 %595
101856|     ;; sy = i64 %789
101862|  %864 = mul nuw nsw i64 %789, 30                                                                                       ;L31<112<850
101863|  %865 = add nuw nsw i64 %864, %595                                                                                     ;L31<112<850
101864|  %866 = mul nuw nsw i64 %865, 900                                                                                      ;L112<850
101865|  %867 = add nuw nsw i64 %560, %866                                                                                     ;L112<850
101866|     ;; index = i64 %867
101867|     ;; index = i64 %867
101868|     ;; self = i64 %867
101871|     ;; self[8..+8] = i64 %863
101872|     ;; slice[8..+8] = i64 %863
101873|  %868 = icmp ult i64 %867, %863                                                                                        ;L272<19<3864<112<850
101874|  br i1 %868, label %871, label %869                                                                                    ;L272<19<3864<112<850
101875| 
101876| 869: ; preds = %861
101877|  invoke void @core::panicking18panic_bounds_check(i64 %867, i64 %863, ptr @anon.afb017e85830050c32e5158c55060899.26) #25
101878|  to label %870 unwind label %628                                                                                       ;L272<19<3864<112<850
101879| 
101880| 870: ; preds = %869
101881|  unreachable                                                                                                           ;L272<19<3864<112<850
101882| 
101883| 871: ; preds = %861
101884|  %872 = gep %842, i64 24                                                                                               ;L850
101885|  %873 = load ptr, ptr %872, , !!8, !!8                                                                                 ;L850
101886|     ;; self[0..+8] = ptr %873
101887|     ;; slice[0..+8] = ptr %873
101888|  %874 = getelementptr i16, ptr %873, i64 %867                                                                          ;L272<19<3864<112<850
101889|  %875 = load i16, ptr %874, , !!8                                                                                      ;L112<850
101890|     ;; d = i16 %875
101891|  %876 = icmp eq i16 %875, -1                                                                                           ;L851
101892|  %877 = select i1 %876, i16 150, i16 %875                                                                              ;L851
101893|     ;; d = i16 %877
101894|  %878 = zext i16 %877 to i64                                                                                           ;L851
101895|     ;; d = i64 %878
101896|  %879 = mul i64 %524, %878                                                                                             ;L852
101897|  %880 = mul i64 %840, %520                                                                                             ;L852
101898|  %881 = add i64 %880, %879                                                                                             ;L852
101899|  %882 = sub i64 0, %881                                                                                                ;L852
101900|     ;; score = i64 %882
101901|  br label %851                                                                                                         ;L849
101902| 
101903| 883: ; preds = %859, %851
101904|  %884 = phi i64 [ %855, %851 ], [ %860, %859 ]                                                                         ;L1432<859
101905|     ;; self = ptr %35
101906|  %885 = load ptr, ptr %35, , !!62882, !!8, !!8                                                                         ;L138<1432<859
101907|     ;; self = ptr %885
101908|     ;; count = i64 %884
101909|  %886 = gepS %885, i64 %884                                                                                            ;L961<1432<859
101910|     ;; end = ptr %886
101911|     ;; dst = ptr %886
101912|  store i64 %600, ptr %886,                                                                                             ;L1933<1433<859
101913|  %887 = gep %886, i64 8                                                                                                ;L1933<1433<859
101914|  store i64 %854, ptr %887,                                                                                             ;L1933<1433<859
101915|  %888 = gep %886, i64 16                                                                                               ;L1933<1433<859
101916|  store i64 %852, ptr %888,                                                                                             ;L1933<1433<859
101917|  %889 = load i64, ptr %342, , !!62882, !!8                                                                             ;L1434<859
101918|  %890 = add i64 %889, 1                                                                                                ;L1434<859
101919|  store i64 %890, ptr %342, , !!62882                                                                                   ;L1434<859
101920|     ;; self = ptr %34
101921|  %891 = load ptr, ptr %34, , !!8                                                                                       ;L742<861
101922|  %892 = icmp eq ptr %891, null                                                                                         ;L742<861
101923|  %893 = or i1 %523, %892
101924|  %894 = or i1 %893, %526
101925|  br i1 %894, label %921, label %895                                                                                    ;L742<861
101926| 
101927| 895: ; preds = %883
101928|     ;; ex = i64 %522
101929|     ;; x2 = i64 %522
101930|     ;; other = i64 %522
101931|     ;; other = i64 %522
101932|     ;; ey = i64 %521
101933|     ;; y2 = i64 %521
101934|     ;; other = i64 %521
101935|     ;; other = i64 %521
101936|     ;; free = ptr %34
101937|     ;; home_me = i16 %561
101938|     ;; self = i64 %333
101939|     ;; min = i64 0
101940|     ;; max = i64 29
101941|     ;; self = i64 %334
101942|     ;; min = i64 0
101943|     ;; max = i64 29
101944|  %896 = gep %891, i64 32                                                                                               ;L863
101945|  %897 = load i64, ptr %896,                                                                                            ;L863
101947|     ;; sx = i64 %595
101948|     ;; sy = i64 %789
101954|  %898 = mul nuw nsw i64 %789, 30                                                                                       ;L31<112<863
101955|  %899 = add nuw nsw i64 %898, %595                                                                                     ;L31<112<863
101956|  %900 = mul nuw nsw i64 %899, 900                                                                                      ;L112<863
101957|  %901 = add nuw nsw i64 %560, %900                                                                                     ;L112<863
101958|     ;; index = i64 %901
101959|     ;; index = i64 %901
101960|     ;; self = i64 %901
101963|     ;; self[8..+8] = i64 %897
101964|     ;; slice[8..+8] = i64 %897
101965|  %902 = icmp ult i64 %901, %897                                                                                        ;L272<19<3864<112<863
101966|  br i1 %902, label %905, label %903                                                                                    ;L272<19<3864<112<863
101967| 
101968| 903: ; preds = %895
101969|  invoke void @core::panicking18panic_bounds_check(i64 %901, i64 %897, ptr @anon.afb017e85830050c32e5158c55060899.26) #25
101970|  to label %904 unwind label %628                                                                                       ;L272<19<3864<112<863
101971| 
101972| 904: ; preds = %903
101973|  unreachable                                                                                                           ;L272<19<3864<112<863
101974| 
101975| 905: ; preds = %895
101976|  %906 = gep %891, i64 24                                                                                               ;L863
101977|  %907 = load ptr, ptr %906, , !!8, !!8                                                                                 ;L863
101978|     ;; self[0..+8] = ptr %907
101979|     ;; slice[0..+8] = ptr %907
101980|  %908 = getelementptr i16, ptr %907, i64 %901                                                                          ;L272<19<3864<112<863
101981|  %909 = load i16, ptr %908, , !!8                                                                                      ;L112<863
101982|     ;; home_cand = i16 %909
101983|     ;; dx = i64 %606
101984|  %910 = icmp ult i64 %854, %521                                                                                        ;L3147<8<864
101985|  %911 = sub nuw i64 %521, %854                                                                                         ;L3147<8<864
101986|  %912 = sub nuw nsw i64 %854, %521                                                                                     ;L3147<8<864
101987|  %913 = select i1 %910, i64 %911, i64 %912                                                                             ;L3147<8<864
101988|     ;; dy = i64 %913
101989|  %914 = mul i64 %913, %913                                                                                             ;L9<864
101990|  %915 = add i64 %914, %607                                                                                             ;L9<864
101991|  %916 = icmp ult i64 %915, %585                                                                                        ;L864
101992|  %917 = icmp ne i16 %909, -1
101993|  %918 = and i1 %916, %917
101994|  %919 = icmp ugt i16 %909, %561
101995|     ;; c1_worse_both = i1 %919
101996|  %920 = and i1 %919, %918                                                                                              ;L864
101997|  br i1 %920, label %928, label %921                                                                                    ;L864
101998| 
101999| 921: ; preds = %905, %883
102000|     ;; value[0..+8] = i64 %600
102001|     ;; src[0..+8] = i64 %600
102002|     ;; value[8..+8] = i64 %854
102003|     ;; src[8..+8] = i64 %854
102004|     ;; value[16..+8] = i64 %852
102005|     ;; src[16..+8] = i64 %852
102006|     ;; self = ptr %32
102007|     ;; self = ptr %32
102008|     ;; additional = i64 1
102009|     ;; needed_extra_cap = i64 1
102010|     ;; needed_extra_cap = i64 1
102011|     ;; strategy = i8 1
102012|  %922 = load i64, ptr %564, , !!62977, !!8                                                                             ;L1428<871
102013|     ;; self = ptr %32
102014|  %923 = load i64, ptr %563, , !!62977, !!8                                                                             ;L149<1428<871
102015|  %924 = icmp eq i64 %922, %923                                                                                         ;L1428<871
102016|  br i1 %924, label %925, label %930                                                                                    ;L1428<871
102017| 
102018| 925: ; preds = %921
102019|     ;; self = ptr %32
102020|     ;; self = ptr %32
102021|     ;; self = ptr %32
102022|     ;; used_cap = i64 %922
102023|     ;; used_cap = i64 %922
102024|  invoke void @_RNvMs2_NtNtCshWfHDMLkPaX_7bumpalo11collections7raw_vecINtB5_6RawVecTyyxEE25reserve_internal_or_panicCshdEBA0ozCnw_7game_ai(ptr %32, i64 %922, i64 1, i1 zeroext true)
102025|  to label %926 unwind label %628                                                                                       ;L619<430<738<1429<871
102026| 
102027| 926: ; preds = %925
102028|  %927 = load i64, ptr %564, , !!62977                                                                                  ;L1432<871
102029|  br label %930                                                                                                         ;L1428<871
102030| 
102031| 928: ; preds = %930, %905
102034|  br label %929                                                                                                         ;L821
102035| 
102036| 929: ; preds = %928, %805, %792, %787
102037|  br label %784                                                                                                         ;L1916<900<985<821
102038| 
102039| 930: ; preds = %926, %921
102040|  %931 = phi i64 [ %922, %921 ], [ %927, %926 ]                                                                         ;L1432<871
102041|     ;; self = ptr %32
102042|  %932 = load ptr, ptr %32, , !!62977, !!8, !!8                                                                         ;L138<1432<871
102043|     ;; self = ptr %932
102044|     ;; count = i64 %931
102045|  %933 = gepS %932, i64 %931                                                                                            ;L961<1432<871
102046|     ;; end = ptr %933
102047|     ;; dst = ptr %933
102048|  store i64 %600, ptr %933,                                                                                             ;L1933<1433<871
102049|  %934 = gep %933, i64 8                                                                                                ;L1933<1433<871
102050|  store i64 %854, ptr %934,                                                                                             ;L1933<1433<871
102051|  %935 = gep %933, i64 16                                                                                               ;L1933<1433<871
102052|  store i64 %852, ptr %935,                                                                                             ;L1933<1433<871
102053|  %936 = load i64, ptr %564, , !!62977, !!8                                                                             ;L1434<871
102054|  %937 = add i64 %936, 1                                                                                                ;L1434<871
102055|  store i64 %937, ptr %564, , !!62977                                                                                   ;L1434<871
102056|  br label %928                                                                                                         ;L870
102057| 
102058| 938: ; preds = %628
102059|  cleanupret from %631 unwind label %438
102060| 
102061| 939: ; preds = %628
102062|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecTyyxEEECshdEBA0ozCnw_7game_ai(ptr %32) #26 [ "funclet"(token %631) ] ;L939
102063|  cleanupret from %631 unwind label %438                                                                                ;L939
102064| 
102065| 940: ; preds = %473
102067|     ;; e = ptr %472
102068|     ;; self = ptr %472
102069|  %941 = invoke zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %461, ptr %127, ptr %129, ptr %4, ptr %472)
102070|  to label %942 unwind label %438                                                                                       ;L792
102071| 
102072| 942: ; preds = %940
102073|  br i1 %941, label %943, label %960                                                                                    ;L792
102074| 
102075| 943: ; preds = %942
102076|  %944 = gep %472, i64 1632                                                                                             ;L2158<795
102077|  %945 = load i64, ptr %944, , !!8                                                                                      ;L2158<795
102078|     ;; x1 = i64 %945
102079|     ;; self = i64 %945
102080|  %946 = gep %472, i64 1640                                                                                             ;L2158<795
102081|  %947 = load i64, ptr %946, , !!8                                                                                      ;L2158<795
102082|     ;; y1 = i64 %947
102083|     ;; self = i64 %947
102084|  %948 = icmp ult i64 %945, %327                                                                                        ;L3147<7<2158<795
102085|  %949 = sub nuw i64 %327, %945                                                                                         ;L3147<7<2158<795
102086|  %950 = sub nuw i64 %945, %327                                                                                         ;L3147<7<2158<795
102087|  %951 = select i1 %948, i64 %949, i64 %950                                                                             ;L3147<7<2158<795
102088|     ;; dx = i64 %951
102089|  %952 = icmp ult i64 %947, %330                                                                                        ;L3147<8<2158<795
102090|  %953 = sub nuw i64 %330, %947                                                                                         ;L3147<8<2158<795
102091|  %954 = sub nuw i64 %947, %330                                                                                         ;L3147<8<2158<795
102092|  %955 = select i1 %952, i64 %953, i64 %954                                                                             ;L3147<8<2158<795
102093|     ;; dy = i64 %955
102094|  %956 = mul i64 %951, %951                                                                                             ;L9<2158<795
102095|  %957 = mul i64 %955, %955                                                                                             ;L9<2158<795
102096|  %958 = add i64 %957, %956                                                                                             ;L9<2158<795
102097|  %959 = icmp ugt i64 %958, 40000000000                                                                                 ;L795
102098|  br i1 %959, label %960, label %961                                                                                    ;L795
102099| 
102100| 960: ; preds = %943, %942
102101|  br label %464                                                                                                         ;L1
102102| 
102103| 961: ; preds = %943
102104|  %962 = invoke i64 @ai::fight_check9fight_dps(i64 %2, ptr %60, ptr %472, ptr %56)
102105|  to label %963 unwind label %438                                                                                       ;L799
102106| 
102107| 963: ; preds = %961
102108|     ;; self = i64 %962
102109|     ;; self = i64 %962
102110|     ;; self = i64 %962
102111|  %964 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %962, i64 %453)                                              ;L3178<1288<2517<799
102112|  %965 = extractvalue { i64, i1 } %964, 0                                                                               ;L3178<1288<2517<799
102113|  %966 = extractvalue { i64, i1 } %964, 1                                                                               ;L3178<1288<2517<799
102114|     ;; b = i1 %966
102115|     ;; b = i1 %966
102116|     ;; a = i64 %965
102117|     ;; rhs = i64 %965
102118|  br i1 %966, label %967, label %968                                                                                    ;L459<1289<2517<799
102119| 
102120| 967: ; preds = %963
102121|     ;; a = i64 -1
102122|     ;; rhs = i64 -1
102123|  br label %968                                                                                                         ;L2519<799
102124| 
102125| 968: ; preds = %967, %963
102126|  %969 = phi i64 [ -1, %967 ], [ %965, %963 ]                                                                           ;L0<799
102127|     ;; rhs = i64 %969
102128|     ;; a = i64 %969
102129|  %970 = call i64 @llvm.uadd.sat.i64(i64 %463, i64 %969)                                                                ;L2428<798
102130|     ;; edge_dmg_milli = i64 %970
102131|     ;; self = i64 %970
102132|  br label %462                                                                                                         ;L791
102133| 
102134| 971: ; preds = %373
102135|  cleanupret from %375 unwind to caller
102136| 
102137| 972: ; preds = %373
102138|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecTyyxEEECshdEBA0ozCnw_7game_ai(ptr %35) #26 [ "funclet"(token %375) ] ;L939
102139|  cleanupret from %375 unwind to caller                                                                                 ;L939
102140| 
102141| 973: ; preds = %995, %979
102142|  call void @llvm.memcpy.p0.p0.i64(ptr %1, ptr %16, i64 72, i1 false)                                                   ;L956
102144|  %974 = load i8, ptr %145,                                                                                             ;L764<985
102145|     ;; self = ptr %1
102146|  %975 = icmp eq i8 %974, 2                                                                                             ;L764<985
102147|  br i1 %975, label %1016, label %976                                                                                   ;L764<985
102148| 
102149| 976: ; preds = %973
102150|  %977 = load i64, ptr %138,                                                                                            ;L987
102151|  %978 = load i64, ptr %140,                                                                                            ;L987
102152|  br label %1001                                                                                                        ;L764<985
102153| 
102154| 979: ; preds = %136
102155|  %980 = icmp ult i64 %137, 2                                                                                           ;L948
102156|     ;; policy[4..+1] = i1 %980
102159|  store ptr %23, ptr %15,                                                                                               ;L956
102160|  %981 = gep %15, i64 8                                                                                                 ;L956
102161|  store ptr %40, ptr %981,                                                                                              ;L956
102162|  %982 = gep %15, i64 16                                                                                                ;L956
102163|  store ptr %4, ptr %982,                                                                                               ;L956
102164|  %983 = gep %15, i64 24                                                                                                ;L956
102165|  store ptr %5, ptr %983,                                                                                               ;L956
102166|  %984 = gep %15, i64 32                                                                                                ;L956
102167|  store ptr %6, ptr %984,                                                                                               ;L956
102168|  %985 = gep %15, i64 40                                                                                                ;L956
102169|  store ptr %39, ptr %985,                                                                                              ;L956
102170|  %986 = gep %15, i64 48                                                                                                ;L956
102171|  store ptr %18, ptr %986,                                                                                              ;L956
102172|  %987 = gep %15, i64 56                                                                                                ;L956
102173|  store ptr %138, ptr %987,                                                                                             ;L956
102174|  %988 = gep %15, i64 64                                                                                                ;L956
102175|  store ptr %140, ptr %988,                                                                                             ;L956
102176|  %989 = gep %15, i64 72                                                                                                ;L956
102177|  store ptr %17, ptr %989,                                                                                              ;L956
102178|  %990 = gep %15, i64 80                                                                                                ;L956
102179|  store ptr %21, ptr %990,                                                                                              ;L956
102180|  %991 = gep %15, i64 88                                                                                                ;L956
102181|  store ptr %20, ptr %991,                                                                                              ;L956
102182|  %992 = gep %15, i64 96                                                                                                ;L956
102183|  store ptr %22, ptr %992,                                                                                              ;L956
102184|  call void @_RINvMNtCshdEBA0ozCnw_7game_ai11path_finderNtB3_10PathFinder22new_target_with_policyNCNvMs1_NtNtB5_12small_action12move_actionsNtB1r_17SmallActionRecall9get_inputs8_0EB5_(ptr sret([72 x i8]) %16, ptr %3, ptr %60, i64 %137, ptr @anon.afb017e85830050c32e5158c55060899.72, i64 17, i32 %142, i1 zeroext %980, i64 %69, i64 %107, i64 %139, i64 %141, ptr %15) ;L956
102188|  %993 = load i8, ptr %145, , !!8                                                                                       ;L825<956
102189|  %994 = icmp eq i8 %993, 2                                                                                             ;L825<956
102190|  br i1 %994, label %973, label %995                                                                                    ;L825<956
102191| 
102192| 995: ; preds = %979
102193|  %996 = gep %1, i64 40                                                                                                 ;L825<956
102194|  %997 = load ptr, ptr %996, , !!8, !!8                                                                                 ;L825<956
102195|  %998 = gep %1, i64 48                                                                                                 ;L825<956
102196|  %999 = load ptr, ptr %998,                                                                                            ;L825<956
102200|     ;; ptr = ptr %997
102201|     ;; layout[0..+8] = i64 8
102202|     ;; layout[8..+8] = i64 1120
102205|     ;; ptr = ptr %997
102206|     ;; ptr = ptr %997
102207|     ;; ptr = ptr %997
102208|     ;; ptr = ptr %997
102209|     ;; layout[0..+8] = i64 8
102210|     ;; layout[0..+8] = i64 8
102211|     ;; layout[0..+8] = i64 8
102212|     ;; layout[0..+8] = i64 8
102213|     ;; layout[8..+8] = i64 1120
102214|     ;; layout[8..+8] = i64 1120
102215|     ;; layout[8..+8] = i64 1120
102216|     ;; layout[8..+8] = i64 1120
102217|  call void @_RNvCseLSQwpavqd5_7___rustc14___rust_dealloc(ptr %997, i64 1120, i64 8) #24, !!63025                       ;L128<229<344<462<1956<825<825<825<956
102218|  %1000 = icmp ne ptr %999, null
102219|  call void @llvm.assume(i1 %1000)
102222|     ;; ptr = ptr %999
102223|     ;; layout[0..+8] = i64 1
102224|     ;; layout[8..+8] = i64 70
102227|     ;; ptr = ptr %999
102228|     ;; ptr = ptr %999
102229|     ;; ptr = ptr %999
102230|     ;; ptr = ptr %999
102231|     ;; layout[0..+8] = i64 1
102232|     ;; layout[0..+8] = i64 1
102233|     ;; layout[0..+8] = i64 1
102234|     ;; layout[0..+8] = i64 1
102235|     ;; layout[8..+8] = i64 70
102236|     ;; layout[8..+8] = i64 70
102237|     ;; layout[8..+8] = i64 70
102238|     ;; layout[8..+8] = i64 70
102239|  call void @_RNvCseLSQwpavqd5_7___rustc14___rust_dealloc(ptr %999, i64 70, i64 1) #24, !!63025                         ;L128<229<344<462<1956<825<825<825<956
102240|  br label %973                                                                                                         ;L825<956
102241| 
102242| 1001: ; preds = %976, %136
102243|  %1002 = phi i64 [ %978, %976 ], [ %141, %136 ]                                                                        ;L987
102244|  %1003 = phi i64 [ %977, %976 ], [ %139, %136 ]                                                                        ;L987
102245|     ;; p = ptr %1
102247|  store ptr %23, ptr %14,                                                                                               ;L987
102248|  %1004 = gep %14, i64 8                                                                                                ;L987
102249|  store ptr %40, ptr %1004,                                                                                             ;L987
102250|  %1005 = gep %14, i64 16                                                                                               ;L987
102251|  store ptr %4, ptr %1005,                                                                                              ;L987
102252|  %1006 = gep %14, i64 24                                                                                               ;L987
102253|  store ptr %5, ptr %1006,                                                                                              ;L987
102254|  %1007 = gep %14, i64 32                                                                                               ;L987
102255|  store ptr %6, ptr %1007,                                                                                              ;L987
102256|  %1008 = gep %14, i64 40                                                                                               ;L987
102257|  store ptr %39, ptr %1008,                                                                                             ;L987
102258|  %1009 = gep %14, i64 48                                                                                               ;L987
102259|  store ptr %18, ptr %1009,                                                                                             ;L987
102260|  %1010 = gep %14, i64 56                                                                                               ;L987
102261|  store ptr %138, ptr %1010,                                                                                            ;L987
102262|  %1011 = gep %14, i64 64                                                                                               ;L987
102263|  store ptr %140, ptr %1011,                                                                                            ;L987
102264|  %1012 = gep %14, i64 72                                                                                               ;L987
102265|  store ptr %17, ptr %1012,                                                                                             ;L987
102266|  %1013 = gep %14, i64 80                                                                                               ;L987
102267|  store ptr %21, ptr %1013,                                                                                             ;L987
102268|  %1014 = gep %14, i64 88                                                                                               ;L987
102269|  store ptr %20, ptr %1014,                                                                                             ;L987
102270|  %1015 = gep %14, i64 96                                                                                               ;L987
102271|  store ptr %22, ptr %1015,                                                                                             ;L987
102272|  call void @_RINvMNtCshdEBA0ozCnw_7game_ai11path_finderNtB3_10PathFinder11update_pathNCNvMs1_NtNtB5_12small_action12move_actionsNtB1g_17SmallActionRecall9get_inputs9_0EB5_(ptr %1, ptr %3, ptr %60, i64 %69, i64 %107, i64 %1003, i64 %1002, ptr %14) ;L987
102274|  call void @ai::path_finderNtB2_10PathFinder9get_input(ptr sret([32 x i8]) %0, ptr %1, ptr %4, ptr %5, i8 2, i1 zeroext false) ;L1015
102283|  br label %1018                                                                                                        ;L1016
102284| 
102285| 1016: ; preds = %973
102286|  store i64 -1, ptr %0,                                                                                                 ;L2790<985
102293|  br label %1017                                                                                                        ;L1
102294| 
102295| 1017: ; preds = %1016, %125
102298|  br label %1018                                                                                                        ;L1
102299| 
102300| 1018: ; preds = %1017, %1001, %87
102301|  ret void                                                                                                              ;L1016
102302| }
