#!/usr/bin/env bash
target_path="./target/linux"

# 清理上一次构建
rm -rf "$target_path"
mkdir -p "$target_path"

# 资源文件
mv dist "$target_path/"
mv ./src-rs/target/release/meme "$target_path/"
cp -af ./src-rs/resource/. "$target_path/"
