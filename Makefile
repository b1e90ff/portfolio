.DEFAULT_GOAL := help

CSS_IN  := styles/main.css
CSS_OUT := public/css/main.css
TAILWIND := scripts/tailwind.sh

.PHONY: help css css-watch run dev build release fmt clippy test test-js e2e check audit clean

help:
	@printf "Targets:\n"
	@printf "  css          Build CSS once (minified)\n"
	@printf "  css-watch    Rebuild CSS on change\n"
	@printf "  run          cargo run (debug)\n"
	@printf "  dev          Watch CSS + run server (requires cargo-watch)\n"
	@printf "  build        cargo build --release\n"
	@printf "  fmt          cargo fmt --all\n"
	@printf "  clippy       cargo clippy --all-targets -- -D warnings\n"
	@printf "  test         cargo test --all-features\n"
	@printf "  test-js      Frontend unit tests (node --test)\n"
	@printf "  e2e          Browser smoke tests (Playwright)\n"
	@printf "  check        fmt --check + clippy + test + test-js\n"
	@printf "  audit        cargo audit\n"
	@printf "  clean        cargo clean + remove built CSS\n"

css:
	@mkdir -p $(dir $(CSS_OUT))
	$(TAILWIND) -i $(CSS_IN) -o $(CSS_OUT) --minify

css-watch:
	@mkdir -p $(dir $(CSS_OUT))
	$(TAILWIND) -i $(CSS_IN) -o $(CSS_OUT) --watch

run: css
	cargo run

dev:
	$(TAILWIND) -i $(CSS_IN) -o $(CSS_OUT) --watch &
	cargo watch -x run

build: css
	cargo build --release

release: build

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --all-features

test-js:
	node --import ./tests/js/register.mjs --test 'tests/js/*.test.mjs'

e2e: css
	npx playwright test

check:
	cargo fmt --all --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test --all-features
	$(MAKE) test-js

audit:
	cargo audit --deny warnings

clean:
	cargo clean
	rm -rf $(CSS_OUT) bin/tailwindcss*
