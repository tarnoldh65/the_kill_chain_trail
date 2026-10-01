# The Kill Chain Trail: Redesign

Design document for taking the MVP to a fuller game. This describes what the game is and how it plays; the step-by-step build plan with success criteria goes in [PLAN.md](PLAN.md) once this design is approved.

## 1. Pitch

You run the security operations center (SOC) at a company racing toward its IPO. The bell is scheduled to ring in 26 weeks, if nothing pushes it back. Between now and then, attackers you mostly cannot see are trying to steal data, take systems down, and embarrass the company before it goes public. Spend your budget, your calendar, and your team's sanity to reach IPO day with the highest valuation you can.

The name stays. The kill chain is still there, but on the attacker's side and hidden from you. You only see its shadow: alerts, odd log entries, and, when you fail, incidents. The after-action report at the end finally shows you the trail the attackers were walking.

## 2. What changes from the MVP

| MVP | Redesign |
| --- | --- |
| Progress is the attacker moving through seven kill chain stages | Progress is the calendar moving toward IPO day |
| One decision per stage, seven turns total | Days pass continuously; actions take days or weeks; events interrupt |
| Containment meter | Hidden security posture and hidden attacker campaigns |
| Win = contain the attacker | Win = reach the IPO; score = final valuation |
| Fixed roster | Hire and name analysts at setup and at landmarks |
| Choices cost budget, coffee, fatigue | Actions cost budget and/or days; tools must be bought and then deployed |

Kept as-is or lightly reworked: name entry (company and incident lead), the leadership cast, burnout and Oregon Trail-style departure messages, the log plus numbered options layout, illustrated incident reports, the intern coffee run, 8-bit graphics, synthesized music and sound, 640x480 scaling, the web build.

## 3. Core loop

Like Oregon Trail, the game alternates between traveling and stopping.

1. **Outfit**: before day 1, spend the starting budget on staff, tools, and supplies.
2. **Travel**: choose "Continue". Days tick by along the timeline automatically, one day every half second or so, until something stops you: an alert, an event, an incident, a landmark, or the player pressing a key.
3. **Stop**: when the clock stops, the player gets the day menu (section 6), deals with whatever happened, and continues.
4. **Landmarks**: fixed dates on the calendar that act as forts (shop, hire, rest) and/or rivers (a challenge you must get across).
5. **IPO day**: the game ends, the valuation is final, and the after-action report shows what happened behind the scenes.

## 4. Setup

### 4.1 Names

Enter the company name and your name as incident lead (unchanged from the MVP).

### 4.2 Company profile (the "banker, carpenter, farmer" choice)

| Profile | Starting budget | Base valuation | Score multiplier | Flavor |
| --- | --- | --- | --- | --- |
| Fintech | High | $1.5B | x1 | Money attracts money-motivated attackers |
| Healthtech | Medium | $1.0B | x2 | PII breaches hurt the most |
| Gaming startup | Low | $600M | x3 | Fewer controls, angry teenagers, DDoS |

The profile sets the starting budget, the starting valuation, which threat actors are most likely, and the final score multiplier. All numbers in this document are starting points for tuning.

### 4.3 Outfitting at the Vendor Hall (the "general store")

A shop screen where the player spends the starting budget. A leftover balance carries into the game and pays weekly costs.

**Staff** (paid weekly):
- Junior analyst: cheap, slower, burns out faster.
- Senior analyst: expensive, faster at actions, better at investigating alerts.
- The player names each hire. Three to five analysts is a reasonable start.

**Tools** (one-time license cost; each must be *deployed* by a day action before it helps):
- EDR: endpoint protection and detection.
- SIEM: detection across the board.
- MFA tokens: identity protection.
- Email security gateway: phishing protection.
- DDoS protection / WAF: perimeter.
- Immutable backups: resilience against ransomware and outages.

A tool bought but never deployed is shelfware, and the end report says so.

**Services and supplies**:
- Incident response retainer: unlocks "Call the IR firm" during incidents.
- Cyber insurance: reduces budget losses from incidents.
- Coffee: pots for the break room (existing capacity of 36).

The Vendor Hall reopens at certain landmarks, usually at higher prices.

## 5. The calendar and timeline

- The run is scheduled for 26 weeks (182 days), but IPO delays can extend it (section 5.2). Day 1 is a Monday.
- A persistent timeline bar spans the top of every gameplay screen: a line from today to IPO day with landmark icons along it, a marker for the current day, and text such as `DAY 43  -  139 DAYS TO IPO`.
- Every Monday is payday: salaries and coffee upkeep are deducted. The log notes it.
- Threat activity ramps up as the IPO approaches and the company becomes more visible.

### 5.1 Landmarks

| Week | Landmark | Type | What happens |
| --- | --- | --- | --- |
| 0 | IPO Kickoff | Fort | Outfitting (section 4.3) |
| 4 | Board Security Briefing | River | Board judges your progress; trust rises or falls based on posture and incidents so far |
| 7 | Security Conference | Fort | Send yourself and chosen teammates for 3 days (section 5.3); Vendor Hall reopens; hire at the job fair |
| 10 | SOC 2 Type II Audit | River | Choose how to cross: rely on your controls, hire consultants (budget), or crunch to fix findings (days and fatigue). Failing hurts valuation and trust and delays the IPO |
| 13 | Confidential S-1 Filing | Fort | Decide whether to disclose past incidents in the risk factors: disclosure costs valuation now; hiding it risks a much bigger hit if it surfaces later |
| 16 | Third-Party Pen Test | River | Testers grade your posture as letter grades, the only time it is ever shown; weak areas cost valuation |
| 19 | Public S-1 Flip | Fort | The company is now public knowledge; threat level jumps. Vendor Hall and hiring reopen; budget top-up if trust is high |
| 23 | Roadshow | River | Executives travel; whaling and travel-related attacks spike. Choose how much to protect them |
| 26 | IPO Day | End | The bell rings. Final valuation is calculated |

Forts are where the player can buy, hire, and rest without the clock moving. Rivers are a single decision with a risky outcome, like fording, caulking, or taking the ferry.

### 5.2 IPO delays

Some failures push IPO day back by 2 weeks:
- Failing the SOC 2 Type II audit.
- A PII breach or insider leak after the Public S-1 Flip, which forces an amended filing.
- An unresolved Systems down condition when the Roadshow starts.

A delay moves IPO day and every landmark still ahead by 2 weeks, and the timeline bar redraws to the new length. It costs valuation and trust up front, and the extra weeks cost more payroll and give the attackers more time. They also give the defender more time to fix things. If total delays would exceed 6 weeks, the board pulls the IPO and the game ends.

### 5.3 The Security Conference

The week 7 landmark is a small minigame. You always attend; the question is who goes with you.

**1. Pick attendees.** The roster appears with each analyst's burnout. Number keys toggle who goes; ENTER confirms. Each attendee, including you, costs a ticket and travel (placeholder $6K each), and the running total is shown.

**2. Pick a track for each attendee.** Each track draws from its own pool of outcomes:

| Track | Likely result |
| --- | --- |
| Talks | Usually expertise in a random posture area; small burnout relief |
| Villages | Hands-on expertise (People, Endpoint, Identity) or pure fun; medium burnout relief |
| Expo floor | Expertise tied to a tool category (Perimeter, Detection, Resilience), plus swag; small burnout relief |
| Hallway track and parties | Almost always pure fun; large burnout relief |

**3. The conference runs for 3 days.** The clock keeps moving and attackers do not attend conferences. Whoever stays home holds the fort: Detection drops in proportion to how many analysts are away, and the stay-home crew gains extra burnout. Send everyone and the SOC is dark for 3 days.

**4. Conference report.** One illustrated card per attendee, revealed one at a time with ENTER, like the incident reports. No two attendees get the same card. Outcomes are always positive or neutral for posture; the worst case is that someone learned nothing and had a great time.

**Expertise belongs to the person.** An analyst who gains expertise adds a bonus to that posture area for as long as they stay on the team. If they burn out and leave, the bonus leaves with them:

- "Priya has quit to raise alpacas in Vermont. She took her WAF expertise with her."

**Example analyst cards** (`{name}` is the attendee):

| Track | Card | Effect |
| --- | --- | --- |
| Talks | "{name} sat through a three-hour talk on passwordless login and will not stop saying 'FIDO2'." | Identity expertise |
| Talks | "{name} attended 'Your Logs Are Lying To You' and now trusts nobody, including the logs." | Detection expertise |
| Talks | "{name} fell asleep in a keynote and woke up during a ransomware recovery workshop. Lucky." | Resilience expertise |
| Villages | "{name} spent the whole time at the Lockpick Village. They learned nothing useful but have never been happier." | Large burnout relief |
| Villages | "{name} won the Social Engineering Village by talking the hotel into giving them the keynote speaker's room key." | People expertise |
| Villages | "{name} took apart a laptop at the Hardware Village and only had two screws left over." | Endpoint expertise |
| Expo floor | "{name} attended one vendor session on WAFs and is now the company's self-declared WAF expert." | Perimeter expertise |
| Expo floor | "{name} collected 47 vendor t-shirts and will not need to do laundry until the IPO." | Medium burnout relief |
| Expo floor | "{name} got a free SIEM tote bag and, somehow, also learned to write detection rules." | Detection expertise |
| Hallway | "{name} skipped every talk for the hallway track. Three new friends, zero notes." | Large burnout relief |
| Hallway | "{name} discovered the hotel pool. The pool is now their favorite security control." | Large burnout relief |
| Hallway | "{name} wandered into a closed-door CISO roundtable and nobody noticed. They learned a lot and loved the free lunch." | Identity expertise and large burnout relief (rare) |

**Example cards for you, the incident lead** (you have no burnout meter, so yours affect the company instead):

| Card | Effect |
| --- | --- |
| "You gave a lightning talk about your SOC. The CISO saw it on LinkedIn." | Trust up |
| "You let a vendor scan your badge. You now get 40 emails a day and a 20% discount at the Vendor Hall." | Vendor Hall discount at this conference |
| "You met a senior analyst at the after-party who wants a job." | A senior candidate waits at the job fair with no hiring fee |
| "You spent the whole conference answering Slack from your hotel room." | Nothing; your team is grateful you were reachable |

After the report, the Vendor Hall and the job fair open as at any fort.

## 6. The day menu

When the clock stops, the player sees numbered options (layout unchanged from the MVP):

1. **Continue**: let the days roll.
2. **Take an action**: open the action list (section 7).
3. **Check the team**: roster with each member's burnout (Good / Fair / Poor / Very poor).
4. **Set the tempo**: Relaxed, Steady, or Crunch (the "pace" setting).
5. **Send the intern for coffee**: the existing street minigame.

Tempo is a standing setting rather than an action:

| Tempo | Action speed | Burnout | Coffee use |
| --- | --- | --- | --- |
| Relaxed | Slower | Recovers slowly | Low |
| Steady | Normal | Rises slowly | Normal |
| Crunch | Faster | Rises fast | High |

## 7. Defender actions

Actions cost money, days, or both. While an action is running, days pass and the world keeps moving; an alert or incident can interrupt it, and the player can resume it afterward. Each game offers the core actions plus a few that depend on what you own and what has happened.

| Action | Days | Cost | Effect |
| --- | --- | --- | --- |
| Deploy a purchased tool | 3-10 | None | Turns a tool on; raises its posture area |
| Enforce MFA everywhere | 5 | None (needs MFA tokens) | Big identity boost; brief morale dip from annoyed employees |
| Phishing simulation | 2 | Small | People boost; occasionally upsets a VP (trust dip) |
| Patch sprint | 4 | None | Endpoint and perimeter boost; fatigue |
| Threat hunt | 3 | None | Chance to find hidden attacker campaigns; better with seniors and SIEM |
| Tabletop exercise | 1 | None | Resilience boost; small trust boost with leadership |
| Backup restore test | 2 | None (needs backups) | Resilience boost |
| Tune the SIEM | 3 | None (needs SIEM) | Fewer false positives |
| Recruit an analyst | 7 | Hiring fee | Hire and name a new analyst |
| Give everyone the day off | 1 | None | Large burnout recovery; nobody watching for a day |
| Team offsite | 5 | Medium | Very large burnout recovery; reduced watch for the week |
| Brief leadership | 1 | None | Trust boost; the CIO may assign a pet project |

## 8. Hidden security posture

Posture is always hidden: no numbers, meters, or grades on screen. Six hidden values, each 0-100:

- **Identity**: MFA, password hygiene, privileged access.
- **Endpoint**: EDR, patching.
- **People**: phishing awareness, email security.
- **Perimeter**: DDoS protection, WAF, exposed services.
- **Resilience**: backups, tabletop exercises, IR retainer. Reduces the damage of a successful attack instead of preventing it.
- **Detection**: SIEM, tuning, threat hunting, analyst skill. Controls how early you notice an attacker and how many false alarms you see.

Every area also gets a bonus from analysts with conference expertise in it (section 5.3), which does not decay.

Posture slowly decays (new SaaS apps, new hires, configuration drift), so it has to be maintained, not just bought. The player gets fuzzy hints: the CISO's comments in the log, the board briefing reaction, and the pen test report card. The report card is a one-time snapshot and is not kept on screen afterward.

## 9. The attacker

### 9.1 Campaigns

Behind the scenes, threat actors start **campaigns** at random, more often as the IPO nears. Each campaign has an actor, an objective, and a hidden progress through the kill chain:

`Reconnaissance -> Initial access -> Foothold -> Objective`

| Actor | Usual entry | Objective | Defended by |
| --- | --- | --- | --- |
| Ransomware gang | Phishing, unpatched VPN | Encrypt and extort | People, Endpoint, Resilience |
| Data thief | Stolen credentials | PII leak | Identity, Detection |
| Competitor espionage | Spear phishing an engineer | Steal IP and roadmap | People, Identity |
| Hacktivists | Exposed services | DDoS and defacement | Perimeter |
| Nation-state APT | Supply chain, zero-day | Quiet long-term access | Detection |
| Malicious insider | Already inside | Leak S-1 details | Identity, Detection |

Each day, every active campaign rolls to advance one step against the posture areas that defend it. Better posture means slower or failed advances; a failed advance can end the campaign.

### 9.2 Alerts: seeing the shadow

When a campaign acts, a Detection roll decides whether the SOC sees anything. A detected step produces an alert that stops the clock:

> Impossible travel: the CFO logged in from Ohio and Romania 10 minutes apart.

The player picks:
1. **Investigate** (1-3 days): a chance to evict the campaign, higher with seniors.
2. **Call the IR firm** (needs retainer, costs budget): almost always evicts.
3. **Ignore**: no cost, no help.

Not every alert is real. False positives are mixed in, more often with an untuned SIEM, and investigating them wastes days. The player never knows for sure which is which; that uncertainty is the core of the defender experience.

### 9.3 Incidents: when the attacker wins

When a campaign reaches its objective, it becomes an **incident**: the clock stops, an illustrated incident report pops up, and the player must respond before continuing. Responses trade days, budget, valuation, and trust, with Resilience softening the blow.

| Incident | Response choices (examples) | Lingering condition |
| --- | --- | --- |
| Ransomware | Restore from backups (days, needs Resilience), pay the ransom (budget, trust), rebuild (weeks) | **Systems down**: most actions unavailable until recovered |
| PII breach | Disclose now, delay disclosure, call outside counsel | **Regulator inquiry**: no budget top-ups; a later event can fine you |
| IP theft | Lawsuit, quiet remediation | **Leaky roadmap**: valuation drifts down weekly until fixed |
| DDoS / defacement | Buy emergency mitigation, wait it out | **Downtime**: brand loyalty drops each day it lasts |
| APT discovered | Full eviction (weeks), partial cleanup | **Persistent access**: future campaigns start at Foothold |
| Insider leak | Fire the suspect, investigate | **Paranoia**: team burnout rises faster |

Lingering conditions are how a successful attack changes the actions available to the defender. Each shows as a status tag on screen and is cleared by a specific action.

## 10. Random events (the broken wagon wheels)

Non-attacker events that interrupt travel and force small decisions, good and bad:

- A senior analyst catches the flu (out 3 days).
- A major cloud provider goes down for a day.
- A zero-day drops for software you run (patch now or accept the risk).
- A key vendor is breached (supply chain risk rises).
- The CEO announces the IPO date on social media (threat level up).
- The CIO wants to migrate the data center before the IPO (accept: posture dip; refuse: trust dip).
- Coffee machine breaks (no coffee until fixed).
- Holiday week: fewer people around, attackers do not take holidays.
- Board approves a budget top-up.

## 11. People

### 11.1 Analysts

Each hired analyst has a name, a level (junior or senior), burnout from 0 to 100, and at most one area of conference expertise (section 5.3). Burnout rises with workload, Crunch tempo, incidents, and missing coffee, and falls with days off and Relaxed tempo. Workload is shared, so more analysts means less burnout per person, at a higher weekly cost.

At 100 burnout an analyst leaves, with Oregon Trail-style messages:

- "Priya has quit to raise alpacas in Vermont."
- "Marcus has died of alert fatigue."
- "Jen accepted an offer at a competitor for twice the salary."

### 11.2 Leadership

Same cast as the MVP, with clearer jobs:

- **CISO**: your boss. Gives hints about posture. Fired if corporate trust gets too low, which makes future trust gains smaller.
- **CIO**: brings pet projects and conflicting priorities through events.
- **SOC Manager**: let go if the budget gets too low, after which burnout rises faster.

The CEO and the board appear only through events and landmarks.

## 12. Meters and endings

### 12.1 Visible meters

- **Valuation**: the score. Starts at the profile's base. Rises when landmarks go well; falls from incidents, failed rivers, and lingering conditions.
- **Corporate trust** (0-100): leadership's confidence in the SOC.
- **Brand loyalty** (0-100): customer and public confidence. Low brand drags valuation down each week.
- **Budget**: cash on hand; weekly payroll and upkeep come out every Monday.
- **Coffee**: pots in the break room.

### 12.2 Endings

| Ending | Trigger |
| --- | --- |
| **IPO** (win) | Reach IPO day. Score = final valuation x profile multiplier |
| **IPO pulled** | Valuation falls below 40% of the base, or IPO delays would exceed 6 weeks. The board shelves the IPO |
| **Shut down** | Brand loyalty hits 0. Customers leave and the company folds |
| **Fired** | Corporate trust hits 0. You are walked out by security |
| **Team collapsed** | No analysts left |
| **Bankrupt** | Cannot make payroll on a Monday after laying off everyone you can |

Only the IPO ending produces a score; every other ending is a straight loss with no score.

Every ending shows the after-action report: a timeline of every attacker campaign, how far it got along the kill chain, whether you saw it, and which alerts were real. A top-ten scoreboard of IPO valuations stays on the title screen (stored locally).

## 13. Screen layout (640x480)

```
+------------------------------------------------------------------+
| KICKOFF  BOARD  CONF  AUDIT  S-1  PENTEST  FLIP  ROADSHOW   IPO  |
| *=========>-----o------o------o-----o--------o------o--------[B] |
| DAY 43  WED                                    139 DAYS TO IPO   |
+---------------------------+--------------------------------------+
| ACME CORP                 | LOG                                  |
| VALUATION   $1.21B        | Mon: Payroll $42K.                   |
| TRUST       ######----    | Tue: Patch sprint complete.          |
| BRAND       #######---    | Wed: ALERT - Impossible travel on    |
| BUDGET      $310K         |      the CFO's account.              |
| COFFEE      18 POTS       |                                      |
| TEMPO       STEADY        |                                      |
| STATUS      REGULATOR     |                                      |
|                           |                                      |
| TEAM                      |                                      |
|  PRIYA (SR)   GOOD        |                                      |
|  MARCUS (JR)  POOR        |                                      |
+---------------------------+--------------------------------------+
| 1 INVESTIGATE (1-3 DAYS)   2 CALL IR FIRM ($40K)   3 IGNORE      |
+------------------------------------------------------------------+
```

The timeline marker is a small 8-bit sprite (an analyst pushing a server rack, standing in for the wagon) that moves along the bar as days pass.

## 14. Audio and art

- Keep the existing synthesized music tracks and the Rush Hour coffee run track.
- New sound cues: day tick, payday, alert klaxon, incident sting, landmark fanfare, IPO bell.
- New sprites: timeline marker, landmark icons, incident illustrations per incident type, one illustration per conference track, IPO bell.
- Everything stays in the existing palette and 8x8 font.

## 15. Testing and balance

The game logic stays separate from input and rendering, deterministic for a given seed, so it can be simulated in unit tests.

Balance targets, verified by simulated strategies across many seeds:
- **Idle** (always Continue, ignore every alert): almost always loses before IPO day.
- **Random**: reaches the IPO roughly a third of the time.
- **Sensible scripted** (deploy tools, investigate alerts, rest when burned out): reaches the IPO most of the time.
- Conference outcomes never reduce posture, every conference card occurs across seeds, and expertise leaves with the analyst who holds it.
- Every ending is reachable, every incident type occurs, every landmark outcome occurs, and every IPO delay trigger occurs.
- No single action or purchase dominates: a scripted strategy that skips any one of them can still win.

## 16. Scope

**In**: everything above.

**Out** (for now): saving mid-game, multiple difficulty levels beyond the company profile, analyst specializations beyond junior, senior, and conference expertise, mouse input, online scoreboards.

## 17. Decisions

- Posture stays hidden. The pen test report card is the only time it is shown.
- The IPO can be delayed (section 5.2).
- All three company profiles are kept.
- The Security Conference is a minigame: choose attendees and tracks, get humorous outcome cards that only improve posture or reduce burnout (section 5.3).
- The run is 26 weeks; numbers are tuned through playtesting.
- Only reaching the IPO produces a score; every other ending is a straight loss.
