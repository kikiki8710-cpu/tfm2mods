# 배포용 설명서 생성: 구조 서술(수기) + 설정값 전수(편집기 소스에서 자동 추출).
#  ★함수 주소·소스 파일명·내부 문서 참조는 전부 제거한다(배포용).
import re, io, sys, os
sys.stdout.reconfigure(encoding="utf-8")

HERE = os.path.dirname(os.path.abspath(__file__))
SRC  = os.path.join(HERE, "src", "main.rs")
OUT  = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust\설명서.md"
NARR = os.path.join(HERE, "manual_intro.md")

s = io.open(SRC, encoding="utf-8").read()
STR = re.compile('"([^"\\\\]*(?:\\\\.[^"\\\\]*)*)"')
TAB = re.compile(r'Tab\{ id:"([a-z0-9_]+)", title:"([^"]*)", keys:&\[(.*?)\], note:\s*(".*?")\s*\},', re.S)

def clean(t):
    """HTML 태그·줄이음 제거 + 배포에 부적절한 내부 참조 삭제."""
    t = t.replace("\\\n", " ").replace("\\n", " ")
    t = re.sub(r'<br\s*/?>', "\n", t)
    t = re.sub(r'<[^>]+>', "", t)
    t = t.replace("&nbsp;", " ").replace("&quot;", '"').replace("&amp;", "&")
    t = t.replace("&lt;", "<").replace("&gt;", ">")
    t = re.sub(r'`([^`]*)`', r'\1', t)
    # 내부 참조 제거
    t = re.sub(r'0x[0-9a-fA-F]{4,}', "", t)                       # 함수 주소
    t = re.sub(r'[A-Za-z_][A-Za-z0-9_]*\.rs(:\d+(~\d+)?)?', "", t) # 소스 파일
    t = re.sub(r'[a-z0-9_]+_imm\.txt', "적용확인 로그", t)          # 진단 파일명
    t = re.sub(r'적용확인\s*=\s*적용확인 로그(\s*·\s*적용확인 로그)*\.?', "", t)
    t = re.sub(r'\(원본\s*\)', "", t)
    # ── 내부 용어 → 일반 용어 (배포용) ──
    for pat, rep in [
        (r'(\d+)\s*사이트', r'\1곳'), (r'사이트', '자리'),
        (r'일괄 패치', '한꺼번에 적용'), (r'바이트\s*패치|byte-patch', '값 교체'), (r'패치', '적용'),
        (r'branch\s*[ABC]', '경로'), (r'사본\s*\[[ABC]\]', '사본'),
        (r'\bdisc\s*(\d+)', r'판단 \1'), (r'\bdisc(\d+)', r'판단 \1'),
        (r'sub_plan', '실행 단위'), (r'\bRVA\b', '위치'), (r'\bimm\b', '값'),
        (r'레버리지', '영향 범위'), (r'레버', '조절값'),
        (r'위협 평가 정본', '위협 평가 본체'), (r'평가 정본', '평가 본체'),
        (r'정본 함수 전용', '본체 전용'), (r'정본', '본체'),
        (r'imm32[^)]*', '내부 표현 한계로 매우 큰 값은 못 넣습니다'),
        (r'\(\s*코드\s*0x[0-9a-fA-F]+\s*\)', ''), (r'\(0x[0-9a-fA-F]+\)', ''),
        (r'원본\s*0x[0-9a-fA-F]+\s*=\s*', '원본 '), (r'원본\s*0x[0-9a-fA-F]+', '원본'),
        (r'â\[ì¬ë°°ì \]\s*', ''), (r'â', ''), (r'param(\d)', r'í­ëª© '),
        (r'HOLD', 'ëê¸° ì ì§'),
        # ââ ê°ë° ì´ë ¥ íí ì ê±°(ë°°í¬ì©) ââ
        (r'â \[ì ì \][^.]*?\.', ''), (r'\(êµ¬ [^)]*ì¤ê¸°\)', ''), (r'\(êµ¬ ë¬¸ì[^)]*\)', ''),
        (r'\[ì¬ë°°ì \]\s*', ''), (r'âì»¤ë²ë¸ë¡ì´ ë§í ìì´ ê°ì´ ì ë¨¹ìì', ''),
        (r'â³ë¶ë¶ë°ì\s*â\s*', ''), (r'êµ¬ .{0,12}íê¸°ë[^.]*\.', ''),
        (r'subplan\s*(\d+)', r'ì¤í ë¨ì '), (r'subplan', 'ì¤í ë¨ì'),
        (r'\[공통\]\s*', ''), (r'\[실행\]\s*', ''), (r'\[상위\]\s*', ''), (r'\[기타\]\s*', ''),
        (r'\[매크로\]\s*', ''), (r'\[\d+[·\d]*\]\s*', ''), (r'게임 원본\s+[a-z_]+\)', '게임 원본)'),
    ]:
        t = re.sub(pat, rep, t)
    t = re.sub(r'[ \t]{2,}', " ", t)
    t = re.sub(r'\n{3,}', "\n\n", t)
    return t.strip(" ·\n")

def keys_of(fn):
    i = s.index("fn %s(" % fn); j = s.index("\n}", i)
    d = {}
    for m in re.finditer(r'"([a-z0-9_]+)" => "((?:[^"\\]|\\.)*)"', s[i:j]):
        d.setdefault(m.group(1), m.group(2))
    return d
DESC = keys_of("desc_static")
ORIG = keys_of("orig_val")

# 배포 기본값 = default.txt
DEF = {}
dp = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust\config\default.txt"
if os.path.exists(dp):
    for ln in io.open(dp, encoding="utf-8"):
        ln = ln.strip()
        if ln and not ln.startswith("#") and " = " in ln:
            k, v = ln.split(" = ", 1); DEF[k.strip()] = v.strip()

TOGGLE = set(re.findall(r'"([a-z0-9_]+)"', s[s.index("fn is_toggle("): s.index("\n}", s.index("fn is_toggle("))]))
DEAD   = set(re.findall(r'"([a-z0-9_]+)"', s[s.index("fn is_dead("):   s.index("\n}", s.index("fn is_dead("))]))

out = [io.open(NARR, encoding="utf-8").read().rstrip(), "", "---", "", "# 설정값 전체 목록", "",
       "> 표기 — **기본**은 배포 상태의 값, **원본**은 게임 원래 값입니다.",
       "> 기본이 `-1`인 항목은 “게임 원래대로 두기”라는 뜻이라, 그대로 두면 아무것도 바뀌지 않습니다.",
       "> 체크 항목은 **켬/끔**으로 적었습니다.", ""]

nk = 0
for m in TAB.finditer(s):
    tid, title, body, note = m.group(1), m.group(2), m.group(3), m.group(4)
    items = STR.findall(body)
    ks = [i for i in items if not i.startswith("\u00a7")]
    if not ks: continue
    t = clean(title).lstrip("• ").strip()
    t = re.sub(r'^\[[^\]]*\]\s*', '', t)            # [0·1·3] 같은 내부 번호
    t = re.sub(r'\s*\([^)]*\)\s*$', '', t)           # (passive_line) 같은 내부 이름
    t = re.sub(r'\s*\([A-Za-z][^)]*$', '', t)         # 괄호가 안 닫힌 경우
    out.append("## %s" % t.strip())
    n = clean(note[1:-1])
    if n: out.append(""); out.append("> " + n.replace("\n", "\n> "))
    cur = None
    for it in items:
        if it.startswith("\u00a7"):
            cur = clean(it[1:])
            out.append(""); out.append("### " + cur)
            out.append(""); out.append("| 설정값 | 기본 | 원본 | 설명 |")
            out.append("|---|---|---|---|")
            continue
        if cur is None:
            out.append(""); out.append("| 설정값 | 기본 | 원본 | 설명 |")
            out.append("|---|---|---|---|"); cur = ""
        d = DEF.get(it, "—"); o = ORIG.get(it, "—")
        if it in TOGGLE:
            d = "켬" if d in ("1", "true") else "끔"
            o = "켬" if o in ("1", "true") else ("끔" if o in ("0", "false") else o)
        desc = clean(DESC.get(it, ""))
        if it in DEAD: desc = "⛔ 값을 넣어도 반영되지 않습니다. " + desc
        out.append("| `%s` | %s | %s | %s |" % (it, d, o, desc.replace("\n", " ").replace("|", "\\|")))
        nk += 1
    out.append("")

hdr = out.index("# 설정값 전체 목록")
tabs = [l[3:] for l in out[hdr:] if l.startswith("## ")]
out = out[:hdr+5] + ["", "**설정값 묶음 목록**", ""] + ["- " + t for t in tabs] + out[hdr+5:]
import manual_post
io.open(OUT, "w", encoding="utf-8").write(manual_post.fix("\n".join(out)) + "\n")
print("설명서 생성: %s (%d 설정값)" % (OUT, nk))
