// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod document;
mod utils;

fn main() {
    init_log();
    log::info!("logging started");
    run()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    eprintln!("Tauri application is starting...");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![contents,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn contents() -> String {
    let markdown = r#"
# タイトル1
## タイトル2
　こんな感じで打ったらどうよ。preでないでね。

* 箇条書きもしてみる。
* もう一つ。

さて、**強調**はどう?ちゃんと*斜体*もでるかしら?
    "#;
    let parser = pulldown_cmark::Parser::new_ext(markdown, pulldown_cmark::Options::all());
    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);
    output
}

/// ロギング機構の初期設定
/// ログファイルは、設定ディレクトリ内の`mdviewer.log`及び、標準エラー出力に出力される。
fn init_log() {
    let mut log_file_name = utils::config_path();
    log_file_name.push("mdviewer.log");
    let log_file = fern::log_file(log_file_name).expect("Failed to create log file");

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
        .level(log::LevelFilter::Debug)
        .chain(std::io::stderr())
        .chain(log_file)
        .apply()
        .expect("Failed to initialize logging");
}
