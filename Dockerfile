FROM rust:buster
WORKDIR /usr/app
COPY . .

RUN rustup override set nightly
RUN rustup target add wasm32-unknown-unknown
RUN apt install binaryen
RUN cargo install --features no_downloads --locked cargo-leptos
RUN cargo leptos build --release --bin-features ssr
EXPOSE 3000
ENTRYPOINT ["./target/release/leptos_chat_app"]