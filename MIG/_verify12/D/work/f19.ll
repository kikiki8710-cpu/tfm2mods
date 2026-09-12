define noundef range(i8 0, 6) i8 @_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old14passive_jungle16best_jungle_goal(i64 noundef %0, ptr noalias noundef align 16 dereferenceable(320) %1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(2528) %2, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %3, ptr noundef nonnull align 8 %4, i8 noundef range(i8 -1, 6) %5, ptr noalias noundef align 8 dereferenceable(224) %6) unnamed_addr #1 personality ptr @__CxxFrameHandler3 !dbg !71466 {
  %8 = alloca [40 x i8], align 8
  %9 = alloca [48 x i8], align 8
    #dbg_value(ptr poison, !71488, !DIExpression(), !71491)
    #dbg_value(ptr poison, !71495, !DIExpression(), !71497)
  %10 = alloca [72 x i8], align 8
  %11 = alloca [32 x i8], align 8
  %12 = alloca [4 x i8], align 1
  %13 = alloca [8 x i8], align 8
  store i64 %0, ptr %13, align 8
    #dbg_value(i8 %5, !71475, !DIExpression(), !71498)
    #dbg_declare(ptr %13, !71470, !DIExpression(), !71499)
    #dbg_value(ptr %1, !71471, !DIExpression(), !71498)
    #dbg_value(ptr %2, !71472, !DIExpression(), !71498)
    #dbg_value(ptr %3, !71473, !DIExpression(), !71498)
    #dbg_value(ptr %4, !71474, !DIExpression(), !71498)
    #dbg_value(ptr %6, !71476, !DIExpression(), !71498)
    #dbg_declare(ptr %12, !71477, !DIExpression(), !71500)
    #dbg_declare(ptr %12, !71501, !DIExpression(), !71526)
    #dbg_declare(ptr %12, !71501, !DIExpression(), !71528)
    #dbg_declare(ptr %11, !71480, !DIExpression(), !71530)
    #dbg_declare(ptr poison, !71531, !DIExpression(), !71564)
  call void @llvm.lifetime.start.p0(ptr nonnull %12), !dbg !71565
  store i8 0, ptr %12, align 1, !dbg !71565
  %14 = getelementptr inbounds nuw i8, ptr %12, i64 1, !dbg !71565
  store i8 1, ptr %14, align 1, !dbg !71565
  %15 = getelementptr inbounds nuw i8, ptr %12, i64 2, !dbg !71565
  store i8 3, ptr %15, align 1, !dbg !71565
  %16 = getelementptr inbounds nuw i8, ptr %12, i64 3, !dbg !71565
  store i8 2, ptr %16, align 1, !dbg !71565
  call void @llvm.lifetime.start.p0(ptr nonnull %11), !dbg !71566
  call void @llvm.lifetime.start.p0(ptr nonnull %10), !dbg !71527
  %17 = load i32, ptr %12, align 1, !dbg !71567
    #dbg_value(i64 0, !71568, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !71587)
    #dbg_value(i64 0, !71589, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !71606)
    #dbg_value(i64 4, !71568, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !71587)
    #dbg_value(i64 4, !71589, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !71606)
    #dbg_value(i32 %17, !71568, !DIExpression(DW_OP_LLVM_fragment, 128, 32), !71587)
    #dbg_value(i32 %17, !71589, !DIExpression(DW_OP_LLVM_fragment, 128, 32), !71606)
    #dbg_value(i64 0, !71560, !DIExpression(DW_OP_LLVM_fragment, 384, 64), !71608)
    #dbg_value(i64 4, !71560, !DIExpression(DW_OP_LLVM_fragment, 448, 64), !71608)
    #dbg_value(i32 %17, !71560, !DIExpression(DW_OP_LLVM_fragment, 512, 32), !71608)
    #dbg_value(i32 undef, !71560, !DIExpression(DW_OP_LLVM_fragment, 544, 32), !71608)
    #dbg_value(ptr %2, !71560, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !71608)
    #dbg_value(ptr %13, !71560, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !71608)
    #dbg_value(ptr %1, !71560, !DIExpression(DW_OP_LLVM_fragment, 128, 64), !71608)
    #dbg_value(ptr %3, !71560, !DIExpression(DW_OP_LLVM_fragment, 192, 64), !71608)
    #dbg_value(ptr %4, !71560, !DIExpression(DW_OP_LLVM_fragment, 256, 64), !71608)
    #dbg_value(ptr %6, !71560, !DIExpression(DW_OP_LLVM_fragment, 320, 64), !71608)
  store ptr %2, ptr %10, align 8, !dbg !71609
  %18 = getelementptr inbounds nuw i8, ptr %10, i64 8, !dbg !71609
  store ptr %13, ptr %18, align 8, !dbg !71609
  %19 = getelementptr inbounds nuw i8, ptr %10, i64 16, !dbg !71609
  store ptr %1, ptr %19, align 8, !dbg !71609
  %20 = getelementptr inbounds nuw i8, ptr %10, i64 24, !dbg !71609
  store ptr %3, ptr %20, align 8, !dbg !71609
  %21 = getelementptr inbounds nuw i8, ptr %10, i64 32, !dbg !71609
  store ptr %4, ptr %21, align 8, !dbg !71609
  %22 = getelementptr inbounds nuw i8, ptr %10, i64 40, !dbg !71609
  store ptr %6, ptr %22, align 8, !dbg !71609
  %23 = getelementptr inbounds nuw i8, ptr %10, i64 48, !dbg !71609
  store i64 0, ptr %23, align 8, !dbg !71609
  %24 = getelementptr inbounds nuw i8, ptr %10, i64 56, !dbg !71609
  store i64 4, ptr %24, align 8, !dbg !71609
  %25 = getelementptr inbounds nuw i8, ptr %10, i64 64, !dbg !71609
  store i32 %17, ptr %25, align 8, !dbg !71609
  %26 = getelementptr inbounds nuw i8, ptr %3, i64 8, !dbg !71527
  %27 = load ptr, ptr %26, align 8, !dbg !71527, !nonnull !8, !align !14311, !noundef !8
  %28 = load ptr, ptr %27, align 8, !dbg !71527, !nonnull !8, !align !14311, !noundef !8
  call void @_RINvMNtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB3_3VecNtNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6jungle10JungleTypeE12from_iter_inINtNtNtNtCsjihNppCmMEE_4core4iter8adapters3map3MapINtNtB2m_6filter6FilterINtNtNtB2q_5array4iter8IntoIterBU_Kj4_ENCNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old14passive_jungle16best_jungle_goal0ENCB47_s_0EEB4f_(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %11, ptr noalias noundef nonnull align 8 captures(address) dereferenceable(72) %10, ptr noundef nonnull align 8 %28), !dbg !71566
  call void @llvm.lifetime.end.p0(ptr nonnull %10), !dbg !71527
    #dbg_value(ptr %11, !71613, !DIExpression(), !71620)
    #dbg_value(ptr %11, !71622, !DIExpression(), !71628)
  %29 = getelementptr inbounds nuw i8, ptr %11, i64 24, !dbg !71630
  %30 = load i64, ptr %29, align 8, !dbg !71630, !noundef !8
  %31 = icmp eq i64 %30, 0, !dbg !71621
  br i1 %31, label %32, label %62, !dbg !71621

32:                                               ; preds = %7
  %33 = load ptr, ptr %3, align 8, !dbg !71529, !nonnull !8, !align !14311, !noundef !8
  %34 = load ptr, ptr %33, align 8, !dbg !71529, !nonnull !8, !noundef !8
  %35 = getelementptr inbounds nuw i8, ptr %33, i64 8, !dbg !71529
  %36 = load ptr, ptr %35, align 8, !dbg !71529, !nonnull !8, !align !14311, !noundef !8
    #dbg_value(ptr %34, !71577, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !71587)
    #dbg_value(ptr %34, !71603, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !71606)
    #dbg_value(ptr %36, !71577, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !71587)
    #dbg_value(ptr %36, !71603, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !71606)
    #dbg_value(ptr %2, !71577, !DIExpression(DW_OP_LLVM_fragment, 128, 64), !71587)
    #dbg_value(ptr %2, !71603, !DIExpression(DW_OP_LLVM_fragment, 128, 64), !71606)
    #dbg_declare(ptr %9, !71631, !DIExpression(), !71644)
    #dbg_declare(ptr poison, !71640, !DIExpression(), !71644)
  call void @llvm.lifetime.start.p0(ptr nonnull %9), !dbg !71646, !noalias !71647
  call void @llvm.experimental.noalias.scope.decl(metadata !71651), !dbg !71646
  call void @llvm.experimental.noalias.scope.decl(metadata !71654), !dbg !71646
  %37 = getelementptr inbounds nuw i8, ptr %9, i64 24, !dbg !71656
  %38 = getelementptr inbounds nuw i8, ptr %9, i64 32, !dbg !71656
  store i64 4, ptr %38, align 8, !dbg !71656, !alias.scope !71660, !noalias !71662
  %39 = getelementptr inbounds nuw i8, ptr %9, i64 40, !dbg !71656
  store i32 %17, ptr %39, align 8, !dbg !71656, !alias.scope !71660, !noalias !71662
  store ptr %34, ptr %9, align 8, !dbg !71656, !alias.scope !71663, !noalias !71664
  %40 = getelementptr inbounds nuw i8, ptr %9, i64 8, !dbg !71656
  store ptr %36, ptr %40, align 8, !dbg !71656, !alias.scope !71663, !noalias !71664
  %41 = getelementptr inbounds nuw i8, ptr %9, i64 16, !dbg !71656
  store ptr %2, ptr %41, align 8, !dbg !71656, !alias.scope !71663, !noalias !71664
  call void @llvm.experimental.noalias.scope.decl(metadata !71665), !dbg !71668
    #dbg_declare(ptr %9, !71669, !DIExpression(), !71686)
    #dbg_declare(ptr poison, !71677, !DIExpression(), !71686)
  call void @llvm.experimental.noalias.scope.decl(metadata !71688), !dbg !71691
    #dbg_value(ptr %9, !71692, !DIExpression(), !71699)
    #dbg_value(ptr %37, !71701, !DIExpression(), !71707)
    #dbg_value(ptr %37, !71709, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !71726)
    #dbg_value(i64 4, !71709, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !71726)
    #dbg_value(ptr %37, !71728, !DIExpression(), !71731)
    #dbg_value(ptr %37, !71733, !DIExpression(), !71738)
    #dbg_value(i64 0, !71736, !DIExpression(), !71740)
  store i64 1, ptr %37, align 8, !dbg !71741, !alias.scope !71742, !noalias !71647
    #dbg_value(i64 0, !71747, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !71767)
    #dbg_value(i64 1, !71747, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !71767)
    #dbg_value(ptr %39, !71764, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !71767)
    #dbg_value(i64 4, !71764, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !71767)
    #dbg_value(i64 0, !71765, !DIExpression(), !71768)
    #dbg_value(i64 0, !71769, !DIExpression(), !71775)
    #dbg_value(ptr %39, !71774, !DIExpression(DW_OP_deref), !71775)
    #dbg_value(ptr %39, !71777, !DIExpression(), !71785)
    #dbg_value(ptr %39, !71787, !DIExpression(), !71792)
    #dbg_value(ptr %39, !71794, !DIExpression(), !71797)
  %42 = trunc i32 %17 to i8, !dbg !71799
    #dbg_value(i8 %42, !71800, !DIExpression(), !71813)
    #dbg_value(ptr %9, !71810, !DIExpression(), !71813)
    #dbg_value(ptr %9, !71815, !DIExpression(), !71826)
    #dbg_value(i8 %42, !71811, !DIExpression(), !71828)
    #dbg_value(i8 %42, !71820, !DIExpression(), !71826)
    #dbg_value(i8 %42, !71829, !DIExpression(), !71835)
    #dbg_value(ptr poison, !71832, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !71835)
    #dbg_value(ptr poison, !71837, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !71847)
    #dbg_value(ptr poison, !71843, !DIExpression(), !71847)
  %43 = getelementptr inbounds nuw i8, ptr %36, i64 64, !dbg !71849
  %44 = load ptr, ptr %43, align 8, !dbg !71849, !invariant.load !8, !noalias !71850, !nonnull !8
  %45 = invoke { i64, ptr } %44(ptr noundef nonnull %34)
          to label %46 unwind label %66, !dbg !71849

46:                                               ; preds = %32
  %47 = extractvalue { i64, ptr } %45, 0, !dbg !71849
    #dbg_value(i64 %47, !71855, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !71860)
    #dbg_value(ptr poison, !71855, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !71860)
  %48 = icmp eq i64 %47, 0, !dbg !71862
  br i1 %48, label %51, label %49, !dbg !71862, !prof !42356

49:                                               ; preds = %46
  invoke void @_RNvNtCsjihNppCmMEE_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @anon.168add0ea037d45d276f5936ae758fe5.42) #30
          to label %50 unwind label %66, !dbg !71863

50:                                               ; preds = %49
  unreachable, !dbg !71863

51:                                               ; preds = %46
  %52 = extractvalue { i64, ptr } %45, 1, !dbg !71849
    #dbg_value(ptr %52, !71855, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !71860)
  %53 = icmp ne ptr %52, null
  call void @llvm.assume(i1 %53)
    #dbg_value(ptr %52, !71866, !DIExpression(), !71869)
    #dbg_value(ptr %52, !71870, !DIExpression(), !71873)
  %54 = getelementptr inbounds nuw i8, ptr %52, i64 24, !dbg !71874
  %55 = getelementptr inbounds nuw i8, ptr %2, i64 2352, !dbg !71849
  %56 = load i64, ptr %55, align 8, !dbg !71849, !noalias !71850, !noundef !8
  %57 = invoke noundef nonnull align 8 ptr @_RNvMs1_NtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6jungleNtB5_12JungleRunner14get_camp_state(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(480) %54, i64 noundef %56, i8 noundef range(i8 0, 6) %42)
          to label %58 unwind label %66, !dbg !71849

58:                                               ; preds = %51
  %59 = getelementptr inbounds nuw i8, ptr %57, i64 24, !dbg !71849
  %60 = load i64, ptr %59, align 8, !dbg !71849, !noalias !71850, !noundef !8
    #dbg_value(i64 %60, !71678, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !71875)
    #dbg_value(i8 %42, !71678, !DIExpression(DW_OP_LLVM_fragment, 64, 8), !71875)
  %61 = invoke { i64, i8 } @_RINvXs0_NtNtNtCsjihNppCmMEE_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5array4iter8IntoIterNtNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6jungle10JungleTypeKj4_ENCINvNvNtNtNtBa_6traits8iterator8Iterator10min_by_key3keyB1r_jNCNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old14passive_jungle16best_jungle_goals0_0E0EB2L_4foldTjB1r_ENCINvNvB2L_6min_by4foldB5f_INvB2J_7compareB1r_jEE0EB3O_(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(48) %9, i64 noundef %60, i8 noundef %42)
          to label %69 unwind label %66, !dbg !71876

62:                                               ; preds = %7
  %63 = getelementptr inbounds nuw i8, ptr %2, i64 2352, !dbg !71877
  %64 = load i64, ptr %63, align 8, !dbg !71877, !noundef !8
  %65 = icmp ult i64 %64, 2, !dbg !71877
  br i1 %65, label %78, label %77, !dbg !71877

66:                                               ; preds = %145, %135, %130, %126, %125, %77, %75, %58, %51, %49, %32
  %67 = phi i1 [ true, %75 ], [ true, %58 ], [ true, %145 ], [ true, %135 ], [ false, %130 ], [ false, %126 ], [ true, %51 ], [ true, %77 ], [ true, %32 ], [ true, %49 ], [ false, %125 ], !dbg !71878
  %68 = cleanuppad within none []
  br i1 %67, label %147, label %146, !dbg !71879

69:                                               ; preds = %58
  %70 = extractvalue { i64, i8 } %61, 1, !dbg !71876
  call void @llvm.lifetime.end.p0(ptr nonnull %9), !dbg !71646, !noalias !71647
    #dbg_value(i8 %70, !71880, !DIExpression(), !71890)
  %71 = icmp eq i8 %70, -1, !dbg !71892
  br i1 %71, label %75, label %72, !dbg !71892, !prof !15655

72:                                               ; preds = %69
    #dbg_value(i8 %70, !71484, !DIExpression(), !71893)
    #dbg_value(ptr %11, !13113, !DIExpression(), !71894)
  invoke void @_RNvXsu_NtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB5_3VecNtNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6jungle10JungleTypeENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr noalias noundef nonnull align 8 dereferenceable(32) %11)
          to label %131 unwind label %73, !dbg !71896

73:                                               ; preds = %72
  %74 = cleanuppad within none []
    #dbg_value(ptr %11, !13118, !DIExpression(), !71897)
  call void @_RNvXs4_NtNtCshWfHDMLkPaX_7bumpalo11collections7raw_vecINtB5_6RawVecNtNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6jungle10JungleTypeENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr noalias noundef nonnull align 8 dereferenceable(32) %11) [ "funclet"(token %74) ], !dbg !71899
  cleanupret from %74 unwind to caller, !dbg !71896

75:                                               ; preds = %69
  invoke void @_RNvNtCsjihNppCmMEE_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @anon.168add0ea037d45d276f5936ae758fe5.269) #30
          to label %76 unwind label %66, !dbg !71900

76:                                               ; preds = %145, %130, %77, %75
  unreachable

77:                                               ; preds = %62
  invoke void @_RNvNtCsjihNppCmMEE_4core9panicking18panic_bounds_check(i64 noundef %64, i64 noundef 2, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @anon.168add0ea037d45d276f5936ae758fe5.270) #30
          to label %76 unwind label %66, !dbg !71877

78:                                               ; preds = %62
    #dbg_value(ptr %2, !71901, !DIExpression(DW_OP_plus_uconst, 2496, DW_OP_stack_value), !71904)
  %79 = getelementptr inbounds nuw i8, ptr %2, i64 2496, !dbg !71905
  %80 = load i32, ptr %79, align 8, !dbg !71905, !range !28546, !noundef !8
  %81 = zext nneg i32 %80 to i64, !dbg !71905
  %82 = load ptr, ptr %3, align 8, !dbg !71877, !nonnull !8, !align !14311, !noundef !8
  %83 = getelementptr inbounds nuw i8, ptr %82, i64 480, !dbg !71877
  %84 = getelementptr inbounds nuw [5 x ptr], ptr %83, i64 %64, !dbg !71877
  %85 = getelementptr inbounds nuw ptr, ptr %84, i64 %81, !dbg !71877
  %86 = load ptr, ptr %85, align 8, !dbg !71877, !align !14311, !noundef !8
    #dbg_value(ptr %86, !71482, !DIExpression(), !71906)
    #dbg_value(ptr undef, !71495, !DIExpression(), !71497)
    #dbg_value(ptr undef, !71488, !DIExpression(), !71491)
  %87 = icmp eq ptr %86, null, !dbg !71907
  br i1 %87, label %88, label %90, !dbg !71496

88:                                               ; preds = %78
  %89 = icmp eq i8 %5, -1, !dbg !71908
  br i1 %89, label %135, label %137, !dbg !71908

90:                                               ; preds = %78
    #dbg_value(ptr %86, !71486, !DIExpression(), !71909)
  %91 = load ptr, ptr %11, align 8, !dbg !71910, !nonnull !8, !noundef !8
    #dbg_declare(ptr poison, !71911, !DIExpression(), !71934)
    #dbg_value(ptr %91, !71929, !DIExpression(), !71936)
    #dbg_value(ptr %91, !71937, !DIExpression(), !71943)
    #dbg_value(i64 %30, !71942, !DIExpression(), !71943)
  %92 = getelementptr inbounds nuw i8, ptr %91, i64 %30, !dbg !71945
  %93 = getelementptr inbounds nuw i8, ptr %27, i64 32, !dbg !71910
  %94 = load ptr, ptr %93, align 8, !dbg !71910, !nonnull !8, !align !14311, !noundef !8
    #dbg_value(ptr %94, !71946, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !71966)
    #dbg_value(ptr %94, !71968, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !71986)
    #dbg_value(ptr %2, !71946, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !71966)
    #dbg_value(ptr %2, !71968, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !71986)
    #dbg_value(ptr %86, !71946, !DIExpression(DW_OP_LLVM_fragment, 128, 64), !71966)
    #dbg_value(ptr %86, !71968, !DIExpression(DW_OP_LLVM_fragment, 128, 64), !71986)
    #dbg_value(ptr %91, !71956, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !71966)
    #dbg_value(ptr %92, !71956, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !71966)
    #dbg_declare(ptr %8, !71988, !DIExpression(), !72001)
    #dbg_declare(ptr poison, !71997, !DIExpression(), !72001)
  call void @llvm.lifetime.start.p0(ptr nonnull %8), !dbg !72003, !noalias !72004
  call void @llvm.experimental.noalias.scope.decl(metadata !72007), !dbg !72003
    #dbg_value(ptr %91, !71983, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !71986)
    #dbg_value(ptr %92, !71983, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !71986)
  %95 = getelementptr inbounds nuw i8, ptr %8, i64 24, !dbg !72010
  %96 = getelementptr inbounds nuw i8, ptr %8, i64 32, !dbg !72010
  store ptr %92, ptr %96, align 8, !dbg !72010, !alias.scope !72014, !noalias !72016
  store ptr %94, ptr %8, align 8, !dbg !72010, !alias.scope !72017
  %97 = getelementptr inbounds nuw i8, ptr %8, i64 8, !dbg !72010
  store ptr %2, ptr %97, align 8, !dbg !72010, !alias.scope !72017
  %98 = getelementptr inbounds nuw i8, ptr %8, i64 16, !dbg !72010
  store ptr %86, ptr %98, align 8, !dbg !72010, !alias.scope !72017
  call void @llvm.experimental.noalias.scope.decl(metadata !72018), !dbg !72021
    #dbg_declare(ptr %8, !72022, !DIExpression(), !72039)
    #dbg_declare(ptr poison, !72030, !DIExpression(), !72039)
  call void @llvm.experimental.noalias.scope.decl(metadata !72041), !dbg !72044
    #dbg_value(ptr %8, !72045, !DIExpression(), !72052)
  call void @llvm.experimental.noalias.scope.decl(metadata !72054), !dbg !72057
    #dbg_value(ptr %95, !72058, !DIExpression(), !72066)
    #dbg_value(i64 1, !72068, !DIExpression(), !72074)
    #dbg_value(ptr %91, !72064, !DIExpression(), !72076)
    #dbg_value(ptr %91, !72077, !DIExpression(), !72080)
    #dbg_value(ptr %91, !72073, !DIExpression(), !72074)
  %99 = getelementptr inbounds nuw i8, ptr %91, i64 1, !dbg !72082
  store ptr %99, ptr %95, align 8, !dbg !72083, !alias.scope !72084, !noalias !72004
  %100 = load i8, ptr %91, align 1, !dbg !72085, !range !41907, !noalias !72086, !noundef !8
    #dbg_value(i8 %100, !72087, !DIExpression(), !72100)
    #dbg_value(ptr %8, !72097, !DIExpression(), !72100)
    #dbg_value(ptr %8, !72102, !DIExpression(), !72109)
    #dbg_value(i8 %100, !72098, !DIExpression(), !72111)
    #dbg_value(i8 %100, !72107, !DIExpression(), !72109)
    #dbg_value(i8 %100, !72112, !DIExpression(), !72118)
    #dbg_value(ptr poison, !72115, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !72118)
    #dbg_value(ptr poison, !72120, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !72131)
    #dbg_value(ptr poison, !72127, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !72131)
    #dbg_value(ptr poison, !72128, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !72131)
    #dbg_value(ptr poison, !72126, !DIExpression(), !72131)
  %101 = icmp eq i64 %64, 0, !dbg !72133
  %102 = invoke { i64, i64 } @_RNvMNtNtCs97f5S1uJLkH_9game_core10simulation7map_defNtB2_6MapDef8camp_pos(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(28112) %94, i8 noundef range(i8 0, 6) %100, i1 noundef zeroext %101)
          to label %106 unwind label %103, !dbg !72133, !noalias !72134

103:                                              ; preds = %106, %90
  %104 = phi i1 [ false, %106 ], [ true, %90 ], !dbg !72135
  %105 = cleanuppad within none []
  br i1 %104, label %126, label %125, !dbg !72136

106:                                              ; preds = %90
  %107 = extractvalue { i64, i64 } %102, 0, !dbg !72133
  %108 = extractvalue { i64, i64 } %102, 1, !dbg !72133
    #dbg_value(i64 %107, !72129, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !72137)
    #dbg_value(i64 %108, !72129, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !72137)
  %109 = getelementptr inbounds nuw i8, ptr %86, i64 1632, !dbg !72138
  %110 = load i64, ptr %109, align 8, !dbg !72138, !noalias !72139, !noundef !8
    #dbg_value(i64 %110, !72144, !DIExpression(), !72154)
    #dbg_value(i64 %110, !72156, !DIExpression(), !72160)
  %111 = getelementptr inbounds nuw i8, ptr %86, i64 1640, !dbg !72138
  %112 = load i64, ptr %111, align 8, !dbg !72138, !noalias !72139, !noundef !8
    #dbg_value(i64 %112, !72147, !DIExpression(), !72154)
    #dbg_value(i64 %112, !72156, !DIExpression(), !72162)
    #dbg_value(i64 %107, !72148, !DIExpression(), !72154)
    #dbg_value(i64 %107, !72159, !DIExpression(), !72160)
    #dbg_value(i64 %108, !72149, !DIExpression(), !72154)
    #dbg_value(i64 %108, !72159, !DIExpression(), !72162)
  %113 = icmp ult i64 %110, %107, !dbg !72164
  %114 = sub nuw i64 %107, %110, !dbg !72164
  %115 = sub nuw i64 %110, %107, !dbg !72164
  %116 = select i1 %113, i64 %114, i64 %115, !dbg !72164
    #dbg_value(i64 %116, !72150, !DIExpression(), !72165)
  %117 = icmp ult i64 %112, %108, !dbg !72166
  %118 = sub nuw i64 %108, %112, !dbg !72166
  %119 = sub nuw i64 %112, %108, !dbg !72166
  %120 = select i1 %117, i64 %118, i64 %119, !dbg !72166
    #dbg_value(i64 %120, !72152, !DIExpression(), !72167)
  %121 = mul i64 %116, %116, !dbg !72168
  %122 = mul i64 %120, %120, !dbg !72168
  %123 = add i64 %122, %121, !dbg !72168
    #dbg_value(i64 %123, !72031, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !72169)
    #dbg_value(i8 %100, !72031, !DIExpression(DW_OP_LLVM_fragment, 64, 8), !72169)
  %124 = invoke { i64, i8 } @_RINvXs0_NtNtNtCsjihNppCmMEE_4core4iter8adapters3mapINtB6_3MapINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec8IntoIterNtNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6jungle10JungleTypeENCINvNvNtNtNtBa_6traits8iterator8Iterator10min_by_key3keyB1Q_yNCNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old14passive_jungle16best_jungle_goals1_0E0EB36_4foldTyB1Q_ENCINvNvB36_6min_by4foldB5A_INvB34_7compareB1Q_yEE0EB49_(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(40) %8, i64 noundef %123, i8 noundef %100)
          to label %127 unwind label %103, !dbg !72170, !noalias !72004

125:                                              ; preds = %126, %103
  cleanupret from %105 unwind label %66

126:                                              ; preds = %103
    #dbg_value(ptr %8, !72171, !DIExpression(), !72178)
    #dbg_value(ptr %95, !72180, !DIExpression(), !72187)
  invoke void @_RNvXsD_NtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB5_8IntoIterNtNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6jungle10JungleTypeENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr noalias noundef nonnull align 8 dereferenceable(16) %95) [ "funclet"(token %105) ]
          to label %125 unwind label %66, !dbg !72189

127:                                              ; preds = %106
  %128 = extractvalue { i64, i8 } %124, 1, !dbg !72170
  call void @llvm.lifetime.end.p0(ptr nonnull %8), !dbg !72003, !noalias !72004
    #dbg_value(i8 %128, !71880, !DIExpression(), !72190)
  %129 = icmp eq i8 %128, -1, !dbg !72192
  br i1 %129, label %130, label %133, !dbg !72192, !prof !29922

130:                                              ; preds = %127
  invoke void @_RNvNtCsjihNppCmMEE_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @anon.168add0ea037d45d276f5936ae758fe5.271) #30
          to label %76 unwind label %66, !dbg !72193

131:                                              ; preds = %137, %72
  %132 = phi i8 [ %138, %137 ], [ %70, %72 ]
  call void @_RNvXs4_NtNtCshWfHDMLkPaX_7bumpalo11collections7raw_vecINtB5_6RawVecNtNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6jungle10JungleTypeENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr noalias noundef nonnull align 8 dereferenceable(32) %11), !dbg !72194
  br label %133, !dbg !71879

133:                                              ; preds = %131, %127
  %134 = phi i8 [ %128, %127 ], [ %132, %131 ], !dbg !72196
  call void @llvm.lifetime.end.p0(ptr nonnull %11), !dbg !71879
  call void @llvm.lifetime.end.p0(ptr nonnull %12), !dbg !72197
    #dbg_value(i8 %134, !71484, !DIExpression(), !71893)
  ret i8 %134, !dbg !72197

135:                                              ; preds = %88
  %136 = invoke noundef ptr @_RINvXNtCsMBkRBYhlca_4rand3seqSNtNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6jungle10JungleTypeNtB3_11SliceRandom6chooseNtNtNtB5_4rngs3std6StdRngECshdEBA0ozCnw_7game_ai(ptr noalias noundef nonnull readonly captures(address, read_provenance) %12, i64 noundef 4, ptr noalias noundef nonnull align 16 dereferenceable(320) %1)
          to label %141 unwind label %66, !dbg !72198

137:                                              ; preds = %143, %88
  %138 = phi i8 [ %144, %143 ], [ %5, %88 ], !dbg !71906
    #dbg_value(i8 %138, !71484, !DIExpression(), !71893)
    #dbg_value(ptr %11, !13113, !DIExpression(), !72199)
  invoke void @_RNvXsu_NtNtCshWfHDMLkPaX_7bumpalo11collections3vecINtB5_3VecNtNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6jungle10JungleTypeENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr noalias noundef nonnull align 8 dereferenceable(32) %11)
          to label %131 unwind label %139, !dbg !72201

139:                                              ; preds = %137
  %140 = cleanuppad within none []
    #dbg_value(ptr %11, !13118, !DIExpression(), !72202)
  call void @_RNvXs4_NtNtCshWfHDMLkPaX_7bumpalo11collections7raw_vecINtB5_6RawVecNtNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6jungle10JungleTypeENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr noalias noundef nonnull align 8 dereferenceable(32) %11) [ "funclet"(token %140) ], !dbg !72204
  cleanupret from %140 unwind to caller, !dbg !72201

141:                                              ; preds = %135
    #dbg_value(ptr %136, !72205, !DIExpression(), !72210)
  %142 = icmp eq ptr %136, null, !dbg !72211
  br i1 %142, label %145, label %143, !dbg !72211, !prof !15655

143:                                              ; preds = %141
    #dbg_value(ptr %136, !72212, !DIExpression(), !72219)
  %144 = load i8, ptr %136, align 1, !dbg !72220, !range !41907, !noundef !8
    #dbg_value(i8 %144, !71484, !DIExpression(), !71893)
  br label %137, !dbg !72221

145:                                              ; preds = %141
  invoke void @_RNvNtCsjihNppCmMEE_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @anon.168add0ea037d45d276f5936ae758fe5.272) #30
          to label %76 unwind label %66, !dbg !72223

146:                                              ; preds = %147, %66
  cleanupret from %68 unwind to caller, !dbg !71499

147:                                              ; preds = %66
  call fastcc void @_RINvNtCsjihNppCmMEE_4core3ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6jungle10JungleTypeEECshdEBA0ozCnw_7game_ai(ptr noalias noundef align 8 dereferenceable(32) %11) #31 [ "funclet"(token %68) ], !dbg !71879
  br label %146, !dbg !71879
}
