# my_login_api

## How to run

### Manual

- Install postgresql and sqlite backend libraries, more details here
    - __For Windows:__ Add `PQ_LIB_DIR` and `SQLITE3_LIB_DIR` environment variable with value the binaries libs. Then restart all terminal windows. And then, run `cargo clean`.
    - __For Linux:__ Install `libpq` and `libsqlite3` depends on your distribution. And then, run `cargo clean`.

## Postgress Windows Instructions

This step is critical in order to be able to run `cargo run` in Windows. You need the requested libraries in your system enviroment variables.

- __Step 1:__ Download PostgreSQL Binaries v16.4 from https://www.enterprisedb.com/download-postgresql-binaries"
- __Step 2:__ Extract the zip file to `C:\\pgsql`
- __Step 3:__ Set the environment variable by running:"

    ```bash
    setx PQ_LIB_DIR \"C:\\pgsql\\lib\"
    ```

- __Step 4:__ Run `cargo clean`
- __Step 5:__ Run `cargo run` to start the application.

And that's it enjoy!
