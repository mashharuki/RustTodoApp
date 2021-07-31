# Rustイメージを指定
FROM rust:1.43
# 実行ディレクトリを指定。
WORKDIR /todo
# ビルド時に必要なファイルをイメージにコピーする。
COPY Cargo.toml Cargo.toml
COPY ./src ./src
COPY ./templates ./templates
# ビルド実行
RUN cargo build --release
# パスの通った場所にインストールする
RUN cargo install --path .
# Webアプリ実行
CMD ["todo"]