#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

bun_version=$(sed -n '/"packageManager"/,/}/s/.*"version": *"\([^"]*\)".*/\1/p' package.json)
export BUN_INSTALL="${BUN_INSTALL:-$HOME/.bun}"
export PATH="$BUN_INSTALL/bin:$PATH"

if [ "$(bun --version 2>/dev/null)" != "$bun_version" ]; then
	curl -fsSL https://bun.sh/install | bash -s "bun-v$bun_version"
fi

export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export PATH="$CARGO_HOME/bin:$PATH"
if ! command -v rustup > /dev/null; then
	curl -fsSL https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain none
fi
rustup toolchain install

bun install --frozen-lockfile
bun vp run ci
