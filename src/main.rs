extern crate exit_code;

fn main() {
    use std::process::exit;
    use exit_code::SUCCESS;

    println!("Hello, World!");
    exit(SUCCESS);
}
