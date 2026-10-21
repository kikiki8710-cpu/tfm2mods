# 0.6.5 저신뢰·실패 엔트리 목록 (수동 판정 대상) — 0.6.4 _review_064.txt 형식
import json,collections
m=json.load(open(r'C:\tfm2mods\MIG\repin_map_065.aligned.json',encoding='utf-8'))
rows=[];cnt=collections.Counter()
for mod,ents in m.items():
    for old,e in ents.items():
        k=e.get('kind','');al=(e.get('align') or {}).get('verdict','')
        strong = (k.startswith('BYTES_UNIQUE') or k.startswith('FN_UNIQUE')) and al in ('AGREE','')
        inline = e['name'].startswith('INLINE@')
        if strong and not inline: continue
        if al=='AGREE' and not inline and 'CG_' not in k and k!='FAIL': continue
        why='FAIL' if k=='FAIL' else ('INLINE' if inline else ('CG' if 'CG_' in k else 'ALIGN_'+(al or 'NONE')))
        cnt[why]+=1
        rows.append('%-24s %-36s %10s -> %10s  %-28s align=%s aligned=%s loc=%s'%(mod,e['name'],old,e.get('new'),k,al,e.get('new_aligned'),e.get('locs')))
open(r'C:\tfm2mods\MIG\_review_065.txt','w',encoding='utf-8').write('\n'.join(rows)+'\n')
print(len(rows),dict(cnt))
