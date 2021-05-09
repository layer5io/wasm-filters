# check wether compiled with wasm-pack
ifeq ($(shell test -e ./pkg/tcp_metrics_bg.wasm && echo -n y),y)
	WASM_PATH=./pkg/tcp_metrics_bg.wasm
endif

# a small optimized binary without debug info, useful for releases
build: clean
	wasm-pack build --release

build-unoptimized: clean
	cargo +nightly build --target=wasm32-unknown-unknown --release 

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
