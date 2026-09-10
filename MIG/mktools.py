#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""mktools.py — `MIG\\TOOLS.md`(도구 인벤토리)를 **자동 재생성**한다.

왜 자동인가: 도구가 계속 늘어난다(2026-09-11 현재 102개). 손으로 적은 목록은
반드시 썩는다 — 실제로 `IR_TOOLKIT §4` 는 7개만 적고 있었다.
여기서는 **파일의 첫 docstring 줄을 그대로 끌어와** 표를 만든다.
⟹ 도구를 만들 때 **docstring 첫 줄만 제대로 쓰면** 문서가 알아서 최신이 된다.

★분류에 없는 새 도구는 「미분류」 절에 자동으로 뜬다. 거기 보이면 CAT 에 한 줄 추가.

사용:  python -X utf8 mktools.py         # TOOLS.md 재생성
       python -X utf8 mktools.py --check # 미분류만 보고(쓰지 않음)
"""
import io
import os
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

HERE = os.path.dirname(os.path.abspath(__file__))
DST = os.path.join(HERE, 'TOOLS.md')

# 분류 = (제목, 한 줄 설명, [파일명...])  — 순서가 곧 문서 순서
CAT = [
 ('★사전 — 타입·오프셋을 묻는 곳',
  '**여기부터 친다.** DWARF 를 손으로 타지 마라.',
  ['tcxdict.py', 'tcxaudit.py', 'tcxverify.py', 'tcxcross.py', 'tcxfield.py', 'tcxq.py',
   'tcxpub.py', 'tcxbuild.ps1', 'tcxrun.ps1', 'divtable.py', 'distruct.py', 'dienum.py']),
 ('재료 추출 — rlib 에서 꺼내기',
  'IR·rmeta 를 뽑고 훑는다.',
  ['rlib2ll.py', 'bundlegrep.py', 'rmetadocs.py', 'rmeta_srcmap.py', 'rmetagrep.py', 'rmetadump.py',
   'dqctx.py', 'dqraw.py', 'rmeta_probe1.py', 'rmeta_probe2.py']),
 ('IR 독해 — 본문을 읽을 때',
  '함수 조각 찾기 → 줄번호 잇기 → 값·조건 추적.',
  ['fnparts.py', 'dloc.py', 'dbgchain.py', 'llann.py', 'srcmap.py', 'irann.py',
   'ann.py', 'ann2.py', 'ann3.py', 'irfn.py', 'irctx.py', 'dl.py',
   'guard.py', 'inlsites.py', 'vbr.py',
   'fieldall2.py', 'fieldall.py', 'fieldcodes.py', 'knobscan.py',
   'cachedfns.py', 'cachekeys.py', 'memofns.py', 'tlsscan.py', 'coreglobals.py']),
 ('★오라클 — SDK 함수를 진짜 실행',
  '`-C lto=fat` 로 rlib 이 exe 로 링크된다. `pub` 이면 **진리표로 검증**할 수 있다. 전문 = `IR_TOOLKIT §7`.',
  ['spanprobe.ps1', 'rdocprobe.ps1', 'probe.py', 'aiprobe.py']),
 ('명세 파이프라인 — 배치를 굴릴 때',
  '절차 = `SPEC_RUNBOOK.md`. 규격 = `SPEC_GUIDE.md`.',
  ['pick20.py', 'qcspec.py', 'speccmp.py', 'mkspec20.py', 'mkspec_md.py', 'mkmap_html.py',
   'corpus.py', 'corpus2.py']),
 ('exe ↔ IR 잇기 — 이름·주소 붙이기',
  'IR 의 이름을 exe RVA 에 잇거나, exe 함수에 이름을 붙인다.',
  ['name2rva.py', 'panicloc.py', 'srcident.py', 'typeid_map.py', 'irskew.py', 'irskew2.py',
   'dllmatch.py', 'dmcheck.py', 'dupmatch.py', 'percolate.py',
   'cg.py', 'callgraph.py', 'callees.py', 'callcount.py',
   'aibound.py', 'aiclass.py', 'aiscope.py', 'ainoret.py', 'aimap.py',
   'ghidra_syms.py', 'ghidra_inject.py', 'ghidra_cycle.ps1']),
 ('마이그레이션 — 패치가 왔을 때',
  '진입점은 `run.py`. 상세 = `MODS\\MIGRATION.md`.',
  ['run.py', 'mig_verify.py', 'repin.py', 'midpin.py', 'sitepin.py', 'fncheck.py',
   'chain.py', 'offsets.py', 'env.py', 'posdiff.py', 'apply_manual.py',
   'bump_deps.py', 'aidiff.py']),
 ('재현·포팅 — judge 계층 만들 때',
  '',
  ['aiport.py', 'aidump.py', 'aifill.py', 'aifix.py', 'ailink.py', 'gensweep.py',
   'agent_digest_cmp.py']),
 ('운영 — 빌드·크래시·로그',
  '',
  ['mktools.py', 'logsnap.py', 'modbisect.py', 'apgate.py']),
]

# 쓰지 말아야 할 것 (사유를 문서에 박는다)
DEPRECATED = {
 'fieldcodes.py': '⛔**오염** — gep 결과 레지스터를 **파일 전역**으로 매칭해 다른 함수의 동명 `%N` 을 잡는다. 후속 정본 = `fieldall2.py`',
 'srcident.py': '⛔STALE(2026-09-09 · 0.5.8) — 판정 정본 = `panicloc.py`',
 'irskew.py': '⛔폐기 — exe 는 상수가 접혀 소스 줄로 못 되돌린다. 후속 = `irskew2.py`(필드 오프셋 지문)',
 'distruct.py': '⚠**2차 폴백** — 키가 leaf 이름 1개뿐이라 동명끼리 조용히 덮어쓴다. 1차 = `tcxdict.py`',
 'dienum.py': '⚠**2차 폴백** — tcx 와 모순 0이나 커버리지 39.7% 결손. 1차 = `tcxdict.py --enum`',
}


def first_doc(path):
    try:
        t = io.open(path, encoding='utf-8', errors='replace').read(1500)
    except Exception:
        return ''
    m = re.search(r'"""(.{0,300}?)(?:\n|""")', t, re.S)
    if m:
        return m.group(1).strip()
    if path.endswith('.ps1'):
        for l in t.split('\n')[:8]:
            if l.strip().startswith('#'):
                return l.strip('# ').strip()
    return ''


def main():
    files = sorted(f for f in os.listdir(HERE)
                   if f.endswith(('.py', '.ps1')) and not f.startswith('_'))
    known = set()
    for _, _, fs in CAT:
        known |= set(fs)
    unknown = [f for f in files if f not in known]

    if '--check' in sys.argv:
        print('전체 %d · 분류됨 %d · 미분류 %d' % (len(files), len(files) - len(unknown), len(unknown)))
        for f in unknown:
            print('  미분류  %-24s %s' % (f, first_doc(os.path.join(HERE, f))[:90]))
        return

    o = []
    o.append('# MIG 도구 인벤토리')
    o.append('')
    o.append('> ★**이 파일은 `mktools.py` 가 생성한다. 손으로 고치지 마라** — 다음 재생성에서 날아간다.')
    o.append('> 도구를 만들면 **파일 첫 docstring 줄**을 제대로 쓰고 `python -X utf8 mktools.py` 를 돌려라.')
    o.append('> 분류에 없는 새 도구는 맨 아래 **미분류** 절에 자동으로 뜬다(거기 보이면 `mktools.py` 의 `CAT` 에 한 줄 추가).')
    o.append('')
    o.append('**어떤 상황에 무엇을 집는가 = `METHOD_MAP.md` §0**(라우팅표). 이 문서는 *무엇이 있는가* 만 센다.')
    o.append('')
    o.append('전체 %d개 · 생성 시각 기준 자동 집계 (`_` 로 시작하는 1회용 스크래치는 제외)' % len(files))
    o.append('')
    for title, note, fs in CAT:
        present = [f for f in fs if f in files]
        if not present:
            continue
        o.append('## %s' % title)
        if note:
            o.append('')
            o.append(note)
        o.append('')
        o.append('| 도구 | 하는 일 |')
        o.append('|---|---|')
        for f in present:
            doc = first_doc(os.path.join(HERE, f))
            doc = re.sub(r'^%s\s*[-—]\s*' % re.escape(f), '', doc).strip() or '(docstring 없음 — 채워라)'
            doc = doc.replace('|', '\\|')
            dep = DEPRECATED.get(f)
            if dep:
                doc = dep + ' · ' + doc
            o.append('| `%s` | %s |' % (f, doc))
        o.append('')
    if unknown:
        o.append('## 미분류 (새로 생긴 도구 — `mktools.py` 의 `CAT` 에 넣어라)')
        o.append('')
        o.append('| 도구 | 하는 일 |')
        o.append('|---|---|')
        for f in unknown:
            doc = first_doc(os.path.join(HERE, f)) or '(docstring 없음 — 채워라)'
            o.append('| `%s` | %s |' % (f, doc.replace('|', '\\|')))
        o.append('')
    io.open(DST, 'w', encoding='utf-8').write('\n'.join(o))
    print('%s  (%d개 · 미분류 %d)' % (DST, len(files), len(unknown)))


if __name__ == '__main__':
    main()
