"""56개 정적 검증 후보의 카탈로그/네이티브 창을 공용 생성기에 연결한다."""
import json, re, struct
from pathlib import Path
from capstone import CS_GRP_JUMP, CS_GRP_CALL, CS_GRP_RET
from capstone.x86 import X86_OP_IMM, X86_OP_MEM, X86_REG_RIP
ROOT = Path(__file__).resolve().parent.parent

def records():
    docs = [json.loads((ROOT/'tools'/'targets'/f'extra_{part}_0.6.5.json').read_text(encoding='utf-8')) for part in ('lane','combat','ops')]
    rows = sorted([t for d in docs for t in d['tactics']], key=lambda t:t['audit_id'])
    assert len(rows)==56 and len({t['audit_id'] for t in rows})==56
    assert len({t['id'] for t in rows})==56
    return docs, rows

def write_catalog():
    _,rows=records()
    q=lambda s:json.dumps(s,ensure_ascii=False)
    lines=['    // BEGIN GENERATED EXTRAS — tools/gen_extras.py']
    for t in rows:
        args=','.join(q(t[k]) for k in ('id',)) + ','+q('extra_'+t['audit_id'].lower())+','+q(t['name'])+','+q(t['desc'])+','+('P' if t['scope']=='Player' else 'T')
        if t['kind']=='Toggle': lines.append('    t('+args+'),')
        else: lines.append('    v('+args+','+','.join(str(t[k]) for k in ('default','low','high','min','max'))+'),')
    lines.append('    // END GENERATED EXTRAS')
    path=ROOT/'src'/'tactics.rs';s=path.read_text(encoding='utf-8')
    s=re.sub(r'    // BEGIN GENERATED EXTRAS.*?    // END GENERATED EXTRAS\n','',s,flags=re.S)
    s=s.replace('\n];\n\npub const N:', '\n'+'\n'.join(lines)+'\n];\n\npub const N:',1)
    s=re.sub(r'pub const N: usize = \d+;', 'pub const N: usize = 88;',s)
    path.write_text(s,encoding='utf-8',newline='\n')
    # Preserve existing category membership and save identities; append audit rows.
    path=ROOT/'src'/'presentation.rs';s=path.read_text(encoding='utf-8')
    old=s[s.index('pub const CATEGORIES:'):s.index('\npub fn tactic_at')]
    cats=[]
    for name,body in re.findall(r'Category \{ name: "([^"]+)", ids: &\[(.*?)\] \}',old,re.S):
        cats.append((name,[i for i in re.findall(r'"([a-z0-9_]+)"',body) if i not in {t['id'] for t in rows}]))
    for name in ('오브젝트','위치·기술'):
        if name not in [n for n,_ in cats]:cats.append((name,[]))
    for t in rows:
        match=next((ids for name,ids in cats if name==t['tab']),None)
        if match is None:raise ValueError('unknown tab '+t['tab'])
        match.append(t['id'])
    catalog='pub const CATEGORIES: &[Category] = &[\n'+''.join('    Category { name: '+q(n)+', ids: &[\n'+''.join('        '+q(i)+',\n' for i in ids)+'    ] },\n' for n,ids in cats)+'];\n'
    s=s.replace(old,catalog.rstrip())
    s=re.sub(r'pub const ROWS: usize = \d+;',f'pub const ROWS: usize = {max(len(ids) for _,ids in cats)};',s)
    s=re.sub(r'    // BEGIN EXTRA VISIBILITY.*?    // END EXTRA VISIBILITY\n','',s,flags=re.S)
    visibility='    // BEGIN EXTRA VISIBILITY\n    match id {\n'+''.join('        '+q(t['id'])+' => return '+str(t['pos_mask'])+' & (1 << col) != 0,\n' for t in rows)+'        _ => {}\n    }\n    // END EXTRA VISIBILITY\n'
    s=s.replace('    match id {',visibility+'    match id {',1)
    path.write_text(s,encoding='utf-8',newline='\n')

def load(pe,sha,md,starts,ends,order,func_cache,interior_all,log):
    from gen_sites import find_func,parse_src,JCC
    docs,rows=records();assert all(d['exe_sha256'].lower()==sha.lower() for d in docs)
    patches=[]
    for t in rows:
        assert t['min']<=t['low']<=t['max'] and t['min']<=t['high']<=t['max']
        assert t['sites'],t['audit_id']
        for idx,s in enumerate(t['sites']):
            rva=int(s['rva'],16); win=bytes.fromhex(s['win_hex']);dyn=bytes.fromhex(s['dynamic_hex']);first=s['first_len']
            assert len(win)>=5 and pe.read(rva,len(win))==win,(t['id'],s['rva'],'bytes')
            ins=list(md.disasm(win,0x140000000+rva));assert sum(i.size for i in ins)==len(win)
            bounds={i.address-0x140000000-rva for i in ins}|{len(win)}
            assert first in bounds and 0<first<=len(win),(t['id'],'fragment boundary')
            branches=[]
            for i in ins:
                off=i.address-0x140000000-rva
                assert not i.group(CS_GRP_CALL) and not i.group(CS_GRP_RET)
                assert not any(o.type==X86_OP_MEM and o.mem.base==X86_REG_RIP for o in i.operands)
                if i.group(CS_GRP_JUMP):
                    assert off>=first and i.operands[0].type==X86_OP_IMM
                    branches.append((off,i.size,JCC.get(i.mnemonic),i.operands[0].imm-0x140000000))
            di=list(md.disasm(dyn,0));assert dyn and sum(i.size for i in di)==len(dyn)
            assert not any(i.group(CS_GRP_JUMP) or i.group(CS_GRP_CALL) or i.group(CS_GRP_RET) for i in di)
            fn=find_func(starts,ends,rva);assert fn and rva+len(win)<=fn[1]
            if fn not in func_cache:
                tg=set()
                for i in md.disasm(pe.read(fn[0],fn[1]-fn[0]),0x140000000+fn[0]):
                    if (i.group(CS_GRP_JUMP) or i.group(CS_GRP_CALL)) and i.operands and i.operands[0].type==X86_OP_IMM:tg.add(i.operands[0].imm-0x140000000)
                    if i.mnemonic=='lea' and len(i.operands)==2 and i.operands[1].type==X86_OP_MEM and i.operands[1].mem.base==X86_REG_RIP:
                        tb=i.address+i.size+i.operands[1].mem.disp-0x140000000
                        raw=pe.read(tb,8192) if pe.off(tb) is not None else b'';mode=None
                        for j in range(len(raw)//4):
                            e=struct.unpack_from('<i',raw,4*j)[0]
                            if mode is None:mode='rel' if fn[0]<=tb+e<fn[1] else ('abs' if fn[0]<=e<fn[1] else None)
                            if mode is None:break
                            cand=tb+e if mode=='rel' else e
                            if not fn[0]<=cand<fn[1]:break
                            tg.add(cand)
                func_cache[fn]=tg
            interior=set(range(rva+1,rva+len(win)));assert not interior&func_cache[fn],(t['id'],s['rva'],'incoming branch')
            for h in interior:interior_all[h]=(t['id'],s['rva'])
            tf=s['tf']
            if 'calc' in tf:
                assert re.fullmatch(r'extra_(lane|combat|ops)::[a-z0-9_]+',tf['calc'])
                trans='Tf::Compute(crate::'+tf['calc']+')'
            elif tf['kind']=='Add':trans=f"Tf::Add({tf['value']})"
            elif tf['kind']=='Sq1':trans='Tf::Sq1'
            elif tf['kind']=='Map':trans=f"Tf::Map {{ on: {tf['on']}, off: {tf['off']} }}"
            else:raise ValueError(tf)
            fields=[f'Field {{ off: 0, width: 8, tf: {trans}, max: i64::MAX }}']
            asm=' ; '.join(i.mnemonic+' '+i.op_str for i in ins)
            patches.append(dict(name=f"extra_{t['audit_id'].lower()}@{s['rva']}",tactic=order['extra_'+t['audit_id'].lower()],rva=rva,win=win,fields=fields,branches=branches,src=parse_src(s['src']),chk=parse_src(s.get('team_chk')),asm=asm,func=fn[0],first_len=first,dynamic=dyn))
            log.append(f"[추가창] {t['audit_id']} {s['rva']} {asm} | 주체 {s['src']}")
    assert len({p['tactic'] for p in patches})==56
    return patches
