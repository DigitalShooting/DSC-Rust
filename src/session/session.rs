use session::counter::Counter;
use session::shot::*;
use session::part::*;
use session::info::{Line, Info};
use discipline::*;


/// The index of the currently active session
pub type ActiveSession = usize;


/// Main data struct, contains:
/// - all parts (probe, match, new targets)/ active part index
/// - the discipline used
/// - user info (user, club, team)
/// - statistics
#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub parts: Vec<Part>,
    active_part: ActiveSession,
    pub discipline: Discipline,
    pub info: Info,
    sum: Counter,
    number_of_shots: i32,
}

impl Session {
    /// New session with given discipline
    /// line:           Line config to use
    /// discipline:     Discipline to use
    /// return:         Empty session
    pub fn new(line: Line, discipline: Discipline) -> Session {
        Session {
            parts: vec![
                Part::new(),
            ],
            active_part: 0,
            discipline: discipline,
            info: Info::new(line),
            sum: Counter::empty(),
            number_of_shots: 0,
        }
    }

    /// Return the active part
    /// return:     None if the active_part is set wrong
    pub fn get_active_part(&mut self) -> Option<&Part> {
        self.parts.get(self.active_part)
    }

    /// Get the active discipline part
    pub fn get_active_discipline_part(&mut self) -> Option<DisciplinePart> {
        let active_part_type = self.get_active_part()?.part_type.clone();
        self.discipline.get_part_from_type(active_part_type)
    }
}






impl AddShotRaw for Session {
    fn add_shot_raw(&mut self, shot_raw: ShotRaw) {
        match self.get_active_discipline_part() {
            Some(discipline_part) => {
                let count_mode = discipline_part.count_mode;
                let shot = Shot::from_raw(shot_raw, &self.discipline.target, &count_mode);

                self.sum.add(shot.ring_count, &count_mode);
                self.number_of_shots += 1;

                // add shot to the active session
                let active_part = &mut self.parts[self.active_part];
                active_part.add_shot(shot, &self.discipline, &discipline_part.count_mode);
            },
            None => println!("no discipline_part"),
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
        let shot = Shot::from_cartesian_coordinates (0, 0, &target, &CountMode::Integer);

        let mut session = Session::new(Line::demo(), discipline);
        assert_eq!(1, session.parts.len());
        assert_eq!(0, session.active_part);
    }

}
