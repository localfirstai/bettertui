#!/usr/bin/env bash
set -eo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EXAMPLES_DIR="$ROOT/examples"

EXAMPLES=(
  "fundamentals/hello-world"
  "fundamentals/counter"
  "fundamentals/forms"
  "fundamentals/layouts"
  "fundamentals/tables"
  "fundamentals/terminal"
  "fundamentals/tree"
  "showcase/capability-inspector"
  "showcase/dashboard"
  "showcase/markdown-viewer"
  "showcase/performance-lab"
  "showcase/system-monitor"
  "showcase/terminal-showcase"
  "showcase/widget-gallery"
)

SEP="---------------------------------------------"

echo ""
echo "$SEP"
echo "  Building all examples"
echo "$SEP"
echo ""

for example in "${EXAMPLES[@]}"; do
  dir="$EXAMPLES_DIR/$example"
  name="$(basename "$example")"
  echo -n "  $name ... "
  if (cd "$dir" && pnpm build > /dev/null 2>&1); then
    echo "ok"
  else
    echo "FAIL"
  fi
done

echo ""
echo "$SEP"
echo "  Running examples (2s each)"
echo "$SEP"

for example in "${EXAMPLES[@]}"; do
  dir="$EXAMPLES_DIR/$example"
  name="$(basename "$example")"

  echo ""
  echo "  +--------------------------------------+"
  echo "  |  $name"
  echo "  +--------------------------------------+"
  echo ""

  (cd "$dir" && node dist/index.mjs 2>&1) &
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
