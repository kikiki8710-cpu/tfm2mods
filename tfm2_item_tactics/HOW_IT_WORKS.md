# tfm2_item_tactics — How It Works (v2.8.1 · game 0.5.4)

> Written in actual chronological order: game boots → you designate items → a match starts →
> purchases happen → the screen is drawn. RVA values are for **0.5.4** and change with every game
> patch — the constants in the source (`lib.rs` / `ui_inject.rs`) are canonical. 한국어판: `동작원리.md`.

## 0. When the game boots — mod preparation

1. **Version gate**: if the exe size does not match 0.5.4, the mod installs *no hooks at all* and
   disables itself (logged to `version_gate.txt`). This is the first safety layer — never run with
   shifted addresses.
2. **Two byte patches** (when slots=4): ① the game's "stop buying at 3 items" comparison constant
   3→4 (`cmp [athlete+0x448], 3`) ② the slot-limit branch (`jbe`) inside the buy resolver. Together
   they make the game treat a 4th item as legal to buy.
3. **Hook installation**: checked each frame in post_update. Entry points shared with other mods
   (serpen on the launcher) are **chain-hooked** — we wait for the foreign hook to appear, then embed
   its jump target into our trampoline so both detours fire in sequence. Installed exactly once,
   then only a once-per-second self-heal check.
4. **Modded-item registry**: scans the game DB for non-vanilla item arrays and builds a catalog
   (`MOD_REGISTRY`). ⚠ A fresh save must be saved and the game restarted before modded items merge
   into the DB, so the scan retries up to 500 times and logs its failure reason to
   `item_tactics_registry.txt`.
5. **Acquiring my team id (pid)**: from both the InGame screen and the management tick
   (server_state) — whichever comes first. pid is used to publish my starting roster
   (`MY_ATHLETES` = the five athlete_ids of db.team(pid).last_starting).

## 1. When you designate items in the tactics screen

1. The moment the tactics/training screen loads, the **UI loader/parser hooks append-inject** the
   4th-slot node into each row (append + runtime coordinates, never replace — so nodes added by
   other mods survive).
2. Dropdown options are built through the game's own function (`FN_DD_SETOPT`): six vanilla stats +
   Auto + items from **enabled** item mods (disabled mods filtered via `ModItemEntry+0x190`).
   Labels are game i18n references (`#asset/base/text/...`), so they localize automatically.
3. When you pick something, the mod updates its (champion, slot) → item snapshot and **persists it
   to `item_tactics_sel.txt`**. The format stores a **category number (1–6) or an item key string**,
   never a list index — so designations survive mods being toggled on/off. Designations made inside
   composition test are stored under **per-side (blue/red) keys**, separate from league play.
4. Nothing touches the game at this point — a designation is just a reservation. The mod only
   intervenes inside matches.

## 2. When a match starts — classifying "what match is this?"

Every simulation, on-screen or background, starts through one game function (the **launcher**).
Our detour runs once per match start and classifies it by **who called the launcher (return address)**:

| Caller (retaddr) | Kind | What the detour does |
|---|---|---|
| scene builder ×2 | **on-screen match** (spectate / my match) | captures that match's **seed (r8) into `LIVE_SEED`** — the key used later to recognize the on-screen sim at buy time |
| comp-test ×2 | on-screen (main / replay-from-record) | seed capture + `COMPTEST_MATCH` flag (team-gate bypass) |
| worker.rs | **tournament (league) background sim** | **TN capture** — §3 |
| solo_rank / meta sims | other background sims | nothing (membership judgment is sufficient there) |

Right after, when the game constructs the RNG provider, the **seed-ctor hook** checks "is this
provider being built for `LIVE_SEED`?" and remembers the pointer as `RENDER_PROVIDER`. The seed
stays inside the provider (`+0xeb28`) as its initial value, so comparing it against the provider
that buy() carries tells us **exactly which sim a purchase belongs to**.

## 3. TN — tournament background matches are identified at creation time, teams included

> **TN is short for "TourNament"** — the prefix of this feature's code symbols
> (`TN_*`, `tourn_capture`), reused in this document.

Tournament background sims are launched by a rayon worker, and the **match execution record lives
in that caller's stack frame**. From the launcher's entry rsp we compute `caller_rbp = rsp+0x88`, then:

1. Read `cfg = [rbp+0x1cdc0]` (background-sim state), `set_end = [rbp+0x1cce0]` (end of the set
   block about to run), `db = [rbp+0x1cde8]` — **check ①: db must equal the server db we know**.
2. Reverse-scan the hashmaps at `cfg+0x2a0/+0x2d0` (regular season / postseason) for the record
   whose set-Vec exactly contains `set_end` (check ②), plus match-key self-consistency /
   not-finished / map-byte (check ③).
3. Record `+0x140/+0x148` = the two team ids. Using the set's side byte (`set+0xf8`):
   **side0 (blue) team = `rec+0x140+(sb^1)×8`** — proven statically, watched in-game by a
   permanent NG=0 counter.
4. Publish `(seed → side0 team, side1 team)` into a **16-slot ring table** — the third judgment
   source in §4.

If the scan doesn't match, we simply give up (= previous behavior). TN is built so it cannot lose
anything by being wrong.

## 4. When an athlete tries to buy an item — judgment and injection (the core)

Every purchase attempt in a game sim calls the **buy function**, and our detour runs before the original:

**(a) Which match is this purchase from?** Read the seed from the provider (r9 argument):
equal to `LIVE_SEED` → the on-screen match; found in the TN table → a tournament background match;
neither → some other background sim.

**(b) Is this athlete on my team?** Three sources, **OR-combined (any single hit approves)**:

| Source | Coverage | Principle |
|---|---|---|
| ① scene side | on-screen matches | read the match info from the db (two team ids + is_team1_blue), compute my side, compare with the athlete's side (`athlete+0x810`). Permanently cross-checked against ② on a 1/256 sample |
| ② athlete-id membership | background sims in general | the athlete's `athlete_id (+0x800)` ∈ `MY_ATHLETES` |
| ③ TN | tournament background | my side from the §3 table vs the athlete's side — rescues athletes ② misses (**substitutes, unregistered athlete ids**) |

- ③ is **approve-only** (never blocks) — a strict superset of previous behavior; a TN mistake can
  never cost an injection.
- Composition test bypasses the team gate entirely (both sides are user-built; screen-verified only).
- "Undecidable" means **no injection** — avoiding contamination of other teams always wins.
- Non-my athletes pass through untouched right here (~94 % of background buys take this early-out;
  its cost is one VEH read + a hash-set lookup).

**(c) Injection** — for a confirmed my-team athlete, overwrite the **goal-build array**
(build Vec at `+0x480/488/490`):

- Slots 0–2: the designated item's catalog index — but **only into slots where
  `owned ≤ slot index`** (items already bought are never touched).
- Slot 3: the designated item if any; on Auto, a **shadow-call into the game's item neural net**
  (5-feature scoring) picks the highest-scoring candidate.

**(d) The game does the buying** — the mod only replaces the *goal*. The vanilla buy resolver
builds up toward it (t0→t4) with normal gold and timing; the §0 byte patches merely allow owning four.

## 5. When the match is drawn on screen

The vanilla UI renders only three item boxes (node names are hard-coded). The **GameView update
hook** reads the view model each frame, **draws the 4th item's icon node directly**, and on hover
calls the game's own tooltip-show function with all 11 arguments — a genuine tooltip, zero code
surgery on the game.

## 6. Always-on self-diagnostics

`item_tactics_registry.txt` is kept up to date with its reading rules embedded: pid / roster
publication state, gate pass/block counters (★ "blocked because undecidable" must be 0),
scene↔membership cross-check (mismatch must be 0), TN scan success rate, side-mapping NG (must
be 0), and TN rescue counts. **One file is enough to triage any "injection didn't happen" report.**

## Appendix — hook & patch addresses (0.5.4)

| Site | RVA | | Site | RVA |
|---|---|---|---|---|
| UI loader / parser | `0x2e35d0` / `0x1a3ce0` | | buy (injection) | `0xe767e0` |
| dropdown set-options | `0x1c1ad0` | | GameView update | `0xaa06c0` |
| launcher | `0x13b53d0` | | tooltip show | `0x236dc00` |
| seed-ctor | `0x14e16d0` | | item-net forward | `0x145a680` |
| owned-cap patch | `0x1420b30` (imm) | | gate3 patch | `0xe76b24` (jbe) |

Files: `4items.cfg` (slots 3/4, release default 3) · `item_tactics_sel.txt` (designation
persistence — deleting it loses designations) · `item_tactics_registry.txt` (self-diagnostics) ·
`version_gate.txt` (version-gate log)
