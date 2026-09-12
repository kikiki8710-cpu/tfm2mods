import json
V3=json.load(open('_spec/specs20_v3.json',encoding='utf-8'))['specs']
V2=json.load(open('_spec/specs20.json',encoding='utf-8'))['specs']
for i in (15,16,17,18,19):
    a=V3[i]; b=V2[i]
    print('='*20,i,a['name'])
    print(' v3: mem',len(a['mem']),'consts',len(a['consts']),'knobs',len(a['knobs']),'closed',len(a['closed']),'open',len(a['open']),'notes',len(a['notes']),'callees',len(a['callees']),'siblings',len(a.get('siblings') or []),'callers',len(a.get('callers') or []))
    print(' v2: reads',len(b['reads']),'writes',len(b.get('writes') or []),'constants',len(b['constants']),'knobs',len(b['knobs']),'new_knobs',len(b.get('new_knobs') or []),'unknown',len(b.get('unknown') or []),'still_unknown',len(b.get('still_unknown') or []),'resolved',len(b.get('resolved') or []))
