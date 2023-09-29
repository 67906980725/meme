use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::style)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Style {
  pub id: Option<i32>,
  pub name: String,
  pub sort: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::folder)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Folder {
  pub id: Option<i32>,
  pub name: String,
  pub click: i32,
  pub style_id: i32,
  pub style_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::keyword)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Keyword {
  pub id: Option<i32>,
  pub name: String,
  pub style_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::keyword_folder_r)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct KeywordFolderR {
  pub id: Option<i32>,
  pub keyword_id: i32,
  pub folder_id: i32,
  pub style_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::img)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Img {
  pub id: Option<i32>,
  // 相对asset目录的路径
  pub path: String,
  pub click: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::img_folder_r)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ImgFolderR {
  pub id: Option<i32>,
  pub img_id: i32,
  pub folder_id: i32,
}
