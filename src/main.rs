use std::{env, io::{stdin, stdout, Write}};

/// Interprets a brainfuck program using defined standards for a brainfuck 
/// program:
///
/// - 30,000 bytes of 8-bit memory.
/// - single pointer to memory.
pub fn interpret(code: String) {
    let instructions = code.chars().collect::<Vec<char>>();
    let mut instructions_ptr_stack = vec![0_usize];

    let mut memory = [0_u8; 30000];
    let mut memory_ptr = 0_usize;
    let mut memory_ptr_stack = vec![0_usize];

    // The memory pointer stack only contains the position of the pointer when
    // a loop began. It differs from the instructions pointer stack that states
    // of the position of each loop in the iteration process.

    while instructions_ptr_stack.get(0).unwrap() < &instructions.len() {
        let instruction = instructions
            .get(*instructions_ptr_stack.last().unwrap())
            .unwrap();

        if let Some(last_ptr) = instructions_ptr_stack.last_mut() {
            *last_ptr += 1;
        }

        match instruction {
            '>' => {
                assert_ne!(memory_ptr, 30000, "Pointing out of memory");
                memory_ptr += 1;
            }
            '<' => {
                assert_ne!(memory_ptr, 0, "Pointing out of memory");
                memory_ptr -= 1;
            }
            '+' => { 
                if memory[memory_ptr] == u8::MAX {
                    memory[memory_ptr] = u8::MIN;
                } else {
                    memory[memory_ptr] += 1;
                }
            }
            '-' => {
                if memory[memory_ptr] == u8::MIN {
                    memory[memory_ptr] = u8::MAX;
                } else {
                    memory[memory_ptr] -= 1;
                }
            }
            '.' => {
                let charmem = char::from_u32(memory[memory_ptr]
                        .try_into()
                        .unwrap())
                    .expect("Failed to convert address to a char");
                stdout().write(String::from(charmem).as_bytes()).unwrap();
                stdout().flush().unwrap();
            }
            ',' => {
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                
                let input: Vec<char> = input.trim().chars().collect();
                memory[memory_ptr] = input[0] as u8;
            }
            '[' => {
                instructions_ptr_stack.push(
                    *instructions_ptr_stack.get(instructions_ptr_stack.len() - 1)
                    .unwrap());
                memory_ptr_stack.push(memory_ptr);
            }
            ']' => {
                if memory[*memory_ptr_stack.last().unwrap()] == 0 {
                    let ptr_pos = instructions_ptr_stack.len() - 2;

                    *instructions_ptr_stack.get_mut(ptr_pos).unwrap() = 
                        instructions_ptr_stack.pop().unwrap(); 
                    memory_ptr_stack.pop();
                } else {
                    *instructions_ptr_stack.last_mut().unwrap() = 
                        *instructions_ptr_stack
                            .get(instructions_ptr_stack.len() - 2)
                            .unwrap();
                }
            }
            _ => ()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(code) = args.get(1) {
        interpret(code.into());
    }
}
