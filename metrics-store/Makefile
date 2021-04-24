# check wether compiled with wasm-pack
ifeq ($(shell test -e ./metrics-collector/pkg/metrics_collector_bg.wasm && echo -n y),y)
	METRICS_COLLECTOR_WASM=./metrics_collector/pkg/metrics_collector_bg.wasm
endif
ifeq ($(shell test -e ./singleton-queue/pkg/singleton_queue_bg.wasm && echo -n y),y)
	SINGLETON_QUEUE_WASM=./singleton-queue/pkg/singleton_queue_bg.wasm
endif

# a small optimized binary without debug info, useful for releases
build: clean
	cd singleton-queue && wasm-pack build --release
	cd metrics-collector && wasm-pack build --release

build-unoptimized: clean
	cd metrics-collector; \
	cargo +nightly build --target=wasm32-unknown-unknown --release;
	cd singleton-queue; \
	cargo +nightly build --target=wasm32-unknown-unknown --release;

deploy:
	METRICS_COLLECTOR_WASM=$(METRICS_COLLECTOR_WASM) \
	SINGLETON_QUEUE_WASM=$(SINGLETON_QUEUE_WASM) \
	docker-compose up --build --remove-orphans;

# shows only the logs related to WASM filter/singleton
deploy-filtered:
	METRICS_COLLECTOR_WASM=$(METRICS_COLLECTOR_WASM) \
	SINGLETON_QUEUE_WASM=$(SINGLETON_QUEUE_WASM) \
	docker-compose up --build --remove-orphans | grep "\[wasm\]\|Starting";

run: build deploy

run-filtered: build deploy-filtered

clean:
	cd singleton-queue && cargo clean && rm -rf ./pkg
	cd metrics-collector && cargo clean && rm -rf ./pkg
