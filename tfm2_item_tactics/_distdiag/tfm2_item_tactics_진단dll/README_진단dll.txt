tfm2_item_tactics 진단용 dll — 아이템 주입 안 됨 원인 로그
======================================================================

[이게 뭔가요]
평소 쓰던 tfm2_item_tactics 와 "완전히 똑같이 동작"하면서, 아이템을 사는
순간마다 어디서 막히는지를 buy_report.txt 로 기록하는 dll 입니다.
mod 설정·아이템 지정(SEL)·3/4칸 cfg 는 지금 쓰던 그대로 유지됩니다.
(dll 파일 하나만 바꾸는 것이라, 지정이나 설정을 다시 할 필요가 없습니다)

[교체 방법]
1. 게임을 끕니다.

2. 아래 폴더로 갑니다.
     ...\Teamfight Manager2\mods\tfm2_item_tactics\

3. 기존 파일 tfm2_item_tactics.dll 을 딴 이름으로 백업해 둡니다.
     예) tfm2_item_tactics.dll  →  tfm2_item_tactics.dll.bak
     (진단 끝나고 되돌릴 때 씁니다)

4. 이 zip 안의 tfm2_item_tactics.dll 을 그 폴더에 넣습니다(덮어쓰기).

5. 게임을 켭니다. (mods.json / 모드 설정은 건드리지 않습니다 — item_tactics 는
   원래 켜져 있던 그대로)

6. 아이템 주입이 안 되던 그 상황(관전 경기)을 재현합니다.
   경기(관전)를 시작해서 선수들이 아이템을 몇 개 살 때까지 지켜보면 됩니다.

[결과 회수]
     ...\Teamfight Manager2\mods\tfm2_item_tactics\buy_report.txt
   이 파일을 그대로 보내주세요. 8단계 중 어디서 막혔는지 다 나옵니다.

[진단 끝난 뒤 원복]
   게임 끄고 → 백업해둔 tfm2_item_tactics.dll.bak 을 다시
   tfm2_item_tactics.dll 로 되돌리면 원래대로 돌아옵니다.

[안전성]
주입 로직은 원본과 완전히 동일합니다. 값을 읽어서 기록만 합니다.
