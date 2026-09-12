import json,sys
V2=json.load(open('_spec/specs20.json',encoding='utf-8'))['specs']
i=int(sys.argv[1])
print(V2[i]['logic'])
