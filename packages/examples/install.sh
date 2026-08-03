#!/bin/sh
set -e

# BetterTUI Examples — local setup script
# Bootstraps the vanilla TypeScript examples package inside this pnpm workspace.
# (The examples are source run via Node, not a downloaded binary.)

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

MIN_NODE_MAJOR=24

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$SCRIPT_DIR/../../.." && pwd)

echo "${GREEN}BetterTUI Examples — setup${NC}"
echo "Repo root: $REPO_ROOT"
echo ""

# --- Prerequisite checks --------------------------------------------------

if ! command -v node >/dev/null 2>&1; then
  printf "${RED}Error: Node.js is not installed.${NC}\n"
  echo "Install Node.js >= $MIN_NODE_MAJOR (see https://nodejs.org)."
  exit 1
fi

NODE_MAJOR=$(node -v | sed 's/^v//' | cut -d. -f1)
if [ "$NODE_MAJOR" -lt "$MIN_NODE_MAJOR" ]; then
  printf "${RED}Error: Node.js v$NODE_MAJOR detected, but >= v$MIN_NODE_MAJOR is required.${NC}\n"
  exit 1
fi

if ! command -v pnpm >/dev/null 2>&1; then
  printf "${RED}Error: pnpm is not installed.${NC}\n"
  echo "This repo is pinned to pnpm@9.15.0 — install it with: corepack enable && corepack prepare pnpm@9.15.0 --activate"
  exit 1
fi

echo "${BLUE}Prerequisites OK${NC} (node v$(node -v), pnpm $(pnpm -v))"

# --- Install --------------------------------------------------------------

echo ""
echo "Installing workspace dependencies (root)..."
cd "$REPO_ROOT"
pnpm install

# --- Build native addon ---------------------------------------------------

# @bettertui/core needs its Rust engine addon built before any native call runs.
echo ""
printf "${YELLOW}Note: @bettertui/core requires the native engine addon.${NC}\n"
printf "Build it with: ${BLUE}pnpm --filter @bettertui/core build:native${NC}\n"
if [ -n "$BUILD_NATIVE" ]; then
  echo "BUILD_NATIVE set — building now..."
  pnpm --filter @bettertui/core build:native
fi

# --- Done -----------------------------------------------------------------

printf "${GREEN}✓ BetterTUI examples are set up.${NC}\n"
echo ""
printf "${BLUE}Run the example launcher with:${NC}\n"
echo "  pnpm --filter @bettertui/examples dev"
echo ""
echo "Or from this directory:"
echo "  pnpm dev"
