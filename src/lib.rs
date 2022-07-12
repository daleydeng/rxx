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

#[cfg(test)]
mod tests {
    use super::*;
    use core::ffi::c_void;
    use std::mem::MaybeUninit;

    impl UniquePtrTarget for i64 {
        unsafe fn __drop(this: *mut c_void) {
            extern "C" {
                fn test_delete_unique_ptr(this: *mut c_void);
            }
            test_delete_unique_ptr(this);
        }
    }

    fn test_new_unique_ptr(v: i64) -> UniquePtr<i64> {
        extern "C" {
            fn test_new_unique_ptr(val: i64, out: *mut c_void);
        }

        let mut out = MaybeUninit::<UniquePtr<i64>>::uninit();
        unsafe {
            test_new_unique_ptr(v, out.as_mut_ptr() as *mut c_void);
            out.assume_init()
        }
    }

    impl SharedPtrTarget for i64 {
        unsafe fn __clone(this: *const c_void, out: *mut c_void) {
            extern "C" {
                fn test_clone_shared_ptr(this: *const c_void, out: *mut c_void);
            }
            test_clone_shared_ptr(this, out);
        }

        unsafe fn __drop(this: *mut c_void) {
            extern "C" {
                fn test_delete_shared_ptr(this: *mut c_void);
            }
            test_delete_shared_ptr(this);
        }
    }

    fn test_new_shared_ptr(v: i64) -> SharedPtr<i64> {
        extern "C" {
            fn test_new_shared_ptr(val: i64, obj: *mut c_void);
        }

        let mut out = MaybeUninit::<SharedPtr<i64>>::uninit();
        unsafe {
            test_new_shared_ptr(v, out.as_mut_ptr() as *mut c_void);
            out.assume_init()
        }
    }

    impl WeakPtrTarget for i64 {
        unsafe fn __clone(this: *const c_void, out: *mut c_void) {
            extern "C" {fn test_clone_weak_ptr(this: *const c_void, out: *mut c_void);}
            test_clone_weak_ptr(this, out);
        }

        unsafe fn __drop(this: *mut c_void) {
            extern "C" {fn test_delete_weak_ptr(this: *mut c_void);}
            test_delete_weak_ptr(this);
        }

        unsafe fn __downgrade(shared: *const c_void, weak: *mut c_void) {
            extern "C" {fn test_downgrade_weak_ptr(shared: *const c_void, weak: *mut c_void);}
            test_downgrade_weak_ptr(shared, weak);
        }

        unsafe fn __upgrade(weak: *const c_void, shared: *mut c_void) {
            extern "C" {
                fn test_upgrade_weak_ptr(weak: *const c_void, shared: *mut c_void);
            }
            test_upgrade_weak_ptr(weak, shared);
        }
    }

    impl VectorElement for i64 {
        unsafe fn __drop(this: &mut CxxVector<i64>) {
            extern "C" {fn test_delete_vector(this: &mut CxxVector<i64>);}
            test_delete_vector(this);
        }

        unsafe fn __size(this: &CxxVector<i64>) -> usize {
            extern "C" {fn test_vector_size(this: &CxxVector<i64>) -> usize;}
            test_vector_size(this)
        }

        unsafe fn __get_unchecked(this: &CxxVector<i64>, pos: usize) -> &Self {
            extern "C" {fn test_vector_get(this: &CxxVector<i64>, pos: usize) -> &i64;}
            test_vector_get(this, pos)
        }
        unsafe fn __get_unchecked_mut(this: &mut CxxVector<i64>, pos: usize) -> &mut Self {
            extern "C" {fn test_vector_get_mut(this: &mut CxxVector<i64>, pos: usize) -> &mut i64;}
            test_vector_get_mut(this, pos)
        }

        unsafe fn __push_back(this: &mut CxxVector<i64>, value: &mut i64) {
            extern "C" {
                fn test_vector_push_back(this: &mut CxxVector<i64>, value: &mut i64);
            }
            test_vector_push_back(this, value);
        }

        unsafe fn __pop_back(this: &mut CxxVector<i64>, value: *mut i64) {
            extern "C" {
                fn test_vector_pop_back(this: &mut CxxVector<i64>, value: *mut i64);
            }
            test_vector_pop_back(this, value);
        }
    }

    fn test_new_vector(data: &[i64]) -> CxxVector<i64> {
        extern "C" {
            fn test_new_vector(
                data: *const i64,
                len: usize,
                out: *mut CxxVector<i64>
            );
        }

        let mut out = MaybeUninit::<CxxVector<i64>>::uninit();
        unsafe {
            test_new_vector(data.as_ptr(), data.len(), out.as_mut_ptr());
            out.assume_init()
        }
    }

    #[test]
    fn test_unique_ptr() {
        let v = 64;
        let o: UniquePtr<i64> = UniquePtr::null();
        assert_eq!(o.to_string(), "nullptr");
        assert!(o.is_null());
        let mut o = test_new_unique_ptr(v);
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
        let o = test_new_shared_ptr(v);
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
        assert_eq!(s.to_str().unwrap(), a);

        let c = &*s; // since we cannnot move CxxString, we can reborrow it
        assert_eq!(c.len(), len);

        let mut d = s; // Pin self can copy
        assert_eq!(d.len(), len);

        d.as_mut().reserve(10);
        d.as_mut().push_str("abc");
        assert_eq!(d.to_str().unwrap(), "helloabc");

        d.as_mut().clear();
        assert_eq!(d.len(), 0);
    }

    #[test]
    fn test_vector() {
        let a = [1,2,3,4];
        let v = test_new_vector(&a);
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
}
