extern crate porter2;

use std::io::File;
use std::io::BufferedReader;
use std::path;

fn main () {
    let input = File::open(&path::Path::new("test-data/voc.txt")).unwrap();
    let mut input_reader = BufferedReader::new(input);

    let mut output = match File::create(&path::Path::new("test-data/porter2-output.txt")) {
        Ok(file) => file,
        Err(_)   => fail!("Something went wrong with creating the file"),
    };

    loop {
        match input_reader.read_line() {
            Ok(word) => match porter2::get(word.as_slice().trim()) {
                Ok(result) => match output.write_line(result.as_slice()) {
                    Ok(_)  => continue,
                    Err(_) => fail!("Couldn't write to file"),
                },
                Err(_)     => println!("Something went wrong with stemming"),
            },
            Err(_) => break,
        }
    }
}