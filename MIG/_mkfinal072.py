# 0.7.2 수동 판정 반영 → repin_map_072.final.json
# 근거 = MIG\_patchday\0.7.2\judge_*.json (2026-11-11 판정 에이전트 5갈래, RE 원문 = REPORT\<MOD>\RE\2026-11-11_0.7.2-재핀판정*.md)
# 판정 JSON 은 모드 엔트리 전부(엔진값 채택분 포함)를 담으므로 모든 엔트리의 new 를 판정값으로 덮는다.
import glob, json, os
HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, 'repin_map_072.aligned.json')
DST = os.path.join(HERE, 'repin_map_072.final.json')
SKIP = {'tfm2_detail_tactics', 'tfm2_tactics_verify'}   # 생성물 모드 — repin apply 대상 아님

m = json.load(open(SRC, encoding='utf-8'))
judged = {}
for p in glob.glob(os.path.join(HERE, '_patchday', '0.7.2', 'judge_*.json')):
    for mod, ents in json.load(open(p, encoding='utf-8')).items():
        judged.setdefault(mod, {}).update(ents)

n = changed = 0
missing = []
for mod, ents in m.items():
    if mod in SKIP:
        continue
    jm = judged.get(mod, {})
    for old, e in ents.items():
        if old not in jm:
            missing.append((mod, old, e.get('name')))
            continue
        new, why = jm[old]
        if new is not None:
            new = hex(int(new, 16))
        if e.get('new') != new:
            e['new_repin'] = e.get('new')
            changed += 1
        e['manual'] = why
        if new is None:
            e.pop('new', None)
        else:
            e['new'] = new
        n += 1
extra = [(mod, o) for mod, jm in judged.items() for o in jm if o not in m.get(mod, {})]
# ★0.7.2: 판정이 맵 밖 키를 확정한 경우(check 가 PASS 로 넘겨 repin 대상에서 빠진 엔트리 — serpen LAUNCHER_RVA 는
#   구 주소에 진입 12B 만 같은 다른 함수가 있어 PASS 가 거짓이었다) → 맵에 넣어 apply 가 치환하게 한다.
for mod, o in extra:
    if mod in SKIP:
        continue
    new, why = judged[mod][o]
    row = {'name': '(판정 추가 — check PASS 였으나 판정이 확정)', 'manual': why, 'added_by_judge': True}
    if new is not None:
        row['new'] = hex(int(new, 16))
    m.setdefault(mod, {})[o] = row
json.dump(m, open(DST, 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
print(f'판정 반영 {n} · 엔진값과 달라진 것 {changed} · 판정 누락 {len(missing)} · 맵에 없는 판정 키 {len(extra)} -> {DST}')
for x in missing: print('  누락', x)
for x in extra: print('  맵 밖 판정 키 → 맵에 추가', x, judged[x[0]][x[1]][0])
nulls = [(mod, o, e.get('name')) for mod, ents in m.items() if mod not in SKIP for o, e in ents.items() if not e.get('new')]
for x in nulls: print('  new 없음(null)', x)
