use std::time::Duration;
use diesel::SqliteConnection;
use notify::event::{ModifyKind, RenameMode};
use notify::{EventKind, RecursiveMode, Watcher};
use notify_debouncer_full::{DebouncedEvent, new_debouncer};
use crate::{configs, folder_actions, img_actions, keyword_actions, style_actions};
use crate::util::{files, path_bufs};

// 文件监听
// 监听图片资源目录, 同步变更数据库
pub async fn async_watch() -> notify::Result<()> {
  let asset_path_buf = configs::get_asset_path_buf();
  log::info!("async_watch: {:?}", asset_path_buf);

  let (tx, rx) = std::sync::mpsc::channel();
  let sync_delay = configs::get_sync_delay();
  let mut debouncer = new_debouncer(Duration::from_secs(sync_delay), None, tx)?;
  debouncer
    .watcher()
    .watch(asset_path_buf.as_ref(), RecursiveMode::Recursive)?;

  for result in rx {
    match result {
      Ok(events) => events.iter().for_each(|event| {
        log::info!("{event:?}");
        let mut conn = configs::establish_connection();
        process_event(&mut conn, event);
        drop(conn);
      }),
      Err(errors) => errors.iter().for_each(|error| log::error!("{error:?}")),
    }
  }

  Ok(())
}

fn process_event(conn: &mut SqliteConnection, event: &DebouncedEvent) {
  let asset_path_buf = configs::get_asset_path_buf();
  let abs_asset_path_buf = path_bufs::to_absolute(asset_path_buf);
  let asset_path = abs_asset_path_buf.as_path();

  let event_path_buf = event.paths[0].clone();
  let event_file_name = String::from(event_path_buf.file_name().unwrap().to_str().unwrap());

  let parent = event_path_buf.parent().unwrap();
  let parent_name = String::from(parent.file_name().unwrap().to_str().unwrap());
  let parent_parent = parent.parent().unwrap();
  let parent_parent_name = String::from(parent_parent.file_name().unwrap().to_str().unwrap());
  let parent_parent_parent = parent_parent.parent().unwrap();

  let is_txt = files::is_txt(&event_path_buf);
  let is_style = parent.eq(asset_path) && !is_txt;
  let is_folder = parent_parent.eq(asset_path) && !is_txt;
  let is_order = parent_parent.eq(asset_path) && is_txt;
  let is_img = parent_parent_parent.eq(asset_path) && !is_txt;
  let is_keyword = parent_parent_parent.eq(asset_path) && is_txt;

  let mut style_name_opt = None;
  let mut folder_name_opt = None;

  if is_keyword || is_img {
    style_name_opt = Some(parent_parent_name.clone());
    folder_name_opt = Some(parent_name.clone());
  } else if is_folder || is_order {
    style_name_opt = Some(parent_name.clone());
    folder_name_opt = Some(event_file_name.clone());
  } else if is_style {
    style_name_opt = Some(event_file_name.clone());
  }

  let mut style_opt = None;
  if style_name_opt.is_some() {
    style_opt = style_actions::find_by_name(conn, style_name_opt.clone().unwrap());
  }
  let mut folder_opt = None;
  if folder_name_opt.is_some() && style_opt.is_some(){
    folder_opt = folder_actions::find_by_name_and_style(conn, folder_name_opt.clone().unwrap(), style_opt.clone().unwrap().id.unwrap());
  }

  match event.kind {
    EventKind::Remove(_) | EventKind::Modify(ModifyKind::Name(RenameMode::From)) => {
      // EventKind::Remove(RemoveKind::Any) // 只有当事文件(夹会有)
      // 文件: 删除图片 目录: 删除folder/style,逐级删除 txt: 默认txt只更新不删除
      if is_img {
        let img_path = format!("{}/{}/{}", &parent_parent_name, &parent_name, &event_file_name);
        img_actions::del(conn, img_path);
      } else if is_folder {
        folder_actions::del(conn, event_file_name, parent_name.clone());
      } else if is_style {
        style_actions::del(conn, event_file_name);
      }
    }
    EventKind::Create(_) | EventKind::Modify(ModifyKind::Name(RenameMode::To)) | EventKind::Modify(ModifyKind::Any) => {
      // EventKind::Create(CreateKind::Any) // 所有新增文件都会有
      // 文件: 新增图片 目录: 逐级新增
      if is_keyword {
        if folder_opt.is_some() {
          keyword_actions::add_by_file_and_folder(conn, &event_path_buf, folder_opt.unwrap(), true);
        }
        return;
      } else if is_img {
        if style_opt.is_some() && folder_opt.is_some() {
          img_actions::add_by_parents(conn, style_opt.unwrap(), folder_opt.unwrap(), event_file_name);
        }
        return;
      } else if is_order {
        if style_opt.is_some() {
          style_actions::set_sort(conn, &event_path_buf, style_opt.unwrap());
        }
        return;
      } else if is_folder {
        if style_opt.is_some() {
          folder_actions::add_with_childs(conn, &event_path_buf, style_opt.unwrap(), false);
        }
        return;
      } else if is_style {
        style_actions::add_with_childs(conn, &event_path_buf, true);
        return;
      }
    }
    EventKind::Modify(_) => {}
    EventKind::Access(_) => {}
    EventKind::Any => {}
    EventKind::Other => {}
  }
}
