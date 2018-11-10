extern crate lazy_static;

#[cfg(windows)]
extern crate winapi;

#[cfg(unix)]
extern crate libc;

mod backend;
mod terminal;
mod terminal_backend;
mod os;

pub use terminal::{
    terminal,
    Terminal,
};