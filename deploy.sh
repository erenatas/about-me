#!/usr/bin/env bash
# Refresh the on-disk source for the about-me Stack so the next
# Komodo Deploy uses fresh code.
#
# This script does NOT trigger the deploy itself — Komodo owns the
# build + rollout (run_build=true on the Stack, see Komodo UI →
# Stacks → about-me → Deploy when ready).
#
# Prerequisites:
#   - SSH key at ~/.ssh/id_* trusted by github.com/erenatas
#   - Working tree clean (or `git stash` first)

set -euo pipefail
cd "$(dirname "$0")"

branch=$(git rev-parse --abbrev-ref HEAD)
echo ">> on branch: $branch"
git fetch --prune
git status --short

echo ">> git pull --ff-only"
git pull --ff-only

echo
echo "Source refreshed. Open Komodo UI → Servers → peri-homelab →"
echo "Stacks → about-me → Deploy (or Pull / Deploy) to roll the new"
echo "build. Watch the live log in the UI; cold builds take ~10 min."
