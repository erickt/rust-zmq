// dlsym.rs is taken from mio:
// https://github.com/carllerche/mio/blob/master/src/sys/unix/dlsym.rs
// The windows code is based on https://github.com/rust-lang/rust/blob/master/src/librustc_metadata/dynamic_lib.rs

macro_rules! dlsym {
	(fn $name:ident($($t:ty),*) -> $ret:ty) => (
		#[allow(bad_style)]
		static $name: ::dlsym::DlSym<unsafe extern fn($($t),*) -> $ret> =
			::dlsym::DlSym {
				name: concat!(stringify!($name), "\0"),
				addr: ::std::sync::atomic::AtomicUsize::new(0),
				_marker: ::std::marker::PhantomData,
			};
	)
}

use std::marker;
use std::mem;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct DlSym<F> {
    pub name: &'static str,
    pub addr: AtomicUsize,
    pub _marker: marker::PhantomData<F>,
}

impl<F> DlSym<F> {
    pub fn get(&self) -> Option<&F> {
        assert_eq!(mem::size_of::<F>(), mem::size_of::<usize>());
        unsafe {
            if self.addr.load(Ordering::SeqCst) == 0 {
                self.addr.store(fetch(self.name), Ordering::SeqCst);
            }
            if self.addr.load(Ordering::SeqCst) == 1 {
                None
            } else {
                mem::transmute::<&AtomicUsize, Option<&F>>(&self.addr)
            }
        }
    }
}

#[cfg(unix)]
unsafe fn fetch(name: &str) -> usize {
    use libc;
    assert_eq!(name.as_bytes()[name.len() - 1], 0);
    match libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr() as *const _) as usize {
        0 => 1,
        n => n,
    }
}

#[cfg(windows)]
unsafe fn fetch(name: &str) -> usize {
    use libc::c_void;
    use zmq_sys::zmq_socket; // a canary function which should be in the zmq library

    type DWORD = u32;
    type HMODULE = *mut u8;
    type BOOL = i32;
    type LPCSTR = *const i8;

    extern "system" {
        fn GetModuleHandleExW(dwFlags: DWORD, name: *const u8, handle: *mut HMODULE) -> BOOL;
        fn GetProcAddress(handle: HMODULE, name: LPCSTR) -> *mut c_void;
    }

    assert_eq!(name.as_bytes()[name.len() - 1], 0);
    const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 0x4;
    const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 0x2; // we already linked this module, so it should held in memory by us.
    let mut handle = std::ptr::null_mut();
    match GetModuleHandleExW(
        GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
        zmq_socket as *const u8,
        &mut handle,
    ) {
        0 => GetProcAddress(handle, name.as_ptr() as *const _) as usize,
        _ => 1,
    }
}
