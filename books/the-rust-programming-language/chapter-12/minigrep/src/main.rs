use std::env;
use std::fs;

fn main() {
    // How can we prevent the user from passing a very large number of
    // parameters?
    let args: Vec<String> = env::args().collect();

    // How do we validate if there are no args, so we can print a message?
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{}", contents);
}