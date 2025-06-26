//! コマンドライン引数の処理

use clap::Parser;
use std::path::PathBuf;

/// コマンドライン引数
#[derive(Parser, Debug)]
#[command(version, author, about)]
pub struct Args {
    /// 入力ファイル
    pub file_name: Option<PathBuf>,
    /// ログの出力を行う
    #[arg(short, long)]
    pub log: bool,
    /// デバッグログの出力を行う(--logより優先する)
    #[arg(short, long)]
    pub debug: bool,
}
