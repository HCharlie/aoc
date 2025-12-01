mod types;
mod aoc_client;
mod runner;

// Explicitly re-export only the public items needed from each module
pub use types::Level;
pub use aoc_client::{get_input_content, submit_check_answer};
pub use runner::{run_real};
