


#[derive(Serialize, Deserialize, Debug)]
pub struct WebColor {
    pub hex: String,
    pub alpha: f32,
}


// protocol Interface: Codable {
//     var name: String { get }
// }


#[derive(Serialize, Deserialize, Debug)]
pub struct Discipline {
    pub id: String,
    pub title: String,
//    var interface: Interface
    pub time: Time,
    pub target: Target,
    pub parts: Vec<DisciplinePart>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Time {
    InstantStart { duration: i32 },
    FirstShot { duration: i32 },
    None,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    pub title: String,
    pub rings: Vec<Ring>,
    pub rings_draw_only: Vec<Ring>,
    pub default_hit_color: WebColor,
    pub default_zoom: Zoom,
    pub min_zoom: Zoom,
    pub inner_ten: i32, // renamed innenZehner
    pub trial_corner_color: WebColor, // renamed probeEcke.color probeEcke.alpha
    pub bullet_diameter: f64, // renamed kugelDurchmesser
}

pub type Zoom = f32;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ring {
    pub value: i32,
    pub width:  f64,
    pub color: WebColor,
    pub has_text: bool, // renamed text
    pub text_color: WebColor,
    pub zoom: Zoom,
    pub hit_color: WebColor,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub enum PartCountMode {
    Integer,
    Tenth,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum PartAverage {
    Average { number_of_shots: i32 },
    None,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PartExitType {
    Always,
    BeforeFirst,
    None,
}
