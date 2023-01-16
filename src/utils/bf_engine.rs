use std::{
    io::{stdout, Write}
};
use std::vec::Vec;

// Get a BrainFuck content to run and runs it
pub fn handle_bf(content: String) {
    // Convert the content into a vector
    let content = content.chars().collect::<Vec<char>>();

    // Initializes the 30,000b one dimensional array.
    // ! Please notice that this engine doesn't support negative values on bytes.
    let mut byte_arr = [0 as u32; 30000];

    // Initializes two pseudo-pointers, the first one points to the currently
    // read byte in the BF string. While the other is about pointing to the
    // currently used value in the array.
    let mut content_ptr: usize = 0;
    let mut byte_arr_ptr: usize = 0;

    // Initializes a loop pointer which points to the start of the last current
    // found loop.
    // ! Please notice that this engine doesn't support nested loops yet.
    let mut loop_ptr: Vec<usize> = Vec::new();

    // Read the BF string and apply proper actions to it.
    while content_ptr < content.len() {
        let key = content[content_ptr];

        if key == '>' {
            if byte_arr_ptr == 30000 {
                panic!("Tried to move pointer outside the byte array.")
            }
            byte_arr_ptr += 1;
        } else if key == '<' {
            if byte_arr_ptr == 0 {
                panic!("Tried to move pointer outside the byte array.")
            }
            byte_arr_ptr -= 1;
        } else if key == '+' {
            byte_arr[byte_arr_ptr] += 1;
        } else if key == '-' {
            byte_arr[byte_arr_ptr] -= 1;
        } else if key == '.' {
            let chkey = char::from_u32(byte_arr[byte_arr_ptr]).expect(&format!(
                "An error happened while processing the value at: {}",
                byte_arr_ptr
            ));
            stdout()
                .write(String::from(chkey).as_bytes())
                .expect(&format!("Failed to output the byte at: {}", byte_arr_ptr));
            stdout().flush().expect("Failed to flush stdout.");
        } else if key == ',' {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let line = line.trim().chars().collect::<Vec<char>>();
            byte_arr[byte_arr_ptr] = line[0] as u32;
        } else if key == '[' {
            loop_ptr.push(content_ptr);
        } else if key == ']' {
            if byte_arr[byte_arr_ptr] == 0 {
                loop_ptr.pop();
            } else {
                content_ptr = *loop_ptr.last().unwrap();
            }
        }

        content_ptr += 1;
    }
}
