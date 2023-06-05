use std::env;





fn main() {
    let args = env::args().collect::<Vec<String>>();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
