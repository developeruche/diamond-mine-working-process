use std::{fs};

#[no_mangle]
pub extern "C" fn create_a_project(project_name: &String) -> i32 {
    // creating all the need directories 
    println!("Creating all the need directories: {}", project_name);
    fs::create_dir(project_name).unwrap_or_else(|err| {
        println!("Error creating project directory: {}", err);
    });
    fs::create_dir(format!("{}/src", &project_name)).unwrap_or_else(|err| {
        println!("Error creating src directory: {}", err);
    });
    println!("Project directory created successfully!");


    // here goes the projects content 
    let main_rs = format!("fn main() {{\n\tprintln!(\"Hello, world!\");\n}}");
    let lib_rs = format!("pub fn hello() {{\n\tprintln!(\"Hello, world!\");\n}}");
    let cargo_toml = format!(r#"
        [package]
        name = "{}"
        version = "0.1.0"
        edition = "2021"

        [dependencies]
    "#, project_name);
    println!("Creating project files content!");


    fs::write(format!("{}/src/main.rs", project_name), main_rs).unwrap_or_else(|err| {
        println!("Error writing to main.rs: {}", err);
    });
    fs::write(format!("{}/src/lib.rs", project_name), lib_rs).unwrap_or_else(|err| {
        println!("Error writing to lib.rs: {}", err);
    });
    fs::write(format!("{}/Cargo.toml", project_name), cargo_toml).unwrap_or_else(|err| {
        println!("Error writing to Cargo.toml: {}", err);
    });
    println!("Project files created successfully!");


    println!("All directories has been created!");

    1

    // creating and writing to all files
}

