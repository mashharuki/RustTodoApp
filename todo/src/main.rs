/**
 * メインファイル
 */

 // 必要なモジュール読み込み
use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;

// エラー用の列挙型変数
#[derive(Error, Debug)]
enum MyError {}

// Myerrorを継承したインターフェース
impl ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let response_body = "Hello World!";
    Ok(HttpResponse::Ok().body(response_body))
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
