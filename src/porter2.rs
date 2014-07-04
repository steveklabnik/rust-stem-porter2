#![crate_id = "porter2#0.1.0"]
#![crate_type = "lib"]

use std::ascii;
use std::ascii::Ascii;
use std::vec::Vec;

pub struct Stemmer {
    b: Vec<ascii::Ascii>,
    k: uint,
    j: uint,
}

impl Stemmer {
    pub fn new(word: &str) -> Result<Stemmer, &str> {
        if !word.is_ascii() {
            Err("Only support English words with ASCII characters")
        } else {
            let b = unsafe { word.to_ascii_nocheck().to_lower() };
            let k = b.len();
            Ok(Stemmer {
                b: b,
                k: k,
                j: 0,
            })
        }
    }

    pub fn get(&self) -> String {
        let borrowed = self.b.slice_to(self.k);
        borrowed.as_str_ascii().into_string()
    }

    /// stem.ends(s) is true <=> [0, k) ends with the string s.
    pub fn ends(&mut self, s: &str) -> bool {
        let len = s.len();
        let k = self.k;
        if s[len - 1] != self.b.get(k-1).to_byte() { return false } /* tiny speed-up */
        if len > k { return false }
        let mut iter = s.bytes();
        for ac in self.b.slice(k - len, k).iter() {
            if ac.to_byte() != iter.next().unwrap() { return false }
        }
        self.j = k - len;
        return true;
    }

    pub fn step1a(&mut self) {
        if self.ends("sses") {
            self.k -= 2;
        }
    }
}

pub fn get(word: &str) -> Result<String, &str> {
    if word.len() > 2 {
        match Stemmer::new(word) {
            Ok(w) => {
                let mut mw = w;
                mw.step1a();
                Ok(mw.get())
            }
            Err(e) => Err(e),
        }
    } else {
        Ok(word.into_string())
    }
}

#[cfg(test)]
mod test {
    use std::io::File;
    use std::io::BufferedReader;
    use std::path;

    use super::get;

    #[test]
    fn lexicon() {
        let input = File::open(&path::Path::new("test-data/voc.txt")).unwrap();
        let result = File::open(&path::Path::new("test-data/porter2-output.txt")).unwrap();
        let mut input_reader = BufferedReader::new(input);
        let mut result_reader = BufferedReader::new(result);
        loop {
          match input_reader.read_line() {
              Ok(word) => match get(word.as_slice().trim()) {
                  Ok(stem) => {
                      match result_reader.read_line() {
                          Ok(answer) => if answer.as_slice().trim() != stem.as_slice() {
                              fail!("\n[FAILED] '{:s}' != '{:s}'", stem, answer);
                          } else {
                              print!(".");
                          },
                          Err(_) => break,
                      }
                  },
                  Err(e) => fail!("\n[FAILED] Cannot get stem for '{:s}': {:s}", word, e),
              },
              Err(_) => break,
          }
        }
        println!("");
    }

}