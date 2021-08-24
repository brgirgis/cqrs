
SUB_LIBS = cqrs-es2 cqrs-es2-store tokio-cqrs-es2-store

all:
	make clean
	make build
	make doc

clean:
	rm -rf target
	rm -rf Cargo.lock
	rm -rf test.db*

build:
	cargo build

up:
	docker-compose up -d
	rm -rf test.db*

down:
	docker-compose down

test:
	cargo test

doc:
	cargo doc --lib --no-deps --all-features

deploy:
	cargo publish --token ${CRATES_IO_TOKEN}

dry_deploy:
	for dir in $(SUB_LIBS); do \
    (cd $$dir; cargo publish --dry-run --allow-dirty); \
  done

check_fmt:
	cargo fmt --all -- --check
