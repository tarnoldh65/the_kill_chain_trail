#!/usr/bin/env bash
set -euo pipefail

# Native build libraries for macroquad, plus headless Chromium and chromedriver for browser playtests.
sudo apt-get update
sudo apt-get install -y --no-install-recommends \
    libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev chromium chromium-driver
sudo chown vscode:vscode /home/vscode/.claude

rustup target add wasm32-unknown-unknown

# Lets Claude Code drive the web build in headless Chromium; software WebGL needs SwiftShader.
cat > /home/vscode/.playwright-mcp.json <<'JSON'
{ "browser": { "launchOptions": { "args": ["--enable-unsafe-swiftshader"] } } }
JSON
claude mcp add --scope user playwright -- npx -y @playwright/mcp@latest \
    --headless --no-sandbox --browser chromium --executable-path /usr/bin/chromium \
    --config /home/vscode/.playwright-mcp.json
