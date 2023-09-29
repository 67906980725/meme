mod schema;
mod models;
mod util;
mod configs;
mod actions;
mod params;

use std::fs;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use actix_cors::Cors;
use actix_files::{Files};

use actix_web::{get, web, App, HttpServer, Responder, HttpRequest, HttpResponse, middleware, post};
use crate::actions::{folder_actions, img_actions, keyword_actions, style_actions, watch_actions};
use actix_multipart::{Multipart};
use actix_rt::{Arbiter};
use futures::TryStreamExt;
use crate::util::uuids;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  dotenvy::dotenv().ok(); // 要在std::env::var(<VARIABLE_NAME>)前调用
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  // 异步监听图片资源目录
  let arbiter = Arbiter::new();
  arbiter.spawn(async { 
      // 初始化
      // 创建tmp目录
      fs::create_dir_all(configs::get_tmp_path_buf()).unwrap();
      let mut conn = configs::establish_connection();
      style_actions::clean_invalid(&mut conn);
      sleep(Duration::from_secs(2));
      style_actions::init_db(&mut conn, true);
      drop(conn);
      println!("欢迎! 访问 http://127.0.0.1:{} 以使用meme", configs::get_port());
      watch_actions::async_watch().await.unwrap() 
  });

  HttpServer::new(move || {
    App::new()
      .service(greet)
      .service(add_style).service(list_style)
      .service(add_folder).service(by_key).service(click_folder)
      .service(open_folder).service(get_imgs)
      .service(add_img).service(click_img).service(copy_image)
      .service(init_db).service(re_init_db)
      // 图片资源目录映射到/file接口
      .service(Files::new("/file", configs::get_asset_path_buf())
        .show_files_listing())
      // 页面资源映射到/接口
      .service(Files::new("/", "dist")
        .index_file("index.html"))
      .wrap(middleware::Logger::default())
      // 允许跨域
      .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
  })
    .workers(1)
    .bind(("127.0.0.1", configs::get_port()))
    .unwrap()
    .run()
    .await
    .unwrap();

  // 结束异步任务  
  arbiter.stop();
  Ok(())
}

// hello
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
  HttpResponse::Ok().json(format!("Hello {name}!"))
}

// 添加style
#[get("/style/add/{name}")]
async fn add_style(name: web::Path<String>) -> impl Responder {
  let r = web::block(move || {
    let mut conn = configs::establish_connection();
    let r = style_actions::atomic_add(&mut conn, name.to_string());
    drop(conn);
    r
  }).await.unwrap();
  HttpResponse::Ok().json(r)
}

#[get("/style/list")]
async fn list_style() -> impl Responder {
  let r = web::block(move || {
    let mut conn = configs::establish_connection();
    let r = style_actions::list(&mut conn);
    drop(conn);
    r
  }).await.unwrap();
  HttpResponse::Ok().json(r)
}

// 添加folder
#[post("/folder/add")]
async fn add_folder(params: web::Json<params::AddFolderParam>) -> impl Responder {
  let r = web::block(move || {
    let mut conn = configs::establish_connection();
    let r = folder_actions::add_with_keyword(&mut conn, params.0);
    drop(conn);
    r
  }).await.unwrap();
  HttpResponse::Ok()
}

// 关键词查folder
#[get("/folder/by_key")]
async fn by_key(params: web::Query<params::FolderParam>) -> impl Responder {
  let r = web::block(move || {
    let mut conn = configs::establish_connection();
    let r = folder_actions::list_by_key(&mut conn, params.keyword.clone(), params.styleId);
    drop(conn);
    r
  }).await.unwrap();
  HttpResponse::Ok().json(r)
}

// 点击folder 增加排序权重
#[get("/folder/click")]
async fn click_folder(params: web::Query<params::ClickParam>) -> impl Responder {
  web::block(move || {
    let mut conn = configs::establish_connection();
    let auto = params.auto == 1;
    folder_actions::click(&mut conn, params.id, auto);
    drop(conn);
  }).await.unwrap();
  HttpResponse::Ok()
}

// 打开folder文件夹
#[get("/folder/open")]
async fn open_folder(params: web::Query<params::IdParam>) -> impl Responder {
  web::block(move || {
    let mut conn = configs::establish_connection();
    folder_actions::open(&mut conn, params.id);
    drop(conn);
  }).await.unwrap();
  HttpResponse::Ok()
}

// folder下全部img
#[get("/folder/get_imgs")]
async fn get_imgs(params: web::Query<params::IdParam>) -> impl Responder {
  let r = web::block(move || {
    let mut conn = configs::establish_connection();
    let r = img_actions::list_by_folder_id(&mut conn, params.id);
    drop(conn);
    r
  }).await.unwrap();
  HttpResponse::Ok().json(r)
}

// 点击img 增加排序权重
#[get("/folder/img_click")]
async fn click_img(params: web::Query<params::ClickParam>) -> impl Responder {
  web::block(move || {
    let mut conn = configs::establish_connection();
    let auto = params.auto == 1;
    img_actions::click(&mut conn, params.id, auto);
    drop(conn);
  }).await.unwrap();
  HttpResponse::Ok()
}

// 上传图片并添加img数据
#[post("/img/add")]
async fn add_img(params: web::Query<params::IdParam>, mut payload: Multipart) -> impl Responder {
  // 上传到 tmp/{uuid}.{suffix}
  let tmp_path = configs::get_tmp_path_buf();
  let mut files = Vec::new();
  while let Ok(Some(mut field)) = payload.try_next().await {
    let content_disposition = field.content_disposition().clone();
    let filename = content_disposition.get_filename().unwrap();

    let suffix = std::path::Path::new(filename).extension().unwrap();
    let tmp_file_name = format!("{}.{}", uuids::uuid_string(), suffix.to_str().unwrap());
    let tmp_file_path_buf = tmp_path.clone().join(&tmp_file_name);
    let tmp_file_path_buf_clone = tmp_file_path_buf.clone();
    let mut tmp_file = web::block(move || fs::File::create(&tmp_file_path_buf)).await.unwrap().unwrap();
    while let Some(chunk) = field.try_next().await.unwrap() {
      tmp_file = web::block(move || tmp_file.write_all(&chunk).map(|_| tmp_file)).await.unwrap().unwrap();
    }
    files.push((tmp_file_path_buf_clone, String::from(filename)));
  }

  web::block(move || {
    let mut conn = configs::establish_connection();
    img_actions::add_by_files(&mut conn, params.id, &files);
    drop(conn);
  }).await.unwrap();

  HttpResponse::Ok()
}

// 没用
#[get("/folder/init_db")]
async fn init_db() -> impl Responder {
  web::block(move || {
    let mut conn = configs::establish_connection();
    // style_actions::clean(&mut conn);
    // folder_actions::clean(&mut conn);
    // keyword_actions::clean(&mut conn);
    // img_actions::clean(&mut conn);
    style_actions::init_db(&mut conn, false);
    drop(conn);
  }).await.unwrap();
  HttpResponse::Ok()
}

// 根据图片资源目录初始化数据库
#[get("/folder/re_init_db")]
async fn re_init_db() -> impl Responder {
  web::block(move || {
    let mut conn = configs::establish_connection();
    style_actions::clean_invalid(&mut conn);
    sleep(Duration::from_secs(2));
    style_actions::init_db(&mut conn, true);
    drop(conn);
  }).await.unwrap();
  HttpResponse::Ok()
}

// 根据img相对资源目录复制图片
#[get("/image/copy")]
async fn copy_image(params: web::Query<params::CpImgParam>) -> impl Responder {
  web::block(move || {
    img_actions::copy_image(params.path.clone());
  }).await.unwrap();
  HttpResponse::Ok()
}


