use winapi::um::{
    wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO},
};

use super::{get_stdout_handle, die_if_win32_error};

pub fn get_terminal_size() -> (usize, usize) {
    unsafe {
        let std_out_handle = get_stdout_handle();

        let mut info = CONSOLE_SCREEN_BUFFER_INFO::default();
        if GetConsoleScreenBufferInfo(std_out_handle, &mut info) == 0 {
            die_if_win32_error();
        }

        let width = info.srWindow.Right - info.srWindow.Left + 1;
        let height = info.srWindow.Bottom - info.srWindow.Top + 1;

        (width as usize, height as usize)
    }
}