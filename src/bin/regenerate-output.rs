extern crate porter2;

use std::io::File;
use std::io::BufferedReader;
use std::path;
use std::os;

fn main () {
    let args = os::args();
    let filename = args.get(1).as_slice();

    let input = File::open(&path::Path::new(filename)).unwrap();
    let mut input_reader = BufferedReader::new(input);

    loop {
        match input_reader.read_line() {
            Ok(word) => match porter2::get(word.as_slice().trim()) {
                Ok(result) => println!("{}", result),
                Err(_)     => println!("Something went wrong with stemming"),
            },
            Err(_) => break,
        }
    }
}