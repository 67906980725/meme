use std::fs;
use std::fs::File;
use std::env;
use diesel::dsl::max;
use diesel::insert_into;
use diesel::prelude::*;

use crate::models::{Folder, Img, ImgFolderR, Style};
use crate::{configs, schema};
use crate::schema::img::dsl::img;
use crate::schema::img_folder_r::dsl::img_folder_r;
use std::option::Option;
use std::path::{Path, PathBuf};
// use clipboard_win::{Clipboard, formats, Getter, Setter};
// use clipboard_win::formats::{RawData, Unicode, Bitmap, CF_TEXT, CF_UNICODETEXT, CF_BITMAP, FileList, CF_HDROP};
// use image::{DynamicImage, GenericImageView};
use std::process::Command;
use crate::actions::{folder_actions, style_actions};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn add_by_parents(conn: &mut SqliteConnection, 
                      style_po: Style, 
                      folder_po: Folder, 
                      add_file_name: String) -> Img {
  log::info!("创建img, style_name:{}, folder_name:{}, img_name: {}", style_po.name, folder_po.name, add_file_name);
  let add_path = style_po.name + "/" + &folder_po.name + "/" + &add_file_name;
  add_with_r(conn, add_path, folder_po.id.unwrap())
}

pub fn add_with_r(conn: &mut SqliteConnection, add_path: String, add_folder_id: i32) -> Img {
  let po = atomic_add(conn, add_path);
  add_img_folder_r(conn, po.id.unwrap(), add_folder_id);
  po
}

pub fn atomic_add(conn: &mut SqliteConnection, add_path: String) -> Img {
  let add_path_cp = add_path.clone();
  // 根据name查询, 有的话直接返回
  let po_opt = find_by_path(conn, add_path);
  if po_opt.is_some() { return po_opt.unwrap(); }

  let mut vo = Img { path: add_path_cp, click: 0, id: None };
  insert_into(img).values(&vo).execute(conn).unwrap();

  let id_opt = img.select(max(schema::img::columns::id)).first::<std::option::Option<i32>>(conn).unwrap();
  vo.id = id_opt;
  vo
}

pub fn add_img_folder_r(conn: &mut SqliteConnection, add_img_id: i32, add_folder_id: i32) -> ImgFolderR {
  let r_po = img_folder_r
    .filter(schema::img_folder_r::columns::img_id.eq(add_img_id))
    .filter(schema::img_folder_r::columns::folder_id.eq(add_folder_id))
    .first::<ImgFolderR>(conn)
    .optional()
    .unwrap();
  if r_po.is_some() {
    return r_po.unwrap();
  }

  let mut vo = ImgFolderR { id: None, img_id: add_img_id, folder_id: add_folder_id };
  insert_into(img_folder_r).values(&vo).execute(conn).unwrap();
  let id_opt = img_folder_r.select(max(schema::img_folder_r::columns::id)).first::<std::option::Option<i32>>(conn).unwrap();
  vo.id = id_opt;
  vo
}

pub fn click(conn: &mut SqliteConnection, click_id: i32, auto: bool) {
  let plus = if auto { 1 } else { 2 };
  let po = img.filter(schema::img::columns::id.eq(Some(click_id))).first::<Img>(conn).optional().unwrap();
  match po {
    Some(po) => {
      let r = diesel::update(img.filter(schema::img::columns::id.eq(po.id)))
        .set(schema::img::columns::click.eq(po.click + plus)).execute(conn);
      match r {
        Ok(_) => {}
        Err(e) => {
          log::error!("img click err: {}", e.to_string());
        }
      }
    }
    None => ()
  }
}

pub fn del(conn: &mut SqliteConnection, del_path: String) {
  log::info!("删除img, path:{:?}", &del_path);
  let pos = img.filter(schema::img::columns::path.eq(del_path)).load::<Img>(conn).unwrap();
  for po in pos {
    diesel::delete(
      img_folder_r
        .filter(schema::img_folder_r::columns::img_id.eq(po.id.unwrap()))
    ).execute(conn).unwrap();
    diesel::delete(img.filter(schema::img::columns::id.eq(po.id))).execute(conn).unwrap();
  }
}

pub fn del_by_folder_id(conn: &mut SqliteConnection, folder_id: i32) {
  let r_pos: Vec<ImgFolderR> = img_folder_r
    .filter(schema::img_folder_r::columns::folder_id.eq(folder_id))
    .load::<ImgFolderR>(conn)
    .unwrap();
  let ids: Vec<Option<i32>> = r_pos.iter().map(|r_po| Some(r_po.img_id)).collect();
  let r_ids: Vec<Option<i32>> = r_pos.iter().map(|r_po| r_po.id).collect();
  diesel::delete(img.filter(schema::img::columns::id.eq_any(&ids))).execute(conn).unwrap();
  diesel::delete(img_folder_r.filter(schema::img_folder_r::columns::id.eq_any(&r_ids))).execute(conn).unwrap();
}

pub fn update_path(conn: &mut SqliteConnection, update_path: String, new_path: String) {
  log::info!("更新img path, old:{}, new:{}", &update_path, &new_path);
  diesel::update(img.filter(schema::img::columns::path.eq(update_path)))
    .set(schema::img::columns::path.eq(new_path))
    .execute(conn)
    .unwrap();
}

pub fn list_by_folder_id(conn: &mut SqliteConnection, list_folder_id: i32) -> Vec<Img> {
  // database is locked问题: 先在前端用同步请求
  let img_id_list: Vec<i32> = img_folder_r
    .filter(schema::img_folder_r::columns::folder_id.eq(list_folder_id)).select(schema::img_folder_r::columns::img_id).load::<i32>(conn).unwrap();
  let img_id_opt_list: Vec<Option<&i32>> = img_id_list.iter().map(|img_id| Option::Some(img_id)).collect();
  img.filter(schema::img::columns::id.eq_any(img_id_opt_list))
    .order_by(schema::img::columns::click.desc())
    .load::<Img>(conn)
    .unwrap()
}

pub fn find_by_path(conn: &mut SqliteConnection,
                    add_path: String) -> std::option::Option<Img> {
  img
    .filter(schema::img::columns::path.eq(add_path.clone()))
    .first::<Img>(conn)
    .optional()
    .unwrap()
}

pub fn clean(conn: &mut SqliteConnection) {
  diesel::delete(img).execute(conn).unwrap();
  diesel::delete(img_folder_r).execute(conn).unwrap();
}

pub fn copy_image(p: String) {
  let asset_path = configs::get_asset_path_buf();
  let full_path_buf = asset_path.join(p); // 相对img中path来说, 并非全真正的全路径
  let full_path = full_path_buf.to_str().unwrap(); // PathBuf to_str需要新开变量, 前期先记住
  cp(full_path);
}

#[cfg(target_os = "android")]
pub fn cp(full_path: &str) {
  let cmd = format!("\"{}\"", &full_path);
  let output = Command::new("termux-share")
    .args(&[&cmd])
    .output()
    .expect("图片复制命令执行异常");
  let ls_list = String::from_utf8(output.stdout);
  log::info!("图片复制命令执行结果:{:?}", ls_list);
}

#[cfg(target_os = "linux")]
pub fn cp(full_path: &str) {
  if let Ok(session_type) = env::var("XDG_SESSION_TYPE") {
    if session_type == "x11" {
      // 需要安装xclip
      let output = Command::new("xclip")
        .args(&["-selection", "clipboard", "-t", "image/png", "-i", &format!("\"{}\"", full_path)])
        .output()
        .expect("图片复制命令执行异常");
      let ls_list = String::from_utf8(output.stdout);
      // log::info!("图片复制命令执行结果:{:?}", ls_list);
      return;
    } else if session_type == "wayland" {
      println!("当前环境为wayland");
    } else {
      println!("无法确定当前环境");
    }
  } else {
    println!("无法获取XDG_SESSION_TYPE环境变量");
  }
  // 需要安装wl-clipboard
  let cmd = format!("wl-copy -t image/png < \"{}\"", full_path);
  let output = Command::new("sh")
    .args(&["-c", &cmd])
    .output()
    .expect("图片复制命令执行异常");
  let ls_list = String::from_utf8(output.stdout);
  // log::info!("图片复制命令执行结果:{:?}", ls_list);
}

#[cfg(windows)]
pub fn cp(full_path: &str) {
  let window_full_path = full_path.replace("/", r"\");

  // https://github.com/DoumanAsh/clipboard-win/blob/master/tests/test_clip.rs
  // 转bitmap
  // let dynamic_image = image::open(window_full_path).expect("Failed to open image!");
  // let bitmap_raw = dynamic_image.to_luma8().into_raw();
  // Bitmap.write_clipboard(&bitmap_raw)
  //   .unwrap_or_else(|e| { log::error!("向剪切板写入图片失败:{:?}", e.message()) }); // todo 参数大小错误

  // 复制到临时文件 避免全路径过长
  let mut tmp_file_buf = configs::get_tmp_path_buf().join("1");
  fs::copy(window_full_path, tmp_file_buf.clone()).unwrap();
  
  let current_dir = env::current_dir().unwrap();
  let absolute_tmp_file_buf: PathBuf = current_dir.join(&tmp_file_buf);
  if ! tmp_file_buf.is_absolute() {
    tmp_file_buf = absolute_tmp_file_buf;
  }
  
  // 使用脚本复制
  let script_buf = Path::new("copy_img_win.ps1");
  let script = script_buf.to_str().unwrap();
  // {} 后字符串不会被""包裹, {:?}后输出的是适合放在代码里的字符串对象, 无法直接放在命令行里执行
  let cmd = format!(".\\{} -img \"{}\"", &script, &tmp_file_buf.to_str().unwrap());
  let output = Command::new("powershell")
    .args(&["-ExecutionPolicy", "RemoteSigned", "-Command", &cmd])
    .output()
    .expect("图片复制命令执行异常");
  let ls_list = String::from_utf8(output.stdout);
  // log::info!("图片复制命令执行结果:{:?}", ls_list);
}

pub fn list_r_by_folder(
  conn: &mut SqliteConnection,
  list_folder_id: i32,
) -> Vec<ImgFolderR> {
  img_folder_r
    .filter(schema::img_folder_r::columns::folder_id.eq(list_folder_id))
    .load::<ImgFolderR>(conn)
    .unwrap()
}

pub fn del_data_by_folder_id(conn: &mut SqliteConnection, del_folder_id: i32) {
  let r_pos: Vec<ImgFolderR> = img_folder_r
    .filter(schema::img_folder_r::columns::folder_id.eq(del_folder_id))
    .load::<ImgFolderR>(conn)
    .unwrap();
  if r_pos.len() == 0 { return; }

  let r_po_ids: Vec<Option<i32>> = r_pos.iter().map(|r_po| r_po.id).collect();
  let po_ids: Vec<Option<i32>> = r_pos.iter().map(|r_po| Some(r_po.img_id)).collect();

  log::info!("删除img_folder_r:{:?}", &r_po_ids);
  diesel::delete(img_folder_r.filter(
    schema::img_folder_r::columns::id.eq_any(&r_po_ids))
  ).execute(conn).unwrap();

  log::info!("删除img:{:?}", &po_ids);
  diesel::delete(img.filter(
    schema::img::columns::id.eq_any(&po_ids))
  ).execute(conn).unwrap();
}

pub fn clean_invalid_by_folder(conn: &mut SqliteConnection,
                               clean_folder: &Folder) {
  let r_pos = list_r_by_folder(conn, clean_folder.id.unwrap());
  if r_pos.len() == 0 { return; }

  let ids: Vec<Option<i32>> = r_pos.iter().map(|r_po| Some(r_po.img_id)).collect();
  let pos: Vec<Img> = img.filter(schema::img::columns::id.eq_any(&ids))
    .load::<Img>(conn)
    .unwrap();

  let asset_path_buf = configs::get_asset_path_buf();

  let po_ids: Vec<Option<i32>> = pos.iter().filter(|po| {
    let po_path_buf = asset_path_buf.join(&po.path);
    !fs::metadata(po_path_buf).is_ok()
  }).map(|po| po.id).collect();

  if po_ids.len() > 0 {
    let ids: Vec<i32> = po_ids.iter().map(|po_id| po_id.unwrap()).collect();
    log::info!("删除img_folder_r:{:?}", &ids);
    diesel::delete(img_folder_r.filter(
      schema::img_folder_r::columns::img_id.eq_any(&ids))
    ).execute(conn).unwrap();
    log::info!("删除img:{:?}", &po_ids);
    diesel::delete(img.filter(
      schema::img::columns::id.eq_any(&po_ids))
    ).execute(conn).unwrap();
  }
}

pub fn add_by_files(conn: &mut SqliteConnection, add_folder_id: i32, files: &Vec<(PathBuf, String)>) {
  let folder_po = folder_actions::get(conn, add_folder_id).expect("folder 不存在");
  let style_po = style_actions::get(conn, folder_po.style_id).expect("style 不存在");
  let folder_path_buf = configs::get_asset_path_buf().join(&style_po.name).join(&folder_po.name);
  for (tmp_file_path, origin_file_name) in files {
    add_by_file(conn, &style_po, &folder_po, &folder_path_buf, tmp_file_path, origin_file_name)
  }
}

pub fn add_by_file(conn: &mut SqliteConnection,
                   style_po: &Style,
                   folder_po: &Folder,
                   folder_path_buf: &PathBuf,
                   tmp_file_path: &PathBuf,
                   origin_file_name: &str) {
  // 是否存在相同文件(文件名,大小相同就算)
  let exist_path_buf = folder_path_buf.clone().join(origin_file_name);
  let name_exist = exist_path_buf.clone().metadata().is_ok();
  let file_exist = name_exist && fs::metadata(tmp_file_path).unwrap().len() == File::open(&exist_path_buf).unwrap().metadata().unwrap().len();

  // 有相同文件, 删除tmp_file
  if file_exist {
    fs::remove_file(tmp_file_path).unwrap();
    return;
  }

  let f_name_str = if !name_exist { origin_file_name } else { tmp_file_path.file_name().unwrap().to_str().unwrap() };
  let f_name = String::from(f_name_str);
  let img_path_buf = folder_path_buf.join(&f_name);
  fs::rename(&tmp_file_path, &img_path_buf).unwrap();

  let add_img_path = format!("{}/{}/{}",
                             style_po.name.clone(),
                             folder_po.name.clone(),
                             f_name.clone());
  add_with_r(conn, add_img_path, folder_po.id.unwrap());
}
