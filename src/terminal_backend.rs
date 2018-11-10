pub trait TerminalBackend: Send {
    fn enable_raw_mode(&mut self);
    fn disable_raw_mode(&mut self);
}