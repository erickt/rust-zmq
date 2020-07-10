#[cfg(unix)]
use libc as errno;
#[cfg(windows)]
use windows::errno;

const ZMQ_HAUSNUMERO: i32 = 156_384_712;

pub const EACCES:           i32 = errno::EACCES;
pub const EADDRINUSE:       i32 = errno::EADDRINUSE;
pub const EAGAIN:           i32 = errno::EAGAIN;
pub const EBUSY:            i32 = errno::EBUSY;
pub const ECONNREFUSED:     i32 = errno::ECONNREFUSED;
pub const EFAULT:           i32 = errno::EFAULT;
pub const EINTR:            i32 = errno::EINTR;
pub const EHOSTUNREACH:     i32 = errno::EHOSTUNREACH;
pub const EINPROGRESS:      i32 = errno::EINPROGRESS;
pub const EINVAL:           i32 = errno::EINVAL;
pub const EMFILE:           i32 = errno::EMFILE;
pub const EMSGSIZE:         i32 = errno::EMSGSIZE;
pub const ENAMETOOLONG:     i32 = errno::ENAMETOOLONG;
pub const ENODEV:           i32 = errno::ENODEV;
pub const ENOENT:           i32 = errno::ENOENT;
pub const ENOMEM:           i32 = errno::ENOMEM;
pub const ENOTCONN:         i32 = errno::ENOTCONN;
pub const ENOTSOCK:         i32 = errno::ENOTSOCK;
pub const EPROTO:           i32 = errno::EPROTO;
pub const EPROTONOSUPPORT:  i32 = errno::EPROTONOSUPPORT;
pub const ENOTSUP:          i32 = errno::ENOTSUP;
pub const ENOBUFS:          i32 = errno::ENOBUFS;
pub const ENETDOWN:         i32 = errno::ENETDOWN;
pub const EADDRNOTAVAIL:    i32 = errno::EADDRNOTAVAIL;

// native zmq error codes
pub const EFSM:             i32 = ZMQ_HAUSNUMERO + 51;
pub const ENOCOMPATPROTO:   i32 = ZMQ_HAUSNUMERO + 52;
pub const ETERM:            i32 = ZMQ_HAUSNUMERO + 53;
pub const EMTHREAD:         i32 = ZMQ_HAUSNUMERO + 54;

// These may be returned by libzmq if the target platform does not define these
// errno codes.
pub const ENOTSUP_ALT:         i32 = ZMQ_HAUSNUMERO + 1;
pub const EPROTONOSUPPORT_ALT: i32 = ZMQ_HAUSNUMERO + 2;
pub const ENOBUFS_ALT:         i32 = ZMQ_HAUSNUMERO + 3;
pub const ENETDOWN_ALT:        i32 = ZMQ_HAUSNUMERO + 4;
pub const EADDRINUSE_ALT:      i32 = ZMQ_HAUSNUMERO + 5;
pub const EADDRNOTAVAIL_ALT:   i32 = ZMQ_HAUSNUMERO + 6;
pub const ECONNREFUSED_ALT:    i32 = ZMQ_HAUSNUMERO + 7;
pub const EINPROGRESS_ALT:     i32 = ZMQ_HAUSNUMERO + 8;
pub const ENOTSOCK_ALT:        i32 = ZMQ_HAUSNUMERO + 9;
pub const EMSGSIZE_ALT:        i32 = ZMQ_HAUSNUMERO + 10;
pub const EAFNOSUPPORT_ALT:    i32 = ZMQ_HAUSNUMERO + 11;
pub const ENETUNREACH_ALT:     i32 = ZMQ_HAUSNUMERO + 12;
pub const ECONNABORTED_ALT:    i32 = ZMQ_HAUSNUMERO + 13;
pub const ECONNRESET_ALT:      i32 = ZMQ_HAUSNUMERO + 14;
pub const ENOTCONN_ALT:        i32 = ZMQ_HAUSNUMERO + 15;
pub const ETIMEDOUT_ALT:       i32 = ZMQ_HAUSNUMERO + 16;
pub const EHOSTUNREACH_ALT:    i32 = ZMQ_HAUSNUMERO + 17;
pub const ENETRESET_ALT:       i32 = ZMQ_HAUSNUMERO + 18;
