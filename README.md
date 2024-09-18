# my_login_api

## How to run

### Manual

- Install postgresql and sqlite backend libraries, more details here
    - __For Windows:__ Add `PQ_LIB_DIR` and `SQLITE3_LIB_DIR` environment variable with value the binaries libs. Then restart all terminal windows. And then, run `cargo clean`.
    - __For Linux:__ Install `libpq` and `libsqlite3` depends on your distribution. And then, run `cargo clean`.

## Postgress Windows Instructions

This step is critical in order to be able to run `cargo run` in Windows. You need the requested libraries in your system enviroment variables.

- __Step 1:__ Download PostgreSQL Binaries v16.4 from https://www.enterprisedb.com/download-postgresql-binaries
- __Step 2:__ Extract the zip file to `C:\pgsql`
- __Step 3:__ Set the environment variable by running in CMD:

    ```bash
    setx PQ_LIB_DIR "C:\pgsql\lib"
    ```

- __Step 4:__ Run `cargo clean`
- __Step 5:__ Run `cargo run` to start the application.

And that's it enjoy!

## MySQL 8.4 Installation Guide for Ubuntu 22.04

### Step 1: Download `.deb` Files

```bash
wget http://repo.mysql.com/apt/ubuntu/pool/mysql-8.4-lts/m/mysql-community/mysql-common_8.4.2-1ubuntu22.04_amd64.deb
wget http://repo.mysql.com/apt/ubuntu/pool/mysql-8.4-lts/m/mysql-community/mysql-community-client-plugins_8.4.2-1ubuntu22.04_amd64.deb
wget http://repo.mysql.com/apt/ubuntu/pool/mysql-8.4-lts/m/mysql-community/libmysqlclient24_8.4.2-1ubuntu22.04_amd64.deb
wget http://repo.mysql.com/apt/ubuntu/pool/mysql-8.4-lts/m/mysql-community/libmysqlclient-dev_8.4.2-1ubuntu22.04_amd64.deb
```

### Step 2: Install in Order

```bash
sudo dpkg -i mysql-common_8.4.2-1ubuntu22.04_amd64.deb
sudo dpkg -i mysql-community-client-plugins_8.4.2-1ubuntu22.04_amd64.deb
sudo dpkg -i libmysqlclient24_8.4.2-1ubuntu22.04_amd64.deb
sudo dpkg -i libmysqlclient-dev_8.4.2-1ubuntu22.04_amd64.deb
```

### Or simply install in 1 go

```bash
wget http://repo.mysql.com/apt/ubuntu/pool/mysql-8.4-lts/m/mysql-community/mysql-common_8.4.2-1ubuntu22.04_amd64.deb && \
wget http://repo.mysql.com/apt/ubuntu/pool/mysql-8.4-lts/m/mysql-community/mysql-community-client-plugins_8.4.2-1ubuntu22.04_amd64.deb && \
wget http://repo.mysql.com/apt/ubuntu/pool/mysql-8.4-lts/m/mysql-community/libmysqlclient24_8.4.2-1ubuntu22.04_amd64.deb && \
wget http://repo.mysql.com/apt/ubuntu/pool/mysql-8.4-lts/m/mysql-community/libmysqlclient-dev_8.4.2-1ubuntu22.04_amd64.deb && \
sudo dpkg -i mysql-common_8.4.2-1ubuntu22.04_amd64.deb && \
sudo dpkg -i mysql-community-client-plugins_8.4.2-1ubuntu22.04_amd64.deb && \
sudo dpkg -i libmysqlclient24_8.4.2-1ubuntu22.04_amd64.deb && \
sudo dpkg -i libmysqlclient-dev_8.4.2-1ubuntu22.04_amd64.deb && \
rm mysql-common_8.4.2-1ubuntu22.04_amd64.deb mysql-community-client-plugins_8.4.2-1ubuntu22.04_amd64.deb libmysqlclient24_8.4.2-1ubuntu22.04_amd64.deb libmysqlclient-dev_8.4.2-1ubuntu22.04_amd64.deb && /
dpkg --list | grep mysql
```

## Install diesel cli

```bash
cargo install diesel_cli

cargo install diesel_cli --no-default-features --features postgres
```

```bash
# Linux/MacOS
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh

# Windows
powershell -c "irm https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.ps1 | iex"
```

## `mysqlclient-sys` Runtime Error (exit code: 0xc0000135, STATUS_DLL_NOT_FOUND) on Windows

If you encounter the error:

![WindowsTerminal_tJwkxZMmCu](https://github.com/user-attachments/assets/2df34b04-14d5-4971-9552-5a2bb5c799e0)

```bash
error: process didn't exit successfully: `target\debug\mysqlclient_example.exe` (exit code: 0xc0000135, STATUS_DLL_NOT_FOUND)
```

This indicates a missing DLL issue. Use [Dependency Walker](https://www.dependencywalker.com/) to identify the missing DLLs. In this case, the problem was missing `LIBCRYPTO-3-X64.DLL` and `LIBSSL-3-X64.DLL` from the MySQL bin directory.

### Solution

Add the MySQL bin directory to your PATH environment variable. For example, with MySQL version 8.0.36:

```bash
PATH=C:\mysql-8.0.36-winx64\bin
MYSQLCLIENT_LIB_DIR=C:\mysql-8.0.36-winx64\lib
MYSQLCLIENT_VERSION=8.0.36
```

Ensure these DLLs are present in the bin directory, or copy them into your project's target folder.

### Explanation of Dependency Walker

Dependency Walker is a tool that inspects and identifies all the DLLs that an executable depends on. It helps by:

1. **Identifying Missing DLLs:** If your application fails to run due to missing dependencies, Dependency Walker shows you exactly which DLLs are missing.
2. **Tracing Load Issues:** You can see how the system tries to load each DLL, which helps pinpoint issues related to PATH configuration or version conflicts.
3. **Fixing Environment Setup:** Once you've identified missing DLLs (such as `LIBCRYPTO-3-X64.DLL` and `LIBSSL-3-X64.DLL`), you can update the PATH environment variable to include the correct directories or manually add the DLLs to the necessary locations.

This tool was essential in resolving the missing dependency issue I faced in this project, allowing to ensure the proper setup of the environment variables and dependencies.

### Tested in
- diesel = { version = "2.2.3", features = ["mysql"] }

- cargo 1.80.1 (376290515 2024-07-16)

- diesel-cli. Version: 2.2.3. Supported Backends: postgres mysql sqlite
