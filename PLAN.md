# Plan for The Kill Chain Trail

MVP plan based on [AGENTS.md](AGENTS.md). A phase is complete only when every success criterion is checked and `cargo test` and `cargo clippy` are clean.

## Libraries

- Rust 2024 edition, latest stable toolchain.
- [macroquad](https://crates.io/crates/macroquad) for window, scaling, 2D drawing, input, and audio.
- [font8x8](https://crates.io/crates/font8x8) for a public domain 8x8 pixel font.
- Sound effects and music are synthesized in code (square/triangle/noise waves) so no external audio assets are required.

## Phase 1: Project scaffolding

Tasks:
- Cargo project, `.gitignore`, minimal README.

Success criteria:
- [x] `cargo build`, `cargo test`, and `cargo clippy` run clean.
- [x] `.gitignore` covers build output and editor noise without duplicates.
- [x] Cargo edition is 2024.
- [x] README describes only what exists.

## Phase 2: Core game rules

Tasks:
- Seven kill chain landmarks played in order: Reconnaissance, Weaponization, Delivery, Exploitation, Installation, Command and Control (C2), Actions on Objectives.
- Each landmark offers its own choices trading budget, coffee, fatigue, containment, trust, and brand.
- Budget limits which choices are available; coffee is consumed each stage and running out hurts the team.
- Falling behind the attacker each stage costs corporate trust and brand loyalty.
- Outcomes: Contained, Breached, Team Collapsed, Fired.
- Game logic is independent of input and output so it can be driven by the terminal now and the graphical front end later.

Success criteria:
- [x] A decision is made at all seven landmarks, including Actions on Objectives.
- [x] Each landmark has distinct choices.
- [x] Budget, coffee, trust, brand, and containment all affect the outcome.
- [x] A test exhaustively plays every choice path and proves all four outcomes are reachable.
- [x] Invalid input re-prompts instead of silently picking a choice.

## Phase 3: Crew and consequences

Tasks:
- Roster of analysts, a manager, a CISO, and a CIO, each with individual burnout.
- Members who reach full burnout leave in humorous ways (quitting, heart attack, stroke, dysentery); low trust gets the CISO fired; low budget makes the Manager redundant.
- Losing analysts reduces containment progress; losing all analysts collapses the team.
- Narrative log of decisions, departures, firings, and warnings shown to the player each turn.

Success criteria:
- [x] The player enters a company name and incident lead name at startup.
- [x] Roster and burnout are shown each turn and affect gameplay.
- [x] Burnout departures, firings, and redundancies produce Oregon Trail-style messages.
- [x] Unit tests cover departures, firing, coffee shortage, and each outcome trigger.

## Phase 4: Graphical front end

Tasks:
- macroquad window at 640x480 logical resolution, scaled with integer steps to larger windows.
- Limited 8-bit palette and pixel font.
- Screens: title ("The Kill Chain Trail"), name entry, landmark decision, status panel with roster, event log, end summary.
- Remove the terminal front end once the graphical one covers the same flow.

Success criteria:
- [x] Opening screen displays "The Kill Chain Trail".
- [x] The full game is playable start to finish in the window with keyboard input.
- [x] Layout is readable at 640x480 and scales cleanly when resized.

## Phase 5: Audio

Tasks:
- Synthesized 8-bit sound effects for choices, departures, and outcomes.
- Original synthesized cyber-techno music loop.

Success criteria:
- [ ] Sound effects play for major events.
- [ ] Background music loops during play.
- [x] No external or proprietary audio assets are used.

## Phase 6: Polish and playtest

Tasks:
- Tune balance so a thoughtful player usually wins and a careless one usually loses.
- Walk through [PLAYTEST.md](PLAYTEST.md).
- Remove unused code; finalize README.

Success criteria:
- [ ] Every item in PLAYTEST.md passes.
- [x] No unused code or clippy warnings.
- [x] README is accurate and minimal.

## Current status

- [x] Phase 1: Project scaffolding
- [x] Phase 2: Core game rules
- [x] Phase 3: Crew and consequences
- [x] Phase 4: Graphical front end
- [ ] Phase 5: Audio
- [ ] Phase 6: Polish and playtest
