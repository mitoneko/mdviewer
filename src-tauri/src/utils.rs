//! ユーティリティー関数たち

use std::path::PathBuf;

/// アプリケーション用の設定ファイルのディレクトリを取得する。
/// もし、なければ、ディレクトリを生成する。
pub fn config_path() -> PathBuf {
    let project_dirs =
        directories::ProjectDirs::from("jp", "laki", "mdviewer").expect("Failed get project dirs");
    let path = project_dirs.config_dir().to_path_buf();
    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create config directory");
    }
    path
}
