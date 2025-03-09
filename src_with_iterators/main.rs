use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    //you could also use unwrap_or_else alternatively
    /* 
    let Result = minigrep::run(config).unwrap_or_else(|err|{
        eprint!("Application error {err}");
        process::exit(1);
    });
    */

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
        
    
}

