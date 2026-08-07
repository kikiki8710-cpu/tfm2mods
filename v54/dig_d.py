# -*- coding: utf-8 -*-
"""D군(설명 없는 은닉 노브) 49개의 정체를 소스에서 캐낸다.
   각 키마다: 읽는 파일·줄 / 감싸는 함수 / 저장처 static / 줄끝 주석 / 사이트 수."""
import sys, io, re, os, glob, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

D = json.load(io.open('C:/tfm2mods/v54/hidden_class.json', encoding='utf-8'))['D_설명없음_보류']
SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'

files = {}
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    files[os.path.basename(f)] = io.open(f, encoding='utf-8').read().split('\n')

FN = re.compile(r'^\s*(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+(\w+)')

out = []
for k in D:
    hits = []
    for fname, lines in files.items():
        for i, ln in enumerate(lines):
            if '"%s"' % k not in ln:
                continue
            if not re.search(r'tune\(\s*"%s"|"%s"\s*=>' % (re.escape(k), re.escape(k)), ln):
                continue
            fn = ''
            for j in range(i, max(0, i - 400), -1):
                m = FN.match(lines[j])
                if m:
                    fn = m.group(1); break
            cm = ''
            mc = re.search(r'//\s*(.+)$', ln)
            if mc:
                cm = mc.group(1).strip()
            if not cm:                       # 바로 윗줄 주석도 본다
                for j in (i - 1, i - 2):
                    if j >= 0 and lines[j].strip().startswith('//'):
                        cm = lines[j].strip().lstrip('/ ').strip(); break
            st = ''
            ms = re.search(r'(\b[A-Z][A-Z0-9_]{2,})\s*\.\s*store', ln)
            if ms:
                st = ms.group(1)
            hits.append((fname, i + 1, fn, st, cm))
    out.append((k, hits))

for k, hits in out:
    if not hits:
        print('%-22s (읽는 곳을 못 찾음)' % k); continue
    f, ln, fn, st, cm = hits[0]
    tail = ' +%d곳' % (len(hits) - 1) if len(hits) > 1 else ''
    print('%-22s %s:%d  fn=%s%s%s' % (k, f, ln, fn or '?', (' -> ' + st) if st else '', tail))
    if cm:
        print('%24s%s' % ('', cm[:150]))
