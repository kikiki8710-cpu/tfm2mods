# 0.6.2: 훅 원본바이트 상수(*_ORIG/PROL_*/*_PROLOGUE) ↔ 신 exe 대조
import re,sys,io,glob,os; sys.path.insert(0,'MIG'); from _x063 import N,rd
from mig_verify import mask_code
MODS=('tfm2_comptest_unlock','tfm2_elemental_serpen','tfm2_level_cap','tfm2_champion_exclude','tfm2_champ_pos_lock','banpick_view_plus','ui_kit')
files=[f for f in glob.glob('*/src/*.rs')+glob.glob('ui_kit/*.rs') if os.path.normpath(f).split(os.sep)[0] in MODS and '_release_backup' not in f]
for f in files:
  s=mask_code(io.open(f,encoding='utf-8').read())
  consts={m.group(1):int(m.group(2),16) for m in re.finditer(r'const (\w+)\s*:\s*usize\s*=\s*(0x[0-9a-f]+)',s)}
  for m in re.finditer(r'const (\w+)\s*:\s*\[u8;\s*\d+\]\s*=\s*\[([^\]]*)\]',s):
    nm=m.group(1); ob=bytes(int(x,16) for x in re.findall(r'0x[0-9a-f]+',m.group(2)))
    stem=re.sub(r'^(PROL_|PROLOGUE_)|(_ORIG|_PROLOGUE|_PROL)$','',nm)
    for k in [k for k in consts if k in (stem+'_RVA','RVA_'+stem,stem)]:
      cur=rd(N,consts[k],len(ob)); print('%-5s %-32s %-30s %s %s'%('OK' if cur==ob else 'DIFF',f[-32:],nm+'@'+k,hex(consts[k]), '' if cur==ob else (ob.hex(),cur.hex())))
