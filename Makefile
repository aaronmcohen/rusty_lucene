# Makefile for Rust Lucene Scaffolding
# Default target: help
# Available targets: help, references, build, test, run, clean

.DEFAULT_GOAL ?= help

help:
	@echo "Available targets:"
	@echo "  help     - Show this help message"
	@echo "  references - Clone Apache Lucene repository to references/"
	@echo "  build    - Run cargo build"
	@echo "  test     - Run cargo test"
	@echo "  run      - Run cargo run"
	@echo "  clean    - Run cargo clean"

references:
	@echo "Cloning Apache Lucene repository..."
	git clone --depth 1 https://github.com/apache/lucene.git references
	@echo "Lucene repository cloned to references/"

build:
	cargo build

test:
	cargo test

run:
	cargo run

clean:
	cargo clean
