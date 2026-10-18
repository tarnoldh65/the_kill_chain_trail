# Plan for The Kill Chain Trail

Plan for the redesign in [REDESIGN.md](REDESIGN.md), based on [AGENTS.md](AGENTS.md). A phase is complete only when every success criterion is checked and `cargo test` and `cargo clippy` are clean for native, and the wasm release build succeeds.

## Libraries

- Rust 2024 edition, latest stable toolchain.
- [macroquad](https://crates.io/crates/macroquad) for window, scaling, 2D drawing, input, and audio.
- [font8x8](https://crates.io/crates/font8x8) for a public domain 8x8 pixel font.
- The top-ten scoreboard is saved without a library: a plain file natively, and a small `web/storage.js` plugin for localStorage in the browser. (quad-storage was planned, but its JavaScript depends on sapp_jsutils helpers that the current macroquad bundle keeps private.)
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
- Top-ten scoreboard of IPO scores on the title screen, saved to `top_ten.txt` natively and to localStorage through `web/storage.js` in the browser.

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
- [x] A deployed tool's contribution scales with its condition, and condition drops 8 every Monday (unit tested).
- [x] Each maintenance action restores every deployed tool in its category and is offered only when one needs it (unit tested).
- [x] An unmaintained SIEM raises more false positives than a maintained one (unit tested across seeds).
- [x] Pen test grades, attacker odds, and result cards all count tool contributions (unit tested).
- [x] The Defenses screen shows every deployed tool's state.

## Phase 23: Quarterly board funding

Goal: money keeps arriving, scaled by how much the board trusts the SOC, so trust matters and there is always something to spend late.

Tasks:
- Every sixth Monday (days 43, 85, 127, 169, and on through any delays), the board releases a grant: the profile's quarterly amount (Fintech $180K, Healthtech $160K, Gaming startup $120K) times trust / 60, so trust 60 gets the full grant, 30 gets half, 90 gets one and a half.
- The grant appears in the log and as a result card, and the next funding day shows on the SOC Status panel.
- The Public S-1 Flip top-up is removed; the random "board approves $100K" event stays.
- Starting budgets drop to $600K, $450K, and $300K; with grants, total money over a game is about 50% higher than before.
- The Vendor Hall's spare after costs becomes the lowest the budget reaches before the IPO, stepping through each week's costs and each grant at today's trust. (Later removed from the screen as confusing; the Vendor Hall was renamed Procurement.)

Success criteria:
- [x] Grants arrive only on funding days, keep coming after IPO delays, and scale with trust as documented (unit tested).
- [x] Each profile's grant matches its documented amount at trust 60 (unit tested).
- [x] The Flip no longer adds budget (unit tested).
- [x] The next funding day shows on the SOC Status panel.

## Phase 24: Rebalance and release

Tasks:
- Update the balance strategies: Sensible buys tiers it can afford, keeps tools maintained, and protects trust; Random buys and maintains at random.
- Tune prices, boosts, decay, grants, and starting budgets until the Phase 20 targets hold again.
- Update REDESIGN.md, README, and PLAYTEST.md; playtest natively and in the browser; deploy.

Success criteria:
- [x] Idle reaches the IPO under 10%, Random under 25%, and Sensible over 70% on every profile (unit tested).
- [x] Every ending is still reachable, and Sensible skipping any one purchase or action still wins some games (unit tested).
- [ ] In a playtest, the last eight weeks still offer meaningful choices every week.
- [ ] Every item in PLAYTEST.md passes, and the Pages URL serves the new version.

## Phase 25: Posture from tools and analysts

Issue #19. Raw posture starts at 30 and loses 2 every Monday, so it is 0 by the pen test and the best grade a player can earn is about a C. Posture becomes a value calculated from what the player controls.

Tasks:
- Remove the stored `posture` array, its weekly decay, and its starting value of 30. An area's level is the sum of deployed tools' boosts times condition / 100, plus specialty skill (junior 15, senior 25), plus 10 per analyst with conference expertise in the area, plus active operation boosts (Phase 26), capped at 100. Every area is 0 on day 1.
- Analysts may carry a specialty area. New hires have none for now; choosing a specialty belongs to a later hiring feature. A specialty leaves with the analyst.
- Retune tool boosts so pricier tools never give less:
  - Identity: password manager 12, MFA 30, PAM 30 plus Detection 15.
  - Endpoint: antivirus 12, allowlisting 25, EDR 30 plus Detection 15.
  - People: posters 10, email gateway 25, training platform 30.
  - Perimeter: vulnerability scanner 12 plus Endpoint 5, WAF 30, ASM 30 plus Detection 10.
  - Resilience: runbooks 10, backups 25, DR site 35.
  - Detection: log collection 12, SIEM 30, MDR 35.
- Events wear tools instead of posture. The zero-day's "Patch it tonight" restores 20 condition to Perimeter and Endpoint tools, and "Accept the risk" costs Perimeter tools 30. Going along with the CIO's migration costs every deployed tool 10.
- Until Phase 26 replaces them, the patch sprint, phishing simulation, and tabletop restore their areas' tools to full condition instead of adding posture.

Success criteria:
- [x] A new game is 0 in every area, and a Monday lowers a level only through tool wear (unit tested).
- [x] A specialty adds 15 for a junior and 25 for a senior, and leaves with the analyst (unit tested).
- [x] Event wear changes only the tools in the named areas, and condition stays in 0-100 (unit tested).
- [x] With tools at 88% condition, nothing grades F, MFA plus a junior specialist grades C, two tools plus a senior grade B, and all three tools plus a senior grade A (unit tested).
- [x] Within each area, a pricier tool never gives less total boost (unit tested).

## Phase 26: Tool operations

Patch sprint, phishing simulation, tabletop, and threat hunt stop being always available. Operations become longer tasks that a deployed tool unlocks, each with a tangible payoff. Posters, MFA, antivirus, allowlisting, the email gateway, the WAF, and MDR have none.

Tasks:
- Operations by tool:

| Tool | Operation | Days | Payoff |
|---|---|---|---|
| Password manager | Sticky note sweep | 3 | Identity +10; 1 in 4 chance trust -2 |
| PAM | Rotate the keys | 5 | Evict data thieves and insiders; Identity +10 |
| EDR | Compromise assessment | 6 | Evict ransomware and nation-states; Endpoint +10 |
| Training platform | Phishing simulation | 4, $5K | People +20; 1 in 3 chance trust -3 |
| Vulnerability scanner | Patch sprint | 5 | Perimeter +15, Endpoint +10; burnout +8 |
| ASM | Forgotten server cleanup | 5 | Evict hacktivists; Perimeter +10 |
| Runbooks | Tabletop exercise | 2 | Resilience +10; trust +2 |
| Immutable backups | Backup recovery drill | 4 | Brace against ransomware; Resilience +10 |
| DR site | Failover test | 5 | Brace against every attacker; 1 in 4 chance brand -3 |
| Log collection | Log review | 3 | Weak eviction roll against insiders; Detection +10 |
| SIEM | Threat hunt | 6 | Eviction roll against every campaign |

- Boosts last four weeks; running an operation again refreshes its boost instead of stacking. When a boost wears off, the log says so.
- Boosts and eviction odds scale with the tool's condition when the operation finishes. Seniors still improve eviction odds.
- A brace halves the damage of the next incident from a matching attacker, then is used up.

Success criteria:
- [x] Each operation is offered only while its tool is deployed (unit tested).
- [x] A finished operation applies exactly its boosts, scaled by the tool's condition (unit tested).
- [x] Boosts expire after four weeks, and a re-run refreshes instead of stacking (unit tested).
- [x] A brace halves one matching incident's damage and is then gone (unit tested).
- [x] Targeted evictions only end campaigns by their attackers (unit tested).
- [x] Every operation has three distinct result messages (unit tested).

## Phase 27: Actions screen, balance, and docs

Tasks:
- The Actions screen gets LEFT/RIGHT tabs, one per kind: Recovery, Implementation, Maintenance, Operations, Management. Empty tabs are dimmed and skipped. The menu opens on the first tab with actions, so Recovery comes first after an incident. When a tool is not fresh, the action's description says the results will be weaker.
- The IMPROVES strip in Procurement and on the Actions screen shows relative benefit without numbers: + for small (under 15), ++ for medium (15-29), +++ for large (30 and up), under each area it improves.
- The defenses screen lists active operation boosts with days left, and braces under Resilience.
- The Sensible strategy runs operations in rotation and pushes harder in the four weeks before the pen test. Operation boosts rise by 5 (Identity, Endpoint, Perimeter, Resilience, and Detection boosts become 15; the phishing simulation 25; the patch sprint 20 Perimeter and 15 Endpoint). The Phase 20 targets still hold, so attacker and landmark odds are unchanged.
- Sensible pen test grades are locked in as regression floors rather than a C everywhere: Gaming can afford only cheap tools, and real C floors wait for a hiring feature that adds specialists.
- Update REDESIGN.md (sections 7, 8, 10, 15, 17), the manual, README, and PLAYTEST.md.

Success criteria:
- [x] LEFT/RIGHT change tabs and skip empty ones, the menu opens on Recovery when a condition is active, and ENTER starts the action under the cursor (unit tested).
- [x] Benefit strength maps to +, ++, and +++ at the documented thresholds (unit tested).
- [x] Idle reaches the IPO under 10%, Random under 25%, and Sensible over 70% on every profile (unit tested).
- [x] Every ending is still reachable, and Sensible skipping any one purchase or action, including all operations, still wins some games (unit tested).
- [x] Sensible pen test grade points average at least 1.9 for Fintech, 1.3 for Healthtech, and 0.7 for Gaming, where A is 4 and F is 0 (unit tested).
- [ ] In a playtest, a maintained build earns Bs or As on the pen test, and no Actions tab overflows.

## Phase 28: Resumes and candidate searches

Issue #20. Analysts become choices: each candidate has a resume with proficiencies, an optional tool specialty, and a salary that reflects them. Hiring happens from a slate of resumes, and new slates come from a two-week candidate search.

Tasks:
- An analyst has a level, one or more proficiencies (defense areas), an optional tool specialty, and a salary. This replaces the Phase 25 single specialty area.
  - Each proficiency adds the level's full skill to its area: 15 for a junior, 25 for a senior.
  - A tool specialty halves the days (rounded up, at least 1) of every action tied to that tool: deploying it, maintaining its area while it is deployed, and its operation. A second specialist in the same tool adds nothing.
  - Salary is $4K a week for a junior and $8K for a senior, plus $1K for each proficiency after the first and $1K for a specialty.
- Resumes show the title (Security Analyst or Senior Security Analyst), proficiencies, tool specialty, salary, and a generated name, which is prefilled when hiring and can be changed.
  - Juniors have 1-2 proficiencies and sometimes a specialty; seniors have 1-3 and more often a specialty. A specialty is always a tool in one of their proficiency areas.
  - Resumes are generated from the game's seed, so a seed always offers the same candidates.
- A slate holds 5 senior and 5 junior resumes. Hiring from it is instant and has no fee; the hired resume leaves the slate. Hiring windows are short, and candidates who are not picked find other jobs:
  - Day 1 starts with a slate, which clears when the first Procurement opens for business.
  - A new Management action, Post job openings, costs $10K and takes a day; the search then runs in the background, and the resumes arrive 14 days after posting. Only one search runs at a time. When it ends, a result card says the candidates are in, and Procurement offers Review resumes for 7 days, with the countdown on SOC STATUS.
  - Once the resumes have been reviewed, leaving Procurement clears the slate. If they are not reviewed within the 7 days, a result card says the candidates all found other jobs.
- Attending the security conference brings a separate job fair slate, available only at the conference fort and cleared on leaving it. It never touches a waiting slate or a running search.
  - It is smaller but stronger: 2 seniors and 2 juniors, each with a tool specialty and one more proficiency than usual (juniors 2, seniors 2-3). Salaries follow the usual formula.
  - The after-party card adds a bargain senior: a fifth job fair resume at the base salary, whatever its skills.
- Paranoia blocks posting job openings and hiring, as it blocks hiring today. The eight-desk limit still applies.
- Remove the recruiter's fee, week-long per-analyst searches, instant hiring rules, and the free-senior perk.

Success criteria:
- [x] Proficiencies add 15 or 25 per area by level, and leave with the analyst (unit tested).
- [x] A specialist halves the days of deploying, maintaining, and operating their tool, and no other action (unit tested).
- [x] Salaries follow the formula, and every resume's specialty is a tool in one of its proficiency areas (unit tested).
- [x] Day 1 offers 5 senior and 5 junior resumes; hiring one is instant, free, and removes it from the slate; opening for business clears the rest (unit tested).
- [x] Posting job openings costs $10K and brings 5 and 5 new resumes 14 days later with a result card; a second search cannot start while one runs (unit tested).
- [x] A slate clears on leaving Procurement after review, or after 7 unreviewed days with a result card saying the candidates found other jobs (unit tested).
- [x] The job fair offers 2 seniors and 2 juniors, all with a specialty and an extra proficiency, only at the conference fort, without touching a waiting slate or a running search; the after-party adds a senior at base salary (unit tested).
- [x] Paranoia blocks posting and hiring, and the desk limit holds (unit tested).
- [x] The same seed always offers the same resumes (unit tested).

## Phase 29: Resume screens, balance, and docs

Tasks:
- While a slate is open, Procurement's Staff tab lists "Review senior resumes" and "Review junior resumes" with how many are waiting, and at the conference fort a "Job fair candidates" row. ENTER opens the resume viewer for that row. With no slate, the tab says when the next resumes arrive, or to post job openings.
- The resume viewer shows one resume as a card. LEFT/RIGHT cycle through that row's resumes, ENTER hires (opening the name popup with the suggested name), and B goes back. Once every resume at a level is hired, its row says so.
- The team screen shows the selected analyst's proficiencies and specialty below the roster. SOC STATUS shows days left in a running search, then days left to review resumes.
- The Sensible strategy hires from the slate, preferring candidates whose proficiencies cover its weakest areas, and posts job openings when it is short-handed. Re-measure the pen test grades and raise the floors to match.
- Update REDESIGN.md (setup, people, and posture sections), the manual, README, and PLAYTEST.md.

Success criteria:
- [x] The Staff tab opens the resume viewer for each level; LEFT/RIGHT cycle, ENTER hires with the suggested name, and B returns to Procurement (unit tested).
- [x] With no slate open, nothing can be hired, and the Staff tab says how to get more candidates (unit tested).
- [x] Idle reaches the IPO under 10%, Random under 25%, and Sensible over 70% on every profile, and every ending is still reachable (unit tested).
- [x] Sensible pen test grade floors are re-measured and raised to 2.7 for Fintech, 2.2 for Healthtech, and 1.15 for Gaming (unit tested).
- [ ] In a playtest, the resume viewer is readable at 640x480, and hiring, posting job openings, and the conference slate all work.

## Phase 30: Song arrangements

The three music tracks grow from 4-bar loops into 48-bar songs (90 seconds at 128 BPM) of five sections that loop back to the intro:
- Intro (bars 1-8): the original 4-bar loop, played twice.
- Melody (bars 9-16): an 8-bar square-wave main melody through a low-pass filter that opens over the section.
- Full groove (bars 17-24): the filter is fully open, with a sub-bass and denser drums.
- Breakdown (bars 25-40): the kick, bass, and sub-bass drop out. The melody and backing modulate up a half step after 8 bars while the filter closes and sweeps open again, with a noise riser and an accelerating snare roll over the last 4 bars.
- Final (bars 41-48): the full groove slams back in the home key, so the loop back to the intro is seamless.
The last bar of every other section is a snare fill.

Tasks:
- Add a one-pole low-pass filter with a cutoff that changes over time.
- Describe the arrangement as a table of sections, and render a song from it given its 8-bar melody (with rests) and per-bar pattern.
- Packet Storm (A minor) keeps its kick, off-beat hats, octave bass, and arpeggio, with a flowing main melody; the groove adds sixteenth hats, a backbeat snare, and a sub-bass.
- Zero Day Drive (D minor) keeps its rolling sixteenth bass, snare, and triangle lead, with a staccato call-and-response main melody ending in a chromatic slide; the groove adds sixteenth hats and a sub-bass.
- Neon Firewall (E minor) keeps its broken-beat kicks, triangle pad, and pentatonic lead, with a main melody of long ringing notes; the groove adds a kick and a sub-bass.
- Rush Hour stays unchanged.

Success criteria:
- [x] Every music track is 48 bars, 90 seconds (unit tested).
- [x] Per bar, each breakdown has far less low end than the groove (unit tested).
- [x] The low-pass filter passes a steady signal and attenuates a square wave more at a lower cutoff (unit tested).
- [x] No music track clips, and every section is audible (unit tested).
- [x] Rush Hour is still four bars and never clips (unit tested).
- [ ] In a playtest, the sections are audible in order, the loop back to the intro is seamless, and the game still starts promptly on desktop and web.

## Phase 31: Fight for budget

A Street Fighter-style mini game against the boss, offered as an action when money runs low.

Rules:
- "Fight for budget" appears on the Management tab of Take an action only while the budget is below 50% of the profile's starting budget (Fintech $300K, Healthtech $225K, Gaming $150K), and at most once per week, like the coffee run. It takes no days and costs nothing to start.
- Picking it opens a stake popup: 1 $10,000, 2 $40,000, 3 $75,000, 4 $100,000, B backs out.
- One round, 60 seconds. Each fighter has a health bar. Knocking the boss to zero wins; running out of health loses; at time up, the higher health wins and a tie goes to the boss.
- Win: the stake is added to the budget and brand rises 10% (brand * 1.1, capped at 100). Lose: trust drops 30% (trust * 0.7) and the budget loses 10% of its current amount. A result card shows the change, and a log line records it with a humorous message. Losing can end the game through trust like any other trust loss.
- Controls: UP stands, DOWN crouches, LEFT/RIGHT walk backward/forward while held (anywhere across the 640px screen, but fighters cannot pass through each other), Q punches, E kicks.
- Standing attacks are high and miss a crouching target. Crouching attacks are low and hit either stance. Punches are quick with short reach and light damage; kicks are slower with longer reach and heavier damage. Each attack has a wind-up, an active frame window, and a recovery, so it cannot be spammed.
- The boss is driven by a simple timer-based AI: it walks into range, attacks, sometimes crouches under high attacks, and backs off after taking a hit. Higher stakes give the boss more health, more damage, faster walking, and quicker reactions, in four fixed tiers.

Tasks:
- `src/fight.rs`: fighter state (x, stance, attack and its phase, health, hit stun), the boss AI, hit detection, the round timer, and the result. Pure logic driven by `tick(dt, held)` and key presses, with its own seeded random rolls so fights do not disturb the shared generator.
- Input: `Screen::tick` takes the held LEFT/RIGHT direction from `main.rs` (held keys are needed for walking; everything else stays press-based).
- `GameState`: `can_fight()`, `fight_settled(stake, won)` applying the rewards or penalties, with a once-per-week flag reset on Monday alongside the coffee run's.
- `ui.rs`: `Action::FightForBudget` (or a dedicated Management row) leading to `Screen::Stake`, then `Screen::Fight`, then the result card back on the day screen. Cues for swing, kick, hit landed, pain, and win/lose.
- Art: 8-bit sprites for a muscular fighter in a suit with a tie tied around the head, in six poses (stand, crouch, stand punch, stand kick, crouch punch, crouch kick) plus a hit pose. The player faces right on the left; the boss is the same art mirrored, with a different suit and tie color. Background: an office floor with desks and a row of tall windows showing a night city skyline. Health bars, names, and the timer across the top.
- Audio: a new fight track ("Boardroom Brawl"), faster and harder than the others (about 150 BPM, minor key, driving bass, pounding kicks), swapped in and out like Rush Hour and following the music volume and Off setting. Synthesized effects for a punch swing, a kick swing, a landed strike, and a pain grunt, following the game play sounds volume.
- Update the manual, README, and PLAYTEST.md.

Success criteria:
- [x] Fight for budget is offered only below 50% of the starting budget and once per week (unit tested).
- [x] Each stake starts a fight with the matching boss tier, and B on the stake popup returns to the action list (unit tested).
- [x] Holding LEFT/RIGHT walks within the screen, and the fighters never pass through each other (unit tested).
- [x] Standing attacks miss a crouching target, crouching attacks hit both stances, and an attack only lands during its active frames (unit tested).
- [x] Winning adds the stake and raises brand 10%; losing cuts trust 30% and the budget 10% (unit tested).
- [x] Time up awards the fight to the higher health, with a tie going to the boss (unit tested).
- [x] With scripted idle input the boss wins at every tier, and tougher tiers win faster (unit tested).
- [x] The fight track is a whole number of bars and never clips, and the new effects never clip (unit tested).
- [ ] In a playtest, the fight is readable at 640x480, the controls feel responsive, the $10,000 boss is beatable by a new player, and the $100,000 boss is hard but fair.

## Phase 32: Intelligence

A seventh defense area, Intelligence: knowing who is coming before they arrive. It slows the targeted attackers directly, and its tools and operations push what they learn into Endpoint, Detection, and Perimeter.

Rules:
- `Area::Intelligence` joins the hidden posture (0-100), the pen test report card (seven grades), and the defenses screen. It is built the same way as every other area: deployed tools by condition, analyst proficiencies and conference expertise, and operation boosts.
- Intelligence slows the attackers who pick their targets: nation-state APTs (Detection, Intelligence), competitor espionage (People, Identity, Intelligence), and hacktivists (Perimeter, Intelligence). Ransomware, data thieves, and insiders are unchanged.
- Procurement gets an Intelligence tab, between Detection and Services:

| Tool | Price | Days to deploy | Boosts |
|---|---|---|---|
| OSINT process | $5K | 2 | Intelligence 12, Perimeter 5 |
| Threat intel feeds | $30K | 3 | Intelligence 25, Detection 10 |
| Threat intel platform | $80K | 6 | Intelligence 30, Detection 10, Endpoint 5, Perimeter 5 |

- Activities. The first three are operations unlocked by a deployed tool (four-week boosts, scaled by condition, like Phase 26); the fourth is the area's maintenance action:

| Activity | Unlocked by | Days | Payoff |
|---|---|---|---|
| Check the dark web for targeted campaigns | OSINT process | 3 | Intelligence +15; brace against ransomware and data thieves; 1 in 5 chance burnout +5 (it is the dark web) |
| Consult industry ISAC | Threat intel feeds | 2, $2K | Intelligence +15, Detection +10; trust +1 |
| Update indicators of compromise | Threat intel platform | 3 | Detection +15, Endpoint +10, Perimeter +10; weak eviction roll (25%) against APTs and espionage |
| Tune threat feeds | Maintenance | 2 | Restores Intelligence tools to full strength |

- Resumes: proficiencies and tool specialties already draw from every area and its tools, so intel analysts and OSINT, feed, and platform specialists appear automatically. Add two intel-flavored candidate quotes.
- Everything that lists or loops over areas covers seven: the all-areas event wear, the pen test average, the IMPROVES strip, and the defenses screen. Add one Talks and one Villages conference card with Intelligence expertise.
- Each new tool gets three rollout messages, each operation three results and a fade message, and Tune threat feeds three upkeep messages, all in the game's humor.

Tasks:
- `game.rs`: the area, three items (price, label, description, rollout, category, boosts, deploy days), the Tune threat feeds label, quotes, and every `[_; 6]` sized by `Area::ALL.len()`.
- `attack.rs`: the new defenses. `operations.rs`: the three operations. `events.rs` and `conference.rs`: wear and cards.
- `ui.rs` and `draw.rs`: a ninth Procurement tab; the IMPROVES strip and defenses screen fit seven areas at 640x480 (shorter label "Intel" on the strip, tighter row spacing on the defenses screen).
- `balance.rs`: the Sensible strategy buys the OSINT process early, feeds in the mid tier, and the platform with the premiums, and rotates the new operations. Re-measure win rates and pen test floors.
- Update REDESIGN.md (tools, operations, posture, and attacker tables), the manual (areas and actions pages), README, and PLAYTEST.md.

Success criteria:
- [x] Intelligence starts at 0 and rises from intel tools by condition, proficiencies, expertise, and operation boosts (unit tested).
- [x] Each intel tool adds exactly its documented boosts to Intelligence, Endpoint, Detection, and Perimeter (unit tested).
- [x] Higher Intelligence slows APTs, espionage, and hacktivists, and leaves ransomware, data thieves, and insiders unchanged (unit tested).
- [x] Each intel operation is offered only while its tool is deployed and applies its boosts, brace, eviction, trust, or cost as documented; Tune threat feeds appears once an intel tool ages and restores it (unit tested).
- [x] Resumes can carry Intelligence proficiency and intel tool specialties, and every specialty is still a tool in a proficiency area (unit tested).
- [x] The pen test grades seven areas, and the Intelligence tab lists exactly the three intel tools (unit tested).
- [x] Idle reaches the IPO under 10%, Random under 25%, and Sensible over 70% on every profile; every ending is still reachable; pen test floors are re-measured and set to 2.65 for Fintech, 2.1 for Healthtech, and 0.95 for Gaming (unit tested).
- [ ] In a playtest, the Procurement tabs, IMPROVES strip, and defenses screen fit at 640x480 with seven areas.

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
- [x] Phase 22: Tool upkeep
- [x] Phase 23: Quarterly board funding
- [ ] Phase 24: Rebalance and release
- [x] Phase 25: Posture from tools and analysts
- [x] Phase 26: Tool operations
- [ ] Phase 27: Actions screen, balance, and docs
- [x] Phase 28: Resumes and candidate searches
- [ ] Phase 29: Resume screens, balance, and docs
- [ ] Phase 30: Song arrangements
- [ ] Phase 31: Fight for budget
