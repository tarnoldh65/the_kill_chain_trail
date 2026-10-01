## Business Requirements

- A computer game simulating a security operations center (SOC) defending a company in the same realm as Oregon Trail. The full design is in REDESIGN.md.
- The opening screen displays the game name "The Kill Chain Trail".
- At the beginning of the game the player names the company being protected and their own name as incident lead, picks a company profile, then spends a starting budget on staff, tools, and supplies.
- The goal is to reach the company's IPO, 26 weeks away, with the highest valuation. Only reaching the IPO produces a score; every other ending is a straight loss.
- Progression is a calendar toward IPO day, shown as a persistent timeline along the top of the screen. Days pass continuously, actions take days or weeks, and random events, alerts, and incidents interrupt, like Oregon Trail.
- Landmarks are fixed dates on the calendar (board briefing, security conference, SOC 2 audit, S-1 filing, pen test, roadshow, IPO day). Some failures delay the IPO.
- The defender does not know what stage an attacker is in. Attackers move through the kill chain in the background against a hidden security posture that the player's actions improve. Successful attacks become incidents that hurt valuation and change the actions available.
- Like Oregon Trail there are characters: analysts the player hires and names, and leadership (CIO, CISO, and Managers).
- The games graphics and sound should be in 8 bit format like Oregon Trail.
- Game play music should be original and have a cyber like techno sound.
- Valuation, corporate trust, brand loyalty, budget, and coffee are tracked as the game progresses.
- Similar to death messages in Oregon Trail there will be humorous messages regarding people leaving, being fired over bad decisions, or burnout.

## Technical Details
- 8 bit graphics and sound.
- Written in Rust.
- Plan for a 640x480 window but can be expanded and scaled.

## Strategy
1. Write plan with success criteria for each phase to be checked off. Include project scaffolding, including .gitignore, and rigorous unit testing.
2. Do not execute the plan until it has been verified.

## Coding Standards

1. Use latest versions of libraries and idiomatic approaches as of today
2. Keep it simple - NEVER over-engineer, ALWAYS simplify, NO unnecessary defensive programming. No extra features - focus on simplicity.
3. Be concise. Keep README minimal and update as changes are made. IMPORTANT: no emojis ever