#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod errors;
mod params;
mod pipe;
mod steps;
mod tasks;

pub use self::{
    errors::TaskError,
    params::*,
    pipe::Conduit,
    steps::{Step, TaskStep},
    tasks::Task,
};
