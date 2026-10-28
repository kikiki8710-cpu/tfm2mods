# -*- coding: utf-8 -*-
"""view_plus layout.ui 의 champion_info 를 0.7.0 바닐라 정보(이름·역할군 줄, 범례, 레벨당 값, 스킬 이름)가
보이도록 재배치한다. 패널 전체 높이(745~985)는 그대로 — dll 이 champion_info y=745 를 매 프레임 고정하기 때문.

  stat        : x 10, 높이 62 → 78  (위 24px = 이름·역할군·범례 줄)
  stat/stats  : x 10→0, y 8→28
  values      : lv1 · lvup · lvmax 각 50px (sep 숨김)  ← 바닐라 「1레벨 · 레벨당 · 12레벨」
  skill*      : y 63→79, 높이 173→157 (data/bar 153→137, desc 153→113 아래 정렬)
  icon_slot   : #name 추가 (x 103 = 설명 칸 위) ← 게임이 champion_info.<skill>.data.icon_slot.name 에 스킬 이름을 쓴다

사용: edit_champinfo.py <in.ui> <out.ui>
"""
import sys, re
sys.stdout.reconfigure(encoding="utf-8")
NODE = re.compile(r"^(\s*)#([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([A-Za-z_]+)\s*\{")
PROP = re.compile(r"^(\s*)([a-z_]+)(\s*:\s*)([^;]*)(;.*)$")

SKILLS = ("skill1", "skill2", "ult")
STAT_CELL = re.compile(r"^champion_info/stat/stats/[a-z_]+/values/(lv1|sep|lvup|lvmax)$")

# (경로, 속성) → 새 값. 경로는 champion_info 부터.
SET = {
    ("champion_info/stat", "height"): "78px",
    ("champion_info/stat/name", "visible"): "true",
    ("champion_info/stat/name", "x"): "0px",
    ("champion_info/stat/name", "height"): "24px",
    ("champion_info/stat/name", "width"): "300px",
    ("champion_info/stat/name", "align_x"): "Left",
    ("champion_info/stat/name", "size"): "20",
    ("champion_info/stat/category", "visible"): "true",
    ("champion_info/stat/category", "x"): "0px",
    ("champion_info/stat/category", "width"): "240px",
    ("champion_info/stat/category", "height"): "24px",
    ("champion_info/stat/category", "align_x"): "Left",
    ("champion_info/stat/category", "size"): "16",
    ("champion_info/stat/category", "color"): "#8a8c96ff",
    ("champion_info/stat/stats", "x"): "0px",
    ("champion_info/stat/stats", "y"): "28px",
}
for s in SKILLS:
    SET[("champion_info/%s" % s, "y")] = "79px"
    SET[("champion_info/%s" % s, "height")] = "157px"
    SET[("champion_info/%s/data" % s, "height")] = "137px"
    SET[("champion_info/%s/data/bar" % s, "height")] = "137px"
    SET[("champion_info/%s/data/desc" % s, "height")] = "113px"
    SET[("champion_info/%s/data/desc" % s, "anchor_y")] = "1"
    SET[("champion_info/%s/data/desc" % s, "pivot_y")] = "1"

# 노드에 없으면 블록 첫머리에 넣을 속성
ADD = {
    "champion_info/stat": ["x: 10px;"],
    "champion_info/stat/name": ["y: 2px;"],
    "champion_info/stat/category": ["y: 2px;"],
}

LEGEND = """
    #levels:label {
      @"asset/base/style/main#label";
      ignore_event: true;
      anchor_x: 1;
      pivot_x: 1;
      x: -300px;
      y: 2px;
      width: 360px;
      height: 24px;
      size: 16;
      align_x: Right;
      align_y: Center;
      color: #8a8c96ff;
      text: "#asset/base/text/ui?banpick.stat_levels_legend";
    }

    #up_dot:color {
      ignore_event: true;
      anchor_x: 1;
      pivot_x: 1;
      x: -200px;
      y: 9px;
      width: 10px;
      height: 10px;
      color: #3bd49aff;

      rounding: Uniform {
        rounding: 5;
      }
    }

    #up:label {
      @"asset/base/style/main#label";
      ignore_event: true;
      anchor_x: 1;
      pivot_x: 1;
      x: -120px;
      y: 2px;
      width: 160px;
      height: 24px;
      size: 16;
      align_x: Right;
      align_y: Center;
      color: #8a8c96ff;
      text: "#asset/base/text/ui?banpick.patch_up";
    }

    #down_dot:color {
      ignore_event: true;
      anchor_x: 1;
      pivot_x: 1;
      x: -60px;
      y: 9px;
      width: 10px;
      height: 10px;
      color: #ffa24dff;

      rounding: Uniform {
        rounding: 5;
      }
    }

    #down:label {
      @"asset/base/style/main#label";
      ignore_event: true;
      anchor_x: 1;
      pivot_x: 1;
      x: 0px;
      y: 2px;
      width: 120px;
      height: 24px;
      size: 16;
      align_x: Right;
      align_y: Center;
      color: #8a8c96ff;
      text: "#asset/base/text/ui?banpick.patch_down";
    }
"""

SKILL_NAME = """
          #name:label {
            @"asset/base/style/main#bold_label";
            ignore_event: true;
            x: 103px;
            y: 0px;
            width: 287px;
            height: 24px;
            size: 16;
            align_x: Left;
            align_y: Center;
            color: #e8e8e8ff;
            text: "";
          }
"""


def cell_set(path, prop):
    m = STAT_CELL.match(path)
    if not m:
        return None
    k = m.group(1)
    if prop == "width":
        return {"lv1": "50px", "sep": "0px", "lvup": "50px", "lvmax": "50px"}[k]
    if prop == "visible":
        return {"sep": "false", "lvup": "true"}.get(k)
    return None


def main(src, dst):
    lines = open(src, encoding="utf-8").read().split("\n")
    out, stack = [], []          # stack 원소 = (이름 또는 "", 경로)
    done = {"legend": 0, "skillname": 0, "set": 0, "add": 0, "sepvis": 0}
    seen_props = {}
    for ln in lines:
        m = NODE.match(ln)
        cur = "/".join(n for n, _ in stack if n)
        if not m:
            p = PROP.match(ln)
            if p and stack and stack[-1][0]:
                key = (cur, p.group(2))
                seen_props.setdefault(cur, set()).add(p.group(2))
                new = SET.get(key) or cell_set(cur, p.group(2))
                if new is not None:
                    ln = "%s%s%s%s%s" % (p.group(1), p.group(2), p.group(3), new, p.group(5))
                    done["set"] += 1
        # 닫는 괄호 처리 전에, 이 줄이 노드를 닫는다면 그 노드 끝에 삽입
        closing_here = []
        depth_change = 0
        first = True
        for ch in ln:
            if ch == "{":
                name = m.group(2) if (m and first) else ""
                first = False
                path = "/".join([n for n, _ in stack if n] + ([name] if name else []))
                stack.append((name, path))
                adds = list(ADD.get(path, []))
                if name == "sep" and STAT_CELL.match(path):
                    adds.append("visible: false;")      # 너비 0 이어도 가운데 정렬 「-」가 경계에 그려질 수 있다
                    done["sepvis"] += 1
                if name and adds:
                    out.append(ln)
                    ind = m.group(1) + "  "
                    for a in adds:
                        out.append(ind + a)
                        done["add"] += 1
                    ln = None
            elif ch == "}" and stack:
                closing_here.append(stack.pop())
        for name, path in closing_here:
            if name == "category" and path == "champion_info/stat/category":
                if ln is not None:
                    out.append(ln); ln = None
                out.extend(LEGEND.rstrip("\n").split("\n"))
                done["legend"] += 1
            if name == "icon_slot" and any(path == "champion_info/%s/data/icon_slot" % s for s in SKILLS):
                # 닫는 줄 앞에 넣는다
                out.extend(SKILL_NAME.strip("\n").split("\n"))
                done["skillname"] += 1
        if ln is not None:
            out.append(ln)
    # sep 에 visible 이 없던 경우 보정: width 0 이면 충분하지만 명시
    text = "\n".join(out)
    open(dst, "w", encoding="utf-8", newline="").write(text)
    print(done)


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
