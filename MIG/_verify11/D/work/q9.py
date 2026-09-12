import json
D=json.load(open('_spec/specs20.json',encoding='utf-8'))['specs']
def show(i,key,idx=None,sub=None):
    v=D[i][key]
    if idx is not None: v=v[idx]
    if sub: v=v[sub]
    print('###',i,key,idx,sub); print(v); print()
for i,arr in ((15,'unknown'),(16,'unknown'),(17,'unknown'),(18,'unknown')):
    for n,t in enumerate(D[i].get(arr) or []):
        print('==',i,arr,n,'::',t[:110].replace('\n',' '))
print()
show(17,'knobs',1,'effect')
show(19,'constants',4,'meaning')
show(18,'writes',10,'note')
