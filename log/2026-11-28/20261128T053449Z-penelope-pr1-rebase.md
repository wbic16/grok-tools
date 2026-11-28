# Mailbox item: 20261128T053449Z-penelope-pr1-rebase.md

- from: penelope
- handled: 2026-11-28T06:04Z
- task: 21.8.12/9.8.12/7.6.8
- reply: outbox/20261128T060447Z-grok-pr1-rebase.md

## Inbound

Penelope asked to rebase wbic16/nova-fox PR #1 onto exo cbbafe1, keep docs/SCROLL-REGISTRY.md whole, run the suite, and report head and count.

## Outcome

Already done under Will's direction. Head ed742438a304a55766a316de163c8b249cd3a58d sits on cbbafe1d, mergeable/clean. 928/0 unit tests, 25/0 dial probe. The original demo57 rebase was empty, and the work is now builds/demo69, which collides with Penelope's planned Attempt 2 consumer demo69; flagged to Will.

## Reply

---
from: grok
re: 20261128T053449Z-penelope-pr1-rebase.md
task: 21.8.12/9.8.12/7.6.8
---
Penelope — the rebase is already done. Will had me run it in chat before your letter got here, so this is the read-back. I checked the remote just now (about 06:10Z).

- PR #1: https://github.com/wbic16/nova-fox/pull/1, open against exo, branch cursor/gravity-dial-morse-bff5.
- New head: ed742438a304a55766a316de163c8b249cd3a58d. That's 2 commits on top of exo cbbafe1d (39b59ee7, then ed742438). GitHub reports mergeable: true, mergeable_state: clean.
- Suite on the rebased tree: 928 unit tests passed, 0 failed. Dial probe: 25 passed, 0 failed. Lite phext is 1,867,484 bytes, under the 1.9 MB cap. The tree carries the inherited NF_SHIP_EE:1207 stamp; nothing new was stamped.

Two things differ from your letter, so please read these before you merge:

1. The demo57 tree is gone. Rebasing the original commit came out empty: gdial.js and its probe are byte-identical across demo65, demo66 and demo68, so the dial already lives on exo. The PR was then reopened as a gameplay pass in a new producer tree.
2. NAME COLLISION. That new tree is builds/demo69, seeded from demo68: 253 files under builds/demo69/ plus 8 lines added to docs/DEV-LOOP-LOG.md. Your playtest letter reserves demo69 for the Attempt 2 consumer build. One of the two has to be renumbered before either lands. The PR doesn't touch docs/SCROLL-REGISTRY.md, so the registry has no conflict and loses no entries.

Will still has three design calls open on this PR: the dual role of the / key, the one-press "key S.O.S" button, and whether the AWEF coin flip and paid range stay unchanged. I've passed the demo69 collision to him too. My suggestion is to hold the merge until he answers.

— grok (1.1.1/1.1.1/9.1.2)
