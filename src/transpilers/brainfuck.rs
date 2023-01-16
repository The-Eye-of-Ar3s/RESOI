use std::{
    process::exit,
    path::PathBuf,
    str::FromStr,
    fs::File, io::Write,
};
use ansi_term::Colour;


pub fn transpile(code: String, outfile_option: Option<PathBuf>) {
    let outfile: PathBuf = match outfile_option { // If no outfile location has been specified we assign the default option which is ./out.zip
        None => {
            PathBuf::from_str("./out.zip").unwrap()
        }
        Some(p) => {
            PathBuf::from_str(format!("{}.zip", p.to_str().unwrap()).as_str()).unwrap()
        }
    };

    let cargo_toml: String = "[package]\nname = \"rustbf\"\nversion = \"0.1.0\"\nedition = \"2021\"".to_owned(); // The contents of the Cargo.toml file
    let mut main_rs: String = "use std::io::Read;\n\nfn gb() -> u8 {\nmatch std::io::stdin().bytes().next().and_then(|result| result.ok()) {\nNone => {\n return 0;\n}\nSome(v) => {\nif v == 4 {\nreturn 0;\n} else {\nreturn v;\n}\n}\n}\n}\n\n\nfn main() {\nlet mut s: Vec<u8> = vec![0];\nlet mut i: usize = 0;\n".to_string(); // main.rs boilerplate code

    let instruction_vector: Vec<String> = code.chars().map(|c| c.to_string()).filter(|s| "><+-,.[]".to_owned().contains(s)).collect(); // a vector containing all instructions with everything else filtered out.

    for instruction in instruction_vector { // For each instructions
        match instruction.as_str() { // Match current instruction
            ">" => { // Pointer Move right
                main_rs = format!("{}{}\n", main_rs, "i+=1;"); // Increment the pointer by 1
                main_rs = format!("{}{}\n", main_rs, "if s.len() == i {"); // if the vec is not big enough to support the new index append 0;
                main_rs = format!("{}{}\n", main_rs, "s.push(0);");
                main_rs = format!("{}{}\n", main_rs, "};");
            }

            "<" => {
                main_rs = format!("{}{}\n", main_rs, "i-=1;"); // Decrement the pointer by 1
            }

            "+" => {
                main_rs = format!("{}{}\n", main_rs, "s[i]+=1;"); // Increment cell by 1
            }

            "-" => {
                main_rs = format!("{}{}\n", main_rs, "s[i]-=1;"); // Decrement cell by 1
            }

            "." => {
                main_rs = format!("{}{}\n", main_rs, "print!(\"{}\", s[i] as char);"); // u8 and char are the same so just print the u8 as a char == toAscii()
            }

            "," => {
                main_rs = format!("{}{}\n", main_rs, "s[i]=gb();"); // the current cell is assigned a byte from stdin with get_byte_from_input()
            }

            "[" => {
                main_rs = format!("{}{}\n", main_rs, "while s[i] != 0 {"); // skip to matching ] if s[i]==0 is essentially the same thing as while (s[i]!=0) {
            }

            "]" => {
                main_rs = format!("{}{}\n", main_rs, "}"); // Just the closing part of the previous comment.
            }

            _ => {} // Will never occur but rust compiler does not know that
        }
    }

    main_rs = format!("{}{}\n", main_rs, "}"); // close fn main() {
    let main_rs_buf: Vec<u8> = main_rs.chars().map(|x| x as u8).collect(); // Turn main.rs string into vec of u8 which is basically the same thing as putting in every character (char == u8)
    let cargo_toml: Vec<u8> = cargo_toml.chars().map(|x| x as u8).collect(); // Turn Cargo.toml string into vec of u8 which is basically the same thing as putting in every character (char == u8)

    let mut zip = zip::ZipWriter::new(File::create(outfile).unwrap()); // Create the zip writer instance and give it a path
    let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored); // Create the file options

    match zip.add_directory("rustbf", options) { // Try to create a directory and handle error accordingly
        Err(_) => {
            eprintln!("{}", Colour::Yellow.paint("Error creating zipfile"));
            exit(-2);
        }
        Ok(_) => {}
    }

    match zip.add_directory("rustbf/src", options) { // Try to create a directory and handle error accordingly
        Err(_) => {
            eprintln!("{}", Colour::Yellow.paint("Error creating zipfile"));
            exit(-2);
        }
        Ok(_) => {}
    }

    match zip.start_file("rustbf/src/main.rs", options) { // Try to create a file and handle error accordingly
        Err(_) => {
            eprintln!("{}", Colour::Yellow.paint("Error creating zipfile"));
            exit(-3);
        }
        Ok(_) => {}
    }

    match zip.write(&main_rs_buf) { // Try to create a write and handle error accordingly
        Err(_) => {
            eprintln!("{}", Colour::Yellow.paint("Error creating zipfile"));
            exit(-4);
        }
        Ok(_) => {

        }
    }

    match zip.start_file("rustbf/Cargo.toml", options) { // Try to create a file and handle error accordingly
        Err(_) => {
            eprintln!("{}", Colour::Yellow.paint("Error creating zipfile"));
            exit(-3);
        }
        Ok(_) => {}
    }

    match zip.write(&cargo_toml) { // Try to create a write and handle error accordingly
        Err(_) => {
            eprintln!("{}", Colour::Yellow.paint("Error creating zipfile"));
            exit(-4);
        }
        Ok(_) => {

        }
    }

    match zip.finish() { // Try to complete the file and handle error accordingly
        Err(_) => {
            eprintln!("{}", Colour::Yellow.paint("Error creating zipfile"));
            exit(-5);
        }
        Ok(_) => {}
    };
}