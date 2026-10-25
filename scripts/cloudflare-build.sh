#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

bun_version=$(sed -n '/"packageManager"/,/}/s/.*"version": *"\([^"]*\)".*/\1/p' package.json)
export BUN_INSTALL="${BUN_INSTALL:-$HOME/.bun}"
export PATH="$BUN_INSTALL/bin:$PATH"

if [ "$(bun --version 2>/dev/null)" != "$bun_version" ]; then
	curl -fsSL https://bun.sh/install | bash -s "bun-v$bun_version"
fi

bun install --frozen-lockfile
bun vp run ci
