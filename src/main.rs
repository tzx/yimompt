mod modules;

fn main() {
    let x = modules::directory::get_dir();
    println!("{}", x);
    println!("$ ");
}
