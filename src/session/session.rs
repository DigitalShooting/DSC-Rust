use session::counter::Counter;
use session::shot::*;
use session::part::*;
use session::info::{Line, Info};
use discipline::*;


/// The index of the currently active session
pub type ActivePart = usize;


/// Main data struct, contains:
/// - all parts (probe, match, new targets)/ active part index
/// - the discipline used
/// - user info (user, club, team)
/// - statistics
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub parts: Vec<Part>,
    active_part: ActivePart,
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
        let part_type = discipline.parts[0].id.clone();
        Session {
            parts: vec![
                Part::new(part_type),
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
    pub fn get_active_part(&self) -> &Part {
        self.parts.get(self.active_part).unwrap()
    }

    /// Get the active discipline part
    pub fn get_active_discipline_part(&self) -> Option<DisciplinePart> {
        let active_part_type = self.get_active_part().part_type.clone();
        self.discipline.get_part_from_type(active_part_type)
    }

    // Check if
    fn can_exit_part(&self, force: bool) -> bool {
        let current_part_type = self.get_active_part().part_type.clone();
        match self.discipline.get_part_from_type(current_part_type) {
            Some(discipline_part) => {
                if force == false {
                    match discipline_part.exit_type {
                        PartExitType::Always => true,
                        PartExitType::BeforeFirst => self.number_of_shots == 0,
                        PartExitType::None => false,
                    }
                }
                else {
                    true
                }
            }
            None => true
        }
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








pub trait Update {

    // Set new target
    // If allowed in the current discipline part, we add a new part to the session of the same
    // type as the current one.
    //
    // force:   Flag if we should check if the discipline allows a part change ot not
    fn new_target(&mut self, force: bool);



    // Update the user of the current session
    //
    // user:    new user
    // fn set_user(&mut self, user: User);

    // Update the team of the current session
    //
    // team:    new team
    // fn set_team(&mut self, team: Team);

    // Update the club of the current session
    //
    // club:    new club
    // fn set_club(&mut self, club: Club);



    // Change to a different part, which has to be in the current discipline parts.
    //
    // part:    PartType string of the part we want to change to
    // force:   Flag if we should check if the discipline allows a part change ot not
    fn set_part(&mut self, part_type: PartType, force: bool);

    // Change the active part, index has to be in the range of the sessions parts
    //
    // index:   Index of the part to change to
    // force:   Flag if we should check if the discipline allows a part change ot not
    fn set_active_part(&mut self, index: ActivePart, force: bool);
}

impl Update for Session {

    fn new_target(&mut self, force: bool) {
        let part_type = self.get_active_part().part_type.clone();
        self.set_part(part_type, force);
    }

    fn set_part(&mut self, part_type: PartType, force: bool) {
        if let Some(discipline_part) = self.discipline.get_part_from_type(part_type.clone()) {
            if self.can_exit_part(force) {
                self.parts.push(Part::new(part_type));
                self.active_part = self.parts.len();
            }
        }
    }

    fn set_active_part(&mut self, index: ActivePart, force: bool) {
        if index < self.parts.len() && self.can_exit_part(force) {
            self.active_part = index;
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
