"""명시한 run_id의 정본 journal로 수집 요청·drain ack를 확인한다. 게임/UI 변경 없음."""
import argparse
import json
import os
import re
import sys
import time
from pathlib import Path

TAIL_LIMIT = 1024 * 1024
RUN_PATTERN = re.compile(r"[0-9]+_[0-9]+\Z")


class SummaryPending(RuntimeError):
    """새 정본의 완전한 줄이 아직 없거나 현재 snapshot이 오래된 상태."""


def _unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"중복 JSON 키: {key}")
        result[key] = value
    return result


def _decode(text):
    value = json.loads(text, object_pairs_hook=_unique_object)
    if not isinstance(value, dict):
        raise ValueError("summary는 JSON 객체여야 함")
    return value


def _integer(value):
    return isinstance(value, int) and not isinstance(value, bool) and value >= 0


def load_summary(summary, run_id, max_age=5.0, tail_limit=TAIL_LIMIT):
    """새 journal이 있으면 alias/다른 실행으로 회귀하지 않고 그 실행만 읽는다."""
    if not RUN_PATTERN.fullmatch(run_id):
        raise ValueError("run_id는 PID_timestamp 형식 필요")
    summary = Path(summary)
    journal = summary.parent / f"summaries_{run_id}.jsonl"
    if journal.exists():
        # fstat 시점 길이까지만 읽어 계속 append되는 파일에서도 읽기량이 유한하다.
        with journal.open("rb") as stream:
            size = os.fstat(stream.fileno()).st_size
            start = max(0, size - tail_limit)
            stream.seek(start)
            data = stream.read(size - start)
        if start:
            first = data.find(b"\n")
            data = data[first + 1:] if first >= 0 else b""
        end = data.rfind(b"\n")
        if end < 0:
            raise SummaryPending("명시 run journal의 완전한 줄 대기; 이전 alias 사용 금지")
        complete = data[:end].split(b"\n")
        # 부분 끝행만 무시한다. 마지막 완전행 손상은 이전 정상행으로 숨기지 않는다.
        value = _decode(complete[-1].decode("utf-8"))
        if value.get("summary_source") != "journal":
            raise ValueError("정본 journal 표식 누락")
        if not _integer(value.get("record_seq")) or value["record_seq"] == 0:
            raise ValueError("정본 record_seq 누락/오류")
        timestamp = value.get("summary_timestamp_unix_ms")
        if not _integer(timestamp):
            raise ValueError("정본 timestamp 누락/오류")
        if max_age is not None:
            age = time.time() - timestamp / 1000
            if age > max_age or age < -2:
                raise SummaryPending(f"정본 snapshot freshness 불충족: age={age:.3f}s")
        source = journal
    else:
        # 요청한 run의 journal이 아직 안 열렸더라도 다른 journal이 있으면 legacy 회귀 금지.
        if any(summary.parent.glob("summaries_*.jsonl")):
            raise SummaryPending("명시 run journal 없음; 다른 실행/alias 회귀 금지")
        try:
            value = _decode(summary.read_text(encoding="utf-8"))
        except FileNotFoundError as error:
            raise SummaryPending("summary 생성 대기") from error
        if value.get("summary_source") == "journal":
            raise SummaryPending("alias가 가리키는 정본 journal 없음; alias로 회귀 금지")
        source = summary
    if value.get("schema_version") != 1 or value.get("run_id") != run_id:
        raise ValueError("명시 run_id/schema 불일치")
    return value, source


def ack_ready(value, sequence, command, initial_record_seq=0, journal=False):
    """stop은 정본 ack·진행0·큐0·기록수 일치와 hard IO0을 모두 요구한다."""
    if value.get("control_sequence") != sequence or value.get("capture_enabled") != (command == "start"):
        return False
    if journal and value.get("record_seq", 0) <= initial_record_seq:
        return False
    if command == "start":
        return True
    pending = value.get("queue_pending", value.get("queued_events"))
    if value.get("in_flight") != 0 or pending != 0:
        return False
    probes = value.get("probes")
    if not isinstance(probes, list):
        return False
    if not all(_integer(x.get("enqueued")) and _integer(x.get("persisted")) and x["enqueued"] == x["persisted"] for x in probes):
        return False
    if value.get("io_errors") != 0 or value.get("io_lost_or_uncertain_events") != 0:
        raise RuntimeError("정본/관측 hard I/O 오류: 정상 drain으로 판정 금지")
    return True


def write_control(path, text):
    """제어 파일 교체의 공유 오류도 유한하게만 재시도하며 실패를 숨기지 않는다."""
    temp = path.with_suffix(".tmp")
    temp.write_text(text, encoding="utf-8")
    for attempt in range(21):
        try:
            temp.replace(path)
            return
        except OSError as error:
            if error.winerror not in (5, 32, 33) or attempt == 20:
                raise
            time.sleep(.05)


def _wait_initial(summary, run_id, timeout, max_age):
    deadline = time.monotonic() + timeout
    while True:
        try:
            return load_summary(summary, run_id, max_age)
        except (SummaryPending, PermissionError):
            if time.monotonic() >= deadline:
                raise
            time.sleep(.15)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=["start", "stop"])
    parser.add_argument("--summary", type=Path, required=True, help="구 summary 경로; 같은 폴더의 명시 run journal 우선")
    parser.add_argument("--run-id", required=True)
    parser.add_argument("--label", default="")
    parser.add_argument("--timeout", type=float, default=15)
    parser.add_argument("--max-age", type=float, default=5)
    args = parser.parse_args()
    if args.timeout <= 0 or args.max_age <= 0:
        parser.error("timeout/max-age는 양수 필요")
    initial, source = _wait_initial(args.summary, args.run_id, args.timeout, args.max_age)
    if args.command == "start":
        if not initial.get("armed") or not initial.get("identity_verified") or initial.get("init_ready") is False:
            raise RuntimeError("armed/identity/초기 준비 확인 필요")
        if initial.get("io_errors") != 0 or initial.get("io_lost_or_uncertain_events") != 0:
            raise RuntimeError("hard I/O 오류가 있는 실행의 수집 시작 거절")
        if initial.get("capture_label") or initial.get("stop_pending_sequence", 0):
            raise RuntimeError("실행당 수집 한 번: 이미 시작/종료 요청됨")
        if not any(x["install_status"] == "INSTALLED_UNVALIDATED_RUNTIME" for x in initial["probes"]):
            raise RuntimeError("설치 완료 프로브 없음")
        if not args.label or len(args.label.encode()) > 200 or any(c in args.label for c in "\n\r\0"):
            raise RuntimeError("짧은 단일행 label 필요")
    sequence = max(initial.get("control_sequence", 0), initial.get("stop_pending_sequence", 0)) + 1
    text = f"run_id={args.run_id}\nsequence={sequence}\ncommand={args.command}\nlabel={args.label}\n"
    write_control(args.summary.parent / "capturectl.cfg", text)
    deadline = time.monotonic() + args.timeout
    initial_seq = initial.get("record_seq", 0)
    while time.monotonic() < deadline:
        try:
            value, current_source = load_summary(args.summary, args.run_id, args.max_age)
        except (SummaryPending, PermissionError):
            time.sleep(.15)
            continue
        if current_source != source:
            raise RuntimeError("제어 요청 뒤 정본 source 변경 감지")
        if value.get("control_error"):
            raise RuntimeError(value["control_error"])
        if ack_ready(value, sequence, args.command, initial_seq, source.suffix == ".jsonl"):
            print(json.dumps(dict(command=args.command, run_id=args.run_id, sequence=sequence,
                label=value.get("capture_label"), in_flight=value.get("in_flight"), queue_drained=args.command == "stop",
                summary_source=str(source), record_seq=value.get("record_seq"),
                summary_alias_errors=value.get("summary_alias_errors", 0)), ensure_ascii=False))
            return
        time.sleep(.15)
    raise RuntimeError("정본 control/drain ack 시간초과; 표본 완전기록으로 판정하지 않음")


if __name__ == "__main__":
    if "--self-test" in sys.argv:
        import tempfile
        import unittest

        class JournalTests(unittest.TestCase):
            def setUp(self):
                self.temp = tempfile.TemporaryDirectory(prefix="aiav_control_own_")
                self.addCleanup(self.temp.cleanup)
                self.root = Path(self.temp.name)
                self.alias = self.root / "summary.json"
                self.run = "1_100"
                self.journal = self.root / f"summaries_{self.run}.jsonl"

            def record(self, **changes):
                value = dict(schema_version=1, run_id=self.run, summary_source="journal", record_seq=1,
                    summary_timestamp_unix_ms=int(time.time()*1000), control_sequence=2, capture_enabled=False,
                    in_flight=0, queue_pending=0, io_errors=0, io_lost_or_uncertain_events=0,
                    probes=[dict(enqueued=4, persisted=4)])
                value.update(changes)
                return value

            def put(self, value):
                self.journal.write_text(json.dumps(value)+"\n", encoding="utf-8")

            def test_journal_beats_stale_alias(self):
                self.alias.write_text(json.dumps(dict(run_id="old")))
                self.put(self.record())
                value, source = load_summary(self.alias, self.run)
                self.assertEqual(source, self.journal)
                self.assertEqual(value["run_id"], self.run)

            def test_partial_tail_uses_last_complete(self):
                self.put(self.record())
                with self.journal.open("ab") as writer:
                    writer.write(b'{"partial":')
                value, _ = load_summary(self.alias, self.run)
                self.assertEqual(value["record_seq"], 1)

            def test_empty_current_does_not_fallback(self):
                self.alias.write_text(json.dumps(self.record()))
                self.journal.write_bytes(b"")
                with self.assertRaises(SummaryPending):load_summary(self.alias, self.run)

            def test_wrong_run_journal_never_fallback(self):
                self.alias.write_text(json.dumps(self.record()))
                self.put(self.record())
                with self.assertRaises(SummaryPending):load_summary(self.alias, "2_200")

            def test_complete_corruption_is_error(self):
                self.put(self.record())
                with self.journal.open("ab") as writer:writer.write(b'not-json\n')
                with self.assertRaises(ValueError):load_summary(self.alias, self.run)

            def test_run_id_and_freshness(self):
                self.put(self.record(run_id="2_200"))
                with self.assertRaises(ValueError):load_summary(self.alias, self.run)
                self.put(self.record(summary_timestamp_unix_ms=1))
                with self.assertRaises(SummaryPending):load_summary(self.alias, self.run)

            def test_legacy_only_without_journal(self):
                legacy = self.record()
                legacy.pop("summary_source")
                self.alias.write_text(json.dumps(legacy))
                _, source = load_summary(self.alias, self.run)
                self.assertEqual(source, self.alias)

            def test_stop_requires_new_record_and_exact_drain(self):
                value = self.record(record_seq=5)
                self.assertTrue(ack_ready(value, 2, "stop", 4, True))
                self.assertFalse(ack_ready(value, 2, "stop", 5, True))
                for key, bad in [("in_flight", 1), ("queue_pending", 1), ("control_sequence", 1)]:
                    self.assertFalse(ack_ready(dict(value, **{key:bad}), 2, "stop", 4, True))
                for key in ["io_errors", "io_lost_or_uncertain_events"]:
                    with self.assertRaises(RuntimeError):ack_ready(dict(value, **{key:1}), 2, "stop", 4, True)
                self.assertFalse(ack_ready(dict(value, probes=[dict(enqueued=4,persisted=3)]),2,"stop",4,True))

            def test_actual_file_stop_ack_ignores_alias(self):
                self.put(self.record(control_sequence=1, record_seq=4))
                initial, source = load_summary(self.alias, self.run)
                with self.journal.open("a", encoding="utf-8") as writer:
                    writer.write(json.dumps(self.record(control_sequence=2, record_seq=5))+"\n")
                self.alias.write_text(json.dumps(self.record(control_sequence=99, record_seq=99)))
                latest, same_source = load_summary(self.alias, self.run)
                self.assertEqual(source, same_source)
                self.assertTrue(ack_ready(latest, 2, "stop", initial["record_seq"], True))
                self.assertFalse(ack_ready(latest, 99, "stop", initial["record_seq"], True))

            def test_new_alias_cannot_replace_missing_journal(self):
                self.alias.write_text(json.dumps(self.record()))
                with self.assertRaises(SummaryPending):load_summary(self.alias, self.run)

            def test_duplicate_keys_rejected(self):
                with self.assertRaises(ValueError):_decode('{"x":1,"x":2}')

            def test_control_file_owned_fixture(self):
                target = self.root / "capturectl.cfg"
                write_control(target,"run_id=1_100\nsequence=1\ncommand=stop\n")
                self.assertIn("sequence=1",target.read_text())

        unittest.main(argv=[sys.argv[0]], verbosity=2)
    else:
        main()
