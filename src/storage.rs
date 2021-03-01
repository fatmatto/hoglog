// Possible search backends
// https://github.com/valeriansaliou/sonic
// https://tantivy-search.github.io/examples/basic_search.html è una libreria permetterebbe di avere un unico processo per il receiver

use std::fs::OpenOptions;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Error, Write};
pub struct Log {
  time: String,
  level: String,
  message: String,
}

pub fn write(log: String) -> Result<(), Error> {
  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open("hog.log")
    .unwrap();

  let log_to_write = format!("{}\n", log);
  file
    .write_all(log_to_write.as_bytes())
    .expect("write failed");

  Ok(())
}

pub fn read() -> Result<String, Error> {
  let path = "hog.log";

  // let input = File::open(path)?;
  // let buffered = BufReader::new(input);

  // for line in buffered.lines() {
  //   println!("{}", line?);
  // }
  //Ok(buffered.as_string())
  let content: String = read_to_string(path)?;
  Ok(content)
}
