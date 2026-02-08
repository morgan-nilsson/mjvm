use clap::Parser; 

mod class_file;
use class_file::class_file::ClassFile;
use std::fs::File;
use std::io::BufReader;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {

    // verbose flag
    #[arg(short, long)]
    verbose: bool,

    // main class file to run
    #[arg(value_name = "FILE")]
    input: String

}

fn main() {

    let args = Args::parse();
    println!("Args: {:?}", args);
    
    let file = File::open(args.input).expect("Failed to open class file");
    let mut reader = BufReader::new(file);

    let class_file = ClassFile::from_reader(&mut reader);
    println!("Class File: {:?}", class_file);
}
