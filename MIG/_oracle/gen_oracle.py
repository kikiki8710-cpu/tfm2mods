# -*- coding: utf-8 -*-
"""rule_scope pub 13함수 전수 진리표 프로브(o_main.rs) 생성기.

실행:  python C:\\tfm2mods\\MIG\\_oracle\\gen_oracle.py
"""
import io, os
os.chdir(os.path.dirname(os.path.abspath(__file__)))

TUT = ['None','First','TopSolo','Bottom','MidSolo','MidBottom','JungleOnly','Line','Total']
LINE = ['Mid','Bottom','Top']          # LineType 선언 순서 = 판별자 0,1,2
POS  = ['Top','Jungle','Mid','Bottom','Support']
JT   = ['Rhino','Mushroom','Stump','Bee','Morgard','Serpen']
ST   = ['Epic','Serpen']
OPH  = ['Setup','Assemble','Hunt','None']
STOP = ['TowerFocused','TowerDiveFail','Outnumbered','LowHp','BurstRisk','LowHpMatch']
CANC = ['LowHpSelf','LowHpAlly','TargetMissing','PlanChange','OperationAbort']
SGU  = ['StackAhead','Outnumbered']
SK   = ['Lurk','Commit']

L = []
def w(s): L.append(s)

w('#![allow(unused,dead_code,non_snake_case)]')
w('use game_core::*;')
w('use game_ai::plan_legacy::rule_scope as rs;')
w('use game_ai::plan_legacy::team_plan::{MainObjective as MO, SubObjective as SO, SubObjectiveType as SOT, ObjectPhase as OP};')
w('use game_ai::plan_legacy::steal::StealAction as SA;')
w('fn row(f:&str,inp:&str,out:&str){ println!("{}\\t{}\\t{}",f,inp,out); }')
w('fn main(){')
w('  let pool = bumpalo::Bump::new();')
w('  let setting: GameSetting = Default::default();')
w('  let mw: MacroWeights = Default::default();')
w('  let ms: MapSetting = Default::default();')
w('  let map = MapDef::moba(&setting);')
w('  let champs: Vec<String> = Vec::new();')
w('  let items: Vec<Box<dyn ItemInfo>> = Vec::new();')
w('  let tuts = [' + ','.join('TutorialType::' + t for t in TUT) + '];')
w('  let tnames = [' + ','.join('"%s"' % t for t in TUT) + '];')
w('  for ti in 0..9usize {')
w('    let ctx = GameContext{ pool:&pool, setting:&setting, macro_weights:&mw, map_setting:&ms,')
w('      map:&map, champion_list:&champs, item_list:&items, ignore_minion:false, debug:false,')
w('      tutorial:tuts[ti], trace_level:TraceLevel::Off };')
w('    let tn = tnames[ti];')

def emit(fn, inp_fmt, call):
    w('    row("%s", &format!("%s",tn), &format!("{}", %s));' % (fn, inp_fmt, call))
def emitd(fn, inp_fmt, call):
    w('    row("%s", &format!("%s",tn), &format!("{:?}", %s));' % (fn, inp_fmt, call))

# valid_lines
w('    { let v = rs::valid_lines(&ctx); let s: Vec<String> = v.iter().map(|x| format!("{:?}",x)).collect();')
w('      row("valid_lines", &format!("tut={}",tn), &s.join(",")); }')

for l in LINE:
    emit('line_exists', 'tut={};line=' + l, 'rs::line_exists(&ctx, LineType::%s)' % l)
    emitd('fallback_line', 'tut={};line=' + l, 'rs::fallback_line(&ctx, LineType::%s)' % l)
for p in POS:
    emit('position_exists', 'tut={};pos=' + p, 'rs::position_exists(&ctx, Position::%s)' % p)
emit('morgard_exists', 'tut={}', 'rs::morgard_exists(&ctx)')
emit('serpen_exists', 'tut={}', 'rs::serpen_exists(&ctx)')
for s in ST:
    emit('steal_target_allowed', 'tut={};tgt=' + s, 'rs::steal_target_allowed(&ctx, StealTarget::%s)' % s)
emit('steal_action_allowed', 'tut={};act=None', 'rs::steal_action_allowed(&ctx, SA::None)')
for k in ['Lurk','Commit']:
    for s in ST:
        emit('steal_action_allowed', 'tut={};act=%s(%s)' % (k, s),
             'rs::steal_action_allowed(&ctx, SA::%s(StealTarget::%s))' % (k, s))

# goal_allowed
for l in LINE:
    emit('goal_allowed', 'tut={};goal=Line(%s)' % l,
         'rs::goal_allowed(&ctx, BigGoal::Line{line:LineType::%s})' % l)
for j in JT:
    for t in (0, 1):
        emit('goal_allowed', 'tut={};goal=Jungle(%s,%d)' % (j, t),
             'rs::goal_allowed(&ctx, BigGoal::Jungle{camp:JungleType::%s, team:%d})' % (j, t))
emit('goal_allowed', 'tut={};goal=Epic', 'rs::goal_allowed(&ctx, BigGoal::Epic)')
emit('goal_allowed', 'tut={};goal=Serpen', 'rs::goal_allowed(&ctx, BigGoal::Serpen)')
for t in (0, 1):
    emit('goal_allowed', 'tut={};goal=Nexus(%d)' % t,
         'rs::goal_allowed(&ctx, BigGoal::Nexus{team:%d})' % t)
emit('goal_allowed', 'tut={};goal=Battle(None)', 'rs::goal_allowed(&ctx, BigGoal::Battle{focus:None})')
emit('goal_allowed', 'tut={};goal=Battle(Some(0))', 'rs::goal_allowed(&ctx, BigGoal::Battle{focus:Some(0)})')
emit('goal_allowed', 'tut={};goal=Recall', 'rs::goal_allowed(&ctx, BigGoal::Recall)')

# main_objective_allowed
for v in ['Morgard','Serpen']:
    for ph in OPH:
        for b in ('false','true'):
            emit('main_objective_allowed', 'tut={};mo=%s(ph=%s,wb=%s)' % (v, ph, b),
                 'rs::main_objective_allowed(&ctx, MO::%s{phase:OP::%s, with_battle:%s})' % (v, ph, b))
emit('main_objective_allowed', 'tut={};mo=Defense', 'rs::main_objective_allowed(&ctx, MO::Defense)')
emit('main_objective_allowed', 'tut={};mo=Repair', 'rs::main_objective_allowed(&ctx, MO::Repair)')
for v in ['DefenseLine','Nexus','PressEpic','SplitEpic']:
    for l in LINE:
        emit('main_objective_allowed', 'tut={};mo=%s(%s)' % (v, l),
             'rs::main_objective_allowed(&ctx, MO::%s(LineType::%s))' % (v, l))
for v in ['Gank','Dive','PressTower']:
    for l in LINE:
        emit('main_objective_allowed', 'tut={};mo=%s(%s)' % (v, l),
             'rs::main_objective_allowed(&ctx, MO::%s{line:LineType::%s})' % (v, l))
for l in LINE:
    for b in ('false','true'):
        emit('main_objective_allowed', 'tut={};mo=ComebackPick(%s,ready=%s)' % (l, b),
             'rs::main_objective_allowed(&ctx, MO::ComebackPick{line:LineType::%s, ready:%s})' % (l, b))

# sub_objective_allowed
for l in LINE:
    for b in ('false','true'):
        emit('sub_objective_allowed', 'tut={};so=LineBattle(%s,dive=%s)' % (l, b),
             'rs::sub_objective_allowed(&ctx, SO{ty:SOT::LineBattle{line:LineType::%s, do_dive:%s}, tick:0})' % (l, b))
for j in JT:
    for t in (0, 1):
        emit('sub_objective_allowed', 'tut={};so=JungleBattle(%s,%d)' % (j, t),
             'rs::sub_objective_allowed(&ctx, SO{ty:SOT::JungleBattle{camp:JungleType::%s, team:%d}, tick:0})' % (j, t))

# chat_allowed
CH = []
def ch(idx, name, args, lab=''):
    CH.append((idx, name, args, lab))

ch(0, 'Start', '(0)')
for p in POS: ch(1, 'Mia', '(Position::%s,0)' % p, 'pos=%s' % p)
for l in LINE: ch(2, 'JungleCheck', '(LineType::%s,0)' % l, 'line=%s' % l)
ch(3, 'Battle', '(0,0)')
ch(4, 'BattleDive', '(0,0)')
for l in LINE: ch(5, 'BattleLine', '(LineType::%s,0)' % l, 'line=%s' % l)
ch(6, 'BattleHelp', '(0,0)')
for r in STOP: ch(7, 'BattleStop', '(StopReason::%s)' % r, 'r=%s' % r)
for i, n in [(8,'GankRequest'),(9,'CoverLine'),(10,'GankLineCover'),(11,'HideLine'),
             (12,'HideLineToo'),(13,'LineCover'),(14,'DefenseLine')]:
    for l in LINE: ch(i, n, '(LineType::%s,0)' % l, 'line=%s' % l)
ch(15, 'Ok', '(0)')
ch(16, 'Reject', '(0)')
for r in CANC: ch(17, 'Cancel', '(CancelReason::%s)' % r, 'r=%s' % r)
for j in JT: ch(18, 'CounterJungle', '(JungleType::%s,0)' % j, 'camp=%s' % j)
ch(19, 'Lead', '(0)')
for i, n in [(20,'Split'),(21,'Press'),(22,'PressChange')]:
    for l in LINE: ch(i, n, '(LineType::%s,0)' % l, 'line=%s' % l)
ch(23, 'Repair', '(0)')
ch(24, 'SerpenPrepare', '(0,0)')
for i, n in [(25,'SerpenSetup'),(26,'SerpenCheck'),(27,'SerpenEnemyHunt'),
             (28,'SerpenAssemble'),(29,'SerpenHunt'),(30,'SerpenBattle')]:
    ch(i, n, '(0)')
for r in SGU: ch(31, 'SerpenGiveUp', '(SerpenGiveUpReason::%s)' % r, 'r=%s' % r)
for k in SK: ch(32, 'SerpenSteal', '(StealKind::%s)' % k, 'k=%s' % k)
ch(33, 'MorgardPrepare', '(0,0)')
for i, n in [(34,'MorgardSetup'),(35,'MorgardCheck'),(36,'MorgardEnemyHunt'),
             (37,'MorgardAssemble'),(38,'MorgardHunt'),(39,'MorgardBattle')]:
    ch(i, n, '(0)')
ch(40, 'MorgardGiveUp', '')
for k in SK: ch(41, 'MorgardSteal', '(StealKind::%s)' % k, 'k=%s' % k)
for l in LINE: ch(42, 'AttackNexus', '(LineType::%s,0)' % l, 'line=%s' % l)
ch(43, 'DefenseNexus', '(0)')
for i, n in [(44,'GankDive'),(45,'PressTower'),(46,'ComebackPick')]:
    for l in LINE: ch(i, n, '(LineType::%s,0)' % l, 'line=%s' % l)
for i, n in [(47,'PlayCall'),(48,'PlayPhaseChange')]:
    for a in range(8):
        for b in range(8): ch(i, n, '(%d,%d,0)' % (a, b), 'a=%d,b=%d' % (a, b))
for a in range(8):
    for b in range(8):
        for c in range(4): ch(49, 'PlayPropose', '(%d,%d,%d,0)' % (a, b, c), 'a=%d,b=%d,c=%d' % (a, b, c))
for l in LINE:
    for a in range(8): ch(50, 'GankPlan', '(LineType::%s,%d,0)' % (l, a), 'line=%s,a=%d' % (l, a))
for i, n in [(51,'EarlyPlan'),(52,'EarlyPlanChange'),(53,'AcceptAfterTask'),
             (54,'DeclineWithReason'),(56,'ReadySignal')]:
    for a in range(8): ch(i, n, '(%d,0)' % a, 'a=%d' % a)
for a in range(8):
    for b in range(8): ch(55, 'CounterRequest', '(%d,%d,0)' % (a, b), 'a=%d,b=%d' % (a, b))

for idx, name, args, lab in CH:
    tag = 'tut={};chat=%d:%s' % (idx, name) + ((';' + lab) if lab else '')
    emit('chat_allowed', tag, 'rs::chat_allowed(&ctx, &Chat::%s%s)' % (name, args))

w('  }')
w('}')

io.open('o_main.rs', 'w', encoding='utf-8').write('\n'.join(L) + '\n')
print('generated o_main.rs  lines=%d  chat_rows_per_tut=%d' % (len(L), len(CH)))
