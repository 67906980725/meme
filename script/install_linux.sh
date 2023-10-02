#!/usr/bin/env bash
target_path="$HOME/.local/app/meme"
mkdir -p "$target_path"

# 复制原有数据库和配置文件
[ -f "$target_path/meme.db" ] && cp "$target_path/meme.db" ./target/linux/
[ -f "$target_path/.env" ] && cp "$target_path/.env" ./target/linux/
# 安装
cp -af ./target/linux/. "$target_path/"

# 链接资源目录, 如果没有(链接)资源目录就启动 meme 会启动失败
# ln -s $HOME/asset/tel/recreation/meme/.asset $HOME/.local/app/meme/asset
# 如果是使用链接的资源目录, 并且资源目录与应用不在同一分区, 需要同时创建与资源目录同分区的tmp目录链接

# 桌面图标
mkdir -p "$HOME/.local/share/applications"
ln -sf "$target_path/.local/share/applications/meme_web.desktop" "$HOME/.local/share/applications/meme_web.desktop"

# 服务
mkdir -p "$HOME/.config/systemd/user"
ln -sf "$target_path/.config/systemd/user/meme.service" "$HOME/.config/systemd/user/meme.service"
systemctl enable --now --user meme.service 
# 作为服务自启动后在 gnome 环境下使用 `O` 按钮无法打开表情文件夹
