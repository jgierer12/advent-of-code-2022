use std::env;
use std::fs;

pub fn read_input_file() -> String {
    const DEMO_FILE_PATH: &str = "inputs/demo.txt";
    const CHALLENGE_FILE_PATH: &str = "inputs/challenge.txt";

    let args: Vec<String> = env::args().collect();

    let file_path = {
        if args.len() > 1 && &args[1] == "demo" {
            DEMO_FILE_PATH
        } else {
            CHALLENGE_FILE_PATH
        }
    };

    fs::read_to_string(file_path).expect(&format!("Error while trying to read file '{file_path}'"))
}
