# Etapa 1: Build
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Etapa 2: Execução
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/api_rest_rust /app
CMD ["./api_rest_rust"]