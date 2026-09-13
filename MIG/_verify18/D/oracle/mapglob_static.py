import re, struct, sys
BS = chr(92)
def parse(line):
    m = re.search(r'c"(.*)"', line); s = m.group(1)
    out = bytearray(); i = 0
    while i < len(s):
        if s[i] == BS:
            out.append(int(s[i+1:i+3], 16)); i += 3
        else:
            out.append(ord(s[i])); i += 1
    return bytes(out)
L = open('C:/tfm2mods/_gcbc/g07.ll', encoding='utf-8', errors='replace').readlines()
regions = parse(L[275]); rd = parse(L[276]); rc = parse(L[277])
print('sizes', len(regions), len(rd), len(rc))
R = [[struct.unpack_from('<Q', regions, (y*30+x)*8)[0] for x in range(30)] for y in range(30)]
D = [[struct.unpack_from('<Q', rd, (a*27+b)*8)[0] for b in range(27)] for a in range(27)]
C = [struct.unpack_from('<QQ', rc, i*16) for i in range(27)]
print('regions[21][21]=', R[21][21], ' regions[9][9]=', R[9][9])
print('region_centers[2]=', C[2], ' [7]=', C[7], ' [0]=', C[0], ' [26]=', C[26])
print('region_dist[2][2]=', D[2][2], ' [7][7]=', D[7][7], ' symmetric=', all(D[a][b] == D[b][a] for a in range(27) for b in range(27)), ' max=', max(map(max, D)))
print('rows with dist<4 to region2:', [a for a in range(27) if D[a][2] < 4])
print('rows with dist<4 to region7:', [a for a in range(27) if D[a][7] < 4])
print('dist==0 to 7:', [a for a in range(27) if D[a][7] == 0])
print('cell(21,21) center=', 21*32000+16000)
# cells whose region==2 / ==7
c2 = [(y, x) for y in range(30) for x in range(30) if R[y][x] == 2]
c7 = [(y, x) for y in range(30) for x in range(30) if R[y][x] == 7]
print('cells region2:', c2)
print('cells region7:', c7)
print('blue nexus (96000,864000) -> regions[27][3] =', R[27][3], ' | red nexus (864000,96000) -> regions[3][27] =', R[3][27])
print('team0 start Top (80000,820000) -> regions[25][2] =', R[25][2], ' team1 (820000,80000) -> regions[2][25]=', R[2][25])
