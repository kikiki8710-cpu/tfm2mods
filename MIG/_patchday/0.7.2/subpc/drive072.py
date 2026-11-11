# -*- coding: utf-8 -*-
"""drive: 서브 PC 의 TFM2 를 화면 판별 + 클릭으로 자동 진행(메인 PC 에서 실행, VNC 터널 5911 필요).
python drive.py --selftest                 기준 이미지 자기 판별 시험
python drive.py <pid> <YYYY-MM-DD 목표> [--max-min N]
  목표 날짜 이상이 되면(메인 화면에서 서브 PC dbdate.py --read 로 확인) 종료코드 0.
  모르는 화면이 연속 MAX_UNK 번이면 스크린샷 남기고 종료코드 2. 시간 초과 3.
판별 = 반 해상도 화면의 버튼 영역을 기준 스크린샷(같은 해상도)의 같은 영역과 평균 색 차이로 비교.
"""
import contextlib, datetime, io, json, os, select, struct, subprocess, sys, time
sys.path.insert(0, r'F:\claude\sub_pc\tools')
from PIL import Image, ImageChops, ImageStat
import vnc_ctl

HERE = r'F:\claude\tfm2\_tmp\salary_070'
LOG = os.path.join(HERE, 'drive.log')
SSH = ['ssh', '-i', os.path.expanduser(r'~\.ssh\subpc_ed25519'), '-o', 'BatchMode=yes', 'owner@192.168.11.59']
MAX_UNK = 60
STOP_AT = set(sys.argv[sys.argv.index('--stop-at') + 1].split(',')) if '--stop-at' in sys.argv else set()   # 0.7.2: 쉼표로 여러 화면   # 이 화면이 뜨면 누르지 않고 끝낸다(경기 관전용)
THR = 14.0
CLICK_HOVER, CLICK_DOWN, CLICK_RELEASE = 0.3, 0.1, 0.3
AFTER_CLICK = 0.6

# (이름, 기준 이미지, 판별 영역(반 해상도), 클릭 좌표(원본 해상도) 목록, 클릭 후 대기 초)
SCREENS = [
    ('FEEDBACK', 'sub_screen1.png', (692, 488, 946, 526), [(1636, 1012)], 3),
    ('SIDE',     's2.png',          (388, 304, 572, 326), [(1054, 628)], 3),
    ('LINEUP',   's3.png',          (815, 499, 942, 519), [(1756, 1016)], 3),
    ('LINEUP0',  'lineup0_ref.png', (680, 499, 942, 519), [(1486, 1016)], 2),   # 명단 비어 제출 불가 → 수석 코치에게 위임
    ('BANPICK0', 'banpick0_red_ref.png', (8, 6, 108, 24), [(116, 30)], 2),   # 레드 진영 밴픽 화면(왼쪽 막대 색이 달라 원본 기준과 차이 19.8)
    ('BANPICK0', 'banpick0_ref.png', (8, 6, 108, 24),    [(116, 30)], 2),   # 밴픽 미위임 → 「밴픽을 수석코치에게 위임」
    ('SWAP',     's5.png',          (484, 452, 805, 485), [(1286, 934)], 3),
    ('TACTICS',  's6.png',          (486, 492, 946, 525), [(1430, 1016)], 3),
    ('MATCH',    's7.png',          (684, 452, 950, 471), [(1632, 922)], 3),
    ('RESULT',   's8.png',          (386, 488, 574, 526), [(960, 1012)], 3),
    ('MAIN',     't5.png',          (858, 506, 948, 530), [(594, 1034), (1806, 1034)], 3),
    ('MATCHDAY', 'matchday_ref.png', (858, 506, 948, 530), [(1806, 1034)], 3),   # 오늘의 경기 → 「상대 정보 확인」
]
# 응답이 필요한 소식(연습 경기 요청 등 — 수락/거절 두 버튼): MAIN 보다 먼저 본다 → 거절
DECIDE = ('DECIDE', 'u7.png', (778, 462, 940, 483), [(1800, 944)], 2)
# 우리 선수 이적 제안(수락/거절/협상 세 버튼 · 기한 지나기 전엔 진행을 막는다) → 거절
OFFER = ('OFFER', 'offer_ref.png', (697, 462, 940, 483), [(1624, 944)], 2)
# 「이적시즌 마감하기」 확인 창 → 확인 (마감 중 들어오는 제안은 게임이 자동 거절)
CLOSEWIN = ('CLOSEWIN', 'u9.png', (360, 290, 600, 312), [(1082, 600)], 3)
# 이적 기간 중 메인 화면(아래쪽에 「이적시즌 마감하기」 버튼) → 그 버튼
WINMAIN = ('WINMAIN', 't5.png', (770, 506, 850, 530), [(1610, 1034)], 2)
# 경기 전 인터뷰 등 카드형 선택(오른쪽 위 빨간 「응답 필요」) → 첫 번째 카드
INTERVIEW = ('INTERVIEW', 'interview_ref.png', (884, 70, 932, 84), [(1344, 424)], 2)
# 코치가 밴픽 진행 중 오버레이(「밴픽 즉시 완료」)는 위임 버튼 화면(BANPICK0)보다 먼저 — 순서가 뒤면 위임을 켰다 껐다 반복(11-02 192회)
BANPICK_P = ('BANPICK', 's4.png', (400, 285, 560, 304), [(960, 588)], 3)
BANPICK2 = ('BANPICK2', 'codex_delegate_releasehold_after.png', (485, 285, 645, 305), [(1130, 588)], 3)
MATCHWAIT = ('MATCHWAIT', 'codex_match_wait_ref.png', (684, 452, 950, 471), [], 5)
CALWAIT = ('CALWAIT', 'codex_advancing_ref.png', (12, 12, 948, 26), [], 5)
OS_QUICK = ('OS_QUICK', 'unk_081323_01.png', (775, 343, 951, 508), [], 1)
OS_NOTIFY = ('OS_NOTIFY', 'unk_080320_01.png', (786, 4, 952, 30), [], 1)
OS_BAR = ('OS_BAR', 'codex_quick_dismissed.png', (0, 516, 960, 540), [(960, 40)], 2)
CONSOLEWAIT = ('CONSOLEWAIT', 'unk_081323_01.png', (100, 110, 575, 500), [], 5)
WINDOWWAIT = ('WINDOWWAIT', 'window_advancing_ref.png', (278, 174, 427, 212), [], 5)
CALBANNER = ('CALBANNER', 'year_advancing_ref.png', (8, 34, 167, 70), [], 5)
PRIORITY = [CONSOLEWAIT, OS_QUICK, OS_NOTIFY, OS_BAR, CALBANNER, WINDOWWAIT, CLOSEWIN, DECIDE, OFFER, INTERVIEW, BANPICK2, BANPICK_P, MATCHWAIT, CALWAIT, WINMAIN]

def log(s):
    line = '%s %s' % (time.strftime('%H:%M:%S'), s)
    print(line, flush=True)
    open(LOG, 'a', encoding='utf-8').write(line + '\n')

def save_status(name, difference, date, counts, state='실행중'):
    path = os.path.join(HERE, 'drive_status.json')
    temporary = path + '.%d.tmp' % os.getpid()
    data = {'시각':time.strftime('%Y-%m-%d %H:%M:%S'), '화면':name, '차이':difference,
            '날짜':str(date), '누적':counts, '진행기상태':state}
    with open(temporary,'w',encoding='utf-8') as sf:
        json.dump(data,sf,ensure_ascii=False)
    for attempt in range(5):
        try:
            os.replace(temporary,path); return
        except PermissionError:
            time.sleep(0.1)
    log('상태 JSON 교체 실패: 다음 반복에서 재시도')

_refs = {}
def ref_crop(name, fn, box):
    k = (fn, box)
    if k not in _refs:
        _refs[k] = Image.open(os.path.join(HERE, fn)).convert('RGB').crop(box)
    return _refs[k]

STRICT = {'INTERVIEW': 3.0, 'CLOSEWIN': 3.0, 'WINDOWWAIT':3.0, 'CALBANNER':3.0}   # 유사 팝업은 엄격 판별

STRICT.update({'BANPICK': 3.0})

def classify(half):
    inbox_box = (0, 55, 128, 126)
    inbox_diff = sum(ImageStat.Stat(ImageChops.difference(half.crop(inbox_box), ref_crop('INBOX', 'codex_header_focus_verified.png', inbox_box))).mean) / 3
    for name, fn, box, _, _ in PRIORITY:
        if name == 'BANPICK2':
            top = (8, 4, 150, 21)
            dim_diff = sum(ImageStat.Stat(ImageChops.difference(half.crop(top), ref_crop('DELEGATED', 'codex_delegate_releasehold_after.png', top))).mean) / 3
            if dim_diff < 8:
                skip = choose_skip(half)
                if skip: return (skip['name'], skip['score'])
        if name.startswith('OS_') and inbox_diff >= 3.0:
            continue  # 게임 소식함 배경까지 확인한 경우에만 OS 패널 복구를 허용한다.
        dp = sum(ImageStat.Stat(ImageChops.difference(half.crop(box), ref_crop(name, fn, box))).mean) / 3
        limit = 8.0 if name == 'CONSOLEWAIT' else (3.0 if name.startswith('OS_') or name in ('BANPICK2', 'MATCHWAIT', 'CALWAIT') else STRICT.get(name, THR))
        if dp < limit: return (name, dp)
    best = (None, 1e9)
    for name, fn, box, _, _ in SCREENS:
        a = half.crop(box); b = ref_crop(name, fn, box)
        d = sum(ImageStat.Stat(ImageChops.difference(a, b)).mean) / 3
        if d < best[1]: best = (name, d)
    if best[1] < THR: return best
    # 예비 규칙: 오른쪽 아래 초록 버튼(글자 무관 — 진행하기/상대 정보 확인/경기장으로 …)이 보이면 그 자리를 누른다(11-02)
    def near(px, ref=(18, 79, 67), tol=8): return all(abs(a - b) <= tol for a, b in zip(px, ref))
    if all(near(half.getpixel(p)) for p in ((866, 512), (940, 512), (868, 524))):
        return ('GREEN', 0.0)
    return (None, best[1])

def shoot(v, max_wait=10):
    start = time.monotonic()
    while True:
        with contextlib.redirect_stderr(io.StringIO()):
            img = v.shot(wait=0)
        if img.size != (1920, 1080):
            raise OSError('예상과 다른 화면 크기: %s' % (img.size,))
        hist = img.convert('L').histogram()
        visible = sum(hist[9:]) / (img.width * img.height)
        if visible >= 0.01:
            break
        if time.monotonic() - start >= max_wait:
            raise OSError('커서뿐인 초기 프레임이 계속되어 입력하지 않음')
        time.sleep(0.3)  # 첫 프레임에 커서만 있어도 전체 화면 갱신 전에는 입력하지 않는다.
    return img.resize((img.width // 2, img.height // 2), Image.LANCZOS), img

_BOXES = None
def shoot_fast(v):
    """판별에 쓰는 영역만 원본 해상도로 받아 반 해상도 캔버스에 붙인다(전체 8MB → 수십 KB · 11-02 속도 개선)."""
    global _BOXES
    if _BOXES is None:
        _BOXES = sorted({b[2] for b in SCREENS + PRIORITY})
    half = Image.new('RGB', (960, 540))
    for (x0, y0, x1, y1) in _BOXES:
        m = 4
        X0, Y0, X1, Y1 = max(0, x0 - m), max(0, y0 - m), min(960, x1 + m), min(540, y1 + m)
        reg = v.shot((X0 * 2, Y0 * 2, (X1 - X0) * 2, (Y1 - Y0) * 2), wait=0)   # 원래 검은 영역에서 8초 기다리지 않게
        half.paste(reg.resize((X1 - X0, Y1 - Y0), Image.LANCZOS), (X0, Y0))
    return half

def click(v, x, y, hover=None):
    v.pointer(x, y, 0); time.sleep(CLICK_HOVER if hover is None else hover)
    v.pointer(x, y, 1); time.sleep(CLICK_DOWN); v.pointer(x, y, 0); time.sleep(CLICK_RELEASE)
    v.pointer(960, 2, 0)   # 버튼 위 hover 색 변화·툴팁이 판별을 흐리지 않게 치움(오른쪽 끝은 밴픽 상대 카드 위라 툴팁이 뜸 — 11-02)

def all_read_status(half):
    box = (203, 506, 392, 530)
    difference = sum(ImageStat.Stat(ImageChops.difference(half.crop(box), ref_crop('ALLREAD', 'codex_header_focus_verified.png', box))).mean) / 3
    blue = 0
    for y in range(12, 30):
        for x in range(100, 123):
            r, g, b = half.getpixel((x, y))
            blue += b > 150 and g > 90 and r < 100
    return {'필요': difference < 3 and blue >= 15, '버튼차이': difference, '배지파란픽셀': blue}

def delegate_ready(half):
    box = (8, 4, 150, 21)
    crop = half.crop(box)
    diff = min(sum(ImageStat.Stat(ImageChops.difference(crop, ref_crop('DELEGATE_ACTIVE', fn, box))).mean) / 3
               for fn in ('banpick0_ref.png', 'banpick0_red_ref.png'))   # 레드 진영은 왼쪽 막대 색이 다르다
    brightness = sum(ImageStat.Stat(crop).mean) / 3
    return brightness >= 150 and diff < 8

def choose_skip(half):
    choices = []
    for name, box, point in (('BANPICK2', (485,285,645,305), (1130,588)), ('BANPICK', (400,285,560,305), (960,588))):
        green = white = 0
        for y in range(box[1], box[3]):
            for x in range(box[0], box[2]):
                r,g,b = half.getpixel((x,y))
                green += g >= 50 and g > 1.5*r and b >= 30 and g >= b
                white += min(r,g,b) > 180
        fraction = green / ((box[2]-box[0])*(box[3]-box[1]))
        if fraction >= 0.7 and white >= 30:
            choices.append({'name':name, 'point':point, 'score':1-fraction, 'green':fraction, 'white':white})
    return choices[0] if len(choices) == 1 else None

def write_action(data):
    with open(os.path.join(HERE, 'drive_actions.jsonl'), 'a', encoding='utf-8') as af:
        af.write(json.dumps(data, ensure_ascii=False) + '\n')

def wait_transition(v, name, base_wait, frame_box):
    started = time.perf_counter()
    result = {'mode': 'transition_incremental', 'polls': 0, 'changed': False, 'next': name,
              'sleep_s': 0.0, 'socket_wait_s': 0.0, 'capture_s': 0.0, 'same_polls': 0, 'unknown_polls': 0}
    if name in ('MAIN', 'WINMAIN'):
        time.sleep(0.8)
        result.update(mode='fixed_main', sleep_s=time.perf_counter() - started, elapsed_s=time.perf_counter()-started)
        return result
    limit = 20.0 if name in ('BANPICK0','BANWAIT') else max(1.0, base_wait * 2)
    deadline = started + limit
    previous_timeout = v.s.gettimeout()
    pending = False
    image = frame_box[0].copy()
    if name in ('BANPICK0','BANWAIT'):
        v.pointer(1130,588,0)
        result['hover_target'] = [1130,588]
        hover_started = time.perf_counter()
    try:
        while time.perf_counter() < deadline:
            if os.path.exists(os.path.join(HERE, 'drive.stop')):
                result['stop'] = True; break
            if not pending:
                if result['polls']:
                    sleeping = time.perf_counter()
                    time.sleep(min(0.3 if name in ('BANPICK0','BANWAIT') else 0.5, max(0, deadline-sleeping)))
                    result['sleep_s'] += time.perf_counter()-sleeping
                v.s.sendall(struct.pack('>BBHHHH',3,1,0,0,v.w,v.h))
                pending = True
            remaining = deadline - time.perf_counter()
            if remaining <= 0: break
            selecting = time.perf_counter()
            ready,_,_ = select.select([v.s],[],[],min(0.3 if name in ('BANPICK0','BANWAIT') else 0.5,remaining))
            result['socket_wait_s'] += time.perf_counter()-selecting
            if not ready: continue
            v.s.settimeout(max(0.05, min(previous_timeout or 15, remaining)))
            capturing = time.perf_counter()
            try:
                v._read_update(image,0,0)
            except (OSError, SystemExit) as error:
                result['error'] = str(error); break
            finally:
                result['capture_s'] += time.perf_counter() - capturing
            pending = False
            frame_box[0] = image
            result['polls'] += 1
            hist = image.convert('L').histogram()
            if sum(hist[9:]) / (image.width*image.height) < 0.01:
                result['unknown_polls'] += 1; continue
            half = image.resize((960,540),Image.LANCZOS)
            observed = classify(half)[0]
            protected = observed == 'CONSOLEWAIT' or (observed or '').startswith('OS_')
            skip = choose_skip(half) if name in ('BANPICK0','BANWAIT') and not protected else None
            new_name = skip['name'] if skip else observed
            result['next'] = new_name
            result['same_polls'] += new_name == name
            result['unknown_polls'] += new_name is None
            if name == 'BANPICK0' and new_name == 'BANPICK0' and delegate_ready(half) and time.perf_counter()-started >= 3:
                result['needs_delegation'] = True; break
            if new_name is not None and new_name != name:
                result['changed'] = True; break
    finally:
        with contextlib.suppress(OSError):
            v.s.settimeout(previous_timeout)
    result['elapsed_s'] = time.perf_counter() - started
    if 'hover_target' in result: result['hover_s'] = time.perf_counter()-hover_started
    result['limit_reached'] = not result['changed'] and result['elapsed_s'] >= limit - 0.05
    return result

def game_date(pid):
    try:
        r = subprocess.run(SSH + ['python -X utf8 %%LOCALAPPDATA%%\\Temp\\tfm2probe\\dbdate.py %d --read' % pid],
                           capture_output=True, text=True, timeout=40)
        s = r.stdout.strip().splitlines()[-1] if r.stdout.strip() else ''
        return datetime.date.fromisoformat(s)
    except Exception as e:
        log('날짜 읽기 실패: %r' % e); return None

def rescan(pid):
    try:
        r = subprocess.run(SSH + [r'python -X utf8 %%LOCALAPPDATA%%\Temp\tfm2probe\dbdate.py %d --scan' % pid],
                           capture_output=True, text=True, timeout=200)
        log('재스캔: ' + ' / '.join(r.stdout.strip().splitlines()[-8:]))
    except Exception as e:
        log('재스캔 실패: %r' % e)

def selftest():
    for name, fn, box, _, _ in SCREENS:
        half = Image.open(os.path.join(HERE, fn)).convert('RGB')
        got = classify(half)
        others = []
        for n2, f2, b2, _, _ in SCREENS:
            if n2 == name: continue
            o = Image.open(os.path.join(HERE, f2)).convert('RGB')
            c = classify(o)
            if c[0] == name: others.append(n2)
        print('%-9s ← %-16s 판별=%s(%.1f)  오판(다른 기준이 이 화면으로)=%s' % (name, fn, got[0], got[1], others))
    for fn in ('t12.png', 't13.png', 's9.png', 't16.png', 't7.png'):
        p = os.path.join(HERE, fn)
        if os.path.exists(p): print('  기타 %-8s → %s' % (fn, classify(Image.open(p).convert('RGB'))))

def run(pid, target, max_min, keep_window=False):
    v = None
    t_end = time.time() + max_min * 60
    unk = 0; mains = 0; counts = {}; last_date = None; same = 0; rep = 0; last_name = None
    last_save = 0; connection_failures = 0
    banpick_pending = False; delegation_at = 0; delegation_tries = 0; banpick_started = 0
    skip_proofs = set()
    log('시작 pid=%d 목표=%s' % (pid, target))
    profile = {'시각': time.strftime('%Y-%m-%d %H:%M:%S'), 'unix': time.time(),
               '호버': CLICK_HOVER, '누름': CLICK_DOWN, '해제뒤': CLICK_RELEASE, '클릭뒤': AFTER_CLICK,
               '모두읽음': '버튼과 미독 배지 확인 후 조건부', '대기': '증분 전환 판별·밴픽 즉시 완료 연속 입력', '이적기간유지': keep_window}
    with open(os.path.join(HERE, 'drive_profiles.jsonl'), 'a', encoding='utf-8') as pf:
        pf.write(json.dumps(profile, ensure_ascii=False) + '\n')
    log('입력 대기 설정 ' + json.dumps(profile, ensure_ascii=False))
    try:
        while time.time() < t_end:
            if os.path.exists(os.path.join(HERE, 'drive.stop')):
                log('정지 요청 파일 → 종료'); return 5
            connection_started = time.perf_counter()
            if v is not None:
                v.close()
                v = None
                time.sleep(2)
            try:
                v = vnc_ctl.RFB()  # 매 반복마다 기존 연결을 닫은 뒤 새 연결로 최신 화면을 확인한다.
                connection_seconds = time.perf_counter() - connection_started
                capture_started = time.perf_counter()
                half, full = shoot(v)
                capture_seconds = time.perf_counter() - capture_started
                connection_failures = 0
            except (OSError, SystemExit) as e:
                connection_failures += 1
                log('VNC 연결·캡처 실패 %d/3: %s' % (connection_failures, e))
                if connection_failures >= 3:
                    return 6
                time.sleep(3)
                continue
            name, d = classify(half)
            if name and name in STOP_AT:
                log('멈춤 화면 도달 ' + name); return 0
            if name in ('MAIN','WINMAIN','MATCHDAY','LINEUP','LINEUP0','SWAP','TACTICS','MATCH','RESULT','FEEDBACK','SIDE'):
                banpick_pending = False; delegation_tries = 0; banpick_started = 0
            if name in ('BANPICK0','BANPICK','BANPICK2') and not banpick_started:
                banpick_started = time.monotonic()
            if name is None and banpick_pending:
                name = 'BANWAIT'
            if name in ('BANPICK0','BANWAIT') and banpick_started and time.monotonic()-banpick_started > 120:
                half.save(os.path.join(HERE,'banpick_stuck.png'))
                log('밴픽 120초 대기 초과 → 안전하게 입력 중단'); return 2
            if time.time() - last_save >= 15:
                half.save(os.path.join(HERE, 'drive_current.png'))
                last_save = time.time()
            save_status(name,d,last_date,counts)
            if name is None:
                unk += 1
                if unk in (1, 20, 40) or unk >= MAX_UNK:
                    fn = os.path.join(HERE, 'unk_%s_%02d.png' % (time.strftime('%H%M%S'), unk))
                    half.save(fn)
                if unk >= MAX_UNK:
                    log('모르는 화면 %d회 연속 → 중단 (마지막 diff %.1f)' % (unk, d)); return 2
                time.sleep(5); continue
            unk = 0
            # 안전장치: 같은 특수 화면(MAIN 제외)이 5번 넘게 연속되면 오판으로 보고 「진행하기」로 넘긴다
            #   (2026-11-02 인터뷰 화면 1,055회 반복 실사고 — 결정 뒤에도 같은 화면으로 판별됨)
            rep = rep + 1 if name == last_name else 1; last_name = name
            if name.startswith('OS_') and rep > 3:
                half.save(os.path.join(HERE, 'os_recovery_stuck.png'))
                log('OS 패널 복구 3회 뒤에도 같은 화면 → 입력 중단'); return 2
            if rep > 5 and name not in ('MAIN', 'MATCH', 'MATCHWAIT', 'CALWAIT', 'CALBANNER', 'WINDOWWAIT', 'CONSOLEWAIT','BANPICK0','BANPICK','BANPICK2','BANWAIT'):
                log('%s 화면 %d회 연속 → 진행하기로 넘김' % (name, rep))
                click(v, 594, 1034); time.sleep(1); click(v, 1806, 1034); time.sleep(6); rep = 0; continue
            counts[name] = counts.get(name, 0) + 1
            trace_stamp = round(time.time(), 1)
            date_seconds = 0.0
            with open(os.path.join(HERE, 'drive_trace.log'), 'a', encoding='utf-8') as tf:
                tf.write('%.1f %s %.1f\n' % (trace_stamp, name, d))
            if name in ('MAIN', 'WINMAIN'):
                mains += 1
                if name == 'WINMAIN' or mains % 4 == 1 or (last_date and (last_date >= datetime.date(2026, 11, 25) or 0 <= (target-last_date).days <= 14)):
                    date_started = time.perf_counter()
                    gd = game_date(pid)
                    date_seconds = time.perf_counter() - date_started
                    log('MAIN #%d 날짜=%s 누적=%s' % (mains, gd, counts))
                    if name == 'WINMAIN' and gd is None:
                        half.save(os.path.join(HERE,'window_date_failed.png'))
                        log('이적 기간 날짜 읽기 실패 → 기간 마감 없이 중단'); return 7
                    if gd and gd >= target:
                        save_status(name,d,gd,counts,state='목표도달')
                        log('목표 날짜 도달 → 종료'); return 0
                    if gd and gd == last_date:
                        same += 1
                        if same == 8:
                            log('날짜 정체 → DB 재스캔'); rescan(pid)
                        if same >= 16:
                            half.save(os.path.join(HERE, 'stuck_%s.png' % time.strftime('%H%M%S')))
                            log('날짜가 %s 에서 16번 연속 그대로 → 중단' % gd); return 4
                    else:
                        same = 0; last_date = gd
            GREEN = ('GREEN', '', None, [(1806, 1034)], 3)
            BANWAIT = ('BANWAIT','',None,[],1)
            if name in ('OS_QUICK', 'OS_NOTIFY'):
                v.key(vnc_ctl.keysym('esc'), True); time.sleep(0.15)
                v.key(vnc_ctl.keysym('esc'), False); time.sleep(1)
                continue  # Esc 뒤 새 화면을 확인하고 OS_BAR일 때만 게임 제목을 클릭한다.
            spec = {p[0]: p for p in PRIORITY + [GREEN,BANWAIT]}.get(name) or SCREENS[[s[0] for s in SCREENS].index(name)]
            clicks = list(spec[3])
            all_read = None
            if name == 'WINMAIN' and keep_window:
                clicks = [(594,1034), (1806,1034)]
            if name == 'MAIN' or (name == 'WINMAIN' and keep_window):
                all_read = all_read_status(half)
                if not all_read['필요']:
                    clicks = [point for point in clicks if point != (594, 1034)]
            if name == 'BANPICK0':
                if not delegate_ready(half) or (banpick_pending and time.monotonic()-delegation_at < 3):
                    clicks = []
                elif delegation_tries >= 2:
                    half.save(os.path.join(HERE,'delegate_retry_stuck.png'))
                    log('활성 미위임 상태에서 위임 2회 실패 → 중단'); return 2
                if clicks:
                    banpick_pending = True; delegation_at = time.monotonic(); delegation_tries += 1
            if name in ('BANPICK','BANPICK2'):
                skip = choose_skip(half)
                clicks = [skip['point']] if skip else []
                banpick_pending = True
            after_click = 0 if name in ('BANPICK0','BANPICK','BANPICK2') else AFTER_CLICK
            input_started = time.perf_counter()
            for (x, y) in clicks:
                click(v, x, y); time.sleep(after_click)
            input_seconds = time.perf_counter() - input_started
            wait_started = time.perf_counter()
            frame_box = [full]
            transition = wait_transition(v, name, spec[4], frame_box)
            wait_seconds = transition.get('elapsed_s',time.perf_counter()-wait_started)
            if name in ('BANPICK0','BANWAIT') and transition.get('next') in ('BANPICK','BANPICK2'):
                seen = frame_box[0].resize((960,540),Image.LANCZOS)
                skip = choose_skip(seen)
                if skip:
                    kind = skip['name']; st = round(time.time(),1)
                    counts[kind] = counts.get(kind,0)+1
                    with open(os.path.join(HERE,'drive_trace.log'),'a',encoding='utf-8') as tf:
                        tf.write('%.1f %s %.1f\n' % (st,kind,skip['score']))
                    hovering = max(0,CLICK_HOVER-transition.get('hover_s',0)) if transition.get('hover_target') == list(skip['point']) else CLICK_HOVER
                    nested_start = time.perf_counter()
                    click(v,*skip['point'],hover=hovering)
                    nested_input = time.perf_counter()-nested_start
                    if kind not in skip_proofs:
                        seen.save(os.path.join(HERE,'skip_seen_%s_%s.png' % (kind,time.strftime('%H%M%S')))); skip_proofs.add(kind)
                    nested_wait = wait_transition(v,kind,3,frame_box)
                    write_action({'t':st,'screen':kind,'clicks':1,'points':[skip['point']],'fast_skip':True,
                        'input_fixed_s':hovering+CLICK_DOWN+CLICK_RELEASE,'input_s':nested_input,
                        'wait_s':nested_wait['elapsed_s'],'sleep_s':nested_wait['sleep_s'],'transition':nested_wait,
                        'connect_s':0,'capture_s':0,'date_s':0})
                    log('밴픽 즉시 완료 연속 입력 %s 좌표=%s' % (kind,skip['point']))
            write_action({'t':trace_stamp,'screen':name,'clicks':len(clicks),'points':clicks,'all_read':all_read,
                'input_fixed_s':len(clicks)*(CLICK_HOVER+CLICK_DOWN+CLICK_RELEASE+after_click),
                'input_s':input_seconds,'wait_s':wait_seconds,'sleep_s':transition['sleep_s'],'transition':transition,
                'connect_s':connection_seconds,'capture_s':capture_seconds,'date_s':date_seconds})
        log('시간 초과'); return 3
    finally:
        if v is not None:
            v.close()

if __name__ == '__main__':
    if '--selftest' in sys.argv:
        selftest(); sys.exit(0)
    pid = int(sys.argv[1]); target = datetime.date.fromisoformat(sys.argv[2])
    mm = int(sys.argv[sys.argv.index('--max-min') + 1]) if '--max-min' in sys.argv else 120
    sys.exit(run(pid, target, mm, keep_window='--keep-window' in sys.argv))
