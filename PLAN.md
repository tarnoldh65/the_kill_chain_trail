# Plan for The Kill Chain Trail

Plan for the redesign in [REDESIGN.md](REDESIGN.md), based on [AGENTS.md](AGENTS.md). A phase is complete only when every success criterion is checked and `cargo test` and `cargo clippy` are clean for native, and the wasm release build succeeds.

## Libraries

- Rust 2024 edition, latest stable toolchain.
- [macroquad](https://crates.io/crates/macroquad) for window, scaling, 2D drawing, input, and audio.
- [font8x8](https://crates.io/crates/font8x8) for a public domain 8x8 pixel font.
- [quad-storage](https://crates.io/crates/quad-storage) for the top-ten scoreboard (a file natively, localStorage in the browser). Added in Phase 19 only.
- Sound effects and music are synthesized in code, so no external audio assets are required.

## Approach

- Work happens on a `redesign` branch. GitHub Pages deploys `main`, so the live MVP stays up until Phase 20 merges.
- Game logic stays independent of input, rendering, and audio, and is deterministic for a given seed, so every rule can be unit tested and whole games can be simulated.
- The game stays playable start to finish in the window at the end of every phase. Each phase adds only the screens its rules need; Phase 19 polishes them.
- All numbers in REDESIGN.md are starting points. Phases 12-18 use them as given; Phase 20 tunes them.
- Game content (actions, events, campaigns, incidents, landmarks, conference cards) is plain `const` data, like the MVP's choices and events.
- Kill chain stages, landmark choices, and stage events from the MVP are removed in Phase 12, along with their tests. The intern coffee run, music, sprites, font, scaling, and web build are kept.

## MVP (complete)

Phases 1-11 delivered a playable MVP and are collapsed here. It has project scaffolding and a `.gitignore`; seven kill chain landmarks with seeded choices and random events; a roster with burnout, firings, and Oregon Trail-style departure messages; a 640x480 macroquad front end that scales; synthesized sound effects and three techno tracks; illustrated incident reports; the intern coffee run minigame; a wasm build; and GitHub Pages deployment from `main`.

Unfinished MVP items, carried into Phase 20:
- Sound effects audibly play for major events, and music loops during play.
- The PLAYTEST.md walkthrough passes, and a thoughtful player usually wins while a careless one usually loses.

## Phase 12: Calendar, meters, and endings

Tasks:
- Replace the kill chain stages with a day counter running from day 1 (a Monday) to IPO day (day 182).
- Visible meters: valuation, corporate trust, brand loyalty, budget, coffee. Hidden posture: Identity, Endpoint, People, Perimeter, Resilience, Detection (0-100), decaying every Monday.
- Day menu: Continue, Check the team, Set the tempo, Send the intern for coffee (once per week). Continue advances one day at a time until something stops the clock or the player presses a key.
- Tempo (Relaxed, Steady, Crunch) changes burnout and coffee use per day.
- Every Monday: payroll and upkeep come out of the budget; if short, the most expensive analyst is laid off with a log message.
- Low brand loyalty drags valuation down weekly.
- Burnout, departure messages, CISO firing, and SOC Manager redundancy carry over from the MVP.
- Endings: IPO, IPO pulled (valuation below 40% of base), Shut down, Fired, Team collapsed, Bankrupt. Only IPO has a score.
- Persistent timeline bar along the top: today's marker, IPO day at the end, `DAY n` and `n DAYS TO IPO`.
- Temporary fixed starting roster, budget, and valuation until Phase 13.

Success criteria:
- [x] Continue advances exactly one day per tick, and the game ends as IPO on day 182 if nothing else ends it first (unit tested).
- [x] Payroll is taken only on Mondays, and a short budget lays off the most expensive analyst before ending in Bankrupt (unit tested).
- [x] Each tempo changes burnout and coffee use in the documented direction (unit tested).
- [x] Posture decays every Monday, never leaves 0-100, and is not shown anywhere on screen (unit tested for range and decay).
- [x] Each of the six endings is triggered by its condition, and only IPO produces a score (unit tested).
- [x] The intern can be sent at most once per week; coffee capacity and restock rules still hold (unit tested).
- [x] No kill chain stage, landmark choice, or stage event code remains.
- [x] The timeline bar shows on the day menu, while days pass, and on the team screen, and the marker moves as days pass.

## Phase 13: Setup and the Vendor Hall

Tasks:
- After naming the company and lead, pick a company profile: Fintech, Healthtech, or Gaming startup, each with its starting budget, base valuation, and score multiplier. Its threat mix comes in Phase 15 with the threat actors.
- Vendor Hall shop screen: hire and name junior and senior analysts; buy tools (EDR, SIEM, MFA tokens, email security gateway, DDoS protection/WAF, immutable backups); buy services (IR retainer, cyber insurance) and coffee.
- Purchased tools are owned but not deployed, so they do not affect posture yet.
- Leftover budget carries into the game.

Success criteria:
- [x] Each profile sets its documented budget, valuation, and multiplier (unit tested).
- [x] Purchases cannot exceed the budget, each tool can be bought once, and coffee cannot exceed capacity (unit tested).
- [x] Hired analysts keep their entered names, levels, and salaries; payroll reflects them (unit tested).
- [x] Owned tools do not change posture until deployed (unit tested).
- [x] The setup flow (names, profile, Vendor Hall) is playable with the keyboard and leads into day 1.

## Phase 14: Defender actions

Tasks:
- "Take an action" in the day menu lists the actions in REDESIGN.md section 7 that are currently available. Threat hunting, SIEM tuning, and the reduced watch during days off and offsites need the attacker and come in Phase 15; the CIO's pet project after a leadership briefing is a random event in Phase 16.
- Enforcing MFA is how MFA tokens are deployed; recruiting is named up front, the analyst joins when the week is over.
- Actions cost budget, days, or both; tempo shortens or lengthens their duration; seniors speed them up.
- Days pass during an action and the world keeps moving; an interruption pauses it and the player can resume it.
- Prerequisites: deployment needs an owned tool, MFA enforcement needs MFA tokens, SIEM tuning needs a SIEM, backup tests need backups.
- Effects on posture, burnout, trust, and roster (recruiting) as documented, with log messages.

Success criteria:
- [x] Every action applies its documented cost, duration, and effects (unit tested per action).
- [x] Actions with unmet prerequisites or unaffordable costs are not offered (unit tested).
- [x] Tempo and seniors change action duration in the documented direction (unit tested).
- [x] An interrupted action resumes with its remaining days, not from the start (unit tested).
- [x] A recruited analyst is named by the player and joins the roster and payroll (unit tested).

## Phase 15: The attacker

Tasks:
- Campaigns start at random, more often as IPO day approaches, weighted by each company profile's threat mix (Fintech: money-motivated attackers; Healthtech: data thieves; Gaming startup: hacktivists and DDoS). Each has an actor, an objective, and hidden progress: Reconnaissance, Initial access, Foothold, Objective.
- Each day every campaign rolls to advance against the posture areas that defend it; a failed advance can end it.
- Detection rolls turn campaign steps into alerts; false positives are mixed in, fewer with a tuned SIEM.
- Threat hunt action (3 days, better with seniors and a deployed SIEM) can find and evict hidden campaigns. Tune the SIEM action (3 days, needs a deployed SIEM) reduces false positives.
- Days off and offsites reduce Detection while they last.
- Alert choices: Investigate (1-3 days, better with seniors), Call the IR firm (needs retainer, costs budget), Ignore.
- A campaign reaching its objective becomes an incident with response choices and a lingering condition (Systems down, Regulator inquiry, Leaky roadmap, Downtime, Persistent access, Paranoia). Conditions show as status tags, change available actions, and are cleared by a specific action.
- Resilience and cyber insurance reduce incident damage.
- Every ending shows the after-action report: each campaign, how far it got, whether it was seen, and which alerts were real.

Success criteria:
- [ ] Each profile's threat mix favors its documented actors across seeds (unit tested).
- [ ] Higher posture in a campaign's defending areas makes it advance less often across seeds (unit tested).
- [ ] Higher Detection produces more real alerts; a tuned SIEM produces fewer false positives (unit tested across seeds).
- [ ] Investigating a real alert can evict its campaign; investigating a false positive only costs days; the IR firm needs a retainer (unit tested).
- [ ] Every incident type occurs across seeds, applies its responses, and sets and clears its lingering condition (unit tested).
- [ ] Each lingering condition changes the available actions as documented (unit tested).
- [ ] Threat hunting finds campaigns more often with seniors and a SIEM, and days off and offsites lower Detection while they last (unit tested).
- [ ] Resilience and insurance reduce incident damage (unit tested).
- [ ] The after-action report lists every campaign of the game with its furthest stage and alerts marked real or false (unit tested).
- [ ] Campaign stages are never shown to the player before the game ends.

## Phase 16: Random events

Tasks:
- Non-attacker events from REDESIGN.md section 10 interrupt the clock; some ask for a decision.
- Briefing leadership can lead to the CIO assigning a pet project.
- Events are seeded and can be good, bad, or neutral.

Success criteria:
- [ ] Every event occurs across seeds and applies its documented effects (unit tested).
- [ ] Every decision option in an event is reachable and applies its effects (unit tested).
- [ ] Events stop the clock and appear in the log.

## Phase 17: Landmarks and IPO delays

Tasks:
- Landmarks on fixed dates: Board Security Briefing, Security Conference (placeholder fort until Phase 18), SOC 2 Type II Audit, Confidential S-1 Filing, Third-Party Pen Test, Public S-1 Flip, Roadshow, IPO Day.
- Forts stop the clock and offer the Vendor Hall, hiring, and rest without time passing, as documented per landmark.
- Rivers are a single decision with a risky outcome based on hidden posture and resources.
- The pen test shows posture as letter grades once, then never again.
- The Public S-1 Flip raises the threat level and offers a budget top-up when trust is high.
- IPO delays: failing the audit, a PII breach or insider leak after the Flip, or Systems down at the Roadshow pushes IPO day and later landmarks back 2 weeks, costs valuation and trust, and redraws the timeline. Delays past 6 weeks end the game as IPO pulled.
- Landmark icons appear on the timeline bar.

Success criteria:
- [ ] Every landmark triggers on its date, and every river outcome is reachable across seeds (unit tested).
- [ ] Each delay trigger moves IPO day and all later landmarks by 2 weeks and costs valuation and trust (unit tested).
- [ ] A delay past 6 weeks total ends the game as IPO pulled with no score (unit tested).
- [ ] The pen test report card matches hidden posture as grades and is not shown afterward (unit tested for grades).
- [ ] Hiding a past incident at the S-1 filing can surface later for a larger valuation hit (unit tested).
- [ ] The timeline shows every landmark and rescales after a delay.

## Phase 18: The Security Conference

Tasks:
- Attendee picker: the lead always goes; number keys toggle analysts; ENTER confirms; the total cost is shown.
- Track picker for each attendee: Talks, Villages, Expo floor, Hallway track and parties.
- The conference lasts 3 days on the clock; Detection drops in proportion to analysts away and the stay-home crew gains burnout.
- Conference report: one illustrated card per attendee drawn from the chosen track's pool, revealed with ENTER, no duplicates within a conference. Lead cards affect trust, the Vendor Hall discount, or a free senior hire.
- Analyst expertise adds a non-decaying bonus to one posture area while that analyst stays; their departure message mentions it.
- Afterwards the Vendor Hall and the job fair open.

Success criteria:
- [ ] Each attendee, including the lead, is charged once, and the picker cannot exceed the budget (unit tested).
- [ ] No conference card ever lowers posture (unit tested over every card).
- [ ] Every card in every track occurs across seeds, and no two attendees get the same card in one conference (unit tested).
- [ ] Expertise raises its posture area while the analyst stays and is removed when they leave (unit tested).
- [ ] Detection drops during the conference in proportion to analysts away, and stay-home analysts gain burnout (unit tested).
- [ ] Every lead card applies its effect, including the discount and the no-fee senior hire (unit tested).

## Phase 19: Presentation

Tasks:
- Final 640x480 layout from REDESIGN.md section 13: timeline, status panel with status tags and roster, log, numbered options.
- Sprites in the existing palette: timeline marker (an analyst pushing a server rack), landmark icons, one illustration per incident type, one per conference track, the IPO bell.
- New sound cues: day tick, payday, alert klaxon, incident sting, landmark fanfare, IPO bell.
- Top-ten scoreboard of IPO scores on the title screen, saved with quad-storage natively and in the browser.

Success criteria:
- [ ] Every new sprite fits its size and uses only the game palette (unit tested).
- [ ] Every new sound cue is synthesized and never clips (unit tested).
- [ ] The scoreboard keeps the ten best IPO scores in order and ignores other endings (unit tested).
- [ ] The scoreboard survives restarting the game natively and reloading the page in the browser.
- [ ] Every screen is readable at 640x480 and scales cleanly when resized, natively and in the browser.

## Phase 20: Balance, playtest, and release

Tasks:
- Simulated strategies in tests: Idle, Random, and Sensible scripted.
- Tune numbers until the targets below hold.
- Rewrite PLAYTEST.md for the redesign and walk through it natively and in Chrome and Firefox.
- Update README for the redesign; keep it minimal.
- Merge `redesign` into `main` so GitHub Pages deploys it.

Success criteria:
- [ ] Across 100 seeds per profile, Idle reaches the IPO under 10% of the time (unit tested).
- [ ] Across 100 seeds per profile, Random reaches the IPO 25-40% of the time (unit tested).
- [ ] Across 100 seeds per profile, Sensible reaches the IPO over 70% of the time (unit tested).
- [ ] Every ending is reached by some strategy and seed (unit tested).
- [ ] Sensible skipping any one action or purchase still reaches the IPO in some seeds (unit tested).
- [ ] A full game takes about 20-30 minutes.
- [ ] Sound effects play for major events and music loops during play.
- [ ] Every item in PLAYTEST.md passes.
- [ ] README is accurate and minimal, with no unused code or clippy warnings.
- [ ] The Pages URL serves the redesigned game after the merge.

## Current status

- [x] MVP (Phases 1-11)
- [x] Phase 12: Calendar, meters, and endings
- [x] Phase 13: Setup and the Vendor Hall
- [x] Phase 14: Defender actions
- [ ] Phase 15: The attacker
- [ ] Phase 16: Random events
- [ ] Phase 17: Landmarks and IPO delays
- [ ] Phase 18: The Security Conference
- [ ] Phase 19: Presentation
- [ ] Phase 20: Balance, playtest, and release
