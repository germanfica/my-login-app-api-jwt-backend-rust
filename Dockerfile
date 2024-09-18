# Etapa de construcción
FROM ubuntu:22.04 as builder

# Actualizar los repositorios y paquetes mínimos necesarios
RUN apt-get update && apt-get install -y \
    wget \
    build-essential \
    curl \
    libgssapi-krb5-2 \
    libkrb5-3 \
    libsasl2-2 \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Instalar Rust utilizando rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# MySQL 8.4.2: Descargar e instalar dependencias de MySQL
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

# Preparar el entorno de trabajo
WORKDIR /usr/src/my_login_app_api

# Copiar el código fuente al contenedor
COPY . .

# Compilar la aplicación Rust en modo release
RUN cargo build --release

# Etapa final
FROM debian:bookworm-slim

# Copiar las bibliotecas necesarias desde la etapa de compilación
COPY --from=builder /usr/lib/x86_64-linux-gnu/libmysqlclient.so.24 /usr/lib/x86_64-linux-gnu/libmysqlclient.so.24
COPY --from=builder /usr/lib/x86_64-linux-gnu/libssl.so.3 /usr/lib/x86_64-linux-gnu/libssl.so.3
COPY --from=builder /usr/lib/x86_64-linux-gnu/libcrypto.so.3 /usr/lib/x86_64-linux-gnu/libcrypto.so.3

# Copiar el binario desde la etapa de compilación
COPY --from=builder /usr/src/my_login_app_api/target/release/my_login_app_api /usr/local/bin/my_login_app_api

# Exponer el puerto de la aplicación
EXPOSE 8080

# CMD ["my_login_app_api"]
CMD ["/usr/local/bin/my_login_app_api"]
# for Testing purpose only
# CMD ["tail", "-f", "/dev/null"]
# ldd --version
