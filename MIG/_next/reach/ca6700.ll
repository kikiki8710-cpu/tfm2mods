 48954| define hidden void @core::iter8adapters3map3MapINtB3_8IntoIterBW_ENCNvMNtNtNtB10_11plan_legacy7handler7auctionNtB3l_17LegacyPlanHandler16get_small_actions1_0EEB10_(ptr sret([32 x i8]) %0, ptr dead_on_return %1, ptr %2) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 48955|  %4 = alloca [184 x i8],
 48956|  %5 = alloca [184 x i8],
 48957|  %6 = alloca [192 x i8],
 48958|  %7 = alloca [80 x i8],
 48959|  %8 = alloca [80 x i8],
 48960|  %9 = alloca [32 x i8],
 48961|     ;; iter = ptr %1
 48962|     ;; bump = ptr %2
 48963|     ;; bump = ptr %2
 48964|     ;; v = ptr %9
 48966|  store ptr inttoptr (i64 8 to ptr), ptr %9,                                                                            ;L547<606
 48967|  %10 = gep %9, i64 8                                                                                                   ;L547<606
 48968|  store ptr %2, ptr %10,                                                                                                ;L547<606
 48969|  %11 = gep %9, i64 16                                                                                                  ;L547<606
 48970|  %12 = gep %9, i64 24                                                                                                  ;L547<606
 48972|     ;; self = ptr %9
 48973|     ;; self = ptr %9
 48974|     ;; iter = ptr %1
 48975|     ;; iter = ptr %8
 48976|     ;; iter = ptr %7
 48977|     ;; t = ptr %6
 48978|     ;; strategy = i8 1
 48979|  call void @llvm.memset.p0.i64(ptr %11, i8 0, i64 16, i1 false)                                                        ;L547<606
 48981|     ;; self = ptr %1
 48982|  call void @llvm.memcpy.p0.p0.i64(ptr %8, ptr %1, i64 80, i1 false), !!70052                                           ;L323<2152<607
 48983|  %13 = gep %8, i64 64                                                                                                  ;L2153<607
 48984|  %14 = load ptr, ptr %13, , !!70106, !!8                                                                               ;L2153<607
 48985|  %15 = gep %8, i64 72                                                                                                  ;L2153<607
 48986|  %16 = load ptr, ptr %15, , !!70106, !!8                                                                               ;L2153<607
 48989|     ;; p = ptr %16
 48990|     ;; origin = ptr %14
 48991|     ;; pointee_size = i64 184
 48992|     ;; self = ptr %16
 48993|     ;; rhs = ptr %14
 48994|     ;; d = !DIArgList(ptr %16, ptr %14)
 48995|  %17 = ptrtoint ptr %16 to i64                                                                                         ;L213<2460<112<2153<607
 48996|     ;; d = !DIArgList(i64 %17, ptr %14)
 48997|     ;; self = i64 %17
 48998|  %18 = ptrtoint ptr %14 to i64                                                                                         ;L213<2460<112<2153<607
 48999|     ;; d = !DIArgList(i64 %17, i64 %18)
 49000|     ;; rhs = i64 %18
 49001|  %19 = sub i64 %17, %18                                                                                                ;L2202<213<2460<112<2153<607
 49002|     ;; d = i64 %19
 49003|     ;; additional = i64 %19
 49004|     ;; needed_extra_cap = i64 %19
 49005|     ;; needed_extra_cap = i64 %19
 49006|     ;; self = ptr %9
 49007|     ;; self = ptr %9
 49008|     ;; used_cap = i64 0
 49009|     ;; used_cap = i64 0
 49010|  %20 = add i64 %19, 183                                                                                                ;L614<430<738<2153<607
 49011|  %21 = icmp ult i64 %20, 367                                                                                           ;L614<430<738<2153<607
 49012|  br i1 %21, label %27, label %25                                                                                       ;L614<430<738<2153<607
 49013| 
 49014| 22: ; preds = %90, %25
 49015|  %23 = phi i1 [ false, %90 ], [ true, %25 ]                                                                            ;L0<607
 49016|  %24 = cleanuppad within none []
 49017|  br i1 %23, label %124, label %123                                                                                     ;L2158<607
 49018| 
 49019| 25: ; preds = %3
 49020|  %26 = sdiv i64 %19, 184                                                                                               ;L214<2460<112<2153<607
 49021|     ;; additional = i64 %26
 49022|     ;; needed_extra_cap = i64 %26
 49023|     ;; needed_extra_cap = i64 %26
 49024|  invoke void @ai::small_action15SmallActionPlayEE25reserve_internal_or_panicB19_(ptr %9, i64 0, i64 %26, i1 zeroext true)
 49025|  to label %27 unwind label %22, !!70169                                                                                ;L619<430<738<2153<607
 49026| 
 49027| 27: ; preds = %25, %3
 49029|  call void @llvm.memcpy.p0.p0.i64(ptr %7, ptr %1, i64 80, i1 false), !!70052                                           ;L2155<607
 49030|  %28 = gep %7, i64 64
 49031|  %29 = gep %7, i64 72
 49032|  %30 = load ptr, ptr %29, , !!70176, !!8
 49033|  %31 = load ptr, ptr %28, , !!70176
 49034|  %32 = icmp eq ptr %31, %30                                                                                            ;L2436<107<2155<607
 49035|  br i1 %32, label %102, label %33                                                                                      ;L2436<107<2155<607
 49036| 
 49037| 33: ; preds = %27
 49038|  %34 = gep %5, i64 178
 49039|  %35 = gep %5, i64 177
 49040|  %36 = gep %7, i64 8
 49041|  %37 = gep %7, i64 16
 49042|  %38 = gep %7, i64 24
 49043|  %39 = gep %7, i64 32
 49044|  %40 = gep %7, i64 40
 49045|  %41 = gep %7, i64 48
 49046|  %42 = gep %7, i64 56
 49047|  %43 = load ptr, ptr %7, , !!70106, !!8
 49048|  %44 = load ptr, ptr %36, , !!70106, !!8
 49049|  %45 = load ptr, ptr %37, , !!70106, !!8
 49050|  %46 = load ptr, ptr %38, , !!70106, !!8
 49051|  %47 = load ptr, ptr %39, , !!70106, !!8
 49052|  %48 = load ptr, ptr %40, , !!70106, !!8
 49053|  %49 = load ptr, ptr %41, , !!70106, !!8
 49054|  %50 = load ptr, ptr %42, , !!70106, !!8
 49055|  %51 = gep %6, i64 8
 49056|  %52 = gep %6, i64 185
 49057|  %53 = gep %6, i64 186
 49058|  br label %54                                                                                                          ;L2436<107<2155<607
 49059| 
 49060| 54: ; preds = %117, %33
 49061|  %55 = phi ptr [ %31, %33 ], [ %56, %117 ]
 49064|     ;; old = ptr %55
 49065|     ;; src = ptr %55
 49066|     ;; self = ptr %55
 49067|  %56 = gep %55, i64 184                                                                                                ;L384<2448<107<2155<607
 49068|  %57 = gep %55, i64 177                                                                                                ;L1733<2450<107<2155<607
 49069|  %58 = load i8, ptr %57, , !!70215                                                                                     ;L1733<2450<107<2155<607
 49070|     ;; self[177..+1] = i8 %58
 49071|     ;; f = ptr %7
 49072|     ;; self = ptr %7
 49073|  %59 = icmp eq i8 %58, -1                                                                                              ;L1161<107<2155<607
 49074|  br i1 %59, label %102, label %60                                                                                      ;L1161<107<2155<607
 49075| 
 49076| 60: ; preds = %54
 49077|  %61 = gep %55, i64 178                                                                                                ;L1733<2450<107<2155<607
 49079|  call void @llvm.memcpy.p0.p0.i64(ptr %5, ptr %55, i64 177, i1 false), !!70245                                         ;L1162<107<2155<607
 49080|     ;; x[177..+1] = i8 %58
 49081|  call void @llvm.memcpy.p0.p0.i64(ptr %34, ptr %61, i64 6, i1 false), !!70245                                          ;L1162<107<2155<607
 49082|  store i8 %58, ptr %35, , !!70243                                                                                      ;L1162<107<2155<607
 49091|     ;; c = ptr %5
 49092|  %62 = load i64, ptr %44, , !!70266, !!8                                                                               ;L178<310<1162<107<2155<607
 49093|  %63 = invoke i64 @ai::plan_legacy8sub_planNtB4_7SubPlan5score(ptr %43, i64 %62, ptr %45, ptr %46, ptr %47, ptr %48, ptr %5, ptr %49)
 49094|  to label %66 unwind label %64, !!70271                                                                                ;L178<310<1162<107<2155<607
 49095| 
 49096| 64: ; preds = %60
 49097|  %65 = cleanuppad within none []
 49098|  store ptr %56, ptr %28, , !!70106                                                                                     ;L0<107<2155<607
 49099|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %5) #20 [ "funclet"(token %65) ], !!70271 ;L185<310<1162<107<2155<607
 49100|  cleanupret from %65 unwind label %90
 49101| 
 49102| 66: ; preds = %60
 49103|     ;; score = i64 %63
 49104|     ;; self = i64 %63
 49105|     ;; self = ptr %5
 49106|  %67 = icmp ne i8 %58, 10                                                                                              ;L309<179<310<1162<107<2155<607
 49107|  call void @llvm.assume(i1 %67)                                                                                        ;L309<179<310<1162<107<2155<607
 49108|  %68 = add nsw i8 %58, -3                                                                                              ;L309<179<310<1162<107<2155<607
 49109|  %69 = icmp samesign ugt i8 %58, 2                                                                                     ;L309<179<310<1162<107<2155<607
 49110|  %70 = select i1 %69, i8 %68, i8 7                                                                                     ;L309<179<310<1162<107<2155<607
 49111|  switch i8 %70, label %71 [
 49112|  i8 0, label %81
 49113|  i8 1, label %81
 49114|  i8 2, label %72
 49115|  i8 3, label %72
 49116|  i8 4, label %73
 49117|  i8 5, label %81
 49118|  i8 6, label %74
 49119|  i8 7, label %73
 49120|  i8 8, label %73
 49121|  i8 9, label %73
 49122|  i8 10, label %72
 49123|  i8 11, label %75
 49124|  i8 12, label %76
 49125|  i8 13, label %77
 49126|  i8 14, label %78
 49127|  i8 15, label %79
 49128|  i8 16, label %80
 49129|  ]                                                                                                                     ;L309<179<310<1162<107<2155<607
 49130| 
 49131| 71: ; preds = %66
 49132|  unreachable                                                                                                           ;L309<179<310<1162<107<2155<607
 49133| 
 49134| 72: ; preds = %66, %66, %66
 49135|  br label %81                                                                                                          ;L316<179<310<1162<107<2155<607
 49136| 
 49137| 73: ; preds = %66, %66, %66, %66
 49138|  br label %81                                                                                                          ;L314<179<310<1162<107<2155<607
 49139| 
 49140| 74: ; preds = %66
 49141|  br label %81                                                                                                          ;L312<179<310<1162<107<2155<607
 49142| 
 49143| 75: ; preds = %66
 49144|  br label %81                                                                                                          ;L321<179<310<1162<107<2155<607
 49145| 
 49146| 76: ; preds = %66
 49147|  br label %81                                                                                                          ;L322<179<310<1162<107<2155<607
 49148| 
 49149| 77: ; preds = %66
 49150|  br label %81                                                                                                          ;L323<179<310<1162<107<2155<607
 49151| 
 49152| 78: ; preds = %66
 49153|  br label %81                                                                                                          ;L324<179<310<1162<107<2155<607
 49154| 
 49155| 79: ; preds = %66
 49156|  br label %81                                                                                                          ;L325<179<310<1162<107<2155<607
 49157| 
 49158| 80: ; preds = %66
 49159|  br label %81                                                                                                          ;L326<179<310<1162<107<2155<607
 49160| 
 49161| 81: ; preds = %80, %79, %78, %77, %76, %75, %74, %73, %72, %66, %66, %66
 49162|  %82 = phi i64 [ 10, %80 ], [ 9, %79 ], [ 8, %78 ], [ 7, %77 ], [ 6, %76 ], [ 4, %75 ], [ 3, %73 ], [ 0, %66 ], [ 2, %72 ], [ 0, %66 ], [ 1, %74 ], [ 0, %66 ]
 49164|  %83 = call i64 @llvm.abs.i64(i64 %63, i1 false)                                                                       ;L3648<180<310<1162<107<2155<607
 49165|  %84 = icmp slt i64 %83, 6                                                                                             ;L180<310<1162<107<2155<607
 49166|  br i1 %84, label %92, label %85                                                                                       ;L180<310<1162<107<2155<607
 49167| 
 49168| 85: ; preds = %81
 49169|  %86 = getelementptr i64, ptr %50, i64 %82                                                                             ;L179<310<1162<107<2155<607
 49170|  %87 = load i64, ptr %86, , !!70271, !!8                                                                               ;L179<310<1162<107<2155<607
 49171|     ;; ratio = i64 %87
 49172|  %88 = mul i64 %87, %63                                                                                                ;L183<310<1162<107<2155<607
 49173|  %89 = sdiv i64 %88, 1000                                                                                              ;L183<310<1162<107<2155<607
 49174|     ;; score = i64 %89
 49175|     ;; self = i64 %89
 49176|  br label %92                                                                                                          ;L180<310<1162<107<2155<607
 49177| 
 49178| 90: ; preds = %100, %64
 49179|  %91 = cleanuppad within none []
 49180|  call fastcc void @core::ptr9drop_glueINtNtNtNtB4_4iter8adapters3map3MapINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec8IntoIterNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayENCNvMNtNtNtB26_11plan_legacy7handler7auctionNtB35_17LegacyPlanHandler16get_small_actions1_0EEB26_(ptr %7) #20 [ "funclet"(token %91) ], !!70169 ;L2157<607
 49181|  cleanupret from %91 unwind label %22                                                                                  ;L2157<607
 49182| 
 49183| 92: ; preds = %85, %81
 49184|  %93 = phi i64 [ %89, %85 ], [ %63, %81 ]                                                                              ;L0<310<1162<107<2155<607
 49185|     ;; self = i64 %93
 49186|     ;; score = i64 %93
 49188|  call void @llvm.memcpy.p0.p0.i64(ptr %51, ptr %5, i64 177, i1 false), !!70106                                         ;L184<310<1162<107<2155<607
 49189|  call void @llvm.memcpy.p0.p0.i64(ptr %53, ptr %34, i64 6, i1 false), !!70106                                          ;L184<310<1162<107<2155<607
 49191|  store i64 %93, ptr %6, , !!70106                                                                                      ;L2155<607
 49192|  store i8 %58, ptr %52, , !!70106                                                                                      ;L2155<607
 49193|     ;; self = ptr %9
 49194|     ;; self = ptr %9
 49195|     ;; value = ptr %6
 49196|     ;; src = ptr %6
 49197|     ;; additional = i64 1
 49198|     ;; needed_extra_cap = i64 1
 49199|     ;; needed_extra_cap = i64 1
 49200|     ;; strategy = i8 1
 49201|  %94 = load i64, ptr %12, , !!70348, !!8                                                                               ;L1428<2156<607
 49202|     ;; self = ptr %9
 49203|  %95 = load i64, ptr %11, , !!70348, !!8                                                                               ;L149<1428<2156<607
 49204|  %96 = icmp eq i64 %94, %95                                                                                            ;L1428<2156<607
 49205|  br i1 %96, label %97, label %117                                                                                      ;L1428<2156<607
 49206| 
 49207| 97: ; preds = %92
 49208|     ;; self = ptr %9
 49209|     ;; self = ptr %9
 49210|     ;; self = ptr %9
 49211|     ;; used_cap = i64 %94
 49212|     ;; used_cap = i64 %94
 49213|  invoke void @ai::small_action15SmallActionPlayEE25reserve_internal_or_panicB19_(ptr %9, i64 %94, i64 1, i1 zeroext true)
 49214|  to label %98 unwind label %100, !!70348                                                                               ;L619<430<738<1429<2156<607
 49215| 
 49216| 98: ; preds = %97
 49217|  %99 = load i64, ptr %12, , !!70348                                                                                    ;L1432<2156<607
 49218|  br label %117                                                                                                         ;L619<430<738<1429<2156<607
 49219| 
 49220| 100: ; preds = %97
 49221|  %101 = cleanuppad within none []
 49222|  store ptr %56, ptr %28, , !!70106                                                                                     ;L0<107<2155<607
 49224|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %51) [ "funclet"(token %101) ], !!70169 ;L825<1436<2156<607
 49225|  cleanupret from %101 unwind label %90
 49226| 
 49227| 102: ; preds = %117, %54, %27
 49228|  %103 = phi ptr [ %31, %27 ], [ %56, %54 ], [ %56, %117 ]
 49231|     ;; self = ptr %28
 49232|     ;; self = ptr %28
 49233|     ;; self = ptr %28
 49234|     ;; self = ptr %28
 49245|     ;; self = ptr %28
 49249|  %104 = icmp eq ptr %103, %30                                                                                          ;L2436<2493<4312<380<4256<887<2499<825<825<2157<607
 49250|  br i1 %104, label %125, label %105                                                                                    ;L2436<2493<4312<380<4256<887<2499<825<825<2157<607
 49251| 
 49252| 105: ; preds = %102
 49253|  %106 = gep %4, i64 177
 49254|  %107 = gep %4, i64 178
 49255|  br label %108                                                                                                         ;L2436<2493<4312<380<4256<887<2499<825<825<2157<607
 49256| 
 49257| 108: ; preds = %113, %105
 49258|  %109 = phi ptr [ %103, %105 ], [ %114, %113 ]
 49259|     ;; old = ptr %109
 49260|     ;; src = ptr %109
 49261|     ;; self = ptr %109
 49262|  %110 = gep %109, i64 177                                                                                              ;L1733<2450<2493<4312<380<4256<887<2499<825<825<2157<607
 49263|  %111 = load i8, ptr %110, , !!70468                                                                                   ;L1733<2450<2493<4312<380<4256<887<2499<825<825<2157<607
 49264|  %112 = icmp eq i8 %111, -1                                                                                            ;L2493<4312<380<4256<887<2499<825<825<2157<607
 49265|  br i1 %112, label %125, label %113                                                                                    ;L2493<4312<380<4256<887<2499<825<825<2157<607
 49266| 
 49267| 113: ; preds = %108
 49268|  %114 = gep %109, i64 184                                                                                              ;L384<2448<2493<4312<380<4256<887<2499<825<825<2157<607
 49269|  %115 = gep %109, i64 178                                                                                              ;L1733<2450<2493<4312<380<4256<887<2499<825<825<2157<607
 49271|  call void @llvm.memcpy.p0.p0.i64(ptr %4, ptr %109, i64 177, i1 false), !!70482                                        ;L2493<4312<380<4256<887<2499<825<825<2157<607
 49272|     ;; x[177..+1] = i8 %111
 49273|  call void @llvm.memcpy.p0.p0.i64(ptr %107, ptr %115, i64 6, i1 false), !!70482                                        ;L2493<4312<380<4256<887<2499<825<825<2157<607
 49274|  store i8 %111, ptr %106, , !!70481                                                                                    ;L2494<4312<380<4256<887<2499<825<825<2157<607
 49276|     ;; b = ptr %4
 49278|     ;; item = ptr %4
 49281|     ;; _x = ptr %4
 49282|  call fastcc void @core::ptr9drop_glueNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEBF_(ptr %4), !!70482   ;L1004<166<884<420<2494<4312<380<4256<887<2499<825<825<2157<607
 49284|     ;; self = ptr %28
 49285|     ;; count = i64 1
 49286|  %116 = icmp eq ptr %114, %30                                                                                          ;L2436<2493<4312<380<4256<887<2499<825<825<2157<607
 49287|  br i1 %116, label %125, label %108                                                                                    ;L2436<2493<4312<380<4256<887<2499<825<825<2157<607
 49288| 
 49289| 117: ; preds = %98, %92
 49290|  %118 = phi i64 [ %99, %98 ], [ %94, %92 ]                                                                             ;L1434<2156<607
 49291|     ;; self = ptr %9
 49292|  %119 = load ptr, ptr %9, , !!70348, !!8, !!8                                                                          ;L138<1432<2156<607
 49293|     ;; self = ptr %119
 49294|     ;; count = i64 %118
 49295|  %120 = getelementptr { i64, { [177 x i8], i8, [6 x i8] } }, ptr %119, i64 %118                                        ;L961<1432<2156<607
 49296|     ;; end = ptr %120
 49297|     ;; dst = ptr %120
 49298|  call void @llvm.memcpy.p0.p0.i64(ptr %120, ptr %6, i64 192, i1 false), !!70169                                        ;L1933<1433<2156<607
 49299|  %121 = add i64 %118, 1                                                                                                ;L1434<2156<607
 49300|  store i64 %121, ptr %12, , !!70348                                                                                    ;L1434<2156<607
 49306|     ;; self = ptr %7
 49307|     ;; args = ptr %5
 49308|     ;; self = ptr %28
 49309|     ;; count = i64 1
 49310|  %122 = icmp eq ptr %56, %30                                                                                           ;L2436<107<2155<607
 49311|  br i1 %122, label %102, label %54                                                                                     ;L2436<107<2155<607
 49312| 
 49313| 123: ; preds = %124, %22
 49314|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecTxNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayEEEB1v_(ptr %9) #20 [ "funclet"(token %24) ] ;L609
 49315|  cleanupret from %24 unwind to caller                                                                                  ;L605
 49316| 
 49317| 124: ; preds = %22
 49318|  call fastcc void @core::ptr9drop_glueINtNtNtNtB4_4iter8adapters3map3MapINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec8IntoIterNtNtCshdEBA0ozCnw_7game_ai12small_action15SmallActionPlayENCNvMNtNtNtB26_11plan_legacy7handler7auctionNtB35_17LegacyPlanHandler16get_small_actions1_0EEB26_(ptr %8) #20 [ "funclet"(token %24) ], !!70169 ;L2158<607
 49319|  br label %123                                                                                                         ;L2158<607
 49320| 
 49321| 125: ; preds = %113, %108, %102
 49324|  call void @llvm.memcpy.p0.p0.i64(ptr %0, ptr %9, i64 32, i1 false)                                                    ;L608
 49326|  ret void                                                                                                              ;L609
 49327| }
