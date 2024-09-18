# Etapa de construcción
FROM debian:bookworm-slim as builder

# This script is specifically designed for MySQL Community Edition.
# If you need to download other MySQL products, such as MySQL Cluster or other MySQL product,
# you will need to rewrite the links.

# Definir variables de construcción
ARG OS_NAME=debian
ARG OS_VERSION=12
ARG MYSQL_VERSION=8.4.2
ARG POOL_NAME=mysql-8.4-lts
# It may slightly vary, but it is usually between 1 and 2
ARG FILE_VERSION=1

# Establecer variables de entorno con los argumentos pasados
ENV OS_NAME=${OS_NAME}
ENV OS_VERSION=${OS_VERSION}
ENV MYSQL_VERSION=${MYSQL_VERSION}
ENV POOL_NAME=${POOL_NAME}
ENV FILE_VERSION=${FILE_VERSION}

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

# MySQL Community 8.4.2: Descargar e instalar dependencias de MySQL
# Oficial MySQL repo: https://repo.mysql.com/apt/
RUN wget http://repo.mysql.com/apt/${OS_NAME}/pool/${POOL_NAME}/m/mysql-community/libmysqlclient-dev_${MYSQL_VERSION}-${FILE_VERSION}${OS_NAME}${OS_VERSION}_amd64.deb && \
    wget http://repo.mysql.com/apt/${OS_NAME}/pool/${POOL_NAME}/m/mysql-community/libmysqlclient24_${MYSQL_VERSION}-${FILE_VERSION}${OS_NAME}${OS_VERSION}_amd64.deb && \
    wget http://repo.mysql.com/apt/${OS_NAME}/pool/${POOL_NAME}/m/mysql-community/mysql-common_${MYSQL_VERSION}-${FILE_VERSION}${OS_NAME}${OS_VERSION}_amd64.deb && \
    wget http://repo.mysql.com/apt/${OS_NAME}/pool/${POOL_NAME}/m/mysql-community/mysql-community-client-plugins_${MYSQL_VERSION}-${FILE_VERSION}${OS_NAME}${OS_VERSION}_amd64.deb && \
    dpkg -i mysql-common_${MYSQL_VERSION}-${FILE_VERSION}${OS_NAME}${OS_VERSION}_amd64.deb && \
    dpkg -i mysql-community-client-plugins_${MYSQL_VERSION}-${FILE_VERSION}${OS_NAME}${OS_VERSION}_amd64.deb && \
    dpkg -i libmysqlclient24_${MYSQL_VERSION}-${FILE_VERSION}${OS_NAME}${OS_VERSION}_amd64.deb && \
    dpkg -i libmysqlclient-dev_${MYSQL_VERSION}-${FILE_VERSION}${OS_NAME}${OS_VERSION}_amd64.deb && \
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

# Copiar el binario desde la etapa de compilación
COPY --from=builder /usr/src/my_login_app_api/target/release/my_login_app_api /usr/local/bin/my_login_app_api

# Exponer el puerto de la aplicación
EXPOSE 8080

# CMD ["my_login_app_api"]
CMD ["/usr/local/bin/my_login_app_api"]
# for Testing purpose only
# CMD ["tail", "-f", "/dev/null"]
# ldd --version
