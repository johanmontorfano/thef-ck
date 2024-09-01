use std::{env, io::{stdin, stdout, Write}};

/// Interprets a brainfuck program using defined standards for a brainfuck 
/// program:
///
/// - 30,000 bytes of 8-bit memory.
/// - single pointer to memory.
pub fn interpret(code: String) {
    let instructions = code.chars().collect::<Vec<char>>();
    let mut instruction_ptr = 0;
    let mut memory = [0_u8; 30000];
    let mut memory_ptr = 0_usize;
    let mut loop_stack = vec![];

    // The memory pointer stack only contains the position of the pointer when
    // a loop began. It differs from the instructions pointer stack that states
    // of the position of each loop in the iteration process.

    while instruction_ptr < instructions.len() {
        match instructions[instruction_ptr] {
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
                if memory[memory_ptr] == 0 {
                    let mut open_brackets = 1;

                    while open_brackets > 0 {
                        instruction_ptr += 1;
                        match instructions[instruction_ptr] {
                            '[' => open_brackets += 1,
                            ']' => open_brackets -= 1,
                            _ => ()
                        }
                    }
                } else {
                    loop_stack.push(instruction_ptr);
                }
            }
            ']' => {
                if memory[memory_ptr] != 0 {
                    instruction_ptr = *loop_stack.last().unwrap();
                } else {
                    loop_stack.pop();
                }
            }
            _ => ()
        }

        instruction_ptr += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(code) = args.get(1) {
        interpret(code.into());
    }
}
