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

    unsafe {
        ioctl(STDIN_FILENO, TIOCGWINSZ, &mut size);
    }

    Ok((size.ws_row as usize, size.ws_col as usize))
}