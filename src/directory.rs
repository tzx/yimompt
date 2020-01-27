use std::io::{self, Write};

pub fn display() {
    let current_path = std::env::current_dir().unwrap();
    let shortened_dir = shorten_path(current_path.to_str().unwrap());

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    write!(stdout, "{}", shortened_dir).unwrap();
}

fn shorten_path(path: &str) -> String {
    match dirs::home_dir() {
	Some(home_path) => path.replace(home_path.to_str().unwrap(), "~"),
	None => String::from(path),
    }
}
