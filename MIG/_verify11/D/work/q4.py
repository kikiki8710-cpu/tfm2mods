import json,sys
i=int(sys.argv[1]); arr=sys.argv[2]
V3=json.load(open('_spec/specs20_v3.json',encoding='utf-8'))['specs']
for n,e in enumerate(V3[i][arr]):
    print('---',n)
    print(json.dumps(e,ensure_ascii=False,indent=1)[:2500])
