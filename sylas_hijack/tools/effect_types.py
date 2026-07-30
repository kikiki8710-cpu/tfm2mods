# effect 타입 테이블 추출 (DataEffectDef enum disc -> 타입명)
import struct
EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
IB = 0x140000000
def load():
    data = open(EXE,"rb").read()
    pe = data.index(b'PE\x00\x00'); coff = pe+4
    nsec = struct.unpack_from("<H",data,coff+2)[0]; optsz = struct.unpack_from("<H",data,coff+16)[0]
    sec = coff+20+optsz; secs=[]
    for i in range(nsec):
        o=sec+i*40; vsz,va,rawsz,raw=struct.unpack_from("<IIII",data,o+8); secs.append((va,vsz,raw,rawsz))
    return data, secs
def rva2off(secs,rva):
    for va,vsz,raw,rawsz in secs:
        if va<=rva<va+max(vsz,rawsz): return raw+(rva-va)
    return None
def build_type_table():
    data,secs=load()
    def rdstr(absp,ln):
        o=rva2off(secs,absp-IB)
        if o is None: return None
        try: return data[o:o+ln].decode('latin1')
        except: return None
    # (ptr,len) 배열: 0x1429869b0 에서 유효 문자열 나오는 첫 지점부터가 disc=0
    base=0x1429869b0; o=rva2off(secs,base-IB)
    table={}; disc=0; started=False
    for i in range(200):
        ptr,ln=struct.unpack_from("<QQ",data,o+i*16)
        if ln==0 or ln>40 or ptr<IB or ptr>IB+0x10000000:
            if started: break  # 배열 끝
            continue
        s=rdstr(ptr,ln)
        if s and all(32<=ord(c)<127 for c in s) and s[0].isupper():
            if not started: started=True
            table[disc]=s; disc+=1
        elif started: break
    return table
if __name__=="__main__":
    t=build_type_table()
    for d in sorted(t): print(f"  {d} (0x{d:02x}) = {t[d]}")
    print(f"총 {len(t)}종")
