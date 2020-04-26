use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;

use session::Line;
use discipline::*;
use config::error::Error as ConfigError;

type Result<T> = std::result::Result<T, ConfigError>;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "mode")]
pub enum DatabaseConfig {
    None,
    FileSystem {
        path: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSocketConfig {
    // Local URL:PORT to bind websocket server to
    pub url: String,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MainConfig {
    pub line: Line,
    pub default_discipline: String,
    pub database: DatabaseConfig,
    pub websocket: WebSocketConfig,
}




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub line: Line,
    pub disciplines: HashMap<String, Discipline>,
    pub default_discipline: Discipline,
    pub database: DatabaseConfig,
    pub websocket: WebSocketConfig,
}

impl Config {
    pub fn new(config_dir: &Path, modes_dir: &Path) -> Result<Config> {
        
        
        
        let targets = Config::parse_targets(modes_dir.join("targets/"))?;
        let disciplines = Config::parse_disciplines(modes_dir.join("disciplines/"), targets)?;
        
        let config = Config::parse_config(config_dir.to_path_buf(), &disciplines)?;
        let default_discipline = Config::get_default_discipline(config.default_discipline, &disciplines)?;

        
        
        // let line = Config::parse_line(config_dir.join("line.json"))?;
        // let default_discipline = Config::parse_default_discipline(config_dir.join("default_discipline.json"), &disciplines)?;
        // let websocket = Config::parse_websocket(config_dir.join("websocket.json"))?;
        // let database = Config::parse_database(config_dir.join("database.json"))?;
        
        Ok(Config {
            line: config.line,
            disciplines,
            default_discipline,
            database: config.database,
            websocket: config.websocket,
        })
    }



    pub fn get_discipline(&self, name: &str) -> Option<Discipline> {
        match self.disciplines.get(name) {
            Some(discipline) => Some(discipline.clone()),
            None => None,
        }
    }






    /// Read file at given path and return its content
    /// path:       path of the file to read
    /// return:     error of content of the file
    fn read_file(path: PathBuf) -> io::Result<String> {
        let mut file = File::open(path)?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        return Ok(string);
    }


    // fn parse_line(path: PathBuf) -> Result<Line> {
    //     let raw_json = Config::read_file(path)?;
    //     let line: Line = serde_json::from_str(&raw_json)?;
    //     Ok(line)
    // }

    // fn parse_websocket(path: PathBuf) -> Result<WebSocketConfig> {
    //     let raw_json = Config::read_file(path)?;
    //     let websocket: WebSocketConfig = serde_json::from_str(&raw_json)?;
    //     Ok(websocket)
    // }
    
    // fn parse_database(path: PathBuf) -> Result<DatabaseConfig> {
    //     let raw_json = Config::read_file(path)?;
    //     let database_config: DatabaseConfig = serde_json::from_str(&raw_json)?;
    //     Ok(database_config)
    // }



    fn parse_config(path: PathBuf, disciplines: &HashMap<String, Discipline>) -> Result<MainConfig> {
        let raw_json = Config::read_file(path)?;
        let config: MainConfig = serde_json::from_str(&raw_json)?;
        Ok(config)
    }
    
    

    fn get_default_discipline(default_discipline_id: String, disciplines: &HashMap<String, Discipline>) -> Result<Discipline> {
        match disciplines.get(&default_discipline_id) {
            Some(discipline) => Ok(discipline.clone()),
            None => Err(ConfigError::DefaultDisciplineNotFound),
        }
    }



    fn parse_disciplines(path: PathBuf, targets: HashMap<String, Target>) -> Result<HashMap<String, Discipline>> {
        let paths = fs::read_dir(path)?;
        let mut disciplines: HashMap<String, Discipline> = HashMap::new();
        for dir_entry in paths {
            if let Ok(path) = dir_entry {
                match Config::parse_discipline(path.path(), &targets) {
                    Ok((filename, discipline)) => {
                        disciplines.insert(filename, discipline);
                    },
                    Err(err) => {
                        return Err(ConfigError::DisciplineParsing(path.path(), Box::new(err)));
                    },
                }
            }
        }
        return Ok(disciplines);
    }

    fn parse_discipline(path: PathBuf, targets: &HashMap<String, Target>) -> Result<(String, Discipline)> {
        let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
        let raw_json = Config::read_file(path)?;
        let discipline_config: DisciplineConfig = serde_json::from_str(&raw_json)?;
        let discipline = DisciplineConfig::to_discipline(discipline_config, &targets)?;
        Ok((filename, discipline))
    }




    fn parse_targets(path: PathBuf) -> Result<HashMap<String, Target>> {
        let paths = fs::read_dir(path)?;
        let mut targets: HashMap<String, Target> = HashMap::new();
        for dir_entry in paths {
            if let Ok(path) = dir_entry {
                match Config::parse_target(path.path()) {
                    Ok((filename, target)) => {
                        targets.insert(filename, target);
                    },
                    Err(err) => {
                        return Err(ConfigError::TargetParsing(path.path(), Box::new(err)));
                    },
                }
            }
        }
        return Ok(targets);
    }

    fn parse_target(path: PathBuf) -> Result<(String, Target)> {
        let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
        let raw_json = Config::read_file(path)?;
        let target: Target = serde_json::from_str(&raw_json)?;
        Ok((filename, target))
    }
}
