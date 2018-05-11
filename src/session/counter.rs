use discipline::PartCountMode;



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
            value: count_mode.round(value),
            text: count_mode.to_string(value),
        }
    }

    pub fn add(&mut self, other: f64, count_mode: &PartCountMode) {
        self.value += count_mode.round(other);
        self.text = count_mode.to_string(self.value);
    }
}








#[cfg(test)]
mod test {
    use session::counter::Counter;
    use discipline::PartCountMode;

    #[test]
    fn test_new_counter() {
        let counter_int = Counter::new(9.3, &PartCountMode::Integer);
        assert_eq!("9".to_string(), counter_int.text);
        assert_eq!(9.0, counter_int.value);

        let counter_tenth = Counter::new(9.3, &PartCountMode::Tenth);
        assert_eq!("9.3".to_string(), counter_tenth.text);
        assert_eq!(9.3, counter_tenth.value);
    }

    #[test]
    fn test_add_int() {
        let mut counter = Counter::new(9.3, &PartCountMode::Integer);
        assert_eq!("9".to_string(), counter.text);
        assert_eq!(9.0, counter.value);

        counter.add(1.6, &PartCountMode::Integer);
        assert_eq!("10".to_string(), counter.text);
        assert_eq!(10.0, counter.value);
    }

    #[test]
    fn test_add_tenth() {
        let mut counter = Counter::new(9.3, &PartCountMode::Tenth);
        assert_eq!("9.3".to_string(), counter.text);
        assert_eq!(9.3, counter.value);

        counter.add(1.7, &PartCountMode::Tenth);
        assert_eq!("11.0".to_string(), counter.text);
        assert_eq!(11.0, counter.value);
    }

}
