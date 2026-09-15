 37842| define internal fastcc i16 @_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster20count_nearby_enemies(i64 %0, i64 %1, ptr %2, ptr %3, ptr %4) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 37847|     ;; player = ptr %2
 37849|     ;; count = i16 0
 37850|  %6 = gep %2, i64 2352                                                                                                 ;L832
 37851|  %7 = load i64, ptr %6, , !!8                                                                                          ;L832
 37852|  %8 = sub i64 1, %7                                                                                                    ;L832
 37853|  %9 = icmp ult i64 %8, 2                                                                                               ;L832
 37854|  br i1 %9, label %10, label %19                                                                                        ;L832
 37855| 
 37856| 10: ; preds = %5
 37857|  %11 = icmp ne ptr %3, null
 37858|  tail call void @llvm.assume(i1 %11)
 37859|  %12 = gep %3, i64 480                                                                                                 ;L832
 37860|  %13 = getelementptr [5 x ptr], ptr %12, i64 %8                                                                        ;L832
 37861|     ;; iter[0..+8] = ptr %13
 37862|     ;; iter[8..+8] = ptr %13
 37863|     ;; iter[16..+8] = i64 0
 37864|  %14 = icmp ne ptr %4, null
 37865|  %15 = getelementptr { { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, [5 x { i64, [2 x i64] }], [5 x { i64, { i8, [23 x i8] } }], [5 x { i8, [7 x i8], i64 }], [5 x i64], [5 x i64], [5 x i64], [5 x { i64, i64 }], i64, i64, i64, i64, i64, [5 x i32], i8, [3 x i8] }, ptr %4, i64 %8
 37866|  %16 = gep %3, i64 8
 37867|     ;; count = i16 0
 37868|     ;; iter[0..+8] = !DIArgList(ptr %13, i64 8)
 37871|     ;; c = ptr %13
 37872|  %17 = load ptr, ptr %13, , !!8                                                                                        ;L833
 37873|  %18 = icmp eq ptr %17, null                                                                                           ;L833
 37874|  br i1 %18, label %20, label %134                                                                                      ;L833
 37875| 
 37876| 19: ; preds = %5
 37877|  tail call void @core::panicking18panic_bounds_check(i64 %8, i64 2, ptr @anon.b0108feec1ab8ff62b7a37c1a95c251f.151) #35 ;L832
 37878|  unreachable                                                                                                           ;L832
 37879| 
 37880| 20: ; preds = %138, %134, %10
 37881|  %21 = phi i16 [ 0, %10 ], [ %155, %138 ], [ 0, %134 ]                                                                 ;L0
 37882|     ;; count = i16 %21
 37883|     ;; iter[0..+8] = !DIArgList(ptr %13, i64 8)
 37885|     ;; self = ptr undef
 37886|     ;; self = ptr undef
 37887|     ;; count = i64 1
 37888|     ;; ptr = !DIArgList(ptr %13, i64 8)
 37889|     ;; self = !DIArgList(ptr %13, i64 8)
 37890|     ;; end_or_len = ptr %13
 37891|  %22 = gep %13, i64 8                                                                                                  ;L656<185<80<832
 37892|     ;; iter[0..+8] = !DIArgList(ptr %13, i64 16)
 37895|     ;; c = ptr %22
 37896|  %23 = load ptr, ptr %22, , !!8                                                                                        ;L833
 37897|  %24 = icmp eq ptr %23, null                                                                                           ;L833
 37898|  br i1 %24, label %48, label %25                                                                                       ;L833
 37899| 
 37900| 25: ; preds = %20
 37902|  tail call void @llvm.assume(i1 %14)
 37903|  %26 = load ptr, ptr %3, , !!8, !!8                                                                                    ;L834
 37904|  %27 = load ptr, ptr %16, , !!8, !!8                                                                                   ;L834
 37905|     ;; self = ptr %23
 37906|  %28 = tail call zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %15, ptr %26, ptr %27, ptr %2, ptr %23) ;L834
 37907|  br i1 %28, label %29, label %48                                                                                       ;L834
 37908| 
 37909| 29: ; preds = %25
 37910|  %30 = gep %23, i64 1632                                                                                               ;L2158<835
 37911|  %31 = load i64, ptr %30, , !!8                                                                                        ;L2158<835
 37912|     ;; x1 = i64 %31
 37913|     ;; self = i64 %31
 37914|  %32 = gep %23, i64 1640                                                                                               ;L2158<835
 37915|  %33 = load i64, ptr %32, , !!8                                                                                        ;L2158<835
 37916|     ;; y1 = i64 %33
 37917|     ;; self = i64 %33
 37918|     ;; x2 = i64 %0
 37919|     ;; other = i64 %0
 37920|     ;; y2 = i64 %1
 37921|     ;; other = i64 %1
 37922|  %34 = icmp ult i64 %31, %0                                                                                            ;L3147<7<2158<835
 37923|  %35 = sub nuw i64 %0, %31                                                                                             ;L3147<7<2158<835
 37924|  %36 = sub nuw i64 %31, %0                                                                                             ;L3147<7<2158<835
 37925|  %37 = select i1 %34, i64 %35, i64 %36                                                                                 ;L3147<7<2158<835
 37926|     ;; dx = i64 %37
 37927|  %38 = icmp ult i64 %33, %1                                                                                            ;L3147<8<2158<835
 37928|  %39 = sub nuw i64 %1, %33                                                                                             ;L3147<8<2158<835
 37929|  %40 = sub nuw i64 %33, %1                                                                                             ;L3147<8<2158<835
 37930|  %41 = select i1 %38, i64 %39, i64 %40                                                                                 ;L3147<8<2158<835
 37931|     ;; dy = i64 %41
 37932|  %42 = mul i64 %37, %37                                                                                                ;L9<2158<835
 37933|  %43 = mul i64 %41, %41                                                                                                ;L9<2158<835
 37934|  %44 = add i64 %43, %42                                                                                                ;L9<2158<835
 37935|  %45 = icmp ult i64 %44, 40000000001                                                                                   ;L835
 37936|  %46 = or disjoint i16 %21, 2
 37937|  %47 = select i1 %45, i16 %46, i16 %21                                                                                 ;L835
 37938|  br label %48                                                                                                          ;L835
 37939| 
 37940| 48: ; preds = %29, %25, %20
 37941|  %49 = phi i16 [ %21, %20 ], [ %47, %29 ], [ %21, %25 ]                                                                ;L0
 37942|     ;; count = i16 %49
 37943|     ;; iter[0..+8] = !DIArgList(ptr %13, i64 16)
 37945|     ;; self = ptr undef
 37946|     ;; self = ptr undef
 37947|     ;; count = i64 1
 37948|     ;; ptr = !DIArgList(ptr %13, i64 16)
 37949|     ;; self = !DIArgList(ptr %13, i64 16)
 37950|     ;; end_or_len = ptr %13
 37951|  %50 = gep %13, i64 16                                                                                                 ;L656<185<80<832
 37952|     ;; iter[0..+8] = !DIArgList(ptr %13, i64 24)
 37955|     ;; c = ptr %50
 37956|  %51 = load ptr, ptr %50, , !!8                                                                                        ;L833
 37957|  %52 = icmp eq ptr %51, null                                                                                           ;L833
 37958|  br i1 %52, label %76, label %53                                                                                       ;L833
 37959| 
 37960| 53: ; preds = %48
 37962|  tail call void @llvm.assume(i1 %14)
 37963|  %54 = load ptr, ptr %3, , !!8, !!8                                                                                    ;L834
 37964|  %55 = load ptr, ptr %16, , !!8, !!8                                                                                   ;L834
 37965|     ;; self = ptr %51
 37966|  %56 = tail call zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %15, ptr %54, ptr %55, ptr %2, ptr %51) ;L834
 37967|  br i1 %56, label %57, label %76                                                                                       ;L834
 37968| 
 37969| 57: ; preds = %53
 37970|  %58 = gep %51, i64 1632                                                                                               ;L2158<835
 37971|  %59 = load i64, ptr %58, , !!8                                                                                        ;L2158<835
 37972|     ;; x1 = i64 %59
 37973|     ;; self = i64 %59
 37974|  %60 = gep %51, i64 1640                                                                                               ;L2158<835
 37975|  %61 = load i64, ptr %60, , !!8                                                                                        ;L2158<835
 37976|     ;; y1 = i64 %61
 37977|     ;; self = i64 %61
 37978|     ;; x2 = i64 %0
 37979|     ;; other = i64 %0
 37980|     ;; y2 = i64 %1
 37981|     ;; other = i64 %1
 37982|  %62 = icmp ult i64 %59, %0                                                                                            ;L3147<7<2158<835
 37983|  %63 = sub nuw i64 %0, %59                                                                                             ;L3147<7<2158<835
 37984|  %64 = sub nuw i64 %59, %0                                                                                             ;L3147<7<2158<835
 37985|  %65 = select i1 %62, i64 %63, i64 %64                                                                                 ;L3147<7<2158<835
 37986|     ;; dx = i64 %65
 37987|  %66 = icmp ult i64 %61, %1                                                                                            ;L3147<8<2158<835
 37988|  %67 = sub nuw i64 %1, %61                                                                                             ;L3147<8<2158<835
 37989|  %68 = sub nuw i64 %61, %1                                                                                             ;L3147<8<2158<835
 37990|  %69 = select i1 %66, i64 %67, i64 %68                                                                                 ;L3147<8<2158<835
 37991|     ;; dy = i64 %69
 37992|  %70 = mul i64 %65, %65                                                                                                ;L9<2158<835
 37993|  %71 = mul i64 %69, %69                                                                                                ;L9<2158<835
 37994|  %72 = add i64 %71, %70                                                                                                ;L9<2158<835
 37995|  %73 = icmp ult i64 %72, 40000000001                                                                                   ;L835
 37996|  %74 = or i16 %49, 4
 37997|  %75 = select i1 %73, i16 %74, i16 %49                                                                                 ;L835
 37998|  br label %76                                                                                                          ;L835
 37999| 
 38000| 76: ; preds = %57, %53, %48
 38001|  %77 = phi i16 [ %49, %48 ], [ %75, %57 ], [ %49, %53 ]                                                                ;L0
 38002|     ;; count = i16 %77
 38003|     ;; iter[0..+8] = !DIArgList(ptr %13, i64 24)
 38005|     ;; self = ptr undef
 38006|     ;; self = ptr undef
 38007|     ;; count = i64 1
 38008|     ;; ptr = !DIArgList(ptr %13, i64 24)
 38009|     ;; self = !DIArgList(ptr %13, i64 24)
 38010|     ;; end_or_len = ptr %13
 38011|  %78 = gep %13, i64 24                                                                                                 ;L656<185<80<832
 38012|     ;; iter[0..+8] = !DIArgList(ptr %13, i64 32)
 38015|     ;; c = ptr %78
 38016|  %79 = load ptr, ptr %78, , !!8                                                                                        ;L833
 38017|  %80 = icmp eq ptr %79, null                                                                                           ;L833
 38018|  br i1 %80, label %104, label %81                                                                                      ;L833
 38019| 
 38020| 81: ; preds = %76
 38022|  tail call void @llvm.assume(i1 %14)
 38023|  %82 = load ptr, ptr %3, , !!8, !!8                                                                                    ;L834
 38024|  %83 = load ptr, ptr %16, , !!8, !!8                                                                                   ;L834
 38025|     ;; self = ptr %79
 38026|  %84 = tail call zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %15, ptr %82, ptr %83, ptr %2, ptr %79) ;L834
 38027|  br i1 %84, label %85, label %104                                                                                      ;L834
 38028| 
 38029| 85: ; preds = %81
 38030|  %86 = gep %79, i64 1632                                                                                               ;L2158<835
 38031|  %87 = load i64, ptr %86, , !!8                                                                                        ;L2158<835
 38032|     ;; x1 = i64 %87
 38033|     ;; self = i64 %87
 38034|  %88 = gep %79, i64 1640                                                                                               ;L2158<835
 38035|  %89 = load i64, ptr %88, , !!8                                                                                        ;L2158<835
 38036|     ;; y1 = i64 %89
 38037|     ;; self = i64 %89
 38038|     ;; x2 = i64 %0
 38039|     ;; other = i64 %0
 38040|     ;; y2 = i64 %1
 38041|     ;; other = i64 %1
 38042|  %90 = icmp ult i64 %87, %0                                                                                            ;L3147<7<2158<835
 38043|  %91 = sub nuw i64 %0, %87                                                                                             ;L3147<7<2158<835
 38044|  %92 = sub nuw i64 %87, %0                                                                                             ;L3147<7<2158<835
 38045|  %93 = select i1 %90, i64 %91, i64 %92                                                                                 ;L3147<7<2158<835
 38046|     ;; dx = i64 %93
 38047|  %94 = icmp ult i64 %89, %1                                                                                            ;L3147<8<2158<835
 38048|  %95 = sub nuw i64 %1, %89                                                                                             ;L3147<8<2158<835
 38049|  %96 = sub nuw i64 %89, %1                                                                                             ;L3147<8<2158<835
 38050|  %97 = select i1 %94, i64 %95, i64 %96                                                                                 ;L3147<8<2158<835
 38051|     ;; dy = i64 %97
 38052|  %98 = mul i64 %93, %93                                                                                                ;L9<2158<835
 38053|  %99 = mul i64 %97, %97                                                                                                ;L9<2158<835
 38054|  %100 = add i64 %99, %98                                                                                               ;L9<2158<835
 38055|  %101 = icmp ult i64 %100, 40000000001                                                                                 ;L835
 38056|  %102 = or i16 %77, 8
 38057|  %103 = select i1 %101, i16 %102, i16 %77                                                                              ;L835
 38058|  br label %104                                                                                                         ;L835
 38059| 
 38060| 104: ; preds = %85, %81, %76
 38061|  %105 = phi i16 [ %77, %76 ], [ %103, %85 ], [ %77, %81 ]                                                              ;L0
 38062|     ;; count = i16 %105
 38063|     ;; iter[0..+8] = !DIArgList(ptr %13, i64 32)
 38065|     ;; self = ptr undef
 38066|     ;; self = ptr undef
 38067|     ;; count = i64 1
 38068|     ;; ptr = !DIArgList(ptr %13, i64 32)
 38069|     ;; self = !DIArgList(ptr %13, i64 32)
 38070|     ;; end_or_len = ptr %13
 38071|  %106 = gep %13, i64 32                                                                                                ;L656<185<80<832
 38072|     ;; iter[0..+8] = !DIArgList(ptr %13, i64 40)
 38075|     ;; c = ptr %106
 38076|  %107 = load ptr, ptr %106, , !!8                                                                                      ;L833
 38077|  %108 = icmp eq ptr %107, null                                                                                         ;L833
 38078|  br i1 %108, label %132, label %109                                                                                    ;L833
 38079| 
 38080| 109: ; preds = %104
 38082|  tail call void @llvm.assume(i1 %14)
 38083|  %110 = load ptr, ptr %3, , !!8, !!8                                                                                   ;L834
 38084|  %111 = load ptr, ptr %16, , !!8, !!8                                                                                  ;L834
 38085|     ;; self = ptr %107
 38086|  %112 = tail call zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %15, ptr %110, ptr %111, ptr %2, ptr %107) ;L834
 38087|  br i1 %112, label %113, label %132                                                                                    ;L834
 38088| 
 38089| 113: ; preds = %109
 38090|  %114 = gep %107, i64 1632                                                                                             ;L2158<835
 38091|  %115 = load i64, ptr %114, , !!8                                                                                      ;L2158<835
 38092|     ;; x1 = i64 %115
 38093|     ;; self = i64 %115
 38094|  %116 = gep %107, i64 1640                                                                                             ;L2158<835
 38095|  %117 = load i64, ptr %116, , !!8                                                                                      ;L2158<835
 38096|     ;; y1 = i64 %117
 38097|     ;; self = i64 %117
 38098|     ;; x2 = i64 %0
 38099|     ;; other = i64 %0
 38100|     ;; y2 = i64 %1
 38101|     ;; other = i64 %1
 38102|  %118 = icmp ult i64 %115, %0                                                                                          ;L3147<7<2158<835
 38103|  %119 = sub nuw i64 %0, %115                                                                                           ;L3147<7<2158<835
 38104|  %120 = sub nuw i64 %115, %0                                                                                           ;L3147<7<2158<835
 38105|  %121 = select i1 %118, i64 %119, i64 %120                                                                             ;L3147<7<2158<835
 38106|     ;; dx = i64 %121
 38107|  %122 = icmp ult i64 %117, %1                                                                                          ;L3147<8<2158<835
 38108|  %123 = sub nuw i64 %1, %117                                                                                           ;L3147<8<2158<835
 38109|  %124 = sub nuw i64 %117, %1                                                                                           ;L3147<8<2158<835
 38110|  %125 = select i1 %122, i64 %123, i64 %124                                                                             ;L3147<8<2158<835
 38111|     ;; dy = i64 %125
 38112|  %126 = mul i64 %121, %121                                                                                             ;L9<2158<835
 38113|  %127 = mul i64 %125, %125                                                                                             ;L9<2158<835
 38114|  %128 = add i64 %127, %126                                                                                             ;L9<2158<835
 38115|  %129 = icmp ult i64 %128, 40000000001                                                                                 ;L835
 38116|  %130 = or i16 %105, 16
 38117|  %131 = select i1 %129, i16 %130, i16 %105                                                                             ;L835
 38118|  br label %132                                                                                                         ;L835
 38119| 
 38120| 132: ; preds = %113, %109, %104
 38121|  %133 = phi i16 [ %105, %104 ], [ %131, %113 ], [ %105, %109 ]                                                         ;L0
 38122|     ;; count = i16 %133
 38123|     ;; iter[0..+8] = !DIArgList(ptr %13, i64 40)
 38125|     ;; self = ptr undef
 38126|     ;; self = ptr undef
 38127|     ;; count = i64 1
 38128|     ;; ptr = !DIArgList(ptr %13, i64 40)
 38129|     ;; self = !DIArgList(ptr %13, i64 40)
 38130|     ;; end_or_len = ptr %13
 38131|  ret i16 %133                                                                                                          ;L841
 38132| 
 38133| 134: ; preds = %10
 38135|  tail call void @llvm.assume(i1 %14)
 38136|  %135 = load ptr, ptr %3, , !!8, !!8                                                                                   ;L834
 38137|  %136 = load ptr, ptr %16, , !!8, !!8                                                                                  ;L834
 38138|     ;; self = ptr %17
 38139|  %137 = tail call zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %15, ptr %135, ptr %136, ptr %2, ptr %17) ;L834
 38140|  br i1 %137, label %138, label %20                                                                                     ;L834
 38141| 
 38142| 138: ; preds = %134
 38143|  %139 = gep %17, i64 1632                                                                                              ;L2158<835
 38144|  %140 = load i64, ptr %139, , !!8                                                                                      ;L2158<835
 38145|     ;; x1 = i64 %140
 38146|     ;; self = i64 %140
 38147|  %141 = gep %17, i64 1640                                                                                              ;L2158<835
 38148|  %142 = load i64, ptr %141, , !!8                                                                                      ;L2158<835
 38149|     ;; y1 = i64 %142
 38150|     ;; self = i64 %142
 38151|     ;; x2 = i64 %0
 38152|     ;; other = i64 %0
 38153|     ;; y2 = i64 %1
 38154|     ;; other = i64 %1
 38155|  %143 = icmp ult i64 %140, %0                                                                                          ;L3147<7<2158<835
 38156|  %144 = sub nuw i64 %0, %140                                                                                           ;L3147<7<2158<835
 38157|  %145 = sub nuw i64 %140, %0                                                                                           ;L3147<7<2158<835
 38158|  %146 = select i1 %143, i64 %144, i64 %145                                                                             ;L3147<7<2158<835
 38159|     ;; dx = i64 %146
 38160|  %147 = icmp ult i64 %142, %1                                                                                          ;L3147<8<2158<835
 38161|  %148 = sub nuw i64 %1, %142                                                                                           ;L3147<8<2158<835
 38162|  %149 = sub nuw i64 %142, %1                                                                                           ;L3147<8<2158<835
 38163|  %150 = select i1 %147, i64 %148, i64 %149                                                                             ;L3147<8<2158<835
 38164|     ;; dy = i64 %150
 38165|  %151 = mul i64 %146, %146                                                                                             ;L9<2158<835
 38166|  %152 = mul i64 %150, %150                                                                                             ;L9<2158<835
 38167|  %153 = add i64 %152, %151                                                                                             ;L9<2158<835
 38168|  %154 = icmp ult i64 %153, 40000000001                                                                                 ;L835
 38169|  %155 = zext i1 %154 to i16                                                                                            ;L835
 38170|  br label %20                                                                                                          ;L835
 38171| }
