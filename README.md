# The Kill Chain Trail

A cyber security operations game inspired by Oregon Trail, written in Rust.

Run the SOC at a company racing toward its IPO, 26 weeks away. Keep corporate trust, brand loyalty, the valuation, the budget, and your team's sanity intact until the bell rings. The game is mid-redesign; see [REDESIGN.md](REDESIGN.md) and [PLAN.md](PLAN.md).

Pick a music track (1-3) or turn music off (4) on the title screen, then name your company and yourself and pick a profile (Fintech, Healthtech, or Gaming startup). At the Vendor Hall, use the up and down arrows and ENTER to hire and name analysts and buy tools, services, and coffee; leftover budget pays the weekly payroll. From the day menu, press 1 to let the days pass (any key stops the clock), 2 to take an action such as deploying a tool, a patch sprint, recruiting, or a day off (days pass while the SOC works on it), 3 to check the team, 4 to change the tempo (Relaxed, Steady, Crunch), or 5 once a week to send the intern for coffee ($10): use the arrow keys to cross four lanes of traffic to the coffee shop door and back to the office door. Payroll comes out every Monday. Attackers work in the background where you cannot see them; press 1-3 to investigate, call the IR firm, or ignore an alert (some are false alarms), and pick a response when an incident hits. Random events (the flu, zero-days, a broken coffee machine) interrupt the days too, and some ask you to choose. The after-action report at the end shows what the attackers were really doing. The window renders at 640x480 and scales when resized. Sound effects and techno music are synthesized in code.

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
