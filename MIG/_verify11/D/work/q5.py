import json,sys
V2=json.load(open('_spec/specs20.json',encoding='utf-8'))['specs']
i=int(sys.argv[1])
for arr in ('reads','writes','constants','knobs','new_knobs'):
    a=V2[i].get(arr) or []
    if a: print(arr, json.dumps(a[0],ensure_ascii=False)[:600]); print('   keys:',sorted(a[0].keys()))
