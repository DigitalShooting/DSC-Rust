use super::{Counter, CountMode, Shot, AddShot};
use discipline::*;
use helper::round_to_one::RoundToOne;



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Series {
    pub shots: Vec<Shot>,
    sum: Counter,
    number_of_shots: i32,
}

impl Series {
    /// New, empty series
    pub fn new() -> Series {
        Series {
            shots: vec![],
            sum: Counter::empty(),
            number_of_shots: 0,
        }
    }
}

impl Series {
    /// Check if the series is full, for given discipline_part
    /// discipline_part:    part tho use to check if the series is full
    /// return:             true/ false, if full or not
    pub fn is_full<'a, 'b>(&'a self, discipline_part: &'b DisciplinePart) -> bool {
        return self.shots.len() as i32 >= discipline_part.series_length
    }
}



impl AddShot for Series {
    fn add_shot(&mut self, shot: Shot, _discipline: &Discipline, count_mode: &CountMode) {
        // add ring count so series sum
        self.sum.add(shot.ring_count, &count_mode);
        self.number_of_shots += 1;

        // add shot to series
        self.shots.push(shot);

        // print!("{}", Svg(discipline.target.draw(), 500, 500));

        // print!("{}", Svg(self.draw(), 500, 500));
    }
}








// use simplesvg::*;
//
// pub trait Draw {
//     fn draw(&self) -> Vec<Fig>;
// }
//
//
// impl Draw for Series {
//     fn draw(&self) -> Vec<Fig> {
//         let fig = Fig::Rect(10., 10., 200., 100.)
//                 .styled(Attr::default().fill(Color(0xff, 0, 0)));
//         let text = Fig::Text(0., 20., "<XML & Stuff>".to_string());
//         let c = Fig::Circle(20., 20., 100.);
//         vec![fig, text, c]
//     }
// }
