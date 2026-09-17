# Makefile for Rust Lucene Scaffolding
# Default target: help
# Available targets: help, references, build, test, run, clean, jni-build, jni-test, jni-clean

.DEFAULT_GOAL ?= help

help:
	@echo "Available targets:"
	@echo "  help         - Show this help message"
	@echo "  references   - Clone Apache Lucene repository to references/"
	@echo "  build        - Run cargo build for Rust workspace"
	@echo "  test         - Run cargo test for Rust workspace"
	@echo "  run          - Run cargo run"
	@echo "  clean        - Run cargo clean"
	@echo "  jni-build    - Build JNI wrapper java project"
	@echo "  jni-test     - Run JNI wrapper tests"
	@echo "  jni-clean    - Clean JNI wrapper directory"

references:
	@echo "Cloning Apache Lucene repository..."
	git clone --depth 1 https://github.com/apache/lucene.git references
	@echo "Lucene repository cloned to references/"

build:
	cargo build --workspace

test:
	cargo test --workspace

run:
	cargo run

clean:
	@if [ -d references/ ]; then \
	  rm -rf references/; \
	  echo "Cleaned references/"; \
	else \
	  echo "No references/ directory to clean"; \
	fi
	cargo clean

jni-build:
	@echo "Building JNI wrapper..."
	cd jni/rusty-lucene-core && ./gradlew nativeCompile

jni-test: jni-build
	@echo "Running JNI wrapper tests..."
	cd jni/rusty-lucene-core && ./gradlew nativeTest

jni-clean:
	@echo "Cleaning JNI wrapper..."
	rm -rf jni/rusty-lucene-core/build
	rm -f jni/rusty-lucene-core/src/main/cpp/rusty_lucene_jni.dylib 2>/dev/null || true

clean-jni: jni-clean
	# Alias for jni-clean
	@echo "JNI wrapper cleaned."

# Convenience target: build + test everything
all: build jni-build

check: build jni-build jni-test
	@echo "✓ All checks passed"
