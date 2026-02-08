#!/bin/bash
# verification/check_docker_hardening.sh

if ! grep -q "USER nonroot" Dockerfile; then
  echo "FAIL: Dockerfile does not specify a nonroot user."
  exit 1
fi

if ! grep -q "read_only: true" docker-compose.yml; then
  echo "FAIL: docker-compose.yml does not specify read_only: true for molt-bot."
  exit 1
fi

if ! grep -q "security_opt:" docker-compose.yml; then
  echo "FAIL: docker-compose.yml does not specify security_opt."
  exit 1
fi

echo "PASS: Docker hardening verified."
exit 0
