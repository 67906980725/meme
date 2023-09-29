use std::fs;
use std::fs::File;
use std::io::Write;
use diesel::dsl::max;
use diesel::insert_into;
use diesel::prelude::*;
use crate::{configs, keyword_actions, schema};
use crate::models::{Folder, Keyword, KeywordFolderR, Style};
use crate::schema::folder::dsl::folder;
use crate::schema::keyword_folder_r::dsl::keyword_folder_r;
use std::option::Option;
use std::process::Command;
use comparator::{as_fn, comparing};
use log::log;
use walkdir::WalkDir;
use crate::actions::{img_actions, style_actions};
use crate::keyword_actions::{find_by_name_like as find_keyword_by_name_like};
use crate::keyword_actions::add_by_file_and_folder as add_keywords;
use crate::img_actions::add_by_parents as add_img;
use crate::img_actions::del_by_folder_id as del_img;
use crate::params::AddFolderParam;
use crate::util::vecs;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn atomic_add(
  conn: &mut SqliteConnection,
  add_name: String,
  add_style: Style
) -> Folder {
  let add_style_name2 = add_style.name.clone();
  let add_name2 = add_name.clone();

  let po_opt = find_by_name_and_style(conn, add_name, add_style.id.unwrap());
  if po_opt.is_some() {
    return po_opt.unwrap();
  }

  let mut vo = Folder { name: add_name2, style_id: add_style.id.unwrap(), style_name: add_style_name2, click: 0, id: None };
  insert_into(folder).values(&vo).execute(conn).unwrap();

  let id_opt = folder.select(max(schema::folder::columns::id)).first::<Option<i32>>(conn).unwrap();
  vo.id = id_opt;
  vo
}

pub fn find_by_name_and_style(
  conn: &mut SqliteConnection,
  add_name: String,
  add_style_id: i32,
) -> Option<Folder> {
  folder.filter(schema::folder::columns::name.eq(add_name))
    .filter(schema::folder::columns::style_id.eq(add_style_id))
    .first::<Folder>(conn)
    .optional()
    .unwrap()
}

pub fn list_by_style(
  conn: &mut SqliteConnection,
  list_style_id: i32,
) -> Vec<Folder> {
  folder
    .filter(schema::folder::columns::style_id.eq(list_style_id))
    .load::<Folder>(conn)
    .unwrap()
}

pub fn click(conn: &mut SqliteConnection, click_id: i32, auto: bool) {
  let plus = if auto { 1 } else { 2 };
  let po_opt = folder.filter(schema::folder::columns::id.eq(Some(click_id)))
    .first::<Folder>(conn)
    .optional()
    .unwrap();
  match po_opt {
    Some(po) => {
      let r = diesel::update(folder.filter(schema::folder::columns::id.eq(po.id)))
        .set(schema::folder::columns::click.eq(po.click + plus))
        .execute(conn);
      match r {
        Ok(_) => {}
        Err(e) => {
          log::error!("folder click err: {}", e.to_string());
        }
      }
    }
    None => ()
  };
}

pub fn open(conn: &mut SqliteConnection, open_folder_id: i32)  {
  let po = get(conn, open_folder_id).expect("folder不存在");
  let style_po = style_actions::get(conn, po.style_id).expect("style不存在");
  let asset_path_buf = configs::get_asset_path_buf();
  let folder_path_buf = asset_path_buf.join(style_po.name).join(po.name);
  if cfg!(windows) {
    Command::new("explorer")
      .args(&[&folder_path_buf])
      .output()
      .expect("打开文夹失败");
  } else if cfg!(target_os = "linux") {
    Command::new("xdg-open")
      .args(&[&folder_path_buf])
      .output()
      .expect("打开文夹失败");
  } else {
    
  }
}

pub fn del(conn: &mut SqliteConnection, del_name: String, del_style_name: String) {
  log::info!("删除folder, style_name:{}, folder_name:{}", del_style_name, del_name);
  // 查出匹配id
  let id_opt_list: Vec<Option<i32>> = folder.filter(schema::folder::columns::name.eq(del_name))
    .filter(schema::folder::columns::style_name.eq(del_style_name))
    .select(schema::folder::columns::id)
    .load::<Option<i32>>(conn)
    .unwrap();
  del_by_ids(conn, &id_opt_list);
}

fn del_by_ids(conn: &mut SqliteConnection, id_opt_list: &Vec<Option<i32>>) {
  // 从folder表删除
  diesel::delete(
    folder.filter(schema::folder::columns::id.eq_any(id_opt_list)))
    .execute(conn)
    .unwrap();
  // 从keyword_folder_r表删除
  let id_list: Vec<i32> = id_opt_list.clone().iter().map(|id_opt| id_opt.unwrap()).collect();
  diesel::delete(
    keyword_folder_r
      .filter(schema::keyword_folder_r::columns::folder_id.eq_any(id_list)))
    .execute(conn)
    .unwrap();

  id_opt_list.iter().map(|id_opt| id_opt.unwrap()).for_each(|folder_id| del_img(conn, folder_id));
}

pub fn del_by_style_id(conn: &mut SqliteConnection, style_id: i32) {
  let id_opt_list = folder.filter(schema::folder::columns::style_id.eq(style_id))
    .select(schema::folder::columns::id)
    .load::<Option<i32>>(conn).unwrap();
  del_by_ids(conn, &id_opt_list);
}

pub fn clean(conn: &mut SqliteConnection) {
  diesel::delete(folder).execute(conn).unwrap();
}

// 根据关键词查询folder
pub fn list_by_key(conn: &mut SqliteConnection, multi_keyword: String, l_style_id: i32) -> Vec<Folder> {
  // 入参切分为多字符串, 没有入参时返回全部folder
  let keyword_str_list: Vec<String> = multi_keyword
    .split(" ")
    .into_iter()
    .map(|k| k.replace("?", "？"))
    .collect();
  if keyword_str_list.len() == 0 {
    return folder.filter(schema::folder::columns::style_id.eq(l_style_id))
      .order_by(schema::folder::columns::click.desc())
      .load::<Folder>(conn)
      .unwrap();
  }

  // 模糊查keyword todo 按匹配度排序?
  let mut keywords = Vec::new();
  for keyword_str in &keyword_str_list {
    let mut liked_keyword_pos = find_keyword_by_name_like(conn, keyword_str.clone(), l_style_id);
    if liked_keyword_pos.len() > 0 {
      keywords.append(&mut liked_keyword_pos)
    }
  }
  if keywords.len() == 0 { return Vec::new(); }

  // 找到和入参相等的keyword 并以输入顺序排序
  let mut matched_keywords: Vec<Keyword> = keywords.iter()
    .filter(|keyword_po| keyword_str_list.contains(&keyword_po.name.clone()))
    .map(|keyword_po| keyword_po.clone())
    .collect();
  // todo 都不匹配时按相似度排序
  matched_keywords.sort_by(as_fn(comparing(
    |o: &Keyword| vecs::index_of(&keyword_str_list, &o.name).unwrap_or(9999999)
  )));

  // 找到和入参相等的keyword对应的folderId
  let mut matched_folder_ids = Vec::new();
  if matched_keywords.len() > 0 {
    let matched_keyword_ids: Vec<i32> = matched_keywords.iter()
      .map(|keyword_po| keyword_po.id.unwrap())
      .collect();
    let mut matched_keyword_folder_r_list: Vec<KeywordFolderR> = keyword_folder_r
      .filter(schema::keyword_folder_r::columns::keyword_id.eq_any(matched_keyword_ids.clone()))
      .filter(schema::keyword_folder_r::columns::style_id.eq(l_style_id))
      .load::<KeywordFolderR>(conn)
      .unwrap();
    matched_keyword_folder_r_list.sort_by(as_fn(comparing(
      |o: &KeywordFolderR| vecs::index_of(&matched_keyword_ids, &o.keyword_id).unwrap_or(9999999)
    )));

    matched_folder_ids = matched_keyword_folder_r_list.iter()
      .map(|matched_keyword_folder_r| matched_keyword_folder_r.folder_id)
      .collect();
  }

  // 获取folder关系
  let keyword_ids: Vec<i32> = keywords.iter().map(|keyword_po| keyword_po.id.unwrap()).collect();
  let keyword_folder_r_pos: Vec<KeywordFolderR> = keyword_folder_r
    .filter(schema::keyword_folder_r::columns::keyword_id.eq_any(keyword_ids))
    .filter(schema::keyword_folder_r::columns::style_id.eq(l_style_id))
    .load::<KeywordFolderR>(conn).unwrap();
  let folder_ids: Vec<i32> = keyword_folder_r_pos.iter()
    .map(|r_po| r_po.folder_id)
    // .dedup() // todo 去重
    .collect();

  // 把和入参相等的folder放在最前边
  let mut folder_pos = folder.filter(schema::folder::columns::id.eq_any(folder_ids))
    .order_by(schema::folder::columns::click.desc())
    .load::<Folder>(conn)
    .unwrap();
  folder_pos.sort_by(as_fn(comparing(
    |o: &Folder| vecs::index_of(&matched_folder_ids, &o.id.unwrap()).unwrap_or(9999999)
  )));
  return folder_pos;
}

pub fn add_with_keyword(conn: &mut SqliteConnection, params: AddFolderParam) {
  let style_po = style_actions::get(conn, params.styleId).expect("style 不存在");
  let folder_po = atomic_add(conn, params.name.clone(), style_po.clone());
  keyword_actions::add(conn, folder_po.name.clone(), folder_po.style_id, folder_po.id.unwrap());

  let asset_path_buf = configs::get_asset_path_buf();
  let folder_path_buf = asset_path_buf.join(&style_po.name).join(&folder_po.name);
  fs::create_dir_all(&folder_path_buf).unwrap();
  
  if let Some(add_keyword) = params.keyword {
    let add_keyword = add_keyword.trim();
    if add_keyword.len() == 0 { return }
    let keyword_path_buf = folder_path_buf.join("keyword.txt");
    let mut keyword_fle = File::create(&keyword_path_buf).unwrap();
    keyword_fle.write_all(add_keyword.as_bytes()).unwrap();
    keyword_actions::add_by_file_and_folder(conn, &keyword_path_buf, folder_po, false);
  }
}

pub fn add_with_childs(conn: &mut SqliteConnection,
                       add_path: &std::path::Path,
                       add_style: Style, clean_keyword: bool) {
  log::info!("创建folder, style_name:{}, folder_path:{:?}", add_style.name, &add_path);
  let add_name = add_path.file_name().unwrap();
  let folder_po = atomic_add(conn, String::from(add_name.to_str().unwrap()), add_style.clone());
  keyword_actions::add(conn, String::from(folder_po.name.clone()), folder_po.style_id, folder_po.id.unwrap());
  WalkDir::new(add_path).max_depth(1).min_depth(1).into_iter().filter_map(|r| r.ok())
    .filter(|dir_entry| dir_entry.metadata().is_ok())
    .for_each(|dir_entry| {
      let child_path = dir_entry.path();
      let is_txt = child_path.extension().map_or(false, |ext| ext == "txt");
      if is_txt {
        add_keywords(conn, child_path, folder_po.clone(), clean_keyword);
      } else {
        add_img(conn, add_style.clone(), folder_po.clone(), String::from(dir_entry.file_name().to_str().unwrap()));
      }
    })
}

pub fn get(conn: &mut SqliteConnection, get_id: i32) -> Option<Folder> {
  folder
    .filter(schema::folder::columns::id.eq(get_id))
    .first::<Folder>(conn)
    .optional()
    .unwrap()
}

pub fn del_data_by_style_id(conn: &mut SqliteConnection, del_style_id: i32) {
  let pos: Vec<Folder> = folder.filter(schema::folder::columns::style_id.eq(del_style_id))
    .load::<Folder>(conn)
    .unwrap();
  if pos.len() == 0 { return; }
  
  let del_ids: Vec<Option<i32>> = pos.iter().map(|po| po.id).collect();

  for po in pos {
    let po_id = po.id.unwrap();
    img_actions::del_data_by_folder_id(conn, po_id);
    keyword_actions::del_data_by_folder_id(conn, po_id)
  }

  log::info!("删除folder:{:?}", &del_ids);
  diesel::delete(folder.filter(
    schema::folder::columns::id.eq_any(&del_ids))
  ).execute(conn).unwrap();
}

pub fn clean_invalid_by_style(conn: &mut SqliteConnection, clean_style: &Style) {
  let pos = list_by_style(conn, clean_style.id.unwrap());
  if pos.len() == 0 { return; }

  let asset_path_buf = configs::get_asset_path_buf();
  let style_path_buf = asset_path_buf.join(&clean_style.name);
  for po in pos {
    let po_id = po.id.unwrap();
    let po_path_buf = style_path_buf.clone().join(&po.name);
    let file_exist = fs::metadata(po_path_buf).is_ok();
    if file_exist {
      img_actions::clean_invalid_by_folder(conn, &po);
      // keyword 不清理 keyword_fold_r在init_db处理txt文件时清理
    } else {
      img_actions::del_data_by_folder_id(conn, po_id);
      keyword_actions::del_data_by_folder_id(conn, po_id);
      log::info!("删除style:{}", po.id.unwrap());
      diesel::delete(folder.filter(
        schema::folder::columns::id.eq(po.id))
      ).execute(conn).unwrap();
    }
  }
}
