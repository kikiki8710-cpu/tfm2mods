# -*- coding: utf-8 -*-
"""item_tactics 0.5.3 -> 0.5.4 마이그 보조."""
import sys, re, struct, bisect
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

E53 = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe"
E54 = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\TeamfightManager2.exe"
BASE = 0x140000000

import _it_scan as S
Exe = S.Exe
riprefs = S.riprefs
branches_to = S.branches_to

O = Exe(E53)   # old = 0.5.3
N = Exe(E54)   # new = 0.5.4
