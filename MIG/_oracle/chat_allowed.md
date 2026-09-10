# `chat_allowed` 전수 진리표 (오라클 실측)

> Chat 태그별 튜토리얼 게이트

> 출처: SDK rlib 직접 링크 실행(`_oracle/o_main.rs`,`o_plan.rs`) / SDK `sdk_058` / 게임 0.5.8

| 입력 | 0 None | 1 First | 2 TopSolo | 3 Bottom | 4 MidSolo | 5 MidBottom | 6 JungleOnly | 7 Line | 8 Total | 허용 tut |
|---|---|---|---|---|---|---|---|---|---|---|
| `chat=0:Start` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=1:Mia;pos=Top` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=1:Mia;pos=Jungle` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `chat=1:Mia;pos=Mid` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=1:Mia;pos=Bottom` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=1:Mia;pos=Support` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=2:JungleCheck;line=Mid` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=2:JungleCheck;line=Bottom` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=2:JungleCheck;line=Top` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=3:Battle` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=4:BattleDive` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=5:BattleLine;line=Mid` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=5:BattleLine;line=Bottom` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=5:BattleLine;line=Top` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=6:BattleHelp` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=7:BattleStop;r=TowerFocused` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=7:BattleStop;r=TowerDiveFail` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=7:BattleStop;r=Outnumbered` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=7:BattleStop;r=LowHp` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=7:BattleStop;r=BurstRisk` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=7:BattleStop;r=LowHpMatch` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=8:GankRequest;line=Mid` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=8:GankRequest;line=Bottom` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=8:GankRequest;line=Top` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=9:CoverLine;line=Mid` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=9:CoverLine;line=Bottom` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=9:CoverLine;line=Top` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=10:GankLineCover;line=Mid` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=10:GankLineCover;line=Bottom` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=10:GankLineCover;line=Top` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=11:HideLine;line=Mid` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=11:HideLine;line=Bottom` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=11:HideLine;line=Top` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=12:HideLineToo;line=Mid` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=12:HideLineToo;line=Bottom` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=12:HideLineToo;line=Top` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=13:LineCover;line=Mid` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=13:LineCover;line=Bottom` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=13:LineCover;line=Top` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=14:DefenseLine;line=Mid` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=14:DefenseLine;line=Bottom` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=14:DefenseLine;line=Top` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=15:Ok` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=16:Reject` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=17:Cancel;r=LowHpSelf` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=17:Cancel;r=LowHpAlly` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=17:Cancel;r=TargetMissing` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=17:Cancel;r=PlanChange` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=17:Cancel;r=OperationAbort` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=18:CounterJungle;camp=Rhino` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `chat=18:CounterJungle;camp=Mushroom` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `chat=18:CounterJungle;camp=Stump` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `chat=18:CounterJungle;camp=Bee` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `chat=18:CounterJungle;camp=Morgard` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=18:CounterJungle;camp=Serpen` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=19:Lead` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=20:Split;line=Mid` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=20:Split;line=Bottom` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=20:Split;line=Top` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=21:Press;line=Mid` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=21:Press;line=Bottom` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=21:Press;line=Top` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=22:PressChange;line=Mid` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=22:PressChange;line=Bottom` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=22:PressChange;line=Top` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=23:Repair` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=24:SerpenPrepare` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=25:SerpenSetup` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=26:SerpenCheck` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=27:SerpenEnemyHunt` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=28:SerpenAssemble` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=29:SerpenHunt` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=30:SerpenBattle` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=31:SerpenGiveUp;r=StackAhead` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=31:SerpenGiveUp;r=Outnumbered` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=32:SerpenSteal;k=Lurk` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=32:SerpenSteal;k=Commit` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=33:MorgardPrepare` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=34:MorgardSetup` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=35:MorgardCheck` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=36:MorgardEnemyHunt` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=37:MorgardAssemble` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=38:MorgardHunt` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=39:MorgardBattle` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=40:MorgardGiveUp` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=41:MorgardSteal;k=Lurk` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=41:MorgardSteal;k=Commit` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=42:AttackNexus;line=Mid` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=42:AttackNexus;line=Bottom` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=42:AttackNexus;line=Top` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=43:DefenseNexus` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=44:GankDive;line=Mid` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=44:GankDive;line=Bottom` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=44:GankDive;line=Top` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=45:PressTower;line=Mid` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=45:PressTower;line=Bottom` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=45:PressTower;line=Top` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=46:ComebackPick;line=Mid` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=46:ComebackPick;line=Bottom` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=46:ComebackPick;line=Top` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=0,b=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=47:PlayCall;a=0,b=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=47:PlayCall;a=0,b=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=47:PlayCall;a=0,b=3` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=47:PlayCall;a=0,b=4` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=47:PlayCall;a=0,b=5` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=47:PlayCall;a=0,b=6` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=47:PlayCall;a=0,b=7` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=47:PlayCall;a=1,b=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=47:PlayCall;a=1,b=1` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=47:PlayCall;a=1,b=2` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=47:PlayCall;a=1,b=3` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=47:PlayCall;a=1,b=4` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=47:PlayCall;a=1,b=5` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=47:PlayCall;a=1,b=6` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=47:PlayCall;a=1,b=7` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=47:PlayCall;a=2,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=2,b=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=47:PlayCall;a=2,b=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=47:PlayCall;a=2,b=3` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=2,b=4` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=2,b=5` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=2,b=6` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=2,b=7` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=3,b=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=47:PlayCall;a=3,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=3,b=2` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=47:PlayCall;a=3,b=3` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=3,b=4` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=3,b=5` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=3,b=6` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=3,b=7` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=4,b=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=47:PlayCall;a=4,b=1` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=47:PlayCall;a=4,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=4,b=3` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=4,b=4` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=4,b=5` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=4,b=6` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=4,b=7` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=5,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=5,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=5,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=5,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=5,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=5,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=5,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=5,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=6,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=6,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=6,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=6,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=6,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=6,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=6,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=6,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=7,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=7,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=7,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=7,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=7,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=7,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=7,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=7,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=48:PlayPhaseChange;a=0,b=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=48:PlayPhaseChange;a=0,b=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=48:PlayPhaseChange;a=0,b=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=48:PlayPhaseChange;a=0,b=3` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=48:PlayPhaseChange;a=0,b=4` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=48:PlayPhaseChange;a=0,b=5` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=48:PlayPhaseChange;a=0,b=6` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=48:PlayPhaseChange;a=0,b=7` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=48:PlayPhaseChange;a=1,b=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=48:PlayPhaseChange;a=1,b=1` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=1,b=2` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=1,b=3` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=1,b=4` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=1,b=5` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=1,b=6` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=1,b=7` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=2,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=48:PlayPhaseChange;a=2,b=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=48:PlayPhaseChange;a=2,b=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=48:PlayPhaseChange;a=2,b=3` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=48:PlayPhaseChange;a=2,b=4` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=48:PlayPhaseChange;a=2,b=5` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=48:PlayPhaseChange;a=2,b=6` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=48:PlayPhaseChange;a=2,b=7` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=48:PlayPhaseChange;a=3,b=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=48:PlayPhaseChange;a=3,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=3,b=2` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=3,b=3` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=3,b=4` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=3,b=5` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=3,b=6` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=3,b=7` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=4,b=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=48:PlayPhaseChange;a=4,b=1` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=4,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=4,b=3` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=4,b=4` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=4,b=5` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=4,b=6` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=4,b=7` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=5,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=48:PlayPhaseChange;a=5,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=5,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=5,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=48:PlayPhaseChange;a=5,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=48:PlayPhaseChange;a=5,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=48:PlayPhaseChange;a=5,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=48:PlayPhaseChange;a=5,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=48:PlayPhaseChange;a=6,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=48:PlayPhaseChange;a=6,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=6,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=6,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=48:PlayPhaseChange;a=6,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=48:PlayPhaseChange;a=6,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=48:PlayPhaseChange;a=6,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=48:PlayPhaseChange;a=6,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=48:PlayPhaseChange;a=7,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=48:PlayPhaseChange;a=7,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=7,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=48:PlayPhaseChange;a=7,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=48:PlayPhaseChange;a=7,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=48:PlayPhaseChange;a=7,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=48:PlayPhaseChange;a=7,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=48:PlayPhaseChange;a=7,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=0,b=0,c=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=0,c=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=0,c=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=0,c=3` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=1,c=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=1,c=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=1,c=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=1,c=3` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=2,c=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=2,c=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=2,c=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=2,c=3` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=3,c=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=3,c=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=3,c=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=3,c=3` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=4,c=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=4,c=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=4,c=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=4,c=3` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=5,c=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=5,c=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=5,c=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=5,c=3` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=6,c=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=6,c=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=6,c=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=6,c=3` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=7,c=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=7,c=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=7,c=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=7,c=3` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=1,b=0,c=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=1,b=0,c=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=1,b=0,c=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=1,b=0,c=3` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=1,b=1,c=0` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=1,c=1` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=1,c=2` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=1,c=3` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=2,c=0` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=2,c=1` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=2,c=2` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=2,c=3` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=3,c=0` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=3,c=1` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=3,c=2` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=3,c=3` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=4,c=0` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=4,c=1` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=4,c=2` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=4,c=3` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=5,c=0` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=5,c=1` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=5,c=2` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=5,c=3` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=6,c=0` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=6,c=1` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=6,c=2` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=6,c=3` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=7,c=0` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=7,c=1` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=7,c=2` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=7,c=3` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=2,b=0,c=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=0,c=1` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=0,c=2` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=0,c=3` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=1,c=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=2,b=1,c=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=2,b=1,c=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=2,b=1,c=3` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=2,b=2,c=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=2,b=2,c=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=2,b=2,c=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=2,b=2,c=3` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=2,b=3,c=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=3,c=1` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=3,c=2` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=3,c=3` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=4,c=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=4,c=1` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=4,c=2` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=4,c=3` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=5,c=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=5,c=1` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=5,c=2` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=5,c=3` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=6,c=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=6,c=1` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=6,c=2` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=6,c=3` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=7,c=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=7,c=1` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=7,c=2` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=7,c=3` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=3,b=0,c=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=3,b=0,c=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=3,b=0,c=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=3,b=0,c=3` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=3,b=1,c=0` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=1,c=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=1,c=2` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=1,c=3` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=2,c=0` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=2,c=1` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=2,c=2` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=2,c=3` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=3,c=0` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=3,c=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=3,c=2` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=3,c=3` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=4,c=0` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=4,c=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=4,c=2` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=4,c=3` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=5,c=0` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=5,c=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=5,c=2` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=5,c=3` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=6,c=0` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=6,c=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=6,c=2` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=6,c=3` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=7,c=0` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=7,c=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=7,c=2` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=7,c=3` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=0,c=0` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=4,b=0,c=1` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=4,b=0,c=2` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=4,b=0,c=3` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=4,b=1,c=0` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=1,c=1` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=1,c=2` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=1,c=3` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=2,c=0` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=2,c=1` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=2,c=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=2,c=3` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=3,c=0` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=3,c=1` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=3,c=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=3,c=3` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=4,c=0` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=4,c=1` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=4,c=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=4,c=3` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=5,c=0` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=5,c=1` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=5,c=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=5,c=3` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=6,c=0` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=6,c=1` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=6,c=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=6,c=3` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=7,c=0` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=7,c=1` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=7,c=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=7,c=3` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=0,c=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=5,b=0,c=1` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=5,b=0,c=2` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=5,b=0,c=3` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=5,b=1,c=0` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=1,c=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=1,c=2` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=1,c=3` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=2,c=0` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=2,c=1` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=2,c=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=2,c=3` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=3,c=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=3,c=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=3,c=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=3,c=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=4,c=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=4,c=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=4,c=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=4,c=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=5,c=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=5,c=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=5,c=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=5,c=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=6,c=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=6,c=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=6,c=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=6,c=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=7,c=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=7,c=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=7,c=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=7,c=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=0,c=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=6,b=0,c=1` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=6,b=0,c=2` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=6,b=0,c=3` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=6,b=1,c=0` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=6,b=1,c=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=6,b=1,c=2` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=6,b=1,c=3` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=6,b=2,c=0` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=6,b=2,c=1` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=6,b=2,c=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=6,b=2,c=3` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=6,b=3,c=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=3,c=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=3,c=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=3,c=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=4,c=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=4,c=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=4,c=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=4,c=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=5,c=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=5,c=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=5,c=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=5,c=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=6,c=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=6,c=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=6,c=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=6,c=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=7,c=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=7,c=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=7,c=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=7,c=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=0,c=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=7,b=0,c=1` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=7,b=0,c=2` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=7,b=0,c=3` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=7,b=1,c=0` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=7,b=1,c=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=7,b=1,c=2` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=7,b=1,c=3` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=7,b=2,c=0` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=7,b=2,c=1` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=7,b=2,c=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=7,b=2,c=3` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=7,b=3,c=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=3,c=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=3,c=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=3,c=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=4,c=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=4,c=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=4,c=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=4,c=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=5,c=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=5,c=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=5,c=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=5,c=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=6,c=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=6,c=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=6,c=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=6,c=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=7,c=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=7,c=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=7,c=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=7,c=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=50:GankPlan;line=Mid,a=0` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=50:GankPlan;line=Mid,a=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=50:GankPlan;line=Mid,a=2` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=50:GankPlan;line=Mid,a=3` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=50:GankPlan;line=Mid,a=4` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=50:GankPlan;line=Mid,a=5` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=50:GankPlan;line=Mid,a=6` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=50:GankPlan;line=Mid,a=7` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=50:GankPlan;line=Bottom,a=0` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=50:GankPlan;line=Bottom,a=1` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=50:GankPlan;line=Bottom,a=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=50:GankPlan;line=Bottom,a=3` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=50:GankPlan;line=Bottom,a=4` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=50:GankPlan;line=Bottom,a=5` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=50:GankPlan;line=Bottom,a=6` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=50:GankPlan;line=Bottom,a=7` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=50:GankPlan;line=Top,a=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=50:GankPlan;line=Top,a=1` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=50:GankPlan;line=Top,a=2` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=50:GankPlan;line=Top,a=3` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=50:GankPlan;line=Top,a=4` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=50:GankPlan;line=Top,a=5` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=50:GankPlan;line=Top,a=6` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=50:GankPlan;line=Top,a=7` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=51:EarlyPlan;a=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=51:EarlyPlan;a=1` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=51:EarlyPlan;a=2` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=51:EarlyPlan;a=3` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=51:EarlyPlan;a=4` | O | . | . | . | . | . | O | . | O | `JUNGLE` [0, 6, 8] |
| `chat=51:EarlyPlan;a=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=51:EarlyPlan;a=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=51:EarlyPlan;a=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=52:EarlyPlanChange;a=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=52:EarlyPlanChange;a=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=52:EarlyPlanChange;a=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=52:EarlyPlanChange;a=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=52:EarlyPlanChange;a=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=52:EarlyPlanChange;a=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=52:EarlyPlanChange;a=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=52:EarlyPlanChange;a=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=53:AcceptAfterTask;a=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=53:AcceptAfterTask;a=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=53:AcceptAfterTask;a=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=53:AcceptAfterTask;a=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=53:AcceptAfterTask;a=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=53:AcceptAfterTask;a=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=53:AcceptAfterTask;a=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=53:AcceptAfterTask;a=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=54:DeclineWithReason;a=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=54:DeclineWithReason;a=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=54:DeclineWithReason;a=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=54:DeclineWithReason;a=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=54:DeclineWithReason;a=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=54:DeclineWithReason;a=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=54:DeclineWithReason;a=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=54:DeclineWithReason;a=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=56:ReadySignal;a=0` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=56:ReadySignal;a=1` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=56:ReadySignal;a=2` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=56:ReadySignal;a=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=56:ReadySignal;a=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=56:ReadySignal;a=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=56:ReadySignal;a=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=56:ReadySignal;a=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=0,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=0,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=0,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=0,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=0,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=0,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=0,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=0,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=1,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=1,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=1,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=1,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=1,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=1,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=1,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=1,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=2,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=2,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=2,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=2,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=2,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=2,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=2,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=2,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=3,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=3,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=3,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=3,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=3,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=3,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=3,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=3,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=4,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=4,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=4,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=4,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=4,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=4,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=4,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=4,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=5,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=5,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=5,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=5,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=5,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=5,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=5,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=5,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=6,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=6,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=6,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=6,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=6,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=6,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=6,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=6,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=7,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=7,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=7,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=7,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=7,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=7,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=7,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=7,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=0,b=0,c=4` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=0,c=5` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=0,c=6` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=0,c=7` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=1,c=4` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=1,c=5` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=1,c=6` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=1,c=7` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=2,c=4` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=2,c=5` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=2,c=6` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=2,c=7` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=3,c=4` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=3,c=5` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=3,c=6` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=3,c=7` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=4,c=4` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=4,c=5` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=4,c=6` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=4,c=7` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=5,c=4` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=5,c=5` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=5,c=6` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=5,c=7` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=6,c=4` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=6,c=5` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=6,c=6` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=6,c=7` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=7,c=4` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=7,c=5` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=7,c=6` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=0,b=7,c=7` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=1,b=0,c=4` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=1,b=0,c=5` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=1,b=0,c=6` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=1,b=0,c=7` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=1,b=1,c=4` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=1,c=5` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=1,c=6` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=1,c=7` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=2,c=4` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=2,c=5` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=2,c=6` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=2,c=7` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=3,c=4` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=3,c=5` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=3,c=6` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=3,c=7` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=4,c=4` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=4,c=5` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=4,c=6` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=4,c=7` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=5,c=4` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=5,c=5` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=5,c=6` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=5,c=7` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=6,c=4` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=6,c=5` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=6,c=6` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=6,c=7` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=7,c=4` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=7,c=5` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=7,c=6` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=1,b=7,c=7` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=2,b=0,c=4` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=0,c=5` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=0,c=6` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=0,c=7` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=1,c=4` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=2,b=1,c=5` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=2,b=1,c=6` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=2,b=1,c=7` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=2,b=2,c=4` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=2,b=2,c=5` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=2,b=2,c=6` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=2,b=2,c=7` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=2,b=3,c=4` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=3,c=5` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=3,c=6` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=3,c=7` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=4,c=4` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=4,c=5` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=4,c=6` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=4,c=7` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=5,c=4` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=5,c=5` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=5,c=6` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=5,c=7` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=6,c=4` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=6,c=5` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=6,c=6` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=6,c=7` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=7,c=4` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=7,c=5` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=7,c=6` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=2,b=7,c=7` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=3,b=0,c=4` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=3,b=0,c=5` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=3,b=0,c=6` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=3,b=0,c=7` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=3,b=1,c=4` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=1,c=5` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=1,c=6` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=1,c=7` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=2,c=4` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=2,c=5` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=2,c=6` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=2,c=7` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=3,c=4` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=3,c=5` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=3,c=6` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=3,c=7` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=4,c=4` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=4,c=5` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=4,c=6` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=4,c=7` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=5,c=4` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=5,c=5` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=5,c=6` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=5,c=7` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=6,c=4` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=6,c=5` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=6,c=6` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=6,c=7` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=7,c=4` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=7,c=5` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=7,c=6` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=3,b=7,c=7` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=0,c=4` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=4,b=0,c=5` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=4,b=0,c=6` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=4,b=0,c=7` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=49:PlayPropose;a=4,b=1,c=4` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=1,c=5` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=1,c=6` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=1,c=7` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=2,c=4` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=2,c=5` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=2,c=6` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=2,c=7` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=3,c=4` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=3,c=5` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=3,c=6` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=3,c=7` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=4,c=4` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=4,c=5` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=4,c=6` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=4,c=7` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=5,c=4` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=5,c=5` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=5,c=6` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=5,c=7` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=6,c=4` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=6,c=5` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=6,c=6` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=6,c=7` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=7,c=4` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=7,c=5` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=7,c=6` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=4,b=7,c=7` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=0,c=4` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=5,b=0,c=5` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=5,b=0,c=6` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=5,b=0,c=7` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=5,b=1,c=4` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=1,c=5` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=1,c=6` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=1,c=7` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=2,c=4` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=2,c=5` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=2,c=6` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=2,c=7` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=5,b=3,c=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=3,c=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=3,c=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=3,c=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=4,c=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=4,c=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=4,c=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=4,c=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=5,c=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=5,c=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=5,c=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=5,c=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=6,c=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=6,c=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=6,c=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=6,c=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=7,c=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=7,c=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=7,c=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=5,b=7,c=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=0,c=4` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=6,b=0,c=5` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=6,b=0,c=6` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=6,b=0,c=7` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=6,b=1,c=4` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=6,b=1,c=5` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=6,b=1,c=6` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=6,b=1,c=7` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=6,b=2,c=4` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=6,b=2,c=5` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=6,b=2,c=6` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=6,b=2,c=7` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=6,b=3,c=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=3,c=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=3,c=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=3,c=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=4,c=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=4,c=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=4,c=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=4,c=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=5,c=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=5,c=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=5,c=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=5,c=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=6,c=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=6,c=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=6,c=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=6,c=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=7,c=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=7,c=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=7,c=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=6,b=7,c=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=0,c=4` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=7,b=0,c=5` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=7,b=0,c=6` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=7,b=0,c=7` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=49:PlayPropose;a=7,b=1,c=4` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=7,b=1,c=5` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=7,b=1,c=6` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=7,b=1,c=7` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=49:PlayPropose;a=7,b=2,c=4` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=7,b=2,c=5` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=7,b=2,c=6` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=7,b=2,c=7` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=49:PlayPropose;a=7,b=3,c=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=3,c=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=3,c=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=3,c=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=4,c=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=4,c=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=4,c=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=4,c=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=5,c=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=5,c=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=5,c=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=5,c=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=6,c=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=6,c=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=6,c=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=6,c=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=7,c=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=7,c=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=7,c=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=49:PlayPropose;a=7,b=7,c=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=50:GankPlan;line=Mid,a=8` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=50:GankPlan;line=Mid,a=9` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=50:GankPlan;line=Mid,a=10` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=50:GankPlan;line=Mid,a=11` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=50:GankPlan;line=Mid,a=12` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=50:GankPlan;line=Mid,a=13` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=50:GankPlan;line=Mid,a=14` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=50:GankPlan;line=Mid,a=15` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=50:GankPlan;line=Bottom,a=8` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=50:GankPlan;line=Bottom,a=9` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=50:GankPlan;line=Bottom,a=10` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=50:GankPlan;line=Bottom,a=11` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=50:GankPlan;line=Bottom,a=12` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=50:GankPlan;line=Bottom,a=13` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=50:GankPlan;line=Bottom,a=14` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=50:GankPlan;line=Bottom,a=15` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=50:GankPlan;line=Top,a=8` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=50:GankPlan;line=Top,a=9` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=50:GankPlan;line=Top,a=10` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=50:GankPlan;line=Top,a=11` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=50:GankPlan;line=Top,a=12` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=50:GankPlan;line=Top,a=13` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=50:GankPlan;line=Top,a=14` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=50:GankPlan;line=Top,a=15` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=0,b=8` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=55:CounterRequest;a=0,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=0,b=9` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=55:CounterRequest;a=0,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=0,b=10` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=55:CounterRequest;a=0,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=0,b=11` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=55:CounterRequest;a=0,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=0,b=12` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=55:CounterRequest;a=0,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=0,b=13` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=55:CounterRequest;a=0,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=0,b=14` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=55:CounterRequest;a=0,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=0,b=15` | O | . | . | . | . | . | . | O | O | `EPIC` [0, 7, 8] |
| `chat=55:CounterRequest;a=0,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=1,b=8` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=55:CounterRequest;a=1,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=1,b=9` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=55:CounterRequest;a=1,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=1,b=10` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=55:CounterRequest;a=1,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=1,b=11` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=55:CounterRequest;a=1,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=1,b=12` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=55:CounterRequest;a=1,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=1,b=13` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=55:CounterRequest;a=1,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=1,b=14` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=55:CounterRequest;a=1,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=1,b=15` | O | . | . | . | . | O | . | O | O | `SERPEN` [0, 5, 7, 8] |
| `chat=55:CounterRequest;a=1,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=2,b=8` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=2,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=2,b=9` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=2,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=2,b=10` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=2,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=2,b=11` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=2,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=2,b=12` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=2,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=2,b=13` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=2,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=2,b=14` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=2,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=2,b=15` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=2,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=3,b=8` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=3,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=3,b=9` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=3,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=3,b=10` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=3,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=3,b=11` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=3,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=3,b=12` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=3,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=3,b=13` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=3,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=3,b=14` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=3,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=3,b=15` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=3,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=4,b=8` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=4,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=4,b=9` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=4,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=4,b=10` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=4,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=4,b=11` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=4,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=4,b=12` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=4,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=4,b=13` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=4,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=4,b=14` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=4,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=4,b=15` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=4,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=5,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=5,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=5,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=5,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=5,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=5,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=5,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=5,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=5,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=5,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=5,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=5,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=5,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=5,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=5,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=5,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=6,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=6,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=6,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=6,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=6,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=6,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=6,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=6,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=6,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=6,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=6,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=6,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=6,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=6,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=6,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=6,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=7,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=7,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=7,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=7,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=7,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=7,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=7,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=7,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=7,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=7,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=7,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=7,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=7,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=7,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=7,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=7,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=8,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=8,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=8,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=8,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=8,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=8,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=8,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=8,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=8,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=8,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=8,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=8,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=8,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=8,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=8,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=8,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=8,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=8,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=8,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=8,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=8,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=8,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=8,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=8,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=8,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=8,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=8,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=8,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=8,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=8,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=8,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=8,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=9,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=9,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=9,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=9,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=9,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=9,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=9,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=9,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=9,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=9,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=9,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=9,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=9,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=9,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=9,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=9,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=9,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=9,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=9,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=9,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=9,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=9,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=9,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=9,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=9,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=9,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=9,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=9,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=9,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=9,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=9,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=9,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=10,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=10,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=10,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=10,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=10,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=10,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=10,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=10,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=10,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=10,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=10,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=10,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=10,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=10,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=10,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=10,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=10,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=10,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=10,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=10,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=10,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=10,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=10,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=10,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=10,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=10,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=10,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=10,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=10,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=10,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=10,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=10,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=11,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=11,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=11,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=11,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=11,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=11,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=11,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=11,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=11,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=11,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=11,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=11,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=11,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=11,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=11,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=11,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=11,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=11,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=11,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=11,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=11,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=11,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=11,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=11,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=11,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=11,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=11,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=11,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=11,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=11,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=11,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=11,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=12,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=12,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=12,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=12,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=12,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=12,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=12,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=12,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=12,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=12,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=12,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=12,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=12,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=12,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=12,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=12,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=12,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=12,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=12,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=12,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=12,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=12,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=12,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=12,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=12,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=12,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=12,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=12,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=12,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=12,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=12,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=12,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=13,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=13,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=13,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=13,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=13,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=13,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=13,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=13,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=13,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=13,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=13,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=13,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=13,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=13,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=13,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=13,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=13,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=13,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=13,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=13,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=13,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=13,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=13,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=13,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=13,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=13,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=13,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=13,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=13,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=13,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=13,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=13,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=14,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=14,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=14,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=14,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=14,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=14,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=14,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=14,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=14,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=14,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=14,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=14,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=14,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=14,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=14,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=14,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=14,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=14,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=14,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=14,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=14,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=14,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=14,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=14,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=14,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=14,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=14,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=14,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=14,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=14,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=14,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=14,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=15,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=55:CounterRequest;a=15,b=0` | O | . | O | . | . | . | . | O | O | `L_Top` [0, 2, 7, 8] |
| `chat=47:PlayCall;a=15,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=55:CounterRequest;a=15,b=1` | O | . | . | . | O | O | . | O | O | `L_Mid` [0, 4, 5, 7, 8] |
| `chat=47:PlayCall;a=15,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=55:CounterRequest;a=15,b=2` | O | O | . | O | . | O | . | O | O | `L_Bot` [0, 1, 3, 5, 7, 8] |
| `chat=47:PlayCall;a=15,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=15,b=3` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=15,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=15,b=4` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=15,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=15,b=5` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=15,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=15,b=6` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=15,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=15,b=7` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=15,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=15,b=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=15,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=15,b=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=15,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=15,b=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=15,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=15,b=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=15,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=15,b=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=15,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=15,b=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=15,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=15,b=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=47:PlayCall;a=15,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=55:CounterRequest;a=15,b=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=51:EarlyPlan;a=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=52:EarlyPlanChange;a=8` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=51:EarlyPlan;a=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=52:EarlyPlanChange;a=9` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=51:EarlyPlan;a=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=52:EarlyPlanChange;a=10` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=51:EarlyPlan;a=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=52:EarlyPlanChange;a=11` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=51:EarlyPlan;a=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=52:EarlyPlanChange;a=12` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=51:EarlyPlan;a=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=52:EarlyPlanChange;a=13` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=51:EarlyPlan;a=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=52:EarlyPlanChange;a=14` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=51:EarlyPlan;a=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
| `chat=52:EarlyPlanChange;a=15` | O | O | O | O | O | O | O | O | O | `ALL` [0, 1, 2, 3, 4, 5, 6, 7, 8] |
