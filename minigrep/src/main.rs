use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {
    // NOTE: cargo run can be ran outside of src.
    // file search is searching in PWD of where cargo run is triggered.

    // passed in flags / parameters into cli tool
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // handles error here and exit if err. 
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("args {:?}", args);
    println!("searching for : {}", config.query);
    println!("in file : {}", config.filename);

    // match env::current_dir() {
    //     Ok(path) => println!("Current directory: {}", path.display()),
    //     Err(err) => println!("Error getting current directory: {}", err),
    // }
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };

}