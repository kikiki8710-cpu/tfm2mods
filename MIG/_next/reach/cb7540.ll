 20312| define void @ai::plan_legacy8sub_plan4hideNtB2_11HideSubPlan17action_candidates(ptr sret([32 x i8]) %0, ptr %1, i64 %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr readnone %7) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
 20313|  %9 = alloca [24 x i8],
 20314|  %10 = alloca [184 x i8],
 20315|  %11 = alloca [24 x i8],
 20316|  %12 = alloca [184 x i8],
 20317|  %13 = alloca [24 x i8],
 20318|  %14 = alloca [184 x i8],
 20320|     ;; self[16..+1136] = ptr @anon.94acafa22d01e083ca1cc62f01598c8f.148
 20321|  %15 = alloca [1168 x i8],
 20322|  %16 = alloca [32 x i8],
 20323|  %17 = alloca [72 x i8],
 20324|  %18 = alloca [32 x i8],
 20325|  %19 = alloca [32 x i8],
 20326|  %20 = alloca [32 x i8],
 20327|  %21 = alloca [136 x i8],
 20328|  %22 = alloca [184 x i8],
 20329|  %23 = alloca [32 x i8],
 20330|  %24 = alloca [136 x i8],
 20331|  %25 = alloca [184 x i8],
 20332|  %26 = alloca [32 x i8],
 20333|  %27 = alloca [120 x i8],
 20334|  %28 = alloca [184 x i8],
 20335|  %29 = alloca [120 x i8],
 20336|  %30 = alloca [184 x i8],
 20337|  %31 = alloca [136 x i8],
 20338|  %32 = alloca [184 x i8],
 20339|  %33 = alloca [32 x i8],
 20340|  %34 = alloca [136 x i8],
 20341|  %35 = alloca [184 x i8],
 20342|  %36 = alloca [184 x i8],
 20343|  %37 = alloca [32 x i8],
 20344|  %38 = alloca [8 x i8],
 20345|  %39 = alloca [8 x i8],
 20346|     ;; self[16..+1136] = ptr @anon.94acafa22d01e083ca1cc62f01598c8f.148
 20347|  %40 = alloca [1168 x i8],
 20348|  %41 = alloca [184 x i8],
 20349|  %42 = alloca [184 x i8],
 20350|  %43 = alloca [184 x i8],
 20351|  %44 = alloca [184 x i8],
 20352|     ;; self[16..+1136] = ptr @anon.94acafa22d01e083ca1cc62f01598c8f.148
 20353|  %45 = alloca [1168 x i8],
 20354|  %46 = alloca [32 x i8],
 20355|     ;; self = ptr %1
 20356|     ;; version = i64 %2
 20357|     ;; rnd = ptr %3
 20358|     ;; player = ptr %4
 20359|     ;; data = ptr %5
 20360|     ;; _parameter = ptr %6
 20361|     ;; _debug = ptr %7
 20362|     ;; res = ptr %46
 20363|     ;; bx = ptr %39
 20364|     ;; by = ptr %38
 20369|     ;; len = i64 5
 20370|     ;; count = i64 5
 20371|     ;; count = i64 5
 20372|     ;; count = i64 5
 20373|     ;; len = i64 5
 20374|     ;; count = i64 5
 20375|     ;; count = i64 5
 20376|     ;; count = i64 5
 20378|  %47 = gep %5, i64 8                                                                                                   ;L20
 20379|  %48 = load ptr, ptr %47, , !!8, !!8                                                                                   ;L20
 20380|     ;; context = ptr %48
 20381|  %49 = load ptr, ptr %48, , !!8, !!8                                                                                   ;L20
 20382|     ;; bump = ptr %49
 20383|  store ptr inttoptr (i64 8 to ptr), ptr %46,                                                                           ;L547<20
 20384|  %50 = gep %46, i64 8                                                                                                  ;L547<20
 20385|  store ptr %49, ptr %50,                                                                                               ;L547<20
 20386|  %51 = gep %46, i64 16                                                                                                 ;L547<20
 20387|  %52 = gep %46, i64 24                                                                                                 ;L547<20
 20388|  %53 = gep %4, i64 2352                                                                                                ;L22
 20389|  call void @llvm.memset.p0.i64(ptr %51, i8 0, i64 16, i1 false)                                                        ;L547<20
 20390|  %54 = load i64, ptr %53, , !!8                                                                                        ;L22
 20391|     ;; team = i64 %54
 20392|     ;; team = i64 %54
 20393|  %55 = icmp ult i64 %54, 2                                                                                             ;L22
 20394|  br i1 %55, label %60, label %56                                                                                       ;L22
 20395| 
 20396| 56: ; preds = %8
 20397|  invoke void @core::panicking18panic_bounds_check(i64 %54, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.149) #31
 20398|  to label %59 unwind label %57                                                                                         ;L22
 20399| 
 20400| 57: ; preds = %1930, %1925, %1920, %1918, %1905, %1900, %1899, %1892, %1883, %1881, %1879, %1858, %1587, %1427, %1417, %1402, %1372, %1364, %1360, %1355, %1354, %1347, %1338, %1336, %1334, %1313, %1042, %879, %452, %450, %443, %431, %428, %402, %378, %352, %328, %259, %243, %236, %234, %204, %202, %175, %173, %151, %149, %136, %116, %114, %80, %70, %56
 20401|  %58 = cleanuppad within none []
 20402|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %46) #30 [ "funclet"(token %58) ] ;L139
 20403|  cleanupret from %58 unwind to caller                                                                                  ;L19
 20404| 
 20405| 59: ; preds = %352, %136, %80, %56
 20406|  unreachable
 20407| 
 20408| 60: ; preds = %8
 20409|     ;; self = ptr %4
 20410|  %61 = gep %4, i64 2496                                                                                                ;L581<22
 20411|  %62 = load i32, ptr %61, , !!8                                                                                        ;L581<22
 20412|  %63 = zext nneg i32 %62 to i64                                                                                        ;L581<22
 20413|  %64 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L22
 20414|     ;; self = ptr %64
 20415|     ;; self = ptr %64
 20416|     ;; self = ptr %64
 20417|     ;; self = ptr %64
 20418|     ;; self = ptr %64
 20419|     ;; self = ptr %64
 20420|  %65 = gep %64, i64 480                                                                                                ;L22
 20421|  %66 = getelementptr [5 x ptr], ptr %65, i64 %54                                                                       ;L22
 20422|  %67 = getelementptr ptr, ptr %66, i64 %63                                                                             ;L22
 20423|  %68 = load ptr, ptr %67, , !!8                                                                                        ;L22
 20424|     ;; self = ptr %68
 20425|  %69 = icmp eq ptr %68, null                                                                                           ;L1011<22
 20426|  br i1 %69, label %80, label %70                                                                                       ;L1011<22
 20427| 
 20428| 70: ; preds = %60
 20429|     ;; champ = ptr %68
 20430|  %71 = load ptr, ptr %64, , !!8, !!8                                                                                   ;L23
 20431|  %72 = gep %64, i64 8                                                                                                  ;L23
 20432|  %73 = load ptr, ptr %72, , !!8, !!8                                                                                   ;L23
 20433|  %74 = sub nuw nsw i64 1, %54                                                                                          ;L23
 20434|     ;; team = i64 %74
 20435|     ;; team = i64 %74
 20436|     ;; team = i64 %74
 20437|     ;; team = i64 %74
 20438|  %75 = gep %68, i64 1472                                                                                               ;L23
 20439|  %76 = load i64, ptr %75, , !!8                                                                                        ;L23
 20440|  %77 = gep %73, i64 248                                                                                                ;L23
 20441|  %78 = load ptr, ptr %77, , !!8                                                                                        ;L23
 20442|  %79 = invoke zeroext i1 %78(ptr %71, i64 %74, i64 %76)
 20443|  to label %81 unwind label %57                                                                                         ;L23
 20444| 
 20445| 80: ; preds = %60
 20446|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.150) #31
 20447|  to label %59 unwind label %57                                                                                         ;L1013<22
 20448| 
 20449| 81: ; preds = %70
 20450|     ;; is_visible_to_enemy = i1 %79
 20451|  br i1 %79, label %88, label %82                                                                                       ;L26
 20452| 
 20453| 82: ; preds = %81
 20454|  %83 = gep %1, i64 9                                                                                                   ;L28
 20455|  %84 = load i8, ptr %83, , !!8                                                                                         ;L28
 20456|  %85 = trunc nuw i8 %84 to i1                                                                                          ;L28
 20457|  br i1 %85, label %131, label %86                                                                                      ;L28
 20458| 
 20459| 86: ; preds = %82
 20460|  %87 = gep %1, i64 9                                                                                                   ;L34
 20461|  br label %94                                                                                                          ;L34
 20462| 
 20463| 88: ; preds = %81
 20464|  %89 = gep %1, i64 10                                                                                                  ;L27
 20465|  store i8 1, ptr %89,                                                                                                  ;L27
 20466|  %90 = gep %1, i64 9
 20467|  %91 = load i8, ptr %90,                                                                                               ;L34
 20468|  %92 = trunc nuw i8 %91 to i1                                                                                          ;L34
 20469|  %93 = gep %1, i64 9                                                                                                   ;L34
 20470|  br i1 %92, label %128, label %94                                                                                      ;L34
 20471| 
 20472| 94: ; preds = %88, %86
 20473|  %95 = phi ptr [ %87, %86 ], [ %93, %88 ]
 20475|     ;; self[0..+8] = i64 0
 20476|     ;; self[8..+8] = i64 71
 20477|  %96 = gep %48, i64 32                                                                                                 ;L35
 20478|  %97 = load ptr, ptr %96, , !!8, !!8                                                                                   ;L35
 20479|     ;; predicate[0..+8] = ptr %97
 20480|     ;; predicate[8..+8] = ptr %1
 20481|  %98 = gep %45, i64 24                                                                                                 ;L28<957<35
 20482|  store i64 71, ptr %98,                                                                                                ;L28<957<35
 20483|  %99 = gep %45, i64 32                                                                                                 ;L28<957<35
 20484|  call void @llvm.memcpy.p0.p0.i64(ptr %99, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.148, i64 1136, i1 false)         ;L28<957<35
 20485|     ;; self = ptr %45
 20486|     ;; self = ptr %45
 20487|     ;; self = ptr %45
 20488|     ;; predicate = ptr %45
 20489|     ;; f = ptr %45
 20490|     ;; f = ptr %45
 20491|     ;; self[0..+8] = ptr %45
 20492|     ;; self[8..+8] = i64 71
 20493|     ;; data[0..+8] = ptr %99
 20494|     ;; data[8..+8] = i64 71
 20495|     ;; f[0..+8] = ptr %99
 20496|     ;; f[8..+8] = i64 71
 20497|     ;; f = ptr %45
 20498|     ;; f[16..+8] = ptr %45
 20499|     ;; self = ptr %45
 20500|     ;; self = ptr %45
 20502|     ;; rhs = i64 1
 20503|  %100 = gep %97, i64 7320
 20504|  %101 = load i64, ptr %1,
 20505|  br label %104                                                                                                         ;L167<215<263<2971<98<35
 20506| 
 20507| 102: ; preds = %118
 20508|  %103 = icmp eq i64 %106, 71                                                                                           ;L167<215<263<2971<98<35
 20509|  br i1 %103, label %136, label %104                                                                                    ;L167<215<263<2971<98<35
 20510| 
 20511| 104: ; preds = %102, %94
 20512|  %105 = phi i64 [ 0, %94 ], [ %106, %102 ]
 20513|     ;; i = i64 %105
 20514|     ;; value = i64 %105
 20515|     ;; self = i64 %105
 20516|  %106 = add nuw nsw i64 %105, 1                                                                                        ;L971<63<169<215<263<2971<98<35
 20518|     ;; f = ptr undef
 20520|     ;; idx = i64 %105
 20521|     ;; index = i64 %105
 20522|     ;; self = i64 %105
 20523|     ;; self[0..+8] = ptr %99
 20524|     ;; slice[0..+8] = ptr %99
 20525|     ;; self[8..+8] = i64 71
 20526|     ;; slice[8..+8] = i64 71
 20527|  %107 = gepS %99, i64 %105                                                                                             ;L253<646<219<170<215<263<2971<98<35
 20528|     ;; self = ptr %107
 20529|     ;; self = ptr %107
 20530|     ;; src = ptr %107
 20531|  %108 = load i64, ptr %107, , !!30080, !!8                                                                             ;L1733<1171<798<219<170<215<263<2971<98<35
 20532|  %109 = gep %107, i64 8                                                                                                ;L1733<1171<798<219<170<215<263<2971<98<35
 20533|  %110 = load i64, ptr %109, , !!30080, !!8                                                                             ;L1733<1171<798<219<170<215<263<2971<98<35
 20534|     ;; elem[0..+8] = i64 %108
 20535|     ;; elem[8..+8] = i64 %110
 20536|     ;; x[0..+8] = i64 %108
 20537|     ;; x[8..+8] = i64 %110
 20547|  %111 = icmp ult i64 %110, 30                                                                                          ;L35<298<2967<220<170<215<263<2971<98<35
 20548|  br i1 %111, label %112, label %114                                                                                    ;L35<298<2967<220<170<215<263<2971<98<35
 20549| 
 20550| 112: ; preds = %104
 20551|  %113 = icmp ult i64 %108, 30                                                                                          ;L35<298<2967<220<170<215<263<2971<98<35
 20552|  br i1 %113, label %118, label %116                                                                                    ;L35<298<2967<220<170<215<263<2971<98<35
 20553| 
 20554| 114: ; preds = %104
 20555|  invoke void @core::panicking18panic_bounds_check(i64 %110, i64 30, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.40) #31
 20556|  to label %115 unwind label %57                                                                                        ;L35<298<2967<220<170<215<263<2971<98<35
 20557| 
 20558| 115: ; preds = %114
 20559|  unreachable                                                                                                           ;L35<298<2967<220<170<215<263<2971<98<35
 20560| 
 20561| 116: ; preds = %112
 20562|  invoke void @core::panicking18panic_bounds_check(i64 %108, i64 30, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.40) #31
 20563|  to label %117 unwind label %57                                                                                        ;L35<298<2967<220<170<215<263<2971<98<35
 20564| 
 20565| 117: ; preds = %116
 20566|  unreachable                                                                                                           ;L35<298<2967<220<170<215<263<2971<98<35
 20567| 
 20568| 118: ; preds = %112
 20569|  %119 = getelementptr [30 x i64], ptr %100, i64 %110                                                                   ;L35<298<2967<220<170<215<263<2971<98<35
 20570|  %120 = getelementptr i64, ptr %119, i64 %108                                                                          ;L35<298<2967<220<170<215<263<2971<98<35
 20571|  %121 = load i64, ptr %120, , !!30131, !!8                                                                             ;L35<298<2967<220<170<215<263<2971<98<35
 20572|  %122 = icmp eq i64 %121, %101                                                                                         ;L35<298<2967<220<170<215<263<2971<98<35
 20573|  br i1 %122, label %137, label %102                                                                                    ;L2967<220<170<215<263<2971<98<35
 20574| 
 20575| 123: ; preds = %206, %201, %177, %172
 20576|  %124 = phi ptr [ inttoptr (i64 8 to ptr), %201 ], [ %207, %206 ], [ %178, %177 ], [ inttoptr (i64 8 to ptr), %172 ]
 20577|  %125 = phi i64 [ 0, %201 ], [ %210, %206 ], [ %181, %177 ], [ 0, %172 ]
 20578|  %126 = load i8, ptr %95, , !!8                                                                                        ;L57
 20579|  %127 = trunc nuw i8 %126 to i1                                                                                        ;L57
 20580|  br i1 %127, label %212, label %211                                                                                    ;L57
 20581| 
 20582| 128: ; preds = %88
 20583|  %129 = load i8, ptr %93, , !!8                                                                                        ;L57
 20584|  %130 = trunc nuw i8 %129 to i1                                                                                        ;L57
 20585|  br i1 %130, label %212, label %244                                                                                    ;L57
 20586| 
 20587| 131: ; preds = %82
 20588|  %132 = gep %1, i64 10                                                                                                 ;L30
 20589|  store i8 0, ptr %132,                                                                                                 ;L30
 20590|  %133 = gep %1, i64 9                                                                                                  ;L34
 20591|  %134 = load i8, ptr %133, , !!8                                                                                       ;L57
 20592|  %135 = trunc nuw i8 %134 to i1                                                                                        ;L57
 20593|  br i1 %135, label %212, label %243                                                                                    ;L57
 20594| 
 20595| 136: ; preds = %102
 20596|     ;; self[0..+8] = i64 0
 20597|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.151) #31
 20598|  to label %59 unwind label %57                                                                                         ;L1013<35
 20599| 
 20600| 137: ; preds = %118
 20601|     ;; self[8..+8] = i64 %108
 20602|     ;; self[16..+8] = i64 %110
 20603|     ;; self[0..+8] = i64 1
 20604|     ;; bush_pos[0..+8] = i64 %108
 20605|     ;; bush_pos[8..+8] = i64 %110
 20607|  %138 = mul nuw nsw i64 %108, 32000                                                                                    ;L37
 20608|  %139 = add nuw nsw i64 %138, 16000                                                                                    ;L37
 20609|     ;; bx = i64 %139
 20610|     ;; x = i64 %139
 20611|  %140 = mul nsw i64 %110, -32000                                                                                       ;L37
 20612|  %141 = add nsw i64 %140, -16000                                                                                       ;L37
 20613|     ;; by = i64 %110
 20614|     ;; y = i64 %110
 20615|  %142 = gep %48, i64 8                                                                                                 ;L22<39
 20616|  %143 = load ptr, ptr %142, , !!8, !!8                                                                                 ;L22<39
 20617|  %144 = gep %143, i64 4800                                                                                             ;L22<39
 20618|  %145 = load i64, ptr %144, , !!8                                                                                      ;L22<39
 20619|  %146 = add i64 %141, %145                                                                                             ;L22<39
 20620|     ;; ry = i64 %146
 20621|  %147 = icmp ult i64 %146, %139                                                                                        ;L24<39
 20622|  %148 = icmp eq i64 %54, 0                                                                                             ;L0
 20623|  br i1 %147, label %149, label %151                                                                                    ;L39
 20624| 
 20625| 149: ; preds = %137
 20626|  %150 = invoke { i64, i64 } @gc::simulation7map_defNtB2_6MapDef8camp_pos(ptr %97, i8 5, i1 zeroext %148)
 20627|  to label %153 unwind label %57                                                                                        ;L48
 20628| 
 20629| 151: ; preds = %137
 20630|  %152 = invoke { i64, i64 } @gc::simulation7map_defNtB2_6MapDef8camp_pos(ptr %97, i8 4, i1 zeroext %148)
 20631|  to label %182 unwind label %57                                                                                        ;L40
 20632| 
 20633| 153: ; preds = %149
 20634|  %154 = extractvalue { i64, i64 } %150, 0                                                                              ;L48
 20635|  %155 = extractvalue { i64, i64 } %150, 1                                                                              ;L48
 20636|     ;; ex = i64 %154
 20637|     ;; x2 = i64 %154
 20638|     ;; other = i64 %154
 20639|     ;; ey = i64 %155
 20640|     ;; y2 = i64 %155
 20641|     ;; other = i64 %155
 20642|  %156 = gep %68, i64 1632                                                                                              ;L49
 20643|  %157 = load i64, ptr %156, , !!8                                                                                      ;L49
 20644|     ;; x1 = i64 %157
 20645|     ;; self = i64 %157
 20646|  %158 = gep %68, i64 1640                                                                                              ;L49
 20647|  %159 = load i64, ptr %158, , !!8                                                                                      ;L49
 20648|     ;; y1 = i64 %159
 20649|     ;; self = i64 %159
 20650|  %160 = icmp ult i64 %157, %154                                                                                        ;L3147<7<49
 20651|  %161 = sub nuw i64 %154, %157                                                                                         ;L3147<7<49
 20652|  %162 = sub nuw i64 %157, %154                                                                                         ;L3147<7<49
 20653|  %163 = select i1 %160, i64 %161, i64 %162                                                                             ;L3147<7<49
 20654|     ;; dx = i64 %163
 20655|  %164 = icmp ult i64 %159, %155                                                                                        ;L3147<8<49
 20656|  %165 = sub nuw i64 %155, %159                                                                                         ;L3147<8<49
 20657|  %166 = sub nuw i64 %159, %155                                                                                         ;L3147<8<49
 20658|  %167 = select i1 %164, i64 %165, i64 %166                                                                             ;L3147<8<49
 20659|     ;; dy = i64 %167
 20660|  %168 = mul i64 %163, %163                                                                                             ;L9<49
 20661|  %169 = mul i64 %167, %167                                                                                             ;L9<49
 20662|  %170 = add i64 %169, %168                                                                                             ;L9<49
 20663|  %171 = icmp ugt i64 %170, 10000000000                                                                                 ;L49
 20664|  br i1 %171, label %173, label %172                                                                                    ;L49
 20665| 
 20666| 172: ; preds = %153
 20667|  store i8 1, ptr %95,                                                                                                  ;L52
 20668|  br label %123                                                                                                         ;L49
 20669| 
 20670| 173: ; preds = %153
 20673|  invoke void @ai::small_action6aroundNtB5_25SmallActionAroundPosition17new_with_out_line(ptr sret([184 x i8]) %41, ptr %3, ptr %5, i64 %154, i64 %155, i64 5, i8 1)
 20674|  to label %174 unwind label %57                                                                                        ;L50
 20675| 
 20676| 174: ; preds = %173
 20677|  call void @llvm.memcpy.p0.p0.i64(ptr %42, ptr %41, i64 184, i1 false)                                                 ;L50
 20680|     ;; self = ptr %46
 20681|     ;; self = ptr %46
 20682|     ;; value = ptr %42
 20683|     ;; src = ptr %42
 20684|     ;; additional = i64 1
 20685|     ;; needed_extra_cap = i64 1
 20686|     ;; needed_extra_cap = i64 1
 20687|     ;; strategy = i8 1
 20688|     ;; self = ptr %46
 20689|     ;; self = ptr %46
 20690|     ;; self = ptr %46
 20691|     ;; self = ptr %46
 20692|     ;; used_cap = i64 0
 20693|     ;; used_cap = i64 0
 20694|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %46, i64 0, i64 1, i1 zeroext true)
 20695|  to label %177 unwind label %175, !!30210                                                                              ;L619<430<738<1429<50
 20696| 
 20697| 175: ; preds = %174
 20698|  %176 = cleanuppad within none []
 20699|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %42) #30 [ "funclet"(token %176) ], !!30191 ;L1436<50
 20700|  cleanupret from %176 unwind label %57
 20701| 
 20702| 177: ; preds = %174
 20703|  %178 = load ptr, ptr %46, , !!30210                                                                                   ;L138<1432<50
 20704|  %179 = load i64, ptr %52, , !!30210                                                                                   ;L1432<50
 20705|     ;; self = ptr %46
 20706|     ;; self = ptr %178
 20707|     ;; count = i64 %179
 20708|  %180 = gepS %178, i64 %179                                                                                            ;L961<1432<50
 20709|     ;; end = ptr %180
 20710|     ;; dst = ptr %180
 20711|  call void @llvm.memcpy.p0.p0.i64(ptr %180, ptr %42, i64 184, i1 false), !!30191                                       ;L1933<1433<50
 20712|  %181 = add i64 %179, 1                                                                                                ;L1434<50
 20713|  store i64 %181, ptr %52, , !!30210                                                                                    ;L1434<50
 20715|  br label %123                                                                                                         ;L49
 20716| 
 20717| 182: ; preds = %151
 20718|  %183 = extractvalue { i64, i64 } %152, 0                                                                              ;L40
 20719|  %184 = extractvalue { i64, i64 } %152, 1                                                                              ;L40
 20720|     ;; ex = i64 %183
 20721|     ;; x2 = i64 %183
 20722|     ;; other = i64 %183
 20723|     ;; ey = i64 %184
 20724|     ;; y2 = i64 %184
 20725|     ;; other = i64 %184
 20726|  %185 = gep %68, i64 1632                                                                                              ;L42
 20727|  %186 = load i64, ptr %185, , !!8                                                                                      ;L42
 20728|     ;; x1 = i64 %186
 20729|     ;; self = i64 %186
 20730|  %187 = gep %68, i64 1640                                                                                              ;L42
 20731|  %188 = load i64, ptr %187, , !!8                                                                                      ;L42
 20732|     ;; y1 = i64 %188
 20733|     ;; self = i64 %188
 20734|  %189 = icmp ult i64 %186, %183                                                                                        ;L3147<7<42
 20735|  %190 = sub nuw i64 %183, %186                                                                                         ;L3147<7<42
 20736|  %191 = sub nuw i64 %186, %183                                                                                         ;L3147<7<42
 20737|  %192 = select i1 %189, i64 %190, i64 %191                                                                             ;L3147<7<42
 20738|     ;; dx = i64 %192
 20739|  %193 = icmp ult i64 %188, %184                                                                                        ;L3147<8<42
 20740|  %194 = sub nuw i64 %184, %188                                                                                         ;L3147<8<42
 20741|  %195 = sub nuw i64 %188, %184                                                                                         ;L3147<8<42
 20742|  %196 = select i1 %193, i64 %194, i64 %195                                                                             ;L3147<8<42
 20743|     ;; dy = i64 %196
 20744|  %197 = mul i64 %192, %192                                                                                             ;L9<42
 20745|  %198 = mul i64 %196, %196                                                                                             ;L9<42
 20746|  %199 = add i64 %198, %197                                                                                             ;L9<42
 20747|  %200 = icmp ugt i64 %199, 10000000000                                                                                 ;L42
 20748|  br i1 %200, label %202, label %201                                                                                    ;L42
 20749| 
 20750| 201: ; preds = %182
 20751|  store i8 1, ptr %95,                                                                                                  ;L45
 20752|  br label %123                                                                                                         ;L42
 20753| 
 20754| 202: ; preds = %182
 20757|  invoke void @ai::small_action6aroundNtB5_25SmallActionAroundPosition17new_with_out_line(ptr sret([184 x i8]) %43, ptr %3, ptr %5, i64 %183, i64 %184, i64 5, i8 1)
 20758|  to label %203 unwind label %57                                                                                        ;L43
 20759| 
 20760| 203: ; preds = %202
 20761|  call void @llvm.memcpy.p0.p0.i64(ptr %44, ptr %43, i64 184, i1 false)                                                 ;L43
 20764|     ;; self = ptr %46
 20765|     ;; self = ptr %46
 20766|     ;; value = ptr %44
 20767|     ;; src = ptr %44
 20768|     ;; additional = i64 1
 20769|     ;; needed_extra_cap = i64 1
 20770|     ;; needed_extra_cap = i64 1
 20771|     ;; strategy = i8 1
 20772|     ;; self = ptr %46
 20773|     ;; self = ptr %46
 20774|     ;; self = ptr %46
 20775|     ;; self = ptr %46
 20776|     ;; used_cap = i64 0
 20777|     ;; used_cap = i64 0
 20778|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %46, i64 0, i64 1, i1 zeroext true)
 20779|  to label %206 unwind label %204, !!30256                                                                              ;L619<430<738<1429<43
 20780| 
 20781| 204: ; preds = %203
 20782|  %205 = cleanuppad within none []
 20783|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %44) #30 [ "funclet"(token %205) ], !!30237 ;L1436<43
 20784|  cleanupret from %205 unwind label %57
 20785| 
 20786| 206: ; preds = %203
 20787|  %207 = load ptr, ptr %46, , !!30256                                                                                   ;L138<1432<43
 20788|  %208 = load i64, ptr %52, , !!30256                                                                                   ;L1432<43
 20789|     ;; self = ptr %46
 20790|     ;; self = ptr %207
 20791|     ;; count = i64 %208
 20792|  %209 = gepS %207, i64 %208                                                                                            ;L961<1432<43
 20793|     ;; end = ptr %209
 20794|     ;; dst = ptr %209
 20795|  call void @llvm.memcpy.p0.p0.i64(ptr %209, ptr %44, i64 184, i1 false), !!30237                                       ;L1933<1433<43
 20796|  %210 = add i64 %208, 1                                                                                                ;L1434<43
 20797|  store i64 %210, ptr %52, , !!30256                                                                                    ;L1434<43
 20799|  br label %123                                                                                                         ;L42
 20800| 
 20801| 211: ; preds = %123
 20802|  br i1 %79, label %244, label %243                                                                                     ;L107
 20803| 
 20804| 212: ; preds = %131, %128, %123
 20805|  %213 = phi i64 [ 0, %131 ], [ %125, %123 ], [ 0, %128 ]
 20806|  %214 = phi ptr [ inttoptr (i64 8 to ptr), %131 ], [ %124, %123 ], [ inttoptr (i64 8 to ptr), %128 ]
 20807|  %215 = phi ptr [ %133, %131 ], [ %95, %123 ], [ %93, %128 ]
 20809|     ;; self[0..+8] = i64 0
 20810|     ;; self[8..+8] = i64 71
 20811|  %216 = gep %48, i64 32                                                                                                ;L58
 20812|  %217 = load ptr, ptr %216, , !!8, !!8                                                                                 ;L58
 20813|     ;; predicate[0..+8] = ptr %217
 20814|     ;; predicate[8..+8] = ptr %1
 20815|  %218 = gep %40, i64 24                                                                                                ;L28<957<58
 20816|  store i64 71, ptr %218,                                                                                               ;L28<957<58
 20817|  %219 = gep %40, i64 32                                                                                                ;L28<957<58
 20818|  call void @llvm.memcpy.p0.p0.i64(ptr %219, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.148, i64 1136, i1 false)        ;L28<957<58
 20819|     ;; self = ptr %40
 20820|     ;; self = ptr %40
 20821|     ;; self = ptr %40
 20822|     ;; predicate = ptr %40
 20823|     ;; f = ptr %40
 20824|     ;; f = ptr %40
 20825|     ;; self[0..+8] = ptr %40
 20826|     ;; self[8..+8] = i64 71
 20827|     ;; data[0..+8] = ptr %219
 20828|     ;; data[8..+8] = i64 71
 20829|     ;; f[0..+8] = ptr %219
 20830|     ;; f[8..+8] = i64 71
 20831|     ;; f = ptr %40
 20832|     ;; f[16..+8] = ptr %40
 20833|     ;; self = ptr %40
 20834|     ;; self = ptr %40
 20836|     ;; rhs = i64 1
 20837|  %220 = gep %217, i64 7320
 20838|  %221 = load i64, ptr %1,
 20839|  br label %224                                                                                                         ;L167<215<263<2971<98<58
 20840| 
 20841| 222: ; preds = %238
 20842|  %223 = icmp eq i64 %226, 71                                                                                           ;L167<215<263<2971<98<58
 20843|  br i1 %223, label %352, label %224                                                                                    ;L167<215<263<2971<98<58
 20844| 
 20845| 224: ; preds = %222, %212
 20846|  %225 = phi i64 [ 0, %212 ], [ %226, %222 ]
 20847|     ;; i = i64 %225
 20848|     ;; value = i64 %225
 20849|     ;; self = i64 %225
 20850|  %226 = add nuw nsw i64 %225, 1                                                                                        ;L971<63<169<215<263<2971<98<58
 20852|     ;; f = ptr undef
 20854|     ;; idx = i64 %225
 20855|     ;; index = i64 %225
 20856|     ;; self = i64 %225
 20857|     ;; self[0..+8] = ptr %219
 20858|     ;; slice[0..+8] = ptr %219
 20859|     ;; self[8..+8] = i64 71
 20860|     ;; slice[8..+8] = i64 71
 20861|  %227 = gepS %219, i64 %225                                                                                            ;L253<646<219<170<215<263<2971<98<58
 20862|     ;; self = ptr %227
 20863|     ;; self = ptr %227
 20864|     ;; src = ptr %227
 20865|  %228 = load i64, ptr %227, , !!30363, !!8                                                                             ;L1733<1171<798<219<170<215<263<2971<98<58
 20866|  %229 = gep %227, i64 8                                                                                                ;L1733<1171<798<219<170<215<263<2971<98<58
 20867|  %230 = load i64, ptr %229, , !!30363, !!8                                                                             ;L1733<1171<798<219<170<215<263<2971<98<58
 20868|     ;; elem[0..+8] = i64 %228
 20869|     ;; elem[8..+8] = i64 %230
 20870|     ;; x[0..+8] = i64 %228
 20871|     ;; x[8..+8] = i64 %230
 20881|  %231 = icmp ult i64 %230, 30                                                                                          ;L58<298<2967<220<170<215<263<2971<98<58
 20882|  br i1 %231, label %232, label %234                                                                                    ;L58<298<2967<220<170<215<263<2971<98<58
 20883| 
 20884| 232: ; preds = %224
 20885|  %233 = icmp ult i64 %228, 30                                                                                          ;L58<298<2967<220<170<215<263<2971<98<58
 20886|  br i1 %233, label %238, label %236                                                                                    ;L58<298<2967<220<170<215<263<2971<98<58
 20887| 
 20888| 234: ; preds = %224
 20889|  invoke void @core::panicking18panic_bounds_check(i64 %230, i64 30, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.41) #31
 20890|  to label %235 unwind label %57                                                                                        ;L58<298<2967<220<170<215<263<2971<98<58
 20891| 
 20892| 235: ; preds = %234
 20893|  unreachable                                                                                                           ;L58<298<2967<220<170<215<263<2971<98<58
 20894| 
 20895| 236: ; preds = %232
 20896|  invoke void @core::panicking18panic_bounds_check(i64 %228, i64 30, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.41) #31
 20897|  to label %237 unwind label %57                                                                                        ;L58<298<2967<220<170<215<263<2971<98<58
 20898| 
 20899| 237: ; preds = %236
 20900|  unreachable                                                                                                           ;L58<298<2967<220<170<215<263<2971<98<58
 20901| 
 20902| 238: ; preds = %232
 20903|  %239 = getelementptr [30 x i64], ptr %220, i64 %230                                                                   ;L58<298<2967<220<170<215<263<2971<98<58
 20904|  %240 = getelementptr i64, ptr %239, i64 %228                                                                          ;L58<298<2967<220<170<215<263<2971<98<58
 20905|  %241 = load i64, ptr %240, , !!30409, !!8                                                                             ;L58<298<2967<220<170<215<263<2971<98<58
 20906|  %242 = icmp eq i64 %241, %221                                                                                         ;L58<298<2967<220<170<215<263<2971<98<58
 20907|  br i1 %242, label %353, label %222                                                                                    ;L2967<220<170<215<263<2971<98<58
 20908| 
 20909| 243: ; preds = %1929, %1909, %1904, %1894, %1428, %1424, %211, %131
 20911|  invoke void @ai::fight_check20attack_summon_action(ptr sret([32 x i8]) %19, ptr %4, ptr %5)
 20912|  to label %1930 unwind label %57                                                                                       ;L136
 20913| 
 20914| 244: ; preds = %1424, %211, %128
 20915|  %245 = getelementptr [5 x ptr], ptr %65, i64 %74                                                                      ;L1905<108
 20916|     ;; self[0..+8] = ptr %245
 20917|     ;; slice[0..+8] = ptr %245
 20918|     ;; self[8..+8] = i64 5
 20919|     ;; slice[8..+8] = i64 5
 20920|     ;; ptr = ptr %245
 20921|     ;; self = ptr %245
 20922|  %246 = gep %245, i64 40                                                                                               ;L961<100<1042<1905<108
 20923|     ;; self[0..+8] = ptr %245
 20924|     ;; self[0..+8] = ptr %245
 20925|     ;; self[8..+8] = ptr %246
 20926|     ;; self[8..+8] = ptr %246
 20927|     ;; self[16..+8] = ptr %68
 20928|     ;; self[16..+8] = ptr %68
 20930|     ;; f = ptr %68
 20931|     ;; self = ptr %18
 20935|     ;; f = ptr %68
 20936|  %247 = gep %18, i64 8                                                                                                 ;L69<836<3387<110
 20937|  store ptr %246, ptr %247, , !!30493                                                                                   ;L69<836<3387<110
 20938|  %248 = gep %18, i64 16                                                                                                ;L69<836<3387<110
 20939|  store ptr %68, ptr %248, , !!30493                                                                                    ;L69<836<3387<110
 20940|  %249 = gep %18, i64 24                                                                                                ;L69<836<3387<110
 20941|  store ptr %68, ptr %249, , !!30496                                                                                    ;L69<836<3387<110
 20943|     ;; self = ptr %18
 20946|     ;; self = ptr %18
 20948|     ;; self = ptr %18
 20950|     ;; self = ptr %18
 20953|     ;; self = ptr %18
 20957|     ;; f[0..+8] = ptr %18
 20959|     ;; self = ptr %18
 20962|     ;; self = ptr %18
 20963|     ;; count = i64 1
 20964|     ;; ptr = ptr %245
 20965|     ;; self = ptr %245
 20966|     ;; end_or_len = ptr %246
 20969|  %250 = gep %68, i64 8
 20970|     ;; x = ptr %245
 20971|  %251 = load ptr, ptr %245, , !!30614, !!8                                                                             ;L2494<138<2971<98<107<2706<3416<3387<110
 20976|  %252 = icmp eq ptr %251, null                                                                                         ;L49<2494<138<2971<98<107<2706<3416<3387<110
 20977|  br i1 %252, label %267, label %253                                                                                    ;L49<2494<138<2971<98<107<2706<3416<3387<110
 20978| 
 20979| 253: ; preds = %244
 20980|     ;; x = ptr %251
 20982|     ;; x = ptr %251
 20989|     ;; self = ptr %251
 20990|     ;; entity = ptr %68
 20991|     ;; self = ptr %68
 20992|  %254 = load i64, ptr %68, , !!30685, !!8                                                                              ;L1136<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 20993|  %255 = trunc nuw i64 %254 to i1                                                                                       ;L1136<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 20994|  br i1 %255, label %328, label %256                                                                                    ;L1136<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 20995| 
 20996| 256: ; preds = %253
 20997|     ;; team = ptr %68
 20998|  %257 = load i64, ptr %250, , !!30685, !!8                                                                             ;L1137<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 20999|     ;; team = i64 %257
 21000|  %258 = icmp ult i64 %257, 2                                                                                           ;L1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21001|  br i1 %258, label %262, label %259                                                                                    ;L1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21002| 
 21003| 259: ; preds = %319, %304, %289, %274, %256
 21004|  %260 = phi i64 [ %257, %256 ], [ %275, %274 ], [ %290, %289 ], [ %305, %304 ], [ %320, %319 ]                         ;L1137<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21005|  invoke void @core::panicking18panic_bounds_check(i64 %260, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 21006|  to label %261 unwind label %57                                                                                        ;L1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21007| 
 21008| 261: ; preds = %259
 21009|  unreachable                                                                                                           ;L1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21010| 
 21011| 262: ; preds = %256
 21013|  %263 = gep %251, i64 56                                                                                               ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21014|  %264 = gepS %263, i64 %257                                                                                            ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21015|  %265 = load i64, ptr %264, , !!30614, !!8                                                                             ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21016|  %266 = icmp eq i64 %265, 0                                                                                            ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21017|  br i1 %266, label %328, label %267                                                                                    ;L2968<50<2494<138<2971<98<107<2706<3416<3387<110
 21018| 
 21019| 267: ; preds = %262, %244
 21020|     ;; self = ptr %18
 21021|     ;; count = i64 1
 21022|     ;; ptr = !DIArgList(ptr %245, i64 8)
 21023|     ;; self = !DIArgList(ptr %245, i64 8)
 21024|     ;; end_or_len = ptr %246
 21027|  %268 = gep %245, i64 8                                                                                                ;L656<185<2493<138<2971<98<107<2706<3416<3387<110
 21028|     ;; ptr = ptr %268
 21029|     ;; x = ptr %268
 21030|  %269 = load ptr, ptr %268, , !!30614, !!8                                                                             ;L2494<138<2971<98<107<2706<3416<3387<110
 21035|  %270 = icmp eq ptr %269, null                                                                                         ;L49<2494<138<2971<98<107<2706<3416<3387<110
 21036|  br i1 %270, label %282, label %271                                                                                    ;L49<2494<138<2971<98<107<2706<3416<3387<110
 21037| 
 21038| 271: ; preds = %267
 21039|     ;; x = ptr %269
 21041|     ;; x = ptr %269
 21048|     ;; self = ptr %269
 21049|     ;; entity = ptr %68
 21050|     ;; self = ptr %68
 21051|  %272 = load i64, ptr %68, , !!30699, !!8                                                                              ;L1136<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21052|  %273 = trunc nuw i64 %272 to i1                                                                                       ;L1136<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21053|  br i1 %273, label %328, label %274                                                                                    ;L1136<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21054| 
 21055| 274: ; preds = %271
 21056|     ;; team = ptr %68
 21057|  %275 = load i64, ptr %250, , !!30699, !!8                                                                             ;L1137<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21058|     ;; team = i64 %275
 21059|  %276 = icmp ult i64 %275, 2                                                                                           ;L1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21060|  br i1 %276, label %277, label %259                                                                                    ;L1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21061| 
 21062| 277: ; preds = %274
 21064|  %278 = gep %269, i64 56                                                                                               ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21065|  %279 = gepS %278, i64 %275                                                                                            ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21066|  %280 = load i64, ptr %279, , !!30614, !!8                                                                             ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21067|  %281 = icmp eq i64 %280, 0                                                                                            ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21068|  br i1 %281, label %328, label %282                                                                                    ;L2968<50<2494<138<2971<98<107<2706<3416<3387<110
 21069| 
 21070| 282: ; preds = %277, %267
 21071|     ;; self = ptr %18
 21072|     ;; count = i64 1
 21073|     ;; ptr = !DIArgList(ptr %245, i64 16)
 21074|     ;; self = !DIArgList(ptr %245, i64 16)
 21075|     ;; end_or_len = ptr %246
 21078|  %283 = gep %245, i64 16                                                                                               ;L656<185<2493<138<2971<98<107<2706<3416<3387<110
 21079|     ;; ptr = ptr %283
 21080|     ;; x = ptr %283
 21081|  %284 = load ptr, ptr %283, , !!30614, !!8                                                                             ;L2494<138<2971<98<107<2706<3416<3387<110
 21086|  %285 = icmp eq ptr %284, null                                                                                         ;L49<2494<138<2971<98<107<2706<3416<3387<110
 21087|  br i1 %285, label %297, label %286                                                                                    ;L49<2494<138<2971<98<107<2706<3416<3387<110
 21088| 
 21089| 286: ; preds = %282
 21090|     ;; x = ptr %284
 21092|     ;; x = ptr %284
 21099|     ;; self = ptr %284
 21100|     ;; entity = ptr %68
 21101|     ;; self = ptr %68
 21102|  %287 = load i64, ptr %68, , !!30702, !!8                                                                              ;L1136<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21103|  %288 = trunc nuw i64 %287 to i1                                                                                       ;L1136<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21104|  br i1 %288, label %328, label %289                                                                                    ;L1136<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21105| 
 21106| 289: ; preds = %286
 21107|     ;; team = ptr %68
 21108|  %290 = load i64, ptr %250, , !!30702, !!8                                                                             ;L1137<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21109|     ;; team = i64 %290
 21110|  %291 = icmp ult i64 %290, 2                                                                                           ;L1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21111|  br i1 %291, label %292, label %259                                                                                    ;L1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21112| 
 21113| 292: ; preds = %289
 21115|  %293 = gep %284, i64 56                                                                                               ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21116|  %294 = gepS %293, i64 %290                                                                                            ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21117|  %295 = load i64, ptr %294, , !!30614, !!8                                                                             ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21118|  %296 = icmp eq i64 %295, 0                                                                                            ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21119|  br i1 %296, label %328, label %297                                                                                    ;L2968<50<2494<138<2971<98<107<2706<3416<3387<110
 21120| 
 21121| 297: ; preds = %292, %282
 21122|     ;; self = ptr %18
 21123|     ;; count = i64 1
 21124|     ;; ptr = !DIArgList(ptr %245, i64 24)
 21125|     ;; self = !DIArgList(ptr %245, i64 24)
 21126|     ;; end_or_len = ptr %246
 21129|  %298 = gep %245, i64 24                                                                                               ;L656<185<2493<138<2971<98<107<2706<3416<3387<110
 21130|     ;; ptr = ptr %298
 21131|     ;; x = ptr %298
 21132|  %299 = load ptr, ptr %298, , !!30614, !!8                                                                             ;L2494<138<2971<98<107<2706<3416<3387<110
 21137|  %300 = icmp eq ptr %299, null                                                                                         ;L49<2494<138<2971<98<107<2706<3416<3387<110
 21138|  br i1 %300, label %312, label %301                                                                                    ;L49<2494<138<2971<98<107<2706<3416<3387<110
 21139| 
 21140| 301: ; preds = %297
 21141|     ;; x = ptr %299
 21143|     ;; x = ptr %299
 21150|     ;; self = ptr %299
 21151|     ;; entity = ptr %68
 21152|     ;; self = ptr %68
 21153|  %302 = load i64, ptr %68, , !!30705, !!8                                                                              ;L1136<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21154|  %303 = trunc nuw i64 %302 to i1                                                                                       ;L1136<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21155|  br i1 %303, label %328, label %304                                                                                    ;L1136<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21156| 
 21157| 304: ; preds = %301
 21158|     ;; team = ptr %68
 21159|  %305 = load i64, ptr %250, , !!30705, !!8                                                                             ;L1137<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21160|     ;; team = i64 %305
 21161|  %306 = icmp ult i64 %305, 2                                                                                           ;L1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21162|  br i1 %306, label %307, label %259                                                                                    ;L1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21163| 
 21164| 307: ; preds = %304
 21166|  %308 = gep %299, i64 56                                                                                               ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21167|  %309 = gepS %308, i64 %305                                                                                            ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21168|  %310 = load i64, ptr %309, , !!30614, !!8                                                                             ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21169|  %311 = icmp eq i64 %310, 0                                                                                            ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21170|  br i1 %311, label %328, label %312                                                                                    ;L2968<50<2494<138<2971<98<107<2706<3416<3387<110
 21171| 
 21172| 312: ; preds = %307, %297
 21173|     ;; self = ptr %18
 21174|     ;; count = i64 1
 21175|     ;; ptr = !DIArgList(ptr %245, i64 32)
 21176|     ;; self = !DIArgList(ptr %245, i64 32)
 21177|     ;; end_or_len = ptr %246
 21180|  %313 = gep %245, i64 32                                                                                               ;L656<185<2493<138<2971<98<107<2706<3416<3387<110
 21181|     ;; ptr = ptr %313
 21182|     ;; x = ptr %313
 21183|  %314 = load ptr, ptr %313, , !!30614, !!8                                                                             ;L2494<138<2971<98<107<2706<3416<3387<110
 21188|  %315 = icmp eq ptr %314, null                                                                                         ;L49<2494<138<2971<98<107<2706<3416<3387<110
 21189|  br i1 %315, label %327, label %316                                                                                    ;L49<2494<138<2971<98<107<2706<3416<3387<110
 21190| 
 21191| 316: ; preds = %312
 21192|     ;; x = ptr %314
 21194|     ;; x = ptr %314
 21201|     ;; self = ptr %314
 21202|     ;; entity = ptr %68
 21203|     ;; self = ptr %68
 21204|  %317 = load i64, ptr %68, , !!30708, !!8                                                                              ;L1136<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21205|  %318 = trunc nuw i64 %317 to i1                                                                                       ;L1136<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21206|  br i1 %318, label %328, label %319                                                                                    ;L1136<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21207| 
 21208| 319: ; preds = %316
 21209|     ;; team = ptr %68
 21210|  %320 = load i64, ptr %250, , !!30708, !!8                                                                             ;L1137<1482<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21211|     ;; team = i64 %320
 21212|  %321 = icmp ult i64 %320, 2                                                                                           ;L1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21213|  br i1 %321, label %322, label %259                                                                                    ;L1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21214| 
 21215| 322: ; preds = %319
 21217|  %323 = gep %314, i64 56                                                                                               ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21218|  %324 = gepS %323, i64 %320                                                                                            ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21219|  %325 = load i64, ptr %324, , !!30614, !!8                                                                             ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21220|  %326 = icmp eq i64 %325, 0                                                                                            ;L122<1483<109<298<2967<50<2494<138<2971<98<107<2706<3416<3387<110
 21221|  br i1 %326, label %328, label %327                                                                                    ;L2968<50<2494<138<2971<98<107<2706<3416<3387<110
 21222| 
 21223| 327: ; preds = %322, %312
 21224|     ;; self = ptr %18
 21225|     ;; count = i64 1
 21226|     ;; ptr = !DIArgList(ptr %245, i64 40)
 21227|     ;; self = !DIArgList(ptr %245, i64 40)
 21228|     ;; end_or_len = ptr %246
 21232|     ;; nearest_enemy = ptr null
 21233|  br label %1587                                                                                                        ;L112
 21234| 
 21235| 328: ; preds = %322, %316, %307, %301, %292, %286, %277, %271, %262, %253
 21236|  %329 = phi i64 [ 8, %262 ], [ 8, %253 ], [ 16, %271 ], [ 16, %277 ], [ 24, %286 ], [ 24, %292 ], [ 32, %301 ], [ 32, %307 ], [ 40, %316 ], [ 40, %322 ] ;L656<185<2493<138<2971<98<107<2706<3416<3387<110
 21237|  %330 = phi ptr [ %251, %262 ], [ %251, %253 ], [ %269, %271 ], [ %269, %277 ], [ %284, %286 ], [ %284, %292 ], [ %299, %301 ], [ %299, %307 ], [ %314, %316 ], [ %314, %322 ] ;L2494<138<2971<98<107<2706<3416<3387<110
 21238|  %331 = gep %245, i64 %329
 21239|  store ptr %331, ptr %18, , !!30715                                                                                    ;L185<2493<138<2971<98<107<2706<3416<3387<110
 21240|     ;; self = ptr %330
 21241|     ;; f = ptr %18
 21242|     ;; self = ptr %18
 21243|     ;; x = ptr %330
 21244|     ;; args = ptr %330
 21246|     ;; x = ptr %330
 21250|     ;; self = ptr %330
 21251|     ;; other = ptr %68
 21252|  %332 = gep %330, i64 1632                                                                                             ;L2158<110<3379<310<1162<107<2706<3416<3387<110
 21253|  %333 = load i64, ptr %332, , !!30768, !!8                                                                             ;L2158<110<3379<310<1162<107<2706<3416<3387<110
 21254|     ;; x1 = i64 %333
 21255|     ;; self = i64 %333
 21256|  %334 = gep %330, i64 1640                                                                                             ;L2158<110<3379<310<1162<107<2706<3416<3387<110
 21257|  %335 = load i64, ptr %334, , !!30768, !!8                                                                             ;L2158<110<3379<310<1162<107<2706<3416<3387<110
 21258|     ;; y1 = i64 %335
 21259|     ;; self = i64 %335
 21260|  %336 = gep %68, i64 1632                                                                                              ;L2158<110<3379<310<1162<107<2706<3416<3387<110
 21261|  %337 = load i64, ptr %336, , !!30789, !!8                                                                             ;L2158<110<3379<310<1162<107<2706<3416<3387<110
 21262|     ;; x2 = i64 %337
 21263|     ;; other = i64 %337
 21264|  %338 = gep %68, i64 1640                                                                                              ;L2158<110<3379<310<1162<107<2706<3416<3387<110
 21265|  %339 = load i64, ptr %338, , !!30789, !!8                                                                             ;L2158<110<3379<310<1162<107<2706<3416<3387<110
 21266|     ;; y2 = i64 %339
 21267|     ;; other = i64 %339
 21268|  %340 = icmp ult i64 %333, %337                                                                                        ;L3147<7<2158<110<3379<310<1162<107<2706<3416<3387<110
 21269|  %341 = sub nuw i64 %337, %333                                                                                         ;L3147<7<2158<110<3379<310<1162<107<2706<3416<3387<110
 21270|  %342 = sub nuw i64 %333, %337                                                                                         ;L3147<7<2158<110<3379<310<1162<107<2706<3416<3387<110
 21271|  %343 = select i1 %340, i64 %341, i64 %342                                                                             ;L3147<7<2158<110<3379<310<1162<107<2706<3416<3387<110
 21272|     ;; dx = i64 %343
 21273|  %344 = icmp ult i64 %335, %339                                                                                        ;L3147<8<2158<110<3379<310<1162<107<2706<3416<3387<110
 21274|  %345 = sub nuw i64 %339, %335                                                                                         ;L3147<8<2158<110<3379<310<1162<107<2706<3416<3387<110
 21275|  %346 = sub nuw i64 %335, %339                                                                                         ;L3147<8<2158<110<3379<310<1162<107<2706<3416<3387<110
 21276|  %347 = select i1 %344, i64 %345, i64 %346                                                                             ;L3147<8<2158<110<3379<310<1162<107<2706<3416<3387<110
 21277|     ;; dy = i64 %347
 21278|  %348 = mul i64 %343, %343                                                                                             ;L9<2158<110<3379<310<1162<107<2706<3416<3387<110
 21279|  %349 = mul i64 %347, %347                                                                                             ;L9<2158<110<3379<310<1162<107<2706<3416<3387<110
 21280|  %350 = add i64 %349, %348                                                                                             ;L9<2158<110<3379<310<1162<107<2706<3416<3387<110
 21281|     ;; first[0..+8] = i64 %350
 21282|     ;; first[8..+8] = ptr %330
 21283|  %351 = invoke { i64, ptr } @core::iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtB8_10filter_map9FilterMapINtNtNtBc_5slice4iter4IterINtNtBc_6option6OptionRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENCNvMs3_B2E_NtB2E_21AbstractGameWithCache14iter_champions0ENCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan4hideNtB4y_11HideSubPlan17action_candidatess5_0ENCINvNvNtNtNtBa_6traits8iterator8Iterator10min_by_key3keyB2z_yNCB4v_s6_0E0EB6f_4foldTyB2z_ENCINvNvB6f_6min_by4foldB7u_INvB6d_7compareB2z_yEE0EB4E_(ptr %18, i64 %350, ptr %330)
 21284|  to label %1429 unwind label %57                                                                                       ;L2707<3416<3387<110
 21285| 
 21286| 352: ; preds = %222
 21287|     ;; self[0..+8] = i64 0
 21288|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.152) #31
 21289|  to label %59 unwind label %57                                                                                         ;L1013<58
 21290| 
 21291| 353: ; preds = %238
 21292|     ;; self[8..+8] = i64 %228
 21293|     ;; self[16..+8] = i64 %230
 21294|     ;; self[0..+8] = i64 1
 21295|     ;; bx = i64 %228
 21296|     ;; by = i64 %230
 21298|  %354 = mul nuw nsw i64 %228, 32000                                                                                    ;L59
 21299|  %355 = add nuw nsw i64 %354, 16000                                                                                    ;L59
 21300|  %356 = mul nuw nsw i64 %230, 32000                                                                                    ;L59
 21301|  %357 = add nuw nsw i64 %356, 16000                                                                                    ;L59
 21303|  store i64 %355, ptr %39,                                                                                              ;L59
 21305|  store i64 %357, ptr %38,                                                                                              ;L59
 21306|  %358 = getelementptr [5 x ptr], ptr %65, i64 %74                                                                      ;L1905<60
 21307|     ;; self[0..+8] = ptr %358
 21308|     ;; slice[0..+8] = ptr %358
 21309|     ;; self[8..+8] = i64 5
 21310|     ;; slice[8..+8] = i64 5
 21311|     ;; ptr = ptr %358
 21312|     ;; self = ptr %358
 21313|  %359 = gep %358, i64 40                                                                                               ;L961<100<1042<1905<60
 21314|  %360 = gep %5, i64 16                                                                                                 ;L61
 21315|  %361 = load ptr, ptr %360, , !!8, !!8                                                                                 ;L61
 21316|     ;; self[0..+8] = ptr %358
 21317|     ;; self[0..+8] = ptr %358
 21318|     ;; self[8..+8] = ptr %359
 21319|     ;; self[8..+8] = ptr %359
 21320|     ;; self[16..+8] = ptr %71
 21321|     ;; self[16..+8] = ptr %71
 21322|     ;; self[24..+8] = ptr %73
 21323|     ;; self[24..+8] = ptr %73
 21324|     ;; self[32..+8] = ptr %361
 21325|     ;; self[32..+8] = ptr %361
 21326|     ;; self[40..+8] = ptr %4
 21327|     ;; self[40..+8] = ptr %4
 21328|     ;; self[48..+8] = ptr %39
 21329|     ;; self[48..+8] = ptr %39
 21330|     ;; self[56..+8] = ptr %38
 21331|     ;; self[56..+8] = ptr %38
 21333|     ;; f = ptr %68
 21334|     ;; self = ptr %17
 21338|     ;; f = ptr %68
 21339|  %362 = gep %17, i64 8                                                                                                 ;L69<836<3387<63
 21340|  store ptr %359, ptr %362, , !!30893                                                                                   ;L69<836<3387<63
 21341|  %363 = gep %17, i64 16                                                                                                ;L69<836<3387<63
 21342|  store ptr %71, ptr %363, , !!30893                                                                                    ;L69<836<3387<63
 21343|  %364 = gep %17, i64 24                                                                                                ;L69<836<3387<63
 21344|  store ptr %73, ptr %364, , !!30893                                                                                    ;L69<836<3387<63
 21345|  %365 = gep %17, i64 32                                                                                                ;L69<836<3387<63
 21346|  store ptr %361, ptr %365, , !!30893                                                                                   ;L69<836<3387<63
 21347|  %366 = gep %17, i64 40                                                                                                ;L69<836<3387<63
 21348|  store ptr %4, ptr %366, , !!30893                                                                                     ;L69<836<3387<63
 21349|  %367 = gep %17, i64 48                                                                                                ;L69<836<3387<63
 21350|  store ptr %39, ptr %367, , !!30893                                                                                    ;L69<836<3387<63
 21351|  %368 = gep %17, i64 56                                                                                                ;L69<836<3387<63
 21352|  store ptr %38, ptr %368, , !!30893                                                                                    ;L69<836<3387<63
 21353|  %369 = gep %17, i64 64                                                                                                ;L69<836<3387<63
 21354|  store ptr %68, ptr %369, , !!30896                                                                                    ;L69<836<3387<63
 21356|     ;; self = ptr %17
 21359|     ;; self = ptr %17
 21361|     ;; self = ptr %17
 21363|     ;; self = ptr %17
 21366|     ;; self = ptr %17
 21370|     ;; self = ptr %17
 21372|     ;; fold[0..+8] = ptr %17
 21375|     ;; self = ptr %17
 21379|     ;; self = ptr %17
 21380|     ;; count = i64 1
 21381|     ;; ptr = ptr %358
 21382|     ;; self = ptr %358
 21383|     ;; end_or_len = ptr %359
 21386|     ;; fold[0..+8] = ptr %17
 21387|  %370 = getelementptr { { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, [5 x { i64, [2 x i64] }], [5 x { i64, { i8, [23 x i8] } }], [5 x { i8, [7 x i8], i64 }], [5 x i64], [5 x i64], [5 x i64], [5 x { i64, i64 }], i64, i64, i64, i64, i64, [5 x i32], i8, [3 x i8] }, ptr %361, i64 %74
 21388|  br label %371                                                                                                         ;L180<2493<138<157<2971<98<107<2706<3416<3387<63
 21389| 
 21390| 371: ; preds = %400, %353
 21391|  %372 = phi i64 [ 0, %353 ], [ %374, %400 ]
 21392|  %373 = gep %358, i64 %372                                                                                             ;L656<185<2493<138<157<2971<98<107<2706<3416<3387<63
 21393|     ;; ptr = ptr %373
 21394|  %374 = add nuw nsw i64 %372, 8                                                                                        ;L656<185<2493<138<157<2971<98<107<2706<3416<3387<63
 21395|     ;; x = ptr %373
 21396|  %375 = gep %358, i64 %374                                                                                             ;L2494<138<157<2971<98<107<2706<3416<3387<63
 21397|  %376 = load ptr, ptr %373, , !!31040, !!8                                                                             ;L2494<138<157<2971<98<107<2706<3416<3387<63
 21402|  %377 = icmp eq ptr %376, null                                                                                         ;L49<2494<138<157<2971<98<107<2706<3416<3387<63
 21403|  br i1 %377, label %400, label %378                                                                                    ;L49<2494<138<157<2971<98<107<2706<3416<3387<63
 21404| 
 21405| 378: ; preds = %371
 21406|     ;; x = ptr %376
 21407|     ;; item = ptr %376
 21414|  %379 = invoke zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %370, ptr %71, ptr %73, ptr %4, ptr %376)
 21415|  to label %380 unwind label %57                                                                                        ;L61<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21416| 
 21417| 380: ; preds = %378
 21418|  br i1 %379, label %381, label %400                                                                                    ;L86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21419| 
 21420| 381: ; preds = %380
 21422|     ;; x = ptr %376
 21430|  %382 = gep %376, i64 1632                                                                                             ;L62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21431|  %383 = load i64, ptr %382, , !!31040, !!8                                                                             ;L62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21432|     ;; x1 = i64 %383
 21433|     ;; self = i64 %383
 21434|  %384 = gep %376, i64 1640                                                                                             ;L62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21435|  %385 = load i64, ptr %384, , !!31040, !!8                                                                             ;L62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21436|     ;; y1 = i64 %385
 21437|     ;; self = i64 %385
 21438|  %386 = load i64, ptr %39, , !!31146, !!8                                                                              ;L62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21439|     ;; x2 = i64 %386
 21440|     ;; other = i64 %386
 21441|  %387 = load i64, ptr %38, , !!31146, !!8                                                                              ;L62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21442|     ;; y2 = i64 %387
 21443|     ;; other = i64 %387
 21444|  %388 = icmp ult i64 %383, %386                                                                                        ;L3147<7<62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21445|  %389 = sub nuw i64 %386, %383                                                                                         ;L3147<7<62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21446|  %390 = sub nuw i64 %383, %386                                                                                         ;L3147<7<62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21447|  %391 = select i1 %388, i64 %389, i64 %390                                                                             ;L3147<7<62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21448|     ;; dx = i64 %391
 21449|  %392 = icmp ult i64 %385, %387                                                                                        ;L3147<8<62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21450|  %393 = sub nuw i64 %387, %385                                                                                         ;L3147<8<62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21451|  %394 = sub nuw i64 %385, %387                                                                                         ;L3147<8<62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21452|  %395 = select i1 %392, i64 %393, i64 %394                                                                             ;L3147<8<62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21453|     ;; dy = i64 %395
 21454|  %396 = mul i64 %391, %391                                                                                             ;L9<62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21455|  %397 = mul i64 %395, %395                                                                                             ;L9<62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21456|  %398 = add i64 %397, %396                                                                                             ;L9<62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21457|  %399 = icmp ult i64 %398, 62500000001                                                                                 ;L62<298<2967<86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21458|  br i1 %399, label %402, label %400                                                                                    ;L86<50<2494<138<157<2971<98<107<2706<3416<3387<63
 21459| 
 21460| 400: ; preds = %381, %380, %371
 21461|     ;; self = ptr %17
 21462|     ;; count = i64 1
 21463|     ;; ptr = ptr %375
 21464|     ;; self = ptr %375
 21465|     ;; end_or_len = ptr %359
 21468|  %401 = icmp eq i64 %374, 40                                                                                           ;L1714<180<2493<138<157<2971<98<107<2706<3416<3387<63
 21469|  br i1 %401, label %422, label %371                                                                                    ;L180<2493<138<157<2971<98<107<2706<3416<3387<63
 21470| 
 21471| 402: ; preds = %381
 21472|  store ptr %375, ptr %17, , !!31157                                                                                    ;L185<2493<138<157<2971<98<107<2706<3416<3387<63
 21473|     ;; self = ptr %376
 21474|     ;; f = ptr %17
 21475|     ;; self = ptr %17
 21476|     ;; x = ptr %376
 21477|     ;; args = ptr %376
 21478|     ;; x = ptr %376
 21482|     ;; self = ptr %376
 21483|     ;; other = ptr %68
 21484|     ;; x1 = i64 %383
 21485|     ;; self = i64 %383
 21486|     ;; y1 = i64 %385
 21487|     ;; self = i64 %385
 21488|  %403 = gep %68, i64 1632                                                                                              ;L2158<63<3379<310<1162<107<2706<3416<3387<63
 21489|  %404 = load i64, ptr %403, , !!31226, !!8                                                                             ;L2158<63<3379<310<1162<107<2706<3416<3387<63
 21490|     ;; x2 = i64 %404
 21491|     ;; other = i64 %404
 21492|  %405 = gep %68, i64 1640                                                                                              ;L2158<63<3379<310<1162<107<2706<3416<3387<63
 21493|  %406 = load i64, ptr %405, , !!31226, !!8                                                                             ;L2158<63<3379<310<1162<107<2706<3416<3387<63
 21494|     ;; y2 = i64 %406
 21495|     ;; other = i64 %406
 21496|  %407 = icmp ult i64 %383, %404                                                                                        ;L3147<7<2158<63<3379<310<1162<107<2706<3416<3387<63
 21497|  %408 = sub nuw i64 %404, %383                                                                                         ;L3147<7<2158<63<3379<310<1162<107<2706<3416<3387<63
 21498|  %409 = sub nuw i64 %383, %404                                                                                         ;L3147<7<2158<63<3379<310<1162<107<2706<3416<3387<63
 21499|  %410 = select i1 %407, i64 %408, i64 %409                                                                             ;L3147<7<2158<63<3379<310<1162<107<2706<3416<3387<63
 21500|     ;; dx = i64 %410
 21501|  %411 = icmp ult i64 %385, %406                                                                                        ;L3147<8<2158<63<3379<310<1162<107<2706<3416<3387<63
 21502|  %412 = sub nuw i64 %406, %385                                                                                         ;L3147<8<2158<63<3379<310<1162<107<2706<3416<3387<63
 21503|  %413 = sub nuw i64 %385, %406                                                                                         ;L3147<8<2158<63<3379<310<1162<107<2706<3416<3387<63
 21504|  %414 = select i1 %411, i64 %412, i64 %413                                                                             ;L3147<8<2158<63<3379<310<1162<107<2706<3416<3387<63
 21505|     ;; dy = i64 %414
 21506|  %415 = mul i64 %410, %410                                                                                             ;L9<2158<63<3379<310<1162<107<2706<3416<3387<63
 21507|  %416 = mul i64 %414, %414                                                                                             ;L9<2158<63<3379<310<1162<107<2706<3416<3387<63
 21508|  %417 = add i64 %416, %415                                                                                             ;L9<2158<63<3379<310<1162<107<2706<3416<3387<63
 21509|     ;; first[0..+8] = i64 %417
 21510|     ;; first[8..+8] = ptr %376
 21511|  %418 = invoke { i64, ptr } @core::iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterIBY_INtNtB8_10filter_map9FilterMapINtNtNtBc_5slice4iter4IterINtNtBc_6option6OptionRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENCNvMs3_B2I_NtB2I_21AbstractGameWithCache14iter_champions0ENCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan4hideNtB4C_11HideSubPlan17action_candidatess0_0ENCB4z_s1_0ENCINvNvNtNtNtBa_6traits8iterator8Iterator10min_by_key3keyB2D_yNCB4z_s2_0E0EB6u_4foldTyB2D_ENCINvNvB6u_6min_by4foldB7J_INvB6s_7compareB2D_yEE0EB4I_(ptr %17, i64 %417, ptr %376)
 21512|  to label %419 unwind label %57                                                                                        ;L2707<3416<3387<63
 21513| 
 21514| 419: ; preds = %402
 21515|  %420 = extractvalue { i64, ptr } %418, 1                                                                              ;L2707<3416<3387<63
 21517|     ;; nearest_enemy = ptr %420
 21518|  %421 = icmp eq ptr %420, null                                                                                         ;L0
 21519|  br i1 %79, label %427, label %425                                                                                     ;L66
 21520| 
 21521| 422: ; preds = %400
 21523|     ;; nearest_enemy = ptr null
 21524|  br i1 %79, label %1042, label %423                                                                                    ;L66
 21525| 
 21526| 423: ; preds = %422
 21527|  %424 = load i64, ptr %1,                                                                                              ;L96
 21528|  br label %431                                                                                                         ;L66
 21529| 
 21530| 425: ; preds = %419
 21531|  %426 = load i64, ptr %1,                                                                                              ;L96
 21532|  br i1 %421, label %431, label %428                                                                                    ;L93
 21533| 
 21534| 427: ; preds = %419
 21535|  br i1 %421, label %1042, label %886                                                                                   ;L67
 21536| 
 21537| 428: ; preds = %425
 21538|     ;; nearest_enemy = ptr %420
 21541|  %429 = gep %1, i64 8                                                                                                  ;L94
 21542|  %430 = load i8, ptr %429, , !!8                                                                                       ;L94
 21543|  invoke void @ai::small_action6aroundNtB5_21SmallActionAroundBush15new_with_target(ptr sret([120 x i8]) %29, ptr %5, ptr %420, i64 %426, i8 %430)
 21544|  to label %435 unwind label %57                                                                                        ;L94
 21545| 
 21546| 431: ; preds = %425, %423
 21547|  %432 = phi i64 [ %424, %423 ], [ %426, %425 ]                                                                         ;L96
 21550|  %433 = gep %1, i64 8                                                                                                  ;L96
 21551|  %434 = load i8, ptr %433, , !!8                                                                                       ;L96
 21552|  invoke void @ai::small_action6aroundNtB5_21SmallActionAroundBush17new_with_out_line(ptr sret([120 x i8]) %27, ptr %3, ptr %5, ptr %4, i64 %432, i8 %434)
 21553|  to label %871 unwind label %57                                                                                        ;L96
 21554| 
 21555| 435: ; preds = %428
 21556|  call void @llvm.memcpy.p0.p0.i64(ptr %30, ptr %29, i64 120, i1 false)                                                 ;L94
 21557|  %436 = gep %30, i64 177                                                                                               ;L94
 21558|  store i8 12, ptr %436,                                                                                                ;L94
 21561|     ;; self = ptr %46
 21562|     ;; self = ptr %46
 21563|     ;; value = ptr %30
 21564|     ;; src = ptr %30
 21565|     ;; additional = i64 1
 21566|     ;; needed_extra_cap = i64 1
 21567|     ;; needed_extra_cap = i64 1
 21568|     ;; strategy = i8 1
 21569|     ;; self = ptr %46
 21570|  %437 = load i64, ptr %51, , !!31260, !!8                                                                              ;L149<1428<94
 21571|  %438 = icmp eq i64 %213, %437                                                                                         ;L1428<94
 21572|  br i1 %438, label %439, label %445                                                                                    ;L1428<94
 21573| 
 21574| 439: ; preds = %435
 21575|     ;; self = ptr %46
 21576|     ;; self = ptr %46
 21577|     ;; self = ptr %46
 21578|     ;; used_cap = i64 %213
 21579|     ;; used_cap = i64 %213
 21580|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %46, i64 %213, i64 1, i1 zeroext true)
 21581|  to label %440 unwind label %443, !!31260                                                                              ;L619<430<738<1429<94
 21582| 
 21583| 440: ; preds = %439
 21584|  %441 = load i64, ptr %52, , !!31260                                                                                   ;L1432<94
 21585|  %442 = load ptr, ptr %46, , !!31260                                                                                   ;L138<1432<94
 21586|  br label %445                                                                                                         ;L619<430<738<1429<94
 21587| 
 21588| 443: ; preds = %439
 21589|  %444 = cleanuppad within none []
 21590|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %30) #30 [ "funclet"(token %444) ], !!31243 ;L1436<94
 21591|  cleanupret from %444 unwind label %57
 21592| 
 21593| 445: ; preds = %440, %435
 21594|  %446 = phi ptr [ %442, %440 ], [ %214, %435 ]                                                                         ;L138<1432<94
 21595|  %447 = phi i64 [ %441, %440 ], [ %213, %435 ]                                                                         ;L1432<94
 21596|     ;; self = ptr %46
 21597|     ;; self = ptr %446
 21598|     ;; count = i64 %447
 21599|  %448 = gepS %446, i64 %447                                                                                            ;L961<1432<94
 21600|     ;; end = ptr %448
 21601|     ;; dst = ptr %448
 21602|  call void @llvm.memcpy.p0.p0.i64(ptr %448, ptr %30, i64 184, i1 false), !!31243                                       ;L1933<1433<94
 21603|  %449 = add i64 %447, 1                                                                                                ;L1434<94
 21604|  store i64 %449, ptr %52, , !!31260                                                                                    ;L1434<94
 21606|  br label %454                                                                                                         ;L93
 21607| 
 21608| 450: ; preds = %864, %854, %787, %784, %779, %758, %748, %682, %679, %675, %666, %656, %646, %580, %577, %568, %564, %487, %480, %478
 21609|  %451 = cleanuppad within none []
 21610|  invoke fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %16) #30 [ "funclet"(token %451) ]
 21611|  to label %452 unwind label %57                                                                                        ;L219<101
 21612| 
 21613| 452: ; preds = %450
 21614|  cleanupret from %451 unwind label %57
 21615| 
 21616| 453: ; preds = %487
 21617|  unreachable
 21618| 
 21619| 454: ; preds = %881, %445
 21621|     ;; self = ptr %1
 21624|     ;; data = ptr %5
 21625|     ;; res = ptr %16
 21626|     ;; steal_range = i64 150000
 21630|     ;; count = i64 1
 21632|  %455 = load ptr, ptr %48, , !!31351, !!8, !!8                                                                         ;L143<101
 21633|     ;; bump = ptr %455
 21634|  store ptr inttoptr (i64 8 to ptr), ptr %16, , !!31351                                                                 ;L547<143<101
 21635|  %456 = gep %16, i64 8                                                                                                 ;L547<143<101
 21636|  store ptr %455, ptr %456, , !!31351                                                                                   ;L547<143<101
 21637|  %457 = gep %16, i64 16                                                                                                ;L547<143<101
 21638|  %458 = gep %16, i64 24                                                                                                ;L547<143<101
 21639|  call void @llvm.memset.p0.i64(ptr %457, i8 0, i64 16, i1 false), !!31351                                              ;L547<143<101
 21640|  %459 = load ptr, ptr %67, , !!31361, !!8                                                                              ;L144<101
 21641|     ;; self = ptr %459
 21642|  %460 = icmp eq ptr %459, null                                                                                         ;L1011<144<101
 21643|  br i1 %460, label %487, label %461                                                                                    ;L1011<144<101
 21644| 
 21645| 461: ; preds = %454
 21646|     ;; champ = ptr %459
 21647|     ;; caster = ptr %459
 21648|     ;; self = ptr %459
 21649|     ;; other = ptr %459
 21650|     ;; caster = ptr %459
 21651|     ;; self = ptr %459
 21652|     ;; other = ptr %459
 21653|     ;; self = ptr %459
 21654|     ;; caster = ptr %459
 21655|     ;; self = ptr %459
 21656|     ;; other = ptr %459
 21658|     ;; self[0..+8] = i64 0
 21659|     ;; self[8..+8] = i64 71
 21660|  %462 = load ptr, ptr %216, , !!31361, !!8, !!8                                                                        ;L147<101
 21661|     ;; predicate[0..+8] = ptr %462
 21662|     ;; predicate[8..+8] = ptr %1
 21663|  %463 = gep %15, i64 32                                                                                                ;L28<957<147<101
 21664|  call void @llvm.memcpy.p0.p0.i64(ptr %463, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.148, i64 1136, i1 false), !!31351 ;L28<957<147<101
 21665|     ;; self = ptr %15
 21666|     ;; self = ptr %15
 21667|     ;; self = ptr %15
 21668|     ;; predicate = ptr %15
 21669|     ;; f = ptr %15
 21670|     ;; f = ptr %15
 21671|     ;; self[0..+8] = ptr %15
 21672|     ;; self[8..+8] = i64 71
 21673|     ;; data[0..+8] = ptr %463
 21674|     ;; data[8..+8] = i64 71
 21675|     ;; f[0..+8] = ptr %463
 21676|     ;; f[8..+8] = i64 71
 21677|     ;; f = ptr %15
 21678|     ;; f[16..+8] = ptr %15
 21679|     ;; self = ptr %15
 21680|     ;; self = ptr %15
 21682|     ;; rhs = i64 1
 21683|  %464 = gep %462, i64 7320
 21684|  %465 = load i64, ptr %1, , !!31475
 21685|  br label %468                                                                                                         ;L167<215<263<2971<98<148<101
 21686| 
 21687| 466: ; preds = %482
 21688|  %467 = icmp eq i64 %470, 71                                                                                           ;L167<215<263<2971<98<148<101
 21689|  br i1 %467, label %497, label %468                                                                                    ;L167<215<263<2971<98<148<101
 21690| 
 21691| 468: ; preds = %466, %461
 21692|  %469 = phi i64 [ 0, %461 ], [ %470, %466 ]
 21693|     ;; i = i64 %469
 21694|     ;; value = i64 %469
 21695|     ;; self = i64 %469
 21696|  %470 = add nuw nsw i64 %469, 1                                                                                        ;L971<63<169<215<263<2971<98<148<101
 21698|     ;; f = ptr undef
 21700|     ;; idx = i64 %469
 21701|     ;; index = i64 %469
 21702|     ;; self = i64 %469
 21703|     ;; self[0..+8] = ptr %463
 21704|     ;; slice[0..+8] = ptr %463
 21705|     ;; self[8..+8] = i64 71
 21706|     ;; slice[8..+8] = i64 71
 21707|  %471 = gepS %463, i64 %469                                                                                            ;L253<646<219<170<215<263<2971<98<148<101
 21708|     ;; self = ptr %471
 21709|     ;; self = ptr %471
 21710|     ;; src = ptr %471
 21711|  %472 = load i64, ptr %471, , !!31509, !!8                                                                             ;L1733<1171<798<219<170<215<263<2971<98<148<101
 21712|  %473 = gep %471, i64 8                                                                                                ;L1733<1171<798<219<170<215<263<2971<98<148<101
 21713|  %474 = load i64, ptr %473, , !!31509, !!8                                                                             ;L1733<1171<798<219<170<215<263<2971<98<148<101
 21714|     ;; elem[0..+8] = i64 %472
 21715|     ;; elem[8..+8] = i64 %474
 21716|     ;; x[0..+8] = i64 %472
 21717|     ;; x[8..+8] = i64 %474
 21727|  %475 = icmp ult i64 %474, 30                                                                                          ;L147<298<2967<220<170<215<263<2971<98<148<101
 21728|  br i1 %475, label %476, label %478                                                                                    ;L147<298<2967<220<170<215<263<2971<98<148<101
 21729| 
 21730| 476: ; preds = %468
 21731|  %477 = icmp ult i64 %472, 30                                                                                          ;L147<298<2967<220<170<215<263<2971<98<148<101
 21732|  br i1 %477, label %482, label %480                                                                                    ;L147<298<2967<220<170<215<263<2971<98<148<101
 21733| 
 21734| 478: ; preds = %468
 21735|  invoke void @core::panicking18panic_bounds_check(i64 %474, i64 30, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.42) #31
 21736|  to label %479 unwind label %450, !!31361                                                                              ;L147<298<2967<220<170<215<263<2971<98<148<101
 21737| 
 21738| 479: ; preds = %478
 21739|  unreachable                                                                                                           ;L147<298<2967<220<170<215<263<2971<98<148<101
 21740| 
 21741| 480: ; preds = %476
 21742|  invoke void @core::panicking18panic_bounds_check(i64 %472, i64 30, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.42) #31
 21743|  to label %481 unwind label %450, !!31361                                                                              ;L147<298<2967<220<170<215<263<2971<98<148<101
 21744| 
 21745| 481: ; preds = %480
 21746|  unreachable                                                                                                           ;L147<298<2967<220<170<215<263<2971<98<148<101
 21747| 
 21748| 482: ; preds = %476
 21749|  %483 = getelementptr [30 x i64], ptr %464, i64 %474                                                                   ;L147<298<2967<220<170<215<263<2971<98<148<101
 21750|  %484 = getelementptr i64, ptr %483, i64 %472                                                                          ;L147<298<2967<220<170<215<263<2971<98<148<101
 21751|  %485 = load i64, ptr %484, , !!31555, !!8                                                                             ;L147<298<2967<220<170<215<263<2971<98<148<101
 21752|  %486 = icmp eq i64 %485, %465                                                                                         ;L147<298<2967<220<170<215<263<2971<98<148<101
 21753|  br i1 %486, label %488, label %466                                                                                    ;L2967<220<170<215<263<2971<98<148<101
 21754| 
 21755| 487: ; preds = %454
 21756|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.153) #31
 21757|  to label %453 unwind label %450, !!31361                                                                              ;L1013<144<101
 21758| 
 21759| 488: ; preds = %482
 21760|     ;; self[8..+8] = i64 %472
 21761|     ;; self[16..+8] = i64 %474
 21762|     ;; self[0..+8] = i64 1
 21763|     ;; x[0..+8] = i64 %472
 21764|     ;; x = i64 %472
 21765|     ;; x[8..+8] = i64 %474
 21766|     ;; y = i64 %474
 21767|  %489 = mul nuw nsw i64 %472, 32000                                                                                    ;L149<1162<149<101
 21768|  %490 = add nuw nsw i64 %489, 16000                                                                                    ;L149<1162<149<101
 21769|  %491 = mul nuw nsw i64 %474, 32000                                                                                    ;L149<1162<149<101
 21770|  %492 = add nuw nsw i64 %491, 16000                                                                                    ;L149<1162<149<101
 21771|     ;; self[8..+8] = i64 %490
 21772|     ;; self[16..+8] = i64 %492
 21773|     ;; self[0..+8] = i64 1
 21774|  %493 = gep %459, i64 1632                                                                                             ;L150<101
 21775|  %494 = load i64, ptr %493, , !!31361, !!8                                                                             ;L150<101
 21776|  %495 = gep %459, i64 1640                                                                                             ;L150<101
 21777|  %496 = load i64, ptr %495, , !!31361, !!8                                                                             ;L150<101
 21778|     ;; default[0..+8] = i64 %494
 21779|     ;; default[8..+8] = i64 %496
 21780|     ;; bx = i64 %490
 21781|     ;; x2 = i64 %490
 21782|     ;; other = i64 %490
 21783|     ;; by = i64 %492
 21784|     ;; y2 = i64 %492
 21785|     ;; other = i64 %492
 21786|  br label %502                                                                                                         ;L1043<150<101
 21787| 
 21788| 497: ; preds = %466
 21789|     ;; self[0..+8] = i64 0
 21790|     ;; self[0..+8] = i64 0
 21791|  %498 = gep %459, i64 1632                                                                                             ;L150<101
 21792|  %499 = load i64, ptr %498, , !!31361, !!8                                                                             ;L150<101
 21793|  %500 = gep %459, i64 1640                                                                                             ;L150<101
 21794|  %501 = load i64, ptr %500, , !!31361, !!8                                                                             ;L150<101
 21795|     ;; default[0..+8] = i64 %499
 21796|     ;; default[8..+8] = i64 %501
 21797|     ;; bx = i64 %499
 21798|     ;; x2 = i64 %499
 21799|     ;; other = i64 %499
 21800|     ;; by = i64 %501
 21801|     ;; y2 = i64 %501
 21802|     ;; other = i64 %501
 21803|  br label %502                                                                                                         ;L1041<150<101
 21804| 
 21805| 502: ; preds = %497, %488
 21806|  %503 = phi i64 [ %496, %488 ], [ %501, %497 ]                                                                         ;L150<101
 21807|  %504 = phi i64 [ %494, %488 ], [ %499, %497 ]                                                                         ;L150<101
 21808|  %505 = phi i64 [ %490, %488 ], [ %499, %497 ]                                                                         ;L0<150<101
 21809|  %506 = phi i64 [ %492, %488 ], [ %501, %497 ]                                                                         ;L0<150<101
 21810|     ;; other = i64 %506
 21811|     ;; y2 = i64 %506
 21812|     ;; by = i64 %506
 21813|     ;; other = i64 %505
 21814|     ;; x2 = i64 %505
 21815|     ;; bx = i64 %505
 21817|     ;; enemy_team = i64 %74
 21818|     ;; team = i64 %74
 21819|     ;; self = ptr %64
 21820|     ;; self = ptr %64
 21821|  %507 = gep %64, i64 208                                                                                               ;L138<2073<156<101
 21822|  %508 = load ptr, ptr %507, , !!31361, !!8, !!8                                                                        ;L138<2073<156<101
 21823|     ;; p = ptr %508
 21824|  %509 = gep %64, i64 232                                                                                               ;L2075<156<101
 21825|  %510 = load i64, ptr %509, , !!31361, !!8                                                                             ;L2075<156<101
 21826|     ;; len = i64 %510
 21827|     ;; count = i64 %510
 21828|     ;; self[0..+8] = ptr %508
 21829|     ;; slice[0..+8] = ptr %508
 21830|     ;; self[8..+8] = i64 %510
 21831|     ;; slice[8..+8] = i64 %510
 21832|     ;; ptr = ptr %508
 21833|     ;; self = ptr %508
 21834|  %511 = getelementptr ptr, ptr %508, i64 %510                                                                          ;L961<100<1042<156<101
 21835|     ;; iter[0..+8] = ptr %508
 21836|     ;; iter[8..+8] = ptr %511
 21837|  %512 = gep %459, i64 1600
 21838|  %513 = gep %459, i64 1168
 21839|  %514 = gep %459, i64 1216
 21840|  %515 = gep %459, i64 1184
 21841|  %516 = gep %459, i64 1192
 21842|  %517 = gep %459, i64 1480
 21843|  %518 = gep %459, i64 1080
 21844|  %519 = gep %459, i64 1136
 21845|  %520 = gep %459, i64 1664
 21846|  %521 = gep %14, i64 177
 21847|  %522 = gep %459, i64 1224
 21848|  %523 = gep %459, i64 1272
 21849|  %524 = gep %459, i64 1264
 21850|  %525 = gep %459, i64 1240
 21851|  %526 = gep %459, i64 1248
 21852|  %527 = gep %12, i64 177
 21853|  %528 = gep %459, i64 1280
 21854|  %529 = gep %10, i64 177
 21855|  br label %530                                                                                                         ;L156<101
 21856| 
 21857| 530: ; preds = %767, %502
 21858|  %531 = phi ptr [ inttoptr (i64 8 to ptr), %502 ], [ %768, %767 ]
 21859|  %532 = phi ptr [ inttoptr (i64 8 to ptr), %502 ], [ %769, %767 ]
 21860|  %533 = phi ptr [ inttoptr (i64 8 to ptr), %502 ], [ %770, %767 ]
 21861|  %534 = phi i64 [ 0, %502 ], [ %771, %767 ]
 21862|  %535 = phi ptr [ %508, %502 ], [ %538, %767 ]                                                                         ;L156<101
 21863|     ;; iter[0..+8] = ptr %535
 21864|     ;; self = ptr undef
 21865|     ;; ptr = ptr %535
 21866|     ;; self = ptr %535
 21867|     ;; end_or_len = ptr %511
 21870|  %536 = icmp eq ptr %535, %511                                                                                         ;L1714<180<156<101
 21871|  br i1 %536, label %1427, label %537                                                                                   ;L180<156<101
 21872| 
 21873| 537: ; preds = %530
 21874|  %538 = gep %535, i64 8                                                                                                ;L656<185<156<101
 21875|     ;; iter[0..+8] = ptr %538
 21876|     ;; jungle = ptr %535
 21877|  %539 = load ptr, ptr %535, , !!31361, !!8, !!8                                                                        ;L158<101
 21878|     ;; self = ptr %539
 21879|  %540 = gep %539, i64 104                                                                                              ;L1378<158<101
 21880|  %541 = load i64, ptr %540, , !!31361, !!8                                                                             ;L1378<158<101
 21881|  %542 = icmp eq i64 %541, 4                                                                                            ;L1378<158<101
 21882|  br i1 %542, label %543, label %767                                                                                    ;L1378<158<101
 21883| 
 21884| 543: ; preds = %537
 21885|  %544 = gep %539, i64 152                                                                                              ;L1378<158<101
 21886|  %545 = load i64, ptr %544, , !!31361, !!8                                                                             ;L1378<158<101
 21887|  %546 = icmp eq i64 %545, %74                                                                                          ;L1378<158<101
 21888|  br i1 %546, label %547, label %767                                                                                    ;L1378<158<101
 21889| 
 21890| 547: ; preds = %543
 21891|  %548 = gep %539, i64 1632                                                                                             ;L163<101
 21892|  %549 = load i64, ptr %548, , !!31361, !!8                                                                             ;L163<101
 21893|     ;; x1 = i64 %549
 21894|     ;; self = i64 %549
 21895|  %550 = gep %539, i64 1640                                                                                             ;L163<101
 21896|  %551 = load i64, ptr %550, , !!31361, !!8                                                                             ;L163<101
 21897|     ;; y1 = i64 %551
 21898|     ;; self = i64 %551
 21899|  %552 = icmp ult i64 %549, %505                                                                                        ;L3147<7<163<101
 21900|  %553 = sub nuw i64 %505, %549                                                                                         ;L3147<7<163<101
 21901|  %554 = sub nuw i64 %549, %505                                                                                         ;L3147<7<163<101
 21902|  %555 = select i1 %552, i64 %553, i64 %554                                                                             ;L3147<7<163<101
 21903|     ;; dx = i64 %555
 21904|  %556 = icmp ult i64 %551, %506                                                                                        ;L3147<8<163<101
 21905|  %557 = sub nuw i64 %506, %551                                                                                         ;L3147<8<163<101
 21906|  %558 = sub nuw i64 %551, %506                                                                                         ;L3147<8<163<101
 21907|  %559 = select i1 %556, i64 %557, i64 %558                                                                             ;L3147<8<163<101
 21908|     ;; dy = i64 %559
 21909|  %560 = mul i64 %555, %555                                                                                             ;L9<163<101
 21910|  %561 = mul i64 %559, %559                                                                                             ;L9<163<101
 21911|  %562 = add i64 %561, %560                                                                                             ;L9<163<101
 21912|  %563 = icmp ugt i64 %562, 22500000000                                                                                 ;L163<101
 21913|  br i1 %563, label %767, label %564                                                                                    ;L163<101
 21914| 
 21915| 564: ; preds = %547
 21916|  %565 = load i64, ptr %512, , !!31361, !!8                                                                             ;L168<101
 21917|     ;; move_speed = i64 %565
 21918|  %566 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_attack(ptr %459)
 21919|  to label %567 unwind label %450, !!31361                                                                              ;L171<101
 21920| 
 21921| 567: ; preds = %564
 21922|  br i1 %566, label %574, label %568                                                                                    ;L171<101
 21923| 
 21924| 568: ; preds = %658, %616, %574, %567
 21925|  %569 = phi ptr [ %531, %574 ], [ %531, %616 ], [ %659, %658 ], [ %531, %567 ]
 21926|  %570 = phi ptr [ %532, %574 ], [ %532, %616 ], [ %660, %658 ], [ %532, %567 ]
 21927|  %571 = phi ptr [ %533, %574 ], [ %533, %616 ], [ %661, %658 ], [ %533, %567 ]
 21928|  %572 = phi i64 [ %534, %574 ], [ %534, %616 ], [ %664, %658 ], [ %534, %567 ]
 21929|  %573 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity9can_skill(ptr %459)
 21930|  to label %665 unwind label %450, !!31361                                                                              ;L186<101
 21931| 
 21932| 574: ; preds = %567
 21933|     ;; self = ptr %459
 21934|  %575 = load i32, ptr %514, , !!31361, !!8                                                                             ;L742<172<101
 21935|  %576 = icmp eq i32 %575, -1                                                                                           ;L742<172<101
 21936|  br i1 %576, label %568, label %577                                                                                    ;L742<172<101
 21937| 
 21938| 577: ; preds = %574
 21939|     ;; atk = ptr %513
 21940|     ;; self = ptr %513
 21941|  %578 = load ptr, ptr %535, , !!31361, !!8, !!8                                                                        ;L173<101
 21942|  %579 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %513, ptr %48, ptr %459, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %578)
 21943|  to label %580 unwind label %450, !!31361                                                                              ;L173<101
 21944| 
 21945| 580: ; preds = %577
 21946|     ;; expected_dmg = i64 %579
 21947|  %581 = load i64, ptr %515, , !!31361, !!8                                                                             ;L26<174<101
 21948|  %582 = load i64, ptr %516, , !!31361, !!8                                                                             ;L26<174<101
 21949|  %583 = load i64, ptr %517, , !!31361, !!8                                                                             ;L26<174<101
 21950|  %584 = load i64, ptr %518, , !!31361, !!8                                                                             ;L26<174<101
 21951|  %585 = load ptr, ptr %535, , !!31361, !!8, !!8                                                                        ;L174<101
 21952|  %586 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %513, ptr %459, ptr %585)
 21953|  to label %587 unwind label %450, !!31361                                                                              ;L174<101
 21954| 
 21955| 587: ; preds = %580
 21956|  %588 = add i64 %583, -1                                                                                               ;L26<174<101
 21957|  %589 = mul i64 %588, %582                                                                                             ;L26<174<101
 21958|  %590 = load i32, ptr %519, , !!31361, !!8                                                                             ;L1511<174<101
 21959|     ;; mult = i32 %590
 21960|  %591 = icmp eq i32 %590, 0                                                                                            ;L1512<174<101
 21961|  br i1 %591, label %592, label %594                                                                                    ;L1512<174<101
 21962| 
 21963| 592: ; preds = %587
 21964|  %593 = load i64, ptr %520, , !!31361, !!8                                                                             ;L1513<174<101
 21965|  br label %600                                                                                                         ;L1512<174<101
 21966| 
 21967| 594: ; preds = %587
 21968|  %595 = sext i32 %590 to i64                                                                                           ;L1511<174<101
 21969|     ;; mult = i64 %595
 21970|  %596 = load i64, ptr %520, , !!31361, !!8                                                                             ;L1515<174<101
 21971|  %597 = add nsw i64 %595, 100                                                                                          ;L1515<174<101
 21972|  %598 = mul i64 %596, %597                                                                                             ;L1515<174<101
 21973|  %599 = udiv i64 %598, 100                                                                                             ;L1515<174<101
 21974|  br label %600                                                                                                         ;L1512<174<101
 21975| 
 21976| 600: ; preds = %594, %592
 21977|  %601 = phi i64 [ %593, %592 ], [ %599, %594 ]                                                                         ;L0<174<101
 21978|  %602 = load ptr, ptr %535, , !!31361, !!8, !!8                                                                        ;L174<101
 21979|     ;; self = ptr %602
 21980|  %603 = gep %602, i64 1136                                                                                             ;L1511<174<101
 21981|  %604 = load i32, ptr %603, , !!31361, !!8                                                                             ;L1511<174<101
 21982|     ;; mult = i32 %604
 21983|  %605 = icmp eq i32 %604, 0                                                                                            ;L1512<174<101
 21984|  br i1 %605, label %606, label %609                                                                                    ;L1512<174<101
 21985| 
 21986| 606: ; preds = %600
 21987|  %607 = gep %602, i64 1664                                                                                             ;L1513<174<101
 21988|  %608 = load i64, ptr %607, , !!31361, !!8                                                                             ;L1513<174<101
 21989|  br label %616                                                                                                         ;L1512<174<101
 21990| 
 21991| 609: ; preds = %600
 21992|  %610 = sext i32 %604 to i64                                                                                           ;L1511<174<101
 21993|     ;; mult = i64 %610
 21994|  %611 = gep %602, i64 1664                                                                                             ;L1515<174<101
 21995|  %612 = load i64, ptr %611, , !!31361, !!8                                                                             ;L1515<174<101
 21996|  %613 = add nsw i64 %610, 100                                                                                          ;L1515<174<101
 21997|  %614 = mul i64 %612, %613                                                                                             ;L1515<174<101
 21998|  %615 = udiv i64 %614, 100                                                                                             ;L1515<174<101
 21999|  br label %616                                                                                                         ;L1512<174<101
 22000| 
 22001| 616: ; preds = %609, %606
 22002|  %617 = phi i64 [ %608, %606 ], [ %615, %609 ]                                                                         ;L0<174<101
 22004|     ;; self = ptr %602
 22005|  %618 = gep %602, i64 1632                                                                                             ;L2158<175<101
 22006|  %619 = load i64, ptr %618, , !!31361, !!8                                                                             ;L2158<175<101
 22007|     ;; x1 = i64 %619
 22008|     ;; self = i64 %619
 22009|  %620 = gep %602, i64 1640                                                                                             ;L2158<175<101
 22010|  %621 = load i64, ptr %620, , !!31361, !!8                                                                             ;L2158<175<101
 22011|     ;; y1 = i64 %621
 22012|     ;; self = i64 %621
 22013|     ;; x2 = i64 %504
 22014|     ;; other = i64 %504
 22015|     ;; y2 = i64 %503
 22016|     ;; other = i64 %503
 22017|  %622 = icmp ult i64 %619, %504                                                                                        ;L3147<7<2158<175<101
 22018|  %623 = sub nuw i64 %504, %619                                                                                         ;L3147<7<2158<175<101
 22019|  %624 = sub nuw i64 %619, %504                                                                                         ;L3147<7<2158<175<101
 22020|  %625 = select i1 %622, i64 %623, i64 %624                                                                             ;L3147<7<2158<175<101
 22021|     ;; dx = i64 %625
 22022|  %626 = icmp ult i64 %621, %503                                                                                        ;L3147<8<2158<175<101
 22023|  %627 = sub nuw i64 %503, %621                                                                                         ;L3147<8<2158<175<101
 22024|  %628 = sub nuw i64 %621, %503                                                                                         ;L3147<8<2158<175<101
 22025|  %629 = select i1 %626, i64 %627, i64 %628                                                                             ;L3147<8<2158<175<101
 22026|     ;; dy = i64 %629
 22027|  %630 = mul i64 %625, %625                                                                                             ;L9<2158<175<101
 22028|  %631 = mul i64 %629, %629                                                                                             ;L9<2158<175<101
 22029|  %632 = add i64 %631, %630                                                                                             ;L9<2158<175<101
 22030|     ;; dist_sq = i64 %632
 22031|  %633 = mul i64 %565, 20                                                                                               ;L176<101
 22032|  %634 = add i64 %581, %633                                                                                             ;L26<174<101
 22033|  %635 = add i64 %634, %584                                                                                             ;L26<174<101
 22034|  %636 = add i64 %635, %589                                                                                             ;L174<101
 22035|  %637 = add i64 %636, %586                                                                                             ;L174<101
 22036|  %638 = add i64 %637, %601                                                                                             ;L174<101
 22037|  %639 = add i64 %638, %617                                                                                             ;L176<101
 22038|     ;; max_dist = i64 %639
 22039|  %640 = gep %602, i64 1648                                                                                             ;L179<101
 22040|  %641 = load i64, ptr %640, , !!31361, !!8                                                                             ;L179<101
 22041|  %642 = icmp ule i64 %641, %579                                                                                        ;L179<101
 22042|  %643 = mul i64 %639, %639                                                                                             ;L179<101
 22043|  %644 = icmp ule i64 %632, %643                                                                                        ;L179<101
 22044|  %645 = select i1 %642, i1 %644, i1 false                                                                              ;L179<101
 22045|  br i1 %645, label %646, label %568                                                                                    ;L179<101
 22046| 
 22047| 646: ; preds = %616
 22050|  %647 = gep %602, i64 1472                                                                                             ;L180<101
 22051|  %648 = load i64, ptr %647, , !!31361, !!8                                                                             ;L180<101
 22052|  invoke void @ai::small_action4castNtB2_17SmallActionAttack3new(ptr sret([24 x i8]) %13, ptr %5, i64 %648)
 22053|  to label %649 unwind label %450, !!31361                                                                              ;L180<101
 22054| 
 22055| 649: ; preds = %646
 22056|  call void @llvm.memcpy.p0.p0.i64(ptr %14, ptr %13, i64 24, i1 false), !!31351                                         ;L180<101
 22057|  store i8 15, ptr %521, , !!31351                                                                                      ;L180<101
 22060|     ;; self = ptr %16
 22061|     ;; self = ptr %16
 22062|     ;; value = ptr %14
 22063|     ;; src = ptr %14
 22064|     ;; additional = i64 1
 22065|     ;; needed_extra_cap = i64 1
 22066|     ;; needed_extra_cap = i64 1
 22067|     ;; strategy = i8 1
 22068|     ;; self = ptr %16
 22069|  %650 = load i64, ptr %457, , !!31767, !!8                                                                             ;L149<1428<180<101
 22070|  %651 = icmp eq i64 %534, %650                                                                                         ;L1428<180<101
 22071|  br i1 %651, label %652, label %658                                                                                    ;L1428<180<101
 22072| 
 22073| 652: ; preds = %649
 22074|     ;; self = ptr %16
 22075|     ;; self = ptr %16
 22076|     ;; self = ptr %16
 22077|     ;; used_cap = i64 %534
 22078|     ;; used_cap = i64 %534
 22079|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %16, i64 %534, i64 1, i1 zeroext true)
 22080|  to label %653 unwind label %656, !!31773                                                                              ;L619<430<738<1429<180<101
 22081| 
 22082| 653: ; preds = %652
 22083|  %654 = load i64, ptr %458, , !!31767                                                                                  ;L1432<180<101
 22084|  %655 = load ptr, ptr %16, , !!31767                                                                                   ;L138<1432<180<101
 22085|  br label %658                                                                                                         ;L619<430<738<1429<180<101
 22086| 
 22087| 656: ; preds = %652
 22088|  %657 = cleanuppad within none []
 22089|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %14) #30 [ "funclet"(token %657) ], !!31778 ;L1436<180<101
 22090|  cleanupret from %657 unwind label %450
 22091| 
 22092| 658: ; preds = %653, %649
 22093|  %659 = phi ptr [ %655, %653 ], [ %531, %649 ]
 22094|  %660 = phi ptr [ %655, %653 ], [ %532, %649 ]
 22095|  %661 = phi ptr [ %655, %653 ], [ %533, %649 ]                                                                         ;L138<1432<180<101
 22096|  %662 = phi i64 [ %654, %653 ], [ %534, %649 ]                                                                         ;L1432<180<101
 22097|     ;; self = ptr %16
 22098|     ;; self = ptr %661
 22099|     ;; count = i64 %662
 22100|  %663 = gepS %661, i64 %662                                                                                            ;L961<1432<180<101
 22101|     ;; end = ptr %663
 22102|     ;; dst = ptr %663
 22103|  call void @llvm.memcpy.p0.p0.i64(ptr %663, ptr %14, i64 184, i1 false), !!31778                                       ;L1933<1433<180<101
 22104|  %664 = add i64 %662, 1                                                                                                ;L1434<180<101
 22105|  store i64 %664, ptr %458, , !!31767                                                                                   ;L1434<180<101
 22107|  br label %568                                                                                                         ;L179<101
 22108| 
 22109| 665: ; preds = %568
 22110|  br i1 %573, label %672, label %666                                                                                    ;L186<101
 22111| 
 22112| 666: ; preds = %760, %718, %678, %672, %665
 22113|  %667 = phi ptr [ %569, %672 ], [ %569, %678 ], [ %569, %718 ], [ %761, %760 ], [ %569, %665 ]
 22114|  %668 = phi ptr [ %570, %672 ], [ %570, %678 ], [ %570, %718 ], [ %762, %760 ], [ %570, %665 ]
 22115|  %669 = phi ptr [ %571, %672 ], [ %571, %678 ], [ %571, %718 ], [ %762, %760 ], [ %571, %665 ]
 22116|  %670 = phi i64 [ %572, %672 ], [ %572, %678 ], [ %572, %718 ], [ %765, %760 ], [ %572, %665 ]
 22117|  %671 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_skill2(ptr %459)
 22118|  to label %766 unwind label %450, !!31361                                                                              ;L202<101
 22119| 
 22120| 672: ; preds = %665
 22121|     ;; self = ptr %459
 22122|  %673 = load i32, ptr %523, , !!31361, !!8                                                                             ;L742<187<101
 22123|  %674 = icmp eq i32 %673, -1                                                                                           ;L742<187<101
 22124|  br i1 %674, label %666, label %675                                                                                    ;L742<187<101
 22125| 
 22126| 675: ; preds = %672
 22127|     ;; skill = ptr %522
 22128|     ;; self = ptr %522
 22129|  %676 = load ptr, ptr %535, , !!31361, !!8, !!8                                                                        ;L188<101
 22130|  %677 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %524, ptr %459, ptr %676)
 22131|  to label %678 unwind label %450, !!31361                                                                              ;L188<101
 22132| 
 22133| 678: ; preds = %675
 22134|  br i1 %677, label %679, label %666                                                                                    ;L188<101
 22135| 
 22136| 679: ; preds = %678
 22137|  %680 = load ptr, ptr %535, , !!31361, !!8, !!8                                                                        ;L189<101
 22138|  %681 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %522, ptr %48, ptr %459, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %680)
 22139|  to label %682 unwind label %450, !!31361                                                                              ;L189<101
 22140| 
 22141| 682: ; preds = %679
 22142|     ;; expected_dmg = i64 %681
 22143|  %683 = load i64, ptr %525, , !!31361, !!8                                                                             ;L26<190<101
 22144|  %684 = load i64, ptr %526, , !!31361, !!8                                                                             ;L26<190<101
 22145|  %685 = load i64, ptr %517, , !!31361, !!8                                                                             ;L26<190<101
 22146|  %686 = load i64, ptr %518, , !!31361, !!8                                                                             ;L26<190<101
 22147|  %687 = load ptr, ptr %535, , !!31361, !!8, !!8                                                                        ;L190<101
 22148|  %688 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %522, ptr %459, ptr %687)
 22149|  to label %689 unwind label %450, !!31361                                                                              ;L190<101
 22150| 
 22151| 689: ; preds = %682
 22152|  %690 = add i64 %685, -1                                                                                               ;L26<190<101
 22153|  %691 = mul i64 %690, %684                                                                                             ;L26<190<101
 22154|  %692 = load i32, ptr %519, , !!31361, !!8                                                                             ;L1511<190<101
 22155|     ;; mult = i32 %692
 22156|  %693 = icmp eq i32 %692, 0                                                                                            ;L1512<190<101
 22157|  br i1 %693, label %694, label %696                                                                                    ;L1512<190<101
 22158| 
 22159| 694: ; preds = %689
 22160|  %695 = load i64, ptr %520, , !!31361, !!8                                                                             ;L1513<190<101
 22161|  br label %702                                                                                                         ;L1512<190<101
 22162| 
 22163| 696: ; preds = %689
 22164|  %697 = sext i32 %692 to i64                                                                                           ;L1511<190<101
 22165|     ;; mult = i64 %697
 22166|  %698 = load i64, ptr %520, , !!31361, !!8                                                                             ;L1515<190<101
 22167|  %699 = add nsw i64 %697, 100                                                                                          ;L1515<190<101
 22168|  %700 = mul i64 %698, %699                                                                                             ;L1515<190<101
 22169|  %701 = udiv i64 %700, 100                                                                                             ;L1515<190<101
 22170|  br label %702                                                                                                         ;L1512<190<101
 22171| 
 22172| 702: ; preds = %696, %694
 22173|  %703 = phi i64 [ %695, %694 ], [ %701, %696 ]                                                                         ;L0<190<101
 22174|  %704 = load ptr, ptr %535, , !!31361, !!8, !!8                                                                        ;L190<101
 22175|     ;; self = ptr %704
 22176|  %705 = gep %704, i64 1136                                                                                             ;L1511<190<101
 22177|  %706 = load i32, ptr %705, , !!31361, !!8                                                                             ;L1511<190<101
 22178|     ;; mult = i32 %706
 22179|  %707 = icmp eq i32 %706, 0                                                                                            ;L1512<190<101
 22180|  br i1 %707, label %708, label %711                                                                                    ;L1512<190<101
 22181| 
 22182| 708: ; preds = %702
 22183|  %709 = gep %704, i64 1664                                                                                             ;L1513<190<101
 22184|  %710 = load i64, ptr %709, , !!31361, !!8                                                                             ;L1513<190<101
 22185|  br label %718                                                                                                         ;L1512<190<101
 22186| 
 22187| 711: ; preds = %702
 22188|  %712 = sext i32 %706 to i64                                                                                           ;L1511<190<101
 22189|     ;; mult = i64 %712
 22190|  %713 = gep %704, i64 1664                                                                                             ;L1515<190<101
 22191|  %714 = load i64, ptr %713, , !!31361, !!8                                                                             ;L1515<190<101
 22192|  %715 = add nsw i64 %712, 100                                                                                          ;L1515<190<101
 22193|  %716 = mul i64 %714, %715                                                                                             ;L1515<190<101
 22194|  %717 = udiv i64 %716, 100                                                                                             ;L1515<190<101
 22195|  br label %718                                                                                                         ;L1512<190<101
 22196| 
 22197| 718: ; preds = %711, %708
 22198|  %719 = phi i64 [ %710, %708 ], [ %717, %711 ]                                                                         ;L0<190<101
 22200|     ;; self = ptr %704
 22201|  %720 = gep %704, i64 1632                                                                                             ;L2158<191<101
 22202|  %721 = load i64, ptr %720, , !!31361, !!8                                                                             ;L2158<191<101
 22203|     ;; x1 = i64 %721
 22204|     ;; self = i64 %721
 22205|  %722 = gep %704, i64 1640                                                                                             ;L2158<191<101
 22206|  %723 = load i64, ptr %722, , !!31361, !!8                                                                             ;L2158<191<101
 22207|     ;; y1 = i64 %723
 22208|     ;; self = i64 %723
 22209|     ;; x2 = i64 %504
 22210|     ;; other = i64 %504
 22211|     ;; y2 = i64 %503
 22212|     ;; other = i64 %503
 22213|  %724 = icmp ult i64 %721, %504                                                                                        ;L3147<7<2158<191<101
 22214|  %725 = sub nuw i64 %504, %721                                                                                         ;L3147<7<2158<191<101
 22215|  %726 = sub nuw i64 %721, %504                                                                                         ;L3147<7<2158<191<101
 22216|  %727 = select i1 %724, i64 %725, i64 %726                                                                             ;L3147<7<2158<191<101
 22217|     ;; dx = i64 %727
 22218|  %728 = icmp ult i64 %723, %503                                                                                        ;L3147<8<2158<191<101
 22219|  %729 = sub nuw i64 %503, %723                                                                                         ;L3147<8<2158<191<101
 22220|  %730 = sub nuw i64 %723, %503                                                                                         ;L3147<8<2158<191<101
 22221|  %731 = select i1 %728, i64 %729, i64 %730                                                                             ;L3147<8<2158<191<101
 22222|     ;; dy = i64 %731
 22223|  %732 = mul i64 %727, %727                                                                                             ;L9<2158<191<101
 22224|  %733 = mul i64 %731, %731                                                                                             ;L9<2158<191<101
 22225|  %734 = add i64 %733, %732                                                                                             ;L9<2158<191<101
 22226|     ;; dist_sq = i64 %734
 22227|  %735 = mul i64 %565, 20                                                                                               ;L192<101
 22228|  %736 = add i64 %683, %735                                                                                             ;L26<190<101
 22229|  %737 = add i64 %736, %686                                                                                             ;L26<190<101
 22230|  %738 = add i64 %737, %691                                                                                             ;L190<101
 22231|  %739 = add i64 %738, %688                                                                                             ;L190<101
 22232|  %740 = add i64 %739, %703                                                                                             ;L190<101
 22233|  %741 = add i64 %740, %719                                                                                             ;L192<101
 22234|     ;; max_dist = i64 %741
 22235|  %742 = gep %704, i64 1648                                                                                             ;L194<101
 22236|  %743 = load i64, ptr %742, , !!31361, !!8                                                                             ;L194<101
 22237|  %744 = icmp ule i64 %743, %681                                                                                        ;L194<101
 22238|  %745 = mul i64 %741, %741                                                                                             ;L194<101
 22239|  %746 = icmp ule i64 %734, %745                                                                                        ;L194<101
 22240|  %747 = select i1 %744, i1 %746, i1 false                                                                              ;L194<101
 22241|  br i1 %747, label %748, label %666                                                                                    ;L194<101
 22242| 
 22243| 748: ; preds = %718
 22246|  %749 = gep %704, i64 1472                                                                                             ;L195<101
 22247|  %750 = load i64, ptr %749, , !!31361, !!8                                                                             ;L195<101
 22248|  invoke void @ai::small_action4castNtB5_16SmallActionSkill3new(ptr sret([24 x i8]) %11, ptr %5, i64 %750)
 22249|  to label %751 unwind label %450, !!31361                                                                              ;L195<101
 22250| 
 22251| 751: ; preds = %748
 22252|  call void @llvm.memcpy.p0.p0.i64(ptr %12, ptr %11, i64 24, i1 false), !!31351                                         ;L195<101
 22253|  store i8 16, ptr %527, , !!31351                                                                                      ;L195<101
 22256|     ;; self = ptr %16
 22257|     ;; self = ptr %16
 22258|     ;; value = ptr %12
 22259|     ;; src = ptr %12
 22260|     ;; additional = i64 1
 22261|     ;; needed_extra_cap = i64 1
 22262|     ;; needed_extra_cap = i64 1
 22263|     ;; strategy = i8 1
 22264|     ;; self = ptr %16
 22265|  %752 = load i64, ptr %457, , !!31843, !!8                                                                             ;L149<1428<195<101
 22266|  %753 = icmp eq i64 %572, %752                                                                                         ;L1428<195<101
 22267|  br i1 %753, label %754, label %760                                                                                    ;L1428<195<101
 22268| 
 22269| 754: ; preds = %751
 22270|     ;; self = ptr %16
 22271|     ;; self = ptr %16
 22272|     ;; self = ptr %16
 22273|     ;; used_cap = i64 %572
 22274|     ;; used_cap = i64 %572
 22275|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %16, i64 %572, i64 1, i1 zeroext true)
 22276|  to label %755 unwind label %758, !!31849                                                                              ;L619<430<738<1429<195<101
 22277| 
 22278| 755: ; preds = %754
 22279|  %756 = load i64, ptr %458, , !!31843                                                                                  ;L1432<195<101
 22280|  %757 = load ptr, ptr %16, , !!31843                                                                                   ;L138<1432<195<101
 22281|  br label %760                                                                                                         ;L619<430<738<1429<195<101
 22282| 
 22283| 758: ; preds = %754
 22284|  %759 = cleanuppad within none []
 22285|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %12) #30 [ "funclet"(token %759) ], !!31854 ;L1436<195<101
 22286|  cleanupret from %759 unwind label %450
 22287| 
 22288| 760: ; preds = %755, %751
 22289|  %761 = phi ptr [ %757, %755 ], [ %569, %751 ]
 22290|  %762 = phi ptr [ %757, %755 ], [ %570, %751 ]                                                                         ;L138<1432<195<101
 22291|  %763 = phi i64 [ %756, %755 ], [ %572, %751 ]                                                                         ;L1432<195<101
 22292|     ;; self = ptr %16
 22293|     ;; self = ptr %762
 22294|     ;; count = i64 %763
 22295|  %764 = gepS %762, i64 %763                                                                                            ;L961<1432<195<101
 22296|     ;; end = ptr %764
 22297|     ;; dst = ptr %764
 22298|  call void @llvm.memcpy.p0.p0.i64(ptr %764, ptr %12, i64 184, i1 false), !!31854                                       ;L1933<1433<195<101
 22299|  %765 = add i64 %763, 1                                                                                                ;L1434<195<101
 22300|  store i64 %765, ptr %458, , !!31843                                                                                   ;L1434<195<101
 22302|  br label %666                                                                                                         ;L194<101
 22303| 
 22304| 766: ; preds = %666
 22305|  br i1 %671, label %772, label %767                                                                                    ;L202<101
 22306| 
 22307| 767: ; preds = %866, %824, %783, %772, %766, %547, %543, %537
 22308|  %768 = phi ptr [ %531, %537 ], [ %531, %543 ], [ %531, %547 ], [ %667, %766 ], [ %867, %866 ], [ %667, %824 ], [ %667, %783 ], [ %667, %772 ]
 22309|  %769 = phi ptr [ %532, %537 ], [ %532, %543 ], [ %532, %547 ], [ %668, %766 ], [ %867, %866 ], [ %668, %824 ], [ %668, %783 ], [ %668, %772 ]
 22310|  %770 = phi ptr [ %533, %537 ], [ %533, %543 ], [ %533, %547 ], [ %669, %766 ], [ %867, %866 ], [ %669, %824 ], [ %669, %783 ], [ %669, %772 ]
 22311|  %771 = phi i64 [ %534, %537 ], [ %534, %543 ], [ %534, %547 ], [ %670, %766 ], [ %870, %866 ], [ %670, %824 ], [ %670, %783 ], [ %670, %772 ]
 22312|  br label %530                                                                                                         ;L1714<180<156<101
 22313| 
 22314| 772: ; preds = %766
 22315|  %773 = load i64, ptr %517, , !!31361, !!8                                                                             ;L1693<203<101
 22316|  %774 = icmp ugt i64 %773, 2                                                                                           ;L1693<203<101
 22317|  %775 = select i1 %774, ptr %528, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.19                                        ;L1693<203<101
 22318|     ;; self = ptr %775
 22319|  %776 = gep %775, i64 48                                                                                               ;L742<203<101
 22320|  %777 = load i32, ptr %776, , !!31361, !!8                                                                             ;L742<203<101
 22321|  %778 = icmp eq i32 %777, -1                                                                                           ;L742<203<101
 22322|  br i1 %778, label %767, label %779                                                                                    ;L742<203<101
 22323| 
 22324| 779: ; preds = %772
 22325|     ;; skill2 = ptr %775
 22326|     ;; self = ptr %775
 22327|  %780 = gep %775, i64 40                                                                                               ;L204<101
 22328|  %781 = load ptr, ptr %535, , !!31361, !!8, !!8                                                                        ;L204<101
 22329|  %782 = invoke zeroext i1 @gc::simulation6effect4typeNtB4_13CastingTarget5check(ptr %780, ptr %459, ptr %781)
 22330|  to label %783 unwind label %450, !!31361                                                                              ;L204<101
 22331| 
 22332| 783: ; preds = %779
 22333|  br i1 %782, label %784, label %767                                                                                    ;L204<101
 22334| 
 22335| 784: ; preds = %783
 22336|  %785 = load ptr, ptr %535, , !!31361, !!8, !!8                                                                        ;L205<101
 22337|  %786 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %775, ptr %48, ptr %459, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.54, ptr %785)
 22338|  to label %787 unwind label %450, !!31361                                                                              ;L205<101
 22339| 
 22340| 787: ; preds = %784
 22341|     ;; expected_dmg = i64 %786
 22342|  %788 = gep %775, i64 16                                                                                               ;L26<206<101
 22343|  %789 = load i64, ptr %788, , !!31361, !!8                                                                             ;L26<206<101
 22344|  %790 = gep %775, i64 24                                                                                               ;L26<206<101
 22345|  %791 = load i64, ptr %790, , !!31361, !!8                                                                             ;L26<206<101
 22346|  %792 = load i64, ptr %518, , !!31361, !!8                                                                             ;L26<206<101
 22347|  %793 = load ptr, ptr %535, , !!31361, !!8, !!8                                                                        ;L206<101
 22348|  %794 = invoke i64 @gc::simulation6effectNtB2_6Effect12range_adjust(ptr %775, ptr %459, ptr %793)
 22349|  to label %795 unwind label %450, !!31361                                                                              ;L206<101
 22350| 
 22351| 795: ; preds = %787
 22352|  %796 = add i64 %773, -1                                                                                               ;L26<206<101
 22353|  %797 = mul i64 %791, %796                                                                                             ;L26<206<101
 22354|  %798 = load i32, ptr %519, , !!31361, !!8                                                                             ;L1511<206<101
 22355|     ;; mult = i32 %798
 22356|  %799 = icmp eq i32 %798, 0                                                                                            ;L1512<206<101
 22357|  br i1 %799, label %800, label %802                                                                                    ;L1512<206<101
 22358| 
 22359| 800: ; preds = %795
 22360|  %801 = load i64, ptr %520, , !!31361, !!8                                                                             ;L1513<206<101
 22361|  br label %808                                                                                                         ;L1512<206<101
 22362| 
 22363| 802: ; preds = %795
 22364|  %803 = sext i32 %798 to i64                                                                                           ;L1511<206<101
 22365|     ;; mult = i64 %803
 22366|  %804 = load i64, ptr %520, , !!31361, !!8                                                                             ;L1515<206<101
 22367|  %805 = add nsw i64 %803, 100                                                                                          ;L1515<206<101
 22368|  %806 = mul i64 %804, %805                                                                                             ;L1515<206<101
 22369|  %807 = udiv i64 %806, 100                                                                                             ;L1515<206<101
 22370|  br label %808                                                                                                         ;L1512<206<101
 22371| 
 22372| 808: ; preds = %802, %800
 22373|  %809 = phi i64 [ %801, %800 ], [ %807, %802 ]                                                                         ;L0<206<101
 22374|  %810 = load ptr, ptr %535, , !!31361, !!8, !!8                                                                        ;L206<101
 22375|     ;; self = ptr %810
 22376|  %811 = gep %810, i64 1136                                                                                             ;L1511<206<101
 22377|  %812 = load i32, ptr %811, , !!31361, !!8                                                                             ;L1511<206<101
 22378|     ;; mult = i32 %812
 22379|  %813 = icmp eq i32 %812, 0                                                                                            ;L1512<206<101
 22380|  br i1 %813, label %814, label %817                                                                                    ;L1512<206<101
 22381| 
 22382| 814: ; preds = %808
 22383|  %815 = gep %810, i64 1664                                                                                             ;L1513<206<101
 22384|  %816 = load i64, ptr %815, , !!31361, !!8                                                                             ;L1513<206<101
 22385|  br label %824                                                                                                         ;L1512<206<101
 22386| 
 22387| 817: ; preds = %808
 22388|  %818 = sext i32 %812 to i64                                                                                           ;L1511<206<101
 22389|     ;; mult = i64 %818
 22390|  %819 = gep %810, i64 1664                                                                                             ;L1515<206<101
 22391|  %820 = load i64, ptr %819, , !!31361, !!8                                                                             ;L1515<206<101
 22392|  %821 = add nsw i64 %818, 100                                                                                          ;L1515<206<101
 22393|  %822 = mul i64 %820, %821                                                                                             ;L1515<206<101
 22394|  %823 = udiv i64 %822, 100                                                                                             ;L1515<206<101
 22395|  br label %824                                                                                                         ;L1512<206<101
 22396| 
 22397| 824: ; preds = %817, %814
 22398|  %825 = phi i64 [ %816, %814 ], [ %823, %817 ]                                                                         ;L0<206<101
 22400|     ;; self = ptr %810
 22401|  %826 = gep %810, i64 1632                                                                                             ;L2158<207<101
 22402|  %827 = load i64, ptr %826, , !!31361, !!8                                                                             ;L2158<207<101
 22403|     ;; x1 = i64 %827
 22404|     ;; self = i64 %827
 22405|  %828 = gep %810, i64 1640                                                                                             ;L2158<207<101
 22406|  %829 = load i64, ptr %828, , !!31361, !!8                                                                             ;L2158<207<101
 22407|     ;; y1 = i64 %829
 22408|     ;; self = i64 %829
 22409|     ;; x2 = i64 %504
 22410|     ;; other = i64 %504
 22411|     ;; y2 = i64 %503
 22412|     ;; other = i64 %503
 22413|  %830 = icmp ult i64 %827, %504                                                                                        ;L3147<7<2158<207<101
 22414|  %831 = sub nuw i64 %504, %827                                                                                         ;L3147<7<2158<207<101
 22415|  %832 = sub nuw i64 %827, %504                                                                                         ;L3147<7<2158<207<101
 22416|  %833 = select i1 %830, i64 %831, i64 %832                                                                             ;L3147<7<2158<207<101
 22417|     ;; dx = i64 %833
 22418|  %834 = icmp ult i64 %829, %503                                                                                        ;L3147<8<2158<207<101
 22419|  %835 = sub nuw i64 %503, %829                                                                                         ;L3147<8<2158<207<101
 22420|  %836 = sub nuw i64 %829, %503                                                                                         ;L3147<8<2158<207<101
 22421|  %837 = select i1 %834, i64 %835, i64 %836                                                                             ;L3147<8<2158<207<101
 22422|     ;; dy = i64 %837
 22423|  %838 = mul i64 %833, %833                                                                                             ;L9<2158<207<101
 22424|  %839 = mul i64 %837, %837                                                                                             ;L9<2158<207<101
 22425|  %840 = add i64 %839, %838                                                                                             ;L9<2158<207<101
 22426|     ;; dist_sq = i64 %840
 22427|  %841 = mul i64 %565, 20                                                                                               ;L208<101
 22428|  %842 = add i64 %789, %841                                                                                             ;L26<206<101
 22429|  %843 = add i64 %842, %797                                                                                             ;L26<206<101
 22430|  %844 = add i64 %843, %792                                                                                             ;L206<101
 22431|  %845 = add i64 %844, %794                                                                                             ;L206<101
 22432|  %846 = add i64 %845, %809                                                                                             ;L206<101
 22433|  %847 = add i64 %846, %825                                                                                             ;L208<101
 22434|     ;; max_dist = i64 %847
 22435|  %848 = gep %810, i64 1648                                                                                             ;L210<101
 22436|  %849 = load i64, ptr %848, , !!31361, !!8                                                                             ;L210<101
 22437|  %850 = icmp ule i64 %849, %786                                                                                        ;L210<101
 22438|  %851 = mul i64 %847, %847                                                                                             ;L210<101
 22439|  %852 = icmp ule i64 %840, %851                                                                                        ;L210<101
 22440|  %853 = select i1 %850, i1 %852, i1 false                                                                              ;L210<101
 22441|  br i1 %853, label %854, label %767                                                                                    ;L210<101
 22442| 
 22443| 854: ; preds = %824
 22446|  %855 = gep %810, i64 1472                                                                                             ;L211<101
 22447|  %856 = load i64, ptr %855, , !!31361, !!8                                                                             ;L211<101
 22448|  invoke void @ai::small_action4castNtB5_17SmallActionSkill23new(ptr sret([24 x i8]) %9, ptr %5, i64 %856)
 22449|  to label %857 unwind label %450, !!31361                                                                              ;L211<101
 22450| 
 22451| 857: ; preds = %854
 22452|  call void @llvm.memcpy.p0.p0.i64(ptr %10, ptr %9, i64 24, i1 false), !!31351                                          ;L211<101
 22453|  store i8 17, ptr %529, , !!31351                                                                                      ;L211<101
 22456|     ;; self = ptr %16
 22457|     ;; self = ptr %16
 22458|     ;; value = ptr %10
 22459|     ;; src = ptr %10
 22460|     ;; additional = i64 1
 22461|     ;; needed_extra_cap = i64 1
 22462|     ;; needed_extra_cap = i64 1
 22463|     ;; strategy = i8 1
 22464|     ;; self = ptr %16
 22465|  %858 = load i64, ptr %457, , !!31918, !!8                                                                             ;L149<1428<211<101
 22466|  %859 = icmp eq i64 %670, %858                                                                                         ;L1428<211<101
 22467|  br i1 %859, label %860, label %866                                                                                    ;L1428<211<101
 22468| 
 22469| 860: ; preds = %857
 22470|     ;; self = ptr %16
 22471|     ;; self = ptr %16
 22472|     ;; self = ptr %16
 22473|     ;; used_cap = i64 %670
 22474|     ;; used_cap = i64 %670
 22475|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %16, i64 %670, i64 1, i1 zeroext true)
 22476|  to label %861 unwind label %864, !!31924                                                                              ;L619<430<738<1429<211<101
 22477| 
 22478| 861: ; preds = %860
 22479|  %862 = load i64, ptr %458, , !!31918                                                                                  ;L1432<211<101
 22480|  %863 = load ptr, ptr %16, , !!31918                                                                                   ;L138<1432<211<101
 22481|  br label %866                                                                                                         ;L619<430<738<1429<211<101
 22482| 
 22483| 864: ; preds = %860
 22484|  %865 = cleanuppad within none []
 22485|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %10) #30 [ "funclet"(token %865) ], !!31929 ;L1436<211<101
 22486|  cleanupret from %865 unwind label %450
 22487| 
 22488| 866: ; preds = %861, %857
 22489|  %867 = phi ptr [ %863, %861 ], [ %667, %857 ]                                                                         ;L138<1432<211<101
 22490|  %868 = phi i64 [ %862, %861 ], [ %670, %857 ]                                                                         ;L1432<211<101
 22491|     ;; self = ptr %16
 22492|     ;; self = ptr %867
 22493|     ;; count = i64 %868
 22494|  %869 = gepS %867, i64 %868                                                                                            ;L961<1432<211<101
 22495|     ;; end = ptr %869
 22496|     ;; dst = ptr %869
 22497|  call void @llvm.memcpy.p0.p0.i64(ptr %869, ptr %10, i64 184, i1 false), !!31929                                       ;L1933<1433<211<101
 22498|  %870 = add i64 %868, 1                                                                                                ;L1434<211<101
 22499|  store i64 %870, ptr %458, , !!31918                                                                                   ;L1434<211<101
 22501|  br label %767                                                                                                         ;L210<101
 22502| 
 22503| 871: ; preds = %431
 22504|  call void @llvm.memcpy.p0.p0.i64(ptr %28, ptr %27, i64 120, i1 false)                                                 ;L96
 22505|  %872 = gep %28, i64 177                                                                                               ;L96
 22506|  store i8 12, ptr %872,                                                                                                ;L96
 22509|     ;; self = ptr %46
 22510|     ;; self = ptr %46
 22511|     ;; value = ptr %28
 22512|     ;; src = ptr %28
 22513|     ;; additional = i64 1
 22514|     ;; needed_extra_cap = i64 1
 22515|     ;; needed_extra_cap = i64 1
 22516|     ;; strategy = i8 1
 22517|     ;; self = ptr %46
 22518|  %873 = load i64, ptr %51, , !!31954, !!8                                                                              ;L149<1428<96
 22519|  %874 = icmp eq i64 %213, %873                                                                                         ;L1428<96
 22520|  br i1 %874, label %875, label %881                                                                                    ;L1428<96
 22521| 
 22522| 875: ; preds = %871
 22523|     ;; self = ptr %46
 22524|     ;; self = ptr %46
 22525|     ;; self = ptr %46
 22526|     ;; used_cap = i64 %213
 22527|     ;; used_cap = i64 %213
 22528|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %46, i64 %213, i64 1, i1 zeroext true)
 22529|  to label %876 unwind label %879, !!31954                                                                              ;L619<430<738<1429<96
 22530| 
 22531| 876: ; preds = %875
 22532|  %877 = load i64, ptr %52, , !!31954                                                                                   ;L1432<96
 22533|  %878 = load ptr, ptr %46, , !!31954                                                                                   ;L138<1432<96
 22534|  br label %881                                                                                                         ;L619<430<738<1429<96
 22535| 
 22536| 879: ; preds = %875
 22537|  %880 = cleanuppad within none []
 22538|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %28) #30 [ "funclet"(token %880) ], !!31937 ;L1436<96
 22539|  cleanupret from %880 unwind label %57
 22540| 
 22541| 881: ; preds = %876, %871
 22542|  %882 = phi ptr [ %878, %876 ], [ %214, %871 ]                                                                         ;L138<1432<96
 22543|  %883 = phi i64 [ %877, %876 ], [ %213, %871 ]                                                                         ;L1432<96
 22544|     ;; self = ptr %46
 22545|     ;; self = ptr %882
 22546|     ;; count = i64 %883
 22547|  %884 = gepS %882, i64 %883                                                                                            ;L961<1432<96
 22548|     ;; end = ptr %884
 22549|     ;; dst = ptr %884
 22550|  call void @llvm.memcpy.p0.p0.i64(ptr %884, ptr %28, i64 184, i1 false), !!31937                                       ;L1933<1433<96
 22551|  %885 = add i64 %883, 1                                                                                                ;L1434<96
 22552|  store i64 %885, ptr %52, , !!31954                                                                                    ;L1434<96
 22554|  br label %454                                                                                                         ;L93
 22555| 
 22556| 886: ; preds = %427
 22557|     ;; enemy = ptr %420
 22558|     ;; self[0..+8] = ptr %66
 22559|     ;; slice[0..+8] = ptr %66
 22560|     ;; self[8..+8] = i64 5
 22561|     ;; slice[8..+8] = i64 5
 22562|     ;; self = ptr %66
 22563|     ;; self[0..+8] = ptr %66
 22564|     ;; self[8..+8] = ptr %66
 22565|     ;; self[16..+8] = ptr %68
 22566|     ;; init = i64 0
 22569|     ;; self[0..+8] = ptr %66
 22570|     ;; iter[0..+8] = ptr %66
 22571|     ;; self[0..+8] = ptr %66
 22572|     ;; self[8..+8] = ptr %66
 22573|     ;; iter[8..+8] = ptr %66
 22574|     ;; self[8..+8] = ptr %66
 22575|     ;; self[16..+8] = ptr %68
 22576|     ;; iter[16..+8] = ptr %68
 22577|     ;; self[16..+8] = ptr %68
 22579|     ;; self[0..+8] = ptr %66
 22580|     ;; self[8..+8] = ptr %66
 22581|     ;; init = i64 0
 22582|     ;; fold = ptr %68
 22584|     ;; f = ptr %68
 22585|     ;; self[0..+8] = ptr %66
 22586|     ;; self[8..+8] = ptr %66
 22587|     ;; init = i64 0
 22588|     ;; acc = i64 0
 22589|     ;; i = i64 0
 22590|     ;; len = i64 5
 22591|  %887 = load i64, ptr %75, , !!32096
 22592|  %888 = load i64, ptr %403, , !!32096
 22593|  %889 = load i64, ptr %405, , !!32096
 22594|     ;; self = ptr %66
 22595|     ;; count = i64 0
 22596|  %890 = load ptr, ptr %66, , !!32106, !!8                                                                              ;L279<146<128<52<3674<142<70
 22598|     ;; acc = i64 0
 22600|  %891 = icmp eq ptr %890, null                                                                                         ;L39<279<146<128<52<3674<142<70
 22601|  br i1 %891, label %914, label %892                                                                                    ;L39<279<146<128<52<3674<142<70
 22602| 
 22603| 892: ; preds = %886
 22604|     ;; x = ptr %890
 22606|     ;; acc = i64 0
 22607|     ;; elt = ptr %890
 22608|     ;; x = ptr %890
 22612|  %893 = gep %890, i64 1472                                                                                             ;L69<138<88<40<279<146<128<52<3674<142<70
 22613|  %894 = load i64, ptr %893, , !!32106, !!8                                                                             ;L69<138<88<40<279<146<128<52<3674<142<70
 22614|  %895 = icmp eq i64 %894, %887                                                                                         ;L69<138<88<40<279<146<128<52<3674<142<70
 22615|  br i1 %895, label %914, label %896                                                                                    ;L69<138<88<40<279<146<128<52<3674<142<70
 22616| 
 22617| 896: ; preds = %892
 22618|     ;; self = ptr %890
 22619|     ;; other = ptr %68
 22620|  %897 = gep %890, i64 1632                                                                                             ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22621|  %898 = load i64, ptr %897, , !!32106, !!8                                                                             ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22622|     ;; x1 = i64 %898
 22623|     ;; self = i64 %898
 22624|  %899 = gep %890, i64 1640                                                                                             ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22625|  %900 = load i64, ptr %899, , !!32106, !!8                                                                             ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22626|     ;; y1 = i64 %900
 22627|     ;; self = i64 %900
 22628|     ;; x2 = i64 %888
 22629|     ;; other = i64 %888
 22630|     ;; y2 = i64 %889
 22631|     ;; other = i64 %889
 22632|  %901 = icmp ult i64 %898, %888                                                                                        ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22633|  %902 = sub nuw i64 %888, %898                                                                                         ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22634|  %903 = sub nuw i64 %898, %888                                                                                         ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22635|  %904 = select i1 %901, i64 %902, i64 %903                                                                             ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22636|     ;; dx = i64 %904
 22637|  %905 = icmp ult i64 %900, %889                                                                                        ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22638|  %906 = sub nuw i64 %889, %900                                                                                         ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22639|  %907 = sub nuw i64 %900, %889                                                                                         ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22640|  %908 = select i1 %905, i64 %906, i64 %907                                                                             ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22641|     ;; dy = i64 %908
 22642|  %909 = mul i64 %904, %904                                                                                             ;L9<2158<69<138<88<40<279<146<128<52<3674<142<70
 22643|  %910 = mul i64 %908, %908                                                                                             ;L9<2158<69<138<88<40<279<146<128<52<3674<142<70
 22644|  %911 = add i64 %910, %909                                                                                             ;L9<2158<69<138<88<40<279<146<128<52<3674<142<70
 22645|  %912 = icmp ult i64 %911, 22500000001                                                                                 ;L69<138<88<40<279<146<128<52<3674<142<70
 22646|  %913 = zext i1 %912 to i64                                                                                            ;L138<88<40<279<146<128<52<3674<142<70
 22647|  br label %914                                                                                                         ;L69<138<88<40<279<146<128<52<3674<142<70
 22648| 
 22649| 914: ; preds = %896, %892, %886
 22650|  %915 = phi i64 [ 0, %886 ], [ %913, %896 ], [ 0, %892 ]                                                               ;L0<279<146<128<52<3674<142<70
 22651|     ;; acc = i64 %915
 22652|     ;; i = i64 1
 22653|     ;; self = ptr %66
 22654|     ;; count = i64 1
 22655|  %916 = gep %66, i64 8                                                                                                 ;L656<279<146<128<52<3674<142<70
 22656|  %917 = load ptr, ptr %916, , !!32106, !!8                                                                             ;L279<146<128<52<3674<142<70
 22658|     ;; acc = i64 %915
 22660|  %918 = icmp eq ptr %917, null                                                                                         ;L39<279<146<128<52<3674<142<70
 22661|  br i1 %918, label %944, label %919                                                                                    ;L39<279<146<128<52<3674<142<70
 22662| 
 22663| 919: ; preds = %914
 22664|     ;; x = ptr %917
 22666|     ;; acc = i64 %915
 22667|     ;; elt = ptr %917
 22668|     ;; x = ptr %917
 22672|  %920 = gep %917, i64 1472                                                                                             ;L69<138<88<40<279<146<128<52<3674<142<70
 22673|  %921 = load i64, ptr %920, , !!32106, !!8                                                                             ;L69<138<88<40<279<146<128<52<3674<142<70
 22674|  %922 = icmp eq i64 %921, %887                                                                                         ;L69<138<88<40<279<146<128<52<3674<142<70
 22675|  br i1 %922, label %941, label %923                                                                                    ;L69<138<88<40<279<146<128<52<3674<142<70
 22676| 
 22677| 923: ; preds = %919
 22678|     ;; self = ptr %917
 22679|     ;; other = ptr %68
 22680|  %924 = gep %917, i64 1632                                                                                             ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22681|  %925 = load i64, ptr %924, , !!32106, !!8                                                                             ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22682|     ;; x1 = i64 %925
 22683|     ;; self = i64 %925
 22684|  %926 = gep %917, i64 1640                                                                                             ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22685|  %927 = load i64, ptr %926, , !!32106, !!8                                                                             ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22686|     ;; y1 = i64 %927
 22687|     ;; self = i64 %927
 22688|     ;; x2 = i64 %888
 22689|     ;; other = i64 %888
 22690|     ;; y2 = i64 %889
 22691|     ;; other = i64 %889
 22692|  %928 = icmp ult i64 %925, %888                                                                                        ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22693|  %929 = sub nuw i64 %888, %925                                                                                         ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22694|  %930 = sub nuw i64 %925, %888                                                                                         ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22695|  %931 = select i1 %928, i64 %929, i64 %930                                                                             ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22696|     ;; dx = i64 %931
 22697|  %932 = icmp ult i64 %927, %889                                                                                        ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22698|  %933 = sub nuw i64 %889, %927                                                                                         ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22699|  %934 = sub nuw i64 %927, %889                                                                                         ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22700|  %935 = select i1 %932, i64 %933, i64 %934                                                                             ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22701|     ;; dy = i64 %935
 22702|  %936 = mul i64 %931, %931                                                                                             ;L9<2158<69<138<88<40<279<146<128<52<3674<142<70
 22703|  %937 = mul i64 %935, %935                                                                                             ;L9<2158<69<138<88<40<279<146<128<52<3674<142<70
 22704|  %938 = add i64 %937, %936                                                                                             ;L9<2158<69<138<88<40<279<146<128<52<3674<142<70
 22705|  %939 = icmp ult i64 %938, 22500000001                                                                                 ;L69<138<88<40<279<146<128<52<3674<142<70
 22706|  %940 = zext i1 %939 to i64                                                                                            ;L138<88<40<279<146<128<52<3674<142<70
 22707|  br label %941                                                                                                         ;L69<138<88<40<279<146<128<52<3674<142<70
 22708| 
 22709| 941: ; preds = %923, %919
 22710|  %942 = phi i64 [ %940, %923 ], [ 0, %919 ]                                                                            ;L69<138<88<40<279<146<128<52<3674<142<70
 22712|     ;; a = i64 %915
 22713|     ;; b = i64 %942
 22714|  %943 = add nuw nsw i64 %942, %915                                                                                     ;L55<88<40<279<146<128<52<3674<142<70
 22715|  br label %944                                                                                                         ;L42<279<146<128<52<3674<142<70
 22716| 
 22717| 944: ; preds = %941, %914
 22718|  %945 = phi i64 [ %943, %941 ], [ %915, %914 ]                                                                         ;L0<279<146<128<52<3674<142<70
 22719|     ;; acc = i64 %945
 22720|     ;; i = i64 2
 22721|     ;; self = ptr %66
 22722|     ;; count = i64 2
 22723|  %946 = gep %66, i64 16                                                                                                ;L656<279<146<128<52<3674<142<70
 22724|  %947 = load ptr, ptr %946, , !!32106, !!8                                                                             ;L279<146<128<52<3674<142<70
 22726|     ;; acc = i64 %945
 22728|  %948 = icmp eq ptr %947, null                                                                                         ;L39<279<146<128<52<3674<142<70
 22729|  br i1 %948, label %974, label %949                                                                                    ;L39<279<146<128<52<3674<142<70
 22730| 
 22731| 949: ; preds = %944
 22732|     ;; x = ptr %947
 22734|     ;; acc = i64 %945
 22735|     ;; elt = ptr %947
 22736|     ;; x = ptr %947
 22740|  %950 = gep %947, i64 1472                                                                                             ;L69<138<88<40<279<146<128<52<3674<142<70
 22741|  %951 = load i64, ptr %950, , !!32106, !!8                                                                             ;L69<138<88<40<279<146<128<52<3674<142<70
 22742|  %952 = icmp eq i64 %951, %887                                                                                         ;L69<138<88<40<279<146<128<52<3674<142<70
 22743|  br i1 %952, label %971, label %953                                                                                    ;L69<138<88<40<279<146<128<52<3674<142<70
 22744| 
 22745| 953: ; preds = %949
 22746|     ;; self = ptr %947
 22747|     ;; other = ptr %68
 22748|  %954 = gep %947, i64 1632                                                                                             ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22749|  %955 = load i64, ptr %954, , !!32106, !!8                                                                             ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22750|     ;; x1 = i64 %955
 22751|     ;; self = i64 %955
 22752|  %956 = gep %947, i64 1640                                                                                             ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22753|  %957 = load i64, ptr %956, , !!32106, !!8                                                                             ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22754|     ;; y1 = i64 %957
 22755|     ;; self = i64 %957
 22756|     ;; x2 = i64 %888
 22757|     ;; other = i64 %888
 22758|     ;; y2 = i64 %889
 22759|     ;; other = i64 %889
 22760|  %958 = icmp ult i64 %955, %888                                                                                        ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22761|  %959 = sub nuw i64 %888, %955                                                                                         ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22762|  %960 = sub nuw i64 %955, %888                                                                                         ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22763|  %961 = select i1 %958, i64 %959, i64 %960                                                                             ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22764|     ;; dx = i64 %961
 22765|  %962 = icmp ult i64 %957, %889                                                                                        ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22766|  %963 = sub nuw i64 %889, %957                                                                                         ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22767|  %964 = sub nuw i64 %957, %889                                                                                         ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22768|  %965 = select i1 %962, i64 %963, i64 %964                                                                             ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22769|     ;; dy = i64 %965
 22770|  %966 = mul i64 %961, %961                                                                                             ;L9<2158<69<138<88<40<279<146<128<52<3674<142<70
 22771|  %967 = mul i64 %965, %965                                                                                             ;L9<2158<69<138<88<40<279<146<128<52<3674<142<70
 22772|  %968 = add i64 %967, %966                                                                                             ;L9<2158<69<138<88<40<279<146<128<52<3674<142<70
 22773|  %969 = icmp ult i64 %968, 22500000001                                                                                 ;L69<138<88<40<279<146<128<52<3674<142<70
 22774|  %970 = zext i1 %969 to i64                                                                                            ;L138<88<40<279<146<128<52<3674<142<70
 22775|  br label %971                                                                                                         ;L69<138<88<40<279<146<128<52<3674<142<70
 22776| 
 22777| 971: ; preds = %953, %949
 22778|  %972 = phi i64 [ %970, %953 ], [ 0, %949 ]                                                                            ;L69<138<88<40<279<146<128<52<3674<142<70
 22780|     ;; a = i64 %945
 22781|     ;; b = i64 %972
 22782|  %973 = add nuw nsw i64 %972, %945                                                                                     ;L55<88<40<279<146<128<52<3674<142<70
 22783|  br label %974                                                                                                         ;L42<279<146<128<52<3674<142<70
 22784| 
 22785| 974: ; preds = %971, %944
 22786|  %975 = phi i64 [ %973, %971 ], [ %945, %944 ]                                                                         ;L0<279<146<128<52<3674<142<70
 22787|     ;; acc = i64 %975
 22788|     ;; i = i64 3
 22789|     ;; self = ptr %66
 22790|     ;; count = i64 3
 22791|  %976 = gep %66, i64 24                                                                                                ;L656<279<146<128<52<3674<142<70
 22792|  %977 = load ptr, ptr %976, , !!32106, !!8                                                                             ;L279<146<128<52<3674<142<70
 22794|     ;; acc = i64 %975
 22796|  %978 = icmp eq ptr %977, null                                                                                         ;L39<279<146<128<52<3674<142<70
 22797|  br i1 %978, label %1004, label %979                                                                                   ;L39<279<146<128<52<3674<142<70
 22798| 
 22799| 979: ; preds = %974
 22800|     ;; x = ptr %977
 22802|     ;; acc = i64 %975
 22803|     ;; elt = ptr %977
 22804|     ;; x = ptr %977
 22808|  %980 = gep %977, i64 1472                                                                                             ;L69<138<88<40<279<146<128<52<3674<142<70
 22809|  %981 = load i64, ptr %980, , !!32106, !!8                                                                             ;L69<138<88<40<279<146<128<52<3674<142<70
 22810|  %982 = icmp eq i64 %981, %887                                                                                         ;L69<138<88<40<279<146<128<52<3674<142<70
 22811|  br i1 %982, label %1001, label %983                                                                                   ;L69<138<88<40<279<146<128<52<3674<142<70
 22812| 
 22813| 983: ; preds = %979
 22814|     ;; self = ptr %977
 22815|     ;; other = ptr %68
 22816|  %984 = gep %977, i64 1632                                                                                             ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22817|  %985 = load i64, ptr %984, , !!32106, !!8                                                                             ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22818|     ;; x1 = i64 %985
 22819|     ;; self = i64 %985
 22820|  %986 = gep %977, i64 1640                                                                                             ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22821|  %987 = load i64, ptr %986, , !!32106, !!8                                                                             ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22822|     ;; y1 = i64 %987
 22823|     ;; self = i64 %987
 22824|     ;; x2 = i64 %888
 22825|     ;; other = i64 %888
 22826|     ;; y2 = i64 %889
 22827|     ;; other = i64 %889
 22828|  %988 = icmp ult i64 %985, %888                                                                                        ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22829|  %989 = sub nuw i64 %888, %985                                                                                         ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22830|  %990 = sub nuw i64 %985, %888                                                                                         ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22831|  %991 = select i1 %988, i64 %989, i64 %990                                                                             ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22832|     ;; dx = i64 %991
 22833|  %992 = icmp ult i64 %987, %889                                                                                        ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22834|  %993 = sub nuw i64 %889, %987                                                                                         ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22835|  %994 = sub nuw i64 %987, %889                                                                                         ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22836|  %995 = select i1 %992, i64 %993, i64 %994                                                                             ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22837|     ;; dy = i64 %995
 22838|  %996 = mul i64 %991, %991                                                                                             ;L9<2158<69<138<88<40<279<146<128<52<3674<142<70
 22839|  %997 = mul i64 %995, %995                                                                                             ;L9<2158<69<138<88<40<279<146<128<52<3674<142<70
 22840|  %998 = add i64 %997, %996                                                                                             ;L9<2158<69<138<88<40<279<146<128<52<3674<142<70
 22841|  %999 = icmp ult i64 %998, 22500000001                                                                                 ;L69<138<88<40<279<146<128<52<3674<142<70
 22842|  %1000 = zext i1 %999 to i64                                                                                           ;L138<88<40<279<146<128<52<3674<142<70
 22843|  br label %1001                                                                                                        ;L69<138<88<40<279<146<128<52<3674<142<70
 22844| 
 22845| 1001: ; preds = %983, %979
 22846|  %1002 = phi i64 [ %1000, %983 ], [ 0, %979 ]                                                                          ;L69<138<88<40<279<146<128<52<3674<142<70
 22848|     ;; a = i64 %975
 22849|     ;; b = i64 %1002
 22850|  %1003 = add nuw nsw i64 %1002, %975                                                                                   ;L55<88<40<279<146<128<52<3674<142<70
 22851|  br label %1004                                                                                                        ;L42<279<146<128<52<3674<142<70
 22852| 
 22853| 1004: ; preds = %1001, %974
 22854|  %1005 = phi i64 [ %1003, %1001 ], [ %975, %974 ]                                                                      ;L0<279<146<128<52<3674<142<70
 22855|     ;; acc = i64 %1005
 22856|     ;; i = i64 4
 22857|     ;; self = ptr %66
 22858|     ;; count = i64 4
 22859|  %1006 = gep %66, i64 32                                                                                               ;L656<279<146<128<52<3674<142<70
 22860|  %1007 = load ptr, ptr %1006, , !!32106, !!8                                                                           ;L279<146<128<52<3674<142<70
 22862|     ;; acc = i64 %1005
 22864|  %1008 = icmp eq ptr %1007, null                                                                                       ;L39<279<146<128<52<3674<142<70
 22865|  br i1 %1008, label %1034, label %1009                                                                                 ;L39<279<146<128<52<3674<142<70
 22866| 
 22867| 1009: ; preds = %1004
 22868|     ;; x = ptr %1007
 22870|     ;; acc = i64 %1005
 22871|     ;; elt = ptr %1007
 22872|     ;; x = ptr %1007
 22876|  %1010 = gep %1007, i64 1472                                                                                           ;L69<138<88<40<279<146<128<52<3674<142<70
 22877|  %1011 = load i64, ptr %1010, , !!32106, !!8                                                                           ;L69<138<88<40<279<146<128<52<3674<142<70
 22878|  %1012 = icmp eq i64 %1011, %887                                                                                       ;L69<138<88<40<279<146<128<52<3674<142<70
 22879|  br i1 %1012, label %1031, label %1013                                                                                 ;L69<138<88<40<279<146<128<52<3674<142<70
 22880| 
 22881| 1013: ; preds = %1009
 22882|     ;; self = ptr %1007
 22883|     ;; other = ptr %68
 22884|  %1014 = gep %1007, i64 1632                                                                                           ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22885|  %1015 = load i64, ptr %1014, , !!32106, !!8                                                                           ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22886|     ;; x1 = i64 %1015
 22887|     ;; self = i64 %1015
 22888|  %1016 = gep %1007, i64 1640                                                                                           ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22889|  %1017 = load i64, ptr %1016, , !!32106, !!8                                                                           ;L2158<69<138<88<40<279<146<128<52<3674<142<70
 22890|     ;; y1 = i64 %1017
 22891|     ;; self = i64 %1017
 22892|     ;; x2 = i64 %888
 22893|     ;; other = i64 %888
 22894|     ;; y2 = i64 %889
 22895|     ;; other = i64 %889
 22896|  %1018 = icmp ult i64 %1015, %888                                                                                      ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22897|  %1019 = sub nuw i64 %888, %1015                                                                                       ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22898|  %1020 = sub nuw i64 %1015, %888                                                                                       ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22899|  %1021 = select i1 %1018, i64 %1019, i64 %1020                                                                         ;L3147<7<2158<69<138<88<40<279<146<128<52<3674<142<70
 22900|     ;; dx = i64 %1021
 22901|  %1022 = icmp ult i64 %1017, %889                                                                                      ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22902|  %1023 = sub nuw i64 %889, %1017                                                                                       ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22903|  %1024 = sub nuw i64 %1017, %889                                                                                       ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22904|  %1025 = select i1 %1022, i64 %1023, i64 %1024                                                                         ;L3147<8<2158<69<138<88<40<279<146<128<52<3674<142<70
 22905|     ;; dy = i64 %1025
 22906|  %1026 = mul i64 %1021, %1021                                                                                          ;L9<2158<69<138<88<40<279<146<128<52<3674<142<70
 22907|  %1027 = mul i64 %1025, %1025                                                                                          ;L9<2158<69<138<88<40<279<146<128<52<3674<142<70
 22908|  %1028 = add i64 %1027, %1026                                                                                          ;L9<2158<69<138<88<40<279<146<128<52<3674<142<70
 22909|  %1029 = icmp ult i64 %1028, 22500000001                                                                               ;L69<138<88<40<279<146<128<52<3674<142<70
 22910|  %1030 = zext i1 %1029 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<70
 22911|  br label %1031                                                                                                        ;L69<138<88<40<279<146<128<52<3674<142<70
 22912| 
 22913| 1031: ; preds = %1013, %1009
 22914|  %1032 = phi i64 [ %1030, %1013 ], [ 0, %1009 ]                                                                        ;L69<138<88<40<279<146<128<52<3674<142<70
 22916|     ;; a = i64 %1005
 22917|     ;; b = i64 %1032
 22918|  %1033 = add nuw nsw i64 %1032, %1005                                                                                  ;L55<88<40<279<146<128<52<3674<142<70
 22919|  br label %1034                                                                                                        ;L42<279<146<128<52<3674<142<70
 22920| 
 22921| 1034: ; preds = %1031, %1004
 22922|  %1035 = phi i64 [ %1033, %1031 ], [ %1005, %1004 ]                                                                    ;L0<279<146<128<52<3674<142<70
 22923|     ;; acc = i64 %1035
 22924|     ;; i = i64 5
 22925|     ;; nearby_allies = i64 %1035
 22926|     ;; self[0..+8] = ptr %358
 22927|     ;; slice[0..+8] = ptr %358
 22928|     ;; self[8..+8] = i64 5
 22929|     ;; slice[8..+8] = i64 5
 22930|     ;; self = ptr %358
 22931|     ;; self[0..+8] = ptr %358
 22932|     ;; self[8..+8] = ptr %359
 22933|     ;; self[16..+8] = ptr %68
 22934|     ;; init = i64 0
 22937|     ;; self[0..+8] = ptr %358
 22938|     ;; iter[0..+8] = ptr %358
 22939|     ;; self[0..+8] = ptr %358
 22940|     ;; self[8..+8] = ptr %359
 22941|     ;; iter[8..+8] = ptr %359
 22942|     ;; self[8..+8] = ptr %359
 22943|     ;; self[16..+8] = ptr %68
 22944|     ;; iter[16..+8] = ptr %68
 22945|     ;; self[16..+8] = ptr %68
 22947|     ;; self[0..+8] = ptr %358
 22948|     ;; self[8..+8] = ptr %359
 22949|     ;; init = i64 0
 22950|     ;; fold = ptr %68
 22952|     ;; f = ptr %68
 22953|     ;; self[0..+8] = ptr %358
 22954|     ;; self[8..+8] = ptr %359
 22955|     ;; init = i64 0
 22956|     ;; acc = i64 0
 22957|     ;; i = i64 0
 22958|     ;; len = i64 5
 22959|  %1036 = load i64, ptr %68, , !!32328
 22960|  %1037 = freeze i64 %1036
 22961|  %1038 = trunc i64 %1037 to i1
 22962|  %1039 = gep %68, i64 8
 22963|  %1040 = load i64, ptr %1039, , !!32328
 22964|  %1041 = freeze i64 %1040
 22965|  br i1 %1038, label %1043, label %1160
 22966| 
 22967| 1042: ; preds = %427, %422
 22970|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %31, ptr %5, ptr %4, i64 5, i1 zeroext false)
 22971|  to label %1409 unwind label %57                                                                                       ;L91
 22972| 
 22973| 1043: ; preds = %1034
 22974|     ;; acc = i64 0
 22975|     ;; i = i64 0
 22976|     ;; self = ptr %358
 22977|     ;; count = i64 0
 22978|  %1044 = load ptr, ptr %358, , !!32339, !!8                                                                            ;L279<146<128<52<3674<142<73
 22980|     ;; acc = i64 0
 22982|  %1045 = icmp eq ptr %1044, null                                                                                       ;L39<279<146<128<52<3674<142<73
 22983|  br i1 %1045, label %1064, label %1046                                                                                 ;L39<279<146<128<52<3674<142<73
 22984| 
 22985| 1046: ; preds = %1043
 22986|     ;; x = ptr %1044
 22988|     ;; acc = i64 0
 22989|     ;; elt = ptr %1044
 22990|     ;; x = ptr %1044
 22994|     ;; self = ptr %1044
 22995|     ;; entity = ptr %68
 22996|     ;; self = ptr %1044
 22997|     ;; other = ptr %68
 22998|  %1047 = gep %1044, i64 1632                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 22999|  %1048 = load i64, ptr %1047, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23000|     ;; x1 = i64 %1048
 23001|     ;; self = i64 %1048
 23002|  %1049 = gep %1044, i64 1640                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23003|  %1050 = load i64, ptr %1049, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23004|     ;; y1 = i64 %1050
 23005|     ;; self = i64 %1050
 23006|     ;; x2 = i64 %888
 23007|     ;; other = i64 %888
 23008|     ;; y2 = i64 %889
 23009|     ;; other = i64 %889
 23010|  %1051 = icmp ult i64 %1048, %888                                                                                      ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23011|  %1052 = sub nuw i64 %888, %1048                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23012|  %1053 = sub nuw i64 %1048, %888                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23013|  %1054 = select i1 %1051, i64 %1052, i64 %1053                                                                         ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23014|     ;; dx = i64 %1054
 23015|  %1055 = icmp ult i64 %1050, %889                                                                                      ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23016|  %1056 = sub nuw i64 %889, %1050                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23017|  %1057 = sub nuw i64 %1050, %889                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23018|  %1058 = select i1 %1055, i64 %1056, i64 %1057                                                                         ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23019|     ;; dy = i64 %1058
 23020|  %1059 = mul i64 %1054, %1054                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23021|  %1060 = mul i64 %1058, %1058                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23022|  %1061 = add i64 %1060, %1059                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23023|  %1062 = icmp ult i64 %1061, 22500000001                                                                               ;L72<138<88<40<279<146<128<52<3674<142<73
 23024|  %1063 = zext i1 %1062 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<73
 23026|     ;; a = i64 0
 23028|  br label %1064                                                                                                        ;L42<279<146<128<52<3674<142<73
 23029| 
 23030| 1064: ; preds = %1046, %1043
 23031|  %1065 = phi i64 [ %1063, %1046 ], [ 0, %1043 ]                                                                        ;L0<279<146<128<52<3674<142<73
 23032|     ;; acc = i64 %1065
 23033|     ;; i = i64 1
 23034|     ;; self = ptr %358
 23035|     ;; count = i64 1
 23036|  %1066 = gep %358, i64 8                                                                                               ;L656<279<146<128<52<3674<142<73
 23037|  %1067 = load ptr, ptr %1066, , !!32339, !!8                                                                           ;L279<146<128<52<3674<142<73
 23039|     ;; acc = i64 %1065
 23041|  %1068 = icmp eq ptr %1067, null                                                                                       ;L39<279<146<128<52<3674<142<73
 23042|  br i1 %1068, label %1088, label %1069                                                                                 ;L39<279<146<128<52<3674<142<73
 23043| 
 23044| 1069: ; preds = %1064
 23045|     ;; x = ptr %1067
 23047|     ;; acc = i64 %1065
 23048|     ;; elt = ptr %1067
 23049|     ;; x = ptr %1067
 23053|     ;; self = ptr %1067
 23054|     ;; entity = ptr %68
 23055|     ;; self = ptr %1067
 23056|     ;; other = ptr %68
 23057|  %1070 = gep %1067, i64 1632                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23058|  %1071 = load i64, ptr %1070, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23059|     ;; x1 = i64 %1071
 23060|     ;; self = i64 %1071
 23061|  %1072 = gep %1067, i64 1640                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23062|  %1073 = load i64, ptr %1072, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23063|     ;; y1 = i64 %1073
 23064|     ;; self = i64 %1073
 23065|     ;; x2 = i64 %888
 23066|     ;; other = i64 %888
 23067|     ;; y2 = i64 %889
 23068|     ;; other = i64 %889
 23069|  %1074 = icmp ult i64 %1071, %888                                                                                      ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23070|  %1075 = sub nuw i64 %888, %1071                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23071|  %1076 = sub nuw i64 %1071, %888                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23072|  %1077 = select i1 %1074, i64 %1075, i64 %1076                                                                         ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23073|     ;; dx = i64 %1077
 23074|  %1078 = icmp ult i64 %1073, %889                                                                                      ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23075|  %1079 = sub nuw i64 %889, %1073                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23076|  %1080 = sub nuw i64 %1073, %889                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23077|  %1081 = select i1 %1078, i64 %1079, i64 %1080                                                                         ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23078|     ;; dy = i64 %1081
 23079|  %1082 = mul i64 %1077, %1077                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23080|  %1083 = mul i64 %1081, %1081                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23081|  %1084 = add i64 %1083, %1082                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23082|  %1085 = icmp ult i64 %1084, 22500000001                                                                               ;L72<138<88<40<279<146<128<52<3674<142<73
 23083|  %1086 = zext i1 %1085 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<73
 23085|     ;; a = i64 %1065
 23087|  %1087 = add nuw nsw i64 %1065, %1086                                                                                  ;L55<88<40<279<146<128<52<3674<142<73
 23088|  br label %1088                                                                                                        ;L42<279<146<128<52<3674<142<73
 23089| 
 23090| 1088: ; preds = %1069, %1064
 23091|  %1089 = phi i64 [ %1087, %1069 ], [ %1065, %1064 ]                                                                    ;L0<279<146<128<52<3674<142<73
 23092|     ;; acc = i64 %1089
 23093|     ;; i = i64 2
 23094|     ;; self = ptr %358
 23095|     ;; count = i64 2
 23096|  %1090 = gep %358, i64 16                                                                                              ;L656<279<146<128<52<3674<142<73
 23097|  %1091 = load ptr, ptr %1090, , !!32339, !!8                                                                           ;L279<146<128<52<3674<142<73
 23099|     ;; acc = i64 %1089
 23101|  %1092 = icmp eq ptr %1091, null                                                                                       ;L39<279<146<128<52<3674<142<73
 23102|  br i1 %1092, label %1112, label %1093                                                                                 ;L39<279<146<128<52<3674<142<73
 23103| 
 23104| 1093: ; preds = %1088
 23105|     ;; x = ptr %1091
 23107|     ;; acc = i64 %1089
 23108|     ;; elt = ptr %1091
 23109|     ;; x = ptr %1091
 23113|     ;; self = ptr %1091
 23114|     ;; entity = ptr %68
 23115|     ;; self = ptr %1091
 23116|     ;; other = ptr %68
 23117|  %1094 = gep %1091, i64 1632                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23118|  %1095 = load i64, ptr %1094, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23119|     ;; x1 = i64 %1095
 23120|     ;; self = i64 %1095
 23121|  %1096 = gep %1091, i64 1640                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23122|  %1097 = load i64, ptr %1096, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23123|     ;; y1 = i64 %1097
 23124|     ;; self = i64 %1097
 23125|     ;; x2 = i64 %888
 23126|     ;; other = i64 %888
 23127|     ;; y2 = i64 %889
 23128|     ;; other = i64 %889
 23129|  %1098 = icmp ult i64 %1095, %888                                                                                      ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23130|  %1099 = sub nuw i64 %888, %1095                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23131|  %1100 = sub nuw i64 %1095, %888                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23132|  %1101 = select i1 %1098, i64 %1099, i64 %1100                                                                         ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23133|     ;; dx = i64 %1101
 23134|  %1102 = icmp ult i64 %1097, %889                                                                                      ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23135|  %1103 = sub nuw i64 %889, %1097                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23136|  %1104 = sub nuw i64 %1097, %889                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23137|  %1105 = select i1 %1102, i64 %1103, i64 %1104                                                                         ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23138|     ;; dy = i64 %1105
 23139|  %1106 = mul i64 %1101, %1101                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23140|  %1107 = mul i64 %1105, %1105                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23141|  %1108 = add i64 %1107, %1106                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23142|  %1109 = icmp ult i64 %1108, 22500000001                                                                               ;L72<138<88<40<279<146<128<52<3674<142<73
 23143|  %1110 = zext i1 %1109 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<73
 23145|     ;; a = i64 %1089
 23147|  %1111 = add nuw nsw i64 %1089, %1110                                                                                  ;L55<88<40<279<146<128<52<3674<142<73
 23148|  br label %1112                                                                                                        ;L42<279<146<128<52<3674<142<73
 23149| 
 23150| 1112: ; preds = %1093, %1088
 23151|  %1113 = phi i64 [ %1111, %1093 ], [ %1089, %1088 ]                                                                    ;L0<279<146<128<52<3674<142<73
 23152|     ;; acc = i64 %1113
 23153|     ;; i = i64 3
 23154|     ;; self = ptr %358
 23155|     ;; count = i64 3
 23156|  %1114 = gep %358, i64 24                                                                                              ;L656<279<146<128<52<3674<142<73
 23157|  %1115 = load ptr, ptr %1114, , !!32339, !!8                                                                           ;L279<146<128<52<3674<142<73
 23159|     ;; acc = i64 %1113
 23161|  %1116 = icmp eq ptr %1115, null                                                                                       ;L39<279<146<128<52<3674<142<73
 23162|  br i1 %1116, label %1136, label %1117                                                                                 ;L39<279<146<128<52<3674<142<73
 23163| 
 23164| 1117: ; preds = %1112
 23165|     ;; x = ptr %1115
 23167|     ;; acc = i64 %1113
 23168|     ;; elt = ptr %1115
 23169|     ;; x = ptr %1115
 23173|     ;; self = ptr %1115
 23174|     ;; entity = ptr %68
 23175|     ;; self = ptr %1115
 23176|     ;; other = ptr %68
 23177|  %1118 = gep %1115, i64 1632                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23178|  %1119 = load i64, ptr %1118, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23179|     ;; x1 = i64 %1119
 23180|     ;; self = i64 %1119
 23181|  %1120 = gep %1115, i64 1640                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23182|  %1121 = load i64, ptr %1120, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23183|     ;; y1 = i64 %1121
 23184|     ;; self = i64 %1121
 23185|     ;; x2 = i64 %888
 23186|     ;; other = i64 %888
 23187|     ;; y2 = i64 %889
 23188|     ;; other = i64 %889
 23189|  %1122 = icmp ult i64 %1119, %888                                                                                      ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23190|  %1123 = sub nuw i64 %888, %1119                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23191|  %1124 = sub nuw i64 %1119, %888                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23192|  %1125 = select i1 %1122, i64 %1123, i64 %1124                                                                         ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23193|     ;; dx = i64 %1125
 23194|  %1126 = icmp ult i64 %1121, %889                                                                                      ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23195|  %1127 = sub nuw i64 %889, %1121                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23196|  %1128 = sub nuw i64 %1121, %889                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23197|  %1129 = select i1 %1126, i64 %1127, i64 %1128                                                                         ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23198|     ;; dy = i64 %1129
 23199|  %1130 = mul i64 %1125, %1125                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23200|  %1131 = mul i64 %1129, %1129                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23201|  %1132 = add i64 %1131, %1130                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23202|  %1133 = icmp ult i64 %1132, 22500000001                                                                               ;L72<138<88<40<279<146<128<52<3674<142<73
 23203|  %1134 = zext i1 %1133 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<73
 23205|     ;; a = i64 %1113
 23207|  %1135 = add nuw nsw i64 %1113, %1134                                                                                  ;L55<88<40<279<146<128<52<3674<142<73
 23208|  br label %1136                                                                                                        ;L42<279<146<128<52<3674<142<73
 23209| 
 23210| 1136: ; preds = %1117, %1112
 23211|  %1137 = phi i64 [ %1135, %1117 ], [ %1113, %1112 ]                                                                    ;L0<279<146<128<52<3674<142<73
 23212|     ;; acc = i64 %1137
 23213|     ;; i = i64 4
 23214|     ;; self = ptr %358
 23215|     ;; count = i64 4
 23216|  %1138 = gep %358, i64 32                                                                                              ;L656<279<146<128<52<3674<142<73
 23217|  %1139 = load ptr, ptr %1138, , !!32339, !!8                                                                           ;L279<146<128<52<3674<142<73
 23219|     ;; acc = i64 %1137
 23221|  %1140 = icmp eq ptr %1139, null                                                                                       ;L39<279<146<128<52<3674<142<73
 23222|  br i1 %1140, label %1331, label %1141                                                                                 ;L39<279<146<128<52<3674<142<73
 23223| 
 23224| 1141: ; preds = %1136
 23225|     ;; x = ptr %1139
 23227|     ;; acc = i64 %1137
 23228|     ;; elt = ptr %1139
 23229|     ;; x = ptr %1139
 23233|     ;; self = ptr %1139
 23234|     ;; entity = ptr %68
 23235|     ;; self = ptr %1139
 23236|     ;; other = ptr %68
 23237|  %1142 = gep %1139, i64 1632                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23238|  %1143 = load i64, ptr %1142, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23239|     ;; x1 = i64 %1143
 23240|     ;; self = i64 %1143
 23241|  %1144 = gep %1139, i64 1640                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23242|  %1145 = load i64, ptr %1144, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23243|     ;; y1 = i64 %1145
 23244|     ;; self = i64 %1145
 23245|     ;; x2 = i64 %888
 23246|     ;; other = i64 %888
 23247|     ;; y2 = i64 %889
 23248|     ;; other = i64 %889
 23249|  %1146 = icmp ult i64 %1143, %888                                                                                      ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23250|  %1147 = sub nuw i64 %888, %1143                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23251|  %1148 = sub nuw i64 %1143, %888                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23252|  %1149 = select i1 %1146, i64 %1147, i64 %1148                                                                         ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23253|     ;; dx = i64 %1149
 23254|  %1150 = icmp ult i64 %1145, %889                                                                                      ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23255|  %1151 = sub nuw i64 %889, %1145                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23256|  %1152 = sub nuw i64 %1145, %889                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23257|  %1153 = select i1 %1150, i64 %1151, i64 %1152                                                                         ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23258|     ;; dy = i64 %1153
 23259|  %1154 = mul i64 %1149, %1149                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23260|  %1155 = mul i64 %1153, %1153                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23261|  %1156 = add i64 %1155, %1154                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23262|  %1157 = icmp ult i64 %1156, 22500000001                                                                               ;L72<138<88<40<279<146<128<52<3674<142<73
 23263|  %1158 = zext i1 %1157 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<73
 23265|     ;; a = i64 %1137
 23267|  %1159 = add nuw nsw i64 %1137, %1158                                                                                  ;L55<88<40<279<146<128<52<3674<142<73
 23268|  br label %1331                                                                                                        ;L42<279<146<128<52<3674<142<73
 23269| 
 23270| 1160: ; preds = %1034
 23271|  %1161 = icmp ult i64 %1041, 2
 23272|     ;; i = i64 0
 23273|     ;; i = i64 0
 23274|     ;; self = ptr %358
 23275|     ;; self = ptr %358
 23276|     ;; count = i64 0
 23277|     ;; count = i64 0
 23278|  %1162 = load ptr, ptr %358, , !!32339, !!8                                                                            ;L279<146<128<52<3674<142<73
 23283|  %1163 = icmp eq ptr %1162, null                                                                                       ;L39<279<146<128<52<3674<142<73
 23284|  br i1 %1161, label %1165, label %1164
 23285| 
 23286| 1164: ; preds = %1160
 23289|  br i1 %1163, label %1315, label %1313                                                                                 ;L39<279<146<128<52<3674<142<73
 23290| 
 23291| 1165: ; preds = %1160
 23292|     ;; acc = i64 0
 23293|     ;; acc = i64 0
 23294|  br i1 %1163, label %1189, label %1166                                                                                 ;L39<279<146<128<52<3674<142<73
 23295| 
 23296| 1166: ; preds = %1165
 23297|     ;; x = ptr %1162
 23299|     ;; acc = i64 0
 23300|     ;; elt = ptr %1162
 23301|     ;; x = ptr %1162
 23305|     ;; self = ptr %1162
 23306|     ;; entity = ptr %68
 23307|     ;; team = i64 %1040
 23309|  %1167 = gep %1162, i64 56                                                                                             ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23310|  %1168 = gepS %1167, i64 %1041                                                                                         ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23311|  %1169 = load i64, ptr %1168, , !!32339, !!8                                                                           ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23312|  %1170 = icmp eq i64 %1169, 0                                                                                          ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23313|  br i1 %1170, label %1171, label %1189                                                                                 ;L72<138<88<40<279<146<128<52<3674<142<73
 23314| 
 23315| 1171: ; preds = %1166
 23316|     ;; self = ptr %1162
 23317|     ;; other = ptr %68
 23318|  %1172 = gep %1162, i64 1632                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23319|  %1173 = load i64, ptr %1172, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23320|     ;; x1 = i64 %1173
 23321|     ;; self = i64 %1173
 23322|  %1174 = gep %1162, i64 1640                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23323|  %1175 = load i64, ptr %1174, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23324|     ;; y1 = i64 %1175
 23325|     ;; self = i64 %1175
 23326|     ;; x2 = i64 %888
 23327|     ;; other = i64 %888
 23328|     ;; y2 = i64 %889
 23329|     ;; other = i64 %889
 23330|  %1176 = icmp ult i64 %1173, %888                                                                                      ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23331|  %1177 = sub nuw i64 %888, %1173                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23332|  %1178 = sub nuw i64 %1173, %888                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23333|  %1179 = select i1 %1176, i64 %1177, i64 %1178                                                                         ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23334|     ;; dx = i64 %1179
 23335|  %1180 = icmp ult i64 %1175, %889                                                                                      ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23336|  %1181 = sub nuw i64 %889, %1175                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23337|  %1182 = sub nuw i64 %1175, %889                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23338|  %1183 = select i1 %1180, i64 %1181, i64 %1182                                                                         ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23339|     ;; dy = i64 %1183
 23340|  %1184 = mul i64 %1179, %1179                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23341|  %1185 = mul i64 %1183, %1183                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23342|  %1186 = add i64 %1185, %1184                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23343|  %1187 = icmp ult i64 %1186, 22500000001                                                                               ;L72<138<88<40<279<146<128<52<3674<142<73
 23344|  %1188 = zext i1 %1187 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<73
 23345|  br label %1189                                                                                                        ;L72<138<88<40<279<146<128<52<3674<142<73
 23346| 
 23347| 1189: ; preds = %1171, %1166, %1165
 23348|  %1190 = phi i64 [ 0, %1165 ], [ %1188, %1171 ], [ 0, %1166 ]                                                          ;L0<279<146<128<52<3674<142<73
 23349|     ;; acc = i64 %1190
 23350|     ;; i = i64 1
 23351|     ;; self = ptr %358
 23352|     ;; count = i64 1
 23353|  %1191 = gep %358, i64 8                                                                                               ;L656<279<146<128<52<3674<142<73
 23354|  %1192 = load ptr, ptr %1191, , !!32339, !!8                                                                           ;L279<146<128<52<3674<142<73
 23356|     ;; acc = i64 %1190
 23358|  %1193 = icmp eq ptr %1192, null                                                                                       ;L39<279<146<128<52<3674<142<73
 23359|  br i1 %1193, label %1220, label %1194                                                                                 ;L39<279<146<128<52<3674<142<73
 23360| 
 23361| 1194: ; preds = %1189
 23362|     ;; x = ptr %1192
 23364|     ;; acc = i64 %1190
 23365|     ;; elt = ptr %1192
 23366|     ;; x = ptr %1192
 23370|     ;; self = ptr %1192
 23371|     ;; entity = ptr %68
 23372|     ;; team = i64 %1040
 23374|  %1195 = gep %1192, i64 56                                                                                             ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23375|  %1196 = gepS %1195, i64 %1041                                                                                         ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23376|  %1197 = load i64, ptr %1196, , !!32339, !!8                                                                           ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23377|  %1198 = icmp eq i64 %1197, 0                                                                                          ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23378|  br i1 %1198, label %1199, label %1217                                                                                 ;L72<138<88<40<279<146<128<52<3674<142<73
 23379| 
 23380| 1199: ; preds = %1194
 23381|     ;; self = ptr %1192
 23382|     ;; other = ptr %68
 23383|  %1200 = gep %1192, i64 1632                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23384|  %1201 = load i64, ptr %1200, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23385|     ;; x1 = i64 %1201
 23386|     ;; self = i64 %1201
 23387|  %1202 = gep %1192, i64 1640                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23388|  %1203 = load i64, ptr %1202, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23389|     ;; y1 = i64 %1203
 23390|     ;; self = i64 %1203
 23391|     ;; x2 = i64 %888
 23392|     ;; other = i64 %888
 23393|     ;; y2 = i64 %889
 23394|     ;; other = i64 %889
 23395|  %1204 = icmp ult i64 %1201, %888                                                                                      ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23396|  %1205 = sub nuw i64 %888, %1201                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23397|  %1206 = sub nuw i64 %1201, %888                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23398|  %1207 = select i1 %1204, i64 %1205, i64 %1206                                                                         ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23399|     ;; dx = i64 %1207
 23400|  %1208 = icmp ult i64 %1203, %889                                                                                      ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23401|  %1209 = sub nuw i64 %889, %1203                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23402|  %1210 = sub nuw i64 %1203, %889                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23403|  %1211 = select i1 %1208, i64 %1209, i64 %1210                                                                         ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23404|     ;; dy = i64 %1211
 23405|  %1212 = mul i64 %1207, %1207                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23406|  %1213 = mul i64 %1211, %1211                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23407|  %1214 = add i64 %1213, %1212                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23408|  %1215 = icmp ult i64 %1214, 22500000001                                                                               ;L72<138<88<40<279<146<128<52<3674<142<73
 23409|  %1216 = zext i1 %1215 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<73
 23410|  br label %1217                                                                                                        ;L72<138<88<40<279<146<128<52<3674<142<73
 23411| 
 23412| 1217: ; preds = %1199, %1194
 23413|  %1218 = phi i64 [ %1216, %1199 ], [ 0, %1194 ]                                                                        ;L72<138<88<40<279<146<128<52<3674<142<73
 23415|     ;; a = i64 %1190
 23416|     ;; b = i64 %1218
 23417|  %1219 = add nuw nsw i64 %1218, %1190                                                                                  ;L55<88<40<279<146<128<52<3674<142<73
 23418|  br label %1220                                                                                                        ;L42<279<146<128<52<3674<142<73
 23419| 
 23420| 1220: ; preds = %1217, %1189
 23421|  %1221 = phi i64 [ %1219, %1217 ], [ %1190, %1189 ]                                                                    ;L0<279<146<128<52<3674<142<73
 23422|     ;; acc = i64 %1221
 23423|     ;; i = i64 2
 23424|     ;; self = ptr %358
 23425|     ;; count = i64 2
 23426|  %1222 = gep %358, i64 16                                                                                              ;L656<279<146<128<52<3674<142<73
 23427|  %1223 = load ptr, ptr %1222, , !!32339, !!8                                                                           ;L279<146<128<52<3674<142<73
 23429|     ;; acc = i64 %1221
 23431|  %1224 = icmp eq ptr %1223, null                                                                                       ;L39<279<146<128<52<3674<142<73
 23432|  br i1 %1224, label %1251, label %1225                                                                                 ;L39<279<146<128<52<3674<142<73
 23433| 
 23434| 1225: ; preds = %1220
 23435|     ;; x = ptr %1223
 23437|     ;; acc = i64 %1221
 23438|     ;; elt = ptr %1223
 23439|     ;; x = ptr %1223
 23443|     ;; self = ptr %1223
 23444|     ;; entity = ptr %68
 23445|     ;; team = i64 %1040
 23447|  %1226 = gep %1223, i64 56                                                                                             ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23448|  %1227 = gepS %1226, i64 %1041                                                                                         ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23449|  %1228 = load i64, ptr %1227, , !!32339, !!8                                                                           ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23450|  %1229 = icmp eq i64 %1228, 0                                                                                          ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23451|  br i1 %1229, label %1230, label %1248                                                                                 ;L72<138<88<40<279<146<128<52<3674<142<73
 23452| 
 23453| 1230: ; preds = %1225
 23454|     ;; self = ptr %1223
 23455|     ;; other = ptr %68
 23456|  %1231 = gep %1223, i64 1632                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23457|  %1232 = load i64, ptr %1231, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23458|     ;; x1 = i64 %1232
 23459|     ;; self = i64 %1232
 23460|  %1233 = gep %1223, i64 1640                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23461|  %1234 = load i64, ptr %1233, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23462|     ;; y1 = i64 %1234
 23463|     ;; self = i64 %1234
 23464|     ;; x2 = i64 %888
 23465|     ;; other = i64 %888
 23466|     ;; y2 = i64 %889
 23467|     ;; other = i64 %889
 23468|  %1235 = icmp ult i64 %1232, %888                                                                                      ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23469|  %1236 = sub nuw i64 %888, %1232                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23470|  %1237 = sub nuw i64 %1232, %888                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23471|  %1238 = select i1 %1235, i64 %1236, i64 %1237                                                                         ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23472|     ;; dx = i64 %1238
 23473|  %1239 = icmp ult i64 %1234, %889                                                                                      ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23474|  %1240 = sub nuw i64 %889, %1234                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23475|  %1241 = sub nuw i64 %1234, %889                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23476|  %1242 = select i1 %1239, i64 %1240, i64 %1241                                                                         ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23477|     ;; dy = i64 %1242
 23478|  %1243 = mul i64 %1238, %1238                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23479|  %1244 = mul i64 %1242, %1242                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23480|  %1245 = add i64 %1244, %1243                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23481|  %1246 = icmp ult i64 %1245, 22500000001                                                                               ;L72<138<88<40<279<146<128<52<3674<142<73
 23482|  %1247 = zext i1 %1246 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<73
 23483|  br label %1248                                                                                                        ;L72<138<88<40<279<146<128<52<3674<142<73
 23484| 
 23485| 1248: ; preds = %1230, %1225
 23486|  %1249 = phi i64 [ %1247, %1230 ], [ 0, %1225 ]                                                                        ;L72<138<88<40<279<146<128<52<3674<142<73
 23488|     ;; a = i64 %1221
 23489|     ;; b = i64 %1249
 23490|  %1250 = add nuw nsw i64 %1249, %1221                                                                                  ;L55<88<40<279<146<128<52<3674<142<73
 23491|  br label %1251                                                                                                        ;L42<279<146<128<52<3674<142<73
 23492| 
 23493| 1251: ; preds = %1248, %1220
 23494|  %1252 = phi i64 [ %1250, %1248 ], [ %1221, %1220 ]                                                                    ;L0<279<146<128<52<3674<142<73
 23495|     ;; acc = i64 %1252
 23496|     ;; i = i64 3
 23497|     ;; self = ptr %358
 23498|     ;; count = i64 3
 23499|  %1253 = gep %358, i64 24                                                                                              ;L656<279<146<128<52<3674<142<73
 23500|  %1254 = load ptr, ptr %1253, , !!32339, !!8                                                                           ;L279<146<128<52<3674<142<73
 23502|     ;; acc = i64 %1252
 23504|  %1255 = icmp eq ptr %1254, null                                                                                       ;L39<279<146<128<52<3674<142<73
 23505|  br i1 %1255, label %1282, label %1256                                                                                 ;L39<279<146<128<52<3674<142<73
 23506| 
 23507| 1256: ; preds = %1251
 23508|     ;; x = ptr %1254
 23510|     ;; acc = i64 %1252
 23511|     ;; elt = ptr %1254
 23512|     ;; x = ptr %1254
 23516|     ;; self = ptr %1254
 23517|     ;; entity = ptr %68
 23518|     ;; team = i64 %1040
 23520|  %1257 = gep %1254, i64 56                                                                                             ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23521|  %1258 = gepS %1257, i64 %1041                                                                                         ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23522|  %1259 = load i64, ptr %1258, , !!32339, !!8                                                                           ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23523|  %1260 = icmp eq i64 %1259, 0                                                                                          ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23524|  br i1 %1260, label %1261, label %1279                                                                                 ;L72<138<88<40<279<146<128<52<3674<142<73
 23525| 
 23526| 1261: ; preds = %1256
 23527|     ;; self = ptr %1254
 23528|     ;; other = ptr %68
 23529|  %1262 = gep %1254, i64 1632                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23530|  %1263 = load i64, ptr %1262, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23531|     ;; x1 = i64 %1263
 23532|     ;; self = i64 %1263
 23533|  %1264 = gep %1254, i64 1640                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23534|  %1265 = load i64, ptr %1264, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23535|     ;; y1 = i64 %1265
 23536|     ;; self = i64 %1265
 23537|     ;; x2 = i64 %888
 23538|     ;; other = i64 %888
 23539|     ;; y2 = i64 %889
 23540|     ;; other = i64 %889
 23541|  %1266 = icmp ult i64 %1263, %888                                                                                      ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23542|  %1267 = sub nuw i64 %888, %1263                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23543|  %1268 = sub nuw i64 %1263, %888                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23544|  %1269 = select i1 %1266, i64 %1267, i64 %1268                                                                         ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23545|     ;; dx = i64 %1269
 23546|  %1270 = icmp ult i64 %1265, %889                                                                                      ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23547|  %1271 = sub nuw i64 %889, %1265                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23548|  %1272 = sub nuw i64 %1265, %889                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23549|  %1273 = select i1 %1270, i64 %1271, i64 %1272                                                                         ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23550|     ;; dy = i64 %1273
 23551|  %1274 = mul i64 %1269, %1269                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23552|  %1275 = mul i64 %1273, %1273                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23553|  %1276 = add i64 %1275, %1274                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23554|  %1277 = icmp ult i64 %1276, 22500000001                                                                               ;L72<138<88<40<279<146<128<52<3674<142<73
 23555|  %1278 = zext i1 %1277 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<73
 23556|  br label %1279                                                                                                        ;L72<138<88<40<279<146<128<52<3674<142<73
 23557| 
 23558| 1279: ; preds = %1261, %1256
 23559|  %1280 = phi i64 [ %1278, %1261 ], [ 0, %1256 ]                                                                        ;L72<138<88<40<279<146<128<52<3674<142<73
 23561|     ;; a = i64 %1252
 23562|     ;; b = i64 %1280
 23563|  %1281 = add nuw nsw i64 %1280, %1252                                                                                  ;L55<88<40<279<146<128<52<3674<142<73
 23564|  br label %1282                                                                                                        ;L42<279<146<128<52<3674<142<73
 23565| 
 23566| 1282: ; preds = %1279, %1251
 23567|  %1283 = phi i64 [ %1281, %1279 ], [ %1252, %1251 ]                                                                    ;L0<279<146<128<52<3674<142<73
 23568|     ;; acc = i64 %1283
 23569|     ;; i = i64 4
 23570|     ;; self = ptr %358
 23571|     ;; count = i64 4
 23572|  %1284 = gep %358, i64 32                                                                                              ;L656<279<146<128<52<3674<142<73
 23573|  %1285 = load ptr, ptr %1284, , !!32339, !!8                                                                           ;L279<146<128<52<3674<142<73
 23575|     ;; acc = i64 %1283
 23577|  %1286 = icmp eq ptr %1285, null                                                                                       ;L39<279<146<128<52<3674<142<73
 23578|  br i1 %1286, label %1331, label %1287                                                                                 ;L39<279<146<128<52<3674<142<73
 23579| 
 23580| 1287: ; preds = %1282
 23581|     ;; x = ptr %1285
 23583|     ;; acc = i64 %1283
 23584|     ;; elt = ptr %1285
 23585|     ;; x = ptr %1285
 23589|     ;; self = ptr %1285
 23590|     ;; entity = ptr %68
 23591|     ;; team = i64 %1040
 23593|  %1288 = gep %1285, i64 56                                                                                             ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23594|  %1289 = gepS %1288, i64 %1041                                                                                         ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23595|  %1290 = load i64, ptr %1289, , !!32339, !!8                                                                           ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23596|  %1291 = icmp eq i64 %1290, 0                                                                                          ;L122<1483<72<138<88<40<279<146<128<52<3674<142<73
 23597|  br i1 %1291, label %1292, label %1310                                                                                 ;L72<138<88<40<279<146<128<52<3674<142<73
 23598| 
 23599| 1292: ; preds = %1287
 23600|     ;; self = ptr %1285
 23601|     ;; other = ptr %68
 23602|  %1293 = gep %1285, i64 1632                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23603|  %1294 = load i64, ptr %1293, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23604|     ;; x1 = i64 %1294
 23605|     ;; self = i64 %1294
 23606|  %1295 = gep %1285, i64 1640                                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23607|  %1296 = load i64, ptr %1295, , !!32339, !!8                                                                           ;L2158<72<138<88<40<279<146<128<52<3674<142<73
 23608|     ;; y1 = i64 %1296
 23609|     ;; self = i64 %1296
 23610|     ;; x2 = i64 %888
 23611|     ;; other = i64 %888
 23612|     ;; y2 = i64 %889
 23613|     ;; other = i64 %889
 23614|  %1297 = icmp ult i64 %1294, %888                                                                                      ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23615|  %1298 = sub nuw i64 %888, %1294                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23616|  %1299 = sub nuw i64 %1294, %888                                                                                       ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23617|  %1300 = select i1 %1297, i64 %1298, i64 %1299                                                                         ;L3147<7<2158<72<138<88<40<279<146<128<52<3674<142<73
 23618|     ;; dx = i64 %1300
 23619|  %1301 = icmp ult i64 %1296, %889                                                                                      ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23620|  %1302 = sub nuw i64 %889, %1296                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23621|  %1303 = sub nuw i64 %1296, %889                                                                                       ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23622|  %1304 = select i1 %1301, i64 %1302, i64 %1303                                                                         ;L3147<8<2158<72<138<88<40<279<146<128<52<3674<142<73
 23623|     ;; dy = i64 %1304
 23624|  %1305 = mul i64 %1300, %1300                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23625|  %1306 = mul i64 %1304, %1304                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23626|  %1307 = add i64 %1306, %1305                                                                                          ;L9<2158<72<138<88<40<279<146<128<52<3674<142<73
 23627|  %1308 = icmp ult i64 %1307, 22500000001                                                                               ;L72<138<88<40<279<146<128<52<3674<142<73
 23628|  %1309 = zext i1 %1308 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<73
 23629|  br label %1310                                                                                                        ;L72<138<88<40<279<146<128<52<3674<142<73
 23630| 
 23631| 1310: ; preds = %1292, %1287
 23632|  %1311 = phi i64 [ %1309, %1292 ], [ 0, %1287 ]                                                                        ;L72<138<88<40<279<146<128<52<3674<142<73
 23634|     ;; a = i64 %1283
 23635|     ;; b = i64 %1311
 23636|  %1312 = add nuw nsw i64 %1311, %1283                                                                                  ;L55<88<40<279<146<128<52<3674<142<73
 23637|  br label %1331                                                                                                        ;L42<279<146<128<52<3674<142<73
 23638| 
 23639| 1313: ; preds = %1327, %1323, %1319, %1315, %1164
 23640|     ;; x = ptr %1162
 23643|     ;; elt = ptr %1162
 23644|     ;; x = ptr %1162
 23648|     ;; self = ptr %1162
 23649|     ;; entity = ptr %68
 23650|     ;; team = i64 %1040
 23651|  invoke void @core::panicking18panic_bounds_check(i64 %1041, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 23652|  to label %1314 unwind label %57                                                                                       ;L1483<72<138<88<40<279<146<128<52<3674<142<73
 23653| 
 23654| 1314: ; preds = %1313
 23655|  unreachable                                                                                                           ;L1483<72<138<88<40<279<146<128<52<3674<142<73
 23656| 
 23657| 1315: ; preds = %1164
 23659|     ;; i = i64 1
 23660|     ;; self = ptr %358
 23661|     ;; count = i64 1
 23662|  %1316 = gep %358, i64 8                                                                                               ;L656<279<146<128<52<3674<142<73
 23663|  %1317 = load ptr, ptr %1316, , !!32339, !!8                                                                           ;L279<146<128<52<3674<142<73
 23667|  %1318 = icmp eq ptr %1317, null                                                                                       ;L39<279<146<128<52<3674<142<73
 23668|  br i1 %1318, label %1319, label %1313                                                                                 ;L39<279<146<128<52<3674<142<73
 23669| 
 23670| 1319: ; preds = %1315
 23672|     ;; i = i64 2
 23673|     ;; self = ptr %358
 23674|     ;; count = i64 2
 23675|  %1320 = gep %358, i64 16                                                                                              ;L656<279<146<128<52<3674<142<73
 23676|  %1321 = load ptr, ptr %1320, , !!32339, !!8                                                                           ;L279<146<128<52<3674<142<73
 23680|  %1322 = icmp eq ptr %1321, null                                                                                       ;L39<279<146<128<52<3674<142<73
 23681|  br i1 %1322, label %1323, label %1313                                                                                 ;L39<279<146<128<52<3674<142<73
 23682| 
 23683| 1323: ; preds = %1319
 23685|     ;; i = i64 3
 23686|     ;; self = ptr %358
 23687|     ;; count = i64 3
 23688|  %1324 = gep %358, i64 24                                                                                              ;L656<279<146<128<52<3674<142<73
 23689|  %1325 = load ptr, ptr %1324, , !!32339, !!8                                                                           ;L279<146<128<52<3674<142<73
 23693|  %1326 = icmp eq ptr %1325, null                                                                                       ;L39<279<146<128<52<3674<142<73
 23694|  br i1 %1326, label %1327, label %1313                                                                                 ;L39<279<146<128<52<3674<142<73
 23695| 
 23696| 1327: ; preds = %1323
 23698|     ;; i = i64 4
 23699|     ;; self = ptr %358
 23700|     ;; count = i64 4
 23701|  %1328 = gep %358, i64 32                                                                                              ;L656<279<146<128<52<3674<142<73
 23702|  %1329 = load ptr, ptr %1328, , !!32339, !!8                                                                           ;L279<146<128<52<3674<142<73
 23706|  %1330 = icmp eq ptr %1329, null                                                                                       ;L39<279<146<128<52<3674<142<73
 23707|  br i1 %1330, label %1331, label %1313                                                                                 ;L39<279<146<128<52<3674<142<73
 23708| 
 23709| 1331: ; preds = %1327, %1310, %1282, %1141, %1136
 23710|  %1332 = phi i64 [ %1137, %1136 ], [ %1283, %1282 ], [ %1159, %1141 ], [ %1312, %1310 ], [ 0, %1327 ]                  ;L0<146<128<52<3674<142<73
 23711|     ;; nearby_enemies = i64 %1332
 23712|  %1333 = icmp samesign ult i64 %1035, %1332                                                                            ;L75
 23713|     ;; dominated = i1 %1333
 23714|  br i1 %1333, label %1336, label %1334                                                                                 ;L76
 23715| 
 23716| 1334: ; preds = %1331
 23717|  %1335 = invoke zeroext i1 @ai::utils9can1v1win(ptr %3, ptr %4, ptr %5, ptr %68, ptr %420)
 23718|  to label %1337 unwind label %57                                                                                       ;L76
 23719| 
 23720| 1336: ; preds = %1337, %1331
 23723|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %34, ptr %5, ptr %4, i64 5, i1 zeroext false)
 23724|  to label %1339 unwind label %57                                                                                       ;L84
 23725| 
 23726| 1337: ; preds = %1334
 23727|     ;; can_fight = i1 %1335
 23728|  br i1 %1335, label %1338, label %1336                                                                                 ;L78
 23729| 
 23730| 1338: ; preds = %1337
 23732|  invoke void @ai::fight_check13battle_action(ptr sret([32 x i8]) %37, i64 %2, ptr %3, ptr %4, ptr %5, i64 5)
 23733|  to label %1360 unwind label %57                                                                                       ;L80
 23734| 
 23735| 1339: ; preds = %1336
 23736|  call void @llvm.memcpy.p0.p0.i64(ptr %35, ptr %34, i64 136, i1 false)                                                 ;L84
 23737|  %1340 = gep %35, i64 177                                                                                              ;L84
 23738|  store i8 3, ptr %1340,                                                                                                ;L84
 23741|     ;; self = ptr %46
 23742|     ;; self = ptr %46
 23743|     ;; value = ptr %35
 23744|     ;; src = ptr %35
 23745|     ;; additional = i64 1
 23746|     ;; needed_extra_cap = i64 1
 23747|     ;; needed_extra_cap = i64 1
 23748|     ;; strategy = i8 1
 23749|     ;; self = ptr %46
 23750|  %1341 = load i64, ptr %51, , !!32483, !!8                                                                             ;L149<1428<84
 23751|  %1342 = icmp eq i64 %213, %1341                                                                                       ;L1428<84
 23752|  br i1 %1342, label %1343, label %1349                                                                                 ;L1428<84
 23753| 
 23754| 1343: ; preds = %1339
 23755|     ;; self = ptr %46
 23756|     ;; self = ptr %46
 23757|     ;; self = ptr %46
 23758|     ;; used_cap = i64 %213
 23759|     ;; used_cap = i64 %213
 23760|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %46, i64 %213, i64 1, i1 zeroext true)
 23761|  to label %1344 unwind label %1347, !!32483                                                                            ;L619<430<738<1429<84
 23762| 
 23763| 1344: ; preds = %1343
 23764|  %1345 = load i64, ptr %52, , !!32483                                                                                  ;L1432<84
 23765|  %1346 = load ptr, ptr %46, , !!32483                                                                                  ;L138<1432<84
 23766|  br label %1349                                                                                                        ;L619<430<738<1429<84
 23767| 
 23768| 1347: ; preds = %1343
 23769|  %1348 = cleanuppad within none []
 23770|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %35) #30 [ "funclet"(token %1348) ], !!32466 ;L1436<84
 23771|  cleanupret from %1348 unwind label %57
 23772| 
 23773| 1349: ; preds = %1344, %1339
 23774|  %1350 = phi ptr [ %1346, %1344 ], [ %214, %1339 ]                                                                     ;L138<1432<84
 23775|  %1351 = phi i64 [ %1345, %1344 ], [ %213, %1339 ]                                                                     ;L1432<84
 23776|     ;; self = ptr %46
 23777|     ;; self = ptr %1350
 23778|     ;; count = i64 %1351
 23779|  %1352 = gepS %1350, i64 %1351                                                                                         ;L961<1432<84
 23780|     ;; end = ptr %1352
 23781|     ;; dst = ptr %1352
 23782|  call void @llvm.memcpy.p0.p0.i64(ptr %1352, ptr %35, i64 184, i1 false), !!32466                                      ;L1933<1433<84
 23783|  %1353 = add i64 %1351, 1                                                                                              ;L1434<84
 23784|  store i64 %1353, ptr %52, , !!32483                                                                                   ;L1434<84
 23786|  br i1 %1333, label %1424, label %1354                                                                                 ;L86
 23787| 
 23788| 1354: ; preds = %1349
 23790|  invoke void @ai::fight_check13battle_action(ptr sret([32 x i8]) %33, i64 %2, ptr %3, ptr %4, ptr %5, i64 5)
 23791|  to label %1355 unwind label %57                                                                                       ;L87
 23792| 
 23793| 1355: ; preds = %1354
 23794|  %1356 = load ptr, ptr %33, , !!8, !!8                                                                                 ;L87
 23795|  %1357 = gep %33, i64 24                                                                                               ;L87
 23796|  %1358 = load i64, ptr %1357, , !!8                                                                                    ;L87
 23797|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %46, ptr %1356, i64 %1358)
 23798|  to label %1359 unwind label %57                                                                                       ;L87
 23799| 
 23800| 1359: ; preds = %1355
 23802|  br label %1424                                                                                                        ;L86
 23803| 
 23804| 1360: ; preds = %1338
 23805|  %1361 = load ptr, ptr %37, , !!8, !!8                                                                                 ;L80
 23806|  %1362 = gep %37, i64 24                                                                                               ;L80
 23807|  %1363 = load i64, ptr %1362, , !!8                                                                                    ;L80
 23808|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %46, ptr %1361, i64 %1363)
 23809|  to label %1364 unwind label %57                                                                                       ;L80
 23810| 
 23811| 1364: ; preds = %1360
 23814|  %1365 = gep %420, i64 1472                                                                                            ;L81
 23815|  %1366 = load i64, ptr %1365, , !!8                                                                                    ;L81
 23816|     ;; data = ptr %5
 23817|     ;; target = i64 %1366
 23818|     ;; end_delay = i64 5
 23820|     ;; default = i64 0
 23822|     ;; default = i64 0
 23823|  %1367 = load ptr, ptr %64, , !!32512, !!8, !!8                                                                        ;L43<81
 23824|  %1368 = load ptr, ptr %72, , !!32512, !!8, !!8                                                                        ;L43<81
 23825|  %1369 = gep %1368, i64 496                                                                                            ;L43<81
 23826|  %1370 = load ptr, ptr %1369, , !!32512, !!8                                                                           ;L43<81
 23827|  %1371 = invoke ptr %1370(ptr %1367, i64 %1366)
 23828|  to label %1372 unwind label %57                                                                                       ;L43<81
 23829| 
 23830| 1372: ; preds = %1364
 23831|     ;; target_entity = ptr %1371
 23832|     ;; self = ptr %1371
 23833|  %1373 = gep %1368, i64 40                                                                                             ;L45<81
 23834|  %1374 = load ptr, ptr %1373, , !!32512, !!8                                                                           ;L45<81
 23835|  %1375 = invoke i64 %1374(ptr %1367)
 23836|  to label %1376 unwind label %57                                                                                       ;L45<81
 23837| 
 23838| 1376: ; preds = %1372
 23839|  %1377 = icmp eq ptr %1371, null                                                                                       ;L1161<47<81
 23840|  br i1 %1377, label %1383, label %1378                                                                                 ;L1161<47<81
 23841| 
 23842| 1378: ; preds = %1376
 23843|     ;; x = ptr %1371
 23844|     ;; t = ptr %1371
 23845|  %1379 = gep %1371, i64 1632                                                                                           ;L47<1162<47<81
 23846|  %1380 = load i64, ptr %1379, , !!32512, !!8                                                                           ;L47<1162<47<81
 23847|     ;; self[8..+8] = i64 %1380
 23848|     ;; self[0..+8] = i64 1
 23849|     ;; self = ptr %1371
 23850|     ;; x = ptr %1371
 23851|     ;; t = ptr %1371
 23852|  %1381 = gep %1371, i64 1640                                                                                           ;L48<1162<48<81
 23853|  %1382 = load i64, ptr %1381, , !!32512, !!8                                                                           ;L48<1162<48<81
 23854|     ;; self[8..+8] = i64 %1382
 23855|     ;; self[0..+8] = i64 1
 23856|  br label %1383                                                                                                        ;L1043<48<81
 23857| 
 23858| 1383: ; preds = %1378, %1376
 23859|  %1384 = phi i64 [ %1382, %1378 ], [ 0, %1376 ]                                                                        ;L0<48<81
 23860|  %1385 = phi i64 [ %1380, %1378 ], [ 0, %1376 ]                                                                        ;L0<47<81
 23861|  store i64 0, ptr %36,                                                                                                 ;L81
 23862|  %1386 = gep %36, i64 85                                                                                               ;L81
 23863|  store i8 2, ptr %1386,                                                                                                ;L81
 23864|  %1387 = gep %36, i64 88                                                                                               ;L81
 23865|  store i64 %1375, ptr %1387,                                                                                           ;L81
 23866|  %1388 = gep %36, i64 96                                                                                               ;L81
 23867|  store i64 %1366, ptr %1388,                                                                                           ;L81
 23868|  %1389 = gep %36, i64 104                                                                                              ;L81
 23869|  store i64 %1385, ptr %1389,                                                                                           ;L81
 23870|  %1390 = gep %36, i64 112                                                                                              ;L81
 23871|  store i64 %1384, ptr %1390,                                                                                           ;L81
 23872|  %1391 = gep %36, i64 120                                                                                              ;L81
 23873|  store i64 15000, ptr %1391,                                                                                           ;L81
 23874|  %1392 = gep %36, i64 128                                                                                              ;L81
 23875|  store i64 5, ptr %1392,                                                                                               ;L81
 23876|  %1393 = gep %36, i64 136                                                                                              ;L81
 23877|  %1394 = gep %36, i64 149                                                                                              ;L81
 23878|  call void @llvm.memset.p0.i64(ptr %1393, i8 0, i64 13, i1 false)                                                      ;L81
 23879|  store i8 2, ptr %1394,                                                                                                ;L81
 23880|  %1395 = gep %36, i64 177                                                                                              ;L81
 23881|  store i8 14, ptr %1395,                                                                                               ;L81
 23883|     ;; self = ptr %46
 23884|     ;; self = ptr %46
 23885|     ;; value = ptr %36
 23886|     ;; src = ptr %36
 23887|     ;; additional = i64 1
 23888|     ;; needed_extra_cap = i64 1
 23889|     ;; needed_extra_cap = i64 1
 23890|     ;; strategy = i8 1
 23891|  %1396 = load i64, ptr %52, , !!32545, !!8                                                                             ;L1428<81
 23892|     ;; self = ptr %46
 23893|  %1397 = load i64, ptr %51, , !!32545, !!8                                                                             ;L149<1428<81
 23894|  %1398 = icmp eq i64 %1396, %1397                                                                                      ;L1428<81
 23895|  br i1 %1398, label %1399, label %1404                                                                                 ;L1428<81
 23896| 
 23897| 1399: ; preds = %1383
 23898|     ;; self = ptr %46
 23899|     ;; self = ptr %46
 23900|     ;; self = ptr %46
 23901|     ;; used_cap = i64 %1396
 23902|     ;; used_cap = i64 %1396
 23903|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %46, i64 %1396, i64 1, i1 zeroext true)
 23904|  to label %1400 unwind label %1402, !!32545                                                                            ;L619<430<738<1429<81
 23905| 
 23906| 1400: ; preds = %1399
 23907|  %1401 = load i64, ptr %52, , !!32545                                                                                  ;L1432<81
 23908|  br label %1404                                                                                                        ;L619<430<738<1429<81
 23909| 
 23910| 1402: ; preds = %1399
 23911|  %1403 = cleanuppad within none []
 23912|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %36) #30 [ "funclet"(token %1403) ], !!32530 ;L1436<81
 23913|  cleanupret from %1403 unwind label %57
 23914| 
 23915| 1404: ; preds = %1400, %1383
 23916|  %1405 = phi i64 [ %1401, %1400 ], [ %1396, %1383 ]                                                                    ;L1432<81
 23917|     ;; self = ptr %46
 23918|  %1406 = load ptr, ptr %46, , !!32545, !!8, !!8                                                                        ;L138<1432<81
 23919|     ;; self = ptr %1406
 23920|     ;; count = i64 %1405
 23921|  %1407 = gepS %1406, i64 %1405                                                                                         ;L961<1432<81
 23922|     ;; end = ptr %1407
 23923|     ;; dst = ptr %1407
 23924|  call void @llvm.memcpy.p0.p0.i64(ptr %1407, ptr %36, i64 184, i1 false), !!32530                                      ;L1933<1433<81
 23925|  %1408 = add i64 %1405, 1                                                                                              ;L1434<81
 23926|  store i64 %1408, ptr %52, , !!32545                                                                                   ;L1434<81
 23928|  br label %1424                                                                                                        ;L78
 23929| 
 23930| 1409: ; preds = %1042
 23931|  call void @llvm.memcpy.p0.p0.i64(ptr %32, ptr %31, i64 136, i1 false)                                                 ;L91
 23932|  %1410 = gep %32, i64 177                                                                                              ;L91
 23933|  store i8 3, ptr %1410,                                                                                                ;L91
 23936|     ;; self = ptr %46
 23937|     ;; self = ptr %46
 23938|     ;; value = ptr %32
 23939|     ;; src = ptr %32
 23940|     ;; additional = i64 1
 23941|     ;; needed_extra_cap = i64 1
 23942|     ;; needed_extra_cap = i64 1
 23943|     ;; strategy = i8 1
 23944|     ;; self = ptr %46
 23945|  %1411 = load i64, ptr %51, , !!32581, !!8                                                                             ;L149<1428<91
 23946|  %1412 = icmp eq i64 %213, %1411                                                                                       ;L1428<91
 23947|  br i1 %1412, label %1413, label %1419                                                                                 ;L1428<91
 23948| 
 23949| 1413: ; preds = %1409
 23950|     ;; self = ptr %46
 23951|     ;; self = ptr %46
 23952|     ;; self = ptr %46
 23953|     ;; used_cap = i64 %213
 23954|     ;; used_cap = i64 %213
 23955|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %46, i64 %213, i64 1, i1 zeroext true)
 23956|  to label %1414 unwind label %1417, !!32581                                                                            ;L619<430<738<1429<91
 23957| 
 23958| 1414: ; preds = %1413
 23959|  %1415 = load i64, ptr %52, , !!32581                                                                                  ;L1432<91
 23960|  %1416 = load ptr, ptr %46, , !!32581                                                                                  ;L138<1432<91
 23961|  br label %1419                                                                                                        ;L619<430<738<1429<91
 23962| 
 23963| 1417: ; preds = %1413
 23964|  %1418 = cleanuppad within none []
 23965|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %32) #30 [ "funclet"(token %1418) ], !!32564 ;L1436<91
 23966|  cleanupret from %1418 unwind label %57
 23967| 
 23968| 1419: ; preds = %1414, %1409
 23969|  %1420 = phi ptr [ %1416, %1414 ], [ %214, %1409 ]                                                                     ;L138<1432<91
 23970|  %1421 = phi i64 [ %1415, %1414 ], [ %213, %1409 ]                                                                     ;L1432<91
 23971|     ;; self = ptr %46
 23972|     ;; self = ptr %1420
 23973|     ;; count = i64 %1421
 23974|  %1422 = gepS %1420, i64 %1421                                                                                         ;L961<1432<91
 23975|     ;; end = ptr %1422
 23976|     ;; dst = ptr %1422
 23977|  call void @llvm.memcpy.p0.p0.i64(ptr %1422, ptr %32, i64 184, i1 false), !!32564                                      ;L1933<1433<91
 23978|  %1423 = add i64 %1421, 1                                                                                              ;L1434<91
 23979|  store i64 %1423, ptr %52, , !!32581                                                                                   ;L1434<91
 23981|  br label %1424                                                                                                        ;L67
 23982| 
 23983| 1424: ; preds = %1419, %1404, %1359, %1349
 23986|  %1425 = load i8, ptr %215, , !!8                                                                                      ;L107
 23987|  %1426 = trunc nuw i8 %1425 to i1                                                                                      ;L107
 23988|  br i1 %1426, label %243, label %244                                                                                   ;L107
 23989| 
 23990| 1427: ; preds = %530
 23992|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %46, ptr %531, i64 %534)
 23993|  to label %1428 unwind label %57                                                                                       ;L101
 23994| 
 23995| 1428: ; preds = %1427
 23998|  br label %243                                                                                                         ;L107
 23999| 
 24000| 1429: ; preds = %328
 24001|  %1430 = extractvalue { i64, ptr } %351, 1                                                                             ;L2707<3416<3387<110
 24003|     ;; nearest_enemy = ptr %1430
 24004|  %1431 = icmp eq ptr %1430, null                                                                                       ;L112
 24005|  br i1 %1431, label %1587, label %1432                                                                                 ;L112
 24006| 
 24007| 1432: ; preds = %1429
 24008|     ;; enemy = ptr %1430
 24009|     ;; self[0..+8] = ptr %66
 24010|     ;; slice[0..+8] = ptr %66
 24011|     ;; self[8..+8] = i64 5
 24012|     ;; slice[8..+8] = i64 5
 24013|     ;; self = ptr %66
 24014|     ;; self[0..+8] = ptr %66
 24015|     ;; self[8..+8] = ptr %66
 24016|     ;; self[16..+8] = ptr %68
 24017|     ;; init = i64 0
 24020|     ;; self[0..+8] = ptr %66
 24021|     ;; iter[0..+8] = ptr %66
 24022|     ;; self[0..+8] = ptr %66
 24023|     ;; self[8..+8] = ptr %66
 24024|     ;; iter[8..+8] = ptr %66
 24025|     ;; self[8..+8] = ptr %66
 24026|     ;; self[16..+8] = ptr %68
 24027|     ;; iter[16..+8] = ptr %68
 24028|     ;; self[16..+8] = ptr %68
 24030|     ;; self[0..+8] = ptr %66
 24031|     ;; self[8..+8] = ptr %66
 24032|     ;; init = i64 0
 24033|     ;; fold = ptr %68
 24035|     ;; f = ptr %68
 24036|     ;; self[0..+8] = ptr %66
 24037|     ;; self[8..+8] = ptr %66
 24038|     ;; init = i64 0
 24039|     ;; acc = i64 0
 24040|     ;; i = i64 0
 24041|     ;; len = i64 5
 24042|  %1433 = load i64, ptr %75, , !!32718
 24043|  %1434 = load i64, ptr %336, , !!32718
 24044|  %1435 = load i64, ptr %338, , !!32718
 24045|     ;; self = ptr %66
 24046|     ;; count = i64 0
 24047|  %1436 = load ptr, ptr %66, , !!32728, !!8                                                                             ;L279<146<128<52<3674<142<115
 24049|     ;; acc = i64 0
 24051|  %1437 = icmp eq ptr %1436, null                                                                                       ;L39<279<146<128<52<3674<142<115
 24052|  br i1 %1437, label %1460, label %1438                                                                                 ;L39<279<146<128<52<3674<142<115
 24053| 
 24054| 1438: ; preds = %1432
 24055|     ;; x = ptr %1436
 24057|     ;; acc = i64 0
 24058|     ;; elt = ptr %1436
 24059|     ;; x = ptr %1436
 24063|  %1439 = gep %1436, i64 1472                                                                                           ;L114<138<88<40<279<146<128<52<3674<142<115
 24064|  %1440 = load i64, ptr %1439, , !!32728, !!8                                                                           ;L114<138<88<40<279<146<128<52<3674<142<115
 24065|  %1441 = icmp eq i64 %1440, %1433                                                                                      ;L114<138<88<40<279<146<128<52<3674<142<115
 24066|  br i1 %1441, label %1460, label %1442                                                                                 ;L114<138<88<40<279<146<128<52<3674<142<115
 24067| 
 24068| 1442: ; preds = %1438
 24069|     ;; self = ptr %1436
 24070|     ;; other = ptr %68
 24071|  %1443 = gep %1436, i64 1632                                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24072|  %1444 = load i64, ptr %1443, , !!32728, !!8                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24073|     ;; x1 = i64 %1444
 24074|     ;; self = i64 %1444
 24075|  %1445 = gep %1436, i64 1640                                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24076|  %1446 = load i64, ptr %1445, , !!32728, !!8                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24077|     ;; y1 = i64 %1446
 24078|     ;; self = i64 %1446
 24079|     ;; x2 = i64 %1434
 24080|     ;; other = i64 %1434
 24081|     ;; y2 = i64 %1435
 24082|     ;; other = i64 %1435
 24083|  %1447 = icmp ult i64 %1444, %1434                                                                                     ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24084|  %1448 = sub nuw i64 %1434, %1444                                                                                      ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24085|  %1449 = sub nuw i64 %1444, %1434                                                                                      ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24086|  %1450 = select i1 %1447, i64 %1448, i64 %1449                                                                         ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24087|     ;; dx = i64 %1450
 24088|  %1451 = icmp ult i64 %1446, %1435                                                                                     ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24089|  %1452 = sub nuw i64 %1435, %1446                                                                                      ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24090|  %1453 = sub nuw i64 %1446, %1435                                                                                      ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24091|  %1454 = select i1 %1451, i64 %1452, i64 %1453                                                                         ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24092|     ;; dy = i64 %1454
 24093|  %1455 = mul i64 %1450, %1450                                                                                          ;L9<2158<114<138<88<40<279<146<128<52<3674<142<115
 24094|  %1456 = mul i64 %1454, %1454                                                                                          ;L9<2158<114<138<88<40<279<146<128<52<3674<142<115
 24095|  %1457 = add i64 %1456, %1455                                                                                          ;L9<2158<114<138<88<40<279<146<128<52<3674<142<115
 24096|  %1458 = icmp ult i64 %1457, 22500000001                                                                               ;L114<138<88<40<279<146<128<52<3674<142<115
 24097|  %1459 = zext i1 %1458 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<115
 24098|  br label %1460                                                                                                        ;L114<138<88<40<279<146<128<52<3674<142<115
 24099| 
 24100| 1460: ; preds = %1442, %1438, %1432
 24101|  %1461 = phi i64 [ 0, %1432 ], [ %1459, %1442 ], [ 0, %1438 ]                                                          ;L0<279<146<128<52<3674<142<115
 24102|     ;; acc = i64 %1461
 24103|     ;; i = i64 1
 24104|     ;; self = ptr %66
 24105|     ;; count = i64 1
 24106|  %1462 = gep %66, i64 8                                                                                                ;L656<279<146<128<52<3674<142<115
 24107|  %1463 = load ptr, ptr %1462, , !!32728, !!8                                                                           ;L279<146<128<52<3674<142<115
 24109|     ;; acc = i64 %1461
 24111|  %1464 = icmp eq ptr %1463, null                                                                                       ;L39<279<146<128<52<3674<142<115
 24112|  br i1 %1464, label %1490, label %1465                                                                                 ;L39<279<146<128<52<3674<142<115
 24113| 
 24114| 1465: ; preds = %1460
 24115|     ;; x = ptr %1463
 24117|     ;; acc = i64 %1461
 24118|     ;; elt = ptr %1463
 24119|     ;; x = ptr %1463
 24123|  %1466 = gep %1463, i64 1472                                                                                           ;L114<138<88<40<279<146<128<52<3674<142<115
 24124|  %1467 = load i64, ptr %1466, , !!32728, !!8                                                                           ;L114<138<88<40<279<146<128<52<3674<142<115
 24125|  %1468 = icmp eq i64 %1467, %1433                                                                                      ;L114<138<88<40<279<146<128<52<3674<142<115
 24126|  br i1 %1468, label %1487, label %1469                                                                                 ;L114<138<88<40<279<146<128<52<3674<142<115
 24127| 
 24128| 1469: ; preds = %1465
 24129|     ;; self = ptr %1463
 24130|     ;; other = ptr %68
 24131|  %1470 = gep %1463, i64 1632                                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24132|  %1471 = load i64, ptr %1470, , !!32728, !!8                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24133|     ;; x1 = i64 %1471
 24134|     ;; self = i64 %1471
 24135|  %1472 = gep %1463, i64 1640                                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24136|  %1473 = load i64, ptr %1472, , !!32728, !!8                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24137|     ;; y1 = i64 %1473
 24138|     ;; self = i64 %1473
 24139|     ;; x2 = i64 %1434
 24140|     ;; other = i64 %1434
 24141|     ;; y2 = i64 %1435
 24142|     ;; other = i64 %1435
 24143|  %1474 = icmp ult i64 %1471, %1434                                                                                     ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24144|  %1475 = sub nuw i64 %1434, %1471                                                                                      ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24145|  %1476 = sub nuw i64 %1471, %1434                                                                                      ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24146|  %1477 = select i1 %1474, i64 %1475, i64 %1476                                                                         ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24147|     ;; dx = i64 %1477
 24148|  %1478 = icmp ult i64 %1473, %1435                                                                                     ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24149|  %1479 = sub nuw i64 %1435, %1473                                                                                      ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24150|  %1480 = sub nuw i64 %1473, %1435                                                                                      ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24151|  %1481 = select i1 %1478, i64 %1479, i64 %1480                                                                         ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24152|     ;; dy = i64 %1481
 24153|  %1482 = mul i64 %1477, %1477                                                                                          ;L9<2158<114<138<88<40<279<146<128<52<3674<142<115
 24154|  %1483 = mul i64 %1481, %1481                                                                                          ;L9<2158<114<138<88<40<279<146<128<52<3674<142<115
 24155|  %1484 = add i64 %1483, %1482                                                                                          ;L9<2158<114<138<88<40<279<146<128<52<3674<142<115
 24156|  %1485 = icmp ult i64 %1484, 22500000001                                                                               ;L114<138<88<40<279<146<128<52<3674<142<115
 24157|  %1486 = zext i1 %1485 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<115
 24158|  br label %1487                                                                                                        ;L114<138<88<40<279<146<128<52<3674<142<115
 24159| 
 24160| 1487: ; preds = %1469, %1465
 24161|  %1488 = phi i64 [ %1486, %1469 ], [ 0, %1465 ]                                                                        ;L114<138<88<40<279<146<128<52<3674<142<115
 24163|     ;; a = i64 %1461
 24164|     ;; b = i64 %1488
 24165|  %1489 = add nuw nsw i64 %1488, %1461                                                                                  ;L55<88<40<279<146<128<52<3674<142<115
 24166|  br label %1490                                                                                                        ;L42<279<146<128<52<3674<142<115
 24167| 
 24168| 1490: ; preds = %1487, %1460
 24169|  %1491 = phi i64 [ %1489, %1487 ], [ %1461, %1460 ]                                                                    ;L0<279<146<128<52<3674<142<115
 24170|     ;; acc = i64 %1491
 24171|     ;; i = i64 2
 24172|     ;; self = ptr %66
 24173|     ;; count = i64 2
 24174|  %1492 = gep %66, i64 16                                                                                               ;L656<279<146<128<52<3674<142<115
 24175|  %1493 = load ptr, ptr %1492, , !!32728, !!8                                                                           ;L279<146<128<52<3674<142<115
 24177|     ;; acc = i64 %1491
 24179|  %1494 = icmp eq ptr %1493, null                                                                                       ;L39<279<146<128<52<3674<142<115
 24180|  br i1 %1494, label %1520, label %1495                                                                                 ;L39<279<146<128<52<3674<142<115
 24181| 
 24182| 1495: ; preds = %1490
 24183|     ;; x = ptr %1493
 24185|     ;; acc = i64 %1491
 24186|     ;; elt = ptr %1493
 24187|     ;; x = ptr %1493
 24191|  %1496 = gep %1493, i64 1472                                                                                           ;L114<138<88<40<279<146<128<52<3674<142<115
 24192|  %1497 = load i64, ptr %1496, , !!32728, !!8                                                                           ;L114<138<88<40<279<146<128<52<3674<142<115
 24193|  %1498 = icmp eq i64 %1497, %1433                                                                                      ;L114<138<88<40<279<146<128<52<3674<142<115
 24194|  br i1 %1498, label %1517, label %1499                                                                                 ;L114<138<88<40<279<146<128<52<3674<142<115
 24195| 
 24196| 1499: ; preds = %1495
 24197|     ;; self = ptr %1493
 24198|     ;; other = ptr %68
 24199|  %1500 = gep %1493, i64 1632                                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24200|  %1501 = load i64, ptr %1500, , !!32728, !!8                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24201|     ;; x1 = i64 %1501
 24202|     ;; self = i64 %1501
 24203|  %1502 = gep %1493, i64 1640                                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24204|  %1503 = load i64, ptr %1502, , !!32728, !!8                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24205|     ;; y1 = i64 %1503
 24206|     ;; self = i64 %1503
 24207|     ;; x2 = i64 %1434
 24208|     ;; other = i64 %1434
 24209|     ;; y2 = i64 %1435
 24210|     ;; other = i64 %1435
 24211|  %1504 = icmp ult i64 %1501, %1434                                                                                     ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24212|  %1505 = sub nuw i64 %1434, %1501                                                                                      ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24213|  %1506 = sub nuw i64 %1501, %1434                                                                                      ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24214|  %1507 = select i1 %1504, i64 %1505, i64 %1506                                                                         ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24215|     ;; dx = i64 %1507
 24216|  %1508 = icmp ult i64 %1503, %1435                                                                                     ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24217|  %1509 = sub nuw i64 %1435, %1503                                                                                      ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24218|  %1510 = sub nuw i64 %1503, %1435                                                                                      ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24219|  %1511 = select i1 %1508, i64 %1509, i64 %1510                                                                         ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24220|     ;; dy = i64 %1511
 24221|  %1512 = mul i64 %1507, %1507                                                                                          ;L9<2158<114<138<88<40<279<146<128<52<3674<142<115
 24222|  %1513 = mul i64 %1511, %1511                                                                                          ;L9<2158<114<138<88<40<279<146<128<52<3674<142<115
 24223|  %1514 = add i64 %1513, %1512                                                                                          ;L9<2158<114<138<88<40<279<146<128<52<3674<142<115
 24224|  %1515 = icmp ult i64 %1514, 22500000001                                                                               ;L114<138<88<40<279<146<128<52<3674<142<115
 24225|  %1516 = zext i1 %1515 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<115
 24226|  br label %1517                                                                                                        ;L114<138<88<40<279<146<128<52<3674<142<115
 24227| 
 24228| 1517: ; preds = %1499, %1495
 24229|  %1518 = phi i64 [ %1516, %1499 ], [ 0, %1495 ]                                                                        ;L114<138<88<40<279<146<128<52<3674<142<115
 24231|     ;; a = i64 %1491
 24232|     ;; b = i64 %1518
 24233|  %1519 = add nuw nsw i64 %1518, %1491                                                                                  ;L55<88<40<279<146<128<52<3674<142<115
 24234|  br label %1520                                                                                                        ;L42<279<146<128<52<3674<142<115
 24235| 
 24236| 1520: ; preds = %1517, %1490
 24237|  %1521 = phi i64 [ %1519, %1517 ], [ %1491, %1490 ]                                                                    ;L0<279<146<128<52<3674<142<115
 24238|     ;; acc = i64 %1521
 24239|     ;; i = i64 3
 24240|     ;; self = ptr %66
 24241|     ;; count = i64 3
 24242|  %1522 = gep %66, i64 24                                                                                               ;L656<279<146<128<52<3674<142<115
 24243|  %1523 = load ptr, ptr %1522, , !!32728, !!8                                                                           ;L279<146<128<52<3674<142<115
 24245|     ;; acc = i64 %1521
 24247|  %1524 = icmp eq ptr %1523, null                                                                                       ;L39<279<146<128<52<3674<142<115
 24248|  br i1 %1524, label %1550, label %1525                                                                                 ;L39<279<146<128<52<3674<142<115
 24249| 
 24250| 1525: ; preds = %1520
 24251|     ;; x = ptr %1523
 24253|     ;; acc = i64 %1521
 24254|     ;; elt = ptr %1523
 24255|     ;; x = ptr %1523
 24259|  %1526 = gep %1523, i64 1472                                                                                           ;L114<138<88<40<279<146<128<52<3674<142<115
 24260|  %1527 = load i64, ptr %1526, , !!32728, !!8                                                                           ;L114<138<88<40<279<146<128<52<3674<142<115
 24261|  %1528 = icmp eq i64 %1527, %1433                                                                                      ;L114<138<88<40<279<146<128<52<3674<142<115
 24262|  br i1 %1528, label %1547, label %1529                                                                                 ;L114<138<88<40<279<146<128<52<3674<142<115
 24263| 
 24264| 1529: ; preds = %1525
 24265|     ;; self = ptr %1523
 24266|     ;; other = ptr %68
 24267|  %1530 = gep %1523, i64 1632                                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24268|  %1531 = load i64, ptr %1530, , !!32728, !!8                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24269|     ;; x1 = i64 %1531
 24270|     ;; self = i64 %1531
 24271|  %1532 = gep %1523, i64 1640                                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24272|  %1533 = load i64, ptr %1532, , !!32728, !!8                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24273|     ;; y1 = i64 %1533
 24274|     ;; self = i64 %1533
 24275|     ;; x2 = i64 %1434
 24276|     ;; other = i64 %1434
 24277|     ;; y2 = i64 %1435
 24278|     ;; other = i64 %1435
 24279|  %1534 = icmp ult i64 %1531, %1434                                                                                     ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24280|  %1535 = sub nuw i64 %1434, %1531                                                                                      ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24281|  %1536 = sub nuw i64 %1531, %1434                                                                                      ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24282|  %1537 = select i1 %1534, i64 %1535, i64 %1536                                                                         ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24283|     ;; dx = i64 %1537
 24284|  %1538 = icmp ult i64 %1533, %1435                                                                                     ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24285|  %1539 = sub nuw i64 %1435, %1533                                                                                      ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24286|  %1540 = sub nuw i64 %1533, %1435                                                                                      ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24287|  %1541 = select i1 %1538, i64 %1539, i64 %1540                                                                         ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24288|     ;; dy = i64 %1541
 24289|  %1542 = mul i64 %1537, %1537                                                                                          ;L9<2158<114<138<88<40<279<146<128<52<3674<142<115
 24290|  %1543 = mul i64 %1541, %1541                                                                                          ;L9<2158<114<138<88<40<279<146<128<52<3674<142<115
 24291|  %1544 = add i64 %1543, %1542                                                                                          ;L9<2158<114<138<88<40<279<146<128<52<3674<142<115
 24292|  %1545 = icmp ult i64 %1544, 22500000001                                                                               ;L114<138<88<40<279<146<128<52<3674<142<115
 24293|  %1546 = zext i1 %1545 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<115
 24294|  br label %1547                                                                                                        ;L114<138<88<40<279<146<128<52<3674<142<115
 24295| 
 24296| 1547: ; preds = %1529, %1525
 24297|  %1548 = phi i64 [ %1546, %1529 ], [ 0, %1525 ]                                                                        ;L114<138<88<40<279<146<128<52<3674<142<115
 24299|     ;; a = i64 %1521
 24300|     ;; b = i64 %1548
 24301|  %1549 = add nuw nsw i64 %1548, %1521                                                                                  ;L55<88<40<279<146<128<52<3674<142<115
 24302|  br label %1550                                                                                                        ;L42<279<146<128<52<3674<142<115
 24303| 
 24304| 1550: ; preds = %1547, %1520
 24305|  %1551 = phi i64 [ %1549, %1547 ], [ %1521, %1520 ]                                                                    ;L0<279<146<128<52<3674<142<115
 24306|     ;; acc = i64 %1551
 24307|     ;; i = i64 4
 24308|     ;; self = ptr %66
 24309|     ;; count = i64 4
 24310|  %1552 = gep %66, i64 32                                                                                               ;L656<279<146<128<52<3674<142<115
 24311|  %1553 = load ptr, ptr %1552, , !!32728, !!8                                                                           ;L279<146<128<52<3674<142<115
 24313|     ;; acc = i64 %1551
 24315|  %1554 = icmp eq ptr %1553, null                                                                                       ;L39<279<146<128<52<3674<142<115
 24316|  br i1 %1554, label %1580, label %1555                                                                                 ;L39<279<146<128<52<3674<142<115
 24317| 
 24318| 1555: ; preds = %1550
 24319|     ;; x = ptr %1553
 24321|     ;; acc = i64 %1551
 24322|     ;; elt = ptr %1553
 24323|     ;; x = ptr %1553
 24327|  %1556 = gep %1553, i64 1472                                                                                           ;L114<138<88<40<279<146<128<52<3674<142<115
 24328|  %1557 = load i64, ptr %1556, , !!32728, !!8                                                                           ;L114<138<88<40<279<146<128<52<3674<142<115
 24329|  %1558 = icmp eq i64 %1557, %1433                                                                                      ;L114<138<88<40<279<146<128<52<3674<142<115
 24330|  br i1 %1558, label %1577, label %1559                                                                                 ;L114<138<88<40<279<146<128<52<3674<142<115
 24331| 
 24332| 1559: ; preds = %1555
 24333|     ;; self = ptr %1553
 24334|     ;; other = ptr %68
 24335|  %1560 = gep %1553, i64 1632                                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24336|  %1561 = load i64, ptr %1560, , !!32728, !!8                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24337|     ;; x1 = i64 %1561
 24338|     ;; self = i64 %1561
 24339|  %1562 = gep %1553, i64 1640                                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24340|  %1563 = load i64, ptr %1562, , !!32728, !!8                                                                           ;L2158<114<138<88<40<279<146<128<52<3674<142<115
 24341|     ;; y1 = i64 %1563
 24342|     ;; self = i64 %1563
 24343|     ;; x2 = i64 %1434
 24344|     ;; other = i64 %1434
 24345|     ;; y2 = i64 %1435
 24346|     ;; other = i64 %1435
 24347|  %1564 = icmp ult i64 %1561, %1434                                                                                     ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24348|  %1565 = sub nuw i64 %1434, %1561                                                                                      ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24349|  %1566 = sub nuw i64 %1561, %1434                                                                                      ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24350|  %1567 = select i1 %1564, i64 %1565, i64 %1566                                                                         ;L3147<7<2158<114<138<88<40<279<146<128<52<3674<142<115
 24351|     ;; dx = i64 %1567
 24352|  %1568 = icmp ult i64 %1563, %1435                                                                                     ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24353|  %1569 = sub nuw i64 %1435, %1563                                                                                      ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24354|  %1570 = sub nuw i64 %1563, %1435                                                                                      ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24355|  %1571 = select i1 %1568, i64 %1569, i64 %1570                                                                         ;L3147<8<2158<114<138<88<40<279<146<128<52<3674<142<115
 24356|     ;; dy = i64 %1571
 24357|  %1572 = mul i64 %1567, %1567                                                                                          ;L9<2158<114<138<88<40<279<146<128<52<3674<142<115
 24358|  %1573 = mul i64 %1571, %1571                                                                                          ;L9<2158<114<138<88<40<279<146<128<52<3674<142<115
 24359|  %1574 = add i64 %1573, %1572                                                                                          ;L9<2158<114<138<88<40<279<146<128<52<3674<142<115
 24360|  %1575 = icmp ult i64 %1574, 22500000001                                                                               ;L114<138<88<40<279<146<128<52<3674<142<115
 24361|  %1576 = zext i1 %1575 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<115
 24362|  br label %1577                                                                                                        ;L114<138<88<40<279<146<128<52<3674<142<115
 24363| 
 24364| 1577: ; preds = %1559, %1555
 24365|  %1578 = phi i64 [ %1576, %1559 ], [ 0, %1555 ]                                                                        ;L114<138<88<40<279<146<128<52<3674<142<115
 24367|     ;; a = i64 %1551
 24368|     ;; b = i64 %1578
 24369|  %1579 = add nuw nsw i64 %1578, %1551                                                                                  ;L55<88<40<279<146<128<52<3674<142<115
 24370|  br label %1580                                                                                                        ;L42<279<146<128<52<3674<142<115
 24371| 
 24372| 1580: ; preds = %1577, %1550
 24373|  %1581 = phi i64 [ %1579, %1577 ], [ %1551, %1550 ]                                                                    ;L0<279<146<128<52<3674<142<115
 24374|     ;; acc = i64 %1581
 24375|     ;; i = i64 5
 24376|     ;; nearby_allies = i64 %1581
 24377|     ;; self[0..+8] = ptr %245
 24378|     ;; slice[0..+8] = ptr %245
 24379|     ;; self[8..+8] = i64 5
 24380|     ;; slice[8..+8] = i64 5
 24381|     ;; self = ptr %245
 24382|     ;; self[0..+8] = ptr %245
 24383|     ;; self[8..+8] = ptr %246
 24384|     ;; self[16..+8] = ptr %68
 24385|     ;; init = i64 0
 24388|     ;; self[0..+8] = ptr %245
 24389|     ;; iter[0..+8] = ptr %245
 24390|     ;; self[0..+8] = ptr %245
 24391|     ;; self[8..+8] = ptr %246
 24392|     ;; iter[8..+8] = ptr %246
 24393|     ;; self[8..+8] = ptr %246
 24394|     ;; self[16..+8] = ptr %68
 24395|     ;; iter[16..+8] = ptr %68
 24396|     ;; self[16..+8] = ptr %68
 24398|     ;; self[0..+8] = ptr %245
 24399|     ;; self[8..+8] = ptr %246
 24400|     ;; init = i64 0
 24401|     ;; fold = ptr %68
 24403|     ;; f = ptr %68
 24404|     ;; self[0..+8] = ptr %245
 24405|     ;; self[8..+8] = ptr %246
 24406|     ;; init = i64 0
 24407|     ;; acc = i64 0
 24408|     ;; i = i64 0
 24409|     ;; len = i64 5
 24410|  %1582 = load i64, ptr %68, , !!32950
 24411|  %1583 = freeze i64 %1582
 24412|  %1584 = trunc i64 %1583 to i1
 24413|  %1585 = load i64, ptr %250, , !!32950
 24414|  %1586 = freeze i64 %1585
 24415|  br i1 %1584, label %1588, label %1705
 24416| 
 24417| 1587: ; preds = %1429, %327
 24420|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %21, ptr %5, ptr %4, i64 5, i1 zeroext false)
 24421|  to label %1910 unwind label %57                                                                                       ;L132
 24422| 
 24423| 1588: ; preds = %1580
 24424|     ;; acc = i64 0
 24425|     ;; i = i64 0
 24426|     ;; self = ptr %245
 24427|     ;; count = i64 0
 24428|  %1589 = load ptr, ptr %245, , !!32961, !!8                                                                            ;L279<146<128<52<3674<142<118
 24430|     ;; acc = i64 0
 24432|  %1590 = icmp eq ptr %1589, null                                                                                       ;L39<279<146<128<52<3674<142<118
 24433|  br i1 %1590, label %1609, label %1591                                                                                 ;L39<279<146<128<52<3674<142<118
 24434| 
 24435| 1591: ; preds = %1588
 24436|     ;; x = ptr %1589
 24438|     ;; acc = i64 0
 24439|     ;; elt = ptr %1589
 24440|     ;; x = ptr %1589
 24444|     ;; self = ptr %1589
 24445|     ;; entity = ptr %68
 24446|     ;; self = ptr %1589
 24447|     ;; other = ptr %68
 24448|  %1592 = gep %1589, i64 1632                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24449|  %1593 = load i64, ptr %1592, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24450|     ;; x1 = i64 %1593
 24451|     ;; self = i64 %1593
 24452|  %1594 = gep %1589, i64 1640                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24453|  %1595 = load i64, ptr %1594, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24454|     ;; y1 = i64 %1595
 24455|     ;; self = i64 %1595
 24456|     ;; x2 = i64 %1434
 24457|     ;; other = i64 %1434
 24458|     ;; y2 = i64 %1435
 24459|     ;; other = i64 %1435
 24460|  %1596 = icmp ult i64 %1593, %1434                                                                                     ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24461|  %1597 = sub nuw i64 %1434, %1593                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24462|  %1598 = sub nuw i64 %1593, %1434                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24463|  %1599 = select i1 %1596, i64 %1597, i64 %1598                                                                         ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24464|     ;; dx = i64 %1599
 24465|  %1600 = icmp ult i64 %1595, %1435                                                                                     ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24466|  %1601 = sub nuw i64 %1435, %1595                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24467|  %1602 = sub nuw i64 %1595, %1435                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24468|  %1603 = select i1 %1600, i64 %1601, i64 %1602                                                                         ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24469|     ;; dy = i64 %1603
 24470|  %1604 = mul i64 %1599, %1599                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24471|  %1605 = mul i64 %1603, %1603                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24472|  %1606 = add i64 %1605, %1604                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24473|  %1607 = icmp ult i64 %1606, 22500000001                                                                               ;L117<138<88<40<279<146<128<52<3674<142<118
 24474|  %1608 = zext i1 %1607 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<118
 24476|     ;; a = i64 0
 24478|  br label %1609                                                                                                        ;L42<279<146<128<52<3674<142<118
 24479| 
 24480| 1609: ; preds = %1591, %1588
 24481|  %1610 = phi i64 [ %1608, %1591 ], [ 0, %1588 ]                                                                        ;L0<279<146<128<52<3674<142<118
 24482|     ;; acc = i64 %1610
 24483|     ;; i = i64 1
 24484|     ;; self = ptr %245
 24485|     ;; count = i64 1
 24486|  %1611 = gep %245, i64 8                                                                                               ;L656<279<146<128<52<3674<142<118
 24487|  %1612 = load ptr, ptr %1611, , !!32961, !!8                                                                           ;L279<146<128<52<3674<142<118
 24489|     ;; acc = i64 %1610
 24491|  %1613 = icmp eq ptr %1612, null                                                                                       ;L39<279<146<128<52<3674<142<118
 24492|  br i1 %1613, label %1633, label %1614                                                                                 ;L39<279<146<128<52<3674<142<118
 24493| 
 24494| 1614: ; preds = %1609
 24495|     ;; x = ptr %1612
 24497|     ;; acc = i64 %1610
 24498|     ;; elt = ptr %1612
 24499|     ;; x = ptr %1612
 24503|     ;; self = ptr %1612
 24504|     ;; entity = ptr %68
 24505|     ;; self = ptr %1612
 24506|     ;; other = ptr %68
 24507|  %1615 = gep %1612, i64 1632                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24508|  %1616 = load i64, ptr %1615, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24509|     ;; x1 = i64 %1616
 24510|     ;; self = i64 %1616
 24511|  %1617 = gep %1612, i64 1640                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24512|  %1618 = load i64, ptr %1617, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24513|     ;; y1 = i64 %1618
 24514|     ;; self = i64 %1618
 24515|     ;; x2 = i64 %1434
 24516|     ;; other = i64 %1434
 24517|     ;; y2 = i64 %1435
 24518|     ;; other = i64 %1435
 24519|  %1619 = icmp ult i64 %1616, %1434                                                                                     ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24520|  %1620 = sub nuw i64 %1434, %1616                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24521|  %1621 = sub nuw i64 %1616, %1434                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24522|  %1622 = select i1 %1619, i64 %1620, i64 %1621                                                                         ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24523|     ;; dx = i64 %1622
 24524|  %1623 = icmp ult i64 %1618, %1435                                                                                     ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24525|  %1624 = sub nuw i64 %1435, %1618                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24526|  %1625 = sub nuw i64 %1618, %1435                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24527|  %1626 = select i1 %1623, i64 %1624, i64 %1625                                                                         ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24528|     ;; dy = i64 %1626
 24529|  %1627 = mul i64 %1622, %1622                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24530|  %1628 = mul i64 %1626, %1626                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24531|  %1629 = add i64 %1628, %1627                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24532|  %1630 = icmp ult i64 %1629, 22500000001                                                                               ;L117<138<88<40<279<146<128<52<3674<142<118
 24533|  %1631 = zext i1 %1630 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<118
 24535|     ;; a = i64 %1610
 24537|  %1632 = add nuw nsw i64 %1610, %1631                                                                                  ;L55<88<40<279<146<128<52<3674<142<118
 24538|  br label %1633                                                                                                        ;L42<279<146<128<52<3674<142<118
 24539| 
 24540| 1633: ; preds = %1614, %1609
 24541|  %1634 = phi i64 [ %1632, %1614 ], [ %1610, %1609 ]                                                                    ;L0<279<146<128<52<3674<142<118
 24542|     ;; acc = i64 %1634
 24543|     ;; i = i64 2
 24544|     ;; self = ptr %245
 24545|     ;; count = i64 2
 24546|  %1635 = gep %245, i64 16                                                                                              ;L656<279<146<128<52<3674<142<118
 24547|  %1636 = load ptr, ptr %1635, , !!32961, !!8                                                                           ;L279<146<128<52<3674<142<118
 24549|     ;; acc = i64 %1634
 24551|  %1637 = icmp eq ptr %1636, null                                                                                       ;L39<279<146<128<52<3674<142<118
 24552|  br i1 %1637, label %1657, label %1638                                                                                 ;L39<279<146<128<52<3674<142<118
 24553| 
 24554| 1638: ; preds = %1633
 24555|     ;; x = ptr %1636
 24557|     ;; acc = i64 %1634
 24558|     ;; elt = ptr %1636
 24559|     ;; x = ptr %1636
 24563|     ;; self = ptr %1636
 24564|     ;; entity = ptr %68
 24565|     ;; self = ptr %1636
 24566|     ;; other = ptr %68
 24567|  %1639 = gep %1636, i64 1632                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24568|  %1640 = load i64, ptr %1639, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24569|     ;; x1 = i64 %1640
 24570|     ;; self = i64 %1640
 24571|  %1641 = gep %1636, i64 1640                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24572|  %1642 = load i64, ptr %1641, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24573|     ;; y1 = i64 %1642
 24574|     ;; self = i64 %1642
 24575|     ;; x2 = i64 %1434
 24576|     ;; other = i64 %1434
 24577|     ;; y2 = i64 %1435
 24578|     ;; other = i64 %1435
 24579|  %1643 = icmp ult i64 %1640, %1434                                                                                     ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24580|  %1644 = sub nuw i64 %1434, %1640                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24581|  %1645 = sub nuw i64 %1640, %1434                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24582|  %1646 = select i1 %1643, i64 %1644, i64 %1645                                                                         ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24583|     ;; dx = i64 %1646
 24584|  %1647 = icmp ult i64 %1642, %1435                                                                                     ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24585|  %1648 = sub nuw i64 %1435, %1642                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24586|  %1649 = sub nuw i64 %1642, %1435                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24587|  %1650 = select i1 %1647, i64 %1648, i64 %1649                                                                         ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24588|     ;; dy = i64 %1650
 24589|  %1651 = mul i64 %1646, %1646                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24590|  %1652 = mul i64 %1650, %1650                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24591|  %1653 = add i64 %1652, %1651                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24592|  %1654 = icmp ult i64 %1653, 22500000001                                                                               ;L117<138<88<40<279<146<128<52<3674<142<118
 24593|  %1655 = zext i1 %1654 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<118
 24595|     ;; a = i64 %1634
 24597|  %1656 = add nuw nsw i64 %1634, %1655                                                                                  ;L55<88<40<279<146<128<52<3674<142<118
 24598|  br label %1657                                                                                                        ;L42<279<146<128<52<3674<142<118
 24599| 
 24600| 1657: ; preds = %1638, %1633
 24601|  %1658 = phi i64 [ %1656, %1638 ], [ %1634, %1633 ]                                                                    ;L0<279<146<128<52<3674<142<118
 24602|     ;; acc = i64 %1658
 24603|     ;; i = i64 3
 24604|     ;; self = ptr %245
 24605|     ;; count = i64 3
 24606|  %1659 = gep %245, i64 24                                                                                              ;L656<279<146<128<52<3674<142<118
 24607|  %1660 = load ptr, ptr %1659, , !!32961, !!8                                                                           ;L279<146<128<52<3674<142<118
 24609|     ;; acc = i64 %1658
 24611|  %1661 = icmp eq ptr %1660, null                                                                                       ;L39<279<146<128<52<3674<142<118
 24612|  br i1 %1661, label %1681, label %1662                                                                                 ;L39<279<146<128<52<3674<142<118
 24613| 
 24614| 1662: ; preds = %1657
 24615|     ;; x = ptr %1660
 24617|     ;; acc = i64 %1658
 24618|     ;; elt = ptr %1660
 24619|     ;; x = ptr %1660
 24623|     ;; self = ptr %1660
 24624|     ;; entity = ptr %68
 24625|     ;; self = ptr %1660
 24626|     ;; other = ptr %68
 24627|  %1663 = gep %1660, i64 1632                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24628|  %1664 = load i64, ptr %1663, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24629|     ;; x1 = i64 %1664
 24630|     ;; self = i64 %1664
 24631|  %1665 = gep %1660, i64 1640                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24632|  %1666 = load i64, ptr %1665, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24633|     ;; y1 = i64 %1666
 24634|     ;; self = i64 %1666
 24635|     ;; x2 = i64 %1434
 24636|     ;; other = i64 %1434
 24637|     ;; y2 = i64 %1435
 24638|     ;; other = i64 %1435
 24639|  %1667 = icmp ult i64 %1664, %1434                                                                                     ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24640|  %1668 = sub nuw i64 %1434, %1664                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24641|  %1669 = sub nuw i64 %1664, %1434                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24642|  %1670 = select i1 %1667, i64 %1668, i64 %1669                                                                         ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24643|     ;; dx = i64 %1670
 24644|  %1671 = icmp ult i64 %1666, %1435                                                                                     ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24645|  %1672 = sub nuw i64 %1435, %1666                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24646|  %1673 = sub nuw i64 %1666, %1435                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24647|  %1674 = select i1 %1671, i64 %1672, i64 %1673                                                                         ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24648|     ;; dy = i64 %1674
 24649|  %1675 = mul i64 %1670, %1670                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24650|  %1676 = mul i64 %1674, %1674                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24651|  %1677 = add i64 %1676, %1675                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24652|  %1678 = icmp ult i64 %1677, 22500000001                                                                               ;L117<138<88<40<279<146<128<52<3674<142<118
 24653|  %1679 = zext i1 %1678 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<118
 24655|     ;; a = i64 %1658
 24657|  %1680 = add nuw nsw i64 %1658, %1679                                                                                  ;L55<88<40<279<146<128<52<3674<142<118
 24658|  br label %1681                                                                                                        ;L42<279<146<128<52<3674<142<118
 24659| 
 24660| 1681: ; preds = %1662, %1657
 24661|  %1682 = phi i64 [ %1680, %1662 ], [ %1658, %1657 ]                                                                    ;L0<279<146<128<52<3674<142<118
 24662|     ;; acc = i64 %1682
 24663|     ;; i = i64 4
 24664|     ;; self = ptr %245
 24665|     ;; count = i64 4
 24666|  %1683 = gep %245, i64 32                                                                                              ;L656<279<146<128<52<3674<142<118
 24667|  %1684 = load ptr, ptr %1683, , !!32961, !!8                                                                           ;L279<146<128<52<3674<142<118
 24669|     ;; acc = i64 %1682
 24671|  %1685 = icmp eq ptr %1684, null                                                                                       ;L39<279<146<128<52<3674<142<118
 24672|  br i1 %1685, label %1876, label %1686                                                                                 ;L39<279<146<128<52<3674<142<118
 24673| 
 24674| 1686: ; preds = %1681
 24675|     ;; x = ptr %1684
 24677|     ;; acc = i64 %1682
 24678|     ;; elt = ptr %1684
 24679|     ;; x = ptr %1684
 24683|     ;; self = ptr %1684
 24684|     ;; entity = ptr %68
 24685|     ;; self = ptr %1684
 24686|     ;; other = ptr %68
 24687|  %1687 = gep %1684, i64 1632                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24688|  %1688 = load i64, ptr %1687, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24689|     ;; x1 = i64 %1688
 24690|     ;; self = i64 %1688
 24691|  %1689 = gep %1684, i64 1640                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24692|  %1690 = load i64, ptr %1689, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24693|     ;; y1 = i64 %1690
 24694|     ;; self = i64 %1690
 24695|     ;; x2 = i64 %1434
 24696|     ;; other = i64 %1434
 24697|     ;; y2 = i64 %1435
 24698|     ;; other = i64 %1435
 24699|  %1691 = icmp ult i64 %1688, %1434                                                                                     ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24700|  %1692 = sub nuw i64 %1434, %1688                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24701|  %1693 = sub nuw i64 %1688, %1434                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24702|  %1694 = select i1 %1691, i64 %1692, i64 %1693                                                                         ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24703|     ;; dx = i64 %1694
 24704|  %1695 = icmp ult i64 %1690, %1435                                                                                     ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24705|  %1696 = sub nuw i64 %1435, %1690                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24706|  %1697 = sub nuw i64 %1690, %1435                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24707|  %1698 = select i1 %1695, i64 %1696, i64 %1697                                                                         ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24708|     ;; dy = i64 %1698
 24709|  %1699 = mul i64 %1694, %1694                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24710|  %1700 = mul i64 %1698, %1698                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24711|  %1701 = add i64 %1700, %1699                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24712|  %1702 = icmp ult i64 %1701, 22500000001                                                                               ;L117<138<88<40<279<146<128<52<3674<142<118
 24713|  %1703 = zext i1 %1702 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<118
 24715|     ;; a = i64 %1682
 24717|  %1704 = add nuw nsw i64 %1682, %1703                                                                                  ;L55<88<40<279<146<128<52<3674<142<118
 24718|  br label %1876                                                                                                        ;L42<279<146<128<52<3674<142<118
 24719| 
 24720| 1705: ; preds = %1580
 24721|  %1706 = icmp ult i64 %1586, 2
 24722|     ;; i = i64 0
 24723|     ;; i = i64 0
 24724|     ;; self = ptr %245
 24725|     ;; self = ptr %245
 24726|     ;; count = i64 0
 24727|     ;; count = i64 0
 24728|  %1707 = load ptr, ptr %245, , !!32961, !!8                                                                            ;L279<146<128<52<3674<142<118
 24733|  %1708 = icmp eq ptr %1707, null                                                                                       ;L39<279<146<128<52<3674<142<118
 24734|  br i1 %1706, label %1710, label %1709
 24735| 
 24736| 1709: ; preds = %1705
 24739|  br i1 %1708, label %1860, label %1858                                                                                 ;L39<279<146<128<52<3674<142<118
 24740| 
 24741| 1710: ; preds = %1705
 24742|     ;; acc = i64 0
 24743|     ;; acc = i64 0
 24744|  br i1 %1708, label %1734, label %1711                                                                                 ;L39<279<146<128<52<3674<142<118
 24745| 
 24746| 1711: ; preds = %1710
 24747|     ;; x = ptr %1707
 24749|     ;; acc = i64 0
 24750|     ;; elt = ptr %1707
 24751|     ;; x = ptr %1707
 24755|     ;; self = ptr %1707
 24756|     ;; entity = ptr %68
 24757|     ;; team = i64 %1585
 24759|  %1712 = gep %1707, i64 56                                                                                             ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 24760|  %1713 = gepS %1712, i64 %1586                                                                                         ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 24761|  %1714 = load i64, ptr %1713, , !!32961, !!8                                                                           ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 24762|  %1715 = icmp eq i64 %1714, 0                                                                                          ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 24763|  br i1 %1715, label %1716, label %1734                                                                                 ;L117<138<88<40<279<146<128<52<3674<142<118
 24764| 
 24765| 1716: ; preds = %1711
 24766|     ;; self = ptr %1707
 24767|     ;; other = ptr %68
 24768|  %1717 = gep %1707, i64 1632                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24769|  %1718 = load i64, ptr %1717, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24770|     ;; x1 = i64 %1718
 24771|     ;; self = i64 %1718
 24772|  %1719 = gep %1707, i64 1640                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24773|  %1720 = load i64, ptr %1719, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24774|     ;; y1 = i64 %1720
 24775|     ;; self = i64 %1720
 24776|     ;; x2 = i64 %1434
 24777|     ;; other = i64 %1434
 24778|     ;; y2 = i64 %1435
 24779|     ;; other = i64 %1435
 24780|  %1721 = icmp ult i64 %1718, %1434                                                                                     ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24781|  %1722 = sub nuw i64 %1434, %1718                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24782|  %1723 = sub nuw i64 %1718, %1434                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24783|  %1724 = select i1 %1721, i64 %1722, i64 %1723                                                                         ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24784|     ;; dx = i64 %1724
 24785|  %1725 = icmp ult i64 %1720, %1435                                                                                     ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24786|  %1726 = sub nuw i64 %1435, %1720                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24787|  %1727 = sub nuw i64 %1720, %1435                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24788|  %1728 = select i1 %1725, i64 %1726, i64 %1727                                                                         ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24789|     ;; dy = i64 %1728
 24790|  %1729 = mul i64 %1724, %1724                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24791|  %1730 = mul i64 %1728, %1728                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24792|  %1731 = add i64 %1730, %1729                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24793|  %1732 = icmp ult i64 %1731, 22500000001                                                                               ;L117<138<88<40<279<146<128<52<3674<142<118
 24794|  %1733 = zext i1 %1732 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<118
 24795|  br label %1734                                                                                                        ;L117<138<88<40<279<146<128<52<3674<142<118
 24796| 
 24797| 1734: ; preds = %1716, %1711, %1710
 24798|  %1735 = phi i64 [ 0, %1710 ], [ %1733, %1716 ], [ 0, %1711 ]                                                          ;L0<279<146<128<52<3674<142<118
 24799|     ;; acc = i64 %1735
 24800|     ;; i = i64 1
 24801|     ;; self = ptr %245
 24802|     ;; count = i64 1
 24803|  %1736 = gep %245, i64 8                                                                                               ;L656<279<146<128<52<3674<142<118
 24804|  %1737 = load ptr, ptr %1736, , !!32961, !!8                                                                           ;L279<146<128<52<3674<142<118
 24806|     ;; acc = i64 %1735
 24808|  %1738 = icmp eq ptr %1737, null                                                                                       ;L39<279<146<128<52<3674<142<118
 24809|  br i1 %1738, label %1765, label %1739                                                                                 ;L39<279<146<128<52<3674<142<118
 24810| 
 24811| 1739: ; preds = %1734
 24812|     ;; x = ptr %1737
 24814|     ;; acc = i64 %1735
 24815|     ;; elt = ptr %1737
 24816|     ;; x = ptr %1737
 24820|     ;; self = ptr %1737
 24821|     ;; entity = ptr %68
 24822|     ;; team = i64 %1585
 24824|  %1740 = gep %1737, i64 56                                                                                             ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 24825|  %1741 = gepS %1740, i64 %1586                                                                                         ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 24826|  %1742 = load i64, ptr %1741, , !!32961, !!8                                                                           ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 24827|  %1743 = icmp eq i64 %1742, 0                                                                                          ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 24828|  br i1 %1743, label %1744, label %1762                                                                                 ;L117<138<88<40<279<146<128<52<3674<142<118
 24829| 
 24830| 1744: ; preds = %1739
 24831|     ;; self = ptr %1737
 24832|     ;; other = ptr %68
 24833|  %1745 = gep %1737, i64 1632                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24834|  %1746 = load i64, ptr %1745, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24835|     ;; x1 = i64 %1746
 24836|     ;; self = i64 %1746
 24837|  %1747 = gep %1737, i64 1640                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24838|  %1748 = load i64, ptr %1747, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24839|     ;; y1 = i64 %1748
 24840|     ;; self = i64 %1748
 24841|     ;; x2 = i64 %1434
 24842|     ;; other = i64 %1434
 24843|     ;; y2 = i64 %1435
 24844|     ;; other = i64 %1435
 24845|  %1749 = icmp ult i64 %1746, %1434                                                                                     ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24846|  %1750 = sub nuw i64 %1434, %1746                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24847|  %1751 = sub nuw i64 %1746, %1434                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24848|  %1752 = select i1 %1749, i64 %1750, i64 %1751                                                                         ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24849|     ;; dx = i64 %1752
 24850|  %1753 = icmp ult i64 %1748, %1435                                                                                     ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24851|  %1754 = sub nuw i64 %1435, %1748                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24852|  %1755 = sub nuw i64 %1748, %1435                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24853|  %1756 = select i1 %1753, i64 %1754, i64 %1755                                                                         ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24854|     ;; dy = i64 %1756
 24855|  %1757 = mul i64 %1752, %1752                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24856|  %1758 = mul i64 %1756, %1756                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24857|  %1759 = add i64 %1758, %1757                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24858|  %1760 = icmp ult i64 %1759, 22500000001                                                                               ;L117<138<88<40<279<146<128<52<3674<142<118
 24859|  %1761 = zext i1 %1760 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<118
 24860|  br label %1762                                                                                                        ;L117<138<88<40<279<146<128<52<3674<142<118
 24861| 
 24862| 1762: ; preds = %1744, %1739
 24863|  %1763 = phi i64 [ %1761, %1744 ], [ 0, %1739 ]                                                                        ;L117<138<88<40<279<146<128<52<3674<142<118
 24865|     ;; a = i64 %1735
 24866|     ;; b = i64 %1763
 24867|  %1764 = add nuw nsw i64 %1763, %1735                                                                                  ;L55<88<40<279<146<128<52<3674<142<118
 24868|  br label %1765                                                                                                        ;L42<279<146<128<52<3674<142<118
 24869| 
 24870| 1765: ; preds = %1762, %1734
 24871|  %1766 = phi i64 [ %1764, %1762 ], [ %1735, %1734 ]                                                                    ;L0<279<146<128<52<3674<142<118
 24872|     ;; acc = i64 %1766
 24873|     ;; i = i64 2
 24874|     ;; self = ptr %245
 24875|     ;; count = i64 2
 24876|  %1767 = gep %245, i64 16                                                                                              ;L656<279<146<128<52<3674<142<118
 24877|  %1768 = load ptr, ptr %1767, , !!32961, !!8                                                                           ;L279<146<128<52<3674<142<118
 24879|     ;; acc = i64 %1766
 24881|  %1769 = icmp eq ptr %1768, null                                                                                       ;L39<279<146<128<52<3674<142<118
 24882|  br i1 %1769, label %1796, label %1770                                                                                 ;L39<279<146<128<52<3674<142<118
 24883| 
 24884| 1770: ; preds = %1765
 24885|     ;; x = ptr %1768
 24887|     ;; acc = i64 %1766
 24888|     ;; elt = ptr %1768
 24889|     ;; x = ptr %1768
 24893|     ;; self = ptr %1768
 24894|     ;; entity = ptr %68
 24895|     ;; team = i64 %1585
 24897|  %1771 = gep %1768, i64 56                                                                                             ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 24898|  %1772 = gepS %1771, i64 %1586                                                                                         ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 24899|  %1773 = load i64, ptr %1772, , !!32961, !!8                                                                           ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 24900|  %1774 = icmp eq i64 %1773, 0                                                                                          ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 24901|  br i1 %1774, label %1775, label %1793                                                                                 ;L117<138<88<40<279<146<128<52<3674<142<118
 24902| 
 24903| 1775: ; preds = %1770
 24904|     ;; self = ptr %1768
 24905|     ;; other = ptr %68
 24906|  %1776 = gep %1768, i64 1632                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24907|  %1777 = load i64, ptr %1776, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24908|     ;; x1 = i64 %1777
 24909|     ;; self = i64 %1777
 24910|  %1778 = gep %1768, i64 1640                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24911|  %1779 = load i64, ptr %1778, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24912|     ;; y1 = i64 %1779
 24913|     ;; self = i64 %1779
 24914|     ;; x2 = i64 %1434
 24915|     ;; other = i64 %1434
 24916|     ;; y2 = i64 %1435
 24917|     ;; other = i64 %1435
 24918|  %1780 = icmp ult i64 %1777, %1434                                                                                     ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24919|  %1781 = sub nuw i64 %1434, %1777                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24920|  %1782 = sub nuw i64 %1777, %1434                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24921|  %1783 = select i1 %1780, i64 %1781, i64 %1782                                                                         ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24922|     ;; dx = i64 %1783
 24923|  %1784 = icmp ult i64 %1779, %1435                                                                                     ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24924|  %1785 = sub nuw i64 %1435, %1779                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24925|  %1786 = sub nuw i64 %1779, %1435                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24926|  %1787 = select i1 %1784, i64 %1785, i64 %1786                                                                         ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24927|     ;; dy = i64 %1787
 24928|  %1788 = mul i64 %1783, %1783                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24929|  %1789 = mul i64 %1787, %1787                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24930|  %1790 = add i64 %1789, %1788                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 24931|  %1791 = icmp ult i64 %1790, 22500000001                                                                               ;L117<138<88<40<279<146<128<52<3674<142<118
 24932|  %1792 = zext i1 %1791 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<118
 24933|  br label %1793                                                                                                        ;L117<138<88<40<279<146<128<52<3674<142<118
 24934| 
 24935| 1793: ; preds = %1775, %1770
 24936|  %1794 = phi i64 [ %1792, %1775 ], [ 0, %1770 ]                                                                        ;L117<138<88<40<279<146<128<52<3674<142<118
 24938|     ;; a = i64 %1766
 24939|     ;; b = i64 %1794
 24940|  %1795 = add nuw nsw i64 %1794, %1766                                                                                  ;L55<88<40<279<146<128<52<3674<142<118
 24941|  br label %1796                                                                                                        ;L42<279<146<128<52<3674<142<118
 24942| 
 24943| 1796: ; preds = %1793, %1765
 24944|  %1797 = phi i64 [ %1795, %1793 ], [ %1766, %1765 ]                                                                    ;L0<279<146<128<52<3674<142<118
 24945|     ;; acc = i64 %1797
 24946|     ;; i = i64 3
 24947|     ;; self = ptr %245
 24948|     ;; count = i64 3
 24949|  %1798 = gep %245, i64 24                                                                                              ;L656<279<146<128<52<3674<142<118
 24950|  %1799 = load ptr, ptr %1798, , !!32961, !!8                                                                           ;L279<146<128<52<3674<142<118
 24952|     ;; acc = i64 %1797
 24954|  %1800 = icmp eq ptr %1799, null                                                                                       ;L39<279<146<128<52<3674<142<118
 24955|  br i1 %1800, label %1827, label %1801                                                                                 ;L39<279<146<128<52<3674<142<118
 24956| 
 24957| 1801: ; preds = %1796
 24958|     ;; x = ptr %1799
 24960|     ;; acc = i64 %1797
 24961|     ;; elt = ptr %1799
 24962|     ;; x = ptr %1799
 24966|     ;; self = ptr %1799
 24967|     ;; entity = ptr %68
 24968|     ;; team = i64 %1585
 24970|  %1802 = gep %1799, i64 56                                                                                             ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 24971|  %1803 = gepS %1802, i64 %1586                                                                                         ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 24972|  %1804 = load i64, ptr %1803, , !!32961, !!8                                                                           ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 24973|  %1805 = icmp eq i64 %1804, 0                                                                                          ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 24974|  br i1 %1805, label %1806, label %1824                                                                                 ;L117<138<88<40<279<146<128<52<3674<142<118
 24975| 
 24976| 1806: ; preds = %1801
 24977|     ;; self = ptr %1799
 24978|     ;; other = ptr %68
 24979|  %1807 = gep %1799, i64 1632                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24980|  %1808 = load i64, ptr %1807, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24981|     ;; x1 = i64 %1808
 24982|     ;; self = i64 %1808
 24983|  %1809 = gep %1799, i64 1640                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24984|  %1810 = load i64, ptr %1809, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 24985|     ;; y1 = i64 %1810
 24986|     ;; self = i64 %1810
 24987|     ;; x2 = i64 %1434
 24988|     ;; other = i64 %1434
 24989|     ;; y2 = i64 %1435
 24990|     ;; other = i64 %1435
 24991|  %1811 = icmp ult i64 %1808, %1434                                                                                     ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24992|  %1812 = sub nuw i64 %1434, %1808                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24993|  %1813 = sub nuw i64 %1808, %1434                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24994|  %1814 = select i1 %1811, i64 %1812, i64 %1813                                                                         ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 24995|     ;; dx = i64 %1814
 24996|  %1815 = icmp ult i64 %1810, %1435                                                                                     ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24997|  %1816 = sub nuw i64 %1435, %1810                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24998|  %1817 = sub nuw i64 %1810, %1435                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 24999|  %1818 = select i1 %1815, i64 %1816, i64 %1817                                                                         ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 25000|     ;; dy = i64 %1818
 25001|  %1819 = mul i64 %1814, %1814                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 25002|  %1820 = mul i64 %1818, %1818                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 25003|  %1821 = add i64 %1820, %1819                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 25004|  %1822 = icmp ult i64 %1821, 22500000001                                                                               ;L117<138<88<40<279<146<128<52<3674<142<118
 25005|  %1823 = zext i1 %1822 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<118
 25006|  br label %1824                                                                                                        ;L117<138<88<40<279<146<128<52<3674<142<118
 25007| 
 25008| 1824: ; preds = %1806, %1801
 25009|  %1825 = phi i64 [ %1823, %1806 ], [ 0, %1801 ]                                                                        ;L117<138<88<40<279<146<128<52<3674<142<118
 25011|     ;; a = i64 %1797
 25012|     ;; b = i64 %1825
 25013|  %1826 = add nuw nsw i64 %1825, %1797                                                                                  ;L55<88<40<279<146<128<52<3674<142<118
 25014|  br label %1827                                                                                                        ;L42<279<146<128<52<3674<142<118
 25015| 
 25016| 1827: ; preds = %1824, %1796
 25017|  %1828 = phi i64 [ %1826, %1824 ], [ %1797, %1796 ]                                                                    ;L0<279<146<128<52<3674<142<118
 25018|     ;; acc = i64 %1828
 25019|     ;; i = i64 4
 25020|     ;; self = ptr %245
 25021|     ;; count = i64 4
 25022|  %1829 = gep %245, i64 32                                                                                              ;L656<279<146<128<52<3674<142<118
 25023|  %1830 = load ptr, ptr %1829, , !!32961, !!8                                                                           ;L279<146<128<52<3674<142<118
 25025|     ;; acc = i64 %1828
 25027|  %1831 = icmp eq ptr %1830, null                                                                                       ;L39<279<146<128<52<3674<142<118
 25028|  br i1 %1831, label %1876, label %1832                                                                                 ;L39<279<146<128<52<3674<142<118
 25029| 
 25030| 1832: ; preds = %1827
 25031|     ;; x = ptr %1830
 25033|     ;; acc = i64 %1828
 25034|     ;; elt = ptr %1830
 25035|     ;; x = ptr %1830
 25039|     ;; self = ptr %1830
 25040|     ;; entity = ptr %68
 25041|     ;; team = i64 %1585
 25043|  %1833 = gep %1830, i64 56                                                                                             ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 25044|  %1834 = gepS %1833, i64 %1586                                                                                         ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 25045|  %1835 = load i64, ptr %1834, , !!32961, !!8                                                                           ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 25046|  %1836 = icmp eq i64 %1835, 0                                                                                          ;L122<1483<117<138<88<40<279<146<128<52<3674<142<118
 25047|  br i1 %1836, label %1837, label %1855                                                                                 ;L117<138<88<40<279<146<128<52<3674<142<118
 25048| 
 25049| 1837: ; preds = %1832
 25050|     ;; self = ptr %1830
 25051|     ;; other = ptr %68
 25052|  %1838 = gep %1830, i64 1632                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 25053|  %1839 = load i64, ptr %1838, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 25054|     ;; x1 = i64 %1839
 25055|     ;; self = i64 %1839
 25056|  %1840 = gep %1830, i64 1640                                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 25057|  %1841 = load i64, ptr %1840, , !!32961, !!8                                                                           ;L2158<117<138<88<40<279<146<128<52<3674<142<118
 25058|     ;; y1 = i64 %1841
 25059|     ;; self = i64 %1841
 25060|     ;; x2 = i64 %1434
 25061|     ;; other = i64 %1434
 25062|     ;; y2 = i64 %1435
 25063|     ;; other = i64 %1435
 25064|  %1842 = icmp ult i64 %1839, %1434                                                                                     ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 25065|  %1843 = sub nuw i64 %1434, %1839                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 25066|  %1844 = sub nuw i64 %1839, %1434                                                                                      ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 25067|  %1845 = select i1 %1842, i64 %1843, i64 %1844                                                                         ;L3147<7<2158<117<138<88<40<279<146<128<52<3674<142<118
 25068|     ;; dx = i64 %1845
 25069|  %1846 = icmp ult i64 %1841, %1435                                                                                     ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 25070|  %1847 = sub nuw i64 %1435, %1841                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 25071|  %1848 = sub nuw i64 %1841, %1435                                                                                      ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 25072|  %1849 = select i1 %1846, i64 %1847, i64 %1848                                                                         ;L3147<8<2158<117<138<88<40<279<146<128<52<3674<142<118
 25073|     ;; dy = i64 %1849
 25074|  %1850 = mul i64 %1845, %1845                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 25075|  %1851 = mul i64 %1849, %1849                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 25076|  %1852 = add i64 %1851, %1850                                                                                          ;L9<2158<117<138<88<40<279<146<128<52<3674<142<118
 25077|  %1853 = icmp ult i64 %1852, 22500000001                                                                               ;L117<138<88<40<279<146<128<52<3674<142<118
 25078|  %1854 = zext i1 %1853 to i64                                                                                          ;L138<88<40<279<146<128<52<3674<142<118
 25079|  br label %1855                                                                                                        ;L117<138<88<40<279<146<128<52<3674<142<118
 25080| 
 25081| 1855: ; preds = %1837, %1832
 25082|  %1856 = phi i64 [ %1854, %1837 ], [ 0, %1832 ]                                                                        ;L117<138<88<40<279<146<128<52<3674<142<118
 25084|     ;; a = i64 %1828
 25085|     ;; b = i64 %1856
 25086|  %1857 = add nuw nsw i64 %1856, %1828                                                                                  ;L55<88<40<279<146<128<52<3674<142<118
 25087|  br label %1876                                                                                                        ;L42<279<146<128<52<3674<142<118
 25088| 
 25089| 1858: ; preds = %1872, %1868, %1864, %1860, %1709
 25090|     ;; x = ptr %1707
 25093|     ;; elt = ptr %1707
 25094|     ;; x = ptr %1707
 25098|     ;; self = ptr %1707
 25099|     ;; entity = ptr %68
 25100|     ;; team = i64 %1585
 25101|  invoke void @core::panicking18panic_bounds_check(i64 %1586, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.26) #31
 25102|  to label %1859 unwind label %57                                                                                       ;L1483<117<138<88<40<279<146<128<52<3674<142<118
 25103| 
 25104| 1859: ; preds = %1858
 25105|  unreachable                                                                                                           ;L1483<117<138<88<40<279<146<128<52<3674<142<118
 25106| 
 25107| 1860: ; preds = %1709
 25109|     ;; i = i64 1
 25110|     ;; self = ptr %245
 25111|     ;; count = i64 1
 25112|  %1861 = gep %245, i64 8                                                                                               ;L656<279<146<128<52<3674<142<118
 25113|  %1862 = load ptr, ptr %1861, , !!32961, !!8                                                                           ;L279<146<128<52<3674<142<118
 25117|  %1863 = icmp eq ptr %1862, null                                                                                       ;L39<279<146<128<52<3674<142<118
 25118|  br i1 %1863, label %1864, label %1858                                                                                 ;L39<279<146<128<52<3674<142<118
 25119| 
 25120| 1864: ; preds = %1860
 25122|     ;; i = i64 2
 25123|     ;; self = ptr %245
 25124|     ;; count = i64 2
 25125|  %1865 = gep %245, i64 16                                                                                              ;L656<279<146<128<52<3674<142<118
 25126|  %1866 = load ptr, ptr %1865, , !!32961, !!8                                                                           ;L279<146<128<52<3674<142<118
 25130|  %1867 = icmp eq ptr %1866, null                                                                                       ;L39<279<146<128<52<3674<142<118
 25131|  br i1 %1867, label %1868, label %1858                                                                                 ;L39<279<146<128<52<3674<142<118
 25132| 
 25133| 1868: ; preds = %1864
 25135|     ;; i = i64 3
 25136|     ;; self = ptr %245
 25137|     ;; count = i64 3
 25138|  %1869 = gep %245, i64 24                                                                                              ;L656<279<146<128<52<3674<142<118
 25139|  %1870 = load ptr, ptr %1869, , !!32961, !!8                                                                           ;L279<146<128<52<3674<142<118
 25143|  %1871 = icmp eq ptr %1870, null                                                                                       ;L39<279<146<128<52<3674<142<118
 25144|  br i1 %1871, label %1872, label %1858                                                                                 ;L39<279<146<128<52<3674<142<118
 25145| 
 25146| 1872: ; preds = %1868
 25148|     ;; i = i64 4
 25149|     ;; self = ptr %245
 25150|     ;; count = i64 4
 25151|  %1873 = gep %245, i64 32                                                                                              ;L656<279<146<128<52<3674<142<118
 25152|  %1874 = load ptr, ptr %1873, , !!32961, !!8                                                                           ;L279<146<128<52<3674<142<118
 25156|  %1875 = icmp eq ptr %1874, null                                                                                       ;L39<279<146<128<52<3674<142<118
 25157|  br i1 %1875, label %1876, label %1858                                                                                 ;L39<279<146<128<52<3674<142<118
 25158| 
 25159| 1876: ; preds = %1872, %1855, %1827, %1686, %1681
 25160|  %1877 = phi i64 [ %1682, %1681 ], [ %1828, %1827 ], [ %1704, %1686 ], [ %1857, %1855 ], [ 0, %1872 ]                  ;L0<146<128<52<3674<142<118
 25161|     ;; nearby_enemies = i64 %1877
 25162|  %1878 = icmp samesign ult i64 %1581, %1877                                                                            ;L120
 25163|     ;; dominated = i1 %1878
 25164|  br i1 %1878, label %1881, label %1879                                                                                 ;L121
 25165| 
 25166| 1879: ; preds = %1876
 25167|  %1880 = invoke zeroext i1 @ai::utils9can1v1win(ptr %3, ptr %4, ptr %5, ptr %68, ptr %1430)
 25168|  to label %1882 unwind label %57                                                                                       ;L121
 25169| 
 25170| 1881: ; preds = %1882, %1876
 25173|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %24, ptr %5, ptr %4, i64 5, i1 zeroext false)
 25174|  to label %1884 unwind label %57                                                                                       ;L126
 25175| 
 25176| 1882: ; preds = %1879
 25177|     ;; can_fight = i1 %1880
 25178|  br i1 %1880, label %1883, label %1881                                                                                 ;L123
 25179| 
 25180| 1883: ; preds = %1882
 25182|  invoke void @ai::fight_check13battle_action(ptr sret([32 x i8]) %26, i64 %2, ptr %3, ptr %4, ptr %5, i64 5)
 25183|  to label %1905 unwind label %57                                                                                       ;L124
 25184| 
 25185| 1884: ; preds = %1881
 25186|  call void @llvm.memcpy.p0.p0.i64(ptr %25, ptr %24, i64 136, i1 false)                                                 ;L126
 25187|  %1885 = gep %25, i64 177                                                                                              ;L126
 25188|  store i8 3, ptr %1885,                                                                                                ;L126
 25191|     ;; self = ptr %46
 25192|     ;; self = ptr %46
 25193|     ;; value = ptr %25
 25194|     ;; src = ptr %25
 25195|     ;; additional = i64 1
 25196|     ;; needed_extra_cap = i64 1
 25197|     ;; needed_extra_cap = i64 1
 25198|     ;; strategy = i8 1
 25199|  %1886 = load i64, ptr %52, , !!33103, !!8                                                                             ;L1428<126
 25200|     ;; self = ptr %46
 25201|  %1887 = load i64, ptr %51, , !!33103, !!8                                                                             ;L149<1428<126
 25202|  %1888 = icmp eq i64 %1886, %1887                                                                                      ;L1428<126
 25203|  br i1 %1888, label %1889, label %1894                                                                                 ;L1428<126
 25204| 
 25205| 1889: ; preds = %1884
 25206|     ;; self = ptr %46
 25207|     ;; self = ptr %46
 25208|     ;; self = ptr %46
 25209|     ;; used_cap = i64 %1886
 25210|     ;; used_cap = i64 %1886
 25211|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %46, i64 %1886, i64 1, i1 zeroext true)
 25212|  to label %1890 unwind label %1892, !!33103                                                                            ;L619<430<738<1429<126
 25213| 
 25214| 1890: ; preds = %1889
 25215|  %1891 = load i64, ptr %52, , !!33103                                                                                  ;L1432<126
 25216|  br label %1894                                                                                                        ;L619<430<738<1429<126
 25217| 
 25218| 1892: ; preds = %1889
 25219|  %1893 = cleanuppad within none []
 25220|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %25) #30 [ "funclet"(token %1893) ], !!33088 ;L1436<126
 25221|  cleanupret from %1893 unwind label %57
 25222| 
 25223| 1894: ; preds = %1890, %1884
 25224|  %1895 = phi i64 [ %1891, %1890 ], [ %1886, %1884 ]                                                                    ;L1432<126
 25225|     ;; self = ptr %46
 25226|  %1896 = load ptr, ptr %46, , !!33103, !!8, !!8                                                                        ;L138<1432<126
 25227|     ;; self = ptr %1896
 25228|     ;; count = i64 %1895
 25229|  %1897 = gepS %1896, i64 %1895                                                                                         ;L961<1432<126
 25230|     ;; end = ptr %1897
 25231|     ;; dst = ptr %1897
 25232|  call void @llvm.memcpy.p0.p0.i64(ptr %1897, ptr %25, i64 184, i1 false), !!33088                                      ;L1933<1433<126
 25233|  %1898 = add i64 %1895, 1                                                                                              ;L1434<126
 25234|  store i64 %1898, ptr %52, , !!33103                                                                                   ;L1434<126
 25236|  br i1 %1878, label %243, label %1899                                                                                  ;L127
 25237| 
 25238| 1899: ; preds = %1894
 25240|  invoke void @ai::fight_check13battle_action(ptr sret([32 x i8]) %23, i64 %2, ptr %3, ptr %4, ptr %5, i64 5)
 25241|  to label %1900 unwind label %57                                                                                       ;L128
 25242| 
 25243| 1900: ; preds = %1899
 25244|  %1901 = load ptr, ptr %23, , !!8, !!8                                                                                 ;L128
 25245|  %1902 = gep %23, i64 24                                                                                               ;L128
 25246|  %1903 = load i64, ptr %1902, , !!8                                                                                    ;L128
 25247|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %46, ptr %1901, i64 %1903)
 25248|  to label %1904 unwind label %57                                                                                       ;L128
 25249| 
 25250| 1904: ; preds = %1900
 25252|  br label %243                                                                                                         ;L127
 25253| 
 25254| 1905: ; preds = %1883
 25255|  %1906 = load ptr, ptr %26, , !!8, !!8                                                                                 ;L124
 25256|  %1907 = gep %26, i64 24                                                                                               ;L124
 25257|  %1908 = load i64, ptr %1907, , !!8                                                                                    ;L124
 25258|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %46, ptr %1906, i64 %1908)
 25259|  to label %1909 unwind label %57                                                                                       ;L124
 25260| 
 25261| 1909: ; preds = %1905
 25263|  br label %243                                                                                                         ;L123
 25264| 
 25265| 1910: ; preds = %1587
 25266|  call void @llvm.memcpy.p0.p0.i64(ptr %22, ptr %21, i64 136, i1 false)                                                 ;L132
 25267|  %1911 = gep %22, i64 177                                                                                              ;L132
 25268|  store i8 3, ptr %1911,                                                                                                ;L132
 25271|     ;; self = ptr %46
 25272|     ;; self = ptr %46
 25273|     ;; value = ptr %22
 25274|     ;; src = ptr %22
 25275|     ;; additional = i64 1
 25276|     ;; needed_extra_cap = i64 1
 25277|     ;; needed_extra_cap = i64 1
 25278|     ;; strategy = i8 1
 25279|  %1912 = load i64, ptr %52, , !!33139, !!8                                                                             ;L1428<132
 25280|     ;; self = ptr %46
 25281|  %1913 = load i64, ptr %51, , !!33139, !!8                                                                             ;L149<1428<132
 25282|  %1914 = icmp eq i64 %1912, %1913                                                                                      ;L1428<132
 25283|  br i1 %1914, label %1915, label %1920                                                                                 ;L1428<132
 25284| 
 25285| 1915: ; preds = %1910
 25286|     ;; self = ptr %46
 25287|     ;; self = ptr %46
 25288|     ;; self = ptr %46
 25289|     ;; used_cap = i64 %1912
 25290|     ;; used_cap = i64 %1912
 25291|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %46, i64 %1912, i64 1, i1 zeroext true)
 25292|  to label %1916 unwind label %1918, !!33139                                                                            ;L619<430<738<1429<132
 25293| 
 25294| 1916: ; preds = %1915
 25295|  %1917 = load i64, ptr %52, , !!33139                                                                                  ;L1432<132
 25296|  br label %1920                                                                                                        ;L619<430<738<1429<132
 25297| 
 25298| 1918: ; preds = %1915
 25299|  %1919 = cleanuppad within none []
 25300|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %22) #30 [ "funclet"(token %1919) ], !!33124 ;L1436<132
 25301|  cleanupret from %1919 unwind label %57
 25302| 
 25303| 1920: ; preds = %1916, %1910
 25304|  %1921 = phi i64 [ %1917, %1916 ], [ %1912, %1910 ]                                                                    ;L1432<132
 25305|     ;; self = ptr %46
 25306|  %1922 = load ptr, ptr %46, , !!33139, !!8, !!8                                                                        ;L138<1432<132
 25307|     ;; self = ptr %1922
 25308|     ;; count = i64 %1921
 25309|  %1923 = gepS %1922, i64 %1921                                                                                         ;L961<1432<132
 25310|     ;; end = ptr %1923
 25311|     ;; dst = ptr %1923
 25312|  call void @llvm.memcpy.p0.p0.i64(ptr %1923, ptr %22, i64 184, i1 false), !!33124                                      ;L1933<1433<132
 25313|  %1924 = add i64 %1921, 1                                                                                              ;L1434<132
 25314|  store i64 %1924, ptr %52, , !!33139                                                                                   ;L1434<132
 25317|  invoke void @ai::fight_check13battle_action(ptr sret([32 x i8]) %20, i64 %2, ptr %3, ptr %4, ptr %5, i64 5)
 25318|  to label %1925 unwind label %57                                                                                       ;L133
 25319| 
 25320| 1925: ; preds = %1920
 25321|  %1926 = load ptr, ptr %20, , !!8, !!8                                                                                 ;L133
 25322|  %1927 = gep %20, i64 24                                                                                               ;L133
 25323|  %1928 = load i64, ptr %1927, , !!8                                                                                    ;L133
 25324|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %46, ptr %1926, i64 %1928)
 25325|  to label %1929 unwind label %57                                                                                       ;L133
 25326| 
 25327| 1929: ; preds = %1925
 25329|  br label %243                                                                                                         ;L112
 25330| 
 25331| 1930: ; preds = %243
 25332|  %1931 = load ptr, ptr %19, , !!8, !!8                                                                                 ;L136
 25333|  %1932 = gep %19, i64 24                                                                                               ;L136
 25334|  %1933 = load i64, ptr %1932, , !!8                                                                                    ;L136
 25335|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %46, ptr %1931, i64 %1933)
 25336|  to label %1934 unwind label %57                                                                                       ;L136
 25337| 
 25338| 1934: ; preds = %1930
 25340|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %46, i64 32, i1 false)                                                   ;L138
 25342|  ret void                                                                                                              ;L139
 25343| }
