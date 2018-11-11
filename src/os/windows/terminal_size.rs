use winapi::um::{
    winbase::STD_OUTPUT_HANDLE,
    wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO},
    processenv::GetStdHandle,
    errhandlingapi::GetLastError,
};

pub fn get_terminal_size() -> (usize, usize) {
    unsafe {
        let std_out_handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let error_code = GetLastError();
        if error_code != 0 {
            panic!("Could not GetStdHandle");
        }

        let mut info = CONSOLE_SCREEN_BUFFER_INFO::default();
        if GetConsoleScreenBufferInfo(std_out_handle, &mut info) == 0 {
            panic!("Could not GetConsoleScreenBufferInfo");
        }

        let width = info.srWindow.Right - info.srWindow.Left + 1;
        let height = info.srWindow.Bottom - info.srWindow.Top + 1;

        (width as usize, height as usize)
    }
}