use discipline::PartCountMode;

// use std::ops::{Add, Sub};



#[derive(Serialize, Deserialize, Debug)]
pub struct Counter {
    value: f64,
    text: String,
}

impl Counter {
    pub fn empty() -> Counter {
        Counter::new(0_f64, &PartCountMode::Integer)
    }

    pub fn new(value: f64, count_mode: &PartCountMode) -> Counter {
        Counter {
            value,
            text: count_mode.to_string(value),
        }
    }

    pub fn add(&mut self, other: f64, count_mode: &PartCountMode) {
        self.value += other;
        self.text = count_mode.to_string(self.value);
    }
}
//
// impl Add for Counter {
//     type Output = Counter;
//
//     fn add(self, other: Counter) -> Counter {
//         Counter::new(self.value + other.value, self.count_mode)
//     }
// }
