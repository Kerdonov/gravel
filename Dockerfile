FROM rust:1.91.1-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:12-slim
WORKDIR /app
COPY --from=builder /app/target/release/stdsrv .
CMD ["./stdsrv"]
