  9581| define void @ai::plan_legacy8sub_plan10epic_checkNtB2_16EpicCheckSubPlan17action_candidates(ptr sret([32 x i8]) %0, ptr %1, i64 %2, ptr %3, ptr %4, ptr %5, ptr %6, ptr %7, ptr %8) unnamed_addr #1 personality ptr @__CxxFrameHandler3 {
  9582|  %10 = alloca [32 x i8],
  9583|  %11 = alloca [24 x i8],
  9584|  %12 = alloca [32 x i8],
  9585|  %13 = alloca [32 x i8],
  9586|  %14 = alloca [136 x i8],
  9587|  %15 = alloca [184 x i8],
  9594|  %16 = alloca [184 x i8],
  9595|  %17 = alloca [184 x i8],
  9596|  %18 = alloca [184 x i8],
  9597|  %19 = alloca [184 x i8],
  9598|  %20 = alloca [184 x i8],
  9599|  %21 = alloca [184 x i8],
  9600|  %22 = alloca [184 x i8],
  9601|  %23 = alloca [16 x i8],
  9602|  %24 = alloca [24 x i8],
  9603|  %25 = alloca [24 x i8],
  9604|  %26 = alloca [184 x i8],
  9605|  %27 = alloca [184 x i8],
  9606|  %28 = alloca [136 x i8],
  9607|  %29 = alloca [184 x i8],
  9608|  %30 = alloca [88 x i8],
  9609|  %31 = alloca [88 x i8],
  9610|  %32 = alloca [136 x i8],
  9611|  %33 = alloca [184 x i8],
  9612|  %34 = alloca [56 x i8],
  9617|  %35 = alloca [32 x i8],
  9621|     ;; version = i64 %2
  9622|     ;; self = ptr %1
  9623|     ;; rnd = ptr %3
  9624|     ;; player = ptr %4
  9625|     ;; data = ptr %5
  9626|     ;; parameter = ptr %6
  9627|     ;; team_plan = ptr %7
  9628|     ;; debug = ptr %8
  9629|     ;; res = ptr %35
  9630|     ;; position_score = ptr %34
  9631|     ;; posture = ptr %31
  9632|     ;; posture = ptr %30
  9633|     ;; value = ptr %24
  9634|     ;; raw = ptr %11
  9638|  %36 = gep %5, i64 8                                                                                                   ;L15
  9639|  %37 = load ptr, ptr %36, , !!8, !!8                                                                                   ;L15
  9640|     ;; context = ptr %37
  9641|     ;; context = ptr %37
  9642|     ;; context = ptr %37
  9643|  %38 = load ptr, ptr %37, , !!8, !!8                                                                                   ;L15
  9644|     ;; bump = ptr %38
  9645|  store ptr inttoptr (i64 8 to ptr), ptr %35,                                                                           ;L547<15
  9646|  %39 = gep %35, i64 8                                                                                                  ;L547<15
  9647|  store ptr %38, ptr %39,                                                                                               ;L547<15
  9648|  %40 = gep %35, i64 16                                                                                                 ;L547<15
  9649|  %41 = gep %35, i64 24                                                                                                 ;L547<15
  9650|  %42 = gep %4, i64 2352                                                                                                ;L17
  9651|  call void @llvm.memset.p0.i64(ptr %40, i8 0, i64 16, i1 false)                                                        ;L547<15
  9652|  %43 = load i64, ptr %42, , !!8                                                                                        ;L17
  9653|     ;; team = i64 %43
  9654|  %44 = icmp ult i64 %43, 2                                                                                             ;L17
  9655|  br i1 %44, label %49, label %45                                                                                       ;L17
  9656| 
  9657| 45: ; preds = %9
  9658|  invoke void @core::panicking18panic_bounds_check(i64 %43, i64 2, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.123) #31
  9659|  to label %48 unwind label %46                                                                                         ;L17
  9660| 
  9661| 46: ; preds = %490, %482, %477, %476, %475, %467, %458, %452, %414, %400, %390, %389, %387, %363, %341, %332, %298, %292, %273, %271, %261, %260, %259, %257, %244, %236, %225, %211, %208, %201, %194, %165, %156, %129, %128, %112, %110, %100, %88, %76, %69, %59, %45
  9662|  %47 = cleanuppad within none []
  9663|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEB1t_(ptr %35) #30 [ "funclet"(token %47) ] ;L104
  9664|  cleanupret from %47 unwind to caller                                                                                  ;L14
  9665| 
  9666| 48: ; preds = %59, %45
  9667|  unreachable
  9668| 
  9669| 49: ; preds = %9
  9670|     ;; self = ptr %4
  9671|  %50 = gep %4, i64 2496                                                                                                ;L581<17
  9672|  %51 = load i32, ptr %50, , !!8                                                                                        ;L581<17
  9673|  %52 = zext nneg i32 %51 to i64                                                                                        ;L581<17
  9674|  %53 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L17
  9675|     ;; self = ptr %53
  9676|     ;; self = ptr %53
  9677|  %54 = gep %53, i64 480                                                                                                ;L17
  9678|  %55 = getelementptr [5 x ptr], ptr %54, i64 %43                                                                       ;L17
  9679|  %56 = getelementptr ptr, ptr %55, i64 %52                                                                             ;L17
  9680|  %57 = load ptr, ptr %56, , !!8                                                                                        ;L17
  9681|     ;; self = ptr %57
  9682|     ;; self = ptr %57
  9683|  %58 = icmp eq ptr %57, null                                                                                           ;L1011<17
  9684|  br i1 %58, label %59, label %60                                                                                       ;L1011<17
  9685| 
  9686| 59: ; preds = %49
  9687|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.124) #31
  9688|  to label %48 unwind label %46                                                                                         ;L1013<17
  9689| 
  9690| 60: ; preds = %49
  9691|     ;; champ = ptr %57
  9692|     ;; champ = ptr %57
  9693|     ;; self = ptr %4
  9694|  %61 = sub nuw nsw i64 1, %43                                                                                          ;L23
  9695|     ;; team = i64 %61
  9696|     ;; team = i64 %61
  9697|  %62 = getelementptr [5 x ptr], ptr %54, i64 %61                                                                       ;L1905<23
  9698|     ;; self = ptr undef
  9699|     ;; self = ptr undef
  9700|     ;; f[0..+8] = ptr undef
  9701|     ;; f[8..+8] = ptr %4
  9702|     ;; f[16..+8] = ptr %5
  9703|     ;; f[24..+8] = ptr %57
  9704|     ;; fold[0..+8] = ptr undef
  9705|     ;; fold[8..+8] = ptr %4
  9706|     ;; fold[16..+8] = ptr %5
  9707|     ;; fold[24..+8] = ptr %57
  9710|     ;; f[8..+8] = ptr undef
  9711|     ;; f[16..+8] = ptr %4
  9712|     ;; f[24..+8] = ptr %5
  9713|     ;; f[32..+8] = ptr %57
  9714|     ;; self = ptr undef
  9717|     ;; self = ptr undef
  9718|     ;; count = i64 1
  9719|     ;; ptr = ptr %62
  9720|     ;; self = ptr %62
  9721|     ;; end_or_len = ptr %62
  9724|  br label %63                                                                                                          ;L180<2493<138<2897<23
  9725| 
  9726| 63: ; preds = %80, %60
  9727|  %64 = phi i64 [ 0, %60 ], [ %66, %80 ]
  9728|  %65 = gep %62, i64 %64                                                                                                ;L656<185<2493<138<2897<23
  9729|     ;; ptr = ptr %65
  9730|  %66 = add nuw nsw i64 %64, 8                                                                                          ;L656<185<2493<138<2897<23
  9731|     ;; x = ptr %65
  9732|  %67 = load ptr, ptr %65, , !!19550, !!8                                                                               ;L2494<138<2897<23
  9733|     ;; f = ptr undef
  9737|  %68 = icmp eq ptr %67, null                                                                                           ;L49<2494<138<2897<23
  9738|  br i1 %68, label %80, label %69                                                                                       ;L49<2494<138<2897<23
  9739| 
  9740| 69: ; preds = %63
  9741|     ;; x = ptr %67
  9744|     ;; x = ptr %67
  9749|     ;; c = ptr %67
  9750|     ;; self = ptr %67
  9751|     ;; self = ptr %67
  9752|     ;; self = ptr %67
  9753|     ;; self = ptr %67
  9754|     ;; self = ptr %67
  9755|  %70 = invoke zeroext i1 @ai::utils26nontarget_windup_perceived(i64 %2, ptr %4, ptr %5, ptr %67)
  9756|  to label %71 unwind label %46                                                                                         ;L24<2893<50<2494<138<2897<23
  9757| 
  9758| 71: ; preds = %69
  9759|  %72 = gep %67, i64 104
  9760|  %73 = load i64, ptr %72, , !!19613
  9761|  %74 = icmp eq i64 %73, 13
  9762|  %75 = select i1 %70, i1 %74, i1 false                                                                                 ;L24<2893<50<2494<138<2897<23
  9763|  br i1 %75, label %82, label %80                                                                                       ;L24<2893<50<2494<138<2897<23
  9764| 
  9765| 76: ; preds = %102, %102, %92, %92, %90
  9766|  %77 = phi ptr [ %97, %92 ], [ %91, %90 ], [ %97, %92 ], [ %107, %102 ], [ %107, %102 ]
  9767|  %78 = invoke zeroext i1 @gc::simulation6effectNtB2_6Effect11is_in_range(ptr %77, ptr %67, ptr %57)
  9768|  to label %79 unwind label %46                                                                                         ;L0<2893<50<2494<138<2897<23
  9769| 
  9770| 79: ; preds = %76
  9771|  br i1 %78, label %112, label %80                                                                                      ;L2494<138<2897<23
  9772| 
  9773| 80: ; preds = %102, %92, %85, %82, %79, %71, %63
  9774|     ;; self = ptr undef
  9775|     ;; count = i64 1
  9776|     ;; ptr = !DIArgList(ptr %62, i64 %66)
  9777|     ;; self = !DIArgList(ptr %62, i64 %66)
  9778|     ;; end_or_len = ptr %62
  9781|  %81 = icmp eq i64 %66, 40                                                                                             ;L1714<180<2493<138<2897<23
  9782|  br i1 %81, label %112, label %63                                                                                      ;L180<2493<138<2897<23
  9783| 
  9784| 82: ; preds = %71
  9785|     ;; champ = ptr %67
  9786|  %83 = gep %67, i64 112                                                                                                ;L1572<25<2893<50<2494<138<2897<23
  9787|  %84 = load i64, ptr %83, , !!19613, !!8                                                                               ;L1572<25<2893<50<2494<138<2897<23
  9788|  switch i64 %84, label %80 [
  9789|  i64 4, label %85
  9790|  i64 5, label %92
  9791|  i64 6, label %102
  9792|  ]                                                                                                                     ;L25<2893<50<2494<138<2897<23
  9793| 
  9794| 85: ; preds = %82
  9795|     ;; self = ptr %67
  9796|  %86 = gep %67, i64 1272                                                                                               ;L742<25<2893<50<2494<138<2897<23
  9797|  %87 = load i32, ptr %86, , !!19613, !!8                                                                               ;L742<25<2893<50<2494<138<2897<23
  9798|  switch i32 %87, label %80 [
  9799|  i32 -1, label %88
  9800|  i32 1, label %90
  9801|  i32 2, label %90
  9802|  ]                                                                                                                     ;L742<25<2893<50<2494<138<2897<23
  9803| 
  9804| 88: ; preds = %85
  9805|     ;; self = ptr null
  9806|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.18) #31
  9807|  to label %89 unwind label %46                                                                                         ;L1013<25<2893<50<2494<138<2897<23
  9808| 
  9809| 89: ; preds = %88
  9810|  unreachable                                                                                                           ;L1013<25<2893<50<2494<138<2897<23
  9811| 
  9812| 90: ; preds = %85, %85
  9813|  %91 = gep %67, i64 1224                                                                                               ;L742<25<2893<50<2494<138<2897<23
  9814|     ;; self = ptr %67
  9815|     ;; self = ptr %91
  9816|  br label %76                                                                                                          ;L25<2893<50<2494<138<2897<23
  9817| 
  9818| 92: ; preds = %82
  9819|  %93 = gep %67, i64 1480                                                                                               ;L1693<27<2893<50<2494<138<2897<23
  9820|  %94 = load i64, ptr %93, , !!19613, !!8                                                                               ;L1693<27<2893<50<2494<138<2897<23
  9821|  %95 = icmp ugt i64 %94, 2                                                                                             ;L1693<27<2893<50<2494<138<2897<23
  9822|  %96 = gep %67, i64 1280                                                                                               ;L1693<27<2893<50<2494<138<2897<23
  9823|  %97 = select i1 %95, ptr %96, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.19                                           ;L1693<27<2893<50<2494<138<2897<23
  9824|     ;; self = ptr %97
  9825|  %98 = gep %97, i64 48                                                                                                 ;L742<27<2893<50<2494<138<2897<23
  9826|  %99 = load i32, ptr %98, , !!19613, !!8                                                                               ;L742<27<2893<50<2494<138<2897<23
  9827|  switch i32 %99, label %80 [
  9828|  i32 -1, label %100
  9829|  i32 1, label %76
  9830|  i32 2, label %76
  9831|  ]                                                                                                                     ;L742<27<2893<50<2494<138<2897<23
  9832| 
  9833| 100: ; preds = %92
  9834|     ;; self = ptr null
  9835|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.20) #31
  9836|  to label %101 unwind label %46                                                                                        ;L1013<27<2893<50<2494<138<2897<23
  9837| 
  9838| 101: ; preds = %100
  9839|  unreachable                                                                                                           ;L1013<27<2893<50<2494<138<2897<23
  9840| 
  9841| 102: ; preds = %82
  9842|  %103 = gep %67, i64 1480                                                                                              ;L1701<29<2893<50<2494<138<2897<23
  9843|  %104 = load i64, ptr %103, , !!19613, !!8                                                                             ;L1701<29<2893<50<2494<138<2897<23
  9844|  %105 = icmp ugt i64 %104, 4                                                                                           ;L1701<29<2893<50<2494<138<2897<23
  9845|  %106 = gep %67, i64 1336                                                                                              ;L1701<29<2893<50<2494<138<2897<23
  9846|  %107 = select i1 %105, ptr %106, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.19                                        ;L1701<29<2893<50<2494<138<2897<23
  9847|     ;; self = ptr %107
  9848|  %108 = gep %107, i64 48                                                                                               ;L742<29<2893<50<2494<138<2897<23
  9849|  %109 = load i32, ptr %108, , !!19613, !!8                                                                             ;L742<29<2893<50<2494<138<2897<23
  9850|  switch i32 %109, label %80 [
  9851|  i32 -1, label %110
  9852|  i32 1, label %76
  9853|  i32 2, label %76
  9854|  ]                                                                                                                     ;L742<29<2893<50<2494<138<2897<23
  9855| 
  9856| 110: ; preds = %102
  9857|     ;; self = ptr null
  9858|  invoke void @core::option13unwrap_failed(ptr @anon.94acafa22d01e083ca1cc62f01598c8f.21) #31
  9859|  to label %111 unwind label %46                                                                                        ;L1013<29<2893<50<2494<138<2897<23
  9860| 
  9861| 111: ; preds = %110
  9862|  unreachable                                                                                                           ;L1013<29<2893<50<2494<138<2897<23
  9863| 
  9864| 112: ; preds = %80, %79
  9865|  %113 = phi i1 [ false, %80 ], [ true, %79 ]                                                                           ;L1714<180<2493<138<2897<23
  9866|     ;; has_non_target_action_range = i1 %113
  9868|  %114 = gep %6, i64 2544                                                                                               ;L36
  9869|  %115 = gep %57, i64 1632                                                                                              ;L37
  9870|  %116 = load i64, ptr %115, , !!8                                                                                      ;L37
  9871|     ;; x = i64 %116
  9872|     ;; x = i64 %116
  9873|     ;; x = i64 %116
  9874|     ;; x1 = i64 %116
  9875|     ;; self = i64 %116
  9876|     ;; x1 = i64 %116
  9877|     ;; self = i64 %116
  9878|  %117 = gep %57, i64 1640                                                                                              ;L37
  9879|  %118 = load i64, ptr %117, , !!8                                                                                      ;L37
  9880|     ;; y = i64 %118
  9881|     ;; y = i64 %118
  9882|     ;; y = i64 %118
  9883|     ;; y1 = i64 %118
  9884|     ;; self = i64 %118
  9885|     ;; y1 = i64 %118
  9886|     ;; self = i64 %118
  9887|  invoke void @ai::position_eval26position_score_at_position(ptr sret([56 x i8]) %34, i64 %2, ptr %4, ptr %5, ptr %114, i64 %116, i64 %118, i8 11)
  9888|  to label %119 unwind label %46                                                                                        ;L36
  9889| 
  9890| 119: ; preds = %112
  9891|  %120 = gep %34, i64 48                                                                                                ;L38
  9892|  %121 = load i8, ptr %120, , !!8                                                                                       ;L38
  9893|  %122 = trunc nuw i8 %121 to i1                                                                                        ;L38
  9894|  %123 = gep %34, i64 49                                                                                                ;L38
  9895|  %124 = load i8, ptr %123,                                                                                             ;L38
  9896|  %125 = trunc nuw i8 %124 to i1                                                                                        ;L38
  9897|     ;; on_trajectory = i1 %125
  9898|  %126 = or i1 %113, %125
  9899|  %127 = select i1 %122, i1 true, i1 %126                                                                               ;L38
  9900|  br i1 %127, label %128, label %129                                                                                    ;L38
  9901| 
  9902| 128: ; preds = %119
  9905|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %32, ptr %5, ptr %4, i64 5, i1 zeroext true)
  9906|  to label %488 unwind label %46                                                                                        ;L40
  9907| 
  9908| 129: ; preds = %119
  9911|  invoke void @ai::plan_legacy9team_plan20objective_disciplineNtB4_8TeamPlan21v25_objective_posture(ptr sret([88 x i8]) %31, ptr %7, i64 %2, ptr %4, ptr %5, i8 4)
  9912|  to label %130 unwind label %46                                                                                        ;L45
  9913| 
  9914| 130: ; preds = %129
  9915|  %131 = load i64, ptr %31, , !!8                                                                                       ;L46
  9916|  %132 = icmp eq i64 %131, -1                                                                                           ;L46
  9917|  br i1 %132, label %133, label %138                                                                                    ;L46
  9918| 
  9919| 133: ; preds = %153, %130
  9920|  %134 = phi ptr [ %154, %153 ], [ inttoptr (i64 8 to ptr), %130 ]
  9921|  %135 = phi i64 [ %155, %153 ], [ 0, %130 ]
  9922|  %136 = load i8, ptr %1, , !!8                                                                                         ;L65
  9923|  %137 = trunc nuw i8 %136 to i1                                                                                        ;L65
  9924|  br i1 %137, label %292, label %279                                                                                    ;L65
  9925| 
  9926| 138: ; preds = %130
  9928|  call void @llvm.memcpy.p0.p0.i64(ptr %30, ptr %31, i64 88, i1 false)                                                  ;L46
  9929|  %139 = gep %30, i64 80                                                                                                ;L47
  9930|  %140 = load i8, ptr %139, , !!8                                                                                       ;L47
  9932|  %141 = icmp samesign ugt i8 %140, 2                                                                                   ;L186<47
  9933|  br i1 %141, label %142, label %148                                                                                    ;L47
  9934| 
  9935| 142: ; preds = %138
  9936|  %143 = icmp eq i8 %140, 4                                                                                             ;L48
  9937|  %144 = gep %30, i64 56                                                                                                ;L48
  9938|  %145 = load i64, ptr %144,                                                                                            ;L48
  9939|  %146 = icmp ne i64 %145, 0                                                                                            ;L48
  9940|  %147 = select i1 %143, i1 %146, i1 false                                                                              ;L48
  9941|  br i1 %147, label %208, label %201                                                                                    ;L48
  9942| 
  9943| 148: ; preds = %138
  9945|  %149 = icmp eq i8 %140, 2                                                                                             ;L190<58
  9946|  %150 = load i64, ptr %30,
  9947|  %151 = trunc nuw i64 %150 to i1
  9948|  %152 = select i1 %149, i1 %151, i1 false                                                                              ;L58
  9949|  br i1 %152, label %156, label %153                                                                                    ;L58
  9950| 
  9951| 153: ; preds = %196, %148
  9952|  %154 = phi ptr [ inttoptr (i64 8 to ptr), %148 ], [ %197, %196 ]
  9953|  %155 = phi i64 [ 0, %148 ], [ %200, %196 ]
  9955|  br label %133                                                                                                         ;L46
  9956| 
  9957| 156: ; preds = %148
  9958|  %157 = gep %30, i64 8                                                                                                 ;L59
  9959|  %158 = load i64, ptr %157, , !!8                                                                                      ;L59
  9960|     ;; focus_enemy = i64 %158
  9962|     ;; data = ptr %5
  9963|     ;; target = i64 %158
  9964|     ;; end_delay = i64 5
  9965|     ;; data = ptr %5
  9966|     ;; target = i64 %158
  9967|     ;; end_delay = i64 5
  9968|     ;; attack_range_margin = i64 15000
  9970|     ;; default = i64 0
  9972|     ;; default = i64 0
  9973|  %159 = load ptr, ptr %53, , !!19750, !!8, !!8                                                                         ;L80<76<60
  9974|  %160 = gep %53, i64 8                                                                                                 ;L80<76<60
  9975|  %161 = load ptr, ptr %160, , !!19750, !!8, !!8                                                                        ;L80<76<60
  9976|  %162 = gep %161, i64 496                                                                                              ;L80<76<60
  9977|  %163 = load ptr, ptr %162, , !!19750, !!8                                                                             ;L80<76<60
  9978|  %164 = invoke ptr %163(ptr %159, i64 %158)
  9979|  to label %165 unwind label %46                                                                                        ;L80<76<60
  9980| 
  9981| 165: ; preds = %156
  9982|     ;; target_entity = ptr %164
  9983|     ;; self = ptr %164
  9984|  %166 = gep %161, i64 40                                                                                               ;L82<76<60
  9985|  %167 = load ptr, ptr %166, , !!19750, !!8                                                                             ;L82<76<60
  9986|  %168 = invoke i64 %167(ptr %159)
  9987|  to label %169 unwind label %46                                                                                        ;L82<76<60
  9988| 
  9989| 169: ; preds = %165
  9990|  %170 = icmp eq ptr %164, null                                                                                         ;L1161<84<76<60
  9991|  br i1 %170, label %176, label %171                                                                                    ;L1161<84<76<60
  9992| 
  9993| 171: ; preds = %169
  9994|     ;; x = ptr %164
  9995|     ;; t = ptr %164
  9996|  %172 = gep %164, i64 1632                                                                                             ;L84<1162<84<76<60
  9997|  %173 = load i64, ptr %172, , !!19750, !!8                                                                             ;L84<1162<84<76<60
  9998|     ;; self[8..+8] = i64 %173
  9999|     ;; self[0..+8] = i64 1
 10000|     ;; self = ptr %164
 10001|     ;; x = ptr %164
 10002|     ;; t = ptr %164
 10003|  %174 = gep %164, i64 1640                                                                                             ;L85<1162<85<76<60
 10004|  %175 = load i64, ptr %174, , !!19750, !!8                                                                             ;L85<1162<85<76<60
 10005|     ;; self[8..+8] = i64 %175
 10006|     ;; self[0..+8] = i64 1
 10007|  br label %176                                                                                                         ;L1043<85<76<60
 10008| 
 10009| 176: ; preds = %171, %169
 10010|  %177 = phi i64 [ %175, %171 ], [ 0, %169 ]                                                                            ;L0<85<76<60
 10011|  %178 = phi i64 [ %173, %171 ], [ 0, %169 ]                                                                            ;L0<84<76<60
 10012|  store i64 0, ptr %22,                                                                                                 ;L60
 10013|  %179 = gep %22, i64 85                                                                                                ;L60
 10014|  store i8 2, ptr %179,                                                                                                 ;L60
 10015|  %180 = gep %22, i64 88                                                                                                ;L60
 10016|  store i64 %168, ptr %180,                                                                                             ;L60
 10017|  %181 = gep %22, i64 96                                                                                                ;L60
 10018|  store i64 %158, ptr %181,                                                                                             ;L60
 10019|  %182 = gep %22, i64 104                                                                                               ;L60
 10020|  store i64 %178, ptr %182,                                                                                             ;L60
 10021|  %183 = gep %22, i64 112                                                                                               ;L60
 10022|  store i64 %177, ptr %183,                                                                                             ;L60
 10023|  %184 = gep %22, i64 120                                                                                               ;L60
 10024|  store i64 15000, ptr %184,                                                                                            ;L60
 10025|  %185 = gep %22, i64 128                                                                                               ;L60
 10026|  store i64 5, ptr %185,                                                                                                ;L60
 10027|  %186 = gep %22, i64 136                                                                                               ;L60
 10028|  store i64 0, ptr %186,                                                                                                ;L60
 10029|  %187 = gep %22, i64 144                                                                                               ;L60
 10030|  store i8 0, ptr %187,                                                                                                 ;L60
 10031|  %188 = gep %22, i64 145                                                                                               ;L60
 10032|  store i8 1, ptr %188,                                                                                                 ;L60
 10033|  %189 = gep %22, i64 146                                                                                               ;L60
 10034|  store i8 0, ptr %189,                                                                                                 ;L60
 10035|  %190 = gep %22, i64 147                                                                                               ;L60
 10036|  store i8 0, ptr %190,                                                                                                 ;L60
 10037|  %191 = gep %22, i64 148                                                                                               ;L60
 10038|  store i8 0, ptr %191,                                                                                                 ;L60
 10039|  %192 = gep %22, i64 149                                                                                               ;L60
 10040|  store i8 2, ptr %192,                                                                                                 ;L60
 10041|  %193 = gep %22, i64 177                                                                                               ;L60
 10042|  store i8 14, ptr %193,                                                                                                ;L60
 10044|     ;; self = ptr %35
 10045|     ;; self = ptr %35
 10046|     ;; value = ptr %22
 10047|     ;; src = ptr %22
 10048|     ;; additional = i64 1
 10049|     ;; needed_extra_cap = i64 1
 10050|     ;; needed_extra_cap = i64 1
 10051|     ;; strategy = i8 1
 10052|     ;; self = ptr %35
 10053|     ;; self = ptr %35
 10054|     ;; self = ptr %35
 10055|     ;; self = ptr %35
 10056|     ;; used_cap = i64 0
 10057|     ;; used_cap = i64 0
 10058|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %35, i64 0, i64 1, i1 zeroext true)
 10059|  to label %196 unwind label %194, !!19790                                                                              ;L619<430<738<1429<60
 10060| 
 10061| 194: ; preds = %176
 10062|  %195 = cleanuppad within none []
 10063|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %22) #30 [ "funclet"(token %195) ], !!19771 ;L1436<60
 10064|  cleanupret from %195 unwind label %46
 10065| 
 10066| 196: ; preds = %176
 10067|  %197 = load ptr, ptr %35, , !!19790                                                                                   ;L138<1432<60
 10068|  %198 = load i64, ptr %41, , !!19790                                                                                   ;L1432<60
 10069|     ;; self = ptr %35
 10070|     ;; self = ptr %197
 10071|     ;; count = i64 %198
 10072|  %199 = gepS %197, i64 %198                                                                                            ;L961<1432<60
 10073|     ;; end = ptr %199
 10074|     ;; dst = ptr %199
 10075|  call void @llvm.memcpy.p0.p0.i64(ptr %199, ptr %22, i64 184, i1 false), !!19771                                       ;L1933<1433<60
 10076|  %200 = add i64 %198, 1                                                                                                ;L1434<60
 10077|  store i64 %200, ptr %41, , !!19790                                                                                    ;L1434<60
 10079|  br label %153                                                                                                         ;L59
 10080| 
 10081| 201: ; preds = %213, %142
 10082|  %202 = phi ptr [ %214, %213 ], [ inttoptr (i64 8 to ptr), %142 ]
 10083|  %203 = phi i64 [ %217, %213 ], [ 0, %142 ]
 10086|  %204 = gep %30, i64 32                                                                                                ;L51
 10087|  %205 = load i64, ptr %204, , !!8                                                                                      ;L51
 10088|  %206 = gep %30, i64 40                                                                                                ;L51
 10089|  %207 = load i64, ptr %206, , !!8                                                                                      ;L51
 10090|  invoke void @ai::small_action6aroundNtB5_25SmallActionAroundPosition3new(ptr sret([184 x i8]) %26, ptr %3, ptr %5, i64 %205, i64 %207, i64 5)
 10091|  to label %218 unwind label %46                                                                                        ;L51
 10092| 
 10093| 208: ; preds = %142
 10096|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway14new_with_skill(ptr sret([136 x i8]) %28, ptr %5, ptr %4, i64 5, i1 zeroext false)
 10097|  to label %209 unwind label %46                                                                                        ;L49
 10098| 
 10099| 209: ; preds = %208
 10100|  call void @llvm.memcpy.p0.p0.i64(ptr %29, ptr %28, i64 136, i1 false)                                                 ;L49
 10101|  %210 = gep %29, i64 177                                                                                               ;L49
 10102|  store i8 3, ptr %210,                                                                                                 ;L49
 10105|     ;; self = ptr %35
 10106|     ;; self = ptr %35
 10107|     ;; value = ptr %29
 10108|     ;; src = ptr %29
 10109|     ;; additional = i64 1
 10110|     ;; needed_extra_cap = i64 1
 10111|     ;; needed_extra_cap = i64 1
 10112|     ;; strategy = i8 1
 10113|     ;; self = ptr %35
 10114|     ;; self = ptr %35
 10115|     ;; self = ptr %35
 10116|     ;; self = ptr %35
 10117|     ;; used_cap = i64 0
 10118|     ;; used_cap = i64 0
 10119|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %35, i64 0, i64 1, i1 zeroext true)
 10120|  to label %213 unwind label %211, !!19825                                                                              ;L619<430<738<1429<49
 10121| 
 10122| 211: ; preds = %209
 10123|  %212 = cleanuppad within none []
 10124|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %29) #30 [ "funclet"(token %212) ], !!19806 ;L1436<49
 10125|  cleanupret from %212 unwind label %46
 10126| 
 10127| 213: ; preds = %209
 10128|  %214 = load ptr, ptr %35, , !!19825                                                                                   ;L138<1432<49
 10129|  %215 = load i64, ptr %41, , !!19825                                                                                   ;L1432<49
 10130|     ;; self = ptr %35
 10131|     ;; self = ptr %214
 10132|     ;; count = i64 %215
 10133|  %216 = gepS %214, i64 %215                                                                                            ;L961<1432<49
 10134|     ;; end = ptr %216
 10135|     ;; dst = ptr %216
 10136|  call void @llvm.memcpy.p0.p0.i64(ptr %216, ptr %29, i64 184, i1 false), !!19806                                       ;L1933<1433<49
 10137|  %217 = add i64 %215, 1                                                                                                ;L1434<49
 10138|  store i64 %217, ptr %41, , !!19825                                                                                    ;L1434<49
 10140|  br label %201                                                                                                         ;L48
 10141| 
 10142| 218: ; preds = %201
 10143|  call void @llvm.memcpy.p0.p0.i64(ptr %27, ptr %26, i64 184, i1 false)                                                 ;L51
 10146|     ;; self = ptr %35
 10147|     ;; self = ptr %35
 10148|     ;; value = ptr %27
 10149|     ;; src = ptr %27
 10150|     ;; additional = i64 1
 10151|     ;; needed_extra_cap = i64 1
 10152|     ;; needed_extra_cap = i64 1
 10153|     ;; strategy = i8 1
 10154|     ;; self = ptr %35
 10155|  %219 = load i64, ptr %40, , !!19855, !!8                                                                              ;L149<1428<51
 10156|  %220 = icmp eq i64 %203, %219                                                                                         ;L1428<51
 10157|  br i1 %220, label %221, label %227                                                                                    ;L1428<51
 10158| 
 10159| 221: ; preds = %218
 10160|     ;; self = ptr %35
 10161|     ;; self = ptr %35
 10162|     ;; self = ptr %35
 10163|     ;; used_cap = i64 %203
 10164|     ;; used_cap = i64 %203
 10165|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %35, i64 %203, i64 1, i1 zeroext true)
 10166|  to label %222 unwind label %225, !!19855                                                                              ;L619<430<738<1429<51
 10167| 
 10168| 222: ; preds = %221
 10169|  %223 = load i64, ptr %41, , !!19855                                                                                   ;L1432<51
 10170|  %224 = load ptr, ptr %35, , !!19855                                                                                   ;L138<1432<51
 10171|  br label %227                                                                                                         ;L619<430<738<1429<51
 10172| 
 10173| 225: ; preds = %221
 10174|  %226 = cleanuppad within none []
 10175|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %27) #30 [ "funclet"(token %226) ], !!19838 ;L1436<51
 10176|  cleanupret from %226 unwind label %46
 10177| 
 10178| 227: ; preds = %222, %218
 10179|  %228 = phi ptr [ %224, %222 ], [ %202, %218 ]                                                                         ;L138<1432<51
 10180|  %229 = phi i64 [ %223, %222 ], [ %203, %218 ]                                                                         ;L1432<51
 10181|     ;; self = ptr %35
 10182|     ;; self = ptr %228
 10183|     ;; count = i64 %229
 10184|  %230 = gepS %228, i64 %229                                                                                            ;L961<1432<51
 10185|     ;; end = ptr %230
 10186|     ;; dst = ptr %230
 10187|  call void @llvm.memcpy.p0.p0.i64(ptr %230, ptr %27, i64 184, i1 false), !!19838                                       ;L1933<1433<51
 10188|  %231 = add i64 %229, 1                                                                                                ;L1434<51
 10189|  store i64 %231, ptr %41, , !!19855                                                                                    ;L1434<51
 10191|  %232 = gep %37, i64 59                                                                                                ;L52
 10192|  %233 = load i8, ptr %232, , !!8                                                                                       ;L52
 10193|  %234 = trunc nuw i8 %233 to i1                                                                                        ;L52
 10194|  br i1 %234, label %236, label %235                                                                                    ;L52
 10195| 
 10196| 235: ; preds = %274, %227
 10197|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %35, i64 32, i1 false)                                                   ;L55
 10200|  br label %487                                                                                                         ;L1
 10201| 
 10202| 236: ; preds = %227
 10203|     ;; self = ptr %8
 10204|  %237 = gep %57, i64 1472                                                                                              ;L53
 10205|  %238 = load i64, ptr %237, , !!8                                                                                      ;L53
 10206|     ;; key = i64 %238
 10208|  %239 = gep %8, i64 160                                                                                                ;L1014<53
 10209|  invoke void @_RNvMNtCs5gUUnHMsxBL_9hashbrown11rustc_entryINtNtB4_3map7HashMapjINtNtCs9LexZzt9XJB_5alloc3vec3VecNtNtB15_6string6StringENtNtCs9EYcZKFYzm_5ahash12random_state11RandomStateE11rustc_entryCshdEBA0ozCnw_7game_ai(ptr sret([24 x i8]) %11, ptr %239, i64 %238)
 10210|  to label %240 unwind label %46                                                                                        ;L1014<53
 10211| 
 10212| 240: ; preds = %236
 10213|  %241 = load ptr, ptr %11, , !!8                                                                                       ;L3008<1014<53
 10214|  %242 = icmp eq ptr %241, null                                                                                         ;L3008<1014<53
 10215|  %243 = gep %11, i64 8                                                                                                 ;L0<1014<53
 10216|  br i1 %242, label %253, label %244                                                                                    ;L3008<1014<53
 10217| 
 10218| 244: ; preds = %240
 10219|  %245 = load i64, ptr %243,                                                                                            ;L3010<1014<53
 10220|  %246 = gep %11, i64 16                                                                                                ;L3010<1014<53
 10221|  %247 = load i64, ptr %246,                                                                                            ;L3010<1014<53
 10222|     ;; self[0..+8] = ptr %241
 10223|     ;; self[8..+8] = i64 %245
 10224|     ;; self[16..+8] = i64 %247
 10227|  store i64 0, ptr %25,                                                                                                 ;L464<53
 10228|  %248 = gep %25, i64 8                                                                                                 ;L464<53
 10229|  store ptr inttoptr (i64 8 to ptr), ptr %248,                                                                          ;L464<53
 10230|  %249 = gep %25, i64 16                                                                                                ;L464<53
 10231|  store i64 0, ptr %249,                                                                                                ;L464<53
 10232|     ;; default = ptr %25
 10235|     ;; entry[8..+8] = i64 %245
 10236|     ;; self[8..+8] = i64 %245
 10237|     ;; self[8..+8] = i64 %245
 10238|     ;; entry[16..+8] = i64 %247
 10239|     ;; self[16..+8] = i64 %247
 10240|     ;; self[16..+8] = i64 %247
 10241|     ;; entry[0..+8] = ptr %241
 10242|     ;; self[0..+8] = ptr %241
 10243|     ;; self[0..+8] = ptr %241
 10244|  %250 = gep %10, i64 8                                                                                                 ;L576<2911<2519<53
 10246|  call void @llvm.memcpy.p0.p0.i64(ptr %250, ptr %25, i64 24, i1 false), !!19931                                        ;L2519<53
 10247|  store i64 %247, ptr %10, , !!19926                                                                                    ;L576<2911<2519<53
 10248|  %251 = invoke ptr @_RNvMs6_NtCs5gUUnHMsxBL_9hashbrown3rawINtB5_8RawTableTjINtNtCs9LexZzt9XJB_5alloc3vec3VecNtNtBV_6string6StringEEE14insert_no_growCshdEBA0ozCnw_7game_ai(ptr %241, i64 %245, ptr %10)
 10249|  to label %252 unwind label %46                                                                                        ;L576<2911<2519<53
 10250| 
 10251| 252: ; preds = %244
 10253|  br label %261                                                                                                         ;L2521<53
 10254| 
 10255| 253: ; preds = %240
 10256|  %254 = load ptr, ptr %243, , !!8, !!8                                                                                 ;L3009<1014<53
 10257|     ;; self[0..+8] = ptr null
 10258|     ;; self[8..+8] = ptr %254
 10262|  store i64 0, ptr %25,                                                                                                 ;L464<53
 10263|  %255 = gep %25, i64 8                                                                                                 ;L464<53
 10264|  store ptr inttoptr (i64 8 to ptr), ptr %255,                                                                          ;L464<53
 10265|  %256 = gep %25, i64 16                                                                                                ;L464<53
 10266|  store i64 0, ptr %256,                                                                                                ;L464<53
 10267|     ;; default = ptr %25
 10271|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %25)
 10272|  to label %260 unwind label %257, !!19931                                                                              ;L825<2521<53
 10273| 
 10274| 257: ; preds = %253
 10275|  %258 = cleanuppad within none []
 10277|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %25) [ "funclet"(token %258) ]
 10278|  to label %259 unwind label %46                                                                                        ;L825<825<2521<53
 10279| 
 10280| 259: ; preds = %257
 10281|  cleanupret from %258 unwind label %46
 10282| 
 10283| 260: ; preds = %253
 10285|  invoke void @core::ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %25)
 10286|  to label %261 unwind label %46                                                                                        ;L825<825<2521<53
 10287| 
 10288| 261: ; preds = %260, %252
 10289|  %262 = phi ptr [ %251, %252 ], [ %254, %260 ]
 10290|  %263 = gep %262, i64 -24                                                                                              ;L0<53
 10291|     ;; self = ptr %263
 10293|     ;; args = ptr %139
 10295|  store ptr %139, ptr %23,                                                                                              ;L53
 10296|  %264 = gep %23, i64 8                                                                                                 ;L53
 10297|  store ptr @ai::plan_legacy9team_planNtB5_20ObjectivePostureKindNtNtCsjihNppCmMEE_4core3fmt5Debug3fmt, ptr %264,       ;L53
 10298|     ;; args[0..+8] = ptr @anon.94acafa22d01e083ca1cc62f01598c8f.122
 10299|     ;; args[8..+8] = ptr %23
 10300|     ;; self[0..+8] = ptr null
 10301|     ;; self[8..+8] = i64 undef
 10305|  invoke void @_RNvNvNtCs9LexZzt9XJB_5alloc3fmt6format12format_inner(ptr sret([24 x i8]) %24, ptr @anon.94acafa22d01e083ca1cc62f01598c8f.122, ptr %23)
 10306|  to label %265 unwind label %46                                                                                        ;L659<1275<659<53
 10307| 
 10308| 265: ; preds = %261
 10310|     ;; self = ptr %263
 10311|     ;; self = ptr %263
 10312|     ;; value = ptr %24
 10314|     ;; elem_size = i64 24
 10315|  %266 = gep %262, i64 -8                                                                                               ;L1037<1004<53
 10316|  %267 = load i64, ptr %266, , !!20026, !!8                                                                             ;L1037<1004<53
 10317|     ;; len = i64 %267
 10318|     ;; count = i64 %267
 10319|     ;; self = ptr %263
 10320|  %268 = load i64, ptr %263, , !!20026, !!8                                                                             ;L619<309<1040<1004<53
 10321|  %269 = icmp eq i64 %267, %268                                                                                         ;L1040<1004<53
 10322|  br i1 %269, label %270, label %274                                                                                    ;L1040<1004<53
 10323| 
 10324| 270: ; preds = %265
 10325|  invoke void @_RNvMs3_NtCs9LexZzt9XJB_5alloc7raw_vecINtB5_6RawVecNtNtB7_6string6StringE8grow_oneCszutcqs0z2F_10sys_locale(ptr %263)
 10326|  to label %274 unwind label %271, !!20026                                                                              ;L1041<1004<53
 10327| 
 10328| 271: ; preds = %270
 10329|  %272 = cleanuppad within none []
 10330|  invoke fastcc void @core::ptr9drop_glueNtNtCs9LexZzt9XJB_5alloc6string6StringECshdEBA0ozCnw_7game_ai(ptr %24) #30 [ "funclet"(token %272) ]
 10331|  to label %273 unwind label %46                                                                                        ;L1050<1004<53
 10332| 
 10333| 273: ; preds = %271
 10334|  cleanupret from %272 unwind label %46
 10335| 
 10336| 274: ; preds = %270, %265
 10337|  %275 = gep %262, i64 -16                                                                                              ;L614<609<296<2052<1044<1004<53
 10338|  %276 = load ptr, ptr %275, , !!20026, !!8, !!8                                                                        ;L614<609<296<2052<1044<1004<53
 10339|     ;; self = ptr %276
 10340|  %277 = getelementptr { { { { i64, ptr, {} }, {} }, i64 } }, ptr %276, i64 %267                                        ;L961<1044<1004<53
 10341|     ;; end = ptr %277
 10342|     ;; dst = ptr %277
 10343|  call void @llvm.memcpy.p0.p0.i64(ptr %277, ptr %24, i64 24, i1 false)                                                 ;L1933<1045<1004<53
 10344|  %278 = add i64 %267, 1                                                                                                ;L1046<1004<53
 10345|  store i64 %278, ptr %266, , !!20026                                                                                   ;L1046<1004<53
 10346|  br label %235                                                                                                         ;L1050<1004<53
 10347| 
 10348| 279: ; preds = %133
 10349|  %280 = icmp eq i64 %43, 0                                                                                             ;L58<66
 10350|  %281 = gep %37, i64 8                                                                                                 ;L7<0<66
 10351|  %282 = load ptr, ptr %281, , !!8, !!8                                                                                 ;L7<0<66
 10352|  %283 = gep %282, i64 4800                                                                                             ;L7<0<66
 10353|  %284 = load i64, ptr %283, , !!8                                                                                      ;L7<0<66
 10354|  %285 = sub i64 %116, %118                                                                                             ;L7<0<66
 10355|  %286 = add i64 %285, %284                                                                                             ;L8<0<66
 10356|  %287 = gep %282, i64 4792                                                                                             ;L8<0<66
 10357|  %288 = load i64, ptr %287, , !!8                                                                                      ;L8<0<66
 10358|  %289 = icmp ugt i64 %286, %288                                                                                        ;L8<0<66
 10359|  %290 = xor i1 %280, %289                                                                                              ;L58<66
 10360|  br i1 %290, label %298, label %291                                                                                    ;L58<66
 10361| 
 10362| 291: ; preds = %302, %279
 10363|  store i8 1, ptr %1,                                                                                                   ;L0
 10364|  br label %292                                                                                                         ;L77
 10365| 
 10366| 292: ; preds = %302, %291, %133
 10367|  %293 = phi i1 [ true, %133 ], [ false, %302 ], [ true, %291 ]
 10368|  %294 = gep %37, i64 32                                                                                                ;L77
 10369|  %295 = load ptr, ptr %294, , !!8, !!8                                                                                 ;L77
 10370|  %296 = icmp eq i64 %43, 0                                                                                             ;L77
 10371|  %297 = invoke { i64, i64 } @gc::simulation7map_defNtB2_6MapDef8camp_pos(ptr %295, i8 4, i1 zeroext %296)
 10372|  to label %317 unwind label %46                                                                                        ;L77
 10373| 
 10374| 298: ; preds = %279
 10375|  %299 = gep %37, i64 32                                                                                                ;L69
 10376|  %300 = load ptr, ptr %299, , !!8, !!8                                                                                 ;L69
 10377|  %301 = invoke { i64, i64 } @gc::simulation7map_defNtB2_6MapDef8camp_pos(ptr %300, i8 2, i1 zeroext %280)
 10378|  to label %302 unwind label %46                                                                                        ;L69
 10379| 
 10380| 302: ; preds = %298
 10381|  %303 = extractvalue { i64, i64 } %301, 0                                                                              ;L69
 10382|  %304 = extractvalue { i64, i64 } %301, 1                                                                              ;L69
 10383|     ;; camp[0..+8] = i64 %303
 10384|     ;; camp[8..+8] = i64 %304
 10385|     ;; x2 = i64 %303
 10386|     ;; other = i64 %303
 10387|     ;; y2 = i64 %304
 10388|     ;; other = i64 %304
 10389|  %305 = icmp ult i64 %116, %303                                                                                        ;L3147<7<71
 10390|  %306 = sub nuw i64 %303, %116                                                                                         ;L3147<7<71
 10391|  %307 = sub nuw i64 %116, %303                                                                                         ;L3147<7<71
 10392|  %308 = select i1 %305, i64 %306, i64 %307                                                                             ;L3147<7<71
 10393|     ;; dx = i64 %308
 10394|  %309 = icmp ult i64 %118, %304                                                                                        ;L3147<8<71
 10395|  %310 = sub nuw i64 %304, %118                                                                                         ;L3147<8<71
 10396|  %311 = sub nuw i64 %118, %304                                                                                         ;L3147<8<71
 10397|  %312 = select i1 %309, i64 %310, i64 %311                                                                             ;L3147<8<71
 10398|     ;; dy = i64 %312
 10399|  %313 = mul i64 %308, %308                                                                                             ;L9<71
 10400|  %314 = mul i64 %312, %312                                                                                             ;L9<71
 10401|  %315 = add i64 %313, %314                                                                                             ;L9<71
 10402|  %316 = icmp ult i64 %315, 4900000001                                                                                  ;L71
 10403|  br i1 %316, label %291, label %292                                                                                    ;L71
 10404| 
 10405| 317: ; preds = %292
 10406|  %318 = extractvalue { i64, i64 } %297, 0                                                                              ;L77
 10407|  %319 = extractvalue { i64, i64 } %297, 1                                                                              ;L77
 10408|     ;; camp[0..+8] = i64 %318
 10409|     ;; camp[8..+8] = i64 %319
 10410|     ;; x2 = i64 %318
 10411|     ;; other = i64 %318
 10412|     ;; y2 = i64 %319
 10413|     ;; other = i64 %319
 10414|  %320 = icmp ult i64 %116, %318                                                                                        ;L3147<7<78
 10415|  %321 = sub nuw i64 %318, %116                                                                                         ;L3147<7<78
 10416|  %322 = sub nuw i64 %116, %318                                                                                         ;L3147<7<78
 10417|  %323 = select i1 %320, i64 %321, i64 %322                                                                             ;L3147<7<78
 10418|     ;; dx = i64 %323
 10419|  %324 = icmp ult i64 %118, %319                                                                                        ;L3147<8<78
 10420|  %325 = sub nuw i64 %319, %118                                                                                         ;L3147<8<78
 10421|  %326 = sub nuw i64 %118, %319                                                                                         ;L3147<8<78
 10422|  %327 = select i1 %324, i64 %325, i64 %326                                                                             ;L3147<8<78
 10423|     ;; dy = i64 %327
 10424|  %328 = mul i64 %323, %323                                                                                             ;L9<78
 10425|  %329 = mul i64 %327, %327                                                                                             ;L9<78
 10426|  %330 = add i64 %328, %329                                                                                             ;L9<78
 10427|  %331 = icmp ugt i64 %330, 22500000000                                                                                 ;L78
 10428|  br i1 %331, label %333, label %332                                                                                    ;L78
 10429| 
 10430| 332: ; preds = %317
 10433|  invoke void @ai::small_action6aroundNtB5_25SmallActionAroundPosition3new(ptr sret([184 x i8]) %16, ptr %3, ptr %5, i64 %318, i64 %319, i64 5)
 10434|  to label %334 unwind label %46                                                                                        ;L87
 10435| 
 10436| 333: ; preds = %317
 10437|  br i1 %293, label %389, label %387                                                                                    ;L79
 10438| 
 10439| 334: ; preds = %332
 10440|  call void @llvm.memcpy.p0.p0.i64(ptr %17, ptr %16, i64 184, i1 false)                                                 ;L87
 10443|     ;; self = ptr %35
 10444|     ;; self = ptr %35
 10445|     ;; value = ptr %17
 10446|     ;; src = ptr %17
 10447|     ;; additional = i64 1
 10448|     ;; needed_extra_cap = i64 1
 10449|     ;; needed_extra_cap = i64 1
 10450|     ;; strategy = i8 1
 10451|     ;; self = ptr %35
 10452|  %335 = load i64, ptr %40, , !!20104, !!8                                                                              ;L149<1428<87
 10453|  %336 = icmp eq i64 %135, %335                                                                                         ;L1428<87
 10454|  br i1 %336, label %337, label %343                                                                                    ;L1428<87
 10455| 
 10456| 337: ; preds = %334
 10457|     ;; self = ptr %35
 10458|     ;; self = ptr %35
 10459|     ;; self = ptr %35
 10460|     ;; used_cap = i64 %135
 10461|     ;; used_cap = i64 %135
 10462|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %35, i64 %135, i64 1, i1 zeroext true)
 10463|  to label %338 unwind label %341, !!20104                                                                              ;L619<430<738<1429<87
 10464| 
 10465| 338: ; preds = %337
 10466|  %339 = load i64, ptr %41, , !!20104                                                                                   ;L1432<87
 10467|  %340 = load ptr, ptr %35, , !!20104                                                                                   ;L138<1432<87
 10468|  br label %343                                                                                                         ;L619<430<738<1429<87
 10469| 
 10470| 341: ; preds = %337
 10471|  %342 = cleanuppad within none []
 10472|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %17) #30 [ "funclet"(token %342) ], !!20087 ;L1436<87
 10473|  cleanupret from %342 unwind label %46
 10474| 
 10475| 343: ; preds = %338, %334
 10476|  %344 = phi ptr [ %340, %338 ], [ %134, %334 ]                                                                         ;L138<1432<87
 10477|  %345 = phi i64 [ %339, %338 ], [ %135, %334 ]                                                                         ;L1432<87
 10478|     ;; self = ptr %35
 10479|     ;; self = ptr %344
 10480|     ;; count = i64 %345
 10481|  %346 = gepS %344, i64 %345                                                                                            ;L961<1432<87
 10482|     ;; end = ptr %346
 10483|     ;; dst = ptr %346
 10484|  call void @llvm.memcpy.p0.p0.i64(ptr %346, ptr %17, i64 184, i1 false), !!20087                                       ;L1933<1433<87
 10485|  %347 = add i64 %345, 1                                                                                                ;L1434<87
 10486|  store i64 %347, ptr %41, , !!20104                                                                                    ;L1434<87
 10488|  br label %348                                                                                                         ;L78
 10489| 
 10490| 348: ; preds = %416, %402, %343
 10491|  %349 = phi ptr [ %344, %343 ], [ %417, %416 ], [ %403, %402 ]
 10492|  %350 = phi i64 [ %347, %343 ], [ %420, %416 ], [ %406, %402 ]
 10493|     ;; self = ptr undef
 10494|     ;; self = ptr undef
 10495|  %351 = load ptr, ptr %53, , !!8, !!8                                                                                  ;L90
 10496|  %352 = gep %53, i64 8                                                                                                 ;L90
 10497|  %353 = load ptr, ptr %352, , !!8, !!8                                                                                 ;L90
 10498|  %354 = gep %5, i64 16                                                                                                 ;L90
 10499|  %355 = load ptr, ptr %354, , !!8, !!8                                                                                 ;L90
 10500|     ;; f[0..+8] = ptr %351
 10501|     ;; f[8..+8] = ptr %353
 10502|     ;; f[16..+8] = ptr %355
 10503|     ;; f[24..+8] = ptr %4
 10504|     ;; f[32..+8] = ptr %57
 10505|     ;; fold[0..+8] = ptr %351
 10506|     ;; fold[8..+8] = ptr %353
 10507|     ;; fold[16..+8] = ptr %355
 10508|     ;; fold[24..+8] = ptr %4
 10509|     ;; fold[32..+8] = ptr %57
 10512|     ;; f[8..+8] = ptr %351
 10513|     ;; f[16..+8] = ptr %353
 10514|     ;; f[24..+8] = ptr %355
 10515|     ;; f[32..+8] = ptr %4
 10516|     ;; f[40..+8] = ptr %57
 10517|     ;; self = ptr undef
 10520|     ;; self = ptr undef
 10521|     ;; count = i64 1
 10522|     ;; ptr = ptr %62
 10523|     ;; self = ptr %62
 10524|     ;; end_or_len = ptr %62
 10527|  %356 = getelementptr { { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, [5 x { i64, [2 x i64] }], [5 x { i64, { i8, [23 x i8] } }], [5 x { i8, [7 x i8], i64 }], [5 x i64], [5 x i64], [5 x i64], [5 x { i64, i64 }], i64, i64, i64, i64, i64, [5 x i32], i8, [3 x i8] }, ptr %355, i64 %61
 10528|  br label %357                                                                                                         ;L180<2493<138<2897<90
 10529| 
 10530| 357: ; preds = %385, %348
 10531|  %358 = phi i64 [ 0, %348 ], [ %360, %385 ]
 10532|  %359 = gep %62, i64 %358                                                                                              ;L656<185<2493<138<2897<90
 10533|     ;; ptr = ptr %359
 10534|  %360 = add nuw nsw i64 %358, 8                                                                                        ;L656<185<2493<138<2897<90
 10535|     ;; x = ptr %359
 10536|  %361 = load ptr, ptr %359, , !!20150, !!8                                                                             ;L2494<138<2897<90
 10537|     ;; f = ptr undef
 10541|  %362 = icmp eq ptr %361, null                                                                                         ;L49<2494<138<2897<90
 10542|  br i1 %362, label %385, label %363                                                                                    ;L49<2494<138<2897<90
 10543| 
 10544| 363: ; preds = %357
 10545|     ;; x = ptr %361
 10548|     ;; x = ptr %361
 10552|     ;; c = ptr %361
 10553|     ;; self = ptr %361
 10554|  %364 = invoke zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %356, ptr %351, ptr %353, ptr %4, ptr %361)
 10555|  to label %365 unwind label %46                                                                                        ;L90<2893<50<2494<138<2897<90
 10556| 
 10557| 365: ; preds = %363
 10558|  br i1 %364, label %366, label %385                                                                                    ;L90<2893<50<2494<138<2897<90
 10559| 
 10560| 366: ; preds = %365
 10561|     ;; other = ptr %57
 10562|  %367 = gep %361, i64 1632                                                                                             ;L2158<91<2893<50<2494<138<2897<90
 10563|  %368 = load i64, ptr %367, , !!20194, !!8                                                                             ;L2158<91<2893<50<2494<138<2897<90
 10564|     ;; x1 = i64 %368
 10565|     ;; self = i64 %368
 10566|  %369 = gep %361, i64 1640                                                                                             ;L2158<91<2893<50<2494<138<2897<90
 10567|  %370 = load i64, ptr %369, , !!20194, !!8                                                                             ;L2158<91<2893<50<2494<138<2897<90
 10568|     ;; y1 = i64 %370
 10569|     ;; self = i64 %370
 10570|  %371 = load i64, ptr %115, , !!20194, !!8                                                                             ;L2158<91<2893<50<2494<138<2897<90
 10571|     ;; x2 = i64 %371
 10572|     ;; other = i64 %371
 10573|  %372 = load i64, ptr %117, , !!20194, !!8                                                                             ;L2158<91<2893<50<2494<138<2897<90
 10574|     ;; y2 = i64 %372
 10575|     ;; other = i64 %372
 10576|  %373 = icmp ult i64 %368, %371                                                                                        ;L3147<7<2158<91<2893<50<2494<138<2897<90
 10577|  %374 = sub nuw i64 %371, %368                                                                                         ;L3147<7<2158<91<2893<50<2494<138<2897<90
 10578|  %375 = sub nuw i64 %368, %371                                                                                         ;L3147<7<2158<91<2893<50<2494<138<2897<90
 10579|  %376 = select i1 %373, i64 %374, i64 %375                                                                             ;L3147<7<2158<91<2893<50<2494<138<2897<90
 10580|     ;; dx = i64 %376
 10581|  %377 = icmp ult i64 %370, %372                                                                                        ;L3147<8<2158<91<2893<50<2494<138<2897<90
 10582|  %378 = sub nuw i64 %372, %370                                                                                         ;L3147<8<2158<91<2893<50<2494<138<2897<90
 10583|  %379 = sub nuw i64 %370, %372                                                                                         ;L3147<8<2158<91<2893<50<2494<138<2897<90
 10584|  %380 = select i1 %377, i64 %378, i64 %379                                                                             ;L3147<8<2158<91<2893<50<2494<138<2897<90
 10585|     ;; dy = i64 %380
 10586|  %381 = mul i64 %376, %376                                                                                             ;L9<2158<91<2893<50<2494<138<2897<90
 10587|  %382 = mul i64 %380, %380                                                                                             ;L9<2158<91<2893<50<2494<138<2897<90
 10588|  %383 = add i64 %382, %381                                                                                             ;L9<2158<91<2893<50<2494<138<2897<90
 10589|  %384 = icmp ult i64 %383, 22500000001                                                                                 ;L91<2893<50<2494<138<2897<90
 10590|  br i1 %384, label %458, label %385                                                                                    ;L2494<138<2897<90
 10591| 
 10592| 385: ; preds = %366, %365, %357
 10593|     ;; self = ptr undef
 10594|     ;; count = i64 1
 10595|     ;; ptr = !DIArgList(ptr %62, i64 %360)
 10596|     ;; self = !DIArgList(ptr %62, i64 %360)
 10597|     ;; end_or_len = ptr %62
 10600|  %386 = icmp eq i64 %360, 40                                                                                           ;L1714<180<2493<138<2897<90
 10601|  br i1 %386, label %421, label %357                                                                                    ;L180<2493<138<2897<90
 10602| 
 10603| 387: ; preds = %333
 10604|  %388 = invoke { i64, i64 } @gc::simulation7map_defNtB2_6MapDef8camp_pos(ptr %295, i8 2, i1 zeroext %296)
 10605|  to label %390 unwind label %46                                                                                        ;L83
 10606| 
 10607| 389: ; preds = %333
 10610|  invoke void @ai::small_action6aroundNtB5_25SmallActionAroundPosition3new(ptr sret([184 x i8]) %20, ptr %3, ptr %5, i64 %318, i64 %319, i64 5)
 10611|  to label %407 unwind label %46                                                                                        ;L80
 10612| 
 10613| 390: ; preds = %387
 10614|  %391 = extractvalue { i64, i64 } %388, 0                                                                              ;L83
 10615|  %392 = extractvalue { i64, i64 } %388, 1                                                                              ;L83
 10616|     ;; camp[0..+8] = i64 %391
 10617|     ;; camp[8..+8] = i64 %392
 10620|  invoke void @ai::small_action6aroundNtB5_25SmallActionAroundPosition3new(ptr sret([184 x i8]) %18, ptr %3, ptr %5, i64 %391, i64 %392, i64 5)
 10621|  to label %393 unwind label %46                                                                                        ;L84
 10622| 
 10623| 393: ; preds = %390
 10624|  call void @llvm.memcpy.p0.p0.i64(ptr %19, ptr %18, i64 184, i1 false)                                                 ;L84
 10627|     ;; self = ptr %35
 10628|     ;; self = ptr %35
 10629|     ;; value = ptr %19
 10630|     ;; src = ptr %19
 10631|     ;; additional = i64 1
 10632|     ;; needed_extra_cap = i64 1
 10633|     ;; needed_extra_cap = i64 1
 10634|     ;; strategy = i8 1
 10635|     ;; self = ptr %35
 10636|  %394 = load i64, ptr %40, , !!20247, !!8                                                                              ;L149<1428<84
 10637|  %395 = icmp eq i64 %135, %394                                                                                         ;L1428<84
 10638|  br i1 %395, label %396, label %402                                                                                    ;L1428<84
 10639| 
 10640| 396: ; preds = %393
 10641|     ;; self = ptr %35
 10642|     ;; self = ptr %35
 10643|     ;; self = ptr %35
 10644|     ;; used_cap = i64 %135
 10645|     ;; used_cap = i64 %135
 10646|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %35, i64 %135, i64 1, i1 zeroext true)
 10647|  to label %397 unwind label %400, !!20247                                                                              ;L619<430<738<1429<84
 10648| 
 10649| 397: ; preds = %396
 10650|  %398 = load i64, ptr %41, , !!20247                                                                                   ;L1432<84
 10651|  %399 = load ptr, ptr %35, , !!20247                                                                                   ;L138<1432<84
 10652|  br label %402                                                                                                         ;L619<430<738<1429<84
 10653| 
 10654| 400: ; preds = %396
 10655|  %401 = cleanuppad within none []
 10656|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %19) #30 [ "funclet"(token %401) ], !!20230 ;L1436<84
 10657|  cleanupret from %401 unwind label %46
 10658| 
 10659| 402: ; preds = %397, %393
 10660|  %403 = phi ptr [ %399, %397 ], [ %134, %393 ]                                                                         ;L138<1432<84
 10661|  %404 = phi i64 [ %398, %397 ], [ %135, %393 ]                                                                         ;L1432<84
 10662|     ;; self = ptr %35
 10663|     ;; self = ptr %403
 10664|     ;; count = i64 %404
 10665|  %405 = gepS %403, i64 %404                                                                                            ;L961<1432<84
 10666|     ;; end = ptr %405
 10667|     ;; dst = ptr %405
 10668|  call void @llvm.memcpy.p0.p0.i64(ptr %405, ptr %19, i64 184, i1 false), !!20230                                       ;L1933<1433<84
 10669|  %406 = add i64 %404, 1                                                                                                ;L1434<84
 10670|  store i64 %406, ptr %41, , !!20247                                                                                    ;L1434<84
 10672|  br label %348                                                                                                         ;L79
 10673| 
 10674| 407: ; preds = %389
 10675|  call void @llvm.memcpy.p0.p0.i64(ptr %21, ptr %20, i64 184, i1 false)                                                 ;L80
 10678|     ;; self = ptr %35
 10679|     ;; self = ptr %35
 10680|     ;; value = ptr %21
 10681|     ;; src = ptr %21
 10682|     ;; additional = i64 1
 10683|     ;; needed_extra_cap = i64 1
 10684|     ;; needed_extra_cap = i64 1
 10685|     ;; strategy = i8 1
 10686|     ;; self = ptr %35
 10687|  %408 = load i64, ptr %40, , !!20281, !!8                                                                              ;L149<1428<80
 10688|  %409 = icmp eq i64 %135, %408                                                                                         ;L1428<80
 10689|  br i1 %409, label %410, label %416                                                                                    ;L1428<80
 10690| 
 10691| 410: ; preds = %407
 10692|     ;; self = ptr %35
 10693|     ;; self = ptr %35
 10694|     ;; self = ptr %35
 10695|     ;; used_cap = i64 %135
 10696|     ;; used_cap = i64 %135
 10697|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %35, i64 %135, i64 1, i1 zeroext true)
 10698|  to label %411 unwind label %414, !!20281                                                                              ;L619<430<738<1429<80
 10699| 
 10700| 411: ; preds = %410
 10701|  %412 = load i64, ptr %41, , !!20281                                                                                   ;L1432<80
 10702|  %413 = load ptr, ptr %35, , !!20281                                                                                   ;L138<1432<80
 10703|  br label %416                                                                                                         ;L619<430<738<1429<80
 10704| 
 10705| 414: ; preds = %410
 10706|  %415 = cleanuppad within none []
 10707|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %21) #30 [ "funclet"(token %415) ], !!20264 ;L1436<80
 10708|  cleanupret from %415 unwind label %46
 10709| 
 10710| 416: ; preds = %411, %407
 10711|  %417 = phi ptr [ %413, %411 ], [ %134, %407 ]                                                                         ;L138<1432<80
 10712|  %418 = phi i64 [ %412, %411 ], [ %135, %407 ]                                                                         ;L1432<80
 10713|     ;; self = ptr %35
 10714|     ;; self = ptr %417
 10715|     ;; count = i64 %418
 10716|  %419 = gepS %417, i64 %418                                                                                            ;L961<1432<80
 10717|     ;; end = ptr %419
 10718|     ;; dst = ptr %419
 10719|  call void @llvm.memcpy.p0.p0.i64(ptr %419, ptr %21, i64 184, i1 false), !!20264                                       ;L1933<1433<80
 10720|  %420 = add i64 %418, 1                                                                                                ;L1434<80
 10721|  store i64 %420, ptr %41, , !!20281                                                                                    ;L1434<80
 10723|  br label %348                                                                                                         ;L79
 10724| 
 10725| 421: ; preds = %385
 10726|  %422 = gep %53, i64 240                                                                                               ;L92
 10727|  %423 = getelementptr { { ptr, ptr, i64 }, i64 }, ptr %422, i64 %61                                                    ;L92
 10728|     ;; self = ptr %423
 10729|     ;; self = ptr %423
 10730|  %424 = load ptr, ptr %423, , !!8, !!8                                                                                 ;L138<2073<92
 10731|     ;; p = ptr %424
 10732|  %425 = gep %423, i64 24                                                                                               ;L2075<92
 10733|  %426 = load i64, ptr %425, , !!8                                                                                      ;L2075<92
 10734|     ;; len = i64 %426
 10735|     ;; count = i64 %426
 10736|     ;; self[0..+8] = ptr %424
 10737|     ;; slice[0..+8] = ptr %424
 10738|     ;; self[8..+8] = i64 %426
 10739|     ;; slice[8..+8] = i64 %426
 10740|     ;; ptr = ptr %424
 10741|     ;; self = ptr %424
 10742|  %427 = getelementptr ptr, ptr %424, i64 %426                                                                          ;L961<100<1042<92
 10744|     ;; f = ptr %57
 10745|     ;; self = ptr undef
 10746|     ;; self = ptr undef
 10747|     ;; count = i64 1
 10748|  %428 = load i64, ptr %115, , !!20371
 10749|  %429 = load i64, ptr %117, , !!20371
 10750|  br label %430                                                                                                         ;L331<92
 10751| 
 10752| 430: ; preds = %433, %421
 10753|  %431 = phi ptr [ %434, %433 ], [ %424, %421 ]
 10754|     ;; ptr = ptr %431
 10755|     ;; self = ptr %431
 10756|     ;; end_or_len = ptr %427
 10759|  %432 = icmp eq ptr %431, %427                                                                                         ;L1714<180<331<92
 10760|  br i1 %432, label %452, label %433                                                                                    ;L180<331<92
 10761| 
 10762| 433: ; preds = %430
 10763|  %434 = gep %431, i64 8                                                                                                ;L656<185<331<92
 10764|     ;; x = ptr %431
 10765|  %435 = load ptr, ptr %431, , !!20387, !!8, !!8                                                                        ;L332<92
 10768|     ;; self = ptr %435
 10769|     ;; other = ptr %57
 10770|  %436 = gep %435, i64 1632                                                                                             ;L2158<92<332<92
 10771|  %437 = load i64, ptr %436, , !!20387, !!8                                                                             ;L2158<92<332<92
 10772|     ;; x1 = i64 %437
 10773|     ;; self = i64 %437
 10774|  %438 = gep %435, i64 1640                                                                                             ;L2158<92<332<92
 10775|  %439 = load i64, ptr %438, , !!20387, !!8                                                                             ;L2158<92<332<92
 10776|     ;; y1 = i64 %439
 10777|     ;; self = i64 %439
 10778|     ;; x2 = i64 %428
 10779|     ;; other = i64 %428
 10780|     ;; y2 = i64 %429
 10781|     ;; other = i64 %429
 10782|  %440 = icmp ult i64 %437, %428                                                                                        ;L3147<7<2158<92<332<92
 10783|  %441 = sub nuw i64 %428, %437                                                                                         ;L3147<7<2158<92<332<92
 10784|  %442 = sub nuw i64 %437, %428                                                                                         ;L3147<7<2158<92<332<92
 10785|  %443 = select i1 %440, i64 %441, i64 %442                                                                             ;L3147<7<2158<92<332<92
 10786|     ;; dx = i64 %443
 10787|  %444 = icmp ult i64 %439, %429                                                                                        ;L3147<8<2158<92<332<92
 10788|  %445 = sub nuw i64 %429, %439                                                                                         ;L3147<8<2158<92<332<92
 10789|  %446 = sub nuw i64 %439, %429                                                                                         ;L3147<8<2158<92<332<92
 10790|  %447 = select i1 %444, i64 %445, i64 %446                                                                             ;L3147<8<2158<92<332<92
 10791|     ;; dy = i64 %447
 10792|  %448 = mul i64 %443, %443                                                                                             ;L9<2158<92<332<92
 10793|  %449 = mul i64 %447, %447                                                                                             ;L9<2158<92<332<92
 10794|  %450 = add i64 %449, %448                                                                                             ;L9<2158<92<332<92
 10795|  %451 = icmp ult i64 %450, 22500000001                                                                                 ;L92<332<92
 10796|  br i1 %451, label %458, label %430                                                                                    ;L332<92
 10797| 
 10798| 452: ; preds = %469, %430
 10799|  %453 = gep %57, i64 1472                                                                                              ;L98
 10800|  %454 = load i64, ptr %453, , !!8                                                                                      ;L98
 10801|  %455 = gep %353, i64 248                                                                                              ;L98
 10802|  %456 = load ptr, ptr %455, , !!8                                                                                      ;L98
 10803|  %457 = invoke zeroext i1 %456(ptr %351, i64 %61, i64 %454)
 10804|  to label %474 unwind label %46                                                                                        ;L98
 10805| 
 10806| 458: ; preds = %433, %366
 10809|  invoke void @ai::small_action12move_actionsNtB2_18SmallActionRunAway3new(ptr sret([136 x i8]) %14, ptr %5, ptr %4, i64 5)
 10810|  to label %459 unwind label %46                                                                                        ;L95
 10811| 
 10812| 459: ; preds = %458
 10813|  call void @llvm.memcpy.p0.p0.i64(ptr %15, ptr %14, i64 136, i1 false)                                                 ;L95
 10814|  %460 = gep %15, i64 177                                                                                               ;L95
 10815|  store i8 3, ptr %460,                                                                                                 ;L95
 10818|     ;; self = ptr %35
 10819|     ;; self = ptr %35
 10820|     ;; value = ptr %15
 10821|     ;; src = ptr %15
 10822|     ;; additional = i64 1
 10823|     ;; needed_extra_cap = i64 1
 10824|     ;; needed_extra_cap = i64 1
 10825|     ;; strategy = i8 1
 10826|     ;; self = ptr %35
 10827|  %461 = load i64, ptr %40, , !!20449, !!8                                                                              ;L149<1428<95
 10828|  %462 = icmp eq i64 %350, %461                                                                                         ;L1428<95
 10829|  br i1 %462, label %463, label %469                                                                                    ;L1428<95
 10830| 
 10831| 463: ; preds = %459
 10832|     ;; self = ptr %35
 10833|     ;; self = ptr %35
 10834|     ;; self = ptr %35
 10835|     ;; used_cap = i64 %350
 10836|     ;; used_cap = i64 %350
 10837|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %35, i64 %350, i64 1, i1 zeroext true)
 10838|  to label %464 unwind label %467, !!20449                                                                              ;L619<430<738<1429<95
 10839| 
 10840| 464: ; preds = %463
 10841|  %465 = load i64, ptr %41, , !!20449                                                                                   ;L1432<95
 10842|  %466 = load ptr, ptr %35, , !!20449                                                                                   ;L138<1432<95
 10843|  br label %469                                                                                                         ;L619<430<738<1429<95
 10844| 
 10845| 467: ; preds = %463
 10846|  %468 = cleanuppad within none []
 10847|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %15) #30 [ "funclet"(token %468) ], !!20432 ;L1436<95
 10848|  cleanupret from %468 unwind label %46
 10849| 
 10850| 469: ; preds = %464, %459
 10851|  %470 = phi ptr [ %466, %464 ], [ %349, %459 ]                                                                         ;L138<1432<95
 10852|  %471 = phi i64 [ %465, %464 ], [ %350, %459 ]                                                                         ;L1432<95
 10853|     ;; self = ptr %35
 10854|     ;; self = ptr %470
 10855|     ;; count = i64 %471
 10856|  %472 = gepS %470, i64 %471                                                                                            ;L961<1432<95
 10857|     ;; end = ptr %472
 10858|     ;; dst = ptr %472
 10859|  call void @llvm.memcpy.p0.p0.i64(ptr %472, ptr %15, i64 184, i1 false), !!20432                                       ;L1933<1433<95
 10860|  %473 = add i64 %471, 1                                                                                                ;L1434<95
 10861|  store i64 %473, ptr %41, , !!20449                                                                                    ;L1434<95
 10863|  br label %452                                                                                                         ;L94
 10864| 
 10865| 474: ; preds = %452
 10866|  br i1 %457, label %475, label %476                                                                                    ;L98
 10867| 
 10868| 475: ; preds = %474
 10870|  invoke void @ai::fight_check13battle_action(ptr sret([32 x i8]) %13, i64 %2, ptr %3, ptr %4, ptr %5, i64 5)
 10871|  to label %477 unwind label %46                                                                                        ;L99
 10872| 
 10873| 476: ; preds = %481, %474
 10875|  invoke void @ai::fight_check20attack_summon_action(ptr sret([32 x i8]) %12, ptr %4, ptr %5)
 10876|  to label %482 unwind label %46                                                                                        ;L101
 10877| 
 10878| 477: ; preds = %475
 10879|  %478 = load ptr, ptr %13, , !!8, !!8                                                                                  ;L99
 10880|  %479 = gep %13, i64 24                                                                                                ;L99
 10881|  %480 = load i64, ptr %479, , !!8                                                                                      ;L99
 10882|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %35, ptr %478, i64 %480)
 10883|  to label %481 unwind label %46                                                                                        ;L99
 10884| 
 10885| 481: ; preds = %477
 10887|  br label %476                                                                                                         ;L98
 10888| 
 10889| 482: ; preds = %476
 10890|  %483 = load ptr, ptr %12, , !!8, !!8                                                                                  ;L101
 10891|  %484 = gep %12, i64 24                                                                                                ;L101
 10892|  %485 = load i64, ptr %484, , !!8                                                                                      ;L101
 10893|  invoke fastcc void @core::iter6traits7collect6ExtendBX_E6extendBN_EB11_(ptr %35, ptr %483, i64 %485)
 10894|  to label %486 unwind label %46                                                                                        ;L101
 10895| 
 10896| 486: ; preds = %482
 10898|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %35, i64 32, i1 false)                                                   ;L103
 10900|  br label %487                                                                                                         ;L104
 10901| 
 10902| 487: ; preds = %492, %486, %235
 10904|  ret void                                                                                                              ;L104
 10905| 
 10906| 488: ; preds = %128
 10907|  call void @llvm.memcpy.p0.p0.i64(ptr %33, ptr %32, i64 136, i1 false)                                                 ;L40
 10908|  %489 = gep %33, i64 177                                                                                               ;L40
 10909|  store i8 3, ptr %489,                                                                                                 ;L40
 10912|     ;; self = ptr %35
 10913|     ;; self = ptr %35
 10914|     ;; value = ptr %33
 10915|     ;; src = ptr %33
 10916|     ;; additional = i64 1
 10917|     ;; needed_extra_cap = i64 1
 10918|     ;; needed_extra_cap = i64 1
 10919|     ;; strategy = i8 1
 10920|     ;; self = ptr %35
 10921|     ;; self = ptr %35
 10922|     ;; self = ptr %35
 10923|     ;; self = ptr %35
 10924|     ;; used_cap = i64 0
 10925|     ;; used_cap = i64 0
 10926|  invoke void @ai::small_action15SmallActionPlayE25reserve_internal_or_panicB17_(ptr %35, i64 0, i64 1, i1 zeroext true)
 10927|  to label %492 unwind label %490, !!20489                                                                              ;L619<430<738<1429<40
 10928| 
 10929| 490: ; preds = %488
 10930|  %491 = cleanuppad within none []
 10931|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %33) #30 [ "funclet"(token %491) ], !!20470 ;L1436<40
 10932|  cleanupret from %491 unwind label %46
 10933| 
 10934| 492: ; preds = %488
 10935|  %493 = load ptr, ptr %35, , !!20489                                                                                   ;L138<1432<40
 10936|  %494 = load i64, ptr %41, , !!20489                                                                                   ;L1432<40
 10937|     ;; self = ptr %35
 10938|     ;; self = ptr %493
 10939|     ;; count = i64 %494
 10940|  %495 = gepS %493, i64 %494                                                                                            ;L961<1432<40
 10941|     ;; end = ptr %495
 10942|     ;; dst = ptr %495
 10943|  call void @llvm.memcpy.p0.p0.i64(ptr %495, ptr %33, i64 184, i1 false), !!20470                                       ;L1933<1433<40
 10944|  %496 = add i64 %494, 1                                                                                                ;L1434<40
 10945|  store i64 %496, ptr %41, , !!20489                                                                                    ;L1434<40
 10947|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %35, i64 32, i1 false)                                                   ;L41
 10949|  br label %487                                                                                                         ;L1
 10950| }
