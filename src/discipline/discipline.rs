use discipline::interface::Interface;
use discipline::target::Target;
use discipline::part::DisciplinePart;
use discipline::time::Time;



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
    pub fn get_part_from_type(&self, id: String) -> Option<DisciplinePart> {
        for part in self.parts.clone() {
            if part.id == id {
                return Some(part);
            }
        }
        return None;
    }
}
