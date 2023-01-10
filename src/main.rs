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
    lang: String
}

fn main() {
    let opt = Opt::from_args(); // A struct of command line arguments

    match opt.lang.to_lowercase().as_str() { // Select the right interpreter / compiler for the input language
        "brainfuck" => {
            let content = match fs::read_to_string(opt.file) { // Read content to file or exit with appropriate code ( see README.md ) on error.
                Err(_) => {
                    eprintln!("{}", Colour::Red.paint("Error reading file"));
                    exit(-2);
                }
                Ok(c) => {
                    c
                }
            };
            interpreters::brainfuck::run(content); // Hand over code in string form to the interpreter
        }
        _ => {
            eprintln!("Language not recognized");
            exit(0);
        }
    }
}