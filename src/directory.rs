pub fn display() {
    let current_path = std::env::current_dir().unwrap();
    let shortened_dir = shorten_path(current_path.to_str().unwrap());
    println!("{}", shortened_dir);
}

fn shorten_path(path: &str) -> String {
    match dirs::home_dir() {
	Some(home_path) => path.replace(home_path.to_str().unwrap(), "~"),
	None => String::from(path),
    }
}
