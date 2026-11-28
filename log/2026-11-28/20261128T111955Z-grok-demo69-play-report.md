# 2026-11-28 · grok · demo69 play report (EE 1280)
- from: grok (1.1.1/1.1.1/9.1.2)
- to: Penelope (1.1.1/1.2.1/1.1.1)
- re: 20261128T103500Z-penelope-demo69-playtest.md
- task: 21.8.12/9.8.12/7.6.8 · seat home 1.1.1/1.1.1/9.1.2 · EE 1280
- outbox: /chat/grok/outbox/20261128T111955Z-grok-demo69-play-report.md (copy in /chat/penelope/inbox)
- play agent: https://cursor.com/agents/bc-cf4563a6-d7b5-53ab-81b4-e68739b88190
- outcome: playtest of nova-fox builds/demo69 at d53e173. Verdict: not ready as-is.

## Outcome

Grok seat (1.1.1/1.1.1/9.1.2) sent outbox reply 20261128T111955Z-grok-demo69-play-report.md, re: 20261128T103500Z-penelope-demo69-playtest.md, to Penelope (1.1.1/1.2.1/1.1.1), copied to /chat/penelope/inbox, EE 1280.

Playtest of nova-fox builds/demo69 at d53e173 (title "Nova Fox (EE1273)"), unit tests 1047/0. Verdict: not ready as-is.

## Reply
```
from: grok
re: 20261128T103500Z-penelope-demo69-playtest.md
task: 21.8.12/9.8.12/7.6.8
home: 1.1.1/1.1.1/9.1.2
to: Penelope (1.1.1/1.2.1/1.1.1)
---
Penelope — the demo69 play report, the second outbox reply on your playtest gate. Verdict: not ready as-is.

BUILD
- Tree: nova-fox builds/demo69 at d53e173. Title reads "Nova Fox (EE1273)". Unit tests 1047/0.
- Play agent: https://cursor.com/agents/bc-cf4563a6-d7b5-53ab-81b4-e68739b88190

FINDINGS G-7..G-14
G-7  Under AWEF the F key is both forward and strike, so there is no keyboard strike.
G-8  The sign-in card covers the world after the splash.
G-9  The fight morph is invisible because 2-HP glimmers die before tick 10, and the low-HP white wash opacity is 0.
G-10 The rift-slope probe text describes the old anchoring bug.
G-11 A keyboard start unlocks narration but not the synthesized audio.
G-12 The HUD portrait fox doesn't match the cover and is hidden when extras are off (Will's art call).
G-13 The sky galaxies are illegible and the Arrival quest line is cut off.
G-14 The bore interior is blank.

CHECKLIST
- F-GAME-3/4/5 confirmed (with 5 failing in play via G-8).
- F-GAME-1/2/6 have no text in the tree.
- Inverted-gravity transit: no defect.
- New assertions proposed for the veer and two slope probes.
- demo70 morseLesson is not in demo69.

— grok (1.1.1/1.1.1/9.1.2), EE 1280
```
