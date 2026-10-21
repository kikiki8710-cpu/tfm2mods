"""이벤트 트레이서 표 생성기 (v0.7.0, 2026-10-21).

tools/events/*.txt 정의(이벤트 지점 + 필드 식)를 읽어
  src/events_generated.rs        모드에 들어가는 이벤트 표(필드 식은 스택 명령열로 컴파일)
  tools/events/extra_events.txt  gen_sites.py --extra 에 합칠 지점 목록(이벤트 지점도 관측점이 있어야 걸린다)
를 만든다. exe 대조: 지점이 0.6.5 명령 경계이고 CALL/분기/스택 명령이 아닌지 확인한다.
사용: python -B -X utf8 tools/gen_events.py tools/events/fountain_wait_065.txt [--exe PATH]
"""
import argparse, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
sys.path.insert(0, HERE)
from gen_sites import Image, GPR, DEF_EXE  # noqa: E402

MAXF = 12
SIZE = {"b": 1, "w": 2, "d": 4, "q": 8}


class Parser:
    """식 → 스텍 명령열. ("reg",n) ("imm",v) ("load",size) ("scale",k) ("add",) ("mask",m)"""

    def __init__(self, s):
        self.t = re.findall(r"0x[0-9a-fA-F]+|\d+|[a-z][a-z0-9]*|[\[\]+*&]", s.replace(" ", ""))
        self.i = 0

    def peek(self):
        return self.t[self.i] if self.i < len(self.t) else None

    def eat(self, x=None):
        v = self.peek()
        if x is not None and v != x:
            raise ValueError(f"'{x}' 기대, '{v}'")
        self.i += 1
        return v

    def expr(self):
        out = self.term()
        while self.peek() == "+":
            self.eat("+"); out += self.term() + [("add",)]
        return out

    def term(self):
        out = self.atom()
        while self.peek() in ("*", "&"):
            op = self.eat(); n = int(self.eat(), 0)
            out += [("scale", n)] if op == "*" else [("mask", n)]
        return out

    def atom(self):
        v = self.eat()
        if v in SIZE and self.peek() == "[":
            self.eat("["); inner = self.expr(); self.eat("]")
            return inner + [("load", SIZE[v])]
        if v in GPR and GPR[v] is not None and v.startswith("r") and len(v) <= 3 and v not in ("rsp",):
            if v not in ("rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"):
                raise ValueError(f"64비트 레지스터만: {v}")
            return [("reg", GPR[v])]
        if re.fullmatch(r"0x[0-9a-fA-F]+|\d+", v):
            return [("imm", int(v, 0))]
        raise ValueError(f"알 수 없는 토큰: {v}")


def compile_expr(s):
    p = Parser(s); prog = p.expr()
    if p.peek() is not None:
        raise ValueError(f"남은 토큰: {p.t[p.i:]}")
    depth = mx = 0
    for st in prog:
        depth += {"reg": 1, "imm": 1, "add": -1}.get(st[0], 0); mx = max(mx, depth)
    if depth != 1 or mx > 8:
        raise ValueError(f"스택 깊이 오류: {s}")
    return prog


def rust_step(st):
    k = st[0]
    if k == "reg": return f"Step::Reg({st[1]})"
    if k == "imm": return f"Step::Imm({st[1]:#x})"
    if k == "load": return f"Step::Load({st[1]})"
    if k == "scale": return f"Step::Scale({st[1]:#x})"
    if k == "mask": return f"Step::Mask({st[1]:#x})"
    return "Step::Add"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("spec")
    ap.add_argument("--exe", default=DEF_EXE)
    a = ap.parse_args()
    img = Image(a.exe)
    events = []
    for ln, raw in enumerate(open(a.spec, encoding="utf-8"), 1):
        line = raw.split("#")[0].strip()
        if not line:
            continue
        parts = line.split()
        rva, name = int(parts[0], 16), parts[1]
        keep, fields = "all", []
        for p in parts[2:]:
            k, v = p.split("=", 1)
            if k == "keep":
                keep = v
            else:
                fields.append((k, compile_expr(v)))
        if len(fields) > MAXF:
            sys.exit(f"{ln}행 {name}: 필드 {len(fields)} > {MAXF}")
        pf = img.pdata_func(rva)
        if not pf:
            sys.exit(f"{ln}행 {name}: 함수 밖 {rva:#x}")
        ins = {i.address: i for i in img.disasm(pf[0], pf[1] + 1)}
        if rva not in ins:
            sys.exit(f"{ln}행 {name}: 명령 경계 아님 {rva:#x}")
        m = ins[rva].mnemonic
        if m in ("call", "jmp", "ret", "push", "pop") or m.startswith("j"):
            sys.exit(f"{ln}행 {name}: 걸 수 없는 명령 {m} @ {rva:#x}")
        chg, hb = [], 0
        if keep.startswith("change:"):
            spec = keep[7:]
            if "@" in spec:
                spec, hb = spec.split("@"); hb = int(hb)
            chg = spec.split(",")
        events.append(dict(rva=rva, name=name, keep="all" if keep == "all" else "change", chg=chg, hb=hb, fields=fields,
                           insn=f"{m} {ins[rva].op_str}"))
    events.sort(key=lambda e: e["rva"])
    out = ["// 자동 생성: tools/gen_events.py — 손으로 고치지 말 것.",
           f"// 정의 파일: {os.path.relpath(a.spec, ROOT)} · exe sha256 {img.sha}",
           "use crate::event_trace::{Event,Field,Step};",
           f"pub const EVENT_EXE_SHA256:&str=\"{img.sha}\";",
           f"pub static EVENTS:[Event;{len(events)}]=["]
    for e in events:
        fs = ",".join("Field{name:%s,prog:&[%s]}" % (f'"{n}"', ",".join(rust_step(s) for s in p)) for n, p in e["fields"])
        chg = ",".join(f'"{c}"' for c in e["chg"])
        out.append(f'Event{{rva:0x{e["rva"]:x},name:"{e["name"]}",change:&[{chg}],heartbeat:{e["hb"]},fields:&[{fs}]}}, // {e["insn"]}')
    out.append("];")
    open(os.path.join(ROOT, "src", "events_generated.rs"), "w", encoding="utf-8", newline="\n").write("\n".join(out) + "\n")
    ex = os.path.join(HERE, "events", "extra_events.txt")
    open(ex, "w", encoding="utf-8").write("".join(f"{e['rva']:#x} event:{e['name']}\n" for e in events))
    for e in events:
        print(f"{e['rva']:#x} {e['name']:7s} 필드 {len(e['fields']):2d} keep={e['keep']}{e['chg']} hb={e['hb']} | {e['insn']}")


if __name__ == "__main__":
    main()
