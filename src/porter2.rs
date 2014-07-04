#![crate_id = "porter2#0.1.0"]
#![crate_type = "lib"]

pub fn get(word: &str) -> Result<String, &str> {
    Ok(word.into_string())
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