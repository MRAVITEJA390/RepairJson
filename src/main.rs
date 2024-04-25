use std::fs::{File};
use std::io;
use std::io::{BufRead, Write};
use std::sync::{Mutex};
use rayon::prelude::*;
use clap::Parser;

#[derive(Parser)]
struct CommandArgs {
    ///input file location
    #[arg(short, long)]
    input: String,

    /// output file location
    #[arg(short, long)]
    output: String,
}


fn repair_json_line(line: &str) -> String {
    let mut inside_string = false;
    let mut fixed_line = String::new();
    let mut last_char = '\0';
    for ch in line.chars() {
        if ch == '"' && last_char != '\\' {
            inside_string = !inside_string;
            // handle.write_all(ch.encode_utf8(&mut [0; 4]).as_bytes())?;
        } else if ch == ';' && !inside_string {
            fixed_line.push(':');
            last_char = ch;
            continue;
        }
        fixed_line.push(ch);
        last_char = ch;
    }
    fixed_line.push('\n');
    fixed_line
}


fn main() -> io::Result<()> {
    let std_in;
    let std_in_file;
    let std_in_reader;
    let file_reader;
    match CommandArgs::try_parse() {
        Err(_err) => {
            std_in = io::stdin();
            std_in_reader = io::BufReader::new(std_in);
            std_in_reader.lines().par_bridge().for_each(|line|
                {
                    let fixed_line = repair_json_line(&line.unwrap());
                    io::stdout().write_all(fixed_line.as_bytes()).unwrap();
                }
            );
        }
        Ok(args) => {
            let input_path = args.input.as_str();
            std_in_file = File::open(input_path)?;
            file_reader = io::BufReader::new(std_in_file);
            let std_out_file = Mutex::new(File::create(args.output.as_str())?);
            file_reader.lines().par_bridge().for_each(|line|
                {
                    let fixed_line = repair_json_line(&line.unwrap());
                    std_out_file.lock().unwrap().write_all(fixed_line.as_bytes()).unwrap();
                }
            );
        }
    }
    Ok(())
}

