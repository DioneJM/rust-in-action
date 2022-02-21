use std::env;
use std::fs::File;
use std::io::Read;

const BYTES_PER_LINE: usize = 16;

fn main() -> std::io::Result<()> {
    let arg1 = env::args().nth(1);
    let file_name = arg1.expect("usage: fview FILE_NAME");
    let mut file = File::open(&file_name).expect(&format!("File not found: {}", file_name));

    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    while let Ok(_) = file.read_exact(&mut buffer) {
        print!("[0x{:08x}] ", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(". "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte)
            }
        }
        println!();
        pos += BYTES_PER_LINE;
    }

    Ok(())
}
