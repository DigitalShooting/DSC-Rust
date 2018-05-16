use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;

use session::Line;
use discipline::*;
use config::error::Error as ConfigError;


#[derive(Debug)]
pub struct Config {
    pub line: Line,
    pub disciplines: HashMap<String, Discipline>,
    pub default_discipline: Discipline,
}

impl Config {
    pub fn new(config_dir: &Path) -> Result<Config, ConfigError> {
        let line = Config::get_line(config_dir.join("line.json"))?;
        let targets = Config::get_targets(config_dir.join("targets/"));
        let disciplines = Config::get_disciplines(config_dir.join("disciplines/"), targets);
        let default_discipline = Config::get_default_discipline(config_dir.join("default_discipline.json"), &disciplines).unwrap();

        Ok(Config {
            line,
            disciplines,
            default_discipline,
        })
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
    fn read_file_new(path: PathBuf) -> io::Result<String> {
        let mut file = File::open(path)?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        return Ok(string);
    }

    fn get_line(path: PathBuf) -> Result<Line, ConfigError> {
        let raw_json = Config::read_file_new(path)?;
        let line: Line = serde_json::from_str(&raw_json)?;
        Ok(line)
    }



    fn get_default_discipline(path: PathBuf, disciplines: &HashMap<String, Discipline>) -> Result<Discipline, ConfigError> {
        let raw_json = Config::read_file_new(path)?;
        let json: serde_json::Value = serde_json::from_str(&raw_json)?;
        let discipline_id = json["name"].as_str().unwrap();
        Ok(disciplines.get(discipline_id).unwrap().clone())
    }


    // TODO return error
    fn get_disciplines(path: PathBuf, targets: HashMap<String, Target>) -> HashMap<String, Discipline> {
        let path = path.to_str().unwrap().to_string();
        let paths = fs::read_dir(path).unwrap();
        let mut disciplines: HashMap<String, Discipline> = HashMap::new();
        for dir_entry in paths {
            if let Ok(path) = dir_entry {
                match Config::get_discipline(path.path(), &targets) {
                    Ok((filename, discipline)) => {
                        disciplines.insert(filename, discipline);
                    },
                    Err(err) => println!("Error parsing discipline json at path {:?}: {}", path, err),
                }
            }
        }
        return disciplines;
    }

    fn get_discipline(path: PathBuf, targets: &HashMap<String, Target>) -> Result<(String, Discipline), ConfigError> {
        let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
        let raw_json = Config::read_file_new(path)?;
        let discipline_config: DisciplineConfig = serde_json::from_str(&raw_json)?;
        let discipline = DisciplineConfig::to_discipline(discipline_config, &targets)?;
        Ok((filename, discipline))
    }




    // TODO return error
    fn get_targets(path: PathBuf) -> HashMap<String, Target> {
        let path = path.to_str().unwrap().to_string();
        let paths = fs::read_dir(path).unwrap();
        let mut targets: HashMap<String, Target> = HashMap::new();
        for dir_entry in paths {
            if let Ok(path) = dir_entry {
                match Config::get_target(path.path()) {
                    Ok((filename, target)) => {
                        targets.insert(filename, target);
                    },
                    Err(err) => println!("Error parsing target json at path {:?}: {}", path, err),
                }
            }
        }
        return targets;
    }

    fn get_target(path: PathBuf) -> Result<(String, Target), ConfigError> {
        let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
        let raw_json = Config::read_file_new(path)?;
        let target: Target = serde_json::from_str(&raw_json)?;
        Ok((filename, target))
    }
}
