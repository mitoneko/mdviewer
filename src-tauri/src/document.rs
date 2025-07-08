//! 現在扱っているMDファイルの操作
use notify::{event::ModifyKind, recommended_watcher, Event, Watcher};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

pub struct Document {
    file_path: Mutex<Option<PathBuf>>,
    notify: Mutex<notify::RecommendedWatcher>,
}

impl Document {
    /// MDファイルの内容を取得する。
    pub async fn md_contents(&self) -> Result<String, DocumentError> {
        let file_path = self.file_path();
        match file_path {
            Some(path) => {
                let f = File::open(path).await?;
                let reader = BufReader::new(f);
                let mut contents = String::new();
                let mut lines = reader.lines();
                while let Some(line) = lines.next_line().await? {
                    contents.push_str(&line);
                    contents.push('\n');
                }
                Ok(contents)
            }
            None => Ok("".to_string()),
        }
    }

    /// html変換済みドキュメントを取得する。
    pub async fn html_contents(&self) -> Result<String, DocumentError> {
        let md_contents = self.md_contents().await?;
        let parser_options = pulldown_cmark::Options::all();
        let parser = pulldown_cmark::Parser::new_ext(&md_contents, parser_options);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        Ok(html_output)
    }

    /// ドキュメントのファイルパスを設定
    pub fn set_file_path<P: AsRef<Path>>(&self, path: P) -> Result<(), DocumentError> {
        let path: &Path = path.as_ref();
        file_path_exists(path)?;

        let mut file_path = self.file_path.lock().unwrap_or_else(|mut e| {
            **e.get_mut() = None;
            self.file_path.clear_poison();
            e.into_inner()
        });
        *file_path = Some(path.to_path_buf());

        Ok(())
    }

    /// ドキュメントのファイルパスを取得
    fn file_path(&self) -> Option<PathBuf> {
        let file_path = self.file_path.lock().unwrap_or_else(|mut e| {
            **e.get_mut() = None;
            self.file_path.clear_poison();
            e.into_inner()
        });
        file_path.clone()
    }

    /// ファイルの監視を登録する。
    /// ファイルパスが未設定の場合は、何もしない。
    pub fn regist_watch(&self) -> Result<(), DocumentError> {
        let Some(file_path) = self.file_path() else {
            return Ok(());
        };
        self.notify
            .lock()
            .map_err(|_| DocumentError::NotifyLockPoisoned)?
            .watch(&file_path, notify::RecursiveMode::NonRecursive)?;
        log::debug!("notify: ファイル監視を登録しました: {:?}", file_path);
        Ok(())
    }

    /// ファイル監視を解除する。
    /// ファイルパスが未設定の場合は、何もしない。
    /// unwatchが、ファイル未発見の場合は、無視する。(Ok(())を返す)
    pub fn unregist_watch(&self) -> Result<(), DocumentError> {
        let Some(file_path) = self.file_path() else {
            return Ok(());
        };
        let res = self
            .notify
            .lock()
            .map_err(|_| DocumentError::NotifyLockPoisoned)?
            .unwatch(&file_path);
        match res {
            Ok(_) => {
                log::debug!("notify: ファイル監視を解除しました: {:?}", file_path);
                Ok(())
            }
            Err(e) => {
                match e.kind {
                    notify::ErrorKind::WatchNotFound => {
                        // ファイル未発見の場合は、スルーする
                        log::debug!("notify: 監視がありません。スルーします。");
                        Ok(())
                    }
                    _ => {
                        log::error!("notify: ファイル監視の解除に失敗: {}", e);
                        Err(DocumentError::WatchRegistrationFailed(e))
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct DocumentBuilder {
    file_path: Option<PathBuf>,
}

impl DocumentBuilder {
    pub fn set_file_path<P: AsRef<Path>>(mut self, path: P) -> Result<Self, DocumentError> {
        let path: &Path = path.as_ref();
        file_path_exists(path)?;
        self.file_path = Some(path.to_path_buf());
        Ok(self)
    }

    pub fn build(self, app: &AppHandle) -> Result<Document, DocumentError> {
        // 新規notifyの生成
        let new_notify = recommended_watcher(NotifyHandler::new(app.clone()))
            .map_err(DocumentError::NotifyInitializationFailed)?;
        log::debug!("notify: 初期化完了");

        let new_document = Document {
            file_path: Mutex::new(self.file_path),
            notify: Mutex::new(new_notify),
        };
        new_document.regist_watch()?;
        Ok(new_document)
    }
}

/// notifyのイベントハンドラー
struct NotifyHandler {
    app: AppHandle,
}

impl NotifyHandler {
    fn new(app: AppHandle) -> Self {
        NotifyHandler { app }
    }

    fn emit_invalid_content(&self) {
        self.app.emit("invalid_content", ()).unwrap_or_else(|e| {
            log::error!("notify: イベント送信に失敗: {}", e);
        });
    }
}

impl notify::EventHandler for NotifyHandler {
    fn handle_event(&mut self, event: notify::Result<Event>) {
        match event {
            Ok(event) => {
                log::debug!("notify: イベント発生: {:?}", event);
                match event.kind {
                    notify::EventKind::Modify(ModifyKind::Data(_)) => {
                        log::debug!("コンテンツ無効信号を送信(ModifyKind::Data)");
                        self.emit_invalid_content();
                        log::debug!("notify: イベント送信完了");
                    }
                    notify::EventKind::Remove(_) => {
                        log::debug!("コンテンツ無効信号を送信(Remove)");
                        log::debug!("notify: Removeイベント開始");
                        self.emit_invalid_content();
                    }
                    _ => {}
                }
            }
            Err(e) => log::error!("notify: エラーイベント発生: {}", e),
        }
    }
}

/// ファイルパスの存在チェックを行う。
///
/// # Errors
///
/// ファイルが存在しない・ファイルアクセスにエラーが発生した場合、Errを返す。
///
fn file_path_exists<P: AsRef<Path>>(path: P) -> Result<(), DocumentError> {
    let path: &Path = path.as_ref();
    match path.try_exists() {
        Ok(exists) => {
            if !exists {
                Err(DocumentError::FileNotFound(
                    path.to_string_lossy().to_string(),
                ))
            } else {
                Ok(())
            }
        }
        Err(e) => Err(DocumentError::IoError(e)),
    }
}

/// ドキュメント操作に関するエラー
#[derive(thiserror::Error, Debug)]
pub enum DocumentError {
    #[error("ioエラー発生: {0}")]
    IoError(#[from] std::io::Error),
    #[error("ファイルが見つかりません: {0}")]
    FileNotFound(String),
    #[error("notifyはすでに初期化済みです。")]
    NotifyAlreadyInitialized,
    #[error("notifyの初期化に失敗")]
    NotifyInitializationFailed(#[source] notify::Error),
    #[error("notifyのロック破損")]
    NotifyLockPoisoned,
    #[error("notifyが初期化されていません。")]
    NotifyNotInitialized,
    #[error("ファイル監視の登録に失敗: {0}")]
    WatchRegistrationFailed(#[from] notify::Error),
}
