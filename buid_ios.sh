#!/bin/bash

# 设置变量
FRAMEWORK_NAME="RustVad"
FRAMEWORK_PATH="./target/$FRAMEWORK_NAME.xcframework"
BUILD_PATH="./target"
HEADER_PATH="./export/ios"

# 确保已安装 iOS 目标
rustup target add aarch64-apple-ios x86_64-apple-ios

# 构建真机版本 (arm64)
ORT_LIB_LOCATION=./deps/ios/ cargo build --lib --target aarch64-apple-ios --release
# 构建模拟器版本 (x86_64)
ORT_LIB_LOCATION=./deps/ios_sim/ cargo build --lib --target aarch64-apple-ios-sim --release
cargo build --target x86_64-apple-ios --release

# 创建输出目录
mkdir -p $BUILD_PATH

# 合并静态库
lipo -create \
    "./target/aarch64-apple-ios/release/librust_vad.a" \
    "./target/x86_64-apple-ios/release/librust_vad.a" \
    -output "$BUILD_PATH/librust_vad.a"

# 创建 XCFramework
xcodebuild -create-xcframework \
    -library "$BUILD_PATH/librust_vad.a" \
    -headers "$HEADER_PATH" \
    -output "$FRAMEWORK_PATH"

echo "XCFramework 已创建在: $FRAMEWORK_PATH"
