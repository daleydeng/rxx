use core::str;
use core::marker::{PhantomData, PhantomPinned};
use core::pin::Pin;
use core::fmt::{self, Debug};
use core::ops::Deref;

#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use alloc::string::String;

// C++ string has self reference, we cannot move it
#[repr(C)]
pub struct CxxString {
    _private: [u8; 0],
    _pin: PhantomData<PhantomPinned>,
}

// CxxString has self reference, so we need Pin
impl CxxString {
    pub fn string_len(&self) -> usize {
        extern "C" {
            fn rxx_string_length(this: &CxxString) -> usize;
        }

        unsafe { rxx_string_length(self) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.as_ptr(), self.string_len()) }
    }

    pub fn as_ptr(&self) -> *const u8 {
        extern "C" {
            fn rxx_string_data(this: &CxxString) -> *const u8;
        }
        unsafe { rxx_string_data(self) }
    }

    pub fn to_str(&self) -> &str {
        str::from_utf8(self.as_bytes()).unwrap()
    }

    #[cfg(feature = "alloc")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
    pub fn to_string_lossy(&self) -> Cow<str> {
        String::from_utf8_lossy(self.as_bytes())
    }

    pub fn clear(self: Pin<&mut Self>) {
        extern "C" {
            fn rxx_string_clear(this: Pin<&mut CxxString>);
        }
        unsafe { rxx_string_clear(self) }
    }

    pub fn reserve(self: Pin<&mut Self>, n: usize) {
        extern "C" {
            fn rxx_string_reserve(this: Pin<&mut CxxString>, n: usize);
        }
        unsafe { rxx_string_reserve(self, n) }
    }

    pub fn push_str(self: Pin<&mut Self>, s: &str) {
        self.push_bytes(s.as_bytes());
    }

    pub fn push_bytes(self: Pin<&mut Self>, bytes: &[u8]) {
        extern "C" {
            fn rxx_string_push(this: Pin<&mut CxxString>, ptr: *const u8, len: usize);
        }
        unsafe { rxx_string_push(self, bytes.as_ptr(), bytes.len()) }
    }
}

impl Debug for CxxString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.as_bytes())
    }
}

impl Deref for CxxString
{
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}


const STRING_SIZE: usize = 32; // gcc version > 5 or STRING_SIZE = 8

#[repr(C)]
#[derive(Default, Debug)]
pub struct StackString {
    space: [u8; STRING_SIZE],
    _pin: PhantomData<PhantomPinned>,
}

impl StackString {
    pub fn init(&mut self, val: &str) -> Pin<&mut CxxString> {
        extern "C" {
            fn rxx_string_init(ptr: *const u8, len: usize, out: &mut StackString);
        }
        unsafe {rxx_string_init(val.as_ptr(), val.len(), self);}
        self.pin_str()
    }

    pub fn pin_str(&mut self) -> Pin<&mut CxxString> {
        unsafe {Pin::new_unchecked(&mut *(self as *mut Self as *mut CxxString))}
    }
}

impl Drop for StackString {
    fn drop(&mut self) {
        extern "C" {
            fn rxx_string_destroy(this: &mut StackString);
        }
        unsafe { rxx_string_destroy(self) }
    }
}
