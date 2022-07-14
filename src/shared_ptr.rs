use crate::weak_ptr::{WeakPtr, WeakPtrTarget};
use core::ffi::c_void;
use core::fmt::{self, Debug, Display};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use std::mem::MaybeUninit;

pub trait SharedPtrTarget {
    unsafe fn __drop(this: *mut c_void);
    unsafe fn __clone(this: *const c_void, out: *mut c_void);
}

#[repr(C)]
pub struct SharedPtr<T: SharedPtrTarget> {
    ptr: *mut c_void,
    ctrl: *mut c_void,
    pd: PhantomData<T>,
}

impl<T: SharedPtrTarget> SharedPtr<T> {
    pub fn get_ptr(&self) -> *const T {
        self.ptr as *const T
    }

    pub fn null() -> Self {
        SharedPtr {
            ptr: std::ptr::null_mut(),
            ctrl: std::ptr::null_mut(),
            pd: PhantomData,
        }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn as_ref(&self) -> Option<&T> {
        unsafe { (self.ptr as *const T).as_ref() }
    }

    pub fn as_mut(&mut self) -> Option<&mut T>
    where
        T: Unpin,
    {
        unsafe { (self.ptr as *mut T).as_mut() }
    }

    pub fn as_pin_mut(&mut self) -> Option<Pin<&mut T>> {
        unsafe {
            let p = (self.ptr as *mut T).as_mut()?;
            Some(Pin::new_unchecked(p))
        }
    }

    pub fn pin_mut(&mut self) -> Pin<&mut T> {
        match self.as_pin_mut() {
            Some(target) => target,
            None => panic!(
                "called pin_mut on a null UniquePtr<{}>",
                std::any::type_name::<T>(),
            ),
        }
    }

    pub fn downgrade(&self) -> WeakPtr<T>
    where
        T: WeakPtrTarget,
    {
        let mut out = MaybeUninit::<WeakPtr<T>>::uninit();
        unsafe {
            T::__downgrade(
                self as *const Self as *const c_void,
                out.as_mut_ptr().cast(),
            );
            out.assume_init()
        }
    }
}

unsafe impl<T> Send for SharedPtr<T> where T: Send + SharedPtrTarget {}
unsafe impl<T> Sync for SharedPtr<T> where T: Sync + SharedPtrTarget {}

impl<T: SharedPtrTarget> Clone for SharedPtr<T> {
    fn clone(&self) -> Self {
        let mut out = MaybeUninit::<Self>::uninit();
        unsafe {
            T::__clone(
                self as *const Self as *const c_void,
                out.as_mut_ptr().cast(),
            );
            out.assume_init()
        }
    }
}

impl<T: SharedPtrTarget> Drop for SharedPtr<T> {
    fn drop(&mut self) {
        unsafe {
            T::__drop(self as *mut Self as *mut c_void);
        }
    }
}

impl<T: SharedPtrTarget> Deref for SharedPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.as_ref() {
            Some(target) => target,
            None => panic!(
                "called deref on a null SharedPtr<{}>",
                std::any::type_name::<T>(),
            ),
        }
    }
}

impl<T: SharedPtrTarget + Unpin> DerefMut for SharedPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self.as_mut() {
            Some(target) => target,
            None => panic!(
                "called deref_mut on a null SharedPtr<{}>",
                std::any::type_name::<T>(),
            ),
        }
    }
}

impl<T: Debug + SharedPtrTarget> Debug for SharedPtr<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.as_ref() {
            None => formatter.write_str("nullptr"),
            Some(value) => Debug::fmt(value, formatter),
        }
    }
}

impl<T: Display + SharedPtrTarget> Display for SharedPtr<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.as_ref() {
            None => formatter.write_str("nullptr"),
            Some(value) => Display::fmt(value, formatter),
        }
    }
}
