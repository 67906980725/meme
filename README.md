# meme 本地表情图分享

这是一个根据关键词搜索本地表情图片的程序

## 功能

- 为同一类图片创建多个关键词
- 关键词自动转拼音/拼音首字母
- 多关键词搜索(空号分隔)
- 从其它应用拖动图片到meme(不支持firefox)
- 点击图片复制到剪切板(不支持termux)
- 自动同步数据库(延时5分钟)
- 多模式切换: 工作/摸鱼/专属... 不同的场景使用不同的风格聊天

## 使用方式

在命令行(windows可以双击)启动后在浏览器访问 127.0.0.1:8899

## 初始化

### 资源

- 在 `asset` 目录下存放你的按**风格(模式)文件夹**\\**分类文件夹**分好类的表情图片
- 每个**分类文件夹**下存放一个`utf-8`编码的文本文件(如`keywork.txt`), 里边写好对于这个类目你所能联想到的尽可能多的关键词并以逗号分隔
- `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned -Force` 在 powershell 执行命令(windows)

.asset 目录结构如下

``` plaintext
.asset
    └─工作
        │ order.txt
        │
        ├─好的
        │      keywords.txt
        │      2ba4fda4_1440w.jpg
```

### windows

下载 sqlite3.dll 到 meme 目录下

### linux

安装 `xclip`

`wayland` 环境下需要安装 `wl-clipboard`

### termux

``` bash
# 更新源
apt update
apt upgrade -y

# 安装基础依赖
pkg install -y termux-api termux-services
# 存储权限
termux-setup-storage



# 安装glibc
pkg ins pacman -y
github_proxy='https://download.nuaa.cf/'
arch='aarch64'
wget ${github_proxy}https://github.com/Maxython/glibc-for-termux/releases/download/20221025/gpft-20221025-${arch}.tar.xz
tar xJf gpft-20221025-${arch}.tar.xz
pacman -U glibc-for-termux/*
#grun --shell # 起一个可以访问 glibc 命令的shell

# 安装sqlite
pkg install -y sqlite
```

## 构建

### 开发环境(debian

- nodejs npm yarn
- rustup
- libsqlite3-dev

windows 需要构建 `SQLite3.lib` `sqlite3.exp` 并复制 `sqlite3.dll` 到工作目录

### 构建 meme

``` shell
# 安装依赖
yarn install

# 如果是 linux 或 termux
#  - 打包资源文件到 target 目录
yarn run package-linux
#  - linux 直接安装 (到 ~/.local/app/meme)
yarn run install-linux
#  - termux 直接安装
yarn run install-termux
```
