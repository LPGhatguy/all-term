use libc::{
    winsize,
    ioctl,
    TIOCGWINSZ,
    STDIN_FILENO,
};

pub fn get_terminal_size() -> Result<(usize, usize), String> {
    let mut size = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    let result = unsafe {
        ioctl(STDIN_FILENO, TIOCGWINSZ, &mut size)
    };

    if result == 0 {
        Ok((size.ws_col as usize, size.ws_row as usize))
    } else {
        Err(format!("ioctl error {}", result))
    }
}