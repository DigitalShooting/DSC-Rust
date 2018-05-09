


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Time {
    InstantStart { duration: i32 },
    FirstShot { duration: i32 },
    None,
}
