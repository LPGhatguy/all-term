use winapi::um::{
    winbase::STD_OUTPUT_HANDLE,
    wincon::{ENABLE_WRAP_AT_EOL_OUTPUT, ENABLE_LINE_INPUT},
    processenv::GetStdHandle,
    errhandlingapi::GetLastError,
    consoleapi::{GetConsoleMode, SetConsoleMode},
};

pub fn enable_raw_mode() {
    unsafe {
        let std_out_handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let error_code = GetLastError();
        if error_code != 0 {
            panic!("Could not GetStdHandle");
        }

        let mut console_mode: u32 = 0;
        GetConsoleMode(std_out_handle, &mut console_mode);
        let error_code = GetLastError();
        if error_code != 0 {
            panic!("Could not GetConsoleMode");
        }

        let new_mode = console_mode & !(ENABLE_WRAP_AT_EOL_OUTPUT | ENABLE_LINE_INPUT);

        SetConsoleMode(std_out_handle, new_mode);
        let error_code = GetLastError();
        if error_code != 0 {
            panic!("Could not SetConsoleMode");
        }
    }
}

pub fn disable_raw_mode() {
    unsafe {
        let std_out_handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let error_code = GetLastError();
        if error_code != 0 {
            panic!("Could not GetStdHandle");
        }

        let mut console_mode: u32 = 0;
        GetConsoleMode(std_out_handle, &mut console_mode);
        let error_code = GetLastError();
        if error_code != 0 {
            panic!("Could not GetConsoleMode");
        }

        let new_mode = console_mode | (ENABLE_WRAP_AT_EOL_OUTPUT | ENABLE_LINE_INPUT);

        SetConsoleMode(std_out_handle, new_mode);
        let error_code = GetLastError();
        if error_code != 0 {
            panic!("Could not SetConsoleMode");
        }
    }
}