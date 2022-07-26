use lazy_static::lazy_static;

use handlebars::Handlebars;
use serde_json::json;

static TPL_RET_OBJECT_FN: &str = r#"
extern "C" void {{name}}({{{decl_link_args}}}{{{ret_type}}} *__ret) noexcept {
    {{{ret_type}}} (*__func)({{{decl_args}}}) = {{{fn}}};
    new (__ret) ({{{ret_type}}})(__func({{{call_args}}}));
}
"#;

static TPL_RET_ATOMIC_FN: &str = r#"
extern "C" {{{ret_type}}} {{name}}({{{decl_link_args}}}) noexcept {
    {{{ret_type}}} (*__func)({{{decl_args}}}) = {{{fn}}};
    return __func({{{call_args}}});
}
"#;

static TPL_VOID_FN: &str = r#"
extern "C" void {{name}}({{{decl_link_args}}}) noexcept {
    void (*__func)({{{decl_args}}}) = {{{fn}}};
    __func({{{call_args}}});
}
"#;

static TPL_RET_OBJECT_MEMFN: &str = r#"
extern "C" void {{name}}({{{cls}}} const &self{{{decl_link_args}}}{{{ret_type}}} *__ret) noexcept {
    {{{ret_type}}} ({{{cls}}}::*__func)({{{decl_args}}}) const = {{{fn}}};
    new (__ret) {{{ret_type}}}((self.*__func)({{{call_args}}}));
}
"#;

static TPL_RET_ATOMIC_MEMFN: &str = r#"
extern "C" {{{ret_type}}} {{name}}({{{cls}}} const &self{{{decl_link_args}}}) noexcept {
    {{{ret_type}}} ({{{cls}}}::*__func)({{{decl_args}}}) const = {{{fn}}};
    return (self.*__func)({{{call_args}}});
}
"#;

static TPL_VOID_MEMFN: &str = r#"
extern "C" void {{name}}({{{cls}}} const &self{{{decl_link_args}}}) noexcept {
    void ({{{cls}}}::*__func)({{{decl_args}}}) const = {{{fn}}};
    (self.*__func)({{{call_args}}});
}
"#;

static TPL_RET_OBJECT_MEMFN_MUT: &str = r#"
extern "C" void {{name}}({{{cls}}} &self{{{decl_link_args}}}{{{ret_type}}} *__ret) noexcept {
    {{{ret_type}}} ({{{cls}}}::*__func)({{{decl_args}}}) = {{{fn}}};
    new (__ret) {{{ret_type}}}((self.*__func)({{{call_args}}}));
}
"#;

static TPL_RET_ATOMIC_MEMFN_MUT: &str = r#"
extern "C" {{{ret_type}}} {{name}}({{{cls}}} &self{{{decl_link_args}}}) noexcept {
    {{{ret_type}}} ({{{cls}}}::*__func)({{{decl_args}}}) = {{{fn}}};
    return (self.*__func)({{{call_args}}});
}
"#;

static TPL_VOID_MEMFN_MUT: &str = r#"
extern "C" void {{name}}({{{cls}}} &self{{{decl_link_args}}}) noexcept {
    void ({{{cls}}}::*__func)({{{decl_args}}}) = {{{fn}}};
    (self.*__func)({{{call_args}}});
}
"#;

static TPL_UNIQUE_PTR: &str = r#"
extern "C" void {{name}}_delete({{{c_tp}}} &self) noexcept {
    rxx::destroy(&self);
}
"#;

static TPL_SHARED_PTR: &str = r#"
extern "C" void {{name}}_delete({{{c_tp}}} &self) noexcept {
    rxx::destroy(&self);
}

extern "C" void {{name}}_clone(const {{{c_tp}}} &self, {{{c_tp}}} *out) noexcept {
    rxx::shared_ptr_clone(self, out);
}
"#;

static TPL_WEAK_PTR: &str = r#"
extern "C" void {{name}}_delete({{{c_tp}}} &self) noexcept {
    rxx::destroy(&self);
}

extern "C" void {{name}}_clone(const {{{c_tp}}} &self, {{{c_tp}}} *out) noexcept {
    rxx::weak_ptr_clone(self, out);
}

extern "C" void {{name}}_upgrade(const {{{c_tp}}} &self, {{{c_shared_tp}}} *out) {
    rxx::weak_ptr_upgrade(self, out);
}

extern "C"  void {{name}}_downgrade(const {{{c_shared_tp}}} &self, {{{c_tp}}} *out) {
    rxx::weak_ptr_downgrade(self, out);
}
"#;

static TPL_VECTOR: &str = r#"
extern "C" void {{name}}_delete(const {{{c_tp}}} &self) {
    rxx::destroy(&self);
}

extern "C" std::size_t {{name}}_size(const {{{c_tp}}} &self) {
    return rxx::vector_size(self);
}

extern "C" const {{{c_item_tp}}}& {{name}}_get(const {{{c_tp}}} &self, size_t pos) {
    return rxx::vector_get(self, pos);
}

extern "C" {{{c_item_tp}}}& {{name}}_get_mut({{{c_tp}}} &self, size_t pos) {
    return rxx::vector_get_mut(self, pos);
}

extern "C" void {{name}}_push_back({{{c_tp}}} &self, {{{c_item_tp}}} &val) {
    return rxx::vector_push_back(self, val);
}

extern "C" void {{name}}_pop_back({{{c_tp}}} &self, {{{c_item_tp}}} *out) {
    rxx::vector_pop_back(self, out);
}
"#;

lazy_static! {
    static ref HANDLEBARS: Handlebars<'static> = {
        let mut hb = Handlebars::new();
        hb.set_strict_mode(true);
        for (k, v) in &[
            ("tpl_ret_object_fn", TPL_RET_OBJECT_FN),
            ("tpl_ret_atomic_fn", TPL_RET_ATOMIC_FN),
            ("tpl_void_fn", TPL_VOID_FN),
            ("tpl_ret_object_memfn", TPL_RET_OBJECT_MEMFN),
            ("tpl_ret_atomic_memfn", TPL_RET_ATOMIC_MEMFN),
            ("tpl_void_memfn", TPL_VOID_MEMFN),
            ("tpl_ret_object_memfn_mut", TPL_RET_OBJECT_MEMFN_MUT),
            ("tpl_ret_atomic_memfn_mut", TPL_RET_ATOMIC_MEMFN_MUT),
            ("tpl_void_memfn_mut", TPL_VOID_MEMFN_MUT),
            ("tpl_unique_ptr", TPL_UNIQUE_PTR),
            ("tpl_shared_ptr", TPL_SHARED_PTR),
            ("tpl_weak_ptr", TPL_WEAK_PTR),
            ("tpl_vector", TPL_VECTOR),
        ] {
            hb.register_template_string(k, v.trim_start()).unwrap();
        }

        hb
    };
}

#[derive(Default)]
pub enum ReturnType<'a> {
    #[default]
    None,
    Object(&'a str),
    Atomic(&'a str),
}

impl ReturnType<'_> {
    pub fn is_none(&self) -> bool {
        matches!(*self, Self::None)
    }

    pub fn is_object(&self) -> bool {
        matches!(*self, Self::Object(_))
    }

    pub fn is_atomic(&self) -> bool {
        matches!(*self, Self::Atomic(_))
    }
}

#[derive(Default)]
pub struct FnSig<'a> {
    pub cls: Option<&'a str>,
    pub is_mut: bool,
    pub fn_name: &'a str,

    pub ret_type: ReturnType<'a>,
    pub args: &'a [(&'a str, &'a str)],
}

pub fn genc_fn(link_name: &str, fn_sig: FnSig) -> String {
    let s_decl_args = fn_sig
        .args
        .iter()
        .map(|(tp, val)| format!("{tp} {val}"))
        .collect::<Vec<_>>()
        .join(",");

    let s_call_args = fn_sig
        .args
        .iter()
        .map(|(_, val)| val.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let mut s_decl_link_args = s_decl_args.clone();

    if !s_decl_link_args.is_empty() {
        if fn_sig.ret_type.is_object() {
            s_decl_link_args += ", ";
        }
		if fn_sig.cls.is_some() {
			s_decl_link_args.insert_str(0, ", ");
		}
    } else {
		if fn_sig.ret_type.is_object() && fn_sig.cls.is_some() {
			s_decl_link_args = ", ".to_string();
		}
	}

    match fn_sig.cls {
        None => {
            let (ret_type, tpl_name) = match fn_sig.ret_type {
                ReturnType::None => ("", "tpl_void_fn"),
                ReturnType::Object(rt) => (rt, "tpl_ret_object_fn"),
                ReturnType::Atomic(rt) => (rt, "tpl_ret_atomic_fn"),
            };

            HANDLEBARS
                .render(
                    tpl_name,
                    &json!({
                    "name": link_name,
                    "fn": fn_sig.fn_name,
                    "ret_type": ret_type,
                    "decl_link_args": s_decl_link_args,
                    "decl_args": s_decl_args,
                    "call_args": s_call_args,

                    }),
                )
                .unwrap()
        }

        Some(cls) => {
            let (ret_type, tpl_name) = match (fn_sig.ret_type, fn_sig.is_mut) {
                (ReturnType::None, false) => ("", "tpl_void_memfn"),
                (ReturnType::None, true) => ("", "tpl_void_memfn_mut"),
                (ReturnType::Object(rt), false) => (rt, "tpl_ret_object_memfn"),
                (ReturnType::Object(rt), true) => (rt, "tpl_ret_object_memfn_mut"),
                (ReturnType::Atomic(rt), false) => (rt, "tpl_ret_atomic_memfn"),
                (ReturnType::Atomic(rt), true) => (rt, "tpl_ret_atomic_memfn_mut"),
            };

            let fn_name = fn_sig.fn_name.replace("$C", cls);
            let ret_type = ret_type.replace("$C", cls);
            let s_decl_link_args = s_decl_link_args.replace("$C", cls);
            let s_decl_args = s_decl_args.replace("$C", cls);
            let s_call_args = s_call_args.replace("$C", cls);

            HANDLEBARS
                .render(
                    tpl_name,
                    &json!({
                    "cls": cls,
                    "name": link_name,
                    "fn": fn_name,
                    "ret_type": ret_type,
                    "decl_link_args": s_decl_link_args,
                    "decl_args": s_decl_args,
                    "call_args": s_call_args,

                    }),
                )
                .unwrap()
        }
    }
}

pub fn genc_unique_ptr(link_name: &str, c_tp: &str) -> String {
    HANDLEBARS
        .render(
            "tpl_unique_ptr",
            &json!({
            "name": link_name,
            "c_tp": c_tp,
            }),
        )
        .unwrap()
}

pub fn genc_shared_ptr(link_name: &str, c_tp: &str) -> String {
    HANDLEBARS
        .render(
            "tpl_shared_ptr",
            &json!({
            "name": link_name,
            "c_tp": c_tp,
            }),
        )
        .unwrap()
}

pub fn genc_weak_ptr(link_name: &str, c_tp: &str, c_shared_tp: &str) -> String {
    HANDLEBARS
        .render(
            "tpl_weak_ptr",
            &json!({
            "name": link_name,
            "c_tp": c_tp,
            "c_shared_tp": c_shared_tp,
            }),
        )
        .unwrap()
}

pub fn genc_vector(link_name: &str, c_tp: &str, c_item_tp: &str) -> String {
    HANDLEBARS
        .render(
            "tpl_vector",
            &json!({
            "name": link_name,
            "c_tp": c_tp,
            "c_item_tp": c_item_tp,
            }),
        )
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn() {
        let s = genc_fn(
            "MapMut_Matrix3d_new",
            FnSig {
                fn_name: "MapMut_fixed_new<Matrix3d, double>",
                ret_type: ReturnType::Object("Eigen::Map<Matrix3d>"),
                args: &[("double *", "data")],
                ..FnSig::default()
            },
        );

        assert_eq!(
            s,
            r#"
extern "C" void MapMut_Matrix3d_new(double * data, Eigen::Map<Matrix3d> *__ret) noexcept {
    Eigen::Map<Matrix3d> (*__func)(double * data) = MapMut_fixed_new<Matrix3d, double>;
    new (__ret) (Eigen::Map<Matrix3d>)(__func(data));
}
"#
            .trim_start()
        );

        let s = genc_fn(
            "rxx_Matrix3d_print",
            FnSig {
                fn_name: "Matrix3d_print",
                args: &[("Matrix3d const &", "self")],
                ..FnSig::default()
            },
        );

        assert_eq!(
            s,
            r#"
extern "C" void rxx_Matrix3d_print(Matrix3d const & self) noexcept {
    void (*__func)(Matrix3d const & self) = Matrix3d_print;
    __func(self);
}
"#
            .trim_start()
        );
    }

    #[test]
    fn test_std() {
        let s = genc_unique_ptr("rxx_unique_string", "std::unique_ptr<std::string>");
        assert_eq!(
            s,
            r#"
extern "C" void rxx_unique_string_delete(std::unique_ptr<std::string> &self) noexcept {
    rxx::destroy(&self);
}
"#
            .trim_start()
        );

        let s = genc_shared_ptr("rxx_shared_string", "std::shared_ptr<std::string>");
        assert_eq!(s, r#"
extern "C" void rxx_shared_string_delete(std::shared_ptr<std::string> &self) noexcept {
    rxx::destroy(&self);
}

extern "C" void rxx_shared_string_clone(const std::shared_ptr<std::string> &self, std::shared_ptr<std::string> *out) noexcept {
    rxx::shared_ptr_clone(self, out);
}
"#.trim_start());

        let s = genc_weak_ptr(
            "rxx_weak_string",
            "std::weak_ptr<std::string>",
            "std::shared_ptr<std::string>",
        );
        assert_eq!(s, r#"
extern "C" void rxx_weak_string_delete(std::weak_ptr<std::string> &self) noexcept {
    rxx::destroy(&self);
}

extern "C" void rxx_weak_string_clone(const std::weak_ptr<std::string> &self, std::weak_ptr<std::string> *out) noexcept {
    rxx::weak_ptr_clone(self, out);
}

extern "C" void rxx_weak_string_upgrade(const std::weak_ptr<std::string> &self, std::shared_ptr<std::string> *out) {
    rxx::weak_ptr_upgrade(self, out);
}

extern "C"  void rxx_weak_string_downgrade(const std::shared_ptr<std::string> &self, std::weak_ptr<std::string> *out) {
    rxx::weak_ptr_downgrade(self, out);
}
"#.trim_start());

        let s = genc_vector(
            "rxx_vector_string",
            "std::vector<std::string>",
            "std::string",
        );
        assert_eq!(s,  r#"
extern "C" void rxx_vector_string_delete(const std::vector<std::string> &self) {
    rxx::destroy(&self);
}

extern "C" std::size_t rxx_vector_string_size(const std::vector<std::string> &self) {
    return rxx::vector_size(self);
}

extern "C" const std::string& rxx_vector_string_get(const std::vector<std::string> &self, size_t pos) {
    return rxx::vector_get(self, pos);
}

extern "C" std::string& rxx_vector_string_get_mut(std::vector<std::string> &self, size_t pos) {
    return rxx::vector_get_mut(self, pos);
}

extern "C" void rxx_vector_string_push_back(std::vector<std::string> &self, std::string &val) {
    return rxx::vector_push_back(self, val);
}

extern "C" void rxx_vector_string_pop_back(std::vector<std::string> &self, std::string *out) {
    rxx::vector_pop_back(self, out);
}
"#.trim_start());
    }
}
