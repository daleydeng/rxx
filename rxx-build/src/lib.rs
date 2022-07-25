use lazy_static::lazy_static;

use serde_json::json;
use handlebars::Handlebars;

static TPL_RET_FN: &str = r#"
extern "C" void {{name}}({{{decl_link_args}}} {{{ret_type}}} *__ret) noexcept {
    {{{ret_type}}} (*__func)({{{decl_args}}}) = {{{fn}}};
    new (__ret) {{{ret_type}}}(__func({{{call_args}}}));
}
"#;

static TPL_VOID_FN: &str = r#"
extern "C" void {{name}}({{{decl_link_args}}}) noexcept {
    void (*__func)({{{decl_args}}}) = {{{fn}}};
    __func({{{call_args}}});
}
"#;

// static TPL_MEM_RET_FN: &str = r#"
// extern "C" void {{name}}({{{cls}}} const& self {{{decl_link_args}}} {{{ret_type}}} *__ret) noexcept {
//     {{{ret_type}}} ({{{cls}}}::*__func)({{{decl_args}}}) const = {{{fn}}};
//     new (__ret) {{{ret_type}}}((self.*__func)({{{call_args}}}));
// }
// "#;

// static TPL_MEM_RET_FN_MUT: &str = r#"
// extern "C" void {{name}}({{{cls}}} & self {{{decl_link_args}}} {{{ret_type}}} *__ret) noexcept {
//     {{{ret_type}}} ({{{cls}}}::*__func)({{{decl_args}}}) = {{{fn}}};
//     new (__ret) {{{ret_type}}}((self.*__func)({{{call_args}}}));
// }
// "#;

// static TPL_MEM_VOID_FN: &str = r#"
// void {{name}}({{{cls}}} const& self, {{{decl_link_args}}}) noexcept {
//     {{{ret_type}}} ({{{cls}}}::*__func)({decl_args}) const = {{{fn}}};
//     new (__ret) {{{ret_type}}}((self.*__func)({{{call_args}}}));
// }
// "#;


// static TPL_MEM_VOID_FN_MUT: &str = r#"
// void {{name}}({{{cls}}} const& self, {{{decl_link_args}}}) noexcept {
//     {{{ret_type}}} ({{{cls}}}::*__func)({decl_args}) const = {{{fn}}};
//     new (__ret) {{{ret_type}}}((self.*__func)({{{call_args}}}));
// }
// "#;

static TPL_UNIQUE_PTR: &str = r#"
extern "C" void {{name}}_delete({{{c_tp}}} &self) noexcept {
    rxx::destroy(&self);
}
"#;

static TPL_SHARED_PTR: &str =r#"
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
	hb.register_template_string("tpl_ret_fn", TPL_RET_FN.trim_start()).unwrap();
	hb.register_template_string("tpl_void_fn", TPL_VOID_FN.trim_start()).unwrap();
	hb.register_template_string("tpl_unique_ptr", TPL_UNIQUE_PTR.trim_start()).unwrap();
	hb.register_template_string("tpl_shared_ptr", TPL_SHARED_PTR.trim_start()).unwrap();
	hb.register_template_string("tpl_weak_ptr", TPL_WEAK_PTR.trim_start()).unwrap();
	hb.register_template_string("tpl_vector", TPL_VECTOR.trim_start()).unwrap();
	hb
    };
}

#[derive(Default)]
pub struct FnSig<'a, 'b> {
    pub cls: Option<&'a str>,
    pub fn_name: &'a str,

    pub ret_type: Option<&'a str>,
    pub args: &'b [(&'a str, &'a str)],
}

pub fn genc_fn(link_name: &str, fn_sig: FnSig) -> String {
    let s_decl_args = fn_sig.args.iter().map(|(tp, val)| {
	format!("{} {}", tp, val)
    }).collect::<Vec<_>>().join(",");

    let s_call_args = fn_sig.args.iter().map(|(_, val)| {
	val.to_string()
    }).collect::<Vec<_>>().join(",");

    let mut s_decl_link_args = s_decl_args.clone();

    if fn_sig.ret_type.is_some() {
	if !s_decl_link_args.is_empty() {
	    s_decl_link_args += ",";
	}
    }

    match fn_sig.cls {
	None => {
	    match fn_sig.ret_type {
		Some(ret_type) => {
		    HANDLEBARS.render("tpl_ret_fn", &json!({
			"name": link_name,
			"fn": fn_sig.fn_name,
			"ret_type": ret_type,
			"decl_link_args": s_decl_link_args,
			"decl_args": s_decl_args,
			"call_args": s_call_args,

		    })).unwrap()
		},
		None => {
		    HANDLEBARS.render("tpl_void_fn", &json!({
			"name": link_name,
			"fn": fn_sig.fn_name,
			"decl_link_args": s_decl_link_args,
			"decl_args": s_decl_args,
			"call_args": s_call_args,

		    })).unwrap()
		}
	    }
	},

	Some(cls) => {
	    String::new()
	}
    }
}

pub fn genc_unique_ptr(link_name: &str, c_tp: &str) -> String {
    HANDLEBARS.render("tpl_unique_ptr", &json!({
	"name": link_name,
	"c_tp": c_tp,
    })).unwrap()
}

pub fn genc_shared_ptr(link_name: &str, c_tp: &str) -> String {
    HANDLEBARS.render("tpl_shared_ptr", &json!({
	"name": link_name,
	"c_tp": c_tp,
    })).unwrap()
}

pub fn genc_weak_ptr(link_name: &str, c_tp: &str, c_shared_tp: &str) -> String {
    HANDLEBARS.render("tpl_weak_ptr", &json!({
	"name": link_name,
	"c_tp": c_tp,
	"c_shared_tp": c_shared_tp,
    })).unwrap()
}

pub fn genc_vector(link_name: &str, c_tp: &str, c_item_tp: &str) -> String {
    HANDLEBARS.render("tpl_vector", &json!({
	"name": link_name,
	"c_tp": c_tp,
	"c_item_tp": c_item_tp,
    })).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn() {
	let s=  genc_fn("MapMut_Matrix3d_new", FnSig {
	    fn_name: "MapMut_fixed_new<Matrix3d, double>",
	    ret_type: Some("Eigen::Map<Matrix3d>"),
	    args: &[
		("double *", "data"),
	    ],
	    ..FnSig::default()
	});

	assert_eq!(s, r#"
extern "C" void MapMut_Matrix3d_new(double * data, Eigen::Map<Matrix3d> *__ret) noexcept {
    Eigen::Map<Matrix3d> (*__func)(double * data) = MapMut_fixed_new<Matrix3d, double>;
    new (__ret) Eigen::Map<Matrix3d>(__func(data));
}
"#.trim_start());

	let s = genc_fn("rxx_Matrix3d_print", FnSig {
	    fn_name: "Matrix3d_print",
	    args: &[("Matrix3d const &", "self")],
	    ..FnSig::default()
	});

	assert_eq!(s, r#"
extern "C" void rxx_Matrix3d_print(Matrix3d const & self) noexcept {
    void (*__func)(Matrix3d const & self) = Matrix3d_print;
    __func(self);
}
"#.trim_start());

    }

    #[test]
    fn test_std() {
	let s = genc_unique_ptr("rxx_unique_string", "std::unique_ptr<std::string>");
	assert_eq!(s, r#"
extern "C" void rxx_unique_string_delete(std::unique_ptr<std::string> &self) noexcept {
    rxx::destroy(&self);
}
"#.trim_start());

	let s = genc_shared_ptr("rxx_shared_string", "std::shared_ptr<std::string>");
	assert_eq!(s, r#"
extern "C" void rxx_shared_string_delete(std::shared_ptr<std::string> &self) noexcept {
    rxx::destroy(&self);
}

extern "C" void rxx_shared_string_clone(const std::shared_ptr<std::string> &self, std::shared_ptr<std::string> *out) noexcept {
    rxx::shared_ptr_clone(self, out);
}
"#.trim_start());

	let s = genc_weak_ptr("rxx_weak_string", "std::weak_ptr<std::string>", "std::shared_ptr<std::string>");
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

	let s = genc_vector("rxx_vector_string", "std::vector<std::string>", "std::string");
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
