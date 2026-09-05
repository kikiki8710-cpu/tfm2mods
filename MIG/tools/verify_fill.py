# -*- coding: utf-8 -*-
import io, os, re, sys
ROOT = r"C:\tfm2mods\MIG\decomp\0.5.8"
FILES = """plan_legacy/handler.md
plan_legacy/old/battle.md
plan_legacy/old/death_battle.md
plan_legacy/handler/engage.md
plan_legacy/old/passive_line.md
plan_legacy/old/epic.md
plan_legacy/old/fight_model.md
plan_legacy/old/passive_jungle.md
plan_legacy/old/single_battle.md
plan_legacy/old/serpen.md
plan_legacy/old/defense_nexus.md
plan_legacy/handler/auction.md
plan_legacy/old/single_line.md
plan_legacy/old/line_gank/ganker.md
plan_legacy/steal.md
plan_legacy/handler/chat.md
plan_legacy/old/serpen/hunt_and_poke.md
plan_legacy/old/epic/hunt_and_poke.md
plan_legacy/action_eval.md
plan_legacy/old/attack_nexus.md
plan_legacy/old/epic/hunt_and_battle.md
plan_legacy/old/serpen/hunt_and_battle.md
plan_legacy/handler/modes.md
plan_legacy/old/active_recall.md
plan_legacy/old/line_gank/cover.md""".split("\n")
HDR = re.compile(r"^## `0x([0-9a-fA-F]+)`")
tot=filled=empty=failed=trunc=0
det=[]
for rel in FILES:
    p=os.path.join(ROOT, rel)
    if not os.path.exists(p):
        det.append("MISSING "+rel); continue
    s=io.open(p,encoding="utf-8").read()
    lines=s.split("\n")
    n=e=f=t=0
    i=0
    while i<len(lines):
        if HDR.match(lines[i]):
            n+=1
            k=i+1
            while k<len(lines) and lines[k]!="```c" and not HDR.match(lines[k]): k+=1
            if k<len(lines) and lines[k]=="```c":
                j=k+1
                while j<len(lines) and lines[j]!="```": j+=1
                body="\n".join(lines[k+1:j])
                if "<본문>" in body or body.strip()=="": e+=1
                if "DECOMP FAILED" in body: f+=1
                if "점프테이블 미복구" in body: t+=1
        i+=1
    tot+=n; filled+=n-e; empty+=e; failed+=f; trunc+=t
    det.append("%-45s 함수 %3d / 미충전 %d / 디컴실패 %d / 점프테이블보강 %d  (%d KB)"%(rel,n,e,f,t,os.path.getsize(p)//1024))
for d in det: print(d)
print("=== 총 %d 함수, 채움 %d, 미충전 %d, 디컴실패 %d, 점프테이블보강 %d"%(tot,filled,empty,failed,trunc))
