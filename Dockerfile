# Etapa de construcción
FROM rust:1.80.1 as builder

WORKDIR /usr/src/my_login_app_api
COPY . .

RUN cargo build --release

# Etapa final
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq-dev libpq5 libc6

COPY --from=builder /usr/src/my_login_app_api/target/release/my_login_app_api /usr/local/bin/my_login_app_api

EXPOSE 8080

# CMD ["my_login_app_api"]
CMD ["/usr/local/bin/my_login_app_api"]
# for Testing purpose only
# CMD ["tail", "-f", "/dev/null"]
# ldd --version
