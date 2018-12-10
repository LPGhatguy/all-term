#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset,

    #[doc(hidden)]
    __Nonexhaustive,
}

// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#text-formatting
impl Color {
    pub(crate) fn to_ansi_foreground(&self) -> u8 {
        match self {
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
            Color::Reset => 39,
            Color::__Nonexhaustive => unreachable!(),
        }
    }

    pub(crate) fn to_ansi_background(&self) -> u8 {
        match self {
            Color::Black => 40,
            Color::Red => 41,
            Color::Green => 42,
            Color::Yellow => 43,
            Color::Blue => 44,
            Color::Magenta => 45,
            Color::Cyan => 46,
            Color::White => 47,
            Color::Reset => 49,
            Color::__Nonexhaustive => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub(crate) foreground: Option<Color>,
    pub(crate) background: Option<Color>,
}

impl Style {
    pub fn new() -> Style {
        Style {
            foreground: None,
            background: None,
        }
    }

    pub fn fg(self, color: Color) -> Style {
        Style {
            foreground: Some(color),
            ..self
        }
    }

    pub fn bg(self, color: Color) -> Style {
        Style {
            background: Some(color),
            ..self
        }
    }
}