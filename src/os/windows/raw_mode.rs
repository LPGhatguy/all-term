use winapi::{
    um::{
        wincon::{ENABLE_LINE_INPUT, ENABLE_ECHO_INPUT, ENABLE_PROCESSED_INPUT},
        consoleapi::{GetConsoleMode, SetConsoleMode},
    },
    shared::minwindef::DWORD,
};

use super::{get_stdin_handle, check_win32_error};

const RAW_FLAGS: DWORD = !(ENABLE_ECHO_INPUT | ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT);

pub fn enable_raw_mode() -> Result<(), String> {
    unsafe {
        let stdin_handle = get_stdin_handle()?;

        let mut console_mode: u32 = 0;
        if GetConsoleMode(stdin_handle, &mut console_mode) == 0 {
            check_win32_error()?;
        }

        let new_mode = console_mode & RAW_FLAGS;

        if SetConsoleMode(stdin_handle, new_mode) == 0 {
            check_win32_error()?;
        }

        Ok(())
    }
}

pub fn disable_raw_mode() -> Result<(), String> {
    unsafe {
        let stdin_handle = get_stdin_handle()?;

        let mut console_mode: u32 = 0;
        if GetConsoleMode(stdin_handle, &mut console_mode) == 0 {
            check_win32_error()?;
        }

        let new_mode = console_mode | !RAW_FLAGS;

        if SetConsoleMode(stdin_handle, new_mode) == 0 {
            check_win32_error()?;
        }

        Ok(())
    }
}