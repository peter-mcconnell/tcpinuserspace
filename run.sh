#!/usr/bin/env sh

set -e

echo "tcp-izzles"

pkill -9 tcpinuserspace 2> /dev/null || true
cargo b --release
sudo setcap CAP_NET_ADMIN=eip $CARGO_TARGET_DIR/release/tcpinuserspace
$CARGO_TARGET_DIR/release/tcpinuserspace &
pid=$!
trap "kill -9 $!" TERM
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set dev tun0 up
wait $pid
