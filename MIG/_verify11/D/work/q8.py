import json
V2=json.load(open('_spec/specs20.json',encoding='utf-8'))['specs']
for i in (15,16,17,18,19):
    print('==',i)
    for p in V2[i]['signature'].get('params') or []:
        print('  ',{k:(str(v)[:40]) for k,v in p.items()})
