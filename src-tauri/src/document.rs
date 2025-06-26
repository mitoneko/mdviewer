//! 現在扱っているMDファイルの操作
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

pub struct Document {
    file_path: Mutex<Option<PathBuf>>,
}

impl Document {
    /// 新規ドキュメントの生成
    pub fn new() -> Self {
        Self::default()
    }

    /// ドキュメントのファイルパスを設定
    pub fn set_file_path<P: AsRef<Path>>(&self, path: P) -> Result<(), DocumentError> {
        let path: &Path = path.as_ref();
        self.file_path_exists(path)?;

        let mut file_path = self.file_path.lock().unwrap_or_else(|mut e| {
            **e.get_mut() = None;
            self.file_path.clear_poison();
            e.into_inner()
        });
        *file_path = Some(path.to_path_buf());
        Ok(())
    }

    /// ドキュメントのファイルパスを取得
    pub fn file_path(&self) -> Option<PathBuf> {
        let file_path = self.file_path.lock().unwrap_or_else(|mut e| {
            **e.get_mut() = None;
            self.file_path.clear_poison();
            e.into_inner()
        });
        file_path.clone()
    }

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

    /// ファイルパスの存在チェックを行う。
    fn file_path_exists<P: AsRef<Path>>(&self, path: P) -> Result<(), DocumentError> {
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
}

impl Default for Document {
    fn default() -> Self {
        Document {
            file_path: Mutex::new(None),
        }
    }
}

/// ドキュメント操作に関するエラー
#[derive(thiserror::Error, Debug)]
pub enum DocumentError {
    #[error("ioエラー発生: {0}")]
    IoError(#[from] std::io::Error),
    #[error("ファイルが見つかりません: {0}")]
    FileNotFound(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_document() {
        let doc = Document::new();
        assert!(doc.file_path().is_none());
    }

    #[test]
    fn test_set_and_get_file_path() {
        let doc = Document::new();
        let path = PathBuf::from("test.md");
        doc.set_file_path(&path);
        assert_eq!(doc.file_path(), Some(path));
    }
}
