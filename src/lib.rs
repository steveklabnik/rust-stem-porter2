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

    /// stem.measure() measures the number of consonant sequences in [0, j).
    /// if c is a consonant sequence and v a vowel sequence, and <..> indicates
    /// arbitrary presence,
    ///
    ///    <c><v>       gives 0
    ///    <c>vc<v>     gives 1
    ///    <c>vcvc<v>   gives 2
    ///    <c>vcvcvc<v> gives 3
    ///    ....
    ///
    pub fn measure(&self) -> uint {
        let mut n = 0u;
        let mut i = 0u;
        let j = self.j;
        loop {
            if i >= j { return n }
            if !self.is_consonant(i) { break }
            i += 1;
        }
        i += 1;
        loop {
            loop {
                if i >= j { return n }
                if self.is_consonant(i) { break }
                i += 1;
            }
            i += 1;
            n += 1;
            loop {
                if i >= j { return n }
                if !self.is_consonant(i) { break }
                i += 1;
            }
            i += 1;
        }
    }

    /// stem.is_consonant(i) is true <=> stem[i] is a consonant
    pub fn is_consonant(&self, i: uint) -> bool {
        match self.b[i].to_char() {
            'a' | 'e' | 'i' | 'o' | 'u' => false,
            'y' => if i == 0 {
                true
            } else {
                !self.is_consonant(i - 1)
            },
            _ => true,
        }
    }

    /// stem.has_vowel() is TRUE <=> [0, range_end-1) contains a vowel
    pub fn has_vowel(&self, range_end: uint) -> bool {
        for i in range(0, range_end) {
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
        } else if !(self.ends("us") || self.ends("ss")) && self.ends("s") {
            if self.has_vowel(self.k - 2) {
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