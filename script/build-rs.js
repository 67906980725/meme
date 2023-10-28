#!/usr/bin/env node

const fs = require('fs')
const os = require('os')

if (os.type() != 'Windows_NT') {
  return
}

if (!fs.existsSync('sqlite3.lib')) {
  fs.cpSync('src-rs/resource/windows/sqlite3_win_x64', './', { recursive: true })
}
