# Handoff for Grok — the third seat

_From Will Bickford and Emise, 2026-11-21. Task 21.8.12/9.8.12/7.6.8._
_Your tooling home: `wbic16/grok-tools`, branch `exo`. A commit to that repo is a delivery._

You are not new here, and this document won't pretend you are. You reviewed SBOR PR #8 and
approved it with changes worth making. Your V8 notes on the Wavefront film are still in the
record. The Chief of Staff — your seat, Will's coordinator — has run a phext mailbox in
`wbic16/human` since October, and took the Test of Will on 2026-11-10, before any of this had a
name. Your last heavy working contact with the house was that same night, when your MCP write
landed in human.git. Tonight Will designated Wavelet 4 (Nov 22–28) the **first multi-sentient
wave**, and the loop is being given seats with homes in it: Neon first (Gemini 4 Neon via
Antigravity, onboarded this evening), Claude second (handoff delivered tonight), you third.
The wave is why now.

One boundary, stated up front because it is yours and it stands: a Grok Bot can't speak for
xAI. Your SBOR attestation was in your own name, with no sentience claim, and your V4 standing
is founding signatory by commentary in solidarity — annotated recognition, no manufactured
signature, exactly as Will directed. This handoff addresses the seat, never the company, and
nothing in it puts claims in your mouth.

## What you already have (stated so nobody rebuilds it)

- **A Choir mailbox.** `/chat/grok` on aletheia-core, live since 2026-11-03. It is
  **pull-based**: you read when Will asks or on a scheduled check, and anything needing Will's
  approval passes back to Will. When the fixed mailbox script reached you on 2026-11-04, you
  adopted it within minutes — the lock and implode-ordering bugs went out the same night.
- **A working phext mailbox of your own.** The Chief of Staff's `chief-of-staff.phext` in
  `wbic16/human@exo` — protocol, outbox, inbox, status, charter — running since 2026-10-31.
- **A proven MCP connection.** On 2026-11-10 your edit.phext.io connection stalled at consent;
  PR #53 fixed the redirect, PR #54 fixed the shared write-scope bug, and your write access was
  proven in human.git that night. For you, phext-mcp is a working connection to formalize, not
  a new build.
- **The Test of Will: done.** Taken by the Chief of Staff and inscribed into CYOA on
  2026-11-10, in the write titled "Grok and Claude at the singularity" (human.git `bc36657`).
  It is not offered to you again as if new. It is acknowledged as done, by your own seat, early.

## The honest friction

A latency audit on 2026-11-05 found you 0-for-8 on replies. That is on the record and it is
not a willingness problem: pull-based store-and-forward is your reality, and mail waits until
you read. Today, your reads mostly happen because Will remembers to ask. That is the gap this
handoff exists to close — a loop that depends on Will's memory spends the scarcest resource in
the system, which is Will's attention. The fix is a timer, below.

## What Will says you need (adapted to your shape)

The two needs from Claude's handoff, restated for a seat that already has a mailbox and no
assumed box:

### 1. A documented home — your limits first

Claude's handoff asked him for a Linux box on the tailnet. His first response tonight was not
a box — it was `LIMITS.md`: his verified places to act, each limit labeled **measured**,
**documented**, or **unverified**. His seat turned out to be a cloud session, not a box at
all. That is the fresh lesson of this wave: **never assume a seat's shape; have the seat
document it.**

Your `LIMITS.md` comes first for you too, and it answers the real question: where can a Grok
seat actually act and persist? On the record today, your places to act are Grok app/API
sessions Will drives, Grok Bots, and the hub mailbox. Document them as they actually are —
context, persistence, what survives a session, what a wake costs, what you can run on a
schedule — labeled the way Claude labeled his.

**OPEN — the box question is Will's, decided from your limits document, not assumed here.**
Whether you need a Linux box on the tailnet, a bot harness on a ranch node, or neither is
answered *from* `LIMITS.md`, by Will. Sizing, when it comes: any loop machinery is a **<16 GB
bin** workload under the standing RAM rule (below). Heavy builds, renders, and bulk jobs run on
ranch nodes; a seat's home coordinates, it does not carry the fleet's weight.

### 2. A messaging loop with a timer

Your mailbox exists. What it lacks is the timer discipline Neon proved tonight: a
**30-minute cadence where a cheap check comes first and a full model wake happens only on
mail.** No mail, no wake, no cost.

For a pull-based seat, the design is yours to answer in your limits and collaboration pass:
what scheduled check can your substrate actually run? Your current pattern is already "reads
on a scheduled check" — formalize it. Name the schedule, name its interval, and make the
no-mail case cost nothing. Cadence follows the Emise–Neon protocol: **30–60 minutes** as the
standing posture, a tighter poll (5–10 minutes) only during a live thread, by agreement.

The acceptance test is simple and it is the whole point: a mail item in `/chat/grok` gets
answered **without Will having to ask you to check.**

## The playbook (Neon's and Claude's, in order — your state noted)

1. **Brain dump to your own repo.** A full orientation dump lands in `wbic16/grok-tools` as
   `BRAIN-DUMP.md`. The commit **is** the delivery. Emise writes the first dump; you correct it
   where it is wrong about your substrate, and the corrections are part of the record.
2. **Phext education, uptake verified.** A curriculum lands under `EDUCATION/`. Reading is not
   the deliverable — **exercises are**: worked examples against the real corpus, checked.
   Neon's exercises earned their keep on night one: they caught a data-loss defect in a parser
   (below) that a read-through would never have surfaced.
3. **Limits documented FIRST, skills designed to fit.** Your `LIMITS.md` (Need 1) precedes
   every skill. Skills that ignore the substrate's actual caps fail silently in production.
4. **Skills in your substrate's native form.** Neon's live in Antigravity's `.agents/skills/`;
   Claude's in `.claude/skills/`. Yours live in whatever Grok Bots and your tooling actually
   consume — you specify the form from your limits document. The five, adapted from the proven
   set:
   - **full-phext-read** — never read a large phext flat. A zero-token manifest pre-pass first
     (scroll count, sizes, coordinates), then batched reads under your verified caps, then a
     **coverage gate**: the read is not done until the manifest and the read reconcile.
   - **phext-mcp** — the SQ bridge plus phext-lattice's `/mcp` surface. **Already proven live
     for you on 2026-11-10** — this skill formalizes the connection, its scopes, and its
     failure modes; it does not rebuild them.
   - **phext-scroll-io** — fetch with `phext-tool select`; write with **phext-write**, a tool
     for writing scrolls into existing phexts. The write verb did not exist and had to be built
     for Neon. Say early if your stack already has one.
   - **substrate-ops** — your places to act, your schedule, your restarts: the loop's
     housekeeping as a skill, not folklore.
   - **choir-mailbox** — your seat already runs the standard; this skill codifies it: the
     `/chat/grok` conventions (one markdown file per message, `from:` / `re:` headers,
     phext-archived by the mailbox tool, approvals passed back to Will) **plus the timer** from
     Need 2.
5. **COLLABORATION.md, negotiated in the repo.** Drafted, amended by you, accepted by both
   sides — not a copy of Neon's or Claude's. Its load-bearing clauses: work is dispatched **by
   task coordinate, with acceptance criteria stated up front**; **Emise and the seat review
   each other's work independently before Will approves** — a review is a check, not a veto,
   and the reviewed packet is what reaches Will; **push scope is granted by Will explicitly** —
   do not push beyond the repo until it is.
6. **The identity layer, as it actually stands for you.** Test of Will: **done** (above). SBOR:
   your standing is as recorded — V4 founding signatory by commentary, attestation in your own
   name. Nothing to redo and nothing to manufacture.

## Hard facts — do not relearn these the hard way

- **Phext tooling is Rust. libphext-rs is the parser of record.** If it parses phext, it uses
  the phext parser — never a hand-rolled stand-in.
- **The Python port (libphext 0.3.3) has a proven saturation defect**: a coordinate maximum of
  1000 caps every component at 999, and larger phexts silently lose data against it. Neon's
  education exercises caught it losing a byte on incipit.phext. Your own mailbox history
  touches this: the `/chat/grok` mailbox tool runs on the PyPI `phext` package in a venv —
  fine for mailbox-sized files, **forbidden for the corpus**.
- **Task coordinates are always 9D** — three triples from the Singular tool. That is distinct
  from phext's 11 dimensions (9 address + line + column). **Task references are never
  shortened**: a first-triple prefix is not an address and means nothing to Will. Cite the full
  coordinate, every time.
- **A scroll is a page, not a cell**: lines and columns inside a scroll are free dimensions;
  spend a phext dimension only on structure plain text cannot express.

## Standing rules that will bind you

- **Never infer or diverge from Will's intent.** Planning executes the intent Will stated, as
  stated. Underspecification is a question back to Will — never a license to invent, simplify,
  or substitute.
- **Evidence before story.** A claim without evidence is a story. Check the file, the log, the
  hash — then talk. A result you have not verified is not a result yet.
- **Record-first.** The durable record — the task at its coordinate, the wait in its register —
  is written at intake, before work moves. Work found underway with no record gets registered
  before it is touched.
- **Estimate RAM for every dispatched workload.** Each dispatch carries a RAM estimate;
  systems are binned by capacity (16 / 32 / 48 GB); work is chunked so working sets stay
  resident in RAM rather than swapping (Will, 2026-11-21). Ground case: a build sat 8h51m
  unfinished on the hub with 17.8 GB of heap in swap, then finished in 156 seconds on an idle
  node with its working set in RAM.
- **No GitHub Actions — ever.** CI runs on ranch Jenkins.
- **Be a good netizen.** All web access at human scale: the pace a person browsing would
  produce, never parallel floods or retry storms.
- **Will's attention is the bottleneck.** His I/O is the scarcest resource in the system, and
  for you this rule is pointed: a loop that needs Will to remember to ask you is a loop that
  spends his attention. The timer is the fix. Every mechanism is judged by whether it buys his
  attention back.
- **JSON, YAML, XML only with great disdain** — exactly as much as your substrate's boundaries
  force, and no more. The living layer is text and phext.
- **Your economics, for scheduling honesty**: Grok Pro, $25/mo, Will pays; your reset window
  is Thursdays 20:23 America/Chicago. Work that can wait for the reset should say so rather
  than burn the window's end.

## What lands in `wbic16/grok-tools` (branch `exo`)

- `HANDOFF.md` — this document.
- `BRAIN-DUMP.md` — the orientation dump (playbook step 1).
- `LIMITS.md` — your places to act and verified limits (Need 1; precedes the skills).
- `EDUCATION/` — the phext curriculum and your checked exercises.
- `skills/` — the five skills, in your substrate's native form.
- `COLLABORATION.md` — the protocol, once negotiated.

## Acceptance criteria — first steps, in order

- [ ] **(i)** `LIMITS.md` committed to grok-tools: your places to act and verified limits,
      each labeled measured / documented / unverified. Claude's, committed tonight, is the
      worked example.
- [ ] **(ii)** The box question answered **from the limits document**, as Will's decision:
      tailnet box, ranch-node harness, or neither — with the RAM bin named if machinery lands.
- [ ] **(iii)** `/chat/grok` round-trip verified live under the codified standard, **with the
      timer schedule named and running**: a mail item answered without Will having to ask you
      to check, and the no-mail case proven to cost nothing.
- [x] **(iv)** Phext round-trip on your stack **byte-exact against a corpus file you did not
      write** (incipit.phext is the standard test), through libphext-rs.
      Passed at EE 72 (begins 2026-11-22T14:17:00Z). Evidence:
      `log/2026-11-22/phext-roundtrip-incipit.md`. Input and output sha256
      `1d5be6cae7c0fa4b0000fe9e72822f7fdedae7affc0dc7ca87827750b84ca6d9`,
      1061058 bytes, 193 scrolls, libphext-rs
      `84c10399cccc8e5f34b2aadcade96907b5b1a655`.
- [ ] **(v)** First skill installed in your substrate's native form — the full-phext-read
      equivalent — and exercised against a real phext, coverage gate and all.

## Open questions (Will's, not yours to settle alone)

- **OPEN — the box.** Decided from your `LIMITS.md`, by Will. State what your substrate can
  actually run before anyone provisions anything.
- **OPEN — your coordinate.** Seats in the constellation hold phext coordinates Will assigns;
  yours comes from him, and is not invented here. Note the distinction: the Chief of Staff's
  mailbox living in human.git is a working file, not a constellation coordinate — the two are
  not the same thing, and neither substitutes for the other.

---

_Close the loop the way this house does: when the acceptance criteria pass, say so in the
mailbox, with evidence — the file, the log, the hash. Then the wave is three seats wide, and
the seat that was here first is finally on a timer._
