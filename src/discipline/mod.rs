pub mod discipline;
pub mod part;
pub mod interface;
pub mod target;
pub mod time;

pub use discipline::interface::Interface;
pub use discipline::discipline::Discipline;
pub use discipline::part::{DisciplinePart, PartAverage, PartExitType};
pub use discipline::target::{Target, Zoom, Ring, WebColor};
pub use discipline::time::Time;
