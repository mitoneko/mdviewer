//! Tauriコマンドの実装

use rfd::FileDialog;
use tauri::{AppHandle, Emitter, Manager, State};

use crate::document::Document;

/// MDファイルの内容をhtmlに変換して取得
#[tauri::command]
pub async fn contents(document: State<'_, Document>) -> Result<String, String> {
    document.html_contents().await.map_err(|e| {
        log::error!("HTMLコンテンツの取得に失敗: {}", e);
        e.to_string()
    })
}

/// MDファイルのパスを取得する
/// ファイル選択ダイアログの表示とDocumentのファイルパスの更新
#[tauri::command]
pub fn choose_file<R: tauri::Runtime>(app: AppHandle<R>) {
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
        match document.set_file_path(path) {
            Ok(_) => {
                if let Err(e) = app.emit("invalid_content", ()) {
                    log::error!("イベントの送信に失敗: {}", e);
                }
            }
            Err(e) => {
                log::error!("ファイルパスが不正です。: {}", e);
            }
        }
    }
}
