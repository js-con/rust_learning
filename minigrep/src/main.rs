use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    // args[0] 保存的是程序名
    let query = &args[1];
    let filename = &args[2];
    
    println!("Searching for {},in file {}", query, filename);

    let contents = fs::read_to_string(filename).expect("Error when reading the file!");
    println!("With test:\n {}", contents);
}
