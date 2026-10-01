# The Kill Chain Trail

A cyber security operations game inspired by Oregon Trail, written in Rust.

Lead a SOC team through the seven landmarks of the cyber kill chain (Reconnaissance, Weaponization, Delivery, Exploitation, Installation, Command and Control, Actions on Objectives), spending budget, coffee, and your team's sleep to contain the attacker before trust, brand loyalty, or your analysts run out.

Pick a music track (1-3) or turn music off (4) on the title screen. Enter your company and name, then press 1-3 to pick one of three choices drawn at random from six for each landmark (a random event also strikes each turn); an illustrated incident report shows what happened before the next stage. Once per stage, unless the 36-pot break room is full, press 4 to send the intern for coffee ($10): use the arrow keys to cross four lanes of traffic to the coffee shop door and back to the office door. The window renders at 640x480 and scales when resized. Sound effects and techno music are synthesized in code; see [PLAN.md](PLAN.md).

## Run

```bash
cargo run --release
```

## Run in a browser

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
