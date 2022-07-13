#!/usr/bin/env python
import sys
import os.path as osp
import importlib
from jinja2 import Environment, BaseLoader, FileSystemLoader
from easydict import EasyDict as edict
import click

funcs = {}

def register(fn):
    name = fn.__name__
    funcs[name] = fn
    return fn

def args_decl(args):
    return ", ".join(f"{t} {v}" for t, v in args)

def args_call(args):
    return ", ".join(v for _, v in args)

def format_args(args, cls):
    return [(t.replace("$C", cls), v) for t, v in args]

def rs_args_decl(args, is_mem=False):
    if is_mem:
        args = [(args[0][0], '_')] + args[1:]
    return ", ".join(("&self" if t == "&Self" and v == "self" else f"{v}: {t}") for t, v in args)

c_fn_tpls = {
    "ret": """
void {link_name}({decl_link_args}) noexcept {{
    {ret_type} (*_func)({decl_args}) = {fn};
    new ({ret_ptr}) {ret_type}(_func({call_args}));
}}
""",
    "void": """
void {link_name}({decl_link_args}) noexcept {{
    {ret_type} (*_func)({decl_args}) = {fn};
    _func({call_args});
}}
"""
}

rs_fn_tpls = {
    "ret": """
pub fn {fn}({decl_args}) -> {ret_type} {{
    extern "C" {{
        #[link_name = "{link_name}"]
        fn __func{fn_gt}({decl_link_args});
    }}
    unsafe {{
        let mut ret = std::mem::MaybeUninit::<{ret_type}>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func({call_link_args});
        ret.assume_init()
    }}
}}
""",
    "void": """
pub fn {fn}({decl_args}) -> {ret_type} {{
    extern "C" {{
        #[link_name = "{link_name}"]
        fn __func{fn_gt}({decl_link_args});
    }}
    unsafe {{
        __func({call_link_args});
    }}
}}
"""
}

c_mem_fn_tpls = {
    "mem_ret": """
void {link_name}({decl_link_args}) noexcept {{
    {ret_type} ({cls}::*__func)({decl_args}) {postfix} = {fn};
    new ({ret_ptr}) {ret_type}((self.*__func)({call_args}));
}}
""",
    "mem_void": """
void {link_name}({decl_link_args}) noexcept {{
    {ret_type} ({cls}::*__func)({decl_args}) {postfix} = {fn};
    (self.*__func)({call_args});
}}
""",
    "fn_ret": """
void {link_name}({decl_link_args}) noexcept {{
    {ret_type} (*__func)({decl_self_args}) {postfix} = {fn};
    new ({ret_ptr}) {ret_type}(__func({call_self_args}));
}}
""",
    "fn_void": """
void {link_name}({decl_link_args}) noexcept {{
    {ret_type} (*__func)({decl_self_args}) {postfix} = {fn};
    __func({call_self_args});
}}
""",
}

rs_mem_fn_tpls = {
    "mem_const_ret": """
impl{impl_gt} {cls} {{
    pub fn {fn}({decl_self_args}) -> {ret_type} {{
        extern "C" {{
            #[link_name = "{link_name}"]
            fn __func{fn_gt}({decl_link_args});
        }}
        unsafe {{
            let mut ret = std::mem::MaybeUninit::<{ret_type}>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func({call_link_args});
            ret.assume_init()
        }}
    }}
}}
""",

    "mem_const_void": """
impl{impl_gt} {cls} {{
    pub fn {fn}({decl_self_args}) {{
        extern "C" {{
            #[link_name = "{link_name}"]
            fn __func{fn_gt}({decl_link_args});
        }}
        unsafe {{
            __func({call_link_args});
        }}
    }}
}}
""",

    "mem_mut_void": """
impl{impl_gt} {cls} {{
    pub fn {fn}({decl_self_args}) {{
        extern "C" {{
            #[link_name = "{link_name}"]
            fn __func({decl_link_args});
        }}
        unsafe {{
            __func({call_link_args});
        }}
    }}
}}
"""
}

def format_c_sig(v):
    v.setdefault('args', [])
    v.setdefault('ret_type', 'void')
    v.setdefault('is_const', True)
    v['need_return'] = v['ret_type'] != 'void'
    v['has_cls'] = 'cls' in v
    if v['has_cls']:
        cls = v['cls']
        v['fn'] = v['fn'].replace('$C', cls)
        v['ret_type'] = v['ret_type'].replace('$C', cls)
        v['args'] = [(t.replace("$C", cls), v) for t, v in v['args']]

        is_fn_member = v.get("is_member", None)
        if is_fn_member is None:
            is_fn_member = f'{v["cls"]}::' in v['fn']
        v["is_fn_member"] = is_fn_member

    return edict(v)

@register
def cffi_genc_fn(link_name, c_sig, **kws):
    c_sig = format_c_sig(c_sig)
    fn = c_sig.fn
    args = c_sig.args
    ret_type = c_sig.ret_type
    need_return = c_sig.need_return

    if not c_sig.has_cls:
        link_args = args.copy()
        ret_ptr = "return_"
        if need_return:
            link_args += [(f"{ret_type}*", ret_ptr)]

        c_str = c_fn_tpls["ret" if need_return else "void"].format(
            link_name=link_name,
            decl_link_args=args_decl(link_args),

            decl_args=args_decl(args),
            call_args=args_call(args),

            fn=fn,
            ret_type=ret_type,
            ret_ptr=ret_ptr,
        )
        return c_str.strip()

    cls = c_sig.cls
    is_const = c_sig.is_const
    if is_const:
        self_args = [(f"const {cls} &", "self")] + list(args)
    else:
        self_args = [(f"{cls} &", "self")] + list(args)

    if is_const:
        link_args = [(f'const {cls} &', "self")] + args
    else:
        link_args = [(f'{cls} &', "self")] + args

    if is_const and c_sig.is_fn_member:
        postfix = "const"
    else:
        postfix = ""

    ret_ptr = "return_"
    if need_return:
        link_args += [(f"{ret_type}*", ret_ptr)]

    tpl_key = '{}_{}'.format(
        "mem" if c_sig.is_fn_member else "fn",
        "ret" if need_return else "void")
    c_str = c_mem_fn_tpls[tpl_key].format(

        link_name=link_name,
        decl_link_args=args_decl(link_args),

        decl_args=args_decl(args),
        call_args=args_call(args),

        decl_self_args=args_decl(self_args),
        call_self_args=args_call(self_args),

        postfix=postfix,
        cls=cls,
        fn=fn,
        ret_type=ret_type,
        ret_ptr=ret_ptr
    )

    return c_str.strip()


def to_rs_type(ct, type_mapping):
    if ct == 'void' or ct is None:
        return ct

    ct = ct.strip()

    mod = ""
    if ct.endswith('&'):
        ct = ct[:-1].strip()
        if ct.endswith(' const'):
            ct = ct[:-len(' const')]
            mod = "is_const_ref"
        else:
            mod = "is_mut_ref"

    elif ct.endswith('*'):
        ct = ct[:-1].strip()
        if ct.endswith(' const'):
            ct = ct[:-len(' const')]
            mod = "is_const_ptr"
        else:
            mod = 'is_mut_ptr'

    rs_tp = type_mapping.get(ct, ct)
    if mod == "is_const_ref":
        return f'&{rs_tp}'
    elif mod == 'is_mut_ref':
        return f'&mut {rs_tp}'
    elif mod == 'is_const_ptr':
        return f'*const {rs_tp}'
    elif mod == 'is_mut_ptr':
        return f'*mut {rs_tp}'

    return rs_tp

@register
def cffi_genrs_fn(link_name, rs_sig=None, c_sig={}, type_mapping={}, **kws):
    if rs_sig is None:
        return ""

    c_sig = format_c_sig(c_sig)
    tm = type_mapping

    has_cls = 'cls' in rs_sig or 'cls' in c_sig
    rs_fn = rs_sig.get("fn", link_name.split('$')[-1])
    rs_ret_type = rs_sig.get("ret_type", to_rs_type(c_sig.ret_type, tm))
    rs_args = rs_sig.get("args", [(to_rs_type(t, tm), v) for t, v in c_sig.args])

    has_return = rs_ret_type != 'void'
    if not has_cls:
        rs_link_args = rs_args.copy()
        if has_return:
            rs_link_args += [(f"*mut {rs_ret_type}", "__return")]

        if has_return:
            tpl = rs_fn_tpls["ret"]
        else:
            tpl = rs_fn_tpls["void"]

        rs_str = tpl.format(
            link_name=link_name,
            decl_args=rs_args_decl(rs_args),
            decl_link_args=rs_args_decl(rs_link_args, is_mem=True),
            call_link_args=args_call(rs_link_args),

            fn=rs_fn,
            fn_gt=rs_sig.get("fn_gt", ""),
            ret_type=rs_ret_type,
        )
        return rs_str.strip()

    rs_cls = rs_sig.get('cls', to_rs_type(c_sig.get('cls', None), tm))
    is_const = rs_sig.get('is_const', c_sig.is_const)

    if is_const:
        rs_self_args = [("&Self", "self")] + rs_args
        rs_link_args = [(f"&{rs_cls}", "self")] + rs_args
    else:
        rs_self_args = [("&mut Self", "self")] + rs_args
        rs_link_args = [(f"&mut {rs_cls}", "self")] + rs_args

    if has_return:
        rs_link_args += [(f"*mut {rs_ret_type}", "__return")]

    tpl_key = ""
    if is_const:
        if has_return:
            tpl_key = "mem_const_ret"
        else:
            tpl_key = "mem_const_void"
    else:
        if has_return:
            raise
        else:
            tpl_key = "mem_mut_void"

    if not tpl_key:
        return ""

    return rs_mem_fn_tpls[tpl_key].format(
        link_name=link_name,
        decl_self_args=rs_args_decl(rs_self_args),
        decl_link_args=rs_args_decl(rs_link_args, is_mem=True),
        call_link_args=args_call(rs_link_args),
        cls=rs_cls,
        fn=rs_fn,
        ret_type=rs_ret_type,

        fn_gt=rs_sig.get('fn_gt', rs_sig.get('gt', '')),
        impl_gt=rs_sig.get('impl_gt', rs_sig.get('gt', '')),
    ).strip()

def file_name(f):
    return osp.splitext(osp.basename(f))[0]

@click.command()
@click.option('-o', '--out', default=None)
@click.option('-g', "--gvars", default="")
@click.argument("src")
def main(out, gvars, src):
    sys.path.insert(0, osp.abspath(osp.dirname(gvars)))
    m = importlib.import_module(file_name(gvars))
    gvars = m.get_cffi_config()

    base_dir = osp.dirname(src)
    tpl = Environment(
        loader=FileSystemLoader(base_dir),
        extensions=['jinja2.ext.do'],
    ).from_string(open(src).read())

    tpl.globals.update(funcs)
    tpl.globals.update(gvars)

    s = tpl.render()
    if not out:
        print (s)
    else:
        print (s, file=open(out, 'w'))

if __name__ == "__main__":
    main()
