use std::fs::File;
use std::io::Read;
use std::path::Path;
use diesel::dsl::max;
use diesel::insert_into;
use diesel::prelude::*;
use crate::models::{Folder, Keyword, KeywordFolderR};
use crate::{schema};
use crate::schema::keyword::dsl::keyword;
use crate::schema::keyword_folder_r::dsl::keyword_folder_r;
use crate::util::pinyins;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn add(conn: &mut SqliteConnection,
           add_name: String,
           add_style_id: i32,
           add_folder_id: i32) -> Vec<Keyword> {
  let pos = add_with_pinyin(conn, add_name, add_style_id);
  for po in pos.clone() {
    add_folder_r(conn, po.id.unwrap(), add_folder_id, add_style_id);
  }
  pos
}

pub fn add_with_pinyin(conn: &mut SqliteConnection, add_name: String,
                       add_style_id: i32) -> Vec<Keyword> {
  let add_name2 = add_name.clone();
  let add_name3 = add_name.clone();
  let po = atomic_add(conn, add_name, add_style_id);
  let with_pinyin = std::env::var("WITH_PINYIN")
    .map(|v| v == "1" || v.to_lowercase() == "true")
    .unwrap_or_else(|_| true);
  let with_py = std::env::var("WITH_PY")
    .map(|v| v == "1" || v.to_lowercase() == "true")
    .unwrap_or_else(|_| true);
  if !with_pinyin && !with_py || !pinyins::contains_chinese(&add_name2) {
    return vec![po];
  }

  let mut pos = Vec::new();
  pos.push(po);

  let (pinyin_string, py_string) = pinyins::str_to_pinyin_and_py(&add_name3);
  if with_pinyin {
    let nkw_po = atomic_add(conn, pinyin_string, add_style_id);
    pos.push(nkw_po);
  }
  if with_py {
    let nkw_po = atomic_add(conn, py_string, add_style_id);
    pos.push(nkw_po);
  }
  return pos;
}

pub fn add_folder_r(conn: &mut SqliteConnection,
                    add_keyword_id: i32,
                    add_folder_id: i32,
                    add_style_id: i32) -> KeywordFolderR {
  let keyword_folder_r_opt = keyword_folder_r
    .filter(schema::keyword_folder_r::columns::keyword_id.eq(add_keyword_id))
    .filter(schema::keyword_folder_r::columns::folder_id.eq(add_folder_id))
    .filter(schema::keyword_folder_r::columns::style_id.eq(add_style_id))
    .first::<KeywordFolderR>(conn)
    .optional()
    .unwrap();
  if keyword_folder_r_opt.is_some() {
    return keyword_folder_r_opt.unwrap();
  }

  let mut vo = KeywordFolderR { id: None, keyword_id: add_keyword_id, folder_id: add_folder_id, style_id: add_style_id };
  insert_into(keyword_folder_r).values(&vo).execute(conn).unwrap();

  let id_opt = keyword_folder_r.select(max(schema::keyword_folder_r::columns::id)).first::<Option<i32>>(conn).unwrap();
  vo.id = id_opt;
  vo
}

pub fn atomic_add(conn: &mut SqliteConnection,
                  add_name: String,
                  add_style_id: i32) -> Keyword {
  let add_name_cp = add_name.clone();
  // 根据name查询, 有的话直接返回
  let po_opt = find_by_name(conn, add_name, add_style_id);
  if po_opt.is_some() { return po_opt.unwrap(); }

  let mut vo = Keyword { name: add_name_cp, style_id: add_style_id, id: None };
  insert_into(keyword).values(&vo).execute(conn).unwrap();

  let id_opt = keyword.select(max(schema::keyword::columns::id)).first::<Option<i32>>(conn).unwrap();
  vo.id = id_opt;
  vo
}

pub fn find_by_name(conn: &mut SqliteConnection,
                    add_name: String, add_style_id: i32) -> Option<Keyword> {
  keyword
    .filter(schema::keyword::columns::name.eq(add_name.clone()))
    .filter(schema::keyword::columns::style_id.eq(add_style_id))
    .first::<Keyword>(conn)
    .optional()
    .unwrap()
}

pub fn find_by_name_like(conn: &mut SqliteConnection,
                         add_name: String, add_style_id: i32) -> Vec<Keyword> {
  keyword
    .filter(schema::keyword::columns::name.like( format!("%{}%", add_name.clone())))
    .filter(schema::keyword::columns::style_id.eq(add_style_id))
    .load::<Keyword>(conn)
    .unwrap()
}

// 用文件创建关键词
pub fn add_by_file_and_folder(conn: &mut SqliteConnection, file_path: &Path, add_folder: Folder, clean: bool) {
  log::info!("用文件创建关键词, folder_name:{}, path:{:?}", add_folder.name, &file_path);
  let mut content = String::new();
  File::open(file_path).unwrap().read_to_string(&mut content).unwrap();
  // 中文逗号换成英文逗号
  content = content.replace("，", ",");
  let content_split = content.split(",");
  let mut keyword_pos = Vec::new();
  // 生成关键词条目
  content_split.into_iter()
    .map(|s| s.trim())
    .filter(|s| s.len() > 0)
    .for_each(|keyword_str| {
      let mut this_keyword_pos = add(conn, String::from(keyword_str), add_folder.style_id, add_folder.id.unwrap());
      if clean { keyword_pos.append(&mut this_keyword_pos); }
    });

  if !clean { return; }

  // 更改keyword文件时清理不再关联的关联关系
  let mut keyword_ids: Vec<i32> = keyword_pos.iter().map(|o| o.id.unwrap()).collect();

  // folder自己的keywords
  let mut names = Vec::new();
  if pinyins::contains_chinese(&add_folder.name) {
    let (pinyin_str, py_str) = pinyins::str_to_pinyin_and_py(&add_folder.name);
    names.push(pinyin_str);
    names.push(py_str);
  }
  names.push(add_folder.name);
  let id_opt_list: Vec<Option<i32>> = keyword.filter(schema::keyword::columns::name.eq_any(names))
    .select(schema::keyword::columns::id)
    .load::<Option<i32>>(conn)
    .unwrap();
  let mut ids: Vec<i32> = id_opt_list.iter().map(|id_opt| id_opt.unwrap()).collect();
  keyword_ids.append(&mut ids);

  // 删除folder所有r里keyword_id不在上列的
  diesel::delete(
    keyword_folder_r
      .filter(schema::keyword_folder_r::columns::folder_id.eq(add_folder.id.unwrap()))
      .filter(schema::keyword_folder_r::columns::keyword_id.ne_all(&keyword_ids))
  ).execute(conn).unwrap();
}

pub fn clean(conn: &mut SqliteConnection) {
  diesel::delete(keyword).execute(conn).unwrap();
  diesel::delete(keyword_folder_r).execute(conn).unwrap();
}

pub fn list_r_by_folder(
  conn: &mut SqliteConnection,
  list_folder_id: i32,
) -> Vec<KeywordFolderR> {
  keyword_folder_r
    .filter(schema::keyword_folder_r::columns::folder_id.eq(list_folder_id))
    .load::<KeywordFolderR>(conn)
    .unwrap()
}

pub fn del_data_by_folder_id(conn: &mut SqliteConnection, del_folder_id: i32) {
  let r_pos: Vec<KeywordFolderR> = keyword_folder_r
    .filter(schema::keyword_folder_r::columns::folder_id.eq(del_folder_id))
    .load::<KeywordFolderR>(conn)
    .unwrap();
  if r_pos.len() == 0 { return; }

  let r_po_ids: Vec<Option<i32>> = r_pos.iter().map(|r_po| r_po.id).collect();
  let po_ids: Vec<Option<i32>> = r_pos.iter().map(|r_po| Some(r_po.keyword_id)).collect();

  log::info!("删除keyword_folder_r:{:?}", &r_po_ids);
  diesel::delete(keyword_folder_r.filter(
    schema::keyword_folder_r::columns::id.eq_any(&r_po_ids))
  ).execute(conn).unwrap();
  log::info!("删除keyword:{:?}", &po_ids);
  diesel::delete(keyword.filter(
    schema::keyword::columns::id.eq_any(&po_ids))
  ).execute(conn).unwrap();
}
