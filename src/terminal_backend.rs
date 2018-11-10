pub trait TerminalBackend: Send {
    fn enable_raw_mode(&mut self);
    fn disable_raw_mode(&mut self);
    fn enable_alternate_screen(&mut self);
    fn disable_alternate_screen(&mut self);
}