// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod args;
mod document;
mod utils;

use clap::Parser;
use document::Document;
use tauri::State;

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
        .invoke_handler(tauri::generate_handler![contents,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn contents(document: State<'_, Document>) -> Result<String, String> {
    document.html_contents().await.map_err(|e| {
        log::error!("HTMLコンテンツの取得に失敗: {}", e);
        e.to_string()
    })
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
