
schema:
	ls ./contracts | xargs -n 1 -t beaker wasm ts-gen

build:
	cargo wasm

check:
	ls artifacts | grep .wasm | xargs -n 1 -I x -t cosmwasm-check ./artifacts/x
