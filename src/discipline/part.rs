use super::time::Time;
use session::CountMode;



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisciplinePart {
    pub id: String,
    pub name: String,
    pub has_trial_corner: bool, // renamed probeEcke
    pub main_part: bool, // TODO in use?
    pub enable_reset_to_new_target: bool, // renamed neueScheibe
    pub series_length: i32, // renamed serienLength
    pub number_of_shots: Option<i32>, // renamed anzahlShots
    pub show_infos: bool,
    pub count_mode: CountMode,
    pub time: Time,
    pub average: PartAverage,
    pub exit_type: PartExitType,
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
