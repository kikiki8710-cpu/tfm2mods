# 지점 이전 0.6.4 → 0.6.5

- 구 exe A1D05045405A12E2 · 신 exe DAF040DB8A3A3AE1

| key | 구 RVA | 신 RVA | 근거 | 명령(구 → 신) |
|---|---|---|---|---|
| 0xee4418 | 0xee4418 | 0x100932f | auto: BYTES_UNIQUE(12B) bytes동일 → 0x100932f | `cmp rax, 0x28` → `cmp rax, 0x28` |
| 0xee4444 | 0xee4444 | 0x100935b | auto: BYTES_UNIQUE(12B) bytes동일 → 0x100935b | `cmp rax, 0x28` → `cmp rax, 0x28` |
| 0xee44d5 | 0xee44d5 | 0x1009475 | auto: BYTES_UNIQUE(12B) bytes동일 → 0x1009475 | `cmp qword ptr [rbp + 0x138], 0x32` → `cmp qword ptr [rbp + 0x138], 0x32` |
| 0xee4e8a | 0xee4e8a | 0x1009e2c | auto: BYTES_UNIQUE(12B) bytes동일 → 0x1009e2c | `cmp rax, 0x32` → `cmp rax, 0x32` |
| 0xee50cd | 0xee50cd | 0x100a06f | auto: BYTES_UNIQUE(12B) bytes동일 → 0x100a06f | `lea rax, [rax*4]` → `lea rax, [rax*4]` |
| 0xee50d5 | 0xee50d5 | 0x100a077 | auto: BYTES_UNIQUE(12B) bytes동일 → 0x100a077 | `lea rsi, [rax + rax*2]` → `lea rsi, [rax + rax*2]` |
| 0xee515b | 0xee515b | 0x100a0fd | auto: BYTES_UNIQUE(12B) bytes동일 → 0x100a0fd | `cmp rsi, rax` → `cmp rsi, rax` |
| 0xee4f9f | 0xee4f9f | 0x1009f41 | auto: BYTES_UNIQUE(12B) bytes동일 → 0x1009f41 | `inc rax` → `inc rax` |
| 0xee4fa2 | 0xee4fa2 | 0x1009f44 | auto: BYTES_UNIQUE(12B) bytes동일 → 0x1009f44 | `cmp rsi, rax` → `cmp rsi, rax` |
| 0xf7bbf6 | 0xf7bbf6 | 0xfc4006 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xfc4006 | `cmp rax, 0x29` → `cmp rax, 0x29` |
| 0xf7bc33 | 0xf7bc33 | 0xfc4043 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xfc4043 | `cmp rax, 0x29` → `cmp rax, 0x29` |
| 0xf7bcd6 | 0xf7bcd6 | 0xfc40e6 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xfc40e6 | `cmp dword ptr [r12 + 0x68], 4` → `cmp dword ptr [r12 + 0x68], 4` |
| 0xf7bd09 | 0xf7bd09 | 0xfc4119 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xfc4119 | `cmp dword ptr [rax + 0x68], 3` → `cmp dword ptr [rax + 0x68], 3` |
| 0xf7bd1f | 0xf7bd1f | 0xfc412f | auto: BYTES_UNIQUE(12B) bytes동일 → 0xfc412f | `cmp rax, qword ptr [r12 + 0x5b8]` → `cmp rax, qword ptr [r12 + 0x5b8]` |
| 0xf7bd27 | 0xf7bd27 | 0xfc4137 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xfc4137 | `jne 0xf7bd3e` → `jne 0xfc414e` |
| 0xfcbd0c | 0xfcbd0c | 0xf161dc | auto: BYTES_UNIQUE(12B) bytes동일 → 0xf161dc | `cmp r14, 6` → `cmp r14, 6` |
| 0xfcbdac | 0xfcbdac | 0xf1627c | auto: BYTES_UNIQUE(12B) bytes동일 → 0xf1627c | `cmp r14, 5` → `cmp r14, 5` |
| 0xfcbe6c | 0xfcbe6c | 0xf1633c | auto: BYTES_UNIQUE(12B) bytes동일 → 0xf1633c | `cmp r8, 5` → `cmp r8, 5` |
| 0xfc5041 | 0xfc5041 | 0xf09b11 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xf09b11 | `cmp r14, rsi` → `cmp r14, rsi` |
| 0xfc5044 | 0xfc5044 | 0xf09b14 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xf09b14 | `mov eax, 0x32` → `mov eax, 0x32` |
| 0xfc5049 | 0xfc5049 | 0xf09b19 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xf09b19 | `mov edx, 0x23` → `mov edx, 0x23` |
| 0xfc5052 | 0xfc5052 | 0xf09b22 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xf09b22 | `mov eax, 0x41` → `mov eax, 0x41` |
| 0xfc505b | 0xfc505b | 0xf09b2b | override: 싱글라인 후퇴 직접 비교 — 대응표·repin 일치 0xf09b2b | `cmp qword ptr [rbp + 0x1e8], rax` → `cmp qword ptr [rbp + 0x1e8], rax` |
| 0xfc57bc | 0xfc57bc | 0xf0a51e | override: 싱글라인 최종 50 — 대응표 0xf0a51e cmp rdi,0x32 (repin 동일오프셋 가정은 명령 중간) | `cmp r13, 0x32` → `cmp rdi, 0x32` |
| 0xf07f93 | 0xf07f93 | 0xf9603d | override: 라인 후퇴 최종 50(passive) — 대응표 0xf9603d cmp r13,0x32 | `cmp r12, 0x32` → `cmp r13, 0x32` |
| 0xddaf46 | 0xddaf46 | 0xe5b546 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xe5b546 | `cmp rax, 0x23` → `cmp rax, 0x23` |
| 0xdda2cf | 0xdda2cf | 0xe5a8cf | auto: BYTES_UNIQUE(12B) bytes동일 → 0xe5a8cf | `imul rcx, rcx, 0x23` → `imul rcx, rcx, 0x23` |
| 0xd5a731 | 0xd5a731 | 0xd6f931 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xd6f931 | `lea rdx, [rcx + rcx*4]` → `lea rdx, [rcx + rcx*4]` |
| 0xd5a735 | 0xd5a735 | 0xd6f935 | override: 전투 콜 도착 5×TPS — repin MID_OWNER_HEAD_UNIQUE, 같은 오프셋 0x4a5·명령 동일 | `cmp rax, rdx` → `cmp rax, rdx` |
| 0xd5aa84 | 0xd5aa84 | 0xd6fc84 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xd6fc84 | `lea rax, [rax + rax*4]` → `lea rax, [rax + rax*4]` |
| 0xd5aac7 | 0xd5aac7 | 0xd6fcc7 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xd6fcc7 | `cmp rdx, qword ptr [rbp + 0x6c0]` → `cmp rdx, qword ptr [rbp + 0x6c0]` |
| 0xd5a9c3 | 0xd5a9c3 | 0xd6fbc3 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xd6fbc3 | `lea rcx, [rcx + rcx*4]` → `lea rcx, [rcx + rcx*4]` |
| 0xef243c | 0xef243c | 0x1017bcc | auto: BYTES_UNIQUE(12B) bytes동일 → 0x1017bcc | `movabs rcx, 0x53d1ac101` → `movabs rcx, 0x53d1ac101` |
| 0xef2486 | 0xef2486 | 0x1017c16 | auto: BYTES_UNIQUE(12B) bytes동일 → 0x1017c16 | `cmp r11, rcx` → `cmp r11, rcx` |
| 0xfd43f8 | 0xfd43f8 | 0xf276e8 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xf276e8 | `cmp rax, 0x1e` → `cmp rax, 0x1e` |
| 0xde40c5 | 0xde40c5 | 0xe63da9 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xe63da9 | `imul rdx, rcx, 0x23` → `imul rdx, rcx, 0x23` |
| 0xde40d1 | 0xde40d1 | 0xe63db5 | override: 자기 위험 무시 35% 비교 — 대응표 0xe63db5 cmp rax,rdx (repin 의 0xe63db1 은 명령 중간) | `cmp rax, rdx` → `cmp rax, rdx` |
| 0xf24fab | 0xf24fab | (비활성) | override: 후반 푸시 문턱 130 — 0.6.5 로직변경(감산항 삭제, 문턱 비교 구조 바뀜 · 사라진손잡이45 판정 §1) → 설계 변경 대상, 이 버전 비활성 | — |
| 0xf24fcc | 0xf24fcc | (비활성) | override: 후반 푸시 문턱 비교 — 위와 같음 | — |
| 0xda8804 | 0xda8804 | 0xdbd25a | override: push_sustain 최소 인원 3 — 대응표 cmp r8,3 동일 | `cmp r8, 3` → `cmp r8, 3` |
| 0xda87ca | 0xda87ca | 0xdbd220 | auto: BYTES_UNIQUE(48B) bytes동일 → 0xdbd220 | `mov eax, 3` → `mov eax, 3` |
| 0xda8815 | 0xda8815 | 0xdbd26b | override: push_sustain 여유 비교 — 대응표 cmp r8,rax 동일 | `cmp r8, rax` → `cmp r8, rax` |
| 0xd5a06f | 0xd5a06f | 0xd7079f | auto: BYTES_UNIQUE(12B) bytes동일 → 0xd7079f | `cmp rax, 0x3b` → `cmp rax, 0x3b` |
| 0xd5a08f | 0xd5a08f | 0xd707bf | auto: BYTES_UNIQUE(12B) bytes동일 → 0xd707bf | `test eax, eax` → `test eax, eax` |
| 0xd5a098 | 0xd5a098 | 0xd707c8 | override: 갱 콜 수락 난수 범위 mov r8d,0x3e8 — 명령 동일(주변 call 대상만 다름) | `mov r8d, 0x3e8` → `mov r8d, 0x3e8` |
| 0xd5a0ab | 0xd5a0ab | 0xd707db | auto: BYTES_UNIQUE(12B) bytes동일 → 0xd707db | `cmp eax, 0x15e` → `cmp eax, 0x15e` |
| 0xdd56c3 | 0xdd56c3 | 0xe52923 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xe52923 | `cmp rax, 0x29` → `cmp rax, 0x29` |
| 0xdd56c9 | 0xdd56c9 | 0xe52929 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xe52929 | `add rdx, 2` → `add rdx, 2` |
| 0xdd56cd | 0xdd56cd | 0xe5292d | auto: BYTES_UNIQUE(12B) bytes동일 → 0xe5292d | `cmp r12, rdx` → `cmp r12, rdx` |
| 0xff9487 | 0xff9487 | 0xe035ed | override: 오브젝트 사냥 위험 배수 lea rax,[rax+rax*2] (flag0/event5) — 뒤따르는 mov cl,1 꼴로 짝 확정 | `lea rax, [rax + rax*2]` → `lea rax, [rax + rax*2]` |
| 0xff9570 | 0xff9570 | 0xe035f3 | override: 오브젝트 사냥 위험 비교 cmp rdx,rax — 디스어셈 대조 | `cmp rdx, rax` → `cmp rdx, rax` |
| 0xff955e | 0xff955e | 0xe03492 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xe03492 | `lea rcx, [rax + rax*2]` → `lea rcx, [rax + rax*2]` |
| 0xff951f | 0xff951f | 0xe03608 | override: 오브젝트 사냥 위험 배수 lea rax,[rax+rax*2] (flag≠0/event5) — cmp bl,5 뒤 꼴로 짝 확정 | `lea rax, [rax + rax*2]` → `lea rax, [rax + rax*2]` |
| 0xff9597 | 0xff9597 | 0xe035b4 | auto: BYTES_UNIQUE(12B) bytes동일 → 0xe035b4 | `lea r8, [rax + rax*2]` → `lea r8, [rax + rax*2]` |
| 0xff95b2 | 0xff95b2 | 0xe035cb | override: 오브젝트 사냥 후퇴 30% — 0.6.5 에서 사본 3개(RE A9: e035cb/e03612/e0361c) | `cmp r14, 0x1e` → `cmp r15, 0x1e` |
| 0xff95b2#2 | 0xff95b2 | 0xe03612 | override: 오브젝트 사냥 후퇴 30% — 0.6.5 에서 사본 3개(RE A9: e035cb/e03612/e0361c) | `cmp r14, 0x1e` → `cmp r15, 0x1e` |
| 0xff95b2#3 | 0xff95b2 | 0xe0361c | override: 오브젝트 사냥 후퇴 30% — 0.6.5 에서 사본 3개(RE A9: e035cb/e03612/e0361c) | `cmp r14, 0x1e` → `cmp r15, 0x1e` |
| 0xff952e | 0xff952e | 0xe035d3 | override: 오브젝트 사냥 후퇴 29 비교(cmp r14,0x1d → cmp r15,0x1d) — 디스어셈 대조 | `cmp r14, 0x1d` → `cmp r15, 0x1d` |
