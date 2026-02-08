#!/bin/bash
# verification/check_web_server_deps.sh

if ! grep -q "axum" Cargo.toml; then
  echo "FAIL: axum dependency not found."
  exit 1
fi

if ! grep -q "tower" Cargo.toml; then
  echo "FAIL: tower dependency not found."
  exit 1
fi

if ! grep -q "tower-http" Cargo.toml; then
  echo "FAIL: tower-http dependency not found."
  exit 1
fi

echo "PASS: Web server dependencies verified."
exit 0
