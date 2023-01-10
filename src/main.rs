use std::{
    fs,
    path::PathBuf,
    process::exit
};
use structopt::StructOpt;
use ansi_term::Colour;

mod interpreters;
mod transpilers;
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

    // Weather to transpile or interpret the file
    #[structopt(short, long)]
    transpile: Option<bool>,

    // Output file
    #[structopt(short, long, parse(from_os_str))]
    outfile: Option<PathBuf>
}

fn main() {
    let opt = Opt::from_args(); // A struct of command line arguments

    let transpile = match opt.transpile {
        None => {
            false
        }
        Some(b) => {
            b
        }
    };

    match opt.lang.to_lowercase().as_str() { // Select the right interpreter / compiler for the input language
        "brainfuck" => {
            if transpile {
                let content = match fs::read_to_string(opt.file) { // Read content to file or exit with appropriate code ( see README.md ) on error.
                    Err(_) => {
                        eprintln!("{}", Colour::Red.paint("Error reading file"));
                        exit(-2);
                    }
                    Ok(c) => {
                        c
                    }
                };
                transpilers::brainfuck::transpile(content, opt.outfile); // Hand over code in string form to the transpiler
            } else {
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
        }

        "ambored++" => {
            let content = match fs::read_to_string(opt.file) { // Read content to file or exit with appropriate code ( see README.md ) on error.
                Err(_) => {
                    eprintln!("{}", Colour::Red.paint("Error reading file"));
                    exit(-2);
                }
                Ok(c) => {
                    c
                }
            };
            interpreters::am_bored_mm::run(content); // Hand over code in string form to the interpreter
        }

        "4dchess" => {
            let content = match fs::read_to_string(opt.file) { // Read content to file or exit with appropriate code ( see README.md ) on error.
                Err(_) => {
                    eprintln!("{}", Colour::Red.paint("Error reading file"));
                    exit(-2);
                }
                Ok(c) => {
                    c
                }
            };
            interpreters::_4dchess::run(content); // Hand over code in string form to the interpreter
        }
        _ => {
            eprintln!("Language not recognized");
            exit(0);
        }
    }
}