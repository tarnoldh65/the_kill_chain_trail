# The Kill Chain Trail

A cyber security operations game inspired by Oregon Trail, written in Rust.

Lead a SOC team through the seven landmarks of the cyber kill chain (Reconnaissance, Weaponization, Delivery, Exploitation, Installation, Command and Control, Actions on Objectives), spending budget, coffee, and your team's sleep to contain the attacker before trust, brand loyalty, or your analysts run out.

Pick a music track (1-3) or turn music off (4) on the title screen. Enter your company and name, then press 1-3 to choose at each landmark; an illustrated incident report shows what happened before the next stage. The window renders at 640x480 and scales when resized. Sound effects and techno music are synthesized in code; see [PLAN.md](PLAN.md).

## Run

```bash
cargo run --release
```

## Test

```bash
cargo test
```

See [PLAYTEST.md](PLAYTEST.md) for the manual playtest checklist.

## Run in a browser

The game also builds to WebAssembly and can be served as a static site (for example on Unraid via Docker):

```bash
docker compose up -d
```

Then open `http://localhost:8080`. The `web/` directory holds the static HTML and macroquad JS loader; `Dockerfile` compiles the `wasm32-unknown-unknown` target and serves it with nginx.
