import json, io, sys, re
crates = ['game_ai', 'game_core']
pat = re.compile(sys.argv[1], re.I)
want = sys.argv[2] if len(sys.argv) > 2 else None
for c in crates:
    d = json.load(io.open('_tcx/%s.json' % c, encoding='utf-8'))
    for x in d['items']:
        p = x.get('p', '')
        if not pat.search(p):
            continue
        if want and want not in x.get('k', ''):
            continue
        sp = x.get('sp') or {}
        print('%-9s %-14s %-9s mir=%-5s %s:%s  %s' % (
            c, x.get('k'), x.get('v'), x.get('mir'), sp.get('f', '?'), sp.get('l', '?'), p))
        if x.get('sig'):
            print('        sig: %s' % x['sig'])
