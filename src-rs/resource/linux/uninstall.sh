#!/usr/bin/env bash

cd $(dirname "$0")

systemctl disable --now --user meme.service
rm -f "$HOME/.config/systemd/user/meme.service"
rm -f "$HOME/.local/share/applications/meme_web.desktop"
