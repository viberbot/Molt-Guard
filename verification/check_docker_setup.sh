#!/bin/bash
# verification/check_docker_setup.sh

if [ ! -f "Dockerfile" ]; then
  echo "FAIL: Dockerfile not found."
  exit 1
fi

if ! grep -q "gcr.io/distroless/cc-debian12" Dockerfile; then
  echo "FAIL: Dockerfile does not use Google Distroless (cc-debian12)."
  exit 1
fi

if [ ! -f "docker-compose.yml" ]; then
  echo "FAIL: docker-compose.yml not found."
  exit 1
fi

if ! grep -q "molt-bot:" docker-compose.yml; then
  echo "FAIL: docker-compose.yml does not define molt-bot service."
  exit 1
fi

if ! grep -q "vault:" docker-compose.yml; then
  echo "FAIL: docker-compose.yml does not define vault service."
  exit 1
fi

echo "PASS: Docker environment verified."
exit 0
