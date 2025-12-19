// Data models
// This module will contain data structures for space weather data

pub mod space_weather;
pub mod validation;

#[cfg(test)]
mod tests;

pub use space_weather::*;
pub use validation::*;

