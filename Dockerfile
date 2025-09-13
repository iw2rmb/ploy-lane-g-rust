FROM rust:1.79-slim AS build
WORKDIR /src
COPY Cargo.toml .
RUN mkdir -p src && echo "fn main(){}" > src/main.rs && cargo build --release || true
COPY . .
RUN cargo build --release

FROM debian:stable-slim
ENV PORT=8080
EXPOSE 8080
COPY --from=build /src/target/release/hello /app
ENTRYPOINT ["/app"]
