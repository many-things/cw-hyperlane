PWD:=$(shell pwd)
BASE:=$(shell basename "$(PWD)")

clean:
	@cargo clean
	@rm -rf ./artifacts

install:
	cargo install --force cosmwasm-check
	rustup target add wasm32-unknown-unknown

schema:
	ls ./contracts | xargs -n 1 -t beaker wasm ts-gen

build:
	cargo build
	cargo wasm
	docker run --rm -v "$(PWD)":/code \
		--mount type=volume,source="$(BASE)_cache",target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/optimizer:0.15.0
	rename --force 's/(.*)-(.*)\.wasm/$$1\.wasm/d' artifacts/*

check: build
	ls -d ./artifacts/*.wasm | xargs -I x cosmwasm-check x

ci-build: check
	zip -jr wasm_codes.zip artifacts
