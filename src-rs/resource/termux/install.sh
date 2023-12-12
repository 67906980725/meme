#!/usr/bin/env bash

# 依赖
pkg update
pkg install -y termux-api termux-services
pkg install -y sqlite libsqlite

_work_dir=$(pwd)
cd $(dirname "$0")
_app_dir=$(readlink -f $(pwd))

# 链接资源目录
# 如果没有资源目录时 meme 会自动创建默认资源目录 "asset"
# ln -s $HOME/storage/shared/asset/tel/recreation/meme/.asset $HOME/.local/app/meme/asset
# 如果是使用链接的资源目录, 并且资源目录与应用不在同一分区, 需要同时创建与资源目录同分区的 tmp 目录链接
# mkdir -p $HOME/storage/shared/asset/tmp
# ln -s $HOME/storage/shared/asset/tmp $HOME/.local/app/meme/tmp

create_service() {
  svc_name="$1" cmd="$2" work_dir="$3"

  work_dir=$(pwd)

  dir_path="$PREFIX/var/service/$svc_name"

  mkdir -p "$dir_path/log"
  cd "$dir_path"
  ln -sf $PREFIX/share/termux-services/svlogger $dir_path/log/run

  echo '#!/data/data/com.termux/files/usr/bin/sh' > run
  #echo 'termux-wake-lock' >> run
  echo 'exec 2>&1' >> run
  if [ "$work_dir" != "" ]; then
    echo "cd '"$work_dir"'" >> run
  fi
  echo "exec '"$cmd"' 2>&1" >> run
  chmod +x run

  sv-enable "$svc_name"
  sv up "$svc_name"

  cd "$work_dir"
  # 可能需要重启设备
}
create_service meme "$_app_dir/meme" "$_app_dir"

cd $_work_dir
