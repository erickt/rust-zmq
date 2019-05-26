use libc::size_t;

use std::ffi;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_void;
use std::{ptr, slice, str};

use super::errno_to_error;

/// Holds a 0MQ message.
///
/// A message is a single frame, either received or created locally and then
/// sent over the wire. Multipart messages are transmitted as multiple
/// `Message`s.
///
/// In rust-zmq, you aren't required to create message objects if you use the
/// convenience APIs provided (e.g. `Socket::recv_bytes()` or
/// `Socket::send()`). However, using message objects can make multiple
/// operations in a loop more efficient, since allocated memory can be reused.
pub struct Message {
    msg: zmq_sys::zmq_msg_t,
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe {
            let rc = zmq_sys::zmq_msg_close(&mut self.msg);
            assert_eq!(rc, 0);
        }
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

unsafe extern "C" fn drop_msg_content_box(data: *mut c_void, _hint: *mut c_void) {
    let _ = Box::from_raw(data as *mut u8);
}

impl Message {
    unsafe fn alloc<F>(f: F) -> Message
    where
        F: FnOnce(&mut zmq_sys::zmq_msg_t) -> i32,
    {
        let mut msg = zmq_sys::zmq_msg_t::default();
        let rc = f(&mut msg);
        if rc == -1 {
            panic!(errno_to_error())
        }
        Message { msg }
    }

    /// Create an empty `Message`.
    pub fn new() -> Message {
        unsafe { Self::alloc(|msg| zmq_sys::zmq_msg_init(msg)) }
    }

    /// Create a `Message` preallocated with `len` uninitialized bytes.
    ///
    /// Since it is very easy to introduce undefined behavior using this
    /// function, its use is not recommended, and it will be removed in a future
    /// release. If there is a use-case that cannot be handled efficiently by
    /// the safe message constructors, please file an issue.
    #[deprecated(
        since = "0.9.1",
        note = "This method has an unintuitive name, and should not be needed."
    )]
    pub unsafe fn with_capacity_unallocated(len: usize) -> Message {
        Self::alloc(|msg| zmq_sys::zmq_msg_init_size(msg, len as size_t))
    }

    unsafe fn with_size_uninit(len: usize) -> Message {
        Self::alloc(|msg| zmq_sys::zmq_msg_init_size(msg, len as size_t))
    }

    /// Create a `Message` with space for `len` bytes that are initialized to 0.
    pub fn with_size(len: usize) -> Message {
        unsafe {
            let mut msg = Message::with_size_uninit(len);
            ptr::write_bytes(msg.as_mut_ptr(), 0, len);
            msg
        }
    }

    /// Create a `Message` with space for `len` bytes that are initialized to 0.
    #[deprecated(
        since = "0.9.1",
        note = "This method has a name which does not match its semantics. Use `with_size` instead"
    )]
    pub fn with_capacity(len: usize) -> Message {
        Self::with_size(len)
    }

    /// Create a `Message` from a `&[u8]`. This will copy `data` into the message.
    ///
    /// This is equivalent to using the `From<&[u8]>` trait.
    #[deprecated(since = "0.9.1", note = "Use the `From` trait instead.")]
    pub fn from_slice(data: &[u8]) -> Message {
        Self::from_slice_(data)
    }

    fn from_slice_(data: &[u8]) -> Message {
        unsafe {
            let mut msg = Message::with_size_uninit(data.len());
            ptr::copy_nonoverlapping(data.as_ptr(), msg.as_mut_ptr(), data.len());
            msg
        }
    }

    fn from_box(data: Box<[u8]>) -> Message {
        let n = data.len();
        if n == 0 {
            return Message::new();
        }
        let raw = Box::into_raw(data);
        unsafe {
            Self::alloc(|msg| {
                zmq_sys::zmq_msg_init_data(
                    msg,
                    raw as *mut c_void,
                    n,
                    drop_msg_content_box as *mut zmq_sys::zmq_free_fn,
                    ptr::null_mut(),
                )
            })
        }
    }

    /// Return the message content as a string slice if it is valid UTF-8.
    pub fn as_str(&self) -> Option<&str> {
        str::from_utf8(self).ok()
    }

    /// Return the `ZMQ_MORE` flag, which indicates if more parts of a multipart
    /// message will follow.
    pub fn get_more(&self) -> bool {
        let rc = unsafe { zmq_sys::zmq_msg_more(&self.msg as *const _ as *mut _) };
        rc != 0
    }

    /// Query a message metadata property.
    pub fn gets<'a>(&'a mut self, property: &str) -> Option<&'a str> {
        let c_str = ffi::CString::new(property.as_bytes()).unwrap();

        let value = unsafe { zmq_sys::zmq_msg_gets(&mut self.msg, c_str.as_ptr()) };

        if value.is_null() {
            None
        } else {
            Some(unsafe { str::from_utf8(ffi::CStr::from_ptr(value).to_bytes()).unwrap() })
        }
    }
}

impl Deref for Message {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        // This is safe because we're constraining the slice to the lifetime of
        // this message.
        unsafe {
            let ptr = &self.msg as *const _ as *mut _;
            let data = zmq_sys::zmq_msg_data(ptr);
            let len = zmq_sys::zmq_msg_size(ptr) as usize;
            slice::from_raw_parts(data as *mut u8, len)
        }
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Message) -> bool {
        self[..] == other[..]
    }
}

impl Eq for Message {}

impl DerefMut for Message {
    fn deref_mut(&mut self) -> &mut [u8] {
        // This is safe because we're constraining the slice to the lifetime of
        // this message.
        unsafe {
            let data = zmq_sys::zmq_msg_data(&mut self.msg);
            let len = zmq_sys::zmq_msg_size(&mut self.msg) as usize;
            slice::from_raw_parts_mut(data as *mut u8, len)
        }
    }
}

impl<'a> From<&'a [u8]> for Message {
    /// Construct from a byte slice by copying the data.
    fn from(msg: &'a [u8]) -> Self {
        Message::from_slice_(msg)
    }
}

impl From<Vec<u8>> for Message {
    /// Construct from a byte vector without copying the data.
    fn from(msg: Vec<u8>) -> Self {
        Message::from_box(msg.into_boxed_slice())
    }
}

impl<'a> From<&'a str> for Message {
    /// Construct from a string slice by copying the UTF-8 data.
    fn from(msg: &str) -> Self {
        Message::from_slice_(msg.as_bytes())
    }
}

impl<'a> From<&'a String> for Message {
    /// Construct from a string slice by copying the UTF-8 data.
    fn from(msg: &String) -> Self {
        Message::from_slice_(msg.as_bytes())
    }
}

impl<'a, T> From<&'a T> for Message
where
    T: Into<Message> + Clone,
{
    fn from(v: &'a T) -> Self {
        v.clone().into()
    }
}

/// Get the low-level C pointer.
pub fn msg_ptr(msg: &mut Message) -> *mut zmq_sys::zmq_msg_t {
    &mut msg.msg
}
