use crate::{
    terminal_backend::TerminalBackend,
    os::windows::{enable_raw_mode, disable_raw_mode},
};

pub struct WindowsTerminal;

impl TerminalBackend for WindowsTerminal {
    fn enable_raw_mode(&mut self) {
        enable_raw_mode();
    }

    fn disable_raw_mode(&mut self) {
        disable_raw_mode();
    }
}