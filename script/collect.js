#!/usr/bin/env node

const fs = require('fs')
const path = require('path')
const os = require('os')

// cargo 构建目录
// const cargoTargetDir = '/tmp/.cargo/target' // 自定义时
const cargoTargetDir = './src-rs/target' // 默认

// 最终生成目录
const targetPath = './target'

const isWindows = os.type() == 'Windows_NT'
const isTermux = process.argv.length > 2 && process.argv[2] == 'termux'
const isLinux = os.type() == 'Linux' && !isTermux
  
// 清理上一次构建
try { fs.rmSync(targetPath, { recursive: true }) } catch (error) {  }
fs.mkdirSync(targetPath, { recursive: true })

// 资源文件
fs.copyFileSync('./README.md', path.join(targetPath, 'README.md'))
fs.cpSync('./src-rs/resource/universal', targetPath, { recursive: true })
if (isWindows) {
  fs.cpSync('./src-rs/resource/windows', targetPath, { recursive: true })
} else if (isLinux) {
  fs.cpSync('./src-rs/resource/linux', targetPath, { recursive: true })
} else if (isTermux) {
  fs.cpSync('./src-rs/resource/termux', targetPath, { recursive: true })
}

// 页面
fs.cpSync('dist', path.join(targetPath, 'dist'), { recursive: true })

// 服务器
let release = path.join(cargoTargetDir, '/release/meme')
let bin_name = 'meme'
if (isWindows) {
  release += '.exe'
  bin_name += '.exe'
}
fs.copyFileSync(release, path.join(targetPath, bin_name))
