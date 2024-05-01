use project::run; // Replace `project` with the actual crate name if different
use std::io;

fn main() {
    if let Err(e) = run(&mut io::stdout()) {
        eprintln!("Error running the application: {}", e);
    }
}