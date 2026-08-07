# -*- coding: utf-8 -*-
"""원래 테스트C(=Downloads 원본 cfg)에서 **기본값이 아니었던 키**만 뽑아 원본 → 설정 을 보인다.
   기준 원본값 = 편집기 orig_val 맵(472/475 채워짐). -1 은 '원본 유지'라 기본값으로 친다."""
import sys, io, re, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

st = json.load(io.open('C:/tfm2mods/v54/audit_state.json', encoding='utf-8'))
orig, desc, in_tab = st['orig'], st['desc'], set(st['in_tab'])

# 편집기 orig_val 은 비숫자(켜짐/꺼짐 등)도 있어 원문 그대로 다시 읽는다
ed = io.open('C:/tfm2mods/ai_adjust_editor/src/main.rs', encoding='utf-8').read()
m = re.search(r'\nfn orig_val\(', ed)
nx = re.search(r'\nfn \w+\(', ed[m.end():])
seg = ed[m.start(): m.end() + (nx.start() if nx else 0)]
ORIG = {}
for mm in re.finditer(r'"([a-z][a-zA-Z0-9_]{2,})"\s*=>\s*"([^"]*)"', seg):
    ORIG.setdefault(mm.group(1), mm.group(2))
# or-패턴 `"a" | "b" | "c" => "120"`
for mm in re.finditer(r'((?:"[a-z][a-zA-Z0-9_]{2,}"\s*\|\s*)+"[a-z][a-zA-Z0-9_]{2,}")\s*=>\s*"([^"]*)"', seg):
    for k in re.findall(r'"([^"]+)"', mm.group(1)):
        ORIG.setdefault(k, mm.group(2))

SRC = r'C:\Users\dev\Downloads\tfm2_ai_adjust\tfm2_ai_adjust.cfg'
CLS = re.compile(r'_class_(melee|range|magician|util|assassin)$')

glob_diff, cls_over, unknown, same = [], [], [], 0
for ln in io.open(SRC, encoding='utf-8').read().split('\n'):
    s = ln.strip()
    if not s or s.startswith('#') or '=' not in s:
        continue
    k, v = [x.strip() for x in s.split('=', 1)]
    if k.startswith('__'):
        continue
    if CLS.search(k):
        cls_over.append((k, v)); continue
    if v == '-1':
        same += 1; continue
    o = ORIG.get(k)
    if o is None:
        unknown.append((k, v)); continue
    if o == v:
        same += 1; continue
    glob_diff.append((k, o, v))

print('원래 테스트C 활성 키 중')
print('  기본값과 다름      = %d' % len(glob_diff))
print('  클래스별 오버라이드 = %d' % len(cls_over))
print('  기본값과 같음/-1    = %d' % same)
print('  원본값 미상        = %d' % len(unknown))

print('\n== 기본값과 다른 전역 설정 ==')
print('%-26s %14s → %-14s %s' % ('키', '원본', '설정', '한 줄 설명'))
for k, o, v in glob_diff:
    d = re.sub(r'<[^>]+>|\*\*', '', desc.get(k, ''))
    d = re.sub(r'\s+', ' ', d)[:46]
    print('%-26s %14s → %-14s %s' % (k, o, v, d))

if cls_over:
    print('\n== 클래스별 오버라이드(전역과 별개로 그 클래스에만 적용) ==')
    for k, v in cls_over:
        base = CLS.sub('', k)
        print('  %-34s = %-10s (전역 원본 %s)' % (k, v, ORIG.get(base, '?')))

if unknown:
    print('\n== 원본값을 몰라 비교 못 한 것 ==')
    for k, v in unknown:
        print('  %-26s = %s' % (k, v))
