#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Escape,
    Backspace,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    F(u8),
    Char(char),

    #[doc(hidden)]
    __Nonexhaustive,
}