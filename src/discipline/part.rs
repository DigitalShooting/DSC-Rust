use discipline::interface::Interface;
use discipline::target::Target;
use discipline::time::Time;



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisciplinePart {
    pub id: String,
    pub name: String,
    pub has_trial_corner: bool, // renamed probeEcke
    pub main_part: bool, // TODO in use?
    pub enable_reset_to_new_target: bool, // renamed neueScheibe
    pub series_length: i32, // renamed serienLength
    pub number_of_shots: i32, // renamed anzahlShots
    pub show_infos: bool,
    pub count_mode: PartCountMode,
    pub time: Time,
    pub average: PartAverage,
    pub exit_type: PartExitType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PartCountMode {
    Integer,
    Tenth,
}
impl PartCountMode {
    pub fn to_string(self, value: f64) -> String {
        match self {
            PartCountMode::Integer => format!("{:.0}", value),
            PartCountMode::Tenth => format!("{:.1}", value),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum PartAverage {
    Average { number_of_shots: i32 },
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PartExitType {
    Always,
    BeforeFirst,
    None,
}
