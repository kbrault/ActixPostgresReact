FROM rust:1.80-slim-bookworm
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .env ./
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config
RUN cargo build --release
EXPOSE 8080
ENTRYPOINT [ "/app/target/release/ActixPostgresReact-Backend" ]