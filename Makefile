clean:
	@cargo clean
	@rm -rf ./artifacts

install:
	cargo install cw-optimizoor cosmwasm-check beaker
  rustup target add wasm32-unknown-unknown

schema:
	ls ./contracts | xargs -n 1 -t beaker wasm ts-gen

build:
	cargo build
	cargo wasm
	cargo cw-optimizoor
	rename --force 's/(.*)-(.*)\.wasm/$$1\.wasm/d' artifacts/*

check: build
	ls -d ./artifacts/*.wasm | xargs -I x cosmwasm-check x

ci-build: check
	zip -jr wasm_codes.zip artifacts
