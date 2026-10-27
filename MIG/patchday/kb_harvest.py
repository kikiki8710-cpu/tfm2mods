# -*- coding: utf-8 -*-
"""kb_harvest.py — 지식베이스(메모리·ANA·REPORT) 문서에 흩어진 특정 게임버전의 RVA 를 수확해 exe(.pdata·capstone)로 검증한 주소록 JSON 을 만든다(읽기 전용)."""
# 사용(0.6.5 기준선 — 다음 버전엔 --ver/--exe/--prev-exe/--since/--out 만 바꾼다):
#   python -X utf8 F:\claude\tfm2mods\MIG\patchday\kb_harvest.py ^
#       --ver 0.6.5 --exe F:\claude\tfm2\tfm2_0.6.5\TeamfightManager2.exe ^
#       --prev-exe F:\claude\tfm2\tfm2_0.6.4\TeamfightManager2.exe --since 2026-10-21T15:43 ^
#       --out F:\claude\tfm2mods\MIG\patchday\kb_rows_0.6.5.json
#   (--since = 대상 버전 설치 시각 · CURRENT.md §1 · 약 20초)
#
# 왜: 매니페스트(MIG\manifest) 밖에 "문서로만" 남은 0.6.x RVA(RE·회차 정본·INDEX 역맵·DONE 표)가 많다.
#     패치 날 그걸 다시 모으려고 문서를 뒤지지 않도록, 버전 귀속·exe 검증까지 끝낸 주소록을 미리 만든다.
#
# 범위: MEM *.md · ANA *.md(_archive 제외) · REPORT *.md/*.json(_archive 제외). mtime < since−1일 파일은 건너뜀
#   (대상 버전 설치 전에 마지막으로 고쳐진 파일엔 대상 버전 사실이 있을 수 없다).
# ★버전 귀속 규칙(토큰 단위 · 위가 우선) — "다른 버전 태그 값은 제외"를 기계로 하려는 것:
#   취소선 ~~..~~ 안 → 제외(struck) · 「구/옛/old 0x..」 → 제외(old_prefix) · 경로 참조 「analysis_064/…/0xdd1fb0/」 → 제외(path_ref)
#   바로 앞 「0.6.4 0x..」「0.6.5=0x..」 → 그 버전(inline)   바로 뒤 「0x.. (0.6.5 …)」 → 그 버전(suffix)
#   표: 열 머리에 버전/「구값·구RVA」 → 그 버전/제외(table) · 「버전」 열이 있으면 그 행의 값(table · 같은 칸 태그가 우선)
#   같은 칸/줄의 남는 버전 태그(비교 「0.6.4 와 동일」·경로·deps·모드버전 v0.x 제외) → 단일이면 line,
#     섞이면 150자 안의 가장 가까운 태그(line-mixed)
#   표 머리 전체의 단일 버전 → table · 제목(#) 버전 → heading(섞이면 heading-mixed)
#   문서: 경로/파일명 버전표지(_0.6.5_·065) → doc · 대상 버전 이후 태어난 문서의 머리 30줄 SHA/단일 태그 → doc ·
#     작성일(경로 날짜 · REPORT 만 ctime) ≥ since → date · 문서에 다른 게임버전이 한 번이라도 나오면 doc → doc-multi
#     ⚠MEM·ANA 는 도구의 원자적 재기록으로 ctime==mtime(실측) → 작성일 근거로 쓰지 않음(CTIME_ROOTS)
#   이전→신규 화살표(0xA→0xB, 괄호 하나 사이 허용)의 왼쪽: 이주 맥락이면 제외, 아니면 +arrow_left(inferred)
#   confidence: explicit = inline/suffix/table/line/heading/doc/doc-multi 근거 1개 이상 · inferred = 그 밖
# ★검증: .text → .pdata 이진탐색(체인 언와인드는 루트로) → 시작이면 FN_START, 중간이면 owner 를 capstone 선형 디스어셈해
#   명령 경계 확인(MID) · .pdata 밖 .text 는 앞 조각 끝에서 CC/90 패딩을 건너뛰어 같은 확인(note=nopdata) ·
#   .rdata(+섹션 정렬 꼬리) → RDATA · .data → DATA(스키마 확장) · 그 밖·경계 아님 → rejected.
#   명령 중간 기각엔 insn_start/insn_off 를 붙인다(즉치값 바이트 패치 지점 — 실측 comptest RPLY 0xd0af9 = mov rbx,-1 의 +3).
#   --prev-exe: 버전이 섞인 맥락(heading-mixed·doc-mixed·line-mixed·doc-multi·arrow_left)으로만 귀속된 값이
#   직전 버전 함수 시작이고 대상 버전에선 시작이 아니면 구값으로 기각(순수 대상 문서엔 미적용 — 우연 일치 ≈0.3%).
# ★0x 없는 맨 hex(감사 문서의 `fb51f3`)는 대상 버전 맥락의 줄에서만 후보 · notation=bare · 데이터 섹션에 떨어지면
#   문맥 어휘(vtable·JT·문자열·rip…)가 있을 때만 수용(bare_data_unverified).
# 출력 스키마: {"game_ver","exe_sha256","rows":[{rva,kind,owner,names,sources,confidence,basis,notation,listing_only,
#              src_types,manifest,note}],"rejected":[{rva,source,why,notation,n,sources,manifest,basis,prev_fn_start,
#              insn_start,insn_off,owner}],"stats":{...}}
# 제약: KB 파일은 읽기만 한다(쓰기 없음). 출력 1파일 외 임시물 없음.

import argparse, bisect, collections, datetime, glob, hashlib, io, json, os, re, struct, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
MODS = os.path.abspath(os.path.join(HERE, '..', '..'))          # F:\claude\tfm2mods
sys.path.insert(0, MODS)
import fnindex                                                  # PE 판독(load·roff) 재사용(import 시 stdout 을 utf-8 로 재포장함 — 그대로 둔다)

from capstone import Cs, CS_ARCH_X86, CS_MODE_64

DEF_MEM = r'C:\Users\jungs\.claude\projects\C--Users-jungs-Desktop-claude-tfm2\memory'
DEF_ANA = r'F:\claude\tfm2\팀파매2모드 분석'
DEF_REP = r'F:\claude\tfm2\mods_report'
DEF_MAN = os.path.join(MODS, 'MIG', 'manifest')
IMAGE_BASE = 0x140000000
MIXED_WIN = 150                                                 # 버전이 섞인 줄에서 태그를 빌려올 최대 거리(자)
CTIME_ROOTS = ('REPORT',)                                       # ctime(생성시각)을 작성일로 믿어도 되는 루트
IMAGE_LIMIT = [0x10000000]                                     # main 에서 exe SizeOfImage 근사값으로 갱신

# ───────────────────────── 정규식 ─────────────────────────
# 게임 버전 태그: 0.1.x~0.6.x · 앞이 v/V(모드 버전)·부등호(deps)·경로 구분자면 제외
VER_RE = re.compile(r'(?<![0-9A-Za-z.<>=_\\/\-])(0\.[1-6]\.\d{1,2})(?![0-9]|\.\d|[-_][0-9A-Za-z])')
TOK_RE = re.compile(
    r'(?P<fun>(?<![0-9A-Za-z_])(?:FUN|LAB|DAT|PTR|thunk_FUN|SUB|sub|loc|off)_(?P<funv>14[0-9a-fA-F]{7})(?![0-9A-Za-z_]))'
    r'|(?P<abs>(?<![0-9A-Za-z_])0x(?P<absv>14[0-9a-fA-F]{7})(?![0-9A-Za-z_]))'
    r'|(?P<hex>(?<![0-9A-Za-z_])0x(?P<hexv>[0-9a-fA-F]{5,8})(?![0-9A-Za-z_]))'
    r'|(?P<bare>(?<![0-9A-Za-z_.+\-−\[#@$\\])(?P<barev>[0-9a-fA-F]{5,7})(?![0-9A-Za-z_\]]))')
HEX_ONLY = re.compile(r'^\s*(?:0x)?([0-9a-fA-F]{5,9})\s*$')
STRIKE_RE = re.compile(r'~~.+?~~')
HEADING_RE = re.compile(r'^(#{1,6})\s+(.*)$')
FENCE_RE = re.compile(r'^\s*(```|~~~)')
LISTING_LINE = re.compile(r'^\s*(?:0x)?[0-9a-fA-F]{5,9}:?\s+(?:[0-9a-fA-F]{2}\s)+')
LISTING_LINE2 = re.compile(r'^\s*(?:0x)?[0-9a-fA-F]{5,9}:?\s+(?:j[a-z]{1,3}|call|mov[a-z]*|cmp|test|lea|add|sub|xor|and|or|push|pop|ret|nop|jmp|movs[sd]|ucomis[sd]|cvt\w+|imul|shl|shr|sar|set\w+|cmov\w+)\b', re.I)
INLINE_PRE = re.compile(r'(0\.[1-6]\.\d{1,2})(?:\s*(?:판|기준|에서|은|는|값|대응|대응값|구값|신값|현행|=|:|：|RVA|rva|exe))*\s*[`*(\[「]*\s*$')
SUFFIX_TAG = re.compile(r'^[`*\]」]*\s*[(（]\s*(0\.[1-6]\.\d{1,2})(?!\d)(?!\s*(?:판|기준|에서|=|:)?\s*[`*]*\s*(?:0x)?[0-9a-fA-F]{5,})')
OLD_PRE = re.compile(r'(?:(?<=[\s(（\[「·,|/~>])|^)(?:구|옛|old|was|이전값|구값|이전)\s*[=:：]?\s*[`*]*\s*$', re.I)
ARROW_AFTER = re.compile(r'^[`*\]」)]*\s*(?:[(（][^()（）\n]{0,40}[)）])?[`*]*\s*(?:→|->|=>|⇒|➜)\s*[`*\[「(]*\s*(?:0x)?[0-9a-fA-F]{5,}')
ARROW_BEFORE = re.compile(r'(?:→|->|=>|⇒|➜)\s*[`*\[「(]*\s*$')
RANGE_BEFORE = re.compile(r'(?:0x)?[0-9a-fA-F]{5,9}[`*]*\s*(?:\.\.|–|~)\s*[`*]*\s*$')
OFFSET_WORD = re.compile(r'(?<![A-Za-z0-9_])(?:오프셋|offset|off|프레임|frame|stack|스택|크기|size|len|길이|필드|field|stride|간격|delta|델타|tick|틱|ms|count|개수|횟수|cap|상한|budget|예산|seed|시드|mask|마스크|flag|플래그|bytes?|슬롯|slot|disp|변위)\s*[=:：]?\s*[`*]*\s*$', re.I)
# md 코드펜스 안 JSON 「"end": "0x…"」 = 창/범위 끝(JSON_END_KEYS 와 같은 규칙 · 2026-10-27 검증B 표본 C023/C035)
MD_JSON_END = re.compile(r'"(?:end|end_rva|stop|hi|limit|fn_end|end_addr)"\s*:\s*"?\s*$', re.I)
# 이름 후보가 산문 문장 머리의 조사 조각뿐인 경우(「결과는」「이후」「ESI는」「대상이 있으면」) — 이름이 아니다(검증B 표본 60건 중 12건)
WEAK_NAME = re.compile(r'^(?:[가-힣A-Za-z0-9]{0,6}\s?){1,2}(?:은|는|이|가|을|를|의|에서|와|과|로|으로|면|며|고|도|만|에|후|이후|뒤|다음|때)$')
JSON_KEY_NAME = re.compile(r'^"([A-Za-z_][\w.]*)"\s*:\s*"?$')
COMMIT_WORD = re.compile(r'(?:커밋|commit|소스|source|sha|해시|hash|@|tfm2mods|master|HEAD|브랜치|branch|rev|빌드|build)[^|]{0,14}$', re.I)
ROUND_CONST = re.compile(r'^(?:[1-9a-f]0{4,}|f{5,}|7f{4,}|([0-9a-f])\1{4,}|f+0{3,}|8+0{4,})$')
COMPARE_AFTER = re.compile(r'^\s*(?:와|과|하고|대비|와의|과의)?\s*(?:와|과)?\s*(?:동일|같|대비|비교|불변|그대로|유지|≡|==)')
COMPARE_BEFORE = re.compile(r'(?:≡|대비|比)\s*$')
MIG_PATH = re.compile(r'(마이그|migration|migrate|repin|_mig|mig_|이전_|이주)', re.I)
MIG_WORDS = re.compile(r'(구값|신값|이동|재핀|repin|STALE|대응|migrat|마이그|old|new|구\s|신\s)', re.I)
DATE_IN_PATH = re.compile(r'(?<!\d)(20\d\d)[-_]?(\d\d)[-_]?(\d\d)(?!\d)')
VERSION_HDR_KEY = re.compile(r'^\s*(버전|version|ver|게임\s*버전|game_?ver|게임)\s*$', re.I)
OLD_COL = re.compile(r'(?:^|[\s(*])(구|옛|old|이전|from|before|prev)(?:값|RVA|rva|주소|$|[\s)(*])', re.I)
DATA_WORDS = re.compile(r'(rdata|\.data|vt\b|vtable|vt\+|JT|점프\s*표|테이블|table|문자열|string|str_|상수|const|rip|lea\b|data_rva|전역|global|static)', re.I)

JSON_SKIP_KEYS = re.compile(r'(sha|hash|bytes|hex$|^hex|orig|pattern|sig$|signature|digest|md5|crc|uuid|session|commit'
                            r'|file_?off|raw_?off|fileoff)', re.I)   # 「rva32_file_offsets」= 파일 오프셋(RVA 아님 · 검증B 자체시험에서 발견)
JSON_END_KEYS = re.compile(r'^(end|end_rva|stop|hi|limit|fn_end|end_addr)$', re.I)
JSON_LIST_KEYS = re.compile(r'^(asm|disasm|insn|insns|instructions|listing|code|text|lines|dis)$', re.I)
JSON_RVA_KEYS = re.compile(r'(rva|addr|address|site|start|ret|caller|callee|target|entry|^fn$|func|function|owner|patch|branch|call|load|vt|vtable|getter|anchor|location|table|jump|dest)', re.I)
JSON_OLD_KEY = re.compile(r'(?:^|[_\-.])(old|prev|previous|legacy|migrated_from|from_ver|from_exe|from_rva|src_ver|before_mig|base_old)(?:$|[_\-.])', re.I)
OLD_VER_KEY = None   # main 에서 대상 버전 기준으로 채움(대상 미만 버전 숫자 키)
JSON_NAME_KEYS = ('name', 'id', 'tactic', 'knob', 'label', 'title', 'patch', 'role', 'what', 'field',
                  'site_name', 'key', 'tactic_id', 'knob_id', 'kind', 'desc', 'purpose', 'symbol')
JSON_VER_KEYS = ('ver', 'game_ver', 'version', 'exe_ver', 'game_version', 'base_version', 'exe_version')


# ───────────────────────── 공통 도우미 ─────────────────────────
def ver_tuple(v):
    return tuple(int(x) for x in v.split('.'))


def clean_md(s):
    s = s.replace('**', '').replace('`', '').replace('~~', '')
    s = re.sub(r'\[\[([^\]]+)\]\]', r'\1', s)
    s = re.sub(r'\s+', ' ', s)
    return s.strip(' *_-:·|,;=()（）[]「」→')


def game_tags(text):
    """(pos, end, ver, role) 목록. role = compare|tag"""
    out = []
    for m in VER_RE.finditer(text):
        v = m.group(1)
        pre = text[max(0, m.start() - 2):m.start()]
        if pre.endswith('v') or pre.endswith('V'):
            continue
        after = text[m.end():m.end() + 14]
        before = text[max(0, m.start() - 4):m.start()]
        role = 'compare' if (COMPARE_AFTER.match(after) or COMPARE_BEFORE.search(before)) else 'tag'
        out.append((m.start(), m.end(), v, role))
    return out


def path_marker(path_parts, target):
    """경로 조각(파일명 우선)에서 버전 표지 → ('target'|'other'|None, 표지)."""
    tv = target
    tcomp = target.replace('.', '')            # 065
    for part in path_parts:                    # 가까운 것부터
        hits = set()
        for m in re.finditer(r'(?<![0-9A-Za-z])(0[._]?[4-6][._]?\d{1,2})(?![0-9])', part):
            raw = m.group(1)
            dig = re.sub(r'[._]', '', raw)
            if len(dig) == 3:
                hits.add('0.%s.%s' % (dig[1], dig[2]))
            elif len(dig) == 4 and dig[1] == '4':
                hits.add('0.4.%s' % dig[2:])
        hits = {h for h in hits if ver_tuple(h) >= (0, 4, 0)}
        if hits:
            if hits == {tv}:
                return 'target', part
            if tv in hits:
                return 'mixed', part
            return 'other', part
    return None, None


class Ctx:
    def __init__(self, args, sha_full, sha_known):
        self.target = args.ver
        self.sha_full = sha_full.upper()
        self.sha16 = self.sha_full[:16]
        self.sha_known_other = {s.upper() for s in sha_known if s.upper() != self.sha16}
        self.since = args.since_dt
        self.stats = collections.Counter()
        self.occ = []          # 수용 후보 occurrence
        self.root_files = collections.Counter()
        self.root_hit_files = collections.Counter()

    def date_basis(self, path, rel_parts, use_ctime=True):
        """작성일 기준: 경로 날짜 > since 날짜 → date · 같은 날이면 ctime ≥ since · 경로 날짜 없으면 ctime.
        ⚠메모리·ANA 는 도구가 원자적으로 다시 써서 ctime == mtime 이 된다(실측 2026-10-27: 아틀라스·DONE.md·tfm2-banpick-ai.md)
        → 그 루트는 use_ctime=False(경로 날짜만). REPORT 는 편집 후에도 ctime 이 보존된다(실측)."""
        d = None
        for part in rel_parts:
            m = DATE_IN_PATH.search(part)
            if m:
                try:
                    d = datetime.date(int(m.group(1)), int(m.group(2)), int(m.group(3)))
                    break
                except ValueError:
                    pass
        born = None
        if use_ctime:
            try:
                born = datetime.datetime.fromtimestamp(os.stat(path).st_ctime)
            except OSError:
                born = None
        if d is not None:
            if d > self.since.date():
                return True
            if d < self.since.date():
                return False
        return born is not None and born >= self.since


# ───────────────────────── 토큰 귀속 ─────────────────────────
def split_cells(line):
    """표 행이면 [(lo,hi)] 칸 범위, 아니면 None."""
    s = line.rstrip()
    if not s.lstrip().startswith('|'):
        return None
    pos = [i for i, ch in enumerate(s) if ch == '|' and (i == 0 or s[i - 1] != '\\')]
    if len(pos) < 2:
        return None
    return [(pos[i] + 1, pos[i + 1]) for i in range(len(pos) - 1)]


def analyze_text(ctx, text, base_ver, base_basis, src, names_hint, listing, mig_doc,
                 table=None, cells=None, allow_bare=True, json_mode=False):
    """한 줄(또는 JSON 문자열)의 토큰을 귀속·필터해 ctx.occ 에 넣는다.
    base_ver/base_basis = 줄 밖 맥락(표 머리 전체/제목/문서)에서 온 버전·근거."""
    if not text or ('0x' not in text and not any(c in text for c in 'abcdefABCDEF')):
        return
    struck = [(m.start(), m.end()) for m in STRIKE_RE.finditer(text)]
    tags = game_tags(text)
    toks = []
    for m in TOK_RE.finditer(text):
        if m.group('fun'):
            v = int(m.group('funv'), 16) - IMAGE_BASE; nota = 'abs'
        elif m.group('abs'):
            v = int(m.group('absv'), 16) - IMAGE_BASE; nota = 'abs'
        elif m.group('hex'):
            v = int(m.group('hexv'), 16); nota = '0x'
        else:
            bv = m.group('barev')
            if not allow_bare or not re.search(r'[a-fA-F]', bv) or not re.search(r'\d', bv):
                continue
            if re.fullmatch(r'\d+[bB]', bv):      # 「29166B」= 바이트 수(검증B: serpen 재핀판정 0x29166b 오수확)
                ctx.stats['skip_offset_bytes'] += 1
                continue
            v = int(bv, 16); nota = 'bare'
        toks.append((m.start(), m.end(), v, nota))
    if not toks:
        return
    # 인라인 접두 태그(소비) 판정
    consumed = set()
    inline = {}
    for i, (s, e, v, nota) in enumerate(toks):
        left = text[max(0, s - 40):s]
        mm = INLINE_PRE.search(left)
        if mm:
            tagpos = s - (len(left) - mm.start(1))
            inline[i] = mm.group(1)
            consumed.add(tagpos)
    free_tags = [(p, e, v) for (p, e, v, role) in tags if role == 'tag' and p not in consumed
                 and not any(a <= p < b for a, b in struck)]
    # 다른 토큰 바로 뒤 「(0.6.4 …」 처럼 그 토큰에 묶인 접미 태그 — 섞인 경우 판단에서 뒤로 미룬다
    bound = set()
    for (s, e, v, nota) in toks:
        ms = SUFFIX_TAG.match(text[e:e + 60])
        if ms:
            bound.add(e + ms.start(1))
    free_unbound = [t for t in free_tags if t[0] not in bound]
    other_in_line = any(v != ctx.target for (_, _, v) in free_tags) or any(v != ctx.target for v in inline.values())
    filtered_prev = None
    for i, (s, e, v, nota) in enumerate(toks):
        ctx.stats['tok_' + nota] += 1
        left = text[max(0, s - 40):s]
        right = text[e:e + 60]
        hexs = '%x' % v if v >= 0 else ''
        # ── 필터(주소 아님) ──
        if v < 0x1000:                       # abs 표기의 이미지 베이스 자체(0x140000000) 등
            ctx.stats['skip_below_text'] += 1; continue
        if any(a <= s < b for a, b in struck):
            ctx.stats['skip_struck'] += 1; continue
        ch_before = text[s - 1] if s > 0 else ''
        # 경로 참조 「analysis_064/parallel/batch35/battle/0xdd1fb0/contract.md」 = 다른 분석 폴더 이름(그 버전의 주소).
        # 앞 조각이 hex 인 「fb51f3/fb51ff」·「0xd70494/0xd704a5」 나열은 경로가 아니다.
        if ch_before in '/\\' and re.search(r'(?:^|[^0-9A-Za-z_])(?!(?:0x)?[0-9a-fA-F]{5,9}[/\\]$)[\w.\-]*[A-Za-z_][\w.\-]*[/\\]$', left):
            wl = re.search(r'[^\s`"\'()（）\[\]|,;·]*$', text[:s]).group(0)
            wr = re.match(r'[^\s`"\'()（）\[\]|,;·]*', text[e:]).group(0)
            pm, _ = path_marker([wl + wr], ctx.target)
            if pm != 'target':
                ctx.stats['skip_path_ref'] += 1; continue
        # 「exe+0x1ca463d」「base+0x5019760」= 모듈+RVA 표기 → 오프셋 아님
        sign = re.search(r'[+\-−]\s*[`*]*$', left)
        modrel = sign and re.search(r'(?:exe|dll|base|image|module|모듈|기준)\s*\+\s*[`*]*$', left, re.I)
        why_off = None
        if sign and not modrel:
            why_off = 'skip_offset_sign'
        elif OFFSET_WORD.search(left):
            why_off = 'skip_offset_word'
        elif re.match(r'^\s*(?:B\b|bytes|바이트)', right):
            why_off = 'skip_offset_bytes'
        elif filtered_prev is not None and filtered_prev == i - 1 and re.search(r'(?:→|->|/|=>)\s*[`*]*\s*$', left):
            why_off = 'skip_offset_chain'
        if why_off:
            ctx.stats[why_off] += 1; filtered_prev = i; continue
        if ROUND_CONST.match(hexs) and nota != 'abs':
            ctx.stats['skip_round_const'] += 1; filtered_prev = i; continue
        if RANGE_BEFORE.search(left) or (not json_mode and MD_JSON_END.search(left)):
            ctx.stats['skip_range_end'] += 1; continue
        if nota == 'bare' and COMMIT_WORD.search(left):
            ctx.stats['skip_commit_ctx'] += 1; continue
        if OLD_PRE.search(left):
            ctx.stats['skip_old_prefix'] += 1; continue
        # ── 버전 귀속 ──
        ver, basis = None, None
        if i in inline:
            ver, basis = inline[i], 'inline'
        if ver is None:
            ms = SUFFIX_TAG.match(right)
            if ms:
                ver, basis = ms.group(1), 'suffix'
        if ver is None and table is not None and cells is not None:
            ci = next((k for k, (a, b) in enumerate(cells) if a <= s < b), None)
            if ci is not None:
                cv = table['colver'].get(ci)
                if cv == 'OLD':
                    ctx.stats['skip_old_column'] += 1; continue
                if cv:
                    ver, basis = cv, 'table'
                elif table.get('vercol') is not None and table['vercol'] < len(cells):
                    a, b = cells[table['vercol']]
                    rv = [t for t in game_tags(text[a:b]) if t[3] == 'tag']
                    if rv:
                        vs = {t[2] for t in rv}
                        ver, basis = (rv[0][2], 'table') if len(vs) == 1 else (None, None)
                    elif re.search(r'버전\s*무관|무관|n/?a', text[a:b], re.I):
                        ver, basis = None, 'agnostic'
                # 같은 칸의 단일 태그는 「버전무관」 행이라도 그 예시 값의 버전이다(DONE 교훈 행 「(0.6.5 tower_discipline 0xeee590…)」)
                if ver is None:
                    a, b = cells[ci]
                    ct = [(p, q, vv) for (p, q, vv) in free_tags if a <= p < b]
                    if ct:
                        vs = {t[2] for t in ct}
                        if len(vs) == 1:
                            ver, basis = ct[0][2], 'line'
        if ver is None and basis != 'agnostic' and free_tags:
            vs = {t[2] for t in free_tags}
            vu = {t[2] for t in free_unbound}
            if len(vs) == 1:
                ver, basis = free_tags[0][2], 'line'
            elif len(vu) == 1:
                ver, basis = free_unbound[0][2], 'line'
            else:
                pool = free_unbound or free_tags
                # 가장 가까운 태그(거리 동률이면 뒤쪽) — INDEX 「0xA (0.6.5)」 는 뒤, 산문 「0.5.6 함수 = 0xA」 는 앞
                near = [(t[0] - e if t[0] >= e else s - t[1], 0 if t[0] >= e else 1, t) for t in pool]
                near = [x for x in near if x[0] <= MIXED_WIN]
                pick = min(near)[2] if near else None
                if pick is None:
                    ctx.stats['skip_mixed_far'] += 1; continue
                ver, basis = pick[2], 'line-mixed'
        if ver is None and basis != 'agnostic':
            ver, basis = base_ver, base_basis
        if ver is None:
            ctx.stats['skip_no_version'] += 1; continue
        if ver != ctx.target:
            ctx.stats['skip_other_version'] += 1; continue
        # 화살표 왼쪽(이전→신규): 이주 맥락이면 제외 · 아니면(흐름 화살표일 수 있음) inferred 로 강등
        arrow_left = bool(ARROW_AFTER.match(right))
        if arrow_left:
            if basis not in ('inline', 'suffix') and (mig_doc or other_in_line or MIG_WORDS.search(text)
                                                      or basis in ('heading-mixed', 'doc-mixed')):
                ctx.stats['skip_arrow_old'] += 1; continue
            basis = basis + '+arrow_left'
        # 이름
        name = guess_name(text, s, e, cells, table) if not json_mode else None
        hint = list(names_hint or [])
        if not json_mode and name and hint:      # md: 제목 꼬리표(§…)는 이름을 못 찾았을 때만
            hint = [h for h in hint if not str(h).startswith('§')]
        nm = [n for n in ([name] if name else []) + hint if n]
        lst = listing or bool(LISTING_LINE.match(text) or LISTING_LINE2.match(text))
        ctx.occ.append(dict(rva=v, nota=nota, basis=basis, src=src, names=nm, listing=lst,
                            dctx=bool(nota != 'bare' or DATA_WORDS.search(text))))
        ctx.stats['occ_accepted'] += 1


NAME_SPLIT = re.compile(r'[·;,|(（)）\[\]「」/]|\.\s|(?:→|->)|0x[0-9a-fA-F]+'
                        r'|(?<![0-9A-Za-z_])[0-9a-fA-F]{5,7}(?![0-9A-Za-z_])')
PARTICLE = re.compile(r'^\s*[`*]*\s*(에서|의|로|으로|은|는|이|가|을|를|와|과|에|=|:)\s*')


def _nm(s):
    s = re.sub(r'(?<![0-9A-Za-z.])0\.[1-6]\.\d{1,2}(?!\d)', '', s)       # 버전 태그는 이름이 아니다
    jk = JSON_KEY_NAME.match(s.strip())
    if jk:                                       # md 코드펜스 JSON 「"anchor": "」 → 키 이름
        return jk.group(1)
    s = clean_md(s)
    s = re.sub(r'^[\s=:：…★⚠✓\-]+|[\s=:：…]+$', '', s)
    if WEAK_NAME.match(s) and not re.search(r'[A-Za-z_]{4,}', s):
        return ''                                # 조사로 끝나는 짧은 문장 머리 = 이름 아님
    return s if len(s) >= 2 and not HEX_ONLY.match(s) else ''


def guess_name(text, s, e, cells, table):
    """사람이 붙인 이름/역할 추정: 같은 칸(또는 구분자 사이)의 왼쪽 글 → 오른쪽 글 → 줄(칸) 머리. 표면 행 키·열 머리 덧붙임."""
    lo, hi = 0, len(text)
    rowkey = hdr = ''
    if cells:
        ci = next((k for k, (a, b) in enumerate(cells) if a <= s < b), None)
        if ci is not None:
            lo, hi = cells[ci]
            if ci != 0:
                rowkey = _nm(text[cells[0][0]:cells[0][1]])[:50]
            if table and table.get('hdr') and ci < len(table['hdr']):
                hdr = _nm(table['hdr'][ci])[:30]
    nm = _nm(NAME_SPLIT.split(text[lo:s])[-1])
    if not nm:                                   # 칸/줄 머리(첫 토큰 앞)
        m = TOK_RE.search(text, lo)
        pre = text[lo:m.start()] if m and m.start() < s + 1 else text[lo:s]
        nm = _nm(pre)[-60:]
    if not nm:                                   # 오른쪽 첫 의미 조각(40자 안) — 표 칸 안이거나 짧은 꼬리표일 때만
        for piece in NAME_SPLIT.split(text[e:min(hi, e + 40)]):
            nm = _nm(PARTICLE.sub('', piece))
            if nm and not cells and (len(nm) > 24 or re.search(r'[.다]\s*$|니다|이다|한다|있다|없다', nm)):
                nm = ''                          # 산문의 다음 문장 조각(검증B 표본 「C042의4개60 창은 이 후보 추가보다…」)
            if nm:
                break
    parts = [p for p in (rowkey, nm[-60:]) if p]
    out = ' / '.join(dict.fromkeys(parts))
    if hdr and out and hdr not in out:
        out += ' [' + hdr + ']'
    return out[:100] or (hdr or None)


# ───────────────────────── 문서 처리 ─────────────────────────
def file_ctx(ctx, path, root_abbr, root_dir):
    rel = os.path.relpath(path, root_dir)
    parts = rel.split(os.sep)
    near_first = list(reversed(parts))
    mk, part = path_marker(near_first, ctx.target)
    return rel, parts, mk


def header_decl(ctx, lines, born_after):
    """머리 30줄의 버전 선언. 대상 버전 이전에 태어난 문서(여러 버전이 쌓인 살아있는 문서)는
    머리말이 현행을 말해도 본문 전체를 대상 버전으로 보지 않는다(→ None · 줄/제목 태그만 인정)."""
    head = '\n'.join(lines[:30])
    up = head.upper()
    has_sha = ctx.sha16 in up
    other_sha = any(s in up for s in ctx.sha_known_other)
    tags = {t[2] for t in game_tags(head) if t[3] == 'tag'}
    if other_sha and not has_sha:
        return 'doc-other'
    if not born_after:
        return None
    if has_sha and not other_sha:
        return 'doc'
    if has_sha and other_sha:
        return 'doc-mixed'
    if tags == {ctx.target}:
        return 'doc'
    return None


def process_md(ctx, path, root_abbr, root_dir):
    try:
        raw = open(path, encoding='utf-8', errors='replace').read()
    except OSError as ex:
        ctx.stats['read_error'] += 1
        return
    rel, parts, mk = file_ctx(ctx, path, root_abbr, root_dir)
    lines = raw.split('\n')
    dated = ctx.date_basis(path, list(reversed(parts)), use_ctime=(root_abbr in CTIME_ROOTS))
    hd = header_decl(ctx, lines, dated)
    up = raw.upper()
    if mk == 'other':
        doc_ver, doc_basis = None, None
    elif mk == 'target':
        doc_ver, doc_basis = ctx.target, 'doc'
    elif hd == 'doc':
        doc_ver, doc_basis = ctx.target, 'doc'
    elif hd == 'doc-other':
        doc_ver, doc_basis = None, None
    elif hd == 'doc-mixed' or mk == 'mixed':
        doc_ver, doc_basis = ctx.target, 'doc-mixed'
    elif dated:
        doc_ver, doc_basis = ctx.target, 'date'
    else:
        doc_ver, doc_basis = None, None
    if doc_ver is None and ctx.target not in raw and ctx.sha16 not in up:
        ctx.stats['md_skipped_no_target'] += 1
        return
    ctx.stats['md_scanned'] += 1
    # 마이그 경로이거나 파일 어디든 다른 게임 버전이 나오면 화살표 왼쪽은 구값으로 본다
    mig_doc = bool(MIG_PATH.search(rel)) or any(t[2] != ctx.target for t in game_tags(raw))
    if mig_doc and doc_basis == 'doc':
        doc_basis = 'doc-multi'        # 대상 버전 문서지만 다른 버전 값도 섞여 있다(회차 정본·재핀 판정 등)
    src_pre = root_abbr + '\\' + rel
    heads = []          # [(level, ver, basis)]
    htitle = []         # [(level, 제목 글)] — 이름을 못 찾은 산문 토큰의 꼬리표(§제목)
    in_fence = False
    table = None
    hit_before = len(ctx.occ)
    for ln, line in enumerate(lines, 1):
        if FENCE_RE.match(line):
            in_fence = not in_fence
            continue
        if not in_fence:
            hm = HEADING_RE.match(line)
            if hm:
                lvl = len(hm.group(1))
                heads = [h for h in heads if h[0] < lvl]
                htitle = [h for h in htitle if h[0] < lvl]
                _ht = _nm(TOK_RE.sub('', hm.group(2)))[:60]
                if _ht:
                    htitle.append((lvl, _ht))
                ht = {t[2] for t in game_tags(hm.group(2)) if t[3] == 'tag'}
                if ht == {ctx.target}:
                    heads.append((lvl, ctx.target, 'heading'))
                elif ht and ctx.target in ht:
                    heads.append((lvl, ctx.target, 'heading-mixed'))
                elif ht:
                    heads.append((lvl, sorted(ht, key=ver_tuple)[-1], 'heading'))
                else:
                    heads.append((lvl, None, None))
                table = None
                continue
        cells = None if in_fence else split_cells(line)
        if cells is not None:
            if table is None:
                hdr = [line[a:b] for a, b in cells]
                colver = {}
                vercol = None
                allv = set()
                for k, h in enumerate(hdr):
                    hv = {t[2] for t in game_tags(h) if t[3] == 'tag'}
                    allv |= hv
                    if len(hv) == 1:
                        colver[k] = hv.pop()
                    elif OLD_COL.search(clean_md(h) + ' '):
                        colver[k] = 'OLD'
                    if VERSION_HDR_KEY.match(clean_md(h)):
                        vercol = k
                # 머리에 버전이 하나뿐이고 열 하나에만 있으면 표 전체 맥락으로 승격(아틀라스 「함수 (0.6.5) | RVA」)
                tv = None
                if len(allv) == 1 and len([c for c in colver.values() if c != 'OLD']) == 1:
                    tv = next(iter(allv))
                    colver = {k: c for k, c in colver.items() if c == 'OLD'}
                table = dict(hdr=hdr, colver=colver, vercol=vercol, tablever=tv, row=0)
                continue
            if re.match(r'^\s*\|?\s*:?-{2,}', line):
                continue
            table['row'] += 1
        else:
            table = None
        # 줄 밖 맥락
        bv, bb = None, None
        if table is not None and table.get('tablever'):
            bv, bb = table['tablever'], 'table'
        if bv is None:
            for lvl, hv, hb in reversed(heads):
                if hv is not None or hb is not None:
                    bv, bb = hv, hb
                    break
        if bv is None and not any(h[1] for h in heads):
            bv, bb = doc_ver, doc_basis
        allow_bare = bv == ctx.target or ctx.target in line
        analyze_text(ctx, line, bv, bb, src_pre + ':' + str(ln),
                     (['§' + htitle[-1][1]] if htitle else None), in_fence, mig_doc,
                     table=table, cells=cells, allow_bare=allow_bare)
    if len(ctx.occ) > hit_before:
        ctx.root_hit_files[root_abbr] += 1


def process_json(ctx, path, root_abbr, root_dir, max_mb):
    try:
        sz = os.path.getsize(path)
    except OSError:
        return
    if sz > max_mb * 1024 * 1024:
        ctx.stats['json_skipped_big'] += 1
        return
    try:
        raw = open(path, encoding='utf-8-sig', errors='replace').read()
        obj = json.loads(raw)
    except Exception:
        ctx.stats['json_parse_error'] += 1
        return
    rel, parts, mk = file_ctx(ctx, path, root_abbr, root_dir)
    up = raw.upper()
    has_sha = ctx.sha16 in up
    other_sha = any(s in up for s in ctx.sha_known_other)
    dated = ctx.date_basis(path, list(reversed(parts)), use_ctime=(root_abbr in CTIME_ROOTS))
    if mk == 'other':
        doc_ver, doc_basis = None, None
    elif mk == 'target' or (has_sha and not other_sha) or ('"%s"' % ctx.target in raw and not other_sha):
        doc_ver, doc_basis = ctx.target, 'doc'
    elif other_sha and not has_sha:
        doc_ver, doc_basis = None, None
    elif has_sha and other_sha:
        doc_ver, doc_basis = ctx.target, 'doc-mixed'
    elif dated:
        doc_ver, doc_basis = ctx.target, 'date'
    else:
        doc_ver, doc_basis = None, None
    ctx.stats['json_scanned'] += 1
    nl = [i for i, ch in enumerate(raw) if ch == '\n']
    cursor = [0]
    src_pre = root_abbr + '\\' + rel
    mig_doc = bool(MIG_PATH.search(rel))
    hit_before = len(ctx.occ)

    def lineno(sval):
        cands = [json.dumps(sval, ensure_ascii=False), json.dumps(sval)]
        for c in cands:
            p = raw.find(c, cursor[0])
            if p >= 0:
                cursor[0] = p + len(c)
                return bisect.bisect_right(nl, p) + 1
        return None

    def dict_ctx(d, pver, pbasis):
        for k in JSON_VER_KEYS:
            if k in d and isinstance(d[k], str):
                t = [x for x in game_tags(d[k]) if x[3] == 'tag']
                if t:
                    return t[0][2], 'inline'
        for k, val in d.items():
            if isinstance(val, str) and 'sha' in k.lower() and len(val) >= 16:
                u = val.upper()
                if u.startswith(ctx.sha16):
                    return ctx.target, 'doc'
                if any(u.startswith(s) for s in ctx.sha_known_other):
                    return 'OTHER', 'doc'
        return pver, pbasis

    def names_of(d):
        out = []
        for k in JSON_NAME_KEYS:
            vv = d.get(k)
            if isinstance(vv, (str, int)) and not isinstance(vv, bool):
                s = str(vv)
                if s and not HEX_ONLY.match(s) and len(s) <= 80:
                    out.append(s)
            if len(out) >= 2:
                break
        return out

    def walk(o, path_keys, pver, pbasis, nhint, listing):
        if isinstance(o, dict):
            v2, b2 = dict_ctx(o, pver, pbasis)
            nh = names_of(o) or nhint
            for k, val in o.items():
                ks = str(k)
                if JSON_SKIP_KEYS.search(ks):
                    continue
                if JSON_OLD_KEY.search(ks):
                    ctx.stats['skip_json_old_key'] += 1
                    continue
                kt = [x for x in game_tags(ks) if x[3] == 'tag']
                kv, kb = (kt[0][2], 'inline') if kt else (v2, b2)
                if not kt and OLD_VER_KEY.search(ks.lower()) and ctx.target.replace('.', '') not in ks:
                    ctx.stats['skip_json_old_key'] += 1
                    continue
                walk(val, path_keys + [ks], kv, kb, nh, listing or bool(JSON_LIST_KEYS.match(ks)))
        elif isinstance(o, list):
            for val in o:
                walk(val, path_keys, pver, pbasis, nhint, listing)
        elif isinstance(o, str):
            if '0x' not in o and not re.search(r'[0-9a-fA-F]{5,}', o):
                return
            key = path_keys[-1] if path_keys else ''
            if JSON_END_KEYS.match(key):
                ctx.stats['skip_range_end'] += 1
                return
            ver, basis = (None, None) if pver == 'OTHER' else (pver, pbasis)
            ln = lineno(o)
            src = src_pre + ':' + (str(ln) if ln else '$' + '.'.join(path_keys[-3:]))
            label = '.'.join(path_keys[-2:])
            hint = [h for h in ([label] + list(nhint or [])) if h]
            m = HEX_ONLY.match(o)
            if m and len(m.group(1)) <= 7 and not o.strip().startswith('0x'):
                # 0x 없는 hex 전체값 — RVA 키일 때만
                if not JSON_RVA_KEYS.search(key):
                    return
                o2 = '0x' + m.group(1)
            else:
                o2 = o
            analyze_text(ctx, o2, ver, basis, src, hint, listing, mig_doc, allow_bare=(ver == ctx.target),
                         json_mode=True)
        elif isinstance(o, int) and not isinstance(o, bool):
            key = path_keys[-1] if path_keys else ''
            if not re.search(r'(rva|addr|address)$', key, re.I) or JSON_END_KEYS.match(key):
                return
            if pver == 'OTHER' or pver != ctx.target:
                ctx.stats['skip_other_version' if pver else 'skip_no_version'] += 1
                return
            if not (0x1000 <= o < IMAGE_LIMIT[0]):          # 힙 포인터·작은 정수 = RVA 주장 아님
                ctx.stats['skip_json_int_out_of_image'] += 1
                return
            p = raw.find(str(o), cursor[0])
            ln = bisect.bisect_right(nl, p) + 1 if p >= 0 else None
            if p >= 0:
                cursor[0] = p
            src = src_pre + ':' + (str(ln) if ln else '$' + '.'.join(path_keys[-3:]))
            ctx.occ.append(dict(rva=o, nota='json_int', basis=pbasis, src=src,
                                names=['.'.join(path_keys[-2:])] + list(nhint or []), listing=listing, dctx=True))
            ctx.stats['occ_accepted'] += 1

    walk(obj, [], doc_ver, doc_basis, None, False)
    if len(ctx.occ) > hit_before:
        ctx.root_hit_files[root_abbr] += 1


def iter_files(root, exts, excl_dirs=('_archive', '__pycache__', '.git', 'node_modules')):
    for dp, dn, fn in os.walk(root):
        dn[:] = [d for d in dn if d not in excl_dirs and not d.startswith('.')]
        for f in fn:
            if os.path.splitext(f)[1].lower() in exts:
                yield os.path.join(dp, f)


# ───────────────────────── exe 검증 ─────────────────────────
class Exe:
    def __init__(self, path):
        d, ib, secs, ex_rva, ex_sz = fnindex.load(path)
        self.d, self.ib, self.secs = d, ib, secs
        po = fnindex.roff(secs, ex_rva)
        n = ex_sz // 12
        arr = struct.unpack_from('<%dI' % (n * 3), d, po)
        self.beg = list(arr[0::3]); self.end = list(arr[1::3]); unw = arr[2::3]
        order = sorted(range(n), key=lambda i: self.beg[i])
        self.beg = [self.beg[i] for i in order]; self.end = [self.end[i] for i in order]
        unw = [unw[i] for i in order]
        self.root = {}
        idx_of = {b: i for i, b in enumerate(self.beg)}
        nchain = 0
        for i, u in enumerate(unw):
            off = fnindex.roff(secs, u & ~1) if u else None
            if off is None:
                continue
            flags = d[off] >> 3
            if flags & 0x4:                                       # UNW_FLAG_CHAININFO
                cnt = d[off + 2]
                co = off + 4 + ((cnt + 1) & ~1) * 2
                pb = struct.unpack_from('<I', d, co)[0]
                self.root[self.beg[i]] = pb
                nchain += 1
        self.nchain = nchain
        self.sect = {nm: (va, vsz) for nm, va, vsz, _, _ in secs}
        ss = sorted(secs, key=lambda x: x[1])
        self.sect_sorted = [(nm, va, vsz, (ss[k + 1][1] if k + 1 < len(ss) else va + vsz))
                            for k, (nm, va, vsz, _, _) in enumerate(ss)]
        self.md = Cs(CS_ARCH_X86, CS_MODE_64)
        self.md.detail = False
        self.bcache = {}
        self.icache = {}

    def root_of(self, b):
        seen = 0
        while b in self.root and seen < 16:
            b = self.root[b]; seen += 1
        return b

    def section(self, rva):
        for nm, va, vsz, nxt in self.sect_sorted:
            if va <= rva < va + vsz:
                return nm, False
            if va + vsz <= rva < nxt:
                return nm, True                                   # 섹션 정렬 꼬리
        return None, False

    def bounds(self, start, stop):
        key = (start, stop)
        if key in self.bcache:
            return self.bcache[key]
        off = fnindex.roff(self.secs, start)
        code = self.d[off:off + (stop - start)]
        s = set(); last = start; lst = []
        for (a, sz, mn, op) in self.md.disasm_lite(code, start):
            s.add(a); last = a + sz; lst.append((a, sz, mn, op))
        self.bcache[key] = (s, last)
        self.icache[key] = lst
        return s, last

    def containing(self, start, stop, rva):
        """rva 를 품은 명령(시작, 길이, 니모닉, 피연산자) — 즉치값·변위 바이트 패치 지점 설명용."""
        lst = self.icache.get((start, stop)) or []
        k = bisect.bisect_right([x[0] for x in lst], rva) - 1
        if 0 <= k < len(lst) and lst[k][0] <= rva < lst[k][0] + lst[k][1]:
            return lst[k]
        return None

    def classify(self, rva):
        """→ (kind, owner, note) 또는 (None, None, why). 명령 중간이면 self.last_insn = (명령 시작, 오프셋, owner)"""
        self.last_insn = None
        if rva < 0x1000:
            return None, None, 'out_of_range(header)'
        nm, tail = self.section(rva)
        if nm is None:
            return None, None, 'out_of_range(image)'
        if nm == '.rdata':
            return 'RDATA', None, ('rdata_align_tail' if tail else None)
        if nm == '.data':
            return 'DATA', None, ('data_align_tail' if tail else None)
        if nm != '.text':
            return None, None, 'section_%s' % nm.strip('.')
        if tail:
            return None, None, 'text_align_tail'
        i = bisect.bisect_right(self.beg, rva) - 1
        if i >= 0 and self.beg[i] <= rva < self.end[i]:
            b = self.beg[i]
            r = self.root_of(b)
            if rva == b:
                if r == b:
                    return 'FN_START', b, None
                return 'MID', r, 'chained_chunk_start(0x%x)' % b
            s, last = self.bounds(b, self.end[i])
            if rva in s:
                return 'MID', r, (None if r == b else 'chunk 0x%x' % b)
            ins = self.containing(b, self.end[i], rva)
            self.last_insn = (ins[0], rva - ins[0], r) if ins else None
            if ins:
                return None, None, 'not_insn_boundary(owner 0x%x · 명령 0x%x+%d %s %s)' % (r, ins[0], rva - ins[0], ins[2], ins[3])
            if rva >= last:
                return None, None, 'not_insn_boundary(owner 0x%x · 선형 디스어셈 0x%x 에서 중단)' % (r, last)
            return None, None, 'not_insn_boundary(owner 0x%x)' % r
        # .pdata 밖 — 앞 조각 끝 이후 패딩 건너뛰기
        if i >= 0 and rva == self.end[i]:
            return 'MID', self.root_of(self.beg[i]), 'fn_end(0x%x)' % self.beg[i]
        gstart = self.end[i] if i >= 0 else self.sect['.text'][0]
        gstop = self.beg[i + 1] if i + 1 < len(self.beg) else self.sect['.text'][0] + self.sect['.text'][1]
        off = fnindex.roff(self.secs, gstart)
        p = gstart
        while p < rva and self.d[off + (p - gstart)] in (0xCC, 0x90, 0x00):
            p += 1
        if p == rva:
            return 'FN_START', rva, 'nopdata'
        if p > rva:
            return None, None, 'padding'
        s, last = self.bounds(p, gstop)
        if rva in s:
            # 한 .pdata 틈에 leaf 가 여럿 붙어 있을 수 있다(검증B 표본: vtable 대상 0xfe4d30·0x17217d0 이
            # 앞 leaf 의 MID 로 오분류). 종결(ret/jmp) 뒤 int3(CC) 패딩을 건너뛴 첫 바이트 = 새 leaf 시작.
            leaf = p
            lst = self.icache.get((p, gstop)) or []
            for k, (a, sz, mn, op) in enumerate(lst):
                if a >= rva:
                    break
                if mn in ('ret', 'retf', 'jmp') and k + 1 < len(lst):
                    # (leaf = 호출·프레임 없는 함수라 「call noreturn; int3」 같은 함수 안 CC 는 없다 — 종결+CC 만 경계)
                    q = a + sz
                    o2 = fnindex.roff(self.secs, q)
                    npad = 0
                    while q + npad < rva + 1 and self.d[o2 + npad] == 0xCC:
                        npad += 1
                    if npad:
                        leaf = q + npad
            if leaf == rva:
                return 'FN_START', rva, 'nopdata(leaf · 앞 leaf 0x%x 뒤 패딩)' % p
            if leaf > rva:
                return None, None, 'padding'
            return 'MID', leaf, 'nopdata(leaf 0x%x)' % leaf
        return None, None, 'not_insn_boundary(nopdata gap 0x%x)' % p


# ───────────────────────── 매니페스트 ─────────────────────────
def manifest_values(mdir, ver):
    out = collections.defaultdict(set)
    for f in sorted(glob.glob(os.path.join(mdir, '*.json'))):
        try:
            m = json.load(open(f, encoding='utf-8'))
        except Exception:
            continue
        mod = os.path.splitext(os.path.basename(f))[0]
        for e in m.get('entries') or []:
            if e.get('ver') != ver or e.get('ignore') or e.get('kind') == 'NOT_RVA':
                continue
            try:
                out[int(str(e.get('value')), 16)].add(mod)
            except (TypeError, ValueError):
                pass
    return out


def known_shas(mods_dir):
    out = set()
    for f in glob.glob(os.path.join(mods_dir, 'MIG', '_aidiff_*_*.pkl')):
        m = re.match(r'_aidiff_([0-9a-f]{16})_', os.path.basename(f))
        if m:
            out.add(m.group(1).upper())
    return out


# ───────────────────────── 메인 ─────────────────────────
def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('--ver', required=True, help='대상 게임 버전(예 0.6.5)')
    ap.add_argument('--exe', required=True, help='대상 버전 exe')
    ap.add_argument('--since', required=True, help='대상 버전 설치 시각 YYYY-MM-DD[THH:MM] (작성일 기준 판정)')
    ap.add_argument('--out', required=True)
    ap.add_argument('--mem', default=DEF_MEM)
    ap.add_argument('--ana', default=DEF_ANA)
    ap.add_argument('--report', default=DEF_REP)
    ap.add_argument('--manifest-dir', default=DEF_MAN)
    ap.add_argument('--other-sha', action='append', default=[], help='다른 버전 exe sha256 앞16(반복 가능 · MIG\\_aidiff_*.pkl 이름에서 자동 수집도 함)')
    ap.add_argument('--max-json-mb', type=float, default=64)
    ap.add_argument('--no-bare', action='store_true', help='0x 없는 맨 hex 후보 끄기')
    ap.add_argument('--prev-exe', help='직전 버전 exe(선택). 태그 없이 맥락으로만 귀속된 값이 직전 버전의 함수 시작이고 '
                                       '대상 버전에선 시작이 아니면 구값으로 보고 기각한다')
    args = ap.parse_args()
    s = args.since
    args.since_dt = datetime.datetime.fromisoformat(s if 'T' in s else s + 'T00:00')
    global OLD_VER_KEY
    tv = ver_tuple(args.ver)
    olds = ['0\\.%d\\.%d|0%d%d|0_%d_%d' % (a, b, a, b, a, b) for a in (4, 5, 6) for b in range(0, 20)
            if (0, a, b) < tv and b < 10] + ['0\\.%d\\.%d' % (a, b) for a in (4, 5, 6) for b in range(10, 20) if (0, a, b) < tv]
    OLD_VER_KEY = re.compile(r'(?<![0-9a-z])(?:%s)(?![0-9a-z])' % '|'.join(olds))

    t0 = time.time()
    sha = hashlib.sha256(open(args.exe, 'rb').read()).hexdigest().upper()
    others = known_shas(MODS) | {x.upper()[:16] for x in args.other_sha}
    ctx = Ctx(args, sha, others)
    print('[exe] %s sha256=%s' % (args.exe, sha))
    print('[ctx] 다른 버전 sha16 %d개 · since=%s' % (len(ctx.sha_known_other), args.since_dt))
    exe = Exe(args.exe)
    IMAGE_LIMIT[0] = max(va + vsz for _, va, vsz, _, _ in exe.secs)
    print('[pdata] %d 조각 · chained %d' % (len(exe.beg), exe.nchain))
    prev_starts = None
    if args.prev_exe:
        pe_ = Exe(args.prev_exe)
        prev_starts = {b for b in pe_.beg if pe_.root_of(b) == b}
        print('[prev] %s 함수 시작 %d' % (args.prev_exe, len(prev_starts)))
        del pe_

    since_ts = args.since_dt.timestamp()
    roots = [('MEM', args.mem, {'.md'}), ('ANA', args.ana, {'.md'}), ('REPORT', args.report, {'.md', '.json'})]
    for abbr, rd, exts in roots:
        n0 = len(ctx.occ)
        for p in iter_files(rd, exts):
            ext = os.path.splitext(p)[1].lower()
            ctx.root_files[abbr] += 1
            try:
                mt = os.stat(p).st_mtime
            except OSError:
                continue
            # 대상 버전 설치 이전에 마지막으로 고쳐진 파일엔 대상 버전 사실이 있을 수 없다
            if mt < since_ts - 86400:
                ctx.stats['skip_mtime_%s' % ext.strip('.')] += 1
                continue
            if ext == '.md':
                process_md(ctx, p, abbr, rd)
            else:
                process_json(ctx, p, abbr, rd, args.max_json_mb)
        print('[%s] 파일 %d · 수용 occurrence %d · %.0fs' % (abbr, ctx.root_files[abbr], len(ctx.occ) - n0, time.time() - t0),
              flush=True)

    if args.no_bare:
        ctx.occ = [o for o in ctx.occ if o['nota'] != 'bare']

    # 병합
    by = collections.OrderedDict()
    for o in ctx.occ:
        r = by.setdefault(o['rva'], dict(srcs=collections.OrderedDict(), names=collections.Counter(),
                                         basis=set(), nota=set(), listing=[], dctx=False))
        r['srcs'].setdefault(o['src'], 1)
        for n in o['names']:
            r['names'][n] += 1
        r['basis'].add(o['basis'])
        r['nota'].add(o['nota'])
        r['listing'].append(o['listing'])
        r['dctx'] = r['dctx'] or o['dctx']
    print('[merge] 고유 rva %d' % len(by), flush=True)

    man = manifest_values(args.manifest_dir, args.ver)
    rows, rej = [], []
    EXPL = ('inline', 'suffix', 'table', 'line', 'heading', 'doc', 'doc-multi')
    # 직전 버전 구값 판별은 버전이 섞인 맥락에서만(순수 대상 버전 문서에 적용하면 우연 일치 ≈0.3% 로 오기각 — 실측 0xdc2bc0)
    MIXED_BASIS = ('heading-mixed', 'doc-mixed', 'line-mixed', 'doc-multi')
    for k, rva in enumerate(sorted(by)):
        r = by[rva]
        kind, owner, note = exe.classify(rva)
        srcs = list(r['srcs'])
        STRONG = ('inline', 'suffix', 'table', 'line', 'heading')
        if kind in ('RDATA', 'DATA') and not r['dctx']:
            kind, note = None, 'bare_data_unverified(맨 hex 가 데이터 섹션 — 명령 경계로 확인 불가·데이터 문맥 어휘 없음)'
        elif (kind is not None and kind != 'FN_START' and prev_starts is not None and rva in prev_starts
              and not any(b in STRONG for b in r['basis'])
              and any(b in MIXED_BASIS or b.endswith('+arrow_left') for b in r['basis'])):
            kind, note = None, 'prev_version_fn_start(태그 없이 맥락 귀속 · 직전 버전 함수 시작 = 구값 추정 · 대상 판정 %s)' % note
        if kind is None:
            rj = dict(rva='0x%x' % rva, source=srcs[0], why=note, notation=sorted(r['nota']),
                      n=len(srcs), sources=srcs[:5], manifest=sorted(man.get(rva, ())), basis=sorted(r['basis']))
            if prev_starts is not None:
                rj['prev_fn_start'] = rva in prev_starts
            if exe.last_insn and note.startswith('not_insn_boundary'):
                # 즉치값·변위 바이트 패치 지점일 수 있다(실측: comptest RPLY 0xd0af9 = mov rbx,-1 의 +3) — 추적은 명령 시작으로
                rj['insn_start'] = '0x%x' % exe.last_insn[0]
                rj['insn_off'] = exe.last_insn[1]
                rj['owner'] = '0x%x' % exe.last_insn[2]
            rej.append(rj)
            continue
        conf = 'explicit' if any(b in EXPL for b in r['basis']) else 'inferred'
        names = [n for n, _ in r['names'].most_common(10)]
        row = dict(rva='0x%x' % rva, kind=kind, owner=('0x%x' % owner if owner is not None else None),
                   names=names, sources=srcs, confidence=conf, basis=sorted(r['basis']),
                   notation=sorted(r['nota']), listing_only=all(r['listing']),
                   src_types=sorted({('json' if '.json:' in x.lower() else 'md') for x in srcs}),
                   manifest=sorted(man.get(rva, ())))
        if note:
            row['note'] = note
        rows.append(row)
        if k % 2000 == 0 and k:
            print('  ... 검증 %d/%d %.0fs' % (k, len(by), time.time() - t0), flush=True)

    # 통계
    st = collections.OrderedDict()
    st['files_seen'] = dict(ctx.root_files)
    st['files_with_hits'] = dict(ctx.root_hit_files)
    rr = collections.Counter(); rr_only = collections.Counter()
    for row in rows:
        rts = {s.split('\\', 1)[0] for s in row['sources']}
        for x in rts:
            rr[x] += 1
        if len(rts) == 1:
            rr_only[next(iter(rts))] += 1
    st['rows'] = len(rows)
    st['rows_by_root'] = dict(rr)
    st['rows_only_in_root'] = dict(rr_only)
    st['confidence'] = dict(collections.Counter(r['confidence'] for r in rows))
    st['kind'] = dict(collections.Counter(r['kind'] for r in rows))
    st['notation'] = dict(collections.Counter(n for r in rows for n in r['notation']))
    st['rows_bare_only'] = sum(1 for r in rows if r['notation'] == ['bare'])
    st['rows_with_md_source'] = sum(1 for r in rows if 'md' in r['src_types'])
    st['rows_md_explicit_non_listing'] = sum(1 for r in rows if 'md' in r['src_types'] and r['confidence'] == 'explicit'
                                             and not r['listing_only'])
    st['rejected_in_manifest'] = sum(1 for x in rej if x['manifest'])
    st['rejected_prev_fn_start'] = sum(1 for x in rej if x.get('prev_fn_start'))
    st['rejected_inside_insn'] = sum(1 for x in rej if 'insn_start' in x)
    st['listing_only'] = sum(1 for r in rows if r['listing_only'])
    st['rejected'] = len(rej)
    st['rejected_why'] = dict(collections.Counter(re.sub(r'\(.*', '', x['why']) for x in rej))
    st['rejected_non_bare'] = sum(1 for x in rej if x['notation'] != ['bare'])
    mset = set(man)
    rset = {int(r['rva'], 16) for r in rows}
    st['manifest_ver_values'] = len(mset)
    st['manifest_overlap'] = len(rset & mset)
    st['manifest_overlap_by_mod'] = dict(collections.Counter(m for r in rows for m in r['manifest']))
    st['rows_outside_manifest'] = len(rset - mset)
    st['rows_outside_manifest_explicit_non_listing'] = sum(
        1 for r in rows if not r['manifest'] and r['confidence'] == 'explicit' and not r['listing_only'])
    st['manifest_values_not_in_kb'] = len(mset - rset)
    st['occurrence_filters'] = dict(sorted(ctx.stats.items()))
    st['elapsed_s'] = round(time.time() - t0, 1)
    out = collections.OrderedDict()
    out['game_ver'] = args.ver
    out['exe_sha256'] = sha
    out['generated'] = datetime.datetime.now().isoformat(timespec='seconds')
    out['generator'] = 'MIG\\patchday\\kb_harvest.py'
    out['args'] = dict(exe=args.exe, prev_exe=args.prev_exe, since=str(args.since_dt), mem=args.mem, ana=args.ana, report=args.report,
                       manifest_dir=args.manifest_dir, no_bare=args.no_bare)
    out['schema_note'] = ('kind=FN_START|MID|RDATA|DATA(.data 확장) · owner=.pdata 루트 시작(RDATA/DATA=null) · '
                          'confidence: explicit=inline/suffix/table/line/heading/doc/doc-multi 근거 1개 이상, inferred=그 밖 · '
                          'basis=귀속 근거 집합(+arrow_left=이전→신규 화살표 왼쪽) · notation=0x|abs|bare|json_int · '
                          'listing_only=디스어셈 열/코드펜스 안에서만 등장 · src_types=md|json · manifest=같은 값을 ver==대상으로 가진 매니페스트 · '
                          'rejected.insn_start/insn_off=명령 중간(즉치값 패치 지점일 수 있음) · rejected.prev_fn_start=직전 버전 함수 시작')
    out['rows'] = rows
    out['rejected'] = rej
    out['stats'] = st
    os.makedirs(os.path.dirname(os.path.abspath(args.out)), exist_ok=True)
    with open(args.out, 'w', encoding='utf-8') as f:
        json.dump(out, f, ensure_ascii=False, indent=1)
    print('[out] %s rows=%d rejected=%d %.0fs' % (args.out, len(rows), len(rej), time.time() - t0))
    print(json.dumps({k: v for k, v in st.items() if k != 'occurrence_filters'}, ensure_ascii=False))
    print(json.dumps(st['occurrence_filters'], ensure_ascii=False))


if __name__ == '__main__':
    main()
