use std::collections::HashMap;

use super::interface::Interface;
use super::target::Target;
use super::part::DisciplinePart;
use super::time::Time;
use super::error::Error as DisciplineError;



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Discipline {
    pub id: String,
    pub title: String,
    pub interface: Interface,
    pub time: Time,
    pub target: Target,
    pub parts: Vec<DisciplinePart>,
}

impl Discipline {
    // TODO maybe use reference for id
    pub fn get_part_from_type(&self, id: String) -> Option<DisciplinePart> {
        for part in self.parts.clone() {
            if part.id == id {
                return Some(part);
            }
        }
        return None;
    }
}




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisciplineConfig {
    id: String,
    title: String,
    interface: Interface,
    time: Time,
    target_name: String,
    parts: Vec<DisciplinePart>,
}

impl DisciplineConfig {
    pub fn to_discipline(config: DisciplineConfig, targets: &HashMap<String, Target>) -> Result<Discipline, DisciplineError> {
        match targets.get(&config.target_name) {
            Some(target) => Ok(Discipline {
                id: config.id,
                title: config.title,
                interface: config.interface,
                time: config.time,
                target: target.clone(),
                parts: config.parts,
            }),
            None => Err(DisciplineError::TargetNotFound),
        }
    }
}
