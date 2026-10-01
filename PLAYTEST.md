# Playtest checklist

Walk through natively (`cargo run --release`) and in Chrome and Firefox.

## Flow
- [x] The title shows "The Kill Chain Trail", the music picker works (1-3 tracks, 4 off), and the top ten appears once an IPO is scored.
- [x] Company and lead names accept typing and Backspace; empty names are refused.
- [x] Each profile shows its budget, valuation, and multiplier, and starts with them.
- [x] The Vendor Hall hires and names analysts, buys tools and services once, stops coffee at 36 pots, and needs an analyst before opening.
- [x] Option 6 opens the Vendor Hall any day; hires after day 1 start a week-long search shown on the Hiring line, and the analyst joins with a result card.
- [x] Days pass on Continue and any key stops the clock; the timeline marker moves and the days-to-IPO count drops.
- [x] Actions start, show in the status panel, pause when interrupted, and resume with option 1.
- [x] The intern coffee run is playable and fair, and Rush Hour music plays during it.
- [x] The coffee subscription charges $2K and restocks the break room every Monday, and the intern's fancy coffee lowers analyst burnout and shows a result card.
- [x] Alerts take over the menu; investigating shows in the Probe line; the IR firm needs the retainer.
- [x] Incidents show their picture and responses; lasting conditions show as red tags and their cure appears in the actions.
- [x] Random events appear in the log, and events with choices take over the menu.
- [x] The log groups entries under day headers with space between them, and alerts and incidents stand out.
- [x] Finished actions, investigations, IR calls, incident responses, event choices, and river crossings show a result card that is also in the log.
- [x] The team screen lets you choose an analyst and fire them after a Y confirmation, for a week's salary; absent analysts and the last analyst cannot be fired.
- [ ] The Vendor Hall's tabs list three tools per defense area; deployed tools show as DEPLOYED.
- [ ] Deployed tools age week by week (fresh, aging, stale on the defenses screen), and each area's maintenance action appears when needed and restores them.
- [ ] The board releases funding every sixth Monday, scaled by trust, with a result card; SOC Status shows the next funding day, and spare after costs never runs out before a grant arrives.
- [ ] The last eight weeks still offer a meaningful choice every week.
- [x] D opens the defenses screen from any play screen: each defense shows who it slows, what is helping it (amber for tools not yet deployed), and the pen test grade, which stays after the pen test.
- [x] The Vendor Hall and action list light up the defenses the selected item or action improves.
- [x] L opens the full log on its newest page from any play screen; LEFT/RIGHT turn pages back to the first entry, and ENTER returns where you were.
- [x] Every landmark stops the clock on its day: forts offer the Vendor Hall, rest, and moving on; rivers offer their choices; the pen test shows letter grades.
- [x] The conference picker, track picker, three days away, and card reveal all work, and perks apply at the Vendor Hall.
- [x] An IPO delay moves IPO day and later landmarks on the timeline.
- [x] Each ending shows its message, ENTER shows the after-action report, and ENTER returns to the title.

## Quality
- [x] Every screen is readable at 640x480 and scales cleanly when resized.
- [x] The flow is understandable without extra explanation.
- [x] Sound effects play for days, paydays, alerts, incidents, landmarks, and the IPO, and music loops during play.
- [x] A full game takes about 20-30 minutes.
- [x] A thoughtful player usually reaches the IPO; a careless one usually loses.
