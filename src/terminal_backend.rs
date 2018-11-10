use crate::{
    style::Style,
};

pub trait TerminalBackend: Send {
    fn enable_raw_mode(&mut self);
    fn disable_raw_mode(&mut self);
    fn enable_alternate_screen(&mut self);
    fn disable_alternate_screen(&mut self);
    fn clear_screen(&mut self);
    fn hide_cursor(&mut self);
    fn show_cursor(&mut self);
    fn move_cursor(&mut self, x: usize, y: usize);
    fn print(&mut self, text: &str, style: Style);
}