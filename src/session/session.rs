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
    pub discipline: Discipline,
    pub info: Info,
    active_session: ActiveSession,
    sum: f64,
}

impl Session {
    pub fn new(line: Line, discipline: Discipline) -> Session {
        Session {
            parts: vec![
                Part::new(),
            ],
            discipline: discipline,
            info: Info::new(line),
            active_session: 0,
            sum: 0_f64,
        }
    }

    pub fn get_active_part(&mut self) -> Option<&Part> {
        return self.parts.get(self.active_session);
    }

    pub fn get_active_discipline_part(&mut self) -> Option<DisciplinePart> {
        let active_part_type = self.get_active_part()?.part_type.clone();
        self.discipline.get_part_from_type(active_part_type)
    }
}






impl AddShot for Session {
    fn add_shot(&mut self, shot: Shot) {
        // add ring count to session sum
        // TODO round
        self.sum += shot.ring_count;

        // add shot to the active session
        let active_session = &mut self.parts[self.active_session];
        active_session.add_shot(shot, &self.discipline);
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
