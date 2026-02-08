#!/bin/bash
# verification/check_project_setup.sh

if [ ! -f "Cargo.toml" ]; then
  echo "FAIL: Cargo.toml not found."
  exit 1
fi

if ! grep -q "tokio" Cargo.toml; then
  echo "FAIL: tokio dependency not found."
  exit 1
fi

if ! grep -q "reqwest" Cargo.toml; then
  echo "FAIL: reqwest dependency not found."
  exit 1
fi

if ! grep -q "serde" Cargo.toml; then
  echo "FAIL: serde dependency not found."
  exit 1
fi

echo "PASS: Project structure and dependencies verified."
exit 0
