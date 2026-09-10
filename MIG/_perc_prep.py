#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""_perc_prep.py - percolate.py 가 쓸 **입력 캐시**를 만든다(읽기 전용, 기존 파일 수정 없음).

만드는 것 = `_perc_cache.json`
  nodes_e   : exe 호출그래프 노드(.pdata 경계) rva 목록
  cg_e/rcg_e: exe 정/역 호출그래프
  cg_d/rcg_d: 내 DLL 정/역 호출그래프
  dsym      : 망글심볼 -> [dll_rva, size]   (MAP + .pdata 경계)
  src_of    : 망글심볼 -> 소스모듈           (IR 인덱스)
  seeds     : exe_rva -> {mangled, grade, src}
  fp_e/fp_d : 지문(메모리 오퍼랜드 변위 집합) - 씨앗 이웃 k홉 범위만

씨앗 등급:
  A = manual.json(RE 근거) / ghidra_syms.json(주입 확정)
  B = dllmatch.json 중 **지문 단독**(via != callgraph) & jaccard >= 0.99
  C = 나머지(dllmatch 약한 지문, dllmatch 의 callgraph 전파분, dupmatch 중복군집)
★C 안의 'callgraph' 출신은 **호출그래프로 만들어진 라벨**이라 호출그래프 방법의
  정답지로 쓰면 순환논증이 된다. 별도 플래그(`from_cg`)로 표시해 실험에서 뺄 수 있게 한다.
"""
import io
import json
import os
import sys

import capstone

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import _dllmatch_lib as L  # noqa: E402

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, '_perc_cache.json')
KHOP = 3          # 씨앗에서 몇 홉까지 지문을 뽑아둘지


def jload(p):
    return json.load(io.open(p, encoding='utf-8'))


def rev(g):
    out = {}
    for a, bs in g.items():
        for b in bs:
            out.setdefault(b, []).append(a)
    return out


def main():
    cg_e = {int(k): v for k, v in jload(os.path.join(HERE, 'cg_exe.json')).items()}
    cg_d = {int(k): v for k, v in jload(os.path.join(HERE, 'cg_dll.json')).items()}
    rcg_e, rcg_d = rev(cg_e), rev(cg_d)
    print('exe: 노드 %d 엣지 %d · dll: 노드 %d 엣지 %d'
          % (len(cg_e), sum(len(v) for v in cg_e.values()),
             len(cg_d), sum(len(v) for v in cg_d.values())))

    # ── 내 DLL 심볼표: 망글 -> (rva, size). 크기는 .pdata.
    mt = io.open(L.MAP, encoding='utf-8', errors='ignore').read()
    dll_d, dll_base, dll_secs, dll_exc = L.pe(L.DLL)
    dll_off = L.make_off(dll_secs)
    dll_fr = L.func_ranges(dll_d, dll_secs, dll_exc)
    dsym = {}
    for m in L.MAP_RE.finditer(mt):
        name, va = m.group(1), int(m.group(2), 16)
        rva = va - dll_base
        sz = dll_fr.get(rva)
        if sz is not None:
            dsym[name] = (rva, sz)
    del mt
    src_of, _lines_of = L.ir_index()
    print('MAP 심볼 %d개(.pdata 경계 있는 것) · 그중 rlib 소스 확인 %d개'
          % (len(dsym), sum(1 for n in dsym if n in src_of)))

    rva_of = {n: r for n, (r, _s) in dsym.items() if n in src_of}
    sym_at = {}
    for n, r in rva_of.items():
        sym_at.setdefault(r, n)
    print('dll rva -> 심볼 역맵 %d개 · 그중 호출그래프 노드 %d개'
          % (len(sym_at), sum(1 for r in sym_at if r in cg_d or r in rcg_d)))

    # ── 씨앗 모으기
    seeds = {}

    def put(rva, mang, grade, src, from_cg=False):
        if mang not in rva_of:
            return False       # DLL 쪽 짝이 없으면 그래프 씨앗이 될 수 없다
        old = seeds.get(rva)
        if old and 'ABC'.index(old['grade']) <= 'ABC'.index(grade):
            return False
        seeds[rva] = {'m': mang, 'grade': grade, 'src': src, 'from_cg': from_cg}
        return True

    stat = {}
    for r in jload(os.path.join(HERE, 'ghidra_syms.json')):
        if isinstance(r, dict) and r.get('mangled'):
            stat['ghidra'] = stat.get('ghidra', 0) + put(r['rva'], r['mangled'], 'A', 'ghidra')
    nomanual = 0
    for r in jload(os.path.join(HERE, 'manual.json')):
        if isinstance(r, dict) and r.get('rva', -1) >= 0 and r.get('mangled'):
            if not put(r['rva'], r['mangled'], 'A', 'manual'):
                nomanual += 1
    for r in jload(os.path.join(HERE, 'dllmatch.json')):
        if not isinstance(r, dict) or not r.get('mangled'):
            continue
        cgv = r.get('via') == 'callgraph'
        g = 'B' if (not cgv and float(r.get('jaccard', 0)) >= 0.99) else 'C'
        stat['dllmatch/' + g] = stat.get('dllmatch/' + g, 0) + put(r['rva'], r['mangled'], g,
                                                                  'dllmatch', cgv)
    for r in jload(os.path.join(HERE, 'dupmatch.json')):
        if isinstance(r, dict) and r.get('mangled'):
            stat['dup'] = stat.get('dup', 0) + put(r['rva'], r['mangled'], 'C', 'dupmatch')
    print('씨앗 채택 %d개 %s · manual 중 DLL짝 없어 탈락 %d' % (len(seeds), stat, nomanual))

    innode = sum(1 for r in seeds if r in cg_e or r in rcg_e)
    dnode = sum(1 for v in seeds.values()
                if rva_of[v['m']] in cg_d or rva_of[v['m']] in rcg_d)
    print('  exe 호출그래프에 노드로 있는 씨앗 %d · dll 쪽도 노드인 것 %d' % (innode, dnode))
    both = sum(1 for r, v in seeds.items()
               if (r in cg_e or r in rcg_e) and (rva_of[v['m']] in cg_d or rva_of[v['m']] in rcg_d))
    print('  ★양쪽 다 그래프 노드(=전파에 실제로 쓸 수 있는 씨앗) %d' % both)

    # ── 지문: dll 은 심볼 전부, exe 는 씨앗 k홉 이웃만
    exe_d, exe_base, exe_secs, exe_exc = L.pe(L.EXE)
    exe_off = L.make_off(exe_secs)
    exe_fr = L.func_ranges(exe_d, exe_secs, exe_exc)
    cs = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    cs.detail = True

    fp_d = {}
    for n, r in rva_of.items():
        f = L.fingerprint(cs, dll_d, dll_off, r, dsym[n][1])
        if f:
            fp_d[n] = sorted(f)

    frontier = set(seeds)
    for _ in range(KHOP):
        nxt = set(frontier)
        for x in frontier:
            nxt.update(cg_e.get(x, ()))
            nxt.update(rcg_e.get(x, ()))
        frontier = nxt
    print('exe 지문 대상(씨앗 %d홉 이웃) %d개 - 디스어셈 중...' % (KHOP, len(frontier)))
    fp_e = {}
    for r in frontier:
        sz = exe_fr.get(r)
        if not sz or sz > 0x20000:
            continue
        f = L.fingerprint(cs, exe_d, exe_off, r, sz)
        if f:
            fp_e[r] = sorted(f)
    print('  지문 확보: exe %d · dll %d' % (len(fp_e), len(fp_d)))

    out = {
        'cg_e': {str(k): v for k, v in cg_e.items()},
        'cg_d': {str(k): v for k, v in cg_d.items()},
        'dsym': {k: list(v) for k, v in dsym.items() if k in rva_of},
        'src_of': {k: src_of[k] for k in rva_of},
        'seeds': {str(k): v for k, v in seeds.items()},
        'fp_e': {str(k): v for k, v in fp_e.items()},
        'fp_d': fp_d,
        'size_e': {str(k): exe_fr[k] for k in fp_e},
    }
    with io.open(OUT, 'w', encoding='utf-8') as f:
        json.dump(out, f, separators=(',', ':'))
    print('저장: %s (%.1f MB)' % (OUT, os.path.getsize(OUT) / 1e6))


main()
