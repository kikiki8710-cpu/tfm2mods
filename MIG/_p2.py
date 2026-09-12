# -*- coding: utf-8 -*-
u"""`probe.rs` 리포트에 **슬롯 넘침 경고**를 넣는다.

★왜 별 파일로 만드나 — Bash heredoc 으로 파이썬 문자열을 만들면 백슬래시·비ASCII 가 깨진다
  (CLAUDE.md 경고). 방금 그걸로 한글이 망가져 `SyntaxError` 가 났다. Write 로 만들어 실행한다.

★무엇을 막는 경고인가 — 진입부 표가 `CS_SLOT0`(32)을 넘으면 그만큼 **설치조차 안 된다.**
  회계 줄(`진입부 18 + 호출부 1 + 표밖 1 = 20/20`)은 **표 길이로 세므로 그래도 맞아 보인다** —
  즉 「회계는 맞는데 측정이 빠진」 상태가 조용히 생긴다. 그래서 회계 줄보다 **먼저, 크게** 찍는다.
"""
import io
import sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
P = r"C:\tfm2mods\tfm2_judge_verify\src\probe.rs"
s = io.open(P, encoding="utf-8").read()

ANCHOR = u'    s.push_str(&format!(\n        "\\n명세 20함수 회계:'
WARN = (
    u'    // \u2605\u2605\uc2ac\ub86f \ub118\uce68\uc740 **\ud68c\uacc4\uac00 \ub9de\uc544\ub3c4 '
    u'\uc5bc\uc744 \ubabb \uc7ac\ub294** \uc0c1\ud0dc\ub2e4 \u2014 \ud68c\uacc4 \uc904\uc740 \ud45c '
    u'\uae38\uc774\ub85c \uc138\ubbc0\ub85c \uadf8\ub798\ub3c4 \ub9de\uc544 \ubcf4\uc778\ub2e4.\n'
    u'    //   \u21d2 \uc870\uc6a9\ud788 \uc794\ub9ac\uba74 \u300c\ud45c\uc5d0 \uc788\ub294\ub2e4 '
    u'\uc65c \ub9ac\ud3ec\ud2b8\uc5d0 \uc5c6\ub098\u300d\ub97c \uc544\ubb34\ub3c4 \ubabb \ubb3b\ub294\ub2e4.\n'
    u'    if OVERFLOW > 0 {\n'
    u'        s.push_str(&format!(\n'
    u'            "\\n\u2605\u2605\u2605\uacbd\uace0: \uc9c4\uc785\ubd80 \ud45c\uac00 \uc2ac\ub86f '
    u'\uc0c1\ud55c(CS_SLOT0={})\uc744 **{}\uac1c \ub118\ucceC\ub2e4** \u2014 \uadf8\ub9cc\ud07c\uc740 '
    u'\uc124\uc9c0\uc870\ucc28 \uc548 \ub410\ub2e4. \\\n'
    u'             `probe.rs` \uc758 CS_SLOT0/CS_CAP \ub97c \ub298\ub9ac\uace0 \uc7ac\ubc14\ub4dc\ud574\uc57c '
    u'\ud55c\ub2e4. \uc544\ub798 \uc218\uce58\ub294 **\ubd80\uc644\uc804**\ud558\ub2e4.\\n",\n'
    u'            CS_SLOT0, OVERFLOW\n'
    u'        ));\n'
    u'    }\n')

if u"CS_SLOT0={}" in s:
    print(u"이미 적용돼 있다 — 변경 없음")
    sys.exit(0)
if ANCHOR not in s:
    print(u"⛔앵커 문면 불일치 — 수동 확인 필요")
    sys.exit(1)
s = s.replace(ANCHOR, WARN + ANCHOR, 1)
io.open(P, "w", encoding="utf-8").write(s)
print(u"슬롯 넘침 경고 추가 완료")
