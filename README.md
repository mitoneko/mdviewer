# MDファイルビューア

## 概要

マークダウンファイルの単機能な閲覧アプリです。

ファイルが更新された場合、自動的に再読込を行います。
編集機能はありません。

## 起動

```bash
$ mdviewer -h
MDファイルビューア

Usage: mdviewer [OPTIONS] [FILE_NAME]

Arguments:
  [FILE_NAME]  入力ファイル

Options:
  -l, --log      ログの出力を行う
  -d, --debug    デバッグログの出力を行う(--logより優先する)
  -h, --help     Print help
  -V, --version  Print version
```

基本的に、ファイル名を指定して実行すれば、内容を表示します。

ログの表示を指定されると、標準エラー出力と、 ```./.config/mdviewer/mdviewer.log``` にログが出力されます。
