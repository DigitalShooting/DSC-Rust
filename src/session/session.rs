

use shot::*;
use user::*;
use discipline::*;






#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub parts: Vec<Part>,
    pub user: User,
    pub club: Club,
    pub team: Team,
    active_session: usize,
}

impl Session {
    // fn new(discipline: Discipline) -> Session {
    pub fn new() -> Session {
        Session {
            parts: vec![
                Part::new(),
            ],
            user: User::empty(),
            club: Club::empty(),
            team: Team::empty(),
            active_session: 0,
        }
    }
}




#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    pub series: Vec<Series>,
    pub part_type: PartType,
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




#[derive(Serialize, Deserialize, Debug)]
pub struct Series {
    pub shots: Vec<Shot>,
}

impl Series {
    pub fn new() -> Series {
        Series {
            shots: vec![],
        }
    }
}

impl Series {
    fn is_full<'a, 'b>(&'a self, discipline_part: &'b DisciplinePart) -> bool {
        return self.shots.len() as i32 >= discipline_part.series_length
    }
}









pub trait AddShot {
    fn add_shot(&mut self, Shot, &Discipline);
}

impl AddShot for Session {
    fn add_shot(&mut self, shot: Shot, discipline: &Discipline) {
        let active_session = &mut self.parts[self.active_session];
        active_session.add_shot(shot, discipline);
    }
}

impl AddShot for Part {
    fn add_shot(&mut self, shot: Shot, discipline: &Discipline) {
        match self.get_discipline_part(discipline) {
            Some(discipline_part) => {
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

impl AddShot for Series {
    fn add_shot(&mut self, shot: Shot, _discipline: &Discipline) {
        self.shots.push(shot);
    }
}

















#[cfg(test)]
mod test {
    use shot::*;
    use session::*;
    use discipline::*;
    use helper;

    #[test]
    fn test_add_shot() {
        let target = helper::dsc_demo::lg_target();
        let discipline = helper::dsc_demo::lg_discipline();
        let shot = Shot::from_cartesian_coordinates (0, 0, &target);

        let mut session = Session::new();
        assert_eq!(1, session.parts.len());
        assert_eq!(0, session.active_session);

        session.add_shot(shot, &discipline);
    }

}
