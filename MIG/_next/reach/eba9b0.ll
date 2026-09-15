 31120| define internal fastcc i64 @ai::fight_check28check_kill_die_tick_uncached(i64 %0, ptr %1, ptr %2, ptr %3, ptr dead_on_return %4, ptr dead_on_return %5) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
 31121|  %7 = alloca [8 x i8],
 31123|  %8 = alloca [16 x i8],
 31166|     ;; version = i64 %0
 31168|     ;; data = ptr %1
 31169|     ;; judger = ptr %2
 31170|     ;; focus = ptr %3
 31171|     ;; other = ptr %3
 31172|     ;; enemy = ptr %4
 31173|     ;; towers = ptr %5
 31175|     ;; iter = ptr %8
 31176|     ;; rhs = i64 -7046029254386353131
 31177|     ;; count = i64 1
 31178|     ;; count = i64 1
 31179|     ;; count = i64 1
 31180|     ;; count = i64 1
 31181|     ;; len = i64 5
 31182|     ;; count = i64 5
 31183|     ;; count = i64 1
 31184|     ;; count = i64 1
 31185|     ;; init = i64 0
 31187|  %9 = gep %3, i64 1160                                                                                                 ;L976
 31188|  %10 = load i8, ptr %9, , !!8                                                                                          ;L976
 31189|  %11 = trunc nuw i8 %10 to i1                                                                                          ;L976
 31190|  br i1 %11, label %18, label %12                                                                                       ;L976
 31191| 
 31192| 12: ; preds = %6
 31193|  %13 = load ptr, ptr %1, , !!8, !!8                                                                                    ;L979
 31194|     ;; self = ptr %13
 31195|  %14 = gep %3, i64 1472                                                                                                ;L979
 31196|  %15 = load i64, ptr %14, , !!8                                                                                        ;L979
 31197|  %16 = tail call fastcc ptr @gc::simulationNtB5_21AbstractGameWithCache21player_by_champion_id(ptr %13, i64 %15)       ;L979
 31198|     ;; self = ptr %16
 31199|  %17 = icmp eq ptr %16, null                                                                                           ;L1011<979
 31200|  br i1 %17, label %25, label %27                                                                                       ;L1011<979
 31201| 
 31202| 18: ; preds = %6
 31204|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %5)
 31205|  to label %22 unwind label %19                                                                                         ;L825<1110
 31206| 
 31207| 19: ; preds = %18
 31208|  %20 = cleanuppad within none []
 31210|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %5) [ "funclet"(token %20) ]
 31211|  to label %21 unwind label %710                                                                                        ;L825<825<1110
 31212| 
 31213| 21: ; preds = %19
 31214|  cleanupret from %20 unwind label %710
 31215| 
 31216| 22: ; preds = %18
 31218|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %5)
 31219|  to label %733 unwind label %710                                                                                       ;L825<825<1110
 31220| 
 31221| 23: ; preds = %723, %684, %675, %652, %645, %630, %619, %608, %592, %566, %556, %545, %518, %483, %462, %461, %458, %412, %398, %378, %348, %333, %320, %318, %288, %284, %250, %246, %204, %200, %158, %154, %114, %110, %87, %85, %49, %44, %27, %25
 31222|  %24 = cleanuppad within none []
 31223|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEECshdEBA0ozCnw_7game_ai(ptr %5) #32 [ "funclet"(token %24) ] ;L1110
 31224|  cleanupret from %24 unwind label %710                                                                                 ;L1110
 31225| 
 31226| 25: ; preds = %12
 31227|  invoke void @core::option13unwrap_failed(ptr @anon.e5f9a016f1220e5cee86475419aee1e5.195) #30
 31228|  to label %26 unwind label %23                                                                                         ;L1013<979
 31229| 
 31230| 26: ; preds = %608, %518, %483, %461, %412, %348, %288, %250, %204, %158, %114, %87, %49, %25
 31231|  unreachable
 31232| 
 31233| 27: ; preds = %12
 31234|     ;; player = ptr %16
 31235|  %28 = gep %1, i64 8                                                                                                   ;L986
 31236|  %29 = load ptr, ptr %28, , !!8, !!8                                                                                   ;L986
 31237|     ;; self = ptr %29
 31238|  %30 = gep %29, i64 8                                                                                                  ;L986
 31239|  %31 = load ptr, ptr %30, , !!8, !!8                                                                                   ;L986
 31240|     ;; self = ptr %31
 31241|  %32 = gep %31, i64 4856                                                                                               ;L986
 31242|  %33 = load i64, ptr %32, , !!8                                                                                        ;L986
 31243|     ;; self = i64 %33
 31244|     ;; other = i64 1
 31246|  %34 = load ptr, ptr %13, , !!8, !!8                                                                                   ;L987
 31247|  %35 = gep %13, i64 8                                                                                                  ;L987
 31248|  %36 = load ptr, ptr %35, , !!8, !!8                                                                                   ;L987
 31249|  %37 = gep %36, i64 40                                                                                                 ;L987
 31250|  %38 = load ptr, ptr %37, , !!8                                                                                        ;L987
 31251|  %39 = invoke i64 %38(ptr %34)
 31252|  to label %40 unwind label %23                                                                                         ;L987
 31253| 
 31254| 40: ; preds = %27
 31255|  %41 = tail call i64 @llvm.umax.i64(i64 %33, i64 1)                                                                    ;L1039<986
 31256|     ;; tps = i64 %41
 31257|  %42 = shl i64 %41, 1                                                                                                  ;L987
 31258|  %43 = icmp eq i64 %42, 0                                                                                              ;L987
 31259|  br i1 %43, label %49, label %44                                                                                       ;L987
 31260| 
 31261| 44: ; preds = %40
 31263|  %45 = gep %2, i64 2344                                                                                                ;L989
 31264|  %46 = load i64, ptr %45, , !!8                                                                                        ;L989
 31265|     ;; self = i64 %46
 31267|     ;; rnd = ptr undef
 31268|     ;; self = ptr undef
 31269|     ;; self = ptr undef
 31270|     ;; self = ptr undef
 31271|     ;; self = ptr undef
 31272|     ;; self = ptr undef
 31273|     ;; self = ptr undef
 31274|     ;; self = ptr undef
 31275|     ;; self = ptr undef
 31276|     ;; self = ptr undef
 31277|  %47 = gep %2, i64 384                                                                                                 ;L994
 31278|  %48 = invoke i64 @gc::simulation5state6playerNtB4_16AthleteParameter14judge_accuracy(ptr %47)
 31279|  to label %50 unwind label %23                                                                                         ;L994
 31280| 
 31281| 49: ; preds = %40
 31282|  invoke void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.e5f9a016f1220e5cee86475419aee1e5.196) #30
 31283|  to label %26 unwind label %23                                                                                         ;L987
 31284| 
 31285| 50: ; preds = %44
 31286|  %51 = mul i64 %46, -7046029254386353131                                                                               ;L2660<989
 31287|  %52 = shl i64 %15, 24                                                                                                 ;L990
 31288|  %53 = xor i64 %51, %52                                                                                                ;L989
 31289|  %54 = udiv i64 %39, %42                                                                                               ;L987
 31290|     ;; bucket = i64 %54
 31291|     ;; stable_rng = !DIArgList(i64 %53, i64 %54)
 31292|  %55 = xor i64 %53, %54                                                                                                ;L989
 31293|     ;; stable_rng = i64 %55
 31294|     ;; judge_accuracy = i64 %48
 31295|  %56 = sub i64 1000, %48                                                                                               ;L995
 31296|  %57 = lshr i64 %56, 1                                                                                                 ;L995
 31297|  %58 = sub nsw i64 1000, %57                                                                                           ;L995
 31298|     ;; range_min = i64 %58
 31299|     ;; lo = i64 %58
 31300|     ;; lo = i64 %58
 31301|     ;; lo = i64 %58
 31302|     ;; lo = i64 %58
 31303|     ;; lo = i64 %58
 31304|     ;; lo = i64 %58
 31305|     ;; lo = i64 %58
 31306|     ;; lo = i64 %58
 31307|     ;; lo = i64 %58
 31308|     ;; range_max = i64 %57
 31309|     ;; hi = i64 %57
 31310|     ;; hi = i64 %57
 31311|     ;; hi = i64 %57
 31312|     ;; hi = i64 %57
 31313|     ;; hi = i64 %57
 31314|     ;; hi = i64 %57
 31315|     ;; hi = i64 %57
 31316|     ;; hi = i64 %57
 31317|     ;; hi = i64 %57
 31318|     ;; enemy_dps = i64 0
 31319|     ;; enemy_dps = i64 0
 31320|     ;; self = i64 0
 31321|     ;; enemy_nuke = i64 0
 31322|     ;; enemy_nuke = i64 0
 31323|     ;; rhs = i64 0
 31324|     ;; self = ptr %4
 31325|     ;; self = ptr %4
 31326|     ;; self = ptr %4
 31327|  %59 = load ptr, ptr %4, , !!8, !!8                                                                                    ;L138<2073<2136<1000
 31328|     ;; p = ptr %59
 31329|  %60 = gep %4, i64 24                                                                                                  ;L2075<2136<1000
 31330|  %61 = load i64, ptr %60, , !!8                                                                                        ;L2075<2136<1000
 31331|     ;; len = i64 %61
 31332|     ;; count = i64 %61
 31333|     ;; self[0..+8] = ptr %59
 31334|     ;; slice[0..+8] = ptr %59
 31335|     ;; self[8..+8] = i64 %61
 31336|     ;; slice[8..+8] = i64 %61
 31337|     ;; ptr = ptr %59
 31338|     ;; self = ptr %59
 31339|  %62 = getelementptr ptr, ptr %59, i64 %61                                                                             ;L961<100<1042<2136<1000
 31340|     ;; iter[0..+8] = ptr %59
 31341|     ;; iter[8..+8] = ptr %62
 31342|  %63 = gep %16, i64 2496
 31343|  %64 = gep %13, i64 640
 31344|  %65 = or i64 %56, 1
 31345|  %66 = zext i64 %65 to i128
 31346|  br label %67                                                                                                          ;L1000
 31347| 
 31348| 67: ; preds = %380, %50
 31349|  %68 = phi i64 [ %55, %50 ], [ %381, %380 ]                                                                            ;L0
 31350|  %69 = phi ptr [ %59, %50 ], [ %74, %380 ]                                                                             ;L1000
 31351|  %70 = phi i64 [ 0, %50 ], [ %383, %380 ]                                                                              ;L0
 31352|  %71 = phi i64 [ 0, %50 ], [ %382, %380 ]                                                                              ;L998
 31353|     ;; stable_rng = i64 %68
 31354|     ;; self = i64 %71
 31355|     ;; enemy_dps = i64 %71
 31356|     ;; enemy_dps = i64 %71
 31357|     ;; rhs = i64 %70
 31358|     ;; enemy_nuke = i64 %70
 31359|     ;; enemy_nuke = i64 %70
 31360|     ;; iter[0..+8] = ptr %69
 31361|     ;; self = ptr undef
 31362|     ;; ptr = ptr %69
 31363|     ;; self = ptr %69
 31364|     ;; end_or_len = ptr %62
 31367|  %72 = icmp eq ptr %69, %62                                                                                            ;L1714<180<1000
 31368|  br i1 %72, label %80, label %73                                                                                       ;L180<1000
 31369| 
 31370| 73: ; preds = %67
 31371|  %74 = gep %69, i64 8                                                                                                  ;L656<185<1000
 31372|     ;; iter[0..+8] = ptr %74
 31373|  %75 = load ptr, ptr %69, , !!8, !!8                                                                                   ;L1000
 31374|     ;; pchamp = ptr %75
 31375|     ;; self = ptr %75
 31376|     ;; self = ptr %75
 31377|     ;; self = ptr %75
 31378|     ;; self = ptr %75
 31379|     ;; self = ptr %75
 31380|  %76 = gep %75, i64 1472                                                                                               ;L1002
 31381|  %77 = load i64, ptr %76, , !!8                                                                                        ;L1002
 31382|  %78 = tail call fastcc ptr @gc::simulationNtB5_21AbstractGameWithCache21player_by_champion_id(ptr %13, i64 %77)       ;L1002
 31383|     ;; self = ptr %78
 31384|  %79 = icmp eq ptr %78, null                                                                                           ;L1011<1002
 31385|  br i1 %79, label %87, label %85                                                                                       ;L1011<1002
 31386| 
 31387| 80: ; preds = %67
 31388|     ;; self = ptr %5
 31389|     ;; self = ptr %5
 31390|     ;; self = ptr %5
 31391|  %81 = load ptr, ptr %5, , !!8, !!8                                                                                    ;L138<2073<2136<1049
 31392|     ;; p = ptr %81
 31393|  %82 = gep %5, i64 24                                                                                                  ;L2075<2136<1049
 31394|  %83 = load i64, ptr %82, , !!8                                                                                        ;L2075<2136<1049
 31395|     ;; len = i64 %83
 31396|     ;; count = i64 %83
 31397|     ;; self[0..+8] = ptr %81
 31398|     ;; slice[0..+8] = ptr %81
 31399|     ;; self[8..+8] = i64 %83
 31400|     ;; slice[8..+8] = i64 %83
 31401|     ;; ptr = ptr %81
 31402|     ;; self = ptr %81
 31403|  %84 = getelementptr ptr, ptr %81, i64 %83                                                                             ;L961<100<1042<2136<1049
 31404|     ;; iter[0..+8] = ptr %81
 31405|     ;; iter[8..+8] = ptr %84
 31406|  br label %441                                                                                                         ;L1049
 31407| 
 31408| 85: ; preds = %73
 31409|     ;; p = ptr %78
 31410|     ;; nuke = i64 0
 31411|  %86 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_attack(ptr %75)
 31412|  to label %88 unwind label %23                                                                                         ;L1005
 31413| 
 31414| 87: ; preds = %73
 31415|  invoke void @core::option13unwrap_failed(ptr @anon.e5f9a016f1220e5cee86475419aee1e5.197) #30
 31416|  to label %26 unwind label %23                                                                                         ;L1013<1002
 31417| 
 31418| 88: ; preds = %85
 31419|  br i1 %86, label %92, label %89                                                                                       ;L1005
 31420| 
 31421| 89: ; preds = %88
 31422|  %90 = gep %75, i64 104                                                                                                ;L1748<1005
 31423|  %91 = load i64, ptr %90, , !!8                                                                                        ;L1748<1005
 31424|  switch i64 %91, label %96 [
 31425|  i64 0, label %92
 31426|  i64 1, label %105
 31427|  i64 2, label %97
 31428|  i64 3, label %92
 31429|  i64 4, label %98
 31430|  i64 5, label %99
 31431|  i64 6, label %99
 31432|  i64 7, label %98
 31433|  i64 8, label %100
 31434|  i64 9, label %101
 31435|  i64 10, label %102
 31436|  i64 11, label %103
 31437|  i64 12, label %104
 31438|  i64 13, label %100
 31439|  ]                                                                                                                     ;L1748<1005
 31440| 
 31441| 92: ; preds = %105, %89, %89, %88
 31442|  %93 = gep %78, i64 2352                                                                                               ;L1006
 31443|  %94 = load i64, ptr %93, , !!8                                                                                        ;L1006
 31444|  %95 = icmp ult i64 %94, 2                                                                                             ;L1006
 31445|  br i1 %95, label %115, label %114                                                                                     ;L1006
 31446| 
 31447| 96: ; preds = %89
 31448|  unreachable
 31449| 
 31450| 97: ; preds = %89
 31451|     ;; info = ptr %75
 31452|  br label %105                                                                                                         ;L1758<1005
 31453| 
 31454| 98: ; preds = %89, %89
 31455|     ;; info = ptr %75
 31456|  br label %105                                                                                                         ;L1750<1005
 31457| 
 31458| 99: ; preds = %89, %89
 31459|     ;; info = ptr %75
 31460|  br label %105                                                                                                         ;L1760<1005
 31461| 
 31462| 100: ; preds = %89, %89
 31463|     ;; info = ptr %75
 31464|  br label %105                                                                                                         ;L1753<1005
 31465| 
 31466| 101: ; preds = %89
 31467|     ;; info = ptr %75
 31468|  br label %105                                                                                                         ;L1754<1005
 31469| 
 31470| 102: ; preds = %89
 31471|     ;; info = ptr %75
 31472|  br label %105                                                                                                         ;L1755<1005
 31473| 
 31474| 103: ; preds = %89
 31475|     ;; info = ptr %75
 31476|  br label %105                                                                                                         ;L1756<1005
 31477| 
 31478| 104: ; preds = %89
 31479|     ;; info = ptr %75
 31480|  br label %105                                                                                                         ;L1757<1005
 31481| 
 31482| 105: ; preds = %104, %103, %102, %101, %100, %99, %98, %97, %89
 31483|  %106 = phi i64 [ 232, %98 ], [ 208, %104 ], [ 216, %103 ], [ 240, %102 ], [ 200, %101 ], [ 176, %100 ], [ 272, %97 ], [ 184, %89 ], [ 496, %99 ]
 31484|  %107 = gep %75, i64 %106                                                                                              ;L0<1005
 31485|  %108 = load i64, ptr %107, , !!8                                                                                      ;L0<1005
 31486|  %109 = icmp ugt i64 %108, %33                                                                                         ;L1005
 31487|  br i1 %109, label %110, label %92                                                                                     ;L1005
 31488| 
 31489| 110: ; preds = %115, %105
 31490|  %111 = phi i64 [ %125, %115 ], [ %68, %105 ]                                                                          ;L0
 31491|  %112 = phi i64 [ %140, %115 ], [ 0, %105 ]                                                                            ;L0
 31492|     ;; stable_rng = i64 %111
 31493|     ;; nuke = i64 %112
 31494|  %113 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity9can_skill(ptr %75)
 31495|  to label %141 unwind label %23                                                                                        ;L1010
 31496| 
 31497| 114: ; preds = %92
 31498|  invoke void @core::panicking18panic_bounds_check(i64 %94, i64 2, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.198) #30
 31499|  to label %26 unwind label %23                                                                                         ;L1006
 31500| 
 31501| 115: ; preds = %92
 31502|     ;; self = ptr %78
 31503|  %116 = gep %78, i64 2496                                                                                              ;L581<1006
 31504|  %117 = load i32, ptr %116, , !!8                                                                                      ;L581<1006
 31505|  %118 = zext nneg i32 %117 to i64                                                                                      ;L581<1006
 31506|     ;; self = ptr %16
 31507|  %119 = load i32, ptr %63, , !!8                                                                                       ;L581<1006
 31508|  %120 = zext nneg i32 %119 to i64                                                                                      ;L581<1006
 31509|  %121 = getelementptr [5 x { [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64] }], ptr %64, i64 %94 ;L1006
 31510|  %122 = gepS %121, i64 %118                                                                                            ;L1006
 31511|  %123 = getelementptr i64, ptr %122, i64 %120                                                                          ;L1006
 31512|  %124 = load i64, ptr %123, , !!8                                                                                      ;L1006
 31513|     ;; self = ptr undef
 31514|     ;; self = ptr undef
 31515|     ;; self = ptr undef
 31516|     ;; lo = i64 %58
 31517|     ;; hi = i64 %57
 31518|     ;; rhs = i64 1
 31519|     ;; rhs = i64 -7046029254386353131
 31520|     ;; rhs = i64 -4658895280553007687
 31521|     ;; rhs = i64 -7723592293110705685
 31522|     ;; rhs = i64 -7046029254386353131
 31523|     ;; rhs = i64 -4658895280553007687
 31524|     ;; rhs = i64 -7723592293110705685
 31525|     ;; self = !DIArgList(i64 %57, i64 %58)
 31526|     ;; span = !DIArgList(i64 %57, i64 %58)
 31527|     ;; self = i64 %68
 31528|  %125 = add i64 %68, -7046029254386353131                                                                              ;L2584<182<197<202<1007
 31529|     ;; stable_rng = i64 %125
 31530|     ;; self = !DIArgList(i64 %125, i64 %125)
 31531|     ;; self = !DIArgList(i64 %125, i64 %125, i64 %125, i64 %125)
 31532|     ;; z = !DIArgList(i64 %125, i64 %125, i64 %125, i64 %125)
 31533|  %126 = lshr i64 %125, 30                                                                                              ;L184<197<202<1007
 31534|     ;; self = !DIArgList(i64 %125, i64 %125, i64 %126, i64 %126)
 31535|     ;; z = !DIArgList(i64 %125, i64 %125, i64 %126, i64 %126)
 31536|     ;; self = !DIArgList(i64 %125, i64 %126)
 31537|  %127 = xor i64 %126, %125                                                                                             ;L184<197<202<1007
 31538|     ;; z = !DIArgList(i64 %127, i64 %127)
 31539|     ;; self = !DIArgList(i64 %127, i64 %127)
 31540|     ;; self = i64 %127
 31541|  %128 = mul i64 %127, -4658895280553007687                                                                             ;L2660<184<197<202<1007
 31542|     ;; self = !DIArgList(i64 %128, i64 %128)
 31543|     ;; z = !DIArgList(i64 %128, i64 %128)
 31544|  %129 = lshr i64 %128, 27                                                                                              ;L185<197<202<1007
 31545|     ;; z = !DIArgList(i64 %128, i64 %129)
 31546|     ;; self = !DIArgList(i64 %128, i64 %129)
 31547|  %130 = xor i64 %129, %128                                                                                             ;L185<197<202<1007
 31548|     ;; self = i64 %130
 31549|     ;; z = i64 %130
 31550|  %131 = mul i64 %130, -7723592293110705685                                                                             ;L2660<185<197<202<1007
 31551|     ;; z = i64 %131
 31552|  %132 = lshr i64 %131, 31                                                                                              ;L186<197<202<1007
 31553|  %133 = xor i64 %132, %131                                                                                             ;L186<197<202<1007
 31554|  %134 = zext i64 %133 to i128                                                                                          ;L197<202<1007
 31556|     ;; span = i64 %65
 31557|  %135 = mul nuw i128 %134, %66                                                                                         ;L197<202<1007
 31558|  %136 = lshr i128 %135, 64                                                                                             ;L197<202<1007
 31559|  %137 = trunc nuw i128 %136 to i64                                                                                     ;L197<202<1007
 31560|  %138 = add i64 %58, %137                                                                                              ;L197<202<1007
 31561|  %139 = mul i64 %124, %138                                                                                             ;L1006
 31562|  %140 = udiv i64 %139, 1000                                                                                            ;L1006
 31563|     ;; self = i64 0
 31564|     ;; other = i64 %140
 31565|  br label %110                                                                                                         ;L1039<1006
 31566| 
 31567| 141: ; preds = %110
 31568|  br i1 %113, label %146, label %142                                                                                    ;L1010
 31569| 
 31570| 142: ; preds = %141
 31571|  %143 = gep %75, i64 104                                                                                               ;L1775<1010
 31572|  %144 = load i64, ptr %143, , !!8                                                                                      ;L1775<1010
 31573|  %145 = icmp eq i64 %144, 13                                                                                           ;L1775<1010
 31574|  br i1 %145, label %150, label %146                                                                                    ;L1775<1010
 31575| 
 31576| 146: ; preds = %150, %142, %141
 31577|  %147 = gep %78, i64 2352                                                                                              ;L1011
 31578|  %148 = load i64, ptr %147, , !!8                                                                                      ;L1011
 31579|  %149 = icmp ult i64 %148, 2                                                                                           ;L1011
 31580|  br i1 %149, label %159, label %158                                                                                    ;L1011
 31581| 
 31582| 150: ; preds = %142
 31583|     ;; champ = ptr %75
 31584|  %151 = gep %75, i64 184                                                                                               ;L1776<1010
 31585|  %152 = load i64, ptr %151, , !!8                                                                                      ;L1776<1010
 31586|  %153 = icmp ugt i64 %152, %33                                                                                         ;L1010
 31587|  br i1 %153, label %154, label %146                                                                                    ;L1010
 31588| 
 31589| 154: ; preds = %159, %150
 31590|  %155 = phi i64 [ %170, %159 ], [ %111, %150 ]                                                                         ;L182<197<202<0
 31591|  %156 = phi i64 [ %186, %159 ], [ %112, %150 ]                                                                         ;L0
 31592|     ;; stable_rng = i64 %155
 31593|     ;; nuke = i64 %156
 31594|  %157 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity10can_skill2(ptr %75)
 31595|  to label %187 unwind label %23                                                                                        ;L1015
 31596| 
 31597| 158: ; preds = %146
 31598|  invoke void @core::panicking18panic_bounds_check(i64 %148, i64 2, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.199) #30
 31599|  to label %26 unwind label %23                                                                                         ;L1011
 31600| 
 31601| 159: ; preds = %146
 31602|     ;; self = ptr %78
 31603|  %160 = gep %78, i64 2496                                                                                              ;L581<1011
 31604|  %161 = load i32, ptr %160, , !!8                                                                                      ;L581<1011
 31605|  %162 = zext nneg i32 %161 to i64                                                                                      ;L581<1011
 31606|     ;; self = ptr %16
 31607|  %163 = load i32, ptr %63, , !!8                                                                                       ;L581<1011
 31608|  %164 = zext nneg i32 %163 to i64                                                                                      ;L581<1011
 31609|  %165 = getelementptr [5 x { [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64] }], ptr %13, i64 %148 ;L1011
 31610|  %166 = gepS %165, i64 %162                                                                                            ;L1011
 31611|  %167 = gep %166, i64 680                                                                                              ;L1011
 31612|  %168 = getelementptr i64, ptr %167, i64 %164                                                                          ;L1011
 31613|  %169 = load i64, ptr %168, , !!8                                                                                      ;L1011
 31614|     ;; self = ptr undef
 31615|     ;; self = ptr undef
 31616|     ;; self = ptr undef
 31617|     ;; lo = i64 %58
 31618|     ;; hi = i64 %57
 31619|     ;; rhs = i64 1
 31620|     ;; rhs = i64 -7046029254386353131
 31621|     ;; rhs = i64 -4658895280553007687
 31622|     ;; rhs = i64 -7723592293110705685
 31623|     ;; rhs = i64 -7046029254386353131
 31624|     ;; rhs = i64 -4658895280553007687
 31625|     ;; rhs = i64 -7723592293110705685
 31626|     ;; self = !DIArgList(i64 %57, i64 %58)
 31627|     ;; span = !DIArgList(i64 %57, i64 %58)
 31628|     ;; self = i64 %111
 31629|  %170 = add i64 %111, -7046029254386353131                                                                             ;L2584<182<197<202<1012
 31630|     ;; stable_rng = i64 %170
 31631|     ;; self = !DIArgList(i64 %170, i64 %170)
 31632|     ;; self = !DIArgList(i64 %170, i64 %170, i64 %170, i64 %170)
 31633|     ;; z = !DIArgList(i64 %170, i64 %170, i64 %170, i64 %170)
 31634|  %171 = lshr i64 %170, 30                                                                                              ;L184<197<202<1012
 31635|     ;; self = !DIArgList(i64 %170, i64 %170, i64 %171, i64 %171)
 31636|     ;; z = !DIArgList(i64 %170, i64 %170, i64 %171, i64 %171)
 31637|     ;; self = !DIArgList(i64 %170, i64 %171)
 31638|  %172 = xor i64 %171, %170                                                                                             ;L184<197<202<1012
 31639|     ;; z = !DIArgList(i64 %172, i64 %172)
 31640|     ;; self = !DIArgList(i64 %172, i64 %172)
 31641|     ;; self = i64 %172
 31642|  %173 = mul i64 %172, -4658895280553007687                                                                             ;L2660<184<197<202<1012
 31643|     ;; self = !DIArgList(i64 %173, i64 %173)
 31644|     ;; z = !DIArgList(i64 %173, i64 %173)
 31645|  %174 = lshr i64 %173, 27                                                                                              ;L185<197<202<1012
 31646|     ;; z = !DIArgList(i64 %173, i64 %174)
 31647|     ;; self = !DIArgList(i64 %173, i64 %174)
 31648|  %175 = xor i64 %174, %173                                                                                             ;L185<197<202<1012
 31649|     ;; self = i64 %175
 31650|     ;; z = i64 %175
 31651|  %176 = mul i64 %175, -7723592293110705685                                                                             ;L2660<185<197<202<1012
 31652|     ;; z = i64 %176
 31653|  %177 = lshr i64 %176, 31                                                                                              ;L186<197<202<1012
 31654|  %178 = xor i64 %177, %176                                                                                             ;L186<197<202<1012
 31655|  %179 = zext i64 %178 to i128                                                                                          ;L197<202<1012
 31657|     ;; span = i64 %65
 31658|  %180 = mul nuw i128 %179, %66                                                                                         ;L197<202<1012
 31659|  %181 = lshr i128 %180, 64                                                                                             ;L197<202<1012
 31660|  %182 = trunc nuw i128 %181 to i64                                                                                     ;L197<202<1012
 31661|  %183 = add i64 %58, %182                                                                                              ;L197<202<1012
 31662|  %184 = mul i64 %169, %183                                                                                             ;L1011
 31663|  %185 = udiv i64 %184, 1000                                                                                            ;L1011
 31664|     ;; self = i64 %112
 31665|     ;; other = i64 %185
 31666|  %186 = tail call i64 @llvm.umax.i64(i64 %185, i64 %112)                                                               ;L1039<1011
 31667|  br label %154                                                                                                         ;L1039<1011
 31668| 
 31669| 187: ; preds = %154
 31670|  br i1 %157, label %192, label %188                                                                                    ;L1015
 31671| 
 31672| 188: ; preds = %187
 31673|  %189 = gep %75, i64 104                                                                                               ;L1790<1015
 31674|  %190 = load i64, ptr %189, , !!8                                                                                      ;L1790<1015
 31675|  %191 = icmp eq i64 %190, 13                                                                                           ;L1790<1015
 31676|  br i1 %191, label %196, label %192                                                                                    ;L1790<1015
 31677| 
 31678| 192: ; preds = %196, %188, %187
 31679|  %193 = gep %78, i64 2352                                                                                              ;L1016
 31680|  %194 = load i64, ptr %193, , !!8                                                                                      ;L1016
 31681|  %195 = icmp ult i64 %194, 2                                                                                           ;L1016
 31682|  br i1 %195, label %205, label %204                                                                                    ;L1016
 31683| 
 31684| 196: ; preds = %188
 31685|     ;; champ = ptr %75
 31686|  %197 = gep %75, i64 192                                                                                               ;L1791<1015
 31687|  %198 = load i64, ptr %197, , !!8                                                                                      ;L1791<1015
 31688|  %199 = icmp ugt i64 %198, %33                                                                                         ;L1015
 31689|  br i1 %199, label %200, label %192                                                                                    ;L1015
 31690| 
 31691| 200: ; preds = %205, %196
 31692|  %201 = phi i64 [ %216, %205 ], [ %155, %196 ]                                                                         ;L182<197<202<0
 31693|  %202 = phi i64 [ %232, %205 ], [ %156, %196 ]                                                                         ;L0
 31694|     ;; stable_rng = i64 %201
 31695|     ;; nuke = i64 %202
 31696|  %203 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity7can_ult(ptr %75)
 31697|  to label %233 unwind label %23                                                                                        ;L1021
 31698| 
 31699| 204: ; preds = %192
 31700|  invoke void @core::panicking18panic_bounds_check(i64 %194, i64 2, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.200) #30
 31701|  to label %26 unwind label %23                                                                                         ;L1016
 31702| 
 31703| 205: ; preds = %192
 31704|     ;; self = ptr %78
 31705|  %206 = gep %78, i64 2496                                                                                              ;L581<1016
 31706|  %207 = load i32, ptr %206, , !!8                                                                                      ;L581<1016
 31707|  %208 = zext nneg i32 %207 to i64                                                                                      ;L581<1016
 31708|     ;; self = ptr %16
 31709|  %209 = load i32, ptr %63, , !!8                                                                                       ;L581<1016
 31710|  %210 = zext nneg i32 %209 to i64                                                                                      ;L581<1016
 31711|  %211 = getelementptr [5 x { [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64] }], ptr %13, i64 %194 ;L1016
 31712|  %212 = gepS %211, i64 %208                                                                                            ;L1016
 31713|  %213 = gep %212, i64 720                                                                                              ;L1016
 31714|  %214 = getelementptr i64, ptr %213, i64 %210                                                                          ;L1016
 31715|  %215 = load i64, ptr %214, , !!8                                                                                      ;L1016
 31716|     ;; self = ptr undef
 31717|     ;; self = ptr undef
 31718|     ;; self = ptr undef
 31719|     ;; lo = i64 %58
 31720|     ;; hi = i64 %57
 31721|     ;; rhs = i64 1
 31722|     ;; rhs = i64 -7046029254386353131
 31723|     ;; rhs = i64 -4658895280553007687
 31724|     ;; rhs = i64 -7723592293110705685
 31725|     ;; rhs = i64 -7046029254386353131
 31726|     ;; rhs = i64 -4658895280553007687
 31727|     ;; rhs = i64 -7723592293110705685
 31728|     ;; self = !DIArgList(i64 %57, i64 %58)
 31729|     ;; span = !DIArgList(i64 %57, i64 %58)
 31730|     ;; self = i64 %155
 31731|  %216 = add i64 %155, -7046029254386353131                                                                             ;L2584<182<197<202<1017
 31732|     ;; stable_rng = i64 %216
 31733|     ;; self = !DIArgList(i64 %216, i64 %216)
 31734|     ;; self = !DIArgList(i64 %216, i64 %216, i64 %216, i64 %216)
 31735|     ;; z = !DIArgList(i64 %216, i64 %216, i64 %216, i64 %216)
 31736|  %217 = lshr i64 %216, 30                                                                                              ;L184<197<202<1017
 31737|     ;; self = !DIArgList(i64 %216, i64 %216, i64 %217, i64 %217)
 31738|     ;; z = !DIArgList(i64 %216, i64 %216, i64 %217, i64 %217)
 31739|     ;; self = !DIArgList(i64 %216, i64 %217)
 31740|  %218 = xor i64 %217, %216                                                                                             ;L184<197<202<1017
 31741|     ;; z = !DIArgList(i64 %218, i64 %218)
 31742|     ;; self = !DIArgList(i64 %218, i64 %218)
 31743|     ;; self = i64 %218
 31744|  %219 = mul i64 %218, -4658895280553007687                                                                             ;L2660<184<197<202<1017
 31745|     ;; self = !DIArgList(i64 %219, i64 %219)
 31746|     ;; z = !DIArgList(i64 %219, i64 %219)
 31747|  %220 = lshr i64 %219, 27                                                                                              ;L185<197<202<1017
 31748|     ;; z = !DIArgList(i64 %219, i64 %220)
 31749|     ;; self = !DIArgList(i64 %219, i64 %220)
 31750|  %221 = xor i64 %220, %219                                                                                             ;L185<197<202<1017
 31751|     ;; self = i64 %221
 31752|     ;; z = i64 %221
 31753|  %222 = mul i64 %221, -7723592293110705685                                                                             ;L2660<185<197<202<1017
 31754|     ;; z = i64 %222
 31755|  %223 = lshr i64 %222, 31                                                                                              ;L186<197<202<1017
 31756|  %224 = xor i64 %223, %222                                                                                             ;L186<197<202<1017
 31757|  %225 = zext i64 %224 to i128                                                                                          ;L197<202<1017
 31759|     ;; span = i64 %65
 31760|  %226 = mul nuw i128 %225, %66                                                                                         ;L197<202<1017
 31761|  %227 = lshr i128 %226, 64                                                                                             ;L197<202<1017
 31762|  %228 = trunc nuw i128 %227 to i64                                                                                     ;L197<202<1017
 31763|  %229 = add i64 %58, %228                                                                                              ;L197<202<1017
 31764|  %230 = mul i64 %215, %229                                                                                             ;L1016
 31765|  %231 = udiv i64 %230, 1000                                                                                            ;L1016
 31766|     ;; self = i64 %156
 31767|     ;; other = i64 %231
 31768|  %232 = tail call i64 @llvm.umax.i64(i64 %231, i64 %156)                                                               ;L1039<1016
 31769|  br label %200                                                                                                         ;L1039<1016
 31770| 
 31771| 233: ; preds = %200
 31772|  br i1 %203, label %238, label %234                                                                                    ;L1021
 31773| 
 31774| 234: ; preds = %233
 31775|  %235 = gep %75, i64 104                                                                                               ;L1805<1021
 31776|  %236 = load i64, ptr %235, , !!8                                                                                      ;L1805<1021
 31777|  %237 = icmp eq i64 %236, 13                                                                                           ;L1805<1021
 31778|  br i1 %237, label %242, label %238                                                                                    ;L1805<1021
 31779| 
 31780| 238: ; preds = %242, %234, %233
 31781|  %239 = gep %78, i64 2352                                                                                              ;L1022
 31782|  %240 = load i64, ptr %239, , !!8                                                                                      ;L1022
 31783|  %241 = icmp ult i64 %240, 2                                                                                           ;L1022
 31784|  br i1 %241, label %251, label %250                                                                                    ;L1022
 31785| 
 31786| 242: ; preds = %234
 31787|     ;; champ = ptr %75
 31788|  %243 = gep %75, i64 200                                                                                               ;L1806<1021
 31789|  %244 = load i64, ptr %243, , !!8                                                                                      ;L1806<1021
 31790|  %245 = icmp ugt i64 %244, %33                                                                                         ;L1021
 31791|  br i1 %245, label %246, label %238                                                                                    ;L1021
 31792| 
 31793| 246: ; preds = %251, %242
 31794|  %247 = phi i64 [ %262, %251 ], [ %201, %242 ]                                                                         ;L182<197<202<0
 31795|  %248 = phi i64 [ %278, %251 ], [ %202, %242 ]                                                                         ;L0
 31796|     ;; stable_rng = i64 %247
 31797|     ;; nuke = i64 %248
 31798|  %249 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity15is_block_attack(ptr %75)
 31799|  to label %279 unwind label %23                                                                                        ;L1026
 31800| 
 31801| 250: ; preds = %238
 31802|  invoke void @core::panicking18panic_bounds_check(i64 %240, i64 2, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.201) #30
 31803|  to label %26 unwind label %23                                                                                         ;L1022
 31804| 
 31805| 251: ; preds = %238
 31806|     ;; self = ptr %78
 31807|  %252 = gep %78, i64 2496                                                                                              ;L581<1022
 31808|  %253 = load i32, ptr %252, , !!8                                                                                      ;L581<1022
 31809|  %254 = zext nneg i32 %253 to i64                                                                                      ;L581<1022
 31810|     ;; self = ptr %16
 31811|  %255 = load i32, ptr %63, , !!8                                                                                       ;L581<1022
 31812|  %256 = zext nneg i32 %255 to i64                                                                                      ;L581<1022
 31813|  %257 = getelementptr [5 x { [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64] }], ptr %13, i64 %240 ;L1022
 31814|  %258 = gepS %257, i64 %254                                                                                            ;L1022
 31815|  %259 = gep %258, i64 760                                                                                              ;L1022
 31816|  %260 = getelementptr i64, ptr %259, i64 %256                                                                          ;L1022
 31817|  %261 = load i64, ptr %260, , !!8                                                                                      ;L1022
 31818|     ;; self = ptr undef
 31819|     ;; self = ptr undef
 31820|     ;; self = ptr undef
 31821|     ;; lo = i64 %58
 31822|     ;; hi = i64 %57
 31823|     ;; rhs = i64 1
 31824|     ;; rhs = i64 -7046029254386353131
 31825|     ;; rhs = i64 -4658895280553007687
 31826|     ;; rhs = i64 -7723592293110705685
 31827|     ;; rhs = i64 -7046029254386353131
 31828|     ;; rhs = i64 -4658895280553007687
 31829|     ;; rhs = i64 -7723592293110705685
 31830|     ;; self = !DIArgList(i64 %57, i64 %58)
 31831|     ;; span = !DIArgList(i64 %57, i64 %58)
 31832|     ;; self = i64 %201
 31833|  %262 = add i64 %201, -7046029254386353131                                                                             ;L2584<182<197<202<1023
 31834|     ;; stable_rng = i64 %262
 31835|     ;; self = !DIArgList(i64 %262, i64 %262)
 31836|     ;; self = !DIArgList(i64 %262, i64 %262, i64 %262, i64 %262)
 31837|     ;; z = !DIArgList(i64 %262, i64 %262, i64 %262, i64 %262)
 31838|  %263 = lshr i64 %262, 30                                                                                              ;L184<197<202<1023
 31839|     ;; self = !DIArgList(i64 %262, i64 %262, i64 %263, i64 %263)
 31840|     ;; z = !DIArgList(i64 %262, i64 %262, i64 %263, i64 %263)
 31841|     ;; self = !DIArgList(i64 %262, i64 %263)
 31842|  %264 = xor i64 %263, %262                                                                                             ;L184<197<202<1023
 31843|     ;; z = !DIArgList(i64 %264, i64 %264)
 31844|     ;; self = !DIArgList(i64 %264, i64 %264)
 31845|     ;; self = i64 %264
 31846|  %265 = mul i64 %264, -4658895280553007687                                                                             ;L2660<184<197<202<1023
 31847|     ;; self = !DIArgList(i64 %265, i64 %265)
 31848|     ;; z = !DIArgList(i64 %265, i64 %265)
 31849|  %266 = lshr i64 %265, 27                                                                                              ;L185<197<202<1023
 31850|     ;; z = !DIArgList(i64 %265, i64 %266)
 31851|     ;; self = !DIArgList(i64 %265, i64 %266)
 31852|  %267 = xor i64 %266, %265                                                                                             ;L185<197<202<1023
 31853|     ;; self = i64 %267
 31854|     ;; z = i64 %267
 31855|  %268 = mul i64 %267, -7723592293110705685                                                                             ;L2660<185<197<202<1023
 31856|     ;; z = i64 %268
 31857|  %269 = lshr i64 %268, 31                                                                                              ;L186<197<202<1023
 31858|  %270 = xor i64 %269, %268                                                                                             ;L186<197<202<1023
 31859|  %271 = zext i64 %270 to i128                                                                                          ;L197<202<1023
 31861|     ;; span = i64 %65
 31862|  %272 = mul nuw i128 %271, %66                                                                                         ;L197<202<1023
 31863|  %273 = lshr i128 %272, 64                                                                                             ;L197<202<1023
 31864|  %274 = trunc nuw i128 %273 to i64                                                                                     ;L197<202<1023
 31865|  %275 = add i64 %58, %274                                                                                              ;L197<202<1023
 31866|  %276 = mul i64 %261, %275                                                                                             ;L1022
 31867|  %277 = udiv i64 %276, 1000                                                                                            ;L1022
 31868|     ;; self = i64 %202
 31869|     ;; other = i64 %277
 31870|  %278 = tail call i64 @llvm.umax.i64(i64 %277, i64 %202)                                                               ;L1039<1022
 31871|  br label %246                                                                                                         ;L1039<1022
 31872| 
 31873| 279: ; preds = %246
 31874|  br i1 %249, label %284, label %280                                                                                    ;L1026
 31875| 
 31876| 280: ; preds = %279
 31877|  %281 = gep %78, i64 2352                                                                                              ;L1027
 31878|  %282 = load i64, ptr %281, , !!8                                                                                      ;L1027
 31879|  %283 = icmp ult i64 %282, 2                                                                                           ;L1027
 31880|  br i1 %283, label %289, label %288                                                                                    ;L1027
 31881| 
 31882| 284: ; preds = %289, %279
 31883|  %285 = phi i64 [ %247, %279 ], [ %300, %289 ]                                                                         ;L182<197<202<0
 31884|  %286 = phi i64 [ %71, %279 ], [ %316, %289 ]                                                                          ;L0
 31885|     ;; stable_rng = i64 %285
 31886|     ;; self = i64 %286
 31887|     ;; enemy_dps = i64 %286
 31888|     ;; enemy_dps = i64 %286
 31889|  %287 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity14is_block_skill(ptr %75)
 31890|  to label %317 unwind label %23                                                                                        ;L1031
 31891| 
 31892| 288: ; preds = %280
 31893|  invoke void @core::panicking18panic_bounds_check(i64 %282, i64 2, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.202) #30
 31894|  to label %26 unwind label %23                                                                                         ;L1027
 31895| 
 31896| 289: ; preds = %280
 31897|     ;; self = ptr %78
 31898|  %290 = gep %78, i64 2496                                                                                              ;L581<1027
 31899|  %291 = load i32, ptr %290, , !!8                                                                                      ;L581<1027
 31900|  %292 = zext nneg i32 %291 to i64                                                                                      ;L581<1027
 31901|     ;; self = ptr %16
 31902|  %293 = load i32, ptr %63, , !!8                                                                                       ;L581<1027
 31903|  %294 = zext nneg i32 %293 to i64                                                                                      ;L581<1027
 31904|  %295 = getelementptr [5 x { [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64] }], ptr %13, i64 %282 ;L1027
 31905|  %296 = gepS %295, i64 %292                                                                                            ;L1027
 31906|  %297 = gep %296, i64 1040                                                                                             ;L1027
 31907|  %298 = getelementptr i64, ptr %297, i64 %294                                                                          ;L1027
 31908|  %299 = load i64, ptr %298, , !!8                                                                                      ;L1027
 31909|     ;; self = ptr undef
 31910|     ;; self = ptr undef
 31911|     ;; self = ptr undef
 31912|     ;; lo = i64 %58
 31913|     ;; hi = i64 %57
 31914|     ;; rhs = i64 1
 31915|     ;; rhs = i64 -7046029254386353131
 31916|     ;; rhs = i64 -4658895280553007687
 31917|     ;; rhs = i64 -7723592293110705685
 31918|     ;; rhs = i64 -7046029254386353131
 31919|     ;; rhs = i64 -4658895280553007687
 31920|     ;; rhs = i64 -7723592293110705685
 31921|     ;; self = !DIArgList(i64 %57, i64 %58)
 31922|     ;; span = !DIArgList(i64 %57, i64 %58)
 31923|     ;; self = i64 %247
 31924|  %300 = add i64 %247, -7046029254386353131                                                                             ;L2584<182<197<202<1028
 31925|     ;; stable_rng = i64 %300
 31926|     ;; self = !DIArgList(i64 %300, i64 %300)
 31927|     ;; self = !DIArgList(i64 %300, i64 %300, i64 %300, i64 %300)
 31928|     ;; z = !DIArgList(i64 %300, i64 %300, i64 %300, i64 %300)
 31929|  %301 = lshr i64 %300, 30                                                                                              ;L184<197<202<1028
 31930|     ;; self = !DIArgList(i64 %300, i64 %300, i64 %301, i64 %301)
 31931|     ;; z = !DIArgList(i64 %300, i64 %300, i64 %301, i64 %301)
 31932|     ;; self = !DIArgList(i64 %300, i64 %301)
 31933|  %302 = xor i64 %301, %300                                                                                             ;L184<197<202<1028
 31934|     ;; z = !DIArgList(i64 %302, i64 %302)
 31935|     ;; self = !DIArgList(i64 %302, i64 %302)
 31936|     ;; self = i64 %302
 31937|  %303 = mul i64 %302, -4658895280553007687                                                                             ;L2660<184<197<202<1028
 31938|     ;; self = !DIArgList(i64 %303, i64 %303)
 31939|     ;; z = !DIArgList(i64 %303, i64 %303)
 31940|  %304 = lshr i64 %303, 27                                                                                              ;L185<197<202<1028
 31941|     ;; z = !DIArgList(i64 %303, i64 %304)
 31942|     ;; self = !DIArgList(i64 %303, i64 %304)
 31943|  %305 = xor i64 %304, %303                                                                                             ;L185<197<202<1028
 31944|     ;; self = i64 %305
 31945|     ;; z = i64 %305
 31946|  %306 = mul i64 %305, -7723592293110705685                                                                             ;L2660<185<197<202<1028
 31947|     ;; z = i64 %306
 31948|  %307 = lshr i64 %306, 31                                                                                              ;L186<197<202<1028
 31949|  %308 = xor i64 %307, %306                                                                                             ;L186<197<202<1028
 31950|  %309 = zext i64 %308 to i128                                                                                          ;L197<202<1028
 31952|     ;; span = i64 %65
 31953|  %310 = mul nuw i128 %309, %66                                                                                         ;L197<202<1028
 31954|  %311 = lshr i128 %310, 64                                                                                             ;L197<202<1028
 31955|  %312 = trunc nuw i128 %311 to i64                                                                                     ;L197<202<1028
 31956|  %313 = add i64 %58, %312                                                                                              ;L197<202<1028
 31957|  %314 = mul i64 %299, %313                                                                                             ;L1027
 31958|  %315 = udiv i64 %314, 1000                                                                                            ;L1027
 31959|  %316 = add i64 %315, %71                                                                                              ;L1027
 31960|     ;; enemy_dps = i64 %316
 31961|     ;; enemy_dps = i64 %316
 31962|     ;; self = i64 %316
 31963|  br label %284                                                                                                         ;L1026
 31964| 
 31965| 317: ; preds = %284
 31966|  br i1 %287, label %320, label %318                                                                                    ;L1031
 31967| 
 31968| 318: ; preds = %317
 31969|  %319 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity19is_block_move_skill(ptr %75)
 31970|  to label %324 unwind label %23                                                                                        ;L1031
 31971| 
 31972| 320: ; preds = %349, %347, %317
 31973|  %321 = phi i64 [ %285, %317 ], [ %285, %347 ], [ %360, %349 ]                                                         ;L182<197<202<0
 31974|  %322 = phi i64 [ %286, %317 ], [ %286, %347 ], [ %376, %349 ]                                                         ;L0
 31975|     ;; stable_rng = i64 %321
 31976|     ;; self = i64 %322
 31977|     ;; enemy_dps = i64 %322
 31978|     ;; enemy_dps = i64 %322
 31979|  %323 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity14is_block_skill(ptr %75)
 31980|  to label %377 unwind label %23                                                                                        ;L1036
 31981| 
 31982| 324: ; preds = %318
 31983|  br i1 %319, label %329, label %325                                                                                    ;L1031
 31984| 
 31985| 325: ; preds = %347, %329, %324
 31986|  %326 = gep %78, i64 2352                                                                                              ;L1032
 31987|  %327 = load i64, ptr %326, , !!8                                                                                      ;L1032
 31988|  %328 = icmp ult i64 %327, 2                                                                                           ;L1032
 31989|  br i1 %328, label %349, label %348                                                                                    ;L1032
 31990| 
 31991| 329: ; preds = %324
 31992|     ;; self = ptr %75
 31993|  %330 = gep %75, i64 1272                                                                                              ;L742<1031
 31994|  %331 = load i32, ptr %330, , !!8                                                                                      ;L742<1031
 31995|  %332 = icmp eq i32 %331, -1                                                                                           ;L742<1031
 31998|     ;; default = i1 false
 32000|  br i1 %332, label %325, label %333                                                                                    ;L1226<1031
 32001| 
 32002| 333: ; preds = %329
 32003|  %334 = gep %75, i64 1224                                                                                              ;L742<1031
 32005|  %335 = load ptr, ptr %334, , !!8, !!8                                                                                 ;L1227<1031
 32006|  %336 = gep %75, i64 1232                                                                                              ;L1227<1031
 32007|  %337 = load ptr, ptr %336, , !!8, !!8                                                                                 ;L1227<1031
 32011|  %338 = gep %337, i64 16                                                                                               ;L2445<1031<1227<1031
 32012|  %339 = load i64, ptr %338, , !!42916                                                                                  ;L2445<1031<1227<1031
 32013|  %340 = add nsw i64 %339, -1                                                                                           ;L2445<1031<1227<1031
 32014|  %341 = and i64 %340, -16                                                                                              ;L2445<1031<1227<1031
 32015|  %342 = gep %335, i64 %341                                                                                             ;L2445<1031<1227<1031
 32016|  %343 = gep %342, i64 16                                                                                               ;L2445<1031<1227<1031
 32017|  %344 = gep %337, i64 288                                                                                              ;L1031<1227<1031
 32018|  %345 = load ptr, ptr %344, , !!42916, !!8                                                                             ;L1031<1227<1031
 32019|  %346 = invoke zeroext i1 %345(ptr %343)
 32020|  to label %347 unwind label %23                                                                                        ;L1031<1227<1031
 32021| 
 32022| 347: ; preds = %333
 32023|  br i1 %346, label %320, label %325                                                                                    ;L1031
 32024| 
 32025| 348: ; preds = %325
 32026|  invoke void @core::panicking18panic_bounds_check(i64 %327, i64 2, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.203) #30
 32027|  to label %26 unwind label %23                                                                                         ;L1032
 32028| 
 32029| 349: ; preds = %325
 32030|     ;; self = ptr %78
 32031|  %350 = gep %78, i64 2496                                                                                              ;L581<1032
 32032|  %351 = load i32, ptr %350, , !!8                                                                                      ;L581<1032
 32033|  %352 = zext nneg i32 %351 to i64                                                                                      ;L581<1032
 32034|     ;; self = ptr %16
 32035|  %353 = load i32, ptr %63, , !!8                                                                                       ;L581<1032
 32036|  %354 = zext nneg i32 %353 to i64                                                                                      ;L581<1032
 32037|  %355 = getelementptr [5 x { [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64] }], ptr %13, i64 %327 ;L1032
 32038|  %356 = gepS %355, i64 %352                                                                                            ;L1032
 32039|  %357 = gep %356, i64 1080                                                                                             ;L1032
 32040|  %358 = getelementptr i64, ptr %357, i64 %354                                                                          ;L1032
 32041|  %359 = load i64, ptr %358, , !!8                                                                                      ;L1032
 32042|     ;; self = ptr undef
 32043|     ;; self = ptr undef
 32044|     ;; self = ptr undef
 32045|     ;; lo = i64 %58
 32046|     ;; hi = i64 %57
 32047|     ;; rhs = i64 1
 32048|     ;; rhs = i64 -7046029254386353131
 32049|     ;; rhs = i64 -4658895280553007687
 32050|     ;; rhs = i64 -7723592293110705685
 32051|     ;; rhs = i64 -7046029254386353131
 32052|     ;; rhs = i64 -4658895280553007687
 32053|     ;; rhs = i64 -7723592293110705685
 32054|     ;; self = !DIArgList(i64 %57, i64 %58)
 32055|     ;; span = !DIArgList(i64 %57, i64 %58)
 32056|     ;; self = i64 %285
 32057|  %360 = add i64 %285, -7046029254386353131                                                                             ;L2584<182<197<202<1033
 32058|     ;; stable_rng = i64 %360
 32059|     ;; self = !DIArgList(i64 %360, i64 %360)
 32060|     ;; self = !DIArgList(i64 %360, i64 %360, i64 %360, i64 %360)
 32061|     ;; z = !DIArgList(i64 %360, i64 %360, i64 %360, i64 %360)
 32062|  %361 = lshr i64 %360, 30                                                                                              ;L184<197<202<1033
 32063|     ;; self = !DIArgList(i64 %360, i64 %360, i64 %361, i64 %361)
 32064|     ;; z = !DIArgList(i64 %360, i64 %360, i64 %361, i64 %361)
 32065|     ;; self = !DIArgList(i64 %360, i64 %361)
 32066|  %362 = xor i64 %361, %360                                                                                             ;L184<197<202<1033
 32067|     ;; z = !DIArgList(i64 %362, i64 %362)
 32068|     ;; self = !DIArgList(i64 %362, i64 %362)
 32069|     ;; self = i64 %362
 32070|  %363 = mul i64 %362, -4658895280553007687                                                                             ;L2660<184<197<202<1033
 32071|     ;; self = !DIArgList(i64 %363, i64 %363)
 32072|     ;; z = !DIArgList(i64 %363, i64 %363)
 32073|  %364 = lshr i64 %363, 27                                                                                              ;L185<197<202<1033
 32074|     ;; z = !DIArgList(i64 %363, i64 %364)
 32075|     ;; self = !DIArgList(i64 %363, i64 %364)
 32076|  %365 = xor i64 %364, %363                                                                                             ;L185<197<202<1033
 32077|     ;; self = i64 %365
 32078|     ;; z = i64 %365
 32079|  %366 = mul i64 %365, -7723592293110705685                                                                             ;L2660<185<197<202<1033
 32080|     ;; z = i64 %366
 32081|  %367 = lshr i64 %366, 31                                                                                              ;L186<197<202<1033
 32082|  %368 = xor i64 %367, %366                                                                                             ;L186<197<202<1033
 32083|  %369 = zext i64 %368 to i128                                                                                          ;L197<202<1033
 32085|     ;; span = i64 %65
 32086|  %370 = mul nuw i128 %369, %66                                                                                         ;L197<202<1033
 32087|  %371 = lshr i128 %370, 64                                                                                             ;L197<202<1033
 32088|  %372 = trunc nuw i128 %371 to i64                                                                                     ;L197<202<1033
 32089|  %373 = add i64 %58, %372                                                                                              ;L197<202<1033
 32090|  %374 = mul i64 %359, %373                                                                                             ;L1032
 32091|  %375 = udiv i64 %374, 1000                                                                                            ;L1032
 32092|  %376 = add i64 %375, %286                                                                                             ;L1032
 32093|     ;; enemy_dps = i64 %376
 32094|     ;; enemy_dps = i64 %376
 32095|     ;; self = i64 %376
 32096|  br label %320                                                                                                         ;L1031
 32097| 
 32098| 377: ; preds = %320
 32099|  br i1 %323, label %380, label %378                                                                                    ;L1036
 32100| 
 32101| 378: ; preds = %377
 32102|  %379 = invoke zeroext i1 @gc::simulation6entityNtB5_6Entity19is_block_move_skill(ptr %75)
 32103|  to label %384 unwind label %23                                                                                        ;L1036
 32104| 
 32105| 380: ; preds = %413, %411, %377
 32106|  %381 = phi i64 [ %321, %377 ], [ %321, %411 ], [ %424, %413 ]                                                         ;L182<197<202<0
 32107|  %382 = phi i64 [ %322, %377 ], [ %322, %411 ], [ %440, %413 ]                                                         ;L0
 32108|     ;; stable_rng = i64 %381
 32109|     ;; self = i64 %382
 32110|     ;; enemy_dps = i64 %382
 32111|     ;; enemy_dps = i64 %382
 32112|  %383 = add i64 %248, %70                                                                                              ;L1040
 32113|     ;; enemy_nuke = i64 %383
 32114|     ;; enemy_nuke = i64 %383
 32115|     ;; rhs = i64 %383
 32116|  br label %67                                                                                                          ;L1000
 32117| 
 32118| 384: ; preds = %378
 32119|  br i1 %379, label %389, label %385                                                                                    ;L1036
 32120| 
 32121| 385: ; preds = %411, %389, %384
 32122|  %386 = gep %78, i64 2352                                                                                              ;L1037
 32123|  %387 = load i64, ptr %386, , !!8                                                                                      ;L1037
 32124|  %388 = icmp ult i64 %387, 2                                                                                           ;L1037
 32125|  br i1 %388, label %413, label %412                                                                                    ;L1037
 32126| 
 32127| 389: ; preds = %384
 32128|  %390 = gep %75, i64 1480                                                                                              ;L1693<1036
 32129|  %391 = load i64, ptr %390, , !!8                                                                                      ;L1693<1036
 32130|  %392 = icmp ugt i64 %391, 2                                                                                           ;L1693<1036
 32131|  %393 = gep %75, i64 1280                                                                                              ;L1693<1036
 32132|  %394 = select i1 %392, ptr %393, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.58                                        ;L1693<1036
 32133|     ;; self = ptr %394
 32134|  %395 = gep %394, i64 48                                                                                               ;L742<1036
 32135|  %396 = load i32, ptr %395, , !!8                                                                                      ;L742<1036
 32136|  %397 = icmp eq i32 %396, -1                                                                                           ;L742<1036
 32139|     ;; default = i1 false
 32141|  br i1 %397, label %385, label %398                                                                                    ;L1226<1036
 32142| 
 32143| 398: ; preds = %389
 32145|  %399 = load ptr, ptr %394, , !!8, !!8                                                                                 ;L1227<1036
 32146|  %400 = gep %394, i64 8                                                                                                ;L1227<1036
 32147|  %401 = load ptr, ptr %400, , !!8, !!8                                                                                 ;L1227<1036
 32151|  %402 = gep %401, i64 16                                                                                               ;L2445<1036<1227<1036
 32152|  %403 = load i64, ptr %402, , !!42983                                                                                  ;L2445<1036<1227<1036
 32153|  %404 = add nsw i64 %403, -1                                                                                           ;L2445<1036<1227<1036
 32154|  %405 = and i64 %404, -16                                                                                              ;L2445<1036<1227<1036
 32155|  %406 = gep %399, i64 %405                                                                                             ;L2445<1036<1227<1036
 32156|  %407 = gep %406, i64 16                                                                                               ;L2445<1036<1227<1036
 32157|  %408 = gep %401, i64 288                                                                                              ;L1036<1227<1036
 32158|  %409 = load ptr, ptr %408, , !!42983, !!8                                                                             ;L1036<1227<1036
 32159|  %410 = invoke zeroext i1 %409(ptr %407)
 32160|  to label %411 unwind label %23                                                                                        ;L1036<1227<1036
 32161| 
 32162| 411: ; preds = %398
 32163|  br i1 %410, label %380, label %385                                                                                    ;L1036
 32164| 
 32165| 412: ; preds = %385
 32166|  invoke void @core::panicking18panic_bounds_check(i64 %387, i64 2, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.204) #30
 32167|  to label %26 unwind label %23                                                                                         ;L1037
 32168| 
 32169| 413: ; preds = %385
 32170|     ;; self = ptr %78
 32171|  %414 = gep %78, i64 2496                                                                                              ;L581<1037
 32172|  %415 = load i32, ptr %414, , !!8                                                                                      ;L581<1037
 32173|  %416 = zext nneg i32 %415 to i64                                                                                      ;L581<1037
 32174|     ;; self = ptr %16
 32175|  %417 = load i32, ptr %63, , !!8                                                                                       ;L581<1037
 32176|  %418 = zext nneg i32 %417 to i64                                                                                      ;L581<1037
 32177|  %419 = getelementptr [5 x { [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64], [5 x i64] }], ptr %13, i64 %387 ;L1037
 32178|  %420 = gepS %419, i64 %416                                                                                            ;L1037
 32179|  %421 = gep %420, i64 1120                                                                                             ;L1037
 32180|  %422 = getelementptr i64, ptr %421, i64 %418                                                                          ;L1037
 32181|  %423 = load i64, ptr %422, , !!8                                                                                      ;L1037
 32182|     ;; self = ptr undef
 32183|     ;; self = ptr undef
 32184|     ;; self = ptr undef
 32185|     ;; lo = i64 %58
 32186|     ;; hi = i64 %57
 32187|     ;; rhs = i64 1
 32188|     ;; rhs = i64 -7046029254386353131
 32189|     ;; rhs = i64 -4658895280553007687
 32190|     ;; rhs = i64 -7723592293110705685
 32191|     ;; rhs = i64 -7046029254386353131
 32192|     ;; rhs = i64 -4658895280553007687
 32193|     ;; rhs = i64 -7723592293110705685
 32194|     ;; self = !DIArgList(i64 %57, i64 %58)
 32195|     ;; span = !DIArgList(i64 %57, i64 %58)
 32196|     ;; self = i64 %321
 32197|  %424 = add i64 %321, -7046029254386353131                                                                             ;L2584<182<197<202<1038
 32198|     ;; stable_rng = i64 %424
 32199|     ;; self = !DIArgList(i64 %424, i64 %424)
 32200|     ;; self = !DIArgList(i64 %424, i64 %424, i64 %424, i64 %424)
 32201|     ;; z = !DIArgList(i64 %424, i64 %424, i64 %424, i64 %424)
 32202|  %425 = lshr i64 %424, 30                                                                                              ;L184<197<202<1038
 32203|     ;; self = !DIArgList(i64 %424, i64 %424, i64 %425, i64 %425)
 32204|     ;; z = !DIArgList(i64 %424, i64 %424, i64 %425, i64 %425)
 32205|     ;; self = !DIArgList(i64 %424, i64 %425)
 32206|  %426 = xor i64 %425, %424                                                                                             ;L184<197<202<1038
 32207|     ;; z = !DIArgList(i64 %426, i64 %426)
 32208|     ;; self = !DIArgList(i64 %426, i64 %426)
 32209|     ;; self = i64 %426
 32210|  %427 = mul i64 %426, -4658895280553007687                                                                             ;L2660<184<197<202<1038
 32211|     ;; self = !DIArgList(i64 %427, i64 %427)
 32212|     ;; z = !DIArgList(i64 %427, i64 %427)
 32213|  %428 = lshr i64 %427, 27                                                                                              ;L185<197<202<1038
 32214|     ;; z = !DIArgList(i64 %427, i64 %428)
 32215|     ;; self = !DIArgList(i64 %427, i64 %428)
 32216|  %429 = xor i64 %428, %427                                                                                             ;L185<197<202<1038
 32217|     ;; self = i64 %429
 32218|     ;; z = i64 %429
 32219|  %430 = mul i64 %429, -7723592293110705685                                                                             ;L2660<185<197<202<1038
 32220|     ;; z = i64 %430
 32221|  %431 = lshr i64 %430, 31                                                                                              ;L186<197<202<1038
 32222|  %432 = xor i64 %431, %430                                                                                             ;L186<197<202<1038
 32223|  %433 = zext i64 %432 to i128                                                                                          ;L197<202<1038
 32225|     ;; span = i64 %65
 32226|  %434 = mul nuw i128 %433, %66                                                                                         ;L197<202<1038
 32227|  %435 = lshr i128 %434, 64                                                                                             ;L197<202<1038
 32228|  %436 = trunc nuw i128 %435 to i64                                                                                     ;L197<202<1038
 32229|  %437 = add i64 %58, %436                                                                                              ;L197<202<1038
 32230|  %438 = mul i64 %423, %437                                                                                             ;L1037
 32231|  %439 = udiv i64 %438, 1000                                                                                            ;L1037
 32232|  %440 = add i64 %439, %322                                                                                             ;L1037
 32233|     ;; enemy_dps = i64 %440
 32234|     ;; enemy_dps = i64 %440
 32235|     ;; self = i64 %440
 32236|  br label %380                                                                                                         ;L1036
 32237| 
 32238| 441: ; preds = %484, %80
 32239|  %442 = phi i64 [ %68, %80 ], [ %485, %484 ]                                                                           ;L0
 32240|  %443 = phi ptr [ %81, %80 ], [ %448, %484 ]                                                                           ;L1049
 32241|  %444 = phi i64 [ %70, %80 ], [ %503, %484 ]                                                                           ;L0
 32242|  %445 = phi i64 [ %71, %80 ], [ %500, %484 ]                                                                           ;L0
 32243|     ;; stable_rng = i64 %442
 32244|     ;; self = i64 %445
 32245|     ;; enemy_dps = i64 %445
 32246|     ;; enemy_dps = i64 %445
 32247|     ;; rhs = i64 %444
 32248|     ;; enemy_nuke = i64 %444
 32249|     ;; enemy_nuke = i64 %444
 32250|     ;; iter[0..+8] = ptr %443
 32251|     ;; self = ptr undef
 32252|     ;; ptr = ptr %443
 32253|     ;; self = ptr %443
 32254|     ;; end_or_len = ptr %84
 32257|  %446 = icmp eq ptr %443, %84                                                                                          ;L1714<180<1049
 32258|  br i1 %446, label %453, label %447                                                                                    ;L180<1049
 32259| 
 32260| 447: ; preds = %441
 32261|  %448 = gep %443, i64 8                                                                                                ;L656<185<1049
 32262|     ;; iter[0..+8] = ptr %448
 32263|  %449 = load ptr, ptr %443, , !!8, !!8                                                                                 ;L1049
 32264|     ;; tower = ptr %449
 32265|     ;; self = ptr %449
 32266|  %450 = gep %449, i64 1216                                                                                             ;L742<1050
 32267|  %451 = load i32, ptr %450, , !!8                                                                                      ;L742<1050
 32268|  %452 = icmp eq i32 %451, -1                                                                                           ;L742<1050
 32269|  br i1 %452, label %461, label %458                                                                                    ;L742<1050
 32270| 
 32271| 453: ; preds = %441
 32272|  %454 = gep %16, i64 2352                                                                                              ;L1058
 32273|  %455 = load i64, ptr %454, , !!8                                                                                      ;L1058
 32274|     ;; team = i64 %455
 32275|  %456 = sub i64 1, %455                                                                                                ;L1058
 32276|  %457 = icmp ult i64 %456, 2                                                                                           ;L1058
 32277|  br i1 %457, label %504, label %518                                                                                    ;L1058
 32278| 
 32279| 458: ; preds = %447
 32280|  %459 = gep %449, i64 1168                                                                                             ;L742<1050
 32281|     ;; self = ptr %459
 32282|  %460 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %459, ptr %29, ptr %449, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.56, ptr %3)
 32283|  to label %462 unwind label %23                                                                                        ;L1050
 32284| 
 32285| 461: ; preds = %447
 32286|     ;; self = ptr null
 32287|  invoke void @core::option13unwrap_failed(ptr @anon.e5f9a016f1220e5cee86475419aee1e5.205) #30
 32288|  to label %26 unwind label %23                                                                                         ;L1013<1050
 32289| 
 32290| 462: ; preds = %458
 32291|     ;; damage = i64 %460
 32292|     ;; self = ptr undef
 32293|     ;; self = ptr undef
 32294|     ;; self = ptr undef
 32295|     ;; lo = i64 %58
 32296|     ;; hi = i64 %57
 32297|     ;; rhs = i64 1
 32298|     ;; rhs = i64 -7046029254386353131
 32299|     ;; rhs = i64 -4658895280553007687
 32300|     ;; rhs = i64 -7723592293110705685
 32301|     ;; rhs = i64 -7046029254386353131
 32302|     ;; rhs = i64 -4658895280553007687
 32303|     ;; rhs = i64 -7723592293110705685
 32304|     ;; self = !DIArgList(i64 %57, i64 %58)
 32305|     ;; span = !DIArgList(i64 %57, i64 %58)
 32306|     ;; self = i64 %442
 32307|  %463 = add i64 %442, -7046029254386353131                                                                             ;L2584<182<197<202<1052
 32308|     ;; stable_rng = i64 %463
 32309|     ;; self = !DIArgList(i64 %463, i64 %463)
 32310|     ;; self = !DIArgList(i64 %463, i64 %463, i64 %463, i64 %463)
 32311|     ;; z = !DIArgList(i64 %463, i64 %463, i64 %463, i64 %463)
 32312|  %464 = lshr i64 %463, 30                                                                                              ;L184<197<202<1052
 32313|     ;; self = !DIArgList(i64 %463, i64 %463, i64 %464, i64 %464)
 32314|     ;; z = !DIArgList(i64 %463, i64 %463, i64 %464, i64 %464)
 32315|     ;; self = !DIArgList(i64 %463, i64 %464)
 32316|  %465 = xor i64 %464, %463                                                                                             ;L184<197<202<1052
 32317|     ;; z = !DIArgList(i64 %465, i64 %465)
 32318|     ;; self = !DIArgList(i64 %465, i64 %465)
 32319|     ;; self = i64 %465
 32320|  %466 = mul i64 %465, -4658895280553007687                                                                             ;L2660<184<197<202<1052
 32321|     ;; self = !DIArgList(i64 %466, i64 %466)
 32322|     ;; z = !DIArgList(i64 %466, i64 %466)
 32323|  %467 = lshr i64 %466, 27                                                                                              ;L185<197<202<1052
 32324|     ;; z = !DIArgList(i64 %466, i64 %467)
 32325|     ;; self = !DIArgList(i64 %466, i64 %467)
 32326|  %468 = xor i64 %467, %466                                                                                             ;L185<197<202<1052
 32327|     ;; self = i64 %468
 32328|     ;; z = i64 %468
 32329|  %469 = mul i64 %468, -7723592293110705685                                                                             ;L2660<185<197<202<1052
 32330|     ;; z = i64 %469
 32331|  %470 = lshr i64 %469, 31                                                                                              ;L186<197<202<1052
 32332|  %471 = xor i64 %470, %469                                                                                             ;L186<197<202<1052
 32333|  %472 = zext i64 %471 to i128                                                                                          ;L197<202<1052
 32335|     ;; span = i64 %65
 32336|  %473 = mul nuw i128 %472, %66                                                                                         ;L197<202<1052
 32337|  %474 = lshr i128 %473, 64                                                                                             ;L197<202<1052
 32338|  %475 = trunc nuw i128 %474 to i64                                                                                     ;L197<202<1052
 32339|  %476 = add i64 %58, %475                                                                                              ;L197<202<1052
 32340|  %477 = mul i64 %476, %33                                                                                              ;L1051
 32341|  %478 = mul i64 %477, %460                                                                                             ;L1051
 32342|  %479 = udiv i64 %478, 1000                                                                                            ;L1051
 32343|  %480 = invoke i64 @gc::simulation6entityNtB5_6Entity15attack_cooltime(ptr %449)
 32344|  to label %481 unwind label %23                                                                                        ;L1053
 32345| 
 32346| 481: ; preds = %462
 32347|  %482 = icmp eq i64 %480, 0                                                                                            ;L1051
 32348|  br i1 %482, label %483, label %484                                                                                    ;L1051
 32349| 
 32350| 483: ; preds = %481
 32351|  invoke void @core::panicking11panic_const23panic_const_div_by_zero(ptr @anon.e5f9a016f1220e5cee86475419aee1e5.206) #30
 32352|  to label %26 unwind label %23                                                                                         ;L1051
 32353| 
 32354| 484: ; preds = %481
 32358|     ;; self = ptr undef
 32359|     ;; self = ptr undef
 32360|     ;; self = ptr undef
 32361|     ;; lo = i64 %58
 32362|     ;; hi = i64 %57
 32363|     ;; rhs = i64 1
 32364|     ;; rhs = i64 -7046029254386353131
 32365|     ;; rhs = i64 -4658895280553007687
 32366|     ;; rhs = i64 -7723592293110705685
 32367|     ;; rhs = i64 -7046029254386353131
 32368|     ;; rhs = i64 -4658895280553007687
 32369|     ;; rhs = i64 -7723592293110705685
 32370|     ;; self = !DIArgList(i64 %57, i64 %58)
 32371|     ;; span = !DIArgList(i64 %57, i64 %58)
 32372|     ;; self = i64 %463
 32373|  %485 = add i64 %442, 4354685564936845354                                                                              ;L2584<182<197<202<1054
 32374|     ;; stable_rng = i64 %485
 32375|     ;; self = !DIArgList(i64 %485, i64 %485)
 32376|     ;; self = !DIArgList(i64 %485, i64 %485, i64 %485, i64 %485)
 32377|     ;; z = !DIArgList(i64 %485, i64 %485, i64 %485, i64 %485)
 32378|  %486 = lshr i64 %485, 30                                                                                              ;L184<197<202<1054
 32379|     ;; self = !DIArgList(i64 %485, i64 %485, i64 %486, i64 %486)
 32380|     ;; z = !DIArgList(i64 %485, i64 %485, i64 %486, i64 %486)
 32381|     ;; self = !DIArgList(i64 %485, i64 %486)
 32382|  %487 = xor i64 %486, %485                                                                                             ;L184<197<202<1054
 32383|     ;; z = !DIArgList(i64 %487, i64 %487)
 32384|     ;; self = !DIArgList(i64 %487, i64 %487)
 32385|     ;; self = i64 %487
 32386|  %488 = mul i64 %487, -4658895280553007687                                                                             ;L2660<184<197<202<1054
 32387|     ;; self = !DIArgList(i64 %488, i64 %488)
 32388|     ;; z = !DIArgList(i64 %488, i64 %488)
 32389|  %489 = lshr i64 %488, 27                                                                                              ;L185<197<202<1054
 32390|     ;; z = !DIArgList(i64 %488, i64 %489)
 32391|     ;; self = !DIArgList(i64 %488, i64 %489)
 32392|  %490 = xor i64 %489, %488                                                                                             ;L185<197<202<1054
 32393|     ;; self = i64 %490
 32394|     ;; z = i64 %490
 32395|  %491 = mul i64 %490, -7723592293110705685                                                                             ;L2660<185<197<202<1054
 32396|     ;; z = i64 %491
 32397|  %492 = lshr i64 %491, 31                                                                                              ;L186<197<202<1054
 32398|  %493 = xor i64 %492, %491                                                                                             ;L186<197<202<1054
 32399|  %494 = zext i64 %493 to i128                                                                                          ;L197<202<1054
 32401|     ;; span = i64 %65
 32402|  %495 = mul nuw i128 %494, %66                                                                                         ;L197<202<1054
 32403|  %496 = lshr i128 %495, 64                                                                                             ;L197<202<1054
 32404|  %497 = trunc nuw i128 %496 to i64                                                                                     ;L197<202<1054
 32405|  %498 = add i64 %58, %497                                                                                              ;L197<202<1054
 32406|  %499 = udiv i64 %479, %480                                                                                            ;L1051
 32407|     ;; self = !DIArgList(i64 %445, i64 %499)
 32408|     ;; enemy_dps = !DIArgList(i64 %445, i64 %499)
 32409|     ;; enemy_dps = !DIArgList(i64 %445, i64 %499)
 32410|  %500 = add i64 %499, %445                                                                                             ;L1051
 32411|     ;; enemy_dps = i64 %500
 32412|     ;; enemy_dps = i64 %500
 32413|     ;; self = i64 %500
 32414|  %501 = mul i64 %460, %498                                                                                             ;L1054
 32415|  %502 = udiv i64 %501, 1000                                                                                            ;L1054
 32416|  %503 = add i64 %502, %444                                                                                             ;L1054
 32417|     ;; enemy_nuke = i64 %503
 32418|     ;; enemy_nuke = i64 %503
 32419|     ;; rhs = i64 %503
 32420|  br label %441                                                                                                         ;L1049
 32421| 
 32422| 504: ; preds = %453
 32423|  %505 = gep %13, i64 240                                                                                               ;L1058
 32424|  %506 = getelementptr { { ptr, ptr, i64 }, i64 }, ptr %505, i64 %456                                                   ;L1058
 32425|     ;; self = ptr %506
 32426|     ;; self = ptr %506
 32427|  %507 = load ptr, ptr %506, , !!8, !!8                                                                                 ;L138<2073<1058
 32428|     ;; p = ptr %507
 32429|  %508 = gep %506, i64 24                                                                                               ;L2075<1058
 32430|  %509 = load i64, ptr %508, , !!8                                                                                      ;L2075<1058
 32431|     ;; len = i64 %509
 32432|     ;; count = i64 %509
 32433|     ;; self[0..+8] = ptr %507
 32434|     ;; slice[0..+8] = ptr %507
 32435|     ;; self[8..+8] = i64 %509
 32436|     ;; slice[8..+8] = i64 %509
 32437|     ;; ptr = ptr %507
 32438|     ;; self = ptr %507
 32439|  %510 = getelementptr ptr, ptr %507, i64 %509                                                                          ;L961<100<1042<1058
 32440|     ;; iter[0..+8] = ptr %507
 32441|     ;; iter[8..+8] = ptr %510
 32442|  %511 = gep %3, i64 1632
 32443|  %512 = load i64, ptr %511,
 32444|  %513 = gep %3, i64 1640
 32445|  %514 = load i64, ptr %513,
 32446|  %515 = gep %2, i64 2352
 32447|  %516 = load i64, ptr %515,
 32448|  %517 = gep %36, i64 248
 32449|  br label %562                                                                                                         ;L1058
 32450| 
 32451| 518: ; preds = %453
 32452|  invoke void @core::panicking18panic_bounds_check(i64 %456, i64 2, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.207) #30
 32453|  to label %26 unwind label %23                                                                                         ;L1058
 32454| 
 32455| 519: ; preds = %562, %555
 32456|  %520 = phi ptr [ %523, %555 ], [ %563, %562 ]                                                                         ;L1058
 32457|     ;; self = i64 %565
 32458|     ;; enemy_dps = i64 %565
 32459|     ;; enemy_dps = i64 %565
 32460|     ;; rhs = i64 %564
 32461|     ;; enemy_nuke = i64 %564
 32462|     ;; enemy_nuke = i64 %564
 32463|     ;; iter[0..+8] = ptr %520
 32464|     ;; self = ptr undef
 32465|     ;; ptr = ptr %520
 32466|     ;; self = ptr %520
 32467|     ;; end_or_len = ptr %510
 32470|  %521 = icmp eq ptr %520, %510                                                                                         ;L1714<180<1058
 32471|  br i1 %521, label %541, label %522                                                                                    ;L180<1058
 32472| 
 32473| 522: ; preds = %519
 32474|  %523 = gep %520, i64 8                                                                                                ;L656<185<1058
 32475|     ;; iter[0..+8] = ptr %523
 32476|  %524 = load ptr, ptr %520, , !!8, !!8                                                                                 ;L1058
 32477|     ;; e = ptr %524
 32478|     ;; self = ptr %524
 32479|  %525 = gep %524, i64 1632                                                                                             ;L2158<1059
 32480|  %526 = load i64, ptr %525, , !!8                                                                                      ;L2158<1059
 32481|     ;; x1 = i64 %526
 32482|     ;; self = i64 %526
 32483|  %527 = gep %524, i64 1640                                                                                             ;L2158<1059
 32484|  %528 = load i64, ptr %527, , !!8                                                                                      ;L2158<1059
 32485|     ;; y1 = i64 %528
 32486|     ;; self = i64 %528
 32487|     ;; x2 = i64 %512
 32488|     ;; other = i64 %512
 32489|     ;; y2 = i64 %514
 32490|     ;; other = i64 %514
 32491|  %529 = icmp ult i64 %526, %512                                                                                        ;L3147<7<2158<1059
 32492|  %530 = sub nuw i64 %512, %526                                                                                         ;L3147<7<2158<1059
 32493|  %531 = sub nuw i64 %526, %512                                                                                         ;L3147<7<2158<1059
 32494|  %532 = select i1 %529, i64 %530, i64 %531                                                                             ;L3147<7<2158<1059
 32495|     ;; dx = i64 %532
 32496|  %533 = icmp ult i64 %528, %514                                                                                        ;L3147<8<2158<1059
 32497|  %534 = sub nuw i64 %514, %528                                                                                         ;L3147<8<2158<1059
 32498|  %535 = sub nuw i64 %528, %514                                                                                         ;L3147<8<2158<1059
 32499|  %536 = select i1 %533, i64 %534, i64 %535                                                                             ;L3147<8<2158<1059
 32500|     ;; dy = i64 %536
 32501|  %537 = mul i64 %532, %532                                                                                             ;L9<2158<1059
 32502|  %538 = mul i64 %536, %536                                                                                             ;L9<2158<1059
 32503|  %539 = add i64 %538, %537                                                                                             ;L9<2158<1059
 32504|  %540 = icmp ugt i64 %539, 22500000000                                                                                 ;L1059
 32505|  br i1 %540, label %555, label %545                                                                                    ;L1059
 32506| 
 32507| 541: ; preds = %519
 32508|     ;; self = ptr %4
 32509|     ;; self = ptr %4
 32510|     ;; self = ptr %4
 32511|  %542 = load ptr, ptr %4, , !!8, !!8                                                                                   ;L138<2073<2136<1075
 32512|     ;; p = ptr %542
 32513|  %543 = load i64, ptr %60, , !!8                                                                                       ;L2075<2136<1075
 32514|     ;; len = i64 %543
 32515|     ;; count = i64 %543
 32516|     ;; self[0..+8] = ptr %542
 32517|     ;; slice[0..+8] = ptr %542
 32518|     ;; self[8..+8] = i64 %543
 32519|     ;; slice[8..+8] = i64 %543
 32520|     ;; ptr = ptr %542
 32521|     ;; self = ptr %542
 32522|  %544 = getelementptr ptr, ptr %542, i64 %543                                                                          ;L961<100<1042<2136<1075
 32523|     ;; iter[0..+8] = ptr %542
 32524|     ;; iter[8..+8] = ptr %544
 32525|  br label %574                                                                                                         ;L1075
 32526| 
 32527| 545: ; preds = %522
 32528|  %546 = gep %524, i64 1472                                                                                             ;L1063
 32529|  %547 = load i64, ptr %546, , !!8                                                                                      ;L1063
 32530|  %548 = load ptr, ptr %517, , !!8                                                                                      ;L1063
 32531|  %549 = invoke zeroext i1 %548(ptr %34, i64 %516, i64 %547)
 32532|  to label %550 unwind label %23                                                                                        ;L1063
 32533| 
 32534| 550: ; preds = %545
 32535|  br i1 %549, label %551, label %555                                                                                    ;L1063
 32536| 
 32537| 551: ; preds = %550
 32538|     ;; self = ptr %524
 32539|  %552 = gep %524, i64 1216                                                                                             ;L742<1066
 32540|  %553 = load i32, ptr %552, , !!8                                                                                      ;L742<1066
 32541|  %554 = icmp eq i32 %553, -1                                                                                           ;L742<1066
 32542|  br i1 %554, label %559, label %556                                                                                    ;L742<1066
 32543| 
 32544| 555: ; preds = %550, %522
 32545|  br label %519                                                                                                         ;L1
 32546| 
 32547| 556: ; preds = %551
 32548|  %557 = gep %524, i64 1168                                                                                             ;L742<1066
 32549|     ;; atk = ptr %557
 32550|  %558 = invoke i64 @gc::simulation6effectNtB2_6Effect22expected_damage_target(ptr %557, ptr %29, ptr %524, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.56, ptr %3)
 32551|  to label %566 unwind label %23                                                                                        ;L1067
 32552| 
 32553| 559: ; preds = %568, %551
 32554|  %560 = phi i64 [ %573, %568 ], [ %564, %551 ]                                                                         ;L0
 32555|  %561 = phi i64 [ %572, %568 ], [ %565, %551 ]                                                                         ;L0
 32556|     ;; self = i64 %561
 32557|     ;; enemy_dps = i64 %561
 32558|     ;; enemy_dps = i64 %561
 32559|     ;; rhs = i64 %560
 32560|     ;; enemy_nuke = i64 %560
 32561|     ;; enemy_nuke = i64 %560
 32562|  br label %562                                                                                                         ;L1058
 32563| 
 32564| 562: ; preds = %559, %504
 32565|  %563 = phi ptr [ %523, %559 ], [ %507, %504 ]
 32566|  %564 = phi i64 [ %560, %559 ], [ %444, %504 ]
 32567|  %565 = phi i64 [ %561, %559 ], [ %445, %504 ]
 32568|  br label %519                                                                                                         ;L180<1058
 32569| 
 32570| 566: ; preds = %556
 32571|     ;; damage = i64 %558
 32572|  %567 = invoke i64 @gc::simulation6entityNtB5_6Entity15attack_cooltime(ptr %524)
 32573|  to label %568 unwind label %23                                                                                        ;L1068
 32574| 
 32575| 568: ; preds = %566
 32576|  %569 = mul i64 %558, %33                                                                                              ;L1068
 32577|     ;; self = i64 %567
 32578|     ;; other = i64 1
 32579|  %570 = tail call i64 @llvm.umax.i64(i64 %567, i64 1)                                                                  ;L1039<1068
 32580|  %571 = udiv i64 %569, %570                                                                                            ;L1068
 32581|  %572 = add i64 %571, %565                                                                                             ;L1068
 32582|     ;; enemy_dps = i64 %572
 32583|     ;; enemy_dps = i64 %572
 32584|     ;; self = i64 %572
 32585|  %573 = add i64 %558, %564                                                                                             ;L1069
 32586|     ;; enemy_nuke = i64 %573
 32587|     ;; enemy_nuke = i64 %573
 32588|     ;; rhs = i64 %573
 32589|  br label %559                                                                                                         ;L1066
 32590| 
 32591| 574: ; preds = %588, %541
 32592|  %575 = phi ptr [ %542, %541 ], [ %579, %588 ]                                                                         ;L1075
 32593|  %576 = phi i64 [ %565, %541 ], [ %590, %588 ]                                                                         ;L998
 32594|     ;; self = i64 %576
 32595|     ;; enemy_dps = i64 %576
 32596|     ;; enemy_dps = i64 %576
 32597|     ;; iter[0..+8] = ptr %575
 32598|     ;; self = ptr undef
 32599|     ;; ptr = ptr %575
 32600|     ;; self = ptr %575
 32601|     ;; end_or_len = ptr %544
 32604|  %577 = icmp eq ptr %575, %544                                                                                         ;L1714<180<1075
 32605|  br i1 %577, label %586, label %578                                                                                    ;L180<1075
 32606| 
 32607| 578: ; preds = %574
 32608|  %579 = gep %575, i64 8                                                                                                ;L656<185<1075
 32609|     ;; iter[0..+8] = ptr %579
 32610|  %580 = load ptr, ptr %575, , !!8, !!8                                                                                 ;L1075
 32611|     ;; pchamp = ptr %580
 32612|     ;; self = ptr %580
 32613|     ;; self = ptr %580
 32614|     ;; self = ptr %580
 32615|  %581 = gep %580, i64 760                                                                                              ;L614<609<296<1968<1864<3787<1076
 32616|  %582 = load ptr, ptr %581, , !!8, !!8                                                                                 ;L614<609<296<1968<1864<3787<1076
 32617|  %583 = gep %580, i64 768                                                                                              ;L1864<3787<1076
 32618|  %584 = load i64, ptr %583, , !!8                                                                                      ;L1864<3787<1076
 32619|     ;; len = i64 %584
 32620|     ;; count = i64 %584
 32621|     ;; self[0..+8] = ptr %582
 32622|     ;; slice[0..+8] = ptr %582
 32623|     ;; self[8..+8] = i64 %584
 32624|     ;; slice[8..+8] = i64 %584
 32625|     ;; ptr = ptr %582
 32626|     ;; self = ptr %582
 32627|  %585 = getelementptr { { { { { ptr, ptr } } }, {} }, {} }, ptr %582, i64 %584                                         ;L961<100<1042<1076
 32628|     ;; iter[0..+8] = ptr %582
 32629|     ;; iter[8..+8] = ptr %585
 32630|  br label %588                                                                                                         ;L1076
 32631| 
 32632| 586: ; preds = %574
 32633|     ;; ally_heal = i64 0
 32634|     ;; rhs = i64 0
 32635|  %587 = icmp ult i64 %455, 2                                                                                           ;L1905<1085
 32636|  br i1 %587, label %602, label %608                                                                                    ;L1905<1085
 32637| 
 32638| 588: ; preds = %599, %578
 32639|  %589 = phi ptr [ %582, %578 ], [ %600, %599 ]                                                                         ;L1076
 32640|  %590 = phi i64 [ %576, %578 ], [ %601, %599 ]                                                                         ;L0
 32641|     ;; self = i64 %590
 32642|     ;; enemy_dps = i64 %590
 32643|     ;; enemy_dps = i64 %590
 32644|     ;; iter[0..+8] = ptr %589
 32645|     ;; self = ptr undef
 32646|     ;; ptr = ptr %589
 32647|     ;; self = ptr %589
 32648|     ;; end_or_len = ptr %585
 32651|  %591 = icmp eq ptr %589, %585                                                                                         ;L1714<180<1076
 32652|  br i1 %591, label %574, label %592                                                                                    ;L180<1076
 32653| 
 32654| 592: ; preds = %588
 32655|     ;; iter[0..+8] = ptr %589
 32656|     ;; b = ptr %589
 32657|  %593 = load ptr, ptr %589, , !!8, !!8                                                                                 ;L1077
 32658|  %594 = gep %589, i64 8                                                                                                ;L1077
 32659|  %595 = load ptr, ptr %594, , !!8, !!8                                                                                 ;L1077
 32660|  %596 = gep %595, i64 128                                                                                              ;L1077
 32661|  %597 = load ptr, ptr %596, , !!8                                                                                      ;L1077
 32662|  %598 = invoke i64 %597(ptr %593, ptr %29, ptr %580, ptr %3)
 32663|  to label %599 unwind label %23                                                                                        ;L1077
 32664| 
 32665| 599: ; preds = %592
 32666|  %600 = gep %589, i64 16                                                                                               ;L656<185<1076
 32667|     ;; iter[0..+8] = ptr %600
 32668|  %601 = add i64 %598, %590                                                                                             ;L1077
 32669|     ;; enemy_dps = i64 %601
 32670|     ;; enemy_dps = i64 %601
 32671|     ;; self = i64 %601
 32672|  br label %588                                                                                                         ;L1076
 32673| 
 32674| 602: ; preds = %586
 32675|  %603 = gep %13, i64 480                                                                                               ;L1905<1085
 32676|  %604 = getelementptr [5 x ptr], ptr %603, i64 %455                                                                    ;L1905<1085
 32677|     ;; self[0..+8] = ptr %604
 32678|     ;; slice[0..+8] = ptr %604
 32679|     ;; self[8..+8] = i64 5
 32680|     ;; slice[8..+8] = i64 5
 32681|     ;; ptr = ptr %604
 32682|     ;; self = ptr %604
 32683|  %605 = gep %604, i64 40                                                                                               ;L961<100<1042<1905<1085
 32685|  %606 = gep %8, i64 8                                                                                                  ;L1085
 32686|  store ptr %605, ptr %606,                                                                                             ;L1085
 32687|  %607 = gep %8, i64 16
 32688|  br label %612                                                                                                         ;L1085
 32689| 
 32690| 608: ; preds = %586
 32691|  invoke void @core::panicking18panic_bounds_check(i64 %455, i64 2, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.85) #30
 32692|  to label %26 unwind label %23                                                                                         ;L1905<1085
 32693| 
 32694| 609: ; preds = %719
 32695|  %610 = load ptr, ptr %606, , !!43327
 32696|  %611 = load ptr, ptr %8, , !!43327
 32697|  br label %612                                                                                                         ;L365<64<1085
 32698| 
 32699| 612: ; preds = %609, %602
 32700|  %613 = phi ptr [ %604, %602 ], [ %611, %609 ]
 32701|  %614 = phi ptr [ %605, %602 ], [ %610, %609 ]
 32702|  %615 = phi i64 [ 0, %602 ], [ %720, %609 ]                                                                            ;L1084
 32703|     ;; rhs = i64 %615
 32704|     ;; ally_heal = i64 %615
 32705|     ;; self = ptr %8
 32708|  store ptr %607, ptr %7, , !!43339
 32709|     ;; self = ptr %8
 32710|     ;; self = ptr %8
 32711|     ;; f = ptr %7
 32712|     ;; count = i64 1
 32713|  br label %616                                                                                                         ;L365<64<1085
 32714| 
 32715| 616: ; preds = %622, %612
 32716|  %617 = phi ptr [ %620, %622 ], [ %613, %612 ]
 32717|     ;; ptr = ptr %617
 32718|     ;; self = ptr %617
 32719|     ;; end_or_len = ptr %614
 32722|  %618 = icmp eq ptr %617, %614                                                                                         ;L1714<180<365<64<1085
 32723|  br i1 %618, label %630, label %619                                                                                    ;L180<365<64<1085
 32724| 
 32725| 619: ; preds = %616
 32726|  %620 = gep %617, i64 8                                                                                                ;L656<185<365<64<1085
 32727|  store ptr %620, ptr %8, , !!43327                                                                                     ;L185<365<64<1085
 32728|     ;; x = ptr %617
 32729|  %621 = invoke ptr @gc::simulationNtBW_21AbstractGameWithCache14iter_champions0INtB7_5FnMutTRINtNtBb_6option6OptionRNtNtBW_6entity6EntityEEE8call_mutCshdEBA0ozCnw_7game_ai(ptr %7, ptr %617)
 32730|  to label %622 unwind label %23                                                                                        ;L366<64<1085
 32731| 
 32732| 622: ; preds = %619
 32733|  %623 = icmp eq ptr %621, null                                                                                         ;L366<64<1085
 32734|  br i1 %623, label %616, label %624                                                                                    ;L366<64<1085
 32735| 
 32736| 624: ; preds = %622
 32738|     ;; ally = ptr %621
 32739|     ;; self = ptr %621
 32740|     ;; self = ptr %621
 32741|     ;; self = ptr %621
 32742|  %625 = gep %621, i64 760                                                                                              ;L614<609<296<1968<1864<3787<1086
 32743|  %626 = load ptr, ptr %625, , !!8, !!8                                                                                 ;L614<609<296<1968<1864<3787<1086
 32744|  %627 = gep %621, i64 768                                                                                              ;L1864<3787<1086
 32745|  %628 = load i64, ptr %627, , !!8                                                                                      ;L1864<3787<1086
 32746|     ;; len = i64 %628
 32747|     ;; count = i64 %628
 32748|     ;; self[0..+8] = ptr %626
 32749|     ;; slice[0..+8] = ptr %626
 32750|     ;; self[8..+8] = i64 %628
 32751|     ;; slice[8..+8] = i64 %628
 32752|     ;; ptr = ptr %626
 32753|     ;; self = ptr %626
 32754|  %629 = getelementptr { { { { { ptr, ptr } } }, {} }, {} }, ptr %626, i64 %628                                         ;L961<100<1042<1086
 32755|     ;; iter[0..+8] = ptr %626
 32756|     ;; iter[8..+8] = ptr %629
 32757|  br label %719                                                                                                         ;L1086
 32758| 
 32759| 630: ; preds = %616
 32762|  %631 = call i64 @llvm.usub.sat.i64(i64 %576, i64 %615)                                                                ;L2472<1090
 32763|     ;; enemy_dps = i64 %631
 32764|     ;; enemy_dps = i64 %631
 32765|     ;; self = i64 %631
 32766|  %632 = gep %36, i64 64                                                                                                ;L1095
 32767|  %633 = load ptr, ptr %632, , !!8                                                                                      ;L1095
 32768|  %634 = invoke { i64, ptr } %633(ptr %34)
 32769|  to label %635 unwind label %23                                                                                        ;L1095
 32770| 
 32771| 635: ; preds = %630
 32772|  %636 = extractvalue { i64, ptr } %634, 0                                                                              ;L1095
 32773|     ;; self[0..+8] = i64 %636
 32775|  %637 = icmp ne i64 %636, 0                                                                                            ;L231<1095
 32776|  %638 = extractvalue { i64, ptr } %634, 1
 32778|     ;; default = i64 0
 32780|  %639 = icmp eq ptr %638, null                                                                                         ;L1226<1095
 32781|  %640 = select i1 %637, i1 true, i1 %639                                                                               ;L1226<1095
 32782|  br i1 %640, label %652, label %641                                                                                    ;L1226<1095
 32783| 
 32784| 641: ; preds = %635
 32785|  %642 = load i64, ptr %454,                                                                                            ;L1095
 32786|     ;; t = ptr %638
 32788|     ;; m = ptr %638
 32789|     ;; self = ptr %638
 32790|  %643 = sub i64 1, %642                                                                                                ;L1095<1227<1095
 32791|     ;; team = i64 %643
 32792|  %644 = icmp ult i64 %643, 2                                                                                           ;L210<1095<1227<1095
 32793|  br i1 %644, label %647, label %645                                                                                    ;L210<1095<1227<1095
 32794| 
 32795| 645: ; preds = %641
 32796|  invoke void @core::panicking18panic_bounds_check(i64 %643, i64 2, ptr @anon.e5f9a016f1220e5cee86475419aee1e5.82) #30
 32797|  to label %646 unwind label %23                                                                                        ;L210<1095<1227<1095
 32798| 
 32799| 646: ; preds = %645
 32800|  unreachable                                                                                                           ;L210<1095<1227<1095
 32801| 
 32802| 647: ; preds = %641
 32803|  %648 = gep %638, i64 576                                                                                              ;L210<1095<1227<1095
 32804|  %649 = getelementptr i64, ptr %648, i64 %643                                                                          ;L210<1095<1227<1095
 32805|  %650 = load i64, ptr %649, , !!8                                                                                      ;L210<1095<1227<1095
 32806|  %651 = icmp eq i64 %650, 0                                                                                            ;L1095
 32807|  br label %652                                                                                                         ;L1230<1095
 32808| 
 32809| 652: ; preds = %647, %635
 32810|  %653 = phi i1 [ %651, %647 ], [ true, %635 ]                                                                          ;L0<1095
 32811|     ;; enemy_epic_buff = i1 %653
 32812|  %654 = gep %3, i64 1648                                                                                               ;L1096
 32813|  %655 = load i64, ptr %654, , !!8                                                                                      ;L1096
 32814|  %656 = gep %3, i64 1576                                                                                               ;L1096
 32815|  %657 = load i64, ptr %656, , !!8                                                                                      ;L1096
 32816|     ;; self = i64 %657
 32817|     ;; other = i64 1
 32818|  %658 = call i64 @llvm.umax.i64(i64 %657, i64 1)                                                                       ;L1039<1096
 32819|  %659 = mul i64 %655, 100                                                                                              ;L1096
 32820|  %660 = mul i64 %658, 75                                                                                               ;L1096
 32821|  %661 = icmp ugt i64 %659, %660                                                                                        ;L1096
 32822|     ;; low_enough_to_care_minions = i1 %661
 32823|  %662 = invoke i64 %38(ptr %34)
 32824|  to label %663 unwind label %23                                                                                        ;L1097
 32825| 
 32826| 663: ; preds = %652
 32827|     ;; tick = i64 %662
 32828|     ;; tick = i64 %662
 32829|     ;; self = ptr %29
 32830|  %664 = gep %29, i64 56                                                                                                ;L263<399<1097
 32831|  %665 = load i8, ptr %664, , !!8                                                                                       ;L263<399<1097
 32832|  switch i8 %665, label %667 [
 32833|  i8 0, label %666
 32834|  i8 7, label %666
 32835|  i8 8, label %666
 32836|  i8 5, label %666
 32837|  ]                                                                                                                     ;L263<399<1097
 32838| 
 32839| 666: ; preds = %663, %663, %663, %663
 32841|     ;; rhs = i64 %33
 32843|  br i1 %653, label %668, label %675                                                                                    ;L1098
 32844| 
 32845| 667: ; preds = %663
 32846|     ;; line_phase = i8 1
 32847|  br i1 %653, label %677, label %675                                                                                    ;L1098
 32848| 
 32849| 668: ; preds = %666
 32850|  %669 = gep %31, i64 2216                                                                                              ;L703<399<1097
 32851|  %670 = load i64, ptr %669, , !!8                                                                                      ;L703<399<1097
 32852|     ;; self = i64 %670
 32853|  %671 = mul i64 %33, 30                                                                                                ;L704<399<1097
 32854|     ;; rhs = i64 %671
 32855|  %672 = call i64 @llvm.usub.sat.i64(i64 %670, i64 %671)                                                                ;L2472<703<399<1097
 32856|     ;; line_phase = !DIArgList(i64 %662, i64 %672)
 32857|  %673 = icmp ult i64 %662, %672                                                                                        ;L703<399<1097
 32858|     ;; line_phase = i1 %673
 32859|  %674 = or i1 %661, %673                                                                                               ;L1098
 32860|  br i1 %674, label %677, label %675                                                                                    ;L1098
 32861| 
 32862| 675: ; preds = %668, %667, %666
 32863|  %676 = invoke i64 @ai::minion_wave_risk29enemy_minion_wave_risk_dps_at(i64 %0, ptr %1, ptr %3, i64 %512, i64 %514)
 32864|  to label %698 unwind label %23                                                                                        ;L1099
 32865| 
 32866| 677: ; preds = %698, %668, %667
 32867|  %678 = phi i64 [ %699, %698 ], [ %631, %667 ], [ %631, %668 ]                                                         ;L0
 32868|     ;; self = i64 %678
 32869|     ;; enemy_dps = i64 %678
 32870|     ;; enemy_dps = i64 %678
 32871|     ;; self = ptr %3
 32872|     ;; self = ptr %3
 32873|     ;; self = ptr %3
 32874|  %679 = gep %3, i64 760                                                                                                ;L614<609<296<1968<1864<3787<1104
 32875|  %680 = load ptr, ptr %679, , !!8, !!8                                                                                 ;L614<609<296<1968<1864<3787<1104
 32876|  %681 = gep %3, i64 768                                                                                                ;L1864<3787<1104
 32877|  %682 = load i64, ptr %681, , !!8                                                                                      ;L1864<3787<1104
 32878|     ;; count = i64 %682
 32879|     ;; self[0..+8] = ptr %680
 32880|     ;; slice[0..+8] = ptr %680
 32881|     ;; self[8..+8] = i64 %682
 32882|     ;; slice[8..+8] = i64 %682
 32883|     ;; self = ptr %680
 32884|     ;; self[0..+8] = ptr %680
 32885|     ;; iter[0..+8] = ptr %680
 32886|     ;; self[0..+8] = ptr %680
 32887|     ;; self[8..+8] = !DIArgList(ptr %680, i64 %682)
 32888|     ;; iter[8..+8] = !DIArgList(ptr %680, i64 %682)
 32889|     ;; self[8..+8] = !DIArgList(ptr %680, i64 %682)
 32890|     ;; self[16..+8] = ptr %29
 32891|     ;; iter[16..+8] = ptr %29
 32892|     ;; self[16..+8] = ptr %29
 32893|     ;; self[24..+8] = ptr %3
 32894|     ;; iter[24..+8] = ptr %3
 32895|     ;; self[24..+8] = ptr %3
 32896|     ;; f[0..+8] = ptr %29
 32897|     ;; f[8..+8] = ptr %3
 32898|     ;; self[0..+8] = ptr %680
 32899|     ;; self[8..+8] = !DIArgList(ptr %680, i64 %682)
 32900|     ;; init = i64 0
 32901|     ;; rhs = i64 1
 32902|     ;; end = !DIArgList(ptr %680, i64 %682)
 32905|  %683 = icmp eq i64 %682, 0                                                                                            ;L1714<44<128<52<3674<1104
 32906|  br i1 %683, label %700, label %684                                                                                    ;L25<128<52<3674<1104
 32907| 
 32908| 684: ; preds = %694, %677
 32909|  %685 = phi i64 [ %696, %694 ], [ 0, %677 ]                                                                            ;L0<128<52<3674<1104
 32910|  %686 = phi i64 [ %695, %694 ], [ 0, %677 ]                                                                            ;L0<128<52<3674<1104
 32911|     ;; acc = i64 %686
 32912|     ;; self = i64 %685
 32913|     ;; i = i64 %685
 32914|     ;; self = ptr %680
 32915|     ;; count = i64 %685
 32916|  %687 = getelementptr { { { { { ptr, ptr } } }, {} }, {} }, ptr %680, i64 %685                                         ;L656<279<128<52<3674<1104
 32917|  %688 = load ptr, ptr %687, , !!43541, !!8, !!8                                                                        ;L279<128<52<3674<1104
 32918|  %689 = gep %687, i64 8                                                                                                ;L279<128<52<3674<1104
 32919|  %690 = load ptr, ptr %689, , !!43541, !!8, !!8                                                                        ;L279<128<52<3674<1104
 32921|     ;; acc = i64 %686
 32926|  %691 = gep %690, i64 144                                                                                              ;L1104<88<279<128<52<3674<1104
 32927|  %692 = load ptr, ptr %691, , !!8                                                                                      ;L1104<88<279<128<52<3674<1104
 32928|  %693 = invoke i64 %692(ptr %688, ptr %29, ptr %3)
 32929|  to label %694 unwind label %23                                                                                        ;L1104<88<279<128<52<3674<1104
 32930| 
 32931| 694: ; preds = %684
 32933|     ;; a = i64 %686
 32934|     ;; b = i64 %693
 32935|  %695 = add i64 %693, %686                                                                                             ;L55<88<279<128<52<3674<1104
 32936|     ;; acc = i64 %695
 32937|  %696 = add nuw i64 %685, 1                                                                                            ;L971<283<128<52<3674<1104
 32938|     ;; i = i64 %696
 32939|     ;; self = i64 %696
 32940|  %697 = icmp eq i64 %696, %682                                                                                         ;L284<128<52<3674<1104
 32941|  br i1 %697, label %700, label %684                                                                                    ;L284<128<52<3674<1104
 32942| 
 32943| 698: ; preds = %675
 32944|  %699 = add i64 %676, %631                                                                                             ;L1099
 32945|     ;; enemy_dps = i64 %699
 32946|     ;; enemy_dps = i64 %699
 32947|     ;; self = i64 %699
 32948|  br label %677                                                                                                         ;L1098
 32949| 
 32950| 700: ; preds = %694, %677
 32951|  %701 = phi i64 [ 0, %677 ], [ %695, %694 ]                                                                            ;L0<128<52<3674<1104
 32952|     ;; revive_hp = i64 %701
 32953|  %702 = add i64 %701, %655                                                                                             ;L1107
 32954|     ;; self = i64 %702
 32955|  %703 = call i64 @llvm.usub.sat.i64(i64 %702, i64 %564)                                                                ;L2472<1107
 32956|  %704 = mul i64 %703, 60                                                                                               ;L1107
 32957|     ;; self = i64 %678
 32958|     ;; other = i64 1
 32959|  %705 = call i64 @llvm.umax.i64(i64 %678, i64 1)                                                                       ;L1039<1107
 32961|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %5)
 32962|  to label %709 unwind label %706                                                                                       ;L825<1110
 32963| 
 32964| 706: ; preds = %700
 32965|  %707 = cleanuppad within none []
 32967|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %5) [ "funclet"(token %707) ]
 32968|  to label %708 unwind label %710                                                                                       ;L825<825<1110
 32969| 
 32970| 708: ; preds = %706
 32971|  cleanupret from %707 unwind label %710
 32972| 
 32973| 709: ; preds = %700
 32975|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %5)
 32976|  to label %712 unwind label %710                                                                                       ;L825<825<1110
 32977| 
 32978| 710: ; preds = %709, %708, %706, %23, %22, %21, %19
 32979|  %711 = cleanuppad within none []
 32980|  call fastcc void @core::ptr9drop_glueINtNtNtCshWfHDMLkPaX_7bumpalo11collections3vec3VecRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEECshdEBA0ozCnw_7game_ai(ptr %4) #32 [ "funclet"(token %711) ] ;L1110
 32981|  cleanupret from %711 unwind to caller                                                                                 ;L970
 32982| 
 32983| 712: ; preds = %709
 32985|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %4)
 32986|  to label %715 unwind label %713                                                                                       ;L825<1110
 32987| 
 32988| 713: ; preds = %712
 32989|  %714 = cleanuppad within none []
 32991|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %4) [ "funclet"(token %714) ] ;L825<825<1110
 32992|  cleanupret from %714 unwind to caller                                                                                 ;L825<1110
 32993| 
 32994| 715: ; preds = %712
 32995|  %716 = udiv i64 %704, %705                                                                                            ;L1107
 32997|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %4)    ;L825<825<1110
 32998|  br label %717                                                                                                         ;L1110
 32999| 
 33000| 717: ; preds = %736, %715
 33001|  %718 = phi i64 [ 9223372036854775807, %736 ], [ %716, %715 ]                                                          ;L0
 33002|  ret i64 %718                                                                                                          ;L1110
 33003| 
 33004| 719: ; preds = %730, %624
 33005|  %720 = phi i64 [ %615, %624 ], [ %732, %730 ]                                                                         ;L0
 33006|  %721 = phi ptr [ %626, %624 ], [ %731, %730 ]                                                                         ;L1086
 33007|     ;; iter[0..+8] = ptr %721
 33008|     ;; rhs = i64 %720
 33009|     ;; ally_heal = i64 %720
 33010|     ;; self = ptr undef
 33011|     ;; ptr = ptr %721
 33012|     ;; self = ptr %721
 33013|     ;; end_or_len = ptr %629
 33016|  %722 = icmp eq ptr %721, %629                                                                                         ;L1714<180<1086
 33017|  br i1 %722, label %609, label %723                                                                                    ;L180<1086
 33018| 
 33019| 723: ; preds = %719
 33020|     ;; iter[0..+8] = ptr %721
 33021|     ;; b = ptr %721
 33022|  %724 = load ptr, ptr %721, , !!8, !!8                                                                                 ;L1087
 33023|  %725 = gep %721, i64 8                                                                                                ;L1087
 33024|  %726 = load ptr, ptr %725, , !!8, !!8                                                                                 ;L1087
 33025|  %727 = gep %726, i64 136                                                                                              ;L1087
 33026|  %728 = load ptr, ptr %727, , !!8                                                                                      ;L1087
 33027|  %729 = invoke i64 %728(ptr %724, ptr %29, ptr %621, ptr %3)
 33028|  to label %730 unwind label %23                                                                                        ;L1087
 33029| 
 33030| 730: ; preds = %723
 33031|  %731 = gep %721, i64 16                                                                                               ;L656<185<1086
 33032|     ;; iter[0..+8] = ptr %731
 33033|  %732 = add i64 %729, %720                                                                                             ;L1087
 33034|     ;; ally_heal = i64 %732
 33035|     ;; rhs = i64 %732
 33036|  br label %719                                                                                                         ;L1086
 33037| 
 33038| 733: ; preds = %22
 33040|  invoke void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %4)
 33041|  to label %736 unwind label %734                                                                                       ;L825<1110
 33042| 
 33043| 734: ; preds = %733
 33044|  %735 = cleanuppad within none []
 33046|  call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %4) [ "funclet"(token %735) ] ;L825<825<1110
 33047|  cleanupret from %735 unwind to caller                                                                                 ;L825<1110
 33048| 
 33049| 736: ; preds = %733
 33051|  tail call void @gc::simulation6entity6EntityENtNtNtCsjihNppCmMEE_4core3ops4drop4Drop4dropCshdEBA0ozCnw_7game_ai(ptr %4) ;L825<825<1110
 33052|  br label %717                                                                                                         ;L1110
 33053| }
