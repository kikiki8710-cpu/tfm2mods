exec(open('srcmap.py',encoding='utf-8').read())
import pickle
A57=pickle.load(open('_s57c.pkl','rb'))[0]; A58=pickle.load(open('_s58c.pkl','rb'))[0]
def mods(strs,pref):
    return sorted({v for v in strs.values() if pref in v})
p='plan_legacy'
m7=mods(A57,p); m8=mods(A58,p)
s7=set(m7); s8=set(m8)
print("0.5.7 plan_legacy modules=%d / 0.5.8=%d"%(len(m7),len(m8)))
print("--- 0.5.7 only:"); [print("   ",x) for x in sorted(s7-s8)]
print("--- 0.5.8 only:"); [print("   ",x) for x in sorted(s8-s7)]
print("--- common (0.5.7 list):")
for x in m7: print("   ",x)
