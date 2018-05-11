


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Interface {
    /// Häring ESA interface
    ESA {
        /// Serial port path
        port: String,
        /// time in 1/10s to move the paper on a part change
        on_part_band: u8,
        /// time in 1/10s to move the paper after each shot
        on_shot_band: u8,
    },

    /// Demo interface
    Demo {
        /// Interval between each simulated shot
        interval: u64,
        /// If not None, interface will stop generating shots after this number
        max_shots: Option<u32>,
    },
}
