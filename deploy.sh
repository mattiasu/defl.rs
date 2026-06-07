#!/usr/bin/env bash
set -euo pipefail

TARGET=aarch64-unknown-linux-gnu
BINARY=target/$TARGET/release/defl
REMOTE=muhlegard@mupi

echo "==> Building..."
if ! cargo build --release --target $TARGET; then
    echo "ERROR: build failed — nothing deployed"
    exit 1
fi

echo "==> Stopping defl service on mupi..."
ssh $REMOTE 'sudo systemctl stop defl'

echo "==> Copying binary..."
if ! scp $BINARY $REMOTE:~/defl; then
    echo "ERROR: scp failed — service is stopped on mupi, binary not updated"
    echo "      Manually restart with: ssh $REMOTE 'sudo systemctl start defl'"
    exit 1
fi

echo "==> Starting defl service..."
if ! ssh $REMOTE 'sudo systemctl start defl && sudo systemctl is-active defl'; then
    echo "ERROR: service failed to start — check logs with: ssh $REMOTE 'sudo journalctl -u defl -n 50'"
    exit 1
fi

echo "==> Done"
