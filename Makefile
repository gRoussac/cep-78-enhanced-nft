PINNED_TOOLCHAIN := $(shell cat rust-toolchain)

prepare:
	rustup target add wasm32-unknown-unknown
	rustup component add clippy --toolchain ${PINNED_TOOLCHAIN}
	rustup component add rustfmt --toolchain ${PINNED_TOOLCHAIN}
	rustup component add rust-src --toolchain ${PINNED_TOOLCHAIN}

build-contract:
	cd contract && RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort
	cd client/mint_session && RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort
	cd client/balance_of_session && RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort
	cd client/owner_of_session && RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort
	cd client/get_approved_session && RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort
	cd client/is_approved_for_all_session && RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort
	cd client/transfer_session && RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort
	cd client/updated_receipts && RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort
	cd test-contracts/minting_contract && RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort
	cd test-contracts/mangle_named_keys && RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort
	cd test-contracts/transfer_filter_contract && RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort
	wasm-strip contract/target/wasm32-unknown-unknown/release/contract.wasm
	wasm-strip client/mint_session/target/wasm32-unknown-unknown/release/mint_call.wasm
	wasm-strip client/balance_of_session/target/wasm32-unknown-unknown/release/balance_of_call.wasm
	wasm-strip client/owner_of_session/target/wasm32-unknown-unknown/release/owner_of_call.wasm
	wasm-strip client/get_approved_session/target/wasm32-unknown-unknown/release/get_approved_call.wasm
	wasm-strip client/is_approved_for_all_session/target/wasm32-unknown-unknown/release/is_approved_for_all_call.wasm
	wasm-strip client/transfer_session/target/wasm32-unknown-unknown/release/transfer_call.wasm
	wasm-strip client/updated_receipts/target/wasm32-unknown-unknown/release/updated_receipts.wasm
	wasm-strip test-contracts/minting_contract/target/wasm32-unknown-unknown/release/minting_contract.wasm
	wasm-strip test-contracts/transfer_filter_contract/target/wasm32-unknown-unknown/release/transfer_filter_contract.wasm

VERSIONS := 1_0_0 1_1_0 1_2_0 1_3_0 1_4_0 1_5_0

setup-test: build-contract
	mkdir -p tests/wasm
	$(foreach version,$(VERSIONS), \
		if [ ! -d "tests/wasm/$(version)" ]; then \
			mkdir -p tests/wasm/$(version); \
			curl -L https://github.com/casper-ecosystem/cep-78-enhanced-nft/releases/download/v$(subst _,.,$(version))/cep-78-wasm.tar.gz | tar zxv -C tests/wasm/$(version)/; \
		fi; \
	)

	cp contract/target/wasm32-unknown-unknown/release/contract.wasm tests/wasm
	cp client/mint_session/target/wasm32-unknown-unknown/release/mint_call.wasm tests/wasm
	cp client/balance_of_session/target/wasm32-unknown-unknown/release/balance_of_call.wasm tests/wasm
	cp client/owner_of_session/target/wasm32-unknown-unknown/release/owner_of_call.wasm tests/wasm
	cp client/get_approved_session/target/wasm32-unknown-unknown/release/get_approved_call.wasm tests/wasm
	cp client/is_approved_for_all_session/target/wasm32-unknown-unknown/release/is_approved_for_all_call.wasm tests/wasm
	cp client/transfer_session/target/wasm32-unknown-unknown/release/transfer_call.wasm tests/wasm
	cp client/updated_receipts/target/wasm32-unknown-unknown/release/updated_receipts.wasm tests/wasm
	cp test-contracts/minting_contract/target/wasm32-unknown-unknown/release/minting_contract.wasm tests/wasm
	cp test-contracts/mangle_named_keys/target/wasm32-unknown-unknown/release/mangle_named_keys.wasm tests/wasm
	cp test-contracts/transfer_filter_contract/target/wasm32-unknown-unknown/release/transfer_filter_contract.wasm tests/wasm

test: setup-test
	cd tests && cargo test

clippy:
	cd contract && cargo clippy --target wasm32-unknown-unknown --bins -- -D warnings
	cd contract && cargo clippy --no-default-features --lib -- -D warnings
	cd client/mint_session && cargo clippy --release --target wasm32-unknown-unknown -- -D warnings
	cd client/balance_of_session && cargo clippy --release --target wasm32-unknown-unknown -- -D warnings
	cd client/owner_of_session && cargo clippy --release --target wasm32-unknown-unknown -- -D warnings
	cd client/get_approved_session && cargo clippy --release --target wasm32-unknown-unknown -- -D warnings
	cd client/transfer_session && cargo clippy --release --target wasm32-unknown-unknown -- -D warnings
	cd client/updated_receipts && cargo clippy --release --target wasm32-unknown-unknown -- -D warnings
	cd test-contracts/minting_contract && cargo clippy --release --target wasm32-unknown-unknown -- -D warnings
	cd test-contracts/mangle_named_keys && cargo clippy --release --target wasm32-unknown-unknown -- -D warnings
	cd test-contracts/transfer_filter_contract && cargo clippy --release --target wasm32-unknown-unknown -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contract && cargo fmt -- --check
	cd client/mint_session && cargo fmt -- --check
	cd client/balance_of_session && cargo fmt -- --check
	cd client/owner_of_session && cargo fmt -- --check
	cd client/get_approved_session && cargo fmt -- --check
	cd client/transfer_session && cargo fmt -- --check
	cd client/updated_receipts && cargo fmt -- --check
	cd test-contracts/minting_contract && cargo fmt -- --check
	cd test-contracts/mangle_named_keys && cargo fmt -- --check
	cd test-contracts/transfer_filter_contract && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy fmt

format:
	cd contract && cargo fmt
	cd client/mint_session && cargo fmt
	cd client/balance_of_session && cargo fmt
	cd client/owner_of_session && cargo fmt
	cd client/get_approved_session && cargo fmt
	cd client/transfer_session && cargo fmt
	cd client/updated_receipts && cargo fmt
	cd test-contracts/minting_contract && cargo fmt
	cd test-contracts/mangle_named_keys && cargo fmt
	cd test-contracts/transfer_filter_contract && cargo fmt
	cd tests && cargo fmt

clean:
	cd contract && cargo clean
	cd client/mint_session && cargo clean
	cd client/balance_of_session && cargo clean
	cd client/owner_of_session && cargo clean
	cd client/get_approved_session && cargo clean
	cd client/transfer_session && cargo clean
	cd client/updated_receipts && cargo clean
	cd test-contracts/minting_contract && cargo clean
	cd test-contracts/mangle_named_keys && cargo clean
	cd test-contracts/transfer_filter_contract && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
	rm -rf ./*/Cargo.lock

cargo-update:
	cd contract && cargo update
	cd client/mint_session && cargo update
	cd client/balance_of_session && cargo update
	cd client/owner_of_session && cargo update
	cd client/get_approved_session && cargo update
	cd client/transfer_session && cargo update
	cd client/updated_receipts && cargo update
	cd test-contracts/minting_contract && cargo update
	cd test-contracts/mangle_named_keys && cargo update
	cd test-contracts/transfer_filter_contract && cargo update
	cd tests && cargo update