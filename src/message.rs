use libc::size_t;

use std::cmp::Ordering;
use std::ffi;
use std::fmt;
use std::hash::{Hash, Hasher};
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

unsafe extern "C" fn drop_msg_data_box(data: *mut c_void, hint: *mut c_void) {
    drop(Box::from_raw(ptr::slice_from_raw_parts_mut(
        data as *mut u8,
        hint as usize,
    )));
}

impl Message {
    unsafe fn alloc<F>(f: F) -> Message
    where
        F: FnOnce(&mut zmq_sys::zmq_msg_t) -> i32,
    {
        let mut msg = zmq_sys::zmq_msg_t::default();
        let rc = f(&mut msg);
        if rc == -1 {
            panic!("{}", errno_to_error())
        }
        Message { msg }
    }

    /// Create an empty `Message`.
    pub fn new() -> Message {
        unsafe { Self::alloc(|msg| zmq_sys::zmq_msg_init(msg)) }
    }

    /// Create a `Message` from an initialized `zmq_sys::zmq_msg_t`.
    ///
    /// # Safety
    ///
    /// `msg` must be initialized.
    pub unsafe fn from_msg(msg: zmq_sys::zmq_msg_t) -> Self {
        Message { msg }
    }

    /// Create a `Message` preallocated with `len` uninitialized bytes.
    ///
    /// Since it is very easy to introduce undefined behavior using this
    /// function, its use is not recommended, and it will be removed in a future
    /// release. If there is a use-case that cannot be handled efficiently by
    /// the safe message constructors, please file an issue.
    ///
    /// # Safety
    ///
    /// The returned message contains uninitialized memory, and hence the
    /// `Deref` and `DerefMut` traits must not be used until the memory has been
    /// initialized. Since there is no proper API to do so, this function is
    /// basically not usable safely, unless you happen to invoke C code that
    /// takes a raw message pointer and initializes its contents.
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
        Self::from(data)
    }

    /// Returns a raw pointer to the message to be used with ffi calls.
    pub fn as_message_ptr(&self) -> *const zmq_sys::zmq_msg_t {
        &self.msg
    }

    /// Returns a raw mutable pointer to the message to be used with ffi calls.
    pub fn as_mut_message_ptr(&mut self) -> *mut zmq_sys::zmq_msg_t {
        &mut self.msg
    }

    /// Returns the amount of bytes that are stored in this `Message`.
    pub fn len(&self) -> usize {
        unsafe { zmq_sys::zmq_msg_size(self.as_message_ptr()) }
    }

    /// Return `true` is there is at least one byte stored in this `Message`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a raw pointer to the buffer of this message.
    pub fn as_ptr(&self) -> *const u8 {
        let ptr = unsafe { zmq_sys::zmq_msg_data(self.as_message_ptr().cast_mut()) };
        ptr as *const u8
    }

    /// Returns a raw mutable pointer to the buffer of this message.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        let ptr = unsafe { zmq_sys::zmq_msg_data(self.as_mut_message_ptr()) };
        ptr as *mut u8
    }

    /// Return the message content as a string slice if it is valid UTF-8.
    pub fn as_str(&self) -> Option<&str> {
        str::from_utf8(self).ok()
    }

    /// Return the `ZMQ_MORE` flag, which indicates if more parts of a multipart
    /// message will follow.
    pub fn get_more(&self) -> bool {
        let rc = unsafe { zmq_sys::zmq_msg_more(&self.msg) };
        rc != 0
    }

    /// Query a message metadata property.
    ///
    /// # Non-UTF8 values
    ///
    /// The `zmq_msg_gets` man page notes "The encoding of the property and
    /// value shall be UTF8". However, this is not actually enforced. For API
    /// compatibility reasons, this function will return `None` when
    /// encountering a non-UTF8 value; so a missing and a non-UTF8 value cannot
    /// currently be distinguished.
    ///
    /// This is considered a bug in the bindings, and will be fixed with the
    /// next API-breaking release.
    pub fn gets<'a>(&'a mut self, property: &str) -> Option<&'a str> {
        let c_str = ffi::CString::new(property.as_bytes()).unwrap();

        let value = unsafe { zmq_sys::zmq_msg_gets(&self.msg, c_str.as_ptr()) };

        if value.is_null() {
            None
        } else {
            unsafe { ffi::CStr::from_ptr(value) }.to_str().ok()
        }
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Message) -> bool {
        self[..] == other[..]
    }
}

impl Eq for Message {}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> Ordering {
        self[..].cmp(&other[..])
    }
}

impl Clone for Message {
    fn clone(&self) -> Self {
        self[..].into()
    }
}

impl Default for Message {
    fn default() -> Self {
        Self::new()
    }
}

impl Hash for Message {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self[..].hash(state);
    }
}

impl Deref for Message {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        // This is safe because we're constraining the slice to the lifetime of
        // this message.
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
    }
}

impl DerefMut for Message {
    fn deref_mut(&mut self) -> &mut [u8] {
        // This is safe because we're constraining the slice to the lifetime of
        // this message.
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
    }
}

impl<'a> From<&'a [u8]> for Message {
    /// Construct a message from a byte slice by copying the data.
    fn from(data: &'a [u8]) -> Self {
        unsafe {
            let mut msg = Message::with_size_uninit(data.len());
            ptr::copy_nonoverlapping(data.as_ptr(), msg.as_mut_ptr(), data.len());
            msg
        }
    }
}

impl From<Vec<u8>> for Message {
    /// Construct a message from a byte vector without copying the data.
    fn from(msg: Vec<u8>) -> Self {
        Message::from(msg.into_boxed_slice())
    }
}

impl From<Box<[u8]>> for Message {
    /// Construct a message from a boxed slice without copying the data.
    fn from(data: Box<[u8]>) -> Self {
        let len = data.len();
        if len == 0 {
            return Message::new();
        }
        let raw = Box::into_raw(data);
        unsafe {
            Self::alloc(|msg| {
                zmq_sys::zmq_msg_init_data(
                    msg,
                    raw as *mut c_void,
                    len,
                    Some(drop_msg_data_box),
                    len as *mut c_void,
                )
            })
        }
    }
}

impl<'a> From<&'a str> for Message {
    /// Construct a message from a string slice by copying the UTF-8 data.
    fn from(msg: &str) -> Self {
        Message::from(msg.as_bytes())
    }
}

impl<'a> From<&'a String> for Message {
    /// Construct a message from a string slice by copying the UTF-8 data.
    fn from(msg: &String) -> Self {
        Message::from(msg.as_bytes())
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
