use std::io;
use std::fs;
use std::io::Read;

pub fn read_file(file_path: &str) -> Result<String, io::Error>  {
    let file = fs::File::open(file_path)?;
    let mut buf_reader = io::BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}
