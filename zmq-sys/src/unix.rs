pub extern crate libc;

pub use std::os::unix::io::RawFd as RawSocket;
pub use self::libc as errno;
