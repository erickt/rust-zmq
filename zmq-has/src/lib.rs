#[cfg(not(any(cross, olderzmq)))]
use std::ffi::CString;
#[cfg(not(any(cross, olderzmq)))]
use std::os::raw::{c_char, c_int};

#[cfg(not(any(cross, olderzmq)))]
#[link(name = "zmq")]
extern "C" {
    fn zmq_has(capability: *const c_char) -> c_int;
}

#[cfg(not(any(cross, olderzmq)))]
pub fn zmq_capabilities() -> Vec<String> {
    let mut res = Vec::<String>::with_capacity(6 as usize);
    for has in ["ipc", "pgm", "tipc", "norm", "curve", "gssapi"].into_iter() {
        if unsafe { zmq_has(CString::new(has.as_bytes()).unwrap().as_ptr()) } == 1 {
            res.push(has.to_string());
        }
    }
    res
}

macro_rules! zmq_has_from_features {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                if cfg!(feature = $x) {
                    temp_vec.push(String::from($x));
                }
            )*
            temp_vec
        }
    };
}

#[cfg(any(cross, olderzmq))]
pub fn zmq_capabilities() -> Vec<String> {
    zmq_has_from_features!["ipc", "pgm", "tipc", "norm", "curve", "gssapi"]
}