# -*- coding: utf-8 -*-
import io, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
P = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust\tfm2_ai_adjust.cfg'
raw = open(P, 'rb').read()
txt = raw.decode('utf-8')
keep = [ln for ln in txt.split('\n')
        if 'champ_verify' not in ln and '08-06 검증용' not in ln and 'athlete_id 오프셋 0x810' not in ln]
out = '\n'.join(keep)
open(P, 'wb').write(out.encode('utf-8'))
print('champ_verify 관련 줄 %d개 제거' % (len(txt.split('\n')) - len(keep)))
print('첫3바이트 =', open(P, 'rb').read(3).hex())
print('champ_verify 잔존 =', 'champ_verify' in open(P, encoding='utf-8').read())
