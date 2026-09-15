 57757| define hidden zeroext i1 @ai::plan_legacy8sub_plan13battle_common29v15_can_keep_support_pressure(i64 %0, ptr %1, ptr %2, ptr %3, i64 %4, i64 %5) unnamed_addr #2 personality ptr @__CxxFrameHandler3 {
 57758|  %7 = alloca [56 x i8],
 57759|  %8 = alloca [16 x i8],
 57760|     ;; support_target[0..+8] = i64 %4
 57761|     ;; support_target[8..+8] = i64 %5
 57762|     ;; _version = i64 %0
 57763|     ;; player = ptr %1
 57764|     ;; data = ptr %2
 57765|     ;; parameter = ptr %3
 57766|     ;; len = i64 5
 57767|     ;; count = i64 5
 57770|  %9 = trunc nuw i64 %4 to i1                                                                                           ;L8
 57771|  br i1 %9, label %10, label %109                                                                                       ;L8
 57772| 
 57773| 10: ; preds = %6
 57774|     ;; target_id = i64 %5
 57775|  %11 = gep %1, i64 2352                                                                                                ;L12
 57776|  %12 = load i64, ptr %11, , !!8                                                                                        ;L12
 57777|  %13 = icmp ult i64 %12, 2                                                                                             ;L12
 57778|  br i1 %13, label %15, label %14                                                                                       ;L12
 57779| 
 57780| 14: ; preds = %10
 57781|  tail call void @core::panicking18panic_bounds_check(i64 %12, i64 2, ptr @anon.fdc8f8a40baf2989242b37eb35661f10.200) #29 ;L12
 57782|  unreachable                                                                                                           ;L12
 57783| 
 57784| 15: ; preds = %10
 57785|     ;; self = ptr %1
 57786|  %16 = gep %1, i64 2496                                                                                                ;L581<12
 57787|  %17 = load i32, ptr %16, , !!8                                                                                        ;L581<12
 57788|  %18 = zext nneg i32 %17 to i64                                                                                        ;L581<12
 57789|  %19 = load ptr, ptr %2, , !!8, !!8                                                                                    ;L12
 57790|     ;; self = ptr %19
 57791|  %20 = gep %19, i64 480                                                                                                ;L12
 57792|  %21 = getelementptr [5 x ptr], ptr %20, i64 %12                                                                       ;L12
 57793|  %22 = getelementptr ptr, ptr %21, i64 %18                                                                             ;L12
 57794|  %23 = load ptr, ptr %22, , !!8                                                                                        ;L12
 57795|  %24 = icmp eq ptr %23, null                                                                                           ;L12
 57796|  br i1 %24, label %109, label %25                                                                                      ;L12
 57797| 
 57798| 25: ; preds = %15
 57799|     ;; champ = ptr %23
 57800|     ;; other = ptr %23
 57801|  %26 = gep %23, i64 1648                                                                                               ;L16
 57802|  %27 = load i64, ptr %26, , !!8                                                                                        ;L16
 57803|  %28 = gep %23, i64 1576                                                                                               ;L16
 57804|  %29 = load i64, ptr %28, , !!8                                                                                        ;L16
 57805|     ;; self = i64 %29
 57806|     ;; other = i64 1
 57807|  %30 = tail call i64 @llvm.umax.i64(i64 %29, i64 1)                                                                    ;L1039<16
 57808|  %31 = mul i64 %27, 100                                                                                                ;L16
 57809|  %32 = udiv i64 %31, %30                                                                                               ;L16
 57810|  %33 = icmp ult i64 %32, 45                                                                                            ;L16
 57811|  %34 = gep %3, i64 2448
 57812|  %35 = load i64, ptr %34,
 57813|  %36 = icmp ne i64 %35, 0
 57814|  %37 = select i1 %33, i1 true, i1 %36                                                                                  ;L16
 57815|  br i1 %37, label %109, label %38                                                                                      ;L16
 57816| 
 57817| 38: ; preds = %25
 57818|  %39 = gep %3, i64 2440                                                                                                ;L18
 57819|  %40 = load i64, ptr %39, , !!8                                                                                        ;L18
 57820|  %41 = mul i64 %40, 100                                                                                                ;L18
 57821|     ;; self = i64 %27
 57822|     ;; other = i64 1
 57823|  %42 = tail call i64 @llvm.umax.i64(i64 %27, i64 1)                                                                    ;L1039<18
 57824|  %43 = mul i64 %42, 20                                                                                                 ;L18
 57825|  %44 = icmp ult i64 %41, %43                                                                                           ;L18
 57826|  br i1 %44, label %45, label %109                                                                                      ;L18
 57827| 
 57828| 45: ; preds = %38
 57829|  %46 = load ptr, ptr %19, , !!8, !!8                                                                                   ;L22
 57830|  %47 = gep %19, i64 8                                                                                                  ;L22
 57831|  %48 = load ptr, ptr %47, , !!8, !!8                                                                                   ;L22
 57832|  %49 = gep %48, i64 496                                                                                                ;L22
 57833|  %50 = load ptr, ptr %49, , !!8                                                                                        ;L22
 57834|  %51 = tail call ptr %50(ptr %46, i64 %5)                                                                              ;L22
 57835|  %52 = icmp eq ptr %51, null                                                                                           ;L22
 57836|  br i1 %52, label %109, label %53                                                                                      ;L22
 57837| 
 57838| 53: ; preds = %45
 57839|     ;; target = ptr %51
 57840|     ;; self = ptr %51
 57841|  %54 = tail call zeroext i1 @ai::plan_legacy8sub_plan13battle_common30v21_support_pressure_too_risky(ptr %1, ptr %2, ptr %3, ptr %23, ptr %51) ;L26
 57842|  br i1 %54, label %109, label %55                                                                                      ;L26
 57843| 
 57844| 55: ; preds = %53
 57845|     ;; self = ptr %51
 57846|     ;; self = ptr %51
 57847|     ;; other = ptr %23
 57848|     ;; other = ptr %23
 57849|  %56 = load i64, ptr %51, , !!8                                                                                        ;L1127<264<30
 57850|  %57 = gep %51, i64 8                                                                                                  ;L1127<264<30
 57851|     ;; __self_discr = i64 %56
 57852|  %58 = load i64, ptr %23, , !!8                                                                                        ;L1127<264<30
 57853|  %59 = gep %23, i64 8                                                                                                  ;L1127<264<30
 57854|     ;; __arg1_discr = i64 %58
 57855|  %60 = icmp eq i64 %56, %58                                                                                            ;L1127<264<30
 57856|  br i1 %60, label %61, label %82                                                                                       ;L1127<264<30
 57857| 
 57858| 61: ; preds = %55
 57859|  %62 = icmp eq i64 %56, 0                                                                                              ;L1127<264<30
 57860|  br i1 %62, label %63, label %67                                                                                       ;L1127<264<30
 57861| 
 57862| 63: ; preds = %61
 57863|     ;; __self_0 = ptr %51
 57864|     ;; self = ptr %51
 57865|     ;; __arg1_0 = ptr %23
 57866|     ;; other = ptr %23
 57869|  %64 = load i64, ptr %57, , !!8                                                                                        ;L1878<2123<1127<264<30
 57870|  %65 = load i64, ptr %59, , !!8                                                                                        ;L1878<2123<1127<264<30
 57871|  %66 = icmp eq i64 %64, %65                                                                                            ;L1878<2123<1127<264<30
 57872|  br i1 %66, label %67, label %82                                                                                       ;L30
 57873| 
 57874| 67: ; preds = %63, %61
 57876|     ;; team = !DIArgList(i64 1, i64 %12)
 57877|  %68 = sub nuw nsw i64 1, %12                                                                                          ;L35
 57878|     ;; team = i64 %68
 57879|  %69 = getelementptr [5 x ptr], ptr %20, i64 %68                                                                       ;L1905<35
 57880|     ;; self[0..+8] = ptr %69
 57881|     ;; slice[0..+8] = ptr %69
 57882|     ;; self[8..+8] = i64 5
 57883|     ;; slice[8..+8] = i64 5
 57884|     ;; ptr = ptr %69
 57885|     ;; self = ptr %69
 57886|  %70 = gep %69, i64 40                                                                                                 ;L961<100<1042<1905<35
 57887|     ;; self[0..+8] = ptr %69
 57888|     ;; self[8..+8] = ptr %70
 57889|  store ptr %69, ptr %8,                                                                                                ;L24<1002<1905<35
 57890|  %71 = gep %8, i64 8                                                                                                   ;L24<1002<1905<35
 57891|  store ptr %70, ptr %71,                                                                                               ;L24<1002<1905<35
 57892|     ;; self = ptr %8
 57893|     ;; self = ptr %8
 57894|  %72 = gep %2, i64 16                                                                                                  ;L36
 57895|  %73 = load ptr, ptr %72, , !!8, !!8                                                                                   ;L36
 57896|     ;; f[0..+8] = ptr %46
 57897|     ;; f[8..+8] = ptr %48
 57898|     ;; f[16..+8] = ptr %73
 57899|     ;; f[24..+8] = ptr %1
 57900|     ;; f[32..+8] = ptr %51
 57901|     ;; f[40..+8] = ptr %23
 57902|     ;; fold[0..+8] = ptr %46
 57903|     ;; fold[0..+8] = ptr %46
 57904|     ;; fold[8..+8] = ptr %48
 57905|     ;; fold[8..+8] = ptr %48
 57906|     ;; fold[16..+8] = ptr %73
 57907|     ;; fold[16..+8] = ptr %73
 57908|     ;; fold[24..+8] = ptr %1
 57909|     ;; fold[24..+8] = ptr %1
 57910|     ;; fold[32..+8] = ptr %51
 57911|     ;; fold[32..+8] = ptr %51
 57912|     ;; fold[40..+8] = ptr %23
 57913|     ;; fold[40..+8] = ptr %23
 57915|  %74 = gep %8, i64 16                                                                                                  ;L138<2897<36
 57916|     ;; f = ptr %74
 57917|  store ptr %74, ptr %7,                                                                                                ;L49<138<2897<36
 57918|  %75 = gep %7, i64 8                                                                                                   ;L49<138<2897<36
 57919|  store ptr %46, ptr %75,                                                                                               ;L49<138<2897<36
 57920|  %76 = gep %7, i64 16                                                                                                  ;L49<138<2897<36
 57921|  store ptr %48, ptr %76,                                                                                               ;L49<138<2897<36
 57922|  %77 = gep %7, i64 24                                                                                                  ;L49<138<2897<36
 57923|  store ptr %73, ptr %77,                                                                                               ;L49<138<2897<36
 57924|  %78 = gep %7, i64 32                                                                                                  ;L49<138<2897<36
 57925|  store ptr %1, ptr %78,                                                                                                ;L49<138<2897<36
 57926|  %79 = gep %7, i64 40                                                                                                  ;L49<138<2897<36
 57927|  store ptr %51, ptr %79,                                                                                               ;L49<138<2897<36
 57928|  %80 = gep %7, i64 48                                                                                                  ;L49<138<2897<36
 57929|  store ptr %23, ptr %80,                                                                                               ;L49<138<2897<36
 57930|  %81 = call fastcc zeroext i1 @core::slice4iter4IterINtNtBa_6option6OptionRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNtNtB28_8adapters10filter_map19filter_map_try_foldRBJ_B15_uINtNtNtBa_3ops12control_flow11ControlFlowuENCNvMs3_B1a_NtB1a_21AbstractGameWithCache14iter_champions0NCINvNvB22_3any5checkB15_NCNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan13battle_common29v15_can_keep_support_pressure0E0E0B3T_EB65_(ptr %8, ptr %7) ;L138<2897<36
 57933|  br label %109                                                                                                         ;L41
 57934| 
 57935| 82: ; preds = %63, %55
 57936|  %83 = sub nuw nsw i64 1, %12                                                                                          ;L31
 57937|  %84 = gep %2, i64 16                                                                                                  ;L31
 57938|  %85 = load ptr, ptr %84, , !!8, !!8                                                                                   ;L31
 57939|  %86 = getelementptr { { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, { { i64, [1 x i64] }, i64, i64, i32, [1 x i32] }, [5 x { i64, [2 x i64] }], [5 x { i64, { i8, [23 x i8] } }], [5 x { i8, [7 x i8], i64 }], [5 x i64], [5 x i64], [5 x i64], [5 x { i64, i64 }], i64, i64, i64, i64, i64, [5 x i32], i8, [3 x i8] }, ptr %85, i64 %83 ;L31
 57940|  %87 = tail call zeroext i1 @gc::simulation4game10blackboardNtB5_10Blackboard17is_recent_visible(ptr %86, ptr %46, ptr %48, ptr %1, ptr %51) ;L31
 57941|  br i1 %87, label %88, label %109                                                                                      ;L31
 57942| 
 57943| 88: ; preds = %82
 57944|  %89 = gep %51, i64 1632                                                                                               ;L2158<32
 57945|  %90 = load i64, ptr %89, , !!8                                                                                        ;L2158<32
 57946|     ;; x1 = i64 %90
 57947|     ;; self = i64 %90
 57948|  %91 = gep %51, i64 1640                                                                                               ;L2158<32
 57949|  %92 = load i64, ptr %91, , !!8                                                                                        ;L2158<32
 57950|     ;; y1 = i64 %92
 57951|     ;; self = i64 %92
 57952|  %93 = gep %23, i64 1632                                                                                               ;L2158<32
 57953|  %94 = load i64, ptr %93, , !!8                                                                                        ;L2158<32
 57954|     ;; x2 = i64 %94
 57955|     ;; other = i64 %94
 57956|  %95 = gep %23, i64 1640                                                                                               ;L2158<32
 57957|  %96 = load i64, ptr %95, , !!8                                                                                        ;L2158<32
 57958|     ;; y2 = i64 %96
 57959|     ;; other = i64 %96
 57960|  %97 = icmp ult i64 %90, %94                                                                                           ;L3147<7<2158<32
 57961|  %98 = sub nuw i64 %94, %90                                                                                            ;L3147<7<2158<32
 57962|  %99 = sub nuw i64 %90, %94                                                                                            ;L3147<7<2158<32
 57963|  %100 = select i1 %97, i64 %98, i64 %99                                                                                ;L3147<7<2158<32
 57964|     ;; dx = i64 %100
 57965|  %101 = icmp ult i64 %92, %96                                                                                          ;L3147<8<2158<32
 57966|  %102 = sub nuw i64 %96, %92                                                                                           ;L3147<8<2158<32
 57967|  %103 = sub nuw i64 %92, %96                                                                                           ;L3147<8<2158<32
 57968|  %104 = select i1 %101, i64 %102, i64 %103                                                                             ;L3147<8<2158<32
 57969|     ;; dy = i64 %104
 57970|  %105 = mul i64 %100, %100                                                                                             ;L9<2158<32
 57971|  %106 = mul i64 %104, %104                                                                                             ;L9<2158<32
 57972|  %107 = add i64 %106, %105                                                                                             ;L9<2158<32
 57973|  %108 = icmp ult i64 %107, 32400000001                                                                                 ;L32
 57974|  br label %109                                                                                                         ;L31
 57975| 
 57976| 109: ; preds = %88, %82, %67, %53, %45, %38, %25, %15, %6
 57977|  %110 = phi i1 [ false, %82 ], [ false, %45 ], [ %81, %67 ], [ %108, %88 ], [ false, %53 ], [ false, %15 ], [ false, %6 ], [ false, %38 ], [ false, %25 ]
 57978|  ret i1 %110                                                                                                           ;L41
 57979| }
