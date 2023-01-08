use std::{
    fs,
    path::PathBuf,
    process::exit
};
use structopt::StructOpt;
use ansi_term::Colour;

mod interpreters;
mod misc;

// Command line arguments are defined in the following struct
// Derive StructOpt is just so the struct has the necessary traits for StructOpt
#[derive(StructOpt)]
#[structopt(name = "basic")]
struct Opt { // Triple commented out lines are descriptors for the help menu
    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,

    /// Esolang to run against
    #[structopt(short, long)] // short means the flag can be abbreviated, long means the long version still works
    lang: String,

    /// Should attempt to compile?
    #[structopt(short, long)]
    compile: Option<bool> // Wrapping the type in an options declares that input as optional
}

fn main() {
    let opt = Opt::from_args(); // A struct of command line arguments
    let compile: bool = match opt.compile { // Convert None to flase and Some(b) to b
        None => {
            false
        }
        Some(b) => {
            b
        }
    };
    match opt.lang.to_lowercase().as_str() { // Select the right interpreter / compiler for the input language
        "brainfuck" => {
            if compile { // If the compile flag was set this should run
                println!("{}", Colour::Yellow.paint("Sorry Compilation not yet implemented for Brainfuck")); // Warn of non-implementation and exit with appropriate exit code ( see README.md )
                exit(2);
            } else {
                let content = match fs::read_to_string(opt.file) { // Read content to file or exit with appropriate code ( see README.md ) on error.
                    Err(_) => {
                        println!("{}", Colour::Red.paint("Error reading file"));
                        exit(-2);
                    }
                    Ok(c) => {
                        c
                    }
                };
                interpreters::brainfuck::run(content); // Hand over code in string form to the interpreter
            }
        }
        _ => {
            println!("Language not recognized");
            exit(0);
        }
    }
}