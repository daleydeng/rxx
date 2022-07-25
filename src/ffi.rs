
#[macro_export]
macro_rules! genrs_fn {
    ($vis:vis fn $fn:ident ($($arg:ident : $arg_type:ty),*) -> $ret_type:path, $link_name:ident) => {
	$vis fn $fn($($arg: $arg_type),*) -> $ret_type {
	    extern "C" {
		#[link_name = stringify!($link_name)]
		fn __func($($arg: $arg_type),*, __ret: *mut $ret_type);
	    }
	    unsafe {
		let mut __ret = std::mem::MaybeUninit::<$ret_type>::uninit();
		let mut __ret_ptr = __ret.as_mut_ptr();
		__func($($arg),*, __ret_ptr);
		__ret.assume_init()
	    }
	}
    };

    ($vis:vis fn $fn:ident ($($arg:ident: $arg_type:ty),*) -> $ret_type:ty) => {
	genrs_fn!($vis fn $fn($($arg: $arg_type),*) -> $ret_type, $fn);
    };

    ($vis:vis fn $fn:ident ($($arg:ident: $arg_type:ty),*), $link_name:ident) => {
	$vis fn $fn($($arg: $arg_type),*) {
	    extern "C" {
		#[link_name = stringify!($link_name)]
		fn __func($($arg: $arg_type),*);
	    }
	    unsafe {
		__func($($arg),*);
	    }
	}
    };

    ($vis:vis fn $fn:ident ($($arg:ident: $arg_type:ty),*)) => {
	genrs_fn!($vis fn $fn($($arg: $arg_type),*), $fn);
    };

    // &self ret
    ($vis:vis fn $cls:ident :: $fn:ident (&self $(, $arg:ident : $arg_type:ty)*) -> $ret_type:path, $link_name:ident) => {
	impl $cls {
	    $vis fn $fn(&self $(, $arg: $arg_type)*) -> $ret_type {
		extern "C" {
		    #[link_name = stringify!($link_name)]
		    fn __func(this: &$cls $(, $arg: $arg_type)*, __ret: *mut $ret_type);
		}
		unsafe {
		    let mut __ret = std::mem::MaybeUninit::<$ret_type>::uninit();
		    let mut __ret_ptr = __ret.as_mut_ptr();
		    __func(self $(, $arg)*, __ret_ptr);
		    __ret.assume_init()
		}
	    }
	}
    };

    ($vis:vis fn $cls:ident :: $fn:ident (&self $(, $arg:ident : $arg_type:ty)*) -> $ret_type:path) => {
	genrs_fn!($vis fn $cls :: $fn(&self $(, $arg : $arg_type)*) -> $ret_type, $fn);
    };

    // &self void
    ($vis:vis fn $cls:ident :: $fn:ident (&self $(, $arg:ident : $arg_type:ty)*), $link_name:ident) => {
	impl $cls {
	    $vis fn $fn(&self $(, $arg: $arg_type)*) {
		extern "C" {
		    #[link_name = stringify!($link_name)]
		    fn __func(this: &$cls $(, $arg: $arg_type)*);
		}
		unsafe {
		    __func(self $(, $arg)*);
		}
	    }
	}
    };

    ($vis:vis fn $cls:ident :: $fn:ident (&self $(, $arg:ident: $arg_type:ty)*)) => {
	genrs_fn!($vis fn $cls :: $fn(&self $(, $arg: $arg_type)*), $fn);
    };

    // &mut self ret
    ($vis:vis fn $cls:ident :: $fn:ident (&mut self $(, $arg:ident : $arg_type:ty)*) -> $ret_type:path, $link_name:ident) => {
	impl $cls {
	    $vis fn $fn(&mut self $(, $arg: $arg_type)*) -> $ret_type {
		extern "C" {
		    #[link_name = stringify!($link_name)]
		    fn __func(this: &mut $cls $(, $arg: $arg_type)*, __ret: *mut $ret_type);
		}
		unsafe {
		    let mut __ret = std::mem::MaybeUninit::<$ret_type>::uninit();
		    let mut __ret_ptr = __ret.as_mut_ptr();
		    __func(self $(, $arg)*, __ret_ptr);
		    __ret.assume_init()
		}
	    }
	}
    };

    ($vis:vis fn $cls:ident :: $fn:ident (&mut self $(, $arg:ident: $arg_type:ty)*) -> $ret_type:path) => {
	genrs_fn!($vis fn $cls :: $fn(&mut self $(, $arg: $arg_type)*) -> $ret_type, $fn);
    };

    // &self void
    ($vis:vis fn $cls:ident :: $fn:ident (&mut self $(, $arg:ident : $arg_type:ty)*), $link_name:ident) => {
	impl $cls {
	    $vis fn $fn(&mut self $(, $arg: $arg_type)*) {
		extern "C" {
		    #[link_name = stringify!($link_name)]
		    fn __func(this: &mut $cls $(, $arg: $arg_type)*);
		}
		unsafe {
		    __func(self $(, $arg)*);
		}
	    }
	}
    };

    ($vis:vis fn $cls:ident :: $fn:ident (&mut self $(, $arg:ident: $arg_type:ty)*)) => {
	genrs_fn!($vis fn $cls :: $fn(&mut self $(, $arg: $arg_type)*), $fn);
    };

}

#[macro_export]
macro_rules! genrs_unique_ptr {
    ($link_name:ident, $tp:path) => {
	paste::paste! {
	    impl $crate::UniquePtrTarget for $tp {
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
macro_rules! genrs_shared_ptr {
    ($link_name:ident, $tp:path) => {
	paste::paste! {
	    impl $crate::SharedPtrTarget for $tp {
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
macro_rules! genrs_weak_ptr {
    ($link_name:ident, $tp:path) => {
	paste::paste! {
	    impl $crate::WeakPtrTarget for $tp {
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
macro_rules! genrs_vector {
    ($link_name:ident, $tp:path) => {
	paste::paste! {
	    impl $crate::VectorElement for $tp {
		unsafe fn __drop(this: &mut $crate::CxxVector<$tp>) {
		    extern "C" {
			#[link_name=stringify!([<$link_name _delete>])]
			fn func(this: &mut $crate::CxxVector<$tp>);
		    }
		    func(this)
		}

		unsafe fn __size(this: &$crate::CxxVector<$tp>) -> usize {
		    extern "C" {
			#[link_name=stringify!([<$link_name _size>])]
			fn func(this: &$crate::CxxVector<$tp>) -> usize;
		    }
		    func(this)
		}

		unsafe fn __get_unchecked(this: &$crate::CxxVector<$tp>, pos: usize) -> &Self {
		    extern "C" {
			#[link_name=stringify!([<$link_name _get>])]
			fn func(this: &$crate::CxxVector<$tp>, pos: usize) -> &$tp;
		    }
		    func(this, pos)
		}

		unsafe fn __get_unchecked_mut(this: &mut $crate::CxxVector<$tp>, pos: usize) -> &mut Self {
		    extern "C" {
			#[link_name=stringify!([<$link_name _get_mut>])]
			fn func(this: &mut $crate::CxxVector<$tp>, pos: usize) -> &mut $tp;
		    }
		    func(this, pos)
		}

		unsafe fn __push_back(this: &mut $crate::CxxVector<$tp>, value: &mut $tp) {
		    extern "C" {
			#[link_name=stringify!([<$link_name _push_back>])]
			fn func(this: &mut $crate::CxxVector<$tp>, value: &mut $tp);
		    }
		    func(this, value)
		}

		unsafe fn __pop_back(this: &mut $crate::CxxVector<$tp>, value: *mut $tp) {
		    extern "C" {
			#[link_name=stringify!([<$link_name _pop_back>])]
			fn func(this: &mut $crate::CxxVector<$tp>, value: *mut $tp);
		    }
		    func(this, value)
		}

	    }
	}
    }
}

genrs_unique_ptr!(rxx_unique_string, crate::CxxString);
genrs_shared_ptr!(rxx_shared_string, crate::CxxString);
genrs_weak_ptr!(rxx_weak_string, crate::CxxString);
