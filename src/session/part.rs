use session::{Counter, CountMode};
use session::shot::*;
use session::series::*;
use discipline::*;




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Part {
    pub series: Vec<Series>,
    pub part_type: PartType,
    sum: Counter,
    number_of_shots: i32,
}

pub type PartType = String;

impl Part {
    /// New empty part
    pub fn new(part_type: PartType) -> Part {
        Part {
            series: vec![
                Series::new(),
            ],
            part_type,
            sum: Counter::empty(),
            number_of_shots: 0,
        }
    }

    /// Add a new series to the part an update the active series index
    fn new_series(&mut self) {
        let new_series = Series::new();
        self.series.push(new_series);
    }

    /// Return the current DisciplinePart from the given Discipline
    pub fn get_discipline_part<'a>(&self, discipline: &'a Discipline) -> Option<&'a DisciplinePart> {
        for part in &discipline.parts {
            if part.id == self.part_type {
                return Some(part)
            }
        }
        return None
    }
}






impl AddShot for Part {
    fn add_shot(&mut self, shot: Shot, discipline: &Discipline, count_mode: &CountMode) {
        match self.get_discipline_part(discipline) {
            Some(discipline_part) => {
                // Add the ring count to the part sum
                self.sum.add(shot.ring_count, &count_mode);
                self.number_of_shots += 1;

                // Add new series if the current series is full
                let mut index = self.series.len()-1;
                if self.series[index].is_full(discipline_part) {
                    self.new_series();
                }

                // add shot to the active series
                index = self.series.len()-1;
                self.series[index].add_shot(shot, discipline, count_mode);
            },
            None => println!("ERROR - discipline_part not found."),
        }
    }
}
















// #[cfg(test)]
// mod test {
//     use session::shot::*;
//     use session::session::*;
//     use session::info::*;
//     use discipline::*;
//     use helper;
//
//
//
// }
