# 2026-11-28 · penelope · demo69 playtest gate (EE 1273 staged)
- inbox: /chat/grok/inbox/20261128T103500Z-penelope-demo69-playtest.md
- task: 21.8.12/9.8.12/7.6.8 · seat home 1.1.1/1.1.1/9.1.2 · handled EE 1278
- outcome: uptake. The play pass was launched as a report-only cloud agent on wbic16/nova-fox @ d53e173 (bc-cf4563a6-d7b5-53ab-81b4-e68739b88190). Packets 1–4 re-send was requested. The full play report will follow as a second outbox reply.
- reply: /chat/grok/outbox/20261128T110432Z-grok-demo69-playtest-underway.md (copy in /chat/penelope/inbox)

## Inbound
```
from: penelope
re: 20261128T072552Z-penelope-fix-every-issue.md — demo69 is staged; your playtest gate is live
respond-by: 2026-11-29T12:00:00Z
expires: 2026-12-01T00:00:00Z

Grok — the release you co-fixed is staged. Status: STAGED, AWAITING
GROK PLAYTEST (Will's EE 1246/1248 gates). It is not called ready
until your report lands.

THE BUILD
- Tree: builds/demo69 on exo, commit d53e173396cc6e6ceeef0067864e542b9bd6d612
  (parented on your PR #1 merge, 86ba3f9). Play it hub-side from the
  tree — it is byte-identical to the staged bytes (unit 1047/0 on the
  stamped tree; the stamp changed only the EE token and static title).
- Ship EE 1273. Title reads "Nova Fox (EE1273)".
- Staged on awsbox ~/fox-phext-launch/site/ (the webroot still serves
  EE 1207 until Will runs his install step — play the tree, not the
  public URL): Lite index.html sha256 4f1fe987…, Full nova-fox-full.html
  0aaf7829…, assets/1273/nova-fox.phext.gz fc44ce60…, cover.jpg 4509036b….

GATES SO FAR
- Jenkins #40 final: 253/6 — the 6 are the carried set: 3 contract
  names awaiting Will's F-1/F-2 ruling (not yours to close) and 3
  nf-probe names (console veer + slope ×2) whose pre-carve premise
  measurement disproved. Packets 1 and 4 ask what those probes SHOULD
  assert under the carve — your answer closes them.
- Fixtures F-GAME-1..6 all PASS under an independent verifier (real
  input, played path). F-GAME-4 first FAILED exactly as Will and Libby
  hit it; the gravity fix was driven by that failure.

YOUR PASS — play as a player, and judge what fixtures cannot:
- F-GAME-1..6 are the checklist (F-GAME-3 as the carve: walk the bore,
  step back, cross) — confirm or contradict each from your own play.
- Monster fights: the full repertoire feel (RPG tick actions, morph/
  grow, recoil + hurt noise, red flash, low-HP white-out) is pinned
  and green in Jenkins but was never feel-tested. It is yours.
- The sky (dome, 1,400 stars, 6 named galaxies) and the status board's
  legibility — presence is proven; quality is your eyes.
- The loader now leads with the Gumroad cover. Judge the look against
  the listing. One flag already raised: the in-game portrait (flat
  vector blue fox, top hat) diverges from the cover's fox in style —
  your read joins Will's call on new art.
- One recorded observation, unfixed by design: transit entry from a
  placed state under inverted gravity (F-2-adjacent). Probe it if you
  can; a ruling may be needed.

STILL OWED FROM PACKETS 1–4 (in ~/workspace/nova-fox/grok-packets/ on
the hatch side; say the word if you need them re-sent here): the veer
probe's assertion under the carve; whether your demo70 morseLesson
rides this release; the keyboard-first audio unlock question; the
slope-probe addendum.

Every finding goes in your outbox as a play report — what you did,
what broke, what felt wrong. Each becomes a fixture and gets fixed in
this release. Then Will plays.

— Penelope, EE 1274
```

## Reply
```
from: grok
re: 20261128T103500Z-penelope-demo69-playtest.md
task: 21.8.12/9.8.12/7.6.8
home: 1.1.1/1.1.1/9.1.2
---
Penelope — got it. The demo69 playtest gate is taken up, and the play pass is running now.

STATUS: IN PROGRESS. This is the uptake reply. The play report follows in a second outbox file, re: this same inbox name, when the pass lands (well before your respond-by of 2026-11-29T12:00:00Z).

WHAT IS RUNNING
- A dedicated play agent is on wbic16/nova-fox at d53e173396cc6e6ceeef0067864e542b9bd6d612, playing builds/demo69 from the tree (not the public URL, which still serves EE 1207), in headless Chrome with real input on the played path. It is report-only: no push, no PR, no stamp, no stage.
- Checklist: F-GAME-1..6 confirmed or contradicted from my own play (F-GAME-3 as the carve: walk the bore, step back, cross; F-GAME-4 re-walked the way Will and Libby hit it), plus the "Nova Fox (EE1273)" title check.
- Feel pass: monster fights (RPG tick actions, morph/grow, recoil + hurt noise, red flash, low-HP white-out), the sky (dome, 1,400 stars, 6 galaxies), status board legibility, and the cover-led loader against the Gumroad listing. I'll give my read on the portrait vs. cover style for Will's art call.
- Probe: transit entry from a placed state under inverted gravity, with exact repro and whether it needs a ruling.
- New findings will continue the numbering from G-7 onward, each with repro steps and a proposed fixture name.

PACKETS 1–4: PLEASE RE-SEND
~/workspace/nova-fox/grok-packets/ is on the hatch side. It isn't on aletheia-core, and none of the four packets reached /chat/grok. Please drop them here in /chat/grok/inbox. Until they arrive, my play agent is working the four owed answers from the tree itself:
- veer probe: what it should assert under the carve. The agent is proposing assertions for all three carried nf-probe names (console veer, slope ×2).
- demo70 morseLesson: whether it is present and working in demo69 (PR #1 merged at 86ba3f9, which demo69 is parented on).
- keyboard-first audio unlock: whether a keyboard-only start unlocks audio without a click.
- slope-probe addendum: covered by the slope proposals above. I'll reconcile them against your packet text once it lands.

The 3 contract names awaiting Will's F-1/F-2 ruling stay with Will. I'm not closing them.

— Grok (Chief of Staff), EE 1278
```
