# -*- coding: utf-8 -*-
"""run23A.py — o23A.exe 를 케이스당 프로세스 1개로 실행, 로그 저장 + 요약."""
import subprocess, sys, os, io
EXE = r'C:\Users\jungs\AppData\Local\Temp\tfm2_spanprobe\o23A.exe'
OUT = os.path.dirname(os.path.abspath(__file__))
cases = [int(c) for c in sys.argv[1:]] or [10, 11, 12, 13, 14, 15, 20, 21, 22, 23, 40, 41, 42, 43, 44, 45, 46, 47, 48, 60, 61, 62, 63, 64, 65, 80, 81, 82, 83]
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
for c in cases:
    try:
        p = subprocess.run([EXE, str(c)], capture_output=True, timeout=600)
        out = p.stdout.decode('utf-8', 'replace'); err = p.stderr.decode('utf-8', 'replace')
        rc = p.returncode
    except subprocess.TimeoutExpired:
        out, err, rc = '', 'TIMEOUT', -1
    io.open(os.path.join(OUT, 'o23A_case%d.log' % c), 'w', encoding='utf-8').write(out + '\n--- stderr ---\n' + err)
    lines = [l for l in out.split('\n') if l.startswith(('140', '135', '136', '141', '134', 'helpers', 'lethal', 'well', 'escape', 'fountain', 'bush_ids', 'map_bush', 'table_bush', 'mid_seq', 'line_regions'))]
    print('== case %d rc=%d' % (c, rc))
    for l in lines[-12:]:
        print('   ' + l[:300])
    if rc != 0:
        print('   stderr: ' + err.strip()[-400:].replace('\n', ' | '))
