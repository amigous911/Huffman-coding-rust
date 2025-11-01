use huffman_coding::{decode, encode};
use std::{env, fs, io::Write, path::Path, process};

struct Arguments {
    source_file: String,
    dest_file: String,
    decode_flag: bool,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        if args.len() > 5 {
            return Err("too many argumentss");
        }

        let mut source_file: Option<String> = None;
        let mut dest_file: Option<String> = None;
        let mut decode_flag = false;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-d" | "d" => {
                    decode_flag = true;
                }
                "-o" | "o" => {
                    if i + 1 >= args.len() {
                        return Err("-o requires a filename");
                    }
                    dest_file = Some(args[i + 1].clone());
                    i += 1;
                }
                default => {
                    source_file = Some(default.to_string());
                }
            }
            i += 1;
        }

        let source_file = match source_file {
            Some(value) => value,
            None => return Err("source file required"),
        };

        let dest_file = match dest_file {
            Some(value) => value,
            None => {
                if decode_flag {
                    match Path::new(&source_file).file_stem() {
                        Some(stem) => stem.to_string_lossy().to_string(),
                        None => source_file.clone(),
                    }
                } else {
                    format!("{}.huf", source_file)
                }
            }
        };

        Ok(Arguments {
            source_file,
            dest_file,
            decode_flag,
        })
    }
}

fn main() {
    // fetch arguments
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });
}
