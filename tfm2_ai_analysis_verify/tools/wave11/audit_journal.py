"""STOP 정본과 전체 append-only journal을 읽기 전용으로 대조한다. 자르기/복구 없음."""
from pathlib import Path
import hashlib,json

def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def duplicate_free(pairs):
    d={}
    for k,v in pairs:
        if k in d:raise ValueError('중복JSON키:'+k)
        d[k]=v
    return d
def load(p):return json.loads(Path(p).read_text(encoding='utf-8-sig'),object_pairs_hook=duplicate_free)
def integer(v):
    if type(v)is not int or v<0:raise ValueError('정본sequence/시간 정수형식')
    return v
def drained(s):
    if s.get('capture_enabled')is not False:raise ValueError('STOP capturefalse 필요')
    for k in ('in_flight','queue_pending','queued_events'):
        if integer(s[k])!=0:raise ValueError('STOP drain 미완료:'+k)
    if integer(s['control_sequence'])<2 or s['control_sequence']!=s['stop_pending_sequence'] or s.get('control_error'):
        raise ValueError('STOP control ACK 미완료')

def audit(summary_path,journal_path,expected_run_id,expected_record_seq=None):
    s=load(summary_path);drained(s)
    if s.get('run_id')!=expected_run_id or s.get('summary_source')!='journal':raise ValueError('STOP run/source 불일치')
    seq=integer(s['record_seq'])
    if seq!=integer(s['summary_record_sequence']):raise ValueError('STOP record별칭 불일치')
    if expected_record_seq is not None and seq!=expected_record_seq:raise ValueError('기대 STOP record 불일치')
    raw=Path(journal_path).read_bytes()
    if not raw or not raw.endswith(b'\n'):raise ValueError('journal 완전 마지막줄 필요')
    if s['summary_journal']!=Path(journal_path).name:raise ValueError('journal 파일이름/run 바인딩 불일치')
    selected=None;last=None;after=0;rows=0;laststamp=0
    protected=('baseline_sha256','actual_sha256','identity_verified','armed','init_ready','prepared_probes','control_sequence','stop_pending_sequence','io_errors','io_lost_or_uncertain_events')
    counters=('sample_attempts','compared','matches','mismatches','skipped','unstable','model_panics','dropped','foreign_matches','foreign_mismatches','processing_errors','unlogged_errors','persisted','enqueued','eligible_calls','queue_full','queue_disconnected')
    for line in raw.decode('utf-8-sig').splitlines():
        if not line:raise ValueError('journal 빈행 불허')
        row=json.loads(line,object_pairs_hook=duplicate_free);rows+=1
        n=integer(row['record_seq'])
        if n!=rows or integer(row['summary_record_sequence'])!=n:raise ValueError('journal 누락/중복/역행 record')
        stamp=integer(row['summary_timestamp_unix_ms'])
        if stamp<laststamp:raise ValueError('journal 시간역행: freshness 미확정')
        laststamp=stamp
        if row.get('run_id')!=expected_run_id or row.get('summary_source')!='journal':raise ValueError('journal run/source 혼합')
        if row.get('summary_journal')!=Path(journal_path).name:raise ValueError('journal 경로혼합')
        if n==seq:
            if row!=s:raise ValueError('선택 STOP summary와 정본 journal row 다름')
            selected=row
        elif n>seq:
            if selected is None:raise ValueError('선택 STOP 정본 없음')
            drained(row);after+=1
            for key in protected:
                if row.get(key)!=s.get(key):raise ValueError('STOP후 protected변동:'+key)
            a={p['id']:p for p in s['probes']};b={p['id']:p for p in row['probes']}
            if a.keys()!=b.keys():raise ValueError('STOP후 probe집합변동')
            for pid in a:
                if a[pid]['rva']!=b[pid]['rva']:raise ValueError('STOP후 RVA변동')
                for k in counters:
                    if a[pid].get(k)!=b[pid].get(k):raise ValueError('STOP후 표본/영속카운터변동:'+k)
        last=row
    if selected is None:raise ValueError('선택 STOP record가 journal에 없음')
    return dict(status='PASS_FIXED_STOP_JOURNAL',run_id=expected_run_id,stop_record_seq=seq,journal_records=rows,
        following_off_heartbeats=after,last_record_seq=last['record_seq'],journal_preserved_untrimmed=True,
        source_files={str(Path(p).resolve()):sha(p)for p in (summary_path,journal_path)},
        actual_runtime_samples_added=0,new_game_calls=0,limits=['STOP후 호출수/calls는 계속 증가할 수 있어 불변대상에서 제외',
        '실제 벽시계 freshness를 독립입증하지 않음. 명시 run/STOP seq/정본 완전성·후속OFF 불변범위만 검사'])
