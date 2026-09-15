 53032| define zeroext i1 @ai::tower_discipline47v22_current_line_non_champion_action_tower_risk(i64 %0, ptr %1, ptr %2, ptr %3, ptr %4) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 53033|  %6 = alloca [56 x i8],
 53034|  %7 = alloca [120 x i8],
 53035|  %8 = alloca [1 x i8],
 53036|     ;; version = i64 %0
 53037|     ;; context = ptr %1
 53038|     ;; self = ptr %1
 53039|     ;; cache = ptr %2
 53040|     ;; player = ptr %3
 53042|     ;; target = ptr %4
 53043|     ;; recently_hit_by_enemy_tower = ptr %8
 53045|     ;; __arg1_discr = i64 0
 53048|  %9 = load ptr, ptr %2, , !!8, !!8                                                                                     ;L225
 53049|  %10 = gep %2, i64 8                                                                                                   ;L225
 53050|  %11 = load ptr, ptr %10, , !!8, !!8                                                                                   ;L225
 53051|     ;; f[0..+8] = ptr %9
 53052|     ;; f[8..+8] = ptr %11
 53053|  %12 = gep %11, i64 40                                                                                                 ;L225
 53054|  %13 = load ptr, ptr %12, , !!8                                                                                        ;L225
 53055|  %14 = tail call i64 %13(ptr %9)                                                                                       ;L225
 53056|     ;; tick = i64 %14
 53057|     ;; tick = i64 %14
 53058|     ;; self = ptr %1
 53059|  %15 = gep %1, i64 56                                                                                                  ;L263<399<225
 53060|  %16 = load i8, ptr %15, , !!8                                                                                         ;L263<399<225
 53061|  %17 = gep %1, i64 8
 53062|  %18 = load ptr, ptr %17,                                                                                              ;L0
 53063|  switch i8 %16, label %27 [
 53064|  i8 0, label %19
 53065|  i8 7, label %19
 53066|  i8 8, label %19
 53067|  i8 5, label %19
 53068|  ]                                                                                                                     ;L263<399<225
 53069| 
 53070| 19: ; preds = %5, %5, %5, %5
 53071|     ;; self = ptr %18
 53072|  %20 = gep %18, i64 2216                                                                                               ;L703<399<225
 53073|  %21 = load i64, ptr %20, , !!8                                                                                        ;L703<399<225
 53074|     ;; self = i64 %21
 53075|  %22 = gep %18, i64 4856                                                                                               ;L704<399<225
 53076|  %23 = load i64, ptr %22, , !!8                                                                                        ;L704<399<225
 53077|  %24 = mul i64 %23, 30                                                                                                 ;L704<399<225
 53078|     ;; rhs = i64 %24
 53079|  %25 = tail call i64 @llvm.usub.sat.i64(i64 %21, i64 %24)                                                              ;L2472<703<399<225
 53080|  %26 = icmp ult i64 %14, %25                                                                                           ;L703<399<225
 53081|  br i1 %26, label %27, label %81                                                                                       ;L225
 53082| 
 53083| 27: ; preds = %19, %5
 53084|  %28 = tail call i64 %13(ptr %9)                                                                                       ;L226
 53085|  %29 = gep %18, i64 5112                                                                                               ;L226
 53086|  %30 = load i64, ptr %29, , !!8                                                                                        ;L226
 53087|  %31 = icmp ult i64 %28, %30                                                                                           ;L226
 53088|  br i1 %31, label %32, label %81                                                                                       ;L226
 53089| 
 53090| 32: ; preds = %27
 53091|  %33 = gep %3, i64 2352                                                                                                ;L230
 53092|  %34 = load i64, ptr %33, , !!8                                                                                        ;L230
 53093|  %35 = icmp ult i64 %34, 2                                                                                             ;L230
 53094|  br i1 %35, label %37, label %36                                                                                       ;L230
 53095| 
 53096| 36: ; preds = %32
 53097|  tail call void @core::panicking18panic_bounds_check(i64 %34, i64 2, ptr @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.182) #25 ;L230
 53098|  unreachable                                                                                                           ;L230
 53099| 
 53100| 37: ; preds = %32
 53101|     ;; self = ptr %3
 53102|  %38 = gep %3, i64 2496                                                                                                ;L581<230
 53103|  %39 = load i32, ptr %38, , !!8                                                                                        ;L581<230
 53104|  %40 = zext nneg i32 %39 to i64                                                                                        ;L581<230
 53105|  %41 = gep %2, i64 480                                                                                                 ;L230
 53106|  %42 = getelementptr [5 x ptr], ptr %41, i64 %34                                                                       ;L230
 53107|  %43 = getelementptr ptr, ptr %42, i64 %40                                                                             ;L230
 53108|  %44 = load ptr, ptr %43, , !!8                                                                                        ;L230
 53109|  %45 = icmp eq ptr %44, null                                                                                           ;L230
 53110|  br i1 %45, label %81, label %46                                                                                       ;L230
 53111| 
 53112| 46: ; preds = %37
 53113|     ;; champ = ptr %44
 53115|  %47 = gep %44, i64 40                                                                                                 ;L233
 53116|  %48 = load i64, ptr %47, , !!8                                                                                        ;L233
 53117|     ;; self[0..+8] = i64 %48
 53119|  %49 = trunc nuw i64 %48 to i1                                                                                         ;L1542<234
 53120|  br i1 %49, label %50, label %60                                                                                       ;L1542<234
 53121| 
 53122| 50: ; preds = %46
 53123|  %51 = gep %44, i64 48                                                                                                 ;L233
 53124|  %52 = load i64, ptr %51,                                                                                              ;L233
 53125|     ;; self[8..+8] = i64 %52
 53126|     ;; x = i64 %52
 53127|     ;; id = i64 %52
 53128|  %53 = gep %11, i64 496                                                                                                ;L234<1543<234
 53129|  %54 = load ptr, ptr %53, , !!8                                                                                        ;L234<1543<234
 53130|  %55 = tail call ptr %54(ptr %9, i64 %52)                                                                              ;L234<1543<234
 53131|     ;; self = ptr %55
 53132|     ;; f = ptr %3
 53133|  %56 = icmp eq ptr %55, null                                                                                           ;L659<235
 53134|  br i1 %56, label %60, label %57                                                                                       ;L659<235
 53135| 
 53136| 57: ; preds = %50
 53137|     ;; x = ptr %55
 53138|     ;; e = ptr %55
 53139|     ;; self = ptr %55
 53141|  %58 = load i64, ptr %55, , !!8                                                                                        ;L1127<235<661<235
 53142|     ;; __self_discr = i64 %58
 53143|  %59 = icmp eq i64 %58, 0                                                                                              ;L1127<235<661<235
 53144|  br i1 %59, label %71, label %60                                                                                       ;L1127<235<661<235
 53145| 
 53146| 60: ; preds = %76, %71, %57, %50, %46
 53147|  %61 = phi i8 [ %80, %76 ], [ 0, %71 ], [ 0, %57 ], [ 0, %46 ], [ 0, %50 ]
 53148|  store i8 %61, ptr %8,                                                                                                 ;L0<235
 53150|  %62 = sub nuw nsw i64 1, %34                                                                                          ;L237
 53151|  call void @gc::simulationNtB5_21AbstractGameWithCache25iter_towers_without_nexus(ptr sret([120 x i8]) %7, ptr %2, i64 %62) ;L237
 53152|     ;; self = ptr %7
 53153|     ;; self = ptr %7
 53154|     ;; f[0..+8] = ptr %44
 53155|     ;; f[8..+8] = ptr %4
 53156|     ;; f[16..+8] = ptr %8
 53157|     ;; f[24..+8] = ptr %1
 53158|     ;; f[32..+8] = ptr %2
 53159|     ;; f[40..+8] = ptr %3
 53160|     ;; fold[0..+8] = ptr %44
 53161|     ;; fold[0..+8] = ptr %44
 53162|     ;; fold[8..+8] = ptr %4
 53163|     ;; fold[8..+8] = ptr %4
 53164|     ;; fold[16..+8] = ptr %8
 53165|     ;; fold[16..+8] = ptr %8
 53166|     ;; fold[24..+8] = ptr %1
 53167|     ;; fold[24..+8] = ptr %1
 53168|     ;; fold[32..+8] = ptr %2
 53169|     ;; fold[32..+8] = ptr %2
 53170|     ;; fold[40..+8] = ptr %3
 53171|     ;; fold[40..+8] = ptr %3
 53173|  %63 = gep %7, i64 120                                                                                                 ;L157<2897<239
 53174|     ;; predicate = ptr %63
 53175|  store ptr %63, ptr %6,                                                                                                ;L86<157<2897<239
 53176|  %64 = gep %6, i64 8                                                                                                   ;L86<157<2897<239
 53177|  store ptr %44, ptr %64,                                                                                               ;L86<157<2897<239
 53178|  %65 = gep %6, i64 16                                                                                                  ;L86<157<2897<239
 53179|  store ptr %4, ptr %65,                                                                                                ;L86<157<2897<239
 53180|  %66 = gep %6, i64 24                                                                                                  ;L86<157<2897<239
 53181|  store ptr %8, ptr %66,                                                                                                ;L86<157<2897<239
 53182|  %67 = gep %6, i64 32                                                                                                  ;L86<157<2897<239
 53183|  store ptr %1, ptr %67,                                                                                                ;L86<157<2897<239
 53184|  %68 = gep %6, i64 40                                                                                                  ;L86<157<2897<239
 53185|  store ptr %2, ptr %68,                                                                                                ;L86<157<2897<239
 53186|  %69 = gep %6, i64 48                                                                                                  ;L86<157<2897<239
 53187|  store ptr %3, ptr %69,                                                                                                ;L86<157<2897<239
 53188|  %70 = call zeroext i1 @core::iter8adapters5chainINtB5_5ChainINtNtB7_7flatten7FlattenINtNtNtBb_5array4iter8IntoIterINtNtBb_6option6OptionRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEKj6_EEINtNtB7_6copied6CopiedINtNtNtBb_5slice4iter4IterB2e_EEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNtB7_6filter15filter_try_foldB2e_uINtNtNtBb_3ops12control_flow11ControlFlowuENCNvNtCshdEBA0ozCnw_7game_ai16tower_discipline47v22_current_line_non_champion_action_tower_risks0_0NCINvNvB49_3any5checkB2e_NCB6d_s1_0E0E0B5u_EB6h_(ptr %7, ptr %6) ;L157<2897<239
 53192|  br label %81                                                                                                          ;L264
 53193| 
 53194| 71: ; preds = %57
 53195|  %72 = gep %55, i64 8                                                                                                  ;L1127<235<661<235
 53196|  %73 = sub nuw nsw i64 1, %34                                                                                          ;L235<661<235
 53197|     ;; __self_0 = ptr %55
 53198|     ;; self = ptr %55
 53203|  %74 = load i64, ptr %72, , !!8                                                                                        ;L1878<2123<1127<235<661<235
 53204|  %75 = icmp eq i64 %74, %73                                                                                            ;L1878<2123<1127<235<661<235
 53205|  br i1 %75, label %76, label %60                                                                                       ;L235<661<235
 53206| 
 53207| 76: ; preds = %71
 53208|     ;; self = ptr %55
 53209|  %77 = gep %55, i64 104                                                                                                ;L1386<235<661<235
 53210|  %78 = load i64, ptr %77, , !!8                                                                                        ;L1386<235<661<235
 53211|  %79 = icmp eq i64 %78, 2                                                                                              ;L1386<235<661<235
 53212|  %80 = zext i1 %79 to i8                                                                                               ;L1386<235<661<235
 53213|  br label %60                                                                                                          ;L235<661<235
 53214| 
 53215| 81: ; preds = %60, %37, %27, %19
 53216|  %82 = phi i1 [ false, %19 ], [ %70, %60 ], [ false, %27 ], [ false, %37 ]                                             ;L0
 53217|  ret i1 %82                                                                                                            ;L264
 53218| }
