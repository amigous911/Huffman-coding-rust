use huffman_coding::{decode, encode};
use std::{
    env, fs,
    io::{BufReader, Read, Write},
    path::Path,
    process,
};

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

    // prepare files
    let mut source_file = fs::File::open(&arguments.source_file).unwrap_or_else(|err| {
        eprintln!(
            "problem opening source file {}: {}",
            arguments.source_file, err
        );
        process::exit(1);
    });
    let mut output_file = fs::File::create(&arguments.dest_file).unwrap_or_else(|err| {
        eprintln!(
            "problem creating output file {}: {}",
            arguments.dest_file, err
        );
        process::exit(2);
    });

    // encode/decode
    let result: Vec<u8> = if arguments.decode_flag {
        decode(BufReader::new(source_file)).unwrap_or_else(|err| {
            eprintln!("problem decoding file {}: {}", arguments.source_file, err);
            process::exit(3);
        })
    } else {
        let mut data: Vec<u8> = Vec::new();
        source_file.read_to_end(&mut data).unwrap_or_else(|_| {
            eprintln!("problem reading source file {}", arguments.source_file);
            process::exit(1);
        });
        encode(data).unwrap_or_else(|err| {
            eprintln!("problem encoding file {}: {}", arguments.source_file, err);
            process::exit(3);
        })
    };

    // write result
    output_file.write_all(&result).unwrap_or_else(|err| {
        eprintln!(
            "problem writing to output file {}: {}",
            arguments.dest_file, err
        );
        process::exit(2);
    });
}
