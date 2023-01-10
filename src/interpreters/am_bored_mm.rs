extern crate sdl2;
extern crate bit;

use bit::BitIndex;
use std::{
    process::exit,
    collections::HashMap,
    thread::sleep,
    time::Duration
};

use ansi_term::Colour;

use sdl2::{
    render::Canvas,
    video::Window,
    pixels::Color,
    rect::Point,
    event::Event
};

use crate::misc::get_byte_from_input;


pub fn run(code: String) {
    let instruction_vector: Vec<String> = code.to_lowercase().chars().map(|c| c.to_string()).filter(|s| "><v^*+[],.".to_owned().contains(s)).collect();
    let mut instruction_pointer: usize = 0;

    let mut array: [[bool; 64]; 64] = [[false; 64]; 64];
    let mut array_pointer: [u8; 2] = [0, 0];
    
    let mut stack: Vec<bool> = vec![];

    let loop_map: HashMap<usize, usize> = generate_loop_map(instruction_vector.clone());

    let pixel_scale: u32 = 10;
    let delay_milliseconds: u64 = 500;

    let mut instruction: String;

    let sdl_context = match sdl2::init() {
        Err(_) => {
            eprintln!("Error Initializing SDL2");
            exit(4);
        }
        Ok(c) => {
            c
        }
    };

    let video_subsystem = match sdl_context.video() {
        Err(_) => {
            eprintln!("Error Initializing SDL2");
            exit(5);
        }
        Ok(v )=> {
            v
        }
    };

    let window = match video_subsystem.window("AmBored--", 64*pixel_scale, 64*pixel_scale).position_centered().build() {
        Err(_) => {
            eprintln!("Error Initializing SDL2");
            exit(6);
        }
        Ok(w) => {
            w
        }
    };

    let mut canvas: Canvas<Window> = match window.into_canvas().build() {
        Err(_) => {
            eprintln!("Error Initializing SDL2");
            exit(7);
        }
        Ok(c) => {
            c
        }
    };

    let mut event_pump = match sdl_context.event_pump() {
        Err(_) => {
            eprintln!("Error Initializing SDL2");
            exit(8);
        }
        Ok(e) => {
            e
        }
    };

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    loop {
        if instruction_pointer == instruction_vector.len() {
            break;
        }

        instruction = instruction_vector[instruction_pointer].clone();

        match instruction.as_str() {
            ">" => { // Move the memory pointer right
                array_pointer[1] += 1;
            }

            "<" => { // Move the memory pointer left
                array_pointer[1] -= 1;
            }

            "v" => { // Move the memory pointer down
                array_pointer[0] += 1;
            }

            "^" => { // Move the memory pointer up
                array_pointer[0] -= 1;
            }

            "*" => { // Change the pointed cell's color
                array[array_pointer[0] as usize][array_pointer[1] as usize] = !array[array_pointer[0] as usize][array_pointer[1] as usize]
            }

            "+" => { // Set the pointed cell to white
                array[array_pointer[0] as usize][array_pointer[1] as usize] = true;
            }

            "," => { // Input a number and write the LSB to the pointed cell
                array[array_pointer[0] as usize][array_pointer[1] as usize] = get_byte_from_input().bit(0);
            }

            "." => { // Refresh the screen
                canvas.clear();
                let point_sets = generate_points(array, pixel_scale);
                
                canvas.set_draw_color(Color::RGB(0, 0, 0));

                for point in point_sets[1].clone() {
                    match canvas.draw_point(point) {
                        Err(_) => {
                            eprintln!("{}", Colour::Red.paint("SDL2 RUNTIME ERROR"));
                            exit(9);
                        }
                        Ok(_) => {}
                    };
                }
                
                canvas.set_draw_color(Color::RGB(255, 255, 255));

                for point in point_sets[0].clone() {
                    match canvas.draw_point(point) {
                        Err(_) => {
                            eprintln!("{}", Colour::Red.paint("SDL2 RUNTIME ERROR"));
                            exit(9);
                        }
                        Ok(_) => {}
                    };
                }
                canvas.present();
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }

            "[" => { // Jump past the matching ] if the pointed cell is 0 (black)
                instruction_pointer = *loop_map.get(&instruction_pointer).unwrap();
            }

            "]" => { // Jump back to the matching [ if the pointed cell is 1 (white)
                instruction_pointer = *loop_map.get(&instruction_pointer).unwrap();
            }

            "(" => { // Push the value of the pointed cell to the stack
                stack.push(array[array_pointer[0] as usize][array_pointer[1] as usize]);
                
                if stack.len() > 16 {
                    stack.remove(0);
                }
            }

            ")" => { // Pop the top value of the stack to the pointed cell
                array[array_pointer[0] as usize][array_pointer[1] as usize] = match stack.pop() {
                    None => {
                        false
                    }
                    Some(b) => {
                        b
                    }
                }
            }

            _ => {}
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}  => {
                    exit(0);
                },
                _ => {}
            }
        }

        instruction_pointer += 1;

        sleep(Duration::from_millis(delay_milliseconds));
    }

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}  => {
                    exit(0);
                },
                _ => {}
            }
        }
    }
}

fn generate_points(array: [[bool; 64]; 64], size: u32) -> [Vec<Point>; 2] {
    let mut white_point_vec: Vec<Point> = vec![];
    let mut black_point_vec: Vec<Point> = vec![];

    for x in 0..64 {
        for y in 0..64 {
            if array[y][x] == true {
                for px in 0..size as usize {
                    for py in 0..size as usize {
                        white_point_vec.push(Point::new((x*size as usize+px) as i32, (y*size as usize+py) as i32))
                    }
                }
            } else {
                for px in 0..size as usize {
                    for py in 0..size as usize {
                        black_point_vec.push(Point::new((x*size as usize+px) as i32, (y*size as usize+py) as i32))
                    }
                }
            }
        }
    }

    return [white_point_vec, black_point_vec];
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
                exit(10);
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
        exit(11);
    }
    return return_map;
}