
build:
	cargo build

test:
	cargo test

doc:
	cargo doc --lib --no-deps

deploy:
	cargo publish --token ${CRATES_IO_TOKEN}

check:
	cargo publish --dry-run --allow-dirty
