# Etapa de construcción
FROM rust:1.80.1-slim as builder

# Actualizar los repositorios y paquetes
RUN apt-get update && apt-get install -y \
    wget \
    libgssapi-krb5-2 \
    libkrb5-3 \
    libsasl2-2 \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Paso 1: Desinstalar libmariadb-dev-compat para evitar conflictos (only for rust:1.80.1)
# RUN apt-get update && apt-get remove --purge -y libmariadb-dev-compat libmariadb-dev libmariadb3 mariadb-common

# MySQL 8.4.2
RUN wget https://dev.mysql.com/get/Downloads/MySQL-8.4/libmysqlclient-dev_8.4.2-1ubuntu22.04_amd64.deb && \
    wget http://repo.mysql.com/apt/ubuntu/pool/mysql-8.4-lts/m/mysql-community/libmysqlclient24_8.4.2-1ubuntu22.04_amd64.deb && \
    wget https://dev.mysql.com/get/Downloads/MySQL-8.4/mysql-common_8.4.2-1ubuntu22.04_amd64.deb && \
    wget https://dev.mysql.com/get/Downloads/MySQL-8.4/mysql-community-client-plugins_8.4.2-1ubuntu22.04_amd64.deb && \
    dpkg -i mysql-common_8.4.2-1ubuntu22.04_amd64.deb && \
    dpkg -i mysql-community-client-plugins_8.4.2-1ubuntu22.04_amd64.deb && \
    dpkg -i libmysqlclient24_8.4.2-1ubuntu22.04_amd64.deb && \
    dpkg -i libmysqlclient-dev_8.4.2-1ubuntu22.04_amd64.deb && \
    rm -rf /var/lib/apt/lists/* && \
    rm -f *.deb

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
# RUN apt-get update && apt-get install -y --no-install-recommends libmariadb3 \
#     && rm -rf /var/lib/apt/lists/*

# Copiar las bibliotecas necesarias desde la etapa de compilación
COPY --from=builder /usr/lib/x86_64-linux-gnu/libmysqlclient.so.24 /usr/lib/x86_64-linux-gnu/libmysqlclient.so.24
COPY --from=builder /usr/lib/x86_64-linux-gnu/libssl.so.3 /usr/lib/x86_64-linux-gnu/libssl.so.3
COPY --from=builder /usr/lib/x86_64-linux-gnu/libcrypto.so.3 /usr/lib/x86_64-linux-gnu/libcrypto.so.3

COPY --from=builder /usr/src/my_login_app_api/target/release/my_login_app_api /usr/local/bin/my_login_app_api

EXPOSE 8080

# CMD ["my_login_app_api"]
CMD ["/usr/local/bin/my_login_app_api"]
# for Testing purpose only
# CMD ["tail", "-f", "/dev/null"]
# ldd --version
