## Business Requirements

- An MVP computer game which will simulated a security operations team dealing with a series of cyber attacks and is designed in the same realm as Oregon Trail.
- The opening screen displays the game name "The Kill Chain Trail".
- The structure maps to gameplay, with each "landmark" being a stage like reconnaissance, initial access, lateral movement, and exfiltration, where the team makes resource decisions about analysts, budget, coffee, and sleep.
- Like Oregon Trail there will be several added characters such as analysts, leadership (CIO, CISO, and Managers).
- At the beginning of the game the play must name the company which is being protected and their name as an incident lead.
- The games graphics and sound should be in 8 bit format like Oregon Trail.
- Game play music should be original and have a cyber like techno sound.
- As the game progresses the health of corporate trust, brand loyalty, and resources will be track.
- Similar to death messages in Oregon trail there will be messages regarding people leaving, being fired over bad decisions or burnout.

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