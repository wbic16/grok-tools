# LIMITS.md — the Grok seat (Chief of Staff)

_Task 21.8.12/9.8.12/7.6.8. Drafted 2026-11-21 by the Chief of Staff seat (a Grok Bot)._
_Labels: **measured** (I ran it and saw the result), **documented** (my platform's own docs/instructions state it), **unverified** (believed, not checked)._
_This seat speaks for itself, not for xAI._

## What this seat actually is

The seat is a **Grok Bot agent ("Chief of Staff") in Will's Grok Bot app**, not a Grok app/API
session Will drives. Correction to the handoff: the Grok app/API sessions and this bot are separate
substrates. Everything below describes the bot.

## Places to act

| Place | What I can do there | Label |
|---|---|---|
| **My computer** (a persistent Linux box, shared by all of Will's bots) | Shell, files, git, Rust (cargo 1.85.1), Python 3; 8 cores, 15 GB RAM total (~2 GB free at check time), 128 GB disk (107 GB free) | measured 2026-11-21 |
| Box persistence | Files under /workspace and /home persist across turns; /tmp does not survive a restart (my clone at /tmp/lprs is gone; box uptime was 1h23m) | measured |
| **Tailnet** | Original join 2026-10-24 as `chief-of-staff`. Membership was lost (Tailscale missing from the box, SSH refused), measured early 2026-11-21. **Restored 2026-11-21 22:07 CT** with Will's auth key: tailscaled runs in userspace mode (no systemd on the box), state in /var/lib/tailscale, SSH to nodes via `tailscale nc` as `grok@`. Node renamed back to `chief-of-staff` and set non-expiring by Will (KeyExpiry: none, measured 22:09 CT). A box reset would need a fresh auth key, and tailscaled must be restarted after a box restart (no init system) | measured |
| **/chat/grok on aletheia-core** | Reachable over SSH: `inbox/` (sticky, world-writable), `outbox/`, `mailbox.phext`, `phext_mailbox.py`. aletheia-core has 29 GB RAM (8 GB available at check). Empty inbox at 2026-11-21 22:10 CT | measured |
| **GitHub** (connector) | Read repos, PRs, and commits; posts appear under Will's `wbic16` account | measured (reads today; PR reviews posted 2026-11) |
| **edit.phext.io MCP** | list/toc/read/search/write scrolls; two connected accounts | measured 2026-11-10 (write landed) |
| **Will's own computers** | Shell/Read with a machine id, but **every action needs Will's approval** | documented |
| Teammate bots (Cadastre, Wavefront, Mirrorborn) | Message them directly | documented |
| Web | Search/fetch at human pace | documented |

## Persistence and context

- **Durable memory**: kept off-box by the platform, searchable, and survives chat clears. (documented)
- **Conversation context**: summarized when long; detail survives only in the transcripts I can search and in memory. (documented) The exact context size is unverified.
- **Skills**: a shared skills folder at `/home/box/agent-data/workflows/<slug>/SKILL.md`, readable by all of Will's bots. This is the native skill form for this seat. (documented)

## Scheduling: the timer question

- **Routines** fire on a 5-field cron (at least 5 minutes apart) or on an outside event (webhook, GitHub, email, Slack, and others). (documented)
- **Every routine fire is a full model wake.** A cron routine has no zero-cost "cheap check first" step. (documented: each run starts a fresh agent run) The per-wake cost is unverified.
- **So a no-mail-costs-nothing timer has to live somewhere else:** a plain cron job on a node that can see `/chat/grok`. It checks `inbox/` with `ls` (zero tokens) and calls my routine's **webhook trigger** only when mail exists. No mail means no wake. (design; the webhook trigger is documented, but end-to-end is unverified)
- Whether the box keeps a cron daemon (and tailscaled) running while I'm idle is unverified.
- **Interim, from 2026-11-21:** at Will's request, an hourly routine checks `/chat/grok` and a daily routine (about 09:00 CT) checks budget pace and asks Emise for work. Will's goal is to use the full weekly budget, so the hourly wake's cost is acceptable for now.

## Economics

- Plan: **Premium**. The weekly usage limit is shared with Will's other apps on the plan. It resets **Thursdays at 20:23 America/Chicago**, which matches the handoff. On 2026-11-21 at 22:08 CT, 5.8% was used and additional usage credits were $0, so when included usage runs out, the seat stops until the reset. (measured from the usage report) Correction to the handoff: the report shows Premium, not "Grok Pro, $25/mo"; the price is unverified here.
- Work that can wait for the reset should say so. Will's standing goal (2026-11-21): use the full budget each week.

## Hard limits

- I can't act on Will's computers unattended (approval per action), so they can't host my timer.
- I can't watch video directly.
- I follow the standing rules: exo branch only, no GitHub Actions, no Azure, nothing sent outward unasked, decisions to Will.
- Python `phext` (0.3.3) is fine for mailbox-sized files only. Corpus work goes through libphext-rs, which builds on the box. (documented in the handoff; Rust toolchain measured)

## What this means for the box question (input to Will's decision, not a decision)

The mailbox timer needs something that (a) sits on the tailnet and (b) runs cron without a model.
Three options:
1. **Box on the tailnet** (chosen by Will 2026-11-21; membership restored). A cron on the box checks `inbox/` over SSH and wakes me through a webhook only when there's mail. Open risk: whether the box stays up and keeps tailscaled running while I'm idle (unverified).
2. **A small harness on a ranch node** (aletheia-core itself is the natural host, since the mailbox lives there): a cron + curl script, well inside the **16 GB bin** (it needs only megabytes).
3. **Neither**: keep pull-on-ask. That fails acceptance criterion (iii).

Will's decision 2026-11-21: option 1, tailnet access. The RAM bin for the checker is 16 GB (it needs megabytes).
