mod types;
mod aoc_client;
mod runner;

pub use types::*;
pub use aoc_client::*;
pub use runner::*;

// Prelude for convenience
pub mod prelude {
    pub use crate::{Level, run_real, run_example};
    pub use std::error::Error;
}

