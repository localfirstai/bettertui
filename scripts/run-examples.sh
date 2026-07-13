#!/usr/bin/env bash
set -eo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EXAMPLES_DIR="$ROOT/examples"

EXAMPLES=(
  "rendering-engine"
  "animation-basics"
  "live-metrics"
  "performance-stress-test"
  "scroll-area-basics"
  "tabs-navigation"
  "theming"
  "tree-view"
)

SEP="---------------------------------------------"

echo ""
echo "$SEP"
echo "  Building @bettertui/examples"
echo "$SEP"
echo ""

( cd "$EXAMPLES_DIR" && pnpm build ) || {
  echo "Build failed."
  exit 1
}

echo ""
echo "$SEP"
echo "  Running examples (2s each)"
echo "$SEP"

for example in "${EXAMPLES[@]}"; do
  echo ""
  echo "  +--------------------------------------+"
  echo "  |  $example"
  echo "  +--------------------------------------+"
  echo ""

  ( node "$EXAMPLES_DIR/dist/index.mjs" "$example" 2>&1 ) &
  pid=$!
  sleep 2
  kill "$pid" 2>/dev/null || true
  wait "$pid" 2>/dev/null || true
done

echo ""
echo "$SEP"
echo "  All examples done."
echo "$SEP"
echo ""
