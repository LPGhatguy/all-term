extern crate lazy_static;

#[cfg(windows)] extern crate winapi;

mod backend;
mod terminal;
mod terminal_backend;

pub use terminal::{
    terminal,
    Terminal,
};