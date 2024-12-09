.DEFAULT_GOAL := run

.PHONY: check
check: | fmt lint test

.PHONY: test
test:
	cargo test --color=always

.PHONY: lint
lint:
	cargo clippy --color=always 2>&1

.PHONY: fmt
fmt:
	cargo fmt
	find -name '*.md' | xargs --no-run-if-empty prettier --print-width 80 --prose-wrap always --write
	find -name '*.toml' | xargs --no-run-if-empty taplo format

.PHONY: run
run:
	@cargo run --release --quiet -- 1
	@cargo run --release --quiet -- 2
