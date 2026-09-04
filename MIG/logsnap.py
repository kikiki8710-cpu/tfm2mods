#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""logsnap.py — 인게임 검증 전/후 **모드 로그 스냅샷과 diff**.

왜: "됐나요?"를 눈으로만 판정하면 조용한 실패를 놓친다.
    플레이 **전에** 전 모드 로그를 찍어 두고, **후에** diff 하면
    "어느 모드가 실제로 발화했고, 무엇이 새로 찍혔고, 크래시가 났는가"가 기계로 나온다.
    (0.5.7 회차에 `_pretest_057.txt` 스냅샷 대비 증분으로 판독한 방식을 도구화)

사용
  python MIG\logsnap.py before          # 플레이 직전 — 전 모드 로그 크기·mtime·꼬리 저장
  python MIG\logsnap.py after           # 플레이 후 — 증분만 뽑아 판독용으로 출력
  python MIG\logsnap.py after --full    # 증분 전문(길다)
저장 위치: MIG\logsnap\<before|after>.json  ·  판독 결과는 표준출력
"""
import os, sys, json, argparse, datetime

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
import mig_verify as MV  # noqa: E402

MODS_DIR = os.path.join(os.path.dirname(MV.GAME_EXE), 'mods')
WS = os.path.join('C:', os.sep, 'Program Files (x86)', 'Steam', 'steamapps',
                  'workshop', 'content', '3009300')
OUT = os.path.join(MV.MIGD, 'logsnap')
TAIL = 4000                      # 꼬리 보관 바이트
CRASH_MARK = ('=== CRASH', 'CRASH (', 'panic', 'STATUS_', 'byte mismatch',
              'mismatch', 'SKIP', 'BLOCK', '실패', '불일치', '비활성')


def scan():
    """mods\\ 와 워크샵의 모든 .txt 로그를 수집."""
    out = {}
    roots = [MODS_DIR] + ([WS] if os.path.isdir(WS) else [])
    for root in roots:
        for d in sorted(os.listdir(root)):
            base = os.path.join(root, d)
            if not os.path.isdir(base):
                continue
            for f in sorted(os.listdir(base)):
                if not f.endswith('.txt') or '.bak' in f or f.startswith('_old'):
                    continue
                p = os.path.join(base, f)
                try:
                    st = os.stat(p)
                    raw = open(p, 'rb').read()[-TAIL:]
                except OSError:
                    continue
                out[d + '/' + f] = {'size': st.st_size, 'mtime': st.st_mtime,
                                    'tail': raw.decode('utf-8', 'replace')}
    return out


def cmd_before():
    os.makedirs(OUT, exist_ok=True)
    snap = scan()
    json.dump({'when': datetime.datetime.now().isoformat(timespec='seconds'), 'files': snap},
              open(os.path.join(OUT, 'before.json'), 'w', encoding='utf-8'), ensure_ascii=False)
    print('스냅샷 %d개 파일 저장 (%s)' % (len(snap), os.path.join(OUT, 'before.json')))
    print('→ 이제 게임을 켜고 런북대로 진행하세요. 끝나면 `logsnap.py after`.')


def cmd_after(a):
    bp = os.path.join(OUT, 'before.json')
    if not os.path.isfile(bp):
        print('before.json 없음 — 플레이 전에 `logsnap.py before` 를 먼저 돌려야 한다')
        return 1
    before = json.load(open(bp, encoding='utf-8'))['files']
    now = scan()
    json.dump({'when': datetime.datetime.now().isoformat(timespec='seconds'), 'files': now},
              open(os.path.join(OUT, 'after.json'), 'w', encoding='utf-8'), ensure_ascii=False)

    grew, made, still = [], [], []
    for k, v in sorted(now.items()):
        b = before.get(k)
        if b is None:
            made.append(k)
        elif v['size'] != b['size'] or v['mtime'] > b['mtime'] + 1:
            grew.append(k)
        else:
            still.append(k)

    print('=' * 74)
    print('■ 발화한 모드 (로그가 자란 것) — %d개' % len(grew))
    print('=' * 74)
    for k in grew:
        b = before.get(k, {})
        delta = now[k]['size'] - b.get('size', 0)
        print('  %-52s %+d B' % (k, delta))
    if made:
        print('\n■ 새로 생긴 로그 — %d개' % len(made))
        for k in made:
            print('  %s' % k)
    print('\n■ 안 움직인 로그 — %d개  (그 모드가 이번에 안 돌았거나 죽어 있다)' % len(still))
    for k in still:
        print('  %s' % k)

    print('\n' + '=' * 74)
    print('■ 증분에서 잡힌 위험 신호')
    print('=' * 74)
    hits = 0
    for k in grew + made:
        b = before.get(k, {}).get('tail', '')
        cur = now[k]['tail']
        # 꼬리 기준 증분: 이전 꼬리가 현재 꼬리에 있으면 그 뒤만, 아니면 현재 꼬리 전체
        idx = cur.find(b[-200:]) if len(b) >= 200 else -1
        inc = cur[idx + 200:] if idx >= 0 else cur
        bad = [l.strip() for l in inc.split('\n')
               if any(m in l for m in CRASH_MARK) and l.strip()]
        if bad:
            hits += len(bad)
            print('\n[%s]' % k)
            for l in bad[:12]:
                print('   %s' % l[:150])
        elif a.full and inc.strip():
            print('\n[%s] (증분 %d자)' % (k, len(inc)))
            for l in inc.strip().split('\n')[-8:]:
                print('   %s' % l[:150])
    if not hits:
        print('  (없음)')
    print('\n판독 요령: "안 움직인 로그" 에 그 회차에 돌았어야 할 모드가 있으면 조용한 실패다.')
    return 0


if __name__ == '__main__':
    ap = argparse.ArgumentParser()
    ap.add_argument('cmd', choices=['before', 'after'])
    ap.add_argument('--full', action='store_true', help='증분 전문 출력')
    a = ap.parse_args()
    if a.cmd == 'before':
        cmd_before()
    else:
        sys.exit(cmd_after(a))
