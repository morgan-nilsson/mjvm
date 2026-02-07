mod class_file;
use class_file::class_file::ClassFile;
use std::fs::File;
use std::io::BufReader;

static FILENAME: &str = "Main.class";

fn main() {
    
    let file = File::open(FILENAME).expect("Failed to open class file");
    let mut reader = BufReader::new(file);

    let class_file = ClassFile::from_reader(&mut reader);
    println!("Class File: {:?}", class_file);
}
