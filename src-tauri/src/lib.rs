// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
