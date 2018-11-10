use winapi::um::{
    processenv::GetStdHandle,
    errhandlingapi::GetLastError,
    consoleapi::{GetConsoleMode, SetConsoleMode},
};

const STD_OUT_HANDLE: u32 = -11i32 as u32;
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

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
        let std_out_handle = GetStdHandle(STD_OUT_HANDLE);
        let error_code = GetLastError();
        if error_code != 0 {
            return false;
        }

        let mut console_mode: u32 = 0;
        GetConsoleMode(std_out_handle, &mut console_mode);
        let error_code = GetLastError();
        if error_code != 0 {
            return false;
        }

        // Is ANSI mode already enabled?
        if console_mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING == 0 {
            SetConsoleMode(std_out_handle, console_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
            let error_code = GetLastError();
            if error_code != 0 {
                return false;
            }
        }
    }

    true
}