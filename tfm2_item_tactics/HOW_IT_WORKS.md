# tfm2_item_tactics — How It Works (v2.8.0 · game 0.5.4)

> This document describes only the paths that are actually running today. RVA values are for **0.5.4**
> and change with every game patch — the constants in the source (`lib.rs` / `ui_inject.rs`) are always
> the canonical reference. 한국어판: `동작원리.md`.

## 1. What the mod does

- Expands item slots **3 → 4** (`slots = 4` in `4items.cfg`; `3` keeps three slots but still allows designation)
- Lets you **designate an item for every slot 1–4** in the personal-tactics screen (vanilla categories
  plus modded items; applied to your own team only)
- If slot 4 is left on Auto, it is **auto-recommended by the game's item neural net**
- Draws the **4th item icon + tooltip during matches** (the vanilla UI only renders three)
- In composition test, designations are **per-side (blue/red)** and stored separately from league play
- Items from disabled item mods are automatically filtered out of the dropdowns

## 2. Hook map — where the mod intervenes

| # | Target (0.5.4 RVA) | Technique | Purpose |
|---|---|---|---|
| H1 | UI loader `0x2e35d0` / parser `0x1a3ce0` | trampoline detour (`ui_inject.rs`) | When tactics/training screens load, **append-inject** the 4th-slot dropdown nodes (preserves nodes added by other mods) |
| H2 | dropdown set-options `0x1c1ad0` | direct game-function call | Builds dropdown options — 6 vanilla stats + Auto (localized via the game's own i18n references) + active modded items |
| H3 | match launcher `0x13b53d0` | entry detour (minimal body: atomics + VEH reads only) | Fires once per simulation start. ① captures the **seed** of on-screen matches (`LIVE_SEED`) ② classifies the match by return address (screen / comp-test / background) ③ for tournament background matches, captures the **TN descriptor** (§5) |
| H4 | seed-ctor `0x14e16d0` | entry detour | Captures the on-screen match's **provider pointer** (`RENDER_PROVIDER`) — used by the buy hook for is_live matching |
| H5 | **buy `0xe767e0`** | entry detour | ★**the injection site** — §3 |
| H6 | GameView update `0xaa06c0` | entry detour | During matches, reads the view model directly to **draw the 4th icon node**, and on hover calls the game's own tooltip-show function (`0x236dc00`, 11 args) |
| P1 | owned-cap byte patch (imm of `cmp …+0x448, 3` → 4) | always on (mode 4) | Lifts the game's "stop buying at 3 items" cap to 4 |
| P2 | gate3 byte patch (`jbe` inside the buy resolver) | always on (mode 4) | Routes the 4th slot through the normal build-up path |

Multi-mod coexistence: entry points shared with other mods (e.g. serpen on the launcher) use
**chain hooking** — if a foreign hook (movabs+jmp) is present, its target is embedded in our trampoline
so both detours fire in sequence. Installed late (post_update), exactly once.

## 3. The injection site — buy detour in detail

Every time an athlete in a game simulation attempts to buy an item, the buy function runs and our
detour executes first:

1. **Match identification**: read the seed at `provider(r9)+0xeb28` and compare with `LIVE_SEED`
   → is this the on-screen match (is_live)?
2. **Team gate** (§4) — if the athlete is not on my team, pass through untouched.
3. **Goal-build injection**: for a confirmed my-team athlete, write into the athlete's build Vec
   (`+0x480/+0x488/+0x490`):
   - Slots 0–2: the catalog index of the designated item, but **only into slots where
     `owned <= slot index`** (items already bought are never touched).
   - Slot 3: the designated item if any; on Auto, a **shadow-call into the item neural net**
     (`itemnet forward 0x145a680`, 5-feature scoring) picks the best candidate.
4. **Actual purchasing stays 100% vanilla**: the game's buy resolver builds up toward the goal
   (t0→t4) with normal gold and timing. The mod only replaces the *goal*; P1/P2 merely allow four
   items to be owned.

So the injection is exactly one thing — **overwriting the goal build array** — and the game does the rest.

## 4. Team gate — three sources OR-combined (any single hit approves injection)

| Source | Matches covered | Principle |
|---|---|---|
| ① scene side | on-screen matches | Read match info from the db (`+0x17A8`/`+0x17C8` two team ids + `+0x1900` is_team1_blue) → compute my side → compare with athlete side (`+0x810`). A 1/256 sample cross-checks against ② permanently |
| ② athlete-id membership | background sims in general | `athlete_id (+0x800)` ∈ my starting roster (`MY_ATHLETES` = db.team(pid).last_starting) |
| ③ TN descriptor | **tournament background sims** (new in v2.8.0) | §5 — team ids and sides fixed at match-creation time; rescues athletes ② misses (substitutes, unregistered athlete ids) |

- ③ is **approve-only** — never used to block (a strict superset of previous behavior; zero risk of
  losing injections to a false negative).
- Composition test bypasses the team gate entirely (both sides are user-built; screen matches only,
  identified by launcher return address).
- pid (my team id) is acquired from both InGame and the management tick (whichever comes first).

## 5. TN — the tournament background match descriptor (mapped v2.7.4–v2.8.0)

> **TN is short for "TourNament".** It is the prefix used by this feature's code symbols
> (`TN_ENABLED`, `TN_GATE`, `TN_TAB_*`, `tourn_capture`, …), and this document reuses it.

Tournament (league) background matches are launched by a rayon worker. The **match execution record
lives in the caller's stack frame**:

- From the launcher's entry rsp: `caller_rbp = rsp+0x88`, then `cfg = [rbp+0x1cdc0]`,
  `set_end = [rbp+0x1cce0]`, `db = [rbp+0x1cde8]` (the server db — self-check ①).
- Reverse-scan the hashmaps at `cfg+0x2a0/+0x2d0` (0x160-byte entries) for the record whose set-Vec
  contains `set_end` (check ②), plus key self-match / not-finished / map-byte (check ③).
- Record `+0x140/+0x148` = the two team ids. With the set's side byte (`set+0xf8`):
  **side0 (blue) team = `rec+0x140+(sb^1)×8`** — statically proven, with a permanent in-game
  NG=0 watchdog.
- Each capture publishes `(seed → side0 team, side1 team)` into a 16-slot ring table; the buy hook
  looks it up by provider seed.

Screen matches and solo-rank background matches do not take this path (the solo-rank record was
proven to contain no team ids at all).

## 6. Files (mod folder)

| File | Purpose |
|---|---|
| `4items.cfg` | `slots = 3/4` toggle (release default: 3) |
| `item_tactics_sel.txt` | designation persistence (vanilla category number 1–6, or item key string — survives list changes). ⚠ deleting it loses designations |
| `item_tactics_registry.txt` | **self-diagnostics** (reading rules embedded) — one file is enough to triage an injection-failure report |
| `version_gate.txt` | version gate log — the mod deactivates itself if the exe is not 0.5.4 |

## 7. Safety

- Every raw pointer read is **VEH-protected** (`safe_read_*`) — stale pointers return None instead of crashing.
- The launcher detour has a minimal body (the function owns a 91 KB stack frame; no format!/locks/file IO — atomics only).
- Version gate: on exe size mismatch no hooks are installed at all (prevents silent misbehavior).
- A team-gate result of "undecidable" means **no injection** — avoiding contamination always wins over injecting.
