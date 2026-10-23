"""생성된 실제 x64 원본/동적/전체 스텁을 Unicorn에서 차등 실행한다(게임 실행 아님)."""
import argparse,json,random,struct,sys
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--deps');p.add_argument('--fixture',required=True);a=p.parse_args()
if a.deps:sys.path.insert(0,a.deps)
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
REGS=[UC_X86_REG_RAX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RBX,UC_X86_REG_RSP,UC_X86_REG_RBP,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]
ORDER=[0,1,2,3,5,6,7,8,9,10,11,12,13,14,15]
CODE=0x100000;STUB=0x200000;DYNAMIC=0x300000;STACK=0x800000;RSP=STACK+0x8000;RBP=STACK+0x10000
rows=json.loads(Path(a.fixture).read_text());runs=0
# 매 실행 전에 환경을 초기화한다. 게임 주소 범위는 종료 분기 검사용.
u=Uc(UC_ARCH_X86,UC_MODE_64)
for start,size in [(CODE,0x10000),(STUB,0x10000),(DYNAMIC,0x10000),(STACK,0x20000),(0x140000000,0x2000000)]:u.mem_map(start,size)
dispatch=rows[0]['dispatch'];u.mem_map(dispatch&~0xfff,0x2000)
state={}
def hook(uc,addr,size,_):
    if addr==dispatch:
        assert uc.reg_read(UC_X86_REG_RSP)%16==8,'Windows ABI stack alignment'
        frame=uc.reg_read(UC_X86_REG_RDX)
        assert frame==RSP-240
        snapshot=struct.unpack('<15Q',bytes(uc.mem_read(frame+96,120)))
        for k,r in enumerate(ORDER):assert snapshot[14-k]==state['regs'][r],('snapshot',r)
        assert uc.reg_read(UC_X86_REG_RCX)==state['index']
        uc.mem_write(frame+224,struct.pack('<Q',state['value']))
        sp=uc.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',bytes(uc.mem_read(sp,8)))[0]
        for r in [0,1,2,8,9,10,11]:uc.reg_write(REGS[r],0xdeadbeef)
        for n in range(6):uc.reg_write(UC_X86_REG_XMM0+n,(1<<128)-1)
        uc.reg_write(UC_X86_REG_EFLAGS,0x202)
        uc.reg_write(UC_X86_REG_RAX,DYNAMIC if state['apply'] else CODE)
        uc.reg_write(UC_X86_REG_RSP,sp+8);uc.reg_write(UC_X86_REG_RIP,ret)
    elif addr>=0x140000000:
        state['exit']=addr;uc.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
for index,row in enumerate(rows):
    for value in row['values']:
        for seed in range(12):
            rng=random.Random(seed*1777+index)
            regs=[rng.getrandbits(64) for _ in REGS];regs[4]=RSP;regs[5]=RBP
            # 갱 토글 창의 fallthrough는 [rsi+0x30]에 종료 플래그를 쓴다.
            # 실제 메모리 쓰기도 원본/동적/전체 스텁 사이에서 대조한다.
            if row['name'].startswith('gank_abort_tower_reason9'):
                regs[6]=STACK+0x12000
            flags=0x202|(rng.getrandbits(12)&0x8d5) # defined arithmetic bits + DF
            memory=rng.randbytes(0x20000)
            xmm=[rng.getrandbits(128) for _ in range(16)]
            def run(mode):
                state.clear();state.update(index=index,value=value,regs=regs,apply=mode=='stub',exit=None)
                u.mem_write(STACK,memory)
                u.mem_write(RSP-16,struct.pack('<Q',value))
                for r,v in zip(REGS,regs):u.reg_write(r,v)
                for n,v in enumerate(xmm):u.reg_write(UC_X86_REG_XMM0+n,v)
                u.reg_write(UC_X86_REG_EFLAGS,flags)
                ref=bytearray(row['original'])
                if mode!='fallback':
                    toggle=row.get('branch_toggle')
                    if toggle:
                        assert value in (toggle['on'],toggle['off'])
                        if value==toggle['off']:
                            ref=bytearray(b'\xff\x25\0\0\0\0'+struct.pack('<Q',toggle['target']))
                    else:
                        off=row['field_off'];w=row['field_width'];ref[off:off+w]=value.to_bytes(w,'little')
                u.mem_write(CODE,bytes(ref));u.mem_write(DYNAMIC,bytes(row['dynamic']));u.mem_write(STUB,bytes(row['stub']))
                for lo in (CODE,STUB,DYNAMIC):u.ctl_remove_cache(lo,lo+0x10000)
                start={'reference':CODE,'dynamic':DYNAMIC,'stub':STUB,'fallback':STUB}[mode]
                u.emu_start(start,0,count=300)
                assert state['exit'] is not None,('no exit',row['name'],mode)
                return ([u.reg_read(r) for r in REGS],u.reg_read(UC_X86_REG_EFLAGS),[u.reg_read(UC_X86_REG_XMM0+n) for n in range(16)],state['exit'],bytes(u.mem_read(STACK+0x12000,0x100)))
            ref=run('reference')
            mask=0xcd5
            # IMUL defines CF/OF; PF/AF/ZF/SF are undefined.
            if row['win'][1:2]==[0x6b]:mask=0xc01
            # XOR는 AF를 정의하지 않는다(끔 경로는 원래 flags를 보존).
            if row['name'].startswith('bot_duo_partner_required'):mask &= ~0x10
            for mode in ['dynamic','stub']:
                got=run(mode)
                assert got[0]==ref[0],(row['name'],value,seed,mode,'GPR',[(i,hex(x),hex(y))for i,(x,y)in enumerate(zip(got[0],ref[0]))if x!=y])
                assert (got[1]&mask)==(ref[1]&mask),(row['name'],value,seed,mode,'flags',hex(got[1]),hex(ref[1]))
                assert got[2:]==ref[2:],(row['name'],value,seed,mode,'XMM/exit')
                runs+=1
            # 기본 단계/남의 팀/식별 실패의 콜백 결과는 원본 트램폴린이다.
            original_value=int.from_bytes(bytes(row['win'][row['field_off']:row['field_off']+row['field_width']]),'little')
            old_value=value;value=original_value
            vanilla=run('reference');fallback=run('fallback');value=old_value
            assert fallback[0]==vanilla[0] and (fallback[1]&mask)==(vanilla[1]&mask) and fallback[2:]==vanilla[2:],(row['name'],'fallback')
            runs+=1
print(json.dumps({'sites':len(rows),'comparisons':runs,'mismatches':0,'checks':['all GPR','RSP','defined flags','XMM0-15','branch destination','ABI alignment','snapshot']},ensure_ascii=False))
