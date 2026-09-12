#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""mkmap_html.py — `TFM2 AI 함수 지도` 아티팩트에 **20함수 상세 명세**를 붙인다.

입력:
  ① 기존 지도 HTML(아티팩트 저장본) — 641개 함수 · 호출관계 · 근거표기
  ② `_spec/specs20.json` — 20함수 상세 명세(1~6차 + 09-10 미확정 해소)

붙이는 방식:
  - 지도의 (모듈, 이름)으로 조인되는 15개는 **그 함수 상세 패널에** 명세를 펼친다.
  - 조인이 안 되는 5개는 `spec-<id>` 가짜 주소로 rail 에 별도 모듈(`명세만`)로 올린다.
    (exe 에서 인라인됐거나 지도 선별 밖 — 각각 사유를 패널에 적는다.)
  - 필터 칩 `상세 명세` 를 추가한다.

⚠원본 IIFE 를 앵커 문자열로 수술한다. 앵커가 하나라도 안 맞으면 즉시 죽는다(조용한
  누락 방지). 앵커는 전부 `assert` 로 존재를 확인한다.
"""
import io
import json
import os
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

HERE = os.path.dirname(os.path.abspath(__file__))
# 2026-09-13: 기반 = `fnmap_update.py` 가 현행 사실(RVA 정정·ev1)로 갱신한 사본. 원본 아티팩트는
#   `_spec\fnmap_base_2026-09-10.html` 로 보존(옛 SRC 는 세션 tool-results 휘발 경로였다).
SRC = os.path.join(HERE, '_spec', 'fnmap_base_current.html')
SPECS = os.path.join(HERE, '_spec', 'specs20.json')
# 2026-09-13: 출력 = REPORT 정본(master). 옛 DST(silerus worktree)는 stale.
DST = os.path.join(r'C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_ai_adjust', u'AI함수지도.html')

CSS = r'''
/* ── 상세 명세 패널 (2026-09-10 추가) ───────────────── */
.spec{border-top:1px solid var(--line-soft)}
.spec-hd{padding:12px 18px; background:var(--new-soft); border-bottom:1px solid var(--new-line);
  display:flex; align-items:center; justify-content:space-between; gap:10px; flex-wrap:wrap}
.spec-hd h3{margin:0; font-size:12px; letter-spacing:.08em; text-transform:uppercase; color:var(--new); font-weight:700}
.spec-hd .sub{font-size:12px; color:var(--ink-2)}
.spec-one{padding:13px 18px; font-size:14px; color:var(--ink); border-bottom:1px solid var(--line-soft)}
.spec-one b{color:var(--new)}
.spec sec{display:block; border-bottom:1px solid var(--line-soft); padding:12px 18px}
.spec sec:last-child{border-bottom:0}
.spec sec > h4{margin:0 0 9px; font-size:10.5px; letter-spacing:.08em; text-transform:uppercase;
  color:var(--muted); font-weight:600; display:flex; gap:8px; align-items:baseline}
.spec sec > h4 .num{color:var(--ink-2); font-family:var(--mono)}
pre.logic{margin:0; font-family:var(--mono); font-size:12px; line-height:1.55; white-space:pre;
  overflow-x:auto; background:var(--sunk); border:1px solid var(--line-soft); border-radius:6px; padding:12px 14px}
table.st{width:100%; border-collapse:collapse; font-size:12.5px}
table.st th{text-align:left; font-size:10.5px; letter-spacing:.06em; text-transform:uppercase;
  color:var(--muted); font-weight:600; padding:0 10px 6px 0; border-bottom:1px solid var(--line-soft)}
table.st td{padding:6px 10px 6px 0; border-bottom:1px solid var(--line-soft); vertical-align:top}
table.st tr:last-child td{border-bottom:0}
table.st td.k{font-family:var(--mono); white-space:nowrap; color:var(--accent); font-variant-numeric:tabular-nums}
table.st td.w{font-family:var(--mono); font-size:11.5px; color:var(--muted); white-space:nowrap}
.tblwrap{overflow-x:auto}
ul.plain{list-style:none; margin:0; padding:0; display:flex; flex-direction:column; gap:7px; font-size:13px}
ul.plain li{padding-left:14px; position:relative; color:var(--ink-2)}
ul.plain li::before{content:"–"; position:absolute; left:0; color:var(--muted)}
ul.plain.done li::before{content:"✓"; color:var(--new)}
ul.plain code, .spec code{font-family:var(--mono); font-size:11.5px; background:var(--sunk); padding:1px 5px; border-radius:3px}
.pill{font-family:var(--mono); font-size:10.5px; padding:2px 7px; border-radius:3px; white-space:nowrap;
  border:1px solid var(--new-line); background:var(--new-soft); color:var(--new)}
.pill.warn{border-color:var(--live-line); background:var(--live-soft); color:var(--live)}
.spec details{border:1px solid var(--line-soft); border-radius:6px; background:var(--surface-2)}
.spec details > summary{cursor:pointer; padding:8px 12px; font-size:12.5px; color:var(--ink-2); list-style:none}
.spec details > summary::-webkit-details-marker{display:none}
.spec details > summary::before{content:"▸ "; color:var(--muted)}
.spec details[open] > summary::before{content:"▾ "}
.spec details > div{padding:0 12px 12px}
.nomap{padding:13px 18px; background:var(--live-soft); border-left:3px solid var(--live);
  border-bottom:1px solid var(--line-soft); font-size:13px; color:var(--ink-2)}
'''

JS_HELPERS = r'''
  /* ── 상세 명세 (2026-09-10 추가) ─────────────────────
     20함수의 IR 독해 결과. 지도의 (모듈,이름)으로 15개가 붙고,
     나머지 5개는 `spec-` 가짜 주소로 rail 에 따로 올라간다. */
  var SPECDOC = JSON.parse(document.getElementById('specdata').textContent);
  var SPEC = {};                      /* 'module__name' -> spec */
  SPECDOC.specs.forEach(function(s){
    if(s.exe) SPEC[s.exe.module + '__' + s.exe.name0] = s;
  });
  function specOf(d){
    if(d.spec) return d.spec;
    return SPEC[d.m + '__' + d.n] || null;
  }
  function esc2(s){ return esc(s == null ? '' : s); }
  function mono(s){ return '<code>'+esc2(s)+'</code>'; }
  /* 본문에 `백틱` 이 있으면 코드로 렌더 — 명세 원문이 그 표기를 쓴다 */
  function rich(s){
    return esc2(s).replace(/`([^`]+)`/g, '<code>$1</code>')
                  .replace(/\*\*([^*]+)\*\*/g, '<b>$1</b>')
                  .replace(/\n/g, '<br>');
  }
  function secBlock(title, n, body){
    if(!body) return '';
    return '<sec><h4>'+title+(n!=null?' <span class="num">'+n+'</span>':'')+'</h4>'+body+'</sec>';
  }
  function tbl(head, rows){
    if(!rows.length) return '';
    return '<div class="tblwrap"><table class="st"><tr>'+
      head.map(function(h){return '<th>'+h+'</th>';}).join('')+'</tr>'+
      rows.join('')+'</table></div>';
  }

  function drawSpec(d){
    var el = document.getElementById('dSpec');
    var s = specOf(d);
    if(!s){ el.innerHTML = ''; return; }
    var P = [];
    P.push('<div class="spec">');
    P.push('<div class="spec-hd"><h3>상세 명세</h3><span class="sub">'+
      esc2(s.layer)+' · '+esc2(s.src)+':'+esc2(s.src_line)+' · IR '+
      esc2(s.ir.file)+':'+s.ir.frm+'~'+s.ir.to+' · '+s.rounds+'회 독립 재작성</span></div>');
    if(!s.exe && s.no_map_reason)
      P.push('<div class="nomap"><b>지도에 대응 항목이 없다.</b> '+rich(s.no_map_reason)+'</div>');
    /* ⚠(모듈,이름)이 exe 에 여러 곳이면 어느 쪽 명세인지 단정하면 안 된다.
       `hunt_and_poke::is_end` 는 Epic 판과 Serpen 판 둘이다 — 명세는 Epic 것이다. */
    if(s.exe && s.exe.ambiguous)
      P.push('<div class="nomap"><b>⚠ 이 이름은 exe 에 '+s.exe.ambiguous.length+'곳이다</b> ('+
        s.exe.ambiguous.map(function(a){return '0x140'+a;}).join(' / ')+
        '). 아래 명세는 <b>'+esc2(s.src)+'</b> 기준이고, <b>어느 주소가 그것인지는 확인하지 않았다.</b> '+
        '다른 한 쪽은 동명 다른 타입(예: Serpen 판)일 수 있다 — 주소를 그대로 믿고 패치하지 마라.</div>');
    P.push('<div class="spec-one"><b>요지</b> — '+rich(s.one_line)+'</div>');

    /* 판정 로직 */
    if(s.logic) P.push(secBlock('판정 로직', null, '<pre class="logic">'+esc2(s.logic)+'</pre>'));

    /* 노브 */
    var kn = (s.knobs||[]).map(function(k){
      if(typeof k !== 'object') return '<tr><td colspan="3">'+rich(k)+'</td></tr>';
      return '<tr><td>'+rich(k.what)+'</td><td class="k">'+esc2(k.value)+'</td>'+
        '<td>'+rich(k.effect||'')+'<div class="w">'+esc2(k.where)+'</div></td></tr>';
    });
    (s.new_knobs||[]).forEach(function(k){
      kn.push('<tr><td><span class="pill">신규</span> '+rich(k.what)+'</td><td class="k">'+
        esc2(k.value==null?'—':k.value)+'</td><td>'+rich(k.effect||'')+
        '<div class="w">'+esc2(k.where)+'</div></td></tr>');
    });
    P.push(secBlock('바꿀 수 있는 값', kn.length, tbl(['무엇','값','바꾸면'], kn)));

    /* 판정 상수 */
    var cs = (s.constants||[]).map(function(c){
      return '<tr><td class="k">'+esc2(c.value)+'</td><td>'+rich(c.meaning||'')+
        (c.folded_from!=null?' <span class="pill warn">접힘 · 원값 '+esc2(c.folded_from)+'</span>':'')+
        '</td><td class="w">'+esc2(c.src_line?('줄 '+c.src_line):'')+'</td></tr>';
    });
    P.push(secBlock('판정 상수', cs.length, tbl(['값','의미','소스'], cs)));

    /* 오프셋 */
    function offRows(arr){
      return (arr||[]).map(function(r){
        if(typeof r !== 'object') return '<tr><td colspan="3">'+rich(r)+'</td></tr>';
        return '<tr><td class="w">'+esc2(r.base)+'</td><td class="k">'+esc2(r.offset)+
          '</td><td>'+mono(r.name)+' '+rich(r.note||'')+'</td></tr>';
      });
    }
    var rd = offRows(s.reads), wr = offRows(s.writes);
    if(rd.length) P.push(secBlock('읽는 필드', rd.length,
      '<details><summary>'+rd.length+'개 — 구조체 · 오프셋 · 필드</summary><div>'+
      tbl(['구조체','오프셋','필드'], rd)+'</div></details>'));
    if(wr.length) P.push(secBlock('쓰는 필드', wr.length,
      '<details><summary>'+wr.length+'개</summary><div>'+tbl(['구조체','오프셋','필드'], wr)+'</div></details>'));

    /* 호출 */
    if((s.calls||[]).length) P.push(secBlock('부르는 함수', s.calls.length,
      '<ul class="plain">'+s.calls.map(function(c){
        return '<li>'+mono(typeof c==='object'?(c.name||''):c)+'</li>'; }).join('')+'</ul>'));

    /* 오늘 해소된 것 */
    if((s.resolved||[]).length) P.push(secBlock('2026-09-10 에 확정된 것', s.resolved.length,
      '<ul class="plain done">'+s.resolved.map(function(r){
        var b = '<b>'+rich(r.was||'')+'</b><br>'+rich(r.now||'');
        Object.keys(r).forEach(function(k){
          if(['was','now','aux'].indexOf(k)>=0) return;
          var v = r[k];
          if(typeof v === 'string') b += '<br><span class="w">'+esc2(k)+'</span> '+rich(v);
        });
        if(r.aux) b += '<br><span class="w">aux</span> '+mono(JSON.stringify(r.aux));
        return '<li>'+b+'</li>'; }).join('')+'</ul>'));

    /* 미확정 */
    var uk = (s.still_unknown||[]).slice();
    if(uk.length) P.push(secBlock('아직 모르는 것 (해소 후 잔여)', uk.length,
      '<ul class="plain">'+uk.map(function(u){return '<li>'+rich(u)+'</li>';}).join('')+'</ul>'));
    if((s.unknown||[]).length) P.push(secBlock('1~6차 원본 미확정 기록', s.unknown.length,
      '<details><summary>'+s.unknown.length+'개 — 상당수는 위에서 해소됐다</summary><div><ul class="plain">'+
      s.unknown.map(function(u){return '<li>'+rich(typeof u==='object'?JSON.stringify(u):u)+'</li>';}).join('')+
      '</ul></div></details>'));

    P.push('</div>');
    el.innerHTML = P.join('');
  }
'''


def main():
    h = io.open(SRC, encoding='utf-8', errors='replace').read()
    doc = json.load(io.open(SPECS, encoding='utf-8'))

    # 지도 조인용 이름을 명시적으로 넣어 둔다(JS 에서 s.exe.name0 로 쓴다).
    for s in doc['specs']:
        if s.get('exe'):
            s['exe']['name0'] = s['name']

    anchors = []

    def sub(old, new, why):
        nonlocal h
        assert h.count(old) == 1, '앵커 %r 이 %d번 나온다 — %s' % (old[:60], h.count(old), why)
        h = h.replace(old, new, 1)
        anchors.append(why)

    # ① CSS
    sub('@media (prefers-reduced-motion:reduce){ *{transition:none !important; animation:none !important} }\n</style>',
        CSS + '\n@media (prefers-reduced-motion:reduce){ *{transition:none !important; animation:none !important} }\n</style>',
        'CSS 추가')

    # ② 명세 데이터 블록
    sub('<script>\n(function(){',
        '<script id="specdata" type="application/json">' +
        json.dumps(doc, ensure_ascii=False, separators=(',', ':')) +
        '</script>\n\n<script>\n(function(){',
        '명세 JSON 삽입')

    # ③ 상세 패널 컨테이너
    sub('''    <dl class="meta" id="dMeta"></dl>
    </div>''',
        '''    <dl class="meta" id="dMeta"></dl>
      <div id="dSpec"></div>
    </div>''',
        '#dSpec 컨테이너')

    # ④ 헬퍼 + 가짜 엔트리
    sub('''  var VIA = {
    fp:{label:'기계어 지문', cls:'v-fp'},''',
        JS_HELPERS + '''
  var VIA = {
    spec:{label:'상세 명세', cls:'v-re'},
    fp:{label:'기계어 지문', cls:'v-fp'},''',
        'JS 헬퍼 + VIA.spec')

    # ⑤ 지도에 없는 5개를 DATA 에 올린다(가짜 주소).
    sub('''  var BY = {}, i;
  for(i=0;i<DATA.length;i++) BY[DATA[i].a] = DATA[i];''',
        '''  var BY = {}, i;
  /* 지도에 대응 항목이 없는 명세(인라인·선별 밖)를 `spec-` 주소로 올린다.
     그래야 rail 에서 고를 수 있다. exe 메타는 없으므로 주소·크기는 비운다. */
  (function(){
    var sd = JSON.parse(document.getElementById('specdata').textContent);
    sd.specs.forEach(function(s){
      if(s.exe) return;
      DATA.push({a:'spec-'+s.id, n:s.name, m:'명세만', b:0, i:0, l:[],
                 v:'spec', f:'', x:null, ai:1, o:[], c:[], spec:s});
    });
  })();
  for(i=0;i<DATA.length;i++) BY[DATA[i].a] = DATA[i];''',
        '가짜 엔트리 5개')

    # ⑥ 주소 표기 — `spec-` 는 0x140 을 붙이면 안 된다
    sub("""'<div class="d-sub"><span class="d-addr mono">0x140'+d.a+'</span>'+""",
        """'<div class="d-sub"><span class="d-addr mono">'+
          (d.a.indexOf('spec-')===0 ? 'exe 미대응' : '0x140'+d.a)+'</span>'+""",
        '가짜 주소 표기 가드')

    # ⑦ 선택 시 명세 렌더
    sub('''    drawEgo(d);
    drawRel(d);''',
        '''    drawSpec(d);
    drawEgo(d);
    drawRel(d);''',
        'select() 에서 drawSpec 호출')

    # ⑧ 필터 칩
    sub("""    '<button class="chip" data-flag="H" aria-pressed="false">후킹 중</button>' +""",
        """    '<button class="chip" data-flag="S" aria-pressed="false">상세 명세 '+
      '<span class="num" style="color:var(--muted)">'+SPECDOC.specs.length+'</span></button>' +
    '<button class="chip" data-flag="H" aria-pressed="false">후킹 중</button>' +""",
        '명세 필터 칩')

    sub("""    var flags = ['H','D','O'].filter(function(f){ return active['flag:'+f]; });
    for(var j=0;j<flags.length;j++) if(d.f.indexOf(flags[j])<0) return false;""",
        """    if(active['flag:S'] && !specOf(d)) return false;
    var flags = ['H','D','O'].filter(function(f){ return active['flag:'+f]; });
    for(var j=0;j<flags.length;j++) if(d.f.indexOf(flags[j])<0) return false;""",
        '명세 필터 판정')

    # ⑨ 통계 한 칸
    sub("""    [fmt(reN.length),'수기·디컴 확정']
  ];""",
        """    [fmt(reN.length),'수기·디컴 확정'],
    [fmt(SPECDOC.specs.length),'상세 명세']
  ];""",
        '통계 칸 추가')

    # ⑩ 부제 갱신
    sub('왼쪽에서 함수를 고르면 그 함수를 부르는 쪽과 그 함수가 부르는 쪽이 펼쳐진다.',
        '왼쪽에서 함수를 고르면 그 함수를 부르는 쪽과 그 함수가 부르는 쪽이 펼쳐진다. '
        '<b>20개</b>에는 IR 을 직접 읽어 만든 <b>상세 명세</b>(판정 로직·바꿀 수 있는 값·오프셋·아직 모르는 것)가 붙어 있다 — '
        '필터의 <b>상세 명세</b> 칩으로 추릴 수 있다.',
        '부제 갱신')

    io.open(DST, 'w', encoding='utf-8', newline='').write(h)
    print('%s  (%.0f KB)' % (DST, os.path.getsize(DST) / 1024.0))
    for a in anchors:
        print('  OK  ' + a)


if __name__ == '__main__':
    main()
