


pub trait RoundToOne {
    fn round_to_one(self) -> f64;
    fn cut_at_one(self) -> f64;
}

impl RoundToOne for f64 {
    fn round_to_one(self) -> f64 {
        return (self * 10_f64).round() / 10_f64;
    }
    fn cut_at_one(self) -> f64 {
        return (self * 10_f64).floor() / 10_f64;
    }
}
