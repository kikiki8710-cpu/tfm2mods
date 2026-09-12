import json, io
d = json.load(io.open(r'C:\tfm2mods\MIG\_spec\specs20_v3.json', encoding='utf-8'))
for i in range(15, 20):
    s = d['specs'][i]
    print('=== %d %s ===' % (i, s['name']))
    for arr in ('mem', 'consts', 'knobs'):
        for j, r in enumerate(s.get(arr, [])):
            if (r.get('ev') or 0) < 4:
                continue
            if arr == 'mem':
                desc = '%s +%s %s dir=%s' % (r.get('base'), r.get('offset'), r.get('name'), r.get('dir'))
            elif arr == 'consts':
                desc = 'val=%s line=%s kind=%s :: %s' % (r.get('value'), r.get('src_line'), r.get('kind'), (r.get('meaning') or '')[:90])
            else:
                desc = '%s | where=%s | val=%s | src=%s' % (r.get('what'), r.get('where'), r.get('value'), r.get('src'))
            print('  /specs[%d]/%s[%d]  ev%s  %s' % (i, arr, j, r.get('ev'), desc))
