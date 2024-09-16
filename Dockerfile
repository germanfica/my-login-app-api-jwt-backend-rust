# Etapa de construcción
FROM rust:1.80.1 as builder

WORKDIR /usr/src/my_login_app_api
COPY . .

RUN cargo build --release

# Etapa final
FROM debian:bookworm-slim

# dev only for PostgreSQL
# RUN apt-get update && apt-get install -y libpq-dev libpq5 libc6

# dev only for MySQL
# RUN apt-get update && apt-get install -y libmariadb-dev-compat

# prod only for MySQL
# RUN apt-get update && apt-get install -y libmariadb3

# Instalar solo las dependencias mínimas necesarias
RUN apt-get update && apt-get install -y --no-install-recommends libmariadb3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/my_login_app_api/target/release/my_login_app_api /usr/local/bin/my_login_app_api

EXPOSE 8080

# CMD ["my_login_app_api"]
CMD ["/usr/local/bin/my_login_app_api"]
# for Testing purpose only
# CMD ["tail", "-f", "/dev/null"]
# ldd --version
