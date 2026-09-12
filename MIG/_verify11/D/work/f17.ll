define void @_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old12death_battleNtB2_16DeathMatchBattle3new(ptr dead_on_unwind noalias noundef writable writeonly sret([384 x i8]) align 8 captures(none) dereferenceable(384) %0, i64 noundef %1, ptr dead_on_return noalias noundef readonly align 8 captures(address) dereferenceable(24) %2, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(2528) %4) unnamed_addr #2 personality ptr @__CxxFrameHandler3 !dbg !26946 {
  %6 = alloca [24 x i8], align 8
    #dbg_value(i64 %1, !26951, !DIExpression(), !26959)
    #dbg_declare(ptr %2, !26952, !DIExpression(), !26960)
    #dbg_value(ptr %3, !26953, !DIExpression(), !26959)
    #dbg_value(ptr %4, !26954, !DIExpression(), !26959)
    #dbg_declare(ptr %6, !26955, !DIExpression(), !26961)
  call void @llvm.lifetime.start.p0(ptr nonnull %6), !dbg !26962
  store i64 0, ptr %6, align 8, !dbg !26963
  %7 = getelementptr inbounds nuw i8, ptr %6, i64 8, !dbg !26963
  store ptr inttoptr (i64 8 to ptr), ptr %7, align 8, !dbg !26963
  %8 = getelementptr inbounds nuw i8, ptr %6, i64 16, !dbg !26963
  store i64 0, ptr %8, align 8, !dbg !26963
  %9 = load i64, ptr %2, align 8, !dbg !26968, !range !20208, !noundef !8
  %10 = icmp eq i64 %9, 0, !dbg !26968
  br i1 %10, label %11, label %14, !dbg !26968

11:                                               ; preds = %5
  %12 = getelementptr inbounds nuw i8, ptr %2, i64 8, !dbg !26968
  %13 = load i64, ptr %12, align 8, !dbg !26968, !noundef !8
    #dbg_value(i64 %13, !26957, !DIExpression(), !26969)
    #dbg_value(ptr %6, !26970, !DIExpression(), !26977)
    #dbg_value(i64 %13, !26976, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !26977)
    #dbg_value(i64 %13, !26979, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !26992)
    #dbg_value(i64 %13, !26994, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !27000)
    #dbg_value(i64 0, !26976, !DIExpression(DW_OP_LLVM_fragment, 128, 64), !26977)
    #dbg_value(i64 0, !26979, !DIExpression(DW_OP_LLVM_fragment, 128, 64), !26992)
    #dbg_value(i64 0, !26994, !DIExpression(DW_OP_LLVM_fragment, 128, 64), !27000)
    #dbg_value(i8 3, !26976, !DIExpression(DW_OP_LLVM_fragment, 0, 8), !26977)
    #dbg_value(i8 3, !26979, !DIExpression(DW_OP_LLVM_fragment, 0, 8), !26992)
    #dbg_value(i8 3, !26994, !DIExpression(DW_OP_LLVM_fragment, 0, 8), !27000)
    #dbg_value(ptr %6, !26986, !DIExpression(), !26992)
    #dbg_value(ptr %6, !27002, !DIExpression(), !27008)
    #dbg_value(i64 0, !26987, !DIExpression(), !27010)
  invoke void @_RNvMs3_NtCs9LexZzt9XJB_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player4ChatE8grow_oneCshdEBA0ozCnw_7game_ai(ptr noalias noundef nonnull align 8 dereferenceable(24) %6)
          to label %18 unwind label %16, !dbg !27011

14:                                               ; preds = %18, %5
  %15 = invoke { i64, i64 } @_RNvMs_NtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6battleNtB4_14BattlePlanGoal13base_sub_goal(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %2, i64 noundef %1, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %4, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %3)
          to label %22 unwind label %16, !dbg !27012

16:                                               ; preds = %22, %14, %11
  %17 = cleanuppad within none []
  call void @_RINvNtCsjihNppCmMEE_4core3ptr9drop_glueINtNtCs9LexZzt9XJB_5alloc3vec3VecNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player4ChatEECshdEBA0ozCnw_7game_ai(ptr noalias noundef nonnull align 8 dereferenceable(24) %6) #27 [ "funclet"(token %17) ], !dbg !27013
  cleanupret from %17 unwind to caller, !dbg !26960

18:                                               ; preds = %11
  %19 = load ptr, ptr %7, align 8, !dbg !27014, !alias.scope !27036, !noalias !27039, !nonnull !8, !noundef !8
    #dbg_value(ptr %19, !26989, !DIExpression(), !27041)
    #dbg_value(ptr %19, !26999, !DIExpression(), !27000)
  store i8 3, ptr %19, align 8, !dbg !27042
  %20 = getelementptr inbounds nuw i8, ptr %19, i64 8, !dbg !27042
  store i64 %13, ptr %20, align 8, !dbg !27042
  %21 = getelementptr inbounds nuw i8, ptr %19, i64 16, !dbg !27042
  store i64 0, ptr %21, align 8, !dbg !27042
  store i64 1, ptr %8, align 8, !dbg !27043, !alias.scope !27036, !noalias !27039
  br label %14, !dbg !27044

22:                                               ; preds = %14
  %23 = load ptr, ptr %3, align 8, !dbg !27045, !nonnull !8, !align !8990, !noundef !8
  %24 = load ptr, ptr %23, align 8, !dbg !27045, !nonnull !8, !noundef !8
  %25 = getelementptr inbounds nuw i8, ptr %23, i64 8, !dbg !27045
  %26 = load ptr, ptr %25, align 8, !dbg !27045, !nonnull !8, !align !8990, !noundef !8
  %27 = getelementptr inbounds nuw i8, ptr %26, i64 40, !dbg !27045
  %28 = load ptr, ptr %27, align 8, !dbg !27045, !invariant.load !8, !nonnull !8
  %29 = invoke noundef i64 %28(ptr noundef nonnull %24)
          to label %30 unwind label %16, !dbg !27045

30:                                               ; preds = %22
  %31 = extractvalue { i64, i64 } %15, 1, !dbg !27012
  %32 = extractvalue { i64, i64 } %15, 0, !dbg !27012
  %33 = getelementptr inbounds nuw i8, ptr %0, i64 248, !dbg !27012
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %33, ptr noundef nonnull align 8 dereferenceable(24) %6, i64 24, i1 false), !dbg !27046
  %34 = getelementptr inbounds nuw i8, ptr %0, i64 208, !dbg !27012
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %34, ptr noundef nonnull align 8 dereferenceable(24) %2, i64 24, i1 false), !dbg !27012
  %35 = getelementptr inbounds nuw i8, ptr %0, i64 232, !dbg !27012
  store i64 %32, ptr %35, align 8, !dbg !27012
  %36 = getelementptr inbounds nuw i8, ptr %0, i64 240, !dbg !27012
  store i64 %31, ptr %36, align 8, !dbg !27012
  %37 = getelementptr inbounds nuw i8, ptr %0, i64 374, !dbg !27012
  store i8 0, ptr %37, align 2, !dbg !27012
  %38 = getelementptr inbounds nuw i8, ptr %0, i64 375, !dbg !27012
  store i8 0, ptr %38, align 1, !dbg !27012
  store i64 0, ptr %0, align 8, !dbg !27012
  %39 = getelementptr inbounds nuw i8, ptr %0, i64 368, !dbg !27012
  store i8 0, ptr %39, align 8, !dbg !27012
  %40 = getelementptr inbounds nuw i8, ptr %0, i64 378, !dbg !27012
  store i8 -1, ptr %40, align 2, !dbg !27012
  %41 = getelementptr inbounds nuw i8, ptr %0, i64 272, !dbg !27012
  store i64 %29, ptr %41, align 8, !dbg !27012
  %42 = getelementptr inbounds nuw i8, ptr %0, i64 16, !dbg !27012
  store i64 0, ptr %42, align 8, !dbg !27012
  %43 = getelementptr inbounds nuw i8, ptr %0, i64 369, !dbg !27012
  store i8 0, ptr %43, align 1, !dbg !27012
  %44 = getelementptr inbounds nuw i8, ptr %0, i64 48, !dbg !27012
  store i64 0, ptr %44, align 8, !dbg !27012
  %45 = getelementptr inbounds nuw i8, ptr %0, i64 379, !dbg !27012
  store i8 -1, ptr %45, align 1, !dbg !27012
  %46 = getelementptr inbounds nuw i8, ptr %0, i64 370, !dbg !27012
  store i8 0, ptr %46, align 2, !dbg !27012
  %47 = getelementptr inbounds nuw i8, ptr %0, i64 64, !dbg !27012
  store i64 0, ptr %47, align 8, !dbg !27012
  %48 = getelementptr inbounds nuw i8, ptr %0, i64 371, !dbg !27012
  store i8 0, ptr %48, align 1, !dbg !27012
  %49 = getelementptr inbounds nuw i8, ptr %0, i64 372, !dbg !27012
  store i8 0, ptr %49, align 4, !dbg !27012
  %50 = getelementptr inbounds nuw i8, ptr %0, i64 80, !dbg !27012
  store i64 0, ptr %50, align 8, !dbg !27012
  %51 = getelementptr inbounds nuw i8, ptr %0, i64 280, !dbg !27012
  store i64 9223372036854775807, ptr %51, align 8, !dbg !27012
  %52 = getelementptr inbounds nuw i8, ptr %0, i64 288, !dbg !27012
  %53 = getelementptr inbounds nuw i8, ptr %0, i64 382, !dbg !27012
  store i8 0, ptr %53, align 2, !dbg !27012
  %54 = getelementptr inbounds nuw i8, ptr %0, i64 376, !dbg !27012
  store i8 0, ptr %54, align 8, !dbg !27012
  %55 = getelementptr inbounds nuw i8, ptr %0, i64 112, !dbg !27012
  store i64 0, ptr %55, align 8, !dbg !27012
  %56 = getelementptr inbounds nuw i8, ptr %0, i64 144, !dbg !27012
  store i64 0, ptr %56, align 8, !dbg !27012
  %57 = getelementptr inbounds nuw i8, ptr %0, i64 377, !dbg !27012
  store i8 0, ptr %57, align 1, !dbg !27012
  %58 = getelementptr inbounds nuw i8, ptr %0, i64 168, !dbg !27012
  store i64 0, ptr %58, align 8, !dbg !27012
  %59 = getelementptr inbounds nuw i8, ptr %0, i64 373, !dbg !27012
  store i8 0, ptr %59, align 1, !dbg !27012
  %60 = getelementptr inbounds nuw i8, ptr %0, i64 192, !dbg !27012
  store i64 0, ptr %60, align 8, !dbg !27012
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(80) %52, i8 0, i64 80, i1 false), !dbg !27012
  call void @llvm.lifetime.end.p0(ptr nonnull %6), !dbg !27013
  ret void, !dbg !27013
}
