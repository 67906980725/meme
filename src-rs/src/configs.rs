use std::env;
use std::path::{PathBuf};
use diesel::{Connection, SqliteConnection};

// 端口
pub fn get_port() -> u16 {
  std::env::var("PORT")
    .map_or_else(|_| 8899, |s| s.parse::<u16>().expect("端口错误"))
}

// 创建数据库连接 默认为工作目录下的 "meme.db"
// lib.exe 根据.def文件生成.LIB文件(SQLite3)  https://www.cnblogs.com/hjbf/p/12829525.html
// todo sqlite用数据库连接池时异步线程无法共享连接池?
pub fn establish_connection() -> SqliteConnection {
  dotenvy::dotenv().ok();
  let db_path = std::env::var("DB").unwrap_or_else(|_| "meme.db".to_string());
  let database_url = env::var("DATABASE_URL").unwrap_or(db_path);
  SqliteConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// 图片资源目录 默认为工作目录下的 "asset"
pub fn get_asset_path_buf() -> PathBuf {
  // 环境变量$ASSET_DIR 或 工作目录下的.asset文件夹
  let asset_dir_r = std::env::var("ASSET_DIR").unwrap_or_else(|_| "asset".to_string());
  PathBuf::from(asset_dir_r)
}

// 临时文件目录 默认为工作目录下的 "tmp"
pub fn get_tmp_path_buf() -> PathBuf {
  let tmp_dir_r = std::env::var("TMP_DIR").unwrap_or_else(|_| "tmp".to_string());
  PathBuf::from(tmp_dir_r)
}
