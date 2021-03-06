/**
 * メインファイル
 */

 // 必要なモジュール読み込み
use actix_web::{get, http::header, post ,web ,App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Deserialize;

// Addtodo用のエントリ
#[derive(Deserialize)]
struct AddParams {
    text: String,
}

// Deletetodo用のエントリ
#[derive(Deserialize)]
struct DeleteParams {
    id: u32,
}

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

    #[error("Failed to get connetion")]
    ConnectionPoolError(#[from] r2d2::Error),

    #[error("Failed SQL execution")]
    SQLiteErro(#[from] rusqlite::Error),
}

// Myerrorを継承したインターフェース
impl ResponseError for MyError {}

// 追加の設定
#[post("/add")]
async fn add_todo(params: web::Form<AddParams>, db: web::Data<r2d2::Pool<SqliteConnectionManager>>,) -> Result<HttpResponse, MyError> {
    // コネクション取得
    let conn = db.get()?;
    // SQL文を実行する。
    conn.execute("INSERT INTO todo (text) VALUES (?)", &[&params.text])?;
    // ルート画面へ遷移する。
    Ok(HttpResponse::SeeOther().header(header::LOCATION, "/").finish())
}

// 削除の場合
#[post("/delete")]
async fn delete_todo(params: web::Form<DeleteParams>, db: web::Data<r2d2::Pool<SqliteConnectionManager>>,) -> Result<HttpResponse, MyError> {
    // コネクション取得
    let conn = db.get()?;
    // SQL文を実行する。
    conn.execute("DELETE FROM todo WHERE id=?", &[&params.id])?;
    // ルート画面へ遷移する。
    Ok(HttpResponse::SeeOther().header(header::LOCATION, "/").finish())
}

// ルート画面の場合
#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    // コネクション取得
    let conn = db.get()?;
    // SQL文を格納する。
    let mut statement = conn.prepare("SELECT id, text FROM todo")?;
    // SQLから取得した結果を格納する。
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry { id, text })
    })?; 
    // 埋め込み用の変数を生成する。
    let mut entries = Vec::new();
    // SQLの取得結果を格納する。
    for row in rows {
        entries.push(row?);
    }
    // htmlを読み込む
    let html = IndexTemplate{entries};
    // body部を作成
    let response_body = html.render()?;
    // レスポンスの内容
    Ok(HttpResponse::Ok().content_type("text/html").body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    // todo.dbファイルを開く
    let manager = SqliteConnectionManager::file("todo.db");
    // コネクションプールを生成する。
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");
    // コネクション生成
    let conn = pool.get().expect("Failed to get the connection from the pool");
    // SQL文生成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL
        )",
        params![],
    ).expect("Failed to create a table `todo`.");
    // Webサーバーの設定(コネクションプールを渡す。)
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(add_todo)
            .service(delete_todo)
            .data(pool.clone())
        })
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    return Ok(());
}
