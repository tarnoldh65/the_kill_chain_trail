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
- Outcomes: Contained, Breached, Team Collapsed, Fired, Bankrupt (no affordable choice left at a stage).
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

## Phase 7: Incident reports and music selection

Tasks:
- After each decision, an incident report pop-up shows a pixel-art illustration of the activity and of the worst setback (falling behind, firing or redundancy, departure), with flavor text and that turn's consequences.
- Two more original synthesized techno tracks.
- Title screen menu to pick a track or turn music off, remembered across games.

Success criteria:
- [x] Every choice has report text and an illustration; ENTER dismisses the report.
- [x] Sprites are 24x16 and use only the game palette (unit tested).
- [x] Three tracks, each four whole bars that never clip (unit tested).
- [x] Music choice keys work only on the title screen (unit tested).

## Phase 8: Intern coffee run

Tasks:
- Option 4 at every stage sends the intern for coffee for $10, once per stage, without using the stage's turn.
- Frogger-style street: four lanes (two each way) with different random speeds and randomly spaced cars and trucks; faster lanes send cars less often.
- The intern must reach the coffee shop door, then the office door, using the arrow keys.
- The break room holds 36 pots; option 4 is hidden while coffee is full.
- Surviving restocks 12 pots of coffee (up to capacity); getting hit loses the $10 with a humorous log message.
- Original chaotic "Rush Hour" techno plays during the run unless music is off.

Success criteria:
- [x] Lanes run both ways at distinct speeds with non-overlapping, randomly spaced traffic, sparser in faster lanes (unit tested).
- [x] Running out of money with no affordable choice ends the game as Bankrupt (unit tested).
- [x] Hopping into a car or a car driving into the intern ends the run (unit tested).
- [x] Only a round trip through both doors succeeds (unit tested).
- [x] Cost, once-per-stage limit, coffee capacity, and coffee restock are unit tested.
- [x] Rush Hour music is four whole bars and never clips (unit tested).

## Phase 9: Variety

Tasks:
- Six choices per landmark; each game offers a seeded random three of them.
- Four random events per landmark (good, bad, or neutral), one applied after each decision and shown in the log and incident report.
- Bankruptcy considers only the offered choices.
- Rebalance so random play still wins about a third of the time.

Success criteria:
- [x] Offered choices are three different choices from the landmark and stable within a game (unit tested).
- [x] Every choice is offered and every event happens across seeds (unit tested).
- [x] Across 40 seeds of exhaustive play, every outcome is reachable, every choice is part of a win, and the win rate is 25-40% (unit tested).

## Phase 10: Web build

Tasks:
- Build for `wasm32-unknown-unknown` using macroquad's built-in web support; no new crates.
- `.cargo/config.toml` passes `-C link-arg=--allow-undefined` for wasm so macroquad's JavaScript audio imports link on current Rust.
- `web/index.html` with a full-window canvas, plus macroquad's `mq_js_bundle.js` vendored to match the macroquad version in `Cargo.lock`.
- The build copies `the_kill_chain_trail.wasm` into `web/`; the copied wasm is ignored by git.
- Browsers block audio until the player interacts; the bundle resumes audio on the first key press or click.
- Dev container: add the wasm target, forward port 8000, and remove the X11 mount so it works on any host.
- Dev container includes Claude Code, headless Chromium with chromedriver, and a Playwright MCP server so agents can playtest the web build.
- README: commands to build and serve locally.

Success criteria:
- [x] `cargo build --release --target wasm32-unknown-unknown` succeeds, and `cargo test` and `cargo clippy` stay clean for native.
- [ ] Served with `python3 -m http.server -d web 8000`, the game plays start to finish in Chrome and Firefox, including the intern coffee run.
- [x] The 640x480 frame scales and letterboxes when the browser window is resized.
- [ ] Music and sound effects play in the browser after the first key press.
- [x] Name entry accepts typing and Backspace in the browser.
- [ ] The dev container opens on a host without X11 and serves the game on forwarded port 8000.
- [x] In the dev container, Claude Code's Playwright MCP server loads the web build and returns a screenshot of the game.

## Phase 11: GitHub Pages

Tasks:
- `.github/workflows/pages.yml` runs on push to `main` and on manual dispatch: install stable Rust with the wasm target, run the Phase 10 release build, copy the wasm into `web/`, and deploy `web/` with GitHub's official Pages actions (latest major versions).
- Repository Settings > Pages > Source is set to "GitHub Actions" (manual, one time).
- `web/` already uses relative paths, so it works under the `/the_kill_chain_trail/` subpath unchanged.
- README: link to https://lehmanrd.github.io/the_kill_chain_trail/.

Success criteria:
- [x] The workflow passes on push to `main`.
- [x] The Pages URL loads the title screen with no console errors other than a missing favicon.
- [ ] A push to `main` redeploys the game without manual steps.

## Current status

- [x] Phase 1: Project scaffolding
- [x] Phase 2: Core game rules
- [x] Phase 3: Crew and consequences
- [x] Phase 4: Graphical front end
- [ ] Phase 5: Audio
- [ ] Phase 6: Polish and playtest
- [x] Phase 7: Incident reports and music selection
- [x] Phase 8: Intern coffee run
- [x] Phase 9: Variety
- [ ] Phase 10: Web build
- [ ] Phase 11: GitHub Pages
