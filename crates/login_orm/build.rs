use std::{env, fs, path::Path, process::Command};

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let lib_dir = match target_os.as_str() {
        "windows" => "C:\\pgsql\\lib",
        "linux" => "libs/linux",
        _ => panic!("Unsupported OS"),
    };

    println!("El sistema operativo actual es: {}", target_os);
    println!(
        "Ruta recomendada de la libreria para el sistema operativo actual: {}",
        lib_dir
    );

    // Ruta relativa al directorio del proyecto
    if target_os == "windows" {
        let destination = Path::new(lib_dir);
        let pq_lib_dir = destination.to_str().unwrap();

        println!("PQ_LIB_DIR: {}", pq_lib_dir);

        // Imprimir el contenido del directorio lib_dir
        println!("Contents of {}:", lib_dir);
        if let Ok(entries) = fs::read_dir(lib_dir) {
            for entry in entries {
                let entry = entry.expect("Failed to read directory entry");
                let path = entry.path();
                if path.is_file() {
                    println!("File: {:?}", path);
                } else if path.is_dir() {
                    println!("Directory: {:?}", path);
                }
            }
        } else {
            println!("Failed to read directory: {}", lib_dir);
        }
    }

    // Windows instructions
    if target_os == "windows" {
        println!("");
        println!("Windows Setup Instructions:");
        println!("Step 1: Download PostgreSQL Binaries v16.4 from https://www.enterprisedb.com/download-postgresql-binaries");
        println!("Step 2: Extract the zip file to C:\\pgsql");
        println!("Step 3: Set the environment variable by running in CMD:");
        println!("        setx PQ_LIB_DIR \"C:\\pgsql\\lib\"");
        println!("Step 4: Run 'cargo clean'");
        println!("Step 5: Run 'cargo run' to start the application.");
        println!("Enjoy!");

        println!("");
        println!("Step 3: Set the environment variable by running in CMD:");
        println!("        setx MYSQLCLIENT_LIB_DIR \"C:\\path\\to\\mysql\\lib\"");
        println!("        setx MYSQLCLIENT_VERSION \"8.4.2\"");

        // Automatically set PQ_LIB_DIR for the current session if not already set
        // if env::var("PQ_LIB_DIR").is_err() {
        //     env::set_var("PQ_LIB_DIR", "C:\\pgsql\\lib");
        //     Command::new("cmd")
        //         .args(&["/C", "setx", "PQ_LIB_DIR", "C:\\pgsql\\lib"])
        //         .output()
        //         .expect("Failed to set the PQ_LIB_DIR environment variable.");
        //     println!("PQ_LIB_DIR has been set to C:\\pgsql\\lib");
        // }

        // Optionally run `cargo clean` programmatically
        // Command::new("cargo")
        //     .arg("clean")
        //     .output()
        //     .expect("Failed to run 'cargo clean'.");
        // Verificar si las variables de entorno necesarias están presentes
        env::var("PQ_LIB_DIR")
            .expect("Error: La variable de entorno PQ_LIB_DIR no está establecida.");
        env::var("SQLITE3_LIB_DIR")
            .expect("Error: La variable de entorno SQLITE3_LIB_DIR no está establecida.");
        env::var("MYSQLCLIENT_LIB_DIR")
            .expect("Error: La variable de entorno MYSQLCLIENT_LIB_DIR no está establecida.");
        env::var("MYSQLCLIENT_VERSION")
            .expect("Error: La variable de entorno MYSQLCLIENT_VERSION no está establecida.");

        // Puedes añadir otras verificaciones o configuraciones necesarias aquí
    }
}
