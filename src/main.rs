use project::run; 
use std::io;

fn main() {
    if let Err(e) = run(&mut io::stdout()) {
        eprintln!("Error running the application: {}", e);
    }
}