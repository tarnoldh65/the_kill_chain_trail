# The Kill Chain Trail

A cyber security operations game inspired by Oregon Trail, written in Rust.

Run the SOC at a company racing toward its IPO, 26 weeks away. Attackers you cannot see are working through the kill chain the whole time. Reach the IPO with the highest valuation you can; trust, brand loyalty, budget, coffee, and your team's burnout can all end the game early. See [REDESIGN.md](REDESIGN.md) for the full design.

## How to play

- Title screen: 1-3 picks a music track, 4 turns music off, ENTER starts.
- Name your company and yourself, then pick a profile (1-3). Harder profiles multiply your score.
- Vendor Hall: LEFT/RIGHT switch between staff, the six defense categories (three tools each, from cheap to premium), and services; UP/DOWN and ENTER hire and name analysts and buy. Leftover budget pays the payroll every Monday, and every six weeks the board adds funding scaled by its trust in you. Spare after costs is what you can spend without missing a payday. Open it again any day with 6; after day 1, hiring starts a week-long search.
- Day menu: 1 lets the days pass (any key stops the clock), 2 starts an action (deploy a tool, patch, rest, and more), 3 shows the team (UP/DOWN and F let an analyst go for a week's salary in severance), 4 changes the tempo, 5 sends the intern for fancy coffee once a week (arrow keys across four lanes of traffic and back) for a burnout boost, 6 opens the Vendor Hall. Sign up for the coffee subscription ($2K a week) at the Vendor Hall or the break room runs dry.
- Alerts: 1 investigates, 2 calls the IR firm, 3 ignores. Some are false alarms. Incidents, random events, and landmarks ask you to pick a numbered option.
- Finished actions and every choice you make show a result card (ENTER to dismiss) with what visibly changed. Press L at any time during play to read the full log, grouped by day; LEFT/RIGHT turn its pages. Deployed tools lose strength every week until you run their category's maintenance action (an access review, a firewall rule update, and so on). Press D to see the six defenses you are graded on, who each one slows, and what is helping it; the Vendor Hall and action list light up the defenses each choice improves.
- Landmarks on the timeline: forts let the team rest, and the conference job fair hires on the spot; at the security conference you pick who goes and their tracks. Rivers are risky choices, and failures can delay the IPO. More than six weeks of delays and the board pulls it.
- The after-action report shows what the attackers were really doing. The ten best IPO scores appear on the title screen, saved to `top_ten.txt` natively or the browser's local storage.

The window renders at 640x480 and scales when resized. Graphics, sound effects, and techno music are made in code.

## Run

```bash
cargo run --release
```

## Run in a browser

Play online at https://lehmanrd.github.io/the_kill_chain_trail/ (deployed from `main` by GitHub Actions), or build and serve locally:

```bash
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/the_kill_chain_trail.wasm web/
python3 -m http.server -d web 8000
```

Open http://localhost:8000. Sound starts after the first key press. Requires `rustup target add wasm32-unknown-unknown`.

## Test

```bash
cargo test
```

## Dev Container

Open the folder in VS Code and choose "Reopen in Container", then use the browser commands above; port 8000 is forwarded to the host. The container includes Claude Code, headless Chromium with chromedriver, and a Playwright MCP server so agents can playtest the web build. Claude Code settings persist in a Docker volume.

See [PLAYTEST.md](PLAYTEST.md) for the manual playtest checklist.
