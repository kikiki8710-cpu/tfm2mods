import json
V3=json.load(open('_spec/specs20_v3.json',encoding='utf-8'))['specs']
for i in (15,16,17,18,19):
    a=V3[i]
    print('='*20,i,a['name'])
    print(' siblings:',json.dumps(a.get('siblings'),ensure_ascii=False)[:1500])
    print(' callers:',json.dumps(a.get('callers'),ensure_ascii=False)[:900])
