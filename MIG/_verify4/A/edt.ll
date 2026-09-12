define noundef i64 @_RNvMNtNtCs97f5S1uJLkH_9game_core10simulation6effectNtB2_6Effect22expected_damage_target(ptr noalias noundef readonly align 8 captures(none) dereferenceable(56) %0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(64) %1, ptr noundef nonnull %2, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(88) %3, ptr noalias noundef readonly align 8 captures(none) dereferenceable(1728) %4) unnamed_addr #0 personality ptr @__CxxFrameHandler3 !dbg !63700 {
  %6 = alloca [24 x i8], align 8
  %7 = alloca [24 x i8], align 8
    #dbg_value(ptr %0, !63705, !DIExpression(), !63718)
    #dbg_value(ptr %0, !63719, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !63737)
    #dbg_value(ptr poison, !63739, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !63746)
    #dbg_value(ptr %1, !63706, !DIExpression(), !63718)
    #dbg_value(ptr %1, !63719, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !63737)
    #dbg_value(ptr poison, !63744, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !63746)
    #dbg_value(ptr %2, !63707, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !63718)
    #dbg_value(ptr %2, !63719, !DIExpression(DW_OP_LLVM_fragment, 128, 64), !63737)
    #dbg_value(ptr %3, !63707, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !63718)
    #dbg_value(ptr %3, !63719, !DIExpression(DW_OP_LLVM_fragment, 192, 64), !63737)
    #dbg_value(ptr %4, !63708, !DIExpression(), !63718)
    #dbg_declare(ptr %7, !63709, !DIExpression(), !63748)
    #dbg_declare(ptr %6, !63734, !DIExpression(), !63749)
    #dbg_value(i64 53, !63750, !DIExpression(), !63755)
    #dbg_value(i8 0, !63757, !DIExpression(), !63764)
  call void @llvm.lifetime.start.p0(ptr nonnull %7), !dbg !63756
    #dbg_value(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED, !63763, !DIExpression(), !63764)
    #dbg_value(ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED, !63766, !DIExpression(), !63772)
    #dbg_value(i8 0, !63771, !DIExpression(), !63772)
  %8 = load atomic i8, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof7ENABLED monotonic, align 1, !dbg !63774
  %9 = icmp eq i8 %8, 0, !dbg !63765
  br i1 %9, label %15, label %10, !dbg !63765

10:                                               ; preds = %5
  %11 = tail call { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant3now(), !dbg !63775
  %12 = extractvalue { i64, i32 } %11, 0, !dbg !63775
  %13 = extractvalue { i64, i32 } %11, 1, !dbg !63775
  store i64 53, ptr %7, align 8, !dbg !63775
  %14 = getelementptr inbounds nuw i8, ptr %7, i64 8, !dbg !63775
  store i64 %12, ptr %14, align 8, !dbg !63775
  br label %15, !dbg !63776

15:                                               ; preds = %10, %5
  %16 = phi i32 [ %13, %10 ], [ -1, %5 ], !dbg !63755
  %17 = getelementptr inbounds nuw i8, ptr %7, i64 16, !dbg !63755
  store i32 %16, ptr %17, align 8, !dbg !63755
    #dbg_value(ptr %4, !63777, !DIExpression(DW_OP_plus_uconst, 104, DW_OP_stack_value), !63784)
  %18 = getelementptr inbounds nuw i8, ptr %4, i64 104, !dbg !63786
  %19 = load i64, ptr %18, align 8, !dbg !63786, !range !30476, !noundef !8
  %20 = and i64 %19, 14, !dbg !63786
  %21 = icmp eq i64 %20, 2, !dbg !63786
  br i1 %21, label %35, label %22, !dbg !63786

22:                                               ; preds = %15
    #dbg_value(ptr %0, !63787, !DIExpression(), !63790)
    #dbg_value(ptr %0, !63792, !DIExpression(), !63795)
  %23 = load ptr, ptr %0, align 8, !dbg !63798, !nonnull !8, !noundef !8
  %24 = getelementptr inbounds nuw i8, ptr %0, i64 8, !dbg !63798
  %25 = load ptr, ptr %24, align 8, !dbg !63798, !nonnull !8, !align !29465, !noundef !8
  %26 = getelementptr inbounds nuw i8, ptr %25, i64 16, !dbg !63802
  %27 = load i64, ptr %26, align 8, !dbg !63802, !range !29209, !invariant.load !8
  %28 = add nsw i64 %27, -1, !dbg !63802
  %29 = and i64 %28, -16, !dbg !63802
  %30 = getelementptr inbounds nuw i8, ptr %23, i64 %29, !dbg !63802
  %31 = getelementptr inbounds nuw i8, ptr %30, i64 16, !dbg !63802
  %32 = getelementptr inbounds nuw i8, ptr %25, i64 40, !dbg !63791
  %33 = load ptr, ptr %32, align 8, !dbg !63791, !invariant.load !8, !nonnull !8
  %34 = invoke { i64, i64 } %33(ptr noundef nonnull %31, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(64) %1, ptr noundef nonnull %2, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(88) %3)
          to label %78 unwind label %47, !dbg !63791

35:                                               ; preds = %15
  call void @llvm.lifetime.start.p0(ptr nonnull %6), !dbg !63738
    #dbg_value(ptr %0, !63787, !DIExpression(), !63803)
    #dbg_value(ptr %0, !63792, !DIExpression(), !63804)
  %36 = load ptr, ptr %0, align 8, !dbg !63806, !nonnull !8, !noundef !8
  %37 = getelementptr inbounds nuw i8, ptr %0, i64 8, !dbg !63806
  %38 = load ptr, ptr %37, align 8, !dbg !63806, !nonnull !8, !align !29465, !noundef !8
  %39 = getelementptr inbounds nuw i8, ptr %38, i64 16, !dbg !63805
  %40 = load i64, ptr %39, align 8, !dbg !63805, !range !29209, !invariant.load !8
  %41 = add nsw i64 %40, -1, !dbg !63805
  %42 = and i64 %41, -16, !dbg !63805
  %43 = getelementptr inbounds nuw i8, ptr %36, i64 %42, !dbg !63805
  %44 = getelementptr inbounds nuw i8, ptr %43, i64 16, !dbg !63805
  %45 = getelementptr inbounds nuw i8, ptr %38, i64 48, !dbg !63738
  %46 = load ptr, ptr %45, align 8, !dbg !63738, !invariant.load !8, !nonnull !8
  invoke void %46(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %6, ptr noundef nonnull %44, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(64) %1, ptr noundef nonnull %2, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(88) %3)
          to label %49 unwind label %47, !dbg !63738

47:                                               ; preds = %254, %192, %187, %163, %101, %93, %67, %57, %35, %22
  %48 = cleanuppad within none []
  call fastcc void @_RINvNtCsjihNppCmMEE_4core3ptr9drop_glueINtNtB4_6option6OptionNtNtNtCs97f5S1uJLkH_9game_core10simulation4prof9ProfTimerEEB13_(ptr noalias noundef align 8 dereferenceable(24) %7) #45 [ "funclet"(token %48) ], !dbg !63808
  cleanupret from %48 unwind to caller, !dbg !63809

49:                                               ; preds = %35
  %50 = load i64, ptr %6, align 8, !dbg !63810, !range !26733, !noundef !8
  %51 = trunc nuw i64 %50 to i1, !dbg !63810
  br i1 %51, label %52, label %57, !dbg !63810

52:                                               ; preds = %49
  %53 = getelementptr inbounds nuw i8, ptr %6, i64 8, !dbg !63811
  %54 = load i64, ptr %53, align 8, !dbg !63811, !noundef !8
  %55 = getelementptr inbounds nuw i8, ptr %6, i64 16, !dbg !63811
  %56 = load i64, ptr %55, align 8, !dbg !63811, !noundef !8
  br label %64, !dbg !63812

57:                                               ; preds = %49
    #dbg_value(ptr %0, !63787, !DIExpression(), !63813)
    #dbg_value(ptr %0, !63792, !DIExpression(), !63815)
  %58 = getelementptr inbounds nuw i8, ptr %38, i64 40, !dbg !63814
  %59 = load ptr, ptr %58, align 8, !dbg !63814, !invariant.load !8, !nonnull !8
  %60 = invoke { i64, i64 } %59(ptr noundef nonnull %44, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(64) %1, ptr noundef nonnull %2, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(88) %3)
          to label %61 unwind label %47, !dbg !63814

61:                                               ; preds = %57
  %62 = extractvalue { i64, i64 } %60, 0, !dbg !63814
  %63 = extractvalue { i64, i64 } %60, 1, !dbg !63814
  br label %64, !dbg !63747

64:                                               ; preds = %61, %52
  %65 = phi i64 [ %56, %52 ], [ %63, %61 ], !dbg !63737
  %66 = phi i64 [ %54, %52 ], [ %62, %61 ], !dbg !63737
  call void @llvm.lifetime.end.p0(ptr nonnull %6), !dbg !63738
  br label %67, !dbg !63785

67:                                               ; preds = %78, %64
  %68 = phi i64 [ %29, %78 ], [ %42, %64 ], !dbg !63818
  %69 = phi ptr [ %25, %78 ], [ %38, %64 ], !dbg !63820
  %70 = phi ptr [ %23, %78 ], [ %36, %64 ], !dbg !63820
  %71 = phi i64 [ %80, %78 ], [ %65, %64 ], !dbg !63825
  %72 = phi i64 [ %79, %78 ], [ %66, %64 ], !dbg !63825
    #dbg_value(i64 %72, !63711, !DIExpression(), !63826)
    #dbg_value(i64 %71, !63713, !DIExpression(), !63826)
    #dbg_value(ptr %0, !63787, !DIExpression(), !63827)
    #dbg_value(ptr %0, !63792, !DIExpression(), !63828)
  %73 = getelementptr inbounds nuw i8, ptr %70, i64 %68, !dbg !63818
  %74 = getelementptr inbounds nuw i8, ptr %73, i64 16, !dbg !63818
  %75 = getelementptr inbounds nuw i8, ptr %69, i64 56, !dbg !63819
  %76 = load ptr, ptr %75, align 8, !dbg !63819, !invariant.load !8, !nonnull !8
  %77 = invoke noundef i64 %76(ptr noundef nonnull %74, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(64) %1, ptr noundef nonnull %2, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(88) %3)
          to label %81 unwind label %47, !dbg !63819

78:                                               ; preds = %22
  %79 = extractvalue { i64, i64 } %34, 0, !dbg !63791
  %80 = extractvalue { i64, i64 } %34, 1, !dbg !63791
  br label %67, !dbg !63785

81:                                               ; preds = %67
    #dbg_value(i64 %77, !63714, !DIExpression(), !63829)
  %82 = icmp eq i64 %77, 0, !dbg !63830
  br i1 %82, label %89, label %83, !dbg !63830

83:                                               ; preds = %81
  %84 = getelementptr inbounds nuw i8, ptr %4, i64 1576, !dbg !63830
  %85 = load i64, ptr %84, align 8, !dbg !63830, !noundef !8
  %86 = mul i64 %85, %77, !dbg !63830
  %87 = udiv i64 %86, 100, !dbg !63830
  %88 = add i64 %87, %72, !dbg !63830
    #dbg_value(i64 %88, !63716, !DIExpression(), !63831)
  br label %89, !dbg !63830

89:                                               ; preds = %83, %81
  %90 = phi i64 [ %88, %83 ], [ %72, %81 ], !dbg !63830
    #dbg_value(i64 %90, !63716, !DIExpression(), !63831)
  %91 = or i64 %90, %71, !dbg !63832
  %92 = icmp eq i64 %91, 0, !dbg !63832
  br i1 %92, label %165, label %93, !dbg !63832

93:                                               ; preds = %89
  %94 = getelementptr inbounds nuw i8, ptr %0, i64 44, !dbg !63833
  %95 = load i32, ptr %94, align 4, !dbg !63833, !range !63834, !noundef !8
  %96 = getelementptr inbounds nuw i8, ptr %3, i64 56, !dbg !63833
  %97 = load ptr, ptr %96, align 8, !dbg !63833
  %98 = getelementptr inbounds nuw i8, ptr %3, i64 64, !dbg !63833
  %99 = load ptr, ptr %98, align 8, !dbg !63833
  call void @llvm.experimental.noalias.scope.decl(metadata !63835), !dbg !63833
    #dbg_value(i64 %90, !63838, !DIExpression(), !63869)
    #dbg_value(ptr %2, !63843, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !63869)
    #dbg_value(ptr poison, !63843, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !63869)
    #dbg_value(ptr %4, !63844, !DIExpression(), !63869)
    #dbg_value(i32 %95, !63845, !DIExpression(), !63869)
    #dbg_value(i32 0, !63846, !DIExpression(), !63869)
    #dbg_value(i64 100, !63871, !DIExpression(), !63875)
    #dbg_value(i64 100, !63871, !DIExpression(), !63877)
  %100 = invoke noundef nonnull align 8 ptr %97(ptr noundef nonnull %2)
          to label %101 unwind label %47, !dbg !63869

101:                                              ; preds = %93
    #dbg_value(ptr %4, !63847, !DIExpression(DW_OP_plus_uconst, 1560, DW_OP_stack_value), !63879)
    #dbg_value(ptr %100, !63849, !DIExpression(), !63880)
  %102 = getelementptr inbounds nuw i8, ptr %4, i64 1584, !dbg !63881
  %103 = load i64, ptr %102, align 8, !dbg !63881, !alias.scope !63835, !noundef !8
    #dbg_value(i64 %103, !63851, !DIExpression(), !63882)
    #dbg_value(i64 %103, !63855, !DIExpression(), !63883)
  %104 = invoke noundef nonnull align 8 ptr %99(ptr noundef nonnull %2)
          to label %105 unwind label %47, !dbg !63884

105:                                              ; preds = %101
    #dbg_value(ptr %104, !63853, !DIExpression(), !63885)
  switch i32 %95, label %120 [
    i32 0, label %106
    i32 2, label %130
    i32 3, label %130
  ], !dbg !63886

106:                                              ; preds = %105
  %107 = getelementptr inbounds nuw i8, ptr %104, i64 208, !dbg !63887
  %108 = load i64, ptr %107, align 8, !dbg !63887, !noalias !63835, !noundef !8
  %109 = icmp eq i64 %108, 0, !dbg !63887
  br i1 %109, label %135, label %110, !dbg !63887

110:                                              ; preds = %106
  %111 = getelementptr inbounds nuw i8, ptr %4, i64 1576, !dbg !63888
  %112 = load i64, ptr %111, align 8, !dbg !63888, !alias.scope !63835, !noundef !8
  %113 = mul i64 %112, %108, !dbg !63888
  %114 = udiv i64 %113, 100, !dbg !63888
  %115 = add i64 %114, %90, !dbg !63888
    #dbg_value(i64 %115, !63838, !DIExpression(), !63869)
  br label %135, !dbg !63887

116:                                              ; preds = %124, %120
  %117 = phi i64 [ %90, %120 ], [ %129, %124 ]
    #dbg_value(i64 %117, !63838, !DIExpression(), !63869)
  %118 = and i32 %95, 6, !dbg !63889
  %119 = icmp eq i32 %118, 2, !dbg !63889
  br i1 %119, label %130, label %135, !dbg !63889

120:                                              ; preds = %105
  %121 = getelementptr inbounds nuw i8, ptr %104, i64 224, !dbg !63890
  %122 = load i64, ptr %121, align 8, !dbg !63890, !noalias !63835, !noundef !8
  %123 = icmp eq i64 %122, 0, !dbg !63890
  br i1 %123, label %116, label %124, !dbg !63890

124:                                              ; preds = %120
  %125 = getelementptr inbounds nuw i8, ptr %4, i64 1576, !dbg !63891
  %126 = load i64, ptr %125, align 8, !dbg !63891, !alias.scope !63835, !noundef !8
  %127 = mul i64 %126, %122, !dbg !63891
  %128 = udiv i64 %127, 100, !dbg !63891
  %129 = add i64 %128, %90, !dbg !63891
    #dbg_value(i64 %129, !63838, !DIExpression(), !63869)
  br label %116, !dbg !63890

130:                                              ; preds = %116, %105, %105
  %131 = phi i64 [ %90, %105 ], [ %117, %116 ], [ %90, %105 ]
    #dbg_value(i64 %131, !63838, !DIExpression(), !63869)
  %132 = getelementptr inbounds nuw i8, ptr %104, i64 240, !dbg !63892
  %133 = load i64, ptr %132, align 8, !dbg !63892, !noalias !63835, !noundef !8
  %134 = icmp eq i64 %133, 0, !dbg !63892
  br i1 %134, label %144, label %140, !dbg !63892

135:                                              ; preds = %116, %110, %106
  %136 = phi i64 [ %117, %116 ], [ %115, %110 ], [ %90, %106 ]
  %137 = getelementptr inbounds nuw i8, ptr %104, i64 216, !dbg !63893
  %138 = load i64, ptr %137, align 8, !dbg !63893, !noalias !63835, !noundef !8
  %139 = icmp eq i64 %138, 0, !dbg !63893
  br i1 %139, label %144, label %149, !dbg !63893

140:                                              ; preds = %130
  %141 = add i64 %133, 100, !dbg !63894
  %142 = mul i64 %141, %131, !dbg !63894
  %143 = udiv i64 %142, 100, !dbg !63894
    #dbg_value(i64 %143, !63838, !DIExpression(), !63869)
  br label %144, !dbg !63892

144:                                              ; preds = %149, %140, %135, %130
  %145 = phi i64 [ %136, %135 ], [ %154, %149 ], [ %131, %130 ], [ %143, %140 ]
    #dbg_value(i64 %145, !63838, !DIExpression(), !63869)
  %146 = getelementptr inbounds nuw i8, ptr %104, i64 168, !dbg !63895
  %147 = load i64, ptr %146, align 8, !dbg !63895, !noalias !63835, !noundef !8
    #dbg_value(i64 %147, !63874, !DIExpression(), !63875)
  %148 = icmp eq i64 %147, 0, !dbg !63895
  br i1 %148, label %159, label %155, !dbg !63895

149:                                              ; preds = %135
  %150 = getelementptr inbounds nuw i8, ptr %100, i64 16, !dbg !63896
  %151 = load i64, ptr %150, align 8, !dbg !63896, !noalias !63835, !noundef !8
  %152 = mul i64 %151, %138, !dbg !63896
  %153 = udiv i64 %152, 100, !dbg !63896
  %154 = add i64 %153, %136, !dbg !63896
    #dbg_value(i64 %154, !63838, !DIExpression(), !63869)
  br label %144, !dbg !63893

155:                                              ; preds = %144
  %156 = call i64 @llvm.usub.sat.i64(i64 100, i64 %147), !dbg !63897
  %157 = mul i64 %156, %103, !dbg !63898
  %158 = udiv i64 %157, 100, !dbg !63898
    #dbg_value(i64 %158, !63851, !DIExpression(), !63882)
    #dbg_value(i64 %158, !63855, !DIExpression(), !63883)
  br label %159, !dbg !63895

159:                                              ; preds = %155, %144
  %160 = phi i64 [ %103, %144 ], [ %158, %155 ], !dbg !63880
    #dbg_value(i64 %160, !63855, !DIExpression(), !63883)
    #dbg_value(i64 %160, !63851, !DIExpression(), !63882)
  %161 = add i64 %160, 100, !dbg !63899
  %162 = icmp eq i64 %161, 0, !dbg !63899
  br i1 %162, label %163, label %187, !dbg !63899

163:                                              ; preds = %159
  invoke void @_RNvNtNtCsjihNppCmMEE_4core9panicking11panic_const23panic_const_div_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @anon.92df479ad083757a8f3d2193749aeb7a.10) #44
          to label %164 unwind label %47, !dbg !63899

164:                                              ; preds = %163
  unreachable, !dbg !63899

165:                                              ; preds = %256, %89
  %166 = phi i64 [ %260, %256 ], [ 0, %89 ], !dbg !63831
  call void @llvm.experimental.noalias.scope.decl(metadata !63900), !dbg !63808
    #dbg_value(ptr %7, !27623, !DIExpression(), !63903)
  %167 = load i32, ptr %17, align 8, !dbg !63905, !range !27628, !alias.scope !63900, !noundef !8
  %168 = icmp eq i32 %167, -1, !dbg !63905
  br i1 %168, label %186, label %169, !dbg !63905

169:                                              ; preds = %165
  call void @llvm.experimental.noalias.scope.decl(metadata !63906), !dbg !63905
    #dbg_value(ptr %7, !27632, !DIExpression(), !63909)
  call void @llvm.experimental.noalias.scope.decl(metadata !63911), !dbg !63914
    #dbg_value(ptr %7, !27644, !DIExpression(), !63915)
    #dbg_value(i8 0, !27651, !DIExpression(), !63917)
    #dbg_value(i8 0, !27664, !DIExpression(), !63919)
    #dbg_value(i64 1, !27661, !DIExpression(), !63921)
    #dbg_value(i8 0, !27651, !DIExpression(), !63921)
    #dbg_value(i64 1, !27672, !DIExpression(), !63923)
    #dbg_value(i8 0, !27664, !DIExpression(), !63923)
  %170 = load i64, ptr %7, align 8, !dbg !63925, !alias.scope !63926, !noundef !8
  %171 = icmp ult i64 %170, 132, !dbg !63925
  br i1 %171, label %173, label %172, !dbg !63925

172:                                              ; preds = %169
  call void @_RNvNtCsjihNppCmMEE_4core9panicking18panic_bounds_check(i64 noundef %170, i64 noundef 132, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @anon.92df479ad083757a8f3d2193749aeb7a.963) #44, !dbg !63925, !noalias !63926
  unreachable, !dbg !63925

173:                                              ; preds = %169
  %174 = getelementptr inbounds nuw { { { i64 } } }, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_NANOS, i64 %170, !dbg !63925
    #dbg_value(ptr %174, !27659, !DIExpression(), !63917)
  %175 = getelementptr inbounds nuw i8, ptr %7, i64 8, !dbg !63925
  %176 = call { i64, i32 } @_RNvMNtCs9ec1k27omRZ_3std4timeNtB2_7Instant7elapsed(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %175), !dbg !63925
  %177 = extractvalue { i64, i32 } %176, 0, !dbg !63925
  %178 = extractvalue { i64, i32 } %176, 1, !dbg !63925
    #dbg_value(ptr poison, !27684, !DIExpression(), !63927)
  %179 = mul i64 %177, 1000000000, !dbg !63928
  %180 = icmp ult i32 %178, 1000000000, !dbg !63929
  call void @llvm.assume(i1 %180), !dbg !63929
  %181 = zext nneg i32 %178 to i64, !dbg !63928
  %182 = add i64 %179, %181, !dbg !63928
    #dbg_value(i64 %182, !27660, !DIExpression(), !63917)
    #dbg_value(i64 %182, !27671, !DIExpression(), !63919)
    #dbg_value(ptr %174, !27670, !DIExpression(), !63919)
  %183 = atomicrmw add ptr %174, i64 %182 monotonic, align 8, !dbg !63931, !noalias !63926
  %184 = getelementptr inbounds nuw { { { i64 } } }, ptr @_RNvNtNtCs97f5S1uJLkH_9game_core10simulation4prof11PHASE_CALLS, i64 %170, !dbg !63932
    #dbg_value(ptr %184, !27659, !DIExpression(), !63921)
    #dbg_value(ptr %184, !27670, !DIExpression(), !63923)
  %185 = atomicrmw add ptr %184, i64 1 monotonic, align 8, !dbg !63933, !noalias !63926
  br label %186, !dbg !63905

186:                                              ; preds = %173, %165
  call void @llvm.lifetime.end.p0(ptr nonnull %7), !dbg !63808
  ret i64 %166, !dbg !63808

187:                                              ; preds = %159
  %188 = mul i64 %145, 100, !dbg !63869
  %189 = udiv i64 %188, %161, !dbg !63869
  %190 = call noundef range(i64 1, 0) i64 @llvm.umax.i64(i64 %189, i64 1), !dbg !63934
  call void @llvm.experimental.noalias.scope.decl(metadata !63936), !dbg !63833
    #dbg_value(i64 %71, !63838, !DIExpression(), !63939)
    #dbg_value(ptr %2, !63843, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !63939)
    #dbg_value(ptr poison, !63843, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !63939)
    #dbg_value(ptr %4, !63844, !DIExpression(), !63939)
    #dbg_value(i32 %95, !63845, !DIExpression(), !63939)
    #dbg_value(i32 1, !63846, !DIExpression(), !63939)
    #dbg_value(i64 100, !63871, !DIExpression(), !63941)
    #dbg_value(i64 100, !63871, !DIExpression(), !63943)
  %191 = invoke noundef nonnull align 8 ptr %97(ptr noundef nonnull %2)
          to label %192 unwind label %47, !dbg !63939

192:                                              ; preds = %187
    #dbg_value(ptr %4, !63857, !DIExpression(DW_OP_plus_uconst, 1560, DW_OP_stack_value), !63945)
    #dbg_value(ptr %191, !63859, !DIExpression(), !63946)
  %193 = getelementptr inbounds nuw i8, ptr %4, i64 1592, !dbg !63947
  %194 = load i64, ptr %193, align 8, !dbg !63947, !alias.scope !63936, !noundef !8
    #dbg_value(i64 %194, !63861, !DIExpression(), !63948)
    #dbg_value(i64 %194, !63865, !DIExpression(), !63949)
  %195 = invoke noundef nonnull align 8 ptr %99(ptr noundef nonnull %2)
          to label %196 unwind label %47, !dbg !63950

196:                                              ; preds = %192
    #dbg_value(ptr %195, !63863, !DIExpression(), !63951)
  switch i32 %95, label %211 [
    i32 0, label %197
    i32 2, label %221
    i32 3, label %221
  ], !dbg !63952

197:                                              ; preds = %196
  %198 = getelementptr inbounds nuw i8, ptr %195, i64 208, !dbg !63953
  %199 = load i64, ptr %198, align 8, !dbg !63953, !noalias !63936, !noundef !8
  %200 = icmp eq i64 %199, 0, !dbg !63953
  br i1 %200, label %226, label %201, !dbg !63953

201:                                              ; preds = %197
  %202 = getelementptr inbounds nuw i8, ptr %4, i64 1576, !dbg !63954
  %203 = load i64, ptr %202, align 8, !dbg !63954, !alias.scope !63936, !noundef !8
  %204 = mul i64 %203, %199, !dbg !63954
  %205 = udiv i64 %204, 100, !dbg !63954
  %206 = add i64 %205, %71, !dbg !63954
    #dbg_value(i64 %206, !63838, !DIExpression(), !63939)
  br label %226, !dbg !63953

207:                                              ; preds = %215, %211
  %208 = phi i64 [ %71, %211 ], [ %220, %215 ]
    #dbg_value(i64 %208, !63838, !DIExpression(), !63939)
  %209 = and i32 %95, 6, !dbg !63955
  %210 = icmp eq i32 %209, 2, !dbg !63955
  br i1 %210, label %221, label %226, !dbg !63955

211:                                              ; preds = %196
  %212 = getelementptr inbounds nuw i8, ptr %195, i64 224, !dbg !63956
  %213 = load i64, ptr %212, align 8, !dbg !63956, !noalias !63936, !noundef !8
  %214 = icmp eq i64 %213, 0, !dbg !63956
  br i1 %214, label %207, label %215, !dbg !63956

215:                                              ; preds = %211
  %216 = getelementptr inbounds nuw i8, ptr %4, i64 1576, !dbg !63957
  %217 = load i64, ptr %216, align 8, !dbg !63957, !alias.scope !63936, !noundef !8
  %218 = mul i64 %217, %213, !dbg !63957
  %219 = udiv i64 %218, 100, !dbg !63957
  %220 = add i64 %219, %71, !dbg !63957
    #dbg_value(i64 %220, !63838, !DIExpression(), !63939)
  br label %207, !dbg !63956

221:                                              ; preds = %207, %196, %196
  %222 = phi i64 [ %71, %196 ], [ %208, %207 ], [ %71, %196 ]
    #dbg_value(i64 %222, !63838, !DIExpression(), !63939)
  %223 = getelementptr inbounds nuw i8, ptr %195, i64 240, !dbg !63958
  %224 = load i64, ptr %223, align 8, !dbg !63958, !noalias !63936, !noundef !8
  %225 = icmp eq i64 %224, 0, !dbg !63958
  br i1 %225, label %235, label %231, !dbg !63958

226:                                              ; preds = %207, %201, %197
  %227 = phi i64 [ %208, %207 ], [ %206, %201 ], [ %71, %197 ]
  %228 = getelementptr inbounds nuw i8, ptr %195, i64 216, !dbg !63959
  %229 = load i64, ptr %228, align 8, !dbg !63959, !noalias !63936, !noundef !8
  %230 = icmp eq i64 %229, 0, !dbg !63959
  br i1 %230, label %235, label %240, !dbg !63959

231:                                              ; preds = %221
  %232 = add i64 %224, 100, !dbg !63960
  %233 = mul i64 %232, %222, !dbg !63960
  %234 = udiv i64 %233, 100, !dbg !63960
    #dbg_value(i64 %234, !63838, !DIExpression(), !63939)
  br label %235, !dbg !63958

235:                                              ; preds = %240, %231, %226, %221
  %236 = phi i64 [ %227, %226 ], [ %245, %240 ], [ %222, %221 ], [ %234, %231 ]
    #dbg_value(i64 %236, !63838, !DIExpression(), !63939)
  %237 = getelementptr inbounds nuw i8, ptr %195, i64 176, !dbg !63961
  %238 = load i64, ptr %237, align 8, !dbg !63961, !noalias !63936, !noundef !8
    #dbg_value(i64 %238, !63874, !DIExpression(), !63943)
  %239 = icmp eq i64 %238, 0, !dbg !63961
  br i1 %239, label %250, label %246, !dbg !63961

240:                                              ; preds = %226
  %241 = getelementptr inbounds nuw i8, ptr %191, i64 16, !dbg !63962
  %242 = load i64, ptr %241, align 8, !dbg !63962, !noalias !63936, !noundef !8
  %243 = mul i64 %242, %229, !dbg !63962
  %244 = udiv i64 %243, 100, !dbg !63962
  %245 = add i64 %244, %227, !dbg !63962
    #dbg_value(i64 %245, !63838, !DIExpression(), !63939)
  br label %235, !dbg !63959

246:                                              ; preds = %235
  %247 = call i64 @llvm.usub.sat.i64(i64 100, i64 %238), !dbg !63963
  %248 = mul i64 %247, %194, !dbg !63964
  %249 = udiv i64 %248, 100, !dbg !63964
    #dbg_value(i64 %249, !63861, !DIExpression(), !63948)
    #dbg_value(i64 %249, !63865, !DIExpression(), !63949)
  br label %250, !dbg !63961

250:                                              ; preds = %246, %235
  %251 = phi i64 [ %194, %235 ], [ %249, %246 ], !dbg !63946
    #dbg_value(i64 %251, !63865, !DIExpression(), !63949)
    #dbg_value(i64 %251, !63861, !DIExpression(), !63948)
  %252 = add i64 %251, 100, !dbg !63965
  %253 = icmp eq i64 %252, 0, !dbg !63965
  br i1 %253, label %254, label %256, !dbg !63965

254:                                              ; preds = %250
  invoke void @_RNvNtNtCsjihNppCmMEE_4core9panicking11panic_const23panic_const_div_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @anon.92df479ad083757a8f3d2193749aeb7a.11) #44
          to label %255 unwind label %47, !dbg !63965

255:                                              ; preds = %254
  unreachable, !dbg !63965

256:                                              ; preds = %250
  %257 = mul i64 %236, 100, !dbg !63939
  %258 = udiv i64 %257, %252, !dbg !63939
  %259 = call noundef range(i64 1, 0) i64 @llvm.umax.i64(i64 %258, i64 1), !dbg !63966
  %260 = add i64 %259, %190, !dbg !63833
  br label %165, !dbg !63832
}

; Function Attrs: uwtable
