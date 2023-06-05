use std::fs;
use std::error::Error;




pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // Check if the number of args is correct
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Extracting the args from env::args
        let query = args[1].clone();
        let filename = args[2].clone();

        // Return the Config struct
        Ok(Config { query, filename })
    }
}




pub fn run(config: Config) -> Result<(), Box<dyn Error>> { // the error type would be determined at run-time
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}



