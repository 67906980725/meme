# meme 本地表情图分享

这是一个根据关键词搜索本地表情图片的程序

## 功能

- 为同一类图片创建多个关键词
- 关键词自动转拼音/拼音首字母
- 多关键词搜索(空号分隔)
- 从其它应用拖动/复制图片到meme(firefox不支持)
- 点击图片复制到剪切板(termux不支持)
- 多模式切换: 工作/摸鱼/专属... 不同的场景使用不同的风格聊天

## 使用方式

- [启动] 在命令行(windows可以双击)启动后在浏览器访问 `127.0.0.1:8899`
- [添加模式/风格] 点击 `S` 按钮添加模式/风格
- [添加表情分类] 点击 `F` 按钮添加表情分类, 在 `关键词` 栏目输入你所能联想到的尽可能多的关键词并以逗号分隔
- [添加表情图片 (`pc:chrome`)] 拖动/复制图片到当前分类下空白处
- [添加表情图片 (`android:termux`)] 点击 `+` 按钮选择图片
- [搜索表情图片] 在搜索框输入 关键词 / 关键词拼音 / 关键词拼音缩写 / 多个关键词 后回车
- [复制表情图片 (`pc`)] 点击表情图片
- [修改关键词] 添加相同名称的表情分类
- [删除风格/分类/表情] 可以用文件管理器删除, 会延时自动同步, 或手动同步

## 依赖

### windows

- 下载 `sqlite3.dll` 到 `meme` 目录下
- 在 `powershell` 执行命令以允许运行脚本 `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned -Force`

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
# 申请存储权限
termux-setup-storage

# 安装sqlite
pkg install -y sqlite

# 安装glibc
pkg ins pacman -y
github_proxy='https://download.nuaa.cf/'
arch='aarch64'
wget ${github_proxy}https://github.com/Maxython/glibc-for-termux/releases/download/20221025/gpft-20221025-${arch}.tar.xz
tar xJf gpft-20221025-${arch}.tar.xz
pacman -U glibc-for-termux/*
#grun --shell # 起一个可以访问 glibc 命令的shell
```

## 资源

- 在 `asset` 目录下存放着按**风格(模式)文件夹**/**分类文件夹**分好类的表情图片
- 每个**表情分类文件夹**下存有放一个`utf-8`编码, 内容为关键词的文本文件(默认为 `keywork.txt`)
- 资源管理使用了文件监听, 可以使用文件管理器管理资源, 会自动延时同步数据库

`.asset` 目录结构如下

``` plaintext
.asset
    └─工作
        │ order.txt
        │
        ├─好的
        │      keywords.txt
        │      2ba4fda4_1440w.jpg
```

## 构建

### 开发环境(debian

- nodejs npm yarn
- rustup
- libsqlite3-dev

`windows` 需要构建 `SQLite3.lib` `sqlite3.exp` 并复制 `sqlite3.dll` 到工作目录

### 构建 meme

``` shell
# 安装依赖
yarn install

# 如果是 linux 或 termux
#  - 打包资源文件到 target 目录
yarn run collect
#  - linux 直接安装 (到 ~/.local/app/meme)
yarn run install-linux
#  - termux 直接安装
yarn run install-termux
```
