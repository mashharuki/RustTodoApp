/**
 * メインファイル
 */

 // 必要なモジュール読み込み
use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;
use askama::Template; 

// Todoリスト用のエントリ
struct TodoEntry {
    id: u32,
    text: String,
}

// テンプレートにデータを埋め込むための変数
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

// エラー用の列挙型変数
#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}

// Myerrorを継承したインターフェース
impl ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    // 埋め込み用の変数を生成する。
    let mut entries = Vec::new();
    // 変数を代入する。
    entries.push(TodoEntry {
        id: 1,
        text: "First entry".to_string(),
    });
    entries.push(TodoEntry {
        id: 2,
        text: "Second entry".to_string(),
    });
    // htmlを読み込む
    let html = IndexTemplate{entries};
    // body部を作成
    let response_body = html.render()?;
    // レスポンスの内容
    Ok(HttpResponse::Ok().content_type("text/html").body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    // Webサーバーの設定
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    return Ok(());
}
