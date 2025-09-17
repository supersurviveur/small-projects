PKG_NAME := tcp-rust

run: build
	./run.sh

ip:
	sudo ip addr add 192.168.0.1/24 dev tun0
	sudo ip link set up dev tun0

build:
	cargo b --release
	sudo setcap cap_net_admin=eip ./target/release/tcp-rust
