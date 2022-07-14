#include "wrapper.hh"
extern "C" {

  using namespace rxx;

  void rxx_string_init(const uint8_t *ptr, size_t len,
                       std::string *out) noexcept {
    new (out) std::string(reinterpret_cast<const char *>(ptr), len);
  }

  void rxx_string_destroy(std::string *self) noexcept {
    using std::string;
    self->~string();
  }

  size_t rxx_string_length(const std::string &self) noexcept {
    return self.length();
  }

  const char* rxx_string_data(const std::string &self) noexcept {
    return self.data();
  }

  void rxx_string_clear(std::string &self) noexcept { self.clear(); }

  void rxx_string_reserve(std::string &self, size_t n) noexcept {
    self.reserve(n);
  }

  void rxx_string_push(std::string &self, const uint8_t *ptr, size_t len) noexcept {
    self.append((const char*)ptr, len);
  }

  void rxx_unique_ptr_string_delete(std::unique_ptr<std::string> &self) {
    destroy(&self);
  }

  void test_new_unique_ptr(int64_t v, std::unique_ptr<int64_t> *out) {
    new (out) std::unique_ptr<int64_t>(new int64_t(v));
  }

  void test_delete_unique_ptr(std::unique_ptr<int64_t> &self) {
    destroy(&self);
  }

  void test_new_shared_ptr(int64_t v, std::shared_ptr<int64_t> *out) {
    new (out) std::shared_ptr<int64_t>(new int64_t(v));
  }

  void test_delete_shared_ptr(std::shared_ptr<int64_t> &self) {
    destroy(&self);
  }

  void test_clone_shared_ptr(const std::shared_ptr<int64_t> &self, std::shared_ptr<int64_t> *out) {
    shared_ptr_clone(self, out);
  }

  void test_upgrade_weak_ptr(const std::weak_ptr<int64_t> &self, std::shared_ptr<int64_t> *out) {
    weak_ptr_upgrade(self, out);
  }

  void test_downgrade_weak_ptr(const std::shared_ptr<int64_t> &self, std::weak_ptr<int64_t> *out) {
    weak_ptr_downgrade(self, out);
  }

  void test_delete_weak_ptr(std::weak_ptr<int64_t> &self) {
    destroy(&self);
  }

  void test_clone_weak_ptr(const std::weak_ptr<int64_t> &self, std::weak_ptr<int64_t> *out) {
    weak_ptr_clone(self, out);
  }

  void test_new_vector(const int64_t *data, size_t len, std::vector<int64_t> *out) {
    new (out) std::vector<int64_t>;
    out->assign(data, data+len);
  }

  void test_delete_vector(std::vector<int64_t> &out) {
    destroy(&out);
  }

  size_t test_vector_size(const std::vector<int64_t> &self) {
    return vector_size(self);
  }

  const int64_t& test_vector_get(const std::vector<int64_t> &self, size_t pos) {
    return vector_get(self, pos);
  }

  int64_t& test_vector_get_mut(std::vector<int64_t> &self, size_t pos) {
    return vector_get_mut(self, pos);
  }

  void test_vector_push_back(std::vector<int64_t> &self, int64_t &value) {
    vector_push_back(self, value);
  }

  void test_vector_pop_back(std::vector<int64_t> &self, int64_t *out) {
    vector_pop_back(self, out);
  }

  void test_new_unique_ptr_string(std::unique_ptr<std::string> *out) {
    new (out) std::unique_ptr<std::string>(new std::string("test"));
  }

  void test_new_shared_ptr_string(std::shared_ptr<std::string> *out) {
    new (out) std::shared_ptr<std::string>(new std::string("test"));
  }

/*[[[cog
 from genrxx import *
 cog.outl(cog_genc())
]]]*/
//[[[end]]]
}
