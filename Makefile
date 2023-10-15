
schema:
	ls ./contracts | xargs -n 1 -t beaker wasm ts-gen

build:
	cargo wasm

build-dev:
	cargo cw-optimizoor

check: build-dev
	ls -d ./artifacts/*.wasm | xargs -I x cosmwasm-check x
