use std::sync::{Arc, Mutex, Weak};

use lazy_static::lazy_static;

use crate::{
    style::Style,
    backend::ansi::AnsiTerminal,
    terminal_backend::TerminalBackend,
};

#[cfg(windows)]
fn choose_backend() -> Box<TerminalBackend> {
    use crate::backend::windows::{
        enable_ansi_mode,
        WindowsTerminal,
    };

    if enable_ansi_mode() {
        Box::new(AnsiTerminal)
    } else {
        Box::new(WindowsTerminal)
    }
}

#[cfg(not(windows))]
fn choose_backend() -> Box<TerminalBackend> {
    Box::new(AnsiTerminal)
}

/// Provide's access to the application's terminal.
pub fn terminal() -> Arc<Mutex<Terminal>> {
    lazy_static! {
        static ref TERMINAL: Mutex<Weak<Mutex<Terminal>>> = Mutex::new(Weak::new());
    }

    let mut maybe_terminal = TERMINAL.lock().unwrap();

    if let Some(terminal) = maybe_terminal.upgrade() {
        terminal
    } else {
        let backend = choose_backend();
        let terminal = Arc::new(Mutex::new(Terminal::with_backend(backend)));

        *maybe_terminal = Arc::downgrade(&terminal);

        terminal
    }
}

pub struct Terminal {
    backend: Box<TerminalBackend>,
    raw_mode_enabled: bool,
    alternate_screen_enabled: bool,
}

impl Terminal {
    fn with_backend(backend: Box<TerminalBackend>) -> Terminal {
        Terminal {
            backend,
            raw_mode_enabled: false,
            alternate_screen_enabled: false,
        }
    }

    pub fn enable_raw_mode(&mut self) {
        if !self.raw_mode_enabled {
            self.backend.enable_raw_mode();
            self.raw_mode_enabled = true;
        }
    }

    pub fn disable_raw_mode(&mut self) {
        if self.raw_mode_enabled {
            self.backend.disable_raw_mode();
            self.raw_mode_enabled = false;
        }
    }

    pub fn enable_alternate_screen(&mut self) {
        if !self.alternate_screen_enabled {
            self.backend.enable_alternate_screen();
            self.alternate_screen_enabled = true;
        }
    }

    pub fn disable_alternate_screen(&mut self) {
        if self.alternate_screen_enabled {
            self.backend.disable_alternate_screen();
            self.alternate_screen_enabled = false;
        }
    }

    pub fn clear_screen(&mut self) {
        self.backend.clear_screen();
    }

    pub fn show_cursor(&mut self) {
        self.backend.show_cursor();
    }

    pub fn hide_cursor(&mut self) {
        self.backend.hide_cursor();
    }

    pub fn move_cursor(&mut self, x: usize, y: usize) {
        self.backend.move_cursor(x, y);
    }

    pub fn print(&mut self, text: &str, style: Style) {
        self.backend.print(text, style);
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.disable_raw_mode();
        self.disable_alternate_screen();
    }
}