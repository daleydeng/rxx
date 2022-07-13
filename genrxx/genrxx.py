from functools import partial

def genc_unique_ptr(link_name, c_tp, **kws):
    return f"""
void {link_name}_delete(std::unique_ptr<{c_tp}> &self) {{
    destroy(&self);
}}
""".strip()

def genrs_unique_ptr(link_name, rs_tp, crate='rxx', **kws):
    return f"""
impl {crate}::UniquePtrTarget for {rs_tp} {{
    unsafe fn __drop(this: *mut core::ffi::c_void) {{
        extern "C" {{
            fn {link_name}_delete(this: *mut core::ffi::c_void);
        }}
        {link_name}_delete(this);
    }}
}}
""".strip()

def genc_shared_ptr(link_name, c_tp, **kws):
    return f"""
void {link_name}_delete(std::shared_ptr<{c_tp}> &self) {{
    destroy(&self);
}}

void {link_name}_clone(const std::shared_ptr<{c_tp}> &self, std::shared_ptr<{c_tp}> *out) {{
    shared_ptr_clone(self, out);
}}
""".strip()

def genrs_shared_ptr(link_name, rs_tp, crate='rxx', **kws):
    return f"""
impl {crate}::SharedPtrTarget for {rs_tp} {{
    unsafe fn __drop(this: *mut core::ffi::c_void) {{
        extern "C" {{
            fn {link_name}_delete(this: *mut core::ffi::c_void);
        }}
        {link_name}_delete(this);
    }}

    unsafe fn __clone(this: *const core::ffi::c_void, out: *mut core::ffi::c_void) {{
        extern "C" {{
            fn {link_name}_clone(this: *const core::ffi::c_void, out: *mut core::ffi::c_void);
        }}
        {link_name}_clone(this, out);
    }}
}}
""".strip()

def get_cffi_config():
    return {
        'cffi_genc_unique_ptr': partial(genc_unique_ptr, crate='crate'),
        'cffi_genrs_unique_ptr': partial(genrs_unique_ptr, crate='crate'),
        'cffi_genc_shared_ptr': partial(genc_shared_ptr, crate='crate'),
        'cffi_genrs_shared_ptr': partial(genrs_shared_ptr, crate='crate'),

        'cffi_unique_ptrs': {
            'rxx_unique_string': {
                'c_tp': 'std::string',
                'rs_tp': 'crate::CxxString',
            },
        },
        'cffi_shared_ptrs': {
            'rxx_shared_string': {
                'c_tp': 'std::string',
                'rs_tp': 'crate::CxxString',
            },
        },
    }
