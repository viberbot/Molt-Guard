#!/bin/bash
# verification/check_docker_config.sh

if ! grep -q "3005" Dockerfile; then
  echo "FAIL: Dockerfile does not expose port 3005."
  exit 1
fi

if ! grep -q "3005:3005" docker-compose.yml; then
  echo "FAIL: docker-compose.yml does not map port 3005."
  exit 1
fi

if ! grep -q "VALIDATION_MODE" docker-compose.yml; then
  echo "FAIL: docker-compose.yml does not set VALIDATION_MODE."
  exit 1
fi

echo "PASS: Docker configuration verified."
exit 0
