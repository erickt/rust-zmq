extern crate zmq_sys;

use libc::{c_int, c_uint, size_t, int64_t, uint64_t};
use std::os::raw::c_void;
use std::{mem, ptr, str};
use std::result;

use super::{Result, PollEvents};

pub trait Getter where Self: Sized {
    fn get(sock: *mut c_void, opt: c_int) -> Result<Self>;
}

pub trait Setter where Self: Sized {
    fn set(sock: *mut c_void, opt: c_int, value: Self) -> Result<()>;
}

macro_rules! getsockopt_num(
    ($c_ty:ty, $ty:ty) => (
        impl Getter for $ty {
            #[allow(trivial_casts)]
            fn get(sock: *mut c_void, opt: c_int) -> Result<$ty> {
                let mut value: $c_ty = 0;
                let value_ptr = &mut value as *mut $c_ty;
                let mut size = mem::size_of::<$c_ty>() as size_t;

                zmq_try!(unsafe {
                    zmq_sys::zmq_getsockopt(
                        sock,
                        opt,
                        value_ptr as *mut c_void,
                        &mut size)
                });
                Ok(value as $ty)
            }
        }
    )
);

getsockopt_num!(c_int, i32);
getsockopt_num!(c_uint, u32);
getsockopt_num!(int64_t, i64);
getsockopt_num!(uint64_t, u64);

pub fn get_bytes(sock: *mut c_void, opt: c_int, size: size_t) -> Result<Vec<u8>> {
    let mut size = size;
    let mut value = vec![0u8; size];

    zmq_try!(unsafe {
        zmq_sys::zmq_getsockopt(
            sock,
            opt,
            value.as_mut_ptr() as *mut c_void,
            &mut size)
    });
    value.truncate(size);
    Ok(value)
}

pub fn get_string(sock: *mut c_void, opt: c_int, size: size_t, remove_nulbyte: bool)
                  -> Result<result::Result<String, Vec<u8>>> {
    let mut value = try!(get_bytes(sock, opt, size));

    if remove_nulbyte {
        value.pop();
    }
    Ok(String::from_utf8(value).map_err(|e| e.into_bytes()))
}

macro_rules! setsockopt_num(
    ($ty:ty) => (
        impl Setter for $ty {
            #[allow(trivial_casts)]
            fn set(sock: *mut c_void, opt: c_int, value: $ty) -> Result<()> {
                let size = mem::size_of::<$ty>() as size_t;

                zmq_try!(unsafe {
                    zmq_sys::zmq_setsockopt(
                        sock,
                        opt,
                        (&value as *const $ty) as *const c_void,
                        size)
                });
                Ok(())
            }
        }
    )
);

setsockopt_num!(i32);
setsockopt_num!(i64);
setsockopt_num!(u64);

fn setsockopt_null(sock: *mut c_void, opt: c_int) -> Result<()> {
    zmq_try!(unsafe { zmq_sys::zmq_setsockopt(sock, opt, ptr::null(), 0) });
    Ok(())
}

impl<'a> Setter for &'a str {
    fn set(sock: *mut c_void, opt: c_int, value: Self) -> Result<()> {
        set(sock, opt, value.as_bytes())
    }
}

impl<'a> Setter for Option<&'a str> {
    fn set(sock: *mut c_void, opt: c_int, value: Self) -> Result<()> {
        if let Some(s) = value {
            set(sock, opt, s.as_bytes())
        } else {
            setsockopt_null(sock, opt)
        }
    }
}

impl Getter for bool {
    fn get(sock: *mut c_void, opt: c_int) -> Result<Self> {
        let result: i32 = try!(get(sock, opt));
        Ok(result == 1)
    }
}

impl Setter for bool {
    fn set(sock: *mut c_void, opt: c_int, value: Self) -> Result<()> {
        set(sock, opt, if value { 1i32 } else { 0i32 })
    }
}

impl<'a> Setter for &'a [u8] {
    fn set(sock: *mut c_void, opt: c_int, value: &'a [u8]) -> Result<()> {
        zmq_try!(unsafe {
            zmq_sys::zmq_setsockopt(
                sock,
                opt,
                value.as_ptr() as *const c_void,
                value.len() as size_t
            )
        });
        Ok(())
    }
}

impl Getter for PollEvents {
    fn get(sock: *mut c_void, opt: c_int) -> Result<Self> {
        get::<c_int>(sock, opt).map(|bits| PollEvents::from_bits_truncate(bits as i16))
    }
}

pub fn get<T: Getter>(sock: *mut c_void, opt: c_int) -> Result<T> {
    T::get(sock, opt)
}

pub fn set<T: Setter>(sock: *mut c_void, opt: c_int, value: T) -> Result<()> {
    T::set(sock, opt, value)
}
