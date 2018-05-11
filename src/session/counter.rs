


/// Wrapper struct to support multiple ways to calculate the ring value.
/// Currenty we have 2 count modes, Integer and Tenth.
#[derive(Serialize, Deserialize, Debug)]
pub struct Counter {
    value: f64,
    text: String,
}

impl Counter {
    /// Create new counter with 0 as value.
    pub fn empty() -> Counter {
        Counter::new(0_f64, &CountMode::Integer)
    }

    /// Create new counter with given value and count mode.
    /// value:          value to use
    /// count_mode:     count mode to apply on the value, will only be used once
    /// return:         counter
    pub fn new(value: f64, count_mode: &CountMode) -> Counter {
        Counter {
            value: count_mode.round(value),
            text: count_mode.to_string(value),
        }
    }

    /// Add a value to the counter.
    /// other:          value to add
    /// count_mode:     will be used befor we add the value to self.value,
    ///                 and for the updated string
    pub fn add(&mut self, other: f64, count_mode: &CountMode) {
        self.value += count_mode.round(other);
        self.text = count_mode.to_string(self.value);
    }
}








#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CountMode {
    Integer,
    Tenth,
}
impl CountMode {
    /// Creates a string according to the CountMode
    /// value:  value to round
    /// return: rounded string
    pub fn to_string(self, value: f64) -> String {
        match self {
            CountMode::Integer => format!("{:.0}", value.floor()),
            CountMode::Tenth => format!("{:.1}", value),
        }
    }

    /// Round given value according to the CountMode
    /// value:  value to round
    /// return: rounded value
    pub fn round(self, value: f64) -> f64 {
        match self {
            CountMode::Integer => value.floor(),
            CountMode::Tenth => (value * 10.0).round() / 10.0,
        }
    }
}






#[cfg(test)]
mod test {
    use session::counter::{Counter, CountMode};

    #[test]
    fn test_new_counter() {
        let counter_int = Counter::new(9.3, &CountMode::Integer);
        assert_eq!("9".to_string(), counter_int.text);
        assert_eq!(9.0, counter_int.value);

        let counter_tenth = Counter::new(9.3, &CountMode::Tenth);
        assert_eq!("9.3".to_string(), counter_tenth.text);
        assert_eq!(9.3, counter_tenth.value);
    }

    #[test]
    fn test_add_int() {
        let mut counter = Counter::new(9.3, &CountMode::Integer);
        assert_eq!("9".to_string(), counter.text);
        assert_eq!(9.0, counter.value);

        counter.add(1.6, &CountMode::Integer);
        assert_eq!("10".to_string(), counter.text);
        assert_eq!(10.0, counter.value);
    }

    #[test]
    fn test_add_tenth() {
        let mut counter = Counter::new(9.3, &CountMode::Tenth);
        assert_eq!("9.3".to_string(), counter.text);
        assert_eq!(9.3, counter.value);

        counter.add(1.7, &CountMode::Tenth);
        assert_eq!("11.0".to_string(), counter.text);
        assert_eq!(11.0, counter.value);
    }

}
