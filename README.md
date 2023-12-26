# mini web server(Rust)
* Youtube で見たC言語の最小限Webサーバ[^1]を練習がてらRustで実装する

## プログラム[^2]の基本的な動作
1. IPv4のTCPソケットを作成
2. IPアドレスとポートを指定しクライアントからの接続を待機
3. クライアントからの接続後HTTPリクエストを受信
4. ファイルをクライアントに送信してプログラム終了

## 実行方法
```bash
/mini_web_server> cargo run  
```
## 使用方法
* cargo run の後同じマシンのコマンドラインで以下を実行
```bash
> wget localhost:8080/index.html
```
* index.htmlをダウンロード可能

## 引用元
[^1]: [Making Minimalist Web Server in C on Linux](https://www.youtube.com/watch?v=2HrYIl6GpYg)
[^2]: [nir9/server.c](https://gist.github.com/nir9/3d22d954a599a71c1ccf64ea63c4e38f)