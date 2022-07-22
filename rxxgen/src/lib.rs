use lazy_static::lazy_static;

use serde_json::json;
use handlebars::Handlebars;

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
	hb.register_template_string("tpl_unique_ptr", TPL_UNIQUE_PTR.trim_start()).unwrap();
	hb.register_template_string("tpl_shared_ptr", TPL_SHARED_PTR.trim_start()).unwrap();
	hb.register_template_string("tpl_weak_ptr", TPL_WEAK_PTR.trim_start()).unwrap();
	hb.register_template_string("tpl_vector", TPL_VECTOR.trim_start()).unwrap();
	hb
    };
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
    fn test_templates() {
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
