#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
pub mod unix;

// Is this a good idea? Should I write a platform trait instead?
#[cfg(windows)]
pub use self::windows as current;

#[cfg(unix)]
pub use self::unix as current;