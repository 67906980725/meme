use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use diesel::dsl::max;
use diesel::insert_into;
use diesel::prelude::*;
use walkdir::WalkDir;
use crate::{configs};
use crate::actions::{folder_actions};

use crate::models::{Style};
use crate::schema::style::dsl::*;
// 只能用* 具体到如style时filter就会找不到字段 wtf
use crate::folder_actions::{add_with_childs as add_folder};
use crate::folder_actions::del_by_style_id as del_folder;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn atomic_add(
  conn: &mut SqliteConnection,
  add_name: String,
  sort_opt: Option<i32>
) -> Style {
  let add_name_cp = add_name.clone();
  // 根据name查询, 有的话直接返回
  let po_opt = find_by_name(conn, add_name);
  if po_opt.is_some() { return po_opt.unwrap(); }

  let mut vo = Style { name: add_name_cp, sort: sort_opt.unwrap_or_else(|| 2), id: None };
  insert_into(style).values(&vo).execute(conn).unwrap();

  let id_opt = style.select(max(id)).first::<Option<i32>>(conn).unwrap();
  vo.id = id_opt;
  vo
}

pub fn list(conn: &mut SqliteConnection) -> Vec<Style> {
  style.select(Style::as_select()).order_by(sort.asc()).load(conn).unwrap()
}

pub fn del(conn: &mut SqliteConnection, del_name: String) {
  log::info!("删除style, name:{}", del_name);
  let id_opt_list: Vec<Option<i32>> = style.filter(name.eq(del_name)).select(id).load::<Option<i32>>(conn).unwrap();
  diesel::delete(style.filter(id.eq_any(&id_opt_list.clone()))).execute(conn).unwrap();
  for id_opt in id_opt_list {
    del_folder(conn, id_opt.unwrap());
  }
}

pub fn get(conn: &mut SqliteConnection, get_id: i32) -> Option<Style> {
  style
    .filter(id.eq(get_id))
    .first::<Style>(conn)
    .optional()
    .unwrap()
}

pub fn find_by_name(conn: &mut SqliteConnection,
                    add_name: String) -> Option<Style> {
  style
    .filter(name.eq(add_name))
    .first::<Style>(conn)
    .optional()
    .unwrap()
}

pub fn set_sort(conn: &mut SqliteConnection, file_path: &Path, set_style: Style) {
  log::info!("设置style序号, style_name:{}, path:{:?}", set_style.name, &file_path);
  let mut content = String::new();
  File::open(file_path).unwrap().read_to_string(&mut content).unwrap();
  diesel::update(style.filter(id.eq(set_style.id)))
    .set(sort.eq(content.parse::<i32>().unwrap()))
    .execute(conn)
    .unwrap();
}

pub fn clean(conn: &mut SqliteConnection) {
  diesel::delete(style).execute(conn).unwrap();
}

pub fn add_with_childs(conn: &mut SqliteConnection, add_path: &std::path::Path, clean_keyword: bool) {
  log::info!("创建style:{:?}", &add_path);
  // 是文件夹 不是隐藏文件
  if !add_path.is_dir() { return; }
  let add_name = add_path.file_name().unwrap().to_str().unwrap();
  if add_name.starts_with(".") { return; }

  let style_po = crate::style_actions::atomic_add(conn, String::from(add_name), Option::None);
  WalkDir::new(add_path).max_depth(1).min_depth(1).into_iter()
    .filter_map(|r| r.ok())
    .filter(|entry2| entry2.metadata().is_ok())
    .for_each(|entry2| {
      let metadata2 = entry2.metadata().unwrap();
      let path2 = entry2.path();
      if !metadata2.is_dir() {
        let is_txt = path2.extension().map_or(false, |ext| ext == "txt");
        if is_txt {
          set_sort(conn, path2, style_po.clone());
        }
      } else {
        // let name2 = entry2.file_name();
        add_folder(conn, path2, style_po.clone(), clean_keyword);
      }
    })
}

pub fn init_db(conn: &mut SqliteConnection, clean_keyword: bool) {
  log::info!("开始初始化数据库");
  let asset_path = configs::get_asset_path_buf();
  let asset_walk_dir = WalkDir::new(asset_path).max_depth(1).min_depth(1);
  asset_walk_dir.into_iter()
    .filter_map(|r| r.ok())
    .filter(|entry| entry.metadata().is_ok())
    .for_each(|entry1| {
      let path1 = entry1.path();
      add_with_childs(conn, path1, clean_keyword);
    });
}

pub fn del_data_with_child(conn: &mut SqliteConnection, del_id: Option<i32>) {}

pub fn clean_invalid(conn: &mut SqliteConnection) {
  let asset_path_buf = configs::get_asset_path_buf();
  let pos = list(conn);
  if pos.len() == 0 { return; }

  for po in pos {
    let style_path_buf = asset_path_buf.join(&po.name);
    let file_exist = fs::metadata(style_path_buf).is_ok();
    if file_exist {
      folder_actions::clean_invalid_by_style(conn, &po);
    } else {
      folder_actions::del_data_by_style_id(conn, po.id.unwrap());
      log::info!("删除style:{}", &po.id.unwrap());
      diesel::delete(style.filter(id.eq(po.id))).execute(conn).unwrap();
    }
  }
}
