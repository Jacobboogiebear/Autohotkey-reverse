use std::fs::File;
use std::io::Write;

use clap::{arg, command, Parser};
use data_extractor::Extractor;
use owo_colors::colors::*;
use owo_colors::OwoColorize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_name = "Input")]
    input: String,
    #[arg(value_name = "Output")]
    output: String,
}

fn main() {
    let args = Args::parse();
    let input_file = File::open(args.input.clone());
    let output_file = File::create(args.output);
    if input_file.is_ok() {
        if output_file.is_ok() {
            drop(input_file);
            let mut output_file = output_file.unwrap();
            let script = Extractor::default(args.input.clone());
            if script.is_some() {
                let source = script.unwrap().obtain_source();
                output_file.write_all(source.as_bytes()).unwrap();
                println!(
                    "{}",
                    "Success: Your AHK Script has been extracted from the binary"
                        .fg::<BrightGreen>()
                )
            } else {
                println!("{}", "Error: Invalid AHK Script binary".fg::<BrightRed>());
            }
        } else {
            println!(
                "{}",
                "Error: Folder for output file doesn't exist".fg::<BrightRed>()
            );
            return;
        }
    } else {
        println!("{}", "Error: Input file doesn't exist".fg::<BrightRed>());
        return;
    }
}
