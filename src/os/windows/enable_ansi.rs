use winapi::{
    um::{
        consoleapi::{GetConsoleMode, SetConsoleMode},
        wincon::{ENABLE_VIRTUAL_TERMINAL_PROCESSING, ENABLE_VIRTUAL_TERMINAL_INPUT, DISABLE_NEWLINE_AUTO_RETURN},
    },
    shared::minwindef::DWORD,
};

use crate::os::windows::{get_stdout_handle, get_stdin_handle, die_if_win32_error};

const ANSI_INPUT_FLAGS: DWORD = ENABLE_VIRTUAL_TERMINAL_INPUT;
const ANSI_OUTPUT_FLAGS: DWORD = ENABLE_VIRTUAL_TERMINAL_PROCESSING;

/// Enables ANSI terminal mode on Windows if supported.
///
/// Based on ansi_term's implementation of this operation:
/// https://github.com/ogham/rust-ansi-term/blob/bb2cdf9fcab0362c8620f202ac5db21ac39338b1/src/windows.rs
///
/// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences
///
/// Return value indicates success.
pub fn enable_ansi_mode() -> bool {
    unsafe {
        {
            let stdout_handle = get_stdout_handle();

            let mut console_mode = 0;
            if GetConsoleMode(stdout_handle, &mut console_mode) == 0 {
                die_if_win32_error();
            }

            let new_mode = console_mode | ANSI_OUTPUT_FLAGS;

            if SetConsoleMode(stdout_handle, new_mode) == 0 {
                println!("Failed to enable ANSI mode");
                return false;
            }
        }

        {
            let stdin_handle = get_stdin_handle();

            let mut console_mode = 0;
            if GetConsoleMode(stdin_handle, &mut console_mode) == 0 {
                die_if_win32_error();
            }

            let new_mode = console_mode | ANSI_INPUT_FLAGS;

            if SetConsoleMode(stdin_handle, new_mode) == 0 {
                println!("Failed to enable ANSI mode");
                return false;
            }
        }
    }

    true
}