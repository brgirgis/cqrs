
all:
	make clean
	make build
	make doc
	make test

clean:
	rm -rf target
	rm -rf Cargo.lock
	rm -rf test.db*

build:
	cargo build

test:
	cargo test

doc:
	cargo doc --lib --no-deps --all-features

deploy:
	cargo publish --token ${CRATES_IO_TOKEN}

dry_deploy:
	cargo publish --dry-run --allow-dirty

check_fmt:
	cargo +nightly fmt --all -- --check
