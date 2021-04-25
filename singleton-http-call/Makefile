# check wether compiled with wasm-pack
ifeq ($(shell test -e ./pkg/singleton_http_call_bg.wasm && echo -n y),y)
	WASM_PATH=./pkg/singleton_http_call_bg.wasm
endif

# a small optimized binary without debug info, useful for releases
build: clean
	wasm-pack build --release

build-unoptimized: clean
	cargo +nightly build --target=wasm32-unknown-unknown --release 

# read more about building .wasm binaries here:
# https://rustwasm.github.io/docs/wasm-pack/commands/build.html

deploy:
	WASM_PATH=$(WASM_PATH) docker-compose up --build --remove-orphans

# shows only the logs related to WASM filter/singleton
deploy-filtered:
	WASM_PATH=$(WASM_PATH) docker-compose up --build --remove-orphans | grep "\[wasm\]\|Starting"

run: build deploy

run-filtered: build deploy-filtered

clean:
	cargo clean
	rm -rf ./pkg
