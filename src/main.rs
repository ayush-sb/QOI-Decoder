pub mod qoidecoder;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::env;

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = String::from(&args[1]);
    let my_vec = get_file_as_byte_vec(&filename);
    let bytes = &my_vec[0..];
    let _res = crate::qoidecoder::readimg::get_pixels(&bytes);
}