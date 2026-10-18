"""해당 run_id의 수집 시작/종료만 요청한다. 게임의 값이나 UI는 변경하지 않는다."""
import argparse,json,time
from pathlib import Path

def main():
    p=argparse.ArgumentParser();p.add_argument('command',choices=['start','stop']);p.add_argument('--summary',type=Path,required=True);p.add_argument('--label',default='');p.add_argument('--timeout',type=float,default=15)
    a=p.parse_args();s=json.loads(a.summary.read_text(encoding='utf-8'))
    if a.command=='start':
        assert s['armed'] and s['identity_verified'],'armed/identity 확인 필요'
        assert not s.get('io_errors') and not s.get('io_lost_or_uncertain_events'),'I/O 오류가 있는 실행의 수집 시작 거절'
        assert not s.get('capture_label'),'실행당 수집 한 번: 이미 시작됨'
        assert any(x['install_status']=='INSTALLED_UNVALIDATED_RUNTIME' for x in s['probes']),'설치완료 프로브 없음'
        assert a.label and len(a.label.encode())<=200 and '\n' not in a.label and '\r' not in a.label,'짧은 단일행 label 필요'
    sequence=s.get('control_sequence',0)+1
    text=f"run_id={s['run_id']}\nsequence={sequence}\ncommand={a.command}\nlabel={a.label}\n"
    target=a.summary.parent/'capturectl.cfg';temp=target.with_suffix('.tmp');temp.write_text(text,encoding='utf-8');temp.replace(target)
    end=time.monotonic()+a.timeout
    while time.monotonic()<end:
        q=json.loads(a.summary.read_text(encoding='utf-8'))
        assert q['run_id']==s['run_id'],'실행 변경 감지'
        if q.get('control_error'):raise RuntimeError(q['control_error'])
        if q.get('control_sequence')==sequence and q.get('capture_enabled')==(a.command=='start'):
            if a.command=='start' or (q.get('in_flight')==0 and q.get('queued_events')==0 and all(x.get('enqueued')==x.get('persisted') for x in q['probes'])):
                if a.command=='stop':
                    assert not q.get('io_errors') and not q.get('io_lost_or_uncertain_events'),'기록 I/O 실패: 정상 drain으로 판정 금지'
                print(json.dumps(dict(command=a.command,run_id=q['run_id'],sequence=sequence,label=q.get('capture_label'),in_flight=q.get('in_flight'),queue_drained=a.command=='stop'),ensure_ascii=False));return
        time.sleep(.15)
    raise RuntimeError('control 또는 drain 응답 시간초과; 표본완전기록으로 판정하지 않음')
if __name__=='__main__':main()
