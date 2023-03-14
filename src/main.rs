use clap::Parser;
use std::{
    fs::{self, File},
    io::Read,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    input_file: std::path::PathBuf,
}

mod exe;

fn main() {
    let args = Args::parse();
    let mut file_handle = File::open(args.input_file.clone()).unwrap();
    let file_size = fs::metadata(args.input_file.clone()).unwrap().len();
    let mut file_contents: Vec<u8> = Vec::with_capacity(file_size as usize);
    file_handle.read_to_end(&mut file_contents).unwrap();

    let mut file = exe::Executable::new(&mut file_contents);
    drop(file_contents);

    println!("AHK Version: {}", file.clone().unwrap().parse_ahk_version());
    println!("Compiler Version: {}", file.unwrap().parse_ahk_compiler());
}
