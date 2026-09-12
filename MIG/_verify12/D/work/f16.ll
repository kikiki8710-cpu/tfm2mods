define noundef i64 @_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6battle24max_range_nearly_can_use(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(1728) %0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(1728) %1, i64 noundef %2) unnamed_addr #1 personality ptr @__CxxFrameHandler3 !dbg !56143 {
    #dbg_value(ptr %0, !56145, !DIExpression(), !56166)
    #dbg_value(ptr %0, !56167, !DIExpression(), !56194)
    #dbg_value(ptr %0, !56196, !DIExpression(), !56200)
    #dbg_value(ptr %0, !56202, !DIExpression(), !56207)
    #dbg_value(ptr %0, !56196, !DIExpression(), !56209)
    #dbg_value(ptr %0, !56211, !DIExpression(), !56214)
    #dbg_value(ptr %0, !56216, !DIExpression(), !56221)
    #dbg_value(ptr %0, !56196, !DIExpression(), !56223)
    #dbg_value(ptr %0, !56225, !DIExpression(), !56228)
    #dbg_value(ptr %0, !56230, !DIExpression(), !56235)
    #dbg_value(ptr %0, !56196, !DIExpression(), !56237)
    #dbg_value(ptr %1, !56146, !DIExpression(), !56166)
    #dbg_value(ptr %1, !56239, !DIExpression(), !56258)
    #dbg_value(ptr %1, !56239, !DIExpression(), !56259)
    #dbg_value(ptr %1, !56239, !DIExpression(), !56260)
    #dbg_value(ptr %1, !56239, !DIExpression(), !56261)
    #dbg_value(i64 %2, !56147, !DIExpression(), !56166)
    #dbg_value(i64 0, !56148, !DIExpression(), !56262)
    #dbg_value(ptr %0, !56263, !DIExpression(DW_OP_plus_uconst, 1168, DW_OP_stack_value), !56274)
  %4 = getelementptr inbounds nuw i8, ptr %0, i64 1168, !dbg !56276
  %5 = getelementptr inbounds nuw i8, ptr %0, i64 1216, !dbg !56276
  %6 = load i32, ptr %5, align 8, !dbg !56276, !range !9450, !noundef !8
  %7 = icmp eq i32 %6, -1, !dbg !56276
  br i1 %7, label %11, label %8, !dbg !56276

8:                                                ; preds = %3
    #dbg_value(ptr %4, !56150, !DIExpression(), !56277)
    #dbg_value(ptr %4, !56152, !DIExpression(), !56278)
    #dbg_value(ptr %4, !56199, !DIExpression(), !56200)
  %9 = getelementptr inbounds nuw i8, ptr %0, i64 1208, !dbg !56195
  %10 = tail call noundef zeroext i1 @_RNvMs_NtNtNtCs97f5S1uJLkH_9game_core10simulation6effect4typeNtB4_13CastingTarget5check(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %9, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %1), !dbg !56195
  br i1 %10, label %17, label %11, !dbg !56195

11:                                               ; preds = %74, %29, %8, %3
  %12 = phi i64 [ %80, %74 ], [ 0, %3 ], [ 0, %8 ], [ 0, %29 ], !dbg !56166
    #dbg_value(i64 %12, !56148, !DIExpression(), !56262)
    #dbg_value(ptr %0, !56263, !DIExpression(DW_OP_plus_uconst, 1224, DW_OP_stack_value), !56279)
  %13 = getelementptr inbounds nuw i8, ptr %0, i64 1224, !dbg !56281
  %14 = getelementptr inbounds nuw i8, ptr %0, i64 1272, !dbg !56281
  %15 = load i32, ptr %14, align 8, !dbg !56281, !range !9450, !noundef !8
  %16 = icmp eq i32 %15, -1, !dbg !56281
  br i1 %16, label %84, label %81, !dbg !56281

17:                                               ; preds = %8
  %18 = getelementptr inbounds nuw i8, ptr %0, i64 104, !dbg !56282
  %19 = load i64, ptr %18, align 8, !dbg !56282, !range !10463, !noundef !8
  switch i64 %19, label %20 [
    i64 0, label %34
    i64 1, label %29
    i64 2, label %21
    i64 3, label %34
    i64 4, label %22
    i64 5, label %23
    i64 6, label %23
    i64 7, label %22
    i64 8, label %24
    i64 9, label %25
    i64 10, label %26
    i64 11, label %27
    i64 12, label %28
    i64 13, label %24
  ], !dbg !56282

20:                                               ; preds = %17
  unreachable

21:                                               ; preds = %17
    #dbg_value(ptr %0, !56188, !DIExpression(DW_OP_plus_uconst, 112, DW_OP_stack_value), !56283)
  br label %29, !dbg !56284

22:                                               ; preds = %17, %17
    #dbg_value(ptr %0, !56172, !DIExpression(DW_OP_plus_uconst, 112, DW_OP_stack_value), !56285)
  br label %29, !dbg !56286

23:                                               ; preds = %17, %17
    #dbg_value(ptr %0, !56192, !DIExpression(DW_OP_plus_uconst, 112, DW_OP_stack_value), !56287)
  br label %29, !dbg !56288

24:                                               ; preds = %17, %17
    #dbg_value(ptr %0, !56178, !DIExpression(DW_OP_plus_uconst, 112, DW_OP_stack_value), !56289)
  br label %29, !dbg !56290

25:                                               ; preds = %17
    #dbg_value(ptr %0, !56180, !DIExpression(DW_OP_plus_uconst, 112, DW_OP_stack_value), !56291)
  br label %29, !dbg !56292

26:                                               ; preds = %17
    #dbg_value(ptr %0, !56182, !DIExpression(DW_OP_plus_uconst, 112, DW_OP_stack_value), !56293)
  br label %29, !dbg !56294

27:                                               ; preds = %17
    #dbg_value(ptr %0, !56184, !DIExpression(DW_OP_plus_uconst, 112, DW_OP_stack_value), !56295)
  br label %29, !dbg !56296

28:                                               ; preds = %17
    #dbg_value(ptr %0, !56186, !DIExpression(DW_OP_plus_uconst, 112, DW_OP_stack_value), !56297)
  br label %29, !dbg !56298

29:                                               ; preds = %28, %27, %26, %25, %24, %23, %22, %21, %17
  %30 = phi i64 [ 232, %22 ], [ 208, %28 ], [ 216, %27 ], [ 240, %26 ], [ 200, %25 ], [ 176, %24 ], [ 272, %21 ], [ 184, %17 ], [ 496, %23 ]
  %31 = getelementptr inbounds nuw i8, ptr %0, i64 %30, !dbg !56194
  %32 = load i64, ptr %31, align 8, !dbg !56194, !noundef !8
  %33 = icmp ugt i64 %32, %2, !dbg !56195
  br i1 %33, label %11, label %34, !dbg !56195

34:                                               ; preds = %29, %17, %17
  %35 = getelementptr inbounds nuw i8, ptr %0, i64 1184, !dbg !56299
  %36 = load i64, ptr %35, align 8, !dbg !56299, !noundef !8
  %37 = getelementptr inbounds nuw i8, ptr %0, i64 1192, !dbg !56299
  %38 = load i64, ptr %37, align 8, !dbg !56299, !noundef !8
  %39 = getelementptr inbounds nuw i8, ptr %0, i64 1480, !dbg !56299
  %40 = load i64, ptr %39, align 8, !dbg !56299, !noundef !8
  %41 = add i64 %40, -1, !dbg !56299
  %42 = mul i64 %41, %38, !dbg !56299
  %43 = getelementptr inbounds nuw i8, ptr %0, i64 1080, !dbg !56299
  %44 = load i64, ptr %43, align 8, !dbg !56299, !noundef !8
  %45 = tail call noundef i64 @_RNvMNtNtCs97f5S1uJLkH_9game_core10simulation6effectNtB2_6Effect12range_adjust(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(56) %4, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %1), !dbg !56201
  %46 = getelementptr inbounds nuw i8, ptr %0, i64 1136, !dbg !56300
  %47 = load i32, ptr %46, align 8, !dbg !56300, !noundef !8
    #dbg_value(i32 %47, !56242, !DIExpression(DW_OP_LLVM_convert, 32, DW_ATE_signed, DW_OP_LLVM_convert, 64, DW_ATE_signed, DW_OP_stack_value), !56301)
  %48 = icmp eq i32 %47, 0, !dbg !56302
  br i1 %48, label %49, label %52, !dbg !56302

49:                                               ; preds = %34
  %50 = getelementptr inbounds nuw i8, ptr %0, i64 1664, !dbg !56303
  %51 = load i64, ptr %50, align 8, !dbg !56303, !noundef !8
  br label %59, !dbg !56302

52:                                               ; preds = %34
  %53 = sext i32 %47 to i64, !dbg !56300
    #dbg_value(i64 %53, !56242, !DIExpression(), !56301)
  %54 = getelementptr inbounds nuw i8, ptr %0, i64 1664, !dbg !56304
  %55 = load i64, ptr %54, align 8, !dbg !56304, !noundef !8
  %56 = add nsw i64 %53, 100, !dbg !56304
  %57 = mul i64 %55, %56, !dbg !56304
  %58 = udiv i64 %57, 100, !dbg !56304
  br label %59, !dbg !56302

59:                                               ; preds = %52, %49
  %60 = phi i64 [ %51, %49 ], [ %58, %52 ], !dbg !56301
  %61 = getelementptr inbounds nuw i8, ptr %1, i64 1136, !dbg !56300
  %62 = load i32, ptr %61, align 8, !dbg !56300, !noundef !8
    #dbg_value(i32 %62, !56244, !DIExpression(DW_OP_LLVM_convert, 32, DW_ATE_signed, DW_OP_LLVM_convert, 64, DW_ATE_signed, DW_OP_stack_value), !56305)
  %63 = icmp eq i32 %62, 0, !dbg !56306
  br i1 %63, label %64, label %67, !dbg !56306

64:                                               ; preds = %59
  %65 = getelementptr inbounds nuw i8, ptr %1, i64 1664, !dbg !56307
  %66 = load i64, ptr %65, align 8, !dbg !56307, !noundef !8
  br label %74, !dbg !56306

67:                                               ; preds = %59
  %68 = sext i32 %62 to i64, !dbg !56300
    #dbg_value(i64 %68, !56244, !DIExpression(), !56305)
  %69 = getelementptr inbounds nuw i8, ptr %1, i64 1664, !dbg !56308
  %70 = load i64, ptr %69, align 8, !dbg !56308, !noundef !8
  %71 = add nsw i64 %68, 100, !dbg !56308
  %72 = mul i64 %70, %71, !dbg !56308
  %73 = udiv i64 %72, 100, !dbg !56308
  br label %74, !dbg !56306

74:                                               ; preds = %67, %64
  %75 = phi i64 [ %66, %64 ], [ %73, %67 ], !dbg !56305
  %76 = add i64 %44, %36, !dbg !56299
  %77 = add i64 %76, %42, !dbg !56299
  %78 = add i64 %77, %45, !dbg !56201
  %79 = add i64 %78, %60, !dbg !56201
  %80 = add i64 %79, %75, !dbg !56201
    #dbg_value(i64 0, !15752, !DIExpression(), !56309)
    #dbg_value(i64 %80, !15757, !DIExpression(), !56309)
    #dbg_value(i64 %80, !56148, !DIExpression(), !56262)
  br label %11, !dbg !56195

81:                                               ; preds = %11
    #dbg_value(ptr %13, !56154, !DIExpression(), !56311)
    #dbg_value(ptr %13, !56156, !DIExpression(), !56312)
    #dbg_value(ptr %13, !56199, !DIExpression(), !56209)
  %82 = getelementptr inbounds nuw i8, ptr %0, i64 1264, !dbg !56208
  %83 = tail call noundef zeroext i1 @_RNvMs_NtNtNtCs97f5S1uJLkH_9game_core10simulation6effect4typeNtB4_13CastingTarget5check(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %82, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %1), !dbg !56208
  br i1 %83, label %94, label %84, !dbg !56208

84:                                               ; preds = %142, %94, %81, %11
  %85 = phi i64 [ %149, %142 ], [ %12, %11 ], [ %12, %81 ], [ %12, %94 ], !dbg !56277
    #dbg_value(i64 %85, !56148, !DIExpression(), !56262)
  %86 = getelementptr inbounds nuw i8, ptr %0, i64 1480, !dbg !56313
  %87 = load i64, ptr %86, align 8, !dbg !56313, !noundef !8
  %88 = icmp ugt i64 %87, 2, !dbg !56313
  %89 = getelementptr inbounds nuw i8, ptr %0, i64 1280, !dbg !56313
  %90 = select i1 %88, ptr %89, ptr @anon.ff23c5838f81fa2a3acb125bb4b6e568.40, !dbg !56313
    #dbg_value(ptr %90, !56263, !DIExpression(), !56314)
  %91 = getelementptr inbounds nuw i8, ptr %90, i64 48, !dbg !56315
  %92 = load i32, ptr %91, align 8, !dbg !56315, !range !9450, !noundef !8
  %93 = icmp eq i32 %92, -1, !dbg !56315
  br i1 %93, label %153, label %150, !dbg !56315

94:                                               ; preds = %81
  %95 = getelementptr inbounds nuw i8, ptr %0, i64 104, !dbg !56316
  %96 = load i64, ptr %95, align 8, !dbg !56316, !range !10463, !noundef !8
  %97 = icmp eq i64 %96, 13, !dbg !56316
  %98 = getelementptr inbounds nuw i8, ptr %0, i64 184, !dbg !56316
  %99 = load i64, ptr %98, align 8, !dbg !56316
  %100 = icmp ugt i64 %99, %2, !dbg !56316
  %101 = select i1 %97, i1 %100, i1 false, !dbg !56316
  br i1 %101, label %84, label %102, !dbg !56208

102:                                              ; preds = %94
  %103 = getelementptr inbounds nuw i8, ptr %0, i64 1240, !dbg !56317
  %104 = load i64, ptr %103, align 8, !dbg !56317, !noundef !8
  %105 = getelementptr inbounds nuw i8, ptr %0, i64 1248, !dbg !56317
  %106 = load i64, ptr %105, align 8, !dbg !56317, !noundef !8
  %107 = getelementptr inbounds nuw i8, ptr %0, i64 1480, !dbg !56317
  %108 = load i64, ptr %107, align 8, !dbg !56317, !noundef !8
  %109 = add i64 %108, -1, !dbg !56317
  %110 = mul i64 %109, %106, !dbg !56317
  %111 = getelementptr inbounds nuw i8, ptr %0, i64 1080, !dbg !56317
  %112 = load i64, ptr %111, align 8, !dbg !56317, !noundef !8
  %113 = tail call noundef i64 @_RNvMNtNtCs97f5S1uJLkH_9game_core10simulation6effectNtB2_6Effect12range_adjust(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(56) %13, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %1), !dbg !56210
  %114 = getelementptr inbounds nuw i8, ptr %0, i64 1136, !dbg !56318
  %115 = load i32, ptr %114, align 8, !dbg !56318, !noundef !8
    #dbg_value(i32 %115, !56246, !DIExpression(DW_OP_LLVM_convert, 32, DW_ATE_signed, DW_OP_LLVM_convert, 64, DW_ATE_signed, DW_OP_stack_value), !56319)
  %116 = icmp eq i32 %115, 0, !dbg !56320
  br i1 %116, label %117, label %120, !dbg !56320

117:                                              ; preds = %102
  %118 = getelementptr inbounds nuw i8, ptr %0, i64 1664, !dbg !56321
  %119 = load i64, ptr %118, align 8, !dbg !56321, !noundef !8
  br label %127, !dbg !56320

120:                                              ; preds = %102
  %121 = sext i32 %115 to i64, !dbg !56318
    #dbg_value(i64 %121, !56246, !DIExpression(), !56319)
  %122 = getelementptr inbounds nuw i8, ptr %0, i64 1664, !dbg !56322
  %123 = load i64, ptr %122, align 8, !dbg !56322, !noundef !8
  %124 = add nsw i64 %121, 100, !dbg !56322
  %125 = mul i64 %123, %124, !dbg !56322
  %126 = udiv i64 %125, 100, !dbg !56322
  br label %127, !dbg !56320

127:                                              ; preds = %120, %117
  %128 = phi i64 [ %119, %117 ], [ %126, %120 ], !dbg !56319
  %129 = getelementptr inbounds nuw i8, ptr %1, i64 1136, !dbg !56318
  %130 = load i32, ptr %129, align 8, !dbg !56318, !noundef !8
    #dbg_value(i32 %130, !56248, !DIExpression(DW_OP_LLVM_convert, 32, DW_ATE_signed, DW_OP_LLVM_convert, 64, DW_ATE_signed, DW_OP_stack_value), !56323)
  %131 = icmp eq i32 %130, 0, !dbg !56324
  br i1 %131, label %132, label %135, !dbg !56324

132:                                              ; preds = %127
  %133 = getelementptr inbounds nuw i8, ptr %1, i64 1664, !dbg !56325
  %134 = load i64, ptr %133, align 8, !dbg !56325, !noundef !8
  br label %142, !dbg !56324

135:                                              ; preds = %127
  %136 = sext i32 %130 to i64, !dbg !56318
    #dbg_value(i64 %136, !56248, !DIExpression(), !56323)
  %137 = getelementptr inbounds nuw i8, ptr %1, i64 1664, !dbg !56326
  %138 = load i64, ptr %137, align 8, !dbg !56326, !noundef !8
  %139 = add nsw i64 %136, 100, !dbg !56326
  %140 = mul i64 %138, %139, !dbg !56326
  %141 = udiv i64 %140, 100, !dbg !56326
  br label %142, !dbg !56324

142:                                              ; preds = %135, %132
  %143 = phi i64 [ %134, %132 ], [ %141, %135 ], !dbg !56323
  %144 = add i64 %112, %104, !dbg !56317
  %145 = add i64 %144, %110, !dbg !56317
  %146 = add i64 %145, %113, !dbg !56210
  %147 = add i64 %146, %128, !dbg !56210
  %148 = add i64 %147, %143, !dbg !56210
    #dbg_value(i64 %12, !15752, !DIExpression(), !56327)
    #dbg_value(i64 %148, !15757, !DIExpression(), !56327)
  %149 = tail call noundef i64 @llvm.umax.i64(i64 %148, i64 %12), !dbg !56329
    #dbg_value(i64 %149, !56148, !DIExpression(), !56262)
  br label %84, !dbg !56208

150:                                              ; preds = %84
    #dbg_value(ptr %90, !56158, !DIExpression(), !56330)
    #dbg_value(ptr %90, !56160, !DIExpression(), !56331)
    #dbg_value(ptr %90, !56199, !DIExpression(), !56223)
  %151 = getelementptr inbounds nuw i8, ptr %90, i64 40, !dbg !56222
  %152 = tail call noundef zeroext i1 @_RNvMs_NtNtNtCs97f5S1uJLkH_9game_core10simulation6effect4typeNtB4_13CastingTarget5check(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %151, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %1), !dbg !56222
  br i1 %152, label %161, label %153, !dbg !56222

153:                                              ; preds = %211, %161, %150, %84
  %154 = phi i64 [ %214, %211 ], [ %85, %84 ], [ %85, %150 ], [ %85, %161 ], !dbg !56311
    #dbg_value(i64 %154, !56148, !DIExpression(), !56262)
  %155 = icmp ugt i64 %87, 4, !dbg !56332
  %156 = getelementptr inbounds nuw i8, ptr %0, i64 1336, !dbg !56332
  %157 = select i1 %155, ptr %156, ptr @anon.ff23c5838f81fa2a3acb125bb4b6e568.40, !dbg !56332
    #dbg_value(ptr %157, !56263, !DIExpression(), !56333)
  %158 = getelementptr inbounds nuw i8, ptr %157, i64 48, !dbg !56334
  %159 = load i32, ptr %158, align 8, !dbg !56334, !range !9450, !noundef !8
  %160 = icmp eq i32 %159, -1, !dbg !56334
  br i1 %160, label %218, label %215, !dbg !56334

161:                                              ; preds = %150
  %162 = getelementptr inbounds nuw i8, ptr %0, i64 104, !dbg !56335
  %163 = load i64, ptr %162, align 8, !dbg !56335, !range !10463, !noundef !8
  %164 = icmp eq i64 %163, 13, !dbg !56335
  %165 = getelementptr inbounds nuw i8, ptr %0, i64 192, !dbg !56335
  %166 = load i64, ptr %165, align 8, !dbg !56335
  %167 = icmp ugt i64 %166, %2, !dbg !56335
  %168 = select i1 %164, i1 %167, i1 false, !dbg !56335
  br i1 %168, label %153, label %169, !dbg !56222

169:                                              ; preds = %161
  %170 = getelementptr inbounds nuw i8, ptr %90, i64 16, !dbg !56336
  %171 = load i64, ptr %170, align 8, !dbg !56336, !noundef !8
  %172 = getelementptr inbounds nuw i8, ptr %90, i64 24, !dbg !56336
  %173 = load i64, ptr %172, align 8, !dbg !56336, !noundef !8
  %174 = add i64 %87, -1, !dbg !56336
  %175 = mul i64 %173, %174, !dbg !56336
  %176 = add i64 %175, %171, !dbg !56336
  %177 = getelementptr inbounds nuw i8, ptr %0, i64 1080, !dbg !56336
  %178 = load i64, ptr %177, align 8, !dbg !56336, !noundef !8
  %179 = add i64 %176, %178, !dbg !56336
  %180 = tail call noundef i64 @_RNvMNtNtCs97f5S1uJLkH_9game_core10simulation6effectNtB2_6Effect12range_adjust(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(56) %90, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %1), !dbg !56224
  %181 = add i64 %179, %180, !dbg !56224
  %182 = getelementptr inbounds nuw i8, ptr %0, i64 1136, !dbg !56337
  %183 = load i32, ptr %182, align 8, !dbg !56337, !noundef !8
    #dbg_value(i32 %183, !56250, !DIExpression(DW_OP_LLVM_convert, 32, DW_ATE_signed, DW_OP_LLVM_convert, 64, DW_ATE_signed, DW_OP_stack_value), !56338)
  %184 = icmp eq i32 %183, 0, !dbg !56339
  br i1 %184, label %185, label %188, !dbg !56339

185:                                              ; preds = %169
  %186 = getelementptr inbounds nuw i8, ptr %0, i64 1664, !dbg !56340
  %187 = load i64, ptr %186, align 8, !dbg !56340, !noundef !8
  br label %195, !dbg !56339

188:                                              ; preds = %169
  %189 = sext i32 %183 to i64, !dbg !56337
    #dbg_value(i64 %189, !56250, !DIExpression(), !56338)
  %190 = getelementptr inbounds nuw i8, ptr %0, i64 1664, !dbg !56341
  %191 = load i64, ptr %190, align 8, !dbg !56341, !noundef !8
  %192 = add nsw i64 %189, 100, !dbg !56341
  %193 = mul i64 %191, %192, !dbg !56341
  %194 = udiv i64 %193, 100, !dbg !56341
  br label %195, !dbg !56339

195:                                              ; preds = %188, %185
  %196 = phi i64 [ %187, %185 ], [ %194, %188 ], !dbg !56338
  %197 = add i64 %181, %196, !dbg !56224
  %198 = getelementptr inbounds nuw i8, ptr %1, i64 1136, !dbg !56337
  %199 = load i32, ptr %198, align 8, !dbg !56337, !noundef !8
    #dbg_value(i32 %199, !56252, !DIExpression(DW_OP_LLVM_convert, 32, DW_ATE_signed, DW_OP_LLVM_convert, 64, DW_ATE_signed, DW_OP_stack_value), !56342)
  %200 = icmp eq i32 %199, 0, !dbg !56343
  br i1 %200, label %201, label %204, !dbg !56343

201:                                              ; preds = %195
  %202 = getelementptr inbounds nuw i8, ptr %1, i64 1664, !dbg !56344
  %203 = load i64, ptr %202, align 8, !dbg !56344, !noundef !8
  br label %211, !dbg !56343

204:                                              ; preds = %195
  %205 = sext i32 %199 to i64, !dbg !56337
    #dbg_value(i64 %205, !56252, !DIExpression(), !56342)
  %206 = getelementptr inbounds nuw i8, ptr %1, i64 1664, !dbg !56345
  %207 = load i64, ptr %206, align 8, !dbg !56345, !noundef !8
  %208 = add nsw i64 %205, 100, !dbg !56345
  %209 = mul i64 %207, %208, !dbg !56345
  %210 = udiv i64 %209, 100, !dbg !56345
  br label %211, !dbg !56343

211:                                              ; preds = %204, %201
  %212 = phi i64 [ %203, %201 ], [ %210, %204 ], !dbg !56342
  %213 = add i64 %197, %212, !dbg !56224
    #dbg_value(i64 %85, !15752, !DIExpression(), !56346)
    #dbg_value(i64 %213, !15757, !DIExpression(), !56346)
  %214 = tail call noundef i64 @llvm.umax.i64(i64 %213, i64 %85), !dbg !56348
    #dbg_value(i64 %214, !56148, !DIExpression(), !56262)
  br label %153, !dbg !56222

215:                                              ; preds = %153
    #dbg_value(ptr %157, !56162, !DIExpression(), !56349)
    #dbg_value(ptr %157, !56164, !DIExpression(), !56350)
    #dbg_value(ptr %157, !56199, !DIExpression(), !56237)
  %216 = getelementptr inbounds nuw i8, ptr %157, i64 40, !dbg !56236
  %217 = tail call noundef zeroext i1 @_RNvMs_NtNtNtCs97f5S1uJLkH_9game_core10simulation6effect4typeNtB4_13CastingTarget5check(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %216, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %1), !dbg !56236
  br i1 %217, label %220, label %218, !dbg !56236

218:                                              ; preds = %270, %220, %215, %153
  %219 = phi i64 [ %273, %270 ], [ %154, %153 ], [ %154, %215 ], [ %154, %220 ], !dbg !56330
    #dbg_value(i64 %219, !56148, !DIExpression(), !56262)
  ret i64 %219, !dbg !56351

220:                                              ; preds = %215
  %221 = getelementptr inbounds nuw i8, ptr %0, i64 104, !dbg !56352
  %222 = load i64, ptr %221, align 8, !dbg !56352, !range !10463, !noundef !8
  %223 = icmp eq i64 %222, 13, !dbg !56352
  %224 = getelementptr inbounds nuw i8, ptr %0, i64 200, !dbg !56352
  %225 = load i64, ptr %224, align 8, !dbg !56352
  %226 = icmp ugt i64 %225, %2, !dbg !56352
  %227 = select i1 %223, i1 %226, i1 false, !dbg !56352
  br i1 %227, label %218, label %228, !dbg !56236

228:                                              ; preds = %220
  %229 = getelementptr inbounds nuw i8, ptr %157, i64 16, !dbg !56353
  %230 = load i64, ptr %229, align 8, !dbg !56353, !noundef !8
  %231 = getelementptr inbounds nuw i8, ptr %157, i64 24, !dbg !56353
  %232 = load i64, ptr %231, align 8, !dbg !56353, !noundef !8
  %233 = add i64 %87, -1, !dbg !56353
  %234 = mul i64 %232, %233, !dbg !56353
  %235 = add i64 %234, %230, !dbg !56353
  %236 = getelementptr inbounds nuw i8, ptr %0, i64 1080, !dbg !56353
  %237 = load i64, ptr %236, align 8, !dbg !56353, !noundef !8
  %238 = add i64 %235, %237, !dbg !56353
  %239 = tail call noundef i64 @_RNvMNtNtCs97f5S1uJLkH_9game_core10simulation6effectNtB2_6Effect12range_adjust(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(56) %157, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1728) %1), !dbg !56238
  %240 = add i64 %238, %239, !dbg !56238
  %241 = getelementptr inbounds nuw i8, ptr %0, i64 1136, !dbg !56354
  %242 = load i32, ptr %241, align 8, !dbg !56354, !noundef !8
    #dbg_value(i32 %242, !56254, !DIExpression(DW_OP_LLVM_convert, 32, DW_ATE_signed, DW_OP_LLVM_convert, 64, DW_ATE_signed, DW_OP_stack_value), !56355)
  %243 = icmp eq i32 %242, 0, !dbg !56356
  br i1 %243, label %244, label %247, !dbg !56356

244:                                              ; preds = %228
  %245 = getelementptr inbounds nuw i8, ptr %0, i64 1664, !dbg !56357
  %246 = load i64, ptr %245, align 8, !dbg !56357, !noundef !8
  br label %254, !dbg !56356

247:                                              ; preds = %228
  %248 = sext i32 %242 to i64, !dbg !56354
    #dbg_value(i64 %248, !56254, !DIExpression(), !56355)
  %249 = getelementptr inbounds nuw i8, ptr %0, i64 1664, !dbg !56358
  %250 = load i64, ptr %249, align 8, !dbg !56358, !noundef !8
  %251 = add nsw i64 %248, 100, !dbg !56358
  %252 = mul i64 %250, %251, !dbg !56358
  %253 = udiv i64 %252, 100, !dbg !56358
  br label %254, !dbg !56356

254:                                              ; preds = %247, %244
  %255 = phi i64 [ %246, %244 ], [ %253, %247 ], !dbg !56355
  %256 = add i64 %240, %255, !dbg !56238
  %257 = getelementptr inbounds nuw i8, ptr %1, i64 1136, !dbg !56354
  %258 = load i32, ptr %257, align 8, !dbg !56354, !noundef !8
    #dbg_value(i32 %258, !56256, !DIExpression(DW_OP_LLVM_convert, 32, DW_ATE_signed, DW_OP_LLVM_convert, 64, DW_ATE_signed, DW_OP_stack_value), !56359)
  %259 = icmp eq i32 %258, 0, !dbg !56360
  br i1 %259, label %260, label %263, !dbg !56360

260:                                              ; preds = %254
  %261 = getelementptr inbounds nuw i8, ptr %1, i64 1664, !dbg !56361
  %262 = load i64, ptr %261, align 8, !dbg !56361, !noundef !8
  br label %270, !dbg !56360

263:                                              ; preds = %254
  %264 = sext i32 %258 to i64, !dbg !56354
    #dbg_value(i64 %264, !56256, !DIExpression(), !56359)
  %265 = getelementptr inbounds nuw i8, ptr %1, i64 1664, !dbg !56362
  %266 = load i64, ptr %265, align 8, !dbg !56362, !noundef !8
  %267 = add nsw i64 %264, 100, !dbg !56362
  %268 = mul i64 %266, %267, !dbg !56362
  %269 = udiv i64 %268, 100, !dbg !56362
  br label %270, !dbg !56360

270:                                              ; preds = %263, %260
  %271 = phi i64 [ %262, %260 ], [ %269, %263 ], !dbg !56359
  %272 = add i64 %256, %271, !dbg !56238
    #dbg_value(i64 %154, !15752, !DIExpression(), !56363)
    #dbg_value(i64 %272, !15757, !DIExpression(), !56363)
  %273 = tail call noundef i64 @llvm.umax.i64(i64 %272, i64 %154), !dbg !56365
    #dbg_value(i64 %273, !56148, !DIExpression(), !56262)
  br label %218, !dbg !56236
}
