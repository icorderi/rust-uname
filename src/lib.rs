extern crate libc;

use std::io;
use std::ffi::CString;

use libc::{utsname, c_char};

#[derive(Debug)]
pub struct Info {
    pub sysname: String,
    pub nodename: String,
    pub release: String,
    pub version: String,
    pub machine: String,
    _priv: (),
}

impl Info {
    pub fn new() -> io::Result<Self> {
        unsafe {
            let mut n = std::mem::zeroed();
            if libc::uname(&mut n) == 0 {
                Ok(From::from(n))
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }
}

fn parse_or(buf: &mut [c_char], default: String) -> String {
    unsafe { CString::from_raw(buf.as_mut_ptr()).into_string().unwrap_or(default) }
}

impl From<utsname> for Info {
    fn from(mut x: utsname) -> Self {
        Info {
            sysname: parse_or(&mut x.sysname[..], String::default()),
            nodename: parse_or(&mut x.nodename[..], String::default()),
            release: parse_or(&mut x.release[..], String::default()),
            version: parse_or(&mut x.version[..], String::default()),
            machine: parse_or(&mut x.machine[..], String::default()),
            _priv: (),
        }
    }
}

pub fn uname() -> io::Result<Info> {
    Info::new()
}
