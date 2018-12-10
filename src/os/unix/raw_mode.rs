/// This module is completely untested.

use std::sync::Mutex;

use lazy_static::lazy_static;

use libc::{
    tcgetattr,
    tcsetattr,
    termios,
    STDIN_FILENO,
    TCSAFLUSH,
    ECHO,
    ICANON,
};

struct RawMode {
    original_termios: Option<termios>,
}

impl RawMode {
    fn new() -> RawMode {
        RawMode {
            original_termios: None,
        }
    }
}

lazy_static! {
    static ref RAW_MODE: Mutex<RawMode> = Mutex::new(RawMode::new());
}

pub fn enable_raw_mode() -> Result<(), String> {
    let mut raw_mode = RAW_MODE.lock().unwrap();

    let mut original_termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };

    unsafe {
        tcgetattr(STDIN_FILENO, &mut original_termios);
    }

    raw_mode.original_termios = Some(original_termios);

    let mut with_raw = original_termios;
    with_raw.c_lflag &= !(ECHO | ICANON);

    unsafe {
        tcsetattr(STDIN_FILENO, TCSAFLUSH, &with_raw);
    }

    Ok(())
}

pub fn disable_raw_mode() -> Result<(), String> {
    let mut raw_mode = RAW_MODE.lock().unwrap();

    if let Some(termios) = raw_mode.original_termios {
        unsafe {
            tcsetattr(STDIN_FILENO, TCSAFLUSH, &termios);
        }
    }

    raw_mode.original_termios = None;

    Ok(())
}