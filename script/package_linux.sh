#!/usr/bin/env bash
target_path="./target/linux"

# 清理上一次构建
rm -rf "$target_path"
# 创建构建目录
mkdir -p "$target_path"
# 前端
mv dist "$target_path/"
# 后端
mv ./src-rs/target/release/meme "$target_path/"
# 配置文件
cp ./src-rs/resource/.env "$target_path/"
# 数据库
cp ./src-rs/resource/meme.db "$target_path/"
# 服务
mkdir -p "$target_path/.config/systemd/user"
cp -r "./script/.config" "$target_path/"
# 启动图标
mkdir -p "$target_path/.local/share/applications"
cp -r "./script/.local" "$target_path/"
