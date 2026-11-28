# A2-MORSE-LESSON

Producer delta. morseLesson rides this release. No new dependencies and no new assets. `build3.py` already strips `export` from `gdial.js` and concatenates it with `game3.js`, so `dialLock` calls `morseLesson` in the same scope. `cardBySymbol`, `reviewCard`, `persistMorse`, and `morseDeck` are already in that scope.

## Source

`86ba3f945722af3d53f0864b5f2d038c31d00c4e` (`86ba3f9`) `builds/demo70`

Taken from there:

- `gdial.js` `morseLesson` (the +28 lines between `decodeMorse` and `isSos`, comment included)
- `game3.js` `dialLock` lesson branch (demo70 ~5906–5917): a locked S or O toasts the lesson; the chart toast remains the fallback
- the four `morseLesson` pins from `test3.mjs` ~4208–4217

## Target bases

- `gdial.js` and `game3.js`: `builds/demo68` at `6931bc293153e889257a02bf217521c4ebd1026f` (`6931bc2`)
- Pins: nova-fox ref `a2-dial` @ `65f7dcb` is not in the repo. `git ls-remote` has no such ref, and `git fetch origin 65f7dcb` failed (`couldn't find remote ref`). Pins are based on `builds/demo69/test3.mjs` at `d53e173396cc6e6ceeef0067864e542b9bd6d612` (`d53e173`).

The pin anchor (the flight-gate range check, then the refused-shift check) and the `gdial.js` import line are the same text in demo68 and in that demo69 file. The delivered patch therefore uses `builds/demo68/` paths, and a path rewrite `builds/demo68/` → `builds/demo69/` lands the same pins in the stacked suite.

At `d53e173` the plain chart toast is `builds/demo69/game3.js:6182` (`dialLock` at 6175–6210). The failure branch matches demo68, so the hunk applied there with no retarget.

## Files touched

| file | delta |
| --- | --- |
| `builds/demo68/gdial.js` | +28 |
| `builds/demo68/game3.js` | +11 / −1 |
| `builds/demo68/test3.mjs` | +11 / −1 (import `morseLesson`, four pins) |

Line delta: **+50 / −2** (net +48). Patch sha256 `a3e682610ab3d7e639fed1cf712b1c005aa5f676e349eee22174b34d90991468`.

## Apply-check

Scratch clone of `wbic16/nova-fox`. Read only. Nothing pushed there.

- `git apply --check` on demo68 @ `6931bc2`: **clean** (exit 0) for `game3.js`, `gdial.js`, `test3.mjs`.
- Same patch with paths rewritten to `builds/demo69/`, on demo69 @ `d53e173`: **`git apply --check` clean** (exit 0), then `git apply` clean (exit 0). Lesson landed at `game3.js:6181`, pins at `test3.mjs:4276`.

## Tests

Runner: `node test3.mjs` in `builds/demo69` (the runner named in that tree's `PLAN.md`), after the path-adapted apply, pins included.

**1051 passed, 0 failed** (exit 0). The four new pins passed:

- play: locking S is a lesson, not a missed coordinate
- play: locking O names the three dashes
- play: S.O.S names home in the lesson
- play: a word that is not one mark is not a lesson

Re-ran after `python3 build3.py --lite`: **1051 passed, 0 failed** again.

## Lite size

`build3.py --lite` exists and exited 0. Stripped Lite module: **1,826,274 bytes** (1.826 MB), under the 1.9 MB cap. `node --check` on that module passed inside the build. Loader `lite.html` 10,856 bytes. Package `nova-fox.phext.gz` 426,820 bytes. Full `index.html` 6,022,548 bytes (module 5,959,585).

## Left in demo70

These neighbors are not morseLesson, so they are not in the patch:

- `game3.js`: locked-range copy, Joy's deck sentence, `joyMorseOnExplore("enter")` on open, the move-lesson threshold, the objective strings, and the `gdial-sos` click handler (one-button S.O.S. is already A2-DIAL on the demo69 stack)
- `test3.mjs` 4218–4220, inside the cited ~4208–4220 window: `play: the dial sits outside the hidden console`, plus the short-walk, opening-objective, and key-S.O.S. pins that follow it

Nothing in the morseLesson hunks needed a manual edit. The only base substitution is the missing `a2-dial` ref, described above.
