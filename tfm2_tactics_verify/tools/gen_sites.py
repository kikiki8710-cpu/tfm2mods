"""probe_targets_<버전>.json + 그 버전 exe 바이트 → src/sites.rs(게이트·노브 표), tools/sites.json(분석 규칙).

실행: python -I tools/gen_sites.py tools/targets/probe_targets_<버전>.json [<exe 경로>]
- 2026-10-22 주간 패치 대응: exe SHA 는 targets 파일의 exe_sha256 과 대조한다(코드에 박지 않음).
  지점 주소는 targets 파일에만 있고, 노브(KNOBS)는 버전 무관 key(최초 0.6.4 RVA, 사본은 #n)로 지점을 가리킨다.
  패치 후 = tools/migrate_targets.py 로 새 targets 를 만든 뒤 이 스크립트를 다시 돌린다. disabled 엔트리·그 노브는 건너뛴다.
- 모든 사이트의 원래 바이트를 exe 에서 다시 읽어 probe_targets 의 bytes_hex 와 대조한다(불일치 = 생성 중단).
- 게이트 = 비교 명령(cmp/test). 피연산자를 capstone 으로 풀어 기록 방식(레지스터/스택/없음)을 정한다.
- 노브 = cfg `set=<id>=<값>` 로 바꿀 수 있는 문턱. 변환 규칙은 아래 KNOBS 표(사람이 확인한 것만).
"""
import hashlib, json, os, struct, sys

import capstone
from capstone import x86 as X

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
EXE_DEFAULT = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
SHA = None  # targets 파일에서 읽는다

# CONTEXT(AMD64) 범용 레지스터 오프셋
CTX = {'rax': 0x78, 'rcx': 0x80, 'rdx': 0x88, 'rbx': 0x90, 'rsp': 0x98, 'rbp': 0xA0, 'rsi': 0xA8, 'rdi': 0xB0,
       'r8': 0xB8, 'r9': 0xC0, 'r10': 0xC8, 'r11': 0xD0, 'r12': 0xD8, 'r13': 0xE0, 'r14': 0xE8, 'r15': 0xF0}
ALIAS = {}
for full, names in {'rax': 'eax ax al', 'rcx': 'ecx cx cl', 'rdx': 'edx dx dl', 'rbx': 'ebx bx bl',
                    'rsp': 'esp sp spl', 'rbp': 'ebp bp bpl', 'rsi': 'esi si sil', 'rdi': 'edi di dil'}.items():
    for n in names.split():
        ALIAS[n] = full
for i in range(8, 16):
    for suf in ('d', 'w', 'b'):
        ALIAS[f'r{i}{suf}'] = f'r{i}'
FLAG_SETTERS = ('cmp', 'test', 'add', 'sub', 'and', 'or', 'xor', 'inc', 'dec', 'imul', 'mul', 'shl', 'shr', 'sar',
                'neg', 'adc', 'sbb', 'bt', 'popcnt', 'lzcnt', 'tzcnt')

# 논리값 → 바이트. write: (사이트 rva, 명령 안 오프셋, 폭, 변환)
#   변환: ('v', d) = V+d / ('sq1',) = V*V+1 / ('toggle', on_bytes, off_bytes) = V!=0 이면 on
KNOBS = [
    ('jg_gank_hp_floor_40', 40, 0, 127, [('0xee4418', 3, 1, ('v', 0)), ('0xee4444', 3, 1, ('v', 0))]),
    ('jg_gank_hp_start_50', 50, 0, 127, [('0xee44d5', 7, 1, ('v', 0)), ('0xee4e8a', 3, 1, ('v', 0))]),
    ('gank_abort_hp_41', 41, 0, 127, [('0xf7bbf6', 3, 1, ('v', 0)), ('0xf7bc33', 3, 1, ('v', 0))]),
    ('gank_abort_tower_reason9', 1, 0, 1, [('0xf7bd27', 0, 1, ('toggle', b'\x75', b'\xeb'))]),
    ('base_defense_min_enemies_6', 6, 1, 127, [('0xfcbd0c', 3, 1, ('v', 0)), ('0xfcbdac', 3, 1, ('v', -1)),
                                               ('0xfcbe6c', 3, 1, ('v', -1))]),
    ('lane_retreat_hp_fewer_65', 65, 0, 127, [('0xfc5052', 1, 1, ('v', 0))]),
    ('lane_retreat_hp_equal_50', 50, 0, 127, [('0xfc5044', 1, 1, ('v', 0))]),
    ('lane_retreat_hp_more_35', 35, 0, 127, [('0xfc5049', 1, 1, ('v', 0))]),
    ('lane_retreat_final_50', 50, 0, 127, [('0xfc57bc', 3, 1, ('v', 0)), ('0xf07f93', 3, 1, ('v', 0))]),
    ('combat_retreat_hp_35', 35, 0, 127, [('0xddaf46', 3, 1, ('v', 0)), ('0xdda2cf', 3, 1, ('v', 0))]),
    ('jungle_safe_radius_150000', 150000, 1000, 2000000, [('0xef243c', 2, 8, ('sq1',))]),
    ('lane_assign_hp_30', 30, 0, 127, [('0xfd43f8', 3, 1, ('v', 0))]),
    ('self_risk_ignore_hp_35', 35, 0, 127, [('0xde40c5', 3, 1, ('v', 0))]),
    ('late_push_threshold_130', 130, 0, 255, [('0xf24fab', 2, 1, ('v', 0))]),
    ('push_sustain_min_healthy_3', 3, 0, 127, [('0xda8804', 3, 1, ('v', 0))]),
    ('push_sustain_margin_2', 2, 0, 126, [('0xda87ca', 1, 1, ('v', 1))]),
    ('gank_call_accept_hp_60', 60, 1, 127, [('0xd5a06f', 3, 1, ('v', -1))]),
    ('gank_call_accept_random_350', 350, 0, 1000, [('0xd5a0ab', 1, 4, ('v', 0))]),
    ('combat_avoid_hp_41', 41, 0, 127, [('0xdd56c3', 3, 1, ('v', 0))]),
    ('combat_avoid_count_margin_2', 2, 0, 127, [('0xdd56c9', 3, 1, ('v', 0))]),
    ('objective_hunt_exit_hp_30', 30, 1, 127, [('0xff95b2', 3, 1, ('v', 0)), ('0xff95b2#2', 3, 1, ('v', 0)), ('0xff95b2#3', 3, 1, ('v', 0)),
                                               ('0xff952e', 3, 1, ('v', -1))]),  # 0.6.5: 30% 비교가 사본 3개(RE A9)
]


def load_exe(path):
    data = open(path, 'rb').read()
    sha = hashlib.sha256(data).hexdigest()
    if sha.lower() != SHA.lower():
        sys.exit(f'exe SHA 불일치: {sha}')
    pe = struct.unpack_from('<I', data, 0x3c)[0]
    nsec = struct.unpack_from('<H', data, pe + 6)[0]
    optsz = struct.unpack_from('<H', data, pe + 20)[0]
    secs = []
    off = pe + 24 + optsz
    for i in range(nsec):
        vsize, va, rawsz, rawptr = struct.unpack_from('<IIII', data, off + 8)
        secs.append((va, max(vsize, rawsz), rawptr))
        off += 40
    def read(rva, n):
        for va, sz, raw in secs:
            if va <= rva < va + sz:
                return data[raw + rva - va: raw + rva - va + n]
        raise KeyError(hex(rva))
    return read


def main():
    global SHA
    targets = json.load(open(sys.argv[1], encoding='utf-8'))
    SHA = targets['exe_sha256'].lower()
    read = load_exe(sys.argv[2] if len(sys.argv) > 2 else EXE_DEFAULT)
    disabled = {e['id'] for e in targets['entries'] if e.get('disabled')}
    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    md.detail = True

    sites = {}
    bykey = {}
    for e in targets['entries']:
        if e.get('disabled'):
            continue
        for s in e['sites']:
            bykey[s.get('key', s['rva'])] = int(s['rva'], 16)
            rva = int(s['rva'], 16)
            want = bytes.fromhex(s['bytes_hex'].replace(' ', ''))
            got = read(rva, len(want))
            if got != want:
                sys.exit(f'바이트 불일치 {e["id"]} {s["rva"]}: exe={got.hex()} 표={want.hex()}')
            sites[rva] = {'entry': e['id'], 'site': s, 'bytes': want}

    def insn_at(rva):
        code = read(rva, 16)
        return next(md.disasm(code, rva))

    def operand_cap(ins, op):
        if op.type == X.X86_OP_REG:
            name = ins.reg_name(op.reg)
            full = ALIAS.get(name, name)
            if full not in CTX:
                return None
            return {'kind': 'reg', 'off': CTX[full], 'width': op.size, 'name': name}
        if op.type == X.X86_OP_MEM:
            base = ins.reg_name(op.mem.base) if op.mem.base else None
            if base == 'rip':
                sys.exit(f'RIP 상대 피연산자는 스텁 복사 불가: {hex(ins.address)} {ins.op_str}')
            if base in ('rsp', 'rbp') and not op.mem.index:
                return {'kind': 'stack', 'off': CTX[base], 'disp': op.mem.disp, 'width': op.size,
                        'name': f'[{base}{op.mem.disp:+#x}]'}
            return {'kind': 'heap', 'name': ins.op_str}
        if op.type == X.X86_OP_IMM:
            # 값은 실행 시 '패치 반영된' 명령 바이트에서 읽는다(at/size). 부호 확장 후 피연산자 폭으로 자른다.
            return {'kind': 'imm', 'value': op.imm, 'width': op.size, 'at': ins.imm_offset, 'size': ins.imm_size}
        return None

    gates = []
    for rva, info in sorted(sites.items()):
        if info['site'].get('role') == 'toggle':
            continue
        ins = insn_at(rva)
        if ins.mnemonic not in ('cmp', 'test'):
            continue
        ops = list(ins.operands)
        a = operand_cap(ins, ops[0])
        b = operand_cap(ins, ops[1]) if len(ops) > 1 else None
        if ins.mnemonic == 'test':
            # test r,r 만 지원: 플래그가 cmp r,0 과 같다(SF/ZF, CF=OF=0).
            if not (len(ops) == 2 and ops[0].type == X.X86_OP_REG and ops[1].type == X.X86_OP_REG and ops[0].reg == ops[1].reg):
                sys.exit(f'test 형태 미지원 {hex(rva)}')
            b = {'kind': 'const', 'value': 0, 'width': a['width']}
        # 다음 플래그 소비 명령(최대 5개 앞까지, 사이에 플래그 설정 명령이 없을 때만)
        cond, at = None, rva + ins.size
        for _ in range(5):
            nx = insn_at(at)
            m = nx.mnemonic
            if m.startswith(('j', 'set', 'cmov')) and m != 'jmp':
                cond = {'mnemonic': m, 'rva': hex(at), 'op_str': nx.op_str}
                break
            if m.startswith(FLAG_SETTERS) or m in ('call', 'ret', 'jmp'):
                break
            at += nx.size
        branch = info['site'].get('branch') or {}
        gates.append({
            'id': f"{info['entry']}@{hex(rva)}", 'entry': info['entry'], 'rva': hex(rva),
            'bytes': info['bytes'].hex(), 'asm': f'{ins.mnemonic} {ins.op_str}', 'a': a, 'b': b, 'cond': cond,
            'taken_means': branch.get('taken_means'), 'not_taken_means': branch.get('not_taken_means'),
            'operand_meaning': info['site'].get('operand_meaning'), 'role': info['site'].get('role'),
        })

    knobs = []
    for kid, default, lo, hi, writes in KNOBS:
        ws = []
        # 사본 key(#n)는 그 버전에 있으면 쓴다(갈라지기 전 버전에는 없음). 원래 key 가 없으면 노브 전체를 건너뛴다.
        writes = [w for w in writes if '#' not in w[0] or w[0] in bykey]
        if any(k not in bykey for k, *_ in writes):
            lost = [k for k, *_ in writes if k not in bykey]
            print(f'노브 {kid} 건너뜀(이 버전에서 지점 비활성/없음: {lost})')
            continue
        for key, at, width, tf in writes:
            rva = bykey[key]
            rva_s = hex(rva)
            if rva not in sites and tf[0] != 'toggle':
                sys.exit(f'노브 {kid}: 사이트 {rva_s} 가 표에 없음')
            orig = sites[rva]['bytes'] if rva in sites else read(rva, 1)
            cur = orig[at:at + width]
            if tf[0] == 'v':
                val = int.from_bytes(cur, 'little', signed=False)
                if val != default + tf[1]:
                    sys.exit(f'노브 {kid} {rva_s}: 현재 {val} ≠ 기본값 {default}+{tf[1]}')
            elif tf[0] == 'sq1':
                if int.from_bytes(cur, 'little') != default * default + 1:
                    sys.exit(f'노브 {kid}: R²+1 불일치')
            elif tf[0] == 'toggle':
                if read(rva, 1) != tf[1]:
                    sys.exit(f'노브 {kid}: 토글 원래 바이트 불일치 {read(rva, 1).hex()}')
            ws.append({'rva': rva_s, 'orig': (orig if rva in sites else read(rva, 1)).hex(), 'at': at,
                       'width': width, 'tf': [tf[0]] + [x.hex() if isinstance(x, bytes) else x for x in tf[1:]]})
        knobs.append({'id': kid, 'default': default, 'min': lo, 'max': hi, 'writes': ws})

    # ---- sites.rs
    def cap_rs(c):
        if not c or c['kind'] == 'heap':
            return 'Cap::None'
        if c['kind'] == 'imm':
            return f"Cap::Imm {{ at: {c['at']}, size: {c['size']}, width: {c['width']} }}"
        if c['kind'] == 'const':
            return f"Cap::Const {{ value: {c['value']}, width: {c['width']} }}"
        if c['kind'] == 'reg':
            return f"Cap::Reg {{ off: {c['off']:#x}, width: {c['width']} }}"
        return f"Cap::Stack {{ base: {c['off']:#x}, disp: {c['disp']}, width: {c['width']} }}"

    def tf_rs(t):
        if t[0] == 'v':
            return f'Tf::Add({t[1]})'
        if t[0] == 'sq1':
            return 'Tf::SquarePlus1'
        return f"Tf::Toggle {{ on: 0x{t[1]}, off: 0x{t[2]} }}"

    def bytes_rs(h):
        return '&[' + ', '.join(f'0x{h[i:i + 2]}' for i in range(0, len(h), 2)) + ']'

    out = ['// 자동 생성: tools/gen_sites.py — 직접 편집하지 않는다.',
           f'pub const BASELINE_EXE_SHA256: &str = "{SHA}";',
           '#[derive(Clone, Copy)] pub enum Cap { None, Reg { off: u16, width: u8 }, Stack { base: u16, disp: i32, width: u8 }, Imm { at: u8, size: u8, width: u8 }, Const { value: i64, width: u8 } }',
           '#[derive(Clone, Copy)] pub enum Tf { Add(i64), SquarePlus1, Toggle { on: u8, off: u8 } }',
           'pub struct GateDef { pub id: &\'static str, pub rva: usize, pub orig: &\'static [u8], pub a: Cap, pub b: Cap }',
           'pub struct KnobWrite { pub rva: usize, pub orig: &\'static [u8], pub at: usize, pub width: usize, pub tf: Tf }',
           'pub struct KnobDef { pub id: &\'static str, pub default: i64, pub min: i64, pub max: i64, pub writes: &\'static [KnobWrite] }',
           'pub static GATES: &[GateDef] = &[']
    for g in gates:
        out.append(f"    GateDef {{ id: \"{g['id']}\", rva: {g['rva']}, orig: {bytes_rs(g['bytes'])}, a: {cap_rs(g['a'])}, b: {cap_rs(g['b'])} }},")
    out.append('];')
    out.append('pub static KNOBS: &[KnobDef] = &[')
    for k in knobs:
        ws = ', '.join(f"KnobWrite {{ rva: {w['rva']}, orig: {bytes_rs(w['orig'])}, at: {w['at']}, width: {w['width']}, tf: {tf_rs(w['tf'])} }}"
                       for w in k['writes'])
        out.append(f"    KnobDef {{ id: \"{k['id']}\", default: {k['default']}, min: {k['min']}, max: {k['max']}, writes: &[{ws}] }},")
    out.append('];')
    open(os.path.join(ROOT, 'src', 'sites.rs'), 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
    json.dump({'exe_sha256': SHA, 'gates': gates, 'knobs': knobs}, open(os.path.join(HERE, 'sites.json'), 'w', encoding='utf-8'),
              ensure_ascii=False, indent=1)
    print(f'게이트 {len(gates)}개, 노브 {len(knobs)}개')
    for g in gates:
        print(' ', g['id'].ljust(46), g['asm'][:36].ljust(36), 'a=', (g['a'] or {}).get('kind'), 'b=', (g['b'] or {}).get('kind'),
              'cond=', (g['cond'] or {}).get('mnemonic'))


if __name__ == '__main__':
    main()
