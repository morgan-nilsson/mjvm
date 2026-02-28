use clap::Parser;
use log::debug;
use std::fs::File;
use std::io::BufReader;

use mjvm::class_file::ClassFile;

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

    init_logging();

    let args = Args::parse();

    debug!("Input file: {}", args.input);


    let file = File::open(args.input).expect("Failed to open class file");
    let mut reader = BufReader::new(file);

    let class_file = ClassFile::from_reader(&mut reader);
    println!("Parsed class file: {:#?}", class_file);

    println!("DONE.");
}

use simplelog::*;
use std::time::{SystemTime, UNIX_EPOCH};

fn init_logging() {
    // create a timestamped filename
    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards");

    // create logs directory if it doesn't exist
    std::fs::create_dir_all("logs").expect("Failed to create logs directory");
    let filename = format!("logs/log_{}.log", start.as_secs());

    let file = File::create(&filename).unwrap();

    CombinedLogger::init(vec![
        // logger for stdout
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        // logger for file
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            file,
        ),
    ]).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_true() {
        assert!(true);
    }
}