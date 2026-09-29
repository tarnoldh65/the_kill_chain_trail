FROM rust:1-slim AS build
WORKDIR /app
RUN rustup target add wasm32-unknown-unknown
COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo
COPY src ./src
RUN cargo build --release --target wasm32-unknown-unknown

FROM nginx:alpine
COPY web /usr/share/nginx/html
COPY --from=build /app/target/wasm32-unknown-unknown/release/the_kill_chain_trail.wasm /usr/share/nginx/html/the_kill_chain_trail.wasm
EXPOSE 80
