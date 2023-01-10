use std::{process::exit, collections::HashMap};
use ansi_term::Colour;

use crate::misc::get_byte_from_input;

pub fn run(code: String) {
    // The instruction_vector is just the code with all non command characters removed and turned into a vector for easy access
    // This is done by first turning all characters to their lowercase versions
    // Next: turning the code string into an iterator of chars
    // These chars are then mapped into strings resulting in an iterator of strings
    // This is then filtered to remove any non command characters and collected to yield a vector of strings
    let instruction_vector: Vec<String> = code.to_lowercase().chars().map(|c| c.to_string()).filter(|s| "><^v*o@?+-.,[]".to_owned().contains(s)).collect();
    
    // The stack in 4DChess is a 4 dimensional array of 8 bit unsigned integers each dimension limited to 8 cells for a total of 4096;
    let mut stack: [[[[u8; 8]; 8]; 8]; 8] = [[[[0; 8]; 8]; 8]; 8];
    
    // The stack_pointer is a mutable fixed sized array of 4 unsigned integers;
    let mut stack_pointer: [usize; 4] = [0; 4];

    // The instruction_pointer indexes the instruction_vector to show what instruction should be exectued
    let mut instruction_pointer: usize = 0;
    
    // The instruction is the result of indexing the instruction_vector with the instruction_pointer and tells us the next step to execute
    let mut instruction;

    // The loop_map tells the program where to skip to when encoutering loops ( "[" & "]" )
    let loop_map: HashMap<usize, usize> = generate_loop_map(instruction_vector.clone());

    // As the program ends when the end of the commands is reached this just repeats until the instrution_pointer exceeds the instruction vector
    while (instruction_pointer as usize) < instruction_vector.len() {
        // We grab the instruction by indexing the instruction_vector with the instruction_pointer
        instruction = instruction_vector[instruction_pointer as usize].as_str();

        // We now match the instruction against the possible commands
        match instruction {
            ">" => { // Increase the pointer's position along the X axis
                stack_pointer[0] += 1;
                if stack_pointer[0] >= 8 {
                    eprintln!("{}", Colour::Yellow.paint("You fell of the hypercube"));
                    exit(12);
                }
            }

            "<" => { // Decrease the pointer's position along the X axis
                if stack_pointer[0] != 0 {
                    stack_pointer[0] -= 1;
                } else {
                    // if the program attempts to index a memory address below 0 raise a range error
                    eprintln!("{}", Colour::Red.paint("You fell of the hypercube"));
                    exit(12);
                };
            }

            "^" => { // Increase the pointer's position along the Y axis
                if stack_pointer[1] != 8 {
                    stack_pointer[1] += 1;
                } else {
                    // if the program attempts to index a memory address below 0 raise a range error
                    eprintln!("{}", Colour::Red.paint("You fell of the hypercube"));
                    exit(12);
                };
            }

            "v" => { // Decrease the pointer's position along the Y axis
                if stack_pointer[1] != 0 {
                    stack_pointer[1] -= 1;
                } else {
                    // if the program attempts to index a memory address below 0 raise a range error
                    eprintln!("{}", Colour::Red.paint("You fell of the hypercube"));
                    exit(12);
                };
            }

            "*" => { // Increase the pointer's position along the Z axis
                if stack_pointer[2] != 8 {
                    stack_pointer[2] += 1;
                } else {
                    // if the program attempts to index a memory address below 0 raise a range error
                    eprintln!("{}", Colour::Red.paint("You fell of the hypercube"));
                    exit(12);
                };
            }

            "o" => { // Decrease the pointer's position along the Z axis
                if stack_pointer[2] != 0 {
                    stack_pointer[2] -= 1;
                } else {
                    // if the program attempts to index a memory address below 0 raise a range error
                    eprintln!("{}", Colour::Red.paint("You fell of the hypercube"));
                    exit(12);
                };
            }

            "@" => { // Increase the pointer's position along the W axis
                if stack_pointer[3] != 8 {
                    stack_pointer[3] += 1;
                } else {
                    // if the program attempts to index a memory address below 0 raise a range error
                    eprintln!("{}", Colour::Red.paint("You fell of the hypercube"));
                    exit(12);
                };
            }

            "?" => { // Decrease the pointer's position along the W axis
                if stack_pointer[3] != 0 {
                    stack_pointer[3] -= 1;
                } else {
                    // if the program attempts to index a memory address below 0 raise a range error
                    eprintln!("{}", Colour::Red.paint("You fell of the hypercube"));
                    exit(12);
                };
            }

            "+" => { // Increment the memory cell under the pointer
                if stack[stack_pointer[0]][stack_pointer[1]][stack_pointer[2]][stack_pointer[3]] == u8::MAX {
                    eprintln!("{}", Colour::Yellow.paint("Overflow Error"));
                    exit(13);
                } else {
                    stack[stack_pointer[0]][stack_pointer[1]][stack_pointer[2]][stack_pointer[3]] += 1;
                };
            }

            "-" => { // Decrement the memory cell under the pointer
                if stack[stack_pointer[0]][stack_pointer[1]][stack_pointer[2]][stack_pointer[3]] == 0 {
                    eprintln!("{}", Colour::Yellow.paint("Overflow Error"));
                    exit(14);
                } else {
                    stack[stack_pointer[0]][stack_pointer[1]][stack_pointer[2]][stack_pointer[3]] -= 1;
                };
            }

            "." => { // Output the character signified by the cell at the pointer's position on the X, Y, Z, and W axis
                // As Ascii is a u8 same as stack values. A char is just a u8 wrapper so doing u8 as char will convert a number to a character which can be printed.
                print!("{}", stack[stack_pointer[0]][stack_pointer[1]][stack_pointer[2]][stack_pointer[3]] as char);
            }

            "," => { // Input a character and store it in the cell at the pointer's position on the X, Y, Z, and W axis
                stack[stack_pointer[0]][stack_pointer[1]][stack_pointer[2]][stack_pointer[3]] = get_byte_from_input();
            }

            "[" => { // Jump past the matching ] if the cell under the pointer is 0
                if stack[stack_pointer[0]][stack_pointer[1]][stack_pointer[2]][stack_pointer[3]] == 0 {
                    instruction_pointer = loop_map.get(&instruction_pointer).unwrap().clone();
                };
            }

            "]" => { // Jump back to the matching [ if the cell under the pointer is nonzero
                if stack[stack_pointer[0]][stack_pointer[1]][stack_pointer[2]][stack_pointer[3]] != 0 {
                    instruction_pointer = loop_map.get(&instruction_pointer).unwrap().clone();
                };
            }

            _ => {
                continue;
            }
        };

        instruction_pointer += 1;
    };
}

fn generate_loop_map(instruction_vector: Vec<String>) -> HashMap<usize, usize>{
    // The return_map is a hashmap used to directly get the corresponding loop end to another loop end represented by their respective indicies in the instruction_vector
    let mut return_map: HashMap<usize, usize> = HashMap::new();
    // The open_vector keeps track of all loop starts that have not yet been closed by a corresponding loop end
    let mut open_vector: Vec<usize> = vec![];
    // The index just represents the current index of the instruction vector
    let mut index: usize = 0;
    // A temp value to make handling errors easier ( the last element of the open vector will be assigned to this value during hashmap inserts )
    let mut value1: usize;
    // We loop over each command in the instruction vector and keep track of the index
    for loop_command in instruction_vector {
        if loop_command.as_str() == "[" {
            // If the command is a loop start append index to the open vectors
            open_vector.push(index)
        } else if loop_command.as_str() == "]" {
            // If the command is a loop end make sure there is a corresponding loop start if not raise an error then append both ( loop_start_index: loop_end_index ) and ( loop_end_index: loop_start_index ) then remove the last element of open_vector
            match open_vector.last() {
                None => {
                    eprintln!("{}", Colour::Red.paint("MISMATCHED LOOPS"));
                    exit(13);
                }
                Some(v) => {
                    value1 = v.clone();
                }
            }
            return_map.insert(value1, index);
            return_map.insert(index, value1);
            open_vector.pop();
        }
        // Increment index
        index += 1;
    }
    // If there is an unclosed loop start raise an error
    if open_vector.len() != 0 {
        eprintln!("{}", Colour::Red.paint("MISMATCHED LOOPS"));
        exit(14);
    }
    return return_map;
}