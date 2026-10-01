# Plan for The Kill Chain Trail

Plan for the redesign in [REDESIGN.md](REDESIGN.md), based on [AGENTS.md](AGENTS.md). A phase is complete only when every success criterion is checked and `cargo test` and `cargo clippy` are clean for native, and the wasm release build succeeds.

## Libraries

- Rust 2024 edition, latest stable toolchain.
- [macroquad](https://crates.io/crates/macroquad) for window, scaling, 2D drawing, input, and audio.
- [font8x8](https://crates.io/crates/font8x8) for a public domain 8x8 pixel font.
- The top-ten scoreboard is saved without a library: a plain file natively, and a small `web/top-ten.js` plugin for localStorage in the browser. (quad-storage was planned, but its JavaScript depends on sapp_jsutils helpers that the current macroquad bundle keeps private.)
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
- Alert choices: Investigate (1-3 days, better with seniors; runs in the background alongside any action, one at a time), Call the IR firm (needs retainer, costs budget), Ignore.
- A campaign reaching its objective becomes an incident with response choices and a lingering condition (Systems down, Regulator inquiry, Leaky roadmap, Downtime, Persistent access, Paranoia). Conditions show as status tags, change available actions, and are cleared by a specific action.
- Resilience and cyber insurance reduce incident damage.
- Every ending shows the after-action report: each campaign, how far it got, whether it was seen, and which alerts were real.

Success criteria:
- [x] Each profile's threat mix favors its documented actors across seeds (unit tested).
- [x] Higher posture in a campaign's defending areas makes it advance less often across seeds (unit tested).
- [x] Higher Detection produces more real alerts; a tuned SIEM produces fewer false positives (unit tested across seeds).
- [x] Investigating a real alert can evict its campaign; investigating a false positive only costs days; the IR firm needs a retainer (unit tested).
- [x] Every incident type occurs across seeds, applies its responses, and sets and clears its lingering condition (unit tested).
- [x] Each lingering condition changes the available actions as documented (unit tested).
- [x] Threat hunting finds campaigns more often with seniors and a SIEM, and days off and offsites lower Detection while they last (unit tested).
- [x] Resilience and insurance reduce incident damage (unit tested).
- [x] The after-action report lists every campaign of the game with its furthest stage and alerts marked real or false (unit tested).
- [x] Campaign stages are never shown to the player before the game ends.

## Phase 16: Random events

Tasks:
- Non-attacker events from REDESIGN.md section 10 interrupt the clock; some ask for a decision.
- Briefing leadership can lead to the CIO assigning a pet project.
- Events are seeded and can be good, bad, or neutral.

Success criteria:
- [x] Every event occurs across seeds and applies its documented effects (unit tested).
- [x] Every decision option in an event is reachable and applies its effects (unit tested).
- [x] Events stop the clock and appear in the log.

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
- [x] Every landmark triggers on its date, and every river outcome is reachable across seeds (unit tested).
- [x] Each delay trigger moves IPO day and all later landmarks by 2 weeks and costs valuation and trust (unit tested).
- [x] A delay past 6 weeks total ends the game as IPO pulled with no score (unit tested).
- [x] The pen test report card matches hidden posture as grades and is not shown afterward (unit tested for grades).
- [x] Hiding a past incident at the S-1 filing can surface later for a larger valuation hit (unit tested).
- [x] The timeline shows every landmark and rescales after a delay.

## Phase 18: The Security Conference

Tasks:
- Attendee picker: the lead always goes; number keys toggle analysts; ENTER confirms; the total cost is shown.
- Track picker for each attendee: Talks, Villages, Expo floor, Hallway track and parties.
- The conference lasts 3 days on the clock; Detection drops in proportion to analysts away and the stay-home crew gains burnout.
- Conference report: one illustrated card per attendee drawn from the chosen track's pool, revealed with ENTER, no duplicates within a conference. Lead cards affect trust, the Vendor Hall discount, or a free senior hire.
- Analyst expertise adds a non-decaying bonus to one posture area while that analyst stays; their departure message mentions it.
- Afterwards the Vendor Hall and the job fair open.

Success criteria:
- [x] Each attendee, including the lead, is charged once, and the picker cannot exceed the budget (unit tested).
- [x] No conference card ever lowers posture (unit tested over every card).
- [x] Every card in every track occurs across seeds, and no two attendees get the same card in one conference (unit tested).
- [x] Expertise raises its posture area while the analyst stays and is removed when they leave (unit tested).
- [x] Detection drops during the conference in proportion to analysts away, and stay-home analysts gain burnout (unit tested).
- [x] Every lead card applies its effect, including the discount and the no-fee senior hire (unit tested).

## Phase 19: Presentation

Tasks:
- Final 640x480 layout from REDESIGN.md section 13: timeline, status panel with status tags and roster, log, numbered options.
- Sprites in the existing palette: timeline marker (an analyst pushing a server rack), landmark icons, one illustration per incident type, one per conference track, the IPO bell.
- New sound cues: day tick, payday, alert klaxon, incident sting, landmark fanfare, IPO bell.
- Top-ten scoreboard of IPO scores on the title screen, saved to `top_ten.txt` natively and to localStorage through `web/top-ten.js` in the browser.

Success criteria:
- [x] Every new sprite fits its size and uses only the game palette (unit tested).
- [x] Every new sound cue is synthesized and never clips (unit tested).
- [x] The scoreboard keeps the ten best IPO scores in order and ignores other endings (unit tested).
- [x] The scoreboard survives restarting the game natively and reloading the page in the browser.
- [x] Every screen is readable at 640x480 and scales cleanly when resized, natively and in the browser.

## Phase 20: Balance, playtest, and release

Tasks:
- Simulated strategies in tests: Idle, Random, and Sensible scripted.
- Tune numbers until the targets below hold.
- Rewrite PLAYTEST.md for the redesign and walk through it natively and in Chrome and Firefox.
- Update README for the redesign; keep it minimal.
- Merge `redesign` into `main` so GitHub Pages deploys it.

Success criteria:
- [x] Across 100 seeds per profile, Idle reaches the IPO under 10% of the time (unit tested).
- [x] Across 100 seeds per profile, Random reaches the IPO under 25% of the time, so a careless player usually loses (unit tested).
- [x] Across 100 seeds per profile, Sensible reaches the IPO over 70% of the time (unit tested).
- [x] Every ending is reached by some strategy and seed (unit tested).
- [x] Sensible skipping any one action or purchase still reaches the IPO in some seeds (unit tested).
- [x] A full game takes about 20-30 minutes.
- [x] Sound effects play for major events and music loops during play.
- [x] Every item in PLAYTEST.md passes.
- [x] README is accurate and minimal, with no unused code or clippy warnings.
- [x] The Pages URL serves the redesigned game after the merge.

## Phase 21: Tool tiers

Goal: two or three tools per defense category at different price points, so spending is a real choice. All prices and boosts below are starting points for Phase 24.

Tasks:
- Replace the six tools with a catalog of 18, three per category. Each tool improves its own category and may add a smaller boost elsewhere:

| Category | Budget tier | Mid tier | Premium tier |
| --- | --- | --- | --- |
| Identity | Password manager ($10K, +12) | MFA tokens ($20K, +30) | Privileged access management ($70K, +25, Detection +10) |
| Endpoint | Antivirus suite ($10K, +12) | Application allowlisting ($35K, +20) | EDR ($60K, +25, Detection +10) |
| People | Awareness posters ($5K, +8) | Email security gateway ($25K, +20) | Training platform ($45K, +25) |
| Perimeter | Vulnerability scanner ($15K, +12, Endpoint +5) | DDoS protection and WAF ($40K, +30) | Attack surface management ($60K, +25, Detection +5) |
| Resilience | Incident runbooks ($8K, +10) | Immutable backups ($30K, +25) | Disaster recovery site ($80K, +30) |
| Detection | Log collection ($15K, +12) | SIEM ($80K, +30) | Managed detection service ($100K, +35) |

- Boosts stack within a category, still capped at 100.
- Existing dependencies keep their tools: SIEM tuning and the threat-hunt bonus need the SIEM, restore tests and restoring from backups need immutable backups, and enforcing MFA is how MFA tokens deploy.
- Vendor Hall gets category tabs: Staff, the six defense categories, and Services (IR retainer, insurance, coffee, subscription). Left/Right switch tabs, Up/Down choose within a tab, and the IMPROVES strip and spare-after-costs header stay.
- The Defenses screen, action list, and balance strategies use the new catalog.

Success criteria:
- [x] Every defense category offers at least two tools at different prices, and every tool improves its own category (unit tested).
- [x] Each tool's deployment applies exactly its listed boosts (unit tested over the whole catalog).
- [x] Existing dependencies still require their tool (unit tested).
- [x] Left/Right switch Vendor Hall tabs, Up/Down stay within a tab, and every item is reachable (unit tested).
- [x] Every tab fits on the 640x480 screen.

## Phase 22: Tool upkeep

Goal: deployed tools need regular maintenance, so there is always meaningful work in the late game.

Tasks:
- Tool boosts stop being added to posture permanently. Instead each deployed tool has a condition from 0 to 100 and contributes its boosts times condition / 100 on top of base posture and expertise.
- Condition drops 8 per week, so an untended tool falls to half strength in about six weeks.
- One maintenance action per category restores every deployed tool in it to full: Run an access review (Identity), Update endpoint policies (Endpoint), Refresh email filters (People), Update firewall and WAF rules (Perimeter), Rehearse disaster recovery (Resilience, replacing the backup restore test), Tune detections (Detection, replacing Tune the SIEM). Each takes 2 days, is free, and is offered only when a deployed tool in that category is below 80.
- False positives follow the SIEM's condition instead of a one-time tuned flag.
- The Defenses screen shows each tool's state: Fresh, Aging (below 80), or Stale (below 50), in green, amber, or red.

Success criteria:
- [ ] A deployed tool's contribution scales with its condition, and condition drops 8 every Monday (unit tested).
- [ ] Each maintenance action restores every deployed tool in its category and is offered only when one needs it (unit tested).
- [ ] An unmaintained SIEM raises more false positives than a maintained one (unit tested across seeds).
- [ ] Pen test grades, attacker odds, and result cards all count tool contributions (unit tested).
- [ ] The Defenses screen shows every deployed tool's state.

## Phase 23: Quarterly board funding

Goal: money keeps arriving, scaled by how much the board trusts the SOC, so trust matters and there is always something to spend late.

Tasks:
- Every sixth Monday (days 43, 85, 127, 169, and on through any delays), the board releases a grant: the profile's quarterly amount (Fintech $200K, Healthtech $160K, Gaming startup $120K) times trust / 60, so trust 60 gets the full grant, 30 gets half, 90 gets one and a half.
- The grant appears in the log and as a result card, and the next funding day shows on the SOC Status panel.
- The Public S-1 Flip top-up is removed; the random "board approves $100K" event stays.
- Starting budgets are reduced so total money over a game is similar, then tuned in Phase 24.

Success criteria:
- [ ] Grants arrive only on funding days, keep coming after IPO delays, and scale with trust as documented (unit tested).
- [ ] Each profile's grant matches its documented amount at trust 60 (unit tested).
- [ ] The Flip no longer adds budget (unit tested).
- [ ] The next funding day shows on the SOC Status panel.

## Phase 24: Rebalance and release

Tasks:
- Update the balance strategies: Sensible buys tiers it can afford, keeps tools maintained, and protects trust; Random buys and maintains at random.
- Tune prices, boosts, decay, grants, and starting budgets until the Phase 20 targets hold again.
- Update REDESIGN.md, README, and PLAYTEST.md; playtest natively and in the browser; deploy.

Success criteria:
- [ ] Idle reaches the IPO under 10%, Random under 25%, and Sensible over 70% on every profile (unit tested).
- [ ] Every ending is still reachable, and Sensible skipping any one purchase or action still wins some games (unit tested).
- [ ] In a playtest, the last eight weeks still offer meaningful choices every week.
- [ ] Every item in PLAYTEST.md passes, and the Pages URL serves the new version.

## Current status

- [x] MVP (Phases 1-11)
- [x] Phase 12: Calendar, meters, and endings
- [x] Phase 13: Setup and the Vendor Hall
- [x] Phase 14: Defender actions
- [x] Phase 15: The attacker
- [x] Phase 16: Random events
- [x] Phase 17: Landmarks and IPO delays
- [x] Phase 18: The Security Conference
- [x] Phase 19: Presentation
- [x] Phase 20: Balance, playtest, and release
- [x] Phase 21: Tool tiers
- [ ] Phase 22: Tool upkeep
- [ ] Phase 23: Quarterly board funding
- [ ] Phase 24: Rebalance and release
