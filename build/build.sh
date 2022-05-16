#! /bin/bash

# Script executed when running the build container

# Cause the script to exit with an error on unitialized variable usage
# or when any command returns an error
set -euo pipefail

# The default gocache location is inside the container's filesystem, not the host's.
# Change the build cache location to a shared location.
# export GOCACHE=$(pwd)/.gobuildcache

# Build the binary to /bin/app
# CGO_ENABLED=0 means build the binary with cgo instead of go
BUILD_NUMBER=${DRONE_BUILD_NUMBER-0}
BUILD_VERSION=$(git rev-parse --short HEAD)
# CGO_ENABLED=0 go build -ldflags "-X main.VersionString=${BUILD_NUMBER}-${BUILD_VERSION}" -installsuffix cgo -o bin/main src/main.go
cargo install --path . 
cargo build --release

ls -la
