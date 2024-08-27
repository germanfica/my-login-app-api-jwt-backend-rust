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

## MySQL version

### MySQL Tested versions:
- 8.0.0 (works)

- 8.0.11 (works)

    ```
    libmysql.dll
    libmysql.lib
    ```

- 8.0.16 (works)

- 8.0.17 (works)

- 8.0.18 (not working)

- 8.0.19 (not working)

- 8.0.20 (not working)

- 8.0.30 (not working)

- 8.0.37 (not working)

- 8.4.2 LTS (not working) - Windows (x86, 64-bit), MSI Installer	- (mysql-8.4.2-winx64.msi)	MD5: 888dc0f177ce11ed461294ff797824c7 

### Error

```bash
error: process didn't exit successfully: `target\debug\my_login_app_api.exe` (exit code: 0xc0000135, STATUS_DLL_NOT_FOUND)
```

### Tested in
- diesel = { version = "2.2.3", features = ["mysql"] }

- cargo 1.80.1 (376290515 2024-07-16)

- diesel-cli. Version: 2.2.3. Supported Backends: postgres mysql sqlite
