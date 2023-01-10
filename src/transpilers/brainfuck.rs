use std::{
    process::exit,
    path::PathBuf,
    str::FromStr,
    fs::File, io::Write,
};
use ansi_term::Colour;


pub fn transpile(code: String, outfile_option: Option<PathBuf>) {
    let outfile: PathBuf = match outfile_option {
        None => {
            PathBuf::from_str("./out.zip").unwrap()
        }
        Some(p) => {
            PathBuf::from_str(format!("{}.zip", p.to_str().unwrap()).as_str()).unwrap()
        }
    };

    let cargo_toml: String = "[package]\nname = \"rustbf\"\nversion = \"0.1.0\"\nedition = \"2021\"".to_owned();
    let mut main_rs: String = "use std::io::Read;\n\nfn gb() -> u8 {\nmatch std::io::stdin().bytes().next().and_then(|result| result.ok()) {\nNone => {\n return 0;\n}\nSome(v) => {\nif v == 4 {\nreturn 0;\n} else {\nreturn v;\n}\n}\n}\n}\n\n\nfn main() {\nlet mut s: Vec<u8> = vec![0];\nlet mut i: usize = 0;\n".to_string();

    let instruction_vector: Vec<String> = code.chars().map(|c| c.to_string()).filter(|s| "><+-,.[]".to_owned().contains(s)).collect();

    for instruction in instruction_vector {
        match instruction.as_str() {
            ">" => {
                main_rs = format!("{}{}\n", main_rs, "i+=1;");
                main_rs = format!("{}{}\n", main_rs, "if s.len() == i {");
                main_rs = format!("{}{}\n", main_rs, "s.push(0);");
                main_rs = format!("{}{}\n", main_rs, "};");
            }

            "<" => {
                main_rs = format!("{}{}\n", main_rs, "i-=1;");
            }

            "+" => {
                main_rs = format!("{}{}\n", main_rs, "s[i]+=1;");
            }

            "-" => {
                main_rs = format!("{}{}\n", main_rs, "s[i]-=1;");
            }

            "." => {
                main_rs = format!("{}{}\n", main_rs, "print!(\"{}\", s[i] as char);");
            }

            "," => {
                main_rs = format!("{}{}\n", main_rs, "s[i]=gb();");
            }

            "[" => {
                main_rs = format!("{}{}\n", main_rs, "while s[i] != 0 {");
            }

            "]" => {
                main_rs = format!("{}{}\n", main_rs, "}");
            }
            
            _ => {}
        }
    }

    main_rs = format!("{}{}\n", main_rs, "}");
    let main_rs_buf: Vec<u8> = main_rs.chars().map(|x| x as u8).collect();
    let cargo_toml: Vec<u8> = cargo_toml.chars().map(|x| x as u8).collect();

    let mut zip = zip::ZipWriter::new(File::create(outfile).unwrap());
    let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    match zip.add_directory("rustbf", options) {
        Err(_) => {
            eprintln!("{}", Colour::Yellow.paint("Error creating zipfile"));
            exit(-2);
        }
        Ok(_) => {}
    }

    match zip.add_directory("rustbf/src", options) {
        Err(_) => {
            eprintln!("{}", Colour::Yellow.paint("Error creating zipfile"));
            exit(-2);
        }
        Ok(_) => {}
    }

    match zip.start_file("rustbf/src/main.rs", options) {
        Err(_) => {
            eprintln!("{}", Colour::Yellow.paint("Error creating zipfile"));
            exit(-3);
        }
        Ok(_) => {}
    }

    match zip.write(&main_rs_buf) {
        Err(_) => {
            eprintln!("{}", Colour::Yellow.paint("Error creating zipfile"));
            exit(-4);
        }
        Ok(_) => {

        }
    }

    match zip.start_file("rustbf/Cargo.toml", options) {
        Err(_) => {
            eprintln!("{}", Colour::Yellow.paint("Error creating zipfile"));
            exit(-3);
        }
        Ok(_) => {}
    }

    match zip.write(&cargo_toml) {
        Err(_) => {
            eprintln!("{}", Colour::Yellow.paint("Error creating zipfile"));
            exit(-4);
        }
        Ok(_) => {

        }
    }

    match zip.finish() {
        Err(_) => {
            eprintln!("{}", Colour::Yellow.paint("Error creating zipfile"));
            exit(-5);
        }
        Ok(_) => {}
    };
}