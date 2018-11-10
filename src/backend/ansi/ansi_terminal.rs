use crate::{
    terminal_backend::TerminalBackend,
    os::current::{enable_raw_mode, disable_raw_mode},
};

pub struct AnsiTerminal;

impl TerminalBackend for AnsiTerminal {
    fn enable_raw_mode(&mut self) {
        enable_raw_mode();
    }

    fn disable_raw_mode(&mut self) {
        disable_raw_mode();
    }
}