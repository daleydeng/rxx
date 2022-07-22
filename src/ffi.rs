use crate::CxxString;

#[macro_export]
macro_rules! genrs_unique_ptr{
    ($link_name:ident, $tp:ty, $c:ident) => {
	paste::paste! {
	    impl $c::UniquePtrTarget for $tp {
		unsafe fn __drop(this: *mut core::ffi::c_void) {
		    extern "C" {
			#[link_name=stringify!([<$link_name _delete>])]
			fn func(this: *mut core::ffi::c_void);
		    }
		    func(this);
		}
	    }
	}
    }
}

#[macro_export]
macro_rules! genrs_shared_ptr{
    ($link_name:ident, $tp:ident, $c:ident) => {
	paste::paste! {
	    impl $c::SharedPtrTarget for $tp {
		unsafe fn __drop(this: *mut core::ffi::c_void) {
		    extern "C" {
			#[link_name=stringify!([<$link_name _delete>])]
			fn func(this: *mut core::ffi::c_void);
		    }
		    func(this);
		}

		unsafe fn __clone(this: *const core::ffi::c_void, out: *mut core::ffi::c_void) {
		    extern "C" {
			#[link_name=stringify!([<$link_name _clone>])]
			fn func(this: *const core::ffi::c_void, out: *mut core::ffi::c_void);
		    }
		    func(this, out);
		}
	    }
	}
    }
}

#[macro_export]
macro_rules! genrs_weak_ptr{
    ($link_name:ident, $tp:ident, $c:ident) => {
	paste::paste! {
	    impl $c::WeakPtrTarget for $tp {
		unsafe fn __drop(this: *mut core::ffi::c_void) {
		    extern "C" {
			#[link_name=stringify!([<$link_name _delete>])]
			fn func(this: *mut core::ffi::c_void);
		    }
		    func(this);
		}

		unsafe fn __clone(this: *const core::ffi::c_void, out: *mut core::ffi::c_void) {
		    extern "C" {
			#[link_name=stringify!([<$link_name _clone>])]
			fn func(this: *const core::ffi::c_void, out: *mut core::ffi::c_void);
		    }
		    func(this, out);
		}

		unsafe fn __downgrade(shared: *const core::ffi::c_void, weak: *mut core::ffi::c_void) {
		    extern "C" {
			#[link_name=stringify!([<$link_name _downgrade>])]
			fn func(shared: *const core::ffi::c_void, weak: *mut core::ffi::c_void);
		    }
		    func(shared, weak);
		}

		unsafe fn __upgrade(weak: *const core::ffi::c_void, shared: *mut core::ffi::c_void) {
		    extern "C" {
			#[link_name=stringify!([<$link_name _upgrade>])]
			fn func(weak: *const core::ffi::c_void, shared: *mut core::ffi::c_void);
		    }
		    func(weak, shared);
		}
	    }
	}
    }
}

#[macro_export]
macro_rules! genrs_vector{
    ($link_name:ident, $tp:ty, $c:ident) => {
	paste::paste! {
	    impl $c::VectorElement for $tp {
		unsafe fn __drop(this: &mut CxxVector<$tp>) {
		    extern "C" {
			#[link_name=stringify!([<$link_name _delete>])]
			fn func(this: &mut CxxVector<$tp>);
		    }
		    func(this)
		}

		unsafe fn __size(this: &CxxVector<$tp>) -> usize {
		    extern "C" {
			#[link_name=stringify!([<$link_name _size>])]
			fn func(this: &CxxVector<$tp>) -> usize;
		    }
		    func(this)
		}

		unsafe fn __get_unchecked(this: &CxxVector<$tp>, pos: usize) -> &Self {
		    extern "C" {
			#[link_name=stringify!([<$link_name _get>])]
			fn func(this: &CxxVector<$tp>, pos: usize) -> &$tp;
		    }
		    func(this, pos)
		}

		unsafe fn __get_unchecked_mut(this: &mut CxxVector<$tp>, pos: usize) -> &mut Self {
		    extern "C" {
			#[link_name=stringify!([<$link_name _get_mut>])]
			fn func(this: &mut CxxVector<$tp>, pos: usize) -> &mut $tp;
		    }
		    func(this, pos)
		}

		unsafe fn __push_back(this: &mut CxxVector<$tp>, value: &mut $tp) {
		    extern "C" {
			#[link_name=stringify!([<$link_name _push_back>])]
			fn func(this: &mut CxxVector<$tp>, value: &mut $tp);
		    }
		    func(this, value)
		}

		unsafe fn __pop_back(this: &mut CxxVector<$tp>, value: *mut $tp) {
		    extern "C" {
			#[link_name=stringify!([<$link_name _pop_back>])]
			fn func(this: &mut CxxVector<$tp>, value: *mut $tp);
		    }
		    func(this, value)
		}

	    }
	}
    }
}

genrs_unique_ptr!(rxx_unique_string, CxxString, crate);
genrs_shared_ptr!(rxx_shared_string, CxxString, crate);
genrs_weak_ptr!(rxx_weak_string, CxxString, crate);
