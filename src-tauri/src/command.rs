//! Tauriコマンドの実装

use rfd::FileDialog;
use tauri::{AppHandle, Emitter, Manager, State};

use crate::document::Document;

/// MDファイルの内容をhtmlに変換して取得
#[tauri::command]
pub async fn contents(document: State<'_, Document>) -> Result<String, String> {
    log::debug!("ファイル監視解除(コンテンツ取得時)");
    document.unregist_watch().unwrap_or_else(|e| {
        log::warn!("ファイル監視の解除に失敗: {}", e);
    });
    let response = document.html_contents().await.map_err(|e| {
        log::error!("HTMLコンテンツの取得に失敗: {}", e);
        e.to_string()
    });
    log::debug!("ファイル監視登録(コンテンツ取得後)");
    document.regist_watch().unwrap_or_else(|e| {
        log::warn!("ファイル監視の登録に失敗: {}", e);
    });
    response
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
        log::debug!("ファイル監視解除(ファイル選択時)");
        document.unregist_watch().unwrap_or_else(|e| {
            log::warn!("ファイル監視の解除に失敗: {}", e);
        });
        match document.set_file_path(path) {
            Ok(_) => {
                log::debug!("コンテンツ無効信号を送信(ファイル選択後)");
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
