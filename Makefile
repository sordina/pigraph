
all: target/wasm32-unknown-unknown/release/web.wasm

target/wasm32-unknown-unknown/release/web.wasm:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: dependencies
dependencies:
	rustup update
	rustup target add wasm32-unknown-unknown
	cargo install --git https://github.com/alexcrichton/wasm-gc

.PHONY: server
server:
	srvdir --port 9999
