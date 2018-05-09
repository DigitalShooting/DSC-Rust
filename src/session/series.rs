use session::shot::*;
use discipline::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct Series {
    pub shots: Vec<Shot>,
    sum: f64,
}

impl Series {
    pub fn new() -> Series {
        Series {
            shots: vec![],
            sum: 0_f64,
        }
    }
}

impl Series {
    pub fn is_full<'a, 'b>(&'a self, discipline_part: &'b DisciplinePart) -> bool {
        return self.shots.len() as i32 >= discipline_part.series_length
    }
}



impl AddShotWithDiscipline for Series {
    fn add_shot(&mut self, shot: Shot, _discipline: &Discipline) {
        // add ring count so series sum
        // TODO round
        self.sum += shot.ring_count;

        // add shot to series
        self.shots.push(shot);
    }
}
