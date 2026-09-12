import json,sys
V3=json.load(open('_spec/specs20_v3.json',encoding='utf-8'))
V2=json.load(open('_spec/specs20.json',encoding='utf-8'))
def get(d):
    return d['specs'] if isinstance(d,dict) and 'specs' in d else d
s3=get(V3); s2=get(V2)
print('v3 type',type(V3), 'len', len(s3))
print('v2 type',type(V2), 'len', len(s2))
print('v3 keys of spec19:', list(s3[19].keys()))
print('v2 keys of spec19:', list(s2[19].keys()))
