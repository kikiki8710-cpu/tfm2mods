# -*- coding: utf-8 -*-
"""문서의 진단 규칙 정정 — `checked=` 만 보고 판정하면 안 된다.
2026-08-06 실사고: 체인이 정상 실행(03:17:54, *_imm.txt 40개 갱신)된 뒤 게임이 한 번 더 떴고,
그 짧은 프로세스가 imm_guard_summary.txt 를 checked=10 으로 덮었다.
그 값을 '무효 판'으로 읽고 원인을 두 번 잘못 지목했다(오프셋 되돌림 · champ_verify OFF — 둘 다 무관)."""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

REP = 'C:/Users/dev/Desktop/claude/tfm2/mods_report/tfm2_ai_adjust/'
n = 0


def sub(fname, old, new, tag):
    global n
    p = REP + fname
    t = io.open(p, encoding='utf-8').read()
    if old not in t:
        print('  [건너뜀] %s / %s' % (fname, tag)); return
    io.open(p, 'w', encoding='utf-8', newline='\n').write(t.replace(old, new, 1))
    n += 1
    print('  [ok] %s / %s' % (fname, tag))


sub('01_구조.md',
'''★진단법: `checked=756`이면 체인이 돌았다. `checked=10`이면 **그 판의 노브 결과는 전부 무효**다 —
`applied` 수치를 읽기 전에 이것부터 본다.''',
'''★진단법 — ⚠**`checked=` 숫자만 보면 안 된다. 타임스탬프를 함께 봐야 한다.**

`imm_guard_summary.txt` 는 **가장 최근 프로세스**의 값이다. 체인이 정상 실행된 뒤 게임이 한 번 더 뜨면,
그 짧은 프로세스가 이 파일을 `checked=10` 으로 **덮어쓴다**. 그러면 노브는 멀쩡히 적용됐는데도
"그 판은 무효"로 오독하게 된다(2026-08-06 실사고 — 이 오독으로 원인을 두 번 잘못 지목했다).

**올바른 순서**
1. `*_imm.txt` 들의 **LastWriteTime 이 한 배치로 같은지** 본다 → 그게 체인이 돈 시각이다.
2. `hooks.txt` · `d19_imm.txt`(둘 다 **로드 시점** 산출물)의 시각과 비교한다.
   이들이 `*_imm.txt` 배치보다 **나중**이면 그 뒤에 새 프로세스가 떴다는 뜻 —
   `imm_guard_summary.txt` 는 그 새 프로세스 것이므로 **체인 판정에 쓰면 안 된다**.
3. 실제 판정은 각 묶음의 `applied=N/M` 으로 한다. 파일이 갱신됐고 N 이 정상이면 그 판은 유효하다.

`checked=756` 은 "그 프로세스에서 체인이 돌았다"는 뜻으로만 읽는다.''', 'checked 규칙')

sub('00_흐름도.md',
'''★**확인 순서**: `imm_guard_summary.txt`의 `checked=`부터 본다.
**756이면 유효한 판, 10이면 그 판의 노브 결과는 전부 무효**다 — `applied` 숫자를 읽기 전에 이것부터.''',
'''★**확인 순서** — ⚠`checked=` 숫자만 보면 안 된다.

`imm_guard_summary.txt` 는 **가장 최근에 뜬 프로세스**의 값이다. 설정값이 정상 적용된 뒤 게임을 한 번 더 켜면
그 짧은 프로세스가 이 파일을 `checked=10` 으로 덮어써서, 멀쩡한 판을 "무효"로 오독하게 만든다.

**먼저 `*_imm.txt` 들의 수정 시각이 한 배치로 같은지** 보고, 그 시각이 `hooks.txt`·`d19_imm.txt`
(로드 시점 산출물)보다 **나중**인지 확인한다. 로드 산출물이 더 나중이면 그 뒤에 게임이 다시 뜬 것이므로
`imm_guard_summary.txt` 는 다른 판의 값이다. 판정은 각 묶음의 `applied=N/M` 으로 한다.''', '흐름도 규칙')

sub('00_흐름도.md',
'''| 전 노브가 무반응 | 그 판에 후퇴 판단이 안 떴다 | `checked=10` |''',
'''| 전 노브가 무반응 | 그 판에 후퇴 판단이 안 떴다 | `*_imm.txt` 들이 **갱신 자체가 안 됨**(시각이 옛날) |
| 무반응처럼 보이지만 아님 | 뒤에 게임이 한 번 더 떠서 guard 파일만 덮인 것 | `checked=10` 인데 `*_imm.txt` 는 정상 |''', '증상표')

print('\n정정 %d건' % n)
