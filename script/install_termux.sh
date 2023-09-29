#!/usr/bin/env bash
target_path="$HOME/.local/app/meme"
mkdir -p "$target_path"

# 复制原有数据库和配置文件
[ -f "$target_path/meme.db" ] && cp "$target_path/meme.db" ./target/linux/
[ -f "$target_path/.env" ] && cp "$target_path/.env" ./target/linux/
# 安装
cp -af ./target/linux/. "$target_path/"

# 链接资源目录
# ln -s /sdcard/asset/tel/recreation/meme/.asset $HOME/.local/app/meme/asset

create_service() {
  svc_name="$1"
  cmd="$2"
  work_dir="$3"

  dir_path="$PREFIX/var/service/$svc_name"
  mkdir -p "$dir_path"
  cd "$dir_path"
  mkdir log
  ln -sf $PREFIX/share/termux-services/svlogger $dir_path/log/run

  echo '#!/data/data/com.termux/files/usr/bin/sh' > run
#   echo 'termux-wake-lock' >> run
  echo 'exec 2>&1' >> run
  if [ "$work_dir" != "" ]; then
    echo "cd '"$work_dir"'" >> run
  fi
  echo "exec '"$cmd"' 2>&1" >> run
  chmod +x run

  # 可能需要重启设备
  sv-enable "$svc_name"
  sv up "$svc_name"
}

# 服务
create_service meme "$target_path/meme" "$target_path"
