import json
V2=json.load(open('_spec/specs20.json',encoding='utf-8'))['specs']
for i in (15,):
    for arr in ('reads','writes'):
        for n,e in enumerate(V2[i].get(arr) or []):
            print(i,arr,n,e.get('base'),e.get('offset'),e.get('name'),'| dir=',e.get('dir'))
