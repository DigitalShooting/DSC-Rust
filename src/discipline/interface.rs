




#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Interface {
    /// Häring ESA interface
    ESA {
        port: String,
        on_part_band: u8,
        on_shot_band: u8,
    },

    /// Demo interface
    Demo {
        interval: u64,
        max_shots: Option<u32>,
    },
}
