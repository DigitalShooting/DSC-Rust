use rand::{self, Rng};

use helper::round_to_one::RoundToOne;
use discipline::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct Shot {
    pub teiler: f64,
    pub angle: f64,
    pub x: i32,
    pub y: i32,
    pub ring: f64,
    pub ring_count: f64,
    // date: ???,
}

impl Shot {
    pub fn random(target: &Target) -> Shot {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-3000, 3000);
        let y = rng.gen_range(-3000, 3000);
        return Shot::from_cartesian_coordinates(x, y, target);
    }

    // fn from_cartesian_coordinates(x: i32, y: i32, target: Target, part: Part) -> Shot {
    pub fn from_cartesian_coordinates(x: i32, y: i32, target: &Target) -> Shot {
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
        // this.ring.display = parseFloat(ring).toFixed(1);
        // this.ring.int = parseFloat(ring).toFixedDown(0);

        // TODO use ring_count
        // if (part.zehntel === true) {
        //   this.ring.value = parseFloat(ring).toFixedDown(1);
        // }
        // else {
        //   this.ring.value = parseFloat(ring).toFixedDown(0);
        // }
        // let ring_count = (ring*10_f64).round() / 10_f64;
        let ring_count = ring.round();

        return Shot {teiler, angle, x, y, ring, ring_count};
    }

    fn get_ring_from_teiler(teiler: f64, target: &Target) -> f64 {
        let ring_big = target.rings.first().unwrap();
        let ring_small = target.rings.last().unwrap();
        let k = target.bullet_diameter * 100_f64 / 2_f64;

        let ring: f64;
        if teiler == 0_f64 {
            ring = 10.9_f64;
        }
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















#[cfg(test)]
mod test {
    use shot::*;
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
