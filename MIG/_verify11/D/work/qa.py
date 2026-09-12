import json
D=json.load(open('_spec/specs20.json',encoding='utf-8'))['specs']
print('### 18 writes[4] (v3 mem[16])'); print(json.dumps(D[18]['writes'][4],ensure_ascii=False,indent=1))
print('### 18 writes[3] (v3 mem[15])'); print(json.dumps(D[18]['writes'][3],ensure_ascii=False,indent=1)[:700])
print('### 16 knobs[3]'); print(json.dumps(D[16]['knobs'][3],ensure_ascii=False,indent=1)[:900])
print('### 16 unknown[3]'); print(D[16]['unknown'][3])
print('### 15 unknown[4]'); print(D[15]['unknown'][4])
print('### 17 unknown[1]'); print(D[17]['unknown'][1])
print('### 18 unknown[3]'); print(D[18]['unknown'][3])
