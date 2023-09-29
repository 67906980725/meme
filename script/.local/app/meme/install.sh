#!/usr/bin/env bash

cd $(dirname "$0")

# 桌面图标
mkdir -p "$HOME/.local/share/applications"
cp -f "./.local/share/applications/meme_web.desktop" "$HOME/.local/share/applications/meme_web.desktop"

# 服务
mkdir -p "$HOME/.config/systemd/user"
cp -f "./.config/systemd/user/meme.service" "$HOME/.config/systemd/user/meme.service"
systemctl enable --now --user meme.service
