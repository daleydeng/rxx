#![allow(clippy::missing_safety_doc)]
#![feature(associated_type_defaults)]

pub mod unique_ptr;
pub use unique_ptr::*;

pub mod shared_ptr;
pub use shared_ptr::*;

pub mod weak_ptr;
pub use weak_ptr::*;

pub mod cxx_string;
pub use cxx_string::*;

pub mod cxx_vector;
pub use cxx_vector::*;

pub mod ffi;
pub use ffi::*;

#[cfg(test)]
mod tests {
    use core::ffi::c_void;
    use std::marker::PhantomData;
    use std::mem::MaybeUninit;

    use super::*;

    genrs_fn!(pub fn rxx_dummy_cpp_new_vector_i64(a: i32) -> CxxVector<i64>);

    genrs_fn!(pub fn rxx_dummy_cpp_add_vector_i64(a: &mut CxxVector<i64>, i: i32));
    genrs_fn!(pub fn rxx_dummy_cpp_addret_vector_i64(a: &mut CxxVector<i64>, i: i32) -> i64, cret=atomic); // match with build.rs

    genrs_fn!(pub fn rxx_dummy_cpp_get_vector_i64(a: &CxxVector<i64>) -> i64);
    genrs_fn!(pub fn rxx_dummy_cpp_getvoid_vector_i64(a: &CxxVector<i64>, i: i32));
    genrs_fn!(pub fn rxx_dummy_cpp_getref_vector_i64<'a>(a: &'a CxxVector<i64>, i: i32) -> &'a i64, cret=atomic);

    genrs_fn!(CxxVector<i64>;; pub fn add(&mut self, a: i32), ln=rxx_dummy_cpp_add_vector_i64);
    genrs_fn!(CxxVector<i64>;; pub fn addret(&mut self, a: i32) -> i64, cret=atomic, ln=rxx_dummy_cpp_addret_vector_i64);
    genrs_fn!(CxxVector<i64>;; pub fn get1(&self) -> i64, ln=rxx_dummy_cpp_get_vector_i64);
    genrs_fn!(CxxVector<i64>;; pub fn getvoid(&self, a: i32), ln=rxx_dummy_cpp_getvoid_vector_i64);
    genrs_fn!(CxxVector<i64>;; pub fn getref(&self, a: i32) -> &i64, cret=atomic, ln=rxx_dummy_cpp_getref_vector_i64);

    genrs_unique_ptr!(rxx_unique_i64, i64);
    genrs_shared_ptr!(rxx_shared_i64, i64);
    genrs_weak_ptr!(rxx_weak_i64, i64);
    genrs_vector!(rxx_vector_i64, i64);

    fn new_unique_i64(v: i64) -> UniquePtr<i64> {
        extern "C" {
            #[link_name = "rxx_dummy_new_unique_i64"]
            fn __func(val: i64, out: *mut c_void);
        }

        let mut out = MaybeUninit::<UniquePtr<i64>>::uninit();
        unsafe {
            __func(v, out.as_mut_ptr() as *mut c_void);
            out.assume_init()
        }
    }

    fn new_shared_i64(v: i64) -> SharedPtr<i64> {
        extern "C" {
            #[link_name = "rxx_dummy_new_shared_i64"]
            fn __func(val: i64, obj: *mut c_void);
        }

        let mut out = MaybeUninit::<SharedPtr<i64>>::uninit();
        unsafe {
            __func(v, out.as_mut_ptr() as *mut c_void);
            out.assume_init()
        }
    }

    fn new_vector_i64(data: &[i64]) -> CxxVector<i64> {
        extern "C" {
            #[link_name = "rxx_dummy_new_vector_i64"]
            fn __func(data: *const i64, len: usize, out: *mut CxxVector<i64>);
        }

        let mut out = MaybeUninit::<CxxVector<i64>>::uninit();
        unsafe {
            __func(data.as_ptr(), data.len(), out.as_mut_ptr());
            out.assume_init()
        }
    }

    fn new_unique_string() -> UniquePtr<CxxString> {
        extern "C" {
            #[link_name = "rxx_dummy_new_unique_string"]
            fn __func(out: *mut UniquePtr<CxxString>);
        }
        let mut out = MaybeUninit::<UniquePtr<CxxString>>::uninit();
        unsafe {
            __func(out.as_mut_ptr());
            out.assume_init()
        }
    }

    fn new_shared_ptr_string() -> SharedPtr<CxxString> {
        extern "C" {
            #[link_name = "rxx_dummy_new_shared_string"]
            fn __func(out: *mut SharedPtr<CxxString>);
        }
        let mut out = MaybeUninit::<SharedPtr<CxxString>>::uninit();
        unsafe {
            __func(out.as_mut_ptr());
            out.assume_init()
        }
    }

    #[repr(C)]
    struct Dummy<'a> {
        data: *mut i64,
        len: usize,
        _ty: PhantomData<&'a i64>,
    }

    genrs_fn!(Dummy<'_>;; pub fn get(&self, idx: usize) -> i64, cret=atomic, ln=rxx_Dummy_get);
    genrs_fn!(Dummy<'a>; impl<'a>; pub fn get_mut(&mut self, idx: usize) -> &'a mut i64, cret=atomic, ln=rxx_Dummy_get_mut);
    genrs_fn!(Dummy<'_>;; pub fn add(&mut self, val: i64), ln=rxx_Dummy_add);

    #[test]
    fn test_cpp_fn() {
        let mut a = rxx_dummy_cpp_new_vector_i64(123);
        assert_eq!(a[0], 123);

        rxx_dummy_cpp_add_vector_i64(&mut a, 1);
        assert_eq!(a[0], 124);

        let b = rxx_dummy_cpp_addret_vector_i64(&mut a, 20);
        assert_eq!(b, 144);

        let c = rxx_dummy_cpp_get_vector_i64(&a);
        assert_eq!(c, 144);

        rxx_dummy_cpp_getvoid_vector_i64(&a, 10);

        a.add(20);
        assert_eq!(a[0], 164);

        assert_eq!(a.addret(20), 184);
        assert_eq!(a.get1(), 184);
        a.getvoid(10);

        let b = rxx_dummy_cpp_getref_vector_i64(&a, 0);
        assert_eq!(*b, 184);

        let b = a.getref(0);
        assert_eq!(*b, 184);
    }

    #[test]
    fn test_cpp_cls() {
        let mut buf = [1i64, 2, 3, 4];
        let a = Dummy {
            data: buf.as_mut_ptr(),
            len: buf.len(),
            _ty: PhantomData,
        };

        assert_eq!(a.get(0), 1);
        assert_eq!(a.get(2), 3);

        let mut b = a;
        let i = b.get_mut(0);
        *i = 8;
        assert_eq!(b.get(0), 8);

        b.add(3);
        assert_eq!(b.get(0), 11);
    }

    #[test]
    fn test_unique_ptr() {
        let v = 64;
        let o: UniquePtr<i64> = UniquePtr::null();
        assert_eq!(o.to_string(), "nullptr");
        assert!(o.is_null());
        let mut o = new_unique_i64(v);

        assert!(!o.is_null());
        assert_eq!(*o, v);

        let mut b = o.pin_mut();
        *b = 5;
        assert_eq!(*b, 5);
    }

    #[test]
    fn test_shared_ptr() {
        let v = 64;
        let o: SharedPtr<i64> = SharedPtr::null();
        assert_eq!(o.to_string(), "nullptr");
        assert!(o.is_null());
        let o = new_shared_i64(v);
        assert!(!o.is_null());
        assert_eq!(*o, v);

        let mut b = o.clone();
        assert_eq!(*b, v);

        let c = o.downgrade();
        assert_eq!(*c.upgrade(), v);

        let bb = b.pin_mut();
        assert_eq!(*bb, v);
    }

    #[test]
    fn test_string() {
        let a = "hello";
        let len = a.len();
        let mut s = StackString::default();
        let s = s.init(a);

        assert_eq!(s.len(), len);
        assert_eq!(s.to_str(), a);

        let c = &*s; // since we cannnot move CxxString, we can reborrow it
        assert_eq!(c.len(), len);

        let mut d = s; // Pin self can copy
        assert_eq!(d.len(), len);

        d.as_mut().reserve(10);
        d.as_mut().push_str("abc");
        assert_eq!(d.to_str(), "helloabc");

        d.as_mut().clear();
        assert_eq!(d.len(), 0);
    }

    #[test]
    fn test_vector() {
        let a = [1, 2, 3, 4];
        let v = new_vector_i64(&a);
        let mut b = v;
        assert_eq!(b.len(), 4);
        assert_eq!(*b.get(2).unwrap(), 3);

        b[0] = 5;
        assert_eq!(b.as_slice(), &[5, 2, 3, 4]);

        b.push(3);
        assert_eq!(b.as_slice(), &[5, 2, 3, 4, 3]);

        let c = b.pop().unwrap();
        assert_eq!(c, 3);
    }

    #[test]
    fn test_unique_string() {
        let s = new_unique_string();
        assert_eq!(s.to_str(), "test");
    }

    #[test]
    fn test_shared_string() {
        let s = new_shared_ptr_string();
        assert_eq!(s.to_str(), "test");
    }
}
