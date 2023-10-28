#!/usr/bin/env bash

_work_dir=$(pwd)
cd $(dirname "$0")
_cur_dir=$(readlink -f $(pwd))

source ./.env

# 桌面图标
mkdir -p "$HOME/.local/share/applications"
ln -sf "$_cur_dir/.local/share/applications/meme_web.desktop" "$HOME/.local/share/applications/meme_web.desktop"
sed -i '/^Exec=/c\Exec=sh\ -c\ "xdg-open http://127.0.0.1:$PORT"' "$HOME/.local/share/applications/meme_web.desktop"

# 服务
# 作为服务自启动后在 gnome 环境下使用 `O` 按钮无法打开表情文件夹
mkdir -p "$HOME/.config/systemd/user"
ln -sf "$_cur_dir/.config/systemd/user/meme.service" "$HOME/.config/systemd/user/meme.service"
sed -i '/^WorkingDirectory=/c\WorkingDirectory='$_cur_dir'' "$HOME/.config/systemd/user/meme.service"
sed -i '/^ExecStart=/c\ExecStart='$_cur_dir/meme'' "$HOME/.config/systemd/user/meme.service"
# 链接资源目录
# 如果没有资源目录时 meme 会自动创建默认资源目录 "asset"
# ln -s $HOME/asset/tel/recreation/meme/.asset $HOME/.local/app/meme/asset
# 如果是使用链接的资源目录, 并且资源目录与应用不在同一分区, 需要同时创建与资源目录同分区的 tmp 目录链接
systemctl enable --now --user meme.service

cd $_work_dir
