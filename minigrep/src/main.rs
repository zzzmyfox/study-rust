use std::env;

use minigrep::Config;

fn main() {
    let config =  match Config::new(env::args()) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("problem parsing arguments: {}", err);
            return;
        }
    };    
   if let Err(err) = minigrep::run(config) {
        eprintln!("application error: {}", err);
        return;
    }
}