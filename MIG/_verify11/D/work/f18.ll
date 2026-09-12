define internal fastcc noundef zeroext i1 @_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epicNtNtB6_9team_plan8TeamPlan22v3_epicops_buff_window(ptr noalias noundef nonnull align 8 dereferenceable(1064) %0, i64 noundef range(i64 2, 0) %1, ptr noalias noundef nonnull align 16 dereferenceable(320) %2, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %3, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %4, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(248) %5, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(384) %6) unnamed_addr #0 personality ptr @__CxxFrameHandler3 !dbg !14691 {
    #dbg_value(ptr poison, !14710, !DIExpression(), !14740)
    #dbg_value(ptr poison, !14746, !DIExpression(), !14751)
    #dbg_value(ptr poison, !14752, !DIExpression(), !14763)
  %8 = alloca [24 x i8], align 4
    #dbg_value(ptr %0, !14697, !DIExpression(), !14765)
    #dbg_value(i64 %1, !14698, !DIExpression(), !14765)
    #dbg_value(ptr %2, !14699, !DIExpression(), !14765)
    #dbg_value(ptr %3, !14700, !DIExpression(), !14765)
    #dbg_value(ptr %4, !14701, !DIExpression(), !14765)
    #dbg_value(ptr %5, !14702, !DIExpression(), !14765)
    #dbg_value(ptr %6, !14703, !DIExpression(), !14765)
    #dbg_declare(ptr %8, !14704, !DIExpression(), !14766)
  %9 = getelementptr inbounds nuw i8, ptr %3, i64 2352, !dbg !14767
  %10 = load i64, ptr %9, align 8, !dbg !14767, !noundef !8
  %11 = load ptr, ptr %4, align 8, !dbg !14767
  %12 = tail call fastcc noundef i8 @_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic22v3_epicops_repair_need(i64 %10, ptr %11, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(384) %6), !dbg !14767
  switch i8 %12, label %13 [
    i8 1, label %15
    i8 2, label %29
  ], !dbg !14767

13:                                               ; preds = %7
  %14 = tail call noundef zeroext i1 @_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers30is_object_being_taken_by_enemy(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %3, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %4, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(248) %5, ptr noundef nonnull align 8 %0, i1 noundef zeroext true), !dbg !14768
  br i1 %14, label %33, label %35, !dbg !14768

15:                                               ; preds = %7
  %16 = getelementptr inbounds nuw i8, ptr %0, i64 1055, !dbg !14769
  store i8 7, ptr %16, align 1, !dbg !14769
  %17 = getelementptr inbounds nuw i8, ptr %0, i64 192, !dbg !14770
    #dbg_value(ptr %17, !14771, !DIExpression(), !14775)
    #dbg_value(i64 0, !14774, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !14775)
    #dbg_value(i64 0, !13792, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !14776)
    #dbg_value(i64 0, !13807, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !14778)
    #dbg_value(i8 23, !14774, !DIExpression(DW_OP_LLVM_fragment, 0, 8), !14775)
    #dbg_value(i8 23, !13792, !DIExpression(DW_OP_LLVM_fragment, 0, 8), !14776)
    #dbg_value(i8 23, !13807, !DIExpression(DW_OP_LLVM_fragment, 0, 8), !14778)
    #dbg_value(ptr %17, !13799, !DIExpression(), !14776)
    #dbg_value(ptr %17, !13815, !DIExpression(), !14780)
    #dbg_value(i64 24, !13823, !DIExpression(), !14782)
  %18 = getelementptr inbounds nuw i8, ptr %0, i64 208, !dbg !14785
  %19 = load i64, ptr %18, align 8, !dbg !14785, !alias.scope !14786, !noalias !14789, !noundef !8
    #dbg_value(i64 %19, !13800, !DIExpression(), !14791)
    #dbg_value(i64 %19, !13844, !DIExpression(), !14792)
    #dbg_value(ptr %17, !13835, !DIExpression(), !14793)
  %20 = load i64, ptr %17, align 8, !dbg !14794, !range !7894, !alias.scope !14786, !noalias !14789, !noundef !8
  %21 = icmp eq i64 %19, %20, !dbg !14795
  br i1 %21, label %22, label %23, !dbg !14795

22:                                               ; preds = %15
  tail call void @_RNvMs3_NtCs9LexZzt9XJB_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player4ChatE8grow_oneCshdEBA0ozCnw_7game_ai(ptr noalias noundef nonnull align 8 dereferenceable(24) %17), !dbg !14796, !noalias !14789
  br label %23, !dbg !14795

23:                                               ; preds = %22, %15
  %24 = getelementptr inbounds nuw i8, ptr %0, i64 200, !dbg !14797
  %25 = load ptr, ptr %24, align 8, !dbg !14797, !alias.scope !14786, !noalias !14789, !nonnull !8, !noundef !8
    #dbg_value(ptr %25, !13849, !DIExpression(), !14792)
  %26 = getelementptr inbounds nuw { i8, [23 x i8] }, ptr %25, i64 %19, !dbg !14801
    #dbg_value(ptr %26, !13802, !DIExpression(), !14802)
    #dbg_value(ptr %26, !13812, !DIExpression(), !14778)
  store i8 23, ptr %26, align 8, !dbg !14803
  %27 = getelementptr inbounds nuw i8, ptr %26, i64 8, !dbg !14803
  store i64 0, ptr %27, align 8, !dbg !14803
  %28 = add i64 %19, 1, !dbg !14804
  store i64 %28, ptr %18, align 8, !dbg !14804, !alias.scope !14786, !noalias !14789
  br label %31, !dbg !14805

29:                                               ; preds = %7
  %30 = getelementptr inbounds nuw i8, ptr %0, i64 1055, !dbg !14806
  store i8 7, ptr %30, align 1, !dbg !14806
  br label %31, !dbg !14807

31:                                               ; preds = %74, %73, %56, %29, %23
  %32 = phi i1 [ true, %56 ], [ false, %74 ], [ false, %73 ], [ true, %23 ], [ true, %29 ], !dbg !14765
  ret i1 %32, !dbg !14808

33:                                               ; preds = %13
  %34 = tail call noundef zeroext i1 @_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6serpen27v3_serpen_contest_clear_win(i64 noundef %1, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %3, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %4, ptr noundef nonnull align 8 %0), !dbg !14809
  br i1 %34, label %43, label %35, !dbg !14809

35:                                               ; preds = %33, %13
  call void @llvm.lifetime.start.p0(ptr nonnull %8), !dbg !14810
  %36 = load ptr, ptr %11, align 8, !dbg !14810, !nonnull !8, !noundef !8
  %37 = getelementptr inbounds nuw i8, ptr %11, i64 8, !dbg !14810
  %38 = load ptr, ptr %37, align 8, !dbg !14810, !nonnull !8, !align !8871, !noundef !8
  call void @_RNvMsb_NtNtNtCs97f5S1uJLkH_9game_core10simulation5state6playerNtB5_11PlayerState8strategy(ptr noalias noundef nonnull sret([24 x i8]) align 4 captures(address) dereferenceable(24) %8, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %3, ptr noalias noundef nonnull align 16 dereferenceable(320) %2, ptr noundef nonnull %36, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(816) %38), !dbg !14810
  %39 = getelementptr inbounds nuw i8, ptr %8, i64 4, !dbg !14811
  %40 = load i64, ptr %39, align 4, !dbg !14811
  %41 = call noundef i8 @_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic18v3_epic_group_line(i64 %40, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %3, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %4), !dbg !14811
  %42 = icmp eq i8 %41, -1, !dbg !14811
  br i1 %42, label %73, label %62, !dbg !14811

43:                                               ; preds = %33
  %44 = getelementptr inbounds nuw i8, ptr %0, i64 1040, !dbg !14812
  %45 = load i64, ptr %44, align 8, !dbg !14812, !noundef !8
  %46 = add i64 %45, 1, !dbg !14812
  store i64 %46, ptr %44, align 8, !dbg !14812
  %47 = getelementptr inbounds nuw i8, ptr %0, i64 1055, !dbg !14813
  store i8 1, ptr %47, align 1, !dbg !14813
  %48 = getelementptr inbounds nuw i8, ptr %0, i64 1056, !dbg !14813
  store i8 1, ptr %48, align 1, !dbg !14813
  %49 = getelementptr inbounds nuw i8, ptr %0, i64 1057, !dbg !14813
  store i8 1, ptr %49, align 1, !dbg !14813
  %50 = getelementptr inbounds nuw i8, ptr %0, i64 192, !dbg !14814
    #dbg_value(ptr %50, !14771, !DIExpression(), !14815)
    #dbg_value(i64 0, !14774, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !14815)
    #dbg_value(i64 0, !13792, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !14816)
    #dbg_value(i64 0, !13807, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !14818)
    #dbg_value(i8 25, !14774, !DIExpression(DW_OP_LLVM_fragment, 0, 8), !14815)
    #dbg_value(i8 25, !13792, !DIExpression(DW_OP_LLVM_fragment, 0, 8), !14816)
    #dbg_value(i8 25, !13807, !DIExpression(DW_OP_LLVM_fragment, 0, 8), !14818)
    #dbg_value(ptr %50, !13799, !DIExpression(), !14816)
    #dbg_value(ptr %50, !13815, !DIExpression(), !14820)
    #dbg_value(i64 24, !13823, !DIExpression(), !14822)
  %51 = getelementptr inbounds nuw i8, ptr %0, i64 208, !dbg !14825
  %52 = load i64, ptr %51, align 8, !dbg !14825, !alias.scope !14826, !noalias !14829, !noundef !8
    #dbg_value(i64 %52, !13800, !DIExpression(), !14831)
    #dbg_value(i64 %52, !13844, !DIExpression(), !14832)
    #dbg_value(ptr %50, !13835, !DIExpression(), !14833)
  %53 = load i64, ptr %50, align 8, !dbg !14834, !range !7894, !alias.scope !14826, !noalias !14829, !noundef !8
  %54 = icmp eq i64 %52, %53, !dbg !14835
  br i1 %54, label %55, label %56, !dbg !14835

55:                                               ; preds = %43
  tail call void @_RNvMs3_NtCs9LexZzt9XJB_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player4ChatE8grow_oneCshdEBA0ozCnw_7game_ai(ptr noalias noundef nonnull align 8 dereferenceable(24) %50), !dbg !14836, !noalias !14829
  br label %56, !dbg !14835

56:                                               ; preds = %55, %43
  %57 = getelementptr inbounds nuw i8, ptr %0, i64 200, !dbg !14837
  %58 = load ptr, ptr %57, align 8, !dbg !14837, !alias.scope !14826, !noalias !14829, !nonnull !8, !noundef !8
    #dbg_value(ptr %58, !13849, !DIExpression(), !14832)
  %59 = getelementptr inbounds nuw { i8, [23 x i8] }, ptr %58, i64 %52, !dbg !14841
    #dbg_value(ptr %59, !13802, !DIExpression(), !14842)
    #dbg_value(ptr %59, !13812, !DIExpression(), !14818)
  store i8 25, ptr %59, align 8, !dbg !14843
  %60 = getelementptr inbounds nuw i8, ptr %59, i64 8, !dbg !14843
  store i64 0, ptr %60, align 8, !dbg !14843
  %61 = add i64 %52, 1, !dbg !14844
  store i64 %61, ptr %51, align 8, !dbg !14844, !alias.scope !14826, !noalias !14829
  br label %31, !dbg !14845

62:                                               ; preds = %35
    #dbg_value(i8 %41, !14706, !DIExpression(), !14847)
    #dbg_value(ptr %0, !14848, !DIExpression(DW_OP_plus_uconst, 1054, DW_OP_stack_value), !14858)
    #dbg_value(ptr %0, !14860, !DIExpression(DW_OP_plus_uconst, 1054, DW_OP_stack_value), !14867)
    #dbg_value(ptr poison, !14854, !DIExpression(), !14858)
    #dbg_value(ptr poison, !14863, !DIExpression(), !14867)
  %63 = getelementptr inbounds nuw i8, ptr %0, i64 1054, !dbg !14869
  %64 = load i8, ptr %63, align 2, !dbg !14869, !range !14870, !noundef !8
  %65 = icmp eq i8 %64, %41, !dbg !14869
  br i1 %65, label %74, label %66, !dbg !14859

66:                                               ; preds = %62
  %67 = icmp ult i64 %10, 2
  %68 = getelementptr inbounds nuw i8, ptr %11, i64 480
  %69 = getelementptr inbounds nuw [5 x ptr], ptr %68, i64 %10
  br i1 %67, label %70, label %75, !dbg !14871

70:                                               ; preds = %66
    #dbg_value(i64 0, !14731, !DIExpression(), !14893)
    #dbg_value(i64 0, !14887, !DIExpression(), !14894)
    #dbg_value(ptr poison, !14888, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !14894)
    #dbg_declare(ptr poison, !14889, !DIExpression(), !14895)
    #dbg_value(ptr poison, !14878, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !14896)
    #dbg_value(ptr poison, !14879, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !14896)
    #dbg_value(ptr poison, !14880, !DIExpression(DW_OP_deref), !14896)
    #dbg_value(ptr poison, !14877, !DIExpression(), !14896)
    #dbg_declare(ptr poison, !14897, !DIExpression(), !14910)
    #dbg_value(ptr %69, !14912, !DIExpression(), !14915)
  %71 = load ptr, ptr %69, align 8, !dbg !14917, !noalias !14918, !align !8871, !noundef !8
  %72 = icmp eq ptr %71, null, !dbg !14917
  br i1 %72, label %84, label %76, !dbg !14871

73:                                               ; preds = %35
  call void @llvm.lifetime.end.p0(ptr nonnull %8), !dbg !14808
  br label %31, !dbg !14845

74:                                               ; preds = %146, %62
  call void @llvm.lifetime.end.p0(ptr nonnull %8), !dbg !14808
  br label %31, !dbg !14808

75:                                               ; preds = %66
  call void @_RNvNtCsjihNppCmMEE_4core9panicking18panic_bounds_check(i64 noundef %10, i64 noundef 2, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @anon.17beaf855860b1a5c4c224343f517d87.22) #28, !dbg !14871, !noalias !14918
  unreachable, !dbg !14871

76:                                               ; preds = %70
  %77 = call noundef i32 @_RNvMs_NtNtCs97f5S1uJLkH_9game_core10simulation6entityNtB4_8Position10from_index(i64 noundef 0), !dbg !14926, !noalias !14918
  %78 = call { i8, i8 } @_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic22v3_epic_formation_role(i64 %40, i32 noundef %77, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %3, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %4), !dbg !14926, !noalias !14918
  %79 = extractvalue { i8, i8 } %78, 0, !dbg !14926
    #dbg_value(i8 %79, !14907, !DIExpression(DW_OP_LLVM_fragment, 0, 8), !14927)
    #dbg_value(i8 poison, !14907, !DIExpression(DW_OP_LLVM_fragment, 8, 8), !14927)
  %80 = icmp ne i8 %79, 2, !dbg !14928
  %81 = trunc i8 %79 to i1, !dbg !14928
  %82 = xor i1 %80, %81, !dbg !14928
  %83 = freeze i1 %82
  br i1 %83, label %132, label %84, !dbg !14929

84:                                               ; preds = %76, %70
    #dbg_value(i64 1, !14731, !DIExpression(), !14893)
    #dbg_value(i64 1, !14887, !DIExpression(), !14894)
    #dbg_value(ptr poison, !14888, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !14894)
    #dbg_declare(ptr poison, !14889, !DIExpression(), !14895)
    #dbg_value(ptr poison, !14878, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !14896)
    #dbg_value(ptr poison, !14879, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !14896)
    #dbg_value(ptr poison, !14880, !DIExpression(DW_OP_deref), !14896)
    #dbg_value(ptr poison, !14877, !DIExpression(), !14896)
    #dbg_declare(ptr poison, !14897, !DIExpression(), !14910)
  %85 = getelementptr inbounds nuw i8, ptr %69, i64 8, !dbg !14871
    #dbg_value(ptr %85, !14912, !DIExpression(), !14915)
  %86 = load ptr, ptr %85, align 8, !dbg !14917, !noalias !14918, !align !8871, !noundef !8
  %87 = icmp eq ptr %86, null, !dbg !14917
  br i1 %87, label %96, label %88, !dbg !14871

88:                                               ; preds = %84
  %89 = call noundef i32 @_RNvMs_NtNtCs97f5S1uJLkH_9game_core10simulation6entityNtB4_8Position10from_index(i64 noundef 1), !dbg !14926, !noalias !14918
  %90 = call { i8, i8 } @_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic22v3_epic_formation_role(i64 %40, i32 noundef %89, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %3, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %4), !dbg !14926, !noalias !14918
  %91 = extractvalue { i8, i8 } %90, 0, !dbg !14926
    #dbg_value(i8 %91, !14907, !DIExpression(DW_OP_LLVM_fragment, 0, 8), !14927)
    #dbg_value(i8 poison, !14907, !DIExpression(DW_OP_LLVM_fragment, 8, 8), !14927)
  %92 = icmp ne i8 %91, 2, !dbg !14928
  %93 = trunc i8 %91 to i1, !dbg !14928
  %94 = xor i1 %92, %93, !dbg !14928
  %95 = freeze i1 %94
  br i1 %95, label %132, label %96, !dbg !14929

96:                                               ; preds = %88, %84
    #dbg_value(i64 2, !14731, !DIExpression(), !14893)
    #dbg_value(i64 2, !14887, !DIExpression(), !14894)
    #dbg_value(ptr poison, !14888, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !14894)
    #dbg_declare(ptr poison, !14889, !DIExpression(), !14895)
    #dbg_value(ptr poison, !14878, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !14896)
    #dbg_value(ptr poison, !14879, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !14896)
    #dbg_value(ptr poison, !14880, !DIExpression(DW_OP_deref), !14896)
    #dbg_value(ptr poison, !14877, !DIExpression(), !14896)
    #dbg_declare(ptr poison, !14897, !DIExpression(), !14910)
  %97 = getelementptr inbounds nuw i8, ptr %69, i64 16, !dbg !14871
    #dbg_value(ptr %97, !14912, !DIExpression(), !14915)
  %98 = load ptr, ptr %97, align 8, !dbg !14917, !noalias !14918, !align !8871, !noundef !8
  %99 = icmp eq ptr %98, null, !dbg !14917
  br i1 %99, label %108, label %100, !dbg !14871

100:                                              ; preds = %96
  %101 = call noundef i32 @_RNvMs_NtNtCs97f5S1uJLkH_9game_core10simulation6entityNtB4_8Position10from_index(i64 noundef 2), !dbg !14926, !noalias !14918
  %102 = call { i8, i8 } @_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic22v3_epic_formation_role(i64 %40, i32 noundef %101, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %3, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %4), !dbg !14926, !noalias !14918
  %103 = extractvalue { i8, i8 } %102, 0, !dbg !14926
    #dbg_value(i8 %103, !14907, !DIExpression(DW_OP_LLVM_fragment, 0, 8), !14927)
    #dbg_value(i8 poison, !14907, !DIExpression(DW_OP_LLVM_fragment, 8, 8), !14927)
  %104 = icmp ne i8 %103, 2, !dbg !14928
  %105 = trunc i8 %103 to i1, !dbg !14928
  %106 = xor i1 %104, %105, !dbg !14928
  %107 = freeze i1 %106
  br i1 %107, label %132, label %108, !dbg !14929

108:                                              ; preds = %100, %96
    #dbg_value(i64 3, !14731, !DIExpression(), !14893)
    #dbg_value(i64 3, !14887, !DIExpression(), !14894)
    #dbg_value(ptr poison, !14888, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !14894)
    #dbg_declare(ptr poison, !14889, !DIExpression(), !14895)
    #dbg_value(ptr poison, !14878, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !14896)
    #dbg_value(ptr poison, !14879, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !14896)
    #dbg_value(ptr poison, !14880, !DIExpression(DW_OP_deref), !14896)
    #dbg_value(ptr poison, !14877, !DIExpression(), !14896)
    #dbg_declare(ptr poison, !14897, !DIExpression(), !14910)
  %109 = getelementptr inbounds nuw i8, ptr %69, i64 24, !dbg !14871
    #dbg_value(ptr %109, !14912, !DIExpression(), !14915)
  %110 = load ptr, ptr %109, align 8, !dbg !14917, !noalias !14918, !align !8871, !noundef !8
  %111 = icmp eq ptr %110, null, !dbg !14917
  br i1 %111, label %120, label %112, !dbg !14871

112:                                              ; preds = %108
  %113 = call noundef i32 @_RNvMs_NtNtCs97f5S1uJLkH_9game_core10simulation6entityNtB4_8Position10from_index(i64 noundef 3), !dbg !14926, !noalias !14918
  %114 = call { i8, i8 } @_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic22v3_epic_formation_role(i64 %40, i32 noundef %113, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %3, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %4), !dbg !14926, !noalias !14918
  %115 = extractvalue { i8, i8 } %114, 0, !dbg !14926
    #dbg_value(i8 %115, !14907, !DIExpression(DW_OP_LLVM_fragment, 0, 8), !14927)
    #dbg_value(i8 poison, !14907, !DIExpression(DW_OP_LLVM_fragment, 8, 8), !14927)
  %116 = icmp ne i8 %115, 2, !dbg !14928
  %117 = trunc i8 %115 to i1, !dbg !14928
  %118 = xor i1 %116, %117, !dbg !14928
  %119 = freeze i1 %118
  br i1 %119, label %132, label %120, !dbg !14929

120:                                              ; preds = %112, %108
    #dbg_value(i64 4, !14731, !DIExpression(), !14893)
    #dbg_value(i64 4, !14887, !DIExpression(), !14894)
    #dbg_value(ptr poison, !14888, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !14894)
    #dbg_declare(ptr poison, !14889, !DIExpression(), !14895)
    #dbg_value(ptr poison, !14878, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !14896)
    #dbg_value(ptr poison, !14879, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !14896)
    #dbg_value(ptr poison, !14880, !DIExpression(DW_OP_deref), !14896)
    #dbg_value(ptr poison, !14877, !DIExpression(), !14896)
    #dbg_declare(ptr poison, !14897, !DIExpression(), !14910)
  %121 = getelementptr inbounds nuw i8, ptr %69, i64 32, !dbg !14871
    #dbg_value(ptr %121, !14912, !DIExpression(), !14915)
  %122 = load ptr, ptr %121, align 8, !dbg !14917, !noalias !14918, !align !8871, !noundef !8
  %123 = icmp eq ptr %122, null, !dbg !14917
  br i1 %123, label %146, label %124, !dbg !14871

124:                                              ; preds = %120
  %125 = call noundef i32 @_RNvMs_NtNtCs97f5S1uJLkH_9game_core10simulation6entityNtB4_8Position10from_index(i64 noundef 4), !dbg !14926, !noalias !14918
  %126 = call { i8, i8 } @_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic22v3_epic_formation_role(i64 %40, i32 noundef %125, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2528) %3, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %4), !dbg !14926, !noalias !14918
  %127 = extractvalue { i8, i8 } %126, 0, !dbg !14926
    #dbg_value(i8 %127, !14907, !DIExpression(DW_OP_LLVM_fragment, 0, 8), !14927)
    #dbg_value(i8 poison, !14907, !DIExpression(DW_OP_LLVM_fragment, 8, 8), !14927)
  %128 = icmp ne i8 %127, 2, !dbg !14928
  %129 = trunc i8 %127 to i1, !dbg !14928
  %130 = xor i1 %128, %129, !dbg !14928
  %131 = freeze i1 %130
  br i1 %131, label %132, label %146, !dbg !14929

132:                                              ; preds = %124, %112, %100, %88, %76
  %133 = phi i64 [ 0, %76 ], [ 1, %88 ], [ 2, %100 ], [ 3, %112 ], [ 4, %124 ]
    #dbg_value(i64 %133, !14708, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !14930)
    #dbg_value(i64 1, !14708, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !14930)
    #dbg_value(ptr undef, !14752, !DIExpression(), !14763)
    #dbg_value(ptr %3, !14931, !DIExpression(DW_OP_plus_uconst, 2496, DW_OP_stack_value), !14934)
  %134 = getelementptr inbounds nuw i8, ptr %3, i64 2496, !dbg !14935
  %135 = load i32, ptr %134, align 8, !dbg !14935, !range !12403, !noundef !8
  %136 = zext nneg i32 %135 to i64, !dbg !14935
    #dbg_value(ptr poison, !14759, !DIExpression(), !14763)
    #dbg_value(ptr undef, !14760, !DIExpression(DW_OP_plus_uconst, 8, DW_OP_stack_value), !14936)
    #dbg_value(ptr undef, !14937, !DIExpression(DW_OP_plus_uconst, 8, DW_OP_stack_value), !14942)
    #dbg_value(ptr poison, !14762, !DIExpression(), !14936)
    #dbg_value(ptr poison, !14941, !DIExpression(), !14942)
  %137 = icmp eq i64 %133, %136, !dbg !14942
  br i1 %137, label %147, label %146, !dbg !14764

138:                                              ; preds = %147
  call void @_RNvMs3_NtCs9LexZzt9XJB_5alloc7raw_vecINtB5_6RawVecNtNtNtNtCs97f5S1uJLkH_9game_core10simulation5state6player4ChatE8grow_oneCshdEBA0ozCnw_7game_ai(ptr noalias noundef nonnull align 8 dereferenceable(24) %150), !dbg !14945, !noalias !8
  br label %139, !dbg !14947

139:                                              ; preds = %147, %138
  %140 = getelementptr inbounds nuw i8, ptr %0, i64 200, !dbg !14947
  %141 = load ptr, ptr %140, align 8, !dbg !14947, !noalias !8, !nonnull !8, !noundef !8
  %142 = getelementptr inbounds nuw { i8, [23 x i8] }, ptr %141, i64 %152, !dbg !14952
  store i8 %155, ptr %142, align 8, !dbg !14953
  %143 = getelementptr inbounds nuw i8, ptr %142, i64 1, !dbg !14953
  store i8 %41, ptr %143, align 1, !dbg !14953
  %144 = getelementptr inbounds nuw i8, ptr %142, i64 8, !dbg !14953
  store i64 0, ptr %144, align 8, !dbg !14953
  %145 = add i64 %152, 1, !dbg !14955
  store i64 %145, ptr %151, align 8, !dbg !14955, !noalias !8
  br label %146, !dbg !14956

146:                                              ; preds = %139, %132, %124, %120
  store i8 %41, ptr %63, align 2, !dbg !14956
  br label %74, !dbg !14859

147:                                              ; preds = %132
    #dbg_value(ptr %0, !14957, !DIExpression(DW_OP_plus_uconst, 1054, DW_OP_stack_value), !14963)
    #dbg_value(ptr %0, !14965, !DIExpression(DW_OP_plus_uconst, 1054, DW_OP_stack_value), !14969)
  %148 = load i8, ptr %63, align 2, !dbg !14971, !range !14870, !noundef !8
  %149 = icmp eq i8 %148, -1, !dbg !14971
  %150 = getelementptr inbounds nuw i8, ptr %0, i64 192, !dbg !14930
    #dbg_value(ptr %150, !14771, !DIExpression(), !14972)
    #dbg_value(ptr %150, !14771, !DIExpression(), !14974)
    #dbg_value(i8 %41, !14774, !DIExpression(DW_OP_LLVM_fragment, 8, 8), !14972)
    #dbg_value(i8 %41, !14774, !DIExpression(DW_OP_LLVM_fragment, 8, 8), !14974)
    #dbg_value(i8 %41, !13792, !DIExpression(DW_OP_LLVM_fragment, 8, 8), !14976)
    #dbg_value(i8 %41, !13792, !DIExpression(DW_OP_LLVM_fragment, 8, 8), !14978)
    #dbg_value(i8 %41, !13807, !DIExpression(DW_OP_LLVM_fragment, 8, 8), !14980)
    #dbg_value(i8 %41, !13807, !DIExpression(DW_OP_LLVM_fragment, 8, 8), !14982)
    #dbg_value(i64 0, !14774, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !14972)
    #dbg_value(i64 0, !14774, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !14974)
    #dbg_value(i64 0, !13792, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !14976)
    #dbg_value(i64 0, !13792, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !14978)
    #dbg_value(i64 0, !13807, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !14980)
    #dbg_value(i64 0, !13807, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !14982)
    #dbg_value(ptr %150, !13799, !DIExpression(), !14976)
    #dbg_value(ptr %150, !13799, !DIExpression(), !14978)
    #dbg_value(ptr %150, !13815, !DIExpression(), !14984)
    #dbg_value(ptr %150, !13815, !DIExpression(), !14986)
    #dbg_value(i64 24, !13823, !DIExpression(), !14988)
    #dbg_value(i64 24, !13823, !DIExpression(), !14991)
  %151 = getelementptr inbounds nuw i8, ptr %0, i64 208, !dbg !14994
  %152 = load i64, ptr %151, align 8, !dbg !14994, !noalias !8, !noundef !8
    #dbg_value(i64 %152, !13800, !DIExpression(), !14995)
    #dbg_value(i64 %152, !13800, !DIExpression(), !14996)
    #dbg_value(i64 %152, !13844, !DIExpression(), !14997)
    #dbg_value(i64 %152, !13844, !DIExpression(), !14998)
    #dbg_value(ptr %150, !13835, !DIExpression(), !14999)
    #dbg_value(ptr %150, !13835, !DIExpression(), !15000)
  %153 = load i64, ptr %150, align 8, !dbg !15001, !range !7894, !noalias !8, !noundef !8
  %154 = icmp eq i64 %152, %153, !dbg !15003
  %155 = select i1 %149, i8 21, i8 22, !dbg !15003
  br i1 %154, label %138, label %139, !dbg !15003
}
