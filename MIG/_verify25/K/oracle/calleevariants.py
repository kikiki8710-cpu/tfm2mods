# -*- coding: utf-8 -*-
import json, sys, subprocess, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import tlsfull as T
tls, g = T.load()
names = sys.argv[1:] or ['11fight_check13battle_action', '11fight_check20attack_summon_action',
                         '11fight_check29attack_structure_skill_action', '14abstract_input11wait_around']
for nm in names:
    k = [s for s in g if s.endswith(nm)][0]
    mod, a, b = g[k][:3]
    print("==", nm, mod, a, b)
    p = os.path.join(r"C:\tfm2mods\_gaibc", mod)
    out = subprocess.run([sys.executable, '-X', 'utf8', 'elemstores.py', p, str(a), str(b)],
                         capture_output=True, text=True, encoding='utf-8').stdout
    for l in out.split('\n'):
        if '+177 ' in l or 'wait_around' in nm or 'memcpy 184' in l or 'memcpy 136' in l or 'memcpy 24B' in l or l.startswith('allocas'):
            print('  ', l)
