//! アプリケーションメニューの実装

use tauri::{
    menu::{Menu, MenuEvent, MenuId, MenuItem, Submenu},
    AppHandle, Manager,
};

use crate::command::choose_file;

/// アプリケーションメニューの構築
pub fn build_menu<R: tauri::Runtime, M: Manager<R>>(manager: &M) -> tauri::Result<Menu<R>> {
    let menu = Menu::new(manager)?;
    let menu_file = Submenu::new(manager, "ファイル", true)?;
    let menu_item_open = MenuItem::with_id(
        manager,
        MenuId::new("open_file"),
        "ファイルを開く",
        true,
        Some("Ctrl+O"),
    )?;
    let menu_item_quit = MenuItem::with_id(manager, MenuId::new("quit"), "終了", true, Some("Q"))?;
    menu_file.append_items(&[&menu_item_open, &menu_item_quit])?;
    menu.append(&menu_file)?;
    Ok(menu)
}

/// メニューイベントハンドラの実装
pub fn menu_event_handler<R: tauri::Runtime>(app: &AppHandle<R>, event: MenuEvent) {
    match event.id().0.as_str() {
        "quit" => {
            log::info!("アプリケーションを終了します。");
            app.exit(0);
        }
        "open_file" => {
            log::debug!("ファイルのオープン処理を開始します。(メニューから)");
            let app = app.clone();
            choose_file(app);
        }
        _ => {}
    }
}
