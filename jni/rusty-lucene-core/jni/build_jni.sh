#!/bin/bash
set -e

# Resolve SCRIPT_DIR and PROJECT_DIR correctly
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"
echo "SCRIPT_DIR=$SCRIPT_DIR"
echo "PROJECT_DIR=$PROJECT_DIR"
CPP_DIR="$PROJECT_DIR/jni/rusty-lucene-core/src/main/cpp"
echo "CPP_DIR=$CPP_DIR"
BUILD_DIR="$PROJECT_DIR/jni/rusty-lucene-core/build"
echo "BUILD_DIR=$BUILD_DIR"
NATIVE_DIR="$BUILD_DIR/native"
echo "NATIVE_DIR=$NATIVE_DIR"
RUST_LIB_DIR="$PROJECT_DIR/target/release"
RUST_LIB_NAME="librusty_lucene_core"
LIB_PATH="$RUST_LIB_DIR/$RUST_LIB_NAME.dylib" # macOS

INCLUDE_PATH="/opt/homebrew/opt/openjdk@17/include"

mkdir -p "$NATIVE_DIR"

# Compile JNI glue
CPP_FILE="$CPP_DIR/rusty_lucene_jni.cpp"
echo "CPP_FILE=$CPP_FILE"
# Determine platform-specific flags
OS_NAME=$(uname -s)
if [[ "$OS_NAME" == "Darwin" ]]; then
	# macOS
	LIB_PATH="$RUST_LIB_DIR/librusty_lucene_core.dylib"
	CMD="g++ -dynamiclib -o \"$NATIVE_DIR/rusty_lucene_jni.dylib\" \"$CPP_FILE\" -L \"$RUST_LIB_DIR\" -I \"$INCLUDE_PATH\" -lrusty_lucene_core -lobjc"
elif [[ "$OS_NAME" == "Linux" ]]; then
	# Linux
	LIB_PATH="$RUST_LIB_DIR/librusty_lucene_core.so"
	CMD="g++ -shared -o \"$NATIVE_DIR/rusty_lucene_jni.so\" \"$CPP_FILE\" -L \"$RUST_LIB_DIR\" -I \"$INCLUDE_PATH\" -lrusty_lucene_core"
else
	echo "Unsupported OS: $OS_NAME"
	exit 1
fi

echo "Compiling JNI glue: $CMD"
eval "$CMD"

echo "JNI native library built at: $NATIVE_DIR/rusty_lucene_jni"$([[ "$OS_NAME" == "Darwin" ]] && echo ".dylib" || echo ".so")
