extern crate porter2;

use std::io;

fn main () {
    for line in io::stdin().lines() {
        match line {
            Ok(word) => match porter2::get(word.as_slice().trim()) {
                Ok(result) => println!("{}", result),
                Err(_)     => println!("Something went wrong with stemming"),
            },
            Err(_) => break,
        }
    }
}