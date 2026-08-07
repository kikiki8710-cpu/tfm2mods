# -*- coding: utf-8 -*-
"""cfg 를 현행(0.5.4) 기준으로 변환한다.
  ①단위가 바뀐 키 값 환산  ②0.5.4에서 원본값이 바뀐 마스크 보정
  ③배선 없는 키·죽은 노브는 **주석 처리**(삭제하지 않는다 — 무엇을 껐는지 남겨야 한다)
출력은 BOM 없는 UTF-8. 원본은 건드리지 않고 새 경로에 쓴다."""
import sys, io, re, json

st = json.load(io.open('C:/tfm2mods/v54/audit_state.json', encoding='utf-8'))
wired, desc = set(st['wired']), st['desc']
DEAD = re.compile(r'⛔|\[은퇴\]|작동하지 않습니다|폐기된 값')
CLS = re.compile(r'_class_(melee|range|magician|util|assassin)$')


def convert(src, dst, title):
    out = []
    txt = io.open(src, encoding='utf-8').read()
    n_unit = n_mask = n_dead = n_miss = 0
    for ln in txt.split('\n'):
        s = ln.strip()
        if not s or s.startswith('#') or '=' not in s:
            out.append(ln); continue
        k, v = [x.strip() for x in s.split('=', 1)]
        base = CLS.sub('', k)

        # ① 단위 환산 — 0.5.4에서 1/1000 로 바뀜
        if base == 'pe_noise_exempt':
            try:
                iv = int(v)
                if iv > 200:
                    out.append('# [0.5.4 변환] 단위가 1/1000 로 바뀌어 %s → %d 로 환산. (유효 0~127)' % (v, round(iv / 1000)))
                    out.append('%s = %d' % (k, round(iv / 1000)))
                    n_unit += 1; continue
            except ValueError:
                pass

        # ② 0.5.4에서 원본값이 바뀐 마스크 — 구 원본값 그대로면 새 원본값으로
        if base == 'ldsc_early_mask' and v.strip() == '128611':
            out.append('# [0.5.4 변환] 이 마스크의 원본값이 128611 → 129123 으로 바뀌었다(비트 9 추가).')
            out.append('#   구 원본값을 그대로 두면 "비트 9를 끈 상태"가 되므로 새 원본값으로 맞춘다.')
            out.append('%s = 129123' % k)
            n_mask += 1; continue

        # ③ 무반응 키 / 죽은 노브 → 주석 처리
        if base not in wired:
            out.append('# [0.5.4 무반응 — 배선 없음] ' + s)
            n_miss += 1; continue
        if DEAD.search(desc.get(base, '')):
            out.append('# [0.5.4 死노브 — 값 무의미] ' + s)
            n_dead += 1; continue

        out.append(ln)

    hdr = ['# ' + title,
           '# 변환 2026-08-06 — 게임 0.5.4 기준. 원본: ' + src.replace('\\', '/'),
           '#   단위 환산 %d · 마스크 보정 %d · 무반응 키 주석 %d · 死노브 주석 %d' % (n_unit, n_mask, n_miss, n_dead),
           '#   주석 처리한 줄은 지우지 않았다 — 무엇을 껐는지 남기기 위해서다.', '']
    io.open(dst, 'w', encoding='utf-8', newline='\n').write('\n'.join(hdr + out))
    print('%s\n   단위 %d · 마스크 %d · 무반응 %d · 死 %d  → %s' % (title, n_unit, n_mask, n_miss, n_dead, dst))


if __name__ == '__main__':
    convert(sys.argv[1], sys.argv[2], sys.argv[3])
