// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod args;
mod document;
mod utils;

use clap::Parser;
use document::Document;
use rfd::FileDialog;
use tauri::{AppHandle, Emitter, Manager, State};

fn main() {
    let args = args::Args::parse();
    if args.log || args.debug {
        init_log(args.debug);
    }
    let doc = Document::new();
    if let Some(file_name) = args.file_name {
        match file_name.try_exists() {
            Ok(exists) => {
                if !exists {
                    log::error!("指定されたファイルが存在しません。{:?}", file_name);
                    std::process::exit(1);
                }
            }
            Err(e) => {
                log::error!("ファイルの存在確認に失敗しました。OSエラー:{}", e);
                std::process::exit(1);
            }
        }
        doc.set_file_path(file_name);
    }
    match doc.file_path() {
        Some(path) => log::info!("ファイル名: {:?}", path),
        None => log::info!("ファイル名は指定されていません。"),
    }

    run(doc)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(document: document::Document) {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(document)
        .menu(build_menu)
        .on_menu_event(menu_event_handler)
        .invoke_handler(tauri::generate_handler![contents, choose_file,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use tauri::menu::{Menu, MenuEvent, MenuId, MenuItem, Submenu};
fn build_menu<R: tauri::Runtime, M: tauri::Manager<R>>(manager: &M) -> tauri::Result<Menu<R>> {
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

fn menu_event_handler<R: tauri::Runtime>(app: &AppHandle<R>, event: MenuEvent) {
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

#[tauri::command]
async fn contents(document: State<'_, Document>) -> Result<String, String> {
    document.html_contents().await.map_err(|e| {
        log::error!("HTMLコンテンツの取得に失敗: {}", e);
        e.to_string()
    })
}

#[tauri::command]
fn choose_file<R: tauri::Runtime>(app: tauri::AppHandle<R>) {
    log::info!("ファイル選択ダイアログオープン");
    let window = app.get_webview_window("main").unwrap();
    let cwd = std::env::current_dir().unwrap_or_else(|_| {
        log::warn!("カレントディレクトリの取得に失敗。\".\"を使用します。");
        std::path::PathBuf::from(".")
    });
    let file = FileDialog::new()
        .set_directory(cwd)
        .set_parent(&window)
        .pick_file();
    if let Some(path) = file {
        log::info!("選択されたファイル: {:?}", path);
        let document = app.state::<Document>();
        document.set_file_path(path);
        if let Err(err) = app.emit("invalid_content", ()) {
            log::error!("イベントの送信に失敗: {}", err);
        }
    }
}

/// ロギング機構の初期設定
/// ログファイルは、設定ディレクトリ内の`mdviewer.log`及び、標準エラー出力に出力される。
fn init_log(debug: bool) {
    let mut log_file_name = utils::config_path();
    log_file_name.push("mdviewer.log");
    let log_file = fern::log_file(log_file_name).expect("Failed to create log file");
    let log_level = if debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}:{} {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stderr())
        .chain(log_file)
        .apply()
        .expect("Failed to initialize logging");
}
