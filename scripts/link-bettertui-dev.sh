#!/usr/bin/env bash

set -e

LINK_REACT=false
LINK_SOLID=false
TARGET_ROOT=""

while [[ $# -gt 0 ]]; do
  case $1 in
  --react)
    LINK_REACT=true
    shift
    ;;
  --solid)
    LINK_SOLID=true
    shift
    ;;
  *)
    if [ -z "$TARGET_ROOT" ]; then
      TARGET_ROOT="$1"
    else
      echo "Error: Unexpected argument '$1'"
      exit 1
    fi
    shift
    ;;
  esac
done

if [ -z "$TARGET_ROOT" ]; then
  echo "Usage: $0 <target-project-root> [--react] [--solid]"
  echo "Example: $0 /path/to/your/project"
  echo "Example: $0 /path/to/your/project --react"
  echo ""
  echo "This script links BetterTUI dev packages (@bettertui/core, @bettertui/shared, etc.)"
  echo "into an external target project's node_modules for local development."
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BETTERTUI_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
NODE_MODULES_DIR="$TARGET_ROOT/node_modules"

if [ ! -d "$TARGET_ROOT" ]; then
  echo "Error: Target project root directory does not exist: $TARGET_ROOT"
  exit 1
fi

if [ ! -d "$NODE_MODULES_DIR" ]; then
  echo "Error: node_modules directory does not exist in target: $NODE_MODULES_DIR"
  echo "Please run package manager install in the target project first."
  exit 1
fi

BETTERTUI_TARGET_DIR="$NODE_MODULES_DIR/@bettertui"
mkdir -p "$BETTERTUI_TARGET_DIR"

link_package() {
  local pkg_name="$1"
  local src_dir="$2"
  local dest_dir="$BETTERTUI_TARGET_DIR/$pkg_name"

  if [ -e "$dest_dir" ] || [ -L "$dest_dir" ]; then
    rm -rf "$dest_dir"
  fi

  ln -s "$src_dir" "$dest_dir"
  echo "  ✓ Linked @bettertui/$pkg_name -> $dest_dir"
}

echo "Linking BetterTUI dev packages from: $BETTERTUI_ROOT"
echo "To target project: $TARGET_ROOT"
echo ""

echo "Linking @bettertui/shared..."
link_package "shared" "$BETTERTUI_ROOT/packages/shared"

echo "Linking @bettertui/core..."
link_package "core" "$BETTERTUI_ROOT/packages/core"

if [ "$LINK_REACT" = true ]; then
  echo "Linking @bettertui/react..."
  link_package "react" "$BETTERTUI_ROOT/packages/react"
fi

if [ "$LINK_SOLID" = true ]; then
  echo "Linking @bettertui/solid..."
  link_package "solid" "$BETTERTUI_ROOT/packages/solid"
fi

echo ""
echo "✓ BetterTUI development linking complete!"
