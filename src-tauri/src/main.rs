// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod args;
mod command;
mod document;
mod menu;
mod utils;

use clap::Parser;

use command::{choose_file, contents};
use document::Document;
use menu::{build_menu, menu_event_handler};

fn main() {
    let args = args::Args::parse();
    if args.log || args.debug {
        init_log(args.debug);
    }
    let doc = Document::new();
    if let Some(file_name) = args.file_name {
        if let Err(e) = doc.set_file_path(file_name) {
            log::error!("引数のファイル名が不正です。: {}", e);
            eprintln!("引数のファイル名が不正です。: {}", e);
            std::process::exit(1);
        }
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
