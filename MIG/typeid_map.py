"""typeid_map.py — game_ai IR 의 TypeId(i128) 상수 ↔ exe 의 실제 16B 상수 매핑.
왜: rlib(IR)과 exe 는 크레이트 해시가 달라 TypeId 가 다르다(실측 0/11 존재). 비트코드를 링크한 내 사본이
    게임이 만든 Effect 객체의 type_id 와 비교하려면 exe 쪽 값으로 바꿔야 한다.
방법: IR 함수 → DISubprogram(file,line) → 스캐폴딩 md 섹션(원본 행 범위) → RVA → capstone 으로
      16B rip-상대 피연산자(pcmpeqb/movdqa 등)와 movabs imm64 쌍을 수집 → IR 상수 순서와 정렬."""
import re,glob,os,io,struct,collections,capstone
EXE=r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
d=open(EXE,'rb').read()
pe=struct.unpack_from('<I',d,0x3c)[0]; nsec=struct.unpack_from('<H',d,pe+6)[0]; osz=struct.unpack_from('<H',d,pe+20)[0]
secs=[struct.unpack_from('<IIII',d,pe+24+osz+i*40+8) for i in range(nsec)]
def off(rva):
    for vsz,va,rsz,ra in secs:
        if va<=rva<va+max(vsz,rsz): return ra+rva-va
BASE=0x140000000
# 1) IR: 사이트 수집 + DISubprogram
sites=collections.OrderedDict()   # fn -> [(const,...)]
meta={}
for f in sorted(glob.glob(r'C:\tfm2mods\_gaibc\m*.ll')):
    cur=None; dbg=None; sp={}; files={}; pending={}
    lines=open(f,encoding='utf-8',errors='ignore').read().split('\n')
    for l in lines:
        if l.startswith('define '):
            cur=re.search(r'@("?[^"( ]+"?)\(',l).group(1).strip('"'); m=re.search(r'!dbg !(\d+)',l); dbg=m.group(1) if m else None
            if dbg: sites.setdefault(cur,[]); pending[cur]=dbg
        elif 'icmp eq i128' in l and cur:
            m=re.search(r', (-?\d{20,})',l)
            if m: sites[cur].append(int(m.group(1)))
        elif l.startswith('!'):
            m=re.match(r'!(\d+) = distinct !DISubprogram\(name: "([^"]+)".*?file: !(\d+), line: (\d+)',l)
            if m: sp[m.group(1)]=(m.group(2),m.group(3),int(m.group(4)))
            m=re.match(r'!(\d+) = !DIFile\(filename: "([^"]+)"',l)
            if m: files[m.group(1)]=m.group(2)
    for fn,dd in pending.items():
        if dd in sp:
            n,fi,ln=sp[dd]; meta[fn]=(n,files.get(fi,'?'),ln)
sites={k:v for k,v in sites.items() if v}
# 2) md 섹션 색인
sect={}   # mdpath -> [(A,B,rva,end)]
for md in glob.glob(r'C:\tfm2mods\MIG\decomp\0.5.8\**\*.md',recursive=True):
    t=io.open(md,encoding='utf-8').read()
    key=os.path.relpath(md,'C:/tfm2mods/MIG/decomp/0.5.8').replace(chr(92),'/')[:-3]
    L=[]
    for m in re.finditer(r'^## `(0x[0-9a-f]+)`\s+—\s+원본 행 (\d+)~(\d+).*?\n\| RVA \| `0x[0-9a-f]+` ~ `(0x[0-9a-f]+)`',t,re.M|re.S):
        L.append((int(m.group(2)),int(m.group(3)),int(m.group(1),16),int(m.group(4),16)))
    sect[key]=L
md_cs=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64); md_cs.detail=True
def ent(v): return len(set(v.to_bytes(16,'little',signed=True)))>=10
def scan(rva,end):
    o=off(rva); out=[]; imm=[]
    for ins in md_cs.disasm(d[o:o+(end-rva)],BASE+rva):
        for op in ins.operands:
            if op.type==capstone.x86.X86_OP_MEM and op.mem.base==capstone.x86.X86_REG_RIP and op.size==16:
                tgt=ins.address+ins.size+op.mem.disp; oo=off(tgt-BASE)
                if oo: 
                    v=int.from_bytes(d[oo:oo+16],'little',signed=True)
                    if ent(v): out.append(('rdata@0x%x'%(tgt-BASE),v))
            if op.type==capstone.x86.X86_OP_IMM and op.size==8 and ins.mnemonic in('movabs','mov'):
                imm.append(op.imm&0xffffffffffffffff)
    for a,b in zip(imm,imm[1:]):
        v=int.from_bytes(a.to_bytes(8,'little')+b.to_bytes(8,'little'),'little',signed=True)
        if ent(v) and a>0xffffffff and b>0xffffffff: out.append(('imm64pair',v))
    return out
print('IR TypeId 비교 함수 %d개'%len(sites))
agg=collections.defaultdict(collections.Counter)
for fn,cs in sites.items():
    name,file,line=meta.get(fn,('?','?',0))
    key=(file.replace(chr(92)*2,'/').replace(chr(92),'/').split('game-ai/src/')[-1][:-3]) if file.endswith('.rs') else '?'
    L=sect.get(key,[])
    cand=[s for s in L if s[0]>=line] 
    cand.sort(key=lambda s:s[0]-line)
    print('\n%s  (%s:%d)  IR상수 %s'%(name,key,line,[str(c)[:8] for c in cs]))
    for A,B,rva,end in cand[:2]:
        found=scan(rva,end)
        uniq=list(dict.fromkeys(v for _,v in found))
        print('   후보 RVA 0x%x (행 %d~%d, %dB): exe 16B상수 %d개 %s'%(rva,A,B,end-rva,len(uniq),[hex(v&((1<<128)-1))[:12] for v in uniq]))
        ic=list(dict.fromkeys(cs))
        if len(uniq)==len(ic):
            for a,b in zip(ic,uniq): agg[a][b]+=1
print('\n=== 집계(IR상수 → exe상수, 함수 간 일관성)')
for a,c in agg.items():
    print('  IR %s → %s'%(str(a)[:12], ', '.join('%s×%d'%(hex(b&((1<<128)-1))[:14],n) for b,n in c.most_common())))
