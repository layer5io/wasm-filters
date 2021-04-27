# gets the rust nightly toolchain and support for wasm compilation
rust-toolchain:
	rustup toolchain install nightly
	rustup target add wasm32-unknown-unknown
	cargo install wasm-pack
