use crate::{
    style::Style,
    key::Key,
};

pub trait TerminalBackend: Send {
    fn enable_raw_mode(&mut self) -> Result<(), String>;
    fn disable_raw_mode(&mut self) -> Result<(), String>;
    fn enable_alternate_screen(&mut self);
    fn disable_alternate_screen(&mut self);
    fn clear_screen(&mut self);
    fn hide_cursor(&mut self);
    fn show_cursor(&mut self);
    fn move_cursor(&mut self, x: usize, y: usize);
    fn print(&mut self, text: &str, style: Style);
    fn get_size(&self) -> Result<(usize, usize), String>;
    fn read_key(&mut self) -> Key;
}