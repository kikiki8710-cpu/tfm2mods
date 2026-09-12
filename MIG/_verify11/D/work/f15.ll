define internal fastcc void @_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler5modesNtB4_17LegacyPlanHandler17single_try_engage(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(144) %0, ptr noundef nonnull align 8 %1, i64 noundef %2, ptr noalias noundef nonnull align 16 dereferenceable(320) %3, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %4, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %5, i64 noundef %6, ptr noalias noundef nonnull align 8 dereferenceable(224) %7) unnamed_addr #0 personality ptr @__CxxFrameHandler3 !dbg !38540 {
  %9 = alloca [128 x i8], align 8
  %10 = alloca [128 x i8], align 8
  %11 = alloca [24 x i8], align 8
  %12 = alloca [120 x i8], align 8
  %13 = alloca [24 x i8], align 8
  %14 = alloca [144 x i8], align 8
  %15 = alloca [144 x i8], align 8
    #dbg_value(ptr %1, !38545, !DIExpression(), !38566)
    #dbg_value(i64 %2, !38546, !DIExpression(), !38566)
    #dbg_value(ptr %3, !38547, !DIExpression(), !38566)
    #dbg_value(ptr %4, !38548, !DIExpression(), !38566)
    #dbg_value(ptr %5, !38549, !DIExpression(), !38566)
    #dbg_value(i64 %6, !38550, !DIExpression(), !38566)
    #dbg_value(ptr %7, !38551, !DIExpression(), !38566)
    #dbg_declare(ptr %15, !38554, !DIExpression(), !38567)
    #dbg_declare(ptr %14, !38562, !DIExpression(), !38568)
    #dbg_declare(ptr poison, !38569, !DIExpression(), !38582)
  %16 = load ptr, ptr %5, align 8, !dbg !38584, !nonnull !8, !align !11920, !noundef !8
  %17 = load ptr, ptr %16, align 8, !dbg !38584, !nonnull !8, !noundef !8
  %18 = getelementptr inbounds nuw i8, ptr %16, i64 8, !dbg !38584
  %19 = load ptr, ptr %18, align 8, !dbg !38584, !nonnull !8, !align !11920, !noundef !8
  %20 = getelementptr inbounds nuw i8, ptr %19, i64 496, !dbg !38584
  %21 = load ptr, ptr %20, align 8, !dbg !38584, !invariant.load !8, !nonnull !8
  %22 = tail call noundef align 8 ptr %21(ptr noundef nonnull %17, i64 noundef %6), !dbg !38584
    #dbg_value(ptr %22, !38585, !DIExpression(), !38600)
    #dbg_value(ptr %4, !38597, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !38600)
    #dbg_value(ptr %5, !38597, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !38600)
  %23 = icmp eq ptr %22, null, !dbg !38601
  br i1 %23, label %26, label %24, !dbg !38601

24:                                               ; preds = %8
    #dbg_value(ptr %22, !38598, !DIExpression(), !38602)
    #dbg_value(ptr poison, !38603, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !38610)
    #dbg_value(ptr poison, !38603, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 64, 64), !38610)
    #dbg_value(ptr poison, !38609, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !38610)
    #dbg_value(ptr %22, !38608, !DIExpression(), !38610)
  %25 = tail call noundef zeroext i1 @_RNvNtCshdEBA0ozCnw_7game_ai16tower_discipline20engage_requires_dive(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %4, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %5, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %22), !dbg !38612
    #dbg_value(i1 %25, !38552, !DIExpression(DW_OP_LLVM_convert, 1, DW_ATE_unsigned, DW_OP_LLVM_convert, 8, DW_ATE_unsigned, DW_OP_stack_value), !38613)
  call void @llvm.lifetime.start.p0(ptr nonnull %15), !dbg !38614
  br i1 %25, label %30, label %27, !dbg !38614

26:                                               ; preds = %8
    #dbg_value(i8 0, !38552, !DIExpression(), !38613)
  call void @llvm.lifetime.start.p0(ptr nonnull %15), !dbg !38614
  br label %27, !dbg !38614

27:                                               ; preds = %26, %24
  call void @llvm.lifetime.start.p0(ptr nonnull %11), !dbg !38615
  %28 = getelementptr inbounds nuw i8, ptr %11, i64 8, !dbg !38615
  store i64 %6, ptr %28, align 8, !dbg !38615
  %29 = getelementptr inbounds nuw i8, ptr %11, i64 16, !dbg !38615
  store i64 60, ptr %29, align 8, !dbg !38615
  store i64 0, ptr %11, align 8, !dbg !38615
  call void @_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13single_battleNtB2_16SinglePlanBattle3new(ptr noalias noundef nonnull sret([144 x i8]) align 8 captures(none) dereferenceable(144) %15, i64 noundef %2, ptr noalias noundef nonnull readonly align 8 captures(address) dereferenceable(24) %11, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %5, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %4), !dbg !38615
  call void @llvm.lifetime.end.p0(ptr nonnull %11), !dbg !38615
  br label %33, !dbg !38614

30:                                               ; preds = %24
  %31 = tail call noundef align 8 ptr %21(ptr noundef nonnull %17, i64 noundef %6), !dbg !38616
    #dbg_value(ptr %31, !38617, !DIExpression(), !38622)
  %32 = icmp eq ptr %31, null, !dbg !38623
  br i1 %32, label %39, label %36, !dbg !38623

33:                                               ; preds = %118, %27
  %34 = getelementptr inbounds nuw i8, ptr %1, i64 2448, !dbg !38624
  %35 = getelementptr inbounds nuw i8, ptr %1, i64 248, !dbg !38624
  invoke void @_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13single_battleNtB2_16SinglePlanBattle6update(ptr noalias noundef nonnull align 8 dereferenceable(144) %15, i64 noundef %2, ptr noalias noundef nonnull align 16 dereferenceable(320) %3, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %4, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %5, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2760) %34, ptr noundef nonnull align 8 %35, ptr noalias noundef nonnull align 8 dereferenceable(224) %7)
          to label %123 unwind label %121, !dbg !38624

36:                                               ; preds = %30
    #dbg_value(ptr %31, !38556, !DIExpression(), !38625)
  %37 = getelementptr inbounds nuw i8, ptr %1, i64 248, !dbg !38626
  %38 = tail call noundef zeroext i1 @_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13single_battle27single_tower_dive_is_viable(i64 noundef %2, ptr noalias noundef nonnull align 16 dereferenceable(320) %3, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %4, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %5, ptr noundef nonnull align 8 %37, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %31, ptr noalias noundef nonnull align 8 dereferenceable(224) %7), !dbg !38626
  br i1 %38, label %40, label %39, !dbg !38626

39:                                               ; preds = %36, %30
  store i64 -1, ptr %0, align 8, !dbg !38613
  br label %46, !dbg !38627

40:                                               ; preds = %36
  call void @llvm.lifetime.start.p0(ptr nonnull %14), !dbg !38628
  call void @llvm.lifetime.start.p0(ptr nonnull %13), !dbg !38628
  %41 = getelementptr inbounds nuw i8, ptr %13, i64 8, !dbg !38628
  store i64 %6, ptr %41, align 8, !dbg !38628
  %42 = getelementptr inbounds nuw i8, ptr %13, i64 16, !dbg !38628
  store i64 60, ptr %42, align 8, !dbg !38628
  store i64 0, ptr %13, align 8, !dbg !38628
  call void @_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13single_battleNtB2_16SinglePlanBattle8new_dive(ptr noalias noundef nonnull sret([144 x i8]) align 8 captures(none) dereferenceable(144) %14, i64 noundef %2, ptr noalias noundef nonnull readonly align 8 captures(address) dereferenceable(24) %13, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %5, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %4), !dbg !38628
  call void @llvm.lifetime.end.p0(ptr nonnull %13), !dbg !38628
  call void @llvm.lifetime.start.p0(ptr nonnull %12), !dbg !38629
  %43 = getelementptr inbounds nuw i8, ptr %4, i64 2352, !dbg !38629
  %44 = load i64, ptr %43, align 8, !dbg !38629, !noundef !8
  %45 = sub i64 1, %44, !dbg !38629
  invoke void @_RNvMs3_NtCs97f5S1uJLkH_9game_core10simulationNtB5_21AbstractGameWithCache25iter_towers_without_nexus(ptr noalias noundef nonnull sret([120 x i8]) align 8 captures(address) dereferenceable(120) %12, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8840) %16, i64 noundef %45)
          to label %49 unwind label %47, !dbg !38629

46:                                               ; preds = %131, %126, %39
  call void @llvm.lifetime.end.p0(ptr nonnull %15), !dbg !38630
  ret void, !dbg !38627

47:                                               ; preds = %84, %77, %40
  %48 = cleanuppad within none []
  call fastcc void @_RINvNtCsjihNppCmMEE_4core3ptr9drop_glueNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13single_battle16SinglePlanBattleEBJ_(ptr noalias noundef align 8 dereferenceable(144) %14) #31 [ "funclet"(token %48) ], !dbg !38631
  cleanupret from %48 unwind to caller, !dbg !38631

49:                                               ; preds = %40
    #dbg_declare(ptr %12, !38632, !DIExpression(), !38649)
    #dbg_value(ptr %31, !38640, !DIExpression(), !38651)
    #dbg_declare(ptr %10, !38652, !DIExpression(), !38669)
    #dbg_declare(ptr poison, !38666, !DIExpression(), !38669)
  call void @llvm.lifetime.start.p0(ptr nonnull %10), !dbg !38671, !noalias !38672
  call void @llvm.experimental.noalias.scope.decl(metadata !38676), !dbg !38671
    #dbg_declare(ptr %12, !38679, !DIExpression(), !38686)
    #dbg_value(ptr %31, !38684, !DIExpression(), !38688)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(128) %10, ptr noundef nonnull readonly align 8 dereferenceable(120) %12, i64 120, i1 false), !dbg !38689, !alias.scope !38693, !noalias !38695
  %50 = getelementptr inbounds nuw i8, ptr %10, i64 120, !dbg !38689
  store ptr %31, ptr %50, align 8, !dbg !38689, !alias.scope !38697, !noalias !38698
  call void @llvm.experimental.noalias.scope.decl(metadata !38699), !dbg !38702
    #dbg_declare(ptr %10, !38703, !DIExpression(), !38716)
    #dbg_declare(ptr poison, !38708, !DIExpression(), !38716)
    #dbg_value(ptr %10, !38718, !DIExpression(), !38725)
    #dbg_value(ptr %10, !14024, !DIExpression(), !38727)
    #dbg_value(ptr %10, !14034, !DIExpression(), !38729)
    #dbg_value(ptr %10, !14055, !DIExpression(), !38731)
    #dbg_declare(ptr poison, !14043, !DIExpression(), !38733)
  %51 = load i64, ptr %10, align 8, !dbg !38734, !range !14067, !alias.scope !38735, !noalias !38672, !noundef !8
  %52 = icmp eq i64 %51, -1, !dbg !38734
  br i1 %52, label %73, label %53, !dbg !38734

53:                                               ; preds = %49
    #dbg_value(ptr %10, !14075, !DIExpression(), !38742)
    #dbg_declare(ptr poison, !14081, !DIExpression(), !38744)
    #dbg_value(ptr %10, !14091, !DIExpression(), !38745)
    #dbg_value(ptr %10, !14098, !DIExpression(), !38747)
  %54 = getelementptr inbounds nuw i8, ptr %10, i64 8, !dbg !38749
  %55 = getelementptr inbounds nuw i8, ptr %10, i64 16, !dbg !38753
  %56 = trunc nuw i64 %51 to i1
  %57 = load i64, ptr %55, align 8, !alias.scope !38760, !noalias !38672
  %58 = getelementptr inbounds nuw i8, ptr %10, i64 24
  br i1 %56, label %59, label %72, !dbg !38767

59:                                               ; preds = %53
  %60 = load i64, ptr %54, align 1, !alias.scope !38760, !noalias !38672
    #dbg_value(ptr %10, !14138, !DIExpression(), !38768)
    #dbg_value(ptr %10, !14129, !DIExpression(), !38769)
    #dbg_value(ptr %10, !14118, !DIExpression(), !38770)
    #dbg_value(ptr %54, !14200, !DIExpression(), !38771)
    #dbg_value(ptr %54, !14192, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !38772)
    #dbg_value(i64 6, !14192, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !38772)
    #dbg_value(i64 1, !14215, !DIExpression(), !38773)
    #dbg_value(ptr %54, !14173, !DIExpression(), !38776)
    #dbg_value(ptr %54, !14154, !DIExpression(), !38777)
    #dbg_value(ptr %54, !14148, !DIExpression(), !38778)
    #dbg_value(ptr %54, !14247, !DIExpression(), !38779)
    #dbg_value(ptr poison, !14164, !DIExpression(), !38780)
  %61 = icmp eq i64 %57, %60, !dbg !38781
  br i1 %61, label %71, label %64, !dbg !38781

62:                                               ; preds = %64
    #dbg_value(ptr %10, !14138, !DIExpression(), !38768)
    #dbg_value(ptr %10, !14129, !DIExpression(), !38769)
    #dbg_value(ptr %10, !14118, !DIExpression(), !38770)
    #dbg_value(ptr %54, !14200, !DIExpression(), !38771)
    #dbg_value(ptr %54, !14192, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !38772)
    #dbg_value(i64 6, !14192, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !38772)
    #dbg_value(i64 1, !14215, !DIExpression(), !38773)
    #dbg_value(ptr %54, !14173, !DIExpression(), !38776)
    #dbg_value(ptr %54, !14154, !DIExpression(), !38777)
    #dbg_value(ptr %54, !14148, !DIExpression(), !38778)
    #dbg_value(ptr %54, !14247, !DIExpression(), !38779)
    #dbg_value(ptr poison, !14164, !DIExpression(), !38780)
  %63 = icmp eq i64 %57, %66, !dbg !38781
  br i1 %63, label %71, label %64, !dbg !38781

64:                                               ; preds = %62, %59
  %65 = phi i64 [ %66, %62 ], [ %60, %59 ]
    #dbg_value(i64 %65, !14248, !DIExpression(), !38782)
    #dbg_value(i64 %65, !14238, !DIExpression(), !38773)
  %66 = add nuw nsw i64 %65, 1, !dbg !38783
    #dbg_value(i64 %65, !14258, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !38784)
    #dbg_value(i64 1, !14258, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !38784)
    #dbg_value(ptr %58, !14277, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !38784)
    #dbg_value(i64 6, !14277, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !38784)
    #dbg_value(i64 %65, !14278, !DIExpression(), !38785)
    #dbg_value(i64 %65, !14282, !DIExpression(), !38786)
    #dbg_value(i64 %65, !14290, !DIExpression(), !38788)
    #dbg_value(i64 %65, !14304, !DIExpression(), !38790)
    #dbg_value(ptr %58, !14287, !DIExpression(DW_OP_deref), !38786)
    #dbg_value(ptr %58, !14298, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !38788)
    #dbg_value(ptr %58, !14317, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !38790)
    #dbg_value(i64 6, !14298, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !38788)
    #dbg_value(i64 6, !14317, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !38790)
  %67 = icmp ult i64 %65, 6, !dbg !38792
  call void @llvm.assume(i1 %67), !dbg !38792
  %68 = getelementptr inbounds nuw ptr, ptr %58, i64 %65, !dbg !38793
    #dbg_value(ptr %68, !14323, !DIExpression(), !38794)
    #dbg_value(ptr %68, !14331, !DIExpression(), !38795)
    #dbg_value(ptr %68, !14341, !DIExpression(), !38797)
  %69 = load ptr, ptr %68, align 8, !dbg !38799, !alias.scope !38800, !noalias !38672, !align !11920, !noundef !8
    #dbg_value(ptr %69, !14105, !DIExpression(), !38807)
  %70 = icmp eq ptr %69, null, !dbg !38808
  br i1 %70, label %62, label %83, !dbg !38808

71:                                               ; preds = %62, %59
  store i64 %57, ptr %54, align 1, !dbg !38809, !noalias !38672
  br label %72, !dbg !38810

72:                                               ; preds = %71, %53
    #dbg_value(ptr null, !14044, !DIExpression(), !38811)
  store i64 -1, ptr %10, align 8, !dbg !38810, !alias.scope !38735, !noalias !38672
  br label %73, !dbg !38812

73:                                               ; preds = %72, %49
  %74 = getelementptr inbounds nuw i8, ptr %10, i64 104, !dbg !38813
    #dbg_value(ptr null, !14361, !DIExpression(), !38814)
    #dbg_value(ptr %74, !14374, !DIExpression(), !38814)
    #dbg_value(ptr poison, !14379, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !38816)
    #dbg_value(ptr %74, !14390, !DIExpression(), !38818)
  %75 = load ptr, ptr %74, align 8, !dbg !38820, !alias.scope !38821, !noalias !38826, !noundef !8
  %76 = icmp eq ptr %75, null, !dbg !38820
  br i1 %76, label %107, label %77, !dbg !38820

77:                                               ; preds = %73
  %78 = invoke noundef align 8 ptr @_RNvXs_NtNtNtCsjihNppCmMEE_4core4iter8adapters6copiedINtB4_6CopiedINtNtNtBa_5slice4iter4IterRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENtNtNtB8_6traits8iterator8Iterator4nextCshdEBA0ozCnw_7game_ai(ptr noalias noundef nonnull align 8 dereferenceable(16) %74)
          to label %79 unwind label %47, !dbg !38828

79:                                               ; preds = %77
    #dbg_value(ptr %78, !38829, !DIExpression(), !38841)
    #dbg_value(ptr %10, !38838, !DIExpression(DW_OP_plus_uconst, 120, DW_OP_stack_value), !38841)
    #dbg_value(ptr %10, !38843, !DIExpression(DW_OP_plus_uconst, 120, DW_OP_stack_value), !38850)
  %80 = icmp eq ptr %78, null, !dbg !38852
  br i1 %80, label %107, label %81, !dbg !38852

81:                                               ; preds = %79
  %82 = load ptr, ptr %50, align 8, !dbg !38853, !alias.scope !38854, !noalias !38672
  br label %84, !dbg !38852

83:                                               ; preds = %64
  store i64 %66, ptr %54, align 1, !dbg !38809, !noalias !38672
  br label %84, !dbg !38853

84:                                               ; preds = %83, %81
  %85 = phi ptr [ %82, %81 ], [ %31, %83 ], !dbg !38853
  %86 = phi ptr [ %78, %81 ], [ %69, %83 ]
    #dbg_value(ptr %50, !38838, !DIExpression(), !38841)
    #dbg_value(ptr %50, !38843, !DIExpression(), !38850)
    #dbg_value(ptr %86, !38839, !DIExpression(), !38855)
    #dbg_value(ptr %86, !38848, !DIExpression(), !38850)
  call void @llvm.experimental.noalias.scope.decl(metadata !38856), !dbg !38853
    #dbg_value(ptr %86, !38859, !DIExpression(), !38865)
    #dbg_value(ptr poison, !38862, !DIExpression(DW_OP_deref), !38865)
    #dbg_value(ptr poison, !38867, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !38874)
    #dbg_value(ptr poison, !38873, !DIExpression(), !38874)
    #dbg_value(ptr %86, !38876, !DIExpression(), !38880)
    #dbg_value(ptr %85, !38879, !DIExpression(), !38880)
  %87 = getelementptr inbounds nuw i8, ptr %86, i64 1632, !dbg !38882
  %88 = load i64, ptr %87, align 8, !dbg !38882, !alias.scope !38856, !noalias !38883, !noundef !8
    #dbg_value(i64 %88, !38884, !DIExpression(), !38894)
    #dbg_value(i64 %88, !38896, !DIExpression(), !38900)
  %89 = getelementptr inbounds nuw i8, ptr %86, i64 1640, !dbg !38882
  %90 = load i64, ptr %89, align 8, !dbg !38882, !alias.scope !38856, !noalias !38883, !noundef !8
    #dbg_value(i64 %90, !38887, !DIExpression(), !38894)
    #dbg_value(i64 %90, !38896, !DIExpression(), !38902)
  %91 = getelementptr inbounds nuw i8, ptr %85, i64 1632, !dbg !38882
  %92 = load i64, ptr %91, align 8, !dbg !38882, !noalias !38904, !noundef !8
    #dbg_value(i64 %92, !38888, !DIExpression(), !38894)
    #dbg_value(i64 %92, !38899, !DIExpression(), !38900)
  %93 = getelementptr inbounds nuw i8, ptr %85, i64 1640, !dbg !38882
  %94 = load i64, ptr %93, align 8, !dbg !38882, !noalias !38904, !noundef !8
    #dbg_value(i64 %94, !38889, !DIExpression(), !38894)
    #dbg_value(i64 %94, !38899, !DIExpression(), !38902)
  %95 = icmp ult i64 %88, %92, !dbg !38905
  %96 = sub nuw i64 %92, %88, !dbg !38905
  %97 = sub nuw i64 %88, %92, !dbg !38905
  %98 = select i1 %95, i64 %96, i64 %97, !dbg !38905
    #dbg_value(i64 %98, !38890, !DIExpression(), !38906)
  %99 = icmp ult i64 %90, %94, !dbg !38907
  %100 = sub nuw i64 %94, %90, !dbg !38907
  %101 = sub nuw i64 %90, %94, !dbg !38907
  %102 = select i1 %99, i64 %100, i64 %101, !dbg !38907
    #dbg_value(i64 %102, !38892, !DIExpression(), !38908)
  %103 = mul i64 %98, %98, !dbg !38909
  %104 = mul i64 %102, %102, !dbg !38909
  %105 = add i64 %104, %103, !dbg !38909
    #dbg_value(i64 %105, !38709, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !38910)
    #dbg_value(ptr %86, !38709, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !38910)
  call void @llvm.lifetime.start.p0(ptr nonnull %9), !dbg !38911, !noalias !38912
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(128) %9, ptr noundef nonnull align 8 dereferenceable(128) %10, i64 128, i1 false), !dbg !38911, !noalias !38672
  %106 = invoke { i64, ptr } @_RINvXs0_NtNtNtCsjihNppCmMEE_4core4iter8adapters3mapINtB6_3MapINtNtB8_5chain5ChainINtNtB8_7flatten7FlattenINtNtNtBc_5array4iter8IntoIterINtNtBc_6option6OptionRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEKj6_EEINtNtB8_6copied6CopiedINtNtNtBc_5slice4iter4IterB2v_EEENCINvNvNtNtNtBa_6traits8iterator8Iterator10min_by_key3keyB2v_yNCNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler5modesNtB5x_17LegacyPlanHandler17single_try_engages_0E0EB4x_4foldTyB2v_ENCINvNvB4x_6min_by4foldB7l_INvB4v_7compareB2v_yEE0EB5B_(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(128) %9, i64 noundef %105, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %86)
          to label %108 unwind label %47, !dbg !38911

107:                                              ; preds = %79, %73
  call void @llvm.lifetime.end.p0(ptr nonnull %10), !dbg !38671, !noalias !38672
    #dbg_value(ptr null, !38579, !DIExpression(), !38913)
  call void @llvm.lifetime.end.p0(ptr nonnull %12), !dbg !38629
  br label %118, !dbg !38914

108:                                              ; preds = %84
  %109 = extractvalue { i64, ptr } %106, 1, !dbg !38911
  call void @llvm.lifetime.end.p0(ptr nonnull %9), !dbg !38911, !noalias !38912
  call void @llvm.lifetime.end.p0(ptr nonnull %10), !dbg !38671, !noalias !38672
    #dbg_value(ptr %109, !38579, !DIExpression(), !38913)
  call void @llvm.lifetime.end.p0(ptr nonnull %12), !dbg !38629
  %110 = icmp eq ptr %109, null, !dbg !38914
  br i1 %110, label %118, label %111, !dbg !38914

111:                                              ; preds = %108
    #dbg_value(ptr %109, !38580, !DIExpression(), !38915)
  %112 = getelementptr i8, ptr %109, i64 104, !dbg !38916
  %113 = load i64, ptr %112, align 8, !dbg !38916, !range !11183, !noundef !8
  %114 = getelementptr i8, ptr %109, i64 296, !dbg !38916
  %115 = load i8, ptr %114, align 8, !dbg !38916
    #dbg_declare(ptr poison, !38917, !DIExpression(), !38925)
    #dbg_value(ptr poison, !38922, !DIExpression(), !38927)
  %116 = icmp eq i64 %113, 2, !dbg !38928
  %117 = select i1 %116, i8 %115, i8 -1, !dbg !38928
  br label %118, !dbg !38925

118:                                              ; preds = %111, %108, %107
  %119 = phi i8 [ -1, %108 ], [ %117, %111 ], [ -1, %107 ], !dbg !38913
  %120 = getelementptr inbounds nuw i8, ptr %14, i64 140, !dbg !38629
  store i8 %119, ptr %120, align 4, !dbg !38629
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(144) %15, ptr noundef nonnull align 8 dereferenceable(144) %14, i64 144, i1 false), !dbg !38929
  call void @llvm.lifetime.end.p0(ptr nonnull %14), !dbg !38631
  br label %33, !dbg !38614

121:                                              ; preds = %33
  %122 = cleanuppad within none []
  call fastcc void @_RINvNtCsjihNppCmMEE_4core3ptr9drop_glueNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13single_battle16SinglePlanBattleEBJ_(ptr noalias noundef align 8 dereferenceable(144) %15) #31 [ "funclet"(token %122) ], !dbg !38630
  cleanupret from %122 unwind to caller, !dbg !38630

123:                                              ; preds = %33
  %124 = getelementptr inbounds nuw i8, ptr %15, i64 88, !dbg !38930
  %125 = load i64, ptr %124, align 8, !dbg !38930, !range !17349, !noundef !8
  switch i64 %125, label %126 [
    i64 3, label %127
    i64 4, label %127
    i64 7, label %127
  ], !dbg !38930

126:                                              ; preds = %123
    #dbg_value(i1 true, !38564, !DIExpression(DW_OP_LLVM_convert, 1, DW_ATE_unsigned, DW_OP_LLVM_convert, 8, DW_ATE_unsigned, DW_OP_stack_value), !38931)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(144) %0, ptr noundef nonnull align 8 dereferenceable(144) %15, i64 144, i1 false), !dbg !38932
  br label %46, !dbg !38630

127:                                              ; preds = %123, %123, %123
    #dbg_value(i1 false, !38564, !DIExpression(DW_OP_LLVM_convert, 1, DW_ATE_unsigned, DW_OP_LLVM_convert, 8, DW_ATE_unsigned, DW_OP_stack_value), !38931)
  store i64 -1, ptr %0, align 8, !dbg !38932
    #dbg_value(ptr %15, !12712, !DIExpression(), !38933)
  %128 = getelementptr inbounds nuw i8, ptr %15, i64 104, !dbg !38935
    #dbg_value(ptr %128, !9948, !DIExpression(), !38936)
  invoke void @_RNvXso_NtCs9LexZzt9XJB_5alloc3vecINtB5_3VecNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player4ChatENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr noalias noundef nonnull align 8 dereferenceable(24) %128)
          to label %131 unwind label %129, !dbg !38938

129:                                              ; preds = %127
  %130 = cleanuppad within none []
    #dbg_value(ptr %128, !9953, !DIExpression(), !38939)
  call void @_RNvXs1_NtCs9LexZzt9XJB_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player4ChatENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr noalias noundef nonnull align 8 dereferenceable(24) %128) [ "funclet"(token %130) ], !dbg !38941
  cleanupret from %130 unwind to caller, !dbg !38938

131:                                              ; preds = %127
    #dbg_value(ptr %128, !9953, !DIExpression(), !38942)
  call void @_RNvXs1_NtCs9LexZzt9XJB_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player4ChatENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr noalias noundef nonnull align 8 dereferenceable(24) %128), !dbg !38944
  br label %46, !dbg !38630
}
