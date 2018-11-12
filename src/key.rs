#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Escape,
    Backspace,
    Up,
    Down,
    Left,
    Right,
    F(u8),
    Char(char),
}