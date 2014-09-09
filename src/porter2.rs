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

    /// stem.is_consonant(i) is true <=> stem[i] is a consonant
    pub fn is_consonant(&self, i: uint) -> bool {
        match self.b.get(i).to_char() {
            'a' | 'e' | 'i' | 'o' | 'u' => false,
            'y' => if i == 0 {
                true
            } else {
                !self.is_consonant(i - 1)
            },
            _ => true,
        }
    }

    /// stem.has_vowel() is TRUE <=> [0, j-1) contains a vowel
    pub fn has_vowel(&self) -> bool {
        for i in range(0, self.j) {
            if !self.is_consonant(i) {
                return true;
            }
        }
        return false;
    }

    pub fn get(&self) -> String {
        let borrowed = self.b.slice_to(self.k);
        borrowed.as_str_ascii().into_string()
    }

    /// stem.ends(s) is true <=> [0, k) ends with the string s.
    pub fn ends(&mut self, s: &str) -> bool {
        self.get().as_slice().ends_with(s)
    }

    pub fn step1a(&mut self) {
        if self.ends("sses") {
            self.k -= 2;
        } else if self.ends("ied") || self.ends("ies") {
            if self.k > 4 {
                self.k -= 2;
            } else {
                self.k -= 1;
            }
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