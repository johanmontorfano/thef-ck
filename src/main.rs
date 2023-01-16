#[path = "./utils/read_file.rs"]
mod rf;
#[path = "./utils/bf_engine.rs"]
mod bf;
use std::env;

fn main() {
    // Gather command args to determine where is the BrainFuck file to parse
    let args: Vec<String> = env::args().collect();
    
    // The second argument has to be a BrainFuck file, (so it should be ending in .bf)
    if args.len() < 2 {
        panic!("Cannot run, no BrainFuck file given. Make sure the file you provided ends with .bf");
    } else {
        // This contains the result of reading the file at the given path, at this point
        // of the program, this variable is used to store results before error handling.
        let bf_file = rf::read_file(&args[1]);

        if bf_file.is_err() {
            panic!("Cannot run, BrainFuck file reading failed.");
        } else {
            bf::handle_bf(bf_file.unwrap());
        }
    }
}
