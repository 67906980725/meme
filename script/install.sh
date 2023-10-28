#!/usr/bin/env bash

target_path="$HOME/.local/app/meme"
mkdir -p "$target_path"

# 复制原有数据库和配置文件
[ -f "$target_path/meme.db" ] && cp "$target_path/meme.db" ./target/
[ -f "$target_path/.env" ] && cp "$target_path/.env" ./target/

# 安装
cp -af ./target/. "$target_path/"
if [ "$1" = "termux" ]; then
  sh "$target_path/install_termux.sh"
else
  sh "$target_path/install.sh"
fi
