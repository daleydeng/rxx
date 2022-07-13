use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::slice;
use core::mem::{ManuallyDrop, MaybeUninit};
// use core::marker::{PhantomData, PhantomPinned};

pub trait VectorElement: Sized {
    unsafe fn __drop(this: &mut CxxVector<Self>);
    unsafe fn __size(this: &CxxVector<Self>) -> usize;
    unsafe fn __get_unchecked(this: &CxxVector<Self>, pos: usize) -> &Self;
    unsafe fn __get_unchecked_mut(this: &mut CxxVector<Self>, pos: usize) -> &mut Self;
    unsafe fn __push_back(this: &mut CxxVector<Self>, value: &mut Self);
    unsafe fn __pop_back(this: &mut CxxVector<Self>, value: *mut Self);
}

const VECTOR_SIZE: usize = 24;

#[repr(C)]
pub struct CxxVector<T: VectorElement> {
    _space: [u8; VECTOR_SIZE],
    _pd: PhantomData<T>
}

impl<T: VectorElement> CxxVector<T> {
    pub fn vector_len(&self) -> usize {
        unsafe { T::__size(self) }
    }

    pub unsafe fn get_unchecked(&self, pos: usize) -> &T {
        T::__get_unchecked(self, pos)
    }

    pub unsafe fn get_unchecked_mut(&mut self, pos: usize) -> &mut T {
        T::__get_unchecked_mut(self, pos)
    }

    pub fn get(&self, pos: usize) -> Option<&T> {
        if pos < self.len() {
            Some(unsafe {self.get_unchecked(pos)})
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, pos: usize) -> Option<&mut T> {
        if pos < self.len() {
            Some(unsafe {self.get_unchecked_mut(pos)})
        } else {
            None
        }
    }

     /// Returns a slice to the underlying contiguous array of elements.
    pub fn as_slice(&self) -> &[T] {
        let len = self.vector_len();
        if len == 0 {
            // The slice::from_raw_parts in the other branch requires a nonnull
            // and properly aligned data ptr. C++ standard does not guarantee
            // that data() on a vector with size 0 would return a nonnull
            // pointer or sufficiently aligned pointer, so using it would be
            // undefined behavior. Create our own empty slice in Rust instead
            // which upholds the invariants.
            &[]
        } else {
            unsafe {
                let d = self.get_unchecked(0);
                slice::from_raw_parts(d as *const T, len)
            }
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T]
    {
        let len = self.len();
        if len == 0 {
            &mut []
        } else {
            unsafe {
                let d = self.get_unchecked_mut(0);
                slice::from_raw_parts_mut(d as *mut T, len)
            }
        }
    }

    pub fn push(&mut self, value: T) {
        // disable value's destructor, C++ calls move constructor fallowed by destructor on `value`, C++ manages this
        let mut value = ManuallyDrop::new(value);
        unsafe {
            T::__push_back(self, &mut value);
        }
    }

    pub fn pop(&mut self) -> Option<T>
    {
        if self.is_empty() {
            None
        } else {
            let mut out = MaybeUninit::uninit();
            Some(unsafe {
                T::__pop_back(self, out.as_mut_ptr());
                out.assume_init()
            })
        }
    }
}

impl<T: VectorElement> Deref for CxxVector<T>
{
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T: VectorElement> DerefMut for CxxVector<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T: VectorElement> Drop for CxxVector<T> {
    fn drop(&mut self) {
        unsafe { T::__drop(self) }
    }
}
