use std::{fs, error::Error};

#[no_mangle]
pub extern fn create_a_project(project_name: String) -> Result<(), Box<dyn Error>> {
    // creating all the need directories 
    println!("Creating all the need directories: {}", &project_name);
    fs::create_dir(&project_name)?;
    fs::create_dir(format!("{}/src", &project_name))?;
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


    fs::write(format!("{}/src/main.rs", project_name), main_rs)?;
    fs::write(format!("{}/src/lib.rs", project_name), lib_rs)?;
    fs::write(format!("{}/Cargo.toml", project_name), cargo_toml)?;
    println!("Project files created successfully!");


    println!("All directories has been created!");

    Ok(())


    // creating and writing to all files
}

