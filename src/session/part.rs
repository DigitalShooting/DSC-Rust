use session::shot::*;
use session::series::*;
use discipline::*;




#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    pub series: Vec<Series>,
    pub part_type: PartType,
    sum: f64,
}

pub type PartType = String;

impl Part {
    // fn new(discipline: Discipline) -> Part {
    pub fn new() -> Part {
        Part {
            series: vec![
                Series::new(),
            ],
            part_type: String::from("probe"),
            sum: 0_f64,
        }
    }

    // Add a new series to the part an update the active series index
    fn new_series(&mut self) {
        let new_series = Series::new();
        self.series.push(new_series);
    }

    fn get_discipline_part<'a>(&self, discipline: &'a Discipline) -> Option<&'a DisciplinePart> {
        for part in &discipline.parts {
            if part.id == self.part_type {
                return Some(part)
            }
        }
        return None
    }
}






impl AddShotWithDiscipline for Part {
    fn add_shot(&mut self, shot: Shot, discipline: &Discipline) {
        match self.get_discipline_part(discipline) {
            Some(discipline_part) => {
                // Add the ring count to the part sum
                // TODO round
                self.sum += shot.ring_count;
                self.sum = (self.sum*10_f64).round() / 10_f64;

                // Add new series if the current series is full
                let mut index = self.series.len()-1;
                if self.series[index].is_full(discipline_part) {
                    self.new_series();
                }

                // add shot to the active series
                index = self.series.len()-1;
                self.series[index].add_shot(shot, discipline);
            },
            None => println!("ERROR - discipline_part not found."),
        }
    }
}
















#[cfg(test)]
mod test {
    use session::shot::*;
    use session::session::*;
    use discipline::*;
    use helper;

    #[test]
    fn test_add_shot() {
        let target = helper::dsc_demo::lg_target();
        let discipline = helper::dsc_demo::lg_discipline();
        let shot = Shot::from_cartesian_coordinates (0, 0, &target);

        let mut session = Session::new(discipline);
        assert_eq!(1, session.parts.len());
        assert_eq!(0, session.active_session);

        session.add_shot(shot);
    }

}
