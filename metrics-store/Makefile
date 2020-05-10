build:
	cd singleton-queue && cargo +nightly build --target=wasm32-unknown-unknown --release 
	cd metrics-collector && cargo +nightly build --target=wasm32-unknown-unknown --release 

deploy:
	docker-compose up --build --remove-orphans

# shows only the logs related to WASM filter/singleton
deploy-filtered:
	docker-compose up --build --remove-orphans | grep "\[wasm\]\|Starting"

run: build deploy

run-filtered: build deploy-filtered

clean:
	cd singleton-queue && cargo clean
	cd metrics-collector && cargo clean