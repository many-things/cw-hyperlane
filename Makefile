
clean:
	@cargo clean
	@rm -rf ./artifacts

schema:
	ls ./contracts | xargs -n 1 -t beaker wasm ts-gen

ci-build:
	cargo install cw-optimizoor --force
	cargo cw-optimizoor
	rename 's/(.*)(-x86)(.*)/$1$3/d' *
	cd artifacts && zip -r ../wasm_codes.zip * && cd ../

build:
	cargo wasm

build-dev: clean
	cargo cw-optimizoor

check: build-dev
	ls -d ./artifacts/*.wasm | xargs -I x cosmwasm-check x
