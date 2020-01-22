pub fn get_dir() -> String {
    const HOME_SYMBOL: &str = "~";
    let current_dir = match std::env::current_dir() {
        Ok(x) => x,
        Err(e) => {
            panic!("Cannot get current directory: {}", e);
        },
    };
    let home_dir = dirs::home_dir().unwrap();
    
    current_dir.to_str().unwrap().to_string()
}

