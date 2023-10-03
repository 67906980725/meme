use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FolderParam {
  pub styleId: i32,
  pub keyword: String,
}
#[derive(Serialize, Deserialize)]
pub struct AddStyleParam {
  pub name: String,
  pub sort: i32,
}
#[derive(Serialize, Deserialize)]
pub struct AddFolderParam {
  pub styleId: i32,
  pub name: String,
  pub keyword: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct ClickParam {
  pub id: i32,
  pub auto: i32,
}
#[derive(Serialize, Deserialize)]
pub struct IdParam {
  pub id: i32,
}
#[derive(Serialize, Deserialize)]
pub struct CpImgParam {
  pub path: String,
}
