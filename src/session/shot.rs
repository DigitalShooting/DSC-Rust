use rand::{self, Rng};

use helper::round_to_one::RoundToOne;
use discipline::*;



#[derive(Debug)]
pub struct ShotRaw {
    pub x: i32,
    pub y: i32,
}

impl ShotRaw {

    /// Generate a random shot. We need a target to calculate the ring
    /// discipline:     Discipline to use to calculate ring
    pub fn random() -> ShotRaw {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-8000, 8000);
        let y = rng.gen_range(-8000, 8000);
        return ShotRaw { x, y };
    }

}




/// Represents a single shot, with all its metadata.
#[derive(Serialize, Deserialize, Debug)]
pub struct Shot {
    pub teiler: f64,
    pub angle: f64,
    pub x: i32,
    pub y: i32,

    /// the actual ring, always with tenths
    pub ring: f64,
    /// the counted part of the ring, e.g
    /// - no tenth 10.3 => ring_count = 10
    /// - tenth 10.3 => ring_count = 10.3
    pub ring_count: f64,

    // TODO
    // date: ???,
}


struct Counter {
    value: i32,
    pub text: String,
    count_mode: PartCountMode,
}

// impl Counter {
//     pub fn new(value: i32, count_mode: PartCountMode) {
//         let text = "todo".to_string();
//         Counter { value, text, count_mode }
//     }
//
//     pub fn add(&mut self, rhs: Counter) {
//         if rhs.count_mode != self.count_mode {
//             // TODO raise error
//         }
//         value += value
//     }
//
//     pub fn as_float() -> f64 {
//         0_f64 // TODO
//     }
//
//     fn update_text(&mut self) {
//
//     }
// }


impl Shot {

    pub fn from_raw(raw: ShotRaw, target: &Target, discipline_part: &DisciplinePart) -> Shot {
        Shot::from_cartesian_coordinates(raw.x, raw.y, target, discipline_part)
    }

    /// New shot from x and y coordinates in 1/1000 mm
    /// x:                  x coordinate in 1/1000 mm
    /// y:                  y coordinate in 1/1000 mm
    /// target:             Target to use to calculate ring
    /// discipline_part     Active discipline_part to get PartCountMode
    /// TODO Custom type maybe for tenth/ no tenth handling?
    fn from_cartesian_coordinates(x: i32, y: i32, target: &Target, discipline_part: &DisciplinePart) -> Shot {
        let x_f64 = x as f64;
        let y_f64 = y as f64;

        // Calculate teiler from cartesian coordinates (pythagoras),
        // and round it to one decimal digit
        let teiler = ((x_f64.powi(2) + y_f64.powi(2)).sqrt() / 10_f64).round_to_one();

        // Get the angle, and round it to one decimal digit.
        // Then move the range from [-180, 180] to [0, 360].
        let mut angle = y_f64.atan2(x_f64).to_degrees().round_to_one() % 360_f64;
        if angle < 0_f64 {
            angle += 360_f64;
        }

        let ring = Shot::get_ring_from_teiler(teiler, target);
        let ring_count: f64 = match discipline_part.count_mode {
            PartCountMode::Integer => ring.floor(),
            PartCountMode::Tenth => ring, //(ring*10_f64).floor(),
        };

        return Shot {teiler, angle, x, y, ring, ring_count};
    }

    /// Helper to calculate the actual ring for a given teiler
    /// teiler:     Teiler of the shot (1/100mm)
    /// target:     Target to use
    fn get_ring_from_teiler(teiler: f64, target: &Target) -> f64 {
        let ring_big = target.rings.first().unwrap();
        let ring_small = target.rings.last().unwrap();
        let k = target.bullet_diameter * 100_f64 / 2_f64;

        let ring: f64;
        // If its 0, its a 10.9, not an 11
        if teiler == 0_f64 {
            ring = 10.9_f64;
        }
        // If it is smaller than the smallest ring, its a 0
        else if teiler > ring_small.width * 100_f64 / 2_f64 + k {
            ring = 0_f64;
        }
        else {
            let m = ((ring_big.value - ring_small.value) as f64) / (ring_big.width*100_f64/2_f64 - ring_small.width*100_f64/2_f64);
            let t =  (ring_big.value as f64) - m * (ring_big.width*100_f64/2_f64 + k);
            ring = (m * teiler + t).cut_at_one();
        }
        return ring;
    }
}




pub trait AddShotWithDiscipline {
    fn add_shot(&mut self, Shot, &Discipline);
}

pub trait AddShot {
    fn add_shot(&mut self, Shot);
}





#[cfg(test)]
mod test {
    use session::shot::*;
    use discipline::*;
    use helper;

    #[test]
    fn test_zero_teiler() {
        let target = helper::dsc_demo::lg_target();
        let shot = Shot::from_cartesian_coordinates (0, 0, &target);
        assert_eq!(0_f64, shot.teiler);
        assert_eq!(0_f64, shot.angle);
        assert_eq!(0_i32, shot.x);
        assert_eq!(0_i32, shot.y);
        assert_eq!(10.9_f64, shot.ring);
    }

    #[test]
    fn test_last_ten_1() {
        let target = helper::dsc_demo::lg_target();
        let shot = Shot::from_cartesian_coordinates (2500, 0, &target);
        assert_eq!(250.0_f64, shot.teiler);
        assert_eq!(0_f64, shot.angle);
        assert_eq!(2500_i32, shot.x);
        assert_eq!(0_i32, shot.y);
        assert_eq!(10.0_f64, shot.ring);
    }

    #[test]
    fn test_last_ten_2() {
        let target = helper::dsc_demo::lg_target();
        let shot = Shot::from_cartesian_coordinates (0, 2500, &target);
        assert_eq!(250.0_f64, shot.teiler);
        assert_eq!(90_f64, shot.angle);
        assert_eq!(0_i32, shot.x);
        assert_eq!(2500_i32, shot.y);
        assert_eq!(10.0_f64, shot.ring);
    }

    #[test]
    fn test_first_nine() {
        let target = helper::dsc_demo::lg_target();
        let shot = Shot::from_cartesian_coordinates (2501, 0, &target);
        assert_eq!(250.1_f64, shot.teiler);
        assert_eq!(0_f64, shot.angle);
        assert_eq!(2501_i32, shot.x);
        assert_eq!(0_i32, shot.y);
        assert_eq!(9.9_f64, shot.ring);
    }

    #[test]
    fn test_zero() {
        let target = helper::dsc_demo::lg_target();
        let shot = Shot::from_cartesian_coordinates (-1000000, 0, &target);
        assert_eq!(100000_f64, shot.teiler);
        assert_eq!(180_f64, shot.angle);
        assert_eq!(-1000000, shot.x);
        assert_eq!(0_i32, shot.y);
        assert_eq!(0_f64, shot.ring);
    }
}
