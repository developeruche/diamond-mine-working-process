use std::env;
use std::fmt::format;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

// project_name
// |- src
// |  |- main.rs 
// |  |- lib.rs
// |- Cargo.toml

/// This simple project is just going to create a new directory and a new file
fn main() -> io::Result<()> {

    // Obtain the project name from the command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter a project name!");
        return Ok(());
    }

    let project_name = &args[2];
    println!("Creating project: {}", project_name);

    // now creating the project directories
    fs::create_dir(project_name)?;
    fs::create_dir(format!("{}/src", project_name))?;
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


    // now creating the files
    fs::write(format!("{}/src/main.rs", project_name), main_rs)?;
    fs::write(format!("{}/src/lib.rs", project_name), lib_rs)?;
    fs::write(format!("{}/Cargo.toml", project_name), cargo_toml)?;
    println!("Project files created successfully!");


    // done with the project creation
    println!("Project created successfully:: {}!", project_name);
    Ok(())
}
