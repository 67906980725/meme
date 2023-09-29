#!/usr/bin/env bash
target_path="$HOME/.local/app/meme"
mkdir -p "$target_path"

# 复制原有数据库和配置文件
[ -f "$target_path/meme.db" ] && cp "$target_path/meme.db" ./target/linux/
[ -f "$target_path/.env" ] && cp "$target_path/.env" ./target/linux/
# 安装
cp -af ./target/linux/. "$target_path/"

# 链接资源目录
# ln -s $HOME/asset/tel/recreation/meme/.asset $HOME/.local/app/meme/asset

# 桌面图标
mkdir -p "$HOME/.local/share/applications"
ln -sf "$target_path/.local/share/applications/meme_web.desktop" "$HOME/.local/share/applications/meme_web.desktop"

# 服务
mkdir -p "$HOME/.config/systemd/user"
ln -sf "$target_path/.config/systemd/user/meme.service" "$HOME/.config/systemd/user/meme.service"
systemctl enable --now --user meme.service
