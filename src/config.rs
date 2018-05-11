use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::io;

use session::Line;



#[derive(Debug)]
pub struct Config {
    line: Line,
}

impl Config {
    pub fn new(config_dir: String) -> Config {
        Config {
            line: Config::get_line(config_dir + &"line.json".to_string()),
        }
    }

    /// Read file at given path and return its content
    /// path:       path of the file to read
    /// return:     error of content of the file
    fn read_file(path: String) -> io::Result<String> {
        let mut file = File::open(path)?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        return Ok(string);
    }

    fn get_line(path: String) -> Line {
        let raw_json = Config::read_file(path.clone())
            .expect(&format!("reading line config file (at path {})", path));
        let line: Line = serde_json::from_str(&raw_json)
            .expect(&format!("parsing json in line config file (at path {})", path));
        return line;
    }
}
