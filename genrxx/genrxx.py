from functools import partial

def genc_unique_ptr(link_name, c_tp, **kws):
    return f"""
void {link_name}_delete({c_tp} &self) {{
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

def get_cffi_config():
    return {
        'cffi_genc_unique_ptr': partial(genc_unique_ptr, crate='crate'),
        'cffi_genrs_unique_ptr': partial(genrs_unique_ptr, crate='crate'),

        'cffi_unique_ptrs': {
            'rxx_unique_string': {
                'c_tp': 'std::string',
                'rs_tp': 'crate::CxxString',
            }
        },
    }
