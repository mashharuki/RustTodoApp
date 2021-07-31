# Rustイメージを指定
FROM rust:1.43 as builder
# 実行ディレクトリを指定。
WORKDIR /todo
# ビルド時に必要なファイルをイメージにコピーする。
COPY Cargo.toml Cargo.toml
# ビルドのために何もしないソースコードを入れておく。
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
# 上記ソースと依存クレートのビルド実行
RUN cargo build --release
# アプリケーションのコードをイメージにコピーする。
COPY ./src ./src
COPY ./templates ./templates
# 先ほどビルドした生成物のうち、アプリケーションのもののみを削除する。
RUN rm -f target/release/deps/todo*
# 改めてビルド
RUN cargo build --release
# 新しくリリース前のイメージを用意します。
FROM debian:10.4
# builderイメージからtodoのみをコピーして/usr/local/binに配置します。
COPY --from=builder /todo/target/release/todo /usr/local/bin/todo
# Webアプリ実行
CMD ["todo"]