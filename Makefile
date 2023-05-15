# This is an auto documented Makefile. For more information see the following article
# @see http://marmelab.com/blog/2016/02/29/auto-documented-makefile.html


.DEFAULT_GOAL := help

.PHONY: help
help:
	@echo "❓ Use \`make <target>' where <target> can be"
	@grep -E '^\.PHONY: [a-zA-Z0-9_-]+ .*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = "(: |##)"}; {printf "\033[36m%-30s\033[0m %s\n", $$2, $$3}'

.PHONY: setup ## Install needed dependencies
setup:
	cargo install cargo-tarpaulin
	rustup component add clippy
	rustup component add rustfmt
	cargo install cargo-audit

.PHONY: dev ## Launch the Development loop ( check / test / run)
dev:
	cargo watch -x check -x test -x run

.PHONY: test ## Run Cargo test
test:
	cargo test
	
.PHONY: format ## Format and Lint Code
format:
	rustup component add rustfmt
	rustup component add clippy
	cargo fmt --check
	cargo clippy -- -D warnings

.PHONY: coverage ## Code Coverage
coverage:
	cargo install cargo-tarpaulin
	cargo tarpaulin --ignore-tests

.PHONY: security ## Security Audit
security:
	cargo install cargo-audit
	cargo audit

