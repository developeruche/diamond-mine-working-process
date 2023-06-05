use std::env;
use std::fs;
use std::process;
use std::error::Error;



struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
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

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    run(config).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    });
}


fn run(config: Config) -> Result<(), Box<dyn Error>> { // the error type would be determined at run-time
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}